# parasolid-rs validation TODO

**Mission:** make `parasolid-rs` a trustworthy **golden oracle** for the
CADabra geometric kernel (`~/git/exp-cadabra`). Every binding CADabra will be
checked against must be validated end-to-end against the real
`pskernel.dll` (Parasolid V37.01.243, SOLIDWORKS 2025) before we rely on its
output as "ground truth."

## Why this ordering

CADabra's near-term surface (per its `ROADMAP.md` / `STATUS.md`) is:
analytic primitives **Plane, Sphere, Cylinder, Cone, Torus, Line3**; typed
frames/coordinates; **surface-surface intersection (SSI)** producing points /
curves / regions with coverage; bounded/trimmed arrangement; a **B-rep spine**
(Body → Region → Shell → Face → Loop → Fin → Edge → Vertex); and eventually
**regularized Booleans** via imprint → arrangement → classification → commit.

So the oracle must, in priority order, be able to: build those primitives,
evaluate their geometry exactly, intersect surfaces, interrogate topology,
compute mass/box properties, and round-trip bodies through XT files (so
CADabra-built and Parasolid-built models can be compared).

## Validation methodology (apply to every item)

1. **Signature audit** — diff the Rust `extern "C"` decl and every option/
   result struct against the mirrored V35 header docs in
   `~/git/solidworks-notes/headers/pk_<name>.html`. Field order and by-value
   vs by-pointer matter (this is what caused the original crashes).
2. **Runtime test** — exercise it under Wine against the real DLL; assert on
   concrete numeric/topological output, not just "no error."
3. **Token/enum probe** — any enum whose numeric values aren't in the docs
   gets probed from the DLL (see `crates/parasolid-test/src/bin/probe.rs`) and
   annotated `[probed]` / `[family]` / `[guess]` / `[unknown]` in `parasolid-sys`.
4. **Record** — mark the item below and note residual risks in
   `docs/pskernel-solidworks.md`.

Status legend: `[x]` validated at runtime · `[~]` signature-audited, not yet
runtime-tested · `[ ]` unaudited / untested · `[!]` known-wrong or suspect.

---

## P0 — Oracle foundation (blocks everything)

- [x] Session lifecycle: `PK_SESSION_start` / `_stop`, frustrum registration,
      `PK_SESSION_ask_kernel_version`.
- [x] Default frustrum: memory + file I/O callbacks, XT header handling, FFABOR.
- [x] `PK_CLASS_t` values for topology + analytic geom (probed). **Gap:**
      non-topology/geom classes (attrib, group, mesh, lattice, frame, …) are
      still `[guess]`/`[unknown]` — probe or leave clearly unusable.
- [x] `PK_BODY_type_t` values solid/sheet/wire/minimum (probed). **Gap:**
      acorn/empty/general/compound/unspecified are `[guess]` — probe via
      multi-vertex body, empty body, and a non-manifold body.
- [~] **Error code values** (`error.rs`): audit the full `PK_ERROR_*` numeric
      table against docs; several are needed to interpret oracle failures
      (`PK_ERROR_missing_geom`, `_bad_topology`, `_not_a_*`, …).
      **Fixed so far (probed under Wine):** the error-inquiry path used to
      *crash the process on any PK error*. Two bugs: (a) `PK_ERROR_sf_t.function`
      / `bad_arg_names` are **inline char arrays**, not `*const char` — the old
      code dereferenced ASCII bytes as a pointer and page-faulted; (b)
      `PK_THREAD_ask_last_error` faults inside the kernel. `query_last_error`
      now reads a raw buffer via `PK_ERROR_ask_last` and extracts only the
      confirmed fields: function name (inline, bytes 0..32) and code (i32 @32).
      Confirmed codes: `field_of_wrong_type`=5014, `o_t_version_unknown`=5022.
      **Remaining:** the real `PK_ERROR_sf_t` is larger and carries extra inline
      string fields (error-name, bad-field name); severity / n_bad_args /
      bad_args / entity offsets are still unknown — needs a header audit before
      `bad_args` can be surfaced. `PK_THREAD_ask_last_error` needs a
      signature/threading audit.
