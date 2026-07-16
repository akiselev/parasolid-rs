//! Offsetting, hollowing, and thickening bindings.
//!
//! Bindings for `PK_BODY_offset_2`, `PK_FACE_offset_2`, `PK_EDGE_offset_on_body`,
//! `PK_BODY_hollow_2`, `PK_FACE_hollow_3`, and `PK_BODY_thicken_3` (Chapters 56-58).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_double, c_int, c_void};

use crate::*;

// =============================================================================
// Shared enum types for offset/hollow/thicken
// =============================================================================

// -- offset_method: self-intersection removal method --------------------------

pub type PK_offset_method_t = c_int;
/// Trim and extend (legacy).
pub const PK_offset_method_sx_trim_c: PK_offset_method_t = 22150;
/// Trim, repair, and extend.
pub const PK_offset_method_sx_repair_1_c: PK_offset_method_t = 22151;
/// Extended B-surface patching (recommended).
pub const PK_offset_method_sx_repair_2_c: PK_offset_method_t = 22152;

// -- offset_step: side face creation ------------------------------------------

pub type PK_offset_step_t = c_int;
/// No side faces (default).
pub const PK_offset_step_no_c: PK_offset_step_t = 22310;
/// Side faces at all smooth boundaries.
pub const PK_offset_step_all_c: PK_offset_step_t = 22312;
/// Side faces only where growing fails (not for facet geometry).
pub const PK_offset_step_site_c: PK_offset_step_t = 22313;
/// Side faces only at offset/non-offset boundaries (legacy v18.0).
pub const PK_offset_step_pierce_c: PK_offset_step_t = 22311;

// -- fix_degens: degeneracy repair --------------------------------------------

pub type PK_fix_degens_t = c_int;
/// No repair (default).
pub const PK_fix_degens_no_c: PK_fix_degens_t = 24850;
/// Repair only offset faces.
pub const PK_fix_degens_offset_c: PK_fix_degens_t = 24851;
/// Repair offset faces and their non-offset pairs.
pub const PK_fix_degens_all_c: PK_fix_degens_t = 24852;

// -- blend_edges: blend creation at offset edges ------------------------------

pub type PK_EDGE_offset_blend_t = c_int;
/// Replace convex offset edges with blends.
pub const PK_EDGE_offset_blend_convex_c: PK_EDGE_offset_blend_t = 22641;

// -- vertex_limit: vertex tolerance constraint --------------------------------

pub type PK_VERTEX_limit_t = c_int;
/// No limit (default).
pub const PK_VERTEX_limit_no_c: PK_VERTEX_limit_t = 24280;
/// Allow but report large tolerances.
pub const PK_VERTEX_limit_report_c: PK_VERTEX_limit_t = 24281;
/// Fail if limits not satisfied.
pub const PK_VERTEX_limit_yes_c: PK_VERTEX_limit_t = 24282;

// -- edge_limit: edge tolerance constraint ------------------------------------

pub type PK_EDGE_limit_t = c_int;
/// No limit (default).
pub const PK_EDGE_limit_no_c: PK_EDGE_limit_t = 24320;
/// Allow but report large tolerances.
pub const PK_EDGE_limit_report_c: PK_EDGE_limit_t = 24321;
/// Fail if limits not satisfied.
pub const PK_EDGE_limit_yes_c: PK_EDGE_limit_t = 24322;

// -- track_del: track partially tight-curved face deletions -------------------

pub type PK_offset_track_del_t = c_int;
/// Track partially tight-curved face deletions.
pub const PK_offset_track_del_sx_repair_c: PK_offset_track_del_t = 24141;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_offset_track_del_no_c: PK_offset_track_del_t = 24140;

// -- ortho_vx_split: laminar vertex splitting ---------------------------------

pub type PK_ortho_vx_split_t = c_int;

// -- local_ops_update: update switch ------------------------------------------

// -- grow: heal overflowing features ------------------------------------------

