#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! FFI bindings for Parasolid faceting, rendering, mesh topology, convergent
//! modeling, PSM import, mesh checking, and picking functions.
//!
//! Covers:
//! - `PK_TOPOL_facet_2` — tabular facet output
//! - `PK_TOPOL_render_facet` — GO-based facet output
//! - `PK_TOPOL_render_line` — line rendering (wireframe, silhouette, hidden-line)
//! - `PK_GEOM_render` — geometry rendering (B-curves, B-surfaces)
//! - Mesh topology enquiry (`PK_MESH_*`, `PK_MFACET_*`, `PK_MFIN_*`, `PK_MVERTEX_*`)
//! - Convergent modeling session/partition functions
//! - PSM import (`PK_MESH_create_from_facets`)
//! - Mesh checking and repair (`PK_MESH_find_defects`, `PK_MESH_fix_defects`)
//! - Picking (`PK_BODY_pick_topols`)
//! - View matrices (`PK_TRANSF_create_view` is in transform.rs)

use std::os::raw::{c_double, c_int, c_void};

use crate::*;

// =============================================================================
// Facet shape constraint
// =============================================================================

pub type PK_facet_shape_t = c_int;

/// All interior angles convex, no interior holes (default).
pub const PK_facet_shape_convex_c: PK_facet_shape_t = 20502;
/// Concave angles permitted, no interior holes.
pub const PK_facet_shape_cut_c: PK_facet_shape_t = 20501;
/// Concave angles and interior holes permitted.
pub const PK_facet_shape_any_c: PK_facet_shape_t = 20500;

// =============================================================================
// Facet matching between adjacent face meshes
// =============================================================================

pub type PK_facet_match_t = c_int;

/// Topology matching -- single mesh, shared topology at boundaries (default).
pub const PK_facet_match_topol_c: PK_facet_match_t = 20522;
/// Geometry matching -- boundaries meet exactly but topologically disjoint.
pub const PK_facet_match_geom_c: PK_facet_match_t = 20521;
/// Trimmed -- no matching, gaps/overlaps within tolerance.
pub const PK_facet_match_trimmed_c: PK_facet_match_t = 20523;

// =============================================================================
// Facet density (view-dependent)
// =============================================================================

pub type PK_facet_density_t = c_int;

/// Independent of views (default).
pub const PK_facet_density_no_view_c: PK_facet_density_t = 20540;
/// Increase density at silhouettes.
pub const PK_facet_density_use_view_c: PK_facet_density_t = 20541;
/// Increase density where normals nearly parallel to view.
pub const PK_facet_density_parallel_c: PK_facet_density_t = 20542;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_facet_density_sil_and_par_c: PK_facet_density_t = 20543;

// =============================================================================
// Facet culling
// =============================================================================

pub type PK_facet_cull_t = c_int;

/// No culling (default).
pub const PK_facet_cull_none_c: PK_facet_cull_t = 20560;
/// Cull back-facing facets (requires view).
pub const PK_facet_cull_back_c: PK_facet_cull_t = 20561;

// =============================================================================
// Degeneracy handling
// =============================================================================

pub type PK_facet_degen_t = c_int;

/// Unique vertex per facet at degeneracy (default).
pub const PK_facet_degen_multiple_vxs_c: PK_facet_degen_t = 20580;
/// Single shared vertex (PK_TOPOL_facet_2 only).
pub const PK_facet_degen_single_vx_c: PK_facet_degen_t = 20581;
/// Unique vertex, averaged degenerate parameter.
pub const PK_facet_degen_average_parms_c: PK_facet_degen_t = 20582;

// =============================================================================
// Wire edges in faces
// =============================================================================

pub type PK_facet_wire_edges_t = c_int;

/// Ignore wire edges (default).
pub const PK_facet_wire_edges_no_c: PK_facet_wire_edges_t = 22140;
/// Respect wire edges (no facet crosses wire edge).
pub const PK_facet_wire_edges_yes_c: PK_facet_wire_edges_t = 22141;

// =============================================================================
// Ignoring small features
// =============================================================================

pub type PK_facet_ignore_t = c_int;

/// Facet all features (default).
pub const PK_facet_ignore_no_c: PK_facet_ignore_t = 22111;
/// Ignore features smaller than absolute value.
pub const PK_facet_ignore_absolute_c: PK_facet_ignore_t = 22112;
/// Ignore features smaller than ratio of overall box.
pub const PK_facet_ignore_ratio_c: PK_facet_ignore_t = 22113;
/// Ignore features smaller than ratio of owning body box.
pub const PK_facet_ignore_body_ratio_c: PK_facet_ignore_t = 22114;

// =============================================================================
// Ignore scope for loops
// =============================================================================

pub type PK_facet_ignore_scope_t = c_int;

/// Consider whole owning body (default).
pub const PK_facet_ignore_scope_global_c: PK_facet_ignore_scope_t = 22131;
/// Consider face as separate entity.
pub const PK_facet_ignore_scope_local_c: PK_facet_ignore_scope_t = 22130;

// =============================================================================
// Incremental faceting
// =============================================================================

pub type PK_facet_incr_t = c_int;

/// Off (default).
pub const PK_facet_incr_no_c: PK_facet_incr_t = 22560;
/// Off + delete existing incremental data.
pub const PK_facet_incr_clear_c: PK_facet_incr_t = 22561;
/// On + reuse existing data.
pub const PK_facet_incr_yes_c: PK_facet_incr_t = 22563;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_facet_incr_refresh_c: PK_facet_incr_t = 22562;

// =============================================================================
// Incremental faceting method
// =============================================================================

pub type PK_facet_incr_method_t = c_int;

/// Store data in system attributes (default).
pub const PK_facet_incr_method_attrib_c: PK_facet_incr_method_t = 25600;
/// Parasolid auto-manages incremental data.
pub const PK_facet_incr_method_auto_c: PK_facet_incr_method_t = 25601;

/// Incremental faceting propagation / refinement / report controls (enum tokens
/// not yet probed — stubbed as `c_int` for the `PK_TOPOL_facet_mesh_2_o_t`
/// layout).
pub type PK_facet_incr_prop_t = c_int;
pub type PK_facet_incr_refine_t = c_int;
pub type PK_facet_incr_report_t = c_int;

// =============================================================================
// Incremental faceting transformation handling
// =============================================================================

pub type PK_facet_incr_tf_t = c_int;

/// Refacet transformed bodies (default).
pub const PK_facet_incr_tf_no_c: PK_facet_incr_tf_t = 26610;
/// Skip rigid-transformed bodies.
pub const PK_facet_incr_tf_rigid_c: PK_facet_incr_tf_t = 26611;
/// Skip rigid+reflection-transformed bodies.
pub const PK_facet_incr_tf_reflection_c: PK_facet_incr_tf_t = 26612;

// =============================================================================
// Faceting around inflection points
// =============================================================================

pub type PK_facet_inflect_t = c_int;

/// No special treatment (default).
pub const PK_facet_inflect_no_c: PK_facet_inflect_t = 22800;
/// Split facets near inflection points.
pub const PK_facet_inflect_split_near_c: PK_facet_inflect_t = 22801;

// =============================================================================
// Facet quality
// =============================================================================

pub type PK_facet_quality_t = c_int;

/// No extra checks (default).
pub const PK_facet_quality_standard_c: PK_facet_quality_t = 23050;
/// Extra quality checks (slower).
pub const PK_facet_quality_improved_c: PK_facet_quality_t = 23051;

// =============================================================================
// Facet topology at singularities
// =============================================================================

pub type PK_facet_sing_topol_t = c_int;

/// Single shared vertex at singularity (default).
pub const PK_facet_sing_topol_default_c: PK_facet_sing_topol_t = 23360;
/// Degenerate facets separate adjacent facets at singularity.
pub const PK_facet_sing_topol_degen_c: PK_facet_sing_topol_t = 23361;

// =============================================================================
// Offset face handling
// =============================================================================

pub type PK_facet_respect_t = c_int;

/// Ignore offset relationships (default).
pub const PK_facet_respect_no_c: PK_facet_respect_t = 25400;
/// Reduce facet clashing on offset faces.
pub const PK_facet_respect_yes_c: PK_facet_respect_t = 25401;

// =============================================================================
// GO output control enums (PK_TOPOL_render_facet)
// =============================================================================

pub type PK_facet_go_normals_t = c_int;

/// Do not output surface normals (default).
pub const PK_facet_go_normals_no_c: PK_facet_go_normals_t = 20600;
/// Output surface normals at facet vertices.
pub const PK_facet_go_normals_yes_c: PK_facet_go_normals_t = 20601;

pub type PK_facet_go_parameters_t = c_int;

/// Do not output parameters (default).
pub const PK_facet_go_parameters_no_c: PK_facet_go_parameters_t = 20620;
/// Output surface parameters at vertices.
pub const PK_facet_go_parameters_d0_c: PK_facet_go_parameters_t = 20621;
/// Output parameters + first derivatives.
pub const PK_facet_go_parameters_d1_c: PK_facet_go_parameters_t = 20622;
/// Output parameters + first and second derivatives.
pub const PK_facet_go_parameters_d2_c: PK_facet_go_parameters_t = 20623;

pub type PK_facet_go_curvatures_t = c_int;

/// Do not output curvatures (default).
pub const PK_facet_go_curvatures_no_c: PK_facet_go_curvatures_t = 26180;
/// Output principal directions and curvatures at vertices.
pub const PK_facet_go_curvatures_yes_c: PK_facet_go_curvatures_t = 26181;

pub type PK_facet_go_edges_t = c_int;

/// Do not output edge data (default).
pub const PK_facet_go_edges_no_c: PK_facet_go_edges_t = 20640;
/// Output edge entities at facet boundary edges.
pub const PK_facet_go_edges_yes_c: PK_facet_go_edges_t = 20641;

pub type PK_facet_go_strips_t = c_int;

/// Individual facet output (default).
pub const PK_facet_go_strips_no_c: PK_facet_go_strips_t = 20660;
/// Output as facet strips (always triangular).
pub const PK_facet_go_strips_yes_c: PK_facet_go_strips_t = 20661;

pub type PK_facet_go_interleaved_t = c_int;

/// Output body-by-body (default).
pub const PK_facet_go_interleaved_no_c: PK_facet_go_interleaved_t = 20680;
/// Interleave faces from different bodies (enables multi-thread GO).
pub const PK_facet_go_interleaved_yes_c: PK_facet_go_interleaved_t = 20681;

// =============================================================================
// Tabular output enums (PK_TOPOL_facet_2)
// =============================================================================

pub type PK_facet_smp_t = c_int;

/// No SMP (default).
pub const PK_facet_smp_no_c: PK_facet_smp_t = 24110;
/// Facet different bodies simultaneously.
pub const PK_facet_smp_body_c: PK_facet_smp_t = 24111;

pub type PK_facet_consistent_parms_t = c_int;

/// No consistent parameterization (default).
pub const PK_facet_consistent_parms_no_c: PK_facet_consistent_parms_t = 22510;
/// Consistent params using face UV box.
pub const PK_facet_consistent_parms_fa_c: PK_facet_consistent_parms_t = 22512;
/// Legacy: consistent within half-period.
pub const PK_facet_consistent_parms_su_c: PK_facet_consistent_parms_t = 22511;

pub type PK_facet_pt_report_t = c_int;

/// No report (default).
pub const PK_facet_pt_report_no_c: PK_facet_pt_report_t = 24570;
/// Report points off model edges.
pub const PK_facet_pt_report_off_eds_c: PK_facet_pt_report_t = 24571;
/// Report points off edges + internal points off faces.
pub const PK_facet_pt_report_off_tpl_c: PK_facet_pt_report_t = 24572;

/// Faceting error token type — used in error_object table.
pub type PK_facet_fault_t = c_int;

// =============================================================================
// Rendering enums (PK_TOPOL_render_line / PK_GEOM_render)
// =============================================================================

// --- Edge rendering ---

pub type PK_render_edge_t = c_int;

/// Render edges (default).
pub const PK_render_edge_yes_c: PK_render_edge_t = 20101;
/// Do not render edges.
pub const PK_render_edge_no_c: PK_render_edge_t = 20100;

// --- Silhouette rendering ---

pub type PK_render_silhouette_t = c_int;

/// No silhouettes (default).
pub const PK_render_silhouette_no_c: PK_render_silhouette_t = 20120;
/// Render silhouettes.
pub const PK_render_silhouette_yes_c: PK_render_silhouette_t = 20122;
/// Render silhouettes, detect near-circular as arcs (drafting, slow).
pub const PK_render_silhouette_arcs_c: PK_render_silhouette_t = 20121;

// --- Mesh normal field for silhouettes ---

pub type PK_MESH_normal_field_t = c_int;

/// Use mvertex normals (default, smoother).
pub const PK_MESH_normal_field_mvertex_c: PK_MESH_normal_field_t = 26260;
/// Use mfacet normals (follows facet boundaries).
pub const PK_MESH_normal_field_mfacet_c: PK_MESH_normal_field_t = 26261;

// --- Sharp mfins rendering ---

pub type PK_render_sharp_mfins_t = c_int;

/// Do not render sharp mfins (default).
pub const PK_render_sharp_mfins_no_c: PK_render_sharp_mfins_t = 20260;
/// Render sharp mfins.
pub const PK_render_sharp_mfins_yes_c: PK_render_sharp_mfins_t = 20261;

// --- Visibility ---

pub type PK_render_vis_t = c_int;

/// No visibility evaluation, all lines visible (default / wireframe).
pub const PK_render_vis_no_c: PK_render_vis_t = 20300;
/// Hidden lines not returned.
pub const PK_render_vis_hid_c: PK_render_vis_t = 20301;
/// Hidden lines returned, marked invisible.
pub const PK_render_vis_inv_c: PK_render_vis_t = 20302;
/// Hidden lines returned, distinguish edge-hidden vs face-hidden.
pub const PK_render_vis_inv_draft_c: PK_render_vis_t = 20304;
/// Enable `invisible`, `drafting`, `self_hidden` sub-options.
pub const PK_render_vis_extended_c: PK_render_vis_t = 20305;

// --- Extended visibility sub-options ---

pub type PK_render_invisible_t = c_int;

/// Do not output invisible lines (default).
pub const PK_render_invisible_no_c: PK_render_invisible_t = 20440;
/// Output invisible lines.
pub const PK_render_invisible_yes_c: PK_render_invisible_t = 20441;

