//! Option structs of the convergent-modelling (mesh) entry points, with layouts
//! verified against Parasolid V37.01.243 (`pskernel.dll` SHA-256
//! `c900fa3f430fe67c2adb15a50f9604d9812ed81a02c5006b66689dac28263073`).
//!
//! These replace the declarations in `facet.rs` that lacked `o_t_version`
//! (parasolid-re blocker B-R45-1).
//!
//! # Evidence
//!
//! For every struct the kernel's own code gives the layout:
//! - **Field names, order and offsets** (static-observed): each PK wrapper
//!   journals its options field by field, passing the field name as a string and
//!   loading the value from a fixed offset (`mov ecx, [rbx+0x44]` /
//!   `lea rdx, "track_edges"`). Where the decompiler dropped a name, it was read
//!   from the asm.
//! - **Version and defaults** (static-observed): the `options == NULL` branch,
//!   inline in the wrapper or in a separate migration routine, writes the
//!   current version and every default. Older versions are copied field by field
//!   into that image. A version outside the range is deferred as
//!   `PK_ERROR_o_t_version_unknown` (5022).
//! - **Live** (dynamic-observed): the `mesh_options_probe` binary checks every
//!   struct (162 checks: 58 version and 104 field, all as predicted). The
//!   current version is accepted, and version + 1 and version 0 give 5022.
//!   Hand-built v1/v2 buffers are accepted where the kernel migrates them.
//!   Garbage written at the Rust `offset_of!` offset of each validated field gives
//!   that field's error: 908 for a logical, 5014 for a token, 9999 for a NaN
//!   double, 907 for `max_defects`. Garbage in padding bytes gives 0. The Wine
//!   tests `mesh_*` in `parasolid-test` call every entry point end to end and
//!   check one non-default field's visible effect.
//!
//! Raw evidence (decompiles, asm, probe logs): parasolid-re
//! `artifacts/rs/20261021T031231Z-prs-mesh/` (`notes.md`, `raw/`).
//!
//! `PK_LOGICAL_t` is one byte: adjacent logicals pack, and the next wider field
//! aligns to its own size.
//!
//! Each `Default` impl reproduces the kernel's `NULL`-options image. Callers
//! that pass a struct get exactly the kernel defaults, plus whatever they change.
//!
//! A pointer field whose element type was not verified is `*const c_void`, with
//! the header's element type in its doc comment. Leave it null unless you have
//! verified that type.

#![allow(non_camel_case_types)]

use core::mem::{offset_of, size_of};
use std::os::raw::{c_double, c_int, c_void};

use crate::*;

// =============================================================================
// Mesh option enums (values: parasolid-re catalog/pk-enum-values.tsv, from the
// V35 token appendix cross-checked with binary bands; the defaults below are
// additionally static-observed in the NULL-options images).
// =============================================================================

/// `PK_ERROR_on_fail_t`: how `PK_BODY_make_facet_body` treats faces/edges that
/// fail to facet.
pub type PK_ERROR_on_fail_t = c_int;
/// Fail if any face or edge fails to facet (default; static-observed 0x6176).
pub const PK_ERROR_on_fail_any_c: PK_ERROR_on_fail_t = 24950;
/// Fail only if all faces and edges fail to facet (the wrapper tests 0x6177).
pub const PK_ERROR_on_fail_all_c: PK_ERROR_on_fail_t = 24951;

pub type PK_MTOPOL_find_defect_t = c_int;
pub const PK_MTOPOL_find_defect_no_c: PK_MTOPOL_find_defect_t = 25950;
/// Default (static-observed 0x655f).
pub const PK_MTOPOL_find_defect_yes_c: PK_MTOPOL_find_defect_t = 25951;

pub type PK_MFACET_find_defect_t = c_int;
pub const PK_MFACET_find_defect_no_c: PK_MFACET_find_defect_t = 25960;
/// Default (static-observed 0x6569).
pub const PK_MFACET_find_defect_all_c: PK_MFACET_find_defect_t = 25961;

pub type PK_MVERTEX_find_defect_t = c_int;
pub const PK_MVERTEX_find_defect_no_c: PK_MVERTEX_find_defect_t = 26120;
/// Default (static-observed 0x6609).
pub const PK_MVERTEX_find_defect_all_c: PK_MVERTEX_find_defect_t = 26121;

pub type PK_MESH_find_self_int_t = c_int;
pub const PK_MESH_find_self_int_no_c: PK_MESH_find_self_int_t = 25970;
pub const PK_MESH_find_self_int_sx_c: PK_MESH_find_self_int_t = 25971;
pub const PK_MESH_find_self_int_slits_c: PK_MESH_find_self_int_t = 25972;
/// Default (static-observed 0x6575).
pub const PK_MESH_find_self_int_all_c: PK_MESH_find_self_int_t = 25973;

pub type PK_MESH_find_foldover_t = c_int;
/// Default (static-observed 0x657c).
pub const PK_MESH_find_foldover_no_c: PK_MESH_find_foldover_t = 25980;
pub const PK_MESH_find_foldover_all_c: PK_MESH_find_foldover_t = 25981;

pub type PK_MESH_find_disjoint_t = c_int;
/// Default (static-observed 0x6586).
pub const PK_MESH_find_disjoint_no_c: PK_MESH_find_disjoint_t = 25990;
pub const PK_MESH_find_disjoint_yes_c: PK_MESH_find_disjoint_t = 25991;

pub type PK_MFACET_fix_defect_t = c_int;
/// The wrapper accepts only this and `degen` (it tests 0x65d6/0x65d7).
pub const PK_MFACET_fix_defect_none_c: PK_MFACET_fix_defect_t = 26070;
/// Default (static-observed 0x65d7).
pub const PK_MFACET_fix_defect_degen_c: PK_MFACET_fix_defect_t = 26071;

pub type PK_MESH_fix_self_int_t = c_int;
pub const PK_MESH_fix_self_int_no_c: PK_MESH_fix_self_int_t = 26080;
pub const PK_MESH_fix_self_int_sx_c: PK_MESH_fix_self_int_t = 26081;
pub const PK_MESH_fix_self_int_slits_c: PK_MESH_fix_self_int_t = 26082;
/// Default (static-observed 0x65e3).
pub const PK_MESH_fix_self_int_all_c: PK_MESH_fix_self_int_t = 26083;

pub type PK_MESH_fix_foldover_t = c_int;
/// Default (static-observed 0x65cc).
pub const PK_MESH_fix_foldover_none_c: PK_MESH_fix_foldover_t = 26060;
pub const PK_MESH_fix_foldover_all_c: PK_MESH_fix_foldover_t = 26061;

