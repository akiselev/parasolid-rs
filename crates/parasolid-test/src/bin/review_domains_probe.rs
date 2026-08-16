//! Adversarial Stage 4 review probe. Read-only: creates nothing in the repo.

use parasolid::*;
use parasolid_sys::*;

fn basis() -> Axis2 {
    Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    )
}

fn d(a: Vec3, b: Vec3) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

// ---------------------------------------------------------------- claim 1
fn stride_sentinel() {
    println!("== CLAIM 1: PK_SURF_ask_params stride/extent of writes ==");
    let s = Surf::sphere(basis(), 4.0).unwrap();
    // 160 bytes of scratch, sentinel filled.
    let mut buf = vec![0xEEu8; 160];
    let rc = unsafe { PK_SURF_ask_params(s.tag(), buf.as_mut_ptr() as *mut PK_PARAM_sf_t) };
    println!("  rc={rc}");
    let mut first_clean = None;
    for i in 0..160 {
        if buf[i] == 0xEE && first_clean.is_none() {
            // keep scanning; we want the LAST written byte
        }
        let _ = i;
    }
    let last_written = (0..160).rev().find(|&i| buf[i] != 0xEE);
    println!("  last byte modified: {last_written:?}  (40-byte x2 => expect 71 or less)");
    // per-byte map
    let map: String = (0..160)
        .map(|i| if buf[i] == 0xEE { '.' } else { 'X' })
        .collect();
    for row in 0..5 {
        println!("  [{:3}] {}", row * 32, &map[row * 32..(row + 1) * 32]);
    }
    for i in 0..160 {
        if buf[i] == 0xEE {
            continue;
        }
    }
    // decode as two records at stride 40
    let recs = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const PK_PARAM_sf_t, 2) };
    for (k, r) in recs.iter().enumerate() {
        println!(
            "  rec{k}: range=[{:.6},{:.6}] extent={} form={} periodic={} curve_class={} closed={:#x}",
            r.range.low, r.range.high, r.extent, r.form, r.periodic, r.curve_class, r.closed
        );
    }
    // also check what a stride-of-36 or 44 reading would give (sanity)
    let ints = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const i32, 40) };
    print!("  raw i32[0..20]:");
    for i in 0..20 {
        print!(" {}:{}", i, ints[i]);
    }
    println!();
    println!("  first_clean_unused={first_clean:?}");
    first_clean = Some(0);
    let _ = first_clean;
}

// ---------------------------------------------------------------- claim 2/3
fn dump_surf(label: &str, s: &Surf) {
    dump_surf_tag(label, s.tag());
}

fn dump_surf_tag(label: &str, tag: i32) {
    let s = SurfTag(tag);
    let mut sf: [PK_PARAM_sf_t; 2] = unsafe { std::mem::zeroed() };
    let rc = unsafe { PK_SURF_ask_params(s.tag(), sf.as_mut_ptr()) };
    if rc != PK_ERROR_no_errors {
        println!("  {label:26} ask_params err {rc}");
        return;
    }
    for (dir, p) in [("u", sf[0]), ("v", sf[1])] {
        println!(
            "  {label:26} {dir}: range=[{:9.4},{:9.4}] extent={} form={} periodic={} class={} closed={}",
            p.range.low,
            p.range.high,
            p.extent,
            p.form,
            p.periodic,
            p.curve_class,
            p.closed & 0xff
        );
    }
}

fn dump_curve(label: &str, c: &Curve) {
    dump_curve_tag(label, c.tag());
}

struct SurfTag(i32);
impl SurfTag {
    fn tag(&self) -> i32 {
        self.0
    }
}

fn dump_curve_tag(label: &str, tag: i32) {
    let c = SurfTag(tag);
    let mut sf = unsafe { std::mem::zeroed::<PK_PARAM_sf_t>() };
    let rc = unsafe { PK_CURVE_ask_param(c.tag(), &mut sf) };
    if rc != PK_ERROR_no_errors {
        println!("  {label:26} ask_param err {rc}");
        return;
    }
    println!(
        "  {label:26}  : range=[{:9.4},{:9.4}] extent={} form={} periodic={} class={} closed={}",
        sf.range.low,
        sf.range.high,
        sf.extent,
        sf.form,
        sf.periodic,
        sf.curve_class,
        sf.closed & 0xff
    );
}

fn open_bcurve() -> Curve {
    let cps = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 0.0),
        Vec3::new(3.0, -1.0, 1.0),
        Vec3::new(5.0, 1.0, 0.0),
    ];
    Curve::bcurve(3, &cps, &[0.0, 1.0], &[4, 4]).unwrap()
}

