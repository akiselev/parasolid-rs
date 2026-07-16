#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! Error handling, signal handling, and thread-safe error functions for the
//! Parasolid PK_* C API (chapters 117-119).

use std::os::raw::{c_char, c_int, c_void};

use crate::*;

// =============================================================================
// Return type alias
// =============================================================================

/// Every PK function returns this type. 0 = success, non-zero = error code.
pub type PK_ERROR_t = PK_ERROR_code_t;

// =============================================================================
// Additional error code constants (beyond those in types.rs)
// =============================================================================

/// General system error; Parasolid failed unexpectedly.
pub const PK_ERROR_system_error: PK_ERROR_code_t = 2;

/// Fatal error; session so corrupted no further PK calls are possible.
pub const PK_ERROR_fatal_error: PK_ERROR_code_t = 3;

/// Unforeseeable error Parasolid cannot diagnose internally.
pub const PK_ERROR_unhandleable_condition: PK_ERROR_code_t = 4;

/// Run-time error in Parasolid kernel being processed by signal handler.
pub const PK_ERROR_run_time_error: PK_ERROR_code_t = 5;

/// PK_SESSION_abort called by signal handler due to user interrupt.
pub const PK_ERROR_aborted: PK_ERROR_code_t = 6;

/// User interrupt attempted during a PK function that cannot be safely aborted.
pub const PK_ERROR_cant_be_aborted: PK_ERROR_code_t = 7;

/// Negative radius, height, or similar geometric parameter.
pub const PK_ERROR_distance_le_0: PK_ERROR_code_t = 502;

/// Run-time error occurred in a frustrum function (not PK kernel).
pub const PK_ERROR_fru_error: PK_ERROR_code_t = 8;

// =============================================================================
// Error severity constants
// =============================================================================

pub type PK_ERROR_severity_t = c_int;

/// No error severity (returned when no error exists).
pub const PK_ERROR_none: PK_ERROR_severity_t = 0;

/// Mild — operation failed, parts not altered. Continue normally.
pub const PK_ERROR_mild: PK_ERROR_severity_t = 1;

/// Serious — parts may be altered/invalid, rest of session intact.
pub const PK_ERROR_serious: PK_ERROR_severity_t = 2;

/// Fatal — session corrupted; rollback ineffective.
pub const PK_ERROR_fatal: PK_ERROR_severity_t = 3;

// =============================================================================
// PK_ERROR_sf_t — error information structure
// =============================================================================

/// Length of each inline character-array field in `PK_ERROR_sf_t`
/// (function, code_token, argument_name). Journal-confirmed: each string field
/// spans exactly 0x20 bytes to the following int field.
pub const PK_ERROR_STRING_LEN: usize = 32;

/// Error information structure returned by PK_ERROR_ask_last and passed to
/// error handler callbacks.
///
/// Layout: `PKU_journal_ERROR_sf` (V37.01.243) — the string fields are **inline
/// fixed-size char arrays**, NOT `*const c_char` pointers, and there is a single
/// bad argument (number/name/index), NOT an array of 20. Byte offsets:
/// - `function`        @0   — inline name of the PK function that raised the error
/// - `code`            @32  — the error code
/// - `code_token`      @36  — inline error-code token string
/// - `severity`        @68  — PK_ERROR_mild / PK_ERROR_serious / PK_ERROR_fatal
/// - `argument_number` @72  — 1-based index of the invalid argument (0 if none)
/// - `argument_name`   @76  — inline name of the invalid argument
/// - `argument_index`  @108 — index within the argument (e.g. array element)
/// - `entity`          @112 — entity to which the error applies
///
/// Total size: 116 bytes. (The old binding modelled `function`/`bad_arg_names`
/// as pointers and carried a phantom `bad_args[20]`; reading the inline strings
/// as pointers dereferences ASCII bytes as an address and page-faults.)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_ERROR_sf_t {
    pub function: [c_char; PK_ERROR_STRING_LEN],       // @0
    pub code: PK_ERROR_code_t,                         // @32
    pub code_token: [c_char; PK_ERROR_STRING_LEN],     // @36
    pub severity: PK_ERROR_severity_t,                 // @68
    pub argument_number: c_int,                        // @72
    pub argument_name: [c_char; PK_ERROR_STRING_LEN],  // @76
    pub argument_index: c_int,                         // @108
    pub entity: PK_ENTITY_t,                           // @112
} // 116 bytes

