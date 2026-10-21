//! Runtime tests for the mesh (convergent-modelling) option structs in
//! `parasolid_sys::mesh_options` and the 8-byte `PK_MTOPOL_t` handle (parasolid-re
//! B-R45-1; evidence in `artifacts/rs/20261021T031231Z-prs-mesh/`).
//!
//! Each test calls a real entry point through its struct with defaults (rc 0 and
//! a plausible result), then changes one field and checks its visible effect.
//! The old declarations had no `o_t_version`, so the kernel read the first field
//! as the version and every call failed with 5022, or read the wrong offsets.
//!
//! Operands: a faceted copy of a block (6 face meshes of 2 mfacets each) or of a
//! cylinder (its side mesh is an open tube of 32 mfacets), and orphan copies of
//! those meshes made with `PK_ENTITY_copy`.

use parasolid::*;
use parasolid_sys::*;
use std::error::Error;
use std::os::raw::{c_int, c_void};

pub type R = Result<(), Box<dyn Error>>;

fn session() -> Result<Session, Box<dyn Error>> {
    let s = Session::start(SessionConfig::new().check_arguments(true))?;
    let rc = unsafe { PK_SESSION_set_facet_geometry(PK_facet_geometry_all_c) };
    assert_eq!(rc, 0, "PK_SESSION_set_facet_geometry");
    let mut fg = 0;
    let rc = unsafe { PK_SESSION_ask_facet_geometry(&mut fg) };
    assert_eq!(
        (rc, fg),
        (0, PK_facet_geometry_all_c),
        "facet geometry enabled"
    );
    Ok(s)
}

fn rc_ok(rc: PK_ERROR_code_t, what: &str) -> R {
    if rc != 0 {
        return Err(format!("{what} returned PK error {rc}").into());
    }
    Ok(())
}

fn facet_copy(body: &Body, o: &PK_BODY_make_facet_body_o_t) -> Result<PK_BODY_t, Box<dyn Error>> {
    let mut copy = 0;
    let mut tr = PK_TOPOL_track_r_t::default();
    let mut red = PK_TOPOL_track_r_t::default();
    let rc = unsafe {
        PK_BODY_make_facet_body(
            body.tag(),
            PK_ENTITY_null,
            o as *const _ as *mut _,
            &mut copy,
            &mut tr,
            &mut red,
        )
    };
    rc_ok(rc, "PK_BODY_make_facet_body")?;
    Ok(copy)
}

fn tags<T: Copy>(n: c_int, p: *mut T) -> Vec<T> {
    if n <= 0 || p.is_null() {
        return Vec::new();
    }
    let v = unsafe { std::slice::from_raw_parts(p, n as usize).to_vec() };
    unsafe { PK_MEMORY_free(p as *mut c_void) };
    v
}

fn faces_of(body: PK_BODY_t) -> Result<Vec<PK_FACE_t>, Box<dyn Error>> {
    let (mut n, mut p) = (0, std::ptr::null_mut());
    rc_ok(
        unsafe { PK_BODY_ask_faces(body, &mut n, &mut p) },
        "PK_BODY_ask_faces",
    )?;
    Ok(tags(n, p))
}

fn fins_of(body: PK_BODY_t) -> Result<Vec<PK_FIN_t>, Box<dyn Error>> {
    let (mut n, mut p) = (0, std::ptr::null_mut());
    rc_ok(
        unsafe { PK_BODY_ask_fins(body, &mut n, &mut p) },
        "PK_BODY_ask_fins",
    )?;
    Ok(tags(n, p))
}

fn mesh_of(face: PK_FACE_t) -> Result<PK_MESH_t, Box<dyn Error>> {
    let mut s = 0;
    rc_ok(
        unsafe { PK_FACE_ask_surf(face, &mut s) },
        "PK_FACE_ask_surf",
    )?;
    Ok(s)
}

fn class_of(e: PK_ENTITY_t) -> PK_CLASS_t {
    let mut c = 0;
    unsafe { PK_ENTITY_ask_class(e, &mut c) };
    c
}

fn n_mfacets(mesh: PK_MESH_t) -> Result<c_int, Box<dyn Error>> {
    let mut n = 0;
    rc_ok(
        unsafe { PK_MESH_ask_n_mfacets(mesh, &mut n) },
        "PK_MESH_ask_n_mfacets",
    )?;
    Ok(n)
}

fn copy_of(e: PK_ENTITY_t) -> Result<PK_ENTITY_t, Box<dyn Error>> {
    let mut c = 0;
    rc_ok(unsafe { PK_ENTITY_copy(e, &mut c) }, "PK_ENTITY_copy")?;
    Ok(c)
}

fn position(mv: PK_MVERTEX_t) -> Result<[f64; 3], Box<dyn Error>> {
    let mut p: PK_VECTOR_t = [f64::NAN; 3];
    rc_ok(
        unsafe { PK_MVERTEX_ask_position(mv, &mut p) },
        "PK_MVERTEX_ask_position",
    )?;
    Ok(p)
}

struct FinMtopols {
    mvertices: Vec<PK_MVERTEX_t>,
    mvertices_null: bool,
    n_mvertices: c_int,
    mfins: Vec<PK_MFIN_t>,
}

