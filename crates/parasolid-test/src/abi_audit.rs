//! Regression tests for the 2026-10-16 ABI audit (parasolid-re
//! `cadabra-HANDOFF.md` §3, `artifacts/port/oracle/README.md` "ABI basis",
//! `catalog/pk-validated-abi.md`).
//!
//! Each test is written so that it would have **failed** against the bindings
//! before the corresponding fix; the comment on each says how.

use parasolid::*;
use parasolid_sys::*;
use std::error::Error;

pub type R = Result<(), Box<dyn Error>>;

fn cfg() -> SessionConfig {
    SessionConfig::new().check_arguments(true)
}

fn z_basis(origin: Vec3) -> Axis2 {
    Axis2::new(origin, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
}

fn translate(body: &Body, dx: f64, dy: f64, dz: f64) -> Result<(), Box<dyn Error>> {
    body.transform(&Transform::translation(dx, dy, dz)?)?;
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect 1 — PK_LOGICAL_t is one byte
// -----------------------------------------------------------------------------

/// The kernel writes exactly one byte through a `PK_LOGICAL_t *` output. With
/// the old 4-byte typedef the three bytes after it were caller garbage.
pub fn abi_logical_is_one_byte() -> R {
    let _s = Session::start(cfg())?;
    let mut buf = [0xAAu8; 4];
    let rc = unsafe { PK_SESSION_ask_check_arguments(buf.as_mut_ptr()) };
    assert_eq!(rc, 0);
    assert_eq!(buf[0], 1, "check_arguments is on");
    assert_eq!(
        buf[1..],
        [0xAA; 3],
        "kernel must write only one byte: {buf:x?}"
    );
    assert_eq!(std::mem::size_of::<PK_LOGICAL_t>(), 1);
    Ok(())
}

/// A logical *array* output (`PK_SHELL_ask_oriented_faces`). The void region's
/// shell sees every face from outside, so all orientations are the opposite of
/// the solid shell's. Read as 4-byte ints, the all-`true` byte array decoded as
/// 0x01010101 / out-of-bounds bytes, i.e. all `false`, and this test failed.
pub fn abi_logical_array_shell_orientations() -> R {
    let _s = Session::start(cfg())?;
    let body = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let regions = body.regions()?;
    assert_eq!(regions.len(), 2);
    let mut seen = std::collections::HashMap::new();
    for r in &regions {
        let solid = r.is_solid()?;
        for sh in r.shells()? {
            let of = sh.oriented_faces()?;
            assert_eq!(of.len(), 6, "each shell sees all 6 faces");
            let orients: Vec<bool> = of.iter().map(|(_, o)| *o).collect();
            // Every face in a shell has the same orientation on a block.
            assert!(
                orients.iter().all(|&o| o == orients[0]),
                "uniform orientation per shell: {orients:?}"
            );
            for (f, o) in of {
                seen.entry(f.tag())
                    .or_insert_with(Vec::new)
                    .push((solid, o));
            }
        }
    }
    assert_eq!(seen.len(), 6);
    for (tag, v) in seen {
        assert_eq!(v.len(), 2, "face {tag} in both shells");
        assert_ne!(
            v[0].1, v[1].1,
            "face {tag}: opposite orientation in the two shells"
        );
        // Solid shell: false (normal points out of the material), void: true.
        for (solid, o) in v {
            assert_eq!(o, !solid, "face {tag}: solid={solid} orient={o}");
        }
    }
    Ok(())
}

/// `PK_TOPOL_eval_mass_props_o_t` at version 2 reads `use_facets` at byte 17.
/// Under the old 4-byte `single` that byte happened to be zero; once `single`
/// became one byte it was padding and the kernel returned 908 `not_a_logical`.
pub fn abi_mass_props_v2_reads_use_facets() -> R {
    let _s = Session::start(cfg())?;
    let b = Body::create_solid_block(2.0, 3.0, 4.0)?;
    let mut o = PK_TOPOL_eval_mass_props_o_t::default();
    assert_eq!(o.o_t_version, 2);
    let call = |o: &PK_TOPOL_eval_mass_props_o_t| {
        let (mut amount, mut mass, mut per) = (0.0, 0.0, 0.0);
        let (mut cg, mut mi) = ([0.0; 3], [0.0; 9]);
        let t = b.tag();
        let rc = unsafe {
            PK_TOPOL_eval_mass_props(
                1,
                &t,
                0.99,
                o,
                &mut amount,
                &mut mass,
                &mut cg,
                &mut mi,
                &mut per,
            )
        };
        (rc, amount)
    };
    let (rc, amount) = call(&o);
    assert_eq!(rc, 0);
    assert!((amount - 24.0).abs() < 1e-9, "volume {amount}");
    // The field is live: a non-logical byte there is refused.
    o.use_facets = 0xAB;
    assert_eq!(
        call(&o).0,
        PK_ERROR_not_a_logical,
        "use_facets is read at v2"
    );
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect 2 — PK_check_fault_t is 40 bytes; check options are tokens
// -----------------------------------------------------------------------------

/// `PK_GEOM_check` on a line whose direction is off unit length by ~4e-11
/// (built with argument checking off) reports two faults, both naming the
/// line: `PK_LINE_state_bad_unit_vec_c` then `PK_ENTITY_state_invalid_c`
/// (parasolid-re P9 probe `PASSC_curve_geomcheck`). The 12-byte decode read
/// `entity` from the state slot and the second fault from byte 12.
pub fn check_fault_is_40_bytes() -> R {
    let _s = Session::start(SessionConfig::new().check_arguments(false))?;
    let sf = PK_LINE_sf_t {
        basis_set: PK_AXIS1_sf_t {
            location: [1.0, 0.0, 0.0],
            axis: [f64::from_bits(0x3ff000000002bfb0), 0.0, 0.0],
        },
    };
    let mut line: PK_LINE_t = 0;
    assert_eq!(unsafe { PK_LINE_create(&sf, &mut line) }, 0);
    let opts = GeomCheckOptions {
        max_faults: 10,
        extra_faults: PK_check_extra_faults_latest_c,
        ..Default::default()
    };
    let faults = check_geom(line, Some(&opts))?;
    let got: Vec<(i32, Option<i32>)> = faults
        .iter()
        .map(|f| (f.state, f.entity.map(|e| e.tag())))
        .collect();
    assert_eq!(
        got,
        vec![
            (PK_LINE_state_bad_unit_vec_c, Some(line)),
            (PK_ENTITY_state_invalid_c, Some(line)),
        ],
        "faults {faults:?}"
    );
    // Default options (max_faults 1, extra_faults 0) report one fault; NULL
    // options give the same answer, consistent with Default == kernel default.
    let d = check_geom(line, Some(&GeomCheckOptions::default()))?;
    let n = check_geom(line, None)?;
    assert_eq!(d, n, "Default options equal NULL options");
    assert_eq!(d.len(), 1);
    // Observed: with max_faults 1 the kernel reports the generic fault first.
    assert_eq!(d[0].state, PK_ENTITY_state_invalid_c);
    assert_eq!(d[0].entity.map(|e| e.tag()), Some(line));
    Ok(())
}

/// The four topology check calls accept full, explicit option structs at their
/// current versions, report nothing on a clean block, and reject version+1.
pub fn check_option_structs_round_trip() -> R {
    let _s = Session::start(cfg())?;
    let b = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let faces = b.faces()?;
    let edges = b.edges()?;
    assert!(b.check_with(&BodyCheckOptions::default())?.is_empty());
    assert!(
        faces[0]
            .check_with(&FaceCheckOptions::default())?
            .is_empty()
    );
    assert!(
        edges[0]
            .check_with(&EdgeCheckOptions::default())?
            .is_empty()
    );
    assert!(
        faces[0]
            .check_pair_with(&faces[1], &FacePairCheckOptions::default())?
            .is_empty()
    );
    assert!(faces[0].check()?.is_empty() && edges[0].check()?.is_empty());
    assert!(faces[0].check_pair(&faces[1])?.is_empty());
    let code = |r: PsResult<Vec<CheckFault>>| match r {
        Err(e) => e.details().map(|d| d.code),
        Ok(_) => None,
    };
    let unknown = Some(PK_ERROR_o_t_version_unknown);
    let bo = BodyCheckOptions {
        o_t_version: 6,
        ..Default::default()
    };
    assert_eq!(code(b.check_with(&bo)), unknown);
    let fo = FaceCheckOptions {
        o_t_version: 5,
        ..Default::default()
    };
    assert_eq!(code(faces[0].check_with(&fo)), unknown);
    let eo = EdgeCheckOptions {
        o_t_version: 4,
        ..Default::default()
    };
    assert_eq!(code(edges[0].check_with(&eo)), unknown);
    let po = FacePairCheckOptions {
        o_t_version: 2,
        ..Default::default()
    };
    assert_eq!(code(faces[0].check_pair_with(&faces[1], &po)), unknown);
    // A logical where a token belongs is refused: the fields are enums.
    let bad = BodyCheckOptions {
        geom: 1,
        ..Default::default()
    };
    assert!(
        b.check_with(&bad).is_err(),
        "geom=1 is not a PK_check_geom_t token"
    );
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect 3 — PK_BODY_imprint_o_t / Body::imprint_body
// -----------------------------------------------------------------------------

/// parasolid-re fixture `r1-blind-hole` / `repro-imprint-body-v1-options`:
/// block 40×40×20 and an r=4, h=20 cylinder moved to (3,-2,10). The old
/// wrapper passed version 1, with which the kernel null-derefs (process crash),
/// so this test could not have run. Expected (dynamic-observed in parasolid-re
/// `runs/20261016T032759Z`): target 7/13/8, tool 4/3/0, one paired edge.
pub fn imprint_body_v5_pairs_edges() -> R {
    let _s = Session::start(cfg())?;
    let a = Body::create_solid_block(40.0, 40.0, 20.0)?;
    let b = Body::create_solid_cylinder(4.0, 20.0)?;
    translate(&b, 3.0, -2.0, 10.0)?;
    let r = a.imprint_body(&b)?;
    let fev = |x: &Body| -> Result<(usize, usize, usize), Box<dyn Error>> {
        Ok((x.faces()?.len(), x.edges()?.len(), x.vertices()?.len()))
    };
    assert_eq!(fev(&a)?, (7, 13, 8), "target imprinted");
    assert_eq!(
        fev(&b)?,
        (4, 3, 0),
        "tool imprinted (imprint_tool default true)"
    );
    assert_eq!(r.target_edges.len(), 1, "{r:?}");
    assert_eq!(r.tool_edges.len(), 1);
    assert_eq!(r.target_vertices.len(), r.tool_vertices.len());
    for (te, oe) in r.target_edges.iter().zip(&r.tool_edges) {
        assert_eq!(te.entity().class()?, PkClass::Edge);
        let tc = te.curve()?.ask_circle()?;
        let oc = oe.curve()?.ask_circle()?;
        assert!((tc.radius - 4.0).abs() < 1e-9 && (oc.radius - 4.0).abs() < 1e-9);
        for c in [tc, oc] {
            let o = c.basis.origin;
            assert!(
                (o.x - 3.0).abs() < 1e-9 && (o.y + 2.0).abs() < 1e-9 && (o.z - 20.0).abs() < 1e-9,
                "paired circle on the top face: {o:?}"
            );
        }
        assert!(
            a.edges()?.iter().any(|e| e.tag() == te.tag()),
            "target edge belongs to the target"
        );
        assert!(
            b.edges()?.iter().any(|e| e.tag() == oe.tag()),
            "tool edge belongs to the tool"
        );
    }
    Ok(())
}

/// The safe wrapper refuses the crashing option versions without calling the
/// kernel, and the kernel refuses version 6.
pub fn imprint_body_refuses_crashing_versions() -> R {
    let _s = Session::start(cfg())?;
    let a = Body::create_solid_block(40.0, 40.0, 20.0)?;
    let b = Body::create_solid_cylinder(4.0, 20.0)?;
    translate(&b, 3.0, -2.0, 10.0)?;
    for v in [0, 1, 6] {
        let o = PK_BODY_imprint_o_t {
            o_t_version: v,
            ..Default::default()
        };
        assert!(a.imprint_body_with(&b, &o).is_err(), "version {v} refused");
    }
    assert_eq!(a.faces()?.len(), 6, "nothing imprinted by refused calls");
    // Version 2 (the lowest that builds the default matched_region) works.
    let o = PK_BODY_imprint_o_t {
        o_t_version: 2,
        ..Default::default()
    };
    a.imprint_body_with(&b, &o)?;
    assert_eq!(a.faces()?.len(), 7);
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect 4 — PK_BODY_sew_bodies has 10 arguments
// -----------------------------------------------------------------------------

/// Two 10×10 planar sheets sharing the edge x = 5. With the 3-argument
/// declaration the kernel read `gap_width_bound` from an integer register and
/// wrote six outputs through garbage argument slots. NULL options crash the
/// kernel (no NULL branch in the migration), so the v8 defaults are passed.
pub fn sew_bodies_ten_args() -> R {
    let _s = Session::start(cfg())?;
    let s1 = Body::create_sheet_rectangle(10.0, 10.0, z_basis(Vec3::zero()))?;
    let s2 = Body::create_sheet_rectangle(10.0, 10.0, z_basis(Vec3::new(10.0, 0.0, 0.0)))?;
    let bodies = [s1.tag(), s2.tag()];
    let opts = PK_BODY_sew_bodies_o_t::default();
    let (mut n_sewn, mut n_unsewn, mut n_prob) = (-1, -1, -1);
    let mut sewn: *mut PK_BODY_t = std::ptr::null_mut();
    let mut unsewn: *mut PK_BODY_t = std::ptr::null_mut();
    let mut prob: *mut PK_BODY_problem_group_t = std::ptr::null_mut();
    let rc = unsafe {
        PK_BODY_sew_bodies(
            2,
            bodies.as_ptr(),
            1e-6,
            &opts,
            &mut n_sewn,
            &mut sewn,
            &mut n_unsewn,
            &mut unsewn,
            &mut n_prob,
            &mut prob,
        )
    };
    assert_eq!(rc, 0, "sew_bodies rc");
    assert_eq!((n_sewn, n_unsewn, n_prob), (1, 0, 0));
    let out = Body::from_tag_checked(unsafe { *sewn })?;
    assert_eq!(out.body_type()?, BodyType::Sheet);
    assert_eq!(
        (
            out.faces()?.len(),
            out.edges()?.len(),
            out.vertices()?.len()
        ),
        (2, 7, 6),
        "two quads glued along one edge"
    );
    assert!(out.check()?.is_empty());
    unsafe {
        PK_MEMORY_free(sewn as *mut _);
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect 5 — PK_CLASS_* tokens
// -----------------------------------------------------------------------------

/// Raw class tokens of entities whose kind is known by construction.
/// Before the fix: a b-curve read as `Icurve`, an SSI quartic as `Bcurve`,
/// and an offset surface as `Fsurf`.
pub fn classes_are_vendor_tokens() -> R {
    let _s = Session::start(cfg())?;
    let raw = |e: Entity| -> Result<i32, Box<dyn Error>> { Ok(e.class()?.to_raw()) };
    // b-curve
    let cps = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 0.0),
        Vec3::new(3.0, 2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
    ];
    let bc = Curve::bcurve(3, &cps, &[0.0, 1.0], &[4, 4])?;
    assert_eq!(raw(bc.entity())?, 3005);
    assert_eq!(bc.curve_type()?, CurveType::Bcurve);
    // ellipse
    let el = Curve::ellipse(z_basis(Vec3::zero()), 3.0, 2.0)?;
    assert_eq!(raw(el.entity())?, PK_CLASS_ellipse);
    // intersection curve: unequal perpendicular cylinders meet in a quartic
    let c1 = Surf::cylinder(z_basis(Vec3::zero()), 5.0)?;
    let c2 = Surf::cylinder(
        Axis2::new(
            Vec3::zero(),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        3.0,
    )?;
    let ssi = c1.intersect(&c2)?;
    assert!(!ssi.curves.is_empty());
    for c in &ssi.curves {
        assert_eq!(raw(c.curve.entity())?, 3004);
        assert_eq!(c.curve.curve_type()?, CurveType::Icurve);
    }
    // offset surface of a spun b-curve (cannot simplify to an analytic form)
    let prof = Curve::bcurve(
        3,
        &[
            Vec3::new(5.0, 0.0, 0.0),
            Vec3::new(6.0, 0.0, 1.0),
            Vec3::new(5.5, 0.0, 2.0),
            Vec3::new(7.0, 0.0, 3.0),
        ],
        &[0.0, 1.0],
        &[4, 4],
    )?;
    let spun = Surf::spun(&prof, Vec3::zero(), Vec3::new(0.0, 0.0, 1.0))?;
    assert_eq!(raw(spun.entity())?, 4010);
    let off = Surf::offset_surface(&spun, 0.5)?;
    assert_eq!(raw(off.entity())?, 4008, "offset surface class");
    assert_eq!(off.surf_type()?, SurfType::Offset);
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect 6 — a failed Boolean is an error
// -----------------------------------------------------------------------------

/// parasolid-re fixture `r1-cyl-tangent-box-face-internal`: an r=5 cylinder
/// internally tangent to a box side face. `PK_BODY_boolean_2` returns no error
/// but `PK_boolean_result_failed_c` with two `non_manifold` reports. The old
/// wrapper returned `Ok([target])`.
pub fn boolean_failed_is_an_error() -> R {
    let _s = Session::start(cfg())?;
    let mk = || -> Result<(Body, Body), Box<dyn Error>> {
        let a = Body::create_solid_block(40.0, 40.0, 20.0)?;
        let b = Body::create_solid_cylinder(5.0, 40.0)?;
        translate(&b, 15.0, 0.0, -10.0)?;
        Ok((a, b))
    };
    let (a, b) = mk()?;
    match boolean::boolean(a, vec![b], BooleanOp::Subtract, &BooleanOptions::new()) {
        Err(PsError::BooleanFailed { result, reports }) => {
            assert_eq!(result, PK_boolean_result_failed_c);
            assert_eq!(reports.len(), 2, "{reports:?}");
            assert!(
                reports
                    .iter()
                    .all(|(r, _)| *r == PK_boolean_report_non_manifold_c)
            );
        }
        other => panic!("expected BooleanFailed, got {other:?}"),
    }
    let (a, b) = mk()?;
    let out = boolean_detailed(a, vec![b], BooleanOp::Subtract, &BooleanOptions::new())?;
    assert_eq!(out.result, BooleanResult::Failed);
    assert_eq!(out.reports.len(), 2);
    // A good subtract still succeeds through both paths.
    let a = Body::create_solid_block(40.0, 40.0, 20.0)?;
    let b = Body::create_solid_cylinder(4.0, 40.0)?;
    translate(&b, 0.0, 0.0, -10.0)?;
    let out = boolean_detailed(a, vec![b], BooleanOp::Subtract, &BooleanOptions::new())?;
    assert_eq!(out.result, BooleanResult::Success);
    assert!(out.reports.is_empty());
    assert_eq!(out.bodies.len(), 1);
    let v = out.bodies[0].mass_props()?.amount;
    let want = 40.0 * 40.0 * 20.0 - std::f64::consts::PI * 16.0 * 20.0;
    assert!(
        (v - want).abs() < 1e-6 * want,
        "through-hole volume {v} vs {want}"
    );
    Ok(())
}

// -----------------------------------------------------------------------------
// Documented behaviour — PK_BODY_transform drops tiny motions
// -----------------------------------------------------------------------------

/// parasolid-re `specs/repro/repro-transform-snap.json`: `PK_BODY_transform`
/// leaves the body unchanged for a pure translation of magnitude <= 1e-8
/// (session precision) or a rotation of angle <= 1e-11 rad (angular
/// precision), and returns no error. Band sweeps must not move a body by a
/// small transform around a zero nominal.
pub fn transform_snaps_tiny_motions() -> R {
    let _s = Session::start(cfg())?;
    let zmin_after = |dz: f64| -> Result<f64, Box<dyn Error>> {
        let c = Body::create_solid_cylinder(5.0, 10.0)?;
        translate(&c, 0.0, 0.0, dz)?;
        Ok(c.bounding_box()?.min.z)
    };
    assert_eq!(zmin_after(5e-9)?, 0.0, "5e-9 translation ignored");
    assert_eq!(zmin_after(1e-8)?, 0.0, "1e-8 translation ignored");
    let z = zmin_after(2e-8)?;
    assert!((z - 2e-8).abs() < 1e-15, "2e-8 translation applied: {z:e}");
    let ymin_after = |angle: f64| -> Result<f64, Box<dyn Error>> {
        let c = Body::create_solid_cylinder(5.0, 10.0)?;
        c.transform(&Transform::rotation(
            Vec3::zero(),
            Vec3::new(1.0, 0.0, 0.0),
            angle,
        )?)?;
        Ok(c.bounding_box()?.min.y)
    };
    let y0 = ymin_after(0.0)?;
    assert_eq!(ymin_after(1e-11)?, y0, "1e-11 rad rotation ignored");
    assert_ne!(ymin_after(1e-9)?, y0, "1e-9 rad rotation applied");
    Ok(())
}
