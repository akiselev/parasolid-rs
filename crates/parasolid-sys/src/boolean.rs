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

// [static-observed] The function tokens are NOT 0/1/2. `PK_BODY_boolean_2`
// dispatches on `function` with `function - 0x3e1e` and special-cases 0x3e1f,
// so the enum base is 0x3e1e (15902). The unite/subtract/intersect *mapping*
// over 0x3e1e..0x3e20 is not yet confirmed dynamically (the boolean needs its
// nested option sub-structs built first — see the `boolean()` wrapper note).
pub type PK_boolean_function_t = c_int;
pub const PK_boolean_unite_c: PK_boolean_function_t = 15903; // 15902 [static, order unconfirmed]
pub const PK_boolean_subtract_c: PK_boolean_function_t = 15902; // 15903
pub const PK_boolean_intersect_c: PK_boolean_function_t = 15901; // 15904

// =============================================================================
// Boolean material side enum
// =============================================================================

pub type PK_boolean_material_t = c_int;
pub const PK_boolean_material_default_c: PK_boolean_material_t = 22832;
pub const PK_boolean_material_inside_c: PK_boolean_material_t = 22831;
pub const PK_boolean_material_outside_c: PK_boolean_material_t = 22833;
pub const PK_boolean_material_none_c: PK_boolean_material_t = 22830;

// =============================================================================
// Boolean match style enum
// =============================================================================

pub type PK_boolean_match_style_t = c_int;
pub const PK_boolean_match_style_basic_c: PK_boolean_match_style_t = 21990;
pub const PK_boolean_match_style_auto_c: PK_boolean_match_style_t = 21992;
pub const PK_boolean_match_style_relax_c: PK_boolean_match_style_t = 21991; // DEPRECATED

// =============================================================================
// Boolean match type enum
// =============================================================================

pub type PK_boolean_match_type_t = c_int;
pub const PK_boolean_match_exact_c: PK_boolean_match_type_t = 18250;
pub const PK_boolean_match_contains_c: PK_boolean_match_type_t = 18251;
pub const PK_boolean_match_overlap_c: PK_boolean_match_type_t = 18252;
pub const PK_boolean_match_imprinted_c: PK_boolean_match_type_t = 18253;

// =============================================================================
// Face overflow enum (laminar)
// =============================================================================

pub type PK_FACE_overflow_t = c_int;
pub const PK_FACE_overflow_tangent_c: PK_FACE_overflow_t = 23740;
pub const PK_FACE_overflow_ruled_c: PK_FACE_overflow_t = 23741;
pub const PK_FACE_overflow_swept_c: PK_FACE_overflow_t = 23742;

// Face overflow enum (interior)
pub const PK_FACE_overflow_none_c: PK_FACE_overflow_t = 23745;
pub const PK_FACE_overflow_added_c: PK_FACE_overflow_t = 23743;
pub const PK_FACE_overflow_mixed_c: PK_FACE_overflow_t = 23744;

// =============================================================================
// Selector type enum
// =============================================================================

pub type PK_selector_type_t = c_int;
pub const PK_selector_type_off_c: PK_selector_type_t = 25212;
pub const PK_selector_type_exclude_c: PK_selector_type_t = 25210;
pub const PK_selector_type_include_c: PK_selector_type_t = 25211;

// =============================================================================
// Selector split action enum
// =============================================================================

pub type PK_selector_split_t = c_int;
pub const PK_selector_split_fail_c: PK_selector_split_t = 25130;
pub const PK_selector_split_propagate_c: PK_selector_split_t = 25131;

// =============================================================================
// Boolean select enum (local)
// =============================================================================

pub type PK_boolean_select_t = c_int;
pub const PK_boolean_include_c: PK_boolean_select_t = 15906;
pub const PK_boolean_exclude_c: PK_boolean_select_t = 15905;
pub const PK_boolean_mixed_selection_c: PK_boolean_select_t = 15948;
// [re-abi] appended 3 missing member(s) from pk-enums.h
pub const PK_boolean_select_specific_c: PK_boolean_select_t = 21000;
pub const PK_boolean_select_adjacent_c: PK_boolean_select_t = 21001;
pub const PK_boolean_select_propagate_c: PK_boolean_select_t = 21002;

// =============================================================================
// Resulting body type preference enum
// =============================================================================

pub type PK_boolean_prefer_t = c_int;
pub const PK_boolean_prefer_original_c: PK_boolean_prefer_t = 22922;
pub const PK_boolean_prefer_solid_c: PK_boolean_prefer_t = 22920;
pub const PK_boolean_prefer_sheet_c: PK_boolean_prefer_t = 22921;
pub const PK_boolean_prefer_wire_c: PK_boolean_prefer_t = 22923;
pub const PK_boolean_prefer_general_c: PK_boolean_prefer_t = 22924;
pub const PK_boolean_prefer_simplest_c: PK_boolean_prefer_t = 22925;

// =============================================================================
// Non-manifold edge repair enum
// =============================================================================

pub type PK_nm_edge_repair_t = c_int;
pub const PK_nm_edge_repair_no_c: PK_nm_edge_repair_t = 23470;
pub const PK_nm_edge_repair_blend_c: PK_nm_edge_repair_t = 23471;

// =============================================================================
// Boolean tracking type enum
// =============================================================================

pub type PK_boolean_track_type_t = c_int;
pub const PK_boolean_track_type_basic_c: PK_boolean_track_type_t = 23540;
pub const PK_boolean_track_type_comp_c: PK_boolean_track_type_t = 23541;

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
pub const PK_boolean_fence_none_c: PK_boolean_fence_t = 18212;
pub const PK_boolean_fence_front_c: PK_boolean_fence_t = 18210;
pub const PK_boolean_fence_back_c: PK_boolean_fence_t = 18211;

// =============================================================================
// Boolean no-effect detection enum
// =============================================================================

