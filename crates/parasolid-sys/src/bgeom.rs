#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! B-curve and B-surface (NURBS) creation, query, and modification.
//!
//! Covers `PK_BCURVE_*`, `PK_BSURF_*`, and related conversion/splining functions
//! from the Parasolid PK C API (Chapter 18).

use crate::*;
use std::os::raw::{c_double, c_int};

// =============================================================================
// Standard form structs
// =============================================================================

/// Standard form of a B-curve (NURBS curve).
///
/// - `vertex_dim` = 3 for non-rational, 4 for rational (homogeneous coords)
/// - `n_knots = n_vertices + degree + 1` (expanded knot vector length)
/// - Rational vertices are stored as `(x*w, y*w, z*w, w)`
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BCURVE_sf_t {
    pub degree: c_int,
    pub n_vertices: c_int,
    pub vertex_dim: c_int,
    /// Control vertices: length = `n_vertices * vertex_dim`
    pub vertices: *const c_double,
    pub n_knots: c_int,
    /// Knot vector: length = `n_knots`
    pub knots: *const c_double,
    pub is_rational: PK_LOGICAL_t,
    pub is_periodic: PK_LOGICAL_t,
    pub is_closed: PK_LOGICAL_t,
}

/// Standard form of a B-surface (NURBS surface).
///
/// Control vertices are stored in column-major order: `cols x rows` grid.
/// - `cols >= u_degree + 1`, `rows >= v_degree + 1`
/// - `vertex_dim` = 3 for non-rational, 4 for rational
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BSURF_sf_t {
    pub u_degree: c_int,
    pub v_degree: c_int,
    pub n_u_vertices: c_int,
    pub n_v_vertices: c_int,
    pub vertex_dim: c_int,
    /// Control vertices: length = `n_u_vertices * n_v_vertices * vertex_dim`
    pub vertices: *const c_double,
    pub n_u_knots: c_int,
    /// U-direction knot vector: length = `n_u_knots`
    pub u_knots: *const c_double,
    pub n_v_knots: c_int,
    /// V-direction knot vector: length = `n_v_knots`
    pub v_knots: *const c_double,
    pub is_rational: PK_LOGICAL_t,
    pub is_u_periodic: PK_LOGICAL_t,
    pub is_v_periodic: PK_LOGICAL_t,
    pub is_u_closed: PK_LOGICAL_t,
    pub is_v_closed: PK_LOGICAL_t,
}

// =============================================================================
// Continuity constants
// =============================================================================

pub type PK_continuity_t = c_int;
pub const PK_continuity_c1_c: PK_continuity_t = 1;

// =============================================================================
// Force continuity constants
// =============================================================================

pub type PK_force_continuity_t = c_int;
pub const PK_force_continuity_no_c: PK_force_continuity_t = 0;

// =============================================================================
// Clamp type constants (PK_BCURVE_create_spline_2)
// =============================================================================

pub type PK_BCURVE_clamp_t = c_int;
/// No clamping.
pub const PK_BCURVE_no_clamp_c: PK_BCURVE_clamp_t = 0;
/// Constant clamping.
pub const PK_BCURVE_clamp_constant_c: PK_BCURVE_clamp_t = 1;
/// Constant + locally extreme clamping.
pub const PK_BCURVE_clamp_extreme_c: PK_BCURVE_clamp_t = 2;

// =============================================================================
// Spline method constants
// =============================================================================

pub type PK_BCURVE_spline_method_t = c_int;
/// Interpolate through positions.
pub const PK_BCURVE_spline_interpolate_c: PK_BCURVE_spline_method_t = 0;
/// Fit positions within tolerance.
pub const PK_BCURVE_spline_fit_c: PK_BCURVE_spline_method_t = 1;

// =============================================================================
// Spline update constants
// =============================================================================

pub type PK_spline_update_t = c_int;
/// Use all enhancements (default).
pub const PK_spline_update_default_c: PK_spline_update_t = 0;

