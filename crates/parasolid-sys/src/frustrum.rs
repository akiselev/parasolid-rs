//! Frustrum callback types and registration functions.
//!
//! The "frustrum" is Parasolid's mandatory callback interface. Applications must register
//! frustrum functions for memory management and file I/O before calling `PK_SESSION_start`.
//! Graphical output (GO) callbacks are optional.
//!
//! Reference: Parasolid Functional Description ch006, Downward Interfaces manual.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_int};

// =============================================================================
// Frustrum callback function pointer types
//
// All frustrum callbacks use the traditional Parasolid by-reference calling
// convention: scalar arguments are passed as `const int*`, output values as
// `int*`, and error status is returned via an `ifail` out-parameter.
// =============================================================================

/// FSTART — start the frustrum (called once at session start).
///
/// `ifail` receives 0 on success, nonzero on failure.
pub type PK_FSTART_f_t = Option<unsafe extern "C" fn(ifail: *mut c_int)>;

/// FSTOP — stop the frustrum (called once at session stop).
///
/// `ifail` receives 0 on success, nonzero on failure.
pub type PK_FSTOP_f_t = Option<unsafe extern "C" fn(ifail: *mut c_int)>;

/// FABORT — called at the end of an aborted kernel operation (optional).
///
/// `ifail` is the error code from the kernel that triggered the abort.
pub type PK_FABORT_f_t = Option<unsafe extern "C" fn(ifail: *const c_int)>;

/// FTMKEY — return sample name keys for testing FFOPRD and FFOPWR (optional).
///
/// - `key_type`: which key type to return (input).
/// - `key_len`: length of returned key string (output).
/// - `key`: buffer to receive key string (output).
/// - `ifail`: 0 on success (output).
pub type PK_FTMKEY_f_t = Option<
    unsafe extern "C" fn(
        key_type: *const c_int,
        key_len: *mut c_int,
        key: *mut c_char,
        ifail: *mut c_int,
    ),
>;

/// FFOPRD — open all guises of a file (except roll-back) for reading.
///
/// - `n_guises`: number of file guises to open (input).
/// - `guises`: array of guise codes (input).
/// - `key_len`: length of key string (input).
/// - `key`: file key (name or database index) (input).
/// - `ifail`: 0 on success (output).
pub type PK_FFOPRD_f_t = Option<
    unsafe extern "C" fn(
        n_guises: *const c_int,
        guises: *const c_int,
        key_len: *const c_int,
        key: *const c_char,
        ifail: *mut c_int,
    ),
>;

/// FFOPWR — open all guises of a file (except roll-back) for writing.
///
/// Same signature as FFOPRD.
pub type PK_FFOPWR_f_t = Option<
    unsafe extern "C" fn(
        n_guises: *const c_int,
        guises: *const c_int,
        key_len: *const c_int,
        key: *const c_char,
        ifail: *mut c_int,
    ),
>;

/// FFREAD — read from file.
///
/// - `strid`: stream id (guise index) identifying which file guise to read (input).
/// - `n_chars`: number of characters/bytes to read (input).
/// - `buffer`: buffer to receive data (output).
/// - `ifail`: 0 on success (output).
pub type PK_FFREAD_f_t = Option<
    unsafe extern "C" fn(
        strid: *const c_int,
        n_chars: *const c_int,
        buffer: *mut c_char,
        ifail: *mut c_int,
    ),
>;

/// FFWRIT — write to file.
///
/// - `strid`: stream id (guise index) identifying which file guise to write (input).
/// - `n_chars`: number of characters/bytes to write (input).
/// - `buffer`: data to write (input).
/// - `ifail`: 0 on success (output).
pub type PK_FFWRIT_f_t = Option<
    unsafe extern "C" fn(
        strid: *const c_int,
        n_chars: *const c_int,
        buffer: *const c_char,
        ifail: *mut c_int,
    ),
>;

/// FFCLOS — close file.
///
/// - `strid`: stream id (guise index) of file to close (input).
/// - `ifail`: 0 on success (output).
pub type PK_FFCLOS_f_t = Option<
    unsafe extern "C" fn(strid: *const c_int, ifail: *mut c_int),
>;

