//! Distance, range, clash detection, and intersection functions.
//!
//! Bindings for Parasolid distance/range (Ch. 26), clash detection (Ch. 27),
//! and intersection functions (Ch. 54).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_double, c_int};

use crate::*;

// =============================================================================
// Range enums
// =============================================================================

/// Whether to find minimum or maximum distance.
pub type PK_range_type_t = c_int;
/// Find minimum distance (default).
pub const PK_range_type_minimum_c: PK_range_type_t = 0;
/// Find maximum distance.
pub const PK_range_type_maximum_c: PK_range_type_t = 1;

/// Optimization level for range computation.
pub type PK_range_opt_t = c_int;
/// Optimize for performance (default); may return local extremum.
pub const PK_range_opt_performance_c: PK_range_opt_t = 0;
/// Optimize for accuracy; slower but more reliable global result.
pub const PK_range_opt_accuracy_c: PK_range_opt_t = 1;

/// Result status of a range computation.
pub type PK_range_result_t = c_int;
/// Min/max distance successfully found.
pub const PK_range_result_found_c: PK_range_result_t = 0;
/// No distance greater than supplied lower_bound found.
pub const PK_range_result_lower_c: PK_range_result_t = 1;
/// No distance less than supplied upper_bound found.
pub const PK_range_result_upper_c: PK_range_result_t = 2;

/// Type of initial estimate supplied to a range function.
pub type PK_range_guess_t = c_int;
/// No estimate (default).
pub const PK_range_guess_no_c: PK_range_guess_t = 0;
/// Parameter estimate.
pub const PK_range_guess_param_c: PK_range_guess_t = 1;
/// Position estimate.
pub const PK_range_guess_vector_c: PK_range_guess_t = 2;

// =============================================================================
// Range helper structures
// =============================================================================

/// Initial estimate for a range computation.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_guess_s_t {
    /// Type of estimate.
    pub guess_type: PK_range_guess_t,
    /// Parameter values (up to 2: one for curve/edge, two for surface/face).
    pub parameters: [c_double; 2],
    /// Position vector (used when `guess_type == PK_range_guess_vector_c`).
    pub vector: PK_VECTOR_t,
}

/// Parametric bounds for a geometrical entity used in range functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_param_bound_t {
    /// Parameter interval for curves.
    pub interval: PK_INTERVAL_t,
    /// UV-box for surfaces.
    pub uvbox: PK_UVBOX_t,
}

/// Details of one endpoint in a range result.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_end_t {
    /// Entity tag at this endpoint.
    pub entity: PK_ENTITY_t,
    /// Sub-entity tag (edge/vertex on which the closest point lies).
    pub sub_entity: PK_ENTITY_t,
    /// Position of the endpoint.
    pub position: PK_VECTOR_t,
    /// Parameter values at the endpoint (1 for curve, 2 for surface).
    pub parameters: [c_double; 2],
}

/// Result of a range computation between two entities.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_2_r_t {
    /// Result status.
    pub result: PK_range_result_t,
    /// Computed distance.
    pub distance: c_double,
    /// Details for the first entity endpoint.
    pub end_1: PK_range_end_t,
    /// Details for the second entity endpoint.
    pub end_2: PK_range_end_t,
}

/// Result of a range computation between an entity and a position vector.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_1_r_t {
    /// Result status.
    pub result: PK_range_result_t,
    /// Computed distance.
    pub distance: c_double,
    /// Details for the entity endpoint.
    pub end: PK_range_end_t,
}

// =============================================================================
// Range options structures
// =============================================================================

/// Options common to entity-to-entity range functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_range_o_t {
    pub o_t_version: c_int,
    /// Whether a tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Accuracy tolerance.
    pub tolerance: c_double,
    /// Upper/lower distance bound.
    pub bound: c_double,
    /// Optimization level.
    pub opt_level: PK_range_opt_t,
    /// Type of range (min or max).
    pub range_type: PK_range_type_t,
    /// Initial estimate for entity 1.
    pub guess_1: PK_range_guess_s_t,
    /// Initial estimate for entity 2.
    pub guess_2: PK_range_guess_s_t,
}

