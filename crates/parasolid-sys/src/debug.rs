#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! Debug, version control, and update switch bindings for the Parasolid PK_* C API.
//!
//! Covers:
//! - PK_DEBUG_* functions (chapter 99): session checking, body comparison,
//!   debug reports, behaviour debugging, shuffle testing, watch callbacks
//! - PK_SESSION_behaviour_t and related constants (chapters 114-115)
//! - PK_SESSION_software_option_t and session-level switch constants
//! - Update switch structures (chapter 116): per-function version control options

use std::os::raw::{c_char, c_int};

use crate::*;

// =============================================================================
// Version control — behaviour types (consolidated system, V28.1+)
// =============================================================================

/// Behaviour type selector for `PK_SESSION_behaviour_t`.
pub type PK_SESSION_behave_as_t = c_int;

/// Use the latest behaviour for the current Parasolid version.
pub const PK_SESSION_behave_as_latest_c: PK_SESSION_behave_as_t = 0;

/// Use behaviour from a specific patch release. `behaviour_value` format: MMmmpp.
pub const PK_SESSION_behave_as_value_c: PK_SESSION_behave_as_t = 1;

/// Use original system (session switches + update switches). Initial default.
pub const PK_SESSION_behave_as_unset_c: PK_SESSION_behave_as_t = 2;

/// Session behaviour descriptor.
///
/// Used with `PK_SESSION_set_behaviour`, `PK_SESSION_ask_behaviour`,
/// and `PK_SESSION_ask_latest_behaviour`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_behaviour_t {
    /// One of `PK_SESSION_behave_as_*` constants.
    pub behaviour_type: PK_SESSION_behave_as_t,
    /// Version value in MMmmpp format (e.g. 280103). Only meaningful when
    /// `behaviour_type == PK_SESSION_behave_as_value_c`.
    pub behaviour_value: c_int,
}

impl Default for PK_SESSION_behaviour_t {
    fn default() -> Self {
        Self {
            behaviour_type: PK_SESSION_behave_as_unset_c,
            behaviour_value: 0,
        }
    }
}

/// Behaviour status returned by `PK_SESSION_set_behaviour`.
pub type PK_behaviour_status_t = c_int;

/// Behaviour was set as requested.
pub const PK_behaviour_status_ok_c: PK_behaviour_status_t = 0;

/// Requested behaviour not known; Parasolid set closest match before the requested value.
pub const PK_behaviour_status_unknown_c: PK_behaviour_status_t = 1;

// =============================================================================
// Session-level switch constants (original system, PK_SESSION_software_option_t fields)
// =============================================================================

/// Session software option enumeration — identifies a session-level switch.
///
/// Used with `PK_SESSION_set_software_option` and `PK_SESSION_ask_software_option`.
/// Each constant corresponds to a backward-compatibility switch that emulates
/// older Parasolid behaviour in a specific functional area.
pub type PK_SESSION_sw_t = c_int;

