//! Session management, initialization, threading, SMP, and related support functions.
//!
//! Covers the Parasolid session lifecycle (register frustrum, start, stop),
//! session parameters, memory management, tag management, journaling,
//! user fields, session transmit/receive, multi-threading, and SMP.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_double, c_int, c_void};

use crate::*;

// =============================================================================
// PK_SESSION_frustrum_t — frustrum registration structure
// =============================================================================

/// Structure containing all frustrum callback function pointers.
///
/// Initialize all fields to `None` before selectively assigning callbacks.
/// Must be registered via `PK_SESSION_register_frustrum` before `PK_SESSION_start`.
///
/// # Field Ordering
///
/// The field order exactly matches the `PK_SESSION_frustrum_s` definition in
/// the Parasolid V35 header docs (identical from v12 through V35 — the struct
/// is append-stable). Do NOT reorder: the kernel indexes callbacks by struct
/// offset, and a wrong order crashes inside `PK_SESSION_start`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_frustrum_t {
    // Control
    pub fstart: PK_FSTART_f_t,
    pub fabort: PK_FABORT_f_t,
    pub fstop: PK_FSTOP_f_t,
    // Memory
    pub fmallo: PK_FMALLO_f_t,
    pub fmfree: PK_FMFREE_f_t,
    // Graphics
    pub gosgmt: PK_GOSGMT_f_t,
    pub goopsg: PK_GOOPSG_f_t,
    pub goclsg: PK_GOCLSG_f_t,
    // Shaded images (obsolete)
    pub gopixl: PK_GOPIXL_f_t,
    pub gooppx: PK_GOOPPX_f_t,
    pub goclpx: PK_GOCLPX_f_t,
    // File I/O
    pub ffoprd: PK_FFOPRD_f_t,
    pub ffopwr: PK_FFOPWR_f_t,
    pub ffclos: PK_FFCLOS_f_t,
    pub ffread: PK_FFREAD_f_t,
    pub ffwrit: PK_FFWRIT_f_t,
    // Rollback file I/O (obsolete — use PK_DELTA instead)
    pub ffoprb: PK_FFOPRB_f_t,
    pub ffseek: PK_FFSEEK_f_t,
    pub fftell: PK_FFTELL_f_t,
    // Foreign geometry
    pub fgcrcu: PK_FGCRCU_f_t,
    pub fgcrsu: PK_FGCRSU_f_t,
    pub fgevcu: PK_FGEVCU_f_t,
    pub fgevsu: PK_FGEVSU_f_t,
    pub fgprcu: PK_FGPRCU_f_t,
    pub fgprsu: PK_FGPRSU_f_t,
    // Unicode file I/O
    pub ucoprd: PK_UCOPRD_f_t,
    pub ucopwr: PK_UCOPWR_f_t,
}

impl Default for PK_SESSION_frustrum_t {
    fn default() -> Self {
        Self {
            fstart: None,
            fabort: None,
            fstop: None,
            fmallo: None,
            fmfree: None,
            gosgmt: None,
            goopsg: None,
            goclsg: None,
            gopixl: None,
            gooppx: None,
            goclpx: None,
            ffoprd: None,
            ffopwr: None,
            ffclos: None,
            ffread: None,
            ffwrit: None,
            ffoprb: None,
            ffseek: None,
            fftell: None,
            fgcrcu: None,
            fgcrsu: None,
            fgevcu: None,
            fgevsu: None,
            fgprcu: None,
            fgprsu: None,
            ucoprd: None,
            ucopwr: None,
        }
    }
}

// =============================================================================
// Session start options
// =============================================================================

/// Options structure for `PK_SESSION_start`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_start_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Journal filename (null-terminated). NULL = no journaling.
    pub journal_file: *const c_char,
    /// Length of user fields attached to entities (0 = none).
    pub user_field_len: c_int,
}

impl Default for PK_SESSION_start_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            journal_file: std::ptr::null(),
            user_field_len: 0,
        }
    }
}

// =============================================================================
// Session transmit/receive option types
// =============================================================================

/// Transmit marks option: save all session marks.
pub const PK_SESSION_xmt_marks_all_c: c_int = 26221;

/// Transmit deltas option: save all pmarks/deltas in all partitions.
pub const PK_PARTITION_xmt_deltas_all_c: c_int = 18231;

