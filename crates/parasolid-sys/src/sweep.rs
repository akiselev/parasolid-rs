#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! Sweep, extrusion, lofting, profiling, shadow curves, and emboss bindings.
//!
//! Covers Parasolid chapters 34--40:
//! - Ch. 34: Creating profiles (outline curves)
//! - Ch. 35: Creating extruded bodies
//! - Ch. 36: Sweeping
//! - Ch. 37: Swept tool bodies
//! - Ch. 38: Lofting
//! - Ch. 39: Shadow curves
//! - Ch. 40: Emboss features

use crate::*;

use std::os::raw::{c_double, c_int};

// =============================================================================
// Common tracking structure (shared across sweep/extrude/loft/emboss)
// =============================================================================

/// Topological tracking information returned by sweep/extrude/loft operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_tracking_t {
    pub n_original_topols: c_int,
    pub original_topols: *mut PK_TOPOL_t,
    pub n_new_topols: *mut c_int,
    pub new_topols: *mut *mut PK_TOPOL_t,
}

// =============================================================================
// Axis definition (used by helical functions, swept tool, emboss, spun outline)
// =============================================================================

// =============================================================================
// Handedness enum (helical functions)
// =============================================================================

// =============================================================================
// Topological dimension (outline body creation)
// =============================================================================

// =============================================================================
// Parameterisation consistency
// =============================================================================

pub type PK_PARAM_consistent_t = c_int;
/// Default (inconsistent).
pub const PK_PARAM_consistent_unset_c: PK_PARAM_consistent_t = 0;
/// Align v-parameter with sweep direction.
pub const PK_PARAM_consistent_set_c: PK_PARAM_consistent_t = 1;

// =============================================================================
// Periodicity
// =============================================================================

pub type PK_PARAM_periodic_t = c_int;
pub const PK_PARAM_periodic_no_c: PK_PARAM_periodic_t = 0;
pub const PK_PARAM_periodic_yes_c: PK_PARAM_periodic_t = 1;

// =============================================================================
// Ch. 34 — Profiling: Outline projection modes
// =============================================================================

pub type PK_outline_project_t = c_int;
/// Output 3D curves (default).
pub const PK_outline_project_no_c: PK_outline_project_t = 0;
/// Project onto plane.
pub const PK_outline_project_plane_c: PK_outline_project_t = 1;
/// Project onto sphere.
pub const PK_outline_project_sphere_c: PK_outline_project_t = 2;
/// Project onto cylinder.
pub const PK_outline_project_cylinder_c: PK_outline_project_t = 3;

// =============================================================================
// Edge-on handling (curves_outline only)
// =============================================================================

pub type PK_outline_edge_on_t = c_int;
/// Do not create outlines (default).
pub const PK_outline_edge_on_none_c: PK_outline_edge_on_t = 0;
/// Create outlines if all input bodies are wire/edge-on sheet.
pub const PK_outline_edge_on_all_c: PK_outline_edge_on_t = 1;
/// Create outlines from all components, trimmed.
pub const PK_outline_edge_on_both_c: PK_outline_edge_on_t = 2;

// =============================================================================
// Outline update control
// =============================================================================

/// Use all enhancements (default).
pub const PK_outline_update_default_c: PK_outline_update_t = 0;

// =============================================================================
// Outline keep_as_facet control
// =============================================================================

pub type PK_outline_keep_as_facet_t = c_int;
pub const PK_outline_keep_as_facet_no_c: PK_outline_keep_as_facet_t = 0;
pub const PK_outline_keep_as_facet_yes_c: PK_outline_keep_as_facet_t = 1;

// =============================================================================
// Perspective clipping modes
// =============================================================================

pub type PK_persp_clipping_t = c_int;
/// No clipping (default).
pub const PK_persp_clipping_no_c: PK_persp_clipping_t = 0;
/// Clip to cone.
pub const PK_persp_clipping_cone_c: PK_persp_clipping_t = 1;
/// Clip to spherical sector.
pub const PK_persp_clipping_sector_c: PK_persp_clipping_t = 2;
/// Clip to wedge.
pub const PK_persp_clipping_wedge_c: PK_persp_clipping_t = 3;
/// Clip to spherical pyramid.
pub const PK_persp_clipping_pyramid_c: PK_persp_clipping_t = 4;
/// Clip to body outline cone.
pub const PK_persp_clipping_body_c: PK_persp_clipping_t = 5;

// =============================================================================
// Ch. 34 — Options structures for outline functions
// =============================================================================

/// Options for `PK_BODY_make_curves_outline` (parallel projection outline).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_curves_outline_o_t {
    pub project: PK_outline_project_t,
    pub tolerance: c_double,
    pub want_body: PK_LOGICAL_t,
    pub body_dimension: PK_TOPOL_dimension_t,
    pub keep_as_facet: PK_outline_keep_as_facet_t,
    pub project_position: PK_VECTOR_t,
    pub want_topols: PK_LOGICAL_t,
    pub edge_on: PK_outline_edge_on_t,
    pub update: PK_outline_update_t,
}

/// Options for `PK_BODY_make_spun_outline` (rotational outline).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_spun_outline_o_t {
    pub project: PK_outline_project_t,
    pub tolerance: c_double,
    pub want_body: PK_LOGICAL_t,
    pub body_dimension: PK_TOPOL_dimension_t,
    pub keep_as_facet: PK_outline_keep_as_facet_t,
    pub project_position: PK_VECTOR_t,
    pub want_topols: PK_LOGICAL_t,
    pub update: PK_outline_update_t,
}