pub type PK_boolean_no_effect_t = c_int;
pub const PK_boolean_no_effect_basic_c: PK_boolean_no_effect_t = 23160;
pub const PK_boolean_no_effect_advanced_c: PK_boolean_no_effect_t = 23161;

// =============================================================================
// Boolean face check enum
// =============================================================================

pub type PK_boolean_check_fa_t = c_int;
pub const PK_boolean_check_fa_yes_c: PK_boolean_check_fa_t = 21801;
pub const PK_boolean_check_fa_no_c: PK_boolean_check_fa_t = 21800;

// =============================================================================
// Boolean update (version compat) enum
// =============================================================================

pub type PK_boolean_update_t = c_int;
pub const PK_boolean_update_default_c: PK_boolean_update_t = 24941;
pub const PK_boolean_update_0_c: PK_boolean_update_t = 24940;
pub const PK_boolean_update_5_c: PK_boolean_update_t = 24946;
pub const PK_boolean_update_v261_c: PK_boolean_update_t = 24947;
// [re-abi] appended 7 missing member(s) from pk-enums.h
pub const PK_boolean_update_1_c: PK_boolean_update_t = 24942;
pub const PK_boolean_update_2_c: PK_boolean_update_t = 24943;
pub const PK_boolean_update_3_c: PK_boolean_update_t = 24944;
pub const PK_boolean_update_4_c: PK_boolean_update_t = 24945;
pub const PK_boolean_update_v270_c: PK_boolean_update_t = 24948;
pub const PK_boolean_update_v271_c: PK_boolean_update_t = 24949;
pub const PK_boolean_update_v280_c: PK_boolean_update_t = 25750;

// =============================================================================
// Topology dimension enum (for merge options)
// =============================================================================

pub const PK_TOPOL_dimension_1_c: PK_TOPOL_dimension_t = 24341;
pub const PK_TOPOL_dimension_2_c: PK_TOPOL_dimension_t = 24342;

// =============================================================================
// Topology track record enum
// =============================================================================

pub type PK_TOPOL_track_record_t = c_int;
pub const PK_TOPOL_track_derive_c: PK_TOPOL_track_record_t = 21504;
pub const PK_TOPOL_track_create_c: PK_TOPOL_track_record_t = 21502;

// =============================================================================
// Boolean result status enum
// =============================================================================

pub type PK_boolean_result_t = c_int;
pub const PK_boolean_result_success_c: PK_boolean_result_t = 21650;
pub const PK_boolean_result_no_clash_c: PK_boolean_result_t = 21651;
pub const PK_boolean_result_no_effect_c: PK_boolean_result_t = 21652;
pub const PK_boolean_result_imprint_c: PK_boolean_result_t = 21656;
pub const PK_boolean_result_not_solid_c: PK_boolean_result_t = 21654;
pub const PK_boolean_result_multiple_c: PK_boolean_result_t = 21657; // DEPRECATED
pub const PK_boolean_result_failed_c: PK_boolean_result_t = 21653;

// =============================================================================
// Body type enum
// =============================================================================

// =============================================================================
// Imprint-related enums
// =============================================================================

pub type PK_imprint_complete_t = c_int;
pub const PK_imprint_complete_no_c: PK_imprint_complete_t = 22780;
pub const PK_imprint_complete_edge_c: PK_imprint_complete_t = 22781;
pub const PK_imprint_complete_laminar_c: PK_imprint_complete_t = 22782;
pub const PK_imprint_complete_faceset_c: PK_imprint_complete_t = 22783;

pub type PK_imprint_extend_t = c_int;
pub const PK_imprint_extend_tangent_c: PK_imprint_extend_t = 22790;
// [re-abi] appended 2 missing member(s) from pk-enums.h
pub const PK_imprint_extend_orth_fwd_c: PK_imprint_extend_t = 22791;
pub const PK_imprint_extend_orth_back_c: PK_imprint_extend_t = 22792;

pub type PK_imprint_face_list_t = c_int;
pub const PK_imprint_face_list_no_c: PK_imprint_face_list_t = 0;
pub const PK_imprint_face_list_target_c: PK_imprint_face_list_t = 1;
pub const PK_imprint_face_list_tool_c: PK_imprint_face_list_t = 2;
pub const PK_imprint_face_list_both_c: PK_imprint_face_list_t = 3;

pub type PK_imprint_dir_t = c_int;
pub const PK_imprint_dir_no_check_c: PK_imprint_dir_t = 24831;
pub const PK_imprint_dir_consistent_c: PK_imprint_dir_t = 24830;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_REPORT_1_imp_dir_undef_c: PK_imprint_dir_t = 23903;

pub type PK_imprint_precision_t = c_int;
pub const PK_imprint_precision_auto_c: PK_imprint_precision_t = 25380;
pub const PK_imprint_precision_accurate_c: PK_imprint_precision_t = 25381;

pub type PK_imprint_connect_t = c_int;
pub const PK_imprint_connect_none_c: PK_imprint_connect_t = 22810;
pub const PK_imprint_connect_side_c: PK_imprint_connect_t = 22811;
pub const PK_imprint_connect_side_all_c: PK_imprint_connect_t = 22813;
pub const PK_imprint_connect_all_c: PK_imprint_connect_t = 22812;
pub const PK_imprint_connect_hidden_all_c: PK_imprint_connect_t = 22814;

pub type PK_imprint_intersect_t = c_int;
pub const PK_imprint_intersect_fix_c: PK_imprint_intersect_t = 24582;
pub const PK_imprint_intersect_fail_c: PK_imprint_intersect_t = 24581;
pub const PK_imprint_intersect_update_c: PK_imprint_intersect_t = 24580;

pub type PK_imprint_proj_dist_t = c_int;
pub const PK_imprint_proj_dist_no_c: PK_imprint_proj_dist_t = 23430;
pub const PK_imprint_proj_dist_whole_c: PK_imprint_proj_dist_t = 23431;

