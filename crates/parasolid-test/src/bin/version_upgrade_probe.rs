//! Probe the accepted `o_t_version` ceiling for every options struct the
//! high-level wrapper uses, per the "Track the latest API" policy in CLAUDE.md
//! and the procedure in `docs/option-version-protocol.md`.
//!
//! Method per struct: call the real entry point at version N twice — once with
//! a legal token in the LAST field this crate models, once with a deliberate
//! GARBAGE token in that same field — and compare the return codes.
//!
//!   rc 5022 (o_t_version_unknown)  => N is above the ceiling
//!   rc 5043 (o_t_version_incorrect)=> N exists but this entry point refuses it
//!   rc 5000 (not_implemented)      => N known but unusable
//!   rc 5014 (field_of_wrong_type)  => N accepted AND that field is READ
//!   rc 0 / other                   => N accepted, field NOT read
//!
//! The garbage field is deliberately the *last* field of our binding, so a
//! 5014 proves the kernel migrates the whole struct we hand it — i.e. our
//! layout reaches at least that far and version N is safe to stamp.
//!
//! Run:
//!   cargo build -p parasolid-test --target x86_64-pc-windows-gnu
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/version_upgrade_probe.exe

use parasolid::*;
use parasolid_sys::*;
use std::os::raw::{c_int, c_void};

const GARBAGE: c_int = 987654;

fn ax(o: Vec3) -> Axis2 {
    Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
}

fn verdict(rc: i32) -> &'static str {
    match rc {
        5022 => "above ceiling (version_unknown)",
        5043 => "version_incorrect",
        5000 => "known but not implemented",
        5014 => "field_of_wrong_type",
        0 => "ok",
        _ => "other",
    }
}

fn accepted(rc: i32) -> bool {
    !matches!(rc, 5022 | 5043 | 5000)
}

/// Sweep one entry point over versions `1..=max`.
///
/// `call(version, garbage)` must build FRESH operands each time (an accepted
/// version runs the real operation, and destructive ops consume their input).
fn sweep(name: &str, max: i32, mut call: impl FnMut(i32, bool) -> i32) {
    println!("\n-- {name}");
    let mut ceiling = 0;
    let mut best_read = 0;
    use std::io::Write;
    let trace = std::env::var("PROBE_TRACE").is_ok();
    for v in 1..=max {
        if trace {
            print!("   v{v} clean...");
            let _ = std::io::stdout().flush();
        }
        let clean = call(v, false);
        if trace {
            print!(" rc={clean}; garbage...");
            let _ = std::io::stdout().flush();
        }
        let dirty = call(v, true);
        if trace {
            println!(" rc={dirty}");
        }
        let reads = accepted(clean) && dirty == 5014 && clean != 5014;
        if accepted(clean) {
            ceiling = v;
            if reads {
                best_read = v;
            }
        }
        println!(
            "   v{v:<3} clean={clean:<6} ({:<32}) garbage={dirty:<6}{}",
            verdict(clean),
            if reads { "  <-- LAST FIELD READ" } else { "" }
        );
    }
    println!("   => ceiling={ceiling}  highest-version-reading-our-last-field={best_read}");
}

fn free_all(ps: &[*mut c_void]) {
    unsafe {
        for &p in ps {
            if !p.is_null() {
                let _ = PK_MEMORY_free(p);
            }
        }
    }
}

// ============================================================================
// group 1 — intersection (SSI / SCI / CCI / face variants)
// ============================================================================

