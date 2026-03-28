//! Curve type — geometry attached to edges.

use parasolid_sys::*;
use crate::error::PsResult;
use crate::entity::Entity;
use crate::geom::{Vec3, Axis2};

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
}
