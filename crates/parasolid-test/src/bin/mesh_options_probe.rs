//! Live layout probe for the mesh (convergent-modelling) option structs in
//! `parasolid_sys::mesh_options` (parasolid-re B-R45-1).
//!
//! For each struct:
//! - `version`: the default struct stamped N (the current version) must not give
//!   5022, and N + 1 must give `PK_ERROR_o_t_version_unknown` (5022);
//! - `field`: garbage in one validated field, written at the Rust `offset_of!`
//!   offset, must give that field's own error (908 not_a_logical for a logical,
//!   5014 field_of_wrong_type for a token), while the same call with defaults
//!   does not. That proves the kernel reads the field at our offset.
//!
//! Argument checking is on (the validators run only then).
//!
//! Run (from the target dir, with pskernel.dll next to the exe):
//!   WINEDEBUG=-all wine ./mesh_options_probe.exe [explore|versions|fields|all]

use core::mem::offset_of;
use parasolid::*;
use parasolid_sys::*;
use std::os::raw::{c_int, c_void};

const NL: PK_LOGICAL_t = 2; // not a logical
const BAD_TOKEN: c_int = 987654;

fn session() -> Session {
    let s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    unsafe {
        let rc = PK_SESSION_set_facet_geometry(PK_facet_geometry_all_c);
        assert_eq!(rc, 0, "set_facet_geometry");
    }
    s
}

/// Facet copy of a block (via the verified v2 struct with defaults).
fn facet_body(classic: PK_BODY_t) -> (i32, PK_BODY_t) {
    let opts = PK_BODY_make_facet_body_o_t::default();
    let mut copy: PK_BODY_t = 0;
    let mut tr = PK_TOPOL_track_r_t::default();
    let mut red = PK_TOPOL_track_r_t::default();
    let rc = unsafe {
        PK_BODY_make_facet_body(
            classic,
            PK_ENTITY_null,
            &opts as *const _ as *mut _,
            &mut copy,
            &mut tr,
            &mut red,
        )
    };
    (rc, copy)
}

fn faces_of(body: PK_BODY_t) -> Vec<PK_FACE_t> {
    let mut n = 0;
    let mut p: *mut PK_FACE_t = std::ptr::null_mut();
    unsafe {
        assert_eq!(PK_BODY_ask_faces(body, &mut n, &mut p), 0);
        let v = std::slice::from_raw_parts(p, n as usize).to_vec();
        PK_MEMORY_free(p as *mut c_void);
        v
    }
}

fn fins_of(body: PK_BODY_t) -> Vec<PK_FIN_t> {
    let mut n = 0;
    let mut p: *mut PK_FIN_t = std::ptr::null_mut();
    unsafe {
        assert_eq!(PK_BODY_ask_fins(body, &mut n, &mut p), 0);
        let v = std::slice::from_raw_parts(p, n as usize).to_vec();
        PK_MEMORY_free(p as *mut c_void);
        v
    }
}

fn surf_of(face: PK_FACE_t) -> PK_SURF_t {
    let mut s = 0;
    unsafe { assert_eq!(PK_FACE_ask_surf(face, &mut s), 0) };
    s
}

fn class_of(e: PK_ENTITY_t) -> PK_CLASS_t {
    let mut c = 0;
    unsafe { PK_ENTITY_ask_class(e, &mut c) };
    c
}

fn sharp_mvxs(mesh: PK_MESH_t, o: &PK_MESH_find_sharp_mvxs_o_t) -> (i32, Vec<PK_MVERTEX_t>) {
    let mut n = 0;
    let mut p: *mut PK_MVERTEX_t = std::ptr::null_mut();
    let rc = unsafe { PK_MESH_find_sharp_mvxs(mesh, o, &mut n, &mut p) };
    let v = if rc == 0 && !p.is_null() {
        let v = unsafe { std::slice::from_raw_parts(p, n as usize).to_vec() };
        unsafe { PK_MEMORY_free(p as *mut c_void) };
        v
    } else {
        Vec::new()
    };
    (rc, v)
}

fn sharp_mfins(mesh: PK_MESH_t, o: &PK_MESH_find_sharp_mfins_o_t) -> (i32, i32, Vec<PK_MFIN_t>) {
    let mut n = 0;
    let mut p: *mut PK_MFIN_t = std::ptr::null_mut();
    let rc = unsafe { PK_MESH_find_sharp_mfins(mesh, o, &mut n, &mut p) };
    let v = if rc == 0 && !p.is_null() {
        let v = unsafe { std::slice::from_raw_parts(p, n as usize).to_vec() };
        unsafe { PK_MEMORY_free(p as *mut c_void) };
        v
    } else {
        Vec::new()
    };
    (rc, n, v)
}

fn n_mfacets(mesh: PK_MESH_t) -> (i32, i32) {
    let mut n = -1;
    let rc = unsafe { PK_MESH_ask_n_mfacets(mesh, &mut n) };
    (rc, n)
}

fn n_mvertices(mesh: PK_MESH_t) -> (i32, i32) {
    let mut n = -1;
    let rc = unsafe { PK_MESH_ask_n_mvertices(mesh, &mut n) };
    (rc, n)
}

fn explore() {
    println!("== explore");
    let _s = session();
    let block = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    let (rc, fb) = facet_body(block.tag());
    println!(
        "make_facet_body(block) rc={rc} copy={fb} class={}",
        class_of(fb)
    );
    let faces = faces_of(fb);
    println!("facet body faces: {}", faces.len());
    let mut meshes = Vec::new();
    for &f in &faces {
        let s = surf_of(f);
        let (r1, nf) = n_mfacets(s);
        let (r2, nv) = n_mvertices(s);
        println!(
            "  face {f}: surf {s} class {} n_mfacets rc={r1} {nf}  n_mvertices rc={r2} {nv}",
            class_of(s)
        );
        meshes.push(s);
    }
    meshes.sort();
    meshes.dedup();
    println!("distinct meshes: {:?}", meshes);
    for &m in &meshes {
        let (rc, v) = sharp_mvxs(m, &PK_MESH_find_sharp_mvxs_o_t::default());
        println!("  find_sharp_mvxs(mesh {m}) rc={rc} n={}", v.len());
        for mv in v.iter().take(10) {
            let mut pos: PK_VECTOR_t = [f64::NAN; 3];
            let r = unsafe { PK_MVERTEX_ask_position(*mv, &mut pos) };
            println!(
                "     mvx words={:?} ask_position rc={r} pos={:?}",
                mv.words, pos
            );
        }
        let (rc, n, v) = sharp_mfins(m, &PK_MESH_find_sharp_mfins_o_t::default());
        println!("  find_sharp_mfins(mesh {m}) rc={rc} n={n} got={}", v.len());
        for mf in v.iter().take(4) {
            let mut mvx = PK_MTOPOL_t { words: [0, 0] };
            let r = unsafe { PK_MFIN_ask_mvertex(*mf, &mut mvx) };
            let mut pos: PK_VECTOR_t = [f64::NAN; 3];
            let r2 = unsafe { PK_MVERTEX_ask_position(mvx, &mut pos) };
            println!(
                "     mfin words={:?} ask_mvertex rc={r} -> {:?} pos rc={r2} {:?}",
                mf.words, mvx.words, pos
            );
        }
    }
    // sanity: a cylinder
    let cyl = Body::create_solid_cylinder(5.0, 10.0).unwrap();
    let (rc, fc) = facet_body(cyl.tag());
    println!("make_facet_body(cylinder) rc={rc} copy={fc}");
    for f in faces_of(fc) {
        let s = surf_of(f);
        let (rc, v) = sharp_mvxs(s, &PK_MESH_find_sharp_mvxs_o_t::default());
        let (rc2, n2, _) = sharp_mfins(s, &PK_MESH_find_sharp_mfins_o_t::default());
        println!(
            "  cyl face {f} mesh {s} nf={:?} nv={:?} sharp_mvxs rc={rc} n={} sharp_mfins rc={rc2} n={n2}",
            n_mfacets(s),
            n_mvertices(s),
            v.len()
        );
    }
    let fins = fins_of(fb);
    println!("facet body fins: {}", fins.len());
    if let Some(&fin) = fins.first() {
        let o = PK_FIN_find_mtopols_o_t::default();
        let (mut nv, mut nf) = (0, 0);
        let mut pv: *mut PK_MVERTEX_t = std::ptr::null_mut();
        let mut pf: *mut PK_MFIN_t = std::ptr::null_mut();
        let mut sense: PK_LOGICAL_t = 9;
        let rc = unsafe {
            PK_FIN_find_mtopols(
                fin,
                &o as *const _ as *mut _,
                &mut nv,
                &mut pv,
                &mut nf,
                &mut pf,
                &mut sense,
            )
        };
        println!(
            "  FIN_find_mtopols(fin {fin}) rc={rc} n_mvertices={nv} n_mfins={nf} sense={sense}"
        );
        if rc == 0 {
            for i in 0..nv.min(4) as usize {
                let mv = unsafe { *pv.add(i) };
                let mut pos: PK_VECTOR_t = [f64::NAN; 3];
                let r = unsafe { PK_MVERTEX_ask_position(mv, &mut pos) };
                println!("     mvx {:?} pos rc={r} {:?}", mv.words, pos);
            }
        }
    }
}

