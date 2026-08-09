//! Error types, severity classification, and the `pk_call!` macro.

use std::fmt;
use std::os::raw::c_char;

use parasolid_sys::*;

// =============================================================================
// Severity
// =============================================================================

/// Error severity as reported by Parasolid.
///
/// Determines the recovery strategy:
/// - **Mild**: operation failed, model untouched — retry with different inputs.
/// - **Serious**: model may be corrupted — must rollback to a valid pmark.
/// - **Fatal**: session corrupted — must stop and restart.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Mild,
    Serious,
    Fatal,
}

impl Severity {
    /// Map a `PK_ERROR_sf_t.severity` token to a `Severity`.
    ///
    /// Returns `None` for `PK_ERROR_none` (0) and for any unrecognised value,
    /// so the caller can fall back rather than silently reporting `Mild`.
    fn from_token(token: PK_ERROR_severity_t) -> Option<Self> {
        match token {
            PK_ERROR_mild => Some(Severity::Mild),
            PK_ERROR_serious => Some(Severity::Serious),
            PK_ERROR_fatal => Some(Severity::Fatal),
            _ => None,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Mild => f.write_str("mild"),
            Severity::Serious => f.write_str("serious"),
            Severity::Fatal => f.write_str("fatal"),
        }
    }
}

// =============================================================================
// BadArg
// =============================================================================

/// Information about an invalid argument reported by Parasolid.
///
/// The kernel reports **at most one** bad argument per error (confirmed against
/// `PK_ERROR_sf_t`, which carries a single `argument_number`/`argument_name`
/// pair — not the array of 20 an earlier binding modelled).
#[derive(Debug, Clone)]
pub struct BadArg {
    /// 1-based index of the invalid argument (0 when the error names no
    /// positional argument, e.g. a bad option-struct field).
    pub index: i32,
    /// Name of the argument, if available (e.g. `"x"`, `"entity"`, `"body"`).
    pub name: Option<String>,
    /// Index *within* the argument when it is an array; `-1` when not
    /// applicable.
    pub element: i32,
}

// =============================================================================
// ErrorDetails
// =============================================================================

/// Detailed information about a Parasolid error, extracted from `PK_ERROR_sf_t`.
#[derive(Debug, Clone)]
pub struct ErrorDetails {
    /// PK error code (e.g. `PK_ERROR_not_an_entity`). Numeric values live in
    /// `parasolid_sys::error_codes` and are probed, not documented.
    pub code: i32,
    /// Severity level.
    pub severity: Severity,
    /// Name of the PK function that raised the error.
    pub function: String,
    /// The kernel's own name for `code` (e.g. `"PK_ERROR_distance_le_0"`), read
    /// from `PK_ERROR_sf_t.code_token`. Authoritative — this is how the numeric
    /// table in `parasolid_sys::error_codes` was recovered.
    pub code_token: Option<String>,
    /// Invalid arguments, if any. The kernel reports at most one.
    pub bad_args: Vec<BadArg>,
    /// Entity tag involved in the error, if any (0 = none).
    pub entity: Option<i32>,
}

