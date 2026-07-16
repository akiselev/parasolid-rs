//! Surface type — geometry attached to faces.

use std::os::raw::c_int;

use parasolid_sys::*;
use crate::error::PsResult;
use crate::entity::Entity;
use crate::curve::Curve;
use crate::geom::{Vec3, Axis2};
use crate::memory::PkArray;

/// Surface curvature at a `(u,v)` point (from [`Surf::eval_curvature`]).
#[derive(Debug, Clone, Copy)]
pub struct SurfCurvature {
    pub normal: Vec3,
    pub principal_direction_1: Vec3,
    pub principal_direction_2: Vec3,
    /// First principal curvature (signed, relative to `normal`).
    pub principal_curvature_1: f64,
    /// Second principal curvature (signed, relative to `normal`).
    pub principal_curvature_2: f64,
}

/// Per-direction parameterisation of a surface (from [`Surf::params`]).
#[derive(Debug, Clone, Copy)]
pub struct SurfDirParam {
    pub range: (f64, f64),
    pub periodic: crate::curve::Periodicity,
    pub closed: bool,
}

/// Spun (surface of revolution) parameters (from [`Surf::ask_spun`]).
#[derive(Debug, Clone, Copy)]
pub struct SpunData {
    pub profile: Curve,
    pub axis_location: Vec3,
    pub axis_direction: Vec3,
}

/// Swept (extruded) surface parameters (from [`Surf::ask_swept`]).
#[derive(Debug, Clone, Copy)]
pub struct SweptData {
    pub profile: Curve,
    pub path: Vec3,
}

/// Offset surface parameters (from [`Surf::ask_offset`]).
#[derive(Debug, Clone, Copy)]
pub struct OffsetData {
    pub basis_surf: Surf,
    pub distance: f64,
}

/// A surface's parametric bounds (from [`Surf::uvbox`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UvBox {
    pub u_min: f64,
    pub v_min: f64,
    pub u_max: f64,
    pub v_max: f64,
}

