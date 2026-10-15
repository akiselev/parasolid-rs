//! Dynamic probe: session behaviour (PK_SESSION_ask/set_behaviour), the
//! original-system session switches, and whether the internal version index
//! changes observable results.
//!
//! Signatures per the PK reference pages:
//!   PK_SESSION_ask_behaviour(const PK_SESSION_ask_behaviour_o_t *, PK_SESSION_behaviour_t *)
//!   PK_SESSION_set_behaviour(PK_SESSION_behaviour_t requested, const ..._o_t *,
//!                            PK_SESSION_behaviour_t *set, PK_SESSION_behaviour_t *previous,
//!                            PK_behaviour_status_t *status)
//!   PK_SESSION_ask_latest_behaviour(PK_SESSION_behaviour_t *)
//!   PK_SESSION_ask/set_software_option(PK_SESSION_software_option_t, int / int *)
//! PK_SESSION_behaviour_t = { int behaviour_type; int behaviour_value; }.
//! Options pointers are NULL (the decompile of each function substitutes defaults).
//!
//! Memory is only READ (never written): the session node pointer is
//! *(DAT_1845caf50) + 0x10 and the index/mode fields are +0x5c/+0x60, per the
//! static evidence in parasolid-re next7/dynamic-session/static/.

use parasolid::*;
use parasolid_sys as sys;
use sha2::{Digest, Sha256};

const IMAGE_BASE: u64 = 0x1_8000_0000;

// The PK header declares `option` as the token type PK_SESSION_software_option_t
// (an int token, e.g. PK_SESSION_pre_v281_switch_0_c = 5200); the decompile of both
// functions compares it against token constants. parasolid-sys models it as a
// 4-byte struct, so declare the int form locally.
#[link(name = "pskernel")]
unsafe extern "C" {
    #[link_name = "PK_SESSION_ask_software_option"]
    fn pk_ask_swopt(option: i32, ival: *mut i32) -> i32;
    #[link_name = "PK_SESSION_set_software_option"]
    fn pk_set_swopt(option: i32, ival: i32) -> i32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut core::ffi::c_void;
    fn GetModuleFileNameA(module: *mut core::ffi::c_void, buf: *mut u8, size: u32) -> u32;
}

fn base() -> u64 {
    (unsafe { GetModuleHandleA(b"pskernel.dll\0".as_ptr()) }) as u64
}

unsafe fn rd<T: Copy>(addr: u64) -> T {
    core::ptr::read_volatile(addr as *const T)
}

fn va(b: u64, v: u64) -> u64 {
    b + (v - IMAGE_BASE)
}

/// (index @+0x5c, mode @+0x60, n set bits in switch block @+0x70, int-switch list head @+0x40,
///  thread-0 slot index, thread-0 slot mode, override global)
fn mem(tag: &str) {
    let b = base();
    unsafe {
        let root: u64 = rd(va(b, 0x1845caf50));
        let over: i32 = rd(va(b, 0x18439991c));
        let t0_idx: i32 = rd(va(b, 0x1845cb068));
        let t0_mode: i32 = rd(va(b, 0x1845cb06c));
        if root == 0 {
            println!("  [mem {tag}] root node pointer is NULL");
            return;
        }
        let sess: u64 = rd(root + 0x10);
        let idx: i32 = rd(sess + 0x5c);
        let mode: i32 = rd(sess + 0x60);
        let list: u64 = rd(sess + 0x40);
        let sw: u64 = rd(sess + 0x70);
        let mut set_bits = Vec::new();
        if sw != 0 {
            for w in 0..25u64 {
                let word: u32 = rd(sw + 4 * w);
                for bit in 0..32 {
                    if word & (1 << bit) != 0 {
                        set_bits.push(format!("{:#x}", w * 32 + bit));
                    }
                }
            }
        }
        println!(
            "  [mem {tag}] session+0x5c index={idx:#x} ({idx})  +0x60 mode={mode}  +0x40 list={list:#x}  \
             switch bits set={:?}  thread0 slot idx={t0_idx:#x} mode={t0_mode}  override DAT_18439991c={over}",
            set_bits
        );
    }
}

fn ask(tag: &str) {
    let mut b = sys::PK_SESSION_behaviour_t::default();
    let rc = unsafe { sys::PK_SESSION_ask_behaviour(std::ptr::null(), &mut b) };
    println!(
        "  [ask {tag}] rc={rc} type={} value={}",
        b.behaviour_type, b.behaviour_value
    );
}

