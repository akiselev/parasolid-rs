#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
//! Checking, validation, and report bindings for the Parasolid PK_* C API.
//!
//! Covers:
//! - Entity check functions (`PK_BODY_check`, `PK_EDGE_check`, etc.)
//! - Session check control (`PK_SESSION_set_check_*`, `PK_SESSION_ask_check_*`)
//! - Fault state tokens (~120 constants)
//! - Check option structs
//! - Report functions (`PK_REPORT_*`)

use crate::*;
use std::os::raw::{c_char, c_double, c_int};

// =============================================================================
// Check returns enhancement level
// =============================================================================

pub type PK_check_returns_t = c_int;
pub const PK_check_returns_0_c: PK_check_returns_t = 23661;
pub const PK_check_returns_1_c: PK_check_returns_t = 23662;
pub const PK_check_returns_2_c: PK_check_returns_t = 23663;
pub const PK_check_returns_3_c: PK_check_returns_t = 23664;
pub const PK_check_returns_4_c: PK_check_returns_t = 23665;
pub const PK_check_returns_5_c: PK_check_returns_t = 23666;
/// Sentinel selecting the newest checker version — a distinct token (23660),
/// NOT an alias of `_4_c`/`_5_c` in the real ABI.
pub const PK_check_returns_latest_c: PK_check_returns_t = 23660;

// =============================================================================
// Check extra faults level
// =============================================================================

pub type PK_check_extra_faults_t = c_int;
pub const PK_check_extra_faults_0_c: PK_check_extra_faults_t = 24720;
pub const PK_check_extra_faults_1_c: PK_check_extra_faults_t = 24721;
pub const PK_check_extra_faults_latest_c: PK_check_extra_faults_t =
    PK_check_extra_faults_1_c;

// =============================================================================
// Attribute checking constants
// =============================================================================

pub type PK_check_attribs_t = c_int;
pub const PK_check_attribs_no_c: PK_check_attribs_t = 25230;
pub const PK_check_attribs_yes_c: PK_check_attribs_t = 25231;

// =============================================================================
// Local check result constants
// =============================================================================

pub type PK_local_check_t = c_int;
pub const PK_local_check_ok_c: PK_local_check_t = 18101;
pub const PK_local_check_negated_c: PK_local_check_t = 18102;
pub const PK_local_check_failed_c: PK_local_check_t = 18103;
pub const PK_local_check_no_c: PK_local_check_t = 18100;

// =============================================================================
// Fault state tokens — Data Structure Faults
// =============================================================================

pub type PK_check_state_t = c_int;

pub const PK_ASSEMBLY_state_corrupt_c: PK_check_state_t = 22895;
pub const PK_BB_state_corrupt_c: PK_check_state_t = 22891;
pub const PK_BODY_state_corrupt_c: PK_check_state_t = 13802;
pub const PK_GEOM_state_corrupt_c: PK_check_state_t = 22893;
pub const PK_PARTITION_state_corrupt_c: PK_check_state_t = 22896;
pub const PK_SESSION_state_corrupt_c: PK_check_state_t = 22890;
pub const PK_TRANSF_state_corrupt_c: PK_check_state_t = 22894;
pub const PK_BODY_state_invalid_ident_c: PK_check_state_t = 13836;

// =============================================================================
// Fault state tokens — Body Structure Faults (extra_faults detail)
// =============================================================================

pub const PK_BODY_state_bad_type_c: PK_check_state_t = 24622;
pub const PK_BODY_state_no_region_c: PK_check_state_t = 24623;
pub const PK_BODY_state_no_shell_c: PK_check_state_t = 24624;
pub const PK_BODY_state_no_solid_reg_c: PK_check_state_t = 24625;
pub const PK_BODY_state_region_1_solid_c: PK_check_state_t = 24626;
pub const PK_BODY_state_shell_not_first_c: PK_check_state_t = 24627;
pub const PK_BODY_state_wrong_ext_reg_c: PK_check_state_t = 24628;
pub const PK_BODY_state_wrong_num_regs_c: PK_check_state_t = 24629;
pub const PK_BODY_state_inside_out_c: PK_check_state_t = 13801;
pub const PK_BODY_state_bad_regions_c: PK_check_state_t = 13835;