/// Options for `PK_BODY_make_persp_outline` (perspective outline).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_persp_outline_o_t {
    pub project: PK_outline_project_t,
    pub project_location: PK_VECTOR_t,
    pub project_direction: PK_VECTOR_t,
    pub want_tracking: PK_LOGICAL_t,
    pub clipping: PK_persp_clipping_t,
    pub view_angle: c_double,
    pub view_direction: PK_VECTOR_t,
    pub spin_angle: c_double,
    pub spin_direction: PK_VECTOR_t,
    pub clipping_body: PK_BODY_t,
    pub tolerance: c_double,
    pub want_body: PK_LOGICAL_t,
    pub body_dimension: PK_TOPOL_dimension_t,
    pub keep_as_facet: PK_outline_keep_as_facet_t,
}

/// Return structure for `PK_BODY_make_persp_outline`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_persp_outline_r_t {
    pub n_curves: c_int,
    pub curves: *mut PK_CURVE_t,
    pub intervals: *mut PK_INTERVAL_t,
    pub topols: *mut PK_TOPOL_t,
    pub outlines: *mut c_int,
    pub curve_tolerances: *mut c_double,
    pub max_separation: c_double,
}

// =============================================================================
// Ch. 35 — Extrusion: Bound types
// =============================================================================

pub type PK_bound_t = c_int;
/// Bound by distance from profile (default).
pub const PK_bound_distance_c: PK_bound_t = 0;
/// Bound by surface intersection.
pub const PK_bound_surf_c: PK_bound_t = 1;
/// Bound by face intersection.
pub const PK_bound_face_c: PK_bound_t = 2;
/// Bound by body intersection.
pub const PK_bound_body_c: PK_bound_t = 3;
/// Bound by sheet body (sheet is destroyed).
pub const PK_bound_sheet_c: PK_bound_t = 4;
/// No bound (only with extruded_body).
pub const PK_bound_none_c: PK_bound_t = 5;

// =============================================================================
// Extrusion: Bound side
// =============================================================================

pub type PK_bound_side_t = c_int;
/// Inside of bounding entity in first division.
pub const PK_bound_side_in_c: PK_bound_side_t = 0;
/// Outside of bounding entity in first division.
pub const PK_bound_side_out_c: PK_bound_side_t = 1;
/// Both sides in first division.
pub const PK_bound_side_both_c: PK_bound_side_t = 2;

// =============================================================================
// Extrusion: keep_as_facet for extrusion
// =============================================================================

pub type PK_extrude_keep_as_facet_t = c_int;
pub const PK_extrude_keep_as_facet_no_c: PK_extrude_keep_as_facet_t = 0;
pub const PK_extrude_keep_as_facet_yes_c: PK_extrude_keep_as_facet_t = 1;

// =============================================================================
// Extrusion: bound specification (start or end)
// =============================================================================

/// Bound specification for one end of an extrusion.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_extrude_bound_t {
    pub bound: PK_bound_t,
    pub forward: PK_LOGICAL_t,
    pub distance: c_double,
    pub entity: PK_ENTITY_t,
    pub nearest: PK_LOGICAL_t,
    pub nth_division: c_int,
    pub side: PK_bound_side_t,
}

/// Options for `PK_BODY_extrude`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_extrude_o_t {
    pub start_bound: PK_BODY_extrude_bound_t,
    pub end_bound: PK_BODY_extrude_bound_t,
    pub extruded_body: PK_BODY_t,
    pub allow_disjoint: PK_LOGICAL_t,
    pub consistent_params: PK_PARAM_consistent_t,
    pub have_pline_angle: PK_LOGICAL_t,
    pub pline_angle: c_double,
    pub keep_as_facet: PK_extrude_keep_as_facet_t,
}

// =============================================================================
// Ch. 36 — Sweeping: Alignment
// =============================================================================

pub type PK_BODY_sweep_align_t = c_int;
/// Follow path normal (default).
pub const PK_BODY_sweep_align_normal_c: PK_BODY_sweep_align_t = 0;
/// Constant orientation.
pub const PK_BODY_sweep_align_parallel_c: PK_BODY_sweep_align_t = 1;
/// Match by parametric distance (guide wires only).
pub const PK_BODY_sweep_align_parm_c: PK_BODY_sweep_align_t = 2;
/// Match by arc-length (guide wires only).
pub const PK_BODY_sweep_align_arclength_c: PK_BODY_sweep_align_t = 3;

// =============================================================================
// Sweeping: Lock type
// =============================================================================

pub type PK_sweep_lock_t = c_int;
/// Fix to both path and lock direction (default).
pub const PK_sweep_lock_path_and_dir_c: PK_sweep_lock_t = 0;
/// Fix to path only.
pub const PK_sweep_lock_path_c: PK_sweep_lock_t = 1;
/// Rotation lock.
pub const PK_sweep_lock_rotation_c: PK_sweep_lock_t = 2;

// =============================================================================
// Sweeping: Profile law
// =============================================================================

pub type PK_sweep_profile_law_t = c_int;
/// Profiles untransformed (default).
pub const PK_sweep_profile_law_no_c: PK_sweep_profile_law_t = 0;
/// Profiles pre-transformed.
pub const PK_sweep_profile_law_yes_c: PK_sweep_profile_law_t = 1;

// =============================================================================
// Sweeping: Scale type
// =============================================================================

pub type PK_BODY_sweep_scale_t = c_int;
/// Vary both position and size (default).
pub const PK_BODY_sweep_scale_both_c: PK_BODY_sweep_scale_t = 0;
/// Vary position only.
pub const PK_BODY_sweep_scale_posn_c: PK_BODY_sweep_scale_t = 1;
/// Vary size only.
pub const PK_BODY_sweep_scale_size_c: PK_BODY_sweep_scale_t = 2;

