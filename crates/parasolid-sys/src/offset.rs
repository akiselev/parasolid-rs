//! Offsetting, hollowing, and thickening bindings.
//!
//! Bindings for `PK_BODY_offset_2`, `PK_FACE_offset_2`, `PK_EDGE_offset_on_body`,
//! `PK_BODY_hollow_2`, `PK_FACE_hollow_3`, and `PK_BODY_thicken_3` (Chapters 56-58).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_double, c_int, c_void};

use crate::*;

// =============================================================================
// Shared enum types for offset/hollow/thicken
// =============================================================================

// -- offset_method: self-intersection removal method --------------------------

pub type PK_offset_method_t = c_int;
/// Trim and extend (legacy).
pub const PK_offset_method_sx_trim_c: PK_offset_method_t = 0;
/// Trim, repair, and extend.
pub const PK_offset_method_sx_repair_1_c: PK_offset_method_t = 1;
/// Extended B-surface patching (recommended).
pub const PK_offset_method_sx_repair_2_c: PK_offset_method_t = 2;

// -- offset_step: side face creation ------------------------------------------

pub type PK_offset_step_t = c_int;
/// No side faces (default).
pub const PK_offset_step_no_c: PK_offset_step_t = 0;
/// Side faces at all smooth boundaries.
pub const PK_offset_step_all_c: PK_offset_step_t = 1;
/// Side faces only where growing fails (not for facet geometry).
pub const PK_offset_step_site_c: PK_offset_step_t = 2;
/// Side faces only at offset/non-offset boundaries (legacy v18.0).
pub const PK_offset_step_pierce_c: PK_offset_step_t = 3;

// -- fix_degens: degeneracy repair --------------------------------------------

pub type PK_fix_degens_t = c_int;
/// No repair (default).
pub const PK_fix_degens_no_c: PK_fix_degens_t = 0;
/// Repair only offset faces.
pub const PK_fix_degens_offset_c: PK_fix_degens_t = 1;
/// Repair offset faces and their non-offset pairs.
pub const PK_fix_degens_all_c: PK_fix_degens_t = 2;

// -- blend_edges: blend creation at offset edges ------------------------------

pub type PK_EDGE_offset_blend_t = c_int;
/// Replace convex offset edges with blends.
pub const PK_EDGE_offset_blend_convex_c: PK_EDGE_offset_blend_t = 1;

// -- vertex_limit: vertex tolerance constraint --------------------------------

pub type PK_VERTEX_limit_t = c_int;
/// No limit (default).
pub const PK_VERTEX_limit_no_c: PK_VERTEX_limit_t = 0;
/// Allow but report large tolerances.
pub const PK_VERTEX_limit_report_c: PK_VERTEX_limit_t = 1;
/// Fail if limits not satisfied.
pub const PK_VERTEX_limit_yes_c: PK_VERTEX_limit_t = 2;

// -- edge_limit: edge tolerance constraint ------------------------------------

pub type PK_EDGE_limit_t = c_int;
/// No limit (default).
pub const PK_EDGE_limit_no_c: PK_EDGE_limit_t = 0;
/// Allow but report large tolerances.
pub const PK_EDGE_limit_report_c: PK_EDGE_limit_t = 1;
/// Fail if limits not satisfied.
pub const PK_EDGE_limit_yes_c: PK_EDGE_limit_t = 2;

// -- track_del: track partially tight-curved face deletions -------------------

pub type PK_offset_track_del_t = c_int;
/// Track partially tight-curved face deletions.
pub const PK_offset_track_del_sx_repair_c: PK_offset_track_del_t = 1;

// -- ortho_vx_split: laminar vertex splitting ---------------------------------

pub type PK_ortho_vx_split_t = c_int;

// -- local_ops_update: update switch ------------------------------------------

// -- grow: heal overflowing features ------------------------------------------

pub type PK_FACE_grow_t = c_int;
/// Do not grow any faces.
pub const PK_FACE_grow_no_c: PK_FACE_grow_t = 0;
/// Grow the faces that are moving.
pub const PK_FACE_grow_moving_c: PK_FACE_grow_t = 1;
/// Grow the faces that are not moving.
pub const PK_FACE_grow_fixed_c: PK_FACE_grow_t = 2;

