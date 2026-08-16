//! Adversarial review probe for Stage 7 (surface/surface intersection).
//! Read-only: creates orphan geometry, calls PK_SURF_intersect_surf and friends
//! directly, prints raw results. No files touched outside this one.

use parasolid::*;
use parasolid_sys::*;
use std::os::raw::{c_int, c_void};

#[derive(Debug, Clone, Copy)]
struct Br {
    tag: i32,
    class: i32,
    lo: f64,
    hi: f64,
    ty: i32,
}

struct Res {
    rc: i32,
    pts: Vec<[f64; 3]>,
    brs: Vec<Br>,
}

fn class_of(tag: i32) -> i32 {
    let mut c: PK_CLASS_t = -1;
    unsafe { PK_ENTITY_ask_class(tag, &mut c) };
    c
}

fn cname(c: i32) -> &'static str {
    match c {
        3001 => "line",
        3002 => "circle",
        3003 => "ellipse",
        3004 => "bcurve",
        3005 => "icurve",
        3006 => "fcurve",
        3007 => "spcurve",
        3008 => "trcurve",
        _ => "?",
    }
}

fn ssi_raw(s1: i32, s2: i32, version: i32, cat: i32) -> Res {
    let mut o: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
    o.o_t_version = version;
    o.mixed_curve_category = cat;
    let (mut nv, mut nc): (c_int, c_int) = (-999, -999);
    let mut vecs: *mut PK_VECTOR_t = 0x1 as *mut _;
    let mut curs: *mut PK_CURVE_t = 0x1 as *mut _;
    let mut bnds: *mut PK_INTERVAL_t = 0x1 as *mut _;
    let mut tys: *mut PK_intersect_curve_t = 0x1 as *mut _;
    let rc = unsafe {
        PK_SURF_intersect_surf(
            s1, s2, &o, &mut nv, &mut vecs, &mut nc, &mut curs, &mut bnds, &mut tys,
        )
    };
    let mut pts = Vec::new();
    let mut brs = Vec::new();
    if rc == PK_ERROR_no_errors {
        for i in 0..nv.max(0) as usize {
            let v = unsafe { *vecs.add(i) };
            pts.push([v[0], v[1], v[2]]);
        }
        for i in 0..nc.max(0) as usize {
            let t = unsafe { *curs.add(i) };
            let b = unsafe { *bnds.add(i) };
            let ty = unsafe { *tys.add(i) };
            brs.push(Br {
                tag: t,
                class: class_of(t),
                lo: b.low,
                hi: b.high,
                ty,
            });
        }
        unsafe {
            for p in [
                vecs as *mut c_void,
                curs as *mut c_void,
                bnds as *mut c_void,
                tys as *mut c_void,
            ] {
                if !p.is_null() && p as usize != 1 {
                    let _ = PK_MEMORY_free(p);
                }
            }
        }
    }
    Res { rc, pts, brs }
}

fn ssi(s1: &Surf, s2: &Surf) -> Res {
    ssi_raw(s1.tag(), s2.tag(), 2, PK_mixed_intersection_classic_c)
}

fn ceval(tag: i32, t: f64) -> [f64; 3] {
    let mut p = [0.0f64; 3];
    let rc = unsafe { PK_CURVE_eval(tag, t, 0, p.as_mut_ptr()) };
    if rc != 0 { [f64::NAN; 3] } else { p }
}

fn at(o: Vec3) -> Axis2 {
    Axis2::new(o, Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
}
fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}
fn saddle() -> Surf {
    let mut pts = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            let x = -3.0 + 3.0 * i as f64;
            let y = -3.0 + 3.0 * j as f64;
            pts.push(Vec3::new(x, y, 0.15 * (x * x - y * y) / 3.0));
        }
    }
    Surf::bsurf(2, 2, 3, 3, &pts, &[0.0, 1.0], &[3, 3], &[0.0, 1.0], &[3, 3]).unwrap()
}

fn flat_patch() -> Surf {
    let mut pts = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            pts.push(Vec3::new(-3.0 + 3.0 * i as f64, -3.0 + 3.0 * j as f64, 0.0));
        }
    }
    Surf::bsurf(2, 2, 3, 3, &pts, &[0.0, 1.0], &[3, 3], &[0.0, 1.0], &[3, 3]).unwrap()
}

fn plane_n(o: Vec3, n: Vec3) -> Surf {
    // build a ref direction orthogonal to n
    let a = if n.x.abs() < 0.9 {
        (1.0, 0.0, 0.0)
    } else {
        (0.0, 1.0, 0.0)
    };
    let r = (
        n.y * a.2 - n.z * a.1,
        n.z * a.0 - n.x * a.2,
        n.x * a.1 - n.y * a.0,
    );
    let l = (r.0 * r.0 + r.1 * r.1 + r.2 * r.2).sqrt();
    Surf::plane(Axis2::new(o, n, Vec3::new(r.0 / l, r.1 / l, r.2 / l))).unwrap()
}
fn unit(x: f64, y: f64, z: f64) -> Vec3 {
    let l = (x * x + y * y + z * z).sqrt();
    Vec3::new(x / l, y / l, z / l)
}

// ---- independent implicit residuals (no kernel projection involved) ----
#[derive(Clone, Copy)]
enum Imp {
    Plane { z: f64 },
    CylZ { r: f64, cx: f64, cy: f64 },
    SphZ { r: f64, c: [f64; 3] },
    TorZ { maj: f64, min: f64, c: [f64; 3] },
    ConeZ { r: f64, half: f64, c: [f64; 3] }, // radius r at z=c.z, half-angle
}

impl Imp {
    // signed distance-ish residual, normalised to length units
    fn resid(&self, p: [f64; 3]) -> f64 {
        match *self {
            Imp::Plane { z } => (p[2] - z).abs(),
            Imp::CylZ { r, cx, cy } => {
                (((p[0] - cx).powi(2) + (p[1] - cy).powi(2)).sqrt() - r).abs()
            }
            Imp::SphZ { r, c } => {
                (((p[0] - c[0]).powi(2) + (p[1] - c[1]).powi(2) + (p[2] - c[2]).powi(2)).sqrt() - r)
                    .abs()
            }
            Imp::TorZ { maj, min, c } => {
                let rho = ((p[0] - c[0]).powi(2) + (p[1] - c[1]).powi(2)).sqrt();
                (((rho - maj).powi(2) + (p[2] - c[2]).powi(2)).sqrt() - min).abs()
            }
            Imp::ConeZ { r, half, c } => {
                let rho = ((p[0] - c[0]).powi(2) + (p[1] - c[1]).powi(2)).sqrt();
                let expect = r + (p[2] - c[2]) * half.tan();
                (rho - expect).abs() * half.cos()
            }
        }
    }
}

