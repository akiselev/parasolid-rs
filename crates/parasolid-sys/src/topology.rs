//! Topological model structure bindings.
//!
//! Covers body creation (primitives, from-geometry, from-topology), topological
//! navigation, compound body operations, and body type/configuration constants.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;
use std::os::raw::{c_double, c_int};

// =============================================================================
// Body type constants — returned by PK_BODY_ask_type
// =============================================================================

pub type PK_BODY_type_t = c_int;

/// Body has no topology.
// Body type token values determined EMPIRICALLY by probing pskernel.dll
// V37.1.243 (SOLIDWORKS 2025): created bodies of each type and printed
// PK_BODY_ask_type. [probed] = observed; [guess] = not yet verified.
pub const PK_BODY_type_solid_c: PK_BODY_type_t = 5601; // [probed]
pub const PK_BODY_type_sheet_c: PK_BODY_type_t = 5602; // [probed]
pub const PK_BODY_type_minimum_c: PK_BODY_type_t = 5603; // [probed]
pub const PK_BODY_type_wire_c: PK_BODY_type_t = 5604; // [probed]
pub const PK_BODY_type_acorn_c: PK_BODY_type_t = 5605; // [guess]
pub const PK_BODY_type_empty_c: PK_BODY_type_t = 5606; // [guess]
pub const PK_BODY_type_general_c: PK_BODY_type_t = 5607; // [guess]
pub const PK_BODY_type_compound_c: PK_BODY_type_t = 5608; // [guess]
pub const PK_BODY_type_unspecified_c: PK_BODY_type_t = 5609; // [guess]

// =============================================================================
// Body configuration constants — returned by PK_BODY_ask_config
// =============================================================================

pub type PK_BODY_config_t = c_int;

/// Standard (non-compound) body.
pub const PK_BODY_config_standard_c: PK_BODY_config_t = 0;
/// Compound body (container for child bodies).
pub const PK_BODY_config_compound_c: PK_BODY_config_t = 1;
/// Child body within a compound.
pub const PK_BODY_config_child_c: PK_BODY_config_t = 2;

// =============================================================================
// Loop type constants — returned by PK_LOOP_ask_type
// =============================================================================

pub type PK_LOOP_type_t = c_int;

pub const PK_LOOP_type_outer_c: PK_LOOP_type_t = 0;
pub const PK_LOOP_type_inner_c: PK_LOOP_type_t = 1;
pub const PK_LOOP_type_winding_c: PK_LOOP_type_t = 2;
pub const PK_LOOP_type_vertex_c: PK_LOOP_type_t = 3;
pub const PK_LOOP_type_wire_c: PK_LOOP_type_t = 4;
pub const PK_LOOP_type_likely_inner_c: PK_LOOP_type_t = 5;
pub const PK_LOOP_type_likely_outer_c: PK_LOOP_type_t = 6;
pub const PK_LOOP_type_unclear_c: PK_LOOP_type_t = 7;
pub const PK_LOOP_type_inner_sing_c: PK_LOOP_type_t = 8;

// =============================================================================
// Edge type constants — returned by PK_EDGE_ask_type
// =============================================================================

pub type PK_EDGE_type_t = c_int;

/// Wireframe edge (0 fins).
pub const PK_EDGE_type_wireframe_c: PK_EDGE_type_t = 0;
/// Laminar edge (1 fin).
pub const PK_EDGE_type_laminar_c: PK_EDGE_type_t = 1;
/// Normal manifold edge (2 fins, opposite sense).
pub const PK_EDGE_type_normal_c: PK_EDGE_type_t = 2;
/// General non-manifold edge (2 fins, same sense).
pub const PK_EDGE_type_general_c: PK_EDGE_type_t = 3;

// =============================================================================
// Shell type constants — returned by PK_SHELL_ask_type
// =============================================================================

pub type PK_SHELL_type_t = c_int;

pub const PK_SHELL_type_face_c: PK_SHELL_type_t = 0;
pub const PK_SHELL_type_wire_c: PK_SHELL_type_t = 1;
pub const PK_SHELL_type_acorn_c: PK_SHELL_type_t = 2;

// =============================================================================
// Vertex type constants — returned by PK_VERTEX_ask_type
// =============================================================================

pub type PK_VERTEX_type_t = c_int;

