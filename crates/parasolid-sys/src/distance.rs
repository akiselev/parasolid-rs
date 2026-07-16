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
pub const PK_range_type_minimum_c: PK_range_type_t = 22820;
/// Find maximum distance.
pub const PK_range_type_maximum_c: PK_range_type_t = 22821;

/// Optimization level for range computation.
pub type PK_range_opt_t = c_int;
/// Optimize for performance (default); may return local extremum.
pub const PK_range_opt_performance_c: PK_range_opt_t = 23760;
/// Optimize for accuracy; slower but more reliable global result.
pub const PK_range_opt_accuracy_c: PK_range_opt_t = 23761;

/// Result status of a range computation.
pub type PK_range_result_t = c_int;
/// Min/max distance successfully found.
pub const PK_range_result_found_c: PK_range_result_t = 18270;
/// No distance greater than supplied lower_bound found.
pub const PK_range_result_lower_c: PK_range_result_t = 18271;
/// No distance less than supplied upper_bound found.
pub const PK_range_result_upper_c: PK_range_result_t = 18272;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_range_result_not_found_c: PK_range_result_t = 18273;

/// Type of initial estimate supplied to a range function.
pub type PK_range_guess_t = c_int;
/// No estimate (default).
pub const PK_range_guess_no_c: PK_range_guess_t = 18260;
/// Parameter estimate.
pub const PK_range_guess_param_c: PK_range_guess_t = 18261;
/// Position estimate.
pub const PK_range_guess_vector_c: PK_range_guess_t = 18262;

/// Which entity level the endpoint on the found sub-topology refers to
/// (`PK_TOPOL_range_vector`).
pub type PK_range_param_entity_t = c_int;
/// Report the containing topological entity (default).
pub const PK_range_param_entity_topol_c: PK_range_param_entity_t = 24990;
/// Report the sub-entity.
pub const PK_range_param_entity_sub_c: PK_range_param_entity_t = 24991;

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
///
/// NOTE (decompile-verified): the r_t does **not** carry a status field — the
/// status is the separate `range_result` out-param. `distance` is at offset 0.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_2_r_t {
    /// Computed distance.
    pub distance: c_double,      // @0
    /// Details for the first entity endpoint.
    pub end_1: PK_range_end_t,   // @8
    /// Details for the second entity endpoint.
    pub end_2: PK_range_end_t,   // @56
}

/// Result of a range computation between an entity and a position vector.
/// `distance` is at offset 0 (no status field — see [`PK_range_2_r_t`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_1_r_t {
    /// Computed distance.
    pub distance: c_double,   // @0
    /// Details for the entity endpoint.
    pub end: PK_range_end_t,  // @8
}

// =============================================================================
// Range options structures
// =============================================================================

/// Optional lower/upper distance bounds for a range computation.
///
/// **A 32-byte STRUCT, not the `PK_bound_t` enum** — recovered by decompiling
/// the field validator `FUN_181106a20`: it reads `have_lower_bound`/
/// `have_upper_bound` as 0/1 flags and the two doubles only when the flag is
/// TRUE. All-zero == "no bound" and validates cleanly (the default).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_range_bound_t {
    pub have_lower_bound: PK_LOGICAL_t, // @0
    pub lower_bound: c_double,          // @8
    pub have_upper_bound: PK_LOGICAL_t, // @16
    pub upper_bound: c_double,          // @24
} // 32 bytes

impl Default for PK_range_bound_t {
    fn default() -> Self {
        Self {
            have_lower_bound: PK_LOGICAL_false,
            lower_bound: 0.0,
            have_upper_bound: PK_LOGICAL_false,
            upper_bound: 0.0,
        }
    }
}