// =============================================================================
// Sweeping: Cross-section output
// =============================================================================

pub type PK_sweep_output_xsect_t = c_int;
/// Produce body (default).
pub const PK_sweep_output_xsect_no_c: PK_sweep_output_xsect_t = 0;
/// Produce cross-sections.
pub const PK_sweep_output_xsect_yes_c: PK_sweep_output_xsect_t = 1;
/// Body first, cross-sections on failure.
pub const PK_sweep_output_xsect_on_fail_c: PK_sweep_output_xsect_t = 2;

// =============================================================================
// Sweeping: Cross-section grouping
// =============================================================================

pub type PK_sweep_group_xsect_t = c_int;
/// n_xsects along entire path.
pub const PK_sweep_group_xsect_no_c: PK_sweep_group_xsect_t = 0;
/// n_xsects per path edge.
pub const PK_sweep_group_xsect_per_edge_c: PK_sweep_group_xsect_t = 1;
/// One per path vertex.
pub const PK_sweep_group_xsect_per_vx_c: PK_sweep_group_xsect_t = 2;

// =============================================================================
// Sweeping/Lofting shared: Topology form
// =============================================================================

/// Sweep topology form enum (not to be confused with `enquiry::PK_sweep_topology_form_t` result struct).
pub type PK_sweep_topology_form_t = c_int;

/// Minimum faces.
pub const PK_BODY_topology_minimal_c: PK_sweep_topology_form_t = 0;
/// Columnar topology.
pub const PK_BODY_topology_columns_c: PK_sweep_topology_form_t = 1;
/// Grid topology.
pub const PK_BODY_topology_grid_c: PK_sweep_topology_form_t = 2;

// =============================================================================
// Sweeping/Lofting shared: Simplification
// =============================================================================

pub type PK_BODY_simplify_t = c_int;
/// Simplify to analytics (default).
pub const PK_BODY_simplify_analytic_c: PK_BODY_simplify_t = 0;
/// Also try swept/spun surfaces.
pub const PK_BODY_simplify_swept_spun_c: PK_BODY_simplify_t = 1;
/// No simplification.
pub const PK_BODY_simplify_no_c: PK_BODY_simplify_t = 2;

// =============================================================================
// Sweeping: Corner type
// =============================================================================

pub type PK_sweep_corner_type_t = c_int;
/// Mitred corners (default for sweep).
pub const PK_sweep_corner_type_mitre_c: PK_sweep_corner_type_t = 0;
/// Rounded corners (default for swept tool).
pub const PK_sweep_corner_type_spin_c: PK_sweep_corner_type_t = 1;

// =============================================================================
// Sweeping: Self-intersection repair
// =============================================================================

pub type PK_sweep_repair_t = c_int;
/// No repair (default).
pub const PK_sweep_repair_no_c: PK_sweep_repair_t = 0;
/// Repair local self-intersections.
pub const PK_sweep_repair_yes_c: PK_sweep_repair_t = 1;
/// Repair and report repaired faces.
pub const PK_sweep_repair_report_c: PK_sweep_repair_t = 2;

// =============================================================================
// Sweeping: Derivative magnitude
// =============================================================================

pub type PK_sweep_deriv_mag_t = c_int;
/// Smooth magnitude variation (default).
pub const PK_sweep_deriv_mag_smooth_c: PK_sweep_deriv_mag_t = 0;
/// Rounded magnitude variation.
pub const PK_sweep_deriv_mag_round_c: PK_sweep_deriv_mag_t = 1;

// =============================================================================
// Sweeping: Profile clamp type
// =============================================================================

pub type PK_sweep_clamp_t = c_int;
/// No clamp (default).
pub const PK_sweep_clamp_none_c: PK_sweep_clamp_t = 0;

// =============================================================================
// Sweeping: Guide scope
// =============================================================================

pub type PK_sweep_guide_scope_t = c_int;
/// Each guide influences whole sweep (default, max 3 guides).
pub const PK_sweep_guide_scope_global_c: PK_sweep_guide_scope_t = 0;
/// Each guide influences only neighbor region (3+ guides).
pub const PK_sweep_guide_scope_local_c: PK_sweep_guide_scope_t = 1;

// =============================================================================
// Sweeping: Guide method
// =============================================================================

pub type PK_sweep_guide_method_t = c_int;
/// Rotate and scale to contact guides (default).
pub const PK_sweep_guide_point_c: PK_sweep_guide_method_t = 0;
/// Rotate only, one point on guide line.
pub const PK_sweep_guide_chord_c: PK_sweep_guide_method_t = 1;
/// Rotate only, profile slides along guide.
pub const PK_sweep_guide_curve_c: PK_sweep_guide_method_t = 2;
/// Rotate and scale, fixed orientation to lock direction.
pub const PK_sweep_guide_project_c: PK_sweep_guide_method_t = 3;

// =============================================================================
// Sweeping: Guide scale
// =============================================================================

pub type PK_sweep_guide_scale_t = c_int;
/// Uniform 2D scaling (default).
pub const PK_sweep_guide_uniform_c: PK_sweep_guide_scale_t = 0;
/// Scale only between guide points.
pub const PK_sweep_guide_lateral_c: PK_sweep_guide_scale_t = 1;

// =============================================================================
// Sweeping: Guide clamp type
// =============================================================================

