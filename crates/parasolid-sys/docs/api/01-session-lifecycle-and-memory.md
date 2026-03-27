# Session Lifecycle, Memory Management, and Partitions/Rollback

## 1. Session Lifecycle (Start/Stop/Restart)

### Initialization Order (Critical)

1. **Frustrum Registration (before session start)** — `PK_SESSION_register_frustrum()`
   - Must supply all required callbacks: FSTART, FSTOP, FFOPRD, FFOPWR, FFREAD, FFWRIT, FFCLOS, FMALLO, FMFREE
   - Optional: FABORT, FTMKEY, GO callbacks (GOOPSG, GOSGMT, GOCLSG)
   - Failure to register frustrum before starting session results in error

2. **Session Start** — `PK_SESSION_start(options)`
   - Creates one initial partition (becomes current partition)
   - Initializes session parameters (precision, continuity checks, etc.)
   - Options include: journal file, user field length
   - Calls FSTART frustrum callback once
   - Session is corrupted if start fails; must stop and restart

3. **Optional: Register Delta Frustrum** — `PK_DELTA_register_callbacks()` (must be before start)
   - Required only if using partitioned rollback
   - Provides: open_for_write, open_for_read, close, write, read, delete callbacks
   - Enables pmark/delta functionality

4. **Session Stop** — `PK_SESSION_stop()`
   - Calls FSTOP frustrum callback once
   - Cleans up all memory allocated via FMALLO
   - Resets memory block size to default
   - Clears all session state

5. **Session Restart** — requires full stop then re-register frustrum and start again
   - After fatal error: mandatory approach
   - After serious error: can rollback instead (recommended)
   - Current partition value is NOT affected by partition rollback, only session rollback

**State After Start:** Single current partition exists, at initial pmark if rollback enabled.

## 2. Memory Management

### Ownership Model

- **Parasolid owns all PK-allocated entities** (bodies, faces, edges, etc.)
- Application only holds **tags** (opaque integer handles), not memory pointers
- Parasolid retains internal references and manages lifecycle internally

### Memory Allocation

- Parasolid calls `FMALLO(nbytes, memory_ptr, ifail)` for all internal allocations
- Block size controlled by `PK_MEMORY_set_block_size()` (default ~1/8 MB, range 1/8 MB to 16 MB)
- Larger blocks reduce FMALLO/FMFREE calls but may reduce OS memory reclamation
- Application frustrum decides buffering strategy (e.g., small pool for <0.1 MB, OS allocation for >1 MB)

### Memory Deallocation (Critical Pattern)

```c
// FMFREE signature:
// nbytes: MUST match the original allocation size passed to FMALLO
// memory: pointer-to-pointer; set to NULL on output by application
// ifail: 0 on success
void FMFREE(const int* nbytes, char** memory, int* ifail) {
    free(*memory);
    *memory = NULL;  // CRITICAL: must nullify the pointer
    *ifail = 0;
}
```

**Key Invariant:** FMFREE must receive the same `nbytes` value as the original FMALLO call. Parasolid does not track block sizes; mismatch causes memory leaks or heap corruption.

### Session Memory Queries

- `PK_SESSION_ask_memory_usage()` — total model data structure memory
- `PK_BODY_ask_memory_usage()` — per-body memory footprint

### PK_MEMORY_free Patterns

- Results from enquiry functions (e.g., `PK_SESSION_ask_partitions()`) return arrays allocated by Parasolid via FMALLO
- Application **must** call `PK_MEMORY_free()` on these output arrays
- Failure to free: memory leak in FMALLO pool
- Double-free: causes heap corruption

## 3. Frustrum Callbacks (Application Responsibilities)

### Required Callbacks