/// Options structure for `PK_SESSION_transmit`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_transmit_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Format for the transmitted snapshot.
    pub transmit_format: c_int,
    /// Whether to include user fields.
    pub transmit_user_fields: PK_LOGICAL_t,
    /// Delta control (e.g. `PK_PARTITION_xmt_deltas_all_c`).
    pub transmit_deltas: c_int,
    /// Mark control (e.g. `PK_SESSION_xmt_marks_all_c`).
    pub transmit_marks: c_int,
}

impl Default for PK_SESSION_transmit_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            transmit_format: 0,
            transmit_user_fields: PK_LOGICAL_false,
            transmit_deltas: PK_PARTITION_xmt_deltas_all_c,
            transmit_marks: PK_SESSION_xmt_marks_all_c,
        }
    }
}

/// Options structure for `PK_SESSION_receive`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_receive_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
}

impl Default for PK_SESSION_receive_o_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

// =============================================================================
// Error callback types
// =============================================================================

/// Error handler callback function pointer.
pub type PK_ERROR_handler_f_t =
    Option<unsafe extern "C" fn(error_code: PK_ERROR_code_t, context: *const c_char)>;

/// Structure for registering error callbacks.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_ERROR_callbacks_t {
    /// Error handler function.
    pub handler: PK_ERROR_handler_f_t,
}

impl Default for PK_ERROR_callbacks_t {
    fn default() -> Self {
        Self { handler: None }
    }
}

// =============================================================================
// PK_ERROR_sf_t — standard form of a PK error
// =============================================================================

impl Default for PK_ERROR_sf_t {
    fn default() -> Self {
        // Layout journal-confirmed (PKU_journal_ERROR_sf): inline char[32] arrays
        // for function/code_token/argument_name, single argument (not a bad_args
        // array). See PK_ERROR_sf_t in error.rs.
        Self {
            function: [0; PK_ERROR_STRING_LEN],
            code: PK_ERROR_no_errors,
            code_token: [0; PK_ERROR_STRING_LEN],
            severity: 0,
            argument_number: 0,
            argument_name: [0; PK_ERROR_STRING_LEN],
            argument_index: 0,
            entity: PK_ENTITY_null,
        }
    }
}

// =============================================================================
// Thread chain types
// =============================================================================

/// Thread chain type enumeration.
pub type PK_THREAD_chain_t = c_int;

/// Not in a chain (returned by `PK_THREAD_is_in_chain`).
pub const PK_THREAD_chain_none_c: PK_THREAD_chain_t = 24922;
/// Exclusive chain — recommended for single-thread applications.
pub const PK_THREAD_chain_exclusive_c: PK_THREAD_chain_t = 24921;

// =============================================================================
// Thread local level constants
// =============================================================================

/// Thread local level type.
pub type PK_THREAD_local_level_t = c_int;

/// Enables consolidated version control at thread-level within a chain.
pub const PK_THREAD_local_versioning_c: PK_THREAD_local_level_t = 26151;

// =============================================================================
// Function run-mode values
// =============================================================================

/// Function run-mode type.
pub type PK_FUNCTION_run_t = c_int;

/// Function always runs concurrently (immutable).
pub const PK_FUNCTION_run_concurrent_c: PK_FUNCTION_run_t = 25410;
/// Function always runs exclusively (immutable).
pub const PK_FUNCTION_run_exclusive_c: PK_FUNCTION_run_t = 25411;
/// Function is running concurrently, can be changed to exclusive.
pub const PK_FUNCTION_run_mutable_conc_c: PK_FUNCTION_run_t = 25413;
/// Function is running as locally exclusive, can be changed to concurrent.
pub const PK_FUNCTION_run_mutable_exc_c: PK_FUNCTION_run_t = 25414;

// =============================================================================
// Function identifier type
// =============================================================================

/// Opaque function identifier returned by `PK_FUNCTION_find`.
pub type PK_FUNCTION_t = c_int;

// =============================================================================
// Memory callback types for thread-level registration
// =============================================================================

/// Memory callbacks structure for `PK_THREAD_register_memory_cbs`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_THREAD_memory_cbs_t {
    pub alloc: PK_FMALLO_f_t,
    pub free: PK_FMFREE_f_t,
}

impl Default for PK_THREAD_memory_cbs_t {
    fn default() -> Self {
        Self {
            alloc: None,
            free: None,
        }
    }
}

// =============================================================================
// SMP constants
// =============================================================================

/// Report record: OS failed to create worker threads.
pub const PK_REPORT_1_osthread_fail_c: c_int = 23901;
/// Report record: OS resumed creating worker threads.
pub const PK_REPORT_1_osthread_ok_c: c_int = 23902;