pub type PK_MESH_fix_disjoint_t = c_int;
/// Default (static-observed 0x65ea).
pub const PK_MESH_fix_disjoint_no_c: PK_MESH_fix_disjoint_t = 26090;
pub const PK_MESH_fix_disjoint_split_c: PK_MESH_fix_disjoint_t = 26091;

/// `PK_MESH_fix_result_t` values (catalog `binary_confirmed`).
pub const PK_MESH_fix_result_success_c: PK_MESH_fix_result_t = 26100;
pub const PK_MESH_fix_result_incomplete_c: PK_MESH_fix_result_t = 26101;
pub const PK_MESH_fix_result_failure_c: PK_MESH_fix_result_t = 26102;

pub type PK_MTOPOL_track_t = c_int;
/// Default (static-observed 0x6680).
pub const PK_MTOPOL_track_none_c: PK_MTOPOL_track_t = 26240;
pub const PK_MTOPOL_track_all_c: PK_MTOPOL_track_t = 26241;

pub type PK_MESH_fill_shape_t = c_int;
/// Default (static-observed 0x6504).
pub const PK_MESH_fill_shape_linear_c: PK_MESH_fill_shape_t = 25860;
pub const PK_MESH_fill_shape_refined_c: PK_MESH_fill_shape_t = 25861;
pub const PK_MESH_fill_shape_tangent_c: PK_MESH_fill_shape_t = 25862;
pub const PK_MESH_fill_shape_curvature_c: PK_MESH_fill_shape_t = 25863;

// =============================================================================
// PK_BODY_make_facet_body_o_t
// =============================================================================

/// Current `o_t_version` of [`PK_BODY_make_facet_body_o_t`].
pub const PK_BODY_make_facet_body_o_t_version: c_int = 2;

/// Options for `PK_BODY_make_facet_body`: the **version-2 struct, 80 bytes**.
///
/// Evidence: wrapper `PK_BODY_make_facet_body` @0x1800e2bb0. Its NULL branch
/// writes the v2 defaults. Its v1 branch copies @4..@0x48 into the v2 image, so
/// v1 lacks only `partial_conversion`. Versions other than 1 and 2 give 5022. The
/// journal block names all 15 fields at the offsets asserted below.
/// Live: `mesh_options_probe`, test `mesh_make_facet_body_block`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_make_facet_body_o_t {
    pub o_t_version: c_int,
    pub is_max_facet_width: PK_LOGICAL_t,
    pub max_facet_width: c_double,
    pub is_distance_tolerance: PK_LOGICAL_t,
    pub distance_tolerance: c_double,
    pub is_angular_tolerance: PK_LOGICAL_t,
    pub angular_tolerance: c_double,
    pub is_max_chord_length: PK_LOGICAL_t,
    pub max_chord_length: c_double,
    pub retain_attributes: PK_LOGICAL_t,
    pub retain_groups: PK_LOGICAL_t,
    pub track_faces: PK_LOGICAL_t,
    /// `PK_track_edges_t`; default `PK_track_edges_no_c`.
    pub track_edges: PK_track_edges_t,
    pub return_redundant: PK_LOGICAL_t,
    /// Default [`PK_ERROR_on_fail_any_c`].
    pub partial_conversion: PK_ERROR_on_fail_t,
}

impl Default for PK_BODY_make_facet_body_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_BODY_make_facet_body_o_t_version,
            is_max_facet_width: PK_LOGICAL_false,
            max_facet_width: 0.0,
            is_distance_tolerance: PK_LOGICAL_false,
            distance_tolerance: 0.0,
            is_angular_tolerance: PK_LOGICAL_false,
            angular_tolerance: 0.0,
            is_max_chord_length: PK_LOGICAL_false,
            max_chord_length: 0.0,
            retain_attributes: PK_LOGICAL_false,
            retain_groups: PK_LOGICAL_false,
            track_faces: PK_LOGICAL_false,
            track_edges: PK_track_edges_no_c,
            return_redundant: PK_LOGICAL_false,
            partial_conversion: PK_ERROR_on_fail_any_c,
        }
    }
}

const _: () = {
    type S = PK_BODY_make_facet_body_o_t;
    assert!(size_of::<S>() == 0x50);
    assert!(offset_of!(S, is_max_facet_width) == 0x4);
    assert!(offset_of!(S, max_facet_width) == 0x8);
    assert!(offset_of!(S, is_distance_tolerance) == 0x10);
    assert!(offset_of!(S, distance_tolerance) == 0x18);
    assert!(offset_of!(S, is_angular_tolerance) == 0x20);
    assert!(offset_of!(S, angular_tolerance) == 0x28);
    assert!(offset_of!(S, is_max_chord_length) == 0x30);
    assert!(offset_of!(S, max_chord_length) == 0x38);
    assert!(offset_of!(S, retain_attributes) == 0x40);
    assert!(offset_of!(S, retain_groups) == 0x41);
    assert!(offset_of!(S, track_faces) == 0x42);
    assert!(offset_of!(S, track_edges) == 0x44);
    assert!(offset_of!(S, return_redundant) == 0x48);
    assert!(offset_of!(S, partial_conversion) == 0x4c);
};

// =============================================================================
// PK_MESH_make_bodies_o_t
// =============================================================================

pub const PK_MESH_make_bodies_o_t_version: c_int = 1;

/// Options for `PK_MESH_make_bodies`: **version 1, 24 bytes**.
///
/// Evidence: wrapper @0x180378710. The NULL branch writes `vertex_angle` = 4.0
/// (`movsd` from 0x183f768c0), `allow_disjoint` = 1 and
/// `preferred_body_type` = 0x15e2. Only version 1 is accepted. With argument
/// checking on, the validator rejects a NaN `vertex_angle`, a non-logical
/// `allow_disjoint` (908), and a `preferred_body_type` outside 5601..=5609
/// (5014).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_make_bodies_o_t {
    pub o_t_version: c_int,
    pub vertex_angle: c_double,
    pub allow_disjoint: PK_LOGICAL_t,
    /// `PK_BODY_type_t`; default 5602 (`PK_BODY_type_sheet_c`).
    pub preferred_body_type: PK_BODY_type_t,
}

impl Default for PK_MESH_make_bodies_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_make_bodies_o_t_version,
            vertex_angle: 4.0,
            allow_disjoint: PK_LOGICAL_true,
            preferred_body_type: PK_BODY_type_sheet_c,
        }
    }
}

const _: () = {
    type S = PK_MESH_make_bodies_o_t;
    assert!(size_of::<S>() == 24);
    assert!(offset_of!(S, vertex_angle) == 0x8);
    assert!(offset_of!(S, allow_disjoint) == 0x10);
    assert!(offset_of!(S, preferred_body_type) == 0x14);
};

// =============================================================================
// PK_MTOPOL_make_meshes_o_t
// =============================================================================

