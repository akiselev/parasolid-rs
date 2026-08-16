//! Adversarial Stage 6 review probe (read-only; creates no repo state).
//!
//!   cargo build -p parasolid-test --bin review_boxes_probe --target x86_64-pc-windows-gnu
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/review_boxes_probe.exe

use std::f64::consts::{FRAC_PI_2, PI, TAU};

use parasolid::*;
use parasolid_sys::*;

fn basis() -> Axis2 {
    Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    )
}

fn status_name(t: i32) -> &'static str {
    match t {
        18270 => "found",
        18271 => "lower",
        18272 => "upper",
        18273 => "not_found",
        _ => "??",
    }
}

// ---------------------------------------------------------------------------
// 1. PK_range_bound_t field order, at runtime.
// ---------------------------------------------------------------------------
fn topol_range(a: i32, b: i32, mutate: impl FnOnce(&mut PK_TOPOL_range_o_t)) -> (i32, i32, f64) {
    let mut opts = PK_TOPOL_range_o_t::default();
    mutate(&mut opts);
    let mut status: PK_range_result_t = -1;
    let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
    let err = unsafe { PK_TOPOL_range(a, b, &mut opts, &mut status, &mut r) };
    (err, status, r.distance)
}

fn test_range_bound() {
    println!("== 1. PK_range_bound_t field order (runtime semantics) ==");
    // Two unit-ish blocks 5 apart along x.
    let b1 = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    let b2 = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    let t = Transform::translation(7.0, 0.0, 0.0).unwrap();
    b2.transform(&t).unwrap();
    // blocks span x in [-1,1] and [6,8]  => gap 5.0
    let (a, bb) = (b1.tag(), b2.tag());

    let (e, s, d) = topol_range(a, bb, |_| {});
    println!(
        "  defaults           : err={e} status={s} ({}) dist={d:.6}",
        status_name(s)
    );

    // Struct-named upper bound of 1.0 (true distance is 5).  If the binding is
    // right this must come back `upper` (18272).
    let (e, s, d) = topol_range(a, bb, |o| {
        o.bound.have_upper_bound = PK_LOGICAL_true;
        o.bound.upper_bound = 1.0;
    });
    println!(
        "  have_upper=1 up=1.0: err={e} status={s} ({}) dist={d:.6}   [expect upper/18272]",
        status_name(s)
    );

    // Struct-named lower bound of 10.0 (true distance is 5) => `lower` (18271).
    let (e, s, d) = topol_range(a, bb, |o| {
        o.bound.have_lower_bound = PK_LOGICAL_true;
        o.bound.lower_bound = 10.0;
    });
    println!(
        "  have_lower=1 lo=10 : err={e} status={s} ({}) dist={d:.6}   [expect lower/18271]",
        status_name(s)
    );

    // Controls: bounds that do NOT cut should leave `found`.
    let (e, s, d) = topol_range(a, bb, |o| {
        o.bound.have_upper_bound = PK_LOGICAL_true;
        o.bound.upper_bound = 100.0;
    });
    println!(
        "  have_upper=1 up=100: err={e} status={s} ({}) dist={d:.6}   [expect found]",
        status_name(s)
    );
    let (e, s, d) = topol_range(a, bb, |o| {
        o.bound.have_lower_bound = PK_LOGICAL_true;
        o.bound.lower_bound = 0.1;
    });
    println!(
        "  have_lower=1 lo=0.1: err={e} status={s} ({}) dist={d:.6}   [expect found]",
        status_name(s)
    );

    // Raw-byte cross-check: write the flag/double pairs by offset directly so the
    // Rust field names cannot mask a swap.
    for (flag_off, val_off, val, label) in [
        (
            0usize,
            8usize,
            1.0f64,
            "@0 flag,@8 val = 1.0 (claimed UPPER)",
        ),
        (
            16usize,
            24usize,
            10.0f64,
            "@16 flag,@24 val = 10.0 (claimed LOWER)",
        ),
    ] {
        let mut opts = PK_TOPOL_range_o_t::default();
        unsafe {
            let base = (&mut opts as *mut PK_TOPOL_range_o_t as *mut u8).add(16); // bound @16
            *(base.add(flag_off) as *mut i32) = 1;
            *(base.add(val_off) as *mut f64) = val;
        }
        let mut status: PK_range_result_t = -1;
        let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
        let err = unsafe { PK_TOPOL_range(a, bb, &mut opts, &mut status, &mut r) };
        println!(
            "  raw {label:38}: err={err} status={status} ({}) dist={:.6}",
            status_name(status),
            r.distance
        );
    }
    println!();
}

