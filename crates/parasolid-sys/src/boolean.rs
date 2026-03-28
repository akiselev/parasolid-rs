//! Boolean operations, imprinting, sectioning, instancing, and patterning.
//!
//! Covers:
//! - Global booleans: `PK_BODY_boolean_2`
//! - Local booleans: `PK_FACE_boolean_2`
//! - Specialised manifold booleans: `PK_BODY_unite_bodies`, `PK_BODY_subtract_bodies`,
//!   `PK_BODY_intersect_bodies`
//! - Imprinting: `PK_CURVE_project`, `PK_BODY_imprint_*`, `PK_FACE_imprint_*`,
//!   `PK_EDGE_imprint_point`, `PK_FACE_imprint_point`, `PK_REGION_imprint_point`
//! - Sectioning: `PK_BODY_section_with_surf`, `PK_BODY_section_with_sheet_2`,
//!   `PK_FACE_section_with_sheet_2`, `PK_BODY_make_section`, `PK_BODY_make_section_with_surfs`,
//!   `PK_FACE_make_sect_with_sfs`
//! - Instancing/patterning: `PK_FACE_instance_tools`, `PK_FACE_instance_bodies`, `PK_FACE_pattern`

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;
use std::os::raw::{c_double, c_int};

// =============================================================================
// Boolean function enum
// =============================================================================

pub type PK_boolean_function_t = c_int;
pub const PK_boolean_unite_c: PK_boolean_function_t = 0;
pub const PK_boolean_subtract_c: PK_boolean_function_t = 1;
pub const PK_boolean_intersect_c: PK_boolean_function_t = 2;

// =============================================================================
// Boolean material side enum
// =============================================================================

pub type PK_boolean_material_t = c_int;
pub const PK_boolean_material_default_c: PK_boolean_material_t = 0;
pub const PK_boolean_material_inside_c: PK_boolean_material_t = 1;
pub const PK_boolean_material_outside_c: PK_boolean_material_t = 2;
pub const PK_boolean_material_none_c: PK_boolean_material_t = 3;

// =============================================================================
// Boolean match style enum
// =============================================================================

pub type PK_boolean_match_style_t = c_int;
pub const PK_boolean_match_style_basic_c: PK_boolean_match_style_t = 0;
pub const PK_boolean_match_style_auto_c: PK_boolean_match_style_t = 1;
pub const PK_boolean_match_style_relax_c: PK_boolean_match_style_t = 2; // DEPRECATED

// =============================================================================
// Boolean match type enum
// =============================================================================

pub type PK_boolean_match_type_t = c_int;
pub const PK_boolean_match_exact_c: PK_boolean_match_type_t = 0;
pub const PK_boolean_match_contains_c: PK_boolean_match_type_t = 1;
pub const PK_boolean_match_overlap_c: PK_boolean_match_type_t = 2;
pub const PK_boolean_match_imprinted_c: PK_boolean_match_type_t = 3;

// =============================================================================
// Face overflow enum (laminar)
// =============================================================================

pub type PK_FACE_overflow_t = c_int;
pub const PK_FACE_overflow_tangent_c: PK_FACE_overflow_t = 0;
pub const PK_FACE_overflow_ruled_c: PK_FACE_overflow_t = 1;
pub const PK_FACE_overflow_swept_c: PK_FACE_overflow_t = 2;

// Face overflow enum (interior)
pub const PK_FACE_overflow_none_c: PK_FACE_overflow_t = 10;
pub const PK_FACE_overflow_added_c: PK_FACE_overflow_t = 11;
pub const PK_FACE_overflow_mixed_c: PK_FACE_overflow_t = 12;

// =============================================================================
// Selector type enum
// =============================================================================

pub type PK_selector_type_t = c_int;
pub const PK_selector_type_off_c: PK_selector_type_t = 0;
pub const PK_selector_type_exclude_c: PK_selector_type_t = 1;
pub const PK_selector_type_include_c: PK_selector_type_t = 2;

// =============================================================================
// Selector split action enum
// =============================================================================

pub type PK_selector_split_t = c_int;
pub const PK_selector_split_fail_c: PK_selector_split_t = 0;
pub const PK_selector_split_propagate_c: PK_selector_split_t = 1;

// =============================================================================
// Boolean select enum (local)
// =============================================================================

pub type PK_boolean_select_t = c_int;
pub const PK_boolean_include_c: PK_boolean_select_t = 0;
pub const PK_boolean_exclude_c: PK_boolean_select_t = 1;
pub const PK_boolean_mixed_selection_c: PK_boolean_select_t = 2;

// =============================================================================
// Resulting body type preference enum
// =============================================================================

pub type PK_boolean_prefer_t = c_int;
pub const PK_boolean_prefer_original_c: PK_boolean_prefer_t = 0;
pub const PK_boolean_prefer_solid_c: PK_boolean_prefer_t = 1;
pub const PK_boolean_prefer_sheet_c: PK_boolean_prefer_t = 2;
pub const PK_boolean_prefer_wire_c: PK_boolean_prefer_t = 3;
pub const PK_boolean_prefer_general_c: PK_boolean_prefer_t = 4;
pub const PK_boolean_prefer_simplest_c: PK_boolean_prefer_t = 5;

// =============================================================================
// Non-manifold edge repair enum
// =============================================================================

pub type PK_nm_edge_repair_t = c_int;
pub const PK_nm_edge_repair_no_c: PK_nm_edge_repair_t = 0;
pub const PK_nm_edge_repair_blend_c: PK_nm_edge_repair_t = 1;

// =============================================================================
// Boolean tracking type enum
// =============================================================================

pub type PK_boolean_track_type_t = c_int;
pub const PK_boolean_track_type_basic_c: PK_boolean_track_type_t = 0;
pub const PK_boolean_track_type_comp_c: PK_boolean_track_type_t = 1;

// =============================================================================
// Region tracking enum
// =============================================================================

pub type PK_region_track_t = c_int;
pub const PK_region_track_no_c: PK_region_track_t = 0;
pub const PK_region_track_basic_c: PK_region_track_t = 1;

// =============================================================================
// Boolean fence enum
// =============================================================================

pub type PK_boolean_fence_t = c_int;
pub const PK_boolean_fence_none_c: PK_boolean_fence_t = 0;
pub const PK_boolean_fence_front_c: PK_boolean_fence_t = 1;
pub const PK_boolean_fence_back_c: PK_boolean_fence_t = 2;

// =============================================================================
// Boolean no-effect detection enum
// =============================================================================

pub type PK_boolean_no_effect_t = c_int;
pub const PK_boolean_no_effect_basic_c: PK_boolean_no_effect_t = 0;
pub const PK_boolean_no_effect_advanced_c: PK_boolean_no_effect_t = 1;

// =============================================================================
// Boolean face check enum
// =============================================================================

pub type PK_boolean_check_fa_t = c_int;
pub const PK_boolean_check_fa_yes_c: PK_boolean_check_fa_t = 0;
pub const PK_boolean_check_fa_no_c: PK_boolean_check_fa_t = 1;

// =============================================================================
// Boolean update (version compat) enum
// =============================================================================

pub type PK_boolean_update_t = c_int;
pub const PK_boolean_update_default_c: PK_boolean_update_t = 0;
pub const PK_boolean_update_0_c: PK_boolean_update_t = 1;
pub const PK_boolean_update_5_c: PK_boolean_update_t = 5;
pub const PK_boolean_update_v261_c: PK_boolean_update_t = 261;

// =============================================================================
// Topology dimension enum (for merge options)
// =============================================================================

pub const PK_TOPOL_dimension_1_c: PK_TOPOL_dimension_t = 1;
pub const PK_TOPOL_dimension_2_c: PK_TOPOL_dimension_t = 2;

// =============================================================================
// Topology track record enum
// =============================================================================

pub type PK_TOPOL_track_record_t = c_int;
pub const PK_TOPOL_track_derive_c: PK_TOPOL_track_record_t = 0;
pub const PK_TOPOL_track_create_c: PK_TOPOL_track_record_t = 1;

// =============================================================================
// Boolean result status enum
// =============================================================================