pub type PK_render_drafting_t = c_int;

/// No distinction line-hidden vs face-hidden (default).
pub const PK_render_drafting_no_c: PK_render_drafting_t = 20450;
/// Distinguish line-hidden vs face-hidden.
pub const PK_render_drafting_yes_c: PK_render_drafting_t = 20451;

pub type PK_render_self_hidden_t = c_int;

/// No distinction self-hidden vs other invisible (default).
pub const PK_render_self_hidden_no_c: PK_render_self_hidden_t = 20460;
/// Distinguish self-hidden lines.
pub const PK_render_self_hidden_yes_c: PK_render_self_hidden_t = 20461;

// --- Smoothness ---

pub type PK_render_smooth_t = c_int;

/// Don't indicate smoothness (default).
pub const PK_render_smooth_no_c: PK_render_smooth_t = 20320;
/// Indicate smooth edges.
pub const PK_render_smooth_yes_c: PK_render_smooth_t = 20321;
/// Indicate smoothness + coincidence with other lines.
pub const PK_render_smooth_draft_c: PK_render_smooth_t = 20322;

// --- Internal edges ---

pub type PK_render_internal_t = c_int;

/// Don't indicate internal edges (default).
pub const PK_render_internal_no_c: PK_render_internal_t = 20340;
/// Indicate internal edges.
pub const PK_render_internal_yes_c: PK_render_internal_t = 20341;

// --- Sketching missing geometry ---

pub type PK_render_ske_missing_t = c_int;

/// Error on missing geometry (default).
pub const PK_render_ske_missing_no_c: PK_render_ske_missing_t = 23750;
/// Skip entities with missing geometry.
pub const PK_render_ske_missing_yes_c: PK_render_ske_missing_t = 23751;

// --- Ignore small features (render) ---

pub type PK_render_ignore_t = c_int;

/// No features ignored (default).
pub const PK_render_ignore_no_c: PK_render_ignore_t = 21820;
/// Absolute size threshold.
pub const PK_render_ignore_absolute_c: PK_render_ignore_t = 21821;
/// Ratio of feature box to model box.
pub const PK_render_ignore_ratio_c: PK_render_ignore_t = 21822;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_render_ignore_body_ratio_c: PK_render_ignore_t = 21823;

// --- Hierarchical output ---

pub type PK_render_hierarch_t = c_int;

/// No hierarchical output (default).
pub const PK_render_hierarch_no_c: PK_render_hierarch_t = 20380;
/// Visibility segments only (geometry from prior call).
pub const PK_render_hierarch_no_geom_c: PK_render_hierarch_t = 20381;
/// Geometry + visibility segments.
pub const PK_render_hierarch_yes_c: PK_render_hierarch_t = 20382;
/// Geometry + visibility segments + polyline parameterization.
pub const PK_render_hierarch_param_c: PK_render_hierarch_t = 20383;

// --- B-curve output format ---

pub type PK_render_bcurve_t = c_int;

/// Output B-curves as polylines (default).
pub const PK_render_bcurve_polyline_c: PK_render_bcurve_t = 20400;
/// Output B-curves in Bezier form.
pub const PK_render_bcurve_bezier_c: PK_render_bcurve_t = 20401;
/// Output B-curves in NURBS form.
pub const PK_render_bcurve_nurbs_c: PK_render_bcurve_t = 20402;

// --- Memory target ---

pub type PK_render_memory_target_t = c_int;

/// No memory limit (default).
pub const PK_render_memory_target_no_c: PK_render_memory_target_t = 22160;
/// Attempt to keep within memory_target_value bytes.
pub const PK_render_memory_target_yes_c: PK_render_memory_target_t = 22161;

// --- Report lines ---

pub type PK_render_report_line_t = c_int;

/// Report nothing (default).
pub const PK_render_report_line_no_c: PK_render_report_line_t = 25550;
/// Report failed line fits.
pub const PK_render_report_line_fail_c: PK_render_report_line_t = 25551;
/// Report loose-tolerance fits.
pub const PK_render_report_line_loose_c: PK_render_report_line_t = 25552;
/// Report both failed and loose.
pub const PK_render_report_line_all_c: PK_render_report_line_t = 25553;

// --- Boundary rendering (PK_GEOM_render) ---

pub type PK_render_boundary_t = c_int;

/// Render surface boundaries (default).
pub const PK_render_boundary_yes_c: PK_render_boundary_t = 20221;
/// Don't render surface boundaries.
pub const PK_render_boundary_no_c: PK_render_boundary_t = 20220;

// --- Parametric hatching ---

pub type PK_render_param_t = c_int;

/// No parametric hatching (default).
pub const PK_render_param_no_c: PK_render_param_t = 20180;
/// Hatch faces with hatch attribute.
pub const PK_render_param_attrib_c: PK_render_param_t = 20181;
// [re-abi] appended 3 missing member(s) from pk-enums.h
pub const PK_render_param_spaced_c: PK_render_param_t = 20182;
pub const PK_render_param_spaced_free_c: PK_render_param_t = 20183;
pub const PK_render_param_number_free_c: PK_render_param_t = 20184;

// --- Lattice rendering ---

pub type PK_render_lattice_t = c_int;

/// Rods as straight lines (default).
pub const PK_render_lattice_line_c: PK_render_lattice_t = 20270;
/// Balls as spheres, rods as cylinders/cones.
pub const PK_render_lattice_solid_c: PK_render_lattice_t = 20271;
/// Balls as spheres, rods as lines.
pub const PK_render_lattice_composite_c: PK_render_lattice_t = 20272;

// --- Planar hatching ---

pub type PK_render_planar_t = c_int;

/// No planar hatching (default).
pub const PK_render_planar_no_c: PK_render_planar_t = 20140;
/// Hatch planar faces with hatch attribute.
pub const PK_render_planar_attrib_c: PK_render_planar_t = 20141;
/// Hatch all planar faces, Parasolid picks start.
pub const PK_render_planar_free_c: PK_render_planar_t = 20142;
/// Hatch all planar faces with specified axis + location.
pub const PK_render_planar_yes_c: PK_render_planar_t = 20143;

// --- Radial hatching ---

pub type PK_render_radial_t = c_int;

/// No radial hatching (default).
pub const PK_render_radial_no_c: PK_render_radial_t = 20160;
/// Hatch radial faces with hatch attribute.
pub const PK_render_radial_attrib_c: PK_render_radial_t = 20161;
/// Hatch all radial faces, Parasolid picks start.
pub const PK_render_radial_free_c: PK_render_radial_t = 20163;
/// Hatch all radial faces with specified start values.
pub const PK_render_radial_yes_c: PK_render_radial_t = 20162;

// --- Unfixed blends ---

pub type PK_render_unfix_t = c_int;

/// No unfixed blend rendering (default).
pub const PK_render_unfix_no_c: PK_render_unfix_t = 20200;
/// Render using blend attribute values.
pub const PK_render_unfix_attrib_c: PK_render_unfix_t = 20201;
/// Render all unfixed blends.
pub const PK_render_unfix_yes_c: PK_render_unfix_t = 20202;

// --- Overlapping bodies ---

pub type PK_render_overlap_t = c_int;

/// Assume no overlap (default).
pub const PK_render_overlap_no_c: PK_render_overlap_t = 21871;
/// Detect overlaps, split lines, no new curves.
pub const PK_render_overlap_yes_c: PK_render_overlap_t = 21872;
/// Detect overlaps + generate intersection curves (all vs all).
pub const PK_render_overlap_intsec_all_c: PK_render_overlap_t = 21874;
/// Detect overlaps + generate intersection curves (pairwise).
pub const PK_render_overlap_intsec_pair_c: PK_render_overlap_t = 21875;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_render_overlap_intersect_c: PK_render_overlap_t = 21873;

// --- Transparency ---

pub type PK_render_transparent_t = c_int;

/// All bodies opaque (default).
pub const PK_render_transparent_no_c: PK_render_transparent_t = 20240;
/// Transparent if SDL/TYSA_TRANSPARENCY attribute with non-zero coefficient.
pub const PK_render_transparent_yes_c: PK_render_transparent_t = 20241;
/// Transparent by `transparent_indices` array.
pub const PK_render_transparent_index_c: PK_render_transparent_t = 20242;

pub type PK_render_transp_hid_t = c_int;

/// Transparent bodies opaque to themselves, transparent to others.
pub const PK_render_transp_hid_yes_c: PK_render_transp_hid_t = 20244;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_render_transp_hid_no_c: PK_render_transp_hid_t = 20243;

// --- Viewport ---

pub type PK_render_viewport_t = c_int;

/// No viewport (default).
pub const PK_render_viewport_no_c: PK_render_viewport_t = 20420;
/// Single viewport.
pub const PK_render_viewport_yes_c: PK_render_viewport_t = 20421;
/// Multiple viewports.
pub const PK_render_viewport_array_c: PK_render_viewport_t = 20422;

pub type PK_render_viewport_type_t = c_int;

/// 3D cuboid viewport (default).
pub const PK_render_viewport_type_3D_c: PK_render_viewport_type_t = 20480;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_render_viewport_type_2D_c: PK_render_viewport_type_t = 20481;

pub type PK_render_viewport_clip_t = c_int;

/// No clipping (default).
pub const PK_render_viewport_clip_no_c: PK_render_viewport_clip_t = 20470;
/// Clip rendering to viewport boundary.
pub const PK_render_viewport_clip_yes_c: PK_render_viewport_clip_t = 20471;

// --- Regional data ---

pub type PK_render_region_t = c_int;

/// No regional data (default).
pub const PK_render_region_no_c: PK_render_region_t = 20360;
/// Regional data for faces with SDL/TYSA_REGION attribute.
pub const PK_render_region_attrib_c: PK_render_region_t = 20361;
/// Regional data for all boundary lines.
pub const PK_render_region_yes_c: PK_render_region_t = 20362;

// =============================================================================
// Picking enums
// =============================================================================

pub type PK_BODY_pick_method_t = c_int;

/// Order by position along ray (default).
pub const PK_BODY_pick_axial_c: PK_BODY_pick_method_t = 20901;
/// Order by absolute distance from ray location.
pub const PK_BODY_pick_axial_location_c: PK_BODY_pick_method_t = 20903;
/// Order by radial distance from ray (edges/vertices only).
pub const PK_BODY_pick_radial_c: PK_BODY_pick_method_t = 20900;
/// Combined axial+radial (controlled by `ratio` option).
pub const PK_BODY_pick_ratio_c: PK_BODY_pick_method_t = 20902;

pub type PK_pick_approximate_t = c_int;

/// Use approximate curve representations (default).
pub const PK_pick_approximate_yes_c: PK_pick_approximate_t = 22181;
/// Use accurate geometry.
pub const PK_pick_approximate_no_c: PK_pick_approximate_t = 22180;

// =============================================================================
// Convergent modeling enums
// =============================================================================

pub type PK_facet_geometry_t = c_int;

/// Facet geometry disabled (default).
pub const PK_facet_geometry_no_c: PK_facet_geometry_t = 25830;
/// Facet geometry enabled.
pub const PK_facet_geometry_all_c: PK_facet_geometry_t = 25831;

pub type PK_receive_mixed_t = c_int;

/// Error on encountering mixed geometry (default).
pub const PK_receive_mixed_fail_c: PK_receive_mixed_t = 26490;
/// Allow receiving mixed parts.
pub const PK_receive_mixed_allow_c: PK_receive_mixed_t = 26492;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_receive_mixed_make_facet_c: PK_receive_mixed_t = 26491;

pub type PK_related_topols_t = c_int;

/// Return top-level component parts.
pub const PK_related_topols_top_c: PK_related_topols_t = 25880;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_related_topols_no_c: PK_related_topols_t = 25881;

/// Geometry category returned by PK_GEOM_ask_geom_category.
pub type PK_geom_category_t = c_int;
// [re-abi] appended 5 missing member(s) from pk-enums.h
pub const PK_GEOM_category_classic_c: PK_geom_category_t = 25870;
pub const PK_GEOM_category_facet_c: PK_geom_category_t = 25871;
pub const PK_GEOM_category_none_c: PK_geom_category_t = 25872;
pub const PK_GEOM_category_mixed_c: PK_geom_category_t = 25874;
pub const PK_GEOM_category_lattice_c: PK_geom_category_t = 25875;

// =============================================================================
// Facet body conversion enums
// =============================================================================

/// Only track laminar and wire edges.
pub const PK_track_edges_laminar_wire_c: PK_track_edges_t = 23982;

pub type PK_MFACET_map_t = c_int;

/// Do not map mfacets (default).
pub const PK_MFACET_map_no_c: PK_MFACET_map_t = 26441;
/// Map mfacets to source.
pub const PK_MFACET_map_yes_c: PK_MFACET_map_t = 26440;

pub type PK_MVERTEX_map_t = c_int;

/// Do not map mvertices (default).
pub const PK_MVERTEX_map_no_c: PK_MVERTEX_map_t = 26541;
/// Map mvertices to source.
pub const PK_MVERTEX_map_yes_c: PK_MVERTEX_map_t = 26540;

pub type PK_BODY_keep_as_facet_t = c_int;

/// Preserve facet geometry (default, legacy).
pub const PK_BODY_keep_as_facet_yes_c: PK_BODY_keep_as_facet_t = 26620;
/// Allow mixed geometry result.
pub const PK_BODY_keep_as_facet_no_c: PK_BODY_keep_as_facet_t = 26621;

// =============================================================================
// PSM import enums
// =============================================================================

pub type PK_MESH_create_t = c_int;

/// Create facets immediately (default).
pub const PK_MESH_create_now_c: PK_MESH_create_t = 26500;
/// Delay creation until needed by operation.
pub const PK_MESH_create_later_c: PK_MESH_create_t = 26501;

pub type PK_MESH_cb_status_t = c_int;

/// Stop reading (last block).
pub const PK_MESH_cb_status_stop_c: PK_MESH_cb_status_t = 100262;
/// Continue reading (more blocks follow).
pub const PK_MESH_cb_status_continue_c: PK_MESH_cb_status_t = 100260;

pub type PK_MESH_facet_type_t = c_int;

/// Triangle strip data block.
pub const PK_MESH_facet_type_strip_c: PK_MESH_facet_type_t = 100752;
/// Independent facet (vector) data block.
pub const PK_MESH_facet_type_vector_c: PK_MESH_facet_type_t = 100755;

