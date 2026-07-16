#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! Geometry types and functions: analytic surfaces, analytic curves, evaluation,
//! parameterisation, precision, nominal geometry, and geometry attach/detach.

use crate::*;
use std::os::raw::{c_double, c_int};

// =============================================================================
// Basis set — shared by all analytic geometry standard forms
// =============================================================================

/// Orthonormal coordinate frame: location + axis (Z) + ref_direction (X).
/// Y is implicitly axis x ref_direction.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_AXIS2_sf_t {
    pub location: PK_VECTOR_t,
    pub axis: PK_VECTOR_t,
    pub ref_direction: PK_VECTOR_t,
}

/// Axis1 frame: location + axis (used by lines).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_AXIS1_sf_t {
    pub location: PK_VECTOR_t,
    pub axis: PK_VECTOR_t,
}

// =============================================================================
// Surface standard forms
//
// Field order verified against Parasolid V35 header docs: basis_set always
// comes FIRST in analytic sf structs (see solidworks-notes/headers mirror).
// =============================================================================

/// Plane standard form — defined by a basis set (location + normal + ref_direction).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PLANE_sf_t {
    pub basis_set: PK_AXIS2_sf_t,
}

/// Cylinder standard form — radius + basis set. Axis is the cylinder axis.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CYL_sf_t {
    pub basis_set: PK_AXIS2_sf_t,
    pub radius: c_double,
}

/// Cone standard form — radius (at apex end), semi-angle, basis set.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CONE_sf_t {
    pub basis_set: PK_AXIS2_sf_t,
    pub radius: c_double,
    pub semi_angle: c_double,
}

/// Sphere standard form — radius + basis set.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SPHERE_sf_t {
    pub basis_set: PK_AXIS2_sf_t,
    pub radius: c_double,
}

/// Torus standard form — major radius, minor radius, basis set.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TORUS_sf_t {
    pub basis_set: PK_AXIS2_sf_t,
    pub major_radius: c_double,
    pub minor_radius: c_double,
}

/// Spun surface standard form — profile curve revolved about an axis.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SPUN_sf_t {
    pub profile: PK_CURVE_t,
    pub axis: PK_AXIS1_sf_t,
}

/// Swept surface standard form — profile curve swept along a direction.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SWEPT_sf_t {
    pub profile: PK_CURVE_t,
    pub path: PK_VECTOR_t,
}

/// Offset surface standard form — base surface + offset distance.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_OFFSET_sf_t {
    pub basis_surf: PK_SURF_t,
    pub distance: c_double,
}

// =============================================================================
// Curve standard forms
// =============================================================================

/// Line standard form — location + direction (axis1 basis set).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_LINE_sf_t {
    pub basis_set: PK_AXIS1_sf_t,
}

/// Circle standard form — radius + basis set. Circle lies in the XY plane of the basis set.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CIRCLE_sf_t {
    pub basis_set: PK_AXIS2_sf_t,
    pub radius: c_double,
}

/// Ellipse standard form — R1 (major), R2 (minor) + basis set.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_ELLIPSE_sf_t {
    pub basis_set: PK_AXIS2_sf_t,
    pub R1: c_double,
    pub R2: c_double,
}

/// Point standard form — just a position.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_POINT_sf_t {
    pub position: PK_VECTOR_t,
}

/// SP-curve standard form — curve on surface, defined by a surface + 2D B-curve in UV space.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SPCURVE_sf_t {
    pub surface: PK_SURF_t,
    pub bcurve: PK_BCURVE_t,
}

/// Trimmed curve standard form — base curve restricted to an interval.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TRCURVE_sf_t {
    pub curve: PK_CURVE_t,
    pub interval: PK_INTERVAL_t,
}

// =============================================================================
// Parameterisation types
// =============================================================================

/// Parameterisation type for a single direction.
pub type PK_PARAM_direction_t = c_int;

/// Not periodic, not closed.
pub const PK_PARAM_direction_non_periodic_c: PK_PARAM_direction_t = 0;
/// Periodic (wraps around).
pub const PK_PARAM_direction_periodic_c: PK_PARAM_direction_t = 1;