fn group_intersect() {
    println!("\n=== group 1: intersection ===");

    sweep("PK_SURF_intersect_surf_o_t", 8, |v, g| {
        let cyl = Surf::cylinder(ax(Vec3::new(0.0, 0.0, 0.0)), 5.0).unwrap();
        let pl = Surf::plane(ax(Vec3::new(0.0, 0.0, 3.0))).unwrap();
        let mut o: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
        o.o_t_version = v;
        o.mixed_curve_category = PK_mixed_intersection_classic_c;
        o._use_reserved = if g { GARBAGE } else { 0 };
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
        free_all(&[a as _, c as _, d as _, e as _]);
        rc
    });

    // Second sweep of SSI targeting `mixed_curve_category` (the field we already
    // rely on) rather than the trailing reserved slot.
    sweep(
        "PK_SURF_intersect_surf_o_t[mixed_curve_category]",
        8,
        |v, g| {
            let cyl = Surf::cylinder(ax(Vec3::new(0.0, 0.0, 0.0)), 5.0).unwrap();
            let pl = Surf::plane(ax(Vec3::new(0.0, 0.0, 3.0))).unwrap();
            let mut o: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
            o.o_t_version = v;
            o.mixed_curve_category = if g {
                GARBAGE
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
            free_all(&[a as _, c as _, d as _, e as _]);
            rc
        },
    );

    sweep("PK_SURF_intersect_curve_o_t", 8, |v, g| {
        let cyl = Surf::cylinder(ax(Vec3::new(0.0, 0.0, 0.0)), 5.0).unwrap();
        let circ = Curve::circle(ax(Vec3::new(0.0, 0.0, 0.0)), 3.0).unwrap();
        let mut o: PK_SURF_intersect_curve_o_t = unsafe { std::mem::zeroed() };
        o.o_t_version = v;
        o._interest_reserved = if g { GARBAGE } else { 0 };
        let mut n = 0;
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
                &mut n,
                &mut a,
                &mut c,
                &mut d,
                &mut e,
            )
        };
        free_all(&[a as _, c as _, d as _, e as _]);
        rc
    });

    sweep("PK_CURVE_intersect_curve_o_t", 8, |v, g| {
        let circ = Curve::circle(ax(Vec3::new(0.0, 0.0, 0.0)), 3.0).unwrap();
        let circ2 = Curve::circle(ax(Vec3::new(1.0, 0.0, 0.0)), 3.0).unwrap();
        let o = PK_CURVE_intersect_curve_o_t {
            o_t_version: v,
            have_box: if g { GARBAGE } else { PK_LOGICAL_false },
            r#box: PK_BOX_t { coord: [0.0; 6] },
            common_surf: PK_ENTITY_null,
        };
        let iv = PK_INTERVAL_t {
            low: 0.0,
            high: std::f64::consts::TAU,
        };
        let mut n = 0;
        let (mut a, mut c, mut d, mut e) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let rc = unsafe {
            PK_CURVE_intersect_curve(
                circ.tag(),
                iv,
                circ2.tag(),
                iv,
                &o,
                &mut n,
                &mut a,
                &mut c,
                &mut d,
                &mut e,
            )
        };
        free_all(&[a as _, c as _, d as _, e as _]);
        rc
    });

    sweep("PK_FACE_intersect_face_o_t", 8, |v, g| {
        let b1 = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let b2 = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let f1 = b1.faces().unwrap()[0].tag();
        let f2 = b2.faces().unwrap()[1].tag();
        let mut o: PK_FACE_intersect_face_o_t = unsafe { std::mem::zeroed() };
        o.o_t_version = v;
        o.mixed_curve_category = PK_mixed_intersection_classic_c;
        o._use_reserved = if g { GARBAGE } else { 0 };
        let (mut nv, mut nc) = (0, 0);
        let (mut a, mut c, mut d, mut e) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let rc = unsafe {
            PK_FACE_intersect_face(f1, f2, &o, &mut nv, &mut a, &mut nc, &mut c, &mut d, &mut e)
        };
        free_all(&[a as _, c as _, d as _, e as _]);
        rc
    });

    sweep("PK_FACE_intersect_surf_o_t", 8, |v, g| {
        let b1 = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let f1 = b1.faces().unwrap()[0].tag();
        let pl = Surf::plane(ax(Vec3::new(0.0, 0.0, 1.0))).unwrap();
        let mut o: PK_FACE_intersect_surf_o_t = unsafe { std::mem::zeroed() };
        o.o_t_version = v;
        o.mixed_curve_category = PK_mixed_intersection_classic_c;
        o._use_reserved = if g { GARBAGE } else { 0 };
        let (mut nv, mut nc) = (0, 0);
        let (mut a, mut c, mut d, mut e) = (
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let rc = unsafe {
            PK_FACE_intersect_surf(
                f1,
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
        free_all(&[a as _, c as _, d as _, e as _]);
        rc
    });
}

// ============================================================================
// group 2 — range / distance / boxes / clash
// ============================================================================

fn group_range() {
    println!("\n=== group 2: range, boxes, clash ===");

    sweep("PK_TOPOL_range_o_t", 8, |v, g| {
        let b1 = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let b2 = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
        let mut o = PK_TOPOL_range_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.opt_level = if g { GARBAGE } else { PK_range_opt_accuracy_c };
        let mut status = 0;
        let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_TOPOL_range(b1.tag(), b2.tag(), &mut o, &mut status, &mut r) }
    });

    sweep("PK_TOPOL_range_vector_o_t", 8, |v, g| {
        let b1 = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let mut o = PK_TOPOL_range_vector_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.param_entity = if g {
            GARBAGE
        } else {
            PK_range_param_entity_topol_c
        };
        let vec: PK_VECTOR_t = [20.0, 20.0, 20.0];
        let mut status = 0;
        let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_TOPOL_range_vector(b1.tag(), &vec, &mut o, &mut status, &mut r) }
    });

    sweep("PK_GEOM_range_o_t", 8, |v, g| {
        let s1 = Surf::plane(ax(Vec3::new(0.0, 0.0, 0.0))).unwrap();
        let s2 = Surf::sphere(ax(Vec3::new(0.0, 0.0, 20.0)), 3.0).unwrap();
        let mut o = PK_GEOM_range_o_t::default();
        o.o_t_version = v;
        o.opt_level = if g { GARBAGE } else { PK_range_opt_accuracy_c };
        let mut status = 0;
        let mut r: PK_range_2_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_GEOM_range(s1.tag(), s2.tag(), &mut o, &mut status, &mut r) }
    });

    sweep("PK_GEOM_range_vector_o_t", 8, |v, g| {
        let s1 = Surf::sphere(ax(Vec3::new(0.0, 0.0, 0.0)), 3.0).unwrap();
        let mut o = PK_GEOM_range_vector_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.opt_level = if g { GARBAGE } else { PK_range_opt_accuracy_c };
        let vec: PK_VECTOR_t = [10.0, 0.0, 0.0];
        let mut status = 0;
        let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_GEOM_range_vector(s1.tag(), &vec, &mut o, &mut status, &mut r) }
    });

    sweep("PK_TOPOL_clash_o_t", 8, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let mut o = PK_TOPOL_clash_o_t {
            o_t_version: v,
            find_all: 1,
            find_intersect: 1,
            ..Default::default()
        };
        if g {
            o.n_parts_with_scales = -GARBAGE;
        }
        let (mut t1, mut t2) = ([b.tag()], [b.tag()]);
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
        free_all(&[cl as _]);
        rc
    });

    // Swept far past 8 deliberately: if no version is ever rejected then the
    // entry point does not version-migrate this struct at all.
    sweep("PK_CURVE_find_box_o_t", 40, |v, g| {
        let circ = Curve::circle(ax(Vec3::new(0.0, 0.0, 0.0)), 3.0).unwrap();
        let o = PK_CURVE_find_box_o_t {
            o_t_version: v,
            have_interval: if g { GARBAGE } else { PK_LOGICAL_false },
            interval: PK_INTERVAL_t {
                low: 0.0,
                high: 1.0,
            },
        };
        let mut bx = PK_BOX_t { coord: [0.0; 6] };
        unsafe { PK_CURVE_find_box(circ.tag(), &o, &mut bx) }
    });

    sweep("PK_SURF_find_box_o_t", 40, |v, g| {
        let cyl = Surf::cylinder(ax(Vec3::new(0.0, 0.0, 0.0)), 5.0).unwrap();
        let o = PK_SURF_find_box_o_t {
            o_t_version: v,
            have_uvbox: if g { GARBAGE } else { PK_LOGICAL_true },
            uvbox: PK_UVBOX_t {
                param: [0.0, 0.0, 6.28, 6.0],
            },
        };
        let mut bx = PK_BOX_t { coord: [0.0; 6] };
        unsafe { PK_SURF_find_box(cyl.tag(), &o, &mut bx) }
    });
}

