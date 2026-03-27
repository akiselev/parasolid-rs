#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

//! File I/O bindings — transmit/receive, import/export, and repair.
//!
//! Covers Parasolid chapters 22 (reading/writing data), 87-89 (import/export),
//! and 98 (archives).
//!
//! Geometry creation functions (PK_BCURVE_create, PK_PLANE_create, etc.) live in
//! `bgeom` and `geometry`. Session settings (PK_SESSION_set_check_*) live in
//! `session`. Topology attach/reverse functions live in `topology`. Checking
//! functions (PK_BODY_check, PK_GEOM_check) live in `checking`. Entity enquiry
//! (PK_ENTITY_ask_identifier) lives in `class`.

use std::os::raw::{c_char, c_double, c_int, c_void};

use crate::*;

// =============================================================================
// Transmit format / mesh / seek enums
// =============================================================================

pub type PK_transmit_format_t = c_int;
/// Indexed format for per-face receive.
pub const PK_transmit_format_indexio_c: PK_transmit_format_t = 1;

pub type PK_transmit_meshes_t = c_int;
/// Meshes written to a separate file.
pub const PK_transmit_meshes_separate_c: PK_transmit_meshes_t = 0;
/// Meshes embedded within the part file.
pub const PK_transmit_meshes_embedded_c: PK_transmit_meshes_t = 1;

pub type PK_receive_using_seek_t = c_int;
/// Load mesh data sequentially during receive.
pub const PK_receive_using_seek_no_c: PK_receive_using_seek_t = 0;
/// Load mesh data lazily (only when needed).
pub const PK_receive_using_seek_yes_c: PK_receive_using_seek_t = 1;

// =============================================================================
// Attribute definition mismatch enum
// =============================================================================

pub type PK_ATTDEF_mismatch_t = c_int;
/// Fail on attribute definition mismatch (default).
pub const PK_ATTDEF_mismatch_fail_c: PK_ATTDEF_mismatch_t = 0;
/// Strip mismatched attribute definitions silently.
pub const PK_ATTDEF_mismatch_ignore_c: PK_ATTDEF_mismatch_t = 1;

// =============================================================================
// Partition transmit/receive delta enums
// =============================================================================

pub type PK_PARTITION_xmt_deltas_t = c_int;
/// Do not transmit deltas (default).
pub const PK_PARTITION_xmt_deltas_none_c: PK_PARTITION_xmt_deltas_t = 0;
/// Transmit deltas for main-line pmarks only.
pub const PK_PARTITION_xmt_deltas_main_c: PK_PARTITION_xmt_deltas_t = 2;
/// Defer delta transmit to a later separate call.
pub const PK_PARTITION_xmt_deltas_later_c: PK_PARTITION_xmt_deltas_t = 3;

pub type PK_PARTITION_rcv_deltas_t = c_int;
/// Load pmarks and deltas during receive.
pub const PK_PARTITION_rcv_deltas_yes_c: PK_PARTITION_rcv_deltas_t = 0;
/// Do not load pmarks or deltas.
pub const PK_PARTITION_rcv_deltas_no_c: PK_PARTITION_rcv_deltas_t = 1;
/// Delay loading pmarks and deltas.
pub const PK_PARTITION_rcv_deltas_later_c: PK_PARTITION_rcv_deltas_t = 2;

// =============================================================================
// Pmark new-at-mark enum
// =============================================================================

pub type PK_PMARK_new_at_t = c_int;
/// Pmarks deleted on rollback to before delta receive (default).
pub const PK_PMARK_new_at_current_mark_c: PK_PMARK_new_at_t = 0;
/// Pmarks preserved until partition is deleted.
pub const PK_PMARK_new_with_partition_c: PK_PMARK_new_at_t = 1;

// =============================================================================
// PK_FACE_output_surf_trimmed option enums
// =============================================================================

