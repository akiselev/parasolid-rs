//! Default frustrum callback implementations and configuration.
//!
//! The "frustrum" is Parasolid's mandatory callback interface. Applications must
//! provide memory management and file I/O callbacks before starting a session.
//!
//! This module provides sensible defaults:
//! - **Memory**: uses `std::alloc` with 16-byte alignment.
//! - **File I/O**: uses `std::fs` with guise-based file extensions per PK §6.4.2.
//!
//! Custom callbacks can be supplied by building a `PK_SESSION_frustrum_t`
//! directly and passing it to `PK_SESSION_register_frustrum`.

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::os::raw::{c_char, c_int};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use parasolid_sys::*;

// =============================================================================
// FrustrumConfig
// =============================================================================

/// Configuration for the default frustrum callbacks.
#[derive(Debug, Clone)]
pub struct FrustrumConfig {
    /// Base directory for resolving file keys. If `None`, keys are treated as
    /// absolute or relative to the current working directory.
    pub(crate) base_dir: Option<PathBuf>,
}

impl FrustrumConfig {
    /// Create a new config with defaults.
    pub fn new() -> Self {
        FrustrumConfig { base_dir: None }
    }

    /// Set the base directory for file I/O. File keys passed by Parasolid will
    /// be resolved relative to this directory.
    pub fn base_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.base_dir = Some(dir.into());
        self
    }
}

impl Default for FrustrumConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Global state for default callbacks
// =============================================================================

/// Stored config — resettable between sessions (unlike OnceLock).
static CONFIG: Mutex<Option<FrustrumConfig>> = Mutex::new(None);

/// Open file streams, indexed by stream ID (guise value).
enum StreamHandle {
    Reader(BufReader<File>),
    Writer(BufWriter<File>),
}

static STREAM_TABLE: LazyLock<Mutex<HashMap<i32, StreamHandle>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Alignment for all FMALLO allocations. 16 bytes covers f64 SIMD alignment.
const ALLOC_ALIGN: usize = 16;

// =============================================================================
// Public API
// =============================================================================

/// Build a `PK_SESSION_frustrum_t` populated with the default callbacks.
pub(crate) fn build_frustrum(config: &FrustrumConfig) -> PK_SESSION_frustrum_t {
    // Store config for use by file callbacks (replaces any previous config)
    if let Ok(mut guard) = CONFIG.lock() {
        *guard = Some(config.clone());
    }

    PK_SESSION_frustrum_t {
        fstart: Some(default_fstart),
        fstop: Some(default_fstop),
        fabort: None,
        ftmkey: None,
        ffoprd: Some(default_ffoprd),
        ffopwr: Some(default_ffopwr),
        ffread: Some(default_ffread),
        ffwrit: Some(default_ffwrit),
        ffclos: Some(default_ffclos),
        fmallo: Some(default_fmallo),
        fmfree: Some(default_fmfree),
        goopsg: None,
        gosgmt: None,
        goclsg: None,
    }
}

/// Reset global frustrum state. Called when a session is stopped.
pub(crate) fn reset() {
    if let Ok(mut table) = STREAM_TABLE.lock() {
        table.clear();
    }
    if let Ok(mut guard) = CONFIG.lock() {
        *guard = None;
    }
}

// =============================================================================
// Guise-to-extension mapping (PK §6.4.2)
// =============================================================================

/// Map a PK file guise code to the standard text-format extension.
///
/// Per the Parasolid frustrum convention, the application appends a
/// type-appropriate extension to the key before opening the file.
fn guise_extension(guise: c_int) -> &'static str {
    match guise {
        PK_FFCXMT_transmit => ".xmt_txt",
        PK_FFCXMT_schema => ".sch_txt",
        PK_FFCXMT_journal => ".jnl_txt",
        PK_FFCXMT_snapshot => ".snp_txt",
        PK_FFCXMT_partition => ".xmp_txt",
        PK_FFCXMT_delta => ".xmd_txt",
        PK_FFCXMT_mesh => ".xmm_txt",
        _ => "",
    }
}