/// `PK_TOPOL_find_nabox` is swept on its own because a garbage `quality` at
/// v3 hard-crashes the process (see report). The options struct is backed by a
/// 512-byte zeroed buffer with the v2 144-byte layout written into its prefix,
/// so any read past our modelled struct lands in owned, zeroed memory rather
/// than off the end of the allocation.
fn group_nabox() {
    println!("\n=== group 2b: find_nabox (isolated — can crash) ===");
    let only: Option<i32> = std::env::args().nth(2).and_then(|s| s.parse().ok());
    for v in 1..=6 {
        if let Some(o) = only {
            if o != v {
                continue;
            }
        }
        for g in [false, true] {
            let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
            let mut tag = b.tag();
            let mut buf = [0u8; 512];
            unsafe {
                let p = buf.as_mut_ptr() as *mut PK_TOPOL_find_nabox_o_t;
                *p = PK_TOPOL_find_nabox_o_t {
                    o_t_version: v,
                    ..Default::default()
                };
                (*p).quality = if g {
                    GARBAGE
                } else {
                    PK_NABOX_quality_standard_c
                };
            }
            let mut sf: PK_NABOX_sf_t = unsafe { std::mem::zeroed() };
            println!("   v{v} garbage={g} ... calling");
            let rc = unsafe {
                PK_TOPOL_find_nabox(
                    1,
                    &mut tag,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut _,
                    &mut sf,
                )
            };
            println!("   v{v} garbage={g} rc={rc} ({})", verdict(rc));
        }
    }
}

