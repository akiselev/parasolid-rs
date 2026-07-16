//! Confirm the angular coincidence threshold tracks PK_SESSION angle precision.
use parasolid::*;

fn two_planes_angle(theta: f64) -> usize {
    let a = Surf::plane(Axis2::new(Vec3::zero(), Vec3::new(0.0,0.0,1.0), Vec3::new(1.0,0.0,0.0))).unwrap();
    let nb = Vec3::new(0.0, theta.sin(), theta.cos());
    let b = Surf::plane(Axis2::new(Vec3::zero(), nb, Vec3::new(1.0,0.0,0.0))).unwrap();
    a.intersect(&b).map(|si| si.curves.len()).unwrap_or(0)
}

// largest theta that still returns NO line (empty) i.e. below-threshold; report bracket
fn threshold() -> (f64, f64) {
    let mut lo = 1e-15f64; // empty (parallel)
    let mut hi = 1e-2f64;  // line
    for _ in 0..80 {
        let m = (lo.ln()*0.5 + hi.ln()*0.5).exp();
        if two_planes_angle(m) == 0 { lo = m; } else { hi = m; }
        if hi/lo - 1.0 < 1e-9 { break; }
    }
    (lo, hi)
}

fn main() {
    let cfg = SessionConfig::new().check_arguments(false).frustrum(
        FrustrumConfig::new().base_dir("/tmp/claude-1000/-home-dev-projects-parasolid-re/ccf7881f-5d90-471c-9b56-208ecdb81733/scratchpad/xt"));
    let s = Session::start(cfg).unwrap();
    for &ap in &[1e-11_f64, 1e-9, 1e-13, 1e-11] {
        unsafe { parasolid_sys::PK_SESSION_set_angle_precision(ap); }
        let mut got = 0.0; unsafe { parasolid_sys::PK_SESSION_ask_angle_precision(&mut got); }
        let (lo, hi) = threshold();
        println!("angle_precision set={:.0e} ask={:.3e} -> plane-tilt line-appears at theta ~ {:.3e}..{:.3e}", ap, got, lo, hi);
    }
    let _ = s;
}
