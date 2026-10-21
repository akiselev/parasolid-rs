//! Rendering and graphical-output (GO) tests (parasolid-re R-44 blockers
//! B-R44-1 and B-R44-2; evidence in parasolid-re
//! `artifacts/rs/20261021T031234Z-prs-render/notes.md`).
//!
//! Before the fix, `PK_TOPOL_render_line_o_t`, `PK_TOPOL_render_facet_o_t` and
//! `PK_GEOM_render_o_t` had no `o_t_version` and a wrong field order, and the
//! GO callbacks were declared with 2 arguments instead of 8, so none of these
//! calls could be made. Each test here would have failed to compile or run
//! against the old bindings.

use parasolid::go::{self, GoCall, GoRecord};
use parasolid::*;
use parasolid_sys::*;
use std::error::Error;

pub type R = Result<(), Box<dyn Error>>;

fn go_cfg() -> SessionConfig {
    SessionConfig::new()
        .check_arguments(true)
        .frustrum(FrustrumConfig::new().capture_go())
}

/// Read the i32 at byte `off` of a struct.
fn i32_at<T>(o: &T, off: usize) -> i32 {
    let base = o as *const T as *const u8;
    unsafe { (base.add(off) as *const i32).read_unaligned() }
}

fn f64_at<T>(o: &T, off: usize) -> f64 {
    let base = o as *const T as *const u8;
    unsafe { (base.add(off) as *const f64).read_unaligned() }
}

/// Write an i32 at byte `off` of a struct (for garbage-token probes).
fn put_i32<T>(o: &mut T, off: usize, v: i32) {
    let base = o as *mut T as *mut u8;
    unsafe { (base.add(off) as *mut i32).write_unaligned(v) }
}

fn render_line(bodies: &[&Body], o: &PK_TOPOL_render_line_o_t) -> (i32, Vec<GoRecord>) {
    let mut tags: Vec<i32> = bodies.iter().map(|b| b.tag()).collect();
    let mut o = *o;
    go::clear();
    let rc = unsafe {
        PK_TOPOL_render_line(
            tags.len() as i32,
            tags.as_mut_ptr(),
            std::ptr::null_mut(),
            PK_ENTITY_null,
            &mut o,
        )
    };
    (rc, go::take())
}

/// The kernel's last error record, as text (for failure messages).
fn last_error() -> String {
    let mut sf: PK_ERROR_sf_t = unsafe { std::mem::zeroed() };
    let mut was: PK_LOGICAL_t = PK_LOGICAL_false;
    unsafe { PK_ERROR_ask_last(&mut was, &mut sf) };
    let txt = |a: &[std::os::raw::c_char]| {
        let b: Vec<u8> = a
            .iter()
            .take_while(|&&c| c != 0)
            .map(|&c| c as u8)
            .collect();
        String::from_utf8_lossy(&b).into_owned()
    };
    format!(
        "{} {} {} arg {} {:?}[{}]",
        txt(&sf.function),
        sf.code,
        txt(&sf.code_token),
        sf.argument_number,
        txt(&sf.argument_name),
        sf.argument_index
    )
}

fn segments(recs: &[GoRecord]) -> Vec<&GoRecord> {
    recs.iter().filter(|r| r.call == GoCall::Segment).collect()
}

/// Compact histogram of a GO stream: count per (call, segtyp, line type).
fn summary(recs: &[GoRecord]) -> String {
    let mut h: std::collections::BTreeMap<(String, i32, i32, i32), usize> = Default::default();
    for r in recs {
        let lt = r.lntp.get(1).copied().unwrap_or(0);
        *h.entry((format!("{:?}", r.call), r.segtyp, lt, r.ngeom))
            .or_default() += 1;
    }
    h.iter()
        .map(|((c, t, l, n), k)| format!(" {k}x{c}({t},lt {l},ngeom {n})"))
        .collect()
}

/// A block turned so that no edge is seen end-on along z: rotate 30° about x,
/// then 40° about y.
fn turned_block() -> Result<Body, Box<dyn Error>> {
    let b = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let o = Vec3::new(0.0, 0.0, 0.0);
    let t = Transform::rotation(o, Vec3::new(1.0, 0.0, 0.0), 30f64.to_radians())?.then(
        &Transform::rotation(o, Vec3::new(0.0, 1.0, 0.0), 40f64.to_radians())?,
    )?;
    b.transform(&t)?;
    Ok(b)
}

// -----------------------------------------------------------------------------
// PK_TOPOL_render_line
// -----------------------------------------------------------------------------

