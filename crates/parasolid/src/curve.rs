//! Curve type — geometry attached to edges.

use parasolid_sys::*;
use crate::error::PsResult;
use crate::entity::Entity;
use crate::body::Body;
use crate::geom::{Vec3, Axis2};

/// Periodicity of a parametric direction (`PK_PARAM_periodic_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Periodicity {
    NonPeriodic,
    Periodic,
    Seamed,
    Other(i32),
}

impl Periodicity {
    pub(crate) fn from_token(t: i32) -> Self {
        match t {
            PK_PARAM_periodic_no_c => Periodicity::NonPeriodic,
            PK_PARAM_periodic_yes_c => Periodicity::Periodic,
            PK_PARAM_periodic_seamed_c => Periodicity::Seamed,
            other => Periodicity::Other(other),
        }
    }
    /// True for `Periodic` or `Seamed`.
    pub fn is_periodic(&self) -> bool {
        matches!(self, Periodicity::Periodic | Periodicity::Seamed)
    }
}

/// Curve parameterisation summary (from [`Curve::param`]).
#[derive(Debug, Clone, Copy)]
pub struct CurveParam {
    pub range: (f64, f64),
    pub periodic: Periodicity,
    pub closed: bool,
}

/// B-curve (NURBS curve) standard form (from [`Curve::ask_bcurve`]).
#[derive(Debug, Clone)]
pub struct BCurveData {
    pub degree: i32,
    pub n_vertices: i32,
    /// Control points (x,y,z); for a rational curve the weight is dropped here.
    pub control_points: Vec<Vec3>,
    pub knots: Vec<f64>,
    pub is_rational: bool,
}

/// Curvature frame of a curve at a parameter (from [`Curve::eval_curvature`]).
#[derive(Debug, Clone, Copy)]
pub struct CurveCurvature {
    pub tangent: Vec3,
    pub principal_normal: Vec3,
    pub binormal: Vec3,
    /// Curvature magnitude κ = 1/radius (≥ 0).
    pub curvature: f64,
}

/// Concrete curve type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurveType {
    Line, Circle, Ellipse, Bcurve, Icurve, Fcurve, Scurve, Tcurve, Cpcurve, Pline,
}

/// A curve entity handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Curve {
    pub(crate) tag: PK_CURVE_t,
}

/// Line parameters: origin point and direction unit vector.
#[derive(Debug, Clone, Copy)]
pub struct LineData {
    pub origin: Vec3,
    pub direction: Vec3,
}

/// Circle parameters: radius and placement frame.
#[derive(Debug, Clone, Copy)]
pub struct CircleData {
    pub radius: f64,
    pub basis: Axis2,
}

/// Ellipse parameters: semi-axes R1 (major) and R2 (minor), and placement frame.
#[derive(Debug, Clone, Copy)]
pub struct EllipseData {
    pub r1: f64,
    pub r2: f64,
    pub basis: Axis2,
}

