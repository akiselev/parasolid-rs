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

impl Transform {
    /// Apply this transform to orphan geometry (`PK_GEOM_transform_2`).
    ///
    /// Returns, per input geom, the resulting geom tag and whether the kernel
    /// achieved the transform **exactly**. That second flag is the point of
    /// this call: a rotation of an analytic surface stays analytic and exact,
    /// but a non-uniform scale generally cannot, and the kernel reports which
    /// happened instead of silently degrading the representation.
    ///
    /// Options are passed as NULL so the kernel applies its own defaults; the
    /// `modify` field's tokens are not yet probed.
    pub fn apply_to_geoms(&self, geoms: &[i32]) -> PsResult<Vec<(i32, bool)>> {
        if geoms.is_empty() {
            return Ok(Vec::new());
        }
        let mut out_geoms = vec![0i32; geoms.len()];
        let mut exact = vec![PK_LOGICAL_false; geoms.len()];
        pk_call!(PK_GEOM_transform_2(
            geoms.len() as std::os::raw::c_int,
            geoms.as_ptr(),
            self.tag,
            std::ptr::null(),
            out_geoms.as_mut_ptr(),
            exact.as_mut_ptr(),
        ));
        Ok(out_geoms
            .into_iter()
            .zip(exact)
            .map(|(g, e)| (g, e == PK_LOGICAL_true))
            .collect())
    }
}

// =============================================================================
// Classification — the Stage 2 lattice
// =============================================================================

/// How a transform's linear 3×3 sub-matrix classifies (`PK_matrix_type_t`).
///
/// This is the lattice CADabra's frame types have to mirror: whether a
/// transform is a rigid motion, whether it flips handedness, and whether it is
/// a similarity at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatrixType {
    /// The identity.
    Identity,
    /// A pure rotation — orthonormal, determinant +1.
    Rotation,
    /// Orthonormal with a handedness flip — determinant −1.
    Reflection,
    /// Valid, but carries non-uniform (local) scaling or shear.
    General,
    /// The kernel could not classify it.
    Unclassified,
    /// A token outside the documented set.
    Other(i32),
}

impl MatrixType {
    fn from_token(t: PK_matrix_type_t) -> Self {
        match t {
            PK_matrix_type_identity_c => MatrixType::Identity,
            PK_matrix_type_rotation_c => MatrixType::Rotation,
            PK_matrix_type_reflection_c => MatrixType::Reflection,
            PK_matrix_type_general_c => MatrixType::General,
            PK_matrix_type_unclassified_c => MatrixType::Unclassified,
            other => MatrixType::Other(other),
        }
    }

    /// Whether the *linear part* is a distance-preserving, orientation-
    /// preserving map, according to the kernel's own token.
    ///
    /// A reflection is excluded: it preserves distances but reverses
    /// orientation, and that difference decides face senses and shell signs
    /// downstream, so callers must opt into it explicitly.
    ///
    /// # This is not a geometric rigid-motion test
    ///
    /// `matrix_type` is initialised to `unclassified` and only overwritten for
    /// cases the kernel recognises (confirmed by decompiling
    /// `PK_TRANSF_classify`, which stores `0x62ce` before classifying). A pure
    /// translation with unit global scale comes back **`Unclassified`** even
    /// though it is plainly a rigid motion — add any global scale factor and
    /// the identical translation classifies `Identity`. Pinned by
    /// `stage2_classify_lattice`.
    ///
    /// So: use this for what the kernel asserts, not as a predicate for "can I
    /// treat this as a rigid motion". For the latter, test the linear part.
    pub fn is_rigid_motion(&self) -> bool {
        matches!(self, MatrixType::Identity | MatrixType::Rotation)
    }

    /// Whether this transform reverses handedness.
    pub fn reverses_orientation(&self) -> bool {
        matches!(self, MatrixType::Reflection)
    }
}