fn fin_mtopols(fin: PK_FIN_t, o: &PK_FIN_find_mtopols_o_t) -> Result<FinMtopols, Box<dyn Error>> {
    let (mut nv, mut nf) = (0, 0);
    let mut pv: *mut PK_MVERTEX_t = std::ptr::null_mut();
    let mut pf: *mut PK_MFIN_t = std::ptr::null_mut();
    let mut sense: PK_LOGICAL_t = 0;
    let rc = unsafe {
        PK_FIN_find_mtopols(
            fin,
            o as *const _ as *mut _,
            &mut nv,
            &mut pv,
            &mut nf,
            &mut pf,
            &mut sense,
        )
    };
    rc_ok(rc, "PK_FIN_find_mtopols")?;
    let mvertices_null = pv.is_null();
    Ok(FinMtopols {
        mvertices: tags(nv, pv),
        mvertices_null,
        n_mvertices: nv,
        mfins: tags(nf, pf),
    })
}

fn sharp_mvxs(
    mesh: PK_MESH_t,
    o: &PK_MESH_find_sharp_mvxs_o_t,
) -> Result<(c_int, bool, Vec<PK_MVERTEX_t>), Box<dyn Error>> {
    let mut n = -1;
    let mut p: *mut PK_MVERTEX_t = std::ptr::null_mut();
    rc_ok(
        unsafe { PK_MESH_find_sharp_mvxs(mesh, o, &mut n, &mut p) },
        "PK_MESH_find_sharp_mvxs",
    )?;
    let null = p.is_null();
    Ok((n, null, tags(n, p)))
}

fn sharp_mfins(mesh: PK_MESH_t, o: &PK_MESH_find_sharp_mfins_o_t) -> Result<c_int, Box<dyn Error>> {
    let mut n = -1;
    let mut p: *mut PK_MFIN_t = std::ptr::null_mut();
    rc_ok(
        unsafe { PK_MESH_find_sharp_mfins(mesh, o, &mut n, &mut p) },
        "PK_MESH_find_sharp_mfins",
    )?;
    let _ = tags(n, p);
    Ok(n)
}

/// The mfacets touching the mvertices of one fin (a block-face mesh's 2 mfacets).
fn mfacets_near_fin(fin: PK_FIN_t) -> Result<Vec<PK_MFACET_t>, Box<dyn Error>> {
    let mut out: Vec<PK_MFACET_t> = Vec::new();
    for mv in fin_mtopols(fin, &PK_FIN_find_mtopols_o_t::default())?.mvertices {
        let (mut n, mut p) = (0, std::ptr::null_mut());
        rc_ok(
            unsafe { PK_MVERTEX_ask_mfacets(mv, &mut n, &mut p) },
            "PK_MVERTEX_ask_mfacets",
        )?;
        for f in tags::<PK_MFACET_t>(n, p) {
            if !out.contains(&f) {
                out.push(f);
            }
        }
    }
    Ok(out)
}

fn near(a: [f64; 3], b: [f64; 3]) -> bool {
    (0..3).all(|i| (a[i] - b[i]).abs() < 1e-9)
}

// -----------------------------------------------------------------------------