// Restricted switches (set only with Parasolid Support advice)
/// Swept surface coincidence — emulates V9.0.
pub const PK_SESSION_SURF_coincide_c: PK_SESSION_sw_t = 1;
/// Isoline curve creation — emulates V10.0.
pub const PK_SESSION_old_isoclines_c: PK_SESSION_sw_t = 2;
/// Tag persistence — emulates V12.0.
pub const PK_SESSION_ENTITY_persist_alt_c: PK_SESSION_sw_t = 3;
/// Smooth intersection curve edge generation along blends — emulates V12.1.
pub const PK_SESSION_old_blend_bounds_c: PK_SESSION_sw_t = 4;
/// Merging planes — emulates V12.1.
pub const PK_SESSION_PLANE_exact_coi_c: PK_SESSION_sw_t = 5;
/// PK_FACE_find_uvbox — emulates V13.0.
pub const PK_SESSION_FACE_old_uvbox_c: PK_SESSION_sw_t = 6;
/// Local operations — emulates V13.0.
pub const PK_SESSION_local_ops_pre_v132_c: PK_SESSION_sw_t = 7;
/// Point contacts between surfaces — emulates V13.0.
pub const PK_SESSION_point_int_pre_v132_c: PK_SESSION_sw_t = 8;
/// Checker ignores ruled boundary degeneracies — emulates V13.2.
pub const PK_SESSION_check_pre_v140_c: PK_SESSION_sw_t = 9;
/// Boolean operations — emulates V14.0.
pub const PK_SESSION_booleans_pre_v141_c: PK_SESSION_sw_t = 10;
/// Offset surface checking — emulates V14.0.
pub const PK_SESSION_check_pre_v141_c: PK_SESSION_sw_t = 11;
/// Helix creation — emulates V14.0.
pub const PK_SESSION_old_helix_c: PK_SESSION_sw_t = 12;
/// Min radius of curvature on swept/spun surfaces — emulates V14.0.
pub const PK_SESSION_SURF_old_min_radii_c: PK_SESSION_sw_t = 13;
/// Check partitions during transmit (added V14.0).
pub const PK_SESSION_check_transmit_c: PK_SESSION_sw_t = 14;
/// Hidden line rendering — emulates V31.0.
pub const PK_SESSION_old_hir_wire_c: PK_SESSION_sw_t = 15;
/// Edge merging during local operations — emulates V14.1.
pub const PK_SESSION_pre_v150_switch_1_c: PK_SESSION_sw_t = 16;
/// Self-intersection checking in B-spline surfaces — emulates V14.1.
pub const PK_SESSION_pre_v150_switch_2_c: PK_SESSION_sw_t = 17;
/// Some local operations — emulates V14.1.
pub const PK_SESSION_pre_v150_switch_3_c: PK_SESSION_sw_t = 18;
/// Region tag persistence, attribute/group handling — emulates V14.1.
pub const PK_SESSION_region_gt_pre_v150_c: PK_SESSION_sw_t = 19;
/// Curve imprinting on faces — emulates V15.0.
pub const PK_SESSION_pre_v151_switch_1_c: PK_SESSION_sw_t = 20;
/// Miscellaneous — emulates V15.1.
pub const PK_SESSION_pre_v160_switch_1_c: PK_SESSION_sw_t = 21;
/// Edge blend capping — emulates V16.0.
pub const PK_SESSION_pre_v161_switch_1_c: PK_SESSION_sw_t = 22;
/// Edge blending — emulates V16.0.
pub const PK_SESSION_pre_v161_switch_2_c: PK_SESSION_sw_t = 23;
/// Faceting — emulates V16.0.
pub const PK_SESSION_pre_v161_switch_3_c: PK_SESSION_sw_t = 24;
/// Capping during taper operations — emulates V16.0.
pub const PK_SESSION_pre_v161_switch_4_c: PK_SESSION_sw_t = 25;
/// Conversion of blend surfaces to B-surfaces — emulates V16.0.
pub const PK_SESSION_pre_v161_switch_5_c: PK_SESSION_sw_t = 26;
/// Local operations — emulates V16.0.
pub const PK_SESSION_pre_v161_switch_6_c: PK_SESSION_sw_t = 27;

// =============================================================================
// PK_SESSION_software_option_t — structure holding all session-level switches
// =============================================================================

/// Structure holding all session-level software option switch values.
///
/// Each field is a `PK_LOGICAL_t` indicating whether the corresponding
/// backward-compatibility switch is enabled. Used with
/// `PK_SESSION_set_software_option` / `PK_SESSION_ask_software_option`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SESSION_software_option_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
}

impl Default for PK_SESSION_software_option_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

// =============================================================================
// Update switch structures (chapter 116) — per-function version control
// =============================================================================

/// Sweep fair switch for `PK_BODY_make_swept_body`.
/// Covers V12.1-V13.0, V13.2.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_sweep_fair_t {
    pub o_t_version: c_int,
}

