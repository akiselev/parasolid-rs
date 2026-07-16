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
pub const PK_blend_xs_shape_conic_c: PK_blend_xs_shape_t = 22201;
/// Curvature-continuous (G2) cross-section.
pub const PK_blend_xs_shape_g2_c: PK_blend_xs_shape_t = 22202;
/// Linear/chamfer cross-section (only via PK_EDGE_set_blend_chain).
pub const PK_blend_xs_shape_chamfer_c: PK_blend_xs_shape_t = 22203;
/// Legacy: implied from other fields (not recommended).
pub const PK_blend_xs_shape_unset_c: PK_blend_xs_shape_t = 22200;

// =============================================================================
// Cross-section plane type — PK_blend_xs_t
// =============================================================================

pub type PK_blend_xs_t = c_int;

/// Rolling-ball: cross-section orthogonal to walls (default).
pub const PK_blend_xs_rolling_ball_c: PK_blend_xs_t = 18490;
/// Disc: cross-section orthogonal to parameter spine.
pub const PK_blend_xs_disc_c: PK_blend_xs_t = 18491;
/// Isoparameter: iso-parametric curves in left wall.
pub const PK_blend_xs_isoparameter_c: PK_blend_xs_t = 18492;

// =============================================================================
// Rho interpretation — PK_blend_rho_t
// =============================================================================

pub type PK_blend_rho_t = c_int;

/// Rho independent of angle subtended by blend (default).
pub const PK_blend_rho_absolute_c: PK_blend_rho_t = 22850;
/// Rho relative to angle subtended by blend.
pub const PK_blend_rho_relative_c: PK_blend_rho_t = 22851;
/// Rho is radius of curvature at centre of cross-section.
pub const PK_blend_rho_centre_c: PK_blend_rho_t = 22852;

// =============================================================================
// Size type — PK_blend_size_t
// =============================================================================

pub type PK_blend_size_t = c_int;

/// Size = distance of face offset (default).
pub const PK_blend_size_face_offset_c: PK_blend_size_t = 26310;
/// Size = range to apex of blend.
pub const PK_blend_size_apex_range_c: PK_blend_size_t = 26311;
/// Size = angle between chord and tangent plane.
pub const PK_blend_size_angle_c: PK_blend_size_t = 26312;

// =============================================================================
// Range type — PK_blend_range_t
// =============================================================================

pub type PK_blend_range_t = c_int;

/// Default range type (face offset).
pub const PK_blend_range_face_offset_c: PK_blend_range_t = 26320;
// [re-abi] appended 2 missing member(s) from pk-enums.h
pub const PK_blend_range_apex_range_c: PK_blend_range_t = 26321;
pub const PK_blend_range_angle_c: PK_blend_range_t = 26322;

// =============================================================================
// Overflow: smooth — PK_blend_ov_smooth_t
// =============================================================================

pub type PK_blend_ov_smooth_t = c_int;

/// Prevent smooth overflow.
pub const PK_blend_ov_smooth_no_c: PK_blend_ov_smooth_t = 18440;
/// Allow smooth overflow at any convexity.
pub const PK_blend_ov_smooth_any_c: PK_blend_ov_smooth_t = 18442;
/// Allow smooth overflow only at different convexity (default).
pub const PK_blend_ov_smooth_diff_c: PK_blend_ov_smooth_t = 18441;

// =============================================================================
// Overflow: cliff — PK_blend_ov_cliff_t
// =============================================================================

pub type PK_blend_ov_cliff_t = c_int;

/// Prevent cliff overflow.
pub const PK_blend_ov_cliff_no_c: PK_blend_ov_cliff_t = 18450;
/// Allow cliff overflow at any convexity.
pub const PK_blend_ov_cliff_any_c: PK_blend_ov_cliff_t = 18452;
/// Allow cliff overflow only at different convexity (default).
pub const PK_blend_ov_cliff_diff_c: PK_blend_ov_cliff_t = 18451;

// =============================================================================
// Overflow: cliff end — PK_blend_ov_cliff_end_t
// =============================================================================

pub type PK_blend_ov_cliff_end_t = c_int;

/// Do not allow cliff end overflow (default).
pub const PK_blend_ov_cliff_end_no_c: PK_blend_ov_cliff_end_t = 18460;
/// Allow cliff end overflow.
pub const PK_blend_ov_cliff_end_yes_c: PK_blend_ov_cliff_end_t = 18461;

// =============================================================================
// Overflow: notch — PK_blend_ov_notch_t
// =============================================================================

pub type PK_blend_ov_notch_t = c_int;

/// Prevent notch overflow.
pub const PK_blend_ov_notch_no_c: PK_blend_ov_notch_t = 18470;
/// Allow notch overflow (default).
pub const PK_blend_ov_notch_yes_c: PK_blend_ov_notch_t = 18471;

// =============================================================================
// Overflow: explicit cliff — PK_blend_ov_exp_cliff_t
// =============================================================================

pub type PK_blend_ov_exp_cliff_t = c_int;

