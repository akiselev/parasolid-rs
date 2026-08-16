//! `PK_FACE_array_t` layout probe.
//!
//! Verifies at runtime the layout recovered from `pskernel.dll` V37.01.243:
//!
//!     #[repr(C)] struct PK_FACE_array_t { array: *mut PK_FACE_t, length: c_int, _pad: c_int }
//!
//! (pointer FIRST at +0, count at +8, 16 bytes — see the doc comment on the
//! struct in `parasolid-sys/src/blend.rs` for the disassembly evidence.)
//!
//! `raw` mode calls `PK_BODY_fix_blends` directly and dumps each `unders[i]`
//! descriptor word-by-word, checks every tag in the inner array is a real face
//! via `PK_ENTITY_ask_class`, then frees inner-first / outer-second exactly as
//! `PK_BODY_find_facesets_r_f` (0x18012d7d0) does. `rawnofree` skips the frees
//! (control). `wrapper`/`wrapper3` exercise `Body::fillet_edges_detailed`,
//! repeatedly, in one session — a bad free shows up as a later page fault.
//!
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/face_array_probe.exe [mode]
//!   modes: all (default) | raw | rawnofree | wrapper | wrapper3

use parasolid::*;
use parasolid_sys::*;
use std::os::raw::{c_int, c_void};

fn raw_pass(n_edges: usize, radius: f64, do_free: bool) {
    eprintln!("== raw pass: {n_edges} edge(s), r={radius} ==");
    let block = Body::create_solid_block(10.0, 10.0, 10.0).expect("block");
    let edges = block.edges().expect("edges");
    let edge_tags: Vec<PK_EDGE_t> = edges[..n_edges].iter().map(|e| e.tag()).collect();

    let mut n_set: c_int = 0;
    let mut set_edges: *mut PK_EDGE_t = std::ptr::null_mut();
    let err = unsafe {
        PK_EDGE_set_blend_constant(
            edge_tags.len() as c_int,
            edge_tags.as_ptr(),
            radius,
            std::ptr::null(),
            &mut n_set,
            &mut set_edges,
        )
    };
    eprintln!("  set_blend_constant -> err={err} n_set={n_set}");
    unsafe {
        if !set_edges.is_null() {
            let _ = PK_MEMORY_free(set_edges as *mut c_void);
        }
    }

    let mut n_blends: c_int = 0;
    let mut blends: *mut PK_FACE_t = std::ptr::null_mut();
    let mut unders: *mut PK_FACE_array_t = std::ptr::null_mut();
    let mut topols: *mut c_int = std::ptr::null_mut();
    let mut fault: PK_blend_fault_t = 0;
    let mut fault_edge: PK_EDGE_t = PK_ENTITY_null;
    let mut fault_topol: PK_ENTITY_t = PK_ENTITY_null;
    let err = unsafe {
        PK_BODY_fix_blends(
            block.tag(),
            std::ptr::null(),
            &mut n_blends,
            &mut blends,
            &mut unders,
            &mut topols,
            &mut fault,
            &mut fault_edge,
            &mut fault_topol,
        )
    };
    eprintln!(
        "  fix_blends -> err={err} n_blends={n_blends} fault={fault} unders={:p} \
         (sizeof PK_FACE_array_t = {})",
        unders,
        std::mem::size_of::<PK_FACE_array_t>()
    );
    assert_eq!(err, 0, "fix_blends failed");
    assert!(n_blends > 0 && !unders.is_null());

    unsafe {
        let descs = std::slice::from_raw_parts(unders, n_blends as usize);
        for (i, d) in descs.iter().enumerate() {
            // Raw words of the 16-byte descriptor, before any interpretation.
            let words = std::slice::from_raw_parts((d as *const PK_FACE_array_t) as *const u32, 4);
            eprintln!(
                "  unders[{i}] raw = [{:08x} {:08x} {:08x} {:08x}]  -> array={:p} length={} pad={}",
                words[0], words[1], words[2], words[3], d.array, d.length, d._padding
            );
            assert!(
                d.length >= 0 && d.length < 10_000,
                "implausible length {}",
                d.length
            );
            // NOTE: the +0xc word is *not* zeroed by the kernel — it holds
            // recycled heap garbage, which is itself evidence that it is
            // padding and not a field the kernel ever reads.
            if d.length > 0 {
                assert!(!d.array.is_null());
                let tags = std::slice::from_raw_parts(d.array, d.length as usize);
                let mut classes = Vec::new();
                for &t in tags {
                    let mut cls: PK_CLASS_t = 0;
                    let e = PK_ENTITY_ask_class(t, &mut cls);
                    assert_eq!(e, 0, "ask_class({t}) -> err {e}");
                    assert_eq!(cls, PK_CLASS_face, "tag {t} is class {cls}, not a face");
                    classes.push(cls);
                }
                eprintln!("      tags={tags:?} all class={} (face)", classes[0]);
            }
        }

        // Free inner arrays first (guarded on length > 0), then the outer block —
        // the order PK_BODY_find_facesets_r_f (0x18012d7d0) uses.
        if !do_free {
            eprintln!("  [marker] skipping all frees");
            return;
        }
        for d in descs {
            if d.length > 0 && !d.array.is_null() {
                let e = PK_MEMORY_free(d.array as *mut c_void);
                assert_eq!(e, 0, "inner free -> err {e}");
            }
        }
        let e = PK_MEMORY_free(unders as *mut c_void);
        assert_eq!(e, 0, "outer free -> err {e}");

        let _ = PK_MEMORY_free(blends as *mut c_void);
        let _ = PK_MEMORY_free(topols as *mut c_void);
    }
    eprintln!(
        "  freed inner+outer cleanly; faces now = {}",
        block.faces().unwrap().len()
    );
}

fn wrapper_pass(n_edges: usize, radius: f64) {
    eprintln!("== wrapper pass: {n_edges} edge(s), r={radius} ==");
    let block = Body::create_solid_block(10.0, 10.0, 10.0).expect("block");
    let edges = block.edges().expect("edges");
    let r = block
        .fillet_edges_detailed(&edges[..n_edges], radius)
        .expect("fillet");
    eprintln!("  blends={} unders={}", r.blends.len(), r.unders.len());
    assert_eq!(r.blends.len(), r.unders.len(), "parallel arrays");
    for (i, (b, u)) in r.blends.iter().zip(r.unders.iter()).enumerate() {
        let bc = b.entity().class().expect("class");
        let ucs: Vec<_> = u
            .iter()
            .map(|f| f.entity().class().expect("class"))
            .collect();
        eprintln!(
            "  blend[{i}] tag={} class={bc:?}  unders={:?} classes={ucs:?}",
            b.tag(),
            u.iter().map(|f| f.tag()).collect::<Vec<_>>()
        );
    }
    eprintln!("  faces now = {}", block.faces().unwrap().len());
}

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_else(|| "all".to_string());
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    match mode.as_str() {
        "raw" => {
            raw_pass(1, 1.0, true);
            raw_pass(3, 0.5, true);
        }
        "rawnofree" => {
            raw_pass(1, 1.0, false);
            raw_pass(3, 0.5, false);
        }
        "wrapper" => {
            wrapper_pass(1, 1.0);
        }
        "wrapper3" => {
            wrapper_pass(1, 1.0);
            wrapper_pass(3, 0.5);
            wrapper_pass(12, 0.25);
        }
        _ => {
            raw_pass(1, 1.0, true);
            raw_pass(3, 0.5, true);
            wrapper_pass(1, 1.0);
            wrapper_pass(3, 0.5);
            wrapper_pass(12, 0.25);
        }
    }
    eprintln!("OK");
}
