//! Enclosure conservativeness probe (Stage 6).
//!
//! One property decides whether an enclosure can be used for exclusion: it must
//! **contain** the geometry. A box even slightly inward silently turns pruning
//! into wrong answers, and nothing downstream can detect it.
//!
//! So: compare every box finder against an analytically exact box and report
//! the signed slack per face. Positive slack = conservative (safe). Any
//! negative slack = the enclosure is inward and unusable for exclusion.
//!
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/box_probe.exe

use parasolid::*;

fn basis() -> Axis2 {
    Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    )
}

/// Report slack of `got` relative to the exact box `[emin, emax]`.
fn report(label: &str, got: &Aabb, emin: Vec3, emax: Vec3) {
    let slack = [
        emin.x - got.min.x,
        emin.y - got.min.y,
        emin.z - got.min.z,
        got.max.x - emax.x,
        got.max.y - emax.y,
        got.max.z - emax.z,
    ];
    let worst = slack.iter().cloned().fold(f64::INFINITY, f64::min);
    let biggest = slack.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    println!(
        "  {label:34} min slack={worst:+.3e} max slack={biggest:+.3e}  {}",
        if worst < 0.0 {
            "*** INWARD — UNSAFE ***"
        } else if biggest == 0.0 {
            "exactly tight"
        } else {
            "conservative"
        }
    );
    println!(
        "      got  [{:.6},{:.6},{:.6}] .. [{:.6},{:.6},{:.6}]",
        got.min.x, got.min.y, got.min.z, got.max.x, got.max.y, got.max.z
    );
    println!(
        "      exact[{:.6},{:.6},{:.6}] .. [{:.6},{:.6},{:.6}]",
        emin.x, emin.y, emin.z, emax.x, emax.y, emax.z
    );
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    let b = basis();

    println!("== PK_CURVE_find_box vs exact ==");
    // A full circle of radius 3 in the XY plane: exact box is [-3,-3,0]..[3,3,0].
    let circ = Curve::circle(b, 3.0).unwrap();
    match circ.find_box(None) {
        Ok(bx) => report(
            "circle r=3 (whole curve)",
            &bx,
            Vec3::new(-3.0, -3.0, 0.0),
            Vec3::new(3.0, 3.0, 0.0),
        ),
        Err(e) => println!("  circle whole-curve box ERROR: {e}"),
    }
    // Restricted to the first quadrant: exact box is [0,0,0]..[3,3,0].
    match circ.find_box(Some((0.0, std::f64::consts::FRAC_PI_2))) {
        Ok(bx) => report(
            "circle r=3 (quarter arc)",
            &bx,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(3.0, 3.0, 0.0),
        ),
        Err(e) => println!("  circle quarter-arc box ERROR: {e}"),
    }

    println!("\n== PK_SURF_find_box vs exact ==");
    // Sphere r=4 over its full uvbox: exact box is [-4,-4,-4]..[4,4,4].
    let sph = Surf::sphere(b, 4.0).unwrap();
    let uvb = sph.uvbox().unwrap();
    match sph.find_box(Some(uvb)) {
        Ok(bx) => report(
            "sphere r=4 (full uvbox)",
            &bx,
            Vec3::new(-4.0, -4.0, -4.0),
            Vec3::new(4.0, 4.0, 4.0),
        ),
        Err(e) => println!("  sphere box ERROR: {e}"),
    }
    // Cylinder r=2 restricted to v in [0,6]: exact [-2,-2,0]..[2,2,6].
    let cyl = Surf::cylinder(b, 2.0).unwrap();
    let cyl_box = UvBox {
        u_min: 0.0,
        u_max: std::f64::consts::TAU,
        v_min: 0.0,
        v_max: 6.0,
    };
    match cyl.find_box(Some(cyl_box)) {
        Ok(bx) => report(
            "cylinder r=2, v in [0,6]",
            &bx,
            Vec3::new(-2.0, -2.0, 0.0),
            Vec3::new(2.0, 2.0, 6.0),
        ),
        Err(e) => println!("  cylinder box ERROR: {e}"),
    }
    // Torus: exact box is [-(R+r), -(R+r), -r] .. [R+r, R+r, r].
    let (maj, min) = (5.0f64, 1.5f64);
    let tor = Surf::torus(b, maj, min).unwrap();
    let tb = tor.uvbox().unwrap();
    match tor.find_box(Some(tb)) {
        Ok(bx) => report(
            "torus 5/1.5 (full uvbox)",
            &bx,
            Vec3::new(-(maj + min), -(maj + min), -min),
            Vec3::new(maj + min, maj + min, min),
        ),
        Err(e) => println!("  torus box ERROR: {e}"),
    }

    println!("\n== unrestricted box on an UNBOUNDED surface ==");
    match Surf::plane(b).unwrap().find_box(None) {
        Ok(bx) => println!(
            "  plane, no uvbox: [{:.3e},{:.3e},{:.3e}]..[{:.3e},{:.3e},{:.3e}]",
            bx.min.x, bx.min.y, bx.min.z, bx.max.x, bx.max.y, bx.max.z
        ),
        Err(e) => println!("  plane unrestricted box ERROR: {e}"),
    }

    println!("\n== PK_TOPOL_find_box (body) vs exact ==");
    let block = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    match block.bounding_box() {
        Ok(bx) => report(
            "block 10x20x30",
            &bx,
            Vec3::new(-5.0, -10.0, 0.0),
            Vec3::new(5.0, 10.0, 30.0),
        ),
        Err(e) => println!("  block box ERROR: {e}"),
    }
    let ball = Body::create_solid_sphere(4.0).unwrap();
    match ball.bounding_box() {
        Ok(bx) => report(
            "sphere body r=4",
            &bx,
            Vec3::new(-4.0, -4.0, -4.0),
            Vec3::new(4.0, 4.0, 4.0),
        ),
        Err(e) => println!("  sphere body box ERROR: {e}"),
    }

    println!("\n== oriented boxes: does `dimension` report degeneracy? ==");
    // A planar circle needs only 2 dimensions; a straight line only 1.
    match circ.find_oriented_box((0.0, std::f64::consts::TAU)) {
        Ok(ob) => println!(
            "  circle: dimension={} widths=({:.4},{:.4},{:.4}) centre=({:.4},{:.4},{:.4})",
            ob.dimension, ob.widths[0], ob.widths[1], ob.widths[2], ob.centre.x, ob.centre.y,
            ob.centre.z
        ),
        Err(e) => println!("  circle obox ERROR: {e}"),
    }
    let line = Curve::line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).unwrap();
    match line.find_oriented_box((0.0, 10.0)) {
        Ok(ob) => println!(
            "  line:   dimension={} widths=({:.4},{:.4},{:.4})",
            ob.dimension, ob.widths[0], ob.widths[1], ob.widths[2]
        ),
        Err(e) => println!("  line obox ERROR: {e}"),
    }
    match sph.find_oriented_box(uvb) {
        Ok(ob) => println!(
            "  sphere: dimension={} widths=({:.4},{:.4},{:.4})",
            ob.dimension, ob.widths[0], ob.widths[1], ob.widths[2]
        ),
        Err(e) => println!("  sphere obox ERROR: {e}"),
    }

    println!("\n== PK_GEOM_range_vector: global closest approach ==");
    for p in [
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 100.0),
    ] {
        match sph.range_to_point(p) {
            Ok(r) => println!(
                "  sphere r=4 vs ({:.0},{:.0},{:.0}): status={:?} distance={:.6} witness=({:.4},{:.4},{:.4})",
                p.x, p.y, p.z, r.status, r.distance,
                r.witness_1.position.x, r.witness_1.position.y, r.witness_1.position.z
            ),
            Err(e) => println!("  sphere range vs ({:.0},{:.0},{:.0}) ERROR: {e}", p.x, p.y, p.z),
        }
    }

    println!("\n== done");
}
