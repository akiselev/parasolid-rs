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
// Matched regions
// =============================================================================

/// `PK_boolean_match_t`: how the two entities of a [`MatchedRegion`] relate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchType {
    Exact,
    Contains,
    /// The manual recommends this for face/face and face/edge pairs.
    Overlap,
    Imprinted,
}

impl MatchType {
    fn raw(self) -> PK_boolean_match_type_t {
        match self {
            Self::Exact => PK_boolean_match_exact_c,
            Self::Contains => PK_boolean_match_contains_c,
            Self::Overlap => PK_boolean_match_overlap_c,
            Self::Imprinted => PK_boolean_match_imprinted_c,
        }
    }
}

/// One supplied matched pair (`PK_boolean_match_region_o_t`): a target and a
/// tool topology tag the Boolean treats as coincident within `tolerance`.
/// Result edges along an overlap pair carry exactly `tolerance`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatchedRegion {
    pub target: i32,
    pub tool: i32,
    pub match_type: MatchType,
    pub tolerance: f64,
}

// =============================================================================
// BooleanOptions
// =============================================================================

/// Options for boolean operations.
pub struct BooleanOptions {
    pub(crate) tracking: bool,
    pub(crate) default_tol: Option<f64>,
    pub(crate) merge_imprinted: bool,
    pub(crate) selective_merge: bool,
    /// `Some` selects `PK_boolean_match_style_auto_c`; the inner value is the
    /// optional `auto_match_tol`.
    pub(crate) auto_match: Option<Option<f64>>,
    pub(crate) match_regions: Vec<MatchedRegion>,
}

impl Default for BooleanOptions {
    fn default() -> Self {
        Self {
            tracking: false,
            default_tol: None,
            merge_imprinted: false,
            selective_merge: false,
            auto_match: None,
            match_regions: Vec::new(),
        }
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
    /// Coincidence tolerance for the Boolean (`default_tol` @48 of the v19
    /// options). Tolerant result edges carry exactly this value. `None`
    /// (the default) keeps the kernel's 1e-5.
    pub fn default_tol(mut self, tol: f64) -> Self {
        self.default_tol = Some(tol);
        self
    }
    /// Merge the mergeable edges the Boolean imprinted on the result
    /// (`merge_imprinted` @32 of the v19 options). Default `false`: the
    /// kernel keeps coplanar regions as separate faces with imprinted edges.
    pub fn merge_imprinted(mut self, enable: bool) -> Self {
        self.merge_imprinted = enable;
        self
    }
    /// With [`Self::merge_imprinted`], leave alone edges that were already
    /// mergeable before the Boolean (`selective_merge` @41 of the v19
    /// options). Default `false`.
    pub fn selective_merge(mut self, enable: bool) -> Self {
        self.selective_merge = enable;
        self
    }
    /// Let the kernel find matched regions itself (`match_style` auto in the
    /// v5 `matched_region` sub-struct), optionally within `tol`
    /// (`auto_match_tol`). Measured on two 10 mm blocks 2e-5 apart: without
    /// a tolerance they stay apart; a tolerance above the gap unites them.
    /// Auto matching did not bridge gaps of 3e-5 or more at any tolerance.
    pub fn auto_match(mut self, tol: Option<f64>) -> Self {
        self.auto_match = Some(tol);
        self
    }
    /// Supply a matched pair (`match_regions` in the `matched_region`
    /// sub-struct). Can be called repeatedly.
    pub fn match_region(mut self, region: MatchedRegion) -> Self {
        self.match_regions.push(region);
        self
    }
}

// =============================================================================
// boolean
// =============================================================================

/// `PK_boolean_result_t` — the outcome token `PK_BODY_boolean_2` writes even
/// when it returns `PK_ERROR_no_errors`. Values: vendor table, band
/// 0x5492..=0x5499 confirmed in the binary; `Success`, `Failed` and `NoClash`
/// dynamic-observed (parasolid-re oracle runs; `boolean_failed_is_an_error`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanResult {
    Success,
    NoClash,
    NoEffect,
    Failed,
    NotSolid,
    Imprint,
    /// Deprecated in the kernel.
    Multiple,
    Unknown(i32),
}

impl BooleanResult {
    pub fn from_raw(raw: i32) -> Self {
        match raw {
            PK_boolean_result_success_c => Self::Success,
            PK_boolean_result_no_clash_c => Self::NoClash,
            PK_boolean_result_no_effect_c => Self::NoEffect,
            PK_boolean_result_failed_c => Self::Failed,
            PK_boolean_result_not_solid_c => Self::NotSolid,
            PK_boolean_result_imprint_c => Self::Imprint,
            PK_boolean_result_multiple_c => Self::Multiple,
            other => Self::Unknown(other),
        }
    }
}

/// One `PK_boolean_report_r_t`: the report token and the entities it names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanReport {
    /// `PK_boolean_report_t` token (e.g. 21605 `non_manifold`).
    pub report: i32,
    /// Entity tags named by the report (may no longer exist after the call).
    pub entities: Vec<i32>,
}

/// The whole `PK_boolean_r_t`, copied out before `PK_boolean_r_f`.
#[derive(Debug, Clone)]
pub struct BooleanOutcome {
    /// The raw result token, decoded.
    pub result: BooleanResult,
    /// The raw `PK_boolean_result_t` value.
    pub result_raw: i32,
    /// Result bodies (the target is replaced in place and listed here).
    pub bodies: Vec<Body>,
    /// The kernel's reports.
    pub reports: Vec<BooleanReport>,
}

