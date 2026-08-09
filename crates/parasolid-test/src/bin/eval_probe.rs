//! Derivative-array layout probe for `PK_SURF_eval` / `PK_CURVE_eval` (Stage 3).
//!
//! The wrapper currently only trusts first order, where the layout was pinned
//! on a sphere (`u` at slot 1, `v` at slot 2). Everything at order ≥ 2 — and
//! the whole `triangular` packing — is unverified, and a mis-indexed derivative
//! is exactly the sort of bug that produces plausible-but-wrong curvature and
//! tangency results downstream.
//!
//! Method: evaluate a surface whose derivatives are known in closed form, then
//! match each returned slot against the closed-form table. A cylinder gives
//! sharply distinguishable derivatives with all mixed partials zero; a torus
//! adds nonzero mixed partials so the (i,j) ordering cannot hide.
//!
//! Run under Wine:
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/eval_probe.exe

use parasolid::*;
use parasolid_sys::*;

const R: f64 = 2.0;

/// Closed-form ∂^(i+j) R / ∂u^i ∂v^j for a cylinder of radius `R`, axis +Z,
/// centred at the origin: R(u,v) = (R cos u, R sin u, v).
fn cyl_deriv(i: usize, j: usize, u: f64) -> Option<[f64; 3]> {
    // Any v-derivative beyond the first kills every term except dv itself.
    match (i, j) {
        (0, 0) => Some([R * u.cos(), R * u.sin(), 0.0]), // v added by caller
        (0, 1) => Some([0.0, 0.0, 1.0]),
        (_, 0) => {
            // d^i/du^i of (R cos u, R sin u, 0) cycles with period 4.
            let (c, s) = (u.cos(), u.sin());
            let v = match i % 4 {
                0 => [R * c, R * s, 0.0],
                1 => [-R * s, R * c, 0.0],
                2 => [-R * c, -R * s, 0.0],
                _ => [R * s, -R * c, 0.0],
            };
            Some(v)
        }
        // Mixed and higher v-derivatives all vanish for a cylinder.
        _ => Some([0.0, 0.0, 0.0]),
    }
}

/// Closed-form derivative for a torus, used only to expose mixed partials.
/// R(u,v) = ((MAJ + MIN cos v) cos u, (MAJ + MIN cos v) sin u, MIN sin v)
fn torus_deriv(i: usize, j: usize, u: f64, v: f64, maj: f64, min: f64) -> [f64; 3] {
    // d/du rotates the planar part; d/dv acts on the (maj + min cos v) radius
    // and the z term. Build by differentiating the two factors separately.
    let radial = |jj: usize| -> f64 {
        match jj % 4 {
            0 => maj + min * v.cos(),
            1 => -min * v.sin(),
            2 => -min * v.cos(),
            _ => min * v.sin(),
        }
    };
    // For j >= 1 the constant `maj` drops out.
    let rad = if j == 0 { radial(0) } else { radial(j) };
    let z = if i > 0 {
        0.0
    } else {
        match j % 4 {
            0 => min * v.sin(),
            1 => min * v.cos(),
            2 => -min * v.sin(),
            _ => -min * v.cos(),
        }
    };
    let (c, s) = (u.cos(), u.sin());
    let (cu, su) = match i % 4 {
        0 => (c, s),
        1 => (-s, c),
        2 => (-c, -s),
        _ => (s, -c),
    };
    [rad * cu, rad * su, z]
}

fn close(a: [f64; 3], b: [f64; 3], tol: f64) -> bool {
    (0..3).all(|k| (a[k] - b[k]).abs() < tol)
}

fn fmt(v: [f64; 3]) -> String {
    format!("({:7.4},{:7.4},{:7.4})", v[0], v[1], v[2])
}