/// Curve parameterisation result structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_param_t {
    pub param_type: PK_PARAM_direction_t,
    pub period: c_double,
}

/// Surface parameterisation result structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_param_t {
    pub u_type: PK_PARAM_direction_t,
    pub u_period: c_double,
    pub v_type: PK_PARAM_direction_t,
    pub v_period: c_double,
}

/// Standard form of a single parametric direction (`PK_CURVE_ask_param` writes
/// one; `PK_SURF_ask_params` writes two, u then v). 40-byte layout recovered by
/// decompiling `PK_CURVE_ask_param`: `range`@0, four i32 enum fields @16/20/24/28
/// (`periodic`@24 ∈ 18020/18021/18022), `closed` logical byte @32.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARAM_sf_t {
    pub range: PK_INTERVAL_t, // @0
    pub extent: c_int,        // @16
    pub form: c_int,          // @20
    pub periodic: c_int,      // @24  (PK_PARAM_periodic_*_c)
    pub convexity: c_int,     // @28
    pub closed: PK_LOGICAL_t, // @32 (value in low byte)
}

const _: () = {
    assert!(core::mem::size_of::<PK_PARAM_sf_t>() == 40);
};

// =============================================================================
// Precision enums
// =============================================================================

/// SP-curve generation method for PK_EDGE_set_precision_2.
pub type PK_set_precision_sp_method_t = c_int;
/// Default G1 continuous SP-curves.
pub const PK_set_precision_default_c: PK_set_precision_sp_method_t = 23550;
/// C2 continuous SP-curves.
pub const PK_set_precision_c2_c: PK_set_precision_sp_method_t = 23551;

/// Short edge reporting mode for PK_EDGE_set_precision_2.
pub type PK_set_precision_report_t = c_int;
/// Return PK_ERROR_bad_tolerance_c (default).
pub const PK_set_precision_report_no_c: PK_set_precision_report_t = 23730;
/// Return PK_ERROR_edge_too_short + edge group.
pub const PK_set_precision_report_yes_c: PK_set_precision_report_t = 23731;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_REPORT_4_error_report_c: PK_set_precision_report_t = 25922;

/// Method for PK_EDGE_reset_precision_2.
pub type PK_reset_prec_method_t = c_int;
/// Any coincident curve acceptable.
pub const PK_reset_prec_method_any_c: PK_reset_prec_method_t = 25090;
/// True intersection curve preferable.
pub const PK_reset_prec_method_inter_c: PK_reset_prec_method_t = 25091;
/// Only true intersection curve acceptable.
pub const PK_reset_prec_method_int_only_c: PK_reset_prec_method_t = 25092;

/// Edge optimise option — whether to include short edges.
pub type PK_EDGE_optimise_short_t = c_int;
/// Do not include short edges.
pub const PK_EDGE_optimise_no_c: PK_EDGE_optimise_short_t = 0;
/// Include short edges.
pub const PK_EDGE_optimise_yes_c: PK_EDGE_optimise_short_t = 1;

// =============================================================================
// Options structures for precision functions
// =============================================================================

/// Options for PK_EDGE_set_precision_2.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_set_precision_2_o_t {
    /// Version of the options structure.
    pub o_t_version: c_int,
    /// SP-curve generation method.
    pub sp_method: PK_set_precision_sp_method_t,
    /// How to report short edge failures.
    pub report_short_edges: PK_set_precision_report_t,
}

/// Options for PK_EDGE_reset_precision_2.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_reset_precision_2_o_t {
    /// Version of the options structure.
    pub o_t_version: c_int,
    /// Method for determining edge geometry.
    pub method: PK_reset_prec_method_t,
}

/// Options for PK_EDGE_optimise.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_optimise_o_t {
    /// Version of the options structure.
    pub o_t_version: c_int,
    /// Whether to include short edges.
    pub optimise_short: PK_EDGE_optimise_short_t,
}

// =============================================================================
// Gap closing enums and options
// =============================================================================

/// Gap closing in 3-space mode.
pub type PK_LOOP_3_space_gap_t = c_int;
/// Close gaps in model space (default).
pub const PK_LOOP_3_space_gap_close_c: PK_LOOP_3_space_gap_t = 23681;

