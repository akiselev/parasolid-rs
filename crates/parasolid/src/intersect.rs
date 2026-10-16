//! Intersection oracle — the full `PK_*_intersect_*` surface.
//!
//! Surface/surface intersection lives on [`Surf::intersect`](crate::Surf) (see
//! `surf.rs`); this module wraps the remaining five: curve/curve, surface/curve,
//! face/curve, face/face, face/surface. Together with `Surf::intersect` and the
//! boolean `Body::intersect`, every `PK_*_intersect_*` export in `distance.rs`
//! is reachable from safe code (see `docs/` for the coverage table).
//!
//! The kernel returns two flavours of result:
//! - **point intersections** (curve/curve, surface/curve, face/curve): isolated
//!   hits with their parameters — modelled as `*Hit` structs here;
//! - **point + curve intersections** (face/face, face/surface, surface/surface):
//!   modelled as [`SurfIntersection`](crate::SurfIntersection).

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::curve::Curve;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::face::Face;
use crate::geom::Vec3;
use crate::memory::PkArray;
use crate::range::Aabb;
use crate::surf::{IntersectionCurve, Surf, SurfIntersectOptions, SurfIntersection};

/// A point where two curves cross.
#[derive(Debug, Clone, Copy)]
pub struct CurveCurveHit {
    /// Position of the intersection.
    pub position: Vec3,
    /// Parameter on the first curve.
    pub t1: f64,
    /// Parameter on the second curve.
    pub t2: f64,
    /// Raw `PK_intersect_vector_t` kind.
    pub kind: i32,
}

/// Options for [`Curve::intersect_curve_checked`].
#[derive(Debug, Clone, Copy, Default)]
pub struct CurveCurveCheck<'a> {
    /// Common surface passed as `PK_CURVE_intersect_curve_o_t.common_surf`
    /// (`None` = `PK_ENTITY_null`, as in [`Curve::intersect_curve`]).
    pub common_surf: Option<&'a Surf>,
    /// Distance at or below which two curves count as meeting when an empty
    /// answer is checked. `None` = the session linear precision.
    pub tolerance: Option<f64>,
    /// Sine of the angle between the two tangents at or below which a hit is
    /// reported as tangential. `None` = 1e-6. This is a heuristic on the hit
    /// the kernel returned, not a kernel tolerance.
    pub angle_tolerance: Option<f64>,
}

/// How an empty `PK_CURVE_intersect_curve` answer was checked.
///
/// The kernel returns no vectors **and no error** both for curves that do not
/// meet and for curve pairs its solver gave up on (ICC status 3 or 4; parasolid-re
/// P1 B12, `FUN_18095ea90`). An empty answer is therefore not evidence of a miss
/// on its own; this records an independent `PK_GEOM_range` over the same intervals.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EmptyCheck {
    /// The curves stay further apart than the tolerance: a genuine miss.
    Separated { min_distance: f64 },
    /// The curves come within the tolerance, yet the kernel returned nothing.
    /// Treat as an oracle failure (silent give-up or a tolerance disagreement),
    /// never as agreement with an empty answer.
    Suspect {
        min_distance: f64,
        position: Vec3,
        t1: f64,
        t2: f64,
    },
    /// `PK_GEOM_range` did not find a distance (raw `PK_range_result_t`), or
    /// failed with this PK error code (negated); nothing can be concluded.
    Unchecked { range_status: i32 },
}

/// A curve/curve hit with an independent tangency measure.
#[derive(Debug, Clone, Copy)]
pub struct CheckedHit {
    /// The hit as the kernel reported it.
    pub hit: CurveCurveHit,
    /// |T1 × T2| / (|T1| |T2|) from `PK_CURVE_eval` at `t1` and `t2`; NaN when a
    /// tangent is zero or the evaluation failed.
    pub sin_angle: f64,
    /// `sin_angle <= angle_tolerance`, for simple (non start/end) hits. The
    /// kernel never reports `PK_intersect_vector_tangent_c` from this function:
    /// a tangency arrives as `simple_c` (parasolid-re P1 B12).
    pub tangential: bool,
}

/// Result of [`Curve::intersect_curve_checked`].
#[derive(Debug, Clone)]
pub struct CheckedCurveCurve {
    pub hits: Vec<CheckedHit>,
    /// `Some` exactly when the kernel returned no hits.
    pub empty: Option<EmptyCheck>,
}

