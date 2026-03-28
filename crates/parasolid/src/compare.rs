//! Comparison utilities for using Parasolid as a validation oracle.
//!
//! Provides tools for extracting surface parameters and sampling face geometry
//! in formats suitable for comparison against other geometric kernels.

use crate::error::PsResult;
use crate::surf::Surf;
use crate::geom::Vec3;

/// A sampled point on a face with position, normal, and UV parameters.
#[derive(Debug, Clone, Copy)]
pub struct SamplePoint {
    pub position: Vec3,
    pub normal: Vec3,
    pub u: f64,
    pub v: f64,
}

/// Analytic surface parameters in a form suitable for cross-kernel comparison.
#[derive(Debug, Clone)]
pub enum SurfaceParams {
    Plane { origin: Vec3, normal: Vec3, ref_dir: Vec3 },
    Cylinder { radius: f64, origin: Vec3, axis: Vec3, ref_dir: Vec3 },
    Cone { radius: f64, semi_angle: f64, origin: Vec3, axis: Vec3, ref_dir: Vec3 },
    Sphere { radius: f64, origin: Vec3, axis: Vec3, ref_dir: Vec3 },
    Torus { major_radius: f64, minor_radius: f64, origin: Vec3, axis: Vec3, ref_dir: Vec3 },
    Other { surf_type: crate::surf::SurfType },
}

/// Extract analytic surface parameters for comparison.
pub fn extract_surface_params(surf: &Surf) -> PsResult<SurfaceParams> {
    use crate::surf::SurfType;
    let st = surf.surf_type()?;
    Ok(match st {
        SurfType::Plane => {
            let d = surf.ask_plane()?;
            SurfaceParams::Plane {
                origin: d.basis.origin,
                normal: d.basis.axis,
                ref_dir: d.basis.ref_direction,
            }
        }
        SurfType::Cylinder => {
            let d = surf.ask_cylinder()?;
            SurfaceParams::Cylinder {
                radius: d.radius,
                origin: d.basis.origin,
                axis: d.basis.axis,
                ref_dir: d.basis.ref_direction,
            }
        }
        SurfType::Cone => {
            let d = surf.ask_cone()?;
            SurfaceParams::Cone {
                radius: d.radius,
                semi_angle: d.semi_angle,
                origin: d.basis.origin,
                axis: d.basis.axis,
                ref_dir: d.basis.ref_direction,
            }
        }
        SurfType::Sphere => {
            let d = surf.ask_sphere()?;
            SurfaceParams::Sphere {
                radius: d.radius,
                origin: d.basis.origin,
                axis: d.basis.axis,
                ref_dir: d.basis.ref_direction,
            }
        }
        SurfType::Torus => {
            let d = surf.ask_torus()?;
            SurfaceParams::Torus {
                major_radius: d.major_radius,
                minor_radius: d.minor_radius,
                origin: d.basis.origin,
                axis: d.basis.axis,
                ref_dir: d.basis.ref_direction,
            }
        }
        other => SurfaceParams::Other { surf_type: other },
    })
}
