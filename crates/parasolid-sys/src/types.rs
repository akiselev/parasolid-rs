//! Primitive types used throughout the Parasolid API.

use std::os::raw::{c_double, c_int};

// =============================================================================
// Tag types — opaque integer handles to Parasolid entities
// =============================================================================

/// Generic entity tag. All specific tag types are aliases.
pub type PK_ENTITY_t = c_int;

/// Null entity constant — represents "no entity".
pub const PK_ENTITY_null: PK_ENTITY_t = 0;

// Topological entity tags
pub type PK_BODY_t = PK_ENTITY_t;
pub type PK_REGION_t = PK_ENTITY_t;
pub type PK_SHELL_t = PK_ENTITY_t;
pub type PK_FACE_t = PK_ENTITY_t;
pub type PK_LOOP_t = PK_ENTITY_t;
pub type PK_FIN_t = PK_ENTITY_t;
pub type PK_EDGE_t = PK_ENTITY_t;
pub type PK_VERTEX_t = PK_ENTITY_t;

// Geometric entity tags
pub type PK_GEOM_t = PK_ENTITY_t;
pub type PK_SURF_t = PK_ENTITY_t;
pub type PK_CURVE_t = PK_ENTITY_t;
pub type PK_POINT_t = PK_ENTITY_t;
pub type PK_PLANE_t = PK_ENTITY_t;
pub type PK_CYLL_t = PK_ENTITY_t;
pub type PK_CONE_t = PK_ENTITY_t;
pub type PK_SPHERE_t = PK_ENTITY_t;
pub type PK_TORUS_t = PK_ENTITY_t;
pub type PK_BSURF_t = PK_ENTITY_t;
pub type PK_FSURF_t = PK_ENTITY_t;
pub type PK_SSURF_t = PK_ENTITY_t;
pub type PK_SPUN_t = PK_ENTITY_t;
pub type PK_SWEPT_t = PK_ENTITY_t;
pub type PK_BLENDSF_t = PK_ENTITY_t;
pub type PK_OFFSET_t = PK_ENTITY_t;
pub type PK_LINE_t = PK_ENTITY_t;
pub type PK_CIRCLE_t = PK_ENTITY_t;
pub type PK_ELLIPSE_t = PK_ENTITY_t;
pub type PK_BCURVE_t = PK_ENTITY_t;
pub type PK_FCURVE_t = PK_ENTITY_t;
pub type PK_SCURVE_t = PK_ENTITY_t;
pub type PK_ICURVE_t = PK_ENTITY_t;
pub type PK_TCURVE_t = PK_ENTITY_t;
pub type PK_CPCURVE_t = PK_ENTITY_t;
pub type PK_PLINE_t = PK_ENTITY_t;

// Session/application entity tags
pub type PK_TOPOL_t = PK_ENTITY_t;
pub type PK_PART_t = PK_ENTITY_t;
pub type PK_ASSEMBLY_t = PK_ENTITY_t;
pub type PK_INSTANCE_t = PK_ENTITY_t;
pub type PK_TRANSF_t = PK_ENTITY_t;
pub type PK_ATTRIB_t = PK_ENTITY_t;
pub type PK_ATTDEF_t = PK_ENTITY_t;
pub type PK_GROUP_t = PK_ENTITY_t;
pub type PK_PARTITION_t = PK_ENTITY_t;
pub type PK_MARK_t = PK_ENTITY_t;
pub type PK_PMARK_t = PK_ENTITY_t;
pub type PK_DELTA_t = PK_ENTITY_t;
pub type PK_BB_t = PK_ENTITY_t;

// Convergent modeling tags
pub type PK_MESH_t = PK_ENTITY_t;

