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

    /// Return the faces adjacent to this one (sharing an edge).
    pub fn adjacent_faces(&self) -> PsResult<Vec<Face>> {
        let this = [self.tag];
        let mut n: c_int = 0;
        let mut ptr: *mut PK_FACE_t = std::ptr::null_mut();
        pk_call!(PK_FACE_ask_faces_adjacent(
            1,
            this.as_ptr(),
            std::ptr::null(),
            &mut n,
            &mut ptr,
        ));
        let arr = unsafe { PkArray::from_raw(ptr, n) };
        Ok(arr.iter().map(|&t| Face::from_tag(t)).collect())
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

    /// Return all loops bounding this face.
    pub fn loops(&self) -> PsResult<Vec<crate::Loop>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_FACE_ask_loops(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| crate::Loop::from_tag(tag)).collect())
    }

    /// Return the first loop of this face, or `None` if it has none.
    pub fn first_loop(&self) -> PsResult<Option<crate::Loop>> {
        let mut tag: PK_LOOP_t = PK_ENTITY_null;
        pk_call!(PK_FACE_ask_first_loop(self.tag, &mut tag));
        Ok((tag != PK_ENTITY_null).then(|| crate::Loop::from_tag(tag)))
    }

    /// Return the shell this face belongs to (a face is used by exactly one shell).
    pub fn shell(&self) -> PsResult<crate::Shell> {
        let mut tag: PK_SHELL_t = PK_ENTITY_null;
        pk_call!(PK_FACE_ask_shells(self.tag, &mut tag));
        Ok(crate::Shell::from_tag(tag))
    }

    /// Return the next face in the owning body's face list, or `None` at the end.
    pub fn next_in_body(&self) -> PsResult<Option<Face>> {
        let mut tag: PK_FACE_t = PK_ENTITY_null;
        pk_call!(PK_FACE_ask_next_in_body(self.tag, &mut tag));
        Ok((tag != PK_ENTITY_null).then(|| Face::from_tag(tag)))
    }

    /// The trimmed parameter box of this face on its surface (may be tighter
    /// than the full surface uvbox).
    pub fn uvbox(&self) -> PsResult<crate::UvBox> {
        let mut b = PK_UVBOX_t { param: [0.0; 4] };
        pk_call!(PK_FACE_find_uvbox(self.tag, &mut b));
        Ok(crate::UvBox { u_min: b.param[0], v_min: b.param[1], u_max: b.param[2], v_max: b.param[3] })
    }

    /// Whether this face fills a simple uvbox-bounded patch of its surface, and
    /// if so, that box.
    pub fn is_uvbox(&self) -> PsResult<bool> {
        let mut is_uvbox: PK_LOGICAL_t = PK_LOGICAL_false;
        let mut b = PK_UVBOX_t { param: [0.0; 4] };
        pk_call!(PK_FACE_is_uvbox(self.tag, &mut is_uvbox, &mut b));
        Ok(is_uvbox == PK_LOGICAL_true)
    }

    /// Whether this face is periodic in `(u, v)` (a seam or a full wrap counts
    /// as periodic).
    pub fn is_periodic(&self) -> PsResult<(bool, bool)> {
        let mut u: PK_PARAM_periodic_t = 0;
        let mut v: PK_PARAM_periodic_t = 0;
        pk_call!(PK_FACE_is_periodic(self.tag, &mut u, &mut v));
        let per = |t: PK_PARAM_periodic_t| {
            t == PK_PARAM_periodic_yes_c || t == PK_PARAM_periodic_seamed_c
        };
        Ok((per(u), per(v)))
    }

    /// Imprint `pos` onto this face as an isolated vertex, returning it. The
    /// point must lie on the face. (General-body topology must be enabled for a
    /// solid body, since this creates a non-manifold vertex.)
    pub fn imprint_point(&self, pos: crate::geom::Vec3) -> PsResult<Vertex> {
        let point = crate::Point::create(pos)?;
        let mut v: PK_VERTEX_t = PK_ENTITY_null;
        pk_call!(PK_FACE_imprint_point(self.tag, point.tag(), &mut v));
        Ok(Vertex::from_tag(v))
    }

    /// The edges shared between this face and `other` (the face-face
    /// shared-edge oracle).
    pub fn common_edges(&self, other: Face) -> PsResult<Vec<Edge>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_FACE_find_edges_common(self.tag, other.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&t| Edge::from_tag(t)).collect())
    }

    /// Imprint a curve onto this face over the parameter interval `bounds`,
    /// splitting it where the curve lies within the face.
    ///
    /// The curve must lie on this face's surface. Returns `(new_edges,
    /// new_faces)` created by the imprint. `PK_FACE_imprint_curve` takes the
    /// interval by value (bound as `*const PK_INTERVAL_t`, ABI-equivalent on
    /// Win64).
    pub fn imprint_curve(
        &self,
        curve: &crate::Curve,
        bounds: (f64, f64),
    ) -> PsResult<(Vec<Edge>, Vec<Face>)> {
        let iv = PK_INTERVAL_t { low: bounds.0, high: bounds.1 };
        let mut n_new_edges: c_int = 0;
        let mut new_edges: *mut PK_EDGE_t = std::ptr::null_mut();
        let mut n_new_faces: c_int = 0;
        let mut new_faces: *mut PK_FACE_t = std::ptr::null_mut();
        pk_call!(PK_FACE_imprint_curve(
            self.tag,
            curve.tag(),
            &iv,
            &mut n_new_edges,
            &mut new_edges,
            &mut n_new_faces,
            &mut new_faces,
        ));
        let edges = unsafe { PkArray::from_raw(new_edges, n_new_edges) }
            .iter()
            .map(|&t| Edge::from_tag(t))
            .collect();
        let faces = unsafe { PkArray::from_raw(new_faces, n_new_faces) }
            .iter()
            .map(|&t| Face::from_tag(t))
            .collect();
        Ok((edges, faces))
    }
}
