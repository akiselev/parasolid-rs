//! AUTO-GENERATED opaque/scalar stubs for PK types used only by pointer in
//! the vendor prototypes but not yet hand-modelled. Passing a pointer to an
//! opaque struct is ABI-safe; these unblock signature reconciliation.
#![allow(non_camel_case_types)]

use std::os::raw::c_int;

pub type PK_THREAD_id_t = ::std::os::raw::c_int;
pub type PK_UCHAR_t = ::std::os::raw::c_uchar;

// enum-token / handle typedefs (PK_*_t used only by pointer):
pub type PK_ATTDEF_callback_flags_t = c_int;
pub type PK_BCURVE_fit_fault_t = c_int;
pub type PK_BCURVE_fitted_fault_t = c_int;
pub type PK_BCURVE_spline_t = c_int;
pub type PK_BODY_knit_result_t = c_int;
pub type PK_BSURF_constrained_fault_t = c_int;
pub type PK_BSURF_fitted_fault_t = c_int;
pub type PK_CURVE_make_bcurve_res_t = c_int;
pub type PK_CURVE_make_bcurve_t = c_int;
pub type PK_DEBUG_check_fault_t = c_int;
pub type PK_DEBUG_data_t = c_int;
pub type PK_EDGE_array_t = c_int;
pub type PK_EDGE_ask_type_t = c_int;
pub type PK_EDGE_optimise_result_t = c_int;
pub type PK_GROUP_closure_t = c_int;
pub type PK_ITEM_t = c_int;
pub type PK_KI_LIST_t = c_int;
pub type PK_MESH_defect_array_t = c_int;
pub type PK_MESH_fix_result_t = c_int;
pub type PK_SESSION_applio_t = c_int;
pub type PK_SESSION_indexio_t = c_int;
pub type PK_SESSION_schema_version_t = c_int;
pub type PK_SHELL_repair_stat_t = c_int;
pub type PK_SURF_make_bsurf_res_t = c_int;
pub type PK_SURF_make_bsurf_t = c_int;
pub type PK_THREAD_local_t = c_int;
pub type PK_TOPOL_clash_t = c_int;
pub type PK_TRANSF_array_t = c_int;
pub type PK_VERTEX_optimise_result_t = c_int;
pub type PK_achieved_cont_t = c_int;
pub type PK_blend_type_t = c_int;
pub type PK_clone_record_t = c_int;
pub type PK_clone_state_t = c_int;
pub type PK_reset_prec_t = c_int;

// opaque struct stubs (pointer args only):
#[repr(C)] pub struct PK_ASSEMBLY_check_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ASSEMBLY_check_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ATTDEF_callback_fns_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ATTDEF_is_group_closing_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ATTDEF_register_cb_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ATTDEF_set_group_closing_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ATTRIB_ask_no_roll_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ATTRIB_set_no_roll_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_ask_knots_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_clamp_knots_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_create_by_fitting_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_create_spline_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_extend_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_join_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_lower_degree_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_make_bsurf_lofted_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_make_matched_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_piecewise_sf_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_raise_degree_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_remove_knots_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_reparameterise_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_spline_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BCURVE_splinewise_sf_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_add_to_compound_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_ask_children_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_ask_parent_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_copy_topology_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_create_sheet_planar_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_create_topology_2_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_create_topology_2_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_find_knit_pattern_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_identify_facesets_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_imprint_cus_normal_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_imprint_cus_vec_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_imprint_plane_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_make_compound_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_make_section_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_make_swept_body_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_remove_from_parents_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_repair_shells_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_set_type_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_sweep_tool_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_tracked_loft_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_tracked_sweep_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BODY_transform_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_ask_knots_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_clamp_knots_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_create_fitted_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_lower_degree_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_piecewise_sf_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_raise_degree_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_remove_knots_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_BSURF_splinewise_sf_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_embed_in_surf_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_find_box_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_find_vectors_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_fix_degens_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_fix_degens_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_fix_self_int_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_fix_self_int_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_make_bcurve_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_make_spcurves_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_project_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_CURVE_spin_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_DEBUG_behaviours_start_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_DEBUG_report_start_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_DEBUG_shuffle_start_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_attach_curve_nmnl_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_attach_curves_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_find_deviation_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_find_deviation_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_offset_on_body_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_repair_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_reset_precision_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_reverse_2_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_set_blend_chamfer_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_set_blend_variable_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_EDGE_set_precision_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ENTITY_ask_owning_groups_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ENTITY_find_reparam_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_ENTITY_find_reparam_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_classify_details_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_close_gaps_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_delete_from_sheet_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_delete_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_find_blend_unders_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_find_interior_vec_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_find_outer_loop_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_imprint_curves_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_imprint_cus_vec_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_inst_tools_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_make_sect_with_sfs_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_reparameterise_surf_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_reparameterise_surf_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_replace_surfs_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_reverse_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_split_at_param_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FACE_transform_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_FUNCTION_find_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GEOM_ask_geom_category_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GEOM_range_array_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GEOM_range_array_vector_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GEOM_range_local_vector_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GEOM_range_vector_many_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GEOM_render_line_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GEOM_transform_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GROUP_ask_closure_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GROUP_ask_controls_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GROUP_ask_controls_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GROUP_create_from_ents_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_GROUP_find_entities_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_INSTANCE_sf_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_LBALL_ask_blend_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_LBALL_ask_blend_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MARK_goto_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MARK_goto_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_ask_normal_type_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_discard_normals_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_eval_with_mtopol_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_find_laminar_mfins_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_find_laminar_mfins_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_has_unique_normals_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_imprint_vectors_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_is_loaded_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MESH_is_loaded_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MFACET_ask_mvx_normals_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MFACET_find_perimeters_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MFACET_parameterise_vec_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MFIN_ask_mvx_curvature_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MFIN_ask_mvx_normal_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MFIN_is_sharp_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MVERTEX_ask_normals_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_MVERTEX_set_positions_o_t { _private: [u8; 0] }
// PK_PARAM_sf_t: real 40-byte layout defined in geometry.rs.
#[repr(C)] pub struct PK_PARTITION_ask_cloning_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_ask_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_ask_pmarks_2_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_ask_pmarks_2_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_ask_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_is_clone_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_make_pmark_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_merge_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_receive_meshes_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_reset_attribs_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_start_cloning_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_stop_cloning_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PARTITION_transmit_delta_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PART_ask_attribs_cb_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PART_ask_groups_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PART_delete_attribs_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PART_receive_meshes_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PMARK_ask_entities_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_PMARK_goto_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SESSION_ask_attdefs_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SESSION_ask_behaviour_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SESSION_ask_err_reports_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SESSION_ask_max_threads_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SESSION_set_behaviour_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SESSION_set_err_reports_o_t { _private: [u8; 0] }
// PK_SESSION_smp_o_t has a real 16-byte layout defined in session.rs.
#[repr(C)] pub struct PK_SURF_create_blend_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_find_box_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_find_vectors_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_fix_degens_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_fix_degens_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_fix_self_int_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_fix_self_int_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_make_bsurf_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_SURF_make_cus_isocline_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_ask_err_reports_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_ask_function_run_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_ask_local_level_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_ask_partitions_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_chain_start_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_chain_stop_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_lock_partitions_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_lock_partitions_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_set_err_reports_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_set_function_run_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_set_id_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_set_id_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_THREAD_unlock_partitions_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_facet_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_facet_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_range_array_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_range_array_vector_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_range_geom_array_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_range_geom_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_range_local_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TOPOL_range_local_vector_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TRANSF_classify_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TRANSF_classify_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TRANSF_create_view_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TRANSF_transform_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_TRANSF_transform_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_VERTEX_optimise_o_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_identify_facesets_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_identify_general_r_t { _private: [u8; 0] }
#[repr(C)] pub struct PK_section_2_r_t { _private: [u8; 0] }

