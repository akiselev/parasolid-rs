//! Intersection oracle — the full `PK_*_intersect_*` surface.
//!
//! Surface/surface intersection lives on [`Surf::intersect`](crate::Surf) (see
//! `surf.rs`); this module wraps the remaining five: curve/curve, surface/curve,
//! face/curve, face/face, face/surface. Together with `Surf::intersect` and the
//! boolean `Body::intersect`, every `PK_*_intersect_*` export in `distance.rs`
//! is reachable from safe code (see `docs/` for the coverage table).
//!
//! The kernel returns two flavours of result:
//! - **point intersections** (curve/curve, surface/curve, face/curve): isolated
//!   hits with their parameters — modelled as `*Hit` structs here;
//! - **point + curve intersections** (face/face, face/surface, surface/surface):
//!   modelled as [`SurfIntersection`](crate::SurfIntersection).

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::curve::Curve;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::face::Face;
use crate::geom::Vec3;
use crate::memory::PkArray;
use crate::surf::{IntersectionCurve, Surf, SurfIntersection};

/// A point where two curves cross.
#[derive(Debug, Clone, Copy)]
pub struct CurveCurveHit {
    /// Position of the intersection.
    pub position: Vec3,
    /// Parameter on the first curve.
    pub t1: f64,
    /// Parameter on the second curve.
    pub t2: f64,
    /// Raw `PK_intersect_vector_t` kind.
    pub kind: i32,
}

/// A point where a curve crosses a surface.
#[derive(Debug, Clone, Copy)]
pub struct SurfCurveHit {
    /// Position of the intersection.
    pub position: Vec3,
    /// `(u, v)` parameters on the surface.
    pub uv: (f64, f64),
    /// Parameter on the curve.
    pub t: f64,
    /// Raw `PK_intersect_vector_t` kind.
    pub kind: i32,
}

/// A point where a curve crosses a face.
#[derive(Debug, Clone, Copy)]
pub struct FaceCurveHit {
    /// Position of the intersection.
    pub position: Vec3,
    /// `(u, v)` parameters on the face's surface.
    pub uv: (f64, f64),
    /// Parameter on the curve.
    pub t: f64,
    /// The face topology (face/edge/vertex) hit at this point.
    pub topology: Entity,
    /// Raw `PK_intersect_fc_t` kind.
    pub kind: i32,
}

// Helper: build the point+curve result shared by face/face, face/surf, surf/surf.
fn collect_point_curves(
    n_vectors: c_int,
    vectors: *mut PK_VECTOR_t,
    n_curves: c_int,
    curves: *mut PK_CURVE_t,
    bounds: *mut PK_INTERVAL_t,
    types: *mut PK_intersect_curve_t,
) -> SurfIntersection {
    let vec_arr = unsafe { PkArray::from_raw(vectors, n_vectors) };
    let points = vec_arr.iter().map(|&v| Vec3::from_pk(v)).collect();
    let curve_arr = unsafe { PkArray::from_raw(curves, n_curves) };
    let bound_arr = unsafe { PkArray::from_raw(bounds, n_curves) };
    let type_arr = unsafe { PkArray::from_raw(types, n_curves) };
    let curves = (0..n_curves as usize)
        .map(|i| IntersectionCurve {
            curve: Curve::from_tag(curve_arr[i]),
            bounds: (bound_arr[i].low, bound_arr[i].high),
            kind: type_arr[i],
        })
        .collect();
    SurfIntersection { points, curves }
}

impl Curve {
    /// Intersect a bounded region of this curve with a bounded region of
    /// another (`PK_CURVE_intersect_curve`). Intervals are `(low, high)`
    /// parameter ranges.
    pub fn intersect_curve(
        &self,
        interval: (f64, f64),
        other: &Curve,
        other_interval: (f64, f64),
    ) -> PsResult<Vec<CurveCurveHit>> {
        let opts = PK_CURVE_intersect_curve_o_t {
            o_t_version: 1,
            have_box: PK_LOGICAL_false,
            r#box: PK_BOX_t { coord: [0.0; 6] },
            common_surf: PK_ENTITY_null,
        };
        let iv1 = PK_INTERVAL_t { low: interval.0, high: interval.1 };
        let iv2 = PK_INTERVAL_t { low: other_interval.0, high: other_interval.1 };
        let mut n: c_int = 0;
        let (mut vectors, mut ts1, mut ts2, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_CURVE_intersect_curve(
            self.tag, iv1, other.tag, iv2, &opts,
            &mut n, &mut vectors, &mut ts1, &mut ts2, &mut types,
        ));
        let v = unsafe { PkArray::from_raw(vectors, n) };
        let t1 = unsafe { PkArray::from_raw(ts1, n) };
        let t2 = unsafe { PkArray::from_raw(ts2, n) };
        let k = unsafe { PkArray::from_raw(types, n) };
        Ok((0..n as usize)
            .map(|i| CurveCurveHit {
                position: Vec3::from_pk(v[i]),
                t1: t1[i],
                t2: t2[i],
                kind: k[i],
            })
            .collect())
    }
}