| Function | Called When | Threading |
|----------|------------|-----------|
| FSTART | Session starts | Single thread during session init |
| FSTOP | Session stops | Single thread during session cleanup |
| FMALLO | Parasolid needs memory | May be called from any thread; must be thread-safe |
| FMFREE | Parasolid releases memory | May be called from any thread; must be thread-safe |
| FFOPRD | Open file for reading | Single-threaded or thread-safe via application mutex |
| FFOPWR | Open file for writing | Single-threaded or thread-safe |
| FFREAD | Read from open file | Single-threaded or thread-safe |
| FFWRIT | Write to open file | Single-threaded or thread-safe |
| FFCLOS | Close open file | Single-threaded or thread-safe |

### Threading Concerns (Critical)

- FMALLO/FMFREE: **Must be thread-safe**, called concurrently from different application threads
- File callbacks (FFOPRD, FFOPWR, FFREAD, FFWRIT, FFCLOS): Application must provide thread safety if multi-threaded
  - Parasolid does NOT serialize file I/O; if multiple threads call file operations simultaneously, application must protect via mutex/channel
  - File handle context (strid) is opaque; application must map strid to actual file descriptor and protect access
- FSTART/FSTOP: Called only during session start/stop, not concurrently
- GO callbacks: Optional; called during rendering from potentially multiple threads

### Callback Error Handling

- Callbacks signal failure via `ifail` output parameter (0=success, nonzero=failure)
- Parasolid checks `ifail` and propagates error to calling PK function
- Exception throwing from callbacks: **Not supported; must use `ifail` pattern**

## 4. Partitions (Entity Ownership Scoping)

### Purpose

- Group related entities (bodies, faces, orphan geometry, transforms) into independent rollback units
- Partition is the scope of entity ownership: entities cannot reference entities in different partitions

### Key Rules

| Rule | Implication |
|------|-------------|
| Inter-partition references forbidden | Boolean operations, face attachment, surface intersection only allowed on entities in same partition |
| New entities go to current partition | When partition not determined by reference, entities created in `PK_SESSION_ask_curr_partition()` |
| Current partition affects rollback | Session rollback restores current partition; partition rollback does not |
| Entities cannot move freely | Use `PK_BODY_change_partition()` only on NEW bodies (created since last roll); old bodies must be copied |
| One partition is always current | Set via `PK_PARTITION_set_current()` |

### Partition Types

- **Standard** (default): Full delta history, slower first non-initial pmark advance
- **Light**: No backward delta at first non-initial pmark, faster advance/delete of first pmark
  - Use for frequent pmark advance scenarios
  - Set before creating pmarks if possible

### Partition Lifecycle

1. Created at session start (initial partition, empty)
2. New partitions via `PK_PARTITION_create()`
3. Set current via `PK_PARTITION_set_current()`
4. Deleted via `PK_PARTITION_delete()` (only when not current)
5. Copied via `PK_PARTITION_copy()` with delta options
6. Merged via `PK_PARTITION_merge()` to combine multiple partitions

### Session Marks and Partitions

- `PK_MARK_create()` creates pmarks in all partitions not already at a pmark
- Deleting partition P: if P is current at any session mark, application-selected partition C becomes current at those marks
- Receiving transmit file: bodies created in current partition

## 5. Rollback: Marks, Undo/Redo, Entity Validity

### Two Levels of Rollback

| Type | Scope | Use Case | API |
|------|-------|----------|-----|
| Partitioned | Single partition | Feature-model update, alternative designs | `PK_PMARK_create()`, `PK_PMARK_goto_2()` |
| Session | Entire session | User undo/redo across multiple partitions | `PK_MARK_create()`, `PK_MARK_goto_2()` |

### Pmarks (Partition Marks)

- Record state of single partition at point in time
- Organized as directed acyclic graph (pmark graph): each pmark has preceding pmark, zero or more following pmarks
- Initial pmark created at partition creation
- Advanced via `PK_PARTITION_advance_pmark()` or cloned via `PK_PARTITION_clone_pmark()`

### Deltas (Changes Between Pmarks)

- Record entity creation, modification, deletion between adjacent pmarks
- Stored in delta frustrum files via `PK_DELTA_register_callbacks()`
- Computed at pmark creation
- Retrieved during rollback for efficiency

