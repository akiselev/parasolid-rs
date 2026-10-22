//! Time and memory budgets for PK calls.
//!
//! A runaway kernel call (parasolid-re P1 blocker B23: `PK_SURF_intersect_surf`
//! on a near-tangent plane/cylinder never returns and grows memory at about
//! 70 MB/s) returns [`PsError::BudgetExceeded`] instead of hanging the process.
//!
//! # Mechanism (parasolid-re B23, dynamic-observed under Wine on 37.01.243)
//!
//! - **Time.** One parked watchdog thread. When an armed deadline passes it
//!   calls `PK_SESSION_abort(PK_abort_user_interrupt_c)` under a mutex; the
//!   wrapper disarms under the same mutex, so no abort is issued after the
//!   budgeted work has returned. The abort only sets a per-thread flag. The
//!   kernel services it at its next poll point, which sits in the DS allocator,
//!   and the call returns `PK_ERROR_aborted` (945), or rarely
//!   `PK_ERROR_system_error` (900), about 0.1 s later. If the main thread is
//!   not inside the kernel the abort returns `PK_ERROR_not_interrupted` (946)
//!   and sets nothing, so the watchdog retries every 5 ms until one lands.
//! - **Drain.** A landed abort whose flag was not consumed by a 945 stays
//!   pending through non-allocating calls and aborts the next unrelated
//!   allocating call. The guard therefore runs a sacrificial `PK_PLANE_create`
//!   (and deletes the plane) at most twice, until it stops returning 945,
//!   immediately after the budgeted work returns.
//! - **Memory.** The default `FMALLO` counts live bytes and, while a limit is
//!   armed, refuses with `FR_memory_full` (15), which the kernel treats as a
//!   quiet NULL: it unwinds through its reserve hunks and returns
//!   `PK_ERROR_memory_full` (901).
//!
//! # Limits
//!
//! - Only a call that allocates can be interrupted: a loop that never
//!   allocates never polls. Keep an outer process timeout or cgroup as the
//!   backstop.
//! - Single-threaded sessions only. Budgets apply to PK calls made on the
//!   thread that set them; SMP sub-threads were not tested.
//! - Budgets cover calls that go through this crate's wrappers (the
//!   `pk_call!` macro and the long-running raw call sites routed through
//!   [`guard`]). A raw `parasolid_sys` call is covered only inside
//!   [`Session::with_budget`](crate::Session::with_budget), where the
//!   watchdog and the memory limit span the whole closure.
//! - A budget error is a serious kernel error. In a session with partitioned
//!   rollback, `with_budget` rolls back to a pmark it took on entry; the
//!   per-call default budget takes no pmark. Without rollback the session was
//!   observed usable afterwards, with its memory back at the pre-call level.

use std::cell::{Cell, RefCell};
use std::fmt;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};

use parasolid_sys::*;

use crate::error::PsError;

// =============================================================================
// Public types
// =============================================================================

/// A time and memory budget for PK calls. `None` means unlimited.
///
/// See [`Session::with_budget`](crate::Session::with_budget) and
/// [`Session::set_default_budget`](crate::Session::set_default_budget).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Budget {
    /// Wall-clock time from entry to the abort request.
    pub time: Option<Duration>,
    /// Kernel memory (live `FMALLO` bytes) the work may add on top of what was
    /// live at entry.
    pub memory_bytes: Option<usize>,
}

impl Budget {
    /// A time budget only.
    pub fn time(time: Duration) -> Self {
        Budget {
            time: Some(time),
            memory_bytes: None,
        }
    }

    /// A memory budget only.
    pub fn memory(bytes: usize) -> Self {
        Budget {
            time: None,
            memory_bytes: Some(bytes),
        }
    }

    /// This budget with a time limit.
    pub fn with_time(mut self, time: Duration) -> Self {
        self.time = Some(time);
        self
    }

    /// This budget with a memory limit.
    pub fn with_memory(mut self, bytes: usize) -> Self {
        self.memory_bytes = Some(bytes);
        self
    }

    fn is_unlimited(&self) -> bool {
        self.time.is_none() && self.memory_bytes.is_none()
    }

    /// Fill unset fields from `fallback`.
    fn or(self, fallback: Option<Budget>) -> Self {
        let f = fallback.unwrap_or_default();
        Budget {
            time: self.time.or(f.time),
            memory_bytes: self.memory_bytes.or(f.memory_bytes),
        }
    }
}