pub type PK_boolean_result_t = c_int;
pub const PK_boolean_result_success_c: PK_boolean_result_t = 0;
pub const PK_boolean_result_no_clash_c: PK_boolean_result_t = 1;
pub const PK_boolean_result_no_effect_c: PK_boolean_result_t = 2;
pub const PK_boolean_result_imprint_c: PK_boolean_result_t = 3;
pub const PK_boolean_result_not_solid_c: PK_boolean_result_t = 4;
pub const PK_boolean_result_multiple_c: PK_boolean_result_t = 5; // DEPRECATED
pub const PK_boolean_result_failed_c: PK_boolean_result_t = -1;

// =============================================================================
// Body type enum
// =============================================================================

// =============================================================================
// Imprint-related enums
// =============================================================================

pub type PK_imprint_complete_t = c_int;
pub const PK_imprint_complete_no_c: PK_imprint_complete_t = 0;
pub const PK_imprint_complete_edge_c: PK_imprint_complete_t = 1;
pub const PK_imprint_complete_laminar_c: PK_imprint_complete_t = 2;
pub const PK_imprint_complete_faceset_c: PK_imprint_complete_t = 3;

pub type PK_imprint_extend_t = c_int;
pub const PK_imprint_extend_tangent_c: PK_imprint_extend_t = 0;
pub const PK_imprint_extend_orthogonal_c: PK_imprint_extend_t = 1;

pub type PK_imprint_face_list_t = c_int;
pub const PK_imprint_face_list_no_c: PK_imprint_face_list_t = 0;
pub const PK_imprint_face_list_target_c: PK_imprint_face_list_t = 1;
pub const PK_imprint_face_list_tool_c: PK_imprint_face_list_t = 2;
pub const PK_imprint_face_list_both_c: PK_imprint_face_list_t = 3;

pub type PK_imprint_dir_t = c_int;
pub const PK_imprint_dir_no_check_c: PK_imprint_dir_t = 0;
pub const PK_imprint_dir_consistent_c: PK_imprint_dir_t = 1;

pub type PK_imprint_precision_t = c_int;
pub const PK_imprint_precision_auto_c: PK_imprint_precision_t = 0;
pub const PK_imprint_precision_accurate_c: PK_imprint_precision_t = 1;

pub type PK_imprint_connect_t = c_int;
pub const PK_imprint_connect_none_c: PK_imprint_connect_t = 0;
pub const PK_imprint_connect_side_c: PK_imprint_connect_t = 1;
pub const PK_imprint_connect_side_all_c: PK_imprint_connect_t = 2;
pub const PK_imprint_connect_all_c: PK_imprint_connect_t = 3;
pub const PK_imprint_connect_hidden_all_c: PK_imprint_connect_t = 4;

pub type PK_imprint_intersect_t = c_int;
pub const PK_imprint_intersect_fix_c: PK_imprint_intersect_t = 0;
pub const PK_imprint_intersect_fail_c: PK_imprint_intersect_t = 1;
pub const PK_imprint_intersect_update_c: PK_imprint_intersect_t = 2;

pub type PK_imprint_proj_dist_t = c_int;
pub const PK_imprint_proj_dist_no_c: PK_imprint_proj_dist_t = 0;
pub const PK_imprint_proj_dist_whole_c: PK_imprint_proj_dist_t = 1;

pub type PK_imprint_tracking_t = c_int;
pub const PK_imprint_tracking_basic_c: PK_imprint_tracking_t = 0;
pub const PK_imprint_tracking_curves_c: PK_imprint_tracking_t = 1;

pub type PK_FACE_imprint_hidden_t = c_int;
pub const PK_FACE_imprint_hidden_no_c: PK_FACE_imprint_hidden_t = 0;
pub const PK_FACE_imprint_hidden_body_c: PK_FACE_imprint_hidden_t = 1;
pub const PK_FACE_imprint_hidden_array_c: PK_FACE_imprint_hidden_t = 2;

// =============================================================================
// Projection enums (PK_CURVE_project)
// =============================================================================

pub type PK_proj_function_t = c_int;
pub const PK_proj_function_project_c: PK_proj_function_t = 0;
pub const PK_proj_function_imprint_c: PK_proj_function_t = 1;
pub const PK_proj_function_both_c: PK_proj_function_t = 2;

pub type PK_proj_method_t = c_int;
pub const PK_proj_method_unset_c: PK_proj_method_t = 0;
pub const PK_proj_method_normal_c: PK_proj_method_t = 1;
pub const PK_proj_method_vector_c: PK_proj_method_t = 2;
pub const PK_proj_method_perspective_c: PK_proj_method_t = 3;

pub type PK_proj_max_dist_t = c_int;
pub const PK_proj_max_dist_no_c: PK_proj_max_dist_t = 0;
pub const PK_proj_max_dist_whole_c: PK_proj_max_dist_t = 1;

pub type PK_proj_face_hidden_t = c_int;
pub const PK_proj_face_hidden_no_c: PK_proj_face_hidden_t = 0;
pub const PK_proj_face_hidden_array_c: PK_proj_face_hidden_t = 1;
pub const PK_proj_face_hidden_body_c: PK_proj_face_hidden_t = 2;

pub type PK_proj_connect_t = c_int;
pub const PK_proj_connect_none_c: PK_proj_connect_t = 0;
pub const PK_proj_connect_side_c: PK_proj_connect_t = 1;
pub const PK_proj_connect_all_c: PK_proj_connect_t = 2;
pub const PK_proj_connect_side_all_c: PK_proj_connect_t = 3;
pub const PK_proj_connect_hidden_all_c: PK_proj_connect_t = 4;

pub type PK_proj_split_clash_t = c_int;
pub const PK_proj_split_clash_no_c: PK_proj_split_clash_t = 0;
pub const PK_proj_split_clash_self_c: PK_proj_split_clash_t = 1;
pub const PK_proj_split_clash_all_c: PK_proj_split_clash_t = 2;

pub type PK_proj_to_points_t = c_int;
pub const PK_proj_to_points_no_c: PK_proj_to_points_t = 0;
pub const PK_proj_to_points_end_on_c: PK_proj_to_points_t = 1;
pub const PK_proj_to_points_tol_c: PK_proj_to_points_t = 2;
pub const PK_proj_to_points_all_c: PK_proj_to_points_t = 3; // DEPRECATED

pub type PK_proj_nominal_t = c_int;
pub const PK_proj_nominal_no_c: PK_proj_nominal_t = 0;
pub const PK_proj_nominal_yes_c: PK_proj_nominal_t = 1;

pub type PK_proj_complete_t = c_int;
pub const PK_proj_complete_no_c: PK_proj_complete_t = 0;
pub const PK_proj_complete_edge_c: PK_proj_complete_t = 1;
pub const PK_proj_complete_faceset_c: PK_proj_complete_t = 2;

pub type PK_complete_bound_t = c_int;
pub const PK_complete_bound_none_c: PK_complete_bound_t = 0;
pub const PK_complete_bound_if_within_c: PK_complete_bound_t = 1;

pub type PK_proj_tracking_t = c_int;
pub const PK_proj_tracking_basic_c: PK_proj_tracking_t = 0;
pub const PK_proj_tracking_completion_c: PK_proj_tracking_t = 1;

pub type PK_results_output_t = c_int;
pub const PK_results_output_return_c: PK_results_output_t = 0;
pub const PK_results_output_report_c: PK_results_output_t = 1;

pub const PK_proj_update_default_c: PK_proj_update_t = 0;

pub const PK_continuity_c2_c: PK_continuity_t = 2;

// =============================================================================
// Instancing/patterning enums
// =============================================================================

pub type PK_instance_repair_fa_fa_t = c_int;
pub const PK_instance_repair_fa_fa_yes_c: PK_instance_repair_fa_fa_t = 0;
pub const PK_instance_repair_fa_fa_no_c: PK_instance_repair_fa_fa_t = 1;

pub type PK_instance_merge_t = c_int;
pub const PK_instance_merge_new_c: PK_instance_merge_t = 0;
pub const PK_instance_merge_no_c: PK_instance_merge_t = 1;

pub type PK_instance_track_type_t = c_int;
pub const PK_instance_track_type_none_c: PK_instance_track_type_t = 0;
pub const PK_instance_track_type_inst_c: PK_instance_track_type_t = 1;
pub const PK_instance_track_type_topol_c: PK_instance_track_type_t = 2;
pub const PK_instance_track_type_both_c: PK_instance_track_type_t = 3;

