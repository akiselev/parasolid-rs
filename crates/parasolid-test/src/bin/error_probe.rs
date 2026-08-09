//! Raw-byte probe for `PK_ERROR_sf_t` (Stage 0 item 1).
//!
//! `parasolid-sys` carries a *journal-derived* 116-byte layout for
//! `PK_ERROR_sf_t` (function@0, code@32, code_token@36, severity@68,
//! argument_number@72, argument_name@76, argument_index@108, entity@112), but
//! the high-level wrapper still refuses to trust anything past `code` and
//! guesses severity from the code alone. This probe supplies the runtime
//! evidence to close that gap: it raises several *distinct* kinds of error and
//! dumps the raw buffer `PK_ERROR_ask_last` writes, annotated at the candidate
//! offsets.
//!
//! Run under Wine:
//!   cargo build -p parasolid-test --target x86_64-pc-windows-gnu
//!   WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/error_probe.exe

use parasolid::*;
use parasolid_sys::*;

const BUF: usize = 512;

/// Read the inline NUL-terminated string at `off`, bounded by `len`.
fn inline_str(buf: &[u8], off: usize, len: usize) -> String {
    let s = &buf[off..off + len];
    let end = s.iter().position(|&b| b == 0).unwrap_or(len);
    String::from_utf8_lossy(&s[..end]).into_owned()
}

fn i32_at(buf: &[u8], off: usize) -> i32 {
    i32::from_le_bytes(buf[off..off + 4].try_into().unwrap())
}