// =============================================================================
// Fault state tokens — Geometry Faults
// =============================================================================

pub const PK_ENTITY_state_invalid_c: PK_check_state_t = 13808;
pub const PK_GEOM_state_self_int_c: PK_check_state_t = 13805;
pub const PK_GEOM_state_degenerate_c: PK_check_state_t = 13806;
pub const PK_BCURVE_state_close_knots_c: PK_check_state_t = 22899;
pub const PK_BSURF_state_close_u_knots_c: PK_check_state_t = 22900;
pub const PK_BSURF_state_close_v_knots_c: PK_check_state_t = 22901;
pub const PK_GEOM_state_bad_dep_type_c: PK_check_state_t = 24658;
pub const PK_GEOM_state_bad_geom_owner_c: PK_check_state_t = 24659;
pub const PK_GEOM_state_bad_owner_c: PK_check_state_t = 24660;
pub const PK_GEOM_state_bad_topol_owner_c: PK_check_state_t = 24661;
pub const PK_GEOM_state_dup_geom_owner_c: PK_check_state_t = 24662;
pub const PK_GEOM_state_not_owners_dep_c: PK_check_state_t = 24663;
pub const PK_GEOM_state_not_owning_dep_c: PK_check_state_t = 24664;

// =============================================================================
// Fault state tokens — Topology Faults
// =============================================================================

pub const PK_TOPOL_state_no_geom_c: PK_check_state_t = 13803;
pub const PK_TOPOL_state_not_G1_c: PK_check_state_t = 13827;
pub const PK_TOPOL_state_bad_closed_c: PK_check_state_t = 22903;
pub const PK_TOPOL_state_u_parm_degen_c: PK_check_state_t = 22904;
pub const PK_TOPOL_state_v_parm_degen_c: PK_check_state_t = 22905;
pub const PK_TOPOL_state_parm_degen_c: PK_check_state_t = 22906;
pub const PK_TOPOL_state_u_phys_degen_c: PK_check_state_t = 22907;
pub const PK_TOPOL_state_v_phys_degen_c: PK_check_state_t = 22908;
pub const PK_TOPOL_state_phys_degen_c: PK_check_state_t = 22909;
pub const PK_TOPOL_state_size_box_c: PK_check_state_t = 13810;
pub const PK_TOPOL_state_check_fail_c: PK_check_state_t = 13812;
pub const PK_TOPOL_state_bad_box_c: PK_check_state_t = 24693;
pub const PK_TOPOL_state_bad_geom_share_c: PK_check_state_t = 24694;
pub const PK_TOPOL_state_bad_owner_type_c: PK_check_state_t = 24695;
pub const PK_TOPOL_state_share_no_geom_c: PK_check_state_t = 24696;

// =============================================================================
// Fault state tokens — Edge Faults
// =============================================================================

pub const PK_EDGE_state_open_c: PK_check_state_t = 13817;
pub const PK_EDGE_state_open_nmnl_c: PK_check_state_t = 13837;
pub const PK_EDGE_state_bad_vertex_c: PK_check_state_t = 13818;
pub const PK_EDGE_state_bad_vertex_nmnl_c: PK_check_state_t = 13838;
pub const PK_EDGE_state_reversed_c: PK_check_state_t = 13819;
pub const PK_EDGE_state_reversed_nmnl_c: PK_check_state_t = 13840;
pub const PK_EDGE_state_bad_spcurve_c: PK_check_state_t = 13820;
pub const PK_EDGE_state_bad_sp_nmnl_c: PK_check_state_t = 13839;
pub const PK_EDGE_state_vertices_touch_c: PK_check_state_t = 13821;
pub const PK_EDGE_state_bad_face_order_c: PK_check_state_t = 13832;
pub const PK_EDGE_state_bad_polyline_c: PK_check_state_t = 26570;
pub const PK_EDGE_state_bad_order_c: PK_check_state_t = 16406;
pub const PK_EDGE_state_bad_wire_ed_ed_c: PK_check_state_t = 13831;
pub const PK_EDGE_state_touch_edge_c: PK_check_state_t = 22902;
pub const PK_EDGE_state_bad_fins_c: PK_check_state_t = 24630;
pub const PK_EDGE_state_bad_tol_c: PK_check_state_t = 24631;
pub const PK_EDGE_state_fin_bad_ring_c: PK_check_state_t = 24632;
pub const PK_EDGE_state_fin_ed_next_pos_c: PK_check_state_t = 24633;
pub const PK_EDGE_state_fin_ed_not_ed_c: PK_check_state_t = 24634;
pub const PK_EDGE_state_fin_not_pos_c: PK_check_state_t = 24635;
pub const PK_EDGE_state_single_vertex_c: PK_check_state_t = 24636;
pub const PK_EDGE_state_wire_corrupt_c: PK_check_state_t = 24637;