- [x] `PK_SESSION_set_check_arguments(true)` on by default in tests, so the
      kernel validates our FFI arguments and surfaces mismatches early. Done via
      a `test_config()` helper used by every test.

## P1 — Primitive construction (CADabra's primitive set)

Verify each builder + its `_sf_t` round-trips (`create` then `ask` returns
the same numbers). `_sf_t` field order was wrong before — audit all.

- [x] Block, cylinder, sphere, torus (create + face/edge/vertex counts).
- [x] Cone (`PK_BODY_create_solid_cone`) — **signature was wrong** and always
      failed. Real form is `(radius, height, semi_angle, basis_set, body)` — a
      base radius + apex half-angle, matching `PK_CONE_sf_t`, **not** the
      invented `(top_radius, bottom_radius, height)` frustum API. Base sits on
      z=0 and the cone widens toward +z: top radius = `radius + height *
      tan(semi_angle)`. Fixed and volume-validated against the frustum formula.
- [x] `PK_SPHERE_sf_t`, `PK_CYL_sf_t`, `PK_TORUS_sf_t`, `PK_CIRCLE_sf_t`,
      `PK_ELLIPSE_sf_t`, `PK_CONE_sf_t` field order (basis_set first) — fixed.
- [ ] Standalone geometry create/ask round-trips (not via a body):
      `PK_PLANE_*`, `PK_CYL_*`, `PK_CONE_*`, `PK_SPHERE_*`, `PK_TORUS_*`,
      `PK_LINE_*`, `PK_CIRCLE_*`, `PK_ELLIPSE_*`, `PK_POINT_*`. These are the
      exact analytic types CADabra mirrors — each must round-trip bit-stably.
- [ ] `PK_SURF_make_sheet_body` (by-value uvbox — fixed sig, untested) and
      `PK_CURVE_make_wire_body(_2)` — needed to wrap a bare surface/curve as a
      body for oracle interrogation.
- [ ] B-curve / B-surface create + ask (`bgeom.rs`, 66 fns, all unaudited) —
      needed once CADabra emits NURBS; low priority until then.

## P2 — Geometry evaluation (exact oracle sampling)

The core oracle primitive: given a surface/curve + parameters, get exact
points/derivatives to compare against CADabra's evaluators.

- [x] `PK_SURF_eval` (position + first derivatives), `PK_CURVE_eval`.
- [~] `PK_CURVE_eval_handed` — arg order fixed (n_derivs before hand), untested.
- [~] Surface normal — `Surf::eval_with_normal` (via `PK_SURF_eval` first
      derivatives, rectangular packing) validated: on a sphere it returns a
      unit, outward-radial normal at every sample, so the du/dv layout
      (u @ p[1], v @ p[2]) is correct. Note `PK_SURF_eval_with_normal` itself is
      **mesh-specific** (averages mvertex normals) — not for analytic surfaces.
      Higher-order derivative tables / `triangular` flag still untested.
- [ ] `PK_SURF_parameterise` / `PK_CURVE_parameterise` (point → (u,v)/t) —
      needed to compare parameterizations and for closest-point oracle.
- [ ] `PK_SURF_ask_uvbox` / `PK_FACE_find_uvbox`, periodicity & seam data
      (`PK_SURF_ask_...` periodic flags) — CADabra's seam/pole handling needs
      the oracle's periodicity/singularity conventions pinned down exactly.
- [x] Analytic curve extraction — `ask_line`/`ask_circle` validated: a
      cylinder's 2 circular edges round-trip to radius 5 with centres on the Z
      axis at the cap planes; a block line edge gives a unit direction, an
      arc-length interval matching the chord, and a unit tangent from
      `eval_with_tangent`. `curve.rs` reviewed — no bugs found.
- [ ] `PK_CURVE_ask_..` closed/periodic, sense (interval covered above).

## P3 — Topology interrogation (B-rep spine parity)

CADabra publishes a Body→Region→Shell→Face→Loop→Fin→Edge→Vertex spine;
the oracle must expose the same adjacency so structures can be compared.

