#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! Blending bindings for the Parasolid PK_* C API.
//!
//! Covers:
//! - Edge blending: set/fix/ask/remove (chapters 72-74)
//! - Face-face blending (chapter 75)
//! - Three-face blending (chapter 76)
//! - Blend error/fault codes (chapters 77-78)

use std::os::raw::{c_double, c_int};

use crate::*;

// =============================================================================
// Cross-section shape — PK_blend_xs_shape_t
// =============================================================================

pub type PK_blend_xs_shape_t = c_int;

/// Conic cross-section (tangent-continuous, default).
pub const PK_blend_xs_shape_conic_c: PK_blend_xs_shape_t = 0;
/// Curvature-continuous (G2) cross-section.
pub const PK_blend_xs_shape_g2_c: PK_blend_xs_shape_t = 1;
/// Linear/chamfer cross-section (only via PK_EDGE_set_blend_chain).
pub const PK_blend_xs_shape_chamfer_c: PK_blend_xs_shape_t = 2;
/// Legacy: implied from other fields (not recommended).
pub const PK_blend_xs_shape_unset_c: PK_blend_xs_shape_t = 3;

// =============================================================================
// Cross-section plane type — PK_blend_xs_t
// =============================================================================

pub type PK_blend_xs_t = c_int;

/// Rolling-ball: cross-section orthogonal to walls (default).
pub const PK_blend_xs_rolling_ball_c: PK_blend_xs_t = 0;
/// Disc: cross-section orthogonal to parameter spine.
pub const PK_blend_xs_disc_c: PK_blend_xs_t = 1;
/// Isoparameter: iso-parametric curves in left wall.
pub const PK_blend_xs_isoparameter_c: PK_blend_xs_t = 2;

// =============================================================================
// Rho interpretation — PK_blend_rho_t
// =============================================================================

pub type PK_blend_rho_t = c_int;

/// Rho independent of angle subtended by blend (default).
pub const PK_blend_rho_absolute_c: PK_blend_rho_t = 0;
/// Rho relative to angle subtended by blend.
pub const PK_blend_rho_relative_c: PK_blend_rho_t = 1;
/// Rho is radius of curvature at centre of cross-section.
pub const PK_blend_rho_centre_c: PK_blend_rho_t = 2;
/// Alias: US spelling.
pub const PK_blend_rho_center_c: PK_blend_rho_t = PK_blend_rho_centre_c;

// =============================================================================
// Size type — PK_blend_size_t
// =============================================================================

pub type PK_blend_size_t = c_int;

/// Size = distance of face offset (default).
pub const PK_blend_size_face_offset_c: PK_blend_size_t = 0;
/// Size = range to apex of blend.
pub const PK_blend_size_apex_range_c: PK_blend_size_t = 1;
/// Size = angle between chord and tangent plane.
pub const PK_blend_size_angle_c: PK_blend_size_t = 2;

// =============================================================================
// Range type — PK_blend_range_t
// =============================================================================

pub type PK_blend_range_t = c_int;

/// Default range type (face offset).
pub const PK_blend_range_face_offset_c: PK_blend_range_t = 0;

// =============================================================================
// Overflow: smooth — PK_blend_ov_smooth_t
// =============================================================================

pub type PK_blend_ov_smooth_t = c_int;

/// Prevent smooth overflow.
pub const PK_blend_ov_smooth_no_c: PK_blend_ov_smooth_t = 0;
/// Allow smooth overflow at any convexity.
pub const PK_blend_ov_smooth_any_c: PK_blend_ov_smooth_t = 1;
/// Allow smooth overflow only at different convexity (default).
pub const PK_blend_ov_smooth_diff_c: PK_blend_ov_smooth_t = 2;

// =============================================================================
// Overflow: cliff — PK_blend_ov_cliff_t
// =============================================================================

pub type PK_blend_ov_cliff_t = c_int;

/// Prevent cliff overflow.
pub const PK_blend_ov_cliff_no_c: PK_blend_ov_cliff_t = 0;
/// Allow cliff overflow at any convexity.
pub const PK_blend_ov_cliff_any_c: PK_blend_ov_cliff_t = 1;
/// Allow cliff overflow only at different convexity (default).
pub const PK_blend_ov_cliff_diff_c: PK_blend_ov_cliff_t = 2;

// =============================================================================
// Overflow: cliff end — PK_blend_ov_cliff_end_t
// =============================================================================

pub type PK_blend_ov_cliff_end_t = c_int;

/// Do not allow cliff end overflow (default).
pub const PK_blend_ov_cliff_end_no_c: PK_blend_ov_cliff_end_t = 0;
/// Allow cliff end overflow.
pub const PK_blend_ov_cliff_end_yes_c: PK_blend_ov_cliff_end_t = 1;

// =============================================================================
// Overflow: notch — PK_blend_ov_notch_t
// =============================================================================

pub type PK_blend_ov_notch_t = c_int;

/// Prevent notch overflow.
pub const PK_blend_ov_notch_no_c: PK_blend_ov_notch_t = 0;
/// Allow notch overflow (default).
pub const PK_blend_ov_notch_yes_c: PK_blend_ov_notch_t = 1;

// =============================================================================
// Overflow: explicit cliff — PK_blend_ov_exp_cliff_t
// =============================================================================

pub type PK_blend_ov_exp_cliff_t = c_int;

/// Create cliff edge along specified edge.
pub const PK_blend_ov_exp_cliff_yes_c: PK_blend_ov_exp_cliff_t = 0;
/// Do not create cliff edge along specified edge.
pub const PK_blend_ov_exp_cliff_no_c: PK_blend_ov_exp_cliff_t = 1;

// =============================================================================
// Setback collar — PK_blend_setback_collar_t
// =============================================================================

pub type PK_blend_setback_collar_t = c_int;

/// Every blended edge includes collar face (default, not for G2).
pub const PK_blend_setback_collar_all_c: PK_blend_setback_collar_t = 0;
/// No collar faces.
pub const PK_blend_setback_collar_none_c: PK_blend_setback_collar_t = 1;

// =============================================================================
// Blend ordering — PK_blend_order_t
// =============================================================================

