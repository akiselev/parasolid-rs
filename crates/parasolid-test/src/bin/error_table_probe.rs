//! Recover the `PK_ERROR_*` code → token table from the DLL (Stage 0 item 1).
//!
//! `parasolid-sys` carries ~25 hand-transcribed `PK_ERROR_*` numeric constants,
//! several of which are demonstrably wrong (`distance_le_0` is 15, not 502;
//! `not_an_entity` is 22, not 504 — both confirmed by `error_probe`, which reads
//! the kernel's own `code_token` string) and several of which collide
//! (`system_error`/`modeller_not_started` both 2; `not_implemented`/
//! `cant_heal_wound` both 600). Guessed values are worse than missing ones: the
//! wrapper dispatches `PK_ERROR_not_an_entity` on a code the kernel never emits.
//!
//! Method: `PK_ERROR_raise` takes a `PK_ERROR_sf_t` *by value* (>16 bytes, so
//! passed by hidden pointer on Win64 — our `*const` binding is ABI-correct). We
//! raise a bare code with empty strings and ask whether the kernel fills in the
//! canonical `code_token` on readback. If it does, looping over a code range
//! recovers the whole table.
//!
//! Output is flushed per line so a crash still leaves the evidence gathered so
//! far. Run under Wine:
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/error_table_probe.exe

use std::io::Write;

use parasolid::*;
use parasolid_sys::*;

fn inline_str(buf: &[u8], off: usize, len: usize) -> String {
    let s = &buf[off..off + len];
    let end = s.iter().position(|&b| b == 0).unwrap_or(len);
    String::from_utf8_lossy(&s[..end]).into_owned()
}

/// Raise `code` with empty strings, then read back what the kernel recorded.
/// Returns `(code_token, severity, readback_code)`.
fn raise_and_read(code: i32) -> Option<(String, i32, i32)> {
    let mut sf = [0u8; 116];
    sf[32..36].copy_from_slice(&code.to_le_bytes());
    // severity @68 = mild, so a raise cannot escalate into a session kill.
    sf[68..72].copy_from_slice(&1i32.to_le_bytes());
    sf[108..112].copy_from_slice(&(-1i32).to_le_bytes());

    unsafe { PK_ERROR_raise(sf.as_ptr() as *const PK_ERROR_sf_t) };

    let mut buf = [0u8; 256];
    let mut was_error: PK_LOGICAL_t = PK_LOGICAL_false;
    let rc = unsafe { PK_ERROR_ask_last(&mut was_error, buf.as_mut_ptr() as *mut PK_ERROR_sf_t) };
    if rc != PK_ERROR_no_errors || was_error != PK_LOGICAL_true {
        return None;
    }
    let token = inline_str(&buf, 36, 32);
    let severity = i32::from_le_bytes(buf[68..72].try_into().unwrap());
    let readback = i32::from_le_bytes(buf[32..36].try_into().unwrap());

    let mut cleared: PK_LOGICAL_t = PK_LOGICAL_false;
    unsafe { PK_ERROR_clear_last(&mut cleared) };
    Some((token, severity, readback))
}

fn main() {
    let _session = Session::start(SessionConfig::new()).expect("session");
    let mut out = std::io::stdout();

    // Feasibility check: does the kernel canonicalize a raised bare code?
    println!("== feasibility: raise a bare code and see if the token is filled in");
    for code in [15, 22, 5022] {
        match raise_and_read(code) {
            Some((token, sev, back)) => {
                println!("  code {code:5} -> token {token:?}, severity {sev}, readback {back}")
            }
            None => println!("  code {code:5} -> no record"),
        }
    }
    let _ = out.flush();

    let canonicalizes = raise_and_read(15)
        .map(|(t, _, _)| t == "PK_ERROR_distance_le_0")
        .unwrap_or(false);
    println!(
        "\n  kernel canonicalizes raised codes: {}\n",
        if canonicalizes {
            "YES — sweeping the range"
        } else {
            "NO — raise echoes our struct; table must come from the DLL statically"
        }
    );
    let _ = out.flush();

    if !canonicalizes {
        println!("== falling back to trigger-based recovery (real errors only) ==");
        return;
    }

    println!("== code -> token sweep ==");
    let mut found = 0usize;
    for code in 0..=9000 {
        if let Some((token, sev, back)) = raise_and_read(code)
            && !token.is_empty()
            && back == code
            && !token.starts_with("Unknown error code")
        {
            println!("  {code:5}  sev={sev}  {token}");
            found += 1;
            let _ = out.flush();
        }
    }
    println!("\n== {found} codes recovered ==");
}
