//! Enquiry and interrogation functions — topological classification, geometric output,
//! connectivity queries, parametric evaluation, bounding boxes, containment, coincidence,
//! convexity, discontinuity, self-intersection, and degeneracy checks.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_double, c_int};

use crate::*;

// =============================================================================
// Enum / constant types for enquiry results
// =============================================================================

// -- Body type ----------------------------------------------------------------


// -- Fin type -----------------------------------------------------------------

pub type PK_FIN_type_t = c_int;

// -- Shell type ---------------------------------------------------------------

// -- Vertex type --------------------------------------------------------------

// -- Loop type ----------------------------------------------------------------

// -- Region type --------------------------------------------------------------

// -- Edge convexity -----------------------------------------------------------

pub type PK_EDGE_convexity_t = c_int;
pub const PK_EDGE_convexity_convex_c: PK_EDGE_convexity_t = 0;
pub const PK_EDGE_convexity_concave_c: PK_EDGE_convexity_t = 1;
pub const PK_EDGE_convexity_variable_c: PK_EDGE_convexity_t = 2;
pub const PK_EDGE_convexity_smooth_flat_c: PK_EDGE_convexity_t = 3;
pub const PK_EDGE_convexity_smooth_cvx_c: PK_EDGE_convexity_t = 4;
pub const PK_EDGE_convexity_smooth_ccv_c: PK_EDGE_convexity_t = 5;
pub const PK_EDGE_convexity_smooth_inf_c: PK_EDGE_convexity_t = 6;
pub const PK_EDGE_convexity_smooth_var_c: PK_EDGE_convexity_t = 7;
pub const PK_EDGE_convexity_knife_cvx_c: PK_EDGE_convexity_t = 8;
pub const PK_EDGE_convexity_knife_ccv_c: PK_EDGE_convexity_t = 9;

// -- Face coincidence ---------------------------------------------------------

pub type PK_FACE_coi_t = c_int;
pub const PK_FACE_coi_yes_c: PK_FACE_coi_t = 0;
pub const PK_FACE_coi_yes_reversed_c: PK_FACE_coi_t = 1;
pub const PK_FACE_coi_no_topol_c: PK_FACE_coi_t = 2;
pub const PK_FACE_coi_no_bound_1_c: PK_FACE_coi_t = 3;
pub const PK_FACE_coi_no_bound_2_c: PK_FACE_coi_t = 4;
pub const PK_FACE_coi_no_face_1_c: PK_FACE_coi_t = 5;
pub const PK_FACE_coi_no_face_2_c: PK_FACE_coi_t = 6;
pub const PK_FACE_coi_no_rubber_c: PK_FACE_coi_t = 7;

// -- Handedness (for evaluation at discontinuities) ---------------------------

pub type PK_HAND_t = c_int;
pub const PK_HAND_left_c: PK_HAND_t = 0;
pub const PK_HAND_right_c: PK_HAND_t = 1;

// -- Containment --------------------------------------------------------------

pub type PK_CONTAINMENT_t = c_int;
pub const PK_CONTAINMENT_in_c: PK_CONTAINMENT_t = 0;
pub const PK_CONTAINMENT_out_c: PK_CONTAINMENT_t = 1;
pub const PK_CONTAINMENT_on_c: PK_CONTAINMENT_t = 2;

// -- Shell sign ---------------------------------------------------------------

pub type PK_SHELL_sign_t = c_int;
pub const PK_SHELL_sign_positive_c: PK_SHELL_sign_t = 1;
pub const PK_SHELL_sign_negative_c: PK_SHELL_sign_t = -1;

// -- Measure type (for PK_CURVE_find_vectors) ---------------------------------

pub type PK_measure_t = c_int;
pub const PK_measure_distance_c: PK_measure_t = 0;
pub const PK_measure_chord_c: PK_measure_t = 1;
pub const PK_measure_distance_ratio_c: PK_measure_t = 2;
pub const PK_measure_chord_ratio_c: PK_measure_t = 3;
pub const PK_measure_2_chords_ratio_c: PK_measure_t = 4;

// -- Surface vector type (for PK_SURF_find_vectors) ---------------------------

pub type PK_SURF_vec_t = c_int;
pub const PK_SURF_vec_proj_c: PK_SURF_vec_t = 0;

pub type PK_SURF_curve_t = c_int;
pub const PK_SURF_curve_linear_sp_c: PK_SURF_curve_t = 0;
pub const PK_SURF_curve_vec_proj_c: PK_SURF_curve_t = 1;

// -- Continuity level ---------------------------------------------------------

pub const PK_continuity_g1_c: PK_continuity_t = 1;
pub const PK_continuity_g2_c: PK_continuity_t = 2;
pub const PK_continuity_g3_c: PK_continuity_t = 3;

// -- Self-intersection type ---------------------------------------------------

