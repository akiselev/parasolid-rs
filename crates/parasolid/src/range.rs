//! Bounding-box oracle — a cheap, robust coarse invariant.
//!
//! Wraps `PK_TOPOL_find_box` (axis-aligned bounding box). Complements the mass
//! properties as a fast sanity signal: cheap to compute, hard to get subtly
//! wrong, and immediately catches gross modelling errors (wrong size, wrong
//! placement).

use parasolid_sys::*;

use crate::body::Body;
use crate::error::PsResult;
use crate::geom::Vec3;

/// An axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    /// Minimum corner `(xmin, ymin, zmin)`.
    pub min: Vec3,
    /// Maximum corner `(xmax, ymax, zmax)`.
    pub max: Vec3,
}

impl Aabb {
    /// The box's extents `(dx, dy, dz)`.
    pub fn size(&self) -> Vec3 {
        Vec3::new(self.max.x - self.min.x, self.max.y - self.min.y, self.max.z - self.min.z)
    }

    /// The box centre.
    pub fn center(&self) -> Vec3 {
        Vec3::new(
            0.5 * (self.min.x + self.max.x),
            0.5 * (self.min.y + self.max.y),
            0.5 * (self.min.z + self.max.z),
        )
    }
}

/// Where a point lies relative to a body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enclosure {
    /// Strictly inside the material.
    Inside,
    /// Strictly outside.
    Outside,
    /// On the boundary (a face, edge, or vertex).
    On,
}

impl Body {
    /// Classify a point against this body (inside / outside / on the boundary).
    ///
    /// Wraps `PK_BODY_contains_vector`. The point must be given in the body's
    /// own coordinate system.
    pub fn contains_point(&self, point: Vec3) -> PsResult<Enclosure> {
        let v = point.to_pk();
        let mut enclosure: PK_enclosure_t = 0;
        let mut topol: PK_TOPOL_t = PK_ENTITY_null;
        pk_call!(PK_BODY_contains_vector(self.tag, &v, &mut enclosure, &mut topol));
        Ok(match enclosure {
            PK_enclosure_inside_c => Enclosure::Inside,
            PK_enclosure_outside_c => Enclosure::Outside,
            PK_enclosure_on_c => Enclosure::On,
            other => {
                return Err(crate::error::PsError::Session(format!(
                    "unexpected PK_enclosure_t value {other}"
                )));
            }
        })
    }

    /// The body's axis-aligned bounding box.
    ///
    /// Note: for curved bodies Parasolid returns a guaranteed-containing box
    /// that may be slightly larger than the tight geometric extent.
    pub fn bounding_box(&self) -> PsResult<Aabb> {
        let mut b = PK_BOX_t { coord: [0.0; 6] };
        pk_call!(PK_TOPOL_find_box(self.tag, &mut b));
        Ok(Aabb {
            min: Vec3::new(b.coord[0], b.coord[1], b.coord[2]),
            max: Vec3::new(b.coord[3], b.coord[4], b.coord[5]),
        })
    }
}