/// Layout and version window of `PK_TOPOL_render_line_o_t`: the defaults sit
/// at the offsets FUN_180472d80 writes them to; version 25 is accepted, 26 is
/// 5022; a garbage token in `edge` (first field) or `region` (last checked
/// enum, byte 0x21c) gives 5014.
pub fn render_line_v25_layout() -> R {
    let o = PK_TOPOL_render_line_o_t::default();
    #[rustfmt::skip]
    let tokens = [
        (0x000, 25), (0x004, 0x4e85), (0x008, 0x4e98), (0x00c, 0x6694), (0x010, 0x4f24), (0x014, 0x4f4c),
        (0x018, 0x4fd8), (0x01c, 0x4fe2), (0x020, 0x4fec), (0x050, 0x4f60), (0x060, 0x4f74), (0x064, 0x5cc6),
        (0x068, 0x553c), (0x078, 0x4f9c), (0x07c, 0x4fb0), (0x080, 0x5690), (0x090, 0x63ce), (0x094, 0x4eac),
        (0x0d0, 0x4ec0), (0x108, 0x4ed4), (0x138, 0x4ee8), (0x148, 0x556f), (0x168, 0x4f10), (0x178, 0x4f13),
        (0x188, 0x4fc4), (0x18c, 0x5000), (0x218, 0x4ff6), (0x21c, 0x4f88), (0x238, 0x6950),
    ];
    for (off, want) in tokens {
        assert_eq!(
            i32_at(&o, off),
            want,
            "default i32 at render_line byte {off:#x}"
        );
    }
    for off in [0xf0, 0xf8, 0x100, 0x128, 0x130] {
        assert_eq!(f64_at(&o, off), -1.0e36, "unset double at byte {off:#x}");
    }

    let _s = Session::start(go_cfg())?;
    let block = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let rc = |f: &dyn Fn(&mut PK_TOPOL_render_line_o_t)| {
        let mut o = PK_TOPOL_render_line_o_t::default();
        f(&mut o);
        render_line(&[&block], &o).0
    };
    let r25 = rc(&|_| {});
    let r24 = rc(&|o| o.o_t_version = 24);
    let r26 = rc(&|o| o.o_t_version = 26);
    let edge_bad = rc(&|o| put_i32(o, 0x004, 12345));
    let region_bad = rc(&|o| put_i32(o, 0x21c, 12345));
    println!(
        "\n    v25 {r25}, v24 {r24}, v26 {r26}, edge=12345 {edge_bad}, region=12345 {region_bad}"
    );
    assert_eq!(r25, 0);
    assert_eq!(r24, 0, "version 24 is migrated, not refused");
    assert_eq!(r26, PK_ERROR_o_t_version_unknown);
    assert_eq!(edge_bad, PK_ERROR_field_of_wrong_type);
    assert_eq!(region_bad, PK_ERROR_field_of_wrong_type);
    Ok(())
}

/// A 10×10×10 block with default options: one GO segment per edge, each a
/// straight line (9 doubles: start, end, direction) between two block corners,
/// tagged with its edge.
pub fn render_line_block_edges() -> R {
    let _s = Session::start(go_cfg())?;
    let block = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let (rc, recs) = render_line(&[&block], &PK_TOPOL_render_line_o_t::default());
    println!("{}", summary(&recs));
    assert_eq!(rc, 0);
    let segs = segments(&recs);
    assert_eq!(segs.len(), 12, "one segment per edge");
    let mut edge_tags: Vec<i32> = block.edges()?.iter().map(|e| e.tag()).collect();
    let mut seg_tags: Vec<i32> = segs.iter().flat_map(|r| r.tags.clone()).collect();
    edge_tags.sort();
    seg_tags.sort();
    assert_eq!(seg_tags, edge_tags, "each segment carries its edge tag");
    let corner =
        |x: f64, y: f64, z: f64| x.abs() == 5.0 && y.abs() == 5.0 && (z == 0.0 || z == 10.0);
    for s in &segs {
        assert_eq!(s.geom.len(), 9, "straight line: start, end, direction");
        let g = &s.geom;
        assert!(
            corner(g[0], g[1], g[2]) && corner(g[3], g[4], g[5]),
            "ends at corners: {g:?}"
        );
        let d = [g[3] - g[0], g[4] - g[1], g[5] - g[2]];
        let len = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        assert!((len - 10.0).abs() < 1e-9);
        let dot = (d[0] * g[6] + d[1] * g[7] + d[2] * g[8]) / len;
        assert!(
            (dot - 1.0).abs() < 1e-9,
            "direction is the unit start->end vector"
        );
    }
    // All the same line type code (lntp[1]); same occurrence (lntp[0]).
    let l1: std::collections::BTreeSet<i32> = segs.iter().map(|s| s.lntp[1]).collect();
    assert_eq!(l1.len(), 1, "line type codes {l1:?}");
    Ok(())
}