/// Full classification of a transform (`PK_TRANSF_classify`).
#[derive(Debug, Clone, Copy)]
pub struct Classification {
    /// Classification of the linear 3×3 part.
    pub matrix_type: MatrixType,
    /// Determinant of the linear part. Negative means a handedness flip.
    pub determinant: f64,
    /// Uniform scale factor.
    pub scale: f64,
    /// Translation component.
    pub translation: Vec3,
    /// Perspective component — zero for every transform Parasolid will accept
    /// on a model.
    pub perspective: Vec3,
    /// Per-row deviation from unit length, or `None` unless diagnostics were
    /// requested.
    pub unit_rows_deviations: Option<Vec3>,
    /// Per-row-pair deviation from orthogonality, or `None` unless diagnostics
    /// were requested.
    pub orthog_rows_deviations: Option<Vec3>,
}

impl Transform {
    /// Classify this transform (`PK_TRANSF_classify`).
    ///
    /// With `diagnostics = true` the kernel also reports how far each matrix
    /// row is from unit length and from mutual orthogonality — the measurement
    /// behind its accept/reject decision, which is what tells us whether a
    /// nearly-orthonormal frame is being silently repaired.
    pub fn classify(&self, diagnostics: bool) -> PsResult<Classification> {
        let opts = PK_TRANSF_classify_o_t {
            o_t_version: 1,
            diagnostics: if diagnostics {
                PK_TRANSF_diagnostics_all_c
            } else {
                PK_TRANSF_diagnostics_none_c
            },
        };
        let mut r: PK_TRANSF_classify_r_t = unsafe { std::mem::zeroed() };
        let code = unsafe { PK_TRANSF_classify(self.tag, &opts, &mut r) };

        // Read everything out before freeing: the result struct is owned by the
        // kernel from here on.
        let out = Classification {
            matrix_type: MatrixType::from_token(r.matrix_type),
            determinant: r.determinant,
            scale: r.scale,
            translation: Vec3::from_pk(r.translation),
            perspective: Vec3::from_pk(r.perspective),
            unit_rows_deviations: diagnostics.then(|| Vec3::from_pk(r.unit_rows_deviations)),
            orthog_rows_deviations: diagnostics.then(|| Vec3::from_pk(r.orthog_rows_deviations)),
        };

        unsafe { PK_TRANSF_classify_r_f(&mut r) };
        crate::error::pk_check(code)?;
        Ok(out)
    }

    /// Run the kernel's consistency checks on this transform
    /// (`PK_TRANSF_check`).
    ///
    /// Returns the faults found — empty means valid. `max_faults` bounds how
    /// many the kernel will report.
    pub fn check(&self, max_faults: i32) -> PsResult<Vec<crate::check::CheckFault>> {
        let opts = PK_TRANSF_check_o_t {
            o_t_version: 1,
            max_faults,
        };
        let mut n_faults: std::os::raw::c_int = 0;
        let mut faults: *mut PK_check_fault_t = std::ptr::null_mut();
        pk_call!(PK_TRANSF_check(
            self.tag,
            &opts,
            &mut n_faults,
            &mut faults
        ));
        let array = unsafe { PkArray::from_raw(faults, n_faults) };
        Ok(array
            .iter()
            .map(|f| crate::check::CheckFault {
                entity: crate::entity::Entity::from_tag(f.entity),
                state: f.state,
                entity_2: (f.entity_2 != PK_ENTITY_null)
                    .then(|| crate::entity::Entity::from_tag(f.entity_2)),
            })
            .collect())
    }
}

// =============================================================================
// Applying a transform to orphan geometry
// =============================================================================

impl crate::surf::Surf {
    /// Return a copy of this surface placed by `transform`
    /// (`PK_GEOM_transform_2`).
    ///
    /// The bool reports whether the kernel achieved the placement **exactly** —
    /// false means the analytic form could not be preserved and the result is
    /// an approximation, which must never silently become oracle truth.
    pub fn transformed(&self, transform: &Transform) -> PsResult<(Self, bool)> {
        let out = transform.apply_to_geoms(&[self.tag()])?;
        let (tag, exact) = out[0];
        Ok((crate::surf::Surf::from_tag(tag), exact))
    }
}

impl crate::curve::Curve {
    /// Return a copy of this curve placed by `transform`
    /// (`PK_GEOM_transform_2`). See [`Surf::transformed`] for the `exact` flag.
    pub fn transformed(&self, transform: &Transform) -> PsResult<(Self, bool)> {
        let out = transform.apply_to_geoms(&[self.tag()])?;
        let (tag, exact) = out[0];
        Ok((crate::curve::Curve::from_tag(tag), exact))
    }
}