fn periodic_bcurve() -> Option<i32> {
    // raw create with is_periodic = 1: uniform knots, wrapped control polygon
    let cps: Vec<Vec3> = (0..6)
        .map(|i| {
            let a = (i as f64) * std::f64::consts::TAU / 6.0;
            Vec3::new(3.0 * a.cos(), 3.0 * a.sin(), 0.0)
        })
        .collect();
    let verts: Vec<f64> = cps.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
    let knots: Vec<f64> = (0..7).map(|i| i as f64).collect();
    let mults: Vec<i32> = vec![1; 7];
    let sf = PK_BCURVE_sf_t {
        degree: 3,
        n_vertices: 6,
        vertex_dim: 3,
        is_rational: PK_LOGICAL_false,
        vertices: verts.as_ptr(),
        _reserved_24: 0,
        n_knots: knots.len() as std::os::raw::c_int,
        knot_mult: mults.as_ptr(),
        knots: knots.as_ptr(),
        knot_type: PK_knot_non_uniform_c,
        is_periodic: 1,
        is_closed: 1,
        _pad: [0; 2],
        self_intersecting: 0,
    };
    let mut tag: PK_BCURVE_t = PK_ENTITY_null;
    let rc = unsafe { PK_BCURVE_create(&sf, &mut tag) };
    if rc != PK_ERROR_no_errors {
        println!("  (periodic bcurve create failed: {rc})");
        return None;
    }
    Some(tag)
}

fn open_bsurf() -> Surf {
    let mut cps = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            cps.push(Vec3::new(i as f64, j as f64, (i * j) as f64 * 0.1));
        }
    }
    Surf::bsurf(3, 3, 4, 4, &cps, &[0.0, 1.0], &[4, 4], &[0.0, 1.0], &[4, 4]).unwrap()
}

fn periodic_bsurf() -> Option<i32> {
    // periodic in u (a tube), open in v
    let nu = 6;
    let nv = 4;
    let mut cps = Vec::new();
    for i in 0..nu {
        let a = (i as f64) * std::f64::consts::TAU / nu as f64;
        for j in 0..nv {
            cps.push(Vec3::new(3.0 * a.cos(), 3.0 * a.sin(), j as f64));
        }
    }
    let verts: Vec<f64> = cps.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
    let uk: Vec<f64> = (0..nu + 1).map(|i| i as f64).collect();
    let um: Vec<i32> = vec![1; nu + 1];
    let vk: Vec<f64> = vec![0.0, 1.0];
    let vm: Vec<i32> = vec![4, 4];
    let sf = PK_BSURF_sf_t {
        u_degree: 3,
        v_degree: 3,
        n_u_vertices: nu as i32,
        n_v_vertices: nv as i32,
        vertex_dim: 3,
        is_rational: PK_LOGICAL_false,
        vertices: verts.as_ptr(),
        _reserved_32: 0,
        n_u_knots: uk.len() as std::os::raw::c_int,
        n_v_knots: vk.len() as std::os::raw::c_int,
        u_knot_mult: um.as_ptr(),
        v_knot_mult: vm.as_ptr(),
        u_knots: uk.as_ptr(),
        v_knots: vk.as_ptr(),
        u_knot_type: PK_knot_non_uniform_c,
        v_knot_type: PK_knot_non_uniform_c,
        is_u_periodic: 1,
        is_v_periodic: 0,
        is_u_closed: 1,
        is_v_closed: 0,
        self_intersecting: 0,
        convexity: 0,
    };
    let mut tag: PK_BSURF_t = PK_ENTITY_null;
    let rc = unsafe { PK_BSURF_create(&sf, &mut tag) };
    if rc != PK_ERROR_no_errors {
        println!("  (periodic bsurf create failed: {rc})");
        return None;
    }
    Some(tag)
}

