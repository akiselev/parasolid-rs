//! Option and result layouts for the CADabra oracle calls, materialised from
//! the 2026-10-16 parasolid-re audit. Each replaces a zero-sized placeholder
//! from `generated_stubs.rs`. A zero-sized result struct is unsound: the kernel
//! writes 40–96 bytes into it.
//!
//! Evidence (same DLL, SHA-256 `c900fa3f…63073`, 37.01.243):
//! `parasolid-re/catalog/pk-validated-abi.md` ("P9 oracle construction calls",
//! "Construction batch", re-validation table) and
//! `artifacts/port/oracle/README.md` "ABI basis". Field order is from the header
//! pages (documented). Offsets, current version and defaults are from each
//! entry point's option-migration routine (static-observed). The calls were
//! made live by the parasolid-re oracle harness and here in
//! `parasolid-test/src/oracle_wrappers.rs` (dynamic-observed).
//!
//! `Default` on each options struct stamps the **current** version with the
//! kernel's own NULL-options defaults.

#![allow(non_camel_case_types, non_upper_case_globals)]

use crate::*;
use std::os::raw::{c_double, c_int};

// =============================================================================
// Tokens
// =============================================================================

pub type PK_GEOM_copy_t = c_int;
pub const PK_GEOM_copy_auto_c: PK_GEOM_copy_t = 24772;

pub type PK_check_vx_on_cu_t = c_int;
pub const PK_check_vx_on_cu_all_c: PK_check_vx_on_cu_t = 24760;
pub const PK_check_vx_on_cu_none_c: PK_check_vx_on_cu_t = 24761;
pub const PK_check_vx_on_cu_unbounded_c: PK_check_vx_on_cu_t = 24762;

pub type PK_EDGE_attach_update_t = c_int;
pub const PK_EDGE_attach_update_default_c: PK_EDGE_attach_update_t = 25050;

pub type PK_make_spcurves_degen_t = c_int;
pub const PK_make_spcurves_degen_no_c: PK_make_spcurves_degen_t = 23670;
pub type PK_make_spcurves_method_t = c_int;
pub const PK_make_spcurves_method_dflt_c: PK_make_spcurves_method_t = 23640;

pub type PK_set_precision_method_t = c_int;

pub const PK_reset_prec_ok_c: PK_reset_prec_t = 17201;
pub const PK_reset_prec_tangent_c: PK_reset_prec_t = 17202;
pub const PK_reset_prec_missing_geom_c: PK_reset_prec_t = 17203;
pub const PK_reset_prec_failure_c: PK_reset_prec_t = 17204;

pub type PK_compound_geom_attr_t = c_int;
pub const PK_compound_geom_attr_keep_c: PK_compound_geom_attr_t = 25220;
pub const PK_compound_geom_attr_merge_c: PK_compound_geom_attr_t = 25221;

pub type PK_section_result_t = c_int;
pub const PK_section_result_success_c: PK_section_result_t = 21750;
pub const PK_section_result_no_clash_c: PK_section_result_t = 21751;
pub const PK_section_result_no_effect_c: PK_section_result_t = 21752;
pub const PK_section_result_failed_c: PK_section_result_t = 21753;


// =============================================================================
// PK_BODY_create_topology_2 (@0x1800fb940)
// =============================================================================

/// `PK_BODY_create_topology_2_o_t`, version 1 (2 → 5022): `{version,
/// body_type}`. Inline migration in the entry; NULL → `general` (5605).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_create_topology_2_o_t {
    pub o_t_version: c_int,
    pub body_type: PK_BODY_type_t,
}

impl Default for PK_BODY_create_topology_2_o_t {
    fn default() -> Self {
        Self { o_t_version: 1, body_type: PK_BODY_type_general_c }
    }
}

/// `PK_create_fault_t`, 16 bytes: `{state, n_indices, indices*}`. `indices`
/// are indices into the caller's `classes` array.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_create_fault_t {
    pub state: PK_check_state_t,
    pub n_indices: c_int,
    pub indices: *mut c_int,
}

/// `PK_BODY_create_topology_2_r_t`, 40 bytes. Free with
/// `PK_BODY_create_topology_2_r_f`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_create_topology_2_r_t {
    pub body: PK_BODY_t,
    pub n_topols: c_int,
    /// One tag per entry of `classes` (index 0 is the body).
    pub topols: *mut PK_TOPOL_t,
    pub n_create_faults: c_int,
    pub create_faults: *mut PK_create_fault_t,
    pub internal_indices: *mut c_int,
}

// =============================================================================
// PK_EDGE_attach_curves_2 (@0x180213c30), tracking result
// =============================================================================