/// A mesh-topology handle (mfacet, mfin or mvertex): **8 bytes, passed by
/// value**, not a 4-byte entity tag. Mesh topology is not made of nodes, so it
/// has no tag of its own.
///
/// Evidence (V37.01.243, static-observed):
/// - `PK_MESH_find_sharp_mvxs` @0x180374990 and `PK_MESH_find_sharp_mfins`
///   @0x180373b70 allocate `n << 3` bytes for their output arrays and store one
///   `u64` per element: the low word is `DS_tag_of_node(...)` and the high word a
///   negated index;
/// - `PK_MVERTEX_ask_position` @0x180390fc0 reads the full 64-bit `RCX`
///   (`CONCAT44(in_register_0000000c, mvertex)`);
/// - the kernel's defect-details printer strides `mtopols` arrays by 8.
///
/// Documented: the V35 manual (ch084) says to "use `PK_MTOPOL_null_m` to create a
/// null `PK_MTOPOL_t`" (a macro value, which fits a struct, not a tag).
///
/// Live (dynamic-observed): handles returned by `PK_MESH_find_sharp_mvxs` give
/// correct positions from `PK_MVERTEX_ask_position` (test
/// `mesh_sharp_mvxs_and_mfins`). Evidence: parasolid-re
/// `artifacts/rs/20261021T031231Z-prs-mesh/`.
///
/// The words stay opaque because their meaning is only partly established. The
/// value of `PK_MTOPOL_null_m` is not known, so there is no null constant.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PK_MTOPOL_t {
    pub words: [c_int; 2],
}
const _: () = assert!(core::mem::size_of::<PK_MTOPOL_t>() == 8);
pub type PK_MFACET_t = PK_MTOPOL_t;
pub type PK_MFIN_t = PK_MTOPOL_t;
pub type PK_MVERTEX_t = PK_MTOPOL_t;

// SP-curve (surface-parametric curve) tag
pub type PK_SPCURVE_t = PK_ENTITY_t;

// Frame tag (coordinate frame attached to topology, class 0xe6)
pub type PK_FRAME_t = PK_ENTITY_t;

// Lattice geometry tags (V35 construction geometry)
pub type PK_LATTICE_t = PK_ENTITY_t;
pub type PK_LBALL_t = PK_ENTITY_t;
pub type PK_LROD_t = PK_ENTITY_t;
pub type PK_LTOPOL_t = PK_ENTITY_t;

// Application items
pub type PK_APPITEM_t = PK_ENTITY_t;

// =============================================================================
// Logical type
// =============================================================================

/// `PK_LOGICAL_t` is **one byte** in this binary (Parasolid V37.01.243, SHA-256
/// `c900fa3f…63073`). The V35 header pages print `typedef int`, which is wrong for
/// this build: the kernel's option-migration routines read and write logical fields
/// as single bytes (parasolid-re `artifacts/port/oracle/README.md` "ABI basis",
/// `catalog/pk-validated-abi.md`), a value above 1 is refused with
/// `PK_ERROR_not_a_logical` (908), and `PK_SESSION_ask_check_arguments` writes only
/// the low byte of its output (runtime test `abi_logical_is_one_byte`).
/// Grade: static-observed + dynamic-observed.
///
/// Consequences of the old `c_int` declaration: struct fields after two adjacent
/// logicals were mis-offset, and every logical *output* left three bytes of caller
/// garbage that a `!= 0` test read as `true`.
pub type PK_LOGICAL_t = u8;
pub const PK_LOGICAL_true: PK_LOGICAL_t = 1;
pub const PK_LOGICAL_false: PK_LOGICAL_t = 0;

// =============================================================================
// Geometric primitives
// =============================================================================

/// 3D vector [x, y, z].
pub type PK_VECTOR_t = [c_double; 3];

/// 1D parameter interval [low, high].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_INTERVAL_t {
    pub low: c_double,
    pub high: c_double,
}

/// 3D axis-aligned bounding box.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BOX_t {
    pub coord: [c_double; 6], // [xmin, ymin, zmin, xmax, ymax, zmax]
}

