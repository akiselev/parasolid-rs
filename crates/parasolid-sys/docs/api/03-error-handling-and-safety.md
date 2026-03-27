# Error Handling, Signal Handling, and Safety Boundaries

## 1. Error Code System

### Return Type

- All PK functions return `PK_ERROR_t` (alias for `PK_ERROR_code_t`, which is `c_int`)
- **0 (`PK_ERROR_no_errors`)** = success
- **Non-zero** = error occurred

### Three Severity Levels

| Severity | Value | State | Recovery Action |
|----------|-------|-------|-----------------|
| **Mild** | 1 | Operation failed, parts not altered | Continue normally; can retry |
| **Serious** | 2 | Parts may be altered/invalid; rest of session intact | **MUST rollback** to valid state or restart session |
| **Fatal** | 3 | Session corrupted; rollback ineffective | **MUST stop and restart** session |

### Error Codes

| Code | Constant | Meaning | Required Action |
|------|----------|---------|-----------------|
| 0 | `PK_ERROR_no_errors` | Success | None |
| 1 | `PK_ERROR_general` | General failure | Per severity |
| 2 | `PK_ERROR_system_error` | Unexpected failure | Per severity; enable argument checking and retry |
| 3 | `PK_ERROR_fatal_error` | Session corrupted | Stop and restart session immediately |
| 4 | `PK_ERROR_unhandleable_condition` | Undiagnosable error | Only recoverable with exception-throwing error handler |
| 5 | `PK_ERROR_run_time_error` | OS signal caught in PK code | Per severity |
| 6 | `PK_ERROR_aborted` | User interrupt via signal handler | Session valid; can continue or retry |
| 7 | `PK_ERROR_cant_be_aborted` | Non-interruptible function | No action; function continues normally |
| 8 | `PK_ERROR_fru_error` | Error in frustrum callback | Per severity |
| 502 | `PK_ERROR_distance_le_0` | Negative geometric parameter | Fix input |
| 504 | `PK_ERROR_not_an_entity` | Bad/dead tag | Fix tag reference |

### Failure Status Codes (NOT Error Codes)

Some functions return `PK_ERROR_no_errors` but set a **status code** in an output argument:

- `PK_blend_fault_t` — blend operation faults
- `PK_BODY_fault_t` — body operation faults
- `PK_local_status_t` — local operation failures (may corrupt model — **must rollback**)
- `PK_boolean_result_t` — boolean result status
- `PK_check_state_t` — checking state
- `PK_section_report_t` — sectioning reports

**Error handlers are NOT called** for status code failures; the application must explicitly check output arguments.

## 2. Error Handler Callbacks

### Registration

```c
pub struct PK_ERROR_frustrum_t {
    pub handler_fn: PK_ERROR_handler_fn_t,
}

pub type PK_ERROR_handler_fn_t =
    Option<unsafe extern "C" fn(error_sf: *const PK_ERROR_sf_t) -> PK_ERROR_code_t>;

// Register via:
PK_ERROR_register_callbacks(&frustrum_struct);
```

### Error Information Passed to Handler

```c
pub struct PK_ERROR_sf_t {
    pub function: *const c_char,           // failing PK function name
    pub code: PK_ERROR_code_t,             // error code
    pub severity: PK_ERROR_severity_t,     // mild/serious/fatal
    pub n_bad_args: c_int,                 // number of invalid args
    pub bad_args: [c_int; 20],             // indices of bad args
    pub bad_arg_names: [*const c_char; 20], // names of bad args
    pub entity: PK_ENTITY_t,               // affected entity (if any)
}
```

### Handler Capabilities and Restrictions

| What It Can Do | What It CANNOT Do |
|---|---|
| Set application error flags | Modify `PK_ERROR_sf_t` structure |
| Perform application-specific cleanup | Call PK_ERROR or PK_THREAD_ERROR functions from within handler |
| Throw exceptions (if used with try/catch) | Call any PK_* functions (except `PK_THREAD_tidy` before throwing) |
| Store error information for later processing | Return a different error code |
| Provide diagnostic information | Call re-entrant PK functions |

### Exception-Throwing Handlers

- If handler throws exception, **MUST call `PK_THREAD_tidy()` before throwing**
- Exception must be thrown to code **outside all PK function calls**
- Only one level of re-entrance permitted

## 3. Error Recovery Strategies

### After Mild Error

- Operation failed but model untouched
- Can continue normally or retry with different data
- No recovery action needed

### After Serious Error

```
Option 1 (recommended): Roll back to valid checkpoint
  PK_SESSION_ask_curr_partition() -> partition
  PK_PARTITION_ask_pmark(partition) -> pmark
  PK_PMARK_goto_2(pmark) -> restore state

Option 2: Restart session (if no rollback available)
  PK_SESSION_stop()
  Re-register frustrum
  PK_SESSION_start()
```

**Multi-partition systems**: Must rollback **every partition** containing entities affected by the error.

**Multi-threaded applications**: After serious/fatal error in one thread, call `PK_THREAD_ask_exclusion` to assess; either rollback or `PK_THREAD_clear_exclusion`.

### After Fatal Error

- **MUST** stop and restart session
- Occasionally may need to exit application entirely
- `PK_SESSION_stop()` → re-register frustrum → `PK_SESSION_start()`

## 4. Signal Handling

### Parasolid Function Classification