/// How an empty surface/curve or surface/surface answer was checked.
///
/// `PK_SURF_intersect_curve` and `PK_SURF_intersect_surf` return nothing **and no
/// error** in three situations: the inputs do not meet; they meet only outside
/// the region the kernel searches; or the kernel did not report a contact it
/// should have (coincident surfaces, for one). An empty answer is therefore
/// not evidence of a miss on its own. This records an independent
/// `PK_GEOM_range` over the same inputs (with the caller's parameter bounds)
/// and places its closest-approach witness against the search region. When
/// that distance is above the tolerance, the range is repeated with every
/// surface bounded by its own `PK_SURF_ask_uvbox`, because without a bound the
/// kernel sometimes cuts an unbounded surface to the size box.
///
/// The search region is [`SurfIntersectOptions::search_region`]: the option
/// box when one is given, else the session size box [-h, h]^3 with
/// h = 0.5 * L / A, each side widened by the check's `region_margin`.
/// parasolid-re `artifacts/rs/20261016T203058Z-empty-check` has the evidence.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RangeCheck {
    /// The two surfaces are coincident (`PK_GEOM_is_coincident`), so the kernel
    /// returns nothing by design (P1 C1). Surface/surface only; tried before
    /// the range. This is the kernel's own coincidence test, not an oracle
    /// failure: compare it with CADabra's `Coincident` lane result.
    ///
    /// `PK_GEOM_is_coincident` reaches the same `COI_surface_surface` as the
    /// intersector's C1 (static-observed, `coi_surface_surface` @0x1803237d0).
    /// On planes it follows the C1 band sin(angle) * L / A + offset <= L at the
    /// session precisions: measured coincident at 0.9 L and not at 1.1 L in
    /// offset, tilt and their sum, anti-parallel included (parasolid-re P1
    /// RQ-1 fixtures `p1-rq1-*`).
    ///
    /// The kernel collapses the sense (COI class 0 or 1) into one logical, so
    /// `same_sense` is computed here: the normal of the first surface at the
    /// middle of its uv box against the second surface's normal at the same
    /// point (`Some(true)` when they agree). `None` when that evaluation
    /// failed.
    Coincident { same_sense: Option<bool> },
    /// The inputs stay further apart than the tolerance over their whole
    /// parameter boxes (or the caller's uv boxes and curve interval): a
    /// genuine miss.
    Separated { min_distance: f64 },
    /// The inputs come within the tolerance at a point inside the search
    /// region, yet the kernel returned nothing. Treat as an oracle failure (a
    /// silent give-up, coincident surfaces, or a tolerance disagreement), never
    /// as agreement with an empty answer.
    ///
    /// `position` is `PK_GEOM_range`'s witness on the first entity (the
    /// receiver surface). `params_1` are its `(u, v)`; `params_2` are the
    /// parameters of the matching point on the second entity: `(u, v)` on a
    /// surface, `[t, 0.0]` on a curve.
    Suspect {
        min_distance: f64,
        position: Vec3,
        params_1: [f64; 2],
        params_2: [f64; 2],
    },
    /// The inputs come within the tolerance, but the witness lies outside the
    /// search region, where the kernel does not report contacts: consistent
    /// with the kernel's region drop (parasolid-re P1 C467 and C2 on surf/surf).
    ///
    /// `PK_GEOM_range` returns one closest pair. When the inputs touch along a
    /// curve or a patch that also crosses the region, the witness can still
    /// fall outside it, so this does not prove that nothing was missed inside
    /// the region. An oracle should accept it only when its own answer also
    /// has no contact inside the region. On surf/curve the kernel keeps hits a
    /// little further out than L (measured: up to 1e-7 beyond the size box at
    /// L = 1e-8, none at 1.5e-7); raise `region_margin` to cover that band.
    OutsideRegion { min_distance: f64, position: Vec3 },
    /// `PK_GEOM_range` did not find a distance (raw `PK_range_result_t`), or
    /// it or `PK_SURF_ask_uvbox` failed with this PK error code (negated);
    /// nothing can be concluded.
    Unchecked { range_status: i32 },
}

/// Options for [`Surf::intersect_curve_checked`].
#[derive(Debug, Clone, Copy, Default)]
pub struct SurfCurveCheck {
    /// Distance at or below which the surface and curve count as meeting when
    /// an empty answer is checked. `None` = the session linear precision.
    pub tolerance: Option<f64>,
    /// How far beyond the search region a witness may lie and still count as
    /// inside it. `None` = the session linear precision.
    pub region_margin: Option<f64>,
}

