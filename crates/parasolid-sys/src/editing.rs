#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! FFI bindings for Parasolid model editing functions.
//!
//! Covers: face change (generic editing), face/edge deletion, surface replacement,
//! face/body tapering, patching, hole filling, face moving/transform, simplification,
//! blend identification, body creation from entities, spin/sweep, and related utilities.
//!
//! Corresponds to Parasolid documentation chapters 60-70.

use std::os::raw::{c_double, c_int, c_void};

use crate::*;

// =============================================================================
// Topology tracking types (shared across many editing operations)
// =============================================================================

// =============================================================================
// Local operation status / report types
// =============================================================================

/// Local operations update switch for version compatibility.
pub type PK_local_ops_update_t = c_int;

/// Default update value — uses all appropriate local operations enhancements.
pub const PK_local_ops_update_default_c: PK_local_ops_update_t = 0;

/// Self-shadowing status (PK_BODY_taper).
pub const PK_local_status_self_shadowng_c: PK_local_status_t = 1;

/// Report status: face-face repair performed.
pub const PK_REPORT_1_fa_fa_repair_c: c_int = 1;
/// Report status: deform surface partially created.
pub const PK_REPORT_1_deform_surf_c: c_int = 2;
/// Report status: vertex geometry cannot be located.
pub const PK_REPORT_1_cant_get_pt_c: c_int = 3;

/// Report type 3: distance error in cover.
pub const PK_REPORT_3_distance_err_c: c_int = 1;
/// Report type 3: worse curvature in cover.
pub const PK_REPORT_3_worse_curvature_c: c_int = 2;
/// Report type 3: cover B-surface constructed before failure.
pub const PK_REPORT_3_cover_surf_c: c_int = 3;
/// Report type 3: edges not visibly G1 smooth.
pub const PK_REPORT_3_sharp_eds_c: c_int = 4;
/// Report type 3: radius of curvature too tight.
pub const PK_REPORT_3_tight_curvature_c: c_int = 5;
/// Report type 3: surface extended during operation.
pub const PK_REPORT_3_surf_extended_c: c_int = 6;
/// Report type 3: fill hole boundary not G1.
pub const PK_REPORT_3_fill_hole_non_g1_c: c_int = 7;
/// Report type 3: fill hole boundary not G2.
pub const PK_REPORT_3_fill_hole_non_g2_c: c_int = 8;

// =============================================================================
// Healing constants (PK_FACE_heal_t)
// =============================================================================

/// Healing action type for wound repair after face deletion.
pub type PK_FACE_heal_t = c_int;

/// Cap wound with a surface containing all wound edges.
pub const PK_FACE_heal_cap_c: PK_FACE_heal_t = 0;
/// Allow adjacent faces to shrink/grow to heal.
pub const PK_FACE_heal_shrink_c: PK_FACE_heal_t = 1;
/// Grow adjacent faces from parent to fill gap left by deleted face.
pub const PK_FACE_heal_grow_from_parent_c: PK_FACE_heal_t = 2;
/// Extend faces around hole.
pub const PK_FACE_heal_grow_from_child_c: PK_FACE_heal_t = 3;
/// Leave wound as rubber face (no geometry).
pub const PK_FACE_heal_no_c: PK_FACE_heal_t = 4;
/// Attempt healing using any available method.
pub const PK_FACE_heal_yes_c: PK_FACE_heal_t = 5;

/// Healing loops method.
pub type PK_FACE_heal_loops_t = c_int;

/// Let Parasolid decide healing method (recommended).
pub const PK_FACE_heal_loops_auto_c: PK_FACE_heal_loops_t = 0;
/// Heal loops separately.
pub const PK_FACE_heal_loops_separate_c: PK_FACE_heal_loops_t = 1;

// =============================================================================
// Face-face repair/checking constants
// =============================================================================

pub type PK_repair_fa_fa_t = c_int;
pub const PK_repair_fa_fa_no_c: PK_repair_fa_fa_t = 0;
pub const PK_repair_fa_fa_yes_c: PK_repair_fa_fa_t = 1;

pub type PK_repair_fa_t = c_int;
pub const PK_repair_fa_yes_c: PK_repair_fa_t = 0;
pub const PK_repair_fa_local_c: PK_repair_fa_t = 1;

pub type PK_check_fa_fa_t = c_int;
pub const PK_check_fa_fa_yes_c: PK_check_fa_fa_t = 0;
pub const PK_check_fa_fa_no_c: PK_check_fa_fa_t = 1;

// =============================================================================
// Delete tracking constants
// =============================================================================

pub type PK_delete_track_t = c_int;
/// No tracking (default).
pub const PK_delete_track_no_c: PK_delete_track_t = 0;
/// Track capping faces.
pub const PK_delete_track_cap_c: PK_delete_track_t = 1;
/// Track rubber faces.
pub const PK_delete_track_rubber_c: PK_delete_track_t = 2;

// =============================================================================
// PK_FACE_delete_2 options
// =============================================================================

/// Options for `PK_FACE_delete_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_delete_2_o_t {
    pub o_t_version: c_int,
    pub heal_action: PK_FACE_heal_t,
    pub heal_loops: PK_FACE_heal_loops_t,
    pub local_check: PK_LOGICAL_t,
    pub allow_disjoint: PK_LOGICAL_t,
    pub repair_fa_fa: PK_repair_fa_fa_t,
    pub track: PK_delete_track_t,
}

// =============================================================================
// PK_EDGE_delete options
// =============================================================================

/// Options for `PK_EDGE_delete`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_delete_o_t {
    pub o_t_version: c_int,
    pub update: PK_local_ops_update_t,
}

// =============================================================================
// PK_FACE_delete_facesets options and detail type
// =============================================================================

/// Detail type identifier.
pub type PK_detail_t = c_int;
pub const PK_detail_any_c: PK_detail_t = 0;
pub const PK_detail_rubber_c: PK_detail_t = 1;
pub const PK_detail_hole_cyl_c: PK_detail_t = 2;
pub const PK_detail_hole_cyl_through_c: PK_detail_t = 3;
pub const PK_detail_hole_cyl_blind_c: PK_detail_t = 4;
pub const PK_detail_hole_cyl_closed_c: PK_detail_t = 5;
pub const PK_detail_blend_rb_const_r_c: PK_detail_t = 6;

/// Options for `PK_FACE_delete_facesets`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_delete_facesets_o_t {
    pub o_t_version: c_int,
    pub allow_disjoint: PK_LOGICAL_t,
    pub heal_action: PK_FACE_heal_t,
    pub n_details: c_int,
    pub details: *const PK_detail_t,
    pub tolerance: c_double,
    pub update: PK_local_ops_update_t,
}

// =============================================================================
// PK_FACE_delete_blends options and related types
// =============================================================================

pub type PK_FACE_simplify_t = c_int;
/// Simplify vertex-adjacent blend surfaces.
pub const PK_FACE_simplify_adj_blends_c: PK_FACE_simplify_t = 0;
/// Do not simplify.
pub const PK_FACE_simplify_no_c: PK_FACE_simplify_t = 1;

pub type PK_blend_delete_cap_t = c_int;
/// Create planar cap faces to heal wounds.
pub const PK_blend_delete_cap_planar_c: PK_blend_delete_cap_t = 1;

pub type PK_blend_cap_type_t = c_int;
/// Delete to within 10% of blend end.
pub const PK_blend_cap_type_within_c: PK_blend_cap_type_t = 0;
/// Delete beyond the blend.
pub const PK_blend_cap_type_beyond_c: PK_blend_cap_type_t = 1;
/// Delete at the edge (cap may not be planar).
pub const PK_blend_cap_type_at_edge_c: PK_blend_cap_type_t = 2;

pub type PK_blend_delete_keep_t = c_int;
pub const PK_blend_delete_keep_yes_c: PK_blend_delete_keep_t = 0;
pub const PK_blend_delete_keep_no_c: PK_blend_delete_keep_t = 1;