/// Each struct's current version is accepted and the next one is refused with
/// `PK_ERROR_o_t_version_unknown` (5022). With the old declarations (no
/// `o_t_version`) every one of these calls returned 5022.
pub fn mesh_option_versions() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let fb = facet_copy(&block, &PK_BODY_make_facet_body_o_t::default())?;
    let mesh = mesh_of(faces_of(fb)?[0])?;
    let fin = fins_of(fb)?[0];
    let mut seen = 0;
    let mut both = |name: &str, rc_cur: i32, rc_next: i32| {
        assert_eq!(rc_cur, 0, "{name}: current version must be accepted");
        assert_eq!(
            rc_next, 5022,
            "{name}: version + 1 must be o_t_version_unknown"
        );
        seen += 1;
    };
    // make_facet_body (v2)
    {
        let mut o = PK_BODY_make_facet_body_o_t::default();
        let call = |o: &mut PK_BODY_make_facet_body_o_t| {
            let mut c = 0;
            let mut t = PK_TOPOL_track_r_t::default();
            let mut r = PK_TOPOL_track_r_t::default();
            unsafe { PK_BODY_make_facet_body(block.tag(), 0, o, &mut c, &mut t, &mut r) }
        };
        let a = call(&mut o);
        o.o_t_version += 1;
        both("PK_BODY_make_facet_body_o_t v2", a, call(&mut o));
    }
    // find_sharp_mvxs (v1) / find_sharp_mfins (v2)
    {
        let mut o = PK_MESH_find_sharp_mvxs_o_t::default();
        let (mut n, mut p) = (0, std::ptr::null_mut());
        let a = unsafe { PK_MESH_find_sharp_mvxs(mesh, &o, &mut n, &mut p) };
        o.o_t_version += 1;
        both("PK_MESH_find_sharp_mvxs_o_t v1", a, unsafe {
            PK_MESH_find_sharp_mvxs(mesh, &o, &mut n, &mut p)
        });
        let mut o = PK_MESH_find_sharp_mfins_o_t::default();
        let a = unsafe { PK_MESH_find_sharp_mfins(mesh, &o, &mut n, &mut p) };
        o.o_t_version += 1;
        both("PK_MESH_find_sharp_mfins_o_t v2", a, unsafe {
            PK_MESH_find_sharp_mfins(mesh, &o, &mut n, &mut p)
        });
    }
    // FIN_find_mtopols (v1)
    {
        let mut o = PK_FIN_find_mtopols_o_t::default();
        let call = |o: &mut PK_FIN_find_mtopols_o_t| {
            let (mut nv, mut nf) = (0, 0);
            let (mut pv, mut pf) = (std::ptr::null_mut(), std::ptr::null_mut());
            let mut s: PK_LOGICAL_t = 0;
            unsafe { PK_FIN_find_mtopols(fin, o, &mut nv, &mut pv, &mut nf, &mut pf, &mut s) }
        };
        let a = call(&mut o);
        o.o_t_version += 1;
        both("PK_FIN_find_mtopols_o_t v1", a, call(&mut o));
    }
    // find_defects (v3)
    {
        let mut o = PK_MESH_find_defects_o_t::default();
        let call = |o: &mut PK_MESH_find_defects_o_t| {
            let (mut n, mut p) = (0, std::ptr::null_mut());
            unsafe { PK_MESH_find_defects(mesh, o, &mut n, &mut p) }
        };
        let a = call(&mut o);
        o.o_t_version += 1;
        both("PK_MESH_find_defects_o_t v3", a, call(&mut o));
    }
    // fix_defects (v2), on an orphan copy
    {
        let mut o = PK_MESH_fix_defects_o_t::default();
        let call = |o: &mut PK_MESH_fix_defects_o_t| -> i32 {
            let m = copy_of(mesh).unwrap();
            let (mut res, mut n) = (0, 0);
            let (mut pm, mut pd) = (std::ptr::null_mut(), std::ptr::null_mut());
            unsafe { PK_MESH_fix_defects(m, o, &mut res, &mut n, &mut pm, &mut pd) }
        };
        let a = call(&mut o);
        o.o_t_version += 1;
        both("PK_MESH_fix_defects_o_t v2", a, call(&mut o));
    }
    // store_normals (v1), on an orphan copy
    {
        let mut o = PK_MESH_store_normals_o_t::default();
        let a = unsafe { PK_MESH_store_normals(copy_of(mesh)?, &o) };
        o.o_t_version += 1;
        both("PK_MESH_store_normals_o_t v1", a, unsafe {
            PK_MESH_store_normals(copy_of(mesh)?, &o)
        });
    }
    // make_bodies (v1) / make_surf_trimmed (v1), on orphan copies
    {
        let mut o = PK_MESH_make_bodies_o_t::default();
        let (mut n, mut p) = (0, std::ptr::null_mut());
        let a = unsafe { PK_MESH_make_bodies(copy_of(mesh)?, &o, &mut n, &mut p) };
        o.o_t_version += 1;
        both("PK_MESH_make_bodies_o_t v1", a, unsafe {
            PK_MESH_make_bodies(copy_of(mesh)?, &o, &mut n, &mut p)
        });
        let mut o = PK_MESH_make_surf_trimmed_o_t::default();
        let call = |o: &PK_MESH_make_surf_trimmed_o_t| -> i32 {
            let (mut surf, mut sense) = (0, 0u8);
            let mut trim = [0u64; 32];
            unsafe {
                PK_MESH_make_surf_trimmed(
                    copy_of(mesh).unwrap(),
                    o,
                    &mut surf,
                    &mut sense,
                    trim.as_mut_ptr() as *mut c_void,
                )
            }
        };
        let a = call(&o);
        o.o_t_version += 1;
        both("PK_MESH_make_surf_trimmed_o_t v1", a, call(&o));
    }
    // MTOPOL_make_meshes (v4), MFACET_find_perimeters (v4)
    {
        let mf = mfacets_near_fin(fin)?;
        let mut o = PK_MTOPOL_make_meshes_o_t::default();
        let call = |o: &PK_MTOPOL_make_meshes_o_t| {
            let (mut n, mut p) = (0, std::ptr::null_mut());
            unsafe { PK_MTOPOL_make_meshes(mf.len() as c_int, mf.as_ptr(), o, &mut n, &mut p) }
        };
        let a = call(&o);
        o.o_t_version += 1;
        both("PK_MTOPOL_make_meshes_o_t v4", a, call(&o));
        let mut o = PK_MFACET_find_perimeters_o_t::default();
        let call = |o: &mut PK_MFACET_find_perimeters_o_t| {
            // PK_MFACET_find_perimeters_r_t is not verified: give it a large buffer.
            let mut res = [0u64; 64];
            let mut m = mf.clone();
            unsafe {
                PK_MFACET_find_perimeters(
                    m.len() as c_int,
                    m.as_mut_ptr(),
                    o,
                    res.as_mut_ptr() as *mut _,
                )
            }
        };
        let a = call(&mut o);
        o.o_t_version += 1;
        both("PK_MFACET_find_perimeters_o_t v4", a, call(&mut o));
    }
    // MESH_imprint_vectors (v2), on an orphan copy
    {
        let mut o = PK_MESH_imprint_vectors_o_t::default();
        let call = |o: &mut PK_MESH_imprint_vectors_o_t| -> i32 {
            let mut v = [[1.0f64, 2.0, 30.0]];
            let mut out = 0;
            let mut mvx = [PK_MTOPOL_t { words: [0, 0] }];
            let mut res = [0u64; 64];
            let mesh_top = mesh_of(faces_of(fb).unwrap()[0]).unwrap();
            unsafe {
                PK_MESH_imprint_vectors(
                    copy_of(mesh_top).unwrap(),
                    1,
                    v.as_mut_ptr(),
                    o,
                    &mut out,
                    mvx.as_mut_ptr(),
                    res.as_mut_ptr() as *mut _,
                )
            }
        };
        let a = call(&mut o);
        o.o_t_version += 1;
        both("PK_MESH_imprint_vectors_o_t v2", a, call(&mut o));
    }
    // PARTITION_ask_facet_geom (v1)
    {
        let part = _s.partitions()?[0].tag();
        let mut o = PK_PARTITION_ask_facet_geom_o_t::default();
        let call = |o: &PK_PARTITION_ask_facet_geom_o_t| {
            let (mut c, mut np, mut ng) = (0, 0, 0);
            let (mut pp, mut pc, mut pg) = (
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            unsafe {
                PK_PARTITION_ask_facet_geom(
                    part, o, &mut c, &mut np, &mut pp, &mut pc, &mut ng, &mut pg,
                )
            }
        };
        let a = call(&o);
        o.o_t_version += 1;
        both("PK_PARTITION_ask_facet_geom_o_t v1", a, call(&o));
    }
    // BODY_pick_topols (v7)
    {
        let mut o = PK_BODY_pick_topols_o_t::default();
        let call = |o: &PK_BODY_pick_topols_o_t| {
            let mut b = [block.tag()];
            let mut ray = PK_AXIS1_sf_t {
                location: [0.0, 0.0, 100.0],
                axis: [0.0, 0.0, -1.0],
            };
            let mut r = PK_BODY_pick_topols_r_t::default();
            let rc = unsafe {
                PK_BODY_pick_topols(
                    1,
                    b.as_mut_ptr(),
                    std::ptr::null_mut(),
                    &mut ray,
                    o as *const _ as *mut _,
                    &mut r,
                )
            };
            if rc == 0 {
                unsafe { PK_BODY_pick_topols_r_f(&mut r) };
            }
            rc
        };
        let a = call(&o);
        o.o_t_version += 1;
        both("PK_BODY_pick_topols_o_t v7", a, call(&o));
    }
    // PK_MESH_fill_holes_o_t is deliberately absent: any version but 1 takes the
    // kernel's null-write path (see its doc comment).
    assert_eq!(seen, 14);
    Ok(())
}