// ---------------------------------------------------------------------------
// 2/3. PK_GEOM_range + param bound.
// ---------------------------------------------------------------------------
fn geom_range(
    g1: i32,
    g2: i32,
    mutate: impl FnOnce(&mut PK_GEOM_range_o_t),
) -> (i32, i32, PK_range_2_r_t) {
    let mut opts = PK_GEOM_range_o_t::default();
    mutate(&mut opts);
    let mut status: PK_range_result_t = -1;
    let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
    let err = unsafe { PK_GEOM_range(g1, g2, &mut opts, &mut status, &mut r) };
    (err, status, r)
}

fn test_geom_range() {
    println!("== 2/3. PK_GEOM_range and PK_range_param_bound_t ==");
    println!(
        "  sizeof(PK_GEOM_range_o_t)      = {}",
        std::mem::size_of::<PK_GEOM_range_o_t>()
    );
    println!(
        "  sizeof(PK_range_param_bound_t) = {}",
        std::mem::size_of::<PK_range_param_bound_t>()
    );

    let b = basis();
    // Sphere radius 4 at origin vs sphere radius 1 at (20,0,0): distance 15.
    let s1 = Surf::sphere(b, 4.0).unwrap();
    let b2 = Axis2::new(
        Vec3::new(20.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let s2 = Surf::sphere(b2, 1.0).unwrap();
    let (e, s, r) = geom_range(s1.tag(), s2.tag(), |_| {});
    println!(
        "  sphere(4)@0 vs sphere(1)@20: err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4}) p2=({:.4},{:.4},{:.4})  [expect 15]",
        status_name(s),
        r.distance,
        r.end_1.position[0],
        r.end_1.position[1],
        r.end_1.position[2],
        r.end_2.position[0],
        r.end_2.position[1],
        r.end_2.position[2]
    );

    // Line along +x through origin vs a point at (100, 5, 0).
    let line = Curve::line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).unwrap();
    let pt = Point::create(Vec3::new(100.0, 5.0, 0.0)).unwrap();
    let (e, s, r) = geom_range(line.tag(), pt.tag(), |_| {});
    println!(
        "  line(+x) vs point(100,5,0)  : err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4})  [expect 5, p1=(100,0,0)]",
        status_name(s),
        r.distance,
        r.end_1.position[0],
        r.end_1.position[1],
        r.end_1.position[2]
    );

    // Now restrict the LINE (geom_1) to parameter [0,10].  Closest point should
    // move to (10,0,0) => distance sqrt(90^2 + 5^2) = 90.1388...
    let expect = (90.0f64 * 90.0 + 25.0).sqrt();
    let (e, s, r) = geom_range(line.tag(), pt.tag(), |o| {
        o.param_bound[0].have_param_bound = PK_LOGICAL_true;
        o.param_bound[0].param_bound_class = PK_range_param_bound_class_interval_c;
        o.param_bound[0].bound = [0.0, 10.0, 0.0, 0.0];
    });
    println!(
        "  + param_bound[0]=[0,10]     : err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4})  [expect {expect:.6}, p1=(10,0,0)]",
        status_name(s),
        r.distance,
        r.end_1.position[0],
        r.end_1.position[1],
        r.end_1.position[2]
    );

    // And the mirror: restrict to [-10,0] => closest at (0,0,0), dist sqrt(100^2+25).
    let expect2 = (100.0f64 * 100.0 + 25.0).sqrt();
    let (e, s, r) = geom_range(line.tag(), pt.tag(), |o| {
        o.param_bound[0].have_param_bound = PK_LOGICAL_true;
        o.param_bound[0].param_bound_class = PK_range_param_bound_class_interval_c;
        o.param_bound[0].bound = [-10.0, 0.0, 0.0, 0.0];
    });
    println!(
        "  + param_bound[0]=[-10,0]    : err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4})  [expect {expect2:.6}, p1=(0,0,0)]",
        status_name(s),
        r.distance,
        r.end_1.position[0],
        r.end_1.position[1],
        r.end_1.position[2]
    );

    // Wrong class value: does the kernel actually validate 0x204?
    let (e, s, r) = geom_range(line.tag(), pt.tag(), |o| {
        o.param_bound[0].have_param_bound = PK_LOGICAL_true;
        o.param_bound[0].param_bound_class = 0;
        o.param_bound[0].bound = [0.0, 10.0, 0.0, 0.0];
    });
    println!(
        "  + class=0 (illegal?)        : err={e} status={s} ({}) dist={:.6}",
        status_name(s),
        r.distance
    );

    // uvbox param bound on a surface: sphere restricted to the +x/+y/+z octant.
    let sph = Surf::sphere(b, 4.0).unwrap();
    let uvb = sph.uvbox().unwrap();
    println!(
        "  sphere uvbox = u[{:.6},{:.6}] v[{:.6},{:.6}]",
        uvb.u_min, uvb.u_max, uvb.v_min, uvb.v_max
    );
    let far = Point::create(Vec3::new(-100.0, 0.0, 0.0)).unwrap();
    let (e, s, r) = geom_range(sph.tag(), far.tag(), |_| {});
    println!(
        "  sphere vs point(-100,0,0)   : err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4})  [expect 96]",
        status_name(s),
        r.distance,
        r.end_1.position[0],
        r.end_1.position[1],
        r.end_1.position[2]
    );
    // Restrict u to [0, pi/2] (the +x/+y quarter) — closest point to (-100,0,0)
    // should move away from (-4,0,0).
    let (e, s, r) = geom_range(sph.tag(), far.tag(), |o| {
        o.param_bound[0].have_param_bound = PK_LOGICAL_true;
        o.param_bound[0].param_bound_class = 0; // non-interval => uvbox form
        o.param_bound[0].bound = [0.0, uvb.v_min, FRAC_PI_2, uvb.v_max];
    });
    println!(
        "  + uvbox u[0,pi/2]           : err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4})  [expect >96 if honoured]",
        status_name(s),
        r.distance,
        r.end_1.position[0],
        r.end_1.position[1],
        r.end_1.position[2]
    );
    println!();
}

// ---------------------------------------------------------------------------
// 4/5. Box restriction really applied?  Sampled-containment test.
// ---------------------------------------------------------------------------
fn sample_curve_box(c: &Curve, t0: f64, t1: f64, n: usize) -> (Vec3, Vec3) {
    let mut lo = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut hi = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for i in 0..=n {
        let t = t0 + (t1 - t0) * (i as f64) / (n as f64);
        if let Ok(p) = c.eval(t) {
            lo = Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
    }
    (lo, hi)
}

fn sample_surf_box(s: &Surf, uv: UvBox, n: usize) -> (Vec3, Vec3) {
    let mut lo = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let mut hi = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);
    for i in 0..=n {
        for j in 0..=n {
            let u = uv.u_min + (uv.u_max - uv.u_min) * (i as f64) / (n as f64);
            let v = uv.v_min + (uv.v_max - uv.v_min) * (j as f64) / (n as f64);
            if let Ok(p) = s.eval(u, v) {
                lo = Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
                hi = Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
            }
        }
    }
    (lo, hi)
}

fn compare(label: &str, got: &Aabb, lo: Vec3, hi: Vec3) {
    // slack = how far the reported box extends BEYOND the sampled extent.
    let slack = [
        lo.x - got.min.x,
        lo.y - got.min.y,
        lo.z - got.min.z,
        got.max.x - hi.x,
        got.max.y - hi.y,
        got.max.z - hi.z,
    ];
    let worst = slack.iter().cloned().fold(f64::INFINITY, f64::min);
    let biggest = slack.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let verdict = if worst < -1e-12 {
        "*** INWARD vs SAMPLES — UNSAFE ***"
    } else if worst < 0.0 {
        "inward by <1e-12 (rounding)"
    } else if biggest == 0.0 {
        "exactly tight"
    } else if biggest > 1e-6 {
        "PADDED / restriction maybe ignored"
    } else {
        "conservative (tiny)"
    };
    println!("  {label:38} worst={worst:+.3e} max={biggest:+.3e}  {verdict}");
    println!(
        "      got   [{:+.9},{:+.9},{:+.9}]..[{:+.9},{:+.9},{:+.9}]",
        got.min.x, got.min.y, got.min.z, got.max.x, got.max.y, got.max.z
    );
    println!(
        "      sample[{:+.9},{:+.9},{:+.9}]..[{:+.9},{:+.9},{:+.9}]",
        lo.x, lo.y, lo.z, hi.x, hi.y, hi.z
    );
}

fn test_boxes() {
    println!("== 4/5. find_box restriction + tightness vs sampled truth ==");
    let b = basis();

    // --- The headline claim: quarter arc. ---
    let circ = Curve::circle(b, 3.0).unwrap();
    let full = circ.find_box(None).unwrap();
    println!(
        "  circle r=3 whole  : [{:+.17e},{:+.17e}]..[{:+.17e},{:+.17e}]",
        full.min.x, full.min.y, full.max.x, full.max.y
    );
    let q = circ.find_box(Some((0.0, FRAC_PI_2))).unwrap();
    println!(
        "  circle r=3 quarter: [{:+.17e},{:+.17e}]..[{:+.17e},{:+.17e}]",
        q.min.x, q.min.y, q.max.x, q.max.y
    );
    println!(
        "  --> quarter restriction {} (min.x = {:+.3e}; -3 means IGNORED)",
        if q.min.x < -1.0 { "IGNORED" } else { "applied" },
        q.min.x
    );
    let (lo, hi) = sample_curve_box(&circ, 0.0, FRAC_PI_2, 4000);
    compare("circle quarter arc", &q, lo, hi);

    // Half arc and a third arc, in case pi/2 is special.
    for (t0, t1, name) in [
        (0.0, PI, "circle half arc [0,pi]"),
        (0.3, 1.1, "circle arc [0.3,1.1]"),
        (2.0, 5.0, "circle arc [2,5]"),
    ] {
        let bx = circ.find_box(Some((t0, t1))).unwrap();
        let (lo, hi) = sample_curve_box(&circ, t0, t1, 4000);
        compare(name, &bx, lo, hi);
    }

    // Ellipse (asymmetric) restricted.
    let ell = Curve::ellipse(b, 6.0, 2.0).unwrap();
    for (t0, t1, name) in [
        (0.0, FRAC_PI_2, "ellipse 6x2 quarter"),
        (0.7, 2.4, "ellipse 6x2 [0.7,2.4]"),
    ] {
        match ell.find_box(Some((t0, t1))) {
            Ok(bx) => {
                let (lo, hi) = sample_curve_box(&ell, t0, t1, 4000);
                compare(name, &bx, lo, hi);
            }
            Err(e) => println!("  {name}: ERROR {e}"),
        }
    }

    // Line restricted — an unbounded carrier, so this is the strongest test of
    // whether have_interval is honoured (unrestricted must fail).
    let line = Curve::line(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 1.0)).unwrap();
    match line.find_box(None) {
        Ok(bx) => println!("  line unrestricted -> {bx:?}  [expected an error]"),
        Err(e) => println!("  line unrestricted -> ERROR {e}  (good)"),
    }
    match line.find_box(Some((2.0, 5.0))) {
        Ok(bx) => {
            let (lo, hi) = sample_curve_box(&line, 2.0, 5.0, 1000);
            compare("line [2,5]", &bx, lo, hi);
        }
        Err(e) => println!("  line [2,5]: ERROR {e}"),
    }

    // --- surfaces ---
    let tor = Surf::torus(b, 5.0, 1.5).unwrap();
    let tuv = tor.uvbox().unwrap();
    for (uv, name) in [
        (tuv, "torus full uvbox"),
        (
            UvBox {
                u_min: 0.0,
                u_max: FRAC_PI_2,
                v_min: tuv.v_min,
                v_max: tuv.v_max,
            },
            "torus u[0,pi/2]",
        ),
        (
            UvBox {
                u_min: 0.4,
                u_max: 2.2,
                v_min: 0.3,
                v_max: 1.9,
            },
            "torus partial u+v",
        ),
    ] {
        match tor.find_box(Some(uv)) {
            Ok(bx) => {
                let (lo, hi) = sample_surf_box(&tor, uv, 400);
                compare(name, &bx, lo, hi);
            }
            Err(e) => println!("  {name}: ERROR {e}"),
        }
    }

    let cone = Surf::cone(b, 2.0, 0.4).unwrap();
    let cuv = UvBox {
        u_min: 0.0,
        u_max: TAU,
        v_min: 0.0,
        v_max: 4.0,
    };
    match cone.find_box(Some(cuv)) {
        Ok(bx) => {
            let (lo, hi) = sample_surf_box(&cone, cuv, 400);
            compare("cone r=2 semi=0.4 v[0,4]", &bx, lo, hi);
        }
        Err(e) => println!("  cone: ERROR {e}"),
    }
    let cuv2 = UvBox {
        u_min: 0.5,
        u_max: 2.0,
        v_min: 1.0,
        v_max: 3.0,
    };
    match cone.find_box(Some(cuv2)) {
        Ok(bx) => {
            let (lo, hi) = sample_surf_box(&cone, cuv2, 400);
            compare("cone partial u+v", &bx, lo, hi);
        }
        Err(e) => println!("  cone partial: ERROR {e}"),
    }

    // Spun surface: a line profile spun about z -> a cone/cylinder-ish.
    let n = (0.3f64 * 0.3 + 1.0).sqrt();
    let prof = Curve::line(Vec3::new(3.0, 0.0, 0.0), Vec3::new(0.3 / n, 0.0, 1.0 / n)).unwrap();
    match Surf::spun(&prof, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)) {
        Ok(sp) => {
            let uv = UvBox {
                u_min: 0.0,
                u_max: 2.0,
                v_min: 0.0,
                v_max: 5.0,
            };
            match sp.find_box(Some(uv)) {
                Ok(bx) => {
                    let (lo, hi) = sample_surf_box(&sp, uv, 400);
                    compare("spun surface partial", &bx, lo, hi);
                }
                Err(e) => println!("  spun: ERROR {e}"),
            }
        }
        Err(e) => println!("  spun create ERROR: {e}"),
    }

    // Swept surface.
    match Surf::swept(&circ, Vec3::new(0.0, 0.0, 7.0)) {
        Ok(sw) => {
            let uv = UvBox {
                u_min: 0.2,
                u_max: 2.5,
                v_min: 1.0,
                v_max: 6.0,
            };
            match sw.find_box(Some(uv)) {
                Ok(bx) => {
                    let (lo, hi) = sample_surf_box(&sw, uv, 400);
                    compare("swept circle partial", &bx, lo, hi);
                }
                Err(e) => println!("  swept: ERROR {e}"),
            }
        }
        Err(e) => println!("  swept create ERROR: {e}"),
    }

    // B-surface: a bi-cubic bump (control net is NOT the tight box, so this is
    // the strongest padding test).
    let mut cps = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            let x = i as f64;
            let y = j as f64;
            let z = if (i == 1 || i == 2) && (j == 1 || j == 2) {
                4.0
            } else {
                0.0
            };
            cps.push(Vec3::new(x, y, z));
        }
    }
    match Surf::bsurf(3, 3, 4, 4, &cps, &[0.0, 1.0], &[4, 4], &[0.0, 1.0], &[4, 4]) {
        Ok(bs) => {
            let uv = bs.uvbox().unwrap_or(UvBox {
                u_min: 0.0,
                u_max: 1.0,
                v_min: 0.0,
                v_max: 1.0,
            });
            match bs.find_box(Some(uv)) {
                Ok(bx) => {
                    let (lo, hi) = sample_surf_box(&bs, uv, 300);
                    compare("bsurf bump (full)", &bx, lo, hi);
                }
                Err(e) => println!("  bsurf: ERROR {e}"),
            }
            let uv2 = UvBox {
                u_min: 0.2,
                u_max: 0.6,
                v_min: 0.1,
                v_max: 0.9,
            };
            match bs.find_box(Some(uv2)) {
                Ok(bx) => {
                    let (lo, hi) = sample_surf_box(&bs, uv2, 300);
                    compare("bsurf bump (partial uv)", &bx, lo, hi);
                }
                Err(e) => println!("  bsurf partial: ERROR {e}"),
            }
        }
        Err(e) => println!("  bsurf create ERROR: {e}"),
    }

    // Oblique/rotated primitive.
    let ang = 0.6f64;
    #[rustfmt::skip]
    let rot = [
        ang.cos(), -ang.sin(), 0.0, 0.0,
        ang.sin(),  ang.cos(), 0.0, 0.0,
        0.0,        0.0,       1.0, 0.0,
        0.0,        0.0,       0.0, 1.0,
    ];
    let tf = Transform::from_matrix(rot).unwrap();
    let cyl = Surf::cylinder(
        Axis2::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        2.0,
    )
    .unwrap();
    match cyl.transformed(&tf) {
        Ok((rc, exact)) => {
            let uv = UvBox {
                u_min: 0.3,
                u_max: 3.1,
                v_min: -1.0,
                v_max: 5.0,
            };
            match rc.find_box(Some(uv)) {
                Ok(bx) => {
                    let (lo, hi) = sample_surf_box(&rc, uv, 400);
                    compare(&format!("rotated cylinder (exact={exact})"), &bx, lo, hi);
                }
                Err(e) => println!("  rotated cylinder: ERROR {e}"),
            }
        }
        Err(e) => println!("  transform surf ERROR: {e}"),
    }

    // Body after a rigid transform (rotation about z by 0.6 then translate).
    let blk = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    #[rustfmt::skip]
    let m = [
        ang.cos(), -ang.sin(), 0.0, 1.5,
        ang.sin(),  ang.cos(), 0.0, -2.5,
        0.0,        0.0,       1.0, 0.75,
        0.0,        0.0,       0.0, 1.0,
    ];
    let tf2 = Transform::from_matrix(m).unwrap();
    blk.transform(&tf2).unwrap();
    let bx = blk.bounding_box().unwrap();
    // exact: rotate the 8 corners.
    let (mut lo, mut hi) = (
        Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
    );
    for &sx in &[-5.0f64, 5.0] {
        for &sy in &[-10.0f64, 10.0] {
            for &sz in &[0.0f64, 30.0] {
                let x = ang.cos() * sx - ang.sin() * sy + 1.5;
                let y = ang.sin() * sx + ang.cos() * sy - 2.5;
                let z = sz + 0.75;
                lo = Vec3::new(lo.x.min(x), lo.y.min(y), lo.z.min(z));
                hi = Vec3::new(hi.x.max(x), hi.y.max(y), hi.z.max(z));
            }
        }
    }
    compare("rotated block body", &bx, lo, hi);
    println!();
}

