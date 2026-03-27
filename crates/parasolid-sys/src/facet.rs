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
pub const PK_facet_shape_convex_c: PK_facet_shape_t = 0;
/// Concave angles permitted, no interior holes.
pub const PK_facet_shape_cut_c: PK_facet_shape_t = 1;
/// Concave angles and interior holes permitted.
pub const PK_facet_shape_any_c: PK_facet_shape_t = 2;

// =============================================================================
// Facet matching between adjacent face meshes
// =============================================================================

pub type PK_facet_match_t = c_int;

/// Topology matching -- single mesh, shared topology at boundaries (default).
pub const PK_facet_match_topol_c: PK_facet_match_t = 0;
/// Geometry matching -- boundaries meet exactly but topologically disjoint.
pub const PK_facet_match_geom_c: PK_facet_match_t = 1;
/// Trimmed -- no matching, gaps/overlaps within tolerance.
pub const PK_facet_match_trimmed_c: PK_facet_match_t = 2;

// =============================================================================
// Facet density (view-dependent)
// =============================================================================

pub type PK_facet_density_t = c_int;

/// Independent of views (default).
pub const PK_facet_density_no_view_c: PK_facet_density_t = 0;
/// Increase density at silhouettes.
pub const PK_facet_density_use_view_c: PK_facet_density_t = 1;
/// Increase density where normals nearly parallel to view.
pub const PK_facet_density_parallel_c: PK_facet_density_t = 2;
/// Both silhouettes and parallel.
pub const PK_facet_density_both_c: PK_facet_density_t = 3;

// =============================================================================
// Facet culling
// =============================================================================

pub type PK_facet_cull_t = c_int;

/// No culling (default).
pub const PK_facet_cull_none_c: PK_facet_cull_t = 0;
/// Cull back-facing facets (requires view).
pub const PK_facet_cull_back_c: PK_facet_cull_t = 1;

// =============================================================================
// Degeneracy handling
// =============================================================================

pub type PK_facet_degen_t = c_int;

/// Unique vertex per facet at degeneracy (default).
pub const PK_facet_degen_multiple_vxs_c: PK_facet_degen_t = 0;
/// Single shared vertex (PK_TOPOL_facet_2 only).
pub const PK_facet_degen_single_vx_c: PK_facet_degen_t = 1;
/// Unique vertex, averaged degenerate parameter.
pub const PK_facet_degen_average_parms_c: PK_facet_degen_t = 2;

// =============================================================================
// Wire edges in faces
// =============================================================================

pub type PK_facet_wire_edges_t = c_int;

/// Ignore wire edges (default).
pub const PK_facet_wire_edges_no_c: PK_facet_wire_edges_t = 0;
/// Respect wire edges (no facet crosses wire edge).
pub const PK_facet_wire_edges_yes_c: PK_facet_wire_edges_t = 1;

// =============================================================================
// Ignoring small features
// =============================================================================

pub type PK_facet_ignore_t = c_int;

/// Facet all features (default).
pub const PK_facet_ignore_no_c: PK_facet_ignore_t = 0;
/// Ignore features smaller than absolute value.
pub const PK_facet_ignore_absolute_c: PK_facet_ignore_t = 1;
/// Ignore features smaller than ratio of overall box.
pub const PK_facet_ignore_ratio_c: PK_facet_ignore_t = 2;
/// Ignore features smaller than ratio of owning body box.
pub const PK_facet_ignore_body_ratio_c: PK_facet_ignore_t = 3;

// =============================================================================
// Ignore scope for loops
// =============================================================================

pub type PK_facet_ignore_scope_t = c_int;

/// Consider whole owning body (default).
pub const PK_facet_ignore_scope_global_c: PK_facet_ignore_scope_t = 0;
/// Consider face as separate entity.
pub const PK_facet_ignore_scope_local_c: PK_facet_ignore_scope_t = 1;

// =============================================================================
// Incremental faceting
// =============================================================================

pub type PK_facet_incr_t = c_int;

/// Off (default).
pub const PK_facet_incr_no_c: PK_facet_incr_t = 0;
/// Off + delete existing incremental data.
pub const PK_facet_incr_clear_c: PK_facet_incr_t = 1;
/// On + delete existing data, generate new.
pub const PK_facet_incr_refresh_no_c: PK_facet_incr_t = 2;
/// On + reuse existing data.
pub const PK_facet_incr_yes_c: PK_facet_incr_t = 3;

// =============================================================================
// Incremental faceting method
// =============================================================================

pub type PK_facet_incr_method_t = c_int;

/// Store data in system attributes (default).
pub const PK_facet_incr_method_attrib_c: PK_facet_incr_method_t = 0;
/// Parasolid auto-manages incremental data.
pub const PK_facet_incr_method_auto_c: PK_facet_incr_method_t = 1;

// =============================================================================
// Incremental faceting transformation handling
// =============================================================================

pub type PK_facet_incr_tf_t = c_int;

/// Refacet transformed bodies (default).
pub const PK_facet_incr_tf_no_c: PK_facet_incr_tf_t = 0;
/// Skip rigid-transformed bodies.
pub const PK_facet_incr_tf_rigid_c: PK_facet_incr_tf_t = 1;
/// Skip rigid+reflection-transformed bodies.
pub const PK_facet_incr_tf_reflection_c: PK_facet_incr_tf_t = 2;

// =============================================================================
// Faceting around inflection points
// =============================================================================

pub type PK_facet_inflect_t = c_int;

/// No special treatment (default).
pub const PK_facet_inflect_no_c: PK_facet_inflect_t = 0;
/// Split facets near inflection points.
pub const PK_facet_inflect_split_near_c: PK_facet_inflect_t = 1;

// =============================================================================
// Facet quality
// =============================================================================

pub type PK_facet_quality_t = c_int;

/// No extra checks (default).
pub const PK_facet_quality_standard_c: PK_facet_quality_t = 0;
/// Extra quality checks (slower).
pub const PK_facet_quality_improved_c: PK_facet_quality_t = 1;

// =============================================================================
// Facet topology at singularities
// =============================================================================

pub type PK_facet_sing_topol_t = c_int;

/// Single shared vertex at singularity (default).
pub const PK_facet_sing_topol_default_c: PK_facet_sing_topol_t = 0;
/// Degenerate facets separate adjacent facets at singularity.
pub const PK_facet_sing_topol_degen_c: PK_facet_sing_topol_t = 1;

// =============================================================================
// Offset face handling
// =============================================================================

pub type PK_facet_respect_t = c_int;

/// Ignore offset relationships (default).
pub const PK_facet_respect_no_c: PK_facet_respect_t = 0;
/// Reduce facet clashing on offset faces.
pub const PK_facet_respect_yes_c: PK_facet_respect_t = 1;

// =============================================================================
// GO output control enums (PK_TOPOL_render_facet)
// =============================================================================

pub type PK_facet_go_normals_t = c_int;

/// Do not output surface normals (default).
pub const PK_facet_go_normals_no_c: PK_facet_go_normals_t = 0;
/// Output surface normals at facet vertices.
pub const PK_facet_go_normals_yes_c: PK_facet_go_normals_t = 1;

pub type PK_facet_go_parameters_t = c_int;

/// Do not output parameters (default).
pub const PK_facet_go_parameters_no_c: PK_facet_go_parameters_t = 0;
/// Output surface parameters at vertices.
pub const PK_facet_go_parameters_d0_c: PK_facet_go_parameters_t = 1;
/// Output parameters + first derivatives.
pub const PK_facet_go_parameters_d1_c: PK_facet_go_parameters_t = 2;
/// Output parameters + first and second derivatives.
pub const PK_facet_go_parameters_d2_c: PK_facet_go_parameters_t = 3;

pub type PK_facet_go_curvatures_t = c_int;

/// Do not output curvatures (default).
pub const PK_facet_go_curvatures_no_c: PK_facet_go_curvatures_t = 0;
/// Output principal directions and curvatures at vertices.
pub const PK_facet_go_curvatures_yes_c: PK_facet_go_curvatures_t = 1;