fn set(ty: i32, value: i32) {
    let req = sys::PK_SESSION_behaviour_t {
        behaviour_type: ty,
        behaviour_value: value,
    };
    let mut s = sys::PK_SESSION_behaviour_t {
        behaviour_type: -1,
        behaviour_value: -1,
    };
    let mut p = s;
    let mut st: sys::PK_behaviour_status_t = -1;
    let rc = unsafe { sys::PK_SESSION_set_behaviour(req, std::ptr::null(), &mut s, &mut p, &mut st) };
    println!(
        "set_behaviour(type={ty}, value={value}) -> rc={rc} set=({},{}) previous=({},{}) status={st}",
        s.behaviour_type, s.behaviour_value, p.behaviour_type, p.behaviour_value
    );
}

fn swopt(tok: i32, name: &str) -> i32 {
    let mut v: i32 = -12345;
    let rc = unsafe { pk_ask_swopt(tok, &mut v) };
    println!("  ask_software_option({tok} {name}) rc={rc} ival={v}");
    v
}

fn set_swopt(tok: i32, name: &str, v: i32) {
    let rc = unsafe { pk_set_swopt(tok, v) };
    println!("set_software_option({tok} {name}, {v}) -> rc={rc}");
}

fn battery(tag: &str) {
    println!("  [battery {tag}]");
    // 1. faceting with default control
    match Body::create_solid_sphere(10.0).and_then(|b| b.facet()) {
        Ok(m) => println!("    facet sphere r=10 default: n_facets={}", m.n_facets),
        Err(e) => println!("    facet sphere: ERR {e}"),
    }
    match Body::create_solid_torus(10.0, 3.0).and_then(|b| b.facet()) {
        Ok(m) => println!("    facet torus 10/3 default: n_facets={}", m.n_facets),
        Err(e) => println!("    facet torus: ERR {e}"),
    }
    // 2. surface/surface intersection result types
    let o = Vec3::zero();
    let z = Vec3::new(0.0, 0.0, 1.0);
    let x = Vec3::new(1.0, 0.0, 0.0);
    let y = Vec3::new(0.0, 1.0, 0.0);
    let pairs: Vec<(&str, PsResult<SurfIntersection>)> = vec![
        ("cyl(Z,5) x cyl(X,4)", (|| {
            let a = Surf::cylinder(Axis2::new(o, z, x), 5.0)?;
            let b = Surf::cylinder(Axis2::new(o, x, y), 4.0)?;
            a.intersect(&b)
        })()),
        ("cyl(Z,5) x cyl(X,5)", (|| {
            let a = Surf::cylinder(Axis2::new(o, z, x), 5.0)?;
            let b = Surf::cylinder(Axis2::new(o, x, y), 5.0)?;
            a.intersect(&b)
        })()),
        ("sphere(8) x cyl(X,3) offset", (|| {
            let a = Surf::sphere(Axis2::new(o, z, x), 8.0)?;
            let b = Surf::cylinder(Axis2::new(Vec3::new(0.0, 0.0, 6.0), x, y), 3.0)?;
            a.intersect(&b)
        })()),
    ];
    for (l, r) in pairs {
        match r {
            Ok(si) => {
                let types: Vec<String> = si
                    .curves
                    .iter()
                    .map(|c| format!("{:?}", c.curve.curve_type()))
                    .collect();
                println!("    {l}: {} pts {} curves {:?}", si.points.len(), si.curves.len(), types);
            }
            Err(e) => println!("    {l}: ERR {e}"),
        }
    }
    // 3. fillet volume
    let r = (|| {
        let b = Body::create_solid_block(10.0, 10.0, 10.0)?;
        let e = b.edges()?;
        let n = b.fillet_edges(&e[..1], 2.0)?;
        Ok::<_, PsError>((n, b.volume()?, b.faces()?.len()))
    })();
    println!("    fillet 1 edge r=2 of 10^3 block: {:?}", r.map(|(n, v, f)| format!("rc/n={n} vol={v:.15} faces={f}")));
    // 4. containment band
    if let Ok(b) = Body::create_solid_block(10.0, 20.0, 30.0) {
        for k in [3e-9, 5e-9, -9e-9, -1e-8] {
            let p = [5.0 + k, 1.3, 15.7];
            let mut enc = 0;
            let mut t = 0;
            let rc = unsafe { sys::PK_BODY_contains_vector(b.tag(), &p, &mut enc, &mut t) };
            println!("    contains face+X offset {k:+e}: rc={rc} enc={enc}");
        }
    }
}