// ---------------------------------------------------------------------------
// 6. widths: half or full?
// ---------------------------------------------------------------------------
fn obb_check(label: &str, ob: &OrientedBox, pts: &[Vec3]) {
    let mut max_proj = [0.0f64; 3];
    for p in pts {
        let d = Vec3::new(p.x - ob.centre.x, p.y - ob.centre.y, p.z - ob.centre.z);
        for i in 0..3 {
            let a = ob.axes[i];
            let pr = (d.x * a.x + d.y * a.y + d.z * a.z).abs();
            if pr > max_proj[i] {
                max_proj[i] = pr;
            }
        }
    }
    println!("  {label}");
    println!(
        "    dim={} widths=({:.6},{:.6},{:.6}) centre=({:.6},{:.6},{:.6})",
        ob.dimension,
        ob.widths[0],
        ob.widths[1],
        ob.widths[2],
        ob.centre.x,
        ob.centre.y,
        ob.centre.z
    );
    println!(
        "    max|proj| = ({:.6},{:.6},{:.6})   ratio width/maxproj = ({:.4},{:.4},{:.4})",
        max_proj[0],
        max_proj[1],
        max_proj[2],
        ob.widths[0] / max_proj[0].max(1e-300),
        ob.widths[1] / max_proj[1].max(1e-300),
        ob.widths[2] / max_proj[2].max(1e-300),
    );
    let inside = pts.iter().all(|p| ob.contains(*p, 1e-9));
    println!("    contains() holds for every sample: {inside}");
    // axis orthonormality
    for i in 0..3 {
        let a = ob.axes[i];
        let n = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();
        print!("    |axis{i}|={n:.6} ");
    }
    println!();
}