pub type PK_blend_order_t = c_int;

/// Parasolid decides order.
pub const PK_blend_order_unset_c: PK_blend_order_t = 0;
/// Blend concave edges first.
pub const PK_blend_order_concave_convex_c: PK_blend_order_t = 1;
/// Blend convex edges first.
pub const PK_blend_order_convex_concave_c: PK_blend_order_t = 2;
/// Blend minority convexity edges first.
pub const PK_blend_order_min_convexity_c: PK_blend_order_t = 3;
/// Blend majority convexity edges first.
pub const PK_blend_order_maj_convexity_c: PK_blend_order_t = 4;

// =============================================================================
// Transfer — PK_blend_transfer_t
// =============================================================================

pub type PK_blend_transfer_t = c_int;

/// Delete overlapped topology (default).
pub const PK_blend_transfer_TOPOL_no_c: PK_blend_transfer_t = 0;
/// Preserve overlapped topology.
pub const PK_blend_transfer_TOPOL_yes_c: PK_blend_transfer_t = 1;

// =============================================================================
// Propagate — PK_blend_propagate_t
// =============================================================================

pub type PK_blend_propagate_t = c_int;

/// Do not propagate blend (default).
pub const PK_blend_propagate_no_c: PK_blend_propagate_t = 0;
/// Propagate blend across smooth edges.
pub const PK_blend_propagate_yes_c: PK_blend_propagate_t = 1;

// =============================================================================
// Set tolerance — PK_blend_set_tol_t
// =============================================================================

pub type PK_blend_set_tol_t = c_int;

/// Apply tolerance if blend would otherwise fail (default).
pub const PK_blend_set_tol_yes_c: PK_blend_set_tol_t = 0;
/// Do not apply tolerance.
pub const PK_blend_set_tol_no_c: PK_blend_set_tol_t = 1;

// =============================================================================
// Tolerance improvement — PK_blend_tolerance_t
// =============================================================================

pub type PK_blend_tolerance_t = c_int;

/// Do not reduce tolerance of new edges (default).
pub const PK_blend_tolerance_standard_c: PK_blend_tolerance_t = 0;
/// Reduce tolerance of new edges if possible.
pub const PK_blend_tolerance_improved_c: PK_blend_tolerance_t = 1;

// =============================================================================
// Face self-intersection repair — PK_blend_repair_fa_X_t
// =============================================================================

pub type PK_blend_repair_fa_X_t = c_int;

/// Do not repair self-intersecting faces (default).
pub const PK_blend_repair_fa_X_no_c: PK_blend_repair_fa_X_t = 0;
/// Attempt repair.
pub const PK_blend_repair_fa_X_yes_c: PK_blend_repair_fa_X_t = 1;

// =============================================================================
// Surface self-intersection repair — PK_blend_repair_su_X_t
// =============================================================================

pub type PK_blend_repair_su_X_t = c_int;

/// Do not repair (default).
pub const PK_blend_repair_su_X_no_c: PK_blend_repair_su_X_t = 0;
/// Repair self-intersecting surfaces.
pub const PK_blend_repair_su_X_yes_c: PK_blend_repair_su_X_t = 1;
/// Repair and report.
pub const PK_blend_repair_su_X_report_c: PK_blend_repair_su_X_t = 2;

// =============================================================================
// Report — PK_blend_report_t
// =============================================================================

pub type PK_blend_report_t = c_int;

/// Do not report repaired faces (default).
pub const PK_blend_report_repaired_no_c: PK_blend_report_t = 0;
/// Save info about repaired surfaces to Report.
pub const PK_blend_report_repaired_yes_c: PK_blend_report_t = 1;

// =============================================================================
// Inside tight — PK_blend_inside_tight_t
// =============================================================================

pub type PK_blend_inside_tight_t = c_int;

/// No tight blending.
pub const PK_blend_inside_tight_no_c: PK_blend_inside_tight_t = 0;
/// Blend across blend-like faces only (default for PK_BODY_fix_blends).
pub const PK_blend_inside_tight_blends_c: PK_blend_inside_tight_t = 1;
/// Blend across full tight faces.
pub const PK_blend_inside_tight_faces_c: PK_blend_inside_tight_t = 2;
/// Blend across partially tight regions.
pub const PK_blend_inside_tight_partial_c: PK_blend_inside_tight_t = 3;

// =============================================================================
// Limit type — PK_blend_limit_type_t
// =============================================================================

pub type PK_blend_limit_type_t = c_int;

/// Edge limit.
pub const PK_blend_limit_type_edge_c: PK_blend_limit_type_t = 0;
/// Overlap limit.
pub const PK_blend_limit_type_overlap_c: PK_blend_limit_type_t = 1;

// =============================================================================
// Limit patch — PK_blend_limit_patch_t
// =============================================================================

pub type PK_blend_limit_patch_t = c_int;

/// Do not patch.
pub const PK_blend_limit_patch_no_c: PK_blend_limit_patch_t = 0;
/// Attempt to patch blend in region of limit.
pub const PK_blend_limit_patch_yes_c: PK_blend_limit_patch_t = 1;

// =============================================================================
// Laminar trim — PK_blend_laminar_trim_t
// =============================================================================

pub type PK_blend_laminar_trim_t = c_int;

/// Trim to edges of underlying faces (default).
pub const PK_blend_laminar_trim_edges_c: PK_blend_laminar_trim_t = 0;
/// Trim to widest face only.
pub const PK_blend_laminar_trim_bound_c: PK_blend_laminar_trim_t = 1;

// =============================================================================
// Output sheet (preview) — PK_blend_output_sheet_t
// =============================================================================

pub type PK_blend_output_sheet_t = c_int;

/// Fix blends normally (default).
pub const PK_blend_output_sheet_no_c: PK_blend_output_sheet_t = 0;
/// Create preview sheet body.
pub const PK_blend_output_sheet_yes_c: PK_blend_output_sheet_t = 1;
/// Create preview sheet only if blend would fail.
pub const PK_blend_output_sheet_on_fail_c: PK_blend_output_sheet_t = 2;

