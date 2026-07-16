//! Wire modeling, sheet modeling, sewing/knitting, mid-surface, and sheet/surface extension.
//!
//! Bindings for Parasolid chapters 42--46: wire body creation, sheet body operations,
//! sewing and knitting, neutral-sheet generation, and body/surface extension.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_double, c_int};

use crate::*;

// =============================================================================
// Sheet modeling update types
// =============================================================================

pub type PK_FACE_cover_update_t = c_int;
pub const PK_FACE_cover_update_default_c: PK_FACE_cover_update_t = 25570;

// =============================================================================
// Wire modeling — enums and constants (Chapter 42)
// =============================================================================

/// Natural curve extensions.
pub const PK_VERTEX_gap_fill_natural_c: PK_VERTEX_gap_fill_t = 21222;

/// Whether curves are supplied in connection sequence.
pub type PK_CURVE_sequential_t = c_int;
/// Curves not in connection sequence (default).
pub const PK_CURVE_sequential_no_c: PK_CURVE_sequential_t = 23460;
/// Curves in connection sequence.
pub const PK_CURVE_sequential_yes_c: PK_CURVE_sequential_t = 23461;

// =============================================================================
// Sheet modeling — enums and constants (Chapter 43)
// =============================================================================

/// Whether to make sheet from copies or originals.
pub type PK_BODY_make_from_t = c_int;
/// Make sheet from copies of input faces (default).
pub const PK_BODY_make_from_copy_c: PK_BODY_make_from_t = 26510;
/// Make sheet from original faces; originals removed from source body.
pub const PK_BODY_make_from_original_c: PK_BODY_make_from_t = 26511;

/// Whether to track edges in tracking output.
pub type PK_track_edges_t = c_int;
/// Do not return edges in tracking (default).
pub const PK_track_edges_no_c: PK_track_edges_t = 23981;
/// Return edges in tracking.
pub const PK_track_edges_yes_c: PK_track_edges_t = 23980;

/// Whether to track vertices in tracking output.
pub type PK_track_vertices_t = c_int;
/// Do not return vertices in tracking (default).
pub const PK_track_vertices_no_c: PK_track_vertices_t = 23991;
/// Return vertices in tracking.
pub const PK_track_vertices_yes_c: PK_track_vertices_t = 23990;

/// Group transfer mode when creating sheet bodies.
pub type PK_GROUP_transfer_t = c_int;
/// Transfer owning groups (default).
pub const PK_GROUP_transfer_owning_c: PK_GROUP_transfer_t = 26551;

// =============================================================================
// Sewing and knitting — enums and constants (Chapter 44)
// =============================================================================

/// Preferred body type for sewing result.
pub type PK_BODY_sewing_t = c_int;
/// Prefer solid if no laminar/non-manifold edges.
pub const PK_BODY_sewing_solid_c: PK_BODY_sewing_t = 18070;
/// Prefer sheet.
pub const PK_BODY_sewing_sheet_c: PK_BODY_sewing_t = 18071;
/// All results as general bodies.
pub const PK_BODY_sewing_general_c: PK_BODY_sewing_t = 18072;
/// No preference; pick most specific type (default).
pub const PK_BODY_sewing_any_c: PK_BODY_sewing_t = 18073;

/// Duplicate sheet removal strategy during sewing.
pub type PK_BODY_sewing_remove_t = c_int;
/// No duplicate removal (default).
pub const PK_BODY_sewing_remove_none_c: PK_BODY_sewing_remove_t = 18076;
/// Remove possible duplicates (Parasolid precision).
pub const PK_BODY_sewing_remove_poss_c: PK_BODY_sewing_remove_t = 18077;
/// Remove certain duplicates (gap_width_bound tolerance).
pub const PK_BODY_sewing_remove_cert_c: PK_BODY_sewing_remove_t = 18078;

/// Whether to sew edges in the same inner loop.
pub type PK_LOOP_sew_up_t = c_int;
/// Sew edges in same inner loop (default).
pub const PK_LOOP_sew_up_loop_c: PK_LOOP_sew_up_t = 25500;
/// Do not sew edges in same inner loop.
pub const PK_LOOP_sew_up_no_c: PK_LOOP_sew_up_t = 25501;

/// Whether to attempt edge tolerance reduction after sewing.
pub type PK_EDGE_reduce_tol_t = c_int;
/// No tolerance reduction (default).
pub const PK_EDGE_reduce_tol_no_c: PK_EDGE_reduce_tol_t = 26450;
/// Attempt to reduce edge tolerance.
pub const PK_EDGE_reduce_tol_yes_c: PK_EDGE_reduce_tol_t = 26451;

