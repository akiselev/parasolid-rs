//! Lattice geometry — V35 construction geometry for graph-based structures.
//!
//! Lattices are graphs of lballs (spherical nodes) and lrods (cylindrical/conical struts).
//! Requires `PK_SESSION_set_facet_geometry` to be enabled.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;
use std::os::raw::c_int;

// =============================================================================
// Callback types
// =============================================================================

/// Callback for streaming graph data to `PK_LATTICE_create_by_graph`.
pub type PK_LATTICE_graph_cb_f_t = Option<unsafe extern "C" fn(PK_POINTER_t) -> PK_ERROR_code_t>;

/// Callback for iterating lballs.
pub type PK_LBALL_cb_f_t = Option<unsafe extern "C" fn(PK_LBALL_t, PK_POINTER_t) -> PK_ERROR_code_t>;

/// Callback for iterating lrods.
pub type PK_LROD_cb_f_t = Option<unsafe extern "C" fn(PK_LROD_t, PK_POINTER_t) -> PK_ERROR_code_t>;

// =============================================================================
// Opaque options/results types
// =============================================================================

/// Options for `PK_LATTICE_create_by_graph`.
#[repr(C)]
pub struct PK_LATTICE_create_by_graph_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_create_by_graph`.
#[repr(C)]
pub struct PK_LATTICE_create_by_graph_r_t { _private: [u8; 0] }

/// Options for `PK_LATTICE_make_patterned`.
#[repr(C)]
pub struct PK_LATTICE_make_patterned_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_make_patterned`.
#[repr(C)]
pub struct PK_LATTICE_make_patterned_r_t { _private: [u8; 0] }

/// Options for `PK_LATTICE_make_bodies`.
#[repr(C)]
pub struct PK_LATTICE_make_bodies_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_make_bodies`.
#[repr(C)]
pub struct PK_LATTICE_make_bodies_r_t { _private: [u8; 0] }

/// Options for `PK_LATTICE_clip`.
#[repr(C)]
pub struct PK_LATTICE_clip_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_clip`.
#[repr(C)]
pub struct PK_LATTICE_clip_r_t { _private: [u8; 0] }

/// Options for `PK_LATTICE_ask_n_lballs`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lballs_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_ask_n_lballs`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lballs_r_t { _private: [u8; 0] }

/// Options for `PK_LATTICE_ask_n_lrods`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lrods_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_ask_n_lrods`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lrods_r_t { _private: [u8; 0] }