impl Default for PK_BODY_sweep_fair_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// Update switch for boolean matching (`PK_BODY_boolean_2`, `PK_BODY_imprint_body`, etc.).
/// Covers V17.0 and earlier.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_match_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_boolean_match_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// Update switch for `PK_CURVE_fix_self_int`.
/// Covers V27.1 and earlier, V28.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_fix_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_CURVE_fix_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// Update switch for edge attach (`PK_EDGE_attach_curves_2`).
/// Covers V24.0 and earlier.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_attach_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_EDGE_attach_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// Update switch for face change (DEPRECATED — see `PK_local_ops_update_t`).
/// Covers V18.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_FACE_change_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// Update switch for B-curve creation (`PK_CURVE_make_bcurve_2`).
/// Covers V26.0 and earlier through V28.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_make_bcurve_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_make_bcurve_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// Update switch for B-surface creation (`PK_SURF_make_bsurf_2`).
/// Covers V23.1 and earlier, V24.0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_make_bsurf_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_make_bsurf_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}


/// Update switch for surface replacement (DEPRECATED — see `PK_local_ops_update_t`).
/// Covers V14.0 and earlier.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_replace_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_replace_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// Update switch for thickening (DEPRECATED — see `PK_local_ops_update_t`).
/// Covers V17.0 and earlier.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_thicken_update_t {
    pub o_t_version: c_int,
}

impl Default for PK_thicken_update_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

// =============================================================================
// Debug options structures
// =============================================================================

/// Options for `PK_DEBUG_SESSION_check`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_DEBUG_SESSION_check_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Number of partitions to check (0 = check whole session).
    pub n_partitions: c_int,
    /// Array of partition tags to check (ignored if `n_partitions == 0`).
    pub partitions: *const PK_PARTITION_t,
    /// Roll direction for checking.
    pub roll_direction: c_int,
    /// Maximum number of faults to report (0 = unlimited).
    pub max_faults: c_int,
}

impl Default for PK_DEBUG_SESSION_check_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_partitions: 0,
            partitions: std::ptr::null(),
            roll_direction: 0,
            max_faults: 0,
        }
    }
}

/// Options for `PK_DEBUG_try_error_handler`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_DEBUG_try_error_handler_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Whether to use protected mode.
    pub use_protected: PK_LOGICAL_t,
    /// Whether to use SMP.
    pub smp: PK_LOGICAL_t,
    /// Whether to call from one thread only.
    pub call_from_one: PK_LOGICAL_t,
    /// Whether to use locks.
    pub use_locks: PK_LOGICAL_t,
}

impl Default for PK_DEBUG_try_error_handler_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            use_protected: PK_LOGICAL_false,
            smp: PK_LOGICAL_false,
            call_from_one: PK_LOGICAL_false,
            use_locks: PK_LOGICAL_false,
        }
    }
}

/// Options for `PK_DEBUG_BODY_compare`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_DEBUG_BODY_compare_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Maximum number of local/geometric differences to report.
    /// 0 = topological (global) comparison only.
    /// Positive value enables local/geometric comparison.
    pub max_diffs: c_int,
}

impl Default for PK_DEBUG_BODY_compare_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            max_diffs: 0,
        }
    }
}

/// Return structure for `PK_DEBUG_BODY_compare`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_DEBUG_BODY_compare_r_t {
    /// Number of differences found.
    pub n_diffs: c_int,
}

impl Default for PK_DEBUG_BODY_compare_r_t {
    fn default() -> Self {
        Self { n_diffs: 0 }
    }
}

// =============================================================================
// Debug callback types for watch functions
// =============================================================================

/// Callback for class creation/destruction watch.
pub type PK_DEBUG_watch_class_f_t =
    Option<unsafe extern "C" fn(entity: PK_ENTITY_t, class: PK_CLASS_t, created: PK_LOGICAL_t)>;

/// Callback for function entry/exit watch.
pub type PK_DEBUG_watch_fn_f_t =
    Option<unsafe extern "C" fn(fn_name: *const c_char, entering: PK_LOGICAL_t)>;

