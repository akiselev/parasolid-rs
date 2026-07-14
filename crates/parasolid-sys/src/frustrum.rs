//! Frustrum callback types and registration functions.
//!
//! The "frustrum" is Parasolid's mandatory callback interface. Applications must register
//! frustrum functions for memory management and file I/O before calling `PK_SESSION_start`.
//! Graphical output (GO) callbacks are optional.
//!
//! Reference: Parasolid Functional Description ch006, Downward Interfaces manual.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_double, c_int};

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
/// NOTE: not a member of `PK_SESSION_frustrum_t` — retained for reference only.
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
/// Signature verified against Parasolid V35 `PK_FFOPRD_f_t` header docs.
///
/// - `guise`: class of file — `FFCSNP`, `FFCJNL`, `FFCXMT`, `FFCXMO`, `FFCSCH`, `FFCLNC` (input).
/// - `format`: format code — `FFBNRY` or `FFTEXT` (input).
/// - `name`: key which identifies the file, not NUL-terminated (input).
/// - `namlen`: length of `name` (input).
/// - `skiphd`: `FFSKHD` to skip the file header (usual case) or `FFLVHD` to leave it (input).
/// - `strid`: receives the stream id for subsequent FFREAD/FFCLOS calls (output).
/// - `ifail`: `FR_no_errors` on success, else `FR_bad_name`/`FR_not_found`/`FR_bad_header`/`FR_open_fail` (output).
pub type PK_FFOPRD_f_t = Option<
    unsafe extern "C" fn(
        guise: *const c_int,
        format: *const c_int,
        name: *const c_char,
        namlen: *const c_int,
        skiphd: *const c_int,
        strid: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// FFOPWR — open all guises of a file (except roll-back) for writing.
///
/// Signature verified against Parasolid V35 `PK_FFOPWR_f_t` header docs.
///
/// - `guise`: class of file — `FFCSNP`, `FFCJNL`, `FFCXMT`, `FFCSCH`, `FFCLNC`, `FFCDBG` (input).
/// - `format`: format code — `FFBNRY`, `FFTEXT`, `FFXML` (input).
/// - `name`: key which identifies the file, not NUL-terminated (input).
/// - `namlen`: length of `name` (input).
/// - `pr2hdr`: part 2 header data (`KEYWORD1=value1;KEYWORD2=value2...`) from Parasolid,
///   for storage in the file header, not NUL-terminated (input).
/// - `pr2len`: length of `pr2hdr` (input).
/// - `strid`: receives the stream id for subsequent FFWRIT/FFCLOS calls (output).
/// - `ifail`: `FR_no_errors` on success, else `FR_bad_name`/`FR_already_exists`/`FR_open_fail`/... (output).
pub type PK_FFOPWR_f_t = Option<
    unsafe extern "C" fn(
        guise: *const c_int,
        format: *const c_int,
        name: *const c_char,
        namlen: *const c_int,
        pr2hdr: *const c_char,
        pr2len: *const c_int,
        strid: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// FFREAD — read from file.
///
/// Signature verified against Parasolid V35 `PK_FFREAD_f_t` header docs.
///
/// - `guise`: class of file (input).
/// - `strid`: frustrum stream id from FFOPRD (input).
/// - `nmax`: maximum number of chars to return (input).
/// - `buffer`: receives the read data (output).
/// - `nactual`: actual number of chars placed in `buffer`; equals `*nmax` except on EOF/error (output).
/// - `ifail`: `FR_no_errors`, `FR_read_fail`, or `FR_end_of_file` (only when zero bytes read) (output).
pub type PK_FFREAD_f_t = Option<
    unsafe extern "C" fn(
        guise: *const c_int,
        strid: *const c_int,
        nmax: *const c_int,
        buffer: *mut c_char,
        nactual: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// FFWRIT — write to file.
///
/// Signature verified against Parasolid V35 `PK_FFWRIT_f_t` header docs.
///
/// - `guise`: class of file (input).
/// - `strid`: frustrum stream id from FFOPWR (input).
/// - `nchars`: number of chars/bytes to write, >= 0 (input).
/// - `buffer`: data to write (input).
/// - `ifail`: `FR_no_errors`, `FR_write_fail`, `FR_disc_full`, or `FR_write_memory_full` (output).
pub type PK_FFWRIT_f_t = Option<
    unsafe extern "C" fn(
        guise: *const c_int,
        strid: *const c_int,
        nchars: *const c_int,
        buffer: *const c_char,
        ifail: *mut c_int,
    ),
>;

/// FFCLOS — close file.
///
/// Signature verified against Parasolid V35 `PK_FFCLOS_f_t` header docs.
///
/// - `guise`: class of file (input).
/// - `strid`: frustrum stream id of the file to close (input).
/// - `action`: `FFNORM` (retain a newly created file) or `FFABOR` (delete it) (input).
/// - `ifail`: `FR_no_errors`, `FR_close_fail`, or `FR_write_memory_full` (output).
pub type PK_FFCLOS_f_t = Option<
    unsafe extern "C" fn(
        guise: *const c_int,
        strid: *const c_int,
        action: *const c_int,
        ifail: *mut c_int,
    ),
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

/// FFOPRB — open file for rollback reading (obsolete, use PK_DELTA instead).
pub type PK_FFOPRB_f_t = Option<
    unsafe extern "C" fn(
        n_guises: *const c_int,
        guises: *const c_int,
        key_len: *const c_int,
        key: *const c_char,
        ifail: *mut c_int,
    ),
>;

/// FFSEEK — seek to position in rollback file (obsolete).
pub type PK_FFSEEK_f_t = Option<
    unsafe extern "C" fn(
        strid: *const c_int,
        position: *const c_int,
        ifail: *mut c_int,
    ),
>;

/// FFTELL — return current position in rollback file (obsolete).
pub type PK_FFTELL_f_t = Option<
    unsafe extern "C" fn(
        strid: *const c_int,
        position: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// FFSKXT — open file for reading XT format specifically (legacy variant of FFOPRD).
///
/// NOTE: not a member of `PK_SESSION_frustrum_t` — retained for reference only.
pub type PK_FFSKXT_f_t = Option<
    unsafe extern "C" fn(
        n_guises: *const c_int,
        guises: *const c_int,
        key_len: *const c_int,
        key: *const c_char,
        ifail: *mut c_int,
    ),
>;

/// UCOPRD — open file for Unicode reading (replaces FFOPRD when Unicode enabled).
pub type PK_UCOPRD_f_t = Option<
    unsafe extern "C" fn(
        n_guises: *const c_int,
        guises: *const c_int,
        key_len: *const c_int,
        key: *const c_int,  // Unicode char array, not c_char
        ifail: *mut c_int,
    ),
>;

/// UCOPWR — open file for Unicode writing (replaces FFOPWR when Unicode enabled).
pub type PK_UCOPWR_f_t = Option<
    unsafe extern "C" fn(
        n_guises: *const c_int,
        guises: *const c_int,
        key_len: *const c_int,
        key: *const c_int,  // Unicode char array, not c_char
        ifail: *mut c_int,
    ),
>;

/// GOOPPX — open pixel buffer for shaded images (obsolete).
pub type PK_GOOPPX_f_t = Option<unsafe extern "C" fn(n_data: *const c_int, data: *const c_int)>;

/// GOPIXL — write pixel data for shaded images (obsolete).
pub type PK_GOPIXL_f_t = Option<unsafe extern "C" fn(n_data: *const c_int, data: *const c_int)>;

/// GOCLPX — close pixel buffer for shaded images (obsolete).
pub type PK_GOCLPX_f_t = Option<unsafe extern "C" fn(n_data: *const c_int, data: *const c_int)>;

// =============================================================================
// FG Module Interface callback function pointer types
//
// These callbacks are registered via PK_SESSION_register_frustrum so the kernel
// can call back into application code to initialize and evaluate foreign curves
// and surfaces.  All scalar in/out parameters follow the Parasolid by-reference
// convention (passed as pointers rather than by value).
// =============================================================================

/// FGCRCU — initialize a foreign curve evaluator.
///
/// Called by the kernel when a foreign curve entity is first touched.
/// - `key`      — evaluator key string (input; not NUL-terminated).
/// - `keylen`   — byte length of `key` (in/out; application may shorten).
/// - `n_kii`    — number of integers in `ki_ints` (output).
/// - `ki_ints`  — KI integer array to fill (output).
/// - `n_kir`    — number of reals in `ki_reals` (output).
/// - `ki_reals` — KI real array to fill (output).
/// - `n_data`   — number of doubles in `fg_data` working space (output).
/// - `fg_data`  — working-space array to fill (output).
/// - `ifail`    — 0 on success, nonzero on failure (output).
pub type PK_FGCRCU_f_t = Option<
    unsafe extern "C" fn(
        key: *const c_char,
        keylen: *mut c_int,
        n_kii: *mut c_int,
        ki_ints: *mut c_int,
        n_kir: *mut c_int,
        ki_reals: *mut c_double,
        n_data: *mut c_int,
        fg_data: *mut c_double,
        ifail: *mut c_int,
    ),
>;

/// FGCRSU — initialize a foreign surface evaluator.
///
/// Identical calling convention to `FGCRCU`; used for surface entities.
pub type PK_FGCRSU_f_t = Option<
    unsafe extern "C" fn(
        key: *const c_char,
        keylen: *mut c_int,
        n_kii: *mut c_int,
        ki_ints: *mut c_int,
        n_kir: *mut c_int,
        ki_reals: *mut c_double,
        n_data: *mut c_int,
        fg_data: *mut c_double,
        ifail: *mut c_int,
    ),
>;

/// FGEVCU — evaluate a foreign curve.
///
/// Called by the kernel to compute curve position and derivatives.
/// - `ki_ints`  — KI integer array (input).
/// - `ki_reals` — KI real array (input).
/// - `fg_data`  — evaluator working space (input).
/// - `t`        — parameter value at which to evaluate (input).
/// - `nderiv`   — number of derivatives requested (input).
/// - `results`  — output array: position followed by derivative vectors (output).
/// - `ifail`    — 0 on success (output).
pub type PK_FGEVCU_f_t = Option<
    unsafe extern "C" fn(
        ki_ints: *const c_int,
        ki_reals: *const c_double,
        fg_data: *const c_double,
        t: *const c_double,
        nderiv: *const c_int,
        results: *mut c_double,
        ifail: *mut c_int,
    ),
>;

/// FGEVSU — evaluate a foreign surface.
///
/// Called by the kernel to compute surface position and partial derivatives.
/// - `ki_ints`  — KI integer array (input).
/// - `ki_reals` — KI real array (input).
/// - `fg_data`  — evaluator working space (input).
/// - `u`        — first parameter value (input).
/// - `v`        — second parameter value (input).
/// - `nu`       — number of derivatives in U direction requested (input).
/// - `nv`       — number of derivatives in V direction requested (input).
/// - `triang`   — whether to compute triangular derivative table (input).
/// - `results`  — output array of evaluated values (output).
/// - `ifail`    — 0 on success (output).
pub type PK_FGEVSU_f_t = Option<
    unsafe extern "C" fn(
        ki_ints: *const c_int,
        ki_reals: *const c_double,
        fg_data: *const c_double,
        u: *const c_double,
        v: *const c_double,
        nu: *const c_int,
        nv: *const c_int,
        triang: *const c_int,
        results: *mut c_double,
        ifail: *mut c_int,
    ),
>;

/// FGPRCU — return foreign curve parametrization properties.
///
/// - `ki_ints`  — KI integer array (input).
/// - `ki_reals` — KI real array (input).
/// - `fg_data`  — evaluator working space (input).
/// - `range`    — two-element array [t_min, t_max] (output).
/// - `period`   — nonzero if the curve is periodic (output).
/// - `ifail`    — 0 on success (output).
pub type PK_FGPRCU_f_t = Option<
    unsafe extern "C" fn(
        ki_ints: *const c_int,
        ki_reals: *const c_double,
        fg_data: *const c_double,
        range: *mut c_double,
        period: *mut c_int,
        ifail: *mut c_int,
    ),
>;

/// FGPRSU — return foreign surface parametrization properties.
///
/// - `ki_ints`  — KI integer array (input).
/// - `ki_reals` — KI real array (input).
/// - `fg_data`  — evaluator working space (input).
/// - `range`    — four-element array [u_min, u_max, v_min, v_max] (output).
/// - `period`   — two-element array: nonzero in each direction if periodic (output).
/// - `ifail`    — 0 on success (output).
pub type PK_FGPRSU_f_t = Option<
    unsafe extern "C" fn(
        ki_ints: *const c_int,
        ki_reals: *const c_double,
        fg_data: *const c_double,
        range: *mut c_double,
        period: *mut c_int,
        ifail: *mut c_int,
    ),
>;

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
    // Foreign geometry — curves (tristate)
    pub fgcrcu: *const PK_FGCRCU_f_t,
    pub fgevcu: *const PK_FGEVCU_f_t,
    pub fgprcu: *const PK_FGPRCU_f_t,
    // Foreign geometry — surfaces (tristate)
    pub fgcrsu: *const PK_FGCRSU_f_t,
    pub fgevsu: *const PK_FGEVSU_f_t,
    pub fgprsu: *const PK_FGPRSU_f_t,
    // Rollback file I/O (obsolete, tristate)
    pub ffoprb: *const PK_FFOPRB_f_t,
    pub ffseek: *const PK_FFSEEK_f_t,
    pub fftell: *const PK_FFTELL_f_t,
    pub ffskxt: *const PK_FFSKXT_f_t,
    // Unicode file I/O (tristate)
    pub ucoprd: *const PK_UCOPRD_f_t,
    pub ucopwr: *const PK_UCOPWR_f_t,
    // Shaded images (obsolete, tristate)
    pub gooppx: *const PK_GOOPPX_f_t,
    pub gopixl: *const PK_GOPIXL_f_t,
    pub goclpx: *const PK_GOCLPX_f_t,
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
///
/// FG module callbacks (FGCRCU, FGCRSU, FGEVCU, FGEVSU, FGPRCU, FGPRSU) are not
/// present — per the Parasolid FG User Manual, FG callbacks are registered at
/// session level only via `PK_SESSION_register_frustrum`.
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
// Frustrum tokens and error codes
//
// Values verified against the Parasolid Downward Interfaces manual,
// "Frustrum Tokens and Error Codes" appendix (frustrum_tokens.h /
// frustrum_ifails.h in the Parasolid release area).
// =============================================================================

// --- Frustrum ifail codes (FR_*) ---

/// Operation was successful.
pub const FR_no_errors: c_int = 0;
/// Bad file name.
pub const FR_bad_name: c_int = 1;
/// File of given name does not exist.
pub const FR_not_found: c_int = 2;
/// File of given name already exists.
pub const FR_already_exists: c_int = 3;
/// File pointer is at end of file.
pub const FR_end_of_file: c_int = 4;
/// Unspecified open error.
pub const FR_open_fail: c_int = 10;
/// No space available to extend the file.
pub const FR_disc_full: c_int = 11;
/// Unspecified write error.
pub const FR_write_fail: c_int = 12;
/// Unspecified read error.
pub const FR_read_fail: c_int = 13;
/// Unspecified close error.
pub const FR_close_fail: c_int = 14;
/// Insufficient contiguous virtual memory.
pub const FR_memory_full: c_int = 15;
/// Bad header found opening file for read.
pub const FR_bad_header: c_int = 16;
/// Rollmark operation within frustrum passed.
pub const FR_rollmark_op_pass: c_int = 20;
/// Rollmark operation within frustrum failed.
pub const FR_rollmark_op_fail: c_int = 21;
/// Unspecified error.
pub const FR_unspecified: c_int = 99;

// --- File guise tokens (FFC*) ---

/// Rollback file guise.
pub const FFCROL: c_int = 1;
/// Snapshot file guise — .snp_txt / .snp_bin / .N_T / .N_B
pub const FFCSNP: c_int = 2;
/// Journal file guise — .jnl / .J_T
pub const FFCJNL: c_int = 3;
/// Transmit file guise (generated by Parasolid) — .xmt_txt / .xmt_bin / .X_T / .X_B
pub const FFCXMT: c_int = 4;
/// Transmit file guise (generated by Romulus, old format) — .xmt_txt
pub const FFCXMO: c_int = 5;
/// Schema file guise — .sch_txt / .S_T
pub const FFCSCH: c_int = 6;
/// Licence file guise — .lnc
pub const FFCLNC: c_int = 7;
/// Partition transmit file guise — .xmp_txt / .xmp_bin / .P_T / .P_B
pub const FFCXMP: c_int = 8;
/// Delta transmit file guise — .xmd_txt / .xmd_bin / .D_T / .D_B
pub const FFCXMD: c_int = 9;
/// Debug report file guise.
pub const FFCDBG: c_int = 10;

// --- File format tokens ---

/// Binary format.
pub const FFBNRY: c_int = 1;
/// Text format.
pub const FFTEXT: c_int = 2;
/// Applio format (application I/O).
pub const FFAPPL: c_int = 3;
/// XML text format.
pub const FFXML: c_int = 4;

// --- File open mode tokens (FFOPRD skiphd argument) ---

/// Skip header after opening file for read (usual case).
pub const FFSKHD: c_int = 1;
/// Leave header after opening file for read (frustrum acceptance tests).
pub const FFLVHD: c_int = 2;

// --- File close mode tokens (FFCLOS action argument) ---

/// Normal: default action on file close (retain a newly created file).
pub const FFNORM: c_int = 1;
/// Abort: delete the newly created file.
pub const FFABOR: c_int = 2;
// (end of frustrum tokens)
