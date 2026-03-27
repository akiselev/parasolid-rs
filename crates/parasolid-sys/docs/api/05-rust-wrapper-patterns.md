# Safe Rust Wrapper Patterns — Prior Art and Recommendations

Research from opencascade-rs, truck, rusqlite, glutin, and general Rust FFI best practices.

## 1. Entity Handle Types

### Pattern A: UniquePtr Newtype (opencascade-rs)

opencascade-rs wraps each OCCT topological type as a Rust struct with `cxx::UniquePtr`:

```rust
pub struct Edge {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Edge>,
}

pub struct Face {
    pub(crate) inner: UniquePtr<ffi::TopoDS_Face>,
}
```

- `inner` is `pub(crate)` — users cannot construct invalid handles
- `UniquePtr` automatically calls C++ destructor on Drop
- Each topological level gets its own Rust struct

### Pattern B: Arc<Mutex<T>> Shared Ownership (truck)

Truck uses reference-counted interior mutability for shared geometry:

```rust
Vertex<P>       // stores Arc<Mutex<P>>
Edge<P,C>       // references two vertices + Arc<Mutex<C>> for the curve
Face<P,C,S>     // boundary wires + Arc<Mutex<S>> for the surface
```

Avoids lifetime annotations. Cloning is cheap (Arc refcount). Tradeoff: runtime synchronization.

### Pattern C: Integer Tag + PhantomData (recommended for Parasolid)

Parasolid uses integer tags, not pointers. Entities are owned by a session/partition:

```rust
pub struct Body<'s> {
    tag: PK_BODY_t,
    _session: PhantomData<&'s Session>,
}

pub struct Face<'s> {
    tag: PK_FACE_t,
    _session: PhantomData<&'s Session>,
}
```

- Tags are `pub(crate)` — cannot construct invalid handles externally
- `PhantomData<&'s Session>` ties handle lifetime to session
- Compiler prevents use after `Session` is dropped

## 2. Session/Context Lifetime Management

### Pattern: Rusqlite's Hierarchical Lifetimes

```
Connection -> RefCell<InnerConnection> -> *mut sqlite3
Statement<'conn> -> RawStatement -> *mut sqlite3_stmt + PhantomData<&'conn ()>
Row<'stmt> -> PhantomData<&'stmt Statement>
```

Compiler enforces: Statement cannot outlive Connection. Row cannot outlive Statement. Prevents use-after-free at compile time.

### Pattern: Glutin's Typestate for State Machines

Glutin encodes OpenGL context state at the type level:

```rust
// Illegal state transitions are compile errors
fn make_current(self, surface: &Surface) -> PossiblyCurrentContext;
fn make_not_current(self) -> NotCurrentContext;
```

`NotCurrentContext` is `Send` (can move between threads). `PossiblyCurrentContext` is NOT `Send` or `Sync`.

### Recommended for Parasolid

```rust
pub struct Session {
    _not_send: PhantomData<*const ()>,  // !Send + !Sync by default
}

impl Session {
    pub fn start(frustrum: Frustrum) -> Result<Session> { ... }
}

impl Drop for Session {
    fn drop(&mut self) {
        unsafe { PK_SESSION_stop(); }
    }
}
```

Optional typestate for session lifecycle:

```rust
pub struct SessionBuilder;  // Before start
pub struct Session;         // Active
// SessionBuilder consumed by start() -> Session
// Session consumed by stop() or Drop
```

## 3. Error Mapping

### Pattern: thiserror Enum + From

```rust
#[derive(Debug, thiserror::Error)]
pub enum PsError {
    #[error("mild error in {function}: {code}")]
    Mild { code: u32, function: String },

    #[error("serious error in {function}: {code} — rollback required")]
    Serious { code: u32, function: String },

    #[error("fatal error — session corrupted")]
    Fatal { code: u32, function: String },

    #[error("entity tag {0} is no longer valid")]
    NotAnEntity(i32),

    #[error("operation aborted by user")]
    Aborted,

    #[error("failure status: {0}")]
    FailureStatus(String),
}
```

### Pattern: Macro for Repetitive Status Checks

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

Every FFI call goes through `pk_call!`, making it impossible to forget error checking.

## 4. Memory Management

### Rule: Allocate and Free on the Same Side

If C allocates, C must free. If Rust allocates, Rust must free.

### RAII Wrapper for PK-Allocated Arrays

```rust
struct PkArray<T> {
    ptr: *mut T,
    len: usize,
}

impl<T> Drop for PkArray<T> {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { PK_MEMORY_free(self.ptr as *mut c_void); }
        }
    }
}

impl<T> Deref for PkArray<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}
```

### Session Drop

Session owns everything. Drop calls `PK_SESSION_stop()`, which cleans up all internal memory.

## 5. Builder Patterns for Option Structs

