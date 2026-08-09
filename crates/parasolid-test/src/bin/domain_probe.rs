//! Parameter-domain probe (Stage 4): intervals, uv-boxes, periodicity, seams.
//!
//! `PK_PARAM_sf_t`'s layout is known (40 bytes: `range`@0, `extent`@16,
//! `form`@20, `periodic`@24, `convexity`@28, `closed`@32) and `periodic` is
//! decoded (18020 no / 18021 yes / 18022 seamed). `extent`, `form` and
//! `convexity` carry tokens that appear in **no** catalog and whose journal
//! strings are absent from this build — so they get recovered the only way
//! left: observe them across every analytic family and let the pattern name
//! them. Nothing is invented; unexplained values stay printed as raw numbers.
//!
//! The probe also answers the questions that actually decide the domain type:
//! is a seam an identification of two parameter values (same position *and*
//! same derivatives), and how does a pole show up in the box?
//!
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/domain_probe.exe

use parasolid::*;
use parasolid_sys::*;

fn basis() -> Axis2 {
    Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    )
}

fn periodic_name(t: i32) -> &'static str {
    match t {
        18020 => "no",
        18021 => "yes",
        18022 => "seamed",
        _ => "??",
    }
}

fn dump_curve_param(label: &str, c: &Curve) {
    let mut sf = unsafe { std::mem::zeroed::<PK_PARAM_sf_t>() };
    let rc = unsafe { PK_CURVE_ask_param(c.tag(), &mut sf) };
    // find_length returns a nominal length AND a range bounding the true arc
    // length — a conservative enclosure, not just a number.
    let len = {
        let mut l = 0.0f64;
        let mut range = PK_INTERVAL_t {
            low: 0.0,
            high: 0.0,
        };
        let iv = PK_INTERVAL_t {
            low: sf.range.low,
            high: sf.range.high,
        };
        let lrc = unsafe { PK_CURVE_find_length(c.tag(), iv, &mut l, &mut range) };
        if lrc == PK_ERROR_no_errors {
            format!(
                "{l:.6} in [{:.6},{:.6}] (w={:.2e})",
                range.low,
                range.high,
                range.high - range.low
            )
        } else {
            format!("err {lrc}")
        }
    };
    println!(
        "  {label:18} rc={rc} range=[{:.4},{:.4}] extent={} form={} periodic={}({}) curve_class={} closed={} len={len}",
        sf.range.low,
        sf.range.high,
        sf.extent,
        sf.form,
        sf.periodic,
        periodic_name(sf.periodic),
        sf.curve_class,
        sf.closed & 0xff
    );
}

fn dump_surf_params(label: &str, s: &Surf) {
    let mut sf: [PK_PARAM_sf_t; 2] = unsafe { std::mem::zeroed() };
    let rc = unsafe { PK_SURF_ask_params(s.tag(), sf.as_mut_ptr()) };
    for (dir, p) in [("u", sf[0]), ("v", sf[1])] {
        println!(
            "  {label:12} {dir}: rc={rc} range=[{:9.4},{:9.4}] extent={} form={} periodic={}({}) curve_class={} closed={}",
            p.range.low,
            p.range.high,
            p.extent,
            p.form,
            p.periodic,
            periodic_name(p.periodic),
            p.curve_class,
            p.closed & 0xff
        );
    }
}