/// Evaluate and report which (i,j) each slot holds.
fn probe_surface(
    label: &str,
    surf: &Surf,
    u: f64,
    v: f64,
    n_u: i32,
    n_v: i32,
    triangular: bool,
    expect: &dyn Fn(usize, usize) -> [f64; 3],
) {
    let count = if triangular {
        // A triangular table of total order <= max(n_u, n_v).
        let n = n_u.max(n_v) as usize;
        (n + 1) * (n + 2) / 2
    } else {
        ((n_u + 1) * (n_v + 1)) as usize
    };
    let mut p = vec![0.0f64; (count + 8) * 3]; // slack, so an overrun is visible
    let uv = [u, v];
    let rc = unsafe {
        PK_SURF_eval(
            surf.tag(),
            uv.as_ptr(),
            n_u,
            n_v,
            if triangular {
                PK_LOGICAL_true
            } else {
                PK_LOGICAL_false
            },
            p.as_mut_ptr(),
        )
    };
    println!(
        "\n-- {label}  n_u={n_u} n_v={n_v} {}  (rc={rc}, {count} slots)",
        if triangular {
            "TRIANGULAR"
        } else {
            "rectangular"
        }
    );
    if rc != PK_ERROR_no_errors {
        println!("   call failed");
        return;
    }

    for slot in 0..count {
        let got = [p[slot * 3], p[slot * 3 + 1], p[slot * 3 + 2]];
        // Find every (i,j) whose closed-form value matches.
        let mut matches = Vec::new();
        for i in 0..=(n_u.max(n_v) as usize) {
            for j in 0..=(n_u.max(n_v) as usize) {
                if close(got, expect(i, j), 1e-9) {
                    matches.push(format!("d{i}u.d{j}v"));
                }
            }
        }
        println!(
            "   [{slot:2}] {}  = {}",
            fmt(got),
            if matches.is_empty() {
                "?? no closed-form match".to_string()
            } else {
                matches.join(" | ")
            }
        );
    }
    // Did the kernel write past the slots we predicted?
    let overrun: Vec<usize> = (count * 3..p.len()).filter(|&k| p[k] != 0.0).collect();
    if !overrun.is_empty() {
        println!("   !! wrote past slot {count}: doubles {overrun:?}");
    }
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");

    let basis = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    // ---- Cylinder: all mixed partials vanish, du^k cycles with period 4 ----
    let cyl = Surf::cylinder(basis, R).expect("cylinder");
    let u = 0.7_f64;
    let v = 1.5_f64;

    // Confirm the parameterisation before trusting the derivative table.
    let pos = cyl.eval(u, v).expect("eval");
    println!("== cylinder r={R}, parameterisation check ==");
    println!(
        "   eval({u},{v}) = ({:.4},{:.4},{:.4})   expected (R cos u, R sin u, v) = ({:.4},{:.4},{:.4})",
        pos.x,
        pos.y,
        pos.z,
        R * u.cos(),
        R * u.sin(),
        v
    );

    let cyl_expect = move |i: usize, j: usize| -> [f64; 3] {
        let mut d = cyl_deriv(i, j, u).unwrap();
        if (i, j) == (0, 0) {
            d[2] = v;
        }
        d
    };

    probe_surface("cylinder", &cyl, u, v, 2, 2, false, &cyl_expect);
    probe_surface("cylinder", &cyl, u, v, 3, 1, false, &cyl_expect);
    probe_surface("cylinder", &cyl, u, v, 2, 2, true, &cyl_expect);

    // ---- Torus: nonzero mixed partials disambiguate the (i,j) ordering ----
    let (maj, min) = (5.0_f64, 1.5_f64);
    let tor = Surf::torus(basis, maj, min).expect("torus");
    let (tu, tv) = (0.6_f64, 0.9_f64);
    let tpos = tor.eval(tu, tv).expect("eval torus");
    println!("\n== torus MAJ={maj} MIN={min}, parameterisation check ==");
    let expect0 = torus_deriv(0, 0, tu, tv, maj, min);
    println!(
        "   eval({tu},{tv}) = ({:.4},{:.4},{:.4})   model = {}",
        tpos.x,
        tpos.y,
        tpos.z,
        fmt(expect0)
    );

    let tor_expect = move |i: usize, j: usize| -> [f64; 3] { torus_deriv(i, j, tu, tv, maj, min) };
    probe_surface("torus", &tor, tu, tv, 2, 2, false, &tor_expect);
    probe_surface("torus", &tor, tu, tv, 2, 2, true, &tor_expect);

    // ---- Curve: same question, one parameter ----
    println!("\n== curve (circle r=3) derivative order ==");
    let circ = Curve::circle(basis, 3.0).expect("circle");
    let t = 0.4_f64;
    let mut cp = [0.0f64; 4 * 3 + 6];
    let rc = unsafe { PK_CURVE_eval(circ.tag(), t, 3, cp.as_mut_ptr()) };
    println!("   PK_CURVE_eval(n_deriv=3) rc={rc}");
    for k in 0..4 {
        let got = [cp[k * 3], cp[k * 3 + 1], cp[k * 3 + 2]];
        let (c, s) = (t.cos(), t.sin());
        let want = match k % 4 {
            0 => [3.0 * c, 3.0 * s, 0.0],
            1 => [-3.0 * s, 3.0 * c, 0.0],
            2 => [-3.0 * c, -3.0 * s, 0.0],
            _ => [3.0 * s, -3.0 * c, 0.0],
        };
        println!(
            "   [{k}] {} = d{k}/dt{}  ({})",
            fmt(got),
            if close(got, want, 1e-9) {
                ""
            } else {
                " MISMATCH"
            },
            fmt(want)
        );
    }

    curvature_and_singularities();

    println!("\n== done");
}