/// Options common to entity-to-vector range functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_range_vector_o_t {
    pub o_t_version: c_int,
    /// Whether a tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Accuracy tolerance.
    pub tolerance: c_double,
    /// Optimization level.
    pub opt_level: PK_range_opt_t,
    /// Initial estimate.
    pub guess: PK_range_guess_s_t,
    /// Whether to return vector/param on sub_entity.
    pub param_entity: PK_LOGICAL_t,
}

/// Options for `PK_GEOM_range` and related geometry range functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_range_o_t {
    pub o_t_version: c_int,
    /// Whether a tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Accuracy tolerance.
    pub tolerance: c_double,
    /// Upper/lower distance bound.
    pub bound: c_double,
    /// Optimization level.
    pub opt_level: PK_range_opt_t,
    /// Type of range (min or max).
    pub range_type: PK_range_type_t,
    /// Initial estimate for entity 1.
    pub guess_1: PK_range_guess_s_t,
    /// Initial estimate for entity 2.
    pub guess_2: PK_range_guess_s_t,
    /// Parametric bounds for entity 1.
    pub param_bound_1: PK_range_param_bound_t,
    /// Parametric bounds for entity 2.
    pub param_bound_2: PK_range_param_bound_t,
}

/// Options for `PK_GEOM_range_vector` and related geometry-to-vector functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_range_vector_o_t {
    pub o_t_version: c_int,
    /// Whether a tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Accuracy tolerance.
    pub tolerance: c_double,
    /// Optimization level.
    pub opt_level: PK_range_opt_t,
    /// Initial estimate.
    pub guess: PK_range_guess_s_t,
    /// Parametric bounds for the geometry.
    pub param_bound: PK_range_param_bound_t,
}

/// Options for local range functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_range_local_o_t {
    pub o_t_version: c_int,
    /// Whether a tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Accuracy tolerance.
    pub tolerance: c_double,
    /// Initial estimate for entity 1.
    pub guess_1: PK_range_guess_s_t,
    /// Initial estimate for entity 2.
    pub guess_2: PK_range_guess_s_t,
    /// Parametric bounds for entity 1.
    pub param_bound_1: PK_range_param_bound_t,
    /// Parametric bounds for entity 2.
    pub param_bound_2: PK_range_param_bound_t,
}

// =============================================================================
// Clash detection enums and structures
// =============================================================================

/// Classification of a clash between two topological entities.
pub type PK_TOPOL_clash_type_t = c_int;
/// Bounding topologies cross; entities share common volume/area/length.
pub const PK_TOPOL_clash_interfere_c: PK_TOPOL_clash_type_t = 0;
/// Bounding topologies touch but do not share common interior.
pub const PK_TOPOL_clash_abut_no_class_c: PK_TOPOL_clash_type_t = 1;
/// Entity A entirely contained within entity B.
pub const PK_TOPOL_clash_a_in_b_c: PK_TOPOL_clash_type_t = 2;
/// Entity B entirely contained within entity A.
pub const PK_TOPOL_clash_b_in_a_c: PK_TOPOL_clash_type_t = 3;
/// Wire body clash detected (no further classification).
pub const PK_TOPOL_clash_exists_c: PK_TOPOL_clash_type_t = 4;

/// Options for `PK_TOPOL_clash`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_clash_o_t {
    pub o_t_version: c_int,
    /// Find all clashes (`PK_LOGICAL_true`) or stop after first (`PK_LOGICAL_false`, default).
    pub find_all: PK_LOGICAL_t,
    /// Classify each clash type; populates `clash_types` in result.
    pub find_intersect: PK_LOGICAL_t,
    /// Supply per-target transforms.
    pub mul_target_tf: PK_LOGICAL_t,
    /// Supply per-tool transforms.
    pub mul_tool_tf: PK_LOGICAL_t,
    /// Owning body of targets (for face-level classification).
    pub target_owner: PK_BODY_t,
    /// Owning body of tools.
    pub tool_owner: PK_BODY_t,
}

impl Default for PK_TOPOL_clash_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            find_all: PK_LOGICAL_false,
            find_intersect: PK_LOGICAL_false,
            mul_target_tf: PK_LOGICAL_false,
            mul_tool_tf: PK_LOGICAL_false,
            target_owner: PK_ENTITY_null,
            tool_owner: PK_ENTITY_null,
        }
    }
}

