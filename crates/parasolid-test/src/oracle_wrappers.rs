//! Runtime tests for the CADabra oracle wrappers added 2026-10-16 (Phase 2 of
//! the parasolid-re ABI audit): topology construction and attachment, tolerant
//! edges, SP-curves, nominal curves, compound bodies, journalling, the
//! Boolean-stage oracles (face sets, face Boolean, section with sheet), the
//! v11 `PK_BODY_boolean_o_t`, and XT transmit/receive at the current option
//! versions. Fixtures mirror parasolid-re `runner/src/construct.rs`,
//! `pkcheck.rs` and `stages*.rs`.

use parasolid::construct::{AttachCurvesOptions, MakeCompoundOptions, MakeSpcurvesOptions, SetPrecisionOptions};
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

fn code<T>(r: PsResult<T>) -> Option<i32> {
    match r {
        Err(e) => e.details().map(|d| d.code),
        Ok(_) => None,
    }
}

fn fev(b: &Body) -> Result<(usize, usize, usize), Box<dyn Error>> {
    Ok((b.faces()?.len(), b.edges()?.len(), b.vertices()?.len()))
}

fn translate(body: &Body, dx: f64, dy: f64, dz: f64) -> R {
    body.transform(&Transform::translation(dx, dy, dz)?)?;
    Ok(())
}

fn ent_class(e: Entity) -> Result<i32, Box<dyn Error>> {
    Ok(e.class()?.to_raw())
}

/// The square sheet of parasolid-re `square_sheet` (side `s`, z = 0): classes
/// body, region, shell, face, loop, then four edges each followed by its new
/// end vertex. Edge k runs vertex k → vertex k+1.
struct Sheet {
    body: Body,
    face: Face,
    surf: Surf,
    edges: Vec<Edge>,
    verts: Vec<Vertex>,
    topo: CreatedTopology,
}

fn square_topology() -> Result<CreatedTopology, Box<dyn Error>> {
    use PkClass::*;
    use TopolSense::{Negative as N, None as O, Positive as P};
    let classes = [Body, Region, Shell, Face, Loop, Edge, Vertex, Vertex, Edge, Vertex, Edge, Vertex, Edge];
    let rel = [
        (0, 1, N), (1, 2, O), (2, 3, O), (3, 4, O),
        (5, 6, O), (5, 7, O), (4, 5, P),
        (8, 7, O), (8, 9, O), (4, 8, P),
        (10, 9, O), (10, 11, O), (4, 10, P),
        (12, 11, O), (12, 6, O), (4, 12, P),
    ];
    Ok(create_topology(&classes, &rel, Some(BodyType::Sheet))?)
}

/// Build the sheet and attach the plane, the four points and lines on the
/// edges listed in `curved`.
fn square_sheet(s: f64, curved: &[usize]) -> Result<Sheet, Box<dyn Error>> {
    let topo = square_topology()?;
    let t = &topo.topols;
    let edges: Vec<Edge> = [t[5], t[8], t[10], t[12]]
        .iter()
        .map(|&x| find_edge(&topo.body, x))
        .collect::<Result<_, _>>()?;
    let verts: Vec<Vertex> = topo
        .body
        .vertices()?
        .into_iter()
        .filter(|v| [t[6], t[7], t[9], t[11]].contains(&v.tag()))
        .collect();
    let order = [t[6], t[7], t[9], t[11]];
    let mut verts_sorted = Vec::new();
    for o in order {
        verts_sorted.push(verts.iter().find(|v| v.tag() == o).copied().ok_or("vertex")?);
    }
    let face = topo.body.faces()?.into_iter().find(|f| f.tag() == t[3]).ok_or("face")?;
    let surf = Surf::plane(z_basis(Vec3::zero()))?;
    construct::attach_surfs(&[face], std::slice::from_ref(&surf), &[true])?;
    let pos = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(s, 0.0, 0.0),
        Vec3::new(s, s, 0.0),
        Vec3::new(0.0, s, 0.0),
    ];
    let pts: Vec<Point> = pos.iter().map(|&p| Point::create(p)).collect::<PsResult<_>>()?;
    construct::attach_points(&verts_sorted, &pts)?;
    let dirs = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    ];
    let ce: Vec<Edge> = curved.iter().map(|&k| edges[k]).collect();
    let cc: Vec<Curve> = curved.iter().map(|&k| Curve::line(pos[k], dirs[k])).collect::<PsResult<_>>()?;
    if !ce.is_empty() {
        construct::attach_curves(&ce, &cc, &AttachCurvesOptions::default())?;
    }
    Ok(Sheet { body: topo.body, face, surf, edges, verts: verts_sorted, topo })
}

