//! Mass properties evaluation.
//!
//! Bindings for `PK_TOPOL_eval_mass_props` and related option types (Chapter 25).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_double, c_int};

use crate::*;

// =============================================================================
// Option enums
// =============================================================================

// Token values below are [probed] against pskernel.dll V37.01.243: recovered
// statically from the option-version-migration routine (`FUN_180441cd0`, which
// initialises the internal option struct with these defaults) and confirmed
// dynamically — each `PK_mass_t` level adds exactly one output
// (amount → +mass → +c_of_g → +m_of_i), and periphery no/yes toggles the
// periphery output. The earlier 0/1/2/3 guesses were wrong.

/// What mass data to compute.
pub type PK_mass_t = c_int;
/// Do not find any data. [probed]
pub const PK_mass_no_c: PK_mass_t = 0x36b1; // 14001
/// Find mass and amount only. [probed]
pub const PK_mass_mass_c: PK_mass_t = 0x36b2; // 14002
/// Find centre of gravity + mass + amount. [probed]
pub const PK_mass_c_of_g_c: PK_mass_t = 0x36b3; // 14003
/// Find moment of inertia + centre of gravity + mass + amount (default). [probed]
pub const PK_mass_m_of_i_c: PK_mass_t = 0x36b4; // 14004

/// Whether to compute periphery (boundary measure).
pub type PK_mass_periphery_t = c_int;
/// Do not calculate periphery. [probed]
pub const PK_mass_periphery_no_c: PK_mass_periphery_t = 0x36b5; // 14005
/// Calculate periphery (default). [probed]
pub const PK_mass_periphery_yes_c: PK_mass_periphery_t = 0x36b6; // 14006

/// Error bound reporting mode.
pub type PK_mass_bound_t = c_int;
/// No error bounds (default). [probed]
pub const PK_mass_bound_no_c: PK_mass_bound_t = 0x36b7; // 14007
/// Error as value +/- modulus. [family — extrapolated, not individually probed]
pub const PK_mass_bound_modulus_c: PK_mass_bound_t = 0x36b8; // 14008
/// Error as value + interval. [family — extrapolated, not individually probed]
pub const PK_mass_bound_interval_c: PK_mass_bound_t = 0x36b9; // 14009

/// Local density handling for same-dimension sub-entities.
///
/// Only present in option-struct versions ≥ 4; the internal default token is
/// `0x61fb` (25083). Individual values are [guess] — not needed by the
/// version-1 struct this crate uses for [`Body::mass_props`].
pub type PK_mass_local_density_t = c_int;
/// Local density added to body density. [guess]
pub const PK_mass_local_density_additive_c: PK_mass_local_density_t = 0;
/// Ignore local densities. [guess]
pub const PK_mass_local_density_ignore_c: PK_mass_local_density_t = 1;
/// Not specified (default). [guess]
pub const PK_mass_local_density_unset_c: PK_mass_local_density_t = 2;
/// Local density overrides body density (same_dim_density only). [guess]
pub const PK_mass_local_density_override_c: PK_mass_local_density_t = 3;

/// Behaviour when mass equals zero. [guess — only present in later versions]
pub type PK_mass_eq_0_t = c_int;
/// Stop and return error on zero mass (default). [guess]
pub const PK_mass_eq_0_fail_c: PK_mass_eq_0_t = 0;
/// Continue, report zero-mass topologies via report stream. [guess]
pub const PK_mass_eq_0_report_c: PK_mass_eq_0_t = 1;

// Report constants for zero-mass reporting.
pub type PK_REPORT_record_type_t = c_int;
/// Report record type for mass_eq_0 reports.
pub const PK_REPORT_record_type_3_c: PK_REPORT_record_type_t = 3;
/// Status code for zero mass.
pub const PK_REPORT_3_mass_eq_0_c: c_int = 1;

// =============================================================================
// Options structure
// =============================================================================