/// Result structure for `PK_TOPOL_clash`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_clash_r_t {
    /// Number of clashes found.
    pub n_clashes: c_int,
    /// Array of target entities involved in clashes.
    pub targets: *mut PK_TOPOL_t,
    /// Array of tool entities involved in clashes.
    pub tools: *mut PK_TOPOL_t,
    /// Array of clash type classifications (populated when `find_intersect` is set).
    pub clash_types: *mut PK_TOPOL_clash_type_t,
}

// =============================================================================
// Intersection options structures
// =============================================================================

/// Options for `PK_CURVE_intersect_curve`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_intersect_curve_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Surface containing both curves (for parametric-space intersection).
    pub common_surf: PK_SURF_t,
}

/// Options for `PK_SURF_intersect_curve`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_intersect_curve_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
}

/// Options for `PK_FACE_intersect_face`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_intersect_face_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Parameter box for face 1.
    pub have_uvbox_1: PK_LOGICAL_t,
    pub uvbox_1: PK_UVBOX_t,
    /// Parameter box for face 2.
    pub have_uvbox_2: PK_LOGICAL_t,
    pub uvbox_2: PK_UVBOX_t,
}

/// Options for `PK_SURF_intersect_surf`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_intersect_surf_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
}

/// Options for `PK_FACE_intersect_surf`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_intersect_surf_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Parameter box for the face.
    pub have_uvbox_1: PK_LOGICAL_t,
    pub uvbox_1: PK_UVBOX_t,
    /// Parameter box for the surface.
    pub have_uvbox_2: PK_LOGICAL_t,
    pub uvbox_2: PK_UVBOX_t,
}