// =============================================================================
// Session lifecycle callbacks
// =============================================================================

unsafe extern "C" fn default_fstart(ifail: *mut c_int) {
    unsafe { *ifail = 0 };
}

unsafe extern "C" fn default_fstop(ifail: *mut c_int) {
    // Close all open streams
    if let Ok(mut table) = STREAM_TABLE.lock() {
        table.clear();
    }
    unsafe { *ifail = 0 };
}

// =============================================================================
// Memory callbacks
// =============================================================================

unsafe extern "C" fn default_fmallo(
    nbytes: *const c_int,
    memory: *mut *mut c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let size = *nbytes as usize;
        if size == 0 {
            *memory = std::ptr::null_mut();
            *ifail = 0;
            return;
        }
        let layout = std::alloc::Layout::from_size_align(size, ALLOC_ALIGN)
            .expect("invalid allocation layout");
        let ptr = std::alloc::alloc_zeroed(layout);
        if ptr.is_null() {
            *ifail = 1;
        } else {
            *memory = ptr as *mut c_char;
            *ifail = 0;
        }
    }));
    if result.is_err() {
        unsafe { *ifail = 1 };
    }
}

// INVARIANT (PK §6.5): Parasolid guarantees that fmfree receives the exact
// same `nbytes` value that was passed to fmallo for the same pointer. This is
// required for Layout reconstruction without a size-tracking header. If this
// contract is ever violated, std::alloc::dealloc receives a wrong Layout → UB.
unsafe extern "C" fn default_fmfree(
    nbytes: *const c_int,
    memory: *mut *mut c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let ptr = *memory;
        if ptr.is_null() {
            *ifail = 0;
            return;
        }
        let size = *nbytes as usize;
        if size == 0 {
            *memory = std::ptr::null_mut();
            *ifail = 0;
            return;
        }
        let layout = std::alloc::Layout::from_size_align(size, ALLOC_ALIGN)
            .expect("invalid deallocation layout");
        std::alloc::dealloc(ptr as *mut u8, layout);
        *memory = std::ptr::null_mut();
        *ifail = 0;
    }));
    if result.is_err() {
        unsafe { *ifail = 1 };
    }
}

// =============================================================================
// File I/O callbacks
// =============================================================================

/// Resolve a PK file key + guise to a filesystem path.
///
/// Appends the guise-appropriate extension to the key, then optionally
/// joins with the configured base directory.
///
/// # Safety
///
/// `key` must point to at least `key_len` valid bytes.
unsafe fn resolve_key_for_guise(
    key: *const c_char,
    key_len: c_int,
    guise: c_int,
) -> Option<PathBuf> {
    if key.is_null() || key_len <= 0 {
        return None;
    }
    let bytes = unsafe { std::slice::from_raw_parts(key as *const u8, key_len as usize) };
    // Trim trailing whitespace/nulls (PK keys may be space-padded)
    let trimmed = std::str::from_utf8(bytes).ok()?.trim_end();
    if trimmed.is_empty() {
        return None;
    }

    // Append guise extension: "my_part" + ".xmt_txt" → "my_part.xmt_txt"
    let ext = guise_extension(guise);
    let filename = format!("{trimmed}{ext}");
    let path = Path::new(&filename);

    // Fail if CONFIG mutex is poisoned rather than silently proceeding
    // without base_dir (which would open files at wrong paths).
    let guard = CONFIG.lock().ok()?;
    let result = if let Some(ref cfg) = *guard {
        if let Some(ref base) = cfg.base_dir {
            if path.is_relative() {
                base.join(path)
            } else {
                path.to_path_buf()
            }
        } else {
            path.to_path_buf()
        }
    } else {
        path.to_path_buf()
    };

    Some(result)
}

/// Remove already-opened guise entries from the stream table on partial failure.
///
/// # Safety
///
/// `guises` must point to at least `count` valid `c_int` elements.
unsafe fn rollback_opened(
    table: &mut HashMap<i32, StreamHandle>,
    guises: *const c_int,
    count: usize,
) {
    for j in 0..count {
        table.remove(unsafe { &*guises.add(j) });
    }
}