pub type PK_FACE_trim_confine_t = c_int;
/// Not confined to single period; loops may have gaps at degeneracies.
pub const PK_FACE_trim_confine_no_c: PK_FACE_trim_confine_t = 0;
/// Confined to single period; loops may have gaps at degeneracies.
pub const PK_FACE_trim_confine_yes_c: PK_FACE_trim_confine_t = 1;
/// Confined to single period; loops closed without gaps; natural boundary may be omitted.
pub const PK_FACE_trim_confine_closed_c: PK_FACE_trim_confine_t = 2;
/// Confined to single period; loops closed; exactly one outer peripheral loop.
pub const PK_FACE_trim_confine_periph_c: PK_FACE_trim_confine_t = 3;

pub type PK_FACE_trim_degen_t = c_int;
/// Do not represent degeneracies.
pub const PK_FACE_trim_degen_no_c: PK_FACE_trim_degen_t = 0;
/// Represent degeneracies (including isolated parametric degeneracies).
pub const PK_FACE_trim_degen_yes_c: PK_FACE_trim_degen_t = 1;

// =============================================================================
// Option structs
// =============================================================================

/// Options for `PK_PART_transmit` / `PK_PART_transmit_u` / `PK_PART_transmit_b`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_PART_transmit_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Schema version to write (0 = current with embedded schema).
    pub transmit_version: c_int,
    /// Transmit format (standard or indexed).
    pub transmit_format: PK_transmit_format_t,
    /// How to handle mesh data.
    pub transmit_meshes: PK_transmit_meshes_t,
    /// Opaque context pointer for indexed IO.
    pub transmit_indexed_context: *mut c_void,
}

impl Default for PK_PART_transmit_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            transmit_version: 0,
            transmit_format: 0,
            transmit_meshes: 0,
            transmit_indexed_context: std::ptr::null_mut(),
        }
    }
}

/// Options for `PK_PART_receive` / `PK_PART_receive_u` / `PK_PART_receive_b`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_PART_receive_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Whether to receive user fields.
    pub receive_user_fields: PK_LOGICAL_t,
    /// What to do on attribute definition mismatch.
    pub attdef_mismatch: PK_ATTDEF_mismatch_t,
    /// Transmit format (standard or indexed).
    pub transmit_format: PK_transmit_format_t,
    /// Part index (for multi-part files).
    pub part_index: c_int,
    /// Number of identifiers to receive.
    pub n_identifiers: c_int,
    /// Array of identifier values.
    pub identifiers: *const c_int,
    /// Opaque context pointer for indexed IO.
    pub receive_indexed_context: *mut c_void,
    /// If true, treat key as partition name.
    pub key_is_partition: PK_LOGICAL_t,
    /// Whether to use seek for mesh loading.
    pub receive_using_seek: PK_receive_using_seek_t,
}

impl Default for PK_PART_receive_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            receive_user_fields: PK_LOGICAL_false,
            attdef_mismatch: PK_ATTDEF_mismatch_fail_c,
            transmit_format: 0,
            part_index: 0,
            n_identifiers: 0,
            identifiers: std::ptr::null(),
            receive_indexed_context: std::ptr::null_mut(),
            key_is_partition: PK_LOGICAL_false,
            receive_using_seek: PK_receive_using_seek_no_c,
        }
    }
}

/// Options for `PK_PARTITION_transmit` / `PK_PARTITION_transmit_u` / `PK_PARTITION_transmit_b`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_PARTITION_transmit_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Schema version to write.
    pub transmit_version: c_int,
    /// How to handle delta transmit.
    pub transmit_deltas: PK_PARTITION_xmt_deltas_t,
    /// Transmit all attdefs or only used ones.
    pub transmit_all_attdefs: PK_LOGICAL_t,
    /// How to handle mesh data.
    pub transmit_meshes: PK_transmit_meshes_t,
}

impl Default for PK_PARTITION_transmit_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            transmit_version: 0,
            transmit_deltas: PK_PARTITION_xmt_deltas_none_c,
            transmit_all_attdefs: PK_LOGICAL_false,
            transmit_meshes: 0,
        }
    }
}