pub const PK_MTOPOL_make_meshes_o_t_version: c_int = 4;

/// Options for `PK_MTOPOL_make_meshes`: **version 4, 88 bytes**.
///
/// Evidence: migration `FUN_18038a7e0`, called from the wrapper @0x18038c140.
/// It accepts 1..=4. The NULL/v1/v2 branches write the v4 defaults asserted in
/// [`Default`]. The journal block names the fields at these offsets.
///
/// Live: the defaults turn the two mfacets of a block-face mesh into one orphan
/// mesh. Garbage gives 908 at `allow_disjoint`, 5002 (unknown_class) at
/// `select_mtopol_class`, and 5014 at `select_type`, `map_mfacets` and
/// `map_mvertices`. `n_max_cb_mtopols` is not validated.
///
/// Kernel defect (static-observed): the **v3 branch builds the migrated copy and
/// then returns the caller's pointer** (`mov rax, rcx` @0x18038a9cb). The copy
/// is discarded, so a v3 caller's `map_mvertices` slot (@0x24) is read
/// unmigrated. Use v4. A faithful port should reproduce this.
///
/// The callbacks' signatures are not verified (the journal names their types
/// `PK_MTOPOL_select_cb_f_t` / `PK_MTOPOL_map_cb_f_t`), so leave them `None`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MTOPOL_make_meshes_o_t {
    pub o_t_version: c_int,
    pub allow_disjoint: PK_LOGICAL_t,
    /// Signature not verified; keep `None`.
    pub select_cb: PK_MTOPOL_select_cb_t,
    pub select_data: *mut c_void,
    /// `PK_CLASS_t`; default `PK_CLASS_mfin` (100016).
    pub select_mtopol_class: PK_CLASS_t,
    /// `PK_selector_type_t`; default `PK_selector_type_include_c`.
    pub select_type: PK_selector_type_t,
    pub map_mfacets: PK_MFACET_map_t,
    pub map_mvertices: PK_MVERTEX_map_t,
    /// Signature not verified; keep `None`.
    pub map_cb: PK_MTOPOL_map_cb_f_t,
    pub map_data: *mut c_void,
    /// Default 500.
    pub n_max_cb_mtopols: c_int,
    pub n_faces: c_int,
    pub faces: *const PK_FACE_t,
    /// Header: `const PK_LOGICAL_t *`; element size not verified.
    pub senses: *const c_void,
    /// Header element type not verified.
    pub contributions: *const c_void,
}

impl Default for PK_MTOPOL_make_meshes_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MTOPOL_make_meshes_o_t_version,
            allow_disjoint: PK_LOGICAL_true,
            select_cb: None,
            select_data: core::ptr::null_mut(),
            select_mtopol_class: PK_CLASS_mfin,
            select_type: PK_selector_type_include_c,
            map_mfacets: PK_MFACET_map_no_c,
            map_mvertices: PK_MVERTEX_map_no_c,
            map_cb: None,
            map_data: core::ptr::null_mut(),
            n_max_cb_mtopols: 500,
            n_faces: 0,
            faces: core::ptr::null(),
            senses: core::ptr::null(),
            contributions: core::ptr::null(),
        }
    }
}

const _: () = {
    type S = PK_MTOPOL_make_meshes_o_t;
    assert!(size_of::<S>() == 0x58);
    assert!(offset_of!(S, allow_disjoint) == 0x4);
    assert!(offset_of!(S, select_cb) == 0x8);
    assert!(offset_of!(S, select_data) == 0x10);
    assert!(offset_of!(S, select_mtopol_class) == 0x18);
    assert!(offset_of!(S, select_type) == 0x1c);
    assert!(offset_of!(S, map_mfacets) == 0x20);
    assert!(offset_of!(S, map_mvertices) == 0x24);
    assert!(offset_of!(S, map_cb) == 0x28);
    assert!(offset_of!(S, map_data) == 0x30);
    assert!(offset_of!(S, n_max_cb_mtopols) == 0x38);
    assert!(offset_of!(S, n_faces) == 0x3c);
    assert!(offset_of!(S, faces) == 0x40);
    assert!(offset_of!(S, senses) == 0x48);
    assert!(offset_of!(S, contributions) == 0x50);
};

// =============================================================================
// PK_MESH_make_surf_trimmed_o_t
// =============================================================================

pub const PK_MESH_make_surf_trimmed_o_t_version: c_int = 1;

/// Options for `PK_MESH_make_surf_trimmed`: **version 1, 56 bytes**.
///
/// Evidence: wrapper @0x1803791e0. The NULL branch writes
/// `fitting_tolerance` = 1e-5 (0x183f678b0), `bdry_tolerance` = 0.0 and
/// `bdry_mvertex_angle` = 0.3 (0x183fa7830), `normal_tolerance` = 0.01
/// (0x183f59bc0), and all logicals false. Only version 1 is accepted. The
/// journal reads the offsets asserted below. The validator checks the three
/// logicals (908) and that the doubles are not NaN.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_make_surf_trimmed_o_t {
    pub o_t_version: c_int,
    pub fitting_tolerance: c_double,
    pub have_bdry_tolerance: PK_LOGICAL_t,
    pub bdry_tolerance: c_double,
    pub bdry_mvertex_angle: c_double,
    pub fit_normals: PK_LOGICAL_t,
    pub have_normal_tolerance: PK_LOGICAL_t,
    pub normal_tolerance: c_double,
}

impl Default for PK_MESH_make_surf_trimmed_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_make_surf_trimmed_o_t_version,
            fitting_tolerance: 1e-5,
            have_bdry_tolerance: PK_LOGICAL_false,
            bdry_tolerance: 0.0,
            bdry_mvertex_angle: 0.3,
            fit_normals: PK_LOGICAL_false,
            have_normal_tolerance: PK_LOGICAL_false,
            normal_tolerance: 0.01,
        }
    }
}

const _: () = {
    type S = PK_MESH_make_surf_trimmed_o_t;
    assert!(size_of::<S>() == 56);
    assert!(offset_of!(S, fitting_tolerance) == 0x8);
    assert!(offset_of!(S, have_bdry_tolerance) == 0x10);
    assert!(offset_of!(S, bdry_tolerance) == 0x18);
    assert!(offset_of!(S, bdry_mvertex_angle) == 0x20);
    assert!(offset_of!(S, fit_normals) == 0x28);
    assert!(offset_of!(S, have_normal_tolerance) == 0x29);
    assert!(offset_of!(S, normal_tolerance) == 0x30);
};

// =============================================================================
// Defect finding and fixing
// =============================================================================

