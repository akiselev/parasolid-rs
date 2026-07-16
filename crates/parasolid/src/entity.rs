//! Base entity handle and PK class enumeration.
//!
//! All Parasolid model objects (bodies, faces, edges, surfaces, curves, etc.)
//! are identified by integer tags. [`Entity`] is a lightweight wrapper around
//! these tags that provides class queries and validity checks.

use std::fmt;

use parasolid_sys::*;

use crate::error::PsResult;
use crate::geom::Vec3;
use crate::memory::PkArray;

/// Result of a minimum-distance query: the gap and the closest points on each
/// side (`point_1` on this entity, `point_2` on the other entity or the query
/// point).
#[derive(Debug, Clone, Copy)]
pub struct RangeResult {
    pub distance: f64,
    pub point_1: Vec3,
    pub point_2: Vec3,
}

/// Oriented (non-axis-aligned) bounding box: three orthonormal basis axes plus
/// the min/max extents expressed in that frame.
#[derive(Debug, Clone, Copy)]
pub struct Obb {
    pub basis: [Vec3; 3],
    pub min: Vec3,
    pub max: Vec3,
}

impl Obb {
    /// Extents (max − min) along the three basis axes.
    pub fn extents(&self) -> [f64; 3] {
        [self.max.x - self.min.x, self.max.y - self.min.y, self.max.z - self.min.z]
    }
}

/// Category of the geometry attached to a topology (`PK_GEOM_category_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeomCategory {
    /// Analytic + spline (planes, cylinders, B-surfaces, …).
    Classic,
    /// Facet / mesh geometry.
    Facet,
    /// No geometry attached.
    None,
    Point,
    /// Mixed classic + facet.
    Mixed,
    Lattice,
    Other(i32),
}

impl GeomCategory {
    fn from_raw(v: i32) -> Self {
        match v {
            25870 => GeomCategory::Classic,
            25871 => GeomCategory::Facet,
            25872 => GeomCategory::None,
            25873 => GeomCategory::Point,
            25874 => GeomCategory::Mixed,
            25875 => GeomCategory::Lattice,
            other => GeomCategory::Other(other),
        }
    }
}

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
        pk_call!(PK_ENTITY_delete(1, &self.tag));
        Ok(())
    }

    /// Create a copy of this entity.
    pub fn copy(&self) -> PsResult<Entity> {
        let mut copy_tag: PK_ENTITY_t = PK_ENTITY_null;
        pk_call!(PK_ENTITY_copy(self.tag, &mut copy_tag));
        Ok(Entity::from_tag(copy_tag))
    }

    /// Minimum distance between this (topological) entity and `other`, with the
    /// closest point on each. Wraps `PK_TOPOL_range` (options defaulted).
    pub fn distance_to(&self, other: Entity) -> PsResult<RangeResult> {
        let mut opts = PK_TOPOL_range_o_t::default();
        let mut status: PK_range_result_t = 0;
        let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_TOPOL_range(
            self.tag,
            other.tag,
            &mut opts,
            &mut status,
            &mut r,
        ));
        Ok(RangeResult {
            distance: r.distance,
            point_1: Vec3::from_pk(r.end_1.position),
            point_2: Vec3::from_pk(r.end_2.position),
        })
    }

    /// Minimum distance from this entity to a point, with the closest point on
    /// the entity. Wraps `PK_TOPOL_range_vector` (position by pointer).
    pub fn distance_to_point(&self, point: Vec3) -> PsResult<RangeResult> {
        let v: PK_VECTOR_t = point.to_pk();
        let mut opts = PK_TOPOL_range_vector_o_t::default();
        let mut status: PK_range_result_t = 0;
        let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_TOPOL_range_vector(
            self.tag,
            &v,
            &mut opts,
            &mut status,
            &mut r,
        ));
        Ok(RangeResult {
            distance: r.distance,
            point_1: Vec3::from_pk(r.end.position),
            point_2: point,
        })
    }

    /// The category of the geometry attached to this topology (analytic/classic
    /// vs facet vs point/lattice). Wraps `PK_TOPOL_categorise_geom` with default
    /// options.
    pub fn geom_category(&self) -> PsResult<GeomCategory> {
        let mut overall: PK_GEOM_category_t = 0;
        let mut direct: PK_GEOM_category_t = 0;
        let mut n_related: std::os::raw::c_int = 0;
        let mut related_topols: *mut PK_TOPOL_t = std::ptr::null_mut();
        let mut related_cats: *mut PK_GEOM_category_t = std::ptr::null_mut();
        pk_call!(PK_TOPOL_categorise_geom(
            self.tag,
            std::ptr::null(),
            &mut overall,
            &mut direct,
            &mut n_related,
            &mut related_topols,
            &mut related_cats,
        ));
        unsafe {
            if !related_topols.is_null() {
                let _ = PkArray::from_raw(related_topols, n_related);
            }
            if !related_cats.is_null() {
                let _ = PkArray::from_raw(related_cats, n_related);
            }
        }
        Ok(GeomCategory::from_raw(overall))
    }

    /// The optimal oriented (non-axis-aligned) bounding box of this entity.
    /// Wraps `PK_TOPOL_find_nabox` (no axis constraints, standard quality).
    pub fn oriented_bounding_box(&self) -> PsResult<Obb> {
        let mut tag = self.tag;
        let mut opts = PK_TOPOL_find_nabox_o_t::default();
        let mut sf: PK_NABOX_sf_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_TOPOL_find_nabox(1, &mut tag, std::ptr::null_mut(), &mut opts, &mut sf));
        Ok(Obb {
            basis: [
                Vec3::from_pk(sf.basis_set[0]),
                Vec3::from_pk(sf.basis_set[1]),
                Vec3::from_pk(sf.basis_set[2]),
            ],
            min: Vec3::new(sf.coord[0], sf.coord[1], sf.coord[2]),
            max: Vec3::new(sf.coord[3], sf.coord[4], sf.coord[5]),
        })
    }

    /// The stable per-entity identifier (`PK_ENTITY_ask_identifier`). May be 0.
    pub fn identifier(&self) -> PsResult<i32> {
        let mut id: std::os::raw::c_int = 0;
        pk_call!(PK_ENTITY_ask_identifier(self.tag, &mut id));
        Ok(id)
    }

    /// Read this entity's user field. The returned vector has length equal to the
    /// session's user-field length; it is empty when the session was started with
    /// `user_field_len = 0`.
    pub fn user_field(&self) -> PsResult<Vec<i32>> {
        let mut len: std::os::raw::c_int = 0;
        pk_call!(PK_SESSION_ask_user_field_len(&mut len));
        let n = len.max(0) as usize;
        let mut buf = vec![0i32; n];
        if n > 0 {
            pk_call!(PK_ENTITY_ask_user_field(self.tag, buf.as_mut_ptr()));
        }
        Ok(buf)
    }

    /// Set this entity's user field (`data` must hold at least the session's
    /// user-field length ints).
    pub fn set_user_field(&self, data: &[i32]) -> PsResult<()> {
        pk_call!(PK_ENTITY_set_user_field(self.tag, data.as_ptr()));
        Ok(())
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