pub type PK_facet_go_edges_t = c_int;

/// Do not output edge data (default).
pub const PK_facet_go_edges_no_c: PK_facet_go_edges_t = 0;
/// Output edge entities at facet boundary edges.
pub const PK_facet_go_edges_yes_c: PK_facet_go_edges_t = 1;

pub type PK_facet_go_strips_t = c_int;

/// Individual facet output (default).
pub const PK_facet_go_strips_no_c: PK_facet_go_strips_t = 0;
/// Output as facet strips (always triangular).
pub const PK_facet_go_strips_yes_c: PK_facet_go_strips_t = 1;

pub type PK_facet_go_interleaved_t = c_int;

/// Output body-by-body (default).
pub const PK_facet_go_interleaved_no_c: PK_facet_go_interleaved_t = 0;
/// Interleave faces from different bodies (enables multi-thread GO).
pub const PK_facet_go_interleaved_yes_c: PK_facet_go_interleaved_t = 1;

// =============================================================================
// Tabular output enums (PK_TOPOL_facet_2)
// =============================================================================

pub type PK_facet_smp_t = c_int;

/// No SMP (default).
pub const PK_facet_smp_no_c: PK_facet_smp_t = 0;
/// Facet different bodies simultaneously.
pub const PK_facet_smp_body_c: PK_facet_smp_t = 1;

pub type PK_facet_consistent_parms_t = c_int;

/// No consistent parameterization (default).
pub const PK_facet_consistent_parms_no_c: PK_facet_consistent_parms_t = 0;
/// Consistent params using face UV box.
pub const PK_facet_consistent_parms_fa_c: PK_facet_consistent_parms_t = 1;
/// Legacy: consistent within half-period.
pub const PK_facet_consistent_parms_su_c: PK_facet_consistent_parms_t = 2;

pub type PK_facet_pt_report_t = c_int;

/// No report (default).
pub const PK_facet_pt_report_no_c: PK_facet_pt_report_t = 0;
/// Report points off model edges.
pub const PK_facet_pt_report_off_eds_c: PK_facet_pt_report_t = 1;
/// Report points off edges + internal points off faces.
pub const PK_facet_pt_report_off_tpl_c: PK_facet_pt_report_t = 2;

/// Faceting error token type — used in error_object table.
pub type PK_facet_fault_t = c_int;

// =============================================================================
// Rendering enums (PK_TOPOL_render_line / PK_GEOM_render)
// =============================================================================

// --- Edge rendering ---

pub type PK_render_edge_t = c_int;

/// Render edges (default).
pub const PK_render_edge_yes_c: PK_render_edge_t = 0;
/// Do not render edges.
pub const PK_render_edge_no_c: PK_render_edge_t = 1;

// --- Silhouette rendering ---

pub type PK_render_silhouette_t = c_int;

/// No silhouettes (default).
pub const PK_render_silhouette_no_c: PK_render_silhouette_t = 0;
/// Render silhouettes.
pub const PK_render_silhouette_yes_c: PK_render_silhouette_t = 1;
/// Render silhouettes, detect near-circular as arcs (drafting, slow).
pub const PK_render_silhouette_arcs_c: PK_render_silhouette_t = 2;

// --- Mesh normal field for silhouettes ---

pub type PK_MESH_normal_field_t = c_int;

/// Use mvertex normals (default, smoother).
pub const PK_MESH_normal_field_mvertex_c: PK_MESH_normal_field_t = 0;
/// Use mfacet normals (follows facet boundaries).
pub const PK_MESH_normal_field_mfacet_c: PK_MESH_normal_field_t = 1;

// --- Sharp mfins rendering ---

pub type PK_render_sharp_mfins_t = c_int;

/// Do not render sharp mfins (default).
pub const PK_render_sharp_mfins_no_c: PK_render_sharp_mfins_t = 0;
/// Render sharp mfins.
pub const PK_render_sharp_mfins_yes_c: PK_render_sharp_mfins_t = 1;

// --- Visibility ---

pub type PK_render_vis_t = c_int;

/// No visibility evaluation, all lines visible (default / wireframe).
pub const PK_render_vis_no_c: PK_render_vis_t = 0;
/// Hidden lines not returned.
pub const PK_render_vis_hid_c: PK_render_vis_t = 1;
/// Hidden lines returned, marked invisible.
pub const PK_render_vis_inv_c: PK_render_vis_t = 2;
/// Hidden lines returned, distinguish edge-hidden vs face-hidden.
pub const PK_render_vis_inv_draft_c: PK_render_vis_t = 3;
/// Enable `invisible`, `drafting`, `self_hidden` sub-options.
pub const PK_render_vis_extended_c: PK_render_vis_t = 4;

// --- Extended visibility sub-options ---

pub type PK_render_invisible_t = c_int;

/// Do not output invisible lines (default).
pub const PK_render_invisible_no_c: PK_render_invisible_t = 0;
/// Output invisible lines.
pub const PK_render_invisible_yes_c: PK_render_invisible_t = 1;

pub type PK_render_drafting_t = c_int;

/// No distinction line-hidden vs face-hidden (default).
pub const PK_render_drafting_no_c: PK_render_drafting_t = 0;
/// Distinguish line-hidden vs face-hidden.
pub const PK_render_drafting_yes_c: PK_render_drafting_t = 1;

pub type PK_render_self_hidden_t = c_int;

/// No distinction self-hidden vs other invisible (default).
pub const PK_render_self_hidden_no_c: PK_render_self_hidden_t = 0;
/// Distinguish self-hidden lines.
pub const PK_render_self_hidden_yes_c: PK_render_self_hidden_t = 1;

// --- Smoothness ---

pub type PK_render_smooth_t = c_int;

/// Don't indicate smoothness (default).
pub const PK_render_smooth_no_c: PK_render_smooth_t = 0;
/// Indicate smooth edges.
pub const PK_render_smooth_yes_c: PK_render_smooth_t = 1;
/// Indicate smoothness + coincidence with other lines.
pub const PK_render_smooth_draft_c: PK_render_smooth_t = 2;

// --- Internal edges ---

pub type PK_render_internal_t = c_int;

/// Don't indicate internal edges (default).
pub const PK_render_internal_no_c: PK_render_internal_t = 0;
/// Indicate internal edges.
pub const PK_render_internal_yes_c: PK_render_internal_t = 1;

// --- Sketching missing geometry ---

pub type PK_render_ske_missing_t = c_int;

/// Error on missing geometry (default).
pub const PK_render_ske_missing_no_c: PK_render_ske_missing_t = 0;
/// Skip entities with missing geometry.
pub const PK_render_ske_missing_yes_c: PK_render_ske_missing_t = 1;

// --- Ignore small features (render) ---

pub type PK_render_ignore_t = c_int;

/// No features ignored (default).
pub const PK_render_ignore_no_c: PK_render_ignore_t = 0;
/// Absolute size threshold.
pub const PK_render_ignore_absolute_c: PK_render_ignore_t = 1;
/// Ratio of feature box to model box.
pub const PK_render_ignore_ratio_c: PK_render_ignore_t = 2;

// --- Hierarchical output ---

pub type PK_render_hierarch_t = c_int;

/// No hierarchical output (default).
pub const PK_render_hierarch_no_c: PK_render_hierarch_t = 0;
/// Visibility segments only (geometry from prior call).
pub const PK_render_hierarch_no_geom_c: PK_render_hierarch_t = 1;
/// Geometry + visibility segments.
pub const PK_render_hierarch_yes_c: PK_render_hierarch_t = 2;
/// Geometry + visibility segments + polyline parameterization.
pub const PK_render_hierarch_param_c: PK_render_hierarch_t = 3;

// --- B-curve output format ---

pub type PK_render_bcurve_t = c_int;