/// The `defect_tolerances` block nested in [`PK_MESH_find_defects_o_t`] (@0x18)
/// and [`PK_MESH_fix_defects_o_t`] (@0x10). Its fields are journalled between
/// `PKU_journal_sym('{', "defect_tolerances")` and `'}'`. 48 bytes.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_defect_tolerances_t {
    pub have_degen_tolerance: PK_LOGICAL_t,
    /// Default 1e-8.
    pub degen_tolerance: c_double,
    pub have_slit_tolerance: PK_LOGICAL_t,
    /// Default 1e-8.
    pub slit_tolerance: c_double,
    pub have_height_tolerance: PK_LOGICAL_t,
    /// Default 1e-8.
    pub height_tolerance: c_double,
}

impl Default for PK_MESH_defect_tolerances_t {
    fn default() -> Self {
        Self {
            have_degen_tolerance: PK_LOGICAL_false,
            degen_tolerance: 1e-8,
            have_slit_tolerance: PK_LOGICAL_false,
            slit_tolerance: 1e-8,
            have_height_tolerance: PK_LOGICAL_false,
            height_tolerance: 1e-8,
        }
    }
}

const _: () = {
    type S = PK_MESH_defect_tolerances_t;
    assert!(size_of::<S>() == 0x30);
    assert!(offset_of!(S, degen_tolerance) == 0x8);
    assert!(offset_of!(S, have_slit_tolerance) == 0x10);
    assert!(offset_of!(S, slit_tolerance) == 0x18);
    assert!(offset_of!(S, have_height_tolerance) == 0x20);
    assert!(offset_of!(S, height_tolerance) == 0x28);
};

pub const PK_MESH_find_defects_o_t_version: c_int = 3;

/// Options for `PK_MESH_find_defects`: **version 3, 112 bytes**.
///
/// Evidence: migration `FUN_18036bb40`, called from the wrapper @0x180370d00. It
/// accepts 1..=3 and returns a v3 caller's struct unchanged. Its NULL branch is
/// the [`Default`] image (`tolerance` and the three nested tolerances 1e-8,
/// `max_foldover_angle` 0.2). v1 lacks `mvertex_find_defects` and the nested
/// tolerances. The journal block (the last name, `mesh_find_disjoint`, read from
/// the asm @0x18037111d) names the fields at these offsets.
///
/// This replaces a declaration whose fields did not match the kernel (no
/// version, no `max_defects`, no selector tokens).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_find_defects_o_t {
    pub o_t_version: c_int,
    /// Default 0. The header documents "max_defects < 0" as `bad_option_data`.
    pub max_defects: c_int,
    pub have_tolerance: PK_LOGICAL_t,
    /// Default 1e-8.
    pub tolerance: c_double,
    pub defect_tolerances: PK_MESH_defect_tolerances_t,
    pub mtopol_find_defects: PK_MTOPOL_find_defect_t,
    pub mfacet_find_defects: PK_MFACET_find_defect_t,
    pub mvertex_find_defects: PK_MVERTEX_find_defect_t,
    pub mesh_find_self_ints: PK_MESH_find_self_int_t,
    pub mesh_find_foldovers: PK_MESH_find_foldover_t,
    /// Default 0.2. The header documents values outside [0, pi/2] as
    /// `PK_ERROR_bad_angle`.
    pub max_foldover_angle: c_double,
    pub mesh_find_disjoint: PK_MESH_find_disjoint_t,
}

impl Default for PK_MESH_find_defects_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_find_defects_o_t_version,
            max_defects: 0,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 1e-8,
            defect_tolerances: PK_MESH_defect_tolerances_t::default(),
            mtopol_find_defects: PK_MTOPOL_find_defect_yes_c,
            mfacet_find_defects: PK_MFACET_find_defect_all_c,
            mvertex_find_defects: PK_MVERTEX_find_defect_all_c,
            mesh_find_self_ints: PK_MESH_find_self_int_all_c,
            mesh_find_foldovers: PK_MESH_find_foldover_no_c,
            max_foldover_angle: 0.2,
            mesh_find_disjoint: PK_MESH_find_disjoint_no_c,
        }
    }
}

const _: () = {
    type S = PK_MESH_find_defects_o_t;
    assert!(size_of::<S>() == 0x70);
    assert!(offset_of!(S, max_defects) == 0x4);
    assert!(offset_of!(S, have_tolerance) == 0x8);
    assert!(offset_of!(S, tolerance) == 0x10);
    assert!(offset_of!(S, defect_tolerances) == 0x18);
    assert!(offset_of!(S, mtopol_find_defects) == 0x48);
    assert!(offset_of!(S, mfacet_find_defects) == 0x4c);
    assert!(offset_of!(S, mvertex_find_defects) == 0x50);
    assert!(offset_of!(S, mesh_find_self_ints) == 0x54);
    assert!(offset_of!(S, mesh_find_foldovers) == 0x58);
    assert!(offset_of!(S, max_foldover_angle) == 0x60);
    assert!(offset_of!(S, mesh_find_disjoint) == 0x68);
};

pub const PK_MESH_fix_defects_o_t_version: c_int = 2;

/// Options for `PK_MESH_fix_defects`: **version 2, 96 bytes**.
///
/// Evidence: wrapper @0x180375540, with the migration inline. The NULL branch
/// is the [`Default`] image. v1 has no nested tolerances (its fields after
/// `tolerance` sit at 0x10..0x28). Versions above 2 give 5022. The journal
/// block (last name `mesh_fix_disjoint` from the asm @0x180375b05) names the
/// fields at these offsets. The validator accepts only the two
/// `mfacet_fix_defects` tokens and the two `mesh_fix_foldovers` tokens.
///
/// The header says this is not implemented for meshes attached to faces, so
/// pass an orphan mesh (for example one made by `PK_MTOPOL_make_meshes`).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_fix_defects_o_t {
    pub o_t_version: c_int,
    pub have_tolerance: PK_LOGICAL_t,
    /// Default 1e-8.
    pub tolerance: c_double,
    pub defect_tolerances: PK_MESH_defect_tolerances_t,
    pub mfacet_fix_defects: PK_MFACET_fix_defect_t,
    pub mesh_fix_self_ints: PK_MESH_fix_self_int_t,
    pub mesh_fix_foldovers: PK_MESH_fix_foldover_t,
    /// Default 0.2.
    pub max_foldover_angle: c_double,
    pub mesh_fix_disjoint: PK_MESH_fix_disjoint_t,
}

impl Default for PK_MESH_fix_defects_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_fix_defects_o_t_version,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 1e-8,
            defect_tolerances: PK_MESH_defect_tolerances_t::default(),
            mfacet_fix_defects: PK_MFACET_fix_defect_degen_c,
            mesh_fix_self_ints: PK_MESH_fix_self_int_all_c,
            mesh_fix_foldovers: PK_MESH_fix_foldover_none_c,
            max_foldover_angle: 0.2,
            mesh_fix_disjoint: PK_MESH_fix_disjoint_no_c,
        }
    }
}