// ============================================================================
// group 3 — mass properties
// ============================================================================

fn group_mass() {
    println!("\n=== group 3: mass properties ===");

    sweep("PK_TOPOL_eval_mass_props_o_t[single]", 10, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        // Deliberately the crate's own 5-field v1 layout, garbage in the LAST
        // modelled field (`single`, a PK_LOGICAL_t).
        let o = PK_TOPOL_eval_mass_props_o_t {
            o_t_version: v,
            mass: PK_mass_m_of_i_c,
            periphery: PK_mass_periphery_yes_c,
            bound: PK_mass_bound_no_c,
            single: if g { GARBAGE } else { PK_LOGICAL_false },
        };
        let (mut amount, mut mass, mut per) = (0.0, 0.0, 0.0);
        let (mut cg, mut mi) = ([0.0; 3], [0.0; 9]);
        let tag = b.tag();
        unsafe {
            PK_TOPOL_eval_mass_props(
                1,
                &tag,
                0.99,
                &o,
                &mut amount,
                &mut mass,
                &mut cg,
                &mut mi,
                &mut per,
            )
        }
    });

    // Same sweep with a padded, over-sized buffer so that reads past the v1
    // layout land in zeroed memory rather than off the end of the allocation.
    // This separates "the version is rejected" from "the version reads more".
    sweep("PK_TOPOL_eval_mass_props_o_t[padded 256B]", 10, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let mut buf = [0u8; 256];
        let p = buf.as_mut_ptr() as *mut c_int;
        unsafe {
            *p = v;
            *p.add(1) = PK_mass_m_of_i_c;
            *p.add(2) = PK_mass_periphery_yes_c;
            *p.add(3) = PK_mass_bound_no_c;
            *p.add(4) = if g { GARBAGE } else { PK_LOGICAL_false };
        }
        let (mut amount, mut mass, mut per) = (0.0, 0.0, 0.0);
        let (mut cg, mut mi) = ([0.0; 3], [0.0; 9]);
        let tag = b.tag();
        unsafe {
            PK_TOPOL_eval_mass_props(
                1,
                &tag,
                0.99,
                buf.as_ptr() as *const PK_TOPOL_eval_mass_props_o_t,
                &mut amount,
                &mut mass,
                &mut cg,
                &mut mi,
                &mut per,
            )
        }
    });
}

// ============================================================================
// group 4 — boolean / imprint / section / extrude
// ============================================================================

fn group_boolean() {
    println!("\n=== group 4: boolean family ===");

    // Capped at 16: v17+ hard-crashes the process (v3.. already read past our
    // 32-byte v2 layout and return not_a_logical, so the band above is moot).
    sweep("PK_BODY_boolean_o_t", 16, |v, g| {
        let target = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let tool = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
        let mut o = PK_BODY_boolean_o_t {
            o_t_version: v,
            function: PK_boolean_unite_c,
            ..Default::default()
        };
        o.fence = if g { GARBAGE } else { PK_boolean_fence_none_c };
        let tools = [tool.tag()];
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut results: PK_boolean_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            PK_BODY_boolean_2(
                target.tag(),
                1,
                tools.as_ptr(),
                &o,
                &mut tracking,
                &mut results,
            )
        };
        unsafe {
            let _ = PK_TOPOL_track_r_f(&mut tracking);
            let _ = PK_boolean_r_f(&mut results);
        }
        rc
    });
}