// =============================================================================
// Error handler callback type
// =============================================================================

/// Error handler callback signature.
///
/// The return value is not used by Parasolid. The handler must NOT modify the
/// PK_ERROR_sf_t structure.
pub type PK_ERROR_handler_fn_t =
    Option<unsafe extern "C" fn(error_sf: *const PK_ERROR_sf_t) -> PK_ERROR_code_t>;

// =============================================================================
// PK_ERROR_frustrum_t — error handler registration structure
// =============================================================================

/// Structure for registering an error handler with Parasolid.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_ERROR_frustrum_t {
    pub handler_fn: PK_ERROR_handler_fn_t,
}

// =============================================================================
// Abort reason type and constants (signal handling, Ch 119)
// =============================================================================

pub type PK_abort_reason_t = c_int;

/// Abort due to run-time error in PK code.
pub const PK_abort_runtime_error_c: PK_abort_reason_t = 2;

/// Abort due to run-time error in frustrum code.
pub const PK_abort_frustrum_error_c: PK_abort_reason_t = 3;

/// Abort due to user interrupt.
pub const PK_abort_user_interrupt_c: PK_abort_reason_t = 1;

// =============================================================================
// Failure status code types (returned via output arguments, not error codes)
// =============================================================================

pub type PK_blend_fault_t = c_int;
pub type PK_BODY_fault_t = c_int;
pub type PK_local_status_t = c_int;
pub type PK_section_report_t = c_int;
// [re-abi] appended 9 missing member(s) from pk-enums.h
pub const PK_section_report_imprint_fail_c: PK_section_report_t = 21606;
pub const PK_section_report_intersect_fail_c: PK_section_report_t = 21607;
pub const PK_section_report_invalid_face_c: PK_section_report_t = 21608;
pub const PK_section_report_failure_c: PK_section_report_t = 21700;
pub const PK_section_report_solid_has_void_c: PK_section_report_t = 21701;
pub const PK_section_report_partial_coi_c: PK_section_report_t = 21702;
pub const PK_section_report_t_sheet_c: PK_section_report_t = 21703;
pub const PK_section_report_non_manifold_c: PK_section_report_t = 21704;
pub const PK_section_report_invalid_match_c: PK_section_report_t = 21705;

// =============================================================================
// Extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // -------------------------------------------------------------------------
    // Error query / manipulation
    // -------------------------------------------------------------------------

    /// Returns info about the last PK error.
    pub fn PK_ERROR_ask_last(
        was_error: *mut PK_LOGICAL_t,
        error_sf: *mut PK_ERROR_sf_t,
    ) -> PK_ERROR_t;

    /// Clears information about the last PK error.
    pub fn PK_ERROR_clear_last(
        was_error: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Raises an artificial PK error; invokes the registered error handler.
    pub fn PK_ERROR_raise(error_sf: *const PK_ERROR_sf_t) -> PK_ERROR_t;

    /// Re-raises the last error.
    pub fn PK_ERROR_reraise(was_error: *mut PK_LOGICAL_t) -> PK_ERROR_t;

    /// Registers an error handler callback.
    pub fn PK_ERROR_register_callbacks(
        frustrum: *const PK_ERROR_frustrum_t,
    ) -> PK_ERROR_t;

    /// Returns the current error handler, or null fields if none registered.
    pub fn PK_ERROR_ask_callbacks(
        frustrum: *mut PK_ERROR_frustrum_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Thread-safe error functions
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Signal handling (Ch 119)
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Session functions used in error recovery
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Tag validity checking
    // -------------------------------------------------------------------------

    /// Whether tag refers to a living entity.
    pub fn PK_ENTITY_is(
        entity: PK_ENTITY_t,
        is_entity: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Partition mark (rollback) — used in error recovery
    // -------------------------------------------------------------------------

    // -------------------------------------------------------------------------
    // Memory
    // -------------------------------------------------------------------------

    /// Free PK-allocated memory.
    pub fn PK_MEMORY_free(ptr: *mut c_void) -> PK_ERROR_t;
}