/// `PK_BODY_make_facet_body` (v2): a block's faceted copy has one 2-mfacet mesh
/// per face; `max_facet_width` (@0x8, gated by @0x4) refines it.
pub fn mesh_make_facet_body_block() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let fb = facet_copy(&block, &PK_BODY_make_facet_body_o_t::default())?;
    assert_eq!(class_of(fb), PK_CLASS_body);
    let faces = faces_of(fb)?;
    assert_eq!(faces.len(), 6, "faceted block keeps its 6 faces");
    let mut total = 0;
    for &f in &faces {
        let m = mesh_of(f)?;
        assert_eq!(class_of(m), PK_CLASS_mesh, "face geometry is a mesh");
        total += n_mfacets(m)?;
    }
    assert_eq!(total, 12, "two mfacets per planar face by default");

    let fine = PK_BODY_make_facet_body_o_t {
        is_max_facet_width: PK_LOGICAL_true,
        max_facet_width: 2.0,
        ..Default::default()
    };
    let fb2 = facet_copy(&block, &fine)?;
    let mut total2 = 0;
    for f in faces_of(fb2)? {
        total2 += n_mfacets(mesh_of(f)?)?;
    }
    assert!(
        total2 > 1000,
        "max_facet_width 2 on a 10x20x30 block gives many mfacets, got {total2}"
    );

    // A non-logical in a logical, or a garbage token, is refused at its offset.
    let mut bad = PK_BODY_make_facet_body_o_t::default();
    bad.track_faces = 2;
    let mut c = 0;
    let (mut t, mut r) = (PK_TOPOL_track_r_t::default(), PK_TOPOL_track_r_t::default());
    assert_eq!(
        unsafe { PK_BODY_make_facet_body(block.tag(), 0, &mut bad, &mut c, &mut t, &mut r) },
        908
    );
    let mut bad = PK_BODY_make_facet_body_o_t {
        partial_conversion: 987654,
        ..Default::default()
    };
    assert_eq!(
        unsafe { PK_BODY_make_facet_body(block.tag(), 0, &mut bad, &mut c, &mut t, &mut r) },
        5014
    );
    Ok(())
}

/// `PK_FIN_find_mtopols` (v1) returns 8-byte mesh-topology handles whose
/// positions are the block's corners; `want_mvertices` = false (@0x4) suppresses
/// the mvertex array. With the old 4-byte `PK_MVERTEX_t` every second element
/// was half a handle.
pub fn mesh_fin_mtopols_8_byte_handles() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let fb = facet_copy(&block, &PK_BODY_make_facet_body_o_t::default())?;
    let mut checked = 0;
    for fin in fins_of(fb)? {
        let r = fin_mtopols(fin, &PK_FIN_find_mtopols_o_t::default())?;
        assert_eq!(r.mvertices.len(), 2, "a block-edge fin spans 2 mvertices");
        assert_eq!(r.mfins.len(), 1, "and one mfin");
        for mv in r.mvertices {
            let p = position(mv)?;
            assert!(
                (p[0].abs() - 5.0).abs() < 1e-9
                    && (p[1].abs() - 10.0).abs() < 1e-9
                    && (p[2] == 0.0 || p[2] == 30.0),
                "mvertex {:?} at {p:?} is not a block corner",
                mv.words
            );
            checked += 1;
        }
    }
    assert_eq!(checked, 48, "24 fins x 2 mvertices");
    let fin = fins_of(fb)?[0];
    let r = fin_mtopols(
        fin,
        &PK_FIN_find_mtopols_o_t {
            want_mvertices: PK_LOGICAL_false,
            ..Default::default()
        },
    )?;
    assert!(
        r.mvertices_null,
        "want_mvertices = false returns no mvertex array"
    );
    assert_eq!(r.n_mvertices, 2, "but still the count");
    assert_eq!(r.mfins.len(), 1);
    Ok(())
}