fn families() {
    println!("\n== CLAIM 2/3: extent/form/class across families (hunting 18001/18002/18043) ==");
    let b = basis();
    dump_curve(
        "line",
        &Curve::line(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.)).unwrap(),
    );
    dump_curve("circle", &Curve::circle(b, 3.0).unwrap());
    dump_curve("ellipse 5x2", &Curve::ellipse(b, 5.0, 2.0).unwrap());
    dump_curve("bcurve open deg3", &open_bcurve());
    if let Some(c) = periodic_bcurve() {
        dump_curve_tag("bcurve periodic", c);
    }

    dump_surf("plane", &Surf::plane(b).unwrap());
    dump_surf("cylinder r2", &Surf::cylinder(b, 2.0).unwrap());
    dump_surf("cone r3 a0.5", &Surf::cone(b, 3.0, 0.5).unwrap());
    dump_surf("sphere r4", &Surf::sphere(b, 4.0).unwrap());
    dump_surf("torus 5/1.5", &Surf::torus(b, 5.0, 1.5).unwrap());
    match Surf::torus(b, 1.0, 1.5) {
        Ok(s) => dump_surf("torus apple 1/1.5", &s),
        Err(e) => println!("  torus apple: err {e:?}"),
    }
    match Surf::torus(b, 1.5, 1.5) {
        Ok(s) => dump_surf("torus lemon 1.5/1.5", &s),
        Err(e) => println!("  torus lemon: err {e:?}"),
    }
    dump_surf("bsurf open", &open_bsurf());
    if let Some(s) = periodic_bsurf() {
        dump_surf_tag("bsurf u-periodic", s);
    }
    // swept / spun
    let line = Curve::line(Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)).unwrap();
    match Surf::spun(&line, Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.)) {
        Ok(s) => dump_surf("spun(line)", &s),
        Err(e) => println!("  spun(line): err {e:?}"),
    }
    let circ = Curve::circle(
        Axis2::new(
            Vec3::new(5.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        1.0,
    )
    .unwrap();
    match Surf::spun(&circ, Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.)) {
        Ok(s) => dump_surf("spun(circle)=torus", &s),
        Err(e) => println!("  spun(circle): err {e:?}"),
    }
    match Surf::spun(&open_bcurve(), Vec3::new(0., 0., 0.), Vec3::new(0., 0., 1.)) {
        Ok(s) => dump_surf("spun(bcurve)", &s),
        Err(e) => println!("  spun(bcurve): err {e:?}"),
    }
    match Surf::swept(&Curve::circle(b, 2.0).unwrap(), Vec3::new(0., 0., 5.)) {
        Ok(s) => dump_surf("swept(circle)", &s),
        Err(e) => println!("  swept(circle): err {e:?}"),
    }
    match Surf::swept(&open_bcurve(), Vec3::new(0., 0., 5.)) {
        Ok(s) => dump_surf("swept(bcurve)", &s),
        Err(e) => println!("  swept(bcurve): err {e:?}"),
    }
    match Surf::swept(
        &Curve::line(Vec3::new(0., 0., 0.), Vec3::new(1., 0., 0.)).unwrap(),
        Vec3::new(0., 1., 0.),
    ) {
        Ok(s) => dump_surf("swept(line)", &s),
        Err(e) => println!("  swept(line): err {e:?}"),
    }
    // offsets
    for (lbl, base) in [
        ("offset(plane)", Surf::plane(b).unwrap()),
        ("offset(cylinder)", Surf::cylinder(b, 2.0).unwrap()),
        ("offset(sphere)", Surf::sphere(b, 4.0).unwrap()),
        ("offset(torus)", Surf::torus(b, 5.0, 1.5).unwrap()),
        ("offset(bsurf)", open_bsurf()),
    ] {
        match Surf::offset_surface(&base, 0.25) {
            Ok(s) => dump_surf(lbl, &s),
            Err(e) => println!("  {lbl}: err {e:?}"),
        }
    }
    // blend surface from a fillet
    match blend_surface() {
        Some(s) => dump_surf("blend(fillet)", &s),
        None => println!("  blend(fillet): unavailable"),
    }
}

fn blend_surface() -> Option<Surf> {
    let body = Body::create_solid_block(10.0, 10.0, 10.0).ok()?;
    let edges = body.edges().ok()?;
    let e = edges.first()?.clone();
    body.fillet_edges(&[e], 1.0).ok()?;
    for f in body.faces().ok()? {
        if let Ok(t) = f.surface_type() {
            if !matches!(t, SurfType::Plane) {
                return f.surf().ok();
            }
        }
    }
    None
}