/// Options for `PK_TOPOL_eval_mass_props`, **version-1 layout** [probed].
///
/// Recovered from pskernel.dll V37.01.243: the option-version-migration routine
/// (`FUN_180441cd0`) reads exactly these five fields when `o_t_version == 1`
/// (user offsets 0/4/8/12/16), and a call built from this struct returns the
/// correct amount / mass / centre-of-gravity / inertia / periphery for the
/// analytic primitives — with `check_arguments` on.
///
/// The public kernel struct has *more* fields at higher `o_t_version`s
/// (`single`/`use_facets`/`facet_tol`/`same_dim_density`/`lower_dim_density`/
/// `n_transfs`/`transfs`/`mass_eq_0`/scale controls/`local_opts`). Those are not
/// modelled here — set `o_t_version = 1` and the kernel migrates this struct to
/// its internal latest form, applying documented defaults for the rest. Using a
/// higher version requires the full layout from a header audit.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_eval_mass_props_o_t {
    /// Structure version. Must be `1` for this layout.
    pub o_t_version: c_int,
    /// What mass data to compute (`PK_mass_*_c`).
    pub mass: PK_mass_t,
    /// Whether to compute periphery (`PK_mass_periphery_*_c`).
    pub periphery: PK_mass_periphery_t,
    /// Error bound mode (`PK_mass_bound_*_c`).
    pub bound: PK_mass_bound_t,
    /// Treat an array of faces / sheet bodies as a single solid.
    pub single: PK_LOGICAL_t,
}

impl Default for PK_TOPOL_eval_mass_props_o_t {
    /// Version-1 defaults matching the kernel's own initialisation: compute
    /// everything (`m_of_i`), include periphery, no error bounds.
    fn default() -> Self {
        Self {
            o_t_version: 1,
            mass: PK_mass_m_of_i_c,
            periphery: PK_mass_periphery_yes_c,
            bound: PK_mass_bound_no_c,
            single: PK_LOGICAL_false,
        }
    }
}

// =============================================================================
// Extern function declaration
// =============================================================================

#[link(name = "pskernel")]
unsafe extern "C" {
    /// Evaluate mass properties for topological entities.
    ///
    /// All entities in `topols` must be the same topological type.
    ///
    /// # Signature note
    ///
    /// [documented] by the PK reference and [probed] against pskernel.dll
    /// V37.01.243: the `options` pointer is the 4th argument, before the five
    /// output pointers. `options` must point at a valid, version-tagged
    /// [`PK_TOPOL_eval_mass_props_o_t`] — pass `o_t_version = 1` and the crate's
    /// version-1 struct. Validated end-to-end (amount / mass / c_of_g / m_of_i /
    /// periphery) against closed-form values with `check_arguments` on.
    ///
    /// # Arguments
    ///
    /// * `n_topols`  - Number of topological entities.
    /// * `topols`    - Array of topological entity tags.
    /// * `accuracy`  - Accuracy control (0.0..1.0; useful range 0.99..0.999999).
    /// * `options`   - Options structure (`PK_TOPOL_eval_mass_props_o_t`).
    /// * `amount`    - (out) Volume for solids, area for sheets, length for wires.
    /// * `mass`      - (out) Integral of density * amount.
    /// * `c_of_g`    - (out) Centre of gravity [x, y, z].
    /// * `m_of_i`    - (out) 3x3 inertia tensor about centre of gravity (row-major).
    /// * `periphery` - (out) Boundary measure (face area for solids, edge length for sheets).
    pub fn PK_TOPOL_eval_mass_props(
        n_topols: c_int,
        topols: *const PK_TOPOL_t,
        accuracy: c_double,
        options: *const PK_TOPOL_eval_mass_props_o_t,
        amount: *mut c_double,
        mass: *mut c_double,
        c_of_g: *mut PK_VECTOR_t,
        m_of_i: *mut [c_double; 9],
        periphery: *mut c_double,
    ) -> PK_ERROR_code_t;
}
