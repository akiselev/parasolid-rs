//! Euler operations, redundant topology control, and topology splitting.
//!
//! Euler operations (Ch. 30) are low-level topology modification functions that produce
//! valid topological data structures but do NOT alter geometry. Resulting bodies are
//! normally invalid until geometry is re-attached.
//!
//! Redundant topology (Ch. 31) functions identify and remove topological entities not
//! required for a complete, valid model definition.
//!
//! Splitting topology (Ch. 32) functions split edges and faces at parameter values.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;

use std::os::raw::{c_double, c_int};

// =============================================================================
// Constants — Redundant topology
// =============================================================================

/// Scope of redundant topology operations.
pub type PK_redundant_merge_t = c_int;

/// Minimal scope: only remove topologies internal to specified topols.
pub const PK_redundant_merge_in_c: PK_redundant_merge_t = 0;

/// Default: remove internal + boundary redundancies, but don't merge topols
/// with unspecified neighbors.
pub const PK_redundant_merge_on_c: PK_redundant_merge_t = 1;

/// Maximal scope: remove all redundancies even if it merges topols with
/// unspecified neighbors.
pub const PK_redundant_merge_out_c: PK_redundant_merge_t = 2;

/// Dimension filter for redundant topology identification.
pub type PK_TOPOL_dimension_t = c_int;

/// Vertices only.
pub const PK_TOPOL_dimension_0_c: PK_TOPOL_dimension_t = 0;

/// All dimensions (default).
pub const PK_TOPOL_dimension_any_c: PK_TOPOL_dimension_t = -1;

/// G0 continuity: all manifold non-laminar edges between mesh faces considered
/// redundant. Removes nearly all topology from facet bodies.
pub const PK_continuity_g0_c: PK_continuity_t = 0;

/// Redundancy propagation control.
pub type PK_redundant_propagate_t = c_int;

/// Don't identify dependent redundant topologies (default).
pub const PK_redundant_propagate_no_c: PK_redundant_propagate_t = 0;

/// Identify topologies that would become redundant if other redundancies removed.
pub const PK_redundant_propagate_yes_c: PK_redundant_propagate_t = 1;

// =============================================================================
// Constants — Splitting topology
// =============================================================================

/// Split along U parameter line.
pub const PK_PARAM_direction_u_c: PK_PARAM_direction_t = 0;

/// Split along V parameter line.
pub const PK_PARAM_direction_v_c: PK_PARAM_direction_t = 1;

// =============================================================================
// Options structs — Redundant topology
// =============================================================================

/// Options for `PK_TOPOL_delete_redundant_2`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_TOPOL_delete_redundant_2_o_t {
    /// Struct version tag (set by `_o_m` initializer).
    pub o_t_version: c_int,
    /// Maximum dimension of topologies regarded as redundant.
    /// Default: `PK_TOPOL_dimension_any_c`.
    pub max_topol_dimension: PK_TOPOL_dimension_t,
    /// Scope of operation. Default: `PK_redundant_merge_on_c`.
    pub scope: PK_redundant_merge_t,
    /// Continuity level for facet geometry. Default: `PK_continuity_g0_c`.
    pub facet_geom_continuity: PK_continuity_t,
    /// Whether `vertex_angle` is supplied. Default: `PK_LOGICAL_false`.
    pub have_vertex_angle: PK_LOGICAL_t,
    /// Angle threshold for mvertex redundancy on polyline edges.
    pub vertex_angle: c_double,
    /// Number of protected topologies.
    pub n_protected_topols: c_int,
    /// Topologies to protect from deletion.
    pub protected_topols: *const PK_TOPOL_t,
}