/// Create cliff edge along specified edge.
pub const PK_blend_ov_exp_cliff_yes_c: PK_blend_ov_exp_cliff_t = 23501;
/// Do not create cliff edge along specified edge.
pub const PK_blend_ov_exp_cliff_no_c: PK_blend_ov_exp_cliff_t = 23500;

// =============================================================================
// Setback collar — PK_blend_setback_collar_t
// =============================================================================

pub type PK_blend_setback_collar_t = c_int;

/// Every blended edge includes collar face (default, not for G2).
pub const PK_blend_setback_collar_all_c: PK_blend_setback_collar_t = 23130;
/// No collar faces.
pub const PK_blend_setback_collar_none_c: PK_blend_setback_collar_t = 23131;

// =============================================================================
// Blend ordering — PK_blend_order_t
// =============================================================================

pub type PK_blend_order_t = c_int;

/// Parasolid decides order.
pub const PK_blend_order_unset_c: PK_blend_order_t = 23124;
/// Blend concave edges first.
pub const PK_blend_order_concave_convex_c: PK_blend_order_t = 23122;
/// Blend convex edges first.
pub const PK_blend_order_convex_concave_c: PK_blend_order_t = 23123;
/// Blend minority convexity edges first.
pub const PK_blend_order_min_convexity_c: PK_blend_order_t = 23120;
/// Blend majority convexity edges first.
pub const PK_blend_order_maj_convexity_c: PK_blend_order_t = 23121;

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
pub const PK_blend_propagate_no_c: PK_blend_propagate_t = 18410;
/// Propagate blend across smooth edges.
pub const PK_blend_propagate_yes_c: PK_blend_propagate_t = 18411;

// =============================================================================
// Set tolerance — PK_blend_set_tol_t
// =============================================================================

pub type PK_blend_set_tol_t = c_int;

/// Apply tolerance if blend would otherwise fail (default).
pub const PK_blend_set_tol_yes_c: PK_blend_set_tol_t = 22380;
/// Do not apply tolerance.
pub const PK_blend_set_tol_no_c: PK_blend_set_tol_t = 22381;

// =============================================================================
// Tolerance improvement — PK_blend_tolerance_t
// =============================================================================

pub type PK_blend_tolerance_t = c_int;

/// Do not reduce tolerance of new edges (default).
pub const PK_blend_tolerance_standard_c: PK_blend_tolerance_t = 24130;
/// Reduce tolerance of new edges if possible.
pub const PK_blend_tolerance_improved_c: PK_blend_tolerance_t = 24131;

// =============================================================================
// Face self-intersection repair — PK_blend_repair_fa_X_t
// =============================================================================

pub type PK_blend_repair_fa_X_t = c_int;

/// Do not repair self-intersecting faces (default).
pub const PK_blend_repair_fa_X_no_c: PK_blend_repair_fa_X_t = 23340;
/// Attempt repair.
pub const PK_blend_repair_fa_X_yes_c: PK_blend_repair_fa_X_t = 23341;

// =============================================================================
// Surface self-intersection repair — PK_blend_repair_su_X_t
// =============================================================================

pub type PK_blend_repair_su_X_t = c_int;

/// Do not repair (default).
pub const PK_blend_repair_su_X_no_c: PK_blend_repair_su_X_t = 22710;
/// Repair self-intersecting surfaces.
pub const PK_blend_repair_su_X_yes_c: PK_blend_repair_su_X_t = 22711;
/// Repair and report.
pub const PK_blend_repair_su_X_report_c: PK_blend_repair_su_X_t = 22712;

// =============================================================================
// Report — PK_blend_report_t
// =============================================================================

pub type PK_blend_report_t = c_int;

/// Do not report repaired faces (default).
pub const PK_blend_report_repaired_no_c: PK_blend_report_t = 25031;
/// Save info about repaired surfaces to Report.
pub const PK_blend_report_repaired_yes_c: PK_blend_report_t = 25030;

// =============================================================================
// Inside tight — PK_blend_inside_tight_t
// =============================================================================

pub type PK_blend_inside_tight_t = c_int;

/// No tight blending.
pub const PK_blend_inside_tight_no_c: PK_blend_inside_tight_t = 22990;
/// Blend across blend-like faces only (default for PK_BODY_fix_blends).
pub const PK_blend_inside_tight_blends_c: PK_blend_inside_tight_t = 22991;
/// Blend across full tight faces.
pub const PK_blend_inside_tight_faces_c: PK_blend_inside_tight_t = 22992;
/// Blend across partially tight regions.
pub const PK_blend_inside_tight_partial_c: PK_blend_inside_tight_t = 22993;

// =============================================================================
// Limit type — PK_blend_limit_type_t
// =============================================================================

pub type PK_blend_limit_type_t = c_int;

/// Edge limit.
pub const PK_blend_limit_type_edge_c: PK_blend_limit_type_t = 24930;
/// Overlap limit.
pub const PK_blend_limit_type_overlap_c: PK_blend_limit_type_t = 24931;

// =============================================================================
// Limit patch — PK_blend_limit_patch_t
// =============================================================================

pub type PK_blend_limit_patch_t = c_int;

