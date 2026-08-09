//! Derivative jets — the Stage 3 evaluation primitive.
//!
//! A *jet* is a position plus a table of partial derivatives at a parameter.
//! Every later algorithm (SSI, tangency classification, curvature, offsets,
//! faceting) reads derivatives out of such a table, so the indexing has to be
//! right once, here, rather than re-derived by each caller from a flat `&[f64]`.
//!
//! # Layout, as measured
//!
//! `PK_SURF_eval` writes `(i, j)` = ∂^(i+j)R/∂u^i∂v^j with **u varying
//! fastest**. Recovered by evaluating a torus — whose mixed partials are all
//! nonzero, so no ordering can hide — and matching every slot against closed
//! form (`crates/parasolid-test/src/bin/eval_probe.rs`):
//!
//! ```text
//! rectangular, n_u = n_v = 2:
//!   [0]=(0,0) [1]=(1,0) [2]=(2,0)
//!   [3]=(0,1) [4]=(1,1) [5]=(2,1)
//!   [6]=(0,2) [7]=(1,2) [8]=(2,2)
//!   index = j*(n_u+1) + i,  slot count = (n_u+1)*(n_v+1)
//!
//! triangular, order 2 — the same ordering with each row truncated to i+j <= n:
//!   [0]=(0,0) [1]=(1,0) [2]=(2,0)
//!   [3]=(0,1) [4]=(1,1)
//!   [5]=(0,2)
//!   index = j*(n+1) - j*(j-1)/2 + i,  slot count = (n+1)*(n+2)/2
//! ```
//!
//! `PK_CURVE_eval` is the trivial case: slot `k` is d^k/dt^k for `k = 0..=n`.

use crate::error::PsResult;
use crate::geom::Vec3;
use parasolid_sys::*;

// =============================================================================
// SurfJet
// =============================================================================

/// Position and partial derivatives of a surface at one `(u, v)`.
///
/// Index with [`SurfJet::d`] rather than reaching into the raw slots — the
/// packing differs between rectangular and triangular tables.
#[derive(Debug, Clone)]
pub struct SurfJet {
    n_u: usize,
    n_v: usize,
    triangular: bool,
    data: Vec<Vec3>,
}

impl SurfJet {
    /// Number of slots a table of this shape occupies.
    fn slot_count(n_u: usize, n_v: usize, triangular: bool) -> usize {
        if triangular {
            let n = n_u.max(n_v);
            (n + 1) * (n + 2) / 2
        } else {
            (n_u + 1) * (n_v + 1)
        }
    }

    /// Slot holding ∂^(i+j)R/∂u^i∂v^j, or `None` when the table does not carry
    /// that derivative.
    fn index_of(&self, i: usize, j: usize) -> Option<usize> {
        if self.triangular {
            let n = self.n_u.max(self.n_v);
            if i + j > n {
                return None;
            }
            // Rows shorten by one each time j increases.
            Some(j * (n + 1) - j * j.saturating_sub(1) / 2 + i)
        } else {
            if i > self.n_u || j > self.n_v {
                return None;
            }
            Some(j * (self.n_u + 1) + i)
        }
    }

    /// ∂^(i+j)R/∂u^i∂v^j, or `None` if this jet does not carry it.
    pub fn d(&self, i: usize, j: usize) -> Option<Vec3> {
        self.index_of(i, j).and_then(|k| self.data.get(k).copied())
    }

    /// The evaluated position, R(u, v).
    pub fn position(&self) -> Vec3 {
        self.data[0]
    }

    /// ∂R/∂u.
    pub fn du(&self) -> Option<Vec3> {
        self.d(1, 0)
    }

    /// ∂R/∂v.
    pub fn dv(&self) -> Option<Vec3> {
        self.d(0, 1)
    }

    /// The highest orders this jet carries, and whether it is triangular.
    pub fn shape(&self) -> (usize, usize, bool) {
        (self.n_u, self.n_v, self.triangular)
    }

