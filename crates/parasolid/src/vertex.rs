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

    /// The vertex classification (isolated / spur / wire / normal).
    pub fn vertex_type(&self) -> PsResult<VertexType> {
        let mut t: PK_VERTEX_type_t = 0;
        pk_call!(PK_VERTEX_ask_type(self.tag, &mut t));
        Ok(VertexType::from_raw(t))
    }

    /// The shells that reference this vertex.
    pub fn shells(&self) -> PsResult<Vec<crate::Shell>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_VERTEX_ask_shells(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| crate::Shell::from_tag(tag)).collect())
    }

    /// The vertex-only loops where this vertex is the sole content.
    pub fn isolated_loops(&self) -> PsResult<Vec<crate::Loop>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_VERTEX_ask_isolated_loops(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| crate::Loop::from_tag(tag)).collect())
    }

    /// The tolerant-vertex precision (the geometric tolerance band of the vertex).
    pub fn precision(&self) -> PsResult<f64> {
        let mut p = 0.0f64;
        pk_call!(PK_VERTEX_ask_precision(self.tag, &mut p));
        Ok(p)
    }

    /// Set this vertex's precision, making it a tolerant vertex.
    pub fn set_precision(&self, tol: f64) -> PsResult<()> {
        pk_call!(PK_VERTEX_set_precision(self.tag, tol));
        Ok(())
    }
}

/// The classification of a vertex.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexType {
    /// A free point not on any edge.
    Isolated,
    /// The end of a dangling (spur) edge.
    Spur,
    /// A vertex on wireframe edges only.
    Wire,
    /// A normal vertex on the solid/sheet boundary.
    Normal,
    Other(i32),
}

impl VertexType {
    fn from_raw(v: PK_VERTEX_type_t) -> Self {
        match v {
            PK_VERTEX_type_isolated_c => VertexType::Isolated,
            PK_VERTEX_type_spur_c => VertexType::Spur,
            PK_VERTEX_type_wire_c => VertexType::Wire,
            PK_VERTEX_type_normal_c => VertexType::Normal,
            o => VertexType::Other(o),
        }
    }
}