pub type PK_FACE_grow_t = c_int;
/// Do not grow any faces.
pub const PK_FACE_grow_no_c: PK_FACE_grow_t = 24124;
/// Grow the faces that are moving.
pub const PK_FACE_grow_moving_c: PK_FACE_grow_t = 24122;
/// Grow the faces that are not moving.
pub const PK_FACE_grow_fixed_c: PK_FACE_grow_t = 24123;

// =============================================================================
// Hollowing-specific enum types
// =============================================================================

pub type PK_hollow_local_t = c_int;
/// Hollow entire body (default).
pub const PK_hollow_local_none_c: PK_hollow_local_t = 23832;
/// Hollow only specified faces (add to existing hollow).
pub const PK_hollow_local_add_c: PK_hollow_local_t = 23830;
/// Hollow everything except specified faces.
pub const PK_hollow_local_exclude_c: PK_hollow_local_t = 23831;

// =============================================================================
// Thickening-specific enum types
// =============================================================================

pub type PK_thicken_method_t = c_int;
/// Create side faces in specified punch direction.
pub const PK_thicken_method_punch_c: PK_thicken_method_t = 22041;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_thicken_method_offset_c: PK_thicken_method_t = 22040;

// =============================================================================
// Edge offset gap fill enum types
// =============================================================================

pub type PK_VERTEX_gap_fill_t = c_int;
/// Fill with round arc.
pub const PK_VERTEX_gap_fill_round_c: PK_VERTEX_gap_fill_t = 21220;
/// Fill with tangent extension.
pub const PK_VERTEX_gap_fill_linear_c: PK_VERTEX_gap_fill_t = 21221;

// =============================================================================
// Report constants
// =============================================================================

/// Degeneracies fixed report status.
pub const PK_REPORT_1_fa_fix_degens_c: c_int = 23904;
/// Large vertex tolerance report status.
pub const PK_REPORT_1_vx_large_tol_c: c_int = 23897;
/// Large edge tolerance report status.
pub const PK_REPORT_1_ed_large_tol_c: c_int = 23896;
/// Track record: face deleted.
pub const PK_TOPOLOGY_track_delete_c: c_int = 1;

// =============================================================================
// Status constants
// =============================================================================

