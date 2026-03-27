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
pub const PK_check_returns_0_c: PK_check_returns_t = 0;
pub const PK_check_returns_1_c: PK_check_returns_t = 1;
pub const PK_check_returns_2_c: PK_check_returns_t = 2;
pub const PK_check_returns_3_c: PK_check_returns_t = 3;
pub const PK_check_returns_4_c: PK_check_returns_t = 4;
pub const PK_check_returns_latest_c: PK_check_returns_t = PK_check_returns_4_c;

// =============================================================================
// Check extra faults level
// =============================================================================

pub type PK_check_extra_faults_t = c_int;
pub const PK_check_extra_faults_0_c: PK_check_extra_faults_t = 0;
pub const PK_check_extra_faults_1_c: PK_check_extra_faults_t = 1;
pub const PK_check_extra_faults_latest_c: PK_check_extra_faults_t =
    PK_check_extra_faults_1_c;

// =============================================================================
// Attribute checking constants
// =============================================================================

pub type PK_check_attribs_t = c_int;
pub const PK_check_attribs_no_c: PK_check_attribs_t = 0;
pub const PK_check_attribs_yes_c: PK_check_attribs_t = 1;

// =============================================================================
// Local check result constants
// =============================================================================

pub type PK_local_check_t = c_int;
pub const PK_local_check_ok_c: PK_local_check_t = 0;
pub const PK_local_check_negated_c: PK_local_check_t = 1;
pub const PK_local_check_failed_c: PK_local_check_t = 2;
pub const PK_local_check_no_c: PK_local_check_t = 3;

// =============================================================================
// Fault state tokens — Data Structure Faults
// =============================================================================

pub type PK_check_state_t = c_int;

pub const PK_ASSEMBLY_state_corrupt_c: PK_check_state_t = 1;
pub const PK_BB_state_corrupt_c: PK_check_state_t = 2;
pub const PK_BODY_state_corrupt_c: PK_check_state_t = 3;
pub const PK_GEOM_state_corrupt_c: PK_check_state_t = 4;
pub const PK_PARTITION_state_corrupt_c: PK_check_state_t = 5;
pub const PK_SESSION_state_corrupt_c: PK_check_state_t = 6;
pub const PK_TRANSF_state_corrupt_c: PK_check_state_t = 7;
pub const PK_BODY_state_invalid_ident_c: PK_check_state_t = 8;

// =============================================================================
// Fault state tokens — Body Structure Faults (extra_faults detail)
// =============================================================================

pub const PK_BODY_state_bad_type_c: PK_check_state_t = 10;
pub const PK_BODY_state_no_region_c: PK_check_state_t = 11;
pub const PK_BODY_state_no_shell_c: PK_check_state_t = 12;
pub const PK_BODY_state_no_solid_reg_c: PK_check_state_t = 13;
pub const PK_BODY_state_region_1_solid_c: PK_check_state_t = 14;
pub const PK_BODY_state_shell_not_first_c: PK_check_state_t = 15;
pub const PK_BODY_state_wrong_ext_reg_c: PK_check_state_t = 16;
pub const PK_BODY_state_wrong_num_regs_c: PK_check_state_t = 17;
pub const PK_BODY_state_inside_out_c: PK_check_state_t = 18;
pub const PK_BODY_state_bad_regions_c: PK_check_state_t = 19;

// =============================================================================
// Fault state tokens — Geometry Faults
// =============================================================================

pub const PK_ENTITY_state_invalid_c: PK_check_state_t = 20;
pub const PK_GEOM_state_self_int_c: PK_check_state_t = 21;
pub const PK_GEOM_state_degenerate_c: PK_check_state_t = 22;
pub const PK_BCURVE_state_close_knots_c: PK_check_state_t = 23;
pub const PK_BSURF_state_close_u_knots_c: PK_check_state_t = 24;
pub const PK_BSURF_state_close_v_knots_c: PK_check_state_t = 25;
pub const PK_GEOM_state_bad_dep_type_c: PK_check_state_t = 26;
pub const PK_GEOM_state_bad_geom_owner_c: PK_check_state_t = 27;
pub const PK_GEOM_state_bad_owner_c: PK_check_state_t = 28;
pub const PK_GEOM_state_bad_topol_owner_c: PK_check_state_t = 29;
pub const PK_GEOM_state_dup_geom_owner_c: PK_check_state_t = 30;
pub const PK_GEOM_state_not_owners_dep_c: PK_check_state_t = 31;
pub const PK_GEOM_state_not_owning_dep_c: PK_check_state_t = 32;