const _: () = {
    type S = PK_MESH_fix_defects_o_t;
    assert!(size_of::<S>() == 0x60);
    assert!(offset_of!(S, have_tolerance) == 0x4);
    assert!(offset_of!(S, tolerance) == 0x8);
    assert!(offset_of!(S, defect_tolerances) == 0x10);
    assert!(offset_of!(S, mfacet_fix_defects) == 0x40);
    assert!(offset_of!(S, mesh_fix_self_ints) == 0x44);
    assert!(offset_of!(S, mesh_fix_foldovers) == 0x48);
    assert!(offset_of!(S, max_foldover_angle) == 0x50);
    assert!(offset_of!(S, mesh_fix_disjoint) == 0x58);
};

/// One element of the array `PK_MESH_find_defects` returns in `*defects`
/// (`n_defects` elements): **64 bytes**.
///
/// Evidence: `PKU_journal_MESH_defect_details_18037d180`, the kernel's printer of
/// that array, walks it with stride 0x40 and names these fields at these
/// offsets. `entities` holds 4-byte entity tags, `mtopols` 8-byte
/// [`PK_MTOPOL_t`] handles, `positions` vectors and `uv_params` UV pairs.
///
/// This replaces a declaration that did not match the kernel.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_defect_details_t {
    pub defect: PK_MESH_defect_t,
    pub n_entities: c_int,
    pub entities: *mut PK_ENTITY_t,
    pub n_mtopols: c_int,
    pub mtopols: *mut PK_MTOPOL_t,
    pub n_positions: c_int,
    pub positions: *mut PK_VECTOR_t,
    pub n_uv_params: c_int,
    pub uv_params: *mut PK_UV_t,
}

const _: () = {
    type S = PK_MESH_defect_details_t;
    assert!(size_of::<S>() == 0x40);
    assert!(offset_of!(S, n_entities) == 0x4);
    assert!(offset_of!(S, entities) == 0x8);
    assert!(offset_of!(S, n_mtopols) == 0x10);
    assert!(offset_of!(S, mtopols) == 0x18);
    assert!(offset_of!(S, n_positions) == 0x20);
    assert!(offset_of!(S, positions) == 0x28);
    assert!(offset_of!(S, n_uv_params) == 0x30);
    assert!(offset_of!(S, uv_params) == 0x38);
};

// =============================================================================
// Normals and sharp features
// =============================================================================

pub const PK_MESH_store_normals_o_t_version: c_int = 1;

/// Options for `PK_MESH_store_normals`: **version 1, 56 bytes**.
///
/// Evidence: wrapper @0x180379ce0. The NULL branch writes `replace` = 0x6522
/// (`PK_MESH_replace_normal_all_c`), with every other field zero or null. Only
/// version 1 is accepted. The journal reads the offsets asserted below. The
/// three arrays share the counts `n_mtopols` (@0x14) and `n_mtopol_normals`
/// (@0x28).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_store_normals_o_t {
    pub o_t_version: c_int,
    pub have_mesh_angle: PK_LOGICAL_t,
    pub mesh_angle: c_double,
    pub replace: PK_MESH_replace_normal_t,
    pub n_mtopols: c_int,
    /// Header: `const PK_MTOPOL_t *` ([`PK_MTOPOL_t`] is 8 bytes); not exercised.
    pub mtopols: *const c_void,
    /// Header: `const int *`; not exercised.
    pub mtopol_normal_indices: *const c_void,
    pub n_mtopol_normals: c_int,
    /// Header: `const PK_VECTOR1_t *`; not exercised.
    pub mtopol_normals: *const c_void,
}

impl Default for PK_MESH_store_normals_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_store_normals_o_t_version,
            have_mesh_angle: PK_LOGICAL_false,
            mesh_angle: 0.0,
            replace: PK_MESH_replace_normal_all_c,
            n_mtopols: 0,
            mtopols: core::ptr::null(),
            mtopol_normal_indices: core::ptr::null(),
            n_mtopol_normals: 0,
            mtopol_normals: core::ptr::null(),
        }
    }
}

const _: () = {
    type S = PK_MESH_store_normals_o_t;
    assert!(size_of::<S>() == 56);
    assert!(offset_of!(S, have_mesh_angle) == 0x4);
    assert!(offset_of!(S, mesh_angle) == 0x8);
    assert!(offset_of!(S, replace) == 0x10);
    assert!(offset_of!(S, n_mtopols) == 0x14);
    assert!(offset_of!(S, mtopols) == 0x18);
    assert!(offset_of!(S, mtopol_normal_indices) == 0x20);
    assert!(offset_of!(S, n_mtopol_normals) == 0x28);
    assert!(offset_of!(S, mtopol_normals) == 0x30);
};

pub const PK_MESH_find_sharp_mvxs_o_t_version: c_int = 1;

/// Options for `PK_MESH_find_sharp_mvxs`: **version 1, 8 bytes**.
///
/// Evidence: wrapper @0x180374990. The NULL branch is `{1, want_sharp_mvxs = 1}`
/// and only version 1 is accepted. The validator checks the byte at +4 (908).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_find_sharp_mvxs_o_t {
    pub o_t_version: c_int,
    /// Default true. If false, only the count is returned.
    pub want_sharp_mvxs: PK_LOGICAL_t,
}

impl Default for PK_MESH_find_sharp_mvxs_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_find_sharp_mvxs_o_t_version,
            want_sharp_mvxs: PK_LOGICAL_true,
        }
    }
}

const _: () = {
    assert!(size_of::<PK_MESH_find_sharp_mvxs_o_t>() == 8);
    assert!(offset_of!(PK_MESH_find_sharp_mvxs_o_t, want_sharp_mvxs) == 4);
};

pub const PK_MESH_find_sharp_mfins_o_t_version: c_int = 2;

/// Options for `PK_MESH_find_sharp_mfins`: **version 2, 16 bytes**.
///
/// Evidence: wrapper @0x180373b70. The NULL branch is
/// `{2, want_sharp_mfins = 1, have_sharp_angle = 0, sharp_angle = 0}`. v1 is
/// `{1, want_sharp_mfins}`, which is migrated. Other versions give 5022. The
/// validator checks both logicals (908) and that `sharp_angle` is not NaN, and
/// range-checks it against pi.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_find_sharp_mfins_o_t {
    pub o_t_version: c_int,
    /// Default true.
    pub want_sharp_mfins: PK_LOGICAL_t,
    pub have_sharp_angle: PK_LOGICAL_t,
    pub sharp_angle: c_double,
}

impl Default for PK_MESH_find_sharp_mfins_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_find_sharp_mfins_o_t_version,
            want_sharp_mfins: PK_LOGICAL_true,
            have_sharp_angle: PK_LOGICAL_false,
            sharp_angle: 0.0,
        }
    }
}

