//! Rigid-body and scaling transforms.
//!
//! A [`Transform`] wraps a Parasolid `PK_TRANSF_t` (a persistent transform
//! entity created from a 4x4 standard-form matrix). Apply one to a body with
//! [`Body::transform`], which moves the body's geometry in place.
//!
//! The standard-form matrix (`PK_TRANSF_sf_t`) is a full 4x4 stored row-major;
//! Parasolid transforms a point as `M · [x y z 1]^T`, so the translation lives
//! in the 4th column and `matrix[3][3]` is the reciprocal global scale.

use parasolid_sys::*;

use crate::body::Body;
use crate::error::PsResult;
use crate::geom::Vec3;
use crate::memory::PkArray;

/// A Parasolid transform entity (`PK_TRANSF_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Transform {
    tag: PK_TRANSF_t,
}

impl Transform {
    /// Wrap a raw PK transform tag.
    pub(crate) fn from_tag(tag: PK_TRANSF_t) -> Self {
        Transform { tag }
    }

    /// Returns the raw PK tag.
    #[inline]
    pub fn tag(&self) -> i32 {
        self.tag
    }

    /// Build a transform from a row-major 4x4 matrix (16 elements).
    pub fn from_matrix(matrix: [f64; 16]) -> PsResult<Transform> {
        let sf = PK_TRANSF_sf_t { matrix };
        let mut tag: PK_TRANSF_t = PK_ENTITY_null;
        pk_call!(PK_TRANSF_create(&sf, &mut tag));
        Ok(Transform::from_tag(tag))
    }

    /// A pure translation by `(dx, dy, dz)`.
    pub fn translation(dx: f64, dy: f64, dz: f64) -> PsResult<Transform> {
        #[rustfmt::skip]
        let m = [
            1.0, 0.0, 0.0, dx,
            0.0, 1.0, 0.0, dy,
            0.0, 0.0, 1.0, dz,
            0.0, 0.0, 0.0, 1.0,
        ];
        Transform::from_matrix(m)
    }

    /// A uniform scaling about the origin by `factor`.
    ///
    /// Encoded via the reciprocal-scale element `matrix[3][3] = 1/factor`
    /// rather than scaling the diagonal, matching Parasolid's convention.
    pub fn uniform_scale(factor: f64) -> PsResult<Transform> {
        #[rustfmt::skip]
        let m = [
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0 / factor,
        ];
        Transform::from_matrix(m)
    }

    /// Read back this transform's standard-form 4x4 matrix (row-major).
    pub fn matrix(&self) -> PsResult<[f64; 16]> {
        let mut sf = PK_TRANSF_sf_t { matrix: [0.0; 16] };
        pk_call!(PK_TRANSF_ask(self.tag, &mut sf));
        Ok(sf.matrix)
    }

    /// A rotation by `angle` radians about the axis through `point` with
    /// direction `axis` (right-hand rule). Built natively via
    /// `PK_TRANSF_create_rotation` rather than a hand-rolled matrix.
    pub fn rotation(point: Vec3, axis: Vec3, angle: f64) -> PsResult<Transform> {
        let p = point.to_pk();
        let a = axis.to_pk();
        let mut tag: PK_TRANSF_t = PK_ENTITY_null;
        pk_call!(PK_TRANSF_create_rotation(&p, &a, angle, &mut tag));
        Ok(Transform::from_tag(tag))
    }

    /// A reflection in the plane through `point` with unit normal `normal`.
    pub fn reflection(point: Vec3, normal: Vec3) -> PsResult<Transform> {
        let p = point.to_pk();
        let n = normal.to_pk();
        let mut tag: PK_TRANSF_t = PK_ENTITY_null;
        pk_call!(PK_TRANSF_create_reflection(&p, &n, &mut tag));
        Ok(Transform::from_tag(tag))
    }

    /// A uniform scale by `factor` about `centre` (`PK_TRANSF_create_equal_scale`).
    pub fn scale_about(factor: f64, centre: Vec3) -> PsResult<Transform> {
        let c = centre.to_pk();
        let mut tag: PK_TRANSF_t = PK_ENTITY_null;
        pk_call!(PK_TRANSF_create_equal_scale(factor, &c, &mut tag));
        Ok(Transform::from_tag(tag))
    }

    /// Compose two transforms: the result applies `self` first, then `other`
    /// (`PK_TRANSF_transform`).
    pub fn then(&self, other: &Transform) -> PsResult<Transform> {
        let mut tag: PK_TRANSF_t = PK_ENTITY_null;
        pk_call!(PK_TRANSF_transform(self.tag, other.tag, &mut tag));
        Ok(Transform::from_tag(tag))
    }

    /// Whether two transforms are numerically equal.
    pub fn is_equal(&self, other: &Transform) -> PsResult<bool> {
        let mut eq: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_TRANSF_is_equal(self.tag, other.tag, &mut eq));
        Ok(eq == PK_LOGICAL_true)
    }

    /// Apply this transform to a **position** vector.
    pub fn apply(&self, point: Vec3) -> PsResult<Vec3> {
        let v = point.to_pk();
        let mut out = PK_VECTOR_t::default();
        pk_call!(PK_VECTOR_transform(&v, self.tag, &mut out));
        Ok(Vec3::from_pk(out))
    }

    /// Apply this transform to a **direction** vector (ignores the translation
    /// component).
    pub fn apply_direction(&self, dir: Vec3) -> PsResult<Vec3> {
        let v = dir.to_pk();
        let mut out = PK_VECTOR_t::default();
        pk_call!(PK_VECTOR_transform_direction(&v, self.tag, &mut out));
        Ok(Vec3::from_pk(out))
    }
}

impl Body {
    /// Transform this body in place by the given [`Transform`].
    ///
    /// Rigid motions and uniform scales are applied exactly. `PK_BODY_transform`
    /// writes its `n_replaces`/`replaces`/`exact` outputs unconditionally, so we
    /// pass real buffers and release the kernel-allocated arrays.
    pub fn transform(&self, transform: &Transform) -> PsResult<()> {
        let mut n_replaces: std::os::raw::c_int = 0;
        let mut replaces: *mut PK_GEOM_t = std::ptr::null_mut();
        let mut exact: *mut PK_LOGICAL_t = std::ptr::null_mut();
        pk_call!(PK_BODY_transform(
            self.tag,
            transform.tag(),
            1.0e-8,
            &mut n_replaces,
            &mut replaces,
            &mut exact,
        ));
        // Release any geometry-replacement tracking the kernel allocated.
        unsafe {
            let _ = PkArray::from_raw(replaces, n_replaces);
            let _ = PkArray::from_raw(exact, n_replaces);
        }
        Ok(())
    }
}