/// Options for entity-to-entity range functions (`PK_TOPOL_range`).
///
/// Authoritative **152-byte** layout recovered by decompiling `PK_TOPOL_range`
/// (V37.01.243): `bound` is a 32-byte `PK_range_bound_t` @16, the two `guesses`
/// are 48-byte `PK_range_guess_s_t` @48/@96, `range_type`@144, `opt_level`@148.
/// The catalog's 40-byte `bound:int` layout was wrong (that dead-ended on err 908).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_range_o_t {
    pub o_t_version: c_int,               // @0
    pub have_tolerance: PK_LOGICAL_t,     // @4
    pub tolerance: c_double,              // @8
    pub bound: PK_range_bound_t,          // @16  (32B)
    pub guesses: [PK_range_guess_s_t; 2], // @48  (96B)
    pub range_type: PK_range_type_t,      // @144
    pub opt_level: PK_range_opt_t,        // @148
} // 152 bytes

impl Default for PK_TOPOL_range_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            bound: PK_range_bound_t::default(),
            guesses: [PK_range_guess_s_t {
                guess_type: PK_range_guess_no_c,
                parameters: [0.0, 0.0],
                vector: [0.0, 0.0, 0.0],
            }; 2],
            range_type: PK_range_type_minimum_c,
            opt_level: PK_range_opt_accuracy_c,
        }
    }
}

/// Options for entity-to-vector range functions (`PK_TOPOL_range_vector`).
///
/// Authoritative **104-byte** layout recovered by decompiling
/// `PK_TOPOL_range_vector`: `bound` is a 32-byte struct @16, `guess` a 48-byte
/// `PK_range_guess_s_t` @48, `opt_level`@96, `param_entity`@100.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_range_vector_o_t {
    pub o_t_version: c_int,                    // @0
    pub have_tolerance: PK_LOGICAL_t,          // @4
    pub tolerance: c_double,                   // @8
    pub bound: PK_range_bound_t,               // @16 (32B)
    pub guess: PK_range_guess_s_t,             // @48 (48B)
    pub opt_level: PK_range_opt_t,             // @96
    pub param_entity: PK_range_param_entity_t, // @100
} // 104 bytes

impl Default for PK_TOPOL_range_vector_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            bound: PK_range_bound_t::default(),
            guess: PK_range_guess_s_t {
                guess_type: PK_range_guess_no_c,
                parameters: [0.0, 0.0],
                vector: [0.0, 0.0, 0.0],
            },
            opt_level: PK_range_opt_accuracy_c,
            param_entity: PK_range_param_entity_topol_c,
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<PK_range_bound_t>() == 32);
    assert!(core::mem::size_of::<PK_range_guess_s_t>() == 48);
    assert!(core::mem::size_of::<PK_TOPOL_range_o_t>() == 152);
    assert!(core::mem::size_of::<PK_TOPOL_range_vector_o_t>() == 104);
};

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
    /// Reserved (`interest`) — kept for correct v1 struct size (64 bytes).
    pub _interest_reserved: c_int,
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
    /// Point of interest to seed the intersection.
    pub have_vector: PK_LOGICAL_t,
    pub vector: PK_VECTOR_t,
    /// Mixed-dimension curve category (`PK_mixed_intersection_t`).
    pub mixed_curve_category: c_int,
    /// Intersection tolerance.
    pub tolerance: c_double,
    /// Reserved (`use`) — kept for correct v1 struct size (192 bytes).
    pub _use_reserved: c_int,
}

/// Options for `PK_SURF_intersect_surf`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_intersect_surf_o_t {
    pub o_t_version: c_int,
    /// 3-space bounding box of interest.
    pub have_box: PK_LOGICAL_t,
    pub r#box: PK_BOX_t,
    /// Parameter box for surface 1.
    pub have_uvbox_1: PK_LOGICAL_t,
    pub uvbox_1: PK_UVBOX_t,
    /// Parameter box for surface 2.
    pub have_uvbox_2: PK_LOGICAL_t,
    pub uvbox_2: PK_UVBOX_t,
    /// Point of interest to seed the intersection.
    pub have_vector: PK_LOGICAL_t,
    pub vector: PK_VECTOR_t,
    /// Mixed-dimension curve category (`PK_mixed_intersection_t`).
    pub mixed_curve_category: c_int,
    /// Intersection tolerance.
    pub tolerance: c_double,
    /// Reserved (`use`) — kept for correct v1 struct size (192 bytes).
    pub _use_reserved: c_int,
}

