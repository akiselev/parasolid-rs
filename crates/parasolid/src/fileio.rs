//! File I/O — transmit and receive Parasolid parts.
//!
//! Parasolid file I/O is partition-scoped: `PK_PART_transmit` writes all
//! entities in a partition to a file keyed by `key`. `PK_PART_receive` reads
//! entities back and returns the partition tags that were created, from which
//! bodies can be retrieved via `PK_PARTITION_ask_bodies`.
//!
//! `PK_PART_t` is a type alias for `PK_ENTITY_t` (i32), and in practice the
//! part tag is the same as the partition tag. Bodies that belong to the same
//! partition are transmitted together.

use std::ffi::CString;
use std::os::raw::c_int;
use parasolid_sys::*;
use crate::error::{PsError, PsResult};
use crate::body::Body;
use crate::memory::PkArray;

// =============================================================================
// transmit
// =============================================================================

/// Transmit bodies to an XT file via frustrum key.
///
/// Each body's partition is resolved with `PK_ENTITY_ask_partition`. Unique
/// partitions are transmitted once per call. The frustrum layer maps `key` to
/// a file path and format (e.g., appending `.x_t` or `.xmt_txt`).
///
/// All bodies in the same partition are written in a single
/// `PK_PART_transmit` call; bodies in different partitions each generate a
/// separate call with the same key.
pub fn transmit(bodies: &[Body], key: &str) -> PsResult<()> {
    let key_cstr = CString::new(key)
        .map_err(|_| PsError::Session("transmit key contains null byte".into()))?;

    // A body IS a part — transmit the body tags directly. (An earlier version
    // resolved each body's partition and transmitted partition tags, which is
    // wrong: `PK_PART_transmit` takes part tags, and `PK_ENTITY_ask_partition`
    // failed with 5048.)
    let part_tags: Vec<PK_PART_t> = bodies.iter().map(|b| b.tag()).collect();

    // Text Parasolid Transmit (`.xmt_txt`). `0` is not a valid format token
    // (the enum starts at 18220), so it must be set explicitly.
    let mut opts = PK_PART_transmit_o_t::default();
    opts.transmit_format = PK_transmit_format_text_c;

    pk_call!(PK_PART_transmit(
        part_tags.len() as c_int,
        part_tags.as_ptr(),
        key_cstr.as_ptr(),
        &opts,
    ));

    Ok(())
}

// =============================================================================
// receive
// =============================================================================

/// Receive bodies from an XT file via frustrum key.
///
/// Calls `PK_PART_receive` to load the file identified by `key`, then
/// iterates each returned part (partition) and collects all bodies via
/// `PK_PARTITION_ask_bodies`.
pub fn receive(key: &str) -> PsResult<Vec<Body>> {
    let key_cstr = CString::new(key)
        .map_err(|_| PsError::Session("receive key contains null byte".into()))?;

    let mut opts = PK_PART_receive_o_t::default();
    opts.transmit_format = PK_transmit_format_text_c;
    let mut n_parts: c_int = 0;
    let mut parts_ptr: *mut PK_PART_t = std::ptr::null_mut();

    pk_call!(PK_PART_receive(
        key_cstr.as_ptr(),
        &opts,
        &mut n_parts,
        &mut parts_ptr,
    ));

    // The returned parts ARE the received bodies/assemblies — return them
    // directly. (An earlier version treated them as partitions and called
    // `PK_PARTITION_ask_bodies`.) For body parts, the part tag is the body.
    let parts = unsafe { PkArray::from_raw(parts_ptr, n_parts) };
    Ok(parts.iter().map(|&tag| Body::from_tag(tag)).collect())
}