/// The mfacets of the mesh that a fin's first mvertex belongs to (via mvertex -> mfacets).
fn mfacets_of_face_mesh(fin: PK_FIN_t) -> Vec<PK_MFACET_t> {
    let o = PK_FIN_find_mtopols_o_t::default();
    let (mut nv, mut nf) = (0, 0);
    let mut pv: *mut PK_MVERTEX_t = std::ptr::null_mut();
    let mut pf: *mut PK_MFIN_t = std::ptr::null_mut();
    let mut sense: PK_LOGICAL_t = 0;
    let rc = unsafe {
        PK_FIN_find_mtopols(
            fin,
            &o as *const _ as *mut _,
            &mut nv,
            &mut pv,
            &mut nf,
            &mut pf,
            &mut sense,
        )
    };
    assert_eq!(rc, 0);
    let mut out: Vec<PK_MFACET_t> = Vec::new();
    for i in 0..nv as usize {
        let mv = unsafe { *pv.add(i) };
        let mut n = 0;
        let mut p: *mut PK_MFACET_t = std::ptr::null_mut();
        let rc = unsafe { PK_MVERTEX_ask_mfacets(mv, &mut n, &mut p) };
        assert_eq!(rc, 0, "MVERTEX_ask_mfacets");
        for j in 0..n as usize {
            let f = unsafe { *p.add(j) };
            if !out.contains(&f) {
                out.push(f);
            }
        }
        unsafe { PK_MEMORY_free(p as *mut c_void) };
    }
    out
}

fn mesh_mvertices_via_fins(body: PK_BODY_t, mesh: PK_MESH_t) -> Vec<PK_MVERTEX_t> {
    let mut out = Vec::new();
    for fin in fins_of(body) {
        let o = PK_FIN_find_mtopols_o_t::default();
        let (mut nv, mut nf) = (0, 0);
        let mut pv: *mut PK_MVERTEX_t = std::ptr::null_mut();
        let mut pf: *mut PK_MFIN_t = std::ptr::null_mut();
        let mut sense: PK_LOGICAL_t = 0;
        if unsafe {
            PK_FIN_find_mtopols(
                fin,
                &o as *const _ as *mut _,
                &mut nv,
                &mut pv,
                &mut nf,
                &mut pf,
                &mut sense,
            )
        } != 0
        {
            continue;
        }
        for i in 0..nv as usize {
            let mv = unsafe { *pv.add(i) };
            if mv.words[0] == mesh && !out.contains(&mv) {
                out.push(mv);
            }
        }
    }
    out
}

