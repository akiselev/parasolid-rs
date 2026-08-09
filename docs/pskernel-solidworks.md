# Running parasolid-rs against the SOLIDWORKS pskernel.dll

Status (2026-07-14): **all 73 integration tests pass** against the
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
  bindings drafted the same way (blends, sweeps, facet/render option structs,
  `PK_SESSION_register_fru_o_t`, error code values in `error.rs`) should be
  audited against the header mirror before use. Grep for `[guess]`/`[unknown]`.
  (Booleans — previously blocked — are now corrected + validated, see below.)
- Schema files (FFCSCH) resolve to `<key>.sch_txt` relative to the
  configured base dir; receiving older-version XT files will need the
  SOLIDWORKS-shipped schema files made available under those keys.

### Body booleans (PK_BODY_boolean_2) — UNBLOCKED + validated

The core solid-modelling operation. Two ABI bugs kept it non-functional; both
fixed, validated by exact volumes (`boolean_unite/subtract/intersect_*`):

1. **`PK_BODY_boolean_o_t` was the wrong struct.** It modelled the internal
   192-byte v19 layout and set `o_t_version = 1`, which the kernel rejects with
   `PK_ERROR_o_t_version_incorrect` (5043) — accepted versions are `2..=19`.
   Decompiling the option-migration routine (`FUN_18049b860`) showed the **v2
   user struct is 32 bytes**: `{o_t_version=2, function@4, configuration@8 (ptr),
   default_tol@16 (f64), 3×u8 flags@24..26, fence@28}`. The kernel copies only
   those fields and defaults everything else internally (tolerances ≈1e-8,
   `check_fa=yes`, …); a NULL `configuration` is auto-filled, so no nested
   sub-struct is needed. `function` unite/subtract/intersect = 15903/15902/15901
   (auth), which the internal struct stores as 0x3e1f/0x3e1e/0x3e1d.
2. **`PK_boolean_r_t` field order was wrong.** Real layout is
   `{result@0, n_bodies@4, bodies@8}`; the old struct put `result` last, so
   `n_bodies` read the success token `PK_boolean_result_success_c` (21650) as the
   body count. The result bodies (which replace the target) come back in
   `bodies`; free the rest via `PK_boolean_r_f` (currently only `bodies` is freed).

Validated: block(20³) with a co-axial Ø6 cylinder → subtract = 8000−180π,
unite = 8000+180π, intersect = 180π (all within tolerance). Coincident faces
(cylinder base flush with the block base) are handled fine.

### Sweep / feature creation (PK_BODY_extrude) — corrected + validated

Feature creation by linear extrusion. Fixes:
- **`PK_BODY_extrude` signature** was 4 args and **missing the output `body`**
  (so it could never return the extruded solid). V35 is 6:
  `(profile, PK_VECTOR1_t path, options, PK_BODY_t *body, PK_TOPOL_track_r_t
  *tracking, PK_TOPOL_local_r_t *results)`. `path` is a `double[3]` typedef →
  decays to a pointer as a C parameter (bind as `*const PK_VECTOR1_t`).
- **`PK_BODY_extrude_o_t`** was missing `o_t_version@0` (all fields shifted).
  Its `start_bound`/`end_bound` are 32-byte `PK_BODY_extrude_bound_t` structs —
  the RE-catalog TSV wrongly assumed them 4-byte; the V35 doc + prior struct win.
- **Both output structs are written unconditionally** (`results.status` "is
  always set"), so NULL faults the kernel — pass a real `PK_TOPOL_track_r_t`
  (freed with `PK_TOPOL_track_r_f`) and a backing buffer for the opaque
  `PK_TOPOL_local_r_t`.
- `path` is a unit direction; the extrusion distance comes from the bounds
  (`PK_bound_distance_c`, start 0 → end `|path|`).

Wrapper: `Body::create_sheet_circle`/`create_sheet_rectangle` (profiles) +
`Body::extrude`. Validated: disk(r=5) extruded 10 → cylinder = 250π;
rect(8×6) extruded 5 → box = 240 (6 faces). Revolve (`make_spun_body`) is not
bound — a future item (needs the swept-body path).

### Blends / fillets — corrected + validated

Two-phase workflow (mark edges, then realise): `PK_EDGE_set_blend_constant` +
`PK_BODY_fix_blends`. Both signatures were wrong:
- **`PK_EDGE_set_blend_constant`** was 3 args (a single edge, no outputs); V35 is
  6: `(int n_edges, const PK_EDGE_t edges[], double radius, options,
  int *n_blend_edges, PK_EDGE_t **blend_edges)`.
- **`PK_BODY_fix_blends`** was 9 args in the wrong order (extra `n_topols`/
  `n_unders` counts, no `fault_edge`); V35 is 8:
  `(body, options, int *n_blends, PK_FACE_t **blends, PK_FACE_array_t **unders,
  int **topols, PK_blend_fault_t *fault, PK_EDGE_t *fault_edge)`. Added the
  `PK_FACE_array_t` type.
- `PK_EDGE_set_blend_constant_o_t` nests a large (~48-byte) `PK_blend_properties_t`
  (the RE-catalog TSV mis-sizes it as 4 bytes, so its offsets are unreliable) —
  the wrapper passes **NULL options** (defaults) for both calls, sidestepping the
  layout; that field order is still unvalidated.

Wrapper: `Body::fillet_edges(edges, radius)`. Validated: rounding one 90° edge of
a 20³ cube with r=3 → a solid with 7 faces and volume 8000 − (1−π/4)·3²·20
(exact — a clean quarter-cylinder fillet).

### Offset / hollow (shelling) — corrected + validated

- **`PK_BODY_offset`** `(body, offset, tolerance, face_face_check)` was already
  correct — offsets every face in place. Validated: 20³ block offset +1 → 22³
  (10648).
- **`PK_BODY_hollow_2`** was missing the `tracking`/`results` output args (4 → 6:
  `(body, offset, tolerance, options, PK_TOPOL_track_r_t *tracking,
  PK_TOPOL_local_r_t *results)`) — both written unconditionally, so NULL faults.
  A negative offset shells inward; NULL options give a closed shell (no pierced
  faces). Validated: 20³ block hollowed to wall 2 → a solid with an internal void,
  material volume 8000 − 16³ = 3904.

Wrapper: `Body::offset(distance)`, `Body::hollow(wall_thickness)`.

### Full topology graph (PK_BODY_ask_topology) — corrected + validated

`PK_BODY_ask_topology` bound only **4** of its **9** args (stopped at `topols`),
so the kernel wrote `classes`/`n_relations`/`parents`/`children`/`senses` through
un-passed arguments — latent stack corruption. Now 9 args:
`(body, options, int *n_topols, PK_TOPOL_t **topols, PK_CLASS_t **classes,
int *n_relations, int **parents, int **children, PK_TOPOL_sense_t **senses)`
(`parents`/`children` are indices into `topols`). Wrapper `Body::ask_topology`
returns every topological entity + the relation count. Validated: a block's graph
contains the body and all 6 faces / 12 edges / 8 vertices, ≥34 topols, >0 relations.

### Section (PK_BODY_section_with_surf) — corrected + validated (splits a solid)

Corrected the signature (7 args → V35's **4**: `(target, surface,
const PK_BODY_section_o_t *options, PK_section_r_t *results)`). The options must be
non-NULL (NULL faults on read); its layout was recovered from the RE catalog —
64 bytes: `{o_t_version@0, fence@4, matched_region*@8, merge_imprinted@16,
merge_new_faces@20, selective_merge@24, check_fa@28, default_tol@32, max_tol@40,
tracking@48, keep_target_edges@52, keep_as_facet@56}`; `o_t_version = 1` is
accepted. The result struct (`PK_section_r_t`, 64 bytes, filled by internal
`FUN_180b997a0`) is **not read** — with `fence = both` the section *splits* the
body: the resulting bodies appear in the session and the target tag becomes one
piece. Validated: a 20³ block sectioned by the z=10 plane → **two** session
bodies, the original tag now a 6-faced 20×20×10 box of volume 4000.
Wrapper: `Body::section_with_surf(surf)`.

### Topology-query signature sweep — corrected + validated

More clean arg-count bugs (like `ask_topology`) from the audit — functions that
dropped an `options` or scalar arg, so the kernel wrote outputs through un-passed
arguments:
- `PK_EDGE_ask_convexity` — was 2 args; V35 3 (added `options`).
- `PK_EDGE_is_smooth` — was 2; V35 3 (added the `double max_angle` threshold).
- `PK_FACE_ask_faces_adjacent` — was a single-face 3-arg form; V35 5 (an ARRAY of
  faces + `options` + `n_faces_adjacent`/`faces_adjacent`).

Wrappers `Edge::convexity`/`is_smooth`, `Face::adjacent_faces`. Validated on a
block: every edge is convex (`23597`) and sharp (not smooth), and each of the 6
faces borders exactly 4 others.

### Scaled signature reconciliation — 390 → 80 arg-count mismatches

Toward covering **every** `parasolid-sys` function, the arg-count mismatches vs
`catalog/pk-clean-prototypes.tsv` were reconciled by codegen (scripts in the
session scratchpad: `siggen.py` regenerates decls with a conservative C→Rust
mapper; `stubgen.py` defines opaque/scalar stubs for pointer-only undefined
types, in `parasolid-sys/src/generated_stubs.rs`). **310 signatures corrected**
across two batches (130 + 163 + earlier manual), all compile-checked and
**regression-tested (the 62 runtime tests hold)**. Safety rules the codegen
follows:
- **Never touch wrapper-USED functions** (6 remain mismatched — they are
  validated by passing tests, and the TSV is fallible, e.g. `ask_kernel_version`).
- Skip anything with **by-value aggregate** params (`PK_VECTOR_t`/`PK_INTERVAL_t`/
  `PK_UVBOX_t` — Win64 struct-by-value ABI is subtle for the `[f64;N]` aliases);
  74 such functions remain and need per-function handling.

These 310 are **signature-reconciled to the vendor prototypes, not
runtime-validated** (they're otherwise-unused bindings). Undefined option/result
structs are opaque stubs — correct as pointer args, but a real layout is needed
before the owning function can be *called* with non-default options.

## RE-ABI correction pass (2026-07-14, from `parasolid-re`)

Correcting `parasolid-sys` against the reverse-engineered authoritative ABI in
`/home/dev/projects/parasolid-re/catalog` (see `re.md`). All items below keep
the suite at **44 passed, 0 failed**.

### Enum constants — DONE, fully reconciled

Every `PK_*_c` token now matches `catalog/pk-enums.h` (V35 token appendix,
cross-validated vs PKToy + 16 binary bands). Scripted, then verified diff → 0:

- **1250 wrong values** replaced (the placeholders were sequential `0,1,2…`;
  real tokens are large, e.g. `PK_mass_m_of_i_c` 3 → 14004,
  `PK_ATTRIB_field_real_c` 0 → 5902, `PK_partition_type_standard_c` 0 →
  `PK_PARTITION_type_standard_c` 23510).
- **22 invented members removed** — names that exist in *no* real enum
  (`PK_LOOP_type_general_c` collided with `wire_c` 5411; `SHELL_type_face/wire`,
  `VERTEX_type_standard/tolerant`, `knot_type_bspline…` were placeholders for
  real families `SHELL acorn/wireframe…`, `VERTEX isolated/spur/wire/normal`,
  `knot unset/non_uniform/uniform…`).
- **376 missing members appended** — enums were badly incomplete
  (`PK_check_state_t` was missing 75 fault codes, `PK_local_status_t` 38,
  `PK_transmit_format_t` 6, the `*_update_t` version families, …).
- **1 type bug**: `PK_scale_factor_t` was `c_double`; it is `int` (a discrete
  1x/10x/100x/1000x token enum) — fixed both `PK_BODY_enlarge`/`PK_GEOM_enlarge`.
- `PK_check_returns_latest_c` is a distinct sentinel (23660), not an alias of
  `_4_c` (23665).

Re-verify: `catalog/pk-enums.h` vs the const blocks must diff to 0 bogus / 0
missing for every auth-known type.

### Option structs — active bugs fixed + validated; latent inventory recorded

The wrapper only *constructs* 14 `_o_t` structs; those are the ones that can
corrupt a live call. Fixed and re-validated dynamically:

- **Truncated intersect structs** (memory-unsafe: kernel reads the full v1
  layout, wrapper allocated less → read past the stack allocation; tests passed
  only because the trailing stack bytes happened to be zero):
  `PK_SURF_intersect_surf_o_t` (56 → 192 B), `PK_FACE_intersect_face_o_t` and
  `PK_FACE_intersect_surf_o_t` (128 → 192 B), `PK_SURF_intersect_curve_o_t`
  (56 → 64 B). Completed to the documented layout, zero-initialised (all
  `have_*` flags false). *Confirmed*: SSI suite still green — so these use the
  full v1 layout, **unlike** `PK_TOPOL_eval_mass_props_o_t` which genuinely uses
  a minimal-v1 form (a larger struct there crashes; see below).
  `PK_CURVE_intersect_curve_o_t` was already correct (the `r#box` raw-ident
  field made a naive audit misread it).
