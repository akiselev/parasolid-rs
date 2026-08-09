//! Lattice geometry — V35 construction geometry for graph-based structures.
//!
//! Lattices are graphs of lballs (spherical nodes) and lrods (cylindrical/conical struts).
//! Requires `PK_SESSION_set_facet_geometry` to be enabled.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;
use std::os::raw::{c_double, c_int, c_void};

// =============================================================================
// Callback types
// =============================================================================

/// Callback for streaming graph data to `PK_LATTICE_create_by_graph`.
pub type PK_LATTICE_graph_cb_f_t = Option<unsafe extern "C" fn(PK_POINTER_t) -> PK_ERROR_code_t>;

/// Callback for iterating lballs.
pub type PK_LBALL_cb_f_t =
    Option<unsafe extern "C" fn(PK_LBALL_t, PK_POINTER_t) -> PK_ERROR_code_t>;

/// Callback for iterating lrods.
pub type PK_LROD_cb_f_t = Option<unsafe extern "C" fn(PK_LROD_t, PK_POINTER_t) -> PK_ERROR_code_t>;

// =============================================================================
// Opaque options/results types
// =============================================================================

/// Options for `PK_LATTICE_create_by_graph`.
///
/// Layout (96 bytes) recovered from `PKU_journal_LATTICE_create_by_graph_o`
/// (V37.01.243). `lrod_shape`/`lrod_shape_opts` field names are inferred from
/// the journal group label "lrod_shape_opts" (discriminant 0x6860 = cyl,
/// 0x6861 = cone).
// layout: PKU_journal_LATTICE_create_by_graph_o
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_LATTICE_create_by_graph_o_t {
    pub o_t_version: c_int, // @0
    /// Estimated number of lballs (allocation hint).
    pub lballs_estimate: c_int, // @4
    /// Estimated number of lrods (allocation hint).
    pub lrods_estimate: c_int, // @8
    // @12: 4 bytes padding
    /// Opaque pointer used to free the graph-reader context.
    pub graph_free: *mut c_void, // @16
    /// Whether the graph reader is thread-safe.
    pub thread_safe: PK_LOGICAL_t, // @24
    // @28: 4 bytes padding
    /// Default lball radius.
    pub lball_radius: c_double, // @32
    /// Minimum lball radius mode.
    pub lball_min_radius: c_int, // @40
    /// Default lball blend type.
    pub lball_blend_type: c_int, // @44
    /// Default lball blend size.
    pub lball_blend_size: c_double, // @48
    /// Default lrod cross-section shape (0x6860 = cyl, 0x6861 = cone). [name inferred]
    pub lrod_shape: c_int, // @56
    // @60: 4 bytes padding
    /// Pointer to the lrod-shape options (cyl/cone variant). [name inferred]
    pub lrod_shape_opts: *mut c_void, // @64
    /// Whether `snap_tolerance` is set.
    pub have_snap_tolerance: PK_LOGICAL_t, // @72
    // @76: 4 bytes padding
    /// Snap tolerance for coincident lballs.
    pub snap_tolerance: c_double, // @80
    /// Whether to require a fully connected graph.
    pub require_connected: c_int, // @88
    /// Whether to merge duplicate lballs/lrods.
    pub merge_duplicates: c_int, // @92
}

