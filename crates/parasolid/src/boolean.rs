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

// =============================================================================
// Stage-level Boolean oracles: face sets, face Boolean, section with sheet
// =============================================================================

fn op_token(op: BooleanOp) -> PK_boolean_function_t {
    match op {
        BooleanOp::Unite => PK_boolean_unite_c,
        BooleanOp::Subtract => PK_boolean_subtract_c,
        BooleanOp::Intersect => PK_boolean_intersect_c,
    }
}

fn copy_tags(p: *const i32, n: c_int) -> Vec<i32> {
    if p.is_null() || n <= 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(p, n as usize) }.to_vec()
    }
}

fn copy_face_sets(p: *const PK_FACE_array_t, n: c_int) -> Vec<Vec<crate::face::Face>> {
    if p.is_null() || n <= 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(p, n as usize) }
        .iter()
        .map(|a| copy_tags(a.array, a.length).into_iter().map(crate::face::Face::from_tag).collect())
        .collect()
}

fn copy_reports(p: *const PK_boolean_report_r_t, n: c_int) -> Vec<BooleanReport> {
    if p.is_null() || n <= 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(p, n as usize) }
        .iter()
        .map(|r| BooleanReport { report: r.report, entities: copy_tags(r.entities, r.n_entities) })
        .collect()
}

/// Result of [`identify_facesets`] (`PK_identify_facesets_r_t`): the faces of
/// each body that survive or are rejected by the Boolean `op`, grouped into
/// connected face sets, plus the bounding (imprinted) edges.
#[derive(Debug, Clone, Default)]
pub struct Facesets {
    pub survive_target: Vec<Vec<crate::face::Face>>,
    pub survive_tool: Vec<Vec<crate::face::Face>>,
    pub reject_target: Vec<Vec<crate::face::Face>>,
    pub reject_tool: Vec<Vec<crate::face::Face>>,
    pub bound_target_edges: Vec<crate::edge::Edge>,
    pub bound_tool_edges: Vec<crate::edge::Edge>,
}

/// `PK_BODY_identify_facesets` (options v3): classify the faces of an
/// imprinted target/tool pair for Boolean `op`. `imprint` is the paired edge
/// and vertex result of [`Body::imprint_body`] on the same pair (manual
/// ch.48 §48.4). This is the classification/colouring oracle.
///
/// Survive/reject naming comes from the header (documented); which internal
/// list feeds which field was not traced (parasolid-re README "Blockers").
/// Tangent and non-manifold configurations return 547 `non_manifold`.
pub fn identify_facesets(
    target: &Body,
    tool: &Body,
    imprint: &crate::body::ImprintResult,
    op: BooleanOp,
) -> PsResult<Facesets> {
    let mut te: Vec<i32> = imprint.target_edges.iter().map(|e| e.tag()).collect();
    let mut oe: Vec<i32> = imprint.tool_edges.iter().map(|e| e.tag()).collect();
    let mut tv: Vec<i32> = imprint.target_vertices.iter().map(|v| v.tag()).collect();
    let mut ov: Vec<i32> = imprint.tool_vertices.iter().map(|v| v.tag()).collect();
    if te.len() != oe.len() || tv.len() != ov.len() {
        return Err(crate::error::PsError::Session(
            "identify_facesets: imprint pairs have unequal lengths".into(),
        ));
    }
    let mut opts = PK_BODY_identify_facesets_o_t { function: op_token(op), ..Default::default() };
    let mut r: PK_identify_facesets_r_t = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        PK_BODY_identify_facesets(
            target.tag(),
            tool.tag(),
            te.len() as c_int,
            te.as_mut_ptr(),
            oe.as_mut_ptr(),
            tv.len() as c_int,
            tv.as_mut_ptr(),
            ov.as_mut_ptr(),
            &mut opts,
            &mut r,
        )
    };
    let out = Facesets {
        survive_target: copy_face_sets(r.survive_target, r.n_survive_target),
        survive_tool: copy_face_sets(r.survive_tool, r.n_survive_tool),
        reject_target: copy_face_sets(r.reject_target, r.n_reject_target),
        reject_tool: copy_face_sets(r.reject_tool, r.n_reject_tool),
        bound_target_edges: copy_tags(r.bound_target_edges.array, r.bound_target_edges.length)
            .into_iter()
            .map(crate::edge::Edge::from_tag)
            .collect(),
        bound_tool_edges: copy_tags(r.bound_tool_edges.array, r.bound_tool_edges.length)
            .into_iter()
            .map(crate::edge::Edge::from_tag)
            .collect(),
    };
    if rc == 0 {
        unsafe { PK_identify_facesets_r_f(&mut r) };
    }
    crate::error::pk_check(rc)?;
    Ok(out)
}