fn find_edge(b: &Body, tag: i32) -> Result<Edge, Box<dyn Error>> {
    Ok(b.edges()?.into_iter().find(|e| e.tag() == tag).ok_or("edge tag not in body")?)
}

/// Square sheet with lines on edges 1 and 3; edges 0 and 2 made tolerant
/// (1e-5) with `PK_EDGE_set_precision_2`, so they have no curve and no fin
/// curves (parasolid-re `tolerant_sheet`).
fn tolerant_sheet(s: f64) -> Result<Sheet, Box<dyn Error>> {
    let sh = square_sheet(s, &[1, 3])?;
    for k in [0, 2] {
        sh.edges[k].set_precision_2(1e-5, &SetPrecisionOptions::default())?;
    }
    Ok(sh)
}

// -----------------------------------------------------------------------------

/// `PK_BODY_create_topology_2` + `PK_FACE_attach_surfs` +
/// `PK_VERTEX_attach_points` + `PK_EDGE_attach_curves_2` build a valid 10×10
/// sheet. Before the fix the declaration ended `(…, *body, **entities)` with
/// 1-byte senses, so the kernel wrote a 40-byte result over a 4-byte slot.
pub fn create_topology_square_sheet() -> R {
    let _s = Session::start(cfg())?;
    let sh = square_sheet(10.0, &[0, 1, 2, 3])?;
    assert_eq!(sh.topo.topols.len(), 13);
    assert_eq!(sh.topo.topols[0], sh.body.tag());
    assert!(sh.topo.is_ok(), "create faults {:?}", sh.topo.faults);
    assert_eq!(sh.topo.faults.len(), 1);
    assert_eq!(sh.topo.faults[0].state, PK_BODY_state_ok_c);
    assert_eq!(sh.body.body_type()?, BodyType::Sheet);
    assert_eq!(fev(&sh.body)?, (1, 4, 4));
    assert!(sh.body.check()?.is_empty(), "{:?}", sh.body.check()?);
    let area = sh.body.mass_props()?.amount;
    assert!((area - 100.0).abs() < 1e-9, "sheet area {area}");
    // Re-attaching geometry is refused with geom_not_needed (64).
    assert_eq!(
        code(construct::attach_surfs(&[sh.face], std::slice::from_ref(&sh.surf), &[true])),
        Some(PK_ERROR_geom_not_needed)
    );
    let p = Point::create(Vec3::zero())?;
    assert_eq!(code(construct::attach_points(&sh.verts[..1], &[p])), Some(PK_ERROR_geom_not_needed));
    Ok(())
}

/// A sheet whose face has no surface: `PK_BODY_check` reports
/// `PK_TOPOL_state_no_geom_c` on that face (40-byte fault decode, P9
/// `PASSS_all`); the option struct is version 1 → general when body_type is
/// omitted.
pub fn create_topology_faults_and_body_type() -> R {
    let _s = Session::start(cfg())?;
    let topo = square_topology()?;
    let face = topo.topols[3];
    let faults = topo.body.check()?;
    assert!(
        faults
            .iter()
            .any(|f| f.state == PK_TOPOL_state_no_geom_c && f.entity.map(|e| e.tag()) == Some(face)),
        "no_geom on the face: {faults:?}"
    );
    // A face with no loop is refused at creation.
    use PkClass::*;
    let r = create_topology(&[Body, Region, Shell, Face], &[(0, 1, TopolSense::Negative), (1, 2, TopolSense::None), (2, 3, TopolSense::None)], Some(BodyType::Sheet));
    match r {
        Err(e) => panic!("expected create faults, got error {e}"),
        Ok(t) => assert!(!t.is_ok(), "a face without loops is a create fault: {:?}", t.faults),
    }
    Ok(())
}