const _: () = {
    type S = PK_MESH_find_sharp_mfins_o_t;
    assert!(size_of::<S>() == 16);
    assert!(offset_of!(S, want_sharp_mfins) == 4);
    assert!(offset_of!(S, have_sharp_angle) == 5);
    assert!(offset_of!(S, sharp_angle) == 8);
};

// =============================================================================
// Perimeters, fins, partition enquiry
// =============================================================================

pub const PK_MFACET_find_perimeters_o_t_version: c_int = 4;

/// Options for `PK_MFACET_find_perimeters`: **version 4, 48 bytes**.
///
/// Evidence: wrapper @0x180380ab0, with the migration inline. v1 is
/// `{ver, want_plines, track_type}`. v2 adds `select_cb`, `select_data` and
/// `select_mtopol_class`. v3 adds `select_wire_mfins`, and v4 adds
/// `min_n_mfacets`. The NULL branch is the [`Default`] image. The journal names
/// all fields at these offsets.
///
/// Live: garbage gives 908 at `want_plines`, 5002 at `select_mtopol_class`,
/// 5000 at `select_wire_mfins`, and 39 (bad_value) for `min_n_mfacets` = -1.
/// Garbage in the padding (@5..7, @0xc, @0x2c) gives 0. `track_type` is not
/// validated.
///
/// The result struct `PK_MFACET_find_perimeters_r_t` is still opaque (not
/// verified).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MFACET_find_perimeters_o_t {
    pub o_t_version: c_int,
    pub want_plines: PK_LOGICAL_t,
    pub track_type: PK_MTOPOL_track_t,
    /// Signature not verified; keep `None`.
    pub select_cb: PK_MTOPOL_select_cb_t,
    pub select_data: *mut c_void,
    /// Default `PK_CLASS_mfin` (100016).
    pub select_mtopol_class: PK_CLASS_t,
    /// Default `PK_selector_type_exclude_c`.
    pub select_wire_mfins: PK_selector_type_t,
    /// Default 1.
    pub min_n_mfacets: c_int,
}

impl Default for PK_MFACET_find_perimeters_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MFACET_find_perimeters_o_t_version,
            want_plines: PK_LOGICAL_false,
            track_type: PK_MTOPOL_track_none_c,
            select_cb: None,
            select_data: core::ptr::null_mut(),
            select_mtopol_class: PK_CLASS_mfin,
            select_wire_mfins: PK_selector_type_exclude_c,
            min_n_mfacets: 1,
        }
    }
}

const _: () = {
    type S = PK_MFACET_find_perimeters_o_t;
    assert!(size_of::<S>() == 48);
    assert!(offset_of!(S, want_plines) == 0x4);
    assert!(offset_of!(S, track_type) == 0x8);
    assert!(offset_of!(S, select_cb) == 0x10);
    assert!(offset_of!(S, select_data) == 0x18);
    assert!(offset_of!(S, select_mtopol_class) == 0x20);
    assert!(offset_of!(S, select_wire_mfins) == 0x24);
    assert!(offset_of!(S, min_n_mfacets) == 0x28);
};

pub const PK_FIN_find_mtopols_o_t_version: c_int = 1;

/// Options for `PK_FIN_find_mtopols`: **version 1, 8 bytes**.
///
/// Evidence: wrapper @0x180315870. The NULL branch writes `{1, 0x0101}`, so both
/// logicals default to true. Only version 1 is accepted. The validator checks
/// both bytes (908).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FIN_find_mtopols_o_t {
    pub o_t_version: c_int,
    pub want_mvertices: PK_LOGICAL_t,
    pub want_mfins: PK_LOGICAL_t,
}

impl Default for PK_FIN_find_mtopols_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_FIN_find_mtopols_o_t_version,
            want_mvertices: PK_LOGICAL_true,
            want_mfins: PK_LOGICAL_true,
        }
    }
}

const _: () = {
    type S = PK_FIN_find_mtopols_o_t;
    assert!(size_of::<S>() == 8);
    assert!(offset_of!(S, want_mvertices) == 4);
    assert!(offset_of!(S, want_mfins) == 5);
};

pub const PK_PARTITION_ask_facet_geom_o_t_version: c_int = 1;

/// Options for `PK_PARTITION_ask_facet_geom`: **version 1, 8 bytes**.
///
/// Evidence: wrapper @0x180396970. The NULL branch writes `{1, 0}`, so both
/// logicals default to false. Only version 1 is accepted. The validator checks
/// both bytes (908).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_PARTITION_ask_facet_geom_o_t {
    pub o_t_version: c_int,
    pub want_parts: PK_LOGICAL_t,
    pub want_geoms: PK_LOGICAL_t,
}

impl Default for PK_PARTITION_ask_facet_geom_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_PARTITION_ask_facet_geom_o_t_version,
            want_parts: PK_LOGICAL_false,
            want_geoms: PK_LOGICAL_false,
        }
    }
}

const _: () = {
    type S = PK_PARTITION_ask_facet_geom_o_t;
    assert!(size_of::<S>() == 8);
    assert!(offset_of!(S, want_parts) == 4);
    assert!(offset_of!(S, want_geoms) == 5);
};

// =============================================================================
// Imprinting and hole filling
// =============================================================================

pub const PK_MESH_imprint_vectors_o_t_version: c_int = 2;

/// Options for `PK_MESH_imprint_vectors`: **version 2, 64 bytes**.
///
/// Evidence: wrapper @0x180377110, with the migration inline. The NULL branch is
/// the [`Default`] image (`map_mfacets` = 0x6749, `n_max_cb_mtopols` = 500). v1
/// ends at `n_max_cb_mtopols`. Versions above 2 give 5022. The journal names the
/// fields at these offsets.
///
/// The result struct `PK_MESH_imprint_vectors_r_t` is still opaque (not
/// verified).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_imprint_vectors_o_t {
    pub o_t_version: c_int,
    /// `PK_ITEM_t` tag; default null (0).
    pub destination: PK_ITEM_t,
    pub want_mvertices: PK_LOGICAL_t,
    pub map_mfacets: PK_MFACET_map_t,
    /// Signature not verified; keep `None`.
    pub map_cb: PK_MTOPOL_map_cb_f_t,
    pub map_data: *mut c_void,
    /// Default 500.
    pub n_max_cb_mtopols: c_int,
    pub n_mtopols: c_int,
    /// Header: `const PK_MTOPOL_t *`; not exercised.
    pub mtopols: *const c_void,
    pub n_parameters: c_int,
    /// Header: `const PK_UV_t *`; not exercised.
    pub parameters: *const c_void,
}