/// Output B-curves as polylines (default).
pub const PK_render_bcurve_polyline_c: PK_render_bcurve_t = 0;
/// Output B-curves in Bezier form.
pub const PK_render_bcurve_bezier_c: PK_render_bcurve_t = 1;
/// Output B-curves in NURBS form.
pub const PK_render_bcurve_nurbs_c: PK_render_bcurve_t = 2;

// --- Memory target ---

pub type PK_render_memory_target_t = c_int;

/// No memory limit (default).
pub const PK_render_memory_target_no_c: PK_render_memory_target_t = 0;
/// Attempt to keep within memory_target_value bytes.
pub const PK_render_memory_target_yes_c: PK_render_memory_target_t = 1;

// --- Report lines ---

pub type PK_render_report_line_t = c_int;

/// Report nothing (default).
pub const PK_render_report_line_no_c: PK_render_report_line_t = 0;
/// Report failed line fits.
pub const PK_render_report_line_fail_c: PK_render_report_line_t = 1;
/// Report loose-tolerance fits.
pub const PK_render_report_line_loose_c: PK_render_report_line_t = 2;
/// Report both failed and loose.
pub const PK_render_report_line_all_c: PK_render_report_line_t = 3;

// --- Boundary rendering (PK_GEOM_render) ---

pub type PK_render_boundary_t = c_int;

/// Render surface boundaries (default).
pub const PK_render_boundary_yes_c: PK_render_boundary_t = 0;
/// Don't render surface boundaries.
pub const PK_render_boundary_no_c: PK_render_boundary_t = 1;

// --- Parametric hatching ---

pub type PK_render_param_t = c_int;

/// No parametric hatching (default).
pub const PK_render_param_no_c: PK_render_param_t = 0;
/// Hatch faces with hatch attribute.
pub const PK_render_param_attrib_c: PK_render_param_t = 1;
/// Hatch using `param_u` / `param_v` from options.
pub const PK_render_param_yes_c: PK_render_param_t = 2;
/// Hatch using `param_u` / `param_v` (Parasolid picks start).
pub const PK_render_param_free_c: PK_render_param_t = 3;

// --- Lattice rendering ---

pub type PK_render_lattice_t = c_int;

/// Rods as straight lines (default).
pub const PK_render_lattice_line_c: PK_render_lattice_t = 0;
/// Balls as spheres, rods as cylinders/cones.
pub const PK_render_lattice_solid_c: PK_render_lattice_t = 1;
/// Balls as spheres, rods as lines.
pub const PK_render_lattice_composite_c: PK_render_lattice_t = 2;

// --- Planar hatching ---

pub type PK_render_planar_t = c_int;

/// No planar hatching (default).
pub const PK_render_planar_no_c: PK_render_planar_t = 0;
/// Hatch planar faces with hatch attribute.
pub const PK_render_planar_attrib_c: PK_render_planar_t = 1;
/// Hatch all planar faces, Parasolid picks start.
pub const PK_render_planar_free_c: PK_render_planar_t = 2;
/// Hatch all planar faces with specified axis + location.
pub const PK_render_planar_yes_c: PK_render_planar_t = 3;

// --- Radial hatching ---

pub type PK_render_radial_t = c_int;

/// No radial hatching (default).
pub const PK_render_radial_no_c: PK_render_radial_t = 0;
/// Hatch radial faces with hatch attribute.
pub const PK_render_radial_attrib_c: PK_render_radial_t = 1;
/// Hatch all radial faces, Parasolid picks start.
pub const PK_render_radial_free_c: PK_render_radial_t = 2;
/// Hatch all radial faces with specified start values.
pub const PK_render_radial_yes_c: PK_render_radial_t = 3;

// --- Unfixed blends ---

pub type PK_render_unfix_t = c_int;

/// No unfixed blend rendering (default).
pub const PK_render_unfix_no_c: PK_render_unfix_t = 0;
/// Render using blend attribute values.
pub const PK_render_unfix_attrib_c: PK_render_unfix_t = 1;
/// Render all unfixed blends.
pub const PK_render_unfix_yes_c: PK_render_unfix_t = 2;

// --- Overlapping bodies ---

pub type PK_render_overlap_t = c_int;

/// Assume no overlap (default).
pub const PK_render_overlap_no_c: PK_render_overlap_t = 0;
/// Detect overlaps, split lines, no new curves.
pub const PK_render_overlap_yes_c: PK_render_overlap_t = 1;
/// Detect overlaps + generate intersection curves (all vs all).
pub const PK_render_overlap_intsec_all_c: PK_render_overlap_t = 2;
/// Detect overlaps + generate intersection curves (pairwise).
pub const PK_render_overlap_intsec_pair_c: PK_render_overlap_t = 3;

// --- Transparency ---

pub type PK_render_transparent_t = c_int;

/// All bodies opaque (default).
pub const PK_render_transparent_no_c: PK_render_transparent_t = 0;
/// Transparent if SDL/TYSA_TRANSPARENCY attribute with non-zero coefficient.
pub const PK_render_transparent_yes_c: PK_render_transparent_t = 1;
/// Transparent by `transparent_indices` array.
pub const PK_render_transparent_index_c: PK_render_transparent_t = 2;

pub type PK_render_transp_hid_t = c_int;

/// Transparent bodies opaque to themselves, transparent to others.
pub const PK_render_transp_hid_yes_c: PK_render_transp_hid_t = 1;

// --- Viewport ---

pub type PK_render_viewport_t = c_int;

/// No viewport (default).
pub const PK_render_viewport_no_c: PK_render_viewport_t = 0;
/// Single viewport.
pub const PK_render_viewport_yes_c: PK_render_viewport_t = 1;
/// Multiple viewports.
pub const PK_render_viewport_array_c: PK_render_viewport_t = 2;

pub type PK_render_viewport_type_t = c_int;

/// 3D cuboid viewport (default).
pub const PK_render_viewport_type_3D_c: PK_render_viewport_type_t = 0;

pub type PK_render_viewport_clip_t = c_int;

/// No clipping (default).
pub const PK_render_viewport_clip_no_c: PK_render_viewport_clip_t = 0;
/// Clip rendering to viewport boundary.
pub const PK_render_viewport_clip_yes_c: PK_render_viewport_clip_t = 1;

// --- Regional data ---

pub type PK_render_region_t = c_int;

/// No regional data (default).
pub const PK_render_region_no_c: PK_render_region_t = 0;
/// Regional data for faces with SDL/TYSA_REGION attribute.
pub const PK_render_region_attrib_c: PK_render_region_t = 1;
/// Regional data for all boundary lines.
pub const PK_render_region_yes_c: PK_render_region_t = 2;

// =============================================================================
// Picking enums
// =============================================================================

pub type PK_BODY_pick_method_t = c_int;

/// Order by position along ray (default).
pub const PK_BODY_pick_axial_c: PK_BODY_pick_method_t = 0;
/// Order by absolute distance from ray location.
pub const PK_BODY_pick_axial_location_c: PK_BODY_pick_method_t = 1;
/// Order by radial distance from ray (edges/vertices only).
pub const PK_BODY_pick_radial_c: PK_BODY_pick_method_t = 2;
/// Combined axial+radial (controlled by `ratio` option).
pub const PK_BODY_pick_ratio_c: PK_BODY_pick_method_t = 3;

pub type PK_pick_approximate_t = c_int;

/// Use approximate curve representations (default).
pub const PK_pick_approximate_yes_c: PK_pick_approximate_t = 0;
/// Use accurate geometry.
pub const PK_pick_approximate_no_c: PK_pick_approximate_t = 1;

// =============================================================================
// Convergent modeling enums
// =============================================================================

pub type PK_facet_geometry_t = c_int;

/// Facet geometry disabled (default).
pub const PK_facet_geometry_no_c: PK_facet_geometry_t = 0;
/// Facet geometry enabled.
pub const PK_facet_geometry_all_c: PK_facet_geometry_t = 1;