/// Do not patch.
pub const PK_blend_limit_patch_no_c: PK_blend_limit_patch_t = 25041;
/// Attempt to patch blend in region of limit.
pub const PK_blend_limit_patch_yes_c: PK_blend_limit_patch_t = 25040;

// =============================================================================
// Laminar trim — PK_blend_laminar_trim_t
// =============================================================================

pub type PK_blend_laminar_trim_t = c_int;

/// Trim to edges of underlying faces (default).
pub const PK_blend_laminar_trim_edges_c: PK_blend_laminar_trim_t = 24740;
/// Trim to widest face only.
pub const PK_blend_laminar_trim_bound_c: PK_blend_laminar_trim_t = 24741;

// =============================================================================
// Output sheet (preview) — PK_blend_output_sheet_t
// =============================================================================

pub type PK_blend_output_sheet_t = c_int;

/// Fix blends normally (default).
pub const PK_blend_output_sheet_no_c: PK_blend_output_sheet_t = 22720;
/// Create preview sheet body.
pub const PK_blend_output_sheet_yes_c: PK_blend_output_sheet_t = 22721;
/// Create preview sheet only if blend would fail.
pub const PK_blend_output_sheet_on_fail_c: PK_blend_output_sheet_t = 22722;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_blend_fault_no_fault_c: PK_blend_output_sheet_t = 18391;

// =============================================================================
// Suggest limit — PK_blend_suggest_limit_t
// =============================================================================

pub type PK_blend_suggest_limit_t = c_int;

/// Do not generate limit data (default).
pub const PK_blend_suggest_limit_no_c: PK_blend_suggest_limit_t = 24780;
/// Generate limit data (not for patching).
pub const PK_blend_suggest_limit_yes_c: PK_blend_suggest_limit_t = 24781;
/// Generate limit data suitable for patching.
pub const PK_blend_suggest_limit_patch_c: PK_blend_suggest_limit_t = 24782;

// =============================================================================
// Report extended — PK_blend_report_extended_t
// =============================================================================

pub type PK_blend_report_extended_t = c_int;

/// Do not report extended chamfers (default).
pub const PK_blend_report_extended_no_c: PK_blend_report_extended_t = 26631;
/// Report extended chamfers.
pub const PK_blend_report_extended_yes_c: PK_blend_report_extended_t = 26630;

// =============================================================================
// Edge update — PK_blend_edge_update_t
// =============================================================================

pub type PK_blend_edge_update_t = c_int;

/// Use all blending enhancements (default).
pub const PK_blend_edge_update_default_c: PK_blend_edge_update_t = 22192;
// [re-abi] appended 23 missing member(s) from pk-enums.h
pub const PK_blend_edge_update_0_c: PK_blend_edge_update_t = 22190;
pub const PK_blend_edge_update_1_c: PK_blend_edge_update_t = 22191;
pub const PK_blend_edge_update_2_c: PK_blend_edge_update_t = 22193;
pub const PK_blend_edge_update_3_c: PK_blend_edge_update_t = 22194;
pub const PK_blend_edge_update_4_c: PK_blend_edge_update_t = 22195;
pub const PK_blend_edge_update_5_c: PK_blend_edge_update_t = 22196;
pub const PK_blend_edge_update_6_c: PK_blend_edge_update_t = 22197;
pub const PK_blend_edge_update_7_c: PK_blend_edge_update_t = 22198;
pub const PK_blend_edge_update_8_c: PK_blend_edge_update_t = 22199;
pub const PK_blend_edge_update_9_c: PK_blend_edge_update_t = 23810;
pub const PK_blend_edge_update_10_c: PK_blend_edge_update_t = 23811;
pub const PK_blend_edge_update_11_c: PK_blend_edge_update_t = 23812;
pub const PK_blend_edge_update_12_c: PK_blend_edge_update_t = 23813;
pub const PK_blend_edge_update_13_c: PK_blend_edge_update_t = 23814;
pub const PK_blend_edge_update_14_c: PK_blend_edge_update_t = 23815;
pub const PK_blend_edge_update_15_c: PK_blend_edge_update_t = 23816;
pub const PK_blend_edge_update_16_c: PK_blend_edge_update_t = 23817;
pub const PK_blend_edge_update_17_c: PK_blend_edge_update_t = 23818;
pub const PK_blend_edge_update_18_c: PK_blend_edge_update_t = 23819;
pub const PK_blend_edge_update_v261_c: PK_blend_edge_update_t = 23822;
pub const PK_blend_edge_update_v270_c: PK_blend_edge_update_t = 23823;
pub const PK_blend_edge_update_v271_c: PK_blend_edge_update_t = 23824;
pub const PK_blend_edge_update_v280_c: PK_blend_edge_update_t = 23825;

// =============================================================================
// Tracking type — PK_blend_track_type_t
// =============================================================================

pub type PK_blend_track_type_t = c_int;

/// Track underlying faces including removed ones.
pub const PK_blend_track_type_unders_c: PK_blend_track_type_t = 24211;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_blend_track_type_basic_c: PK_blend_track_type_t = 24210;