fn effects() {
    println!("== effects");
    let _s = session();
    // cylinder side mesh: 32 mfacets, 32 mvertices, a tube with two open ends
    let cyl = Body::create_solid_cylinder(5.0, 10.0).unwrap();
    let (_, fc) = facet_body(cyl.tag());
    let side = faces_of(fc)
        .into_iter()
        .map(surf_of)
        .max_by_key(|&m| n_mfacets(m).1)
        .unwrap();
    println!(
        "side mesh {side} nf={:?} nv={:?}",
        n_mfacets(side),
        n_mvertices(side)
    );
    let mut copy = 0;
    let rc = unsafe { PK_ENTITY_copy(side, &mut copy) };
    println!(
        "ENTITY_copy(side) rc={rc} copy={copy} class={} nf={:?}",
        class_of(copy),
        n_mfacets(copy)
    );
    // sharp features before/after store_normals with a small mesh_angle
    let (_, v0) = sharp_mvxs(side, &PK_MESH_find_sharp_mvxs_o_t::default());
    let (_, n0, _) = sharp_mfins(side, &PK_MESH_find_sharp_mfins_o_t::default());
    let fa = PK_MESH_find_sharp_mfins_o_t {
        have_sharp_angle: 1,
        sharp_angle: 0.1,
        ..Default::default()
    };
    let (rca, na, _) = sharp_mfins(side, &fa);
    let fb = PK_MESH_find_sharp_mfins_o_t {
        have_sharp_angle: 1,
        sharp_angle: 1.0,
        ..Default::default()
    };
    let (rcb, nb, _) = sharp_mfins(side, &fb);
    println!(
        "sharp before: mvxs={} mfins={n0}; mfins(sharp_angle 0.1) rc={rca} n={na}; (1.0) rc={rcb} n={nb}",
        v0.len()
    );
    for ang in [0.1f64, 1.0] {
        let mut c2 = 0;
        unsafe { PK_ENTITY_copy(side, &mut c2) };
        let target = if c2 != 0 { c2 } else { side };
        let o = PK_MESH_store_normals_o_t {
            have_mesh_angle: 1,
            mesh_angle: ang,
            ..Default::default()
        };
        let rc = unsafe { PK_MESH_store_normals(target, &o) };
        let (_, v1) = sharp_mvxs(target, &PK_MESH_find_sharp_mvxs_o_t::default());
        let (_, n1, _) = sharp_mfins(target, &PK_MESH_find_sharp_mfins_o_t::default());
        let mut nvt = 0;
        let mut nl: *mut PK_MVERTEX_t = std::ptr::null_mut();
        let o2 = PK_MESH_find_sharp_mvxs_o_t {
            want_sharp_mvxs: 0,
            ..Default::default()
        };
        let rc2 = unsafe { PK_MESH_find_sharp_mvxs(target, &o2, &mut nvt, &mut nl) };
        println!(
            "store_normals(mesh {target}, mesh_angle {ang}) rc={rc}: sharp mvxs={} mfins={n1}; want=false rc={rc2} n={nvt} ptr_null={}",
            v1.len(),
            nl.is_null()
        );
        if let Some(mv) = v1.first() {
            let mut pos: PK_VECTOR_t = [0.0; 3];
            unsafe { PK_MVERTEX_ask_position(*mv, &mut pos) };
            println!(
                "   first sharp mvx {:?} r={}",
                pos,
                (pos[0] * pos[0] + pos[1] * pos[1]).sqrt()
            );
        }
    }
    // fill_holes on the orphan tube copy
    if copy != 0 {
        for (label, o) in [
            ("defaults", PK_MESH_fill_holes_o_t::default()),
            (
                "max_n_mfins=3",
                PK_MESH_fill_holes_o_t {
                    max_n_mfins: 3,
                    ..Default::default()
                },
            ),
            (
                "refined",
                PK_MESH_fill_holes_o_t {
                    fill_shape: PK_MESH_fill_shape_refined_c,
                    ..Default::default()
                },
            ),
        ] {
            let mut c3 = 0;
            unsafe { PK_ENTITY_copy(side, &mut c3) };
            let mut filled = 0;
            let rc = unsafe { PK_MESH_fill_holes(c3, &o as *const _ as *mut _, &mut filled) };
            println!(
                "fill_holes(tube copy {c3}, {label}) rc={rc} filled={filled} nf={:?} nv={:?}",
                n_mfacets(filled),
                n_mvertices(filled)
            );
        }
        // boundary mfins of the side mesh, via the fins of the cylinder's circular edges
        let mut bmf: Vec<PK_MFIN_t> = Vec::new();
        for fin in fins_of(fc) {
            let o = PK_FIN_find_mtopols_o_t::default();
            let (mut nv, mut nf) = (0, 0);
            let mut pv: *mut PK_MVERTEX_t = std::ptr::null_mut();
            let mut pf: *mut PK_MFIN_t = std::ptr::null_mut();
            let mut sense: PK_LOGICAL_t = 0;
            if unsafe {
                PK_FIN_find_mtopols(
                    fin,
                    &o as *const _ as *mut _,
                    &mut nv,
                    &mut pv,
                    &mut nf,
                    &mut pf,
                    &mut sense,
                )
            } == 0
            {
                for i in 0..nf as usize {
                    let mf = unsafe { *pf.add(i) };
                    if mf.words[0] == side {
                        bmf.push(mf);
                    }
                }
            }
        }
        println!(
            "side-mesh boundary mfins via fins: {} first {:?}",
            bmf.len(),
            bmf.first().map(|m| m.words)
        );
        for (label, maxn, hole) in [
            ("max 1000", 1000, false),
            ("input_holes[1]", 0, true),
            ("input_holes[1] max 1000", 1000, true),
            ("input_holes[1] max 3", 3, true),
        ] {
            let mut c3 = 0;
            unsafe { PK_ENTITY_copy(side, &mut c3) };
            let h = [PK_MTOPOL_t {
                words: [c3, bmf.first().map(|m| m.words[1]).unwrap_or(0)],
            }];
            let o = PK_MESH_fill_holes_o_t {
                max_n_mfins: maxn,
                n_input_holes: if hole { 1 } else { 0 },
                input_holes: if hole { h.as_ptr() } else { std::ptr::null() },
                ..Default::default()
            };
            let mut filled = 0;
            let rc = unsafe { PK_MESH_fill_holes(c3, &o as *const _ as *mut _, &mut filled) };
            println!(
                "fill_holes(tube copy {c3}, {label}) rc={rc} filled={filled} nf={:?} nv={:?}",
                n_mfacets(filled),
                n_mvertices(filled)
            );
        }
        // fix/find defects on the orphan copy
        let mut n = -1;
        let mut p: *mut PK_MESH_defect_details_t = std::ptr::null_mut();
        let rc = unsafe {
            PK_MESH_find_defects(
                copy,
                &PK_MESH_find_defects_o_t::default() as *const _ as *mut _,
                &mut n,
                &mut p,
            )
        };
        println!("find_defects(tube copy) rc={rc} n={n}");
    }
    // a degenerate mfacet: move one mvertex of an orphan square onto another
    let block = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    let (_, fbody) = facet_body(block.tag());
    let face_mesh = surf_of(faces_of(fbody)[0]);
    let mut sq = 0;
    let rc = unsafe { PK_ENTITY_copy(face_mesh, &mut sq) };
    println!("square copy rc={rc} {sq} nf={:?}", n_mfacets(sq));
    let mvs_orig = mesh_mvertices_via_fins(fbody, face_mesh);
    println!(
        "face mesh mvertices via fins: {:?}",
        mvs_orig.iter().map(|m| m.words).collect::<Vec<_>>()
    );
    // copy's mvertices: same indices, copy's tag
    let mut mvs: Vec<PK_MVERTEX_t> = mvs_orig
        .iter()
        .map(|m| PK_MTOPOL_t {
            words: [sq, m.words[1]],
        })
        .collect();
    if mvs.len() >= 2 {
        let mut p0: PK_VECTOR_t = [0.0; 3];
        let r0 = unsafe { PK_MVERTEX_ask_position(mvs[0], &mut p0) };
        let mut p1: PK_VECTOR_t = [0.0; 3];
        let r1 = unsafe { PK_MVERTEX_ask_position(mvs[1], &mut p1) };
        println!("copy mvx0 rc={r0} {p0:?} mvx1 rc={r1} {p1:?}");
        let mut pos = [p0];
        let rc = unsafe {
            PK_MVERTEX_set_positions(
                1,
                mvs[1..].as_mut_ptr(),
                pos.as_mut_ptr(),
                std::ptr::null_mut(),
            )
        };
        println!("set_positions(mvx1 := mvx0) rc={rc}");
        for (label, o) in [
            ("defaults", PK_MESH_find_defects_o_t::default()),
            (
                "mfacet no",
                PK_MESH_find_defects_o_t {
                    mfacet_find_defects: PK_MFACET_find_defect_no_c,
                    ..Default::default()
                },
            ),
            (
                "max_defects 1",
                PK_MESH_find_defects_o_t {
                    max_defects: 1,
                    ..Default::default()
                },
            ),
        ] {
            let mut n = -1;
            let mut p: *mut PK_MESH_defect_details_t = std::ptr::null_mut();
            let rc = unsafe { PK_MESH_find_defects(sq, &o as *const _ as *mut _, &mut n, &mut p) };
            print!("find_defects(degenerate square, {label}) rc={rc} n={n}");
            if rc == 0 && n > 0 && !p.is_null() {
                for i in 0..n as usize {
                    let d = unsafe { *p.add(i) };
                    print!(
                        " [defect {} n_ent {} n_mtopols {} {:?} n_pos {} n_uv {}]",
                        d.defect,
                        d.n_entities,
                        d.n_mtopols,
                        if d.n_mtopols > 0 {
                            unsafe { (*d.mtopols).words }
                        } else {
                            [0, 0]
                        },
                        d.n_positions,
                        d.n_uv_params
                    );
                }
            }
            println!();
        }
        for (label, o) in [
            ("defaults", PK_MESH_fix_defects_o_t::default()),
            (
                "mfacet none",
                PK_MESH_fix_defects_o_t {
                    mfacet_fix_defects: PK_MFACET_fix_defect_none_c,
                    ..Default::default()
                },
            ),
        ] {
            let mut c4 = 0;
            unsafe { PK_ENTITY_copy(sq, &mut c4) };
            let mut res = 0;
            let mut n = -1;
            let mut pm: *mut PK_MESH_t = std::ptr::null_mut();
            let mut pd: *mut PK_MESH_defect_array_t = std::ptr::null_mut();
            let rc = unsafe {
                PK_MESH_fix_defects(
                    c4,
                    &o as *const _ as *mut _,
                    &mut res,
                    &mut n,
                    &mut pm,
                    &mut pd,
                )
            };
            let m0 = if rc == 0 && n > 0 { unsafe { *pm } } else { 0 };
            println!(
                "fix_defects(degenerate square copy, {label}) rc={rc} result={res} n={n} mesh0={m0} nf={:?}",
                n_mfacets(m0)
            );
        }
    }
    // make_bodies on the orphan square: vertex_angle effect on vertex count
    for va in [4.0f64, 0.1] {
        let mut c5 = 0;
        unsafe { PK_ENTITY_copy(face_mesh, &mut c5) };
        let o = PK_MESH_make_bodies_o_t {
            vertex_angle: va,
            ..Default::default()
        };
        let mut n = 0;
        let mut p: *mut PK_BODY_t = std::ptr::null_mut();
        let rc = unsafe { PK_MESH_make_bodies(c5, &o, &mut n, &mut p) };
        if rc == 0 && n > 0 {
            let b = Body::from_tag_checked(unsafe { *p }).unwrap();
            println!(
                "make_bodies(vertex_angle {va}) rc={rc} n={n} topo={:?} type={:?}",
                b.topology_summary(),
                b.body_type()
            );
        } else {
            println!("make_bodies(vertex_angle {va}) rc={rc} n={n}");
        }
    }
    // make_facet_body max facet width effect
    for w in [None, Some(2.0)] {
        let mut o = PK_BODY_make_facet_body_o_t::default();
        if let Some(w) = w {
            o.is_max_facet_width = 1;
            o.max_facet_width = w;
        }
        let mut copy = 0;
        let mut tr = PK_TOPOL_track_r_t::default();
        let mut red = PK_TOPOL_track_r_t::default();
        let rc = unsafe {
            PK_BODY_make_facet_body(block.tag(), 0, &mut o, &mut copy, &mut tr, &mut red)
        };
        let tot: i32 = if rc == 0 {
            faces_of(copy)
                .into_iter()
                .map(|f| n_mfacets(surf_of(f)).1)
                .sum()
        } else {
            -1
        };
        println!("make_facet_body(max_facet_width {w:?}) rc={rc} total mfacets={tot}");
    }
    // pick: ignore_back_faces
    for ibf in [1u8, 0] {
        let o = PK_BODY_pick_topols_o_t {
            max_faces: 2,
            ignore_back_faces: ibf,
            ..Default::default()
        };
        let mut bodies = [block.tag()];
        let mut ray = PK_AXIS1_sf_t {
            location: [0.0, 0.0, 100.0],
            axis: [0.0, 0.0, -1.0],
        };
        let mut picked = PK_BODY_pick_topols_r_t::default();
        let rc = unsafe {
            PK_BODY_pick_topols(
                1,
                bodies.as_mut_ptr(),
                std::ptr::null_mut(),
                &mut ray,
                &o as *const _ as *mut _,
                &mut picked,
            )
        };
        let hits: Vec<_> = (0..picked.n_faces.max(0) as usize)
            .map(|i| unsafe { *picked.faces.add(i) })
            .collect();
        println!(
            "pick(max_faces 2, ignore_back_faces {ibf}) rc={rc} n_faces={} hits={:?}",
            picked.n_faces,
            hits.iter()
                .map(|h| (h.entity, class_of(h.entity), h.intersect, h.distance))
                .collect::<Vec<_>>()
        );
    }
    // FIN_find_mtopols want_mvertices = false
    let fin = fins_of(fbody)[0];
    let o = PK_FIN_find_mtopols_o_t {
        want_mvertices: 0,
        ..Default::default()
    };
    let (mut nv, mut nf) = (-1, -1);
    let mut pv: *mut PK_MVERTEX_t = std::ptr::null_mut();
    let mut pf: *mut PK_MFIN_t = std::ptr::null_mut();
    let mut sense: PK_LOGICAL_t = 9;
    let rc = unsafe {
        PK_FIN_find_mtopols(
            fin,
            &o as *const _ as *mut _,
            &mut nv,
            &mut pv,
            &mut nf,
            &mut pf,
            &mut sense,
        )
    };
    println!(
        "FIN_find_mtopols(want_mvertices=false) rc={rc} nv={nv} pv_null={} nf={nf}",
        pv.is_null()
    );
    // imprint want_mvertices = true
    let mut c6 = 0;
    unsafe { PK_ENTITY_copy(face_mesh, &mut c6) };
    let o = PK_MESH_imprint_vectors_o_t {
        want_mvertices: 1,
        ..Default::default()
    };
    let mut v = [[1.0f64, 2.0, 30.0]];
    let mut res_mesh = 0;
    let mut mvx = [PK_MTOPOL_t { words: [0, 0] }];
    let mut res = [0u64; 64];
    let rc = unsafe {
        PK_MESH_imprint_vectors(
            c6,
            1,
            v.as_mut_ptr(),
            &o as *const _ as *mut _,
            &mut res_mesh,
            mvx.as_mut_ptr(),
            res.as_mut_ptr() as *mut PK_MESH_imprint_vectors_r_t,
        )
    };
    let mut pos: PK_VECTOR_t = [0.0; 3];
    let rp = unsafe { PK_MVERTEX_ask_position(mvx[0], &mut pos) };
    println!(
        "imprint(want_mvertices) rc={rc} mesh {c6}->{res_mesh} nf={:?} mvx={:?} pos rc={rp} {pos:?} r[0]={:#x}",
        n_mfacets(res_mesh),
        mvx[0].words,
        res[0]
    );
    // partition ask with want_parts
    let part = _s.partitions().unwrap()[0].tag();
    for (wp, wg) in [(0u8, 0u8), (1, 1)] {
        let o = PK_PARTITION_ask_facet_geom_o_t {
            want_parts: wp,
            want_geoms: wg,
            ..Default::default()
        };
        let (mut cat, mut np, mut ng) = (0, -1, -1);
        let mut parts: *mut PK_PART_t = std::ptr::null_mut();
        let mut cats: *mut PK_geom_category_t = std::ptr::null_mut();
        let mut geoms: *mut PK_GEOM_t = std::ptr::null_mut();
        let rc = unsafe {
            PK_PARTITION_ask_facet_geom(
                part, &o, &mut cat, &mut np, &mut parts, &mut cats, &mut ng, &mut geoms,
            )
        };
        let pl: Vec<_> = if rc == 0 && !parts.is_null() {
            (0..np as usize)
                .map(|i| unsafe { (*parts.add(i), *cats.add(i)) })
                .collect()
        } else {
            vec![]
        };
        let gl: Vec<_> = if rc == 0 && !geoms.is_null() {
            (0..ng as usize).map(|i| unsafe { *geoms.add(i) }).collect()
        } else {
            vec![]
        };
        println!(
            "PARTITION_ask_facet_geom(want {wp},{wg}) rc={rc} cat={cat} n_parts={np} {pl:?} n_geoms={ng} {gl:?}"
        );
    }
}

