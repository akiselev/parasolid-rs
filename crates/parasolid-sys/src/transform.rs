#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! FFI bindings for Parasolid transformation functions (`PK_TRANSF_*`).
//!
//! Covers creation, composition, classification, and application of 4x4
//! homogeneous coordinate transformations.

use std::os::raw::{c_double, c_int};

use crate::*;

// =============================================================================
// Standard form: 4x4 homogeneous transformation matrix
// =============================================================================

/// Standard form of a 4x4 homogeneous transformation.
///
/// Layout (row-major):
/// ```text
///   [ m[0]  m[1]  m[2]  0 ]
///   [ m[3]  m[4]  m[5]  0 ]
///   [ m[6]  m[7]  m[8]  0 ]
///   [ m[9]  m[10] m[11] m[12] ]
/// ```
///
/// - Upper-left 3x3 (`m[0..9]`): linear part (rotation, scale, reflection, shear).
/// - `m[9..12]`: translation vector.
/// - `m[12]`: reciprocal of the global scale factor.
/// - Elements at indices 3, 7, 11 in the 4th column are implicitly zero (no perspective).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TRANSF_sf_t {
    pub matrix: [c_double; 13],
}

// =============================================================================
// PK_matrix_type_t — classification of the linear 3x3 sub-matrix
// =============================================================================

pub type PK_matrix_type_t = c_int;

/// Linear transformation is the identity.
pub const PK_matrix_type_identity_c: PK_matrix_type_t = 0;

/// Linear transformation is a pure rotation.
pub const PK_matrix_type_rotation_c: PK_matrix_type_t = 1;

/// Orthonormal matrix with reflection (det = -1).
pub const PK_matrix_type_reflection_c: PK_matrix_type_t = 2;

/// Valid but contains local (non-uniform) scaling.
pub const PK_matrix_type_general_c: PK_matrix_type_t = 3;

/// Cannot be classified.
pub const PK_matrix_type_unclassified_c: PK_matrix_type_t = 4;

// =============================================================================
// PK_TRANSF_diagnostics_t — controls optional output from PK_TRANSF_classify
// =============================================================================

pub type PK_TRANSF_diagnostics_t = c_int;

/// Return unit_rows_deviations and orthog_rows_deviations.
pub const PK_TRANSF_diagnostics_all_c: PK_TRANSF_diagnostics_t = 0;

// =============================================================================
// Extern declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // ---- Creation ----------------------------------------------------------

    /// Create a transformation from a 4x4 matrix in standard form.
    pub fn PK_TRANSF_create(
        sf: *const PK_TRANSF_sf_t,
        transf: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Create a uniform (equal) scale transformation.
    pub fn PK_TRANSF_create_equal_scale(
        scale: c_double,
        transf: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Create a reflection transformation about a plane defined by
    /// a point and a normal direction.
    pub fn PK_TRANSF_create_reflection(
        point: *const PK_VECTOR_t,
        normal: *const PK_VECTOR_t,
        transf: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Create a rotation transformation about an axis (point + direction)
    /// by the given angle in radians.
    pub fn PK_TRANSF_create_rotation(
        point: *const PK_VECTOR_t,
        direction: *const PK_VECTOR_t,
        angle: c_double,
        transf: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Create a translation transformation.
    pub fn PK_TRANSF_create_translation(
        translation: *const PK_VECTOR_t,
        transf: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Create a view transformation.
    pub fn PK_TRANSF_create_view(
        eye: *const PK_VECTOR_t,
        target: *const PK_VECTOR_t,
        up: *const PK_VECTOR_t,
        transf: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    // ---- Query -------------------------------------------------------------

    /// Retrieve the standard form of an existing transformation entity.
    pub fn PK_TRANSF_ask(
        transf: PK_TRANSF_t,
        sf: *mut PK_TRANSF_sf_t,
    ) -> PK_ERROR_code_t;

    // ---- Composition -------------------------------------------------------

    /// Compose (concatenate) transformation `tool` onto `target`.
    pub fn PK_TRANSF_transform(
        target: PK_TRANSF_t,
        tool: PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Compose transformations (version 2, with results).
    pub fn PK_TRANSF_transform_2(
        target: PK_TRANSF_t,
        tool: PK_TRANSF_t,
        result: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Scale (enlarge) a transformation by a factor.
    pub fn PK_TRANSF_enlarge(
        transf: PK_TRANSF_t,
        factor: c_double,
    ) -> PK_ERROR_code_t;

    // ---- Checking & Classification -----------------------------------------

    /// Simple validity check on a transformation.
    pub fn PK_TRANSF_check(
        transf: PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Comprehensive classification of a transformation.
    ///
    /// Returns matrix type, determinant, translation, perspective, scale, and
    /// optionally row deviation diagnostics.
    pub fn PK_TRANSF_classify(
        transf: PK_TRANSF_t,
        diagnostics: PK_TRANSF_diagnostics_t,
        matrix_type: *mut PK_matrix_type_t,
        determinant: *mut c_double,
        unit_rows_deviations: *mut [c_double; 3],
        orthog_rows_deviations: *mut [c_double; 3],
        translation: *mut PK_VECTOR_t,
        perspective: *mut PK_VECTOR_t,
        scale: *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Test whether two transformations are equal.
    pub fn PK_TRANSF_is_equal(
        transf1: PK_TRANSF_t,
        transf2: PK_TRANSF_t,
        is_equal: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // ---- Application to entities -------------------------------------------

    /// Transform an array of geometric entities (points, curves, surfaces).
    pub fn PK_GEOM_transform_2(
        n_geoms: c_int,
        geoms: *const PK_GEOM_t,
        transf: PK_TRANSF_t,
        tolerance: c_double,
        n_out_geoms: *mut c_int,
        out_geoms: *mut *mut PK_GEOM_t,
    ) -> PK_ERROR_code_t;

    /// Transform a solid or sheet body.
    pub fn PK_BODY_transform_2(
        body: PK_BODY_t,
        transf: PK_TRANSF_t,
        tolerance: c_double,
    ) -> PK_ERROR_code_t;

    /// Transform a face.
    pub fn PK_FACE_transform_2(
        face: PK_FACE_t,
        transf: PK_TRANSF_t,
        tolerance: c_double,
    ) -> PK_ERROR_code_t;

    /// Transform a position vector.
    pub fn PK_VECTOR_transform(
        vec: *const PK_VECTOR_t,
        transf: PK_TRANSF_t,
        result: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    /// Transform a direction vector (ignores translation component).
    pub fn PK_VECTOR_transform_direction(
        vec: *const PK_VECTOR_t,
        transf: PK_TRANSF_t,
        result: *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;
}
