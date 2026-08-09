//! ADVERSARIAL REVIEW PROBE — Stage 1 numeric contract.
//!
//! Attacks the blanket claim "create->ask is bit-exact" across every analytic
//! family and every field, plus precision independence and degenerate-input
//! rejection.
//!
//!   cargo build -p parasolid-test --bin review_numerics_probe --target x86_64-pc-windows-gnu
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/review_numerics_probe.exe

use parasolid::*;
use parasolid_sys::*;

fn h(v: f64) -> String {
    format!("{:016x}", v.to_bits())
}

/// Compare authored vs read-back; print PASS/FAIL with bit patterns.
fn cmp(label: &str, want: f64, got: f64) -> bool {
    let ok = want.to_bits() == got.to_bits();
    if ok {
        println!("    ok   {label:<34} {want:+.17e} [{}]", h(want));
    } else {
        let ulp = (want.to_bits() as i64 - got.to_bits() as i64).abs();
        println!(
            "    FAIL {label:<34} want {want:+.17e} [{}]  got {got:+.17e} [{}]  dbits={ulp}",
            h(want),
            h(got)
        );
    }
    ok
}

fn cmp3(label: &str, want: Vec3, got: Vec3) -> bool {
    let a = cmp(&format!("{label}.x"), want.x, got.x);
    let b = cmp(&format!("{label}.y"), want.y, got.y);
    let c = cmp(&format!("{label}.z"), want.z, got.z);
    a && b && c
}

fn cmp_axis2(label: &str, want: Axis2, got: Axis2) -> bool {
    let a = cmp3(&format!("{label}.origin"), want.origin, got.origin);
    let b = cmp3(&format!("{label}.axis"), want.axis, got.axis);
    let c = cmp3(&format!("{label}.ref_dir"), want.ref_direction, got.ref_direction);
    a && b && c
}

fn tok(e: &PsError) -> String {
    match e.details().and_then(|d| d.code_token.clone()) {
        Some(t) => t,
        None => format!("<no token> ({e})"),
    }
}

// =============================================================================

fn banner(s: &str) {
    println!("\n================ {s} ================");
}