fn main() {
    let base_dir = "/tmp/claude-1000/-home-dev-projects-parasolid-re/82611b04-ad9e-4210-a557-7db62fa778dd/scratchpad/dynamic-session/xt";
    let s = Session::start(
        SessionConfig::new()
            .check_arguments(true)
            .frustrum(FrustrumConfig::new().base_dir(base_dir)),
    )
    .unwrap();
    {
        let h = unsafe { GetModuleHandleA(b"pskernel.dll\0".as_ptr()) };
        let mut buf = [0u8; 1024];
        let n = unsafe { GetModuleFileNameA(h, buf.as_mut_ptr(), 1024) } as usize;
        let path = String::from_utf8_lossy(&buf[..n]).to_string();
        let hash = std::fs::read(&path)
            .map(|b| format!("{:x}", Sha256::digest(&b)))
            .unwrap_or_else(|e| format!("<{e}>"));
        println!("loaded pskernel.dll: {path}\n  sha256 {hash}\n  base {:#x}", base());
    }
    println!("kernel version: {:?}", s.kernel_version());
    let mut lat = sys::PK_SESSION_behaviour_t::default();
    let rc = unsafe { sys::PK_SESSION_ask_latest_behaviour(&mut lat) };
    println!("ask_latest_behaviour rc={rc} type={} value={}", lat.behaviour_type, lat.behaviour_value);

    println!("\n== A. wrapper session (Session::start applied set_behaviour(latest)) ==");
    ask("A");
    mem("A");

    println!("\n== B. raw PK_SESSION_stop + PK_SESSION_start(NULL): fresh session, no set_behaviour ==");
    let rc1 = unsafe { sys::PK_SESSION_stop() };
    let rc2 = unsafe { sys::PK_SESSION_start(std::ptr::null()) };
    println!("stop rc={rc1} start rc={rc2}");
    ask("B fresh");
    mem("B fresh");
    let sw: &[(i32, &str)] = &[
        (4211, "boolean_pre_v141 (idx 0xb)"),
        (4221, "pre_v170_switch_1 (idx 0x15)"),
        (4602, "pre_v231_switch_2 (idx 0xc7)"),
        (4607, "pre_v231_switch_7 (idx 0xcc)"),
        (4750, "pre_v250_switch_0 (idx 0x116)"),
        (4760, "pre_v250_switch_10 (idx 0x120)"),
        (4900, "pre_v261_switch_0 (idx 0x173)"),
        (5100, "pre_v271_switch_0 (idx 0x213)"),
        (5200, "pre_v281_switch_0 (idx 0x251)"),
        (5250, "pre_v290_switch_0 (idx 0x270)"),
        (5300, "pre_v291_switch_0 (idx 0x28f)"),
        (5350, "pre_v300_switch_0 (idx 0x2ae)"),
        (5400, "pre_v301_switch_0 (idx 0x2cd)"),
    ];
    println!(" -- fresh software-option values");
    for (t, n) in sw {
        swopt(*t, n);
    }
    battery("B fresh/unset");

    println!("\n== C. set_behaviour sweep ==");
    let cases: &[(i32, i32)] = &[
        (25841, 0),      // latest
        (25840, 0),      // unset
        (25842, 370108), // value: latest patch
        (25842, 370100), // V37.1 FCS
        (25842, 370109), // beyond latest patch
        (25842, 370199),
        (25842, 379999),
        (25842, 380000),
        (25842, 999999),
        (25842, 1000000),
        (25842, 360105),
        (25842, 300000),
        (25842, 280100),
        (25842, 280099),
        (25842, 0),
        (25842, -1),
        (25841, 370108), // latest with nonzero value
        (25840, 1),      // unset with nonzero value
        (25843, 0),      // unknown type
        (0, 0),
    ];
    for &(t, v) in cases {
        set(t, v);
        ask("after");
        mem("after");
    }

    println!("\n== D. unset + original-system switches ==");
    set(25840, 0);
    mem("unset");
    for (t, n) in [(5200, "pre_v281_switch_0"), (5400, "pre_v301_switch_0"), (5100, "pre_v271_switch_0")] {
        set_swopt(t, n, 1);
        swopt(t, n);
        ask("after switch on");
        mem("after switch on");
        set_swopt(t, n, 0);
        mem("after switch off");
    }
    println!(" -- switch on under latest mode");
    set(25841, 0);
    set_swopt(5200, "pre_v281_switch_0", 1);
    mem("latest + v281 switch");
    set_swopt(5200, "pre_v281_switch_0", 0);

    println!("\n== E. behaviour-sensitive battery per mode ==");
    set(25841, 0);
    mem("E latest");
    battery("latest 370108");
    for v in [370100, 300000, 280100] {
        set(25842, v);
        mem("E value");
        battery(&format!("value {v}"));
    }
    set(25840, 0);
    set_swopt(5200, "pre_v281_switch_0", 1);
    mem("E unset+v281");
    battery("unset + pre_v281_switch_0");
    set_swopt(5200, "pre_v281_switch_0", 0);
    set(25841, 0);
    println!("\ndone");
}
