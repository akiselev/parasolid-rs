//! Mass / measurement oracle — the cheapest ground-truth signal.
//!
//! Wraps `PK_TOPOL_eval_mass_props`. Only the **volume/amount** and **mass**
//! outputs are currently validated against closed-form values (block, sphere,
//! cylinder, cone, torus all match to ~1e-6 relative). Centre-of-gravity,
//! inertia and periphery need the real options-struct field offsets, which are
//! not yet modelled — see [`parasolid_sys::PK_TOPOL_eval_mass_props_o_t`] and
//! `TODO.md` (P5). They are intentionally not exposed here rather than returned
//! as plausible-looking zeros.

use std::os::raw::c_void;

use parasolid_sys::*;

use crate::body::Body;
use crate::error::PsResult;

/// Default accuracy for [`Body::volume`].
///
/// Parasolid's useful range is `0.99..=0.999999`; analytic primitives evaluate
/// at essentially machine precision well before the top of that range.
pub const DEFAULT_MASS_ACCURACY: f64 = 0.999;

/// Size of the zeroed options buffer passed to `PK_TOPOL_eval_mass_props`.
///
/// The real options struct is larger than the (incomplete) Rust definition and
/// contains fields we do not model. We pass an over-sized, zeroed buffer with
/// only `o_t_version` set so every field the kernel reads defaults to
/// zero/null — the empirically-verified way to get a correct `amount`/`mass`
/// without crashing. 256 bytes is comfortable headroom over the true size.
const MASS_OPTS_BUF_LEN: usize = 256;

/// The options version the kernel accepts (probed range is 1..=7).
const MASS_OPTS_VERSION: i32 = 1;

impl Body {
    /// The body's measure at [`DEFAULT_MASS_ACCURACY`]: **volume** for solids,
    /// surface area for sheets, edge length for wires.
    ///
    /// At the default unit density this equals `mass`. Validated against
    /// closed-form values for the analytic primitives.
    pub fn volume(&self) -> PsResult<f64> {
        Ok(self.eval_mass_amount(DEFAULT_MASS_ACCURACY)?.0)
    }

    /// The body's mass (integral of density over the measure). Equals
    /// [`Body::volume`] at the default unit density.
    pub fn mass(&self) -> PsResult<f64> {
        Ok(self.eval_mass_amount(DEFAULT_MASS_ACCURACY)?.1)
    }

    /// Raw `(amount, mass)` at a caller-specified accuracy (`0.0..=1.0`; useful
    /// range `0.99..=0.999999`).
    ///
    /// Only `amount` and `mass` are validated; the kernel's centre-of-gravity,
    /// inertia and periphery outputs are not returned here (see module docs).
    pub fn eval_mass_amount(&self, accuracy: f64) -> PsResult<(f64, f64)> {
        let topols: [PK_TOPOL_t; 1] = [self.tag];

        // Zeroed, over-sized options buffer with only o_t_version set. See
        // MASS_OPTS_BUF_LEN. `align(8)` so the double/pointer fields the kernel
        // reads inside it are correctly aligned.
        #[repr(C, align(8))]
        struct OptsBuf([u8; MASS_OPTS_BUF_LEN]);
        let mut opts = OptsBuf([0u8; MASS_OPTS_BUF_LEN]);
        opts.0[0..4].copy_from_slice(&MASS_OPTS_VERSION.to_le_bytes());

        let mut amount: f64 = 0.0;
        let mut mass: f64 = 0.0;
        let mut c_of_g: PK_VECTOR_t = [0.0; 3];
        let mut inertia: [f64; 9] = [0.0; 9];
        let mut periphery: f64 = 0.0;

        // The kernel's argument checker rejects our incomplete options struct
        // (`PK_ERROR_field_of_wrong_type` on the unmodelled `local_opts` field),
        // even though the call itself computes the correct amount. Disable arg
        // checking just for this call and restore the caller's setting after.
        let _guard = CheckArgsGuard::disable()?;

        pk_call!(PK_TOPOL_eval_mass_props(
            1,
            topols.as_ptr(),
            accuracy,
            &opts as *const OptsBuf as *const c_void,
            &mut amount,
            &mut mass,
            &mut c_of_g,
            &mut inertia,
            &mut periphery,
        ));

        Ok((amount, mass))
    }
}

/// Saves the session's `check_arguments` flag, sets it to `false`, and restores
/// the saved value on drop. Used around the mass-props call whose (incomplete)
/// options struct the argument checker rejects.
struct CheckArgsGuard {
    previous: PK_LOGICAL_t,
}

impl CheckArgsGuard {
    fn disable() -> PsResult<Self> {
        let mut previous: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_SESSION_ask_check_arguments(&mut previous));
        pk_call!(PK_SESSION_set_check_arguments(PK_LOGICAL_false));
        Ok(CheckArgsGuard { previous })
    }
}

impl Drop for CheckArgsGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = PK_SESSION_set_check_arguments(self.previous);
        }
    }
}
