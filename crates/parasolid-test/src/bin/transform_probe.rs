//! Probe `PK_TRANSF_classify` tokens and input requirements (Stage 2).
//!
//! Two questions:
//!   1. Are the `PK_matrix_type_*` constants (25290..25294) real, or guessed
//!      like the `PK_ERROR_*` table was? A pure translation classifying as
//!      "unclassified" is the tell.
//!   2. What does `PK_TRANSF_create_rotation` require of its `axis`? It rejects
//!      (1,1,1) with 5019 while accepting (0,0,1), which smells like a
//!      unit-length precondition rather than a bad binding.

use parasolid::*;
use parasolid_sys::*;

fn classify_raw(t: &Transform, diagnostics: bool) -> PK_TRANSF_classify_r_t {
    let opts = PK_TRANSF_classify_o_t {
        o_t_version: 1,
        diagnostics: if diagnostics {
            PK_TRANSF_diagnostics_all_c
        } else {
            PK_TRANSF_diagnostics_none_c
        },
    };
    let mut r: PK_TRANSF_classify_r_t = unsafe { std::mem::zeroed() };
    let rc = unsafe { PK_TRANSF_classify(t.tag(), &opts, &mut r) };
    if rc != PK_ERROR_no_errors {
        println!("    (classify returned {rc})");
    }
    r
}

