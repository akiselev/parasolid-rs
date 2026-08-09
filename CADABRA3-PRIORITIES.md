# CADabra3 kernel build order — oracle priorities

The order in which `parasolid-rs` must become trustworthy, expressed as the
order in which we intend to **build the kernel**, bottom up: numerics, then
frames, then evaluation, then domains, then queries, then intersection, then
topology, then operations. Each stage names the architectural decision that
practice has to settle, and the oracle calls needed to settle it *with evidence
instead of taste*.

This replaces auditing Parasolid section-by-section. We audit exactly what the
next kernel layer needs, in the order we write that layer.

Companion documents: cadabra3 [`docs/plan/README.md`](../cadabra3/docs/plan/README.md)
(the ladder and its experiment shape), [`docs/plan/12-…-audit.md`](../cadabra3/docs/plan/12-parasolid-api-signature-audit.md)
(per-symbol record fields), `TODO.md` (historical P-levels), and
`parasolid-re/catalog/` (decompile evidence).

Status marks below: **✅** runtime-validated · **◐** wrapped, untested or partial ·
**✗** not wrapped · **⚠** demonstrated wrong.

---

## Stage 0 — Unblock ✅ **COMPLETE** (2026-08-09)

Not a build stage. Evidence recorded in
[`docs/pskernel-solidworks.md`](docs/pskernel-solidworks.md) §"Stage 0 — the
trust boundary". Suite went 133 → **137 passed, 0 failed, 1 skipped**.

**Two of the four listed blockers were already fixed before this pass** —
`TODO.md` predated several commits and this document inherited its staleness:

- ~~`PK_PARAM_sf_t`~~ ✅ — 40-byte layout already decompiled from
  `PK_CURVE_ask_param` (`range`@0, `extent`@16, `form`@20, `periodic`@24,
  `convexity`@28, `closed`@32), wrapped as `Curve::param` / `Surf::params` and
  tested. Stage 4 is **not** blocked.
- ~~`PK_BODY_boolean_2_o_t`~~ ✅ — already unblocked and volume-validated. The
  32-byte v2 user struct is confirmed; a NULL `configuration` is auto-filled, so
  the nested sub-structs are not needed at all.

**What this pass actually did:**

1. **`PK_ERROR_sf_t`** ✅ — the 116-byte layout is now **runtime-confirmed**
   (`error_probe.rs`), and the wrapper reads it: real severity from offset 68,
   the kernel's own `code_token`, the named bad argument, and the offending
   entity tag. Two behavioural facts fell out — the error record is **not**
   cleared by a successful call (so `query_last_error` drops records whose code
   disagrees with the one being reported), and **`PK_THREAD_ask_last_error`
   works fine**; its "faults in the kernel" reputation was the old pointer-based
   struct, not the function.
2. **The `PK_ERROR_*` code table** ✅ — *not* on the original list, and the
   biggest thing found. `PK_ERROR_raise` canonicalizes a bare numeric code, so
   sweeping 0..=9000 recovered **631 codes with their official token names**,
   now generated into `parasolid-sys/src/error_codes.rs`. **Every one of the 22
   checkable hand-written constants was wrong**, and two pairs collided. Not
   cosmetic: `pk_check` treated code 7 as a success synonym for
   `cant_be_aborted` (really 965), silently swallowing every
   `PK_ERROR_has_no_name` (really 7); and `NotAnEntity` dispatched on 504, a
   code the kernel never emits.
3. **The option-version protocol** ✅ — written up in
   [`docs/option-version-protocol.md`](docs/option-version-protocol.md), with
   `option_version_probe.rs` implementing the sweep step. Reproduces mass props
   **1..=7** and `boolean_2` **2..=19**. Includes the `PKU_journal_*` method,
   which reads field names straight out of the kernel's own journalling code.
4. **Absent bindings** ✅ — `PK_boolean_r_f` (referenced in a comment but never
   declared; the wrapper leaked everything in the result struct except the
   `bodies` array), `PK_ENTITY_track_r_f`, `PK_TRANSF_classify_r_f`, and the
   five `PK_LATTICE_*` functions that have **no documented prototype anywhere** —
   recovered by decompiling the exports and their journal helpers.
   `PK_LATTICE_ask_regions_r_t` is fully mapped; the other lattice option/result
   structs stay deliberately opaque rather than invented.