- **`PK_PARTITION_delete_o_t`** was missing `o_t_version@0` (kernel read
  `delete_non_empty` as the version). Added; wrapper sets `o_t_version = 1`.

Latent inventory (audit of all 186 field-bearing structs vs
`catalog/pk-option-structs.tsv`), **all in bindings the wrapper does not yet
construct**, so not active bugs — fix per-struct WITH a dynamic fixture when the
owning function is bound:
- **61 structs omit `o_t_version`** at offset 0 (emboss, extrude, most
  `FACE_change_*`, mesh/facet/render families, …).
- ~57 more have field order/type divergences (some are genuine, some are TSV
  `assumed_types` noise — the TSV is *not* authoritative for types).

### Function signatures — audited, NOT bulk-fixed (TSV is fallible)

`catalog/pk-clean-prototypes.tsv` arg-count vs the `extern` decls: **390 / 1037
common functions mismatch** (confirms CLAUDE.md's "most signatures unvalidated"
— e.g. `PK_BODY_ask_topology` binds 4 of 9 args, `PK_BODY_check` 1 of 4). But
the TSV itself is wrong for some: `PK_SESSION_ask_kernel_version` binds 3 args
and is dynamically confirmed (prints v37.1), while the TSV says 1. Only **12**
mismatched functions are wrapper/test-called; the exercised ones pass. **Do not
bulk-rewrite signatures from the TSV** — correct each against the V35 doc mirror
+ a dynamic call. Scripts for all three audits live in the session scratchpad.

### Partition / PMARK subsystem — corrected + partitioned rollback implemented

Reconciled the partition/pmark bindings against the V35 doc mirror and brought
up **partitioned rollback** end-to-end (3 new passing tests: 47 total).

Signature fixes (were wrong; now match the V35 docs, validated dynamically):
- `PK_PARTITION_create` — was 1 arg; is `(options, results)` (added
  `PK_PARTITION_create_o_t`/`_r_t`).
- `PK_PARTITION_delete` — was a 3-arg array form; is `(partition, options)`
  (single partition).
- `PK_PARTITION_advance_pmark` — was 1 arg; is `(partition, options, *pmark)`.
- `PK_PMARK_goto` — was 1 arg; is 7 args `(pmark, *n_new, **new, *n_mod, **mod,
  *n_del, **del)`. `del_entities` is `int**` (dead-entity tags), new/mod are
  `PK_ENTITY_t**`. **The counts are written unconditionally — passing NULL for
  them faults the kernel** (the "optional" doc note is misleading), so even a
  discard-results `goto` must pass real pointers.
- `PK_PMARK_goto_2` — was a 3-arg `_r_t`-struct form; is 8 args with individual
  out-params.

Delta frustrum (in-memory) — `crates/parasolid/src/rollback.rs`, enabled by
`SessionConfig::rollback(true)` (registers via `PK_DELTA_register_callbacks`
*before* `PK_SESSION_start`). The six callback ABIs were **wrong in the drafted
sys typedefs** and were recovered by decompiling the kernel-side callers
(`PKF_delta_*`):
- Callbacks **return** the ifail code (0 = success); there is NO `*ifail`
  out-param, and `read` has no `actual_bytes` out-param.
- `open_for_write(pmark, *strid)` returns a stream id; `open_for_read(pmark)`
  takes ONLY the pmark (write streams key by the returned strid, read streams key
  by the pmark). Our store makes `strid == pmark` so everything keys uniformly.

Parasolid usage constraints found (not bugs — restrictions):
- `PK_PARTITION_create`/`make_pmark`/`ask_curr_partition`/`PK_ENTITY_ask_partition`
  need partitioned rollback active, else `PK_ERROR_rollback_not_started` /
  `5048`. `PK_SESSION_ask_partitions` works without it.
- A partition can't be made current while at its initial pmark
  (`PK_ERROR_cannot_make_current`); a pmark can't be made at the initial pmark.
- Multi-pmark navigation and roll-forward hit `PK_ERROR` 5003 / a redo-path
  issue under this minimal frustrum — left as future work; single-checkpoint
  backward rollback is fully validated (`partition_rollback_goto`,
  `partition_rollback_tracks_topology`).

### Session marks (PK_MARK_*) — corrected + validated on partitioned rollback

Session marks checkpoint every partition at once and **ride on either rollback
system**: `PK_MARK_create`/`PK_MARK_goto` work whenever *partitioned* rollback is
active (my `SessionConfig::rollback`), so no separate `PK_MARK_start` frustrum is
needed (indeed `PK_MARK_start` "starts non-partitioned PK rollback" and is
*mutually exclusive* with partitioned rollback). `PK_MARK_goto` is genuinely
1-arg and `PK_MARK_goto_2` genuinely uses a results struct — unlike their PMARK
counterparts.

Signature fixes (vs V35 docs):
- `PK_SESSION_ask_mark` — was 1 arg; is `(*mark, *at_mark)` (the old form made
  the kernel write `at_mark` through an unpassed argument).
- `PK_MARK_ask_state` — first arg is an OUT `*current`, not `mark` by value.
- `PK_MARK_ask_forward` — is `(*is_enabled)`, 1 arg (was 2).
- `PK_MARK_ask_frustrum` — is `(*frustrum)`, 1 arg (was 2).
- `PK_MARK_start` — was 0 args; is `(PK_MARK_frustrum_t frustrum,
  const PK_MARK_start_o_t *options)` (added `PK_MARK_start_o_t = {forward}`).