/// `PK_MESH_store_normals` (v1) with a small `mesh_angle` (@0x8, gated by @0x4)
/// makes every mvertex of a faceted cylinder's side sharp, which
/// `PK_MESH_find_sharp_mvxs` (v1) and `PK_MESH_find_sharp_mfins` (v2) then report.
/// `want_sharp_mvxs` = false (@0x4) returns the count only.
pub fn mesh_store_normals_sharp_features() -> R {
    let _s = session()?;
    let cyl = Body::create_solid_cylinder(5.0, 10.0)?;
    let fc = facet_copy(&cyl, &PK_BODY_make_facet_body_o_t::default())?;
    let mut side = 0;
    for f in faces_of(fc)? {
        let m = mesh_of(f)?;
        if n_mfacets(m)? == 32 {
            side = m;
        }
    }
    assert_ne!(side, 0, "cylinder side mesh has 32 mfacets");
    let (n0, _, _) = sharp_mvxs(side, &PK_MESH_find_sharp_mvxs_o_t::default())?;
    assert_eq!(n0, 0, "a smooth faceted cylinder has no sharp mvertices");
    assert_eq!(
        sharp_mfins(side, &PK_MESH_find_sharp_mfins_o_t::default())?,
        0
    );

    // Adjacent side mfacets differ by 22.5 deg: a 0.1 rad mesh angle splits every normal.
    let sharp = copy_of(side)?;
    let o = PK_MESH_store_normals_o_t {
        have_mesh_angle: PK_LOGICAL_true,
        mesh_angle: 0.1,
        ..Default::default()
    };
    rc_ok(
        unsafe { PK_MESH_store_normals(sharp, &o) },
        "PK_MESH_store_normals",
    )?;
    let (n1, _, mvs) = sharp_mvxs(sharp, &PK_MESH_find_sharp_mvxs_o_t::default())?;
    assert_eq!(n1, 32, "all 32 side mvertices become sharp");
    for mv in &mvs {
        let p = position(*mv)?;
        let r = (p[0] * p[0] + p[1] * p[1]).sqrt();
        assert!(
            (r - 5.0).abs() < 1e-9 && (p[2] == 0.0 || p[2] == 10.0),
            "sharp mvertex at {p:?}"
        );
    }
    assert_eq!(
        sharp_mfins(sharp, &PK_MESH_find_sharp_mfins_o_t::default())?,
        32
    );
    let (n2, null, _) = sharp_mvxs(
        sharp,
        &PK_MESH_find_sharp_mvxs_o_t {
            want_sharp_mvxs: PK_LOGICAL_false,
            ..Default::default()
        },
    )?;
    assert_eq!(
        (n2, null),
        (32, true),
        "want_sharp_mvxs = false: count only"
    );

    // A 1 rad mesh angle (> 22.5 deg) keeps the normals smooth.
    let smooth = copy_of(side)?;
    let o = PK_MESH_store_normals_o_t {
        have_mesh_angle: PK_LOGICAL_true,
        mesh_angle: 1.0,
        ..Default::default()
    };
    rc_ok(
        unsafe { PK_MESH_store_normals(smooth, &o) },
        "PK_MESH_store_normals",
    )?;
    assert_eq!(
        sharp_mvxs(smooth, &PK_MESH_find_sharp_mvxs_o_t::default())?.0,
        0
    );
    Ok(())
}

