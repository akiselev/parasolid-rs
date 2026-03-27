//! Base entity handle and PK class enumeration.
//!
//! All Parasolid model objects (bodies, faces, edges, surfaces, curves, etc.)
//! are identified by integer tags. [`Entity`] is a lightweight wrapper around
//! these tags that provides class queries and validity checks.

use std::fmt;

use parasolid_sys::*;

use crate::error::PsResult;

// =============================================================================
// PkClass
// =============================================================================

/// Parasolid entity class identifiers.
///
/// Mirrors the `PK_CLASS_*` constants from the C API. The class hierarchy is:
///
/// ```text
/// ENTITY
///   ├── TOPOL
///   │   ├── BODY, REGION, SHELL, FACE, LOOP, FIN, EDGE, VERTEX
///   │   ├── ASSEMBLY, INSTANCE
///   │   └── (PART = BODY | ASSEMBLY)
///   ├── GEOM
///   │   ├── SURF: PLANE, CYL, CONE, SPHERE, TORUS, BSURF, OFFSET, SWEPT, SPUN, FSURF, MESH, BLENDSF, SSURF
///   │   ├── CURVE: LINE, CIRCLE, ELLIPSE, BCURVE, ICURVE, FCURVE, SCURVE, TCURVE, CPCURVE, PLINE
///   │   └── POINT
///   ├── ATTRIB, ATTDEF, GROUP
///   └── TRANSF
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PkClass {
    // Topological
    Body,
    Region,
    Shell,
    Face,
    Loop,
    Fin,
    Edge,
    Vertex,

    // Surfaces
    Surf,
    Plane,
    Cyl,
    Cone,
    Sphere,
    Torus,
    Bsurf,
    Offset,
    Swept,
    Spun,
    Fsurf,
    Mesh,
    Blendsf,
    Ssurf,

    // Curves
    Curve,
    Line,
    Circle,
    Ellipse,
    Bcurve,
    Icurve,
    Fcurve,
    Scurve,
    Tcurve,
    Cpcurve,
    Pline,

    // Other
    Point,
    Transf,
    Assembly,
    Instance,
    Attrib,
    Attdef,
    Group,
    Partition,
    Mark,
    Pmark,
    Delta,
    Blend,

    // Convergent modeling
    Mfacet,
    Mfin,
    Mvertex,
    Mtopol,

    /// Unknown class ID not mapped to a named variant.
    Unknown(i32),
}

impl PkClass {
    /// Convert a raw `PK_CLASS_t` value to a `PkClass`.
    pub fn from_raw(raw: PK_CLASS_t) -> Self {
        match raw {
            PK_CLASS_body => PkClass::Body,
            PK_CLASS_region => PkClass::Region,
            PK_CLASS_shell => PkClass::Shell,
            PK_CLASS_face => PkClass::Face,
            PK_CLASS_loop => PkClass::Loop,
            PK_CLASS_fin => PkClass::Fin,
            PK_CLASS_edge => PkClass::Edge,
            PK_CLASS_vertex => PkClass::Vertex,
            PK_CLASS_surf => PkClass::Surf,
            PK_CLASS_plane => PkClass::Plane,
            PK_CLASS_cyl => PkClass::Cyl,
            PK_CLASS_cone => PkClass::Cone,
            PK_CLASS_sphere => PkClass::Sphere,
            PK_CLASS_torus => PkClass::Torus,
            PK_CLASS_bsurf => PkClass::Bsurf,
            PK_CLASS_offset => PkClass::Offset,
            PK_CLASS_swept => PkClass::Swept,
            PK_CLASS_spun => PkClass::Spun,
            PK_CLASS_fsurf => PkClass::Fsurf,
            PK_CLASS_mesh => PkClass::Mesh,
            PK_CLASS_blendsf => PkClass::Blendsf,
            PK_CLASS_ssurf => PkClass::Ssurf,
            PK_CLASS_curve => PkClass::Curve,
            PK_CLASS_line => PkClass::Line,
            PK_CLASS_circle => PkClass::Circle,
            PK_CLASS_ellipse => PkClass::Ellipse,
            PK_CLASS_bcurve => PkClass::Bcurve,
            PK_CLASS_icurve => PkClass::Icurve,
            PK_CLASS_fcurve => PkClass::Fcurve,
            PK_CLASS_scurve => PkClass::Scurve,
            PK_CLASS_tcurve => PkClass::Tcurve,
            PK_CLASS_cpcurve => PkClass::Cpcurve,
            PK_CLASS_pline => PkClass::Pline,
            PK_CLASS_point => PkClass::Point,
            PK_CLASS_transf => PkClass::Transf,
            PK_CLASS_assembly => PkClass::Assembly,
            PK_CLASS_instance => PkClass::Instance,
            PK_CLASS_attrib => PkClass::Attrib,
            PK_CLASS_attdef => PkClass::Attdef,
            PK_CLASS_group => PkClass::Group,
            PK_CLASS_partition => PkClass::Partition,
            PK_CLASS_mark => PkClass::Mark,
            PK_CLASS_pmark => PkClass::Pmark,
            PK_CLASS_delta => PkClass::Delta,
            PK_CLASS_blend => PkClass::Blend,
            PK_CLASS_mfacet => PkClass::Mfacet,
            PK_CLASS_mfin => PkClass::Mfin,
            PK_CLASS_mvertex => PkClass::Mvertex,
            PK_CLASS_mtopol => PkClass::Mtopol,
            other => PkClass::Unknown(other),
        }
    }

