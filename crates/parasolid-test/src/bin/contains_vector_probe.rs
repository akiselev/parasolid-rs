//! Dynamic probe: `PK_BODY_contains_vector` enclosure codes on regular, boundary,
//! degenerate, near-degenerate and invalid inputs.
//!
//! Signature per the PK reference page (`pk_body_contains_vector.html`):
//! `(PK_BODY_t body, PK_VECTOR_t vector, PK_enclosure_t *enclosure, PK_TOPOL_t *topology)`.
//! Enclosure tokens: inside 5701, outside 5702, on 5703 (PK token table).
//! The returned topology is classified by membership in the body's own
//! face / edge / vertex / region tag lists (no class-token assumptions).
//!
//! The session is started through `Session::start`, which applies
//! `PK_SESSION_set_behaviour(latest)`; the probe prints the behaviour it runs under.

use parasolid::*;
use parasolid_sys as sys;
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut core::ffi::c_void;
    fn GetModuleFileNameA(module: *mut core::ffi::c_void, buf: *mut u8, size: u32) -> u32;
}

fn loaded_dll() -> (String, String) {
    let h = unsafe { GetModuleHandleA(b"pskernel.dll\0".as_ptr()) };
    if h.is_null() {
        return ("<GetModuleHandleA failed>".into(), String::new());
    }
    let mut buf = [0u8; 1024];
    let n = unsafe { GetModuleFileNameA(h, buf.as_mut_ptr(), buf.len() as u32) } as usize;
    let path = String::from_utf8_lossy(&buf[..n]).to_string();
    let hash = match std::fs::read(&path) {
        Ok(bytes) => format!("{:x}", Sha256::digest(&bytes)),
        Err(e) => format!("<read failed: {e}>"),
    };
    (path, hash)
}

fn enc_name(e: i32) -> &'static str {
    match e {
        5701 => "inside",
        5702 => "outside",
        5703 => "on",
        5704 => "unset",
        _ => "?",
    }
}

struct Topo(HashMap<i32, String>);

impl Topo {
    fn of(b: &Body) -> Topo {
        let mut m = HashMap::new();
        for (i, f) in b.faces().unwrap().iter().enumerate() {
            m.insert(f.tag(), format!("face#{i}"));
        }
        for (i, e) in b.edges().unwrap().iter().enumerate() {
            m.insert(e.tag(), format!("edge#{i}"));
        }
        for (i, v) in b.vertices().unwrap().iter().enumerate() {
            m.insert(v.tag(), format!("vertex#{i}"));
        }
        for (i, r) in b.regions().unwrap().iter().enumerate() {
            m.insert(r.tag(), format!("region#{i}"));
        }
        Topo(m)
    }
    fn name(&self, t: i32) -> String {
        if t == 0 {
            return "null".into();
        }
        self.0
            .get(&t)
            .cloned()
            .unwrap_or_else(|| format!("tag{t}(not in body lists)"))
    }
}

fn cv(body: i32, p: [f64; 3]) -> (i32, i32, i32) {
    let mut enc: sys::PK_enclosure_t = 0;
    let mut topol: sys::PK_TOPOL_t = 0;
    let rc = unsafe { sys::PK_BODY_contains_vector(body, &p, &mut enc, &mut topol) };
    (rc, enc, topol)
}

fn row(label: &str, b: i32, topo: &Topo, p: [f64; 3]) {
    let (rc, enc, t) = cv(b, p);
    println!(
        "  {:44} p=({:+.17e},{:+.17e},{:+.17e}) rc={:<5} enc={} ({}) topol={}",
        label,
        p[0],
        p[1],
        p[2],
        rc,
        enc,
        enc_name(enc),
        topo.name(t)
    );
}

const KS: &[f64] = &[
    1e-12, 1e-11, 1e-10, 3e-10, 1e-9, 3e-9, 5e-9, 9e-9, 1e-8, 1.1e-8, 2e-8, 5e-8, 1e-7, 1e-6,
    1e-5,
];

fn sweep(label: &str, b: i32, topo: &Topo, base: [f64; 3], dir: [f64; 3]) {
    println!(" -- {label}: base=({},{},{}) dir=({:.6},{:.6},{:.6})", base[0], base[1], base[2], dir[0], dir[1], dir[2]);
    row("k=0", b, topo, base);
    for &k in KS {
        for s in [1.0f64, -1.0] {
            let p = [
                base[0] + s * k * dir[0],
                base[1] + s * k * dir[1],
                base[2] + s * k * dir[2],
            ];
            row(&format!("k={}{:e}", if s > 0.0 { "+" } else { "-" }, k), b, topo, p);
        }
    }
}

