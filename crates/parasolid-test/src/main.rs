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