/// Internal facet-type dispatch code written into the reader's descriptor for a
/// **vector** block, as read by `FUN_1813f30a0` (V37.01.243). The public
/// `PK_MESH_facet_type_vector_c` token (100755) is illustrative in the docs; the
/// actual descriptor the callback fills carries this small internal code. Other
/// codes (1..6) select strip/index/fan variants; only vector (6) is wired up.
pub const PK_MESH_facet_type_vector_internal_c: PK_MESH_facet_type_t = 6;

// =============================================================================
// Mesh defect enums
// =============================================================================

pub type PK_MESH_defect_t = c_int;

/// Mesh has disjoint components.
pub const PK_MESH_defect_disjoint_c: PK_MESH_defect_t = 26009;
/// Mesh has foldover.
pub const PK_MESH_defect_foldover_c: PK_MESH_defect_t = 26008;
/// Mfacet of zero area.
pub const PK_MESH_defect_flat_mfacet_c: PK_MESH_defect_t = 26005;
/// Mfacet with at least one mfin shorter than precision.
pub const PK_MESH_defect_degen_mfacet_c: PK_MESH_defect_t = 26004;
/// Laminar mfacets occupying same position.
pub const PK_MESH_defect_slit_c: PK_MESH_defect_t = 26007;
/// Mesh contains self-intersecting mfacets.
pub const PK_MESH_defect_self_int_c: PK_MESH_defect_t = 26006;
/// Mesh is non-manifold.
pub const PK_MESH_defect_non_manifold_c: PK_MESH_defect_t = 26002;
/// Mvertex has incorrect normal direction.
pub const PK_MESH_defect_mvertex_normal_c: PK_MESH_defect_t = 26003;
/// Mesh data structure is corrupt.
pub const PK_MESH_defect_corrupt_c: PK_MESH_defect_t = 26001;

pub type PK_check_mesh_t = c_int;

/// Do not perform mesh invalidity checks.
pub const PK_check_mesh_no_c: PK_check_mesh_t = 25940;
/// Basic checks (non-manifold, degenerate, flat, bad normals, disjoint-on-face, boundary matching).
pub const PK_check_mesh_basic_c: PK_check_mesh_t = 25941;
/// Full checks (basic + slits + self-intersection) (default).
pub const PK_check_mesh_yes_c: PK_check_mesh_t = 25942;

pub type PK_MESH_replace_normal_t = c_int;

/// Replace normals for specific mtopols.
pub const PK_MESH_replace_normal_mtopol_c: PK_MESH_replace_normal_t = 25891;
/// Replace all normals in mesh.
pub const PK_MESH_replace_normal_all_c: PK_MESH_replace_normal_t = 25890;

/// Normal type enum for PK_MESH_ask_normal_type.
pub type PK_MESH_normal_type_t = c_int;

// =============================================================================
// Non-aligned box standard form (for viewport)
// =============================================================================

// =============================================================================
// Local tolerance specification
// =============================================================================

/// Per-entity tolerance override for faceting (`local_tols` in
/// [`PK_TOPOL_facet_mesh_o_t`] and `PK_TOPOL_facet_mesh_2_o_t`): seven doubles,
/// 0x38 bytes.
///
/// Evidence (pskernel.dll c900fa3f...3073): both journal printers walk the
/// array with a 0x38-byte stride and print seven doubles at +0x00..+0x30 with no
/// logicals: PKU_journal_TOPOL_facet_mesh_o @0x1814cad10 (v1) and FUN_1814cb3c0
/// (mesh_2) (static-observed; parasolid-re artifacts/rs/20261021T031234Z-prs-render
/// and 20261021T031231Z-prs-mesh/raw/decomp/FUN_1814cb3c0.c). Live: a tight
/// local tolerance on one face changes only that face's facets (harness test
/// `facet_local_tolerances_layout`). Replaces an unverified 5 logical/double-pair
/// layout (80 bytes) that matched neither printer.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PK_facet_local_tolerances_t {
    pub curve_chord_tol: c_double,   // @0x00
    pub curve_chord_max: c_double,   // @0x08
    pub curve_chord_ang: c_double,   // @0x10
    pub surface_plane_tol: c_double, // @0x18
    pub surface_plane_ang: c_double, // @0x20
    pub min_facet_width: c_double,   // @0x28
    pub max_facet_width: c_double,   // @0x30
}

const _: () = {
    assert!(core::mem::size_of::<PK_facet_local_tolerances_t>() == 0x38);
    assert!(core::mem::offset_of!(PK_facet_local_tolerances_t, surface_plane_tol) == 0x18);
    assert!(core::mem::offset_of!(PK_facet_local_tolerances_t, max_facet_width) == 0x30);
};

// =============================================================================
// PK_TOPOL_render_facet options (GO facet output)
//
// Evidence (pskernel.dll SHA-256
// c900fa3f430fe67c2adb15a50f9604d9812ed81a02c5006b66689dac28263073, grade
// static-observed; parasolid-re artifacts/rs/20261021T031234Z-prs-render/notes.md §2):
// - PK_TOPOL_render_facet @0x18046f230 passes the options to the migration
//   FUN_18046d000. The version it switches on is `control.o_t_version`:
//   versions 1..19 are copied into a 0x1b0 buffer, version 20 is used as
//   given, anything else is PK_ERROR_o_t_version_unknown (5022). Its NULL
//   branch writes the defaults used by the `Default` impls below.
// - Names and offsets: the journal printer PKU_journal_TOPOL_facet_mesh_o
//   @0x1814cad10 (the `control` part) and the go_option prints in the wrapper
//   (0x18046f861: `split_strips` at +0x1a8, `consistent_parms` at +0x1ac).
// - The checkers take `options` (0x1814c9630) and `options + 0x188`
//   (0x18046cec0), so `go_option` starts at 0x188.
// =============================================================================

/// The `o_t_version` of `PK_TOPOL_render_facet_o_t` (both parts) on V37.01.243.
pub const PK_TOPOL_render_facet_o_t_version: c_int = 20;

/// Mesh generation options of `PK_TOPOL_render_facet`: the **version-20 user
/// struct (0x188 = 392 bytes)**. This is not the `PK_TOPOL_facet_2` mesh struct
/// (`PK_TOPOL_facet_mesh_2_o_t`): the field order and set differ.
///
/// Version: `o_t_version` 20 is the only version used unmigrated; it must be
/// stamped in `control.o_t_version` (see `PK_TOPOL_render_facet_o_t`).
///
/// `local_tols` points at an array the kernel reads with a 0x38-byte stride
/// (7 doubles: curve_chord_tol, curve_chord_max, curve_chord_ang,
/// surface_plane_tol, surface_plane_ang, min_facet_width, max_facet_width,
/// per the journal printer), which is `PK_facet_local_tolerances_t`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_mesh_o_t {
    pub o_t_version: c_int,
    pub shape: PK_facet_shape_t,
    pub match_: PK_facet_match_t,
    pub density: PK_facet_density_t,
    pub cull: PK_facet_cull_t,
    pub n_loops: c_int,
    pub loops: *const PK_LOOP_t,
    pub max_facet_sides: c_int,
    pub is_min_facet_width: PK_LOGICAL_t,
    pub min_facet_width: c_double,
    pub is_max_facet_width: PK_LOGICAL_t,
    pub max_facet_width: c_double,
    pub is_curve_chord_tol: PK_LOGICAL_t,
    pub curve_chord_tol: c_double,
    pub is_curve_chord_max: PK_LOGICAL_t,
    pub curve_chord_max: c_double,
    pub is_curve_chord_ang: PK_LOGICAL_t,
    pub curve_chord_ang: c_double,
    pub is_surface_plane_tol: PK_LOGICAL_t,
    pub surface_plane_tol: c_double,
    pub is_surface_plane_ang: PK_LOGICAL_t,
    pub surface_plane_ang: c_double,
    pub is_facet_plane_tol: PK_LOGICAL_t,
    pub facet_plane_tol: c_double,
    pub is_facet_plane_ang: PK_LOGICAL_t,
    pub facet_plane_ang: c_double,
    pub is_local_density_tol: PK_LOGICAL_t,
    pub local_density_tol: c_double,
    pub is_local_density_ang: PK_LOGICAL_t,
    pub local_density_ang: c_double,
    pub degen: PK_facet_degen_t,
    pub n_view_directions: c_int,
    pub view_directions: *const PK_VECTOR1_t,
    pub n_local_tols: c_int,
    /// Array of `n_local_tols` [`PK_facet_local_tolerances_t`] (0x38 each; a zero field is unset).
    pub local_tols: *const PK_facet_local_tolerances_t,
    pub n_topols_with_local_tols: c_int,
    pub topols_with_local_tols: *const PK_TOPOL_t,
    pub local_tols_for_topols: *const c_int,
    pub ignore: PK_facet_ignore_t,
    pub ignore_value: c_double,
    pub ignore_scope: PK_facet_ignore_scope_t,
    pub wire_edges: PK_facet_wire_edges_t,
    pub incremental_facetting: PK_facet_incr_t,
    pub incremental_method: PK_facet_incr_method_t,
    pub incremental_propagation: PK_facet_incr_prop_t,
    pub incremental_transformation: PK_facet_incr_tf_t,
    pub incremental_refinement: PK_facet_incr_refine_t,
    pub incremental_report: PK_facet_incr_report_t,
    pub inflect: PK_facet_inflect_t,
    pub quality: PK_facet_quality_t,
    pub vertices_on_planar: PK_LOGICAL_t,
    pub respect_offset: PK_facet_respect_t,
    pub n_bodies_with_scales: c_int,
    pub bodies_with_scales: *const PK_BODY_t,
    pub scale_factors: *const PK_scale_factor_t,
    pub output_scale_factor: PK_scale_factor_t,
    pub n_viewports: c_int,
    pub viewports: *const PK_NABOX_sf_t,
    /// Token of an enum not in the V35 catalog; default 27460 (0x6b44).
    pub facet_frames: c_int,
    pub is_lattice_tol: PK_LOGICAL_t,
    pub lattice_tol: c_double,
    pub is_boundary_guise: PK_LOGICAL_t,
    /// Token of an enum not in the V35 catalog; default 27110 (0x69e6).
    pub boundary_guise: c_int,
}

const _: () = {
    use std::mem::{offset_of, size_of};
    type M = PK_TOPOL_facet_mesh_o_t;
    assert!(size_of::<M>() == 0x188);
    assert!(offset_of!(M, cull) == 0x10);
    assert!(offset_of!(M, loops) == 0x18);
    assert!(offset_of!(M, max_facet_sides) == 0x20);
    assert!(offset_of!(M, is_min_facet_width) == 0x24);
    assert!(offset_of!(M, min_facet_width) == 0x28);
    assert!(offset_of!(M, curve_chord_tol) == 0x48);
    assert!(offset_of!(M, curve_chord_ang) == 0x68);
    assert!(offset_of!(M, surface_plane_ang) == 0x88);
    assert!(offset_of!(M, local_density_ang) == 0xc8);
    assert!(offset_of!(M, degen) == 0xd0);
    assert!(offset_of!(M, view_directions) == 0xd8);
    assert!(offset_of!(M, local_tols) == 0xe8);
    assert!(offset_of!(M, local_tols_for_topols) == 0x100);
    assert!(offset_of!(M, ignore) == 0x108);
    assert!(offset_of!(M, ignore_value) == 0x110);
    assert!(offset_of!(M, ignore_scope) == 0x118);
    assert!(offset_of!(M, incremental_report) == 0x134);
    assert!(offset_of!(M, vertices_on_planar) == 0x140);
    assert!(offset_of!(M, respect_offset) == 0x144);
    assert!(offset_of!(M, bodies_with_scales) == 0x150);
    assert!(offset_of!(M, output_scale_factor) == 0x160);
    assert!(offset_of!(M, viewports) == 0x168);
    assert!(offset_of!(M, facet_frames) == 0x170);
    assert!(offset_of!(M, lattice_tol) == 0x178);
    assert!(offset_of!(M, is_boundary_guise) == 0x180);
    assert!(offset_of!(M, boundary_guise) == 0x184);
};

impl Default for PK_TOPOL_facet_mesh_o_t {
    /// The migration's NULL-branch defaults (FUN_18046d000), stamped at version 20.
    fn default() -> Self {
        // SAFETY: plain-data struct; null pointers, zero counts and
        // false/zero tolerances are the kernel's own defaults.
        let mut o: Self = unsafe { std::mem::zeroed() };
        o.o_t_version = PK_TOPOL_render_facet_o_t_version;
        o.shape = PK_facet_shape_convex_c;
        o.match_ = PK_facet_match_topol_c;
        o.density = PK_facet_density_no_view_c;
        o.cull = PK_facet_cull_none_c;
        o.max_facet_sides = 3;
        o.degen = PK_facet_degen_multiple_vxs_c;
        o.ignore = PK_facet_ignore_no_c;
        o.ignore_scope = PK_facet_ignore_scope_global_c;
        o.wire_edges = PK_facet_wire_edges_no_c;
        o.incremental_facetting = PK_facet_incr_no_c;
        o.incremental_method = PK_facet_incr_method_attrib_c;
        // Migration defaults; the token names are not recovered.
        o.incremental_propagation = 26560;
        o.incremental_transformation = PK_facet_incr_tf_no_c;
        o.incremental_refinement = 22570;
        o.incremental_report = 22580;
        o.inflect = PK_facet_inflect_no_c;
        o.quality = PK_facet_quality_standard_c;
        o.respect_offset = PK_facet_respect_no_c;
        o.output_scale_factor = PK_scale_factor_1x_c;
        o.facet_frames = 27460;
        o.boundary_guise = 27110;
        o
    }
}

/// What `PK_TOPOL_render_facet` sends through GO: the **version-20 struct
/// (0x28 = 40 bytes)**, at byte 0x188 of `PK_TOPOL_render_facet_o_t`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_render_facet_go_o_t {
    pub o_t_version: c_int,
    pub go_normals: PK_facet_go_normals_t,
    pub go_parameters: PK_facet_go_parameters_t,
    pub go_curvatures: PK_facet_go_curvatures_t,
    pub go_edges: PK_facet_go_edges_t,
    pub go_strips: PK_facet_go_strips_t,
    pub go_max_facets_per_strip: c_int,
    pub go_interleaved: PK_facet_go_interleaved_t,
    pub split_strips: PK_facet_split_strip_t,
    pub consistent_parms: PK_facet_consistent_parms_t,
}

