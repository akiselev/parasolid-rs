//! ADVERSARIAL REVIEW PROBE (Stage 5). Read-only: creates nothing in the repo
//! beyond this file. Falsification targets:
//!   1. PK_TOPOL_clash_o_t 56-byte layout / byte-sized logicals
//!   2. PK_TOPOL_clash_rec_t stride + index correctness with n>1
//!   3. clash type tokens across many configurations
//!   4. PK_ENTITY_range_r_t / PK_ENTITY_range_end_t layout (sentinel dump)
//!   5. RangeStatus lower/upper/not_found via PK_range_bound_t
//!   6. PK_range_end_t / PK_range_1_r_t / PK_range_2_r_t true sizes (sentinel)
//!   7. find_extreme with NULL options + tie-breaking

use parasolid::*;
use parasolid_sys::*;
use std::os::raw::{c_double, c_int};

const SENT: u8 = 0xAA;

fn dump_words(buf: &[u8], n: usize, label: &str) {
    println!("  {label}");
    for i in 0..n {
        let o = i * 4;
        let w = i32::from_le_bytes([buf[o], buf[o + 1], buf[o + 2], buf[o + 3]]);
        let mut d = String::new();
        if o % 8 == 0 && o + 8 <= buf.len() {
            let mut b = [0u8; 8];
            b.copy_from_slice(&buf[o..o + 8]);
            let f = f64::from_le_bytes(b);
            if f.is_finite() && (f == 0.0 || (f.abs() > 1e-12 && f.abs() < 1e12)) {
                d = format!("  dbl={f:.6}");
            }
        }
        let sent = if buf[o..o + 4] == [SENT; 4] {
            " <SENTINEL>"
        } else {
            ""
        };
        println!("    @{o:<4} 0x{w:08x} {w:>12}{d}{sent}");
    }
}

fn cls(tag: c_int) -> String {
    if tag == 0 {
        return "null".into();
    }
    let mut c: PK_CLASS_t = 0;
    let rc = unsafe { PK_ENTITY_ask_class(tag, &mut c) };
    if rc != 0 {
        format!("INVALID(rc={rc})")
    } else {
        format!("{:?}", PkClass::from_raw(c))
    }
}

// ---------------------------------------------------------------------------
// 6. True size/offsets of PK_range_end_t via PK_TOPOL_range into a raw buffer
// ---------------------------------------------------------------------------
fn probe_range_layouts() {
    println!("\n===== (A) PK_range_2_r_t / PK_range_1_r_t TRUE LAYOUT (sentinel dump) =====");
    println!(
        "  Rust sizes: PK_range_end_t={} PK_range_1_r_t={} PK_range_2_r_t={}",
        std::mem::size_of::<PK_range_end_t>(),
        std::mem::size_of::<PK_range_1_r_t>(),
        std::mem::size_of::<PK_range_2_r_t>()
    );

    let b1 = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    let b2 = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    b2.transform(&Transform::translation(20.0, 0.0, 0.0).unwrap())
        .unwrap();

    let mut buf = [SENT; 512];
    let mut opts = PK_TOPOL_range_o_t::default();
    let mut status: PK_range_result_t = 0;
    let rc = unsafe {
        PK_TOPOL_range(
            b1.tag(),
            b2.tag(),
            &mut opts,
            &mut status,
            buf.as_mut_ptr() as *mut PK_range_2_r_t,
        )
    };
    println!("  PK_TOPOL_range rc={rc} status={status}");
    // find the last non-sentinel byte
    let last = buf.iter().rposition(|&b| b != SENT).map(|i| i + 1);
    println!("  bytes written (last non-sentinel offset+1) = {last:?}  [Rust struct claims 104]");
    dump_words(&buf, 32, "PK_range_2_r_t raw:");
    println!(
        "  b1 tag={} faces? ; b2 tag={}  (expect end_1.entity in b1's tree, end_2.entity in b2's)",
        b1.tag(),
        b2.tag()
    );
    for &o in &[8usize, 12, 56, 60, 64, 68] {
        let w = i32::from_le_bytes([buf[o], buf[o + 1], buf[o + 2], buf[o + 3]]);
        println!("    @{o}: tag {w} -> {}", cls(w));
    }

    println!("\n  --- PK_range_1_r_t ---");
    let mut buf1 = [SENT; 512];
    let v: PK_VECTOR_t = [0.0, 0.0, 40.0];
    let mut o1 = PK_TOPOL_range_vector_o_t::default();
    let mut st1: PK_range_result_t = 0;
    let rc = unsafe {
        PK_TOPOL_range_vector(
            b1.tag(),
            &v,
            &mut o1,
            &mut st1,
            buf1.as_mut_ptr() as *mut PK_range_1_r_t,
        )
    };
    let last1 = buf1.iter().rposition(|&b| b != SENT).map(|i| i + 1);
    println!("  rc={rc} status={st1} bytes written={last1:?}  [Rust struct claims 56]");
    dump_words(&buf1, 18, "PK_range_1_r_t raw:");
}

