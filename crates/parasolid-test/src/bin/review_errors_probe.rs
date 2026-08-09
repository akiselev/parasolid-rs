//! ADVERSARIAL REVIEW PROBE — PK_ERROR_* table + error-reporting path.
//!
//! Read-only investigation. Creates no repo state.
//!
//!   cargo build -p parasolid-test --bin review_errors_probe --target x86_64-pc-windows-gnu
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/review_errors_probe.exe

use std::io::Write;

use parasolid::*;
use parasolid_sys::*;

const BUF: usize = 512;
const POISON: u8 = 0xAA;

/// (code, name) parsed from parasolid-sys/src/error_codes.rs at review time.
const TABLE: &[(i32, &str)] = &[
    (0, "PK_ERROR_no_errors"),
    (1, "PK_ERROR_bad_angle"),
    (2, "PK_ERROR_buffer_overflow"),
    (3, "PK_ERROR_radii_both_0"),
    (4, "PK_ERROR_cone_too_sharp"),
    (7, "PK_ERROR_has_no_name"),
    (8, "PK_ERROR_has_no_owner"),
    (10, "PK_ERROR_wrong_entity"),
    (11, "PK_ERROR_bad_name"),
    (12, "PK_ERROR_bad_type_combn"),
    (13, "PK_ERROR_not_unique"),
    (14, "PK_ERROR_distance_lt_0"),
    (15, "PK_ERROR_distance_le_0"),
    (16, "PK_ERROR_radius_le_0"),
    (18, "PK_ERROR_radius_lt_0"),
    (19, "PK_ERROR_not_found"),
    (20, "PK_ERROR_not_connected"),
    (22, "PK_ERROR_not_an_entity"),
    (25, "PK_ERROR_null_axis"),
    (27, "PK_ERROR_cant_open_jrnl"),
    (28, "PK_ERROR_has_parent"),
    (29, "PK_ERROR_bad_index"),
    (30, "PK_ERROR_bad_type"),
    (31, "PK_ERROR_null_direction"),
    (32, "PK_ERROR_rot_angle_eq_0"),
    (33, "PK_ERROR_lt_3_sides"),
    (34, "PK_ERROR_is_attached"),
    (35, "PK_ERROR_dont_intersect"),
    (36, "PK_ERROR_majaxi_not_perpn"),
    (37, "PK_ERROR_wrong_transf"),
    (38, "PK_ERROR_bad_selection_code"),
    (39, "PK_ERROR_bad_value"),
    (40, "PK_ERROR_sc_factor_le_0"),
    (41, "PK_ERROR_su_are_coincident"),
    (42, "PK_ERROR_bb_is_off"),
    (48, "PK_ERROR_none_mergeable"),
    (50, "PK_ERROR_cant_do_tweak"),
    (51, "PK_ERROR_inconsistent_geom"),
    (54, "PK_ERROR_not_on_face"),
    (55, "PK_ERROR_impossible_spin"),
    (57, "PK_ERROR_impossible_sweep"),
    (58, "PK_ERROR_key_not_found"),
    (59, "PK_ERROR_not_in_same_part"),
    (61, "PK_ERROR_no_geometry"),
    (62, "PK_ERROR_geom_topol_mismatch"),
    (63, "PK_ERROR_receive_failed"),
    (64, "PK_ERROR_geom_not_needed"),
    (67, "PK_ERROR_not_on_curve"),
    (68, "PK_ERROR_still_referenced"),
    (73, "PK_ERROR_fragment"),
    (77, "PK_ERROR_cant_find_su"),
    (79, "PK_ERROR_empty_list"),
    (80, "PK_ERROR_not_a_list"),
    (82, "PK_ERROR_mass_eq_0"),
    (85, "PK_ERROR_density_le_0"),
    (87, "PK_ERROR_dont_make_solid"),
    (96, "PK_ERROR_missing_geom"),
    (99, "PK_ERROR_attr_not_found"),
    (101, "PK_ERROR_not_solid"),
    (103, "PK_ERROR_corrupt_body"),
    (105, "PK_ERROR_bad_geom_topol"),
    (106, "PK_ERROR_negative_body"),
    (109, "PK_ERROR_bad_char_string"),
    (110, "PK_ERROR_bad_spec_code"),
    (111, "PK_ERROR_weight_le_0"),
    (116, "PK_ERROR_illegal_degeneracy"),
    (120, "PK_ERROR_bad_parameter"),
    (129, "PK_ERROR_discontinuous_surface"),
    (131, "PK_ERROR_discontinuous_curve"),
    (132, "PK_ERROR_order_lt_2"),
    (135, "PK_ERROR_bad_dimension"),
    (141, "PK_ERROR_su_self_intersect"),
    (157, "PK_ERROR_cant_do_intersect"),
    (330, "PK_ERROR_cant_fix_blends"),
    (334, "PK_ERROR_bad_blend_bound"),
    (335, "PK_ERROR_not_blended"),
    (336, "PK_ERROR_blend_didnt_check"),
    (350, "PK_ERROR_bad_request_code"),
    (357, "PK_ERROR_wrong_entity_in_array"),
    (359, "PK_ERROR_not_same_length"),
    (360, "PK_ERROR_bad_view_mx"),
    (361, "PK_ERROR_bad_pixel_map"),
    (364, "PK_ERROR_bad_light_source"),
    (367, "PK_ERROR_eye_in_box"),
    (503, "PK_ERROR_cyclic_assy"),
    (504, "PK_ERROR_anon_sub_part"),
    (505, "PK_ERROR_different_types"),
    (506, "PK_ERROR_existing_attdef"),
    (507, "PK_ERROR_R1_R2_mismatch"),
    (508, "PK_ERROR_radius_sum_le_0"),
    (509, "PK_ERROR_wrong_list_type"),
    (510, "PK_ERROR_bad_tag_in_list"),
    (511, "PK_ERROR_duplicate_array_item"),
    (512, "PK_ERROR_not_in_group"),
    (513, "PK_ERROR_wrong_class_for_group"),
    (519, "PK_ERROR_array_too_short"),
    (520, "PK_ERROR_already_in_group"),
    (522, "PK_ERROR_attr_mismatch"),
    (523, "PK_ERROR_list_wrong_length"),
    (524, "PK_ERROR_part_not_keyed"),
    (525, "PK_ERROR_cant_heal_wound"),
    (528, "PK_ERROR_already_loaded"),
    (529, "PK_ERROR_already_saved"),
    (530, "PK_ERROR_key_in_use"),
    (531, "PK_ERROR_closed_faces"),
    (532, "PK_ERROR_at_singularity"),
    (533, "PK_ERROR_size_mismatch"),
    (540, "PK_ERROR_duplicate_tools"),
    (541, "PK_ERROR_instanced_tools"),
    (542, "PK_ERROR_mixed_sheets_solids"),
    (543, "PK_ERROR_cant_unite_solid_sheet"),
    (545, "PK_ERROR_same_tool_and_target"),
    (546, "PK_ERROR_invalid_bodies"),
    (547, "PK_ERROR_non_manifold"),
    (549, "PK_ERROR_t_sheet"),
    (553, "PK_ERROR_wrong_sub_type"),
    (555, "PK_ERROR_attr_defn_mismatch"),
    (557, "PK_ERROR_cant_find_file"),
    (558, "PK_ERROR_get_snapshot_failed"),
    (560, "PK_ERROR_transmit_failed"),
    (561, "PK_ERROR_bad_filename"),
    (562, "PK_ERROR_save_snapshot_failed"),
    (565, "PK_ERROR_bad_key"),
    (566, "PK_ERROR_journal_not_open"),
    (570, "PK_ERROR_bad_state_combn"),
    (850, "PK_ERROR_rollmark_failed"),
    (854, "PK_ERROR_no_rollmark"),
    (855, "PK_ERROR_roll_is_off"),
    (856, "PK_ERROR_roll_forward_fail"),
    (860, "PK_ERROR_impossible_taper"),
    (900, "PK_ERROR_system_error"),
    (901, "PK_ERROR_memory_full"),
    (902, "PK_ERROR_nitems_lt_0"),
    (903, "PK_ERROR_nitems_le_0"),
    (904, "PK_ERROR_modified_sub_part"),
    (905, "PK_ERROR_part_not_isolated"),
    (906, "PK_ERROR_null_arg_address"),
    (907, "PK_ERROR_bad_option_data"),
    (908, "PK_ERROR_not_a_logical"),
    (909, "PK_ERROR_bad_box"),
    (911, "PK_ERROR_bad_position"),
    (912, "PK_ERROR_empty_assy"),
    (913, "PK_ERROR_keyed_part_mismatch"),
    (914, "PK_ERROR_unsuitable_entity"),
    (915, "PK_ERROR_not_on_surface"),
    (916, "PK_ERROR_bad_shared_entity"),
    (917, "PK_ERROR_bad_shared_dep"),
    (919, "PK_ERROR_attr_type_not_defined"),
    (920, "PK_ERROR_bad_blend_param"),
    (921, "PK_ERROR_bad_sharing"),
    (922, "PK_ERROR_corrupt_file"),
    (923, "PK_ERROR_wrong_version"),
    (924, "PK_ERROR_not_at_rollmark"),
    (925, "PK_ERROR_radius_eq_0"),
    (926, "PK_ERROR_radius_too_large"),
    (927, "PK_ERROR_distance_too_large"),
    (928, "PK_ERROR_cant_open_file"),
    (929, "PK_ERROR_at_terminator"),
    (930, "PK_ERROR_bad_precision"),
    (931, "PK_ERROR_modeller_not_started"),
    (932, "PK_ERROR_modeller_not_stopped"),
    (933, "PK_ERROR_bad_user_field_size"),
    (934, "PK_ERROR_recursive_call"),
    (935, "PK_ERROR_bad_hull"),
    (936, "PK_ERROR_usfd_mismatch"),
    (937, "PK_ERROR_wrong_format"),
    (938, "PK_ERROR_wire_body"),
    (939, "PK_ERROR_not_sheet"),
    (940, "PK_ERROR_bad_wire"),
    (941, "PK_ERROR_bad_end_points"),
    (942, "PK_ERROR_crossing_edge"),
    (943, "PK_ERROR_crossing_vertex"),
    (944, "PK_ERROR_bad_vertex"),
    (945, "PK_ERROR_aborted"),
    (946, "PK_ERROR_not_interrupted"),
    (947, "PK_ERROR_run_time_error"),
    (948, "PK_ERROR_fatal_error"),
    (949, "PK_ERROR_no_user_fields"),
    (950, "PK_ERROR_wrong_surface"),
    (951, "PK_ERROR_opposed_sheets"),
    (952, "PK_ERROR_coplanar"),
    (956, "PK_ERROR_bad_accuracy"),
    (957, "PK_ERROR_coincident"),
    (958, "PK_ERROR_atol_too_small"),
    (959, "PK_ERROR_ctol_too_small"),
    (960, "PK_ERROR_stol_too_small"),
    (961, "PK_ERROR_wrong_direction"),
    (962, "PK_ERROR_non_orth_matrix"),
    (963, "PK_ERROR_bad_component"),
    (964, "PK_ERROR_bad_rollfile_size"),
    (965, "PK_ERROR_cant_be_aborted"),
    (966, "PK_ERROR_hulls_intersect"),
    (967, "PK_ERROR_abort_from_go"),
    (969, "PK_ERROR_all_faces_in_body"),
    (970, "PK_ERROR_schema_access_error"),
    (971, "PK_ERROR_schema_corrupt"),
    (972, "PK_ERROR_cant_intsc_solid_sheet"),
    (973, "PK_ERROR_file_access_error"),
    (974, "PK_ERROR_bad_file_format"),
    (975, "PK_ERROR_bad_file_guise"),
    (976, "PK_ERROR_bad_rolling_ball"),
    (977, "PK_ERROR_coincident_points"),
    (978, "PK_ERROR_bad_knots"),
    (979, "PK_ERROR_bad_derivative"),
    (980, "PK_ERROR_wrong_number_knots"),
    (981, "PK_ERROR_wrong_number_derivs"),
    (982, "PK_ERROR_incompatible_props"),
    (983, "PK_ERROR_repeated_knots"),
    (984, "PK_ERROR_curves_dont_meet"),
    (985, "PK_ERROR_insufficient_curves"),
    (986, "PK_ERROR_bad_curves"),
    (987, "PK_ERROR_bad_order"),
    (988, "PK_ERROR_insufficient_points"),
    (989, "PK_ERROR_bad_parametric_prop"),
    (990, "PK_ERROR_illegal_owner"),
    (991, "PK_ERROR_unchecked_entity"),
    (992, "PK_ERROR_incompatible_curves"),
    (993, "PK_ERROR_cant_make_bspline"),
    (994, "PK_ERROR_cu_are_coincident"),
    (995, "PK_ERROR_withdrawn_surface"),
    (996, "PK_ERROR_face_not_planar"),
    (997, "PK_ERROR_request_not_supported"),
    (998, "PK_ERROR_contradictory_request"),
    (999, "PK_ERROR_invalid_geometry"),
    (1000, "PK_ERROR_file_already_exists"),
    (1001, "PK_ERROR_too_many_control_pts"),
    (1002, "PK_ERROR_bad_string"),
    (1003, "PK_ERROR_mend_attempt_failure"),
    (1004, "PK_ERROR_bad_tag_in_list_tree"),
    (1005, "PK_ERROR_bad_list_tree"),
    (1006, "PK_ERROR_cyclic_list_reference"),
    (1007, "PK_ERROR_empty_list_in_tree"),
    (1008, "PK_ERROR_cant_make_trimmed_sf"),
    (1009, "PK_ERROR_bad_class_event_comb"),
    (1010, "PK_ERROR_too_many_derivatives"),
    (1011, "PK_ERROR_bad_deriv_vertices"),
    (1012, "PK_ERROR_bad_degen_vertices"),
    (1013, "PK_ERROR_not_on_edge"),
    (1014, "PK_ERROR_no_closest_approach"),
    (1015, "PK_ERROR_cant_do_clash"),
    (1016, "PK_ERROR_targ_faces_many_bodies"),
    (1017, "PK_ERROR_tool_faces_many_bodies"),
    (1018, "PK_ERROR_cant_do_imprint"),
    (1019, "PK_ERROR_topol_not_from_body"),
    (1020, "PK_ERROR_inconsistent_facesets"),
    (1021, "PK_ERROR_FG_eval_not_found"),
    (1022, "PK_ERROR_FG_data_alloc_error"),
    (1023, "PK_ERROR_FG_data_not_found"),
    (1024, "PK_ERROR_FG_evaluator_error"),
    (1025, "PK_ERROR_FG_modelling_error"),
    (1026, "PK_ERROR_solid_body"),
    (1027, "PK_ERROR_different_bodies"),
    (1028, "PK_ERROR_wrong_number_edges"),
    (1029, "PK_ERROR_cant_blend_vertex"),
    (1030, "PK_ERROR_blends_overlap"),
    (1031, "PK_ERROR_edges_intersect"),
    (1032, "PK_ERROR_not_in_same_body"),
    (1033, "PK_ERROR_unsuitable_topology"),
    (1034, "PK_ERROR_cu_self_intersect"),
    (1035, "PK_ERROR_linear_multi_seg"),
    (1036, "PK_ERROR_no_eds_from_target"),
    (1037, "PK_ERROR_cant_offset"),
    (1038, "PK_ERROR_FG_doubles"),
    (1039, "PK_ERROR_FG_ints"),
    (1040, "PK_ERROR_partial_coi_found"),
    (1041, "PK_ERROR_bodies_dont_knit"),
    (1042, "PK_ERROR_pattern_invalid"),
    (1043, "PK_ERROR_bad_tolerance"),
    (1044, "PK_ERROR_cant_extract_geom"),
    (1045, "PK_ERROR_bad_basis_surf"),
    (1046, "PK_ERROR_FG_receive_failure"),
    (1047, "PK_ERROR_FG_snapshot_failure"),
    (1048, "PK_ERROR_cant_create_pattern"),
    (1049, "PK_ERROR_tag_limit_exceeded"),
    (1050, "PK_ERROR_tag_limit_out_of_range"),
    (1051, "PK_ERROR_cant_find_extreme"),
    (1052, "PK_ERROR_disc_full"),
    (1053, "PK_ERROR_cant_find_derivs"),
    (1054, "PK_ERROR_too_many_targets"),
    (1055, "PK_ERROR_duplicate_targets"),
    (1056, "PK_ERROR_curve_already_trimmed"),
    (1057, "PK_ERROR_curve_too_short"),
    (1058, "PK_ERROR_boolean_failure"),
    (1059, "PK_ERROR_duplicate_item"),
    (1060, "PK_ERROR_failed_to_trim"),
    (1061, "PK_ERROR_unsuitable_loop"),
    (1062, "PK_ERROR_failed_to_replace"),
    (1063, "PK_ERROR_failed_to_create_sp"),
    (1064, "PK_ERROR_tolerances_too_tight"),
    (1065, "PK_ERROR_fru_error"),
    (1066, "PK_ERROR_incorrect_mc_conf"),
    (1067, "PK_ERROR_partial_no_intersect"),
    (1068, "PK_ERROR_none_shared"),
    (1069, "PK_ERROR_cant_hollow"),
    (1070, "PK_ERROR_not_in_same_shell"),
    (1071, "PK_ERROR_general_body"),
    (1072, "PK_ERROR_bad_thickness"),
    (1073, "PK_ERROR_non_smooth_edge"),
    (1074, "PK_ERROR_degenerate_vertex"),
    (1075, "PK_ERROR_cant_thicken"),
    (1076, "PK_ERROR_crossing_face"),
    (1077, "PK_ERROR_not_in_region"),
    (1078, "PK_ERROR_empty_body"),
    (1079, "PK_ERROR_sheet_untrimmed"),
    (1080, "PK_ERROR_fxf_blend_failed"),
    (1081, "PK_ERROR_fxf_blend_bad_token"),
    (1082, "PK_ERROR_file_read_corruption"),
    (1083, "PK_ERROR_trim_loop_degenerate"),
    (1084, "PK_ERROR_solid_has_void"),
    (1085, "PK_ERROR_fru_missing"),
    (1086, "PK_ERROR_not_in_same_partition"),
    (1087, "PK_ERROR_instanced_body"),
    (1088, "PK_ERROR_entity_not_new"),
    (1089, "PK_ERROR_applio_not_registered"),
    (1090, "PK_ERROR_more_than_one_part"),
    (1091, "PK_ERROR_bad_field_conversion"),
    (1092, "PK_ERROR_bad_text_conversion"),
    (1093, "PK_ERROR_not_licensed"),
    (1094, "PK_ERROR_schema_incompatible"),
    (1095, "PK_ERROR_write_memory_full"),
    (1096, "PK_ERROR_compound_body"),
    (1097, "PK_ERROR_cellular_body"),
    (5000, "PK_ERROR_not_implemented"),
    (5001, "PK_ERROR_not_in_PK"),
    (5002, "PK_ERROR_unknown_class"),
    (5003, "PK_ERROR_frustrum_failure"),
    (5004, "PK_ERROR_recursion_depth"),
    (5005, "PK_ERROR_not_a_part"),
    (5006, "PK_ERROR_bad_mark"),
    (5007, "PK_ERROR_mark_ki_started"),
    (5008, "PK_ERROR_mark_not_started"),
    (5009, "PK_ERROR_num_derivs_not_equal"),
    (5010, "PK_ERROR_memory_not_empty"),
    (5011, "PK_ERROR_no_last_error"),
    (5012, "PK_ERROR_zero_bytes_required"),
    (5013, "PK_ERROR_bad_field_number"),
    (5014, "PK_ERROR_field_of_wrong_type"),
    (5015, "PK_ERROR_invalid_group_class"),
    (5016, "PK_ERROR_unhandleable_condition"),
    (5017, "PK_ERROR_wrong_group_class"),
    (5018, "PK_ERROR_eval_failure"),
    (5019, "PK_ERROR_not_a_unit_vector"),
    (5020, "PK_ERROR_existing_attrib"),
    (5021, "PK_ERROR_solid_region"),
    (5022, "PK_ERROR_o_t_version_unknown"),
    (5023, "PK_ERROR_mark_pk_started"),
    (5024, "PK_ERROR_vectors_are_parallel"),
    (5025, "PK_ERROR_sum_of_offsets_eq_0"),
    (5026, "PK_ERROR_distance_too_small"),
    (5027, "PK_ERROR_vectors_not_orthogonal"),
    (5028, "PK_ERROR_zero_interval"),
    (5029, "PK_ERROR_periodic_open"),
    (5030, "PK_ERROR_periodic_not_smooth"),
    (5031, "PK_ERROR_cant_get_point"),
    (5032, "PK_ERROR_cant_get_curve"),
    (5033, "PK_ERROR_edge_didnt_vanish"),
    (5034, "PK_ERROR_face_face_check_fails"),
    (5035, "PK_ERROR_face_check_fails"),
    (5036, "PK_ERROR_not_a_hand"),
    (5037, "PK_ERROR_bad_interval"),
    (5038, "PK_ERROR_bad_uvbox"),
    (5039, "PK_ERROR_cannot_make_current"),
    (5040, "PK_ERROR_bb_not_empty"),
    (5041, "PK_ERROR_bad_class"),
    (5042, "PK_ERROR_bad_class_combn"),
    (5043, "PK_ERROR_o_t_version_incorrect"),
    (5044, "PK_ERROR_bad_boolean_function"),
    (5045, "PK_ERROR_bad_boolean_region"),
    (5046, "PK_ERROR_distancing_failed"),
    (5047, "PK_ERROR_no_overlap"),
    (5048, "PK_ERROR_rollback_not_started"),
    (5049, "PK_ERROR_rollback_started"),
    (5050, "PK_ERROR_cant_get_surf"),
    (5051, "PK_ERROR_bad_section_fence"),
    (5052, "PK_ERROR_bad_boolean_fence"),
    (5053, "PK_ERROR_cant_get_side_curve"),
    (5054, "PK_ERROR_cant_get_side_surf"),
    (5055, "PK_ERROR_sewing_failed"),
    (5056, "PK_ERROR_duplicate_parts"),
    (5057, "PK_ERROR_deltas_not_available"),
    (5058, "PK_ERROR_not_at_pmark"),
    (5059, "PK_ERROR_bad_boolean_match"),
    (5060, "PK_ERROR_invalid_match_region"),
    (5061, "PK_ERROR_check_error"),
    (5062, "PK_ERROR_check_failure"),
    (5063, "PK_ERROR_no_approx_data"),
    (5064, "PK_ERROR_not_general"),
    (5065, "PK_ERROR_cant_merge_regions"),
    (5066, "PK_ERROR_not_a_bb"),
    (5067, "PK_ERROR_bad_bb_status"),
    (5068, "PK_ERROR_unsupported_operation"),
    (5069, "PK_ERROR_bad_end_condition"),
    (5070, "PK_ERROR_bad_boolean_select"),
    (5071, "PK_ERROR_bad_thread"),
    (5072, "PK_ERROR_no_intersect"),
    (5073, "PK_ERROR_obsolete_function"),
    (5074, "PK_ERROR_reverse_edge_failed"),
    (5075, "PK_ERROR_orientation_failed"),
    (5076, "PK_ERROR_bad_edge"),
    (5077, "PK_ERROR_curve_nmnl_off"),
    (5078, "PK_ERROR_bad_pattern_check"),
    (5079, "PK_ERROR_bad_pattern_status"),
    (5080, "PK_ERROR_bad_pattern_result"),
    (5081, "PK_ERROR_wrong_version_delta"),
    (5082, "PK_ERROR_edge_not_open"),
    (5083, "PK_ERROR_edge_not_manifold"),
    (5084, "PK_ERROR_vertex_not_manifold"),
    (5085, "PK_ERROR_no_common_vertex"),
    (5086, "PK_ERROR_fin_not_at_vertex"),
    (5087, "PK_ERROR_bad_fin"),
    (5088, "PK_ERROR_not_laminar_side"),
    (5089, "PK_ERROR_wireframe_edge"),
    (5090, "PK_ERROR_different_shells"),
    (5091, "PK_ERROR_face_contains_loop"),
    (5092, "PK_ERROR_only_one_loop"),
    (5093, "PK_ERROR_invalid_face"),
    (5094, "PK_ERROR_fins_not_distinct"),
    (5095, "PK_ERROR_bad_check_topol"),
    (5096, "PK_ERROR_bad_check_su_X"),
    (5097, "PK_ERROR_bad_change_topol"),
    (5098, "PK_ERROR_bad_local_status"),
    (5099, "PK_ERROR_bad_topol_track"),
    (5100, "PK_ERROR_fin_not_in_loop"),
    (5101, "PK_ERROR_too_many_edges"),
    (5102, "PK_ERROR_loop_not_isolated"),
    (5103, "PK_ERROR_loops_not_same_face"),
    (5104, "PK_ERROR_bad_boolean_report"),
    (5105, "PK_ERROR_bad_boolean_result"),
    (5106, "PK_ERROR_bad_section_report"),
    (5107, "PK_ERROR_bad_section_result"),
    (5108, "PK_ERROR_edge_is_wire"),
    (5109, "PK_ERROR_bad_boolean_check_fa"),
    (5110, "PK_ERROR_bad_section_check_fa"),
    (5111, "PK_ERROR_inconsistent_offset"),
    (5112, "PK_ERROR_inconsistent_thicken"),
    (5113, "PK_ERROR_bad_twist_law"),
    (5114, "PK_ERROR_bad_scale_law"),
    (5115, "PK_ERROR_bad_profile_matching"),
    (5116, "PK_ERROR_bad_end_conditions"),
    (5117, "PK_ERROR_loft_failed"),
    (5118, "PK_ERROR_sweep_failed"),
    (5119, "PK_ERROR_different_loops"),
    (5120, "PK_ERROR_edge_is_ring"),
    (5121, "PK_ERROR_loops_not_distinct"),
    (5122, "PK_ERROR_point_contact"),
    (5123, "PK_ERROR_sweep_need_piecewise"),
    (5124, "PK_ERROR_zero_vector"),
    (5125, "PK_ERROR_bad_iteration_count"),
    (5126, "PK_ERROR_bad_gap_bound"),
    (5127, "PK_ERROR_bad_match_style"),
    (5128, "PK_ERROR_no_division"),
    (5129, "PK_ERROR_no_nth_division"),
    (5130, "PK_ERROR_bad_reference"),
    (5131, "PK_ERROR_cant_fill_hole"),
    (5132, "PK_ERROR_no_data"),
    (5133, "PK_ERROR_too_many_geoms"),
    (5134, "PK_ERROR_disjoint"),
    (5135, "PK_ERROR_duplicate_name"),
    (5136, "PK_ERROR_failed_to_make_outline"),
    (5137, "PK_ERROR_failed_to_blend"),
    (5138, "PK_ERROR_failed_to_offset"),
    (5139, "PK_ERROR_failed_to_taper"),
    (5140, "PK_ERROR_failed_to_transform"),
    (5141, "PK_ERROR_journalling_on"),
    (5142, "PK_ERROR_dbg_rprt_not_stopped"),
    (5143, "PK_ERROR_dbg_rprt_not_started"),
    (5144, "PK_ERROR_imprint_shadows_failed"),
    (5145, "PK_ERROR_bad_combination"),
    (5146, "PK_ERROR_partition_is_current"),
    (5147, "PK_ERROR_partition_not_empty"),
    (5148, "PK_ERROR_failed_to_bend"),
    (5149, "PK_ERROR_cant_use_curve"),
    (5150, "PK_ERROR_cant_extend"),
    (5151, "PK_ERROR_bad_boolean_material"),
    (5152, "PK_ERROR_cant_complete_imprint"),
    (5153, "PK_ERROR_edge_closure_mismatch"),
    (5154, "PK_ERROR_bad_boolean_prefer"),
    (5155, "PK_ERROR_not_started"),
    (5156, "PK_ERROR_not_stopped"),
    (5157, "PK_ERROR_bad_output_session"),
    (5158, "PK_ERROR_bad_boolean_no_effect"),
    (5159, "PK_ERROR_null_evaluator"),
    (5160, "PK_ERROR_ambiguous_imprint"),
    (5161, "PK_ERROR_bad_match_update"),
    (5162, "PK_ERROR_cant_limit_faces"),
    (5163, "PK_ERROR_bad_partition"),
    (5164, "PK_ERROR_edge_too_short"),
    (5165, "PK_ERROR_loop_is_sliver"),
    (5166, "PK_ERROR_cyclic_group"),
    (5167, "PK_ERROR_find_self_int_failed"),
    (5168, "PK_ERROR_indexio_not_registered"),
    (5169, "PK_ERROR_xt_data_not_indexed"),
    (5170, "PK_ERROR_bad_report"),
    (5171, "PK_ERROR_wrong_report"),
    (5172, "PK_ERROR_fix_self_int_failed"),
    (5173, "PK_ERROR_wrong_number_entities"),
    (5174, "PK_ERROR_clashing_limits"),
    (5175, "PK_ERROR_closed_report"),
    (5176, "PK_ERROR_bad_cap_definition"),
    (5177, "PK_ERROR_mutual_dependency"),
    (5178, "PK_ERROR_artificial_error"),
    (5179, "PK_ERROR_failed_to_deform"),
    (5180, "PK_ERROR_evaluator_failed"),
    (5181, "PK_ERROR_su_not_coincident"),
    (5182, "PK_ERROR_tool_generation"),
    (5183, "PK_ERROR_unsupported_transf"),
    (5184, "PK_ERROR_bad_2d_viewport"),
    (5185, "PK_ERROR_find_degens_failed"),
    (5186, "PK_ERROR_fix_degens_failed"),
    (5187, "PK_ERROR_bad_orientation"),
    (5188, "PK_ERROR_laminar_edge"),
    (5189, "PK_ERROR_copy_failed"),
    (5190, "PK_ERROR_find_interior_failed"),
    (5191, "PK_ERROR_failed_to_facet"),
    (5192, "PK_ERROR_cant_get_vectors"),
    (5193, "PK_ERROR_bad_chord"),
    (5194, "PK_ERROR_interval_exceed_period"),
    (5195, "PK_ERROR_interval_le_0"),
    (5196, "PK_ERROR_no_measurements"),
    (5197, "PK_ERROR_failed_to_change"),
    (5198, "PK_ERROR_invalid_methods"),
    (5199, "PK_ERROR_cant_find_thickness"),
    (5200, "PK_ERROR_zero_thickness"),
    (5201, "PK_ERROR_cant_make_valid_split"),
    (5202, "PK_ERROR_same_edge"),
    (5203, "PK_ERROR_density_lt_0"),
    (5204, "PK_ERROR_cant_cover"),
    (5205, "PK_ERROR_bad_faces"),
    (5206, "PK_ERROR_bad_acorn"),
    (5207, "PK_ERROR_child_body"),
    (5208, "PK_ERROR_grid_not_on_boundary"),
    (5209, "PK_ERROR_grid_disjoint"),
    (5210, "PK_ERROR_grid_not_wireframe"),
    (5211, "PK_ERROR_grid_not_smooth"),
    (5212, "PK_ERROR_not_smooth"),
    (5213, "PK_ERROR_ambiguous_selector"),
    (5214, "PK_ERROR_inconsistent_selection"),
    (5215, "PK_ERROR_callback_failed"),
    (5216, "PK_ERROR_illegal_cliff"),
    (5217, "PK_ERROR_bad_blend_rho_type"),
    (5218, "PK_ERROR_failed_to_project"),
    (5219, "PK_ERROR_all_edges_in_body"),
    (5220, "PK_ERROR_not_edge_on"),
    (5221, "PK_ERROR_function_not_conc"),
    (5222, "PK_ERROR_function_not_exclusive"),
    (5223, "PK_ERROR_function_not_mutable"),
    (5224, "PK_ERROR_cant_find_uvbox"),
    (5225, "PK_ERROR_reparam_failed"),
    (5226, "PK_ERROR_bad_mfacet"),
    (5227, "PK_ERROR_bad_mfin"),
    (5228, "PK_ERROR_bad_mvertex"),
    (5229, "PK_ERROR_bad_mfin_index"),
    (5230, "PK_ERROR_mtopols_not_same_mesh"),
    (5231, "PK_ERROR_wrong_mtopol"),
    (5232, "PK_ERROR_facet_geometry_cant"),
    (5233, "PK_ERROR_failed_to_bound"),
    (5234, "PK_ERROR_not_on_mfacet"),
    (5235, "PK_ERROR_versioning_mixed"),
    (5236, "PK_ERROR_versioning_clash"),
    (5237, "PK_ERROR_facet_geometry"),
    (5238, "PK_ERROR_mesh_has_no_mfacets"),
    (5239, "PK_ERROR_mesh_has_no_mvertices"),
    (5240, "PK_ERROR_index_out_of_range"),
    (5241, "PK_ERROR_facet_invalid_input"),
    (5242, "PK_ERROR_mesh_not_found"),
    (5243, "PK_ERROR_inconsistent_data"),
    (5244, "PK_ERROR_defect_error"),
    (5245, "PK_ERROR_defect_failure"),
    (5246, "PK_ERROR_cant_project_mesh"),
    (5247, "PK_ERROR_cant_parameterise_mesh"),
    (5248, "PK_ERROR_bad_mvx_normals"),
    (5249, "PK_ERROR_tolerance_too_loose"),
    (5250, "PK_ERROR_attrib_field_empty"),
    (5251, "PK_ERROR_mesh_has_no_mfins"),
    (5252, "PK_ERROR_cant_attach_patch"),
    (5253, "PK_ERROR_mesh_open_components"),
    (5254, "PK_ERROR_closed_group"),
    (5255, "PK_ERROR_mass_failure"),
    (5256, "PK_ERROR_classic_geometry"),
    (5257, "PK_ERROR_failed_to_radiate"),
    (5258, "PK_ERROR_inconsistent_tolerance"),
    (5259, "PK_ERROR_bad_item"),
    (5260, "PK_ERROR_inconsistent_senses"),
    (5261, "PK_ERROR_mixed_geometry"),
    (5262, "PK_ERROR_no_topols_from_target"),
    (5263, "PK_ERROR_bad_mesh_box"),
    (5264, "PK_ERROR_mesh_not_created"),
    (5265, "PK_ERROR_reset_failed"),
    (5266, "PK_ERROR_locked"),
    (5267, "PK_ERROR_not_locked"),
    (5268, "PK_ERROR_lattice_has_no_lrods"),
    (5269, "PK_ERROR_lattice_has_no_lballs"),
    (5270, "PK_ERROR_wrong_ltopol"),
    (5271, "PK_ERROR_bad_lball"),
    (5272, "PK_ERROR_bad_lrod"),
    (5273, "PK_ERROR_cant_fix_defects"),
    (5274, "PK_ERROR_cant_make_body"),
    (5275, "PK_ERROR_clip_failure"),
    (5276, "PK_ERROR_already_embedded"),
    (5277, "PK_ERROR_lattice_geometry"),
    (5278, "PK_ERROR_pattern_failed"),
    (5279, "PK_ERROR_vectors_coplanar"),
    (5280, "PK_ERROR_partial_pmark"),
    (5281, "PK_ERROR_too_many_facets"),
    (5282, "PK_ERROR_basis_not_right_handed"),
    (5283, "PK_ERROR_bad_expr"),
    (5284, "PK_ERROR_expr_undef_symb"),
    (5285, "PK_ERROR_expr_duplicate_def"),
    (5286, "PK_ERROR_outside_box"),
    (5287, "PK_ERROR_no_guard"),
    (5288, "PK_ERROR_insufficient_lattices"),
    (5289, "PK_ERROR_size_too_large"),
    (5290, "PK_ERROR_acorn_body"),
    (5291, "PK_ERROR_hidden_by_guise"),
    (5292, "PK_ERROR_cant_embed_lattice"),
    (5293, "PK_ERROR_bad_trim_curves"),
    (5294, "PK_ERROR_bad_form"),
    (5295, "PK_ERROR_bad_loop"),
    (5296, "PK_ERROR_bad_loop_config"),
    (5297, "PK_ERROR_core_breach"),
    (5298, "PK_ERROR_no_lattice"),
    (5299, "PK_ERROR_guise_not_supported"),
    (5300, "PK_ERROR_no_body"),
    (5301, "PK_ERROR_non_cellular"),
    (5302, "PK_ERROR_slice_failure"),
    (5303, "PK_ERROR_out_of_range"),
    (5304, "PK_ERROR_find_outer_failed"),
    (5305, "PK_ERROR_failed_to_embed"),
    (5306, "PK_ERROR_lattice_bad_type"),
    (5307, "PK_ERROR_bad_ijkbox"),
    (5308, "PK_ERROR_void_region"),
    (5309, "PK_ERROR_infinite_region"),
];