pub type PK_imprint_tracking_t = c_int;
pub const PK_imprint_tracking_basic_c: PK_imprint_tracking_t = 23330;
pub const PK_imprint_tracking_curves_c: PK_imprint_tracking_t = 23331;

pub type PK_FACE_imprint_hidden_t = c_int;
pub const PK_FACE_imprint_hidden_no_c: PK_FACE_imprint_hidden_t = 0;
pub const PK_FACE_imprint_hidden_body_c: PK_FACE_imprint_hidden_t = 1;
pub const PK_FACE_imprint_hidden_array_c: PK_FACE_imprint_hidden_t = 2;

// =============================================================================
// Projection enums (PK_CURVE_project)
// =============================================================================

pub type PK_proj_function_t = c_int;
pub const PK_proj_function_project_c: PK_proj_function_t = 25360;
pub const PK_proj_function_imprint_c: PK_proj_function_t = 25361;
pub const PK_proj_function_both_c: PK_proj_function_t = 25362;

pub type PK_proj_method_t = c_int;
pub const PK_proj_method_unset_c: PK_proj_method_t = 26523;
pub const PK_proj_method_normal_c: PK_proj_method_t = 26520;
pub const PK_proj_method_vector_c: PK_proj_method_t = 26521;
pub const PK_proj_method_perspective_c: PK_proj_method_t = 26522;

pub type PK_proj_max_dist_t = c_int;
pub const PK_proj_max_dist_no_c: PK_proj_max_dist_t = 25160;
pub const PK_proj_max_dist_whole_c: PK_proj_max_dist_t = 25161;

pub type PK_proj_face_hidden_t = c_int;
pub const PK_proj_face_hidden_no_c: PK_proj_face_hidden_t = 25150;
pub const PK_proj_face_hidden_array_c: PK_proj_face_hidden_t = 25151;
pub const PK_proj_face_hidden_body_c: PK_proj_face_hidden_t = 25152;

pub type PK_proj_connect_t = c_int;
pub const PK_proj_connect_none_c: PK_proj_connect_t = 25140;
pub const PK_proj_connect_side_c: PK_proj_connect_t = 25141;
pub const PK_proj_connect_all_c: PK_proj_connect_t = 25142;
pub const PK_proj_connect_side_all_c: PK_proj_connect_t = 25143;
pub const PK_proj_connect_hidden_all_c: PK_proj_connect_t = 25144;

pub type PK_proj_split_clash_t = c_int;
pub const PK_proj_split_clash_no_c: PK_proj_split_clash_t = 0;
pub const PK_proj_split_clash_self_c: PK_proj_split_clash_t = 1;
pub const PK_proj_split_clash_all_c: PK_proj_split_clash_t = 2;

pub type PK_proj_to_points_t = c_int;
pub const PK_proj_to_points_no_c: PK_proj_to_points_t = 25180;
pub const PK_proj_to_points_end_on_c: PK_proj_to_points_t = 25182;
pub const PK_proj_to_points_tol_c: PK_proj_to_points_t = 25181;

pub type PK_proj_nominal_t = c_int;
pub const PK_proj_nominal_no_c: PK_proj_nominal_t = 25351;
pub const PK_proj_nominal_yes_c: PK_proj_nominal_t = 25350;

pub type PK_proj_complete_t = c_int;
pub const PK_proj_complete_no_c: PK_proj_complete_t = 25340;
pub const PK_proj_complete_edge_c: PK_proj_complete_t = 25341;
pub const PK_proj_complete_faceset_c: PK_proj_complete_t = 25343;
// [re-abi] appended 1 missing member(s) from pk-enums.h
pub const PK_proj_complete_laminar_c: PK_proj_complete_t = 25342;

pub type PK_complete_bound_t = c_int;
pub const PK_complete_bound_none_c: PK_complete_bound_t = 26210;
pub const PK_complete_bound_if_within_c: PK_complete_bound_t = 26211;

pub type PK_proj_tracking_t = c_int;
pub const PK_proj_tracking_basic_c: PK_proj_tracking_t = 25520;
pub const PK_proj_tracking_completion_c: PK_proj_tracking_t = 25521;

pub type PK_results_output_t = c_int;
pub const PK_results_output_return_c: PK_results_output_t = 25000;
pub const PK_results_output_report_c: PK_results_output_t = 25001;

pub const PK_proj_update_default_c: PK_proj_update_t = 25370;
// [re-abi] appended 7 missing member(s) from pk-enums.h
pub const PK_proj_update_0_c: PK_proj_update_t = 25371;
pub const PK_proj_update_1_c: PK_proj_update_t = 25372;
pub const PK_proj_update_2_c: PK_proj_update_t = 25373;
pub const PK_proj_update_v261_c: PK_proj_update_t = 25374;
pub const PK_proj_update_v270_c: PK_proj_update_t = 25375;
pub const PK_proj_update_v271_c: PK_proj_update_t = 25376;
pub const PK_proj_update_v280_c: PK_proj_update_t = 25377;

pub const PK_continuity_c2_c: PK_continuity_t = 23151;

// =============================================================================
// Instancing/patterning enums
// =============================================================================

pub type PK_instance_repair_fa_fa_t = c_int;
pub const PK_instance_repair_fa_fa_yes_c: PK_instance_repair_fa_fa_t = 24461;
pub const PK_instance_repair_fa_fa_no_c: PK_instance_repair_fa_fa_t = 24460;

pub type PK_instance_merge_t = c_int;
pub const PK_instance_merge_new_c: PK_instance_merge_t = 24481;
pub const PK_instance_merge_no_c: PK_instance_merge_t = 24480;

pub type PK_instance_track_type_t = c_int;
pub const PK_instance_track_type_none_c: PK_instance_track_type_t = 24470;
pub const PK_instance_track_type_inst_c: PK_instance_track_type_t = 24471;
pub const PK_instance_track_type_topol_c: PK_instance_track_type_t = 24472;
pub const PK_instance_track_type_both_c: PK_instance_track_type_t = 24473;