    /// Unnormalised surface normal ∂R/∂u × ∂R/∂v.
    ///
    /// Returns `None` when the jet lacks first derivatives. The magnitude is
    /// meaningful: it vanishes at a parametric singularity, which is why this
    /// is exposed separately from the unit normal.
    pub fn normal_unnormalised(&self) -> Option<Vec3> {
        let (du, dv) = (self.du()?, self.dv()?);
        Some(Vec3::new(
            du.y * dv.z - du.z * dv.y,
            du.z * dv.x - du.x * dv.z,
            du.x * dv.y - du.y * dv.x,
        ))
    }

    /// Unit surface normal, or `None` at a parametric singularity.
    ///
    /// `None` here is a real geometric statement — the parameterisation
    /// degenerates (a sphere pole, a cone apex) and no normal is defined *by
    /// this chart*. It is not an error, and callers must not paper over it with
    /// a fallback direction.
    pub fn unit_normal(&self) -> Option<Vec3> {
        let n = self.normal_unnormalised()?;
        let len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
        if !len.is_finite() || len < 1.0e-14 {
            return None;
        }
        Some(Vec3::new(n.x / len, n.y / len, n.z / len))
    }

    /// Whether the parameterisation is singular here (zero-length normal).
    pub fn is_singular(&self) -> bool {
        self.unit_normal().is_none()
    }
}

impl crate::surf::Surf {
    /// Evaluate a full derivative jet at `(u, v)`.
    ///
    /// `n_u` / `n_v` are the highest derivative orders wanted in each
    /// direction. `triangular` selects the packed table that omits terms with
    /// `i + j > max(n_u, n_v)` — cheaper, and the natural shape when only total
    /// order matters.
    pub fn eval_jet(
        &self,
        u: f64,
        v: f64,
        n_u: usize,
        n_v: usize,
        triangular: bool,
    ) -> PsResult<SurfJet> {
        let count = SurfJet::slot_count(n_u, n_v, triangular);
        let mut raw = vec![0.0f64; count * 3];
        let uv = [u, v];
        pk_call!(PK_SURF_eval(
            self.tag(),
            uv.as_ptr(),
            n_u as std::os::raw::c_int,
            n_v as std::os::raw::c_int,
            if triangular {
                PK_LOGICAL_true
            } else {
                PK_LOGICAL_false
            },
            raw.as_mut_ptr()
        ));
        let data = raw
            .chunks_exact(3)
            .map(|c| Vec3::new(c[0], c[1], c[2]))
            .collect();
        Ok(SurfJet {
            n_u,
            n_v,
            triangular,
            data,
        })
    }
}

// =============================================================================
// CurveJet
// =============================================================================

/// Position and derivatives of a curve at one parameter.
#[derive(Debug, Clone)]
pub struct CurveJet {
    pub(crate) data: Vec<Vec3>,
}

impl CurveJet {
    /// d^k/dt^k, or `None` if this jet does not carry order `k`.
    pub fn d(&self, k: usize) -> Option<Vec3> {
        self.data.get(k).copied()
    }

    /// The evaluated position.
    pub fn position(&self) -> Vec3 {
        self.data[0]
    }

    /// Highest derivative order carried.
    pub fn order(&self) -> usize {
        self.data.len() - 1
    }

    /// Unit tangent, or `None` where the first derivative vanishes (a
    /// parameterisation stationary point — a genuine geometric statement, not
    /// an error).
    pub fn unit_tangent(&self) -> Option<Vec3> {
        let d1 = self.d(1)?;
        let len = (d1.x * d1.x + d1.y * d1.y + d1.z * d1.z).sqrt();
        if !len.is_finite() || len < 1.0e-14 {
            return None;
        }
        Some(Vec3::new(d1.x / len, d1.y / len, d1.z / len))
    }
}

impl crate::curve::Curve {
    /// Evaluate a derivative jet at `t`, up to order `n_deriv`.
    pub fn eval_jet(&self, t: f64, n_deriv: usize) -> PsResult<CurveJet> {
        let mut raw = vec![0.0f64; (n_deriv + 1) * 3];
        pk_call!(PK_CURVE_eval(
            self.tag(),
            t,
            n_deriv as std::os::raw::c_int,
            raw.as_mut_ptr()
        ));
        Ok(CurveJet {
            data: raw
                .chunks_exact(3)
                .map(|c| Vec3::new(c[0], c[1], c[2]))
                .collect(),
        })
    }
}

// =============================================================================
// Minimum radius of curvature
// =============================================================================

