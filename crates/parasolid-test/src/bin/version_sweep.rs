//! Probe the accepted `o_t_version` ceiling for every options struct the
//! high-level wrapper actually uses, and detect which versions actually READ
//! the later fields.
//!
//! Method per struct: call the real entry point with version N and a deliberate
//! GARBAGE token in a late enum field.
//!   - rc 5022 (o_t_version_unknown) => N is above the ceiling
//!   - rc 5000 (not_implemented)     => N is known but unusable
//!   - rc 5014 (field_of_wrong_type) => N is accepted AND the field is READ
//!   - rc 0 / other                  => N accepted, field NOT read (version too low)
//! The best version is the highest one that both works and reads its fields.

use parasolid::*;
use parasolid_sys::*;

fn ax(o: Vec3) -> Axis2 {
    Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
}

fn verdict(rc: i32) -> &'static str {
    match rc {
        5022 => "above ceiling (unknown)",
        5000 => "known but not implemented",
        5014 => "ACCEPTED + field READ",
        0 => "accepted, field ignored",
        _ => "accepted (other rc)",
    }
}

/// Sweep one entry point. `call(version, garbage)` returns the raw rc.
fn sweep(name: &str, max: i32, mut call: impl FnMut(i32, bool) -> i32) {
    println!("\n-- {name}");
    let mut best = None;
    for v in 1..=max {
        let clean = call(v, false);
        let dirty = call(v, true);
        let reads = dirty == 5014 && clean != 5014;
        if clean == 0 || (clean != 5022 && clean != 5000) {
            if reads {
                best = Some(v);
            } else if best.is_none() {
                best = Some(v);
            }
        }
        println!(
            "   v{v}: clean rc={clean:<6} garbage rc={dirty:<6}  {}{}",
            verdict(clean),
            if reads { "  <-- reads late fields" } else { "" }
        );
        if clean == 5022 && v > 1 {
            break;
        }
    }
    println!("   => recommended version: {:?}", best);
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    let b = ax(Vec3::new(0.0, 0.0, 0.0));
    let cyl = Surf::cylinder(b, 5.0).unwrap();
    let pl = Surf::plane(ax(Vec3::new(0.0, 0.0, 3.0))).unwrap();
    let circ = Curve::circle(b, 3.0).unwrap();
    let circ2 = Curve::circle(ax(Vec3::new(1.0, 0.0, 0.0)), 3.0).unwrap();
    let block = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();

    println!("=== SSI family (Stage 7) ===");

    sweep("PK_SURF_intersect_surf_o_t", 6, |v, garbage| {
        let mut o: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
        o.o_t_version = v;
        o.mixed_curve_category = if garbage {
            12345
        } else {
            PK_mixed_intersection_classic_c
        };
        let (mut nv, mut nc) = (0, 0);
        let (mut a, mut c, mut d, mut e) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let rc = unsafe {
            PK_SURF_intersect_surf(
                cyl.tag(),
                pl.tag(),
                &o,
                &mut nv,
                &mut a,
                &mut nc,
                &mut c,
                &mut d,
                &mut e,
            )
        };
        unsafe {
            for p in [
                a as *mut std::os::raw::c_void,
                c as *mut _,
                d as *mut _,
                e as *mut _,
            ] {
                if !p.is_null() {
                    let _ = PK_MEMORY_free(p);
                }
            }
        }
        rc
    });

    sweep("PK_SURF_intersect_curve_o_t", 6, |v, garbage| {
        let mut o: PK_SURF_intersect_curve_o_t = unsafe { std::mem::zeroed() };
        o.o_t_version = v;
        o.have_box = if garbage { 12345 } else { PK_LOGICAL_false };
        let mut nv = 0;
        let (mut a, mut c, mut d, mut e) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let iv = PK_INTERVAL_t {
            low: 0.0,
            high: std::f64::consts::TAU,
        };
        let rc = unsafe {
            PK_SURF_intersect_curve(
                cyl.tag(),
                circ.tag(),
                iv,
                &o,
                &mut nv,
                &mut a,
                &mut c,
                &mut d,
                &mut e,
            )
        };
        unsafe {
            for p in [
                a as *mut std::os::raw::c_void,
                c as *mut _,
                d as *mut _,
                e as *mut _,
            ] {
                if !p.is_null() {
                    let _ = PK_MEMORY_free(p);
                }
            }
        }
        rc
    });

    sweep("PK_CURVE_intersect_curve_o_t", 6, |v, garbage| {
        let mut o: PK_CURVE_intersect_curve_o_t = unsafe { std::mem::zeroed() };
        o.o_t_version = v;
        o.have_box = if garbage { 12345 } else { PK_LOGICAL_false };
        let mut nv = 0;
        let (mut a, mut c, mut d, mut e) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let iv = PK_INTERVAL_t {
            low: 0.0,
            high: std::f64::consts::TAU,
        };
        let rc = unsafe {
            PK_CURVE_intersect_curve(
                circ.tag(),
                iv,
                circ2.tag(),
                iv,
                &o,
                &mut nv,
                &mut a,
                &mut c,
                &mut d,
                &mut e,
            )
        };
        unsafe {
            for p in [
                a as *mut std::os::raw::c_void,
                c as *mut _,
                d as *mut _,
                e as *mut _,
            ] {
                if !p.is_null() {
                    let _ = PK_MEMORY_free(p);
                }
            }
        }
        rc
    });

    println!("\n=== other live oracle paths ===");

    sweep("PK_TOPOL_eval_mass_props_o_t", 9, |v, garbage| {
        #[repr(C)]
        struct O {
            ver: i32,
            mass: i32,
            periphery: i32,
            bound: i32,
            single: u8,
        }
        let o = O {
            ver: v,
            mass: if garbage { 12345 } else { 0x36b4 },
            periphery: 0x36b6,
            bound: 0x36b7,
            single: 1,
        };
        let (mut amount, mut mass, mut per) = (0.0, 0.0, 0.0);
        let (mut cg, mut mi) = ([0.0; 3], [0.0; 9]);
        let tag = block.tag();
        unsafe {
            PK_TOPOL_eval_mass_props(
                1,
                &tag,
                0.99,
                &o as *const O as *const PK_TOPOL_eval_mass_props_o_t,
                &mut amount,
                &mut mass,
                &mut cg,
                &mut mi,
                &mut per,
            )
        }
    });

    sweep("PK_TOPOL_clash_o_t", 6, |v, garbage| {
        let mut o = PK_TOPOL_clash_o_t {
            o_t_version: v,
            find_all: 1,
            find_intersect: 1,
            ..PK_TOPOL_clash_o_t::default()
        };
        if garbage {
            o.n_op_ex = -12345;
        }
        let (mut t1, mut t2) = ([block.tag()], [block.tag()]);
        let (mut f1, mut f2) = ([PK_ENTITY_null], [PK_ENTITY_null]);
        let mut n = 0;
        let mut cl = std::ptr::null_mut();
        let rc = unsafe {
            PK_TOPOL_clash(
                1,
                t1.as_mut_ptr(),
                f1.as_mut_ptr(),
                1,
                t2.as_mut_ptr(),
                f2.as_mut_ptr(),
                &mut o,
                &mut n,
                &mut cl,
            )
        };
        unsafe {
            if !cl.is_null() {
                let _ = PK_MEMORY_free(cl as *mut std::os::raw::c_void);
            }
        }
        rc
    });

    sweep("PK_CURVE_find_box_o_t", 6, |v, garbage| {
        let o = PK_CURVE_find_box_o_t {
            o_t_version: v,
            have_interval: if garbage { 12345 } else { PK_LOGICAL_false },
            interval: PK_INTERVAL_t {
                low: 0.0,
                high: 1.0,
            },
        };
        let mut bx = PK_BOX_t { coord: [0.0; 6] };
        unsafe { PK_CURVE_find_box(circ.tag(), &o, &mut bx) }
    });

    sweep("PK_SURF_find_box_o_t", 6, |v, garbage| {
        let o = PK_SURF_find_box_o_t {
            o_t_version: v,
            have_uvbox: if garbage { 12345 } else { PK_LOGICAL_true },
            uvbox: PK_UVBOX_t {
                param: [0.0, 0.0, 6.28, 6.0],
            },
        };
        let mut bx = PK_BOX_t { coord: [0.0; 6] };
        unsafe { PK_SURF_find_box(cyl.tag(), &o, &mut bx) }
    });

    sweep("PK_TRANSF_classify_o_t", 6, |v, garbage| {
        let t = Transform::translation(1.0, 2.0, 3.0).unwrap();
        let o = PK_TRANSF_classify_o_t {
            o_t_version: v,
            diagnostics: if garbage {
                12345
            } else {
                PK_TRANSF_diagnostics_none_c
            },
        };
        let mut r: PK_TRANSF_classify_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe { PK_TRANSF_classify(t.tag(), &o, &mut r) };
        unsafe { PK_TRANSF_classify_r_f(&mut r) };
        rc
    });

    println!("\n== done");
}