/// Options for `PK_TOPOL_identify_redundant`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_TOPOL_identify_redundant_o_t {
    /// Struct version tag (set by `_o_m` initializer).
    pub o_t_version: c_int,
    /// Maximum dimension of topologies to identify.
    pub max_topol_dimension: PK_TOPOL_dimension_t,
    /// Scope of operation.
    pub scope: PK_redundant_merge_t,
    /// Whether to return redundant topologies or just count.
    /// Default: `PK_LOGICAL_true`.
    pub want_redundant_topols: PK_LOGICAL_t,
    /// Whether to identify dependent redundant topologies.
    /// Default: `PK_redundant_propagate_no_c`.
    pub propagate_redundancy: PK_redundant_propagate_t,
    /// Continuity level for facet geometry. Default: `PK_continuity_g0_c`.
    pub facet_geom_continuity: PK_continuity_t,
    /// Whether `vertex_angle` is supplied. Default: `PK_LOGICAL_false`.
    pub have_vertex_angle: PK_LOGICAL_t,
    /// Angle threshold for mvertex redundancy.
    pub vertex_angle: c_double,
}

// =============================================================================
// Extern "C" — Euler operations (Ch. 30)
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // -------------------------------------------------------------------------
    // Edge slitting / unslitting
    // -------------------------------------------------------------------------

    /// Slit an edge lengthwise, creating a new slit face.
    ///
    /// - `edge`: edge to slit
    /// - `on_left`: which side gets the new face
    /// - `new_edge`: receives new edge tag
    pub fn PK_EDGE_euler_slit(
        edge: PK_EDGE_t,
        on_left: PK_LOGICAL_t,
        new_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Delete a slit face (face must have 1 loop + 2 edges, or 2 loops each
    /// with 1 ring edge).
    ///
    /// - `slit_face`: the slit face to remove
    /// - `surviving_edge`: receives the surviving edge tag
    pub fn PK_FACE_euler_unslit(
        slit_face: PK_FACE_t,
        surviving_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Edge splitting / vertex merging
    // -------------------------------------------------------------------------

    /// Split an edge by adding a vertex.
    ///
    /// - `edge`: edge to split
    /// - `forward`: split direction
    /// - `new_vertex`: receives new vertex
    /// - `new_edge`: receives new edge
    pub fn PK_EDGE_euler_split(
        edge: PK_EDGE_t,
        forward: PK_LOGICAL_t,
        new_vertex: *mut PK_VERTEX_t,
        new_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Delete a vertex from an edge, merging two edges into one.
    ///
    /// - `vertex`: vertex to delete
    /// - `edge`: receives the surviving edge
    pub fn PK_VERTEX_euler_merge_edges(
        vertex: PK_VERTEX_t,
        edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Edge/vertex creation and deletion
    // -------------------------------------------------------------------------

    /// Add a trailing edge + vertex to a vertex in a loop.
    ///
    /// - `fin`: forward vertex of this fin is used as the attachment point
    /// - `new_edge`: receives new edge
    /// - `new_vertex`: receives new vertex
    pub fn PK_LOOP_euler_make_edge(
        fin: PK_FIN_t,
        new_edge: *mut PK_EDGE_t,
        new_vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Create an edge in a loop (alternative edge creation function).
    ///
    /// Present in exports as `PK_LOOP_euler_create_edge`.
    pub fn PK_LOOP_euler_create_edge(
        fin: PK_FIN_t,
        new_edge: *mut PK_EDGE_t,
        new_vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Delete a vertex and its single attached edge.
    pub fn PK_VERTEX_euler_delete(vertex: PK_VERTEX_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Vertex splitting / edge merging
    // -------------------------------------------------------------------------

    /// Split a vertex into two, adding an intervening edge.
    ///
    /// - `fin1`, `fin2`: fins sharing a common forward vertex
    /// - `new_edge`: receives the new intervening edge
    /// - `new_vertex`: receives the new vertex
    pub fn PK_VERTEX_euler_split(
        fin1: PK_FIN_t,
        fin2: PK_FIN_t,
        new_edge: *mut PK_EDGE_t,
        new_vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Merge two vertices by deleting the intervening edge.
    ///
    /// - `edge`: the edge between two vertices
    /// - `vertex_to_delete`: which vertex to remove
    pub fn PK_EDGE_euler_merge_vertices(
        edge: PK_EDGE_t,
        vertex_to_delete: PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Edge zipping
    // -------------------------------------------------------------------------

    /// Split an edge lengthwise into two edges joined at one end.
    ///
    /// - `edge`: edge to split
    /// - `fin`: forward vertex of this fin is the split point
    /// - `new_edge`: receives new edge
    /// - `new_vertex`: receives new vertex
    pub fn PK_EDGE_euler_open_zip(
        edge: PK_EDGE_t,
        fin: PK_FIN_t,
        new_edge: *mut PK_EDGE_t,
        new_vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Zip two edges together by merging vertices at one end.
    ///
    /// - `edge_to_keep`: the edge that survives
    /// - `edge_to_delete`: the edge to remove
    pub fn PK_EDGE_euler_close_zip(
        edge_to_keep: PK_EDGE_t,
        edge_to_delete: PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Isolated loop/vertex
    // -------------------------------------------------------------------------

    /// Add an isolated vertex and loop to a face.
    ///
    /// - `face`: target face
    /// - `new_loop`: receives the new loop
    /// - `new_vertex`: receives the new vertex
    pub fn PK_FACE_euler_make_loop(
        face: PK_FACE_t,
        new_loop: *mut PK_LOOP_t,
        new_vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Delete an isolated vertex and loop from a face.
    pub fn PK_LOOP_euler_delete_isolated(loop_: PK_LOOP_t) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Loop merging/splitting via edge
    // -------------------------------------------------------------------------

    /// Merge two loops in the same face by adding a connecting edge.
    /// `loop2` is deleted.
    ///
    /// - `loop1`, `loop2`: loops to merge (same face)
    /// - `fin1`, `fin2`: attachment fins
    /// - `new_edge`: receives the new connecting edge
    pub fn PK_LOOP_euler_delete_make_edge(
        loop1: PK_LOOP_t,
        loop2: PK_LOOP_t,
        fin1: PK_FIN_t,
        fin2: PK_FIN_t,
        new_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Delete an edge from a loop, splitting it into two loops.
    ///
    /// - `edge`: edge to delete
    /// - `forward`: split direction
    /// - `new_loop`: receives the new loop
    pub fn PK_EDGE_euler_delete_make_loop(
        edge: PK_EDGE_t,
        forward: PK_LOGICAL_t,
        new_loop: *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Face creation/deletion via edge
    // -------------------------------------------------------------------------

    /// Join two vertices in the same loop, creating a new face on the right
    /// of the new edge.
    ///
    /// - `fin1`, `fin2`: attachment fins (same loop)
    /// - `new_edge`: receives the new edge
    /// - `new_face`: receives the new face (on right side)
    pub fn PK_LOOP_euler_make_edge_face(
        fin1: PK_FIN_t,
        fin2: PK_FIN_t,
        new_edge: *mut PK_EDGE_t,
        new_face: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Delete an edge, merging two faces and their loops.
    ///
    /// - `edge`: edge to delete
    /// - `on_left`: determines which face survives
    pub fn PK_EDGE_euler_delete_with_face(
        edge: PK_EDGE_t,
        on_left: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Loop creation/deletion via edge
    // -------------------------------------------------------------------------

    /// Create a new loop by joining two vertices in a loop.
    ///
    /// - `fin1`, `fin2`: attachment fins
    /// - `new_edge`: receives the new edge
    /// - `new_loop`: receives the new loop
    pub fn PK_LOOP_euler_make_edge_loop(
        fin1: PK_FIN_t,
        fin2: PK_FIN_t,
        new_edge: *mut PK_EDGE_t,
        new_loop: *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Delete an edge, merging two loops from the same face.
    pub fn PK_EDGE_euler_delete_with_loop(
        edge: PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Ring edge operations
    // -------------------------------------------------------------------------

    /// Split a face by adding a ring edge. New face on right of edge.
    ///
    /// - `face`: face to split
    /// - `new_edge`: receives the new ring edge
    /// - `new_face`: receives the new face (on right side)
    pub fn PK_FACE_euler_make_ring_face(
        face: PK_FACE_t,
        new_edge: *mut PK_EDGE_t,
        new_face: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Delete a ring edge and its associated face.
    ///
    /// - `edge`: ring edge to delete
    /// - `on_left`: determines which face survives
    pub fn PK_EDGE_euler_delete_ring_face(
        edge: PK_EDGE_t,
        on_left: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Add a new loop to a face via a ring edge (increases genus).
    ///
    /// - `face`: target face
    /// - `new_edge`: receives the new ring edge
    /// - `new_loop`: receives the new loop
    pub fn PK_FACE_euler_make_ring_loop(
        face: PK_FACE_t,
        new_edge: *mut PK_EDGE_t,
        new_loop: *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Delete a bi-wire ring edge and face (decreases genus).
    pub fn PK_EDGE_euler_delete_ring_loop(
        edge: PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Loop transfer
    // -------------------------------------------------------------------------

    /// Transfer a loop from one face to another. Faces must share the same
    /// front/back shells.
    ///
    /// - `loop_`: loop to transfer
    /// - `face`: destination face
    pub fn PK_LOOP_euler_transfer(
        loop_: PK_LOOP_t,
        face: PK_FACE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Fin gluing
    // -------------------------------------------------------------------------

    /// Glue two edges together by gluing fins. The first fin's edge survives;
    /// the second edge is deleted.
    ///
    /// NOTE: The curve of the deleted edge is NOT deleted (exception to the
    /// general Euler rule that deleted topology has geometry removed first).
    pub fn PK_FIN_euler_glue(
        fin1: PK_FIN_t,
        fin2: PK_FIN_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Redundant topology (Ch. 31)
    // =========================================================================

    /// Identify and delete redundant topology, merging entities that become
    /// mergeable. Legacy version (prefer `PK_TOPOL_delete_redundant_2`).
    ///
    /// - `n_topols`: number of input topologies
    /// - `topols`: array of topologies to process
    pub fn PK_TOPOL_delete_redundant(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Identify redundant topological entities without deleting them.
    ///
    /// - `n_topols`: number of input topologies
    /// - `topols`: array of topologies to check
    /// - `options`: identification options
    /// - `n_redundant`: receives count of redundant entities found
    /// - `redundant_topols`: receives array of redundant entity tags
    pub fn PK_TOPOL_identify_redundant(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        options: *const PK_TOPOL_identify_redundant_o_t,
        n_redundant: *mut c_int,
        redundant_topols: *mut *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Related query: G1-continuous edge chains
    // =========================================================================

    /// Find a chain of G1-continuous edges starting from the given edge.
    ///
    /// - `edge`: starting edge
    /// - `angular_tolerance`: angle threshold (matches `vertex_angle` from
    ///   redundant topology options)
    /// - `n_edges`: receives number of edges in chain
    /// - `edges`: receives array of edge tags in chain
    pub fn PK_EDGE_find_g1_edges(
        edge: PK_EDGE_t,
        angular_tolerance: c_double,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Splitting topology (Ch. 32)
    // =========================================================================

    /// Split an edge at a specified parameter value.
    ///
    /// - `edge`: edge to split (must have attached geometry)
    /// - `param`: parameter within edge's range (must not coincide with
    ///   existing vertex)
    /// - `new_vertex`: receives vertex created at the split point
    /// - `new_edge`: receives new edge created by the split
    ///
    /// Accepts both tolerant and accurate edges.
    pub fn PK_EDGE_split_at_param(
        edge: PK_EDGE_t,
        param: c_double,
        new_vertex: *mut PK_VERTEX_t,
        new_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Split a face along a constant parameter line.
    ///
    /// - `face`: face to split (must NOT have facet geometry)
    /// - `param`: parameter value along which to split
    /// - `param_dir`: direction (`PK_PARAM_direction_u_c` or `_v_c`)
    /// - `n_new_edges`: receives number of new edges created
    /// - `new_edges`: receives array of new edges (imprinted onto face;
    ///   derived edges from splitting existing edges are NOT included)
    /// - `n_new_faces`: receives number of new faces created
    /// - `new_faces`: receives array of new faces (original face lies on
    ///   right of one of the new edges)
    ///
    /// New edges go in the direction of `param_dir`.
    pub fn PK_FACE_split_at_param(
        face: PK_FACE_t,
        param: c_double,
        param_dir: PK_PARAM_direction_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
        n_new_faces: *mut c_int,
        new_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;
}