fn group_local() {
    println!("\n=== group 4b: imprint / section / extrude ===");

    // `PK_BODY_imprint_body` hard-crashes this kernel on a straddling block
    // pair even with NULL options, so it cannot be version-swept at all. Run
    // with a second arg of `imprint` to reproduce the crash.
    let do_imprint = std::env::args().nth(2).as_deref() == Some("imprint");

    // Baseline: does the entry point work at all, with NULL options?
    if do_imprint {
        use std::io::Write;
        let target = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let tool = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
        tool.transform(&Transform::translation(5.0, 0.0, 3.0).unwrap())
            .unwrap();
        let mut results = vec![0u8; 1024];
        print!("   NULL-options baseline ...");
        let _ = std::io::stdout().flush();
        let rc = unsafe {
            PK_BODY_imprint_body(
                target.tag(),
                tool.tag(),
                std::ptr::null_mut(),
                results.as_mut_ptr() as *mut PK_imprint_r_t,
            )
        };
        println!(" rc={rc} ({})", verdict(rc));
    }

    if do_imprint {
        sweep("PK_BODY_imprint_o_t", 8, |v, g| {
            let target = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
            let tool = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
            // Straddle the target's +x wall so the imprint has a real intersection
            // and no coincident faces (both blocks otherwise share the z=0 plane).
            tool.transform(&Transform::translation(5.0, 0.0, 3.0).unwrap())
                .unwrap();
            let mut o = PK_BODY_imprint_o_t {
                o_t_version: v,
                ..Default::default()
            };
            o.update = if g {
                GARBAGE
            } else {
                PK_boolean_update_default_c
            };
            // PK_imprint_r_t is backed by a 1 KiB zeroed buffer: with the crate's
            // exact 64-byte binding the kernel hard-crashes the process here, at
            // every version including v1 (see report — the result struct, not the
            // options struct, is the problem).
            let mut results = vec![0u8; 1024];
            let rc = unsafe {
                PK_BODY_imprint_body(
                    target.tag(),
                    tool.tag(),
                    &mut o,
                    results.as_mut_ptr() as *mut PK_imprint_r_t,
                )
            };
            unsafe {
                let _ = PK_imprint_r_f(results.as_mut_ptr() as *mut PK_imprint_r_t);
            }
            rc
        });
    }

    sweep("PK_BODY_section_o_t", 8, |v, g| {
        let target = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let pl = Surf::plane(ax(Vec3::new(0.0, 0.0, 0.0))).unwrap();
        let mut o = PK_BODY_section_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.keep_as_facet = if g {
            GARBAGE
        } else {
            PK_BODY_keep_as_facet_no_c
        };
        let mut results: PK_section_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_BODY_section_with_surf(target.tag(), pl.tag(), &o, &mut results) }
    });

    sweep("PK_BODY_extrude_o_t", 8, |v, g| {
        let profile = match Body::create_sheet_circle(3.0, ax(Vec3::new(0.0, 0.0, 0.0))) {
            Ok(b) => b,
            Err(_) => return -1,
        };
        let mk_bound = |distance: f64| PK_BODY_extrude_bound_t {
            bound: PK_bound_distance_c,
            forward: PK_LOGICAL_true,
            distance,
            entity: PK_ENTITY_null,
            nearest: PK_LOGICAL_false,
            nth_division: 0,
            side: PK_bound_side_both_c,
        };
        let o = PK_BODY_extrude_o_t {
            o_t_version: v,
            start_bound: mk_bound(0.0),
            end_bound: mk_bound(5.0),
            extruded_body: PK_ENTITY_null,
            allow_disjoint: PK_LOGICAL_false,
            consistent_params: PK_PARAM_consistent_unset_c,
            have_pline_angle: PK_LOGICAL_false,
            pline_angle: 0.0,
            keep_as_facet: if g {
                GARBAGE
            } else {
                PK_extrude_keep_as_facet_no_c
            },
        };
        let dir: PK_VECTOR1_t = [0.0, 0.0, 1.0];
        let mut body: PK_BODY_t = PK_ENTITY_null;
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut results_buf = [0u8; 64];
        let rc = unsafe {
            PK_BODY_extrude(
                profile.tag(),
                &dir,
                &o,
                &mut body,
                &mut tracking,
                results_buf.as_mut_ptr() as *mut _,
            )
        };
        unsafe {
            let _ = PK_TOPOL_track_r_f(&mut tracking);
        }
        rc
    });
}

// ============================================================================
// group 5 — file IO
// ============================================================================

fn group_fileio() {
    println!("\n=== group 5: file IO ===");

    sweep("PK_PART_transmit_o_t", 8, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let key = std::ffi::CString::new("verprobe_tx").unwrap();
        let mut o = PK_PART_transmit_o_t {
            o_t_version: v,
            transmit_format: PK_transmit_format_text_c,
            ..Default::default()
        };
        o.transmit_meshes = if g { GARBAGE } else { 0 };
        let tags = [b.tag()];
        unsafe { PK_PART_transmit(1, tags.as_ptr(), key.as_ptr(), &o) }
    });

    // Requires a file written by the transmit sweep above.
    sweep("PK_PART_receive_o_t", 8, |v, g| {
        let key = std::ffi::CString::new("verprobe_tx").unwrap();
        let mut o = PK_PART_receive_o_t {
            o_t_version: v,
            transmit_format: PK_transmit_format_text_c,
            ..Default::default()
        };
        o.receive_mixed = if g { GARBAGE } else { 0 };
        let mut n = 0;
        let mut parts = std::ptr::null_mut();
        let rc = unsafe { PK_PART_receive(key.as_ptr(), &o, &mut n, &mut parts) };
        free_all(&[parts as _]);
        rc
    });
}