**Standing lesson.** The error table is the cautionary tale for this whole
audit: 22 constants, transcribed rather than probed, every one wrong, two of
them silently corrupting live control flow, under a green test suite. Where a
value can be probed, probe it; where it cannot, leave it opaque and say so.
Do not transcribe.

**Still open from Stage 0 (small, non-blocking):**

- Severity is only ever observed as **mild**. `serious` and `fatal` are
  unexercised, so the rollback/restart paths in `PsError` are untested. Needs
  deliberate serious/fatal triggers in isolated sessions (rung 01 experiment 4).
- `PK_imprint_r_t` remains an opaque `[u8;0]` stub — it is not an exported
  function and belongs to Stage 12, where its layout gets recovered alongside
  the imprint work.

---

## Stage 1 — Numerics and tolerance semantics ✅ **COMPLETE** (2026-08-09)

**Building:** the scalar policy. Exact dyadic/rational ingress, what a
`Length`/`Angle` is, and the `ToleranceContext` every later predicate consults.

**The architectural question:** is Parasolid's numeric contract one we can mirror
or one we must diverge from? Specifically — does the kernel read session
precision *implicitly* inside calls we intend to treat as exact, and does an
authored `f64` survive construction bit-for-bit or get repaired?

**Oracle calls:**

| call | state |
|---|---|
| `PK_SESSION_ask_precision` / `_set_precision` | ✅ set→readback→restore exact; defaults asserted |
| `PK_SESSION_ask_angle_precision` / `_set_angle_precision` | ✅ default asserted |
| `PK_POINT_create`/`_ask`, `PK_LINE_*`, `PK_CIRCLE_*`, `PK_ELLIPSE_*`, `PK_PLANE_*`, `PK_CYL_*`, cone/sphere/torus | ✅ **bit-exact** round-trip |

**What practice must produce:** a bit-exactness verdict per family (is
create→ask the identity on the mantissa, or is there normalization?); the set of
calls that silently consult session precision; the behaviour under a scale
ladder (1e-6 to 1e9 model units) and near-denormal inputs; and rejection
behaviour for non-finite / zero-length direction input.

**Verdict — the oracle is exact for authored analytic parameters.** Full-mantissa
`f64` values survive create→ask with **identical bit patterns**, across a
1e-6…1e8 scale ladder, and **independently of session precision** (same surface
built at 1e-9 and 1e-5 reads back bit-identically). So CADabra's comparator may
use **exact relations**, not bands, for these fields. Degenerate input is
rejected rather than repaired, with a specific code token per failure mode
(`PK_ERROR_radius_le_0` for zero/negative radii; a distinct code for a
zero-length axis).

Tests: `stage1_precision_set_readback_restore`, `stage1_create_ask_is_bit_exact`,
`stage1_scale_ladder_round_trips`, `stage1_rejects_degenerate_input`,
`stage1_geometry_storage_is_precision_independent`.

---

## Stage 2 — Frames and transforms ✅ **COMPLETE** (2026-08-09)

**Building:** typed frames, world/local space distinction, rigid-motion
composition. Further along than the rest of this document assumed — six
transform tests already pass (translation, uniform scale, matrix round-trip,
native constructors, compose/equality, rotation). What is missing is the
*classification and validation* half, plus applying transforms to geometry and
bodies.

**The architectural question:** is a transform a value in the geometry algebra or
an operation applied to it? Does Parasolid preserve exactness under composition,
or does it repair to an orthonormal frame with a hard-coded tolerance? Is
authored gauge (which basis was chosen) retained beside canonical semantic
equality, or discarded?

**Oracle calls:**

| call | state |
|---|---|
| `PK_TRANSF_create`, `_create_rotation`, `_create_equal_scale`, `_create_reflection` | ✅ native constructors wrapped + tested |
| `PK_TRANSF_ask`, `_is_equal`, `_transform` | ✅ matrix round-trip, compose, equality tested |
| `PK_TRANSF_check` | ✅ `Transform::check` — valid transforms report no faults |
| `PK_TRANSF_classify` (+ `_classify_r_f`) | ✅ `Transform::classify` — 120-byte result layout recovered and byte-confirmed |
| `PK_TRANSF_create_translation` | ◐ covered by the generic `create` path |
| `PK_GEOM_transform_2` | ✅ `Surf::transformed` / `Curve::transformed`, incl. the `exact` flag |
| `PK_TRANSF_transform_2`, `PK_BODY_transform_2` | ⏸ deferred with reason (see below) |

