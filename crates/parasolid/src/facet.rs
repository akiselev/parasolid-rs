//! Faceting / tessellation — `PK_TOPOL_facet_2`.
//!
//! Produces a triangle mesh of a body's faces via Parasolid's tabular faceting
//! API. This is the bridge to mesh-based validation (e.g. comparing against a
//! reference mesh from a CAD dataset). [`Body::facet`] returns a [`Mesh`] with
//! the vertex positions and the total facet (triangle) count.
//!
//! The two option sub-structures (`control` = mesh generation, `choice` = which
//! output tables) are initialised with the same defaults Parasolid's
//! `PK_TOPOL_facet_2_o_m` macro would set — recovered from the RE enum catalog.
//!
//! # Option versioning (recovered empirically)
//!
//! `PK_TOPOL_facet_2` accepts option `o_t_version` **5** on this kernel
//! (V37.01.243): 1..4 return 5022 (`o_t_version_unknown`), 6+ return 5014
//! (`field_of_wrong_type`). The `PK_TOPOL_facet_mesh_2_o_t` (384 B) and
//! `PK_TOPOL_facet_choice_2_o_t` (144 B) layouts are locked by compile-time
//! offset assertions in `parasolid-sys` (they previously omitted `o_t_version`
//! and, for choice, all 23 table-selection flags — never runtime-validated, so
//! the bugs survived). At version 5 the control struct matches the catalog only
//! through `wire_edges`; the `incremental_*` and later fields must stay zero
//! (setting them returns 908, "bad option data"). See [`default_control`].

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::body::Body;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::geom::Vec3;

// --- PK_facet_* default enum tokens, per ch105 documented defaults -----------
const PK_FACET_SHAPE_CONVEX_C: c_int = 20502;
const PK_FACET_MATCH_TOPOL_C: c_int = 20522;
const PK_FACET_DENSITY_NO_VIEW_C: c_int = 20540;
const PK_FACET_CULL_NONE_C: c_int = 20560;
const PK_FACET_WIRE_EDGES_NO_C: c_int = 22140;
const PK_FACET_IGNORE_NO_C: c_int = 22111;
const PK_FACET_IGNORE_SCOPE_GLOBAL_C: c_int = 22131;
const PK_FACET_SMP_NO_C: c_int = 24110;
const PK_FACET_CONSISTENT_PARMS_NO_C: c_int = 22510;
const PK_FACET_PT_REPORT_NO_C: c_int = 24570;

/// A triangle mesh produced by [`Body::facet`].
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    /// Total number of facets (triangles) generated.
    pub n_facets: usize,
    /// Total number of facet strips.
    pub n_strips: usize,
    /// Total number of fins (facet-boundary half-edges): `3 * n_facets` for a
    /// pure-triangle mesh.
    pub n_fins: usize,
    /// Vertex positions from the `point_vec` table.
    ///
    /// Currently empty: the tabular vertex table requires a facet option version
    /// newer than the version-5 option layout this kernel accepts through
    /// `PK_TOPOL_facet_2` (see the module docs). The topology totals above are
    /// unaffected and validated.
    pub vertices: Vec<Vec3>,
}

/// The kernel's accepted option version for `PK_TOPOL_facet_2` (V37.01.243).
///
/// Found empirically under Wine: versions 1..4 return 5022
/// (`o_t_version_unknown`), version 5 is accepted, 6+ return 5014
/// (`field_of_wrong_type`). At version 5 the struct layout matches the catalog
/// only THROUGH `wire_edges` — the `incremental_*` and later fields must be left
/// zero (setting them to their v-latest enum tokens returns 908, "bad option
/// data", because version 5's tail layout differs). A block faceted with the
/// settings below yields exactly the expected 12 triangles / 8 vertices.
const FACET_O_T_VERSION: c_int = 5;

fn default_control() -> PK_TOPOL_facet_mesh_2_o_t {
    // Zero-init (all `is_*` tolerance flags false → kernel default tolerances,
    // all counts/pointers null) then set only the enum fields up to `wire_edges`
    // to their documented default tokens (ch105). Fields after `wire_edges`
    // (incremental_*, inflect, quality, sing_topol, respect_offset, scales,
    // viewports) MUST stay zero at version 5 — see FACET_O_T_VERSION.
    let mut c: PK_TOPOL_facet_mesh_2_o_t = unsafe { std::mem::zeroed() };
    c.o_t_version = FACET_O_T_VERSION;
    c.shape = PK_FACET_SHAPE_CONVEX_C;
    c.match_ = PK_FACET_MATCH_TOPOL_C;
    c.density = PK_FACET_DENSITY_NO_VIEW_C;
    c.cull = PK_FACET_CULL_NONE_C;
    c.max_facet_sides = 3; // triangles
    c.ignore = PK_FACET_IGNORE_NO_C;
    c.ignore_scope = PK_FACET_IGNORE_SCOPE_GLOBAL_C;
    c.wire_edges = PK_FACET_WIRE_EDGES_NO_C;
    c
}