// =============================================================================
// Trim — PK_blend_trim_t
// =============================================================================

pub type PK_blend_trim_t = c_int;

/// Trim to walls (default).
pub const PK_blend_trim_to_walls_c: PK_blend_trim_t = 17402;
/// Do not trim.
pub const PK_blend_trim_no_c: PK_blend_trim_t = 17401;
/// Long trim (as long as possible).
pub const PK_blend_trim_long_c: PK_blend_trim_t = 17421;
/// Short trim (as short as possible).
pub const PK_blend_trim_short_c: PK_blend_trim_t = 17420;

// =============================================================================
// Trim extent (three-face) — PK_blend_trim_extent_t
// =============================================================================

pub type PK_blend_trim_extent_t = c_int;

/// Include centre wall when trimming (default).
pub const PK_blend_trim_extent_all_c: PK_blend_trim_extent_t = 24380;
/// Ignore centre wall when trimming.
pub const PK_blend_trim_extent_sides_c: PK_blend_trim_extent_t = 24381;

// =============================================================================
// Walls — PK_blend_walls_t
// =============================================================================

pub type PK_blend_walls_t = c_int;

/// Trim and attach (default).
pub const PK_blend_walls_attach_c: PK_blend_walls_t = 17404;
/// No wall trimming, blend as sheet.
pub const PK_blend_walls_trim_no_c: PK_blend_walls_t = 18480;
/// Trim both walls, blend as sheet (sheets only).
pub const PK_blend_walls_trim_both_c: PK_blend_walls_t = 17403;
/// Trim, attach, create solid if closed.
pub const PK_blend_walls_solid_c: PK_blend_walls_t = 17417;
/// Preview sheet without modification.
pub const PK_blend_walls_preview_c: PK_blend_walls_t = 18481;

// =============================================================================
// Orientation (three-face) — PK_blend_orientation_t
// =============================================================================

pub type PK_blend_orientation_t = c_int;

/// Let Parasolid determine (default).
pub const PK_blend_orientation_unknown_c: PK_blend_orientation_t = 22322;
/// Blend in front of wall normal.
pub const PK_blend_orientation_before_c: PK_blend_orientation_t = 22320;
/// Blend behind wall normal.
pub const PK_blend_orientation_behind_c: PK_blend_orientation_t = 22321;

// =============================================================================
// Run out — PK_blend_run_out_t
// =============================================================================

pub type PK_blend_run_out_t = c_int;

/// Do not stop in shallow regions (default).
pub const PK_blend_run_out_no_c: PK_blend_run_out_t = 24751;
/// Stop at specified angle.
pub const PK_blend_run_out_angle_c: PK_blend_run_out_t = 24750;

// =============================================================================
// Master faces — PK_blend_use_master_faces_t
// =============================================================================

pub type PK_blend_use_master_faces_t = c_int;

/// Require master face (default).
pub const PK_blend_use_master_faces_yes_c: PK_blend_use_master_faces_t = 25561;
/// No master face requirement.
pub const PK_blend_use_master_faces_no_c: PK_blend_use_master_faces_t = 25560;

// =============================================================================
// Output rib — PK_blend_output_rib_t
// =============================================================================

pub type PK_blend_output_rib_t = c_int;

/// No ribs (default).
pub const PK_blend_output_rib_no_c: PK_blend_output_rib_t = 21020;
/// Ribs where surface fails.
pub const PK_blend_output_rib_on_fail_c: PK_blend_output_rib_t = 21021;
/// Ribs only, no surfaces.
pub const PK_blend_output_rib_only_c: PK_blend_output_rib_t = 21022;
/// Ribs and surfaces.
pub const PK_blend_output_rib_also_c: PK_blend_output_rib_t = 21023;
/// Single rib at help point.
pub const PK_blend_output_rib_at_help_c: PK_blend_output_rib_t = 21024;

// =============================================================================
// Group rib — PK_blend_group_rib_t
// =============================================================================

pub type PK_blend_group_rib_t = c_int;

/// No grouping (default).
pub const PK_blend_group_rib_no_c: PK_blend_group_rib_t = 21030;
/// Group by blend face.
pub const PK_blend_group_rib_by_face_c: PK_blend_group_rib_t = 21031;
/// Group by parameter intervals.
pub const PK_blend_group_rib_by_parms_c: PK_blend_group_rib_t = 21032;

// =============================================================================
// Check surface self-intersection — PK_blend_check_su_X_t
// =============================================================================

pub type PK_blend_check_su_X_t = c_int;

/// No surface self-intersection check (default).
pub const PK_blend_check_su_X_no_c: PK_blend_check_su_X_t = 21040;
/// Check all except B-surfaces.
pub const PK_blend_check_su_X_not_bsurf_c: PK_blend_check_su_X_t = 21041;

// =============================================================================
// Check face — PK_blend_check_fa_t
// =============================================================================

pub type PK_blend_check_fa_t = c_int;

/// No face check (default).
pub const PK_blend_check_fa_no_c: PK_blend_check_fa_t = 21050;
/// Check faces.
pub const PK_blend_check_fa_yes_c: PK_blend_check_fa_t = 21051;