**What practice must produce:** the classification lattice (identity / rigid /
scaled / reflected / general) with its tolerance; equality semantics for two
transforms composed by different authored paths; handedness and reflection
behaviour; and whether `_check` accepts a slightly non-orthonormal basis
silently (a repair we would have to model explicitly).

**Verdict.** Orphan geometry can now be placed at arbitrary oblique poses and
read back (`Surf::transformed`), with the kernel's own **`exact`** flag saying
whether the analytic form survived — so later SSI fixtures need never be
axis-aligned. The classification lattice is validated against all five
`PK_matrix_type_*` tokens, and shear/non-uniform scale are correctly refused the
status of similarities.

Three findings CADabra must encode:

1. **`matrix_type` is not a rigid-motion predicate.** The kernel initialises it
   to `unclassified` and only overwrites it for recognised cases: a pure
   translation with unit `matrix[3][3]` classifies **`Unclassified`**, while the
   *same* translation with a global scale classifies `Identity`. Test the linear
   part directly instead.
2. **A shear has unit determinant**, so a determinant check cannot detect one —
   only `orthog_rows_deviations` does.
3. **`PK_TRANSF_create_rotation` requires a unit axis** — `(1,1,1)` is rejected
   with `PK_ERROR_not_a_unit_vector`. That belongs in the type system.

Tests: `stage2_classify_lattice`, `stage2_classify_diagnostics`,
`stage2_check_accepts_and_rejects`, `stage2_transform_orphan_geometry`,
`stage2_rotation_axis_must_be_unit`, `stage2_classify_general_and_shear`.

**Deferred with reason:** `PK_TRANSF_transform_2` and `PK_BODY_transform_2` add
only option fields whose tokens are unprobed; their capabilities are already
covered by the validated non-`_2` forms. Option layouts are recorded in
`docs/pskernel-solidworks.md` for when a case needs them.

---

## Stage 3 — Evaluation and jets ✅ **COMPLETE** (2026-08-09)

**Building:** the query-engine core. `CurveJet` / `SurfaceJet`, derivative
ordering, and the memoization boundary.

**The architectural question:** where does the boundary sit between immutable
geometry values, a shared query engine, and cached jets? Get this wrong and
every later algorithm grows its own Newton solver.

**Oracle calls:**

| call | state |
|---|---|
| `PK_CURVE_eval`, `PK_SURF_eval` (position + 1st derivatives) | ✅ |
| the same, higher order + the `triangular` packing flag | ✅ layout measured against a torus; both packings cross-checked |
| `PK_CURVE_eval_with_tangent` | ✅ exercised via `oracle::sample_curve` |
| `PK_CURVE_eval_handed` | ✅ `Curve::eval_jet_handed`; both `PK_HAND_*` tokens confirmed |
| `PK_CURVE_eval_curvature`, `PK_SURF_eval_curvature` | ✅ sign convention pinned on sphere/cylinder/torus |
| `PK_CURVE_find_min_radius`, `PK_SURF_find_min_radii` | ✅ incl. the no-minimum and signed-radius cases |
| `PK_SURF_eval_with_normal` | ⚠ mesh-specific — averages mvertex normals, not for analytic surfaces (normals come from `PK_SURF_eval` first derivatives, u @ p[1], v @ p[2] — validated on a sphere) |

**What practice must produce:** the exact derivative array layout at order ≥ 2
in both rectangular and triangular packing; curvature sign and principal
direction convention; and behaviour at poles/apexes/zero-curvature points, where
the distinction between *singular* and *zero* has to be a type, not a magnitude
test.

**Verdict.** The derivative table is `index = j*(n_u+1) + i` — **u varies
fastest** — measured against a torus whose mixed partials are all nonzero, so no
ordering could hide. The `triangular` packing is the same ordering with each row
truncated to `i+j <= n`. Wrapped as `SurfJet` / `CurveJet`, indexed by `(i,j)`
rather than raw slot, with out-of-table requests returning `None` instead of a
plausible neighbour.

Three findings CADabra must encode:

1. **Curvature sign:** the normal is outward and a sphere's principal curvatures
   are `+1/r`, so **positive means bending away from the normal**. `k1` pairs
   with `direction_1` — the pairing is the meaning, not any magnitude order.