/// Perform a boolean operation: `target OP tools`.
///
/// Uses `PK_BODY_boolean_2` with the version-19 options struct (see
/// [`PK_BODY_boolean_o_t`]). Returns the result bodies.
///
/// **Errors** with [`crate::PsError::BooleanFailed`] when the kernel's result
/// token is `PK_boolean_result_failed_c`, even though the call itself returned
/// no error. Other non-success tokens (`no_clash`, `no_effect`, `not_solid`,
/// `imprint`) are *not* errors here: the bodies the kernel returned are passed
/// through. Use [`boolean_detailed`] to see the token and reports.
pub fn boolean(
    target: Body,
    tools: Vec<Body>,
    op: BooleanOp,
    options: &BooleanOptions,
) -> PsResult<Vec<Body>> {
    let outcome = boolean_detailed(target, tools, op, options)?;
    if outcome.result == BooleanResult::Failed {
        return Err(crate::error::PsError::BooleanFailed {
            result: outcome.result_raw,
            reports: outcome
                .reports
                .into_iter()
                .map(|r| (r.report, r.entities))
                .collect(),
        });
    }
    Ok(outcome.bodies)
}

/// `PK_BODY_boolean_2` returning the raw result token, bodies and reports.
/// Only a non-zero `PK_ERROR_code_t` is an `Err`; a `failed` result token is
/// returned as data.
pub fn boolean_detailed(
    target: Body,
    tools: Vec<Body>,
    op: BooleanOp,
    options: &BooleanOptions,
) -> PsResult<BooleanOutcome> {
    let tool_tags: Vec<PK_BODY_t> = tools.iter().map(|b| b.tag()).collect();

    let regions: Vec<PK_boolean_match_region_o_t> = options
        .match_regions
        .iter()
        .map(|r| PK_boolean_match_region_o_t {
            o_t_version: 1,
            regions: [r.target, r.tool],
            match_type: r.match_type.raw(),
            tolerance: r.tolerance,
        })
        .collect();
    let mut matched = PK_boolean_match_o_t::default();
    if let Some(tol) = options.auto_match {
        matched.match_style = PK_boolean_match_style_auto_c;
        if let Some(t) = tol {
            matched.auto_match = 1;
            matched.auto_match_tol = t;
        }
    }
    if !regions.is_empty() {
        matched.n_match_regions = regions.len() as c_int;
        matched.match_regions = regions.as_ptr();
    }
    let use_matched = options.auto_match.is_some() || !regions.is_empty();

    let opts = PK_BODY_boolean_o_t {
        function: match op {
            BooleanOp::Unite => PK_boolean_unite_c,
            BooleanOp::Subtract => PK_boolean_subtract_c,
            BooleanOp::Intersect => PK_boolean_intersect_c,
        },
        default_tol: options
            .default_tol
            .unwrap_or(PK_BODY_boolean_o_t::default().default_tol),
        merge_imprinted: options.merge_imprinted as PK_LOGICAL_t,
        selective_merge: options.selective_merge as PK_LOGICAL_t,
        matched_region: if use_matched {
            &matched
        } else {
            std::ptr::null()
        },
        ..PK_BODY_boolean_o_t::default()
    };

    let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
    let mut results: PK_boolean_r_t = unsafe { std::mem::zeroed() };

    let code = crate::budget::guard(|| unsafe {
        PK_BODY_boolean_2(
            target.tag(),
            tool_tags.len() as c_int,
            tool_tags.as_ptr(),
            &opts,
            &mut tracking,
            &mut results,
        )
    });

    // Always free tracking data, even on error — PK may have partially
    // populated it before returning an error code.
    unsafe { PK_TOPOL_track_r_f(&mut tracking) };

    // Copy everything out before freeing: `bodies` and the report entity
    // arrays are kernel-allocated and `PK_boolean_r_f` releases them.
    let bodies: Vec<Body> = if results.bodies.is_null() || results.n_bodies <= 0 {
        Vec::new()
    } else {
        (0..results.n_bodies as usize)
            .map(|i| Body::from_tag(unsafe { *results.bodies.add(i) }))
            .collect()
    };
    let reports: Vec<BooleanReport> = if results.faults.is_null() || results.n_faults <= 0 {
        Vec::new()
    } else {
        let rs = unsafe {
            std::slice::from_raw_parts(
                results.faults as *const PK_boolean_report_r_t,
                results.n_faults as usize,
            )
        };
        rs.iter()
            .map(|r| BooleanReport {
                report: r.report,
                entities: if r.entities.is_null() || r.n_entities <= 0 {
                    Vec::new()
                } else {
                    unsafe { std::slice::from_raw_parts(r.entities, r.n_entities as usize) }
                        .to_vec()
                },
            })
            .collect()
    };
    let result_raw = results.result;

    unsafe { PK_boolean_r_f(&mut results) };

    // Check the error code only after both frees, so an error path cannot leak
    // either allocation.
    crate::error::pk_check(code?)?;

    Ok(BooleanOutcome {
        result: BooleanResult::from_raw(result_raw),
        result_raw,
        bodies,
        reports,
    })
}
