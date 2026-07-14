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
pub type PK_MFACET_t = PK_ENTITY_t;
pub type PK_MFIN_t = PK_ENTITY_t;
pub type PK_MVERTEX_t = PK_ENTITY_t;
pub type PK_MTOPOL_t = PK_ENTITY_t;

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

pub type PK_LOGICAL_t = c_int;
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

/// No error.
pub const PK_ERROR_no_errors: PK_ERROR_code_t = 0;
/// General failure.
pub const PK_ERROR_general: PK_ERROR_code_t = 1;
/// Not an entity (bad tag).
pub const PK_ERROR_not_an_entity: PK_ERROR_code_t = 504;

// =============================================================================
// Class identifiers
// =============================================================================

pub type PK_CLASS_t = c_int;

// Class token values determined EMPIRICALLY by probing pskernel.dll
// V37.1.243 (SOLIDWORKS 2025) via PK_ENTITY_ask_class on known entities and
// PK_CLASS_ask_superclass hierarchy enumeration (see docs/notes on the probe).
// Tags: [probed] = directly observed; [family] = inferred from the V35 docs
// token ordering inside a family whose neighbours were probed; [guess] =
// plausible assignment, not yet verified — do not rely on these.

// Root / structural classes
pub const PK_CLASS_class: PK_CLASS_t = 500; // [guess] root of hierarchy
pub const PK_CLASS_primitive: PK_CLASS_t = 501; // [guess]
pub const PK_CLASS_entity: PK_CLASS_t = 1000; // [probed]
pub const PK_CLASS_geom: PK_CLASS_t = 1001; // [probed]
pub const PK_CLASS_topol: PK_CLASS_t = 1002; // [probed]
pub const PK_CLASS_item: PK_CLASS_t = 1003; // [guess] third entity subclass

// Geometry family roots
pub const PK_CLASS_curve: PK_CLASS_t = 2002; // [probed]
pub const PK_CLASS_surf: PK_CLASS_t = 2003; // [probed]
pub const PK_CLASS_transf: PK_CLASS_t = 2004; // [guess] entity subclass
pub const PK_CLASS_point: PK_CLASS_t = 2501; // [probed]

// Curves (3001..3009)
pub const PK_CLASS_line: PK_CLASS_t = 3001; // [probed]
pub const PK_CLASS_circle: PK_CLASS_t = 3002; // [probed]
pub const PK_CLASS_ellipse: PK_CLASS_t = 3003; // [family]
pub const PK_CLASS_bcurve: PK_CLASS_t = 3004; // [family]
pub const PK_CLASS_icurve: PK_CLASS_t = 3005; // [family]
pub const PK_CLASS_fcurve: PK_CLASS_t = 3006; // [family]
pub const PK_CLASS_spcurve: PK_CLASS_t = 3007; // [family]
pub const PK_CLASS_trcurve: PK_CLASS_t = 3008; // [family]
pub const PK_CLASS_cpcurve: PK_CLASS_t = 3009; // [family] superclass is geom, not curve
/// Legacy alias kept for source compatibility (docs name is `spcurve`).
pub const PK_CLASS_scurve: PK_CLASS_t = PK_CLASS_spcurve;
/// Legacy alias kept for source compatibility (docs name is `trcurve`).
pub const PK_CLASS_tcurve: PK_CLASS_t = PK_CLASS_trcurve;

// Surfaces (4001..4011)
pub const PK_CLASS_plane: PK_CLASS_t = 4001; // [probed]
pub const PK_CLASS_cyl: PK_CLASS_t = 4002; // [probed]
pub const PK_CLASS_cone: PK_CLASS_t = 4003; // [family]
pub const PK_CLASS_sphere: PK_CLASS_t = 4004; // [probed]
pub const PK_CLASS_torus: PK_CLASS_t = 4005; // [probed]
pub const PK_CLASS_bsurf: PK_CLASS_t = 4006; // [family]
pub const PK_CLASS_offset: PK_CLASS_t = 4007; // [family]
pub const PK_CLASS_fsurf: PK_CLASS_t = 4008; // [family]
pub const PK_CLASS_swept: PK_CLASS_t = 4009; // [family]
pub const PK_CLASS_spun: PK_CLASS_t = 4010; // [family]
pub const PK_CLASS_blendsf: PK_CLASS_t = 4011; // [family]
/// Not a documented V35 class token — value unknown. Kept for source compat.
pub const PK_CLASS_ssurf: PK_CLASS_t = -1; // [unknown]