/// Which limit of a [`Budget`] stopped the work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BudgetKind {
    /// The watchdog aborted the call (`PK_ERROR_aborted` or
    /// `PK_ERROR_system_error` after a landed abort).
    Time,
    /// `FMALLO` refused an allocation (`PK_ERROR_memory_full`).
    Memory,
}

impl fmt::Display for BudgetKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BudgetKind::Time => f.write_str("time"),
            BudgetKind::Memory => f.write_str("memory"),
        }
    }
}

/// Process-wide budget counters, for diagnostics and tests.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BudgetStats {
    /// Live bytes allocated by the default `FMALLO` and not yet freed.
    pub live_bytes: usize,
    /// `PK_SESSION_abort` calls that returned 0 (the kernel flag was set).
    pub aborts_landed: u64,
    /// `PK_SESSION_abort` calls that returned non-zero (not inside PK, or a
    /// function that cannot be aborted); the watchdog retried.
    pub aborts_missed: u64,
    /// `FMALLO` calls refused by a memory budget.
    pub refusals: u64,
    /// Drains run (a landed abort whose flag no call had consumed).
    pub drains: u64,
    /// Sacrificial `PK_PLANE_create` calls in drains that returned 945, i.e.
    /// stale flags actually removed.
    pub drained_flags: u64,
}

// =============================================================================
// Memory accounting (called from the default frustrum)
// =============================================================================

static LIVE: AtomicUsize = AtomicUsize::new(0);
/// Absolute live-byte ceiling while a memory budget is armed.
static MEM_LIMIT: AtomicUsize = AtomicUsize::new(usize::MAX);
static REFUSALS: AtomicU64 = AtomicU64::new(0);
static LIVE_AT_REFUSAL: AtomicUsize = AtomicUsize::new(0);
static ABORTS_LANDED: AtomicU64 = AtomicU64::new(0);
static ABORTS_MISSED: AtomicU64 = AtomicU64::new(0);
static DRAINS: AtomicU64 = AtomicU64::new(0);
static DRAINED_FLAGS: AtomicU64 = AtomicU64::new(0);

/// `FMALLO` admission: `false` refuses the request (the caller returns
/// `FR_memory_full`).
pub(crate) fn admit(nbytes: usize) -> bool {
    let live = LIVE.load(Ordering::SeqCst);
    if live.saturating_add(nbytes) > MEM_LIMIT.load(Ordering::SeqCst) {
        LIVE_AT_REFUSAL.store(live, Ordering::SeqCst);
        REFUSALS.fetch_add(1, Ordering::SeqCst);
        return false;
    }
    true
}

pub(crate) fn note_alloc(nbytes: usize) {
    LIVE.fetch_add(nbytes, Ordering::SeqCst);
}

pub(crate) fn note_free(nbytes: usize) {
    LIVE.fetch_sub(nbytes, Ordering::SeqCst);
}

pub(crate) fn stats() -> BudgetStats {
    BudgetStats {
        live_bytes: LIVE.load(Ordering::SeqCst),
        aborts_landed: ABORTS_LANDED.load(Ordering::SeqCst),
        aborts_missed: ABORTS_MISSED.load(Ordering::SeqCst),
        refusals: REFUSALS.load(Ordering::SeqCst),
        drains: DRAINS.load(Ordering::SeqCst),
        drained_flags: DRAINED_FLAGS.load(Ordering::SeqCst),
    }
}

// =============================================================================
// Watchdog
// =============================================================================

/// Retry interval after an abort that did not land (946/965).
const RETRY: Duration = Duration::from_millis(5);

#[derive(Default)]
struct Watch {
    generation: u64,
    /// Armed deadline; `None` when disarmed or after an abort landed.
    deadline: Option<Instant>,
    /// An abort returned 0 for this generation.
    landed: bool,
    /// Live bytes when the abort landed.
    live_at_landing: usize,
}

static WATCH: Mutex<Watch> = Mutex::new(Watch {
    generation: 0,
    deadline: None,
    landed: false,
    live_at_landing: 0,
});
static WAKE: Condvar = Condvar::new();
static WATCHDOG: OnceLock<()> = OnceLock::new();

fn watch() -> MutexGuard<'static, Watch> {
    // The lock is never held across a panic point, but recover anyway.
    WATCH.lock().unwrap_or_else(|e| e.into_inner())
}

