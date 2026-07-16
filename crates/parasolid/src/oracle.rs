//! Golden-oracle facade — the stable, **validated-only** comparison surface
//! CADabra's testkit calls.
//!
//! Every item here delegates to a wrapper method that is exercised end-to-end
//! against the real `pskernel.dll` in `parasolid-test` (see `TODO.md` for the
//! per-item validation status). Nothing unvalidated is re-exported through this
//! module, so a caller that stays within `oracle` is guaranteed to be talking
//! to bindings whose numeric/topological output has been checked against closed
//! form or a runtime probe.
//!
//! The surface covers the comparison primitives CADabra needs to diff a
//! CADabra-built model against the Parasolid oracle:
//!
//! - **Primitive construction** — [`block`], [`cylinder`], [`sphere`],
//!   [`cone`], [`torus`].
//! - **Exact geometry sampling** — [`sample_surface`] (position + outward
//!   normal), [`sample_curve`] (position + unit tangent).
//! - **Surface/surface intersection** — [`intersect_surfaces`].
//! - **Coarse invariants** — [`Body::mass_props`](crate::Body::mass_props),
//!   [`Body::bounding_box`](crate::Body::bounding_box),
//!   [`Body::contains_point`](crate::Body::contains_point).
//! - **Structural fingerprint** — [`Body::topology_summary`].
//! - **Model interchange** — [`crate::fileio::transmit`] /
//!   [`crate::fileio::receive`] for whole-body XT round-trips.

use crate::body::Body;
use crate::curve::Curve;
use crate::error::PsResult;
use crate::geom::Vec3;
use crate::surf::{Surf, SurfIntersection};

/// A position on a surface together with the outward unit normal there — the
/// core surface-sampling comparison datum.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceSample {
    pub position: Vec3,
    pub normal: Vec3,
}

/// A position on a curve together with the unit tangent there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CurveSample {
    pub position: Vec3,
    pub tangent: Vec3,
}

/// A structural fingerprint of a body: the entity counts of the whole B-rep
/// spine. Two topologically-identical bodies produce equal summaries, so this
/// is the first-line structural diff for the oracle (geometry is compared
/// separately via sampling).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopologySummary {
    pub regions: usize,
    pub solid_regions: usize,
    pub shells: usize,
    pub faces: usize,
    pub loops: usize,
    pub edges: usize,
    pub vertices: usize,
}

// --- Primitive construction (validated create→ask round-trips) ---------------

/// A solid block with the given full extents (centred in x/y, base at z = 0).
pub fn block(x: f64, y: f64, z: f64) -> PsResult<Body> {
    Body::create_solid_block(x, y, z)
}

/// A solid cylinder of the given radius and height (base on z = 0).
pub fn cylinder(radius: f64, height: f64) -> PsResult<Body> {
    Body::create_solid_cylinder(radius, height)
}

/// A solid sphere of the given radius, centred at the origin.
pub fn sphere(radius: f64) -> PsResult<Body> {
    Body::create_solid_sphere(radius)
}

/// A solid cone of the given base radius, height, and apex half-angle (radians).
pub fn cone(radius: f64, height: f64, semi_angle: f64) -> PsResult<Body> {
    Body::create_solid_cone(radius, height, semi_angle)
}

/// A solid torus of the given major and minor radii.
pub fn torus(major_radius: f64, minor_radius: f64) -> PsResult<Body> {
    Body::create_solid_torus(major_radius, minor_radius)
}

// --- Exact geometry sampling -------------------------------------------------

/// Sample a surface at parameters `(u, v)`: exact position and outward unit
/// normal. Delegates to the validated `PK_SURF_eval` first-derivative frame.
pub fn sample_surface(surf: &Surf, u: f64, v: f64) -> PsResult<SurfaceSample> {
    let (position, normal) = surf.eval_with_normal(u, v)?;
    Ok(SurfaceSample { position, normal })
}

/// Sample a curve at parameter `t`: exact position and **unit** tangent.
///
/// `PK_CURVE_eval` returns the raw first derivative `dP/dt`, whose length is the
/// parametric speed (e.g. the radius for an angle-parameterised circle); this
/// normalises it so the oracle always reports a unit tangent direction. A
/// zero-length derivative (a cusp/degenerate point) is passed through unchanged.
pub fn sample_curve(curve: &Curve, t: f64) -> PsResult<CurveSample> {
    let (position, d) = curve.eval_with_tangent(t)?;
    let len = (d.x * d.x + d.y * d.y + d.z * d.z).sqrt();
    let tangent = if len > 0.0 {
        Vec3::new(d.x / len, d.y / len, d.z / len)
    } else {
        d
    };
    Ok(CurveSample { position, tangent })
}

// --- Surface/surface intersection --------------------------------------------

/// Intersect two orphan surfaces, returning the intersection points and curves
/// (with per-curve parameter bounds and transversal/tangential classification).
pub fn intersect_surfaces(a: &Surf, b: &Surf) -> PsResult<SurfIntersection> {
    a.intersect(b)
}

impl Body {
    /// The body's structural fingerprint — the counts of every level of the
    /// B-rep spine. Cheap and order-independent, so it is the first structural
    /// diff the oracle applies before comparing geometry.
    pub fn topology_summary(&self) -> PsResult<TopologySummary> {
        let regions = self.regions()?;
        let solid_regions = regions.iter().filter(|r| r.is_solid().unwrap_or(false)).count();
        let mut shells = 0usize;
        let mut loops = 0usize;
        for f in self.faces()? {
            loops += f.loops()?.len();
        }
        for r in &regions {
            shells += r.shells()?.len();
        }
        Ok(TopologySummary {
            regions: regions.len(),
            solid_regions,
            shells,
            faces: self.faces()?.len(),
            loops,
            edges: self.edges()?.len(),
            vertices: self.vertices()?.len(),
        })
    }
}