/// Is the seam an identification: same position AND same derivatives?
fn check_seam(label: &str, s: &Surf, u: f64, period: f64, v: f64) {
    let a = s.eval_jet(u, v, 1, 1, false).expect("jet a");
    let b = s.eval_jet(u + period, v, 1, 1, false).expect("jet b");
    let dp = {
        let (p, q) = (a.position(), b.position());
        ((p.x - q.x).powi(2) + (p.y - q.y).powi(2) + (p.z - q.z).powi(2)).sqrt()
    };
    let ddu = {
        let (p, q) = (a.du().unwrap(), b.du().unwrap());
        ((p.x - q.x).powi(2) + (p.y - q.y).powi(2) + (p.z - q.z).powi(2)).sqrt()
    };
    let ddv = {
        let (p, q) = (a.dv().unwrap(), b.dv().unwrap());
        ((p.x - q.x).powi(2) + (p.y - q.y).powi(2) + (p.z - q.z).powi(2)).sqrt()
    };
    println!("  {label:22} |dP|={dp:.3e}  |d(du)|={ddu:.3e}  |d(dv)|={ddv:.3e}");
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    let b = basis();

    println!("== curve parameterisation ==");
    dump_curve_param(
        "line",
        &Curve::line(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)).unwrap(),
    );
    dump_curve_param("circle r=3", &Curve::circle(b, 3.0).unwrap());
    dump_curve_param("ellipse 5x2", &Curve::ellipse(b, 5.0, 2.0).unwrap());

    println!("\n== surface parameterisation ==");
    dump_surf_params("plane", &Surf::plane(b).unwrap());
    dump_surf_params("cylinder r=2", &Surf::cylinder(b, 2.0).unwrap());
    dump_surf_params("sphere r=4", &Surf::sphere(b, 4.0).unwrap());
    dump_surf_params("cone r=3 a=.5", &Surf::cone(b, 3.0, 0.5).unwrap());
    dump_surf_params("torus 5/1.5", &Surf::torus(b, 5.0, 1.5).unwrap());

    println!("\n== seam identification: is u ≡ u+period exactly? ==");
    let tau = std::f64::consts::TAU;
    check_seam(
        "cylinder u seam",
        &Surf::cylinder(b, 2.0).unwrap(),
        0.0,
        tau,
        1.0,
    );
    check_seam(
        "sphere u seam",
        &Surf::sphere(b, 4.0).unwrap(),
        0.0,
        tau,
        0.3,
    );
    check_seam(
        "torus u seam",
        &Surf::torus(b, 5.0, 1.5).unwrap(),
        0.0,
        tau,
        0.4,
    );
    check_seam(
        "torus v seam",
        &Surf::torus(b, 5.0, 1.5).unwrap(),
        0.4,
        tau,
        0.0,
    );
    println!("  (a nonzero |dP| would mean the seam is NOT a clean identification)");

    println!("\n== poles: how does a degenerate boundary appear? ==");
    let sph = Surf::sphere(b, 4.0).unwrap();
    let uvb = sph.uvbox().unwrap();
    println!(
        "  sphere uvbox: u=[{:.6},{:.6}] v=[{:.6},{:.6}]  (v ends are the poles)",
        uvb.u_min, uvb.u_max, uvb.v_min, uvb.v_max
    );
    for (label, v) in [
        ("v_min (south pole)", uvb.v_min),
        ("v_max (north pole)", uvb.v_max),
        ("v mid", 0.0),
    ] {
        let j = sph.eval_jet(0.9, v, 1, 1, false).unwrap();
        let du = j.du().unwrap();
        println!(
            "    {label:20} |du|={:.3e} singular={}",
            (du.x * du.x + du.y * du.y + du.z * du.z).sqrt(),
            j.is_singular()
        );
    }
    // Do two different u values at the pole give the same point?
    let p1 = sph.eval(0.0, uvb.v_max).unwrap();
    let p2 = sph.eval(2.0, uvb.v_max).unwrap();
    println!(
        "    pole collapses in u: |P(0,vmax) - P(2,vmax)| = {:.3e}",
        ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2) + (p1.z - p2.z).powi(2)).sqrt()
    );

    println!("\n== face domains: tight or conservative? ==");
    // A full cylinder body: the side face is periodic in u; the caps are planar.
    let body = Body::create_solid_cylinder(2.0, 6.0).unwrap();
    for (idx, face) in body.faces().unwrap().iter().enumerate() {
        let st = face
            .surface_type()
            .map(|t| format!("{t:?}"))
            .unwrap_or_default();
        let mut uvbox = PK_UVBOX_t { param: [0.0; 4] };
        let rc = unsafe { PK_FACE_find_uvbox(face.tag(), &mut uvbox) };
        let mut u_per: PK_PARAM_periodic_t = 0;
        let mut v_per: PK_PARAM_periodic_t = 0;
        let prc = unsafe { PK_FACE_is_periodic(face.tag(), &mut u_per, &mut v_per) };
        // is_uvbox takes no input box: it reports whether the face IS a
        // parametric rectangle and hands back that rectangle. The reference is
        // explicit that this is one-sided — it may fail to detect a rectangle,
        // but never claims one that is not.
        let mut is_uvbox: PK_LOGICAL_t = PK_LOGICAL_false;
        let mut tight = PK_UVBOX_t { param: [0.0; 4] };
        let irc = unsafe { PK_FACE_is_uvbox(face.tag(), &mut is_uvbox, &mut tight) };
        println!(
            "  face[{idx}] {st:10} uvbox(rc={rc})=[{:.4},{:.4}]x[{:.4},{:.4}]  periodic(rc={prc}) u={} v={}  is_uvbox(rc={irc})={is_uvbox}",
            uvbox.param[0],
            uvbox.param[2],
            uvbox.param[1],
            uvbox.param[3],
            periodic_name(u_per),
            periodic_name(v_per),
        );
        if is_uvbox == PK_LOGICAL_true {
            println!(
                "             is_uvbox rect = [{:.4},{:.4}]x[{:.4},{:.4}]",
                tight.param[0], tight.param[2], tight.param[1], tight.param[3]
            );
        }
    }

    println!("\n== done");
}
