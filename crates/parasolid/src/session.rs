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
use crate::partition::{Partition, Pmark, RollbackResult};

// =============================================================================
// Singleton guard
// =============================================================================

static SESSION_ACTIVE: AtomicBool = AtomicBool::new(false);

// =============================================================================
// Behaviour
// =============================================================================

/// Controls which version of Parasolid behaviour to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Behaviour {
    /// The initial default: behaviour has not been explicitly set, so the kernel
    /// uses the original system switches (`PK_SESSION_behave_as_unset_c`). This
    /// is what a freshly started session reports until [`SessionConfig::behaviour`]
    /// is applied.
    Unset,
    /// Use the latest behaviour for the current kernel version.
    Latest,
    /// Use behaviour from a specific patch release.
    ///
    /// Format: `MMmmpp` where `MM` = major, `mm` = minor, `pp` = patch.
    /// For example, `280103` means version 28.01.03.
    Version(i32),
}

// =============================================================================
// SmpInfo
// =============================================================================

/// SMP configuration returned by [`Session::smp`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmpInfo {
    /// Thread format identifier returned by Parasolid.
    pub thread_format: i32,
    /// Number of SMP worker threads currently configured.
    pub n_threads: i32,
    /// Number of processors reported by the kernel.
    pub n_processors: i32,
}

// =============================================================================
// Mark
// =============================================================================

/// A session mark — a rollback checkpoint spanning all partitions.
///
/// Session marks are created with [`Session::create_mark`] and roll the entire
/// session (all partitions) back to the state when the mark was set.
///
/// # Design note
///
/// `Mark` carries no session lifetime (`'s`), for the same reasons as
/// [`Entity`](crate::Entity) and [`Partition`](crate::Partition): lifetime
/// threading through returned values was deemed too ergonomically burdensome
/// for v0.1. Using a mark tag after the session has stopped is a PK-level
/// error caught by argument checking in dev builds. Revisit before v1.0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mark {
    tag: PK_MARK_t,
}

impl Mark {
    /// Wrap a raw PK mark tag.
    pub(crate) fn from_tag(tag: PK_MARK_t) -> Self {
        Mark { tag }
    }

    /// Returns the raw PK tag for this mark.
    #[inline]
    pub fn tag(&self) -> i32 {
        self.tag
    }

    /// Roll the session back to this mark.
    pub fn goto(&self) -> PsResult<()> {
        pk_call!(PK_MARK_goto(self.tag));
        Ok(())
    }

    /// Roll the session back to this mark, tracking which entities changed.
    pub fn goto_with_tracking(&self) -> PsResult<RollbackResult> {
        let opts = PK_MARK_goto_2_o_t {
            want_new_entities: PK_LOGICAL_true,
            want_mod_entities: PK_LOGICAL_true,
            want_del_entities: PK_LOGICAL_true,
            want_logged_mod: PK_LOGICAL_false,
            want_attrib_mod: PK_LOGICAL_false,
            del_attrib_cb: None,
            del_context: std::ptr::null_mut(),
            n_del_attdefs: 0,
            del_attdefs: std::ptr::null(),
            n_new_entities_classes: 0,
            new_entities_classes: std::ptr::null(),
            n_mod_entities_classes: 0,
            mod_entities_classes: std::ptr::null(),
            n_del_entities_classes: 0,
            del_entities_classes: std::ptr::null(),
            no_roll_diff: PK_LOGICAL_false,
        };
        let mut result: PK_MARK_goto_2_r_t = unsafe { std::mem::zeroed() };
        pk_call!(PK_MARK_goto_2(self.tag, &opts, &mut result));
        let new_entities = unsafe { PkArray::from_raw(result.new_entities, result.n_new_entities) }
            .iter()
            .map(|&tag| crate::Entity::from_tag(tag))
            .collect();
        let modified_entities =
            unsafe { PkArray::from_raw(result.mod_entities, result.n_mod_entities) }
                .iter()
                .map(|&tag| crate::Entity::from_tag(tag))
                .collect();
        let deleted_entities =
            unsafe { PkArray::from_raw(result.del_entities, result.n_del_entities) }
                .iter()
                .map(|&tag| crate::Entity::from_tag(tag))
                .collect();
        Ok(RollbackResult {
            new_entities,
            modified_entities,
            deleted_entities,
        })
    }