/// Assembly sewing method.
pub type PK_BODY_sewing_assy_t = c_int;
/// Use face orientation to determine piece parts.
pub const PK_BODY_sewing_assy_orient_c: PK_BODY_sewing_assy_t = 24191;
/// Also examine shared laminar edges.
pub const PK_BODY_sewing_assy_extend_c: PK_BODY_sewing_assy_t = 24192;

/// General topology sewing mode.
pub type PK_BODY_sewing_gen_t = c_int;
/// Fail on general topology input (default).
pub const PK_BODY_sewing_gen_no_c: PK_BODY_sewing_gen_t = 24240;
/// Sew general bodies at locally manifold boundaries.
pub const PK_BODY_sewing_gen_loc_manf_c: PK_BODY_sewing_gen_t = 24241;

/// Problem group tokens from sewing operations.
pub type PK_BODY_sewing_problem_t = c_int;
/// 3+ sheets meet, no distinct pair found.
pub const PK_BODY_sewing_non_manifold_c: PK_BODY_sewing_problem_t = 18061;
/// Sheet meets itself after half-twist.
pub const PK_BODY_sewing_non_oriented_c: PK_BODY_sewing_problem_t = 18062;
/// Internal algorithmic failure.
pub const PK_BODY_sewing_unspecified_c: PK_BODY_sewing_problem_t = 18063;
/// Overlapping sheets (not currently returned).
pub const PK_BODY_sewing_overlapping_c: PK_BODY_sewing_problem_t = 18060;

/// Preferred body type for knitting.
pub type PK_BODY_type_prefer_t = c_int;
/// Prefer solid body.
pub const PK_BODY_type_prefer_solid_c: PK_BODY_type_prefer_t = 26530;
/// Prefer sheet body.
pub const PK_BODY_type_prefer_sheet_c: PK_BODY_type_prefer_t = 26531;
/// Prefer general body.
pub const PK_BODY_type_prefer_general_c: PK_BODY_type_prefer_t = 26532;
/// Prefer original body type.
pub const PK_BODY_type_prefer_original_c: PK_BODY_type_prefer_t = 26533;

// =============================================================================
// Mid-surface — enums and constants (Chapter 45)
// =============================================================================

/// Face overlap detection for neutral sheet creation.
pub type PK_neutral_face_overlap_t = c_int;
/// Face overlaps not considered (default).
pub const PK_neutral_face_overlap_no_c: PK_neutral_face_overlap_t = 25390;
/// Face overlaps considered; reject non-overlapping pairs.
pub const PK_neutral_face_overlap_yes_c: PK_neutral_face_overlap_t = 25391;

/// Neutral sheet construction method.
pub type PK_neutral_method_t = c_int;
/// Verify faces are offsets; create sheet between them (default).
pub const PK_neutral_method_mid_offset_c: PK_neutral_method_t = 25020;
/// Offset from left faces toward right faces.
pub const PK_neutral_method_offset_left_c: PK_neutral_method_t = 25021;
/// Average mid-surface (no offset requirement).
pub const PK_neutral_method_medial_c: PK_neutral_method_t = 25022;

/// Whether to ignore small faces during neutral sheet trimming.
pub type PK_neutral_ignore_fa_t = c_int;
/// Ignore small faces; maintain original connectivity (default).
pub const PK_neutral_ignore_fa_default_c: PK_neutral_ignore_fa_t = 0;
/// Trim exactly to face set pair limits.
pub const PK_neutral_ignore_fa_no_c: PK_neutral_ignore_fa_t = 1;

/// Overlap handling during neutral sheet trimming.
pub type PK_neutral_overlap_t = c_int;
/// Ignore overlapping faces (default).
pub const PK_neutral_overlap_no_c: PK_neutral_overlap_t = 24540;
/// Detect and report overlapping faces.
pub const PK_neutral_overlap_report_c: PK_neutral_overlap_t = 24541;
/// Repair overlapping faces (delete one set).
pub const PK_neutral_overlap_repair_c: PK_neutral_overlap_t = 24542;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_REPORT_1_fa_overlap_c: PK_neutral_overlap_t = 23900;

/// Neutral sheet trim method.
pub type PK_neutral_trim_method_t = c_int;
/// Trim only against other neutral sheets (default).
pub const PK_neutral_trim_method_sheets_c: PK_neutral_trim_method_t = 24500;
/// Also trim against side faces of original body.
pub const PK_neutral_trim_method_sides_c: PK_neutral_trim_method_t = 24501;

// =============================================================================
// Extending sheets and surfaces — enums and constants (Chapter 46)
// =============================================================================

