//! Mass / area / inertia oracle — the cheapest ground-truth signal.
//!
//! Wraps `PK_TOPOL_eval_mass_props`. At the default unit density this yields
//! exact closed-form invariants (volume, surface area, centre of gravity,
//! inertia) for the analytic primitives — a single-number comparison that
//! catches gross modelling errors fast. The option struct layout and enum
//! tokens were recovered from the DLL and validated end-to-end (see
//! `docs/pskernel-solidworks.md`).

use parasolid_sys::*;

use crate::body::Body;
use crate::error::PsResult;
use crate::geom::Vec3;

/// Mass properties of a body.
///
/// With the default body density of 1.0, `mass == amount`, so for solids both
/// equal the volume.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassProps {
    /// Volume for solids, area for sheets, length for wires.
    pub amount: f64,
    /// Integral of density over `amount`. Equals `amount` at unit density.
    pub mass: f64,
    /// Centre of gravity.
    pub center_of_gravity: Vec3,
    /// Inertia tensor about the centre of gravity, 3×3 row-major.
    pub inertia: [f64; 9],
    /// Boundary measure: total face area for solids, edge length for sheets.
    pub periphery: f64,
}

/// Default accuracy for [`Body::mass_props`].
///
/// Parasolid's useful range is `0.99..=0.999999`; analytic primitives evaluate
/// at essentially machine precision well before the top of that range.
pub const DEFAULT_MASS_ACCURACY: f64 = 0.999;

impl Body {
    /// Evaluate full mass properties at [`DEFAULT_MASS_ACCURACY`].
    pub fn mass_props(&self) -> PsResult<MassProps> {
        self.mass_props_with_accuracy(DEFAULT_MASS_ACCURACY)
    }

    /// The body's measure only: **volume** for solids, area for sheets, length
    /// for wires. Convenience over [`Body::mass_props`].
    pub fn volume(&self) -> PsResult<f64> {
        Ok(self.mass_props()?.amount)
    }

    /// The body's mass (integral of density over the measure). Equals
    /// [`Body::volume`] at the default unit density.
    pub fn mass(&self) -> PsResult<f64> {
        Ok(self.mass_props()?.mass)
    }

    /// Evaluate mass properties at a caller-specified accuracy (`0.0..=1.0`;
    /// useful range `0.99..=0.999999`).
    pub fn mass_props_with_accuracy(&self, accuracy: f64) -> PsResult<MassProps> {
        let topols: [PK_TOPOL_t; 1] = [self.tag];
        let opts = PK_TOPOL_eval_mass_props_o_t::default();

        let mut amount: f64 = 0.0;
        let mut mass: f64 = 0.0;
        let mut c_of_g: PK_VECTOR_t = [0.0; 3];
        let mut inertia: [f64; 9] = [0.0; 9];
        let mut periphery: f64 = 0.0;

        pk_call!(PK_TOPOL_eval_mass_props(
            1,
            topols.as_ptr(),
            accuracy,
            &opts,
            &mut amount,
            &mut mass,
            &mut c_of_g,
            &mut inertia,
            &mut periphery,
        ));

        Ok(MassProps {
            amount,
            mass,
            center_of_gravity: Vec3::from_pk(c_of_g),
            inertia,
            periphery,
        })
    }
}
