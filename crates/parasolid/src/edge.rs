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
use crate::geom::Vec3;

/// The type of an edge as reported by `PK_EDGE_ask_type` — the edge's
/// vertex-topology (open/closed/ring). The manifold family
/// (wireframe/laminar/normal/general) shares the `PK_EDGE_type_t` enum but is a
/// separate classification; it is modelled here too for completeness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeType {
    /// Distinct start/end vertices (e.g. a block edge).
    Open,
    /// One vertex, start == end (e.g. a full circle with a seam vertex).
    Closed,
    /// No vertices (a bare ring).
    Ring,
    /// No fins (a free wireframe edge).
    Wireframe,
    /// One fin (a sheet boundary).
    Laminar,
    /// Two fins of opposite sense (a normal manifold edge).
    Normal,
    /// Two-or-more fins forming a non-manifold junction.
    General,
    Other(i32),
}

impl EdgeType {
    fn from_raw(v: PK_EDGE_type_t) -> Self {
        match v {
            PK_EDGE_type_open_c => EdgeType::Open,
            PK_EDGE_type_closed_c => EdgeType::Closed,
            PK_EDGE_type_ring_c => EdgeType::Ring,
            PK_EDGE_type_wireframe_c => EdgeType::Wireframe,
            PK_EDGE_type_laminar_c => EdgeType::Laminar,
            PK_EDGE_type_normal_c => EdgeType::Normal,
            PK_EDGE_type_general_c => EdgeType::General,
            o => EdgeType::Other(o),
        }
    }
}

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

    /// Raw edge convexity token (`PK_EDGE_convexity_*_c`) — e.g. convex (23597)
    /// or concave (23598) for a manifold edge.
    pub fn convexity(&self) -> PsResult<i32> {
        let mut c: PK_EDGE_convexity_t = 0;
        pk_call!(PK_EDGE_ask_convexity(self.tag, std::ptr::null(), &mut c));
        Ok(c)
    }

    /// Whether the edge is smooth (tangent-continuous) to within `max_angle`
    /// radians between the two adjacent faces.
    pub fn is_smooth(&self, max_angle: f64) -> PsResult<bool> {
        let mut s: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_EDGE_is_smooth(self.tag, max_angle, &mut s));
        Ok(s == PK_LOGICAL_true)
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

    /// Test whether `point` lies on this edge.
    ///
    /// Wraps `PK_EDGE_contains_vector`, which takes the position **by value**
    /// (`PK_VECTOR_t` is a 24-byte `[f64; 3]`, passed indirectly on the Win64
    /// ABI). The kernel reports the sub-topology (edge or bounding vertex) that
    /// contains the point; we return `true` when that handle is non-null.
    pub fn contains_point(&self, point: crate::geom::Vec3) -> PsResult<bool> {
        let v: PK_VECTOR_t = point.to_pk();
        let mut topol: PK_TOPOL_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_contains_vector(self.tag, v, &mut topol));
        Ok(topol != PK_ENTITY_null)
    }

    /// Return all fins (directed edge uses) of this edge. A manifold edge
    /// shared by two faces has two fins.
    pub fn fins(&self) -> PsResult<Vec<crate::Fin>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_EDGE_ask_fins(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| crate::Fin::from_tag(tag)).collect())
    }

    /// The manifold classification of this edge (wireframe/laminar/normal/general).
    pub fn edge_type(&self) -> PsResult<EdgeType> {
        let mut t: PK_EDGE_type_t = 0;
        pk_call!(PK_EDGE_ask_type(self.tag, &mut t));
        Ok(EdgeType::from_raw(t))
    }

    /// The first fin of this edge (cheaper than [`fins`](Self::fins) when only one is needed).
    pub fn first_fin(&self) -> PsResult<crate::Fin> {
        let mut tag: PK_FIN_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_ask_first_fin(self.tag, &mut tag));
        Ok(crate::Fin::from_tag(tag))
    }

    /// The curve carrying this edge plus the edge's sense relative to it.
    pub fn oriented_curve(&self) -> PsResult<(Curve, bool)> {
        let mut tag: PK_CURVE_t = PK_ENTITY_null;
        let mut orient: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_EDGE_ask_oriented_curve(self.tag, &mut tag, &mut orient));
        Ok((Curve::from_tag(tag), orient == PK_LOGICAL_true))
    }

    /// The shells that reference this edge (esp. for wireframe edges).
    pub fn shells(&self) -> PsResult<Vec<crate::Shell>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_EDGE_ask_shells(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| crate::Shell::from_tag(tag)).collect())
    }

    /// The next edge in the owning body's edge list, or `None` at the end.
    pub fn next_in_body(&self) -> PsResult<Option<Edge>> {
        let mut tag: PK_EDGE_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_ask_next_in_body(self.tag, &mut tag));
        Ok((tag != PK_ENTITY_null).then(|| Edge::from_tag(tag)))
    }

    /// Whether this edge lies in a plane; returns the plane normal when it does
    /// and the kernel can determine one (a straight edge lies in many planes and
    /// may return `None`).
    pub fn is_planar(&self) -> PsResult<(bool, Option<Vec3>)> {
        let mut is_planar: PK_LOGICAL_t = PK_LOGICAL_false;
        let mut plane: PK_PLANE_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_is_planar(self.tag, PK_LOGICAL_true, &mut is_planar, &mut plane));
        let planar = is_planar == PK_LOGICAL_true;
        if planar && plane != PK_ENTITY_null {
            let mut sf = unsafe { std::mem::zeroed::<PK_PLANE_sf_t>() };
            pk_call!(PK_PLANE_ask(plane, &mut sf));
            Ok((true, Some(Vec3::from_pk(sf.basis_set.axis))))
        } else {
            Ok((planar, None))
        }
    }

    /// The start/end positions and tangent directions of this edge, as
    /// `((start_pos, start_tan), (end_pos, end_tan))`.
    pub fn end_tangents(&self) -> PsResult<((Vec3, Vec3), (Vec3, Vec3))> {
        let mut start = PK_VECTOR_t::default();
        let mut start_tan = PK_VECTOR_t::default();
        let mut end = PK_VECTOR_t::default();
        let mut end_tan = PK_VECTOR_t::default();
        pk_call!(PK_EDGE_find_end_tangents(
            self.tag,
            &mut start,
            &mut start_tan,
            &mut end,
            &mut end_tan,
        ));
        Ok((
            (Vec3::from_pk(start), Vec3::from_pk(start_tan)),
            (Vec3::from_pk(end), Vec3::from_pk(end_tan)),
        ))
    }

    /// The tolerant-edge precision (the geometric tolerance band of the edge).
    pub fn precision(&self) -> PsResult<f64> {
        let mut p = 0.0f64;
        pk_call!(PK_EDGE_ask_precision(self.tag, &mut p));
        Ok(p)
    }

    /// Split this edge at `pos` (which must lie on the edge), returning the new
    /// vertex and the new edge created by the split.
    pub fn imprint_point(&self, pos: Vec3) -> PsResult<(Vertex, Edge)> {
        let point = crate::Point::create(pos)?;
        let mut v: PK_VERTEX_t = PK_ENTITY_null;
        let mut e: PK_EDGE_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_imprint_point(self.tag, point.tag(), &mut v, &mut e));
        Ok((Vertex::from_tag(v), Edge::from_tag(e)))
    }

    /// The chain of edges tangent-continuous (G1) with this one across shared
    /// vertices, within `tolerance`. If `same_convexity`, the chain is further
    /// restricted to edges of matching convexity (feature-edge grouping).
    pub fn g1_edges(&self, tolerance: f64, same_convexity: bool) -> PsResult<Vec<Edge>> {
        let conv = if same_convexity { PK_LOGICAL_true } else { PK_LOGICAL_false };
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_EDGE_find_g1_edges(self.tag, tolerance, conv, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&t| Edge::from_tag(t)).collect())
    }

    /// The extreme point of this edge in a lexicographic direction ordering
    /// (`dirs[0]` primary, `dirs[1]`/`dirs[2]` tie-breakers), and the sub-topology
    /// (usually a vertex) at that point. A support-function oracle.
    pub fn extreme(&self, dirs: [Vec3; 3]) -> PsResult<(Vec3, Entity)> {
        let d1 = dirs[0].to_pk();
        let d2 = dirs[1].to_pk();
        let d3 = dirs[2].to_pk();
        let mut ex = PK_VECTOR_t::default();
        let mut topol: PK_TOPOL_t = PK_ENTITY_null;
        pk_call!(PK_EDGE_find_extreme(self.tag, &d1, &d2, &d3, &mut ex, &mut topol));
        Ok((Vec3::from_pk(ex), Entity::from_tag(topol)))
    }
}