/// Sub-structure for specifying underlying entities when deleting blends.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_delete_unders_data_t {
    pub n_blends: c_int,
    pub blends: *const PK_FACE_t,
    pub unders: *const PK_FACE_t,
}

/// Blend cap data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_delete_cap_data_t {
    pub cap_type: PK_blend_cap_type_t,
    pub n_caps: c_int,
    pub caps: *const PK_FACE_t,
    pub keep: *const PK_blend_delete_keep_t,
}

/// Options for `PK_FACE_delete_blends`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_delete_blends_o_t {
    pub o_t_version: c_int,
    pub check_fa_fa: PK_check_fa_fa_t,
    pub simplify: PK_FACE_simplify_t,
    pub cap: PK_blend_delete_cap_t,
    pub cap_data: PK_blend_delete_cap_data_t,
    pub update: PK_local_ops_update_t,
    pub unders_data: PK_blend_delete_unders_data_t,
}

// =============================================================================
// PK_TOPOL_delete_redundant_2 options
// =============================================================================

// =============================================================================
// Simplify geometry (Ch. 62)
// =============================================================================

/// Options for `PK_BODY_simplify_geom`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_simplify_geom_o_t {
    pub o_t_version: c_int,
}

/// Options for `PK_FACE_simplify_geom`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_simplify_geom_o_t {
    pub o_t_version: c_int,
}

// =============================================================================
// PK_BODY_find_facesets options and results (Ch. 62)
// =============================================================================

pub type PK_boolean_selector_t = c_int;
pub const PK_boolean_off_c: PK_boolean_selector_t = 0;

/// Options for `PK_BODY_find_facesets`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_find_facesets_o_t {
    pub o_t_version: c_int,
    pub selector: PK_boolean_selector_t,
    pub alternate: PK_LOGICAL_t,
    pub n_selecting_topol: c_int,
    pub selecting_topol: *const PK_TOPOL_t,
    pub want_bounds: PK_LOGICAL_t,
}

/// Result structure for `PK_BODY_find_facesets`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_find_facesets_r_t {
    pub n_facesets: c_int,
    pub facesets: *mut *mut PK_FACE_t,
    pub faceset_sizes: *mut c_int,
    pub n_bounds: c_int,
    pub bounds: *mut *mut PK_EDGE_t,
    pub bound_sizes: *mut c_int,
}

// =============================================================================
// PK_BODY_identify_details options and results (Ch. 62)
// =============================================================================

/// Double constraint for limiting radius ranges.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_double_constraint_t {
    pub interval: PK_INTERVAL_t,
    pub comparison: PK_comparison_t,
}

pub type PK_comparison_t = c_int;
pub const PK_comparison_always_c: PK_comparison_t = 0;
pub const PK_comparison_less_c: PK_comparison_t = 1;
pub const PK_comparison_equal_c: PK_comparison_t = 2;
pub const PK_comparison_greater_c: PK_comparison_t = 3;
pub const PK_comparison_between_c: PK_comparison_t = 4;
pub const PK_comparison_outside_c: PK_comparison_t = 5;
pub const PK_comparison_never_c: PK_comparison_t = 6;

pub type PK_hole_blended_t = c_int;
pub const PK_hole_blended_no_c: PK_hole_blended_t = 0;
pub const PK_hole_blended_in_c: PK_hole_blended_t = 1;
pub const PK_hole_blended_out_c: PK_hole_blended_t = 2;
pub const PK_hole_blended_trimmed_in_c: PK_hole_blended_t = 3;
pub const PK_hole_blended_trimmed_out_c: PK_hole_blended_t = 4;

pub type PK_hole_ortho_t = c_int;
pub const PK_hole_ortho_no_c: PK_hole_ortho_t = 0;
pub const PK_hole_ortho_yes_c: PK_hole_ortho_t = 1;

pub type PK_hole_const_rad_t = c_int;
pub const PK_hole_const_rad_no_c: PK_hole_const_rad_t = 0;
pub const PK_hole_const_rad_yes_c: PK_hole_const_rad_t = 1;

pub type PK_detail_perforated_t = c_int;
pub const PK_detail_perforated_no_c: PK_detail_perforated_t = 0;
pub const PK_detail_perforated_yes_c: PK_detail_perforated_t = 1;

pub type PK_detail_open_t = c_int;
pub const PK_detail_open_no_c: PK_detail_open_t = 0;
pub const PK_detail_open_yes_c: PK_detail_open_t = 1;

pub type PK_hole_update_t = c_int;
pub const PK_hole_update_default_c: PK_hole_update_t = 0;

pub type PK_proj_update_t = c_int;

pub type PK_outline_update_t = c_int;

pub type PK_surf_extend_update_t = c_int;

/// Options for `PK_BODY_identify_details`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_identify_details_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub angle_tolerance: c_double,
    pub hole_cyl_radius: PK_double_constraint_t,
    pub hole_blended: PK_hole_blended_t,
    pub hole_blend_radius: PK_double_constraint_t,
    pub hole_ortho: PK_hole_ortho_t,
    pub hole_const_rad: PK_hole_const_rad_t,
    pub hole_perforated: PK_detail_perforated_t,
    pub hole_open: PK_detail_open_t,
    pub update: PK_hole_update_t,
}

/// Result structure for `PK_BODY_identify_details`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_identify_details_r_t {
    pub n_facesets: c_int,
    pub facesets: *mut *mut PK_FACE_t,
    pub faceset_sizes: *mut c_int,
    pub details: *mut PK_detail_t,
}

// =============================================================================
// PK_FACE_identify_blends options and results (Ch. 62)
// =============================================================================

pub type PK_blend_identify_t = c_int;
/// Only chains contained in supplied faces.
pub const PK_blend_identify_within_c: PK_blend_identify_t = 0;
/// Minimal chain, excluding external branch faces.
pub const PK_blend_identify_exc_chain_c: PK_blend_identify_t = 1;
/// Minimal chain, including external branch faces.
pub const PK_blend_identify_inc_chain_c: PK_blend_identify_t = 2;
/// Maximal chain (as long as possible).
pub const PK_blend_identify_max_chain_c: PK_blend_identify_t = 3;
/// Chains dependent on supplied faces (recursive).
pub const PK_blend_identify_dependent_c: PK_blend_identify_t = 4;

pub type PK_blend_convexity_t = c_int;
pub const PK_blend_convexity_any_c: PK_blend_convexity_t = 0;
pub const PK_blend_convexity_concave_c: PK_blend_convexity_t = 1;
pub const PK_blend_convexity_convex_c: PK_blend_convexity_t = 2;

pub type PK_blend_follow_branch_t = c_int;
pub const PK_blend_follow_branch_yes_c: PK_blend_follow_branch_t = 0;
pub const PK_blend_follow_branch_no_c: PK_blend_follow_branch_t = 1;

pub type PK_blend_distant_unders_t = c_int;
pub const PK_blend_distant_unders_no_c: PK_blend_distant_unders_t = 0;
pub const PK_blend_distant_unders_yes_c: PK_blend_distant_unders_t = 1;

pub type PK_blend_report_blends_t = c_int;
pub const PK_blend_report_blends_no_c: PK_blend_report_blends_t = 0;
pub const PK_blend_report_blends_yes_c: PK_blend_report_blends_t = 1;

pub type PK_chain_optimise_t = c_int;
pub const PK_chain_optimise_none_c: PK_chain_optimise_t = 0;
pub const PK_chain_optimise_for_reblend_c: PK_chain_optimise_t = 1;

/// Options for `PK_FACE_identify_blends`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_identify_blends_o_t {
    pub o_t_version: c_int,
    pub limit_radii: PK_double_constraint_t,
    pub convexity: PK_blend_convexity_t,
    pub allow_pi: PK_LOGICAL_t,
    pub tolerance: c_double,
    pub want_radii: PK_LOGICAL_t,
    pub want_convexities: PK_LOGICAL_t,
    pub follow_branch: PK_blend_follow_branch_t,
    pub have_propagation_angle: PK_LOGICAL_t,
    pub propagation_angle: c_double,
    pub optimise_chains: PK_chain_optimise_t,
    pub update: PK_local_ops_update_t,
    pub report_blends: PK_blend_report_blends_t,
    pub distant_unders: PK_blend_distant_unders_t,
}