/// Options for `PK_LATTICE_find_box`.
#[repr(C)]
pub struct PK_LATTICE_find_box_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_find_box`.
#[repr(C)]
pub struct PK_LATTICE_find_box_r_t { _private: [u8; 0] }

/// Options for `PK_LATTICE_find_nabox`.
#[repr(C)]
pub struct PK_LATTICE_find_nabox_o_t { _private: [u8; 0] }
/// Results from `PK_LATTICE_find_nabox`.
#[repr(C)]
pub struct PK_LATTICE_find_nabox_r_t { _private: [u8; 0] }

/// Options for `PK_LBALL_ask_lballs_adj`.
#[repr(C)]
pub struct PK_LBALL_ask_lballs_adj_o_t { _private: [u8; 0] }
/// Results from `PK_LBALL_ask_lballs_adj`.
#[repr(C)]
pub struct PK_LBALL_ask_lballs_adj_r_t { _private: [u8; 0] }

/// Options for `PK_LBALL_ask_lrods`.
#[repr(C)]
pub struct PK_LBALL_ask_lrods_o_t { _private: [u8; 0] }
/// Results from `PK_LBALL_ask_lrods`.
#[repr(C)]
pub struct PK_LBALL_ask_lrods_r_t { _private: [u8; 0] }

/// Options for `PK_LBALL_ask_position`.
#[repr(C)]
pub struct PK_LBALL_ask_position_o_t { _private: [u8; 0] }
/// Results from `PK_LBALL_ask_position`.
#[repr(C)]
pub struct PK_LBALL_ask_position_r_t { _private: [u8; 0] }

/// Options for `PK_LBALL_ask_radius`.
#[repr(C)]
pub struct PK_LBALL_ask_radius_o_t { _private: [u8; 0] }
/// Results from `PK_LBALL_ask_radius`.
#[repr(C)]
pub struct PK_LBALL_ask_radius_r_t { _private: [u8; 0] }

/// Options for `PK_LROD_ask_geometry`.
#[repr(C)]
pub struct PK_LROD_ask_geometry_o_t { _private: [u8; 0] }
/// Results from `PK_LROD_ask_geometry`.
#[repr(C)]
pub struct PK_LROD_ask_geometry_r_t { _private: [u8; 0] }

/// Options for `PK_LROD_ask_lballs`.
#[repr(C)]
pub struct PK_LROD_ask_lballs_o_t { _private: [u8; 0] }
/// Results from `PK_LROD_ask_lballs`.
#[repr(C)]
pub struct PK_LROD_ask_lballs_r_t { _private: [u8; 0] }

/// Options for `PK_LTOPOL_ask_box`.
#[repr(C)]
pub struct PK_LTOPOL_ask_box_o_t { _private: [u8; 0] }
/// Results from `PK_LTOPOL_ask_box`.
#[repr(C)]
pub struct PK_LTOPOL_ask_box_r_t { _private: [u8; 0] }

/// Options for `PK_LTOPOL_ask_class`.
#[repr(C)]
pub struct PK_LTOPOL_ask_class_o_t { _private: [u8; 0] }
/// Results from `PK_LTOPOL_ask_class`.
#[repr(C)]
pub struct PK_LTOPOL_ask_class_r_t { _private: [u8; 0] }

/// Options for `PK_LTOPOL_is`.
#[repr(C)]
pub struct PK_LTOPOL_is_o_t { _private: [u8; 0] }
/// Results from `PK_LTOPOL_is`.
#[repr(C)]
pub struct PK_LTOPOL_is_r_t { _private: [u8; 0] }

// =============================================================================
// Extern functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // Lattice creation and manipulation
    // =========================================================================

    /// Create lattice from streaming graph callback.
    pub fn PK_LATTICE_create_by_graph(
        graph_reader: PK_LATTICE_graph_cb_f_t,
        context: PK_POINTER_t,
        options: *const PK_LATTICE_create_by_graph_o_t,
        results: *mut PK_LATTICE_create_by_graph_r_t,
    ) -> PK_ERROR_code_t;

    /// Create patterned lattice from core cell.
    pub fn PK_LATTICE_make_patterned(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_make_patterned_o_t,
        results: *mut PK_LATTICE_make_patterned_r_t,
    ) -> PK_ERROR_code_t;

    /// Create solid facet body from lattice.
    pub fn PK_LATTICE_make_bodies(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_make_bodies_o_t,
        results: *mut PK_LATTICE_make_bodies_r_t,
    ) -> PK_ERROR_code_t;

    /// Clip lattice against surfaces/faces.
    pub fn PK_LATTICE_clip(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_clip_o_t,
        results: *mut PK_LATTICE_clip_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Lattice queries
    // =========================================================================

    /// Query number of lballs.
    pub fn PK_LATTICE_ask_n_lballs(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_ask_n_lballs_o_t,
        results: *mut PK_LATTICE_ask_n_lballs_r_t,
    ) -> PK_ERROR_code_t;

    /// Query number of lrods.
    pub fn PK_LATTICE_ask_n_lrods(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_ask_n_lrods_o_t,
        results: *mut PK_LATTICE_ask_n_lrods_r_t,
    ) -> PK_ERROR_code_t;

    /// Query owning part.
    pub fn PK_LATTICE_ask_part(
        lattice: PK_LATTICE_t,
        part: *mut PK_PART_t,
    ) -> PK_ERROR_code_t;

    /// Iterate all lballs via callback.
    pub fn PK_LATTICE_do_for_all_lballs(
        lattice: PK_LATTICE_t,
        cb_fn: PK_LBALL_cb_f_t,
        data: PK_POINTER_t,
        thread_safe: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Iterate all lrods via callback.
    pub fn PK_LATTICE_do_for_all_lrods(
        lattice: PK_LATTICE_t,
        cb_fn: PK_LROD_cb_f_t,
        data: PK_POINTER_t,
        thread_safe: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Axis-aligned bounding box of lattice.
    pub fn PK_LATTICE_find_box(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_find_box_o_t,
        results: *mut PK_LATTICE_find_box_r_t,
    ) -> PK_ERROR_code_t;

    /// Non-axis-aligned bounding box of lattice.
    pub fn PK_LATTICE_find_nabox(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_find_nabox_o_t,
        results: *mut PK_LATTICE_find_nabox_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Lball queries
    // =========================================================================

    /// Query adjacent lballs.
    pub fn PK_LBALL_ask_lballs_adj(
        lball: PK_LBALL_t,
        options: *const PK_LBALL_ask_lballs_adj_o_t,
        results: *mut PK_LBALL_ask_lballs_adj_r_t,
    ) -> PK_ERROR_code_t;

    /// Query incident lrods on lball.
    pub fn PK_LBALL_ask_lrods(
        lball: PK_LBALL_t,
        options: *const PK_LBALL_ask_lrods_o_t,
        results: *mut PK_LBALL_ask_lrods_r_t,
    ) -> PK_ERROR_code_t;

    /// Query lball 3D position.
    pub fn PK_LBALL_ask_position(
        lball: PK_LBALL_t,
        options: *const PK_LBALL_ask_position_o_t,
        results: *mut PK_LBALL_ask_position_r_t,
    ) -> PK_ERROR_code_t;

    /// Query lball radius.
    pub fn PK_LBALL_ask_radius(
        lball: PK_LBALL_t,
        options: *const PK_LBALL_ask_radius_o_t,
        results: *mut PK_LBALL_ask_radius_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Lrod queries
    // =========================================================================

    /// Query lrod geometric form.
    pub fn PK_LROD_ask_geometry(
        lrod: PK_LROD_t,
        options: *const PK_LROD_ask_geometry_o_t,
        results: *mut PK_LROD_ask_geometry_r_t,
    ) -> PK_ERROR_code_t;

    /// Query lrod endpoint lballs.
    pub fn PK_LROD_ask_lballs(
        lrod: PK_LROD_t,
        options: *const PK_LROD_ask_lballs_o_t,
        results: *mut PK_LROD_ask_lballs_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Ltopol queries
    // =========================================================================

    /// Bounding box of lattice topology entities.
    pub fn PK_LTOPOL_ask_box(
        n_ltopols: c_int,
        ltopols: *mut PK_LTOPOL_t,
        options: *const PK_LTOPOL_ask_box_o_t,
        results: *mut PK_LTOPOL_ask_box_r_t,
    ) -> PK_ERROR_code_t;

    /// Query class of ltopol element.
    pub fn PK_LTOPOL_ask_class(
        ltopol: PK_LTOPOL_t,
        options: *const PK_LTOPOL_ask_class_o_t,
        results: *mut PK_LTOPOL_ask_class_r_t,
    ) -> PK_ERROR_code_t;

    /// Test if entity is an ltopol.
    pub fn PK_LTOPOL_is(
        may_be_ltopol: PK_LTOPOL_t,
        options: *const PK_LTOPOL_is_o_t,
        results: *mut PK_LTOPOL_is_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_LATTICE_create_by_graph`.
    pub fn PK_LATTICE_create_by_graph_r_f(results: *mut PK_LATTICE_create_by_graph_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_make_patterned`.
    pub fn PK_LATTICE_make_patterned_r_f(results: *mut PK_LATTICE_make_patterned_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_make_bodies`.
    pub fn PK_LATTICE_make_bodies_r_f(results: *mut PK_LATTICE_make_bodies_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_clip`.
    pub fn PK_LATTICE_clip_r_f(results: *mut PK_LATTICE_clip_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_ask_n_lballs`.
    pub fn PK_LATTICE_ask_n_lballs_r_f(results: *mut PK_LATTICE_ask_n_lballs_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_ask_n_lrods`.
    pub fn PK_LATTICE_ask_n_lrods_r_f(results: *mut PK_LATTICE_ask_n_lrods_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_find_box`.
    pub fn PK_LATTICE_find_box_r_f(results: *mut PK_LATTICE_find_box_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_find_nabox`.
    pub fn PK_LATTICE_find_nabox_r_f(results: *mut PK_LATTICE_find_nabox_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LBALL_ask_lballs_adj`.
    pub fn PK_LBALL_ask_lballs_adj_r_f(results: *mut PK_LBALL_ask_lballs_adj_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LBALL_ask_lrods`.
    pub fn PK_LBALL_ask_lrods_r_f(results: *mut PK_LBALL_ask_lrods_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LBALL_ask_position`.
    pub fn PK_LBALL_ask_position_r_f(results: *mut PK_LBALL_ask_position_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LBALL_ask_radius`.
    pub fn PK_LBALL_ask_radius_r_f(results: *mut PK_LBALL_ask_radius_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LROD_ask_geometry`.
    pub fn PK_LROD_ask_geometry_r_f(results: *mut PK_LROD_ask_geometry_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LROD_ask_lballs`.
    pub fn PK_LROD_ask_lballs_r_f(results: *mut PK_LROD_ask_lballs_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LTOPOL_ask_box`.
    pub fn PK_LTOPOL_ask_box_r_f(results: *mut PK_LTOPOL_ask_box_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LTOPOL_ask_class`.
    pub fn PK_LTOPOL_ask_class_r_f(results: *mut PK_LTOPOL_ask_class_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LTOPOL_is`.
    pub fn PK_LTOPOL_is_r_f(results: *mut PK_LTOPOL_is_r_t) -> PK_ERROR_code_t;

}
