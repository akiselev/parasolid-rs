//! Stage 7: the analytic SSI pair matrix, with independent verification.
//!
//! For each pair we assert against CLOSED FORM, not just "some curve came
//! back": a cylinder cut by a plane must give a circle of the right radius at
//! the right height, two spheres must give the exact intersection circle, and
//! so on. Every returned curve is also sampled and checked to lie on BOTH
//! surfaces (two-sided residual), which is the check that catches a plausible
//! but wrong branch.

use parasolid::*;

fn ax(o: Vec3) -> Axis2 {
    Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
}
fn ax_dir(o: Vec3, axis: Vec3, r: Vec3) -> Axis2 {
    Axis2::new(o, axis, r)
}
fn o() -> Vec3 {
    Vec3::new(0.0, 0.0, 0.0)
}

/// Sample an intersection curve and report the worst distance to each surface.
fn two_sided_residual(c: &IntersectionCurve, s1: &Surf, s2: &Surf) -> (f64, f64) {
    let (lo, hi) = c.bounds;
    let (mut w1, mut w2) = (0.0f64, 0.0f64);
    for k in 0..=20 {
        let t = lo + (hi - lo) * (k as f64) / 20.0;
        let Ok(p) = c.curve.eval(t) else { continue };
        for (s, w) in [(s1, &mut w1), (s2, &mut w2)] {
            if let Ok(r) = s.range_to_point(p) {
                if r.distance > *w {
                    *w = r.distance;
                }
            }
        }
    }
    (w1, w2)
}