// =============================================================================
// Fault state tokens — Face Faults
// =============================================================================

pub const PK_FACE_state_bad_vertex_c: PK_check_state_t = 13822;
pub const PK_FACE_state_bad_edge_c: PK_check_state_t = 13823;
pub const PK_FACE_state_self_int_c: PK_check_state_t = 13804;
pub const PK_FACE_state_bad_edge_order_c: PK_check_state_t = 13824;
pub const PK_FACE_state_bad_loops_c: PK_check_state_t = 13826;
pub const PK_FACE_state_redundant_c: PK_check_state_t = 16402;
pub const PK_FACE_state_no_vtx_at_sing_c: PK_check_state_t = 13825;
pub const PK_FACE_state_bad_wire_fa_ed_c: PK_check_state_t = 13830;
pub const PK_FACE_state_bad_face_face_c: PK_check_state_t = 13816;
pub const PK_FACE_state_check_fail_c: PK_check_state_t = 13829;
pub const PK_FACE_state_diff_sh_same_rg_c: PK_check_state_t = 24642;
pub const PK_FACE_state_edge_moebius_c: PK_check_state_t = 24643;
pub const PK_FACE_state_wrong_sense_c: PK_check_state_t = 24646;

// =============================================================================
// Fault state tokens — Curve Faults
// =============================================================================

pub const PK_CURVE_state_inconsistent_c: PK_check_state_t = 16403;

// =============================================================================
// Fault state tokens — Loop Faults
// =============================================================================

pub const PK_LOOP_state_invalid_c: PK_check_state_t = 16405;
pub const PK_LOOP_state_isolated_has_cu_c: PK_check_state_t = 24669;

// =============================================================================
// Fault state tokens — Fin Faults
// =============================================================================

pub const PK_FIN_state_bad_c: PK_check_state_t = 24647;
pub const PK_FIN_state_bad_ed_fins_c: PK_check_state_t = 24648;
pub const PK_FIN_state_corrupt_c: PK_check_state_t = 24649;
pub const PK_FIN_state_ed_next_is_fin_c: PK_check_state_t = 24650;
pub const PK_FIN_state_edge_diff_vertex_c: PK_check_state_t = 24651;
pub const PK_FIN_state_no_ed_next_in_sh_c: PK_check_state_t = 24652;
pub const PK_FIN_state_non_zero_ident_c: PK_check_state_t = 24653;
pub const PK_FIN_state_not_ed_next_prev_c: PK_check_state_t = 24654;
pub const PK_FIN_state_not_lp_next_prev_c: PK_check_state_t = 24655;
pub const PK_FIN_state_vx_not_vx_c: PK_check_state_t = 24656;
pub const PK_FIN_state_wrong_vertex_c: PK_check_state_t = 24657;

// =============================================================================
// Fault state tokens — Vertex Faults
// =============================================================================

pub const PK_VERTEX_state_bad_fin_c: PK_check_state_t = 24697;
pub const PK_VERTEX_state_bad_isolated_c: PK_check_state_t = 24698;
pub const PK_VERTEX_state_bad_tol_c: PK_check_state_t = 24699;
pub const PK_VERTEX_state_fin_chains_c: PK_check_state_t = 24700;
pub const PK_VERTEX_state_non_manifold_c: PK_check_state_t = 24701;
pub const PK_VERTEX_state_not_dep_of_sh_c: PK_check_state_t = 24702;
pub const PK_VERTEX_state_owner_not_sh_c: PK_check_state_t = 24703;
pub const PK_VERTEX_state_sheet_corrupt_c: PK_check_state_t = 24704;
pub const PK_VERTEX_state_too_many_eds_c: PK_check_state_t = 24705;
pub const PK_VERTEX_state_wf_ed_bad_sh_c: PK_check_state_t = 24706;

