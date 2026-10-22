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

// -----------------------------------------------------------------------------
// Defect 8 — PK_SESSION_kernel_version_t is 36 bytes (parasolid-re REQUESTS R-7)
// -----------------------------------------------------------------------------

/// `PK_SESSION_ask_kernel_version` @0x1803f7400 stores major/minor/build at
/// +0/+4/+8, then scans the embedded build stamp with `"%d-%d-%d %d:%d:%d"`
/// (format @0x183fb50f8) into six more ints at +0xc..+0x20: nine ints, 36
/// bytes (static-observed, objdump of DLL c900fa3f…63073). The old 32-byte
/// struct let the kernel write 4 bytes past a stack allocation, which
/// corrupted the caller's saved registers (cadabra3
/// `docs/plan/rethink-2026-10/parasolid-baseline.md`).
pub fn kernel_version_is_36_bytes() -> R {
    let s = Session::start(cfg())?;
    let mut buf = [0xAAu8; 256];
    let rc = unsafe {
        PK_SESSION_ask_kernel_version(buf.as_mut_ptr().cast::<PK_SESSION_kernel_version_t>())
    };
    assert_eq!(rc, 0);
    let written = buf.iter().rposition(|&b| b != 0xAA).map_or(0, |i| i + 1);
    assert_eq!(written, 36, "kernel writes nine ints: {:x?}", &buf[..48]);
    assert_eq!(std::mem::size_of::<PK_SESSION_kernel_version_t>(), 36);
    assert_eq!(s.kernel_version()?, (37, 1, 243));
    let v = s.kernel_version_info()?;
    assert_eq!(
        (v.year, v.month, v.day, v.hour, v.minute, v.second),
        (2025, 8, 14, 22, 57, 46),
        "build stamp of 37.01.243"
    );
    Ok(())
}

// -----------------------------------------------------------------------------
// Probe — what SurfIntersection::delete_curves takes with it (REQUESTS R-7)
// -----------------------------------------------------------------------------

