# CLAUDE.md

Guidance for working in `parasolid-rs`. Read this first, then `TODO.md` for the
validation roadmap and `docs/pskernel-solidworks.md` for the war stories behind
the current FFI fixes.

## What this repo is

Rust FFI bindings (`parasolid-sys`) and a safe wrapper (`parasolid`) for the
Siemens **Parasolid** kernel (`pskernel`), plus an integration harness
(`parasolid-test`). The end goal is to use Parasolid as a **golden oracle** for
the CADabra geometric kernel — so every binding we rely on must be validated
end-to-end against the real `pskernel.dll` (Parasolid **V37.01.243**,
SOLIDWORKS 2025).

## Proprietary binary — never commit

`pskernel.dll`, `libpskernel.a`, and the Parasolid headers are proprietary
Siemens IP and must never be committed. `lib/` and the DLL are git-ignored. See
`README.md` and `scripts/scrub-proprietary-binaries.sh`.

## Running the tests under Wine (Linux host, cross-compile)

The crates target `x86_64-pc-windows-gnu` and run under Wine against the real
DLL. `.cargo/config.toml` already sets the mingw linker and a `wine` runner.

### One-time setup

On Arch/WSL the toolchain comes from official repos (no AUR needed despite the
"install with yay" framing — these are all in `extra`/`multilib`):

```bash
sudo pacman -S --needed mingw-w64-gcc mingw-w64-binutils wine
rustup target add x86_64-pc-windows-gnu   # (usually already installed)
```

`mingw-w64-gcc` provides `x86_64-w64-mingw32-gcc` (the linker) and
`mingw-w64-binutils` provides `x86_64-w64-mingw32-dlltool`. On current Arch the
Wine binary is just `wine` (there is no separate `wine64`); plain `wine` runs
the 64-bit exe, which is what the `runner` in `.cargo/config.toml` invokes.

### Generate the import library (`lib/libpskernel.a`)

The crate link-time links the kernel (`#[link(name = "pskernel")]`, with
`-L lib` in `.cargo/config.toml`), so the linker needs `lib/libpskernel.a`. We
only ship `pskernel.dll`, so generate the import lib from the DLL's export list
(`crates/parasolid-sys/pk_exports.txt`):

```bash
mkdir -p lib
{ echo "LIBRARY pskernel.dll"; echo "EXPORTS"; sed '/^\s*$/d' crates/parasolid-sys/pk_exports.txt; } > lib/pskernel.def
x86_64-w64-mingw32-dlltool -d lib/pskernel.def -D pskernel.dll -l lib/libpskernel.a
```

Both `lib/*.a` and `lib/*.def` are git-ignored — regenerate them locally.

### Build & run

```bash
cargo build -p parasolid-test --target x86_64-pc-windows-gnu

# Make pskernel.dll findable by Wine's loader: copy it next to the exe
# (or put its directory on WINEPATH).
cp pskernel.dll target/x86_64-pc-windows-gnu/debug/

# Either invoke Wine directly:
WINEDEBUG=-all wine target/x86_64-pc-windows-gnu/debug/parasolid-test.exe
# ...or use cargo's configured wine runner (needs an explicit --bin because the
# crate also has a `probe` binary):
WINEDEBUG=-all cargo run -p parasolid-test --bin parasolid-test --target x86_64-pc-windows-gnu
```

Expected tail: `=== Results: N passed, 0 failed ===`. Harmless `libEGL/DRI3`
warnings appear under WSL (no GPU/X); they don't affect the headless kernel.

The `probe` binary (`crates/parasolid-test/src/bin/probe.rs`) empirically reads
enum/token values out of the DLL — run it the same way when you need to pin down
a constant the docs don't publish.

## Validation methodology (see `TODO.md` for the full version)

Every binding we depend on gets: (1) a **signature audit** against the mirrored
V35 header docs, (2) a **runtime test** under Wine asserting concrete
numeric/topological output (not just "no error"), (3) an **enum probe** for any
value not in the docs, annotated `[probed]`/`[family]`/`[guess]`/`[unknown]` in
`parasolid-sys`, and (4) a note of residual risk in
`docs/pskernel-solidworks.md`.

Treat everything unaudited as suspect: `parasolid-sys` has ~1150 `extern` fns
and only a few dozen are runtime-validated. Grep for `[guess]` / `[unknown]`
before relying on a constant.

## Conventions

- Safe wrappers live in `crates/parasolid/src/*.rs`; each raw call goes through
  the `pk_call!` macro (in `error.rs`), which maps `PK_ERROR_*` codes to
  `PsError`. Follow the existing per-type module layout (`body.rs`, `surf.rs`, …).
- Tests are assertion-based in `crates/parasolid-test/src/main.rs`, run as one
  Wine binary. Keep that single-binary runner; group new tests by P-level.
- `Session` is a runtime singleton and `!Send`/`!Sync`. Tests start/stop a
  fresh session per case.

## Track the latest API — always

**Policy: this crate targets the newest behaviour and the newest option-struct
version the installed kernel accepts.** We do not care about reproducing older
Parasolid releases. Siemens keeps those paths for customers who need
bit-identical output for parts designed decades ago; we are building an oracle
and want the current algorithms.

Two independent mechanisms control this — do not confuse them:

1. **Session behaviour** — `PK_SESSION_set_behaviour`. A session that never sets
   it reports `Unset`, meaning "use the original system switches", i.e. the
   legacy path. `SessionConfig` therefore defaults to `Behaviour::Latest`, and
   the call's `status` output is checked (the kernel can decline or clamp a
   request and still return no error).
2. **`o_t_version`** — the first field of each individual `PK_*_o_t` options
   struct, stamped by the caller. There is no global switch: it is per struct.

### Rules for `o_t_version`

- **Stamp the highest version the entry point accepts**, never 1 by default.
- **Fields are version-gated.** The kernel migrates your struct from the stamped
  version, copying only that version's prefix and overwriting the rest with its
  own defaults. Stamping too low makes later fields **silently dead** — measured
  cases: `range_type` was ignored so "give me the maximum distance" returned the
  *minimum* with no error, and `mixed_curve_category` was ignored in SSI.
- **Too high is an error, not a clamp** — `PK_ERROR_o_t_version_unknown` (5022).
  Some versions are known but unimplemented (5000). So the ceiling must be
  *probed*, per struct.
- **A higher version means the kernel reads more fields**, so every one of them
  needs a legal value. `0` is not a legal token for most enum fields and gives
  `PK_ERROR_field_of_wrong_type` (5014). **Never `mem::zeroed()` an options
  struct that contains enum fields and expect it to work at a high version.**
- The sweep procedure and worked examples are in
  [`docs/option-version-protocol.md`](docs/option-version-protocol.md). Use it
  before trusting any recovered layout — journal helpers describe the kernel's
  *post-migration* struct, not the caller's.

A quick way to tell whether a field is live: set it to a garbage token. If the
call still succeeds, the kernel is not reading it and your version is too low.