2. **Parametric singularity ≠ geometric singularity.** At a sphere pole `∂R/∂u`
   vanishes and no normal exists, yet `PK_SURF_eval` **succeeds without error**
   and `eval_curvature` still returns the correct `1/r`. `SurfJet::unit_normal`
   returns `None`; that is a geometric statement, not a failure. A cone apex is
   singular in both senses.
3. **Absent answers are answers.** A straight line reports `n_radii = 0` — not
   infinity, not an error — and a torus reports two **signed** radii (`3.5`,
   `−1.5`), so taking absolute values loses information.

Tests: `stage3_surf_jet_layout_rectangular`, `stage3_surf_jet_layout_triangular`,
`stage3_curve_jet_orders`, `stage3_curvature_sign_convention`,
`stage3_singularity_is_a_type_not_a_magnitude`,
`stage3_min_radius_of_curvature`, `stage3_handed_evaluation`.

---

## Stage 4 — Domains: intervals, uv-boxes, periodicity, seams ✅ **COMPLETE** (2026-08-09)

**Building:** the parameter-domain type and periodic arithmetic. This is where
seam and pole conventions get fixed, and they are hard to change later.

**The architectural question:** how is a periodic domain represented — wrapped
interval, quotient, or explicit seam curve? What is the identity of a point on a
seam?

**Oracle calls:**

| call | state |
|---|---|
| `PK_SURF_ask_uvbox` | ✅ cylinder u∈[0,2π]/v unbounded; sphere v∈[−π/2,π/2]; torus v∈[−π,π] |
| `PK_SURF_ask_params` / `PK_CURVE_ask_param` (`PK_PARAM_sf_t`) | ✅ all six fields decoded incl. the three that were in no catalog |
| `PK_CURVE_ask_interval` | ✅ `Curve::interval` |
| `PK_CURVE_find_length` | ✅ incl. its **conservative enclosure** (`length_with_bounds`) |
| `PK_FACE_find_uvbox`, `_is_periodic`, `_is_uvbox` | ✅ padding measured; one-sided guarantee recorded; seamed case preserved |

**What practice must produce:** period value *and* periodicity class per
direction; closed-vs-periodic distinction for curves; the pole degeneracy
representation; and whether a face's uv-box is guaranteed tight or merely
conservative.

**Verdict.** A seam **is** an exact identification: across `u → u + period`,
position *and* every first derivative agree to ~1e-15 on cylinder, sphere and
torus. Position agreement alone would still permit a kink; derivative agreement
is what licenses treating the domain as a quotient — so a periodic-shift
canonicalization is sound.

But a **pole is not a seam**. At a sphere's `v` extremes the whole `u` fibre
collapses to one point and `|∂R/∂u|` vanishes. The domain type cannot be just
"wrapped interval": identification and fibre-collapse are distinct boundary
phenomena and need distinct representation.

Three further findings:

1. **`PK_PARAM_sf_t`'s undecoded fields are decoded.** `extent`/`form` map from
   an internal 0..3 code (18000 unbounded / 18003 periodic / 18004 bounded);
   `periodic` is *derived* from `extent`, not independent. The field previously
   called `convexity` is **not one** — it is the underlying iso-curve's class
   tag, renamed `curve_class`. Tokens 18001/18002 are reachable but were never
   produced, so they stay unnamed rather than guessed.
2. **Face uv-boxes are conservative, not tight.** A disc face of radius 2
   reports `[-2.024, 2.024]²` — padded ~1.2%. Safe for exclusion, wrong for
   anything needing tightness. `PK_FACE_is_uvbox` is explicitly **one-sided**:
   `true` is trustworthy, `false` only means "not established".
3. **Arc length carries an enclosure.** A circle's length range has width
   exactly 0; an ellipse's is 3.06e-6 wide. The old `length()` discarded it.

Tests: `stage4_param_record_per_family`,
`stage4_seam_is_an_exact_identification`, `stage4_pole_collapses_the_u_fibre`,
`stage4_face_uvbox_is_conservative`,
`stage4_face_periodicity_keeps_the_seamed_case`,
`stage4_arc_length_carries_an_enclosure`.

---

## Stage 5 — Inversion, projection, distance, extrema ✅ **COMPLETE** (2026-08-09)

**Building:** the result contract that every later algorithm reuses.
Unique / multiple / singular / boundary / indeterminate must be *distinct* — this
is the stage where that gets forced, because iterative convergence never proves
uniqueness.

**The architectural question:** what does a query return? An `Option<tuple>` here
poisons SSI, checking and Booleans later. Also: what is a hint, and what
guarantees survive a stale one?