    /// Delete this session mark.
    pub fn delete(self) -> PsResult<()> {
        pk_call!(PK_MARK_delete(self.tag));
        Ok(())
    }

    /// Return the mark preceding this one in the session mark chain.
    pub fn preceding(&self) -> PsResult<Mark> {
        let mut tag: PK_MARK_t = 0;
        pk_call!(PK_MARK_ask_preceding(self.tag, &mut tag));
        Ok(Mark::from_tag(tag))
    }

    /// Return the mark following this one in the session mark chain.
    pub fn following(&self) -> PsResult<Mark> {
        let mut tag: PK_MARK_t = 0;
        pk_call!(PK_MARK_ask_following(self.tag, &mut tag));
        Ok(Mark::from_tag(tag))
    }

    /// Return the pmarks that would be current if this mark were rolled to.
    pub fn pmarks(&self) -> PsResult<Vec<Pmark>> {
        let mut n = 0;
        let mut ptr = std::ptr::null_mut();
        pk_call!(PK_MARK_ask_pmarks(self.tag, &mut n, &mut ptr));
        let array = unsafe { PkArray::from_raw(ptr, n) };
        Ok(array.iter().map(|&tag| Pmark::from_tag(tag)).collect())
    }
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
    /// Enable partitioned rollback (registers an in-memory delta frustrum via
    /// `PK_DELTA_register_callbacks` before `PK_SESSION_start`).
    pub(crate) rollback: bool,

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
            rollback: false,
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