pub type PK_sweep_guide_clamp_t = c_int;
/// Direction clamp.
pub const PK_sweep_guide_clamp_dirn_c: PK_sweep_guide_clamp_t = 0;
/// Fixed clamp (preserve profile-guide relationship).
pub const PK_sweep_guide_clamp_fixed_c: PK_sweep_guide_clamp_t = 1;

// =============================================================================
// Sweeping: Fault status
// =============================================================================

pub type PK_BODY_sweep_fault_t = c_int;
/// Closed path sweep fails due to torsion mismatch.
pub const PK_BODY_sweep_torsion_failure_c: PK_BODY_sweep_fault_t = 1;
/// Profile too small to span guide wires.
pub const PK_BODY_sweep_bad_path_c: PK_BODY_sweep_fault_t = 2;

// =============================================================================
// Sweeping: Update control
// =============================================================================

pub type PK_BODY_sweep_update_t = c_int;
/// Use all enhancements (default).
pub const PK_BODY_sweep_update_default_c: PK_BODY_sweep_update_t = 0;

// =============================================================================
// Sweeping: Law specification (twist/scale)
// =============================================================================

/// Law definition for twist or scale along a sweep path.
/// In the Parasolid API this is a union-like structure; we represent
/// the most common case (a simple constant or linear law given by values).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_LAW_sf_t {
    /// Law type discriminator.
    pub law_type: c_int,
    /// Number of values.
    pub n_values: c_int,
    /// Law values array.
    pub values: *const c_double,
}

// =============================================================================
// Sweeping: Profile derivative (clamp) specification
// =============================================================================

/// Per-profile derivative clamp.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_sweep_profile_deriv_t {
    pub clamp_type: PK_sweep_clamp_t,
    pub direction: PK_VECTOR_t,
    pub magnitude: c_double,
}

// =============================================================================
// Sweeping: Vertex match specification
// =============================================================================

/// Vertex mapping between profiles (used in sweep and loft).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_sweep_matches_t {
    pub n_matches: c_int,
    pub profile_indices: *const c_int,
    pub n_vertices: *const c_int,
    pub vertices: *const *const PK_VERTEX_t,
}

// =============================================================================
// Sweeping: Guide control specification
// =============================================================================

/// Guide wire control for sweep operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_sweep_guide_controls_t {
    pub n_guide_vertices: c_int,
    pub guide_vertices: *const PK_VERTEX_t,
    pub n_guide_matches: c_int,
    pub guide_match_indices: *const c_int,
    pub guide_match_vertices: *const PK_VERTEX_t,
    pub n_guide_clamps: c_int,
    pub guide_clamp_types: *const PK_sweep_guide_clamp_t,
    pub guide_clamp_directions: *const PK_VECTOR_t,
    pub guide_clamp_magnitudes: *const c_double,
}

// =============================================================================
// Ch. 36 — Sweep options and return structures
// =============================================================================

/// Options for `PK_BODY_make_swept_body_2`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BODY_make_swept_body_2_o_t {
    // Profile control
    pub alignment: PK_BODY_sweep_align_t,
    pub have_lock_direction: PK_LOGICAL_t,
    pub lock_direction: PK_VECTOR_t,
    pub lock_type: PK_sweep_lock_t,
    pub n_lock_faces: c_int,
    pub lock_faces: *const PK_FACE_t,

    // Twist control
    pub twist: PK_LAW_sf_t,
    pub profile_law: PK_sweep_profile_law_t,
    pub have_twist_direction: PK_LOGICAL_t,
    pub twist_direction: PK_VECTOR_t,

    // Scale control
    pub scale: PK_LAW_sf_t,
    pub scale_type: PK_BODY_sweep_scale_t,
    pub scale_point: PK_VECTOR_t,

    // Cross-section output
    pub output_xsect: PK_sweep_output_xsect_t,
    pub group_xsect: PK_sweep_group_xsect_t,
    pub n_xsects: c_int,

    // Topology and surface control
    pub topology_form: PK_sweep_topology_form_t,
    pub simplify: PK_BODY_simplify_t,
    pub preferred_continuity: PK_continuity_t,
    pub n_ignorable_vertices: c_int,
    pub ignorable_vertices: *const PK_VERTEX_t,

    // Tolerance and quality
    pub tolerance: c_double,
    pub minimise_tolerance: PK_LOGICAL_t,
    pub allow_rationals: PK_LOGICAL_t,

    // Self-intersection repair
    pub repair: PK_sweep_repair_t,

    // Profile clamps
    pub n_profile_derivs: c_int,
    pub profile_derivs: *const PK_BODY_sweep_profile_deriv_t,
    pub profile_indices: *const c_int,
    pub deriv_mag: PK_sweep_deriv_mag_t,

    // Corner control
    pub corner_type: PK_sweep_corner_type_t,

    // Guide wire control
    pub n_guides: c_int,
    pub guides: *const PK_BODY_t,
    pub guide_controls: PK_BODY_sweep_guide_controls_t,
    pub guide_scope: PK_sweep_guide_scope_t,
    pub guide_method: PK_sweep_guide_method_t,
    pub guide_scale: PK_sweep_guide_scale_t,
    pub trim_to_guides: PK_LOGICAL_t,
    pub have_trim_point: PK_LOGICAL_t,
    pub trim_point: PK_VECTOR_t,
    pub fixed_guide_index: c_int,

    // Tracking
    pub want_edge_tracking: PK_LOGICAL_t,

    // Update
    pub update: PK_BODY_sweep_update_t,

    // Matching
    pub matches: PK_BODY_sweep_matches_t,
}