impl Default for PK_MESH_imprint_vectors_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_imprint_vectors_o_t_version,
            destination: 0,
            want_mvertices: PK_LOGICAL_false,
            map_mfacets: PK_MFACET_map_no_c,
            map_cb: None,
            map_data: core::ptr::null_mut(),
            n_max_cb_mtopols: 500,
            n_mtopols: 0,
            mtopols: core::ptr::null(),
            n_parameters: 0,
            parameters: core::ptr::null(),
        }
    }
}

const _: () = {
    type S = PK_MESH_imprint_vectors_o_t;
    assert!(size_of::<S>() == 64);
    assert!(offset_of!(S, destination) == 0x4);
    assert!(offset_of!(S, want_mvertices) == 0x8);
    assert!(offset_of!(S, map_mfacets) == 0xc);
    assert!(offset_of!(S, map_cb) == 0x10);
    assert!(offset_of!(S, map_data) == 0x18);
    assert!(offset_of!(S, n_max_cb_mtopols) == 0x20);
    assert!(offset_of!(S, n_mtopols) == 0x24);
    assert!(offset_of!(S, mtopols) == 0x28);
    assert!(offset_of!(S, n_parameters) == 0x30);
    assert!(offset_of!(S, parameters) == 0x38);
};

pub const PK_MESH_fill_holes_o_t_version: c_int = 1;

/// Options for `PK_MESH_fill_holes`: **version 1, 32 bytes**.
///
/// Evidence: wrapper @0x180370230. The journal names the fields at these
/// offsets. The defaults below are the image the default branch writes:
/// `max_n_mfins` 0, no input holes, and `fill_shape` 0x6504
/// (`PK_MESH_fill_shape_linear_c`).
///
/// Live: `max_n_mfins` = 0 (the default) fills no hole by itself. On an open
/// tube (the 32-mfacet side mesh of a faceted cylinder), `max_n_mfins` = 1000
/// fills both ends (60 mfacets), and one boundary mfin in `input_holes` fills
/// that end only (46 mfacets). The argument checker does not validate
/// `fill_shape` or `max_n_mfins`.
///
/// **Kernel defect (static-observed): never pass NULL options or a version
/// other than 1.** That default branch writes its image through a null base
/// register (`mov dword ptr [rdi], 1` and `mov qword ptr [0x4], rdi` with
/// rdi = 0, @0x180370469..0x180370486) instead of to the stack buffer, so it
/// faults. Version 1 skips that branch. A faithful port need not copy the
/// crash.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MESH_fill_holes_o_t {
    pub o_t_version: c_int,
    pub max_n_mfins: c_int,
    pub n_input_holes: c_int,
    /// One mfin on each hole to fill (8-byte [`PK_MFIN_t`]; dynamic-observed).
    pub input_holes: *const PK_MFIN_t,
    pub fill_shape: PK_MESH_fill_shape_t,
}

impl Default for PK_MESH_fill_holes_o_t {
    fn default() -> Self {
        Self {
            o_t_version: PK_MESH_fill_holes_o_t_version,
            max_n_mfins: 0,
            n_input_holes: 0,
            input_holes: core::ptr::null(),
            fill_shape: PK_MESH_fill_shape_linear_c,
        }
    }
}

const _: () = {
    type S = PK_MESH_fill_holes_o_t;
    assert!(size_of::<S>() == 32);
    assert!(offset_of!(S, max_n_mfins) == 0x4);
    assert!(offset_of!(S, n_input_holes) == 0x8);
    assert!(offset_of!(S, input_holes) == 0x10);
    assert!(offset_of!(S, fill_shape) == 0x18);
};

// =============================================================================
// PK_BODY_pick_topols
// =============================================================================

pub const PK_BODY_pick_topols_o_t_version: c_int = 7;

/// Options for `PK_BODY_pick_topols`: **version 7, 224 bytes**.
///
/// Evidence: migration `FUN_18019b1c0`, called from the wrapper @0x18019bcc0. It
/// accepts 1..=7 and returns a v7 caller's struct unchanged. Its NULL branch is
/// the [`Default`] image (`max_faces` 1, `ignore_back_faces` 1, `method` 0x51a5,
/// `pick_approx` 0x56a5, `ray_scale_factor` 0x6950). v1 has no `max_frames`, so
/// its later fields sit 4 bytes lower. The journal printer
/// `candidate_pk_0x18019aec0` names every field at these offsets.
///
/// Live: with argument checking on, a non-logical in any of the seven logicals,
/// or a garbage `pick_approx` or `ray_scale_factor`, gives 5145
/// (bad_combination), while garbage in the nine padding bytes probed gives 0.
/// With `max_faces` = 2, `ignore_back_faces` = false makes a ray through a block
/// return the bottom face as well as the top one.
///
/// This replaces a declaration that did not match the kernel (no version, wrong
/// order).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_pick_topols_o_t {
    pub o_t_version: c_int,
    pub max_faces: c_int,
    pub max_edges: c_int,
    pub max_vertices: c_int,
    pub max_frames: c_int,
    pub near_plane: PK_LOGICAL_t,
    pub far_plane: PK_LOGICAL_t,
    pub is_curve_chord_tol: PK_LOGICAL_t,
    pub curve_chord_tol: c_double,
    pub is_curve_chord_max: PK_LOGICAL_t,
    pub curve_chord_max: c_double,
    pub is_curve_chord_ang: PK_LOGICAL_t,
    pub curve_chord_ang: c_double,
    pub ignore_back_faces: PK_LOGICAL_t,
    pub ignore_excess_entities: PK_LOGICAL_t,
    pub max_edge_dist: c_double,
    pub max_vertex_dist: c_double,
    pub near_point: PK_VECTOR_t,
    pub far_point: PK_VECTOR_t,
    /// `PK_BODY_pick_method_t`; default `PK_BODY_pick_axial_c` (20901).
    pub method: PK_BODY_pick_method_t,
    pub ratio: c_double,
    pub n_geom_classes: c_int,
    /// Header: `const PK_CLASS_t *`; not exercised.
    pub geom_classes: *const c_void,
    /// Default `PK_pick_approximate_yes_c`.
    pub pick_approx: PK_pick_approximate_t,
    pub n_topols_to_pick_from: c_int,
    /// Header: `const PK_TOPOL_t *`; not exercised.
    pub topols_to_pick_from: *const c_void,
    /// Header: `const int *`; not exercised.
    pub topol_indices: *const c_void,
    pub n_bodies_with_scales: c_int,
    /// Header: `const PK_BODY_t *`; not exercised.
    pub bodies_with_scales: *const c_void,
    /// Header: `const PK_scale_factor_t *`; not exercised.
    pub scale_factors: *const c_void,
    /// Default `PK_scale_factor_1x_c`.
    pub ray_scale_factor: PK_scale_factor_t,
}

