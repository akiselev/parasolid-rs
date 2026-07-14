//! Surface type — geometry attached to faces.

use parasolid_sys::*;
use crate::error::PsResult;
use crate::entity::Entity;
use crate::geom::{Vec3, Axis2};

/// Concrete surface type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfType {
    Plane, Cylinder, Cone, Sphere, Torus,
    Bsurf, Offset, Swept, Spun, Fsurf, Mesh, Blendsf, Ssurf,
}

/// A surface entity handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Surf {
    pub(crate) tag: PK_SURF_t,
}

/// Plane parameters: basis frame (origin + normal axis + reference direction).
#[derive(Debug, Clone, Copy)]
pub struct PlaneData { pub basis: Axis2 }

/// Cylinder parameters: radius and axis frame.
#[derive(Debug, Clone, Copy)]
pub struct CylinderData { pub radius: f64, pub basis: Axis2 }

/// Cone parameters: `radius` is the cone radius **at the basis frame's origin**
/// (not at the apex), `semi_angle` is the half-angle in radians, and `basis` is
/// the axis frame. The radius grows with distance along `+axis` as
/// `radius + t·tan(semi_angle)`.
#[derive(Debug, Clone, Copy)]
pub struct ConeData { pub radius: f64, pub semi_angle: f64, pub basis: Axis2 }

/// Sphere parameters: radius and basis frame.
#[derive(Debug, Clone, Copy)]
pub struct SphereData { pub radius: f64, pub basis: Axis2 }

/// Torus parameters: major and minor radii, and axis frame.
#[derive(Debug, Clone, Copy)]
pub struct TorusData { pub major_radius: f64, pub minor_radius: f64, pub basis: Axis2 }

impl Surf {
    pub(crate) fn from_tag(tag: PK_SURF_t) -> Self { Self { tag } }
    pub fn tag(&self) -> i32 { self.tag }
    pub fn entity(&self) -> Entity { Entity::from_tag(self.tag) }

    /// Determine the concrete surface type.
    pub fn surf_type(&self) -> PsResult<SurfType> {
        let class = self.entity().class()?;
        use crate::entity::PkClass;
        Ok(match class {
            PkClass::Plane => SurfType::Plane,
            PkClass::Cyl => SurfType::Cylinder,
            PkClass::Cone => SurfType::Cone,
            PkClass::Sphere => SurfType::Sphere,
            PkClass::Torus => SurfType::Torus,
            PkClass::Bsurf => SurfType::Bsurf,
            PkClass::Offset => SurfType::Offset,
            PkClass::Swept => SurfType::Swept,
            PkClass::Spun => SurfType::Spun,
            PkClass::Fsurf => SurfType::Fsurf,
            PkClass::Mesh => SurfType::Mesh,
            PkClass::Blendsf => SurfType::Blendsf,
            PkClass::Ssurf => SurfType::Ssurf,
            _ => return Err(crate::error::PsError::Session(
                format!("entity {} is not a surface (class {:?})", self.tag, class)
            )),
        })
    }

    /// Extract plane parameters.
    pub fn ask_plane(&self) -> PsResult<PlaneData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_PLANE_sf_t>() };
        pk_call!(PK_PLANE_ask(self.tag, &mut sf));
        Ok(PlaneData { basis: Axis2::from_pk(sf.basis_set) })
    }

    /// Extract cylinder parameters.
    pub fn ask_cylinder(&self) -> PsResult<CylinderData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_CYL_sf_t>() };
        pk_call!(PK_CYL_ask(self.tag, &mut sf));
        Ok(CylinderData { radius: sf.radius, basis: Axis2::from_pk(sf.basis_set) })
    }

    /// Extract cone parameters.
    pub fn ask_cone(&self) -> PsResult<ConeData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_CONE_sf_t>() };
        pk_call!(PK_CONE_ask(self.tag, &mut sf));
        Ok(ConeData { radius: sf.radius, semi_angle: sf.semi_angle, basis: Axis2::from_pk(sf.basis_set) })
    }

    /// Extract sphere parameters.
    pub fn ask_sphere(&self) -> PsResult<SphereData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_SPHERE_sf_t>() };
        pk_call!(PK_SPHERE_ask(self.tag, &mut sf));
        Ok(SphereData { radius: sf.radius, basis: Axis2::from_pk(sf.basis_set) })
    }

    /// Extract torus parameters.
    pub fn ask_torus(&self) -> PsResult<TorusData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_TORUS_sf_t>() };
        pk_call!(PK_TORUS_ask(self.tag, &mut sf));
        Ok(TorusData { major_radius: sf.major_radius, minor_radius: sf.minor_radius, basis: Axis2::from_pk(sf.basis_set) })
    }

    /// Evaluate surface position at (u, v).
    ///
    /// Calls `PK_SURF_eval` with zero derivatives; result is the 3D point R(u,v).
    pub fn eval(&self, u: f64, v: f64) -> PsResult<Vec3> {
        let uv = [u, v];
        let mut p = [0.0f64; 3];
        pk_call!(PK_SURF_eval(
            self.tag,
            uv.as_ptr(),
            0,
            0,
            PK_LOGICAL_false,
            p.as_mut_ptr()
        ));
        Ok(Vec3::from_pk(p))
    }

    /// Evaluate surface position and parametric normal at (u, v).
    ///
    /// Calls `PK_SURF_eval` with first-order derivatives in both u and v
    /// (rectangular layout). The normal is the cross product
    /// dR/du x dR/dv, normalised to unit length.
    ///
    /// # Face orientation
    ///
    /// This returns the **surface** normal (from parametric derivatives), not
    /// the face's outward normal. When a face's orientation is reversed
    /// (`Face::oriented_surf()` returns `sense = false`), the face outward
    /// normal is the negation of this value.
    ///
    /// Returns `Err` at surface singularities where the normal is degenerate.
    pub fn eval_with_normal(&self, u: f64, v: f64) -> PsResult<(Vec3, Vec3)> {
        let uv = [u, v];
        // n_u_deriv=1, n_v_deriv=1, triangular=false: rectangular layout writes
        // (n_u+1)*(n_v+1) = 4 vectors = 12 doubles:
        //   p[0..3]   = R(u,v)       (i=0,j=0)
        //   p[3..6]   = dR/du        (i=1,j=0)
        //   p[6..9]   = dR/dv        (i=0,j=1)
        //   p[9..12]  = d²R/dudv     (i=1,j=1) — unused but must be allocated
        let mut p = [0.0f64; 12];
        pk_call!(PK_SURF_eval(
            self.tag,
            uv.as_ptr(),
            1,
            1,
            PK_LOGICAL_false,
            p.as_mut_ptr()
        ));
        let pos = Vec3::new(p[0], p[1], p[2]);
        let du = Vec3::new(p[3], p[4], p[5]);
        let dv = Vec3::new(p[6], p[7], p[8]);
        // Normal = dR/du x dR/dv, normalised.
        let nx = du.y * dv.z - du.z * dv.y;
        let ny = du.z * dv.x - du.x * dv.z;
        let nz = du.x * dv.y - du.y * dv.x;
        let len = (nx * nx + ny * ny + nz * nz).sqrt();
        if len <= 1e-15 {
            return Err(crate::error::PsError::Session(format!(
                "degenerate surface normal at u={}, v={}: |dR/du x dR/dv| = {:.2e}", u, v, len
            )));
        }
        let normal = Vec3::new(nx / len, ny / len, nz / len);
        Ok((pos, normal))
    }
}