/// Options for `PK_PARTITION_receive` / `PK_PARTITION_receive_u` / `PK_PARTITION_receive_b`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_PARTITION_receive_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Whether to receive deltas/pmarks.
    pub receive_deltas: PK_PARTITION_rcv_deltas_t,
    /// Whether to receive all attribute definitions.
    pub receive_all_attdefs: PK_LOGICAL_t,
    /// Callback for attribute definition processing.
    pub attdef_callback: *mut c_void,
    /// Whether the callback is active.
    pub attdef_callback_on: PK_LOGICAL_t,
    /// Opaque context for attdef callback.
    pub attdef_context: *mut c_void,
    /// Allow missing deltas without error.
    pub allow_missing_deltas: PK_LOGICAL_t,
    /// Use seek-based loading for meshes.
    pub receive_using_seek: PK_receive_using_seek_t,
    /// Receive deltas from previous version.
    pub receive_prev_version_deltas: PK_LOGICAL_t,
}

impl Default for PK_PARTITION_receive_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            receive_deltas: PK_PARTITION_rcv_deltas_yes_c,
            receive_all_attdefs: PK_LOGICAL_false,
            attdef_callback: std::ptr::null_mut(),
            attdef_callback_on: PK_LOGICAL_false,
            attdef_context: std::ptr::null_mut(),
            allow_missing_deltas: PK_LOGICAL_false,
            receive_using_seek: PK_receive_using_seek_no_c,
            receive_prev_version_deltas: PK_LOGICAL_false,
        }
    }
}

/// Options for `PK_PARTITION_receive_deltas_2`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_PARTITION_receive_deltas_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// How new pmarks should behave on rollback.
    pub new_at_mark: PK_PMARK_new_at_t,
}

impl Default for PK_PARTITION_receive_deltas_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            new_at_mark: PK_PMARK_new_at_current_mark_c,
        }
    }
}

/// Options for `PK_FACE_output_surf_trimmed`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_FACE_output_surf_trimmed_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Whether to output the underlying trimmed surface.
    pub trim_surf: PK_LOGICAL_t,
    /// Whether to extend surface beyond face boundary.
    pub extend_surf: PK_LOGICAL_t,
    /// Surface fitting tolerance.
    pub surf_tolerance: c_double,
    /// Force cubic representation.
    pub cubic: PK_LOGICAL_t,
    /// Force non-rational representation.
    pub non_rational: PK_LOGICAL_t,
    /// Curve fitting tolerance.
    pub curve_tolerance: c_double,
    /// Confinement mode for periodic surfaces.
    pub confine: PK_FACE_trim_confine_t,
    /// Degeneracy representation mode.
    pub degen: PK_FACE_trim_degen_t,
    /// Whether to return geometry entities.
    pub want_geoms: PK_LOGICAL_t,
    /// Whether to return topology entities.
    pub want_topols: PK_LOGICAL_t,
}

impl Default for PK_FACE_output_surf_trimmed_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            trim_surf: PK_LOGICAL_false,
            extend_surf: PK_LOGICAL_false,
            surf_tolerance: 0.0,
            cubic: PK_LOGICAL_false,
            non_rational: PK_LOGICAL_false,
            curve_tolerance: 0.0,
            confine: PK_FACE_trim_confine_no_c,
            degen: PK_FACE_trim_degen_no_c,
            want_geoms: PK_LOGICAL_true,
            want_topols: PK_LOGICAL_false,
        }
    }
}

/// Options for `PK_SURF_make_sheet_trimmed`.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_SURF_make_sheet_trimmed_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Check wire topology.
    pub check_wires: PK_LOGICAL_t,
    /// Check self-intersection.
    pub check_self_int: PK_LOGICAL_t,
    /// Check loop consistency.
    pub check_loops: PK_LOGICAL_t,
    /// Use nominal geometry.
    pub nominal_geom: PK_LOGICAL_t,
}

impl Default for PK_SURF_make_sheet_trimmed_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            check_wires: PK_LOGICAL_true,
            check_self_int: PK_LOGICAL_false,
            check_loops: PK_LOGICAL_true,
            nominal_geom: PK_LOGICAL_false,
        }
    }
}

// =============================================================================
// Additional error codes for file I/O
// =============================================================================

/// Non-printing characters in attribute strings when saving pre-V12.1.
pub const PK_ERROR_bad_text_conversion: PK_ERROR_code_t = 900;
/// Attribute definition mismatch on receive (with `PK_ATTDEF_mismatch_fail_c`).
pub const PK_ERROR_attr_defn_mismatch: PK_ERROR_code_t = 901;
/// Invalid option data (e.g. pmarks with `PK_PARTITION_xmt_deltas_later_c`).
pub const PK_ERROR_bad_option_data: PK_ERROR_code_t = 902;