impl Curve {
    pub(crate) fn from_tag(tag: PK_CURVE_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    /// Determine the concrete curve type.
    pub fn curve_type(&self) -> PsResult<CurveType> {
        let class = self.entity().class()?;
        use crate::entity::PkClass;
        Ok(match class {
            PkClass::Line => CurveType::Line,
            PkClass::Circle => CurveType::Circle,
            PkClass::Ellipse => CurveType::Ellipse,
            PkClass::Bcurve => CurveType::Bcurve,
            PkClass::Icurve => CurveType::Icurve,
            PkClass::Fcurve => CurveType::Fcurve,
            PkClass::Scurve => CurveType::Scurve,
            PkClass::Tcurve => CurveType::Tcurve,
            PkClass::Cpcurve => CurveType::Cpcurve,
            PkClass::Pline => CurveType::Pline,
            _ => return Err(crate::error::PsError::Session(
                format!("entity {} is not a curve (class {:?})", self.tag, class)
            )),
        })
    }

    /// Extract line parameters.
    ///
    /// `PK_LINE_sf_t` holds a `PK_AXIS1_sf_t` basis set with `location` and `axis`.
    pub fn ask_line(&self) -> PsResult<LineData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_LINE_sf_t>() };
        pk_call!(PK_LINE_ask(self.tag, &mut sf));
        Ok(LineData {
            origin: Vec3::from_pk(sf.basis_set.location),
            direction: Vec3::from_pk(sf.basis_set.axis),
        })
    }

    /// Extract circle parameters.
    pub fn ask_circle(&self) -> PsResult<CircleData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_CIRCLE_sf_t>() };
        pk_call!(PK_CIRCLE_ask(self.tag, &mut sf));
        Ok(CircleData { radius: sf.radius, basis: Axis2::from_pk(sf.basis_set) })
    }

    /// Extract ellipse parameters.
    ///
    /// `PK_ELLIPSE_sf_t` uses `R1` (major semi-axis) and `R2` (minor semi-axis).
    pub fn ask_ellipse(&self) -> PsResult<EllipseData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_ELLIPSE_sf_t>() };
        pk_call!(PK_ELLIPSE_ask(self.tag, &mut sf));
        Ok(EllipseData { r1: sf.R1, r2: sf.R2, basis: Axis2::from_pk(sf.basis_set) })
    }

    /// Evaluate curve position at parameter t.
    ///
    /// Calls `PK_CURVE_eval` with zero derivatives; result is the 3D point R(t).
    pub fn eval(&self, t: f64) -> PsResult<Vec3> {
        let mut p = [0.0f64; 3];
        pk_call!(PK_CURVE_eval(self.tag, t, 0, p.as_mut_ptr()));
        Ok(Vec3::from_pk(p))
    }

    /// Evaluate curve position and tangent at parameter t.
    ///
    /// Calls `PK_CURVE_eval` with n_deriv=1; result is 6 doubles:
    /// p[0..3] = R(t), p[3..6] = dR/dt (tangent, not necessarily unit length).
    pub fn eval_with_tangent(&self, t: f64) -> PsResult<(Vec3, Vec3)> {
        let mut p = [0.0f64; 6];
        pk_call!(PK_CURVE_eval(self.tag, t, 1, p.as_mut_ptr()));
        let pos = Vec3::new(p[0], p[1], p[2]);
        let tan = Vec3::new(p[3], p[4], p[5]);
        Ok((pos, tan))
    }

    /// Arc length of the curve over the parameter interval `[t0, t1]`.
    ///
    /// Wraps `PK_CURVE_find_length`, which takes the interval **by value**
    /// (`PK_INTERVAL_t` is a 16-byte `{low, high}` struct, passed indirectly on
    /// the Win64 ABI). Returns the length only; the achieved parameter range is
    /// discarded.
    pub fn length(&self, interval: (f64, f64)) -> PsResult<f64> {
        let iv = PK_INTERVAL_t { low: interval.0, high: interval.1 };
        let mut length = 0.0f64;
        let mut range = PK_INTERVAL_t { low: 0.0, high: 0.0 };
        pk_call!(PK_CURVE_find_length(self.tag, iv, &mut length, &mut range));
        Ok(length)
    }

    /// Create a non-rational **B-curve** (NURBS curve) from its `degree`, control
    /// points, and knot vector given as **distinct** `knots` with per-knot
    /// `multiplicities` (Parasolid's form — not an expanded knot vector). For a
    /// clamped open curve of degree `d`, the end knots have multiplicity `d + 1`
    /// and `Σ multiplicities == control_points.len() + d + 1`.
    pub fn bcurve(
        degree: i32,
        control_points: &[Vec3],
        knots: &[f64],
        multiplicities: &[i32],
    ) -> PsResult<Curve> {
        let verts: Vec<f64> = control_points.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
        let sf = PK_BCURVE_sf_t {
            degree,
            n_vertices: control_points.len() as std::os::raw::c_int,
            vertex_dim: 3,
            is_rational: PK_LOGICAL_false,
            vertices: verts.as_ptr(),
            _reserved_24: 0,
            n_knots: knots.len() as std::os::raw::c_int,
            knot_mult: multiplicities.as_ptr(),
            knots: knots.as_ptr(),
            knot_type: PK_knot_non_uniform_c,
            is_periodic: 0,
            is_closed: 0,
            _pad: [0; 2],
            self_intersecting: 0,
        };
        let mut tag: PK_BCURVE_t = PK_ENTITY_null;
        pk_call!(PK_BCURVE_create(&sf, &mut tag));
        Ok(Curve::from_tag(tag))
    }

    /// Read back the B-curve standard form (`PK_BCURVE_ask`; frees the
    /// kernel-allocated vertex/knot arrays).
    pub fn ask_bcurve(&self) -> PsResult<BCurveData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_BCURVE_sf_t>() };
        pk_call!(PK_BCURVE_ask(self.tag, &mut sf));
        let dim = sf.vertex_dim.max(1) as usize;
        let control_points: Vec<Vec3> = if sf.vertices.is_null() {
            Vec::new()
        } else {
            let slice = unsafe { std::slice::from_raw_parts(sf.vertices, sf.n_vertices as usize * dim) };
            (0..sf.n_vertices as usize)
                .map(|i| Vec3::new(slice[i * dim], slice[i * dim + 1], slice[i * dim + 2]))
                .collect()
        };
        let knots: Vec<f64> = if sf.knots.is_null() {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(sf.knots, sf.n_knots as usize) }.to_vec()
        };
        unsafe {
            if !sf.vertices.is_null() { let _ = PK_MEMORY_free(sf.vertices as *mut std::os::raw::c_void); }
            if !sf.knots.is_null() { let _ = PK_MEMORY_free(sf.knots as *mut std::os::raw::c_void); }
            if !sf.knot_mult.is_null() { let _ = PK_MEMORY_free(sf.knot_mult as *mut std::os::raw::c_void); }
        }
        Ok(BCurveData {
            degree: sf.degree,
            n_vertices: sf.n_vertices,
            control_points,
            knots,
            is_rational: sf.is_rational == PK_LOGICAL_true,
        })
    }

    /// Evaluate the curvature frame (tangent / principal normal / binormal / κ)
    /// at parameter `t` via `PK_CURVE_eval_curvature`. Errors on a straight
    /// (zero-curvature) portion where the principal normal is undefined.
    pub fn eval_curvature(&self, t: f64) -> PsResult<CurveCurvature> {
        let mut tangent: PK_VECTOR1_t = [0.0; 3];
        let mut principal_normal: PK_VECTOR1_t = [0.0; 3];
        let mut binormal: PK_VECTOR1_t = [0.0; 3];
        let mut curvature = 0.0f64;
        pk_call!(PK_CURVE_eval_curvature(
            self.tag,
            t,
            &mut tangent,
            &mut principal_normal,
            &mut binormal,
            &mut curvature,
        ));
        Ok(CurveCurvature {
            tangent: Vec3::from_pk(tangent),
            principal_normal: Vec3::from_pk(principal_normal),
            binormal: Vec3::from_pk(binormal),
            curvature,
        })
    }

    /// Parametric interval `(low, high)` of the curve (`PK_CURVE_ask_interval`).
    pub fn interval(&self) -> PsResult<(f64, f64)> {
        let mut iv = PK_INTERVAL_t { low: 0.0, high: 0.0 };
        pk_call!(PK_CURVE_ask_interval(self.tag, &mut iv));
        Ok((iv.low, iv.high))
    }

    /// Parameterisation summary: range, periodicity, closure (`PK_CURVE_ask_param`).
    pub fn param(&self) -> PsResult<CurveParam> {
        let mut sf = unsafe { std::mem::zeroed::<PK_PARAM_sf_t>() };
        pk_call!(PK_CURVE_ask_param(self.tag, &mut sf));
        Ok(CurveParam {
            range: (sf.range.low, sf.range.high),
            periodic: Periodicity::from_token(sf.periodic),
            closed: (sf.closed & 0xff) != 0,
        })
    }

    /// Whether the curve is closed.
    pub fn is_closed(&self) -> PsResult<bool> {
        Ok(self.param()?.closed)
    }

    /// Whether the curve is periodic (periodic or seamed).
    pub fn is_periodic(&self) -> PsResult<bool> {
        Ok(self.param()?.periodic.is_periodic())
    }

    /// Wrap this orphan curve as a **wire body** over the parameter interval
    /// `(low, high)` (`PK_CURVE_make_wire_body`; interval passed by value).
    pub fn make_wire_body(&self, interval: (f64, f64)) -> PsResult<Body> {
        let range = PK_INTERVAL_t { low: interval.0, high: interval.1 };
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_CURVE_make_wire_body(self.tag, range, &mut tag));
        Ok(Body::from_tag(tag))
    }

    /// Invert the curve: find the parameter `t` of a position on the curve
    /// (`PK_CURVE_parameterise_vector`). The position should be on (or very
    /// close to) the curve.
    pub fn parameterise(&self, position: Vec3) -> PsResult<f64> {
        let pos = position.to_pk();
        let mut t = 0.0f64;
        pk_call!(PK_CURVE_parameterise_vector(self.tag, &pos, &mut t));
        Ok(t)
    }
}