fn default_choice() -> PK_TOPOL_facet_choice_2_o_t {
    let mut c: PK_TOPOL_facet_choice_2_o_t = unsafe { std::mem::zeroed() };
    c.o_t_version = FACET_O_T_VERSION;
    c.smp = PK_FACET_SMP_NO_C;
    c.consistent_parms = PK_FACET_CONSISTENT_PARMS_NO_C;
    c.report_pts_off_topol = PK_FACET_PT_REPORT_NO_C;
    // Request the vertex-coordinate table and facet→point index table. (At the
    // version-5 option layout these flags do not yet take effect — the tabular
    // output flags belong to a newer option version; the facet topology totals
    // in the result structure are returned regardless.)
    c.point_vec = PK_LOGICAL_true;
    c.data_point_idx = PK_LOGICAL_true;
    c
}

impl Body {
    /// Tessellate this body into a triangle [`Mesh`].
    ///
    /// Returns the mesh topology totals (`n_facets`, `n_strips`, `n_fins`) from
    /// the result structure, which are validated: a solid block yields 12
    /// triangles and 36 fins. Vertex coordinates (`vertices`) are populated from
    /// the `point_vec` table when available — see the note on [`Mesh::vertices`].
    pub fn facet(&self) -> PsResult<Mesh> {
        let mut options = PK_TOPOL_facet_2_o_t {
            control: default_control(),
            choice: default_choice(),
        };
        let mut result: PK_TOPOL_facet_2_r_t = unsafe { std::mem::zeroed() };
        let mut topol: PK_TOPOL_t = self.tag;
        // A "mild" return from faceting means the mesh was produced but some
        // faces raised faults; the tables are still valid. Only serious/fatal
        // errors abort.
        let code = unsafe {
            PK_TOPOL_facet_2(1, &mut topol, std::ptr::null_mut(), &mut options, &mut result)
        };
        if code != 0 {
            match crate::error::PsError::from_code(code) {
                crate::error::PsError::Mild(_) => {}
                other => return Err(other),
            }
        }

        // point_vec is a flat array of doubles (3 per vertex). Present only when
        // the tabular vertex table is returned (see Mesh::vertices).
        let n_doubles = result.tables.n_point_vec.max(0) as usize;
        let vertices = if result.tables.point_vec.is_null() || n_doubles < 3 {
            Vec::new()
        } else {
            let slice = unsafe { std::slice::from_raw_parts(result.tables.point_vec, n_doubles) };
            slice
                .chunks_exact(3)
                .map(|c| Vec3::new(c[0], c[1], c[2]))
                .collect()
        };
        let n_facets = result.n_facets.max(0) as usize;
        let n_strips = result.n_strips.max(0) as usize;
        let n_fins = result.n_fins.max(0) as usize;

        // Release the kernel-allocated tables.
        pk_call!(PK_TOPOL_facet_2_r_f(&mut result));
        Ok(Mesh {
            n_facets,
            n_strips,
            n_fins,
            vertices,
        })
    }
}

// =============================================================================
// Convergent-modeling mesh (PK_MESH_t) built from facet data
// =============================================================================

/// A **convergent-modeling mesh** entity (`PK_MESH_t`), built from raw facet data
/// via the callback-based `PK_MESH_create_from_facets`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FacetMesh {
    tag: PK_MESH_t,
}

/// Context passed to the facet-reader callback: a single independent-facet
/// "vector" block, delivered on the one reader call with `status = stop`.
struct FacetVectorContext {
    block: PK_MESH_facet_vector_t,
}