impl Surf {
    /// Intersect this surface with a bounded region of a curve
    /// (`PK_SURF_intersect_curve`).
    pub fn intersect_curve(&self, curve: &Curve, interval: (f64, f64)) -> PsResult<Vec<SurfCurveHit>> {
        let opts = PK_SURF_intersect_curve_o_t {
            o_t_version: 1,
            have_box: PK_LOGICAL_false,
            r#box: PK_BOX_t { coord: [0.0; 6] },
        };
        let iv = PK_INTERVAL_t { low: interval.0, high: interval.1 };
        let mut n: c_int = 0;
        let (mut vectors, mut uvs, mut ts, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_SURF_intersect_curve(
            self.tag, curve.tag, iv, &opts,
            &mut n, &mut vectors, &mut uvs, &mut ts, &mut types,
        ));
        let v = unsafe { PkArray::from_raw(vectors, n) };
        let uv = unsafe { PkArray::from_raw(uvs, n) };
        let t = unsafe { PkArray::from_raw(ts, n) };
        let k = unsafe { PkArray::from_raw(types, n) };
        Ok((0..n as usize)
            .map(|i| SurfCurveHit {
                position: Vec3::from_pk(v[i]),
                uv: (uv[i][0], uv[i][1]),
                t: t[i],
                kind: k[i],
            })
            .collect())
    }
}

impl Face {
    /// Intersect this face with a bounded region of a curve
    /// (`PK_FACE_intersect_curve`, no options).
    pub fn intersect_curve(&self, curve: &Curve, interval: (f64, f64)) -> PsResult<Vec<FaceCurveHit>> {
        let iv = PK_INTERVAL_t { low: interval.0, high: interval.1 };
        let mut n: c_int = 0;
        let (mut vectors, mut uvs, mut ts, mut topols, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_FACE_intersect_curve(
            self.tag, curve.tag, iv,
            &mut n, &mut vectors, &mut uvs, &mut ts, &mut topols, &mut types,
        ));
        let v = unsafe { PkArray::from_raw(vectors, n) };
        let uv = unsafe { PkArray::from_raw(uvs, n) };
        let t = unsafe { PkArray::from_raw(ts, n) };
        let tp = unsafe { PkArray::from_raw(topols, n) };
        let k = unsafe { PkArray::from_raw(types, n) };
        Ok((0..n as usize)
            .map(|i| FaceCurveHit {
                position: Vec3::from_pk(v[i]),
                uv: (uv[i][0], uv[i][1]),
                t: t[i],
                topology: Entity::from_tag(tp[i]),
                kind: k[i],
            })
            .collect())
    }

    /// Intersect this face with another face (`PK_FACE_intersect_face`).
    pub fn intersect_face(&self, other: &Face) -> PsResult<SurfIntersection> {
        let opts = face_intersect_face_opts();
        let mut n_vectors: c_int = 0;
        let mut n_curves: c_int = 0;
        let (mut vectors, mut curves, mut bounds, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_FACE_intersect_face(
            self.tag, other.tag, &opts,
            &mut n_vectors, &mut vectors, &mut n_curves, &mut curves, &mut bounds, &mut types,
        ));
        Ok(collect_point_curves(n_vectors, vectors, n_curves, curves, bounds, types))
    }

    /// Intersect this face with a surface (`PK_FACE_intersect_surf`).
    pub fn intersect_surf(&self, surf: &Surf) -> PsResult<SurfIntersection> {
        let opts = face_intersect_surf_opts();
        let mut n_vectors: c_int = 0;
        let mut n_curves: c_int = 0;
        let (mut vectors, mut curves, mut bounds, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_FACE_intersect_surf(
            self.tag, surf.tag(), &opts,
            &mut n_vectors, &mut vectors, &mut n_curves, &mut curves, &mut bounds, &mut types,
        ));
        Ok(collect_point_curves(n_vectors, vectors, n_curves, curves, bounds, types))
    }
}

// The face/face and face/surf option structs carry extra uvbox fields; a
// zero-initialised, version-1 instance means "no restriction". `mem::zeroed`
// then setting o_t_version is the least error-prone way to build them without
// hard-coding every (still-unverified) trailing field.
fn face_intersect_face_opts() -> PK_FACE_intersect_face_o_t {
    let mut o = unsafe { std::mem::zeroed::<PK_FACE_intersect_face_o_t>() };
    o.o_t_version = 1;
    o
}
fn face_intersect_surf_opts() -> PK_FACE_intersect_surf_o_t {
    let mut o = unsafe { std::mem::zeroed::<PK_FACE_intersect_surf_o_t>() };
    o.o_t_version = 1;
    o
}
