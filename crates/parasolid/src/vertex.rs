//! Vertex type — a topological point.

use std::os::raw::c_int;
use parasolid_sys::*;
use crate::error::PsResult;
use crate::memory::PkArray;
use crate::entity::Entity;
use crate::body::Body;
use crate::edge::Edge;
use crate::face::Face;
use crate::geom::Vec3;

/// A vertex in a body's topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub(crate) tag: PK_VERTEX_t,
}

impl Vertex {
    pub(crate) fn from_tag(tag: PK_VERTEX_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    /// Return the body that owns this vertex.
    pub fn body(&self) -> PsResult<Body> {
        let mut body_tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_VERTEX_ask_body(self.tag, &mut body_tag));
        Ok(Body::from_tag(body_tag))
    }

    /// Return the 3D position of this vertex.
    pub fn point(&self) -> PsResult<Vec3> {
        let mut point: PK_POINT_t = PK_ENTITY_null;
        pk_call!(PK_VERTEX_ask_point(self.tag, &mut point));
        let mut sf = PK_POINT_sf_t { position: [0.0; 3] };
        pk_call!(PK_POINT_ask(point, &mut sf));
        Ok(Vec3::from_pk(sf.position))
    }

    /// Return all edges connected to this vertex.
    ///
    /// Uses `PK_VERTEX_ask_oriented_edges`; the orientation sense values are discarded.
    pub fn edges(&self) -> PsResult<Vec<Edge>> {
        let mut n: c_int = 0;
        let mut edge_ptr = std::ptr::null_mut();
        let mut sense_ptr: *mut PK_LOGICAL_t = std::ptr::null_mut();
        pk_call!(PK_VERTEX_ask_oriented_edges(self.tag, &mut n, &mut edge_ptr, &mut sense_ptr));
        let edges = unsafe { PkArray::from_raw(edge_ptr, n) }
            .iter()
            .map(|&tag| Edge::from_tag(tag))
            .collect();
        // Free the sense array — PK-allocated, not managed by the edge PkArray.
        if !sense_ptr.is_null() {
            unsafe {
                let _ = PK_MEMORY_free(sense_ptr as *mut std::os::raw::c_void);
            }
        }
        Ok(edges)
    }

    /// Return all faces connected to this vertex.
    pub fn faces(&self) -> PsResult<Vec<Face>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_VERTEX_ask_faces(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Face::from_tag(tag)).collect())
    }
}