    /// Convert to the raw `PK_CLASS_t` value.
    pub fn to_raw(self) -> PK_CLASS_t {
        match self {
            PkClass::Body => PK_CLASS_body,
            PkClass::Region => PK_CLASS_region,
            PkClass::Shell => PK_CLASS_shell,
            PkClass::Face => PK_CLASS_face,
            PkClass::Loop => PK_CLASS_loop,
            PkClass::Fin => PK_CLASS_fin,
            PkClass::Edge => PK_CLASS_edge,
            PkClass::Vertex => PK_CLASS_vertex,
            PkClass::Surf => PK_CLASS_surf,
            PkClass::Plane => PK_CLASS_plane,
            PkClass::Cyl => PK_CLASS_cyl,
            PkClass::Cone => PK_CLASS_cone,
            PkClass::Sphere => PK_CLASS_sphere,
            PkClass::Torus => PK_CLASS_torus,
            PkClass::Bsurf => PK_CLASS_bsurf,
            PkClass::Offset => PK_CLASS_offset,
            PkClass::Swept => PK_CLASS_swept,
            PkClass::Spun => PK_CLASS_spun,
            PkClass::Fsurf => PK_CLASS_fsurf,
            PkClass::Mesh => PK_CLASS_mesh,
            PkClass::Blendsf => PK_CLASS_blendsf,
            PkClass::Ssurf => PK_CLASS_ssurf,
            PkClass::Curve => PK_CLASS_curve,
            PkClass::Line => PK_CLASS_line,
            PkClass::Circle => PK_CLASS_circle,
            PkClass::Ellipse => PK_CLASS_ellipse,
            PkClass::Bcurve => PK_CLASS_bcurve,
            PkClass::Icurve => PK_CLASS_icurve,
            PkClass::Fcurve => PK_CLASS_fcurve,
            PkClass::Scurve => PK_CLASS_scurve,
            PkClass::Tcurve => PK_CLASS_tcurve,
            PkClass::Cpcurve => PK_CLASS_cpcurve,
            PkClass::Pline => PK_CLASS_pline,
            PkClass::Point => PK_CLASS_point,
            PkClass::Transf => PK_CLASS_transf,
            PkClass::Assembly => PK_CLASS_assembly,
            PkClass::Instance => PK_CLASS_instance,
            PkClass::Attrib => PK_CLASS_attrib,
            PkClass::Attdef => PK_CLASS_attdef,
            PkClass::Group => PK_CLASS_group,
            PkClass::Partition => PK_CLASS_partition,
            PkClass::Mark => PK_CLASS_mark,
            PkClass::Pmark => PK_CLASS_pmark,
            PkClass::Delta => PK_CLASS_delta,
            PkClass::Blend => PK_CLASS_blend,
            PkClass::Mfacet => PK_CLASS_mfacet,
            PkClass::Mfin => PK_CLASS_mfin,
            PkClass::Mvertex => PK_CLASS_mvertex,
            PkClass::Mtopol => PK_CLASS_mtopol,
            PkClass::Unknown(v) => v,
        }
    }

