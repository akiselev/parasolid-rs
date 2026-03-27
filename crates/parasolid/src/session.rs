//! Parasolid session lifecycle and configuration.
//!
//! A Parasolid session is a singleton — only one can be active at a time.
//! The [`Session`] struct enforces this at runtime and provides RAII cleanup.

use std::ffi::CString;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};

use parasolid_sys::*;

use crate::error::{PsError, PsResult};
use crate::frustrum::{self, FrustrumConfig};
use crate::memory::PkArray;
use crate::partition::{Partition, RollbackResult};

// =============================================================================
// Singleton guard
// =============================================================================

static SESSION_ACTIVE: AtomicBool = AtomicBool::new(false);

// =============================================================================
// Behaviour
// =============================================================================

/// Controls which version of Parasolid behaviour to use.
#[derive(Debug, Clone, Copy)]
pub enum Behaviour {
    /// Use the latest behaviour for the current kernel version.
    Latest,
    /// Use behaviour from a specific patch release.
    ///
    /// Format: `MMmmpp` where `MM` = major, `mm` = minor, `pp` = patch.
    /// For example, `280103` means version 28.01.03.
    Version(i32),
}

// =============================================================================
// SessionConfig
// =============================================================================

/// Builder for configuring a Parasolid session before starting it.
///
/// All settings are optional. Unset parameters retain Parasolid's defaults.
///
/// # Example
///
/// ```no_run
/// use parasolid::{Session, SessionConfig, Behaviour, FrustrumConfig};
///
/// let config = SessionConfig::new()
///     .frustrum(FrustrumConfig::new().base_dir("/models"))
///     .check_arguments(true)
///     .precision(1e-6)
///     .behaviour(Behaviour::Latest);
///
/// let session = Session::start(config)?;
/// # Ok::<(), parasolid::PsError>(())
/// ```
#[derive(Debug, Clone)]
pub struct SessionConfig {
    // Frustrum
    pub(crate) frustrum_config: FrustrumConfig,

    // Start options
    pub(crate) journal_file: Option<String>,
    pub(crate) user_field_len: Option<i32>,

    // Session parameters (applied after start)
    pub(crate) check_continuity: Option<i32>,
    pub(crate) check_self_int: Option<bool>,
    pub(crate) general_topology: Option<bool>,
    pub(crate) roll_forward: Option<bool>,
    pub(crate) check_arguments: Option<bool>,
    pub(crate) behaviour: Option<Behaviour>,
    pub(crate) precision: Option<f64>,
    pub(crate) angle_precision: Option<f64>,
    pub(crate) err_reports: Option<bool>,
    pub(crate) smp_threads: Option<i32>,
}

impl SessionConfig {
    /// Create a new config with all parameters unset (PK defaults).
    pub fn new() -> Self {
        SessionConfig {
            frustrum_config: FrustrumConfig::new(),
            journal_file: None,
            user_field_len: None,
            check_continuity: None,
            check_self_int: None,
            general_topology: None,
            roll_forward: None,
            check_arguments: None,
            behaviour: None,
            precision: None,
            angle_precision: None,
            err_reports: None,
            smp_threads: None,
        }
    }

    /// Set the frustrum configuration (memory and file I/O callbacks).
    pub fn frustrum(mut self, config: FrustrumConfig) -> Self {
        self.frustrum_config = config;
        self
    }

    /// Enable journaling to the specified file path.
    pub fn journal_file(mut self, path: impl Into<String>) -> Self {
        self.journal_file = Some(path.into());
        self
    }

    /// Set the user-field byte length attached to every entity (0 = none).
    pub fn user_field_len(mut self, len: i32) -> Self {
        self.user_field_len = Some(len);
        self
    }

    /// Set the continuity checking level.
    ///
    /// - `0`: no checking (fastest)
    /// - `1`: check G1 continuity
    /// - `2`: check G1 and G2 continuity
    pub fn check_continuity(mut self, level: i32) -> Self {
        self.check_continuity = Some(level);
        self
    }

    /// Control whether self-intersecting geometry can be attached to topology.
    pub fn check_self_int(mut self, check: bool) -> Self {
        self.check_self_int = Some(check);
        self
    }

    /// Allow general (non-manifold, disconnected) bodies from boolean operations.
    pub fn general_topology(mut self, allow: bool) -> Self {
        self.general_topology = Some(allow);
        self
    }

    /// Enable roll-forward capability for the session.
    pub fn roll_forward(mut self, enable: bool) -> Self {
        self.roll_forward = Some(enable);
        self
    }

    /// Enable argument checking on PK function calls.
    ///
    /// When enabled, PK validates all arguments before executing. Slower but
    /// produces better error messages. Recommended during development.
    pub fn check_arguments(mut self, check: bool) -> Self {
        self.check_arguments = Some(check);
        self
    }

    /// Set the session behaviour version.
    pub fn behaviour(mut self, b: Behaviour) -> Self {
        self.behaviour = Some(b);
        self
    }