// =============================================================================
// Suggest limit — PK_blend_suggest_limit_t
// =============================================================================

pub type PK_blend_suggest_limit_t = c_int;

/// Do not generate limit data (default).
pub const PK_blend_suggest_limit_no_c: PK_blend_suggest_limit_t = 0;
/// Generate limit data (not for patching).
pub const PK_blend_suggest_limit_yes_c: PK_blend_suggest_limit_t = 1;
/// Generate limit data suitable for patching.
pub const PK_blend_suggest_limit_patch_c: PK_blend_suggest_limit_t = 2;

// =============================================================================
// Report extended — PK_blend_report_extended_t
// =============================================================================

pub type PK_blend_report_extended_t = c_int;

/// Do not report extended chamfers (default).
pub const PK_blend_report_extended_no_c: PK_blend_report_extended_t = 0;
/// Report extended chamfers.
pub const PK_blend_report_extended_yes_c: PK_blend_report_extended_t = 1;

// =============================================================================
// Edge update — PK_blend_edge_update_t
// =============================================================================

pub type PK_blend_edge_update_t = c_int;

/// Use all blending enhancements (default).
pub const PK_blend_edge_update_default_c: PK_blend_edge_update_t = 0;

// =============================================================================
// Tracking type — PK_blend_track_type_t
// =============================================================================

pub type PK_blend_track_type_t = c_int;

/// Track underlying faces including removed ones.
pub const PK_blend_track_type_unders_c: PK_blend_track_type_t = 0;

// =============================================================================
// Trim — PK_blend_trim_t
// =============================================================================

pub type PK_blend_trim_t = c_int;

/// Trim to walls (default).
pub const PK_blend_trim_to_walls_c: PK_blend_trim_t = 0;
/// Do not trim.
pub const PK_blend_trim_no_c: PK_blend_trim_t = 1;
/// Long trim (as long as possible).
pub const PK_blend_trim_long_c: PK_blend_trim_t = 2;
/// Short trim (as short as possible).
pub const PK_blend_trim_short_c: PK_blend_trim_t = 3;

// =============================================================================
// Trim extent (three-face) — PK_blend_trim_extent_t
// =============================================================================

pub type PK_blend_trim_extent_t = c_int;

/// Include centre wall when trimming (default).
pub const PK_blend_trim_extent_all_c: PK_blend_trim_extent_t = 0;
/// Ignore centre wall when trimming.
pub const PK_blend_trim_extent_sides_c: PK_blend_trim_extent_t = 1;

// =============================================================================
// Walls — PK_blend_walls_t
// =============================================================================

pub type PK_blend_walls_t = c_int;

/// Trim and attach (default).
pub const PK_blend_walls_attach_c: PK_blend_walls_t = 0;
/// No wall trimming, blend as sheet.
pub const PK_blend_walls_trim_no_c: PK_blend_walls_t = 1;
/// Trim both walls, blend as sheet (sheets only).
pub const PK_blend_walls_trim_both_c: PK_blend_walls_t = 2;
/// Trim, attach, create solid if closed.
pub const PK_blend_walls_solid_c: PK_blend_walls_t = 3;
/// Trim solid variant for 3-face.
pub const PK_blend_walls_trim_solid_c: PK_blend_walls_t = 4;
/// Preview sheet without modification.
pub const PK_blend_walls_preview_c: PK_blend_walls_t = 5;

// =============================================================================
// Orientation (three-face) — PK_blend_orientation_t
// =============================================================================

pub type PK_blend_orientation_t = c_int;

/// Let Parasolid determine (default).
pub const PK_blend_orientation_unknown_c: PK_blend_orientation_t = 0;
/// Blend in front of wall normal.
pub const PK_blend_orientation_before_c: PK_blend_orientation_t = 1;
/// Blend behind wall normal.
pub const PK_blend_orientation_behind_c: PK_blend_orientation_t = 2;

// =============================================================================
// Run out — PK_blend_run_out_t
// =============================================================================

pub type PK_blend_run_out_t = c_int;

/// Do not stop in shallow regions (default).
pub const PK_blend_run_out_no_c: PK_blend_run_out_t = 0;
/// Stop at specified angle.
pub const PK_blend_run_out_angle_c: PK_blend_run_out_t = 1;

// =============================================================================
// Master faces — PK_blend_use_master_faces_t
// =============================================================================

pub type PK_blend_use_master_faces_t = c_int;

/// Require master face (default).
pub const PK_blend_use_master_faces_yes_c: PK_blend_use_master_faces_t = 0;
/// No master face requirement.
pub const PK_blend_use_master_faces_no_c: PK_blend_use_master_faces_t = 1;

// =============================================================================
// Output rib — PK_blend_output_rib_t
// =============================================================================

pub type PK_blend_output_rib_t = c_int;

/// No ribs (default).
pub const PK_blend_output_rib_no_c: PK_blend_output_rib_t = 0;
/// Ribs where surface fails.
pub const PK_blend_output_rib_on_fail_c: PK_blend_output_rib_t = 1;
/// Ribs only, no surfaces.
pub const PK_blend_output_rib_only_c: PK_blend_output_rib_t = 2;
/// Ribs and surfaces.
pub const PK_blend_output_rib_also_c: PK_blend_output_rib_t = 3;
/// Single rib at help point.
pub const PK_blend_output_rib_at_help_c: PK_blend_output_rib_t = 4;

// =============================================================================
// Group rib — PK_blend_group_rib_t
// =============================================================================

pub type PK_blend_group_rib_t = c_int;

/// No grouping (default).
pub const PK_blend_group_rib_no_c: PK_blend_group_rib_t = 0;
/// Group by blend face.
pub const PK_blend_group_rib_by_face_c: PK_blend_group_rib_t = 1;
/// Group by parameter intervals.
pub const PK_blend_group_rib_by_parms_c: PK_blend_group_rib_t = 2;

// =============================================================================
// Check surface self-intersection — PK_blend_check_su_X_t
// =============================================================================

pub type PK_blend_check_su_X_t = c_int;