/// C2-continuous, mirrors existing geometry.
pub const PK_extension_shape_reflective_c: PK_extension_shape_t = 22752;
/// Constant radii of curvature, circular arc cross-section, G2 match (PK_SURF_extend only).
pub const PK_extension_shape_arc_c: PK_extension_shape_t = 22754;
/// Continues surface shape, C-infinity at boundary (PK_SURF_extend only).
pub const PK_extension_shape_natural_c: PK_extension_shape_t = 22753;

/// Target limit for extend-to-target operations.
pub type PK_extension_limit_t = c_int;
/// Minimum distance to intersect target (default).
pub const PK_extension_limit_minimal_c: PK_extension_limit_t = 23800;
/// Extend until reaching inside of target.
pub const PK_extension_limit_inside_c: PK_extension_limit_t = 23801;
/// Extend until reaching outside of target.
pub const PK_extension_limit_outside_c: PK_extension_limit_t = 23802;

/// Smoothness preservation across internal edges.
pub type PK_extension_smoothness_t = c_int;
/// Preserve G1 smoothness across internal edges.
pub const PK_extension_smoothness_g1_c: PK_extension_smoothness_t = 22911;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_extension_smoothness_g0_c: PK_extension_smoothness_t = 22910;

/// How new topology is created during extension.
pub type PK_extend_create_t = c_int;
/// New topology without original attributes (default).
pub const PK_extend_create_new_c: PK_extend_create_t = 24150;
/// Split from original topology (attributes follow split rules).
pub const PK_extend_create_split_c: PK_extend_create_t = 24151;

/// Extension boundary precision.
pub type PK_extension_boundary_t = c_int;
/// Precise boundary (default).
pub const PK_extension_boundary_precise_c: PK_extension_boundary_t = 24270;
/// Rough boundary (better performance, no detailed tracking).
pub const PK_extension_boundary_loose_c: PK_extension_boundary_t = 24271;

/// Side edge construction method for body extension.
pub type PK_extend_side_t = c_int;
/// Parasolid decides (default).
pub const PK_extend_side_default_c: PK_extend_side_t = 24730;
/// Follow adjacent edges/extensions.
pub const PK_extend_side_follow_adj_c: PK_extend_side_t = 24731;
/// Orthogonal to base edge.
pub const PK_extend_side_ortho_base_c: PK_extend_side_t = 24732;

/// Tracking detail for new laminar side edges during extension.
pub type PK_extend_track_laminar_t = c_int;
/// Track to originating vertex only (default).
pub const PK_extend_track_laminar_basic_c: PK_extend_track_laminar_t = 0;
/// Track to vertex and incident edges.
pub const PK_extend_track_laminar_edges_c: PK_extend_track_laminar_t = 1;

/// Whether to track non-laminar side edges during extension.
pub type PK_extend_track_internal_t = c_int;
/// Only track laminar side edges (default).
pub const PK_extend_track_internal_no_c: PK_extend_track_internal_t = 23580;
/// Also track non-laminar side edges.
pub const PK_extend_track_internal_yes_c: PK_extend_track_internal_t = 23581;

/// Surface extension type.
pub type PK_SURF_extension_t = c_int;
/// No extension (default).
pub const PK_SURF_extension_none_c: PK_SURF_extension_t = 22030;
/// Extend to a point.
pub const PK_SURF_extension_point_c: PK_SURF_extension_t = 22031;
/// Extend to a bounding box.
pub const PK_SURF_extension_box_c: PK_SURF_extension_t = 22032;
/// Extend to a parameter-space box.
pub const PK_SURF_extension_uvbox_c: PK_SURF_extension_t = 22033;
/// Extend by parameter boundary ratios.
pub const PK_SURF_extension_ratio_c: PK_SURF_extension_t = 22034;

/// Status return from PK_SURF_extend.
pub type PK_SURF_extend_status_t = c_int;
/// Extension succeeded.
pub const PK_SURF_extend_ok_c: PK_SURF_extend_status_t = 22010;
/// No extension necessary (e.g. extending a plane).
pub const PK_SURF_extend_unextended_c: PK_SURF_extend_status_t = 22011;
/// Partial extension performed.
pub const PK_SURF_extend_partial_c: PK_SURF_extend_status_t = 22012;
/// Extension would create invalid surface.
pub const PK_SURF_extend_invalid_c: PK_SURF_extend_status_t = 22013;
/// Internal algorithm failure.
pub const PK_SURF_extend_failure_c: PK_SURF_extend_status_t = 22014;

/// Use all enhancements (default).
pub const PK_surf_extend_update_default_c: PK_surf_extend_update_t = 0;

// =============================================================================
// Tracking structures
// =============================================================================