pub type PK_instance_track_edges_t = c_int;
pub const PK_instance_track_edges_no_c: PK_instance_track_edges_t = 24520;
pub const PK_instance_track_edges_laminar_c: PK_instance_track_edges_t = 24521;
pub const PK_instance_track_edges_new_c: PK_instance_track_edges_t = 24522;

pub type PK_pattern_check_loops_t = c_int;
pub const PK_pattern_check_loops_no_c: PK_pattern_check_loops_t = 21310;
pub const PK_pattern_check_loops_yes_c: PK_pattern_check_loops_t = 21311;
pub const PK_pattern_check_loops_outside_c: PK_pattern_check_loops_t = 21312;

pub type PK_pattern_same_face_t = c_int;
pub const PK_pattern_same_face_yes_c: PK_pattern_same_face_t = 21341;
pub const PK_pattern_same_face_no_c: PK_pattern_same_face_t = 21340;

pub type PK_pattern_coi_face_t = c_int;
pub const PK_pattern_coi_face_yes_c: PK_pattern_coi_face_t = 21351;
pub const PK_pattern_coi_face_unknown_c: PK_pattern_coi_face_t = 21350;

pub type PK_pattern_reblend_t = c_int;
pub const PK_pattern_reblend_no_c: PK_pattern_reblend_t = 23270;
pub const PK_pattern_reblend_yes_c: PK_pattern_reblend_t = 23271;

pub type PK_pattern_collision_t = c_int;
pub const PK_pattern_collision_no_c: PK_pattern_collision_t = 21370;
pub const PK_pattern_collision_yes_c: PK_pattern_collision_t = 21371;

pub type PK_pattern_status_t = c_int;
pub const PK_pattern_status_ok_c: PK_pattern_status_t = 21320;
pub const PK_pattern_status_colliding_c: PK_pattern_status_t = 21326;
// [re-abi] appended 5 missing member(s) from pk-enums.h
pub const PK_pattern_status_fail_c: PK_pattern_status_t = 21321;
pub const PK_pattern_status_outside_fa_c: PK_pattern_status_t = 21322;
pub const PK_pattern_status_outside_reg_c: PK_pattern_status_t = 21323;
pub const PK_pattern_status_tf_failed_c: PK_pattern_status_t = 21324;
pub const PK_pattern_status_face_clash_c: PK_pattern_status_t = 21325;

// =============================================================================
// Matched-region option sub-structure
// =============================================================================

/// A single matched-region pair.
///
/// layout: element of the `match_regions` array in `PKU_journal_boolean_match_o`
/// (stride 0x18 = 24 bytes). The journal reads a version tag at element offset 0
/// (`FUN_180a74d60`) before the `regions[2]` pair — the old binding omitted it,
/// so every element after the first was read one field early.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_match_region_o_t {
    /// Structure version tag. // @0
    pub o_t_version: c_int,
    /// Pair of matched entities (target, tool). // @4
    pub regions: [PK_ENTITY_t; 2],
    /// Type of match (exact, contains, overlap, imprinted). // @12
    pub match_type: PK_boolean_match_type_t,
    /// Coincidence tolerance. // @16
    pub tolerance: c_double,
} // 24 bytes

/// Matched-region sub-structure for booleans.
///
/// layout: PKU_journal_boolean_match_o. The journal reads `o_t_version` @0
/// (`FUN_180a74d60`) as the first field — the old binding omitted it, shifting
/// every field down by one slot (kernel would read `match_style` as the version).
/// `auto_match` is a 1-byte flag (`PKU_journal_LOGICAL` byte read @8).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_boolean_match_o_t {
    /// Structure version tag. // @0
    pub o_t_version: c_int,
    /// Matching style (basic, auto, relax). // @4
    pub match_style: PK_boolean_match_style_t,
    /// Supply tolerance for auto matching (1-byte flag). // @8
    pub auto_match: u8,
    /// Auto match tolerance. // @16
    pub auto_match_tol: c_double,
    /// Number of supplied match regions. // @24
    pub n_match_regions: c_int,
    /// Array of match regions. // @32
    pub match_regions: *const PK_boolean_match_region_o_t,
    /// Version compatibility. // @40
    pub update: c_int,
} // 48 bytes

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
///
/// This is the **version-2 user struct (32 bytes)** — the minimal form the
/// kernel's option-migration routine (`FUN_18049b860`, RE c900fa3f430f) reads.
/// The previous 192-byte struct modelled the internal v19 layout and set
/// `o_t_version = 1`, which the kernel rejects with `PK_ERROR_o_t_version_incorrect`
/// (5043) — versions 2..=19 are accepted. For v2 the kernel copies only:
/// `function@4`, `configuration@8` (ptr), `default_tol@16` (f64), three `u8`
/// flags @24/25/26, and `fence@28`; every other option uses an internal default
/// (e.g. tolerances ≈ 1e-8, `check_fa = yes`). A NULL `configuration` is
/// auto-filled by the kernel, so no nested sub-struct is required.
///
/// Journal audit (PKU_journal_BODY_boolean_o): the journal dumps the FULL
/// current-version layout (function@4, configuration@8 ptr, selected_topolset@16
/// ptr, matched_region@24 ptr, byte flags @32+, fence@36, …, > 160 bytes). That
/// diverges from this v2 layout after @16 (v2 has `default_tol` f64 @16 where the
/// current version has a pointer) — exactly the version drift noted above. Kept
/// as the runtime-validated v2 struct; do NOT expand to the journal layout.
#[repr(C)]
pub struct PK_BODY_boolean_o_t {
    /// Structure version — MUST be in `2..=19`. Version 1 → error 5043.
    pub o_t_version: c_int,                          // @0
    /// Boolean operation (unite/subtract/intersect).
    pub function: PK_boolean_function_t,             // @4
    /// Performance sub-structure; NULL → kernel builds the default.
    pub configuration: *const PK_boolean_config_o_t, // @8
    /// Coincidence tolerance; `0.0` → kernel default (≈ 1e-8).
    pub default_tol: c_double,                       // @16
    /// Three v2 boolean flags at offsets 24/25/26 (default 0). Semantics not
    /// individually confirmed — v2 exposes them as `u8`s; likely disjoint/prune
    /// controls. All-zero reproduces the kernel's NULL-options defaults.
    pub flags_v2: [u8; 3],                           // @24..27  (@27 = padding)
    /// Fence behaviour (default `PK_boolean_fence_none_c`).
    pub fence: PK_boolean_fence_t,                   // @28
}