pub type PK_receive_mixed_t = c_int;

/// Error on encountering mixed geometry (default).
pub const PK_receive_mixed_fail_c: PK_receive_mixed_t = 0;
/// Allow receiving mixed parts.
pub const PK_receive_mixed_allow_c: PK_receive_mixed_t = 1;

pub type PK_related_topols_t = c_int;

/// Return top-level component parts.
pub const PK_related_topols_top_c: PK_related_topols_t = 0;

/// Geometry category returned by PK_GEOM_ask_geom_category.
pub type PK_geom_category_t = c_int;

// =============================================================================
// Facet body conversion enums
// =============================================================================

/// Only track laminar and wire edges.
pub const PK_track_edges_laminar_wire_c: PK_track_edges_t = 2;

pub type PK_MFACET_map_t = c_int;

/// Do not map mfacets (default).
pub const PK_MFACET_map_no_c: PK_MFACET_map_t = 0;
/// Map mfacets to source.
pub const PK_MFACET_map_yes_c: PK_MFACET_map_t = 1;

pub type PK_MVERTEX_map_t = c_int;

/// Do not map mvertices (default).
pub const PK_MVERTEX_map_no_c: PK_MVERTEX_map_t = 0;
/// Map mvertices to source.
pub const PK_MVERTEX_map_yes_c: PK_MVERTEX_map_t = 1;

pub type PK_BODY_keep_as_facet_t = c_int;

/// Preserve facet geometry (default, legacy).
pub const PK_BODY_keep_as_facet_yes_c: PK_BODY_keep_as_facet_t = 0;
/// Allow mixed geometry result.
pub const PK_BODY_keep_as_facet_no_c: PK_BODY_keep_as_facet_t = 1;

// =============================================================================
// PSM import enums
// =============================================================================

pub type PK_MESH_create_t = c_int;

/// Create facets immediately (default).
pub const PK_MESH_create_now_c: PK_MESH_create_t = 0;
/// Delay creation until needed by operation.
pub const PK_MESH_create_later_c: PK_MESH_create_t = 1;

pub type PK_MESH_cb_status_t = c_int;

/// Stop reading (last block).
pub const PK_MESH_cb_status_stop_c: PK_MESH_cb_status_t = 0;
/// Continue reading (more blocks follow).
pub const PK_MESH_cb_status_continue_c: PK_MESH_cb_status_t = 1;

pub type PK_MESH_facet_type_t = c_int;

/// Triangle strip data block.
pub const PK_MESH_facet_type_strip_c: PK_MESH_facet_type_t = 0;
/// Independent facet (vector) data block.
pub const PK_MESH_facet_type_vector_c: PK_MESH_facet_type_t = 1;

// =============================================================================
// Mesh defect enums
// =============================================================================

pub type PK_MESH_defect_t = c_int;

/// Mesh has disjoint components.
pub const PK_MESH_defect_disjoint_c: PK_MESH_defect_t = 0;
/// Mesh has foldover.
pub const PK_MESH_defect_foldover_c: PK_MESH_defect_t = 1;
/// Mfacet of zero area.
pub const PK_MESH_defect_flat_mfacet_c: PK_MESH_defect_t = 2;
/// Mfacet with at least one mfin shorter than precision.
pub const PK_MESH_defect_degen_mfacet_c: PK_MESH_defect_t = 3;
/// Laminar mfacets occupying same position.
pub const PK_MESH_defect_slit_c: PK_MESH_defect_t = 4;
/// Mesh contains self-intersecting mfacets.
pub const PK_MESH_defect_self_int_c: PK_MESH_defect_t = 5;
/// Mesh is non-manifold.
pub const PK_MESH_defect_non_manifold_c: PK_MESH_defect_t = 6;
/// Mvertex has incorrect normal direction.
pub const PK_MESH_defect_mvertex_normal_c: PK_MESH_defect_t = 7;
/// Mesh data structure is corrupt.
pub const PK_MESH_defect_corrupt_c: PK_MESH_defect_t = 8;

pub type PK_check_mesh_t = c_int;

/// Do not perform mesh invalidity checks.
pub const PK_check_mesh_no_c: PK_check_mesh_t = 0;
/// Basic checks (non-manifold, degenerate, flat, bad normals, disjoint-on-face, boundary matching).
pub const PK_check_mesh_basic_c: PK_check_mesh_t = 1;
/// Full checks (basic + slits + self-intersection) (default).
pub const PK_check_mesh_yes_c: PK_check_mesh_t = 2;

pub type PK_MESH_replace_normal_t = c_int;

/// Replace normals for specific mtopols.
pub const PK_MESH_replace_normal_mtopol_c: PK_MESH_replace_normal_t = 0;
/// Replace all normals in mesh.
pub const PK_MESH_replace_normal_all_c: PK_MESH_replace_normal_t = 1;

/// Normal type enum for PK_MESH_ask_normal_type.
pub type PK_MESH_normal_type_t = c_int;

// =============================================================================
// Non-aligned box standard form (for viewport)
// =============================================================================

// =============================================================================
// Local tolerance specification
// =============================================================================

/// Per-entity tolerance override for faceting.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_facet_local_tolerances_t {
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
}

// =============================================================================
// Facet mesh generation options (GO-based: PK_TOPOL_render_facet)
// =============================================================================

/// Mesh generation options for PK_TOPOL_render_facet.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_mesh_o_t {
    pub shape: PK_facet_shape_t,
    pub match_: PK_facet_match_t,
    pub density: PK_facet_density_t,
    pub n_view_directions: c_int,
    pub view_directions: *const PK_VECTOR_t,
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
    pub is_local_density_tol: PK_LOGICAL_t,
    pub local_density_tol: c_double,
    pub is_local_density_ang: PK_LOGICAL_t,
    pub local_density_ang: c_double,
    pub n_local_tols: c_int,
    pub local_tols: *const PK_facet_local_tolerances_t,
    pub n_topols_with_local_tols: c_int,
    pub topols_with_local_tols: *const PK_TOPOL_t,
    pub local_tols_for_topols: *const c_int,
    pub degen: PK_facet_degen_t,
    pub is_facet_plane_tol: PK_LOGICAL_t,
    pub facet_plane_tol: c_double,
    pub is_facet_plane_ang: PK_LOGICAL_t,
    pub facet_plane_ang: c_double,
    pub wire_edges: PK_facet_wire_edges_t,
    pub ignore: PK_facet_ignore_t,
    pub ignore_value: c_double,
    pub ignore_scope: PK_facet_ignore_scope_t,
    pub incremental_facetting: PK_facet_incr_t,
    pub incremental_method: PK_facet_incr_method_t,
    pub incremental_transformation: PK_facet_incr_tf_t,
    pub inflect: PK_facet_inflect_t,
    pub quality: PK_facet_quality_t,
    pub vertices_on_planar: PK_LOGICAL_t,
    pub sing_topol: PK_facet_sing_topol_t,
    pub respect_offset: PK_facet_respect_t,
}

// =============================================================================
// GO output options for PK_TOPOL_render_facet
// =============================================================================

/// Controls what data is output through GO for PK_TOPOL_render_facet.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_render_facet_go_o_t {
    pub go_normals: PK_facet_go_normals_t,
    pub go_parameters: PK_facet_go_parameters_t,
    pub go_curvatures: PK_facet_go_curvatures_t,
    pub go_edges: PK_facet_go_edges_t,
    pub go_strips: PK_facet_go_strips_t,
    pub go_interleaved: PK_facet_go_interleaved_t,
    pub go_max_facets_per_strip: c_int,
}

// =============================================================================
// Top-level option struct for PK_TOPOL_render_facet
// =============================================================================

/// Top-level options for PK_TOPOL_render_facet.
/// Contains mesh generation control and GO output options.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_render_facet_o_t {
    pub control: PK_TOPOL_facet_mesh_o_t,
    pub go_option: PK_TOPOL_render_facet_go_o_t,
}

