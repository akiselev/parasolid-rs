//! Body type — the top-level topological entity in Parasolid.

use crate::edge::Edge;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::face::Face;
use crate::memory::PkArray;
use crate::vertex::Vertex;
use parasolid_sys::*;
use std::os::raw::c_int;

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
    pub(crate) fn from_tag(tag: PK_BODY_t) -> Self {
        Self { tag }
    }
    pub fn tag(&self) -> i32 {
        self.tag
    }
    pub fn entity(&self) -> Entity {
        Entity::from_tag(self.tag)
    }

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

    /// Return all shells of this body.
    pub fn shells(&self) -> PsResult<Vec<crate::Shell>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_BODY_ask_shells(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array
            .iter()
            .map(|&tag| crate::Shell::from_tag(tag))
            .collect())
    }

    /// Return all regions of this body (solid material and surrounding void).
    pub fn regions(&self) -> PsResult<Vec<crate::Region>> {
        let mut n: c_int = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_BODY_ask_regions(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array
            .iter()
            .map(|&tag| crate::Region::from_tag(tag))
            .collect())
    }

    // --- Primitive creation ---

    /// Create an axis-aligned solid block whose base is centred at the origin
    /// (x spans ±x/2, y spans ±y/2, z spans 0..z — per PK_BODY_create_solid_block docs).
    pub fn create_solid_block(x: f64, y: f64, z: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_block(
            x,
            y,
            z,
            std::ptr::null(),
            &mut tag
        ));
        Ok(Body::from_tag(tag))
    }

    /// Create a minimum (acorn) body — a body consisting of a single isolated
    /// vertex at `pos`. Its lone vertex is an acorn vertex (see
    /// [`Vertex::delete_acorn`](crate::Vertex::delete_acorn)).
    pub fn create_minimum(pos: crate::geom::Vec3) -> PsResult<Body> {
        let point = crate::Point::create(pos)?;
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_POINT_make_minimum_body(point.tag(), &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// Convert this body into a **general** body (Parasolid's most permissive
    /// body kind, which may mix solid/sheet/wire and carry non-manifold and
    /// isolated topology). Copies this body's topology into a new general body
    /// and returns it. Requires general topology to be enabled on the session.
    pub fn make_general(self) -> PsResult<Body> {
        let mut topols = [self.tag];
        let mut new_body: PK_BODY_t = PK_ENTITY_null;
        let mut copy: PK_TOPOL_t = PK_ENTITY_null;
        pk_call!(PK_TOPOL_make_general_body(
            1,
            topols.as_mut_ptr(),
            &mut new_body,
            &mut copy,
        ));
        Ok(Body::from_tag(new_body))
    }

    /// Create a solid cylinder along the +Z axis with its base on the z=0
    /// plane (it spans `z ∈ 0..height`, so the centroid is at `z = height/2`),
    /// centred on the Z axis in x and y.
    pub fn create_solid_cylinder(radius: f64, height: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_cyl(
            radius,
            height,
            std::ptr::null(),
            &mut tag
        ));
        Ok(Body::from_tag(tag))
    }

    /// Create a circular planar **sheet** (a disk) of `radius` in the plane of
    /// `basis` (centre = `basis` origin, normal = `basis` axis). Useful as an
    /// extrusion/revolution profile.
    pub fn create_sheet_circle(radius: f64, basis: crate::geom::Axis2) -> PsResult<Body> {
        let sf = basis.to_pk();
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_sheet_circle(radius, &sf, &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// **Section** this body with a surface (e.g. a plane), keeping both sides.
    /// With `fence = both` the body is split; the resulting bodies are found in
    /// the session (the opaque `PK_section_r_t` isn't read).
    pub fn section_with_surf(&self, surf: &crate::surf::Surf) -> PsResult<()> {
        let opts = PK_BODY_section_o_t::default();
        let mut results: PK_section_r_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_BODY_section_with_surf(
            self.tag,
            surf.tag(),
            &opts,
            &mut results,
        ));
        Ok(())
    }

    /// Return this body's full **topology graph**: every topological entity
    /// (body, shells, faces, loops, edges, fins, vertices) and the number of
    /// parent→child relations between them (`PK_BODY_ask_topology`).
    pub fn ask_topology(&self) -> PsResult<(Vec<Entity>, usize)> {
        let mut n_topols: c_int = 0;
        let mut topols: *mut PK_TOPOL_t = std::ptr::null_mut();
        let mut classes: *mut PK_CLASS_t = std::ptr::null_mut();
        let mut n_relations: c_int = 0;
        let mut parents: *mut c_int = std::ptr::null_mut();
        let mut children: *mut c_int = std::ptr::null_mut();
        let mut senses: *mut PK_TOPOL_sense_t = std::ptr::null_mut();
        pk_call!(PK_BODY_ask_topology(
            self.tag,
            std::ptr::null(), // default options
            &mut n_topols,
            &mut topols,
            &mut classes,
            &mut n_relations,
            &mut parents,
            &mut children,
            &mut senses,
        ));
        let entities: Vec<Entity> = unsafe { PkArray::from_raw(topols, n_topols) }
            .iter()
            .map(|&t| Entity::from_tag(t))
            .collect();
        unsafe {
            let _ = PkArray::from_raw(classes, n_topols);
            let _ = PkArray::from_raw(parents, n_relations);
            let _ = PkArray::from_raw(children, n_relations);
            let _ = PkArray::from_raw(senses, n_relations);
        }
        Ok((entities, n_relations as usize))
    }

    /// **Offset** every face of this body outward (positive) or inward
    /// (negative) by `distance`, modifying it in place. For a convex solid this
    /// uniformly grows/shrinks it (a 20³ block offset +1 → a 22³ block).
    pub fn offset(&self, distance: f64) -> PsResult<()> {
        pk_call!(PK_BODY_offset(self.tag, distance, 1.0e-6, PK_LOGICAL_false));
        Ok(())
    }

    /// Simplify this body's geometry in place where possible
    /// (`PK_BODY_simplify_geom`): B-curves/B-surfaces that are exactly (within
    /// modeller precision) analytic are replaced by the analytic forms —
    /// rational B-curve → non-rational B-curve / line / circle; non-rational
    /// B-curve → line; rational B-surface → non-rational B-surface / plane /
    /// cylinder / cone / sphere / torus; non-rational B-surface → plane.
    ///
    /// `local = false` replaces a B-geometry only when a **single** simpler
    /// geometry covers the entire original; `local = true` also allows partial
    /// (piecewise) replacement, which may put edges/faces that shared one
    /// B-geometry onto different geometries afterwards.
    ///
    /// Topology is unchanged; replaced geometry is deleted. Returns the new
    /// geometry entities.
    pub fn simplify_geom(&self, local: bool) -> PsResult<Vec<Entity>> {
        let mut n_geoms: c_int = 0;
        let mut geoms: *mut PK_GEOM_t = std::ptr::null_mut();
        pk_call!(PK_BODY_simplify_geom(
            self.tag,
            if local {
                PK_LOGICAL_true
            } else {
                PK_LOGICAL_false
            },
            &mut n_geoms,
            &mut geoms,
        ));
        let array = unsafe { PkArray::from_raw(geoms, n_geoms) };
        Ok(array.iter().map(|&t| Entity::from_tag(t)).collect())
    }

    /// **Hollow** (shell) this solid in place, leaving walls of
    /// `wall_thickness`. With no pierced faces this produces a closed shell — a
    /// solid with an internal void — whose material volume is
    /// `outer − inner`. (Uses a negative offset to shell inward.)
    pub fn hollow(&self, wall_thickness: f64) -> PsResult<()> {
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut results_buf = [0u8; 64];
        pk_call!(PK_BODY_hollow_2(
            self.tag,
            -wall_thickness, // shell inward
            1.0e-6,
            std::ptr::null(), // default options (closed hollow, no pierce faces)
            &mut tracking,
            results_buf.as_mut_ptr() as *mut PK_TOPOL_local_r_t,
        ));
        unsafe { PK_TOPOL_track_r_f(&mut tracking) };
        Ok(())
    }

    /// Round (**fillet**) the given edges with a constant-radius rolling-ball
    /// blend, modifying this body in place. Runs the two-phase Parasolid blend
    /// workflow: `PK_EDGE_set_blend_constant` to mark the edges, then
    /// `PK_BODY_fix_blends` to realise them into fillet faces. Returns the number
    /// of blend faces created.
    pub fn fillet_edges(&self, edges: &[Edge], radius: f64) -> PsResult<i32> {
        let edge_tags: Vec<PK_EDGE_t> = edges.iter().map(|e| e.tag()).collect();

        // Phase 1: set a constant blend on the edges. Default options via NULL
        // (the o_t struct nests a large PK_blend_properties_t whose layout the
        // RE catalog mis-sizes; NULL sidesteps it).
        let mut n_set: c_int = 0;
        let mut set_edges: *mut PK_EDGE_t = std::ptr::null_mut();
        pk_call!(PK_EDGE_set_blend_constant(
            edge_tags.len() as c_int,
            edge_tags.as_ptr(),
            radius,
            std::ptr::null(),
            &mut n_set,
            &mut set_edges,
        ));
        unsafe {
            let _ = PkArray::from_raw(set_edges, n_set);
        }

        // Phase 2: fix the blends into faces.
        let mut n_blends: c_int = 0;
        let mut blends: *mut PK_FACE_t = std::ptr::null_mut();
        let mut unders: *mut PK_FACE_array_t = std::ptr::null_mut();
        let mut topols: *mut c_int = std::ptr::null_mut();
        let mut fault: PK_blend_fault_t = 0;
        let mut fault_edge: PK_EDGE_t = PK_ENTITY_null;
        pk_call!(PK_BODY_fix_blends(
            self.tag,
            std::ptr::null(), // default options
            &mut n_blends,
            &mut blends,
            &mut unders,
            &mut topols,
            &mut fault,
            &mut fault_edge,
        ));
        unsafe {
            let _ = PkArray::from_raw(blends, n_blends);
            let _ = PkArray::from_raw(topols, n_blends);
            // `unders` was previously never freed at all. Release the outer
            // array here.
            //
            // KNOWN RESIDUAL LEAK: each element is a `PK_FACE_array_t` that
            // owns an inner face array. `PK_FACE_array_t` is still an opaque
            // stub in parasolid-sys, so freeing the inner arrays would mean
            // guessing its layout — and freeing a wrong pointer is far worse
            // than leaking. Materialise that struct, then free the inner arrays
            // here.
            if !unders.is_null() {
                let _ = PK_MEMORY_free(unders as *mut std::os::raw::c_void);
            }
        }

        // The kernel reports a FAILED blend through `fault`, not through the
        // return code: PK_BODY_fix_blends answers PK_ERROR_no_errors for a
        // geometrically impossible blend and names the offending edge. Reporting
        // that as Ok(0) told the caller "no error, nothing filleted" when the
        // kernel knew exactly which edge defeated it.
        if fault != PK_blend_fault_no_fault_c {
            return Err(crate::error::PsError::Session(format!(
                "fillet failed: blend fault {fault} on edge {fault_edge} \
                 ({n_set} of {} edges marked, {n_blends} blends produced)",
                edge_tags.len()
            )));
        }
        Ok(n_blends)
    }

    /// Create a rectangular planar **sheet** of `x_length` × `y_length` in the
    /// plane of `basis` (centred on the `basis` origin).
    pub fn create_sheet_rectangle(
        x_length: f64,
        y_length: f64,
        basis: crate::geom::Axis2,
    ) -> PsResult<Body> {
        let sf = basis.to_pk();
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_sheet_rectangle(
            x_length, y_length, &sf, &mut tag
        ));
        Ok(Body::from_tag(tag))
    }

    /// Linearly **extrude** this profile body along `path` (a direction × distance
    /// vector), consuming it. A sheet profile yields a solid; a wire profile
    /// yields a sheet. The profile is bounded at distance 0 and `|path|` along
    /// the (unit) path direction.
    pub fn extrude(self, path: crate::geom::Vec3) -> PsResult<Body> {
        let dist = (path.x * path.x + path.y * path.y + path.z * path.z).sqrt();
        let dir: PK_VECTOR1_t = [path.x / dist, path.y / dist, path.z / dist];
        let mk_bound = |distance: f64| PK_BODY_extrude_bound_t {
            bound: PK_bound_distance_c,
            forward: PK_LOGICAL_true,
            distance,
            entity: PK_ENTITY_null,
            nearest: PK_LOGICAL_false,
            nth_division: 0,
            side: PK_bound_side_both_c,
        };
        let opts = PK_BODY_extrude_o_t {
            o_t_version: 1,
            start_bound: mk_bound(0.0),
            end_bound: mk_bound(dist),
            extruded_body: PK_ENTITY_null,
            allow_disjoint: PK_LOGICAL_false,
            consistent_params: PK_PARAM_consistent_unset_c,
            have_pline_angle: PK_LOGICAL_false,
            pline_angle: 0.0,
            keep_as_facet: PK_extrude_keep_as_facet_no_c,
        };
        let mut body: PK_BODY_t = PK_ENTITY_null;
        // Both output structs are written unconditionally (the `results.status`
        // field "is always set"), so NULL faults the kernel. `tracking` is a
        // real struct we free afterwards; `results` (PK_TOPOL_local_r_t) is
        // opaque — back it with a generously-sized zeroed buffer.
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut results_buf = [0u8; 64];
        pk_call!(PK_BODY_extrude(
            self.tag,
            &dir,
            &opts,
            &mut body,
            &mut tracking,
            results_buf.as_mut_ptr() as *mut PK_TOPOL_local_r_t,
        ));
        unsafe { PK_TOPOL_track_r_f(&mut tracking) };
        Ok(Body::from_tag(body))
    }

    /// Create a solid sphere centered at the origin.
    pub fn create_solid_sphere(radius: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_sphere(
            radius,
            std::ptr::null(),
            &mut tag
        ));
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
        pk_call!(PK_BODY_create_solid_cone(
            radius,
            height,
            semi_angle,
            std::ptr::null(),
            &mut tag
        ));
        Ok(Body::from_tag(tag))
    }

    /// Create a solid torus centered at the origin, major axis along Z.
    pub fn create_solid_torus(major_radius: f64, minor_radius: f64) -> PsResult<Body> {
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_create_solid_torus(
            major_radius,
            minor_radius,
            std::ptr::null(),
            &mut tag
        ));
        Ok(Body::from_tag(tag))
    }

    // --- Boolean operations ---

    /// Unite this body with one or more tool bodies.
    ///
    /// Consumes `self` and the tool bodies. Returns all resulting bodies.
    /// Imprint an (unbounded) plane's cut trace onto this body, splitting the
    /// faces it crosses along a new loop of edges — **without** removing any
    /// material (arrangement only). Returns the newly created edges. The plane is
    /// given as a placement (`origin` on the plane, `axis` = plane normal).
    /// Wraps `PK_BODY_imprint_plane`.
    pub fn imprint_plane(&self, plane: crate::geom::Axis2, tol: f64) -> PsResult<Vec<Edge>> {
        let surf = crate::surf::Surf::plane(plane)?;
        let mut n: c_int = 0;
        let mut ptr: *mut PK_EDGE_t = std::ptr::null_mut();
        pk_call!(PK_BODY_imprint_plane(
            self.tag,
            surf.tag(),
            tol,
            &mut n,
            &mut ptr
        ));
        let arr = unsafe { PkArray::from_raw(ptr, n) };
        Ok(arr.iter().map(|&t| Edge::from_tag(t)).collect())
    }

    /// Mutual imprint of this (target) body and `tool` where their boundaries
    /// intersect: records the intersection as shared edges / split faces on this
    /// body (arrangement only — no material change; `tool` is left unmodified).
    /// The created-topology graph comes back through an opaque result struct, so
    /// this mutates in place — observe the effect via the body's topology counts.
    /// Wraps `PK_BODY_imprint_body`.
    pub fn imprint_body(&self, tool: &Body) -> PsResult<()> {
        let mut opts = PK_BODY_imprint_o_t::default();
        // PK_imprint_r_t (64 B) is filled unconditionally — pass a real buffer.
        let mut results: PK_imprint_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe { PK_BODY_imprint_body(self.tag, tool.tag, &mut opts, &mut results) };
        unsafe {
            let _ = PK_imprint_r_f(&mut results);
        }
        crate::error::pk_check(rc)?;
        Ok(())
    }

    /// **Revolve** (spin) this wire/sheet profile about an axis, turning it into a
    /// higher-dimensional body in place (a sheet region → a solid of revolution).
    /// `angle` is in radians (`TAU` for a full revolution). Consumes `self` and
    /// returns the same tag, now revolved. Wraps `PK_BODY_spin`.
    pub fn spin(
        self,
        axis_origin: crate::geom::Vec3,
        axis_dir: crate::geom::Vec3,
        angle: f64,
    ) -> PsResult<Body> {
        let mut axis = PK_AXIS1_sf_t {
            location: axis_origin.to_pk(),
            axis: axis_dir.to_pk(),
        };
        let mut n_laterals: c_int = 0;
        let mut laterals: *mut PK_TOPOL_t = std::ptr::null_mut();
        let mut bases: *mut PK_TOPOL_t = std::ptr::null_mut();
        let mut check: PK_local_check_t = 0;
        pk_call!(PK_BODY_spin(
            self.tag,
            &mut axis,
            angle,
            PK_LOGICAL_false,
            &mut n_laterals,
            &mut laterals,
            &mut bases,
            &mut check,
        ));
        unsafe {
            let _ = PkArray::from_raw(laterals, n_laterals);
            let _ = PkArray::from_raw(bases, n_laterals);
        }
        Ok(self)
    }

    /// **Sweep** (translate) this wire/sheet profile along `path`, turning it into
    /// a higher-dimensional body in place (a sheet region → a prism solid). `path`
    /// is the full translation vector (direction × distance). Consumes `self` and
    /// returns the same tag, now swept. Wraps `PK_BODY_sweep`.
    pub fn sweep(self, path: crate::geom::Vec3) -> PsResult<Body> {
        let pathv: PK_VECTOR_t = path.to_pk();
        let mut n_laterals: c_int = 0;
        let mut laterals: *mut PK_TOPOL_t = std::ptr::null_mut();
        let mut bases: *mut PK_TOPOL_t = std::ptr::null_mut();
        let mut check: PK_local_check_t = 0;
        pk_call!(PK_BODY_sweep(
            self.tag,
            &pathv,
            PK_LOGICAL_false,
            &mut n_laterals,
            &mut laterals,
            &mut bases,
            &mut check,
        ));
        unsafe {
            let _ = PkArray::from_raw(laterals, n_laterals);
            let _ = PkArray::from_raw(bases, n_laterals);
        }
        Ok(self)
    }

    pub fn unite(self, tools: Vec<Body>) -> PsResult<Vec<Body>> {
        crate::boolean::boolean(
            self,
            tools,
            crate::boolean::BooleanOp::Unite,
            &crate::boolean::BooleanOptions::default(),
        )
    }

    /// Subtract tool bodies from this body.
    ///
    /// Consumes `self` and the tool bodies. Returns all resulting bodies.
    pub fn subtract(self, tools: Vec<Body>) -> PsResult<Vec<Body>> {
        crate::boolean::boolean(
            self,
            tools,
            crate::boolean::BooleanOp::Subtract,
            &crate::boolean::BooleanOptions::default(),
        )
    }

    /// Intersect this body with tool bodies.
    ///
    /// Consumes `self` and the tool bodies. Returns all resulting bodies.
    pub fn intersect(self, tools: Vec<Body>) -> PsResult<Vec<Body>> {
        crate::boolean::boolean(
            self,
            tools,
            crate::boolean::BooleanOp::Intersect,
            &crate::boolean::BooleanOptions::default(),
        )
    }

    /// Split a disconnected body — for example the multi-lump result of uniting
    /// two separated bodies — into one body per connected component. Consumes
    /// `self`; returns every resulting body (including the lump `self` retains
    /// when the body is already connected).
    pub fn disjoin(self) -> PsResult<Vec<Body>> {
        let mut n: c_int = 0;
        let mut ptr: *mut PK_BODY_t = std::ptr::null_mut();
        pk_call!(PK_BODY_disjoin(self.tag, &mut n, &mut ptr));
        let new_bodies = unsafe { PkArray::from_raw(ptr, n) };
        let mut out = vec![self];
        for &t in new_bodies.iter() {
            if t != self.tag {
                out.push(Body::from_tag(t));
            }
        }
        Ok(out)
    }

    // NOTE: PK_BODY_make_section_with_surfs (non-destructive section → wire
    // bodies) returns 0 bodies with both NULL options and a computed
    // PK_BODY_make_section_o_t — the default result_body_type discards the
    // section. Producing output needs the option struct's exact layout
    // (PKU_journal_BODY_make_section_o) decompiled. Deferred; section_with_surf
    // (split-in-place) covers the section oracle meanwhile.

    // --- Lifecycle ---

    pub fn delete(self) -> PsResult<()> {
        self.entity().delete()
    }

    pub fn copy(&self) -> PsResult<Body> {
        let copied = self.entity().copy()?;
        Ok(Body::from_tag(copied.tag()))
    }
}