// =============================================================================
// Fault state tokens — Shell Faults
// =============================================================================

pub const PK_SHELL_state_bad_topol_geom_c: PK_check_state_t = 13833;
pub const PK_SHELL_state_bad_sh_sh_c: PK_check_state_t = 13834;
pub const PK_SHELL_state_bad_fa_orient_c: PK_check_state_t = 24679;
pub const PK_SHELL_state_bad_wf_acorn_c: PK_check_state_t = 24680;
pub const PK_SHELL_state_bad_wf_c: PK_check_state_t = 24681;
pub const PK_SHELL_state_closed_bad_fa_c: PK_check_state_t = 24682;
pub const PK_SHELL_state_ed_fa_in_acorn_c: PK_check_state_t = 24683;
pub const PK_SHELL_state_eds_fragmented_c: PK_check_state_t = 24684;
pub const PK_SHELL_state_fas_fragmented_c: PK_check_state_t = 24685;
pub const PK_SHELL_state_no_vx_ed_fa_c: PK_check_state_t = 24686;
pub const PK_SHELL_state_not_owning_vx_c: PK_check_state_t = 24687;
pub const PK_SHELL_state_open_bad_fa_c: PK_check_state_t = 24688;
pub const PK_SHELL_state_reg_meet_at_ed_c: PK_check_state_t = 24689;
pub const PK_SHELL_state_reg_meet_at_vx_c: PK_check_state_t = 24690;
pub const PK_SHELL_state_sheet_no_fa_c: PK_check_state_t = 24691;
pub const PK_SHELL_state_too_many_fas_c: PK_check_state_t = 24692;

// =============================================================================
// Fault state tokens — Region Faults
// =============================================================================

pub const PK_REGION_state_bad_shells_c: PK_check_state_t = 13828;
pub const PK_REGION_state_no_shell_c: PK_check_state_t = 24676;
pub const PK_REGION_state_wrongly_solid_c: PK_check_state_t = 24677;

// =============================================================================
// Fault state tokens — Partition Faults
// =============================================================================

pub const PK_PARTITION_state_xref_c: PK_check_state_t = 22897;

// =============================================================================
// Fault state tokens — Entity Faults
// =============================================================================

pub const PK_ENTITY_state_bad_owner_c: PK_check_state_t = 24639;
pub const PK_ENTITY_state_has_att_group_c: PK_check_state_t = 24640;
pub const PK_ENTITY_state_in_group_c: PK_check_state_t = 24713;
pub const PK_ENTITY_state_shared_c: PK_check_state_t = 24641;
pub const PK_ITEM_state_unattached_c: PK_check_state_t = 22898;

// =============================================================================
// Fault state tokens — Part Faults
// =============================================================================

pub const PK_PART_state_bad_attrib_c: PK_check_state_t = 24670;
pub const PK_PART_state_bad_attrib_list_c: PK_check_state_t = 24671;
pub const PK_PART_state_bad_state_c: PK_check_state_t = 24672;
pub const PK_PART_state_corrupt_c: PK_check_state_t = 24673;
pub const PK_PART_state_has_key_c: PK_check_state_t = 24674;
pub const PK_PART_state_no_key_c: PK_check_state_t = 24675;

// =============================================================================
// Fault state tokens — Group Faults
// =============================================================================

pub const PK_GROUP_state_bad_c: PK_check_state_t = 24665;

// =============================================================================
// Fault state tokens — Attribute Definition Faults
// =============================================================================

pub const PK_ATTDEF_state_bad_name_c: PK_check_state_t = 13841;

// =============================================================================
// Fault state tokens — Attribute Faults (general)
// =============================================================================

pub const PK_ATTRIB_state_bad_string_c: PK_check_state_t = 13842;

// =============================================================================
// Fault state tokens — System Attribute Check Faults
// =============================================================================