/// `PK_EDGE_attach_curves_o_t`, version 2 (3 → 5022), 56 bytes. Defaults from
/// the NULL table at 0x183f87b50.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_attach_curves_o_t {
    pub o_t_version: c_int,                  // @0  2
    pub n_intervals: c_int,                  // @4  0
    pub intervals: *const PK_INTERVAL_t,     // @8
    pub interval_map: *const c_int,          // @16
    pub have_senses: PK_LOGICAL_t,           // @24 false
    pub senses: *const PK_LOGICAL_t,         // @32
    pub copy_curves: PK_GEOM_copy_t,         // @40 auto
    pub vx_checking: PK_check_vx_on_cu_t,    // @44 all
    pub geom_checking: PK_check_geom_t,      // @48 yes
    pub update: PK_EDGE_attach_update_t,     // @52 default
}

impl Default for PK_EDGE_attach_curves_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 2,
            n_intervals: 0,
            intervals: std::ptr::null(),
            interval_map: std::ptr::null(),
            have_senses: PK_LOGICAL_false,
            senses: std::ptr::null(),
            copy_curves: PK_GEOM_copy_auto_c,
            vx_checking: PK_check_vx_on_cu_all_c,
            geom_checking: PK_check_geom_yes_c,
            update: PK_EDGE_attach_update_default_c,
        }
    }
}

/// `PK_ENTITY_track_r_t`, 40 bytes. The field meanings are not decoded here, so
/// it is kept as a sized, zero-initialisable buffer: the kernel writes it and
/// `PK_ENTITY_track_r_f` frees it. (The previous zero-sized declaration made
/// every caller hand the kernel a 0-byte buffer.)
#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct PK_ENTITY_track_r_t {
    _opaque: [u8; 40],
}

impl Default for PK_ENTITY_track_r_t {
    fn default() -> Self {
        Self { _opaque: [0; 40] }
    }
}

// =============================================================================
// Precision, SP-curves, nominal curves
// =============================================================================

/// `PK_EDGE_set_precision_o_t`, version 2 (3 → 5022): `{version, sp_method,
/// report}` (inline migration @0x180225540).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_set_precision_o_t {
    pub o_t_version: c_int,
    pub sp_method: PK_set_precision_method_t,
    pub report: PK_set_precision_report_t,
}

impl Default for PK_EDGE_set_precision_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 2,
            sp_method: PK_set_precision_default_c,
            report: PK_set_precision_report_no_c,
        }
    }
}

/// `PK_CURVE_make_spcurves_o_t`, version 1, 16 bytes: `{version, degenerate,
/// sense (1 B), sp_method}` (NULL branch in the entry @0x1801eb490).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_make_spcurves_o_t {
    pub o_t_version: c_int,
    pub degenerate: PK_make_spcurves_degen_t,
    pub sense: PK_LOGICAL_t,
    pub sp_method: PK_make_spcurves_method_t,
}

impl Default for PK_CURVE_make_spcurves_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            degenerate: PK_make_spcurves_degen_no_c,
            sense: PK_LOGICAL_true,
            sp_method: PK_make_spcurves_method_dflt_c,
        }
    }
}

/// `PK_EDGE_attach_curve_nmnl_o_t`, 24 bytes: `{version, have_interval (1 B),
/// t_int}`. The entry (@0x180212a70) dereferences the struct with **no NULL
/// branch** and does not check the version, so NULL crashes. Version 1 used.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_attach_curve_nmnl_o_t {
    pub o_t_version: c_int,
    pub have_interval: PK_LOGICAL_t,
    pub t_int: PK_INTERVAL_t,
}

impl Default for PK_EDGE_attach_curve_nmnl_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            have_interval: PK_LOGICAL_false,
            t_int: PK_INTERVAL_t { low: 0.0, high: 0.0 },
        }
    }
}

// =============================================================================
// Compound bodies (@0x1800e23c0, ask_children @0x1800f0a40, ask_parent @0x1800f6e90)
// =============================================================================

/// `PK_BODY_make_compound_o_t`, version 3 (4 → 5022), 12 bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_compound_o_t {
    pub o_t_version: c_int,
    pub geom_attr: PK_compound_geom_attr_t,
    pub report_shared_geometry: PK_LOGICAL_t,
}

impl Default for PK_BODY_make_compound_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 3,
            geom_attr: PK_compound_geom_attr_keep_c,
            report_shared_geometry: PK_LOGICAL_false,
        }
    }
}

/// `PK_BODY_ask_children_o_t`, version 1: `{version}`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_ask_children_o_t {
    pub o_t_version: c_int,
}

impl Default for PK_BODY_ask_children_o_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

/// `PK_BODY_ask_parent_o_t`, version 1: `{version}`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_ask_parent_o_t {
    pub o_t_version: c_int,
}