/// Non-manifold edge/vertex blocking offset on body.
pub const PK_offset_on_body_general_c: PK_local_status_t = 23185;
/// Edge tolerance limit failure.
pub const PK_local_status_ed_large_tol_c: PK_local_status_t = 21476;
// [re-abi] appended 38 missing member(s) from pk-enums.h
pub const PK_local_status_ok_c: PK_local_status_t = 21450;
pub const PK_local_status_nocheck_c: PK_local_status_t = 21451;
pub const PK_local_status_fail_c: PK_local_status_t = 21452;
pub const PK_local_status_cant_get_pt_c: PK_local_status_t = 21453;
pub const PK_local_status_cant_get_cu_c: PK_local_status_t = 21454;
pub const PK_local_status_cant_get_su_c: PK_local_status_t = 21455;
pub const PK_local_status_cant_offset_c: PK_local_status_t = 21456;
pub const PK_local_status_side_cu_fail_c: PK_local_status_t = 21457;
pub const PK_local_status_side_su_fail_c: PK_local_status_t = 21458;
pub const PK_local_status_not_same_by_c: PK_local_status_t = 21459;
pub const PK_local_status_fa_fail_c: PK_local_status_t = 21460;
pub const PK_local_status_fa_fa_fail_c: PK_local_status_t = 21461;
pub const PK_local_status_ed_remains_c: PK_local_status_t = 21462;
pub const PK_local_status_point_contact_c: PK_local_status_t = 21463;
pub const PK_local_status_bad_reference_c: PK_local_status_t = 21464;
pub const PK_local_status_not_supported_c: PK_local_status_t = 21465;
pub const PK_local_status_cant_extend_c: PK_local_status_t = 21466;
pub const PK_local_status_sheet_small_c: PK_local_status_t = 21467;
pub const PK_local_status_cant_use_cu_c: PK_local_status_t = 21468;
pub const PK_local_status_eds_unconnected_c: PK_local_status_t = 21469;
pub const PK_local_status_non_laminar_c: PK_local_status_t = 21470;
pub const PK_local_status_reference_loop_c: PK_local_status_t = 21471;
pub const PK_local_status_lp_outside_fa_c: PK_local_status_t = 21472;
pub const PK_local_status_target_missed_c: PK_local_status_t = 21473;
pub const PK_local_status_wrong_side_c: PK_local_status_t = 21474;
pub const PK_local_status_vx_large_tol_c: PK_local_status_t = 21475;
pub const PK_local_status_blend_too_tight_c: PK_local_status_t = 21477;
pub const PK_local_status_cant_make_blend_c: PK_local_status_t = 21478;
pub const PK_local_status_bad_chamfer_c: PK_local_status_t = 21479;
pub const PK_local_status_bad_blend_c: PK_local_status_t = 21480;
pub const PK_local_status_not_blend_surf_c: PK_local_status_t = 21481;
pub const PK_local_status_steep_edge_c: PK_local_status_t = 21482;
pub const PK_local_status_bad_direction_c: PK_local_status_t = 21483;
pub const PK_local_status_on_target_c: PK_local_status_t = 21484;
pub const PK_local_status_not_extended_c: PK_local_status_t = 21485;
pub const PK_local_status_negated_body_c: PK_local_status_t = 21486;
pub const PK_local_status_miter_fail_c: PK_local_status_t = 21487;
pub const PK_local_status_self_shadowing_c: PK_local_status_t = 21488;

// =============================================================================
// Options structures
// =============================================================================

