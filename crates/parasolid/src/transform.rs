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