const _: () = {
    use std::mem::{offset_of, size_of};
    type G = PK_TOPOL_render_facet_go_o_t;
    assert!(size_of::<G>() == 0x28);
    assert!(offset_of!(G, go_normals) == 4);
    assert!(offset_of!(G, go_edges) == 0x10);
    assert!(offset_of!(G, go_strips) == 0x14);
    assert!(offset_of!(G, go_max_facets_per_strip) == 0x18);
    assert!(offset_of!(G, go_interleaved) == 0x1c);
    assert!(offset_of!(G, split_strips) == 0x20);
    assert!(offset_of!(G, consistent_parms) == 0x24);
};

impl Default for PK_TOPOL_render_facet_go_o_t {
    /// The migration's NULL-branch defaults (FUN_18046d000), stamped at version 20.
    fn default() -> Self {
        Self {
            o_t_version: PK_TOPOL_render_facet_o_t_version,
            go_normals: PK_facet_go_normals_no_c,
            go_parameters: PK_facet_go_parameters_no_c,
            go_curvatures: PK_facet_go_curvatures_no_c,
            go_edges: PK_facet_go_edges_no_c,
            go_strips: PK_facet_go_strips_no_c,
            go_max_facets_per_strip: 0,
            go_interleaved: PK_facet_go_interleaved_no_c,
            split_strips: PK_facet_split_strip_no_c,
            consistent_parms: PK_facet_consistent_parms_no_c,
        }
    }
}

/// Options of `PK_TOPOL_render_facet` (0x1b0 = 432 bytes). There is no outer
/// `o_t_version`: the kernel versions the whole struct by `control.o_t_version`
/// (20 on V37.01.243; 21 is PK_ERROR_o_t_version_unknown). Stamp
/// `go_option.o_t_version` with the same value.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PK_TOPOL_render_facet_o_t {
    pub control: PK_TOPOL_facet_mesh_o_t,
    pub go_option: PK_TOPOL_render_facet_go_o_t,
}

const _: () = {
    use std::mem::{offset_of, size_of};
    assert!(size_of::<PK_TOPOL_render_facet_o_t>() == 0x1b0);
    assert!(offset_of!(PK_TOPOL_render_facet_o_t, go_option) == 0x188);
};

// =============================================================================
// Facet mesh generation options (tabular: PK_TOPOL_facet_2)
// =============================================================================

/// The `o_t_version` of both `PK_TOPOL_facet_2` option sub-structs on V37.01.243.
pub const PK_TOPOL_facet_2_o_t_version: c_int = 26;

/// Mesh generation control options for `PK_TOPOL_facet_2`: the **version-26 user
/// struct (0x198 = 408 bytes)**, the current layout.
///
/// Evidence (pskernel.dll SHA-256
/// `c900fa3f430fe67c2adb15a50f9604d9812ed81a02c5006b66689dac28263073`):
/// - field names and order: the kernel's journal printer of this struct
///   `FUN_1814cb3c0`, which prints every field by name, including the six V37
///   fields absent from the V35 catalog (`output_scale_factor`, `facet_frames`,
///   `is_lattice_tol`, `lattice_tol`, `is_boundary_guise`, `boundary_guise`);
/// - offsets: the PK→FCT converter `FUN_1814cedb0` reads each field at the offset
///   asserted below; the option migration `FUN_180447e00` (NULL branch) writes the
///   defaults used by [`Default`] and sizes the struct;
/// - live: parasolid-re `port/P3-faceting` (179 fixtures, `fixtures/p3-*.json`)
///   ran this layout with every tolerance field and the `match` token live.
///
/// Fields that are tolerances are pairs `is_X` (1-byte `PK_LOGICAL_t`) + `X`
/// (double). An unset tolerance means "kernel default"; see
/// `parasolid::FacetOptions` for the defaulting rule.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_mesh_2_o_t {
    pub o_t_version: c_int,
    pub shape: PK_facet_shape_t,
    pub match_: PK_facet_match_t,
    pub density: PK_facet_density_t,
    pub n_view_directions: c_int,
    pub view_directions: *const PK_VECTOR1_t,
    pub cull: PK_facet_cull_t,
    pub n_cull_transfs: c_int,
    pub cull_transfs: *const PK_TRANSF_t,
    pub n_loops: c_int,
    pub loops: *const PK_LOOP_t,
    pub max_facet_sides: c_int,
    pub is_min_facet_width: PK_LOGICAL_t,
    pub min_facet_width: c_double,
    pub is_max_facet_width: PK_LOGICAL_t,
    pub max_facet_width: c_double,
    pub is_curve_chord_tol: PK_LOGICAL_t,
    pub curve_chord_tol: c_double,
    pub is_curve_chord_max: PK_LOGICAL_t,
    pub curve_chord_max: c_double,
    pub is_curve_chord_ang: PK_LOGICAL_t,
    pub curve_chord_ang: c_double,
    pub is_surface_plane_tol: PK_LOGICAL_t,
    pub surface_plane_tol: c_double,
    pub is_surface_plane_ang: PK_LOGICAL_t,
    pub surface_plane_ang: c_double,
    pub is_facet_plane_tol: PK_LOGICAL_t,
    pub facet_plane_tol: c_double,
    pub is_facet_plane_ang: PK_LOGICAL_t,
    pub facet_plane_ang: c_double,
    pub is_local_density_tol: PK_LOGICAL_t,
    pub local_density_tol: c_double,
    pub is_local_density_ang: PK_LOGICAL_t,
    pub local_density_ang: c_double,
    pub n_local_tols: c_int,
    pub local_tols: *const PK_facet_local_tolerances_t,
    pub n_topols_with_local_tols: c_int,
    pub topols_with_local_tols: *const PK_TOPOL_t,
    pub local_tols_for_topols: *const c_int,
    pub ignore: PK_facet_ignore_t,
    pub ignore_value: c_double,
    pub ignore_scope: PK_facet_ignore_scope_t,
    pub wire_edges: PK_facet_wire_edges_t,
    pub incremental_facetting: PK_facet_incr_t,
    pub incremental_method: PK_facet_incr_method_t,
    pub incremental_propagation: PK_facet_incr_prop_t,
    pub incremental_transformation: PK_facet_incr_tf_t,
    pub incremental_refinement: PK_facet_incr_refine_t,
    pub incremental_report: PK_facet_incr_report_t,
    pub inflect: PK_facet_inflect_t,
    pub quality: PK_facet_quality_t,
    pub vertices_on_planar: PK_LOGICAL_t,
    pub sing_topol: PK_facet_sing_topol_t,
    pub respect_offset: PK_facet_respect_t,
    pub n_bodies_with_scales: c_int,
    pub bodies_with_scales: *const PK_BODY_t,
    pub scale_factors: *const PK_scale_factor_t,
    /// `PK_scale_factor_t` token; default `PK_scale_factor_1x_c`.
    pub output_scale_factor: PK_scale_factor_t,
    pub n_viewports: c_int,
    pub viewports: *const PK_NABOX_sf_t,
    /// Token of an enum not in the V35 catalog; default 27460 (0x6b44, the
    /// migration's NULL default). `[unknown]` name.
    pub facet_frames: c_int,
    pub is_lattice_tol: PK_LOGICAL_t,
    pub lattice_tol: c_double,
    pub is_boundary_guise: PK_LOGICAL_t,
    /// Token of an enum not in the V35 catalog; default 27110 (0x69e6).
    /// `[unknown]` name.
    pub boundary_guise: c_int,
}

impl Default for PK_TOPOL_facet_mesh_2_o_t {
    /// The kernel's NULL-options defaults (`FUN_180447e00`): every tolerance off,
    /// every count 0, every pointer NULL, and these tokens.
    fn default() -> Self {
        // SAFETY: all-zero is a valid bit pattern for this plain-data struct
        // (ints, doubles, 1-byte logicals, raw pointers); the tokens are set below.
        let mut o: Self = unsafe { std::mem::zeroed() };
        o.o_t_version = PK_TOPOL_facet_2_o_t_version;
        o.shape = PK_facet_shape_convex_c;
        o.match_ = PK_facet_match_topol_c;
        o.density = PK_facet_density_no_view_c;
        o.cull = PK_facet_cull_none_c;
        o.max_facet_sides = 3;
        o.ignore = PK_facet_ignore_no_c;
        o.ignore_scope = PK_facet_ignore_scope_global_c;
        o.wire_edges = PK_facet_wire_edges_no_c;
        o.incremental_facetting = PK_facet_incr_no_c;
        o.incremental_method = PK_facet_incr_method_attrib_c;
        // Migration defaults; the token names are not recovered.
        o.incremental_propagation = 26560;
        o.incremental_transformation = PK_facet_incr_tf_no_c;
        o.incremental_refinement = 22570;
        o.incremental_report = 22580;
        o.inflect = PK_facet_inflect_no_c;
        o.quality = PK_facet_quality_standard_c;
        o.sing_topol = PK_facet_sing_topol_default_c;
        o.respect_offset = PK_facet_respect_no_c;
        o.output_scale_factor = PK_scale_factor_1x_c;
        o.facet_frames = 27460;
        o.boundary_guise = 27110;
        o
    }
}

// =============================================================================
// Callback type for per-body-instance table return
// =============================================================================

/// Callback function type for receiving facet tables per body instance.
///
/// Called by PK_TOPOL_facet_2 when `facet_tables_cb` is set. `[unprobed]`:
/// the argument list is from the header docs and was never exercised.
pub type PK_TOPOL_facet_tables_cb_t = Option<
    unsafe extern "C" fn(
        topol: PK_TOPOL_t,
        transform: PK_TRANSF_t,
        tables: *const PK_TOPOL_facet_table_t,
        context: *mut c_void,
    ),
>;

// =============================================================================
// Tabular output choice options for PK_TOPOL_facet_2
// =============================================================================

/// `split_strips` token type (was wrongly a logical; type name from the RE
/// catalog). The value 22500 is the migration default `[probed]`; the constant
/// names and 22501 are `[guess]` (family numbering).
pub type PK_facet_split_strip_t = c_int;
pub const PK_facet_split_strip_no_c: PK_facet_split_strip_t = 22500;
pub const PK_facet_split_strip_yes_c: PK_facet_split_strip_t = 22501;

/// Controls which tables `PK_TOPOL_facet_2` returns: the **version-26 user
/// struct (0x50 = 80 bytes)**.
///
/// Evidence (pskernel.dll SHA-256 `c900fa3f…63073`): field names and offsets from
/// the kernel's journal printer `FUN_180446140` (it prints each field by name);
/// the validator `FUN_180445dc0` checks the tokens at +8/+12/+16/+44 and the 25
/// table bytes +0x30..+0x48; `FUN_180457f70` converts those bytes; the migration
/// `FUN_180447e00` (NULL branch) writes the [`Default`] values. Live: requesting
/// one table byte at a time returns exactly the matching `PK_TOPOL_fctab_*_c`
/// table (parasolid-re `port/P3-faceting`, map runs in the item-m `notes.md`).
///
/// The V35 header order the old definition used is wrong in three places:
/// `split_strips` is a token, not a logical; `facet_topol` follows `facet_face`;
/// `strip_topol` follows `strip_face`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_choice_2_o_t {
    pub o_t_version: c_int,
    pub max_facets_per_strip: c_int,
    pub split_strips: PK_facet_split_strip_t,
    pub consistent_parms: PK_facet_consistent_parms_t,
    pub smp: PK_facet_smp_t,
    /// Callback receiving tables per body instance. NULL for monolithic return.
    pub facet_tables_cb: PK_TOPOL_facet_tables_cb_t,
    /// Context data for callback.
    pub facet_tables_context: PK_POINTER_t,
    /// Whether callback is thread-safe.
    pub thread_safe: PK_LOGICAL_t,
    pub report_pts_off_topol: PK_facet_pt_report_t,
    // --- Table-selection flags: PK_LOGICAL_true requests the table ---
    pub facet_fin: PK_LOGICAL_t,
    pub strip_boundary: PK_LOGICAL_t,
    pub strip_zigzag: PK_LOGICAL_t,
    pub fin_fin: PK_LOGICAL_t,
    pub fin_data: PK_LOGICAL_t,
    pub data_point_idx: PK_LOGICAL_t,
    pub data_normal_idx: PK_LOGICAL_t,
    pub data_param_idx: PK_LOGICAL_t,
    pub data_deriv_idx: PK_LOGICAL_t,
    pub data_curv_idx: PK_LOGICAL_t,
    pub point_vec: PK_LOGICAL_t,
    pub normal_vec: PK_LOGICAL_t,
    pub param_uv: PK_LOGICAL_t,
    pub deriv_dp: PK_LOGICAL_t,
    pub deriv_d2p: PK_LOGICAL_t,
    pub curv_dirs: PK_LOGICAL_t,
    pub facet_face: PK_LOGICAL_t,
    pub facet_topol: PK_LOGICAL_t,
    pub strip_face: PK_LOGICAL_t,
    pub strip_topol: PK_LOGICAL_t,
    pub fin_edge: PK_LOGICAL_t,
    pub point_topol: PK_LOGICAL_t,
    pub fin_topol: PK_LOGICAL_t,
    pub error_object: PK_LOGICAL_t,
    pub incr_faces: PK_LOGICAL_t,
}

impl Default for PK_TOPOL_facet_choice_2_o_t {
    /// The kernel's NULL-options defaults (`FUN_180447e00`): no tables requested.
    fn default() -> Self {
        // SAFETY: all-zero is valid for this plain-data struct (ints, 1-byte
        // logicals, a nullable fn pointer, a raw pointer); tokens are set below.
        let mut o: Self = unsafe { std::mem::zeroed() };
        o.o_t_version = PK_TOPOL_facet_2_o_t_version;
        o.max_facets_per_strip = 10000;
        o.split_strips = PK_facet_split_strip_no_c;
        o.consistent_parms = PK_facet_consistent_parms_no_c;
        o.smp = PK_facet_smp_no_c;
        o.report_pts_off_topol = PK_facet_pt_report_no_c;
        o
    }
}

// =============================================================================
// Top-level option struct for PK_TOPOL_facet_2
// =============================================================================

/// Top-level options for PK_TOPOL_facet_2 (0x1e8 bytes: mesh 0x198 + choice 0x50).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PK_TOPOL_facet_2_o_t {
    pub control: PK_TOPOL_facet_mesh_2_o_t,
    pub choice: PK_TOPOL_facet_choice_2_o_t,
}