/// The facet-reader callback the kernel calls repeatedly. Emits one vector block
/// (all triangles, 3 positions each), then reports `stop`.
unsafe extern "C" fn facet_vector_reader(
    context: *mut std::os::raw::c_void,
    descriptor: *mut PK_MESH_facet_descriptor_t,
    status: *mut PK_MESH_cb_status_t,
) -> PK_ERROR_code_t {
    unsafe {
        let ctx = &mut *(context as *mut FacetVectorContext);
        // Deliver the single block AND signal `stop` on the SAME call: the
        // consumer (`FUN_1813f30a0`) dispatches the descriptor after every reader
        // call and treats `stop` as "this is the last block" (still processed),
        // NOT "no more data". `continue` means "another block follows"; emitting
        // a block on a continue-call and then stopping on the next call would
        // dispatch the block twice. Status is the out-param; return is ignored.
        (*descriptor).facet_type = PK_MESH_facet_type_vector_internal_c;
        (*descriptor).facet_data =
            (&mut ctx.block) as *mut PK_MESH_facet_vector_t as *mut std::os::raw::c_void;
        *status = PK_MESH_cb_status_stop_c;
    }
    PK_ERROR_no_errors
}

impl FacetMesh {
    pub(crate) fn from_tag(tag: PK_MESH_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    /// Build a convergent mesh from independent triangles (each three corner
    /// points). Wraps `PK_MESH_create_from_facets` with an independent-facet
    /// "vector" reader; the kernel groups every 3 positions into one facet.
    pub fn from_triangles(triangles: &[[Vec3; 3]]) -> PsResult<FacetMesh> {
        // Convergent Modeling (facet geometry) is DISABLED by default in every
        // session; `PK_MESH_create_from_facets` returns 5237 unless it is turned
        // on first (the kernel gates on a `DS_roll_data()+0x48` byte). Enable it
        // for the session before building the mesh — see ch082 §82.2.1.
        pk_call!(PK_SESSION_set_facet_geometry(PK_facet_geometry_all_c));

        // Flatten to `n_triangles * 3` vertex positions (kept alive for the call).
        let positions: Vec<f64> = triangles
            .iter()
            .flat_map(|t| t.iter().flat_map(|v| [v.x, v.y, v.z]))
            .collect();
        let n_positions = (triangles.len() * 3) as c_int;
        let mut ctx = FacetVectorContext {
            block: PK_MESH_facet_vector_t {
                // First field is the FACET count, not the vertex count (the
                // consumer reads 3 vertices per facet). Passing n_positions
                // over-reads the buffer 3× and fails with PK error 900.
                n_facets: triangles.len() as c_int,
                vertex_positions: positions.as_ptr(),
                vertex_normals: std::ptr::null(),
            },
        };
        let opts = PK_MESH_create_from_facets_o_t {
            o_t_version: 1,
            vertices_estimate: n_positions,
            facet_estimate: triangles.len() as c_int,
            facet_free: None,
            create: PK_MESH_create_now_c,
            have_box: PK_LOGICAL_false,
            box_: PK_BOX_t { coord: [0.0; 6] },
            thread_safe: PK_LOGICAL_false,
        };
        let mut mesh: PK_MESH_t = PK_ENTITY_null;
        let code = unsafe {
            PK_MESH_create_from_facets(
                Some(facet_vector_reader),
                (&mut ctx) as *mut FacetVectorContext as *mut std::os::raw::c_void,
                &opts,
                &mut mesh,
            )
        };
        // Surface the outcome faithfully: the mesh tag is written (non-null) only
        // when construction succeeds. When the convergent-modeling engine rejects
        // the facet data it returns a mild code (e.g. 5241 =
        // `PSM_mesh_create_result` 4/9) and leaves the tag null — that is a real
        // failure, not a tolerable warning, so report it.
        if mesh == PK_ENTITY_null {
            return Err(crate::error::PsError::from_code(code));
        }
        // Non-null tag: tolerate a mild warning, abort on serious/fatal.
        if code != 0 {
            match crate::error::PsError::from_code(code) {
                crate::error::PsError::Mild(_) => {}
                other => return Err(other),
            }
        }
        // `positions` is still alive here (dropped after the call returns).
        Ok(FacetMesh::from_tag(mesh))
    }

    /// Number of facets (mfacets) in the mesh.
    pub fn n_facets(&self) -> PsResult<i32> {
        let mut n = 0;
        pk_call!(PK_MESH_ask_n_mfacets(self.tag, &mut n));
        Ok(n)
    }

    /// Number of vertices (mvertices) in the mesh.
    pub fn n_vertices(&self) -> PsResult<i32> {
        let mut n = 0;
        pk_call!(PK_MESH_ask_n_mvertices(self.tag, &mut n));
        Ok(n)
    }
}