pub type PK_self_int_type_t = c_int;
pub const PK_self_int_type_general_c: PK_self_int_type_t = 0;
pub const PK_self_int_type_singularity_c: PK_self_int_type_t = 1;
pub const PK_self_int_type_mixed_c: PK_self_int_type_t = 2;

// -- Surface degeneracy type --------------------------------------------------

pub type PK_SURF_degen_type_t = c_int;
pub const PK_SURF_degen_parametric_c: PK_SURF_degen_type_t = 0;
pub const PK_SURF_degen_phys_concave_c: PK_SURF_degen_type_t = 1;
pub const PK_SURF_degen_phys_convex_c: PK_SURF_degen_type_t = 2;
pub const PK_SURF_degen_phys_mixed_c: PK_SURF_degen_type_t = 3;
pub const PK_SURF_degen_undefined_c: PK_SURF_degen_type_t = 4;

// -- Curve degeneracy type ----------------------------------------------------

pub type PK_CURVE_degen_type_t = c_int;
pub const PK_CURVE_degen_parametric_c: PK_CURVE_degen_type_t = 0;
pub const PK_CURVE_degen_physical_c: PK_CURVE_degen_type_t = 1;
pub const PK_CURVE_degen_surface_c: PK_CURVE_degen_type_t = 2;

// -- Geometry category --------------------------------------------------------

pub type PK_GEOM_category_t = c_int;
pub const PK_GEOM_category_curve_c: PK_GEOM_category_t = 0;
pub const PK_GEOM_category_surf_c: PK_GEOM_category_t = 1;
pub const PK_GEOM_category_point_c: PK_GEOM_category_t = 2;
pub const PK_GEOM_category_transf_c: PK_GEOM_category_t = 3;

// -- NABOX quality ------------------------------------------------------------

pub type PK_NABOX_quality_t = c_int;
pub const PK_NABOX_quality_improved_c: PK_NABOX_quality_t = 1;

// -- Error on failure ---------------------------------------------------------

pub const PK_ERROR_on_fail_no_c: c_int = 0;
pub const PK_ERROR_not_implemented: PK_ERROR_code_t = 600;

// =============================================================================
// Standard-form structs for geometric _ask queries
// =============================================================================

/// Non axis-aligned bounding box.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_NABOX_sf_t {
    pub basis_set: [[c_double; 3]; 3], // 3 orthogonal axes
    pub coord: [c_double; 6],          // [min1, min2, min3, max1, max2, max3]
}

/// Foreign curve standard form.
///
/// Passed to `PK_FCURVE_create`; filled by `PK_FCURVE_ask`.
/// - `keylen`   — byte length of the evaluator key string.
/// - `key`      — pointer to the evaluator key string (not NUL-terminated by convention).
/// - `nspace`   — number of `double` slots available to the evaluator in its working space.
/// - `n_kii`    — number of integers in the KI integer array.
/// - `ki_ints`  — pointer to the KI integer array (length `n_kii`).
/// - `n_kir`    — number of reals in the KI real array.
/// - `ki_reals` — pointer to the KI real array (length `n_kir`).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FCURVE_sf_t {
    pub keylen: c_int,
    pub key: *const c_char,
    pub nspace: c_int,
    pub n_kii: c_int,
    pub ki_ints: *const c_int,
    pub n_kir: c_int,
    pub ki_reals: *const c_double,
}

/// Foreign surface standard form.
///
/// Same field layout as `PK_FCURVE_sf_t`; used with `PK_FSURF_create` and
/// `PK_FSURF_ask`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FSURF_sf_t {
    pub keylen: c_int,
    pub key: *const c_char,
    pub nspace: c_int,
    pub n_kii: c_int,
    pub ki_ints: *const c_int,
    pub n_kir: c_int,
    pub ki_reals: *const c_double,
}

// =============================================================================
// Options structs
// =============================================================================

/// Options for PK_TOPOL_find_nabox.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_find_nabox_o_t {
    pub have_axis1: PK_LOGICAL_t,
    pub axis1: PK_VECTOR_t,
    pub have_axis2: PK_LOGICAL_t,
    pub axis2: PK_VECTOR_t,
    pub quality: PK_NABOX_quality_t,
}

/// Options for PK_FACE_is_coincident.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_is_coincident_o_t {
    pub have_transf_1: PK_LOGICAL_t,
    pub transf_1: PK_TRANSF_t,
    pub have_transf_2: PK_LOGICAL_t,
    pub transf_2: PK_TRANSF_t,
    pub tolerance: c_double,
}

/// Options for PK_BODY_contains_vector.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_contains_vector_o_t {
    pub have_direction: PK_LOGICAL_t,
    pub direction: PK_VECTOR_t,
    pub have_transf: PK_LOGICAL_t,
    pub transf: PK_TRANSF_t,
}

/// Options for PK_FACE_contains_vectors.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_contains_vectors_o_t {
    pub tolerance: c_double,
}

/// Options for PK_TOPOL_find_box.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_find_box_o_t {
    pub have_transf: PK_LOGICAL_t,
    pub transf: PK_TRANSF_t,
}