// =============================================================================
// Facet table structures (tabular output)
// =============================================================================

/// Table type token of one returned table (`PK_TOPOL_facet_table_t::fctab`).
/// Values: `return_tables_2` @0x180458940 writes them, the journal printer
/// `PKU_journal_TOPOL_facet_table` @0x180446b60 names them.
pub type PK_TOPOL_fctab_t = c_int;
/// Lookup, records `{facet, fin}` (2 ints).
pub const PK_TOPOL_fctab_facet_fin_c: PK_TOPOL_fctab_t = 22450;
/// Lookup, records `{strip, fin}` (2 ints).
pub const PK_TOPOL_fctab_strip_boundary_c: PK_TOPOL_fctab_t = 22451;
/// Lookup, records `{strip, fin}` (2 ints).
pub const PK_TOPOL_fctab_strip_zigzag_c: PK_TOPOL_fctab_t = 22452;
/// Indexed by fin: co-fin index (1 int).
pub const PK_TOPOL_fctab_fin_fin_c: PK_TOPOL_fctab_t = 22453;
/// Indexed by fin: data index (1 int).
pub const PK_TOPOL_fctab_fin_data_c: PK_TOPOL_fctab_t = 22454;
/// Indexed by data: point index (1 int).
pub const PK_TOPOL_fctab_data_point_c: PK_TOPOL_fctab_t = 22455;
/// Indexed by data: normal index (1 int).
pub const PK_TOPOL_fctab_data_normal_c: PK_TOPOL_fctab_t = 22456;
/// Indexed by data: parameter index (1 int).
pub const PK_TOPOL_fctab_data_param_c: PK_TOPOL_fctab_t = 22457;
/// Indexed by data: derivative index (1 int).
pub const PK_TOPOL_fctab_data_deriv_c: PK_TOPOL_fctab_t = 22458;
/// Indexed by point: position (3 doubles).
pub const PK_TOPOL_fctab_point_vec_c: PK_TOPOL_fctab_t = 22459;
/// Indexed by normal index: unit normal (3 doubles).
pub const PK_TOPOL_fctab_normal_vec_c: PK_TOPOL_fctab_t = 22460;
/// Indexed by parameter index: `(u, v)` (2 doubles).
pub const PK_TOPOL_fctab_param_uv_c: PK_TOPOL_fctab_t = 22461;
/// Indexed by derivative index: `dp/du, dp/dv` (6 doubles).
pub const PK_TOPOL_fctab_deriv_dp_c: PK_TOPOL_fctab_t = 22462;
/// Indexed by derivative index: second derivatives (9 doubles).
pub const PK_TOPOL_fctab_deriv_d2p_c: PK_TOPOL_fctab_t = 22463;
/// Indexed by facet: face tag (1 int).
pub const PK_TOPOL_fctab_facet_face_c: PK_TOPOL_fctab_t = 22464;
/// Indexed by strip: face tag (1 int).
pub const PK_TOPOL_fctab_strip_face_c: PK_TOPOL_fctab_t = 22465;
/// Lookup, records `{fin, edge}` (2 ints) for fins on model edges.
pub const PK_TOPOL_fctab_fin_edge_c: PK_TOPOL_fctab_t = 22466;
/// Lookup, records `{error, object}` (2 ints).
pub const PK_TOPOL_fctab_error_object_c: PK_TOPOL_fctab_t = 22467;
/// Lookup, records `{point, topol}` (2 ints) for points on model edges/vertices.
pub const PK_TOPOL_fctab_point_topol_c: PK_TOPOL_fctab_t = 22468;
/// Lookup, records `{fin, topol}` (2 ints).
pub const PK_TOPOL_fctab_fin_topol_c: PK_TOPOL_fctab_t = 22469;
/// Indexed by data: curvature index (1 int).
pub const PK_TOPOL_fctab_data_curv_c: PK_TOPOL_fctab_t = 22470;
/// Indexed by curvature index: 2 directions + 2 curvatures (8 doubles).
pub const PK_TOPOL_fctab_curv_dirs_c: PK_TOPOL_fctab_t = 22471;
/// Lookup, records of 2 ints.
pub const PK_TOPOL_fctab_incr_faces_c: PK_TOPOL_fctab_t = 22472;
/// Indexed by facet: topology tag (1 int).
pub const PK_TOPOL_fctab_facet_topol_c: PK_TOPOL_fctab_t = 22473;
/// Indexed by strip: topology tag (1 int).
pub const PK_TOPOL_fctab_strip_topol_c: PK_TOPOL_fctab_t = 22474;

/// One returned table body: a kernel-owned array and its record count.
///
/// Layout `{data*, int length}` (16 bytes): `return_tables_2` @0x180458940 stores
/// the array pointer at +0 and the record count at +8, and the journal printer
/// reads them the same way. `length` counts records, not ints/doubles (a
/// `point_vec` record is 3 doubles, a `facet_fin` record 2 ints). The kernel's
/// per-table element names were not recovered, so `data` is untyped; the record
/// shapes are on the `PK_TOPOL_fctab_*_c` constants. Indices are 0-based.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_fctab_data_t {
    pub data: *mut c_void,
    pub length: c_int,
}

/// One returned table: `{fctab, table*}` (16 bytes), names from the journal
/// printer (`"fctab"`, `"table.<name>"`).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_table_t {
    pub fctab: PK_TOPOL_fctab_t,
    pub table: *mut PK_TOPOL_fctab_data_t,
}

/// Result of `PK_TOPOL_facet_2` (24 bytes). `tables` points to `n_tables`
/// entries, one per requested table, in kernel order (not request order). Free
/// with `PK_TOPOL_facet_2_r_f`.
///
/// The old definition embedded a fictional all-tables struct by value; the kernel
/// writes these five fields only (`return_tables_2` @0x180458940 zeroes 24 bytes,
/// then fills the counts and allocates `16 × n` bytes of entries). The header's
/// field names were not recovered; these are descriptive.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_2_r_t {
    pub n_facets: c_int,
    pub n_strips: c_int,
    pub n_fins: c_int,
    pub n_tables: c_int,
    pub tables: *mut PK_TOPOL_facet_table_t,
}

// =============================================================================
// PK_TOPOL_render_line option struct
// =============================================================================

/// The `o_t_version` of `PK_TOPOL_render_line_o_t` on V37.01.243.
pub const PK_TOPOL_render_line_o_t_version: c_int = 25;

/// Options for `PK_TOPOL_render_line`: the **version-25 user struct
/// (0x240 = 576 bytes)**, the current layout.
///
/// Evidence (pskernel.dll SHA-256
/// `c900fa3f430fe67c2adb15a50f9604d9812ed81a02c5006b66689dac28263073`, grade
/// static-observed; parasolid-re `artifacts/rs/20261021T031234Z-prs-render/notes.md` §1):
/// - PK_TOPOL_render_line @0x18047a790 passes `options` to the migration
///   FUN_180472d80: versions 1..24 are copied into a 0x240 buffer, version 25
///   is used as given, anything else is PK_ERROR_o_t_version_unknown (5022).
///   Its NULL branch writes the defaults used by [`Default`].
/// - Names and offsets: the journal printer candidate_pk_0x180472700 prints
///   every field below by name at the asserted offset.
/// - The checker FUN_1804718b0 reads the enum fields at the same offsets
///   (5014 field_of_wrong_type on a bad token) and tests the logicals as bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_render_line_o_t {
    pub o_t_version: c_int,
    pub edge: PK_render_edge_t,
    pub silhouette: PK_render_silhouette_t,
    pub sil_mesh_normal_field: PK_MESH_normal_field_t,
    pub sharp_mfins: PK_render_sharp_mfins_t,
    pub visibility: PK_render_vis_t,
    pub invisible: PK_render_invisible_t,
    pub drafting: PK_render_drafting_t,
    pub self_hidden: PK_render_self_hidden_t,
    pub is_curve_chord_tol: PK_LOGICAL_t,
    pub curve_chord_tol: c_double,
    pub is_curve_chord_max: PK_LOGICAL_t,
    pub curve_chord_max: c_double,
    pub is_curve_chord_ang: PK_LOGICAL_t,
    pub curve_chord_ang: c_double,
    pub smooth: PK_render_smooth_t,
    pub is_edge_smooth_tol: PK_LOGICAL_t,
    pub edge_smooth_tol: c_double,
    pub internal: PK_render_internal_t,
    pub ske_missing: PK_render_ske_missing_t,
    pub ignore: PK_render_ignore_t,
    pub ignore_value: c_double,
    pub hierarch: PK_render_hierarch_t,
    pub bcurve: PK_render_bcurve_t,
    pub memory_target: PK_render_memory_target_t,
    /// `size_t` (the journal prints it with the 8-byte integer printer).
    pub memory_target_value: usize,
    pub report_lines: PK_render_report_line_t,
    pub planar: PK_render_planar_t,
    pub planar_spacing: c_double,
    pub planar_axis: PK_AXIS1_sf_t,
    pub radial: PK_render_radial_t,
    pub radial_around: c_double,
    pub radial_along: c_double,
    pub radial_about: c_double,
    pub radial_around_start: c_double,
    pub radial_along_start: c_double,
    pub radial_about_start: c_double,
    pub param: PK_render_param_t,
    pub param_u: c_double,
    pub param_v: c_double,
    pub param_u_number: c_int,
    pub param_v_number: c_int,
    pub param_u_start: c_double,
    pub param_v_start: c_double,
    pub unfix: PK_render_unfix_t,
    pub unfix_spacing: c_double,
    pub overlap: PK_render_overlap_t,
    pub n_overlap_indices: c_int,
    pub overlap_indices1: *const c_int,
    pub n_overlap_indices2: c_int,
    pub overlap_indices2: *const c_int,
    pub transparent: PK_render_transparent_t,
    pub n_transparent_indices: c_int,
    pub transparent_indices: *const c_int,
    pub transparent_hid: PK_render_transp_hid_t,
    pub n_suppressed_indices: c_int,
    pub suppressed_indices: *const c_int,
    pub viewport: PK_render_viewport_t,
    pub viewport_type: PK_render_viewport_type_t,
    pub n_viewports: c_int,
    /// Array of `n_viewports` boxes, read with a 0x78-byte stride.
    pub viewports: *const PK_NABOX_sf_t,
    pub viewport_sf: PK_NABOX_sf_t,
    pub viewport_clipping: PK_render_viewport_clip_t,
    pub region: PK_render_region_t,
    pub n_bodies_with_scales: c_int,
    pub bodies_with_scales: *const PK_BODY_t,
    pub scale_factors: *const PK_scale_factor_t,
    pub output_scale_factor: PK_scale_factor_t,
}

/// The "unset" double the kernel's option defaults use (-1e36,
/// bits 0xc76812f9cf7920e3) for the hatch start positions.
pub const PK_render_unset_double: c_double = -1.0e36;

const _: () = {
    use std::mem::{offset_of, size_of};
    type L = PK_TOPOL_render_line_o_t;
    assert!(size_of::<PK_AXIS1_sf_t>() == 48);
    assert!(size_of::<PK_NABOX_sf_t>() == 0x78);
    assert!(size_of::<L>() == 0x240);
    assert!(offset_of!(L, edge) == 4);
    assert!(offset_of!(L, silhouette) == 8);
    assert!(offset_of!(L, visibility) == 0x14);
    assert!(offset_of!(L, self_hidden) == 0x20);
    assert!(offset_of!(L, is_curve_chord_tol) == 0x24);
    assert!(offset_of!(L, curve_chord_tol) == 0x28);
    assert!(offset_of!(L, curve_chord_ang) == 0x48);
    assert!(offset_of!(L, smooth) == 0x50);
    assert!(offset_of!(L, is_edge_smooth_tol) == 0x54);
    assert!(offset_of!(L, edge_smooth_tol) == 0x58);
    assert!(offset_of!(L, internal) == 0x60);
    assert!(offset_of!(L, ignore) == 0x68);
    assert!(offset_of!(L, ignore_value) == 0x70);
    assert!(offset_of!(L, hierarch) == 0x78);
    assert!(offset_of!(L, bcurve) == 0x7c);
    assert!(offset_of!(L, memory_target) == 0x80);
    assert!(offset_of!(L, memory_target_value) == 0x88);
    assert!(offset_of!(L, report_lines) == 0x90);
    assert!(offset_of!(L, planar) == 0x94);
    assert!(offset_of!(L, planar_spacing) == 0x98);
    assert!(offset_of!(L, planar_axis) == 0xa0);
    assert!(offset_of!(L, radial) == 0xd0);
    assert!(offset_of!(L, radial_around) == 0xd8);
    assert!(offset_of!(L, radial_about_start) == 0x100);
    assert!(offset_of!(L, param) == 0x108);
    assert!(offset_of!(L, param_u) == 0x110);
    assert!(offset_of!(L, param_u_number) == 0x120);
    assert!(offset_of!(L, param_v_number) == 0x124);
    assert!(offset_of!(L, param_u_start) == 0x128);
    assert!(offset_of!(L, param_v_start) == 0x130);
    assert!(offset_of!(L, unfix) == 0x138);
    assert!(offset_of!(L, unfix_spacing) == 0x140);
    assert!(offset_of!(L, overlap) == 0x148);
    assert!(offset_of!(L, n_overlap_indices) == 0x14c);
    assert!(offset_of!(L, overlap_indices1) == 0x150);
    assert!(offset_of!(L, n_overlap_indices2) == 0x158);
    assert!(offset_of!(L, overlap_indices2) == 0x160);
    assert!(offset_of!(L, transparent) == 0x168);
    assert!(offset_of!(L, transparent_indices) == 0x170);
    assert!(offset_of!(L, transparent_hid) == 0x178);
    assert!(offset_of!(L, n_suppressed_indices) == 0x17c);
    assert!(offset_of!(L, suppressed_indices) == 0x180);
    assert!(offset_of!(L, viewport) == 0x188);
    assert!(offset_of!(L, viewport_type) == 0x18c);
    assert!(offset_of!(L, n_viewports) == 0x190);
    assert!(offset_of!(L, viewports) == 0x198);
    assert!(offset_of!(L, viewport_sf) == 0x1a0);
    assert!(offset_of!(L, viewport_clipping) == 0x218);
    assert!(offset_of!(L, region) == 0x21c);
    assert!(offset_of!(L, n_bodies_with_scales) == 0x220);
    assert!(offset_of!(L, bodies_with_scales) == 0x228);
    assert!(offset_of!(L, scale_factors) == 0x230);
    assert!(offset_of!(L, output_scale_factor) == 0x238);
};