fn show(label: &str, t: &Transform) {
    let r = classify_raw(t, true);
    println!(
        "  {label:22} matrix_type={:<6} det={:<10.6} scale={:<10.6} transl=({:.3},{:.3},{:.3})",
        r.matrix_type, r.determinant, r.scale, r.translation[0], r.translation[1], r.translation[2]
    );
    println!(
        "  {:22} unit_dev=({:.2e},{:.2e},{:.2e}) orth_dev=({:.2e},{:.2e},{:.2e}) persp=({:.1},{:.1},{:.1})",
        "",
        r.unit_rows_deviations[0],
        r.unit_rows_deviations[1],
        r.unit_rows_deviations[2],
        r.orthog_rows_deviations[0],
        r.orthog_rows_deviations[1],
        r.orthog_rows_deviations[2],
        r.perspective[0],
        r.perspective[1],
        r.perspective[2]
    );
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");

    println!("== raw matrix_type tokens by construction ==");
    println!(
        "   (sys claims identity=25290 rotation=25291 reflection=25292 general=25293 unclassified=25294)\n"
    );

    let ident = Transform::from_matrix([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
    .expect("identity");
    show("identity", &ident);

    let transl = Transform::translation(3.0, -4.0, 5.0).expect("translation");
    show("translation", &transl);

    let rot_z = Transform::rotation(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        std::f64::consts::FRAC_PI_3,
    )
    .expect("rotation z");
    show("rotation about Z", &rot_z);

    let refl = Transform::reflection(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0))
        .expect("reflection");
    show("reflection", &refl);

    let scale = Transform::uniform_scale(4.0).expect("uniform scale");
    show("uniform scale x4", &scale);

    let scale_about =
        Transform::scale_about(2.5, Vec3::new(1.0, 2.0, 3.0)).expect("scale about point");
    show("scale about point", &scale_about);

    // Non-uniform scale on the diagonal — should be the "general" case.
    if let Ok(nonuni) = Transform::from_matrix([
        2.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]) {
        show("non-uniform scale", &nonuni);
    } else {
        println!("  non-uniform scale      REJECTED at construction");
    }

    // Shear — valid linear map, not a similarity.
    if let Ok(shear) = Transform::from_matrix([
        1.0, 0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ]) {
        show("shear", &shear);
    } else {
        println!("  shear                  REJECTED at construction");
    }

    println!("\n== matrix readback: where does the translation actually live? ==");
    for (label, t) in [
        ("our translation(3,-4,5)", &transl),
        ("native scale_about(2.5,(1,2,3))", &scale_about),
        ("native reflection", &refl),
    ] {
        let m = t.matrix().expect("ask matrix");
        println!("  {label}");
        for row in 0..4 {
            println!(
                "     [{:8.4} {:8.4} {:8.4} {:8.4}]",
                m[row * 4],
                m[row * 4 + 1],
                m[row * 4 + 2],
                m[row * 4 + 3]
            );
        }
    }

    println!("\n== isolating what drives matrix_type ==");
    let variants: &[(&str, [f64; 16])] = &[
        ("identity, m15=1", [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ]),
        ("transl only, m15=1", [
            1.0, 0.0, 0.0, 3.0, 0.0, 1.0, 0.0, -4.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 0.0, 1.0,
        ]),
        ("scale_about clone, m15=0.4", [
            1.0, 0.0, 0.0, -0.6, 0.0, 1.0, 0.0, -1.2, 0.0, 0.0, 1.0, -1.8, 0.0, 0.0, 0.0, 0.4,
        ]),
        ("transl, m15=0.4", [
            1.0, 0.0, 0.0, 3.0, 0.0, 1.0, 0.0, -4.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 0.0, 0.4,
        ]),
        ("identity, m15=0.4", [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.4,
        ]),
    ];
    for (label, m) in variants {
        match Transform::from_matrix(*m) {
            Ok(t) => {
                let r = classify_raw(&t, false);
                println!(
                    "  {label:28} matrix_type={} det={:.4} scale={:.4}",
                    r.matrix_type, r.determinant, r.scale
                );
            }
            Err(e) => println!("  {label:28} REJECTED: {e}"),
        }
    }

    println!("\n== raw classify result bytes (is matrix_type really @0?) ==");
    for (label, m) in [
        ("identity", [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0f64,
        ]),
        ("translation m15=1", [
            1.0, 0.0, 0.0, 3.0, 0.0, 1.0, 0.0, -4.0, 0.0, 0.0, 1.0, 5.0, 0.0, 0.0, 0.0, 1.0f64,
        ]),
    ] {
        let t = Transform::from_matrix(m).expect("build");
        let opts = PK_TRANSF_classify_o_t {
            o_t_version: 1,
            diagnostics: PK_TRANSF_diagnostics_all_c,
        };
        let mut buf = [0u8; 256];
        let rc =
            unsafe { PK_TRANSF_classify(t.tag(), &opts, buf.as_mut_ptr() as *mut PK_TRANSF_classify_r_t) };
        println!("  {label}  (rc={rc})");
        for row in 0..8 {
            let off = row * 16;
            let hex: Vec<String> = buf[off..off + 16].iter().map(|b| format!("{b:02x}")).collect();
            // Annotate as both i32 lanes and f64 lanes so a misplaced field shows up.
            let i0 = i32::from_le_bytes(buf[off..off + 4].try_into().unwrap());
            let f0 = f64::from_le_bytes(buf[off..off + 8].try_into().unwrap());
            let f1 = f64::from_le_bytes(buf[off + 8..off + 16].try_into().unwrap());
            println!(
                "    @{off:3}  {}  i32={i0:<8} f64=({f0:.4},{f1:.4})",
                hex.join(" ")
            );
        }
        let nonzero_tail: Vec<usize> = (120..256).filter(|&i| buf[i] != 0).collect();
        println!("    nonzero >=120: {nonzero_tail:?}");
    }

    println!("\n== PK_TRANSF_create_rotation: what does `axis` require? ==");
    let cases: &[(&str, Vec3)] = &[
        ("unit Z (0,0,1)", Vec3::new(0.0, 0.0, 1.0)),
        ("unit diag", {
            let s = 1.0 / 3.0_f64.sqrt();
            Vec3::new(s, s, s)
        }),
        ("non-unit (1,1,1)", Vec3::new(1.0, 1.0, 1.0)),
        ("non-unit (0,0,2)", Vec3::new(0.0, 0.0, 2.0)),
        ("zero vector", Vec3::new(0.0, 0.0, 0.0)),
    ];
    for (label, axis) in cases {
        match Transform::rotation(Vec3::new(0.0, 0.0, 0.0), *axis, 0.5) {
            Ok(_) => println!("  {label:20} accepted"),
            Err(e) => {
                let token = e
                    .details()
                    .and_then(|d| d.code_token.clone())
                    .unwrap_or_default();
                println!("  {label:20} REJECTED: {token}");
            }
        }
    }

    println!("\n== done");
}
