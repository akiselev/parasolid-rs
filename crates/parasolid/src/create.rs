//! Standalone (orphan) analytic geometry creation.
//!
//! These build bare `Surf` / `Curve` / `Point` geometry from parameters — the
//! exact analytic types CADabra emits (plane, cylinder, cone, sphere, torus,
//! line, circle, ellipse, point), at arbitrary poses. Orphan surfaces and
//! curves can be fed straight into the intersection oracle
//! ([`Surf::intersect`](crate::Surf), etc.) or wrapped as sheet/wire bodies.
//!
//! Every constructor is validated by a create → `ask` round-trip in the test
//! suite: the numbers you put in come back out.

use parasolid_sys::*;

use crate::curve::Curve;
use crate::error::PsResult;
use crate::geom::{Axis2, Vec3};
use crate::point::Point;
use crate::surf::Surf;

impl Surf {
    /// Create an orphan plane from a placement (origin + normal `axis`).
    pub fn plane(basis: Axis2) -> PsResult<Surf> {
        let sf = PK_PLANE_sf_t { basis_set: basis.to_pk() };
        let mut tag: PK_PLANE_t = PK_ENTITY_null;
        pk_call!(PK_PLANE_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Create an orphan cylinder (axis along `basis.axis`).
    pub fn cylinder(basis: Axis2, radius: f64) -> PsResult<Surf> {
        let sf = PK_CYL_sf_t { basis_set: basis.to_pk(), radius };
        let mut tag: PK_CYLL_t = PK_ENTITY_null;
        pk_call!(PK_CYL_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Create an orphan cone. `radius` is the radius at the basis origin;
    /// `semi_angle` is the half-angle (radians). The surface widens along
    /// `+basis.axis`.
    pub fn cone(basis: Axis2, radius: f64, semi_angle: f64) -> PsResult<Surf> {
        let sf = PK_CONE_sf_t { basis_set: basis.to_pk(), radius, semi_angle };
        let mut tag: PK_CONE_t = PK_ENTITY_null;
        pk_call!(PK_CONE_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Create an orphan sphere centred at `basis.origin`.
    pub fn sphere(basis: Axis2, radius: f64) -> PsResult<Surf> {
        let sf = PK_SPHERE_sf_t { basis_set: basis.to_pk(), radius };
        let mut tag: PK_SPHERE_t = PK_ENTITY_null;
        pk_call!(PK_SPHERE_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Create an orphan torus (axis of revolution along `basis.axis`).
    pub fn torus(basis: Axis2, major_radius: f64, minor_radius: f64) -> PsResult<Surf> {
        let sf = PK_TORUS_sf_t { basis_set: basis.to_pk(), major_radius, minor_radius };
        let mut tag: PK_TORUS_t = PK_ENTITY_null;
        pk_call!(PK_TORUS_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }
}

impl Curve {
    /// Create an orphan line through `location` with direction `direction`.
    pub fn line(location: Vec3, direction: Vec3) -> PsResult<Curve> {
        let sf = PK_LINE_sf_t {
            basis_set: PK_AXIS1_sf_t { location: location.to_pk(), axis: direction.to_pk() },
        };
        let mut tag: PK_LINE_t = PK_ENTITY_null;
        pk_call!(PK_LINE_create(&sf, &mut tag));
        Ok(Curve::from_tag(tag))
    }

    /// Create an orphan circle in the plane of `basis` (normal `basis.axis`),
    /// centred at `basis.origin`.
    pub fn circle(basis: Axis2, radius: f64) -> PsResult<Curve> {
        let sf = PK_CIRCLE_sf_t { basis_set: basis.to_pk(), radius };
        let mut tag: PK_CIRCLE_t = PK_ENTITY_null;
        pk_call!(PK_CIRCLE_create(&sf, &mut tag));
        Ok(Curve::from_tag(tag))
    }

    /// Create an orphan ellipse with semi-axes `r1` (along `basis.ref_direction`)
    /// and `r2`, in the plane of `basis`.
    pub fn ellipse(basis: Axis2, r1: f64, r2: f64) -> PsResult<Curve> {
        let sf = PK_ELLIPSE_sf_t { basis_set: basis.to_pk(), R1: r1, R2: r2 };
        let mut tag: PK_ELLIPSE_t = PK_ENTITY_null;
        pk_call!(PK_ELLIPSE_create(&sf, &mut tag));
        Ok(Curve::from_tag(tag))
    }
}

impl Point {
    /// Create an orphan point at `position`.
    pub fn create(position: Vec3) -> PsResult<Point> {
        let sf = PK_POINT_sf_t { position: position.to_pk() };
        let mut tag: PK_POINT_t = PK_ENTITY_null;
        pk_call!(PK_POINT_create(&sf, &mut tag));
        Ok(Point::from_tag(tag))
    }
}