pub type PK_instance_track_edges_t = c_int;
pub const PK_instance_track_edges_no_c: PK_instance_track_edges_t = 0;
pub const PK_instance_track_edges_laminar_c: PK_instance_track_edges_t = 1;
pub const PK_instance_track_edges_new_c: PK_instance_track_edges_t = 2;

pub type PK_pattern_check_loops_t = c_int;
pub const PK_pattern_check_loops_no_c: PK_pattern_check_loops_t = 0;
pub const PK_pattern_check_loops_yes_c: PK_pattern_check_loops_t = 1;
pub const PK_pattern_check_loops_outside_c: PK_pattern_check_loops_t = 2;

pub type PK_pattern_same_face_t = c_int;
pub const PK_pattern_same_face_yes_c: PK_pattern_same_face_t = 0;
pub const PK_pattern_same_face_no_c: PK_pattern_same_face_t = 1;

pub type PK_pattern_coi_face_t = c_int;
pub const PK_pattern_coi_face_yes_c: PK_pattern_coi_face_t = 0;
pub const PK_pattern_coi_face_unknown_c: PK_pattern_coi_face_t = 1;

pub type PK_pattern_reblend_t = c_int;
pub const PK_pattern_reblend_no_c: PK_pattern_reblend_t = 0;
pub const PK_pattern_reblend_yes_c: PK_pattern_reblend_t = 1;

pub type PK_pattern_collision_t = c_int;
pub const PK_pattern_collision_no_c: PK_pattern_collision_t = 0;
pub const PK_pattern_collision_yes_c: PK_pattern_collision_t = 1;

pub type PK_pattern_status_t = c_int;
pub const PK_pattern_status_ok_c: PK_pattern_status_t = 0;
pub const PK_pattern_status_colliding_c: PK_pattern_status_t = 1;

// =============================================================================
// Matched-region option sub-structure
// =============================================================================

/// A single matched-region pair.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_match_region_o_t {
    /// Pair of matched entities (target, tool).
    pub regions: [PK_ENTITY_t; 2],
    /// Type of match (exact, contains, overlap, imprinted).
    pub match_type: PK_boolean_match_type_t,
    /// Coincidence tolerance.
    pub tolerance: c_double,
}

/// Matched-region sub-structure for booleans.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_match_o_t {
    /// Matching style (basic, auto, relax).
    pub match_style: PK_boolean_match_style_t,
    /// Supply tolerance for auto matching.
    pub auto_match: PK_LOGICAL_t,
    /// Auto match tolerance.
    pub auto_match_tol: c_double,
    /// Number of supplied match regions.
    pub n_match_regions: c_int,
    /// Array of match regions.
    pub match_regions: *const PK_boolean_match_region_o_t,
    /// Version compatibility.
    pub update: c_int,
}

// =============================================================================
// Face overflow sub-structure
// =============================================================================

/// Controls overflow behaviour for target or tool faces at laminar/interior boundaries.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_overflow_o_t {
    /// How to handle laminar boundary overflow.
    pub laminar_overflow: PK_FACE_overflow_t,
    /// Sweep direction for swept overflow.
    pub sweep_direction: PK_VECTOR_t,
    /// Create side face at overflow boundary.
    pub laminar_walled: PK_LOGICAL_t,
    /// How to handle interior boundary overflow.
    pub interior_overflow: PK_FACE_overflow_t,
}

// =============================================================================
// Selected topology set sub-structure (global booleans)
// =============================================================================

/// Region-selection sub-structure for global booleans.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_topolset_o_t {
    /// Number of selector topologies.
    pub n_selectors: c_int,
    /// Array of regions/faces/edges/vertices.
    pub selectors: *const PK_ENTITY_t,
    /// Number of help points (0..n_selectors).
    pub n_help_points: c_int,
    /// Array of help-point vectors identifying topolsets.
    pub help_points: *const PK_VECTOR_t,
    /// Selector type for target.
    pub target_select: PK_selector_type_t,
    /// Selector type for tool.
    pub tool_select: PK_selector_type_t,
    /// What to do if selector is split during boolean.
    pub split_action: PK_selector_split_t,
}

// =============================================================================
// Select region sub-structure (local booleans)
// =============================================================================

/// Region-selection sub-structure for local booleans.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_select_region_o_t {
    /// Include/exclude/mixed.
    pub select_type: PK_boolean_select_t,
    /// Number of selectors.
    pub n_selectors: c_int,
    /// Array of faces/edges/vertices.
    pub selectors: *const PK_ENTITY_t,
    /// Number of help points.
    pub n_help_points: c_int,
    /// Optional help points.
    pub help_points: *const PK_VECTOR_t,
    /// Optional type per selector.
    pub selector_types: *const PK_selector_type_t,
    /// Optional include/exclude per body.
    pub region_types: *const PK_boolean_select_t,
}

// =============================================================================
// Configuration sub-structure (performance hints)
// =============================================================================

/// Performance optimization hints for boolean/instancing operations.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_config_o_t {
    /// Tools do not intersect each other.
    pub no_tool_intersect: PK_LOGICAL_t,
    /// No interference between instances and existing target edges.
    pub no_loop_intersect: PK_LOGICAL_t,
    /// All instance intersection loops are identical.
    pub identical_intersect: PK_LOGICAL_t,
    /// If one tool intersects a target face, all do.
    pub one_in_all_in: PK_LOGICAL_t,
}

// =============================================================================
// PK_BODY_boolean_o_t — Global boolean options
// =============================================================================

/// Options for `PK_BODY_boolean_2` (global boolean).
#[repr(C)]
pub struct PK_BODY_boolean_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Boolean operation type (unite/subtract/intersect).
    pub function: PK_boolean_function_t,
    /// Performance optimization sub-structure (NULL for default).
    pub configuration: *const PK_boolean_config_o_t,
    /// Coincidence/approximation tolerance.
    pub default_tol: c_double,
    /// Maximum entity tolerance (0.0 = not set).
    pub max_tol: c_double,
    /// Matched-region sub-structure (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Region selection for global booleans (NULL for default).
    pub selected_topolset: *const PK_boolean_topolset_o_t,
    /// Force target body type interpretation.
    pub target_material_side: PK_boolean_material_t,
    /// Force tool body type interpretation.
    pub tool_material_side: PK_boolean_material_t,
    /// Imprint completion on target.
    pub imprint_complete_targ: PK_imprint_complete_t,
    /// Imprint completion on tool.
    pub imprint_complete_tool: PK_imprint_complete_t,
    /// Handle overflow from target to tool (NULL for default).
    pub target_face_overflow: *const PK_FACE_overflow_o_t,
    /// Handle overflow from tool to target (NULL for default).
    pub tool_face_overflow: *const PK_FACE_overflow_o_t,
    /// Imprint overlapping boundaries.
    pub imprint_overlapping: PK_LOGICAL_t,
    /// Merge imprinted edges (manifold booleans).
    pub merge_imprinted: PK_LOGICAL_t,
    /// Avoid merging pre-existing edges.
    pub selective_merge: PK_LOGICAL_t,
    /// Which edge survives on coincidence.
    pub keep_target_edges: PK_LOGICAL_t,
    /// Merge face attributes.
    pub merge_attributes: PK_LOGICAL_t,
    /// Merge topologies surrounded by solid (general booleans).
    pub merge_in_solid: PK_LOGICAL_t,
    /// Highest dimension to merge in solid.
    pub merge_in_solid_dimension: PK_TOPOL_dimension_t,
    /// Merge redundant topologies in faces (general booleans).
    pub merge_in_face: PK_LOGICAL_t,
    /// Highest dimension to merge in face.
    pub merge_in_face_dimension: PK_TOPOL_dimension_t,
    /// Merge redundant vertices in edges (general booleans).
    pub merge_in_edge: PK_LOGICAL_t,
    /// Prune void regions (general booleans).
    pub prune_in_void: PK_LOGICAL_t,
    /// Repair non-manifold edges with blends.
    pub nm_edge_repair: PK_nm_edge_repair_t,
    /// Blend radius for nm repair.
    pub blend_radius: c_double,
    /// Preferred result body type.
    pub resulting_body_type: PK_boolean_prefer_t,
    /// Allow disjoint result body.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Request tracking data.
    pub tracking: PK_LOGICAL_t,
    /// Tracking detail level.
    pub tracking_type: PK_boolean_track_type_t,
    /// Track regions (general booleans).
    pub track_regions: PK_region_track_t,
    /// Detect no-effect operations.
    pub flag_no_effect: PK_boolean_no_effect_t,
    /// Check faces adjacent to imprints.
    pub check_fa: PK_boolean_check_fa_t,
    /// Fence behaviour for subtract.
    pub fence: PK_boolean_fence_t,
    /// Version compatibility.
    pub update: PK_boolean_update_t,
}

