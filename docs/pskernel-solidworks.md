# Running parasolid-rs against the SOLIDWORKS pskernel.dll

Status (2026-07-14): **all 43 integration tests pass** against the
`pskernel.dll` shipped with SOLIDWORKS 2025 (Parasolid V37.01.243), with
`PK_SESSION_set_check_arguments(true)` enabled for every test.
`lib/pskernel.dll` in this repo is byte-identical (SHA-256) to
`C:\Program Files\SOLIDWORKS Corp\SOLIDWORKS\pskernel.dll`.

See `CLAUDE.md` for the Wine build/run workflow (including generating the
`lib/libpskernel.a` import library from the DLL export list).

## Build & run (Linux host, cross-compile + wine)

```bash
rustup target add x86_64-pc-windows-gnu   # plus a mingw-w64 toolchain
cargo build --workspace --target x86_64-pc-windows-gnu
# copy lib/pskernel.dll next to the exe (or set WINEPATH)
wine target/x86_64-pc-windows-gnu/debug/parasolid-test.exe
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

## Second pass (2026-07-14): mass props, cone, and the error path

Probed empirically under Wine and cross-checked against the `pskernel.dll`
decompilation in the `parasolid-re` Ghidra project (read-only). Findings are
marked `[probed]` / `[static-observed]` / `[dynamic-observed]` in the source.

8. **`PK_BODY_create_solid_cone` signature was wrong.** The draft modelled a
   non-existent frustum API `(top_radius, bottom_radius, height, …)`; every
   call failed with `PK_ERROR_general`. Real form is
   `(radius, height, semi_angle, basis_set, body)` — base radius + apex
   half-angle (radians), matching `PK_CONE_sf_t`. The base sits on z=0 and the
   solid **widens toward +z**: top radius = `radius + height*tan(semi_angle)`.
   Volume validated against the frustum formula.

9. **`PK_TOPOL_eval_mass_props` — full options struct recovered and validated.**
   The function takes an options pointer as its 4th argument (before the five
   output pointers; `documented` + `static-observed`). The tempting 8-arg
   no-options form makes the kernel read `amount` as the version field →
   `PK_ERROR_o_t_version_unknown` (5022). Accepted `o_t_version` is **1..=7**.
   The option-version-migration routine (`FUN_180441cd0`) shows the **version-1
   user struct** is just `{ o_t_version, mass, periphery, bound, single }` at
   offsets 0/4/8/12/16 — much smaller than the drafted struct, whose extra
   fields overran it and crashed. The enum tokens are **not** 0/1/2/3 but
   (`dynamic-observed`, each level adding one output):
   `PK_mass_no/mass/c_of_g/m_of_i = 0x36b1..0x36b4`,
   `PK_mass_periphery_no/yes = 0x36b5/0x36b6`, `bound_no = 0x36b7`. With the
   version-1 struct and these tokens, amount / mass / centre-of-gravity /
   inertia / periphery all match closed-form for block/sphere/cylinder/cone/
   torus, with `check_arguments` **on**. Wrapped as `Body::mass_props()` →
   `MassProps` (plus `Body::volume()` / `Body::mass()` conveniences).

10. **The error-inquiry path crashed on every PK error.** `PK_ERROR_sf_t`
    modelled `function` and `bad_arg_names` as `*const char`, but the kernel
    stores them as **inline char arrays** — the old code dereferenced the ASCII
    bytes ("PK_TOPOL…") as a pointer and page-faulted. `PK_THREAD_ask_last_error`
    also faults inside the kernel. `query_last_error` now reads a raw buffer via
    `PK_ERROR_ask_last` and extracts only confirmed fields: the inline function
    name (bytes 0..32) and code (i32 @32). Confirmed codes:
    `field_of_wrong_type`=5014, `o_t_version_unknown`=5022. The rest of the real
    `PK_ERROR_sf_t` (severity/n_bad_args/bad_args/entity, plus extra inline
    string fields) is still un-mapped.

11. **`PK_TOPOL_find_box` had an invented `options` argument.** Both the PK
    reference and the decompilation agree the real form is the 2-argument
    `(PK_TOPOL_t topol, PK_BOX_t *box)` with no options (the options form is the
    separate `PK_TOPOL_find_box_2`). Wrapped as `Body::bounding_box()` → `Aabb`;
    the solid block box is exactly `[-5,-10,0, 5,10,30]`.

## Intersection API surface — full coverage (2026-07-14)

Every `*intersect*` export in `pskernel.dll` (7 total) is now either wrapped in
safe code or explicitly accounted for. **All six geometric intersection
bindings were wrong** the same way — the prior agent used a truncated,
wrong-order output list (missing `bounds`/`types`, or the `topols`/`types`
trailing outputs on the curve variants), so the kernel wrote results through
uninitialised pointers. All are now fixed against the documented prototypes and
validated under Wine.

| Low-level export            | Safe API                    | Status | Validated by |
|-----------------------------|-----------------------------|--------|--------------|
| `PK_SURF_intersect_surf`    | `Surf::intersect`           | fixed  | cyl∩plane=circle, plane∩plane=line |
| `PK_FACE_intersect_surf`    | `Face::intersect_surf`      | fixed  | cyl face ∩ cap surf = circle |
| `PK_FACE_intersect_face`    | `Face::intersect_face`      | fixed  | adjacent block faces = line |
| `PK_CURVE_intersect_curve`  | `Curve::intersect_curve`    | fixed  | two block edges = shared vertex |
| `PK_SURF_intersect_curve`   | `Surf::intersect_curve`     | fixed  | vertical line ∩ z-plane = 1 pt |
| `PK_FACE_intersect_curve`   | `Face::intersect_curve`     | fixed  | vertical line ∩ z-face = 1 pt |
| `PK_BODY_intersect_bodies`  | via `Body::intersect` (bool)| n/a    | — |

`PK_BODY_intersect_bodies` is **not a geometric SSI** — it is the specialised
regularized-boolean intersection of solid/sheet bodies (returns a
`PK_boolean_r_t`, not intersection curves). The equivalent operation is reached
through `Body::intersect`, which uses the general `PK_BODY_boolean_2` path, so
the specialised entry point is intentionally left unwrapped.

Result shapes: the surf/face pair functions return points **and** curves
(`SurfIntersection`); the three curve variants return isolated point hits with
their parameters (`CurveCurveHit` / `SurfCurveHit` / `FaceCurveHit`, the last
also carrying the coincident face topology).

### `PK_intersect_*_t` kind tokens

Each result carries a kind code from one of three families. Their **base
transversal token** is confirmed (`dynamic-observed`); the other members
(tangential / coincident / …) are not yet decoded:

| Family (function) | transversal token | seen for |
|---|---|---|
| `PK_intersect_vector_t` (curve∩curve, surf∩curve) | **14611** (0x3913) | two lines crossing; line piercing a plane |
| `PK_intersect_curve_t` (surf∩surf, face∩face, face∩surf) | **14651** (0x393b) | plane∩plane=line, cyl∩plane=circle |
| `PK_intersect_fc_t` (face∩curve) | **14801** (0x39d1) | line piercing a planar face |

A Ghidra pass to enumerate the rest is a **dead end at reasonable depth**: the
values are not set in the public wrappers, their main worker (`FUN_1802d3cc0`),
or the immediate geometry callees (`FUN_1805b*`/`1805c*`) — they're computed
several layers into the shared intersection engine. The tractable route is
dynamic: build **tangential** and **coincident** fixtures and read the codes.
That needs orphan analytic surfaces (`PK_PLANE_create` &c.) so two surfaces can
be placed tangent/coincident — i.e. it is gated on the P1 *standalone geometry
creation* item, not on more decompilation.

## Known remaining risks

- Only the code paths exercised by `parasolid-test` are validated. Other
  bindings drafted the same way (booleans, blends, sweeps, fileio option
  structs, `PK_SESSION_register_fru_o_t`, `PK_MARK_frustrum_t`, error
  code values in `error.rs`) should be audited against the header mirror
  before use. Grep for `[guess]` / `[unknown]` markers.
- Schema files (FFCSCH) resolve to `<key>.sch_txt` relative to the
  configured base dir; receiving older-version XT files will need the
  SOLIDWORKS-shipped schema files made available under those keys.