/// Result structure for `PK_FACE_identify_blends`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_identify_blends_r_t {
    pub n_blend_facesets: c_int,
    pub blend_facesets: *mut *mut PK_FACE_t,
    pub blend_faceset_sizes: *mut c_int,
    pub radii: *mut c_double,
    pub convexities: *mut PK_blend_convexity_t,
}

/// Options for `PK_FACE_find_blend_unders`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_find_blend_unders_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub update: PK_local_ops_update_t,
}

// =============================================================================
// PK_FACE_classify_details options and results (Ch. 62)
// =============================================================================

/// Options for `PK_FACE_classify_details`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_classify_details_o_t {
    pub o_t_version: c_int,
    pub up: PK_VECTOR_t,
    pub include_zero_depth: PK_LOGICAL_t,
    pub want_profiles: PK_LOGICAL_t,
    pub simplify_geom: PK_LOGICAL_t,
    pub face_tracking: PK_LOGICAL_t,
}

/// Hole component descriptor.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_hole_comp_t {
    pub comp_type: c_int,
    pub depth: c_double,
    pub entity: PK_ENTITY_t,
}

// =============================================================================
// Surface replacement (Ch. 63)
// =============================================================================

pub type PK_replace_merge_t = c_int;
pub const PK_replace_merge_no_c: PK_replace_merge_t = 0;
pub const PK_replace_merge_in_c: PK_replace_merge_t = 1;
pub const PK_replace_merge_out_c: PK_replace_merge_t = 2;

pub type PK_replace_use_t = c_int;
pub const PK_replace_use_attempt_c: PK_replace_use_t = 0;
pub const PK_replace_use_yes_c: PK_replace_use_t = 1;
pub const PK_replace_use_existing_c: PK_replace_use_t = 2;

pub type PK_replace_variation_t = c_int;
pub const PK_replace_variation_no_c: PK_replace_variation_t = 0;

/// Edge replacement data for surface replacement.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_replace_edge_data_t {
    pub n_edges: c_int,
    pub edges: *const PK_EDGE_t,
    pub curves: *const PK_CURVE_t,
    pub tolerances: *const c_double,
    pub replace_use: PK_replace_use_t,
}

/// Variation data for surface replacement.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_replace_variation_data_t {
    pub variation: PK_replace_variation_t,
    pub n_variation_faces: c_int,
    pub variation_faces: *const PK_FACE_t,
}

/// Options for `PK_FACE_replace_surfs_3`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_replace_surfs_o_t {
    pub o_t_version: c_int,
    pub merge: PK_replace_merge_t,
    pub variation_data: PK_replace_variation_data_t,
    pub edge_data: PK_replace_edge_data_t,
    pub adjust: PK_LOGICAL_t,
    pub update: PK_local_ops_update_t,
}

// =============================================================================
// Taper constants and options (Ch. 64, 65)
// =============================================================================

pub const PK_taper_method_surface_c: PK_taper_method_t = 2;

pub type PK_taper_smooth_step_t = c_int;
pub const PK_taper_smooth_step_no_c: PK_taper_smooth_step_t = 0;

pub type PK_taper_step_face_t = c_int;
pub const PK_taper_step_face_no_c: PK_taper_step_face_t = 0;
pub const PK_taper_step_face_yes_c: PK_taper_step_face_t = 1;
pub const PK_taper_preserve_smooth_c: PK_taper_step_face_t = 2;

pub type PK_taper_laminar_edge_t = c_int;
pub const PK_taper_laminar_edge_normal_c: PK_taper_laminar_edge_t = 0;
pub const PK_taper_laminar_edge_draw_c: PK_taper_laminar_edge_t = 1;

pub const PK_FACE_grow_auto_c: PK_FACE_grow_t = 3;
pub const PK_FACE_grow_update_c: PK_FACE_grow_t = 4;
pub const PK_FACE_grow_fail_c: PK_FACE_grow_t = 5;

/// Options for `PK_FACE_taper`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_taper_o_t {
    pub o_t_version: c_int,
    pub merge_face: PK_LOGICAL_t,
    pub check_fa_fa: PK_check_fa_fa_t,
    pub n_tapered_edges: c_int,
    pub tapered_edges: *const PK_EDGE_t,
    pub n_normal_edges: c_int,
    pub normal_edges: *const PK_EDGE_t,
    pub method: PK_taper_method_t,
    pub offset: c_double,
    pub top_surface: PK_SURF_t,
    pub taper_smooth_step: PK_taper_smooth_step_t,
    pub taper_step_face: PK_taper_step_face_t,
    pub n_faces: c_int,
    pub taper_faces: *const PK_FACE_t,
    pub angles: *const c_double,
    pub position: PK_taper_laminar_edge_t,
    pub grow: PK_FACE_grow_t,
    pub n_parting_edges: c_int,
    pub parting_edges: *const PK_EDGE_t,
    pub parting_body: PK_BODY_t,
    pub update: PK_local_ops_update_t,
}

// --- Body taper (Ch. 65) ---

pub type PK_taper_miter_t = c_int;
pub const PK_taper_miter_on_ref_c: PK_taper_miter_t = 0;
pub const PK_taper_miter_to_face_c: PK_taper_miter_t = 1;

pub type PK_taper_corner_t = c_int;
pub const PK_taper_corner_extend_c: PK_taper_corner_t = 0;
pub const PK_taper_corner_plane_c: PK_taper_corner_t = 1;

pub type PK_taper_undercut_ref_t = c_int;
pub const PK_taper_undercut_ref_no_c: PK_taper_undercut_ref_t = 0;
pub const PK_taper_undercut_ref_yes_c: PK_taper_undercut_ref_t = 1;

pub type PK_taper_concave_t = c_int;
pub const PK_taper_concave_none_c: PK_taper_concave_t = 0;
pub const PK_taper_concave_radius_c: PK_taper_concave_t = 1;
pub const PK_taper_concave_mix_c: PK_taper_concave_t = 2;

pub type PK_taper_keep_material_t = c_int;
pub const PK_taper_keep_material_no_c: PK_taper_keep_material_t = 0;
pub const PK_taper_keep_material_yes_c: PK_taper_keep_material_t = 1;

pub type PK_isocline_split_t = c_int;
pub const PK_isocline_split_convexity_c: PK_isocline_split_t = 0;

/// Options for `PK_BODY_taper`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_taper_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub miter_at_parting: PK_LOGICAL_t,
    pub miter_type: PK_taper_miter_t,
    pub n_non_miter_edges: c_int,
    pub non_miter_edges: *const PK_EDGE_t,
    pub merge_face: PK_LOGICAL_t,
    pub check_fa_fa: PK_check_fa_fa_t,
    pub default_method: PK_taper_method_t,
    pub n_methods: c_int,
    pub methods: *const PK_taper_method_t,
    pub method_refs: *const PK_EDGE_t,
    pub corner_type: PK_taper_corner_t,
    pub n_parting_edges: c_int,
    pub parting_edges: *const PK_EDGE_t,
    pub n_replace_edges: c_int,
    pub replace_edges: *const PK_EDGE_t,
    pub undercut: PK_taper_undercut_ref_t,
    pub n_upper_faces: c_int,
    pub upper_faces: *const PK_FACE_t,
    pub n_lower_faces: c_int,
    pub lower_faces: *const PK_FACE_t,
    pub concave_type: PK_taper_concave_t,
    pub concave_radius: c_double,
    pub keep_material: PK_taper_keep_material_t,
    pub update: PK_local_ops_update_t,
}

// =============================================================================
// Patching (Ch. 66)
// =============================================================================

pub type PK_replace_patch_t = c_int;
pub const PK_replace_patch_no_c: PK_replace_patch_t = 0;
pub const PK_replace_patch_yes_c: PK_replace_patch_t = 1;