/// Return structure for `PK_BODY_make_swept_body_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_swept_body_2_r_t {
    pub body: PK_BODY_t,
    pub tracking: PK_TOPOL_tracking_t,
    pub fault_status: PK_BODY_sweep_fault_t,
    pub n_faults: c_int,
    pub faults: *mut PK_TOPOL_t,
}

// =============================================================================
// Ch. 37 — Swept tool: Boolean operation enum
// =============================================================================

pub type PK_sweep_boolean_t = c_int;
/// No boolean (default).
pub const PK_sweep_boolean_none_c: PK_sweep_boolean_t = 0;
/// Subtract swept tool from target.
pub const PK_sweep_boolean_subtract_c: PK_sweep_boolean_t = 1;
/// Intersect swept tool with target.
pub const PK_sweep_boolean_intersect_c: PK_sweep_boolean_t = 2;

// =============================================================================
// Swept tool: Cap face report types
// =============================================================================

pub type PK_REPORT_3_sweep_tool_cap_t = c_int;
pub const PK_sweep_tool_cap_enclose_c: PK_REPORT_3_sweep_tool_cap_t = 0;
pub const PK_sweep_tool_cap_undercut_c: PK_REPORT_3_sweep_tool_cap_t = 1;

// =============================================================================
// Swept tool: Update control
// =============================================================================

pub type PK_swept_tool_update_t = c_int;
/// Use all enhancements (default).
pub const PK_swept_tool_update_default_c: PK_swept_tool_update_t = 0;

// =============================================================================
// Ch. 37 — Swept tool options and return structures
// =============================================================================

/// Options for `PK_BODY_make_swept_tool`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_swept_tool_o_t {
    pub tolerance: c_double,
    pub allow_rationals: PK_LOGICAL_t,
    pub want_edge_tracking: PK_LOGICAL_t,
    pub have_lock_direction: PK_LOGICAL_t,
    pub lock_direction: PK_VECTOR_t,
    pub tool_site: PK_VERTEX_t,
    pub n_cap_faces: c_int,
    pub cap_faces: *const PK_FACE_t,
    pub place_tool_on_path: PK_LOGICAL_t,
    pub sweep_boolean: PK_sweep_boolean_t,
    pub target: PK_BODY_t,
    pub corner_type: PK_sweep_corner_type_t,
    pub update: PK_swept_tool_update_t,
}

/// Return structure for `PK_BODY_make_swept_tool`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_swept_tool_r_t {
    pub body: PK_BODY_t,
    pub status: c_int,
    pub n_faults: c_int,
    pub faults: *mut PK_TOPOL_t,
}

// =============================================================================
// Ch. 38 — Lofting: Derivative conditions
// =============================================================================

// Curvature condition
pub type PK_BODY_loft_curvature_t = c_int;
/// No curvature constraint (default, recommended).
pub const PK_BODY_loft_unconstrained_c: PK_BODY_loft_curvature_t = 0;
/// Zero curvature (start/end only, no clamps/guides).
pub const PK_BODY_loft_natural_c: PK_BODY_loft_curvature_t = 1;
/// Constrain curvature per face clamp (G2-continuous).
pub const PK_BODY_loft_clamped_c: PK_BODY_loft_curvature_t = 2;

// Clamp type
pub type PK_BODY_loft_clamp_t = c_int;
/// No clamp.
pub const PK_BODY_loft_clamp_none_c: PK_BODY_loft_clamp_t = 0;
/// Vector clamp.
pub const PK_BODY_loft_clamp_vector_c: PK_BODY_loft_clamp_t = 1;
/// Face clamp.
pub const PK_BODY_loft_clamp_face_c: PK_BODY_loft_clamp_t = 2;
/// Planar clamp.
pub const PK_BODY_loft_clamp_planar_c: PK_BODY_loft_clamp_t = 3;
/// Composite: vector + face.
pub const PK_BODY_loft_clamp_vec_face_c: PK_BODY_loft_clamp_t = 4;
/// Composite: vector + planar.
pub const PK_BODY_loft_clamp_vec_planar_c: PK_BODY_loft_clamp_t = 5;

// =============================================================================
// Lofting: Derivative magnitude
// =============================================================================

pub type PK_BODY_loft_deriv_mag_t = c_int;
/// Single magnitude across smooth sections (default).
pub const PK_BODY_loft_deriv_mag_single_c: PK_BODY_loft_deriv_mag_t = 0;
/// Vary smoothly across sections.
pub const PK_BODY_loft_deriv_mag_smooth_c: PK_BODY_loft_deriv_mag_t = 1;
/// Vary smoothly for rounded shape (recommended).
pub const PK_BODY_loft_deriv_mag_round_c: PK_BODY_loft_deriv_mag_t = 2;

// =============================================================================
// Lofting: Profile smoothness
// =============================================================================

pub type PK_BODY_smoothness_t = c_int;
/// G1 only if within session angle precision.
pub const PK_BODY_smoothness_exact_c: PK_BODY_smoothness_t = 0;
/// G1 if within relaxed (visually smooth) angle.
pub const PK_BODY_smoothness_relax_c: PK_BODY_smoothness_t = 1;

// =============================================================================
// Lofting: Update control
// =============================================================================

pub type PK_BODY_loft_update_t = c_int;
/// Use all enhancements (default).
pub const PK_BODY_loft_update_default_c: PK_BODY_loft_update_t = 0;

// =============================================================================
// Lofting: Vector clamp specification
// =============================================================================

/// Vector clamp data for loft derivative conditions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_loft_vector_clamp_t {
    pub n_tangents: c_int,
    pub tangent_vertices: *const PK_VERTEX_t,
    pub tangent_directions: *const PK_VECTOR_t,
    pub tangent_magnitudes: *const c_double,
}

