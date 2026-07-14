//! B-rep spine types: Region, Shell, Loop, Fin.
//!
//! These expose the Body → Region → Shell → Face → Loop → Fin → Edge → Vertex
//! adjacency so a CADabra-built model can be compared against the Parasolid
//! oracle structure-for-structure.

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::edge::Edge;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::face::Face;
use crate::memory::PkArray;

/// The type of a loop (outer boundary, inner hole, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopType {
    Outer,
    Inner,
    Winding,
    Vertex,
    Wire,
    /// Any other loop-type token the kernel returns (raw value preserved).
    Other(i32),
}

impl LoopType {
    fn from_raw(v: PK_LOOP_type_t) -> Self {
        match v {
            PK_LOOP_type_outer_c => LoopType::Outer,
            PK_LOOP_type_inner_c => LoopType::Inner,
            PK_LOOP_type_winding_c => LoopType::Winding,
            PK_LOOP_type_vertex_c => LoopType::Vertex,
            PK_LOOP_type_wire_c => LoopType::Wire,
            other => LoopType::Other(other),
        }
    }
}

macro_rules! tag_wrapper {
    ($(#[$m:meta])* $name:ident, $tag_ty:ty) => {
        $(#[$m])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name {
            pub(crate) tag: $tag_ty,
        }
        impl $name {
            pub(crate) fn from_tag(tag: $tag_ty) -> Self { Self { tag } }
            /// The raw PK tag.
            pub fn tag(&self) -> i32 { self.tag }
            /// This entity as a generic [`Entity`].
            pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }
        }
    };
}

tag_wrapper!(
    /// A connected volume of space bounded by shells (solid or void).
    Region, PK_REGION_t);
tag_wrapper!(
    /// A connected set of faces (a boundary of a region).
    Shell, PK_SHELL_t);
tag_wrapper!(
    /// A connected boundary of a face, made of fins.
    Loop, PK_LOOP_t);
tag_wrapper!(
    /// A directed use of an edge by a loop (half-edge).
    Fin, PK_FIN_t);

fn collect<T: Copy, U>(ptr: *mut T, n: c_int, f: impl Fn(T) -> U) -> Vec<U> {
    let array = unsafe { PkArray::from_raw(ptr, n) };
    array.iter().map(|&t| f(t)).collect()
}

impl Region {
    /// The shells bounding this region.
    pub fn shells(&self) -> PsResult<Vec<Shell>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_REGION_ask_shells(self.tag, &mut n, &mut ptr));
        Ok(collect(ptr, n, Shell::from_tag))
    }

    /// Whether this region is solid (material) rather than void.
    pub fn is_solid(&self) -> PsResult<bool> {
        let mut is_solid: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_REGION_is_solid(self.tag, &mut is_solid));
        Ok(is_solid == PK_LOGICAL_true)
    }
}

impl Shell {
    /// The region this shell bounds.
    pub fn region(&self) -> PsResult<Region> {
        let mut tag: PK_REGION_t = PK_ENTITY_null;
        pk_call!(PK_SHELL_ask_region(self.tag, &mut tag));
        Ok(Region::from_tag(tag))
    }

    /// The faces of this shell together with each face's orientation relative
    /// to the shell (`true` = surface normal points out of the region).
    pub fn oriented_faces(&self) -> PsResult<Vec<(Face, bool)>> {
        let mut n = 0;
        let mut faces = std::ptr::null_mut();
        let mut orients = std::ptr::null_mut();
        pk_call!(PK_SHELL_ask_oriented_faces(self.tag, &mut n, &mut faces, &mut orients));
        let face_arr = unsafe { PkArray::from_raw(faces, n) };
        let orient_arr = unsafe { PkArray::from_raw(orients, n) };
        Ok(face_arr
            .iter()
            .zip(orient_arr.iter())
            .map(|(&f, &o)| (Face::from_tag(f), o == PK_LOGICAL_true))
            .collect())
    }

    /// The faces of this shell (orientation discarded).
    pub fn faces(&self) -> PsResult<Vec<Face>> {
        Ok(self.oriented_faces()?.into_iter().map(|(f, _)| f).collect())
    }
}

impl Loop {
    /// The face this loop bounds.
    pub fn face(&self) -> PsResult<Face> {
        let mut tag: PK_FACE_t = PK_ENTITY_null;
        pk_call!(PK_LOOP_ask_face(self.tag, &mut tag));
        Ok(Face::from_tag(tag))
    }

    /// The fins (directed edge uses) forming this loop.
    pub fn fins(&self) -> PsResult<Vec<Fin>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_LOOP_ask_fins(self.tag, &mut n, &mut ptr));
        Ok(collect(ptr, n, Fin::from_tag))
    }

    /// The loop type (outer boundary, inner hole, …).
    pub fn loop_type(&self) -> PsResult<LoopType> {
        let mut t: PK_LOOP_type_t = 0;
        pk_call!(PK_LOOP_ask_type(self.tag, &mut t));
        Ok(LoopType::from_raw(t))
    }
}

impl Fin {
    /// The edge this fin is a directed use of.
    pub fn edge(&self) -> PsResult<Edge> {
        let mut tag: PK_EDGE_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_edge(self.tag, &mut tag));
        Ok(Edge::from_tag(tag))
    }

    /// The loop this fin belongs to.
    pub fn loop_(&self) -> PsResult<Loop> {
        let mut tag: PK_LOOP_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_loop(self.tag, &mut tag));
        Ok(Loop::from_tag(tag))
    }

    /// The face this fin lies on.
    pub fn face(&self) -> PsResult<Face> {
        let mut tag: PK_FACE_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_face(self.tag, &mut tag));
        Ok(Face::from_tag(tag))
    }

    /// The next fin around the loop.
    pub fn next_in_loop(&self) -> PsResult<Fin> {
        let mut tag: PK_FIN_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_next_in_loop(self.tag, &mut tag));
        Ok(Fin::from_tag(tag))
    }

    /// The previous fin around the loop.
    pub fn previous_in_loop(&self) -> PsResult<Fin> {
        let mut tag: PK_FIN_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_previous_in_loop(self.tag, &mut tag));
        Ok(Fin::from_tag(tag))
    }
}
