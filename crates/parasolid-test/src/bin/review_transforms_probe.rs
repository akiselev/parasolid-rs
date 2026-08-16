//! ADVERSARIAL REVIEW PROBE — Stage 2 (transforms) and Stage 3 (jets).
//! Throwaway; not part of the test suite.

use parasolid::*;
use parasolid_sys::*;

const SENT: f64 = -1.234_567_890_123e300;

fn is_sent(x: f64) -> bool {
    x == SENT
}

// ============================================================ Stage 2

/// Raw classify into an oversized sentinel buffer so we can see the exact
/// byte extent the kernel touches and read fields at claimed offsets.
fn raw_classify(tag: i32, diagnostics: i32, use_null_opts: bool) -> (i32, Vec<u8>) {
    // 512 bytes of 0xAB sentinel.
    let mut buf = vec![0xABu8; 512];
    let opts = PK_TRANSF_classify_o_t {
        o_t_version: 1,
        diagnostics,
    };
    let rc = unsafe {
        PK_TRANSF_classify(
            tag,
            if use_null_opts {
                std::ptr::null()
            } else {
                &opts
            },
            buf.as_mut_ptr() as *mut PK_TRANSF_classify_r_t,
        )
    };
    (rc, buf)
}

fn rd_i32(b: &[u8], off: usize) -> i32 {
    i32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}
fn rd_f64(b: &[u8], off: usize) -> f64 {
    f64::from_le_bytes(b[off..off + 8].try_into().unwrap())
}
fn rd_v3(b: &[u8], off: usize) -> [f64; 3] {
    [rd_f64(b, off), rd_f64(b, off + 8), rd_f64(b, off + 16)]
}

fn touched_extent(b: &[u8]) -> (Option<usize>, Option<usize>) {
    let first = b.iter().position(|&x| x != 0xAB);
    let last = b.iter().rposition(|&x| x != 0xAB);
    (first, last)
}

fn dump_classify(label: &str, m: [f64; 16], diagnostics: i32) {
    let t = Transform::from_matrix(m).expect("transf");
    let (rc, b) = raw_classify(t.tag(), diagnostics, false);
    let (f, l) = touched_extent(&b);
    println!("\n-- {label}  (rc={rc}, diagnostics={diagnostics})");
    println!("   touched bytes [{f:?} ..= {l:?}]  (claim: 0..=119)");
    println!("   @0   matrix_type            = {}", rd_i32(&b, 0));
    println!("   @4   (pad?)                 = {}", rd_i32(&b, 4));
    println!("   @8   determinant            = {:?}", rd_f64(&b, 8));
    println!("   @16  unit_rows_deviations   = {:?}", rd_v3(&b, 16));
    println!("   @40  orthog_rows_deviations = {:?}", rd_v3(&b, 40));
    println!("   @64  translation            = {:?}", rd_v3(&b, 64));
    println!("   @88  perspective            = {:?}", rd_v3(&b, 88));
    println!("   @112 scale                  = {:?}", rd_f64(&b, 112));
}