/// Result of [`Surf::intersect_curve_checked`].
#[derive(Debug, Clone)]
pub struct CheckedSurfCurve {
    pub hits: Vec<SurfCurveHit>,
    /// `Some` exactly when the kernel returned no hits.
    pub empty: Option<RangeCheck>,
}

/// Options for [`Surf::intersect_with_checked`].
#[derive(Debug, Clone, Copy, Default)]
pub struct SurfSurfCheck {
    /// Distance at or below which the two surfaces count as meeting when an
    /// empty answer is checked. `None` = the session linear precision.
    pub tolerance: Option<f64>,
    /// How far beyond the search region a witness may lie and still count as
    /// inside it. `None` = the session linear precision, the margin of the
    /// kernel's own box tests (C467 `BOX_intersect_boxes`, C2 `COI_surface_box`).
    pub region_margin: Option<f64>,
}

/// Result of [`Surf::intersect_with_checked`].
#[derive(Debug, Clone)]
pub struct CheckedSurfSurf {
    pub result: SurfIntersection,
    /// `Some` exactly when the kernel returned no points and no curves.
    pub empty: Option<RangeCheck>,
}

fn session_precision() -> PsResult<f64> {
    let mut p = 0.0;
    pk_call!(PK_SESSION_ask_precision(&mut p));
    Ok(p)
}

/// One `PK_GEOM_range_o_t` parameter bound, as `(class, payload)`.
type ParamBound = Option<(c_int, [f64; 4])>;

fn geom_range(
    g1: PK_ENTITY_t,
    g2: PK_ENTITY_t,
    bounds: [ParamBound; 2],
) -> Result<PK_range_2_r_t, i32> {
    let mut opts = PK_GEOM_range_o_t::default();
    for (pb, b) in opts.param_bound.iter_mut().zip(bounds) {
        if let Some((class, bound)) = b {
            pb.have_param_bound = PK_LOGICAL_true;
            pb.param_bound_class = class;
            pb.bound = bound;
        }
    }
    let mut status: PK_range_result_t = -1;
    let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
    let err = unsafe { PK_GEOM_range(g1, g2, &mut opts, &mut status, &mut r) };
    if err != 0 {
        Err(-err)
    } else if status != PK_range_result_found_c {
        Err(status)
    } else {
        Ok(r)
    }
}

/// `surf`'s own parameter box as a uvbox parameter bound (`PK_SURF_ask_uvbox`).
fn own_uvbox(surf: PK_ENTITY_t) -> Result<ParamBound, i32> {
    let mut b = PK_UVBOX_t { param: [0.0; 4] };
    let err = unsafe { PK_SURF_ask_uvbox(surf, &mut b) };
    if err != 0 {
        Err(-err)
    } else {
        Ok(Some((PK_range_param_bound_class_uvbox_c, b.param)))
    }
}

/// Minimum distance between `g1` and `g2`, classified against `tol` and the
/// search region widened by `margin`.
///
/// The first `PK_GEOM_range` uses `bounds`. With no parameter bound, the kernel
/// sometimes cuts an unbounded surface to the session size box (a plane against
/// a line: distance 1500 to a line crossing it at x = 2000) and sometimes not (a
/// plane against a sphere, a cylinder against a plane), so a distance above `tol`
/// is not yet a miss. It is repeated with `full` (every surface bounded by its
/// own uvbox), which removes the cut; the smaller distance wins.
fn check_range(
    g1: PK_ENTITY_t,
    g2: PK_ENTITY_t,
    bounds: [ParamBound; 2],
    full: impl FnOnce() -> Result<[ParamBound; 2], i32>,
    tol: f64,
    region: Aabb,
    margin: f64,
) -> RangeCheck {
    let first = geom_range(g1, g2, bounds);
    let r = match first {
        Ok(r) if r.distance <= tol => r,
        _ => {
            let second = full().and_then(|b| geom_range(g1, g2, b));
            match (first, second) {
                (_, Err(s)) => return RangeCheck::Unchecked { range_status: s },
                (Ok(a), Ok(b)) if a.distance <= b.distance => a,
                (_, Ok(b)) => b,
            }
        }
    };
    if r.distance > tol {
        return RangeCheck::Separated {
            min_distance: r.distance,
        };
    }
    let position = Vec3::from_pk(r.end_1.position);
    let (lo, hi) = (region.min, region.max);
    let inside = |p: f64, lo: f64, hi: f64| p >= lo - margin && p <= hi + margin;
    if inside(position.x, lo.x, hi.x)
        && inside(position.y, lo.y, hi.y)
        && inside(position.z, lo.z, hi.z)
    {
        RangeCheck::Suspect {
            min_distance: r.distance,
            position,
            params_1: r.end_1.parameters,
            params_2: r.end_2.parameters,
        }
    } else {
        RangeCheck::OutsideRegion {
            min_distance: r.distance,
            position,
        }
    }
}