// =============================================================================
// Facet mesh generation options (tabular: PK_TOPOL_facet_2)
// =============================================================================

/// Mesh generation options for PK_TOPOL_facet_2.
/// Similar to PK_TOPOL_facet_mesh_o_t but with additional cull_transfs.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_mesh_2_o_t {
    pub shape: PK_facet_shape_t,
    pub match_: PK_facet_match_t,
    pub density: PK_facet_density_t,
    pub n_view_directions: c_int,
    pub view_directions: *const PK_VECTOR_t,
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
    pub is_local_density_tol: PK_LOGICAL_t,
    pub local_density_tol: c_double,
    pub is_local_density_ang: PK_LOGICAL_t,
    pub local_density_ang: c_double,
    pub n_local_tols: c_int,
    pub local_tols: *const PK_facet_local_tolerances_t,
    pub n_topols_with_local_tols: c_int,
    pub topols_with_local_tols: *const PK_TOPOL_t,
    pub local_tols_for_topols: *const c_int,
    pub degen: PK_facet_degen_t,
    pub is_facet_plane_tol: PK_LOGICAL_t,
    pub facet_plane_tol: c_double,
    pub is_facet_plane_ang: PK_LOGICAL_t,
    pub facet_plane_ang: c_double,
    pub wire_edges: PK_facet_wire_edges_t,
    pub ignore: PK_facet_ignore_t,
    pub ignore_value: c_double,
    pub ignore_scope: PK_facet_ignore_scope_t,
    pub incremental_facetting: PK_facet_incr_t,
    pub incremental_method: PK_facet_incr_method_t,
    pub incremental_transformation: PK_facet_incr_tf_t,
    pub inflect: PK_facet_inflect_t,
    pub quality: PK_facet_quality_t,
    pub vertices_on_planar: PK_LOGICAL_t,
    pub sing_topol: PK_facet_sing_topol_t,
    pub respect_offset: PK_facet_respect_t,
}

// =============================================================================
// Callback type for per-body-instance table return
// =============================================================================

/// Callback function type for receiving facet tables per body instance.
///
/// Called by PK_TOPOL_facet_2 when `facet_tables_cb` is set.
///
/// Arguments:
/// - `topol`: body the facets belong to
/// - `transform`: transform applied
/// - `tables`: the faceting tables
/// - `context`: user context pointer
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

/// Controls which data tables are returned by PK_TOPOL_facet_2.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_choice_2_o_t {
    pub smp: PK_facet_smp_t,
    pub max_facets_per_strip: c_int,
    pub split_strips: PK_LOGICAL_t,
    pub consistent_parms: PK_facet_consistent_parms_t,
    pub report_pts_off_topol: PK_facet_pt_report_t,
    /// Callback receiving tables per body instance. NULL for monolithic return.
    pub facet_tables_cb: PK_TOPOL_facet_tables_cb_t,
    /// Context data for callback.
    pub facet_tables_context: *mut c_void,
    /// Whether callback is thread-safe.
    pub thread_safe: PK_LOGICAL_t,
}

// =============================================================================
// Top-level option struct for PK_TOPOL_facet_2
// =============================================================================

/// Top-level options for PK_TOPOL_facet_2.
/// Contains mesh generation control and tabular output selection.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_2_o_t {
    pub control: PK_TOPOL_facet_mesh_2_o_t,
    pub choice: PK_TOPOL_facet_choice_2_o_t,
}

// =============================================================================
// Facet table structures (tabular output)
// =============================================================================

/// Container for all returned data tables from PK_TOPOL_facet_2.
///
/// Each pointer is NULL if the corresponding data was not requested.
/// Indices in lookup/indexed tables are 1-based; -1 = separator, -2 = error,
/// -3 = degenerate at singularity.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_table_t {
    // --- Topological tables ---
    pub n_facet_fin: c_int,
    pub facet_fin: *mut c_int,
    pub n_fin_fin: c_int,
    pub fin_fin: *mut c_int,
    pub n_strip_boundary: c_int,
    pub strip_boundary: *mut c_int,
    pub n_strip_zigzag: c_int,
    pub strip_zigzag: *mut c_int,
    pub n_fin_data: c_int,
    pub fin_data: *mut c_int,

    // --- Index tables ---
    pub n_data_point_idx: c_int,
    pub data_point_idx: *mut c_int,
    pub n_data_normal_idx: c_int,
    pub data_normal_idx: *mut c_int,
    pub n_data_param_idx: c_int,
    pub data_param_idx: *mut c_int,
    pub n_data_deriv_idx: c_int,
    pub data_deriv_idx: *mut c_int,
    pub n_data_curv_idx: c_int,
    pub data_curv_idx: *mut c_int,

    // --- Geometric tables ---
    pub n_point_vec: c_int,
    pub point_vec: *mut c_double,
    pub n_normal_vec: c_int,
    pub normal_vec: *mut c_double,
    pub n_param_uv: c_int,
    pub param_uv: *mut c_double,
    pub n_deriv_dp: c_int,
    pub deriv_dp: *mut c_double,
    pub n_deriv_d2p: c_int,
    pub deriv_d2p: *mut c_double,
    pub n_curv_dirs: c_int,
    pub curv_dirs: *mut c_double,

    // --- Tracking tables ---
    pub n_facet_face: c_int,
    pub facet_face: *mut PK_FACE_t,
    pub n_strip_face: c_int,
    pub strip_face: *mut PK_FACE_t,
    pub n_fin_edge: c_int,
    pub fin_edge: *mut c_int,
    pub n_point_topol: c_int,
    pub point_topol: *mut c_int,
    pub n_fin_topol: c_int,
    pub fin_topol: *mut c_int,

    // --- Error tables ---
    pub n_error_object: c_int,
    pub error_object: *mut c_int,
}

/// Result structure from PK_TOPOL_facet_2.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_facet_2_r_t {
    pub n_facets: c_int,
    pub n_strips: c_int,
    pub n_fins: c_int,
    pub n_tables: c_int,
    pub tables: PK_TOPOL_facet_table_t,
}

// =============================================================================
// PK_TOPOL_render_line option struct
// =============================================================================