fn watchdog_loop() {
    let mut w = watch();
    loop {
        let Some(deadline) = w.deadline else {
            w = WAKE.wait(w).unwrap_or_else(|e| e.into_inner());
            continue;
        };
        let now = Instant::now();
        if now < deadline {
            w = WAKE
                .wait_timeout(w, deadline - now)
                .unwrap_or_else(|e| e.into_inner())
                .0;
            continue;
        }
        // Deadline passed while armed. Abort under the lock, so the wrapper
        // cannot disarm (and return) between our check and the abort.
        let rc = unsafe { PK_SESSION_abort(PK_abort_user_interrupt_c) };
        if rc == PK_ERROR_no_errors {
            ABORTS_LANDED.fetch_add(1, Ordering::SeqCst);
            w.landed = true;
            w.live_at_landing = LIVE.load(Ordering::SeqCst);
            w.deadline = None;
        } else {
            // 946: no thread inside PK (between calls); 965: a function that
            // cannot be aborted. Try again shortly.
            ABORTS_MISSED.fetch_add(1, Ordering::SeqCst);
            w = WAKE
                .wait_timeout(w, RETRY)
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
    }
}

/// A budget armed on entry to budgeted work.
struct Armed {
    generation: u64,
    start: Instant,
    refusals_at_entry: u64,
}

/// What happened while armed.
#[derive(Clone, Copy, Default)]
struct Observed {
    landed: bool,
    live_at_landing: usize,
    refusals: u64,
    live_at_refusal: usize,
    elapsed: Duration,
}

fn arm(budget: Budget) -> Armed {
    let start = Instant::now();
    let refusals_at_entry = REFUSALS.load(Ordering::SeqCst);
    if let Some(bytes) = budget.memory_bytes {
        let limit = LIVE.load(Ordering::SeqCst).saturating_add(bytes);
        MEM_LIMIT.store(limit, Ordering::SeqCst);
    }
    let mut w = watch();
    w.generation += 1;
    w.landed = false;
    w.live_at_landing = 0;
    w.deadline = budget.time.map(|t| start + t);
    let generation = w.generation;
    if w.deadline.is_some() {
        WATCHDOG.get_or_init(|| {
            std::thread::Builder::new()
                .name("parasolid-budget-watchdog".into())
                .spawn(watchdog_loop)
                .expect("spawn the budget watchdog thread");
        });
        WAKE.notify_all();
    }
    Armed {
        generation,
        start,
        refusals_at_entry,
    }
}

fn observe(armed: &Armed, w: &Watch) -> Observed {
    debug_assert_eq!(w.generation, armed.generation);
    Observed {
        landed: w.landed,
        live_at_landing: w.live_at_landing,
        refusals: REFUSALS.load(Ordering::SeqCst) - armed.refusals_at_entry,
        live_at_refusal: LIVE_AT_REFUSAL.load(Ordering::SeqCst),
        elapsed: armed.start.elapsed(),
    }
}

/// Read the state without disarming (inside a scope).
fn peek(armed: &Armed) -> Observed {
    observe(armed, &watch())
}

/// Disarm both limits. After this returns no abort can be issued for `armed`.
fn disarm(armed: &Armed) -> Observed {
    MEM_LIMIT.store(usize::MAX, Ordering::SeqCst);
    let mut w = watch();
    w.deadline = None;
    observe(armed, &w)
}

/// Remove a possibly pending abort flag: a sacrificial allocating heavyweight
/// call, at most twice, until it stops returning 945.
fn drain() {
    DRAINS.fetch_add(1, Ordering::SeqCst);
    let sf = PK_PLANE_sf_t {
        basis_set: PK_AXIS2_sf_t {
            location: [0.0, 0.0, 0.0],
            axis: [0.0, 0.0, 1.0],
            ref_direction: [1.0, 0.0, 0.0],
        },
    };
    for _ in 0..2 {
        let mut plane: PK_PLANE_t = PK_ENTITY_null;
        let rc = unsafe { PK_PLANE_create(&sf, &mut plane) };
        if rc == PK_ERROR_no_errors && plane != PK_ENTITY_null {
            unsafe { PK_ENTITY_delete(1, &plane) };
        }
        if rc != PK_ERROR_aborted {
            break;
        }
        DRAINED_FLAGS.fetch_add(1, Ordering::SeqCst);
    }
}

/// Map a PK code returned under an armed budget (B23 requirement 3).
fn classify(code: PK_ERROR_code_t, seen: &Observed) -> Option<PsError> {
    if code == PK_ERROR_memory_full && seen.refusals > 0 {
        return Some(PsError::BudgetExceeded {
            kind: BudgetKind::Memory,
            pk_code: code,
            elapsed: seen.elapsed,
            live_bytes: seen.live_at_refusal,
        });
    }
    if seen.landed && (code == PK_ERROR_aborted || code == PK_ERROR_system_error) {
        return Some(PsError::BudgetExceeded {
            kind: BudgetKind::Time,
            pk_code: code,
            elapsed: seen.elapsed,
            live_bytes: seen.live_at_landing,
        });
    }
    None
}

/// The same mapping for an error a closure returned (codes the guard did not
/// see, e.g. from a raw `parasolid_sys` call converted with `PsError`).
fn classify_error(e: &PsError, seen: &Observed) -> Option<PsError> {
    let code = match e {
        PsError::Aborted => PK_ERROR_aborted,
        PsError::BudgetExceeded { .. } => return Some(e.clone()),
        other => other.details()?.code,
    };
    classify(code, seen)
}

// =============================================================================
// Session state (thread-local: budgets follow the session's thread)
// =============================================================================

struct Scope {
    armed: Armed,
    /// The first budget error in this scope; later guarded calls return it.
    exceeded: Option<PsError>,
    /// A 945 was returned after the abort landed (the flag is consumed).
    consumed: bool,
}

thread_local! {
    static DEFAULT: Cell<Option<Budget>> = const { Cell::new(None) };
    static ROLLBACK: Cell<bool> = const { Cell::new(false) };
    static SCOPE: RefCell<Option<Scope>> = const { RefCell::new(None) };
}

pub(crate) fn set_default(budget: Option<Budget>) {
    DEFAULT.with(|d| d.set(budget.filter(|b| !b.is_unlimited())));
}

pub(crate) fn default_budget() -> Option<Budget> {
    DEFAULT.with(Cell::get)
}

/// Called by `Session::start` (after the config is applied) and on drop.
pub(crate) fn session_started(rollback: bool, default: Option<Budget>) {
    ROLLBACK.with(|r| r.set(rollback));
    set_default(default);
}

pub(crate) fn session_stopped() {
    ROLLBACK.with(|r| r.set(false));
    set_default(None);
}

// =============================================================================
// Guard (every pk_call! and the long raw call sites)
// =============================================================================

/// Run one PK call under the active budget and return its code, or the budget
/// error. The caller still checks the code (`pk_check`).
///
/// - Inside a [`with_budget`] scope: the scope's budget is already armed; the
///   code is classified, and once the scope is exceeded later calls return the
///   same error without calling PK.
/// - Otherwise, with a default budget: the call is armed, run, disarmed,
///   classified, and drained if an abort landed that the call did not consume.
/// - Otherwise the call runs unchanged.
pub(crate) fn guard(call: impl FnOnce() -> PK_ERROR_code_t) -> Result<PK_ERROR_code_t, PsError> {
    let in_scope = SCOPE.with(|s| {
        let s = s.borrow();
        s.as_ref().map(|sc| sc.exceeded.clone())
    });
    match in_scope {
        Some(Some(err)) => Err(err),
        Some(None) => {
            let code = call();
            SCOPE.with(|s| {
                let mut s = s.borrow_mut();
                let sc = s.as_mut().expect("budget scope vanished during a call");
                let seen = peek(&sc.armed);
                if seen.landed && !sc.consumed {
                    // The deadline has passed. Remove a flag this call did not
                    // consume now, before the closure makes any other call.
                    if code != PK_ERROR_aborted {
                        drain();
                    }
                    sc.consumed = true;
                }
                let err = classify(code, &seen);
                if seen.landed || err.is_some() {
                    // The watchdog does not fire twice, so close the scope to
                    // further PK calls. An abort that landed after this call's
                    // last poll still ends the scope; pk_code records the code
                    // the call actually returned.
                    sc.exceeded = Some(err.clone().unwrap_or(PsError::BudgetExceeded {
                        kind: BudgetKind::Time,
                        pk_code: code,
                        elapsed: seen.elapsed,
                        live_bytes: seen.live_at_landing,
                    }));
                }
                match err {
                    Some(err) => Err(err),
                    None => Ok(code),
                }
            })
        }
        None => match default_budget() {
            None => Ok(call()),
            Some(budget) => {
                let armed = arm(budget);
                let code = call();
                let seen = disarm(&armed);
                if seen.landed && code != PK_ERROR_aborted {
                    drain();
                }
                match classify(code, &seen) {
                    Some(err) => Err(err),
                    None => Ok(code),
                }
            }
        },
    }
}

// =============================================================================
// with_budget
// =============================================================================

/// Closes a scope: disarm, drain, roll back. Runs on normal exit and on panic.
struct ScopeExit {
    pmark: Option<PK_PMARK_t>,
    done: bool,
}

impl ScopeExit {
    /// Returns what was observed and the scope's recorded budget error.
    fn finish(&mut self) -> (Observed, Option<PsError>) {
        self.done = true;
        let Some(scope) = SCOPE.with(|s| s.borrow_mut().take()) else {
            return (Observed::default(), None);
        };
        let seen = disarm(&scope.armed);
        if seen.landed && !scope.consumed {
            drain();
        }
        (seen, scope.exceeded)
    }

    fn roll_back(&self) {
        if let Some(pmark) = self.pmark {
            roll_back_to(pmark);
        }
    }
}

impl Drop for ScopeExit {
    fn drop(&mut self) {
        if !self.done {
            let _ = self.finish();
            self.roll_back();
        }
    }
}

fn roll_back_to(pmark: PK_PMARK_t) {
    // PK_PMARK_goto writes the three counts unconditionally (see Pmark::goto).
    let (mut n_new, mut n_mod, mut n_del) = (0, 0, 0);
    let mut new_p: *mut PK_ENTITY_t = std::ptr::null_mut();
    let mut mod_p: *mut PK_ENTITY_t = std::ptr::null_mut();
    let mut del_p: *mut i32 = std::ptr::null_mut();
    unsafe {
        let _ = PK_PMARK_goto(
            pmark, &mut n_new, &mut new_p, &mut n_mod, &mut mod_p, &mut n_del, &mut del_p,
        );
        let _ = crate::memory::PkArray::from_raw(new_p, n_new);
        let _ = crate::memory::PkArray::from_raw(mod_p, n_mod);
        let _ = crate::memory::PkArray::from_raw(del_p, n_del);
    }
}

pub(crate) fn with_budget<T>(
    budget: Budget,
    f: impl FnOnce() -> crate::PsResult<T>,
) -> crate::PsResult<T> {
    if SCOPE.with(|s| s.borrow().is_some()) {
        return Err(PsError::Session(
            "Session::with_budget called inside another with_budget".into(),
        ));
    }
    let budget = budget.or(default_budget());
    let pmark = if ROLLBACK.with(Cell::get) {
        let mut part: PK_PARTITION_t = PK_ENTITY_null;
        let mut pmark: PK_PMARK_t = PK_ENTITY_null;
        crate::error::pk_check(unsafe { PK_SESSION_ask_curr_partition(&mut part) })?;
        crate::error::pk_check(unsafe { PK_PARTITION_make_pmark(part, &mut pmark) })?;
        Some(pmark)
    } else {
        None
    };
    SCOPE.with(|s| {
        *s.borrow_mut() = Some(Scope {
            armed: arm(budget),
            exceeded: None,
            consumed: false,
        })
    });
    let mut exit = ScopeExit { pmark, done: false };
    let result = f();
    let (seen, exceeded) = exit.finish();
    let exceeded = exceeded
        .or_else(|| result.as_ref().err().and_then(|e| classify_error(e, &seen)))
        .or_else(|| {
            // An abort landed in a call the guard did not see (a raw
            // parasolid_sys call): the deadline passed inside the kernel.
            seen.landed.then(|| PsError::BudgetExceeded {
                kind: BudgetKind::Time,
                pk_code: match &result {
                    Ok(_) => PK_ERROR_no_errors,
                    Err(e) => e.details().map_or(0, |d| d.code),
                },
                elapsed: seen.elapsed,
                live_bytes: seen.live_at_landing,
            })
        });
    match exceeded {
        Some(err) => {
            exit.roll_back();
            Err(err)
        }
        None => result,
    }
}