/// Face clamp data for loft derivative conditions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_loft_face_clamp_t {
    pub face: PK_FACE_t,
    pub magnitude: c_double,
}

/// Planar clamp data for loft derivative conditions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_loft_planar_clamp_t {
    pub direction: PK_VECTOR_t,
    pub magnitude: c_double,
}

// =============================================================================
// Lofting: Derivative conditions structure
// =============================================================================

/// Derivative conditions for a loft profile.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_loft_deriv_conds_t {
    pub curvature_condition: PK_BODY_loft_curvature_t,
    pub clamp_type: PK_BODY_loft_clamp_t,
    pub vector_clamp: PK_BODY_loft_vector_clamp_t,
    pub face_clamp: PK_BODY_loft_face_clamp_t,
    pub planar_clamp: PK_BODY_loft_planar_clamp_t,
}

// =============================================================================
// Lofting: End conditions structure
// =============================================================================

/// End conditions for a loft (start, end, periodic).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_loft_end_conditions_t {
    pub periodic: PK_PARAM_periodic_t,
    pub start: PK_BODY_loft_deriv_conds_t,
    pub end: PK_BODY_loft_deriv_conds_t,
}

// =============================================================================
// Lofting: Match specification
// =============================================================================

/// Vertex matching between loft profiles.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_loft_matches_t {
    pub n_matches: c_int,
    pub profile_indices: *const c_int,
    pub n_vertices: *const c_int,
    pub vertices: *const *const PK_VERTEX_t,
}

// =============================================================================
// Ch. 38 — Loft options and return structures
// =============================================================================

/// Options for `PK_BODY_make_lofted_body`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BODY_make_lofted_body_o_t {
    // End conditions
    pub end_conditions: PK_BODY_loft_end_conditions_t,

    // Intermediate derivative conditions
    pub n_intermediate_derivs: c_int,
    pub intermediate_derivs: *const PK_BODY_loft_deriv_conds_t,
    pub intermediate_profiles: *const c_int,

    // Guide wires
    pub n_guide_wires: c_int,
    pub guide_wires: *const PK_BODY_t,
    pub n_guide_derivs: c_int,
    pub guide_derivs: *const PK_BODY_loft_deriv_conds_t,
    pub guide_indices: *const c_int,

    // Matching
    pub matches: PK_BODY_loft_matches_t,

    // Surface and topology
    pub topology_form: PK_sweep_topology_form_t,
    pub simplify: PK_BODY_simplify_t,
    pub profile_smoothness: PK_BODY_smoothness_t,
    pub deriv_mag: PK_BODY_loft_deriv_mag_t,
    pub tolerance: c_double,
    pub minimise_tolerance: PK_LOGICAL_t,
    pub create_construction_topol: PK_LOGICAL_t,

    // Tracking
    pub want_edge_tracking: PK_LOGICAL_t,
    pub label_profiles: PK_LOGICAL_t,

    // Update
    pub update: PK_BODY_loft_update_t,
}

/// Return structure for `PK_BODY_make_lofted_body`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_lofted_body_r_t {
    pub body: PK_BODY_t,
    pub tracking: PK_TOPOL_tracking_t,
}

// =============================================================================
// Ch. 39 — Shadow curves: Face checking
// =============================================================================

pub type PK_shadow_check_fa_t = c_int;
/// No checks (default, legacy behavior).
pub const PK_shadow_check_fa_no_c: PK_shadow_check_fa_t = 0;
/// Check faces local to imprint (recommended).
pub const PK_shadow_check_fa_yes_c: PK_shadow_check_fa_t = 1;

// =============================================================================
// Ch. 39 — Shadow curve options
// =============================================================================

/// Options for `PK_BODY_imprint_cus_shadow`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_imprint_cus_shadow_o_t {
    pub want_edges: PK_LOGICAL_t,
    pub want_visible_faces: PK_LOGICAL_t,
    pub check_faces: PK_shadow_check_fa_t,
}

// =============================================================================
// Ch. 40 — Emboss: Convexity
// =============================================================================

pub type PK_emboss_convexity_t = c_int;
/// Create both pads and pockets (default).
pub const PK_emboss_convexity_both_c: PK_emboss_convexity_t = 0;
/// Create pads only.
pub const PK_emboss_convexity_pad_c: PK_emboss_convexity_t = 1;
/// Create pockets only.
pub const PK_emboss_convexity_pocket_c: PK_emboss_convexity_t = 2;

// =============================================================================
// Emboss: Profile location
// =============================================================================

pub type PK_emboss_profile_on_t = c_int;
/// Profile can be anywhere (default).
pub const PK_emboss_profile_on_any_c: PK_emboss_profile_on_t = 0;

// =============================================================================
// Emboss: Sidewall type
// =============================================================================

pub type PK_emboss_sidewall_t = c_int;
/// Tapered sidewalls (draw direction + taper angle).
pub const PK_emboss_sidewall_tapered_c: PK_emboss_sidewall_t = 0;
/// Ruled sidewalls (along profile face normals).
pub const PK_emboss_sidewall_ruled_c: PK_emboss_sidewall_t = 1;
/// Swept sidewalls (along draw direction).
pub const PK_emboss_sidewall_swept_c: PK_emboss_sidewall_t = 2;
/// User-supplied sidewall body.
pub const PK_emboss_sidewall_supplied_c: PK_emboss_sidewall_t = 3;

// =============================================================================
// Emboss: Taper method
// =============================================================================