/// Hidden-line removal (`visibility`, byte 0x14, with `silhouette` on): a
/// turned cube shows 9 edges and hides 3. `vis_hid` outputs the 9 visible
/// ones; `vis_inv` outputs all 12, the hidden ones with a different
/// visibility code.
pub fn render_line_hidden_line() -> R {
    let _s = Session::start(go_cfg())?;
    let block = turned_block()?;
    let run = |vis: i32| {
        let mut o = PK_TOPOL_render_line_o_t::default();
        o.visibility = vis;
        // Hidden-line output needs silhouettes on (see the 5014 below).
        o.silhouette = PK_render_silhouette_yes_c;
        let (rc, recs) = render_line(&[&block], &o);
        if rc != 0 {
            println!("\n    vis {vis}: {}", last_error());
        }
        (rc, recs)
    };
    let (rc0, none) = run(PK_render_vis_no_c);
    let (rc1, hid) = run(PK_render_vis_hid_c);
    let (rc2, inv) = run(PK_render_vis_inv_c);
    println!(
        "\n    vis_no{}\n    vis_hid{}\n    vis_inv{}",
        summary(&none),
        summary(&hid),
        summary(&inv)
    );
    assert_eq!((rc0, rc1, rc2), (0, 0, 0));
    // The PK-to-HID converter FUN_18047ed40 refuses any visibility other than
    // vis_no with silhouette_no: 5014 raised at 0x18047f6f2 (static-observed;
    // trace-confirmed, notes.md §5).
    let mut o = PK_TOPOL_render_line_o_t::default();
    o.visibility = PK_render_vis_hid_c;
    assert_eq!(render_line(&[&block], &o).0, PK_ERROR_field_of_wrong_type);
    assert_eq!(segments(&none).len(), 12);
    assert_eq!(segments(&hid).len(), 9, "hidden edges removed");
    assert!(segments(&inv).len() >= 12, "hidden edges kept and marked");
    let codes = |recs: &[GoRecord]| -> std::collections::BTreeSet<Vec<i32>> {
        segments(recs).iter().map(|s| s.lntp.clone()).collect()
    };
    assert!(
        codes(&inv).len() > codes(&hid).len(),
        "hidden segments carry another lntp"
    );
    Ok(())
}

/// Silhouettes (`silhouette`, byte 8): a cylinder seen side-on has two edge
/// circles; with silhouettes on, two silhouette lines are added.
pub fn render_line_silhouette() -> R {
    let _s = Session::start(go_cfg())?;
    let cyl = Body::create_solid_cylinder(5.0, 20.0)?;
    // Axis z; view along z sees it end-on, so turn it 90° about x.
    cyl.transform(&Transform::rotation(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        90f64.to_radians(),
    )?)?;
    let (rc0, plain) = render_line(&[&cyl], &PK_TOPOL_render_line_o_t::default());
    let mut o = PK_TOPOL_render_line_o_t::default();
    o.silhouette = PK_render_silhouette_yes_c;
    let (rc1, sil) = render_line(&[&cyl], &o);
    println!(
        "\n    plain{}\n    silhouette{}",
        summary(&plain),
        summary(&sil)
    );
    assert_eq!((rc0, rc1), (0, 0));
    let n_edges = cyl.edges()?.len();
    assert_eq!(segments(&plain).len(), n_edges);
    assert_eq!(segments(&sil).len(), n_edges + 2, "two silhouette lines");
    Ok(())
}

