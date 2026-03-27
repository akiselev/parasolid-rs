//! Mass properties evaluation.
//!
//! Bindings for `PK_TOPOL_eval_mass_props` and related option types (Chapter 25).

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_double, c_int};

use crate::*;

// =============================================================================
// Option enums
// =============================================================================

/// What mass data to compute.
pub type PK_mass_t = c_int;
/// Do not find any data.
pub const PK_mass_no_c: PK_mass_t = 0;
/// Find mass and amount only.
pub const PK_mass_mass_c: PK_mass_t = 1;
/// Find centre of gravity + mass + amount.
pub const PK_mass_c_of_g_c: PK_mass_t = 2;
/// Find moment of inertia + centre of gravity + mass + amount (default).
pub const PK_mass_m_of_i_c: PK_mass_t = 3;

/// Whether to compute periphery (boundary measure).
pub type PK_mass_periphery_t = c_int;
/// Do not calculate periphery.
pub const PK_mass_periphery_no_c: PK_mass_periphery_t = 0;
/// Calculate periphery (default).
pub const PK_mass_periphery_yes_c: PK_mass_periphery_t = 1;

/// Error bound reporting mode.
pub type PK_mass_bound_t = c_int;
/// No error bounds (default).
pub const PK_mass_bound_no_c: PK_mass_bound_t = 0;
/// Error as value +/- modulus.
pub const PK_mass_bound_modulus_c: PK_mass_bound_t = 1;
/// Error as value + interval.
pub const PK_mass_bound_interval_c: PK_mass_bound_t = 2;

/// Local density handling for same-dimension sub-entities.
pub type PK_mass_local_density_t = c_int;
/// Local density added to body density.
pub const PK_mass_local_density_additive_c: PK_mass_local_density_t = 0;
/// Ignore local densities.
pub const PK_mass_local_density_ignore_c: PK_mass_local_density_t = 1;
/// Not specified (default).
pub const PK_mass_local_density_unset_c: PK_mass_local_density_t = 2;
/// Local density overrides body density (same_dim_density only).
pub const PK_mass_local_density_override_c: PK_mass_local_density_t = 3;

/// Behaviour when mass equals zero.
pub type PK_mass_eq_0_t = c_int;
/// Stop and return error on zero mass (default).
pub const PK_mass_eq_0_fail_c: PK_mass_eq_0_t = 0;
/// Continue, report zero-mass topologies via report stream.
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

/// Options for `PK_TOPOL_eval_mass_props`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PK_TOPOL_eval_mass_props_o_t {
    /// Version tag for structure compatibility.
    pub o_t_version: c_int,
    /// What mass data to compute.
    pub mass: PK_mass_t,
    /// Whether to compute periphery.
    pub periphery: PK_mass_periphery_t,
    /// Error bound mode.
    pub bound: PK_mass_bound_t,
    /// Treat array of faces/sheet bodies as a single solid.
    pub single: PK_LOGICAL_t,
    /// Use alternative faceting method.
    pub use_facets: PK_LOGICAL_t,
    /// Chordal tolerance for faceting method.
    pub facet_tol: c_double,
    /// Local density handling for same-dimension sub-entities.
    pub same_dim_density: PK_mass_local_density_t,
    /// Local density handling for lower-dimension sub-entities.
    pub lower_dim_density: PK_mass_local_density_t,
    /// Number of transformations.
    pub n_transfs: c_int,
    /// Array of transforms applied to topols before calculation.
    pub transfs: *const PK_TRANSF_t,
    /// Behaviour when mass equals zero.
    pub mass_eq_0: PK_mass_eq_0_t,
}

impl Default for PK_TOPOL_eval_mass_props_o_t {
    fn default() -> Self {
        Self {
            o_t_version: 1,
            mass: PK_mass_m_of_i_c,
            periphery: PK_mass_periphery_yes_c,
            bound: PK_mass_bound_no_c,
            single: PK_LOGICAL_false,
            use_facets: PK_LOGICAL_false,
            facet_tol: 0.0,
            same_dim_density: PK_mass_local_density_unset_c,
            lower_dim_density: PK_mass_local_density_unset_c,
            n_transfs: 0,
            transfs: std::ptr::null(),
            mass_eq_0: PK_mass_eq_0_fail_c,
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
    /// # Arguments
    ///
    /// * `n_topols`  - Number of topological entities.
    /// * `topols`    - Array of topological entity tags.
    /// * `accuracy`  - Accuracy control (0.0..1.0; useful range 0.99..0.999999).
    /// * `options`   - Options structure.
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