// --- version and field probes ---------------------------------------------------

/// Write `val` bytes at `off` into a copy of `base` and return the copy.
fn poke<T: Copy, V: Copy>(base: &T, off: usize, val: V) -> T {
    let mut t = *base;
    unsafe {
        let p = (&mut t as *mut T as *mut u8).add(off) as *mut V;
        p.write_unaligned(val);
    }
    t
}

struct Report {
    ok: usize,
    bad: usize,
}

impl Report {
    fn check(&mut self, what: &str, rc: i32, expect: &[i32], note: &str) {
        let good = expect.contains(&rc);
        if good {
            self.ok += 1
        } else {
            self.bad += 1
        }
        println!(
            "  [{}] {what}: rc={rc} expect {:?} {note}",
            if good { "ok" } else { "MISMATCH" },
            expect
        );
    }
}

/// Check one struct: `call(&s) -> rc` with fresh operands per call.
fn version_check<T: Copy>(
    r: &mut Report,
    name: &str,
    def: &T,
    ver: c_int,
    mut call: impl FnMut(&T) -> i32,
) {
    println!("-- {name} (v{ver})");
    let rc0 = call(def);
    r.check(&format!("{name} v{ver} defaults"), rc0, &[0], "");
    let hi = poke(def, 0, ver + 1);
    r.check(
        &format!("{name} v{}", ver + 1),
        call(&hi),
        &[5022],
        "above ceiling",
    );
    let zero = poke(def, 0, 0 as c_int);
    r.check(&format!("{name} v0"), call(&zero), &[5022], "below floor");
}

fn field_check<T: Copy, V: Copy>(
    r: &mut Report,
    name: &str,
    def: &T,
    field: &str,
    off: usize,
    val: V,
    expect: &[i32],
    mut call: impl FnMut(&T) -> i32,
) {
    let t = poke(def, off, val);
    r.check(
        &format!("{name}.{field} @{off:#x} garbage"),
        call(&t),
        expect,
        "",
    );
}