Validated: `session_mark_rollback` (create mark → block → `goto` removes it),
`session_mark_current` (the corrected 2-arg `PK_SESSION_ask_mark`). The
`ask_state`/`ask_forward`/`start` fixes are compile-checked but only reachable
under non-partitioned rollback, so not dynamically exercised here.

### fileio / XT transmit-receive — corrected + validated (toward the CAD dataset)

`PK_PART_transmit`/`PK_PART_receive` **signatures** were already correct; the
**option structs were wrong** and are now fixed against the RE catalog (validated
by a full XT round-trip, `xt_roundtrip`):
- `PK_PART_transmit_o_t` — was 24 B with `transmit_format` at @8 and missing
  `transmit_user_fields`/`transmit_nmnl_geometry`; now the documented 40-B layout
  (`version@0, format@4, user_fields@8, transmit_version@12, nmnl_geometry@16,
  indexed_context@24, meshes@32`).
- `PK_PART_receive_o_t` — was missing `n_part_indices`/`part_indices`/
  `receive_compound`/`receive_mixed` with `transmit_format` at @12; now the 72-B
  layout with `transmit_format@4` (added `PK_receive_compound_t`).
- **`transmit_format` must be set explicitly** — `0` is not a valid token (the
  enum starts at 18220), so the wrapper sets `PK_transmit_format_text_c`; receive
  likewise needs the format set (auto-detect `0` failed here). The frustrum's
  disk file callbacks (`FFOPWR`/`FFOPRD`/`FFREAD`/`FFWRIT`/`FFCLOS`) already work;
  files land in `FrustrumConfig::base_dir`.

**XT (Parasolid Transmit) text format**, from a transmitted `.xmt_txt`:
```
**ABC…abc…**…                 (charset validation lines, 2)
**PART1;FRU=…;APPL=…;          (header: frustrum + application id)
**PART2;SCH=SCH_3701243_37102;USFLD_SIZE=0;;   (SCHEMA name + user-field size)
**PART3;
**END_OF_HEADER***…
T51 : TRANSMIT FILE created by modeller version 370124323 SCH_3701243_37102_1300
<node records: `<typechars><tag> <fields…>`, incl. a `schema_embedding_map`>
```
`SCH_3701243_37102` encodes the kernel version (V37.01.243). The schema is
**embedded** (a `schema_embedding_map` node) — a same-version round-trip needs no
external `.sch_txt`.

**To load real dataset XT files** (e.g. the ABC/Onshape corpus) three things
matter: (1) set `transmit_format` to the file's actual format (text vs binary);
(2) the frustrum maps a *key* to `base_dir/<key><ext>` where `ext` is `.xmt_txt`/
`.xmt_bin` — real files use `.x_t`/`.xt_txt`/`.x_b`, so `guise_extension`/
`resolve_key` in `frustrum.rs` must be taught those extensions (or the files
renamed); (3) cross-version files that reference an external schema need that
`SCH_*.sch_txt` served via `FFOPRD`(`FFCSCH`) from `base_dir`. The receive
machinery itself is proven end-to-end.

## Signature correctness sweep completed + new features (2026-07-14)

### All arg-count and pointer-ness mismatches resolved (0 remaining)

Two audits now come back clean against `parasolid-re/catalog/pk-clean-prototypes.tsv`:

- **Arg-count mismatches: 12 → 0.** Fixed the last exotic ones
  (`PK_SESSION_set_behaviour` 5-arg, `set_err_reports` 2-arg, `set_smp`,
  `watch_tags`, `THREAD_lock_partitions`, the three `DEBUG_SESSION_watch_*`,
  `BSURF_create_constrained`, `GEOM_enlarge`, `PARTITION_receive_b/_version_b`,
  `PART_receive_b`).
- **Pointer-ness mismatches: 26 dangerous → 0 real.** A per-parameter audit
  (`param_audit2.py`) flags only *pointer-vs-≤8-byte-value* disagreements — a
  by-value aggregate >8 B is passed by hidden pointer on Win64, so binding it as
  `*const T` is ABI-equivalent and NOT a bug. 19 old hand-guessed signatures
  were regenerated wholesale from the TSV (`regen_named.py`): e.g.
  `PK_TOPOL_facet`/`render_facet`, `PK_GEOM_transform_2`, `PK_BODY_pick_topols`,
  `PK_FACE_euler_unslit` (was `*mut PK_EDGE_t`, kernel wants a tag by value). The
  5 still flagged are false positives (`PK_POINTER_t == *mut c_void`; the
  `PK_*_frustrum_t` callback structs are the deliberate `*const callbacks` form).

### Real wrapper-used crash bugs fixed + runtime-validated

- `PK_ENTITY_delete(int n, const PK_ENTITY_t[])` — the old binding passed the tag
  as the *count*. Test `entity_delete`.
- `PK_SESSION_ask_kernel_version` / `ask_smp` — wrote a multi-int struct through a
  single `*int` (out-of-bounds; "worked" by luck). Gave them real
  `PK_SESSION_kernel_version_t` / `PK_SESSION_smp_r_t` structs.
- `PK_SESSION_set_behaviour` — 5 args; the 8-byte `behaviour_requested` struct is
  passed **by value** (fits a register), and the function writes all three
  returned args **unconditionally**, so NULL out-params fault — pass real
  buffers (same pattern as `PK_PMARK_goto`/`extrude`/`hollow`). Test
  `session_behaviour_err_reports`.
- `PK_SESSION_set_err_reports(PK_ERROR_reports_t, *opts)` — enum tokens
  on/off/inherit = 26820/1/2.
- `PK_SESSION_set_smp(PK_SESSION_smp_o_t *)` — the old `(int n_threads)` binding
  passed the count where the kernel dereferences a pointer. Real 16-byte options
  `{o_t_version, thread_format:PK_thread_t, n_threads, on_single_processor}`;
  `thread_format = 0` is invalid (tokens disabled/per_processor/absolute =
  21010/1/2), so an explicit count needs `PK_thread_absolute_c`. Test
  `session_set_smp`.

### New validated high-level features

- **Transform / copy** (`transform.rs`). Fixed `PK_TRANSF_sf_t`: it is a full
  **4×4 = 16 doubles**, not the 13-element "compressed" form the old binding had
  (confirmed against the authoritative C# binding in
  `ch122-how-the-c-binding-is-implemented.md`). `Transform::translation` /
  `uniform_scale` / `from_matrix` + `Body::transform`. Tests
  `transform_translation_moves_cog` (CoG shifts by exactly the vector, volume
  preserved), `transform_uniform_scale_volume` (×2 → vol ×8),
  `transform_matrix_roundtrip`, `body_copy_independent`.
- **Attributes** (`attrib.rs`). `AttribDef::find` + `Face::set_colour`/`colour`
  attach and read the system `SDL/TYSA_COLOUR` attribute (field 0 = 3 RGB
  doubles). Test `face_colour_attribute`.
- **Body validity check** (`check.rs`). `Body::check`/`is_valid` wrap
  `PK_BODY_check` (NULL options = default comprehensive checks) — the core
  validity oracle for bodies loaded from external datasets. Test
  `body_check_valid` (primitives + a boolean result are all fault-free).
- **Imprint** (`Face::imprint_curve`). `PK_FACE_imprint_curve` imprints a curve
  onto a face over a parameter interval, splitting it. Test
  `imprint_circle_splits_face` (circle on a block's top face → 6→7 faces, volume
  preserved, body still valid).

### Faceting (`PK_TOPOL_facet_2`) — WORKING + validated (topology totals)

The two option sub-structs were **wrong** and had never been runtime-exercised,
so the bugs survived: `PK_TOPOL_facet_mesh_2_o_t` was missing the leading
`o_t_version` (shifting every field by 4 B) and diverged in field order;
`PK_TOPOL_facet_choice_2_o_t` was missing `o_t_version` **and all 23
table-selection flags**. Both were rebuilt to the authoritative RE-catalog field
order (`pk-option-structs.tsv`) and are locked by compile-time `offset_of!`
assertions in `facet.rs` (mesh = 384 B, choice = 144 B on x64; the catalog's
absolute offsets model 32-bit pointers for the choice struct's two pointer
fields, so field *order* is trusted and `#[repr(C)]` computes the x64 offsets).

`Body::facet` now tessellates: a solid block returns **12 facets / 36 fins** with
`check_arguments` on (test `facet_block_triangles`). Getting there required
recovering the option handshake empirically under Wine:

- **Error 5022 = `PK_ERROR_o_t_version_unknown`** (the same code the mass-props
  war story confirmed). The kernel accepts facet option **`o_t_version` = 5**:
  1..4 → 5022, 6+ → 5014 (`field_of_wrong_type`).