/// Planar hatching (`planar` 0x94, `planar_spacing` 0x98, `planar_axis` 0xa0):
/// planes normal to z every 2.5 hatch the four side faces of the block.
pub fn render_line_planar_hatch() -> R {
    let _s = Session::start(go_cfg())?;
    let block = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let mut o = PK_TOPOL_render_line_o_t::default();
    o.planar = PK_render_planar_free_c;
    o.planar_spacing = 2.5;
    o.planar_axis = PK_AXIS1_sf_t {
        location: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
    };
    let (rc, recs) = render_line(&[&block], &o);
    println!("{}", summary(&recs));
    assert_eq!(rc, 0);
    let segs = segments(&recs);
    let edges: Vec<i32> = block.edges()?.iter().map(|e| e.tag()).collect();
    let hatch: Vec<&&GoRecord> = segs
        .iter()
        .filter(|s| !s.tags.iter().any(|t| edges.contains(t)))
        .collect();
    println!("    {} segments, {} hatch", segs.len(), hatch.len());
    // Planes z = 2.5, 5, 7.5 cut each of the 4 side faces once.
    assert_eq!(hatch.len(), 12, "3 hatch lines on each side face");
    for h in &hatch {
        // Each hatch line lies in a plane z = k * 2.5.
        let z0 = h.geom[2];
        assert!((z0 / 2.5 - (z0 / 2.5).round()).abs() < 1e-9, "z {z0}");
        assert!((h.geom[5] - z0).abs() < 1e-9);
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// PK_TOPOL_render_facet
// -----------------------------------------------------------------------------

fn render_facet(body: &Body, o: &PK_TOPOL_render_facet_o_t) -> (i32, Vec<GoRecord>) {
    let mut tag = body.tag();
    let mut o = *o;
    go::clear();
    let rc =
        unsafe { PK_TOPOL_render_facet(1, &mut tag, std::ptr::null_mut(), PK_ENTITY_null, &mut o) };
    (rc, go::take())
}

/// `PK_TOPOL_render_facet` at version 20: defaults at the migration's offsets;
/// a block's facets arrive through GO with every vertex on a block corner;
/// version 21 is 5022; `go_normals` (byte 0x18c) adds normals to each facet.
pub fn render_facet_v20() -> R {
    let o = PK_TOPOL_render_facet_o_t::default();
    #[rustfmt::skip]
    let tokens = [
        (0x000, 20), (0x004, 0x5016), (0x008, 0x502a), (0x00c, 0x503c), (0x010, 0x5050), (0x020, 3),
        (0x0d0, 0x5064), (0x108, 0x565f), (0x118, 0x5673), (0x11c, 0x567c), (0x120, 0x5820),
        (0x124, 0x6400), (0x128, 0x67c0), (0x12c, 0x67f2), (0x130, 0x582a), (0x134, 0x5834),
        (0x138, 0x5910), (0x13c, 0x5a0a), (0x144, 0x6338), (0x160, 0x6950), (0x170, 0x6b44),
        (0x184, 0x69e6),
        (0x188, 20), (0x18c, 0x5078), (0x190, 0x508c), (0x194, 0x6644), (0x198, 0x50a0),
        (0x19c, 0x50b4), (0x1a0, 0), (0x1a4, 0x50c8), (0x1a8, 0x57e4), (0x1ac, 0x57ee),
    ];
    for (off, want) in tokens {
        assert_eq!(
            i32_at(&o, off),
            want,
            "default i32 at render_facet byte {off:#x}"
        );
    }

    let _s = Session::start(go_cfg())?;
    let block = Body::create_solid_block(10.0, 10.0, 10.0)?;
    let (rc, recs) = render_facet(&block, &o);
    println!("{}", summary(&recs));
    assert_eq!(rc, 0);
    let segs = segments(&recs);
    assert!(!segs.is_empty(), "facets sent through GO");
    for s in &segs {
        assert_eq!(s.lntp[1], 3007, "vertices-only facet");
        assert_eq!(s.geom.len(), 3 * s.ngeom as usize);
        assert!(s.geom.len() >= 9, "points {:?}", s.geom);
        for p in s.geom.chunks(3) {
            assert!(
                p[0].abs() == 5.0 && p[1].abs() == 5.0 && (p[2] == 0.0 || p[2] == 10.0),
                "vertex {p:?}"
            );
        }
    }

    // Brackets: the body bracket holds one bracket per face, each with that
    // face's two triangles (compared as a set: SMP may reorder inside one).
    let br = go::brackets(&recs);
    assert_eq!(br.len(), 1);
    assert_eq!(
        br[0].open.as_ref().map(|r| r.tags.clone()),
        Some(vec![block.tag()])
    );
    let mut face_tags: Vec<i32> = block.faces()?.iter().map(|f| f.tag()).collect();
    let mut bracket_faces: Vec<i32> = br[0]
        .children
        .iter()
        .map(|c| {
            let t = c.open.as_ref().unwrap().tags[0];
            assert_eq!(c.segments.len(), 2);
            assert!(c.segments.iter().all(|s| s.tags == vec![t]));
            t
        })
        .collect();
    face_tags.sort();
    bracket_faces.sort();
    assert_eq!(bracket_faces, face_tags);

    let mut v21 = o;
    v21.control.o_t_version = 21;
    let (rc21, _) = render_facet(&block, &v21);
    let mut n = o;
    n.go_option.go_normals = PK_facet_go_normals_yes_c;
    let (rcn, with_n) = render_facet(&block, &n);
    println!("    v21 {rc21}; with normals:{}", summary(&with_n));
    assert_eq!(rc21, PK_ERROR_o_t_version_unknown);
    assert_eq!(rcn, 0);
    let g0: usize = segs.iter().map(|s| s.geom.len()).sum();
    let g1: usize = segments(&with_n).iter().map(|s| s.geom.len()).sum();
    assert_eq!(g1, 2 * g0, "a normal per facet vertex doubles the geometry");
    // Layout of a 3008 facet: the n vertices, then their n normals.
    for s in segments(&with_n) {
        assert_eq!(s.lntp[1], 3008, "facet with normals");
        let n = s.ngeom as usize / 2;
        let (pts, nrm) = s.geom.split_at(3 * n);
        let p = |i: usize| [pts[3 * i], pts[3 * i + 1], pts[3 * i + 2]];
        let (a, b, c) = (p(0), p(1), p(2));
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let cross = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        let len = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
        for k in nrm.chunks(3) {
            let dot = (k[0] * cross[0] + k[1] * cross[1] + k[2] * cross[2]) / len;
            assert!(
                (dot - 1.0).abs() < 1e-12,
                "normal {k:?} is the facet's outward normal"
            );
        }
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// PK_GEOM_render
// -----------------------------------------------------------------------------

/// `PK_GEOM_render` at version 5 on a cubic B-curve: polyline output by
/// default; `bcurve` (byte 0x24) = NURBS sends the curve's own data instead;
/// version 6 is 5022.
pub fn geom_render_v5() -> R {
    let o = PK_GEOM_render_o_t::default();
    #[rustfmt::skip]
    let tokens = [
        (0x00, 5), (0x04, 0x4ed4), (0x20, 0x4efd), (0x24, 0x4fb0), (0x28, 0x6b08), (0x40, 0x4f2e),
        (0x60, 0x100), (0x64, 0x100), (0x68, 0x100), (0x6c, 0x6b12), (0xb8, 0x6b1c), (0xbc, 0x6b26),
        (0x108, 0x6950),
    ];
    for (off, want) in tokens {
        assert_eq!(
            i32_at(&o, off),
            want,
            "default i32 at GEOM_render byte {off:#x}"
        );
    }
    assert_eq!(
        (f64_at(&o, 0x98), f64_at(&o, 0xa0)),
        (1.0, 1.0),
        "grid axis z, ref x"
    );

    let _s = Session::start(go_cfg())?;
    let p = |x: f64, y: f64| Vec3::new(x, y, 0.0);
    let bc = Curve::bcurve(
        3,
        &[p(0.0, 0.0), p(1.0, 2.0), p(3.0, 2.0), p(4.0, 0.0)],
        &[0.0, 1.0],
        &[4, 4],
    )?;
    let run = |o: &PK_GEOM_render_o_t| {
        let mut g = bc.tag();
        let mut o = *o;
        go::clear();
        let rc = unsafe { PK_GEOM_render(1, &mut g, std::ptr::null_mut(), &mut o) };
        (rc, go::take())
    };
    let (rc, poly) = run(&o);
    let mut nurbs = o;
    nurbs.bcurve = PK_render_bcurve_nurbs_c;
    let (rcn, nr) = run(&nurbs);
    let mut v6 = o;
    v6.o_t_version = 6;
    let (rc6, _) = run(&v6);
    println!(
        "\n    polyline{}\n    nurbs{}\n    v6 {rc6}",
        summary(&poly),
        summary(&nr)
    );
    assert_eq!((rc, rcn), (0, 0));
    assert_eq!(rc6, PK_ERROR_o_t_version_unknown);
    let sp = segments(&poly);
    let sn = segments(&nr);
    assert!(!sp.is_empty() && !sn.is_empty());
    // The polyline starts and ends at the curve ends.
    let g = &sp[0].geom;
    assert!(g.len() >= 6 && g[..3] == [0.0, 0.0, 0.0], "{g:?}");
    assert_ne!(
        sp.iter().map(|s| s.lntp.clone()).collect::<Vec<_>>(),
        sn.iter().map(|s| s.lntp.clone()).collect::<Vec<_>>(),
        "NURBS output uses another line type"
    );
    Ok(())
}