// --- by-value enum typedefs (4-byte, passed in a register) ---
pub type PK_ERROR_reports_t = c_int;
/// `PK_ERROR_reports_t` tokens (RE: pk-enum-values.tsv, verified 26820..26822).
pub const PK_ERROR_reports_on_c: PK_ERROR_reports_t = 26820;
pub const PK_ERROR_reports_off_c: PK_ERROR_reports_t = 26821;
pub const PK_ERROR_reports_inherit_c: PK_ERROR_reports_t = 26822;
pub type PK_blend_edge_shape_t = c_int;
pub type PK_blend_check_t = c_int;
pub type PK_THREAD_chain_type_t = c_int;
pub type PK_THREAD_exclusion_t = c_int;
pub type PK_SESSION_binding_t = c_int;
pub type PK_piecewise_rep_t = c_int;
// --- callback function-pointer typedefs (nullable; opaque signature) ---
pub type PK_ATTRIB_cb_f_t = Option<unsafe extern "C" fn()>;
pub type PK_DEBUG_try_error_handler_f_t = Option<unsafe extern "C" fn()>;
pub type PK_ATTRIB_reset_cb_f_t = Option<unsafe extern "C" fn()>;
pub type PK_DEBUG_SESSION_entry_cb_t = Option<unsafe extern "C" fn()>;
pub type PK_DEBUG_SESSION_exit_cb_t = Option<unsafe extern "C" fn()>;
pub type PK_DEBUG_SESSION_create_cb_t = Option<unsafe extern "C" fn()>;
pub type PK_DEBUG_SESSION_destroy_cb_t = Option<unsafe extern "C" fn()>;
pub type PK_SESSION_watch_create_cb_t = Option<unsafe extern "C" fn()>;
pub type PK_SESSION_watch_destroy_cb_t = Option<unsafe extern "C" fn()>;
// --- thread lock/wait enums (c_int tokens) ---
pub type PK_THREAD_lock_t = c_int;
pub type PK_THREAD_wait_t = c_int;
// --- aggregate descriptors / structs (>8 bytes -> by reference on Win64) ---
#[repr(C)] pub struct PK_CLASS_array_t { _p: [u8; 16] }
#[repr(C)] pub struct PK_ITEM_array_t { _p: [u8; 16] }
#[repr(C)] pub struct PK_int_array_t { _p: [u8; 16] }
#[repr(C)] pub struct PK_SURF_trim_data_t { _p: [u8; 16] }
#[repr(C)] pub struct PK_BSURF_create_rained_o_t { _private: [u8; 0] }
