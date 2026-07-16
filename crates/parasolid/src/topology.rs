//! B-rep spine types: Region, Shell, Loop, Fin.
//!
//! These expose the Body → Region → Shell → Face → Loop → Fin → Edge → Vertex
//! adjacency so a CADabra-built model can be compared against the Parasolid
//! oracle structure-for-structure.

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::body::Body;
use crate::curve::Curve;
use crate::edge::Edge;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::face::Face;
use crate::memory::PkArray;
use crate::vertex::Vertex;

/// The type of a fin (half-edge use).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinType {
    Wire,
    Biwire,
    Normal,
    Other(i32),
}

impl FinType {
    fn from_raw(v: PK_FIN_type_t) -> Self {
        match v {
            PK_FIN_type_wire_c => FinType::Wire,
            PK_FIN_type_biwire_c => FinType::Biwire,
            PK_FIN_type_normal_c => FinType::Normal,
            o => FinType::Other(o),
        }
    }
}

/// The type of a shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellType {
    Acorn,
    Wireframe,
    WireframeFree,
    Mixed,
    /// A normal solid/sheet-boundary shell (or any other token).
    Other(i32),
}

impl ShellType {
    fn from_raw(v: PK_SHELL_type_t) -> Self {
        match v {
            PK_SHELL_type_acorn_c => ShellType::Acorn,
            PK_SHELL_type_wireframe_c => ShellType::Wireframe,
            PK_SHELL_type_wireframe_free_c => ShellType::WireframeFree,
            PK_SHELL_type_mixed_c => ShellType::Mixed,
            o => ShellType::Other(o),
        }
    }
}

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

    /// The body that owns this region.
    pub fn body(&self) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_REGION_ask_body(self.tag, &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// The regions sharing a face with this one — the region-adjacency (cell)
    /// graph the regularized-boolean classify/commit stage operates on.
    pub fn adjacent_regions(&self) -> PsResult<Vec<Region>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_REGION_ask_regions_adjacent(self.tag, &mut n, &mut ptr));
        Ok(collect(ptr, n, Region::from_tag))
    }

    /// Imprint `pos` into this region as an isolated vertex, returning it.
    pub fn imprint_point(&self, pos: crate::geom::Vec3) -> PsResult<Vertex> {
        let point = crate::Point::create(pos)?;
        let mut v: PK_VERTEX_t = PK_ENTITY_null;
        pk_call!(PK_REGION_imprint_point(self.tag, point.tag(), &mut v));
        Ok(Vertex::from_tag(v))
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

    /// The body that owns this shell.
    pub fn body(&self) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_SHELL_ask_body(self.tag, &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// The shell's kind (acorn / wireframe / mixed; a normal face-bounded shell
    /// reports [`ShellType::Other`]).
    pub fn shell_type(&self) -> PsResult<ShellType> {
        let mut t: PK_SHELL_type_t = 0;
        pk_call!(PK_SHELL_ask_type(self.tag, &mut t));
        Ok(ShellType::from_raw(t))
    }

    /// The isolated (acorn) vertex of a vertex-only shell, if any.
    pub fn acorn_vertex(&self) -> PsResult<Option<Vertex>> {
        let mut tag: PK_VERTEX_t = PK_ENTITY_null;
        pk_call!(PK_SHELL_ask_acorn_vertex(self.tag, &mut tag));
        Ok((tag != PK_ENTITY_null).then(|| Vertex::from_tag(tag)))
    }

    /// The dangling wireframe edges of this shell.
    pub fn wireframe_edges(&self) -> PsResult<Vec<Edge>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_SHELL_ask_wireframe_edges(self.tag, &mut n, &mut ptr));
        Ok(collect(ptr, n, Edge::from_tag))
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

    /// The edges bounding this loop (directly, without walking fins).
    pub fn edges(&self) -> PsResult<Vec<Edge>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_LOOP_ask_edges(self.tag, &mut n, &mut ptr));
        Ok(collect(ptr, n, Edge::from_tag))
    }

    /// The vertices around this loop.
    pub fn vertices(&self) -> PsResult<Vec<Vertex>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_LOOP_ask_vertices(self.tag, &mut n, &mut ptr));
        Ok(collect(ptr, n, Vertex::from_tag))
    }

    /// The first fin of this loop (entry point for fin-cycle walking).
    pub fn first_fin(&self) -> PsResult<Fin> {
        let mut tag: PK_FIN_t = PK_ENTITY_null;
        pk_call!(PK_LOOP_ask_first_fin(self.tag, &mut tag));
        Ok(Fin::from_tag(tag))
    }

    /// The next loop in the owning face's loop list, or `None` at the end.
    pub fn next_in_face(&self) -> PsResult<Option<Loop>> {
        let mut tag: PK_LOOP_t = PK_ENTITY_null;
        pk_call!(PK_LOOP_ask_next_in_face(self.tag, &mut tag));
        Ok((tag != PK_ENTITY_null).then(|| Loop::from_tag(tag)))
    }

    /// The body that owns this loop.
    pub fn body(&self) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_LOOP_ask_body(self.tag, &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// Whether this is an isolated (vertex/wire) loop with no bounding edges.
    pub fn is_isolated(&self) -> PsResult<bool> {
        let mut is_iso: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_LOOP_is_isolated(self.tag, &mut is_iso));
        Ok(is_iso == PK_LOGICAL_true)
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

    /// The body that owns this fin.
    pub fn body(&self) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_body(self.tag, &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// The fin type (wire / biwire / normal).
    pub fn fin_type(&self) -> PsResult<FinType> {
        let mut t: PK_FIN_type_t = 0;
        pk_call!(PK_FIN_ask_type(self.tag, &mut t));
        Ok(FinType::from_raw(t))
    }

    /// The curve carrying this fin.
    pub fn curve(&self) -> PsResult<Curve> {
        let mut tag: PK_CURVE_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_curve(self.tag, &mut tag));
        Ok(Curve::from_tag(tag))
    }

    /// The curve carrying this fin plus the fin's sense relative to it.
    pub fn oriented_curve(&self) -> PsResult<(Curve, bool)> {
        let mut tag: PK_CURVE_t = PK_ENTITY_null;
        let mut orient: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_FIN_ask_oriented_curve(self.tag, &mut tag, &mut orient));
        Ok((Curve::from_tag(tag), orient == PK_LOGICAL_true))
    }

    /// Whether the fin runs in the same direction as its edge.
    pub fn is_positive(&self) -> PsResult<bool> {
        let mut pos: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_FIN_is_positive(self.tag, &mut pos));
        Ok(pos == PK_LOGICAL_true)
    }

    /// The next fin in the radial (around-edge) ring — for non-manifold edges.
    pub fn next_of_edge(&self) -> PsResult<Fin> {
        let mut tag: PK_FIN_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_next_of_edge(self.tag, &mut tag));
        Ok(Fin::from_tag(tag))
    }

    /// The previous fin in the radial (around-edge) ring.
    pub fn previous_of_edge(&self) -> PsResult<Fin> {
        let mut tag: PK_FIN_t = PK_ENTITY_null;
        pk_call!(PK_FIN_ask_previous_of_edge(self.tag, &mut tag));
        Ok(Fin::from_tag(tag))
    }

    /// The fin's parametric interval on its curve, as `(t_min, t_max)`.
    pub fn interval(&self) -> PsResult<(f64, f64)> {
        let mut iv = PK_INTERVAL_t { low: 0.0, high: 0.0 };
        pk_call!(PK_FIN_find_interval(self.tag, &mut iv));
        Ok((iv.low, iv.high))
    }

    /// The fin's bounding box in the owning face's UV space.
    pub fn uvbox(&self) -> PsResult<crate::UvBox> {
        let mut b = PK_UVBOX_t { param: [0.0; 4] };
        pk_call!(PK_FIN_find_uvbox(self.tag, &mut b));
        Ok(crate::UvBox { u_min: b.param[0], v_min: b.param[1], u_max: b.param[2], v_max: b.param[3] })
    }

    /// Map a curve parameter `t` on this fin to the owning face's `(u, v)`
    /// surface parameters (the SP-curve → surface map).
    pub fn surf_params(&self, t: f64) -> PsResult<(f64, f64)> {
        let est = PK_UV_t::default();
        let mut parms = PK_UV_t::default();
        pk_call!(PK_FIN_find_surf_parameters(self.tag, t, PK_LOGICAL_false, &est, &mut parms));
        Ok((parms[0], parms[1]))
    }

    /// Inverse of [`surf_params`](Self::surf_params): map face `(u, v)` back to
    /// this fin's curve parameter `t`.
    pub fn curve_param(&self, uv: (f64, f64)) -> PsResult<f64> {
        let parms: PK_UV_t = [uv.0, uv.1];
        let mut t = 0.0f64;
        pk_call!(PK_FIN_find_curve_parameter(self.tag, &parms, PK_LOGICAL_false, 0.0, &mut t));
        Ok(t)
    }
}