pub const PK_ATTRIB_state_bad_data_len_c: PK_check_state_t = 13844;
pub const PK_ATTRIB_state_bad_ustring_c: PK_check_state_t = 13845;
pub const PK_ATTRIB_state_byte_oor_c: PK_check_state_t = 13846;
pub const PK_ATTRIB_state_empty_field_c: PK_check_state_t = 13847;
pub const PK_ATTRIB_state_int_oor_c: PK_check_state_t = 13848;
pub const PK_ATTRIB_state_invalid_att_c: PK_check_state_t = 13843;
pub const PK_ATTRIB_state_non_unit_vec_c: PK_check_state_t = 13849;
pub const PK_ATTRIB_state_real_oor_c: PK_check_state_t = 13850;
pub const PK_ATTRIB_state_short_oor_c: PK_check_state_t = 13851;
pub const PK_ATTRIB_state_p_vector_oor_c: PK_check_state_t = 13852;
// [re-abi] appended 75 missing member(s) from pk-enums.h
pub const PK_BODY_state_ok_c: PK_check_state_t = 16401;
pub const PK_KI_LIST_state_corrupt_c: PK_check_state_t = 22892;
pub const PK_ASSEMBLY_state_cyclic_ref_c: PK_check_state_t = 24621;
pub const PK_INSTANCE_state_bad_transf_c: PK_check_state_t = 24666;
pub const PK_INSTANCE_state_bad_type_c: PK_check_state_t = 24667;
pub const PK_SHELL_state_bad_body_c: PK_check_state_t = 24678;
pub const PK_LOOP_state_not_closed_c: PK_check_state_t = 24707;
pub const PK_TOPOL_state_extra_child_c: PK_check_state_t = 24708;
pub const PK_TOPOL_state_extra_parent_c: PK_check_state_t = 24709;
pub const PK_TOPOL_state_missing_parent_c: PK_check_state_t = 24710;
pub const PK_TOPOL_state_wrong_child_c: PK_check_state_t = 24711;
pub const PK_BODY_state_ok_but_alt_type_c: PK_check_state_t = 24712;
pub const PK_SPCURVE_state_bad_linear_c: PK_check_state_t = 25760;
pub const PK_BCURVE_state_vx_periodic_c: PK_check_state_t = 25761;
pub const PK_BCURVE_state_knot_periodic_c: PK_check_state_t = 25762;
pub const PK_BCURVE_state_knot_bad_mult_c: PK_check_state_t = 25764;
pub const PK_BCURVE_state_knot_vx_count_c: PK_check_state_t = 25765;
pub const PK_BCURVE_state_knot_count_c: PK_check_state_t = 25766;
pub const PK_BCURVE_state_bad_knot_seq_c: PK_check_state_t = 25767;
pub const PK_BCURVE_state_bad_dimen_c: PK_check_state_t = 25768;
pub const PK_BLENDSF_state_bad_spine_c: PK_check_state_t = 25769;
pub const PK_BSURF_state_vx_periodic_c: PK_check_state_t = 25770;
pub const PK_BSURF_state_knot_vx_count_c: PK_check_state_t = 25771;
pub const PK_BSURF_state_uknot_periodic_c: PK_check_state_t = 25772;
pub const PK_BSURF_state_vknot_periodic_c: PK_check_state_t = 25773;
pub const PK_BSURF_state_uknot_bad_mult_c: PK_check_state_t = 25774;
pub const PK_BSURF_state_vknot_bad_mult_c: PK_check_state_t = 25775;
pub const PK_BSURF_state_uknot_count_c: PK_check_state_t = 25776;
pub const PK_BSURF_state_vknot_count_c: PK_check_state_t = 25777;
pub const PK_BSURF_state_bad_uknot_seq_c: PK_check_state_t = 25778;
pub const PK_BSURF_state_bad_vknot_seq_c: PK_check_state_t = 25779;
pub const PK_BSURF_state_bad_dimen_c: PK_check_state_t = 25780;
pub const PK_CIRCLE_state_bad_radius_c: PK_check_state_t = 25781;
pub const PK_GEOM_state_outside_box_c: PK_check_state_t = 25782;
pub const PK_LINE_state_bad_unit_vec_c: PK_check_state_t = 25783;
pub const PK_OFFSET_state_bad_dist_c: PK_check_state_t = 25784;
pub const PK_OFFSET_state_under_sense_c: PK_check_state_t = 25785;
pub const PK_OFFSET_state_under_non_g1_c: PK_check_state_t = 25786;
pub const PK_SPCURVE_state_surf_span_c: PK_check_state_t = 25787;
pub const PK_SPCURVE_state_non_g1_x_u_c: PK_check_state_t = 25788;
pub const PK_SPCURVE_state_non_g1_x_v_c: PK_check_state_t = 25789;
pub const PK_SPCURVE_state_non_c1_x_u_c: PK_check_state_t = 25790;
pub const PK_SPCURVE_state_non_c1_x_v_c: PK_check_state_t = 25791;
pub const PK_SPCURVE_state_non_g1_c: PK_check_state_t = 25792;
pub const PK_SPCURVE_state_bad_dimen_c: PK_check_state_t = 25793;
pub const PK_SPUN_state_profile_x_axis_c: PK_check_state_t = 25794;
pub const PK_SWEPT_state_bad_min_rad_c: PK_check_state_t = 25795;
pub const PK_TRCURVE_state_ends_open_c: PK_check_state_t = 25796;
pub const PK_TRCURVE_state_ends_order_c: PK_check_state_t = 25797;
pub const PK_TRCURVE_state_ends_match_c: PK_check_state_t = 25798;
pub const PK_BSURF_state_bad_vx_uperiod_c: PK_check_state_t = 25801;
pub const PK_BSURF_state_bad_vx_vperiod_c: PK_check_state_t = 25802;
pub const PK_MESH_state_bad_mvx_normal_c: PK_check_state_t = 25930;
pub const PK_MESH_state_corrupt_c: PK_check_state_t = 25931;
pub const PK_MESH_state_degen_mfacet_c: PK_check_state_t = 25932;
pub const PK_MESH_state_flat_mfacet_c: PK_check_state_t = 25933;
pub const PK_MESH_state_non_manifold_c: PK_check_state_t = 25934;
pub const PK_MESH_state_self_int_c: PK_check_state_t = 25935;
pub const PK_MESH_state_slit_c: PK_check_state_t = 25936;
pub const PK_FACE_state_disjoint_mesh_c: PK_check_state_t = 25937;
pub const PK_MESH_state_not_loaded_c: PK_check_state_t = 25938;
pub const PK_MESH_state_not_created_c: PK_check_state_t = 25939;
pub const PK_LATTICE_state_bad_graph_c: PK_check_state_t = 26571;
pub const PK_LATTICE_state_bad_lball_c: PK_check_state_t = 26572;
pub const PK_LATTICE_state_bad_lrod_c: PK_check_state_t = 26573;
pub const PK_LATTICE_state_corrupt_c: PK_check_state_t = 26574;
pub const PK_LATTICE_state_disjoint_c: PK_check_state_t = 26575;
pub const PK_LATTICE_state_dup_lrod_c: PK_check_state_t = 26576;
pub const PK_LATTICE_state_isolated_c: PK_check_state_t = 26577;
pub const PK_LATTICE_state_lrod_end_c: PK_check_state_t = 26578;
pub const PK_LATTICE_state_no_lballs_c: PK_check_state_t = 26579;
pub const PK_LATTICE_state_no_lrods_c: PK_check_state_t = 26770;
pub const PK_LATTICE_state_self_int_c: PK_check_state_t = 26771;
pub const PK_LATTICE_state_short_lrod_c: PK_check_state_t = 26772;
pub const PK_ASSEMBLY_state_invalid_id_c: PK_check_state_t = 26773;