impl SurfIntersectOptions {
    /// The model-space region `PK_SURF_intersect_surf` searches with these
    /// options, before the uv patches and the surfaces' own boxes narrow it:
    /// [`SurfIntersectOptions::region`] when set, else the session size box
    /// [-h, h]^3 with h = 0.5 * L / A (`PK_SESSION_ask_precision` /
    /// `PK_SESSION_ask_angle_precision`).
    ///
    /// The option box **replaces** the size box; it is not intersected with it.
    /// `intersect_surf` @0x180418fa0 starts its box from the size box
    /// (`DAT_1845d31e0..3208`, written by `RES_set_precision` @0x1804fa3d0 as
    /// ±0.5 * L / A) and overwrites it with the option box when `have_box` is
    /// set (asm 0x18041919b-0x1804191e7) (static-observed). Measured: an option
    /// box ±1000 returns a plane/plane line bounded ±1000 and keeps a circle at
    /// x = 700 that the default ±500 box drops; L = 1e-7 or A = 1e-10 moves the
    /// size box to ±5000 or ±50 (dynamic-observed; parasolid-re
    /// `artifacts/rs/20261016T203058Z-empty-check`). `PK_SURF_intersect_curve`
    /// drops hits beyond the same size box.
    pub fn search_region(&self) -> PsResult<Aabb> {
        if let Some(b) = self.region {
            return Ok(b);
        }
        let l = session_precision()?;
        let mut a = 0.0;
        pk_call!(PK_SESSION_ask_angle_precision(&mut a));
        // RES_set_precision's order: (L / A) * 0.5 (asm 0x1804fa4a5, 0x1804fa4bc).
        let h = l / a * 0.5;
        Ok(Aabb {
            min: Vec3::new(-h, -h, -h),
            max: Vec3::new(h, h, h),
        })
    }
}

/// A point where a curve crosses a surface.
#[derive(Debug, Clone, Copy)]
pub struct SurfCurveHit {
    /// Position of the intersection.
    pub position: Vec3,
    /// `(u, v)` parameters on the surface.
    pub uv: (f64, f64),
    /// Parameter on the curve.
    pub t: f64,
    /// Raw `PK_intersect_vector_t` kind.
    pub kind: i32,
}

/// A point where a curve crosses a face.
#[derive(Debug, Clone, Copy)]
pub struct FaceCurveHit {
    /// Position of the intersection.
    pub position: Vec3,
    /// `(u, v)` parameters on the face's surface.
    pub uv: (f64, f64),
    /// Parameter on the curve.
    pub t: f64,
    /// The face topology (face/edge/vertex) hit at this point.
    pub topology: Entity,
    /// Raw `PK_intersect_fc_t` kind.
    pub kind: i32,
}

// Helper: build the point+curve result shared by face/face, face/surf, surf/surf.
fn collect_point_curves(
    n_vectors: c_int,
    vectors: *mut PK_VECTOR_t,
    n_curves: c_int,
    curves: *mut PK_CURVE_t,
    bounds: *mut PK_INTERVAL_t,
    types: *mut PK_intersect_curve_t,
) -> SurfIntersection {
    let vec_arr = unsafe { PkArray::from_raw(vectors, n_vectors) };
    let points = vec_arr.iter().map(|&v| Vec3::from_pk(v)).collect();
    let curve_arr = unsafe { PkArray::from_raw(curves, n_curves) };
    let bound_arr = unsafe { PkArray::from_raw(bounds, n_curves) };
    let type_arr = unsafe { PkArray::from_raw(types, n_curves) };
    let curves = (0..n_curves as usize)
        .map(|i| IntersectionCurve {
            curve: Curve::from_tag(curve_arr[i]),
            bounds: (bound_arr[i].low, bound_arr[i].high),
            kind: type_arr[i],
        })
        .collect();
    SurfIntersection { points, curves }
}