/// Options for PK_TOPOL_render_line.
///
/// Controls edge rendering, silhouettes, hidden-line removal, hatching,
/// viewport, transparency, overlap detection, and more.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_render_line_o_t {
    // --- Edge/silhouette ---
    pub edge: PK_render_edge_t,
    pub silhouette: PK_render_silhouette_t,
    pub mesh_normal_field: PK_MESH_normal_field_t,
    pub sharp_mfins: PK_render_sharp_mfins_t,

    // --- Visibility ---
    pub vis: PK_render_vis_t,
    pub invisible: PK_render_invisible_t,
    pub drafting: PK_render_drafting_t,
    pub self_hidden: PK_render_self_hidden_t,

    // --- Smoothness ---
    pub smooth: PK_render_smooth_t,
    pub is_edge_smooth_tol: PK_LOGICAL_t,
    pub edge_smooth_tol: c_double,

    // --- Internal edges ---
    pub internal: PK_render_internal_t,

    // --- Missing geometry ---
    pub ske_missing: PK_render_ske_missing_t,

    // --- Ignore small features ---
    pub ignore: PK_render_ignore_t,
    pub ignore_value: c_double,

    // --- Hierarchical output ---
    pub hierarch: PK_render_hierarch_t,

    // --- B-curve format ---
    pub bcurve: PK_render_bcurve_t,

    // --- Curve chord tolerances ---
    pub is_curve_chord_tol: PK_LOGICAL_t,
    pub curve_chord_tol: c_double,
    pub is_curve_chord_max: PK_LOGICAL_t,
    pub curve_chord_max: c_double,
    pub is_curve_chord_ang: PK_LOGICAL_t,
    pub curve_chord_ang: c_double,

    // --- Memory target ---
    pub memory_target: PK_render_memory_target_t,
    pub memory_target_value: c_int,

    // --- Report lines ---
    pub report_line: PK_render_report_line_t,

    // --- Planar hatching ---
    pub planar: PK_render_planar_t,
    pub planar_axis: PK_VECTOR_t,
    pub planar_spacing: c_double,
    pub planar_location: PK_VECTOR_t,

    // --- Radial hatching ---
    pub radial: PK_render_radial_t,
    pub radial_around: c_double,
    pub radial_along: c_double,
    pub radial_about: c_double,
    pub radial_around_start: c_double,
    pub radial_along_start: c_double,
    pub radial_about_start: c_double,

    // --- Parametric hatching ---
    pub param: PK_render_param_t,
    pub param_u: c_double,
    pub param_v: c_double,
    pub param_u_start: c_double,
    pub param_v_start: c_double,

    // --- Unfixed blends ---
    pub unfix: PK_render_unfix_t,
    pub unfix_spacing: c_double,

    // --- Overlapping bodies ---
    pub overlap: PK_render_overlap_t,
    pub n_overlap_indices1: c_int,
    pub overlap_indices1: *const c_int,
    pub n_overlap_indices2: c_int,
    pub overlap_indices2: *const c_int,

    // --- Transparency ---
    pub transparent: PK_render_transparent_t,
    pub n_transparent_indices: c_int,
    pub transparent_indices: *const c_int,
    pub transp_hid: PK_render_transp_hid_t,

    // --- Suppressed hidden lines ---
    pub n_suppressed_indices: c_int,
    pub suppressed_indices: *const c_int,

    // --- Viewport ---
    pub viewport: PK_render_viewport_t,
    pub viewport_type: PK_render_viewport_type_t,
    pub viewport_sf: PK_NABOX_sf_t,
    pub n_viewports: c_int,
    pub viewports: *const PK_NABOX_sf_t,
    pub viewport_clip: PK_render_viewport_clip_t,

    // --- Regional data ---
    pub region: PK_render_region_t,
}

// =============================================================================
// PK_GEOM_render option struct
// =============================================================================

/// Options for PK_GEOM_render (geometry rendering).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_render_o_t {
    pub boundary: PK_render_boundary_t,
    pub param: PK_render_param_t,
    pub param_u: c_double,
    pub param_v: c_double,
    pub bcurve: PK_render_bcurve_t,
    pub lattice: PK_render_lattice_t,
    pub n_geom_transfs: c_int,
    pub geom_transfs: *const PK_TRANSF_t,
    pub is_curve_chord_tol: PK_LOGICAL_t,
    pub curve_chord_tol: c_double,
    pub is_curve_chord_max: PK_LOGICAL_t,
    pub curve_chord_max: c_double,
    pub is_curve_chord_ang: PK_LOGICAL_t,
    pub curve_chord_ang: c_double,
}

// =============================================================================
// Picking option/result structs
// =============================================================================

/// Options for PK_BODY_pick_topols.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_pick_topols_o_t {
    pub max_faces: c_int,
    pub max_edges: c_int,
    pub max_vertices: c_int,
    pub max_edge_dist: c_double,
    pub max_vertex_dist: c_double,
    pub ignore_excess_entities: PK_LOGICAL_t,
    pub method: PK_BODY_pick_method_t,
    pub ratio: c_double,
    pub ignore_back_faces: PK_LOGICAL_t,
    pub pick_approx: PK_pick_approximate_t,
    pub location: PK_VECTOR_t,
    pub direction: PK_VECTOR_t,
}

/// Result structure from PK_BODY_pick_topols.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_pick_topols_r_t {
    pub n_faces: c_int,
    pub faces: *mut PK_FACE_t,
    pub n_edges: c_int,
    pub edges: *mut PK_EDGE_t,
    pub n_vertices: c_int,
    pub vertices: *mut PK_VERTEX_t,
    pub e_faces: c_int,
    pub e_edges: c_int,
    pub e_vertices: c_int,
    pub face_points: *mut PK_VECTOR_t,
    pub edge_points: *mut PK_VECTOR_t,
    pub vertex_points: *mut PK_VECTOR_t,
    pub face_distances: *mut c_double,
    pub edge_distances: *mut c_double,
    pub vertex_distances: *mut c_double,
}

// =============================================================================
// Facet body conversion option structs
// =============================================================================

/// Options for PK_BODY_make_facet_body.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_facet_body_o_t {
    pub is_max_facet_width: PK_LOGICAL_t,
    pub max_facet_width: c_double,
    pub is_distance_tolerance: PK_LOGICAL_t,
    pub distance_tolerance: c_double,
    pub is_angular_tolerance: PK_LOGICAL_t,
    pub angular_tolerance: c_double,
    pub is_max_chord_length: PK_LOGICAL_t,
    pub max_chord_length: c_double,
    pub retain_attributes: PK_LOGICAL_t,
    pub retain_groups: PK_LOGICAL_t,
    pub track_faces: PK_LOGICAL_t,
    pub track_edges: PK_track_edges_t,
    pub return_redundant: PK_LOGICAL_t,
}

/// Options for PK_MESH_make_bodies.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_make_bodies_o_t {
    pub vertex_angle: c_double,
    pub allow_disjoint: PK_LOGICAL_t,
    pub preferred_body_type: PK_BODY_type_t,
}

/// Callback type for propagation from seed mtopol (PK_MTOPOL_make_meshes).
pub type PK_MTOPOL_select_cb_t = Option<
    unsafe extern "C" fn(
        mtopol: PK_MTOPOL_t,
        data: *mut c_void,
    ) -> PK_LOGICAL_t,
>;

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

/// Options for PK_MTOPOL_make_meshes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MTOPOL_make_meshes_o_t {
    pub allow_disjoint: PK_LOGICAL_t,
    pub select_cb: PK_MTOPOL_select_cb_t,
    pub select_data: *mut c_void,
    pub select_mtopol_class: PK_CLASS_t,
    pub select_type: PK_selector_type_t,
    pub map_mfacets: PK_MFACET_map_t,
    pub map_mvertices: PK_MVERTEX_map_t,
    pub map_cb: PK_MTOPOL_map_cb_f_t,
    pub map_data: *mut c_void,
    pub n_max_cb_mtopol: c_int,
    pub n_faces: c_int,
    pub faces: *const PK_FACE_t,
    pub senses: *const PK_LOGICAL_t,
    pub contributions: *const c_int,
}

/// Options for PK_MESH_make_surf_trimmed.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_make_surf_trimmed_o_t {
    pub fitting_tolerance: c_double,
    pub have_bdry_tolerance: PK_LOGICAL_t,
    pub bdry_tolerance: c_double,
    pub bdry_mvertex_angle: c_double,
    pub fit_normals: PK_LOGICAL_t,
    pub have_normal_tolerance: PK_LOGICAL_t,
    pub normal_tolerance: c_double,
}

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

/// Facet data block — independent facet vectors. vertex_positions as triples.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_facet_vector_t {
    pub n_vertex_positions: c_int,
    pub vertex_positions: *const c_double,
    pub n_vertex_normals: c_int,
    pub vertex_normals: *const c_double,
}

/// Callback type for facet reader (PK_MESH_create_from_facets).
///
/// Called repeatedly to supply facet data blocks. Returns PK_MESH_cb_status_t.
pub type PK_MESH_facet_reader_t = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        facet_type: *mut PK_MESH_facet_type_t,
        facet_data: *mut *mut c_void,
        status: *mut PK_MESH_cb_status_t,
    ) -> PK_ERROR_code_t,
>;

/// Callback type for freeing facet data blocks.
pub type PK_MESH_facet_free_t = Option<
    unsafe extern "C" fn(
        context: *mut c_void,
        facet_data: *mut c_void,
    ),
>;

