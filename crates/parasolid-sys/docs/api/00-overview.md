# Safe Rust API Wrapper for parasolid-sys — Research Overview

Research conducted to inform the design of a safe Rust wrapper crate around the raw `parasolid-sys` FFI bindings.

## Documents

| # | File | Topic |
|---|------|-------|
| 01 | [Session Lifecycle and Memory](01-session-lifecycle-and-memory.md) | Session init/stop order, memory ownership, frustrum callbacks, partitions, rollback, thread safety |
| 02 | [Entity Types and Hierarchy](02-entity-types-and-hierarchy.md) | Tag system, class hierarchy, body types, topological data structure, tracking, options-struct pattern |
| 03 | [Error Handling and Safety](03-error-handling-and-safety.md) | Error codes/severity, error handlers, signal handling, C binding implementation, precision, safety boundaries |
| 04 | [Geometry and Operations](04-geometry-and-operations.md) | Geometry creation, NURBS, enquiry patterns, booleans, file I/O, faceting, assemblies, API patterns |
| 05 | [Rust Wrapper Patterns](05-rust-wrapper-patterns.md) | Prior art (opencascade-rs, rusqlite, glutin), recommended patterns, anti-patterns |

## Key Design Decisions

1. **Session as root lifetime** — all entity handles carry `'session`, preventing use-after-stop
2. **Newtype per entity class** — `Body`, `Face`, `Edge`, `Surf`, `Curve` etc. with class-specific methods
3. **`pk_call!` macro** — every FFI call goes through error checking; returns `Result<T, PsError>`
4. **RAII for PK-allocated arrays** — `PkArray<T>` calls `PK_MEMORY_free` on drop
5. **Builder pattern for options** — mirrors `PK_*_o_m()` defaults with Rust `Default` + method chaining
6. **Move semantics for consuming ops** — booleans take `Vec<Body>` by value, preventing reuse of consumed tools
7. **`catch_unwind` in all callbacks** — panics must never cross FFI boundary
8. **`!Send + !Sync` by default** — session and entity handles are not thread-safe unless explicitly opted in
9. **Automatic rollback on serious errors** — mark/rollback guards like database transactions
10. **Separate error codes from failure status** — both checked, both surfaced through `Result`

## Sources

- Parasolid XT Reference Manual (chapters 1-125) — `~/cadatomic/solidworks/notes/xt/reference/`
- `parasolid-sys` FFI bindings — `~/cadatomic/parasolid-sys/src/`
- [opencascade-rs](https://github.com/bschwind/opencascade-rs) — OCCT Rust wrapper
- [truck](https://github.com/ricosjp/truck) — Pure-Rust B-rep kernel
- [rusqlite](https://github.com/rusqlite/rusqlite) — SQLite Rust wrapper (lifetime patterns)
- [glutin](https://docs.rs/glutin/) — OpenGL context management (typestate pattern)
- [Effective Rust Item 34](https://effective-rust.com/ffi.html) — FFI boundary control