/// Options for PK_CURVE_find_vectors.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_find_vectors_o_t {
    pub measure: PK_measure_t,
}

/// Options for PK_SURF_find_vectors.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_find_vectors_o_t {
    pub curve_type: PK_SURF_curve_t,
    pub direction: PK_VECTOR_t,
}

/// Options for PK_BODY_find_extreme.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_find_extreme_o_t {
    pub have_transf: PK_LOGICAL_t,
    pub transf: PK_TRANSF_t,
}

/// Options for PK_FACE_find_extreme.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_find_extreme_o_t {
    pub have_transf: PK_LOGICAL_t,
    pub transf: PK_TRANSF_t,
}

/// Options for PK_EDGE_find_extreme.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_find_extreme_o_t {
    pub have_transf: PK_LOGICAL_t,
    pub transf: PK_TRANSF_t,
}

/// Options for PK_CURVE_find_discontinuity.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_find_discontinuity_o_t {
    pub continuity: PK_continuity_t,
    pub interval: PK_INTERVAL_t,
}

/// Options for PK_SURF_find_discontinuity.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_find_discontinuity_o_t {
    pub continuity: PK_continuity_t,
    pub uvbox: PK_UVBOX_t,
}

/// Options for PK_CURVE_find_self_int.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_find_self_int_o_t {
    pub interval: PK_INTERVAL_t,
}

/// Options for PK_SURF_find_self_int.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_find_self_int_o_t {
    pub uvbox: PK_UVBOX_t,
}

/// Options for PK_SURF_find_degens.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_find_degens_o_t {
    pub uvbox: PK_UVBOX_t,
}

/// Options for PK_CURVE_find_degens.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_find_degens_o_t {
    pub interval: PK_INTERVAL_t,
}

/// Options for PK_BCURVE_find_g1_discontinuity.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BCURVE_find_g1_disc_o_t {
    pub interval: PK_INTERVAL_t,
}

/// Options for PK_BSURF_find_g1_discontinuity.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BSURF_find_g1_disc_o_t {
    pub uvbox: PK_UVBOX_t,
}

/// Options for PK_GEOM_is_coincident.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_is_coincident_o_t {
    pub tolerance: c_double,
}

/// Options for PK_BODY_identify_general.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_identify_general_o_t {
    pub _reserved: c_int,
}

/// Options for PK_SURF_parameterise_vector.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_parameterise_vector_o_t {
    pub _reserved: c_int,
}

/// Options for PK_CURVE_find_min_radius.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_find_min_radius_o_t {
    pub interval: PK_INTERVAL_t,
}

/// Options for PK_SURF_find_min_radii.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_find_min_radii_o_t {
    pub uvbox: PK_UVBOX_t,
}

// =============================================================================
// Oriented topology / geometry result structs
// =============================================================================

/// Oriented surface result from PK_FACE_ask_oriented_surf.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_oriented_surf_t {
    pub surf: PK_SURF_t,
    pub sense: PK_LOGICAL_t, // true => same as face normal
}

/// Oriented curve result from PK_EDGE_ask_oriented_curve / PK_FIN_ask_oriented_curve.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_oriented_curve_t {
    pub curve: PK_CURVE_t,
    pub sense: PK_LOGICAL_t,
}

/// Oriented edge result from PK_VERTEX_ask_oriented_edges.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_VERTEX_oriented_edge_t {
    pub edge: PK_EDGE_t,
    pub sense: PK_LOGICAL_t,
}

/// Oriented face result from PK_SHELL_ask_oriented_faces.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SHELL_oriented_face_t {
    pub face: PK_FACE_t,
    pub sense: PK_LOGICAL_t,
}

// =============================================================================
// Body topology result struct
// =============================================================================

/// Result of PK_BODY_ask_topology — all topology in one call.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_topology_t {
    pub n_regions: c_int,
    pub regions: *mut PK_REGION_t,
    pub n_shells: c_int,
    pub shells: *mut PK_SHELL_t,
    pub n_faces: c_int,
    pub faces: *mut PK_FACE_t,
    pub n_loops: c_int,
    pub loops: *mut PK_LOOP_t,
    pub n_fins: c_int,
    pub fins: *mut PK_FIN_t,
    pub n_edges: c_int,
    pub edges: *mut PK_EDGE_t,
    pub n_vertices: c_int,
    pub vertices: *mut PK_VERTEX_t,
}

// =============================================================================
// Opaque options/result structs
// =============================================================================

/// Options for `PK_TOPOL_find_box_2`.
#[repr(C)]
pub struct PK_TOPOL_find_box_2_o_t { _private: [u8; 0] }

/// Results from `PK_TOPOL_find_box_2`.
#[repr(C)]
pub struct PK_TOPOL_find_box_2_r_t { _private: [u8; 0] }

/// Options for `PK_TOPOL_find_connected`.
#[repr(C)]
pub struct PK_TOPOL_find_connected_o_t { _private: [u8; 0] }

