//! Integration tests for parasolid-sys and parasolid crates.
//!
//! Build: cargo build -p parasolid-test --target x86_64-pc-windows-gnu
//! Run:   WINEPATH=/path/to/SOLIDWORKS cargo run -p parasolid-test --target x86_64-pc-windows-gnu

use parasolid::*;

/// Session config used by every test: argument checking on, so the kernel
/// validates our FFI arguments and surfaces struct/signature mismatches early.
fn test_config() -> SessionConfig {
    SessionConfig::new().check_arguments(true)
}

fn main() {
    println!("=== Parasolid Integration Tests ===\n");

    // Diagnostic: try raw session start to capture exact error code
    println!("  [diag] Attempting PK_SESSION_start via safe wrapper...");

    let mut passed = 0;
    let mut failed = 0;

    macro_rules! test {
        ($name:expr, $body:block) => {
            print!("  {} ... ", $name);
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> Result<(), Box<dyn std::error::Error>> { $body; Ok(()) })) {
                Ok(Ok(())) => { println!("OK"); passed += 1; }
                Ok(Err(e)) => { println!("FAIL: {}", e); failed += 1; }
                Err(p) => {
                    let msg = p.downcast_ref::<&str>().map(|s| s.to_string())
                        .or_else(|| p.downcast_ref::<String>().cloned())
                        .unwrap_or_else(|| "unknown panic".to_string());
                    println!("PANIC: {}", msg);
                    failed += 1;
                }
            }
        };
    }

    // =========================================================================
    // Session lifecycle
    // =========================================================================

    test!("session_start_stop", {
        let session = Session::start(test_config())?;
        let (major, minor, _patch) = session.kernel_version()?;
        assert!(major >= 30, "kernel version too old: {}.{}", major, minor);
        println!("(v{}.{}) ", major, minor);
        drop(session);
    });

    // =========================================================================
    // Body creation
    // =========================================================================

    test!("create_solid_block", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 20.0, 30.0)?;
        assert_eq!(body.body_type()?, BodyType::Solid);
        let faces = body.faces()?;
        assert_eq!(faces.len(), 6, "block should have 6 faces, got {}", faces.len());
        let edges = body.edges()?;
        assert_eq!(edges.len(), 12, "block should have 12 edges, got {}", edges.len());
        let verts = body.vertices()?;
        assert_eq!(verts.len(), 8, "block should have 8 vertices, got {}", verts.len());
    });

    test!("create_solid_cylinder", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_cylinder(5.0, 20.0)?;
        assert_eq!(body.body_type()?, BodyType::Solid);
        let faces = body.faces()?;
        assert_eq!(faces.len(), 3, "cylinder should have 3 faces, got {}", faces.len());
    });

    test!("create_solid_sphere", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_sphere(10.0)?;
        assert_eq!(body.body_type()?, BodyType::Solid);
        let faces = body.faces()?;
        assert_eq!(faces.len(), 1, "sphere should have 1 face, got {}", faces.len());
    });

    // =========================================================================
    // Topology navigation
    // =========================================================================

    test!("face_edges_vertices", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 10.0, 10.0)?;
        let faces = body.faces()?;
        for face in &faces {
            let edges = face.edges()?;
            assert_eq!(edges.len(), 4, "block face should have 4 edges");
            let verts = face.vertices()?;
            assert_eq!(verts.len(), 4, "block face should have 4 vertices");
            // Face should know its body
            let owner = face.body()?;
            assert_eq!(owner.tag(), body.tag());
        }
    });

    test!("edge_vertices", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 10.0, 10.0)?;
        let edges = body.edges()?;
        for edge in &edges {
            let (v0, v1) = edge.vertices()?;
            let p0 = v0.point()?;
            let p1 = v1.point()?;
            // Each edge of a 10x10x10 block has length 10
            let dx = p1.x - p0.x;
            let dy = p1.y - p0.y;
            let dz = p1.z - p0.z;
            let len = (dx*dx + dy*dy + dz*dz).sqrt();
            assert!((len - 10.0).abs() < 1e-6, "edge length should be 10, got {}", len);
        }
    });

    test!("vertex_position", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let verts = body.vertices()?;
        // Per PK docs, the block's BASE is centred at the origin:
        // vertices at x = ±5, y = ±10, z = 0 or 30.
        for v in &verts {
            let p = v.point()?;
            assert!((p.x.abs() - 5.0).abs() < 1e-6, "x should be ±5, got {}", p.x);
            assert!((p.y.abs() - 10.0).abs() < 1e-6, "y should be ±10, got {}", p.y);
            assert!(
                p.z.abs() < 1e-6 || (p.z - 30.0).abs() < 1e-6,
                "z should be 0 or 30, got {}",
                p.z
            );
        }
    });

    // =========================================================================
    // Geometry interrogation
    // =========================================================================

    test!("block_face_surface_type", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 10.0, 10.0)?;
        for face in body.faces()? {
            let surf = face.surf()?;
            assert_eq!(surf.surf_type()?, SurfType::Plane);
        }
    });

    test!("cylinder_face_surface_types", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_cylinder(5.0, 20.0)?;
        let mut has_cyl = false;
        let mut has_plane = false;
        for face in body.faces()? {
            let surf = face.surf()?;
            match surf.surf_type()? {
                SurfType::Cylinder => has_cyl = true,
                SurfType::Plane => has_plane = true,
                other => panic!("unexpected surface type: {:?}", other),
            }
        }
        assert!(has_cyl, "cylinder body should have cylindrical face");
        assert!(has_plane, "cylinder body should have planar caps");
    });

    test!("sphere_surface_params", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_sphere(25.0)?;
        let face = &body.faces()?[0];
        let surf = face.surf()?;
        let data = surf.ask_sphere()?;
        assert!((data.radius - 25.0).abs() < 1e-10, "radius should be 25, got {}", data.radius);
    });

    test!("surface_eval", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_sphere(10.0)?;
        let face = &body.faces()?[0];
        let surf = face.surf()?;
        // Evaluate at some parameter
        let pos = surf.eval(0.5, 0.5)?;
        let dist = (pos.x*pos.x + pos.y*pos.y + pos.z*pos.z).sqrt();
        assert!((dist - 10.0).abs() < 1e-6, "point should be on sphere (r=10), dist={}", dist);
    });

    test!("edge_curve_type", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 10.0, 10.0)?;
        for edge in body.edges()? {
            let curve = edge.curve()?;
            assert_eq!(curve.curve_type()?, CurveType::Line);
        }
    });

    test!("curve_eval", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 10.0, 10.0)?;
        let edge = &body.edges()?[0];
        let curve = edge.curve()?;
        let (t0, t1) = edge.interval()?;
        let p0 = curve.eval(t0)?;
        let p1 = curve.eval(t1)?;
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let dz = p1.z - p0.z;
        let len = (dx*dx + dy*dy + dz*dz).sqrt();
        assert!((len - 10.0).abs() < 1e-6, "edge length should be 10, got {}", len);
    });

    // =========================================================================
    // Compare module
    // =========================================================================

    test!("extract_surface_params", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_sphere(15.0)?;
        let surf = body.faces()?[0].surf()?;
        let params = extract_surface_params(&surf)?;
        match params {
            SurfaceParams::Sphere { radius, .. } => {
                assert!((radius - 15.0).abs() < 1e-10);
            }
            _ => panic!("expected sphere params"),
        }
    });

    // =========================================================================
    // P0 — argument checking is actually on (oracle self-trust)
    // =========================================================================

    test!("check_arguments_enabled", {
        let session = Session::start(test_config())?;
        assert!(session.check_arguments()?, "check_arguments should be enabled");
    });

    // =========================================================================
    // P5 — mass / area / inertia oracle (closed-form invariants)
    // =========================================================================
    //
    // Default body density is 1.0, so `mass == amount == volume` for solids and
    // `periphery` is the total surface area. The option struct layout and enum
    // tokens were recovered from the DLL (see docs/pskernel-solidworks.md) and
    // are asserted here against exact closed-form values with check_arguments on.

    const MP_REL: f64 = 1e-6; // relative tolerance for analytic primitives
    fn rel_ok(got: f64, want: f64) -> bool {
        (got - want).abs() <= MP_REL * want.abs().max(1.0)
    }
    fn near0(v: f64, scale: f64) -> bool {
        v.abs() <= MP_REL * scale.abs().max(1.0)
    }

    test!("massprops_block", {
        let _session = Session::start(test_config())?;
        let (x, y, z) = (10.0, 20.0, 30.0);
        let body = Body::create_solid_block(x, y, z)?;
        let mp = body.mass_props()?;
        let vol = x * y * z;
        let area = 2.0 * (x * y + y * z + z * x);
        assert!(rel_ok(mp.amount, vol), "block volume {} != {}", mp.amount, vol);
        assert!(rel_ok(mp.mass, vol), "block mass {} != {}", mp.mass, vol);
        assert!(rel_ok(mp.periphery, area), "block area {} != {}", mp.periphery, area);
        // Base centred at origin, z spans 0..z → CoG = (0, 0, z/2).
        let cg = mp.center_of_gravity;
        assert!(near0(cg.x, x) && near0(cg.y, y), "block CoG x/y not ~0: {:?}", cg);
        assert!((cg.z - z / 2.0).abs() < 1e-6, "block CoG z {} != {}", cg.z, z / 2.0);
        // Solid block inertia about CoG (m=vol): Ixx=m/12(y^2+z^2), etc.
        let (ixx, iyy, izz) = (
            vol / 12.0 * (y * y + z * z),
            vol / 12.0 * (x * x + z * z),
            vol / 12.0 * (x * x + y * y),
        );
        assert!(rel_ok(mp.inertia[0], ixx), "block Ixx {} != {}", mp.inertia[0], ixx);
        assert!(rel_ok(mp.inertia[4], iyy), "block Iyy {} != {}", mp.inertia[4], iyy);
        assert!(rel_ok(mp.inertia[8], izz), "block Izz {} != {}", mp.inertia[8], izz);
        for k in [1usize, 2, 3, 5, 6, 7] {
            assert!(near0(mp.inertia[k], ixx), "block off-diag[{}] {} not ~0", k, mp.inertia[k]);
        }
    });

    test!("massprops_sphere", {
        let _session = Session::start(test_config())?;
        let r = 15.0;
        let body = Body::create_solid_sphere(r)?;
        let mp = body.mass_props()?;
        let vol = 4.0 / 3.0 * std::f64::consts::PI * r.powi(3);
        let area = 4.0 * std::f64::consts::PI * r * r;
        assert!(rel_ok(mp.amount, vol), "sphere volume {} != {}", mp.amount, vol);
        assert!(rel_ok(mp.periphery, area), "sphere area {} != {}", mp.periphery, area);
        let cg = mp.center_of_gravity;
        assert!(near0(cg.x, r) && near0(cg.y, r) && near0(cg.z, r),
            "sphere CoG not ~origin: {:?}", cg);
        // Solid sphere inertia about CoG: I = 2/5 m r^2 on the diagonal, 0 off.
        let i_diag = 2.0 / 5.0 * mp.mass * r * r;
        for k in [0usize, 4, 8] {
            assert!(rel_ok(mp.inertia[k], i_diag), "sphere I diag[{}] {} != {}", k, mp.inertia[k], i_diag);
        }
    });

    test!("massprops_cylinder", {
        let _session = Session::start(test_config())?;
        let (r, h) = (5.0, 12.0);
        let body = Body::create_solid_cylinder(r, h)?;
        let mp = body.mass_props()?;
        let vol = std::f64::consts::PI * r * r * h;
        let area = 2.0 * std::f64::consts::PI * r * r + 2.0 * std::f64::consts::PI * r * h;
        assert!(rel_ok(mp.amount, vol), "cyl volume {} != {}", mp.amount, vol);
        assert!(rel_ok(mp.periphery, area), "cyl area {} != {}", mp.periphery, area);
        // Base on z=0 plane → centroid at z = h/2, centred on the Z axis.
        let cg = mp.center_of_gravity;
        assert!(near0(cg.x, r) && near0(cg.y, r), "cyl CoG x/y not ~0: {:?}", cg);
        assert!((cg.z - h / 2.0).abs() < 1e-6, "cyl CoG z {} != {}", cg.z, h / 2.0);
        // Cylinder about its axis: Izz = 1/2 m r^2.
        let izz = 0.5 * mp.mass * r * r;
        assert!(rel_ok(mp.inertia[8], izz), "cyl Izz {} != {}", mp.inertia[8], izz);
    });

    test!("massprops_cone_truncated", {
        let _session = Session::start(test_config())?;
        // Frustum: base radius rb at z=0, height h, semi-angle 45° → widens to
        // rt = rb + h*tan(a). Volume = pi*h/3*(rb^2 + rb*rt + rt^2).
        let (rb, h) = (5.0, 3.0);
        let semi = std::f64::consts::FRAC_PI_4;
        let rt = rb + h * semi.tan();
        let body = Body::create_solid_cone(rb, h, semi)?;
        let mp = body.mass_props()?;
        let vol = std::f64::consts::PI * h / 3.0 * (rb * rb + rb * rt + rt * rt);
        assert!(rel_ok(mp.amount, vol), "cone volume {} != {}", mp.amount, vol);
    });

    test!("massprops_torus", {
        let _session = Session::start(test_config())?;
        let (major, minor) = (10.0, 3.0);
        let body = Body::create_solid_torus(major, minor)?;
        let mp = body.mass_props()?;
        let vol = 2.0 * std::f64::consts::PI.powi(2) * major * minor * minor;
        let area = 4.0 * std::f64::consts::PI.powi(2) * major * minor;
        assert!(rel_ok(mp.amount, vol), "torus volume {} != {}", mp.amount, vol);
        assert!(rel_ok(mp.periphery, area), "torus area {} != {}", mp.periphery, area);
        // Centred at the origin, major axis along Z.
        let cg = mp.center_of_gravity;
        assert!(near0(cg.x, major) && near0(cg.y, major) && near0(cg.z, minor),
            "torus CoG not ~origin: {:?}", cg);
    });

    // =========================================================================
    // P1 — standalone analytic geometry: create -> ask round-trips
    // =========================================================================

    test!("create_ask_roundtrips", {
        let _session = Session::start(test_config())?;
        let zbasis = |o: Vec3| Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));

        let pl = Surf::plane(zbasis(Vec3::new(0.0, 0.0, 5.0)))?;
        assert_eq!(pl.surf_type()?, SurfType::Plane);
        assert!(rel_ok(pl.ask_plane()?.basis.origin.z, 5.0), "plane origin z");

        let sp = Surf::sphere(zbasis(Vec3::new(1.0, 2.0, 3.0)), 4.0)?;
        let spd = sp.ask_sphere()?;
        assert!(rel_ok(spd.radius, 4.0), "sphere r");
        assert!(rel_ok(spd.basis.origin.x, 1.0) && rel_ok(spd.basis.origin.y, 2.0)
            && rel_ok(spd.basis.origin.z, 3.0), "sphere center {:?}", spd.basis.origin);

        assert!(rel_ok(Surf::cylinder(zbasis(Vec3::zero()), 5.0)?.ask_cylinder()?.radius, 5.0), "cyl r");

        let cod = Surf::cone(zbasis(Vec3::zero()), 3.0, 0.5)?.ask_cone()?;
        assert!(rel_ok(cod.radius, 3.0) && rel_ok(cod.semi_angle, 0.5), "cone {:?}", (cod.radius, cod.semi_angle));

        let td = Surf::torus(zbasis(Vec3::zero()), 10.0, 3.0)?.ask_torus()?;
        assert!(rel_ok(td.major_radius, 10.0) && rel_ok(td.minor_radius, 3.0), "torus radii");

        let lnd = Curve::line(Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))?.ask_line()?;
        assert!(rel_ok(lnd.origin.x, 1.0) && rel_ok(lnd.direction.y, 1.0), "line {:?}", (lnd.origin, lnd.direction));

        assert!(rel_ok(Curve::circle(zbasis(Vec3::zero()), 7.0)?.ask_circle()?.radius, 7.0), "circle r");

        let eld = Curve::ellipse(zbasis(Vec3::zero()), 6.0, 4.0)?.ask_ellipse()?;
        assert!(rel_ok(eld.r1, 6.0) && rel_ok(eld.r2, 4.0), "ellipse radii");

        let pp = Point::create(Vec3::new(9.0, 8.0, 7.0))?.position()?;
        assert!(rel_ok(pp.x, 9.0) && rel_ok(pp.y, 8.0) && rel_ok(pp.z, 7.0), "point {:?}", pp);
    });

    // =========================================================================
    // P4 — SSI on orphan analytic surfaces (the pair matrix)
    // =========================================================================

    test!("ssi_orphan_sphere_sphere", {
        let _session = Session::start(test_config())?;
        let zb = |o: Vec3| Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        // Two r=5 spheres, centres 6 apart → circle of radius sqrt(25-9)=4 at x=3.
        let s1 = Surf::sphere(zb(Vec3::zero()), 5.0)?;
        let s2 = Surf::sphere(zb(Vec3::new(6.0, 0.0, 0.0)), 5.0)?;
        let r = s1.intersect(&s2)?;
        assert_eq!(r.curves.len(), 1, "sphere-sphere = one circle");
        let cd = r.curves[0].curve.ask_circle()?;
        assert!(rel_ok(cd.radius, 4.0), "sphere-sphere circle radius {} != 4", cd.radius);
        assert!(rel_ok(cd.basis.origin.x, 3.0), "circle plane at x=3, got {}", cd.basis.origin.x);
    });

    test!("ssi_orphan_plane_sphere", {
        let _session = Session::start(test_config())?;
        let zb = |o: Vec3| Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        // Plane z=3 ∩ sphere r=5 at origin → circle radius 4.
        let plane = Surf::plane(zb(Vec3::new(0.0, 0.0, 3.0)))?;
        let sph = Surf::sphere(zb(Vec3::zero()), 5.0)?;
        let r = plane.intersect(&sph)?;
        assert_eq!(r.curves.len(), 1, "plane-sphere = one circle");
        assert!(rel_ok(r.curves[0].curve.ask_circle()?.radius, 4.0), "plane-sphere circle radius");
    });

    test!("ssi_orphan_cyl_cyl", {
        let _session = Session::start(test_config())?;
        // Two equal-radius cylinders with perpendicular axes intersect in the
        // classic Steinmetz curves (4 basis-curve segments).
        let ca = Surf::cylinder(Axis2::new(Vec3::zero(), Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)), 3.0)?;
        let cb = Surf::cylinder(Axis2::new(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)), 3.0)?;
        let r = ca.intersect(&cb)?;
        assert!(r.curves.len() >= 1, "perpendicular equal cylinders should intersect, got {}", r.curves.len());
    });

    test!("ssi_pair_matrix", {
        let _session = Session::start(test_config())?;
        let zb = |o: Vec3| Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let yplane = || Surf::plane(Axis2::new(Vec3::zero(), Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)));

        // plane through cylinder axis → 2 lines (transversal).
        let r = yplane()?.intersect(&Surf::cylinder(zb(Vec3::zero()), 3.0)?)?;
        assert_eq!(r.curves.iter().filter(|c| c.curve.curve_type().unwrap() == CurveType::Line).count(), 2, "plane∩cyl = 2 lines");

        // sphere(5) ∩ coaxial cylinder(3) → 2 circles.
        let r = Surf::sphere(zb(Vec3::zero()), 5.0)?.intersect(&Surf::cylinder(zb(Vec3::zero()), 3.0)?)?;
        assert_eq!(r.curves.len(), 2, "sphere∩cyl = 2 circles, got {}", r.curves.len());
        assert!(r.curves.iter().all(|c| c.curve.curve_type().unwrap() == CurveType::Circle));

        // equatorial plane ∩ torus(10,3) → 2 circles (inner r=7, outer r=13).
        let r = Surf::plane(zb(Vec3::zero()))?.intersect(&Surf::torus(zb(Vec3::zero()), 10.0, 3.0)?)?;
        let mut radii: Vec<f64> = r.curves.iter().map(|c| c.curve.ask_circle().unwrap().radius).collect();
        radii.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(radii.len(), 2, "plane∩torus = 2 circles");
        assert!(rel_ok(radii[0], 7.0) && rel_ok(radii[1], 13.0), "torus section radii {:?}", radii);

        // plane through a pointed cone's apex → 2 lines.
        let r = yplane()?.intersect(&Surf::cone(zb(Vec3::zero()), 0.0, 0.5)?)?;
        assert_eq!(r.curves.iter().filter(|c| c.curve.curve_type().unwrap() == CurveType::Line).count(), 2, "plane∩cone thru apex = 2 lines");
    });

    test!("ssi_tangency_coincidence_disjoint", {
        let _session = Session::start(test_config())?;
        let zb = |o: Vec3| Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let s1 = Surf::sphere(zb(Vec3::zero()), 5.0)?;

        // Externally tangent spheres (centres 2r apart) → a single tangent point.
        let tangent = s1.intersect(&Surf::sphere(zb(Vec3::new(10.0, 0.0, 0.0)), 5.0)?)?;
        assert_eq!(tangent.points.len(), 1, "tangent spheres = 1 point");
        assert_eq!(tangent.curves.len(), 0);
        assert!(rel_ok(tangent.points[0].x, 5.0), "tangent point at (5,0,0)");

        // Disjoint spheres → nothing.
        let disjoint = s1.intersect(&Surf::sphere(zb(Vec3::new(20.0, 0.0, 0.0)), 5.0)?)?;
        assert!(disjoint.points.is_empty() && disjoint.curves.is_empty(), "disjoint spheres = empty");

        // Coincident planes → no intersection data (documented).
        let a = Surf::plane(zb(Vec3::new(0.0, 0.0, 2.0)))?;
        let b = Surf::plane(zb(Vec3::new(0.0, 0.0, 2.0)))?;
        let coincident = a.intersect(&b)?;
        assert!(coincident.points.is_empty() && coincident.curves.is_empty(), "coincident planes = empty");

        // Plane tangent to a cylinder → a tangential line (kind classified).
        let cyl = Surf::cylinder(zb(Vec3::zero()), 3.0)?;
        let ptan = Surf::plane(Axis2::new(Vec3::new(3.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)))?;
        let tan = ptan.intersect(&cyl)?;
        assert_eq!(tan.curves.len(), 1, "tangent plane-cyl = 1 line");
        assert_eq!(tan.curves[0].classify(), IntersectionKind::Tangential, "should be tangential");
        // And a transversal case classifies the other way.
        let thru = Surf::plane(Axis2::new(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)))?.intersect(&cyl)?;
        assert!(thru.curves.iter().all(|c| c.classify() == IntersectionKind::Transversal), "through-axis = transversal");
    });

    // =========================================================================
    // P2 — surface normal + analytic param round-trips
    // =========================================================================

    test!("surface_normal_sphere_outward", {
        let _session = Session::start(test_config())?;
        let r = 15.0;
        let body = Body::create_solid_sphere(r)?;
        let surf = body.faces()?[0].surf()?;
        for (u, v) in [(0.0, 0.0), (1.0, 0.5), (2.0, -0.7)] {
            let (p, n) = surf.eval_with_normal(u, v)?;
            let plen = (p.x * p.x + p.y * p.y + p.z * p.z).sqrt();
            let nlen = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
            let dot = (p.x * n.x + p.y * n.y + p.z * n.z) / plen; // n · outward radial
            assert!((plen - r).abs() < 1e-6, "point off sphere: |p|={plen}");
            assert!((nlen - 1.0).abs() < 1e-9, "normal not unit: {nlen}");
            assert!((dot - 1.0).abs() < 1e-6, "sphere surface normal not outward radial: {dot}");
        }
    });

    test!("surface_uvbox_seams_poles", {
        let _session = Session::start(test_config())?;
        let zb = |o: Vec3| Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let tau = std::f64::consts::TAU;
        let pi = std::f64::consts::PI;

        // Cylinder: u periodic [0, 2π] (angular seam), v unbounded.
        let cyl = Surf::cylinder(zb(Vec3::zero()), 5.0)?.uvbox()?;
        assert!(rel_ok(cyl.u_min, 0.0) && rel_ok(cyl.u_max, tau), "cyl u ∈ [0,2π]: {:?}", cyl);
        assert!(cyl.v_max - cyl.v_min > 1e3, "cyl v should be unbounded: {:?}", cyl);

        // Sphere: u periodic [0, 2π]; v [-π/2, π/2] with poles at the ends.
        let sph = Surf::sphere(zb(Vec3::zero()), 5.0)?.uvbox()?;
        assert!(rel_ok(sph.u_min, 0.0) && rel_ok(sph.u_max, tau), "sphere u seam");
        assert!(rel_ok(sph.v_min, -pi / 2.0) && rel_ok(sph.v_max, pi / 2.0), "sphere v poles: {:?}", sph);

        // Torus: u periodic [0, 2π], v periodic [-π, π].
        let tor = Surf::torus(zb(Vec3::zero()), 10.0, 3.0)?.uvbox()?;
        assert!(rel_ok(tor.u_min, 0.0) && rel_ok(tor.u_max, tau), "torus u");
        assert!(rel_ok(tor.v_min, -pi) && rel_ok(tor.v_max, pi), "torus v: {:?}", tor);
    });

    test!("surface_parameterise_roundtrip", {
        let _session = Session::start(test_config())?;
        // Evaluate a sphere at known (u,v), invert, and confirm eval(uv') == p.
        let surf = Body::create_solid_sphere(15.0)?.faces()?[0].surf()?;
        for (u, v) in [(0.4, 0.3), (2.1, -0.6)] {
            let p = surf.eval(u, v)?;
            let (u2, v2) = surf.parameterise(p)?;
            let p2 = surf.eval(u2, v2)?;
            let d = ((p2.x - p.x).powi(2) + (p2.y - p.y).powi(2) + (p2.z - p.z).powi(2)).sqrt();
            assert!(d < 1e-6, "surf parameterise round-trip off by {d}");
        }
    });

    test!("curve_parameterise_roundtrip", {
        let _session = Session::start(test_config())?;
        // A cylinder's circular edge: eval at t, invert, eval again.
        let body = Body::create_solid_cylinder(5.0, 12.0)?;
        let curve = body.edges()?.iter()
            .map(|e| e.curve().unwrap())
            .find(|c| c.curve_type().unwrap() == CurveType::Circle)
            .expect("cylinder circular edge")
            .clone();
        for t in [0.5f64, 2.0, 4.0] {
            let p = curve.eval(t)?;
            let t2 = curve.parameterise(p)?;
            let p2 = curve.eval(t2)?;
            let d = ((p2.x - p.x).powi(2) + (p2.y - p.y).powi(2) + (p2.z - p.z).powi(2)).sqrt();
            assert!(d < 1e-6, "curve parameterise round-trip off by {d}");
        }
    });

    test!("circle_extraction_cylinder", {
        let _session = Session::start(test_config())?;
        let (r, h) = (5.0, 12.0);
        let body = Body::create_solid_cylinder(r, h)?;
        let circles: Vec<_> = body.edges()?.iter()
            .map(|e| e.curve().unwrap())
            .filter(|c| c.curve_type().unwrap() == CurveType::Circle)
            .map(|c| c.ask_circle().unwrap())
            .collect();
        assert_eq!(circles.len(), 2, "cylinder has 2 circular edges, got {}", circles.len());
        for cd in &circles {
            assert!(rel_ok(cd.radius, r), "circle radius {} != {}", cd.radius, r);
            assert!(near0(cd.basis.origin.x, r) && near0(cd.basis.origin.y, r),
                "circle centre off Z axis: {:?}", cd.basis.origin);
        }
        // Centres at the two cap planes z=0 and z=h.
        let mut zs: Vec<f64> = circles.iter().map(|c| c.basis.origin.z).collect();
        zs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(near0(zs[0], h) && rel_ok(zs[1], h), "circle z centres {:?}", zs);
    });

    test!("line_extraction_and_tangent", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let edge = body.edges()?[0];
        let curve = edge.curve()?;
        assert_eq!(curve.curve_type()?, CurveType::Line);
        let ld = curve.ask_line()?;
        // Direction is a unit vector.
        let dlen = (ld.direction.x.powi(2) + ld.direction.y.powi(2) + ld.direction.z.powi(2)).sqrt();
        assert!((dlen - 1.0).abs() < 1e-9, "line direction not unit: {dlen}");
        // eval endpoints span the edge; tangent is unit and along the chord.
        let (t0, t1) = edge.interval()?;
        let (p0, tan) = curve.eval_with_tangent(t0)?;
        let p1 = curve.eval(t1)?;
        let chord = ((p1.x - p0.x).powi(2) + (p1.y - p0.y).powi(2) + (p1.z - p0.z).powi(2)).sqrt();
        assert!((t1 - t0 - chord).abs() < 1e-6, "arc-length param: interval {} != chord {}", t1 - t0, chord);
        let tlen = (tan.x * tan.x + tan.y * tan.y + tan.z * tan.z).sqrt();
        assert!((tlen - 1.0).abs() < 1e-9, "tangent not unit: {tlen}");
    });

    test!("cone_params_roundtrip", {
        let _session = Session::start(test_config())?;
        // radius (at base/basis origin) 5, height 3, semi-angle 45°.
        let body = Body::create_solid_cone(5.0, 3.0, std::f64::consts::FRAC_PI_4)?;
        let cone = body.faces()?.iter()
            .map(|f| f.surf().unwrap())
            .find(|s| s.surf_type().unwrap() == SurfType::Cone)
            .expect("cone should have a conical face")
            .ask_cone()?;
        assert!(rel_ok(cone.radius, 5.0), "cone sf radius {} != 5 (radius is at basis origin)", cone.radius);
        assert!(rel_ok(cone.semi_angle, std::f64::consts::FRAC_PI_4), "cone semi_angle {}", cone.semi_angle);
    });

    // =========================================================================
    // P4 — surface/surface intersection (SSI oracle)
    // =========================================================================

    test!("ssi_cylinder_plane_circle", {
        let _session = Session::start(test_config())?;
        let r = 5.0;
        let cyl = Body::create_solid_cylinder(r, 12.0)?;
        let side = cyl.faces()?.iter().map(|f| f.surf().unwrap())
            .find(|s| s.surf_type().unwrap() == SurfType::Cylinder).expect("side");
        let plane = cyl.faces()?.iter().map(|f| f.surf().unwrap())
            .find(|s| s.surf_type().unwrap() == SurfType::Plane).expect("cap plane");
        let isect = side.intersect(&plane)?;
        assert_eq!(isect.points.len(), 0, "cyl∩plane point count");
        assert_eq!(isect.curves.len(), 1, "cyl∩plane should be one circle");
        let ic = &isect.curves[0];
        assert_eq!(ic.curve.curve_type()?, CurveType::Circle, "intersection is a circle");
        assert!(rel_ok(ic.curve.ask_circle()?.radius, r), "intersection circle radius");
    });

    test!("ssi_plane_plane_line", {
        let _session = Session::start(test_config())?;
        let blk = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let planes: Vec<_> = blk.faces()?.iter().map(|f| f.surf().unwrap()).collect();
        // Find a non-parallel pair (adjacent faces) whose planes meet in a line.
        let mut found_line = false;
        'outer: for i in 0..planes.len() {
            for j in (i + 1)..planes.len() {
                let isect = planes[i].intersect(&planes[j])?;
                if let Some(ic) = isect.curves.first() {
                    if ic.curve.curve_type()? == CurveType::Line {
                        found_line = true;
                        break 'outer;
                    }
                }
            }
        }
        assert!(found_line, "two adjacent block face planes should intersect in a line");
    });

    test!("ssi_face_face_line", {
        let _session = Session::start(test_config())?;
        let blk = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let faces = blk.faces()?;
        let mut found = false;
        'o: for i in 0..faces.len() {
            for j in (i + 1)..faces.len() {
                let r = faces[i].intersect_face(&faces[j])?;
                if let Some(c) = r.curves.first() {
                    if c.curve.curve_type()? == CurveType::Line {
                        found = true;
                        break 'o;
                    }
                }
            }
        }
        assert!(found, "two adjacent block faces should intersect in a line");
    });

    test!("ssi_face_surf_circle", {
        let _session = Session::start(test_config())?;
        let r = 5.0;
        let cyl = Body::create_solid_cylinder(r, 12.0)?;
        let side = cyl.faces()?.into_iter()
            .find(|f| f.surf().unwrap().surf_type().unwrap() == SurfType::Cylinder).unwrap();
        let cap = cyl.faces()?.iter().map(|f| f.surf().unwrap())
            .find(|s| s.surf_type().unwrap() == SurfType::Plane).unwrap();
        let isect = side.intersect_surf(&cap)?;
        assert_eq!(isect.curves.len(), 1, "cyl face ∩ cap surf = one curve");
        assert_eq!(isect.curves[0].curve.curve_type()?, CurveType::Circle);
        assert!(rel_ok(isect.curves[0].curve.ask_circle()?.radius, r), "circle radius");
    });

    test!("ssi_curve_curve_vertex", {
        let _session = Session::start(test_config())?;
        let blk = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let edges = blk.edges()?;
        let e0 = edges[0];
        let (l0, h0) = e0.interval()?;
        let c0 = e0.curve()?;
        // Vertices of e0 — an intersection with an adjacent edge is at one of them.
        let (v0, v1) = e0.vertices()?;
        let (p0, p1) = (v0.point()?, v1.point()?);
        let mut found = false;
        for k in 1..edges.len() {
            let ek = edges[k];
            let (lk, hk) = ek.interval()?;
            let hits = c0.intersect_curve((l0, h0), &ek.curve()?, (lk, hk))?;
            if let Some(h) = hits.first() {
                let at_v0 = ((h.position.x - p0.x).powi(2) + (h.position.y - p0.y).powi(2) + (h.position.z - p0.z).powi(2)).sqrt() < 1e-6;
                let at_v1 = ((h.position.x - p1.x).powi(2) + (h.position.y - p1.y).powi(2) + (h.position.z - p1.z).powi(2)).sqrt() < 1e-6;
                assert!(at_v0 || at_v1, "curve-curve hit should be at a shared vertex, got {:?}", h.position);
                found = true;
                break;
            }
        }
        assert!(found, "e0 should meet some adjacent edge");
    });

    test!("ssi_surf_and_face_intersect_curve", {
        let _session = Session::start(test_config())?;
        let blk = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let faces = blk.faces()?;
        let edges = blk.edges()?;
        // A vertical edge (endpoints differ in z) and a horizontal (z-normal) face.
        let vedge = edges.iter().find(|e| {
            let (a, b) = e.vertices().unwrap();
            (a.point().unwrap().z - b.point().unwrap().z).abs() > 1.0
        }).unwrap();
        let (vl, vh) = vedge.interval()?;
        let vc = vedge.curve()?;
        let hface = faces.iter().find(|f| {
            let s = f.surf().unwrap();
            s.surf_type().unwrap() == SurfType::Plane && s.ask_plane().unwrap().basis.axis.z.abs() > 0.9
        }).unwrap();
        let hsurf = hface.surf()?;
        // Widen so the crossing is interior to the interval.
        let span = vh - vl;
        let sh = hsurf.intersect_curve(&vc, (vl - span, vh + span))?;
        assert_eq!(sh.len(), 1, "vertical line crosses horizontal plane once, got {}", sh.len());
        let fh = hface.intersect_curve(&vc, (vl - span, vh + span))?;
        assert_eq!(fh.len(), 1, "vertical line crosses horizontal face once, got {}", fh.len());
        // Same crossing point from both.
        let d = ((sh[0].position.x - fh[0].position.x).powi(2)
            + (sh[0].position.y - fh[0].position.y).powi(2)
            + (sh[0].position.z - fh[0].position.z).powi(2)).sqrt();
        assert!(d < 1e-6, "surf/face curve-intersection points disagree by {d}");
    });

    // =========================================================================
    // P3 — B-rep spine adjacency (Region/Shell/Loop/Fin) on a solid block
    // =========================================================================

    test!("brep_spine_block", {
        let _session = Session::start(test_config())?;
        let body = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let faces = body.faces()?;
        let edges = body.edges()?;
        assert_eq!(faces.len(), 6, "block faces");
        assert_eq!(edges.len(), 12, "block edges");

        // Regions: exactly one solid, plus the surrounding void → 2 total.
        let regions = body.regions()?;
        assert_eq!(regions.len(), 2, "block regions (solid + void), got {}", regions.len());
        let n_solid = regions.iter().filter(|r| r.is_solid().unwrap()).count();
        assert_eq!(n_solid, 1, "exactly one solid region, got {}", n_solid);

        // Every shell round-trips to a region of this body.
        let region_tags: std::collections::HashSet<i32> = regions.iter().map(|r| r.tag()).collect();
        let shells = body.shells()?;
        assert!(!shells.is_empty(), "block should have >=1 shell");
        for sh in &shells {
            assert!(region_tags.contains(&sh.region()?.tag()), "shell.region not in body");
        }
        // The solid region's shells cover all 6 faces.
        let solid = regions.iter().find(|r| r.is_solid().unwrap()).unwrap();
        let mut solid_faces = std::collections::HashSet::new();
        for sh in solid.shells()? {
            for f in sh.faces()? {
                solid_faces.insert(f.tag());
            }
        }
        assert_eq!(solid_faces.len(), 6, "solid region should touch all 6 faces");

        // Each face: exactly one outer loop of 4 fins forming a cycle.
        let mut total_fins = 0;
        for f in &faces {
            let loops = f.loops()?;
            assert_eq!(loops.len(), 1, "block face has 1 loop, got {}", loops.len());
            let lp = loops[0];
            assert_eq!(lp.face()?.tag(), f.tag(), "loop.face round-trip");
            assert_eq!(lp.loop_type()?, LoopType::Outer, "block face loop should be outer");
            let fins = lp.fins()?;
            assert_eq!(fins.len(), 4, "rectangular face loop has 4 fins, got {}", fins.len());
            total_fins += fins.len();
            // Fins cycle back to the start after 4 next_in_loop steps.
            let mut cur = fins[0];
            for _ in 0..4 {
                assert_eq!(cur.face()?.tag(), f.tag(), "fin.face round-trip");
                cur = cur.next_in_loop()?;
            }
            assert_eq!(cur.tag(), fins[0].tag(), "loop should be a 4-cycle");
        }
        assert_eq!(total_fins, 24, "6 faces * 4 fins = 24");

        // Each of the 12 edges is used by exactly 2 fins (manifold).
        for e in &edges {
            assert_eq!(e.fins()?.len(), 2, "manifold edge has 2 fins");
        }
    });

    // =========================================================================
    // P5 — point containment (inside / outside / on)
    // =========================================================================

    test!("contains_point_block", {
        let _session = Session::start(test_config())?;
        // Block base at origin: x∈±5, y∈±10, z∈0..30.
        let body = Body::create_solid_block(10.0, 20.0, 30.0)?;
        assert_eq!(body.contains_point(Vec3::new(0.0, 0.0, 15.0))?, Enclosure::Inside);
        assert_eq!(body.contains_point(Vec3::new(100.0, 0.0, 0.0))?, Enclosure::Outside);
        assert_eq!(body.contains_point(Vec3::new(0.0, 0.0, -1.0))?, Enclosure::Outside);
        // A point on the +x face (x=5) is on the boundary.
        assert_eq!(body.contains_point(Vec3::new(5.0, 0.0, 15.0))?, Enclosure::On);
    });

    test!("contains_point_sphere", {
        let _session = Session::start(test_config())?;
        let r = 15.0;
        let body = Body::create_solid_sphere(r)?;
        assert_eq!(body.contains_point(Vec3::zero())?, Enclosure::Inside);
        assert_eq!(body.contains_point(Vec3::new(r * 0.9, 0.0, 0.0))?, Enclosure::Inside);
        assert_eq!(body.contains_point(Vec3::new(r + 1.0, 0.0, 0.0))?, Enclosure::Outside);
        assert_eq!(body.contains_point(Vec3::new(r, 0.0, 0.0))?, Enclosure::On);
    });

    // =========================================================================
    // P5 — bounding-box oracle
    // =========================================================================

    test!("bbox_block", {
        let _session = Session::start(test_config())?;
        // Block base centred at origin: x in ±5, y in ±10, z in 0..30.
        let body = Body::create_solid_block(10.0, 20.0, 30.0)?;
        let bb = body.bounding_box()?;
        assert!(rel_ok(bb.min.x, -5.0) && rel_ok(bb.max.x, 5.0), "bbox x {:?}", bb);
        assert!(rel_ok(bb.min.y, -10.0) && rel_ok(bb.max.y, 10.0), "bbox y {:?}", bb);
        assert!(near0(bb.min.z, 30.0) && rel_ok(bb.max.z, 30.0), "bbox z {:?}", bb);
    });

    test!("bbox_sphere", {
        let _session = Session::start(test_config())?;
        let r = 15.0;
        let body = Body::create_solid_sphere(r)?;
        let bb = body.bounding_box()?;
        let sz = bb.size();
        // Guaranteed-containing box: at least the true diameter, not wildly more.
        for (got, axis) in [(sz.x, "x"), (sz.y, "y"), (sz.z, "z")] {
            assert!(got >= 2.0 * r - 1e-6 && got <= 2.0 * r * 1.01,
                "sphere bbox {axis} extent {got} not ~{}", 2.0 * r);
        }
        let c = bb.center();
        assert!(near0(c.x, r) && near0(c.y, r) && near0(c.z, r), "sphere bbox center {:?}", c);
    });

    // =========================================================================
    // Summary
    // =========================================================================

    println!("\n=== Results: {} passed, {} failed ===", passed, failed);
    if failed > 0 {
        std::process::exit(1);
    }
}