impl Default for PK_TOPOL_render_line_o_t {
    /// The migration's NULL-branch defaults (FUN_180472d80), stamped at
    /// version 25: edges only, no silhouettes, no hidden-line, no hatching.
    fn default() -> Self {
        // SAFETY: plain-data struct; null pointers, zero counts and
        // false/zero tolerances are the kernel's own defaults.
        let mut o: Self = unsafe { std::mem::zeroed() };
        o.o_t_version = PK_TOPOL_render_line_o_t_version;
        o.edge = PK_render_edge_yes_c;
        o.silhouette = PK_render_silhouette_no_c;
        o.sil_mesh_normal_field = PK_MESH_normal_field_mvertex_c;
        o.sharp_mfins = PK_render_sharp_mfins_no_c;
        o.visibility = PK_render_vis_no_c;
        o.invisible = PK_render_invisible_no_c;
        o.drafting = PK_render_drafting_no_c;
        o.self_hidden = PK_render_self_hidden_no_c;
        o.smooth = PK_render_smooth_no_c;
        o.internal = PK_render_internal_no_c;
        o.ske_missing = PK_render_ske_missing_no_c;
        o.ignore = PK_render_ignore_no_c;
        o.hierarch = PK_render_hierarch_no_c;
        o.bcurve = PK_render_bcurve_polyline_c;
        o.memory_target = PK_render_memory_target_no_c;
        o.report_lines = PK_render_report_line_no_c;
        o.planar = PK_render_planar_no_c;
        o.radial = PK_render_radial_no_c;
        o.radial_around_start = PK_render_unset_double;
        o.radial_along_start = PK_render_unset_double;
        o.radial_about_start = PK_render_unset_double;
        o.param = PK_render_param_no_c;
        o.param_u_start = PK_render_unset_double;
        o.param_v_start = PK_render_unset_double;
        o.unfix = PK_render_unfix_no_c;
        o.overlap = PK_render_overlap_no_c;
        o.transparent = PK_render_transparent_no_c;
        o.transparent_hid = PK_render_transp_hid_no_c;
        o.viewport = PK_render_viewport_no_c;
        o.viewport_type = PK_render_viewport_type_3D_c;
        o.viewport_clipping = PK_render_viewport_clip_no_c;
        o.region = PK_render_region_no_c;
        o.output_scale_factor = PK_scale_factor_1x_c;
        o
    }
}

// =============================================================================
// PK_GEOM_render option struct
// =============================================================================

/// The `o_t_version` of `PK_GEOM_render_o_t` on V37.01.243.
pub const PK_GEOM_render_o_t_version: c_int = 5;

/// The `sdf_control` part of `PK_GEOM_render_o_t` (0x68 = 104 bytes), as
/// printed by PKU_journal_render_sdf_control @0x1813388a0 (static-observed).
/// The enum tokens are V37-only and their names are not recovered.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_render_sdf_control_t {
    pub voxel_size: c_double,
    pub max_n_voxels: [c_int; 3],
    /// Default 27410 (0x6b12).
    pub axes_type: c_int,
    pub grid_axes: PK_AXIS2_sf_t,
    /// Default 27420 (0x6b1c).
    pub order: c_int,
    /// Default 27430 (0x6b26).
    pub position: c_int,
}

/// Options for `PK_GEOM_render`: the **version-5 user struct (0x110 = 272
/// bytes)**. The geometry transforms are an argument of `PK_GEOM_render`, not
/// part of this struct.
///
/// Evidence (same DLL, static-observed; notes.md §3): PK_GEOM_render
/// @0x180335220 passes `options` to the migration FUN_180334a80 (version 5 used
/// as given, 1..4 copied, else 5022; NULL branch = the defaults below); names
/// and offsets from the journal printer candidate_pk_0x1803347e0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_render_o_t {
    pub o_t_version: c_int,
    pub param: PK_render_param_t,
    pub param_u: c_double,
    pub param_v: c_double,
    pub param_u_number: c_int,
    pub param_v_number: c_int,
    pub boundary: PK_render_boundary_t,
    pub bcurve: PK_render_bcurve_t,
    /// Default 27400 (0x6b08); token name not recovered.
    pub volume_method: c_int,
    pub n_content_methods: c_int,
    pub content_methods: *const c_int,
    pub lattice_contents: *const c_int,
    pub lattice: PK_render_lattice_t,
    pub n_lattices_with_ijkboxes: c_int,
    pub lattices_with_ijkboxes: *const PK_LATTICE_t,
    /// Element type not recovered; leave null with a zero count.
    pub ijkboxes: *const c_void,
    pub sdf_control: PK_render_sdf_control_t,
    pub have_curve_chord_tol: PK_LOGICAL_t,
    pub curve_chord_tol: c_double,
    pub have_curve_chord_max: PK_LOGICAL_t,
    pub curve_chord_max: c_double,
    pub have_curve_chord_ang: PK_LOGICAL_t,
    pub curve_chord_ang: c_double,
    pub n_entities_with_scales: c_int,
    pub entities_with_scales: *const PK_ENTITY_t,
    pub scale_factors: *const PK_scale_factor_t,
    pub output_scale_factor: PK_scale_factor_t,
}

const _: () = {
    use std::mem::{offset_of, size_of};
    type S = PK_render_sdf_control_t;
    assert!(size_of::<S>() == 0x68);
    assert!(offset_of!(S, max_n_voxels) == 8);
    assert!(offset_of!(S, axes_type) == 0x14);
    assert!(offset_of!(S, grid_axes) == 0x18);
    assert!(offset_of!(S, order) == 0x60);
    assert!(offset_of!(S, position) == 0x64);
    type G = PK_GEOM_render_o_t;
    assert!(size_of::<G>() == 0x110);
    assert!(offset_of!(G, param) == 4);
    assert!(offset_of!(G, param_u) == 8);
    assert!(offset_of!(G, param_v) == 0x10);
    assert!(offset_of!(G, param_u_number) == 0x18);
    assert!(offset_of!(G, boundary) == 0x20);
    assert!(offset_of!(G, bcurve) == 0x24);
    assert!(offset_of!(G, volume_method) == 0x28);
    assert!(offset_of!(G, n_content_methods) == 0x2c);
    assert!(offset_of!(G, content_methods) == 0x30);
    assert!(offset_of!(G, lattice_contents) == 0x38);
    assert!(offset_of!(G, lattice) == 0x40);
    assert!(offset_of!(G, n_lattices_with_ijkboxes) == 0x44);
    assert!(offset_of!(G, lattices_with_ijkboxes) == 0x48);
    assert!(offset_of!(G, ijkboxes) == 0x50);
    assert!(offset_of!(G, sdf_control) == 0x58);
    assert!(offset_of!(G, have_curve_chord_tol) == 0xc0);
    assert!(offset_of!(G, curve_chord_tol) == 0xc8);
    assert!(offset_of!(G, have_curve_chord_max) == 0xd0);
    assert!(offset_of!(G, curve_chord_ang) == 0xe8);
    assert!(offset_of!(G, n_entities_with_scales) == 0xf0);
    assert!(offset_of!(G, entities_with_scales) == 0xf8);
    assert!(offset_of!(G, scale_factors) == 0x100);
    assert!(offset_of!(G, output_scale_factor) == 0x108);
};

impl Default for PK_GEOM_render_o_t {
    /// The migration's NULL-branch defaults (FUN_180334a80), stamped at version 5.
    fn default() -> Self {
        // SAFETY: plain-data struct; null pointers and zero counts are the
        // kernel's own defaults.
        let mut o: Self = unsafe { std::mem::zeroed() };
        o.o_t_version = PK_GEOM_render_o_t_version;
        o.param = PK_render_param_no_c;
        o.boundary = PK_render_boundary_yes_c;
        o.bcurve = PK_render_bcurve_polyline_c;
        o.volume_method = 27400;
        o.lattice = PK_render_lattice_line_c;
        o.sdf_control.max_n_voxels = [256; 3];
        o.sdf_control.axes_type = 27410;
        o.sdf_control.grid_axes.axis = [0.0, 0.0, 1.0];
        o.sdf_control.grid_axes.ref_direction = [1.0, 0.0, 0.0];
        o.sdf_control.order = 27420;
        o.sdf_control.position = 27430;
        o.output_scale_factor = PK_scale_factor_1x_c;
        o
    }
}

// =============================================================================
// Picking option/result structs
// =============================================================================

// `PK_BODY_pick_topols_o_t`: moved to `mesh_options.rs` (layout verified).

// `PK_BODY_pick_topols_r_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// Facet body conversion option structs
// =============================================================================

// `PK_BODY_make_facet_body_o_t`: moved to `mesh_options.rs` (layout verified).

// `PK_MESH_make_bodies_o_t`: moved to `mesh_options.rs` (layout verified).

/// Callback type for propagation from seed mtopol (PK_MTOPOL_make_meshes).
pub type PK_MTOPOL_select_cb_t =
    Option<unsafe extern "C" fn(mtopol: PK_MTOPOL_t, data: *mut c_void) -> PK_LOGICAL_t>;

/// Callback type for returning tracking information (PK_MTOPOL_make_meshes).
pub type PK_MTOPOL_map_cb_f_t = Option<
    unsafe extern "C" fn(
        n_new: c_int,
        new_mtopols: *const PK_MTOPOL_t,
        n_old: c_int,
        old_mtopols: *const PK_MTOPOL_t,
        data: *mut c_void,
    ),
>;

// `PK_MTOPOL_make_meshes_o_t`: moved to `mesh_options.rs` (layout verified).

// `PK_MESH_make_surf_trimmed_o_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// PSM import structs
// =============================================================================

/// Facet data block — index type. Facet indices in triples.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_facet_type_index_t {
    pub n_facet_indices: c_int,
    pub facet_indices: *const c_int,
    pub n_vertex_positions: c_int,
    pub vertex_positions: *const c_double,
    pub n_vertex_normals: c_int,
    pub vertex_normals: *const c_double,
    pub is_relative_index: PK_LOGICAL_t,
}

/// Facet data block — triangle strip.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_facet_strip_t {
    pub n_strip_indices: c_int,
    pub strip_indices: *const c_int,
    pub n_vertex_positions: c_int,
    pub vertex_positions: *const c_double,
    pub n_vertex_normals: c_int,
    pub vertex_normals: *const c_double,
    pub is_relative_index: PK_LOGICAL_t,
}

/// Facet data block — triangle fan.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_facet_fan_t {
    pub n_fan_indices: c_int,
    pub fan_indices: *const c_int,
    pub n_vertex_positions: c_int,
    pub vertex_positions: *const c_double,
    pub n_vertex_normals: c_int,
    pub vertex_normals: *const c_double,
    pub is_relative_index: PK_LOGICAL_t,
}

/// Facet data block — independent facet vectors (each triangle = 3 consecutive
/// vertex triples in `vertex_positions`).
///
/// **Decompile-corrected layout** (V37.01.243, `FUN_1813f30a0` vector branch,
/// internal facet-type code 6): the first field is read as the **facet count**
/// (`n_facets`) — the consumer loops `for f in 0..n_facets` and reads 3 vertices
/// each at stride `0x18` (3 f64). It is NOT `n_vertex_positions`; passing the
/// vertex count (3× too large) over-reads the buffer and the operation fails
/// with a deferred error surfaced as PK error 900. There is no `n_vertex_normals`
/// field: `vertex_normals` sits at offset 16 and, when non-null, is parallel to
/// `vertex_positions`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_facet_vector_t {
    /// Number of facets (triangles) in this block. `vertex_positions` must hold
    /// `3 * n_facets` triples (`9 * n_facets` doubles).
    pub n_facets: c_int,
    // @4 implicit padding
    pub vertex_positions: *const c_double, // @8
    pub vertex_normals: *const c_double,   // @16
}

/// Descriptor the facet reader fills in (the 2nd callback argument).
///
/// **Decompile-confirmed** (V37.01.243, `FUN_1813f30a0`): the invoker passes a
/// pointer to this 16-byte struct as arg2. The reader writes the internal
/// facet-type dispatch code at offset 0 (see
/// [`PK_MESH_facet_type_vector_internal_c`]) and a pointer to the type-specific
/// block (e.g. [`PK_MESH_facet_vector_t`]) at offset 8. The consumer then reads
/// `*(descriptor+0)` for the code and dereferences `*(descriptor+8)` as the block.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_facet_descriptor_t {
    pub facet_type: PK_MESH_facet_type_t, // @0  internal dispatch code (1..6)
    pub _pad_4: c_int,                    // @4
    pub facet_data: *mut c_void,          // @8  -> type-specific block struct
}

/// Callback type for facet reader (PK_MESH_create_from_facets).
///
/// Called repeatedly to supply facet data blocks. **Decompile-confirmed ABI**
/// (V37.01.243, `FUN_1813f30a0`): `(*reader)(context, descriptor, status)` —
/// THREE args (RCX=context, RDX=`descriptor*`, R8=`status*`); R9 is unused (a
/// 4-arg signature crashed on the NULL R9). The reader fills the
/// [`PK_MESH_facet_descriptor_t`] (facet-type code + block pointer) and writes
/// the continue/stop token through `status`. The **return value is ignored** by
/// the consumer — the docs' "returns the status" describes the `status` out-param,
/// not the C return. The facet-type written to the descriptor is the internal
/// dispatch code (e.g. 6 = vector), not the public `PK_MESH_facet_type_*_c` token.
pub type PK_MESH_facet_reader_t = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        descriptor: *mut PK_MESH_facet_descriptor_t,
        status: *mut PK_MESH_cb_status_t,
    ) -> PK_ERROR_code_t,
>;

/// Callback type for freeing facet data blocks.
pub type PK_MESH_facet_free_t =
    Option<unsafe extern "C" fn(context: *mut c_void, facet_data: *mut c_void)>;