pub type PK_taper_method_t = c_int;
/// Isocline taper (default).
pub const PK_taper_method_isocline_c: PK_taper_method_t = 0;
/// Normal taper (relative to profile face normals).
pub const PK_taper_method_normal_c: PK_taper_method_t = 1;
/// Offset taper.
pub const PK_taper_method_offset_c: PK_taper_method_t = 2;
/// Curve taper.
pub const PK_taper_method_curve_c: PK_taper_method_t = 3;

// =============================================================================
// Emboss: Overflow
// =============================================================================

pub type PK_emboss_overflow_t = c_int;
/// Extend emboss to include additional faces (default interior).
pub const PK_emboss_overflow_added_c: PK_emboss_overflow_t = 0;
/// Trim at concave, extend at convex edges.
pub const PK_emboss_overflow_mixed_c: PK_emboss_overflow_t = 1;
/// Fail if overflow detected.
pub const PK_emboss_overflow_none_c: PK_emboss_overflow_t = 2;
/// Treat interior overflow as laminar (only with unite=no).
pub const PK_emboss_overflow_laminar_c: PK_emboss_overflow_t = 3;
/// Trim with ruled surfaces at laminar edges.
pub const PK_emboss_overflow_ruled_c: PK_emboss_overflow_t = 4;
/// Trim with swept surfaces at laminar edges (default laminar).
pub const PK_emboss_overflow_swept_c: PK_emboss_overflow_t = 5;

// =============================================================================
// Emboss: Unite control
// =============================================================================

pub type PK_emboss_unite_t = c_int;
/// Attach emboss to target (default).
pub const PK_emboss_unite_sidewall_yes_c: PK_emboss_unite_t = 0;
/// Return emboss separately; target unmodified.
pub const PK_emboss_unite_sidewall_no_c: PK_emboss_unite_t = 1;

// =============================================================================
// Emboss: Local ops update control
// =============================================================================

// =============================================================================
// Emboss: Sidewall data structure
// =============================================================================

/// Sidewall construction data for emboss operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_emboss_sidewall_data_t {
    pub sidewall: PK_emboss_sidewall_t,
    pub taper_method: PK_taper_method_t,
    pub draw_direction: PK_VECTOR_t,
    pub taper_angle: c_double,
    pub offset: c_double,
    pub top_surface: PK_ENTITY_t,
    pub sidewall_body: PK_BODY_t,
    pub n_multi_taper_edges: c_int,
    pub multi_taper_edges: *const PK_EDGE_t,
    pub multi_taper_angles: *const c_double,
}

// =============================================================================
// Emboss: Overflow data structure
// =============================================================================

/// Overflow behavior for emboss operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_emboss_overflow_data_t {
    pub interior_overflow: PK_emboss_overflow_t,
    pub laminar_overflow: PK_emboss_overflow_t,
    pub sweep_direction: PK_VECTOR_t,
    pub laminar_walled: PK_LOGICAL_t,
}

// =============================================================================
// Ch. 40 — Emboss options structures
// =============================================================================

/// Options for `PK_BODY_emboss` (global emboss).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_emboss_o_t {
    pub tolerance: c_double,
    pub convexity: PK_emboss_convexity_t,
    pub profile_on: PK_emboss_profile_on_t,
    pub sidewall_data: PK_emboss_sidewall_data_t,
    pub overflow_data: PK_emboss_overflow_data_t,
    pub unite: PK_emboss_unite_t,
    pub update: PK_local_ops_update_t,
}

/// Options for `PK_FACE_emboss` (local emboss).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_emboss_o_t {
    pub tolerance: c_double,
    pub convexity: PK_emboss_convexity_t,
    pub profile_on: PK_emboss_profile_on_t,
    pub sidewall_data: PK_emboss_sidewall_data_t,
    pub overflow_data: PK_emboss_overflow_data_t,
    pub unite: PK_emboss_unite_t,
    pub update: PK_local_ops_update_t,
}

// =============================================================================
// Opaque options/result types for new sweep functions
// =============================================================================

/// Options for `PK_BODY_make_swept_profiles`.
#[repr(C)]
pub struct PK_BODY_make_swept_profiles_o_t { _private: [u8; 0] }

/// Results from tracked sweep operations.
#[repr(C)]
pub struct PK_BODY_tracked_sweep_2_r_t { _private: [u8; 0] }