impl ErrorDetails {
    /// Build minimal details when PK_ERROR_ask_last is not available.
    fn simple(code: i32, severity: Severity) -> Self {
        ErrorDetails {
            code,
            severity,
            function: String::new(),
            code_token: None,
            bad_args: Vec::new(),
            entity: None,
        }
    }
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.function.is_empty() {
            write!(f, "PK error {} ({})", self.code, self.severity)?;
        } else {
            write!(
                f,
                "PK error {} ({}) in {}",
                self.code, self.severity, self.function
            )?;
        }
        if !self.bad_args.is_empty() {
            write!(f, " [bad args:")?;
            for a in &self.bad_args {
                match &a.name {
                    Some(n) => write!(f, " #{}={}", a.index, n)?,
                    None => write!(f, " #{}", a.index)?,
                }
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

// =============================================================================
// PsError
// =============================================================================

/// The primary error type for all Parasolid wrapper operations.
#[derive(Debug, Clone)]
pub enum PsError {
    /// Mild error — operation failed but the model is unaffected. Can retry.
    Mild(ErrorDetails),

    /// Serious error — model may be corrupted. **Must rollback** to a valid
    /// partition mark before continuing.
    Serious(ErrorDetails),

    /// Fatal error — session is corrupted beyond repair. **Must stop and
    /// restart** the session.
    Fatal(ErrorDetails),

    /// The entity tag is no longer valid (`PK_ERROR_not_an_entity`, code 22
    /// — probed; the previous binding claimed 504, so this arm never fired).
    NotAnEntity {
        /// The invalid tag value.
        tag: i32,
    },

    /// The operation was aborted by a user interrupt signal.
    Aborted,

    /// Session lifecycle error (not started, already active, etc.).
    Session(String),
}

/// Convenience alias for `Result<T, PsError>`.
pub type PsResult<T> = Result<T, PsError>;

impl PsError {
    /// Construct a `PsError` from a non-zero PK error code.
    ///
    /// Queries `PK_THREAD_ask_last_error` and `PK_ERROR_ask_last` to populate
    /// error details. Falls back to the code alone if neither is available.
    pub(crate) fn from_code(code: PK_ERROR_code_t) -> Self {
        // Special codes that don't need detailed error info
        if code == PK_ERROR_aborted {
            return PsError::Aborted;
        }

        // Try to get detailed error info from PK
        let details = query_last_error(code)
            .unwrap_or_else(|| ErrorDetails::simple(code, default_severity(code)));

        // Map to variant
        if code == PK_ERROR_not_an_entity {
            return PsError::NotAnEntity {
                tag: details.entity.unwrap_or(0),
            };
        }

        match details.severity {
            Severity::Mild => PsError::Mild(details),
            Severity::Serious => PsError::Serious(details),
            Severity::Fatal => PsError::Fatal(details),
        }
    }

    /// Returns the severity of this error, if applicable.
    pub fn severity(&self) -> Option<Severity> {
        match self {
            PsError::Mild(d) => Some(d.severity),
            PsError::Serious(d) => Some(d.severity),
            PsError::Fatal(d) => Some(d.severity),
            _ => None,
        }
    }

    /// Returns the error details, if available.
    pub fn details(&self) -> Option<&ErrorDetails> {
        match self {
            PsError::Mild(d) | PsError::Serious(d) | PsError::Fatal(d) => Some(d),
            _ => None,
        }
    }

    /// Returns `true` if this error requires a rollback to recover.
    pub fn requires_rollback(&self) -> bool {
        matches!(self, PsError::Serious(_))
    }

    /// Returns `true` if this error requires a full session restart.
    pub fn requires_restart(&self) -> bool {
        matches!(self, PsError::Fatal(_))
    }
}

impl fmt::Display for PsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PsError::Mild(d) => write!(f, "Parasolid mild error: {d}"),
            PsError::Serious(d) => write!(f, "Parasolid serious error (rollback required): {d}"),
            PsError::Fatal(d) => write!(f, "Parasolid fatal error (restart required): {d}"),
            PsError::NotAnEntity { tag } => write!(f, "invalid entity tag {tag}"),
            PsError::Aborted => f.write_str("operation aborted by user interrupt"),
            PsError::Session(msg) => write!(f, "session error: {msg}"),
        }
    }
}

impl std::error::Error for PsError {}

// =============================================================================
// pk_call! macro and pk_check function
// =============================================================================

/// Check a PK error code and convert to `PsResult<()>`.
///
/// Returns `Ok(())` for `PK_ERROR_no_errors` and `PK_ERROR_cant_be_aborted`
/// (which indicates the function completed normally despite an abort attempt).
/// Returns `Err(PsError)` for all other non-zero codes.
#[inline]
pub fn pk_check(code: PK_ERROR_code_t) -> PsResult<()> {
    if code == PK_ERROR_no_errors || code == PK_ERROR_cant_be_aborted {
        Ok(())
    } else {
        Err(PsError::from_code(code))
    }
}