// =============================================================================
// Check face-face — PK_blend_check_fa_fa_t
// =============================================================================

pub type PK_blend_check_fa_fa_t = c_int;

/// No face-face check (default).
pub const PK_blend_check_fa_fa_no_c: PK_blend_check_fa_fa_t = 21060;
/// Check face-face consistency.
pub const PK_blend_check_fa_fa_yes_c: PK_blend_check_fa_fa_t = 21061;

// =============================================================================
// Track edges — PK_blend_track_edges_t
// =============================================================================

pub type PK_blend_track_edges_t = c_int;

/// No edge tracking (default).
pub const PK_blend_track_edges_no_c: PK_blend_track_edges_t = 24090;
/// Track laminar edges.
pub const PK_blend_track_edges_laminar_c: PK_blend_track_edges_t = 24091;

// =============================================================================
// Update — PK_blend_update_t
// =============================================================================

pub type PK_blend_update_t = c_int;

/// Use all enhancements (default).
pub const PK_blend_update_default_c: PK_blend_update_t = 21072;
// [re-abi] appended 23 missing member(s) from pk-enums.h
pub const PK_blend_update_0_c: PK_blend_update_t = 21070;
pub const PK_blend_update_1_c: PK_blend_update_t = 21071;
pub const PK_blend_update_2_c: PK_blend_update_t = 21073;
pub const PK_blend_update_3_c: PK_blend_update_t = 21074;
pub const PK_blend_update_4_c: PK_blend_update_t = 21075;
pub const PK_blend_update_5_c: PK_blend_update_t = 21076;
pub const PK_blend_update_6_c: PK_blend_update_t = 21077;
pub const PK_blend_update_7_c: PK_blend_update_t = 21078;
pub const PK_blend_update_8_c: PK_blend_update_t = 21079;
pub const PK_blend_update_9_c: PK_blend_update_t = 24220;
pub const PK_blend_update_10_c: PK_blend_update_t = 24221;
pub const PK_blend_update_11_c: PK_blend_update_t = 24222;
pub const PK_blend_update_12_c: PK_blend_update_t = 24223;
pub const PK_blend_update_13_c: PK_blend_update_t = 24224;
pub const PK_blend_update_14_c: PK_blend_update_t = 24225;
pub const PK_blend_update_15_c: PK_blend_update_t = 24226;
pub const PK_blend_update_16_c: PK_blend_update_t = 24227;
pub const PK_blend_update_17_c: PK_blend_update_t = 24228;
pub const PK_blend_update_18_c: PK_blend_update_t = 24229;
pub const PK_blend_update_v261_c: PK_blend_update_t = 24231;
pub const PK_blend_update_v270_c: PK_blend_update_t = 24232;
pub const PK_blend_update_v271_c: PK_blend_update_t = 24233;
pub const PK_blend_update_v280_c: PK_blend_update_t = 24234;

// =============================================================================
// Imprint complete — PK_imprint_complete_t
// =============================================================================

// =============================================================================
// Extension shape — PK_extension_shape_t
// =============================================================================

pub type PK_extension_shape_t = c_int;

/// Linear extension (default).
pub const PK_extension_shape_linear_c: PK_extension_shape_t = 22750;
/// Curvature-continuous extension.
pub const PK_extension_shape_soft_c: PK_extension_shape_t = 22751;

// =============================================================================
// Prevent sharp — PK_blend_prevent_sharp_t
// =============================================================================

pub type PK_blend_prevent_sharp_t = c_int;

/// Do not prevent sharp edges (default).
pub const PK_blend_prevent_sharp_no_c: PK_blend_prevent_sharp_t = 23790;
/// Prevent creation of sharp edges.
pub const PK_blend_prevent_sharp_yes_c: PK_blend_prevent_sharp_t = 23791;

// =============================================================================
// Edge blend fault codes — PK_blend_fault_t
// (PK_blend_fault_t type alias lives in error.rs)
// =============================================================================

// Severe errors
/// Vertex configuration too complex (4+ adjacent edges, at least 2 blended).
pub const PK_blend_fault_vertex_c: PK_blend_fault_t = 16053;
/// Unspecified numerical problem.
pub const PK_blend_fault_unknown_c: PK_blend_fault_t = 16065;

// General configuration errors
/// Blend requires invalid B-surface extension.
pub const PK_blend_fault_bsurf_c: PK_blend_fault_t = 16058;
/// Range inconsistent with adjacent blended edge.
pub const PK_blend_fault_range_c: PK_blend_fault_t = 16059;
/// Adjoining edge not blended (illegal 2-of-3 vertex config).
pub const PK_blend_fault_edge_c: PK_blend_fault_t = 16061;
/// Blend completely overlaps edge loop.
pub const PK_blend_fault_loop_c: PK_blend_fault_t = 16062;
/// Unblended edge overlapped by blend.
pub const PK_blend_fault_overlap_edge_c: PK_blend_fault_t = 16064;
/// Range of blend on face too large (or radius of curvature too small).
pub const PK_blend_fault_face_c: PK_blend_fault_t = 16067;
/// Supplied rho value too large (cross section too flat).
pub const PK_blend_fault_rho_too_large_c: PK_blend_fault_t = 18397;
/// Illegal blend on another edge prevented full check.
pub const PK_blend_fault_other_edge_c: PK_blend_fault_t = 16072;
/// Range on chamfer blend too large (chord misses opposite surface).
pub const PK_blend_fault_apex_range_c: PK_blend_fault_t = 25422;