// =============================================================================
// Extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // Ch. 34 — Profiling
    // =========================================================================

    /// Creates outline curves from body boundaries as seen from a view direction.
    pub fn PK_BODY_make_curves_outline(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        transfs: *const PK_TRANSF_t,
        view_direction: *const c_double,
        options: *const PK_BODY_make_curves_outline_o_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        intervals: *mut *mut PK_INTERVAL_t,
        topols: *mut *mut PK_TOPOL_t,
        outlines: *mut *mut c_int,
        curve_tolerances: *mut *mut c_double,
        max_separation: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Creates perspective outline curves from bodies as seen from an eye position.
    pub fn PK_BODY_make_persp_outline(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        transfs: *const PK_TRANSF_t,
        eye_position: *const c_double,
        options: *const PK_BODY_make_persp_outline_o_t,
        result: *mut PK_BODY_make_persp_outline_r_t,
        tracking: *mut PK_TOPOL_tracking_t,
    ) -> PK_ERROR_code_t;

    /// Free function for `PK_BODY_make_persp_outline_r_t`.
    pub fn PK_BODY_make_persp_outline_r_f(
        result: *mut PK_BODY_make_persp_outline_r_t,
    ) -> PK_ERROR_code_t;

    /// Creates spun outline curves for solid bodies rotated about an axis.
    pub fn PK_BODY_make_spun_outline(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        transfs: *const PK_TRANSF_t,
        spin_axis: PK_AXIS1_sf_t,
        options: *const PK_BODY_make_spun_outline_o_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        intervals: *mut *mut PK_INTERVAL_t,
        topols: *mut *mut PK_TOPOL_t,
        outlines: *mut *mut c_int,
        curve_tolerances: *mut *mut c_double,
        max_separation: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ch. 35 — Extrusion
    // =========================================================================

    /// Creates a body by linear extrusion of a profile body.
    pub fn PK_BODY_extrude(
        profile: PK_BODY_t,
        path: *const c_double,
        options: *const PK_BODY_extrude_o_t,
        tracking: *mut PK_TOPOL_tracking_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ch. 36 — Sweeping
    // =========================================================================

    /// Primary sweep: sweeps profiles along a path to create sheet or solid bodies.
    pub fn PK_BODY_make_swept_body_2(
        n_profiles: c_int,
        profiles: *const PK_BODY_t,
        path: PK_BODY_t,
        n_path_vertices: c_int,
        path_vertices: *const PK_VERTEX_t,
        options: *const PK_BODY_make_swept_body_2_o_t,
        returns: *mut PK_BODY_make_swept_body_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Legacy sweep (v1 API, retained for backwards compatibility).
    pub fn PK_BODY_make_swept_body(
        n_profiles: c_int,
        profiles: *const PK_BODY_t,
        path: PK_BODY_t,
        n_path_vertices: c_int,
        path_vertices: *const PK_VERTEX_t,
        options: *const PK_BODY_make_swept_body_2_o_t,
        returns: *mut PK_BODY_make_swept_body_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Creates a helical curve by sweeping a point about an axis.
    pub fn PK_POINT_make_helical_curve(
        point: *const c_double,
        axis: PK_AXIS1_sf_t,
        hand: PK_HAND_t,
        turns: PK_INTERVAL_t,
        helical_pitch: c_double,
        spiral_pitch: c_double,
        tolerance: c_double,
        curve: *mut PK_CURVE_t,
        interval: *mut PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    /// Creates a helical surface by sweeping a curve about an axis.
    pub fn PK_CURVE_make_helical_surf(
        curve: PK_CURVE_t,
        curve_interval: PK_INTERVAL_t,
        axis: PK_AXIS1_sf_t,
        hand: PK_HAND_t,
        turns: PK_INTERVAL_t,
        helical_pitch: c_double,
        spiral_pitch: c_double,
        tolerance: c_double,
        surface: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ch. 37 — Swept tool
    // =========================================================================

    /// Sweeps a solid convex tool body along a path (CAM workflow).
    pub fn PK_BODY_make_swept_tool(
        tool: PK_BODY_t,
        tool_axis: PK_AXIS1_sf_t,
        path: PK_BODY_t,
        options: *const PK_BODY_make_swept_tool_o_t,
        tracking: *mut PK_TOPOL_tracking_t,
        swept_tool: *mut PK_BODY_make_swept_tool_r_t,
    ) -> PK_ERROR_code_t;

    /// Free function for `PK_BODY_make_swept_tool_r_t`.
    pub fn PK_BODY_sweep_tool_r_f(
        result: *mut PK_BODY_make_swept_tool_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ch. 38 — Lofting
    // =========================================================================

    /// Creates a sheet or solid body by fitting surfaces through profiles.
    pub fn PK_BODY_make_lofted_body(
        n_profiles: c_int,
        profiles: *const PK_BODY_t,
        start_vertices: *const PK_VERTEX_t,
        options: *const PK_BODY_make_lofted_body_o_t,
        lofted_body: *mut PK_BODY_make_lofted_body_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ch. 39 — Shadow curves
    // =========================================================================

    /// Splits faces into visible/invisible regions from a view direction.
    pub fn PK_BODY_imprint_cus_shadow(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        transfs: *const PK_TRANSF_t,
        view_direction: *const c_double,
        options: *const PK_BODY_imprint_cus_shadow_o_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_visible_faces: *mut c_int,
        visible_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ch. 40 — Emboss
    // =========================================================================

    /// Global emboss: adds pad or pocket features across a whole target body.
    pub fn PK_BODY_emboss(
        target: PK_BODY_t,
        profile: PK_BODY_t,
        end_cap: PK_BODY_t,
        options: *const PK_BODY_emboss_o_t,
        tracking: *mut PK_TOPOL_tracking_t,
    ) -> PK_ERROR_code_t;

    /// Local emboss: adds emboss features to selected faces within a body.
    pub fn PK_FACE_emboss(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        profile: PK_BODY_t,
        end_cap: PK_BODY_t,
        options: *const PK_FACE_emboss_o_t,
        tracking: *mut PK_TOPOL_tracking_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ch. 36 — Additional sweep functions
    // =========================================================================

    /// Create swept body from profiles along path.
    pub fn PK_BODY_make_swept_profiles(
        n_profiles: c_int,
        profiles: *const PK_BODY_t,
        path: PK_BODY_t,
        options: *const PK_BODY_make_swept_profiles_o_t,
        results: *mut PK_BODY_tracked_sweep_2_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_BODY_make_swept_profiles`.
    pub fn PK_BODY_make_swept_profiles_r_f(results: *mut PK_BODY_tracked_sweep_2_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_BODY_make_swept_body_2`.
    pub fn PK_BODY_make_swept_body_2_r_f(results: *mut PK_BODY_tracked_sweep_2_r_t) -> PK_ERROR_code_t;

}