// ---------------------------------------------------------------- claim 4
fn seams() {
    println!("\n== CLAIM 4: seam identification, 2nd derivatives, near-pole ==");
    let b = basis();
    let tau = std::f64::consts::TAU;
    let cases: Vec<(&str, Surf, Vec<f64>)> = vec![
        (
            "cylinder",
            Surf::cylinder(b, 2.0).unwrap(),
            vec![-3.0, 0.0, 7.5],
        ),
        (
            "sphere",
            Surf::sphere(b, 4.0).unwrap(),
            vec![-1.5707, -1.5, 0.0, 1.5, 1.5707],
        ),
        (
            "torus",
            Surf::torus(b, 5.0, 1.5).unwrap(),
            vec![0.0, 1.0, 3.0],
        ),
        (
            "cone",
            Surf::cone(b, 3.0, 0.5).unwrap(),
            vec![-1.0, 0.0, 2.0],
        ),
    ];
    for (label, s, vs) in cases {
        for v in vs {
            for u0 in [0.0f64, 0.7] {
                let a = match s.eval_jet(u0, v, 3, 3, false) {
                    Ok(j) => j,
                    Err(e) => {
                        println!("  {label} v={v}: eval err {e:?}");
                        continue;
                    }
                };
                let c = s.eval_jet(u0 + tau, v, 3, 3, false).unwrap();
                let mut worst = 0.0f64;
                let mut worst_ij = (0, 0);
                for i in 0..=3 {
                    for j in 0..=3 {
                        if let (Some(p), Some(q)) = (a.d(i, j), c.d(i, j)) {
                            let e = d(p, q);
                            if e > worst {
                                worst = e;
                                worst_ij = (i, j);
                            }
                        }
                    }
                }
                println!(
                    "  {label:9} u0={u0:.1} v={v:8.4}: worst deriv mismatch {worst:.3e} at d^{}u d^{}v",
                    worst_ij.0, worst_ij.1
                );
            }
        }
    }
    // torus v seam with derivatives
    let t = Surf::torus(b, 5.0, 1.5).unwrap();
    let a = t.eval_jet(0.4, 0.0, 3, 3, false).unwrap();
    let c = t.eval_jet(0.4, tau, 3, 3, false).unwrap();
    let mut worst = 0.0f64;
    for i in 0..=3 {
        for j in 0..=3 {
            if let (Some(p), Some(q)) = (a.d(i, j), c.d(i, j)) {
                worst = worst.max(d(p, q));
            }
        }
    }
    println!("  torus v-seam worst deriv mismatch {worst:.3e}");

    println!("\n  -- Periodicity::Seamed (18022) hunt over face topology --");
    let mut bodies: Vec<(&str, Body)> = Vec::new();
    if let Ok(x) = Body::create_solid_cylinder(2.0, 6.0) {
        bodies.push(("solid cylinder", x));
    }
    if let Ok(x) = Body::create_solid_sphere(4.0) {
        bodies.push(("solid sphere", x));
    }
    if let Ok(x) = Body::create_solid_torus(5.0, 1.5) {
        bodies.push(("solid torus", x));
    }
    if let Ok(x) = Body::create_solid_cone(3.0, 5.0, 0.4) {
        bodies.push(("solid cone", x));
    }
    for (lbl, body) in &bodies {
        for (i, f) in body.faces().unwrap().iter().enumerate() {
            let mut u: PK_PARAM_periodic_t = 0;
            let mut v: PK_PARAM_periodic_t = 0;
            let rc = unsafe { PK_FACE_is_periodic(f.tag(), &mut u, &mut v) };
            println!(
                "  {lbl:15} face{i} {:?}: is_periodic rc={rc} u={u} v={v}",
                f.surface_type().unwrap()
            );
        }
    }
    // sheet bodies over full periodic uvbox (a "cut" periodic face)
    for (lbl, s, ub) in [
        (
            "sheet cyl full",
            Surf::cylinder(b, 2.0).unwrap(),
            UvBox {
                u_min: 0.0,
                v_min: 0.0,
                u_max: tau,
                v_max: 3.0,
            },
        ),
        (
            "sheet cyl half",
            Surf::cylinder(b, 2.0).unwrap(),
            UvBox {
                u_min: 0.0,
                v_min: 0.0,
                u_max: tau / 2.0,
                v_max: 3.0,
            },
        ),
        (
            "sheet sphere full",
            Surf::sphere(b, 4.0).unwrap(),
            UvBox {
                u_min: 0.0,
                v_min: -1.5707963267948966,
                u_max: tau,
                v_max: 1.5707963267948966,
            },
        ),
        (
            "sheet torus full",
            Surf::torus(b, 5.0, 1.5).unwrap(),
            UvBox {
                u_min: 0.0,
                v_min: 0.0,
                u_max: tau,
                v_max: tau,
            },
        ),
    ] {
        match s.make_sheet_body(ub) {
            Ok(body) => {
                for f in body.faces().unwrap() {
                    let mut u: PK_PARAM_periodic_t = 0;
                    let mut v: PK_PARAM_periodic_t = 0;
                    let rc = unsafe { PK_FACE_is_periodic(f.tag(), &mut u, &mut v) };
                    println!("  {lbl:18}: rc={rc} u={u} v={v}");
                }
            }
            Err(e) => println!("  {lbl:18}: make_sheet_body err {e:?}"),
        }
    }
}