// =============================================================================
// Check option structs
// =============================================================================

/// Options for `PK_BODY_check`. Controls which checks are performed and
/// how results are reported.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_BODY_check_o_t {
    /// Structure version marker (set via `PK_BODY_check_o_m`).
    pub o_t_version: c_int,
    /// Maximum number of faults to return (0 = no limit).
    pub max_faults: c_int,
    /// Check geometry validity.
    pub geom: PK_LOGICAL_t,
    /// Check B-geometry continuity.
    pub bgeom: PK_LOGICAL_t,
    /// Check mesh data.
    pub mesh: PK_LOGICAL_t,
    /// Check topology-geometry consistency.
    pub top_geo: PK_LOGICAL_t,
    /// Check for size box violations.
    pub size_box: PK_LOGICAL_t,
    /// Check for face self-intersections.
    pub fa_x: PK_LOGICAL_t,
    /// Check for loop consistency.
    pub loops: PK_LOGICAL_t,
    /// Check for face-face inconsistencies.
    pub fa_fa: PK_LOGICAL_t,
    /// Check for inside-out or inconsistent shells.
    pub sh: PK_LOGICAL_t,
    /// Check for corrupt data structures and identifiers.
    pub corrupt: PK_LOGICAL_t,
    /// Check for nominal geometry errors.
    pub nmnl_geom: PK_LOGICAL_t,
    /// Control level of information returned.
    pub returns: PK_check_returns_t,
    /// Additional corrupt data structure info.
    pub extra_faults: PK_check_extra_faults_t,
    /// Check system attribute validity.
    pub attribs: PK_check_attribs_t,
}