/// Standard topology tracking result.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_track_r_t {
    /// Number of original entities tracked.
    pub n_original_topols: c_int,
    /// Array of original entity tags.
    pub original_topols: *const PK_TOPOL_t,
    /// Number of product entities.
    pub n_product_topols: c_int,
    /// Array of product entity tags.
    pub product_topols: *const PK_TOPOL_t,
}

// =============================================================================
// Wire modeling — options structures (Chapter 42)
// =============================================================================

/// Options for `PK_CURVE_make_wire_body_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_CURVE_make_wire_body_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Max separation for curve connection (default: 1.0e-6).
    pub tolerance: c_double,
    /// Whether disjoint bodies can be created (default: true).
    pub allow_disjoint: PK_LOGICAL_t,
    /// Whether general wire bodies can be created (default: false).
    pub allow_general: PK_LOGICAL_t,
    /// Whether to check created body (default: true).
    pub check: PK_LOGICAL_t,
    /// Whether to return edges (default: false).
    pub want_edges: PK_LOGICAL_t,
    /// Whether to return edge-to-curve mapping (default: false).
    pub want_indices: PK_LOGICAL_t,
    /// Whether curves are in connection sequence.
    pub sequential: PK_CURVE_sequential_t,
}

impl Default for PK_CURVE_make_wire_body_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            tolerance: 1.0e-6,
            allow_disjoint: PK_LOGICAL_true,
            allow_general: PK_LOGICAL_false,
            check: PK_LOGICAL_true,
            want_edges: PK_LOGICAL_false,
            want_indices: PK_LOGICAL_false,
            sequential: PK_CURVE_sequential_no_c,
        }
    }
}

/// Options for `PK_EDGE_make_wire_body`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_make_wire_body_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Whether disjoint bodies can be created (default: false).
    pub allow_disjoint: PK_LOGICAL_t,
    /// Copy curves with dependent geometry vs approximate with b-curves (default: true).
    pub copy_dep_geom: PK_LOGICAL_t,
    /// Use nominal geometry as real geometry (default: false).
    pub use_nmnl_geom: PK_LOGICAL_t,
    /// Tolerance for curve approximation (default: 1.0e-5).
    pub tolerance: c_double,
}

impl Default for PK_EDGE_make_wire_body_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            allow_disjoint: PK_LOGICAL_false,
            copy_dep_geom: PK_LOGICAL_true,
            use_nmnl_geom: PK_LOGICAL_false,
            tolerance: 1.0e-5,
        }
    }
}

/// Options for `PK_BODY_offset_planar_wire`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_offset_planar_wire_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Primary gap fill method.
    pub gap_fill: PK_VERTEX_gap_fill_t,
    /// Secondary gap fill method.
    pub constrained_gap_fill: PK_VERTEX_gap_fill_t,
    /// Angular interval (radians) controlling which gap fill method to use.
    pub gap_fill_angle: PK_INTERVAL_t,
    /// Whether to perform local checks.
    pub local_check: PK_LOGICAL_t,
    /// Tolerance for converting curves.
    pub tolerance: c_double,
    /// Trim self-intersecting edge pairs.
    pub repair_self_int: PK_LOGICAL_t,
    /// Whether result may be disjoint.
    pub allow_disjoint: PK_LOGICAL_t,
}

impl Default for PK_BODY_offset_planar_wire_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            gap_fill: PK_VERTEX_gap_fill_round_c,
            constrained_gap_fill: PK_VERTEX_gap_fill_round_c,
            gap_fill_angle: PK_INTERVAL_t {
                low: 0.0,
                high: std::f64::consts::PI,
            },
            local_check: PK_LOGICAL_true,
            tolerance: 1.0e-6,
            repair_self_int: PK_LOGICAL_false,
            allow_disjoint: PK_LOGICAL_false,
        }
    }
}

// =============================================================================
// Sheet modeling — options structures (Chapter 43)
// =============================================================================

/// Options for `PK_FACE_make_sheet_bodies`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_make_sheet_bodies_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Allow single disjoint body or several bodies (default: false).
    pub allow_disjoint: PK_LOGICAL_t,
    /// Copy input faces or use originals.
    pub make_from: PK_BODY_make_from_t,
    /// Inherit attributes from source topology (default: true).
    pub transfer_attribs: PK_LOGICAL_t,
    /// Transfer/copy groups to new sheet body.
    pub transfer_groups: PK_GROUP_transfer_t,
    /// Whether to return edges in tracking.
    pub track_edges: PK_track_edges_t,
    /// Whether to return vertices in tracking.
    pub track_vertices: PK_track_vertices_t,
}