impl Default for PK_BODY_boolean_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            function: PK_boolean_unite_c,
            configuration: std::ptr::null(),
            default_tol: 0.0,
            max_tol: 0.0,
            matched_region: std::ptr::null(),
            selected_topolset: std::ptr::null(),
            target_material_side: PK_boolean_material_default_c,
            tool_material_side: PK_boolean_material_default_c,
            imprint_complete_targ: PK_imprint_complete_no_c,
            imprint_complete_tool: PK_imprint_complete_no_c,
            target_face_overflow: std::ptr::null(),
            tool_face_overflow: std::ptr::null(),
            imprint_overlapping: PK_LOGICAL_false,
            merge_imprinted: PK_LOGICAL_false,
            selective_merge: PK_LOGICAL_false,
            keep_target_edges: PK_LOGICAL_false,
            merge_attributes: PK_LOGICAL_false,
            merge_in_solid: PK_LOGICAL_false,
            merge_in_solid_dimension: PK_TOPOL_dimension_2_c,
            merge_in_face: PK_LOGICAL_false,
            merge_in_face_dimension: PK_TOPOL_dimension_1_c,
            merge_in_edge: PK_LOGICAL_false,
            prune_in_void: PK_LOGICAL_false,
            nm_edge_repair: PK_nm_edge_repair_no_c,
            blend_radius: 0.0,
            resulting_body_type: PK_boolean_prefer_original_c,
            allow_disjoint: PK_LOGICAL_false,
            tracking: PK_LOGICAL_false,
            tracking_type: PK_boolean_track_type_basic_c,
            track_regions: PK_region_track_no_c,
            flag_no_effect: PK_boolean_no_effect_basic_c,
            check_fa: PK_boolean_check_fa_yes_c,
            fence: PK_boolean_fence_none_c,
            update: PK_boolean_update_default_c,
        }
    }
}

// =============================================================================
// PK_FACE_boolean_o_t — Local boolean options
// =============================================================================

/// Options for `PK_FACE_boolean_2` (local boolean).
#[repr(C)]
pub struct PK_FACE_boolean_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Boolean operation type (unite/subtract/intersect).
    pub function: PK_boolean_function_t,
    /// Performance optimization sub-structure (NULL for default).
    pub configuration: *const PK_boolean_config_o_t,
    /// Coincidence/approximation tolerance.
    pub default_tol: c_double,
    /// Maximum entity tolerance (0.0 = not set).
    pub max_tol: c_double,
    /// Matched-region sub-structure (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Region selection for local booleans (NULL for default).
    pub select_region: *const PK_boolean_select_region_o_t,
    /// Force target body type interpretation.
    pub target_material_side: PK_boolean_material_t,
    /// Force tool body type interpretation.
    pub tool_material_side: PK_boolean_material_t,
    /// Imprint completion on target.
    pub imprint_complete_targ: PK_imprint_complete_t,
    /// Imprint completion on tool.
    pub imprint_complete_tool: PK_imprint_complete_t,
    /// Handle overflow from target to tool (NULL for default).
    pub target_face_overflow: *const PK_FACE_overflow_o_t,
    /// Handle overflow from tool to target (NULL for default).
    pub tool_face_overflow: *const PK_FACE_overflow_o_t,
    /// Add neighboring faces (local only).
    pub extend_face_list: PK_imprint_face_list_t,
    /// Limit target face deletion scope.
    pub limit_target_faces: PK_LOGICAL_t,
    /// Limit tool face deletion scope.
    pub limit_tool_faces: PK_LOGICAL_t,
    /// Merge imprinted edges.
    pub merge_imprinted: PK_LOGICAL_t,
    /// Avoid merging pre-existing edges.
    pub selective_merge: PK_LOGICAL_t,
    /// Which edge survives on coincidence.
    pub keep_target_edges: PK_LOGICAL_t,
    /// Merge face attributes.
    pub merge_attributes: PK_LOGICAL_t,
    /// Repair non-manifold edges with blends.
    pub nm_edge_repair: PK_nm_edge_repair_t,
    /// Blend radius for nm repair.
    pub blend_radius: c_double,
    /// Check tool face self-intersection.
    pub stop_self_intersection: PK_LOGICAL_t,
    /// Preferred result body type.
    pub resulting_body_type: PK_boolean_prefer_t,
    /// Allow disjoint result body.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Request tracking data.
    pub tracking: PK_LOGICAL_t,
    /// Tracking detail level.
    pub tracking_type: PK_boolean_track_type_t,
    /// Detect no-effect operations.
    pub flag_no_effect: PK_boolean_no_effect_t,
    /// Check faces adjacent to imprints.
    pub check_fa: PK_boolean_check_fa_t,
    /// Fence behaviour for subtract.
    pub fence: PK_boolean_fence_t,
    /// Version compatibility.
    pub update: PK_boolean_update_t,
}

impl Default for PK_FACE_boolean_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            function: PK_boolean_unite_c,
            configuration: std::ptr::null(),
            default_tol: 0.0,
            max_tol: 0.0,
            matched_region: std::ptr::null(),
            select_region: std::ptr::null(),
            target_material_side: PK_boolean_material_default_c,
            tool_material_side: PK_boolean_material_default_c,
            imprint_complete_targ: PK_imprint_complete_no_c,
            imprint_complete_tool: PK_imprint_complete_no_c,
            target_face_overflow: std::ptr::null(),
            tool_face_overflow: std::ptr::null(),
            extend_face_list: PK_imprint_face_list_no_c,
            limit_target_faces: PK_LOGICAL_false,
            limit_tool_faces: PK_LOGICAL_false,
            merge_imprinted: PK_LOGICAL_false,
            selective_merge: PK_LOGICAL_false,
            keep_target_edges: PK_LOGICAL_false,
            merge_attributes: PK_LOGICAL_false,
            nm_edge_repair: PK_nm_edge_repair_no_c,
            blend_radius: 0.0,
            stop_self_intersection: PK_LOGICAL_false,
            resulting_body_type: PK_boolean_prefer_original_c,
            allow_disjoint: PK_LOGICAL_false,
            tracking: PK_LOGICAL_false,
            tracking_type: PK_boolean_track_type_basic_c,
            flag_no_effect: PK_boolean_no_effect_basic_c,
            check_fa: PK_boolean_check_fa_yes_c,
            fence: PK_boolean_fence_none_c,
            update: PK_boolean_update_default_c,
        }
    }
}

// =============================================================================
// Boolean results structure
// =============================================================================

/// Results from `PK_BODY_boolean_2` or `PK_FACE_boolean_2`.
#[repr(C)]
pub struct PK_boolean_r_t {
    /// Number of resulting bodies.
    pub n_bodies: c_int,
    /// Array of resulting body tags.
    pub bodies: *mut PK_BODY_t,
    /// Boolean result status code.
    pub result: PK_boolean_result_t,
}

// =============================================================================
// Imprint option structures
// =============================================================================

/// Options for `PK_BODY_imprint_body`.
#[repr(C)]
pub struct PK_BODY_imprint_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Imprint on tool as well as target.
    pub imprint_tool: PK_LOGICAL_t,
    /// Imprint boundaries of overlapping areas.
    pub imprint_overlapping: PK_LOGICAL_t,
    /// Matched topology structure (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Complete imprints on target.
    pub imprint_complete_targ: PK_imprint_complete_t,
    /// Complete imprints on tool.
    pub imprint_complete_tool: PK_imprint_complete_t,
    /// Completion orientation on target.
    pub imprint_extend_targ: PK_imprint_extend_t,
    /// Completion orientation on tool.
    pub imprint_extend_tool: PK_imprint_extend_t,
    /// Whether tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Coincidence tolerance.
    pub tolerance: c_double,
    /// Version compatibility.
    pub update: c_int,
}