    /// Set the session linear precision (default ~1e-8).
    ///
    /// Distances smaller than this are treated as zero.
    pub fn precision(mut self, p: f64) -> Self {
        self.precision = Some(p);
        self
    }

    /// Set the session angular precision (default ~1e-11 radians).
    pub fn angle_precision(mut self, p: f64) -> Self {
        self.angle_precision = Some(p);
        self
    }

    /// Enable or disable error report generation.
    pub fn err_reports(mut self, enable: bool) -> Self {
        self.err_reports = Some(enable);
        self
    }

    /// Set the number of SMP worker threads. 0 = disable SMP.
    pub fn smp_threads(mut self, n: i32) -> Self {
        self.smp_threads = Some(n);
        self
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Session
// =============================================================================

/// An active Parasolid modeling session.
///
/// Only one session can exist at a time (enforced at runtime). The session is
/// automatically stopped when this value is dropped.
///
/// `Session` is `!Send` and `!Sync` — it must remain on the thread that
/// created it. Use `PK_THREAD_*` APIs through `parasolid-sys` directly if you
/// need multi-threaded access.
pub struct Session {
    /// Prevent Send + Sync.
    _not_send: PhantomData<*const ()>,
}

impl Session {
    /// Start a new Parasolid session with the given configuration.
    ///
    /// # Errors
    ///
    /// - `PsError::Session` if a session is already active.
    /// - Any PK error from frustrum registration or session start.
    ///
    /// # Example
    ///
    /// ```no_run
    /// let session = parasolid::Session::start(parasolid::SessionConfig::new())?;
    /// # Ok::<(), parasolid::PsError>(())
    /// ```
    pub fn start(config: SessionConfig) -> PsResult<Session> {
        // Validate config before acquiring the singleton lock, so that
        // validation failures (e.g., null bytes in paths) don't permanently
        // lock SESSION_ACTIVE.
        let journal_cstr = config
            .journal_file
            .as_ref()
            .map(|s| CString::new(s.as_str()))
            .transpose()
            .map_err(|_| PsError::Session("journal path contains null byte".into()))?;

        // Enforce singleton
        if SESSION_ACTIVE.swap(true, Ordering::SeqCst) {
            return Err(PsError::Session(
                "a Parasolid session is already active".into(),
            ));
        }

        // Guard: if anything below panics before Session is constructed,
        // reset SESSION_ACTIVE so the process can retry.
        struct SessionGuard;
        impl Drop for SessionGuard {
            fn drop(&mut self) {
                SESSION_ACTIVE.store(false, Ordering::SeqCst);
            }
        }
        let guard = SessionGuard;

        // Register frustrum callbacks
        let fru = frustrum::build_frustrum(&config.frustrum_config);
        let code = unsafe { PK_SESSION_register_frustrum(&fru) };
        if code != PK_ERROR_no_errors {
            return Err(PsError::from_code(code));
            // guard dropped → SESSION_ACTIVE = false
        }

        // Build start options
        let start_opts = PK_SESSION_start_o_t {
            o_t_version: 1,
            journal_file: journal_cstr
                .as_ref()
                .map_or(std::ptr::null(), |c| c.as_ptr()),
            user_field_len: config.user_field_len.unwrap_or(0),
        };

        // Start the session
        let code = unsafe { PK_SESSION_start(&start_opts) };
        if code != PK_ERROR_no_errors {
            return Err(PsError::from_code(code));
            // guard dropped → SESSION_ACTIVE = false
        }

        // Session is now active — Session::Drop takes over responsibility
        // for resetting SESSION_ACTIVE, so defuse the guard.
        std::mem::forget(guard);

        let session = Session {
            _not_send: PhantomData,
        };

        // If apply_config fails, Session::Drop will call PK_SESSION_stop
        // and reset SESSION_ACTIVE.
        session.apply_config(&config)?;
        Ok(session)
    }

    /// Apply optional session parameters that were set in the config.
    fn apply_config(&self, config: &SessionConfig) -> PsResult<()> {
        if let Some(v) = config.check_arguments {
            pk_call!(PK_SESSION_set_check_arguments(to_logical(v)));
        }
        if let Some(level) = config.check_continuity {
            pk_call!(PK_SESSION_set_check_continuity(level));
        }
        if let Some(v) = config.check_self_int {
            pk_call!(PK_SESSION_set_check_self_int(to_logical(v)));
        }
        if let Some(v) = config.general_topology {
            pk_call!(PK_SESSION_set_general_topology(to_logical(v)));
        }
        if let Some(v) = config.roll_forward {
            pk_call!(PK_SESSION_set_roll_forward(to_logical(v)));
        }
        if let Some(b) = config.behaviour {
            let beh = match b {
                Behaviour::Latest => PK_SESSION_behaviour_t {
                    behaviour_type: PK_SESSION_behave_as_latest_c,
                    behaviour_value: 0,
                },
                Behaviour::Version(v) => PK_SESSION_behaviour_t {
                    behaviour_type: PK_SESSION_behave_as_value_c,
                    behaviour_value: v,
                },
            };
            pk_call!(PK_SESSION_set_behaviour(&beh));
        }
        if let Some(p) = config.precision {
            pk_call!(PK_SESSION_set_precision(p));
        }
        if let Some(p) = config.angle_precision {
            pk_call!(PK_SESSION_set_angle_precision(p));
        }
        if let Some(v) = config.err_reports {
            pk_call!(PK_SESSION_set_err_reports(to_logical(v)));
        }
        if let Some(n) = config.smp_threads {
            pk_call!(PK_SESSION_set_smp(n));
        }
        Ok(())
    }

    // =========================================================================
    // Query methods
    // =========================================================================

    /// Returns the Parasolid kernel version as `(major, minor, patch)`.
    pub fn kernel_version(&self) -> PsResult<(i32, i32, i32)> {
        let mut major = 0;
        let mut minor = 0;
        let mut patch = 0;
        pk_call!(PK_SESSION_ask_kernel_version(
            &mut major, &mut minor, &mut patch
        ));
        Ok((major, minor, patch))
    }

    /// Returns the Parasolid schema version.
    pub fn schema_version(&self) -> PsResult<i32> {
        let mut version = 0;
        pk_call!(PK_SESSION_ask_schema_version(&mut version));
        Ok(version)
    }

    /// Returns the current linear precision.
    pub fn precision(&self) -> PsResult<f64> {
        let mut p = 0.0;
        pk_call!(PK_SESSION_ask_precision(&mut p));
        Ok(p)
    }

    /// Returns the current angular precision.
    pub fn angle_precision(&self) -> PsResult<f64> {
        let mut p = 0.0;
        pk_call!(PK_SESSION_ask_angle_precision(&mut p));
        Ok(p)
    }

    /// Returns whether argument checking is enabled.
    pub fn check_arguments(&self) -> PsResult<bool> {
        let mut check: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_SESSION_ask_check_arguments(&mut check));
        Ok(check == PK_LOGICAL_true)
    }

    /// Returns whether general topology is enabled.
    pub fn general_topology(&self) -> PsResult<bool> {
        let mut allow: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_SESSION_ask_general_topology(&mut allow));
        Ok(allow == PK_LOGICAL_true)
    }