impl Default for PK_BODY_boolean_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 2,
            function: PK_boolean_unite_c,
            configuration: std::ptr::null(),
            default_tol: 0.0,
            flags_v2: [0, 0, 0],
            fence: PK_boolean_fence_none_c,
        }
    }
}

// =============================================================================
// PK_FACE_boolean_o_t — Local boolean options
// =============================================================================

/// Options for `PK_FACE_boolean_2` (local boolean).
///
/// layout: PKU_journal_FACE_boolean_o (V37 current-version, 128 bytes). Field
/// order and byte offsets are taken directly from the journal, which differs
/// substantially from the older header layout the previous binding modelled:
/// - fields are reordered (`default_tol`/`max_tol` sit at @48/@56, after the
///   flag block, not right after `configuration`);
/// - the sub-struct pointers appear as `configuration`@8, `select_region`@16,
///   `matched_region`@24, `target_face_overflow`@104, `tool_face_overflow`@112;
/// - `nm_edge_repair`/`blend_radius` are NOT present in the V37 journal and were
///   removed (they belonged to a different/older layout);
/// - the packed flags are 1-byte fields (journal byte reads at consecutive
///   offsets: @32/@33/@34/@35, @40/@41, @64/@65). `extend_face_list` is likewise
///   a 1-byte small-enum (values 0..3) at @32.
#[repr(C)]
pub struct PK_FACE_boolean_o_t {
    /// Version tag for this options struct. // @0
    pub o_t_version: c_int,
    /// Boolean operation type (unite/subtract/intersect). // @4
    pub function: PK_boolean_function_t,
    /// Performance optimization sub-structure (NULL for default). // @8
    pub configuration: *const PK_boolean_config_o_t,
    /// Region selection for local booleans (NULL for default). // @16
    pub select_region: *const PK_boolean_select_region_o_t,
    /// Matched-region sub-structure (NULL for default). // @24
    pub matched_region: *const PK_boolean_match_o_t,
    /// Add neighboring faces (local only); 1-byte small-enum. // @32
    pub extend_face_list: u8,
    /// Check tool face self-intersection (1-byte flag). // @33
    pub stop_self_intersection: u8,
    /// Request tracking data (1-byte flag). // @34
    pub tracking: u8,
    /// Merge imprinted edges (1-byte flag). // @35
    pub merge_imprinted: u8,
    /// Fence behaviour for subtract. // @36
    pub fence: PK_boolean_fence_t,
    /// Allow disjoint result body (1-byte flag). // @40
    pub allow_disjoint: u8,
    /// Avoid merging pre-existing edges (1-byte flag). // @41
    pub selective_merge: u8,
    /// Check faces adjacent to imprints. // @44
    pub check_fa: PK_boolean_check_fa_t,
    /// Coincidence/approximation tolerance. // @48
    pub default_tol: c_double,
    /// Maximum entity tolerance (0.0 = not set). // @56
    pub max_tol: c_double,
    /// Merge face attributes (1-byte flag). // @64
    pub merge_attributes: u8,
    /// Which edge survives on coincidence (1-byte flag). // @65
    pub keep_target_edges: u8,
    /// Imprint completion on target. // @68
    pub imprint_complete_targ: PK_imprint_complete_t,
    /// Imprint completion on tool. // @72
    pub imprint_complete_tool: PK_imprint_complete_t,
    /// Force target body type interpretation. // @76
    pub target_material_side: PK_boolean_material_t,
    /// Force tool body type interpretation. // @80
    pub tool_material_side: PK_boolean_material_t,
    /// Preferred result body type. // @84
    pub resulting_body_type: PK_boolean_prefer_t,
    /// Limit target face deletion scope (1-byte flag). // @88
    pub limit_target_faces: u8,
    /// Detect no-effect operations. // @92
    pub flag_no_effect: PK_boolean_no_effect_t,
    /// Limit tool face deletion scope (1-byte flag). // @96
    pub limit_tool_faces: u8,
    /// Tracking detail level. // @100
    pub tracking_type: PK_boolean_track_type_t,
    /// Handle overflow from target to tool (NULL for default). // @104
    pub target_face_overflow: *const PK_FACE_overflow_o_t,
    /// Handle overflow from tool to target (NULL for default). // @112
    pub tool_face_overflow: *const PK_FACE_overflow_o_t,
    /// Version compatibility. // @120
    pub update: PK_boolean_update_t,
} // 128 bytes