- [x] `PK_BODY_ask_faces/edges/vertices`, `PK_FACE_ask_surf`,
      `PK_EDGE_ask_curve`, `PK_VERTEX_ask_point`, `PK_ENTITY_ask_class`,
      `PK_CLASS_ask_superclass`.
- [x] `PK_EDGE_ask_geometry` (7-arg signature — fixed).
- [x] `PK_BODY_ask_shells`, `PK_BODY_ask_regions`, `PK_FACE_ask_loops`,
      `PK_LOOP_ask_fins` — wrapped as `Region`/`Shell`/`Loop`/`Fin` and asserted
      by `brep_spine_block`: 2 regions (1 solid + void), shells round-trip to
      regions, 6 faces × 1 outer loop × 4 fins = 24 fins = 2 × 12 edges, fin
      4-cycles, each edge has 2 fins. Fixed `PK_LOOP_type_t` (was 0..8; real
      tokens are 5410..5419, outer=5412/winding=5414 confirmed).
- [ ] Full adjacency matrix test on the two-region/two-shell box (mirrors
      CADabra's target model): every Region↔Shell↔Face↔Loop↔Fin↔Edge↔Vertex
      link + fin orientation/sense, `PK_FIN_ask_..`, `PK_LOOP_ask_type`,
      `PK_FACE_ask_oriented_surf` (face sense vs surface normal).
- [ ] Edge/vertex convexity, `PK_EDGE_ask_convexity`, face-face edge sharing.

## P4 — Surface/surface intersection (SSI oracle — highest oracle value)

This is CADabra's central algorithm; the oracle here is the payoff.

- [ ] `PK_SURF_intersect_surf` — audit signature + result struct
      (`distance.rs`), wrap safely, and build an SSI oracle:
      for each analytic pair (plane-plane, plane-sphere, plane-cyl, plane-cone,
      sphere-sphere, sphere-cyl, cyl-cyl, cyl-cone, cone-cone, torus-*),
      produce the intersection curve set and compare topology + sampled points
      against CADabra rows.
- [ ] `PK_CURVE_intersect_curve` and curve/surface intersection — for p-curve
      arrangement checks.
- [ ] Coincidence/overlap detection to compare against CADabra's
      coincident-region / indeterminate relations.
- [ ] Represent Parasolid intersection curves (intersection curve `icurve`,
      class 3005-ish) well enough to sample — likely via edges of an imprinted
      body if direct curve eval is awkward.

## P5 — Mass / box / measurement (cheap, robust oracle signals)

Coarse invariants that catch gross modeling errors fast.

- [x] `PK_TOPOL_eval_mass_props` — **fully validated** (amount, mass, centre of
      gravity, inertia tensor, periphery) against closed-form values for
      block/sphere/cylinder/cone/torus, with `check_arguments` on. Wrapped as
      `Body::mass_props()` → `MassProps` (+ `volume()`/`mass()` conveniences).
      Signature + option struct recovered via the `parasolid-re` Ghidra project:
      the function takes an **options pointer** as its 4th arg; the **version-1**
      user struct is `{ o_t_version, mass, periphery, bound, single }` (offsets
      0/4/8/12/16), and the enum tokens are `0x36b1..0x36b4` (mass no/mass/
      c_of_g/m_of_i), `0x36b5/0x36b6` (periphery no/yes), `0x36b7` (bound no) —
      not the old 0/1/2/3 guesses. See `docs/pskernel-solidworks.md` finding 9.
      **Follow-up:** higher option versions (`facet_tol`, densities, `transfs`,
      `local_opts`, scale controls) still need their full layout before use.
- [x] Bounding boxes — `PK_TOPOL_find_box`. **Signature was wrong** (had an
      invented `options` arg); real form is `(topol, box)` with no options
      (confirmed by the reference *and* Ghidra; the options form is
      `PK_TOPOL_find_box_2`). Wrapped as `Body::bounding_box()` → `Aabb`,
      validated: block box is exactly `[-5,-10,0, 5,10,30]`, sphere box centred
      at origin with the right extent.
- [ ] `PK_TOPOL_range` / `PK_ENTITY_range` (point→body distance / range).
- [ ] `PK_ENTITY_ask_..` distance: `PK_ENTITY_range` point→body distance,
      `PK_TOPOL_..` clash/`PK_BODY_..` point containment
      (`PK_BODY_contains_vectors` or equivalent) — needed for
      inside/outside classification parity.

## P6 — File I/O round-trip (model interchange oracle)

Lets CADabra export a body and Parasolid read it (or vice-versa) so entire
models are compared, not just per-call outputs.

- [~] `transmit` / `receive` (`fileio.rs`) — wrappers exist, **never tested**.
      Audit `PK_PART_transmit` / `PK_PART_receive` option/result structs; test
      a full write→read→re-interrogate round-trip preserving topology + geometry.
- [ ] Text vs neutral-binary format selection; schema-file handling for
      cross-version XT (`FFCSCH`), since SOLIDWORKS ships older-version parts.
- [ ] Decide the interchange contract with CADabra: does CADabra emit/consume
      XT, or do we compare via a neutral sampled representation? (Design note.)

## P7 — Booleans & imprint (later; CADabra Phase E)

Large, subtle surface (`boolean.rs` = 44 fns, `editing.rs` = 64) — audit only
when CADabra's Boolean path needs oracling.

- [ ] `PK_BODY_boolean_2` (unite/subtract/intersect) — signature + options
      (`PK_boolean_o_t`) audit, then volume/topology checks vs known results.
- [ ] `PK_FACE_imprint_*` / `PK_BODY_imprint_*` — imprint oracle for CADabra's
      imprint→arrangement stage.
- [ ] Local ops (blends `blend.rs`, sweeps `sweep.rs`, offsets `offset.rs`,
      euler `euler.rs`) — defer; not on CADabra's near-term path.

## P8 — Checking (oracle self-trust)

- [ ] `PK_BODY_check` / `PK_ENTITY_check` — audit result structs
      (`checking.rs`); run on every oracle-produced body so we never compare
      against a body Parasolid itself considers invalid.
- [ ] `PK_SESSION_set_check_continuity` / self-intersection checks on.

## P9 — Determinism & precision hygiene

- [ ] Pin `PK_SESSION` precision / tolerance settings and record them, so
      oracle results are reproducible and comparable to CADabra's
      `ToleranceContext`.
- [ ] Confirm the frustrum + session are single-threaded and deterministic
      across runs (same tags, same numbers).

---

## Cross-cutting engineering tasks

- [ ] **Extend `parasolid-test`** from a bespoke harness into grouped suites
      (primitives / eval / topology / ssi / massprops / fileio) so each P-level
      above has runnable assertions. Keep the current single-binary Wine runner.
- [ ] **Audit-sweep the sys crate for guesswork**: grep for `[guess]` /
      `[unknown]` markers and any struct/signature not yet cross-checked
      against the header mirror. `parasolid-sys` has ~1150 `extern` fns; only a
      few dozen are validated. Treat everything unaudited as suspect.
- [ ] Build a tiny **oracle crate/module** (`parasolid` side) exposing the
      comparison primitives CADabra's testkit will call: `eval_surface`,
      `eval_curve`, `mass_props`, `intersect_surfaces`, `body_from_primitive`,
      `transmit/receive` — a stable, validated-only API. Nothing unvalidated
      leaks into the oracle surface.
- [ ] Define the **exp-cadabra ↔ parasolid-rs bridge**: how CADabra feeds a
      primitive/model to the oracle and how results are diffed (tolerances,
      topology canonicalization, parameterization alignment). Write it up in
      `docs/oracle-bridge.md` and mirror the plan into exp-cadabra's testkit
      docs once agreed.

## Housekeeping / security

- [ ] **Proprietary binaries scrubbed from git history** (`lib/pskernel.dll`,
      `lib/libpskernel.a`). Working-tree ignore + README + scrub script are in
      place; the history rewrite + force-push must be run manually
      (`scripts/scrub-proprietary-binaries.sh`) and GitHub Support asked to
      purge caches. **Until done, treat the binary as leaked.**