/// Options for `PK_EDGE_check`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_EDGE_check_o_t {
    pub o_t_version: c_int,
    pub max_faults: c_int,
    pub geom: PK_LOGICAL_t,
    pub bgeom: PK_LOGICAL_t,
    pub top_geo: PK_LOGICAL_t,
    pub size_box: PK_LOGICAL_t,
    pub nmnl_geom: PK_LOGICAL_t,
    pub returns: PK_check_returns_t,
    pub attribs: PK_check_attribs_t,
}

/// Options for `PK_FACE_check`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_check_o_t {
    pub o_t_version: c_int,
    pub max_faults: c_int,
    pub geom: PK_LOGICAL_t,
    pub bgeom: PK_LOGICAL_t,
    pub mesh: PK_LOGICAL_t,
    pub top_geo: PK_LOGICAL_t,
    pub size_box: PK_LOGICAL_t,
    pub fa_x: PK_LOGICAL_t,
    pub loops: PK_LOGICAL_t,
    pub nmnl_geom: PK_LOGICAL_t,
    pub returns: PK_check_returns_t,
    pub attribs: PK_check_attribs_t,
}

/// Options for `PK_GEOM_check`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_GEOM_check_o_t {
    pub o_t_version: c_int,
    pub max_faults: c_int,
    pub geom: PK_LOGICAL_t,
    pub extra_faults: PK_check_extra_faults_t,
    pub attribs: PK_check_attribs_t,
}

/// Options for `PK_TRANSF_check`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TRANSF_check_o_t {
    pub o_t_version: c_int,
    pub max_faults: c_int,
}

/// Options for `PK_ENTITY_check_attribs`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_ENTITY_check_attribs_o_t {
    pub o_t_version: c_int,
    pub max_faults: c_int,
}

/// Options for `PK_FACE_check_pair`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_FACE_check_pair_o_t {
    pub o_t_version: c_int,
    pub max_faults: c_int,
}

// =============================================================================
// Check result structures
// =============================================================================

/// A single fault returned by a check function.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_check_fault_t {
    /// The entity where the fault was found.
    pub entity: PK_ENTITY_t,
    /// Fault state token (one of the `PK_*_state_*_c` constants).
    pub state: PK_check_state_t,
    /// Secondary entity related to the fault (or `PK_ENTITY_null`).
    pub entity_2: PK_ENTITY_t,
}

// =============================================================================
// Report types
// =============================================================================

/// A single record within a report.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_REPORT_record_t {
    /// Name of the function that produced this record.
    pub function: *const c_char,
    /// Record type identifier.
    pub record_type: c_int,
    /// Status code associated with this record.
    pub status: c_int,
    /// Number of integers in the record data.
    pub n_ints: c_int,
    /// Integer data array.
    pub ints: *const c_int,
    /// Number of doubles in the record data.
    pub n_doubles: c_int,
    /// Double data array.
    pub doubles: *const c_double,
}

/// Top-level report return structure from `PK_REPORT_ask`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_REPORT_r_t {
    /// Name of the function that last wrote to the report.
    pub function: *const c_char,
    /// Number of records in the report.
    pub n_records: c_int,
    /// Array of report records.
    pub records: *const PK_REPORT_record_t,
}