impl Default for PK_FACE_make_sheet_bodies_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            allow_disjoint: PK_LOGICAL_false,
            make_from: PK_BODY_make_from_copy_c,
            transfer_attribs: PK_LOGICAL_true,
            transfer_groups: PK_GROUP_transfer_owning_c,
            track_edges: PK_track_edges_no_c,
            track_vertices: PK_track_vertices_no_c,
        }
    }
}

// =============================================================================
// Sewing and knitting — structures (Chapter 44)
// =============================================================================

/// Knitting pattern: edge-pair connectivity inferred between bodies.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_knit_pattern_t {
    /// Number of matching edge pairs.
    pub n_edges: c_int,
    /// Array of edges to match.
    pub edges: *const PK_EDGE_t,
    /// Array of matching edges (1:1 with `edges`).
    pub matches: *const PK_EDGE_t,
    /// Number of bodies needing reversal.
    pub n_reversals: c_int,
    /// Array of bodies to reverse.
    pub reversals: *const PK_BODY_t,
}

/// Options for `PK_BODY_sew_bodies`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_sew_bodies_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Allow disjoint composite sheets (default: false).
    pub allow_disjoint_result: PK_LOGICAL_t,
    /// Assume manifold result intent (default: true).
    pub treat_as_manifold: PK_LOGICAL_t,
    /// Preference for result body type.
    pub prefered_body_type: PK_BODY_sewing_t,
    /// Duplicate sheet removal strategy.
    pub duplicate_removal: PK_BODY_sewing_remove_t,
    /// Sew edges in same inner loop.
    pub sew_up_inner_loops: PK_LOOP_sew_up_t,
    /// Attempt to reduce sewn edge tolerance.
    pub reduce_edge_tolerance: PK_EDGE_reduce_tol_t,
    /// Number of incremental sewing passes (default: 1).
    pub number_of_iterations: c_int,
    /// Array of gap-width bounds for incremental sewing (NULL for single pass).
    pub iteration_bounds: *const c_double,
    /// Assembly sewing method.
    pub assembly_sewing: PK_BODY_sewing_assy_t,
    /// General topology sewing mode.
    pub general_sewing: PK_BODY_sewing_gen_t,
}

impl Default for PK_BODY_sew_bodies_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            allow_disjoint_result: PK_LOGICAL_false,
            treat_as_manifold: PK_LOGICAL_true,
            prefered_body_type: PK_BODY_sewing_any_c,
            duplicate_removal: PK_BODY_sewing_remove_none_c,
            sew_up_inner_loops: PK_LOOP_sew_up_loop_c,
            reduce_edge_tolerance: PK_EDGE_reduce_tol_no_c,
            number_of_iterations: 1,
            iteration_bounds: std::ptr::null(),
            assembly_sewing: PK_BODY_sewing_assy_orient_c,
            general_sewing: PK_BODY_sewing_gen_no_c,
        }
    }
}

/// Options for `PK_BODY_apply_knit_pattern`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_apply_knit_pattern_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Result body type preference.
    pub body_type: PK_BODY_type_prefer_t,
    /// Whether to sort faces into shells.
    pub sort_face_shells: PK_LOGICAL_t,
    /// Whether to close gaps not addressed by knit pattern.
    pub close_marginal_gaps: PK_LOGICAL_t,
}

impl Default for PK_BODY_apply_knit_pattern_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            body_type: PK_BODY_type_prefer_solid_c,
            sort_face_shells: PK_LOGICAL_false,
            close_marginal_gaps: PK_LOGICAL_false,
        }
    }
}

/// Options for `PK_BODY_knit`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_knit_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Body type preference for result.
    pub body_type: PK_BODY_type_prefer_t,
}

impl Default for PK_BODY_knit_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            body_type: PK_BODY_type_prefer_solid_c,
        }
    }
}

// =============================================================================
// Mid-surface — options structures (Chapter 45)
// =============================================================================

/// Options for `PK_FACE_make_neutral_sheet_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_make_neutral_sheet_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// If false, test suitability only (no sheet created).
    pub make_sheet: PK_LOGICAL_t,
    /// Whether face overlap detection is used.
    pub overlap: PK_neutral_face_overlap_t,
    /// Number of construction methods (0 = use default).
    pub n_methods: c_int,
    /// Array of construction methods.
    pub methods: *const PK_neutral_method_t,
    /// Fill holes and extend gaps in neutral sheets (default: true).
    pub extend_and_fill_holes: PK_LOGICAL_t,
    /// Whether tolerance value is provided.
    pub have_tolerance: PK_LOGICAL_t,
    /// Tolerance for offset between face sets.
    pub tolerance: c_double,
}