pub const PK_VERTEX_type_standard_c: PK_VERTEX_type_t = 0;
pub const PK_VERTEX_type_tolerant_c: PK_VERTEX_type_t = 1;

// =============================================================================
// Region type constants
// =============================================================================

pub type PK_REGION_type_t = c_int;

pub const PK_REGION_type_solid_c: PK_REGION_type_t = 0;
pub const PK_REGION_type_void_c: PK_REGION_type_t = 1;

// =============================================================================
// Local coordinate system structure
// =============================================================================

// =============================================================================
// Opaque options types for topology creation
// =============================================================================

/// Options for `PK_BODY_create_topology`.
#[repr(C)]
pub struct PK_BODY_create_topology_o_t { _private: [u8; 0] }

// =============================================================================
// Extern functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    // =========================================================================
    // Body type and configuration enquiry
    // =========================================================================

    /// Return the body type (empty/acorn/wire/sheet/solid/general).
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_type(PK_BODY_t body, PK_BODY_type_t *btype);
    /// ```
    pub fn PK_BODY_ask_type(
        body: PK_BODY_t,
        btype: *mut PK_BODY_type_t,
    ) -> PK_ERROR_code_t;

    /// Set (convert) the body type where topology allows.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_set_type(PK_BODY_t body, PK_BODY_type_t btype);
    /// ```
    /// Signature verified against Parasolid V35 docs (options may be NULL).
    pub fn PK_BODY_set_type(
        body: PK_BODY_t,
        new_type: PK_BODY_type_t,
        options: *const core::ffi::c_void,
    ) -> PK_ERROR_code_t;

    /// Return the body configuration (standard/compound/child).
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_config(PK_BODY_t body, PK_BODY_config_t *config);
    /// ```
    pub fn PK_BODY_ask_config(
        body: PK_BODY_t,
        config: *mut PK_BODY_config_t,
    ) -> PK_ERROR_code_t;

    /// Reverse the orientation of all surfaces in a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_reverse_orientation(PK_BODY_t body);
    /// ```
    pub fn PK_BODY_reverse_orientation(body: PK_BODY_t) -> PK_ERROR_code_t;

    /// Return the components of a body (as representative shells).
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_components(
    ///     PK_BODY_t body,
    ///     int *n_components,
    ///     PK_SHELL_t **components
    /// );
    /// ```
    pub fn PK_BODY_ask_components(
        body: PK_BODY_t,
        n_components: *mut c_int,
        components: *mut *mut PK_SHELL_t,
    ) -> PK_ERROR_code_t;

    /// Check body validity.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_check(PK_BODY_t body);
    /// ```
    pub fn PK_BODY_check(body: PK_BODY_t) -> PK_ERROR_code_t;

    /// Identify general characteristics of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_identify_general(PK_BODY_t body, ...);
    /// ```
    pub fn PK_BODY_identify_general(body: PK_BODY_t) -> PK_ERROR_code_t;

    // =========================================================================
    // Body topological navigation
    // =========================================================================

    /// Return all faces of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_faces(PK_BODY_t body, int *n_faces, PK_FACE_t **faces);
    /// ```
    pub fn PK_BODY_ask_faces(
        body: PK_BODY_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return all edges of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_edges(PK_BODY_t body, int *n_edges, PK_EDGE_t **edges);
    /// ```
    pub fn PK_BODY_ask_edges(
        body: PK_BODY_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Return all vertices of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_vertices(PK_BODY_t body, int *n_vertices, PK_VERTEX_t **vertices);
    /// ```
    pub fn PK_BODY_ask_vertices(
        body: PK_BODY_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Return all shells of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_shells(PK_BODY_t body, int *n_shells, PK_SHELL_t **shells);
    /// ```
    pub fn PK_BODY_ask_shells(
        body: PK_BODY_t,
        n_shells: *mut c_int,
        shells: *mut *mut PK_SHELL_t,
    ) -> PK_ERROR_code_t;

    /// Return all regions of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_regions(PK_BODY_t body, int *n_regions, PK_REGION_t **regions);
    /// ```
    pub fn PK_BODY_ask_regions(
        body: PK_BODY_t,
        n_regions: *mut c_int,
        regions: *mut *mut PK_REGION_t,
    ) -> PK_ERROR_code_t;

    /// Return all loops of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_loops(PK_BODY_t body, int *n_loops, PK_LOOP_t **loops);
    /// ```
    pub fn PK_BODY_ask_loops(
        body: PK_BODY_t,
        n_loops: *mut c_int,
        loops: *mut *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Return all fins of a body.
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_ask_fins(PK_BODY_t body, int *n_fins, PK_FIN_t **fins);
    /// ```
    pub fn PK_BODY_ask_fins(
        body: PK_BODY_t,
        n_fins: *mut c_int,
        fins: *mut *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the first face in a body's internal list.
    pub fn PK_BODY_ask_first_face(
        body: PK_BODY_t,
        face: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return the first edge in a body's internal list.
    pub fn PK_BODY_ask_first_edge(
        body: PK_BODY_t,
        edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Copy a body retaining only topology (no geometry).
    ///
    /// ```c
    /// PK_ERROR_code_t PK_BODY_copy_topology(PK_BODY_t body, PK_BODY_t *copy);
    /// ```
    pub fn PK_BODY_copy_topology(
        body: PK_BODY_t,
        copy: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Face navigation
    // =========================================================================

    /// Return the body that owns a face.
    pub fn PK_FACE_ask_body(
        face: PK_FACE_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return all edges of a face.
    pub fn PK_FACE_ask_edges(
        face: PK_FACE_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Return all loops of a face.
    pub fn PK_FACE_ask_loops(
        face: PK_FACE_t,
        n_loops: *mut c_int,
        loops: *mut *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Return the first loop in a face.
    pub fn PK_FACE_ask_first_loop(
        face: PK_FACE_t,
        loop_: *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Return the next face in the body's internal face list.
    pub fn PK_FACE_ask_next_in_body(
        face: PK_FACE_t,
        next: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return faces adjacent to a given face.
    pub fn PK_FACE_ask_faces_adjacent(
        face: PK_FACE_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return all vertices of a face.
    pub fn PK_FACE_ask_vertices(
        face: PK_FACE_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Return the shells that reference a face.
    pub fn PK_FACE_ask_shells(
        face: PK_FACE_t,
        n_shells: *mut c_int,
        shells: *mut *mut PK_SHELL_t,
    ) -> PK_ERROR_code_t;

    /// Return the oriented surface of a face (surface tag + orientation flag).
    ///
    /// When `orient` is PK_LOGICAL_true, face normal == surface normal.
    pub fn PK_FACE_ask_oriented_surf(
        face: PK_FACE_t,
        surf: *mut PK_SURF_t,
        orient: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the surface attached to a face (without orientation).
    pub fn PK_FACE_ask_surf(
        face: PK_FACE_t,
        surf: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Find the parametric outer loop of a face.
    pub fn PK_FACE_find_outer_loop(
        face: PK_FACE_t,
        loop_: *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Reverse a face normal in a general body.
    pub fn PK_FACE_reverse(face: PK_FACE_t) -> PK_ERROR_code_t;

    /// Return common edges between two faces.
    pub fn PK_FACE_find_edges_common(
        face1: PK_FACE_t,
        face2: PK_FACE_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Edge navigation
    // =========================================================================

    /// Return the body that owns an edge.
    pub fn PK_EDGE_ask_body(
        edge: PK_EDGE_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return the vertices at edge endpoints.
    pub fn PK_EDGE_ask_vertices(
        edge: PK_EDGE_t,
        vertices: *mut [PK_VERTEX_t; 2],
    ) -> PK_ERROR_code_t;

    /// Return faces adjacent to an edge.
    pub fn PK_EDGE_ask_faces(
        edge: PK_EDGE_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return all fins of an edge.
    pub fn PK_EDGE_ask_fins(
        edge: PK_EDGE_t,
        n_fins: *mut c_int,
        fins: *mut *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the first fin of an edge.
    pub fn PK_EDGE_ask_first_fin(
        edge: PK_EDGE_t,
        fin: *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the edge type (wireframe/laminar/normal/general).
    pub fn PK_EDGE_ask_type(
        edge: PK_EDGE_t,
        etype: *mut PK_EDGE_type_t,
    ) -> PK_ERROR_code_t;

    /// Return the curve attached to an edge.
    pub fn PK_EDGE_ask_curve(
        edge: PK_EDGE_t,
        curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Return the oriented curve of an edge (curve tag + orientation flag).
    pub fn PK_EDGE_ask_oriented_curve(
        edge: PK_EDGE_t,
        curve: *mut PK_CURVE_t,
        orient: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the next edge in the body's internal edge list.
    pub fn PK_EDGE_ask_next_in_body(
        edge: PK_EDGE_t,
        next: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Return shells that reference an edge.
    pub fn PK_EDGE_ask_shells(
        edge: PK_EDGE_t,
        n_shells: *mut c_int,
        shells: *mut *mut PK_SHELL_t,
    ) -> PK_ERROR_code_t;

    /// Return edge geometry info.
    /// Signature verified against Parasolid V35 `PK_EDGE_ask_geometry` docs.
    pub fn PK_EDGE_ask_geometry(
        edge: PK_EDGE_t,
        want_interval: PK_LOGICAL_t,
        curve: *mut PK_CURVE_t,
        class: *mut PK_CLASS_t,
        ends: *mut PK_VECTOR_t,
        t_int: *mut PK_INTERVAL_t,
        sense: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Fin navigation
    // =========================================================================

    /// Return the edge that a fin belongs to.
    pub fn PK_FIN_ask_edge(
        fin: PK_FIN_t,
        edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Return the loop that a fin belongs to.
    pub fn PK_FIN_ask_loop(
        fin: PK_FIN_t,
        loop_: *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Return the face that a fin belongs to (via its loop).
    pub fn PK_FIN_ask_face(
        fin: PK_FIN_t,
        face: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return the body that owns a fin.
    pub fn PK_FIN_ask_body(
        fin: PK_FIN_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return the next fin in the same loop.
    pub fn PK_FIN_ask_next_in_loop(
        fin: PK_FIN_t,
        next: *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the previous fin in the same loop.
    pub fn PK_FIN_ask_previous_in_loop(
        fin: PK_FIN_t,
        prev: *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the next fin of the same edge (radial traversal).
    pub fn PK_FIN_ask_next_of_edge(
        fin: PK_FIN_t,
        next: *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the previous fin of the same edge (radial traversal).
    pub fn PK_FIN_ask_previous_of_edge(
        fin: PK_FIN_t,
        prev: *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Test whether a fin is positive (edge direction matches fin direction).
    pub fn PK_FIN_is_positive(
        fin: PK_FIN_t,
        is_positive: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the oriented curve of a fin.
    pub fn PK_FIN_ask_oriented_curve(
        fin: PK_FIN_t,
        curve: *mut PK_CURVE_t,
        orient: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return fin geometry info.
    pub fn PK_FIN_ask_geometry(
        fin: PK_FIN_t,
        curve: *mut PK_CURVE_t,
        t_range: *mut PK_INTERVAL_t,
        sense: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the curve attached to a fin.
    pub fn PK_FIN_ask_curve(
        fin: PK_FIN_t,
        curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Return the fin type.
    pub fn PK_FIN_ask_type(
        fin: PK_FIN_t,
        ftype: *mut c_int,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Loop navigation
    // =========================================================================

    /// Return the body that owns a loop.
    pub fn PK_LOOP_ask_body(
        loop_: PK_LOOP_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return the face that a loop belongs to.
    pub fn PK_LOOP_ask_face(
        loop_: PK_LOOP_t,
        face: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return all fins in a loop.
    pub fn PK_LOOP_ask_fins(
        loop_: PK_LOOP_t,
        n_fins: *mut c_int,
        fins: *mut *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the first fin in a loop.
    pub fn PK_LOOP_ask_first_fin(
        loop_: PK_LOOP_t,
        fin: *mut PK_FIN_t,
    ) -> PK_ERROR_code_t;

    /// Return the next loop in the same face.
    pub fn PK_LOOP_ask_next_in_face(
        loop_: PK_LOOP_t,
        next: *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    /// Return all edges in a loop.
    pub fn PK_LOOP_ask_edges(
        loop_: PK_LOOP_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Return all vertices in a loop.
    pub fn PK_LOOP_ask_vertices(
        loop_: PK_LOOP_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Return the loop type (outer/inner/winding/vertex/wire/unclear).
    pub fn PK_LOOP_ask_type(
        loop_: PK_LOOP_t,
        ltype: *mut PK_LOOP_type_t,
    ) -> PK_ERROR_code_t;

    /// Test whether a loop is an isolated vertex loop.
    pub fn PK_LOOP_is_isolated(
        loop_: PK_LOOP_t,
        is_isolated: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Vertex navigation
    // =========================================================================

    /// Return the body that owns a vertex.
    pub fn PK_VERTEX_ask_body(
        vertex: PK_VERTEX_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return the point attached to a vertex.
    pub fn PK_VERTEX_ask_point(
        vertex: PK_VERTEX_t,
        point: *mut PK_POINT_t,
    ) -> PK_ERROR_code_t;

    /// Return the faces adjacent to a vertex.
    pub fn PK_VERTEX_ask_faces(
        vertex: PK_VERTEX_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Return edges emanating from a vertex with orientation info.
    pub fn PK_VERTEX_ask_oriented_edges(
        vertex: PK_VERTEX_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        senses: *mut *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the vertex type (standard/tolerant).
    pub fn PK_VERTEX_ask_type(
        vertex: PK_VERTEX_t,
        vtype: *mut PK_VERTEX_type_t,
    ) -> PK_ERROR_code_t;

    /// Return shells that reference a vertex.
    pub fn PK_VERTEX_ask_shells(
        vertex: PK_VERTEX_t,
        n_shells: *mut c_int,
        shells: *mut *mut PK_SHELL_t,
    ) -> PK_ERROR_code_t;

    /// Return isolated loops at a vertex.
    pub fn PK_VERTEX_ask_isolated_loops(
        vertex: PK_VERTEX_t,
        n_loops: *mut c_int,
        loops: *mut *mut PK_LOOP_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Shell navigation
    // =========================================================================

    /// Return the body that owns a shell.
    pub fn PK_SHELL_ask_body(
        shell: PK_SHELL_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return the oriented faces of a shell.
    pub fn PK_SHELL_ask_oriented_faces(
        shell: PK_SHELL_t,
        n_faces: *mut c_int,
        faces: *mut *mut PK_FACE_t,
        orients: *mut *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the region that a shell bounds.
    pub fn PK_SHELL_ask_region(
        shell: PK_SHELL_t,
        region: *mut PK_REGION_t,
    ) -> PK_ERROR_code_t;

    /// Return the shell type (face/wire/acorn).
    pub fn PK_SHELL_ask_type(
        shell: PK_SHELL_t,
        stype: *mut PK_SHELL_type_t,
    ) -> PK_ERROR_code_t;

    /// Return the acorn vertex of an acorn shell.
    pub fn PK_SHELL_ask_acorn_vertex(
        shell: PK_SHELL_t,
        vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Return wireframe edges of a shell.
    pub fn PK_SHELL_ask_wireframe_edges(
        shell: PK_SHELL_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Determine the sign of a shell with respect to its region.
    pub fn PK_SHELL_find_sign(
        shell: PK_SHELL_t,
        sign: *mut c_int,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Region navigation
    // =========================================================================

    /// Return the body that owns a region.
    pub fn PK_REGION_ask_body(
        region: PK_REGION_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return all shells bounding a region.
    pub fn PK_REGION_ask_shells(
        region: PK_REGION_t,
        n_shells: *mut c_int,
        shells: *mut *mut PK_SHELL_t,
    ) -> PK_ERROR_code_t;

    /// Return regions adjacent to a given region.
    pub fn PK_REGION_ask_regions_adjacent(
        region: PK_REGION_t,
        n_regions: *mut c_int,
        regions: *mut *mut PK_REGION_t,
    ) -> PK_ERROR_code_t;

    /// Test whether a region is solid (vs. void).
    pub fn PK_REGION_is_solid(
        region: PK_REGION_t,
        is_solid: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return the region type (solid/void).
    pub fn PK_REGION_ask_type(
        region: PK_REGION_t,
        rtype: *mut PK_REGION_type_t,
    ) -> PK_ERROR_code_t;

    /// Change a region to solid.
    pub fn PK_REGION_make_solid(region: PK_REGION_t) -> PK_ERROR_code_t;

    /// Change a region to void.
    pub fn PK_REGION_make_void(region: PK_REGION_t) -> PK_ERROR_code_t;

    /// Transfer a body into the void region of another body.
    pub fn PK_REGION_combine_bodies(
        region: PK_REGION_t,
        body: PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Topological utility
    // =========================================================================

    /// Extract a body component (identified by shell) into a new body.
    pub fn PK_TOPOL_remove_body_component(
        shell: PK_SHELL_t,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Delete redundant topology from entities (version 2).
    pub fn PK_TOPOL_delete_redundant_2(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Detach geometry from topology.
    pub fn PK_TOPOL_detach_geom(
        topol: PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Copy topology subset into a new general body.
    pub fn PK_TOPOL_make_general_body(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Primitive solid body creation
    // =========================================================================

    /// Create a solid block (cuboid).
    ///
    /// Face order in result: -X, +X, -Y, +Y, -Z, +Z.
    pub fn PK_BODY_create_solid_block(
        x_length: c_double,
        y_length: c_double,
        z_length: c_double,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid cylinder.
    pub fn PK_BODY_create_solid_cyl(
        radius: c_double,
        height: c_double,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid cone.
    pub fn PK_BODY_create_solid_cone(
        top_radius: c_double,
        bottom_radius: c_double,
        height: c_double,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid sphere.
    pub fn PK_BODY_create_solid_sphere(
        radius: c_double,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid torus (standard, apple, or lemon variant).
    ///
    /// Apple torus: minor_radius > major_radius, major > 0.
    /// Lemon torus: minor_radius > |major_radius|, major < 0.
    pub fn PK_BODY_create_solid_torus(
        major_radius: c_double,
        minor_radius: c_double,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid prism (extruded regular polygon).
    pub fn PK_BODY_create_solid_prism(
        radius: c_double,
        height: c_double,
        n_sides: c_int,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Primitive sheet body creation
    // =========================================================================

    /// Create a circular sheet body.
    pub fn PK_BODY_create_sheet_circle(
        radius: c_double,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a rectangular sheet body.
    pub fn PK_BODY_create_sheet_rectangle(
        x_length: c_double,
        y_length: c_double,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a polygonal sheet body.
    pub fn PK_BODY_create_sheet_polygon(
        radius: c_double,
        n_sides: c_int,
        basis_set: *const PK_AXIS2_sf_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a planar sheet body with irregular polygon boundary and optional holes.
    pub fn PK_BODY_create_sheet_planar(
        n_points: c_int,
        points: *const PK_VECTOR_t,
        n_holes: c_int,
        n_hole_points: *const c_int,
        hole_points: *const PK_VECTOR_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body creation from existing geometry
    // =========================================================================

    /// Create a minimum body (single vertex) from a point.
    pub fn PK_POINT_make_minimum_body(
        point: PK_POINT_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a wire body from a curve (version 2).
    pub fn PK_CURVE_make_wire_body_2(
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        n_intervals: c_int,
        intervals: *const PK_INTERVAL_t,
        body: *mut PK_BODY_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
        edge_index: *mut *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Create a sheet body from a surface.
    /// Signature verified against Parasolid V35 docs (uv_box passed by value).
    pub fn PK_SURF_make_sheet_body(
        surf: PK_SURF_t,
        uv_box: PK_UVBOX_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid body from a cone surface.
    pub fn PK_CONE_make_solid_body(
        cone: PK_CONE_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid body from a cylinder surface.
    pub fn PK_CYL_make_solid_body(
        cyl: PK_CYLL_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid body from a sphere surface.
    pub fn PK_SPHERE_make_solid_body(
        sphere: PK_SPHERE_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create a solid body from a torus surface.
    pub fn PK_TORUS_make_solid_body(
        torus: PK_TORUS_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Body creation from topology
    // =========================================================================

    /// Create sheet bodies from faces.
    pub fn PK_FACE_make_sheet_bodies(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create solid bodies from faces (capping open regions).
    pub fn PK_FACE_make_solid_bodies(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Extract manifold components from a general body.
    pub fn PK_BODY_make_manifold_bodies(
        body: PK_BODY_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Split a disjoint general body into separate bodies.
    pub fn PK_BODY_disjoin(
        body: PK_BODY_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Compound body operations
    // =========================================================================

    /// Create a compound body from an array of bodies.
    pub fn PK_BODY_make_compound(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        compound: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Add bodies to an existing compound body.
    pub fn PK_BODY_add_to_compound(
        compound: PK_BODY_t,
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Extract child bodies from a compound (make them standalone).
    pub fn PK_BODY_remove_from_parents(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return child bodies of a compound body.
    pub fn PK_BODY_ask_children(
        body: PK_BODY_t,
        n_children: *mut c_int,
        children: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Return the parent compound body of a child body.
    pub fn PK_BODY_ask_parent(
        body: PK_BODY_t,
        parent: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Wire / sheet / general body modifications
    // =========================================================================

    /// Attach rubber faces to closed wire loops (wire -> sheet conversion).
    pub fn PK_EDGE_make_faces_from_wire(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        n_new_faces: *mut c_int,
        new_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Delete a face from a sheet body (pierce the sheet).
    pub fn PK_FACE_delete_from_sheet(
        face: PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Delete wireframe edges from a body.
    pub fn PK_EDGE_delete_wireframe(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Remove faces from a general body.
    pub fn PK_FACE_delete_from_gen_body(
        n_faces: c_int,
        faces: *const PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Remove acorn vertices from a general body.
    pub fn PK_VERTEX_delete_acorn(
        n_vertices: c_int,
        vertices: *const PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Make a wire body from an edge (for tolerant edges).
    pub fn PK_EDGE_make_wire_body(
        edge: PK_EDGE_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Create topology of a minimum body.
    pub fn PK_BODY_create_minimum_topology(
        n_topols: c_int,
        classes: *const PK_CLASS_t,
        n_relations: c_int,
        parents: *const c_int,
        children: *const c_int,
        senses: *const PK_TOPOL_sense_t,
        body: *mut PK_BODY_t,
        topols: *mut PK_TOPOL_t,
        fault: *mut PK_BODY_fault_t,
        fault_index: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Create topology of a sheet body.
    pub fn PK_BODY_create_sheet_topology(
        n_topols: c_int,
        classes: *const PK_CLASS_t,
        n_relations: c_int,
        parents: *const c_int,
        children: *const c_int,
        senses: *const PK_TOPOL_sense_t,
        body: *mut PK_BODY_t,
        topols: *mut PK_TOPOL_t,
        fault: *mut PK_BODY_fault_t,
        fault_index: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Create topology of a solid body.
    pub fn PK_BODY_create_solid_topology(
        n_topols: c_int,
        classes: *const PK_CLASS_t,
        n_relations: c_int,
        parents: *const c_int,
        children: *const c_int,
        senses: *const PK_TOPOL_sense_t,
        body: *mut PK_BODY_t,
        topols: *mut PK_TOPOL_t,
        fault: *mut PK_BODY_fault_t,
        fault_index: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Create topology of a wire body.
    pub fn PK_BODY_create_wire_topology(
        n_topols: c_int,
        classes: *const PK_CLASS_t,
        n_relations: c_int,
        parents: *const c_int,
        children: *const c_int,
        senses: *const PK_TOPOL_sense_t,
        body: *mut PK_BODY_t,
        topols: *mut PK_TOPOL_t,
        fault: *mut PK_BODY_fault_t,
        fault_index: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Create body topology with options.
    pub fn PK_BODY_create_topology(
        n_topols: c_int,
        classes: *const PK_CLASS_t,
        n_relations: c_int,
        parents: *const c_int,
        children: *const c_int,
        senses: *const PK_TOPOL_sense_t,
        options: *const PK_BODY_create_topology_o_t,
        body: *mut PK_BODY_t,
        topols: *mut PK_TOPOL_t,
        fault: *mut PK_BODY_fault_t,
        fault_index: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Transform body by transformation matrix.
    pub fn PK_BODY_transform(
        body: PK_BODY_t,
        transf: PK_TRANSF_t,
        tolerance: c_double,
        n_replaces: *mut c_int,
        replaces: *mut *mut PK_GEOM_t,
        exact: *mut *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Return all frames belonging to a body.
    pub fn PK_BODY_ask_frames(
        body: PK_BODY_t,
        n_frames: *mut c_int,
        frames: *mut *mut PK_FRAME_t,
    ) -> PK_ERROR_code_t;
}