/// A minimum of the radius of curvature, located in parameter space.
#[derive(Debug, Clone, Copy)]
pub struct MinRadius {
    /// The minimum radius of curvature.
    pub radius: f64,
    /// Where on the geometry it occurs.
    pub position: Vec3,
    /// The parameter at which it occurs — `t` for a curve, `(u, v)` for a
    /// surface.
    pub param: (f64, f64),
}

impl crate::curve::Curve {
    /// Minimum radius of curvature over a parameter interval
    /// (`PK_CURVE_find_min_radius`).
    ///
    /// Returns `None` when the curve has no curvature minimum in the interval —
    /// a straight line has no finite radius anywhere, and that is an answer,
    /// not a failure. Callers must not substitute infinity.
    pub fn find_min_radius(&self, t_low: f64, t_high: f64) -> PsResult<Option<MinRadius>> {
        let interval = PK_INTERVAL_t {
            low: t_low,
            high: t_high,
        };
        let mut n_radii: std::os::raw::c_int = 0;
        let mut radius = 0.0f64;
        let mut position: PK_VECTOR_t = [0.0; 3];
        let mut param = 0.0f64;
        pk_call!(PK_CURVE_find_min_radius(
            self.tag(),
            &interval,
            &mut n_radii,
            &mut radius,
            &mut position,
            &mut param
        ));
        if n_radii == 0 {
            return Ok(None);
        }
        Ok(Some(MinRadius {
            radius,
            position: Vec3::from_pk(position),
            param: (param, 0.0),
        }))
    }
}

impl crate::surf::Surf {
    /// Minimum radii of curvature over a uv box (`PK_SURF_find_min_radii`).
    ///
    /// The kernel reports **at most two** minima; an empty result means the
    /// surface has no curvature minimum in the box (a plane).
    pub fn find_min_radii(&self, uvbox: crate::surf::UvBox) -> PsResult<Vec<MinRadius>> {
        let box_ = PK_UVBOX_t {
            param: [uvbox.u_min, uvbox.v_min, uvbox.u_max, uvbox.v_max],
        };
        let mut n_radii: std::os::raw::c_int = 0;
        // The kernel writes up to two entries into each buffer.
        let mut radii = [0.0f64; 2];
        let mut positions: [PK_VECTOR_t; 2] = [[0.0; 3]; 2];
        let mut parms: [PK_UV_t; 2] = [[0.0; 2]; 2];
        pk_call!(PK_SURF_find_min_radii(
            self.tag(),
            &box_,
            &mut n_radii,
            radii.as_mut_ptr(),
            positions.as_mut_ptr(),
            parms.as_mut_ptr()
        ));
        Ok((0..n_radii.clamp(0, 2) as usize)
            .map(|k| MinRadius {
                radius: radii[k],
                position: Vec3::from_pk(positions[k]),
                param: (parms[k][0], parms[k][1]),
            })
            .collect())
    }
}

// =============================================================================
// Handed evaluation
// =============================================================================

/// Which side of a parameter to evaluate from, for curves whose derivatives are
/// discontinuous there (`PK_HAND_t`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

impl Hand {
    fn token(self) -> PK_HAND_t {
        match self {
            Hand::Left => PK_HAND_left_c,
            Hand::Right => PK_HAND_right_c,
        }
    }
}

impl crate::curve::Curve {
    /// Evaluate a jet approaching `t` from one side (`PK_CURVE_eval_handed`).
    ///
    /// For a G1-continuous curve both hands agree. They differ only where the
    /// derivative genuinely jumps — so a caller that needs a one-sided
    /// derivative must ask for it rather than hoping the two-sided form picks
    /// the right branch.
    pub fn eval_jet_handed(&self, t: f64, n_deriv: usize, hand: Hand) -> PsResult<CurveJet> {
        let mut raw = vec![[0.0f64; 3]; n_deriv + 1];
        pk_call!(PK_CURVE_eval_handed(
            self.tag(),
            t,
            n_deriv as std::os::raw::c_int,
            hand.token(),
            raw.as_mut_ptr()
        ));
        Ok(CurveJet {
            data: raw.into_iter().map(Vec3::from_pk).collect(),
        })
    }
}