unsafe extern "C" fn default_ffoprd(
    n_guises: *const c_int,
    guises: *const c_int,
    key_len: *const c_int,
    key: *const c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let n = *n_guises as usize;

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = 1;
                return;
            }
        };

        for i in 0..n {
            let guise = *guises.add(i);
            let path = match resolve_key_for_guise(key, *key_len, guise) {
                Some(p) => p,
                None => {
                    rollback_opened(&mut table, guises, i);
                    *ifail = 1;
                    return;
                }
            };
            let file = match File::open(&path) {
                Ok(f) => f,
                Err(_) => {
                    rollback_opened(&mut table, guises, i);
                    *ifail = 1;
                    return;
                }
            };
            table.insert(guise, StreamHandle::Reader(BufReader::new(file)));
        }
        *ifail = 0;
    }));
    if result.is_err() {
        unsafe { *ifail = 1 };
    }
}

unsafe extern "C" fn default_ffopwr(
    n_guises: *const c_int,
    guises: *const c_int,
    key_len: *const c_int,
    key: *const c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let n = *n_guises as usize;

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = 1;
                return;
            }
        };

        for i in 0..n {
            let guise = *guises.add(i);
            // Each guise opens a separate file with guise-specific extension
            let path = match resolve_key_for_guise(key, *key_len, guise) {
                Some(p) => p,
                None => {
                    rollback_opened(&mut table, guises, i);
                    *ifail = 1;
                    return;
                }
            };
            let file = match File::create(&path) {
                Ok(f) => f,
                Err(_) => {
                    rollback_opened(&mut table, guises, i);
                    *ifail = 1;
                    return;
                }
            };
            table.insert(guise, StreamHandle::Writer(BufWriter::new(file)));
        }
        *ifail = 0;
    }));
    if result.is_err() {
        unsafe { *ifail = 1 };
    }
}

unsafe extern "C" fn default_ffread(
    strid: *const c_int,
    n_chars: *const c_int,
    buffer: *mut c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let id = *strid;
        let count = *n_chars as usize;
        let buf = std::slice::from_raw_parts_mut(buffer as *mut u8, count);

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = 1;
                return;
            }
        };

        match table.get_mut(&id) {
            Some(StreamHandle::Reader(reader)) => match reader.read_exact(buf) {
                Ok(()) => *ifail = 0,
                Err(_) => *ifail = 1,
            },
            _ => *ifail = 1,
        }
    }));
    if result.is_err() {
        unsafe { *ifail = 1 };
    }
}

unsafe extern "C" fn default_ffwrit(
    strid: *const c_int,
    n_chars: *const c_int,
    buffer: *const c_char,
    ifail: *mut c_int,
) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let id = *strid;
        let count = *n_chars as usize;
        let data = std::slice::from_raw_parts(buffer as *const u8, count);

        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = 1;
                return;
            }
        };

        match table.get_mut(&id) {
            Some(StreamHandle::Writer(writer)) => match writer.write_all(data) {
                Ok(()) => *ifail = 0,
                Err(_) => *ifail = 1,
            },
            _ => *ifail = 1,
        }
    }));
    if result.is_err() {
        unsafe { *ifail = 1 };
    }
}

unsafe extern "C" fn default_ffclos(strid: *const c_int, ifail: *mut c_int) {
    let result = catch_unwind(AssertUnwindSafe(|| unsafe {
        let id = *strid;
        let mut table = match STREAM_TABLE.lock() {
            Ok(t) => t,
            Err(_) => {
                *ifail = 1;
                return;
            }
        };
        // Flush writer before closing — propagate flush errors
        if let Some(StreamHandle::Writer(w)) = table.get_mut(&id) {
            if w.flush().is_err() {
                table.remove(&id);
                *ifail = 1;
                return;
            }
        }
        table.remove(&id);
        *ifail = 0;
    }));
    if result.is_err() {
        unsafe { *ifail = 1 };
    }
}