/// 2D parameter-space point [u, v].
pub type PK_UV_t = [c_double; 2];

/// 2D parameter-space bounding box.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_UVBOX_t {
    pub param: [c_double; 4], // [umin, vmin, umax, vmax]
}

// =============================================================================
// Error code type
// =============================================================================

pub type PK_ERROR_code_t = c_int;

// =============================================================================
// Class identifiers
// =============================================================================

pub type PK_CLASS_t = c_int;

// Class token values. Sources and tags:
//   [probed]  directly observed on V37.01.243 via PK_ENTITY_ask_class on a known
//             entity (here or in the parasolid-re oracle harness);
//   [vendor]  Siemens "PK Token Codes (Numeric)" appendix (q-solid V35 mirror,
//             parasolid-re `artifacts/enum-values/pk-tokens-v35-qsolid.tsv`),
//             with the 3001..3009 / 4001..4011 bands confirmed in the binary;
//   [unknown] no value in any source — negative placeholder, never returned by
//             the kernel, do not dispatch on it.
//
// Correction 2026-10-16: the curve band 3004..3009 and the surface values
// 4007/4008/4011 were previously filled in by guessing the order ("[family]"),
// and were wrong. Raw 3004 is an INTERSECTION curve (icurve), not a b-curve —
// so the SSI suite had labelled every intersection curve `Bcurve`. Raw 3005 is
// the b-curve. Root classes 500..502, 1003 and the non-topology classes were
// also guesses or unknown; the vendor values agree with the earlier hierarchy
// probe, which found live classes at 6002/6003/6005 (= ki_list/attdef/group).

// Root / structural classes
pub const PK_CLASS_null: PK_CLASS_t = 500; // [vendor]
pub const PK_CLASS_class: PK_CLASS_t = 501; // [vendor] (was 500 [guess])
pub const PK_CLASS_primitive: PK_CLASS_t = 502; // [vendor] (was 501 [guess])
pub const PK_CLASS_mark: PK_CLASS_t = 506; // [vendor]
pub const PK_CLASS_pmark: PK_CLASS_t = 507; // [vendor]
pub const PK_CLASS_partition: PK_CLASS_t = 508; // [vendor]
pub const PK_CLASS_item: PK_CLASS_t = 523; // [vendor] (was 1003 [guess] — 1003 is attrib)
pub const PK_CLASS_appitem: PK_CLASS_t = 524; // [vendor]
pub const PK_CLASS_entity: PK_CLASS_t = 1000; // [probed]
pub const PK_CLASS_geom: PK_CLASS_t = 1001; // [probed]
pub const PK_CLASS_topol: PK_CLASS_t = 1002; // [probed]
pub const PK_CLASS_attrib: PK_CLASS_t = 1003; // [vendor] third entity subclass

// Geometry family roots
pub const PK_CLASS_curve: PK_CLASS_t = 2002; // [probed]
pub const PK_CLASS_surf: PK_CLASS_t = 2003; // [probed]
pub const PK_CLASS_transf: PK_CLASS_t = 2004; // [vendor]
pub const PK_CLASS_nabox: PK_CLASS_t = 2005; // [vendor]
pub const PK_CLASS_point: PK_CLASS_t = 2501; // [probed]