/// Patch data for `PK_FACE_replace_with_sheet`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_replace_patch_data_t {
    pub patch_type: PK_replace_patch_t,
    pub n_matches: c_int,
    pub tool_patches: *const PK_FACE_t,
    pub target_patches: *const PK_FACE_t,
    pub n_patch_edges: c_int,
    pub patch_edges: *const PK_EDGE_t,
}

/// Options for `PK_FACE_replace_with_sheet`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_replace_with_sheet_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub check_fa_fa: PK_check_fa_fa_t,
    pub patch_data: PK_replace_patch_data_t,
}

// --- PK_FACE_cover (Ch. 66) ---

pub type PK_FACE_cover_smooth_t = c_int;
pub const PK_FACE_cover_smooth_no_c: PK_FACE_cover_smooth_t = 0;

pub type PK_cover_param_prefer_t = c_int;
pub const PK_cover_param_prefer_any_c: PK_cover_param_prefer_t = 0;
pub const PK_cover_param_prefer_uvbox_c: PK_cover_param_prefer_t = 1;

/// Options for `PK_FACE_cover`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_cover_o_t {
    pub o_t_version: c_int,
    pub fitting_tolerance: c_double,
    pub have_bdry_tolerance: PK_LOGICAL_t,
    pub bdry_tolerance: c_double,
    pub maintain_bdry_smoothness: PK_FACE_cover_smooth_t,
    pub param_prefer: PK_cover_param_prefer_t,
    pub update: PK_FACE_cover_update_t,
}

// =============================================================================
// Hole filling (Ch. 67)
// =============================================================================

pub type PK_fill_hole_method_t = c_int;
pub const PK_fill_hole_create_patch_c: PK_fill_hole_method_t = 0;
pub const PK_fill_hole_trim_to_hole_c: PK_fill_hole_method_t = 1;
pub const PK_fill_hole_trim_to_sheet_c: PK_fill_hole_method_t = 2;
pub const PK_fill_hole_extend_adjacent_c: PK_fill_hole_method_t = 3;

pub type PK_fill_hole_topol_t = c_int;
pub const PK_fill_hole_topol_multiple_c: PK_fill_hole_topol_t = 0;
pub const PK_fill_hole_topol_single_c: PK_fill_hole_topol_t = 1;
pub const PK_fill_hole_topol_minimal_c: PK_fill_hole_topol_t = 2;

pub type PK_fill_hole_pref_t = c_int;
pub const PK_fill_hole_smooth_c: PK_fill_hole_pref_t = 0;
pub const PK_fill_hole_non_smooth_c: PK_fill_hole_pref_t = 1;
pub const PK_fill_hole_plane_only_c: PK_fill_hole_pref_t = 2;
pub const PK_fill_hole_prefer_plane_c: PK_fill_hole_pref_t = 3;

pub type PK_fill_hole_non_smooth_t = c_int;
pub const PK_fill_hole_non_smooth_fail_c: PK_fill_hole_non_smooth_t = 0;
pub const PK_fill_hole_non_smooth_allow_c: PK_fill_hole_non_smooth_t = 1;

pub type PK_fill_hole_patch_eds_t = c_int;
pub const PK_fill_hole_patch_eds_sharp_c: PK_fill_hole_patch_eds_t = 0;
pub const PK_fill_hole_patch_eds_smooth_c: PK_fill_hole_patch_eds_t = 1;

pub type PK_fill_hole_imprint_t = c_int;
pub const PK_fill_hole_imprint_sharp_c: PK_fill_hole_imprint_t = 0;
pub const PK_fill_hole_imprint_yes_c: PK_fill_hole_imprint_t = 1;

pub type PK_fill_hole_clamp_type_t = c_int;
pub const PK_fill_hole_clamp_no_c: PK_fill_hole_clamp_type_t = 0;
pub const PK_fill_hole_clamp_planar_c: PK_fill_hole_clamp_type_t = 1;

pub type PK_fill_hole_opt_t = c_int;
pub const PK_fill_hole_opt_quality_c: PK_fill_hole_opt_t = 0;
pub const PK_fill_hole_opt_performance_c: PK_fill_hole_opt_t = 1;

pub type PK_fill_hole_body_type_t = c_int;
pub const PK_fill_hole_body_type_orig_c: PK_fill_hole_body_type_t = 0;
pub const PK_fill_hole_body_type_sheet_c: PK_fill_hole_body_type_t = 1;
pub const PK_fill_hole_body_type_solid_c: PK_fill_hole_body_type_t = 2;

pub type PK_fill_hole_track_t = c_int;
pub const PK_fill_hole_track_default_c: PK_fill_hole_track_t = 0;
pub const PK_fill_hole_track_bdry_edges_c: PK_fill_hole_track_t = 1;

pub type PK_fill_hole_update_t = c_int;
pub const PK_fill_hole_update_default_c: PK_fill_hole_update_t = 0;

pub type PK_fill_hole_fault_t = c_int;
/// Hole too complex to fill.
pub const PK_fill_hole_too_complex_c: PK_fill_hole_fault_t = 1;

/// Clamp specification for fill hole with acorn supporting bodies.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_fill_hole_clamp_t {
    pub clamp_type: PK_fill_hole_clamp_type_t,
    pub have_normal: PK_LOGICAL_t,
    pub normal: PK_VECTOR_t,
}

/// Options for `PK_BODY_fill_hole`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_fill_hole_o_t {
    pub o_t_version: c_int,
    pub method: PK_fill_hole_method_t,
    pub fill_sheet: PK_BODY_t,
    pub patch_topology: PK_fill_hole_topol_t,
    pub fill_preference: PK_fill_hole_pref_t,
    pub n_non_smooth_edges: c_int,
    pub non_smooth_edges: *const PK_EDGE_t,
    pub n_non_g2_smooth_edges: c_int,
    pub non_g2_smooth_edges: *const PK_EDGE_t,
    pub non_g1_behaviour: PK_fill_hole_non_smooth_t,
    pub non_g2_behaviour: PK_fill_hole_non_smooth_t,
    pub smoothness: PK_continuity_t,
    pub internal_smoothness: PK_fill_hole_patch_eds_t,
    pub attach_sheet: PK_LOGICAL_t,
    pub n_supporting_bodies: c_int,
    pub supporting_bodies: *const PK_BODY_t,
    pub imprint_supporting_bodies: PK_fill_hole_imprint_t,
    pub n_clamps: c_int,
    pub clamps: *const PK_fill_hole_clamp_t,
    pub clamp_indices: *const c_int,
    pub optimise: PK_fill_hole_opt_t,
    pub body_type: PK_fill_hole_body_type_t,
    pub check_fa_fa: PK_check_fa_fa_t,
    pub update: PK_fill_hole_update_t,
    pub tracking_control: PK_fill_hole_track_t,
}

// =============================================================================
// Moving faces / PK_FACE_transform_2 (Ch. 68)
// =============================================================================

/// Face adjacency preference.
pub type PK_FACE_prefer_adj_t = c_int;
pub const PK_FACE_prefer_adj_keep_c: PK_FACE_prefer_adj_t = 0;
pub const PK_FACE_prefer_adj_change_c: PK_FACE_prefer_adj_t = 1;

/// Options for `PK_FACE_transform_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_transform_2_o_t {
    pub o_t_version: c_int,
    pub grow: PK_FACE_grow_t,
    pub adjacency: PK_FACE_prefer_adj_t,
}

// =============================================================================
// Body creation from entities (Ch. 69)
// =============================================================================

pub type PK_track_bodies_t = c_int;
pub const PK_track_bodies_no_c: PK_track_bodies_t = 0;
pub const PK_track_bodies_yes_c: PK_track_bodies_t = 1;

/// Options for `PK_EDGE_remove_to_bodies`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_remove_to_bodies_o_t {
    pub o_t_version: c_int,
    pub allow_disjoint: PK_LOGICAL_t,
    pub track_bodies: PK_track_bodies_t,
}