/// Gap closing in 2-space (parameter space) mode.
pub type PK_LOOP_2_space_gap_t = c_int;
/// Don't close parameter space gaps (default).
pub const PK_LOOP_2_space_gap_no_c: PK_LOOP_2_space_gap_t = 23690;
/// Minimize gaps by transforming B-curves to meet in same period.
pub const PK_LOOP_2_space_gap_minimise_c: PK_LOOP_2_space_gap_t = 23691;
/// Close gaps by extending/trimming SP-curves.
pub const PK_LOOP_2_space_gap_close_cu_c: PK_LOOP_2_space_gap_t = 23692;
/// Close gaps by reparameterizing surface if needed.
pub const PK_LOOP_2_space_gap_close_all_c: PK_LOOP_2_space_gap_t = 23693;

/// Fin trimming mode.
pub type PK_LOOP_trim_geom_t = c_int;
/// Don't trim fin geometry (default).
pub const PK_LOOP_trim_geom_no_c: PK_LOOP_trim_geom_t = 24200;

/// Options for PK_LOOP_close_gaps and PK_FACE_close_gaps.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_LOOP_close_gaps_o_t {
    /// Version of the options structure.
    pub o_t_version: c_int,
    /// Gap closing in model space.
    pub gap_in_3_space: PK_LOOP_3_space_gap_t,
    /// Gap closing in parameter space.
    pub gap_in_2_space: PK_LOOP_2_space_gap_t,
    /// Tolerance for parameter space gap.
    pub tol_in_2_space: c_double,
    /// Trim SP-curve geometry outside fin bounds.
    pub trim_fin_geom: PK_LOOP_trim_geom_t,
}

/// Nominal curve state for PK_BODY_set_curve_nmnl_state / PK_BODY_ask_curve_nmnl_state.
pub type PK_BODY_curve_nmnl_state_t = c_int;
/// Nominal curves disabled.
pub const PK_BODY_curve_nmnl_disabled_c: PK_BODY_curve_nmnl_state_t = 0;
/// Nominal curves enabled.
pub const PK_BODY_curve_nmnl_enabled_c: PK_BODY_curve_nmnl_state_t = 1;

// =============================================================================
// Error constants specific to geometry
// =============================================================================

/// Plane axes not orthogonal.
pub const PK_ERROR_vectors_not_orthogonal: PK_ERROR_code_t = 400;
/// Point outside size box.
pub const PK_ERROR_bad_position: PK_ERROR_code_t = 401;
/// Wrong curve type for swept/spun surface.
pub const PK_ERROR_unsuitable_entity: PK_ERROR_code_t = 402;
/// Spun surface would self-intersect.
pub const PK_ERROR_su_self_intersect: PK_ERROR_code_t = 403;
/// Tolerance error during precision setting.
pub const PK_ERROR_bad_tolerance_c: PK_ERROR_code_t = 404;
/// Edge too short during precision setting.
pub const PK_ERROR_edge_too_short: PK_ERROR_code_t = 405;

// =============================================================================
// Opaque options/result structs for geometry operations
// =============================================================================

/// Options for `PK_GEOM_copy`.
#[repr(C)]
pub struct PK_GEOM_copy_o_t { _private: [u8; 0] }

/// Results from `PK_GEOM_copy`.
#[repr(C)]
pub struct PK_GEOM_copy_r_t { _private: [u8; 0] }

/// Options for `PK_GEOM_enlarge`.
#[repr(C)]
pub struct PK_GEOM_enlarge_o_t { _private: [u8; 0] }

/// Results from `PK_GEOM_enlarge`.
#[repr(C)]
pub struct PK_GEOM_enlarge_r_t { _private: [u8; 0] }

/// Options for `PK_VECTOR_make_lsq_plane`.
#[repr(C)]
pub struct PK_VECTOR_make_lsq_plane_o_t { _private: [u8; 0] }

/// Opaque result type for curve degeneracies.
#[repr(C)]
pub struct PK_CURVE_degens_t { _private: [u8; 0] }

/// Opaque result type for curve self-intersections.
#[repr(C)]
pub struct PK_CURVE_self_ints_t { _private: [u8; 0] }