/// Error returned to waiting threads when the modeller is stopped after a fatal error.
pub const PK_ERROR_modeller_not_started: PK_ERROR_code_t = 2;

// =============================================================================
// SMP facet constant
// =============================================================================

// =============================================================================
// extern "C" blocks
// =============================================================================

/// Kernel version returned by `PK_SESSION_ask_kernel_version`. The first three
/// ints are major/minor/build; the tail is reserved so the kernel never writes
/// past the allocation.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PK_SESSION_kernel_version_t {
    pub major: c_int,
    pub minor: c_int,
    pub build: c_int,
    pub _reserved: [c_int; 5],
}

/// SMP configuration returned by `PK_SESSION_ask_smp`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PK_SESSION_smp_r_t {
    pub thread_format: c_int,
    pub n_threads: c_int,
    pub n_processors: c_int,
    pub _reserved: [c_int; 5],
}

/// `PK_thread_t` tokens (RE: pk-enums.h 4088-4090).
pub const PK_thread_disabled_c: c_int = 21010;
pub const PK_thread_per_processor_c: c_int = 21011;
pub const PK_thread_absolute_c: c_int = 21012;

/// Options for `PK_SESSION_set_smp`.
///
/// RE layout (pk-option-structs.tsv): 16 bytes, 4 fields. `thread_format` is
/// `PK_thread_t` (4-byte enum), `on_single_processor` is `PK_LOGICAL_t`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_smp_o_t {
    pub o_t_version: c_int,
    pub thread_format: c_int,
    pub n_threads: c_int,
    pub on_single_processor: PK_LOGICAL_t,
}