fn main() {
    let base_dir = "/tmp/claude-1000/-home-dev-projects-parasolid-re/82611b04-ad9e-4210-a557-7db62fa778dd/scratchpad/dynamic-session/xt";
    let cfg = SessionConfig::new()
        .check_arguments(true)
        .frustrum(FrustrumConfig::new().base_dir(base_dir));
    let s = Session::start(cfg).unwrap();
    let (path, hash) = loaded_dll();
    println!("loaded pskernel.dll: {path}\n  sha256 {hash}");
    println!("kernel version: {:?}", s.kernel_version());
    let mut beh = sys::PK_SESSION_behaviour_t::default();
    let rc = unsafe { sys::PK_SESSION_ask_behaviour(std::ptr::null(), &mut beh) };
    println!(
        "session behaviour: rc={rc} type={} value={}  precision={:?} angle={:?}",
        beh.behaviour_type,
        beh.behaviour_value,
        s.precision(),
        s.angle_precision()
    );

    // ---------------- block 10x20x30 ----------------
    let blk = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    let bb = blk.bounding_box().unwrap();
    println!(
        "\n== block 10x20x30: bbox ({},{},{})..({},{},{}) ==",
        bb.min.x, bb.min.y, bb.min.z, bb.max.x, bb.max.y, bb.max.z
    );
    let bt = Topo::of(&blk);
    let b = blk.tag();
    println!(" -- regular");
    row("interior (0.7,0.3,15.9)", b, &bt, [0.7, 0.3, 15.9]);
    row("centroid (0,0,15)", b, &bt, [0.0, 0.0, 15.0]);
    row("far (100,0,0)", b, &bt, [100.0, 0.0, 0.0]);
    row("far (0,0,-100)", b, &bt, [0.0, 0.0, -100.0]);
    println!(" -- exact boundary");
    row("face +X (5,1.3,15.7)", b, &bt, [5.0, 1.3, 15.7]);
    row("face -Z (1.3,2.1,0)", b, &bt, [1.3, 2.1, 0.0]);
    row("edge +X+Y (5,10,15.7)", b, &bt, [5.0, 10.0, 15.7]);
    row("vertex (5,10,30)", b, &bt, [5.0, 10.0, 30.0]);
    row("vertex (-5,-10,0)", b, &bt, [-5.0, -10.0, 0.0]);
    let r2 = std::f64::consts::FRAC_1_SQRT_2;
    let r3 = 1.0 / 3.0f64.sqrt();
    sweep("face +X along normal", b, &bt, [5.0, 1.3, 15.7], [1.0, 0.0, 0.0]);
    sweep("face +X tangential (in-plane, toward edge)", b, &bt, [5.0, 10.0 - 1e-3, 15.7], [0.0, 1.0, 0.0]);
    sweep("edge +X+Y along bisector", b, &bt, [5.0, 10.0, 15.7], [r2, r2, 0.0]);
    sweep("edge +X+Y along +X only", b, &bt, [5.0, 10.0, 15.7], [1.0, 0.0, 0.0]);
    sweep("vertex (5,10,30) along diagonal", b, &bt, [5.0, 10.0, 30.0], [r3, r3, r3]);

    // ---------------- sphere r=10 (curved face) ----------------
    let sph = Body::create_solid_sphere(10.0).unwrap();
    let sb = sph.bounding_box().unwrap();
    println!(
        "\n== sphere r=10: bbox ({},{},{})..({},{},{}) faces={} edges={} vertices={} ==",
        sb.min.x,
        sb.min.y,
        sb.min.z,
        sb.max.x,
        sb.max.y,
        sb.max.z,
        sph.faces().unwrap().len(),
        sph.edges().unwrap().len(),
        sph.vertices().unwrap().len()
    );
    let st = Topo::of(&sph);
    row("centre (0,0,0)", sph.tag(), &st, [0.0, 0.0, 0.0]);
    row("pole (0,0,10)", sph.tag(), &st, [0.0, 0.0, 10.0]);
    sweep("sphere (6,0,8) radial", sph.tag(), &st, [6.0, 0.0, 8.0], [0.6, 0.0, 0.8]);

    // ---------------- cylinder r=5 h=10 (circular edge) ----------------
    let cyl = Body::create_solid_cylinder(5.0, 10.0).unwrap();
    let cb = cyl.bounding_box().unwrap();
    println!(
        "\n== cylinder r=5 h=10: bbox ({},{},{})..({},{},{}) ==",
        cb.min.x, cb.min.y, cb.min.z, cb.max.x, cb.max.y, cb.max.z
    );
    let ct = Topo::of(&cyl);
    sweep("cyl top circular edge (5,0,top) bisector", cyl.tag(), &ct, [5.0, 0.0, cb.max.z], [r2, 0.0, r2]);
    row("cyl axis on top cap (0,0,top)", cyl.tag(), &ct, [0.0, 0.0, cb.max.z]);

    // ---------------- precision dependence ----------------
    for p in [1e-6f64, 1e-10] {
        let rc = unsafe { sys::PK_SESSION_set_precision(p) };
        println!("\n== set_precision({p:e}) rc={rc}; block face +X sweep ==");
        sweep("face +X along normal", b, &bt, [5.0, 1.3, 15.7], [1.0, 0.0, 0.0]);
        sweep("sphere (6,0,8) radial", sph.tag(), &st, [6.0, 0.0, 8.0], [0.6, 0.0, 0.8]);
    }
    let rc = unsafe { sys::PK_SESSION_set_precision(1e-8) };
    println!("restore set_precision(1e-8) rc={rc}");

    // ---------------- sheet and minimum bodies ----------------
    let sheet = Body::create_sheet_rectangle(
        4.0,
        6.0,
        Axis2::new(Vec3::new(20.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
    )
    .unwrap();
    let sht = Topo::of(&sheet);
    println!("\n== sheet rectangle 4x6 at z=0 centred (20,0,0): type={:?} ==", sheet.body_type());
    row("on sheet (20.5,0.5,0)", sheet.tag(), &sht, [20.5, 0.5, 0.0]);
    row("sheet +1e-9 z", sheet.tag(), &sht, [20.5, 0.5, 1e-9]);
    row("sheet +1e-7 z", sheet.tag(), &sht, [20.5, 0.5, 1e-7]);
    row("off sheet (20.5,0.5,1)", sheet.tag(), &sht, [20.5, 0.5, 1.0]);
    row("sheet edge (22,0.5,0)", sheet.tag(), &sht, [22.0, 0.5, 0.0]);
    let minb = Body::create_minimum(Vec3::new(0.0, 0.0, 50.0)).unwrap();
    let mt = Topo::of(&minb);
    println!("\n== minimum body at (0,0,50): type={:?} ==", minb.body_type());
    row("at the point", minb.tag(), &mt, [0.0, 0.0, 50.0]);
    row("1e-9 off", minb.tag(), &mt, [1e-9, 0.0, 50.0]);
    row("1e-7 off", minb.tag(), &mt, [1e-7, 0.0, 50.0]);

    // ---------------- invalid / extreme inputs ----------------
    println!("\n== invalid / extreme inputs (block) ==");
    row("size-box edge (499,0,0)", b, &bt, [499.0, 0.0, 0.0]);
    row("outside size box (600,0,0)", b, &bt, [600.0, 0.0, 0.0]);
    row("far outside (1e6,0,0)", b, &bt, [1e6, 0.0, 0.0]);
    row("huge (1e300,0,0)", b, &bt, [1e300, 0.0, 0.0]);
    row("+inf x", b, &bt, [f64::INFINITY, 0.0, 0.0]);
    row("NaN x", b, &bt, [f64::NAN, 0.0, 15.0]);
    let face_tag = blk.faces().unwrap()[0].tag();
    row("face tag as body", face_tag, &bt, [0.0, 0.0, 15.0]);
    row("null tag (0)", 0, &bt, [0.0, 0.0, 15.0]);
    row("bogus tag (999999)", 999999, &bt, [0.0, 0.0, 15.0]);
    let tmp = Body::create_solid_block(1.0, 1.0, 1.0).unwrap();
    let dead = tmp.tag();
    tmp.delete().unwrap();
    row("deleted body tag", dead, &bt, [0.0, 0.0, 0.5]);
    println!("\ndone");
}