// Overlapping blend errors
/// Overlapping blends failure.
pub const PK_blend_fault_overlap_c: PK_blend_fault_t = 16063;
/// Overlapping blends at end failure.
pub const PK_blend_fault_overlap_end_c: PK_blend_fault_t = 16068;
/// Blend end failure.
pub const PK_blend_fault_end_c: PK_blend_fault_t = 16069;
/// End boundary intersects unblended edge.
pub const PK_blend_fault_edge_intsec_c: PK_blend_fault_t = 16071;

// Post-fix errors
/// Blend created face-face inconsistency (only with explicit check).
pub const PK_blend_fault_face_face_c: PK_blend_fault_t = 18393;
/// Blend surface is self-intersecting (only with explicit check).
pub const PK_blend_fault_self_int_c: PK_blend_fault_t = 18392;
// [re-abi] appended 18 missing member(s) from pk-enums.h
pub const PK_blend_fault_singularity_c: PK_blend_fault_t = 16051;
pub const PK_blend_fault_obsolete_c: PK_blend_fault_t = 16052;
pub const PK_blend_fault_sheet_c: PK_blend_fault_t = 16054;
pub const PK_blend_fault_general_c: PK_blend_fault_t = 16055;
pub const PK_blend_fault_2_edge_c: PK_blend_fault_t = 16056;
pub const PK_blend_fault_chamfer_c: PK_blend_fault_t = 16057;
pub const PK_blend_fault_type_c: PK_blend_fault_t = 16060;
pub const PK_blend_fault_bad_end_c: PK_blend_fault_t = 16066;
pub const PK_blend_fault_chamfer_intsec_c: PK_blend_fault_t = 16070;
pub const PK_blend_fault_tangent_c: PK_blend_fault_t = 16073;
pub const PK_blend_fault_cliffedge_c: PK_blend_fault_t = 16074;
pub const PK_blend_fault_no_blend_c: PK_blend_fault_t = 18390;
pub const PK_blend_fault_conic_c: PK_blend_fault_t = 18394;
pub const PK_blend_fault_repaired_c: PK_blend_fault_t = 18395;
pub const PK_blend_fault_bad_cap_c: PK_blend_fault_t = 18396;
pub const PK_blend_fault_g2_vx_blend_c: PK_blend_fault_t = 18399;
pub const PK_blend_fault_g2_collar_setb_c: PK_blend_fault_t = 25420;
pub const PK_blend_fault_face_not_g1_c: PK_blend_fault_t = 25421;

// =============================================================================
// Face-face blend fault codes — PK_fxf_fault_t
// =============================================================================

pub type PK_fxf_fault_t = c_int;

// Success / partial success
/// Blend succeeded.
pub const PK_fxf_fault_no_fault_c: PK_fxf_fault_t = 17451;
/// Blend not attached, sheet bodies created (partial success).
pub const PK_fxf_fault_sheet_c: PK_fxf_fault_t = 17452;

// General failures
/// Blend could not be created.
pub const PK_fxf_fault_unknown_c: PK_fxf_fault_t = 17453;
/// Insufficient data to define blend.
pub const PK_fxf_fault_insufficient_c: PK_fxf_fault_t = 17454;
/// Inconsistent data supplied.
pub const PK_fxf_fault_inconsistent_c: PK_fxf_fault_t = 17455;

// Input validation errors
/// Invalid wall of faces.
pub const PK_fxf_fault_wall_c: PK_fxf_fault_t = 17456;
/// Invalid range definition.
pub const PK_fxf_fault_range_c: PK_fxf_fault_t = 17457;
/// Invalid tangent holdline data.
pub const PK_fxf_fault_thl_c: PK_fxf_fault_t = 17458;
/// Invalid cliff-edge data.
pub const PK_fxf_fault_cliff_c: PK_fxf_fault_t = 17459;
/// Invalid conic holdline data.
pub const PK_fxf_fault_chl_c: PK_fxf_fault_t = 17472;
/// Invalid rho values in rho law function.
pub const PK_fxf_fault_rho_value_c: PK_fxf_fault_t = 17470;
/// Asymmetric ranges inconsistent with geometry.
pub const PK_fxf_fault_asymmetric_c: PK_fxf_fault_t = 17471;
/// Invalid parameter spine.
pub const PK_fxf_fault_bad_spine_c: PK_fxf_fault_t = 17473;
/// Invalid rib controls.
pub const PK_fxf_fault_bad_ribs_c: PK_fxf_fault_t = 17474;
/// Limit plane origin not unique.
pub const PK_fxf_fault_plane_origin_c: PK_fxf_fault_t = 17476;
/// No preview rib could be constructed.
pub const PK_fxf_fault_preview_rib_c: PK_fxf_fault_t = 17478;