impl Default for PK_FACE_boolean_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            function: PK_boolean_unite_c,
            configuration: std::ptr::null(),
            select_region: std::ptr::null(),
            matched_region: std::ptr::null(),
            extend_face_list: PK_imprint_face_list_no_c as u8,
            stop_self_intersection: 0,
            tracking: 0,
            merge_imprinted: 0,
            fence: PK_boolean_fence_none_c,
            allow_disjoint: 0,
            selective_merge: 0,
            check_fa: PK_boolean_check_fa_yes_c,
            default_tol: 0.0,
            max_tol: 0.0,
            merge_attributes: 0,
            keep_target_edges: 0,
            imprint_complete_targ: PK_imprint_complete_no_c,
            imprint_complete_tool: PK_imprint_complete_no_c,
            target_material_side: PK_boolean_material_default_c,
            tool_material_side: PK_boolean_material_default_c,
            resulting_body_type: PK_boolean_prefer_original_c,
            limit_target_faces: 0,
            flag_no_effect: PK_boolean_no_effect_basic_c,
            limit_tool_faces: 0,
            tracking_type: PK_boolean_track_type_basic_c,
            target_face_overflow: std::ptr::null(),
            tool_face_overflow: std::ptr::null(),
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
    /// Overall boolean status (@0) — e.g. `PK_boolean_result_success_c` (21650).
    /// The old struct put this field last, so `n_bodies` read the success token
    /// (21650) instead of the real count.
    pub result: PK_boolean_result_t, // @0
    /// Number of resulting bodies (@4).
    pub n_bodies: c_int, // @4
    /// Array of resulting body tags (@8) — includes the (replaced) target body.
    pub bodies: *mut PK_BODY_t, // @8
    /// Reserved — the kernel's results struct carries further fields (freed via
    /// `PK_boolean_r_f`); kept zeroed so the allocation is not undersized.
    pub _reserved: [u8; 8], // @16
}

// =============================================================================
// Imprint option structures
// =============================================================================

/// Options for `PK_BODY_imprint_body`.
///
/// Authoritative 56-byte layout (pk-option-structs.md): the complete/extend
/// fields are paired per target/tool, `update`@40 precedes have_tolerance/
/// tolerance. The old binding interleaved complete/extend and put `update` after
/// tolerance (48 B) with `update=0` (an INVALID token).
#[repr(C)]
pub struct PK_BODY_imprint_o_t {
    pub o_t_version: c_int,                            // @0
    pub imprint_tool: PK_LOGICAL_t,                    // @4
    pub imprint_overlapping: PK_LOGICAL_t,             // @8
    pub matched_region: *const PK_boolean_match_o_t,   // @16
    pub imprint_complete_targ: PK_imprint_complete_t,  // @24
    pub imprint_extend_targ: PK_imprint_extend_t,      // @28
    pub imprint_complete_tool: PK_imprint_complete_t,  // @32
    pub imprint_extend_tool: PK_imprint_extend_t,      // @36
    pub update: PK_boolean_update_t,                   // @40
    pub have_tolerance: PK_LOGICAL_t,                  // @44
    pub tolerance: c_double,                           // @48
} // 56 bytes

impl Default for PK_BODY_imprint_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            imprint_tool: PK_LOGICAL_false,
            imprint_overlapping: PK_LOGICAL_false,
            matched_region: std::ptr::null(),
            imprint_complete_targ: PK_imprint_complete_no_c,
            imprint_extend_targ: PK_imprint_extend_tangent_c,
            imprint_complete_tool: PK_imprint_complete_no_c,
            imprint_extend_tool: PK_imprint_extend_tangent_c,
            update: PK_boolean_update_default_c,
            have_tolerance: PK_LOGICAL_false,
            tolerance: 0.0,
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<PK_BODY_imprint_o_t>() == 56);
};

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
///
/// layout: PKU_journal_BODY_make_section_o (80 bytes). The previous 6-field
/// binding was missing `o_t_version`, `keep_as_facet`, the `offsets`/`n_offsets`
/// and `banned_classes`/`n_banned_classes` array pairs, and `keep_coi_faces`,
/// and had `tracking`/`allow_disjoint` ahead of the tolerances. Per the journal,
/// `tracking`@24 and `allow_disjoint`@25 are adjacent 1-byte flags.
#[repr(C)]
pub struct PK_BODY_make_section_o_t {
    /// Structure version tag. // @0
    pub o_t_version: c_int,
    /// Default tolerance. // @8
    pub default_tol: c_double,
    /// Maximum tolerance. // @16
    pub max_tol: c_double,
    /// Return tracking info (1-byte flag). // @24
    pub tracking: u8,
    /// Allow disjoint bodies (1-byte flag). // @25
    pub allow_disjoint: u8,
    /// Sheet or wire result body type. // @28
    pub result_body_type: PK_BODY_type_t,
    /// Version compatibility. // @32
    pub update: PK_boolean_update_t,
    /// Return facet-only or mixed geometry. // @36
    pub keep_as_facet: PK_BODY_keep_as_facet_t,
    /// Number of section offsets. // @40
    pub n_offsets: c_int,
    /// Array of section offset distances (doubles). // @48
    pub offsets: *const c_double,
    /// Array of banned output face classes. // @56
    pub banned_classes: *const PK_CLASS_t,
    /// Number of banned face classes. // @64
    pub n_banned_classes: c_int,
    /// Keep coincident faces (read as full int by the journal). // @72
    pub keep_coi_faces: PK_LOGICAL_t,
} // 80 bytes

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

/// Which side(s) of the section surface to keep.
pub type PK_section_fence_t = c_int;
pub const PK_section_fence_front_c: PK_section_fence_t = 18200;
pub const PK_section_fence_back_c: PK_section_fence_t = 18201;
pub const PK_section_fence_both_c: PK_section_fence_t = 18202;

/// Whether to check faces adjacent to the section.
pub type PK_section_check_fa_t = c_int;
pub const PK_section_check_fa_no_c: PK_section_check_fa_t = 21810;
pub const PK_section_check_fa_yes_c: PK_section_check_fa_t = 21811;

/// Options for `PK_BODY_section_with_surf` / `_with_sheet` (48 bytes).
///
/// layout: PKU_journal_BODY_section_o. The field order already matched the
/// journal, but the previous binding modelled the five LOGICAL fields as 4-byte
/// `c_int`s (giving 64 bytes). The journal reads them as adjacent 1-byte flags
/// (`merge_imprinted`@16 / `merge_new_faces`@17 / `selective_merge`@18, and
/// `tracking`@40 / `keep_target_edges`@41), so `check_fa`@20, the tolerances
/// @24/@32 and `keep_as_facet`@44 all sit 16 bytes earlier than before.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_BODY_section_o_t {
    pub o_t_version: c_int,                           // @0
    pub fence: PK_section_fence_t,                    // @4
    pub matched_region: *const PK_boolean_match_o_t,  // @8
    pub merge_imprinted: u8,                          // @16
    pub merge_new_faces: u8,                          // @17
    pub selective_merge: u8,                          // @18
    pub check_fa: PK_section_check_fa_t,              // @20
    pub default_tol: c_double,                        // @24
    pub max_tol: c_double,                            // @32
    pub tracking: u8,                                 // @40
    pub keep_target_edges: u8,                        // @41
    pub keep_as_facet: PK_BODY_keep_as_facet_t,       // @44
} // 48 bytes

