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
/// Full 4x4 homogeneous matrix, stored ROW-MAJOR as 16 sequential doubles
/// (`matrix[i][j]` at flat index `i*4 + j`). Confirmed against the authoritative
/// C# binding (`ch122-how-the-c-binding-is-implemented.md`): 16 fields
/// `matrixI0J0 … matrixI3J3`. The previous 13-element "compressed" form was wrong
/// (104 vs 128 bytes) — `PK_TRANSF_create`/`PK_TRANSF_ask` read/write all 16.
///
/// Parasolid convention (point transformed as `M · [x y z 1]^T`):
/// - Upper-left 3x3 (`[0][0..3] [1][0..3] [2][0..3]`): rotation/scale/shear.
/// - 4th column `[i][3]` for `i=0,1,2`: translation vector.
/// - `[3][3]`: reciprocal of the global scale factor (1.0 for unit scale).
/// - `[3][0..3]` (except `[3][3]`): zero (no perspective).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TRANSF_sf_t {
    pub matrix: [c_double; 16],
}

// =============================================================================
// PK_matrix_type_t — classification of the linear 3x3 sub-matrix
// =============================================================================

pub type PK_matrix_type_t = c_int;

/// Linear transformation is the identity.
pub const PK_matrix_type_identity_c: PK_matrix_type_t = 25290;

/// Linear transformation is a pure rotation.
pub const PK_matrix_type_rotation_c: PK_matrix_type_t = 25291;

/// Orthonormal matrix with reflection (det = -1).
pub const PK_matrix_type_reflection_c: PK_matrix_type_t = 25292;

/// Valid but contains local (non-uniform) scaling.
pub const PK_matrix_type_general_c: PK_matrix_type_t = 25293;

/// Cannot be classified.
pub const PK_matrix_type_unclassified_c: PK_matrix_type_t = 25294;

// =============================================================================
// PK_TRANSF_diagnostics_t — controls optional output from PK_TRANSF_classify
// =============================================================================

pub type PK_TRANSF_diagnostics_t = c_int;

/// Return unit_rows_deviations and orthog_rows_deviations.
pub const PK_TRANSF_diagnostics_all_c: PK_TRANSF_diagnostics_t = 25301;

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
        centre: *const PK_VECTOR_t,
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
        view_direction: *const PK_VECTOR1_t,
        options: *mut PK_TRANSF_create_view_o_t,
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
        transf_1: PK_TRANSF_t,
        transf_2: PK_TRANSF_t,
        transf_out: *mut PK_TRANSF_t,
    ) -> PK_ERROR_code_t;

    /// Compose transformations (version 2, with results).
    pub fn PK_TRANSF_transform_2(
        transf_1: PK_TRANSF_t,
        transf_2: PK_TRANSF_t,
        options: *mut PK_TRANSF_transform_o_t,
        results: *mut PK_TRANSF_transform_r_t,
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
        options: *mut PK_TRANSF_check_o_t,
        n_faults: *mut c_int,
        faults: *mut *mut PK_check_fault_t,
    ) -> PK_ERROR_code_t;

    /// Comprehensive classification of a transformation.
    ///
    /// Returns matrix type, determinant, translation, perspective, scale, and
    /// optionally row deviation diagnostics.
    pub fn PK_TRANSF_classify(
        transf: PK_TRANSF_t,
        options: *mut PK_TRANSF_classify_o_t,
        results: *mut PK_TRANSF_classify_r_t,
    ) -> PK_ERROR_code_t;

    /// Test whether two transformations are equal.
    pub fn PK_TRANSF_is_equal(
        transf1: PK_TRANSF_t,
        transf2: PK_TRANSF_t,
        is_equal: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // ---- Application to entities -------------------------------------------

    /// Transform an array of geometric entities (points, curves, surfaces).
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_GEOM_transform_2(
        n_geoms: c_int,
        in_geoms: *mut PK_GEOM_t,
        transf: PK_TRANSF_t,
        options: *mut PK_GEOM_transform_o_t,
        out_geoms: *mut PK_GEOM_t,
        exact: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Transform a solid or sheet body.
    pub fn PK_BODY_transform_2(
        body: PK_BODY_t,
        transf: PK_TRANSF_t,
        tolerance: c_double,
        options: *mut PK_BODY_transform_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    /// Transform a face.
    pub fn PK_FACE_transform_2(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        transfs: *mut PK_TRANSF_t,
        tolerance: c_double,
        options: *mut PK_FACE_transform_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
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