/// Tolerant edges with fin SP-curves: `PK_EDGE_set_precision_2`,
/// `PK_SPCURVE_create`/`_ask`, `PK_CURVE_make_spcurves_2`,
/// `PK_FIN_attach_curves`, `PK_FIN_ask_curve` (parasolid-re `abi-fin-attach`).
pub fn tolerant_edges_and_fin_curves() -> R {
    let _s = Session::start(cfg())?;
    let s = 10.0;
    let sh = tolerant_sheet(s)?;
    for k in [0, 2] {
        let p = sh.edges[k].precision()?;
        assert!((p - 1e-5).abs() < 1e-18, "edge {k} precision {p}");
    }
    let f0 = sh.edges[0].fins()?;
    let f2 = sh.edges[2].fins()?;
    assert_eq!((f0.len(), f2.len()), (1, 1));
    // Edge 0: SP-curve from a 2-D Bezier (0,0)->(s,0) on the face plane.
    let b2 = Curve::bcurve_2d(1, &[(0.0, 0.0), (s, 0.0)], &[0.0, 1.0], &[2, 2])?;
    assert_eq!(ent_class(b2.entity())?, PK_CLASS_bcurve);
    let sp0 = Curve::spcurve(&sh.surf, &b2)?;
    assert_eq!(ent_class(sp0.entity())?, PK_CLASS_spcurve);
    let (su, bc) = sp0.ask_spcurve()?;
    assert_eq!((su.tag(), bc.tag()), (sh.surf.tag(), b2.tag()), "SPCURVE_ask round trip");
    let mid = sp0.eval(0.5)?;
    assert!((mid.x - 5.0).abs() < 1e-12 && mid.y.abs() < 1e-12 && mid.z.abs() < 1e-12, "{mid:?}");
    construct::attach_fin_curves(&f0, &[sp0], &[(0.0, 1.0)])?;
    assert_eq!(ent_class(f0[0].curve()?.entity())?, PK_CLASS_spcurve);
    // A second curve on the same fin is refused (64).
    let b2b = Curve::bcurve_2d(1, &[(0.0, 0.0), (s, 0.0)], &[0.0, 1.0], &[2, 2])?;
    let sp0b = Curve::spcurve(&sh.surf, &b2b)?;
    assert_eq!(
        code(construct::attach_fin_curves(&f0, &[sp0b], &[(0.0, 1.0)])),
        Some(PK_ERROR_geom_not_needed)
    );
    // The fin of an accurate edge is refused (62 geom_topol_mismatch).
    let f1 = sh.edges[1].fins()?;
    let b2c = Curve::bcurve_2d(1, &[(s, 0.0), (s, s)], &[0.0, 1.0], &[2, 2])?;
    let sp1 = Curve::spcurve(&sh.surf, &b2c)?;
    assert_eq!(
        code(construct::attach_fin_curves(&f1, &[sp1], &[(0.0, 1.0)])),
        Some(PK_ERROR_geom_topol_mismatch)
    );
    // Edge 2: SP-curve made from a 3-space line by PK_CURVE_make_spcurves_2.
    let l2 = Curve::line(Vec3::new(s, s, 0.0), Vec3::new(-1.0, 0.0, 0.0))?;
    let sps = l2.make_spcurves((0.0, s), &sh.surf, 1e-8, &MakeSpcurvesOptions::default())?;
    assert_eq!(sps.len(), 1);
    assert_eq!(ent_class(sps[0].entity())?, PK_CLASS_spcurve);
    let iv = sps[0].interval()?;
    construct::attach_fin_curves(&f2, &[sps[0]], &[iv])?;
    assert_eq!(ent_class(f2[0].curve()?.entity())?, PK_CLASS_spcurve);
    // The result is a valid body.
    assert!(sh.body.check()?.is_empty(), "{:?}", sh.body.check()?);
    for k in [0, 2] {
        assert!(sh.edges[k].check()?.is_empty(), "edge {k}: {:?}", sh.edges[k].check()?);
    }
    // Resetting a tolerant edge's precision reports a token.
    let r = sh.edges[0].reset_precision()?;
    assert!((PK_reset_prec_ok_c..=PK_reset_prec_failure_c).contains(&r), "reset token {r}");
    Ok(())
}

