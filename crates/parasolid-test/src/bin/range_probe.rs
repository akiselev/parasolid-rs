//! Inversion / distance / extrema semantics probe (Stage 5).
//!
//! Stage 5 exists to force the *result contract*: unique, multiple, singular,
//! boundary and indeterminate have to be distinguishable, because an
//! `Option<tuple>` here poisons SSI, checking and Booleans later. Before the
//! contract can be designed, the kernel has to be asked what it actually says
//! in each of those situations.
//!
//! Specifically:
//!   - what `PK_range_result_t` values appear, and when;
//!   - whether the witness (`sub_entity`, `parameters`) is populated — the
//!     wrapper currently discards both;
//!   - what happens for a genuinely indeterminate query (a point equidistant
//!     from a whole circle of answers);
//!   - whether a nearest-point answer is global or merely local.
//!
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/range_probe.exe

use parasolid::*;
use parasolid_sys::*;

fn range_result_name(t: PK_range_result_t) -> &'static str {
    match t {
        18270 => "found",
        18271 => "lower(bounded out below)",
        18272 => "upper(bounded out above)",
        18273 => "not_found",
        _ => "??",
    }
}

fn dump_end(label: &str, e: &PK_range_end_t) {
    println!(
        "      {label}: entity={} sub_entity={} pos=({:.4},{:.4},{:.4}) params=({:.4},{:.4})",
        e.entity,
        e.sub_entity,
        e.position[0],
        e.position[1],
        e.position[2],
        e.parameters[0],
        e.parameters[1]
    );
}