/// Options for `PK_BODY_offset_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_offset_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Number of faces to offset by non-default distance.
    pub n_offset_faces: c_int,
    /// Array of faces to offset by non-default distance.
    pub offset_faces: *const PK_FACE_t,
    /// Offset values for `offset_faces`.
    pub offset_values: *const c_double,
    /// Allow disjoint bodies in result.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create step/side faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before offsetting.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Radius for blend faces.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Constrain vertex tolerance.
    pub vertex_limit: PK_VERTEX_limit_t,
    /// Constrain edge tolerance.
    pub edge_limit: PK_EDGE_limit_t,
    /// Track deletion of partially tight-curved faces.
    pub track_del: PK_offset_track_del_t,
    /// How to split laminar vertices.
    pub ortho_vx_split: PK_ortho_vx_split_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_BODY_offset_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_offset_faces: 0,
            offset_faces: std::ptr::null(),
            offset_values: std::ptr::null(),
            allow_disjoint: PK_LOGICAL_true,
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            vertex_limit: PK_VERTEX_limit_no_c,
            edge_limit: PK_EDGE_limit_no_c,
            track_del: 0,
            ortho_vx_split: 0,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_FACE_offset_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_offset_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Allow disjoint bodies in result.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create step/side faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before offsetting.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Radius for blend faces.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Constrain vertex tolerance.
    pub vertex_limit: PK_VERTEX_limit_t,
    /// Constrain edge tolerance.
    pub edge_limit: PK_EDGE_limit_t,
    /// Track deletion of partially tight-curved faces.
    pub track_del: PK_offset_track_del_t,
    /// How to split laminar vertices.
    pub ortho_vx_split: PK_ortho_vx_split_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_FACE_offset_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            allow_disjoint: PK_LOGICAL_true,
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            vertex_limit: PK_VERTEX_limit_no_c,
            edge_limit: PK_EDGE_limit_no_c,
            track_del: 0,
            ortho_vx_split: 0,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_BODY_hollow_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_hollow_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Number of faces to pierce (remove to open void).
    pub n_pierce_faces: c_int,
    /// Array of faces to pierce.
    pub pierce_faces: *const PK_FACE_t,
    /// Number of faces to offset by non-default distance.
    pub n_offset_faces: c_int,
    /// Array of faces to offset by non-default distance.
    pub offset_faces: *const PK_FACE_t,
    /// Offset distances for `offset_faces` (zero = pierce).
    pub offset_values: *const c_double,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create side faces between different-offset faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before hollowing.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Local hollowing mode.
    pub hollow_local: PK_hollow_local_t,
    /// Number of faces for local hollowing.
    pub n_local_faces: c_int,
    /// Faces to hollow or exclude (for local hollowing).
    pub local_faces: *const PK_FACE_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Blend radius.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Number of grow data entries.
    pub n_grow_data: c_int,
    /// Override grow behavior data.
    pub grow_data: *const c_void,
    /// Override grow behavior callback.
    pub grow_cb: *const c_void,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_BODY_hollow_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_pierce_faces: 0,
            pierce_faces: std::ptr::null(),
            n_offset_faces: 0,
            offset_faces: std::ptr::null(),
            offset_values: std::ptr::null(),
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            hollow_local: PK_hollow_local_none_c,
            n_local_faces: 0,
            local_faces: std::ptr::null(),
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            n_grow_data: 0,
            grow_data: std::ptr::null(),
            grow_cb: std::ptr::null(),
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_FACE_hollow_3`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_hollow_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create side faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before hollowing.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Local hollowing mode.
    pub hollow_local: PK_hollow_local_t,
    /// Number of faces for local hollowing.
    pub n_local_faces: c_int,
    /// Faces to hollow or exclude (for local hollowing).
    pub local_faces: *const PK_FACE_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Blend radius.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_FACE_hollow_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            hollow_local: PK_hollow_local_none_c,
            n_local_faces: 0,
            local_faces: std::ptr::null(),
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_BODY_thicken_3`.
///
/// Field order/offsets: `PKU_journal_BODY_thicken_o` (V37.01.243). The previous
/// binding had the `n_edges/edges/n_surfaces/surfaces` block misplaced (right
/// after `punch_dir`) and the tail (`offset_step/grow/update/n_pierce_faces`)
/// scrambled; corrected here to match the journal byte offsets exactly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_thicken_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int, // @0
    /// Number of faces to thicken by non-default amounts.
    pub n_faces: c_int, // @4
    /// Faces to thicken by non-default amounts.
    pub faces: *const PK_FACE_t, // @8
    /// Front offset per face (array of size `n_faces`).
    pub front_offsets: *const c_double, // @16
    /// Back offset per face (array of size `n_faces`).
    pub back_offsets: *const c_double, // @24
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t, // @32
    /// Side face creation method.
    pub method: PK_thicken_method_t, // @36
    /// Punch direction vector (when `method` = `PK_thicken_method_punch_c`).
    pub punch_dir: PK_VECTOR_t, // @40 (24 bytes)
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t, // @64
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t, // @68
    /// Fix degeneracies before thickening.
    pub fix_degens: PK_fix_degens_t, // @72
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t, // @76
    /// Number of laminar edges for user-supplied side surfaces.
    pub n_edges: c_int, // @80
    /// Laminar edges for user-supplied side surfaces.
    pub edges: *const PK_EDGE_t, // @88
    /// Number of user-supplied side surfaces.
    pub n_surfaces: c_int, // @96
    /// User-supplied side surfaces.
    pub surfaces: *const PK_SURF_t, // @104
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t, // @112
    /// Blend radius.
    pub blend_radius: c_double, // @120
    /// Split laminar vertices orthogonally.
    pub ortho_vx_split: PK_ortho_vx_split_t, // @128
    /// Update switch.
    pub update: PK_local_ops_update_t, // @132
    /// Create side faces along smooth edges.
    pub offset_step: PK_offset_step_t, // @136
    /// Number of faces to pierce (remove from result).
    pub n_pierce_faces: c_int, // @140
    /// Faces to pierce.
    pub pierce_faces: *const PK_FACE_t, // @144
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t, // @152
}