/// Options for PK_MESH_create_from_facets.
///
/// Decompile-confirmed layout: `o_t_version`@0 (was missing → kernel read
/// `vertices_estimate` as the version and raised 5022), then vertices/facet
/// estimates, facet_free@16, create@24, have_box@28, box@32, thread_safe@80.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_create_from_facets_o_t {
    pub o_t_version: c_int,               // @0
    pub vertices_estimate: c_int,         // @4
    pub facet_estimate: c_int,            // @8
    pub facet_free: PK_MESH_facet_free_t, // @16
    pub create: PK_MESH_create_t,         // @24
    pub have_box: PK_LOGICAL_t,           // @28
    pub box_: PK_BOX_t,                   // @32
    pub thread_safe: PK_LOGICAL_t,        // @80
}

// =============================================================================
// Mesh checking/defect structs
// =============================================================================

// `PK_MESH_find_defects_o_t`: moved to `mesh_options.rs` (layout verified).

// `PK_MESH_fix_defects_o_t`: moved to `mesh_options.rs` (layout verified).

// `PK_MESH_defect_details_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// Mesh normal storage options
// =============================================================================

// `PK_MESH_store_normals_o_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// Mesh sharp feature options
// =============================================================================

// `PK_MESH_find_sharp_mvxs_o_t`: moved to `mesh_options.rs` (layout verified).

// `PK_MESH_find_sharp_mfins_o_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// Perimeter finding options
// =============================================================================

// `PK_MFACET_find_perimeters_o_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// FIN_find_mtopols options
// =============================================================================

// `PK_FIN_find_mtopols_o_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// Polyline standard form
// =============================================================================

/// Standard form for creating polylines (PK_PLINE_create).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PLINE_sf_t {
    pub n_points: c_int,
    pub points: *const c_double,
}

// =============================================================================
// Partition facet geometry enquiry
// =============================================================================

// `PK_PARTITION_ask_facet_geom_o_t`: moved to `mesh_options.rs` (layout verified).

/// Options for PK_TOPOL_categorise_geom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_categorise_geom_o_t {
    pub o_t_version: c_int,
    pub want_related_topols: PK_related_topols_t,
}

// =============================================================================
// Mesh imprint options
// =============================================================================

// `PK_MESH_imprint_vectors_o_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// Mesh fill holes options
// =============================================================================

// `PK_MESH_fill_holes_o_t`: moved to `mesh_options.rs` (layout verified).