// =============================================================================
// Hollowing-specific enum types
// =============================================================================

pub type PK_hollow_local_t = c_int;
/// Hollow entire body (default).
pub const PK_hollow_local_none_c: PK_hollow_local_t = 0;
/// Hollow only specified faces (add to existing hollow).
pub const PK_hollow_local_add_c: PK_hollow_local_t = 1;
/// Hollow everything except specified faces.
pub const PK_hollow_local_exclude_c: PK_hollow_local_t = 2;

// =============================================================================
// Thickening-specific enum types
// =============================================================================

pub type PK_thicken_method_t = c_int;
/// Create side faces in specified punch direction.
pub const PK_thicken_method_punch_c: PK_thicken_method_t = 1;

// =============================================================================
// Edge offset gap fill enum types
// =============================================================================

pub type PK_VERTEX_gap_fill_t = c_int;
/// Fill with round arc.
pub const PK_VERTEX_gap_fill_round_c: PK_VERTEX_gap_fill_t = 0;
/// Fill with tangent extension.
pub const PK_VERTEX_gap_fill_linear_c: PK_VERTEX_gap_fill_t = 1;

// =============================================================================
// Report constants
// =============================================================================

/// Degeneracies fixed report status.
pub const PK_REPORT_1_fa_fix_degens_c: c_int = 1;
/// Large vertex tolerance report status.
pub const PK_REPORT_1_vx_large_tol_c: c_int = 2;
/// Large edge tolerance report status.
pub const PK_REPORT_1_ed_large_tol_c: c_int = 3;
/// Track record: face deleted.
pub const PK_TOPOLOGY_track_delete_c: c_int = 1;

// =============================================================================
// Status constants
// =============================================================================

/// Non-manifold edge/vertex blocking offset on body.
pub const PK_offset_on_body_general_c: PK_local_status_t = 1;
/// Edge tolerance limit failure.
pub const PK_local_status_ed_large_tol_c: PK_local_status_t = 2;

// =============================================================================
// Options structures
// =============================================================================