impl Default for PK_FACE_make_neutral_sheet_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            make_sheet: PK_LOGICAL_true,
            overlap: PK_neutral_face_overlap_no_c,
            n_methods: 0,
            methods: std::ptr::null(),
            extend_and_fill_holes: PK_LOGICAL_true,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
        }
    }
}

/// Options for `PK_BODY_trim_neutral_sheets_2`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_trim_neutral_sheets_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Whether to ignore small faces (blends, slivers).
    pub ignore: PK_neutral_ignore_fa_t,
    /// How to handle overlapping faces.
    pub overlap: PK_neutral_overlap_t,
    /// Trim against other sheets only, or also against side faces.
    pub trim_method: PK_neutral_trim_method_t,
    /// Extend sheets and fill holes during trimming (default: false).
    pub extend_and_fill_holes: PK_LOGICAL_t,
}

impl Default for PK_BODY_trim_neutral_sheets_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            ignore: PK_neutral_ignore_fa_default_c,
            overlap: PK_neutral_overlap_no_c,
            trim_method: PK_neutral_trim_method_sheets_c,
            extend_and_fill_holes: PK_LOGICAL_false,
        }
    }
}

// =============================================================================
// Extending sheets and surfaces — structures (Chapter 46)
// =============================================================================

/// Per-vertex side edge control data for `PK_BODY_extend`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_extend_side_data_t {
    /// Number of terminal vertices of edge chains.
    pub n_vertices: c_int,
    /// Terminal vertices.
    pub vertices: *const PK_VERTEX_t,
    /// Per-vertex side edge construction methods.
    pub extend_sides: *const PK_extend_side_t,
}

impl Default for PK_extend_side_data_t {
    fn default() -> Self {
        Self {
            n_vertices: 0,
            vertices: std::ptr::null(),
            extend_sides: std::ptr::null(),
        }
    }
}

/// Options for `PK_BODY_extend`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_extend_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Whether to modify original faces (default: false).
    pub modify: PK_LOGICAL_t,
    /// Shape of extension.
    pub extension_shape: PK_extension_shape_t,
    /// Distance to extend (for distance method).
    pub extension_distance: c_double,
    /// Target body (for extend-to-target method).
    pub target: PK_BODY_t,
    /// Where extension stops relative to target.
    pub target_limit: PK_extension_limit_t,
    /// Whether to preserve G1 smoothness across internal edges.
    pub preserve_internal_smoothness: PK_extension_smoothness_t,
    /// Tracking detail for new laminar side edges.
    pub track_laminar: PK_extend_track_laminar_t,
    /// Whether to track non-laminar side edges.
    pub track_internal: PK_extend_track_internal_t,
    /// How new topology is created.
    pub extend_create: PK_extend_create_t,
    /// Precise or loose boundary.
    pub extension_boundary: PK_extension_boundary_t,
    /// Side edge construction method.
    pub extend_side: PK_extend_side_t,
    /// Per-vertex side edge control.
    pub extend_side_data: PK_extend_side_data_t,
    /// Update control.
    pub update: PK_local_ops_update_t,
    /// Return info as argument or in report.
    pub results_output: PK_results_output_t,
}

impl Default for PK_BODY_extend_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            modify: PK_LOGICAL_false,
            extension_shape: PK_extension_shape_linear_c,
            extension_distance: 0.0,
            target: PK_ENTITY_null,
            target_limit: PK_extension_limit_minimal_c,
            preserve_internal_smoothness: PK_extension_smoothness_g1_c,
            track_laminar: PK_extend_track_laminar_basic_c,
            track_internal: PK_extend_track_internal_no_c,
            extend_create: PK_extend_create_new_c,
            extension_boundary: PK_extension_boundary_precise_c,
            extend_side: PK_extend_side_default_c,
            extend_side_data: PK_extend_side_data_t::default(),
            update: PK_local_ops_update_default_c,
            results_output: PK_results_output_return_c,
        }
    }
}

/// Options for `PK_SURF_extend`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_SURF_extend_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// Type of extension.
    pub extension_type: PK_SURF_extension_t,
    /// Point to extend to (for point mode).
    pub extension_point: PK_VECTOR_t,
    /// Box to extend to (for box mode).
    pub extension_box: PK_BOX_t,
    /// Parameter box to extend to (for uvbox mode).
    pub extension_uvbox: PK_UVBOX_t,
    /// Extend low u boundary by ratio.
    pub u_ratio: c_double,
    /// Extend high u boundary by ratio.
    pub U_ratio: c_double,
    /// Extend low v boundary by ratio.
    pub v_ratio: c_double,
    /// Extend high v boundary by ratio.
    pub V_ratio: c_double,
    /// Allow partial extension if full would be invalid (default: false).
    pub allow_partial_extension: PK_LOGICAL_t,
    /// Shape of extension.
    pub extension_shape: PK_extension_shape_t,
    /// Update control.
    pub update: PK_surf_extend_update_t,
}

