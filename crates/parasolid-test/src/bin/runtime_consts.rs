//! DIG #1 — pin the runtime numeric tolerance constants of Parasolid via the live oracle.
//! Route A: direct PK_SESSION_ask_precision / ask_angle_precision.
//! Route B: behavioral binary-search of operational thresholds via Surf::intersect,
//!          swept across session precision to see which thresholds scale with T.

use parasolid::*;

/// Compact classification of an intersect outcome.
#[derive(Debug, Clone, PartialEq)]
enum Cls {
    Err(String),
    Empty,               // 0 points, 0 curves
    Points(usize),       // isolated (tangent) point(s), 0 curves
    Curves(usize, String), // curve(s); string = first curve type
    Mixed(usize, usize, String),
}

fn classify(r: PsResult<SurfIntersection>) -> Cls {
    match r {
        Err(e) => Cls::Err(format!("{e}")),
        Ok(si) => {
            let np = si.points.len();
            let nc = si.curves.len();
            let ty = si
                .curves
                .first()
                .map(|c| format!("{:?}", c.curve.curve_type().unwrap_or(CurveType::Line)))
                .unwrap_or_default();
            match (np, nc) {
                (0, 0) => Cls::Empty,
                (p, 0) => Cls::Points(p),
                (0, c) => Cls::Curves(c, ty),
                (p, c) => Cls::Mixed(p, c, ty),
            }
        }
    }
}

/// Which "regime" a class falls in for threshold detection.
#[derive(PartialEq, Clone, Copy, Debug)]
enum Regime { Curve, Point, Empty, Err }
fn regime(c: &Cls) -> Regime {
    match c {
        Cls::Curves(..) | Cls::Mixed(..) => Regime::Curve,
        Cls::Points(_) => Regime::Point,
        Cls::Empty => Regime::Empty,
        Cls::Err(_) => Regime::Err,
    }
}

const O: fn() -> Vec3 = Vec3::zero;
fn z() -> Vec3 { Vec3::new(0.0, 0.0, 1.0) }
fn x() -> Vec3 { Vec3::new(1.0, 0.0, 0.0) }
fn y() -> Vec3 { Vec3::new(0.0, 1.0, 0.0) }

/// Two equal spheres radius R, centers distance d apart on X. gap g = d - 2R.
fn two_spheres(r: f64, g: f64) -> Cls {
    let d = 2.0 * r + g;
    let a = Surf::sphere(Axis2::new(O(), z(), x()), r);
    let b = Surf::sphere(Axis2::new(Vec3::new(d, 0.0, 0.0), z(), x()), r);
    match (a, b) {
        (Ok(a), Ok(b)) => classify(a.intersect(&b)),
        _ => Cls::Err("ctor".into()),
    }
}

/// Sphere radius R at origin ∩ plane z=h, targeting circle radius rho (h=sqrt(R^2-rho^2)).
fn sphere_plane_circle(r: f64, rho: f64) -> Cls {
    if rho > r { return Cls::Err("rho>r".into()); }
    let h = (r * r - rho * rho).sqrt();
    let s = Surf::sphere(Axis2::new(O(), z(), x()), r);
    let p = Surf::plane(Axis2::new(Vec3::new(0.0, 0.0, h), z(), x()));
    match (s, p) {
        (Ok(s), Ok(p)) => classify(s.intersect(&p)),
        _ => Cls::Err("ctor".into()),
    }
}

/// Two planes through origin; B tilted by angle theta about X. Intersect in x-axis line.
fn two_planes_angle(theta: f64) -> Cls {
    let a = Surf::plane(Axis2::new(O(), z(), x()));
    let nb = Vec3::new(0.0, theta.sin(), theta.cos());
    let b = Surf::plane(Axis2::new(O(), nb, x()));
    match (a, b) {
        (Ok(a), Ok(b)) => classify(a.intersect(&b)),
        _ => Cls::Err("ctor".into()),
    }
}

/// Two parallel planes normal +Z, separated by gap g along Z.
fn two_planes_gap(g: f64) -> Cls {
    let a = Surf::plane(Axis2::new(O(), z(), x()));
    let b = Surf::plane(Axis2::new(Vec3::new(0.0, 0.0, g), z(), x()));
    match (a, b) {
        (Ok(a), Ok(b)) => classify(a.intersect(&b)),
        _ => Cls::Err("ctor".into()),
    }
}

/// Binary-search the boundary in `param` where regime changes from `lo_regime` to something else.
/// Returns (last param with lo_regime, first param past boundary, its regime).
/// `f` maps param -> Cls; searches within [lo, hi] with `lo` giving lo_regime.
fn bisect(lo: f64, hi: f64, log_space: bool, lo_regime: Regime, f: &dyn Fn(f64) -> Cls) -> (f64, f64, Regime) {
    let mut a = lo;
    let mut b = hi;
    // Confirm endpoints straddle a change.
    for _ in 0..60 {
        let m = if log_space { (a.ln() * 0.5 + b.ln() * 0.5).exp() } else { 0.5 * (a + b) };
        if regime(&f(m)) == lo_regime { a = m; } else { b = m; }
        let rel = if log_space { (b / a - 1.0).abs() } else { (b - a).abs() };
        if rel < 1e-12 { break; }
    }
    (a, b, regime(&f(b)))
}