// =============================================================================
// Fault state tokens — Topology Faults
// =============================================================================

pub const PK_TOPOL_state_no_geom_c: PK_check_state_t = 40;
pub const PK_TOPOL_state_not_G1_c: PK_check_state_t = 41;
pub const PK_TOPOL_state_bad_closed_c: PK_check_state_t = 42;
pub const PK_TOPOL_state_u_parm_degen_c: PK_check_state_t = 43;
pub const PK_TOPOL_state_v_parm_degen_c: PK_check_state_t = 44;
pub const PK_TOPOL_state_parm_degen_c: PK_check_state_t = 45;
pub const PK_TOPOL_state_u_phys_degen_c: PK_check_state_t = 46;
pub const PK_TOPOL_state_v_phys_degen_c: PK_check_state_t = 47;
pub const PK_TOPOL_state_phys_degen_c: PK_check_state_t = 48;
pub const PK_TOPOL_state_size_box_c: PK_check_state_t = 49;
pub const PK_TOPOL_state_check_fail_c: PK_check_state_t = 50;
pub const PK_TOPOL_state_bad_box_c: PK_check_state_t = 51;
pub const PK_TOPOL_state_bad_geom_share_c: PK_check_state_t = 52;
pub const PK_TOPOL_state_bad_owner_type_c: PK_check_state_t = 53;
pub const PK_TOPOL_state_share_no_geom_c: PK_check_state_t = 54;

// =============================================================================
// Fault state tokens — Edge Faults
// =============================================================================

pub const PK_EDGE_state_open_c: PK_check_state_t = 60;
pub const PK_EDGE_state_open_nmnl_c: PK_check_state_t = 61;
pub const PK_EDGE_state_bad_vertex_c: PK_check_state_t = 62;
pub const PK_EDGE_state_bad_vertex_nmnl_c: PK_check_state_t = 63;
pub const PK_EDGE_state_reversed_c: PK_check_state_t = 64;
pub const PK_EDGE_state_reversed_nmnl_c: PK_check_state_t = 65;
pub const PK_EDGE_state_bad_spcurve_c: PK_check_state_t = 66;
pub const PK_EDGE_state_bad_sp_nmnl_c: PK_check_state_t = 67;
pub const PK_EDGE_state_vertices_touch_c: PK_check_state_t = 68;
pub const PK_EDGE_state_bad_face_order_c: PK_check_state_t = 69;
pub const PK_EDGE_state_bad_polyline_c: PK_check_state_t = 70;
pub const PK_EDGE_state_bad_order_c: PK_check_state_t = 71;
pub const PK_EDGE_state_bad_wire_ed_ed_c: PK_check_state_t = 72;
pub const PK_EDGE_state_touch_edge_c: PK_check_state_t = 73;
pub const PK_EDGE_state_bad_fins_c: PK_check_state_t = 74;
pub const PK_EDGE_state_bad_tol_c: PK_check_state_t = 75;
pub const PK_EDGE_state_fin_bad_ring_c: PK_check_state_t = 76;
pub const PK_EDGE_state_fin_ed_next_pos_c: PK_check_state_t = 77;
pub const PK_EDGE_state_fin_ed_not_ed_c: PK_check_state_t = 78;
pub const PK_EDGE_state_fin_not_pos_c: PK_check_state_t = 79;
pub const PK_EDGE_state_single_vertex_c: PK_check_state_t = 80;
pub const PK_EDGE_state_wire_corrupt_c: PK_check_state_t = 81;

// =============================================================================
// Fault state tokens — Face Faults
// =============================================================================