/// No surface self-intersection check (default).
pub const PK_blend_check_su_X_no_c: PK_blend_check_su_X_t = 0;
/// Check all except B-surfaces.
pub const PK_blend_check_su_X_not_bsurf_c: PK_blend_check_su_X_t = 1;

// =============================================================================
// Check face — PK_blend_check_fa_t
// =============================================================================

pub type PK_blend_check_fa_t = c_int;

/// No face check (default).
pub const PK_blend_check_fa_no_c: PK_blend_check_fa_t = 0;
/// Check faces.
pub const PK_blend_check_fa_yes_c: PK_blend_check_fa_t = 1;

// =============================================================================
// Check face-face — PK_blend_check_fa_fa_t
// =============================================================================

pub type PK_blend_check_fa_fa_t = c_int;

/// No face-face check (default).
pub const PK_blend_check_fa_fa_no_c: PK_blend_check_fa_fa_t = 0;
/// Check face-face consistency.
pub const PK_blend_check_fa_fa_yes_c: PK_blend_check_fa_fa_t = 1;

// =============================================================================
// Track edges — PK_blend_track_edges_t
// =============================================================================

pub type PK_blend_track_edges_t = c_int;

/// No edge tracking (default).
pub const PK_blend_track_edges_no_c: PK_blend_track_edges_t = 0;
/// Track laminar edges.
pub const PK_blend_track_edges_laminar_c: PK_blend_track_edges_t = 1;

// =============================================================================
// Update — PK_blend_update_t
// =============================================================================

pub type PK_blend_update_t = c_int;

/// Use all enhancements (default).
pub const PK_blend_update_default_c: PK_blend_update_t = 0;

// =============================================================================
// Imprint complete — PK_imprint_complete_t
// =============================================================================

// =============================================================================
// Extension shape — PK_extension_shape_t
// =============================================================================

pub type PK_extension_shape_t = c_int;

/// Linear extension (default).
pub const PK_extension_shape_linear_c: PK_extension_shape_t = 0;
/// Curvature-continuous extension.
pub const PK_extension_shape_soft_c: PK_extension_shape_t = 1;

// =============================================================================
// Prevent sharp — PK_blend_prevent_sharp_t
// =============================================================================

pub type PK_blend_prevent_sharp_t = c_int;

/// Do not prevent sharp edges (default).
pub const PK_blend_prevent_sharp_no_c: PK_blend_prevent_sharp_t = 0;
/// Prevent creation of sharp edges.
pub const PK_blend_prevent_sharp_yes_c: PK_blend_prevent_sharp_t = 1;

// =============================================================================
// Edge blend fault codes — PK_blend_fault_t
// (PK_blend_fault_t type alias lives in error.rs)
// =============================================================================

// Severe errors
/// Vertex configuration too complex (4+ adjacent edges, at least 2 blended).
pub const PK_blend_fault_vertex_c: PK_blend_fault_t = 1;
/// Unspecified numerical problem.
pub const PK_blend_fault_unknown_c: PK_blend_fault_t = 2;

// General configuration errors
/// Blend requires invalid B-surface extension.
pub const PK_blend_fault_bsurf_c: PK_blend_fault_t = 3;
/// Range inconsistent with adjacent blended edge.
pub const PK_blend_fault_range_c: PK_blend_fault_t = 4;
/// Adjoining edge not blended (illegal 2-of-3 vertex config).
pub const PK_blend_fault_edge_c: PK_blend_fault_t = 5;
/// Blend completely overlaps edge loop.
pub const PK_blend_fault_loop_c: PK_blend_fault_t = 6;
/// Unblended edge overlapped by blend.
pub const PK_blend_fault_overlap_edge_c: PK_blend_fault_t = 7;
/// Range of blend on face too large (or radius of curvature too small).
pub const PK_blend_fault_face_c: PK_blend_fault_t = 8;
/// Supplied rho value too large (cross section too flat).
pub const PK_blend_fault_rho_too_large_c: PK_blend_fault_t = 9;
/// Illegal blend on another edge prevented full check.
pub const PK_blend_fault_other_edge_c: PK_blend_fault_t = 10;
/// Range on chamfer blend too large (chord misses opposite surface).
pub const PK_blend_fault_apex_range_c: PK_blend_fault_t = 11;

// Overlapping blend errors
/// Overlapping blends failure.
pub const PK_blend_fault_overlap_c: PK_blend_fault_t = 12;
/// Overlapping blends at end failure.
pub const PK_blend_fault_overlap_end_c: PK_blend_fault_t = 13;
/// Blend end failure.
pub const PK_blend_fault_end_c: PK_blend_fault_t = 14;
/// End boundary intersects unblended edge.
pub const PK_blend_fault_edge_intsec_c: PK_blend_fault_t = 15;

// Post-fix errors
/// Blend created face-face inconsistency (only with explicit check).
pub const PK_blend_fault_face_face_c: PK_blend_fault_t = 16;
/// Blend surface is self-intersecting (only with explicit check).
pub const PK_blend_fault_self_int_c: PK_blend_fault_t = 17;

// =============================================================================
// Face-face blend fault codes — PK_fxf_fault_t
// =============================================================================

pub type PK_fxf_fault_t = c_int;

// Success / partial success
/// Blend succeeded.
pub const PK_fxf_fault_no_fault_c: PK_fxf_fault_t = 0;
/// Blend not attached, sheet bodies created (partial success).
pub const PK_fxf_fault_sheet_c: PK_fxf_fault_t = 1;

// General failures
/// Blend could not be created.
pub const PK_fxf_fault_unknown_c: PK_fxf_fault_t = 2;
/// Insufficient data to define blend.
pub const PK_fxf_fault_insufficient_c: PK_fxf_fault_t = 3;
/// Inconsistent data supplied.
pub const PK_fxf_fault_inconsistent_c: PK_fxf_fault_t = 4;