/// Call a PK FFI function and propagate errors via `?`.
///
/// Wraps the call in `unsafe`, checks the return code, and returns
/// `Err(PsError)` on failure.
///
/// # Example
///
/// ```ignore
/// pk_call!(PK_SESSION_start(&opts));
/// ```
macro_rules! pk_call {
    ($call:expr) => {
        $crate::error::pk_check(unsafe { $call })?
    };
}

// =============================================================================
// Internal helpers
// =============================================================================

/// Decode an inline, NUL-padded `c_char` array into a `String`.
fn inline_str(field: &[c_char]) -> String {
    let bytes: Vec<u8> = field.iter().map(|&c| c as u8).collect();
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

/// Try to get error details from the session-level error state.
///
/// # Layout provenance
///
/// Reads the typed `PK_ERROR_sf_t`, whose 116-byte layout is **runtime-confirmed**
/// against pskernel.dll V37.01.243 under Wine by
/// `crates/parasolid-test/src/bin/error_probe.rs`: raising four distinct kinds
/// of error (bad dimension, bogus tag, wrong entity class, bad option version)
/// produced a coherent reading of every field — `function`@0, `code`@32,
/// `code_token`@36, `severity`@68, `argument_number`@72, `argument_name`@76,
/// `argument_index`@108, `entity`@112 — with no nonzero byte at or past offset
/// 116. The string fields are **inline char arrays**, not pointers; an earlier
/// binding modelled them as `*const c_char` and page-faulted on every error.
///
/// `PK_THREAD_ask_last_error` remains unused: it faults inside the kernel and
/// needs a separate threading audit.
///
/// # Staleness
///
/// The kernel does **not** clear the record on a successful call — `was_error`
/// stays true and still describes the previous failure (observed directly). So
/// this is only meaningful immediately after a call that returned non-zero,
/// which is the only place it is called from. As a guard, a record whose `code`
/// disagrees with the code we are reporting is treated as stale and dropped.
fn query_last_error(expected_code: PK_ERROR_code_t) -> Option<ErrorDetails> {
    let mut sf: PK_ERROR_sf_t = unsafe { std::mem::zeroed() };
    let mut was_error: PK_LOGICAL_t = PK_LOGICAL_false;
    let rc = unsafe { PK_ERROR_ask_last(&mut was_error, &mut sf) };
    if rc != PK_ERROR_no_errors || was_error != PK_LOGICAL_true {
        return None;
    }
    // Stale record from an earlier failure: do not attribute it to this call.
    if sf.code != expected_code {
        return None;
    }

    let bad_args = if sf.argument_number > 0 {
        let name = inline_str(&sf.argument_name);
        vec![BadArg {
            index: sf.argument_number,
            name: (!name.is_empty()).then_some(name),
            element: sf.argument_index,
        }]
    } else {
        Vec::new()
    };

    Some(ErrorDetails {
        code: sf.code,
        severity: Severity::from_token(sf.severity).unwrap_or_else(|| default_severity(sf.code)),
        function: inline_str(&sf.function),
        code_token: {
            let t = inline_str(&sf.code_token);
            (!t.is_empty()).then_some(t)
        },
        bad_args,
        entity: (sf.entity != 0).then_some(sf.entity),
    })
}

/// Guess severity from the error code alone (when PK_ERROR_sf_t is unavailable).
///
/// ASSUMPTION: Both `PK_THREAD_ask_last_error` and `PK_ERROR_ask_last` failed
/// to return error details. Severity is guessed from the code alone as a
/// best-effort fallback. Per PK §118.2.2, certain codes can be mild or serious
/// depending on context, so this classification may be wrong. Callers should
/// treat the result as advisory and prefer conservative recovery (rollback).
fn default_severity(code: PK_ERROR_code_t) -> Severity {
    match code {
        PK_ERROR_fatal_error | PK_ERROR_unhandleable_condition => Severity::Fatal,
        PK_ERROR_system_error | PK_ERROR_run_time_error | PK_ERROR_fru_error => Severity::Serious,
        _ => Severity::Mild,
    }
}
