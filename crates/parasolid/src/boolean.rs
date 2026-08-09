//! Boolean operations — unite, subtract, intersect.

use crate::body::Body;
use crate::error::PsResult;
use parasolid_sys::*;
use std::os::raw::c_int;

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
    pub fn new() -> Self {
        Self::default()
    }
    pub fn tracking(mut self, enable: bool) -> Self {
        self.tracking = enable;
        self
    }
}

// =============================================================================
// boolean
// =============================================================================

/// Perform a boolean operation: `target OP tools`.
///
/// Uses `PK_BODY_boolean_2` with the version-2 options struct (see
/// [`PK_BODY_boolean_o_t`]). Tracking data is always requested through the
/// `tracking` output argument and freed before returning.
pub fn boolean(
    target: Body,
    tools: Vec<Body>,
    op: BooleanOp,
    _options: &BooleanOptions,
) -> PsResult<Vec<Body>> {
    let tool_tags: Vec<PK_BODY_t> = tools.iter().map(|b| b.tag()).collect();

    let opts = PK_BODY_boolean_o_t {
        function: match op {
            BooleanOp::Unite => PK_boolean_unite_c,
            BooleanOp::Subtract => PK_boolean_subtract_c,
            BooleanOp::Intersect => PK_boolean_intersect_c,
        },
        ..PK_BODY_boolean_o_t::default()
    };

    let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
    let mut results: PK_boolean_r_t = unsafe { std::mem::zeroed() };

    let code = unsafe {
        PK_BODY_boolean_2(
            target.tag(),
            tool_tags.len() as c_int,
            tool_tags.as_ptr(),
            &opts,
            &mut tracking,
            &mut results,
        )
    };

    // Always free tracking data, even on error — PK may have partially
    // populated it before returning an error code.
    unsafe { PK_TOPOL_track_r_f(&mut tracking) };

    // Copy the tags out before freeing: `bodies` is kernel-allocated and
    // `PK_boolean_r_f` releases it along with whatever else the result struct
    // owns. Body is a plain tag, so the copy outlives the free.
    let result_bodies: Vec<Body> = if results.bodies.is_null() || results.n_bodies <= 0 {
        Vec::new()
    } else {
        (0..results.n_bodies as usize)
            .map(|i| Body::from_tag(unsafe { *results.bodies.add(i) }))
            .collect()
    };

    // Free the whole result struct through its matching API rather than just
    // the `bodies` array — the struct carries further kernel allocations that a
    // bare array free leaks.
    unsafe { PK_boolean_r_f(&mut results) };

    // Check the boolean result code only after both frees, so an error path
    // cannot leak either allocation.
    crate::error::pk_check(code)?;

    Ok(result_bodies)
}