### Rolling to Pmark — Entity Changes

- `PK_PMARK_goto_2(pmark, options, result)` rolls partition to specified pmark
- Result contains arrays:
  - `new_entities[]`: entities dead before roll, alive after (resurrected)
  - `mod_entities[]`: entities modified by roll (exist before and after)
  - `del_entities[]`: entities alive before roll, dead after
- **Critical**: Tags of rolled-back entities change validity
  - Tag may be reused if entity deleted and later recreated
  - Application must invalidate cached references to deleted entities

### Session Marks

- Record state of entire session (all partitions at specific pmarks)
- Created via `PK_MARK_create()` or `PK_MARK_create_2()`
- Rolled to via `PK_MARK_goto_2()`: all partitions roll simultaneously to their designated pmarks
- Session mark preserves current partition; rolling back also restores that property

### What-If Query (Pre-roll Information)

- `PK_PMARK_ask_entities(pmark, result)` returns entities that would change without actually rolling
- Useful for UI to preview undo/redo effects

### No-Roll Attributes (Persistent Across Rollback)

- Attributes marked "no-roll" retain values during partition rollback
- Query via `PK_ATTRIB_ask_no_roll()`
- Useful for application-specific metadata (e.g., feature names)

### Partition Guards (Safety Mechanism)

- `PK_PARTITION_set_guard(partition, pmark)` prevents rolling below guard pmark
- `PK_PARTITION_goto_guard()` rolls to guard
- Use to protect critical model states

## 6. Thread Safety (Multi-Threading Architecture)

### Parasolid Thread Model

- **Thread-safe**: Can be called concurrently from multiple application threads
- Parasolid manages queuing and locking internally; application does NOT need mutexes for PK calls

### Function Classification (Affects Queuing)

| Class | Concurrency | Queue Behavior |
|-------|-----------|--------|
| **Concurrent** | Multiple threads simultaneous | Runs immediately unless limit exceeded; waits if exclusive holds |
| **Exclusive** | Single thread only | Waits until all other threads exit; blocks concurrent threads |
| **Locally Exclusive** | Thread-specific behavior | If partition locked to thread: runs concurrent; else: exclusive |

### Thread Chaining (Performance Optimization)

- Reduces lock/unlock overhead by bundling multiple PK calls
- `PK_THREAD_chain_start(type, length, local_level)` to `PK_THREAD_chain_stop()`
- Within chain: exclusive functions get exclusive lock per function; concurrent functions run together

### Partition Locking (Thread-Local Modeling)

- `PK_THREAD_lock_partitions(partitions)` locks partition to current thread
- Allows this thread to call locally-exclusive functions concurrently
- Other threads can call locally-exclusive on different partitions simultaneously
- Use for multi-partition feature update across threads

### Error Exclusion After Serious Error (Critical)

- Serious error: Session may be corrupted; Parasolid blocks other threads from entering
- Solution 1 (recommended): `PK_PMARK_goto_2()` or `PK_MARK_goto_2()` to rollback (clears exclusion)
- Solution 2: `PK_THREAD_clear_exclusion()` (only after manual cleanup; dangerous)
- Fatal error: Must call `PK_SESSION_stop()`; other threads get `PK_ERROR_modeller_not_started`

### Longjump Error Handling

- If error handler uses `setjmp`/`longjmp`, **must** call `PK_THREAD_tidy()` after longjump to restore Parasolid state for calling thread

## 7. Key Safety Invariants for Rust Wrapper

| Pattern | Enforcement |
|---------|-------------|
| Frustrum registration -> start | Panic if out of order |
| FMFREE size matching | RAII wrapper for all PK arrays |
| Entity validity after rollback | Version-tagged entity references |
| Multi-thread file I/O | Mutex or exclusive chain |
| Current partition tracking | State machine in wrapper |
| Serious error handling | Attempt rollback or stop session |
| Callback errors | Check ifail, propagate as Result |