impl Curve {
    /// Intersect a bounded region of this curve with a bounded region of
    /// another (`PK_CURVE_intersect_curve`). Intervals are `(low, high)`
    /// parameter ranges.
    ///
    /// An empty answer can also mean the kernel gave up (no error is raised);
    /// oracle code should use [`Curve::intersect_curve_checked`].
    pub fn intersect_curve(
        &self,
        interval: (f64, f64),
        other: &Curve,
        other_interval: (f64, f64),
    ) -> PsResult<Vec<CurveCurveHit>> {
        self.intersect_curve_raw(interval, other, other_interval, PK_ENTITY_null)
    }

    /// [`Curve::intersect_curve`] with the two checks an oracle needs:
    /// - an empty answer is re-examined with `PK_GEOM_range` over the same
    ///   intervals ([`EmptyCheck`]), because the kernel returns nothing and no
    ///   error when its solver gives up;
    /// - every simple hit gets the angle between the two tangents, because the
    ///   kernel reports tangencies as `simple_c`.
    pub fn intersect_curve_checked(
        &self,
        interval: (f64, f64),
        other: &Curve,
        other_interval: (f64, f64),
        check: &CurveCurveCheck,
    ) -> PsResult<CheckedCurveCurve> {
        let surf = check.common_surf.map_or(PK_ENTITY_null, |s| s.tag());
        let raw = self.intersect_curve_raw(interval, other, other_interval, surf)?;
        let ang = check.angle_tolerance.unwrap_or(1e-6);
        let hits: Vec<CheckedHit> = raw
            .into_iter()
            .map(|hit| {
                let sin_angle = match (self.eval_with_tangent(hit.t1), other.eval_with_tangent(hit.t2)) {
                    (Ok((_, a)), Ok((_, b))) => sin_between(a, b),
                    _ => f64::NAN,
                };
                let simple = hit.kind != PK_intersect_vector_start_c && hit.kind != PK_intersect_vector_end_c;
                CheckedHit { hit, sin_angle, tangential: simple && sin_angle <= ang }
            })
            .collect();
        let empty = if hits.is_empty() {
            let tol = match check.tolerance {
                Some(t) => t,
                None => {
                    let mut p = 0.0;
                    pk_call!(PK_SESSION_ask_precision(&mut p));
                    p
                }
            };
            Some(self.check_empty(interval, other, other_interval, tol))
        } else {
            None
        };
        Ok(CheckedCurveCurve { hits, empty })
    }

    /// Minimum distance between the two bounded curves (`PK_GEOM_range`, both
    /// intervals as parameter bounds), classified against `tol`.
    fn check_empty(&self, interval: (f64, f64), other: &Curve, other_interval: (f64, f64), tol: f64) -> EmptyCheck {
        let mut opts = PK_GEOM_range_o_t::default();
        for (pb, iv) in opts.param_bound.iter_mut().zip([interval, other_interval]) {
            pb.have_param_bound = PK_LOGICAL_true;
            pb.param_bound_class = PK_range_param_bound_class_interval_c;
            pb.bound = [iv.0, iv.1, 0.0, 0.0];
        }
        let mut status: PK_range_result_t = -1;
        let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
        let err = unsafe { PK_GEOM_range(self.tag, other.tag, &mut opts, &mut status, &mut r) };
        if err != 0 {
            return EmptyCheck::Unchecked { range_status: -err };
        }
        if status != PK_range_result_found_c {
            return EmptyCheck::Unchecked { range_status: status };
        }
        if r.distance <= tol {
            EmptyCheck::Suspect {
                min_distance: r.distance,
                position: Vec3::from_pk(r.end_1.position),
                t1: r.end_1.parameters[0],
                t2: r.end_2.parameters[0],
            }
        } else {
            EmptyCheck::Separated { min_distance: r.distance }
        }
    }