impl Default for PK_BODY_pick_topols_o_t {
    fn default() -> Self {
        let z: PK_VECTOR_t = [0.0; 3];
        Self {
            o_t_version: PK_BODY_pick_topols_o_t_version,
            max_faces: 1,
            max_edges: 0,
            max_vertices: 0,
            max_frames: 0,
            near_plane: PK_LOGICAL_false,
            far_plane: PK_LOGICAL_false,
            is_curve_chord_tol: PK_LOGICAL_false,
            curve_chord_tol: 0.0,
            is_curve_chord_max: PK_LOGICAL_false,
            curve_chord_max: 0.0,
            is_curve_chord_ang: PK_LOGICAL_false,
            curve_chord_ang: 0.0,
            ignore_back_faces: PK_LOGICAL_true,
            ignore_excess_entities: PK_LOGICAL_false,
            max_edge_dist: 0.0,
            max_vertex_dist: 0.0,
            near_point: z,
            far_point: z,
            method: PK_BODY_pick_axial_c,
            ratio: 0.0,
            n_geom_classes: 0,
            geom_classes: core::ptr::null(),
            pick_approx: PK_pick_approximate_yes_c,
            n_topols_to_pick_from: 0,
            topols_to_pick_from: core::ptr::null(),
            topol_indices: core::ptr::null(),
            n_bodies_with_scales: 0,
            bodies_with_scales: core::ptr::null(),
            scale_factors: core::ptr::null(),
            ray_scale_factor: PK_scale_factor_1x_c,
        }
    }
}

const _: () = {
    type S = PK_BODY_pick_topols_o_t;
    assert!(size_of::<S>() == 0xe0);
    assert!(offset_of!(S, max_faces) == 0x4);
    assert!(offset_of!(S, max_edges) == 0x8);
    assert!(offset_of!(S, max_vertices) == 0xc);
    assert!(offset_of!(S, max_frames) == 0x10);
    assert!(offset_of!(S, near_plane) == 0x14);
    assert!(offset_of!(S, far_plane) == 0x15);
    assert!(offset_of!(S, is_curve_chord_tol) == 0x16);
    assert!(offset_of!(S, curve_chord_tol) == 0x18);
    assert!(offset_of!(S, is_curve_chord_max) == 0x20);
    assert!(offset_of!(S, curve_chord_max) == 0x28);
    assert!(offset_of!(S, is_curve_chord_ang) == 0x30);
    assert!(offset_of!(S, curve_chord_ang) == 0x38);
    assert!(offset_of!(S, ignore_back_faces) == 0x40);
    assert!(offset_of!(S, ignore_excess_entities) == 0x41);
    assert!(offset_of!(S, max_edge_dist) == 0x48);
    assert!(offset_of!(S, max_vertex_dist) == 0x50);
    assert!(offset_of!(S, near_point) == 0x58);
    assert!(offset_of!(S, far_point) == 0x70);
    assert!(offset_of!(S, method) == 0x88);
    assert!(offset_of!(S, ratio) == 0x90);
    assert!(offset_of!(S, n_geom_classes) == 0x98);
    assert!(offset_of!(S, geom_classes) == 0xa0);
    assert!(offset_of!(S, pick_approx) == 0xa8);
    assert!(offset_of!(S, n_topols_to_pick_from) == 0xac);
    assert!(offset_of!(S, topols_to_pick_from) == 0xb0);
    assert!(offset_of!(S, topol_indices) == 0xb8);
    assert!(offset_of!(S, n_bodies_with_scales) == 0xc0);
    assert!(offset_of!(S, bodies_with_scales) == 0xc8);
    assert!(offset_of!(S, scale_factors) == 0xd0);
    assert!(offset_of!(S, ray_scale_factor) == 0xd8);
};

/// One picked entity in [`PK_BODY_pick_topols_r_t`]: **40 bytes**.
///
/// The C type name is not in the binary, so this name is the crate's own. The
/// fields are those of `PKU_journal_BODY_pick_topol_r` @0x18019ad80, which walks
/// each array with stride 0x28 (`entity`, `occurence` [sic], `intersect`,
/// `distance`).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_picked_topol_t {
    pub entity: PK_ENTITY_t,
    /// The kernel spells this field `occurence`.
    pub occurrence: c_int,
    pub intersect: PK_VECTOR_t,
    pub distance: c_double,
}

const _: () = {
    type S = PK_BODY_picked_topol_t;
    assert!(size_of::<S>() == 0x28);
    assert!(offset_of!(S, occurrence) == 0x4);
    assert!(offset_of!(S, intersect) == 0x8);
    assert!(offset_of!(S, distance) == 0x20);
};

/// Result of `PK_BODY_pick_topols`: **56 bytes**.
///
/// Evidence: the wrapper @0x18019bcc0 zeroes exactly 0x38 bytes of `picked`
/// (@0x18019c58d..0x18019c5a5) and journals `n_faces`@0, `faces`@8,
/// `e_faces`@0x10, `n_edges`@0x14, `edges`@0x18, `e_edges`@0x20,
/// `n_vertices`@0x24, `vertices`@0x28 and `e_vertices`@0x30. The `e_*` fields
/// are named but their meaning is not established. Free with
/// `PK_BODY_pick_topols_r_f`.
///
/// This replaces a declaration that did not match the kernel.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_pick_topols_r_t {
    pub n_faces: c_int,
    pub faces: *mut PK_BODY_picked_topol_t,
    pub e_faces: c_int,
    pub n_edges: c_int,
    pub edges: *mut PK_BODY_picked_topol_t,
    pub e_edges: c_int,
    pub n_vertices: c_int,
    pub vertices: *mut PK_BODY_picked_topol_t,
    pub e_vertices: c_int,
}

impl Default for PK_BODY_pick_topols_r_t {
    fn default() -> Self {
        Self {
            n_faces: 0,
            faces: core::ptr::null_mut(),
            e_faces: 0,
            n_edges: 0,
            edges: core::ptr::null_mut(),
            e_edges: 0,
            n_vertices: 0,
            vertices: core::ptr::null_mut(),
            e_vertices: 0,
        }
    }
}

const _: () = {
    type S = PK_BODY_pick_topols_r_t;
    assert!(size_of::<S>() == 0x38);
    assert!(offset_of!(S, faces) == 0x8);
    assert!(offset_of!(S, e_faces) == 0x10);
    assert!(offset_of!(S, n_edges) == 0x14);
    assert!(offset_of!(S, edges) == 0x18);
    assert!(offset_of!(S, e_edges) == 0x20);
    assert!(offset_of!(S, n_vertices) == 0x24);
    assert!(offset_of!(S, vertices) == 0x28);
    assert!(offset_of!(S, e_vertices) == 0x30);
};