/// Records which SSI results share structure with their input surfaces: after
/// `delete_curves()`, are the two inputs still entities?
pub fn ssi_delete_curves_ownership() -> R {
    let _s = Session::start(cfg())?;
    let x_axis = Axis2::new(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
    // Analytic result: plane through a cylinder's axis gives two lines.
    let cyl = Surf::cylinder(z_basis(Vec3::zero()), 5.0)?;
    let pln = Surf::plane(x_axis)?;
    let ssi = cyl.intersect(&pln)?;
    let classes: Vec<i32> = ssi
        .curves
        .iter()
        .map(|c| c.curve.entity().class().map(|k| k.to_raw()))
        .collect::<Result<_, _>>()?;
    ssi.delete_curves()?;
    let analytic = (cyl.entity().is_valid()?, pln.entity().is_valid()?);
    println!("    plane x cylinder: classes {classes:?}, inputs alive after delete {analytic:?}");
    // Intersection-curve result: unequal perpendicular cylinders.
    let c1 = Surf::cylinder(z_basis(Vec3::zero()), 5.0)?;
    let c2 = Surf::cylinder(x_axis, 3.0)?;
    let ssi = c1.intersect(&c2)?;
    let classes: Vec<i32> = ssi
        .curves
        .iter()
        .map(|c| c.curve.entity().class().map(|k| k.to_raw()))
        .collect::<Result<_, _>>()?;
    ssi.delete_curves()?;
    let icurve = (c1.entity().is_valid()?, c2.entity().is_valid()?);
    println!("    cylinder x cylinder: classes {classes:?}, inputs alive after delete {icurve:?}");
    // The same pair on surfaces that faces of two bodies use.
    let b1 = Body::create_solid_cylinder(5.0, 20.0)?;
    let b2 = Body::create_solid_cylinder(3.0, 20.0)?;
    b2.transform(&Transform::rotation(
        Vec3::new(0.0, 0.0, 10.0),
        Vec3::new(0.0, 1.0, 0.0),
        std::f64::consts::FRAC_PI_2,
    )?)?;
    let side = |b: &Body| -> Result<Surf, Box<dyn Error>> {
        for f in b.faces()? {
            let s = f.surf()?;
            if s.surf_type()? == SurfType::Cylinder {
                return Ok(s);
            }
        }
        Err("no cylindrical face".into())
    };
    let (f1, f2) = (side(&b1)?, side(&b2)?);
    let refused = f1.intersect(&f2);
    println!("    face surfaces: {:?}", refused.as_ref().map(|r| r.curves.len()));
    assert!(
        matches!(&refused, Err(PsError::Mild(d)) if d.code == PK_ERROR_bad_shared_entity),
        "PK_SURF_intersect_surf refuses surfaces used by faces (PK_ERROR_bad_shared_entity)"
    );
    assert_eq!((b1.check()?.len(), b2.check()?.len()), (0, 0));
    // dynamic-observed 2026-10-16, DLL c900fa3f…63073: an icurve result owns
    // its orphan input surfaces; analytic results own nothing; surfaces used
    // by faces never reach PK_SURF_intersect_surf.
    assert_eq!(analytic, (true, true), "analytic SSI curves own nothing");
    assert_eq!(icurve, (false, false), "deleting icurves deletes orphan inputs");
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect 7 — PK_BODY_boolean_o_t is the 168-byte v19 layout, default_tol @48
// -----------------------------------------------------------------------------

/// parasolid-re C222 / catalog "PK_BODY_boolean_2 options". The old 32-byte
/// struct named @16 `default_tol`; in v2 that offset is the `matched_region`
/// pointer, so any non-zero tolerance was an access violation, and
/// `size_of` was 32. With the v19 struct:
/// - `size_of == 168`, `default_tol` at @48;
/// - the P7 coaxial equal-overlap lateral band (fixtures
///   `p9-boolean-dtol-coax-p1T-*`: two r=5 h=20 cylinders, B raised 10 and
///   shifted ~1e-8 laterally, whole pair rotated/translated off-axis) unites
///   to 5 faces / 4 edges with exactly one tolerant edge whose precision is
///   exactly the option value (5e-6, 3e-5, and the kernel's 1e-5 default);
/// - `Default` reproduces the kernel-default result (minimal v2 reference) on an ordinary
///   Boolean.
pub fn boolean_options_v19_default_tol() -> R {
    assert_eq!(std::mem::size_of::<PK_BODY_boolean_o_t>(), 168);
    assert_eq!(std::mem::offset_of!(PK_BODY_boolean_o_t, default_tol), 48);
    let _s = Session::start(cfg())?;

    let axis = Vec3::new(0.36, -0.48, 0.8);
    let lat = Vec3::new(0.6, 0.8, 0.0);
    let place = |b: &Body| -> Result<(), Box<dyn Error>> {
        b.transform(&Transform::rotation(Vec3::zero(), axis, 0.723_1)?)?;
        translate(b, 3.217, -1.843, 2.659)
    };
    let band = |tol: Option<f64>| -> Result<(usize, usize, Vec<f64>), Box<dyn Error>> {
        let a = Body::create_solid_cylinder(5.0, 20.0)?;
        place(&a)?;
        let b = Body::create_solid_cylinder(5.0, 20.0)?;
        translate(&b, 0.0, 0.0, 10.0)?;
        // Out and back: the net lateral offset is ~1e-8, below session precision.
        translate(&b, lat.x * 1.000_000_01, lat.y * 1.000_000_01, 0.0)?;
        translate(&b, -lat.x, -lat.y, 0.0)?;
        place(&b)?;
        let mut o = BooleanOptions::new();
        if let Some(t) = tol {
            o = o.default_tol(t);
        }
        let out = boolean_detailed(a, vec![b], BooleanOp::Unite, &o)?;
        assert_eq!(out.result, BooleanResult::Success, "tol {tol:?}");
        assert_eq!(out.bodies.len(), 1);
        let body = &out.bodies[0];
        let mut tolerant = Vec::new();
        for e in body.edges()? {
            if let Ok(p) = e.precision() {
                // Exact edges report half the session precision (5e-9).
                if p > 1e-8 {
                    tolerant.push(p);
                }
            }
        }
        Ok((body.faces()?.len(), body.edges()?.len(), tolerant))
    };
    for (tol, want) in [(None, 1e-5), (Some(5e-6), 5e-6), (Some(3e-5), 3e-5)] {
        let (f, e, tolerant) = band(tol)?;
        assert_eq!((f, e), (5, 4), "tol {tol:?}");
        assert_eq!(tolerant, vec![want], "tol {tol:?}: tolerant edge precisions");
    }

    // Default == the kernel's defaults on an ordinary Boolean: compare against
    // the minimal v2 struct (catalog: version 2 supplies only function,
    // configuration, matched_region, three flags and fence; everything else
    // comes from the kernel's own defaults). Raw NULL options crash this
    // entry point, so v2 is the independent reference.
    #[repr(C)]
    struct V2 {
        o_t_version: i32,
        function: i32,
        configuration: *const std::ffi::c_void,
        matched_region: *const std::ffi::c_void,
        flags: [u8; 3],
        _pad: u8,
        fence: i32,
    }
    assert_eq!(std::mem::size_of::<V2>(), 32);
    let mk = || -> Result<(Body, Body), Box<dyn Error>> {
        let a = Body::create_solid_block(40.0, 40.0, 20.0)?;
        let b = Body::create_solid_cylinder(4.0, 40.0)?;
        translate(&b, 0.0, 0.0, -10.0)?;
        Ok((a, b))
    };
    let (a, b) = mk()?;
    let mine = boolean_detailed(a, vec![b], BooleanOp::Subtract, &BooleanOptions::new())?;
    let (a, b) = mk()?;
    let tools = [b.tag()];
    let v2 = V2 {
        o_t_version: 2,
        function: PK_boolean_subtract_c,
        configuration: std::ptr::null(),
        matched_region: std::ptr::null(),
        flags: [0; 3],
        _pad: 0,
        fence: PK_boolean_fence_none_c,
    };
    let mut tr: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
    let mut r: PK_boolean_r_t = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        PK_BODY_boolean_2(
            a.tag(),
            1,
            tools.as_ptr(),
            &v2 as *const V2 as *const PK_BODY_boolean_o_t,
            &mut tr,
            &mut r,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(r.n_bodies, 1);
    let reference = Body::from_tag_checked(unsafe { *r.bodies })?;
    let ref_result = r.result;
    unsafe {
        PK_TOPOL_track_r_f(&mut tr);
        PK_boolean_r_f(&mut r);
    }
    assert_eq!(mine.result_raw, ref_result);
    assert_eq!(mine.result, BooleanResult::Success);
    assert_eq!(mine.bodies.len(), 1);
    let m = &mine.bodies[0];
    assert_eq!(m.faces()?.len(), reference.faces()?.len());
    assert_eq!(m.edges()?.len(), reference.edges()?.len());
    let (vm, vr) = (m.mass_props()?.amount, reference.mass_props()?.amount);
    assert!((vm - vr).abs() < 1e-9 * vr, "volume {vm} vs {vr}");
    Ok(())
}

// -----------------------------------------------------------------------------
// BooleanOptions merge_imprinted @32 / selective_merge @41 (v19)
// -----------------------------------------------------------------------------

/// parasolid-re item n. The wrapper did not expose the two merge flags; with
/// them, measured live (2026-10-16, DLL c900fa3f…63073), on 10×10×10 blocks:
/// - default Booleans keep coplanar regions as separate faces: abutting
///   blocks unite to F=10, half-overlapping ones to F=14;
/// - `merge_imprinted` merges them to the 6-face box;
/// - it merges only seams the Boolean imprinted: in `(A ∪ B) ∪ C` the A|B
///   seams made by the first (unmerged) Boolean survive (F=10);
/// - `selective_merge` matters only when the new imprint lies on such a
///   pre-existing seam (C spans x 5..25 over the A|B seam at x=5):
///   `merge_imprinted` alone also merges it (F=6), with `selective_merge` it
///   survives (F=10); without `merge_imprinted` it changes nothing.
/// Every result checks clean.
pub fn boolean_options_merge_flags() -> R {
    let _s = Session::start(cfg())?;
    let block = |dx: f64, w: f64| -> Result<Body, Box<dyn Error>> {
        let b = Body::create_solid_block(w, 10.0, 10.0)?;
        translate(&b, dx, 0.0, 0.0)?;
        Ok(b)
    };
    let unite = |t: Body, tool: Body, m: bool, s: bool| -> Result<usize, Box<dyn Error>> {
        let o = BooleanOptions::new().merge_imprinted(m).selective_merge(s);
        let out = boolean_detailed(t, vec![tool], BooleanOp::Unite, &o)?;
        assert_eq!(out.result, BooleanResult::Success);
        assert_eq!(out.bodies.len(), 1);
        assert!(out.bodies[0].check()?.is_empty(), "m={m} s={s}: body check");
        Ok(out.bodies[0].faces()?.len())
    };
    // (A ∪ B) with the A|B seam at x=5 left unmerged.
    let seamed = || -> Result<Body, Box<dyn Error>> {
        let p = parasolid::boolean::boolean(
            block(0.0, 10.0)?,
            vec![block(10.0, 10.0)?],
            BooleanOp::Unite,
            &BooleanOptions::new(),
        )?;
        Ok(p.into_iter().next().unwrap())
    };
    for (m, s, abut, overlap, chain, on_seam) in [
        (false, false, 10, 14, 14, 14),
        (true, false, 6, 6, 10, 6),
        (false, true, 10, 14, 14, 14),
        (true, true, 6, 6, 10, 10),
    ] {
        let got = (
            unite(block(0.0, 10.0)?, block(10.0, 10.0)?, m, s)?,
            unite(block(0.0, 10.0)?, block(5.0, 10.0)?, m, s)?,
            unite(seamed()?, block(20.0, 10.0)?, m, s)?,
            unite(seamed()?, block(15.0, 20.0)?, m, s)?,
        );
        assert_eq!(
            got,
            (abut, overlap, chain, on_seam),
            "merge_imprinted={m} selective_merge={s}"
        );
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// BooleanOptions matched_region @24: PK_boolean_match_o_t is version 5
// -----------------------------------------------------------------------------

/// parasolid-re item n. The binding had the 48-byte match layout right (the
/// journal `PKU_journal_boolean_match_o` @0x18049b3b0 reads exactly it), but
/// it is **version 5**: a version-1 struct of this shape returns
/// `PK_ERROR_system_error`, and with `auto_match` set versions 1..=4 fault.
/// Measured live (2026-10-16, DLL c900fa3f…63073) on 10 mm blocks whose
/// facing x faces are `gap` apart:
/// - gap 5e-4: no options → `no_clash`, two bodies; a face/face overlap
///   region with tolerance 1e-3 → one body, F=10, and its tolerant edges
///   carry exactly 1e-3; tolerance 1e-4 → `no_clash`;
/// - gap 2e-5: auto style alone → `no_clash`; auto with `auto_match_tol`
///   1.5e-5 → `no_clash`; 2.5e-5 → one body, F=10.
pub fn boolean_options_matched_region() -> R {
    assert_eq!(std::mem::size_of::<PK_boolean_match_o_t>(), 48);
    assert_eq!(PK_boolean_match_o_t::default().o_t_version, 5);
    let _s = Session::start(cfg())?;
    let pair = |gap: f64| -> Result<(Body, Body, i32, i32), Box<dyn Error>> {
        let a = Body::create_solid_block(10.0, 10.0, 10.0)?;
        let b = Body::create_solid_block(10.0, 10.0, 10.0)?;
        translate(&b, 10.0 + gap, 0.0, 0.0)?;
        let face_at = |body: &Body, x: f64| -> Result<i32, Box<dyn Error>> {
            for f in body.faces()? {
                if let Ok(p) = f.surf()?.ask_plane() {
                    if (p.basis.origin.x - x).abs() < 1e-9 && p.basis.axis.x.abs() > 0.5 {
                        return Ok(f.tag());
                    }
                }
            }
            Err(format!("no x={x} face").into())
        };
        let (fa, fb) = (face_at(&a, 5.0)?, face_at(&b, 5.0 + gap)?);
        Ok((a, b, fa, fb))
    };
    // (result, n_bodies, faces of the first body, largest edge precision)
    type Row = (BooleanResult, usize, usize, f64);
    let run = |gap: f64, o: BooleanOptions| -> Result<Row, Box<dyn Error>> {
        let (a, b, _, _) = pair(gap)?;
        summarise(boolean_detailed(a, vec![b], BooleanOp::Unite, &o)?)
    };
    fn summarise(out: BooleanOutcome) -> Result<Row, Box<dyn Error>> {
        let mut maxp = 0.0f64;
        for b in &out.bodies {
            assert!(b.check()?.is_empty(), "body check");
            for e in b.edges()? {
                if let Ok(p) = e.precision() {
                    maxp = maxp.max(p);
                }
            }
        }
        let f = match out.bodies.first() {
            Some(b) => b.faces()?.len(),
            None => 0,
        };
        Ok((out.result, out.bodies.len(), f, maxp))
    }
    let region = |gap: f64, tol: f64| -> Result<Row, Box<dyn Error>> {
        let (a, b, fa, fb) = pair(gap)?;
        let o = BooleanOptions::new().match_region(MatchedRegion {
            target: fa,
            tool: fb,
            match_type: MatchType::Overlap,
            tolerance: tol,
        });
        summarise(boolean_detailed(a, vec![b], BooleanOp::Unite, &o)?)
    };
    let apart = |r: Row| r.0 == BooleanResult::NoClash && r.1 == 2;
    let united = |r: Row| r.0 == BooleanResult::Success && (r.1, r.2) == (1, 10);

    let r = run(5e-4, BooleanOptions::new())?;
    assert!(apart(r), "gap 5e-4, no options: {r:?}");
    let r = region(5e-4, 1e-3)?;
    assert!(united(r), "gap 5e-4, region 1e-3: {r:?}");
    assert_eq!(r.3, 1e-3, "tolerant edges carry the region tolerance");
    let r = region(5e-4, 1e-4)?;
    assert!(apart(r), "gap 5e-4, region 1e-4: {r:?}");

    let r = run(2e-5, BooleanOptions::new().auto_match(None))?;
    assert!(apart(r), "gap 2e-5, auto: {r:?}");
    let r = run(2e-5, BooleanOptions::new().auto_match(Some(1.5e-5)))?;
    assert!(apart(r), "gap 2e-5, auto 1.5e-5: {r:?}");
    let r = run(2e-5, BooleanOptions::new().auto_match(Some(2.5e-5)))?;
    assert!(united(r), "gap 2e-5, auto 2.5e-5: {r:?}");
    Ok(())
}

// -----------------------------------------------------------------------------
// Defect — PK_TOPOL_facet_2 options and tables (parasolid-re B-P3-1)
// -----------------------------------------------------------------------------
//
// Evidence: parasolid-re `port/P3-faceting/PACKET.md` §1/§2 and C20, fixtures
// `port/P3-faceting/fixtures/p3-*.json`. Before the fix the binding stamped
// option version 5 (no table flags in its prefix), sized the structs 384/72 B,
// read a fictional by-value table struct and returned no points.

fn facet_default_tolerances(m: &Mesh) {
    let t = &m.tolerances;
    assert_eq!(t.curve_chord_ang, Some(FACET_DEFAULT_ANGLE), "{t:?}");
    assert_eq!(t.surface_plane_ang, Some(FACET_DEFAULT_ANGLE), "{t:?}");
    assert_eq!(
        (
            t.curve_chord_tol,
            t.curve_chord_max,
            t.surface_plane_tol,
            t.min_facet_width,
            t.max_facet_width
        ),
        (None, None, None, None, None),
        "{t:?}"
    );
    assert_eq!(t.facet_match, FacetMatch::Topology);
}

/// A closed, consistently oriented triangle mesh: every directed side appears
/// once and its reverse once.
fn facet_assert_closed(m: &Mesh, what: &str) {
    let mut sides = std::collections::HashMap::<(u32, u32), usize>::new();
    for t in &m.triangles {
        for i in 0..3 {
            *sides.entry((t[i], t[(i + 1) % 3])).or_default() += 1;
        }
    }
    for (&(a, b), &n) in &sides {
        assert_eq!(n, 1, "{what}: side {a}->{b} used {n} times");
        assert_eq!(
            sides.get(&(b, a)),
            Some(&1),
            "{what}: side {a}->{b} has no reverse"
        );
    }
}

/// Every boundary fin's two points are on its edge's polyline, and with
/// topology matching each polyline segment is covered by one fin per side.
fn facet_assert_fins_on_polylines(m: &Mesh, what: &str) {
    assert!(!m.boundary_fins.is_empty(), "{what}: no boundary fins");
    let mut cover = std::collections::HashMap::<(i32, u32, u32), usize>::new();
    for b in &m.boundary_fins {
        let pl = m
            .edge_polyline(b.edge)
            .unwrap_or_else(|| panic!("{what}: edge {} has no polyline", b.edge.tag()));
        for p in b.points {
            assert!(
                pl.points.contains(&p),
                "{what}: fin point {p} of edge {} not on {:?}",
                b.edge.tag(),
                pl.points
            );
        }
        let [a, c] = b.points;
        *cover.entry((b.edge.tag(), a.min(c), a.max(c))).or_default() += 1;
    }
    for pl in &m.edges {
        let n = pl.points.len();
        let segs = if pl.closed { n } else { n - 1 };
        for i in 0..segs {
            let (a, c) = (pl.points[i], pl.points[(i + 1) % n]);
            let k = (pl.edge.tag(), a.min(c), a.max(c));
            assert_eq!(
                cover.get(&k),
                Some(&2),
                "{what}: edge {} segment {a}-{c}",
                pl.edge.tag()
            );
        }
    }
}

/// Layouts of the v26 option structs and of the result/table structs, and the
/// default tokens the kernel's NULL-options migration writes (FUN_180447e00, as
/// in the parasolid-re probe that ran 179 live cases). The old structs were
/// 384 B / 72 B with a by-value table struct in the result.
pub fn facet_v26_layout() -> R {
    use std::mem::{offset_of, size_of};
    assert_eq!(size_of::<PK_TOPOL_facet_mesh_2_o_t>(), 0x198);
    assert_eq!(size_of::<PK_TOPOL_facet_choice_2_o_t>(), 0x50);
    assert_eq!(size_of::<PK_TOPOL_facet_2_o_t>(), 0x1e8);
    assert_eq!(size_of::<PK_TOPOL_facet_2_r_t>(), 24);
    assert_eq!(offset_of!(PK_TOPOL_facet_2_r_t, tables), 16);
    assert_eq!(size_of::<PK_TOPOL_facet_table_t>(), 16);
    assert_eq!(
        (
            offset_of!(PK_TOPOL_fctab_data_t, length),
            size_of::<PK_TOPOL_fctab_data_t>()
        ),
        (8, 16)
    );
    type M = PK_TOPOL_facet_mesh_2_o_t;
    assert_eq!(
        [
            offset_of!(M, is_curve_chord_tol),
            offset_of!(M, curve_chord_tol),
            offset_of!(M, is_curve_chord_ang),
            offset_of!(M, curve_chord_ang),
            offset_of!(M, is_surface_plane_ang),
            offset_of!(M, surface_plane_ang),
            offset_of!(M, is_max_facet_width),
            offset_of!(M, max_facet_width),
        ],
        [0x60, 0x68, 0x80, 0x88, 0xa0, 0xa8, 0x50, 0x58]
    );
    type C = PK_TOPOL_facet_choice_2_o_t;
    assert_eq!(
        [
            offset_of!(C, facet_fin),
            offset_of!(C, fin_data),
            offset_of!(C, data_point_idx),
            offset_of!(C, point_vec),
            offset_of!(C, facet_face),
            offset_of!(C, fin_edge),
            offset_of!(C, error_object),
        ],
        [0x30, 0x34, 0x35, 0x3a, 0x40, 0x44, 0x47]
    );

    let o = PK_TOPOL_facet_2_o_t::default();
    let base = &o as *const PK_TOPOL_facet_2_o_t as *const u8;
    let at = |off: usize| unsafe { (base.add(off) as *const i32).read_unaligned() };
    #[rustfmt::skip]
    let tokens = [
        (0x000, 26), (0x004, 0x5016), (0x008, 0x502a), (0x00c, 0x503c), (0x020, 0x5050), (0x040, 3),
        (0x118, 0x565f), (0x128, 0x5673), (0x12c, 0x567c), (0x130, 0x5820), (0x134, 0x6400),
        (0x138, 0x67c0), (0x13c, 0x67f2), (0x140, 0x582a), (0x144, 0x5834), (0x148, 0x5910),
        (0x14c, 0x5a0a), (0x154, 0x5b40), (0x158, 0x6338), (0x170, 0x6950), (0x180, 0x6b44),
        (0x194, 0x69e6),
        (0x198, 26), (0x19c, 10000), (0x1a0, 0x57e4), (0x1a4, 0x57ee), (0x1a8, 0x5e2e), (0x1c4, 0x5ffa),
    ];
    for (off, want) in tokens {
        assert_eq!(at(off), want, "default i32 at option byte {off:#x}");
    }

    // Live: the kernel accepts the defaults at 26 and knows no later version.
    let _s = Session::start(cfg())?;
    let block = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let rc = |v: i32| {
        let mut o = PK_TOPOL_facet_2_o_t::default();
        o.control.o_t_version = v;
        o.choice.o_t_version = v;
        o.choice.point_vec = PK_LOGICAL_true;
        let mut tag = block.tag();
        let mut r: PK_TOPOL_facet_2_r_t = unsafe { std::mem::zeroed() };
        let code = unsafe { PK_TOPOL_facet_2(1, &mut tag, std::ptr::null_mut(), &mut o, &mut r) };
        let n_tables = r.n_tables;
        if !r.tables.is_null() {
            unsafe { PK_TOPOL_facet_2_r_f(&mut r) };
        }
        (code, n_tables)
    };
    assert_eq!(
        rc(26),
        (0, 1),
        "v26 with defaults returns the one requested table"
    );
    let (c27, _) = rc(27);
    println!("\n    o_t_version 27 -> {c27}");
    assert_eq!(c27, 5022, "version 27 is PK_ERROR_o_t_version_unknown");
    Ok(())
}

/// A 10×10×10 block: 12 triangles over its 8 corners, two per face, closed and
/// outward (signed volume 1000), and one 2-point polyline per edge at the
/// edge's vertices. The old binding returned an empty point table.
pub fn facet_block_tables() -> R {
    let _s = Session::start(cfg())?;
    let block = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let m = block.facet()?;
    assert_eq!(
        (m.n_facets, m.n_fins, m.triangles.len(), m.points.len()),
        (12, 36, 12, 8)
    );
    for p in &m.points {
        assert!(
            p.x.abs() == 5.0 && p.y.abs() == 5.0 && (p.z == 0.0 || p.z == 10.0),
            "corner {p:?}"
        );
    }
    for t in &m.triangles {
        assert!(
            t.iter().all(|&i| (i as usize) < m.points.len())
                && t[0] != t[1]
                && t[1] != t[2]
                && t[0] != t[2]
        );
    }
    let faces = block.faces()?;
    assert_eq!(m.facet_faces.len(), 12);
    for f in &faces {
        assert_eq!(
            m.facet_faces.iter().filter(|&&g| g == *f).count(),
            2,
            "face {} has 2 triangles",
            f.tag()
        );
    }
    let vol: f64 = m
        .triangles
        .iter()
        .map(|t| {
            let [a, b, c] = t.map(|i| m.points[i as usize]);
            (a.x * (b.y * c.z - b.z * c.y) - a.y * (b.x * c.z - b.z * c.x)
                + a.z * (b.x * c.y - b.y * c.x))
                / 6.0
        })
        .sum();
    assert!((vol - 1000.0).abs() < 1e-9, "signed volume {vol}");
    facet_assert_closed(&m, "block");

    let edges = block.edges()?;
    assert_eq!(m.edges.len(), 12);
    for e in &edges {
        let pl = m.edge_polyline(*e).ok_or("edge without polyline")?;
        assert!(
            !pl.closed && pl.points.len() == 2,
            "edge {}: {pl:?}",
            e.tag()
        );
        let (v0, v1) = e.vertices()?;
        let mut want = [v0.point()?, v1.point()?].map(|p| [p.x, p.y, p.z]).to_vec();
        let mut got: Vec<[f64; 3]> = pl
            .points
            .iter()
            .map(|&i| m.points[i as usize])
            .map(|p| [p.x, p.y, p.z])
            .collect();
        want.sort_by(|a, b| a.partial_cmp(b).unwrap());
        got.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(got, want, "edge {} polyline at its vertices", e.tag());
    }
    assert!(m.faults.is_empty());
    facet_default_tolerances(&m);
    Ok(())
}

/// A default cylinder gets 16 chords per circle at any radius (packet §2
/// "Defaults", fixture p3-def-cyl: 64 facets, 34 points at r = 10) and records
/// the 0.4 rad defaults.
pub fn facet_cylinder_default_16_chords() -> R {
    let _s = Session::start(cfg())?;
    for r in [0.01, 10.0, 100.0] {
        let cyl = Body::create_solid_cylinder(r, 20.0)?;
        let m = cyl.facet()?;
        assert_eq!(m.edges.len(), 2, "r {r}");
        for pl in &m.edges {
            assert!(pl.closed && pl.points.len() == 16, "r {r}: {pl:?}");
            for &i in &pl.points {
                let p = m.points[i as usize];
                let d = (p.x.hypot(p.y) - r).abs();
                assert!(
                    d < 1e-12 * r.max(1.0) * 100.0,
                    "r {r}: point {p:?} off the circle by {d}"
                );
            }
        }
        if r == 10.0 {
            assert_eq!((m.triangles.len(), m.points.len()), (64, 34));
        }
        facet_assert_closed(&m, "cylinder");
        facet_default_tolerances(&m);
    }
    Ok(())
}

/// The chord-angle band 2π/8: on the plane cap faceted alone 0.7854 gives 8
/// chords and 0.785 gives 9; on the whole body (sang π/2, 4 columns) the
/// cylinder drops the thread at 0.7854 ≥ sang/2 (4 points) and merges it at
/// 0.785 (12). Fixtures p3-e1-cang0.7854 / p3-e1-cang0.785. A curve chord tol
/// alone switches the 0.4 angle default off: ctol 5 on r = 10 gives 4 chords.
pub fn facet_chord_angle_band() -> R {
    use std::f64::consts::FRAC_PI_2;
    let _s = Session::start(cfg())?;
    let cyl = Body::create_solid_cylinder(10.0, 20.0)?;
    let cap = cyl
        .faces()?
        .into_iter()
        .find(|f| f.surf().and_then(|s| s.ask_plane()).is_ok())
        .ok_or("no planar cap")?;
    let ring_points = |m: &Mesh| -> Vec<usize> { m.edges.iter().map(|p| p.points.len()).collect() };
    for (cang, alone, whole) in [(0.7854, 8, 4), (0.785, 9, 12)] {
        let o = FacetOptions::new()
            .curve_chord_ang(cang)
            .surface_plane_ang(FRAC_PI_2);
        let m = cap.facet_with(&o)?;
        assert_eq!(ring_points(&m), vec![alone], "cap alone, cang {cang}");
        assert!(m.facet_faces.iter().all(|f| *f == cap));
        let t = m.tolerances;
        assert_eq!(
            (t.curve_chord_ang, t.surface_plane_ang, t.curve_chord_tol),
            (Some(cang), Some(FRAC_PI_2), None)
        );
        let m = cyl.facet_with(&o)?;
        assert_eq!(
            ring_points(&m),
            vec![whole, whole],
            "whole body, cang {cang}"
        );
        facet_assert_closed(&m, "cylinder band");
    }
    let m = cap.facet_with(&FacetOptions::new().curve_chord_tol(5.0))?;
    assert_eq!(ring_points(&m), vec![4], "ctol 5 alone: quarter chords");
    let t = m.tolerances;
    assert_eq!(
        (t.curve_chord_tol, t.curve_chord_ang, t.surface_plane_ang),
        (Some(5.0), None, Some(FACET_DEFAULT_ANGLE))
    );
    Ok(())
}

/// Every boundary fin's points lie on its edge's polyline, and each polyline
/// segment is shared by both faces (topology matching), on the block, the
/// default and the banded cylinder, and a plate with a round and a square hole
/// (inner loops on its planar faces; fixture p3-def-plate).
pub fn facet_boundary_fins_on_edge_polylines() -> R {
    let _s = Session::start(cfg())?;
    let block = Body::create_solid_block(10.0, 10.0, 10.0)?;
    facet_assert_fins_on_polylines(&block.facet()?, "block");
    let cyl = Body::create_solid_cylinder(10.0, 20.0)?;
    facet_assert_fins_on_polylines(&cyl.facet()?, "cylinder");
    let o = FacetOptions::new()
        .curve_chord_ang(0.785)
        .surface_plane_ang(std::f64::consts::FRAC_PI_2);
    facet_assert_fins_on_polylines(&cyl.facet_with(&o)?, "cylinder cang 0.785");

    let plate = Body::create_solid_block(40.0, 30.0, 5.0)?;
    let hole = Body::create_solid_cylinder(5.0, 7.0)?;
    translate(&hole, -10.0, 0.0, -1.0)?;
    let prism = Body::create_solid_block(8.0, 8.0, 7.0)?;
    prism.transform(&Transform::rotation(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, 1.0),
        30f64.to_radians(),
    )?)?;
    translate(&prism, 10.0, 0.0, -1.0)?;
    let mut out = plate.subtract(vec![hole])?;
    assert_eq!(out.len(), 1);
    let mut out = out.remove(0).subtract(vec![prism])?;
    assert_eq!(out.len(), 1);
    let plate = out.remove(0);
    let m = plate.facet()?;
    assert_eq!(
        m.edges.len(),
        plate.edges()?.len(),
        "every plate edge has a polyline"
    );
    facet_assert_fins_on_polylines(&m, "plate");
    facet_assert_closed(&m, "plate");
    Ok(())
}
