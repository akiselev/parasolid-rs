//! Stage 7 SSI probe: option versions, intersection-kind tokens, pair matrix.
//!
//! First job is the lesson from Stage 6: `PK_SURF_intersect_surf_o_t` ships
//! with `mixed_curve_category = 0`, which is NOT a legal
//! `PK_mixed_intersection_t` (legal: pline 26650, classic 26651, both 26652).
//! SSI works anyway, which can only mean the field is version-gated — so sweep
//! the accepted `o_t_version` and find where it starts being read, BEFORE
//! trusting any result this call produces.

use parasolid::*;
use parasolid_sys::*;

fn basis() -> Axis2 {
    Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    )
}

/// Raw SSI call with an explicit version + mixed_curve_category.
fn ssi_raw(a: &Surf, b: &Surf, version: i32, mixed: i32) -> (i32, i32, i32) {
    let mut o: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
    o.o_t_version = version;
    o.mixed_curve_category = mixed;
    let (mut nv, mut nc) = (0i32, 0i32);
    let mut vectors = std::ptr::null_mut();
    let mut curves = std::ptr::null_mut();
    let mut bounds = std::ptr::null_mut();
    let mut types = std::ptr::null_mut();
    let rc = unsafe {
        PK_SURF_intersect_surf(
            a.tag(),
            b.tag(),
            &o,
            &mut nv,
            &mut vectors,
            &mut nc,
            &mut curves,
            &mut bounds,
            &mut types,
        )
    };
    unsafe {
        for p in [
            vectors as *mut std::os::raw::c_void,
            curves as *mut _,
            bounds as *mut _,
            types as *mut _,
        ] {
            if !p.is_null() {
                let _ = PK_MEMORY_free(p);
            }
        }
    }
    (rc, nv, nc)
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    let b = basis();
    let cyl = Surf::cylinder(b, 5.0).unwrap();
    let plane = Surf::plane(Axis2::new(
        Vec3::new(0.0, 0.0, 3.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    ))
    .unwrap();

    println!("== o_t_version sweep (mixed_curve_category = 0, i.e. ILLEGAL) ==");
    for v in 0..=8 {
        let (rc, nv, nc) = ssi_raw(&cyl, &plane, v, 0);
        println!("  version {v}: rc={rc:<6} n_vectors={nv} n_curves={nc}");
    }

    println!("\n== at each accepted version, is mixed_curve_category READ? ==");
    for v in 1..=6 {
        let legal = ssi_raw(&cyl, &plane, v, PK_mixed_intersection_classic_c);
        let illegal = ssi_raw(&cyl, &plane, v, 12345);
        println!(
            "  version {v}: classic -> rc={:<6} | garbage 12345 -> rc={:<6} {}",
            legal.0,
            illegal.0,
            if illegal.0 != 0 {
                "<-- FIELD IS READ"
            } else {
                "field ignored"
            }
        );
    }

    println!("\n== mixed_curve_category token legality (at the highest accepted version) ==");
    let mut top = 1;
    for v in 1..=8 {
        if ssi_raw(&cyl, &plane, v, PK_mixed_intersection_classic_c).0 == 0 {
            top = v;
        }
    }
    println!("  highest accepted version = {top}");
    for (name, tok) in [
        ("pline   26650", PK_mixed_intersection_pline_c),
        ("classic 26651", PK_mixed_intersection_classic_c),
        ("both    26652", PK_mixed_intersection_both_c),
        ("zero        0", 0),
    ] {
        let (rc, nv, nc) = ssi_raw(&cyl, &plane, top, tok);
        println!("  {name}: rc={rc:<6} n_vectors={nv} n_curves={nc}");
    }

    println!("\n== analytic pair matrix (via the safe wrapper) ==");
    let sph = |r: f64, z: f64| {
        Surf::sphere(
            Axis2::new(
                Vec3::new(0.0, 0.0, z),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            r,
        )
        .unwrap()
    };
    let cases: Vec<(&str, Surf, Surf)> = vec![
        (
            "cyl(5) x plane z=3 (circle)",
            Surf::cylinder(b, 5.0).unwrap(),
            plane,
        ),
        (
            "sphere(5)@0 x sphere(5)@6 (circle)",
            sph(5.0, 0.0),
            sph(5.0, 6.0),
        ),
        (
            "sphere(5)@0 x sphere(5)@10 (tangent point)",
            sph(5.0, 0.0),
            sph(5.0, 10.0),
        ),
        (
            "sphere(5)@0 x sphere(5)@20 (disjoint)",
            sph(5.0, 0.0),
            sph(5.0, 20.0),
        ),
        ("sphere(5) x plane z=3 (circle r=4)", sph(5.0, 0.0), plane),
        (
            "cyl(5) x cyl(5) coaxial (coincident)",
            Surf::cylinder(b, 5.0).unwrap(),
            Surf::cylinder(b, 5.0).unwrap(),
        ),
        (
            "plane x plane parallel (disjoint)",
            Surf::plane(b).unwrap(),
            plane,
        ),
    ];
    for (label, s1, s2) in cases {
        match s1.intersect(&s2) {
            Ok(r) => {
                let kinds: Vec<String> = r
                    .curves
                    .iter()
                    .map(|c| format!("{:?}({})", c.classify(), c.kind))
                    .collect();
                println!(
                    "  {label:44} points={} curves={} kinds={kinds:?}",
                    r.points.len(),
                    r.curves.len()
                );
            }
            Err(e) => println!("  {label:44} ERROR {e}"),
        }
    }

    println!("\n== done");
}
