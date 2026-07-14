//! Boolean operations — unite, subtract, intersect.

use std::os::raw::c_int;
use parasolid_sys::*;
use crate::error::PsResult;
use crate::body::Body;
use crate::memory::PkArray;

// =============================================================================
// BooleanOp
// =============================================================================

/// Boolean operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOp {
    Unite,
    Subtract,
    Intersect,
}

// =============================================================================
// BooleanOptions
// =============================================================================

/// Options for boolean operations.
pub struct BooleanOptions {
    pub(crate) tracking: bool,
}

impl Default for BooleanOptions {
    fn default() -> Self {
        Self { tracking: false }
    }
}

impl BooleanOptions {
    pub fn new() -> Self { Self::default() }
    pub fn tracking(mut self, enable: bool) -> Self { self.tracking = enable; self }
}

// =============================================================================
// boolean
// =============================================================================

/// Perform a boolean operation: `target OP tools`.
///
/// # ⚠️ Not yet working — blocked on nested-option RE
///
/// The `PK_BODY_boolean_2` *signature* is correct, but the
/// `PK_BODY_boolean_o_t` options struct here does **not** match the kernel and
/// this call currently fails (`PK_ERROR_o_t_version_incorrect`, 5043).
/// Probed against pskernel.dll V37.01.243:
/// - Accepted `o_t_version` is **2..=19** (this struct defaults to 1).
/// - The version-2 *user* struct is only ~32 bytes —
///   `{ o_t_version, function@4, config_ptr@8, default_tol@16 (f64),
///   3 bytes@24..26, int@28 }` — not the 176-byte struct modelled here, whose
///   `max_tol`/material/imprint fields belong to later versions.
/// - `function` tokens are **0x3e1e/0x3e1f/0x3e20**, not 0/1/2.
/// - The struct nests further versioned sub-structs (`configuration`, and a
///   `local_opts` the arg-checker flags), which must be built before the call
///   succeeds.
///
/// Finishing this needs the boolean option-migration routine (`FUN_18049b860`)
/// fully mapped, including the nested sub-structs. Until then this returns the
/// kernel's error rather than a wrong result.
pub fn boolean(target: Body, tools: Vec<Body>, op: BooleanOp, options: &BooleanOptions) -> PsResult<Vec<Body>> {
    let tool_tags: Vec<PK_BODY_t> = tools.iter().map(|b| b.tag()).collect();

    let mut opts = PK_BODY_boolean_o_t {
        function: match op {
            BooleanOp::Unite => PK_boolean_unite_c,
            BooleanOp::Subtract => PK_boolean_subtract_c,
            BooleanOp::Intersect => PK_boolean_intersect_c,
        },
        ..PK_BODY_boolean_o_t::default()
    };
    opts.tracking = if options.tracking { PK_LOGICAL_true } else { PK_LOGICAL_false };

    let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
    let mut results: PK_boolean_r_t = unsafe { std::mem::zeroed() };

    let code = unsafe { PK_BODY_boolean_2(
        target.tag(),
        tool_tags.len() as c_int,
        tool_tags.as_ptr(),
        &opts,
        &mut tracking,
        &mut results,
    ) };

    // Always free tracking data, even on error — PK may have partially
    // populated it before returning an error code.
    unsafe { PK_TOPOL_track_r_f(&mut tracking) };

    // Check the boolean result code after freeing tracking.
    crate::error::pk_check(code)?;

    // Wrap bodies in PkArray BEFORE any further fallible operations so
    // the PK-allocated array is freed on drop regardless of error paths.
    let bodies = unsafe { PkArray::from_raw(results.bodies, results.n_bodies) };
    let result_bodies: Vec<Body> = bodies.iter().map(|&tag| Body::from_tag(tag)).collect();

    Ok(result_bodies)
}