/// `PK_FACE_boolean_2` with options version 17 at the kernel's defaults
/// (`default_tol` 1e-5), returning the raw result token, bodies and reports.
/// `target_faces` and `tool_faces` must come from two different bodies.
/// A `failed` result token is returned as data, as in [`boolean_detailed`].
///
/// Residual risk: only the default options are exercised; the
/// `PK_boolean_match_o_t` sub-struct is unmodelled, so matched regions cannot
/// be steered.
pub fn face_boolean(
    target_faces: &[crate::face::Face],
    tool_faces: &[crate::face::Face],
    op: BooleanOp,
) -> PsResult<BooleanOutcome> {
    let tf: Vec<i32> = target_faces.iter().map(|f| f.tag()).collect();
    let of: Vec<i32> = tool_faces.iter().map(|f| f.tag()).collect();
    let opts = PK_FACE_boolean_o_t { function: op_token(op), ..Default::default() };
    let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
    let mut results: PK_boolean_r_t = unsafe { std::mem::zeroed() };
    let code = unsafe {
        PK_FACE_boolean_2(
            tf.len() as c_int,
            tf.as_ptr(),
            of.len() as c_int,
            of.as_ptr(),
            &opts,
            &mut tracking,
            &mut results,
        )
    };
    unsafe { PK_TOPOL_track_r_f(&mut tracking) };
    let bodies = copy_tags(results.bodies, results.n_bodies).into_iter().map(Body::from_tag).collect();
    let reports = copy_reports(results.faults as *const PK_boolean_report_r_t, results.n_faults);
    let result_raw = results.result;
    unsafe { PK_boolean_r_f(&mut results) };
    crate::error::pk_check(code)?;
    Ok(BooleanOutcome { result: BooleanResult::from_raw(result_raw), result_raw, bodies, reports })
}

/// `PK_section_result_t`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionResult {
    Success,
    NoClash,
    NoEffect,
    Failed,
    Unknown(i32),
}

/// Result of [`Body::section_with_sheet`] (`PK_section_2_r_t`).
#[derive(Debug, Clone)]
pub struct SectionOutcome {
    pub result: SectionResult,
    pub result_raw: i32,
    pub front_bodies: Vec<Body>,
    pub back_bodies: Vec<Body>,
    pub front_faces: Vec<crate::face::Face>,
    pub back_faces: Vec<crate::face::Face>,
    pub reports: Vec<BooleanReport>,
}

impl Body {
    /// `PK_BODY_section_with_sheet_2` (options v5 at the kernel's defaults):
    /// cut this body with a sheet body into front and back pieces.
    ///
    /// Observed (parasolid-re): a closed box sheet fails with 1058
    /// `boolean_failure`; open and closed cylinder sheets work.
    pub fn section_with_sheet(&self, sheet: &Body) -> PsResult<SectionOutcome> {
        let mut opts = PK_BODY_section_o_t::kernel_defaults();
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut r: PK_section_2_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            PK_BODY_section_with_sheet_2(self.tag(), sheet.tag(), &mut opts, &mut tracking, &mut r)
        };
        unsafe { PK_TOPOL_track_r_f(&mut tracking) };
        let face = |p, n| copy_tags(p, n).into_iter().map(crate::face::Face::from_tag).collect();
        let out = SectionOutcome {
            result: match r.result {
                PK_section_result_success_c => SectionResult::Success,
                PK_section_result_no_clash_c => SectionResult::NoClash,
                PK_section_result_no_effect_c => SectionResult::NoEffect,
                PK_section_result_failed_c => SectionResult::Failed,
                other => SectionResult::Unknown(other),
            },
            result_raw: r.result,
            front_bodies: copy_tags(r.front_bodies, r.n_front_bodies).into_iter().map(Body::from_tag).collect(),
            back_bodies: copy_tags(r.back_bodies, r.n_back_bodies).into_iter().map(Body::from_tag).collect(),
            front_faces: face(r.front_faces, r.n_front_faces),
            back_faces: face(r.back_faces, r.n_back_faces),
            reports: copy_reports(r.reports, r.n_reports),
        };
        if rc == 0 {
            unsafe { PK_section_2_r_f(&mut r) };
        }
        crate::error::pk_check(rc)?;
        Ok(out)
    }
}

impl crate::face::Face {
    /// `PK_FACE_make_sheet_bodies` (options v4, kernel defaults: copy the
    /// faces, `allow_disjoint` off): sheet bodies made from copies of `faces`.
    pub fn make_sheet_bodies(faces: &[crate::face::Face]) -> PsResult<Vec<Body>> {
        let mut f: Vec<i32> = faces.iter().map(|x| x.tag()).collect();
        let mut opts = PK_FACE_make_sheet_bodies_o_t::default();
        let mut n: c_int = 0;
        let mut p: *mut PK_BODY_t = std::ptr::null_mut();
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            PK_FACE_make_sheet_bodies(f.len() as c_int, f.as_mut_ptr(), &mut opts, &mut n, &mut p, &mut tracking)
        };
        unsafe { PK_TOPOL_track_r_f(&mut tracking) };
        let a = unsafe { crate::memory::PkArray::from_raw(p, n) };
        crate::error::pk_check(rc)?;
        Ok(a.iter().map(|&t| Body::from_tag(t)).collect())
    }
}