fn report(label: &str, r: &Res, imps: Option<(Imp, Imp)>) {
    if r.rc != PK_ERROR_no_errors {
        println!("  {label}: ERROR rc={}", r.rc);
        return;
    }
    println!("  {label}: {} pts, {} curves", r.pts.len(), r.brs.len());
    for p in &r.pts {
        println!("      pt ({:.6}, {:.6}, {:.6})", p[0], p[1], p[2]);
    }
    for (i, b) in r.brs.iter().enumerate() {
        let mut extra = String::new();
        if let Some((a, c)) = imps {
            let (mut w1, mut w2) = (0.0f64, 0.0f64);
            for k in 0..=32 {
                let t = b.lo + (b.hi - b.lo) * (k as f64) / 32.0;
                let p = ceval(b.tag, t);
                if p[0].is_nan() {
                    continue;
                }
                w1 = w1.max(a.resid(p));
                w2 = w2.max(c.resid(p));
            }
            extra = format!(" implicit_resid=({:.3e}, {:.3e})", w1, w2);
        }
        let p0 = ceval(b.tag, b.lo);
        let pm = ceval(b.tag, 0.5 * (b.lo + b.hi));
        let p1 = ceval(b.tag, b.hi);
        println!(
            "      [{i}] tag={} {} type={} bounds=({:.9},{:.9}) len={:.6}{}",
            b.tag,
            cname(b.class),
            b.ty,
            b.lo,
            b.hi,
            b.hi - b.lo,
            extra
        );
        println!(
            "           start=({:.5},{:.5},{:.5}) mid=({:.5},{:.5},{:.5}) end=({:.5},{:.5},{:.5})",
            p0[0], p0[1], p0[2], pm[0], pm[1], pm[2], p1[0], p1[1], p1[2]
        );
    }
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");
    let o = v(0.0, 0.0, 0.0);

    // =====================================================================
    println!("\n== 1. version ceiling / field gating ==");
    let cyl = Surf::cylinder(at(o), 5.0).unwrap();
    let pl = Surf::plane(at(v(0.0, 0.0, 3.0))).unwrap();
    for ver in 0..=6 {
        for (name, tok) in [
            ("zero", 0),
            ("garbage", 12345),
            ("classic", PK_mixed_intersection_classic_c),
            ("pline", PK_mixed_intersection_pline_c),
            ("both", PK_mixed_intersection_both_c),
        ] {
            let r = ssi_raw(cyl.tag(), pl.tag(), ver, tok);
            println!(
                "  v={ver} cat={name:8} -> rc={} ncurves={}",
                r.rc,
                r.brs.len()
            );
        }
    }

    println!("\n== 1b. other intersect option structs: version gate ==");
    // FACE_intersect_face / FACE_intersect_surf carry the same field.
    let blk = Body::create_solid_block(10.0, 10.0, 10.0).unwrap();
    let faces = blk.faces().unwrap();
    let f0 = faces[0];
    let f1 = faces[1];
    for ver in 0..=4 {
        for (name, tok) in [
            ("zero", 0),
            ("garbage", 12345),
            ("classic", PK_mixed_intersection_classic_c),
        ] {
            let mut oo: PK_FACE_intersect_face_o_t = unsafe { std::mem::zeroed() };
            oo.o_t_version = ver;
            oo.mixed_curve_category = tok;
            let (mut nv, mut nc) = (0, 0);
            let (mut a, mut b, mut c, mut d) = (
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let rc = unsafe {
                PK_FACE_intersect_face(
                    f0.tag(),
                    f1.tag(),
                    &oo,
                    &mut nv,
                    &mut a,
                    &mut nc,
                    &mut b,
                    &mut c,
                    &mut d,
                )
            };
            unsafe {
                for p in [
                    a as *mut c_void,
                    b as *mut c_void,
                    c as *mut c_void,
                    d as *mut c_void,
                ] {
                    if !p.is_null() {
                        let _ = PK_MEMORY_free(p);
                    }
                }
            }
            println!("  FACE_intersect_face v={ver} cat={name:8} -> rc={rc} ncurves={nc}");
        }
    }
    for ver in 0..=4 {
        for (name, tok) in [("zero", 0), ("garbage", 12345)] {
            let mut oo: PK_FACE_intersect_surf_o_t = unsafe { std::mem::zeroed() };
            oo.o_t_version = ver;
            oo.mixed_curve_category = tok;
            let (mut nv, mut nc) = (0, 0);
            let (mut a, mut b, mut c, mut d) = (
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            let rc = unsafe {
                PK_FACE_intersect_surf(
                    f0.tag(),
                    pl.tag(),
                    &oo,
                    &mut nv,
                    &mut a,
                    &mut nc,
                    &mut b,
                    &mut c,
                    &mut d,
                )
            };
            unsafe {
                for p in [
                    a as *mut c_void,
                    b as *mut c_void,
                    c as *mut c_void,
                    d as *mut c_void,
                ] {
                    if !p.is_null() {
                        let _ = PK_MEMORY_free(p);
                    }
                }
            }
            println!("  FACE_intersect_surf v={ver} cat={name:8} -> rc={rc} ncurves={nc}");
        }
    }
    for ver in 0..=4 {
        let mut oo: PK_SURF_intersect_curve_o_t = unsafe { std::mem::zeroed() };
        oo.o_t_version = ver;
        oo._interest_reserved = 12345;
        let line = Curve::line(v(0.0, 0.0, -10.0), v(0.0, 0.0, 1.0)).unwrap();
        let (mut n, mut a, mut b, mut c, mut d) = (
            0,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        let rc = unsafe {
            PK_SURF_intersect_curve(
                pl.tag(),
                line.tag(),
                PK_INTERVAL_t {
                    low: 0.0,
                    high: 20.0,
                },
                &oo,
                &mut n,
                &mut a,
                &mut b,
                &mut c,
                &mut d,
            )
        };
        unsafe {
            for p in [
                a as *mut c_void,
                b as *mut c_void,
                c as *mut c_void,
                d as *mut c_void,
            ] {
                if !p.is_null() {
                    let _ = PK_MEMORY_free(p);
                }
            }
        }
        println!("  SURF_intersect_curve v={ver} interest=garbage -> rc={rc} n={n}");
    }

    // =====================================================================
    println!("\n== 2. ownership / sentinel outputs ==");
    // Disjoint planes: does the kernel write all six outputs when there is
    // nothing to report? (pointers pre-poisoned with 0x1, counts with -999)
    {
        let p1 = Surf::plane(at(o)).unwrap();
        let p2 = Surf::plane(at(v(0.0, 0.0, 9.0))).unwrap();
        let mut oo: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
        oo.o_t_version = 2;
        oo.mixed_curve_category = PK_mixed_intersection_classic_c;
        let (mut nv, mut nc): (c_int, c_int) = (-999, -999);
        let mut vecs: *mut PK_VECTOR_t = 0x1 as *mut _;
        let mut curs: *mut PK_CURVE_t = 0x1 as *mut _;
        let mut bnds: *mut PK_INTERVAL_t = 0x1 as *mut _;
        let mut tys: *mut PK_intersect_curve_t = 0x1 as *mut _;
        let rc = unsafe {
            PK_SURF_intersect_surf(
                p1.tag(),
                p2.tag(),
                &oo,
                &mut nv,
                &mut vecs,
                &mut nc,
                &mut curs,
                &mut bnds,
                &mut tys,
            )
        };
        println!(
            "  disjoint planes rc={rc} nv={nv} nc={nc} vecs={:?} curs={:?} bnds={:?} tys={:?}",
            vecs, curs, bnds, tys
        );
    }
    // Curves-but-no-points case: are `vectors` left poisoned?
    {
        let mut oo: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
        oo.o_t_version = 2;
        oo.mixed_curve_category = PK_mixed_intersection_classic_c;
        let (mut nv, mut nc): (c_int, c_int) = (-999, -999);
        let mut vecs: *mut PK_VECTOR_t = 0x1 as *mut _;
        let mut curs: *mut PK_CURVE_t = 0x1 as *mut _;
        let mut bnds: *mut PK_INTERVAL_t = 0x1 as *mut _;
        let mut tys: *mut PK_intersect_curve_t = 0x1 as *mut _;
        let rc = unsafe {
            PK_SURF_intersect_surf(
                cyl.tag(),
                pl.tag(),
                &oo,
                &mut nv,
                &mut vecs,
                &mut nc,
                &mut curs,
                &mut bnds,
                &mut tys,
            )
        };
        println!(
            "  cyl x plane rc={rc} nv={nv} nc={nc} vecs={:?} curs={:?} bnds={:?} tys={:?}",
            vecs, curs, bnds, tys
        );
        // stride evidence: read one element past n for bounds and types
        if nc > 0 {
            unsafe {
                let raw = bnds as *const f64;
                println!(
                    "    bounds raw doubles: {:?}",
                    (0..(nc as usize * 2 + 2))
                        .map(|i| *raw.add(i))
                        .collect::<Vec<_>>()
                );
                let rawi = tys as *const i32;
                println!(
                    "    types raw ints: {:?}",
                    (0..(nc as usize + 4))
                        .map(|i| *rawi.add(i))
                        .collect::<Vec<_>>()
                );
            }
            unsafe {
                for p in [
                    vecs as *mut c_void,
                    curs as *mut c_void,
                    bnds as *mut c_void,
                    tys as *mut c_void,
                ] {
                    if !p.is_null() && p as usize != 1 {
                        let _ = PK_MEMORY_free(p);
                    }
                }
            }
        }
    }
    // Is the returned curve owned by the caller (leak) or by the session?
    {
        let before = Surf::cylinder(at(o), 5.0).unwrap();
        let r = ssi(&before, &pl);
        let tag = r.brs[0].tag;
        let mut c: PK_CLASS_t = -1;
        let rc1 = unsafe { PK_ENTITY_ask_class(tag, &mut c) };
        // delete it: if it is caller-owned orphan geometry, this succeeds
        let rc2 = unsafe { PK_ENTITY_delete(1, &tag) };
        let rc3 = unsafe { PK_ENTITY_ask_class(tag, &mut c) };
        println!(
            "  curve tag {tag}: ask_class rc={rc1}, delete rc={rc2}, ask_class after rc={rc3}"
        );
    }

    // =====================================================================
    println!("\n== 3. analytic battery / branch completeness ==");
    let plane0 = Surf::plane(at(o)).unwrap();
    // cyl-cyl equal radius, axes crossing
    {
        let c1 = Surf::cylinder(at(o), 5.0).unwrap();
        let c2 = Surf::cylinder(Axis2::new(o, v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)), 5.0).unwrap();
        let r = ssi(&c1, &c2);
        report(
            "cyl(z,5) x cyl(x,5) equal radius",
            &r,
            Some((
                Imp::CylZ {
                    r: 5.0,
                    cx: 0.0,
                    cy: 0.0,
                },
                Imp::CylZ {
                    r: 5.0,
                    cx: 0.0,
                    cy: 0.0,
                },
            )),
        );
        // total arc coverage check via chord sampling
        for b in &r.brs {
            let n = 64;
            let mut len = 0.0;
            let mut prev = ceval(b.tag, b.lo);
            for k in 1..=n {
                let t = b.lo + (b.hi - b.lo) * (k as f64) / (n as f64);
                let p = ceval(b.tag, t);
                len += ((p[0] - prev[0]).powi(2)
                    + (p[1] - prev[1]).powi(2)
                    + (p[2] - prev[2]).powi(2))
                .sqrt();
                prev = p;
            }
            println!("      branch chord-length ~ {len:.6}");
        }
    }
    // cyl-cyl unequal radius (classical: single closed curve)
    {
        let c1 = Surf::cylinder(at(o), 5.0).unwrap();
        let c2 = Surf::cylinder(Axis2::new(o, v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)), 3.0).unwrap();
        let r = ssi(&c1, &c2);
        report("cyl(z,5) x cyl(x,3)", &r, None);
    }
    // sphere x cylinder, sphere radius == cylinder radius, concentric (Viviani-like)
    {
        let sp = Surf::sphere(at(o), 5.0).unwrap();
        let cy = Surf::cylinder(at(v(2.5, 0.0, 0.0)), 2.5).unwrap();
        let r = ssi(&sp, &cy);
        report(
            "Viviani: sphere(5) x cyl(2.5 offset 2.5)",
            &r,
            Some((
                Imp::SphZ {
                    r: 5.0,
                    c: [0.0; 3],
                },
                Imp::CylZ {
                    r: 2.5,
                    cx: 2.5,
                    cy: 0.0,
                },
            )),
        );
    }
    // sphere x cylinder same radius coaxial-offset -> two circles? (internally tangent)
    {
        let sp = Surf::sphere(at(o), 5.0).unwrap();
        let cy = Surf::cylinder(at(o), 5.0).unwrap();
        let r = ssi(&sp, &cy);
        report("sphere(5) x coaxial cyl(5) [tangent circle]", &r, None);
    }
    // cone x cone, apex to apex
    {
        let k1 = Surf::cone(at(v(0.0, 0.0, 0.0)), 0.0, 0.5).unwrap();
        let k2 = Surf::cone(
            Axis2::new(v(0.0, 0.0, 0.0), v(0.0, 0.0, -1.0), v(1.0, 0.0, 0.0)),
            0.0,
            0.5,
        )
        .unwrap();
        let r = ssi(&k1, &k2);
        report("cone x mirrored cone (shared apex)", &r, None);
    }
    // cone x plane through apex
    {
        let k = Surf::cone(at(o), 0.0, 0.5).unwrap();
        let vertplane = Surf::plane(Axis2::new(o, v(0.0, 1.0, 0.0), v(1.0, 0.0, 0.0))).unwrap();
        let r = ssi(&k, &vertplane);
        report("cone x plane through apex (two lines)", &r, None);
        // and a plane tangent along one line
        let k2 = Surf::cone(at(o), 5.0, 0.5).unwrap();
        let r2 = ssi(&k2, &plane0);
        report("cone(r=5@z0, 0.5rad) x z=0 plane", &r2, None);
    }
    // torus x torus
    {
        let t1 = Surf::torus(at(o), 5.0, 1.5).unwrap();
        let t2 = Surf::torus(
            Axis2::new(v(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)),
            5.0,
            1.5,
        )
        .unwrap();
        let r = ssi(&t1, &t2);
        report("torus(z) x torus(x) same size", &r, None);
    }
    // torus x plane grazing / Villarceau
    {
        let (maj, min) = (5.0, 1.5);
        let t = Surf::torus(at(o), maj, min).unwrap();
        for (label, ang, z) in [
            ("z=1.4999 (just inside crown)", 0.0, 1.4999),
            ("z=1.5 (crown, tangent)", 0.0, 1.5),
            ("z=1.50000001", 0.0, 1.50000001),
            ("z=1.0 (two circles-ish)", 0.0, 1.0),
        ] {
            let _ = ang;
            let p = Surf::plane(at(v(0.0, 0.0, z))).unwrap();
            let r = ssi(&t, &p);
            report(
                &format!("torus x plane {label}"),
                &r,
                Some((
                    Imp::TorZ {
                        maj,
                        min,
                        c: [0.0; 3],
                    },
                    Imp::Plane { z },
                )),
            );
        }
        // Villarceau: bitangent plane through the centre, angle asin(min/maj)
        let a = (min / maj).acos();
        let n = v(0.0, a.cos(), a.sin());
        let vp = plane_n(o, n);
        let r = ssi(&t, &vp);
        report(
            "torus x Villarceau bitangent plane (expect 2 circles)",
            &r,
            Some((
                Imp::TorZ {
                    maj,
                    min,
                    c: [0.0; 3],
                },
                Imp::Plane { z: 0.0 }, // not used meaningfully
            )),
        );
    }

    // =====================================================================
    println!("\n== 4. degenerate self-intersecting surfaces ==");
    for (label, maj, min) in [
        ("lemon minor>major (2,5)", 2.0, 5.0),
        ("apple/horn minor==major (5,5)", 5.0, 5.0),
        ("horn minor slightly > major (5,5.5)", 5.0, 5.5),
    ] {
        match Surf::torus(at(o), maj, min) {
            Err(e) => println!("  {label}: torus creation failed: {e:?}"),
            Ok(t) => {
                let p = Surf::plane(at(o)).unwrap();
                let r = ssi(&t, &p);
                report(
                    &format!("{label} x equator plane"),
                    &r,
                    Some((
                        Imp::TorZ {
                            maj,
                            min,
                            c: [0.0; 3],
                        },
                        Imp::Plane { z: 0.0 },
                    )),
                );
                // through the axis: a self-intersecting profile
                let vp = Surf::plane(Axis2::new(o, v(0.0, 1.0, 0.0), v(1.0, 0.0, 0.0))).unwrap();
                let r2 = ssi(&t, &vp);
                report(&format!("{label} x axial plane"), &r2, None);
            }
        }
    }
    // sphere x plane exactly at the pole (tangent at the seam pole)
    {
        let sp = Surf::sphere(at(o), 5.0).unwrap();
        let p = Surf::plane(at(v(0.0, 0.0, 5.0))).unwrap();
        let r = ssi(&sp, &p);
        report("sphere x plane at north pole (tangent point)", &r, None);
        let p2 = Surf::plane(at(v(0.0, 0.0, 5.0 + 1e-12))).unwrap();
        let r2 = ssi(&sp, &p2);
        report("sphere x plane 1e-12 above pole", &r2, None);
        let p3 = Surf::plane(at(v(0.0, 0.0, 5.0 - 1e-9))).unwrap();
        let r3 = ssi(&sp, &p3);
        report("sphere x plane 1e-9 below pole", &r3, None);
    }
    // plane through the cylinder seam
    {
        let cy = Surf::cylinder(at(o), 5.0).unwrap();
        let sp = Surf::plane(Axis2::new(o, v(0.0, 1.0, 0.0), v(1.0, 0.0, 0.0))).unwrap();
        let r = ssi(&cy, &sp);
        report("cyl x plane containing the seam (2 lines)", &r, None);
    }

    // =====================================================================
    println!("\n== 5. coincidence battery ==");
    {
        let cases: Vec<(String, i32, i32)> = vec![
            (
                "identical planes".into(),
                Surf::plane(at(o)).unwrap().tag(),
                Surf::plane(at(o)).unwrap().tag(),
            ),
            (
                "same plane, reversed normal".into(),
                Surf::plane(at(o)).unwrap().tag(),
                Surf::plane(Axis2::new(o, v(0.0, 0.0, -1.0), v(1.0, 0.0, 0.0)))
                    .unwrap()
                    .tag(),
            ),
            (
                "same plane, rotated ref direction".into(),
                Surf::plane(at(o)).unwrap().tag(),
                Surf::plane(Axis2::new(o, v(0.0, 0.0, 1.0), v(0.0, 1.0, 0.0)))
                    .unwrap()
                    .tag(),
            ),
            (
                "same plane, shifted origin in-plane".into(),
                Surf::plane(at(o)).unwrap().tag(),
                Surf::plane(at(v(3.0, 4.0, 0.0))).unwrap().tag(),
            ),
            (
                "coincident cyls".into(),
                Surf::cylinder(at(o), 5.0).unwrap().tag(),
                Surf::cylinder(at(v(0.0, 0.0, 7.0)), 5.0).unwrap().tag(),
            ),
            (
                "cyl vs cyl radius +1e-9".into(),
                Surf::cylinder(at(o), 5.0).unwrap().tag(),
                Surf::cylinder(at(o), 5.0 + 1e-9).unwrap().tag(),
            ),
            (
                "cyl vs cyl radius +1e-6".into(),
                Surf::cylinder(at(o), 5.0).unwrap().tag(),
                Surf::cylinder(at(o), 5.0 + 1e-6).unwrap().tag(),
            ),
            ("same surf tag twice".into(), cyl.tag(), cyl.tag()),
            (
                "sphere vs coincident sphere".into(),
                Surf::sphere(at(o), 5.0).unwrap().tag(),
                Surf::sphere(at(o), 5.0).unwrap().tag(),
            ),
            (
                "torus vs coincident torus".into(),
                Surf::torus(at(o), 5.0, 1.5).unwrap().tag(),
                Surf::torus(at(o), 5.0, 1.5).unwrap().tag(),
            ),
        ];
        for (label, a, b) in cases {
            for (cn, cat) in [
                ("classic", PK_mixed_intersection_classic_c),
                ("pline", PK_mixed_intersection_pline_c),
                ("both", PK_mixed_intersection_both_c),
            ] {
                let r = ssi_raw(a, b, 2, cat);
                println!(
                    "  {label} [{cn}] -> rc={} pts={} curves={} {:?}",
                    r.rc,
                    r.pts.len(),
                    r.brs.len(),
                    r.brs
                        .iter()
                        .map(|b| (cname(b.class), b.ty))
                        .collect::<Vec<_>>()
                );
            }
        }
    }

    // =====================================================================
    println!("\n== 6. mixed_curve_category actually changes anything? ==");
    {
        // procedural pairs where the intersection is not analytic
        let mk = |cat: i32, s1: &Surf, s2: &Surf, label: &str| {
            let r = ssi_raw(s1.tag(), s2.tag(), 2, cat);
            println!(
                "  {label}: rc={} curves={:?} bounds={:?}",
                r.rc,
                r.brs
                    .iter()
                    .map(|b| (cname(b.class), b.class, b.ty))
                    .collect::<Vec<_>>(),
                r.brs.iter().map(|b| (b.lo, b.hi)).collect::<Vec<_>>()
            );
        };
        let t = Surf::torus(at(o), 5.0, 1.5).unwrap();
        let obl = plane_n(v(0.0, 0.0, 0.3), unit(0.3, 0.2, 1.0));
        for (cn, cat) in [
            ("classic", PK_mixed_intersection_classic_c),
            ("pline", PK_mixed_intersection_pline_c),
            ("both", PK_mixed_intersection_both_c),
        ] {
            mk(cat, &t, &obl, &format!("torus x oblique plane [{cn}]"));
        }
        // torus x torus (non-analytic)
        let t2 = Surf::torus(
            Axis2::new(v(3.0, 0.0, 0.0), v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)),
            5.0,
            1.5,
        )
        .unwrap();
        for (cn, cat) in [
            ("classic", PK_mixed_intersection_classic_c),
            ("pline", PK_mixed_intersection_pline_c),
            ("both", PK_mixed_intersection_both_c),
        ] {
            mk(cat, &t, &t2, &format!("torus x torus [{cn}]"));
        }
    }

    // =====================================================================
    println!("\n== 7. residual: range_to_point vs implicit ==");
    {
        let t = Surf::torus(at(o), 5.0, 1.5).unwrap();
        let obl = plane_n(v(0.0, 0.0, 0.4), unit(0.25, 0.1, 1.0));
        let r = ssi(&t, &obl);
        // plane implicit: n.(p - p0) with normalised n
        let n0 = unit(0.25, 0.1, 1.0);
        let nl = (n0.x * n0.x + n0.y * n0.y + n0.z * n0.z).sqrt();
        let nn = [n0.x / nl, n0.y / nl, n0.z / nl];
        let p0 = [0.0, 0.0, 0.4];
        for b in &r.brs {
            let (mut wt, mut wp, mut wr1, mut wr2) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
            for k in 0..=64 {
                let tt = b.lo + (b.hi - b.lo) * (k as f64) / 64.0;
                let p = ceval(b.tag, tt);
                if p[0].is_nan() {
                    continue;
                }
                wt = wt.max(
                    Imp::TorZ {
                        maj: 5.0,
                        min: 1.5,
                        c: [0.0; 3],
                    }
                    .resid(p),
                );
                wp = wp.max(
                    ((p[0] - p0[0]) * nn[0] + (p[1] - p0[1]) * nn[1] + (p[2] - p0[2]) * nn[2])
                        .abs(),
                );
                let pv = v(p[0], p[1], p[2]);
                if let Ok(rr) = t.range_to_point(pv) {
                    wr1 = wr1.max(rr.distance);
                }
                if let Ok(rr) = obl.range_to_point(pv) {
                    wr2 = wr2.max(rr.distance);
                }
            }
            println!(
                "  branch cls={} type={}: implicit torus={:.3e} plane={:.3e} | range_to_point torus={:.3e} plane={:.3e}",
                cname(b.class),
                b.ty,
                wt,
                wp,
                wr1,
                wr2
            );
        }
        // Also: does the reported `bounds` cover the whole branch, or is the
        // curve's own natural interval wider?
        for b in &r.brs {
            let mut iv = PK_INTERVAL_t {
                low: 0.0,
                high: 0.0,
            };
            let rc = unsafe { PK_CURVE_ask_interval(b.tag, &mut iv) };
            let closed = 0;
            let rc2 = 0;
            println!(
                "    bounds=({},{}) natural=({},{}) rc={rc} closed={closed} rc2={rc2}",
                b.lo, b.hi, iv.low, iv.high
            );
        }
    }

    // =====================================================================
    println!("\n== 8. token scan over a wide battery ==");
    {
        let mut seen: std::collections::BTreeMap<i32, String> = Default::default();
        let mut push = |label: &str, r: &Res| {
            for b in &r.brs {
                seen.entry(b.ty).or_insert_with(|| label.to_string());
            }
        };
        let mut cases: Vec<(String, Res)> = Vec::new();
        let sph = Surf::sphere(at(o), 5.0).unwrap();
        let tor = Surf::torus(at(o), 5.0, 1.5).unwrap();
        cases.push(("cyl x plane".into(), ssi(&cyl, &pl)));
        cases.push(("sph x plane".into(), ssi(&sph, &plane0)));
        cases.push((
            "tangent plane to cyl".into(),
            ssi(
                &cyl,
                &Surf::plane(Axis2::new(
                    v(5.0, 0.0, 0.0),
                    v(1.0, 0.0, 0.0),
                    v(0.0, 0.0, 1.0),
                ))
                .unwrap(),
            ),
        ));
        cases.push((
            "torus crown tangent".into(),
            ssi(&tor, &Surf::plane(at(v(0.0, 0.0, 1.5))).unwrap()),
        ));
        cases.push((
            "cone x sphere tangent".into(),
            ssi(
                &Surf::cone(at(o), 0.0, 0.5).unwrap(),
                &Surf::sphere(at(v(0.0, 0.0, 5.0)), 5.0 * 0.5f64.sin()).unwrap(),
            ),
        ));
        cases.push(("bsurf saddle x plane".into(), ssi(&saddle(), &plane0)));
        cases.push((
            "bsurf saddle x offset(bsurf saddle,1)".into(),
            ssi(&saddle(), &Surf::offset_surface(&saddle(), 1.0).unwrap()),
        ));
        cases.push((
            "bsurf saddle x sphere".into(),
            ssi(&saddle(), &Surf::sphere(at(v(0.0, 0.0, 0.0)), 3.0).unwrap()),
        ));
        cases.push(("spun x plane".into(), {
            let prof = Curve::line(v(3.0, 0.0, 0.0), v(0.0, 0.0, 1.0)).unwrap();
            let sp = Surf::spun(&prof, o, v(0.0, 0.0, 1.0)).unwrap();
            ssi(&sp, &pl)
        }));
        cases.push(("swept x plane".into(), {
            let prof = Curve::circle(at(o), 2.0).unwrap();
            let sw = Surf::swept(&prof, v(0.0, 0.0, 1.0)).unwrap();
            ssi(&sw, &Surf::plane(at(v(0.0, 0.0, 2.0))).unwrap())
        }));
        for (l, r) in &cases {
            report(l, r, None);
            push(l, r);
        }
        println!("  distinct PK_intersect_curve_t tokens seen: {:?}", seen);
    }

    println!("\n== 9. lemon torus surface extent (is the inner lobe part of the surface?) ==");
    {
        let t = Surf::torus(at(o), 2.0, 5.0).unwrap();
        let ub = t.uvbox().unwrap();
        println!("  lemon(2,5) uvbox = {:?}", ub);
        for vv in [0.0, 1.0, std::f64::consts::PI, -std::f64::consts::PI / 2.0] {
            match t.eval(0.0, vv) {
                Ok(p) => println!("    eval(u=0,v={vv}) = ({:.6},{:.6},{:.6})", p.x, p.y, p.z),
                Err(e) => println!("    eval(u=0,v={vv}) err {e:?}"),
            }
        }
        for q in [v(3.0, 0.0, 0.0), v(-3.0, 0.0, 0.0), v(7.0, 0.0, 0.0)] {
            match t.range_to_point(q) {
                Ok(r) => println!(
                    "    range_to_point({:.1},{:.1},{:.1}) dist={:.3e}",
                    q.x, q.y, q.z, r.distance
                ),
                Err(e) => println!("    range_to_point err {e:?}"),
            }
        }
        // a plane that should cut the inner lobe only
        for z in [0.0, 2.0, 4.0] {
            let p = Surf::plane(at(v(0.0, 0.0, z))).unwrap();
            let r = ssi(&t, &p);
            report(&format!("lemon(2,5) x plane z={z}"), &r, None);
        }
    }

    println!("\n== 10. coincidence across REPRESENTATIONS ==");
    {
        let cy = Surf::cylinder(at(o), 5.0).unwrap();
        // spun line == cylinder
        let prof = Curve::line(v(5.0, 0.0, 0.0), v(0.0, 0.0, 1.0)).unwrap();
        let spun = Surf::spun(&prof, o, v(0.0, 0.0, 1.0)).unwrap();
        let r = ssi(&cy, &spun);
        println!(
            "  cyl(5) x spun-line-at-5 (same surface, different rep) -> rc={} pts={} curves={}",
            r.rc,
            r.pts.len(),
            r.brs.len()
        );
        report("  detail", &r, None);
        // planar bsurf patch coincident with an infinite plane (PARTIAL coincidence)
        let flat = flat_patch();
        let r2 = ssi(&flat, &Surf::plane(at(o)).unwrap());
        println!(
            "  planar bsurf patch x coincident infinite plane -> rc={} pts={} curves={}",
            r2.rc,
            r2.pts.len(),
            r2.brs.len()
        );
        report("  detail", &r2, None);
        let r2b = ssi(&flat, &Surf::plane(at(v(0.0, 0.0, 1.0))).unwrap());
        println!(
            "  planar bsurf patch x parallel plane z=1 -> rc={} pts={} curves={}",
            r2b.rc,
            r2b.pts.len(),
            r2b.brs.len()
        );
        let r2c = ssi(&flat, &plane_n(o, unit(1.0, 0.0, 0.0)));
        println!(
            "  planar bsurf patch x crossing vertical plane -> rc={} pts={} curves={}",
            r2c.rc,
            r2c.pts.len(),
            r2c.brs.len()
        );
        report("  detail", &r2c, None);
        // swept circle == cylinder
        let circ = Curve::circle(at(o), 5.0).unwrap();
        let sw = Surf::swept(&circ, v(0.0, 0.0, 1.0)).unwrap();
        let r3 = ssi(&cy, &sw);
        println!(
            "  cyl(5) x swept-circle (same surface) -> rc={} pts={} curves={}",
            r3.rc,
            r3.pts.len(),
            r3.brs.len()
        );
        report("  detail", &r3, None);
        // partial coincidence: plane vs a bsurf that is planar over part of it
        let pln = Surf::plane(at(o)).unwrap();
        let _ = pln;
    }

    println!("\n== 11. does the reported branch set cover the true intersection? ==");
    {
        // Sample the true intersection of cyl(z,5) and cyl(x,5) analytically and
        // check every sampled true point is within tolerance of some returned branch.
        let c1 = Surf::cylinder(at(o), 5.0).unwrap();
        let c2 = Surf::cylinder(Axis2::new(o, v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)), 5.0).unwrap();
        let r = ssi(&c1, &c2);
        let mut worst = 0.0f64;
        for k in 0..400 {
            let th = 2.0 * std::f64::consts::PI * (k as f64) / 400.0;
            for sgn in [1.0f64, -1.0] {
                // x=5cos, y=5sin, z = +-sqrt(25 - x^2) requires |x|<=5 always true
                let x = 5.0 * th.cos();
                let y = 5.0 * th.sin();
                let z = sgn * (25.0f64 - y * y).sqrt();
                // point on cyl1 (x^2+y^2=25) and cyl2 (y^2+z^2=25)
                let mut best = f64::INFINITY;
                for b in &r.brs {
                    for j in 0..=200 {
                        let t = b.lo + (b.hi - b.lo) * (j as f64) / 200.0;
                        let p = ceval(b.tag, t);
                        let d =
                            ((p[0] - x).powi(2) + (p[1] - y).powi(2) + (p[2] - z).powi(2)).sqrt();
                        if d < best {
                            best = d;
                        }
                    }
                }
                worst = worst.max(best);
            }
        }
        println!(
            "  worst distance from a TRUE cyl-cyl intersection point to the returned branch set: {worst:.3e}"
        );
        let tags: Vec<i32> = r.brs.iter().map(|b| b.tag).collect();
        println!("  branch tags: {tags:?} (duplicates => the same curve returned twice)");
    }

    println!("\n== 12. do the returned curves leak? ==");
    {
        let a = Surf::cylinder(at(o), 5.0).unwrap();
        let b = Surf::plane(at(v(0.0, 0.0, 3.0))).unwrap();
        let mut first = 0;
        let mut last = 0;
        let mut alive = 0;
        let mut tags = Vec::new();
        for i in 0..200 {
            let r = a.intersect(&b).unwrap(); // the SAFE wrapper
            let t = r.curves[0].curve.tag();
            if i == 0 {
                first = t;
            }
            last = t;
            tags.push(t);
        }
        for t in &tags {
            let mut c: PK_CLASS_t = -1;
            if unsafe { PK_ENTITY_ask_class(*t, &mut c) } == 0 {
                alive += 1;
            }
        }
        println!(
            "  200 x Surf::intersect via the SAFE wrapper: first tag {first}, last tag {last}"
        );
        println!(
            "  curves still alive after the wrapper returned and its PkArray dropped: {alive}/200"
        );
        // are they attached to anything? ask class + is_orphan-ish check
        let mut c: PK_CLASS_t = -1;
        unsafe { PK_ENTITY_ask_class(last, &mut c) };
        println!("  class of the last leaked curve: {c}");
    }

    println!("\n== 13. is range_to_point a fair residual? (floor test) ==");
    {
        let cy = Surf::cylinder(at(o), 5.0).unwrap();
        for d in [0.0, 1e-15, 1e-13, 1e-11, 1e-9, 1e-7, 1e-5] {
            let p = v(5.0 + d, 0.0, 1.0);
            let r = cy.range_to_point(p).unwrap();
            println!(
                "  point {} off the cylinder -> range_to_point = {:.3e} (error {:.1e})",
                d,
                r.distance,
                (r.distance - d).abs()
            );
        }
        // and a plane
        let pl0 = Surf::plane(at(o)).unwrap();
        for d in [1e-15, 1e-13, 1e-11, 1e-9] {
            let r = pl0.range_to_point(v(1.0, 2.0, d)).unwrap();
            println!(
                "  plane: point {d} off -> range_to_point = {:.3e}",
                r.distance
            );
        }
    }

    println!("\n== 14. bounds vs the curve's natural interval ==");
    {
        let c1 = Surf::cylinder(at(o), 5.0).unwrap();
        let c2 = Surf::cylinder(Axis2::new(o, v(1.0, 0.0, 0.0), v(0.0, 0.0, 1.0)), 5.0).unwrap();
        let r = ssi(&c1, &c2);
        for b in &r.brs {
            let mut iv = PK_INTERVAL_t {
                low: 0.0,
                high: 0.0,
            };
            unsafe { PK_CURVE_ask_interval(b.tag, &mut iv) };
            println!(
                "  tag={} bounds=({:.6},{:.6}) natural=({:.6},{:.6}) outside_natural={}",
                b.tag,
                b.lo,
                b.hi,
                iv.low,
                iv.high,
                (b.hi > iv.high + 1e-12) || (b.lo < iv.low - 1e-12)
            );
        }
    }

    println!("\n== 15. Villarceau: independent two-sided implicit residual ==");
    {
        let (maj, min) = (5.0, 1.5);
        let t = Surf::torus(at(o), maj, min).unwrap();
        let a = (min / maj).acos();
        let n = v(0.0, a.cos(), a.sin());
        let vp = plane_n(o, n);
        let r = ssi(&t, &vp);
        let mut total = 0.0;
        for b in &r.brs {
            let (mut wt, mut wp) = (0.0f64, 0.0f64);
            for k in 0..=200 {
                let tt = b.lo + (b.hi - b.lo) * (k as f64) / 200.0;
                let p = ceval(b.tag, tt);
                wt = wt.max(
                    Imp::TorZ {
                        maj,
                        min,
                        c: [0.0; 3],
                    }
                    .resid(p),
                );
                wp = wp.max((p[0] * n.x + p[1] * n.y + p[2] * n.z).abs());
            }
            total += b.hi - b.lo;
            println!(
                "  branch tag={} cls={} type={} len={:.6} implicit torus={:.3e} plane={:.3e}",
                b.tag,
                cname(b.class),
                b.ty,
                b.hi - b.lo,
                wt,
                wp
            );
        }
        println!(
            "  branches={} total param length={:.6} (two Villarceau circles of radius {maj} => {:.6})",
            r.brs.len(),
            total,
            4.0 * std::f64::consts::PI * maj
        );
    }

    println!("\n== 16. near-tangency: is a small branch dropped? ==");
    {
        let sp = Surf::sphere(at(o), 5.0).unwrap();
        for d in [
            1e-12f64, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2,
        ] {
            let z = 5.0 - d;
            let true_r = (25.0 - z * z).sqrt();
            let p = Surf::plane(at(v(0.0, 0.0, z))).unwrap();
            let r = ssi(&sp, &p);
            let got = if r.brs.is_empty() {
                "none".to_string()
            } else {
                let mut sf: PK_CIRCLE_sf_t = unsafe { std::mem::zeroed() };
                let rc = unsafe { PK_CIRCLE_ask(r.brs[0].tag, &mut sf) };
                if rc == 0 {
                    format!("circle r={:.6e} type={}", sf.radius, r.brs[0].ty)
                } else {
                    format!("cls {} type={}", r.brs[0].class, r.brs[0].ty)
                }
            };
            println!(
                "  sphere(5) x plane z=5-{:.0e}: true circle r={:.3e} -> {} pts, {} curves [{}]",
                d,
                true_r,
                r.pts.len(),
                r.brs.len(),
                got
            );
        }
        // cylinder-cylinder near tangency: parallel cylinders barely overlapping
        let c1 = Surf::cylinder(at(o), 5.0).unwrap();
        for d in [1e-9f64, 1e-7, 1e-5, 1e-3] {
            let c2 = Surf::cylinder(at(v(10.0 - d, 0.0, 0.0)), 5.0).unwrap();
            let r = ssi(&c1, &c2);
            println!(
                "  parallel cyls overlapping by {:.0e}: {} pts, {} curves {:?}",
                d,
                r.pts.len(),
                r.brs.len(),
                r.brs
                    .iter()
                    .map(|b| (cname(b.class), b.ty))
                    .collect::<Vec<_>>()
            );
        }
        // torus crown: plane just below the crown, true two circles very close
        let t = Surf::torus(at(o), 5.0, 1.5).unwrap();
        for d in [1e-12f64, 1e-10, 1e-8, 1e-6, 1e-4, 1e-2] {
            let z = 1.5 - d;
            let p = Surf::plane(at(v(0.0, 0.0, z))).unwrap();
            let r = ssi(&t, &p);
            let sep = 2.0 * (2.25 - z * z).sqrt();
            println!(
                "  torus x plane z=1.5-{:.0e}: true 2 circles separated by {:.3e} -> {} curves {:?}",
                d,
                sep,
                r.brs.len(),
                r.brs.iter().map(|b| b.ty).collect::<Vec<_>>()
            );
        }
    }

    println!("\n== 17. range_to_point floor, bisected ==");
    {
        let cy = Surf::cylinder(at(o), 5.0).unwrap();
        let mut lo = 1e-9f64;
        let mut hi = 1e-7f64;
        for _ in 0..60 {
            let mid = (lo * hi).sqrt();
            let r = cy.range_to_point(v(5.0 + mid, 0.0, 1.0)).unwrap();
            if r.distance == 0.0 {
                lo = mid
            } else {
                hi = mid
            }
        }
        println!(
            "  cylinder: range_to_point reports 0.0 below ~{:.4e} and nonzero above ~{:.4e}",
            lo, hi
        );
    }

    println!("\n== 18. does the options `tolerance` field move the collapse threshold? ==");
    {
        let sp = Surf::sphere(at(o), 5.0).unwrap();
        for tol in [0.0f64, 1e-10, 1e-8, 1e-6, 1e-4, 1e-2, 1.0] {
            let mut line = format!("  tolerance={tol:.0e}: ");
            for d in [1e-9f64, 1e-8, 1e-7, 1e-5, 1e-3] {
                let z = 5.0 - d;
                let p = Surf::plane(at(v(0.0, 0.0, z))).unwrap();
                let mut oo: PK_SURF_intersect_surf_o_t = unsafe { std::mem::zeroed() };
                oo.o_t_version = 2;
                oo.mixed_curve_category = PK_mixed_intersection_classic_c;
                oo.tolerance = tol;
                let (mut nv, mut nc) = (0, 0);
                let (mut a, mut b, mut c, mut e) = (
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                );
                let rc = unsafe {
                    PK_SURF_intersect_surf(
                        sp.tag(),
                        p.tag(),
                        &oo,
                        &mut nv,
                        &mut a,
                        &mut nc,
                        &mut b,
                        &mut c,
                        &mut e,
                    )
                };
                unsafe {
                    for q in [
                        a as *mut c_void,
                        b as *mut c_void,
                        c as *mut c_void,
                        e as *mut c_void,
                    ] {
                        if !q.is_null() {
                            let _ = PK_MEMORY_free(q);
                        }
                    }
                }
                line.push_str(&format!("[d={d:.0e} rc={rc} pts={nv} curves={nc}] "));
            }
            println!("{line}");
        }
    }

    println!("\n== done ==");
}