impl Default for PK_LATTICE_create_by_graph_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            lballs_estimate: 0,
            lrods_estimate: 0,
            graph_free: std::ptr::null_mut(),
            thread_safe: PK_LOGICAL_false,
            lball_radius: 0.0,
            lball_min_radius: 0,
            lball_blend_type: 0,
            lball_blend_size: 0.0,
            lrod_shape: 0,
            lrod_shape_opts: std::ptr::null_mut(),
            have_snap_tolerance: PK_LOGICAL_false,
            snap_tolerance: 0.0,
            require_connected: 0,
            merge_duplicates: 0,
        }
    }
}
/// Results from `PK_LATTICE_create_by_graph`.
#[repr(C)]
pub struct PK_LATTICE_create_by_graph_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_make_patterned`.
///
/// Layout (320 bytes) recovered from `PKU_journal_LATTICE_make_patterned_o`
/// (V37.01.243). The `pattern_def`/`bound`/`pattern_form`/`callback` regions are
/// nested structs whose internal layouts are not yet decoded; they are held as
/// correctly-sized opaque byte blocks (block sizes come from journal offset
/// deltas, and the total 320-byte size is confirmed by the last field read at
/// puVar25[0x4e] = @312, an 8-byte pointer ending at @320).
// layout: PKU_journal_LATTICE_make_patterned_o
#[repr(C)]
#[derive(Clone)]
pub struct PK_LATTICE_make_patterned_o_t {
    pub o_t_version: c_int, // @0
    _pad4: [u8; 4],         // @4
    /// Pattern definition block (journal helper @0x08; internals not decoded).
    _pattern_def: [u8; 144], // @8..152
    /// Pattern bound block (PKU_journal_pattern_bound; internals not decoded).
    _bound: [u8; 104], // @152..256
    /// Lrod clip mode.
    pub lrod_clip: c_int, // @256
    /// Lrod coincidence mode.
    pub lrod_coi: c_int, // @260
    /// Lball radius mode (0x6900 / 0x6901).
    pub lball_radius: c_int, // @264
    /// Whether to merge duplicates.
    pub merge_duplicates: c_int, // @268
    /// Duplicate merge tolerance.
    pub duplicate_tolerance: c_double, // @272
    /// Pattern form block (PKU_journal_pattern_form; internals not decoded).
    _pattern_form: [u8; 16], // @280..296
    /// Pattern callback block (PKU_journal_pattern_callback; 4 ints + pointer).
    _callback: [u8; 24], // @296..320
}

impl Default for PK_LATTICE_make_patterned_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            _pad4: [0; 4],
            _pattern_def: [0; 144],
            _bound: [0; 104],
            lrod_clip: 0,
            lrod_coi: 0,
            lball_radius: 0,
            merge_duplicates: 0,
            duplicate_tolerance: 0.0,
            _pattern_form: [0; 16],
            _callback: [0; 24],
        }
    }
}
/// Results from `PK_LATTICE_make_patterned`.
#[repr(C)]
pub struct PK_LATTICE_make_patterned_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_make_bodies`.
///
/// Layout (48 bytes) recovered from `PK_LATTICE_make_bodies` inline journaling
/// (V37.01.243). The `ijkbox` element count (6 ints, min/max per i/j/k axis) is
/// inferred; it is the trailing field so a wrong count would only affect total
/// size.
// layout: PKU_journal (inline) PK_LATTICE_make_bodies
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_LATTICE_make_bodies_o_t {
    pub o_t_version: c_int, // @0
    // @4: 4 bytes padding (double is 8-aligned)
    /// Facet distance tolerance.
    pub distance_tolerance: c_double, // @8
    /// Whether `ijkbox` is set.
    pub have_ijkbox: PK_LOGICAL_t, // @16
    /// Integer i/j/k index box (min/max per axis). [count inferred]
    pub ijkbox: [c_int; 6], // @20
}