fn hexdump(buf: &[u8], n: usize) {
    for row in 0..n.div_ceil(16) {
        let off = row * 16;
        let end = (off + 16).min(n);
        let hex: Vec<String> = buf[off..end].iter().map(|b| format!("{b:02x}")).collect();
        let ascii: String = buf[off..end]
            .iter()
            .map(|&b| {
                if (0x20..0x7f).contains(&b) {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();
        println!("    {off:04x}  {:<47}  |{ascii}|", hex.join(" "));
    }
}

/// Raise an error, then dump whatever `PK_ERROR_ask_last` reports.
fn probe(label: &str, trigger: impl FnOnce() -> PK_ERROR_code_t) {
    println!("\n=== {label}");
    let rc = trigger();
    println!("  return code            = {rc}");

    let mut buf = [0u8; BUF];
    let mut was_error: PK_LOGICAL_t = PK_LOGICAL_false;
    let ask = unsafe { PK_ERROR_ask_last(&mut was_error, buf.as_mut_ptr() as *mut PK_ERROR_sf_t) };
    println!("  PK_ERROR_ask_last rc   = {ask}, was_error = {was_error}");
    if ask != PK_ERROR_no_errors || was_error != PK_LOGICAL_true {
        println!("  (no error record available)");
        return;
    }

    // Interpretation at the journal-derived offsets.
    println!("  function        @0     = {:?}", inline_str(&buf, 0, 32));
    println!("  code            @32    = {}", i32_at(&buf, 32));
    println!("  code_token      @36    = {:?}", inline_str(&buf, 36, 32));
    println!("  severity        @68    = {}", i32_at(&buf, 68));
    println!("  argument_number @72    = {}", i32_at(&buf, 72));
    println!("  argument_name   @76    = {:?}", inline_str(&buf, 76, 32));
    println!("  argument_index  @108   = {}", i32_at(&buf, 108));
    println!("  entity          @112   = {}", i32_at(&buf, 112));

    // Anything nonzero past the claimed 116-byte end would falsify the size.
    let tail_nonzero: Vec<usize> = (116..BUF).filter(|&i| buf[i] != 0).collect();
    println!(
        "  nonzero bytes >=116    = {}",
        if tail_nonzero.is_empty() {
            "none (consistent with size 116)".to_string()
        } else {
            format!(
                "{:?} (layout may be larger)",
                &tail_nonzero[..tail_nonzero.len().min(16)]
            )
        }
    );

    println!("  raw:");
    hexdump(&buf, 128);
}

fn main() {
    let _session = Session::start(SessionConfig::new()).expect("session");
    // Argument checking on: we want the kernel to report bad arguments.
    unsafe { PK_SESSION_set_check_arguments(PK_LOGICAL_true) };

    // 1. Negative dimension -> a value error on a specific argument.
    probe(
        "negative block dimension (expect distance_le_0 = 502)",
        || {
            let mut body: PK_BODY_t = 0;
            unsafe { PK_BODY_create_solid_block(-1.0, 1.0, 1.0, std::ptr::null(), &mut body) }
        },
    );

    // 2. Bogus tag -> an entity error; `entity` should carry the tag.
    probe("ask class of a bogus tag (expect not_an_entity)", || {
        let mut class: PK_CLASS_t = -1;
        unsafe { PK_ENTITY_ask_class(999_999, &mut class) }
    });

    // 3. Wrong entity class for the call.
    probe(
        "ask faces of a non-body tag (expect wrong entity type)",
        || {
            let mut n: i32 = 0;
            let mut faces: *mut PK_FACE_t = std::ptr::null_mut();
            unsafe { PK_BODY_ask_faces(999_999, &mut n, &mut faces) }
        },
    );

    // 4. Bad option-struct version -> exercises the o_t_version path and should
    //    name the offending *field* rather than a positional argument.
    probe(
        "mass props with o_t_version = 99 (expect 5022/5043)",
        || {
            let mut body: PK_BODY_t = 0;
            let rc =
                unsafe { PK_BODY_create_solid_block(1.0, 1.0, 1.0, std::ptr::null(), &mut body) };
            assert_eq!(rc, PK_ERROR_no_errors, "setup block must succeed");

            #[repr(C)]
            struct MassOpts {
                o_t_version: i32,
                mass: i32,
                periphery: i32,
                bound: i32,
                single: u8,
            }
            let opts = MassOpts {
                o_t_version: 99,
                mass: 0x36b4,
                periphery: 0x36b6,
                bound: 0x36b7,
                single: 1,
            };
            let (mut amount, mut mass, mut periphery) = (0.0f64, 0.0f64, 0.0f64);
            let mut c_of_g = [0.0f64; 3];
            let mut m_of_i = [0.0f64; 9];
            unsafe {
                PK_TOPOL_eval_mass_props(
                    1,
                    &body,
                    0.99,
                    &opts as *const MassOpts as *const PK_TOPOL_eval_mass_props_o_t,
                    &mut amount,
                    &mut mass,
                    &mut c_of_g,
                    &mut m_of_i,
                    &mut periphery,
                )
            }
        },
    );

    // 5. Does the record survive a *successful* call, or is it cleared?
    //    Determines whether `query_last_error` may be called unconditionally.
    println!("\n=== record persistence after a successful call");
    let mut body: PK_BODY_t = 0;
    let rc = unsafe { PK_BODY_create_solid_block(2.0, 2.0, 2.0, std::ptr::null(), &mut body) };
    println!("  successful call rc     = {rc}");
    let mut buf = [0u8; BUF];
    let mut was_error: PK_LOGICAL_t = PK_LOGICAL_false;
    unsafe { PK_ERROR_ask_last(&mut was_error, buf.as_mut_ptr() as *mut PK_ERROR_sf_t) };
    println!(
        "  was_error after success = {was_error}  (function = {:?}, code = {})",
        inline_str(&buf, 0, 32),
        i32_at(&buf, 32)
    );

    // 6. PK_THREAD_ask_last_error was recorded as "faults inside the kernel".
    //    That verdict predates the corrected 116-byte PK_ERROR_sf_t, so retest
    //    with a properly sized, zeroed buffer.
    println!("\n=== PK_THREAD_ask_last_error with a correctly sized buffer");
    let mut body: PK_BODY_t = 0;
    let rc = unsafe { PK_BODY_create_solid_block(-3.0, 1.0, 1.0, std::ptr::null(), &mut body) };
    println!("  trigger rc             = {rc}");
    let mut tbuf = [0u8; BUF];
    let mut twas: PK_LOGICAL_t = PK_LOGICAL_false;
    let trc =
        unsafe { PK_THREAD_ask_last_error(&mut twas, tbuf.as_mut_ptr() as *mut PK_ERROR_sf_t) };
    println!("  PK_THREAD_ask_last_error rc = {trc}, was_error = {twas}");
    if twas == PK_LOGICAL_true {
        println!("  function        @0     = {:?}", inline_str(&tbuf, 0, 32));
        println!("  code            @32    = {}", i32_at(&tbuf, 32));
        println!("  code_token      @36    = {:?}", inline_str(&tbuf, 36, 32));
        println!("  severity        @68    = {}", i32_at(&tbuf, 68));
        println!("  argument_name   @76    = {:?}", inline_str(&tbuf, 76, 32));
    }

    println!("\n=== done");
}