**Oracle calls:**

| call | state |
|---|---|
| `PK_SURF_parameterise_vector`, `PK_CURVE_parameterise_vector` | ✅ strict **inversion** — off-surface points are refused, not projected |
| `PK_TOPOL_range`, `_range_vector` | ✅ status + full witness (`sub_entity`, parameters) now surfaced |
| `PK_ENTITY_range` (multi-solution) | ◐ result structs recovered from the journal; **not runtime-validated** |
| `PK_BODY_find_extreme`, `PK_FACE_find_extreme`, `PK_EDGE_find_extreme` | ✅ body form added, witness topology asserted |
| `PK_TOPOL_clash` | ✅ **unblocked** — was never a frustrum problem; options struct + mandatory transform arrays were wrong |
| `PK_CURVE_project` | ⏸ moved to Stage 12 — it is curve-onto-body imprinting with tracking, not point inversion |

**What practice must produce:** how many solutions each call actually returns for
one-solution / two-solution / periodic-equivalent / boundary / no-solution
inputs; whether a nearest-point answer is global or local; and whether hints
change the answer or only the work.

**Verdict.** The result contract now carries what the kernel actually says:
`RangeStatus` plus a per-end `RangeWitness` (`entity`, `sub_entity`, `position`,
`parameters`). On a block, a probe witnesses a FACE, an EDGE or a VERTEX
depending only on where it sits — information the old `{distance, point, point}`
result discarded entirely.

Four findings CADabra must encode:

1. **`Found` never means unique.** A point on a cylinder's axis is equidistant
   from the whole wall; the kernel still answers `Found` with one arbitrary
   (deterministic) witness. Multiplicity is not in the result and must not be
   inferred from it.
2. **Inversion is not projection.** Off-surface points are refused with
   `PK_ERROR_not_on_surface` / `_not_on_curve`, and a sphere's centre is refused
   too. A seam point inverts to one representative, so periodic canonicalization
   stays the caller's job.
3. **`PK_TOPOL_clash` was never frustrum-blocked** — see below.
4. **Extremes name their witness**: three independent directions pin a vertex;
   fewer leave an edge or face extremal, and the returned topology says which.

**The clash story is the cautionary one.** It had been written off as "needs a
fuller frustrum" on the strength of a mild 9999. With Stage 0's error work the
kernel names the actual fault — *bad argument #3, `tf1`* — and there were two
real bugs: the transform arrays are mandatory (entries may be null, the pointer
may not), and `PK_TOPOL_clash_o_t` was missing three leading fields and modelled
1-byte logicals as ints. `PK_TOPOL_clash_t` was a bare `c_int`, so results were
unreadable. All recovered from the RE project; the classification tokens turned
out to be **7 / 4 / 2**, not the 0..4 the bindings claimed.

Tests: `stage5_range_carries_status_and_witness`,
`stage5_found_does_not_mean_unique`,
`stage5_inversion_requires_the_point_to_lie_on_it`,
`stage5_seam_point_inverts_to_one_representative`,
`stage5_find_extreme_names_its_witness`,
`stage5_clash_classifies_configurations`.

---

## Stage 6 — Ranges and conservative enclosures ✅ **COMPLETE** (2026-08-09)

**Building:** `CertifiedRange` — the enclosure contract that later powers
exclusion tests and BVH pruning.

**The architectural question:** is an enclosure guaranteed conservative, and is
tightness ever promised? A single inward box invalidates every pruning decision
built on it.

**Oracle calls:**

| call | state |
|---|---|
| `PK_TOPOL_find_box` | ✅ 2-arg, no options — the options form is `_find_box_2` |
| `PK_CURVE_find_box`, `PK_SURF_find_box` | ✅ option structs recovered (were opaque stubs); tightness measured |
| `PK_CURVE_find_non_aligned_box`, `PK_SURF_find_non_aligned_box` | ✅ `dimension` + **half**-widths |
| `PK_GEOM_range_vector` | ✅ options were mis-ordered; now a validated **global projection** |
| `PK_GEOM_range` | ◐ 240-byte layout recovered and asserted; not yet exercised |
| `PK_TOPOL_range`, `_range_vector` | ✅ (Stage 5) — layouts verified correct against the decompile |
| `_range_array`, `_range_array_vector` | ✗ deferred — single-global-answer forms, no case needs them yet |

