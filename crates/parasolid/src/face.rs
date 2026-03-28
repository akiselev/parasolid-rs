//! Face type — a bounded region of a surface.

use std::os::raw::c_int;
use parasolid_sys::*;
use crate::error::PsResult;
use crate::memory::PkArray;
use crate::entity::Entity;
use crate::body::Body;
use crate::edge::Edge;
use crate::vertex::Vertex;
use crate::surf::Surf;

/// A face in a body's topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face {
    pub(crate) tag: PK_FACE_t,
}

impl Face {
    pub(crate) fn from_tag(tag: PK_FACE_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    /// Return the body that owns this face.
    pub fn body(&self) -> PsResult<Body> {
        let mut body_tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_FACE_ask_body(self.tag, &mut body_tag));
        Ok(Body::from_tag(body_tag))
    }

    /// Return all edges bounding this face.
    pub fn edges(&self) -> PsResult<Vec<Edge>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_FACE_ask_edges(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Edge::from_tag(tag)).collect())
    }

    /// Return all vertices on this face.
    pub fn vertices(&self) -> PsResult<Vec<Vertex>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_FACE_ask_vertices(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Vertex::from_tag(tag)).collect())
    }

    /// Return the surface geometry tag of this face.
    pub fn surf_tag(&self) -> PsResult<PK_SURF_t> {
        let mut surf: PK_SURF_t = PK_ENTITY_null;
        pk_call!(PK_FACE_ask_surf(self.tag, &mut surf));
        Ok(surf)
    }

    /// Return the surface geometry as a typed `Surf` handle.
    pub fn surf(&self) -> PsResult<Surf> {
        Ok(Surf::from_tag(self.surf_tag()?))
    }

    /// Return the orientation of the face relative to its surface.
    ///
    /// `true` means the face normal agrees with the surface normal.
    pub fn orientation(&self) -> PsResult<bool> {
        let mut surf: PK_SURF_t = PK_ENTITY_null;
        let mut orient: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_FACE_ask_oriented_surf(self.tag, &mut surf, &mut orient));
        Ok(orient == PK_LOGICAL_true)
    }
}
