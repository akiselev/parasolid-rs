# Running parasolid-rs against the SOLIDWORKS pskernel.dll

Status (2026-07-14): **all 14 integration tests pass** against the
`pskernel.dll` shipped with SOLIDWORKS 2025 (Parasolid V37.01.243).
`lib/pskernel.dll` in this repo is byte-identical (SHA-256) to
`C:\Program Files\SOLIDWORKS Corp\SOLIDWORKS\pskernel.dll`.

## Build & run (Linux host, cross-compile + wine)

```bash
rustup target add x86_64-pc-windows-gnu   # plus a mingw-w64 toolchain
cargo build --workspace --target x86_64-pc-windows-gnu
# copy lib/pskernel.dll next to the exe (or set WINEPATH)
wine64 target/x86_64-pc-windows-gnu/debug/parasolid-test.exe
```

Notes for llvm-mingw (no libgcc): provide `libgcc.a`/`libgcc_eh.a` shims
(copies of `libclang_rt.builtins-x86_64.a` and `libunwind.a`) on the
linker search path.

On native Windows the same crates should link with the `x86_64-pc-windows-gnu`
toolchain against `lib/libpskernel.a`; put the SOLIDWORKS directory on `PATH`
(or copy `pskernel.dll` next to the exe) at runtime.

## What was wrong, and how it was fixed

The FFI layer had been drafted from prose docs and guesswork; the kernel
crashed inside `PK_SESSION_start`. Fixes, all verified against the
Parasolid V35 per-symbol header docs (mirrored in
`solidworks-notes/headers/`) or probed empirically against the DLL:

1. **`PK_SESSION_frustrum_t` field order** — real order is
   fstart, fabort, fstop, fmallo, fmfree, GO×6, ffoprd, ffopwr, ffclos,
   ffread, ffwrit, ffoprb, ffseek, fftell, FG×6, ucoprd, ucopwr.
   `ftmkey`/`ffskxt` are NOT members. (This was the session-start crash.)
2. **File-I/O frustrum callback signatures** — FFOPRD/FFOPWR/FFREAD/
   FFWRIT/FFCLOS all had invented signatures; replaced with the documented
   ones (guise+format+key in opens, stream ids allocated by the frustrum,
   FFOPWR receives `pr2hdr`, FFCLOS receives an action token).
   The default frustrum now writes/skips the standard `**END_OF_HEADER`
   file header and honours `FFABOR` (delete on abort).
3. **Frustrum tokens** — real FR_*/FFC*/FFBNRY/FFSKHD/FFNORM values from
   the Downward Interfaces appendix (di_chap.13).
4. **`PK_CLASS_*` and `PK_BODY_type_*` values** — the docs don't publish
   numbers, so they were probed from the DLL (see
   `crates/parasolid-test/src/bin/probe.rs`): e.g. entity=1000, geom=1001,
   topol=1002, curve=2002, surf=2003, point=2501, line=3001, circle=3002,
   plane=4001, cyl=4002, sphere=4004, torus=4005, vertex=5001, edge=5002,
   loop=5003, face=5004, shell=5005, body=5006, fin=5010, region=5011,
   part=5012; body types solid=5601, sheet=5602, minimum=5603, wire=5604.
   Constants in `parasolid-sys` are annotated [probed]/[family]/[guess] —
   do not rely on [guess]/[unknown] values.
5. **Analytic `_sf_t` struct layouts** — `basis_set` comes FIRST
   (CYL/CONE/SPHERE/TORUS/CIRCLE/ELLIPSE had radius-first layouts,
   which is why `PK_SPHERE_ask` returned radius 0).
6. **`PK_EDGE_ask_geometry`** — real signature has 7 args
   (`want_interval`, `class`, `ends[2]` were missing) — this caused the
   crash in `curve_eval`.
7. **Misc** — `PK_CURVE_eval_handed` arg order (n_derivs before hand),
   `PK_SURF_make_sheet_body` takes a by-value `PK_UVBOX_t`,
   `PK_BODY_set_type` takes an options pointer,
   `PK_BODY_create_solid_block` centres the *base* at the origin
   (z spans 0..z, not ±z/2).

## Known remaining risks

- Only the code paths exercised by `parasolid-test` are validated. Other
  bindings drafted the same way (booleans, blends, sweeps, fileio option
  structs, `PK_SESSION_register_fru_o_t`, `PK_MARK_frustrum_t`, error
  code values in `error.rs`) should be audited against the header mirror
  before use. Grep for `[guess]` / `[unknown]` markers.
- Schema files (FFCSCH) resolve to `<key>.sch_txt` relative to the
  configured base dir; receiving older-version XT files will need the
  SOLIDWORKS-shipped schema files made available under those keys.