/// Nominal curves: `PK_BODY_set_curve_nmnl_state` (21200/21201, previously 0/1),
/// `PK_EDGE_attach_curve_nmnl`, `PK_EDGE_ask_curve_nmnl` (parasolid-re
/// `abi-nmnl`).
pub fn nominal_curve_round_trip() -> R {
    let _s = Session::start(cfg())?;
    let s = 10.0;
    let sh = tolerant_sheet(s)?;
    let f0 = sh.edges[0].fins()?;
    let f2 = sh.edges[2].fins()?;
    let sp0 = Curve::spcurve(&sh.surf, &Curve::bcurve_2d(1, &[(0.0, 0.0), (s, 0.0)], &[0.0, 1.0], &[2, 2])?)?;
    let sp2 = Curve::spcurve(&sh.surf, &Curve::bcurve_2d(1, &[(s, s), (0.0, s)], &[0.0, 1.0], &[2, 2])?)?;
    construct::attach_fin_curves(&f0, &[sp0], &[(0.0, 1.0)])?;
    construct::attach_fin_curves(&f2, &[sp2], &[(0.0, 1.0)])?;
    let l0 = Curve::line(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0))?;
    assert_eq!(code(sh.edges[0].attach_curve_nmnl(&l0, None)), Some(PK_ERROR_curve_nmnl_off));
    // The old 1 / 0 values are not tokens.
    assert!(unsafe { PK_BODY_set_curve_nmnl_state(sh.body.tag(), 1) } != 0, "1 is not a token");
    sh.body.set_curve_nmnl_state(true)?;
    sh.edges[0].attach_curve_nmnl(&l0, None)?;
    let n = sh.edges[0].curve_nmnl()?.ok_or("nominal curve")?;
    assert_eq!(ent_class(n.entity())?, PK_CLASS_line);
    let l0b = Curve::line(Vec3::zero(), Vec3::new(1.0, 0.0, 0.0))?;
    assert_eq!(code(sh.edges[0].attach_curve_nmnl(&l0b, None)), Some(PK_ERROR_geom_not_needed));
    // Edge 2 (y = s, etol 1e-5): 6e-5 away is inconsistent; a short interval
    // is reported as inconsistent_geom (51), not bad_interval (5037).
    let far = Curve::line(Vec3::new(s, s + 6e-5, 0.0), Vec3::new(-1.0, 0.0, 0.0))?;
    assert_eq!(code(sh.edges[2].attach_curve_nmnl(&far, None)), Some(PK_ERROR_inconsistent_geom));
    let short = Curve::line(Vec3::new(s, s, 0.0), Vec3::new(-1.0, 0.0, 0.0))?;
    assert_eq!(
        code(sh.edges[2].attach_curve_nmnl(&short, Some((0.0, 0.5 * s)))),
        Some(PK_ERROR_inconsistent_geom)
    );
    let near = Curve::line(Vec3::new(s, s + 5e-6, 0.0), Vec3::new(-1.0, 0.0, 0.0))?;
    sh.edges[2].attach_curve_nmnl(&near, Some((0.0, s)))?;
    assert!(sh.edges[2].curve_nmnl()?.is_some());
    // An accurate edge reports its own curve as the nominal.
    let acc = sh.edges[1].curve_nmnl()?.ok_or("accurate edge curve")?;
    assert_eq!(acc.tag(), sh.edges[1].curve()?.tag());
    Ok(())
}

