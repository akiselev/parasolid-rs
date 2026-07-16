// Dynamic probe: pin faceting tolerance -> mesh-density behavior (FACETING.md [LOW]).
use parasolid::*;
use parasolid_sys::*;

fn facet_n(tag: i32, s_tol: Option<f64>, s_ang: Option<f64>) -> (i32, i32) {
    let mut c: PK_TOPOL_facet_mesh_2_o_t = unsafe { std::mem::zeroed() };
    c.o_t_version = 5;
    c.shape = 20502; c.match_ = 20522; c.density = 20540; c.cull = 20560;
    c.max_facet_sides = 3; c.ignore = 22111; c.ignore_scope = 22131; c.wire_edges = 22140;
    if let Some(t) = s_tol { c.is_surface_plane_tol = 1; c.surface_plane_tol = t; }
    if let Some(a) = s_ang { c.is_surface_plane_ang = 1; c.surface_plane_ang = a; }
    let mut ch: PK_TOPOL_facet_choice_2_o_t = unsafe { std::mem::zeroed() };
    ch.o_t_version = 5; ch.smp = 24110; ch.consistent_parms = 22510; ch.report_pts_off_topol = 24570;
    let mut opts = PK_TOPOL_facet_2_o_t { control: c, choice: ch };
    let mut result: PK_TOPOL_facet_2_r_t = unsafe { std::mem::zeroed() };
    let mut topol = tag;
    let code = unsafe { PK_TOPOL_facet_2(1, &mut topol, std::ptr::null_mut(), &mut opts, &mut result) };
    let n = result.n_facets;
    unsafe { let _ = PK_TOPOL_facet_2_r_f(&mut result); }
    (code, n)
}

fn main() {
    let cfg = SessionConfig::new().check_arguments(false).frustrum(
        FrustrumConfig::new().base_dir(
            "/tmp/claude-1000/-home-dev-projects-parasolid-re/ccf7881f-5d90-471c-9b56-208ecdb81733/scratchpad/xt",
        ),
    );
    let _s = Session::start(cfg).unwrap();
    let sphere = Body::create_solid_sphere(10.0).unwrap();
    let tag = sphere.tag();

    // sphere R=10: surface_plane_tol = max sagitta => max edge L≈√(8Rt) => tris ∝ 1/t
    println!("== sphere r=10, SURFACE_PLANE_TOL sweep (loose angle; expect n ∝ 1/tol) ==");
    for t in [4.0, 2.0, 1.0, 0.5, 0.25, 0.1, 0.05, 0.025, 0.01] {
        let (code, n) = facet_n(tag, Some(t), Some(6.3));
        println!("  surf_plane_tol={:7.3}  ->  {:7} tris   (n*tol={:.0})  code={}", t, n, n as f64 * t, code);
    }
    // surface_plane_ang = max normal turn => tris ∝ 1/ang²
    println!("\n== sphere r=10, SURFACE_PLANE_ANG sweep (loose tol; expect n ∝ 1/ang²) ==");
    for a in [1.0_f64, 0.5, 0.25, 0.1, 0.05] {
        let (code, n) = facet_n(tag, Some(1e6), Some(a));
        println!("  surf_ang={:6.3} rad ({:5.1}°)  ->  {:7} tris   (n*ang²={:.1})  code={}", a, a.to_degrees(), n, n as f64 * a * a, code);
    }
    let (code, n) = facet_n(tag, None, None);
    println!("\n== default kernel tolerances  ->  {} tris   code={} ==", n, code);
    let blk = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
    let (bc, bn) = facet_n(blk.tag(), Some(0.001), Some(0.001));
    println!("== flat block (tight tols)  ->  {} tris   code={}  (expect 12: flat needs no refinement) ==", bn, bc);
}