// =============================================================================
// Constrained surface optimization constants
// =============================================================================

pub type PK_constrained_opt_t = c_int;
/// Optimize for performance (default).
pub const PK_constrained_opt_perf_c: PK_constrained_opt_t = 0;
/// Optimize for surface quality.
pub const PK_constrained_opt_smoothness_c: PK_constrained_opt_t = 1;

// =============================================================================
// Constrained surface update constants
// =============================================================================

pub type PK_constrained_update_t = c_int;
/// Use all enhancements (default).
pub const PK_constrained_update_default_c: PK_constrained_update_t = 0;

// =============================================================================
// Curve approximation type constants
// =============================================================================

pub type PK_CURVE_approx_t = c_int;
/// Arc-length parameterisation (default).
pub const PK_CURVE_approx_arclength_c: PK_CURVE_approx_t = 0;
/// Even parameterisation.
pub const PK_CURVE_approx_even_c: PK_CURVE_approx_t = 1;

// =============================================================================
// B-curve extension shape constants
// =============================================================================

pub type PK_BCURVE_extension_shape_t = c_int;
pub const PK_BCURVE_extension_linear_c: PK_BCURVE_extension_shape_t = 0;
pub const PK_BCURVE_extension_soft_c: PK_BCURVE_extension_shape_t = 1;
pub const PK_BCURVE_extension_reflective_c: PK_BCURVE_extension_shape_t = 2;
pub const PK_BCURVE_extension_natural_c: PK_BCURVE_extension_shape_t = 3;
pub const PK_BCURVE_extension_arc_c: PK_BCURVE_extension_shape_t = 4;

// =============================================================================
// B-curve extension type constants
// =============================================================================

pub type PK_BCURVE_extension_type_t = c_int;
/// No extension (default).
pub const PK_BCURVE_extension_none_c: PK_BCURVE_extension_type_t = 0;
/// Extend by arc length distance.
pub const PK_BCURVE_extension_distance_c: PK_BCURVE_extension_type_t = 1;
/// Extend to absolute parameter value.
pub const PK_BCURVE_extension_to_parm_c: PK_BCURVE_extension_type_t = 2;
/// Extend by parameter ratio.
pub const PK_BCURVE_extension_parm_ratio_c: PK_BCURVE_extension_type_t = 3;

// =============================================================================
// Extend closed constants
// =============================================================================

pub type PK_extend_closed_t = c_int;
/// Don't extend closed curves (default).
pub const PK_extend_closed_no_c: PK_extend_closed_t = 0;
/// Allow non-periodic closed curves.
pub const PK_extend_closed_non_periodic_c: PK_extend_closed_t = 1;
/// Allow all closed curves.
pub const PK_extend_closed_yes_c: PK_extend_closed_t = 2;

// =============================================================================
// B-curve extend status
// =============================================================================

pub type PK_BCURVE_extend_status_t = c_int;
/// Returned when a closed curve was not extended.
pub const PK_BCURVE_extend_unextended_c: PK_BCURVE_extend_status_t = 0;

// =============================================================================
// Report constants
// =============================================================================

pub type PK_REPORT_t = c_int;
pub const PK_REPORT_3_discontinuities_c: PK_REPORT_t = 3;

// =============================================================================
// Knot type constants (for piecewise creation)
// =============================================================================

pub type PK_knot_type_t = c_int;
/// B-spline representation.
pub const PK_knot_type_bspline_c: PK_knot_type_t = 0;
/// Bezier representation.
pub const PK_knot_type_bezier_c: PK_knot_type_t = 1;
/// Polynomial coefficients.
pub const PK_knot_type_polynomial_c: PK_knot_type_t = 2;
/// Hermite representation (cubics/bi-cubics only).
pub const PK_knot_type_hermite_c: PK_knot_type_t = 3;
/// Taylor series representation.
pub const PK_knot_type_taylor_c: PK_knot_type_t = 4;

