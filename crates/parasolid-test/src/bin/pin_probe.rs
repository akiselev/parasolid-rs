// Dynamic probe: pin RE items against live Parasolid.
use parasolid::*;

fn main() {
    let cfg = SessionConfig::new().check_arguments(false).frustrum(
        FrustrumConfig::new().base_dir(
            "/tmp/claude-1000/-home-dev-projects-parasolid-re/ccf7881f-5d90-471c-9b56-208ecdb81733/scratchpad/xt",
        ),
    );
    let _s = Session::start(cfg).unwrap();

    // ---- point classification (confirmed earlier: keep a compact check) ----
    let b = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    let bb = b.bounding_box().unwrap();
    let (cx, cy, cz) = ((bb.min.x + bb.max.x) / 2.0, (bb.min.y + bb.max.y) / 2.0, (bb.min.z + bb.max.z) / 2.0);
    println!("== point classification (block {:?}..{:?}) ==", (bb.min.x,bb.min.y,bb.min.z),(bb.max.x,bb.max.y,bb.max.z));
    for (lbl, x, y, z) in [
        ("interior", cx+0.7, cy+0.3, cz+0.9),
        ("on-face", bb.max.x, cy+0.3, cz+0.9),
        ("on-vertex", bb.max.x, bb.max.y, bb.max.z),
        ("+1e-6 inside", bb.max.x-1e-6, cy+0.3, cz+0.9),
        ("+1e-6 outside", bb.max.x+1e-6, cy+0.3, cz+0.9),
    ] { println!("  {:14} -> {:?}", lbl, b.contains_point(Vec3::new(x,y,z))); }

    // ---- the quadric-SSI ORACLE: orphan surface ∩ orphan surface (Surf::intersect) ----
    println!("\n== quadric-SSI oracle (orphan surf ∩ surf) ==");
    let o = Vec3::zero(); let z = Vec3::new(0.0,0.0,1.0); let x = Vec3::new(1.0,0.0,0.0); let y = Vec3::new(0.0,1.0,0.0);
    let report = |label: &str, r: PsResult<SurfIntersection>| match r {
        Ok(si) => {
            let types: Vec<String> = si.curves.iter().map(|c| format!("{:?}", c.curve.curve_type())).collect();
            println!("  {:52} -> {} pts, {} curves {:?}", label, si.points.len(), si.curves.len(), types);
        }
        Err(e) => println!("  {:52} -> ERR {}", label, e),
    };
    // exact-conic rows
    let sph = Surf::sphere(Axis2::new(o, z, x), 8.0).unwrap();
    let pl_z3 = Surf::plane(Axis2::new(Vec3::new(0.0,0.0,3.0), z, x)).unwrap();
    report("sphere(r=8) ∩ plane(z=3)   [expect Circle]", sph.intersect(&pl_z3));
    let cyl_z = Surf::cylinder(Axis2::new(o, z, x), 5.0).unwrap();
    let pl_z0 = Surf::plane(Axis2::new(o, z, x)).unwrap();
    report("cylinder(Z,r=5) ∩ plane(z=0) [expect Circle]", cyl_z.intersect(&pl_z0));
    let nrm = { let l = (2.0_f64).sqrt(); Vec3::new(1.0/l, 0.0, 1.0/l) }; let pl_tilt = Surf::plane(Axis2::new(o, nrm, y)).unwrap();
    report("cyl(Z,r=5) ∩ plane(45deg,normalized) [expect Ellipse]", cyl_z.intersect(&pl_tilt));
    let nrm2 = { let l=(1.0_f64+16.0).sqrt(); Vec3::new(1.0/l,0.0,4.0/l) }; let pl_t2 = Surf::plane(Axis2::new(o, nrm2, y)).unwrap();
    report("cyl(Z,r=5) ∩ plane(shallow 14deg) [expect Ellipse]", cyl_z.intersect(&pl_t2));
    let sphA = Surf::sphere(Axis2::new(o, z, x), 8.0).unwrap();
    let sphB = Surf::sphere(Axis2::new(Vec3::new(10.0,0.0,0.0), z, x), 6.0).unwrap();
    report("sphere(o,r=8) ∩ sphere((10,0,0),r=6) [expect Circle]", sphA.intersect(&sphB));
    // quartic rows
    let cyl_x = Surf::cylinder(Axis2::new(o, x, y), 5.0).unwrap();
    report("cyl(Z,r=5) ∩ cyl(X,r=5)    [Steinmetz, expect Icurve/quartic]", cyl_z.intersect(&cyl_x));
    let cyl_x4 = Surf::cylinder(Axis2::new(o, x, y), 4.0).unwrap();
    report("cyl(Z,r=5) ∩ cyl(X,r=4)    [unequal, expect Icurve/quartic]", cyl_z.intersect(&cyl_x4));
    let sph6 = Surf::sphere(Axis2::new(Vec3::new(0.0,0.0,4.0), z, x), 6.0).unwrap();
    report("cyl(Z,r=5) ∩ sphere((0,0,4),r=6) [expect Icurve/quartic]", cyl_z.intersect(&sph6));
}