### Default + Method-Chaining Builder

```rust
pub struct BooleanOptions {
    tolerance: f64,
    keep_tools: bool,
    match_style: MatchStyle,
}

impl Default for BooleanOptions {
    fn default() -> Self {
        Self {
            tolerance: 1e-6,
            keep_tools: false,
            match_style: MatchStyle::Auto,
        }
    }
}

impl BooleanOptions {
    pub fn tolerance(mut self, tol: f64) -> Self { self.tolerance = tol; self }
    pub fn keep_tools(mut self, keep: bool) -> Self { self.keep_tools = keep; self }

    fn to_ffi(&self) -> PK_BODY_boolean_2_o_t {
        PK_BODY_boolean_2_o_t {
            o_t_version: 1,
            tolerance: self.tolerance,
            keep_tools: if self.keep_tools { 1 } else { 0 },
            match_style: self.match_style as c_int,
            // ... fill defaults for other fields
        }
    }
}
```

Maps directly to Parasolid's `PK_*_o_m()` initialization macros.

## 6. Callback Handling (C into Rust)

### Pattern: catch_unwind at the Boundary

Panics must NEVER unwind across FFI boundaries:

```rust
unsafe extern "C" fn fmallo_wrapper(
    nbytes: *const c_int,
    memory: *mut *mut c_char,
    ifail: *mut c_int,
) {
    if let Err(_) = std::panic::catch_unwind(|| {
        let size = *nbytes as usize;
        let layout = Layout::from_size_align(size, 8).unwrap();
        *memory = std::alloc::alloc(layout) as *mut c_char;
        *ifail = if (*memory).is_null() { 1 } else { 0 };
    }) {
        std::process::abort();
    }
}
```

### Pattern: Trait-Based Callback Abstraction

```rust
pub trait Frustrum: Send + Sync {
    fn alloc(&self, nbytes: usize) -> Option<*mut u8>;
    fn free(&self, ptr: *mut u8, nbytes: usize);
    fn open_read(&self, key: &str) -> Result<Box<dyn Read>>;
    fn open_write(&self, key: &str) -> Result<Box<dyn Write>>;
    // ...
}
```

Application implements the trait. Wrapper generates `extern "C"` trampolines that dispatch to the trait object stored in thread-local or global state.

## 7. Thread Safety Declarations

### Non-Send/Non-Sync by Default

For Parasolid (which manages its own internal locking):

```rust
pub struct Session {
    _not_send: PhantomData<*const ()>,
}
```

Entity handles carry session lifetime, so they inherit `!Send + !Sync`.

If the application needs multi-threading, it can use `PK_THREAD_*` APIs through an explicit escape hatch, but the wrapper defaults to single-threaded safety.

## 8. Key Anti-Patterns to Avoid

1. **Exposing raw tags publicly** — users can construct invalid handles. Use `pub(crate)` for inner fields.

2. **No lifetime binding between parent and child** — entity handles without session lifetime allow use-after-stop.

3. **Allowing panics across FFI** — undefined behavior. Always `catch_unwind` in callbacks.

4. **Silent null pointer propagation** — check every pointer from C; return `Result` or `Option`.

5. **Manual Drop without RAII** — forgetting cleanup. Always implement `Drop` on wrapper types.

6. **Mismatched allocation/deallocation** — freeing PK-allocated memory with Rust's allocator corrupts the heap. Always use `PK_MEMORY_free()`.

7. **Trusting C enum ranges** — C can return values outside defined range. Have an `Unknown(i32)` variant.

8. **`#[repr(C)]` omission** — structs passed across FFI must have `#[repr(C)]`.

## 9. Recommended Pattern Summary

| Concern | Pattern | Exemplar |
|---------|---------|----------|
| Entity handles | `Newtype<PK_*_t>` + `PhantomData<&'session>` | rusqlite `Statement<'conn>` |
| Session lifecycle | RAII `Drop` + optional typestate | glutin context management |
| Error mapping | `From<PK_ERROR_t>` → thiserror enum + `pk_call!()` macro | opencascade-rs |
| Output arrays | RAII `PkArray<T>` calling `PK_MEMORY_free` on drop | Standard FFI pattern |
| Option structs | `Default` impl + method-chaining builder | Common Rust idiom |
| Body consumption | Move semantics — `fn boolean(tools: Vec<Body>)` consumes tools | Ownership transfer |
| Frustrum callbacks | `extern "C"` + `catch_unwind` + abort on panic | General FFI best practice |
| Thread safety | `!Send + !Sync` via `PhantomData<*const ()>` | glutin `PossiblyCurrentContext` |
| Class hierarchy | One newtype per PK class; shared traits for common ops | opencascade-rs topology types |
| B-geometry | Owned `Vec<f64>` for control points; borrow into FFI call | Standard Rust ownership |