impl Default for PK_BODY_imprint_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            imprint_tool: PK_LOGICAL_false,
            imprint_overlapping: PK_LOGICAL_false,
            matched_region: std::ptr::null(),
            imprint_complete_targ: PK_imprint_complete_no_c,
            imprint_complete_tool: PK_imprint_complete_no_c,
            imprint_extend_targ: PK_imprint_extend_tangent_c,
            imprint_extend_tool: PK_imprint_extend_tangent_c,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            update: 0,
        }
    }
}

/// Options for `PK_BODY_imprint_faces_2`.
#[repr(C)]
pub struct PK_BODY_imprint_faces_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Imprint on tool as well as target.
    pub imprint_tool: PK_LOGICAL_t,
    /// Imprint boundaries of overlapping areas.
    pub imprint_overlapping: PK_LOGICAL_t,
    /// Matched topology structure (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Complete imprints on target.
    pub imprint_complete_targ: PK_imprint_complete_t,
    /// Complete imprints on tool.
    pub imprint_complete_tool: PK_imprint_complete_t,
    /// Completion orientation on target.
    pub imprint_extend_targ: PK_imprint_extend_t,
    /// Completion orientation on tool.
    pub imprint_extend_tool: PK_imprint_extend_t,
    /// Whether tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Coincidence tolerance.
    pub tolerance: c_double,
    /// Ensure consistent edge direction.
    pub imprint_dir: PK_imprint_dir_t,
    /// Version compatibility.
    pub update: c_int,
}

impl Default for PK_BODY_imprint_faces_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            imprint_tool: PK_LOGICAL_false,
            imprint_overlapping: PK_LOGICAL_false,
            matched_region: std::ptr::null(),
            imprint_complete_targ: PK_imprint_complete_no_c,
            imprint_complete_tool: PK_imprint_complete_no_c,
            imprint_extend_targ: PK_imprint_extend_tangent_c,
            imprint_extend_tool: PK_imprint_extend_tangent_c,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            imprint_dir: PK_imprint_dir_no_check_c,
            update: 0,
        }
    }
}

/// Options for `PK_FACE_imprint_faces_2`.
#[repr(C)]
pub struct PK_FACE_imprint_faces_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Imprint on tool as well as target.
    pub imprint_tool: PK_LOGICAL_t,
    /// Imprint boundaries of overlapping areas.
    pub imprint_overlapping: PK_LOGICAL_t,
    /// Add neighboring faces to prevent open loops.
    pub extend_face_list: PK_imprint_face_list_t,
    /// Matched topology structure (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Complete imprints on target.
    pub imprint_complete_targ: PK_imprint_complete_t,
    /// Complete imprints on tool.
    pub imprint_complete_tool: PK_imprint_complete_t,
    /// Completion orientation on target.
    pub imprint_extend_targ: PK_imprint_extend_t,
    /// Completion orientation on tool.
    pub imprint_extend_tool: PK_imprint_extend_t,
    /// Whether tolerance is supplied.
    pub have_tolerance: PK_LOGICAL_t,
    /// Coincidence tolerance.
    pub tolerance: c_double,
    /// Ensure consistent edge direction.
    pub imprint_dir: PK_imprint_dir_t,
    /// Version compatibility.
    pub update: c_int,
}

impl Default for PK_FACE_imprint_faces_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            imprint_tool: PK_LOGICAL_false,
            imprint_overlapping: PK_LOGICAL_false,
            extend_face_list: PK_imprint_face_list_no_c,
            matched_region: std::ptr::null(),
            imprint_complete_targ: PK_imprint_complete_no_c,
            imprint_complete_tool: PK_imprint_complete_no_c,
            imprint_extend_targ: PK_imprint_extend_tangent_c,
            imprint_extend_tool: PK_imprint_extend_tangent_c,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
            imprint_dir: PK_imprint_dir_no_check_c,
            update: 0,
        }
    }
}

// =============================================================================
// PK_CURVE_project options and results
// =============================================================================

/// Options for `PK_CURVE_project`.
#[repr(C)]
pub struct PK_CURVE_project_o_t {
    /// Version tag for this options struct.
    pub o_t_version: c_int,
    /// Project only, imprint only, or both.
    pub function: PK_proj_function_t,
    /// Normal, vector, or perspective projection.
    pub method: PK_proj_method_t,
    /// Direction for vector projection.
    pub direction: PK_VECTOR_t,
    /// Source position for perspective projection.
    pub eye_position: PK_VECTOR_t,
    /// Project in both +/- direction.
    pub bidirectional: PK_LOGICAL_t,
    /// Whether to limit projection distance.
    pub use_max_dist: PK_proj_max_dist_t,
    /// Maximum projection distance.
    pub max_dist: c_double,
    /// How to handle hidden faces.
    pub hidden: PK_proj_face_hidden_t,
    /// How to connect disjoint components.
    pub connect: PK_proj_connect_t,
    /// Number of banned output curve classes.
    pub n_banned_classes: c_int,
    /// Output curve classes to exclude.
    pub banned_classes: *const PK_CLASS_t,
    /// How to split at clashes (projecting only).
    pub split_clash: PK_proj_split_clash_t,
    /// How to handle point-like projections.
    pub create_points: PK_proj_to_points_t,
    /// Add projection as construction geometry.
    pub construction: PK_LOGICAL_t,
    /// Accuracy of resultant projected curves.
    pub tolerance: c_double,
    /// Precision of imprinted edges/vertices.
    pub imprint_precision: PK_imprint_precision_t,
    /// Attach projected curves as nominal geometry.
    pub nominal: PK_proj_nominal_t,
    /// How to complete imprinted edges.
    pub complete: PK_proj_complete_t,
    /// Whether to bound imprint completion distance.
    pub complete_bound: PK_complete_bound_t,
    /// Distance to bound imprint completion.
    pub complete_bound_distance: c_double,
    /// Version compatibility.
    pub update: PK_proj_update_t,
    /// Tracking record format.
    pub tracking: PK_proj_tracking_t,
    /// Output results via return or report.
    pub results_output: PK_results_output_t,
    /// Return originating intervals.
    pub want_orig_intervals: PK_LOGICAL_t,
}

impl Default for PK_CURVE_project_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            function: PK_proj_function_project_c,
            method: PK_proj_method_normal_c,
            direction: [0.0, 0.0, 1.0],
            eye_position: [0.0, 0.0, 0.0],
            bidirectional: PK_LOGICAL_false,
            use_max_dist: PK_proj_max_dist_no_c,
            max_dist: 0.0,
            hidden: PK_proj_face_hidden_no_c,
            connect: PK_proj_connect_none_c,
            n_banned_classes: 0,
            banned_classes: std::ptr::null(),
            split_clash: PK_proj_split_clash_no_c,
            create_points: PK_proj_to_points_no_c,
            construction: PK_LOGICAL_false,
            tolerance: 0.0,
            imprint_precision: PK_imprint_precision_auto_c,
            nominal: PK_proj_nominal_no_c,
            complete: PK_proj_complete_no_c,
            complete_bound: PK_complete_bound_none_c,
            complete_bound_distance: 0.0,
            update: PK_proj_update_default_c,
            tracking: PK_proj_tracking_basic_c,
            results_output: PK_results_output_return_c,
            want_orig_intervals: PK_LOGICAL_false,
        }
    }
}

// =============================================================================
// Sectioning option structures
// =============================================================================

/// Options for `PK_BODY_section_with_surf`.
#[repr(C)]
pub struct PK_BODY_section_with_surf_o_t {
    /// Which bodies returned: front, back, or both.
    pub fence: PK_boolean_fence_t,
    /// Merge mergeable imprinted edges.
    pub merge_imprinted: PK_LOGICAL_t,
    /// Merge all mergeable imprinted edges including adjacent to new section faces.
    pub merge_new_faces: PK_LOGICAL_t,
    /// Avoid merging pre-existing mergeable edges.
    pub selective_merge: PK_LOGICAL_t,
    /// Check faces adjacent to imprinted edges.
    pub check_fa: PK_boolean_check_fa_t,
    /// Default tolerance.
    pub default_tol: c_double,
    /// Maximum tolerance (must be > default_tol if set).
    pub max_tol: c_double,
    /// Matched regions between target/tool (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Which edge survives on coincidence.
    pub keep_target_edges: PK_LOGICAL_t,
    /// Return facet-only or mixed geometry.
    pub keep_as_facet: PK_LOGICAL_t,
}