// =============================================================================
// Extern function declarations — Range (Chapter 26)
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // ---- Standard range (entity-to-entity) ----

    /// Global min/max distance between two geometrical entities.
    pub fn PK_GEOM_range(
        geom_1: PK_GEOM_t,
        geom_2: PK_GEOM_t,
        options: *const PK_GEOM_range_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between two topological entities.
    pub fn PK_TOPOL_range(
        topol_1: PK_TOPOL_t,
        topol_2: PK_TOPOL_t,
        options: *const PK_TOPOL_range_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between a topological entity and a geometric entity.
    pub fn PK_TOPOL_range_geom(
        topol: PK_TOPOL_t,
        geom: PK_GEOM_t,
        options: *const PK_TOPOL_range_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Array range (array-to-array) ----

    /// Global min/max distance between two arrays of geometrical entities.
    pub fn PK_GEOM_range_array(
        n_geoms_1: c_int,
        geoms_1: *const PK_GEOM_t,
        n_geoms_2: c_int,
        geoms_2: *const PK_GEOM_t,
        options: *const PK_GEOM_range_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between two arrays of topological entities.
    pub fn PK_TOPOL_range_array(
        n_topols_1: c_int,
        topols_1: *const PK_TOPOL_t,
        n_topols_2: c_int,
        topols_2: *const PK_TOPOL_t,
        options: *const PK_TOPOL_range_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between arrays of topological and geometric entities.
    pub fn PK_TOPOL_range_geom_array(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
        options: *const PK_TOPOL_range_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Vector range (entity-to-position) ----

    /// Global min distance between a geometrical entity and a position.
    pub fn PK_GEOM_range_vector(
        geom: PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *const PK_GEOM_range_vector_o_t,
        result: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distances between a geometrical entity and an array of positions.
    pub fn PK_GEOM_range_vector_many(
        geom: PK_GEOM_t,
        n_vectors: c_int,
        vectors: *const PK_VECTOR_t,
        options: *const PK_GEOM_range_vector_o_t,
        results: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between a topological entity and a position.
    pub fn PK_TOPOL_range_vector(
        topol: PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *const PK_TOPOL_range_vector_o_t,
        result: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between an array of geometrical entities and a position.
    pub fn PK_GEOM_range_array_vector(
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *const PK_GEOM_range_vector_o_t,
        result: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between an array of topological entities and a position.
    pub fn PK_TOPOL_range_array_vector(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *const PK_TOPOL_range_vector_o_t,
        result: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Local range ----

    /// Local min distance between two geometrical entities.
    pub fn PK_GEOM_range_local(
        geom_1: PK_GEOM_t,
        geom_2: PK_GEOM_t,
        options: *const PK_GEOM_range_local_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min distance between two topological entities.
    pub fn PK_TOPOL_range_local(
        topol_1: PK_TOPOL_t,
        topol_2: PK_TOPOL_t,
        options: *const PK_TOPOL_range_o_t,
        result: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min distance between a geometrical entity and a position.
    pub fn PK_GEOM_range_local_vector(
        geom: PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *const PK_GEOM_range_vector_o_t,
        result: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min/max distance between a topological entity and a position.
    pub fn PK_TOPOL_range_local_vector(
        topol: PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *const PK_TOPOL_range_vector_o_t,
        result: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Clash detection (Chapter 27)
    // =========================================================================

    /// Detect clashes between two sets of topological entities.
    ///
    /// Receives target and tool topology sets, returns clashing entity pairs
    /// and optional classification.
    pub fn PK_TOPOL_clash(
        n_targets: c_int,
        targets: *const PK_TOPOL_t,
        n_tools: c_int,
        tools: *const PK_TOPOL_t,
        target_transf: PK_TRANSF_t,
        tool_transf: PK_TRANSF_t,
        options: *const PK_TOPOL_clash_o_t,
        result: *mut PK_TOPOL_clash_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Intersection functions (Chapter 54)
    // =========================================================================

    /// Find intersections between specified regions of two curves.
    ///
    /// Coincident intersections are returned at bounds of coincidence regions.
    pub fn PK_CURVE_intersect_curve(
        curve_1: PK_CURVE_t,
        interval_1: PK_INTERVAL_t,
        curve_2: PK_CURVE_t,
        interval_2: PK_INTERVAL_t,
        options: *const PK_CURVE_intersect_curve_o_t,
        n_points: *mut c_int,
        points: *mut *mut PK_VECTOR_t,
        params_1: *mut *mut c_double,
        params_2: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a surface and a curve.
    ///
    /// Coincident intersections are returned at bounds of coincidence regions.
    pub fn PK_SURF_intersect_curve(
        surf: PK_SURF_t,
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        options: *const PK_SURF_intersect_curve_o_t,
        n_points: *mut c_int,
        points: *mut *mut PK_VECTOR_t,
        params: *mut *mut c_double,
        uvs: *mut *mut PK_UV_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a face and the specified region of a curve.
    ///
    /// Intersections are ordered along the bounded curve and classified
    /// according to curve direction. No options structure.
    pub fn PK_FACE_intersect_curve(
        face: PK_FACE_t,
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        n_points: *mut c_int,
        points: *mut *mut PK_VECTOR_t,
        params: *mut *mut c_double,
        uvs: *mut *mut PK_UV_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between two faces.
    ///
    /// Same-body faces: curves created as construction geometry.
    /// Different-body faces: curves refer to copies of face surfaces.
    /// Fully coincident surfaces yield no intersection data.
    pub fn PK_FACE_intersect_face(
        face_1: PK_FACE_t,
        face_2: PK_FACE_t,
        options: *const PK_FACE_intersect_face_o_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        n_points: *mut c_int,
        points: *mut *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between two surfaces.
    ///
    /// Both surfaces must be orphans or from the same body.
    /// Fully coincident surfaces yield no intersection data.
    pub fn PK_SURF_intersect_surf(
        surf_1: PK_SURF_t,
        surf_2: PK_SURF_t,
        options: *const PK_SURF_intersect_surf_o_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        n_points: *mut c_int,
        points: *mut *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a face and a surface.
    ///
    /// Surface must be orphan or owned by the same body as the face.
    /// Fully coincident surfaces yield no intersection data.
    pub fn PK_FACE_intersect_surf(
        face: PK_FACE_t,
        surf: PK_SURF_t,
        options: *const PK_FACE_intersect_surf_o_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        n_points: *mut c_int,
        points: *mut *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;
}
