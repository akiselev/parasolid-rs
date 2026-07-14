//! Edge type — a bounded segment of a curve.

use std::os::raw::c_int;
use parasolid_sys::*;
use crate::error::PsResult;
use crate::memory::PkArray;
use crate::entity::Entity;
use crate::body::Body;
use crate::face::Face;
use crate::vertex::Vertex;
use crate::curve::Curve;

/// An edge in a body's topology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub(crate) tag: PK_EDGE_t,
}

impl Edge {
    pub(crate) fn from_tag(tag: PK_EDGE_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    /// Return the body that owns this edge.
    pub fn body(&self) -> PsResult<Body> {
        let mut body_tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_ask_body(self.tag, &mut body_tag));
        Ok(Body::from_tag(body_tag))
    }

    /// Return the two vertices at each end of this edge.
    ///
    /// Returns `(start_vertex, end_vertex)`. For closed edges both are the same.
    pub fn vertices(&self) -> PsResult<(Vertex, Vertex)> {
        let mut verts: [PK_VERTEX_t; 2] = [PK_ENTITY_null; 2];
        pk_call!(PK_EDGE_ask_vertices(self.tag, &mut verts));
        Ok((Vertex::from_tag(verts[0]), Vertex::from_tag(verts[1])))
    }

    /// Return all faces adjacent to this edge.
    pub fn faces(&self) -> PsResult<Vec<Face>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_EDGE_ask_faces(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Face::from_tag(tag)).collect())
    }

    /// Return the curve geometry tag of this edge.
    pub fn curve_tag(&self) -> PsResult<PK_CURVE_t> {
        let mut curve: PK_CURVE_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_ask_curve(self.tag, &mut curve));
        Ok(curve)
    }

    /// Return the curve geometry as a typed `Curve` handle.
    pub fn curve(&self) -> PsResult<Curve> {
        Ok(Curve::from_tag(self.curve_tag()?))
    }

    /// Return the parametric interval of this edge on its underlying curve.
    ///
    /// Returns `(t_min, t_max)`.
    pub fn interval(&self) -> PsResult<(f64, f64)> {
        let mut curve: PK_CURVE_t = PK_ENTITY_null;
        let mut class: PK_CLASS_t = 0;
        let mut ends = [PK_VECTOR_t::default(); 2];
        let mut t_range = PK_INTERVAL_t { low: 0.0, high: 0.0 };
        let mut sense: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_EDGE_ask_geometry(
            self.tag,
            PK_LOGICAL_true,
            &mut curve,
            &mut class,
            ends.as_mut_ptr(),
            &mut t_range,
            &mut sense
        ));
        Ok((t_range.low, t_range.high))
    }
}