/// `PK_MESH_find_defects` (v3) and `PK_MESH_fix_defects` (v2) on an orphan square
/// mesh with one mvertex moved onto another: one degenerate mfacet is found
/// (with the 64-byte `PK_MESH_defect_details_t`), `mfacet_find_defects` = no
/// (@0x4c) hides it, and fixing removes it unless `mfacet_fix_defects` = none
/// (@0x40).
pub fn mesh_find_fix_defects_degenerate() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let fb = facet_copy(&block, &PK_BODY_make_facet_body_o_t::default())?;
    let fin = fins_of(fb)?[0];
    let mvs = fin_mtopols(fin, &PK_FIN_find_mtopols_o_t::default())?.mvertices;
    let mesh = mvs[0].words[0];
    assert_eq!(
        class_of(mesh),
        PK_CLASS_mesh,
        "an mvertex handle's first word is its mesh"
    );
    let sq = copy_of(mesh)?;
    // The copy keeps mvertex indices; only the mesh word changes.
    let a = PK_MTOPOL_t {
        words: [sq, mvs[0].words[1]],
    };
    let mut b = [PK_MTOPOL_t {
        words: [sq, mvs[1].words[1]],
    }];
    assert!(near(position(a)?, position(mvs[0])?) && near(position(b[0])?, position(mvs[1])?));

    let (mut n, mut p) = (-1, std::ptr::null_mut());
    rc_ok(
        unsafe {
            PK_MESH_find_defects(sq, &mut PK_MESH_find_defects_o_t::default(), &mut n, &mut p)
        },
        "find_defects",
    )?;
    assert_eq!(n, 0, "the copied square is clean");

    let mut to = [position(a)?];
    rc_ok(
        unsafe {
            PK_MVERTEX_set_positions(1, b.as_mut_ptr(), to.as_mut_ptr(), std::ptr::null_mut())
        },
        "PK_MVERTEX_set_positions",
    )?;

    let (mut n, mut p) = (-1, std::ptr::null_mut::<PK_MESH_defect_details_t>());
    rc_ok(
        unsafe {
            PK_MESH_find_defects(sq, &mut PK_MESH_find_defects_o_t::default(), &mut n, &mut p)
        },
        "find_defects",
    )?;
    assert_eq!(n, 1, "one defect");
    let d = unsafe { *p };
    assert_eq!(d.defect, PK_MESH_defect_degen_mfacet_c);
    assert!(
        d.n_mtopols >= 1 && unsafe { (*d.mtopols).words[0] } == sq,
        "the defect names an mtopol of the mesh"
    );

    let mut o = PK_MESH_find_defects_o_t {
        mfacet_find_defects: PK_MFACET_find_defect_no_c,
        ..Default::default()
    };
    let (mut n, mut p) = (-1, std::ptr::null_mut());
    rc_ok(
        unsafe { PK_MESH_find_defects(sq, &mut o, &mut n, &mut p) },
        "find_defects",
    )?;
    assert_eq!(n, 0, "mfacet_find_defects = no skips the degenerate mfacet");

    let fix = |o: &mut PK_MESH_fix_defects_o_t| -> Result<(PK_MESH_fix_result_t, Vec<PK_MESH_t>), Box<dyn Error>> {
        let m = copy_of(sq)?;
        let (mut res, mut n) = (0, 0);
        let (mut pm, mut pd) = (std::ptr::null_mut(), std::ptr::null_mut());
        rc_ok(unsafe { PK_MESH_fix_defects(m, o, &mut res, &mut n, &mut pm, &mut pd) }, "PK_MESH_fix_defects")?;
        Ok((res, tags(n, pm)))
    };
    let (res, meshes) = fix(&mut PK_MESH_fix_defects_o_t::default())?;
    assert_eq!(res, PK_MESH_fix_result_success_c);
    assert_eq!(meshes.len(), 1);
    assert_eq!(n_mfacets(meshes[0])?, 1, "the degenerate mfacet is removed");
    let (_, meshes) = fix(&mut PK_MESH_fix_defects_o_t {
        mfacet_fix_defects: PK_MFACET_fix_defect_none_c,
        ..Default::default()
    })?;
    assert_eq!(
        n_mfacets(meshes[0])?,
        2,
        "mfacet_fix_defects = none keeps it"
    );

    // The header: not implemented for meshes attached to faces.
    let (mut res, mut n) = (0, 0);
    let (mut pm, mut pd) = (std::ptr::null_mut(), std::ptr::null_mut());
    let rc = unsafe {
        PK_MESH_fix_defects(
            mesh,
            &mut PK_MESH_fix_defects_o_t::default(),
            &mut res,
            &mut n,
            &mut pm,
            &mut pd,
        )
    };
    assert_eq!(rc, PK_ERROR_is_attached);
    Ok(())
}

/// `PK_MTOPOL_make_meshes` (v4) turns a face mesh's mfacets into an orphan mesh;
/// `PK_MESH_make_bodies` (v1) turns that into a sheet whose vertex count follows
/// `vertex_angle` (@0x8); `PK_MESH_make_surf_trimmed` (v1) fits a plane to it.
pub fn mesh_make_meshes_bodies_surf() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let fb = facet_copy(&block, &PK_BODY_make_facet_body_o_t::default())?;
    let mf = mfacets_near_fin(fins_of(fb)?[0])?;
    assert_eq!(mf.len(), 2, "the two mfacets of a block face");
    let (mut n, mut p) = (0, std::ptr::null_mut());
    rc_ok(
        unsafe {
            PK_MTOPOL_make_meshes(
                mf.len() as c_int,
                mf.as_ptr(),
                &PK_MTOPOL_make_meshes_o_t::default(),
                &mut n,
                &mut p,
            )
        },
        "PK_MTOPOL_make_meshes",
    )?;
    let meshes: Vec<PK_MESH_t> = tags(n, p);
    assert_eq!(meshes.len(), 1);
    let orphan = meshes[0];
    assert_eq!(class_of(orphan), PK_CLASS_mesh);
    assert_eq!(n_mfacets(orphan)?, 2);

    let bodies_with = |va: f64| -> Result<Body, Box<dyn Error>> {
        let o = PK_MESH_make_bodies_o_t {
            vertex_angle: va,
            ..Default::default()
        };
        let (mut n, mut p) = (0, std::ptr::null_mut());
        rc_ok(
            unsafe { PK_MESH_make_bodies(copy_of(orphan)?, &o, &mut n, &mut p) },
            "PK_MESH_make_bodies",
        )?;
        let b: Vec<PK_BODY_t> = tags(n, p);
        assert_eq!(b.len(), 1);
        Ok(Body::from_tag_checked(b[0])?)
    };
    let b = bodies_with(4.0)?;
    assert_eq!(b.body_type()?, BodyType::Sheet);
    let t = b.topology_summary()?;
    assert_eq!(
        (t.faces, t.edges, t.vertices),
        (1, 1, 0),
        "default vertex_angle 4.0: one ring edge, no vertex"
    );
    let t = bodies_with(0.1)?.topology_summary()?;
    assert_eq!(
        (t.faces, t.edges, t.vertices),
        (1, 4, 4),
        "vertex_angle 0.1: a vertex at each corner"
    );

    let (mut surf, mut sense) = (0, 9u8);
    let mut trim = [0u64; 32];
    rc_ok(
        unsafe {
            PK_MESH_make_surf_trimmed(
                copy_of(orphan)?,
                &PK_MESH_make_surf_trimmed_o_t::default(),
                &mut surf,
                &mut sense,
                trim.as_mut_ptr() as *mut c_void,
            )
        },
        "PK_MESH_make_surf_trimmed",
    )?;
    assert_eq!(class_of(surf), PK_CLASS_plane, "a planar mesh fits a plane");
    assert!(sense <= 1, "sense is a logical");
    let bad = PK_MESH_make_surf_trimmed_o_t {
        fit_normals: 2,
        ..Default::default()
    };
    let rc = unsafe {
        PK_MESH_make_surf_trimmed(
            copy_of(orphan)?,
            &bad,
            &mut surf,
            &mut sense,
            trim.as_mut_ptr() as *mut c_void,
        )
    };
    assert_eq!(rc, 908, "fit_normals is read at @0x28");
    Ok(())
}

