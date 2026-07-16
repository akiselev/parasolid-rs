//! TRACK A / AGENT 2 — LIVE-MEMORY ground truth for the RES_* tolerance family.
//!
//! libpskernel.a is an IMPORT LIBRARY (966 KB, all __imp_ thunks): the real
//! pskernel.dll (sha256 c900fa3f430f…, byte-identical to the Ghidra-analyzed
//! binary, image base 0x180000000) is loaded into THIS process at runtime by the
//! Windows/Wine loader. So every DAT_1845d3xxx VA is directly readable here at
//!     runtime_addr = GetModuleHandleA("pskernel.dll") + (VA - 0x180000000)
//! No winedbg, no symbols, no rebasing guesswork needed.
//!
//! Cross-validates A1's static formula  DAT_1845d3220 = 10 * L / T  where
//!   L = DAT_1845d32e0[ctx] (per-context length scale), T = DAT_1845d3238 (precision),
//! and the sibling family (0.5·, 100·, 1/·).

use parasolid::*;
use parasolid_sys as sys;

const IMAGE_BASE: u64 = 0x1_8000_0000;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleA(name: *const u8) -> *mut core::ffi::c_void;
}

fn dll_base() -> u64 {
    let name = b"pskernel.dll\0";
    (unsafe { GetModuleHandleA(name.as_ptr()) }) as u64
}

#[inline]
unsafe fn rd(base: u64, va: u64) -> f64 {
    let addr = base + (va - IMAGE_BASE);
    core::ptr::read_volatile(addr as *const f64)
}

// The RES_* precision-derived family (VAs are DLL VAs, image base 0x180000000).
const FAM: &[(&str, u64)] = &[
    ("DAT_1845d31e8  0.5*L/T", 0x1845d31e8),
    ("DAT_1845d3210  1/(10L/T)=T/10L", 0x1845d3210),
    ("DAT_1845d3218  100*L/T", 0x1845d3218),
    ("DAT_1845d3220  10*L/T  <==TARGET", 0x1845d3220),
    ("DAT_1845d3228  L/T (base ratio)", 0x1845d3228),
    ("DAT_1845d3230  T^2", 0x1845d3230),
    ("DAT_1845d3238  T (current prec)", 0x1845d3238),
    ("DAT_1845d3248  T (session prec)", 0x1845d3248),
    ("DAT_1845d3260  L^2 (ctx0)", 0x1845d3260),
    ("DAT_1845d32e0  L (length scale ctx0)", 0x1845d32e0),
];

fn dump(base: u64, tag: &str) {
    let t = unsafe { rd(base, 0x1845d3238) };
    let l = unsafe { rd(base, 0x1845d32e0) };
    println!("  [{}]  measured L={:.6e}  T={:.6e}", tag, l, t);
    for (n, va) in FAM {
        let v = unsafe { rd(base, *va) };
        println!("    {:34} = {:>14.6e}", n, v);
    }
    // A1 formula predictions from the measured L,T:
    println!("    PREDICT from 10*L/T = {:.6e}   100*L/T = {:.6e}   0.5*L/T = {:.6e}   T/(10L) = {:.6e}",
        10.0 * l / t, 100.0 * l / t, 0.5 * l / t, t / (10.0 * l));
}

fn main() {
    let cfg = SessionConfig::new().check_arguments(false).frustrum(
        FrustrumConfig::new().base_dir(
            "/tmp/claude-1000/-home-dev-projects-parasolid-re/ccf7881f-5d90-471c-9b56-208ecdb81733/scratchpad/xt",
        ),
    );
    let s = Session::start(cfg).unwrap();

    let base = dll_base();
    println!(
        "pskernel.dll base = {:#x}   preferred = {:#x}   reloc delta = {:#x}",
        base,
        IMAGE_BASE,
        base.wrapping_sub(IMAGE_BASE) as i64
    );
    if base == 0 {
        println!("!! GetModuleHandleA returned NULL — module name mismatch; cannot read memory");
        return;
    }
    println!(
        "PK_SESSION_ask_precision (API) = {:?}   angle = {:?}",
        s.precision(),
        s.angle_precision()
    );

    println!("\n== DEFAULT (post PK_SESSION_start, no override) ==");
    dump(base, "default");

    // Sweep linear precision; confirm the family tracks the numerator (DAT_32e0) 1:1.
    for p in [1e-6_f64, 1e-7, 1e-8, 1e-9, 1e-10] {
        let code = unsafe { sys::PK_SESSION_set_precision(p) };
        let mut got = 0.0f64;
        let _ = unsafe { sys::PK_SESSION_ask_precision(&mut got) };
        println!("\n== set_precision({:.0e}) -> code {} ; ask_precision={:.6e} ==", p, code, got);
        dump(base, &format!("T={:.0e}", p));
    }

    // Reset linear to default, then sweep ANGULAR precision to identify DAT_3238.
    let _ = unsafe { sys::PK_SESSION_set_precision(1e-8) };
    println!("\n\n#### ANGULAR SWEEP (linear reset to 1e-8) — is DAT_1845d3238 the angular precision? ####");
    for a in [1e-11_f64, 1e-9, 1e-12, 1e-10, 1e-11] {
        let code = unsafe { sys::PK_SESSION_set_angle_precision(a) };
        let mut ga = 0.0f64;
        let _ = unsafe { sys::PK_SESSION_ask_angle_precision(&mut ga) };
        let d3238 = unsafe { rd(base, 0x1845d3238) };
        let d3230 = unsafe { rd(base, 0x1845d3230) };
        let d3220 = unsafe { rd(base, 0x1845d3220) };
        let d3228 = unsafe { rd(base, 0x1845d3228) };
        let d32e0 = unsafe { rd(base, 0x1845d32e0) };
        println!(
            "  set_angle({:.0e}) code={} ask_angle={:.3e} | DAT_3238={:.3e} DAT_3230(=3238^2)={:.3e} \
             DAT_32e0(lin)={:.3e} DAT_3228(=32e0/3238)={:.3e} DAT_3220(=10*3228)={:.3e}",
            a, code, ga, d3238, d3230, d32e0, d3228, d3220
        );
    }
}
