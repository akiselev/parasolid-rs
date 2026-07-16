//! In-memory delta frustrum for **partitioned rollback**.
//!
//! Parasolid stores rollback deltas through an application-supplied frustrum of
//! six callbacks (registered via `PK_DELTA_register_callbacks`, which must be
//! called *before* `PK_SESSION_start`). This module implements those callbacks
//! against a process-memory store keyed by pmark, so `SessionConfig::rollback`
//! can switch on pmarks/`Pmark::goto` without any real files.
//!
//! The store is `thread_local` because the callbacks take no user-context
//! pointer and a Parasolid `Session` is `!Send`/`!Sync` (single-threaded).

use std::cell::RefCell;
use std::collections::HashMap;
use std::os::raw::{c_int, c_void};

use parasolid_sys::*;

/// Frustrum success code (Parasolid `FR_no_errors`).
const FR_OK: c_int = 0;

// Everything is keyed by pmark. The write side returns a strid, but we make
// that strid == the pmark, so write/read/close/delete all key uniformly (the
// read side is only ever given the pmark — see `open_for_read`).
struct DeltaStore {
    deltas: HashMap<i32, Vec<u8>>,      // pmark -> serialized delta
    read_cursor: HashMap<i32, usize>,   // pmark -> current read offset
}

impl DeltaStore {
    fn new() -> Self {
        DeltaStore {
            deltas: HashMap::new(),
            read_cursor: HashMap::new(),
        }
    }
}

thread_local! {
    static STORE: RefCell<DeltaStore> = RefCell::new(DeltaStore::new());
}

/// Clear all stored deltas — called when a rollback session starts so state
/// never leaks between sessions on the same thread.
pub(crate) fn reset_store() {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        *s = DeltaStore::new();
    });
}

unsafe extern "C" fn open_for_write(pmark: PK_PMARK_t, strid: *mut c_int) -> c_int {
    if strid.is_null() {
        return FR_OK;
    }
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        s.deltas.insert(pmark, Vec::new()); // fresh buffer for this pmark
        s.read_cursor.remove(&pmark);
    });
    // Use the pmark itself as the stream id, so write/close key by pmark too.
    unsafe { *strid = pmark };
    FR_OK
}

unsafe extern "C" fn open_for_read(pmark: PK_PMARK_t) -> c_int {
    STORE.with(|s| {
        s.borrow_mut().read_cursor.insert(pmark, 0);
    });
    FR_OK
}

unsafe extern "C" fn close_stream(_strid: c_int) -> c_int {
    // strid == pmark; nothing to release (buffers persist until `delete`).
    FR_OK
}

unsafe extern "C" fn write_delta(strid: c_int, n_bytes: c_int, buffer: *const c_void) -> c_int {
    if buffer.is_null() || n_bytes <= 0 {
        return FR_OK;
    }
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let slice = unsafe { std::slice::from_raw_parts(buffer as *const u8, n_bytes as usize) };
        s.deltas.entry(strid).or_default().extend_from_slice(slice);
    });
    FR_OK
}

unsafe extern "C" fn read_delta(strid: c_int, max_bytes: c_int, buffer: *mut c_void) -> c_int {
    if buffer.is_null() || max_bytes <= 0 {
        return FR_OK;
    }
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        let pmark = strid; // read streams are keyed by pmark
        let cursor = *s.read_cursor.get(&pmark).unwrap_or(&0);
        let want = max_bytes as usize;
        let k = match s.deltas.get(&pmark) {
            Some(blob) => {
                let avail = blob.len().saturating_sub(cursor);
                let k = avail.min(want);
                let dst = unsafe { std::slice::from_raw_parts_mut(buffer as *mut u8, want) };
                if k > 0 {
                    dst[..k].copy_from_slice(&blob[cursor..cursor + k]);
                }
                for b in &mut dst[k..] {
                    *b = 0; // zero any shortfall (defensive)
                }
                k
            }
            None => 0,
        };
        s.read_cursor.insert(pmark, cursor + k);
    });
    FR_OK
}

unsafe extern "C" fn delete_delta(pmark: PK_PMARK_t) -> c_int {
    STORE.with(|s| {
        let mut s = s.borrow_mut();
        s.deltas.remove(&pmark);
        s.read_cursor.remove(&pmark);
    });
    FR_OK
}

/// Build the frustrum callback table for `PK_DELTA_register_callbacks`.
pub(crate) fn delta_callbacks() -> PK_DELTA_callbacks_t {
    PK_DELTA_callbacks_t {
        open_for_write_fn: Some(open_for_write),
        open_for_read_fn: Some(open_for_read),
        close_fn: Some(close_stream),
        write_fn: Some(write_delta),
        read_fn: Some(read_delta),
        delete_fn: Some(delete_delta),
    }
}