// =============================================================================
// Preferred curve type constants (for isoparam extraction)
// =============================================================================

pub type PK_preferred_curve_type_t = c_int;
pub const PK_preferred_curve_type_default_c: PK_preferred_curve_type_t = 0;

// =============================================================================
// Edge curve direction
// =============================================================================

pub type PK_curve_dir_t = c_int;
pub const PK_curve_dir_forward_c: PK_curve_dir_t = 0;
pub const PK_curve_dir_reverse_c: PK_curve_dir_t = 1;

// =============================================================================
// Reparameterise direction (for BSURF)
// =============================================================================

pub type PK_BSURF_reparam_dir_t = c_int;
pub const PK_BSURF_reparam_u_c: PK_BSURF_reparam_dir_t = 0;
pub const PK_BSURF_reparam_v_c: PK_BSURF_reparam_dir_t = 1;
pub const PK_BSURF_reparam_both_c: PK_BSURF_reparam_dir_t = 2;

// =============================================================================
// Raise/lower degree direction (for BSURF)
// =============================================================================

pub type PK_BSURF_degree_dir_t = c_int;
pub const PK_BSURF_degree_u_c: PK_BSURF_degree_dir_t = 0;
pub const PK_BSURF_degree_v_c: PK_BSURF_degree_dir_t = 1;

// =============================================================================
// Options structs
// =============================================================================

/// Options for `PK_BCURVE_create_spline_2`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BCURVE_create_spline_2_o_t {
    pub o_t_version: c_int,
    pub spline_method: PK_BCURVE_spline_method_t,
    pub n_special: c_int,
    pub special_indices: *const c_int,
    pub have_fit_tol: PK_LOGICAL_t,
    pub fit_tol: c_double,
    pub is_periodic: PK_LOGICAL_t,
    pub degree: c_int,
    pub n_knots: c_int,
    pub knots: *const c_double,
    pub knot_muls: *const c_int,
    pub overdefined: PK_LOGICAL_t,
    pub clamp: PK_BCURVE_clamp_t,
    pub have_clamp_axes: PK_LOGICAL_t,
    pub clamp_axes: *const c_double,
    pub have_chordal_tol: PK_LOGICAL_t,
    pub chordal_tol: c_double,
    pub n_breaks: c_int,
    pub break_indices: *const c_int,
    pub update: PK_spline_update_t,
}

/// Options for `PK_CURVE_make_bcurve_2`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_CURVE_make_bcurve_2_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub continuity: PK_continuity_t,
    pub force_continuity: PK_force_continuity_t,
    pub force_non_rational: PK_LOGICAL_t,
    pub have_degree: PK_LOGICAL_t,
    pub degree: c_int,
    pub force_bezier: PK_LOGICAL_t,
}

/// Options for `PK_CURVE_make_bcurve_array`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_CURVE_make_bcurve_array_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub continuity: PK_continuity_t,
    pub force_continuity: PK_force_continuity_t,
    pub force_non_rational: PK_LOGICAL_t,
    pub have_degree: PK_LOGICAL_t,
    pub degree: c_int,
    pub force_bezier: PK_LOGICAL_t,
    pub destination: PK_ENTITY_t,
}

/// Options for `PK_SURF_make_bsurf_2`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_SURF_make_bsurf_2_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub continuity: PK_continuity_t,
    pub force_continuity: PK_force_continuity_t,
    pub force_non_rational: PK_LOGICAL_t,
    pub have_degree: PK_LOGICAL_t,
    pub degree: c_int,
    pub force_bezier: PK_LOGICAL_t,
    pub force_cubic: PK_LOGICAL_t,
}