// Intersection classification tokens. Each family has its own value range;
// only the base "simple/transversal" token of each is confirmed so far.
// [dynamic-observed] — the values are computed deep in the internal intersection
// engine (not read out of the public wrapper or its immediate callees, so a
// static enumeration would need several layers of stripped-function tracing);
// tangential / coincident / etc. tokens need tangency & overlap fixtures, which
// in turn need standalone-surface creation (`PK_PLANE_create` &c., not yet
// wrapped). Treat any value other than the `*_simple_c` below as opaque.

/// Type of an intersection curve from the surf/face intersection functions
/// (`PK_intersect_curve_t`).
pub type PK_intersect_curve_t = c_int;
/// A transversal (clean, non-tangential) intersection curve. [dynamic-observed]
/// Seen for plane∩plane (line), cyl∩plane (circle), face∩face, face∩surf.
pub const PK_intersect_curve_simple_c: PK_intersect_curve_t = 14651; // 0x393b
/// A tangential intersection curve (the surfaces touch without crossing).
/// [dynamic-observed] Seen for a plane tangent to a cylinder (tangent line).
pub const PK_intersect_curve_tangent_c: PK_intersect_curve_t = 14652; // 0x393c

/// Type of a point intersection from `PK_CURVE_intersect_curve` /
/// `PK_SURF_intersect_curve` (`PK_intersect_vector_t`).
pub type PK_intersect_vector_t = c_int;
/// A transversal point intersection. [dynamic-observed] Seen for curve∩curve
/// (two lines crossing) and surf∩curve (line piercing a plane).
pub const PK_intersect_vector_simple_c: PK_intersect_vector_t = 14611; // 0x3913
// [re-abi] appended 3 missing member(s) from pk-enums.h
pub const PK_intersect_vector_tangent_c: PK_intersect_vector_t = 14612;
pub const PK_intersect_vector_start_c: PK_intersect_vector_t = 14613;
pub const PK_intersect_vector_end_c: PK_intersect_vector_t = 14614;