// ============================================================================
// group 6 — transforms, edges, misc
// ============================================================================

fn group_misc() {
    println!("\n=== group 6: transform / edge / misc ===");

    sweep("PK_TRANSF_classify_o_t", 8, |v, g| {
        let t = Transform::translation(1.0, 2.0, 3.0).unwrap();
        let o = PK_TRANSF_classify_o_t {
            o_t_version: v,
            diagnostics: if g {
                GARBAGE
            } else {
                PK_TRANSF_diagnostics_none_c
            },
        };
        let mut r: PK_TRANSF_classify_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe { PK_TRANSF_classify(t.tag(), &o, &mut r) };
        unsafe {
            let _ = PK_TRANSF_classify_r_f(&mut r);
        }
        rc
    });

    // max_faults is an int, not an enum, so no garbage token exists — this
    // sweep measures the ceiling only.
    sweep("PK_TRANSF_check_o_t", 8, |v, _g| {
        let t = Transform::translation(1.0, 2.0, 3.0).unwrap();
        let o = PK_TRANSF_check_o_t {
            o_t_version: v,
            max_faults: 10,
        };
        let mut n = 0;
        let mut faults = std::ptr::null_mut();
        let rc = unsafe { PK_TRANSF_check(t.tag(), &o, &mut n, &mut faults) };
        free_all(&[faults as _]);
        rc
    });

    sweep("PK_EDGE_optimise_o_t", 8, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let e = b.edges().unwrap()[0].tag();
        let mut o = PK_EDGE_optimise_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.optimise_short = if g {
            GARBAGE
        } else {
            PK_EDGE_optimise_short_no_c
        };
        let mut result = 0;
        let mut achieved = 0.0f64;
        unsafe { PK_EDGE_optimise(e, &mut o, &mut result, &mut achieved) }
    });

    sweep("PK_EDGE_make_wire_body_o_t", 8, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let mut tags = [b.edges().unwrap()[0].tag()];
        let mut o = PK_EDGE_make_wire_body_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.use_nmnl_geom = if g { GARBAGE } else { PK_LOGICAL_false };
        let mut body: PK_BODY_t = PK_ENTITY_null;
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe {
            PK_EDGE_make_wire_body(1, tags.as_mut_ptr(), &mut o, &mut body, &mut tracking)
        };
        unsafe {
            let _ = PK_TOPOL_track_r_f(&mut tracking);
        }
        rc
    });
}

// ============================================================================
// group 7 — refinement: WHICH field does each accepted version actually read,
// and does raising the version change the numeric answer?
// ============================================================================