// ---------------------------------------------------------------------------
// 5. RangeStatus lower / upper / not_found
// ---------------------------------------------------------------------------
fn probe_bounds() {
    println!("\n===== (B) RangeStatus via PK_range_bound_t =====");
    let b = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    let v: PK_VECTOR_t = [0.0, 0.0, 40.0]; // true min distance = 38.0

    let cases: [(&str, PK_LOGICAL_t, f64, PK_LOGICAL_t, f64, PK_range_type_t); 6] = [
        ("no bounds (min)", 0, 0.0, 0, 0.0, PK_range_type_minimum_c),
        (
            "upper_bound=10 (min<10? no, min=38)",
            1,
            10.0,
            0,
            0.0,
            PK_range_type_minimum_c,
        ),
        (
            "upper_bound=100 (min=38 < 100)",
            1,
            100.0,
            0,
            0.0,
            PK_range_type_minimum_c,
        ),
        (
            "lower_bound=100 (min)",
            0,
            0.0,
            1,
            100.0,
            PK_range_type_minimum_c,
        ),
        (
            "lower_bound=100 (max)",
            0,
            0.0,
            1,
            100.0,
            PK_range_type_maximum_c,
        ),
        (
            "lower_bound=1 (max, true max~43)",
            0,
            0.0,
            1,
            1.0,
            PK_range_type_maximum_c,
        ),
    ];

    for (label, hu, ub, hl, lb, rt) in cases {
        let mut o = PK_TOPOL_range_vector_o_t::default();
        o.bound = PK_range_bound_t {
            have_upper_bound: hu,
            upper_bound: ub,
            have_lower_bound: hl,
            lower_bound: lb,
        };
        // range_vector_o_t has no range_type; use TOPOL_range_o_t for max cases
        let mut st: PK_range_result_t = 0;
        if rt == PK_range_type_minimum_c {
            let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
            let rc = unsafe { PK_TOPOL_range_vector(b.tag(), &v, &mut o, &mut st, &mut r) };
            println!("  {label:<40} rc={rc} status={st} dist={:.4}", r.distance);
        } else {
            // build a vertex-ish second entity: use another block far away
            let b2 = Body::create_solid_block(1.0, 1.0, 1.0).unwrap();
            b2.transform(&Transform::translation(0.0, 0.0, 40.0).unwrap())
                .unwrap();
            let mut o2 = PK_TOPOL_range_o_t::default();
            o2.range_type = rt;
            o2.bound = PK_range_bound_t {
                have_upper_bound: hu,
                upper_bound: ub,
                have_lower_bound: hl,
                lower_bound: lb,
            };
            let mut buf = [SENT; 512];
            let rc = unsafe {
                PK_TOPOL_range(
                    b.tag(),
                    b2.tag(),
                    &mut o2,
                    &mut st,
                    buf.as_mut_ptr() as *mut PK_range_2_r_t,
                )
            };
            let mut d = [0u8; 8];
            d.copy_from_slice(&buf[0..8]);
            println!(
                "  {label:<40} rc={rc} status={st} dist={:.4}",
                f64::from_le_bytes(d)
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 1/2/3. Clash
// ---------------------------------------------------------------------------
fn clash_raw(
    targets: &mut [PK_TOPOL_t],
    tools: &mut [PK_TOPOL_t],
    find_all: u8,
    find_intersect: u8,
) -> (c_int, Vec<PK_TOPOL_clash_rec_t>, Vec<i32>) {
    let mut tf1 = vec![PK_ENTITY_null; targets.len()];
    let mut tf2 = vec![PK_ENTITY_null; tools.len()];
    let mut opts = PK_TOPOL_clash_o_t {
        find_all,
        find_intersect,
        ..PK_TOPOL_clash_o_t::default()
    };
    let mut n: c_int = 0;
    let mut p: *mut PK_TOPOL_clash_t = std::ptr::null_mut();
    let rc = unsafe {
        PK_TOPOL_clash(
            targets.len() as c_int,
            targets.as_mut_ptr(),
            tf1.as_mut_ptr(),
            tools.len() as c_int,
            tools.as_mut_ptr(),
            tf2.as_mut_ptr(),
            &mut opts,
            &mut n,
            &mut p,
        )
    };
    let mut recs = Vec::new();
    let mut raw = Vec::new();
    if !p.is_null() && n > 0 {
        let ip = p as *const i32;
        for i in 0..(n as usize * 5 + 5) {
            raw.push(unsafe { *ip.add(i) });
        }
        let rp = p as *const PK_TOPOL_clash_rec_t;
        for i in 0..n as usize {
            recs.push(unsafe { *rp.add(i) });
        }
        unsafe {
            let _ = PK_MEMORY_free(p as *mut std::os::raw::c_void);
        }
    }
    (rc, recs, raw)
}

fn show(label: &str, rc: c_int, recs: &[PK_TOPOL_clash_rec_t]) {
    println!("  {label:<46} rc={rc} n={}", recs.len());
    for r in recs {
        println!(
            "      target={}({}) ti={} tool={}({}) li={} type={}",
            r.target,
            cls(r.target),
            r.target_index,
            r.tool,
            cls(r.tool),
            r.tool_index,
            r.clash_type
        );
    }
}

fn probe_wrapper_distance_to() {
    println!("\n===== (A2) Entity::distance_to via the SAFE wrapper =====");
    let b1 = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    let b2 = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    b2.transform(&Transform::translation(20.0, 0.0, 0.0).unwrap())
        .unwrap();
    let r = b1.entity().distance_to(b2.entity()).unwrap();
    println!("  distance={:.6} status={:?}", r.distance, r.status);
    println!("  point_1={:?}", r.point_1);
    println!("  point_2={:?}   <-- should be (20,y,z) on b2", r.point_2);
    println!("  witness_2={:?}", r.witness_2);
    let w2 = r.witness_2.unwrap();
    println!(
        "  witness_2.entity class = {}, sub_entity = {:?}",
        cls(w2.entity.tag()),
        w2.sub_entity.map(|e| cls(e.tag()))
    );
    let d = ((r.point_1.x - r.point_2.x).powi(2)
        + (r.point_1.y - r.point_2.y).powi(2)
        + (r.point_1.z - r.point_2.z).powi(2))
    .sqrt();
    println!("  |point_1 - point_2| = {d}  (must equal distance {})", r.distance);
}

fn probe_range_type() {
    println!("\n===== (B2) does PK_TOPOL_range_o_t.range_type @144 do anything? =====");
    let a = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    let b = Body::create_solid_block(1.0, 1.0, 1.0).unwrap();
    b.transform(&Transform::translation(0.0, 0.0, 40.0).unwrap())
        .unwrap();
    for (label, rt) in [
        ("minimum", PK_range_type_minimum_c),
        ("maximum", PK_range_type_maximum_c),
        ("garbage 12345", 12345),
    ] {
        let mut o = PK_TOPOL_range_o_t::default();
        o.range_type = rt;
        let mut st: PK_range_result_t = 0;
        let mut buf = [SENT; 512];
        let rc = unsafe {
            PK_TOPOL_range(
                a.tag(),
                b.tag(),
                &mut o,
                &mut st,
                buf.as_mut_ptr() as *mut PK_range_2_r_t,
            )
        };
        let mut d = [0u8; 8];
        d.copy_from_slice(&buf[0..8]);
        println!(
            "  range_type={label:<14} rc={rc} status={st} distance={:.6}",
            f64::from_le_bytes(d)
        );
    }
    println!("  (min should be 36.0; a true maximum would be ~41.2)");

    // Hypothesis: range_type was added in a LATER o_t_version, and
    // PKU_update_TOPOL_range_o overwrites it with the default when
    // o_t_version==1 (which is what Default::default() sets).
    for ver in 0..8 {
        let mut o = PK_TOPOL_range_o_t::default();
        o.o_t_version = ver;
        o.range_type = PK_range_type_maximum_c;
        let mut st: PK_range_result_t = 0;
        let mut buf = [SENT; 512];
        let rc = unsafe {
            PK_TOPOL_range(a.tag(), b.tag(), &mut o, &mut st, buf.as_mut_ptr() as *mut PK_range_2_r_t)
        };
        let mut d = [0u8; 8];
        d.copy_from_slice(&buf[0..8]);
        println!("  o_t_version={ver} range_type=maximum -> rc={rc} status={st} dist={:.6}", f64::from_le_bytes(d));
    }
    for ver in 0..8 {
        let mut o = PK_TOPOL_range_o_t::default();
        o.o_t_version = ver;
        o.range_type = 12345;
        let mut st: PK_range_result_t = 0;
        let mut buf = [SENT; 512];
        let rc = unsafe {
            PK_TOPOL_range(a.tag(), b.tag(), &mut o, &mut st, buf.as_mut_ptr() as *mut PK_range_2_r_t)
        };
        println!("  o_t_version={ver} range_type=GARBAGE -> rc={rc} (5014 = validated)");
    }

    // Which fields does o_t_version=1 actually enable, for range_vector?
    let vv: PK_VECTOR_t = [0.0, 0.0, 40.0];
    for ver in 1..5 {
        for (fld, mk) in [
            ("opt_level=GARBAGE", 0usize),
            ("param_entity=GARBAGE", 1usize),
        ] {
            let mut o = PK_TOPOL_range_vector_o_t::default();
            o.o_t_version = ver;
            if mk == 0 { o.opt_level = 999; } else { o.param_entity = 999; }
            let mut st: PK_range_result_t = 0;
            let mut r: PK_range_1_r_t = unsafe { std::mem::zeroed() };
            let rc = unsafe { PK_TOPOL_range_vector(a.tag(), &vv, &mut o, &mut st, &mut r) };
            println!("  range_vector o_t_version={ver} {fld:<22} rc={rc}");
        }
    }

    // same question for opt_level @148
    for (label, ol) in [
        ("performance", PK_range_opt_performance_c),
        ("accuracy", PK_range_opt_accuracy_c),
        ("garbage 999", 999),
    ] {
        let mut o = PK_TOPOL_range_o_t::default();
        o.opt_level = ol;
        let mut st: PK_range_result_t = 0;
        let mut buf = [SENT; 512];
        let rc = unsafe {
            PK_TOPOL_range(
                a.tag(),
                b.tag(),
                &mut o,
                &mut st,
                buf.as_mut_ptr() as *mut PK_range_2_r_t,
            )
        };
        println!("  opt_level={label:<14} rc={rc} status={st}");
    }
}

fn probe_clash() {
    println!("\n===== (C) PK_TOPOL_clash: options bytes, record stride, indices, tokens =====");
    println!(
        "  sizeof PK_TOPOL_clash_o_t = {}  rec = {}",
        std::mem::size_of::<PK_TOPOL_clash_o_t>(),
        std::mem::size_of::<PK_TOPOL_clash_rec_t>()
    );

    let mk = |s: f64, dx: f64, dy: f64, dz: f64| -> Body {
        let b = Body::create_solid_block(s, s, s).unwrap();
        if dx != 0.0 || dy != 0.0 || dz != 0.0 {
            b.transform(&Transform::translation(dx, dy, dz).unwrap())
                .unwrap();
        }
        b
    };

    // --- byte-vs-int logicals -------------------------------------------
    println!("\n  -- find_intersect as a BYTE @25 (classification on/off) --");
    let a = mk(4.0, 0.0, 0.0, 0.0);
    let b = mk(4.0, 2.0, 0.0, 0.0);
    for (fa, fi) in [(1u8, 0u8), (1, 1), (0, 1), (0, 0)] {
        let (rc, recs, _) = clash_raw(&mut [a.tag()], &mut [b.tag()], fa, fi);
        show(&format!("find_all={fa} find_intersect={fi}"), rc, &recs);
    }
    // If the four logicals were 4-byte ints, writing 1 at byte 24 alone
    // (find_all=1, rest 0) would be indistinguishable from find_all=1 as int.
    // The discriminating test: write byte 25 only (find_intersect=1,
    // find_all=0). Under a 4-int layout byte 25 is padding inside find_all
    // and classification would stay OFF while find_all would read as 256.
    println!("  ^ if (0,1) still classifies, the byte layout is confirmed.");

    // --- raw options byte image ------------------------------------------
    let opts = PK_TOPOL_clash_o_t {
        find_all: 1,
        find_intersect: 1,
        mul_target_tf: 1,
        mul_tool_tf: 1,
        ..PK_TOPOL_clash_o_t::default()
    };
    let bytes: [u8; 56] = unsafe { std::mem::transmute(opts) };
    println!("  options byte image 24..32: {:?}", &bytes[24..32]);

    // --- indices with n_targets>1 and n_tools>1 --------------------------
    println!("\n  -- multi-target / multi-tool INDEX correctness --");
    // 3 targets, 3 tools. Only specific pairs clash.
    let t0 = mk(4.0, 0.0, 0.0, 0.0); // at origin
    let t1 = mk(4.0, 0.0, 0.0, 100.0); // far away
    let t2 = mk(4.0, 0.0, 0.0, 200.0);
    let u0 = mk(4.0, 0.0, 0.0, 500.0); // clashes with nothing
    let u1 = mk(4.0, 0.0, 0.0, 201.0); // clashes with t2 (index 2)
    let u2 = mk(4.0, 2.0, 0.0, 100.0); // clashes with t1 (index 1)
    let mut tg = [t0.tag(), t1.tag(), t2.tag()];
    let mut tl = [u0.tag(), u1.tag(), u2.tag()];
    let (rc, recs, raw) = clash_raw(&mut tg, &mut tl, 1, 1);
    println!("  targets={tg:?} tools={tl:?}");
    show("3x3, expect (ti=1,li=2) and (ti=2,li=1)", rc, &recs);
    println!("  raw ints (5*n+5): {raw:?}");

    // --- token zoo --------------------------------------------------------
    println!("\n  -- clash type token zoo --");
    let big = mk(10.0, 0.0, 0.0, 0.0);
    let small = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    let (rc, recs, _) = clash_raw(&mut [big.tag()], &mut [small.tag()], 1, 1);
    show("small INSIDE big  (target=big, tool=small)", rc, &recs);
    let (rc, recs, _) = clash_raw(&mut [small.tag()], &mut [big.tag()], 1, 1);
    show("big CONTAINS target (target=small, tool=big)", rc, &recs);

    // Reproduce the EXACT configuration range_probe.rs called "strictly
    // inside": a 20-cube and a 2-cube translated +9 in z. A 20-cube spans
    // z in [-10,10]; the 2-cube spans [8,10] -> its TOP face is COPLANAR with
    // the big cube's top face. That is NOT strict containment.
    let big20 = Body::create_solid_block(20.0, 20.0, 20.0).unwrap();
    let sm2 = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    sm2.transform(&Transform::translation(0.0, 0.0, 9.0).unwrap())
        .unwrap();
    let (rc, recs, _) = clash_raw(&mut [big20.tag()], &mut [sm2.tag()], 1, 1);
    show("range_probe's 'strictly inside' (20-cube, 2-cube @z=9)", rc, &recs);
    let bb = big20.bounding_box().unwrap();
    let sb = sm2.bounding_box().unwrap();
    println!(
        "    big z=[{:.1},{:.1}]  small z=[{:.1},{:.1}]  -> top faces coplanar? {}",
        bb.min.z,
        bb.max.z,
        sb.min.z,
        sb.max.z,
        (bb.max.z - sb.max.z).abs() < 1e-12
    );
    // genuinely strict containment, no coplanarity anywhere
    let big20b = Body::create_solid_block(20.0, 20.0, 20.0).unwrap();
    let sm2b = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    sm2b.transform(&Transform::translation(0.0, 0.0, 5.0).unwrap())
        .unwrap();
    let (rc, recs, _) = clash_raw(&mut [big20b.tag()], &mut [sm2b.tag()], 1, 1);
    show("TRULY strict containment (2-cube @z=5 inside 20-cube)", rc, &recs);
    let (rc, recs, _) = clash_raw(&mut [sm2b.tag()], &mut [big20b.tag()], 1, 1);
    show("TRULY strict containment, reversed", rc, &recs);

    // Is the token with find_intersect=0 a constant sentinel, or a coarse
    // classification? Sweep the same configurations with classification OFF.
    println!("\n  -- find_intersect=0 token across configurations --");
    {
        let pairs: Vec<(&str, i32, i32)> = vec![
            ("overlap", {
                let x = mk(4.0, 0.0, 0.0, 0.0);
                let t = x.tag();
                std::mem::forget(x);
                t
            }, {
                let y = mk(4.0, 2.0, 0.0, 0.0);
                let t = y.tag();
                std::mem::forget(y);
                t
            }),
            ("abut", {
                let x = mk(4.0, 0.0, 0.0, 0.0);
                let t = x.tag();
                std::mem::forget(x);
                t
            }, {
                let y = mk(4.0, 4.0, 0.0, 0.0);
                let t = y.tag();
                std::mem::forget(y);
                t
            }),
            ("containment", {
                let x = Body::create_solid_block(20.0, 20.0, 20.0).unwrap();
                let t = x.tag();
                std::mem::forget(x);
                t
            }, {
                let y = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
                y.transform(&Transform::translation(5.0, 5.0, 5.0).unwrap())
                    .unwrap();
                let t = y.tag();
                std::mem::forget(y);
                t
            }),
        ];
        for (label, a, b) in pairs {
            let (rc, recs, _) = clash_raw(&mut [a], &mut [b], 1, 0);
            let mut toks: Vec<i32> = recs.iter().map(|r| r.clash_type).collect();
            toks.sort_unstable();
            toks.dedup();
            let cl: Vec<String> = recs.iter().take(1).map(|r| cls(r.target)).collect();
            println!("    {label:<14} find_intersect=0 rc={rc} n={} tokens={toks:?} first target class={cl:?}", recs.len());
            let (rc, recs, _) = clash_raw(&mut [a], &mut [b], 1, 1);
            let mut toks: Vec<i32> = recs.iter().map(|r| r.clash_type).collect();
            toks.sort_unstable();
            toks.dedup();
            println!("    {label:<14} find_intersect=1 rc={rc} n={} tokens={toks:?}", recs.len());
        }
    }

    let c0 = mk(4.0, 0.0, 0.0, 0.0);
    let c1 = mk(4.0, 4.0, 0.0, 0.0);
    let (rc, recs, _) = clash_raw(&mut [c0.tag()], &mut [c1.tag()], 1, 1);
    show("face-to-face abut", rc, &recs);

    let e0 = mk(4.0, 0.0, 0.0, 0.0);
    let e1 = mk(4.0, 4.0, 4.0, 0.0);
    let (rc, recs, _) = clash_raw(&mut [e0.tag()], &mut [e1.tag()], 1, 1);
    show("edge-to-edge touch (diagonal in xy)", rc, &recs);

    let v0 = mk(4.0, 0.0, 0.0, 0.0);
    let v1 = mk(4.0, 4.0, 4.0, 4.0);
    let (rc, recs, _) = clash_raw(&mut [v0.tag()], &mut [v1.tag()], 1, 1);
    show("vertex-to-vertex touch", rc, &recs);

    let id0 = mk(4.0, 0.0, 0.0, 0.0);
    let id1 = mk(4.0, 0.0, 0.0, 0.0);
    let (rc, recs, _) = clash_raw(&mut [id0.tag()], &mut [id1.tag()], 1, 1);
    show("identical blocks", rc, &recs);

    // sheet through a solid
    let basis = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let sheet = Body::create_sheet_rectangle(20.0, 20.0, basis).unwrap();
    let solid = mk(4.0, 0.0, 0.0, 0.0);
    let (rc, recs, _) = clash_raw(&mut [solid.tag()], &mut [sheet.tag()], 1, 1);
    show("solid vs SHEET cutting through it", rc, &recs);
    let (rc, recs, _) = clash_raw(&mut [sheet.tag()], &mut [solid.tag()], 1, 1);
    show("SHEET vs solid (reversed)", rc, &recs);

    // face-level clash (sub-topology targets)
    let fa = mk(4.0, 0.0, 0.0, 0.0);
    let fb = mk(4.0, 2.0, 0.0, 0.0);
    let faces_a: Vec<i32> = fa.faces().unwrap().iter().map(|f| f.tag()).collect();
    let faces_b: Vec<i32> = fb.faces().unwrap().iter().map(|f| f.tag()).collect();
    let mut ta = faces_a.clone();
    let mut tb = faces_b.clone();
    let (rc, recs, raw) = clash_raw(&mut ta, &mut tb, 1, 1);
    show(
        &format!("FACE-level {}x{}", faces_a.len(), faces_b.len()),
        rc,
        &recs,
    );
    println!("  faces_a={faces_a:?}");
    println!("  faces_b={faces_b:?}");
    println!("  raw={raw:?}");
}

// ---------------------------------------------------------------------------
// 4. PK_ENTITY_range
// ---------------------------------------------------------------------------
fn probe_entity_range() {
    println!("\n===== (D) PK_ENTITY_range multi-solution structs =====");
    println!(
        "  Rust: sizeof PK_ENTITY_range_end_t={} PK_ENTITY_range_r_t={} PK_ENTITY_range_o_t={}",
        std::mem::size_of::<PK_ENTITY_range_end_t>(),
        std::mem::size_of::<PK_ENTITY_range_r_t>(),
        std::mem::size_of::<PK_ENTITY_range_o_t>()
    );
    let b1 = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    let b2 = Body::create_solid_block(4.0, 4.0, 4.0).unwrap();
    b2.transform(&Transform::translation(20.0, 0.0, 0.0).unwrap())
        .unwrap();

    let mut o: PK_ENTITY_range_o_t = unsafe { std::mem::zeroed() };
    o.o_t_version = 1;
    o.bound = PK_range_bound_t::default();
    o.range_type = PK_range_type_minimum_c;
    o.param_entity = PK_range_param_entity_topol_c;
    o.output_scale = 0;

    #[repr(align(16))]
    struct Aligned([u8; 256]);
    let mut rb = Aligned([SENT; 256]);
    let rbuf = &mut rb.0;
    let mut e1 = [b1.tag()];
    let mut e2 = [b2.tag()];
    let mut tf1 = [PK_ENTITY_null];
    let mut tf2 = [PK_ENTITY_null];
    let rc = unsafe {
        PK_ENTITY_range(
            1,
            e1.as_mut_ptr(),
            tf1.as_mut_ptr(),
            1,
            e2.as_mut_ptr(),
            tf2.as_mut_ptr(),
            &o,
            rbuf.as_mut_ptr() as *mut PK_ENTITY_range_r_t,
        )
    };
    println!("  PK_ENTITY_range rc={rc}");
    let last = rbuf.iter().rposition(|&x| x != SENT).map(|i| i + 1);
    println!("  r_t bytes written = {last:?} (Rust claims 40)");
    dump_words(rbuf, 12, "r_t raw:");
    if rc == 0 {
        let r = unsafe { *(rbuf.as_ptr() as *const PK_ENTITY_range_r_t) };
        println!(
            "  r_t_version={} n_results={} results={:p} distances={:p} ends_1={:p} ends_2={:p}",
            r.r_t_version, r.n_results, r.results, r.distances, r.ends_1, r.ends_2
        );
        for i in 0..r.n_results.max(0) as usize {
            let st = unsafe { *r.results.add(i) };
            let d = unsafe { *r.distances.add(i) };
            let a = unsafe { *r.ends_1.add(i) };
            let b = unsafe { *r.ends_2.add(i) };
            println!("   [{i}] status={st} dist={d:.6}");
            println!(
                "        end1 leading={} entity={}({}) sub={}({}) vec=({:.4},{:.4},{:.4}) par=({:.4},{:.4}) inside={}",
                a.leading,
                a.entity,
                cls(a.entity),
                a.sub_entity,
                cls(a.sub_entity),
                a.vector[0],
                a.vector[1],
                a.vector[2],
                a.parameters[0],
                a.parameters[1],
                a.inside
            );
            println!(
                "        end2 leading={} entity={}({}) sub={}({}) vec=({:.4},{:.4},{:.4}) par=({:.4},{:.4}) inside={}",
                b.leading,
                b.entity,
                cls(b.entity),
                b.sub_entity,
                cls(b.sub_entity),
                b.vector[0],
                b.vector[1],
                b.vector[2],
                b.parameters[0],
                b.parameters[1],
                b.inside
            );
            // raw dump of the end record
            let raw =
                unsafe { std::slice::from_raw_parts(r.ends_1.add(i) as *const u8, 64) }.to_vec();
            dump_words(&raw, 16, "        end1 raw 64 bytes:");
        }
        unsafe {
            let _ = PK_ENTITY_range_r_f(rbuf.as_mut_ptr() as *mut PK_ENTITY_range_r_t);
        }
    }
}

// ---------------------------------------------------------------------------
// 7. find_extreme
// ---------------------------------------------------------------------------
fn probe_extreme() {
    println!("\n===== (E) find_extreme: NULL options + tie-breaking =====");
    let blk = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
    let cases: [(&str, [f64; 3], [f64; 3], [f64; 3]); 4] = [
        (
            "d1=+Z d2=+Z d3=+Z (whole top face extremal)",
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ),
        (
            "d1=+Z d2=+X d3=+X (top face, then +X edge)",
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ),
        (
            "d1=+Z d2=+X d3=+Y (corner)",
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
        ),
        (
            "d1=+Z d2=+Y d3=+X (swapped tie-breakers)",
            [0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
        ),
    ];
    for (label, d1, d2, d3) in cases {
        let mut ex: PK_VECTOR_t = [0.0; 3];
        let mut topol: PK_TOPOL_t = 0;
        let rc = unsafe {
            PK_BODY_find_extreme(
                blk.tag(),
                &d1,
                &d2,
                &d3,
                std::ptr::null_mut(),
                &mut ex,
                &mut topol,
            )
        };
        println!(
            "  {label:<44} rc={rc} pos=({:.3},{:.3},{:.3}) topol={}({})",
            ex[0],
            ex[1],
            ex[2],
            topol,
            cls(topol)
        );
    }
    // NULL options with a non-null options struct for comparison
    let mut o = PK_BODY_find_extreme_o_t {
        o_t_version: 1,
        have_transf: PK_LOGICAL_false,
        transf: PK_ENTITY_null,
    };
    let d1: PK_VECTOR_t = [0.0, 0.0, 1.0];
    let d2: PK_VECTOR_t = [1.0, 0.0, 0.0];
    let d3: PK_VECTOR_t = [0.0, 1.0, 0.0];
    let mut ex: PK_VECTOR_t = [0.0; 3];
    let mut topol: PK_TOPOL_t = 0;
    let rc = unsafe { PK_BODY_find_extreme(blk.tag(), &d1, &d2, &d3, &mut o, &mut ex, &mut topol) };
    println!("  explicit options struct: rc={rc} topol={topol}({})", cls(topol));

    // Does a 3-int options struct with a leading o_t_version work?
    #[repr(C)]
    struct OtV1 {
        o_t_version: c_int,
        have_transf: c_int,
        transf: c_int,
    }
    let mut o2 = OtV1 {
        o_t_version: 1,
        have_transf: 0,
        transf: 0,
    };
    let mut ex: PK_VECTOR_t = [0.0; 3];
    let mut topol: PK_TOPOL_t = 0;
    let rc = unsafe {
        PK_BODY_find_extreme(
            blk.tag(),
            &d1,
            &d2,
            &d3,
            (&mut o2) as *mut OtV1 as *mut PK_BODY_find_extreme_o_t,
            &mut ex,
            &mut topol,
        )
    };
    println!("  {{o_t_version=1, have_transf, transf}}: rc={rc} topol={topol}({})", cls(topol));

    // can a FACE or EDGE ever be the extreme topology?
    let cyl = Body::create_solid_cylinder(5.0, 10.0).unwrap();
    for (label, a, b, c) in [
        ("cyl d1=+Z d2=+X d3=+Y", [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
        ("cyl d1=+X d2=+Z d3=+Y", [1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0]),
        (
            "cyl d1=+Z d2=(1,1,0) d3=(1,-1,0)",
            [0.0, 0.0, 1.0],
            [1.0, 1.0, 0.0],
            [1.0, -1.0, 0.0],
        ),
    ] {
        let (da, db, dc): (PK_VECTOR_t, PK_VECTOR_t, PK_VECTOR_t) = (a, b, c);
        let mut ex: PK_VECTOR_t = [0.0; 3];
        let mut topol: PK_TOPOL_t = 0;
        let rc = unsafe {
            PK_BODY_find_extreme(
                cyl.tag(),
                &da,
                &db,
                &dc,
                std::ptr::null_mut(),
                &mut ex,
                &mut topol,
            )
        };
        println!(
            "  {label:<34} rc={rc} pos=({:.3},{:.3},{:.3}) topol={}({})",
            ex[0],
            ex[1],
            ex[2],
            topol,
            cls(topol)
        );
    }

    // non-unit / non-orthogonal directions
    let d1: PK_VECTOR_t = [0.0, 0.0, 2.0];
    let mut ex: PK_VECTOR_t = [0.0; 3];
    let mut topol: PK_TOPOL_t = 0;
    let rc = unsafe {
        PK_BODY_find_extreme(
            blk.tag(),
            &d1,
            &d2,
            &d3,
            std::ptr::null_mut(),
            &mut ex,
            &mut topol,
        )
    };
    println!("  non-unit d1=(0,0,2): rc={rc} topol={topol}({})", cls(topol));
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    probe_range_layouts();
    probe_wrapper_distance_to();
    probe_bounds();
    probe_range_type();
    probe_clash();
    probe_entity_range();
    probe_extreme();
    println!("\n=== probe complete ===");
    let _ = std::mem::size_of::<c_double>();
}