/// `PK_BODY_make_compound` (v3), `PK_BODY_ask_children`, `PK_BODY_ask_parent`
/// (parasolid-re `abi-compound`).
pub fn compound_bodies() -> R {
    let _s = Session::start(cfg())?;
    let a = Body::create_solid_block(1.0, 1.0, 1.0)?;
    let b = Body::create_solid_block(2.0, 2.0, 2.0)?;
    translate(&b, 5.0, 0.0, 0.0)?;
    let c = Body::make_compound(&[a, b], &MakeCompoundOptions::default())?;
    let mut t = 0;
    assert_eq!(unsafe { PK_BODY_ask_type(c.tag(), &mut t) }, 0);
    assert_eq!(t, PK_BODY_type_compound_c);
    let kids: Vec<i32> = c.children()?.iter().map(|k| k.tag()).collect();
    assert_eq!(kids.len(), 2);
    assert!(kids.contains(&a.tag()) && kids.contains(&b.tag()));
    assert_eq!(a.parent()?.map(|p| p.tag()), Some(c.tag()));
    assert!(c.parent()?.is_none());
    assert!(c.check()?.is_empty());
    assert_eq!(code(a.check()), Some(PK_ERROR_child_body), "checking a child is refused");
    let bad = MakeCompoundOptions { o_t_version: 4, ..Default::default() };
    let d = Body::create_solid_block(1.0, 1.0, 1.0)?;
    let e = Body::create_solid_block(1.0, 1.0, 1.0)?;
    translate(&e, 3.0, 0.0, 0.0)?;
    assert_eq!(code(Body::make_compound(&[d, e], &bad)), Some(PK_ERROR_o_t_version_unknown));
    Ok(())
}

/// `PK_SESSION_set_journalling` needs a journal file opened at session start.
pub fn journalling_switch() -> R {
    {
        let s = Session::start(cfg())?;
        assert!(!s.journalling()?);
        assert_eq!(code(s.set_journalling(true)), Some(PK_ERROR_journal_not_open));
        assert!(!s.journalling()?);
    }
    let dir = "journal_test_out";
    let _ = std::fs::create_dir_all(dir);
    let s = Session::start(
        cfg().frustrum(FrustrumConfig::new().base_dir(dir)).journal_file("switch"),
    )?;
    assert!(s.journalling()?);
    s.set_journalling(false)?;
    assert!(!s.journalling()?);
    s.set_journalling(true)?;
    assert!(s.journalling()?);
    Ok(())
}

// -----------------------------------------------------------------------------
// Boolean stages
// -----------------------------------------------------------------------------

/// parasolid-re `r1-blind-hole`: block 40×40×20, r=4 h=20 cylinder at (3,-2,10).
fn blind_hole() -> Result<(Body, Body), Box<dyn Error>> {
    let a = Body::create_solid_block(40.0, 40.0, 20.0)?;
    let b = Body::create_solid_cylinder(4.0, 20.0)?;
    translate(&b, 3.0, -2.0, 10.0)?;
    Ok((a, b))
}

/// `PK_TOPOL_find_box` on a face: `[xmin, ymin, zmin, xmax, ymax, zmax]`.
fn face_box(f: &Face) -> Result<[f64; 6], Box<dyn Error>> {
    let mut b = PK_BOX_t { coord: [0.0; 6] };
    let rc = unsafe { PK_TOPOL_find_box(f.tag(), &mut b) };
    if rc != 0 {
        return Err(format!("find_box rc {rc}").into());
    }
    Ok(b.coord)
}

fn area(f: &Face) -> Result<f64, Box<dyn Error>> {
    let t = f.tag();
    let (mut amount, mut mass, mut per) = (0.0, 0.0, 0.0);
    let (mut cg, mut mi) = ([0.0; 3], [0.0; 9]);
    let o = PK_TOPOL_eval_mass_props_o_t::default();
    let rc = unsafe {
        PK_TOPOL_eval_mass_props(1, &t, 0.99, &o, &mut amount, &mut mass, &mut cg, &mut mi, &mut per)
    };
    if rc != 0 {
        return Err(format!("face mass props rc {rc}").into());
    }
    Ok(amount)
}