fn run_battery(label: &str) {
    println!("\n######## BATTERY: {label} ########");

    // --- B.1 tangent spheres (R=1) : sweep gap, then bisect point<->empty (tangent tol) ---
    println!("[B.1] two equal spheres R=1, gap g = d-2R  (curve->point->empty as g grows)");
    for &g in &[-1e-1, -1e-3, -1e-5, -1e-7, -1e-9, -1e-11, 0.0, 1e-11, 1e-9, 1e-7, 1e-5, 1e-3] {
        println!("      g={:+.1e}  -> {:?}", g, two_spheres(1.0, g));
    }
    // point<->empty boundary (positive gap). Find largest g still non-empty.
    {
        let f = |g: f64| two_spheres(1.0, g);
        // establish a lo (non-empty) and hi (empty)
        let lo = 0.0; // g=0 should be point
        let hi = 1e-3;
        let r0 = regime(&f(lo));
        let (last, first, _) = bisect(lo, hi, false, r0, &f);
        println!("      -> R=1 touch-vanishes (non-empty -> empty) at gap ~ {:.3e} .. {:.3e}", last, first);
    }
    // scale check R=100
    {
        let f = |g: f64| two_spheres(100.0, g);
        let lo = 0.0; let hi = 1.0;
        let r0 = regime(&f(lo));
        let (last, first, _) = bisect(lo, hi, false, r0, &f);
        println!("      -> R=100 touch-vanishes at gap ~ {:.3e} .. {:.3e}", last, first);
    }

    // --- B.2 min feature: sphere R=1 ∩ plane, shrink circle radius rho ---
    println!("[B.2] sphere R=1 ∩ plane, target circle radius rho (curve->point->empty as rho->0)");
    for &rho in &[1e-2, 1e-4, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12] {
        println!("      rho={:.1e} -> {:?}", rho, sphere_plane_circle(1.0, rho));
    }
    {
        let f = |rho: f64| sphere_plane_circle(1.0, rho);
        // large rho -> Curve; small -> Point/Empty. bisect Curve boundary.
        let (last, first, endr) = bisect(1e-2, 1e-13, true, Regime::Curve, &f);
        println!("      -> smallest resolvable circle rho ~ {:.3e} (below-> {:.3e}, {:?})", last, first, endr);
    }

    // --- B.3 angular coincidence: two planes through origin, tilt theta ---
    println!("[B.3] two planes through O, B tilted theta about X (line->empty as theta->0)");
    for &t in &[1e-2, 1e-5, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 1e-13, 1e-14] {
        println!("      theta={:.1e} -> {:?}", t, two_planes_angle(t));
    }
    {
        let f = |t: f64| two_planes_angle(t);
        let (last, first, endr) = bisect(1e-2, 1e-15, true, Regime::Curve, &f);
        println!("      -> smallest theta giving a line ~ {:.3e} (below-> {:.3e}, {:?})", last, first, endr);
    }

    // --- B.4 parallel planes gap (linear coincidence observability) ---
    println!("[B.4] two parallel planes, gap g along Z (observe coincidence signal)");
    for &g in &[1.0, 1e-6, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12, 0.0] {
        println!("      g={:.1e} -> {:?}", g, two_planes_gap(g));
    }
}

fn main() {
    let cfg = SessionConfig::new().check_arguments(false).frustrum(
        FrustrumConfig::new().base_dir(
            "/tmp/claude-1000/-home-dev-projects-parasolid-re/ccf7881f-5d90-471c-9b56-208ecdb81733/scratchpad/xt",
        ),
    );
    let s = Session::start(cfg).unwrap();

    // ===== ROUTE A: direct session precision defaults =====
    println!("==== ROUTE A: direct session precision ====");
    println!("default linear  precision (PK_SESSION_ask_precision)       = {:?}", s.precision());
    println!("default angular precision (PK_SESSION_ask_angle_precision) = {:?}", s.angle_precision());

    // ===== ROUTE B at default precision =====
    run_battery("default precision");

    // ===== ROUTE B while varying session precision to test T-dependence =====
    for &p in &[1e-6_f64, 1e-4, 1e-10] {
        // set_precision is on SessionConfig at start; use the sys call directly via a fresh probe.
        unsafe {
            let rc = parasolid_sys::PK_SESSION_set_precision(p);
            println!("\n[set_precision({:.0e}) rc={}]", p, rc);
        }
        let mut got = 0.0f64;
        unsafe { parasolid_sys::PK_SESSION_ask_precision(&mut got); }
        println!("  ask_precision now = {:.3e}", got);
        run_battery(&format!("linear precision = {:.0e}", p));
    }

    // restore
    unsafe { parasolid_sys::PK_SESSION_set_precision(1e-8); }
    println!("\n==== DONE ====");
}
