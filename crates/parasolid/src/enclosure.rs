//! Conservative enclosures — the Stage 6 contract.
//!
//! Every pruning decision downstream (BVH exclusion, clash pre-filter, SSI
//! candidate rejection) rests on one property: an enclosure must **contain**
//! the geometry. A box that is even slightly *inward* silently turns exclusion
//! into wrong answers, and nothing later can detect it.
//!
//! So the types here separate the two questions the kernel answers differently:
//!
//! - [`Aabb`] from a box finder is a **conservative superset**. It is safe to
//!   exclude against and must not be treated as tight. Measured padding on
//!   analytic primitives is small but nonzero — see `stage6_*` tests.
//! - An *exact* extent, where one exists, comes from geometry-specific
//!   knowledge (a sphere's radius, a face reported as a parametric rectangle by
//!   `Face::as_uvbox`), never from a box finder.
//!
//! The oriented form ([`Obb`]) additionally reports a `dimension`, which says
//! whether the enclosure actually needed three axes — a planar curve boxes into
//! a degenerate slab, and that is information, not an error.

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::error::PsResult;
use crate::geom::Vec3;
use crate::range::Aabb;

/// An oriented (non-axis-aligned) enclosure.
#[derive(Debug, Clone, Copy)]
pub struct OrientedBox {
    /// Centre of the box.
    pub centre: Vec3,
    /// The three box axes.
    pub axes: [Vec3; 3],
    /// **Half**-width (semi-extent) along each axis — despite the vendor
    /// reference calling these "box width in each axis direction".
    /// Measured: a circle of radius 3 reports 3.0, a sphere of radius 4 reports
    /// 4.0, and a line over [0,10] reports 5.0. Doubling them would inflate
    /// every enclosure by 2x.
    pub widths: [f64; 3],
    /// How many axes the enclosure genuinely needed: 3 for a general solid, and
    /// fewer when the geometry is degenerate in some direction (a planar curve,
    /// a straight line). A caller that assumes 3 will mis-handle those.
    pub dimension: i32,
}

impl OrientedBox {
    /// Whether a point lies inside this box, within `tol`.
    pub fn contains(&self, p: Vec3, tol: f64) -> bool {
        let d = Vec3::new(
            p.x - self.centre.x,
            p.y - self.centre.y,
            p.z - self.centre.z,
        );
        (0..3).all(|i| {
            let a = self.axes[i];
            let proj = d.x * a.x + d.y * a.y + d.z * a.z;
            proj.abs() <= self.widths[i] + tol
        })
    }
}

fn aabb_from_pk(b: &PK_BOX_t) -> Aabb {
    Aabb {
        min: Vec3::new(b.coord[0], b.coord[1], b.coord[2]),
        max: Vec3::new(b.coord[3], b.coord[4], b.coord[5]),
    }
}

impl crate::curve::Curve {
    /// Axis-aligned enclosure of this curve, optionally restricted to a
    /// parameter interval (`PK_CURVE_find_box`).
    ///
    /// The result is a **conservative superset**, not a tight box.
    pub fn find_box(&self, interval: Option<(f64, f64)>) -> PsResult<Aabb> {
        let opts = match interval {
            Some((low, high)) => PK_CURVE_find_box_o_t {
                o_t_version: 1,
                have_interval: PK_LOGICAL_true,
                interval: PK_INTERVAL_t { low, high },
            },
            None => PK_CURVE_find_box_o_t::default(),
        };
        let mut b = PK_BOX_t { coord: [0.0; 6] };
        pk_call!(PK_CURVE_find_box(self.tag(), &opts, &mut b));
        Ok(aabb_from_pk(&b))
    }

    /// Oriented enclosure of this curve over a parameter interval
    /// (`PK_CURVE_find_non_aligned_box`).
    pub fn find_oriented_box(&self, interval: (f64, f64)) -> PsResult<OrientedBox> {
        let iv = PK_INTERVAL_t {
            low: interval.0,
            high: interval.1,
        };
        let mut centre = PK_VECTOR_t::default();
        let mut axes = [PK_VECTOR_t::default(); 3];
        let mut widths = [0.0f64; 3];
        let mut dimension: c_int = 0;
        pk_call!(PK_CURVE_find_non_aligned_box(
            self.tag(),
            &iv,
            &mut centre,
            axes.as_mut_ptr(),
            widths.as_mut_ptr(),
            &mut dimension,
        ));
        Ok(OrientedBox {
            centre: Vec3::from_pk(centre),
            axes: [
                Vec3::from_pk(axes[0]),
                Vec3::from_pk(axes[1]),
                Vec3::from_pk(axes[2]),
            ],
            widths,
            dimension,
        })
    }
}

impl crate::surf::Surf {
    /// Axis-aligned enclosure of this surface, optionally restricted to a uv
    /// box (`PK_SURF_find_box`).
    ///
    /// The result is a **conservative superset**. An unrestricted call on an
    /// unbounded surface (a plane, a cylinder's infinite v) boxes the whole
    /// carrier, which is rarely what a caller wants — pass the restriction.
    pub fn find_box(&self, uvbox: Option<crate::surf::UvBox>) -> PsResult<Aabb> {
        let opts = match uvbox {
            Some(b) => PK_SURF_find_box_o_t {
                o_t_version: 1,
                have_uvbox: PK_LOGICAL_true,
                uvbox: PK_UVBOX_t {
                    param: [b.u_min, b.v_min, b.u_max, b.v_max],
                },
            },
            None => PK_SURF_find_box_o_t::default(),
        };
        let mut b = PK_BOX_t { coord: [0.0; 6] };
        pk_call!(PK_SURF_find_box(self.tag(), &opts, &mut b));
        Ok(aabb_from_pk(&b))
    }

    /// Oriented enclosure of this surface over a uv box
    /// (`PK_SURF_find_non_aligned_box`).
    pub fn find_oriented_box(&self, uvbox: crate::surf::UvBox) -> PsResult<OrientedBox> {
        let parms = PK_UVBOX_t {
            param: [uvbox.u_min, uvbox.v_min, uvbox.u_max, uvbox.v_max],
        };
        let mut centre = PK_VECTOR_t::default();
        let mut axes = [PK_VECTOR_t::default(); 3];
        let mut widths = [0.0f64; 3];
        let mut dimension: c_int = 0;
        pk_call!(PK_SURF_find_non_aligned_box(
            self.tag(),
            &parms,
            &mut centre,
            axes.as_mut_ptr(),
            widths.as_mut_ptr(),
            &mut dimension,
        ));
        Ok(OrientedBox {
            centre: Vec3::from_pk(centre),
            axes: [
                Vec3::from_pk(axes[0]),
                Vec3::from_pk(axes[1]),
                Vec3::from_pk(axes[2]),
            ],
            widths,
            dimension,
        })
    }

    /// Global closest approach between this surface and a point
    /// (`PK_GEOM_range_vector`).
    ///
    /// Unlike [`Surf::parameterise`], which is a strict inversion and refuses
    /// off-surface points, this is a genuine projection and the vendor
    /// reference states it finds the **global** closest approach.
    pub fn range_to_point(&self, point: Vec3) -> PsResult<crate::entity::RangeResult> {
        let v: PK_VECTOR_t = [point.x, point.y, point.z];
        let mut opts = PK_GEOM_range_vector_o_t::default();
        let mut status: PK_range_result_t = 0;
        let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_GEOM_range_vector(
            self.tag(),
            &v,
            &mut opts,
            &mut status,
            &mut r,
        ));
        Ok(crate::entity::RangeResult::from_range_1(&r, status, point))
    }
}