// =============================================================================
// Extern "C" — Checking functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

    /// Check an edge for validity.
    pub fn PK_EDGE_check(
        edge: PK_EDGE_t,
        options: *const PK_EDGE_check_o_t,
        n_faults: *mut c_int,
        faults: *mut *mut PK_check_fault_t,
    ) -> PK_ERROR_code_t;

    /// Check blend edges for validity.
    pub fn PK_EDGE_check_blends(
        n_edges: c_int,
        edges: *mut PK_EDGE_t,
        level: PK_blend_check_t,
        faults: *mut PK_blend_fault_t,
        fault_topols: *mut PK_TOPOL_t,
    ) -> PK_ERROR_code_t;

    /// Check a face for validity.
    pub fn PK_FACE_check(
        face: PK_FACE_t,
        options: *const PK_FACE_check_o_t,
        n_faults: *mut c_int,
        faults: *mut *mut PK_check_fault_t,
    ) -> PK_ERROR_code_t;

    /// Perform face-face checks on a pair of faces.
    pub fn PK_FACE_check_pair(
        face_1: PK_FACE_t,
        face_2: PK_FACE_t,
        options: *const PK_FACE_check_pair_o_t,
        n_faults: *mut c_int,
        faults: *mut *mut PK_check_fault_t,
    ) -> PK_ERROR_code_t;

    /// Check a geometric entity (point, curve, or surface) for validity.
    pub fn PK_GEOM_check(
        geom: PK_GEOM_t,
        options: *const PK_GEOM_check_o_t,
        n_faults: *mut c_int,
        faults: *mut *mut PK_check_fault_t,
    ) -> PK_ERROR_code_t;

    /// Check system attributes on an entity (and optionally sub-entities).
    pub fn PK_ENTITY_check_attribs(
        entity: PK_ENTITY_t,
        attdef: PK_ATTDEF_t,
        options: *mut PK_ENTITY_check_attribs_o_t,
        n_faults: *mut c_int,
        faults: *mut *mut PK_check_fault_t,
    ) -> PK_ERROR_code_t;
}

// =============================================================================
// Extern "C" — Session check control functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {

}

// =============================================================================
// Extern "C" — Report functions
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    /// Find a named report. Use `"SDL/TYREP00"` for the Parasolid report.
    pub fn PK_REPORT_find(
        name: *const c_char,
        report: *mut PK_REPORT_t,
    ) -> PK_ERROR_code_t;

    /// Copy a report from Parasolid memory into PK memory. Free the result
    /// with `PK_REPORT_r_f`.
    pub fn PK_REPORT_ask(
        report: PK_REPORT_t,
        report_r: *mut PK_REPORT_r_t,
    ) -> PK_ERROR_code_t;

    /// Delete all records in a report and free its memory.
    pub fn PK_REPORT_clear(
        report: PK_REPORT_t,
    ) -> PK_ERROR_code_t;

    /// Free PK memory used by a `PK_REPORT_r_t` returned from `PK_REPORT_ask`.
    /// Does NOT clear the report itself.
    pub fn PK_REPORT_r_f(
        report_r: *mut PK_REPORT_r_t,
    ) -> PK_ERROR_code_t;

    /// Query whether a report is currently open (being written to).
    pub fn PK_REPORT_is_open(
        report: PK_REPORT_t,
        is_open: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Create a new report entity.
    pub fn PK_REPORT_create(
        name: *const c_char,
        report: *mut PK_REPORT_t,
    ) -> PK_ERROR_code_t;

    /// Delete a report entity.
    pub fn PK_REPORT_delete(
        report: PK_REPORT_t,
    ) -> PK_ERROR_code_t;

    /// Close an open report.
    pub fn PK_REPORT_close(
        report: PK_REPORT_t,
    ) -> PK_ERROR_code_t;

    /// Add records to a report.
    pub fn PK_REPORT_add_records(
        report: PK_REPORT_t,
        n_records: c_int,
        records: *const PK_REPORT_record_t,
    ) -> PK_ERROR_code_t;

    /// Set the function name associated with a report.
    pub fn PK_REPORT_set_function(
        report: PK_REPORT_t,
        function: *const c_char,
    ) -> PK_ERROR_code_t;
}