// ---------------------------------------------------------------- claim 5
fn uvboxes() {
    println!("\n== CLAIM 5: find_uvbox superset? is_uvbox exact? ==");
    let b = basis();
    let tau = std::f64::consts::TAU;
    let mut bodies: Vec<(&str, Body)> = Vec::new();
    if let Ok(x) = Body::create_solid_cylinder(2.0, 6.0) {
        bodies.push(("cylinder", x));
    }
    if let Ok(x) = Body::create_solid_sphere(4.0) {
        bodies.push(("sphere", x));
    }
    if let Ok(x) = Body::create_solid_torus(5.0, 1.5) {
        bodies.push(("torus", x));
    }
    if let Ok(x) = Body::create_solid_cone(3.0, 5.0, 0.4) {
        bodies.push(("cone", x));
    }
    if let Ok(x) = Body::create_solid_block(4.0, 5.0, 6.0) {
        bodies.push(("block", x));
    }
    // a sheet body over a KNOWN exact sub-uvbox — as_uvbox should reproduce it
    for (lbl, s, ub) in [
        (
            "sheet sphere sub",
            Surf::sphere(b, 4.0).unwrap(),
            UvBox {
                u_min: 0.3,
                v_min: -0.4,
                u_max: 2.1,
                v_max: 0.9,
            },
        ),
        (
            "sheet cyl sub",
            Surf::cylinder(b, 2.0).unwrap(),
            UvBox {
                u_min: 0.25,
                v_min: -1.0,
                u_max: 2.75,
                v_max: 4.0,
            },
        ),
        (
            "sheet plane sub",
            Surf::plane(b).unwrap(),
            UvBox {
                u_min: -1.0,
                v_min: -2.0,
                u_max: 3.0,
                v_max: 5.0,
            },
        ),
        (
            "sheet torus sub",
            Surf::torus(b, 5.0, 1.5).unwrap(),
            UvBox {
                u_min: 0.1,
                v_min: 0.2,
                u_max: 1.1,
                v_max: 4.2,
            },
        ),
    ] {
        match s.make_sheet_body(ub) {
            Ok(body) => {
                for f in body.faces().unwrap() {
                    let found = f.uvbox().unwrap();
                    let exact = f.as_uvbox().unwrap();
                    println!(
                        "  {lbl:18} requested [{:.6},{:.6}]x[{:.6},{:.6}]",
                        ub.u_min, ub.u_max, ub.v_min, ub.v_max
                    );
                    println!(
                        "  {:18} find   [{:.6},{:.6}]x[{:.6},{:.6}]",
                        "", found.u_min, found.u_max, found.v_min, found.v_max
                    );
                    match exact {
                        Some(e) => {
                            println!(
                                "  {:18} is     [{:.6},{:.6}]x[{:.6},{:.6}]  err_u=({:.3e},{:.3e}) err_v=({:.3e},{:.3e})",
                                "",
                                e.u_min,
                                e.u_max,
                                e.v_min,
                                e.v_max,
                                e.u_min - ub.u_min,
                                e.u_max - ub.u_max,
                                e.v_min - ub.v_min,
                                e.v_max - ub.v_max
                            );
                            let sup = found.u_min <= e.u_min
                                && found.u_max >= e.u_max
                                && found.v_min <= e.v_min
                                && found.v_max >= e.v_max;
                            println!(
                                "  {:18} find superset of is? {sup}   slack u=({:.3e},{:.3e}) v=({:.3e},{:.3e})",
                                "",
                                e.u_min - found.u_min,
                                found.u_max - e.u_max,
                                e.v_min - found.v_min,
                                found.v_max - e.v_max
                            );
                        }
                        None => println!("  {:18} is_uvbox = false", ""),
                    }
                }
            }
            Err(e) => println!("  {lbl}: err {e:?}"),
        }
    }

    // superset test by sampling the face boundary and inverting onto the surface
    println!("\n  -- boundary-sampling superset test --");
    for (lbl, body) in &bodies {
        for (i, f) in body.faces().unwrap().iter().enumerate() {
            let found = match f.uvbox() {
                Ok(x) => x,
                Err(e) => {
                    println!("  {lbl} face{i}: uvbox err {e:?}");
                    continue;
                }
            };
            let s = f.surf().unwrap();
            let (up, vp) = {
                let (pu, pv) = s.params().unwrap();
                (
                    if pu.extent == ParamExtent::Periodic {
                        Some(pu.range.1 - pu.range.0)
                    } else {
                        None
                    },
                    if pv.extent == ParamExtent::Periodic {
                        Some(pv.range.1 - pv.range.0)
                    } else {
                        None
                    },
                )
            };
            let mut worst = 0.0f64;
            let mut n = 0;
            for e in f.edges().unwrap() {
                let (t0, t1) = match e.interval() {
                    Ok(x) => x,
                    Err(_) => continue,
                };
                let c = match e.curve() {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                for k in 0..=20 {
                    let t = t0 + (t1 - t0) * (k as f64) / 20.0;
                    let p = match c.eval(t) {
                        Ok(p) => p,
                        Err(_) => continue,
                    };
                    let (u, v) = match s.parameterise(p) {
                        Ok(x) => x,
                        Err(_) => continue,
                    };
                    n += 1;
                    let excess = |x: f64, lo: f64, hi: f64, per: Option<f64>| -> f64 {
                        let raw = (lo - x).max(x - hi).max(0.0);
                        match per {
                            Some(p) if p > 0.0 => {
                                let mut best = raw;
                                for k in -2..=2 {
                                    let y = x + (k as f64) * p;
                                    best = best.min((lo - y).max(y - hi).max(0.0));
                                }
                                best
                            }
                            _ => raw,
                        }
                    };
                    let eu = excess(u, found.u_min, found.u_max, up);
                    let ev = excess(v, found.v_min, found.v_max, vp);
                    worst = worst.max(eu.max(ev));
                }
            }
            println!(
                "  {lbl:9} face{i} {:?}: box u[{:.5},{:.5}] v[{:.5},{:.5}] samples={} worst_outside={worst:.3e} is_uvbox={}",
                f.surface_type().unwrap(),
                found.u_min,
                found.u_max,
                found.v_min,
                found.v_max,
                n,
                f.as_uvbox().map(|x| x.is_some()).unwrap_or(false)
            );
        }
    }

    // disc face padding, quantified
    println!("\n  -- disc cap padding --");
    let body = Body::create_solid_cylinder(2.0, 6.0).unwrap();
    for f in body.faces().unwrap() {
        if matches!(f.surface_type().unwrap(), SurfType::Plane) {
            let bx = f.uvbox().unwrap();
            println!(
                "  cap: [{:.9},{:.9}]x[{:.9},{:.9}]  pad={:.6}%",
                bx.u_min,
                bx.u_max,
                bx.v_min,
                bx.v_max,
                (bx.u_max - 2.0) / 2.0 * 100.0
            );
        }
    }
    let _ = tau;
}

// ---------------------------------------------------------------- claim 6
fn simpson<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    let n = if n % 2 == 0 { n } else { n + 1 };
    let h = (b - a) / n as f64;
    let mut s = f(a) + f(b);
    for i in 1..n {
        let x = a + h * i as f64;
        s += f(x) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    s * h / 3.0
}

fn arclength() {
    println!("\n== CLAIM 6: find_length enclosure conservative? ==");
    let b = basis();
    let tau = std::f64::consts::TAU;
    let circ = Curve::circle(b, 3.0).unwrap();
    let (len, lo, hi) = circ.length_with_bounds((0.0, tau)).unwrap();
    let truth = 6.0 * std::f64::consts::PI;
    println!(
        "  circle r=3: len={len:.17} lo={lo:.17} hi={hi:.17} width={:.3e}",
        hi - lo
    );
    println!(
        "    truth 6pi = {truth:.17}  len-truth={:.3e}  truth inside [lo,hi]? {}",
        len - truth,
        lo <= truth && truth <= hi
    );
    // quarter arc
    let (l2, lo2, hi2) = circ.length_with_bounds((0.0, tau / 4.0)).unwrap();
    let t2 = 1.5 * std::f64::consts::PI;
    println!(
        "  circle quarter: len={l2:.17} [{lo2:.17},{hi2:.17}] w={:.3e} truth={t2:.17} inside={}",
        hi2 - lo2,
        lo2 <= t2 && t2 <= hi2
    );

    // ellipse: independent high-accuracy perimeter
    let (a, bb) = (5.0f64, 2.0f64);
    let f = |t: f64| (a * a * t.sin() * t.sin() + bb * bb * t.cos() * t.cos()).sqrt();
    let p1 = simpson(f, 0.0, tau, 1_000_000);
    let p2 = simpson(f, 0.0, tau, 2_000_000);
    // trapezoid on a full period converges spectrally for smooth periodic f
    let n = 1_000_000usize;
    let h = tau / n as f64;
    let mut tr = 0.0f64;
    for i in 0..n {
        tr += f(i as f64 * h);
    }
    tr *= h;
    let ell = Curve::ellipse(b, 5.0, 2.0).unwrap();
    let (len, lo, hi) = ell.length_with_bounds((0.0, tau)).unwrap();
    println!(
        "  ellipse 5x2: len={len:.17} lo={lo:.17} hi={hi:.17} width={:.3e}",
        hi - lo
    );
    println!("    simpson(1e6)={p1:.17}  simpson(2e6)={p2:.17}  trapz_periodic={tr:.17}");
    println!(
        "    truth inside enclosure? {}   len-truth={:.3e}   lo-truth={:.3e} hi-truth={:.3e}",
        lo <= tr && tr <= hi,
        len - tr,
        lo - tr,
        hi - tr
    );
    // partial arcs of the ellipse
    for (t0, t1) in [
        (0.0, 1.0),
        (0.0, 0.5),
        (1.0, 2.0),
        (0.0, tau / 4.0),
        (2.5, 6.0),
    ] {
        let (l, lo, hi) = ell.length_with_bounds((t0, t1)).unwrap();
        let truth = simpson(f, t0, t1, 2_000_000);
        println!(
            "  ellipse [{t0},{t1}]: len={l:.15} [{lo:.15},{hi:.15}] w={:.3e} truth={truth:.15} inside={} err={:.3e}",
            hi - lo,
            lo <= truth && truth <= hi,
            l - truth
        );
    }
    // additivity check: does length(0,pi)+length(pi,2pi) == length(0,2pi)?
    let (la, _, _) = ell.length_with_bounds((0.0, std::f64::consts::PI)).unwrap();
    let (lb, _, _) = ell.length_with_bounds((std::f64::consts::PI, tau)).unwrap();
    println!("  ellipse additivity: {:.3e}", la + lb - len);

    // bcurve
    let bc = open_bcurve();
    let (t0, t1) = bc.interval().unwrap();
    let (l, lo, hi) = bc.length_with_bounds((t0, t1)).unwrap();
    let fb = |t: f64| {
        let j = bc.eval_jet(t, 1).unwrap();
        let d1 = j.d(1).unwrap();
        (d1.x * d1.x + d1.y * d1.y + d1.z * d1.z).sqrt()
    };
    let truth = simpson(fb, t0, t1, 20000);
    println!(
        "  bcurve [{t0},{t1}]: len={l:.15} [{lo:.15},{hi:.15}] w={:.3e} truth={truth:.15} inside={} err={:.3e}",
        hi - lo,
        lo <= truth && truth <= hi,
        l - truth
    );
}

// ------------------------------------------------- extra: Seamed (18022) hunt
fn seamed_hunt() {
    println!("\n== EXTRA: hunting PK_PARAM_periodic_seamed_c (18022) ==");
    // closed but NOT periodic b-curve: last control point == first, clamped knots
    let n = 7;
    let mut cps: Vec<Vec3> = (0..n)
        .map(|i| {
            let a = (i as f64) * std::f64::consts::TAU / (n - 1) as f64;
            Vec3::new(3.0 * a.cos(), 3.0 * a.sin(), 0.0)
        })
        .collect();
    let last = cps[0];
    let l = cps.len();
    cps[l - 1] = last;
    let verts: Vec<f64> = cps.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
    let knots: Vec<f64> = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    let mults: Vec<i32> = vec![4, 1, 1, 1, 4];
    for (lbl, closed) in [("closed=1", 1u8), ("closed=0", 0u8)] {
        let sf = PK_BCURVE_sf_t {
            degree: 3,
            n_vertices: n as std::os::raw::c_int,
            vertex_dim: 3,
            is_rational: PK_LOGICAL_false,
            vertices: verts.as_ptr(),
            _reserved_24: 0,
            n_knots: knots.len() as std::os::raw::c_int,
            knot_mult: mults.as_ptr(),
            knots: knots.as_ptr(),
            knot_type: PK_knot_non_uniform_c,
            is_periodic: 0,
            is_closed: closed,
            _pad: [0; 2],
            self_intersecting: 0,
        };
        let mut tag: PK_BCURVE_t = PK_ENTITY_null;
        let rc = unsafe { PK_BCURVE_create(&sf, &mut tag) };
        if rc != PK_ERROR_no_errors {
            println!("  bcurve seam {lbl}: create err {rc}");
            continue;
        }
        dump_curve_tag(&format!("bcurve seam {lbl}"), tag);
        // is it geometrically closed? evaluate the ends
        let mut raw = vec![0xEEu8; 96];
        let rc2 = unsafe { PK_CURVE_ask_param(tag, raw.as_mut_ptr() as *mut PK_PARAM_sf_t) };
        let lastw = (0..96).rev().find(|&i| raw[i] != 0xEE);
        println!(
            "    ask_param rc={rc2} last byte written={lastw:?} bytes32_39={:02x?}",
            &raw[32..40]
        );
        let mut p0 = [0.0f64; 3];
        let mut p1 = [0.0f64; 3];
        unsafe {
            PK_CURVE_eval(tag, 0.0, 0, p0.as_mut_ptr());
            PK_CURVE_eval(tag, 1.0, 0, p1.as_mut_ptr());
        }
        let dd =
            ((p0[0] - p1[0]).powi(2) + (p0[1] - p1[1]).powi(2) + (p0[2] - p1[2]).powi(2)).sqrt();
        println!("    |R(0)-R(1)| = {dd:.3e}  (0 => geometrically closed)");
    }
    // b-surface closed (not periodic) in u
    let nu = 7usize;
    let nv = 4usize;
    let mut cps = Vec::new();
    for i in 0..nu {
        let a = (i % (nu - 1)) as f64 * std::f64::consts::TAU / (nu - 1) as f64;
        for j in 0..nv {
            cps.push(Vec3::new(3.0 * a.cos(), 3.0 * a.sin(), j as f64));
        }
    }
    let verts: Vec<f64> = cps.iter().flat_map(|p| [p.x, p.y, p.z]).collect();
    let uk: Vec<f64> = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    let um: Vec<i32> = vec![4, 1, 1, 1, 4];
    let vk: Vec<f64> = vec![0.0, 1.0];
    let vm: Vec<i32> = vec![4, 4];
    for (lbl, closed) in [("uclosed=1", 1u8), ("uclosed=0", 0u8)] {
        let sf = PK_BSURF_sf_t {
            u_degree: 3,
            v_degree: 3,
            n_u_vertices: nu as i32,
            n_v_vertices: nv as i32,
            vertex_dim: 3,
            is_rational: PK_LOGICAL_false,
            vertices: verts.as_ptr(),
            _reserved_32: 0,
            n_u_knots: uk.len() as std::os::raw::c_int,
            n_v_knots: vk.len() as std::os::raw::c_int,
            u_knot_mult: um.as_ptr(),
            v_knot_mult: vm.as_ptr(),
            u_knots: uk.as_ptr(),
            v_knots: vk.as_ptr(),
            u_knot_type: PK_knot_non_uniform_c,
            v_knot_type: PK_knot_non_uniform_c,
            is_u_periodic: 0,
            is_v_periodic: 0,
            is_u_closed: closed,
            is_v_closed: 0,
            self_intersecting: 0,
            convexity: 0,
        };
        let mut tag: PK_BSURF_t = PK_ENTITY_null;
        let rc = unsafe { PK_BSURF_create(&sf, &mut tag) };
        if rc != PK_ERROR_no_errors {
            println!("  bsurf seam {lbl}: create err {rc}");
            continue;
        }
        dump_surf_tag(&format!("bsurf seam {lbl}"), tag);
        // and as a face
        let ub = PK_UVBOX_t {
            param: [0.0, 0.0, 1.0, 1.0],
        };
        let mut body: PK_BODY_t = PK_ENTITY_null;
        let rc = unsafe { PK_SURF_make_sheet_body(tag, ub, &mut body) };
        if rc != PK_ERROR_no_errors {
            println!("    make_sheet_body err {rc}");
            continue;
        }
        let mut faces: *mut PK_FACE_t = std::ptr::null_mut();
        let mut nf: std::os::raw::c_int = 0;
        let rc = unsafe { PK_BODY_ask_faces(body, &mut nf, &mut faces) };
        if rc == PK_ERROR_no_errors && nf > 0 {
            let f = unsafe { *faces };
            let mut pu: PK_PARAM_periodic_t = 0;
            let mut pv: PK_PARAM_periodic_t = 0;
            let rc = unsafe { PK_FACE_is_periodic(f, &mut pu, &mut pv) };
            println!("    face is_periodic rc={rc} u={pu} v={pv}");
        }
    }
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    stride_sentinel();
    families();
    seams();
    uvboxes();
    arclength();
    seamed_hunt();
}