fn table_name(code: i32) -> Option<&'static str> {
    TABLE.iter().find(|(c, _)| *c == code).map(|(_, n)| *n)
}

fn inline_str(buf: &[u8], off: usize, len: usize) -> String {
    let s = &buf[off..off + len];
    let end = s.iter().position(|&b| b == 0).unwrap_or(len);
    String::from_utf8_lossy(&s[..end]).into_owned()
}

fn i32_at(buf: &[u8], off: usize) -> i32 {
    i32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

#[derive(Debug)]
struct Rec {
    rc: i32,
    was_error: bool,
    function: String,
    code: i32,
    code_token: String,
    severity: i32,
    argument_number: i32,
    argument_name: String,
    argument_index: i32,
    entity: i32,
    tail_written: Vec<usize>,
}

/// Read the record into a POISON-filled buffer so a *zero* write past 116 is
/// still detectable (the repo's own probe zero-filled, so it could only see
/// nonzero writes).
fn read_rec(rc: i32) -> Option<Rec> {
    let mut buf = [POISON; BUF];
    let mut was_error: PK_LOGICAL_t = PK_LOGICAL_false;
    let ask = unsafe { PK_ERROR_ask_last(&mut was_error, buf.as_mut_ptr() as *mut PK_ERROR_sf_t) };
    if ask != PK_ERROR_no_errors {
        println!("    !! PK_ERROR_ask_last itself returned {ask}");
        return None;
    }
    let tail_written: Vec<usize> = (116..BUF).filter(|&i| buf[i] != POISON).collect();
    Some(Rec {
        rc,
        was_error: was_error == PK_LOGICAL_true,
        function: inline_str(&buf, 0, 32),
        code: i32_at(&buf, 32),
        code_token: inline_str(&buf, 36, 32),
        severity: i32_at(&buf, 68),
        argument_number: i32_at(&buf, 72),
        argument_name: inline_str(&buf, 76, 32),
        argument_index: i32_at(&buf, 108),
        entity: i32_at(&buf, 112),
        tail_written,
    })
}

struct Stats {
    cases: usize,
    token_mismatch: Vec<String>,
    rc_vs_code_mismatch: Vec<String>,
    fn_mismatch: Vec<String>,
    tail_writes: Vec<String>,
    severities: std::collections::BTreeMap<i32, usize>,
    no_record: Vec<String>,
}

fn clear() {
    let mut cleared: PK_LOGICAL_t = PK_LOGICAL_false;
    unsafe { PK_ERROR_clear_last(&mut cleared) };
}

/// Run one real trigger and check every claim against it.
fn case(st: &mut Stats, label: &str, expect_fn: &str, trigger: impl FnOnce() -> PK_ERROR_code_t) {
    clear();
    let rc = trigger();
    st.cases += 1;
    println!("\n--- {label}");
    if rc == PK_ERROR_no_errors {
        println!("    rc = 0 (NO ERROR RAISED — trigger did not fail)");
        return;
    }
    let Some(r) = read_rec(rc) else {
        st.no_record.push(label.to_string());
        return;
    };
    if !r.was_error {
        println!("    rc = {rc} but was_error = false  <-- NO RECORD FOR A FAILING CALL");
        st.no_record.push(format!("{label} (rc={rc})"));
        return;
    }
    let expect_tok = table_name(r.code);
    println!(
        "    rc={rc} code={} token={:?} sev={} fn={:?} arg#{} name={:?} idx={} entity={}",
        r.code, r.code_token, r.severity, r.function, r.argument_number, r.argument_name,
        r.argument_index, r.entity
    );
    match expect_tok {
        Some(t) if t == r.code_token => println!("    table: OK ({t} = {})", r.code),
        Some(t) => {
            let m = format!("code {} kernel token {:?} but error_codes.rs says {:?}", r.code, r.code_token, t);
            println!("    table: *** MISMATCH *** {m}");
            st.token_mismatch.push(format!("{label}: {m}"));
        }
        None => {
            let m = format!("code {} ({:?}) NOT IN error_codes.rs", r.code, r.code_token);
            println!("    table: *** MISSING *** {m}");
            st.token_mismatch.push(format!("{label}: {m}"));
        }
    }
    if r.rc != r.code {
        let m = format!("return code {} != record code {}", r.rc, r.code);
        println!("    *** rc/code DISAGREE *** {m}  -> query_last_error would DROP this record");
        st.rc_vs_code_mismatch.push(format!("{label}: {m}"));
    }
    if !expect_fn.is_empty() && r.function != expect_fn {
        let m = format!("record function {:?} != called {:?}", r.function, expect_fn);
        println!("    *** function mismatch *** {m}");
        st.fn_mismatch.push(format!("{label}: {m}"));
    }
    if !r.tail_written.is_empty() {
        let m = format!("bytes written past 116: {:?}", &r.tail_written[..r.tail_written.len().min(24)]);
        println!("    *** LAYOUT *** {m}");
        st.tail_writes.push(format!("{label}: {m}"));
    }
    *st.severities.entry(r.severity).or_insert(0) += 1;
}

fn tag_of(b: &Body) -> i32 { b.tag() }

fn main() {
    let mut out = std::io::stdout();
    let _session = Session::start(SessionConfig::new()).expect("session");
    unsafe { PK_SESSION_set_check_arguments(PK_LOGICAL_true) };

    let mut st = Stats {
        cases: 0,
        token_mismatch: Vec::new(),
        rc_vs_code_mismatch: Vec::new(),
        fn_mismatch: Vec::new(),
        tail_writes: Vec::new(),
        severities: Default::default(),
        no_record: Vec::new(),
    };

    println!("== error_codes.rs table has {} entries ==", TABLE.len());

    // ---------------------------------------------------------------- arg errors
    case(&mut st, "block with negative x", "PK_BODY_create_solid_block", || {
        let mut b: PK_BODY_t = 0;
        unsafe { PK_BODY_create_solid_block(-1.0, 1.0, 1.0, std::ptr::null(), &mut b) }
    });
    case(&mut st, "block with zero z", "PK_BODY_create_solid_block", || {
        let mut b: PK_BODY_t = 0;
        unsafe { PK_BODY_create_solid_block(1.0, 1.0, 0.0, std::ptr::null(), &mut b) }
    });
    case(&mut st, "ask class of bogus tag", "PK_ENTITY_ask_class", || {
        let mut c: PK_CLASS_t = -1;
        unsafe { PK_ENTITY_ask_class(999_999, &mut c) }
    });
    case(&mut st, "ask faces of bogus tag", "PK_BODY_ask_faces", || {
        let mut n = 0;
        let mut f: *mut PK_FACE_t = std::ptr::null_mut();
        unsafe { PK_BODY_ask_faces(999_999, &mut n, &mut f) }
    });
    case(&mut st, "sphere radius 0", "PK_SPHERE_create", || {
        let sf = PK_SPHERE_sf_t {
            basis_set: PK_AXIS2_sf_t { location: [0.0; 3], axis: [0.0, 0.0, 1.0], ref_direction: [1.0, 0.0, 0.0] },
            radius: 0.0,
        };
        let mut s: PK_SPHERE_t = 0;
        unsafe { PK_SPHERE_create(&sf, &mut s) }
    });
    case(&mut st, "sphere radius negative", "PK_SPHERE_create", || {
        let sf = PK_SPHERE_sf_t {
            basis_set: PK_AXIS2_sf_t { location: [0.0; 3], axis: [0.0, 0.0, 1.0], ref_direction: [1.0, 0.0, 0.0] },
            radius: -3.0,
        };
        let mut s: PK_SPHERE_t = 0;
        unsafe { PK_SPHERE_create(&sf, &mut s) }
    });
    case(&mut st, "sphere zero axis", "PK_SPHERE_create", || {
        let sf = PK_SPHERE_sf_t {
            basis_set: PK_AXIS2_sf_t { location: [0.0; 3], axis: [0.0; 3], ref_direction: [1.0, 0.0, 0.0] },
            radius: 1.0,
        };
        let mut s: PK_SPHERE_t = 0;
        unsafe { PK_SPHERE_create(&sf, &mut s) }
    });
    case(&mut st, "sphere non-orthogonal ref_direction", "PK_SPHERE_create", || {
        let sf = PK_SPHERE_sf_t {
            basis_set: PK_AXIS2_sf_t { location: [0.0; 3], axis: [0.0, 0.0, 1.0], ref_direction: [0.0, 0.0, 1.0] },
            radius: 1.0,
        };
        let mut s: PK_SPHERE_t = 0;
        unsafe { PK_SPHERE_create(&sf, &mut s) }
    });
    case(&mut st, "rotation about non-unit axis", "PK_TRANSF_create_rotation", || {
        let p: PK_VECTOR_t = [0.0; 3];
        let d: PK_VECTOR_t = [1.0, 1.0, 1.0];
        let mut t: PK_TRANSF_t = 0;
        unsafe { PK_TRANSF_create_rotation(&p, &d, 0.5, &mut t) }
    });
    case(&mut st, "rotation about zero axis", "PK_TRANSF_create_rotation", || {
        let p: PK_VECTOR_t = [0.0; 3];
        let d: PK_VECTOR_t = [0.0; 3];
        let mut t: PK_TRANSF_t = 0;
        unsafe { PK_TRANSF_create_rotation(&p, &d, 0.5, &mut t) }
    });
    case(&mut st, "negative session precision", "PK_SESSION_set_precision", || unsafe {
        PK_SESSION_set_precision(-1.0)
    });
    case(&mut st, "session precision 0", "PK_SESSION_set_precision", || unsafe {
        PK_SESSION_set_precision(0.0)
    });
    case(&mut st, "curve eval on non-curve tag", "PK_CURVE_eval", || {
        let mut pos = [0.0f64; 12];
        unsafe { PK_CURVE_eval(999_999, 0.0, 1, pos.as_mut_ptr()) }
    });
    case(&mut st, "entity_delete of bogus tag", "PK_ENTITY_delete", || {
        let t: PK_ENTITY_t = 999_998;
        unsafe { PK_ENTITY_delete(1, &t) }
    });
    case(&mut st, "PART_receive with nonexistent key", "PK_PART_receive", || {
        let key = std::ffi::CString::new("no_such_key_xyzzy").unwrap();
        let mut o = PK_PART_receive_o_t::default();
        o.transmit_format = PK_transmit_format_text_c;
        let mut n = 0;
        let mut p: *mut PK_PART_t = std::ptr::null_mut();
        unsafe { PK_PART_receive(key.as_ptr(), &o, &mut n, &mut p) }
    });

    // ---------------------------------------------------------------- option-struct errors
    case(&mut st, "mass props o_t_version = 99", "PK_TOPOL_eval_mass_props", || {
        let mut body: PK_BODY_t = 0;
        let rc = unsafe { PK_BODY_create_solid_block(1.0, 1.0, 1.0, std::ptr::null(), &mut body) };
        assert_eq!(rc, 0);
        #[repr(C)]
        struct MassOpts { v: i32, mass: i32, periphery: i32, bound: i32, single: u8 }
        let o = MassOpts { v: 99, mass: 0x36b4, periphery: 0x36b6, bound: 0x36b7, single: 1 };
        let (mut a, mut m, mut p) = (0.0f64, 0.0f64, 0.0f64);
        let mut cg = [0.0f64; 3];
        let mut mi = [0.0f64; 9];
        unsafe {
            PK_TOPOL_eval_mass_props(1, &body, 0.99,
                &o as *const MassOpts as *const PK_TOPOL_eval_mass_props_o_t,
                &mut a, &mut m, &mut cg, &mut mi, &mut p)
        }
    });

    // ---------------------------------------------------------------- topology / geometry failures
    let blk = Body::create_solid_block(10.0, 10.0, 10.0).expect("block");
    let blk_tag = tag_of(&blk);
    let faces = blk.faces().expect("faces");
    let edges = blk.edges().expect("edges");
    let face_tag = faces[0].tag();
    let edge_tag = edges[0].tag();

    case(&mut st, "delete_acorn on a non-acorn vertex", "PK_VERTEX_delete_acorn", || {
        let vs = blk.vertices().expect("verts");
        let v = vs[0].tag();
        unsafe { PK_VERTEX_delete_acorn(1, &v) }
    });
    case(&mut st, "ask faces of a FACE tag (wrong class)", "PK_BODY_ask_faces", || {
        let mut n = 0;
        let mut f: *mut PK_FACE_t = std::ptr::null_mut();
        unsafe { PK_BODY_ask_faces(face_tag, &mut n, &mut f) }
    });
    case(&mut st, "curve eval on a FACE tag (wrong class)", "PK_CURVE_eval", || {
        let mut pos = [0.0f64; 12];
        unsafe { PK_CURVE_eval(face_tag, 0.0, 1, pos.as_mut_ptr()) }
    });
    case(&mut st, "hollow with wall thicker than the block", "PK_BODY_hollow_2", || {
        let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut rs: PK_TOPOL_local_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_BODY_hollow_2(blk_tag, -20.0, 1.0e-6, std::ptr::null(), &mut tr, &mut rs) }
    });
    case(&mut st, "offset that collapses the body", "PK_BODY_offset_2", || {
        let mut o: PK_BODY_offset_o_t = Default::default();
        let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut rs: PK_TOPOL_local_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_BODY_offset_2(blk_tag, -50.0, 1.0e-6, &mut o, &mut tr, &mut rs) }
    });
    case(&mut st, "offset with negative tolerance", "PK_BODY_offset_2", || {
        let mut o: PK_BODY_offset_o_t = Default::default();
        let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut rs: PK_TOPOL_local_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_BODY_offset_2(blk_tag, 1.0, -1.0, &mut o, &mut tr, &mut rs) }
    });
    case(&mut st, "blend radius far larger than the body", "PK_EDGE_set_blend_constant", || {
        let mut n = 0;
        let mut be: *mut PK_EDGE_t = std::ptr::null_mut();
        unsafe { PK_EDGE_set_blend_constant(1, &edge_tag, 1000.0, std::ptr::null(), &mut n, &mut be) }
    });
    case(&mut st, "blend radius 0", "PK_EDGE_set_blend_constant", || {
        let mut n = 0;
        let mut be: *mut PK_EDGE_t = std::ptr::null_mut();
        unsafe { PK_EDGE_set_blend_constant(1, &edge_tag, 0.0, std::ptr::null(), &mut n, &mut be) }
    });
    case(&mut st, "blend on a FACE tag", "PK_EDGE_set_blend_constant", || {
        let mut n = 0;
        let mut be: *mut PK_EDGE_t = std::ptr::null_mut();
        unsafe { PK_EDGE_set_blend_constant(1, &face_tag, 1.0, std::ptr::null(), &mut n, &mut be) }
    });
    case(&mut st, "delete every face of a solid", "PK_FACE_delete_2", || {
        let ft: Vec<PK_FACE_t> = faces.iter().map(|f| f.tag()).collect();
        let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_FACE_delete_2(ft.len() as i32, ft.as_ptr(), std::ptr::null(), &mut tr) }
    });

    // boolean with the same body as target and tool
    case(&mut st, "boolean unite body with itself", "PK_BODY_boolean_2", || {
        let mut o: PK_BODY_boolean_o_t = Default::default();
        o.function = 15903;
        let mut r: PK_boolean_r_t = unsafe { std::mem::zeroed() };
        let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_BODY_boolean_2(blk_tag, 1, &blk_tag, &mut o, &mut tr, &mut r) }
    });
    // boolean with an o_t_version the kernel rejects
    case(&mut st, "boolean with o_t_version = 1", "PK_BODY_boolean_2", || {
        let other = Body::create_solid_block(2.0, 2.0, 2.0).expect("b2");
        let ot = other.tag();
        let mut o: PK_BODY_boolean_o_t = Default::default();
        o.o_t_version = 1;
        o.function = 15903;
        let mut r: PK_boolean_r_t = unsafe { std::mem::zeroed() };
        let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        std::mem::forget(other);
        unsafe { PK_BODY_boolean_2(blk_tag, 1, &ot, &mut o, &mut tr, &mut r) }
    });

    let _ = out.flush();

    // ---------------------------------------------------------------- corrupt-file receive (serious?)
    println!("\n=== corrupt transmit file -> receive (hunting a serious/fatal severity)");
    {
        let dir = std::env::temp_dir();
        let key = "review_probe_corrupt";
        // Fresh session-scoped frustrum base dir is whatever the default is; write next to cwd.
        let b = Body::create_solid_block(1.0, 2.0, 3.0).expect("blk");
        match parasolid::fileio::transmit(std::slice::from_ref(&b), key) {
            Ok(()) => {
                // find the file the frustrum produced
                let mut found = None;
                for cand in [format!("{key}.x_t"), format!("{key}.xmt_txt"), format!("{key}")] {
                    if std::path::Path::new(&cand).exists() { found = Some(cand); break; }
                }
                println!("  transmitted, file = {found:?} (tmp={})", dir.display());
                if let Some(p) = found {
                    let mut data = std::fs::read(&p).unwrap();
                    // Corrupt the middle of the body data, past the header.
                    let n = data.len();
                    for i in (n / 2)..(n / 2 + 64).min(n) { data[i] = b'Z'; }
                    std::fs::write(&p, &data).unwrap();
                    case(&mut st, "receive a corrupted XT file", "", || {
                        let ck = std::ffi::CString::new(key).unwrap();
                        let mut o = PK_PART_receive_o_t::default();
                        o.transmit_format = PK_transmit_format_text_c;
                        let mut n = 0;
                        let mut pp: *mut PK_PART_t = std::ptr::null_mut();
                        unsafe { PK_PART_receive(ck.as_ptr(), &o, &mut n, &mut pp) }
                    });
                    // truncate hard and retry
                    std::fs::write(&p, &data[..(data.len() / 3)]).unwrap();
                    case(&mut st, "receive a truncated XT file", "", || {
                        let ck = std::ffi::CString::new(key).unwrap();
                        let mut o = PK_PART_receive_o_t::default();
                        o.transmit_format = PK_transmit_format_text_c;
                        let mut n = 0;
                        let mut pp: *mut PK_PART_t = std::ptr::null_mut();
                        unsafe { PK_PART_receive(ck.as_ptr(), &o, &mut n, &mut pp) }
                    });
                    let _ = std::fs::remove_file(&p);
                }
            }
            Err(e) => println!("  transmit failed: {e}"),
        }
    }
    let _ = out.flush();

    // ---------------------------------------------------------------- partition / session errors
    case(&mut st, "delete the current partition", "PK_PARTITION_delete", || {
        let mut part: PK_PARTITION_t = 0;
        unsafe { PK_SESSION_ask_curr_partition(&mut part) };
        unsafe { PK_PARTITION_delete(part, std::ptr::null()) }
    });

    // ---------------------------------------------------------------- CLAIM 3: severity field
    println!("\n=== CLAIM 3 — severity at offset 68 for values other than 1");
    for sev in [1i32, 2, 3] {
        clear();
        let mut sf = [0u8; 116];
        sf[32..36].copy_from_slice(&15i32.to_le_bytes());
        sf[68..72].copy_from_slice(&sev.to_le_bytes());
        sf[108..112].copy_from_slice(&(-1i32).to_le_bytes());
        unsafe { PK_ERROR_raise(sf.as_ptr() as *const PK_ERROR_sf_t) };
        if let Some(r) = read_rec(0) {
            println!("  raised severity {sev} -> readback severity {} token {:?} (echo only, NOT a real error)",
                     r.severity, r.code_token);
        }
    }

    // ---------------------------------------------------------------- CLAIM 4: staleness guard
    println!("\n=== CLAIM 4 — record staleness / rc-vs-code guard");
    clear();
    let mut b: PK_BODY_t = 0;
    let rc1 = unsafe { PK_BODY_create_solid_block(-1.0, 1.0, 1.0, std::ptr::null(), &mut b) };
    let r1 = read_rec(rc1).unwrap();
    println!("  after failing call:  rc={rc1} was_error={} fn={:?} code={}", r1.was_error, r1.function, r1.code);
    let rc2 = unsafe { PK_BODY_create_solid_block(1.0, 1.0, 1.0, std::ptr::null(), &mut b) };
    let r2 = read_rec(rc2).unwrap();
    println!("  after SUCCESSFUL call: rc={rc2} was_error={} fn={:?} code={}  <-- record survives success",
             r2.was_error, r2.function, r2.code);

    // Same-code-different-function hazard: two different functions both fail with the
    // same code. If the second one does NOT refresh the record, query_last_error
    // accepts a stale record from the first because the codes agree.
    clear();
    let mut cl: PK_CLASS_t = 0;
    let rc_a = unsafe { PK_ENTITY_ask_class(999_999, &mut cl) };
    let ra = read_rec(rc_a).unwrap();
    let mut nn = 0;
    let mut ff: *mut PK_FACE_t = std::ptr::null_mut();
    let rc_b = unsafe { PK_BODY_ask_faces(999_998, &mut nn, &mut ff) };
    let rb = read_rec(rc_b).unwrap();
    println!("  A: PK_ENTITY_ask_class rc={rc_a} rec.fn={:?} rec.code={} entity={}", ra.function, ra.code, ra.entity);
    println!("  B: PK_BODY_ask_faces   rc={rc_b} rec.fn={:?} rec.code={} entity={}", rb.function, rb.code, rb.entity);
    if rc_a == rc_b && rb.function == ra.function {
        println!("  *** STALE-ACCEPT: B did not refresh the record and the codes agree, so the guard passes a record belonging to A");
    }

    // PK_THREAD_ask_last_error cross-check
    println!("\n=== PK_THREAD_ask_last_error vs PK_ERROR_ask_last");
    clear();
    let rcx = unsafe { PK_BODY_create_solid_block(-2.0, 1.0, 1.0, std::ptr::null(), &mut b) };
    let mut tbuf = [POISON; BUF];
    let mut twas: PK_LOGICAL_t = PK_LOGICAL_false;
    let trc = unsafe { PK_THREAD_ask_last_error(&mut twas, tbuf.as_mut_ptr() as *mut PK_ERROR_sf_t) };
    println!("  trigger rc={rcx}; THREAD rc={trc} was_error={twas} fn={:?} code={} token={:?} sev={}",
             inline_str(&tbuf, 0, 32), i32_at(&tbuf, 32), inline_str(&tbuf, 36, 32), i32_at(&tbuf, 68));
    let ttail: Vec<usize> = (116..BUF).filter(|&i| tbuf[i] != POISON).collect();
    println!("  THREAD bytes past 116 written: {:?}", &ttail[..ttail.len().min(16)]);


    // ------------------------------------------------- SEVERITY BATTERY (isolated bodies)
    println!("\n=== SEVERITY BATTERY — every case gets its own fresh body");
    let mut sev_hits: Vec<(String, i32, i32, String)> = Vec::new();
    {
        let mut run = |st: &mut Stats, label: &str, f: &mut dyn FnMut() -> (i32, i32)| {
            clear();
            let (_body, rc) = { let (t, rc) = f(); (t, rc) };
            st.cases += 1;
            if rc == 0 { println!("  {label}: rc=0 (no failure)"); return; }
            if let Some(r) = read_rec(rc) {
                println!("  {label}: rc={rc} code={} token={:?} SEV={} fn={:?}", r.code, r.code_token, r.severity, r.function);
                if table_name(r.code) != Some(r.code_token.as_str()) {
                    st.token_mismatch.push(format!("{label}: code {} kernel {:?} table {:?}", r.code, r.code_token, table_name(r.code)));
                }
                if r.rc != r.code { st.rc_vs_code_mismatch.push(format!("{label}: rc={} code={}", r.rc, r.code)); }
                *st.severities.entry(r.severity).or_insert(0) += 1;
                sev_hits.push((label.to_string(), r.severity, r.code, r.code_token.clone()));
            }
        };

        for (label, thick) in [("hollow -20 on 10-cube", -20.0f64), ("hollow -5.0 on 10-cube", -5.0), ("hollow -4.999", -4.999), ("hollow +20 outward", 20.0)] {
            let b = Body::create_solid_block(10.0, 10.0, 10.0).expect("blk");
            let t = b.tag();
            std::mem::forget(b);
            run(&mut st, label, &mut || {
                let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
                let mut buf = [0u8; 128];
                (t, unsafe { PK_BODY_hollow_2(t, thick, 1.0e-6, std::ptr::null(), &mut tr, buf.as_mut_ptr() as *mut PK_TOPOL_local_r_t) })
            });
        }
        for (label, off) in [("PK_BODY_offset(v1) -50", -50.0f64), ("PK_BODY_offset(v1) -5.0", -5.0), ("PK_BODY_offset(v1) -4.9", -4.9)] {
            let b = Body::create_solid_block(10.0, 10.0, 10.0).expect("blk");
            let t = b.tag();
            std::mem::forget(b);
            run(&mut st, label, &mut || (t, unsafe { PK_BODY_offset(t, off, 1.0e-6, PK_LOGICAL_false) }));
        }
        // sheet body thicken failures
        for (label, th) in [("thicken_3 huge on sheet", 1.0e6f64), ("thicken_3 zero", 0.0)] {
            let b = Body::create_sheet_rectangle(10.0, 10.0, Axis2::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,1.0), Vec3::new(1.0,0.0,0.0))).or_else(|_| Body::create_solid_block(10.0,10.0,10.0)).expect("sheet");
            let t = b.tag();
            std::mem::forget(b);
            run(&mut st, label, &mut || {
                let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
                let mut buf = [0u8; 128];
                (t, unsafe { PK_BODY_thicken_3(t, th, th, 1.0e-6, std::ptr::null_mut(), &mut tr, buf.as_mut_ptr() as *mut PK_TOPOL_local_r_t) })
            });
        }
        // imprint a body with itself / disjoint
        {
            let a = Body::create_solid_block(10.0, 10.0, 10.0).expect("a");
            let ta = a.tag(); std::mem::forget(a);
            println!("  imprint body with itself: SKIPPED — PK_BODY_imprint_body(t,t,..) page-faults inside the kernel (observed twice, with NULL and with default options)");
            let _ = ta;
        }
        // boolean subtract producing an empty result then reuse of the dead tag
        {
            let a = Body::create_solid_block(4.0, 4.0, 4.0).expect("a");
            let ta = a.tag(); std::mem::forget(a);
            let bb = Body::create_solid_block(10.0, 10.0, 10.0).expect("b");
            let tb = bb.tag(); std::mem::forget(bb);
            run(&mut st, "subtract bigger from smaller (empty result)", &mut || {
                let mut o: PK_BODY_boolean_o_t = Default::default();
                o.function = 15902;
                let mut r: PK_boolean_r_t = unsafe { std::mem::zeroed() };
                let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
                (ta, unsafe { PK_BODY_boolean_2(ta, 1, &tb, &mut o, &mut tr, &mut r) })
            });
            run(&mut st, "use the consumed target tag afterwards", &mut || {
                let mut n = 0; let mut f: *mut PK_FACE_t = std::ptr::null_mut();
                (ta, unsafe { PK_BODY_ask_faces(ta, &mut n, &mut f) })
            });
        }
        // delete a face from a solid via the sheet-only entry point
        {
            let a = Body::create_solid_block(6.0, 6.0, 6.0).expect("a");
            let fs = a.faces().expect("f");
            let ft = fs[0].tag();
            let ta = a.tag(); std::mem::forget(a);
            run(&mut st, "PK_FACE_delete_from_sheet_body on a solid face", &mut || (ta, unsafe { PK_FACE_delete_from_sheet_body(ft) }));
        }
        // taper with an absurd angle
        {
            let a = Body::create_solid_block(6.0, 6.0, 6.0).expect("a");
            let es = a.edges().expect("e");
            let et = es[0].tag();
            let ta = a.tag(); std::mem::forget(a);
            run(&mut st, "taper 89.9 degrees", &mut || {
                let dir = [0.0f64, 0.0, 1.0];
                let mut status: PK_local_status_t = 0;
                let mut n_err = 0; let mut errs: *mut PK_TOPOL_t = std::ptr::null_mut();
                (ta, unsafe { PK_BODY_taper(ta, 0, 1, &et, 0, std::ptr::null(), 1.5697, 0.0, dir.as_ptr(), std::ptr::null(), &mut status, &mut n_err, &mut errs) })
            });
        }
    }
    // After a SERIOUS error the model may be corrupt. Keep operating on it and see
    // whether the kernel escalates to fatal (severity 3).
    println!("  --- post-serious escalation hunt");
    {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).expect("blk");
        let t = b.tag();
        std::mem::forget(b);
        let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut buf = [0u8; 128];
        let rc = unsafe { PK_BODY_hollow_2(t, -20.0, 1.0e-6, std::ptr::null(), &mut tr, buf.as_mut_ptr() as *mut PK_TOPOL_local_r_t) };
        println!("    seed serious rc={rc}");
        for (label, mut f) in [
            ("check the wounded body", Box::new(move || { let mut n = 0; let mut fl: *mut PK_check_fault_t = std::ptr::null_mut(); unsafe { PK_BODY_check(t, std::ptr::null_mut(), &mut n, &mut fl) } }) as Box<dyn FnMut() -> i32>),
            ("hollow the wounded body again", Box::new(move || { let mut tr2: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() }; let mut b2 = [0u8; 128]; unsafe { PK_BODY_hollow_2(t, -20.0, 1.0e-6, std::ptr::null(), &mut tr2, b2.as_mut_ptr() as *mut PK_TOPOL_local_r_t) } })),
            ("offset the wounded body", Box::new(move || unsafe { PK_BODY_offset(t, -5.0, 1.0e-6, PK_LOGICAL_false) })),
            ("mass props of the wounded body", Box::new(move || { #[repr(C)] struct M { v: i32, mass: i32, per: i32, bnd: i32, single: u8 } let o = M { v: 1, mass: 0x36b4, per: 0x36b6, bnd: 0x36b7, single: 1 }; let (mut a, mut m, mut pp) = (0.0f64, 0.0, 0.0); let mut cg = [0.0f64; 3]; let mut mi = [0.0f64; 9]; unsafe { PK_TOPOL_eval_mass_props(1, &t, 0.99, &o as *const M as *const PK_TOPOL_eval_mass_props_o_t, &mut a, &mut m, &mut cg, &mut mi, &mut pp) } })),
        ] {
            clear();
            let rc = f();
            if rc == 0 { println!("    {label}: rc=0"); continue; }
            if let Some(r) = read_rec(rc) {
                println!("    {label}: rc={rc} code={} token={:?} SEV={} fn={:?}", r.code, r.code_token, r.severity, r.function);
                *st.severities.entry(r.severity).or_insert(0) += 1;
                sev_hits.push((label.to_string(), r.severity, r.code, r.code_token.clone()));
                if r.rc != r.code { st.rc_vs_code_mismatch.push(format!("{label}: rc={} code={}", r.rc, r.code)); }
                if table_name(r.code) != Some(r.code_token.as_str()) {
                    st.token_mismatch.push(format!("{label}: code {} kernel {:?} table {:?}", r.code, r.code_token, table_name(r.code)));
                }
            }
        }
    }

    println!("  --- severity != 1 observations:");
    for (l, s, c, t) in sev_hits.iter().filter(|(_, s, _, _)| *s != 1) {
        println!("    SEV={s} code={c} {t}  <- {l}");
    }

    // ---------------------------------------------------------------- summary
    println!("\n================ SUMMARY ================");
    println!("cases run: {}", st.cases);
    println!("severity histogram (REAL errors only): {:?}", st.severities);
    println!("token mismatches vs error_codes.rs: {}", st.token_mismatch.len());
    for m in &st.token_mismatch { println!("   {m}"); }
    println!("rc != record.code: {}", st.rc_vs_code_mismatch.len());
    for m in &st.rc_vs_code_mismatch { println!("   {m}"); }
    println!("record function != called function: {}", st.fn_mismatch.len());
    for m in &st.fn_mismatch { println!("   {m}"); }
    println!("writes past offset 116: {}", st.tail_writes.len());
    for m in &st.tail_writes { println!("   {m}"); }
    println!("failing calls with NO record: {}", st.no_record.len());
    for m in &st.no_record { println!("   {m}"); }
    let _ = out.flush();
}