fn group_refine() {
    println!("\n=== group 7: refinement ===");

    // ---- PK_EDGE_optimise: v1 vs v2 field liveness, and the true ceiling ----
    sweep("PK_EDGE_optimise_o_t[set_max_dev]", 40, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let e = b.edges().unwrap()[0].tag();
        let mut o = PK_EDGE_optimise_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.set_max_dev = if g {
            GARBAGE
        } else {
            PK_EDGE_max_dev_edge_tol_c
        };
        let mut result = 0;
        let mut achieved = 0.0f64;
        unsafe { PK_EDGE_optimise(e, &mut o, &mut result, &mut achieved) }
    });

    sweep("PK_EDGE_optimise_o_t[optimise_short, to 40]", 40, |v, g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let e = b.edges().unwrap()[0].tag();
        let mut o = PK_EDGE_optimise_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.optimise_short = if g {
            GARBAGE
        } else {
            PK_EDGE_optimise_short_no_c
        };
        let mut result = 0;
        let mut achieved = 0.0f64;
        unsafe { PK_EDGE_optimise(e, &mut o, &mut result, &mut achieved) }
    });

    // ---- PK_EDGE_make_wire_body: which fields are live, real ceiling ----
    sweep(
        "PK_EDGE_make_wire_body_o_t[allow_disjoint, to 40]",
        40,
        |v, g| {
            let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
            let mut tags = [b.edges().unwrap()[0].tag()];
            let mut o = PK_EDGE_make_wire_body_o_t {
                o_t_version: v,
                ..Default::default()
            };
            o.allow_disjoint = if g { GARBAGE } else { PK_LOGICAL_false };
            let mut body: PK_BODY_t = PK_ENTITY_null;
            let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
            let rc = unsafe {
                PK_EDGE_make_wire_body(1, tags.as_mut_ptr(), &mut o, &mut body, &mut tracking)
            };
            unsafe {
                let _ = PK_TOPOL_track_r_f(&mut tracking);
            }
            rc
        },
    );

    // ---- PK_PART_transmit: is anything past o_t_version live at v1/v2/v3? ----
    for (label, which) in [
        ("transmit_user_fields", 0),
        ("transmit_nmnl_geometry", 1),
        ("transmit_meshes", 2),
    ] {
        sweep(&format!("PK_PART_transmit_o_t[{label}]"), 6, |v, g| {
            let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
            let key = std::ffi::CString::new("verprobe_tx").unwrap();
            let mut o = PK_PART_transmit_o_t {
                o_t_version: v,
                transmit_format: PK_transmit_format_text_c,
                ..Default::default()
            };
            if g {
                match which {
                    0 => o.transmit_user_fields = GARBAGE,
                    1 => o.transmit_nmnl_geometry = GARBAGE,
                    _ => o.transmit_meshes = GARBAGE,
                }
            }
            let tags = [b.tag()];
            unsafe { PK_PART_transmit(1, tags.as_ptr(), key.as_ptr(), &o) }
        });
    }

    // ---- PK_FACE_intersect_surf v2: is mixed_curve_category live? ----
    sweep(
        "PK_FACE_intersect_surf_o_t[mixed_curve_category]",
        5,
        |v, g| {
            let b1 = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
            let f1 = b1.faces().unwrap()[0].tag();
            let pl = Surf::plane(ax(Vec3::new(0.0, 0.0, 1.0))).unwrap();
            let mut o: PK_FACE_intersect_surf_o_t = unsafe { std::mem::zeroed() };
            o.o_t_version = v;
            o.mixed_curve_category = if g {
                GARBAGE
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
                PK_FACE_intersect_surf(
                    f1,
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
            free_all(&[a as _, c as _, d as _, e as _]);
            rc
        },
    );

    // ---- PK_BODY_extrude: `keep_as_facet` is never type-checked, so probe a
    // different late enum field to find which versions read past the prefix ----
    sweep("PK_BODY_extrude_o_t[consistent_params]", 8, |v, g| {
        let profile = match Body::create_sheet_circle(3.0, ax(Vec3::new(0.0, 0.0, 0.0))) {
            Ok(b) => b,
            Err(_) => return -1,
        };
        let mk_bound = |distance: f64| PK_BODY_extrude_bound_t {
            bound: PK_bound_distance_c,
            forward: PK_LOGICAL_true,
            distance,
            entity: PK_ENTITY_null,
            nearest: PK_LOGICAL_false,
            nth_division: 0,
            side: PK_bound_side_both_c,
        };
        let o = PK_BODY_extrude_o_t {
            o_t_version: v,
            start_bound: mk_bound(0.0),
            end_bound: mk_bound(5.0),
            extruded_body: PK_ENTITY_null,
            allow_disjoint: PK_LOGICAL_false,
            consistent_params: if g {
                GARBAGE
            } else {
                PK_PARAM_consistent_unset_c
            },
            have_pline_angle: PK_LOGICAL_false,
            pline_angle: 0.0,
            keep_as_facet: PK_extrude_keep_as_facet_no_c,
        };
        let dir: PK_VECTOR1_t = [0.0, 0.0, 1.0];
        let mut body: PK_BODY_t = PK_ENTITY_null;
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut results_buf = [0u8; 64];
        let rc = unsafe {
            PK_BODY_extrude(
                profile.tag(),
                &dir,
                &o,
                &mut body,
                &mut tracking,
                results_buf.as_mut_ptr() as *mut _,
            )
        };
        unsafe {
            let _ = PK_TOPOL_track_r_f(&mut tracking);
        }
        rc
    });

    // ---- PK_BODY_section: confirm a mid-struct enum is live at v5 too ----
    sweep("PK_BODY_section_o_t[check_fa]", 8, |v, g| {
        let target = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let pl = Surf::plane(ax(Vec3::new(0.0, 0.0, 0.0))).unwrap();
        let mut o = PK_BODY_section_o_t {
            o_t_version: v,
            ..Default::default()
        };
        o.check_fa = if g { GARBAGE } else { PK_section_check_fa_no_c };
        let mut results: PK_section_r_t = unsafe { std::mem::zeroed() };
        unsafe { PK_BODY_section_with_surf(target.tag(), pl.tag(), &o, &mut results) }
    });

    // ---- mass props: does v2 give the same numbers as v1? ----
    println!("\n-- PK_TOPOL_eval_mass_props numeric v1 vs v2");
    for v in [1, 2] {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let o = PK_TOPOL_eval_mass_props_o_t {
            o_t_version: v,
            mass: PK_mass_m_of_i_c,
            periphery: PK_mass_periphery_yes_c,
            bound: PK_mass_bound_no_c,
            single: PK_LOGICAL_false,
        };
        let (mut amount, mut mass, mut per) = (0.0, 0.0, 0.0);
        let (mut cg, mut mi) = ([0.0; 3], [0.0; 9]);
        let tag = b.tag();
        let rc = unsafe {
            PK_TOPOL_eval_mass_props(
                1,
                &tag,
                0.99,
                &o,
                &mut amount,
                &mut mass,
                &mut cg,
                &mut mi,
                &mut per,
            )
        };
        println!(
            "   v{v} rc={rc} amount={amount:.12} mass={mass:.12} periphery={per:.12} cg={cg:?} moi[0]={:.12}",
            mi[0]
        );
    }

    // ---- nabox: is the memory past our 144-byte struct read at v2/v3/v4? ----
    println!("\n-- PK_TOPOL_find_nabox pad sensitivity (0x00 pad vs 0xAA pad)");
    for v in [2, 3, 4] {
        for (label, fill) in [("zero", 0u8), ("0xAA", 0xAAu8)] {
            let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
            let mut tag = b.tag();
            let mut buf = [fill; 512];
            unsafe {
                let p = buf.as_mut_ptr() as *mut PK_TOPOL_find_nabox_o_t;
                *p = PK_TOPOL_find_nabox_o_t {
                    o_t_version: v,
                    ..Default::default()
                };
            }
            let mut sf: PK_NABOX_sf_t = unsafe { std::mem::zeroed() };
            let rc = unsafe {
                PK_TOPOL_find_nabox(
                    1,
                    &mut tag,
                    std::ptr::null_mut(),
                    buf.as_mut_ptr() as *mut _,
                    &mut sf,
                )
            };
            println!(
                "   v{v} pad={label:<5} rc={rc:<6} ({}) box[0]={:.6}",
                verdict(rc),
                sf.coord[0]
            );
        }
    }
}

// ============================================================================
// group 8 — dormant / session-management structs (lower priority)
// ============================================================================

fn group_dormant() {
    println!("\n=== group 8: partition / facet ===");

    sweep("PK_PARTITION_create_o_t", 8, |v, g| {
        let o = PK_PARTITION_create_o_t {
            o_t_version: v,
            allow_partial_pmarks: if g { GARBAGE } else { PK_LOGICAL_false },
        };
        let mut r = PK_PARTITION_create_r_t {
            partition: PK_ENTITY_null,
        };
        unsafe { PK_PARTITION_create(&o, &mut r) }
    });

    sweep("PK_PARTITION_delete_o_t", 8, |v, g| {
        let o = PK_PARTITION_create_o_t {
            o_t_version: 1,
            allow_partial_pmarks: PK_LOGICAL_false,
        };
        let mut r = PK_PARTITION_create_r_t {
            partition: PK_ENTITY_null,
        };
        if unsafe { PK_PARTITION_create(&o, &mut r) } != 0 {
            return -1;
        }
        let p = r.partition;
        let d = PK_PARTITION_delete_o_t {
            o_t_version: v,
            delete_non_empty: if g { GARBAGE } else { PK_LOGICAL_false },
        };
        unsafe { PK_PARTITION_delete(p, &d) }
    });

    // Facet is already documented at version 5 in crates/parasolid/src/facet.rs;
    // re-measure the control struct's ceiling to keep that note honest.
    sweep("PK_TOPOL_facet_2_o_t[control.o_t_version]", 9, |v, _g| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
        let mut o: PK_TOPOL_facet_2_o_t = unsafe { std::mem::zeroed() };
        o.control.o_t_version = v;
        o.choice.o_t_version = v;
        let mut tag = b.tag();
        let mut tables: PK_TOPOL_facet_2_r_t = unsafe { std::mem::zeroed() };
        let rc =
            unsafe { PK_TOPOL_facet_2(1, &mut tag, std::ptr::null_mut(), &mut o, &mut tables) };
        unsafe {
            let _ = PK_TOPOL_facet_2_r_f(&mut tables);
        }
        rc
    });
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    let arg = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    let run = |n: &str| arg == "all" || arg == n;
    if run("intersect") {
        group_intersect();
    }
    if run("range") {
        group_range();
    }
    if arg == "nabox" {
        group_nabox();
    }
    if run("mass") {
        group_mass();
    }
    if run("boolean") {
        group_boolean();
    }
    if run("fileio") {
        group_fileio();
    }
    if arg == "local" {
        group_local();
    }
    if arg == "refine" {
        group_refine();
    }
    if arg == "dormant" {
        group_dormant();
    }
    if run("misc") {
        group_misc();
    }
    println!("\n== done");
}