    /// Returns the total memory used by model data structures, in bytes.
    pub fn memory_usage(&self) -> PsResult<i32> {
        let mut n_bytes = 0;
        pk_call!(PK_SESSION_ask_memory_usage(&mut n_bytes));
        Ok(n_bytes)
    }

    /// Returns the number of entity tags remaining before the limit.
    pub fn tags_remaining(&self) -> PsResult<i32> {
        let mut n = 0;
        pk_call!(PK_SESSION_ask_tags_remaining(&mut n));
        Ok(n)
    }

    /// Returns the current session behaviour.
    pub fn behaviour(&self) -> PsResult<Behaviour> {
        let mut beh = PK_SESSION_behaviour_t::default();
        pk_call!(PK_SESSION_ask_behaviour(&mut beh));
        match beh.behaviour_type {
            PK_SESSION_behave_as_latest_c => Ok(Behaviour::Latest),
            PK_SESSION_behave_as_value_c => Ok(Behaviour::Version(beh.behaviour_value)),
            other => Err(PsError::Session(format!(
                "unknown behaviour type {other}"
            ))),
        }
    }

    /// Returns the SMP configuration as `(n_threads, n_processors)`.
    pub fn smp(&self) -> PsResult<(i32, i32)> {
        let mut thread_format = 0;
        let mut n_threads = 0;
        let mut n_processors = 0;
        pk_call!(PK_SESSION_ask_smp(
            &mut thread_format,
            &mut n_threads,
            &mut n_processors
        ));
        Ok((n_threads, n_processors))
    }

    // =========================================================================
    // Partition access
    // =========================================================================

    /// Returns the current partition.
    pub fn current_partition(&self) -> PsResult<Partition> {
        let mut tag: PK_PARTITION_t = 0;
        pk_call!(PK_SESSION_ask_curr_partition(&mut tag));
        Ok(Partition::from_tag(tag))
    }

    /// Returns all partitions in the session.
    pub fn partitions(&self) -> PsResult<Vec<Partition>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_SESSION_ask_partitions(&mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Partition::from_tag(tag)).collect())
    }

    /// Returns all parts (bodies and assemblies) in the session.
    pub fn parts(&self) -> PsResult<Vec<crate::Entity>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_SESSION_ask_parts(&mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array
            .iter()
            .map(|&tag| crate::Entity::from_tag(tag))
            .collect())
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe {
            let _ = PK_SESSION_stop();
        }
        frustrum::reset();
        SESSION_ACTIVE.store(false, Ordering::SeqCst);
    }
}

// Session is explicitly !Send and !Sync via PhantomData<*const ()>.
// This is a safety measure — the default frustrum uses global state.

// =============================================================================
// Helpers
// =============================================================================

#[inline]
fn to_logical(b: bool) -> PK_LOGICAL_t {
    if b {
        PK_LOGICAL_true
    } else {
        PK_LOGICAL_false
    }
}