/// Options for `PK_BODY_offset_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_offset_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Number of faces to offset by non-default distance.
    pub n_offset_faces: c_int,
    /// Array of faces to offset by non-default distance.
    pub offset_faces: *const PK_FACE_t,
    /// Offset values for `offset_faces`.
    pub offset_values: *const c_double,
    /// Allow disjoint bodies in result.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create step/side faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before offsetting.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Radius for blend faces.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Constrain vertex tolerance.
    pub vertex_limit: PK_VERTEX_limit_t,
    /// Constrain edge tolerance.
    pub edge_limit: PK_EDGE_limit_t,
    /// Track deletion of partially tight-curved faces.
    pub track_del: PK_offset_track_del_t,
    /// How to split laminar vertices.
    pub ortho_vx_split: PK_ortho_vx_split_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_BODY_offset_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_offset_faces: 0,
            offset_faces: std::ptr::null(),
            offset_values: std::ptr::null(),
            allow_disjoint: PK_LOGICAL_true,
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            vertex_limit: PK_VERTEX_limit_no_c,
            edge_limit: PK_EDGE_limit_no_c,
            track_del: 0,
            ortho_vx_split: 0,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_FACE_offset_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_offset_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Allow disjoint bodies in result.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create step/side faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before offsetting.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Radius for blend faces.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Constrain vertex tolerance.
    pub vertex_limit: PK_VERTEX_limit_t,
    /// Constrain edge tolerance.
    pub edge_limit: PK_EDGE_limit_t,
    /// Track deletion of partially tight-curved faces.
    pub track_del: PK_offset_track_del_t,
    /// How to split laminar vertices.
    pub ortho_vx_split: PK_ortho_vx_split_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_FACE_offset_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            allow_disjoint: PK_LOGICAL_true,
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            vertex_limit: PK_VERTEX_limit_no_c,
            edge_limit: PK_EDGE_limit_no_c,
            track_del: 0,
            ortho_vx_split: 0,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_BODY_hollow_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_hollow_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Number of faces to pierce (remove to open void).
    pub n_pierce_faces: c_int,
    /// Array of faces to pierce.
    pub pierce_faces: *const PK_FACE_t,
    /// Number of faces to offset by non-default distance.
    pub n_offset_faces: c_int,
    /// Array of faces to offset by non-default distance.
    pub offset_faces: *const PK_FACE_t,
    /// Offset distances for `offset_faces` (zero = pierce).
    pub offset_values: *const c_double,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create side faces between different-offset faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before hollowing.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Local hollowing mode.
    pub hollow_local: PK_hollow_local_t,
    /// Number of faces for local hollowing.
    pub n_local_faces: c_int,
    /// Faces to hollow or exclude (for local hollowing).
    pub local_faces: *const PK_FACE_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Blend radius.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Number of grow data entries.
    pub n_grow_data: c_int,
    /// Override grow behavior data.
    pub grow_data: *const c_void,
    /// Override grow behavior callback.
    pub grow_cb: *const c_void,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_BODY_hollow_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_pierce_faces: 0,
            pierce_faces: std::ptr::null(),
            n_offset_faces: 0,
            offset_faces: std::ptr::null(),
            offset_values: std::ptr::null(),
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            hollow_local: PK_hollow_local_none_c,
            n_local_faces: 0,
            local_faces: std::ptr::null(),
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            n_grow_data: 0,
            grow_data: std::ptr::null(),
            grow_cb: std::ptr::null(),
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_FACE_hollow_3`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_hollow_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Create side faces.
    pub offset_step: PK_offset_step_t,
    /// Fix degeneracies before hollowing.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Local hollowing mode.
    pub hollow_local: PK_hollow_local_t,
    /// Number of faces for local hollowing.
    pub n_local_faces: c_int,
    /// Faces to hollow or exclude (for local hollowing).
    pub local_faces: *const PK_FACE_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Blend radius.
    pub blend_radius: c_double,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_FACE_hollow_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            check_fa_fa: PK_LOGICAL_false,
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            offset_step: PK_offset_step_no_c,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            hollow_local: PK_hollow_local_none_c,
            n_local_faces: 0,
            local_faces: std::ptr::null(),
            blend_edges: 0,
            blend_radius: 0.0,
            grow: PK_FACE_grow_no_c,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_BODY_thicken_3`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_thicken_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Number of faces to thicken by non-default amounts.
    pub n_faces: c_int,
    /// Faces to thicken by non-default amounts.
    pub faces: *const PK_FACE_t,
    /// Front offset per face (array of size `n_faces`).
    pub front_offsets: *const c_double,
    /// Back offset per face (array of size `n_faces`).
    pub back_offsets: *const c_double,
    /// Check face-face inconsistencies.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Side face creation method.
    pub method: PK_thicken_method_t,
    /// Punch direction vector (when `method` = `PK_thicken_method_punch_c`).
    pub punch_dir: PK_VECTOR_t,
    /// Number of laminar edges for user-supplied side surfaces.
    pub n_edges: c_int,
    /// Laminar edges for user-supplied side surfaces.
    pub edges: *const PK_EDGE_t,
    /// Number of user-supplied side surfaces.
    pub n_surfaces: c_int,
    /// User-supplied side surfaces.
    pub surfaces: *const PK_SURF_t,
    /// Self-intersection removal method.
    pub offset_method: PK_offset_method_t,
    /// Report self-intersections.
    pub report_sx: PK_LOGICAL_t,
    /// Fix degeneracies before thickening.
    pub fix_degens: PK_fix_degens_t,
    /// Report degeneracy fixes.
    pub report_fix_degens: PK_LOGICAL_t,
    /// Create blend faces from offset edges.
    pub blend_edges: PK_EDGE_offset_blend_t,
    /// Blend radius.
    pub blend_radius: c_double,
    /// Split laminar vertices orthogonally.
    pub ortho_vx_split: PK_ortho_vx_split_t,
    /// Number of faces to pierce (remove from result).
    pub n_pierce_faces: c_int,
    /// Faces to pierce.
    pub pierce_faces: *const PK_FACE_t,
    /// Create side faces along smooth edges.
    pub offset_step: PK_offset_step_t,
    /// Heal overflowing features.
    pub grow: PK_FACE_grow_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_BODY_thicken_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            n_faces: 0,
            faces: std::ptr::null(),
            front_offsets: std::ptr::null(),
            back_offsets: std::ptr::null(),
            check_fa_fa: PK_LOGICAL_false,
            method: 0,
            punch_dir: [0.0, 0.0, 0.0],
            n_edges: 0,
            edges: std::ptr::null(),
            n_surfaces: 0,
            surfaces: std::ptr::null(),
            offset_method: PK_offset_method_sx_repair_2_c,
            report_sx: PK_LOGICAL_false,
            fix_degens: PK_fix_degens_no_c,
            report_fix_degens: PK_LOGICAL_false,
            blend_edges: 0,
            blend_radius: 0.0,
            ortho_vx_split: 0,
            n_pierce_faces: 0,
            pierce_faces: std::ptr::null(),
            offset_step: PK_offset_step_no_c,
            grow: PK_FACE_grow_no_c,
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Options for `PK_EDGE_offset_on_body`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_offset_on_body_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Fill gaps: round arc or tangent extension.
    pub gap_fill: PK_VERTEX_gap_fill_t,
    /// Fill gaps (constrained by angle).
    pub constrained_gap_fill: PK_LOGICAL_t,
    /// Angular interval constraint (radians).
    pub gap_fill_angle: PK_INTERVAL_t,
    /// Update switch.
    pub update: PK_local_ops_update_t,
}