// =============================================================================
// Extern functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    // =========================================================================
    // Part transmit
    // =========================================================================

    /// Transmit one or more parts to frustrum via key string.
    pub fn PK_PART_transmit(
        n_parts: c_int,
        parts: *const PK_PART_t,
        key: *const c_char,
        options: *const PK_PART_transmit_o_t,
    ) -> PK_ERROR_code_t;

    /// Transmit parts with Unicode key string.
    pub fn PK_PART_transmit_u(
        n_parts: c_int,
        parts: *const PK_PART_t,
        key: *const c_char,
        options: *const PK_PART_transmit_o_t,
    ) -> PK_ERROR_code_t;

    /// Transmit parts to application memory buffer.
    pub fn PK_PART_transmit_b(
        n_parts: c_int,
        parts: *const PK_PART_t,
        options: *const PK_PART_transmit_o_t,
        n_bytes: *mut c_int,
        buffer: *mut *mut c_void,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Part receive
    // =========================================================================

    /// Receive parts from frustrum via key string.
    pub fn PK_PART_receive(
        key: *const c_char,
        options: *const PK_PART_receive_o_t,
        n_parts: *mut c_int,
        parts: *mut *mut PK_PART_t,
    ) -> PK_ERROR_code_t;

    /// Receive parts with Unicode key string.
    pub fn PK_PART_receive_u(
        key: *const c_char,
        options: *const PK_PART_receive_o_t,
        n_parts: *mut c_int,
        parts: *mut *mut PK_PART_t,
    ) -> PK_ERROR_code_t;

    /// Receive parts from application memory buffer.
    pub fn PK_PART_receive_b(
        n_bytes: c_int,
        buffer: *const c_void,
        options: *const PK_PART_receive_o_t,
        n_parts: *mut c_int,
        parts: *mut *mut PK_PART_t,
    ) -> PK_ERROR_code_t;

    /// Get Parasolid version of part transmit data (from frustrum key).
    pub fn PK_PART_receive_version(
        key: *const c_char,
        version: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Get Parasolid version of part transmit data (Unicode key).
    pub fn PK_PART_receive_version_u(
        key: *const c_char,
        version: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Get Parasolid version of part transmit data (memory buffer).
    pub fn PK_PART_receive_version_b(
        n_bytes: c_int,
        buffer: *const c_void,
        version: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Receive meshes for a part (lazy loading).
    pub fn PK_PART_receive_meshes(
        part: PK_PART_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Partition transmit
    // =========================================================================

    // =========================================================================
    // Partition receive
    // =========================================================================

    // =========================================================================
    // Partition enquiry (archive-related)
    // =========================================================================

    /// Find entity tag by identifier within a part.
    pub fn PK_PART_find_entity_by_ident(
        part: PK_PART_t,
        identifier: c_int,
        entity: *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Query construction geometry attached to a part.
    pub fn PK_PART_ask_geoms(
        part: PK_PART_t,
        n_geoms: *mut c_int,
        geoms: *mut *mut PK_GEOM_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Trimmed surface import route
    // =========================================================================

    /// Embed curve in surface; supports analytic 2D curves, extends surface or
    /// adjusts SP-curves to fit, splits at degeneracies and G1 discontinuities.
    pub fn PK_CURVE_embed_in_surf_2(
        curve: PK_CURVE_t,
        surface: PK_SURF_t,
        range: PK_INTERVAL_t,
        spcurve: *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Create SP-curves from surfaces and 3D trimming curves.
    /// Options for zero-length connectors, direction matching, C2 continuity.
    pub fn PK_CURVE_make_spcurves_2(
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        surfaces: *const PK_SURF_t,
        intervals: *const PK_INTERVAL_t,
        n_spcurves: *mut c_int,
        spcurves: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Create a sheet body (single face) from surface geometry + trimming curves
    /// (SP-curves or 3D curves). Topology inferred from geometry.
    pub fn PK_SURF_make_sheet_trimmed(
        surface: PK_SURF_t,
        n_curves: c_int,
        curves: *const PK_ENTITY_t,
        options: *const PK_SURF_make_sheet_trimmed_o_t,
        body: *mut PK_BODY_t,
        face: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Sew sheet bodies together to complete recreation of imported data.
    pub fn PK_BODY_sew_bodies(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        result_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Simplify geometry on a body.
    pub fn PK_BODY_simplify_geom(
        body: PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Simplify geometry on a face.
    pub fn PK_FACE_simplify_geom(
        face: PK_FACE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // BREP import route
    // =========================================================================

    /// Create topology from defined topological entities and relations.
    ///
    /// `classes` — array of class tokens for each entity to create.
    /// `parents`/`children` — index pairs defining parent-child relations.
    /// `senses` — orientation of each relation.
    pub fn PK_BODY_create_topology_2(
        n_entities: c_int,
        classes: *const PK_CLASS_t,
        n_relations: c_int,
        parents: *const c_int,
        children: *const c_int,
        senses: *const PK_LOGICAL_t,
        body: *mut PK_BODY_t,
        entities: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    /// Remove duplicate curves/surfaces; ensure SP-curves reference correct
    /// face surfaces.
    pub fn PK_BODY_share_geom(
        body: PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Repair shell/region structure; fix topological clashes between shells.
    pub fn PK_BODY_repair_shells(
        body: PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Geometry repair
    // =========================================================================

    /// Find self-intersections in a curve within the given parameter interval.
    pub fn PK_CURVE_find_self_int(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        n_ints: *mut c_int,
        params: *mut *mut c_double,
    ) -> PK_ERROR_code_t;

    /// Fix self-intersections in a curve within the given parameter interval.
    pub fn PK_CURVE_fix_self_int(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        new_curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Find self-intersections in a surface within the given UV box.
    pub fn PK_SURF_find_self_int(
        surface: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        n_ints: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Fix self-intersections in a surface within the given UV box.
    pub fn PK_SURF_fix_self_int(
        surface: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        new_surface: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Repair degeneracies in a surface.
    pub fn PK_SURF_fix_degens(
        surface: PK_SURF_t,
        new_surface: *mut PK_SURF_t,
    ) -> PK_ERROR_code_t;

    /// Repair degeneracies in a curve.
    pub fn PK_CURVE_fix_degens(
        curve: PK_CURVE_t,
        new_curve: *mut PK_CURVE_t,
    ) -> PK_ERROR_code_t;

    /// Find degeneracies in a surface within the given UV box.
    pub fn PK_SURF_find_degens(
        surface: PK_SURF_t,
        uvbox: PK_UVBOX_t,
        n_degens: *mut c_int,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Export / output
    // =========================================================================

    /// Output trimmed surface representation of a face.
    ///
    /// Returns the underlying surface, loop structure (number of curves per loop),
    /// trimming curves, and their senses. Does NOT support facet geometry.
    pub fn PK_FACE_output_surf_trimmed(
        face: PK_FACE_t,
        options: *const PK_FACE_output_surf_trimmed_o_t,
        surface: *mut PK_SURF_t,
        n_loops: *mut c_int,
        n_curves_per_loop: *mut *mut c_int,
        curves: *mut *mut PK_ENTITY_t,
        senses: *mut *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Output position vectors along a curve within a parameter interval.
    pub fn PK_CURVE_output_vectors(
        curve: PK_CURVE_t,
        interval: PK_INTERVAL_t,
        tolerance: c_double,
        n_vectors: *mut c_int,
        vectors: *mut *mut PK_VECTOR_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Knitting
    // =========================================================================

    /// Find knitting pattern for assembling sheet bodies.
    pub fn PK_BODY_find_knit_pattern(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        n_pairs: *mut c_int,
        edge_pairs: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Apply knitting pattern to assemble sheet bodies into a single body.
    pub fn PK_BODY_apply_knit_pattern(
        n_bodies: c_int,
        bodies: *const PK_BODY_t,
        n_pairs: c_int,
        edge_pairs: *const PK_EDGE_t,
        result_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

}