// =============================================================================
// Generic face editing / PK_FACE_change (Ch. 70)
// =============================================================================

/// Operation type for `PK_FACE_change`.
pub type PK_FACE_change_type_t = c_int;
pub const PK_FACE_change_type_none_c: PK_FACE_change_type_t = 0;
pub const PK_FACE_change_type_offset_c: PK_FACE_change_type_t = 1;
pub const PK_FACE_change_type_taper_c: PK_FACE_change_type_t = 2;
pub const PK_FACE_change_type_transform_c: PK_FACE_change_type_t = 3;
pub const PK_FACE_change_type_replace_c: PK_FACE_change_type_t = 4;
pub const PK_FACE_change_type_blend_c: PK_FACE_change_type_t = 5;
pub const PK_FACE_change_type_bend_c: PK_FACE_change_type_t = 6;
pub const PK_FACE_change_type_patch_c: PK_FACE_change_type_t = 7;
pub const PK_FACE_change_type_deform_c: PK_FACE_change_type_t = 8;
pub const PK_FACE_change_type_radiate_c: PK_FACE_change_type_t = 9;

// --- Offset operation types ---

pub type PK_ref_alignment_t = c_int;
pub const PK_ref_alignment_opposed_c: PK_ref_alignment_t = 0;
pub const PK_ref_alignment_aligned_c: PK_ref_alignment_t = 1;

/// Offset operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_offset_t {
    pub distance: c_double,
}

/// Offset operation options.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_offset_o_t {
    pub offset_methods: PK_offset_method_t,
    pub offset_steps: PK_offset_step_t,
    pub references: PK_FACE_t,
    pub ref_alignment: PK_ref_alignment_t,
}

// --- Taper operation data (within PK_FACE_change) ---

/// Taper operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_taper_t {
    pub direction: PK_VECTOR_t,
    pub angle: c_double,
    pub n_refs: c_int,
    pub references: *const PK_ENTITY_t,
}

/// Taper operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_taper_o_t {
    pub n_tapered_edges: c_int,
    pub tapered_edges: *const PK_EDGE_t,
    pub n_normal_edges: c_int,
    pub normal_edges: *const PK_EDGE_t,
    pub method: PK_taper_method_t,
    pub taper_smooth_step: PK_taper_smooth_step_t,
    pub taper_step_face: PK_taper_step_face_t,
    pub position: PK_taper_laminar_edge_t,
    pub n_parting_edges: c_int,
    pub parting_edges: *const PK_EDGE_t,
    pub parting_body: PK_BODY_t,
}

// --- Transform operation data ---

pub type PK_transform_step_t = c_int;
pub const PK_transform_step_no_c: PK_transform_step_t = 0;
pub const PK_transform_step_smooth_c: PK_transform_step_t = 1;
pub const PK_transform_step_smooth_site_c: PK_transform_step_t = 2;
pub const PK_transform_step_not_coi_c: PK_transform_step_t = 3;
pub const PK_transform_step_all_c: PK_transform_step_t = 4;

pub type PK_transform_intent_t = c_int;
pub const PK_transform_intent_minimal_c: PK_transform_intent_t = 0;
pub const PK_transform_intent_grow_c: PK_transform_intent_t = 1;
pub const PK_transform_intent_trim_c: PK_transform_intent_t = 2;

/// Transform operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_transform_t {
    pub transform: PK_TRANSF_t,
}

/// Transform operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_transform_o_t {
    pub n_loops: c_int,
    pub base_loops: *const PK_LOOP_t,
    pub target_faces: *const PK_FACE_t,
    pub transform_step: PK_transform_step_t,
    pub transform_intent: PK_transform_intent_t,
}

// --- Replace operation data ---

/// Replace operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_replace_t {
    pub surface: PK_SURF_t,
    pub sense: PK_LOGICAL_t,
}

/// Replace operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_replace_o_t {
    pub merge: PK_replace_merge_t,
    pub variation: PK_replace_variation_t,
}

// --- Blend (reblend) operation types ---

/// Blend (reblend) operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_blend_o_t {
    pub xs_shape: PK_blend_xs_shape_t,
    pub radius: c_double,
    pub n_unders: c_int,
    pub unders: *const PK_FACE_t,
    pub orientations: *const PK_LOGICAL_t,
    pub ranges: *const c_double,
    pub ov_smooth: PK_blend_ov_smooth_t,
}

// --- Bend operation data ---

/// Bend operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_bend_t {
    pub bend_type: c_int,
    pub tool_entity: PK_ENTITY_t,
    pub offset: c_double,
    pub backward_offset: c_double,
}

/// Bend operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_bend_o_t {
    pub merge: PK_LOGICAL_t,
}

// --- Patch operation data ---

/// Patch operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_patch_t {
    pub sheet: PK_BODY_t,
}

pub type PK_patch_mobility_t = c_int;
pub const PK_patch_mobility_fixed_c: PK_patch_mobility_t = 0;
pub const PK_patch_mobility_moving_c: PK_patch_mobility_t = 1;

/// Patch operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_patch_o_t {
    pub patch_data: PK_replace_patch_data_t,
    pub mobility: PK_patch_mobility_t,
}

// --- Deform operation data ---

/// Deform evaluator function pointer.
pub type PK_FACE_change_deform_fn_t = Option<
    unsafe extern "C" fn(
        position: *const c_double,
        face: PK_FACE_t,
        have_params: PK_LOGICAL_t,
        params: *const c_double,
        external_data: *mut c_void,
        deformed_position: *mut c_double,
    ) -> PK_ERROR_code_t,
>;

/// Deform operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_deform_t {
    pub eval_fn: PK_FACE_change_deform_fn_t,
    pub eval_data: *mut c_void,
}

pub type PK_deform_uv_t = c_int;
pub const PK_deform_uv_face_box_c: PK_deform_uv_t = 0;
pub const PK_deform_uv_all_c: PK_deform_uv_t = 1;

/// Deform operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_deform_o_t {
    pub n_matched_edges: c_int,
    pub matched_edges: *const PK_EDGE_t,
    pub thread_safe: PK_LOGICAL_t,
    pub deform_uv: PK_deform_uv_t,
}

// --- Radiate operation data ---

/// Radiate operation data.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_data_radiate_t {
    pub radial_displacement: c_double,
}

pub type PK_radiate_step_t = c_int;
pub const PK_radiate_step_no_c: PK_radiate_step_t = 0;
pub const PK_radiate_step_smooth_c: PK_radiate_step_t = 1;
pub const PK_radiate_step_smooth_site_c: PK_radiate_step_t = 2;
pub const PK_radiate_step_not_coi_c: PK_radiate_step_t = 3;
pub const PK_radiate_step_all_c: PK_radiate_step_t = 4;

/// Axis definition for radiate operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_AXIS1_t {
    pub location: PK_VECTOR_t,
    pub axis: PK_VECTOR_t,
}

/// Radiate operation options within PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_change_radiate_o_t {
    pub have_axis: PK_LOGICAL_t,
    pub axis: PK_AXIS1_t,
    pub axial_displacement: c_double,
    pub radiate_step: PK_radiate_step_t,
}

// --- Edge geometry data for PK_FACE_change ---

pub type PK_change_edge_method_t = c_int;
pub const PK_change_edge_method_entity_c: PK_change_edge_method_t = 0;
pub const PK_change_edge_method_swept_c: PK_change_edge_method_t = 1;
pub const PK_change_edge_method_ruled_c: PK_change_edge_method_t = 2;
pub const PK_change_edge_method_proj_c: PK_change_edge_method_t = 3;

pub type PK_EDGE_step_t = c_int;
pub const PK_EDGE_step_default_c: PK_EDGE_step_t = 0;
pub const PK_EDGE_step_no_c: PK_EDGE_step_t = 1;
pub const PK_EDGE_step_auto_c: PK_EDGE_step_t = 2;
pub const PK_EDGE_step_method_c: PK_EDGE_step_t = 3;

