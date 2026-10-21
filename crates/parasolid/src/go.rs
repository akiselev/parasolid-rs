//! Graphical output (GO) capture.
//!
//! `PK_TOPOL_render_line`, `PK_TOPOL_render_facet` and `PK_GEOM_render` return
//! nothing in memory: they send their output through the GO frustrum
//! callbacks GOOPSG (open a segment), GOSGMT (one segment) and GOCLSG (close a
//! segment). Start the session with [`FrustrumConfig::capture_go`] and this
//! module records every call; read it back with [`take`].
//!
//! The arguments are kept as raw integers and doubles. The segment-type,
//! line-type and visibility codes are defined in the Parasolid Downward
//! Interfaces manual, which parasolid-re does not have (B-R44-4), so no names
//! are given for them here.
//!
//! Ordering: under SMP only the open/close brackets keep their order; the
//! segments inside one bracket may arrive in any order (Functional Description
//! ch112). Compare the segments of a bracket as a multiset; [`brackets`] groups
//! them.
//!
//! Values seen on V37.01.243 (dynamic-observed, parasolid-re
//! `artifacts/rs/20261021T031234Z-prs-render`; meanings inferred from the
//! calls that produced them, not from the manual):
//!
//! | value | where | produced by |
//! |---|---|---|
//! | segtyp 2003 | Open/Close | the body bracket of `render_line` / `render_facet` |
//! | segtyp 2006 / 2007 / 2008 | Segment | edges / silhouettes / planar hatch lines |
//! | segtyp 2017 / 2016 | Open-Close / Segment | a face bracket / its facets |
//! | segtyp 2019 | all three | `PK_GEOM_render` of a curve |
//! | `lntp[1]` 3001 / 3003 / 3006 / 3013 | Segment | line / full circle / polyline / NURBS curve |
//! | `lntp[1]` 3007 / 3008 | Segment | facet vertices / vertices then normals |
//! | `lntp[3]` 1006 / 1007 / 1008 | edge Segment | visible / hidden / not computed |
//!
//! `lntp[0]` is the occurrence number (1 for the first topol passed).
//!
//! Callback ABI evidence: `parasolid_sys::frustrum` (`PK_GOSGMT_f_t`).
//!
//! [`FrustrumConfig::capture_go`]: crate::FrustrumConfig::capture_go

use std::os::raw::{c_double, c_int};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;

/// Which GO callback produced a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoCall {
    /// GOOPSG: open a hierarchical segment.
    Open,
    /// GOSGMT: a single-level segment.
    Segment,
    /// GOCLSG: close a hierarchical segment.
    Close,
}

/// One GO callback, with its arguments copied out.
#[derive(Debug, Clone, PartialEq)]
pub struct GoRecord {
    pub call: GoCall,
    /// `segtyp`, raw.
    pub segtyp: i32,
    /// The `ntags` entity tags.
    pub tags: Vec<i32>,
    /// The raw `ngeom` argument. For line and curve segments it counts
    /// doubles; for facet segments it counts 3-vectors (see [`geom`]).
    ///
    /// [`geom`]: GoRecord::geom
    pub ngeom: i32,
    /// The segment geometry. It holds `ngeom` doubles, except for the facet
    /// line types in [`VECTOR_GEOM_LINE_TYPES`] (by `lntp[1]`), where it holds
    /// `3 * ngeom` doubles (`ngeom` 3-vectors). Facet types not in that list
    /// are copied as `ngeom` doubles only, a prefix of their data.
    pub geom: Vec<f64>,
    /// The `nlntp` line-type / occurrence integers, raw.
    pub lntp: Vec<i32>,
}

/// One open/close bracket and the single-level segments between them.
#[derive(Debug, Clone, PartialEq)]
pub struct GoBracket {
    /// The GOOPSG record, or `None` for segments sent outside any bracket.
    pub open: Option<GoRecord>,
    pub segments: Vec<GoRecord>,
    /// The matching GOCLSG record, if one arrived.
    pub close: Option<GoRecord>,
    /// Brackets opened inside this one, in arrival order.
    pub children: Vec<GoBracket>,
}

/// Facet line types (`lntp[1]`) whose `ngeom` counts 3-vectors rather than
/// doubles. REN_go_facet @0x182377660 sets `ngeom` to the vertex count times
/// 1, 2, 3, 5, 6 or 8 by output mode (static-observed); only the modes listed
/// here were checked live (parasolid-re artifacts/rs/20261021T031234Z-prs-render):
/// 3007 = vertices only, 3008 = vertices with normals (`go_normals` yes).
pub const VECTOR_GEOM_LINE_TYPES: &[i32] = &[3007, 3008];