/// FMALLO — allocate virtual memory.
///
/// Parasolid-style memory allocator (not identical to malloc).
/// - `nbytes`: number of bytes to allocate (input).
/// - `memory`: pointer to receive allocated memory (output).
/// - `ifail`: 0 on success (output).
pub type PK_FMALLO_f_t = Option<
    unsafe extern "C" fn(
        nbytes: *const c_int,
        memory: *mut *mut c_char,
        ifail: *mut c_int,
    ),
>;

/// FMFREE — free virtual memory.
///
/// Parasolid-style memory deallocator (not identical to free).
/// - `nbytes`: number of bytes that were allocated (input).
/// - `memory`: pointer to memory to free (input/output, set to NULL on success).
/// - `ifail`: 0 on success (output).
pub type PK_FMFREE_f_t = Option<
    unsafe extern "C" fn(
        nbytes: *const c_int,
        memory: *mut *mut c_char,
        ifail: *mut c_int,
    ),
>;

/// GOOPSG — open a hierarchical graphical segment (optional).
///
/// - `n_data`: number of integers in `data` array (input).
/// - `data`: segment descriptor data (input).
pub type PK_GOOPSG_f_t = Option<
    unsafe extern "C" fn(n_data: *const c_int, data: *const c_int),
>;

/// GOSGMT — output a non-hierarchical (single-level) graphical segment (optional).
///
/// - `n_data`: number of integers in `data` array (input).
/// - `data`: segment descriptor data (input).
pub type PK_GOSGMT_f_t = Option<
    unsafe extern "C" fn(n_data: *const c_int, data: *const c_int),
>;

/// GOCLSG — close a hierarchical graphical segment (optional).
///
/// - `n_data`: number of integers in `data` array (input).
/// - `data`: segment descriptor data (input).
pub type PK_GOCLSG_f_t = Option<
    unsafe extern "C" fn(n_data: *const c_int, data: *const c_int),
>;

// =============================================================================
// PK_SESSION_frustrum_t — v1 frustrum struct
//
// Declared with PK_SESSION_frustrum_o_m() to zero-initialize all fields.
// Fields are function pointers to the application-supplied frustrum callbacks.
// =============================================================================

/// Macro-equivalent initializer: zero all fields to NULL.
///
/// In C this is `PK_SESSION_frustrum_o_m(fru)`. In Rust, use:
// =============================================================================
// PK_SESSION_register_fru_o_t — v2 frustrum registration options
//
// Used with PK_SESSION_register_fru_2. Fields are pointers-to-function-pointers
// (tristate semantics):
//   - NULL outer pointer = don't change this callback
//   - non-NULL outer, NULL inner = clear this callback
//   - non-NULL outer, non-NULL inner = set this callback
// =============================================================================

/// Frustrum registration options (v2 API, tristate semantics).
///
/// Each field is a pointer-to-function-pointer. NULL means "don't change",
/// pointing to NULL means "clear", pointing to a function means "set".
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_register_fru_o_t {
    /// Struct version (set by initializer macro).
    pub o_t_version: c_int,
    /// FSTART callback (tristate).
    pub fstart: *const PK_FSTART_f_t,
    /// FABORT callback (tristate).
    pub fabort: *const PK_FABORT_f_t,
    /// FSTOP callback (tristate).
    pub fstop: *const PK_FSTOP_f_t,
    /// FFOPRD callback (tristate).
    pub ffoprd: *const PK_FFOPRD_f_t,
    /// FFOPWR callback (tristate).
    pub ffopwr: *const PK_FFOPWR_f_t,
    /// FFREAD callback (tristate).
    pub ffread: *const PK_FFREAD_f_t,
    /// FFWRIT callback (tristate).
    pub ffwrit: *const PK_FFWRIT_f_t,
    /// FFCLOS callback (tristate).
    pub ffclos: *const PK_FFCLOS_f_t,
    /// FMALLO callback (tristate).
    pub fmallo: *const PK_FMALLO_f_t,
    /// FMFREE callback (tristate).
    pub fmfree: *const PK_FMFREE_f_t,
    /// FTMKEY callback (tristate).
    pub ftmkey: *const PK_FTMKEY_f_t,
    /// GOOPSG callback (tristate).
    pub goopsg: *const PK_GOOPSG_f_t,
    /// GOSGMT callback (tristate).
    pub gosgmt: *const PK_GOSGMT_f_t,
    /// GOCLSG callback (tristate).
    pub goclsg: *const PK_GOCLSG_f_t,
}