/// Experiment 1: every family, every field, with a fully-populated oblique
/// basis whose axis and ref_direction are UNIT and PERPENDICULAR (so any
/// renormalisation is a no-op mathematically but may still perturb bits).
fn exp1_oblique_all_families() {
    banner("EXP1  all families, oblique orthonormal basis, all fields");

    // An oblique orthonormal frame built from irrational-ish components.
    // axis = normalize(1,2,3); ref = normalize(component of (1,0,0) ⟂ axis)
    let a = {
        let (x, y, z) = (1.0f64, 2.0, 3.0);
        let n = (x * x + y * y + z * z).sqrt();
        Vec3::new(x / n, y / n, z / n)
    };
    let r = {
        let d = a.x; // dot((1,0,0), a)
        let (x, y, z) = (1.0 - d * a.x, -d * a.y, -d * a.z);
        let n = (x * x + y * y + z * z).sqrt();
        Vec3::new(x / n, y / n, z / n)
    };
    let o = Vec3::new(1.234_567_890_123_456_7, -9.876_543_210_987_654, 0.1);
    let basis = Axis2::new(o, a, r);
    println!(
        "  axis   = ({:.17e},{:.17e},{:.17e})  [{} {} {}]",
        a.x,
        a.y,
        a.z,
        h(a.x),
        h(a.y),
        h(a.z)
    );
    println!(
        "  refdir = ({:.17e},{:.17e},{:.17e})  [{} {} {}]",
        r.x,
        r.y,
        r.z,
        h(r.x),
        h(r.y),
        h(r.z)
    );

    let rad = 3.700_000_000_000_000_4_f64;

    println!("  -- PLANE");
    match Surf::plane(basis) {
        Ok(s) => {
            let d = s.ask_plane().unwrap();
            cmp_axis2("plane", basis, d.basis);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- CYLINDER");
    match Surf::cylinder(basis, rad) {
        Ok(s) => {
            let d = s.ask_cylinder().unwrap();
            cmp("cyl.radius", rad, d.radius);
            cmp_axis2("cyl", basis, d.basis);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- CONE");
    let semi = 0.523_598_775_598_298_9_f64; // ~30deg, non-dyadic
    match Surf::cone(basis, rad, semi) {
        Ok(s) => {
            let d = s.ask_cone().unwrap();
            cmp("cone.radius", rad, d.radius);
            cmp("cone.semi_angle", semi, d.semi_angle);
            cmp_axis2("cone", basis, d.basis);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- SPHERE");
    match Surf::sphere(basis, rad) {
        Ok(s) => {
            let d = s.ask_sphere().unwrap();
            cmp("sph.radius", rad, d.radius);
            cmp_axis2("sph", basis, d.basis);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- TORUS");
    let (maj, min) = (5.100_000_000_000_000_5_f64, 1.234_567_890_123_456_7_f64);
    match Surf::torus(basis, maj, min) {
        Ok(s) => {
            let d = s.ask_torus().unwrap();
            cmp("tor.major", maj, d.major_radius);
            cmp("tor.minor", min, d.minor_radius);
            cmp_axis2("tor", basis, d.basis);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- CIRCLE");
    match Curve::circle(basis, rad) {
        Ok(c) => {
            let d = c.ask_circle().unwrap();
            cmp("cir.radius", rad, d.radius);
            cmp_axis2("cir", basis, d.basis);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- ELLIPSE");
    let (r1, r2) = (7.700_000_000_000_000_7_f64, 2.100_000_000_000_000_1_f64);
    match Curve::ellipse(basis, r1, r2) {
        Ok(c) => {
            let d = c.ask_ellipse().unwrap();
            cmp("ell.R1", r1, d.r1);
            cmp("ell.R2", r2, d.r2);
            cmp_axis2("ell", basis, d.basis);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- LINE (unit direction)");
    match Curve::line(o, a) {
        Ok(c) => {
            let d = c.ask_line().unwrap();
            cmp3("line.origin", o, d.origin);
            cmp3("line.direction", a, d.direction);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }

    println!("  -- POINT");
    match Point::create(o) {
        Ok(p) => {
            let d = p.position().unwrap();
            cmp3("point", o, d);
        }
        Err(e) => println!("    ERR {}", tok(&e)),
    }
}

/// Experiment 2: NON-unit axis / ref_direction, and a ref_direction that is not
/// perpendicular to the axis. Does the kernel renormalise / re-orthogonalise?
fn exp2_nonunit_nonperp() {
    banner("EXP2  non-unit axis, non-perpendicular ref_direction");

    let o = Vec3::zero();

    println!("  -- axis (0,0,2), ref (3,0,0)  [both non-unit, perpendicular]");
    let b = Axis2::new(o, Vec3::new(0.0, 0.0, 2.0), Vec3::new(3.0, 0.0, 0.0));
    match Surf::sphere(b, 1.0) {
        Ok(s) => {
            let d = s.ask_sphere().unwrap();
            println!(
                "     axis back = ({},{},{})  ref back = ({},{},{})",
                d.basis.axis.x,
                d.basis.axis.y,
                d.basis.axis.z,
                d.basis.ref_direction.x,
                d.basis.ref_direction.y,
                d.basis.ref_direction.z
            );
            cmp_axis2("nonunit", b, d.basis);
        }
        Err(e) => println!("     REJECTED {}", tok(&e)),
    }

    println!("  -- axis (0,0,1), ref (1,0,0.5)  [ref NOT perpendicular]");
    let b = Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.5));
    match Surf::sphere(b, 1.0) {
        Ok(s) => {
            let d = s.ask_sphere().unwrap();
            println!(
                "     ref back = ({:.17e},{:.17e},{:.17e}) [{} {} {}]",
                d.basis.ref_direction.x,
                d.basis.ref_direction.y,
                d.basis.ref_direction.z,
                h(d.basis.ref_direction.x),
                h(d.basis.ref_direction.y),
                h(d.basis.ref_direction.z)
            );
            cmp_axis2("nonperp", b, d.basis);
        }
        Err(e) => println!("     REJECTED {}", tok(&e)),
    }

    println!("  -- axis (0,0,1), ref (0,0,1)  [ref PARALLEL to axis]");
    let b = Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 1.0));
    match Surf::sphere(b, 1.0) {
        Ok(s) => {
            let d = s.ask_sphere().unwrap();
            println!(
                "     ACCEPTED; ref back = ({},{},{})",
                d.basis.ref_direction.x, d.basis.ref_direction.y, d.basis.ref_direction.z
            );
        }
        Err(e) => println!("     REJECTED {}", tok(&e)),
    }

    println!("  -- axis (0,0,1), ref (0,0,0)  [ref ZERO]");
    let b = Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 0.0));
    match Surf::sphere(b, 1.0) {
        Ok(s) => {
            let d = s.ask_sphere().unwrap();
            println!(
                "     ACCEPTED; ref back = ({},{},{})",
                d.basis.ref_direction.x, d.basis.ref_direction.y, d.basis.ref_direction.z
            );
        }
        Err(e) => println!("     REJECTED {}", tok(&e)),
    }

    println!("  -- LINE with non-unit direction (0,0,7)");
    match Curve::line(o, Vec3::new(0.0, 0.0, 7.0)) {
        Ok(c) => {
            let d = c.ask_line().unwrap();
            println!(
                "     dir back = ({},{},{}) [{} {} {}]",
                d.direction.x,
                d.direction.y,
                d.direction.z,
                h(d.direction.x),
                h(d.direction.y),
                h(d.direction.z)
            );
        }
        Err(e) => println!("     REJECTED {}", tok(&e)),
    }

    println!("  -- LINE with oblique non-unit direction (1,2,3)");
    let dir = Vec3::new(1.0, 2.0, 3.0);
    match Curve::line(o, dir) {
        Ok(c) => {
            let d = c.ask_line().unwrap();
            let n = (14.0f64).sqrt();
            let expect = Vec3::new(1.0 / n, 2.0 / n, 3.0 / n);
            println!(
                "     dir back = ({:.17e},{:.17e},{:.17e}) [{} {} {}]",
                d.direction.x,
                d.direction.y,
                d.direction.z,
                h(d.direction.x),
                h(d.direction.y),
                h(d.direction.z)
            );
            println!("     naive normalize would be:");
            cmp3("line.dir(vs naive norm)", expect, d.direction);
        }
        Err(e) => println!("     REJECTED {}", tok(&e)),
    }

    println!("  -- LINE with already-unit oblique direction (round-trip stability)");
    let n = (14.0f64).sqrt();
    let u = Vec3::new(1.0 / n, 2.0 / n, 3.0 / n);
    match Curve::line(o, u) {
        Ok(c) => {
            let d = c.ask_line().unwrap();
            cmp3("line.unitdir", u, d.direction);
        }
        Err(e) => println!("     REJECTED {}", tok(&e)),
    }
}

/// Experiment 3: adversarial f64 magnitudes for the scalar fields.
fn exp3_adversarial_scalars() {
    banner("EXP3  adversarial f64 scalar values");

    let basis = Axis2::new(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    let cases: &[(&str, f64)] = &[
        ("MIN_POSITIVE", f64::MIN_POSITIVE),
        ("subnormal 5e-324", f64::from_bits(1)),
        ("1e-300", 1e-300),
        ("1e-12", 1e-12),
        ("1e-8", 1e-8),
        ("0.1", 0.1),
        ("1.0+1ulp", f64::from_bits(1.0f64.to_bits() + 1)),
        ("1e8", 1e8),
        ("1e15", 1e15),
        ("1e100", 1e100),
        ("MAX", f64::MAX),
    ];

    for (name, r) in cases {
        print!("  radius {name:<18} ");
        match Surf::sphere(basis, *r) {
            Ok(s) => {
                let d = s.ask_sphere().unwrap();
                let ok = d.radius.to_bits() == r.to_bits();
                println!(
                    "sphere {} [{} -> {}]",
                    if ok { "bit-exact" } else { "*** DIFFERS ***" },
                    h(*r),
                    h(d.radius)
                );
            }
            Err(e) => println!("sphere REJECTED {}", tok(&e)),
        }
    }

    println!("  -- origin components (point, purest path)");
    for (name, v) in cases {
        match Point::create(Vec3::new(*v, -*v, 0.0)) {
            Ok(p) => {
                let d = p.position().unwrap();
                let ok = d.x.to_bits() == v.to_bits() && d.y.to_bits() == (-*v).to_bits();
                println!(
                    "    point {name:<18} {} [{} -> {}]",
                    if ok { "bit-exact" } else { "*** DIFFERS ***" },
                    h(*v),
                    h(d.x)
                );
            }
            Err(e) => println!("    point {name:<18} REJECTED {}", tok(&e)),
        }
    }

    println!("  -- negative zero in point / sphere origin");
    match Point::create(Vec3::new(-0.0, 0.0, -0.0)) {
        Ok(p) => {
            let d = p.position().unwrap();
            println!(
                "    point(-0,0,-0) -> x bits {} y bits {} z bits {} (neg zero = 8000000000000000)",
                h(d.x),
                h(d.y),
                h(d.z)
            );
        }
        Err(e) => println!("    REJECTED {}", tok(&e)),
    }
    let nzb = Axis2::new(
        Vec3::new(-0.0, -0.0, -0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    match Surf::sphere(nzb, 1.0) {
        Ok(s) => {
            let d = s.ask_sphere().unwrap();
            println!(
                "    sphere origin(-0,-0,-0) -> {} {} {}",
                h(d.basis.origin.x),
                h(d.basis.origin.y),
                h(d.basis.origin.z)
            );
        }
        Err(e) => println!("    REJECTED {}", tok(&e)),
    }

    println!("  -- cone semi_angle sweep");
    for (name, sa) in [
        ("1e-12", 1e-12f64),
        ("1e-8", 1e-8),
        ("1e-6", 1e-6),
        ("0.1", 0.1),
        ("pi/4", std::f64::consts::FRAC_PI_4),
        ("pi/2-1ulp", f64::from_bits(std::f64::consts::FRAC_PI_2.to_bits() - 1)),
        ("pi/2", std::f64::consts::FRAC_PI_2),
        ("pi/2+1ulp", f64::from_bits(std::f64::consts::FRAC_PI_2.to_bits() + 1)),
        ("2.0 (>pi/2)", 2.0),
        ("0.0", 0.0),
        ("-0.3", -0.3),
        ("3.5 (>pi)", 3.5),
    ] {
        print!("    semi_angle {name:<12} ");
        match Surf::cone(basis, 1.0, sa) {
            Ok(s) => {
                let d = s.ask_cone().unwrap();
                let ok = d.semi_angle.to_bits() == sa.to_bits();
                println!(
                    "OK  {} back={:.17e} [{} -> {}]",
                    if ok { "bit-exact" } else { "*** DIFFERS ***" },
                    d.semi_angle,
                    h(sa),
                    h(d.semi_angle)
                );
            }
            Err(e) => println!("REJECTED {}", tok(&e)),
        }
    }

    println!("  -- torus major/minor relationships");
    for (name, maj, min) in [
        ("min<maj", 5.0f64, 1.0f64),
        ("min==maj", 5.0, 5.0),
        ("min>maj (lemon)", 1.0, 5.0),
        ("maj=0", 0.0, 1.0),
        ("maj<0", -5.0, 1.0),
        ("min=0", 5.0, 0.0),
        ("min<0", 5.0, -1.0),
        ("min=maj-1ulp", 5.0, f64::from_bits(5.0f64.to_bits() - 1)),
    ] {
        print!("    torus {name:<18} ");
        match Surf::torus(basis, maj, min) {
            Ok(s) => {
                let d = s.ask_torus().unwrap();
                let ok = d.major_radius.to_bits() == maj.to_bits()
                    && d.minor_radius.to_bits() == min.to_bits();
                println!(
                    "OK {} (back {} / {})",
                    if ok { "bit-exact" } else { "*** DIFFERS ***" },
                    d.major_radius,
                    d.minor_radius
                );
            }
            Err(e) => println!("REJECTED {}", tok(&e)),
        }
    }

    println!("  -- ellipse R1/R2 relationships");
    for (name, r1, r2) in [
        ("R1>R2", 7.0f64, 2.0f64),
        ("R1==R2", 3.0, 3.0),
        ("R2>R1", 2.0, 7.0),
        ("R1=0", 0.0, 2.0),
        ("R2=0", 2.0, 0.0),
        ("R2<0", 2.0, -1.0),
    ] {
        print!("    ellipse {name:<16} ");
        match Curve::ellipse(basis, r1, r2) {
            Ok(c) => {
                let d = c.ask_ellipse().unwrap();
                let ok = d.r1.to_bits() == r1.to_bits() && d.r2.to_bits() == r2.to_bits();
                println!(
                    "OK {} (back R1={} R2={})",
                    if ok {
                        "bit-exact, NOT reordered"
                    } else {
                        "*** DIFFERS/REORDERED ***"
                    },
                    d.r1,
                    d.r2
                );
            }
            Err(e) => println!("REJECTED {}", tok(&e)),
        }
    }
}

/// Experiment 4: NaN / infinity — rejected, or silently stored?
fn exp4_nan_inf(check_args: bool) {
    banner(&format!("EXP4  NaN / Inf inputs (check_arguments = {check_args})"));

    let basis = Axis2::new(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    for (name, v) in [
        ("NaN", f64::NAN),
        ("+Inf", f64::INFINITY),
        ("-Inf", f64::NEG_INFINITY),
    ] {
        print!("  sphere radius = {name:<6} ");
        match Surf::sphere(basis, v) {
            Ok(s) => match s.ask_sphere() {
                Ok(d) => println!(
                    "*** ACCEPTED *** readback radius = {} [{}]  is_nan={} is_inf={}",
                    d.radius,
                    h(d.radius),
                    d.radius.is_nan(),
                    d.radius.is_infinite()
                ),
                Err(e) => println!("created but ask failed: {}", tok(&e)),
            },
            Err(e) => println!("rejected {}", tok(&e)),
        }

        print!("  point coord   = {name:<6} ");
        match Point::create(Vec3::new(v, 0.0, 0.0)) {
            Ok(p) => match p.position() {
                Ok(d) => println!(
                    "*** ACCEPTED *** readback x = {} [{}] is_nan={}",
                    d.x,
                    h(d.x),
                    d.x.is_nan()
                ),
                Err(e) => println!("created but ask failed: {}", tok(&e)),
            },
            Err(e) => println!("rejected {}", tok(&e)),
        }

        print!("  plane axis    = ({name},0,1) ");
        let b = Axis2::new(
            Vec3::zero(),
            Vec3::new(v, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        match Surf::plane(b) {
            Ok(s) => match s.ask_plane() {
                Ok(d) => println!(
                    "*** ACCEPTED *** axis back = ({},{},{})",
                    d.basis.axis.x, d.basis.axis.y, d.basis.axis.z
                ),
                Err(e) => println!("created but ask failed: {}", tok(&e)),
            },
            Err(e) => println!("rejected {}", tok(&e)),
        }

        print!("  sphere origin = ({name},0,0) ");
        let b = Axis2::new(
            Vec3::new(v, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        match Surf::sphere(b, 1.0) {
            Ok(s) => match s.ask_sphere() {
                Ok(d) => println!(
                    "*** ACCEPTED *** origin.x back = {} [{}] is_nan={}",
                    d.basis.origin.x,
                    h(d.basis.origin.x),
                    d.basis.origin.x.is_nan()
                ),
                Err(e) => println!("created but ask failed: {}", tok(&e)),
            },
            Err(e) => println!("rejected {}", tok(&e)),
        }

        print!("  cone semi_ang = {name:<6} ");
        match Surf::cone(basis, 1.0, v) {
            Ok(s) => match s.ask_cone() {
                Ok(d) => println!("*** ACCEPTED *** semi_angle back = {}", d.semi_angle),
                Err(e) => println!("created but ask failed: {}", tok(&e)),
            },
            Err(e) => println!("rejected {}", tok(&e)),
        }
        println!();
    }
}

/// Experiment 5: degenerate radii per family — which error token exactly?
fn exp5_degenerate_tokens() {
    banner("EXP5  degenerate-input rejection tokens per family");

    let basis = Axis2::new(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let nullaxis = Axis2::new(Vec3::zero(), Vec3::zero(), Vec3::new(1.0, 0.0, 0.0));

    macro_rules! p {
        ($label:expr, $e:expr) => {
            match $e {
                Ok(_) => println!("  {:<38} ACCEPTED (no error)", $label),
                Err(e) => println!("  {:<38} {}", $label, tok(&e)),
            }
        };
    }

    p!("sphere r=0", Surf::sphere(basis, 0.0).map(|_| ()));
    p!("sphere r=-1", Surf::sphere(basis, -1.0).map(|_| ()));
    p!("sphere r=1e-300", Surf::sphere(basis, 1e-300).map(|_| ()));
    p!("cyl r=0", Surf::cylinder(basis, 0.0).map(|_| ()));
    p!("cone r=0", Surf::cone(basis, 0.0, 0.5).map(|_| ()));
    p!("cone r=-1", Surf::cone(basis, -1.0, 0.5).map(|_| ()));
    p!("circle r=0", Curve::circle(basis, 0.0).map(|_| ()));
    p!("ellipse R1=0", Curve::ellipse(basis, 0.0, 1.0).map(|_| ()));
    p!("ellipse R2=0", Curve::ellipse(basis, 1.0, 0.0).map(|_| ()));
    p!("torus maj=0", Surf::torus(basis, 0.0, 1.0).map(|_| ()));
    p!("torus min=0", Surf::torus(basis, 1.0, 0.0).map(|_| ()));
    p!("plane null axis", Surf::plane(nullaxis).map(|_| ()));
    p!("sphere null axis", Surf::sphere(nullaxis, 1.0).map(|_| ()));
    p!("line zero direction", Curve::line(Vec3::zero(), Vec3::zero()).map(|_| ()));
    p!(
        "circle ref parallel to axis",
        Curve::circle(
            Axis2::new(
                Vec3::zero(),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(0.0, 0.0, 1.0)
            ),
            1.0
        )
        .map(|_| ())
    );
}

/// Experiment 6: precision set / readback / clamping.
fn exp6_precision(session: &Session) {
    banner("EXP6  PK_SESSION_set_precision / ask_precision");

    let default_linear = session.precision().unwrap();
    let default_angular = session.angle_precision().unwrap();
    println!(
        "  default linear  = {:.17e} [{}]",
        default_linear,
        h(default_linear)
    );
    println!(
        "  default angular = {:.17e} [{}]",
        default_angular,
        h(default_angular)
    );

    for req in [
        1.0e-2f64,
        1.0e-3,
        1.0e-5,
        1.0e-7,
        1.0e-9,
        1.0e-12,
        1.0e-15,
        1.0e-18,
        1.0e-300,
        f64::MIN_POSITIVE,
        1.0,
        1.0e3,
        1.0e30,
    ] {
        let code = unsafe { PK_SESSION_set_precision(req) };
        let actual = session.precision().unwrap();
        let exact = actual.to_bits() == req.to_bits();
        println!(
            "  set {req:>12.3e} -> code {code:<6} readback {:.17e} [{}] {}",
            actual,
            h(actual),
            if exact { "EXACT" } else { "*** CLAMPED/CHANGED ***" }
        );
        unsafe { PK_SESSION_set_precision(default_linear) };
    }

    println!("  -- invalid values");
    for (name, req) in [
        ("0.0", 0.0f64),
        ("-1e-6", -1e-6),
        ("NaN", f64::NAN),
        ("+Inf", f64::INFINITY),
        ("-Inf", f64::NEG_INFINITY),
    ] {
        let code = unsafe { PK_SESSION_set_precision(req) };
        let actual = session.precision().unwrap();
        println!(
            "  set {name:<6} -> code {code:<8} readback {:.17e} [{}] {}",
            actual,
            h(actual),
            if actual.to_bits() == default_linear.to_bits() {
                "(unchanged)"
            } else {
                "*** SESSION PRECISION MUTATED ***"
            }
        );
        unsafe { PK_SESSION_set_precision(default_linear) };
    }

    println!("  -- angle precision");
    for (name, req) in [
        ("1e-3", 1e-3f64),
        ("1e-6", 1e-6),
        ("1e-12", 1e-12),
        ("0.0", 0.0),
        ("-1.0", -1.0),
        ("NaN", f64::NAN),
        ("3.2 (>pi)", 3.2),
        ("1e30", 1e30),
    ] {
        let code = unsafe { PK_SESSION_set_angle_precision(req) };
        let actual = session.angle_precision().unwrap();
        println!(
            "  set_angle {name:<10} -> code {code:<8} readback {:.17e} [{}] {}",
            actual,
            h(actual),
            if actual.to_bits() == req.to_bits() {
                "EXACT"
            } else {
                "changed/clamped"
            }
        );
        unsafe { PK_SESSION_set_angle_precision(default_angular) };
    }

    // Restore.
    unsafe {
        PK_SESSION_set_precision(default_linear);
        PK_SESSION_set_angle_precision(default_angular);
    }
}

/// Experiment 7: does session precision affect construction/storage?
fn exp7_precision_independence(session: &Session) {
    banner("EXP7  geometry vs session precision (extremes + near-degenerate)");

    let default_linear = session.precision().unwrap();
    let basis = Axis2::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    let precisions = [1.0e-2f64, 1.0e-5, 1.0e-9, 1.0e-12];

    // Reference values at default precision.
    println!("  family / precision -> bits (all must match across the row)");

    // sphere with tiny radius
    for (name, r) in [("sphere r=1e-6", 1e-6f64), ("sphere r=1e-9", 1e-9)] {
        print!("    {name:<18}");
        for p in precisions {
            unsafe { PK_SESSION_set_precision(p) };
            match Surf::sphere(basis, r) {
                Ok(s) => print!(" {}", h(s.ask_sphere().unwrap().radius)),
                Err(e) => print!(" ERR:{}", tok(&e)),
            }
        }
        println!();
    }

    // cone with tiny semi angle
    for (name, sa) in [("cone sa=1e-9", 1e-9f64), ("cone sa=1e-14", 1e-14)] {
        print!("    {name:<18}");
        for p in precisions {
            unsafe { PK_SESSION_set_precision(p) };
            match Surf::cone(basis, 1.0, sa) {
                Ok(s) => print!(" {}", h(s.ask_cone().unwrap().semi_angle)),
                Err(e) => print!(" ERR:{}", tok(&e)),
            }
        }
        println!();
    }

    // torus minor ~= major
    print!("    {:<18}", "torus 5/4.9999999");
    for p in precisions {
        unsafe { PK_SESSION_set_precision(p) };
        match Surf::torus(basis, 5.0, 4.9999999) {
            Ok(s) => print!(" {}", h(s.ask_torus().unwrap().minor_radius)),
            Err(e) => print!(" ERR:{}", tok(&e)),
        }
    }
    println!();

    // near-degenerate ellipse
    print!("    {:<18}", "ellipse 5/1e-9");
    for p in precisions {
        unsafe { PK_SESSION_set_precision(p) };
        match Curve::ellipse(basis, 5.0, 1e-9) {
            Ok(c) => print!(" {}", h(c.ask_ellipse().unwrap().r2)),
            Err(e) => print!(" ERR:{}", tok(&e)),
        }
    }
    println!();

    // Does changing precision AFTER build change the readback?
    unsafe { PK_SESSION_set_precision(1e-9) };
    let s = Surf::sphere(basis, 2.718_281_828_459_045).unwrap();
    let before = s.ask_sphere().unwrap();
    unsafe { PK_SESSION_set_precision(1e-2) };
    let after = s.ask_sphere().unwrap();
    println!(
        "  same tag re-asked after precision 1e-9 -> 1e-2: radius {} vs {} ({})",
        h(before.radius),
        h(after.radius),
        if before.radius.to_bits() == after.radius.to_bits() {
            "stable"
        } else {
            "*** MUTATED ***"
        }
    );

    unsafe { PK_SESSION_set_precision(default_linear) };
}

/// Experiment 8: check_arguments actually in force, and effect of turning off.
fn exp8_check_arguments(session: &Session) {
    banner("EXP8  check_arguments readback and effect");

    println!("  check_arguments readback = {:?}", session.check_arguments());
}

/// Experiment 9: characterise the cone semi_angle round-trip loss.
fn exp9_cone_semi_angle_sweep() {
    banner("EXP9  cone semi_angle round-trip — how often is it NOT bit-exact?");

    let basis = Axis2::new(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    let mut n = 0usize;
    let mut bad = 0usize;
    let mut worst = 0i64;
    let mut examples: Vec<(f64, f64)> = Vec::new();

    // Dense sweep over the legal open interval (0, pi/2).
    for k in 1..=2000 {
        let sa = (k as f64) * (std::f64::consts::FRAC_PI_2 / 2001.0);
        if let Ok(s) = Surf::cone(basis, 1.0, sa) {
            let got = s.ask_cone().unwrap().semi_angle;
            n += 1;
            let d = (sa.to_bits() as i64) - (got.to_bits() as i64);
            if d != 0 {
                bad += 1;
                if d.abs() > worst {
                    worst = d.abs();
                }
                if examples.len() < 6 {
                    examples.push((sa, got));
                }
            }
        }
    }
    println!("  dense sweep: {bad} of {n} semi_angles NOT bit-exact; worst |dbits| = {worst}");
    for (a, b) in &examples {
        println!(
            "    in  {:.17e} [{}]   out {:.17e} [{}]",
            a,
            h(*a),
            b,
            h(*b)
        );
    }

    // Round-trip stability: is ask(create(ask(x))) a fixed point?
    println!("  -- fixed-point check on 0.1");
    let mut v = 0.1f64;
    for i in 0..40 {
        let s = Surf::cone(basis, 1.0, v).unwrap();
        let out = s.ask_cone().unwrap().semi_angle;
        if i % 5 == 0 || i > 34 { println!("    iter {i}: {} -> {} ({:.17e})", h(v), h(out), out); }
        v = out;
    }

    // Does the cone RADIUS survive when the angle does not?
    println!("  -- cone radius exactness alongside a lossy angle");
    let r = 3.700_000_000_000_000_4_f64;
    let s = Surf::cone(basis, r, 0.1).unwrap();
    let d = s.ask_cone().unwrap();
    cmp("cone.radius (angle=0.1)", r, d.radius);
    cmp("cone.semi_angle", 0.1, d.semi_angle);

    // Same test for a handful of "human" angles CADabra would plausibly author.
    println!("  -- human angles");
    for (name, sa) in [
        ("0.1", 0.1f64),
        ("0.2", 0.2),
        ("0.3", 0.3),
        ("0.5", 0.5),
        ("1.0", 1.0),
        ("1.5", 1.5),
        ("15deg", 15.0f64.to_radians()),
        ("30deg", 30.0f64.to_radians()),
        ("45deg", 45.0f64.to_radians()),
        ("60deg", 60.0f64.to_radians()),
        ("75deg", 75.0f64.to_radians()),
    ] {
        match Surf::cone(basis, 1.0, sa) {
            Ok(s) => {
                let got = s.ask_cone().unwrap().semi_angle;
                let d = (sa.to_bits() as i64) - (got.to_bits() as i64);
                println!(
                    "    {name:<8} {} -> {}  dbits={d} {}",
                    h(sa),
                    h(got),
                    if d == 0 { "exact" } else { "*** LOSSY ***" }
                );
            }
            Err(e) => println!("    {name:<8} REJECTED {}", tok(&e)),
        }
    }
}

/// Experiment 10: unit-length ref_direction that is NOT perpendicular.
fn exp10_unit_nonperp_ref() {
    banner("EXP10  UNIT ref_direction not perpendicular to axis");

    let s = std::f64::consts::FRAC_1_SQRT_2;
    for (name, axis, refd) in [
        ("ref 45deg off axis", Vec3::new(0.0, 0.0, 1.0), Vec3::new(s, 0.0, s)),
        (
            "ref 1e-9 off perpendicular",
            Vec3::new(0.0, 0.0, 1.0),
            {
                let e = 1e-9f64;
                let n = (1.0 + e * e).sqrt();
                Vec3::new(1.0 / n, 0.0, e / n)
            },
        ),
        (
            "ref 1e-13 off perpendicular",
            Vec3::new(0.0, 0.0, 1.0),
            {
                let e = 1e-13f64;
                let n = (1.0 + e * e).sqrt();
                Vec3::new(1.0 / n, 0.0, e / n)
            },
        ),
    ] {
        let b = Axis2::new(Vec3::zero(), axis, refd);
        print!("  {name:<30} ");
        match Surf::sphere(b, 1.0) {
            Ok(sf) => {
                let d = sf.ask_sphere().unwrap();
                let same = d.basis.ref_direction.x.to_bits() == refd.x.to_bits()
                    && d.basis.ref_direction.y.to_bits() == refd.y.to_bits()
                    && d.basis.ref_direction.z.to_bits() == refd.z.to_bits();
                println!(
                    "ACCEPTED, ref back {} ({},{},{})",
                    if same { "bit-identical" } else { "*** RE-ORTHOGONALISED ***" },
                    h(d.basis.ref_direction.x),
                    h(d.basis.ref_direction.y),
                    h(d.basis.ref_direction.z)
                );
            }
            Err(e) => println!("REJECTED {}", tok(&e)),
        }
    }

    // Non-unit-by-1-ulp axis: how tight is the unit-vector check?
    println!("  -- how tight is the unit-vector check on the axis?");
    for (name, ax) in [
        ("1+1ulp", f64::from_bits(1.0f64.to_bits() + 1)),
        ("1+1e-12", 1.0 + 1e-12),
        ("1+1e-9", 1.0 + 1e-9),
        ("1+1e-7", 1.0 + 1e-7),
        ("1+1e-5", 1.0 + 1e-5),
    ] {
        let b = Axis2::new(
            Vec3::zero(),
            Vec3::new(0.0, 0.0, ax),
            Vec3::new(1.0, 0.0, 0.0),
        );
        print!("    axis z = {name:<10} ");
        match Surf::sphere(b, 1.0) {
            Ok(sf) => {
                let d = sf.ask_sphere().unwrap();
                println!(
                    "ACCEPTED, axis.z back = {} ({})",
                    h(d.basis.axis.z),
                    if d.basis.axis.z.to_bits() == ax.to_bits() {
                        "stored verbatim, NOT normalised"
                    } else {
                        "*** NORMALISED ***"
                    }
                );
            }
            Err(e) => println!("REJECTED {}", tok(&e)),
        }
    }
}

/// Experiment 11: pin the precision-dependent minimum radius, and the
/// size limits for point vs sphere-origin.
fn exp11_thresholds(session: &Session) {
    banner("EXP11  precision-dependent minimum radius; positional size limits");

    let default_linear = session.precision().unwrap();
    let basis = Axis2::new(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    for p in [1.0e-2f64, 1.0e-5, 1.0e-8, 1.0e-9, 1.0e-12] {
        unsafe { PK_SESSION_set_precision(p) };
        // Bisect the smallest accepted sphere radius.
        let (mut lo, mut hi) = (0.0f64, 1.0f64);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if mid <= lo || mid >= hi {
                break;
            }
            if Surf::sphere(basis, mid).is_ok() {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        println!(
            "  precision {p:>10.2e}  min accepted sphere radius ~= {hi:.6e}  (ratio r/prec = {:.4})",
            hi / p
        );
    }
    unsafe { PK_SESSION_set_precision(default_linear) };

    println!("  -- positional magnitude limits (default precision {default_linear:.2e})");
    for (name, mk) in [
        (
            "PK_POINT_create x",
            (|v: f64| Point::create(Vec3::new(v, 0.0, 0.0)).map(|_| ())) as fn(f64) -> PsResult<()>,
        ),
        ("sphere origin.x", |v: f64| {
            Surf::sphere(
                Axis2::new(
                    Vec3::new(v, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                1.0,
            )
            .map(|_| ())
        }),
        ("line location.x", |v: f64| {
            Curve::line(Vec3::new(v, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)).map(|_| ())
        }),
        ("circle origin.x", |v: f64| {
            Curve::circle(
                Axis2::new(
                    Vec3::new(v, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 1.0),
                    Vec3::new(1.0, 0.0, 0.0),
                ),
                1.0,
            )
            .map(|_| ())
        }),
    ] {
        let mut report = String::new();
        for e in [3i32, 5, 6, 7, 8, 9, 10, 12] {
            let v = 10f64.powi(e);
            report.push_str(&format!(
                "1e{e}:{} ",
                match mk(v) {
                    Ok(()) => "ok".to_string(),
                    Err(err) => tok(&err),
                }
            ));
        }
        println!("    {name:<20} {report}");
    }
}

fn exp12_point_limit(session: &Session) {
    banner("EXP12  PK_POINT_create positional limit");
    let default_linear = session.precision().unwrap();
    for p in [1.0e-2f64, 1.0e-8, 1.0e-12] {
        unsafe { PK_SESSION_set_precision(p) };
        let (mut lo, mut hi) = (0.0f64, 1.0e30f64);
        for _ in 0..300 {
            let mid = 0.5 * (lo + hi);
            if mid <= lo || mid >= hi { break; }
            if Point::create(Vec3::new(mid, 0.0, 0.0)).is_ok() { lo = mid; } else { hi = mid; }
        }
        println!("  precision {p:.1e}: max |x| accepted by PK_POINT_create ~= {lo:.9e} [{}]", h(lo));
        // and the same bound for sphere origin
        let (mut lo2, mut hi2) = (0.0f64, 1.0e300f64);
        for _ in 0..400 {
            let mid = 0.5 * (lo2 + hi2);
            if mid <= lo2 || mid >= hi2 { break; }
            let ok = Surf::sphere(Axis2::new(Vec3::new(mid,0.0,0.0), Vec3::new(0.0,0.0,1.0), Vec3::new(1.0,0.0,0.0)), 1.0).is_ok();
            if ok { lo2 = mid; } else { hi2 = mid; }
        }
        println!("  precision {p:.1e}: max |x| accepted as sphere origin ~= {lo2:.9e}");
    }
    unsafe { PK_SESSION_set_precision(default_linear) };
}

fn main() {
    println!("=== Stage 1 adversarial numeric probe ===");

    // --- Phase A: check_arguments(true), matching the test suite. ---
    {
        let session = Session::start(SessionConfig::new().check_arguments(true)).unwrap();
        println!(
            "\nsession started; check_arguments = {:?}",
            session.check_arguments()
        );
        exp1_oblique_all_families();
        exp2_nonunit_nonperp();
        exp3_adversarial_scalars();
        exp4_nan_inf(true);
        exp5_degenerate_tokens();
        exp6_precision(&session);
        exp7_precision_independence(&session);
        exp8_check_arguments(&session);
        exp9_cone_semi_angle_sweep();
        exp10_unit_nonperp_ref();
        exp11_thresholds(&session);
        exp12_point_limit(&session);
    }

    // --- Phase B: check_arguments(false). ---
    {
        let session = Session::start(SessionConfig::new().check_arguments(false)).unwrap();
        println!(
            "\n\n######## PHASE B: check_arguments = {:?} ########",
            session.check_arguments()
        );
        exp4_nan_inf(false);
        exp5_degenerate_tokens();
        exp2_nonunit_nonperp();
    }

    println!("\n=== probe complete ===");
}
