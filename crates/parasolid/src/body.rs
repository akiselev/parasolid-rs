//! Body type — the top-level topological entity in Parasolid.

use std::os::raw::c_int;
use parasolid_sys::*;
use crate::error::PsResult;
use crate::memory::PkArray;
use crate::entity::Entity;
use crate::face::Face;
use crate::edge::Edge;
use crate::vertex::Vertex;

/// The type of a body (dimensionality).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyType {
    Empty,
    Acorn,
    Wire,
    Sheet,
    Solid,
    General,
}

/// A Parasolid body — owns topology (faces, edges, vertices).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Body {
    pub(crate) tag: PK_BODY_t,
}

impl Body {
    pub(crate) fn from_tag(tag: PK_BODY_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    // --- Queries ---

    pub fn body_type(&self) -> PsResult<BodyType> {
        let mut btype: PK_BODY_type_t = 0;
        pk_call!(PK_BODY_ask_type(self.tag, &mut btype));
        Ok(match btype {
            PK_BODY_type_empty_c => BodyType::Empty,
            PK_BODY_type_acorn_c => BodyType::Acorn,
            PK_BODY_type_wire_c => BodyType::Wire,
            PK_BODY_type_sheet_c => BodyType::Sheet,
            PK_BODY_type_solid_c => BodyType::Solid,
            _ => BodyType::General,
        })
    }

    pub fn faces(&self) -> PsResult<Vec<Face>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_BODY_ask_faces(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Face::from_tag(tag)).collect())
    }

    pub fn edges(&self) -> PsResult<Vec<Edge>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_BODY_ask_edges(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Edge::from_tag(tag)).collect())
    }

    pub fn vertices(&self) -> PsResult<Vec<Vertex>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_BODY_ask_vertices(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Vertex::from_tag(tag)).collect())
    }

    // --- Primitive creation ---

    /// Create an axis-aligned solid block whose base is centred at the origin
    /// (x spans ±x/2, y spans ±y/2, z spans 0..z — per PK_BODY_create_solid_block docs).
    pub fn create_solid_block(x: f64, y: f64, z: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_block(x, y, z, std::ptr::null(), &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// Create a solid cylinder along the Z axis, centered at the origin.
    pub fn create_solid_cylinder(radius: f64, height: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_cyl(radius, height, std::ptr::null(), &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// Create a solid sphere centered at the origin.
    pub fn create_solid_sphere(radius: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_sphere(radius, std::ptr::null(), &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// Create a solid cone along the Z axis, base on the z=0 plane.
    ///
    /// The cone is defined by its base `radius` (at the z=0 plane), `height`,
    /// and `semi_angle` (the half-angle, in radians). The kernel's cone widens
    /// toward +z: the top radius at `height` is `radius + height *
    /// tan(semi_angle)` (probed against the DLL). Pass `semi_angle = 0` for a
    /// plain cylinder-like extrusion; a negative `semi_angle` narrows it.
    pub fn create_solid_cone(radius: f64, height: f64, semi_angle: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_cone(radius, height, semi_angle, std::ptr::null(), &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// Create a solid torus centered at the origin, major axis along Z.
    pub fn create_solid_torus(major_radius: f64, minor_radius: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_torus(major_radius, minor_radius, std::ptr::null(), &mut tag));
        Ok(Body::from_tag(tag))
    }

    // --- Boolean operations ---

    /// Unite this body with one or more tool bodies.
    ///
    /// Consumes `self` and the tool bodies. Returns all resulting bodies.
    pub fn unite(self, tools: Vec<Body>) -> PsResult<Vec<Body>> {
        crate::boolean::boolean(self, tools, crate::boolean::BooleanOp::Unite, &crate::boolean::BooleanOptions::default())
    }

    /// Subtract tool bodies from this body.
    ///
    /// Consumes `self` and the tool bodies. Returns all resulting bodies.
    pub fn subtract(self, tools: Vec<Body>) -> PsResult<Vec<Body>> {
        crate::boolean::boolean(self, tools, crate::boolean::BooleanOp::Subtract, &crate::boolean::BooleanOptions::default())
    }

    /// Intersect this body with tool bodies.
    ///
    /// Consumes `self` and the tool bodies. Returns all resulting bodies.
    pub fn intersect(self, tools: Vec<Body>) -> PsResult<Vec<Body>> {
        crate::boolean::boolean(self, tools, crate::boolean::BooleanOp::Intersect, &crate::boolean::BooleanOptions::default())
    }

    // --- Lifecycle ---

    pub fn delete(self) -> PsResult<()> {
        self.entity().delete()
    }

    pub fn copy(&self) -> PsResult<Body> {
        let copied = self.entity().copy()?;
        Ok(Body::from_tag(copied.tag()))
    }
}