pub const PK_FACE_state_bad_vertex_c: PK_check_state_t = 90;
pub const PK_FACE_state_bad_edge_c: PK_check_state_t = 91;
pub const PK_FACE_state_self_int_c: PK_check_state_t = 92;
pub const PK_FACE_state_bad_edge_order_c: PK_check_state_t = 93;
pub const PK_FACE_state_bad_loops_c: PK_check_state_t = 94;
pub const PK_FACE_state_redundant_c: PK_check_state_t = 95;
pub const PK_FACE_state_no_vtx_at_sing_c: PK_check_state_t = 96;
pub const PK_FACE_state_bad_wire_fa_ed_c: PK_check_state_t = 97;
pub const PK_FACE_state_bad_face_face_c: PK_check_state_t = 98;
pub const PK_FACE_state_check_fail_c: PK_check_state_t = 99;
pub const PK_FACE_state_diff_sh_same_rg_c: PK_check_state_t = 100;
pub const PK_FACE_state_edge_moebius_c: PK_check_state_t = 101;
pub const PK_FACE_state_wrong_sense_c: PK_check_state_t = 102;

// =============================================================================
// Fault state tokens — Curve Faults
// =============================================================================

pub const PK_CURVE_state_inconsistent_c: PK_check_state_t = 110;

// =============================================================================
// Fault state tokens — Loop Faults
// =============================================================================

pub const PK_LOOP_state_invalid_c: PK_check_state_t = 120;
pub const PK_LOOP_state_isolated_has_cu_c: PK_check_state_t = 121;

// =============================================================================
// Fault state tokens — Fin Faults
// =============================================================================

pub const PK_FIN_state_bad_c: PK_check_state_t = 130;
pub const PK_FIN_state_bad_ed_fins_c: PK_check_state_t = 131;
pub const PK_FIN_state_corrupt_c: PK_check_state_t = 132;
pub const PK_FIN_state_ed_next_is_fin_c: PK_check_state_t = 133;
pub const PK_FIN_state_edge_diff_vertex_c: PK_check_state_t = 134;
pub const PK_FIN_state_no_ed_next_in_sh_c: PK_check_state_t = 135;
pub const PK_FIN_state_non_zero_ident_c: PK_check_state_t = 136;
pub const PK_FIN_state_not_ed_next_prev_c: PK_check_state_t = 137;
pub const PK_FIN_state_not_lp_next_prev_c: PK_check_state_t = 138;
pub const PK_FIN_state_vx_not_vx_c: PK_check_state_t = 139;
pub const PK_FIN_state_wrong_vertex_c: PK_check_state_t = 140;

// =============================================================================
// Fault state tokens — Vertex Faults
// =============================================================================

pub const PK_VERTEX_state_bad_fin_c: PK_check_state_t = 150;
pub const PK_VERTEX_state_bad_isolated_c: PK_check_state_t = 151;
pub const PK_VERTEX_state_bad_tol_c: PK_check_state_t = 152;
pub const PK_VERTEX_state_fin_chains_c: PK_check_state_t = 153;
pub const PK_VERTEX_state_non_manifold_c: PK_check_state_t = 154;
pub const PK_VERTEX_state_not_dep_of_sh_c: PK_check_state_t = 155;
pub const PK_VERTEX_state_owner_not_sh_c: PK_check_state_t = 156;
pub const PK_VERTEX_state_sheet_corrupt_c: PK_check_state_t = 157;
pub const PK_VERTEX_state_too_many_eds_c: PK_check_state_t = 158;
pub const PK_VERTEX_state_wf_ed_bad_sh_c: PK_check_state_t = 159;

// =============================================================================
// Fault state tokens — Shell Faults
// =============================================================================