/// Edge geometry control data for PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_change_edge_geom_data_t {
    pub n_edge_arrays: c_int,
    pub edge_arrays: *const *const PK_EDGE_t,
    pub edge_array_sizes: *const c_int,
    pub entities: *const PK_ENTITY_t,
    pub methods: *const PK_change_edge_method_t,
    pub steps: *const PK_EDGE_step_t,
    pub directions: *const PK_VECTOR_t,
    pub reversals: *const PK_LOGICAL_t,
    pub offsets: *const c_double,
    pub transforms: *const PK_TRANSF_t,
    pub tolerances: *const c_double,
    pub replace_uses: *const PK_replace_use_t,
}

/// Vertex geometry data for PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_change_vertex_data_t {
    pub n_vertices: c_int,
    pub vertices: *const PK_VERTEX_t,
    pub positions: *const PK_VECTOR_t,
}

/// Help points for edges/vertices in PK_FACE_change.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_replace_help_points_t {
    pub n_topols: c_int,
    pub topols: *const PK_TOPOL_t,
    pub points: *const PK_VECTOR_t,
}

// --- Change track edges ---

pub type PK_change_track_edges_t = c_int;
pub const PK_change_track_edges_no_c: PK_change_track_edges_t = 0;
pub const PK_change_track_edges_laminar_c: PK_change_track_edges_t = 1;

// --- Results output ---

// --- Grow callback ---

pub type PK_FACE_grow_cb_t = c_int;
pub const PK_FACE_grow_cb_auto_c: PK_FACE_grow_cb_t = 0;
pub const PK_FACE_grow_cb_default_c: PK_FACE_grow_cb_t = 1;

/// Grow callback function pointer.
pub type PK_FACE_grow_cb_f_t = Option<
    unsafe extern "C" fn(
        n_faces1: c_int,
        faces1: *const PK_FACE_t,
        n_faces2: c_int,
        faces2: *const PK_FACE_t,
        n_site: c_int,
        site: *const PK_FACE_t,
        context: *mut c_void,
        grow: *mut PK_FACE_grow_cb_t,
    ) -> PK_ERROR_code_t,
>;

/// Trim callback function pointer.
pub type PK_FACE_trim_cb_f_t = Option<
    unsafe extern "C" fn(
        n_faces1: c_int,
        faces1: *const PK_FACE_t,
        n_faces2: c_int,
        faces2: *const PK_FACE_t,
        n_site: c_int,
        site: *const PK_FACE_t,
        context: *mut c_void,
        trim: *mut c_int,
    ) -> PK_ERROR_code_t,
>;

// --- PK_FACE_change operation / options structures (tagged union) ---

/// Single operation for PK_FACE_change. op_param and op_opts are unions;
/// interpret based on `op_type`.
///
/// In C this is a tagged union. We expose it as an opaque blob of sufficient
/// size; callers construct it through helpers or transmute individual data
/// structs. The largest variant data struct determines the size.
///
/// For a fully type-safe wrapper, see the safe `parasolid` crate.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PK_FACE_change_t {
    pub op_type: PK_FACE_change_type_t,
    /// Raw storage for the operation parameter union.
    /// Largest member: PK_FACE_change_data_deform_t (fn ptr + void*) = 16 bytes.
    pub op_param: [u8; 64],
    /// Raw storage for the operation options union.
    /// Largest member: PK_FACE_change_blend_o_t.
    pub op_opts: [u8; 128],
}

/// Global options for `PK_FACE_change`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PK_FACE_change_o_t {
    pub o_t_version: c_int,
    pub merge_face: PK_LOGICAL_t,
    pub allow_disjoint: PK_LOGICAL_t,
    pub check_fa_fa: PK_check_fa_fa_t,
    pub edge_geom_data: PK_change_edge_geom_data_t,
    pub vertex_data: PK_change_vertex_data_t,
    pub edge_help: PK_replace_help_points_t,
    pub vertex_help: PK_replace_help_points_t,
    pub update: PK_local_ops_update_t,
    pub adjacency: PK_FACE_prefer_adj_t,
    pub grow: PK_FACE_grow_t,
    pub grow_data: *mut c_void,
    pub grow_cb: PK_FACE_grow_cb_f_t,
    pub repair_fa_fa: PK_repair_fa_fa_t,
    pub repair_fa: PK_repair_fa_t,
    pub trim_data: *mut c_void,
    pub trim_cb: PK_FACE_trim_cb_f_t,
    pub report_surf_extension: PK_LOGICAL_t,
    pub track_edges: PK_change_track_edges_t,
    pub results_output: PK_results_output_t,
}

// =============================================================================
// PK_FACE_offset_2 options
// =============================================================================

/// Options for `PK_FACE_offset_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_offset_2_o_t {
    pub o_t_version: c_int,
    pub update: PK_local_ops_update_t,
}

// =============================================================================
// PK_BODY_extend options
// =============================================================================

// =============================================================================
// Error codes specific to editing
// =============================================================================

/// Cannot heal wound (edge delete).
pub const PK_ERROR_cant_heal_wound: PK_ERROR_code_t = 600;
/// Invalid option combination.
pub const PK_ERROR_bad_combination: PK_ERROR_code_t = 602;
/// Face change operation failed.
pub const PK_ERROR_failed_to_change: PK_ERROR_code_t = 603;

// =============================================================================
// Loop types (referenced by delete/fill operations)
// =============================================================================

// =============================================================================
// Opaque options/result types for editing operations
// =============================================================================

/// Options for `PK_BODY_create_implicit`.
#[repr(C)]
pub struct PK_BODY_create_implicit_o_t { _private: [u8; 0] }

/// Results from `PK_BODY_create_implicit`.
#[repr(C)]
pub struct PK_BODY_create_implicit_r_t { _private: [u8; 0] }

/// Options for `PK_BODY_is_cellular`.
#[repr(C)]
pub struct PK_BODY_is_cellular_o_t { _private: [u8; 0] }

/// Results from `PK_BODY_is_cellular`.
#[repr(C)]
pub struct PK_BODY_is_cellular_r_t { _private: [u8; 0] }

/// Options for `PK_BODY_is_disjoint`.
#[repr(C)]
pub struct PK_BODY_is_disjoint_o_t { _private: [u8; 0] }

/// Results from `PK_BODY_is_disjoint`.
#[repr(C)]
pub struct PK_BODY_is_disjoint_r_t { _private: [u8; 0] }

/// Options for `PK_BODY_enlarge`.
#[repr(C)]
pub struct PK_BODY_enlarge_o_t { _private: [u8; 0] }

/// Results from `PK_BODY_enlarge`.
#[repr(C)]
pub struct PK_BODY_enlarge_r_t { _private: [u8; 0] }

/// Options for `PK_BODY_slice`.
#[repr(C)]
pub struct PK_BODY_slice_o_t { _private: [u8; 0] }

/// Results from `PK_BODY_slice`.
#[repr(C)]
pub struct PK_BODY_slice_r_t { _private: [u8; 0] }

/// Options for `PK_BODY_make_patterned`.
#[repr(C)]
pub struct PK_BODY_make_patterned_o_t { _private: [u8; 0] }

/// Results from `PK_BODY_make_patterned`.
#[repr(C)]
pub struct PK_BODY_make_patterned_r_t { _private: [u8; 0] }

/// Options for `PK_FACE_make_valid_faces`.
#[repr(C)]
pub struct PK_FACE_make_valid_faces_o_t { _private: [u8; 0] }

/// Results from `PK_FACE_make_valid_faces`.
#[repr(C)]
pub struct PK_FACE_make_valid_faces_r_t { _private: [u8; 0] }

/// Options for `PK_FACE_repair`.
#[repr(C)]
pub struct PK_FACE_repair_o_t { _private: [u8; 0] }

/// Options for `PK_FACE_fix_mesh_defects`.
#[repr(C)]
pub struct PK_FACE_fix_mesh_defects_o_t { _private: [u8; 0] }