**What practice must produce:** conservativeness verdict per call, allocation
ownership for the array forms, and behaviour of oriented boxes under transform
(does the box transform, or is it recomputed?).

**Verdict — the enclosures are tight, not padded.** Measured against
analytically exact boxes (sphere, torus, cylinder, block), the 3-D box finders
return the exact extent with slack 0.0. That is the *opposite* of the
parameter-space `Face::uvbox` from Stage 4 (padded ~1.2%); the two must not be
reasoned about together.

**Tightness removes the safety margin.** A quarter arc touching x=0 exactly
reports `min.x = +1.8e-16` — **inward by one ULP**. Exclusion tests must use a
tolerance; the box is not a guaranteed superset at the bit level. An unbounded
carrier is refused outright (`PK_ERROR_unsuitable_entity`) rather than
approximated.

Two further findings: `widths` from the non-aligned box finders are
**half-widths** despite the reference calling them widths (a radius-3 circle
reports 3.0, not 6.0), and `dimension` correctly reports 1 / 2 / 3 for
line / planar circle / sphere.

**Ghidra found five wrong struct layouts here**, the most dangerous being
`PK_range_bound_t` with its **upper and lower bounds swapped** — a caller
setting a lower bound would have set an upper one, invisible because every
existing call passes zeroed defaults. Also recovered: `PK_range_param_bound_t`
(40-byte tagged union, not a 48-byte pair), both `PK_GEOM_range*_o_t` field
orders, and the two `find_box` option structs that were opaque stubs.
`PK_TOPOL_range_vector_o_t` was checked and is correct — which is what makes the
GEOM discrepancies credible rather than a decoding error.

**A general lesson worth carrying forward:** a `Default` that zeroes an options
struct is wrong whenever it contains enum fields. `PK_GEOM_range_vector`
rejected a zeroed struct with `field_of_wrong_type` because **0 is not a legal
token** for `guess_type` or `opt_level`.

Tests: `stage6_boxes_are_tight_not_padded`,
`stage6_tight_boxes_can_be_one_ulp_inward`,
`stage6_unbounded_surface_needs_a_restriction`,
`stage6_oriented_box_reports_dimension_and_half_widths`,
`stage6_geom_range_is_a_global_projection`.

---

## Stage 7 — Surface/surface intersection

**Building:** CADabra's central algorithm, on analytic pairs first.

**The architectural question:** how does surface-pair dispatch work without a
scatter of switches, and where does branch-completeness evidence live?

**Oracle calls:**

| call | state |
|---|---|
| `PK_SURF_intersect_surf` | ◐ signature fixed to the 6-output form `(n_vectors, vectors, n_curves, curves, bounds, types)`; two pairs validated |
| `PK_SURF_intersect_curve`, `PK_CURVE_intersect_curve` | ✅ |
| `PK_FACE_intersect_curve`/`_face`/`_surf` | ✅ |
| `PK_intersect_curve_t` token decode | ◐ 14651 transversal / 14652 tangential confirmed; rest of band unprobed |

**What practice must produce:** the full analytic pair matrix — sphere-sphere,
cyl-cyl (parallel / skew / equal-radius), cone-plane conic ladder, torus-plane
including Villarceau — each with its coincident, tangent and disjoint strata;
plus the `bounds` array correspondence and isolated-point handling. Coincident
planes yielding *no* data and tangent spheres yielding a single point are already
confirmed and are the shape of the answer we want everywhere else.

**Done when:** branches match as unordered semantic sets with explicit gauge
normalization, and a missing branch is not rescued by matching samples.

---

## Stage 8 — Pcurves and bounded restriction

**Building:** the chart-side representation and restriction of a carrier curve to
finite faces.

**The architectural question:** who owns a pcurve — the fin, the edge, or the
face? And what is the lift residual contract?

**Oracle calls:** `PK_CURVE_make_spcurves_2` ✗, `PK_CURVE_embed_in_surf_2` ✗,
`PK_FIN_ask_geometry` ◐, `PK_FIN_find_surf_parameters` ✗,
`_find_curve_parameter` ✗, `_find_interval` ✗, `_find_uvbox` ✗.

**What practice must produce:** spcurve count and parameter correspondence for a
supplied known 3-D curve; orientation/sense; seam, winding and pole events; lift
residual.

**Done when:** a wrong-chart or wrong-sense pcurve is detected, and bounded
coverage is complete rather than sampled.

---