fn test_widths() {
    println!("== 6. non-aligned box widths: half or full? ==");
    let b = basis();

    // Asymmetric, off-centre: quarter of an ellipse 6x2 -> box [0,0]..[6,2],
    // centre (3,1,0), half-widths (3,1); full widths would be (6,2).
    let ell = Curve::ellipse(b, 6.0, 2.0).unwrap();
    let pts: Vec<Vec3> = (0..=800)
        .map(|i| ell.eval(FRAC_PI_2 * i as f64 / 800.0).unwrap())
        .collect();
    match ell.find_oriented_box((0.0, FRAC_PI_2)) {
        Ok(ob) => obb_check("ellipse 6x2 quarter arc", &ob, &pts),
        Err(e) => println!("  ellipse quarter obox ERROR: {e}"),
    }

    // Full ellipse: centre at origin, half-widths (6,2).
    let pts: Vec<Vec3> = (0..=2000)
        .map(|i| ell.eval(TAU * i as f64 / 2000.0).unwrap())
        .collect();
    match ell.find_oriented_box((0.0, TAU)) {
        Ok(ob) => obb_check("ellipse 6x2 full", &ob, &pts),
        Err(e) => println!("  ellipse full obox ERROR: {e}"),
    }

    // Off-origin line segment: from (1,2,3) to (1,2,13).
    let line = Curve::line(Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 1.0)).unwrap();
    let pts: Vec<Vec3> = (0..=200)
        .map(|i| line.eval(10.0 * i as f64 / 200.0).unwrap())
        .collect();
    match line.find_oriented_box((0.0, 10.0)) {
        Ok(ob) => obb_check("line (1,2,3)->(1,2,13)", &ob, &pts),
        Err(e) => println!("  line obox ERROR: {e}"),
    }

    // Half a cylinder: strongly asymmetric surface.
    let cyl = Surf::cylinder(b, 2.0).unwrap();
    let uv = UvBox {
        u_min: 0.0,
        u_max: PI,
        v_min: 0.0,
        v_max: 6.0,
    };
    let mut pts = Vec::new();
    for i in 0..=100 {
        for j in 0..=40 {
            let u = PI * i as f64 / 100.0;
            let v = 6.0 * j as f64 / 40.0;
            pts.push(cyl.eval(u, v).unwrap());
        }
    }
    match cyl.find_oriented_box(uv) {
        Ok(ob) => obb_check("half cylinder r=2 v[0,6]", &ob, &pts),
        Err(e) => println!("  half cylinder obox ERROR: {e}"),
    }
    println!();
}