    fn intersect_curve_raw(
        &self,
        interval: (f64, f64),
        other: &Curve,
        other_interval: (f64, f64),
        common_surf: PK_ENTITY_t,
    ) -> PsResult<Vec<CurveCurveHit>> {
        // `o_t_version = 1` is already the ceiling: v2+ return
        // `o_t_version_unknown` (5022). `have_box` is live at v1 (a garbage
        // token gives `PK_ERROR_not_a_logical`, 908), so the whole struct is
        // already being read.
        let opts = PK_CURVE_intersect_curve_o_t {
            o_t_version: 1,
            have_box: PK_LOGICAL_false,
            r#box: PK_BOX_t { coord: [0.0; 6] },
            common_surf,
        };
        let iv1 = PK_INTERVAL_t {
            low: interval.0,
            high: interval.1,
        };
        let iv2 = PK_INTERVAL_t {
            low: other_interval.0,
            high: other_interval.1,
        };
        let mut n: c_int = 0;
        let (mut vectors, mut ts1, mut ts2, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_CURVE_intersect_curve(
            self.tag,
            iv1,
            other.tag,
            iv2,
            &opts,
            &mut n,
            &mut vectors,
            &mut ts1,
            &mut ts2,
            &mut types,
        ));
        let v = unsafe { PkArray::from_raw(vectors, n) };
        let t1 = unsafe { PkArray::from_raw(ts1, n) };
        let t2 = unsafe { PkArray::from_raw(ts2, n) };
        let k = unsafe { PkArray::from_raw(types, n) };
        Ok((0..n as usize)
            .map(|i| CurveCurveHit {
                position: Vec3::from_pk(v[i]),
                t1: t1[i],
                t2: t2[i],
                kind: k[i],
            })
            .collect())
    }
}