    /// Returns `true` if this is a topological class (body, face, edge, etc.).
    pub fn is_topol(&self) -> bool {
        matches!(
            self,
            PkClass::Body
                | PkClass::Region
                | PkClass::Shell
                | PkClass::Face
                | PkClass::Loop
                | PkClass::Fin
                | PkClass::Edge
                | PkClass::Vertex
                | PkClass::Assembly
                | PkClass::Instance
        )
    }

    /// Returns `true` if this is a geometric class (surface, curve, point).
    pub fn is_geom(&self) -> bool {
        self.is_surf() || self.is_curve() || matches!(self, PkClass::Point)
    }

    /// Returns `true` if this is a surface class.
    pub fn is_surf(&self) -> bool {
        matches!(
            self,
            PkClass::Surf
                | PkClass::Plane
                | PkClass::Cyl
                | PkClass::Cone
                | PkClass::Sphere
                | PkClass::Torus
                | PkClass::Bsurf
                | PkClass::Offset
                | PkClass::Swept
                | PkClass::Spun
                | PkClass::Fsurf
                | PkClass::Mesh
                | PkClass::Blendsf
                | PkClass::Ssurf
        )
    }

    /// Returns `true` if this is a curve class.
    pub fn is_curve(&self) -> bool {
        matches!(
            self,
            PkClass::Curve
                | PkClass::Line
                | PkClass::Circle
                | PkClass::Ellipse
                | PkClass::Bcurve
                | PkClass::Icurve
                | PkClass::Fcurve
                | PkClass::Scurve
                | PkClass::Tcurve
                | PkClass::Cpcurve
                | PkClass::Pline
        )
    }
}

impl fmt::Display for PkClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PkClass::Body => f.write_str("body"),
            PkClass::Region => f.write_str("region"),
            PkClass::Shell => f.write_str("shell"),
            PkClass::Face => f.write_str("face"),
            PkClass::Loop => f.write_str("loop"),
            PkClass::Fin => f.write_str("fin"),
            PkClass::Edge => f.write_str("edge"),
            PkClass::Vertex => f.write_str("vertex"),
            PkClass::Surf => f.write_str("surf"),
            PkClass::Plane => f.write_str("plane"),
            PkClass::Cyl => f.write_str("cyl"),
            PkClass::Cone => f.write_str("cone"),
            PkClass::Sphere => f.write_str("sphere"),
            PkClass::Torus => f.write_str("torus"),
            PkClass::Bsurf => f.write_str("bsurf"),
            PkClass::Offset => f.write_str("offset"),
            PkClass::Swept => f.write_str("swept"),
            PkClass::Spun => f.write_str("spun"),
            PkClass::Fsurf => f.write_str("fsurf"),
            PkClass::Mesh => f.write_str("mesh"),
            PkClass::Blendsf => f.write_str("blendsf"),
            PkClass::Ssurf => f.write_str("ssurf"),
            PkClass::Curve => f.write_str("curve"),
            PkClass::Line => f.write_str("line"),
            PkClass::Circle => f.write_str("circle"),
            PkClass::Ellipse => f.write_str("ellipse"),
            PkClass::Bcurve => f.write_str("bcurve"),
            PkClass::Icurve => f.write_str("icurve"),
            PkClass::Fcurve => f.write_str("fcurve"),
            PkClass::Scurve => f.write_str("scurve"),
            PkClass::Tcurve => f.write_str("tcurve"),
            PkClass::Cpcurve => f.write_str("cpcurve"),
            PkClass::Pline => f.write_str("pline"),
            PkClass::Point => f.write_str("point"),
            PkClass::Transf => f.write_str("transf"),
            PkClass::Assembly => f.write_str("assembly"),
            PkClass::Instance => f.write_str("instance"),
            PkClass::Attrib => f.write_str("attrib"),
            PkClass::Attdef => f.write_str("attdef"),
            PkClass::Group => f.write_str("group"),
            PkClass::Partition => f.write_str("partition"),
            PkClass::Mark => f.write_str("mark"),
            PkClass::Pmark => f.write_str("pmark"),
            PkClass::Delta => f.write_str("delta"),
            PkClass::Blend => f.write_str("blend"),
            PkClass::Mfacet => f.write_str("mfacet"),
            PkClass::Mfin => f.write_str("mfin"),
            PkClass::Mvertex => f.write_str("mvertex"),
            PkClass::Mtopol => f.write_str("mtopol"),
            PkClass::Unknown(v) => write!(f, "unknown({v})"),
        }
    }
}