impl Default for PK_BODY_ask_parent_o_t {
    fn default() -> Self {
        Self { o_t_version: 1 }
    }
}

// =============================================================================
// Face sets (@0x1801417f0) and section with sheet (@0x18016d550)
// =============================================================================

/// `PK_BODY_identify_facesets_o_t`, version 3 (4 → 5022), 20 bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_identify_facesets_o_t {
    pub o_t_version: c_int,
    /// Boolean function token (`PK_boolean_unite_c` …).
    pub function: PK_boolean_function_t,
    pub fence: PK_boolean_fence_t,
    pub check_manifold: PK_LOGICAL_t,
    pub update: PK_boolean_update_t,
}

impl Default for PK_BODY_identify_facesets_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 3,
            function: PK_boolean_unite_c,
            fence: PK_boolean_fence_none_c,
            check_manifold: PK_LOGICAL_true,
            update: PK_boolean_update_default_c,
        }
    }
}

/// `PK_identify_facesets_r_t`, 96 bytes. Each outer array holds
/// `PK_FACE_array_t` face sets. Free with `PK_identify_facesets_r_f`
/// (@0x180143fb0).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_identify_facesets_r_t {
    pub n_survive_target: c_int,
    pub survive_target: *mut PK_FACE_array_t,
    pub n_survive_tool: c_int,
    pub survive_tool: *mut PK_FACE_array_t,
    pub n_reject_target: c_int,
    pub reject_target: *mut PK_FACE_array_t,
    pub n_reject_tool: c_int,
    pub reject_tool: *mut PK_FACE_array_t,
    pub bound_target_edges: PK_EDGE_array_t,
    pub bound_tool_edges: PK_EDGE_array_t,
}

/// `PK_section_2_r_t`, 80 bytes. Reports have the `PK_boolean_report_r_t`
/// shape. Free with `PK_section_2_r_f`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_section_2_r_t {
    pub result: PK_section_result_t,
    pub n_front_bodies: c_int,
    pub front_bodies: *mut PK_BODY_t,
    pub n_back_bodies: c_int,
    pub back_bodies: *mut PK_BODY_t,
    pub n_front_faces: c_int,
    pub front_faces: *mut PK_FACE_t,
    pub n_back_faces: c_int,
    pub back_faces: *mut PK_FACE_t,
    pub n_reports: c_int,
    pub reports: *mut PK_boolean_report_r_t,
}

const _: () = {
    use core::mem::{offset_of, size_of};
    assert!(size_of::<PK_BODY_create_topology_2_o_t>() == 8);
    assert!(size_of::<PK_create_fault_t>() == 16);
    assert!(size_of::<PK_BODY_create_topology_2_r_t>() == 40);
    assert!(size_of::<PK_EDGE_attach_curves_o_t>() == 56);
    assert!(offset_of!(PK_EDGE_attach_curves_o_t, copy_curves) == 40);
    assert!(offset_of!(PK_EDGE_attach_curves_o_t, update) == 52);
    assert!(size_of::<PK_ENTITY_track_r_t>() == 40);
    assert!(size_of::<PK_EDGE_set_precision_o_t>() == 12);
    assert!(size_of::<PK_CURVE_make_spcurves_o_t>() == 16);
    assert!(offset_of!(PK_CURVE_make_spcurves_o_t, sp_method) == 12);
    assert!(size_of::<PK_EDGE_attach_curve_nmnl_o_t>() == 24);
    assert!(offset_of!(PK_EDGE_attach_curve_nmnl_o_t, t_int) == 8);
    assert!(size_of::<PK_BODY_make_compound_o_t>() == 12);
    assert!(size_of::<PK_BODY_identify_facesets_o_t>() == 20);
    assert!(offset_of!(PK_BODY_identify_facesets_o_t, update) == 16);
    assert!(size_of::<PK_identify_facesets_r_t>() == 96);
    assert!(offset_of!(PK_identify_facesets_r_t, bound_target_edges) == 64);
    assert!(size_of::<PK_section_2_r_t>() == 80);
    assert!(offset_of!(PK_section_2_r_t, reports) == 72);
};

#[link(name = "pskernel")]
unsafe extern "C" {
    /// Free a `PK_BODY_create_topology_2_r_t`.
    pub fn PK_BODY_create_topology_2_r_f(results: *mut PK_BODY_create_topology_2_r_t)
    -> PK_ERROR_code_t;
    /// Free a `PK_identify_facesets_r_t` (@0x180143fb0).
    pub fn PK_identify_facesets_r_f(results: *mut PK_identify_facesets_r_t) -> PK_ERROR_code_t;
    /// Free a `PK_section_2_r_t`.
    pub fn PK_section_2_r_f(results: *mut PK_section_2_r_t) -> PK_ERROR_code_t;
}