## Stage 9 — Freeform and tolerant geometry

**Building:** the authority model — exact vs certified-approximate — and tolerant
edges. Deliberately *after* the analytic path is solid, so the tolerant lane
never becomes a fallback for an unsupported analytic case.

**Oracle calls:** `PK_BCURVE_create`/`_ask`, `PK_BSURF_create`/`_ask`,
`PK_CURVE_make_bcurve_2`, `PK_SURF_make_bsurf_2` — all ✗ (`bgeom.rs` is 66
functions, entirely unaudited). Tolerant topology: `PK_EDGE_ask_geometry_nmnl`,
`_ask_curve_nmnl`, `_ask_precision`, `PK_VERTEX_ask_precision`,
`PK_EDGE_find_deviation_2`, `_set_precision_2`, `_reset_precision_2`,
`_optimise`, `PK_BODY_{ask,set}_curve_nmnl_state` — all ✗.
`PK_CURVE_make_approx` / `PK_FACE_set_approx` only after live probing.

**Done when:** the precision hierarchy (vertex ≥ incident edge) is observable,
and an under-reported deviation or transplanted certificate is caught by an
independent checker that does not trust a serialized `certified: true` bit.

---

## Stage 10 — Topology spine, body classes, checking

**Building:** the incidence model. Mostly done for the regular solid case; the
remainder is general/non-manifold bodies and the checkers.

**Oracle calls:** the spine (`BODY_ask_faces/edges/vertices/shells/regions`,
`FACE_ask_loops`, `LOOP_ask_fins`, fin navigation, `FACE_ask_oriented_surf`,
`SHELL_find_sign`, `EDGE_ask_convexity`) is ✅ — the full adjacency matrix on the
two-region box is asserted, `PK_LOOP_type_t` corrected to the 5410..5419 band.
Remaining: `PK_EDGE_type_t` / `PK_VERTEX_type_t` / `PK_SHELL_type_t` enums ✗;
`PK_BODY_type_t` acorn/empty/general/compound still `[guess]`; non-topology
`PK_CLASS_t` values `[guess]`; `PK_BODY_is_cellular` ✗;
`PK_REGION_ask_regions_adjacent` ✗. Checking: `PK_BODY_check` ◐,
`PK_FACE_check`, `_check_pair`, `PK_EDGE_check`, `PK_GEOM_check` ✗.

**Done when:** wire / sheet / solid / acorn / non-manifold all traverse
correctly under an oblique transform, and checker output carries *requested vs
achieved coverage* rather than a fault list alone.

---

## Stage 11 — Euler operations and reversible edits

**Building:** topology edits from below — one at a time, each with typed
preconditions, an inverse, and a local checker. Primitives get rebuilt on this
path so construction and later operations share it.

**Oracle calls:** all 18 Euler symbols are bound and none are wrapped. Order:
`PK_EDGE_euler_split` ↔ `PK_VERTEX_euler_merge_edges` (the inverse pair) →
`PK_LOOP_euler_make_edge{,_loop,_face}` / `PK_EDGE_euler_delete_make_loop` →
`PK_FACE_euler_make_{loop,ring_loop,ring_face}` → slit / zip →
`PK_FIN_euler_glue`, `PK_LOOP_euler_transfer`.

Rollback arrives here as a dependency, not a feature: `PK_MARK_create_2`,
`_goto_2`, `_delete_2`, `PK_MARK_ask_state`. Each edit runs inside a mark with
the affected neighbourhood traversed before and after.

**Done when:** every edit has an observed inverse restoring semantic incidence
exactly, and a partial mutation is detected.

---

## Stage 12 — Imprint, arrangement, tracking

**Oracle calls:** `PK_FACE_imprint_curve`, `_imprint_curves_2`,
`_imprint_faces_2`, `PK_BODY_imprint_body`, `_imprint_plane_2`,
`PK_FACE_imprint_point`, `PK_EDGE_imprint_point` — all ✗; `PK_imprint_r_t` and
`PK_ENTITY_track_r_f` unbound; `PK_TOPOL_track_r_f` bound, unwrapped;
`PK_BODY_imprint_o_t` and `_imprint_plane_o_t` need materializing (24-byte layout
already computed in `parasolid-re/catalog/pk-option-structs.md`).

The interim shape used by `extrude`/`hollow` — mutate in place, zeroed result
buffer, observe via topology deltas — is a legitimate *first* artifact but does
not finish the stage: the arrangement contract requires source provenance for
every output vertex/edge/cell, which means real tracking.