- The **top-level `PK_TOPOL_facet_2_o_t` is inline** `{control, choice}` (a
  pointer-based top-level returns 5022 — the kernel reads `o_t_version` from what
  would be the pointer's bytes). This matches the reference-manual dot-access.
- At version 5 the **control layout matches the catalog only through
  `wire_edges`**; the `incremental_*` and later fields must be left **zero**
  (setting them to their v-latest enum tokens returns **908**, "bad option
  data"). `max_facet_sides` must be ≥ 3 (0 → error 33). Documented defaults from
  ch105: shape=convex 20502, match=topol 20522, density=no_view 20540,
  cull=none 20560, ignore=no 22111, ignore_scope=global 22131,
  wire_edges=no 22140.

**Remaining gap — tabular vertex tables.** The `choice` table-selection flags
(`point_vec`, `data_point_idx`, …) do not take effect at option version 5
(`n_tables = 0` even with every flag set, and sweeping the inline `choice`
offset finds no position that populates a table) — the rich tabular output
belongs to a **newer option version** whose full struct layout the current
catalog does not perfectly capture (version 6+ trips 5014). So `Mesh.vertices`
is currently empty while `n_facets` / `n_strips` / `n_fins` are validated.
**Next step:** obtain the exact V37 `PK_TOPOL_facet_mesh_2_o_t` /
`PK_TOPOL_facet_choice_2_o_t` layouts (headers or a decompile of
`PK_TOPOL_facet_2_o_m`) to enable the point/normal tables.

## High-level API coverage sweep + by-value aggregate ABI (2026-07-15)

Drove the wrapper test suite from **73 → 91** passing (0 failed) by covering
every remaining public wrapper method, and fixed the last known-skipped `-sys`
category (by-value aggregates). Method as before: audit signature vs
`parasolid-re` catalog → runtime-assert concrete output under Wine → fix.

**By-value aggregate ABI — PROVEN sound on `x86_64-pc-windows-gnu`.** The earlier
reconciliation *skipped* functions passing `PK_VECTOR_t`/`PK_INTERVAL_t`/
`PK_UVBOX_t` by value ("Win64 struct-by-value subtle"). Resolved empirically with
three runtime probes, all passing:
- `PK_CURVE_find_length` (`PK_INTERVAL_t` = 16-byte `{low,high}` by value) →
  circle length over `[0,2π]` = `2πr`, line length over `[0,7]` = 7.
- `PK_EDGE_contains_vector` (`PK_VECTOR_t` = 24-byte `[f64;3]` by value) →
  edge midpoint contained, distant point not (`Edge::contains_point`).
- `PK_SURF_make_sheet_body` (`PK_UVBOX_t` = 32-byte `[f64;4]` by value) → plane
  bounded `[0,10]×[0,20]` yields a sheet body of area 200 (`Surf::make_sheet_body`).

Conclusion: a **by-value >8-byte aggregate is ABI-equivalent to `*const T`** on
Win64 (both pass a pointer indirectly). The `improper_ctypes` warning on
`[f64;N]`-by-value is therefore **spurious** for these; no pointer conversion is
needed. The 16 by-value-aggregate fns whose signatures *match* the vendor
prototype are correct as-is.

**`-sys` signature fixes (8).** Only functions whose signature actually diverged
from the vendor `pk-clean-prototypes.tsv`/`pk-reference.tsv`:
- `PK_CURVE_find_box`, `PK_SURF_find_box`: arg2 was a by-value `interval`/`uvbox`;
  V35 form takes an **`options` pointer** (bounds live in the options struct).
- `PK_SURF_find_degens`, `PK_SURF_find_self_int`: arg2 → `options*`, arg3 → a
  **result struct** (`PK_SURF_degens_t*` / `PK_SURF_self_ints_t*`), not `n_*:int*`.
- `PK_CURVE_find_vectors`: arg3 was `n_vectors:int`; V35 = `tolerance:double`,
  and arg5 is a `PK_CURVE_find_vectors_r_t*` result struct.
- `PK_CURVE_make_helical_surf`, `PK_BODY_make_swept_tool`: `PK_AXIS1_sf_t` axis
  passed **by pointer** (the vendor's convention for `_sf_t` structs; small
  aggregates `VECTOR`/`UVBOX`/`INTERVAL` stay by value).
- `PK_BODY_ask_memory_usage`: the returned `size_t *const total` output was
  **dropped by `clean-prototypes.tsv`** but present in `pk-reference.tsv`; added
  `total: *mut usize`. (Lesson: `clean-prototypes.tsv` occasionally drops a
  returned arg — cross-check the richer `pk-reference.tsv` for query fns whose
  only arg is the entity.)

**Verification.** Full arg-count diff of all 1154 `-sys` externs vs
`clean-prototypes.tsv` → **0 real mismatches**. Per-argument pointerness diff →
27 residual, all either (a) the ABI-equivalent by-value-aggregate/frustrum-
callback `*const` forms (validated, incl. `PK_DELTA_register_callbacks`), or
(b) **shape divergences** where `-sys` uses the classic `(n_out:*mut int,
out:*mut *mut T)` count+array output convention while the vendor shows a `_r_t`
result struct (`PK_BODY_unite/subtract/intersect_bodies`, `PK_BODY_make_section*`,
`PK_CURVE_project`, `PK_ENTITY_find_reparam`, `PK_PARTITION_ask_pmarks_2`,
`PK_EDGE_set_blend_variable`, `PK_*_make_sect_with_sfs`, `PK_MESH_find_laminar_mfins`,
`PK_ATTDEF_register_cb`, …). All (b) are **unused** bindings; per the standing
methodology ("don't trust the fallible TSV blindly — correct against the V35 doc
+ a dynamic call") they are left as-is pending a real consumer, not bulk-rewritten.

**New wrapper methods (validated):** `Curve::length`, `Edge::contains_point`,
`Surf::make_sheet_body`; and a wrapper bug fix — `Session::behaviour()` used to
**error on a fresh session** because the default `PK_SESSION_behave_as_unset_c`
(25840) was unmodelled; added `Behaviour::Unset`.

**New coverage (18 tests):** entity class/predicates, shell/face outward-normal
orientation, mass shortcuts, fin next/prev inverse + `loop_`/`edge`, session
precision/schema/memory/tags/flags/behaviour/user-field/journal getters, partition
pmark navigation + bodies/geoms, matrix-rotation transform, multi-tool boolean via
the low-level `boolean()`, and the three by-value ABI probes above.

**Documented limitation:** `PK_PARTITION_set_current` / `_delete` on a *second*
partition return mild **error 10** under the minimal in-memory delta frustrum
(partition switching needs persistent delta storage) — a test-harness limit, not
an ABI bug. The original partition's full surface is validated.

## Journal-driven struct audit + mesh callback ABI (2026-07-15)

### The `PKU_journal_<TYPE>_<sf|o>` decompile method (reusable)

Every marshalled PK struct has a decompilable `PKU_journal_*` function that names
**every field and its byte offset**: `param_1` is an `undefined4*`, so `param_1[N]`
is a 4-byte field at byte `N*4` (journalled by-value → `c_int`/enum/tag/`PK_LOGICAL`),
`*(longlong*)(param_1+N)` is an 8-byte pointer at byte `N*4` (an array base, preceded
by its `n_*` count), and `FUN_180a74d60(*param_1)` + `PKU_journal_begin_o_t()` mark an
`o_t_version: c_int` at offset 0. The DLL has **435 `PKU_journal_*` symbols**; the
struct-level ones (`_sf`/`_o`) were batch-decompiled by address (persistent ghidra
bridge, ~fast) and diffed against the Rust structs.

**Cross-cutting finding:** V37 option structs pack `PK_LOGICAL` flags and
small-cardinality enums as **1-byte fields** (the journals read them via
`*(undefined1*)` at consecutive byte offsets), not `c_int` — matching the already
runtime-validated `PK_BODY_boolean_o_t.flags_v2: [u8;3]`. Large-token enums
(18xxx–26xxx) stay `c_int`.

**~20 structs corrected** against their journals (missing `o_t_version`, wrong
offsets/order, `c_int`→`u8` flag packing, count/pointer pairing): `boolean.rs`
(`boolean_match_o`, `FACE_boolean_o`, `BODY_make_section_o`, `BODY_section_o`,
`FACE_section_o`), `editing.rs` (`BODY_create_implicit_o`, `is_disjoint`,
`make_patterned`, `slice`, `FACE_replace_surfs`, `REGION_embed_body`), `lattice.rs`
(clip, create_by_graph, find_nabox, make_bodies, make_patterned), `fileio.rs`
(`PARTITION_transmit/receive_o`), `offset.rs` (`BODY_thicken_o` reorder), `enquiry.rs`
(`ENTITY_range_o`/`_vector_o` fleshed out from opaque), `error.rs` (`PK_ERROR_sf_t`
rewritten to the real inline-`char[32]` form, not the manual's pointer/bad-args guess).
`BODY_boolean_o_t` and PART transmit/receive were **verified-correct** (validated
minimal versioned structs; the journal dumps the larger current-version layout —
expected version drift). Two real OOB bugs fixed in `bgeom.rs`: `PK_BCURVE_sf_t` was
missing `self_intersecting` @56 (56 B vs real 64 B → `PK_BCURVE_ask` wrote past the
struct) and `PK_BSURF_sf_t`'s `_extra_96: usize` mislabeled `self_intersecting` @92 +
`convexity` @96. No test regressions (117 passed).

### `PK_MESH_create_from_facets` callback ABI — fully reverse-engineered

The callback contract is **not** what the docs' pseudo-code implies. Confirmed
against `FUN_1813f30a0` (the shared internal facet consumer) on V37.01.243:

1. **Convergent Modeling is disabled by default.** `PK_MESH_create_from_facets`
   returns **5237** unless `PK_SESSION_set_facet_geometry(PK_facet_geometry_all_c)`
   is called first (the kernel gates on `DS_roll_data()+0x48`). See ch082 §82.2.1.
2. **The reader takes 3 args**, `(context, descriptor*, status*)` — not 4. A 4-arg
   signature crashes writing a NULL R9. The **return value is ignored**; the
   continue/stop token is written through the `status` **out-param** (arg3). The
   docs' "returns the status" is literal about the out-param, not the C return.
3. The `descriptor` (arg2) is `{facet_type: c_int @0, pad @4, facet_data: *mut @8}`.
   `facet_type` is the **internal dispatch code** (6 = vector), not the public
   `PK_MESH_facet_type_vector_c` (100755) token.
4. The vector block's first field is the **facet count** (`n_facets`), not
   `n_vertex_positions` — the consumer loops `0..n_facets` reading 3 vertices each at
   stride `0x18`. Passing the vertex count (3×) over-reads → deferred error.
5. `status = stop` means "**this is the last block**" (still processed), not "no
   data": the single block is delivered on the one reader call *with* `stop`.

**Residual blocker (parked, honest SKIP):** with the ABI correct, the reader runs
and delivers the block, but the convergent construction engine rejects the tetra
facet set with mild **`PSM_mesh_create_result` 4/9 → PK 5241** and returns a null
mesh tag (`PKU_interpret_mesh_load_error` maps the internal result). Independent of
winding. `mesh_from_triangles` reports **SKIP** on 5241 (not FAIL) so the blocker
stays visible; it will pass automatically if construction is later unblocked.

## Spine completion + oracle facade (2026-07-15)

Suite **117 → 131 passing** (1 skip). New runtime-validated wrapper surface,
plus an undocumented-function finding and several honest deferrals.

### P0 B-rep spine completion (no option structs)

- `Shell::sign` (`PK_SHELL_find_sign`) — tokens positive=3550/negative=3551/
  **open=3552** (3552 was missing from `-sys`, added). A solid block's bounding
  shells report a closed (positive/negative) sign.
- `Region::region_type`/`make_solid`/`make_void`. `region_type` is derived from
  `is_solid` — the raw `PK_REGION_ask_type` takes an **undocumented options
  struct** (a bare 2-arg call yields 5022 = o_t_version_unknown), so it is not
  used. `make_void`/`make_solid` flip the material flag (validated round-trip).
- `Fin::geometry` (`PK_FIN_ask_geometry`) — returns the fin's underlying **3D
  curve** (a different tag from `PK_FIN_ask_curve`, which is the SP-curve) over a
  non-degenerate interval. Note `PK_FIN_find_interval` returns error 96 on block
  face fins (the existing `Fin::interval` wrapper is latently broken; use
  `geometry`).
- `Face::surface_type` — delegates to the backing surface's class. **`PK_FACE_ask_type`
  is NOT the simple surface-class query the docs imply:** decompiled, it is
  `(face, options*, result*)` with a versioned option struct and a token-filled
  result struct (writes 0x6a10/0x1648/0x6a19). Skipped in favour of the surface
  class, which is the same classification.
- `Face::extreme` (`PK_FACE_find_extreme`) and `Face::is_coincident`
  (`PK_FACE_is_coincident`, NULL options + the direct `tol` arg → `Coincidence`
  enum over `PK_FACE_coi_t` 21880-21887). Validated: two stacked blocks' shared
  faces are coincident; offset parallel faces are not.
- `Edge::find_interval`/`set_precision`/`reset_precision` (`PK_reset_prec_t`
  binary-confirmed ok=17201..failure=17204) and `Edge::make_wire_body`
  (`PK_EDGE_make_wire_body` — a block edge → a 1-edge/2-vertex wire body).
- `Body::create_minimum` (`PK_POINT_make_minimum_body`) → a single **acorn**
  vertex whose lone shell's `acorn_vertex` round-trips.

### P0 Entity/Topol

- `Entity::description` (`PK_ENTITY_ask_description`, NULL opts). The DLL does
  **not export `PK_ENTITY_ask_description_r_f`** — the returned string is a plain
  PK allocation, freed with `PK_MEMORY_free`.
- `Entity::delete_redundant` (`PK_TOPOL_delete_redundant`) — removes an
  imprint-split vertex (validated via a mid-edge imprint then cleanup).

### Native Transform algebra (pure math)

`Transform::rotation`/`reflection`/`scale_about` (`PK_TRANSF_create_rotation`/
`_reflection`/`_equal_scale`), `then` composition (`PK_TRANSF_transform`,
self-then-other order confirmed), `is_equal`, and `apply`/`apply_direction`
(`PK_VECTOR_transform`/`_transform_direction`). All match closed form (rotate
+x→+y about z, reflect across x=0, ×2 scale, rot∘translate).

### Body::disjoin + oracle facade

- `Body::disjoin` (`PK_BODY_disjoin`) — a connected solid disjoins to its single
  component (topology/volume preserved). The multi-lump split path needs a
  boolean with `allow_disjoint`, which the minimal boolean wrapper does not
  expose.
- **`parasolid::oracle`** — the stable, validated-only comparison surface for
  CADabra's testkit: primitive builders, `sample_surface` (position + unit
  outward normal), `sample_curve` (position + **unit** tangent, normalising the
  raw `PK_CURVE_eval` derivative — for a radius-r circle the raw derivative has
  length r), `intersect_surfaces` (orphan surfaces only — a face's owned surface
  gives 916), and `Body::topology_summary` → `TopologySummary`, a
  transform-invariant B-rep fingerprint. Validated end-to-end incl. an XT
  transmit/receive round-trip that preserves volume + topology.

### PK_EDGE_optimise + PK_BODY_simplify_geom (validated 2026-07-30)

- **`PK_EDGE_optimise_o_t` was silently ABI-wrong** and is now fixed from the
  authoritative RE catalog (`parasolid-re/catalog/pk-option-structs.tsv`):
  the real x86-64 layout is `o_t_version:int@0, max_dev:double@8,
  set_max_dev:PK_EDGE_max_dev_t@16, optimise_short@20` (size 24). The old
  binding had only `{o_t_version, optimise_short}` (8 bytes) — the kernel
  would have read 16 bytes past the struct. Token values were also
  placeholders: real ones are `PK_EDGE_optimise_short_{no,yes}_c =
  25640/25641`, `PK_EDGE_max_dev_{edge_tol,supplied}_c = 22880/22881`,
  `PK_EDGE_optimise_{success,failure}_c = 22870/22871` (all from
  `pk-enums.h`). Compile-time size/offset asserts added. `o_t_version = 1`
  accepted by V37.01.243 under argument checking [probed].
- **`Edge::optimise`** validated end-to-end (block edge made tolerant at
  1e-4/5e-4, then optimised): both `set_max_dev` arms drive the kernel
  correctly. Probed semantics: on an **exact** edge the call succeeds with
  `(success, achieved 0.0)` (no argument error); on tolerant edges over a
  30-part corpus + the synthetic block, `achieved_deviation` and the new
  tolerance bottom out at exactly **5.25e-7** (kernel floor, presumably
  1.05 × 5e-7 safety) whenever the true SP-curve deviation is below it —
  EDGE_optimise never returned an edge to exact (that is reset_precision's
  job).
- **`PK_BODY_simplify_geom` has NO option struct** (validated: V35 header doc
  + RE catalog; signature `(body, local:PK_LOGICAL_t, *n_geoms, **geoms)`).
  A vestigial `PK_BODY_simplify_geom_o_t` was removed from `editing.rs`;
  `PK_FACE_simplify_geom_o_t` gained its missing `want_geoms:PK_LOGICAL_t@4`
  (size 8) from the catalog.
- **`Body::simplify_geom`** validated: a degree-2 **rational** B-curve wire
  edge that is exactly a quarter circle (new `Curve::bcurve_rational`,
  homogeneous `(xw,yw,zw,w)` vertices) simplifies to an analytic `Circle`
  with the exact radius. Corpus behavior (30 seeded xt-parser-validation
  parts): replaced one B-surface per body in 20/30 (17 → analytic
  Plane/Cyl/Cone/Sphere/Torus, 3 → simpler B-surface); `local=true` and
  `local=false` gave identical results on this sample. **Side effect worth
  knowing:** after surface replacement the kernel recomputed the geometry of
  53.6% (60/112) of null-curve tolerant edges into exact Line/Circle/Icurve
  edges at 5e-9 precision — tolerant-edge repair came from simplify_geom,
  never from EDGE_optimise. Previously-exact conic edges were never
  disturbed (56/56 preserved).

### Deferred (bindings kept, signature-audited, runtime-blocked)

- `Entity::clashes_with` (`PK_TOPOL_clash`) → mild **9999** even with NULL
  options (reference confirms the 9-arg signature; needs a fuller frustrum).
- `Body::make_general` (`PK_TOPOL_make_general_body`) → mild **10** (needs a
  partition/rollback store), which gates `Vertex::delete_acorn` — decompile
  shows `delete_acorn` raises **5064** unless the vertex's body is an internal
  **general body (type 6)**.
- `Body::make_section_with_surfs` (`PK_BODY_make_section_with_surfs`) returns 0
  bodies with both NULL and a computed 80-byte `PK_BODY_make_section_o_t`
  (the field @72 vs Rust's @68 for `keep_coi_faces` hints the layout is off);
  needs `PKU_journal_BODY_make_section_o` decompiled. `section_with_surf`
  (split-in-place) already covers the section oracle.

## Stage 0 — the trust boundary (2026-08-09)

Closes Stage 0 of [`CADABRA3-PRIORITIES.md`](../CADABRA3-PRIORITIES.md). Two of
the four items listed there were already done before this pass (see "stale
status" below); what follows is the new work.

### `PK_ERROR_sf_t` — 116-byte layout confirmed at runtime

The journal-derived layout in `parasolid-sys` was correct but unused: the
wrapper still read a raw buffer and trusted only `function` and `code`, guessing
severity from the code. `crates/parasolid-test/src/bin/error_probe.rs` raised
four distinct kinds of error and dumped the buffer; every field reads coherently
and **no byte at or past offset 116 is ever written**:

| offset | field | observed |
|---:|---|---|
| 0 | `function` (inline `char[32]`) | `"PK_BODY_create_solid_block"` |
| 32 | `code` | 15 |
| 36 | `code_token` (inline `char[32]`) | `"PK_ERROR_distance_le_0"` |
| 68 | `severity` | 1 (mild) |
| 72 | `argument_number` | 1 |
| 76 | `argument_name` (inline `char[32]`) | `"x"` |
| 108 | `argument_index` | 0 (`-1` when not an array) |
| 112 | `entity` | 999999 for a bogus-tag error |

`query_last_error` now reads the typed struct and reports real severity, the
kernel's own code token, the named bad argument, and the offending entity.

Two behavioural findings:

- **The record is not cleared by a successful call.** After a failure, a
  subsequent successful call leaves `was_error` true and the old record intact.
  Anything reading `PK_ERROR_ask_last` unconditionally will misattribute it, so
  `query_last_error` takes the expected code and drops a record that disagrees.
- **`PK_THREAD_ask_last_error` works.** The previous "faults inside the kernel"
  verdict was an artifact of the old pointer-based struct, not the function. It
  returns the same record as `PK_ERROR_ask_last` for the same signature.

### `PK_ERROR_*` code table — the whole thing was wrong

`PK_ERROR_raise` accepts a `PK_ERROR_sf_t` carrying a bare numeric code, and the
kernel **canonicalizes** it: `PK_ERROR_ask_last` then reports the official
`code_token` for that code. Sweeping 0..=9000
(`crates/parasolid-test/src/bin/error_table_probe.rs`) recovered **631 codes**,
now generated into `parasolid-sys/src/error_codes.rs` and marked `[probed]`.

Every one of the 22 previously hand-written constants that could be checked
disagreed with the kernel, and two pairs collided outright
(`system_error`/`modeller_not_started` both 2; `not_implemented`/
`cant_heal_wound` both 600). Examples:

| constant | was | actually |
|---|---:|---:|
| `PK_ERROR_distance_le_0` | 502 | **15** |
| `PK_ERROR_not_an_entity` | 504 | **22** |
| `PK_ERROR_cant_be_aborted` | 7 | **965** |
| `PK_ERROR_fatal_error` | 3 | **948** |
| `PK_ERROR_not_implemented` | 600 | **5000** |

Two live bugs followed from this:

1. `pk_check` treated code **7** as a success synonym for `cant_be_aborted`,
   silently swallowing every `PK_ERROR_has_no_name` (which really is 7).
2. `PsError::from_code` dispatched `NotAnEntity` on 504 — a code the kernel
   never emits — so that arm could never fire.

Cross-validation: the raised-code path and real triggered errors agree on
`distance_le_0`=15, `not_an_entity`=22, `o_t_version_unknown`=5022,
`field_of_wrong_type`=5014, `o_t_version_incorrect`=5043 and `not_general`=5064,
the last three matching earlier independent findings.

**Caveat: the sweep proves nothing about severity.** The probe supplies severity
on the raised struct and the kernel echoes it back. Severity must be read from
offset 68 of a *real* error. Every real error observed so far is mild (1);
serious and fatal remain unobserved.

### Newly bound functions

- `PK_boolean_r_f` — was referenced in a comment but never declared. The wrapper
  freed only the `bodies` array and leaked the rest of the result struct; it now
  copies the tags out and frees through the matching API.
- `PK_ENTITY_track_r_f` — partner of `PK_ENTITY_track_r_t` outputs already
  produced by `bgeom.rs`/`enquiry.rs`.
- `PK_TRANSF_classify_r_f`; `PK_TRANSF_classify`'s `options` corrected to
  `*const`.
- The five `PK_LATTICE_*` functions with **no documented prototype in any
  reference** — recovered by decompiling the exports plus their journal helpers:

  ```
  PK_LATTICE_ask_core         @1803466c0  (lattice, options, results)
  PK_LATTICE_ask_form         @180347260  (lattice, options, results)
  PK_LATTICE_ask_connectivity @180345ed0  (lattice, options, results)
  PK_LATTICE_ask_regions      @180348eb0  (lattice, options, results)
  PK_LATTICE_create_by_core   @18034b980  (options, results)   <- 2 args
  ```

  `PK_LATTICE_ask_regions_r_t` is **fully mapped** from
  `PKU_journal_LATTICE_ask_regions_r`: `{r_t_version@0, n_regions@4,
  regions@8, senses@16, frames@24}`, where `senses` is byte-indexed (one-byte
  logicals) while `regions`/`frames` are 4-byte tag arrays. The other option and
  result structs keep opaque bodies on purpose — the journal names their leading
  fields but not the full union tails, and inventing the rest is exactly what
  produced the wrong error table.

### The `o_t_version` protocol

Written up as [`docs/option-version-protocol.md`](option-version-protocol.md),
with `crates/parasolid-test/src/bin/option_version_probe.rs` implementing the
sweep step. Reproduced on demand: mass props accepts **1..=7**, `boolean_2`
accepts **2..=19** with version 1 reported `incorrect` — matching the values
recorded during the original boolean investigation.

### Stale status corrected

`TODO.md` predates several commits and understated the position. Already done
before this pass: `PK_PARAM_sf_t` (40-byte layout decompiled from
`PK_CURVE_ask_param`, wrapped as `Curve::param`/`Surf::params`, tested) and
`PK_BODY_boolean_2` (unblocked and volume-validated). Both were listed as
blockers in the first cut of `CADABRA3-PRIORITIES.md`.

Suite: **137 passed, 0 failed, 1 skipped** (was 133 before this pass).

## Stages 1–2 — numerics, frames, transforms (2026-08-09)

Suite: **148 passed, 0 failed, 1 skipped** (137 → 148).

### Stage 1 — the numeric contract

- **Create→ask is bit-exact.** Authored `f64` values with full mantissas
  (radius `3.7000000000000004`, origin components `1.2345678901234567`,
  `-9.876543210987654`) survive `PK_SPHERE_create` / `PK_CYL_create` /
  `PK_CIRCLE_create` / `PK_POINT_create` and come back with **identical bit
  patterns**. Verified by `to_bits()` comparison, not an epsilon.
- **Exactness holds across the scale ladder** 1e-6 … 1e8 for both radii and
  origins.
- **Storage is independent of session precision.** The same surface built under
  precision 1e-9 and 1e-5 reads back bit-identically, so an oracle value does
  not depend on a session setting. This is what lets CADabra's comparator use
  **exact relations** rather than bands for authored analytic parameters.
- **Precision set/readback/restore round-trips exactly** (1e-7, 1e-6, 1e-9),
  and the default linear/angular precisions are asserted to be in the expected
  sub-1e-6 range.
- **Degenerate input is rejected, not repaired**, and now that the error table
  is probed the tests assert *which* failure: zero and negative radii both give
  `PK_ERROR_radius_le_0`; a zero-length axis is a distinct, separately-coded
  failure.

### Stage 2 — classification and validation

Layouts recovered by the journal method and confirmed against raw result bytes:

```text
PK_TRANSF_classify_o_t   { o_t_version@0, diagnostics@4 }
PK_TRANSF_classify_r_t   120 bytes, gapless:
  @0   matrix_type            @8   determinant
  @16  unit_rows_deviations   @40  orthog_rows_deviations
  @64  translation            @88  perspective
  @112 scale
PK_GEOM_transform_o_t    { o_t_version@0, tolerance@8, modify@16,
                           want_out_geoms@20, want_exact@21 }
PK_TRANSF_transform_o_t  { o_t_version@0, operation_1@4, operation_2@8,
                           modify@12 }
```

`PK_TRANSF_check_o_t { o_t_version, max_faults }` was already modelled and the
decompile independently confirms it.

- **`PK_matrix_type_*` tokens are correct** — all five observed at runtime:
  identity 25290, rotation 25291, reflection 25292, general 25293,
  unclassified 25294.
- **`PK_TRANSF_diagnostics_t` has exactly two members** — `none` = 25300
  (the default the kernel writes into its own options) and `all` = 25301;
  `PK_TRANSF_classify` validates against `{0x62d4, 0x62d5}`. `none_c` was not
  previously bound.
- **Trap: `matrix_type` is not a rigid-motion predicate.** The kernel stores
  `unclassified` up front and only overwrites it for recognised cases. A pure
  translation with unit `matrix[3][3]` classifies **`Unclassified`**, while the
  *identical* translation with a global scale classifies `Identity`. Pinned by
  `stage2_classify_lattice` so it cannot change silently. CADabra must test the
  linear part directly rather than trusting this token for translations.
- **Shear has unit determinant**, so determinant alone cannot detect it — only
  `orthog_rows_deviations` does. Non-uniform scale shows up in
  `unit_rows_deviations` and a determinant of 24 for diag(2,3,4).
- **`PK_TRANSF_create_rotation` requires a unit axis**: `(1,1,1)`, `(0,0,2)` and
  the zero vector are all rejected with `PK_ERROR_not_a_unit_vector`. A
  precondition for CADabra's type system, not a runtime discovery.
- **Orphan geometry can be placed obliquely** via `PK_GEOM_transform_2`, wrapped
  as `Surf::transformed` / `Curve::transformed`, which also return the kernel's
  **`exact`** flag — a rigid motion of an analytic sphere is achieved exactly.
  This is what stops later SSI fixtures from being axis-aligned-only.

### Deliberately not done

`PK_TRANSF_transform_2` and `PK_BODY_transform_2` stay unwrapped. Their
capabilities are already covered by the validated non-`_2` forms
(`Transform::then`, `Body::transform`), and the `_2` variants add only option
fields whose tokens (`operation_1`/`operation_2`, `modify`) are not probed.
Binding them now would mean passing NULL options — i.e. the same behaviour with
more surface area. Their option layouts are recorded above for when a case needs
them.

## Stage 3 — evaluation and jets (2026-08-09)

Suite: **155 passed, 0 failed, 1 skipped** (148 → 155). New module
`crates/parasolid/src/jet.rs`; probe
`crates/parasolid-test/src/bin/eval_probe.rs`.

### Derivative array layout — measured, not assumed

`PK_SURF_eval` writes ∂^(i+j)R/∂u^i∂v^j with **u varying fastest**. Recovered by
evaluating a **torus**, whose mixed partials are all nonzero so no ordering can
hide behind a zero, and matching every slot against closed form:

```text
rectangular, n_u = n_v = 2                 index = j*(n_u+1) + i
  [0]=(0,0) [1]=(1,0) [2]=(2,0)            slots = (n_u+1)*(n_v+1)
  [3]=(0,1) [4]=(1,1) [5]=(2,1)
  [6]=(0,2) [7]=(1,2) [8]=(2,2)

triangular, order 2                        index = j*(n+1) - j*(j-1)/2 + i
  [0]=(0,0) [1]=(1,0) [2]=(2,0)            slots = (n+1)*(n+2)/2
  [3]=(0,1) [4]=(1,1)
  [5]=(0,2)
```

Triangular is the *same* ordering with each row truncated to `i+j <= n`. The
asymmetric case `n_u=3, n_v=1` confirms the `n_u+1` stride. `PK_CURVE_eval` is
the trivial case: slot `k` is d^k/dt^k. All verified against closed form, and
the two surface packings are cross-checked against each other.

Wrapped as `Surf::eval_jet` → `SurfJet` and `Curve::eval_jet` → `CurveJet`,
indexed by `(i, j)` / `k` rather than by raw slot. Out-of-table requests return
`None` instead of a plausible neighbour.

### Curvature sign convention

- The reported normal is **outward** on a sphere, and both principal curvatures
  are **+1/r**. So **positive curvature means the surface bends away from the
  normal**.
- `principal_curvature_1` pairs with `principal_direction_1`; the pairing is the
  meaning, not any magnitude ordering. On a cylinder, `k1 = 0` with
  `direction_1` along the axis and `k2 = 1/r` around the hoop.
- Torus confirms the convention end to end: outer equator
  `k1 = +1/(MAJ+MIN)` with positive Gaussian curvature, inner equator
  `k1 = −1/(MAJ−MIN)` with negative Gaussian curvature (a saddle).

### Parametric singularity is not geometric singularity

At a sphere pole the **chart** degenerates — `|∂R/∂u|` is 2.4e-16, so no normal
can be formed — but `PK_SURF_eval` **still succeeds and raises no error**. A
caller that ignored the degenerate cross product would propagate a garbage
direction silently. `SurfJet::unit_normal` returns `None` there and
`is_singular()` is true; `None` is a geometric statement, not an error.

Meanwhile `PK_SURF_eval_curvature` at that same pole returns the correct
**1/r** — the surface is perfectly smooth, only the parameterisation is not.
Conflating the two is a modelling error, so the API keeps them separate. A cone
apex is singular in both senses.

### Minimum radius of curvature

`PK_CURVE_find_min_radius` and `PK_SURF_find_min_radii` were already declared in
`enquiry.rs` (correct by-value → `*const` convention for `PK_INTERVAL_t` /
`PK_UVBOX_t`); now wrapped and validated:

- circle r=3 → 3.0; **straight line → `n_radii = 0`**, surfaced as `None` rather
  than infinity or an error;
- torus (MAJ=5, MIN=1.5) → **two signed radii, `3.5` and `−1.5`** — the sign
  follows the curvature convention, so a caller taking absolute values loses
  information;
- plane → no minima at all.

The surface form writes **up to two** entries into each of three buffers, so all
three must have room for 2 regardless of the returned count.

### Handed evaluation

`PK_HAND_left_c` / `_right_c` (32760 / 32761) were unexercised constants; both
are accepted, and on a smooth circle the left, right and two-sided jets agree
exactly at every order. Wrapped as `Curve::eval_jet_handed`.

## Stage 4 — domains: intervals, uv-boxes, periodicity, seams (2026-08-09)

Suite: **161 passed, 0 failed, 1 skipped** (155 → 161). Probe
`crates/parasolid-test/src/bin/domain_probe.rs`.

Every Stage 4 call was already bound and wrapped, so this stage was entirely
about semantics — and about the information the existing wrappers were throwing
away.

### `PK_PARAM_sf_t` — the three undecoded fields

`extent` / `form` / (formerly) `convexity` carried tokens present in **no**
catalog, and their journal strings are absent from this build. Recovered by
decompiling `PK_CURVE_ask_param` and cross-checking every analytic family at
runtime:

- **`extent` / `form`** — the kernel maps an internal 0..3 code onto tokens in
  *descending* order (`0 → 18003`, `1 → 18002`, `2 → 18001`, `3 → 18000`);
  `PK_SURF_ask_params` also emits `18004`. Observed: **18000** on unbounded
  ranges (line, plane, cylinder v), **18003** on periodic ranges, **18004** on
  bounded ranges (sphere v, cone v). `18001`/`18002` are reachable in the
  mapping but no analytic family produced them, so they are left **unnamed**
  rather than guessed. `extent` and `form` agree everywhere except a
  half-bounded range: a cone's v is `extent = 18004` but `form = 18000`.
- **`periodic`** is *derived*, not independent: the decompile shows extent code
  0 (token 18003) selects yes/seamed and everything else selects no. So
  `extent == Periodic` and `periodic == yes` are two views of one fact.
- **`convexity` was a misnomer.** The field is computed from the underlying
  iso-curve's internal class tag: `30 → 18040`, `31 → 18041`, anything else
  `→ 18042`. Renamed **`curve_class`**, with observed values straight (line,
  plane, cylinder v), circular (circle, cylinder u, torus), other (ellipse,
  sphere v). It is a representation fact, not a geometric measurement.

Exposed as `ParamExtent` / `ParamCurveClass`, both of which carry an
`Other(i32)` arm so an unobserved token passes through instead of being
coerced into a wrong name.

### Seams are exact identifications; poles are not seams

- Across a seam, `u` and `u + period` agree in **position and in every first
  derivative** to ~1e-15 on cylinder, sphere and torus (and in v as well for the
  torus). Position agreement alone would still permit a kink; derivative
  agreement is what licenses treating the domain as a quotient.
- A **pole** is a different animal: at a sphere's `v` extremes the entire `u`
  fibre collapses to one point (spread ~4e-16 across `u = 0, 2, 5`) and
  `|∂R/∂u|` vanishes. So the domain type cannot simply be "wrapped interval" —
  seam identification and fibre collapse are distinct boundary phenomena.

### Face uv-boxes are conservative; `is_uvbox` is one-sided

Measured on a cylinder body:

- The planar cap face is a **disc of radius 2**, whose exact box is `[-2,2]²`.
  `PK_FACE_find_uvbox` reports `[-2.024, 2.024]²` — **padded by ~1.2%**. It is a
  superset, safe for exclusion tests and wrong for anything that needs
  tightness.
- The cylindrical wall reports `[0, 2π] × [0, 6]` — **exact**, and
  `PK_FACE_is_uvbox` confirms it is a parametric rectangle and hands back that
  rectangle.
- The vendor reference states `PK_FACE_is_uvbox` "does not guarantee to detect
  that a face is parametrically rectangular but will never claim a face is
  parametrically rectangular when it is not". So **`true` is trustworthy;
  `false` means "not established", not "definitely not"**. Wrapped as
  `Face::as_uvbox() -> Option<UvBox>` so the exact box comes back with the
  verdict.

`Face::is_periodic` collapsed the kernel's three-way answer into a bool, losing
the **seamed** case — a periodic direction that has been cut, so the face has a
real edge where the parameterisation wraps. Added `Face::periodicity()`
returning the full `Periodicity` per direction.

### Arc length carries a conservative enclosure

`PK_CURVE_find_length` has a fourth output — a **range bounding the true arc
length** — that the wrapper was discarding:

- circle (r=3): length `6π`, enclosure width **exactly 0** — analytically exact;
- ellipse (5×2): length ≈ 23.013113, enclosure `[23.013111, 23.013114]`, width
  **3.06e-6** — no closed form, and the kernel says how well it knows the answer.

Added `Curve::length_with_bounds`. Anywhere a length feeds a tolerance decision,
the plain `length()` is the wrong call.

## Stage 5 — inversion, distance, extrema (2026-08-09)

Suite: **167 passed, 0 failed, 1 skipped** (161 → 167). Probe
`crates/parasolid-test/src/bin/range_probe.rs`. This stage leaned heavily on the
`parasolid-re` Ghidra bridge — three separate struct recoveries below.

### The result contract: status + witness

`RangeResult` carried only `{distance, point_1, point_2}` and threw away both
the `PK_range_result_t` status and the per-end witness, which the kernel
populates fully. Rebuilt to carry `RangeStatus` plus a `RangeWitness`
(`entity`, `sub_entity`, `position`, `parameters`) per end. Validated on a block:
a probe point above a face witnesses a **FACE**, beyond a vertical edge an
**EDGE**, beyond a corner a **VERTEX** — and the witness position is exactly the
reported distance from the probe.

**`Found` does not mean unique.** A point on a cylinder's axis is equidistant
from every point of the wall, and the kernel still answers `Found` with one
arbitrary (though deterministic) witness. `RangeStatus::Found` asserts "a
minimum was located", never "the minimum is unique"; nothing in the result
carries multiplicity. This is the single most important thing for CADabra to
encode here.

### Inversion is not projection

`PK_SURF_parameterise_vector` / `PK_CURVE_parameterise_vector` **reject**
off-surface points rather than returning a nearest point:
`PK_ERROR_not_on_surface` (915) and `PK_ERROR_not_on_curve` (67). A sphere's
centre is refused too. So these are strict inversions; treating them as
projections would silently convert "nowhere near" into a plausible `(u,v)`.

A point on a seam inverts to **one** representative (`u = 0`, not `2π`), so
periodic equivalence remains the caller's to canonicalize — exactly the
Stage 4 result.

### `PK_TOPOL_clash` was never frustrum-blocked

It had been recorded as permanently deferred, "needs a fuller frustrum", on the
strength of a mild **9999**. With the Stage 0 error work in place the kernel
says precisely what is wrong: **bad argument #3, `tf1`**. Two real bugs:

1. **The transform arrays are mandatory.** The reference's "substitutes an
   identity transform internally" means a per-entity array whose *entries* may
   be `PK_ENTITY_null` — not a null array pointer. Passing NULL is rejected.
2. **`PK_TOPOL_clash_o_t` was wrong.** Recovered from
   `PKU_journal_TOPOL_clash_o`: it omitted the three leading exception-list
   fields (`n_op_ex`, `op_ex1`, `op_ex2`) entirely and modelled four logicals as
   4-byte ints when the kernel packs them as **single bytes at 24..27**. Every
   field after `o_t_version` sat at the wrong offset. Real size 56 bytes.

`PK_TOPOL_clash_t` was also a bare `c_int` typedef, so the returned array could
not be read at all. The journalling loop in the real export (@18043f9a0) walks
it with a stride of **5 ints** — `{target, target_index, tool, tool_index,
clash_type}`, 20 bytes — now bound as `PK_TOPOL_clash_rec_t` and surfaced as
`Entity::clash_records`.

**The clash-type tokens were fabricated too.** The bindings claimed 0..4;
probing known configurations gives:

| configuration | token |
|---|---:|
| identical blocks / partial overlap | **7** (interfere) |
| blocks sharing exactly one face | **4** (abut) |
| small block strictly inside a large one | **2** (contained) |
| disjoint blocks | *no records* |

Constants replaced with these `[probed]` values. Containment was not probed in
both directions, so no separate `b_in_a` constant is claimed.

### `PK_ENTITY_range` multi-solution structs

`PK_ENTITY_range_r_t` was an opaque stub, so the **multiple** solutions this
call exists to return could not be read. Recovered from
`PKU_journal_ENTITY_range_r`:

```text
@0 r_t_version   @4 n_results
@8 results  (PK_range_result_t*, per-solution status)
@16 distances (double*)
@24 ends_1  @32 ends_2   (PK_ENTITY_range_end_t*)
```

with `PK_ENTITY_range_end_t` from `PKU_journal_ENTITY_range_end` — a 64-byte
record that, unlike the simpler `PK_range_end_t`, carries a trailing **`inside`**
classification. Both are decompile-derived and **not yet runtime-validated**:
the multi-solution call is bound but not wrapped, and the structs say so.

### Also

`Body::find_extreme` wrapped: the three directions act as ordered tie-breakers,
and the returned topology says whether a vertex, edge or face realised the
extreme. Three independent directions pin a vertex on a block.

`PK_CURVE_project` is **not** part of this stage despite the plan listing it
here — it projects curves onto target bodies with tracking and an opaque result
struct, which is Stage 12 arrangement work.

## Stage 6 — ranges and conservative enclosures (2026-08-09)

Suite: **172 passed, 0 failed, 1 skipped** (167 → 172). New module
`crates/parasolid/src/enclosure.rs`; probe
`crates/parasolid-test/src/bin/box_probe.rs`. Ghidra did most of the work here —
**five** struct layouts were wrong.

### Enclosures are tight, not padded

Measured against analytically exact boxes (sphere, torus, cylinder, circle,
block), `PK_SURF_find_box` / `PK_CURVE_find_box` / `PK_TOPOL_find_box` return the
**exact** extent, slack 0.0 on every face. That is the opposite of the
parameter-space `PK_FACE_find_uvbox` measured in Stage 4, which is padded ~1.2%.
The two must not be reasoned about together.

**Tightness removes the safety margin.** A quarter arc of a circle touches x=0
exactly, and the reported box `min.x` comes back at **+1.8e-16 — inward by about
one ULP**. Any exclusion test using exact comparison could therefore reject a
point genuinely on the boundary. Exclusion must use a tolerance; the enclosure
is *not* a guaranteed superset at the bit level.

An unbounded carrier is refused rather than approximated: an unrestricted
`PK_SURF_find_box` on a plane gives `PK_ERROR_unsuitable_entity` (914). Callers
must pass the uv restriction.

### `widths` are half-widths

`PK_SURF_find_non_aligned_box` / `PK_CURVE_find_non_aligned_box` return what the
vendor reference calls "box width in each axis direction", but the measured
values are **semi-extents**: a circle of radius 3 reports 3.0, a sphere of
radius 4 reports 4.0, a line over [0,10] reports 5.0. Treating them as full
widths would inflate every enclosure by 2×.

`dimension` is genuinely useful and correct: **1** for a straight line, **2** for
a planar circle, **3** for a sphere. A caller assuming 3 mis-handles degenerate
geometry.

### Five wrong struct layouts, all recovered from the journal helpers

1. **`PK_range_bound_t` had upper and lower swapped.** The decompile of
   `PKU_journal_range_bound` gives `{have_upper_bound@0, upper_bound@8,
   have_lower_bound@16, lower_bound@24}`; the binding had lower first. A caller
   setting a lower bound would have set an upper one. Unnoticed because every
   existing call passes defaults (all zero).
2. **`PK_range_param_bound_t` was the wrong shape and size.** It modelled
   `{interval, uvbox}` side by side (48 bytes). Real form is 40 bytes:
   `{have_param_bound@0, param_bound_class@4, union{interval|uvbox}@8}`, with
   class `0x204` selecting the interval form.
3. **`PK_GEOM_range_vector_o_t` had its fields in the wrong order** (`opt_level`
   before `guess`) and carried a `param_bound` the kernel does not read here.
   Real layout matches `PK_TOPOL_range_vector_o_t` minus `param_entity`.
4. **`PK_GEOM_range_o_t` likewise** — real 240-byte layout is
   `{o_t_version, have_tolerance, tolerance, bound@16, guesses[2]@48,
   range_type@144, param_bound[2]@152, opt_level@232}`.
5. **`PK_CURVE_find_box_o_t` / `PK_SURF_find_box_o_t` were opaque stubs.**
   Recovered from how the functions read them: `{o_t_version@0,
   have_interval@4, interval@8}` (24 B) and `{o_t_version@0, have_uvbox@4,
   uvbox@8}` (40 B).

`PK_TOPOL_range_vector_o_t` was checked against the decompile and is **correct**
— which is what makes the GEOM discrepancies credible rather than a decoding
error on my part.

### Zeroed enum fields are not valid options

`PK_GEOM_range_vector` rejected a zeroed options struct with
`PK_ERROR_field_of_wrong_type` (5014) naming `local_opts`. The cause is not a
layout error: **0 is not a legal token** for `guess_type` or `opt_level`, which
need `PK_range_guess_no_c` (18260) and `PK_range_opt_accuracy_c` (23761). A
`Default` that zeroes an options struct is wrong whenever it contains enum
fields — worth remembering for every future option struct.

### Inversion vs projection, settled

With the options fixed, `PK_GEOM_range_vector` works and is a genuine **global**
projection (the vendor reference says so explicitly for the array form):
`(10,0,0)` against a sphere of radius 4 gives distance 6 and witness `(4,0,0)`;
`(0,0,100)` gives 96 and the north pole. Contrast `PK_SURF_parameterise_vector`,
which *refuses* off-surface points (Stage 5). The two are not interchangeable
and are now wrapped separately: `Surf::range_to_point` vs `Surf::parameterise`.