// Geometric errors
/// Face in wall too tightly curved.
pub const PK_fxf_fault_curved_c: PK_fxf_fault_t = 17460;
/// Blend range too small.
pub const PK_fxf_fault_small_c: PK_fxf_fault_t = 17461;
/// Blend range too large.
pub const PK_fxf_fault_large_c: PK_fxf_fault_t = 17462;
/// Rho too large, cross section too flat vs underlying surface.
pub const PK_fxf_fault_rho_too_large_c: PK_fxf_fault_t = 17479;

// Sense errors
/// left_sense is incorrect.
pub const PK_fxf_fault_left_c: PK_fxf_fault_t = 17463;
/// right_sense is incorrect.
pub const PK_fxf_fault_right_c: PK_fxf_fault_t = 17464;
/// Both left_sense and right_sense incorrect.
pub const PK_fxf_fault_both_c: PK_fxf_fault_t = 17465;

// Post-attachment errors
/// Blend sheets intersect each other.
pub const PK_fxf_fault_sheet_clash_c: PK_fxf_fault_t = 17466;
/// Attached blend combined bodies causing face-face inconsistency.
pub const PK_fxf_fault_wall_clash_c: PK_fxf_fault_t = 17467;
/// Attached blend caused face-face inconsistency.
pub const PK_fxf_fault_face_face_c: PK_fxf_fault_t = 17469;
/// Blend face(s) with self-intersecting geometry.
pub const PK_fxf_fault_self_int_c: PK_fxf_fault_t = 17468;
// [re-abi] appended 5 missing member(s) from pk-enums.h
pub const PK_fxf_fault_repaired_c: PK_fxf_fault_t = 17475;
pub const PK_fxf_fault_plane_insuff_c: PK_fxf_fault_t = 17477;
pub const PK_fxf_fault_depth_value_c: PK_fxf_fault_t = 17480;
pub const PK_fxf_fault_skew_value_c: PK_fxf_fault_t = 17481;
pub const PK_blend_prevent_sharp_report_c: PK_fxf_fault_t = 23792;

// =============================================================================
// Three-face blend status — PK_3_face_blend_status_t
// =============================================================================

pub type PK_3_face_blend_status_t = c_int;

/// Success.
pub const PK_3_face_blend_ok_c: PK_3_face_blend_status_t = 22330;
/// Partial success, returned as sheet body.
pub const PK_3_face_blend_sheet_c: PK_3_face_blend_status_t = 22331;

// =============================================================================
// Report record type constants
// =============================================================================

pub const PK_REPORT_record_type_1_c: PK_REPORT_record_type_t = 23850;

// Report status values — type 1
pub type PK_REPORT_1_t = c_int;

/// Faces modified to repair self-intersections.
pub const PK_REPORT_1_rep_sx_faces_c: PK_REPORT_1_t = 23908;
/// G2 blend faces modified to maintain continuity.
pub const PK_REPORT_1_rep_G2_faces_c: PK_REPORT_1_t = 23909;
/// Self-intersecting blend faces.
pub const PK_REPORT_1_blend_faces_sx_c: PK_REPORT_1_t = 23921;

// Report status values — type 3
pub type PK_REPORT_3_t = c_int;

/// Report contains limit data.
pub const PK_REPORT_3_limit_data_c: PK_REPORT_3_t = 0;
/// Report contains limit topology data.
pub const PK_REPORT_3_limit_topol_c: PK_REPORT_3_t = 1;
/// Chamfer face extended outside original range.
pub const PK_REPORT_3_chamfer_extended_c: PK_REPORT_3_t = 25637;
/// Geometry that could not be extended or had G2 discontinuity.
pub const PK_REPORT_3_blend_x_g1_c: PK_REPORT_3_t = 24413;

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
    pub o_t_version: c_int, // @0
    /// Edge at which the blend "cliffs" (PK_ENTITY_null for none). The old
    /// struct swapped this with `properties` and added a bogus `propagate`.
    pub cliff_edge: PK_EDGE_t, // @4
    pub properties: PK_blend_properties_t, // @8
    pub xs_shape: PK_blend_xs_shape_t, // @12
}

impl Default for PK_EDGE_set_blend_constant_o_t {
    fn default() -> Self {
        // Zero everything (cliff_edge = null, properties = all-zero struct), then
        // set the fields that need non-zero defaults. `xs_shape` 0 is not a valid
        // token — the explicit "unset" selects the default conic cross-section.
        let mut s: Self = unsafe { std::mem::zeroed() };
        s.o_t_version = 1;
        s.xs_shape = PK_blend_xs_shape_unset_c;
        s
    }
}