/// |a × b| / (|a| |b|); NaN when either vector is zero.
fn sin_between(a: Vec3, b: Vec3) -> f64 {
    let c = Vec3::new(a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x);
    let n = |v: Vec3| (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    let d = n(a) * n(b);
    if d == 0.0 { f64::NAN } else { n(c) / d }
}

impl Surf {
    /// [`Surf::intersect_curve`] with an empty answer re-examined by
    /// `PK_GEOM_range` (surface, curve over `interval`) ([`RangeCheck`]).
    ///
    /// The kernel returns nothing and no error for a curve that crosses the
    /// surface outside the session size box (measured: a line crossing the
    /// plane z = 0 at x = 2000), so an empty answer is not evidence of a miss.
    /// The witness is placed against the size box; the surface comes first, so
    /// `params_1` are `(u, v)` and `params_2` is `[t, 0.0]`.
    pub fn intersect_curve_checked(
        &self,
        curve: &Curve,
        interval: (f64, f64),
        check: &SurfCurveCheck,
    ) -> PsResult<CheckedSurfCurve> {
        let hits = self.intersect_curve(curve, interval)?;
        let empty = if hits.is_empty() {
            let l = session_precision()?;
            let region = SurfIntersectOptions::default().search_region()?;
            let bound = Some((
                PK_range_param_bound_class_interval_c,
                [interval.0, interval.1, 0.0, 0.0],
            ));
            Some(check_range(
                self.tag(),
                curve.tag,
                [None, bound],
                || Ok([own_uvbox(self.tag())?, bound]),
                check.tolerance.unwrap_or(l),
                region,
                check.region_margin.unwrap_or(l),
            ))
        } else {
            None
        };
        Ok(CheckedSurfCurve { hits, empty })
    }

    /// [`Surf::intersect_with`] with an empty answer re-examined by
    /// `PK_GEOM_range` ([`RangeCheck`]).
    ///
    /// The range is bounded by `uvbox_1` / `uvbox_2` when they are given
    /// (`PK_range_param_bound_class_uvbox_c`), and its witness is placed against
    /// [`SurfIntersectOptions::search_region`]. The kernel returns nothing for
    /// coincident surfaces and for contacts outside that region, and no error
    /// in either case. Coincidence is tested first ([`RangeCheck::Coincident`]);
    /// if `PK_GEOM_is_coincident` fails, the range classification still runs.
    ///
    /// The kernel uses a uv box as the model-space box of that patch, not as a
    /// parametric trim (see [`Surf::intersect_with`]); the check bounds the
    /// range by the patch itself, so a contact outside the patch but inside its
    /// model box is not seen here.
    pub fn intersect_with_checked(
        &self,
        other: &Surf,
        options: &SurfIntersectOptions,
        check: &SurfSurfCheck,
    ) -> PsResult<CheckedSurfSurf> {
        let result = self.intersect_with(other, options)?;
        let empty = if result.points.is_empty() && result.curves.is_empty() {
            if let Ok(true) = self.is_coincident(other) {
                let same_sense = self.same_sense(other).ok();
                return Ok(CheckedSurfSurf {
                    result,
                    empty: Some(RangeCheck::Coincident { same_sense }),
                });
            }
            let l = session_precision()?;
            let region = options.search_region()?;
            let uv = |b: Option<crate::surf::UvBox>| {
                b.map(|b| {
                    (
                        PK_range_param_bound_class_uvbox_c,
                        [b.u_min, b.v_min, b.u_max, b.v_max],
                    )
                })
            };
            let (b1, b2) = (uv(options.uvbox_1), uv(options.uvbox_2));
            let full = || -> Result<[ParamBound; 2], i32> {
                let f1 = if b1.is_some() {
                    b1
                } else {
                    own_uvbox(self.tag())?
                };
                let f2 = if b2.is_some() {
                    b2
                } else {
                    own_uvbox(other.tag())?
                };
                Ok([f1, f2])
            };
            Some(check_range(
                self.tag(),
                other.tag(),
                [b1, b2],
                full,
                check.tolerance.unwrap_or(l),
                region,
                check.region_margin.unwrap_or(l),
            ))
        } else {
            None
        };
        Ok(CheckedSurfSurf { result, empty })
    }

    /// Whether the two surfaces occupy the same positions in space
    /// (`PK_GEOM_is_coincident`), in either sense.
    ///
    /// The entry point takes no options and no tolerance: it decides within
    /// the session precision (static-observed, PK_GEOM_is_coincident
    /// @0x1803205b0 journals only the two tags and the result; the V35 header
    /// agrees). The vendor documentation says it may miss an equivalence but
    /// never claims one that is not there. See [`RangeCheck::Coincident`] for
    /// the measured plane band.
    pub fn is_coincident(&self, other: &Surf) -> PsResult<bool> {
        let mut c: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_GEOM_is_coincident(self.tag(), other.tag(), &mut c));
        Ok(c == PK_LOGICAL_true)
    }

    /// Whether `other`'s normal agrees with this surface's normal at the
    /// middle of this surface's uv box.
    fn same_sense(&self, other: &Surf) -> PsResult<bool> {
        let b = self.uvbox()?;
        let (p, n1) =
            self.eval_with_normal(0.5 * (b.u_min + b.u_max), 0.5 * (b.v_min + b.v_max))?;
        let (u, v) = other.parameterise(p)?;
        let (_, n2) = other.eval_with_normal(u, v)?;
        Ok(n1.x * n2.x + n1.y * n2.y + n1.z * n2.z > 0.0)
    }

    /// Intersect this surface with a bounded region of a curve
    /// (`PK_SURF_intersect_curve`).
    pub fn intersect_curve(
        &self,
        curve: &Curve,
        interval: (f64, f64),
    ) -> PsResult<Vec<SurfCurveHit>> {
        // Full documented v1 layout, zero-initialised (have_box = false).
        //
        // `o_t_version = 1` is already the ceiling: `version_upgrade_probe` on
        // V37.01.243 gets `o_t_version_unknown` (5022) for v2 and every value
        // above it, so there is no later version to adopt. Unlike its
        // surf/surf sibling this entry point never gained a v2, so it has no
        // `mixed_curve_category` to keep alive.
        let mut opts: PK_SURF_intersect_curve_o_t = unsafe { std::mem::zeroed() };
        opts.o_t_version = 1;
        let iv = PK_INTERVAL_t {
            low: interval.0,
            high: interval.1,
        };
        let mut n: c_int = 0;
        let (mut vectors, mut uvs, mut ts, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_SURF_intersect_curve(
            self.tag,
            curve.tag,
            iv,
            &opts,
            &mut n,
            &mut vectors,
            &mut uvs,
            &mut ts,
            &mut types,
        ));
        let v = unsafe { PkArray::from_raw(vectors, n) };
        let uv = unsafe { PkArray::from_raw(uvs, n) };
        let t = unsafe { PkArray::from_raw(ts, n) };
        let k = unsafe { PkArray::from_raw(types, n) };
        Ok((0..n as usize)
            .map(|i| SurfCurveHit {
                position: Vec3::from_pk(v[i]),
                uv: (uv[i][0], uv[i][1]),
                t: t[i],
                kind: k[i],
            })
            .collect())
    }
}