// Curves (3001..3009)
pub const PK_CLASS_line: PK_CLASS_t = 3001; // [probed]
pub const PK_CLASS_circle: PK_CLASS_t = 3002; // [probed]
pub const PK_CLASS_ellipse: PK_CLASS_t = 3003; // [vendor] [probed] (classes_are_vendor_tokens)
pub const PK_CLASS_icurve: PK_CLASS_t = 3004; // [vendor] [probed] SSI quartic (was 3005)
pub const PK_CLASS_bcurve: PK_CLASS_t = 3005; // [vendor] [probed] PK_BCURVE_create (was 3004)
pub const PK_CLASS_spcurve: PK_CLASS_t = 3006; // [vendor] [probed] PK_CURVE_make_spcurves_2 (was 3007)
pub const PK_CLASS_fcurve: PK_CLASS_t = 3007; // [vendor] (was 3006)
pub const PK_CLASS_cpcurve: PK_CLASS_t = 3008; // [vendor] (was 3009)
pub const PK_CLASS_trcurve: PK_CLASS_t = 3009; // [vendor] class check in PK_TRCURVE_ask @180489f20 (was 3008)
/// Legacy alias kept for source compatibility (docs name is `spcurve`).
pub const PK_CLASS_scurve: PK_CLASS_t = PK_CLASS_spcurve;
/// Legacy alias kept for source compatibility (docs name is `trcurve`).
pub const PK_CLASS_tcurve: PK_CLASS_t = PK_CLASS_trcurve;

// Surfaces (4001..4011)
pub const PK_CLASS_plane: PK_CLASS_t = 4001; // [probed]
pub const PK_CLASS_cyl: PK_CLASS_t = 4002; // [probed]
pub const PK_CLASS_cone: PK_CLASS_t = 4003; // [vendor]
pub const PK_CLASS_sphere: PK_CLASS_t = 4004; // [probed]
pub const PK_CLASS_torus: PK_CLASS_t = 4005; // [probed]
pub const PK_CLASS_bsurf: PK_CLASS_t = 4006; // [vendor]
pub const PK_CLASS_blendsf: PK_CLASS_t = 4007; // [vendor] (was 4011)
pub const PK_CLASS_offset: PK_CLASS_t = 4008; // [vendor] [probed] offset of a spun surface (was 4007)
pub const PK_CLASS_swept: PK_CLASS_t = 4009; // [vendor] [probed] swept_surface_roundtrip
pub const PK_CLASS_spun: PK_CLASS_t = 4010; // [vendor] [probed] spun_surface_roundtrip
pub const PK_CLASS_fsurf: PK_CLASS_t = 4011; // [vendor] (was 4008)
/// Not a documented class token — value unknown. Kept for source compat.
pub const PK_CLASS_ssurf: PK_CLASS_t = -1; // [unknown]

// Topology (5001..5012) — absent from the vendor numeric table; probed here.
pub const PK_CLASS_vertex: PK_CLASS_t = 5001; // [probed]
pub const PK_CLASS_edge: PK_CLASS_t = 5002; // [probed]
pub const PK_CLASS_loop: PK_CLASS_t = 5003; // [probed]
pub const PK_CLASS_face: PK_CLASS_t = 5004; // [probed]
pub const PK_CLASS_shell: PK_CLASS_t = 5005; // [probed]
pub const PK_CLASS_body: PK_CLASS_t = 5006; // [probed]
pub const PK_CLASS_instance: PK_CLASS_t = 5007; // [guess] remaining topol subclass
pub const PK_CLASS_assembly: PK_CLASS_t = 5008; // [guess] other subclass of part (5012)
pub const PK_CLASS_fin: PK_CLASS_t = 5010; // [probed]
pub const PK_CLASS_region: PK_CLASS_t = 5011; // [probed]
pub const PK_CLASS_part: PK_CLASS_t = 5012; // [probed] superclass of body & assembly