/// `PK_MESH_fill_holes` (v1) on the open tube of a faceted cylinder's side:
/// `max_n_mfins` = 0 fills nothing, 1000 fills both ends, and one boundary mfin in
/// `input_holes` fills that end only.
pub fn mesh_fill_holes_tube() -> R {
    let _s = session()?;
    let cyl = Body::create_solid_cylinder(5.0, 10.0)?;
    let fc = facet_copy(&cyl, &PK_BODY_make_facet_body_o_t::default())?;
    let mut side = 0;
    for f in faces_of(fc)? {
        let m = mesh_of(f)?;
        if n_mfacets(m)? == 32 {
            side = m;
        }
    }
    let mut boundary: Option<PK_MFIN_t> = None;
    for fin in fins_of(fc)? {
        for mf in fin_mtopols(fin, &PK_FIN_find_mtopols_o_t::default())?.mfins {
            if mf.words[0] == side {
                boundary = Some(mf);
            }
        }
    }
    let boundary = boundary.ok_or("no boundary mfin on the side mesh")?;
    let fill = |o: &PK_MESH_fill_holes_o_t, hole: bool| -> Result<PK_MESH_t, Box<dyn Error>> {
        let tube = copy_of(side)?;
        let h = [PK_MTOPOL_t {
            words: [tube, boundary.words[1]],
        }];
        let mut o = *o;
        if hole {
            o.n_input_holes = 1;
            o.input_holes = h.as_ptr();
        }
        let mut filled = 0;
        rc_ok(
            unsafe { PK_MESH_fill_holes(tube, &mut o, &mut filled) },
            "PK_MESH_fill_holes",
        )?;
        Ok(filled)
    };
    assert_eq!(
        fill(&PK_MESH_fill_holes_o_t::default(), false)?,
        0,
        "max_n_mfins = 0 fills nothing"
    );
    let both = fill(
        &PK_MESH_fill_holes_o_t {
            max_n_mfins: 1000,
            ..Default::default()
        },
        false,
    )?;
    assert_eq!(n_mfacets(both)?, 32 + 2 * 14, "both 16-gon ends filled");
    let one = fill(&PK_MESH_fill_holes_o_t::default(), true)?;
    assert_eq!(
        n_mfacets(one)?,
        32 + 14,
        "only the hole named in input_holes"
    );
    Ok(())
}

/// `PK_MESH_imprint_vectors` (v2) with `want_mvertices` (@0x8) returns the new
/// mvertex at the imprinted point; `PK_PARTITION_ask_facet_geom` (v1, fixed
/// 8-argument signature) lists facet parts only when `want_parts` (@0x4) is set.
pub fn mesh_imprint_and_partition_enquiry() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let fb = facet_copy(&block, &PK_BODY_make_facet_body_o_t::default())?;
    // the top face mesh: every mvertex reached through the body's fins is at z = 30
    let mut top = 0;
    for f in faces_of(fb)? {
        let m = mesh_of(f)?;
        let mut all_top = true;
        for fin in fins_of(fb)? {
            for mv in fin_mtopols(fin, &PK_FIN_find_mtopols_o_t::default())?.mvertices {
                if mv.words[0] == m && position(mv)?[2] != 30.0 {
                    all_top = false;
                }
            }
        }
        if all_top {
            top = m;
        }
    }
    assert_ne!(top, 0, "found the top face mesh");
    let target = [1.0, 2.0, 30.0];
    let o = PK_MESH_imprint_vectors_o_t {
        want_mvertices: PK_LOGICAL_true,
        ..Default::default()
    };
    let mut v = [target];
    let mut out = 0;
    let mut mvx = [PK_MTOPOL_t { words: [0, 0] }];
    let mut res = [0u64; 64];
    rc_ok(
        unsafe {
            PK_MESH_imprint_vectors(
                copy_of(top)?,
                1,
                v.as_mut_ptr(),
                &o as *const _ as *mut _,
                &mut out,
                mvx.as_mut_ptr(),
                res.as_mut_ptr() as *mut _,
            )
        },
        "PK_MESH_imprint_vectors",
    )?;
    assert_eq!(class_of(out), PK_CLASS_mesh);
    assert_eq!(
        n_mfacets(out)?,
        4,
        "one interior point splits 2 mfacets into 4"
    );
    assert_eq!(
        mvx[0].words[0], out,
        "the returned mvertex belongs to the result mesh"
    );
    assert!(
        near(position(mvx[0])?, target),
        "and sits at the imprinted point"
    );

    let part = _s.partitions()?[0].tag();
    let ask = |o: &PK_PARTITION_ask_facet_geom_o_t| -> Result<
        (PK_geom_category_t, Vec<PK_PART_t>, Vec<PK_geom_category_t>),
        Box<dyn Error>,
    > {
        let (mut c, mut np, mut ng) = (0, 0, 0);
        let (mut pp, mut pc, mut pg) = (
            std::ptr::null_mut(),
            std::ptr::null_mut::<PK_geom_category_t>(),
            std::ptr::null_mut::<PK_GEOM_t>(),
        );
        rc_ok(
            unsafe {
                PK_PARTITION_ask_facet_geom(
                    part, o, &mut c, &mut np, &mut pp, &mut pc, &mut ng, &mut pg,
                )
            },
            "PK_PARTITION_ask_facet_geom",
        )?;
        let _ = tags(ng, pg);
        Ok((c, tags(np, pp), tags(np, pc)))
    };
    let (cat, parts, _) = ask(&PK_PARTITION_ask_facet_geom_o_t::default())?;
    assert_eq!(
        cat, PK_GEOM_category_mixed_c,
        "classic block + facet copy: mixed partition"
    );
    assert!(parts.is_empty(), "want_parts = false lists no parts");
    let (_, parts, cats) = ask(&PK_PARTITION_ask_facet_geom_o_t {
        want_parts: PK_LOGICAL_true,
        ..Default::default()
    })?;
    assert!(parts.contains(&fb), "the facet copy is listed: {parts:?}");
    assert!(!parts.contains(&block.tag()), "the classic block is not");
    assert!(cats.iter().all(|&c| c == PK_GEOM_category_facet_c));
    Ok(())
}