/// Options for PK_MESH_create_from_facets.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_create_from_facets_o_t {
    pub vertices_estimate: c_int,
    pub facet_estimate: c_int,
    pub facet_free: PK_MESH_facet_free_t,
    pub create: PK_MESH_create_t,
    pub have_box: PK_LOGICAL_t,
    pub box_: PK_BOX_t,
    pub thread_safe: PK_LOGICAL_t,
}

// =============================================================================
// Mesh checking/defect structs
// =============================================================================

/// Options for PK_MESH_find_defects.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_find_defects_o_t {
    pub have_tolerance: PK_LOGICAL_t,
    pub tolerance: c_double,
    pub have_height_tolerance: PK_LOGICAL_t,
    pub height_tolerance: c_double,
    pub have_degen_tolerance: PK_LOGICAL_t,
    pub degen_tolerance: c_double,
    pub have_slit_tolerance: PK_LOGICAL_t,
    pub slit_tolerance: c_double,
}

/// Options for PK_MESH_fix_defects.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_fix_defects_o_t {
    pub have_tolerance: PK_LOGICAL_t,
    pub tolerance: c_double,
    pub have_height_tolerance: PK_LOGICAL_t,
    pub height_tolerance: c_double,
    pub have_degen_tolerance: PK_LOGICAL_t,
    pub degen_tolerance: c_double,
    pub have_slit_tolerance: PK_LOGICAL_t,
    pub slit_tolerance: c_double,
}

/// Structure containing defect information returned by PK_MESH_find_defects.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_defect_details_t {
    pub n_defects: c_int,
    pub defect_types: *mut PK_MESH_defect_t,
    pub n_entities: c_int,
    pub entities: *mut PK_MTOPOL_t,
    pub entity_defect_indices: *mut c_int,
}

// =============================================================================
// Mesh normal storage options
// =============================================================================

/// Options for PK_MESH_store_normals.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_store_normals_o_t {
    pub replace: PK_MESH_replace_normal_t,
    pub have_mesh_angle: PK_LOGICAL_t,
    pub mesh_angle: c_double,
    pub n_mtopols: c_int,
    pub mtopols: *const PK_MTOPOL_t,
    pub n_mtopol_normals: c_int,
    pub mtopol_normals: *const c_double,
    pub mtopol_normal_indices: *const c_int,
}

// =============================================================================
// Mesh sharp feature options
// =============================================================================

/// Options for PK_MESH_find_sharp_mvxs.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_find_sharp_mvxs_o_t {
    pub want_sharp_mvxs: PK_LOGICAL_t,
}

/// Options for PK_MESH_find_sharp_mfins.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_find_sharp_mfins_o_t {
    pub want_sharp_mfins: PK_LOGICAL_t,
    pub have_sharp_angle: PK_LOGICAL_t,
    pub sharp_angle: c_double,
}

// =============================================================================
// Perimeter finding options
// =============================================================================

/// Options for PK_MFACET_find_perimeters.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MFACET_find_perimeters_o_t {
    pub want_plines: PK_LOGICAL_t,
    pub select_cb: PK_MTOPOL_select_cb_t,
    pub select_data: *mut c_void,
    pub select_mtopol_class: PK_CLASS_t,
    pub select_wire_mfins: PK_selector_type_t,
    pub min_n_mfacets: c_int,
}

// =============================================================================
// FIN_find_mtopols options
// =============================================================================

/// Options for PK_FIN_find_mtopols.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FIN_find_mtopols_o_t {
    pub want_mvertices: PK_LOGICAL_t,
    pub want_mfins: PK_LOGICAL_t,
}

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

/// Options for PK_PARTITION_ask_facet_geom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_ask_facet_geom_o_t {
    pub want_parts: PK_LOGICAL_t,
    pub want_geoms: PK_LOGICAL_t,
}

/// Options for PK_TOPOL_categorise_geom.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_categorise_geom_o_t {
    pub want_related_topols: PK_related_topols_t,
}

// =============================================================================
// Mesh imprint options
// =============================================================================

/// Options for PK_MESH_imprint_vectors.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_imprint_vectors_o_t {
    // Opaque -- details not documented in available notes.
    _opaque: c_int,
}

// =============================================================================
// Mesh fill holes options
// =============================================================================

