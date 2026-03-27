//! Error types, severity classification, and the `pk_call!` macro.

use std::ffi::CStr;
use std::fmt;
use std::os::raw::c_int;

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
#[derive(Debug, Clone)]
pub struct BadArg {
    /// 1-based index of the invalid argument.
    pub index: i32,
    /// Name of the argument, if available.
    pub name: Option<String>,
}

// =============================================================================
// ErrorDetails
// =============================================================================

/// Detailed information about a Parasolid error, extracted from `PK_ERROR_sf_t`.
#[derive(Debug, Clone)]
pub struct ErrorDetails {
    /// PK error code (e.g. `PK_ERROR_general`, `PK_ERROR_not_an_entity`).
    pub code: i32,
    /// Severity level.
    pub severity: Severity,
    /// Name of the PK function that raised the error.
    pub function: String,
    /// Invalid arguments, if any.
    pub bad_args: Vec<BadArg>,
    /// Entity tag involved in the error, if any (0 = none).
    pub entity: Option<i32>,
}

impl ErrorDetails {
    /// Build details from the raw PK error structure.
    fn from_sf(sf: &PK_ERROR_sf_t) -> Self {
        let function = if sf.function.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(sf.function) }
                .to_string_lossy()
                .into_owned()
        };

        let mut bad_args = Vec::new();
        for i in 0..sf.n_bad_args.min(PK_ERROR_MAX_BAD_ARGS as c_int) {
            let name = if sf.bad_arg_names[i as usize].is_null() {
                None
            } else {
                Some(
                    unsafe { CStr::from_ptr(sf.bad_arg_names[i as usize]) }
                        .to_string_lossy()
                        .into_owned(),
                )
            };
            bad_args.push(BadArg {
                index: sf.bad_args[i as usize],
                name,
            });
        }

        let entity = if sf.entity == PK_ENTITY_null {
            None
        } else {
            Some(sf.entity)
        };

        ErrorDetails {
            code: sf.code,
            severity: severity_from_raw(sf.severity),
            function,
            bad_args,
            entity,
        }
    }

    /// Build minimal details when PK_ERROR_ask_last is not available.
    fn simple(code: i32, severity: Severity) -> Self {
        ErrorDetails {
            code,
            severity,
            function: String::new(),
            bad_args: Vec::new(),
            entity: None,
        }
    }
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.function.is_empty() {
            write!(f, "PK error {} ({})", self.code, self.severity)
        } else {
            write!(
                f,
                "PK error {} ({}) in {}",
                self.code, self.severity, self.function
            )
        }
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

    /// The entity tag is no longer valid (`PK_ERROR_not_an_entity`, code 504).
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
        let details = query_last_error().unwrap_or_else(|| {
            ErrorDetails::simple(code, default_severity(code))
        });

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

/// Try to get error details from the PK thread-local or session-level error state.
fn query_last_error() -> Option<ErrorDetails> {
    // Try thread-safe error query first
    let mut error_sf = PK_ERROR_sf_t::default();
    let code = unsafe { PK_THREAD_ask_last_error(&mut error_sf) };
    if code == PK_ERROR_no_errors && error_sf.code != PK_ERROR_no_errors {
        return Some(ErrorDetails::from_sf(&error_sf));
    }

    // Fall back to session-level error query
    let mut was_error: PK_LOGICAL_t = PK_LOGICAL_false;
    let mut error_sf = PK_ERROR_sf_t::default();
    let code = unsafe { PK_ERROR_ask_last(&mut was_error, &mut error_sf) };
    if code == PK_ERROR_no_errors && was_error == PK_LOGICAL_true {
        return Some(ErrorDetails::from_sf(&error_sf));
    }

    None
}

/// Map raw PK severity constant to `Severity`.
fn severity_from_raw(raw: PK_ERROR_severity_t) -> Severity {
    match raw {
        PK_ERROR_serious => Severity::Serious,
        PK_ERROR_fatal => Severity::Fatal,
        _ => Severity::Mild,
    }
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