// Topology (5001..5012)
pub const PK_CLASS_vertex: PK_CLASS_t = 5001; // [probed]
pub const PK_CLASS_edge: PK_CLASS_t = 5002; // [probed]
pub const PK_CLASS_loop: PK_CLASS_t = 5003; // [probed]
pub const PK_CLASS_face: PK_CLASS_t = 5004; // [probed]
pub const PK_CLASS_shell: PK_CLASS_t = 5005; // [probed]
pub const PK_CLASS_body: PK_CLASS_t = 5006; // [probed]
pub const PK_CLASS_instance: PK_CLASS_t = 5007; // [guess] remaining topol subclass
pub const PK_CLASS_assembly: PK_CLASS_t = 5008; // [family] other subclass of part (5012)
pub const PK_CLASS_fin: PK_CLASS_t = 5010; // [probed]
pub const PK_CLASS_region: PK_CLASS_t = 5011; // [probed]
pub const PK_CLASS_part: PK_CLASS_t = 5012; // [probed] superclass of body & assembly

// Unverified classes — hierarchy probe shows valid classes at 6002/6003/6005
// (entity subclasses); names below are NOT yet matched to values. Do not use
// for dispatch until verified.
pub const PK_CLASS_attrib: PK_CLASS_t = -2; // [unknown]
pub const PK_CLASS_attdef: PK_CLASS_t = -3; // [unknown]
pub const PK_CLASS_group: PK_CLASS_t = -4; // [unknown]
pub const PK_CLASS_partition: PK_CLASS_t = -5; // [unknown]
pub const PK_CLASS_mark: PK_CLASS_t = -6; // [unknown]
pub const PK_CLASS_pmark: PK_CLASS_t = -7; // [unknown]
pub const PK_CLASS_delta: PK_CLASS_t = -8; // [unknown]
pub const PK_CLASS_blend: PK_CLASS_t = -9; // [unknown]
pub const PK_CLASS_mesh: PK_CLASS_t = -10; // [unknown]
pub const PK_CLASS_mfacet: PK_CLASS_t = -11; // [unknown]
pub const PK_CLASS_mfin: PK_CLASS_t = -12; // [unknown]
pub const PK_CLASS_mvertex: PK_CLASS_t = -13; // [unknown]
pub const PK_CLASS_mtopol: PK_CLASS_t = -14; // [unknown]
pub const PK_CLASS_pline: PK_CLASS_t = -15; // [unknown]
pub const PK_CLASS_lattice: PK_CLASS_t = -16; // [unknown]
pub const PK_CLASS_lball: PK_CLASS_t = -17; // [unknown]
pub const PK_CLASS_lrod: PK_CLASS_t = -18; // [unknown]
pub const PK_CLASS_frame: PK_CLASS_t = -19; // [unknown]

// =============================================================================
// Unit vector type — same representation as PK_VECTOR_t but semantically normalised
// =============================================================================

pub type PK_VECTOR1_t = [c_double; 3];

// =============================================================================
// Scale factor type
// =============================================================================

pub type PK_scale_factor_t = c_double;

// =============================================================================
// Topology sense — direction relative to geometry
// =============================================================================

pub type PK_TOPOL_sense_t = c_int;
pub const PK_TOPOL_sense_positive_c: PK_TOPOL_sense_t = 0;
pub const PK_TOPOL_sense_negative_c: PK_TOPOL_sense_t = 1;

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

/// Per-face causes array from neutral sheet trimming.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_neutral_causes_array_t {
    pub n_causes: c_int,
    pub causes: *mut c_int,
}