/// Ask-frustrum options (v2 API), used with `PK_SESSION_ask_fru_2`.
///
/// Same tristate pointer-to-function-pointer layout as the registration struct.
/// To retrieve a callback, set the field to point to a local variable initialized
/// to NULL. After the call, the variable will contain the registered callback (or
/// NULL if none). Set the field to NULL to skip retrieval of that callback.
pub type PK_SESSION_ask_fru_o_t = PK_SESSION_register_fru_o_t;

// =============================================================================
// PK_MARK_frustrum_t — mark-level frustrum (rollback support)
// =============================================================================

/// Mark-level frustrum structure (for rollback support).
///
/// Similar layout to `PK_SESSION_frustrum_t` but used per-mark.
/// Registered via `PK_MARK_start` options.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MARK_frustrum_t {
    /// Start the frustrum for this mark. Required.
    pub fstart: PK_FSTART_f_t,
    /// Called on abort. Optional.
    pub fabort: PK_FABORT_f_t,
    /// Stop the frustrum for this mark. Required.
    pub fstop: PK_FSTOP_f_t,
    /// Open file for reading. Required.
    pub ffoprd: PK_FFOPRD_f_t,
    /// Open file for writing. Required.
    pub ffopwr: PK_FFOPWR_f_t,
    /// Read from file. Required.
    pub ffread: PK_FFREAD_f_t,
    /// Write to file. Required.
    pub ffwrit: PK_FFWRIT_f_t,
    /// Close file. Required.
    pub ffclos: PK_FFCLOS_f_t,
    /// Allocate memory. Required.
    pub fmallo: PK_FMALLO_f_t,
    /// Free memory. Required.
    pub fmfree: PK_FMFREE_f_t,
}

impl Default for PK_MARK_frustrum_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// =============================================================================
// PK_MEMORY_frustrum_t — memory-only frustrum
// =============================================================================

/// Memory-only frustrum structure.
///
/// Contains only the memory management callbacks. Registered via
/// `PK_MEMORY_register_callbacks`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MEMORY_frustrum_t {
    /// Allocate memory. Required.
    pub fmallo: PK_FMALLO_f_t,
    /// Free memory. Required.
    pub fmfree: PK_FMFREE_f_t,
}

impl Default for PK_MEMORY_frustrum_t {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

// =============================================================================
// File guise constants
//
// Used in FFOPRD/FFOPWR to indicate which file type is being opened.
// =============================================================================

/// File guise type for FFOPRD/FFOPWR.
pub type PK_FFCXMT_t = c_int;

/// Transmit (part data) file guise — .xmt_txt / .xmt_bin / .X_T / .X_B
pub const PK_FFCXMT_transmit: PK_FFCXMT_t = 1;
/// Schema file guise — .sch_txt / .S_T
pub const PK_FFCXMT_schema: PK_FFCXMT_t = 2;
/// Journal file guise — .jnl_txt / .jnl_bin / .J_T / .J_B
pub const PK_FFCXMT_journal: PK_FFCXMT_t = 3;
/// Snapshot file guise — .snp_txt / .snp_bin / .N_T / .N_B
pub const PK_FFCXMT_snapshot: PK_FFCXMT_t = 4;
/// Partition file guise — .xmp_txt / .xmp_bin / .P_T / .P_B
pub const PK_FFCXMT_partition: PK_FFCXMT_t = 5;
/// Delta transmit file guise — .xmd_txt / .xmd_bin / .D_T / .D_B
pub const PK_FFCXMT_delta: PK_FFCXMT_t = 6;
/// Mesh file guise — .xmm_txt / .xmm_bin / .M_T / .M_B
pub const PK_FFCXMT_mesh: PK_FFCXMT_t = 7;

// =============================================================================
// Extern functions — linked from pskernel.dll
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // -------------------------------------------------------------------------
    // Session-level frustrum registration (v1 API)
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Session-level frustrum registration (v2 API, tristate)
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Mark-level frustrum
    // -------------------------------------------------------------------------

}