// ---------------------------------------------------------------------------
// 7. Default impls: legal tokens?
// ---------------------------------------------------------------------------
fn dump<T>(label: &str, v: &T) {
    let n = std::mem::size_of::<T>();
    let bytes = unsafe { std::slice::from_raw_parts(v as *const T as *const u8, n) };
    print!("  {label} ({n} B):");
    for (i, c) in bytes.chunks(4).enumerate() {
        let w = i32::from_le_bytes([c[0], c[1], c[2], c[3]]);
        print!(" @{}={}", i * 4, w);
    }
    println!();
}

fn test_defaults() {
    println!("== 7. Default option-struct byte images ==");
    dump("PK_range_bound_t", &PK_range_bound_t::default());
    dump("PK_TOPOL_range_o_t", &PK_TOPOL_range_o_t::default());
    dump(
        "PK_TOPOL_range_vector_o_t",
        &PK_TOPOL_range_vector_o_t::default(),
    );
    dump("PK_GEOM_range_o_t", &PK_GEOM_range_o_t::default());
    dump(
        "PK_GEOM_range_vector_o_t",
        &PK_GEOM_range_vector_o_t::default(),
    );
    dump("PK_CURVE_find_box_o_t", &PK_CURVE_find_box_o_t::default());
    dump("PK_SURF_find_box_o_t", &PK_SURF_find_box_o_t::default());
    println!();
}