/// Results from `PK_FACE_fix_mesh_defects`.
#[repr(C)]
pub struct PK_FACE_fix_mesh_defects_r_t { _private: [u8; 0] }

/// Tracking results for entity operations.
#[repr(C)]
pub struct PK_ENTITY_track_r_t { _private: [u8; 0] }

/// Options for `PK_REGION_embed_body`.
#[repr(C)]
pub struct PK_REGION_embed_body_o_t { _private: [u8; 0] }

/// Results from `PK_REGION_embed_body`.
#[repr(C)]
pub struct PK_REGION_embed_body_r_t { _private: [u8; 0] }

/// Local operation results.
#[repr(C)]
pub struct PK_TOPOL_local_r_t { _private: [u8; 0] }

// =============================================================================
// Extern function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    // =========================================================================
    // Face deletion (Ch. 61)
    // =========================================================================

    /// Delete an arbitrary set of faces from a body and heal wounds.
    pub fn PK_FACE_delete_2(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        options: *const PK_FACE_delete_2_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Delete face sets from a body (typically identified by blend/detail functions).
    pub fn PK_FACE_delete_facesets(
        n_facesets: c_int,
        facesets: *const *const PK_FACE_t,
        faceset_sizes: *const c_int,
        options: *const PK_FACE_delete_facesets_o_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
        n_failed_facesets: *mut c_int,
        failed_facesets_indices: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Remove faces from a sheet body (alternate entry point).
    pub fn PK_FACE_delete_from_sheet_body(
        n_faces: c_int,
        faces: *const PK_FACE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Edge deletion (Ch. 61)
    // =========================================================================

    /// Remove trimmed boundary features (laminar/wire edges).
    pub fn PK_EDGE_delete(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        options: *const PK_EDGE_delete_o_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Loop / region / topology deletion (Ch. 61)
    // =========================================================================

    /// Delete interior loops from a sheet body.
    pub fn PK_LOOP_delete_from_sheet_body(
        n_loops: c_int,
        loops: *const PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Simplification and detail identification (Ch. 62)
    // =========================================================================

    /// Find distinct face sets bounded by given edges.
    pub fn PK_BODY_find_facesets(
        body: PK_BODY_t,
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        options: *const PK_BODY_find_facesets_o_t,
        results: *mut PK_BODY_find_facesets_r_t,
    ) -> PK_ERROR_code_t;

    /// Free results from PK_BODY_find_facesets.
    pub fn PK_BODY_find_facesets_r_f(
        results: *mut PK_BODY_find_facesets_r_t,
    ) -> PK_ERROR_code_t;

    /// Identify specific types of details (holes, rubber faces) in a body.
    pub fn PK_BODY_identify_details(
        body: PK_BODY_t,
        n_details: c_int,
        details: *const PK_detail_t,
        options: *const PK_BODY_identify_details_o_t,
        results: *mut PK_identify_details_r_t,
    ) -> PK_ERROR_code_t;

    /// Classify facesets by detail type.
    pub fn PK_FACE_classify_details(
        n_facesets: c_int,
        facesets: *const *const PK_FACE_t,
        faceset_sizes: *const c_int,
        n_details: c_int,
        details: *const PK_detail_t,
        options: *const PK_FACE_classify_details_o_t,
        n_results: *mut c_int,
        results: *mut *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Free results from PK_FACE_classify_details.
    pub fn PK_FACE_classify_details_r_f(
        n_results: c_int,
        results: *mut c_void,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Surface replacement (Ch. 63)
    // =========================================================================

    /// Replace the surface of specified faces (version 2).
    pub fn PK_FACE_replace_surfs_2(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        surfaces: *const PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Replace the surface of specified faces (version 1).
    pub fn PK_FACE_replace_surfs(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        surfaces: *const PK_SURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Face tapering (Ch. 64)
    // =========================================================================

    /// Taper (draft) specific faces in a body.
    pub fn PK_FACE_taper(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        references: *const PK_ENTITY_t,
        direction: *const c_double,
        angle: c_double,
        tolerance: c_double,
        options: *const PK_FACE_taper_o_t,
    ) -> PK_ERROR_code_t;

    /// Imprint isocline curves on faces.
    pub fn PK_FACE_imprint_cus_isoclin(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        direction: *const c_double,
        angle: c_double,
        tolerance: c_double,
        n_ret_faces: *mut c_int,
        ret_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body tapering (Ch. 65)
    // =========================================================================

    /// Taper a solid body using a parting body.
    pub fn PK_BODY_taper(
        body: PK_BODY_t,
        parting_body: PK_BODY_t,
        n_refs_above: c_int,
        refs_above: *const PK_EDGE_t,
        n_refs_below: c_int,
        refs_below: *const PK_EDGE_t,
        angle_above: c_double,
        angle_below: c_double,
        direction: *const c_double,
        options: *const PK_BODY_taper_o_t,
        status: *mut PK_local_status_t,
        n_error_entities: *mut c_int,
        error_entities: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Patching (Ch. 66)
    // =========================================================================

    /// Replace target faces with tool faces from a sheet body.
    pub fn PK_FACE_replace_with_sheet(
        n_target_faces: c_int,
        target_faces: *const PK_FACE_t,
        tool: PK_BODY_t,
        options: *const PK_FACE_replace_with_sheet_o_t,
    ) -> PK_ERROR_code_t;

    /// Replace smoothly connected faces with a single B-surface face.
    pub fn PK_FACE_cover(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        options: *const PK_FACE_cover_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Hole filling (Ch. 67)
    // =========================================================================

    /// Fill holes in a body.
    pub fn PK_BODY_fill_hole(
        target: PK_BODY_t,
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        tolerance: c_double,
        options: *const PK_BODY_fill_hole_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        fault: *mut PK_fill_hole_fault_t,
        n_fault_topols: *mut c_int,
        fault_topols: *mut *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Moving faces (Ch. 68)
    // =========================================================================

    /// Transform faces in a body (version 1).
    pub fn PK_FACE_transform(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        transform: PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Offset faces by a specified distance (version 1).
    pub fn PK_FACE_offset(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        distance: c_double,
    ) -> PK_ERROR_code_t;

    /// Imprint a curve on a face.
    pub fn PK_FACE_imprint_curve(
        face: PK_FACE_t,
        curve: PK_CURVE_t,
        interval: *const PK_INTERVAL_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Imprint isocline curves on faces.
    pub fn PK_FACE_imprint_curves_isocline(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        direction: *const c_double,
        angle: c_double,
        tolerance: c_double,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Spin and sweep (Ch. 68)
    // =========================================================================

    /// Spin a body around an axis.
    pub fn PK_BODY_spin(
        body: PK_BODY_t,
        axis_position: *const c_double,
        axis_direction: *const c_double,
        angle: c_double,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Sweep a body along a direction.
    pub fn PK_BODY_sweep(
        body: PK_BODY_t,
        direction: *const c_double,
        distance: c_double,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Spin a curve to create a surface (version 2).
    pub fn PK_CURVE_spin_2(
        curve: PK_CURVE_t,
        interval: *const PK_INTERVAL_t,
        axis_position: *const c_double,
        axis_direction: *const c_double,
        angle: c_double,
        surface: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Spin a curve to create a surface (version 1).
    pub fn PK_CURVE_spin(
        curve: PK_CURVE_t,
        interval: *const PK_INTERVAL_t,
        axis_position: *const c_double,
        axis_direction: *const c_double,
        angle: c_double,
        surface: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Sweep a curve to create a surface.
    pub fn PK_CURVE_sweep(
        curve: PK_CURVE_t,
        interval: *const PK_INTERVAL_t,
        direction: *const c_double,
        distance: c_double,
        surface: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Spin faces of a body.
    pub fn PK_FACE_spin(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        axis_position: *const c_double,
        axis_direction: *const c_double,
        angle: c_double,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Sweep faces of a body.
    pub fn PK_FACE_sweep(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        direction: *const c_double,
        distance: c_double,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Spin an end vertex of a wire body.
    pub fn PK_VERTEX_spin(
        vertex: PK_VERTEX_t,
        axis_position: *const c_double,
        axis_direction: *const c_double,
        angle: c_double,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Sweep an end vertex of a wire body.
    pub fn PK_VERTEX_sweep(
        vertex: PK_VERTEX_t,
        direction: *const c_double,
        distance: c_double,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body creation from entities (Ch. 69)
    // =========================================================================

    /// Remove faces from a body to create new solid bodies.
    pub fn PK_FACE_remove_to_solid_bodies(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_parent_bodies: *mut c_int,
        parent_bodies: *mut *mut PK_BODY_t,
        n_child_bodies: *mut c_int,
        child_bodies: *mut *mut PK_BODY_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Remove wireframe edges from a body to create new bodies.
    pub fn PK_EDGE_remove_to_bodies(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        options: *const PK_EDGE_remove_to_bodies_o_t,
        n_parent_bodies: *mut c_int,
        parent_bodies: *mut *mut PK_BODY_t,
        n_child_bodies: *mut c_int,
        child_bodies: *mut *mut PK_BODY_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Generic face editing (Ch. 70)
    // =========================================================================

    /// Perform multiple local operations on faces in a single call.
    pub fn PK_FACE_change(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        mapping: *const c_int,
        n_operations: c_int,
        operations: *const PK_FACE_change_t,
        tolerance: c_double,
        options: *const PK_FACE_change_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body extension
    // =========================================================================

    // =========================================================================
    // Body imprinting
    // =========================================================================

    /// Imprint a curve onto a body.
    pub fn PK_BODY_imprint_curve(
        body: PK_BODY_t,
        curve: PK_CURVE_t,
        interval: *const PK_INTERVAL_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Imprint a plane onto a body.
    pub fn PK_BODY_imprint_plane(
        body: PK_BODY_t,
        plane: PK_PLANE_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Imprint faces onto a body.
    pub fn PK_BODY_imprint_faces(
        target: PK_BODY_t,
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body query and creation (implicit/cellular/disjoint/enlarge/slice/pattern)
    // =========================================================================

    /// Create implicit/procedural body.
    pub fn PK_BODY_create_implicit(
        options: *const PK_BODY_create_implicit_o_t,
        results: *mut PK_BODY_create_implicit_r_t,
    ) -> PK_ERROR_code_t;

    /// Query whether body has cellular structure.
    pub fn PK_BODY_is_cellular(
        body: PK_BODY_t,
        options: *const PK_BODY_is_cellular_o_t,
        results: *mut PK_BODY_is_cellular_r_t,
    ) -> PK_ERROR_code_t;

    /// Query whether body contains disjoint shells.
    pub fn PK_BODY_is_disjoint(
        body: PK_BODY_t,
        options: *const PK_BODY_is_disjoint_o_t,
        results: *mut PK_BODY_is_disjoint_r_t,
    ) -> PK_ERROR_code_t;

    /// Scale body by factor with transform.
    pub fn PK_BODY_enlarge(
        body: PK_BODY_t,
        factor: PK_scale_factor_t,
        transf: PK_TRANSF_t,
        tolerance: c_double,
        options: *const PK_BODY_enlarge_o_t,
        returns: *mut PK_BODY_enlarge_r_t,
    ) -> PK_ERROR_code_t;

    /// Slice body with sheet tool.
    pub fn PK_BODY_slice(
        body: PK_BODY_t,
        tool: PK_BODY_t,
        options: *const PK_BODY_slice_o_t,
        results: *mut PK_BODY_slice_r_t,
    ) -> PK_ERROR_code_t;

    /// Create patterned lattice-like mesh body from facet body.
    pub fn PK_BODY_make_patterned(
        body: PK_BODY_t,
        tolerance: c_double,
        options: *const PK_BODY_make_patterned_o_t,
        results: *mut PK_BODY_make_patterned_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Face operations
    // =========================================================================

    /// Delete faces and repair holes.
    pub fn PK_FACE_delete(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        heal_action: PK_FACE_heal_t,
        heal_loops: PK_FACE_heal_loops_t,
        local_check: PK_LOGICAL_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
        check_results: *mut *mut PK_local_check_t,
    ) -> PK_ERROR_code_t;

    /// Create neutral sheet from two faces.
    pub fn PK_FACE_make_neutral_sheet(
        faces: *const PK_FACE_t,
        placement: c_double,
        neutral_sheet: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Repair invalid face.
    pub fn PK_FACE_repair(
        face: PK_FACE_t,
        options: *const PK_FACE_repair_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Create valid face topology from faces.
    pub fn PK_FACE_make_valid_faces(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        options: *const PK_FACE_make_valid_faces_o_t,
        results: *mut PK_FACE_make_valid_faces_r_t,
    ) -> PK_ERROR_code_t;

    /// Create sheet body from faces.
    pub fn PK_FACE_make_sheet_body(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Fix mesh defects on facet faces.
    pub fn PK_FACE_fix_mesh_defects(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        options: *const PK_FACE_fix_mesh_defects_o_t,
        tracking: *mut PK_ENTITY_track_r_t,
        results: *mut PK_FACE_fix_mesh_defects_r_t,
    ) -> PK_ERROR_code_t;

    /// Replace face surfaces with isocline surfaces.
    pub fn PK_FACE_install_surfs_isocline(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        references: *const PK_ENTITY_t,
        direction: PK_VECTOR1_t,
        angle: c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Trim neutral sheets by face-set pairs.
    pub fn PK_BODY_trim_neutral_sheets(
        body: PK_BODY_t,
        n_pairs: c_int,
        pairs: *const PK_FACE_set_pair_t,
        tol: c_double,
        neutral_sheets: *mut PK_BODY_t,
        errors: *mut PK_neutral_error_t,
        causes: *mut PK_FACE_neutral_causes_array_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Region operations
    // =========================================================================

    /// Imprint curve onto region.
    pub fn PK_REGION_imprint_curve(
        region: PK_REGION_t,
        curve: PK_CURVE_t,
        bounds: PK_INTERVAL_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
        n_new_faces: *mut c_int,
        new_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Embed body into region (cellular topology).
    pub fn PK_REGION_embed_body(
        region: PK_REGION_t,
        body: PK_BODY_t,
        options: *const PK_REGION_embed_body_o_t,
        results: *mut PK_REGION_embed_body_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_BODY_create_implicit`.
    pub fn PK_BODY_create_implicit_r_f(results: *mut PK_BODY_create_implicit_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_BODY_is_cellular`.
    pub fn PK_BODY_is_cellular_r_f(results: *mut PK_BODY_is_cellular_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_BODY_is_disjoint`.
    pub fn PK_BODY_is_disjoint_r_f(results: *mut PK_BODY_is_disjoint_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_BODY_enlarge`.
    pub fn PK_BODY_enlarge_r_f(results: *mut PK_BODY_enlarge_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_BODY_slice`.
    pub fn PK_BODY_slice_r_f(results: *mut PK_BODY_slice_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_BODY_make_patterned`.
    pub fn PK_BODY_make_patterned_r_f(results: *mut PK_BODY_make_patterned_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_FACE_make_valid_faces`.
    pub fn PK_FACE_make_valid_faces_r_f(results: *mut PK_FACE_make_valid_faces_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_FACE_fix_mesh_defects`.
    pub fn PK_FACE_fix_mesh_defects_r_f(results: *mut PK_FACE_fix_mesh_defects_r_t) -> PK_ERROR_code_t;

    /// Free entity tracking results.
    pub fn PK_ENTITY_copy_r_f(results: *mut PK_ENTITY_track_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_REGION_embed_body`.
    pub fn PK_REGION_embed_body_r_f(results: *mut PK_REGION_embed_body_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_BODY_thicken_2`.
    pub fn PK_BODY_thicken_r_f(results: *mut PK_BODY_thicken_r_t) -> PK_ERROR_code_t;

}
