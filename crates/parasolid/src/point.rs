//! Point type — a geometric point entity.

use parasolid_sys::*;
use crate::error::PsResult;
use crate::entity::Entity;
use crate::geom::Vec3;

/// A point entity handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub(crate) tag: PK_POINT_t,
}

impl Point {
    pub(crate) fn from_tag(tag: PK_POINT_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    /// Return the 3D position of this point.
    pub fn position(&self) -> PsResult<Vec3> {
        let mut sf = PK_POINT_sf_t { position: [0.0; 3] };
        pk_call!(PK_POINT_ask(self.tag, &mut sf));
        Ok(Vec3::from_pk(sf.position))
    }
}