// Input validation errors
/// Invalid wall of faces.
pub const PK_fxf_fault_wall_c: PK_fxf_fault_t = 5;
/// Invalid range definition.
pub const PK_fxf_fault_range_c: PK_fxf_fault_t = 6;
/// Invalid tangent holdline data.
pub const PK_fxf_fault_thl_c: PK_fxf_fault_t = 7;
/// Invalid cliff-edge data.
pub const PK_fxf_fault_cliff_c: PK_fxf_fault_t = 8;
/// Invalid conic holdline data.
pub const PK_fxf_fault_chl_c: PK_fxf_fault_t = 9;
/// Invalid rho values in rho law function.
pub const PK_fxf_fault_rho_value_c: PK_fxf_fault_t = 10;
/// Asymmetric ranges inconsistent with geometry.
pub const PK_fxf_fault_asymmetric_c: PK_fxf_fault_t = 11;
/// Invalid parameter spine.
pub const PK_fxf_fault_bad_spine_c: PK_fxf_fault_t = 12;
/// Invalid rib controls.
pub const PK_fxf_fault_bad_ribs_c: PK_fxf_fault_t = 13;
/// Limit plane origin not unique.
pub const PK_fxf_fault_plane_origin_c: PK_fxf_fault_t = 14;
/// Limit plane insufficient.
pub const PK_fxf_fault_plane_insuft_c: PK_fxf_fault_t = 15;
/// No preview rib could be constructed.
pub const PK_fxf_fault_preview_rib_c: PK_fxf_fault_t = 16;

// Geometric errors
/// Face in wall too tightly curved.
pub const PK_fxf_fault_curved_c: PK_fxf_fault_t = 17;
/// Blend range too small.
pub const PK_fxf_fault_small_c: PK_fxf_fault_t = 18;
/// Blend range too large.
pub const PK_fxf_fault_large_c: PK_fxf_fault_t = 19;
/// Rho too large, cross section too flat vs underlying surface.
pub const PK_fxf_fault_rho_too_large_c: PK_fxf_fault_t = 20;

// Sense errors
/// left_sense is incorrect.
pub const PK_fxf_fault_left_c: PK_fxf_fault_t = 21;
/// right_sense is incorrect.
pub const PK_fxf_fault_right_c: PK_fxf_fault_t = 22;
/// Both left_sense and right_sense incorrect.
pub const PK_fxf_fault_both_c: PK_fxf_fault_t = 23;

// Post-attachment errors
/// Blend sheets intersect each other.
pub const PK_fxf_fault_sheet_clash_c: PK_fxf_fault_t = 24;
/// Attached blend combined bodies causing face-face inconsistency.
pub const PK_fxf_fault_wall_clash_c: PK_fxf_fault_t = 25;
/// Attached blend caused face-face inconsistency.
pub const PK_fxf_fault_face_face_c: PK_fxf_fault_t = 26;
/// Blend face(s) with self-intersecting geometry.
pub const PK_fxf_fault_self_int_c: PK_fxf_fault_t = 27;

// =============================================================================
// Three-face blend status — PK_3_face_blend_status_t
// =============================================================================

pub type PK_3_face_blend_status_t = c_int;

/// Success.
pub const PK_3_face_blend_ok_c: PK_3_face_blend_status_t = 0;
/// Partial success, returned as sheet body.
pub const PK_3_face_blend_sheet_c: PK_3_face_blend_status_t = 1;

// =============================================================================
// Report record type constants
// =============================================================================

pub const PK_REPORT_record_type_1_c: PK_REPORT_record_type_t = 1;

// Report status values — type 1
pub type PK_REPORT_1_t = c_int;

/// Faces modified to repair self-intersections.
pub const PK_REPORT_1_rep_sx_faces_c: PK_REPORT_1_t = 0;
/// G2 blend faces modified to maintain continuity.
pub const PK_REPORT_1_rep_G2_faces_c: PK_REPORT_1_t = 1;
/// Self-intersecting blend faces.
pub const PK_REPORT_1_blend_faces_sx_c: PK_REPORT_1_t = 2;

// Report status values — type 3
pub type PK_REPORT_3_t = c_int;

/// Report contains limit data.
pub const PK_REPORT_3_limit_data_c: PK_REPORT_3_t = 0;
/// Report contains limit topology data.
pub const PK_REPORT_3_limit_topol_c: PK_REPORT_3_t = 1;
/// Chamfer face extended outside original range.
pub const PK_REPORT_3_chamfer_extended_c: PK_REPORT_3_t = 2;
/// Geometry that could not be extended or had G2 discontinuity.
pub const PK_REPORT_3_blend_x_g1_c: PK_REPORT_3_t = 3;

/// Geometry with soft extension used.
pub type PK_REPORT_geom_t = c_int;
pub const PK_REPORT_geom_extended_c: PK_REPORT_geom_t = 0;

// =============================================================================
// Option structs
// =============================================================================

/// Blend properties sub-structure (used by set_blend_constant, set_blend_chamfer).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_properties_t {
    pub ov_smooth: PK_blend_ov_smooth_t,
    pub ov_cliff: PK_blend_ov_cliff_t,
    pub ov_cliff_end: PK_blend_ov_cliff_end_t,
    pub ov_notch: PK_blend_ov_notch_t,
    pub tolerance: c_double,
    pub render_ribs: PK_LOGICAL_t,
    pub rib_space: c_double,
    pub draw_fix: PK_LOGICAL_t,
}

/// Options for PK_EDGE_set_blend_constant.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_set_blend_constant_o_t {
    pub o_t_version: c_int,
    pub properties: PK_blend_properties_t,
    pub cliff_edge: PK_EDGE_t,
    pub propagate: PK_blend_propagate_t,
    pub xs_shape: PK_blend_xs_shape_t,
}

impl Default for PK_EDGE_set_blend_constant_o_t {
    fn default() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.o_t_version = 1;
        s
    }
}