/// Results from `PK_TOPOL_find_connected`.
#[repr(C)]
pub struct PK_TOPOL_find_connected_r_t { _private: [u8; 0] }

/// Options for `PK_TOPOL_is_connected`.
#[repr(C)]
pub struct PK_TOPOL_is_connected_o_t { _private: [u8; 0] }

/// Results from `PK_TOPOL_is_connected`.
#[repr(C)]
pub struct PK_TOPOL_is_connected_r_t { _private: [u8; 0] }

/// Options for `PK_TOPOL_make_new`.
#[repr(C)]
pub struct PK_TOPOL_make_new_o_t { _private: [u8; 0] }

/// Options for `PK_ENTITY_copy_2`.
#[repr(C)]
pub struct PK_ENTITY_copy_o_t { _private: [u8; 0] }

/// Options for `PK_BODY_ask_topology`.
#[repr(C)]
pub struct PK_BODY_ask_topology_o_t { _private: [u8; 0] }

/// Options for `PK_ENTITY_ask_description`.
#[repr(C)]
pub struct PK_ENTITY_ask_description_o_t { _private: [u8; 0] }

/// Options for `PK_ENTITY_range`.
#[repr(C)]
pub struct PK_ENTITY_range_o_t { _private: [u8; 0] }

/// Results from `PK_ENTITY_range`.
#[repr(C)]
pub struct PK_ENTITY_range_r_t { _private: [u8; 0] }

/// Options for `PK_ENTITY_range_vector`.
#[repr(C)]
pub struct PK_ENTITY_range_vector_o_t { _private: [u8; 0] }

/// Results from `PK_ENTITY_range_vector`.
#[repr(C)]
pub struct PK_ENTITY_range_vector_r_t { _private: [u8; 0] }

/// Options for `PK_LOOP_offset_planar`.
#[repr(C)]
pub struct PK_LOOP_offset_planar_o_t { _private: [u8; 0] }

/// Results from `PK_LOOP_offset_planar`.
#[repr(C)]
pub struct PK_LOOP_offset_planar_r_t { _private: [u8; 0] }

