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

// =============================================================================
// transmit / receive through application memory (`_b` calls)
// =============================================================================

/// Transmit bodies to XT bytes in memory (`PK_PART_transmit_b`).
///
/// Returns the kernel's XT stream exactly as written: for
/// [`TransmitFormat::Text`] it begins `T51 : TRANSMIT FILE created by …`.
/// Unlike a file written through the frustrum it has **no** `**ABCDEF…` /
/// `**END_OF_HEADER` container header; that header is a frustrum concern.
/// A multi-link chain is concatenated in order. The kernel's chain is freed
/// with `PK_MEMORY_block_f` before returning.
pub fn transmit_to_bytes(bodies: &[Body], format: TransmitFormat) -> PsResult<Vec<u8>> {
    let mut part_tags: Vec<PK_PART_t> = bodies.iter().map(|b| b.tag()).collect();
    let mut opts = PK_PART_transmit_o_t::default();
    opts.transmit_format = format.token();
    let mut block = PK_MEMORY_block_t {
        next: std::ptr::null_mut(),
        n_bytes: 0,
        bytes: std::ptr::null(),
    };

    pk_call!(PK_PART_transmit_b(
        part_tags.len() as c_int,
        part_tags.as_mut_ptr(),
        &mut opts,
        &mut block,
    ));

    let mut out = Vec::new();
    let mut link: *const PK_MEMORY_block_t = &block;
    while !link.is_null() {
        // SAFETY: the kernel filled a well-formed chain (layout verified in
        // parasolid-sys `PK_MEMORY_block_t`); each link owns `n_bytes` bytes.
        let l = unsafe { &*link };
        if l.n_bytes > 0 && !l.bytes.is_null() {
            let s = unsafe { std::slice::from_raw_parts(l.bytes as *const u8, l.n_bytes as usize) };
            out.extend_from_slice(s);
        }
        link = l.next;
    }
    pk_call!(PK_MEMORY_block_f(&mut block));
    Ok(out)
}

/// The frustrum container header a file on disk starts with, if `data` has one:
/// returns the bytes after the `**END_OF_HEADER…` line. Same rule as the
/// frustrum's FFOPRD header skip: only when `data` starts with `**` and the
/// marker line ends within the first 64 KiB; otherwise `data` is unchanged.
fn strip_container_header(data: &[u8]) -> &[u8] {
    const END: &[u8] = b"**END_OF_HEADER";
    if !data.starts_with(b"**") {
        return data;
    }
    let scan = &data[..data.len().min(65536)];
    let Some(pos) = scan.windows(END.len()).position(|w| w == END) else {
        return data;
    };
    match data[pos..].iter().position(|&b| b == b'\n') {
        Some(nl) => &data[pos + nl + 1..],
        None => &data[data.len()..],
    }
}

/// Receive bodies from XT bytes in memory (`PK_PART_receive_b`).
///
/// `data` is either the bare kernel stream (as from [`transmit_to_bytes`]) or
/// the content of an `.x_t`/`.xmt_txt` file with its `**…**END_OF_HEADER`
/// container header, which is stripped first: `PK_PART_receive_b` itself
/// rejects the header with `PK_ERROR_corrupt_file` (922, parasolid-re R-49).
///
/// `format` must name the encoding: `PK_transmit_format_t` 0 is not a legal
/// token for this call (5014 `field_of_wrong_type`, R-49). Empty input is
/// refused without calling the kernel.
pub fn receive_from_bytes(data: &[u8], format: TransmitFormat) -> PsResult<Vec<Body>> {
    let stream = strip_container_header(data);
    if stream.is_empty() {
        return Err(PsError::Session("receive_from_bytes: no XT data".into()));
    }
    let block = PK_MEMORY_block_t {
        next: std::ptr::null_mut(),
        n_bytes: stream.len() as u64,
        bytes: stream.as_ptr() as *const std::os::raw::c_char,
    };
    let mut opts = PK_PART_receive_o_t::default();
    opts.transmit_format = format.token();
    let mut n_parts: c_int = 0;
    let mut parts_ptr: *mut PK_PART_t = std::ptr::null_mut();

    pk_call!(PK_PART_receive_b(
        block,
        &opts,
        &mut n_parts,
        &mut parts_ptr
    ));

    let parts = unsafe { PkArray::from_raw(parts_ptr, n_parts) };
    Ok(parts.iter().map(|&tag| Body::from_tag(tag)).collect())
}

/// Receive bodies from XT text in memory: [`receive_from_bytes`] with
/// [`TransmitFormat::Text`].
pub fn receive_from_str(text: &str) -> PsResult<Vec<Body>> {
    receive_from_bytes(text.as_bytes(), TransmitFormat::Text)
}