// Appended: curvature sign convention and singular behaviour.
#[allow(dead_code)]
fn curvature_and_singularities() {
    let basis = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    println!("\n== curvature sign convention (relative to the reported normal) ==");

    let sph = Surf::sphere(basis, 4.0).expect("sphere");
    let c = sph.eval_curvature(0.3, 0.2).expect("sphere curvature");
    let p = sph.eval(0.3, 0.2).expect("pos");
    // For a sphere centred at the origin, the outward direction is p/|p|.
    let outward_dot = c.normal.x * p.x + c.normal.y * p.y + c.normal.z * p.z;
    println!(
        "   sphere r=4:  k1={:+.6} k2={:+.6}  normal.(outward)={:+.3}  => normal points {}",
        c.principal_curvature_1,
        c.principal_curvature_2,
        outward_dot,
        if outward_dot > 0.0 {
            "OUTWARD"
        } else {
            "INWARD"
        }
    );
    println!(
        "                1/r = {:.6}; sign of k vs outward normal tells the convention",
        1.0 / 4.0
    );

    let cyl = Surf::cylinder(basis, 2.0).expect("cyl");
    let c = cyl.eval_curvature(0.4, 1.0).expect("cyl curvature");
    let p = cyl.eval(0.4, 1.0).expect("pos");
    let outward_dot = c.normal.x * p.x + c.normal.y * p.y;
    println!(
        "   cylinder r=2: k1={:+.6} k2={:+.6}  normal.(radial)={:+.3}",
        c.principal_curvature_1, c.principal_curvature_2, outward_dot
    );
    println!(
        "                 dir1=({:.3},{:.3},{:.3}) dir2=({:.3},{:.3},{:.3})",
        c.principal_direction_1.x,
        c.principal_direction_1.y,
        c.principal_direction_1.z,
        c.principal_direction_2.x,
        c.principal_direction_2.y,
        c.principal_direction_2.z
    );

    // Torus: outer equator is convex both ways; inner equator is a saddle. The
    // sign difference between them is the sharpest test of the convention.
    let tor = Surf::torus(basis, 5.0, 1.5).expect("torus");
    for (label, v) in [
        ("outer equator (v=0)", 0.0),
        ("inner equator (v=pi)", std::f64::consts::PI),
    ] {
        let c = tor.eval_curvature(0.0, v).expect("torus curvature");
        println!(
            "   torus {label}: k1={:+.6} k2={:+.6}  (gauss = {:+.6})",
            c.principal_curvature_1,
            c.principal_curvature_2,
            c.principal_curvature_1 * c.principal_curvature_2
        );
    }

    println!("\n== singular points: what does the kernel do? ==");
    // Sphere pole: v = +pi/2 is the pole, where dR/du vanishes.
    let pole_v = std::f64::consts::FRAC_PI_2;
    match sph.eval_jet(0.0, pole_v, 1, 1, false) {
        Ok(j) => println!(
            "   sphere pole eval_jet: ok; |du|={:.3e} |dv|={:.3e} unit_normal={:?}",
            (j.du().unwrap().x.powi(2) + j.du().unwrap().y.powi(2) + j.du().unwrap().z.powi(2))
                .sqrt(),
            (j.dv().unwrap().x.powi(2) + j.dv().unwrap().y.powi(2) + j.dv().unwrap().z.powi(2))
                .sqrt(),
            j.unit_normal().map(|n| (n.x, n.y, n.z))
        ),
        Err(e) => println!("   sphere pole eval_jet: ERROR {e}"),
    }
    match sph.eval_curvature(0.0, pole_v) {
        Ok(c) => println!(
            "   sphere pole curvature: k1={:+.6} k2={:+.6}",
            c.principal_curvature_1, c.principal_curvature_2
        ),
        Err(e) => println!("   sphere pole curvature: ERROR {e}"),
    }

    // Cone apex.
    let cone = Surf::cone(basis, 3.0, 0.5).expect("cone");
    // The apex is where the radius goes to zero: v such that r + v*tan(semi) = 0.
    let v_apex = -3.0 / 0.5_f64.tan();
    match cone.eval_jet(0.0, v_apex, 1, 1, false) {
        Ok(j) => println!(
            "   cone apex eval_jet: ok; unit_normal={:?} singular={}",
            j.unit_normal().map(|n| (n.x, n.y, n.z)),
            j.is_singular()
        ),
        Err(e) => println!("   cone apex eval_jet: ERROR {e}"),
    }

    println!("\n== min radius of curvature ==");
    let circ = Curve::circle(basis, 3.0).expect("circle");
    println!(
        "   circle r=3 over [0,2pi): {:?}",
        circ.find_min_radius(0.0, 6.28)
            .map(|o| o.map(|m| (m.radius, m.param.0)))
    );
    let line = Curve::line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).expect("line");
    println!(
        "   straight line over [0,10]: {:?}",
        line.find_min_radius(0.0, 10.0).map(|o| o.map(|m| m.radius))
    );
    let uvbox = tor.uvbox().expect("uvbox");
    println!(
        "   torus (MAJ=5,MIN=1.5) over full uvbox: {:?}",
        tor.find_min_radii(uvbox)
            .map(|v| v.iter().map(|m| m.radius).collect::<Vec<_>>())
    );
    let plane = Surf::plane(basis).expect("plane");
    let pbox = plane.uvbox().expect("plane uvbox");
    println!(
        "   plane: {:?}",
        plane.find_min_radii(pbox).map(|v| v.len())
    );
}