---

## Stage 13 — Booleans

Comparators supplied by Stages 5, 6, 10, 12. `PK_BODY_boolean_2` is already
unblocked and volume-validated (v2 user struct, 32 bytes), and its result is now
freed through `PK_boolean_r_f` rather than leaking everything but `bodies`.
`PK_BODY_{unite,subtract,intersect}_bodies` ✅ and `PK_BODY_disjoin` ✅ serve as
semantic cross-checks against `PK_BODY_boolean_2` ⚠ once its options are mapped.
`PK_TOPOL_eval_mass_props` ✅ (validated against closed-form volume, centroid,
inertia and periphery for block/sphere/cylinder/cone/torus) and
`PK_BODY_contains_vector` ✅ (enclosure tokens inside=5701 / outside=5702 /
on=5703) are the invariants. `PK_FACE_boolean_2` last.

---

## Stage 14 — Persistence, identity, metadata

XT round-trip is ✅ end-to-end (`oracle_xt_roundtrip_preserves_model`), but the
option and schema surface is not: text vs neutral-binary selection, `FFCSCH`
schema handling for the older XT versions SOLIDWORKS ships, partition-level
transmit/receive and deltas, and `PK_ENTITY_ask_identifier` /
`PK_PART_find_entity_by_ident` / `_rectify_identifiers` for identifier survival.
Attributes, groups and assemblies (35 symbols, 33 unwrapped) are a later slice —
run small probes only to confirm the identity model does not preclude them.

---

## Stage 15 — Faceting and hybrid representations

Last. The faceting path currently reports topology counts without usable
vertices and accepts mild errors; `PK_MESH_create_from_facets` construction is
blocked on PSM 5241. `PK_BODY_create_implicit` and `PK_LATTICE_*` are
representation-boundary probes, not display comparisons — defer until the body
algebra question is actually being asked.

---

## Running rules

- **A stage is not done because the calls return `PK_ERROR_no_errors`.** It is
  done when a deliberately corrupted field in the adapter or comparator turns
  the case non-green. Every stage owes at least one such calibration.
- **Bind the `_r_f` with the call, not later.**
- **Clear the enum guesses the stage depends on, and only those.** Token *bands*
  are in `parasolid-re/catalog/pk-enum-tokens.tsv`; individual members mostly
  are not. Grep `[guess]` / `[unknown]` in `parasolid-sys` before trusting a
  constant.
- **Record per symbol in `docs/audit-ledger.tsv`** using the queue's record
  fields, and reconcile grades with `parasolid-re/catalog/pk-signatures.tsv` in
  both directions — a runtime observation upgrades the RE catalog, a decompile
  informs the binding. Conflicts stay in the row; they are not resolved by
  recency. For scale: 1079 of 1169 catalogued prototypes are grade `documented`
  and exactly 3 are `dynamic-observed`. "Bound" means "typed, untested" — the
  crate has 1155 `extern` declarations and roughly 40 validated symbols.
- **Split `crates/parasolid-test/src/main.rs`** (3.9k lines, 133 cases) into
  per-stage suites as Stage 1 lands, keeping the single Wine runner, so each
  stage has a runnable gate instead of a growing file.

## Start here

Stages 0–6 are complete (172 tests). Next is **Stage 7 — surface/surface
intersection**, the highest-value rung and the one CADabra is being rewritten
around.

Two of its seven intersection entry points are already validated; what is open
is the **full analytic pair matrix** — sphere-sphere, cyl-cyl (parallel / skew /
equal-radius), the cone-plane conic ladder, torus-plane including Villarceau —
each with its coincident, tangent and disjoint strata, plus decoding the rest of
the `PK_intersect_curve_t` band (14651 transversal and 14652 tangential are
confirmed) and the `bounds` array correspondence.

Everything Stage 7 needs is now in place: oblique placement (Stage 2) so the
fixtures are not axis-aligned, jets (Stage 3) for two-sided residuals, seam and
pole semantics (Stage 4) for branch matching, the multiplicity contract
(Stage 5), and enclosures (Stage 6) for candidate rejection.

Carried-forward debts: the `PK_ENTITY_range` multi-solution structs are
decompile-derived but unvalidated; `PK_GEOM_range` has a recovered layout but no
exercise; the `_array` range forms are deferred; and `PK_CURVE_project` belongs
to Stage 12.