/// Options for PK_MESH_fill_holes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_fill_holes_o_t {
    // Opaque -- details not documented in available notes.
    _opaque: c_int,
}

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
        topols: *const PK_TOPOL_t,
        n_transfs: c_int,
        transfs: *const PK_TRANSF_t,
        options: *const PK_TOPOL_facet_2_o_t,
        result: *mut PK_TOPOL_facet_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Free memory for returned facet tables from PK_TOPOL_facet_2.
    pub fn PK_TOPOL_facet_2_r_f(
        result: *mut PK_TOPOL_facet_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Facet topology (legacy version).
    pub fn PK_TOPOL_facet(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        n_transfs: c_int,
        transfs: *const PK_TRANSF_t,
        options: *const c_void,
        result: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Free memory for returned facet tables from PK_TOPOL_facet.
    pub fn PK_TOPOL_facet_r_f(
        result: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Facet topology with output through GO (Graphical Output) interface.
    ///
    /// Outputs facets as closed polygons through registered GO callbacks.
    pub fn PK_TOPOL_render_facet(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        n_transfs: c_int,
        transfs: *const PK_TRANSF_t,
        options: *const PK_TOPOL_render_facet_o_t,
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
        topols: *const PK_TOPOL_t,
        n_transfs: c_int,
        transfs: *const PK_TRANSF_t,
        view_transf: PK_TRANSF_t,
        options: *const PK_TOPOL_render_line_o_t,
    ) -> PK_ERROR_code_t;

    /// Render geometry (B-curves, B-surfaces, foreign geometry).
    ///
    /// Wire-mesh form output via GO interface.
    pub fn PK_GEOM_render(
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
        options: *const PK_GEOM_render_o_t,
    ) -> PK_ERROR_code_t;

    /// Render a line representation of geometry.
    pub fn PK_GEOM_render_line(
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
        options: *const c_void,
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
    pub fn PK_TOPOL_render_volume_r_f(
        result: *mut c_void,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Picking functions
    // =========================================================================

    /// Select faces/edges/vertices from bodies via ray intersection/proximity.
    pub fn PK_BODY_pick_topols(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        n_transfs: c_int,
        transfs: *const PK_TRANSF_t,
        options: *const PK_BODY_pick_topols_o_t,
        result: *mut PK_BODY_pick_topols_r_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_BODY_pick_topols.
    pub fn PK_BODY_pick_topols_r_f(
        result: *mut PK_BODY_pick_topols_r_t,
    ) -> PK_ERROR_code_t;

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
        options: *const PK_BODY_make_facet_body_o_t,
        facet_body: *mut PK_BODY_t,
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
    pub fn PK_MTOPOL_map_r_f(
        result: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Convert mesh to single trimmed classic surface.
    pub fn PK_MESH_make_surf_trimmed(
        mesh: PK_MESH_t,
        options: *const PK_MESH_make_surf_trimmed_o_t,
        surf: *mut PK_SURF_t,
        trim_data: *mut c_void,
        sense: *mut PK_LOGICAL_t,
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
        options: *const PK_MESH_fill_holes_o_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Mesh checking and defect repair
    // =========================================================================

    /// Examine mesh for defects.
    pub fn PK_MESH_find_defects(
        mesh: PK_MESH_t,
        options: *const PK_MESH_find_defects_o_t,
        details: *mut PK_MESH_defect_details_t,
    ) -> PK_ERROR_code_t;

    /// Examine mesh, attempt to fix defects.
    ///
    /// Returns new repaired meshes and remaining (unfixed) defect details.
    pub fn PK_MESH_fix_defects(
        mesh: PK_MESH_t,
        options: *const PK_MESH_fix_defects_o_t,
        n_meshes: *mut c_int,
        meshes: *mut *mut PK_MESH_t,
        details: *mut PK_MESH_defect_details_t,
    ) -> PK_ERROR_code_t;

    /// Free defect array from PK_MESH_find_defects/PK_MESH_fix_defects.
    pub fn PK_MESH_defect_array_f(
        details: *mut PK_MESH_defect_details_t,
    ) -> PK_ERROR_code_t;

    /// Free defect details from PK_MESH_find_defects/PK_MESH_fix_defects.
    pub fn PK_MESH_defect_details_f(
        details: *mut PK_MESH_defect_details_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Mesh normal management
    // =========================================================================

    /// Query whether mesh has stored normals or uses dynamic calculation.
    pub fn PK_MESH_ask_normal_type(
        mesh: PK_MESH_t,
        normal_type: *mut PK_MESH_normal_type_t,
    ) -> PK_ERROR_code_t;

    /// Whether all mvertices share a single normal.
    pub fn PK_MESH_has_unique_normals(
        mesh: PK_MESH_t,
        has_unique: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Store/modify normals on mesh.
    pub fn PK_MESH_store_normals(
        mesh: PK_MESH_t,
        options: *const PK_MESH_store_normals_o_t,
    ) -> PK_ERROR_code_t;

    /// Discard stored normals (revert to dynamic calculation).
    pub fn PK_MESH_discard_normals(
        mesh: PK_MESH_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Mesh enquiry
    // =========================================================================

    /// Number of mfacets in mesh.
    pub fn PK_MESH_ask_n_mfacets(
        mesh: PK_MESH_t,
        n_mfacets: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Number of mvertices in mesh.
    pub fn PK_MESH_ask_n_mvertices(
        mesh: PK_MESH_t,
        n_mvertices: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Evaluate UV-parameters; returns position, mfacet, and mfin/mvertex at position.
    pub fn PK_MESH_eval_with_mtopol(
        mesh: PK_MESH_t,
        uv: *const PK_UV_t,
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
        n_mfins: c_int,
        mfins: *mut PK_MFIN_t,
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
        is_loaded: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_MESH_is_loaded.
    pub fn PK_MESH_is_loaded_r_f(
        result: *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Imprint array of vectors onto a mesh.
    pub fn PK_MESH_imprint_vectors(
        mesh: PK_MESH_t,
        n_vectors: c_int,
        vectors: *const c_double,
        options: *const PK_MESH_imprint_vectors_o_t,
        n_mvertices: *mut c_int,
        mvertices: *mut *mut PK_MVERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_MESH_imprint_vectors.
    pub fn PK_MESH_imprint_vectors_r_f(
        n_mvertices: c_int,
        mvertices: *mut PK_MVERTEX_t,
    ) -> PK_ERROR_code_t;

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
        normals: *mut [PK_VECTOR_t; 3],
    ) -> PK_ERROR_code_t;

    /// Returns normal vector of mfacet.
    pub fn PK_MFACET_ask_normal(
        mfacet: PK_MFACET_t,
        normal: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Returns positions of mvertices of mfacet.
    pub fn PK_MFACET_ask_positions(
        mfacet: PK_MFACET_t,
        positions: *mut [PK_VECTOR_t; 3],
    ) -> PK_ERROR_code_t;

    /// Finds mesh parameterisation of position on mfacet.
    pub fn PK_MFACET_parameterise_vec(
        mfacet: PK_MFACET_t,
        position: *const PK_VECTOR_t,
        uv: *mut PK_UV_t,
    ) -> PK_ERROR_code_t;

    /// Find perimeters (mloops) around supplied set of mfacets.
    pub fn PK_MFACET_find_perimeters(
        n_mfacets: c_int,
        mfacets: *const PK_MFACET_t,
        options: *const PK_MFACET_find_perimeters_o_t,
        n_perimeters: *mut c_int,
        perimeters: *mut *mut c_void,
    ) -> PK_ERROR_code_t;

    /// Free result from PK_MFACET_find_perimeters.
    pub fn PK_MFACET_find_perimeters_r_f(
        n_perimeters: c_int,
        perimeters: *mut c_void,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // MFIN enquiry
    // =========================================================================

    /// Returns mfacet containing the mfin.
    pub fn PK_MFIN_ask_mfacet(
        mfin: PK_MFIN_t,
        mfacet: *mut PK_MFACET_t,
    ) -> PK_ERROR_code_t;

    /// Returns coincident mfin in adjacent facet.
    pub fn PK_MFIN_ask_mfin_adjacent(
        mfin: PK_MFIN_t,
        adjacent: *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    /// Returns index of mfin in its mfacet.
    pub fn PK_MFIN_ask_mfin_index(
        mfin: PK_MFIN_t,
        index: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Returns mvertex pointed to by mfin.
    pub fn PK_MFIN_ask_mvertex(
        mfin: PK_MFIN_t,
        mvertex: *mut PK_MVERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Returns normal at mvertex of given mfin (per-mfin normal if multiple exist).
    pub fn PK_MFIN_ask_mvx_normal(
        mfin: PK_MFIN_t,
        normal: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Returns curvature at mvertex of given mfin.
    pub fn PK_MFIN_ask_mvx_curvature(
        mfin: PK_MFIN_t,
        curvature: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Returns next mfin in mfacet.
    pub fn PK_MFIN_ask_next_in_mfacet(
        mfin: PK_MFIN_t,
        next: *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    /// Returns next mfin around mvertex.
    pub fn PK_MFIN_ask_next_of_mvertex(
        mfin: PK_MFIN_t,
        next: *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

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
    pub fn PK_MFIN_is_laminar(
        mfin: PK_MFIN_t,
        is_laminar: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Whether two mfins belong to same mfacet.
    pub fn PK_MFIN_is_same_mfacet(
        mfin1: PK_MFIN_t,
        mfin2: PK_MFIN_t,
        is_same: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Whether mfin is sharp.
    pub fn PK_MFIN_is_sharp(
        mfin: PK_MFIN_t,
        is_sharp: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // MVERTEX enquiry
    // =========================================================================

    /// Returns all unique normals of mvertex.
    pub fn PK_MVERTEX_ask_normals(
        mvertex: PK_MVERTEX_t,
        n_normals: *mut c_int,
        normals: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Returns all mfacets using mvertex.
    pub fn PK_MVERTEX_ask_mfacets(
        mvertex: PK_MVERTEX_t,
        n_mfacets: *mut c_int,
        mfacets: *mut *mut PK_MFACET_t,
    ) -> PK_ERROR_code_t;

    /// Returns an mfin pointing to mvertex.
    pub fn PK_MVERTEX_ask_mfin(
        mvertex: PK_MVERTEX_t,
        mfin: *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

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
        mvertices: *const PK_MVERTEX_t,
        positions: *const PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // MTOPOL enquiry
    // =========================================================================

    /// Returns bounding box for mtopol.
    pub fn PK_MTOPOL_ask_box(
        mtopol: PK_MTOPOL_t,
        box_: *mut PK_BOX_t,
    ) -> PK_ERROR_code_t;

    /// Returns class of mesh topology element.
    pub fn PK_MTOPOL_ask_class(
        mtopol: PK_MTOPOL_t,
        class: *mut PK_CLASS_t,
    ) -> PK_ERROR_code_t;

    /// Whether given mtopol is valid.
    pub fn PK_MTOPOL_is(
        mtopol: PK_MTOPOL_t,
        is_valid: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Fin to mesh topology
    // =========================================================================

    /// Returns mtopology of a given fin (mvertices and/or mfins along a fin).
    pub fn PK_FIN_find_mtopols(
        fin_: PK_FIN_t,
        options: *const PK_FIN_find_mtopols_o_t,
        n_mvertices: *mut c_int,
        mvertices: *mut *mut PK_MVERTEX_t,
        n_mfins: *mut c_int,
        mfins: *mut *mut PK_MFIN_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Polyline creation
    // =========================================================================

    /// Create a polyline.
    pub fn PK_PLINE_create(
        sf: *const PK_PLINE_sf_t,
        pline: *mut PK_PLINE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Face imprint
    // =========================================================================

}