/// Parasolid array descriptor for faces (`{length, array}`). Treated as opaque
/// here — `PK_BODY_fix_blends` returns `PK_FACE_array_t **unders` which we only
/// pass through as a pointer.
#[repr(C)]
pub struct PK_FACE_array_t {
    _private: [u8; 0],
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
    /// Set a constant-radius rolling-ball blend on edges. V35:
    /// `(int n_edges, const PK_EDGE_t edges[], double radius,
    ///  const PK_EDGE_set_blend_constant_o_t *options, int *n_blend_edges,
    ///  PK_EDGE_t **blend_edges)`. The old binding took a single edge and
    /// dropped the outputs.
    pub fn PK_EDGE_set_blend_constant(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        radius: c_double,
        options: *const PK_EDGE_set_blend_constant_o_t,
        n_blend_edges: *mut c_int,
        blend_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Sets a variable rolling-ball blend on a chain of edges.
    /// Also used for chamfer blends (with xs_shape = PK_blend_xs_shape_chamfer_c).
    pub fn PK_EDGE_set_blend_chain(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        options: *mut PK_EDGE_set_blend_chain_o_t,
        n_blend_edges: *mut c_int,
        blend_edges: *mut *mut PK_EDGE_t,
        n_primary_fins: *mut c_int,
        primary_fins: *mut *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Sets a chamfer (face offset) blend on an edge.
    pub fn PK_EDGE_set_blend_chamfer(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        range_2: c_double,
        range_1: c_double,
        faces: *mut PK_FACE_t,
        options: *mut PK_EDGE_set_blend_chamfer_o_t,
        n_blend_edges: *mut c_int,
        blend_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

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
    /// Realise blends set by `PK_EDGE_set_blend_*` into faces. V35 (8 args):
    /// `(body, options, int *n_blends, PK_FACE_t **blends,
    ///  PK_FACE_array_t **unders, int **topols, PK_blend_fault_t *fault,
    ///  PK_EDGE_t *fault_edge)`. The old binding had extra `n_topols`/`n_unders`
    /// counts, the wrong array order, and no `fault_edge`.
    pub fn PK_BODY_fix_blends(
        body: PK_BODY_t,
        options: *const PK_BODY_fix_blends_o_t,
        n_blends: *mut c_int,
        blends: *mut *mut PK_FACE_t,
        unders: *mut *mut PK_FACE_array_t,
        topols: *mut *mut c_int,
        fault: *mut PK_blend_fault_t,
        fault_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Edge blend: ask / remove
    // -------------------------------------------------------------------------

    /// Returns information about an unfixed blend on an edge.
    pub fn PK_EDGE_ask_blend(
        edge: PK_EDGE_t,
        r#type: *mut PK_blend_type_t,
        left_face: *mut PK_FACE_t,
        right_face: *mut PK_FACE_t,
        edge_shape: *mut PK_blend_edge_shape_t,
        properties: *mut PK_blend_properties_t,
        cliff_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

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
        geom1: PK_GEOM_t,
        range1: c_double,
        geom2: PK_GEOM_t,
        range2: c_double,
        start: *const PK_VECTOR_t,
        end: *const PK_VECTOR_t,
        options: *mut PK_SURF_create_blend_o_t,
        blend_surf: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

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
        faces: *mut PK_FACE_t,
        tolerance: c_double,
        options: *mut PK_FACE_delete_blends_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    /// Finds underlying faces of blend faces.
    pub fn PK_FACE_find_blend_unders(
        face: PK_FACE_t,
        options: *mut PK_FACE_find_blend_unders_o_t,
        results: *mut PK_FACE_find_blend_unders_r_t,
    ) -> PK_ERROR_code_t;

    /// Identifies blend faces on a body.
    pub fn PK_FACE_identify_blends(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        ident_type: PK_blend_identify_t,
        options: *mut PK_FACE_identify_blends_o_t,
        results: *mut PK_FACE_identify_blends_r_t,
    ) -> PK_ERROR_code_t;

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
        local_check: PK_LOGICAL_t,
        edge: *mut PK_EDGE_t,
        vertices: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Free functions for returned arrays
    // -------------------------------------------------------------------------

    /// Frees result structure from PK_FACE_find_blend_unders.
    pub fn PK_FACE_find_blend_unders_r_f(
        under_returns: *mut PK_FACE_find_blend_unders_r_t,
    ) -> PK_ERROR_code_t;

    /// Frees result structure from PK_FACE_identify_blends.
    pub fn PK_FACE_identify_blends_r_f(
        blend_returns: *mut PK_FACE_identify_blends_r_t,
    ) -> PK_ERROR_code_t;

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
        lball: PK_LBALL_t,
        options: *mut PK_LBALL_ask_blend_o_t,
        results: *mut PK_LBALL_ask_blend_r_t,
    ) -> PK_ERROR_code_t;

    /// Frees result from PK_LBALL_ask_blend.
    pub fn PK_LBALL_ask_blend_r_f(
        result: *mut PK_LBALL_ask_blend_r_t,
    ) -> PK_ERROR_code_t;

    /// Query the standard form of a blend surface (rolling-ball blend).
    /// Blend surfaces have no create function — they are produced by blending operations.
    pub fn PK_BLENDSF_ask(blendsf: PK_BLENDSF_t, sf: *mut PK_BLENDSF_sf_t) -> PK_ERROR_code_t;
}