// Other entity classes
pub const PK_CLASS_ki_list: PK_CLASS_t = 6002; // [vendor] live in the hierarchy probe
pub const PK_CLASS_attdef: PK_CLASS_t = 6003; // [probed] PK_ENTITY_ask_class on a real attdef
pub const PK_CLASS_group: PK_CLASS_t = 6005; // [vendor] live in the hierarchy probe
pub const PK_CLASS_delta: PK_CLASS_t = -8; // [unknown]
pub const PK_CLASS_blend: PK_CLASS_t = -9; // [unknown]
pub const PK_CLASS_mesh: PK_CLASS_t = 100001; // [vendor]
pub const PK_CLASS_pline: PK_CLASS_t = 100007; // [vendor]
pub const PK_CLASS_mtopol: PK_CLASS_t = 100014; // [vendor]
pub const PK_CLASS_mfacet: PK_CLASS_t = 100015; // [vendor]
pub const PK_CLASS_mfin: PK_CLASS_t = 100016; // [vendor]
pub const PK_CLASS_mvertex: PK_CLASS_t = 100017; // [vendor]
pub const PK_CLASS_lattice: PK_CLASS_t = 100020; // [vendor]
pub const PK_CLASS_lrod: PK_CLASS_t = 100022; // [vendor]
pub const PK_CLASS_lball: PK_CLASS_t = 100023; // [vendor]
pub const PK_CLASS_frame: PK_CLASS_t = 100024; // [vendor]

// =============================================================================
// Unit vector type — same representation as PK_VECTOR_t but semantically normalised
// =============================================================================

pub type PK_VECTOR1_t = [c_double; 3];

// =============================================================================
// Scale factor type
// =============================================================================

// Real ABI: `typedef int PK_scale_factor_t` — a discrete power-of-10 token enum
// (PK_BODY_enlarge / PK_GEOM_enlarge scale by 1x/10x/100x/1000x, NOT an
// arbitrary double). The old `= c_double` passed a float where the kernel reads
// an int token.
pub type PK_scale_factor_t = c_int;
// [re-abi] appended 4 missing member(s) from pk-enums.h
pub const PK_scale_factor_1x_c: PK_scale_factor_t = 26960;
pub const PK_scale_factor_10x_c: PK_scale_factor_t = 26961;
pub const PK_scale_factor_100x_c: PK_scale_factor_t = 26962;
pub const PK_scale_factor_1000x_c: PK_scale_factor_t = 26963;

// =============================================================================
// Topology sense — direction relative to geometry
// =============================================================================

pub type PK_TOPOL_sense_t = c_int;
pub const PK_TOPOL_sense_positive_c: PK_TOPOL_sense_t = 18542;
pub const PK_TOPOL_sense_negative_c: PK_TOPOL_sense_t = 18541;
/// Band 0x486c..=0x486e (vendor table; static band in the binary).
pub const PK_TOPOL_sense_none_c: PK_TOPOL_sense_t = 18540;

// =============================================================================
// Memory block — opaque transmit/receive buffer
// =============================================================================

/// Opaque memory block used by PK_PARTITION_transmit_b / PK_PART_transmit_b.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_MEMORY_block_t {
    pub data: *mut u8,
    pub length: c_int,
    pub next: *mut PK_MEMORY_block_t,
}

// =============================================================================
// Neutral sheet helper types
// =============================================================================

/// Pair of face sets for neutral sheet trimming.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_set_pair_t {
    pub n_faces_1: c_int,
    pub faces_1: *const PK_FACE_t,
    pub n_faces_2: c_int,
    pub faces_2: *const PK_FACE_t,
}

/// Error code from neutral sheet operations.
pub type PK_neutral_error_t = c_int;
// [re-abi] appended 7 missing member(s) from pk-enums.h
pub const PK_neutral_success_c: PK_neutral_error_t = 3800;
pub const PK_neutral_illegal_input_c: PK_neutral_error_t = 3801;
pub const PK_neutral_numerical_failure_c: PK_neutral_error_t = 3802;
pub const PK_neutral_unknown_config_c: PK_neutral_error_t = 3803;
pub const PK_neutral_imprint_failure_c: PK_neutral_error_t = 3804;
pub const PK_neutral_extend_failure_c: PK_neutral_error_t = 3805;
pub const PK_neutral_fill_hole_failure_c: PK_neutral_error_t = 3806;

/// Per-face causes array from neutral sheet trimming.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_neutral_causes_array_t {
    pub n_causes: c_int,
    pub causes: *mut c_int,
}