/// Callback for tagged item creation/destruction watch.
pub type PK_DEBUG_watch_item_f_t =
    Option<unsafe extern "C" fn(entity: PK_ENTITY_t, created: PK_LOGICAL_t)>;

// =============================================================================
// extern "C" — PK_DEBUG_* functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // Session validity checking
    // =========================================================================

    /// Check validity of the whole session or specific partitions.
    pub fn PK_DEBUG_SESSION_check(
        options: *const PK_DEBUG_SESSION_check_o_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Version control debugging
    // =========================================================================

    /// Start debugging version control mechanisms. Highlights specified fault classes.
    pub fn PK_DEBUG_behaviours_start() -> PK_ERROR_code_t;

    /// Stop debugging version control mechanisms.
    pub fn PK_DEBUG_behaviours_stop() -> PK_ERROR_code_t;

    // =========================================================================
    // Array shuffle testing
    // =========================================================================

    /// Start shuffling return array arguments to test order-independence.
    pub fn PK_DEBUG_shuffle_start() -> PK_ERROR_code_t;

    /// Stop shuffling return array arguments.
    pub fn PK_DEBUG_shuffle_stop() -> PK_ERROR_code_t;

    // =========================================================================
    // Error handler testing
    // =========================================================================

    /// Test application signal handler by calling an error-generating function
    /// from within Parasolid. `PK_SESSION_abort` can be called from the
    /// error-generating function.
    pub fn PK_DEBUG_try_error_handler(
        options: *const PK_DEBUG_try_error_handler_o_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Watch callbacks — class/function/item monitoring
    // =========================================================================

    /// Register `PK_CLASS_t` tokens to watch via creation/destruction callbacks.
    pub fn PK_DEBUG_SESSION_watch_classes(
        n_classes: c_int,
        classes: *const PK_CLASS_t,
    ) -> PK_ERROR_code_t;

    /// Register PK functions to watch via entry/exit callbacks.
    pub fn PK_DEBUG_SESSION_watch_fns(
        n_fns: c_int,
        fn_names: *const *const c_char,
    ) -> PK_ERROR_code_t;

    /// Register tagged items to watch via creation/destruction callbacks.
    pub fn PK_DEBUG_SESSION_watch_items(
        n_items: c_int,
        items: *const PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body comparison
    // =========================================================================

    /// Compare two similar bodies. Default is global (topological count) only;
    /// set `max_diffs > 0` in options for local/geometric comparison.
    /// No local differences are found for wire body pairs.
    pub fn PK_DEBUG_BODY_compare(
        body_a: PK_BODY_t,
        body_b: PK_BODY_t,
        options: *const PK_DEBUG_BODY_compare_o_t,
        result: *mut PK_DEBUG_BODY_compare_r_t,
    ) -> PK_ERROR_code_t;

    /// Return structure registrar for `PK_DEBUG_BODY_compare`.
    pub fn PK_DEBUG_BODY_compare_r_f(
        result: *mut PK_DEBUG_BODY_compare_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Debug data extraction
    // =========================================================================

    /// Extract debug data from the session.
    pub fn PK_DEBUG_BODY_extract_data(
        body: PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Debug report functions
    // =========================================================================

    /// Write a comment to the debug report file.
    pub fn PK_DEBUG_report_comment(
        comment: *const c_char,
    ) -> PK_ERROR_code_t;

    /// Start recording debug info from PK functions.
    pub fn PK_DEBUG_report_start() -> PK_ERROR_code_t;

    /// Stop recording debug info from PK functions.
    pub fn PK_DEBUG_report_stop() -> PK_ERROR_code_t;

    // =========================================================================
    // Debug transmit/receive
    // =========================================================================

    /// Transmit debug data.
    pub fn PK_DEBUG_transmit() -> PK_ERROR_code_t;

    /// Receive debug data.
    pub fn PK_DEBUG_receive() -> PK_ERROR_code_t;

    /// Debug data callback registrar.
    pub fn PK_DEBUG_data_f() -> PK_ERROR_code_t;
}