/// Options for `PK_BODY_section_with_sheet_2`.
#[repr(C)]
pub struct PK_BODY_section_with_sheet_o_t {
    /// Which bodies returned: front, back, or both.
    pub fence: PK_boolean_fence_t,
    /// Merge mergeable imprinted edges.
    pub merge_imprinted: PK_LOGICAL_t,
    /// Merge all mergeable imprinted edges including adjacent to new section faces.
    pub merge_new_faces: PK_LOGICAL_t,
    /// Avoid merging pre-existing mergeable edges.
    pub selective_merge: PK_LOGICAL_t,
    /// Check faces adjacent to imprinted edges.
    pub check_fa: PK_boolean_check_fa_t,
    /// Default tolerance.
    pub default_tol: c_double,
    /// Maximum tolerance.
    pub max_tol: c_double,
    /// Matched regions between target/tool (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Which edge survives on coincidence.
    pub keep_target_edges: PK_LOGICAL_t,
    /// Return facet-only or mixed geometry.
    pub keep_as_facet: PK_LOGICAL_t,
}

/// Options for `PK_FACE_section_with_sheet_2` (local sectioning).
#[repr(C)]
pub struct PK_FACE_section_with_sheet_o_t {
    /// Which bodies returned: front, back, or both.
    pub fence: PK_boolean_fence_t,
    /// Merge mergeable imprinted edges.
    pub merge_imprinted: PK_LOGICAL_t,
    /// Merge all mergeable imprinted edges including adjacent to new section faces.
    pub merge_new_faces: PK_LOGICAL_t,
    /// Avoid merging pre-existing mergeable edges.
    pub selective_merge: PK_LOGICAL_t,
    /// Check faces adjacent to imprinted edges.
    pub check_fa: PK_boolean_check_fa_t,
    /// Default tolerance.
    pub default_tol: c_double,
    /// Maximum tolerance.
    pub max_tol: c_double,
    /// Use additional target/tool faces for incomplete loops.
    pub extend_face_list: PK_imprint_face_list_t,
    /// Regions to include/exclude (NULL for default).
    pub select_region: *const PK_boolean_select_region_o_t,
    /// Matched regions between target/tool (NULL for default).
    pub matched_region: *const PK_boolean_match_o_t,
    /// Which edge survives on coincidence.
    pub keep_target_edges: PK_LOGICAL_t,
    /// Return facet-only or mixed geometry.
    pub keep_as_facet: PK_LOGICAL_t,
}

/// Options for `PK_BODY_make_section` / `PK_BODY_make_section_with_surfs`.
#[repr(C)]
pub struct PK_BODY_make_section_o_t {
    /// Return tracking info.
    pub tracking: PK_LOGICAL_t,
    /// Allow disjoint bodies.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Default tolerance.
    pub default_tol: c_double,
    /// Maximum tolerance.
    pub max_tol: c_double,
    /// Sheet or wire result body type.
    pub result_body_type: PK_BODY_type_t,
    /// Version compatibility.
    pub update: PK_boolean_update_t,
}

/// Options for `PK_FACE_make_sect_with_sfs`.
#[repr(C)]
pub struct PK_FACE_make_sect_with_sfs_o_t {
    /// Return tracking info.
    pub tracking: PK_LOGICAL_t,
    /// Allow disjoint bodies.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Use additional target faces for closed loops.
    pub extend_face_list: PK_LOGICAL_t,
}

// =============================================================================
// Instancing option structures
// =============================================================================

/// Options for `PK_FACE_instance_bodies`.
#[repr(C)]
pub struct PK_FACE_instance_bodies_o_t {
    /// Boolean op: unite, subtract, intersect.
    pub function: PK_boolean_function_t,
    /// Performance hint sub-structure (NULL for default).
    pub configuration: *const PK_boolean_config_o_t,
    /// Add neighboring faces for overflow.
    pub extend_face_list: PK_imprint_face_list_t,
    /// Merge imprinted edges.
    pub merge_imprinted: PK_LOGICAL_t,
    /// Allow disjoint bodies.
    pub allow_disjoint: PK_LOGICAL_t,
    /// Check adjacent faces.
    pub check_fa: PK_boolean_check_fa_t,
    /// Check/repair face self-intersections.
    pub repair_fa_fa: PK_instance_repair_fa_fa_t,
    /// Default tolerance.
    pub default_tol: c_double,
    /// Maximum tolerance.
    pub max_tol: c_double,
    /// Imprint completion on target.
    pub imprint_complete_targ: PK_imprint_complete_t,
    /// Imprint completion on tool.
    pub imprint_complete_tool: PK_imprint_complete_t,
    /// Material side of target.
    pub target_material_side: PK_boolean_material_t,
    /// Material side of tool.
    pub tool_material_side: PK_boolean_material_t,
    /// Preferred result body type.
    pub resulting_body_type: PK_boolean_prefer_t,
    /// Limit deletable faces.
    pub limit_target_faces: PK_LOGICAL_t,
    /// Tracking type.
    pub tracking_type: PK_instance_track_type_t,
    /// Edge tracking detail.
    pub track_edges: PK_instance_track_edges_t,
    /// Version compatibility.
    pub update: PK_boolean_update_t,
}

// =============================================================================
// Patterning option structure
// =============================================================================

/// Options for `PK_FACE_pattern`.
#[repr(C)]
pub struct PK_FACE_pattern_o_t {
    /// Ensure boundary loops contained within a face.
    pub check_loops: PK_pattern_check_loops_t,
    /// Ensure instances don't intersect other faces.
    pub check_fa_fa: PK_LOGICAL_t,
    /// Performance hint: instances in same faces as feature.
    pub same_face: PK_pattern_same_face_t,
    /// Assert boundary loops coincident with destination faces.
    pub coi_face: PK_pattern_coi_face_t,
    /// Recreate blend faces in new instances.
    pub reblend: PK_pattern_reblend_t,
    /// Number of face maps.
    pub n_face_maps: c_int,
    /// Destination faces for each new instance.
    pub face_maps: *const PK_FACE_t,
    /// Report collision errors.
    pub collision: PK_pattern_collision_t,
}

// =============================================================================
// Legacy imprint options
// =============================================================================

/// Options for `PK_BODY_imprint_cus_vec` / `PK_FACE_imprint_cus_vec`.
#[repr(C)]
pub struct PK_imprint_cus_vec_o_t {
    /// Connect disjoint components.
    pub connect: PK_imprint_connect_t,
    /// Imprint in both directions.
    pub bidirectional: PK_LOGICAL_t,
    /// Handle curves intersecting in view direction.
    pub process_intersections: PK_imprint_intersect_t,
    /// Extend imprinted edge to existing edge.
    pub imprint_complete: PK_imprint_complete_t,
    /// Imprint coincident curves exactly.
    pub imprint_coi_exactly: PK_LOGICAL_t,
    /// Force all imprints exact.
    pub imprint_exactly: PK_LOGICAL_t,
    /// Tracking detail level.
    pub tracking_type: PK_imprint_tracking_t,
}

/// Options for `PK_BODY_imprint_cus_normal` / `PK_FACE_imprint_cus_normal`.
#[repr(C)]
pub struct PK_imprint_cus_normal_o_t {
    /// Handle curves intersecting in view direction.
    pub process_intersections: PK_imprint_intersect_t,
    /// Restrict imprints to nearby faces.
    pub use_max_projection_dist: PK_imprint_proj_dist_t,
    /// Max projection distance.
    pub max_projection_dist: c_double,
    /// Extend imprinted edge to existing edge.
    pub imprint_complete: PK_imprint_complete_t,
    /// Imprint coincident curves exactly.
    pub imprint_coi_exactly: PK_LOGICAL_t,
    /// Continuity of projected curves.
    pub preferred_continuity: PK_continuity_t,
    /// Tracking detail level.
    pub tracking_type: PK_imprint_tracking_t,
}