fn stage2() {
    println!("\n================ STAGE 2: PK_TRANSF_classify ================");

    // --- (1) discriminating case: rows ORTHOGONAL but NOT UNIT ---------------
    // diag(2,3,4): r_i . r_j = 0 for i != j; |r_i| != 1.
    // => unit_rows_deviations must be nonzero, orthog_rows_deviations ~ 0.
    dump_classify(
        "diag(2,3,4)  [orthogonal rows, non-unit lengths]",
        [
            2.0, 0.0, 0.0, 0.0, //
            0.0, 3.0, 0.0, 0.0, //
            0.0, 0.0, 4.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );

    // --- (2) discriminating case: rows UNIT but NOT ORTHOGONAL ---------------
    // r0=(1,0,0), r1=(cos t, sin t, 0), r2=(0,0,1): all unit; r0.r1 = cos t.
    // => unit_rows_deviations ~ 0, orthog_rows_deviations nonzero.
    let t = 1.0_f64; // 57.3 deg, cos = 0.5403
    dump_classify(
        "unit-but-skew rows [r0.r1 = cos(1) = 0.5403]",
        [
            1.0,
            0.0,
            0.0,
            0.0, //
            t.cos(),
            t.sin(),
            0.0,
            0.0, //
            0.0,
            0.0,
            1.0,
            0.0, //
            0.0,
            0.0,
            0.0,
            1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );

    // --- (3) asymmetric row lengths, to see WHICH row maps to which slot -----
    dump_classify(
        "diag(1,1,5) [only row 2 is non-unit]",
        [
            1.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 5.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );
    dump_classify(
        "diag(5,1,1) [only row 0 is non-unit]",
        [
            5.0, 0.0, 0.0, 0.0, //
            0.0, 1.0, 0.0, 0.0, //
            0.0, 0.0, 1.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );
    // Only rows 0 and 1 non-orthogonal (skew in xy), rows kept unit.
    dump_classify(
        "skew(0,1) only: r0.r1 != 0, r0.r2 = r1.r2 = 0",
        [
            1.0,
            0.0,
            0.0,
            0.0, //
            t.cos(),
            t.sin(),
            0.0,
            0.0, //
            0.0,
            0.0,
            1.0,
            0.0, //
            0.0,
            0.0,
            0.0,
            1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );
    // Only rows 1 and 2 non-orthogonal.
    dump_classify(
        "skew(1,2) only: r1.r2 != 0",
        [
            1.0,
            0.0,
            0.0,
            0.0, //
            0.0,
            1.0,
            0.0,
            0.0, //
            0.0,
            t.cos(),
            t.sin(),
            0.0, //
            0.0,
            0.0,
            0.0,
            1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );

    // --- (3b) THE decisive row-vs-column discriminator -----------------------
    // M = transpose of the previous one:
    //   rows    r0=(1,cos1,0) NOT unit, r1=(0,sin1,0) NOT unit, r2=(0,0,1)
    //   columns c0=(1,0,0), c1=(cos1,sin1,0), c2=(0,0,1)  ALL UNIT
    // If the deviations are over ROWS  -> unit_dev = [-cos^2 1, -(1-sin^2 1)...] nonzero
    // If the deviations are over COLUMNS of PK_TRANSF_sf_t.matrix -> unit_dev = 0
    dump_classify(
        "columns unit, ROWS non-unit  [transpose of the skew case]",
        [
            1.0,
            t.cos(),
            0.0,
            0.0, //
            0.0,
            t.sin(),
            0.0,
            0.0, //
            0.0,
            0.0,
            1.0,
            0.0, //
            0.0,
            0.0,
            0.0,
            1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );
    // Which slot carries the (0,2) pair?
    dump_classify(
        "skew(0,2) only: entry [0][2] filled",
        [
            1.0,
            0.0,
            0.0,
            0.0, //
            0.0,
            1.0,
            0.0,
            0.0, //
            t.cos(),
            0.0,
            t.sin(),
            0.0, //
            0.0,
            0.0,
            0.0,
            1.0,
        ],
        PK_TRANSF_diagnostics_all_c,
    );

    // --- (4) diagnostics = none: are the vectors left alone / zeroed? --------
    dump_classify(
        "diag(2,3,4) with diagnostics = NONE (25300)",
        [
            2.0, 0.0, 0.0, 0.0, //
            0.0, 3.0, 0.0, 0.0, //
            0.0, 0.0, 4.0, 0.0, //
            0.0, 0.0, 0.0, 1.0,
        ],
        PK_TRANSF_diagnostics_none_c,
    );

    // --- (5) is 25300 really none_c? and is a bogus token rejected? ----------
    let ident = Transform::from_matrix([
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ])
    .unwrap();
    for d in [25299, 25300, 25301, 25302] {
        let (rc, _) = raw_classify(ident.tag(), d, false);
        println!("   diagnostics token {d}: rc = {rc}");
    }
    let (rc_null, bnull) = raw_classify(ident.tag(), 0, true);
    println!(
        "   NULL options: rc = {rc_null}, matrix_type = {}, unit_dev = {:?}",
        rd_i32(&bnull, 0),
        rd_v3(&bnull, 16)
    );

    // --- (6) matrix_type lattice, esp. the translation / scale quirk --------
    let cases: [(&str, [f64; 16]); 8] = [
        (
            "identity",
            [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
        (
            "pure translation, m33=1",
            [
                1.0, 0.0, 0.0, 3.0, 0.0, 1.0, 0.0, -4.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
        (
            "pure translation + global scale m33=0.4",
            [
                1.0, 0.0, 0.0, 3.0, 0.0, 1.0, 0.0, -4.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 0.0, 0.4,
            ],
        ),
        (
            "NO translation + global scale m33=0.4",
            [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.4,
            ],
        ),
        (
            "rotation 90deg about z, no translation",
            [
                0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
        (
            "rotation 90deg about z + translation",
            [
                0.0, -1.0, 0.0, 3.0, 1.0, 0.0, 0.0, -4.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
        (
            "reflection in z=0, no translation",
            [
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
        (
            "reflection in z=0 + translation",
            [
                1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, -1.0, 3.0, 0.0, 0.0, 0.0, 1.0,
            ],
        ),
    ];
    println!("\n-- matrix_type lattice (25290=id 25291=rot 25292=refl 25293=gen 25294=uncl)");
    for (name, m) in cases {
        match Transform::from_matrix(m) {
            Ok(tr) => {
                let (rc, b) = raw_classify(tr.tag(), PK_TRANSF_diagnostics_all_c, false);
                println!(
                    "   {:44} rc={rc} matrix_type={} det={:.4} scale={:.4} transl={:?}",
                    name,
                    rd_i32(&b, 0),
                    rd_f64(&b, 8),
                    rd_f64(&b, 112),
                    rd_v3(&b, 64)
                );
            }
            Err(e) => println!("   {name:44} create failed: {e:?}"),
        }
    }

    // --- (7) PK_TRANSF_classify_r_f: does it free, or just reinitialise? ----
    println!("\n-- PK_TRANSF_classify_r_f behaviour");
    let mut r: PK_TRANSF_classify_r_t = unsafe { std::mem::zeroed() };
    let rc = unsafe { PK_TRANSF_classify_r_f(&mut r) };
    println!(
        "   on an all-zero struct: rc={rc}, matrix_type={} scale={} det={}",
        r.matrix_type, r.scale, r.determinant
    );
    let rc2 = unsafe { PK_TRANSF_classify_r_f(&mut r) };
    println!("   called a second time on the same struct: rc={rc2} (double-free would trap)");
    let rc3 = unsafe { PK_TRANSF_classify_r_f(std::ptr::null_mut()) };
    println!("   on NULL: rc={rc3} (expect 906 = 0x38a)");
    // Does it touch bytes past 120?
    let mut wide = vec![0xABu8; 256];
    let rc4 = unsafe { PK_TRANSF_classify_r_f(wide.as_mut_ptr() as *mut PK_TRANSF_classify_r_t) };
    let (f, l) = touched_extent(&wide);
    println!("   extent it writes: rc={rc4}, bytes [{f:?} ..= {l:?}] (claim: within 0..=119)");
}

// ============================================================ Stage 3

fn torus_deriv(i: usize, j: usize, u: f64, v: f64, maj: f64, min: f64) -> [f64; 3] {
    let radial = |jj: usize| -> f64 {
        match jj % 4 {
            0 => maj + min * v.cos(),
            1 => -min * v.sin(),
            2 => -min * v.cos(),
            _ => min * v.sin(),
        }
    };
    let rad = if j == 0 { radial(0) } else { radial(j) };
    let z = if i > 0 {
        0.0
    } else {
        match j % 4 {
            0 => min * v.sin(),
            1 => min * v.cos(),
            2 => -min * v.sin(),
            _ => -min * v.cos(),
        }
    };
    let (c, s) = (u.cos(), u.sin());
    let (cu, su) = match i % 4 {
        0 => (c, s),
        1 => (-s, c),
        2 => (-c, -s),
        _ => (s, -c),
    };
    [rad * cu, rad * su, z]
}

/// Wrapper's own slot_count / index_of, re-implemented verbatim from
/// crates/parasolid/src/jet.rs so we can compare against the kernel.
fn wrapper_slot_count(n_u: usize, n_v: usize, tri: bool) -> usize {
    if tri {
        let n = n_u.max(n_v);
        (n + 1) * (n + 2) / 2
    } else {
        (n_u + 1) * (n_v + 1)
    }
}
fn wrapper_index_of(n_u: usize, n_v: usize, tri: bool, i: usize, j: usize) -> Option<usize> {
    if tri {
        let n = n_u.max(n_v);
        if i + j > n {
            return None;
        }
        Some(j * (n + 1) - j * j.saturating_sub(1) / 2 + i)
    } else {
        if i > n_u || j > n_v {
            return None;
        }
        Some(j * (n_u + 1) + i)
    }
}

fn probe_eval(tor: &Surf, u: f64, v: f64, maj: f64, min: f64, n_u: i32, n_v: i32, tri: bool) {
    let claim = wrapper_slot_count(n_u as usize, n_v as usize, tri);
    // Huge sentinel buffer: 400 slots. Any write past `claim` is a heap smash
    // in the real wrapper.
    let cap = 400usize;
    let mut p = vec![SENT; cap * 3];
    let uv = [u, v];
    println!("\n>> about to call PK_SURF_eval(n_u={n_u}, n_v={n_v}, triangular={tri}) ...");
    use std::io::Write;
    std::io::stdout().flush().ok();
    let rc = unsafe {
        PK_SURF_eval(
            tor.tag(),
            uv.as_ptr(),
            n_u,
            n_v,
            if tri {
                PK_LOGICAL_true
            } else {
                PK_LOGICAL_false
            },
            p.as_mut_ptr(),
        )
    };
    let last_written = (0..cap * 3).filter(|&k| !is_sent(p[k])).max();
    let slots_written = last_written.map(|k| k / 3 + 1).unwrap_or(0);
    println!(
        "\n-- n_u={n_u} n_v={n_v} {}  rc={rc}  wrapper_slot_count={claim}  kernel_slots_written={slots_written}{}",
        if tri { "TRIANGULAR" } else { "rectangular" },
        if slots_written > claim {
            "   *** BUFFER OVERRUN ***"
        } else {
            ""
        }
    );
    if rc != PK_ERROR_no_errors {
        println!("   (call rejected)");
        return;
    }
    // For each written slot, identify which (i,j) it holds.
    for slot in 0..slots_written {
        let got = [p[slot * 3], p[slot * 3 + 1], p[slot * 3 + 2]];
        let mut m = Vec::new();
        for i in 0..=6usize {
            for j in 0..=6usize {
                let w = torus_deriv(i, j, u, v, maj, min);
                if (0..3).all(|k| (got[k] - w[k]).abs() < 1e-8) {
                    m.push(format!("d{i}u.d{j}v"));
                }
            }
        }
        // What does the wrapper think lives in this slot?
        let mut wrapper_says = Vec::new();
        for i in 0..=6usize {
            for j in 0..=6usize {
                if wrapper_index_of(n_u as usize, n_v as usize, tri, i, j) == Some(slot) {
                    wrapper_says.push(format!("d{i}u.d{j}v"));
                }
            }
        }
        let truth = if m.is_empty() {
            "??".to_string()
        } else {
            m.join("|")
        };
        let ws = if wrapper_says.is_empty() {
            "(unmapped)".to_string()
        } else {
            wrapper_says.join("|")
        };
        let flag = if !m.is_empty() && !wrapper_says.is_empty() && !m.contains(&wrapper_says[0]) {
            "   <<< MISMATCH"
        } else {
            ""
        };
        println!("   [{slot:2}] kernel={truth:24} wrapper_index_of says={ws}{flag}");
    }
}

fn stage3() {
    println!("\n================ STAGE 3: PK_SURF_eval derivative layout ================");
    let basis = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let (maj, min) = (5.0_f64, 1.5_f64);
    let tor = Surf::torus(basis, maj, min).expect("torus");
    let (u, v) = (0.6_f64, 0.9_f64);

    // Symmetric triangular, higher orders.
    probe_eval(&tor, u, v, maj, min, 2, 2, true);
    probe_eval(&tor, u, v, maj, min, 3, 3, true);
    probe_eval(&tor, u, v, maj, min, 4, 4, true);
    // Rectangular, asymmetric + higher order.
    probe_eval(&tor, u, v, maj, min, 3, 1, false);
    probe_eval(&tor, u, v, maj, min, 1, 3, false);
    probe_eval(&tor, u, v, maj, min, 4, 2, false);
    // ASYMMETRIC TRIANGULAR — the untested claim.
    probe_eval(&tor, u, v, maj, min, 3, 1, true);
    probe_eval(&tor, u, v, maj, min, 1, 3, true);
    probe_eval(&tor, u, v, maj, min, 4, 0, true);
    probe_eval(&tor, u, v, maj, min, 0, 4, true);

    // Same, but through the SAFE wrapper, to see what a caller gets.
    println!("\n-- via Surf::eval_jet (safe wrapper) --");
    for (nu, nv) in [(3usize, 1usize), (1, 3), (0, 4), (4, 0)] {
        match tor.eval_jet(u, v, nu, nv, true) {
            Ok(j) => {
                let mut bad = Vec::new();
                for i in 0..=4usize {
                    for jj in 0..=4usize {
                        if let Some(g) = j.d(i, jj) {
                            let w = torus_deriv(i, jj, u, v, maj, min);
                            let ok = (g.x - w[0]).abs() < 1e-8
                                && (g.y - w[1]).abs() < 1e-8
                                && (g.z - w[2]).abs() < 1e-8;
                            if !ok {
                                bad.push(format!("d{i}u.d{jj}v"));
                            }
                        }
                    }
                }
                println!(
                    "   eval_jet(n_u={nu}, n_v={nv}, tri) OK; wrong derivatives: {}",
                    if bad.is_empty() {
                        "none".to_string()
                    } else {
                        bad.join(", ")
                    }
                );
            }
            Err(e) => println!("   eval_jet(n_u={nu}, n_v={nv}, tri) rejected: {e:?}"),
        }
    }
}

// ============================================================ curvature

fn stage3_curvature() {
    println!("\n================ STAGE 3: curvature sign / direction pairing ================");
    let basis = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let (maj, min) = (5.0_f64, 1.5_f64);
    let tor = Surf::torus(basis, maj, min).expect("torus");

    for (label, uu, vv) in [
        ("outer equator (u=0,v=0)", 0.0, 0.0),
        ("inner equator (u=0,v=pi)", 0.0, std::f64::consts::PI),
        ("top (u=0,v=pi/2)", 0.0, std::f64::consts::FRAC_PI_2),
        ("generic (u=0.6,v=2.2)", 0.6, 2.2),
    ] {
        let c = tor.eval_curvature(uu, vv).expect("curv");
        let p = tor.eval(uu, vv).expect("eval");
        println!("\n-- torus {label}");
        println!("   P      = ({:.4},{:.4},{:.4})", p.x, p.y, p.z);
        println!(
            "   normal = ({:.4},{:.4},{:.4})",
            c.normal.x, c.normal.y, c.normal.z
        );
        println!(
            "   k1={:+.6}  dir1=({:.4},{:.4},{:.4})",
            c.principal_curvature_1,
            c.principal_direction_1.x,
            c.principal_direction_1.y,
            c.principal_direction_1.z
        );
        println!(
            "   k2={:+.6}  dir2=({:.4},{:.4},{:.4})",
            c.principal_curvature_2,
            c.principal_direction_2.x,
            c.principal_direction_2.y,
            c.principal_direction_2.z
        );
        // Independent check: normal curvature in a principal direction via
        // the second fundamental form, computed from a jet.
        let jet = tor.eval_jet(uu, vv, 2, 2, false).expect("jet");
        let n = jet.unit_normal().expect("normal");
        println!(
            "   jet unit normal = ({:.4},{:.4},{:.4})  (dot with reported = {:+.4})",
            n.x,
            n.y,
            n.z,
            n.x * c.normal.x + n.y * c.normal.y + n.z * c.normal.z
        );
        let ru = jet.d(1, 0).unwrap();
        let rv = jet.d(0, 1).unwrap();
        let ruu = jet.d(2, 0).unwrap();
        let ruv = jet.d(1, 1).unwrap();
        let rvv = jet.d(0, 2).unwrap();
        let dot = |a: Vec3, b: Vec3| a.x * b.x + a.y * b.y + a.z * b.z;
        let (e, f, g) = (dot(ru, ru), dot(ru, rv), dot(rv, rv));
        // Second fundamental form against the REPORTED normal.
        let nn = c.normal;
        let (l, m, nq) = (dot(ruu, nn), dot(ruv, nn), dot(rvv, nn));
        // Normal curvature along a direction expressed as a*ru + b*rv.
        let kn = |a: f64, b: f64| {
            (l * a * a + 2.0 * m * a * b + nq * b * b) / (e * a * a + 2.0 * f * a * b + g * b * b)
        };
        // Express dir1/dir2 in the (ru, rv) basis by least squares.
        let solve = |d: Vec3| -> (f64, f64) {
            let (p1, p2) = (dot(d, ru), dot(d, rv));
            let det = e * g - f * f;
            ((g * p1 - f * p2) / det, (e * p2 - f * p1) / det)
        };
        let (a1, b1) = solve(c.principal_direction_1);
        let (a2, b2) = solve(c.principal_direction_2);
        println!(
            "   2nd-fund-form k_n(dir1) = {:+.6}  vs reported k1 = {:+.6}   {}",
            kn(a1, b1),
            c.principal_curvature_1,
            if (kn(a1, b1) - c.principal_curvature_1).abs() < 1e-6 {
                "MATCH"
            } else {
                "*** PAIRING/SIGN MISMATCH ***"
            }
        );
        println!(
            "   2nd-fund-form k_n(dir2) = {:+.6}  vs reported k2 = {:+.6}   {}",
            kn(a2, b2),
            c.principal_curvature_2,
            if (kn(a2, b2) - c.principal_curvature_2).abs() < 1e-6 {
                "MATCH"
            } else {
                "*** PAIRING/SIGN MISMATCH ***"
            }
        );
        // And the cross-pairing, to prove it is NOT also a match by accident.
        println!(
            "   cross-check k_n(dir1) vs k2 = {:+.6} / {:+.6}  (should differ when k1 != k2)",
            kn(a1, b1),
            c.principal_curvature_2
        );
    }
}

// ============================================================ min radii

fn stage3_min_radii() {
    println!("\n================ STAGE 3: PK_SURF_find_min_radii buffer extent ================");
    let basis = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );

    let surfaces: Vec<(&str, Surf)> = vec![
        ("torus 5/1.5", Surf::torus(basis, 5.0, 1.5).unwrap()),
        (
            "torus 2/1.9 (near-degenerate)",
            Surf::torus(basis, 2.0, 1.9).unwrap(),
        ),
        ("sphere r=4", Surf::sphere(basis, 4.0).unwrap()),
        ("cylinder r=2", Surf::cylinder(basis, 2.0).unwrap()),
        ("cone", Surf::cone(basis, 3.0, 0.5).unwrap()),
        ("plane", Surf::plane(basis).unwrap()),
    ];

    for (name, s) in &surfaces {
        let ub = s.uvbox().expect("uvbox");
        let box_ = PK_UVBOX_t {
            param: [ub.u_min, ub.v_min, ub.u_max, ub.v_max],
        };
        let mut n: std::os::raw::c_int = -999;
        let mut radii = [SENT; 16];
        let mut positions = [[SENT; 3]; 16];
        let mut parms = [[SENT; 2]; 16];
        let rc = unsafe {
            PK_SURF_find_min_radii(
                s.tag(),
                &box_,
                &mut n,
                radii.as_mut_ptr(),
                positions.as_mut_ptr(),
                parms.as_mut_ptr(),
            )
        };
        let r_written = radii
            .iter()
            .rposition(|&x| !is_sent(x))
            .map(|k| k + 1)
            .unwrap_or(0);
        let p_written = positions
            .iter()
            .rposition(|v| v.iter().any(|&x| !is_sent(x)))
            .map(|k| k + 1)
            .unwrap_or(0);
        let q_written = parms
            .iter()
            .rposition(|v| v.iter().any(|&x| !is_sent(x)))
            .map(|k| k + 1)
            .unwrap_or(0);
        println!(
            "   {:32} rc={rc} n_radii={n}  slots written: radii={r_written} positions={p_written} parms={q_written}{}",
            name,
            if r_written > 2 || p_written > 2 || q_written > 2 {
                "   *** OVERRUN PAST 2 ***"
            } else {
                ""
            }
        );
        if n > 0 {
            for k in 0..(n.min(4) as usize) {
                // Cross-check: does eval(parms[k]) equal positions[k]?
                let e = s.eval(parms[k][0], parms[k][1]);
                let agree = match &e {
                    Ok(p) => {
                        (p.x - positions[k][0]).abs() < 1e-6
                            && (p.y - positions[k][1]).abs() < 1e-6
                            && (p.z - positions[k][2]).abs() < 1e-6
                    }
                    Err(_) => false,
                };
                println!(
                    "        [{k}] radius={:+.6} pos=({:.4},{:.4},{:.4}) uv=({:.4},{:.4})  eval(uv)={}  {}",
                    radii[k],
                    positions[k][0],
                    positions[k][1],
                    positions[k][2],
                    parms[k][0],
                    parms[k][1],
                    match &e {
                        Ok(p) => format!("({:.4},{:.4},{:.4})", p.x, p.y, p.z),
                        Err(er) => format!("{er:?}"),
                    },
                    if agree {
                        "consistent"
                    } else {
                        "*** position/param DISAGREE ***"
                    }
                );
            }
        }
    }
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    stage2();
    stage3();
    stage3_curvature();
    stage3_min_radii();

    // Repeat the eval probe with argument checking OFF, since the kernel's
    // n_u != n_v triangular rejection is gated on the checking flag.
    println!("\n\n################ DEFAULT SessionConfig (check_arguments unset) ################");
    drop(_s);
    let _s2 = Session::start(SessionConfig::new()).expect("session2");
    let basis = Axis2::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
    );
    let tor = Surf::torus(basis, 5.0, 1.5).expect("torus");
    println!(">> default session: Surf::eval_jet(0.6, 0.9, n_u=3, n_v=1, triangular=true)");
    use std::io::Write;
    std::io::stdout().flush().ok();
    match tor.eval_jet(0.6, 0.9, 3, 1, true) {
        Ok(j) => println!("   returned OK, {:?}", j.shape()),
        Err(e) => println!("   rejected: {e:?}"),
    }
    println!(">> SURVIVED the default-session call");

    println!("\n################ check_arguments(false) — isolated ################");
    drop(_s2);
    let _s3 = Session::start(SessionConfig::new().check_arguments(false)).expect("session3");
    let tor2 = Surf::torus(basis, 5.0, 1.5).expect("torus");
    println!(">> sanity: symmetric triangular first");
    std::io::stdout().flush().ok();
    println!(
        "   n_u=n_v=2 tri -> {:?}",
        tor2.eval_jet(0.6, 0.9, 2, 2, true).map(|j| j.shape())
    );
    println!(">> now Surf::eval_jet(0.6, 0.9, n_u=3, n_v=1, triangular=true) with checking OFF");
    std::io::stdout().flush().ok();
    match tor2.eval_jet(0.6, 0.9, 3, 1, true) {
        Ok(j) => println!("   returned OK, {:?}", j.shape()),
        Err(e) => println!("   rejected: {e:?}"),
    }
    println!(">> SURVIVED (no crash)");
}