// =============================================================================
// Extern declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    // =========================================================================
    // Faceting functions
    // =========================================================================

    /// Facet topology with tabular output.
    ///
    /// Returns connected facet topology (facets, strips, fins) and optional
    /// geometric data (points, normals, parameters, curvatures) in tables.
    pub fn PK_TOPOL_facet_2(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        topol_transfs: *mut PK_TRANSF_t,
        options: *mut PK_TOPOL_facet_2_o_t,
        tables: *mut PK_TOPOL_facet_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Free memory for returned facet tables from PK_TOPOL_facet_2.
    pub fn PK_TOPOL_facet_2_r_f(result: *mut PK_TOPOL_facet_2_r_t) -> PK_ERROR_code_t;

    /// Facet topology (legacy version).
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_TOPOL_facet(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        topol_transfs: *mut PK_TRANSF_t,
        view_transf: PK_TRANSF_t,
        options: *mut PK_TOPOL_facet_o_t,
        tables: *mut PK_TOPOL_facet_r_t,
    ) -> PK_ERROR_code_t;

    /// Free memory for returned facet tables from PK_TOPOL_facet.
    pub fn PK_TOPOL_facet_r_f(result: *mut c_void) -> PK_ERROR_code_t;

    /// Facet topology with output through GO (Graphical Output) interface.
    ///
    /// Outputs facets as closed polygons through registered GO callbacks.
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_TOPOL_render_facet(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        topol_transfs: *mut PK_TRANSF_t,
        view_transf: PK_TRANSF_t,
        options: *mut PK_TOPOL_render_facet_o_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Rendering functions
    // =========================================================================

    /// Render topology as lines (wireframe, silhouette, hidden-line).
    ///
    /// Outputs via GO interface. Supports view-independent edges,
    /// view-dependent silhouettes, and hidden-line removal.
    pub fn PK_TOPOL_render_line(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        topol_transfs: *mut PK_TRANSF_t,
        view_transf: PK_TRANSF_t,
        options: *mut PK_TOPOL_render_line_o_t,
    ) -> PK_ERROR_code_t;

    /// Render geometry (B-curves, B-surfaces, foreign geometry).
    ///
    /// Wire-mesh form output via GO interface.
    pub fn PK_GEOM_render(
        n_geoms: c_int,
        geoms: *mut PK_GEOM_t,
        transfs: *mut PK_TRANSF_t,
        options: *mut PK_GEOM_render_o_t,
    ) -> PK_ERROR_code_t;

    /// Render a line representation of geometry.
    pub fn PK_GEOM_render_line(
        n_geoms: c_int,
        geoms: *mut PK_GEOM_t,
        geom_transfs: *mut PK_TRANSF_t,
        options: *mut PK_GEOM_render_line_o_t,
    ) -> PK_ERROR_code_t;

    /// Render volume (hatching).
    pub fn PK_TOPOL_render_volume(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        n_transfs: c_int,
        transfs: *const PK_TRANSF_t,
        options: *const c_void,
        result: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_TOPOL_render_volume.
    pub fn PK_TOPOL_render_volume_r_f(result: *mut c_void) -> PK_ERROR_code_t;

    // =========================================================================
    // Picking functions
    // =========================================================================

    /// Select faces/edges/vertices from bodies via ray intersection/proximity.
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_BODY_pick_topols(
        n_bodies: c_int,
        bodies: *mut PK_PART_t,
        body_transfs: *mut PK_TRANSF_t,
        ray: *mut PK_AXIS1_sf_t,
        options: *mut PK_BODY_pick_topols_o_t,
        picked: *mut PK_BODY_pick_topols_r_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_BODY_pick_topols.
    pub fn PK_BODY_pick_topols_r_f(result: *mut PK_BODY_pick_topols_r_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Convergent modeling session functions
    // =========================================================================

    // =========================================================================
    // Partition/Part facet geometry enquiry
    // =========================================================================

    // =========================================================================
    // Facet body conversion functions
    // =========================================================================

    /// Convert classic body to facet body.
    ///
    /// Returns a copy with facet geometry; original is untouched.
    pub fn PK_BODY_make_facet_body(
        body: PK_BODY_t,
        transf: PK_TRANSF_t,
        options: *mut PK_BODY_make_facet_body_o_t,
        body_copy: *mut PK_BODY_t,
        tracking: *mut PK_TOPOL_track_r_t,
        redundant_topol: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Create facet body from mesh data.
    ///
    /// Analyzes mesh for disjoint components and laminar mfins.
    pub fn PK_MESH_make_bodies(
        mesh: PK_MESH_t,
        options: *const PK_MESH_make_bodies_o_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create meshes from collection of mesh topologies.
    pub fn PK_MTOPOL_make_meshes(
        n_mtopols: c_int,
        mtopols: *const PK_MTOPOL_t,
        options: *const PK_MTOPOL_make_meshes_o_t,
        n_meshes: *mut c_int,
        meshes: *mut *mut PK_MESH_t,
    ) -> PK_ERROR_code_t;

    /// Free tracking map result from PK_MTOPOL_make_meshes.
    pub fn PK_MTOPOL_map_r_f(result: *mut c_void) -> PK_ERROR_code_t;

    /// Convert mesh to single trimmed classic surface.
    ///
    /// Argument order per the V35 header and the wrapper @0x1803791e0 (it journals
    /// `*sense` before the trim data): `sense` comes before `trim_data`. The
    /// previous declaration had them swapped. `PK_SURF_trim_data_t` is not
    /// verified, so `trim_data` stays untyped.
    pub fn PK_MESH_make_surf_trimmed(
        mesh: PK_MESH_t,
        options: *const PK_MESH_make_surf_trimmed_o_t,
        surf: *mut PK_SURF_t,
        sense: *mut PK_LOGICAL_t,
        trim_data: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Create mixed body directly from existing classic body.
    pub fn PK_TOPOL_make_facet_topol(
        topol: PK_TOPOL_t,
        options: *const c_void,
        facet_topol: *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // PSM import (create mesh from foreign facet data)
    // =========================================================================

    /// Create Parasolid mesh (PSM format) from foreign facet data.
    ///
    /// Uses a callback-based reader to supply facet data blocks (index,
    /// strip, fan, or vector format).
    pub fn PK_MESH_create_from_facets(
        facet_reader: PK_MESH_facet_reader_t,
        context: *mut c_void,
        options: *const PK_MESH_create_from_facets_o_t,
        mesh: *mut PK_MESH_t,
    ) -> PK_ERROR_code_t;

    /// Fill holes in mesh.
    pub fn PK_MESH_fill_holes(
        mesh: PK_MESH_t,
        options: *mut PK_MESH_fill_holes_o_t,
        filled_mesh: *mut PK_MESH_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Mesh checking and defect repair
    // =========================================================================

    /// Examine mesh for defects.
    pub fn PK_MESH_find_defects(
        mesh: PK_MESH_t,
        options: *mut PK_MESH_find_defects_o_t,
        n_defects: *mut c_int,
        defects: *mut *mut PK_MESH_defect_details_t,
    ) -> PK_ERROR_code_t;

    /// Examine mesh, attempt to fix defects.
    ///
    /// Returns new repaired meshes and remaining (unfixed) defect details.
    pub fn PK_MESH_fix_defects(
        mesh: PK_MESH_t,
        options: *mut PK_MESH_fix_defects_o_t,
        result: *mut PK_MESH_fix_result_t,
        n_resultant_meshes: *mut c_int,
        resultant_meshes: *mut *mut PK_MESH_t,
        defect_array: *mut *mut PK_MESH_defect_array_t,
    ) -> PK_ERROR_code_t;

    /// Free defect array from PK_MESH_find_defects/PK_MESH_fix_defects.
    pub fn PK_MESH_defect_array_f(details: *mut PK_MESH_defect_details_t) -> PK_ERROR_code_t;

    /// Free defect details from PK_MESH_find_defects/PK_MESH_fix_defects.
    pub fn PK_MESH_defect_details_f(details: *mut PK_MESH_defect_details_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Mesh normal management
    // =========================================================================

    /// Query whether mesh has stored normals or uses dynamic calculation.
    pub fn PK_MESH_ask_normal_type(
        mesh: PK_MESH_t,
        options: *mut PK_MESH_ask_normal_type_o_t,
        normal_type: *mut PK_MESH_normal_type_t,
    ) -> PK_ERROR_code_t;

    /// Whether all mvertices share a single normal.
    pub fn PK_MESH_has_unique_normals(
        mesh: PK_MESH_t,
        options: *mut PK_MESH_has_unique_normals_o_t,
        has_unique_normals: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Store/modify normals on mesh.
    pub fn PK_MESH_store_normals(
        mesh: PK_MESH_t,
        options: *const PK_MESH_store_normals_o_t,
    ) -> PK_ERROR_code_t;

    /// Discard stored normals (revert to dynamic calculation).
    pub fn PK_MESH_discard_normals(
        mesh: PK_MESH_t,
        options: *mut PK_MESH_discard_normals_o_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Mesh enquiry
    // =========================================================================

    /// Number of mfacets in mesh.
    pub fn PK_MESH_ask_n_mfacets(mesh: PK_MESH_t, n_mfacets: *mut c_int) -> PK_ERROR_code_t;

    /// Number of mvertices in mesh.
    pub fn PK_MESH_ask_n_mvertices(mesh: PK_MESH_t, n_mvertices: *mut c_int) -> PK_ERROR_code_t;

    /// Evaluate UV-parameters; returns position, mfacet, and mfin/mvertex at position.
    pub fn PK_MESH_eval_with_mtopol(
        mesh: PK_MESH_t,
        uv: *const PK_UV_t,
        options: *mut PK_MESH_eval_with_mtopol_o_t,
        position: *mut PK_VECTOR_t,
        mfacet: *mut PK_MFACET_t,
        mtopol: *mut PK_MTOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Find laminar mfins in mesh.
    pub fn PK_MESH_find_laminar_mfins(
        mesh: PK_MESH_t,
        n_mfins: *mut c_int,
        mfins: *mut *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_MESH_find_laminar_mfins.
    pub fn PK_MESH_find_laminar_mfins_r_f(
        result: *mut PK_MESH_find_laminar_mfins_r_t,
    ) -> PK_ERROR_code_t;

    /// Find sharp mfins (non-laminar, adjacent mfacets don't share normal).
    pub fn PK_MESH_find_sharp_mfins(
        mesh: PK_MESH_t,
        options: *const PK_MESH_find_sharp_mfins_o_t,
        n_mfins: *mut c_int,
        mfins: *mut *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    /// Find sharp mvertices (those without unique normal).
    pub fn PK_MESH_find_sharp_mvxs(
        mesh: PK_MESH_t,
        options: *const PK_MESH_find_sharp_mvxs_o_t,
        n_mvxs: *mut c_int,
        mvxs: *mut *mut PK_MVERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Iterate over all mfacets in a mesh via callback.
    pub fn PK_MESH_do_for_all_mfacets(
        mesh: PK_MESH_t,
        callback: Option<unsafe extern "C" fn(PK_MFACET_t, *mut c_void)>,
        context: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Iterate over all mvertices in a mesh via callback.
    pub fn PK_MESH_do_for_all_mvertices(
        mesh: PK_MESH_t,
        callback: Option<unsafe extern "C" fn(PK_MVERTEX_t, *mut c_void)>,
        context: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Whether mesh data is loaded.
    pub fn PK_MESH_is_loaded(
        mesh: PK_MESH_t,
        options: *mut PK_MESH_is_loaded_o_t,
        results: *mut PK_MESH_is_loaded_r_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_MESH_is_loaded.
    pub fn PK_MESH_is_loaded_r_f(result: *mut c_void) -> PK_ERROR_code_t;

    /// Imprint array of vectors onto a mesh.
    pub fn PK_MESH_imprint_vectors(
        mesh: PK_MESH_t,
        n_vectors: c_int,
        vectors: *mut PK_VECTOR_t,
        options: *mut PK_MESH_imprint_vectors_o_t,
        resultant_mesh: *mut PK_MESH_t,
        mvertices: *mut PK_MVERTEX_t,
        results: *mut PK_MESH_imprint_vectors_r_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_MESH_imprint_vectors.
    pub fn PK_MESH_imprint_vectors_r_f(result: *mut PK_MESH_imprint_vectors_r_t)
    -> PK_ERROR_code_t;

    // =========================================================================
    // MFACET enquiry
    // =========================================================================

    /// Returns adjacent mfacet via specified mfin (zero for laminar).
    pub fn PK_MFACET_ask_mfacet_adjacent(
        mfacet: PK_MFACET_t,
        mfin_index: c_int,
        adjacent: *mut PK_MFACET_t,
    ) -> PK_ERROR_code_t;

    /// Returns mfin at given mfin_index.
    pub fn PK_MFACET_ask_mfin(
        mfacet: PK_MFACET_t,
        mfin_index: c_int,
        mfin: *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    /// Returns mvertices of mfacet.
    pub fn PK_MFACET_ask_mvertices(
        mfacet: PK_MFACET_t,
        mvertices: *mut [PK_MVERTEX_t; 3],
    ) -> PK_ERROR_code_t;

    /// Returns normals of mvertices of mfacet.
    pub fn PK_MFACET_ask_mvx_normals(
        mfacet: PK_MFACET_t,
        options: *mut PK_MFACET_ask_mvx_normals_o_t,
        mvx_normals: *mut PK_VECTOR1_t,
    ) -> PK_ERROR_code_t;

    /// Returns normal vector of mfacet.
    pub fn PK_MFACET_ask_normal(mfacet: PK_MFACET_t, normal: *mut PK_VECTOR_t) -> PK_ERROR_code_t;

    /// Returns positions of mvertices of mfacet.
    pub fn PK_MFACET_ask_positions(
        mfacet: PK_MFACET_t,
        positions: *mut [PK_VECTOR_t; 3],
    ) -> PK_ERROR_code_t;

    /// Finds mesh parameterisation of position on mfacet.
    pub fn PK_MFACET_parameterise_vec(
        mfacet: PK_MFACET_t,
        position: *const PK_VECTOR_t,
        options: *mut PK_MFACET_parameterise_vec_o_t,
        uv: *mut PK_UV_t,
    ) -> PK_ERROR_code_t;

    /// Find perimeters (mloops) around supplied set of mfacets.
    pub fn PK_MFACET_find_perimeters(
        n_mfacets: c_int,
        mfacets: *mut PK_MFACET_t,
        options: *mut PK_MFACET_find_perimeters_o_t,
        results: *mut PK_MFACET_find_perimeters_r_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_MFACET_find_perimeters.
    pub fn PK_MFACET_find_perimeters_r_f(
        result: *mut PK_MFACET_find_perimeters_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // MFIN enquiry
    // =========================================================================

    /// Returns mfacet containing the mfin.
    pub fn PK_MFIN_ask_mfacet(mfin: PK_MFIN_t, mfacet: *mut PK_MFACET_t) -> PK_ERROR_code_t;

    /// Returns coincident mfin in adjacent facet.
    pub fn PK_MFIN_ask_mfin_adjacent(mfin: PK_MFIN_t, adjacent: *mut PK_MFIN_t) -> PK_ERROR_code_t;

    /// Returns index of mfin in its mfacet.
    pub fn PK_MFIN_ask_mfin_index(mfin: PK_MFIN_t, index: *mut c_int) -> PK_ERROR_code_t;

    /// Returns mvertex pointed to by mfin.
    pub fn PK_MFIN_ask_mvertex(mfin: PK_MFIN_t, mvertex: *mut PK_MVERTEX_t) -> PK_ERROR_code_t;

    /// Returns normal at mvertex of given mfin (per-mfin normal if multiple exist).
    pub fn PK_MFIN_ask_mvx_normal(
        mfin: PK_MFIN_t,
        options: *mut PK_MFIN_ask_mvx_normal_o_t,
        mvx_normal: *mut PK_VECTOR1_t,
    ) -> PK_ERROR_code_t;

    /// Returns curvature at mvertex of given mfin.
    pub fn PK_MFIN_ask_mvx_curvature(
        mfin: PK_MFIN_t,
        options: *mut PK_MFIN_ask_mvx_curvature_o_t,
        mvx_normal: *mut PK_VECTOR1_t,
        principal_direction_1: *mut PK_VECTOR1_t,
        principal_direction_2: *mut PK_VECTOR1_t,
        principal_curvature_1: *mut c_double,
        principal_curvature_2: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Returns next mfin in mfacet.
    pub fn PK_MFIN_ask_next_in_mfacet(mfin: PK_MFIN_t, next: *mut PK_MFIN_t) -> PK_ERROR_code_t;

    /// Returns next mfin around mvertex.
    pub fn PK_MFIN_ask_next_of_mvertex(mfin: PK_MFIN_t, next: *mut PK_MFIN_t) -> PK_ERROR_code_t;

    /// Returns previous mfin in mfacet.
    pub fn PK_MFIN_ask_previous_in_mfacet(
        mfin: PK_MFIN_t,
        previous: *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    /// Returns previous mfin around mvertex.
    pub fn PK_MFIN_ask_previous_of_mvertex(
        mfin: PK_MFIN_t,
        previous: *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    /// Whether mfin is laminar.
    pub fn PK_MFIN_is_laminar(mfin: PK_MFIN_t, is_laminar: *mut PK_LOGICAL_t) -> PK_ERROR_code_t;

    /// Whether two mfins belong to same mfacet.
    pub fn PK_MFIN_is_same_mfacet(
        mfin1: PK_MFIN_t,
        mfin2: PK_MFIN_t,
        is_same: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Whether mfin is sharp.
    pub fn PK_MFIN_is_sharp(
        mfin: PK_MFIN_t,
        options: *mut PK_MFIN_is_sharp_o_t,
        is_sharp: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // MVERTEX enquiry
    // =========================================================================

    /// Returns all unique normals of mvertex.
    pub fn PK_MVERTEX_ask_normals(
        mvertex: PK_MVERTEX_t,
        options: *mut PK_MVERTEX_ask_normals_o_t,
        n_normals: *mut c_int,
        normals: *mut *mut PK_VECTOR1_t,
    ) -> PK_ERROR_code_t;

    /// Returns all mfacets using mvertex.
    pub fn PK_MVERTEX_ask_mfacets(
        mvertex: PK_MVERTEX_t,
        n_mfacets: *mut c_int,
        mfacets: *mut *mut PK_MFACET_t,
    ) -> PK_ERROR_code_t;

    /// Returns an mfin pointing to mvertex.
    pub fn PK_MVERTEX_ask_mfin(mvertex: PK_MVERTEX_t, mfin: *mut PK_MFIN_t) -> PK_ERROR_code_t;

    /// Returns ring of mvertices around mvertex.
    pub fn PK_MVERTEX_ask_mvertices_ring(
        mvertex: PK_MVERTEX_t,
        n_ring: *mut c_int,
        ring: *mut *mut PK_MVERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Returns position at mvertex.
    pub fn PK_MVERTEX_ask_position(
        mvertex: PK_MVERTEX_t,
        position: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Whether mvertex is on laminar boundary.
    pub fn PK_MVERTEX_is_laminar(
        mvertex: PK_MVERTEX_t,
        is_laminar: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Set positions of mvertices.
    pub fn PK_MVERTEX_set_positions(
        n_mvertices: c_int,
        mvertices: *mut PK_MVERTEX_t,
        positions: *mut PK_VECTOR_t,
        options: *mut PK_MVERTEX_set_positions_o_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // MTOPOL enquiry
    // =========================================================================

    /// Returns bounding box for mtopol.
    pub fn PK_MTOPOL_ask_box(mtopol: PK_MTOPOL_t, box_: *mut PK_BOX_t) -> PK_ERROR_code_t;

    /// Returns class of mesh topology element.
    pub fn PK_MTOPOL_ask_class(mtopol: PK_MTOPOL_t, class: *mut PK_CLASS_t) -> PK_ERROR_code_t;

    /// Whether given mtopol is valid.
    pub fn PK_MTOPOL_is(mtopol: PK_MTOPOL_t, is_valid: *mut PK_LOGICAL_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Fin to mesh topology
    // =========================================================================

    /// Returns mtopology of a given fin (mvertices and/or mfins along a fin).
    pub fn PK_FIN_find_mtopols(
        fin: PK_FIN_t,
        options: *mut PK_FIN_find_mtopols_o_t,
        n_mvertices: *mut c_int,
        mvertices: *mut *mut PK_MVERTEX_t,
        n_mfins: *mut c_int,
        mfins: *mut *mut PK_MFIN_t,
        sense: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Polyline creation
    // =========================================================================

    /// Create a polyline.
    pub fn PK_PLINE_create(sf: *const PK_PLINE_sf_t, pline: *mut PK_PLINE_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Face imprint
    // =========================================================================

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_MTOPOL_ask_box`.
    pub fn PK_MTOPOL_ask_box_r_f(results: *mut PK_BOX_t) -> PK_ERROR_code_t;

}

// =============================================================================
// Compile-time ABI layout checks for PK_TOPOL_facet_2 (version 26).
//
// Every offset below is the one the kernel's PK->FCT converter FUN_1814cedb0
// (mesh) and FUN_180457f70 / FUN_180445dc0 (choice) read, and the migration
// FUN_180447e00 writes (pskernel.dll c900fa3f…63073). A field edit that moves
// one fails to compile instead of corrupting the call.
// =============================================================================
const _: () = {
    use core::mem::{offset_of, size_of};
    type M = PK_TOPOL_facet_mesh_2_o_t;
    assert!(size_of::<M>() == 0x198);
    assert!(offset_of!(M, shape) == 0x4);
    assert!(offset_of!(M, match_) == 0x8);
    assert!(offset_of!(M, density) == 0xc);
    assert!(offset_of!(M, view_directions) == 0x18);
    assert!(offset_of!(M, cull) == 0x20);
    assert!(offset_of!(M, loops) == 0x38);
    assert!(offset_of!(M, max_facet_sides) == 0x40);
    assert!(offset_of!(M, is_min_facet_width) == 0x44);
    assert!(offset_of!(M, min_facet_width) == 0x48);
    assert!(offset_of!(M, is_max_facet_width) == 0x50);
    assert!(offset_of!(M, max_facet_width) == 0x58);
    assert!(offset_of!(M, is_curve_chord_tol) == 0x60);
    assert!(offset_of!(M, curve_chord_tol) == 0x68);
    assert!(offset_of!(M, is_curve_chord_max) == 0x70);
    assert!(offset_of!(M, curve_chord_max) == 0x78);
    assert!(offset_of!(M, is_curve_chord_ang) == 0x80);
    assert!(offset_of!(M, curve_chord_ang) == 0x88);
    assert!(offset_of!(M, is_surface_plane_tol) == 0x90);
    assert!(offset_of!(M, surface_plane_tol) == 0x98);
    assert!(offset_of!(M, is_surface_plane_ang) == 0xa0);
    assert!(offset_of!(M, surface_plane_ang) == 0xa8);
    assert!(offset_of!(M, is_facet_plane_tol) == 0xb0);
    assert!(offset_of!(M, facet_plane_ang) == 0xc8);
    assert!(offset_of!(M, local_density_ang) == 0xe8);
    assert!(offset_of!(M, local_tols) == 0xf8);
    assert!(offset_of!(M, local_tols_for_topols) == 0x110);
    assert!(offset_of!(M, ignore) == 0x118);
    assert!(offset_of!(M, ignore_value) == 0x120);
    assert!(offset_of!(M, ignore_scope) == 0x128);
    assert!(offset_of!(M, wire_edges) == 0x12c);
    assert!(offset_of!(M, incremental_facetting) == 0x130);
    assert!(offset_of!(M, incremental_report) == 0x144);
    assert!(offset_of!(M, inflect) == 0x148);
    assert!(offset_of!(M, quality) == 0x14c);
    assert!(offset_of!(M, vertices_on_planar) == 0x150);
    assert!(offset_of!(M, sing_topol) == 0x154);
    assert!(offset_of!(M, respect_offset) == 0x158);
    assert!(offset_of!(M, n_bodies_with_scales) == 0x15c);
    assert!(offset_of!(M, scale_factors) == 0x168);
    assert!(offset_of!(M, output_scale_factor) == 0x170);
    assert!(offset_of!(M, n_viewports) == 0x174);
    assert!(offset_of!(M, viewports) == 0x178);
    assert!(offset_of!(M, facet_frames) == 0x180);
    assert!(offset_of!(M, is_lattice_tol) == 0x184);
    assert!(offset_of!(M, lattice_tol) == 0x188);
    assert!(offset_of!(M, is_boundary_guise) == 0x190);
    assert!(offset_of!(M, boundary_guise) == 0x194);

    type C = PK_TOPOL_facet_choice_2_o_t;
    assert!(size_of::<C>() == 0x50);
    assert!(offset_of!(C, max_facets_per_strip) == 0x4);
    assert!(offset_of!(C, split_strips) == 0x8);
    assert!(offset_of!(C, consistent_parms) == 0xc);
    assert!(offset_of!(C, smp) == 0x10);
    assert!(offset_of!(C, facet_tables_cb) == 0x18);
    assert!(offset_of!(C, facet_tables_context) == 0x20);
    assert!(offset_of!(C, thread_safe) == 0x28);
    assert!(offset_of!(C, report_pts_off_topol) == 0x2c);
    assert!(offset_of!(C, facet_fin) == 0x30);
    assert!(offset_of!(C, fin_fin) == 0x33);
    assert!(offset_of!(C, point_vec) == 0x3a);
    assert!(offset_of!(C, facet_face) == 0x40);
    assert!(offset_of!(C, facet_topol) == 0x41);
    assert!(offset_of!(C, strip_topol) == 0x43);
    assert!(offset_of!(C, fin_edge) == 0x44);
    assert!(offset_of!(C, point_topol) == 0x45);
    assert!(offset_of!(C, error_object) == 0x47);
    assert!(offset_of!(C, incr_faces) == 0x48);

    assert!(size_of::<PK_TOPOL_facet_2_o_t>() == 0x1e8);
    assert!(offset_of!(PK_TOPOL_facet_2_o_t, choice) == 0x198);
    assert!(size_of::<PK_TOPOL_facet_2_r_t>() == 24);
    assert!(offset_of!(PK_TOPOL_facet_2_r_t, tables) == 16);
    assert!(size_of::<PK_TOPOL_facet_table_t>() == 16);
    assert!(offset_of!(PK_TOPOL_facet_table_t, table) == 8);
    assert!(size_of::<PK_TOPOL_fctab_data_t>() == 16);
    assert!(offset_of!(PK_TOPOL_fctab_data_t, length) == 8);
};
