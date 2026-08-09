//! Reusable `o_t_version` sweep for Parasolid option structs (Stage 0 item 4).
//!
//! Parasolid option structs are versioned: the caller stamps `o_t_version` in
//! the first 4 bytes, and the kernel runs a *migration routine* that copies the
//! caller's (older, smaller) user struct into its current internal layout. The
//! consequence that keeps biting us: **the struct the docs describe is the
//! internal one, not the one you are supposed to pass.** `PK_BODY_boolean_2`
//! modelled the 192-byte v19 internal layout and stamped version 1; the kernel
//! rejected it outright, and the real user struct turned out to be 32 bytes.
//!
//! This binary implements step 1 of the protocol in
//! `docs/option-version-protocol.md`: find which versions the kernel accepts,
//! empirically, before anyone decompiles anything.
//!
//! Reading the results:
//!   - `PK_ERROR_o_t_version_unknown` (5022) — the version is outside the range
//!     this build knows about.
//!   - `PK_ERROR_o_t_version_incorrect` (5043) — the version is known but does
//!     not match what this entry point accepts.
//!   - any other error — the version was accepted and the call failed later,
//!     which for this purpose counts as accepted.
//!
//! Run under Wine:
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/option_version_probe.exe

use parasolid::*;
use parasolid_sys::*;

/// Classify one attempt at a given `o_t_version`.
#[derive(PartialEq, Clone, Copy)]
enum Verdict {
    Accepted,
    Unknown,
    Incorrect,
}

fn classify(rc: PK_ERROR_code_t) -> Verdict {
    match rc {
        c if c == PK_ERROR_o_t_version_unknown => Verdict::Unknown,
        c if c == PK_ERROR_o_t_version_incorrect => Verdict::Incorrect,
        _ => Verdict::Accepted,
    }
}

/// Sweep `o_t_version` over `range` and report the accepted set.
///
/// `attempt` receives the version to stamp and returns the raw PK code. It is
/// responsible for building an otherwise-plausible options buffer.
fn sweep(label: &str, range: std::ops::RangeInclusive<i32>, mut attempt: impl FnMut(i32) -> i32) {
    println!("\n=== {label}");
    let mut accepted = Vec::new();
    let mut unknown = 0usize;
    let mut incorrect = 0usize;

    for v in range.clone() {
        match classify(attempt(v)) {
            Verdict::Accepted => accepted.push(v),
            Verdict::Unknown => unknown += 1,
            Verdict::Incorrect => incorrect += 1,
        }
        let mut cleared: PK_LOGICAL_t = PK_LOGICAL_false;
        unsafe { PK_ERROR_clear_last(&mut cleared) };
    }

    let span = |v: &Vec<i32>| -> String {
        match (v.first(), v.last()) {
            (Some(a), Some(b)) if v.len() as i32 == b - a + 1 => format!("{a}..={b}"),
            (Some(_), Some(_)) => format!("{v:?}"),
            _ => "none".to_string(),
        }
    };
    println!(
        "  swept {:?}: accepted {} ({}), unknown {unknown}, incorrect {incorrect}",
        range,
        accepted.len(),
        span(&accepted)
    );
}

fn main() {
    let _session = Session::start(SessionConfig::new().check_arguments(true)).expect("session");

    // --- Worked example 1: PK_TOPOL_eval_mass_props -------------------------
    // The v1 user struct {o_t_version, mass, periphery, bound, single} is
    // already validated; this pins the accepted version window around it.
    let body = Body::create_solid_block(2.0, 2.0, 2.0).expect("block");
    #[repr(C)]
    struct MassOpts {
        o_t_version: i32,
        mass: i32,
        periphery: i32,
        bound: i32,
        single: u8,
    }
    sweep("PK_TOPOL_eval_mass_props_o_t", 0..=24, |v| {
        let opts = MassOpts {
            o_t_version: v,
            mass: 0x36b4,
            periphery: 0x36b6,
            bound: 0x36b7,
            single: 1,
        };
        let (mut amount, mut mass, mut periphery) = (0.0f64, 0.0f64, 0.0f64);
        let mut c_of_g = [0.0f64; 3];
        let mut m_of_i = [0.0f64; 9];
        let tag = body.tag();
        unsafe {
            PK_TOPOL_eval_mass_props(
                1,
                &tag,
                0.99,
                &opts as *const MassOpts as *const PK_TOPOL_eval_mass_props_o_t,
                &mut amount,
                &mut mass,
                &mut c_of_g,
                &mut m_of_i,
                &mut periphery,
            )
        }
    });

    // --- Worked example 2: PK_BODY_boolean_2 --------------------------------
    // The 32-byte v2 user struct, the case that motivated the protocol.
    #[repr(C)]
    struct BoolOpts {
        o_t_version: i32,
        function: i32,
        configuration: *const std::ffi::c_void,
        default_tol: f64,
        flags: [u8; 3],
        _pad: u8,
        fence: i32,
    }
    // The version check happens before any modelling work, so the operands only
    // need to be *valid* — they must not be cheap to the point of erroring
    // first, nor expensive enough to matter. Fresh bodies per attempt, never
    // reused: a successful boolean consumes its target, and touching a consumed
    // tag afterwards is what turns this probe into a crash.
    sweep("PK_BODY_boolean_2_o_t", 0..=24, |v| {
        let target = Body::create_solid_block(10.0, 10.0, 10.0).expect("target");
        let tool = Body::create_solid_block(4.0, 4.0, 20.0).expect("tool");
        let opts = BoolOpts {
            o_t_version: v,
            function: 15902, // subtract
            configuration: std::ptr::null(),
            default_tol: 0.0,
            flags: [0; 3],
            _pad: 0,
            fence: 0,
        };
        let tool_tag = tool.tag();
        let mut results: [u8; 256] = [0; 256];
        let mut tracking: [u8; 256] = [0; 256];
        unsafe {
            PK_BODY_boolean_2(
                target.tag(),
                1,
                &tool_tag,
                &opts as *const BoolOpts as *const PK_BODY_boolean_o_t,
                tracking.as_mut_ptr() as *mut PK_TOPOL_track_r_t,
                results.as_mut_ptr() as *mut PK_boolean_r_t,
            )
        }
    });

    println!("\n=== done");
}