static RECORDS: Mutex<Vec<GoRecord>> = Mutex::new(Vec::new());

/// Remove and return every record captured so far, in arrival order.
pub fn take() -> Vec<GoRecord> {
    match RECORDS.lock() {
        Ok(mut g) => std::mem::take(&mut *g),
        Err(p) => std::mem::take(&mut *p.into_inner()),
    }
}

/// Discard every record captured so far.
pub fn clear() {
    let _ = take();
}

/// Group records into open/close brackets (nested as the kernel sent them).
/// Segments sent outside any bracket are collected in a top-level bracket
/// with `open: None`, which is returned first when present.
pub fn brackets(records: &[GoRecord]) -> Vec<GoBracket> {
    let empty = || GoBracket {
        open: None,
        segments: Vec::new(),
        close: None,
        children: Vec::new(),
    };
    let mut loose = empty();
    let mut done: Vec<GoBracket> = Vec::new();
    let mut stack: Vec<GoBracket> = Vec::new();
    for r in records {
        match r.call {
            GoCall::Open => {
                let mut b = empty();
                b.open = Some(r.clone());
                stack.push(b);
            }
            GoCall::Segment => match stack.last_mut() {
                Some(b) => b.segments.push(r.clone()),
                None => loose.segments.push(r.clone()),
            },
            GoCall::Close => {
                if let Some(mut b) = stack.pop() {
                    b.close = Some(r.clone());
                    match stack.last_mut() {
                        Some(parent) => parent.children.push(b),
                        None => done.push(b),
                    }
                } else {
                    // A close with no open: keep it visible rather than drop it.
                    let mut b = empty();
                    b.close = Some(r.clone());
                    done.push(b);
                }
            }
        }
    }
    // Unclosed brackets are returned as they are.
    while let Some(b) = stack.pop() {
        match stack.last_mut() {
            Some(parent) => parent.children.push(b),
            None => done.push(b),
        }
    }
    if !loose.segments.is_empty() {
        done.insert(0, loose);
    }
    done
}

/// Copy `n` elements from `p`; a null pointer or a non-positive count gives
/// an empty vector.
unsafe fn copy_n<T: Copy>(n: *const c_int, p: *const T) -> Vec<T> {
    if n.is_null() || p.is_null() {
        return Vec::new();
    }
    let n = unsafe { *n };
    if n <= 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(p, n as usize) }.to_vec()
}

#[allow(clippy::too_many_arguments)]
unsafe fn record(
    call: GoCall,
    segtyp: *const c_int,
    ntags: *const c_int,
    tags: *const c_int,
    ngeom: *const c_int,
    geom: *const c_double,
    nlntp: *const c_int,
    lntp: *const c_int,
    ifail: *mut c_int,
) {
    let ok = catch_unwind(AssertUnwindSafe(|| {
        let lntp = unsafe { copy_n(nlntp, lntp) };
        let n = if ngeom.is_null() {
            0
        } else {
            unsafe { *ngeom }
        };
        let vectors = lntp.len() >= 2 && VECTOR_GEOM_LINE_TYPES.contains(&lntp[1]);
        let n_doubles = if vectors { n.saturating_mul(3) } else { n };
        let r = GoRecord {
            call,
            segtyp: if segtyp.is_null() {
                0
            } else {
                unsafe { *segtyp }
            },
            tags: unsafe { copy_n(ntags, tags) },
            ngeom: n,
            geom: unsafe { copy_n(&n_doubles, geom) },
            lntp,
        };
        match RECORDS.lock() {
            Ok(mut g) => g.push(r),
            Err(p) => p.into_inner().push(r),
        }
    }))
    .is_ok();
    if !ifail.is_null() {
        // The kernel presets *ifail to -1011; 0 means "continue".
        unsafe { *ifail = if ok { 0 } else { -1 } };
    }
}

macro_rules! go_callback {
    ($name:ident, $call:expr) => {
        #[allow(clippy::too_many_arguments)]
        pub(crate) unsafe extern "C" fn $name(
            segtyp: *const c_int,
            ntags: *const c_int,
            tags: *const c_int,
            ngeom: *const c_int,
            geom: *const c_double,
            nlntp: *const c_int,
            lntp: *const c_int,
            ifail: *mut c_int,
        ) {
            unsafe { record($call, segtyp, ntags, tags, ngeom, geom, nlntp, lntp, ifail) }
        }
    };
}

go_callback!(capture_goopsg, GoCall::Open);
go_callback!(capture_gosgmt, GoCall::Segment);
go_callback!(capture_goclsg, GoCall::Close);