// =============================================================================
// Entity
// =============================================================================

/// A lightweight handle to any Parasolid entity.
///
/// Entity tags are opaque integer identifiers created by the Parasolid kernel.
/// They are session-unique but not persistent across sessions.
///
/// `Entity` is `Copy` — it's just an integer. The tag may become invalid
/// (dead) if the entity is deleted, rolled back, or the session stops. Use
/// [`is_valid`](Entity::is_valid) to check.
///
/// # Design note
///
/// Entity carries no session lifetime (`'s`). This departs from the design in
/// `docs/api/05-rust-wrapper-patterns.md` Pattern C. Rationale: lifetime
/// threading through `Vec<Entity>` return values was deemed too ergonomically
/// burdensome for v0.1. Mitigation: [`is_valid`](Entity::is_valid) is
/// available; argument checking should be enabled in dev builds via
/// [`SessionConfig::check_arguments`]. Revisit before v1.0.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    tag: PK_ENTITY_t,
}

impl Entity {
    /// Wrap a raw PK entity tag.
    pub(crate) fn from_tag(tag: PK_ENTITY_t) -> Self {
        Entity { tag }
    }

    /// Returns the raw PK entity tag.
    ///
    /// This is an escape hatch for use with lower-level `parasolid_sys` APIs
    /// (e.g., `PK_THREAD_*` multi-threading functions) that are not yet
    /// wrapped by this crate. Prefer the typed methods on this struct.
    ///
    /// The returned tag is only valid while this entity is alive in the
    /// current session. Do not store the raw value beyond the entity's or
    /// session's lifetime.
    #[inline]
    pub fn tag(&self) -> i32 {
        self.tag
    }

    /// Returns `true` if this is the null entity (`PK_ENTITY_null`).
    #[inline]
    pub fn is_null(&self) -> bool {
        self.tag == PK_ENTITY_null
    }

    /// Check whether this tag refers to a living entity in the current session.
    ///
    /// Returns `false` if the entity has been deleted, rolled back, or the
    /// tag was never valid.
    pub fn is_valid(&self) -> PsResult<bool> {
        let mut result: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_ENTITY_is(self.tag, &mut result));
        Ok(result == PK_LOGICAL_true)
    }

    /// Returns the class of this entity.
    pub fn class(&self) -> PsResult<PkClass> {
        let mut raw: PK_CLASS_t = 0;
        pk_call!(PK_ENTITY_ask_class(self.tag, &mut raw));
        Ok(PkClass::from_raw(raw))
    }

    /// Check whether this entity is topological (body, face, edge, etc.).
    pub fn is_topol(&self) -> PsResult<bool> {
        let mut result: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_ENTITY_is_topol(self.tag, &mut result));
        Ok(result == PK_LOGICAL_true)
    }

    /// Check whether this entity is geometric (surface, curve, point).
    pub fn is_geom(&self) -> PsResult<bool> {
        let mut result: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_ENTITY_is_geom(self.tag, &mut result));
        Ok(result == PK_LOGICAL_true)
    }

    /// Check whether this entity is a surface.
    pub fn is_surf(&self) -> PsResult<bool> {
        let mut result: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_ENTITY_is_surf(self.tag, &mut result));
        Ok(result == PK_LOGICAL_true)
    }

    /// Check whether this entity is a curve.
    pub fn is_curve(&self) -> PsResult<bool> {
        let mut result: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_ENTITY_is_curve(self.tag, &mut result));
        Ok(result == PK_LOGICAL_true)
    }

    /// Check whether this entity is a part (body or assembly).
    pub fn is_part(&self) -> PsResult<bool> {
        let mut result: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_ENTITY_is_part(self.tag, &mut result));
        Ok(result == PK_LOGICAL_true)
    }

    /// Delete this entity from the session.
    ///
    /// After deletion, the tag becomes invalid and should not be used.
    pub fn delete(self) -> PsResult<()> {
        pk_call!(PK_ENTITY_delete(self.tag));
        Ok(())
    }

    /// Create a copy of this entity.
    pub fn copy(&self) -> PsResult<Entity> {
        let mut copy_tag: PK_ENTITY_t = PK_ENTITY_null;
        pk_call!(PK_ENTITY_copy(self.tag, &mut copy_tag));
        Ok(Entity::from_tag(copy_tag))
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entity({})", self.tag)
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "entity#{}", self.tag)
    }
}