impl Default for PK_BODY_section_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            fence: PK_section_fence_both_c,
            matched_region: std::ptr::null(),
            merge_imprinted: 0,
            merge_new_faces: 0,
            selective_merge: 0,
            check_fa: PK_section_check_fa_no_c,
            default_tol: 0.0,
            max_tol: 0.0,
            tracking: 0,
            keep_target_edges: 0,
            keep_as_facet: PK_BODY_keep_as_facet_no_c,
        }
    }
}

/// Results from section operations (`PK_section_r_t`, 64 bytes). Kept opaque —
/// the front/back face/body arrays are filled by an internal routine
/// (`FUN_180b997a0`) whose field offsets are not yet mapped; the wrapper backs
/// it with a buffer and reads the resulting bodies from the session instead.
#[repr(C)]
pub struct PK_section_r_t {
    _bytes: [u8; 64],
}

/// Options for `PK_FACE_section_with_sheet` (local sectioning).
///
/// layout: PKU_journal_FACE_section_o (64 bytes). Previously opaque. The journal
/// reads `extend_face_list`@32 / `merge_imprinted`@33 / `merge_new_faces`@34 /
/// `selective_merge`@35 and `tracking`@56 / `keep_target_edges`@57 as adjacent
/// 1-byte flags. `fence`/`check_fa` are typed to match `PK_BODY_section_o_t`
/// (both are 4-byte ints; the exact section-vs-boolean token family is not
/// runtime-confirmed and does not affect layout).
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PK_FACE_section_o_t {
    /// Structure version tag. // @0
    pub o_t_version: c_int,
    /// Which side(s) of the section to keep. // @4
    pub fence: PK_section_fence_t,
    /// Performance optimization sub-structure (NULL for default). // @8
    pub configuration: *const PK_boolean_config_o_t,
    /// Region selection (NULL for default). // @16
    pub select_region: *const PK_boolean_select_region_o_t,
    /// Matched-region sub-structure (NULL for default). // @24
    pub matched_region: *const PK_boolean_match_o_t,
    /// Add neighboring faces (1-byte flag). // @32
    pub extend_face_list: u8,
    /// Merge mergeable imprinted edges (1-byte flag). // @33
    pub merge_imprinted: u8,
    /// Merge edges adjacent to new section faces (1-byte flag). // @34
    pub merge_new_faces: u8,
    /// Avoid merging pre-existing edges (1-byte flag). // @35
    pub selective_merge: u8,
    /// Check faces adjacent to imprinted edges. // @36
    pub check_fa: PK_section_check_fa_t,
    /// Default tolerance. // @40
    pub default_tol: c_double,
    /// Maximum tolerance. // @48
    pub max_tol: c_double,
    /// Return tracking info (1-byte flag). // @56
    pub tracking: u8,
    /// Which edge survives on coincidence (1-byte flag). // @57
    pub keep_target_edges: u8,
} // 64 bytes

/// Options for `PK_FACE_imprint_faces`.
#[repr(C)]
pub struct PK_FACE_imprint_o_t { _private: [u8; 0] }

/// Results from face imprint operations.
#[repr(C)]
pub struct PK_imprint_r_t { _bytes: [u8; 128] }

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