/// Options for `PK_SURF_make_bsurf_array`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_SURF_make_bsurf_array_o_t {
    pub o_t_version: c_int,
    pub tolerance: c_double,
    pub continuity: PK_continuity_t,
    pub force_continuity: PK_force_continuity_t,
    pub force_non_rational: PK_LOGICAL_t,
    pub have_degree: PK_LOGICAL_t,
    pub degree: c_int,
    pub force_bezier: PK_LOGICAL_t,
    pub force_cubic: PK_LOGICAL_t,
    pub have_u_degree: PK_LOGICAL_t,
    pub u_degree: c_int,
    pub have_v_degree: PK_LOGICAL_t,
    pub v_degree: c_int,
    pub destination: PK_ENTITY_t,
}

/// Options for `PK_BSURF_create_constrained`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BSURF_create_constrained_o_t {
    pub o_t_version: c_int,
    pub n_positions: c_int,
    pub positions: *const c_double,
    pub uvs: *const c_double,
    pub uv_surface: PK_SURF_t,
    pub n_normals: c_int,
    pub normals: *const c_double,
    pub normal_indices: *const c_int,
    pub tolerance: c_double,
    pub angular_tolerance: c_double,
    pub optimise: PK_constrained_opt_t,
    pub update: PK_constrained_update_t,
}

/// Per-end extension control for `PK_BCURVE_extend`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_extend_control_t {
    pub extension_shape: PK_BCURVE_extension_shape_t,
    pub extension_type: PK_BCURVE_extension_type_t,
    pub value: c_double,
}

/// Options for `PK_BCURVE_extend`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BCURVE_extend_o_t {
    pub o_t_version: c_int,
    pub low_control: PK_extend_control_t,
    pub high_control: PK_extend_control_t,
    pub extend_closed: PK_extend_closed_t,
}

/// Options for `PK_BCURVE_create_fitted`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BCURVE_create_fitted_o_t {
    pub o_t_version: c_int,
    pub preserve_parameterisation: PK_LOGICAL_t,
}

/// Options for `PK_CURVE_make_approx`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_CURVE_make_approx_o_t {
    pub o_t_version: c_int,
    pub approx_type: PK_CURVE_approx_t,
    pub tolerance: c_double,
}

/// Options for `PK_SURF_make_curve_isoparam`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_SURF_make_curve_isoparam_o_t {
    pub o_t_version: c_int,
    pub preferred_curve_type: PK_preferred_curve_type_t,
}

/// Options for `PK_CURVE_is_isoparam`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_CURVE_is_isoparam_o_t {
    pub o_t_version: c_int,
    pub want_interval: PK_LOGICAL_t,
    pub want_alignment: PK_LOGICAL_t,
}

/// Result for `PK_CURVE_is_isoparam`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_CURVE_is_isoparam_r_t {
    pub is_isoparam: PK_LOGICAL_t,
    pub param_value: c_double,
    pub interval: PK_INTERVAL_t,
    pub alignment: c_int,
}

/// Options for `PK_EDGE_make_curve`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_EDGE_make_curve_o_t {
    pub o_t_version: c_int,
    pub curve_dir: PK_curve_dir_t,
}

/// Options for `PK_BSURF_reparameterise`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BSURF_reparameterise_o_t {
    pub o_t_version: c_int,
    pub transpose: PK_LOGICAL_t,
    pub reverse_u: PK_LOGICAL_t,
    pub reverse_v: PK_LOGICAL_t,
}

// =============================================================================
// Knot query result structs
// =============================================================================

/// Result of `PK_BCURVE_ask_knots`.
#[repr(C)]
#[derive(Debug)]
pub struct PK_BCURVE_ask_knots_r_t {
    pub n_knots: c_int,
    pub knots: *mut c_double,
    pub multiplicities: *mut c_int,
}

/// Result of `PK_BSURF_ask_knots`.
#[repr(C)]
#[derive(Debug)]
pub struct PK_BSURF_ask_knots_r_t {
    pub n_u_knots: c_int,
    pub u_knots: *mut c_double,
    pub u_multiplicities: *mut c_int,
    pub n_v_knots: c_int,
    pub v_knots: *mut c_double,
    pub v_multiplicities: *mut c_int,
}