/// Distance from a topological entity to a point, showing everything returned.
fn probe_range_vector(label: &str, topol: i32, p: Vec3) {
    let v: PK_VECTOR_t = [p.x, p.y, p.z];
    let mut opts = PK_TOPOL_range_vector_o_t::default();
    let mut status: PK_range_result_t = 0;
    let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
    let rc = unsafe { PK_TOPOL_range_vector(topol, &v, &mut opts, &mut status, &mut r) };
    println!(
        "  {label}\n    rc={rc} status={status}({}) distance={:.6}",
        range_result_name(status),
        r.distance
    );
    dump_end("end", &r.end);
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    let b = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    println!("== PK_TOPOL_range_vector: witness fields and status ==");
    let block = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
    // Outside, nearest a face; outside nearest an edge; outside nearest a
    // vertex; and inside. Each should name a different sub_entity class.
    probe_range_vector(
        "point far above the top face",
        block.tag(),
        Vec3::new(0.0, 0.0, 40.0),
    );
    probe_range_vector(
        "point beyond a vertical edge",
        block.tag(),
        Vec3::new(20.0, 20.0, 5.0),
    );
    probe_range_vector(
        "point beyond a corner",
        block.tag(),
        Vec3::new(20.0, 20.0, 40.0),
    );
    probe_range_vector(
        "point inside the block",
        block.tag(),
        Vec3::new(0.0, 0.0, 5.0),
    );

    println!("\n== indeterminate: a point equidistant from a whole circle ==");
    // The centre of a cylinder's axis is equidistant from every point of the
    // circular wall. If the kernel still reports `found` with one arbitrary
    // witness, the caller MUST NOT treat that witness as unique.
    let cyl = Body::create_solid_cylinder(5.0, 10.0).unwrap();
    let faces = cyl.faces().unwrap();
    let wall = faces
        .iter()
        .find(|f| matches!(f.surface_type(), Ok(SurfType::Cylinder)))
        .expect("wall");
    probe_range_vector(
        "axis point vs cylindrical wall",
        wall.tag(),
        Vec3::new(0.0, 0.0, 5.0),
    );
    probe_range_vector(
        "axis point again (determinism?)",
        wall.tag(),
        Vec3::new(0.0, 0.0, 5.0),
    );

    println!("\n== surface inversion at an indeterminate point ==");
    // The centre of a sphere inverts to... every (u,v). What comes back?
    let sph = Surf::sphere(b, 4.0).unwrap();
    match sph.parameterise(Vec3::new(0.0, 0.0, 0.0)) {
        Ok(uv) => println!("  sphere centre -> parameterise = {uv:?}  (a single arbitrary answer)"),
        Err(e) => println!("  sphere centre -> parameterise ERROR: {e}"),
    }
    match sph.parameterise(Vec3::new(10.0, 0.0, 0.0)) {
        Ok(uv) => println!("  point outside sphere -> {uv:?}"),
        Err(e) => println!("  point outside sphere -> ERROR: {e}"),
    }
    // Periodic equivalence: a point ON the seam has two legal u values.
    let seam_pt = sph.eval(0.0, 0.3).unwrap();
    match sph.parameterise(seam_pt) {
        Ok(uv) => println!("  point on the u=0 seam -> {uv:?}  (u=0 and u=2pi are the same point)"),
        Err(e) => println!("  seam point -> ERROR: {e}"),
    }

    println!("\n== local vs global: a curve with two competing minima ==");
    // An ellipse has two equally-near points to a point on its minor axis.
    let ell = Curve::ellipse(b, 5.0, 2.0).unwrap();
    for probe in [Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 0.0, 0.0)] {
        match ell.parameterise(probe) {
            Ok(t) => {
                let back = ell.eval(t).unwrap();
                println!(
                    "  ellipse.parameterise(({:.1},{:.1},{:.1})) = t={:.6} -> ({:.4},{:.4},{:.4})",
                    probe.x, probe.y, probe.z, t, back.x, back.y, back.z
                );
            }
            Err(e) => println!("  ellipse.parameterise -> ERROR: {e}"),
        }
    }

    println!("\n== PK_BODY_find_extreme ==");
    let dir1: PK_VECTOR_t = [0.0, 0.0, 1.0];
    let dir2: PK_VECTOR_t = [1.0, 0.0, 0.0];
    let dir3: PK_VECTOR_t = [0.0, 1.0, 0.0];
    let mut extreme: PK_VECTOR_t = [0.0; 3];
    let mut topol: PK_TOPOL_t = 0;
    let rc = unsafe {
        PK_BODY_find_extreme(
            block.tag(),
            &dir1,
            &dir2,
            &dir3,
            std::ptr::null_mut(),
            &mut extreme,
            &mut topol,
        )
    };
    println!(
        "  block extreme in +Z,+X,+Y: rc={rc} pos=({:.4},{:.4},{:.4}) topol={topol}",
        extreme[0], extreme[1], extreme[2]
    );
    println!("    (witness topology tag {topol}; class looked up below)");

    println!("\n== PK_TOPOL_clash under the minimal frustrum ==");
    let a = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    let c = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    // Build an empirical clash_type token table: the sys constants claim
    // 0..4, but a full overlap already reports 7, so they are fabricated.
    let cases: Vec<(&str, Body, Body)> = vec![
        (
            "identical blocks (full overlap)",
            Body::create_solid_block(4.0, 4.0, 4.0).unwrap(),
            Body::create_solid_block(4.0, 4.0, 4.0).unwrap(),
        ),
        (
            "partial overlap",
            Body::create_solid_block(4.0, 4.0, 4.0).unwrap(),
            {
                let t = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
                t.transform(&Transform::translation(2.0, 0.0, 0.0).unwrap())
                    .unwrap();
                t
            },
        ),
        (
            "abutting (faces touch, no common interior)",
            Body::create_solid_block(4.0, 4.0, 4.0).unwrap(),
            {
                let t = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
                t.transform(&Transform::translation(4.0, 0.0, 0.0).unwrap())
                    .unwrap();
                t
            },
        ),
        (
            "small block strictly inside a big one",
            Body::create_solid_block(20.0, 20.0, 20.0).unwrap(),
            {
                let t = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
                t.transform(&Transform::translation(0.0, 0.0, 9.0).unwrap())
                    .unwrap();
                t
            },
        ),
        (
            "disjoint",
            Body::create_solid_block(4.0, 4.0, 4.0).unwrap(),
            {
                let t = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
                t.transform(&Transform::translation(100.0, 0.0, 0.0).unwrap())
                    .unwrap();
                t
            },
        ),
    ];
    for (label, x, y) in cases {
        let any = x.entity().clashes_with(y.entity());
        let recs = x.entity().clash_records(y.entity());
        match (any, recs) {
            (Ok(v), Ok(rs)) => {
                let mut tokens: Vec<i32> = rs.iter().map(|r| r.clash_type_token).collect();
                tokens.sort_unstable();
                tokens.dedup();
                println!(
                    "  {label:42} clashes={v:5} n_records={:3} distinct clash_type tokens={tokens:?}",
                    rs.len()
                );
            }
            (a, b) => println!("  {label:42} ERROR any={a:?} recs_err={}", b.is_err()),
        }
    }

    println!("\n== done");
}