impl Default for PK_SURF_extend_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            extension_type: PK_SURF_extension_none_c,
            extension_point: [0.0; 3],
            extension_box: PK_BOX_t { coord: [0.0; 6] },
            extension_uvbox: PK_UVBOX_t { param: [0.0; 4] },
            u_ratio: 0.0,
            U_ratio: 0.0,
            v_ratio: 0.0,
            V_ratio: 0.0,
            allow_partial_extension: PK_LOGICAL_false,
            extension_shape: PK_extension_shape_linear_c,
            update: PK_surf_extend_update_default_c,
        }
    }
}

// =============================================================================
// Extern function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // Wire modeling (Chapter 42)
    // =========================================================================

    /// Offsets a planar wire body in its plane.
    ///
    /// Does NOT support facet geometry.
    ///
    /// # Arguments
    /// * `body` - Planar wire body to offset.
    /// * `distance` - Offset distance.
    /// * `options` - Options structure.
    /// * `new_body` - (out) New offset wire body.
    /// * `tracking` - (out) Tracking information.
    pub fn PK_BODY_offset_planar_wire(
        wire_body: PK_BODY_t,
        offset: c_double,
        normal: *const PK_VECTOR1_t,
        r#ref: PK_EDGE_t,
        options: *mut PK_BODY_offset_planar_wire_o_t,
        n_new_wires: *mut c_int,
        new_wires: *mut *mut PK_BODY_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Reverses an edge and its associated geometry.
    ///
    /// Supports wire, sheet, solid, and general bodies.
    pub fn PK_EDGE_reverse_2(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        options: *mut PK_EDGE_reverse_2_o_t,
    ) -> PK_ERROR_code_t;

    /// Orientates all edges of a wire body in the same direction as a given edge.
    pub fn PK_EDGE_propagate_orientation(
        edge: PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Splits a wire body by adding a new vertex at the end of a specified edge.
    ///
    /// # Arguments
    /// * `vertex` - Vertex at which to split.
    /// * `edge` - Edge to add a new vertex to.
    /// * `new_body` - (out) Second body resulting from the split.
    pub fn PK_VERTEX_remove_edge(
        vertex: PK_VERTEX_t,
        edge: PK_EDGE_t,
        new_body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Sheet modeling (Chapter 43)
    // =========================================================================

    /// Trims a sheet body by dividing it into regions.
    ///
    /// # Arguments
    /// * `body` - Sheet body to trim.
    /// * `n_edges` - Number of dividing edges.
    /// * `edges` - Edges defining regions on the body.
    /// * `n_faces` - Number of indicator faces.
    /// * `faces` - Faces indicating regions to keep or discard.
    /// * `keep` - True = keep listed face regions; false = delete them.
    pub fn PK_BODY_trim(
        body: PK_BODY_t,
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        n_faces: c_int,
        faces: *const PK_FACE_t,
        keep: PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Analyzes gaps when PK_BODY_trim fails.
    ///
    /// Returns vertices bordering gaps of size less than supplied tolerance.
    ///
    /// # Arguments
    /// * `body` - Sheet body that failed trimming.
    /// * `n_edges` - Number of edges.
    /// * `edges` - Edges that defined trim regions.
    /// * `tolerance` - Gap tolerance.
    /// * `n_vertices` - (out) Number of vertices bordering gaps.
    /// * `vertices` - (out) Array of gap-bordering vertex tags.
    pub fn PK_BODY_trim_gap_analysis(
        body: PK_BODY_t,
        n_edges: c_int,
        edges: *const PK_EDGE_t,
        tolerance: c_double,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Replaces the surface of a sheet to a new surface of the same type.
    ///
    /// Does NOT support facet geometry. SP-curves and constant parameter curves
    /// are transferred to the new surface; other edge curves are converted to
    /// SP-curves first.
    ///
    /// # Arguments
    /// * `body` - Sheet body whose surface is replaced.
    /// * `surf` - New surface (must be same type as original).
    pub fn PK_BODY_embed_in_surf(
        body: PK_BODY_t,
        surf: PK_SURF_t,
        tolerance: c_double,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Sewing and knitting (Chapter 44)
    // =========================================================================

    /// Calculates maximal distance between a pair of matching edges.
    ///
    /// Distance is between edge geometries; subtract sum of edge precisions
    /// for gap-width.
    ///
    /// # Arguments
    /// * `edge1` - First edge.
    /// * `edge2` - Second edge.
    /// * `n_samples` - Number of distance samples requested.
    /// * `max_deviation` - (out) Maximum deviation between edges.
    /// * `n_deviations` - (out) Number of deviation samples returned.
    /// * `deviations` - (out) Array of deviation samples.
    pub fn PK_EDGE_find_deviation_2(
        edge1: PK_EDGE_t,
        edge2: PK_EDGE_t,
        options: *mut PK_EDGE_find_deviation_o_t,
        result: *mut PK_EDGE_find_deviation_r_t,
    ) -> PK_ERROR_code_t;

    /// Joins bodies using 1:1 matching of supplied topologies (edges or vertices).
    ///
    /// # Arguments
    /// * `body` - Target body.
    /// * `n_topols` - Number of topologies (may contain duplicates).
    /// * `topols` - Array of topology tags.
    /// * `matches` - Array of matching topology tags (no duplicates; these are removed).
    /// * `options` - Options structure.
    /// * `tracking` - (out) Tracking information.
    pub fn PK_BODY_knit(
        body: PK_BODY_t,
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        matches: *const PK_TOPOL_t,
        options: *const PK_BODY_knit_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Mid-surface (Chapter 45)
    // =========================================================================

    /// Creates a neutral sheet between two arrays of faces.
    ///
    /// Surfaces need not be exact offsets if construction method allows.
    /// Neutral sheets may be untrimmed; use `PK_BODY_trim_neutral_sheets_2` afterward.
    /// Does NOT support facet bodies.
    ///
    /// # Arguments
    /// * `n_left_faces` - Number of left faces.
    /// * `left_faces` - Array of left face tags.
    /// * `n_right_faces` - Number of right faces.
    /// * `right_faces` - Array of right face tags.
    /// * `placement` - Position control (0 = halfway between arrays).
    /// * `options` - Options structure.
    /// * `n_bodies` - (out) Number of neutral sheet bodies created.
    /// * `bodies` - (out) Array of neutral sheet body tags.
    pub fn PK_FACE_make_neutral_sheet_2(
        n_left_faces: c_int,
        left_faces: *mut PK_FACE_t,
        n_right_faces: c_int,
        right_faces: *mut PK_FACE_t,
        placement: c_double,
        options: *mut PK_FACE_make_neutral_sheet_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Trims neutral sheets using the original solid body and face set pair info.
    ///
    /// Creates edges where neutral sheets meet, at exterior boundaries, and at
    /// abrupt thickness changes. Deletes unwanted faces after edge creation.
    /// Does NOT support facet bodies.
    ///
    /// # Arguments
    /// * `body` - Original solid body.
    /// * `n_neutral_bodies` - Number of neutral sheet bodies.
    /// * `neutral_bodies` - Array of neutral sheet body tags.
    /// * `n_left_faces` - Number of left faces.
    /// * `left_faces` - Array of left face tags.
    /// * `n_right_faces` - Number of right faces.
    /// * `right_faces` - Array of right face tags.
    /// * `options` - Options structure.
    /// * `tracking` - (out) Tracking information.
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_BODY_trim_neutral_sheets_2(
        body: PK_BODY_t,
        n_pairs: c_int,
        pairs: *mut PK_FACE_set_pair_t,
        tol: c_double,
        options: *mut PK_BODY_trim_neutral_sheets_o_t,
        neutral_sheets: *mut PK_BODY_t,
        errors: *mut PK_neutral_error_t,
        causes: *mut PK_FACE_neutral_causes_array_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Extending sheets and surfaces (Chapter 46)
    // =========================================================================

    /// Extends a body across a set of laminar edges.
    ///
    /// Body may contain non-manifold components if supplied edges are
    /// locally manifold. Partial support for facet geometry.
    ///
    /// # Arguments
    /// * `body` - Body to extend.
    /// * `n_boundary_edges` - Number of laminar edges.
    /// * `boundary_edges` - Array of laminar edges to extend across.
    /// * `options` - Options structure.
    /// * `tracking` - (out) Tracking information.
    pub fn PK_BODY_extend(
        body: PK_BODY_t,
        n_boundary_edges: c_int,
        boundary_edges: *mut PK_EDGE_t,
        options: *mut PK_BODY_extend_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_TOPOL_local_r_t,
    ) -> PK_ERROR_code_t;

    /// Extends surfaces of any class (except foreign geometry).
    ///
    /// Partial support for facet geometry.
    ///
    /// # Arguments
    /// * `surf` - Surface to extend.
    /// * `options` - Options structure.
    /// * `status` - (out) Extension status.
    pub fn PK_SURF_extend(
        surf: PK_SURF_t,
        options: *const PK_SURF_extend_o_t,
        status: *mut PK_SURF_extend_status_t,
    ) -> PK_ERROR_code_t;

}