pub const PK_SHELL_state_bad_topol_geom_c: PK_check_state_t = 170;
pub const PK_SHELL_state_bad_sh_sh_c: PK_check_state_t = 171;
pub const PK_SHELL_state_bad_fa_orient_c: PK_check_state_t = 172;
pub const PK_SHELL_state_bad_wf_acorn_c: PK_check_state_t = 173;
pub const PK_SHELL_state_bad_wf_c: PK_check_state_t = 174;
pub const PK_SHELL_state_closed_bad_fa_c: PK_check_state_t = 175;
pub const PK_SHELL_state_ed_fa_in_acorn_c: PK_check_state_t = 176;
pub const PK_SHELL_state_eds_fragmented_c: PK_check_state_t = 177;
pub const PK_SHELL_state_fas_fragmented_c: PK_check_state_t = 178;
pub const PK_SHELL_state_no_vx_ed_fa_c: PK_check_state_t = 179;
pub const PK_SHELL_state_not_owning_vx_c: PK_check_state_t = 180;
pub const PK_SHELL_state_open_bad_fa_c: PK_check_state_t = 181;
pub const PK_SHELL_state_reg_meet_at_ed_c: PK_check_state_t = 182;
pub const PK_SHELL_state_reg_meet_at_vx_c: PK_check_state_t = 183;
pub const PK_SHELL_state_sheet_no_fa_c: PK_check_state_t = 184;
pub const PK_SHELL_state_too_many_fas_c: PK_check_state_t = 185;

// =============================================================================
// Fault state tokens — Region Faults
// =============================================================================

pub const PK_REGION_state_bad_shells_c: PK_check_state_t = 190;
pub const PK_REGION_state_no_shell_c: PK_check_state_t = 191;
pub const PK_REGION_state_wrongly_solid_c: PK_check_state_t = 192;

// =============================================================================
// Fault state tokens — Partition Faults
// =============================================================================

pub const PK_PARTITION_state_xref_c: PK_check_state_t = 200;

// =============================================================================
// Fault state tokens — Entity Faults
// =============================================================================

pub const PK_ENTITY_state_bad_owner_c: PK_check_state_t = 210;
pub const PK_ENTITY_state_has_att_group_c: PK_check_state_t = 211;
pub const PK_ENTITY_state_in_group_c: PK_check_state_t = 212;
pub const PK_ENTITY_state_shared_c: PK_check_state_t = 213;
pub const PK_ITEM_state_unattached_c: PK_check_state_t = 214;

// =============================================================================
// Fault state tokens — Part Faults
// =============================================================================

pub const PK_PART_state_bad_attrib_c: PK_check_state_t = 220;
pub const PK_PART_state_bad_attrib_list_c: PK_check_state_t = 221;
pub const PK_PART_state_bad_state_c: PK_check_state_t = 222;
pub const PK_PART_state_corrupt_c: PK_check_state_t = 223;
pub const PK_PART_state_has_key_c: PK_check_state_t = 224;
pub const PK_PART_state_no_key_c: PK_check_state_t = 225;

// =============================================================================
// Fault state tokens — Group Faults
// =============================================================================

pub const PK_GROUP_state_bad_c: PK_check_state_t = 230;

// =============================================================================
// Fault state tokens — Attribute Definition Faults
// =============================================================================

pub const PK_ATTDEF_state_bad_name_c: PK_check_state_t = 240;

// =============================================================================
// Fault state tokens — Attribute Faults (general)
// =============================================================================

pub const PK_ATTRIB_state_bad_string_c: PK_check_state_t = 250;

// =============================================================================
// Fault state tokens — System Attribute Check Faults
// =============================================================================

pub const PK_ATTRIB_state_bad_data_len_c: PK_check_state_t = 260;
pub const PK_ATTRIB_state_bad_ustring_c: PK_check_state_t = 261;
pub const PK_ATTRIB_state_byte_oor_c: PK_check_state_t = 262;
pub const PK_ATTRIB_state_empty_field_c: PK_check_state_t = 263;
pub const PK_ATTRIB_state_int_oor_c: PK_check_state_t = 264;
pub const PK_ATTRIB_state_invalid_att_c: PK_check_state_t = 265;
pub const PK_ATTRIB_state_non_unit_vec_c: PK_check_state_t = 266;
pub const PK_ATTRIB_state_real_oor_c: PK_check_state_t = 267;
pub const PK_ATTRIB_state_short_oor_c: PK_check_state_t = 268;
pub const PK_ATTRIB_state_p_vector_oor_c: PK_check_state_t = 269;

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
        edges: *const PK_EDGE_t,
        n_faults: *mut c_int,
        faults: *mut *mut PK_check_fault_t,
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
        options: *const PK_ENTITY_check_attribs_o_t,
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