/// `PK_BODY_identify_facesets` (v3) after a v5 imprint, subtract: the target
/// keeps everything but the imprinted disc, the tool keeps only what lies in
/// the block.
pub fn identify_facesets_blind_hole_subtract() -> R {
    let _s = Session::start(cfg())?;
    let (a, b) = blind_hole()?;
    let imp = a.imprint_body(&b)?;
    let fs = identify_facesets(&a, &b, &imp, BooleanOp::Subtract)?;
    let n = |v: &Vec<Vec<Face>>| v.iter().map(|s| s.len()).sum::<usize>();
    let (st, rt, so, ro) = (n(&fs.survive_target), n(&fs.reject_target), n(&fs.survive_tool), n(&fs.reject_tool));
    assert_eq!(st + rt, 7, "every target face classified: {st}+{rt}");
    assert_eq!(so + ro, 4, "every tool face classified: {so}+{ro}");
    assert_eq!(rt, 1, "the imprinted disc is rejected");
    let disc = fs.reject_target[0][0];
    let disc_area = area(&disc)?;
    assert!((disc_area - std::f64::consts::PI * 16.0).abs() < 1e-6, "disc area {disc_area}");
    // Tool faces that survive a subtract are inside the block (z <= 20).
    for set in &fs.survive_tool {
        for f in set {
            let bx = face_box(f)?;
            assert!(bx[5] <= 20.0 + 1e-9, "surviving tool face above the block: {bx:?}");
        }
    }
    assert_eq!(fs.bound_target_edges.len(), fs.bound_tool_edges.len());
    Ok(())
}

/// `PK_FACE_boolean_2` (v17, kernel defaults) with every face of both bodies
/// gives the same subtract result as `PK_BODY_boolean_2` (parasolid-re
/// three-route probe).
pub fn face_boolean_matches_body_boolean() -> R {
    let _s = Session::start(cfg())?;
    let (a, b) = blind_hole()?;
    let out = face_boolean(&a.faces()?, &b.faces()?, BooleanOp::Subtract)?;
    assert_eq!(out.result, BooleanResult::Success, "{out:?}");
    assert_eq!(out.bodies.len(), 1);
    let v_face = out.bodies[0].mass_props()?.amount;
    let fev_face = fev(&out.bodies[0])?;
    let (a2, b2) = blind_hole()?;
    let body = boolean::boolean(a2, vec![b2], BooleanOp::Subtract, &BooleanOptions::new())?;
    assert_eq!(fev_face, fev(&body[0])?, "same topology as the body Boolean");
    let want = 40.0 * 40.0 * 20.0 - std::f64::consts::PI * 16.0 * 10.0;
    assert!((v_face - want).abs() < 1e-6 * want, "volume {v_face} vs {want}");
    Ok(())
}

/// `PK_FACE_make_sheet_bodies` (v4) then `PK_BODY_section_with_sheet_2` (v5):
/// a horizontal sheet at z = 10 cuts the block into two halves.
pub fn section_with_sheet_halves_block() -> R {
    let _s = Session::start(cfg())?;
    let a = Body::create_solid_block(40.0, 40.0, 20.0)?;
    let top = a
        .faces()?
        .into_iter()
        .find(|f| face_box(f).map(|b| b[2] > 19.0).unwrap_or(false))
        .ok_or("top face")?;
    let sheets = Face::make_sheet_bodies(&[top])?;
    assert_eq!(sheets.len(), 1);
    assert_eq!(sheets[0].body_type()?, BodyType::Sheet);
    assert!((sheets[0].mass_props()?.amount - 1600.0).abs() < 1e-9);
    let cutter = Body::create_sheet_rectangle(100.0, 100.0, z_basis(Vec3::new(0.0, 0.0, 10.0)))?;
    let r = a.section_with_sheet(&cutter)?;
    assert_eq!(r.result, SectionResult::Success, "{r:?}");
    assert_eq!((r.front_bodies.len(), r.back_bodies.len()), (1, 1));
    for b in r.front_bodies.iter().chain(&r.back_bodies) {
        let v = b.mass_props()?.amount;
        assert!((v - 16000.0).abs() < 1e-6, "half volume {v}");
    }
    assert!(r.reports.is_empty());
    Ok(())
}