/// Options for PK_EDGE_set_blend_chain.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_set_blend_chain_o_t {
    pub o_t_version: c_int,
    pub n_positions: c_int,
    pub positions: *const PK_VECTOR_t,
    pub primary_sizes: *const c_double,
    pub secondary_sizes: *const c_double,
    pub primary_fins: *const PK_FIN_t,
    pub rhos: *const c_double,
    pub rho_type: PK_blend_rho_t,
    pub n_mitre_fins: c_int,
    pub mitre_fins: *const PK_FIN_t,
    pub mitre_fins_indices: *const c_int,
    pub n_clamp_indices: c_int,
    pub clamp_indices: *const c_int,
    pub xs_shape: PK_blend_xs_shape_t,
    pub primary_size_type: PK_blend_size_t,
    pub secondary_size_type: PK_blend_size_t,
    pub tolerance: c_double,
    pub ov_smooth: PK_blend_ov_smooth_t,
    pub ov_cliff: PK_blend_ov_cliff_t,
    pub ov_cliff_end: PK_blend_ov_cliff_end_t,
    pub ov_notch: PK_blend_ov_notch_t,
}

impl Default for PK_EDGE_set_blend_chain_o_t {
    fn default() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.o_t_version = 1;
        s
    }
}

/// Cap data for trimming blends.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_cap_data_t {
    pub n_caps: c_int,
    pub caps: *const PK_ENTITY_t,
    pub reverse_cap: *const PK_LOGICAL_t,
}

/// Limit data for stopping blends short.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_limit_data_t {
    pub n_limits: c_int,
    pub limit_points: *const PK_VECTOR_t,
    pub limit_directions: *const PK_VECTOR_t,
    pub edges: *const PK_EDGE_t,
    pub limit_types: *const PK_blend_limit_type_t,
    pub vertices: *const PK_VERTEX_t,
}

/// Options for PK_BODY_fix_blends.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_fix_blends_o_t {
    pub o_t_version: c_int,
    pub n_explicit_cliff_edges: c_int,
    pub explicit_cliff_edges: *const PK_EDGE_t,
    pub explicit_cliff_edges_type: *const PK_blend_ov_exp_cliff_t,
    pub y_blend_data: *const c_int, // opaque, set null if unused
    pub setback_data: *const c_int, // opaque, set null if unused
    pub setback_shape_data: *const c_int, // opaque, set null if unused
    pub vx_twin: PK_TOPOL_t,
    pub local_check: PK_LOGICAL_t,
    pub checks: PK_LOGICAL_t,
    pub transfer: PK_blend_transfer_t,
    pub preserve_notch: PK_LOGICAL_t,
    pub vx_blend_data: *const c_int, // opaque, set null if unused
    pub vx_order_data: *const c_int, // opaque, set null if unused
    pub propagate: PK_blend_propagate_t,
    pub tolerance: c_double,
    pub set_tol: PK_blend_set_tol_t,
    pub improve_tolerance: PK_blend_tolerance_t,
    pub repair_su_X: PK_blend_repair_su_X_t,
    pub repair_fa_X: PK_blend_repair_fa_X_t,
    pub report: PK_blend_report_t,
    pub inside_tight: PK_blend_inside_tight_t,
    pub limit_data: PK_blend_limit_data_t,
    pub cap_data: PK_blend_cap_data_t,
    pub n_limit_topols: c_int,
    pub limit_topols: *const PK_TOPOL_t,
    pub limit_topols_patch: *const PK_blend_limit_patch_t,
    pub limit_topols_unders: *const PK_TOPOL_t,
    pub laminar_trim: PK_blend_laminar_trim_t,
    pub output_sheet: PK_blend_output_sheet_t,
    pub partition: PK_PARTITION_t,
    pub suggest_limit: PK_blend_suggest_limit_t,
    pub ov_smooth: PK_blend_ov_smooth_t,
    pub ov_notch: PK_blend_ov_notch_t,
    pub ov_order: PK_blend_order_t,
    pub tracking_type: PK_blend_track_type_t,
    pub extension_shape: PK_extension_shape_t,
    pub report_extended: PK_blend_report_extended_t,
    pub update: PK_blend_edge_update_t,
}

impl Default for PK_BODY_fix_blends_o_t {
    fn default() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.o_t_version = 1;
        s
    }
}

// =============================================================================
// Face-face blend option sub-structures
// =============================================================================

/// Cross-section shape and size for face-face blending.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_shape_t {
    pub xsection: PK_blend_xs_t,
    pub parameter: PK_ENTITY_t,
    pub xs_shape: PK_blend_xs_shape_t,
    pub radius: c_double,
    pub range1_const: c_double,
    pub range2_const: c_double,
    pub range1: PK_ENTITY_t,
    pub range2: PK_ENTITY_t,
    pub range1_type: PK_blend_size_t,
    pub range2_type: PK_blend_size_t,
    pub width: c_double,
    pub ratio: c_double,
    pub var_width: PK_ENTITY_t,
    pub var_ratio: PK_ENTITY_t,
    pub rho: PK_ENTITY_t,
    pub rho_const: c_double,
    pub rho_type: PK_blend_rho_t,
    pub depth: PK_ENTITY_t,
    pub depth_const: c_double,
    pub skew_const: c_double,
    pub softness: c_double,
}

/// Constraint sub-structure (holdlines, cliff edges, limit planes).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_constraint_t {
    pub n_tangent_edges: c_int,
    pub tangent_edges: *const PK_EDGE_t,
    pub n_conic_edges: c_int,
    pub conic_edges: *const PK_EDGE_t,
    pub n_cliff_edges: c_int,
    pub cliff_edges: *const PK_EDGE_t,
    pub n_inv_tangent_edges: c_int,
    pub inv_tangent_edges: *const PK_EDGE_t,
    pub n_inv_conic_edges: c_int,
    pub inv_conic_edges: *const PK_EDGE_t,
    pub limit_1: PK_ENTITY_t,
    pub limit_2: PK_ENTITY_t,
    pub localise_limit_planes: PK_LOGICAL_t,
    pub n_limit_topols: c_int,
    pub limit_topols: *const PK_ENTITY_t,
    pub n_caps: c_int,
    pub caps: *const PK_ENTITY_t,
    pub reverse_cap: *const PK_LOGICAL_t,
}

/// Local checking options for face-face blending.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_local_check_t {
    pub check_su_X: PK_blend_check_su_X_t,
    pub check_fa: PK_blend_check_fa_t,
    pub check_fa_fa: PK_blend_check_fa_fa_t,
}

