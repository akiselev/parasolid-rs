//! Core geometric types used throughout the safe API.

use parasolid_sys::*;

/// 3D position/vector.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self { Self { x, y, z } }
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0 } }

    pub(crate) fn from_pk(v: PK_VECTOR_t) -> Self {
        Self { x: v[0], y: v[1], z: v[2] }
    }

    pub(crate) fn to_pk(&self) -> PK_VECTOR_t {
        [self.x, self.y, self.z]
    }
}

/// Axis2 placement: origin + Z-axis + X-reference direction.
#[derive(Debug, Clone, Copy)]
pub struct Axis2 {
    pub origin: Vec3,
    pub axis: Vec3,
    pub ref_direction: Vec3,
}

impl Axis2 {
    pub(crate) fn from_pk(sf: PK_AXIS2_sf_t) -> Self {
        Self {
            origin: Vec3::from_pk(sf.location),
            axis: Vec3::from_pk(sf.axis),
            ref_direction: Vec3::from_pk(sf.ref_direction),
        }
    }

    pub(crate) fn to_pk(&self) -> PK_AXIS2_sf_t {
        PK_AXIS2_sf_t {
            location: self.origin.to_pk(),
            axis: self.axis.to_pk(),
            ref_direction: self.ref_direction.to_pk(),
        }
    }
}