/// `PK_BODY_pick_topols` (v7, 224-byte options, 56-byte result): a ray down the
/// z axis picks the block's top face at z = 30; with `max_faces` = 2 and
/// `ignore_back_faces` = false (@0x40) it also picks the bottom face.
pub fn mesh_pick_topols_block() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let pick =
        |o: &PK_BODY_pick_topols_o_t| -> Result<Vec<PK_BODY_picked_topol_t>, Box<dyn Error>> {
            let mut b = [block.tag()];
            let mut ray = PK_AXIS1_sf_t {
                location: [0.0, 0.0, 100.0],
                axis: [0.0, 0.0, -1.0],
            };
            let mut r = PK_BODY_pick_topols_r_t::default();
            rc_ok(
                unsafe {
                    PK_BODY_pick_topols(
                        1,
                        b.as_mut_ptr(),
                        std::ptr::null_mut(),
                        &mut ray,
                        o as *const _ as *mut _,
                        &mut r,
                    )
                },
                "PK_BODY_pick_topols",
            )?;
            assert_eq!(
                (r.n_edges, r.n_vertices),
                (0, 0),
                "max_edges = max_vertices = 0"
            );
            let v = (0..r.n_faces.max(0) as usize)
                .map(|i| unsafe { *r.faces.add(i) })
                .collect();
            unsafe { PK_BODY_pick_topols_r_f(&mut r) };
            Ok(v)
        };
    let hits = pick(&PK_BODY_pick_topols_o_t::default())?;
    assert_eq!(hits.len(), 1);
    assert_eq!(class_of(hits[0].entity), PK_CLASS_face);
    assert!(
        near(hits[0].intersect, [0.0, 0.0, 30.0]),
        "top face hit at {:?}",
        hits[0].intersect
    );
    let hits = pick(&PK_BODY_pick_topols_o_t {
        max_faces: 2,
        ignore_back_faces: PK_LOGICAL_false,
        ..Default::default()
    })?;
    assert_eq!(hits.len(), 2, "the back (bottom) face too");
    assert!(
        near(hits[1].intersect, [0.0, 0.0, 0.0]),
        "bottom face hit at {:?}",
        hits[1].intersect
    );
    assert_ne!(hits[0].entity, hits[1].entity);
    Ok(())
}

/// `PK_MFACET_find_perimeters` (v4) accepts a face mesh's mfacets with defaults,
/// and reads `min_n_mfacets` at @0x28 (-1 is refused with 39 bad_value). Its
/// result struct is not verified, so only the return code is checked.
pub fn mesh_find_perimeters_options() -> R {
    let _s = session()?;
    let block = Body::create_solid_block(10.0, 20.0, 30.0)?;
    let fb = facet_copy(&block, &PK_BODY_make_facet_body_o_t::default())?;
    let mf = mfacets_near_fin(fins_of(fb)?[0])?;
    let run = |o: &PK_MFACET_find_perimeters_o_t| {
        let mut res = [0u64; 64];
        let mut m = mf.clone();
        unsafe {
            PK_MFACET_find_perimeters(
                m.len() as c_int,
                m.as_mut_ptr(),
                o as *const _ as *mut _,
                res.as_mut_ptr() as *mut _,
            )
        }
    };
    assert_eq!(run(&PK_MFACET_find_perimeters_o_t::default()), 0);
    assert_eq!(
        run(&PK_MFACET_find_perimeters_o_t {
            min_n_mfacets: -1,
            ..Default::default()
        }),
        39
    );
    assert_eq!(
        run(&PK_MFACET_find_perimeters_o_t {
            want_plines: 2,
            ..Default::default()
        }),
        908
    );
    Ok(())
}
