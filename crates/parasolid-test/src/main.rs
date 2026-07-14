//! Integration tests for parasolid-sys and parasolid crates.
//!
//! Build: cargo build -p parasolid-test --target x86_64-pc-windows-gnu
//! Run:   WINEPATH=/path/to/SOLIDWORKS cargo run -p parasolid-test --target x86_64-pc-windows-gnu

use parasolid::*;

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
        let session = Session::start(SessionConfig::new())?;
        let (major, minor, _patch) = session.kernel_version()?;
        assert!(major >= 30, "kernel version too old: {}.{}", major, minor);
        println!("(v{}.{}) ", major, minor);
        drop(session);
    });

    // =========================================================================
    // Body creation
    // =========================================================================

    test!("create_solid_block", {
        let _session = Session::start(SessionConfig::new())?;
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
        let _session = Session::start(SessionConfig::new())?;
        let body = Body::create_solid_cylinder(5.0, 20.0)?;
        assert_eq!(body.body_type()?, BodyType::Solid);
        let faces = body.faces()?;
        assert_eq!(faces.len(), 3, "cylinder should have 3 faces, got {}", faces.len());
    });

    test!("create_solid_sphere", {
        let _session = Session::start(SessionConfig::new())?;
        let body = Body::create_solid_sphere(10.0)?;
        assert_eq!(body.body_type()?, BodyType::Solid);
        let faces = body.faces()?;
        assert_eq!(faces.len(), 1, "sphere should have 1 face, got {}", faces.len());
    });

    // =========================================================================
    // Topology navigation
    // =========================================================================

    test!("face_edges_vertices", {
        let _session = Session::start(SessionConfig::new())?;
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
        let _session = Session::start(SessionConfig::new())?;
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
        let _session = Session::start(SessionConfig::new())?;
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
        let _session = Session::start(SessionConfig::new())?;
        let body = Body::create_solid_block(10.0, 10.0, 10.0)?;
        for face in body.faces()? {
            let surf = face.surf()?;
            assert_eq!(surf.surf_type()?, SurfType::Plane);
        }
    });

    test!("cylinder_face_surface_types", {
        let _session = Session::start(SessionConfig::new())?;
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
        let _session = Session::start(SessionConfig::new())?;
        let body = Body::create_solid_sphere(25.0)?;
        let face = &body.faces()?[0];
        let surf = face.surf()?;
        let data = surf.ask_sphere()?;
        assert!((data.radius - 25.0).abs() < 1e-10, "radius should be 25, got {}", data.radius);
    });

    test!("surface_eval", {
        let _session = Session::start(SessionConfig::new())?;
        let body = Body::create_solid_sphere(10.0)?;
        let face = &body.faces()?[0];
        let surf = face.surf()?;
        // Evaluate at some parameter
        let pos = surf.eval(0.5, 0.5)?;
        let dist = (pos.x*pos.x + pos.y*pos.y + pos.z*pos.z).sqrt();
        assert!((dist - 10.0).abs() < 1e-6, "point should be on sphere (r=10), dist={}", dist);
    });

    test!("edge_curve_type", {
        let _session = Session::start(SessionConfig::new())?;
        let body = Body::create_solid_block(10.0, 10.0, 10.0)?;
        for edge in body.edges()? {
            let curve = edge.curve()?;
            assert_eq!(curve.curve_type()?, CurveType::Line);
        }
    });

    test!("curve_eval", {
        let _session = Session::start(SessionConfig::new())?;
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
        let _session = Session::start(SessionConfig::new())?;
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
    // Summary
    // =========================================================================

    println!("\n=== Results: {} passed, {} failed ===", passed, failed);
    if failed > 0 {
        std::process::exit(1);
    }
}