/// Rib generation control for face-face blending.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_rib_control_t {
    pub output_rib: PK_blend_output_rib_t,
    pub group_rib: PK_blend_group_rib_t,
    pub max_n_ribs: c_int,
    pub n_interval_parms: c_int,
    pub interval_parms: *const c_double,
}

/// Return structure for blend ribs.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_blend_rib_r_t {
    pub n_ribs: c_int,
    pub ribs: *mut PK_ENTITY_t,
    pub rib_parms: *mut c_double,
    pub rib_indices: *mut c_int,
}

/// Options for PK_FACE_make_blend.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_make_blend_o_t {
    pub o_t_version: c_int,
    pub shape: PK_blend_shape_t,
    pub constraints: PK_blend_constraint_t,
    pub checks: PK_blend_local_check_t,
    pub rib_control: PK_blend_rib_control_t,
    pub trim: PK_blend_trim_t,
    pub walls: PK_blend_walls_t,
    pub propagate: PK_blend_propagate_t,
    pub tolerance: c_double,
    pub have_propagation_angle: PK_LOGICAL_t,
    pub propagation_angle: c_double,
    pub notch: PK_LOGICAL_t,
    pub master_faces: PK_blend_use_master_faces_t,
    pub multiple: PK_LOGICAL_t,
    pub have_help_point: PK_LOGICAL_t,
    pub help_point: PK_VECTOR_t,
    pub local_check: PK_LOGICAL_t,
    pub repair_su_X: PK_blend_repair_su_X_t,
    pub repair_fa_X: PK_blend_repair_fa_X_t,
    pub inside_tight: PK_blend_inside_tight_t,
    pub prevent_sharp: PK_blend_prevent_sharp_t,
    pub track_edges: PK_blend_track_edges_t,
    pub report: PK_blend_report_t,
    pub update: PK_blend_update_t,
    pub user_surface: PK_ENTITY_t,
    pub partition: PK_PARTITION_t,
    pub imprint_complete: PK_imprint_complete_t,
    pub run_out: PK_blend_run_out_t,
    pub run_out_angle: c_double,
    pub extension_shape: PK_extension_shape_t,
}

impl Default for PK_FACE_make_blend_o_t {
    fn default() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.o_t_version = 1;
        s
    }
}

/// Face-face blend fault return structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_fxf_error_t {
    pub fault: PK_fxf_fault_t,
    pub n_topols: c_int,
    pub topols: *mut PK_TOPOL_t,
    pub n_points: c_int,
    pub points: *mut PK_VECTOR_t,
    pub n_dist: c_int,
    pub dist: *mut c_double,
}

// =============================================================================
// Three-face blend option structures
// =============================================================================

/// Options for PK_FACE_make_3_face_blend.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_make_3_face_blend_o_t {
    pub o_t_version: c_int,
    pub xsection: PK_blend_xs_t,
    pub blend_tolerance: c_double,
    pub propagate: PK_blend_propagate_t,
    pub propagation_tolerance: c_double,
    pub have_propagation_tolerance: PK_LOGICAL_t,
    pub trim: PK_blend_trim_t,
    pub trim_extent: PK_blend_trim_extent_t,
    pub walls: PK_blend_walls_t,
    pub partition: PK_PARTITION_t,
    pub n_limits: c_int,
    pub limits: *const PK_ENTITY_t,
    pub n_caps: c_int,
    pub caps: *const PK_ENTITY_t,
    pub reverse_cap: *const PK_LOGICAL_t,
    pub have_help_point: PK_LOGICAL_t,
    pub help_point: PK_VECTOR_t,
    pub check_fa_fa: PK_blend_check_fa_fa_t,
    pub repair_fa_X: PK_blend_repair_fa_X_t,
    pub track_edges: PK_blend_track_edges_t,
    pub update: PK_blend_update_t,
    pub left_orientation: PK_blend_orientation_t,
    pub centre_orientation: PK_blend_orientation_t,
    pub right_orientation: PK_blend_orientation_t,
}

impl Default for PK_FACE_make_3_face_blend_o_t {
    fn default() -> Self {
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.o_t_version = 1;
        s
    }
}

/// Result structure for PK_FACE_make_3_face_blend.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_make_3_face_blend_r_t {
    pub left_orientation: PK_blend_orientation_t,
    pub centre_orientation: PK_blend_orientation_t,
    pub right_orientation: PK_blend_orientation_t,
    pub status: PK_3_face_blend_status_t,
    pub n_sheets: c_int,
    pub sheets: *mut PK_BODY_t,
    pub n_entities: c_int,
    pub entities: *mut PK_ENTITY_t,
}

// =============================================================================
// PK_BLENDSF_ask return structure
// =============================================================================

/// Return structure for PK_BLENDSF_ask — describes a rolling-ball blend surface.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BLENDSF_sf_t {
    pub geom_1: PK_SURF_t,
    pub geom_2: PK_SURF_t,
    pub radii: [c_double; 2],
    pub spine: PK_CURVE_t,
    pub spine_ext: PK_INTERVAL_t,
}