/// Type of a face/curve point intersection from `PK_FACE_intersect_curve`
/// (`PK_intersect_fc_t`).
pub type PK_intersect_fc_t = c_int;
/// A transversal face/curve point intersection. [dynamic-observed] Seen for a
/// line piercing a planar face.
pub const PK_intersect_fc_simple_c: PK_intersect_fc_t = 14801; // 0x39d1
// [re-abi] appended 12 missing member(s) from pk-enums.h
pub const PK_intersect_fc_tangent_c: PK_intersect_fc_t = 14802;
pub const PK_intersect_fc_out_in_c: PK_intersect_fc_t = 14803;
pub const PK_intersect_fc_in_out_c: PK_intersect_fc_t = 14804;
pub const PK_intersect_fc_out_coi_c: PK_intersect_fc_t = 14805;
pub const PK_intersect_fc_coi_out_c: PK_intersect_fc_t = 14806;
pub const PK_intersect_fc_coi_in_c: PK_intersect_fc_t = 14807;
pub const PK_intersect_fc_in_coi_c: PK_intersect_fc_t = 14808;
pub const PK_intersect_fc_in_tangent_c: PK_intersect_fc_t = 14809;
pub const PK_intersect_fc_out_tangent_c: PK_intersect_fc_t = 14810;
pub const PK_intersect_fc_in_c: PK_intersect_fc_t = 14811;
pub const PK_intersect_fc_start_c: PK_intersect_fc_t = 14812;
pub const PK_intersect_fc_end_c: PK_intersect_fc_t = 14813;

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
    /// Point of interest to seed the intersection.
    pub have_vector: PK_LOGICAL_t,
    pub vector: PK_VECTOR_t,
    /// Mixed-dimension curve category (`PK_mixed_intersection_t`).
    pub mixed_curve_category: c_int,
    /// Intersection tolerance.
    pub tolerance: c_double,
    /// Reserved (`use`) — kept for correct v1 struct size (192 bytes).
    pub _use_reserved: c_int,
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
        options: *mut PK_GEOM_range_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between two topological entities.
    pub fn PK_TOPOL_range(
        topol_1: PK_TOPOL_t,
        topol_2: PK_TOPOL_t,
        options: *mut PK_TOPOL_range_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between a topological entity and a geometric entity.
    pub fn PK_TOPOL_range_geom(
        topol: PK_TOPOL_t,
        geom: PK_GEOM_t,
        options: *mut PK_TOPOL_range_geom_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Array range (array-to-array) ----

    /// Global min/max distance between two arrays of geometrical entities.
    pub fn PK_GEOM_range_array(
        n_geoms_1: c_int,
        geoms_1: *mut PK_GEOM_t,
        n_geoms_2: c_int,
        geoms_2: *mut PK_GEOM_t,
        options: *mut PK_GEOM_range_array_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between two arrays of topological entities.
    pub fn PK_TOPOL_range_array(
        n_topols_1: c_int,
        topols_1: *mut PK_TOPOL_t,
        n_topols_2: c_int,
        topols_2: *mut PK_TOPOL_t,
        options: *mut PK_TOPOL_range_array_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min/max distance between arrays of topological and geometric entities.
    pub fn PK_TOPOL_range_geom_array(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        n_geoms: c_int,
        geoms: *mut PK_GEOM_t,
        options: *mut PK_TOPOL_range_geom_array_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Vector range (entity-to-position) ----

    /// Global min distance between a geometrical entity and a position.
    pub fn PK_GEOM_range_vector(
        geom: PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_GEOM_range_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distances between a geometrical entity and an array of positions.
    pub fn PK_GEOM_range_vector_many(
        geom: PK_GEOM_t,
        n_vectors: c_int,
        vectors: *mut PK_VECTOR_t,
        options: *mut PK_GEOM_range_vector_many_o_t,
        range_results: *mut PK_range_result_t,
        ranges: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between a topological entity and a position.
    pub fn PK_TOPOL_range_vector(
        topol: PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_TOPOL_range_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between an array of geometrical entities and a position.
    pub fn PK_GEOM_range_array_vector(
        n_geoms: c_int,
        geoms: *mut PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_GEOM_range_array_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Global min distance between an array of topological entities and a position.
    pub fn PK_TOPOL_range_array_vector(
        n_topols: c_int,
        topols: *mut PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_TOPOL_range_array_vector_o_t,
        range_result: *mut PK_range_result_t,
        range: *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    // ---- Local range ----

    /// Local min distance between two geometrical entities.
    pub fn PK_GEOM_range_local(
        geom_1: PK_GEOM_t,
        geom_2: PK_GEOM_t,
        options: *mut PK_GEOM_range_local_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min distance between two topological entities.
    pub fn PK_TOPOL_range_local(
        topol_1: PK_TOPOL_t,
        topol_2: PK_TOPOL_t,
        options: *mut PK_TOPOL_range_local_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min distance between a geometrical entity and a position.
    pub fn PK_GEOM_range_local_vector(
        geom: PK_GEOM_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_GEOM_range_local_vector_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_1_r_t,
    ) -> PK_ERROR_code_t;

    /// Local min/max distance between a topological entity and a position.
    pub fn PK_TOPOL_range_local_vector(
        topol: PK_TOPOL_t,
        vector: *const PK_VECTOR_t,
        options: *mut PK_TOPOL_range_local_vector_o_t,
        n_ranges: *mut c_int,
        ranges: *mut *mut PK_range_1_r_t,
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
        targets: *mut PK_TOPOL_t,
        tf1: *mut PK_TRANSF_t,
        n_tools: c_int,
        tools: *mut PK_TOPOL_t,
        tf2: *mut PK_TRANSF_t,
        options: *mut PK_TOPOL_clash_o_t,
        n_clash: *mut c_int,
        clashes: *mut *mut PK_TOPOL_clash_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Intersection functions (Chapter 54)
    // =========================================================================

    /// Find intersections between specified regions of two curves. [documented]
    ///
    /// Outputs: `n_vectors`/`vectors` (positions), `ts_1`/`ts_2` (parameters on
    /// each curve), and `types` (`PK_intersect_vector_t`). The earlier binding
    /// dropped the trailing `types` output, so the kernel wrote it through an
    /// uninitialised pointer.
    pub fn PK_CURVE_intersect_curve(
        curve_1: PK_CURVE_t,
        interval_1: PK_INTERVAL_t,
        curve_2: PK_CURVE_t,
        interval_2: PK_INTERVAL_t,
        options: *const PK_CURVE_intersect_curve_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        ts_1: *mut *mut c_double,
        ts_2: *mut *mut c_double,
        types: *mut *mut PK_intersect_vector_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a surface and a curve. [documented]
    ///
    /// Outputs: `n_vectors`/`vectors`, `uvs` (surface params), `ts` (curve
    /// params), `types` (`PK_intersect_vector_t`). The earlier binding swapped
    /// the `uvs`/`ts` order and dropped `types`.
    pub fn PK_SURF_intersect_curve(
        surf: PK_SURF_t,
        curve: PK_CURVE_t,
        bounds: PK_INTERVAL_t,
        options: *const PK_SURF_intersect_curve_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        uvs: *mut *mut PK_UV_t,
        ts: *mut *mut c_double,
        types: *mut *mut PK_intersect_vector_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a face and the specified region of a curve.
    /// No options structure. [documented]
    ///
    /// Outputs: `n_vectors`/`vectors`, `uvs` (face-surface params), `ts` (curve
    /// params), `topols` (topology hit at each point), `types`
    /// (`PK_intersect_fc_t`). The earlier binding swapped `uvs`/`ts` and dropped
    /// the `topols` and `types` outputs.
    pub fn PK_FACE_intersect_curve(
        face: PK_FACE_t,
        curve: PK_CURVE_t,
        bounds: PK_INTERVAL_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        uvs: *mut *mut PK_UV_t,
        ts: *mut *mut c_double,
        topols: *mut *mut PK_TOPOL_t,
        types: *mut *mut PK_intersect_fc_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between two faces. [documented]
    ///
    /// Six outputs, point intersections first then curves, matching
    /// `PK_SURF_intersect_surf`. The earlier binding had only four outputs in
    /// swapped order and dropped `bounds`/`types`.
    pub fn PK_FACE_intersect_face(
        face_1: PK_FACE_t,
        face_2: PK_FACE_t,
        options: *const PK_FACE_intersect_face_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        bounds: *mut *mut PK_INTERVAL_t,
        types: *mut *mut PK_intersect_curve_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between two surfaces.
    ///
    /// Both surfaces must be orphans or from the same body.
    /// Fully coincident surfaces yield no intersection data.
    /// Intersect two surfaces.
    ///
    /// [documented] + [static-observed]: the real signature has **six** output
    /// arguments in this order — point intersections first, then curves with
    /// their parameter bounds and types. The earlier binding had only four
    /// outputs in swapped order (`n_curves, curves, n_points, points`) and was
    /// missing `bounds`/`types`, so the kernel wrote curve bounds/types through
    /// uninitialised pointers. `bounds[i]` is the parameter interval of
    /// `curves[i]`; `types[i]` is its `PK_intersect_curve_t`.
    pub fn PK_SURF_intersect_surf(
        surf_1: PK_SURF_t,
        surf_2: PK_SURF_t,
        options: *const PK_SURF_intersect_surf_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        bounds: *mut *mut PK_INTERVAL_t,
        types: *mut *mut PK_intersect_curve_t,
    ) -> PK_ERROR_code_t;

    /// Find intersections between a face and a surface. [documented]
    ///
    /// Six outputs, point intersections first then curves, matching
    /// `PK_SURF_intersect_surf`. The earlier binding had only four outputs in
    /// swapped order and dropped `bounds`/`types`.
    pub fn PK_FACE_intersect_surf(
        face: PK_FACE_t,
        surf: PK_SURF_t,
        options: *const PK_FACE_intersect_surf_o_t,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
        bounds: *mut *mut PK_INTERVAL_t,
        types: *mut *mut PK_intersect_curve_t,
    ) -> PK_ERROR_code_t;
}