fn probes(versions: bool, fields: bool) {
    let mut r = Report { ok: 0, bad: 0 };
    let _s = session();
    let block = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();

    // PK_BODY_make_facet_body_o_t
    {
        type S = PK_BODY_make_facet_body_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let mut copy = 0;
            let mut tr = PK_TOPOL_track_r_t::default();
            let mut red = PK_TOPOL_track_r_t::default();
            unsafe {
                PK_BODY_make_facet_body(
                    block.tag(),
                    0,
                    o as *const _ as *mut _,
                    &mut copy,
                    &mut tr,
                    &mut red,
                )
            }
        };
        if versions {
            version_check(&mut r, "make_facet_body", &d, 2, &mut call);
            let v1 = poke(&d, 0, 1 as c_int);
            r.check("make_facet_body v1 (migrated)", call(&v1), &[0], "");
        }
        if fields {
            for (f, off) in [
                ("is_max_facet_width", offset_of!(S, is_max_facet_width)),
                (
                    "is_distance_tolerance",
                    offset_of!(S, is_distance_tolerance),
                ),
                ("is_angular_tolerance", offset_of!(S, is_angular_tolerance)),
                ("is_max_chord_length", offset_of!(S, is_max_chord_length)),
                ("retain_attributes", offset_of!(S, retain_attributes)),
                ("retain_groups", offset_of!(S, retain_groups)),
                ("track_faces", offset_of!(S, track_faces)),
                ("return_redundant", offset_of!(S, return_redundant)),
            ] {
                field_check(&mut r, "make_facet_body", &d, f, off, NL, &[908], &mut call);
            }
            for (f, off) in [
                ("track_edges", offset_of!(S, track_edges)),
                ("partial_conversion", offset_of!(S, partial_conversion)),
            ] {
                field_check(
                    &mut r,
                    "make_facet_body",
                    &d,
                    f,
                    off,
                    BAD_TOKEN,
                    &[5014],
                    &mut call,
                );
            }
            for off in [5usize, 7, 0x11, 0x27, 0x43, 0x49, 0x4b] {
                field_check(
                    &mut r,
                    "make_facet_body",
                    &d,
                    "padding",
                    off,
                    0xABu8,
                    &[0],
                    &mut call,
                );
            }
        }
    }

    // facet operands
    let (rc, fb) = facet_body(block.tag());
    assert_eq!(rc, 0);
    let faces = faces_of(fb);
    let mesh = surf_of(faces[0]);
    let fin = fins_of(fb)[0];

    // PK_MESH_find_sharp_mvxs_o_t
    {
        type S = PK_MESH_find_sharp_mvxs_o_t;
        let d = S::default();
        let mut call = |o: &S| sharp_mvxs(mesh, o).0;
        if versions {
            version_check(&mut r, "find_sharp_mvxs", &d, 1, &mut call);
        }
        if fields {
            field_check(
                &mut r,
                "find_sharp_mvxs",
                &d,
                "want_sharp_mvxs",
                offset_of!(S, want_sharp_mvxs),
                NL,
                &[908],
                &mut call,
            );
        }
    }
    // PK_MESH_find_sharp_mfins_o_t
    {
        type S = PK_MESH_find_sharp_mfins_o_t;
        let d = S::default();
        let mut call = |o: &S| sharp_mfins(mesh, o).0;
        if versions {
            version_check(&mut r, "find_sharp_mfins", &d, 2, &mut call);
            r.check(
                "find_sharp_mfins v1 (migrated)",
                call(&poke(&d, 0, 1 as c_int)),
                &[0],
                "",
            );
        }
        if fields {
            field_check(
                &mut r,
                "find_sharp_mfins",
                &d,
                "want_sharp_mfins",
                offset_of!(S, want_sharp_mfins),
                NL,
                &[908],
                &mut call,
            );
            field_check(
                &mut r,
                "find_sharp_mfins",
                &d,
                "have_sharp_angle",
                offset_of!(S, have_sharp_angle),
                NL,
                &[908],
                &mut call,
            );
            let t = S {
                have_sharp_angle: 1,
                sharp_angle: f64::NAN,
                ..d
            };
            r.check(
                "find_sharp_mfins.sharp_angle NaN (have=1)",
                call(&t),
                &[9999],
                "NaN double",
            );
        }
    }
    // PK_FIN_find_mtopols_o_t
    {
        type S = PK_FIN_find_mtopols_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let (mut nv, mut nf) = (0, 0);
            let mut pv: *mut PK_MVERTEX_t = std::ptr::null_mut();
            let mut pf: *mut PK_MFIN_t = std::ptr::null_mut();
            let mut sense: PK_LOGICAL_t = 0;
            unsafe {
                PK_FIN_find_mtopols(
                    fin,
                    o as *const _ as *mut _,
                    &mut nv,
                    &mut pv,
                    &mut nf,
                    &mut pf,
                    &mut sense,
                )
            }
        };
        if versions {
            version_check(&mut r, "FIN_find_mtopols", &d, 1, &mut call);
        }
        if fields {
            field_check(
                &mut r,
                "FIN_find_mtopols",
                &d,
                "want_mvertices",
                offset_of!(S, want_mvertices),
                NL,
                &[908],
                &mut call,
            );
            field_check(
                &mut r,
                "FIN_find_mtopols",
                &d,
                "want_mfins",
                offset_of!(S, want_mfins),
                NL,
                &[908],
                &mut call,
            );
        }
    }
    // PK_PARTITION_ask_facet_geom_o_t
    {
        type S = PK_PARTITION_ask_facet_geom_o_t;
        let d = S::default();
        let part = _s.partitions().unwrap()[0].tag();
        println!("  (partition tag={part} class={})", class_of(part));
        let mut call = |o: &S| {
            let (mut cat, mut np, mut ng) = (0, -1, -1);
            let mut parts: *mut PK_PART_t = std::ptr::null_mut();
            let mut cats: *mut PK_geom_category_t = std::ptr::null_mut();
            let mut geoms: *mut PK_GEOM_t = std::ptr::null_mut();
            let rc = unsafe {
                PK_PARTITION_ask_facet_geom(
                    part, o, &mut cat, &mut np, &mut parts, &mut cats, &mut ng, &mut geoms,
                )
            };
            if rc == 0 {
                let p0 = if np > 0 && !parts.is_null() {
                    unsafe { *parts }
                } else {
                    -1
                };
                let c0 = if np > 0 && !cats.is_null() {
                    unsafe { *cats }
                } else {
                    -1
                };
                println!(
                    "     (ask_facet_geom category={cat} n_parts={np} parts[0]={p0} cat[0]={c0} n_geoms={ng} geoms={:?})",
                    geoms
                );
            }
            rc
        };
        if versions {
            version_check(&mut r, "PARTITION_ask_facet_geom", &d, 1, &mut call);
        }
        if fields {
            field_check(
                &mut r,
                "PARTITION_ask_facet_geom",
                &d,
                "want_parts",
                offset_of!(S, want_parts),
                NL,
                &[908],
                &mut call,
            );
            field_check(
                &mut r,
                "PARTITION_ask_facet_geom",
                &d,
                "want_geoms",
                offset_of!(S, want_geoms),
                NL,
                &[908],
                &mut call,
            );
        }
    }
    // PK_MESH_store_normals_o_t (mesh attached to a face of fb)
    {
        type S = PK_MESH_store_normals_o_t;
        let d = S::default();
        let mut call = |o: &S| unsafe { PK_MESH_store_normals(mesh, o) };
        if versions {
            version_check(&mut r, "store_normals", &d, 1, &mut call);
        }
        if fields {
            field_check(
                &mut r,
                "store_normals",
                &d,
                "have_mesh_angle",
                offset_of!(S, have_mesh_angle),
                NL,
                &[908],
                &mut call,
            );
            field_check(
                &mut r,
                "store_normals",
                &d,
                "replace",
                offset_of!(S, replace),
                BAD_TOKEN,
                &[5014],
                &mut call,
            );
        }
    }
    // ---- operands for the rest: mfacets of one face mesh, an orphan mesh -------
    let mfacets = mfacets_of_face_mesh(fin);
    println!(
        "  (mfacets of fin's mesh: {:?})",
        mfacets.iter().map(|m| m.words).collect::<Vec<_>>()
    );
    let make_meshes = |o: &PK_MTOPOL_make_meshes_o_t| -> (i32, Vec<PK_MESH_t>) {
        let mut n = 0;
        let mut p: *mut PK_MESH_t = std::ptr::null_mut();
        let rc = unsafe {
            PK_MTOPOL_make_meshes(mfacets.len() as c_int, mfacets.as_ptr(), o, &mut n, &mut p)
        };
        let v = if rc == 0 && !p.is_null() {
            let v = unsafe { std::slice::from_raw_parts(p, n as usize).to_vec() };
            unsafe { PK_MEMORY_free(p as *mut c_void) };
            v
        } else {
            Vec::new()
        };
        (rc, v)
    };
    // PK_MTOPOL_make_meshes_o_t
    {
        type S = PK_MTOPOL_make_meshes_o_t;
        let d = S::default();
        let (rc, ms) = make_meshes(&d);
        println!(
            "  (make_meshes defaults rc={rc} meshes={ms:?} classes={:?} n_mfacets={:?})",
            ms.iter().map(|&m| class_of(m)).collect::<Vec<_>>(),
            ms.iter().map(|&m| n_mfacets(m)).collect::<Vec<_>>()
        );
        let mut call = |o: &S| make_meshes(o).0;
        if versions {
            version_check(&mut r, "MTOPOL_make_meshes", &d, 4, &mut call);
            for v in 1..=2 {
                r.check(
                    &format!("MTOPOL_make_meshes v{v} (migrated)"),
                    call(&poke(&d, 0, v as c_int)),
                    &[0],
                    "",
                );
            }
        }
        if fields {
            field_check(
                &mut r,
                "MTOPOL_make_meshes",
                &d,
                "allow_disjoint",
                offset_of!(S, allow_disjoint),
                NL,
                &[908],
                &mut call,
            );
            for (f, off) in [
                ("select_type", offset_of!(S, select_type)),
                ("map_mfacets", offset_of!(S, map_mfacets)),
                ("map_mvertices", offset_of!(S, map_mvertices)),
            ] {
                field_check(
                    &mut r,
                    "MTOPOL_make_meshes",
                    &d,
                    f,
                    off,
                    BAD_TOKEN,
                    &[5014],
                    &mut call,
                );
            }
            field_check(
                &mut r,
                "MTOPOL_make_meshes",
                &d,
                "select_mtopol_class",
                offset_of!(S, select_mtopol_class),
                BAD_TOKEN,
                &[5002],
                &mut call,
            );
        }
    }
    let orphan = || -> PK_MESH_t {
        let (rc, ms) = make_meshes(&PK_MTOPOL_make_meshes_o_t::default());
        assert!(rc == 0 && !ms.is_empty(), "orphan mesh rc={rc}");
        ms[0]
    };

    // PK_MESH_make_bodies_o_t
    {
        type S = PK_MESH_make_bodies_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let m = orphan();
            let mut n = 0;
            let mut p: *mut PK_BODY_t = std::ptr::null_mut();
            let rc = unsafe { PK_MESH_make_bodies(m, o, &mut n, &mut p) };
            if rc == 0 && n > 0 {
                let b = unsafe { *p };
                println!("     (make_bodies n={n} body={b} class={})", class_of(b));
            }
            rc
        };
        if versions {
            version_check(&mut r, "MESH_make_bodies", &d, 1, &mut call);
        }
        if fields {
            field_check(
                &mut r,
                "MESH_make_bodies",
                &d,
                "vertex_angle",
                offset_of!(S, vertex_angle),
                f64::NAN,
                &[9999],
                &mut call,
            );
            field_check(
                &mut r,
                "MESH_make_bodies",
                &d,
                "allow_disjoint",
                offset_of!(S, allow_disjoint),
                NL,
                &[908],
                &mut call,
            );
            field_check(
                &mut r,
                "MESH_make_bodies",
                &d,
                "preferred_body_type",
                offset_of!(S, preferred_body_type),
                BAD_TOKEN,
                &[5014],
                &mut call,
            );
        }
    }

    // PK_MESH_make_surf_trimmed_o_t
    {
        type S = PK_MESH_make_surf_trimmed_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let m = orphan();
            let mut surf = 0;
            let mut sense: PK_LOGICAL_t = 9;
            let mut trim = [0u64; 32];
            let rc = unsafe {
                PK_MESH_make_surf_trimmed(
                    m,
                    o,
                    &mut surf,
                    &mut sense,
                    trim.as_mut_ptr() as *mut c_void,
                )
            };
            if rc == 0 {
                println!(
                    "     (make_surf_trimmed surf={surf} class={} sense={sense} trim[0..4]={:x?})",
                    class_of(surf),
                    &trim[..4]
                );
            }
            rc
        };
        if versions {
            version_check(&mut r, "MESH_make_surf_trimmed", &d, 1, &mut call);
        }
        if fields {
            for (f, off) in [
                ("have_bdry_tolerance", offset_of!(S, have_bdry_tolerance)),
                ("fit_normals", offset_of!(S, fit_normals)),
                (
                    "have_normal_tolerance",
                    offset_of!(S, have_normal_tolerance),
                ),
            ] {
                field_check(
                    &mut r,
                    "MESH_make_surf_trimmed",
                    &d,
                    f,
                    off,
                    NL,
                    &[908],
                    &mut call,
                );
            }
            for (f, off) in [
                ("fitting_tolerance", offset_of!(S, fitting_tolerance)),
                ("bdry_tolerance", offset_of!(S, bdry_tolerance)),
                ("bdry_mvertex_angle", offset_of!(S, bdry_mvertex_angle)),
                ("normal_tolerance", offset_of!(S, normal_tolerance)),
            ] {
                field_check(
                    &mut r,
                    "MESH_make_surf_trimmed",
                    &d,
                    f,
                    off,
                    f64::NAN,
                    &[9999],
                    &mut call,
                );
            }
        }
    }

    // PK_MESH_find_defects_o_t
    {
        type S = PK_MESH_find_defects_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let mut n = -1;
            let mut p: *mut PK_MESH_defect_details_t = std::ptr::null_mut();
            unsafe { PK_MESH_find_defects(mesh, o as *const _ as *mut _, &mut n, &mut p) }
        };
        if versions {
            version_check(&mut r, "MESH_find_defects", &d, 3, &mut call);
            // v1 = {ver, max_defects, have_tolerance, tolerance, mtopol@0x18, mfacet@0x1c, self_ints@0x20,
            // foldovers@0x24, max_foldover_angle@0x28, disjoint@0x30}; v2 adds mvertex@0x20 (FUN_18036bb40).
            let mut v1 = [0u8; 0x70];
            let mut v2 = [0u8; 0x70];
            unsafe {
                let w = |b: &mut [u8; 0x70], off: usize, v: c_int| {
                    (b.as_mut_ptr().add(off) as *mut c_int).write_unaligned(v)
                };
                let wd = |b: &mut [u8; 0x70], off: usize, v: f64| {
                    (b.as_mut_ptr().add(off) as *mut f64).write_unaligned(v)
                };
                w(&mut v1, 0, 1);
                wd(&mut v1, 0x10, 1e-8);
                w(&mut v1, 0x18, PK_MTOPOL_find_defect_yes_c);
                w(&mut v1, 0x1c, PK_MFACET_find_defect_all_c);
                w(&mut v1, 0x20, PK_MESH_find_self_int_all_c);
                w(&mut v1, 0x24, PK_MESH_find_foldover_no_c);
                wd(&mut v1, 0x28, 0.2);
                w(&mut v1, 0x30, PK_MESH_find_disjoint_no_c);
                w(&mut v2, 0, 2);
                wd(&mut v2, 0x10, 1e-8);
                w(&mut v2, 0x18, PK_MTOPOL_find_defect_yes_c);
                w(&mut v2, 0x1c, PK_MFACET_find_defect_all_c);
                w(&mut v2, 0x20, PK_MVERTEX_find_defect_all_c);
                w(&mut v2, 0x24, PK_MESH_find_self_int_all_c);
                w(&mut v2, 0x28, PK_MESH_find_foldover_no_c);
                wd(&mut v2, 0x30, 0.2);
                w(&mut v2, 0x38, PK_MESH_find_disjoint_no_c);
            }
            let raw = |b: &[u8; 0x70]| {
                let mut n = -1;
                let mut p: *mut PK_MESH_defect_details_t = std::ptr::null_mut();
                unsafe { PK_MESH_find_defects(mesh, b.as_ptr() as *mut _, &mut n, &mut p) }
            };
            r.check(
                "MESH_find_defects v1 buffer (v1 layout)",
                raw(&v1),
                &[0],
                "",
            );
            r.check(
                "MESH_find_defects v2 buffer (v2 layout)",
                raw(&v2),
                &[0],
                "",
            );
            let mut v1bad = v1;
            unsafe { (v1bad.as_mut_ptr().add(0x30) as *mut c_int).write_unaligned(BAD_TOKEN) };
            r.check(
                "MESH_find_defects v1 disjoint@0x30 garbage",
                raw(&v1bad),
                &[5014],
                "v1 offset",
            );
            let mut v2bad = v2;
            unsafe { (v2bad.as_mut_ptr().add(0x38) as *mut c_int).write_unaligned(BAD_TOKEN) };
            r.check(
                "MESH_find_defects v2 disjoint@0x38 garbage",
                raw(&v2bad),
                &[5014],
                "v2 offset",
            );
        }
        if fields {
            field_check(
                &mut r,
                "MESH_find_defects",
                &d,
                "max_defects",
                offset_of!(S, max_defects),
                -1 as c_int,
                &[907],
                &mut call,
            );
            field_check(
                &mut r,
                "MESH_find_defects",
                &d,
                "have_tolerance",
                offset_of!(S, have_tolerance),
                NL,
                &[908],
                &mut call,
            );
            let t = offset_of!(S, defect_tolerances);
            for (f, off) in [
                (
                    "have_degen_tolerance",
                    t + offset_of!(PK_MESH_defect_tolerances_t, have_degen_tolerance),
                ),
                (
                    "have_slit_tolerance",
                    t + offset_of!(PK_MESH_defect_tolerances_t, have_slit_tolerance),
                ),
                (
                    "have_height_tolerance",
                    t + offset_of!(PK_MESH_defect_tolerances_t, have_height_tolerance),
                ),
            ] {
                field_check(
                    &mut r,
                    "MESH_find_defects",
                    &d,
                    f,
                    off,
                    NL,
                    &[908],
                    &mut call,
                );
            }
            for (f, off) in [
                ("mtopol_find_defects", offset_of!(S, mtopol_find_defects)),
                ("mfacet_find_defects", offset_of!(S, mfacet_find_defects)),
                ("mvertex_find_defects", offset_of!(S, mvertex_find_defects)),
                ("mesh_find_self_ints", offset_of!(S, mesh_find_self_ints)),
                ("mesh_find_foldovers", offset_of!(S, mesh_find_foldovers)),
                ("mesh_find_disjoint", offset_of!(S, mesh_find_disjoint)),
            ] {
                field_check(
                    &mut r,
                    "MESH_find_defects",
                    &d,
                    f,
                    off,
                    BAD_TOKEN,
                    &[5014],
                    &mut call,
                );
            }
            field_check(
                &mut r,
                "MESH_find_defects",
                &d,
                "max_foldover_angle",
                offset_of!(S, max_foldover_angle),
                3.0f64,
                &[1],
                &mut call,
            );
            for off in [9usize, 0xf, 0x19, 0x29, 0x39, 0x5c, 0x6c] {
                field_check(
                    &mut r,
                    "MESH_find_defects",
                    &d,
                    "padding",
                    off,
                    0xABu8,
                    &[0],
                    &mut call,
                );
            }
        }
    }

    // PK_MESH_fix_defects_o_t (orphan mesh: the header says attached meshes are not supported)
    {
        type S = PK_MESH_fix_defects_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let m = orphan();
            let mut res = 0;
            let mut n = -1;
            let mut pm: *mut PK_MESH_t = std::ptr::null_mut();
            let mut pd: *mut PK_MESH_defect_array_t = std::ptr::null_mut();
            let rc = unsafe {
                PK_MESH_fix_defects(
                    m,
                    o as *const _ as *mut _,
                    &mut res,
                    &mut n,
                    &mut pm,
                    &mut pd,
                )
            };
            if rc == 0 {
                println!("     (fix_defects result={res} n_resultant={n})");
            }
            rc
        };
        if versions {
            version_check(&mut r, "MESH_fix_defects", &d, 2, &mut call);
            // v1 = {ver, have_tolerance@4, tolerance@8, mfacet_fix@0x10, self_ints@0x14, foldovers@0x18,
            // max_foldover_angle@0x20, disjoint@0x28} (inline migration in the wrapper).
            let mut v1 = [0u8; 0x60];
            unsafe {
                let w = |b: &mut [u8; 0x60], off: usize, v: c_int| {
                    (b.as_mut_ptr().add(off) as *mut c_int).write_unaligned(v)
                };
                w(&mut v1, 0, 1);
                (v1.as_mut_ptr().add(8) as *mut f64).write_unaligned(1e-8);
                w(&mut v1, 0x10, PK_MFACET_fix_defect_degen_c);
                w(&mut v1, 0x14, PK_MESH_fix_self_int_all_c);
                w(&mut v1, 0x18, PK_MESH_fix_foldover_none_c);
                (v1.as_mut_ptr().add(0x20) as *mut f64).write_unaligned(0.2);
                w(&mut v1, 0x28, PK_MESH_fix_disjoint_no_c);
            }
            let raw = |b: &[u8; 0x60]| {
                let m = orphan();
                let mut res = 0;
                let mut n = -1;
                let mut pm: *mut PK_MESH_t = std::ptr::null_mut();
                let mut pd: *mut PK_MESH_defect_array_t = std::ptr::null_mut();
                unsafe {
                    PK_MESH_fix_defects(m, b.as_ptr() as *mut _, &mut res, &mut n, &mut pm, &mut pd)
                }
            };
            r.check("MESH_fix_defects v1 buffer (v1 layout)", raw(&v1), &[0], "");
            let mut v1bad = v1;
            unsafe { (v1bad.as_mut_ptr().add(0x28) as *mut c_int).write_unaligned(BAD_TOKEN) };
            r.check(
                "MESH_fix_defects v1 disjoint@0x28 garbage",
                raw(&v1bad),
                &[5014],
                "v1 offset",
            );
            let a = |o: &S| {
                let mut res = 0;
                let mut n = -1;
                let mut pm: *mut PK_MESH_t = std::ptr::null_mut();
                let mut pd: *mut PK_MESH_defect_array_t = std::ptr::null_mut();
                unsafe {
                    PK_MESH_fix_defects(
                        mesh,
                        o as *const _ as *mut _,
                        &mut res,
                        &mut n,
                        &mut pm,
                        &mut pd,
                    )
                }
            };
            r.check(
                "MESH_fix_defects on attached mesh",
                a(&d),
                &[34],
                "is_attached",
            );
        }
        if fields {
            field_check(
                &mut r,
                "MESH_fix_defects",
                &d,
                "have_tolerance",
                offset_of!(S, have_tolerance),
                NL,
                &[908],
                &mut call,
            );
            let t = offset_of!(S, defect_tolerances);
            for (f, off) in [
                (
                    "have_degen_tolerance",
                    t + offset_of!(PK_MESH_defect_tolerances_t, have_degen_tolerance),
                ),
                (
                    "have_slit_tolerance",
                    t + offset_of!(PK_MESH_defect_tolerances_t, have_slit_tolerance),
                ),
                (
                    "have_height_tolerance",
                    t + offset_of!(PK_MESH_defect_tolerances_t, have_height_tolerance),
                ),
            ] {
                field_check(
                    &mut r,
                    "MESH_fix_defects",
                    &d,
                    f,
                    off,
                    NL,
                    &[908],
                    &mut call,
                );
            }
            for (f, off) in [
                ("mfacet_fix_defects", offset_of!(S, mfacet_fix_defects)),
                ("mesh_fix_self_ints", offset_of!(S, mesh_fix_self_ints)),
                ("mesh_fix_foldovers", offset_of!(S, mesh_fix_foldovers)),
                ("mesh_fix_disjoint", offset_of!(S, mesh_fix_disjoint)),
            ] {
                field_check(
                    &mut r,
                    "MESH_fix_defects",
                    &d,
                    f,
                    off,
                    BAD_TOKEN,
                    &[5014],
                    &mut call,
                );
            }
            field_check(
                &mut r,
                "MESH_fix_defects",
                &d,
                "max_foldover_angle",
                offset_of!(S, max_foldover_angle),
                3.0f64,
                &[1],
                &mut call,
            );
        }
    }

    // PK_MFACET_find_perimeters_o_t
    {
        type S = PK_MFACET_find_perimeters_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let mut res = [0u64; 64];
            let mut mf = mfacets.clone();
            let rc = unsafe {
                PK_MFACET_find_perimeters(
                    mf.len() as c_int,
                    mf.as_mut_ptr(),
                    o as *const _ as *mut _,
                    res.as_mut_ptr() as *mut PK_MFACET_find_perimeters_r_t,
                )
            };
            if rc == 0 {
                println!("     (find_perimeters r[0..6]={:x?})", &res[..6]);
            }
            rc
        };
        if versions {
            version_check(&mut r, "MFACET_find_perimeters", &d, 4, &mut call);
            for v in 1..=3 {
                r.check(
                    &format!("MFACET_find_perimeters v{v} (migrated)"),
                    call(&poke(&d, 0, v as c_int)),
                    &[0],
                    "",
                );
            }
        }
        if fields {
            field_check(
                &mut r,
                "MFACET_find_perimeters",
                &d,
                "want_plines",
                offset_of!(S, want_plines),
                NL,
                &[908],
                &mut call,
            );
            field_check(
                &mut r,
                "MFACET_find_perimeters",
                &d,
                "select_mtopol_class",
                offset_of!(S, select_mtopol_class),
                BAD_TOKEN,
                &[5002],
                &mut call,
            );
            field_check(
                &mut r,
                "MFACET_find_perimeters",
                &d,
                "select_wire_mfins",
                offset_of!(S, select_wire_mfins),
                BAD_TOKEN,
                &[5000],
                &mut call,
            );
            field_check(
                &mut r,
                "MFACET_find_perimeters",
                &d,
                "min_n_mfacets",
                offset_of!(S, min_n_mfacets),
                -1 as c_int,
                &[39],
                &mut call,
            );
            for off in [5usize, 6, 7, 0xc, 0x2c] {
                field_check(
                    &mut r,
                    "MFACET_find_perimeters",
                    &d,
                    "padding",
                    off,
                    0xABu8,
                    &[0],
                    &mut call,
                );
            }
        }
    }

    // PK_MESH_imprint_vectors_o_t: imprint the centroid of the face mesh's first mfacet
    {
        type S = PK_MESH_imprint_vectors_o_t;
        let d = S::default();
        let mut pts: [PK_VECTOR_t; 3] = [[0.0; 3]; 3];
        let rc = unsafe { PK_MFACET_ask_positions(mfacets[0], &mut pts) };
        let c: PK_VECTOR_t = [
            (pts[0][0] + pts[1][0] + pts[2][0]) / 3.0,
            (pts[0][1] + pts[1][1] + pts[2][1]) / 3.0,
            (pts[0][2] + pts[1][2] + pts[2][2]) / 3.0,
        ];
        println!("  (mfacet positions rc={rc} {pts:?} centroid {c:?})");
        let mut call = |o: &S| {
            let m = orphan();
            let mut v = [c];
            let mut res_mesh = 0;
            let mut mvx = [PK_MTOPOL_t { words: [0, 0] }; 1];
            let mut res = [0u64; 64];
            let rc = unsafe {
                PK_MESH_imprint_vectors(
                    m,
                    1,
                    v.as_mut_ptr(),
                    o as *const _ as *mut _,
                    &mut res_mesh,
                    mvx.as_mut_ptr(),
                    res.as_mut_ptr() as *mut PK_MESH_imprint_vectors_r_t,
                )
            };
            if rc == 0 {
                println!(
                    "     (imprint_vectors mesh {m} -> {res_mesh} nf={:?} mvx={:?} r[0..4]={:x?})",
                    n_mfacets(res_mesh),
                    mvx[0].words,
                    &res[..4]
                );
            }
            rc
        };
        if versions {
            version_check(&mut r, "MESH_imprint_vectors", &d, 2, &mut call);
            r.check(
                "MESH_imprint_vectors v1 (migrated)",
                call(&poke(&d, 0, 1 as c_int)),
                &[0],
                "",
            );
        }
        if fields {
            field_check(
                &mut r,
                "MESH_imprint_vectors",
                &d,
                "want_mvertices",
                offset_of!(S, want_mvertices),
                NL,
                &[908],
                &mut call,
            );
            field_check(
                &mut r,
                "MESH_imprint_vectors",
                &d,
                "map_mfacets",
                offset_of!(S, map_mfacets),
                BAD_TOKEN,
                &[5014],
                &mut call,
            );
            for off in [9usize, 0xa, 0xb] {
                field_check(
                    &mut r,
                    "MESH_imprint_vectors",
                    &d,
                    "padding",
                    off,
                    0xABu8,
                    &[0],
                    &mut call,
                );
            }
        }
    }

    // PK_MESH_fill_holes_o_t: v1 only (any other version, or NULL, takes the null-write path)
    {
        type S = PK_MESH_fill_holes_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let m = orphan();
            let mut filled = 0;
            let rc = unsafe { PK_MESH_fill_holes(m, o as *const _ as *mut _, &mut filled) };
            println!(
                "     (fill_holes mesh {m} nf={:?} rc={rc} -> {filled} nf={:?})",
                n_mfacets(m),
                if filled != 0 {
                    n_mfacets(filled)
                } else {
                    (0, 0)
                }
            );
            rc
        };
        if versions {
            let rc = call(&d);
            r.check("MESH_fill_holes v1 defaults", rc, &[0], "");
        }
        if fields {
            field_check(
                &mut r,
                "MESH_fill_holes (not validated)",
                &d,
                "fill_shape",
                offset_of!(S, fill_shape),
                BAD_TOKEN,
                &[0],
                &mut call,
            );
            field_check(
                &mut r,
                "MESH_fill_holes (not validated)",
                &d,
                "max_n_mfins",
                offset_of!(S, max_n_mfins),
                -1 as c_int,
                &[0],
                &mut call,
            );
        }
    }

    // PK_BODY_pick_topols_o_t on the classic block: ray down the z axis
    {
        type S = PK_BODY_pick_topols_o_t;
        let d = S::default();
        let mut call = |o: &S| {
            let mut bodies = [block.tag()];
            let mut ray = PK_AXIS1_sf_t {
                location: [0.0, 0.0, 100.0],
                axis: [0.0, 0.0, -1.0],
            };
            let mut picked = PK_BODY_pick_topols_r_t::default();
            let rc = unsafe {
                PK_BODY_pick_topols(
                    1,
                    bodies.as_mut_ptr(),
                    std::ptr::null_mut(),
                    &mut ray,
                    o as *const _ as *mut _,
                    &mut picked,
                )
            };
            if rc == 0 {
                let f = if picked.n_faces > 0 {
                    unsafe { *picked.faces }
                } else {
                    PK_BODY_picked_topol_t {
                        entity: 0,
                        occurrence: 0,
                        intersect: [0.0; 3],
                        distance: 0.0,
                    }
                };
                println!(
                    "     (pick n_faces={} e={} n_edges={} e={} n_vx={} e={} first={:?})",
                    picked.n_faces,
                    picked.e_faces,
                    picked.n_edges,
                    picked.e_edges,
                    picked.n_vertices,
                    picked.e_vertices,
                    f
                );
                unsafe { PK_BODY_pick_topols_r_f(&mut picked) };
            }
            rc
        };
        if versions {
            version_check(&mut r, "BODY_pick_topols", &d, 7, &mut call);
        }
        if fields {
            for (f, off) in [
                ("near_plane", offset_of!(S, near_plane)),
                ("far_plane", offset_of!(S, far_plane)),
                ("is_curve_chord_tol", offset_of!(S, is_curve_chord_tol)),
                ("is_curve_chord_max", offset_of!(S, is_curve_chord_max)),
                ("is_curve_chord_ang", offset_of!(S, is_curve_chord_ang)),
                ("ignore_back_faces", offset_of!(S, ignore_back_faces)),
                (
                    "ignore_excess_entities",
                    offset_of!(S, ignore_excess_entities),
                ),
            ] {
                field_check(
                    &mut r,
                    "BODY_pick_topols",
                    &d,
                    f,
                    off,
                    NL,
                    &[5145],
                    &mut call,
                );
            }
            for (f, off) in [
                ("pick_approx", offset_of!(S, pick_approx)),
                ("ray_scale_factor", offset_of!(S, ray_scale_factor)),
            ] {
                field_check(
                    &mut r,
                    "BODY_pick_topols",
                    &d,
                    f,
                    off,
                    BAD_TOKEN,
                    &[5145],
                    &mut call,
                );
            }
            // controls: garbage in padding bytes must be ignored
            for off in [0x17usize, 0x21, 0x31, 0x42, 0x47, 0x8c, 0x9c, 0xc4, 0xdc] {
                field_check(
                    &mut r,
                    "BODY_pick_topols",
                    &d,
                    "padding",
                    off,
                    0xABu8,
                    &[0],
                    &mut call,
                );
            }
        }
    }
    println!("\n=== probe: {} ok, {} mismatches", r.ok, r.bad);
}

fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    match arg.as_str() {
        "explore" => explore(),
        "effects" => effects(),
        "versions" => probes(true, false),
        "fields" => probes(false, true),
        _ => {
            explore();
            probes(true, true);
        }
    }
}