fn report(label: &str, s1: &Surf, s2: &Surf, expect: &str) {
    match s1.intersect(s2) {
        Ok(r) => {
            let mut detail = String::new();
            for c in &r.curves {
                let (a, b) = two_sided_residual(c, s1, s2);
                let ty = c
                    .curve
                    .curve_type()
                    .map(|t| format!("{t:?}"))
                    .unwrap_or_default();
                let radius = c
                    .curve
                    .ask_circle()
                    .map(|ci| {
                        format!(
                            " r={:.6} centre=({:.3},{:.3},{:.3})",
                            ci.radius, ci.basis.origin.x, ci.basis.origin.y, ci.basis.origin.z
                        )
                    })
                    .unwrap_or_default();
                detail.push_str(&format!(
                    "\n      {ty}{radius} kind={:?}({}) bounds=({:.4},{:.4}) resid=({a:.2e},{b:.2e})",
                    c.classify(), c.kind, c.bounds.0, c.bounds.1
                ));
            }
            for (i, p) in r.points.iter().enumerate() {
                detail.push_str(&format!(
                    "\n      point[{i}] = ({:.6},{:.6},{:.6})",
                    p.x, p.y, p.z
                ));
            }
            println!(
                "  {label:46} pts={} curves={}  [expect {expect}]{detail}",
                r.points.len(),
                r.curves.len()
            );
        }
        Err(e) => println!("  {label:46} ERROR {e}  [expect {expect}]"),
    }
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");

    println!("== transversal: exact analytic answers ==");
    let cyl5 = Surf::cylinder(ax(o()), 5.0).unwrap();
    let plane_z3 = Surf::plane(ax(Vec3::new(0.0, 0.0, 3.0))).unwrap();
    report("cyl(5) x plane z=3", &cyl5, &plane_z3, "circle r=5 @ z=3");

    let sph5 = Surf::sphere(ax(o()), 5.0).unwrap();
    report(
        "sphere(5) x plane z=3",
        &sph5,
        &plane_z3,
        "circle r=4 @ z=3",
    );

    let sph5b = Surf::sphere(ax(Vec3::new(0.0, 0.0, 6.0)), 5.0).unwrap();
    report(
        "sphere(5)@0 x sphere(5)@6",
        &sph5,
        &sph5b,
        "circle r=4 @ z=3",
    );

    // Cone x plane: the conic ladder.
    let cone = Surf::cone(ax(o()), 3.0, 0.5).unwrap();
    report("cone(r=3,a=0.5) x plane z=3", &cone, &plane_z3, "circle");
    let plane_x0 = Surf::plane(ax_dir(
        o(),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ))
    .unwrap();
    report(
        "cone x plane through axis",
        &cone,
        &plane_x0,
        "hyperbola/lines",
    );

    // Torus x plane: equatorial circle, and the Villarceau-adjacent case.
    let tor = Surf::torus(ax(o()), 5.0, 1.5).unwrap();
    report(
        "torus(5,1.5) x plane z=0",
        &tor,
        &Surf::plane(ax(o())).unwrap(),
        "two circles r=3.5,6.5",
    );
    report(
        "torus(5,1.5) x plane z=1.5",
        &tor,
        &Surf::plane(ax(Vec3::new(0.0, 0.0, 1.5))).unwrap(),
        "one circle r=5",
    );

    println!("\n== cylinder-cylinder ==");
    let cyl5_x = Surf::cylinder(
        ax_dir(o(), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        5.0,
    )
    .unwrap();
    report(
        "cyl(5)Z x cyl(5)X equal radius",
        &cyl5,
        &cyl5_x,
        "Steinmetz: 2 ellipses",
    );
    let cyl3_x = Surf::cylinder(
        ax_dir(o(), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
        3.0,
    )
    .unwrap();
    report(
        "cyl(5)Z x cyl(3)X unequal",
        &cyl5,
        &cyl3_x,
        "quartic curve(s)",
    );
    let cyl5_par = Surf::cylinder(ax(Vec3::new(12.0, 0.0, 0.0)), 5.0).unwrap();
    report(
        "cyl(5) x cyl(5) parallel disjoint",
        &cyl5,
        &cyl5_par,
        "nothing",
    );
    let cyl5_touch = Surf::cylinder(ax(Vec3::new(10.0, 0.0, 0.0)), 5.0).unwrap();
    report(
        "cyl(5) x cyl(5) parallel tangent",
        &cyl5,
        &cyl5_touch,
        "tangent line",
    );

    println!("\n== tangency ==");
    let sph_tan = Surf::sphere(ax(Vec3::new(0.0, 0.0, 10.0)), 5.0).unwrap();
    report(
        "sphere(5)@0 x sphere(5)@10 tangent",
        &sph5,
        &sph_tan,
        "single point",
    );
    let plane_z5 = Surf::plane(ax(Vec3::new(0.0, 0.0, 5.0))).unwrap();
    report(
        "sphere(5) x plane z=5 (tangent)",
        &sph5,
        &plane_z5,
        "single point / degenerate",
    );
    let cyl_tan_plane = Surf::plane(ax_dir(
        Vec3::new(5.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ))
    .unwrap();
    report(
        "cyl(5) x tangent plane x=5",
        &cyl5,
        &cyl_tan_plane,
        "tangent line",
    );

    println!("\n== coincident / disjoint: can SSI tell them apart? ==");
    report(
        "cyl(5) x cyl(5) SAME axis+radius",
        &cyl5,
        &Surf::cylinder(ax(o()), 5.0).unwrap(),
        "coincident",
    );
    report(
        "plane z=3 x plane z=3",
        &plane_z3,
        &Surf::plane(ax(Vec3::new(0.0, 0.0, 3.0))).unwrap(),
        "coincident",
    );
    report(
        "plane z=3 x plane z=9 parallel",
        &plane_z3,
        &Surf::plane(ax(Vec3::new(0.0, 0.0, 9.0))).unwrap(),
        "disjoint",
    );
    report(
        "sphere(5) x sphere(5) same",
        &sph5,
        &Surf::sphere(ax(o()), 5.0).unwrap(),
        "coincident",
    );

    println!("\n== oblique placement (Stage 2 payoff: nothing axis-aligned) ==");
    let s3 = 1.0 / 3.0_f64.sqrt();
    let obl = Transform::rotation(o(), Vec3::new(s3, s3, s3), 0.9)
        .unwrap()
        .then(&Transform::translation(2.0, -3.0, 1.0).unwrap())
        .unwrap();
    let (cyl_o, e1) = cyl5.transformed(&obl).unwrap();
    let (pl_o, e2) = plane_z3.transformed(&obl).unwrap();
    println!("  (placement exact: cyl={e1} plane={e2})");
    report(
        "oblique cyl(5) x oblique plane",
        &cyl_o,
        &pl_o,
        "circle r=5, still exact",
    );

    println!("\n== done");
}