// ---------------------------------------------------------------------------
// 8. o_t_version gating: are range_type / param_bound / opt_level even read?
// ---------------------------------------------------------------------------
fn test_versions() {
    println!("== 8. o_t_version gating of the range options ==");

    // (a) an ILLEGAL opt_level token proves whether the field is read at all.
    let b1 = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    let b2 = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    b2.transform(&Transform::translation(7.0, 0.0, 0.0).unwrap())
        .unwrap();
    for v in [1, 2, 3] {
        let (e, s, d) = topol_range(b1.tag(), b2.tag(), |o| {
            o.o_t_version = v;
            o.opt_level = 12345; // not a legal PK_range_opt_t
        });
        println!(
            "  TOPOL_range v{v} opt_level=12345 : err={e} status={s} ({}) dist={d:.6}",
            status_name(s)
        );
    }
    // (b) range_type = maximum: v1 cannot carry it, v2/v3 can.
    for v in [1, 2, 3] {
        let (e, s, d) = topol_range(b1.tag(), b2.tag(), |o| {
            o.o_t_version = v;
            o.range_type = PK_range_type_maximum_c;
        });
        println!(
            "  TOPOL_range v{v} range_type=MAX  : err={e} status={s} ({}) dist={d:.6}  [min=5]",
            status_name(s)
        );
    }

    // (c) GEOM_range param_bound at o_t_version = 3.
    let line = Curve::line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).unwrap();
    let pt = Point::create(Vec3::new(100.0, 5.0, 0.0)).unwrap();
    let expect = (90.0f64 * 90.0 + 25.0).sqrt();
    for v in [1, 2, 3] {
        let (e, s, r) = geom_range(line.tag(), pt.tag(), |o| {
            o.o_t_version = v;
            o.param_bound[0].have_param_bound = PK_LOGICAL_true;
            o.param_bound[0].param_bound_class = PK_range_param_bound_class_interval_c;
            o.param_bound[0].bound = [0.0, 10.0, 0.0, 0.0];
        });
        println!(
            "  GEOM_range v{v} pbound[0,10]      : err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4})  [expect {expect:.6}]",
            status_name(s),
            r.distance,
            r.end_1.position[0],
            r.end_1.position[1],
            r.end_1.position[2]
        );
    }
    // illegal class at v3 -> should be rejected if 0x204 is really validated
    let (e, s, r) = geom_range(line.tag(), pt.tag(), |o| {
        o.o_t_version = 3;
        o.param_bound[0].have_param_bound = PK_LOGICAL_true;
        o.param_bound[0].param_bound_class = 999;
        o.param_bound[0].bound = [0.0, 10.0, 0.0, 0.0];
    });
    println!(
        "  GEOM_range v3 class=999           : err={e} status={s} ({}) dist={:.6}",
        status_name(s),
        r.distance
    );
    // uvbox form at v3 on a sphere
    let b = basis();
    let sph = Surf::sphere(b, 4.0).unwrap();
    let uvb = sph.uvbox().unwrap();
    let far = Point::create(Vec3::new(-100.0, 0.0, 0.0)).unwrap();
    for class in [0x206, 0x205, 0x203, 0] {
        let (e, s, r) = geom_range(sph.tag(), far.tag(), |o| {
            o.o_t_version = 3;
            o.param_bound[0].have_param_bound = PK_LOGICAL_true;
            o.param_bound[0].param_bound_class = class;
            o.param_bound[0].bound = [0.0, uvb.v_min, FRAC_PI_2, uvb.v_max];
        });
        println!(
            "  GEOM_range v3 uvbox class={class:#x}      : err={e} status={s} ({}) dist={:.6} p1=({:.4},{:.4},{:.4})  [96 = ignored]",
            status_name(s),
            r.distance,
            r.end_1.position[0],
            r.end_1.position[1],
            r.end_1.position[2]
        );
    }

    // (d) GEOM_range_vector opt_level: illegal token accepted at v1?
    for v in [1, 2, 3] {
        let mut opts = PK_GEOM_range_vector_o_t::default();
        opts.o_t_version = v;
        opts.opt_level = 12345;
        let vpos: PK_VECTOR_t = [10.0, 0.0, 0.0];
        let mut status: PK_range_result_t = -1;
        let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
        let e = unsafe { PK_GEOM_range_vector(sph.tag(), &vpos, &mut opts, &mut status, &mut r) };
        println!(
            "  GEOM_range_vector v{v} opt=12345  : err={e} status={status} dist={:.6}",
            r.distance
        );
    }
    // (e) TOPOL_range_vector param_entity: illegal token accepted at v1?
    for v in [1, 2, 3] {
        let mut opts = PK_TOPOL_range_vector_o_t::default();
        opts.o_t_version = v;
        opts.param_entity = 12345;
        let vpos: PK_VECTOR_t = [10.0, 0.0, 0.0];
        let mut status: PK_range_result_t = -1;
        let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
        let e = unsafe { PK_TOPOL_range_vector(b1.tag(), &vpos, &mut opts, &mut status, &mut r) };
        println!(
            "  TOPOL_range_vector v{v} pe=12345  : err={e} status={status} dist={:.6}",
            r.distance
        );
    }
    println!();
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    test_defaults();
    test_versions();
    test_range_bound();
    test_geom_range();
    test_boxes();
    test_widths();
    println!("== done");
}