// =============================================================================
// FFI function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    // =========================================================================
    // Topological classification
    // =========================================================================

    pub fn PK_FACE_ask_type(face: PK_FACE_t, face_type: *mut c_int) -> PK_ERROR_code_t;

    // =========================================================================
    // Topological tests
    // =========================================================================

    // =========================================================================
    // Geometric standard form queries
    // =========================================================================

    /// Return the standard form of a foreign curve.
    pub fn PK_FCURVE_ask(
        fcurve: PK_FCURVE_t,
        fcurve_sf: *mut PK_FCURVE_sf_t,
    ) -> PK_ERROR_code_t;

    /// Return the standard form of a foreign surface.
    pub fn PK_FSURF_ask(
        fsurf: PK_FSURF_t,
        fsurf_sf: *mut PK_FSURF_sf_t,
    ) -> PK_ERROR_code_t;

    // B-curve / B-surface standard form and variants

    // =========================================================================
    // Geometry relationship queries
    // =========================================================================

    pub fn PK_TOPOL_categorise_geom(
        topol: PK_TOPOL_t,
        category: *mut PK_GEOM_category_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_GEOM_ask_dependents(
        geom: PK_GEOM_t,
        n_dependents: *mut c_int,
        dependents: *mut *mut PK_GEOM_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_GEOM_ask_geom_owners(
        geom: PK_GEOM_t,
        n_owners: *mut c_int,
        owners: *mut *mut PK_GEOM_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_GEOM_ask_geom_category(
        geom: PK_GEOM_t,
        category: *mut PK_GEOM_category_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body topology queries
    // =========================================================================

    /// Returns all topological entities belonging to a body.
    ///
    /// NOTE: `PK_BODY_topology_t` is a convenience struct and is NOT the ABI type.
    /// The actual ABI returns a flat `PK_TOPOL_t` array via `topols`/`n_topols`.
    pub fn PK_BODY_ask_topology(
        body: PK_BODY_t,
        options: *const PK_BODY_ask_topology_o_t,
        n_topols: *mut c_int,
        topols: *mut *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_BODY_find_laminar_edges(
        body: PK_BODY_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Primitive body creation
    // =========================================================================

    // =========================================================================
    // Class queries
    // =========================================================================

    // =========================================================================
    // Curve connectivity
    // =========================================================================

    pub fn PK_CURVE_ask_edges(
        curve: PK_CURVE_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_ask_fin(
        curve: PK_CURVE_t,
        fin: *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_ask_part(
        curve: PK_CURVE_t,
        part: *mut PK_PART_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_find_surfs_common(
        curve: PK_CURVE_t,
        n_surfs: *mut c_int,
        surfs: *mut *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Edge connectivity
    // =========================================================================

    // =========================================================================
    // Entity queries
    // =========================================================================

    pub fn PK_ENTITY_ask_identifier(
        entity: PK_ENTITY_t,
        identifier: *mut c_int,
    ) -> PK_ERROR_code_t;

    pub fn PK_ENTITY_ask_owning_groups_2(
        entity: PK_ENTITY_t,
        n_groups: *mut c_int,
        groups: *mut *mut PK_GROUP_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_ENTITY_ask_partition(
        entity: PK_ENTITY_t,
        partition: *mut PK_PARTITION_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Face connectivity
    // =========================================================================

    pub fn PK_FACE_find_interior_vec(
        face: PK_FACE_t,
        position: *mut PK_VECTOR_t,
        uv: *mut PK_UV_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Fin connectivity
    // =========================================================================

    // =========================================================================
    // Group / Loop connectivity
    // =========================================================================

    pub fn PK_GROUP_ask_part(group: PK_GROUP_t, part: *mut PK_PART_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Part / Partition / Session queries
    // =========================================================================

    pub fn PK_PART_ask_all_attdefs(
        part: PK_PART_t,
        n_attdefs: *mut c_int,
        attdefs: *mut *mut PK_ATTDEF_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_PART_ask_construction_curves(
        part: PK_PART_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_PART_ask_construction_points(
        part: PK_PART_t,
        n_points: *mut c_int,
        points: *mut *mut PK_POINT_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_PART_ask_construction_surfs(
        part: PK_PART_t,
        n_surfs: *mut c_int,
        surfs: *mut *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_PART_ask_groups_2(
        part: PK_PART_t,
        n_groups: *mut c_int,
        groups: *mut *mut PK_GROUP_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_POINT_ask_part(point: PK_POINT_t, part: *mut PK_PART_t) -> PK_ERROR_code_t;

    pub fn PK_POINT_ask_vertex(
        point: PK_POINT_t,
        vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Shell / Surface / Vertex connectivity
    // =========================================================================

    pub fn PK_SURF_ask_faces(
        surf: PK_SURF_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_ask_part(surf: PK_SURF_t, part: *mut PK_PART_t) -> PK_ERROR_code_t;

    pub fn PK_SURF_find_curves_common(
        surf1: PK_SURF_t,
        surf2: PK_SURF_t,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Parametric evaluation
    // =========================================================================

    // -- Handed evaluation (at discontinuities / periodic seams) --

    /// Signature verified against Parasolid V35 docs (n_derivs precedes hand).
    pub fn PK_CURVE_eval_handed(
        curve: PK_CURVE_t,
        t: c_double,
        n_derivs: c_int,
        hand_direction: PK_HAND_t,
        p_and_derivs: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_eval_curvature_handed(
        curve: PK_CURVE_t,
        t: c_double,
        hand: PK_HAND_t,
        position: *mut PK_VECTOR_t,
        tangent: *mut PK_VECTOR_t,
        normal: *mut PK_VECTOR_t,
        curvature: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_eval_with_tan_handed(
        curve: PK_CURVE_t,
        t: c_double,
        hand: PK_HAND_t,
        position: *mut PK_VECTOR_t,
        tangent: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_eval_handed(
        surf: PK_SURF_t,
        u: c_double,
        v: c_double,
        u_hand: PK_HAND_t,
        v_hand: PK_HAND_t,
        n_u_deriv: c_int,
        n_v_deriv: c_int,
        p_and_derivs: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_eval_curvature_handed(
        surf: PK_SURF_t,
        u: c_double,
        v: c_double,
        u_hand: PK_HAND_t,
        v_hand: PK_HAND_t,
        position: *mut PK_VECTOR_t,
        normal: *mut PK_VECTOR_t,
        d1: *mut PK_VECTOR_t,
        d2: *mut PK_VECTOR_t,
        k1: *mut c_double,
        k2: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_eval_with_normal_handed(
        surf: PK_SURF_t,
        u: c_double,
        v: c_double,
        u_hand: PK_HAND_t,
        v_hand: PK_HAND_t,
        position: *mut PK_VECTOR_t,
        normal: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Geometric properties
    // =========================================================================

    pub fn PK_CURVE_find_vector_interval(
        curve: PK_CURVE_t,
        vec1: *const c_double,
        vec2: *const c_double,
        interval: *mut PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_find_min_radius(
        curve: PK_CURVE_t,
        options: *const PK_CURVE_find_min_radius_o_t,
        radius: *mut c_double,
        t: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_find_min_radii(
        surf: PK_SURF_t,
        options: *const PK_SURF_find_min_radii_o_t,
        min_radius: *mut c_double,
        u: *mut c_double,
        v: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_EDGE_is_planar(
        edge: PK_EDGE_t,
        is_planar: *mut PK_LOGICAL_t,
        normal: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_EDGE_is_smooth(
        edge: PK_EDGE_t,
        is_smooth: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_GEOM_is_coincident(
        geom1: PK_GEOM_t,
        geom2: PK_GEOM_t,
        options: *const PK_GEOM_is_coincident_o_t,
        is_coincident: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_FIN_find_curve_parameter(
        fin: PK_FIN_t,
        u: c_double,
        v: c_double,
        t: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_FIN_find_surf_parameters(
        fin: PK_FIN_t,
        t: c_double,
        u: *mut c_double,
        v: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_FACE_is_uvbox(
        face: PK_FACE_t,
        is_uvbox: *mut PK_LOGICAL_t,
        uvbox: *mut PK_UVBOX_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_FACE_is_periodic(
        face: PK_FACE_t,
        u_periodic: *mut PK_LOGICAL_t,
        v_periodic: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Bounding boxes
    // =========================================================================

    pub fn PK_TOPOL_find_box(
        topol: PK_TOPOL_t,
        options: *const PK_TOPOL_find_box_o_t,
        box_: *mut PK_BOX_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_TOPOL_find_nabox(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        options: *const PK_TOPOL_find_nabox_o_t,
        nabox: *mut PK_NABOX_sf_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_find_non_aligned_box(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        nabox: *mut PK_NABOX_sf_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_find_non_aligned_box(
        surf: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        nabox: *mut PK_NABOX_sf_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_CURVE_find_box(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        box_: *mut PK_BOX_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_find_box(
        surf: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        box_: *mut PK_BOX_t,
    ) -> PK_ERROR_code_t;

    // -- Parameter boxes --

    pub fn PK_FACE_find_uvbox(
        face: PK_FACE_t,
        uvbox: *mut PK_UVBOX_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_FIN_find_uvbox(
        fin: PK_FIN_t,
        uvbox: *mut PK_UVBOX_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Intervals and extremes
    // =========================================================================

    pub fn PK_EDGE_find_interval(
        edge: PK_EDGE_t,
        interval: *mut PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_FIN_find_interval(
        fin: PK_FIN_t,
        interval: *mut PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_EDGE_find_extreme(
        edge: PK_EDGE_t,
        direction: *const c_double,
        options: *const PK_EDGE_find_extreme_o_t,
        position: *mut c_double,
        t: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_FACE_find_extreme(
        face: PK_FACE_t,
        direction: *const c_double,
        options: *const PK_FACE_find_extreme_o_t,
        position: *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_BODY_find_extreme(
        body: PK_BODY_t,
        n_directions: c_int,
        directions: *const PK_VECTOR_t,
        options: *const PK_BODY_find_extreme_o_t,
        extremes: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Spatial containment
    // =========================================================================

    pub fn PK_BODY_contains_vector(
        body: PK_BODY_t,
        position: *const c_double,
        options: *const PK_BODY_contains_vector_o_t,
        containment: *mut PK_CONTAINMENT_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_FACE_contains_vectors(
        face: PK_FACE_t,
        n_vectors: c_int,
        vectors: *const PK_VECTOR_t,
        options: *const PK_FACE_contains_vectors_o_t,
        containments: *mut PK_CONTAINMENT_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_FACE_set_approx(face: PK_FACE_t) -> PK_ERROR_code_t;

    pub fn PK_FACE_unset_approx(face: PK_FACE_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Vector comparison
    // =========================================================================

    pub fn PK_VECTOR_is_equal(
        vec1: *const c_double,
        vec2: *const c_double,
        is_equal: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_VECTOR_is_zero(
        vec: *const c_double,
        is_zero: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_VECTOR_is_parallel(
        vec1: *const c_double,
        vec2: *const c_double,
        is_parallel: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_VECTOR_normalise(
        vec: *const c_double,
        unit: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Transformation comparison
    // =========================================================================

    // =========================================================================
    // Face coincidence
    // =========================================================================

    pub fn PK_FACE_is_coincident(
        face1: PK_FACE_t,
        face2: PK_FACE_t,
        options: *const PK_FACE_is_coincident_o_t,
        result: *mut PK_FACE_coi_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Edge convexity
    // =========================================================================

    pub fn PK_EDGE_ask_convexity(
        edge: PK_EDGE_t,
        convexity: *mut PK_EDGE_convexity_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Discontinuities
    // =========================================================================

    pub fn PK_CURVE_find_discontinuity(
        curve: PK_CURVE_t,
        options: *const PK_CURVE_find_discontinuity_o_t,
        n_params: *mut c_int,
        params: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_find_discontinuity(
        surf: PK_SURF_t,
        options: *const PK_SURF_find_discontinuity_o_t,
        n_u_params: *mut c_int,
        u_params: *mut *mut c_double,
        n_v_params: *mut c_int,
        v_params: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Self-intersection
    // =========================================================================

    // =========================================================================
    // Degeneracies
    // =========================================================================

    pub fn PK_CURVE_find_degens(
        curve: PK_CURVE_t,
        options: *const PK_CURVE_find_degens_o_t,
        n_degens: *mut c_int,
        degen_params: *mut *mut c_double,
        degen_types: *mut *mut PK_CURVE_degen_type_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // General body analysis
    // =========================================================================

    // =========================================================================
    // Position vectors along curves / surfaces
    // =========================================================================

    pub fn PK_CURVE_find_vectors(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        n_vectors: c_int,
        options: *const PK_CURVE_find_vectors_o_t,
        vectors: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    pub fn PK_SURF_find_vectors(
        surf: PK_SURF_t,
        start_uv: *const c_double,
        end_uv: *const c_double,
        n_vectors: c_int,
        options: *const PK_SURF_find_vectors_o_t,
        vectors: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Edge operations
    // =========================================================================

    /// Reverse edge and its geometry.
    pub fn PK_EDGE_reverse(edge: PK_EDGE_t) -> PK_ERROR_code_t;

    /// Compute distances between two edges (deprecated by _2).
    pub fn PK_EDGE_find_deviation(
        edge1: PK_EDGE_t,
        edge2: PK_EDGE_t,
        how_many: c_int,
        n_distances: *mut c_int,
        distances: *mut *mut c_double,
        edge1_vecs: *mut *mut PK_VECTOR_t,
        edge2_vecs: *mut *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Test if position coincides with edge.
    pub fn PK_EDGE_contains_vector(
        edge: PK_EDGE_t,
        vector: PK_VECTOR_t,
        topol: *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Find end positions and tangent directions of edge.
    pub fn PK_EDGE_find_end_tangents(
        edge: PK_EDGE_t,
        start: *mut PK_VECTOR_t,
        start_tangent: *mut PK_VECTOR_t,
        end: *mut PK_VECTOR_t,
        end_tangent: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Entity operations
    // =========================================================================

    /// Copy entity with tracking.
    pub fn PK_ENTITY_copy_2(
        entity: PK_ENTITY_t,
        options: *const PK_ENTITY_copy_o_t,
        entity_copy: *mut PK_ENTITY_t,
        tracking: *mut PK_ENTITY_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Return textual description of entity internals.
    pub fn PK_ENTITY_ask_description(
        tag: c_int,
        options: *const PK_ENTITY_ask_description_o_t,
        description: *mut *mut std::os::raw::c_char,
    ) -> PK_ERROR_code_t;

    /// Min/max separation between two entity arrays.
    pub fn PK_ENTITY_range(
        n_entities_1: c_int,
        entities_1: *const PK_ENTITY_t,
        tf_1: *const PK_TRANSF_t,
        n_entities_2: c_int,
        entities_2: *const PK_ENTITY_t,
        tf_2: *const PK_TRANSF_t,
        options: *const PK_ENTITY_range_o_t,
        results: *mut PK_ENTITY_range_r_t,
    ) -> PK_ERROR_code_t;

    /// Min separation between entities and positions.
    pub fn PK_ENTITY_range_vector(
        n_entities: c_int,
        entities: *const PK_ENTITY_t,
        tf: *const PK_TRANSF_t,
        n_vectors: c_int,
        vectors: *const PK_VECTOR_t,
        options: *const PK_ENTITY_range_vector_o_t,
        results: *mut PK_ENTITY_range_vector_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Curve operations
    // =========================================================================

    /// Arc length over parametric interval.
    pub fn PK_CURVE_find_length(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        length: *mut c_double,
        range: *mut PK_INTERVAL_t,
    ) -> PK_ERROR_code_t;

    /// Check if PK vs KI parametrisation differs.
    pub fn PK_CURVE_ask_parm_different(
        curve: PK_CURVE_t,
        different: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Embed curve in surface parameter space.
    pub fn PK_CURVE_embed_in_surf(
        curve: PK_CURVE_t,
        surf: PK_SURF_t,
        n_spcurves: *mut c_int,
        spcurves: *mut *mut PK_SPCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create spcurve representation on surface.
    pub fn PK_CURVE_make_spcurves(
        curve: PK_CURVE_t,
        range: PK_INTERVAL_t,
        surf: PK_SURF_t,
        degenerate: PK_LOGICAL_t,
        sense: PK_LOGICAL_t,
        tolerance: c_double,
        n_spcurves: *mut c_int,
        spcurves: *mut *mut PK_SPCURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create wire body from curve (obsolete, superseded by _2).
    pub fn PK_CURVE_make_wire_body(
        curve: PK_CURVE_t,
        range: PK_INTERVAL_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create reversed copy of curve.
    pub fn PK_CURVE_make_curve_reversed(
        curve: PK_CURVE_t,
        reverse: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Convert PK parameter to KI parameter.
    pub fn PK_CURVE_convert_parm_to_ki(
        curve: PK_CURVE_t,
        pk_t: c_double,
        ki_t: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Convert KI parameter to PK parameter.
    pub fn PK_CURVE_convert_parm_to_pk(
        curve: PK_CURVE_t,
        ki_t: c_double,
        pk_t: *mut c_double,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Surface operations
    // =========================================================================

    /// Create offset surface.
    pub fn PK_SURF_offset(
        underlying_surf: PK_SURF_t,
        offset_distance: c_double,
        surf: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Evaluate surface points on rectangular parameter grid.
    pub fn PK_SURF_eval_grid(
        surf: PK_SURF_t,
        n_u: c_int,
        u: *const c_double,
        n_v: c_int,
        v: *const c_double,
        n_u_derivs: c_int,
        n_v_derivs: c_int,
        triangular: PK_LOGICAL_t,
        p: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Create isocline curves on surface (obsolete).
    pub fn PK_SURF_make_curves_isocline(
        surf: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        direction: PK_VECTOR1_t,
        angle: c_double,
        tolerance: c_double,
        n_curves: *mut c_int,
        curves: *mut *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create curve at constant u parameter.
    pub fn PK_SURF_make_curve_u_isoparam(
        surf: PK_SURF_t,
        param: c_double,
        curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Create curve at constant v parameter.
    pub fn PK_SURF_make_curve_v_isoparam(
        surf: PK_SURF_t,
        param: c_double,
        curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Topology operations
    // =========================================================================

    /// Axis-aligned bounding box with per-topology boxes.
    pub fn PK_TOPOL_find_box_2(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        transfs: *const PK_TRANSF_t,
        options: *const PK_TOPOL_find_box_2_o_t,
        results: *mut PK_TOPOL_find_box_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Find all connected topologies.
    pub fn PK_TOPOL_find_connected(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        options: *const PK_TOPOL_find_connected_o_t,
        results: *mut PK_TOPOL_find_connected_r_t,
    ) -> PK_ERROR_code_t;

    /// Test mutual connectivity of topologies.
    pub fn PK_TOPOL_is_connected(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        options: *const PK_TOPOL_is_connected_o_t,
        results: *mut PK_TOPOL_is_connected_r_t,
    ) -> PK_ERROR_code_t;

    /// Replace topology with new tag (strips attributes/groups).
    pub fn PK_TOPOL_make_new(
        topol: PK_TOPOL_t,
        options: *const PK_TOPOL_make_new_o_t,
        new_topol: *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Filter entities by class and attribute ownership.
    pub fn PK_TOPOL_ask_entities_by_attdef(
        topol: PK_TOPOL_t,
        class: PK_CLASS_t,
        have_attrib: PK_LOGICAL_t,
        attdef: PK_ATTDEF_t,
        n_entities: *mut c_int,
        entities: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Find frames on a topology entity.
    pub fn PK_TOPOL_find_frames(
        topol: PK_TOPOL_t,
        n_frames: *mut c_int,
        frames: *mut *mut PK_FRAME_t,
    ) -> PK_ERROR_code_t;

    /// Imprint frames onto topology entities.
    pub fn PK_TOPOL_imprint_frames(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        options: *mut c_int,
        results: *mut c_int,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // SP-curve operations
    // =========================================================================

    /// Approximate evaluation of surface-parametric curve.
    pub fn PK_SPCURVE_eval_approx(
        spcurve: PK_SPCURVE_t,
        t: c_double,
        n_derivs: c_int,
        p_derivs: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Query polyline standard form.
    pub fn PK_PLINE_ask(
        pline: PK_PLINE_t,
        pline_sf: *mut PK_PLINE_sf_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Loop operations
    // =========================================================================

    /// Offset planar loop by distance.
    pub fn PK_LOOP_offset_planar(
        loop_: PK_LOOP_t,
        distance: c_double,
        options: *const PK_LOOP_offset_planar_o_t,
        results: *mut PK_LOOP_offset_planar_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_TOPOL_find_box_2`.
    pub fn PK_TOPOL_find_box_2_r_f(results: *mut PK_TOPOL_find_box_2_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_TOPOL_find_connected`.
    pub fn PK_TOPOL_find_connected_r_f(results: *mut PK_TOPOL_find_connected_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_TOPOL_is_connected`.
    pub fn PK_TOPOL_is_connected_r_f(results: *mut PK_TOPOL_is_connected_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_TOPOL_make_new`.
    pub fn PK_TOPOL_make_new_r_f(results: *mut c_int) -> PK_ERROR_code_t;

    /// Free topology tracking results.
    pub fn PK_TOPOL_track_r_f(results: *mut PK_TOPOL_track_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_ENTITY_range`.
    pub fn PK_ENTITY_range_r_f(results: *mut PK_ENTITY_range_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_ENTITY_range_vector`.
    pub fn PK_ENTITY_range_vector_r_f(results: *mut PK_ENTITY_range_vector_r_t) -> PK_ERROR_code_t;

    /// Free entity description string.
    pub fn PK_ENTITY_ask_description_r_f(description: *mut *mut std::os::raw::c_char) -> PK_ERROR_code_t;

    /// Free results from `PK_LOOP_offset_planar`.
    pub fn PK_LOOP_offset_planar_r_f(results: *mut PK_LOOP_offset_planar_r_t) -> PK_ERROR_code_t;

}