| Category | Characteristics | Error Recovery |
|----------|---|---|
| **Lightweight** | Read-only calls (PK_BSURF_ask, etc) | No protection; exit if signal caught |
| **Heavyweight-Protected** | Modifies model; has recovery mechanism | Can abort safely |
| **Heavyweight-Unprotected** | Read-only part of heavyweight function | No protection like lightweight |

### Determining Code State at Signal Time

```c
PK_SESSION_is_in_kernel_2(&is_in_kernel, &is_protected, &is_subthread);
// is_in_kernel:  true = executing PK function
// is_protected:  true = in protected section with rollback capability
// is_subthread:  true = executing from internal Parasolid thread
```

### Signal Handler Outcomes

| Error Handler? | Code State | Outcome |
|---|---|---|
| No | Protected | Returns error code to application |
| No | Unprotected | Run-time errors force exit |
| Yes (returns) | Protected | Error passed to handler; error code returned |
| Yes (returns) | Unprotected | Run-time errors force exit |
| Yes (throws) | Any | Handler throws exception to application |

### Error Codes from Signals

- Protected + run-time error (abortable): `PK_ERROR_run_time_error` or `PK_ERROR_fru_error`
- Protected + run-time error (non-abortable): `PK_ERROR_fatal_error`
- Protected + user interrupt (abortable): `PK_ERROR_aborted`
- Protected + user interrupt (non-abortable): continues normally
- Unprotected: no recovery; exit

## 5. C Binding Implementation

### Memory Conventions

| Convention | Meaning |
|---|---|
| Input argument | Caller allocates; PK reads |
| Output argument | Caller allocates; PK writes |
| Input/output option struct | Caller allocates and initializes; passed by pointer |
| Array output | PK allocates and returns pointer via out-param |
| PK-allocated memory | Caller must free with `PK_MEMORY_free()` |

### Array Output Pattern

```c
PK_ENTITY_t *entities;
int n_entities;
PK_PARTITION_ask_bodies(partition, &n_entities, &entities);
// ... use entities ...
if (n_entities) PK_MEMORY_free(entities);
```

### Struct Initialization

Parasolid C uses initialization macros:
```c
PK_BCURVE_fit_data_t fit_data = PK_BCURVE_fit_data_m(fit_data);
```

All fields set to safe defaults. Wrapper must replicate these defaults in `Default` impls.

## 6. Session vs Local Precision

### Session Precision (Global)

- **Linear precision**: Default ~1.0e-8 units; distances <= this are zero
- **Angular precision**: Default ~1.0e-11 radians; angles <= this are zero
- All geometry must fit in **size box**: 1000x1000x1000 centered at origin
- Points not intended to be coincident: separate by >100x linear precision (>1.0e-6)

### Local Precision (Tolerant Modelling)

For imported parts with inconsistent topology/geometry:

- `PK_EDGE_set_precision_2(edge, precision_value, &options)` — edges treated as "tubes"
- `PK_EDGE_ask_precision(edge, &precision)` — query current precision
- `PK_EDGE_reset_precision_2(edge, method, &options)` — remove local precision
- `PK_LOOP_close_gaps(loop, &options)` / `PK_FACE_close_gaps(face, &options)` — gap closing

## 7. Mapping PK Errors to Rust Result Types

### Recommended Error Enum

```rust
#[derive(Debug)]
pub enum PsError {
    Mild(ErrorDetails),
    Serious(ErrorDetails),
    Fatal(ErrorDetails),
    RunTimeError(ErrorDetails),
    Aborted,
    CantAbort,
    UnhandleableCondition,
    SystemError(ErrorDetails),
    NotAnEntity(PK_ENTITY_t),
    FailureStatus(String),
}

pub struct ErrorDetails {
    code: u32,
    function: String,
    bad_args: Vec<(usize, Option<String>)>,
    entity: Option<PK_ENTITY_t>,
}
```

### Error Checking Macro

```rust
macro_rules! pk_call {
    ($call:expr) => {{
        let status = unsafe { $call };
        if status != PK_ERROR_no_errors {
            return Err(PsError::from_last_error(status));
        }
    }};
}
```

### Must Check Failure Status Codes Separately

Functions returning `PK_ERROR_no_errors` may still fail via status code. The wrapper must check both the return code and any status output parameters.

## 8. Safety Boundaries

### Session Corruption Points (FATAL)

| Condition | Recovery |
|---|---|
| Unhandled `PK_ERROR_fatal` | Restart session |
| Unhandled `PK_ERROR_fatal_error` | Restart session immediately |
| `PK_ERROR_unhandleable_condition` without exception handler | Exit application |
| Run-time error in unprotected code without exception handler | Exit application |
| Calling PK functions from error handler | Unpredictable results |

### Model Corruption Points (SERIOUS — requires rollback)

| Condition | Recovery |
|---|---|
| Serious error during model modification | **MUST rollback** partition |
| `PK_local_status_t` failure | **MUST rollback** |
| `PK_boolean_result_failed_c` | **SHOULD rollback** |
| Multi-partition error | Rollback **all affected partitions** |

### Recoverable (MILD)

| Condition | Action |
|---|---|
| Mild error (model untouched) | Retry with different inputs |
| Argument validation failure | Fix arguments and retry |
| Geometry precision mismatch | Scale input or increase tolerance |

### Safe Operations (No Corruption Risk)

- Read-only operations (`ask_*`, `check_*`)
- Setting attributes on existing entities
- Reading error information via `PK_ERROR_ask_last`
- Calling `PK_SESSION_is_in_kernel_2` from signal handler
- Calling `PK_SESSION_abort` from signal handler
- Calling `PK_THREAD_tidy` before throwing from error handler