impl Default for PK_EDGE_offset_on_body_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            gap_fill: PK_VERTEX_gap_fill_round_c,
            constrained_gap_fill: PK_LOGICAL_false,
            gap_fill_angle: PK_INTERVAL_t {
                low: 0.0,
                high: 0.0,
            },
            update: PK_local_ops_update_default_c,
        }
    }
}

/// Results from `PK_BODY_thicken_2`.
#[repr(C)]
pub struct PK_BODY_thicken_r_t { _private: [u8; 0] }

// =============================================================================
// Extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // -------------------------------------------------------------------------
    // Offsetting (Chapter 56)
    // -------------------------------------------------------------------------

    /// Offset all faces in a body by a specified distance.
    ///
    /// Positive offset = outward (direction of face normal), negative = inward.
    /// Can optionally offset a subset of faces by different distances via options.
    ///
    /// # Arguments
    ///
    /// * `body`      - Body to offset.
    /// * `distance`  - Default offset distance for all faces.
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure.
    pub fn PK_BODY_offset_2(
        body: PK_BODY_t,
        distance: c_double,
        tolerance: c_double,
        options: *const PK_BODY_offset_o_t,
    ) -> PK_ERROR_code_t;

    /// Offset a specific set of faces in a body.
    ///
    /// Receives set of faces + set of offsets, offsets each face by corresponding
    /// amount. Same underlying functionality as `PK_BODY_offset_2`.
    ///
    /// # Arguments
    ///
    /// * `n_faces`   - Number of faces to offset.
    /// * `faces`     - Array of face tags.
    /// * `offsets`   - Array of offset distances (one per face).
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure.
    pub fn PK_FACE_offset_2(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        offsets: *const c_double,
        tolerance: c_double,
        options: *const PK_FACE_offset_o_t,
    ) -> PK_ERROR_code_t;

    /// Offset connected edges in a given direction within their owning body.
    ///
    /// Edges can be on manifold or general body (must be in locally manifold part).
    /// Offset edges are imprinted on body. Each point offset within its normal
    /// plane; distance measured by arc-length.
    ///
    /// # Arguments
    ///
    /// * `n_edges`    - Number of edges to offset.
    /// * `edges`      - Array of connected edge tags.
    /// * `distance`   - Offset distance (arc-length).
    /// * `direction`  - Offset direction vector.
    /// * `options`    - Options structure.
    /// * `n_new_edges` - (out) Number of new edges created.
    /// * `new_edges`   - (out) Array of new edge tags (PK-allocated, caller must free).
    pub fn PK_EDGE_offset_on_body(
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        distance: c_double,
        direction: *const PK_VECTOR_t,
        options: *const PK_EDGE_offset_on_body_o_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Hollowing (Chapter 57)
    // -------------------------------------------------------------------------

    /// Hollow a solid body by offsetting all faces by a specified distance.
    ///
    /// Positive offset = outward (slightly larger hollowed body).
    /// Negative offset = inward (same outer size as original).
    ///
    /// # Arguments
    ///
    /// * `body`      - Solid body to hollow.
    /// * `distance`  - Default offset distance for all faces.
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure (includes pierce faces, per-face offsets).
    pub fn PK_BODY_hollow_2(
        body: PK_BODY_t,
        distance: c_double,
        tolerance: c_double,
        options: *const PK_BODY_hollow_o_t,
    ) -> PK_ERROR_code_t;

    /// Hollow by specifying exactly which faces to offset and by how much.
    ///
    /// Any faces NOT specified are treated as pierce faces.
    ///
    /// # Arguments
    ///
    /// * `n_faces`   - Number of faces to offset.
    /// * `faces`     - Array of face tags.
    /// * `offsets`   - Array of offset distances (one per face).
    /// * `tolerance` - Tolerance for approximate geometry.
    /// * `options`   - Options structure.
    pub fn PK_FACE_hollow_3(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        offsets: *const c_double,
        tolerance: c_double,
        options: *const PK_FACE_hollow_o_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Thickening (Chapter 58)
    // -------------------------------------------------------------------------

    /// Thicken a sheet body to form a solid body.
    ///
    /// Sheet thickened by offsetting in both directions by specified amounts.
    /// Only manifold sheets can be thickened. Side faces created by surface
    /// normals, punch direction, or user-supplied surfaces.
    ///
    /// # Arguments
    ///
    /// * `body`          - Sheet body to thicken.
    /// * `front_default` - Default front offset distance.
    /// * `back_default`  - Default back offset distance.
    /// * `tolerance`     - Tolerance for approximate geometry.
    /// * `options`       - Options structure (includes per-face offsets, side face method).
    pub fn PK_BODY_thicken_3(
        body: PK_BODY_t,
        front_default: c_double,
        back_default: c_double,
        tolerance: c_double,
        options: *const PK_BODY_thicken_o_t,
    ) -> PK_ERROR_code_t;

    // -------------------------------------------------------------------------
    // Obsolete V1 offset/hollow/thicken functions
    // -------------------------------------------------------------------------

    /// Offset faces of solid/sheet body (obsolete V1).
    pub fn PK_BODY_offset(
        body: PK_BODY_t,
        offset: c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Hollow solid body by offsetting all faces (obsolete V1).
    pub fn PK_BODY_hollow(
        body: PK_BODY_t,
        offset: c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        n_faces: *mut c_int,
        old_faces: *mut *mut PK_FACE_t,
        new_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Thicken sheet body into solid (obsolete V1).
    pub fn PK_BODY_thicken(
        body: PK_BODY_t,
        front: c_double,
        back: c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        n_topols: *mut c_int,
        old_topols: *mut *mut PK_TOPOL_t,
        new_topols: *mut *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Thicken sheet body into solid (current V2).
    pub fn PK_BODY_thicken_2(
        body: PK_BODY_t,
        front_default: c_double,
        back_default: c_double,
        tolerance: c_double,
        options: *const PK_BODY_thicken_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_BODY_thicken_r_t,
    ) -> PK_ERROR_code_t;

    /// Hollow body by offsetting specific faces (obsolete V1).
    pub fn PK_FACE_hollow(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        offsets: *mut c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        new_faces: *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    /// Hollow body by offsetting specific faces (current V2).
    pub fn PK_FACE_hollow_2(
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        offsets: *mut c_double,
        tolerance: c_double,
        face_face_check: PK_LOGICAL_t,
        n_new_faces: *mut c_int,
        old_faces: *mut *mut PK_FACE_t,
        new_faces: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;
}