// =============================================================================
// extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // B-curve creation
    // =========================================================================

    /// Create B-curve from control points and knot vector.
    pub fn PK_BCURVE_create(
        sf: *const PK_BCURVE_sf_t,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create B-curve from piecewise data (Bezier, Hermite, polynomial, Taylor).
    pub fn PK_BCURVE_create_piecewise(
        knot_type: PK_knot_type_t,
        degree: c_int,
        n_pieces: c_int,
        n_vertices: c_int,
        vertex_dim: c_int,
        vertices: *const c_double,
        params: *const c_double,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create B-curve by interpolating or fitting positions.
    pub fn PK_BCURVE_create_spline_2(
        n_positions: c_int,
        positions: *const PK_VECTOR_t,
        options: *const PK_BCURVE_create_spline_2_o_t,
        n_bcurves: *mut c_int,
        bcurves: *mut *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create B-surface by interpolating mesh of points.
    pub fn PK_BSURF_create_splinewise(
        n_u: c_int,
        n_v: c_int,
        positions: *const PK_VECTOR_t,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Fit B-curve to existing curve (guaranteed C2).
    pub fn PK_BCURVE_create_fitted(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        tolerance: c_double,
        options: *const PK_BCURVE_create_fitted_o_t,
        bcurve: *mut PK_BCURVE_t,
        actual_tolerance: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Fit B-surface to existing surface (guaranteed C2).
    pub fn PK_BSURF_create_fitted(
        surface: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        tolerance: c_double,
        bsurf: *mut PK_BSURF_t,
        actual_tolerance: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Fit multiple B-curves to chains of curves (C1 continuous).
    pub fn PK_BCURVE_create_by_fitting(
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        tolerance: c_double,
        bcurve: *mut PK_BCURVE_t,
        actual_tolerance: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // B-surface creation
    // =========================================================================

    /// Create B-surface from control points and knot vectors.
    pub fn PK_BSURF_create(
        sf: *const PK_BSURF_sf_t,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Create B-surface from piecewise data.
    pub fn PK_BSURF_create_piecewise(
        knot_type: PK_knot_type_t,
        u_degree: c_int,
        v_degree: c_int,
        n_u_pieces: c_int,
        n_v_pieces: c_int,
        n_u_vertices: c_int,
        n_v_vertices: c_int,
        vertex_dim: c_int,
        vertices: *const c_double,
        u_params: *const c_double,
        v_params: *const c_double,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Create B-surface from cloud of constraining points.
    pub fn PK_BSURF_create_constrained(
        options: *const PK_BSURF_create_constrained_o_t,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Combining / joining
    // =========================================================================

    /// Join sequence of head-to-tail curves into one B-curve (parameterised on [0,1]).
    pub fn PK_BCURVE_join(
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Combine B-curves.
    pub fn PK_BCURVE_combine(
        n_bcurves: c_int,
        bcurves: *const PK_BCURVE_t,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Sweep / spin
    // =========================================================================

    /// Sweep B-curve into B-surface.
    pub fn PK_BCURVE_sweep(
        bcurve: PK_BCURVE_t,
        direction: *const c_double,
        distance: c_double,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Spin B-curve into B-surface (angle in [-2pi, 2pi]).
    pub fn PK_BCURVE_spin(
        bcurve: PK_BCURVE_t,
        axis_position: *const c_double,
        axis_direction: *const c_double,
        angle: c_double,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Lofting
    // =========================================================================

    /// Create B-surface by interpolating set of B-curves (C2 in loft direction).
    pub fn PK_BCURVE_make_bsurf_lofted(
        n_bcurves: c_int,
        bcurves: *const PK_BCURVE_t,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Amalgamate knot vectors of curves before lofting.
    pub fn PK_BCURVE_make_matched(
        n_bcurves: c_int,
        bcurves: *const PK_BCURVE_t,
        matched_bcurves: *mut *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Conversion to B-geometry
    // =========================================================================

    /// Exact B-curve representation of curve (legacy, prefer `_2` variant).
    pub fn PK_CURVE_make_bcurve(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Exact B-curve representation of curve interval with options.
    pub fn PK_CURVE_make_bcurve_2(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        options: *const PK_CURVE_make_bcurve_2_o_t,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Exact B-curve representations for array of curves.
    pub fn PK_CURVE_make_bcurve_array(
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        options: *const PK_CURVE_make_bcurve_array_o_t,
        bcurves: *mut *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Exact B-surface representation of surface (legacy, prefer `_2` variant).
    pub fn PK_SURF_make_bsurf(
        surface: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Exact B-surface representation of surface interval with options.
    pub fn PK_SURF_make_bsurf_2(
        surface: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        options: *const PK_SURF_make_bsurf_2_o_t,
        bsurf: *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Exact B-surface representations for array of surfaces.
    pub fn PK_SURF_make_bsurf_array(
        n_surfaces: c_int,
        surfaces: *const PK_SURF_t,
        uvboxes: *const PK_UVBOX_t,
        options: *const PK_SURF_make_bsurf_array_o_t,
        bsurfs: *mut *mut PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ask (query) functions
    // =========================================================================

    /// Query B-curve standard form.
    pub fn PK_BCURVE_ask(
        bcurve: PK_BCURVE_t,
        sf: *mut PK_BCURVE_sf_t,
    ) -> PK_ERROR_code_t;

    /// Query B-surface standard form.
    pub fn PK_BSURF_ask(
        bsurf: PK_BSURF_t,
        sf: *mut PK_BSURF_sf_t,
    ) -> PK_ERROR_code_t;

    /// Query knots and multiplicities of a B-curve.
    pub fn PK_BCURVE_ask_knots(
        bcurve: PK_BCURVE_t,
        n_knots: *mut c_int,
        knots: *mut *mut c_double,
        multiplicities: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Query knots and multiplicities of a B-surface.
    pub fn PK_BSURF_ask_knots(
        bsurf: PK_BSURF_t,
        n_u_knots: *mut c_int,
        u_knots: *mut *mut c_double,
        u_multiplicities: *mut *mut c_int,
        n_v_knots: *mut c_int,
        v_knots: *mut *mut c_double,
        v_multiplicities: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Query piecewise representation of a B-curve.
    pub fn PK_BCURVE_ask_piecewise(
        bcurve: PK_BCURVE_t,
        knot_type: PK_knot_type_t,
        n_pieces: *mut c_int,
        n_vertices: *mut c_int,
        vertex_dim: *mut c_int,
        vertices: *mut *mut c_double,
        params: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Query piecewise representation of a B-surface.
    pub fn PK_BSURF_ask_piecewise(
        bsurf: PK_BSURF_t,
        knot_type: PK_knot_type_t,
        n_u_pieces: *mut c_int,
        n_v_pieces: *mut c_int,
        n_u_vertices: *mut c_int,
        n_v_vertices: *mut c_int,
        vertex_dim: *mut c_int,
        vertices: *mut *mut c_double,
        u_params: *mut *mut c_double,
        v_params: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Query splinewise representation of a B-curve.
    pub fn PK_BCURVE_ask_splinewise(
        bcurve: PK_BCURVE_t,
        n_positions: *mut c_int,
        positions: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Query splinewise representation of a B-surface.
    pub fn PK_BSURF_ask_splinewise(
        bsurf: PK_BSURF_t,
        n_u: *mut c_int,
        n_v: *mut c_int,
        positions: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Knot manipulation
    // =========================================================================

    /// Add knot to B-curve.
    pub fn PK_BCURVE_add_knot(
        bcurve: PK_BCURVE_t,
        knot_value: c_double,
    ) -> PK_ERROR_code_t;

    /// Add U-direction knot to B-surface.
    pub fn PK_BSURF_add_u_knot(
        bsurf: PK_BSURF_t,
        knot_value: c_double,
    ) -> PK_ERROR_code_t;

    /// Add V-direction knot to B-surface.
    pub fn PK_BSURF_add_v_knot(
        bsurf: PK_BSURF_t,
        knot_value: c_double,
    ) -> PK_ERROR_code_t;

    /// Remove knots from B-curve within tolerance.
    pub fn PK_BCURVE_remove_knots(
        bcurve: PK_BCURVE_t,
        tolerance: c_double,
        n_removed: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Remove knots from B-surface within tolerance.
    pub fn PK_BSURF_remove_knots(
        bsurf: PK_BSURF_t,
        tolerance: c_double,
        n_removed: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Ensure Bezier end conditions on B-curve.
    pub fn PK_BCURVE_clamp_knots(
        bcurve: PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Ensure Bezier end conditions on B-surface.
    pub fn PK_BSURF_clamp_knots(
        bsurf: PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Degree manipulation
    // =========================================================================

    /// Raise degree of B-curve.
    pub fn PK_BCURVE_raise_degree(
        bcurve: PK_BCURVE_t,
        new_degree: c_int,
    ) -> PK_ERROR_code_t;

    /// Raise degree of B-surface.
    pub fn PK_BSURF_raise_degree(
        bsurf: PK_BSURF_t,
        direction: PK_BSURF_degree_dir_t,
        new_degree: c_int,
    ) -> PK_ERROR_code_t;

    /// Lower degree of B-curve.
    pub fn PK_BCURVE_lower_degree(
        bcurve: PK_BCURVE_t,
        new_degree: c_int,
        tolerance: c_double,
    ) -> PK_ERROR_code_t;

    /// Lower degree of B-surface.
    pub fn PK_BSURF_lower_degree(
        bsurf: PK_BSURF_t,
        direction: PK_BSURF_degree_dir_t,
        new_degree: c_int,
        tolerance: c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Reparameterisation
    // =========================================================================

    /// Rescale/translate knot vector of B-curve.
    pub fn PK_BCURVE_reparameterise(
        bcurve: PK_BCURVE_t,
        new_interval: PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    /// Rescale/translate knot vector of B-surface with options (transpose, reverse).
    pub fn PK_BSURF_reparameterise(
        bsurf: PK_BSURF_t,
        options: *const PK_BSURF_reparameterise_o_t,
    ) -> PK_ERROR_code_t;

    /// Reparameterise surface attached to faces.
    pub fn PK_FACE_reparameterise_surf(
        n_faces: c_int,
        faces: *const PK_FACE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Extension
    // =========================================================================

    /// Extend B-curve in either direction.
    pub fn PK_BCURVE_extend(
        bcurve: PK_BCURVE_t,
        options: *const PK_BCURVE_extend_o_t,
        status: *mut PK_BCURVE_extend_status_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Isoparam / isocline
    // =========================================================================

    /// Extract constant parameter curve (isoparam) from surface.
    pub fn PK_SURF_make_curve_isoparam(
        surface: PK_SURF_t,
        param_value: c_double,
        is_u: PK_LOGICAL_t,
        options: *const PK_SURF_make_curve_isoparam_o_t,
        curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Query if curve is a constant parameter on a surface.
    pub fn PK_CURVE_is_isoparam(
        curve: PK_CURVE_t,
        surface: PK_SURF_t,
        options: *const PK_CURVE_is_isoparam_o_t,
        result: *mut PK_CURVE_is_isoparam_r_t,
    ) -> PK_ERROR_code_t;

    /// Extract U-isoparam B-curve from B-surface.
    pub fn PK_BSURF_make_bcurve_u_isoparam(
        bsurf: PK_BSURF_t,
        param_value: c_double,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Extract V-isoparam B-curve from B-surface.
    pub fn PK_BSURF_make_bcurve_v_isoparam(
        bsurf: PK_BSURF_t,
        param_value: c_double,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create isocline curves on surface.
    pub fn PK_SURF_make_cus_isocline(
        surface: PK_SURF_t,
        direction: *const c_double,
        angle: c_double,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create isocline surface through curve.
    pub fn PK_CURVE_make_surf_isocline(
        curve: PK_CURVE_t,
        surface: PK_SURF_t,
        direction: *const c_double,
        angle: c_double,
        iso_surface: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Approximation
    // =========================================================================

    /// Create re-parameterised approximation of curve.
    pub fn PK_CURVE_make_approx(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        options: *const PK_CURVE_make_approx_o_t,
        approx_curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // G1 discontinuity finding
    // =========================================================================

    /// Find G1 discontinuities in B-curve.
    pub fn PK_BCURVE_find_g1_discontinuity(
        bcurve: PK_BCURVE_t,
        n_params: *mut c_int,
        params: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Find G1 discontinuities in B-surface.
    pub fn PK_BSURF_find_g1_discontinuity(
        bsurf: PK_BSURF_t,
        n_u_params: *mut c_int,
        u_params: *mut *mut c_double,
        n_v_params: *mut c_int,
        v_params: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Approximation evaluation control
    // =========================================================================

    /// Set approximation evaluation on B-curve.
    pub fn PK_BCURVE_set_approx(
        bcurve: PK_BCURVE_t,
        tolerance: c_double,
    ) -> PK_ERROR_code_t;

    /// Unset approximation evaluation on B-curve.
    pub fn PK_BCURVE_unset_approx(
        bcurve: PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Evaluate B-curve using approximation.
    pub fn PK_BCURVE_eval_approx(
        bcurve: PK_BCURVE_t,
        t: c_double,
        n_derivs: c_int,
        position: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Set approximation evaluation on B-surface.
    pub fn PK_BSURF_set_approx(
        bsurf: PK_BSURF_t,
        tolerance: c_double,
    ) -> PK_ERROR_code_t;

    /// Unset approximation evaluation on B-surface.
    pub fn PK_BSURF_unset_approx(
        bsurf: PK_BSURF_t,
    ) -> PK_ERROR_code_t;

    /// Evaluate B-surface using approximation.
    pub fn PK_BSURF_eval_approx(
        bsurf: PK_BSURF_t,
        uv: *const c_double,
        n_derivs: c_int,
        position: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Edge to curve
    // =========================================================================

    /// Create single smooth curve from chain of edges.
    pub fn PK_EDGE_make_curve(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        options: *const PK_EDGE_make_curve_o_t,
        curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Session control
    // =========================================================================

    // =========================================================================
    // Legacy / variant functions
    // =========================================================================

    /// Create B-curve by splining (legacy v1, prefer `PK_BCURVE_create_spline_2`).
    pub fn PK_BCURVE_create_spline(
        n_positions: c_int,
        positions: *const PK_VECTOR_t,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create B-curve splinewise (legacy).
    pub fn PK_BCURVE_create_splinewise(
        n_positions: c_int,
        positions: *const PK_VECTOR_t,
        bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Extend B-curve (return-form variant).
    pub fn PK_BCURVE_extend_r_f(
        bcurve: PK_BCURVE_t,
        options: *const PK_BCURVE_extend_o_t,
        status: *mut PK_BCURVE_extend_status_t,
        new_bcurve: *mut PK_BCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Spline return-form variant.
    pub fn PK_BCURVE_spline_r_f(
        bcurve: PK_BCURVE_t,
        n_positions: *mut c_int,
        positions: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Reparameterise surface of faces (return-form variant).
    pub fn PK_FACE_reparameterise_surf_r_f(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_results: *mut c_int,
        results: *mut *mut c_int,
    ) -> PK_ERROR_code_t;
}