/// Opaque result type for surface degeneracies.
#[repr(C)]
pub struct PK_SURF_degens_t { _private: [u8; 0] }

/// Opaque result type for surface self-intersections.
#[repr(C)]
pub struct PK_SURF_self_ints_t { _private: [u8; 0] }

// =============================================================================
// extern "C" — Surface create/ask functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // -------------------------------------------------------------------------
    // Plane
    // -------------------------------------------------------------------------

    /// Create a plane from its standard form.
    pub fn PK_PLANE_create(sf: *const PK_PLANE_sf_t, plane: *mut PK_PLANE_t) -> PK_ERROR_code_t;

    /// Query the standard form of a plane.
    pub fn PK_PLANE_ask(plane: PK_PLANE_t, sf: *mut PK_PLANE_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Cylinder
    // -------------------------------------------------------------------------

    /// Create a cylinder from its standard form.
    pub fn PK_CYL_create(sf: *const PK_CYL_sf_t, cyl: *mut PK_CYLL_t) -> PK_ERROR_code_t;

    /// Query the standard form of a cylinder.
    pub fn PK_CYL_ask(cyl: PK_CYLL_t, sf: *mut PK_CYL_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Cone
    // -------------------------------------------------------------------------

    /// Create a cone from its standard form.
    pub fn PK_CONE_create(sf: *const PK_CONE_sf_t, cone: *mut PK_CONE_t) -> PK_ERROR_code_t;

    /// Query the standard form of a cone.
    pub fn PK_CONE_ask(cone: PK_CONE_t, sf: *mut PK_CONE_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Sphere
    // -------------------------------------------------------------------------

    /// Create a sphere from its standard form.
    pub fn PK_SPHERE_create(
        sf: *const PK_SPHERE_sf_t,
        sphere: *mut PK_SPHERE_t,
    ) -> PK_ERROR_code_t;

    /// Query the standard form of a sphere.
    pub fn PK_SPHERE_ask(sphere: PK_SPHERE_t, sf: *mut PK_SPHERE_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Torus
    // -------------------------------------------------------------------------

    /// Create a torus from its standard form.
    pub fn PK_TORUS_create(sf: *const PK_TORUS_sf_t, torus: *mut PK_TORUS_t) -> PK_ERROR_code_t;

    /// Query the standard form of a torus.
    pub fn PK_TORUS_ask(torus: PK_TORUS_t, sf: *mut PK_TORUS_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Spun surface (revolution)
    // -------------------------------------------------------------------------

    /// Create a spun surface from its standard form.
    pub fn PK_SPUN_create(sf: *const PK_SPUN_sf_t, spun: *mut PK_SPUN_t) -> PK_ERROR_code_t;

    /// Query the standard form of a spun surface.
    pub fn PK_SPUN_ask(spun: PK_SPUN_t, sf: *mut PK_SPUN_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Swept surface (extrusion)
    // -------------------------------------------------------------------------

    /// Create a swept surface from its standard form.
    pub fn PK_SWEPT_create(sf: *const PK_SWEPT_sf_t, swept: *mut PK_SWEPT_t) -> PK_ERROR_code_t;

    /// Query the standard form of a swept surface.
    pub fn PK_SWEPT_ask(swept: PK_SWEPT_t, sf: *mut PK_SWEPT_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Offset surface
    // -------------------------------------------------------------------------

    /// Create an offset surface from its standard form.
    pub fn PK_OFFSET_create(
        sf: *const PK_OFFSET_sf_t,
        offset: *mut PK_OFFSET_t,
    ) -> PK_ERROR_code_t;

    /// Query the standard form of an offset surface.
    pub fn PK_OFFSET_ask(offset: PK_OFFSET_t, sf: *mut PK_OFFSET_sf_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Curve create/ask functions
    // =========================================================================

    // -------------------------------------------------------------------------
    // Line
    // -------------------------------------------------------------------------

    /// Create a line from its standard form.
    pub fn PK_LINE_create(sf: *const PK_LINE_sf_t, line: *mut PK_LINE_t) -> PK_ERROR_code_t;

    /// Query the standard form of a line.
    pub fn PK_LINE_ask(line: PK_LINE_t, sf: *mut PK_LINE_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Circle
    // -------------------------------------------------------------------------

    /// Create a circle from its standard form.
    pub fn PK_CIRCLE_create(
        sf: *const PK_CIRCLE_sf_t,
        circle: *mut PK_CIRCLE_t,
    ) -> PK_ERROR_code_t;

    /// Query the standard form of a circle.
    pub fn PK_CIRCLE_ask(circle: PK_CIRCLE_t, sf: *mut PK_CIRCLE_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Ellipse
    // -------------------------------------------------------------------------

    /// Create an ellipse from its standard form.
    pub fn PK_ELLIPSE_create(
        sf: *const PK_ELLIPSE_sf_t,
        ellipse: *mut PK_ELLIPSE_t,
    ) -> PK_ERROR_code_t;

    /// Query the standard form of an ellipse.
    pub fn PK_ELLIPSE_ask(ellipse: PK_ELLIPSE_t, sf: *mut PK_ELLIPSE_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Point
    // -------------------------------------------------------------------------

    /// Create a point from its standard form.
    pub fn PK_POINT_create(sf: *const PK_POINT_sf_t, point: *mut PK_POINT_t) -> PK_ERROR_code_t;

    /// Query the standard form of a point.
    pub fn PK_POINT_ask(point: PK_POINT_t, sf: *mut PK_POINT_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // SP-curve (surface parametric curve)
    // -------------------------------------------------------------------------

    /// Create an SP-curve from its standard form.
    pub fn PK_SPCURVE_create(
        sf: *const PK_SPCURVE_sf_t,
        spcurve: *mut PK_SCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Query the standard form of an SP-curve.
    pub fn PK_SPCURVE_ask(spcurve: PK_SCURVE_t, sf: *mut PK_SPCURVE_sf_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Trimmed curve (legacy, read-only)
    // -------------------------------------------------------------------------

    /// Query the standard form of a trimmed curve.
    pub fn PK_TRCURVE_ask(trcurve: PK_TCURVE_t, sf: *mut PK_TRCURVE_sf_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Curve parameterisation and evaluation
    // =========================================================================

    /// Return the parametric interval [t_min, t_max] of a curve.
    pub fn PK_CURVE_ask_interval(
        curve: PK_CURVE_t,
        interval: *mut PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the parameterisation type (periodic/non-periodic) of a curve.
    // V35: writes a single 40-byte PK_PARAM_sf_t (the old PK_CURVE_param_t was bogus).
    pub fn PK_CURVE_ask_param(
        curve: PK_CURVE_t,
        param: *mut PK_PARAM_sf_t,
    ) -> PK_ERROR_code_t;

    /// Evaluate curve position R(t) at parameter value `t`.
    /// `n_deriv` specifies the number of derivatives to compute (0 = position only).
    /// `position` must point to a buffer of at least 3*(n_deriv+1) doubles.
    pub fn PK_CURVE_eval(
        curve: PK_CURVE_t,
        t: c_double,
        n_deriv: c_int,
        position: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Evaluate curve at `t`, returning position and unit tangent vector.
    pub fn PK_CURVE_eval_with_tangent(
        curve: PK_CURVE_t,
        t: c_double,
        n_derivs: c_int,
        p: *mut PK_VECTOR_t,
        tangent: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Evaluate curve curvature at parameter `t`.
    // V35 vendor form: (curve, t, *tangent, *principal_normal, *binormal,
    // *curvature). No position out-arg (the old binding had a bogus `position`
    // and was missing `binormal`).
    pub fn PK_CURVE_eval_curvature(
        curve: PK_CURVE_t,
        t: c_double,
        tangent: *mut PK_VECTOR1_t,
        principal_normal: *mut PK_VECTOR1_t,
        binormal: *mut PK_VECTOR1_t,
        curvature: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Inverse parameterisation: find parameter t closest to `position` on the curve.
    pub fn PK_CURVE_parameterise_vector(
        curve: PK_CURVE_t,
        position: *const PK_VECTOR_t,
        t: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Surface parameterisation and evaluation
    // =========================================================================

    /// Return the UV bounding box of a surface.
    pub fn PK_SURF_ask_uvbox(surf: PK_SURF_t, uvbox: *mut PK_UVBOX_t) -> PK_ERROR_code_t;

    /// Return the parameterisation type (periodic/non-periodic) in U and V.
    // V35: writes TWO 40-byte PK_PARAM_sf_t (u=param[0], v=param[1]); caller must
    // pass a `[PK_PARAM_sf_t; 2]` buffer.
    pub fn PK_SURF_ask_params(surf: PK_SURF_t, param: *mut PK_PARAM_sf_t) -> PK_ERROR_code_t;

    /// Evaluate surface position R(u,v) at parameter values `uv`.
    /// `n_u_deriv` and `n_v_deriv` specify derivative orders.
    /// `position` must point to a buffer of sufficient size for all derivatives.
    pub fn PK_SURF_eval(
        surf: PK_SURF_t,
        uv: *const c_double,
        n_u_deriv: c_int,
        n_v_deriv: c_int,
        triangular: PK_LOGICAL_t,
        position: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Evaluate surface at (u,v), returning position and outward unit normal.
    pub fn PK_SURF_eval_with_normal(
        surf: PK_SURF_t,
        uv: *const PK_UV_t,
        n_u_derivs: c_int,
        n_v_derivs: c_int,
        triangular: PK_LOGICAL_t,
        p: *mut PK_VECTOR_t,
        normal: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Evaluate surface curvature at (u,v).
    pub fn PK_SURF_eval_curvature(
        surf: PK_SURF_t,
        uv: *const PK_UV_t,
        normal: *mut PK_VECTOR1_t,
        principal_direction_1: *mut PK_VECTOR1_t,
        principal_direction_2: *mut PK_VECTOR1_t,
        principal_curvature_1: *mut c_double,
        principal_curvature_2: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Inverse parameterisation: find (u,v) closest to `position` on the surface.
    pub fn PK_SURF_parameterise_vector(
        surf: PK_SURF_t,
        position: *const PK_VECTOR_t,
        uv: *mut PK_UV_t,
    ) -> PK_ERROR_code_t;

    /// Find entities that can be reparameterised for better evaluation results.
    pub fn PK_ENTITY_find_reparam(
        n_entities: c_int,
        entities: *const PK_ENTITY_t,
        n_reparam: *mut c_int,
        reparam: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Session precision
    // =========================================================================

    // =========================================================================
    // Edge precision
    // =========================================================================

    /// Return the current precision of an edge.
    pub fn PK_EDGE_ask_precision(
        edge: PK_EDGE_t,
        precision: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Set local precision on an edge (simple form).
    pub fn PK_EDGE_set_precision(
        edge: PK_EDGE_t,
        precision: c_double,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Set local precision on an edge with options (SP-curve method, short edge reporting).
    pub fn PK_EDGE_set_precision_2(
        edge: PK_EDGE_t,
        precision: c_double,
        options: *mut PK_EDGE_set_precision_o_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Reset precision on a tolerant edge to optimal value.
    pub fn PK_EDGE_optimise(
        edge: PK_EDGE_t,
        options: *mut PK_EDGE_optimise_o_t,
        result: *mut PK_EDGE_optimise_result_t,
        achieved_deviation: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Remove local precision from an edge (simple form, restore exact geometry).
    pub fn PK_EDGE_reset_precision(
        edge: PK_EDGE_t,
        result: *mut PK_reset_prec_t,
    ) -> PK_ERROR_code_t;

    /// Remove local precision from an edge with options (method for geometry determination).
    pub fn PK_EDGE_reset_precision_2(
        edge: PK_EDGE_t,
        options: *mut PK_EDGE_reset_precision_o_t,
        result: *mut PK_reset_prec_t,
    ) -> PK_ERROR_code_t;

    /// Set appropriate precision automatically on an edge (may split edges).
    pub fn PK_EDGE_repair(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        options: *mut PK_EDGE_repair_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Vertex precision
    // =========================================================================

    /// Return the current precision of a vertex.
    pub fn PK_VERTEX_ask_precision(
        vertex: PK_VERTEX_t,
        precision: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Set precision on a vertex.
    pub fn PK_VERTEX_set_precision(
        vertex: PK_VERTEX_t,
        precision: c_double,
    ) -> PK_ERROR_code_t;

    /// Optimize vertex precision to the minimum required value.
    pub fn PK_VERTEX_optimise(
        vertex: PK_VERTEX_t,
        options: *mut PK_VERTEX_optimise_o_t,
        result: *mut PK_VERTEX_optimise_result_t,
        achieved_deviation: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Loop/face gap closing
    // =========================================================================

    /// Close gaps at tolerant vertices in a loop.
    pub fn PK_LOOP_close_gaps(
        r#loop: PK_LOOP_t,
        options: *mut PK_LOOP_close_gaps_o_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Close gaps at tolerant vertices in a face.
    pub fn PK_FACE_close_gaps(
        face: PK_FACE_t,
        options: *mut PK_FACE_close_gaps_o_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Nominal geometry
    // =========================================================================

    /// Enable or disable nominal curves for a body.
    pub fn PK_BODY_set_curve_nmnl_state(
        body: PK_BODY_t,
        state: PK_BODY_curve_nmnl_state_t,
    ) -> PK_ERROR_code_t;

    /// Query whether nominal curves are enabled on a body.
    pub fn PK_BODY_ask_curve_nmnl_state(
        body: PK_BODY_t,
        state: *mut PK_BODY_curve_nmnl_state_t,
    ) -> PK_ERROR_code_t;

    /// Attach a nominal curve to a tolerant edge.
    /// The curve must lie within the edge's tolerance band.
    pub fn PK_EDGE_attach_curve_nmnl(
        edge: PK_EDGE_t,
        curve: PK_CURVE_t,
        options: *mut PK_EDGE_attach_curve_nmnl_o_t,
    ) -> PK_ERROR_code_t;

    /// Detach the nominal curve from a tolerant edge.
    pub fn PK_EDGE_detach_curve_nmnl(edge: PK_EDGE_t) -> PK_ERROR_code_t;

    /// Return the accurate or notionally accurate curve of an edge (nominal geometry).
    pub fn PK_EDGE_ask_curve_nmnl(
        edge: PK_EDGE_t,
        curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Return the accurate or notionally accurate geometry of an edge (nominal geometry).
    pub fn PK_EDGE_ask_geometry_nmnl(
        edge: PK_EDGE_t,
        want_interval: PK_LOGICAL_t,
        curve: *mut PK_CURVE_t,
        class: *mut PK_CLASS_t,
        ends: *mut PK_VECTOR_t,
        t_int: *mut PK_INTERVAL_t,
        sense: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return all edges for which this curve is the nominal geometry.
    pub fn PK_CURVE_ask_edges_nmnl(
        curve: PK_CURVE_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Geometry operations
    // =========================================================================

    /// Copy geometric entities with options.
    pub fn PK_GEOM_copy(
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
        options: *const PK_GEOM_copy_o_t,
        copies: *mut PK_GEOM_copy_r_t,
    ) -> PK_ERROR_code_t;

    /// Transform geometric entity.
    pub fn PK_GEOM_transform(
        in_geom: PK_GEOM_t,
        transf: PK_TRANSF_t,
        tolerance: c_double,
        out_geom: *mut PK_GEOM_t,
        exact: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Delete single geometric entity.
    pub fn PK_GEOM_delete_single(geom: PK_GEOM_t) -> PK_ERROR_code_t;

    /// Enlarge geometries by scale factor.
    /// V35: `(n_geoms, geoms, transfs, PK_ENTITY_t *entries, factor, options, results)` —
    /// the old binding dropped the per-geom `entries` array.
    pub fn PK_GEOM_enlarge(
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
        transfs: *const PK_TRANSF_t,
        entries: *const PK_ENTITY_t,
        factor: PK_scale_factor_t,
        options: *const PK_GEOM_enlarge_o_t,
        results: *mut PK_GEOM_enlarge_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Vector utilities
    // =========================================================================

    /// Least-squares plane fit to positions.
    pub fn PK_VECTOR_make_lsq_plane(
        n_positions: c_int,
        positions: *const PK_VECTOR_t,
        options: *const PK_VECTOR_make_lsq_plane_o_t,
        plane: *mut PK_PLANE_t,
    ) -> PK_ERROR_code_t;

    /// Create viewing transform from direction (deprecated).
    pub fn PK_VECTOR_make_view_transf(
        direct: PK_VECTOR1_t,
        transf: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Compute perpendicular vector.
    pub fn PK_VECTOR_perpendicular(
        vector1: PK_VECTOR1_t,
        vector2: PK_VECTOR_t,
        perpendicular_vector: *mut PK_VECTOR1_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Foreign geometry
    // =========================================================================

    /// Create foreign curve from standard form.
    pub fn PK_FCURVE_create(
        fcurve_sf: *const PK_FCURVE_sf_t,
        fcurve: *mut PK_FCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create foreign surface from standard form.
    pub fn PK_FSURF_create(
        fsurf_sf: *const PK_FSURF_sf_t,
        fsurf: *mut PK_FSURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free curve degeneracies result.
    pub fn PK_CURVE_degens_f(
        result: *mut PK_CURVE_degens_t,
    ) -> PK_ERROR_code_t;

    /// Free curve self-intersections result.
    pub fn PK_CURVE_self_ints_f(
        result: *mut PK_CURVE_self_ints_t,
    ) -> PK_ERROR_code_t;

    /// Free surface degeneracies result.
    pub fn PK_SURF_degens_f(
        result: *mut PK_SURF_degens_t,
    ) -> PK_ERROR_code_t;

    /// Free surface self-intersections result.
    pub fn PK_SURF_self_ints_f(
        result: *mut PK_SURF_self_ints_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Geometry attach/detach
    // =========================================================================

    /// Attach curves to edges (batch).
    pub fn PK_EDGE_attach_curves(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        curves: *const PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Attach curves to edges (version 2, with options).
    pub fn PK_EDGE_attach_curves_2(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        curves: *mut PK_CURVE_t,
        options: *mut PK_EDGE_attach_curves_o_t,
        tracking: *mut PK_ENTITY_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Attach surfaces to faces (batch).
    pub fn PK_FACE_attach_surfs(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        surfs: *mut PK_SURF_t,
        senses: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Create and attach a fitted surface to a face.
    pub fn PK_FACE_attach_surf_fitting(
        face: PK_FACE_t,
        local_check: PK_LOGICAL_t,
        local_check_result: *mut PK_local_check_t,
    ) -> PK_ERROR_code_t;

    /// Replace surfaces on faces (version 3).
    pub fn PK_FACE_replace_surfs_3(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        surfs: *mut PK_SURF_t,
        senses: *mut PK_LOGICAL_t,
        tolerance: c_double,
        options: *mut PK_FACE_replace_surfs_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    /// Attach curves to fins (batch).
    pub fn PK_FIN_attach_curves(
        n_fins: c_int,
        fins: *mut PK_FIN_t,
        curves: *mut PK_CURVE_t,
        intervals: *mut PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    /// Attach points to vertices (batch).
    pub fn PK_VERTEX_attach_points(
        n_vertices: c_int,
        vertices: *const PK_VERTEX_t,
        points: *const PK_POINT_t,
    ) -> PK_ERROR_code_t;

    /// Add construction geometry to a part.
    pub fn PK_PART_add_geoms(
        part: PK_PART_t,
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
    ) -> PK_ERROR_code_t;

    /// Remove construction geometry from a part.
    pub fn PK_PART_remove_geoms(
        part: PK_PART_t,
        n_geoms: c_int,
        geoms: *mut PK_GEOM_t,
        n_removed: *mut c_int,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_GEOM_copy`.
    pub fn PK_GEOM_copy_r_f(results: *mut PK_GEOM_copy_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_GEOM_enlarge`.
    pub fn PK_GEOM_enlarge_r_f(results: *mut PK_GEOM_enlarge_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_VECTOR_make_lsq_plane`.
    pub fn PK_VECTOR_make_lsq_plane_r_f(results: *mut PK_PLANE_t) -> PK_ERROR_code_t;

    /// Free results from `PK_SWEPT_ask`.
    pub fn PK_SWEPT_ask_r_f(results: *mut PK_SWEPT_sf_t) -> PK_ERROR_code_t;

}