    /// Enable **partitioned rollback** (pmarks). Registers an in-memory delta
    /// frustrum via `PK_DELTA_register_callbacks` before the session starts, so
    /// `Partition::create`, `make_pmark`, and `Pmark::goto` work. Deltas are held
    /// in process memory for the lifetime of the session.
    pub fn rollback(mut self, enable: bool) -> Self {
        self.rollback = enable;
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

        // Register the partitioned-rollback (delta) frustrum. Must happen after
        // the main frustrum and BEFORE PK_SESSION_start (the kernel switches on
        // partitioned rollback at registration time).
        if config.rollback {
            crate::rollback::reset_store();
            let delta_cbs = crate::rollback::delta_callbacks();
            let code = unsafe { PK_DELTA_register_callbacks(&delta_cbs) };
            if code != PK_ERROR_no_errors {
                return Err(PsError::from_code(code));
                // guard dropped → SESSION_ACTIVE = false
            }
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
                Behaviour::Unset => PK_SESSION_behaviour_t {
                    behaviour_type: PK_SESSION_behave_as_unset_c,
                    behaviour_value: 0,
                },
                Behaviour::Latest => PK_SESSION_behaviour_t {
                    behaviour_type: PK_SESSION_behave_as_latest_c,
                    behaviour_value: 0,
                },
                Behaviour::Version(v) => PK_SESSION_behaviour_t {
                    behaviour_type: PK_SESSION_behave_as_value_c,
                    behaviour_value: v,
                },
            };
            // Parasolid writes all three returned arguments unconditionally, so
            // pass real buffers rather than NULL (a NULL out-param faults).
            let mut behaviour_set = PK_SESSION_behaviour_t::default();
            let mut behaviour_previous = PK_SESSION_behaviour_t::default();
            let mut status: PK_behaviour_status_t = 0;
            pk_call!(PK_SESSION_set_behaviour(
                beh,
                std::ptr::null(),
                &mut behaviour_set,
                &mut behaviour_previous,
                &mut status,
            ));
        }
        if let Some(p) = config.precision {
            pk_call!(PK_SESSION_set_precision(p));
        }
        if let Some(p) = config.angle_precision {
            pk_call!(PK_SESSION_set_angle_precision(p));
        }
        if let Some(v) = config.err_reports {
            let reports = if v {
                PK_ERROR_reports_on_c
            } else {
                PK_ERROR_reports_off_c
            };
            pk_call!(PK_SESSION_set_err_reports(reports, std::ptr::null()));
        }
        if let Some(n) = config.smp_threads {
            // An explicit thread count means "absolute" thread format.
            let smp_opts = PK_SESSION_smp_o_t {
                thread_format: PK_thread_absolute_c,
                n_threads: n,
                ..Default::default()
            };
            pk_call!(PK_SESSION_set_smp(&smp_opts));
        }
        Ok(())
    }

    // =========================================================================
    // Query methods
    // =========================================================================

    /// Returns the Parasolid kernel version as `(major, minor, build)`.
    pub fn kernel_version(&self) -> PsResult<(i32, i32, i32)> {
        let mut info = PK_SESSION_kernel_version_t::default();
        pk_call!(PK_SESSION_ask_kernel_version(&mut info));
        Ok((info.major, info.minor, info.build))
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
        pk_call!(PK_SESSION_ask_behaviour(std::ptr::null(), &mut beh));
        match beh.behaviour_type {
            PK_SESSION_behave_as_unset_c => Ok(Behaviour::Unset),
            PK_SESSION_behave_as_latest_c => Ok(Behaviour::Latest),
            PK_SESSION_behave_as_value_c => Ok(Behaviour::Version(beh.behaviour_value)),
            other => Err(PsError::Session(format!(
                "unknown behaviour type {other}"
            ))),
        }
    }

    /// Returns whether self-intersection checking is enabled.
    pub fn check_self_int(&self) -> PsResult<bool> {
        let mut check: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_SESSION_ask_check_self_int(&mut check));
        Ok(check == PK_LOGICAL_true)
    }

    /// Returns the continuity checking level (0 = none, 1 = G1, 2 = G1+G2).
    pub fn check_continuity(&self) -> PsResult<i32> {
        let mut level = 0;
        pk_call!(PK_SESSION_ask_check_continuity(&mut level));
        Ok(level)
    }

    /// Returns whether roll-forward capability is enabled.
    pub fn roll_forward(&self) -> PsResult<bool> {
        let mut is_on: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_SESSION_is_roll_forward_on(&mut is_on));
        Ok(is_on == PK_LOGICAL_true)
    }

    /// Returns whether journaling is currently enabled.
    pub fn journalling(&self) -> PsResult<bool> {
        let mut enabled: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_SESSION_ask_journalling(&mut enabled));
        Ok(enabled == PK_LOGICAL_true)
    }

    /// Returns the user-field byte length attached to every entity.
    pub fn user_field_len(&self) -> PsResult<i32> {
        let mut len = 0;
        pk_call!(PK_SESSION_ask_user_field_len(&mut len));
        Ok(len)
    }

    /// Returns the latest supported session behaviour version.
    pub fn latest_behaviour(&self) -> PsResult<Behaviour> {
        let mut beh = PK_SESSION_behaviour_t::default();
        pk_call!(PK_SESSION_ask_latest_behaviour(&mut beh));
        match beh.behaviour_type {
            PK_SESSION_behave_as_unset_c => Ok(Behaviour::Unset),
            PK_SESSION_behave_as_latest_c => Ok(Behaviour::Latest),
            PK_SESSION_behave_as_value_c => Ok(Behaviour::Version(beh.behaviour_value)),
            other => Err(PsError::Session(format!(
                "unknown behaviour type {other}"
            ))),
        }
    }

    /// Returns the SMP configuration.
    pub fn smp(&self) -> PsResult<SmpInfo> {
        let mut r = PK_SESSION_smp_r_t::default();
        pk_call!(PK_SESSION_ask_smp(&mut r));
        Ok(SmpInfo {
            thread_format: r.thread_format,
            n_threads: r.n_threads,
            n_processors: r.n_processors,
        })
    }

    // =========================================================================
    // Mark access
    // =========================================================================

    /// Create a session mark — checkpoints all partitions.
    pub fn create_mark(&self) -> PsResult<Mark> {
        let mut tag: PK_MARK_t = 0;
        pk_call!(PK_MARK_create(&mut tag));
        Ok(Mark::from_tag(tag))
    }

    /// Return the current session mark and whether the modeller is at it.
    /// The mark tag is 0 if no mark has been set yet.
    pub fn current_mark(&self) -> PsResult<(Mark, bool)> {
        let mut tag: PK_MARK_t = 0;
        let mut at_mark: PK_LOGICAL_t = PK_LOGICAL_false;
        pk_call!(PK_SESSION_ask_mark(&mut tag, &mut at_mark));
        Ok((Mark::from_tag(tag), at_mark == PK_LOGICAL_true))
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
