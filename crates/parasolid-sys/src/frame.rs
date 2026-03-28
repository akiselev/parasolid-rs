//! Frame operations — coordinate frames attached to topology.
//!
//! Frames (`PK_FRAME_t`, class 0xe6) carry geometry references (surf/curve/point)
//! and an orientation sense. Session-only entities, not serialized to XT format.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use crate::*;
use std::os::raw::c_int;

#[link(name = "pskernel")]
unsafe extern "C" {
    /// Get owning body of frame.
    pub fn PK_FRAME_ask_body(
        frame: PK_FRAME_t,
        body: *mut PK_BODY_t,
    ) -> PK_ERROR_code_t;

    /// Get geometry attached to frame.
    pub fn PK_FRAME_ask_geometry(
        frame: PK_FRAME_t,
        options: *mut c_int,
        results: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Get owning topology and its class.
    pub fn PK_FRAME_ask_owner(
        frame: PK_FRAME_t,
        owner_tag: *mut c_int,
        owner_class: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Get frame orientation sense.
    pub fn PK_FRAME_ask_sense(
        frame: PK_FRAME_t,
        sense: *mut PK_LOGICAL_t,
    ) -> PK_ERROR_code_t;

    /// Attach surface/curve/point to frame.
    pub fn PK_FRAME_attach_geoms(
        frame: PK_FRAME_t,
        surface: PK_SURF_t,
        curve: PK_CURVE_t,
        point: PK_POINT_t,
        options: *mut c_int,
        results: *mut c_int,
    ) -> PK_ERROR_code_t;

    /// Reverse frame sense.
    pub fn PK_FRAME_reverse(
        frame: PK_FRAME_t,
        options: *mut c_int,
        new_frame: *mut PK_FRAME_t,
    ) -> PK_ERROR_code_t;

    // =========================================================================
    // Result-free functions
    // =========================================================================

    /// Free results from `PK_FRAME_attach_geoms`.
    pub fn PK_FRAME_attach_geoms_r_f(results: *mut c_int) -> PK_ERROR_code_t;

    /// Free results from `PK_FRAME_ask_geometry`.
    pub fn PK_FRAME_ask_geometry_r_f(results: *mut c_int) -> PK_ERROR_code_t;

    /// Free results from `PK_FRAME_ask_owner`.
    pub fn PK_FRAME_ask_owner_r_f(results: *mut c_int) -> PK_ERROR_code_t;

    /// Free results from `PK_FRAME_reverse`.
    pub fn PK_FRAME_reverse_r_f(results: *mut c_int) -> PK_ERROR_code_t;

    /// Free results from `PK_TOPOL_find_frames`.
    pub fn PK_TOPOL_find_frames_r_f(n_frames: c_int, frames: *mut PK_FRAME_t) -> PK_ERROR_code_t;

    /// Free results from `PK_TOPOL_imprint_frames`.
    pub fn PK_TOPOL_imprint_frames_r_f(results: *mut c_int) -> PK_ERROR_code_t;

}