/// Options for `PK_FACE_imprint_cus_normal` (has hidden face option).
#[repr(C)]
pub struct PK_FACE_imprint_cus_normal_o_t {
    /// Handle curves intersecting in view direction.
    pub process_intersections: PK_imprint_intersect_t,
    /// Restrict imprints to nearby faces.
    pub use_max_projection_dist: PK_imprint_proj_dist_t,
    /// Max projection distance.
    pub max_projection_dist: c_double,
    /// Extend imprinted edge to existing edge.
    pub imprint_complete: PK_imprint_complete_t,
    /// Imprint coincident curves exactly.
    pub imprint_coi_exactly: PK_LOGICAL_t,
    /// Continuity of projected curves.
    pub preferred_continuity: PK_continuity_t,
    /// Handle obscuring faces.
    pub hidden: PK_FACE_imprint_hidden_t,
    /// Tracking detail level.
    pub tracking_type: PK_imprint_tracking_t,
}

// =============================================================================
// Opaque options/result types for obsolete V1 boolean/section/imprint/pattern
// =============================================================================

/// Options for `PK_BODY_section_with_sheet`.
#[repr(C)]
pub struct PK_BODY_section_o_t { _private: [u8; 0] }

/// Results from section operations.
#[repr(C)]
pub struct PK_section_r_t { _private: [u8; 0] }

/// Options for `PK_FACE_section_with_sheet`.
#[repr(C)]
pub struct PK_FACE_section_o_t { _private: [u8; 0] }

/// Options for `PK_FACE_imprint_faces`.
#[repr(C)]
pub struct PK_FACE_imprint_o_t { _private: [u8; 0] }

/// Results from face imprint operations.
#[repr(C)]
pub struct PK_imprint_r_t { _private: [u8; 0] }

/// Results from `PK_FACE_pattern`.
#[repr(C)]
pub struct PK_FACE_pattern_r_t { _private: [u8; 0] }

/// Options for `PK_FACE_pattern_2`.
#[repr(C)]
pub struct PK_FACE_pattern_2_o_t { _private: [u8; 0] }

/// Results from `PK_FACE_pattern_2`.
#[repr(C)]
pub struct PK_FACE_pattern_2_r_t { _private: [u8; 0] }

/// Options for `PK_FACE_imprint_cus_vector`.
#[repr(C)]
pub struct PK_FACE_imprint_cus_vector_o_t { _private: [u8; 0] }