/// Options for `PK_BODY_imprint_cus_vector`.
#[repr(C)]
pub struct PK_BODY_imprint_cus_vector_o_t { _private: [u8; 0] }

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
        tracking: *mut PK_TOPOL_track_r_t,
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
        tracking: *mut PK_TOPOL_track_r_t,
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
        options: *mut PK_BODY_imprint_o_t,
        results: *mut PK_imprint_r_t,
    ) -> PK_ERROR_code_t;

    /// Free the contents of a `PK_imprint_r_t` (kernel-allocated arrays).
    pub fn PK_imprint_r_f(results: *mut PK_imprint_r_t) -> PK_ERROR_code_t;

    /// Imprint edges/vertices on target body where specified faces intersect.
    pub fn PK_BODY_imprint_faces_2(
        body: PK_BODY_t,
        n_faces: c_int,
        faces: *mut PK_FACE_t,
        options: *mut PK_BODY_imprint_faces_o_t,
        results: *mut PK_imprint_r_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Imprint edges/vertices on specified face sets.
    pub fn PK_FACE_imprint_faces_2(
        n_targets: c_int,
        targets: *mut PK_FACE_t,
        n_tools: c_int,
        tools: *mut PK_FACE_t,
        options: *mut PK_FACE_imprint_faces_o_t,
        results: *mut PK_imprint_r_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Imprint curves directly on a face (curves must be coincident with face surface).
    pub fn PK_FACE_imprint_curves_2(
        face: PK_FACE_t,
        n_curves: c_int,
        curves: *mut PK_CURVE_t,
        intervals: *mut PK_INTERVAL_t,
        options: *mut PK_FACE_imprint_curves_o_t,
        tracking: *mut PK_ENTITY_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Imprint a plane onto a body.
    pub fn PK_BODY_imprint_plane_2(
        body: PK_BODY_t,
        plane: PK_PLANE_t,
        options: *mut PK_BODY_imprint_plane_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Add a vertex to an edge at specified coordinates, splitting the edge.
    pub fn PK_EDGE_imprint_point(
        edge: PK_EDGE_t,
        point: PK_POINT_t,
        new_vertex: *mut PK_VERTEX_t,
        new_edge: *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Add an isolated vertex to a face, creating a new loop.
    // V35 vendor form: takes a PK_POINT_t entity (by value), not a vector ptr.
    pub fn PK_FACE_imprint_point(
        face: PK_FACE_t,
        point: PK_POINT_t,
        vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Add an acorn vertex to a region of a general body.
    // V35 vendor form: takes a PK_POINT_t entity (by value), not a vector ptr.
    pub fn PK_REGION_imprint_point(
        region: PK_REGION_t,
        point: PK_POINT_t,
        vertex: *mut PK_VERTEX_t,
    ) -> PK_ERROR_code_t;

    /// Given loops of coincident edges, calculates which face sets survive.
    pub fn PK_BODY_identify_facesets(
        target: PK_BODY_t,
        tool: PK_BODY_t,
        n_edges: c_int,
        target_edges: *mut PK_EDGE_t,
        tool_edges: *mut PK_EDGE_t,
        n_vertices: c_int,
        target_vertices: *mut PK_VERTEX_t,
        tool_vertices: *mut PK_VERTEX_t,
        options: *mut PK_BODY_identify_facesets_o_t,
        results: *mut PK_identify_facesets_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Imprinting — legacy (vec / normal)
    // =========================================================================

    /// Legacy: imprint curves onto a body by projecting in a specified direction.
    pub fn PK_BODY_imprint_cus_vec(
        body: PK_BODY_t,
        n_curves: c_int,
        curves: *mut PK_CURVE_t,
        intervals: *mut PK_INTERVAL_t,
        tol: c_double,
        direction: *const PK_VECTOR_t,
        options: *mut PK_BODY_imprint_cus_vec_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Legacy: imprint curves onto specified faces by projecting in a specified direction.
    pub fn PK_FACE_imprint_cus_vec(
        n_targets: c_int,
        targets: *mut PK_FACE_t,
        n_curves: c_int,
        curves: *mut PK_CURVE_t,
        intervals: *mut PK_INTERVAL_t,
        tol: c_double,
        direction: *const PK_VECTOR_t,
        options: *mut PK_FACE_imprint_cus_vec_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Legacy: imprint curves onto a body by projecting down the face normal.
    pub fn PK_BODY_imprint_cus_normal(
        body: PK_BODY_t,
        n_curves: c_int,
        curves: *mut PK_CURVE_t,
        intervals: *mut PK_INTERVAL_t,
        tol: c_double,
        options: *mut PK_BODY_imprint_cus_normal_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
    ) -> PK_ERROR_code_t;

    /// Legacy: imprint curves onto specified faces by projecting down the face normal.
    pub fn PK_FACE_imprint_cus_normal(
        n_targets: c_int,
        targets: *mut PK_FACE_t,
        n_curves: c_int,
        curves: *mut PK_CURVE_t,
        intervals: *mut PK_INTERVAL_t,
        tol: c_double,
        options: *mut PK_FACE_imprint_cus_normal_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
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
    /// Section a body with a surface. V35 (4 args):
    /// `(PK_BODY_t target, PK_SURF_t surface, const PK_BODY_section_o_t *options,
    ///  PK_section_r_t *results)` — the results (front/back faces/bodies) come back
    /// in the opaque `PK_section_r_t`, not individual out-params. The old binding
    /// used 7 args with the wrong option type.
    pub fn PK_BODY_section_with_surf(
        target: PK_BODY_t,
        surface: PK_SURF_t,
        options: *const PK_BODY_section_o_t,
        results: *mut PK_section_r_t,
    ) -> PK_ERROR_code_t;

    /// Section a body with a sheet tool (may be disjoint). Tool is deleted.
    pub fn PK_BODY_section_with_sheet_2(
        target: PK_BODY_t,
        sheet: PK_BODY_t,
        options: *mut PK_BODY_section_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_section_2_r_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Local sectioning
    // =========================================================================

    /// Section particular faces of a target body with faces of a sheet body.
    pub fn PK_FACE_section_with_sheet_2(
        n_targets: c_int,
        targets: *mut PK_FACE_t,
        n_tools: c_int,
        tools: *mut PK_FACE_t,
        options: *mut PK_FACE_section_o_t,
        tracking: *mut PK_TOPOL_track_r_t,
        results: *mut PK_section_2_r_t,
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
    /// [RE-regenerated from V35 TSV prototype]
    pub fn PK_FACE_instance_bodies(
        n_target_faces: c_int,
        target_faces: *mut PK_FACE_t,
        n_tool_bodies: c_int,
        tool_bodies: *mut PK_BODY_t,
        transforms: *mut PK_TRANSF_array_t,
        options: *mut PK_FACE_instance_bodies_o_t,
        instance_tracking: *mut PK_TOPOL_track_r_t,
        tracking: *mut PK_TOPOL_track_r_t,
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
    // Body imprinting (deprecated, superseded by PK_CURVE_project)
    // =========================================================================

    /// Imprint curves on body by projecting down face normals (deprecated).
    pub fn PK_BODY_imprint_curves_normal(
        body: PK_BODY_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        tol: c_double,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Imprint curves on body by projecting in a direction (deprecated).
    pub fn PK_BODY_imprint_curves_vector(
        body: PK_BODY_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        tol: c_double,
        direction: PK_VECTOR_t,
        n_edges: *mut c_int,
        edges: *mut *mut PK_EDGE_t,
    ) -> PK_ERROR_code_t;

    /// Imprint curves on body by vector projection with options (deprecated).
    pub fn PK_BODY_imprint_cus_vector(
        body: PK_BODY_t,
        n_curves: c_int,
        curves: *const PK_CURVE_t,
        intervals: *const PK_INTERVAL_t,
        tol: c_double,
        direction: PK_VECTOR_t,
        options: *const PK_BODY_imprint_cus_vector_o_t,
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