// =============================================================================
// Extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // -------------------------------------------------------------------------
    // Edge blend: set
    // -------------------------------------------------------------------------

    /// Sets a constant rolling-ball blend on an edge.
    pub fn PK_EDGE_set_blend_constant(
        edge: PK_EDGE_t,
        radius: c_double,
        options: *const PK_EDGE_set_blend_constant_o_t,
    ) -> PK_ERROR_t;

    /// Sets a variable rolling-ball blend on a chain of edges.
    /// Also used for chamfer blends (with xs_shape = PK_blend_xs_shape_chamfer_c).
    pub fn PK_EDGE_set_blend_chain(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        options: *const PK_EDGE_set_blend_chain_o_t,
    ) -> PK_ERROR_t;

    /// Sets a chamfer (face offset) blend on an edge.
    pub fn PK_EDGE_set_blend_chamfer(
        edge: PK_EDGE_t,
        range_1: c_double,
        range_2: c_double,
        properties: *const PK_blend_properties_t,
    ) -> PK_ERROR_t;

    /// Sets a variable-radius blend on an edge (legacy).
    pub fn PK_EDGE_set_blend_variable(
        edge: PK_EDGE_t,
        n_radii: c_int,
        positions: *const PK_VECTOR_t,
        radii: *const c_double,
        options: *const PK_EDGE_set_blend_constant_o_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Edge blend: fix
    // -------------------------------------------------------------------------

    /// Fixes (incorporates) unfixed blends into a body.
    /// Converts blend attributes on edges into actual blend faces.
    pub fn PK_BODY_fix_blends(
        body: PK_BODY_t,
        options: *const PK_BODY_fix_blends_o_t,
        n_blends: *mut c_int,
        blends: *mut *mut PK_FACE_t,
        n_topols: *mut c_int,
        topols: *mut *mut PK_TOPOL_t,
        n_unders: *mut c_int,
        unders: *mut *mut PK_FACE_t,
        fault: *mut PK_blend_fault_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Edge blend: ask / remove
    // -------------------------------------------------------------------------

    /// Returns information about an unfixed blend on an edge.
    pub fn PK_EDGE_ask_blend(
        edge: PK_EDGE_t,
        blend_type: *mut PK_CLASS_t,
        left_face: *mut PK_FACE_t,
        right_face: *mut PK_FACE_t,
        properties: *mut PK_blend_properties_t,
        cliff_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_t;

    /// Removes an unfixed blend from an edge.
    /// If blend was set via PK_EDGE_set_blend_chain, removes from ALL edges in chain.
    pub fn PK_EDGE_remove_blend(
        edge: PK_EDGE_t,
    ) -> PK_ERROR_t;

    /// Returns topology created by blend fixing for an edge.
    pub fn PK_EDGE_find_blend_topol(
        edge: PK_EDGE_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Blend surface interrogation
    // -------------------------------------------------------------------------

    /// Creates a blend surface between two surfaces.
    pub fn PK_SURF_create_blend(
        surf_1: PK_SURF_t,
        sense_1: PK_LOGICAL_t,
        range_1: c_double,
        surf_2: PK_SURF_t,
        sense_2: PK_LOGICAL_t,
        range_2: c_double,
        blendsf: *mut PK_BLENDSF_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Face-face blending
    // -------------------------------------------------------------------------

    /// Creates a face-face blend between two walls of faces.
    pub fn PK_FACE_make_blend(
        n_left_wall_faces: c_int,
        left_wall_faces: *const PK_FACE_t,
        n_right_wall_faces: c_int,
        right_wall_faces: *const PK_FACE_t,
        left_sense: PK_LOGICAL_t,
        right_sense: PK_LOGICAL_t,
        options: *const PK_FACE_make_blend_o_t,
        n_sheet_bodies: *mut c_int,
        sheet_bodies: *mut *mut PK_BODY_t,
        n_blend_faces: *mut c_int,
        blend_faces: *mut *mut PK_FACE_t,
        unders: *mut *mut PK_FACE_t,
        ribs: *mut PK_blend_rib_r_t,
        fault: *mut PK_fxf_error_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Three-face blending
    // -------------------------------------------------------------------------

    /// Creates a three-face blend (full round fillet) between three walls of faces.
    pub fn PK_FACE_make_3_face_blend(
        n_left_wall_faces: c_int,
        left_wall_faces: *const PK_FACE_t,
        n_right_wall_faces: c_int,
        right_wall_faces: *const PK_FACE_t,
        n_centre_wall_faces: c_int,
        centre_wall_faces: *const PK_FACE_t,
        guide: PK_ENTITY_t,
        options: *const PK_FACE_make_3_face_blend_o_t,
        results: *mut PK_FACE_make_3_face_blend_r_t,
    ) -> PK_ERROR_t;

    /// Frees result structure from PK_FACE_make_3_face_blend.
    pub fn PK_FACE_make_3_face_blend_r_f(
        results: *mut PK_FACE_make_3_face_blend_r_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Blend identification and analysis
    // -------------------------------------------------------------------------

    /// Deletes blends from faces.
    pub fn PK_FACE_delete_blends(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_deleted: *mut c_int,
        deleted: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Finds underlying faces of blend faces.
    pub fn PK_FACE_find_blend_unders(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_unders: *mut c_int,
        unders: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Identifies blend faces on a body.
    pub fn PK_FACE_identify_blends(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_blends: *mut c_int,
        blends: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Identifies blend faces on a body (version 2, more options).
    pub fn PK_FACE_identify_blends_2(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_blends: *mut c_int,
        blends: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Creates a vertex blend at a vertex.
    pub fn PK_VERTEX_make_blend(
        vertex: PK_VERTEX_t,
        radius: c_double,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_t;

    // -------------------------------------------------------------------------
    // Free functions for returned arrays
    // -------------------------------------------------------------------------

    /// Frees result structure from PK_FACE_find_blend_unders.
    pub fn PK_FACE_find_blend_unders_r_f(
        n_unders: c_int,
        unders: *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Frees result structure from PK_FACE_identify_blends.
    pub fn PK_FACE_identify_blends_r_f(
        n_blends: c_int,
        blends: *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Frees result structure from PK_FACE_identify_blends_2.
    pub fn PK_FACE_identify_blends_2_r_f(
        n_blends: c_int,
        blends: *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Frees rib result structure.
    pub fn PK_blend_rib_r_f(
        ribs: *mut PK_blend_rib_r_t,
    ) -> PK_ERROR_t;

    /// Asks blend information from a local ball (LBALL).
    pub fn PK_LBALL_ask_blend(
        lball: PK_ENTITY_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Frees result from PK_LBALL_ask_blend.
    pub fn PK_LBALL_ask_blend_r_f(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        n_faces: c_int,
        faces: *mut PK_FACE_t,
    ) -> PK_ERROR_t;

    /// Query the standard form of a blend surface (rolling-ball blend).
    /// Blend surfaces have no create function — they are produced by blending operations.
    pub fn PK_BLENDSF_ask(blendsf: PK_BLENDSF_t, sf: *mut PK_BLENDSF_sf_t) -> PK_ERROR_code_t;
}