/// `PK_BODY_boolean_o_t` v11 (96 B): a non-zero `default_tol` works (with the
/// old 32-byte struct it was read as the `matched_region` pointer and crashed
/// the kernel), and the late fields are live — a non-logical in
/// `allow_disjoint` (@32) and a non-token in `nm_edge_repair` (@84) are
/// refused.
pub fn boolean_v11_options_are_live() -> R {
    let _s = Session::start(cfg())?;
    let run = |o: &PK_BODY_boolean_o_t| -> (i32, i32, Vec<i32>) {
        let (a, b) = blind_hole().unwrap();
        let tools = [b.tag()];
        let mut tracking: PK_TOPOL_track_r_t = unsafe { std::mem::zeroed() };
        let mut r: PK_boolean_r_t = unsafe { std::mem::zeroed() };
        let rc = unsafe { PK_BODY_boolean_2(a.tag(), 1, tools.as_ptr(), o, &mut tracking, &mut r) };
        let bodies = if r.bodies.is_null() { vec![] } else { unsafe { std::slice::from_raw_parts(r.bodies, r.n_bodies.max(0) as usize) }.to_vec() };
        let res = r.result;
        unsafe {
            PK_boolean_r_f(&mut r);
            PK_TOPOL_track_r_f(&mut tracking);
        }
        (rc, res, bodies)
    };
    let base = PK_BODY_boolean_o_t { function: PK_boolean_subtract_c, ..Default::default() };
    assert_eq!(base.o_t_version, 11);
    let o = PK_BODY_boolean_o_t { default_tol: 1e-6, ..base };
    let (rc, res, bodies) = run(&o);
    assert_eq!((rc, res), (0, PK_boolean_result_success_c));
    assert_eq!(bodies.len(), 1);
    let v = Body::from_tag_checked(bodies[0])?.mass_props()?.amount;
    let want = 40.0 * 40.0 * 20.0 - std::f64::consts::PI * 16.0 * 10.0;
    assert!((v - want).abs() < 1e-6 * want);
    assert_eq!(run(&PK_BODY_boolean_o_t { allow_disjoint: 0xAB, ..base }).0, PK_ERROR_not_a_logical);
    assert_eq!(run(&PK_BODY_boolean_o_t { nm_edge_repair: 0x7fff_0000, ..base }).0, PK_ERROR_field_of_wrong_type);
    // nm_edge_repair blend is accepted.
    assert_eq!(run(&PK_BODY_boolean_o_t { nm_edge_repair: PK_nm_edge_repair_blend_c, ..base }).0, 0);
    Ok(())
}

/// XT round trip with `PK_PART_transmit_o_t` v4 and `PK_PART_receive_o_t` v8
/// (previously 3 and 1: the later versions failed with 5014 because their enum
/// fields were zero). The next versions are refused with 5022.
pub fn xt_transmit_receive_latest_versions() -> R {
    let _s = Session::start(cfg())?;
    assert_eq!(PK_PART_transmit_o_t::default().o_t_version, 4);
    assert_eq!(PK_PART_receive_o_t::default().o_t_version, 8);
    let b = Body::create_solid_block(3.0, 4.0, 5.0)?;
    fileio::transmit_with_format(&[b], "abi_v4_v8", fileio::TransmitFormat::Text)?;
    let got = fileio::receive("abi_v4_v8")?;
    assert_eq!(got.len(), 1);
    assert!((got[0].mass_props()?.amount - 60.0).abs() < 1e-9);
    assert_eq!(fev(&got[0])?, (6, 12, 8));
    let key = std::ffi::CString::new("abi_v4_v8")?;
    let t = [b.tag()];
    let mut o = PK_PART_transmit_o_t { transmit_format: PK_transmit_format_text_c, ..Default::default() };
    o.o_t_version = 5;
    assert_eq!(unsafe { PK_PART_transmit(1, t.as_ptr(), key.as_ptr(), &o) }, PK_ERROR_o_t_version_unknown);
    let mut r = PK_PART_receive_o_t { transmit_format: PK_transmit_format_text_c, ..Default::default() };
    r.o_t_version = 9;
    let (mut n, mut p) = (0, std::ptr::null_mut());
    assert_eq!(unsafe { PK_PART_receive(key.as_ptr(), &r, &mut n, &mut p) }, PK_ERROR_o_t_version_unknown);
    Ok(())
}