impl Default for PK_LATTICE_make_bodies_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            distance_tolerance: 0.0,
            have_ijkbox: PK_LOGICAL_false,
            ijkbox: [0; 6],
        }
    }
}
/// Results from `PK_LATTICE_make_bodies`.
#[repr(C)]
pub struct PK_LATTICE_make_bodies_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_clip`.
///
/// Layout (88 bytes) recovered from `PKU_journal_LATTICE_clip_o` (V37.01.243).
/// `fence` shares its element count with `n_clip_entities`.
// layout: PKU_journal_LATTICE_clip_o
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_LATTICE_clip_o_t {
    pub o_t_version: c_int, // @0
    /// Number of clip entities (also the length of `fence`).
    pub n_clip_entities: c_int, // @4
    /// Array of clipping entities (surfaces/faces).
    pub clip_entities: *const PK_ENTITY_t, // @8
    /// Per-clip-entity fence sense array.
    pub fence: *const c_int, // @16
    /// Lrod clip mode.
    pub lrod_clip: c_int, // @24
    // @28: 4 bytes padding
    /// Minimum surviving lrod length.
    pub lrod_min_length: c_double, // @32
    /// Lrod coincidence mode.
    pub lrod_coi: c_int, // @40
    // @44: 4 bytes padding
    /// Lrod coincidence tolerance.
    pub lrod_coi_tolerance: c_double, // @48
    /// Lball radius mode.
    pub lball_radius: c_int, // @56
    /// Whether to output clipped lballs.
    pub want_clipped_lballs: PK_LOGICAL_t, // @60
    /// Whether to merge duplicates.
    pub merge_duplicates: c_int, // @64
    // @68: 4 bytes padding
    /// Duplicate merge tolerance.
    pub duplicate_tolerance: c_double, // @72
    /// Whether to require connected output.
    pub require_connected: c_int, // @80
}

impl Default for PK_LATTICE_clip_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_clip_entities: 0,
            clip_entities: std::ptr::null(),
            fence: std::ptr::null(),
            lrod_clip: 0,
            lrod_min_length: 0.0,
            lrod_coi: 0,
            lrod_coi_tolerance: 0.0,
            lball_radius: 0,
            want_clipped_lballs: PK_LOGICAL_false,
            merge_duplicates: 0,
            duplicate_tolerance: 0.0,
            require_connected: 0,
        }
    }
}
/// Results from `PK_LATTICE_clip`.
#[repr(C)]
pub struct PK_LATTICE_clip_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_ask_n_lballs`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lballs_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LATTICE_ask_n_lballs`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lballs_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_ask_n_lrods`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lrods_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LATTICE_ask_n_lrods`.
#[repr(C)]
pub struct PK_LATTICE_ask_n_lrods_r_t {
    _private: [u8; 0],
}

// =============================================================================
// Core / form / connectivity / regions — [decompile-observed] V37.01.243
//
// These five had NO documented prototype in any reference (`pk-reference.tsv`
// lists them MISSING), so arity and pointer-ness come from decompiling the
// exports themselves, and the field names come from each function's own
// journalling code (the `PKU_journal_*` method — the kernel names every field
// it writes to the journal).
//
//   PK_LATTICE_ask_core          @1803466c0   (lattice, options, results)
//   PK_LATTICE_ask_form          @180347260   (lattice, options, results)
//   PK_LATTICE_ask_connectivity  @180345ed0   (lattice, options, results)
//   PK_LATTICE_ask_regions       @180348eb0   (lattice, options, results)
//   PK_LATTICE_create_by_core    @18034b980   (options, results)
//
// Only `PK_LATTICE_ask_regions_r_t` is fully mapped. The others keep opaque
// bodies on purpose: the journal names their leading fields but not the full
// union tails, and inventing the rest is exactly the failure that produced a
// wholly wrong `PK_ERROR_*` table. Callers must zero a generous buffer and
// treat these as unvalidated until a live probe exists (Stage 15).
// =============================================================================

/// Options for `PK_LATTICE_ask_core`. Journal shows a single consumed field,
/// `o_t_version`, which the kernel requires to be 1.
#[repr(C)]
pub struct PK_LATTICE_ask_core_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LATTICE_ask_core`.
///
/// Partially mapped: `r_t_version` @0, then a core-kind token @4 (the empty
/// result writes `0x6b30`), then a union whose `data.implicit_volume` arm the
/// journal names as `implicit_surf`, `basis_set`, `front_offset`,
/// `back_offset`. Tail not mapped — treat as opaque.
#[repr(C)]
pub struct PK_LATTICE_ask_core_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_ask_form` (`o_t_version` only).
#[repr(C)]
pub struct PK_LATTICE_ask_form_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LATTICE_ask_form`. Journal names only `r_t_version`.
#[repr(C)]
pub struct PK_LATTICE_ask_form_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_ask_connectivity` (`o_t_version` only).
#[repr(C)]
pub struct PK_LATTICE_ask_connectivity_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LATTICE_ask_connectivity`.
///
/// Journal field order: `r_t_version`, `lattice_connectivity`, `n_components`
/// — all int-sized, so the head is `{i32, i32, i32}`. Kept opaque pending a
/// live probe confirming there is no tail.
#[repr(C)]
pub struct PK_LATTICE_ask_connectivity_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_ask_regions`.
///
/// Layout from the caller's own journalling: `o_t_version` @0 then
/// `want_senses` @4, journalled via `PKU_journal_LOGICAL` on `param_2[1]`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_LATTICE_ask_regions_o_t {
    pub o_t_version: c_int,
    pub want_senses: PK_LOGICAL_t,
}

/// Results from `PK_LATTICE_ask_regions` — **fully mapped** from
/// `PKU_journal_LATTICE_ask_regions_r` (V37.01.243).
///
/// The helper walks `param_1[0]` as `r_t_version`, `param_1[1]` as the shared
/// element count for all three arrays, then three pointers at `param_1+2`,
/// `param_1+4`, `param_1+6` (byte offsets 8/16/24). `senses` is indexed by
/// byte, not by 4, so it is an array of one-byte logicals; `regions` and
/// `frames` are indexed `* 4` and tagged `REGION` / `FRAME` respectively.
///
/// Free with `PK_LATTICE_ask_regions_r_f`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_LATTICE_ask_regions_r_t {
    pub r_t_version: c_int,          // @0
    pub n_regions: c_int,            // @4  — length of all three arrays
    pub regions: *mut PK_REGION_t,   // @8
    pub senses: *mut PK_LOGICAL_t,   // @16 — one byte per element
    pub frames: *mut PK_ENTITY_t,    // @24 — PK_FRAME_t tags
}

/// Options for `PK_LATTICE_create_by_core`.
///
/// Journal names an implicit-volume core (`implicit_surf`, `basis_set`,
/// `front_offset`, `back_offset`) plus a `bound` field well into the struct
/// (`options + 0x2a` ints). Not fully mapped — opaque.
#[repr(C)]
pub struct PK_LATTICE_create_by_core_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LATTICE_create_by_core`. Journal names `r_t_version`.
#[repr(C)]
pub struct PK_LATTICE_create_by_core_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_find_box`.
#[repr(C)]
pub struct PK_LATTICE_find_box_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LATTICE_find_box`.
#[repr(C)]
pub struct PK_LATTICE_find_box_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LATTICE_find_nabox`.
///
/// Layout (168 bytes) recovered from `PKU_journal_LATTICE_find_nabox_o`
/// (V37.01.243). `axis1` = `PK_AXIS1_sf_t` (48 bytes, journalled by
/// `PKU_journal_AXIS1_sf`); `axis2` = `PK_AXIS2_sf_t` (72 bytes,
/// `PKU_journal_AXIS2_sf`). `ijkbox` element count (6 ints) inferred (trailing
/// field).
// layout: PKU_journal_LATTICE_find_nabox_o
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_LATTICE_find_nabox_o_t {
    pub o_t_version: c_int, // @0
    /// Whether `axis1` is set.
    pub have_axis1: PK_LOGICAL_t, // @4
    /// First constraining axis (location + direction).
    pub axis1: PK_AXIS1_sf_t, // @8  (48 bytes)
    /// Whether `axis2` is set.
    pub have_axis2: PK_LOGICAL_t, // @56
    // @60: 4 bytes padding (AXIS2 is 8-aligned)
    /// Second constraining frame (location + axis + ref_direction).
    pub axis2: PK_AXIS2_sf_t, // @64 (72 bytes)
    /// Whether `ijkbox` is set.
    pub have_ijkbox: PK_LOGICAL_t, // @136
    /// Integer i/j/k index box (min/max per axis). [count inferred]
    pub ijkbox: [c_int; 6], // @140
}

impl Default for PK_LATTICE_find_nabox_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            have_axis1: PK_LOGICAL_false,
            axis1: PK_AXIS1_sf_t {
                location: [0.0; 3],
                axis: [0.0; 3],
            },
            have_axis2: PK_LOGICAL_false,
            axis2: PK_AXIS2_sf_t {
                location: [0.0; 3],
                axis: [0.0; 3],
                ref_direction: [0.0; 3],
            },
            have_ijkbox: PK_LOGICAL_false,
            ijkbox: [0; 6],
        }
    }
}
/// Results from `PK_LATTICE_find_nabox`.
#[repr(C)]
pub struct PK_LATTICE_find_nabox_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LBALL_ask_lballs_adj`.
#[repr(C)]
pub struct PK_LBALL_ask_lballs_adj_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LBALL_ask_lballs_adj`.
#[repr(C)]
pub struct PK_LBALL_ask_lballs_adj_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LBALL_ask_lrods`.
#[repr(C)]
pub struct PK_LBALL_ask_lrods_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LBALL_ask_lrods`.
#[repr(C)]
pub struct PK_LBALL_ask_lrods_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LBALL_ask_position`.
#[repr(C)]
pub struct PK_LBALL_ask_position_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LBALL_ask_position`.
#[repr(C)]
pub struct PK_LBALL_ask_position_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LBALL_ask_radius`.
#[repr(C)]
pub struct PK_LBALL_ask_radius_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LBALL_ask_radius`.
#[repr(C)]
pub struct PK_LBALL_ask_radius_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LROD_ask_geometry`.
#[repr(C)]
pub struct PK_LROD_ask_geometry_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LROD_ask_geometry`.
#[repr(C)]
pub struct PK_LROD_ask_geometry_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LROD_ask_lballs`.
#[repr(C)]
pub struct PK_LROD_ask_lballs_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LROD_ask_lballs`.
#[repr(C)]
pub struct PK_LROD_ask_lballs_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LTOPOL_ask_box`.
#[repr(C)]
pub struct PK_LTOPOL_ask_box_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LTOPOL_ask_box`.
#[repr(C)]
pub struct PK_LTOPOL_ask_box_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LTOPOL_ask_class`.
#[repr(C)]
pub struct PK_LTOPOL_ask_class_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LTOPOL_ask_class`.
#[repr(C)]
pub struct PK_LTOPOL_ask_class_r_t {
    _private: [u8; 0],
}

/// Options for `PK_LTOPOL_is`.
#[repr(C)]
pub struct PK_LTOPOL_is_o_t {
    _private: [u8; 0],
}
/// Results from `PK_LTOPOL_is`.
#[repr(C)]
pub struct PK_LTOPOL_is_r_t {
    _private: [u8; 0],
}

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
    pub fn PK_LATTICE_ask_part(lattice: PK_LATTICE_t, part: *mut PK_PART_t) -> PK_ERROR_code_t;

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

    // ---- Core / form / connectivity / regions ------------------------------
    // [decompile-observed] arity and pointer-ness; see the struct block above.

    /// Ask the core geometry a lattice was built from.
    pub fn PK_LATTICE_ask_core(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_ask_core_o_t,
        results: *mut PK_LATTICE_ask_core_r_t,
    ) -> PK_ERROR_code_t;

    /// Ask a lattice's form.
    pub fn PK_LATTICE_ask_form(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_ask_form_o_t,
        results: *mut PK_LATTICE_ask_form_r_t,
    ) -> PK_ERROR_code_t;

    /// Ask a lattice's connectivity and component count.
    pub fn PK_LATTICE_ask_connectivity(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_ask_connectivity_o_t,
        results: *mut PK_LATTICE_ask_connectivity_r_t,
    ) -> PK_ERROR_code_t;

    /// Ask the regions a lattice occupies, optionally with senses and frames.
    pub fn PK_LATTICE_ask_regions(
        lattice: PK_LATTICE_t,
        options: *const PK_LATTICE_ask_regions_o_t,
        results: *mut PK_LATTICE_ask_regions_r_t,
    ) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_ask_regions`.
    pub fn PK_LATTICE_ask_regions_r_f(
        results: *mut PK_LATTICE_ask_regions_r_t,
    ) -> PK_ERROR_code_t;

    /// Create a lattice from a core geometry definition.
    ///
    /// Two arguments only — the lattice comes back in `results`, matching
    /// `PK_BODY_create_implicit`'s shape rather than the usual
    /// `(entity, options, results)`.
    pub fn PK_LATTICE_create_by_core(
        options: *const PK_LATTICE_create_by_core_o_t,
        results: *mut PK_LATTICE_create_by_core_r_t,
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
    pub fn PK_LATTICE_create_by_graph_r_f(
        results: *mut PK_LATTICE_create_by_graph_r_t,
    ) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_make_patterned`.
    pub fn PK_LATTICE_make_patterned_r_f(
        results: *mut PK_LATTICE_make_patterned_r_t,
    ) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_make_bodies`.
    pub fn PK_LATTICE_make_bodies_r_f(results: *mut PK_LATTICE_make_bodies_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_clip`.
    pub fn PK_LATTICE_clip_r_f(results: *mut PK_LATTICE_clip_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_ask_n_lballs`.
    pub fn PK_LATTICE_ask_n_lballs_r_f(
        results: *mut PK_LATTICE_ask_n_lballs_r_t,
    ) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_ask_n_lrods`.
    pub fn PK_LATTICE_ask_n_lrods_r_f(results: *mut PK_LATTICE_ask_n_lrods_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_find_box`.
    pub fn PK_LATTICE_find_box_r_f(results: *mut PK_LATTICE_find_box_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LATTICE_find_nabox`.
    pub fn PK_LATTICE_find_nabox_r_f(results: *mut PK_LATTICE_find_nabox_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_LBALL_ask_lballs_adj`.
    pub fn PK_LBALL_ask_lballs_adj_r_f(
        results: *mut PK_LBALL_ask_lballs_adj_r_t,
    ) -> PK_ERROR_code_t;

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
