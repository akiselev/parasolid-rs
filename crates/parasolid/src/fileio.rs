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

use crate::body::Body;
use crate::error::{PsError, PsResult};
use crate::memory::PkArray;
use parasolid_sys::*;
use std::ffi::CString;
use std::os::raw::c_int;

// =============================================================================
// transmit
// =============================================================================

/// Public-PK transmit encodings supported by the native frustrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransmitFormat {
    Text,
    BareBinary,
    NeutralBinary,
    TypedBinary,
}

impl TransmitFormat {
    fn token(self) -> PK_transmit_format_t {
        match self {
            Self::Text => PK_transmit_format_text_c,
            Self::BareBinary => PK_transmit_format_binary_c,
            Self::NeutralBinary => PK_transmit_format_neutral_c,
            Self::TypedBinary => PK_transmit_format_typed_binary_c,
        }
    }
}

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
    transmit_with_format(bodies, key, TransmitFormat::Text)
}

/// Transmit bodies to XT using an explicit text or binary encoding.
pub fn transmit_with_format(bodies: &[Body], key: &str, format: TransmitFormat) -> PsResult<()> {
    let key_cstr = CString::new(key)
        .map_err(|_| PsError::Session("transmit key contains null byte".into()))?;

    // A body IS a part — transmit the body tags directly. (An earlier version
    // resolved each body's partition and transmitted partition tags, which is
    // wrong: `PK_PART_transmit` takes part tags, and `PK_ENTITY_ask_partition`
    // failed with 5048.)
    let part_tags: Vec<PK_PART_t> = bodies.iter().map(|b| b.tag()).collect();

    let mut opts = PK_PART_transmit_o_t::default();
    opts.transmit_format = format.token();

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
    let key_cstr =
        CString::new(key).map_err(|_| PsError::Session("receive key contains null byte".into()))?;

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