impl Default for PK_BODY_thicken_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_faces: 0,
            faces: std::ptr::null(),
            front_offsets: std::ptr::null(),
            back_offsets: std::ptr::null(),
            check_fa_fa: PK_LOGICAL_false,
            method: 0,
            punch_dir: [0.0, 0.0, 0.0],
            n_edges: 0,
            edges: std::ptr::null(),
            n_surfaces: 0,
            surfaces: std::ptr::null(),
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            blend_edges: 0,
            blend_radius: 0.0,
            ortho_vx_split: 0,
            n_pierce_faces: 0,
            pierce_faces: std::ptr::null(),
            offset_step: PK_offset_step_no_c,
            grow: PK_FACE_grow_no_c,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_EDGE_offset_on_body`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_offset_on_body_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Fill gaps: round arc or tangent extension.
    pub gap_fill: PK_VERTEX_gap_fill_t,
    /// Fill gaps (constrained by angle).
    pub constrained_gap_fill: PK_LOGICAL_t,
    /// Angular interval constraint (radians).
    pub gap_fill_angle: PK_INTERVAL_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_EDGE_offset_on_body_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            gap_fill: PK_VERTEX_gap_fill_round_c,
            constrained_gap_fill: PK_LOGICAL_false,
            gap_fill_angle: PK_INTERVAL_t {
                low: 0.0,
                high: 0.0,
            },
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Results from `PK_BODY_thicken_2`.
#[repr(C)]
pub struct PK_BODY_thicken_r_t { _private: [u8; 0] }

// =============================================================================
// Extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // -------------------------------------------------------------------------
    // Offsetting (Chapter 56)
    // -------------------------------------------------------------------------

    /// Offset all faces in a body by a specified distance.
    ///
    /// Positive offset = outward (direction of face normal), negative = inward.
    /// Can optionally offset a subset of faces by different distances via options.
    ///
    /// # Arguments
    ///
    /// * `body`      - Body to offset.
    /// * `distance`  - Default offset distance for all faces.
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure.
    pub fn PK_BODY_offset_2(
        body: PK_BODY_t,
        offset: c_double,
        tolerance: c_double,
        options: *mut PK_BODY_offset_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    /// Offset a specific set of faces in a body.
    ///
    /// Receives set of faces + set of offsets, offsets each face by corresponding
    /// amount. Same underlying functionality as `PK_BODY_offset_2`.
    ///
    /// # Arguments
    ///
    /// * `n_faces`   - Number of faces to offset.
    /// * `faces`     - Array of face tags.
    /// * `offsets`   - Array of offset distances (one per face).
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure.
    pub fn PK_FACE_offset_2(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        offsets: *mut c_double,
        tolerance: c_double,
        options: *mut PK_FACE_offset_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    /// Offset connected edges in a given direction within their owning body.
    ///
    /// Edges can be on manifold or general body (must be in locally manifold part).
    /// Offset edges are imprinted on body. Each point offset within its normal
    /// plane; distance measured by arc-length.
    ///
    /// # Arguments
    ///
    /// * `n_edges`    - Number of edges to offset.
    /// * `edges`      - Array of connected edge tags.
    /// * `distance`   - Offset distance (arc-length).
    /// * `direction`  - Offset direction vector.
    /// * `options`    - Options structure.
    /// * `n_new_edges` - (out) Number of new edges created.
    /// * `new_edges`   - (out) Array of new edge tags (PK-allocated, caller must free).
    pub fn PK_EDGE_offset_on_body(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        direction: PK_HAND_t,
        options: *mut PK_EDGE_offset_on_body_o_t,
        results: *mut PK_EDGE_offset_on_body_r_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Hollowing (Chapter 57)
    // -------------------------------------------------------------------------

    /// Hollow a solid body by offsetting all faces by a specified distance.
    ///
    /// Positive offset = outward (slightly larger hollowed body).
    /// Negative offset = inward (same outer size as original).
    ///
    /// # Arguments
    ///
    /// * `body`      - Solid body to hollow.
    /// * `distance`  - Default offset distance for all faces.
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure (includes pierce faces, per-face offsets).
    /// Hollow (shell) a solid. V35: `(PK_BODY_t body, double offset,
    /// double tolerance, const PK_BODY_hollow_o_t *options,
    /// PK_TOPOL_track_r_t *tracking, PK_TOPOL_local_r_t *results)`. The old
    /// binding was missing the `tracking`/`results` outputs (both written
    /// unconditionally, so NULL faults the kernel).
    pub fn PK_BODY_hollow_2(
        body: PK_BODY_t,
        offset: c_double,
        tolerance: c_double,
        options: *const PK_BODY_hollow_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    /// Hollow by specifying exactly which faces to offset and by how much.
    ///
    /// Any faces NOT specified are treated as pierce faces.
    ///
    /// # Arguments
    ///
    /// * `n_faces`   - Number of faces to offset.
    /// * `faces`     - Array of face tags.
    /// * `offsets`   - Array of offset distances (one per face).
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure.
    pub fn PK_FACE_hollow_3(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        offsets: *mut c_double,
        tolerance: c_double,
        options: *mut PK_FACE_hollow_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Thickening (Chapter 58)
    // -------------------------------------------------------------------------

    /// Thicken a sheet body to form a solid body.
    ///
    /// Sheet thickened by offsetting in both directions by specified amounts.
    /// Only manifold sheets can be thickened. Side faces created by surface
    /// normals, punch direction, or user-supplied surfaces.
    ///
    /// # Arguments
    ///
    /// * `body`          - Sheet body to thicken.
    /// * `front_default` - Default front offset distance.
    /// * `back_default`  - Default back offset distance.
    /// * `tolerance`     - Tolerance for approximate geometry.
    /// * `options`       - Options structure (includes per-face offsets, side face method).
    pub fn PK_BODY_thicken_3(
        body: PK_BODY_t,
        front_default: c_double,
        back_default: c_double,
        tolerance: c_double,
        options: *mut PK_BODY_thicken_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Obsolete V1 offset/hollow/thicken functions
    // -------------------------------------------------------------------------

    /// Offset faces of solid/sheet body (obsolete V1).
    pub fn PK_BODY_offset(
        body: PK_BODY_t,
        offset: c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Hollow solid body by offsetting all faces (obsolete V1).
    pub fn PK_BODY_hollow(
        body: PK_BODY_t,
        offset: c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        n_faces: *mut c_int,
        old_faces: *mut *mut PK_FACE_t,
        new_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Thicken sheet body into solid (obsolete V1).
    pub fn PK_BODY_thicken(
        body: PK_BODY_t,
        front: c_double,
        back: c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        n_topols: *mut c_int,
        old_topols: *mut *mut PK_TOPOL_t,
        new_topols: *mut *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Thicken sheet body into solid (current V2).
    pub fn PK_BODY_thicken_2(
        body: PK_BODY_t,
        front_default: c_double,
        back_default: c_double,
        tolerance: c_double,
        options: *const PK_BODY_thicken_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_BODY_thicken_r_t,
    ) -> PK_ERROR_code_t;

    /// Hollow body by offsetting specific faces (obsolete V1).
    pub fn PK_FACE_hollow(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        offsets: *mut c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        new_faces: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Hollow body by offsetting specific faces (current V2).
    pub fn PK_FACE_hollow_2(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        offsets: *mut c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        n_new_faces: *mut c_int,
        old_faces: *mut *mut PK_FACE_t,
        new_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;
}