impl UvBox {
    /// The parametric width in u and v.
    pub fn size(&self) -> (f64, f64) {
        (self.u_max - self.u_min, self.v_max - self.v_min)
    }
}

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

    /// Invert the surface: find the `(u, v)` parameters of a position that lies
    /// on the surface (`PK_SURF_parameterise_vector`). The position should be on
    /// (or very close to) the surface.
    pub fn parameterise(&self, position: Vec3) -> PsResult<(f64, f64)> {
        let pos = position.to_pk();
        let mut uv: PK_UV_t = [0.0; 2];
        pk_call!(PK_SURF_parameterise_vector(self.tag, &pos, &mut uv));
        Ok((uv[0], uv[1]))
    }

    /// The surface's parametric bounds `(u_min, v_min, u_max, v_max)`
    /// (`PK_SURF_ask_uvbox`).
    ///
    /// This encodes the seam/pole conventions CADabra needs: a periodic
    /// direction spans exactly one period (e.g. a cylinder/sphere/torus have
    /// `u ∈ [0, 2π]`, so the seam is at `u = 0 ≡ 2π`); a sphere's `v ∈
    /// [-π/2, π/2]` with poles at the ends; a torus has `v ∈ [-π, π]`; an
    /// unbounded direction is reported as a large finite range (±1e4).
    pub fn uvbox(&self) -> PsResult<UvBox> {
        let mut b = PK_UVBOX_t { param: [0.0; 4] };
        pk_call!(PK_SURF_ask_uvbox(self.tag, &mut b));
        Ok(UvBox { u_min: b.param[0], v_min: b.param[1], u_max: b.param[2], v_max: b.param[3] })
    }

    /// Create a bounded **sheet body** from this surface over the given UV box.
    ///
    /// Wraps `PK_SURF_make_sheet_body`, which takes `PK_UVBOX_t` **by value**
    /// (a 32-byte `[f64; 4]` struct, passed indirectly on the Win64 ABI). For a
    /// plane, the resulting sheet's area is the uvbox's u-extent × v-extent.
    pub fn make_sheet_body(&self, uvbox: UvBox) -> PsResult<crate::body::Body> {
        let uv = PK_UVBOX_t {
            param: [uvbox.u_min, uvbox.v_min, uvbox.u_max, uvbox.v_max],
        };
        let mut tag: PK_BODY_t = PK_ENTITY_null;
        pk_call!(PK_SURF_make_sheet_body(self.tag, uv, &mut tag));
        Ok(crate::body::Body::from_tag(tag))
    }

    /// Evaluate principal curvatures and directions at `(u, v)` via
    /// `PK_SURF_eval_curvature` — the curvature primitive SSI tangency
    /// classification builds on (after position/normal).
    pub fn eval_curvature(&self, u: f64, v: f64) -> PsResult<SurfCurvature> {
        let uv: PK_UV_t = [u, v];
        let mut normal: PK_VECTOR1_t = [0.0; 3];
        let mut d1: PK_VECTOR1_t = [0.0; 3];
        let mut d2: PK_VECTOR1_t = [0.0; 3];
        let mut k1 = 0.0f64;
        let mut k2 = 0.0f64;
        pk_call!(PK_SURF_eval_curvature(self.tag, &uv, &mut normal, &mut d1, &mut d2, &mut k1, &mut k2));
        Ok(SurfCurvature {
            normal: Vec3::from_pk(normal),
            principal_direction_1: Vec3::from_pk(d1),
            principal_direction_2: Vec3::from_pk(d2),
            principal_curvature_1: k1,
            principal_curvature_2: k2,
        })
    }

    /// Parameterisation in U and V (`PK_SURF_ask_params`). Returns `(u, v)`.
    pub fn params(&self) -> PsResult<(SurfDirParam, SurfDirParam)> {
        let mut sf: [PK_PARAM_sf_t; 2] = unsafe { std::mem::zeroed() };
        pk_call!(PK_SURF_ask_params(self.tag, sf.as_mut_ptr()));
        let mk = |p: &PK_PARAM_sf_t| SurfDirParam {
            range: (p.range.low, p.range.high),
            periodic: crate::curve::Periodicity::from_token(p.periodic),
            closed: (p.closed & 0xff) != 0,
        };
        Ok((mk(&sf[0]), mk(&sf[1])))
    }

    /// Create an orphan **spun** surface: revolve `profile` about the axis
    /// through `axis_location` with direction `axis_direction`.
    pub fn spun(profile: &Curve, axis_location: Vec3, axis_direction: Vec3) -> PsResult<Surf> {
        let sf = PK_SPUN_sf_t {
            profile: profile.tag,
            axis: PK_AXIS1_sf_t { location: axis_location.to_pk(), axis: axis_direction.to_pk() },
        };
        let mut tag: PK_SPUN_t = PK_ENTITY_null;
        pk_call!(PK_SPUN_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Read back spun-surface parameters (`PK_SPUN_ask`).
    pub fn ask_spun(&self) -> PsResult<SpunData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_SPUN_sf_t>() };
        pk_call!(PK_SPUN_ask(self.tag, &mut sf));
        Ok(SpunData {
            profile: Curve::from_tag(sf.profile),
            axis_location: Vec3::from_pk(sf.axis.location),
            axis_direction: Vec3::from_pk(sf.axis.axis),
        })
    }

    /// Create an orphan **swept** surface: sweep `profile` along `path`.
    pub fn swept(profile: &Curve, path: Vec3) -> PsResult<Surf> {
        let sf = PK_SWEPT_sf_t { profile: profile.tag, path: path.to_pk() };
        let mut tag: PK_SWEPT_t = PK_ENTITY_null;
        pk_call!(PK_SWEPT_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Read back swept-surface parameters (`PK_SWEPT_ask`).
    pub fn ask_swept(&self) -> PsResult<SweptData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_SWEPT_sf_t>() };
        pk_call!(PK_SWEPT_ask(self.tag, &mut sf));
        Ok(SweptData { profile: Curve::from_tag(sf.profile), path: Vec3::from_pk(sf.path) })
    }

    /// Create an orphan **offset** surface: offset `base` by signed `distance`
    /// along its normal.
    pub fn offset_surface(base: &Surf, distance: f64) -> PsResult<Surf> {
        let sf = PK_OFFSET_sf_t { basis_surf: base.tag, distance };
        let mut tag: PK_OFFSET_t = PK_ENTITY_null;
        pk_call!(PK_OFFSET_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Read back offset-surface parameters (`PK_OFFSET_ask`).
    pub fn ask_offset(&self) -> PsResult<OffsetData> {
        let mut sf = unsafe { std::mem::zeroed::<PK_OFFSET_sf_t>() };
        pk_call!(PK_OFFSET_ask(self.tag, &mut sf));
        Ok(OffsetData { basis_surf: Surf::from_tag(sf.basis_surf), distance: sf.distance })
    }

    /// Create a non-rational **B-surface** (NURBS surface) from its per-direction
    /// degrees, control-point grid (row-major, `n_u × n_v`), and per-direction
    /// **distinct** knots with multiplicities (Parasolid's form).
    #[allow(clippy::too_many_arguments)]
    pub fn bsurf(
        u_degree: i32,
        v_degree: i32,
        n_u: i32,
        n_v: i32,
        control_points: &[Vec3],
        u_knots: &[f64],
        u_mults: &[i32],
        v_knots: &[f64],
        v_mults: &[i32],
    ) -> PsResult<Surf> {
        let verts: Vec<f64> = control_points.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
        let sf = PK_BSURF_sf_t {
            u_degree,
            v_degree,
            n_u_vertices: n_u,
            n_v_vertices: n_v,
            vertex_dim: 3,
            is_rational: PK_LOGICAL_false,
            vertices: verts.as_ptr(),
            _reserved_32: 0,
            n_u_knots: u_knots.len() as c_int,
            n_v_knots: v_knots.len() as c_int,
            u_knot_mult: u_mults.as_ptr(),
            v_knot_mult: v_mults.as_ptr(),
            u_knots: u_knots.as_ptr(),
            v_knots: v_knots.as_ptr(),
            u_knot_type: PK_knot_non_uniform_c,
            v_knot_type: PK_knot_non_uniform_c,
            is_u_periodic: 0,
            is_v_periodic: 0,
            is_u_closed: 0,
            is_v_closed: 0,
            self_intersecting: 0,
            convexity: 0,
        };
        let mut tag: PK_BSURF_t = PK_ENTITY_null;
        pk_call!(PK_BSURF_create(&sf, &mut tag));
        Ok(Surf::from_tag(tag))
    }

    /// Intersect this surface with another (`PK_SURF_intersect_surf`).
    ///
    /// Both surfaces must be orphan geometry or belong to the **same** body.
    /// Returns isolated point intersections and intersection curves (each with
    /// its parameter bounds and raw `PK_intersect_curve_t` kind). Fully
    /// coincident surfaces yield no intersection data.
    pub fn intersect(&self, other: &Surf) -> PsResult<SurfIntersection> {
        // Full documented v1 layout (192 bytes), zero-initialised so every
        // `have_*` flag is false — the kernel reads the whole v1 struct, so a
        // truncated struct would make it read past the allocation.
        let mut opts: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
        opts.o_t_version = 1;
        let mut n_vectors: c_int = 0;
        let mut vectors = std::ptr::null_mut();
        let mut n_curves: c_int = 0;
        let mut curves = std::ptr::null_mut();
        let mut bounds = std::ptr::null_mut();
        let mut types = std::ptr::null_mut();
        pk_call!(PK_SURF_intersect_surf(
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

        Ok(SurfIntersection { points, curves })
    }
}

/// How an intersection curve meets the two surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntersectionKind {
    /// The surfaces cross cleanly along this curve.
    Transversal,
    /// The surfaces touch without crossing along this curve.
    Tangential,
    /// A kind code we have not decoded yet (raw value preserved).
    Other(i32),
}

/// One intersection curve from [`Surf::intersect`].
#[derive(Debug, Clone, Copy)]
pub struct IntersectionCurve {
    /// The basis curve of the intersection.
    pub curve: Curve,
    /// The parameter interval `(low, high)` of the intersection along `curve`.
    pub bounds: (f64, f64),
    /// Raw `PK_intersect_curve_t` kind.
    pub kind: i32,
}

impl IntersectionCurve {
    /// Classify the intersection as transversal / tangential (known kind codes).
    pub fn classify(&self) -> IntersectionKind {
        match self.kind {
            PK_intersect_curve_simple_c => IntersectionKind::Transversal,
            PK_intersect_curve_tangent_c => IntersectionKind::Tangential,
            other => IntersectionKind::Other(other),
        }
    }
}

/// Result of intersecting two surfaces.
#[derive(Debug, Clone)]
pub struct SurfIntersection {
    /// Isolated point (tangential/degenerate) intersections.
    pub points: Vec<Vec3>,
    /// Intersection curves.
    pub curves: Vec<IntersectionCurve>,
}
