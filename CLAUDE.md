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