// =============================================================================
// Extern "C" function declarations
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    // =========================================================================
    // Session — general topology toggle
    // =========================================================================

    // =========================================================================
    // Body type query
    // =========================================================================

    // =========================================================================
    // Vertex precision
    // =========================================================================

    // =========================================================================
    // Global boolean
    // =========================================================================

    /// Global boolean operation. One target body, one or more tool bodies.
    pub fn PK_BODY_boolean_2(
        target: PK_BODY_t,
        n_tools: c_int,
        tools: *const PK_BODY_t,
        options: *const PK_BODY_boolean_o_t,
        results: *mut PK_boolean_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Local boolean
    // =========================================================================

    /// Local boolean operation. Faces from a single target and tool body.
    /// Does NOT support general bodies.
    pub fn PK_FACE_boolean_2(
        n_target_faces: c_int,
        target_faces: *const PK_FACE_t,
        n_tool_faces: c_int,
        tool_faces: *const PK_FACE_t,
        options: *const PK_FACE_boolean_o_t,
        results: *mut PK_boolean_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Specialised manifold booleans
    // =========================================================================

    /// Specialised unite. Target: sheet or solid. Tools: sheet or solid (array).
    pub fn PK_BODY_unite_bodies(
        target: PK_BODY_t,
        n_tools: c_int,
        tools: *const PK_BODY_t,
        options: *const PK_BODY_boolean_o_t,
        results: *mut PK_boolean_r_t,
    ) -> PK_ERROR_code_t;

    /// Specialised subtract. Target: wire, sheet, or solid. Tools: sheet or solid (array).
    pub fn PK_BODY_subtract_bodies(
        target: PK_BODY_t,
        n_tools: c_int,
        tools: *const PK_BODY_t,
        options: *const PK_BODY_boolean_o_t,
        results: *mut PK_boolean_r_t,
    ) -> PK_ERROR_code_t;

    /// Specialised intersect. Target: wire, sheet, or solid. Tools: sheet or solid (array).
    pub fn PK_BODY_intersect_bodies(
        target: PK_BODY_t,
        n_tools: c_int,
        tools: *const PK_BODY_t,
        options: *const PK_BODY_boolean_o_t,
        results: *mut PK_boolean_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Imprinting — body-level
    // =========================================================================

    /// Imprint edges/vertices on target and tool where two bodies intersect.
    pub fn PK_BODY_imprint_body(
        target: PK_BODY_t,
        tool: PK_BODY_t,
        options: *const PK_BODY_imprint_o_t,
        n_target_edges: *mut c_int,
        target_edges: *mut *mut PK_EDGE_t,
        n_target_vertices: *mut c_int,
        target_vertices: *mut *mut PK_VERTEX_t,
        n_tool_edges: *mut c_int,
        tool_edges: *mut *mut PK_EDGE_t,
        n_tool_vertices: *mut c_int,
        tool_vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Imprint edges/vertices on target body where specified faces intersect.
    pub fn PK_BODY_imprint_faces_2(
        target: PK_BODY_t,
        n_tool_faces: c_int,
        tool_faces: *const PK_FACE_t,
        options: *const PK_BODY_imprint_faces_o_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Imprint edges/vertices on specified face sets.
    pub fn PK_FACE_imprint_faces_2(
        n_target_faces: c_int,
        target_faces: *const PK_FACE_t,
        n_tool_faces: c_int,
        tool_faces: *const PK_FACE_t,
        options: *const PK_FACE_imprint_faces_o_t,
        n_target_edges: *mut c_int,
        target_edges: *mut *mut PK_EDGE_t,
        n_target_vertices: *mut c_int,
        target_vertices: *mut *mut PK_VERTEX_t,
        n_tool_edges: *mut c_int,
        tool_edges: *mut *mut PK_EDGE_t,
        n_tool_vertices: *mut c_int,
        tool_vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Imprint curves directly on a face (curves must be coincident with face surface).
    pub fn PK_FACE_imprint_curves_2(
        face: PK_FACE_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        n_new_edges: *mut c_int,
        new_edges: *mut *mut PK_EDGE_t,
        n_new_vertices: *mut c_int,
        new_vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Imprint a plane onto a body.
    pub fn PK_BODY_imprint_plane_2(
        body: PK_BODY_t,
        plane: PK_PLANE_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Add a vertex to an edge at specified coordinates, splitting the edge.
    pub fn PK_EDGE_imprint_point(
        edge: PK_EDGE_t,
        position: *const PK_VECTOR_t,
        vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Add an isolated vertex to a face, creating a new loop.
    pub fn PK_FACE_imprint_point(
        face: PK_FACE_t,
        position: *const PK_VECTOR_t,
        vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Add an acorn vertex to a region of a general body.
    pub fn PK_REGION_imprint_point(
        region: PK_REGION_t,
        position: *const PK_VECTOR_t,
        vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Given loops of coincident edges, calculates which face sets survive.
    pub fn PK_BODY_identify_facesets(
        target: PK_BODY_t,
        tool: PK_BODY_t,
        function: PK_boolean_function_t,
        n_edge_pairs: c_int,
        target_edges: *const PK_EDGE_t,
        tool_edges: *const PK_EDGE_t,
        n_target_survive: *mut c_int,
        target_survive: *mut *mut PK_FACE_t,
        n_tool_survive: *mut c_int,
        tool_survive: *mut *mut PK_FACE_t,
        n_target_delete: *mut c_int,
        target_delete: *mut *mut PK_FACE_t,
        n_tool_delete: *mut c_int,
        tool_delete: *mut *mut PK_FACE_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Imprinting — legacy (vec / normal)
    // =========================================================================

    /// Legacy: imprint curves onto a body by projecting in a specified direction.
    pub fn PK_BODY_imprint_cus_vec(
        body: PK_BODY_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        direction: *const PK_VECTOR_t,
        options: *const PK_imprint_cus_vec_o_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Legacy: imprint curves onto specified faces by projecting in a specified direction.
    pub fn PK_FACE_imprint_cus_vec(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        direction: *const PK_VECTOR_t,
        options: *const PK_imprint_cus_vec_o_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Legacy: imprint curves onto a body by projecting down the face normal.
    pub fn PK_BODY_imprint_cus_normal(
        body: PK_BODY_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        options: *const PK_imprint_cus_normal_o_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Legacy: imprint curves onto specified faces by projecting down the face normal.
    pub fn PK_FACE_imprint_cus_normal(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        options: *const PK_FACE_imprint_cus_normal_o_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
        n_vertices: *mut c_int,
        vertices: *mut *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Curve projection (modern replacement for legacy imprint)
    // =========================================================================

    /// General-purpose curve projection/imprinting onto bodies, surfaces, or face sets.
    pub fn PK_CURVE_project(
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        n_targets: c_int,
        targets: *const PK_ENTITY_t,
        options: *const PK_CURVE_project_o_t,
        n_results: *mut c_int,
        results: *mut *mut PK_ENTITY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Global sectioning
    // =========================================================================

    /// Section a body with a planar or cylindrical surface.
    pub fn PK_BODY_section_with_surf(
        body: PK_BODY_t,
        surf: PK_SURF_t,
        options: *const PK_BODY_section_with_surf_o_t,
        n_front_bodies: *mut c_int,
        front_bodies: *mut *mut PK_BODY_t,
        n_back_bodies: *mut c_int,
        back_bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Section a body with a sheet tool (may be disjoint). Tool is deleted.
    pub fn PK_BODY_section_with_sheet_2(
        body: PK_BODY_t,
        tool: PK_BODY_t,
        options: *const PK_BODY_section_with_sheet_o_t,
        n_front_bodies: *mut c_int,
        front_bodies: *mut *mut PK_BODY_t,
        n_back_bodies: *mut c_int,
        back_bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Local sectioning
    // =========================================================================

    /// Section particular faces of a target body with faces of a sheet body.
    pub fn PK_FACE_section_with_sheet_2(
        n_target_faces: c_int,
        target_faces: *const PK_FACE_t,
        n_tool_faces: c_int,
        tool_faces: *const PK_FACE_t,
        options: *const PK_FACE_section_with_sheet_o_t,
        n_front_bodies: *mut c_int,
        front_bodies: *mut *mut PK_BODY_t,
        n_back_bodies: *mut c_int,
        back_bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Non-destructive sectioning
    // =========================================================================

    /// Section target bodies using sheet bodies. Targets left unchanged.
    pub fn PK_BODY_make_section(
        n_targets: c_int,
        targets: *const PK_BODY_t,
        n_tools: c_int,
        tools: *const PK_BODY_t,
        options: *const PK_BODY_make_section_o_t,
        n_results: *mut c_int,
        results: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Section target bodies using surfaces. Targets left unchanged.
    pub fn PK_BODY_make_section_with_surfs(
        n_targets: c_int,
        targets: *const PK_BODY_t,
        n_surfs: c_int,
        surfs: *const PK_SURF_t,
        options: *const PK_BODY_make_section_o_t,
        n_results: *mut c_int,
        results: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Section target faces using surfaces. Returns wire bodies only.
    pub fn PK_FACE_make_sect_with_sfs(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_surfs: c_int,
        surfs: *const PK_SURF_t,
        options: *const PK_FACE_make_sect_with_sfs_o_t,
        n_results: *mut c_int,
        results: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Instancing
    // =========================================================================

    /// Create instances of a single tool on a single target body.
    /// Target and tool must be solid. Does NOT support general bodies.
    pub fn PK_FACE_instance_tools(
        n_target_faces: c_int,
        target_faces: *const PK_FACE_t,
        n_tool_faces: c_int,
        tool_faces: *const PK_FACE_t,
        n_transforms: c_int,
        transforms: *const PK_TRANSF_t,
        options: *const PK_FACE_boolean_o_t,
        results: *mut PK_boolean_r_t,
    ) -> PK_ERROR_code_t;

    /// Create instances of multiple tools on a single target body.
    /// Does NOT support general bodies.
    pub fn PK_FACE_instance_bodies(
        n_target_faces: c_int,
        target_faces: *const PK_FACE_t,
        n_tools: c_int,
        tools: *const PK_BODY_t,
        n_transforms: c_int,
        transforms: *const PK_TRANSF_t,
        options: *const PK_FACE_instance_bodies_o_t,
        results: *mut PK_boolean_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Patterning
    // =========================================================================

    /// Patterning operation: copy feature faces using transforms.
    /// Does NOT support facet geometry.
    pub fn PK_FACE_pattern(
        n_faces: c_int,
        faces: *const PK_FACE_t,
        n_transforms: c_int,
        transforms: *const PK_TRANSF_t,
        options: *const PK_FACE_pattern_o_t,
        status: *mut PK_pattern_status_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Obsolete V1 boolean/section/imprint/pattern functions
    // =========================================================================

    /// Boolean operation (obsolete, superseded by _2).
    pub fn PK_BODY_boolean(
        target: PK_BODY_t,
        n_tools: c_int,
        tools: *const PK_BODY_t,
        options: *const PK_BODY_boolean_o_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Boolean on face subsets (obsolete, superseded by _2).
    pub fn PK_FACE_boolean(
        n_targets: c_int,
        targets: *const PK_FACE_t,
        n_tools: c_int,
        tools: *const PK_FACE_t,
        options: *const PK_FACE_boolean_o_t,
        n_bodies: *mut c_int,
        bodies: *mut *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Section body with sheet (obsolete, superseded by _2).
    pub fn PK_BODY_section_with_sheet(
        target: PK_BODY_t,
        sheet: PK_BODY_t,
        options: *const PK_BODY_section_o_t,
        results: *mut PK_section_r_t,
    ) -> PK_ERROR_code_t;

    /// Section faces with sheet (obsolete, superseded by _2).
    pub fn PK_FACE_section_with_sheet(
        n_targets: c_int,
        targets: *const PK_FACE_t,
        n_tools: c_int,
        tools: *const PK_FACE_t,
        options: *const PK_FACE_section_o_t,
        results: *mut PK_section_r_t,
    ) -> PK_ERROR_code_t;

    /// Imprint edges on target and tool faces.
    pub fn PK_FACE_imprint_faces(
        n_targets: c_int,
        targets: *const PK_FACE_t,
        n_tools: c_int,
        tools: *const PK_FACE_t,
        options: *const PK_FACE_imprint_o_t,
        results: *mut PK_imprint_r_t,
    ) -> PK_ERROR_code_t;

    /// Pattern faces by transforms (V2).
    pub fn PK_FACE_pattern_2(
        n_pattern_faces: c_int,
        pattern_faces: *const PK_FACE_t,
        n_transforms: c_int,
        transforms: *const PK_TRANSF_t,
        options: *const PK_FACE_pattern_2_o_t,
        results: *mut PK_FACE_pattern_2_r_t,
    ) -> PK_ERROR_code_t;

    /// Imprint curves on faces by vector projection.
    pub fn PK_FACE_imprint_cus_vector(
        n_targets: c_int,
        targets: *const PK_FACE_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        direction: PK_VECTOR_t,
        tol: c_double,
        options: *const PK_FACE_imprint_cus_vector_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_FACE_imprint_faces`.
    pub fn PK_FACE_imprint_faces_r_f(results: *mut PK_imprint_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_FACE_pattern`.
    pub fn PK_FACE_pattern_r_f(results: *mut PK_FACE_pattern_r_t) -> PK_ERROR_code_t;

    /// Free results from `PK_FACE_pattern_2`.
    pub fn PK_FACE_pattern_2_r_f(results: *mut PK_FACE_pattern_2_r_t) -> PK_ERROR_code_t;

    /// Free section results from `PK_BODY_section_with_sheet`.
    pub fn PK_BODY_section_with_sheet_r_f(results: *mut PK_section_r_t) -> PK_ERROR_code_t;

    /// Free section results from `PK_FACE_section_with_sheet`.
    pub fn PK_FACE_section_with_sheet_r_f(results: *mut PK_section_r_t) -> PK_ERROR_code_t;

    /// Free tracking results from `PK_FACE_imprint_cus_vector`.
    pub fn PK_FACE_imprint_cus_vector_r_f(results: *mut PK_TOPOL_track_r_t) -> PK_ERROR_code_t;

}