impl Default for PK_SESSION_smp_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            thread_format: PK_thread_per_processor_c,
            n_threads: 0,
            on_single_processor: 0,
        }
    }
}

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // Session lifecycle
    // =========================================================================

    /// Registers frustrum function pointers with Parasolid.
    /// Must be called before `PK_SESSION_start`.
    pub fn PK_SESSION_register_frustrum(
        fru: *const PK_SESSION_frustrum_t,
    ) -> PK_ERROR_code_t;

    /// Starts a Parasolid modeling session.
    /// Frustrum must already be registered.
    pub fn PK_SESSION_start(
        options: *const PK_SESSION_start_o_t,
    ) -> PK_ERROR_code_t;

    /// Stops the current Parasolid session.
    pub fn PK_SESSION_stop() -> PK_ERROR_code_t;

    /// Aborts the Parasolid session (emergency cleanup).
    pub fn PK_SESSION_abort(
        reason: PK_abort_reason_t,
    ) -> PK_ERROR_code_t;

    /// Tidies the Parasolid session after an incomplete operation.
    pub fn PK_SESSION_tidy() -> PK_ERROR_code_t;

    // =========================================================================
    // Error callback registration
    // =========================================================================

    // =========================================================================
    // Session parameter setters
    // =========================================================================

    /// Controls the level of continuity checking performed.
    /// Also controls whether G1-discontinuous geometry can be attached to topology.
    pub fn PK_SESSION_set_check_continuity(level: c_int) -> PK_ERROR_code_t;

    /// Controls whether self-intersecting geometry can be attached to topology.
    pub fn PK_SESSION_set_check_self_int(check: PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Allows general bodies (non-manifold, disconnected) from Boolean operations.
    pub fn PK_SESSION_set_general_topology(allow: PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Defines whether the session can be rolled forward.
    pub fn PK_SESSION_set_roll_forward(enable: PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Controls whether argument checking is performed on PK function calls.
    pub fn PK_SESSION_set_check_arguments(check: PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Sets the session behaviour version.
    /// V35: `(PK_SESSION_behaviour_t behaviour_requested, options, PK_SESSION_behaviour_t *behaviour_set,
    /// PK_SESSION_behaviour_t *behaviour_previous, PK_behaviour_status_t *status)`. The 8-byte
    /// `behaviour_requested` is passed BY VALUE (fits a register on Win64).
    pub fn PK_SESSION_set_behaviour(
        behaviour_requested: PK_SESSION_behaviour_t,
        options: *const PK_SESSION_set_behaviour_o_t,
        behaviour_set: *mut PK_SESSION_behaviour_t,
        behaviour_previous: *mut PK_SESSION_behaviour_t,
        status: *mut PK_behaviour_status_t,
    ) -> PK_ERROR_code_t;

    /// Sets the session angle precision.
    pub fn PK_SESSION_set_angle_precision(precision: c_double) -> PK_ERROR_code_t;

    /// Sets the session linear precision.
    pub fn PK_SESSION_set_precision(precision: c_double) -> PK_ERROR_code_t;

    /// Sets the cellular guise.
    pub fn PK_SESSION_set_cellular_guise(guise: c_int) -> PK_ERROR_code_t;

    /// Controls close-knot handling for B-geometry.
    pub fn PK_SESSION_set_close_knots(mode: c_int) -> PK_ERROR_code_t;

    /// Controls error report generation.
    /// V35: `(PK_ERROR_reports_t error_reports, PK_SESSION_set_err_reports_o_t *options)`.
    pub fn PK_SESSION_set_err_reports(
        error_reports: PK_ERROR_reports_t,
        options: *const PK_SESSION_set_err_reports_o_t,
    ) -> PK_ERROR_code_t;

    /// Controls facet geometry creation mode.
    pub fn PK_SESSION_set_facet_geometry(mode: c_int) -> PK_ERROR_code_t;

    /// Controls mesh angle.
    pub fn PK_SESSION_set_mesh_angle(angle: c_double) -> PK_ERROR_code_t;

    /// Controls rebuild history.
    pub fn PK_SESSION_set_rebuild_history(
        rebuild_history: PK_int_array_t,
        set_history: *mut PK_int_array_t,
        unknown_history: *mut PK_int_array_t,
    ) -> PK_ERROR_code_t;

    /// Sets the software option.
    pub fn PK_SESSION_set_software_option(
        option: PK_SESSION_software_option_t,
        ival: c_int,
    ) -> PK_ERROR_code_t;

    /// Controls swept/spun surface representation.
    pub fn PK_SESSION_set_swept_spun_surfs(mode: c_int) -> PK_ERROR_code_t;

    /// Sets the unicode mode.
    pub fn PK_SESSION_set_unicode(mode: c_int) -> PK_ERROR_code_t;

    // =========================================================================
    // Session parameter getters
    // =========================================================================

    /// Returns the current continuity checking level.
    pub fn PK_SESSION_ask_check_continuity(level: *mut c_int) -> PK_ERROR_code_t;

    /// Returns whether self-intersection checking is enabled.
    pub fn PK_SESSION_ask_check_self_int(check: *mut PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Returns whether general topology is enabled.
    pub fn PK_SESSION_ask_general_topology(allow: *mut PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Returns whether argument checking is enabled.
    pub fn PK_SESSION_ask_check_arguments(check: *mut PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Returns the current session behaviour version.
    /// V35: `(const PK_SESSION_ask_behaviour_o_t *options, PK_SESSION_behaviour_t
    /// *behaviour)` — the old 1-arg form dropped `options`.
    pub fn PK_SESSION_ask_behaviour(
        options: *const PK_SESSION_ask_behaviour_o_t,
        behaviour: *mut PK_SESSION_behaviour_t,
    ) -> PK_ERROR_code_t;

    /// Returns the latest supported behaviour version.
    pub fn PK_SESSION_ask_latest_behaviour(behaviour: *mut PK_SESSION_behaviour_t) -> PK_ERROR_code_t;

    /// Returns the current angle precision.
    pub fn PK_SESSION_ask_angle_precision(precision: *mut c_double) -> PK_ERROR_code_t;

    /// Returns the current linear precision.
    pub fn PK_SESSION_ask_precision(precision: *mut c_double) -> PK_ERROR_code_t;

    /// Returns the current cellular guise.
    pub fn PK_SESSION_ask_cellular_guise(guise: *mut c_int) -> PK_ERROR_code_t;

    /// Returns the current close-knot handling mode.
    pub fn PK_SESSION_ask_close_knots(mode: *mut c_int) -> PK_ERROR_code_t;

    /// Returns whether error reports are enabled.
    pub fn PK_SESSION_ask_err_reports(
        options: *mut PK_SESSION_ask_err_reports_o_t,
        error_reports: *mut PK_ERROR_reports_t,
    ) -> PK_ERROR_code_t;

    /// Returns the current facet geometry mode.
    pub fn PK_SESSION_ask_facet_geometry(mode: *mut c_int) -> PK_ERROR_code_t;

    /// Returns the current mesh angle.
    pub fn PK_SESSION_ask_mesh_angle(angle: *mut c_double) -> PK_ERROR_code_t;

    /// Returns the current rebuild history mode.
    pub fn PK_SESSION_ask_rebuild_history(mode: *mut c_int) -> PK_ERROR_code_t;

    /// Returns the current software option.
    pub fn PK_SESSION_ask_software_option(
        option: PK_SESSION_software_option_t,
        ival: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Returns the current swept/spun surface mode.
    pub fn PK_SESSION_ask_swept_spun_surfs(mode: *mut c_int) -> PK_ERROR_code_t;

    /// Returns the current unicode mode.
    pub fn PK_SESSION_ask_unicode(mode: *mut c_int) -> PK_ERROR_code_t;

    /// Returns the current session binding.
    pub fn PK_SESSION_ask_binding(
        enquiry: PK_SESSION_binding_t,
        answer: *mut PK_SESSION_binding_t,
    ) -> PK_ERROR_code_t;

    /// Returns the registered frustrum.
    pub fn PK_SESSION_ask_frustrum(
        fru: *mut PK_SESSION_frustrum_t,
    ) -> PK_ERROR_code_t;

    /// Returns the registered frustrum (version 2 — extended).
    pub fn PK_SESSION_ask_fru_2(
        fru: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Returns the PK function currently being executed.
    pub fn PK_SESSION_ask_function(
        name: *mut *const c_char,
    ) -> PK_ERROR_code_t;

    /// Returns the Parasolid kernel version.
    /// V35: `(PK_SESSION_kernel_version_t *info)` — a single struct out-param,
    /// not three int pointers. The old 3-pointer form worked only by luck (the
    /// kernel wrote the struct starting at `&major`, spilling into the adjacent
    /// `minor`/`patch` stack slots).
    pub fn PK_SESSION_ask_kernel_version(
        info: *mut PK_SESSION_kernel_version_t,
    ) -> PK_ERROR_code_t;

    /// Returns the Parasolid schema version.
    pub fn PK_SESSION_ask_schema_version(
        version: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Returns all attribute definitions in the current session.
    pub fn PK_SESSION_ask_attdefs(
        options: *mut PK_SESSION_ask_attdefs_o_t,
        n_attdefs: *mut c_int,
        attdefs: *mut *mut PK_ATTDEF_t,
    ) -> PK_ERROR_code_t;

    /// Returns the current partition.
    pub fn PK_SESSION_ask_curr_partition(
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Returns all partitions in the session.
    pub fn PK_SESSION_ask_partitions(
        n_partitions: *mut c_int,
        partitions: *mut *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Returns all parts in the session.
    pub fn PK_SESSION_ask_parts(
        n_parts: *mut c_int,
        parts: *mut *mut PK_PART_t,
    ) -> PK_ERROR_code_t;

    /// Returns the current mark in the session.
    /// Current session mark and whether the modeller is at it. V35:
    /// `(PK_MARK_t *mark, PK_LOGICAL_t *at_mark)` — the old 1-arg binding made
    /// the kernel write `at_mark` through an unpassed argument.
    pub fn PK_SESSION_ask_mark(
        mark: *mut PK_MARK_t,
        at_mark: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Returns the current applio registration.
    pub fn PK_SESSION_ask_applio(
        applio: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Returns the current applio registration (version 2).
    pub fn PK_SESSION_ask_applio_2(
        applio: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Returns the current indexio registration.
    pub fn PK_SESSION_ask_indexio(
        indexio: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Returns whether the session can be rolled forward.
    pub fn PK_SESSION_is_roll_forward_on(
        is_on: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Returns whether rollback is enabled.
    pub fn PK_SESSION_is_rollback_on(
        is_on: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Tag management
    // =========================================================================

    /// Returns the number of tags remaining in the current session.
    pub fn PK_SESSION_ask_tags_remaining(
        n_remaining: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Sets the upper bound of the Parasolid tag number range.
    pub fn PK_SESSION_set_tag_limit(limit: c_int) -> PK_ERROR_code_t;

    /// Returns the upper bound of Parasolid tag numbers.
    pub fn PK_SESSION_ask_tag_limit(limit: *mut c_int) -> PK_ERROR_code_t;

    /// Returns the maximum tag value currently in use.
    pub fn PK_SESSION_ask_tag_highest(highest: *mut c_int) -> PK_ERROR_code_t;

    /// Watches tag creation/deletion for debugging.
    /// V35: `(int n_tags, int *tags, create_fn, destroy_fn)`.
    pub fn PK_SESSION_watch_tags(
        n_tags: c_int,
        tags: *mut c_int,
        create_fn: PK_SESSION_watch_create_cb_t,
        destroy_fn: PK_SESSION_watch_destroy_cb_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Memory management
    // =========================================================================

    /// Returns the amount of memory occupied by the model data structure.
    pub fn PK_SESSION_ask_memory_usage(
        n_bytes: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Returns the amount of memory occupied by a body's data structures.
    // Full vendor form (pk-reference.tsv): the returned `total` output was
    // dropped by clean-prototypes.tsv. size_t → usize on x86_64.
    pub fn PK_BODY_ask_memory_usage(
        body: PK_BODY_t,
        total: *mut usize,
    ) -> PK_ERROR_code_t;

    /// Sets the minimum contiguous memory block size Parasolid requests via FMALLO.
    /// Range: ~1/8 MB (default) to 16 MB. Zero resets to default.
    pub fn PK_MEMORY_set_block_size(size: c_int) -> PK_ERROR_code_t;

    /// Returns the current minimum memory block size.
    pub fn PK_MEMORY_ask_block_size(size: *mut c_int) -> PK_ERROR_code_t;

    // =========================================================================
    // Journaling
    // =========================================================================

    /// Turns journaling on or off within a session.
    pub fn PK_SESSION_set_journalling(enable: PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Returns whether journaling is currently enabled.
    pub fn PK_SESSION_ask_journalling(enabled: *mut PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Adds a comment to the journal file (only effective when journaling is on).
    pub fn PK_SESSION_comment(
        comment: *const c_char,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // User fields
    // =========================================================================

    /// Sets the user field data for an entity.
    // Vendor form: `const int *user_field` (input, read-only).
    pub fn PK_ENTITY_set_user_field(
        entity: PK_ENTITY_t,
        user_field: *const c_int,
    ) -> PK_ERROR_code_t;

    /// Reads the user field data for an entity.
    pub fn PK_ENTITY_ask_user_field(
        entity: PK_ENTITY_t,
        user_field: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Returns the current user field length setting.
    pub fn PK_SESSION_ask_user_field_len(
        len: *mut c_int,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Session transmit / receive (snapshot)
    // =========================================================================

    /// Creates a snapshot of the session.
    pub fn PK_SESSION_transmit(
        key: *mut c_char,
        options: *mut PK_SESSION_transmit_o_t,
    ) -> PK_ERROR_code_t;

    /// Creates a snapshot of the session (unicode variant).
    pub fn PK_SESSION_transmit_u(
        key: *mut PK_UCHAR_t,
        options: *mut PK_SESSION_transmit_o_t,
    ) -> PK_ERROR_code_t;

    /// Restores a snapshot into the same Parasolid version that transmitted it.
    pub fn PK_SESSION_receive(
        key: *mut c_char,
        options: *mut PK_SESSION_receive_o_t,
    ) -> PK_ERROR_code_t;

    /// Restores a snapshot (unicode variant).
    pub fn PK_SESSION_receive_u(
        key: *mut PK_UCHAR_t,
        options: *mut PK_SESSION_receive_o_t,
    ) -> PK_ERROR_code_t;

    /// Returns version info about the Parasolid that created a session transmit file.
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_SESSION_receive_version(
        key: *mut c_char,
        transmit_format: PK_transmit_format_t,
        version: *mut PK_SESSION_kernel_version_t,
    ) -> PK_ERROR_code_t;

    /// Returns version info about a session transmit file (unicode variant).
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_SESSION_receive_version_u(
        key: *mut PK_UCHAR_t,
        transmit_format: PK_transmit_format_t,
        version: *mut PK_SESSION_kernel_version_t,
    ) -> PK_ERROR_code_t;

    /// Registers application frustrum functions for "applio" transmit format.
    pub fn PK_SESSION_register_applio_2(
        applio: *const c_void,
    ) -> PK_ERROR_code_t;

    /// Registers application frustrum functions for "applio" transmit format (version 1).
    pub fn PK_SESSION_register_applio(
        applio: *const c_void,
    ) -> PK_ERROR_code_t;

    /// Registers the indexio frustrum.
    pub fn PK_SESSION_register_indexio(
        indexio: *const c_void,
    ) -> PK_ERROR_code_t;

    /// Registers the extended frustrum (version 2).
    pub fn PK_SESSION_register_fru_2(
        fru: *const c_void,
    ) -> PK_ERROR_code_t;

    /// Registers session (general registration call).
    pub fn PK_SESSION_register(
        code: c_int,
        data: PK_POINTER_t,
    ) -> PK_ERROR_code_t;

    /// Registers a polling callback for long operations.
    pub fn PK_SESSION_register_polling_cb(
        callback: *const c_void,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Kernel state queries
    // =========================================================================

    /// Returns whether called from within the Parasolid kernel or from outside.
    pub fn PK_SESSION_is_in_kernel(
        is_in_kernel: *mut PK_LOGICAL_t,
        is_protected: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Returns whether called from within the Parasolid kernel (version 2).
    pub fn PK_SESSION_is_in_kernel_2(
        is_in_kernel: *mut PK_LOGICAL_t,
        is_protected: *mut PK_LOGICAL_t,
        is_subthread: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // SMP (Symmetric Multi-Processing)
    // =========================================================================

    /// Enables or disables Parasolid SMP.
    ///
    /// V35: `(PK_SESSION_smp_o_t *options)` — the thread count is a field of the
    /// options struct, NOT a bare int. The old `(int n_threads)` binding passed
    /// the count where the kernel dereferences a pointer.
    pub fn PK_SESSION_set_smp(
        options: *const PK_SESSION_smp_o_t,
    ) -> PK_ERROR_code_t;

    /// Returns the current SMP parameters: thread format, thread count, processor count.
    /// V35: `(PK_SESSION_smp_r_t *results)` — a single struct out-param. The old
    /// 3-pointer form worked only by luck (struct written across adjacent slots).
    pub fn PK_SESSION_ask_smp(results: *mut PK_SESSION_smp_r_t) -> PK_ERROR_code_t;

    /// Returns the maximum number of SMP threads Parasolid can use.
    /// Returns 1 if SMP is disabled.
    pub fn PK_SESSION_ask_max_threads(
        options: *mut PK_SESSION_ask_max_threads_o_t,
        max_threads: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Sets the stack size for Parasolid SMP threads. Zero = platform default.
    pub fn PK_SESSION_set_smp_stacksize(size: c_int) -> PK_ERROR_code_t;

    /// Returns the stack size allocated to Parasolid SMP threads. Zero = platform default.
    pub fn PK_SESSION_ask_smp_stacksize(size: *mut c_int) -> PK_ERROR_code_t;

    // =========================================================================
    // Debug shuffle (for SMP robustness testing)
    // =========================================================================

    // =========================================================================
    // Thread management
    // =========================================================================

    /// Returns the error handler registered for the calling thread, or NULL.
    pub fn PK_THREAD_ask_error_cbs(
        callbacks: *mut PK_ERROR_callbacks_t,
    ) -> PK_ERROR_code_t;

    /// Returns whether the PK interface is excluding other threads,
    /// and whether the calling thread caused the exclusion.
    pub fn PK_THREAD_ask_exclusion(
        is_excluded: *mut PK_LOGICAL_t,
        is_caller: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Returns the name of the PK function executing in the calling thread
    /// and the total recursion depth.
    pub fn PK_THREAD_ask_function(
        depth: c_int,
        fn_name: *mut *mut c_char,
        total_depth: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Returns the standard form of the most recent PK error in the calling thread.
    pub fn PK_THREAD_ask_last_error(
        was_error: *mut PK_LOGICAL_t,
        error_sf: *mut PK_ERROR_sf_t,
    ) -> PK_ERROR_code_t;

    /// Returns the memory callbacks registered for the calling thread.
    pub fn PK_THREAD_ask_memory_cbs(
        cbs: *mut PK_THREAD_memory_cbs_t,
    ) -> PK_ERROR_code_t;

    /// Returns all partitions locked to the calling thread.
    pub fn PK_THREAD_ask_partitions(
        options: *mut PK_THREAD_ask_partitions_o_t,
        n_partitions: *mut c_int,
        partitions: *mut *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    /// Starts chaining PK functions in the calling thread.
    pub fn PK_THREAD_chain_start(
        r#type: PK_THREAD_chain_type_t,
        options: *mut PK_THREAD_chain_start_o_t,
    ) -> PK_ERROR_code_t;

    /// Stops chaining PK functions in the calling thread.
    pub fn PK_THREAD_chain_stop(
        options: *mut PK_THREAD_chain_stop_o_t,
    ) -> PK_ERROR_code_t;

    /// Tries to clear an exclusion preventing other threads from entering Parasolid.
    pub fn PK_THREAD_clear_exclusion(
        clear: PK_THREAD_exclusion_t,
        exclusion: *mut PK_THREAD_exclusion_t,
        was_this_thread: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Clears the most recent PK error in the calling thread.
    pub fn PK_THREAD_clear_last_error(
        was_error: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Returns whether the calling thread is executing inside the kernel.
    pub fn PK_THREAD_is_in_kernel(
        is_in_kernel: *mut PK_LOGICAL_t,
        is_protected: *mut PK_LOGICAL_t,
        is_subthread: *mut PK_LOGICAL_t,
        is_excluding: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Returns whether the calling thread is in a chain, and chain properties.
    pub fn PK_THREAD_is_in_chain(
        chain_type: *mut PK_THREAD_chain_t,
        length: *mut c_int,
        local_level: *mut PK_THREAD_local_level_t,
    ) -> PK_ERROR_code_t;

    /// Locks specified partitions to the calling thread.
    /// V35: `(n_partitions, partitions, PK_THREAD_lock_t lock_type, PK_THREAD_wait_t wait_type, options, results)`.
    pub fn PK_THREAD_lock_partitions(
        n_partitions: c_int,
        partitions: *const PK_PARTITION_t,
        lock_type: PK_THREAD_lock_t,
        wait_type: PK_THREAD_wait_t,
        options: *const PK_THREAD_lock_partitions_o_t,
        results: *mut PK_THREAD_lock_partitions_r_t,
    ) -> PK_ERROR_code_t;

    /// Registers error callbacks for the calling thread.
    pub fn PK_THREAD_register_error_cbs(
        callbacks: *const PK_ERROR_callbacks_t,
    ) -> PK_ERROR_code_t;

    /// Registers memory allocation/free callbacks for the calling thread.
    pub fn PK_THREAD_register_memory_cbs(
        cbs: *const PK_THREAD_memory_cbs_t,
    ) -> PK_ERROR_code_t;

    /// Restores Parasolid to a valid state for the calling thread
    /// after a PK function has not completed (e.g. longjmp from error handler).
    pub fn PK_THREAD_tidy() -> PK_ERROR_code_t;

    /// Unlocks and returns partitions locked to the calling thread.
    pub fn PK_THREAD_unlock_partitions(
        options: *mut PK_THREAD_unlock_partitions_o_t,
        n_partitions: *mut c_int,
        partitions: *mut *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Thread function exclusivity control
    // =========================================================================

    /// Returns run-mode values for the specified functions.
    pub fn PK_THREAD_ask_function_run(
        n_functions: c_int,
        functions: *mut PK_FUNCTION_t,
        options: *mut PK_THREAD_ask_function_run_o_t,
        run_values: *mut PK_FUNCTION_run_t,
    ) -> PK_ERROR_code_t;

    /// Changes the exclusivity of mutable functions.
    pub fn PK_THREAD_set_function_run(
        n_functions: c_int,
        functions: *mut PK_FUNCTION_t,
        run_values: *mut PK_FUNCTION_run_t,
        options: *mut PK_THREAD_set_function_run_o_t,
    ) -> PK_ERROR_code_t;

    /// Identifies PK functions by name for use with `PK_THREAD_ask_function_run`.
    pub fn PK_FUNCTION_find(
        n_function_names: c_int,
        function_names: *mut *mut c_char,
        options: *mut PK_FUNCTION_find_o_t,
        functions: *mut PK_FUNCTION_t,
    ) -> PK_ERROR_code_t;

    /// Returns the localisation level of a calling thread chain.
    pub fn PK_THREAD_ask_local_level(
        options: *mut PK_THREAD_ask_local_level_o_t,
        local_level: *mut PK_THREAD_local_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Thread error report and ID management (from exports)
    // =========================================================================

    /// Returns error reports for the calling thread.
    pub fn PK_THREAD_ask_err_reports(
        options: *mut PK_THREAD_ask_err_reports_o_t,
        error_reports: *mut PK_ERROR_reports_t,
    ) -> PK_ERROR_code_t;

    /// Sets error report generation for the calling thread.
    pub fn PK_THREAD_set_err_reports(
        error_reports: PK_ERROR_reports_t,
        options: *mut PK_THREAD_set_err_reports_o_t,
    ) -> PK_ERROR_code_t;

    /// Returns the thread identifier of the calling thread.
    pub fn PK_THREAD_ask_id(
        thread_id: *mut PK_THREAD_id_t,
        parasolid_id: *mut PK_THREAD_id_t,
        is_subthread: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Sets the thread identifier for the calling thread.
    pub fn PK_THREAD_set_id(
        thread_id: PK_THREAD_id_t,
        options: *mut PK_THREAD_set_id_o_t,
        results: *mut PK_THREAD_set_id_r_t,
    ) -> PK_ERROR_code_t;
}