impl Face {
    /// Intersect this face with a bounded region of a curve
    /// (`PK_FACE_intersect_curve`, no options).
    pub fn intersect_curve(
        &self,
        curve: &Curve,
        interval: (f64, f64),
    ) -> PsResult<Vec<FaceCurveHit>> {
        let iv = PK_INTERVAL_t {
            low: interval.0,
            high: interval.1,
        };
        let mut n: c_int = 0;
        let (mut vectors, mut uvs, mut ts, mut topols, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_FACE_intersect_curve(
            self.tag,
            curve.tag,
            iv,
            &mut n,
            &mut vectors,
            &mut uvs,
            &mut ts,
            &mut topols,
            &mut types,
        ));
        let v = unsafe { PkArray::from_raw(vectors, n) };
        let uv = unsafe { PkArray::from_raw(uvs, n) };
        let t = unsafe { PkArray::from_raw(ts, n) };
        let tp = unsafe { PkArray::from_raw(topols, n) };
        let k = unsafe { PkArray::from_raw(types, n) };
        Ok((0..n as usize)
            .map(|i| FaceCurveHit {
                position: Vec3::from_pk(v[i]),
                uv: (uv[i][0], uv[i][1]),
                t: t[i],
                topology: Entity::from_tag(tp[i]),
                kind: k[i],
            })
            .collect())
    }

    /// Intersect this face with another face (`PK_FACE_intersect_face`).
    pub fn intersect_face(&self, other: &Face) -> PsResult<SurfIntersection> {
        let opts = face_intersect_face_opts();
        let mut n_vectors: c_int = 0;
        let mut n_curves: c_int = 0;
        let (mut vectors, mut curves, mut bounds, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_FACE_intersect_face(
            self.tag,
            other.tag,
            &opts,
            &mut n_vectors,
            &mut vectors,
            &mut n_curves,
            &mut curves,
            &mut bounds,
            &mut types,
        ));
        Ok(collect_point_curves(
            n_vectors, vectors, n_curves, curves, bounds, types,
        ))
    }

    /// Intersect this face with a surface (`PK_FACE_intersect_surf`).
    pub fn intersect_surf(&self, surf: &Surf) -> PsResult<SurfIntersection> {
        let opts = face_intersect_surf_opts();
        let mut n_vectors: c_int = 0;
        let mut n_curves: c_int = 0;
        let (mut vectors, mut curves, mut bounds, mut types) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        pk_call!(PK_FACE_intersect_surf(
            self.tag,
            surf.tag(),
            &opts,
            &mut n_vectors,
            &mut vectors,
            &mut n_curves,
            &mut curves,
            &mut bounds,
            &mut types,
        ));
        Ok(collect_point_curves(
            n_vectors, vectors, n_curves, curves, bounds, types,
        ))
    }
}

// The face/face and face/surf option structs carry extra uvbox fields; a
// zero-initialised, version-1 instance means "no restriction". `mem::zeroed`
// then setting o_t_version is the least error-prone way to build them without
// hard-coding every (still-unverified) trailing field.
/// `o_t_version = 1` — deliberately NOT raised. `PK_FACE_intersect_face` is
/// the odd one out of the intersection family: its measured ceiling is **1**
/// (v2 and v3 both return `PK_ERROR_not_implemented` 5000, v4+ return
/// `o_t_version_unknown` 5022), so the v2 treatment applied to
/// `PK_SURF_intersect_surf` and `PK_FACE_intersect_surf` is not available here.
///
/// Consequence / residual risk: `mixed_curve_category` is not read at v1, so
/// face/face intersections fall back to the kernel's own default mixed-curve
/// category and cannot be pinned the way SSI now is. Revisit if a later
/// Parasolid build implements v2 for this entry point.
fn face_intersect_face_opts() -> PK_FACE_intersect_face_o_t {
    let mut o = unsafe { std::mem::zeroed::<PK_FACE_intersect_face_o_t>() };
    o.o_t_version = 1;
    o
}
/// `o_t_version = 2` — the highest version `PK_FACE_intersect_surf` accepts
/// (v3 returns `PK_ERROR_not_implemented` 5000, v4+ `o_t_version_unknown`
/// 5022). Measured by `version_upgrade_probe`: a garbage `mixed_curve_category`
/// is *ignored* at v1 but returns `field_of_wrong_type` (5014) at v2, so v2 is
/// the first version that actually reads it. This is the same defect already
/// fixed for `PK_SURF_intersect_surf_o_t` — at v1 the mixed-dimension category
/// was silently dead and the kernel used its own default.
fn face_intersect_surf_opts() -> PK_FACE_intersect_surf_o_t {
    let mut o = unsafe { std::mem::zeroed::<PK_FACE_intersect_surf_o_t>() };
    o.o_t_version = 2;
    o.mixed_curve_category = PK_mixed_intersection_classic_c;
    o
}
