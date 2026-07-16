# Parasolid high-level API — unexercised-function ontology

Design map for the next round of high-level types. **169 of 1154** PK functions are wrapped today; the **985** unwrapped functions below are grouped into proposed types, each method mapped to its `PK_*` call, prioritized against the CADabra oracle roadmap (P0 = needed now → P3 = defer), with option-struct blockers flagged.

## Build order (cross-domain priority)

### P0 — build next (oracle-critical)

- **Body (extend) — Imprint / arrangement** (extend Body, _Body & Part modeling operations_) ⛔ — Imprint two bodies (or a plane) on each other so their intersection is recorded as shared edges/split faces — the precursor to CADabra's imprint->arrangement->classification->commit boolean path. This is the top remaining P0 gap in the domain (booleans themselves are already validated).
- **Face (extend) — Imprint** (extend Face, _Body & Part modeling operations_) ⛔ — Face-set and point-level imprints — the finer-grained arrangement primitives CADabra needs to imprint one face-set onto another and to split edges/faces at exact points.
- **Face (extend)** (extend Face, _Topology & B-rep spine_) ⛔ — Complete face interrogation for the B-rep parity oracle: face type, secondary navigation (first loop, next-in-body, owning shells), UV-box/periodicity for surface-param comparison, coincidence classification for the arrangement oracle, and point-imprint. Directly fills the open P3 'full adjacency matrix' and P4 coincidence items.
- **Edge (extend)** (extend Edge, _Topology & B-rep spine_) ⛔ — Finish edge interrogation: edge type, secondary navigation (first fin, next-in-body, owning shells), oriented curve, planarity, end tangents, G1 chains, and precision — the geometric detail the spine oracle compares edge-by-edge. Adds point-imprint and wire-body construction used by the arrangement/interchange oracle.
- **Vertex (extend)** (extend Vertex, _Topology & B-rep spine_) ⛔ — Complete vertex interrogation: vertex type (isolated/acorn/normal), owning shells, isolated loops, and tolerant-vertex precision. Small, no option structs — rounds out the bottom of the spine.
- **Loop (extend)** (extend Loop, _Topology & B-rep spine_) — Add the loop-level navigation missing from the spine: direct edges/vertices of a loop, first fin, next-loop-in-face iteration, owning body, and isolated-loop test. Needed so a CADabra loop can be diffed against the oracle without walking fins manually.
- **Fin (extend)** (extend Fin, _Topology & B-rep spine_) — Fins are the half-edge layer where CADabra's orientation/parameterisation lives; only next/prev-in-loop and edge/loop/face are wrapped. Add type, body, the oriented curve + surface/curve parameter maps, and the fin's UV-box/interval — the payload the SSI-to-B-rep and arrangement oracles compare.
- **Shell (extend)** (extend Shell, _Topology & B-rep spine_) — Round out the shell layer: shell type (solid/void/wire boundary), owning body, the acorn vertex for a vertex-only shell, wireframe edges, and the shell sign (whether it bounds material inside or outside). Needed for the two-region/two-shell box adjacency oracle (open P3 item).
- **Region (extend)** (extend Region, _Topology & B-rep spine_) ⛔ — Regions are the volumetric cells the regularized-boolean classify/commit stage operates on. Add region type + owning body + region adjacency (the cell graph), the solid<->void material flip, and curve/point imprint into a region. Directly serves the P0 'imprint -> arrangement -> classification -> commit' capability.
- **Entity / Topol (extend)** (extend Entity, _Topology & B-rep spine_) ⛔ — PK_TOPOL_* and PK_ENTITY_* act on any topological tag, so they belong on Entity (with Body conveniences). This bundles the oracle's cheap-but-high-value invariants: point->body and body->body distance (open P5 item), oriented/transformed and non-axis-aligned bounding boxes, clash detection (open P5 item), connectivity, geometry categorisation, plus entity identity/user-data and redundant-topology cleanup and general-body construction.

### P1 — soon

- **Body (extend) — Section geometry (arrangement curves)** (extend Body, _Body & Part modeling operations_) — Produce the SECTION CURVES/WIRES from cutting bodies with a plane, surface, or sheet WITHOUT splitting the solid — the direct oracle for CADabra's arrangement/section output (complements the already-wrapped section_with_surf which splits in place).
- **Body (extend) — Revolve & sweep (feature creation)** (extend Body, _Body & Part modeling operations_) ⛔ — Generative feature builders beyond the already-wrapped linear extrude: revolve (spin) a profile into a solid of revolution, translational sweep, general path-sweep, and loft. Revolve is the specific 'make_spun_body' gap the docs flag as a TODO.
- **Body (extend) — Boolean completion & general-body handling** (extend Body, _Body & Part modeling operations_) ⛔ — Finish the boolean surface now that unite/subtract/intersect are validated: split multi-lump results, bind the full result-free, and add face-level (local) booleans.
- **Face imprint & sheet/solid construction (extend Face)** (extend Face, _Topology & B-rep spine_) ⛔ — The imprint->arrangement half of the boolean oracle plus wrapping faces as bodies for interrogation. Separated from core Face interrogation because these are model-mutating and several carry option/result structs of varying readiness.
- **Transform (extend)** (extend Transform, _Geometry_) ⛔ — Native constructors and algebra for PK_TRANSF entities so the oracle can place any primitive at an arbitrary pose and validate CADabra's frame/coordinate transforms exactly. Today only from_matrix/translation/uniform_scale/ask exist; the native rotation/reflection/scale-about-centre constructors, composition, equality and classification are all unexercised. Every constructor takes plain Vec3 args (no option struct) so the core set is unblocked.
- **Vec3 / vector free functions (extend geom.rs)** (extend Vec3, _Geometry_) ⛔ — Apply transforms to raw vectors and do the small vector-algebra kernels CADabra's frame math relies on. All are pure, no option struct except the lsq-plane fit.
- **Surf (extend) — curvature, conversion, extra analytic types** (extend Surf, _Geometry_) ⛔ — Finish the surface oracle: exact principal-curvature evaluation (the next eval primitive after position/normal, and the signal SSI tangency classification leans on), analytic→NURBS conversion for cross-checking CADabra NURBS, and the remaining analytic surface constructors (spun/swept/offset) with create→ask round-trips.
- **Curve (extend) — interval, periodicity, curvature, wrap-as-body, conversion** (extend Curve, _Geometry_) ⛔ — Complete the curve oracle: parametric interval and closed/periodic classification (TODO P2), exact curvature (radius/normal), wrapping an orphan curve as a wire body for interrogation (TODO P1), and analytic→NURBS conversion.
- **Geom lifecycle (extend Surf/Curve/Point + Body)** (extend Surf, _Geometry_) ⛔ — Manage orphan geometry: transform a bare surf/curve/point to a new pose (feeds the SSI oracle at arbitrary placements without building a body), copy, delete a single orphan geom, scale, and attach/detach orphan geoms to a part. PK_GEOM_* operate on any geometry tag, so these mirror onto Curve and Point too.
- **Part (data-exchange + introspection)** (new type, _Infrastructure & data_) — A new-type over PK_PART_t (== PK_ENTITY_t; Body and Assembly are Parts) that owns the whole-model XT interchange oracle (TODO P6) plus part-level geometry/attribute/identifier introspection. This is the mechanism by which a CADabra-built body and a Parasolid-built body get compared as whole models, and by which stable identifiers survive a write→read round-trip.


---

## Body & Part modeling operations (create / edit / boolean / sweep / blend / offset / section / imprint)

_Regularized booleans, primitives, extrude, section-split, offset/hollow, and constant-radius fillet are already wrapped and validated; the next round's highest-value, oracle-relevant gap is the imprint -> arrangement stage (CADabra Phase E) plus the section-CURVE and revolve/general-sweep feature builders. Most option-struct layouts are computed in the RE catalog ("documented" grade, with boolean_2-style version-migration drift as the residual risk); the real blockers are a handful of opaque [u8;0] result structs (PK_imprint_r_t, PK_BODY_tracked_sweep_r_t/loft_r_t) needed to read created topology._

### [P0] Body (extend) — Imprint / arrangement  (extend `Body`)
Imprint two bodies (or a plane) on each other so their intersection is recorded as shared edges/split faces — the precursor to CADabra's imprint->arrangement->classification->commit boolean path. This is the top remaining P0 gap in the domain (booleans themselves are already validated).

- `fn imprint(self, tool: Body) -> PsResult<(Body, Body)>`
  &nbsp;&nbsp;↳ `PK_BODY_imprint_body`
  &nbsp;&nbsp;· Mutual imprint of target+tool where they intersect; validate against a block+cylinder pair (co-axial) then diff each body's topology (new circular edges / split faces). Options default via a validated PK_BODY_imprint_o_t; results via PK_imprint_r_t (blocked for readback).
- `fn imprint_faces(&self, faces: &[Face]) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_BODY_imprint_faces_2`
  &nbsp;&nbsp;· Imprint selected faces of the body against the rest; PK_BODY_imprint_faces_o_t is field-defined (64 B). Same PK_imprint_r_t readback blocker; tracking freed via PK_TOPOL_track_r_f.
- `fn imprint_plane(&self, plane: &Surf) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_BODY_imprint_plane_2`, `PK_BODY_imprint_plane`
  &nbsp;&nbsp;· Imprint a plane's trace onto the body (splits faces along the cut line without removing material) — a cheap arrangement oracle. Mutates in place + tracking; only needs the 24-byte PK_BODY_imprint_plane_o_t materialized (currently a stub).
- `fn imprint_curves_normal(&self, curves: &[Curve]) -> PsResult<()>  /  fn imprint_curves_vector(&self, curves: &[Curve], dir: Vec3) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_BODY_imprint_curves_normal`, `PK_BODY_imprint_curves_vector`, `PK_BODY_imprint_cus_normal`, `PK_BODY_imprint_cus_vec`, `PK_BODY_imprint_cus_vector`
  &nbsp;&nbsp;· Project/imprint free curves onto the body's faces (normal- or vector-projected). Options computed (imprint_cus_normal_o_t 32 B, imprint_cus_vec_o_t 28 B). P1 once the core body imprint lands.

  **Blocker:** PK_imprint_r_t is an opaque [u8;0] stub in generated_stubs.rs — needed to READ the list of created edges/faces. Sidestep for the first cut like extrude/hollow do: mutate the target in place, back the results arg with a zeroed buffer, free via a to-be-bound PK_imprint_r_f, and observe the effect by re-asking the body's topology (face/edge count deltas). PK_BODY_imprint_o_t is field-defined but carries boolean_2-style o_t_version migration drift — probe accepted o_t_version at runtime before trusting the layout. PK_BODY_imprint_plane_o_t is still an opaque stub (24-byte layout computed in pk-option-structs.md — materialize it).

### [P0] Face (extend) — Imprint  (extend `Face`)
Face-set and point-level imprints — the finer-grained arrangement primitives CADabra needs to imprint one face-set onto another and to split edges/faces at exact points.

- `fn imprint_faces(targets: &[Face], tools: &[Face]) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_FACE_imprint_faces_2`, `PK_FACE_imprint_faces`
  &nbsp;&nbsp;· Imprint a target face-set against a tool face-set (PK_FACE_imprint_faces_o_t is field-defined, 80 B). Core to imprinting the arrangement of two overlapping shells. PK_imprint_r_t readback blocked; tracking freeable.
- `fn imprint_curves(&self, curves: &[Curve], intervals: &[(f64,f64)]) -> PsResult<Vec<Edge>>`
  &nbsp;&nbsp;↳ `PK_FACE_imprint_curves_2`
  &nbsp;&nbsp;· Multi-curve generalization of the already-wrapped single-curve imprint; PK_FACE_imprint_curves_o_t is tiny (8 B) and uses PK_ENTITY_track_r_t (field-defined, readable) rather than the opaque imprint_r_t — so created edges ARE recoverable. Strong P1 win.
- `fn imprint_point(&self, point: &Point) -> PsResult<(Vertex, Face)>  (+ Edge::imprint_point, Region::imprint_point)`
  &nbsp;&nbsp;↳ `PK_FACE_imprint_point`, `PK_EDGE_imprint_point`, `PK_REGION_imprint_point`
  &nbsp;&nbsp;· Split a face/edge at an exact point, returning the new vertex (+ new edge for the edge case). No option/result structs — trivially tractable, good coverage-per-effort. P2.

  **Blocker:** Shared PK_imprint_r_t opaque stub again (readback of created edges/faces). Point-imprint variants (PK_*_imprint_point) return concrete new_vertex/new_edge tags with NO option/result struct, so those are fully tractable now. Face::imprint_curve (single curve) is already wrapped and validated.

### [P1] Body (extend) — Section geometry (arrangement curves)  (extend `Body`)
Produce the SECTION CURVES/WIRES from cutting bodies with a plane, surface, or sheet WITHOUT splitting the solid — the direct oracle for CADabra's arrangement/section output (complements the already-wrapped section_with_surf which splits in place).

- `fn make_section(targets: &[Body], tools: &[Body]) -> PsResult<Vec<Body>>`
  &nbsp;&nbsp;↳ `PK_BODY_make_section`, `PK_BODY_make_section_with_surfs`
  &nbsp;&nbsp;· Returns section wire bodies (curves of intersection) that the oracle can interrogate edge-by-edge and eval — a topology-carrying alternative to raw SSI. Validate: a block sectioned by a plane -> a rectangular wire loop of the expected perimeter. The _with_surfs form takes orphan surfaces instead of tool bodies.
- `fn section_with_sheet(self, sheet: Body) -> PsResult<Vec<Body>>`
  &nbsp;&nbsp;↳ `PK_BODY_section_with_sheet`, `PK_BODY_section_with_sheet_2`, `PK_FACE_section_with_sheet`, `PK_FACE_make_sect_with_sfs`
  &nbsp;&nbsp;· Section a solid with a sheet body (finite cutter) rather than an infinite surface; frees result via PK_BODY_section_with_sheet_r_f. P2 — needs the r_t readback layout confirmed.

### [P1] Body (extend) — Revolve & sweep (feature creation)  (extend `Body`)
Generative feature builders beyond the already-wrapped linear extrude: revolve (spin) a profile into a solid of revolution, translational sweep, general path-sweep, and loft. Revolve is the specific 'make_spun_body' gap the docs flag as a TODO.

- `fn revolve(self, axis: Axis1, angle: f64) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_BODY_spin`, `PK_FACE_spin`, `PK_VERTEX_spin`
  &nbsp;&nbsp;· Revolve a sheet/wire profile about an axis into a solid/sheet of revolution — the missing revolve. No option struct: (body, PK_AXIS1_sf_t axis, angle, local_check, n_laterals, laterals, bases, check_result). Validate: a rectangle profile revolved 2pi about an offset axis -> torus/cylinder of known volume.
- `fn sweep(self, path: Vec3) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_BODY_sweep`, `PK_FACE_sweep`, `PK_VERTEX_sweep`
  &nbsp;&nbsp;· Translational local sweep of a body by a vector (distinct from PK_BODY_extrude's profile->solid; this drags existing topology). Option-free; returns lateral topols. P2.
- `fn make_swept_body(profile: Body, path: Body, start: Vertex) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_BODY_make_swept_body`, `PK_BODY_make_swept_body_2`, `PK_BODY_make_swept_profiles`
  &nbsp;&nbsp;· General sweep of a profile along an arbitrary path/wire body. BLOCKED on PK_BODY_tracked_sweep_r_t layout to recover the swept body tag. P2.
- `fn loft(profiles: &[Body]) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_BODY_make_lofted_body`
  &nbsp;&nbsp;· Loft a solid/sheet through ordered profile bodies. BLOCKED on PK_BODY_tracked_loft_r_t layout. P2.

  **Blocker:** spin/sweep are OPTION-FREE and return lateral/base PK_TOPOL_t arrays (freeable) — fully tractable now; spin needs a PK_AXIS1_sf_t (already defined). make_swept_body and make_lofted_body are BLOCKED: their outputs come back only through opaque [u8;0] stubs PK_BODY_tracked_sweep_r_t / PK_BODY_tracked_loft_r_t (no way to extract the resulting body tag until those layouts are RE'd). Their option structs are computed (make_swept_body_o_t 96 B / _2 312 B, make_lofted_body_o_t 120 B).

### [P1] Body (extend) — Boolean completion & general-body handling  (extend `Body`)
Finish the boolean surface now that unite/subtract/intersect are validated: split multi-lump results, bind the full result-free, and add face-level (local) booleans.

- `fn disjoin(self) -> PsResult<Vec<Body>>`
  &nbsp;&nbsp;↳ `PK_BODY_disjoin`
  &nbsp;&nbsp;· Split a disconnected/general body (e.g. a boolean that produced multiple lumps, or general_topology output) into separate bodies. (body, n_bodies, bodies) — no options. Pairs naturally with Session::general_topology(true).
- `fn boolean_faces(&self, faces: &[Face], tools: &[Face], op: BooleanOp) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_FACE_boolean_2`, `PK_FACE_boolean`
  &nbsp;&nbsp;· Local (face-set) boolean — Parasolid's 'partial boolean' used for feature-level cut/join without whole-body regularization. P2.
- `(free-function binding, no new method) — PK_boolean_r_f / PK_imprint_r_f`
  &nbsp;&nbsp;↳ `PK_boolean_r_f`
  &nbsp;&nbsp;· Bind and call the result-free functions so boolean/imprint result structs are fully released, not just the bodies array. Small engineering task, not a user-facing method.

  **Blocker:** disjoin is OPTION-FREE and returns a PK_BODY_t array — trivially tractable now. Engineering follow-up: bind PK_boolean_r_f / PK_imprint_r_f so the whole result struct is freed (docs note only `bodies` is freed today). Face-level boolean uses PK_face_boolean_o_t (computed, 152 B) with the same version-migration-drift caution as body boolean.

### [P2] Body (extend) — Sheet assembly, healing & thickening  (extend `Body`)
Turn collections of sheets into solids and repair/close them — sew, knit, thicken (sheet->solid), extend, trim, fill-hole. Useful for building oracle test bodies from surface patches and for the sewing/healing lane.

- `fn sew(bodies: Vec<Body>) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_BODY_sew_bodies`
  &nbsp;&nbsp;· Sew coincident-edge sheet bodies into one body. (n_bodies, bodies, result_body) — no options, no result struct. Cheapest win in the group.
- `fn thicken(self, front: f64, back: f64, tol: f64) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_BODY_thicken_2`, `PK_BODY_thicken_3`, `PK_BODY_thicken`
  &nbsp;&nbsp;· Offset a sheet body to both sides into a solid slab. Result struct field-defined; validate a planar sheet thickened symmetrically -> a slab of known volume.
- `fn knit(&self, topols: &[Entity], matches: &[Entity]) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_BODY_knit`
  &nbsp;&nbsp;· Knit specified free edges/faces of a sheet body together. Mutates in place + tracking; 8-byte option. P2.
- `fn extend(&self, ...) -> PsResult<()>  /  fn trim(self, ...) -> PsResult<Body>  /  fn fill_hole(&self, ...) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_BODY_extend`, `PK_SURF_extend`, `PK_BODY_trim`, `PK_BODY_trim_gap_analysis`, `PK_BODY_fill_hole`
  &nbsp;&nbsp;· Sheet-body extension/trim/hole-fill healing ops. Option structs computed (extend 72 B, surf_extend 160 B, fill_hole 120 B). P2/P3 — lower oracle value than the above.

  **Blocker:** Mostly tractable: sew_bodies is OPTION-FREE with a direct result_body out (trivial). thicken_2/_3 has a FIELD-DEFINED result (PK_BODY_thicken_r_t) and computed option (thicken_o_t 160 B). knit option is tiny (8 B, field-defined) and mutates in place + tracking. extend option computed (72 B). PK_BODY_knit_result_t is an opaque stub but knit mutates in place (tracking-only path avoids it). trim/fill_hole option structs computed (fill_hole_o_t 120 B) but larger — validate carefully.

### [P2] Edge (extend) — Chamfers & blend variants  (extend `Edge`)
Extend the already-working constant-radius fillet (Body::fillet_edges) with chamfers, variable-radius and chained blends, and blend introspection/removal.

- `fn chamfer(edges: &[Edge], range1: f64, range2: f64) -> PsResult<i32>`
  &nbsp;&nbsp;↳ `PK_EDGE_set_blend_chamfer`, `PK_BODY_fix_blends`
  &nbsp;&nbsp;· Two-phase like fillet: mark the chamfer, then fix_blends. Validate: chamfering one cube edge -> a flat facet of known area, volume = cube - triangular prism. Pass NULL options first.
- `fn set_blend_variable(&self, ...) -> PsResult<()>  /  fn set_blend_chain(edges: &[Edge], ...) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_EDGE_set_blend_variable`, `PK_EDGE_set_blend_chain`
  &nbsp;&nbsp;· Variable-radius and chain blends. P3 — needs the nested PK_blend_properties_t layout for anything beyond defaults.
- `fn blend(&self) -> PsResult<Option<BlendInfo>>  /  fn remove_blend(&self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_EDGE_ask_blend`, `PK_EDGE_remove_blend`, `PK_EDGE_find_blend_topol`
  &nbsp;&nbsp;· Introspect/remove a set (unrealized) blend. P3.

  **Blocker:** PK_EDGE_set_blend_chamfer_o_t (8 B) and PK_EDGE_set_blend_variable_o_t (16 B) are opaque [u8;0] stubs (layouts computed in catalog). Like the validated fillet path, the safest first cut passes NULL options (defaults) because these blend option structs nest PK_blend_properties_t, which the RE-catalog TSV mis-sizes — NULL sidesteps the unreliable offsets. Realization still goes through the already-wrapped PK_BODY_fix_blends.

### [P2] Face (extend) — Local face operations  (extend `Face`)
Direct-editing / local operations on selected faces: offset a face-set, delete faces (heal the gap), cover a boundary into a sheet, extract a face as a standalone sheet body, and pattern faces.

- `fn offset_faces(faces: &[Face], distance: f64) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_FACE_offset_2`, `PK_FACE_offset`
  &nbsp;&nbsp;· Offset a selected face-set (local, unlike whole-body Body::offset). Validate: offsetting one face of a block outward by d -> volume + face_area*d.
- `fn make_sheet_body(&self) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_FACE_make_sheet_body`
  &nbsp;&nbsp;· Extract a single face as an orphan sheet body for isolated interrogation — a useful oracle primitive mirroring Surf::make_sheet_body. Likely option-free/small. P2.
- `fn delete_faces(faces: &[Face]) -> PsResult<()>  /  fn cover(faces: &[Face]) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_FACE_delete_2`, `PK_FACE_delete`, `PK_FACE_cover`, `PK_FACE_delete_from_sheet_body`
  &nbsp;&nbsp;· Delete-and-heal a face-set, or cover a boundary loop into a new sheet face. PK_FACE_delete_o_t stub -> NULL for default heal. P2/P3.
- `fn pattern(faces: &[Face], transforms: &[Transf]) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_FACE_pattern_2`, `PK_FACE_pattern`
  &nbsp;&nbsp;· Replicate a feature (face-set) across a transform pattern. BLOCKED on opaque pattern_2 option+result stubs. P3.
- `fn change(faces: &[Face], ...) / replace_surfs / taper / transform`
  &nbsp;&nbsp;↳ `PK_FACE_change`, `PK_FACE_replace_surfs_2`, `PK_FACE_taper`, `PK_FACE_transform`, `PK_FACE_hollow_2`, `PK_FACE_repair`
  &nbsp;&nbsp;· Tweak-style local mods (move/replace/taper/hollow selected faces). Local-ops lane is rated EXTREME difficulty in the heatmap; defer to P3 unless CADabra needs them.

  **Blocker:** PK_FACE_offset_2 option is field-defined (face_offset_o_t 72 B) — tractable. PK_FACE_delete_o_t is an opaque stub (heal option; NULL likely gives default heal). PK_FACE_make_sheet_body is a clean extractor (good oracle helper). PK_FACE_pattern_2 is BLOCKED: both PK_FACE_pattern_2_o_t and PK_FACE_pattern_2_r_t are opaque [u8;0] stubs.

### [P2] Body (extend) — Whole-body transform, orientation & simplify  (extend `Body`)
Rigid/affine body transforms with options, orientation reversal, uniform enlarge, and geometry simplification — cheap utilities the oracle uses to place/normalize bodies before comparison.

- `fn transform_with(self, t: Transf, opts: TransformOptions) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_BODY_transform_2`
  &nbsp;&nbsp;· Transform with control over geometry sharing / tolerant handling (the plain rigid transform is already wrapped). P2; materialize PK_BODY_transform_o_t or pass NULL.
- `fn reverse_orientation(&self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_BODY_reverse_orientation`
  &nbsp;&nbsp;· Flip a sheet body's sense (inside-out). Cheap, option-free. Useful for orientation-parity oracle checks. P2.
- `fn enlarge(self, factor: f64) -> PsResult<()>  /  fn simplify_geom(&self) -> PsResult<Vec<Surf>>`
  &nbsp;&nbsp;↳ `PK_BODY_enlarge`, `PK_BODY_simplify_geom`
  &nbsp;&nbsp;· Uniformly grow a body / collapse spline geometry back to analytic where possible. P3.

  **Blocker:** PK_BODY_transform (rigid, no options) is already wrapped in the new transform.rs; PK_BODY_transform_2 adds an option struct (transform_o_t is an opaque stub — NULL for default). reverse_orientation / simplify_geom are option-free. enlarge option computed (80 B).

**Deferred groups:**
- _B-curve / B-surface (NURBS) construction & fitting (66)_ — ~66 fns (PK_BCURVE_*, PK_BSURF_*, PK_CURVE_make_bcurve, PK_CURVE_make_spcurves, fit/loft/degree-raise). Off the oracle's near-term path (analytic primitives first, per TODO P1); many option AND result structs are opaque stubs (BCURVE_create_spline_o_t/spline_r_t, BSURF_create_fitted_o_t, etc.). Wrap only once CADabra emits NURBS.
- _Silhouette / outline / helical generators (8)_ — View- or axis-dependent outline curves (PK_BODY_make_curves_outline, make_persp_outline, make_spun_outline, make_swept_tool, PK_CURVE_make_helical_surf, PK_POINT_make_helical_curve). Not a golden-oracle comparison target; several use PK_AXIS1_sf_t + opaque r_t. Defer to P3.
- _Low-level Euler / topology-from-scratch construction (9)_ — Manual B-rep assembly (PK_BODY_create_topology(_2), create_solid_topology, create_sheet_topology, create_wire_topology, create_minimum_topology, PK_TOPOL_make_new(_r_f), PK_TOPOL_make_general_body). Belongs to the topology domain, not the modeling-operation oracle; create_topology_2_o_t/_r_t are opaque stubs. The oracle builds bodies via primitives + booleans + sweeps instead.
- _Convergent / implicit / facet body construction (7)_ — Mesh/lattice/implicit modeling (PK_BODY_create_implicit(_r_f), PK_BODY_make_facet_body, PK_TOPOL_make_facet_topol, PK_FACE_fix_mesh_defects(_r_f), PK_FACE_make_valid_faces). Separate mesh/lattice domain; heatmap 'convergent/implicit/lattice/mesh' is LOWER value for the oracle. Defer.
- _Compound / multi-body container ops (8)_ — Assembly-adjacent grouping (PK_BODY_make_compound, add_to_compound, ask_children/components/parent, remove_from_parents, identify_general). Belongs to the assembly/part-structure domain; make_compound_o_t + ask_children_o_t are opaque stubs. Defer.
- _Mold / mid-surface / neutral-sheet & isocline tooling (8)_ — Specialized mold-design workflow (PK_FACE_imprint_curves_isocline, PK_FACE_imprint_cus_isoclin, install_surfs_isocline, PK_FACE_make_neutral_sheet(_2), PK_BODY_trim_neutral_sheets(_2), FACE_make_valid_faces). No oracle use case; option/result stubs. Defer to P3.
- _Feature recognition / detail & faceset identification (10)_ — Read-oriented analysis rather than construction (PK_BODY_identify_details, identify_facesets, find_facesets(_r_f), is_cellular, is_disjoint, PK_FACE_classify_details(_r_f), PK_FACE_find_blend_unders, identify_blends). Useful later for classification-stage oracling but not needed to validate CADabra output now; several r_t stubs. Defer to P3.
- _Sewing / healing / repair & misc local edits (20)_ — Repair lane and low-value local edits (PK_BODY_repair_shells, PK_EDGE_delete/remove_to_bodies, PK_FACE_repair/replace_with_sheet/remove_to_solid_bodies, PK_BODY_slice(_r_f), PK_BODY_make_patterned, PK_CURVE_project, PK_EDGE_offset_on_body, PK_BODY_offset_planar_wire, PK_REGION_embed_body, PK_BODY_embed_in_surf, PK_BODY_emboss / PK_FACE_emboss, PK_BODY_taper / PK_FACE_taper). 'local operations (tweak/offset/taper)' is rated EXTREME in the difficulty heatmap and off the oracle path; many carry opaque r_t stubs. Defer to P3.


---

## Topology & B-rep spine (Face / Edge / Loop / Fin / Vertex / Shell / Region / Topol / Entity)

_The B-rep spine is half-wrapped: Body/Region/Shell/Face/Loop/Fin/Edge/Vertex adjacency exists but the per-entity TYPE tokens, secondary navigation, fin parameterisation, and the distance/box/clash/classification queries the CADabra oracle needs are all still raw. Of 273 unexercised functions, ~110 are worth wrapping this round on the existing tag-wrapper types (most need no option struct or only a documented-grade one); the remaining ~160 are euler/blend/local-op/facet functions that belong to other domains or are blocked on opaque option/result structs._

### [P0] Face (extend)  (extend `Face`)
Complete face interrogation for the B-rep parity oracle: face type, secondary navigation (first loop, next-in-body, owning shells), UV-box/periodicity for surface-param comparison, coincidence classification for the arrangement oracle, and point-imprint. Directly fills the open P3 'full adjacency matrix' and P4 coincidence items.

- `fn face_type(&self) -> PsResult<FaceType>`
  &nbsp;&nbsp;↳ `PK_FACE_ask_type`
  &nbsp;&nbsp;· New FaceType enum (plane/cyl/cone/sphere/torus/… backing surface class); decode tokens then probe on primitives.
- `fn first_loop(&self) -> PsResult<Option<Loop>>`
  &nbsp;&nbsp;↳ `PK_FACE_ask_first_loop`
  &nbsp;&nbsp;· null tag => None (face with no loops).
- `fn next_in_body(&self) -> PsResult<Option<Face>>`
  &nbsp;&nbsp;↳ `PK_FACE_ask_next_in_body`
  &nbsp;&nbsp;· Body face-list iteration without materialising the whole Vec.
- `fn shells(&self) -> PsResult<Vec<Shell>>`
  &nbsp;&nbsp;↳ `PK_FACE_ask_shells`
  &nbsp;&nbsp;· Closes the Face->Shell up-link (currently only Shell->Face exists).
- `fn uvbox(&self) -> PsResult<UvBox>`
  &nbsp;&nbsp;↳ `PK_FACE_find_uvbox`
  &nbsp;&nbsp;· PK_UVBOX_t is modeled; reuse the UvBox from surf.rs. The trimmed param range of the face vs the full surface uvbox.
- `fn is_periodic(&self) -> PsResult<(bool,bool)>`
  &nbsp;&nbsp;↳ `PK_FACE_is_periodic`
  &nbsp;&nbsp;· (u_periodic, v_periodic).
- `fn is_uvbox(&self) -> PsResult<bool>`
  &nbsp;&nbsp;↳ `PK_FACE_is_uvbox`
  &nbsp;&nbsp;· Whether the face is a simple uvbox-bounded patch.
- `fn contains_points(&self, pts: &[Vec3]) -> PsResult<Vec<Entity>>`
  &nbsp;&nbsp;↳ `PK_FACE_contains_vectors`
  &nbsp;&nbsp;· o_t documented; per-point containing sub-topology. Point-on-face oracle.
- `fn common_edges(&self, other: Face) -> PsResult<Vec<Edge>>`
  &nbsp;&nbsp;↳ `PK_FACE_find_edges_common`
  &nbsp;&nbsp;· Face-face shared-edge oracle (open P3 item).
- `fn extreme(&self, dirs: [Vec3;3]) -> PsResult<(Vec3, Entity)>`
  &nbsp;&nbsp;↳ `PK_FACE_find_extreme`
  &nbsp;&nbsp;· Extreme point of the face in a direction; support-function oracle.
- `fn is_coincident(&self, other: Face, tol: f64) -> PsResult<(FaceCoi, Vec3)>`
  &nbsp;&nbsp;↳ `PK_FACE_is_coincident`
  &nbsp;&nbsp;· PK_FACE_coi_t band 0x5578..0x557f (known). Coincidence/overlap classification the boolean/arrangement oracle needs.
- `fn imprint_point(&self, pos: Vec3) -> PsResult<Vertex>`
  &nbsp;&nbsp;↳ `PK_FACE_imprint_point`
  &nbsp;&nbsp;· No option struct — clean. Vertex-onto-face imprint.
- `fn classify_details(faces: &[Face]) -> PsResult<Vec<FaceClass>>`
  &nbsp;&nbsp;↳ `PK_FACE_classify_details`, `PK_FACE_classify_details_r_f`
  &nbsp;&nbsp;· P2: needs PK_detail_t input struct + PK_FACE_classify_details_r_t layout validated first.
- `fn outer_loop(&self) -> PsResult<Loop>`
  &nbsp;&nbsp;↳ `PK_FACE_find_outer_loop`
  &nbsp;&nbsp;· BLOCKED on opaque o_t; interim: loops().find(Outer).

  **Blocker:** PK_FACE_type_t token values need decoding from pk-enum-values.tsv + a runtime probe (like LOOP_type was). PK_FACE_find_outer_loop and PK_FACE_find_interior_vec are blocked on OPAQUE option structs (PK_FACE_find_outer_loop_o_t, PK_FACE_find_interior_vec_o_t are [u8;0] stubs) — the outer loop is reachable meanwhile via loops()+loop_type()==Outer. PK_FACE_contains_vectors_o_t and PK_FACE_is_coincident_o_t are documented-grade (computed offsets) and can start with NULL defaults. PK_FACE_classify_details needs PK_detail_t + PK_FACE_classify_details_r_t validation (P2).

### [P0] Edge (extend)  (extend `Edge`)
Finish edge interrogation: edge type, secondary navigation (first fin, next-in-body, owning shells), oriented curve, planarity, end tangents, G1 chains, and precision — the geometric detail the spine oracle compares edge-by-edge. Adds point-imprint and wire-body construction used by the arrangement/interchange oracle.

- `fn edge_type(&self) -> PsResult<EdgeType>`
  &nbsp;&nbsp;↳ `PK_EDGE_ask_type`
  &nbsp;&nbsp;· New EdgeType enum; decode + probe.
- `fn oriented_curve(&self) -> PsResult<(Curve, bool)>`
  &nbsp;&nbsp;↳ `PK_EDGE_ask_oriented_curve`
  &nbsp;&nbsp;· Curve + sense of the edge relative to it.
- `fn first_fin(&self) -> PsResult<Fin>`
  &nbsp;&nbsp;↳ `PK_EDGE_ask_first_fin`
  &nbsp;&nbsp;· Cheaper than fins() when only one is needed.
- `fn next_in_body(&self) -> PsResult<Option<Edge>>`
  &nbsp;&nbsp;↳ `PK_EDGE_ask_next_in_body`
  &nbsp;&nbsp;· Body edge-list iteration.
- `fn shells(&self) -> PsResult<Vec<Shell>>`
  &nbsp;&nbsp;↳ `PK_EDGE_ask_shells`
  &nbsp;&nbsp;· Edge->Shell up-link (esp. for wireframe edges).
- `fn is_planar(&self) -> PsResult<(bool, Option<Vec3>)>`
  &nbsp;&nbsp;↳ `PK_EDGE_is_planar`
  &nbsp;&nbsp;· Planarity + plane normal when planar.
- `fn end_tangents(&self) -> PsResult<((Vec3,Vec3),(Vec3,Vec3))>`
  &nbsp;&nbsp;↳ `PK_EDGE_find_end_tangents`
  &nbsp;&nbsp;· ((start_pt,start_tan),(end_pt,end_tan)); no option struct.
- `fn arc_length_interval(&self) -> PsResult<(f64,f64)>`
  &nbsp;&nbsp;↳ `PK_EDGE_find_interval`
  &nbsp;&nbsp;· Distinct from the parametric interval already exposed.
- `fn extreme(&self, dir: Vec3) -> PsResult<(Vec3, Entity)>`
  &nbsp;&nbsp;↳ `PK_EDGE_find_extreme`
  &nbsp;&nbsp;· Extreme point along a direction.
- `fn g1_edges(&self) -> PsResult<Vec<Edge>>`
  &nbsp;&nbsp;↳ `PK_EDGE_find_g1_edges`
  &nbsp;&nbsp;· Tangent-continuous edge chain (feature-edge grouping oracle).
- `fn precision(&self) -> PsResult<f64>`
  &nbsp;&nbsp;↳ `PK_EDGE_ask_precision`
  &nbsp;&nbsp;· Tolerant-edge precision (P9 hygiene).
- `fn set_precision(&self, tol: f64) -> PsResult<()>; fn reset_precision(&self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_EDGE_set_precision`, `PK_EDGE_reset_precision`
  &nbsp;&nbsp;· Base forms take no option struct (P2).
- `fn imprint_point(&self, point: Point) -> PsResult<(Vertex, Edge)>`
  &nbsp;&nbsp;↳ `PK_EDGE_imprint_point`
  &nbsp;&nbsp;· Splits the edge at a point; no option struct. P1 imprint oracle.
- `fn make_wire_body(edges: &[Edge]) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_EDGE_make_wire_body`
  &nbsp;&nbsp;· P1: wrap free edges as a wire body for interrogation. o_t documented; free PK_TOPOL_track_r_t.
- `fn make_faces_from_wire(edges: &[Edge]) -> PsResult<Vec<Face>>`
  &nbsp;&nbsp;↳ `PK_EDGE_make_faces_from_wire`
  &nbsp;&nbsp;· P2: cover a planar wire loop into faces.

  **Blocker:** PK_EDGE_type_t tokens: independently-derived band 0x5c26.. is for convexity; edge_type values need pk-enum-values decode + probe. PK_EDGE_make_wire_body_o_t is documented-grade (NULL default ok); tracking via PK_TOPOL_track_r_t freed with PK_TOPOL_track_r_f (pattern already used in body.rs). set_precision_2/reset_precision_2/find_deviation_2 use OPAQUE o_t stubs — wrap the base (non-_2) forms which take no option struct.

### [P0] Vertex (extend)  (extend `Vertex`)
Complete vertex interrogation: vertex type (isolated/acorn/normal), owning shells, isolated loops, and tolerant-vertex precision. Small, no option structs — rounds out the bottom of the spine.

- `fn vertex_type(&self) -> PsResult<VertexType>`
  &nbsp;&nbsp;↳ `PK_VERTEX_ask_type`
  &nbsp;&nbsp;· Tokens 0x13ed..0x13f0 known; probe to label.
- `fn shells(&self) -> PsResult<Vec<Shell>>`
  &nbsp;&nbsp;↳ `PK_VERTEX_ask_shells`
  &nbsp;&nbsp;· Vertex->Shell up-link.
- `fn isolated_loops(&self) -> PsResult<Vec<Loop>>`
  &nbsp;&nbsp;↳ `PK_VERTEX_ask_isolated_loops`
  &nbsp;&nbsp;· Vertex-loops where this vertex is the sole content.
- `fn precision(&self) -> PsResult<f64>; fn set_precision(&self, tol: f64) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_VERTEX_ask_precision`, `PK_VERTEX_set_precision`
  &nbsp;&nbsp;· Tolerant-vertex precision (P9).
- `fn delete_acorn(self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_VERTEX_delete_acorn`
  &nbsp;&nbsp;· P2: delete an isolated (acorn) vertex.

  **Blocker:** PK_vertex_type_t band 0x13ed..0x13f0 (N=4) is known from the RE catalog — decode is available, confirm with a probe on an acorn/wire body. set_precision base form is clean; no other blockers.

### [P0] Loop (extend)  (extend `Loop`)
Add the loop-level navigation missing from the spine: direct edges/vertices of a loop, first fin, next-loop-in-face iteration, owning body, and isolated-loop test. Needed so a CADabra loop can be diffed against the oracle without walking fins manually.

- `fn edges(&self) -> PsResult<Vec<Edge>>`
  &nbsp;&nbsp;↳ `PK_LOOP_ask_edges`
  &nbsp;&nbsp;· Edges of the loop directly (vs fins()).
- `fn vertices(&self) -> PsResult<Vec<Vertex>>`
  &nbsp;&nbsp;↳ `PK_LOOP_ask_vertices`
  &nbsp;&nbsp;· Vertices around the loop.
- `fn first_fin(&self) -> PsResult<Fin>`
  &nbsp;&nbsp;↳ `PK_LOOP_ask_first_fin`
  &nbsp;&nbsp;· Entry point for fin-cycle walking.
- `fn next_in_face(&self) -> PsResult<Option<Loop>>`
  &nbsp;&nbsp;↳ `PK_LOOP_ask_next_in_face`
  &nbsp;&nbsp;· Iterate a face's loops.
- `fn body(&self) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_LOOP_ask_body`
  &nbsp;&nbsp;· Owning body.
- `fn is_isolated(&self) -> PsResult<bool>`
  &nbsp;&nbsp;↳ `PK_LOOP_is_isolated`
  &nbsp;&nbsp;· Vertex/wire loop with no bounding edges.

### [P0] Fin (extend)  (extend `Fin`)
Fins are the half-edge layer where CADabra's orientation/parameterisation lives; only next/prev-in-loop and edge/loop/face are wrapped. Add type, body, the oriented curve + surface/curve parameter maps, and the fin's UV-box/interval — the payload the SSI-to-B-rep and arrangement oracles compare.

- `fn fin_type(&self) -> PsResult<FinType>`
  &nbsp;&nbsp;↳ `PK_FIN_ask_type`
  &nbsp;&nbsp;· Decode + probe.
- `fn body(&self) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_FIN_ask_body`
  &nbsp;&nbsp;· Owning body.
- `fn curve(&self) -> PsResult<Curve>; fn oriented_curve(&self) -> PsResult<(Curve,bool)>`
  &nbsp;&nbsp;↳ `PK_FIN_ask_curve`, `PK_FIN_ask_oriented_curve`
  &nbsp;&nbsp;· The (possibly SP-)curve carrying the fin + its sense.
- `fn geometry(&self) -> PsResult<(Curve, (f64,f64), bool)>`
  &nbsp;&nbsp;↳ `PK_FIN_ask_geometry`
  &nbsp;&nbsp;· Curve + interval + sense in one call.
- `fn is_positive(&self) -> PsResult<bool>`
  &nbsp;&nbsp;↳ `PK_FIN_is_positive`
  &nbsp;&nbsp;· Fin direction vs edge direction — orientation oracle.
- `fn next_of_edge(&self) -> PsResult<Fin>; fn previous_of_edge(&self) -> PsResult<Fin>`
  &nbsp;&nbsp;↳ `PK_FIN_ask_next_of_edge`, `PK_FIN_ask_previous_of_edge`
  &nbsp;&nbsp;· Radial (around-edge) fin ring, for non-manifold edges.
- `fn surf_params(&self, t: f64) -> PsResult<(f64,f64)>`
  &nbsp;&nbsp;↳ `PK_FIN_find_surf_parameters`
  &nbsp;&nbsp;· Curve param t -> face (u,v); the SP-curve->surface map.
- `fn curve_param(&self, uv: (f64,f64)) -> PsResult<f64>`
  &nbsp;&nbsp;↳ `PK_FIN_find_curve_parameter`
  &nbsp;&nbsp;· Inverse of surf_params.
- `fn interval(&self) -> PsResult<(f64,f64)>; fn uvbox(&self) -> PsResult<UvBox>`
  &nbsp;&nbsp;↳ `PK_FIN_find_interval`, `PK_FIN_find_uvbox`
  &nbsp;&nbsp;· Fin parametric span and its face-UV bounding box.

### [P0] Shell (extend)  (extend `Shell`)
Round out the shell layer: shell type (solid/void/wire boundary), owning body, the acorn vertex for a vertex-only shell, wireframe edges, and the shell sign (whether it bounds material inside or outside). Needed for the two-region/two-shell box adjacency oracle (open P3 item).

- `fn shell_type(&self) -> PsResult<ShellType>`
  &nbsp;&nbsp;↳ `PK_SHELL_ask_type`
  &nbsp;&nbsp;· Decode + probe.
- `fn body(&self) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_SHELL_ask_body`
  &nbsp;&nbsp;· Owning body.
- `fn acorn_vertex(&self) -> PsResult<Option<Vertex>>`
  &nbsp;&nbsp;↳ `PK_SHELL_ask_acorn_vertex`
  &nbsp;&nbsp;· For a vertex-only (acorn) shell.
- `fn wireframe_edges(&self) -> PsResult<Vec<Edge>>`
  &nbsp;&nbsp;↳ `PK_SHELL_ask_wireframe_edges`
  &nbsp;&nbsp;· Dangling wire edges of the shell.
- `fn sign(&self) -> PsResult<i32>`
  &nbsp;&nbsp;↳ `PK_SHELL_find_sign`
  &nbsp;&nbsp;· +/- : does the shell bound material inside or outside (solid vs void discriminator).

### [P0] Region (extend)  (extend `Region`)
Regions are the volumetric cells the regularized-boolean classify/commit stage operates on. Add region type + owning body + region adjacency (the cell graph), the solid<->void material flip, and curve/point imprint into a region. Directly serves the P0 'imprint -> arrangement -> classification -> commit' capability.

- `fn region_type(&self) -> PsResult<RegionType>`
  &nbsp;&nbsp;↳ `PK_REGION_ask_type`
  &nbsp;&nbsp;· solid/void/unknown; decode + probe (already have is_solid()).
- `fn body(&self) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_REGION_ask_body`
  &nbsp;&nbsp;· Owning body.
- `fn adjacent_regions(&self) -> PsResult<Vec<Region>>`
  &nbsp;&nbsp;↳ `PK_REGION_ask_regions_adjacent`
  &nbsp;&nbsp;· The region-adjacency (cell) graph — key boolean-classification oracle.
- `fn make_solid(self) -> PsResult<()>; fn make_void(self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_REGION_make_solid`, `PK_REGION_make_void`
  &nbsp;&nbsp;· Flip material state; no option struct. The classify/commit primitive.
- `fn imprint_curve(&self, curve: &Curve, bounds: (f64,f64)) -> PsResult<(Vec<Edge>,Vec<Face>)>`
  &nbsp;&nbsp;↳ `PK_REGION_imprint_curve`
  &nbsp;&nbsp;· P1: mirrors Face::imprint_curve but into a region; no option struct.
- `fn imprint_point(&self, pos: Vec3) -> PsResult<Vertex>`
  &nbsp;&nbsp;↳ `PK_REGION_imprint_point`
  &nbsp;&nbsp;· P1: point imprint into a region.
- `fn embed_body(&self, tool: Body) -> PsResult<EmbedResult>`
  &nbsp;&nbsp;↳ `PK_REGION_embed_body`, `PK_REGION_embed_body_r_f`
  &nbsp;&nbsp;· P2: embed a body into a region cell; validate o_t/r_t.
- `fn combine_bodies(&self, tools: Vec<Body>) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_REGION_combine_bodies`
  &nbsp;&nbsp;· P2: merge bodies into a region.

  **Blocker:** Typing/adjacency (P0) need only a PK_region_type_t decode. make_solid/make_void/imprint_curve/imprint_point (P1) take NO option struct — imprint_curve takes PK_INTERVAL_t by value (already handled ABI-wise in face.rs). embed_body (P2) uses documented-grade PK_REGION_embed_body_o_t + PK_REGION_embed_body_r_t (validate layout). combine_bodies is clean.

### [P0] Entity / Topol (extend)  (extend `Entity`)
PK_TOPOL_* and PK_ENTITY_* act on any topological tag, so they belong on Entity (with Body conveniences). This bundles the oracle's cheap-but-high-value invariants: point->body and body->body distance (open P5 item), oriented/transformed and non-axis-aligned bounding boxes, clash detection (open P5 item), connectivity, geometry categorisation, plus entity identity/user-data and redundant-topology cleanup and general-body construction.

- `fn distance_to(&self, other: Entity) -> PsResult<RangeResult>`
  &nbsp;&nbsp;↳ `PK_TOPOL_range`
  &nbsp;&nbsp;· P0. min distance + both foot-points; RangeResult{status,distance,end_1,end_2}. Fully buildable today.
- `fn distance_to_point(&self, p: Vec3) -> PsResult<RangeResult>`
  &nbsp;&nbsp;↳ `PK_TOPOL_range_vector`
  &nbsp;&nbsp;· P0. point->body distance (explicit open P5 item). Fully buildable today.
- `fn bounding_box_in(&self, transf: Option<&Transform>) -> PsResult<Aabb>`
  &nbsp;&nbsp;↳ `PK_TOPOL_find_box_2`
  &nbsp;&nbsp;· P1. box in a transformed frame / tighter than find_box; o_t documented.
- `fn narrow_box(&self) -> PsResult<Nabox>`
  &nbsp;&nbsp;↳ `PK_TOPOL_find_nabox`
  &nbsp;&nbsp;· P1. non-axis-aligned (oriented) bounding box; PK_NABOX_sf_t modeled.
- `fn is_connected(topols: &[Entity]) -> PsResult<bool>`
  &nbsp;&nbsp;↳ `PK_TOPOL_is_connected`
  &nbsp;&nbsp;· P1. Verify PK_TOPOL_is_connected_r_t layout (may be simple).
- `fn clash(targets: &[Entity], tools: &[Entity]) -> PsResult<Vec<Clash>>`
  &nbsp;&nbsp;↳ `PK_TOPOL_clash`
  &nbsp;&nbsp;· P1 but BLOCKED: PK_TOPOL_clash_t result element is stubbed as c_int; needs struct RE. o_t documented.
- `fn geom_category(&self) -> PsResult<GeomCategory>`
  &nbsp;&nbsp;↳ `PK_TOPOL_categorise_geom`
  &nbsp;&nbsp;· P1. PK_GEOM_category_t band 0x650e..0x6513 known — analytic vs spline vs mesh classification.
- `fn identifier(&self) -> PsResult<i32>; fn user_field(&self) -> PsResult<i32>; fn set_user_field(&self, v: i32) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_ENTITY_ask_identifier`, `PK_ENTITY_ask_user_field`, `PK_ENTITY_set_user_field`
  &nbsp;&nbsp;· P2. Stable per-entity id + user tag for oracle correlation across CADabra<->PK.
- `fn first_attrib(&self) -> PsResult<Option<Entity>>; fn owning_groups(&self) -> PsResult<Vec<Entity>>; fn partition(&self) -> PsResult<Entity>`
  &nbsp;&nbsp;↳ `PK_ENTITY_ask_first_attrib`, `PK_ENTITY_ask_owning_groups`, `PK_ENTITY_ask_partition`
  &nbsp;&nbsp;· P2. Entity->attrib/group/partition up-links.
- `fn description(&self) -> PsResult<String>`
  &nbsp;&nbsp;↳ `PK_ENTITY_ask_description`, `PK_ENTITY_ask_description_r_f`
  &nbsp;&nbsp;· P2. Human-readable class/description; o_t documented, free r_f string.
- `fn copy_tracked(&self) -> PsResult<(Entity, Tracking)>`
  &nbsp;&nbsp;↳ `PK_ENTITY_copy_2`, `PK_ENTITY_copy_r_f`
  &nbsp;&nbsp;· P2. Copy with a topology-tracking map (needed to relate oracle copies).
- `fn delete_redundant(topols: &[Entity]) -> PsResult<()>; fn identify_redundant(topols: &[Entity]) -> PsResult<Vec<Entity>>`
  &nbsp;&nbsp;↳ `PK_TOPOL_delete_redundant`, `PK_TOPOL_delete_redundant_2`, `PK_TOPOL_identify_redundant`
  &nbsp;&nbsp;· P2. Remove/flag redundant topology before oracle comparison (base form no o_t).
- `fn make_general_body(topols: &[Entity]) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_TOPOL_make_general_body`, `PK_TOPOL_make_new`
  &nbsp;&nbsp;· P2. Assemble a general body from loose topology; make_new o_t documented.

  **Blocker:** BUILDABLE NOW: PK_TOPOL_range / PK_TOPOL_range_vector — result structs PK_range_2_r_t/PK_range_1_r_t/PK_range_end_t/PK_range_result_t are fully modeled in distance.rs; PK_topol_range_o_t is documented-grade (40 B, NULL default ok). PK_TOPOL_find_box_2 / find_nabox: o_t documented, PK_NABOX_sf_t modeled. BLOCKED: PK_TOPOL_clash returns *PK_TOPOL_clash_t which is a c_int STUB (real element is a struct — needs layout); PK_TOPOL_find_connected_r_t and PK_ENTITY_range_r_t are [u8;0] opaque (use PK_TOPOL_range instead of PK_ENTITY_range). PK_ENTITY_ask_description_o_t / copy_o_t / make_new_o_t / delete_redundant_2_o_t / identify_redundant_o_t / is_connected_o_t are documented-grade.

### [P1] Face imprint & sheet/solid construction (extend Face)  (extend `Face`)
The imprint->arrangement half of the boolean oracle plus wrapping faces as bodies for interrogation. Separated from core Face interrogation because these are model-mutating and several carry option/result structs of varying readiness.

- `fn make_sheet_body(faces: &[Face]) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_FACE_make_sheet_body`
  &nbsp;&nbsp;· No option struct — cleanest 'wrap faces as a sheet body' (TODO P1 need).
- `fn make_solid_bodies(faces: &[Face], heal: HealAction) -> PsResult<Vec<Body>>`
  &nbsp;&nbsp;↳ `PK_FACE_make_solid_bodies`
  &nbsp;&nbsp;· No o_t; returns bodies + PK_local_check_t results (token band known).
- `fn make_sheet_bodies(faces: &[Face]) -> PsResult<Vec<Body>>`
  &nbsp;&nbsp;↳ `PK_FACE_make_sheet_bodies`
  &nbsp;&nbsp;· o_t documented; per-face sheet bodies with tracking.
- `fn imprint_faces(&self, tools: &[Face]) -> PsResult<ImprintResult>`
  &nbsp;&nbsp;↳ `PK_FACE_imprint_faces`, `PK_FACE_imprint_faces_2`, `PK_FACE_imprint_faces_r_f`
  &nbsp;&nbsp;· Arrangement oracle: imprint one face-set onto another. o_t documented; validate PK_imprint_r_t.
- `fn imprint_curves(&self, curves: &[Curve]) -> PsResult<(Vec<Edge>,Vec<Face>)>`
  &nbsp;&nbsp;↳ `PK_FACE_imprint_curves_2`
  &nbsp;&nbsp;· BLOCKED on OPAQUE PK_FACE_imprint_curves_o_t; single-curve Face::imprint_curve already exists as a workaround.

  **Blocker:** PK_FACE_make_sheet_body and PK_FACE_make_solid_bodies take NO option struct (make_solid_bodies uses PK_FACE_heal_t + PK_LOGICAL_t + PK_local_check_t out — PK_local_check_t band 0x46b4..0x46b7 known) — buildable now. PK_FACE_imprint_faces / imprint_faces_2 use documented-grade PK_FACE_imprint_o_t + PK_imprint_r_t (validate PK_imprint_r_t). PK_FACE_imprint_curves_2 is BLOCKED (OPAQUE PK_FACE_imprint_curves_o_t). PK_FACE_make_sheet_bodies o_t documented + TOPOL_track_r_t.

**Deferred groups:**
- _Euler operators (manual topology surgery) (25)_ — PK_EDGE_euler_* (11), PK_FACE_euler_* (4), PK_LOOP_euler_* (7), PK_VERTEX_euler_* (3), PK_FIN_euler_glue. Low-level make/delete-edge/loop/ring/zip/slit/split/merge primitives for hand-building topology. The RE catalog explicitly flags EUL_* as a 'topology-only negative control — do NOT RE for algorithms', and TODO P7 defers euler. CADabra builds bodies via primitives+booleans, not euler surgery, so no oracle value near-term. None take option structs, so they are unblocked whenever a use-case appears.
- _Blend marking & interrogation on topology (14)_ — PK_EDGE_set_blend_chamfer/_variable/_chain, PK_EDGE_ask_blend/remove_blend/check_blends/find_blend_topol, PK_FACE_make_blend/make_3_face_blend, PK_VERTEX_make_blend, PK_FACE_find_blend_unders, PK_FACE_identify_blends(_2), PK_FACE_delete_blends. These belong to the Blend domain (blend.rs); RE difficulty HIGH; several use OPAQUE o_t (set_blend_chamfer_o_t) or nest the mis-sized PK_blend_properties_t that already forced body.rs::fillet_edges to pass NULL. Off CADabra's near-term path (TODO P7).
- _Local modeling operations on faces/edges/vertices (58)_ — Face offset/taper/spin/sweep/emboss/change/hollow(_2/_3)/cover/replace_surfs(_2/_3)/reparameterise_surf/simplify_geom/attach_surfs/replace_with_sheet/reverse/section_with_sheet(_2)/make_neutral_sheet(_2)/make_sect_with_sfs/pattern(_2)/instance_bodies/instance_tools/boolean(_2)/output_surf_trimmed/install_surfs_isocline/imprint_curves_isocline/imprint_cus_*/set_approx/unset_approx/split_at_param/delete_2/delete_facesets/delete_from_gen_body/remove_to_solid_bodies; Edge offset_on_body/split_at_param/optimise/reverse(_2)/make_curve/attach_curves(_2)/repair/remove_to_bodies/propagate_orientation/find_deviation(_2)/delete/delete_wireframe; Vertex spin/sweep/optimise/attach_points/remove_edge; Loop offset_planar/close_gaps; Face close_gaps. Local-ops domain (offset.rs/sweep.rs/editing.rs), RE difficulty EXTREME, and heavily gated by OPAQUE option structs. Defer until CADabra needs a local-op oracle (post-boolean).
- _Faceting, rendering & facet-topology (9)_ — PK_TOPOL_facet (OPAQUE PK_TOPOL_facet_o_t) / facet_r_f, PK_TOPOL_render_line/render_facet/render_volume(_r_f), PK_TOPOL_make_facet_topol, PK_FACE_fix_mesh_defects(_r_f). Belongs to the facet.rs domain, which already validates PK_TOPOL_facet_2; these are alternative/older tessellation and line-drawing entry points with no additional oracle value. render_line/render_facet o_t are documented but low priority.
- _Range family (arrays / geom / local) + PK_ENTITY_range (10)_ — PK_TOPOL_range_array/_array_vector/_geom/_geom_array/_local/_local_vector and PK_ENTITY_range/_range_vector(+_r_f). BLOCKED: the array/geom/local variants all use OPAQUE o_t stubs (PK_TOPOL_range_array_o_t etc. are [u8;0]); PK_ENTITY_range's result PK_ENTITY_range_r_t is [u8;0] opaque. The buildable PK_TOPOL_range / PK_TOPOL_range_vector pair (promoted to Entity above) already delivers the point->body and body->body distance the oracle needs, so these stay deferred until their struct layouts are recovered.
- _Attribute / group / attdef plumbing on topology (6)_ — PK_ENTITY_check_attribs, PK_ENTITY_delete_attribs, PK_ENTITY_may_own_attdef, PK_ENTITY_ask_owning_groups_2 (OPAQUE o_t), PK_TOPOL_ask_entities_by_attdef, PK_ENTITY_find_reparam. These are the Attribute/Group domain (attrib.rs) rather than the B-rep spine; the plain first_attrib/owning_groups/ask_attribs cover what the spine oracle needs. Defer to the attribute-domain round.
- _Datum frames & frame imprinting (5)_ — PK_TOPOL_find_frames(_r_f), PK_TOPOL_imprint_frames(_r_f), and the connectivity result PK_TOPOL_find_connected(_r_f) whose PK_TOPOL_find_connected_r_t is [u8;0] opaque. Frames are a datum/coordinate feature (needs the frame result-struct layout RE'd) and are not on CADabra's near-term B-rep-parity path; find_connected is blocked on its opaque result struct (is_connected above gives the boolean answer the oracle actually needs).
- _Nominal / tolerant-geometry attach-detach on edges & faces (6)_ — PK_EDGE_ask_curve_nmnl, PK_EDGE_ask_geometry_nmnl, PK_EDGE_attach_curve_nmnl (OPAQUE o_t), PK_EDGE_attach_curves, PK_EDGE_detach_curve_nmnl, PK_FACE_attach_surf_fitting, PK_FACE_make_valid_faces. Nominal-geometry / tolerant-modeling healing surface; belongs to the sewing/healing domain (RE difficulty HIGH) and only matters once the oracle ingests imperfect imported bodies. Defer.


---

## Geometry — analytic + freeform curves/surfaces, points, transforms, vectors, frames

_Analytic primitive create/ask (plane/cyl/cone/sphere/torus/line/circle/ellipse/point) plus position/tangent/normal eval and SSI are already validated; the next round unlocks the rest of the geometry oracle surface — exact curvature evaluation, the full rigid/scale transform algebra for placing primitives at arbitrary poses (CADabra's "typed frames/coordinates"), orphan-geometry lifecycle (transform/copy/delete/wrap-as-body), curve interval/periodicity, analytic→NURBS conversion, native NURBS create/ask, and the remaining analytic surface types (spun/swept/offset). Most of it is unblocked via simple no-option forms; freeform NURBS editing, fitting, PK_FRAME datums, SP/foreign curves, and tolerant/nominal-geometry attachment are deferred._

### [P1] Transform (extend)  (extend `Transform`)
Native constructors and algebra for PK_TRANSF entities so the oracle can place any primitive at an arbitrary pose and validate CADabra's frame/coordinate transforms exactly. Today only from_matrix/translation/uniform_scale/ask exist; the native rotation/reflection/scale-about-centre constructors, composition, equality and classification are all unexercised. Every constructor takes plain Vec3 args (no option struct) so the core set is unblocked.

- `fn rotation(point: Vec3, axis: Vec3, angle: f64) -> PsResult<Transform>`
  &nbsp;&nbsp;↳ `PK_TRANSF_create_rotation`
  &nbsp;&nbsp;· Rotation about an axis line (point+direction) by angle radians.
- `fn reflection(point: Vec3, normal: Vec3) -> PsResult<Transform>`
  &nbsp;&nbsp;↳ `PK_TRANSF_create_reflection`
  &nbsp;&nbsp;· Mirror about a plane (point+normal).
- `fn scale_about(factor: f64, centre: Vec3) -> PsResult<Transform>`
  &nbsp;&nbsp;↳ `PK_TRANSF_create_equal_scale`
  &nbsp;&nbsp;· Uniform scale about an arbitrary centre — native form, complements the existing matrix-encoded uniform_scale.
- `fn translate(v: Vec3) -> PsResult<Transform>`
  &nbsp;&nbsp;↳ `PK_TRANSF_create_translation`
  &nbsp;&nbsp;· Native translation constructor (parallels existing matrix-built translation; validate they agree).
- `fn compose(&self, then: &Transform) -> PsResult<Transform>`
  &nbsp;&nbsp;↳ `PK_TRANSF_transform`
  &nbsp;&nbsp;· Concatenate two transforms (transf_1 then transf_2 → transf_out). Validate against matrix product of ask().
- `fn enlarge(&self, factor: f64) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_TRANSF_enlarge`
  &nbsp;&nbsp;· Scale the transform in place by a factor.
- `fn is_equal(&self, other: &Transform) -> PsResult<bool>`
  &nbsp;&nbsp;↳ `PK_TRANSF_is_equal`
  &nbsp;&nbsp;· Kernel equality predicate — cheap oracle for transform round-trips.
- `fn classify(&self) -> PsResult<TransfKind>`
  &nbsp;&nbsp;↳ `PK_TRANSF_classify`
  &nbsp;&nbsp;· identity/translation/rotation/reflection/scaling/general. Blocked on applying computed o_t/r_t layouts + PK_TRANSF_diagnostics_t enum.
- `fn check(&self) -> PsResult<Vec<Fault>>`
  &nbsp;&nbsp;↳ `PK_TRANSF_check`
  &nbsp;&nbsp;· Validity check; returns kernel-allocated PK_check_fault_t array (needs fault struct layout + PkArray free).
- `fn view(direction: Vec3) -> PsResult<Transform>`
  &nbsp;&nbsp;↳ `PK_TRANSF_create_view`, `PK_VECTOR_make_view_transf`
  &nbsp;&nbsp;· P2/P3 — view transform; try NULL options first.

  **Blocker:** Constructors + compose + is_equal + enlarge: none (plain vector/scalar args, sf already validated). classify/check: PK_TRANSF_classify_o_t/_r_t and PK_TRANSF_check_o_t are [u8;0] stubs — layout is already computed in the RE catalog (transf_classify_o_t = {o_t_version@0, diagnostics:PK_TRANSF_diagnostics_t@4}), needs applying to sys + the PK_check_fault_t array layout + runtime validation. create_view: PK_TRANSF_create_view_o_t stub (computed 40B), likely NULL-able for defaults.

### [P1] Vec3 / vector free functions (extend geom.rs)  (extend `Vec3`)
Apply transforms to raw vectors and do the small vector-algebra kernels CADabra's frame math relies on. All are pure, no option struct except the lsq-plane fit.

- `fn transformed(&self, t: &Transform) -> PsResult<Vec3>`
  &nbsp;&nbsp;↳ `PK_VECTOR_transform`
  &nbsp;&nbsp;· Position transform (includes translation).
- `fn transformed_direction(&self, t: &Transform) -> PsResult<Vec3>`
  &nbsp;&nbsp;↳ `PK_VECTOR_transform_direction`
  &nbsp;&nbsp;· Direction transform (rotation/scale only, ignores translation).
- `fn perpendicular(&self, other: Vec3) -> PsResult<Vec3>`
  &nbsp;&nbsp;↳ `PK_VECTOR_perpendicular`
  &nbsp;&nbsp;· Unit vector perpendicular to self, coplanar with other — handy for building Axis2 ref_directions.
- `fn best_fit_plane(points: &[Vec3]) -> PsResult<Surf>`
  &nbsp;&nbsp;↳ `PK_VECTOR_make_lsq_plane`, `PK_VECTOR_make_lsq_plane_r_f`
  &nbsp;&nbsp;· Least-squares plane through a point cloud → orphan Surf. P2.

  **Blocker:** transformed/transformed_direction/perpendicular: none. best_fit_plane: PK_VECTOR_make_lsq_plane_o_t is a [u8;0] stub but the computed layout is trivial (o_t_version@0 only) and options is *const → try NULL for defaults; result is an orphan PK_PLANE plus a _r_f companion to free.

### [P1] Surf (extend) — curvature, conversion, extra analytic types  (extend `Surf`)
Finish the surface oracle: exact principal-curvature evaluation (the next eval primitive after position/normal, and the signal SSI tangency classification leans on), analytic→NURBS conversion for cross-checking CADabra NURBS, and the remaining analytic surface constructors (spun/swept/offset) with create→ask round-trips.

- `fn eval_curvature(&self, u: f64, v: f64) -> PsResult<SurfCurvature>`
  &nbsp;&nbsp;↳ `PK_SURF_eval_curvature`
  &nbsp;&nbsp;· Returns normal + 2 principal directions + 2 principal curvatures. High oracle value; validate on sphere (k1=k2=1/r), cylinder (k=1/r,0), plane (0,0), torus.
- `fn to_bsurf(&self, uvbox: UvBox, cubic: bool, non_rational: bool, tol: f64) -> PsResult<(Surf, bool)>`
  &nbsp;&nbsp;↳ `PK_SURF_make_bsurf`
  &nbsp;&nbsp;· Analytic→NURBS conversion, returns (bsurf, exact). Lets the oracle diff CADabra NURBS against a Parasolid NURBS of the same analytic surface.
- `fn spun(profile: &Curve, axis: Axis1) -> PsResult<Surf> / fn ask_spun(&self) -> PsResult<SpunData>`
  &nbsp;&nbsp;↳ `PK_SPUN_create`, `PK_SPUN_ask`
  &nbsp;&nbsp;· Surface of revolution (profile curve + axis line). P2.
- `fn swept(profile: &Curve, path: Vec3) -> PsResult<Surf> / fn ask_swept(&self) -> PsResult<SweptData>`
  &nbsp;&nbsp;↳ `PK_SWEPT_create`, `PK_SWEPT_ask`, `PK_SWEPT_ask_r_f`
  &nbsp;&nbsp;· Linear-extrusion surface (profile + path vector). ask uses _r_f to free. P2.
- `fn offset_surf(base: &Surf, distance: f64) -> PsResult<Surf> / fn ask_offset(&self) -> PsResult<OffsetData>`
  &nbsp;&nbsp;↳ `PK_OFFSET_create`, `PK_OFFSET_ask`
  &nbsp;&nbsp;· Offset surface (base surface + distance). P2.
- `fn isoparam_curve(&self, param: f64, dir: ParamDir) -> PsResult<(Curve,(f64,f64))>`
  &nbsp;&nbsp;↳ `PK_SURF_make_curve_isoparam`
  &nbsp;&nbsp;· Extract a u- or v-isoparametric curve. Needs the small computed o_t. P2/P3.
- `fn ask_params(&self) -> PsResult<SurfPeriodicity>`
  &nbsp;&nbsp;↳ `PK_SURF_ask_params`
  &nbsp;&nbsp;· BLOCKED — PK_SURF_param_t layout wrong; needs RE. uvbox() already covers seam/pole ranges as a stopgap.

  **Blocker:** eval_curvature: none (PK_SURF_eval_curvature is explicit-arg, no option struct). to_bsurf: none (PK_SURF_make_bsurf simple form takes cubic/non_rational/tolerance directly; the _2/_array forms need PK_SURF_make_bsurf_o_t which is a stub — defer those). spun/swept/offset create+ask: none (sf structs already modelled; PK_SWEPT_ask uses a _r_f free companion). isoparam_curve: PK_SURF_make_curve_isoparam_o_t stub (computed 12B, small). ask_params: HARD BLOCKER — PK_SURF_param_t/PK_PARAM_sf_t is mismodelled (raw bytes show 2 doubles + tokens 18000/18003/18020/18021/18040/18041, not {u_type,u_period,v_type,v_period}); needs a Ghidra pass on PK_SURF_ask_params (see TODO P2).

### [P1] Curve (extend) — interval, periodicity, curvature, wrap-as-body, conversion  (extend `Curve`)
Complete the curve oracle: parametric interval and closed/periodic classification (TODO P2), exact curvature (radius/normal), wrapping an orphan curve as a wire body for interrogation (TODO P1), and analytic→NURBS conversion.

- `fn interval(&self) -> PsResult<(f64, f64)>`
  &nbsp;&nbsp;↳ `PK_CURVE_ask_interval`
  &nbsp;&nbsp;· Parametric [t_min,t_max]; complements the arc-length length() already wrapped.
- `fn param_type(&self) -> PsResult<CurvePeriodicity>`
  &nbsp;&nbsp;↳ `PK_CURVE_ask_param`
  &nbsp;&nbsp;· periodic/non-periodic + period. Validate the PK_CURVE_param_t layout against a circle (period 2π).
- `fn eval_curvature(&self, t: f64) -> PsResult<CurveCurvature>`
  &nbsp;&nbsp;↳ `PK_CURVE_eval_curvature`
  &nbsp;&nbsp;· position + tangent + principal_normal + scalar curvature. Validate on a circle (curvature=1/r, normal points to centre).
- `fn make_wire_body(&self, range: (f64,f64)) -> PsResult<Body>`
  &nbsp;&nbsp;↳ `PK_CURVE_make_wire_body`
  &nbsp;&nbsp;· TODO P1 — wrap an orphan curve as a wire body so it can be interrogated/transmitted. Simple by-value-interval form; PK_CURVE_make_wire_body_2 (options) is deferred.
- `fn to_bcurve(&self, range: (f64,f64), cubic: bool, non_rational: bool, tol: f64) -> PsResult<(Curve,bool)>`
  &nbsp;&nbsp;↳ `PK_CURVE_make_bcurve`
  &nbsp;&nbsp;· Analytic→NURBS curve conversion, returns (bcurve, exact). P2.

  **Blocker:** interval/eval_curvature/make_wire_body/to_bcurve: none — all have explicit-arg forms (PK_CURVE_ask_interval; PK_CURVE_eval_curvature; PK_CURVE_make_wire_body takes a by-value PK_INTERVAL_t, no options; PK_CURVE_make_bcurve simple form). param_type: PK_CURVE_param_t is a concrete 2-field struct {param_type, period} but carries the same mis-modelling risk as PK_SURF_param_t — validate at runtime; treat as soft blocker (may need the same RE fix). The _2/array bcurve forms need PK_CURVE_make_bcurve_o_t (stub, computed 48B).

### [P1] Geom lifecycle (extend Surf/Curve/Point + Body)  (extend `Surf`)
Manage orphan geometry: transform a bare surf/curve/point to a new pose (feeds the SSI oracle at arbitrary placements without building a body), copy, delete a single orphan geom, scale, and attach/detach orphan geoms to a part. PK_GEOM_* operate on any geometry tag, so these mirror onto Curve and Point too.

- `fn transform_geom(&self, t: &Transform, tol: f64) -> PsResult<(Surf, bool)>`
  &nbsp;&nbsp;↳ `PK_GEOM_transform`
  &nbsp;&nbsp;· P1 — place an orphan analytic surface/curve at a pose, then intersect. Returns (new_geom, exact). Mirrors on Curve/Point.
- `fn copy_geom(&self) -> PsResult<Surf>`
  &nbsp;&nbsp;↳ `PK_GEOM_copy`, `PK_GEOM_copy_r_f`
  &nbsp;&nbsp;· Copy orphan geometry (batch form available).
- `fn delete_geom(self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_GEOM_delete_single`
  &nbsp;&nbsp;· Delete a single orphan geom entity.
- `fn enlarge_geom(&self, factor: f64) -> PsResult<Surf>`
  &nbsp;&nbsp;↳ `PK_GEOM_enlarge`, `PK_GEOM_enlarge_r_f`
  &nbsp;&nbsp;· Scale orphan geometry. P3 (needs enlarge_o_t).
- `Body::add_geoms(&self, geoms: &[Entity]) / remove_geoms(&self, geoms: &[Entity])`
  &nbsp;&nbsp;↳ `PK_PART_add_geoms`, `PK_PART_remove_geoms`
  &nbsp;&nbsp;· Attach/detach orphan geometry to a part. P2.

  **Blocker:** transform_geom: none — PK_GEOM_transform (simple form) is (in_geom, transf, tol, out_geom, exact), no option struct. delete_geom: none. copy_geoms: PK_GEOM_copy_o_t/_r_t are NOT in the stub set (appear modelled) + PK_GEOM_copy_r_f free — validate. enlarge_geoms: needs PK_GEOM_enlarge_o_t + _r_f. Part add/remove: plain arrays. The PK_GEOM_transform_2 (o_t) form is a stub — use the simple form.

### [P2] NURBS create/ask (extend Curve + Surf)  (extend `Surf`)
Native freeform NURBS: build a B-curve/B-surface from control points, weights and knots, and read them back — the create→ask round-trip CADabra will need once it emits NURBS. The sf structs (PK_BCURVE_sf_t/PK_BSURF_sf_t) are already fully modelled as pointer-to-array standard forms, so the create path is unblocked; the ask path's array ownership needs care.

- `Curve::bcurve(ctrl: &[Vec3], weights: Option<&[f64]>, knots: &[f64], degree: i32, periodic: bool) -> PsResult<Curve>`
  &nbsp;&nbsp;↳ `PK_BCURVE_create`
  &nbsp;&nbsp;· Build a NURBS curve; weights present ⇒ vertex_dim=4/rational.
- `Curve::ask_bcurve(&self) -> PsResult<BcurveData>`
  &nbsp;&nbsp;↳ `PK_BCURVE_ask`, `PK_BCURVE_ask_knots`
  &nbsp;&nbsp;· Read back control net + knots; validate against inputs.
- `Surf::bsurf(net: &[[Vec3]], weights: Option<..>, u_knots: &[f64], v_knots: &[f64], u_deg: i32, v_deg: i32) -> PsResult<Surf>`
  &nbsp;&nbsp;↳ `PK_BSURF_create`
  &nbsp;&nbsp;· Build a NURBS surface (column-major control grid).
- `Surf::ask_bsurf(&self) -> PsResult<BsurfData>`
  &nbsp;&nbsp;↳ `PK_BSURF_ask`, `PK_BSURF_ask_knots`
  &nbsp;&nbsp;· Read back the NURBS surface standard form + knot vectors.
- `Surf::bsurf_isoparam(&self, param: f64, dir: ParamDir) -> PsResult<Curve>`
  &nbsp;&nbsp;↳ `PK_BSURF_make_bcurve_u_isoparam`, `PK_BSURF_make_bcurve_v_isoparam`
  &nbsp;&nbsp;· Extract an isoparametric B-curve from a B-surface (no options). P2/P3.

  **Blocker:** bcurve/bsurf create: none — sf layouts modelled (degree, n_vertices, vertex_dim, vertices*, n_knots, knots*, rational/periodic/closed flags). ask/ask_knots: returns an sf whose vertex/knot arrays are kernel-allocated — confirm the free mechanism (no obvious _r_f companion; likely PK_MEMORY_free or caller-sized), a residual to validate. Interpolation/fitting variants are separate (deferred, need o_t stubs).

### [P3] Frame  (new type)
Wrap PK_FRAME reference/datum-frame entities (a frame owns attached construction geometry with a sense, owned by a body). Distinct from Transform/Axis2. Only needed once CADabra publishes datum frames; not on the near-term oracle path.

- `fn ask_body(&self) -> PsResult<Body> / fn ask_owner(&self) -> PsResult<Entity>`
  &nbsp;&nbsp;↳ `PK_FRAME_ask_body`, `PK_FRAME_ask_owner`, `PK_FRAME_ask_owner_r_f`
- `fn geometry(&self) -> PsResult<Vec<Entity>> / fn sense(&self) -> PsResult<bool>`
  &nbsp;&nbsp;↳ `PK_FRAME_ask_geometry`, `PK_FRAME_ask_geometry_r_f`, `PK_FRAME_ask_sense`
- `fn attach_geoms(&self, geoms: &[Entity]) -> PsResult<()> / fn reverse(&self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_FRAME_attach_geoms`, `PK_FRAME_attach_geoms_r_f`, `PK_FRAME_reverse`, `PK_FRAME_reverse_r_f`
- `Body::find_frames(&self) / Body::imprint_frames(&self, ...)`
  &nbsp;&nbsp;↳ `PK_TOPOL_find_frames_r_f`, `PK_TOPOL_imprint_frames_r_f`
  &nbsp;&nbsp;· Discover/imprint frames on a body.

  **Blocker:** PK_FRAME class token is still [guess]/[unknown] in PK_CLASS_t (TODO P0 gap — probe it). Several accessors have _r_f free companions (ask_geometry_r_f, ask_owner_r_f, attach_geoms_r_f, reverse_r_f). Entity/ownership semantics need a runtime shakedown.

**Deferred groups:**
- _NURBS editing & refinement (PK_BCURVE_*/PK_BSURF_* modify ops) (34)_ — Freeform B-curve/B-surface surgery — add_knot, remove_knots, raise/lower_degree, reparameterise, clamp_knots, extend, combine, join, spin, sweep, make_matched, make_bsurf_lofted, find_g1_discontinuity, piecewise/splinewise create+ask, and the eval_approx/set_approx/unset_approx approximation cache. Not on CADabra's near-term path; most are driven by [u8;0] option-struct stubs (BCURVE_join_o_t, remove_knots_o_t, raise/lower_degree_o_t, reparameterise_o_t, ask_knots_o_t, clamp_knots_o_t, make_matched_o_t, make_bsurf_lofted_o_t, plus BCURVE_extend_r_t/spline_r_t and piecewise/splinewise_sf_t). Wrap once CADabra emits and edits NURBS.
- _NURBS interpolation & fitting (create_spline / by_fitting / fitted / constrained) (11)_ — PK_BCURVE_create_spline(_2), create_by_fitting, create_fitted; PK_BSURF_create_fitted, create_constrained; PK_CURVE/SURF make_bcurve_2/array. High future value (fit a NURBS through sampled CADabra output) but every entry is gated on an opaque option struct — BCURVE_create_spline_o_t (computed 136B, +PK_VECTOR_t deriv arrays), BCURVE_create_by_fitting_o_t (computed 64B, +PK_BCURVE_fit_data_t*), BSURF_create_fitted_o_t and BSURF_create_rained_o_t (note the truncated name — needs RE), CURVE_make_bcurve_o_t (computed 48B). Apply the computed layouts to sys and validate before wrapping.
- _SP-curves, trimmed curves, foreign geometry (5)_ — PK_SPCURVE_create/ask (curve-on-surface via a 2D UV bcurve), PK_TRCURVE_ask (read-only trimmed curve), PK_FCURVE_create / PK_FSURF_create (foreign KI-array geometry). Niche entity kinds CADabra does not emit; SP-curves matter only if the SSI oracle needs to sample intersection carriers in UV space (TODO P4 open item), foreign geometry needs a KI key/int/real array contract.
- _Isocline / isoparam / approx helper curves (5)_ — PK_CURVE_make_surf_isocline, PK_SURF_make_cus_isocline, PK_CURVE_make_approx, PK_CURVE_is_isoparam, PK_ENTITY_find_reparam. Silhouette/isocline and approximation helpers — specialized, no oracle demand yet; make_cus_isocline needs its o_t stub.
- _Tolerant / nominal (nmnl) geometry attachment & healing (24)_ — Edge/vertex/face precision + geometry attachment and repair: PK_EDGE_attach_curves(_2), attach_curve_nmnl, detach_curve_nmnl, ask_curve_nmnl, ask_geometry_nmnl, optimise, repair, set/reset_precision(_2), ask_precision; PK_VERTEX_attach_points, set/ask_precision, optimise; PK_FIN_attach_curves; PK_FACE_attach_surfs, attach_surf_fitting, replace_surfs_3, close_gaps, reparameterise_surf(_r_f); PK_LOOP_close_gaps; PK_BODY_ask/set_curve_nmnl_state; PK_CURVE_ask_edges_nmnl. This is CADabra's eventual tolerant-modeling / healing surface (TODO P8) — audit only when that path is oracled; several are editing ops with option structs.
- _Result-free (_r_f) memory companions & diagnostic frees (11)_ — PK_GEOM_copy_r_f, PK_GEOM_enlarge_r_f, PK_SWEPT_ask_r_f, PK_VECTOR_make_lsq_plane_r_f, PK_BCURVE_extend_r_f, PK_BCURVE_spline_r_f, PK_FACE_reparameterise_surf_r_f, and the degeneracy/self-intersection frees PK_CURVE_degens_f, PK_CURVE_self_ints_f, PK_SURF_degens_f, PK_SURF_self_ints_f. These are not public methods — they are the kernel-array free companions invoked internally by their owning wrapper (like the existing PkArray/_track_r_f pattern), so they get absorbed into the methods above rather than surfaced.


---

## Infrastructure & data — attributes / groups / session / rollback / partitions / convergent-facet / lattices / assemblies / threads / debug / reports / data-exchange

_This domain is the oracle's "plumbing": XT model interchange, per-entity metadata/identifiers for cross-referencing CADabra vs Parasolid models, session determinism controls, and rollback/introspection. A thin slice (session config, marks/pmarks/partitions core, transmit/receive, colour attribute, facet counts, body check) is already wrapped; the highest next-round value is a validated whole-model XT round-trip plus stable identifiers and general attribute/group read-write. Convergent-mesh, lattices, assemblies, threads, and multi-partition rollback are genuine but off CADabra's near-term path and/or blocked by opaque structs or the in-memory delta-frustrum limit._

### [P1] Part (data-exchange + introspection)  (new type)
A new-type over PK_PART_t (== PK_ENTITY_t; Body and Assembly are Parts) that owns the whole-model XT interchange oracle (TODO P6) plus part-level geometry/attribute/identifier introspection. This is the mechanism by which a CADabra-built body and a Parasolid-built body get compared as whole models, and by which stable identifiers survive a write→read round-trip.

- `fn transmit_ex(parts: &[Part], key: &str, format: TransmitFormat, version: Option<i32>) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_PART_transmit`, `PK_PART_transmit_b`, `PK_PART_transmit_u`
  &nbsp;&nbsp;· Generalises the existing text-only fileio::transmit to selectable format (text / neutral-binary / unicode-keyed) and a target schema version (transmit_version field) so SOLIDWORKS-vintage XT can be produced. TODO P6: 'text vs neutral-binary format selection'.
- `fn receive_ex(key: &str, opts: ReceiveOptions) -> PsResult<Vec<Part>>`
  &nbsp;&nbsp;↳ `PK_PART_receive`, `PK_PART_receive_b`, `PK_PART_receive_u`, `PK_PART_receive_version`, `PK_PART_receive_version_b`, `PK_PART_receive_version_u`
  &nbsp;&nbsp;· Round-trip counterpart. receive_version* report the file's schema version for cross-version handling (FFCSCH). key_is_partition / receive_compound fields in part_receive_o_t control assembly vs body receipt.
- `fn identifier(&self) -> PsResult<i32>  //on Entity;  fn find_by_identifier(&self, class: PkClass, id: i32) -> PsResult<Entity>;  fn rectify_identifiers(&self) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_ENTITY_ask_identifier`, `PK_PART_find_entity_by_ident`, `PK_PART_rectify_identifiers`
  &nbsp;&nbsp;· P1-critical: persistent identifiers are the only stable cross-reference between a transmitted and re-received body, so topology diffs survive the round-trip. No option structs. rectify re-derives identifiers after edits.
- `fn geoms(&self) -> PsResult<Vec<Entity>>;  fn add_geoms(&self, g: &[Entity]) -> PsResult<()>;  fn remove_geoms(&self, g: &[Entity]) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_PART_ask_geoms`, `PK_PART_add_geoms`, `PK_PART_remove_geoms`
  &nbsp;&nbsp;· Orphan/construction geometry ownership inside a part; complements Partition::geoms.
- `fn construction_curves(&self)->PsResult<Vec<Curve>>; fn construction_points(&self)->PsResult<Vec<Point>>; fn construction_surfs(&self)->PsResult<Vec<Surf>>; fn ref_instances(&self)->PsResult<Vec<Instance>>; fn con_lattices(&self)->PsResult<Vec<Lattice>>`
  &nbsp;&nbsp;↳ `PK_PART_ask_construction_curves`, `PK_PART_ask_construction_points`, `PK_PART_ask_construction_surfs`, `PK_PART_ask_ref_instances`, `PK_PART_ask_con_lattices`
  &nbsp;&nbsp;· Read-side introspection; all plain ask fns.
- `fn all_attribs(&self) -> PsResult<Vec<Attribute>>;  fn all_attdefs(&self) -> PsResult<Vec<AttribDef>>;  fn attrib_owners(&self, def: &AttribDef) -> PsResult<Vec<Entity>>;  fn delete_attribs(&self, def: Option<&AttribDef>) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_PART_ask_all_attribs`, `PK_PART_ask_all_attdefs`, `PK_PART_ask_attrib_owners`, `PK_PART_ask_attribs_filter`, `PK_PART_delete_attribs`
  &nbsp;&nbsp;· Bulk attribute enumeration over a whole part — pairs with the Attribute type below. attribs_filter needs part_ask_attribs_cb_o_t (opaque) only for the callback form; the plain forms are unblocked.

### [P2] Attribute  (new type)
A new-type over PK_ATTRIB_t giving general typed read/write of application data on any entity, generalising the single hard-coded colour helper in attrib.rs. Lets the oracle carry and compare metadata (names, tolerances, provenance) and read back whatever attributes an imported XT body carries. Also extends AttribDef (custom definitions) and Entity (attribute enumeration).

- `fn create(owner: Entity, def: &AttribDef) -> PsResult<Attribute>;  fn attdef(&self) -> PsResult<AttribDef>;  fn owner(&self) -> PsResult<Entity>`
  &nbsp;&nbsp;↳ `PK_ATTRIB_create_empty`, `PK_ATTRIB_ask_attdef`, `PK_ATTRIB_ask_owner`
  &nbsp;&nbsp;· create_empty already used privately by Face::set_colour; promote to a first-class ctor plus back-links.
- `fn ints(&self,field:i32)->PsResult<Vec<i32>>; fn set_ints(&self,field:i32,v:&[i32])->PsResult<()>; fn doubles(..)->..; fn vectors(..)->..; fn axes(..)->..; fn string(&self,field:i32)->PsResult<String>; fn set_string(..); fn pointers(..)`
  &nbsp;&nbsp;↳ `PK_ATTRIB_ask_ints`, `PK_ATTRIB_set_ints`, `PK_ATTRIB_ask_doubles`, `PK_ATTRIB_set_doubles`, `PK_ATTRIB_ask_vectors`, `PK_ATTRIB_set_vectors`, `PK_ATTRIB_ask_axes`, `PK_ATTRIB_set_axes`, `PK_ATTRIB_ask_string`, `PK_ATTRIB_set_string`, `PK_ATTRIB_ask_ustring`, `PK_ATTRIB_set_ustring`, `PK_ATTRIB_ask_pointers`, `PK_ATTRIB_set_pointers`, `PK_ATTRIB_ask_nth_int`, `PK_ATTRIB_ask_nth_double`, `PK_ATTRIB_ask_nth_vector`, `PK_ATTRIB_ask_nth_axis`, `PK_ATTRIB_ask_nth_pointer`
  &nbsp;&nbsp;· Full by-field typed accessor set; ask_doubles/set_doubles already validated via colour. Vectors/axes carry the by-value PK_VECTOR ABI already proven in surf.rs. pointers = app-owned void*, wrap as usize opaque.
- `fn named_ints(&self,name:&str)->PsResult<Vec<i32>>; fn set_named_doubles(&self,name:&str,v:&[f64])->PsResult<()>; …`
  &nbsp;&nbsp;↳ `PK_ATTRIB_ask_named_ints`, `PK_ATTRIB_ask_named_doubles`, `PK_ATTRIB_ask_named_vectors`, `PK_ATTRIB_ask_named_axes`, `PK_ATTRIB_ask_named_string`, `PK_ATTRIB_ask_named_ustring`, `PK_ATTRIB_ask_named_pointers`, `PK_ATTRIB_set_named_ints`, `PK_ATTRIB_set_named_doubles`, `PK_ATTRIB_set_named_vectors`, `PK_ATTRIB_set_named_axes`, `PK_ATTRIB_set_named_string`, `PK_ATTRIB_set_named_ustring`, `PK_ATTRIB_set_named_pointers`
  &nbsp;&nbsp;· Field-by-name variants for definitions created via PK_ATTDEF_create_2 (named fields). Same ABI, name string instead of index.
- `fn attribs(&self, def: Option<&AttribDef>) -> PsResult<Vec<Attribute>>  //on Entity;  fn first_attrib(&self, def:&AttribDef)->PsResult<Option<Attribute>>;  fn delete_attribs(&self,def:Option<&AttribDef>)->PsResult<()>;  fn check_attribs(&self)->PsResult<Vec<Entity>>`
  &nbsp;&nbsp;↳ `PK_ENTITY_ask_attribs`, `PK_ENTITY_ask_first_attrib`, `PK_ENTITY_delete_attribs`, `PK_ENTITY_check_attribs`
  &nbsp;&nbsp;· Entity-side enumeration; ENTITY_ask_attribs already used privately in Face::colour. check_attribs uses entity_check_attribs_o_t (12 B, recovered).
- `fn create_def(name:&str, fields:&[FieldSpec], owner_classes:&[PkClass]) -> PsResult<AttribDef>;  fn ask(&self) -> PsResult<AttribDefInfo>;  fn contexts(&self)->PsResult<..>`
  &nbsp;&nbsp;↳ `PK_ATTDEF_create`, `PK_ATTDEF_create_2`, `PK_ATTDEF_ask`, `PK_ATTDEF_ask_2`, `PK_ATTDEF_ask_contexts`, `PK_ATTDEF_set_contexts`
  &nbsp;&nbsp;· AttribDef (extend the existing find-only type) with custom-definition creation via PK_ATTDEF_sf_t (typed; needs a layout audit of the fields/type-array before trusting). Enables oracle-owned metadata schemas.

### [P2] Group  (new type)
A new-type over PK_GROUP_t: a named, ordered, optionally-labelled set of entities owned by a part. The oracle uses groups to track the output set of an operation (e.g. the faces produced by a boolean/imprint) so results can be enumerated and diffed against CADabra's arrangement output. Genuinely new entity class (PK_CLASS_group already mapped in entity.rs).

- `fn create(part: Part, class: PkClass, entities: &[Entity]) -> PsResult<Group>;  fn entities(&self) -> PsResult<Vec<Entity>>;  fn part(&self) -> PsResult<Part>;  fn entity_class(&self) -> PsResult<PkClass>`
  &nbsp;&nbsp;↳ `PK_GROUP_create_from_entities`, `PK_GROUP_ask_entities`, `PK_GROUP_ask_part`, `PK_GROUP_ask_entity_class`
  &nbsp;&nbsp;· Unblocked core. create_from_entities_2 (opts) deferred to the advanced ctor.
- `fn add(&self, e:&[Entity])->PsResult<()>; fn remove(&self, e:&[Entity])->PsResult<i32>; fn merge(&self, e:&[Entity])->PsResult<i32>; fn contains(&self, e:Entity)->PsResult<bool>`
  &nbsp;&nbsp;↳ `PK_GROUP_add_entities`, `PK_GROUP_remove_entities`, `PK_GROUP_merge_entities`, `PK_GROUP_contains_entity`
  &nbsp;&nbsp;· Mutation set; remove/merge return counts. No option structs.
- `fn entity_label(&self, e:Entity)->PsResult<i32>; fn set_entity_label(&self, e:Entity, label:i32)->PsResult<()>; fn find_by_label(&self, label:i32)->PsResult<Vec<Entity>>`
  &nbsp;&nbsp;↳ `PK_GROUP_ask_entity_label`, `PK_GROUP_set_entity_label`, `PK_GROUP_find_entities`
  &nbsp;&nbsp;· Per-member labels (ordered/tagged membership). find_entities uses the opaque find_entities_o_t — NULL-default for the simple label query.
- `fn owning_groups(&self) -> PsResult<Vec<Group>>  //on Entity;  fn groups(&self) -> PsResult<Vec<Group>>  //on Part`
  &nbsp;&nbsp;↳ `PK_ENTITY_ask_owning_groups`, `PK_PART_ask_groups`
  &nbsp;&nbsp;· Reverse lookup. entity_ask_owning_groups_o_t (20 B) and part_ask_groups_o_t (20 B) layouts recovered; opaque stubs today, NULL-default works for the plain case. The _2 variants are deferred.
- `fn closure(&self) -> PsResult<Vec<Entity>>;  fn controls(&self) -> PsResult<GroupControls>`
  &nbsp;&nbsp;↳ `PK_GROUP_ask_closure`, `PK_GROUP_ask_controls`
  &nbsp;&nbsp;· Transitive closure / structural controls; both gated on the opaque *_o_t/_r_t stubs (layouts in catalog). Lower priority than the core.

  **Blocker:** Base ops are unblocked: PK_GROUP_create_from_entities v1 takes (part, class, n, entities, group) with NO option struct; add/remove/merge/contains/ask_entities/ask_part likewise. Only the advanced constructors and closure/controls queries need option structs that are opaque stubs today — PK_group_create_from_ents_o_t (64 B), PK_group_ask_closure_o_t (8 B), PK_group_ask_controls_o_t/_r_t, PK_group_find_entities_o_t — all have RE-recovered layouts in the catalog, so upgrading generated_stubs unblocks them; pass NULL for defaults meanwhile.

### [P2] Session (extend)  (extend `Session`)
Fill out the ~40 unwrapped PK_SESSION getters/setters the current Session/SessionConfig omit — chiefly the precision/tolerance/tag/determinism controls (TODO P9 'precision hygiene') and lifecycle escape hatches (abort/tidy). These pin the reproducibility settings the oracle must record so its numbers are comparable to CADabra's ToleranceContext.

- `fn abort(&self) -> PsResult<()>;  fn tidy(&self) -> PsResult<()>;  fn comment(&self, text:&str) -> PsResult<()>`
  &nbsp;&nbsp;↳ `PK_SESSION_abort`, `PK_SESSION_tidy`, `PK_SESSION_comment`
  &nbsp;&nbsp;· abort = request interrupt of a running op (pairs with PsError::Aborted); tidy = release scratch memory (determinism/repeatability); comment = write a journal comment.
- `fn highest_tag(&self)->PsResult<i32>; fn tag_limit(&self)->PsResult<i32>; fn set_tag_limit(&self,limit:i32)->PsResult<()>; fn watch_tags(&self, on:bool)->PsResult<()>`
  &nbsp;&nbsp;↳ `PK_SESSION_ask_tag_highest`, `PK_SESSION_ask_tag_limit`, `PK_SESSION_set_tag_limit`, `PK_SESSION_watch_tags`
  &nbsp;&nbsp;· Tag accounting complements the existing tags_remaining; deterministic-tag verification for oracle repeatability (P9).
- `fn is_rollback_on(&self)->PsResult<bool>; fn is_in_kernel(&self)->PsResult<bool>; fn ask_function(&self)->PsResult<String>`
  &nbsp;&nbsp;↳ `PK_SESSION_is_rollback_on`, `PK_SESSION_is_in_kernel`, `PK_SESSION_is_in_kernel_2`, `PK_SESSION_ask_function`
  &nbsp;&nbsp;· Introspection: is partitioned rollback active, is a kernel call in progress, name of the currently-executing PK function (debug/journaling).
- `fn set_journalling(&self, on:bool)->PsResult<()>; fn software_option(&self, opt:i32)->PsResult<bool>; fn set_software_option(..); fn unicode(&self)->PsResult<i32>; fn set_unicode(..)`
  &nbsp;&nbsp;↳ `PK_SESSION_set_journalling`, `PK_SESSION_ask_software_option`, `PK_SESSION_set_software_option`, `PK_SESSION_ask_unicode`, `PK_SESSION_set_unicode`, `PK_SESSION_ask_close_knots`, `PK_SESSION_set_close_knots`, `PK_SESSION_ask_swept_spun_surfs`, `PK_SESSION_set_swept_spun_surfs`, `PK_SESSION_ask_rebuild_history`, `PK_SESSION_set_rebuild_history`
  &nbsp;&nbsp;· Toggle journalling at runtime (config only sets it at start today); licence/software-option gating; unicode mode for keys; geometry-representation switches (close_knots, swept/spun surface preference) that change what create* emits — worth pinning for the oracle.
- `fn attdefs(&self)->PsResult<Vec<AttribDef>>; fn err_reports(&self)->PsResult<bool>; fn max_threads(&self)->PsResult<i32>`
  &nbsp;&nbsp;↳ `PK_SESSION_ask_attdefs`, `PK_SESSION_ask_err_reports`, `PK_SESSION_ask_max_threads`, `PK_SESSION_ask_smp_stacksize`, `PK_SESSION_set_smp_stacksize`
  &nbsp;&nbsp;· Session-wide attribute-definition list, error-report state readback, SMP capacity. ask_err_reports_o_t/ask_max_threads_o_t are opaque but 1-field; can zero-init.

  **Blocker:** Most are scalar get/set with no option struct and are directly wrappable. A few carry opaque *_o_t stubs (PK_SESSION_ask_err_reports_o_t, ask_max_threads_o_t, ask_behaviour_o_t) but the values they gate are already exposed through the typed paths or are minor. facet_geometry/mesh_angle only matter once convergent modelling is wrapped.

### [P2] Rollback introspection (extend Mark/Pmark/Partition)  (extend `Mark`)
Round out the rollback types with the read-side and single-partition operations that do NOT require the persistent delta store: mark/pmark state and membership queries, partition type. This deepens the already-validated single-checkpoint rollback (docs: 'backward rollback is fully validated') without hitting the known frustrum limit.

- `fn state(&self)->PsResult<MarkState>; fn is(&self)->PsResult<bool>; fn is_on(&self)->PsResult<bool>; fn forward(&self)->PsResult<Vec<Mark>>`
  &nbsp;&nbsp;↳ `PK_MARK_ask_state`, `PK_MARK_is`, `PK_MARK_is_on`, `PK_MARK_ask_forward`, `PK_MARK_ask_frustrum`
  &nbsp;&nbsp;· Mark read-side. ask_state distinguishes live/redo/deleted marks; ask_forward is the redo chain (roll-forward itself is blocked, but querying it is not).
- `fn entities(&self)->PsResult<Vec<Entity>>  //on Pmark;  fn marks(&self)->PsResult<Vec<Mark>>;  fn is_used_by_mark(&self)->PsResult<bool>;  fn delete(self)->PsResult<()>`
  &nbsp;&nbsp;↳ `PK_PMARK_ask_entities`, `PK_PMARK_ask_marks`, `PK_PMARK_is`, `PK_PMARK_is_used_by_mark`, `PK_PMARK_delete`
  &nbsp;&nbsp;· Pmark membership/lifecycle. ask_entities uses the opaque pmark_ask_entities_o_t (NULL-default). Lets a rollback test assert exactly which entities a checkpoint holds.
- `fn partition_type(&self)->PsResult<PartitionType>; fn set_partition_type(&self, t:PartitionType)->PsResult<()>; fn ask(&self)->PsResult<PartitionInfo>`
  &nbsp;&nbsp;↳ `PK_PARTITION_ask_type`, `PK_PARTITION_set_type`, `PK_PARTITION_ask`, `PK_PARTITION_is`, `PK_PARTITION_ask_pmark_size`
  &nbsp;&nbsp;· Partition kind (normal/cloned) and summary. PK_PARTITION_ask_o_t/_r_t opaque; ask_type/is/pmark_size are simple.
- `fn create_2(opts) -> PsResult<Mark>;  fn start()->PsResult<()>;  fn stop()->PsResult<()>`
  &nbsp;&nbsp;↳ `PK_MARK_create_2`, `PK_MARK_start`, `PK_MARK_stop`, `PK_MARK_delete_2`, `PK_MARK_create_r_f`, `PK_MARK_delete_r_f`
  &nbsp;&nbsp;· NON-partitioned rollback path (PK_MARK_start starts PK's own rollback). Currently marks ride on the partitioned system; this is an alternative only needed if we support sessions without SessionConfig::rollback. Lower priority; MARK_goto_o_t/_r_t opaque.

  **Blocker:** The UNBLOCKED subset (ask_state, is/is_on, ask_forward, ask_entities, ask_marks, ask_type, delete) needs no persistent store. The BLOCKED subset — multi-pmark navigation, roll-forward, partition merge/copy/switching, cloning, guards, receive_deltas — hits PK_ERROR 5003/10 under the minimal in-memory delta frustrum (documented limitation) and is deferred until a persistent delta store is implemented. Several o_t/r_t here are also opaque stubs (PK_MARK_goto_o_t/_r_t, PK_PMARK_goto_o_t, PK_PARTITION_ask_o_t/_r_t).

### [P2] Error handling (extend)  (extend ``)
Complete the error subsystem beyond the read-only query_last_error already in error.rs: clearing, raising, and (eventually) registering error callbacks so the still-unknown bad_args/severity/entity offsets can be captured (the open P0 residual in TODO).

- `fn clear_last_error() -> PsResult<()>;  fn raise(code:i32)->PsResult<()>`
  &nbsp;&nbsp;↳ `PK_ERROR_clear_last`, `PK_ERROR_raise`, `PK_ERROR_reraise`
  &nbsp;&nbsp;· Clear the session error latch between oracle calls; raise/reraise for wrapper-level propagation tests.
- `fn register_error_callback(cb: ErrorCb) -> PsResult<()>;  fn ask_callbacks()->PsResult<..>`
  &nbsp;&nbsp;↳ `PK_ERROR_register_callbacks`, `PK_ERROR_ask_callbacks`
  &nbsp;&nbsp;· BLOCKED on the full PK_ERROR_sf_t layout; would finally let bad_args/severity be surfaced (closes the P0 error-path residual). Do the RE offset audit first.

  **Blocker:** clear_last/raise/reraise are simple and unblocked. register_callbacks/ask_callbacks depend on the PK_ERROR_sf_t inline-string layout that is only PARTIALLY recovered (function name @0, code @32 confirmed; severity/n_bad_args/bad_args/entity offsets still unknown per docs) — so surfacing full BadArg detail via a callback stays blocked pending that header/RE audit. PK_ERROR_reports_t and the report tokens are opaque stubs.

### [P3] Mesh / FacetBody (convergent modelling)  (new type)
New-types over PK_MESH_t and its facet topology (PK_MTOPOL/MVERTEX/MFACET/MFIN) plus the Body↔facet-body bridge. This is the convergent-modelling representation: a mesh entity carried inside a body, with its own vertex/facet/fin adjacency mirroring the B-rep spine. Useful as a mesh-comparison oracle (tessellated CADabra output vs Parasolid mesh) but off CADabra's near-term analytic path.

- `fn make_facet_body(&self, tol: FacetTol) -> PsResult<Body>  //on Body;  fn meshes(&self) -> PsResult<Vec<Mesh>>  //from mtopols`
  &nbsp;&nbsp;↳ `PK_MTOPOL_make_meshes`, `PK_MESH_make_bodies`, `PK_MESH_make_surf_trimmed`
  &nbsp;&nbsp;· Bridge: PK_body_make_facet_body (via a Body method, o_t recovered) converts a B-rep solid to a convergent facet body — the inverse of the existing Body::facet count-only path, and the natural mesh-oracle entry point.
- `fn n_mfacets(&self)->PsResult<i32>; fn n_mvertices(&self)->PsResult<i32>; fn create_from_facets(verts:&[Vec3], facets:&[[i32;3]]) -> PsResult<Mesh>`
  &nbsp;&nbsp;↳ `PK_MESH_ask_n_mfacets`, `PK_MESH_ask_n_mvertices`, `PK_MESH_create_from_facets`, `PK_MESH_ask_normal_type`
  &nbsp;&nbsp;· Mesh construction from raw triangle soup + counts. Complements facet.rs which currently only returns totals (its vertex table is empty at option v5).
- `fn position(&self)->PsResult<Vec3>  //Mvertex;  fn facets(&self)->PsResult<Vec<Mfacet>>;  fn normal(&self)->PsResult<Vec3>  //Mfacet;  fn mfin(&self)->..`
  &nbsp;&nbsp;↳ `PK_MVERTEX_ask_position`, `PK_MVERTEX_ask_mfacets`, `PK_MVERTEX_ask_mfin`, `PK_MVERTEX_ask_mvertices_ring`, `PK_MVERTEX_is_laminar`, `PK_MFACET_ask_positions`, `PK_MFACET_ask_mvertices`, `PK_MFACET_ask_normal`, `PK_MFACET_ask_mfin`, `PK_MFACET_ask_mfacet_adjacent`, `PK_MFIN_ask_mvertex`, `PK_MFIN_ask_mfacet`, `PK_MFIN_ask_next_in_mfacet`, `PK_MFIN_ask_previous_in_mfacet`, `PK_MFIN_ask_mfin_adjacent`, `PK_MFIN_is_laminar`, `PK_MTOPOL_ask_box`, `PK_MTOPOL_ask_class`, `PK_MTOPOL_is`
  &nbsp;&nbsp;· Facet-topology spine (mirrors Face/Loop/Fin/Vertex). All plain ask fns; the mvx_normal/curvature variants need opaque per-call o_t (defer those).

  **Blocker:** Creation and topology walk are reachable: PK_MESH_create_from_facets and the MVERTEX/MFACET/MFIN ask fns take arrays or NULL-defaultable o_t; PK_mesh_make_bodies_o_t (24 B), PK_body_make_facet_body_o_t (88 B), PK_mtopol_make_meshes_o_t (88 B) layouts are RE-recovered. BLOCKED: the defect find/fix and normals machinery — PK_MESH_find_defects/_fix_defects/_find_laminar/_imprint/_is_loaded return opaque result stubs (PK_MESH_defect_array_t, _fix_result_t, _find_laminar_mfins_r_t, _imprint_vectors_r_t, _is_loaded_r_t) and consume opaque tolerance sub-structs.

### [P3] Assembly / Instance  (new type)
New-types over PK_ASSEMBLY_t / PK_INSTANCE_t: hierarchical part structure (an assembly references child parts through positioned instances). PK_CLASS_assembly/instance are already mapped in entity.rs. Needed only when the oracle must round-trip multi-body XT assemblies; CADabra's near term is single bodies.

- `fn create_empty() -> PsResult<Assembly>;  fn instances(&self)->PsResult<Vec<Instance>>;  fn parts(&self)->PsResult<Vec<Part>>;  fn parts_transfs(&self)->PsResult<Vec<(Part,Transform)>>`
  &nbsp;&nbsp;↳ `PK_ASSEMBLY_create_empty`, `PK_ASSEMBLY_ask_instances`, `PK_ASSEMBLY_ask_parts`, `PK_ASSEMBLY_ask_parts_transfs`, `PK_ASSEMBLY_make_level_assembly`
  &nbsp;&nbsp;· Assembly structure + flattened part list with positioning transforms (reuses the existing Transform type).
- `fn transform(&self, t:&Transform)->PsResult<()>  //Assembly;  fn check(&self)->PsResult<Vec<CheckFault>>`
  &nbsp;&nbsp;↳ `PK_ASSEMBLY_transform`, `PK_ASSEMBLY_check`
  &nbsp;&nbsp;· check mirrors Body::check; assembly_check_o_t (24 B) + _r_t recoverable from catalog.
- `fn create(assembly:Assembly, part:Part, transf:&Transform)->PsResult<Instance>; fn ask(&self)->PsResult<InstanceInfo>; fn change_part(..); fn replace_transf(..); fn transform(..)`
  &nbsp;&nbsp;↳ `PK_INSTANCE_create`, `PK_INSTANCE_ask`, `PK_INSTANCE_change_part`, `PK_INSTANCE_replace_transf`, `PK_INSTANCE_transform`
  &nbsp;&nbsp;· Instance = (child part, transform) placed in a parent assembly. ask returns PK_INSTANCE_sf_t — BLOCKED on that opaque stub's layout.

  **Blocker:** Read-side is unblocked (ask_instances/ask_parts/ask_parts_transfs, INSTANCE_ask/transform are plain). BLOCKED write-side: PK_INSTANCE_sf_t is an opaque [u8;0] stub, so Instance::create/ask of the full standard form is blocked until its layout is recovered; PK_ASSEMBLY_check_o_t/_r_t are opaque stubs (layouts in catalog — recoverable).

### [P3] Debug / Reports  (new type)
A thin module over the PK_DEBUG_* facilities. The standout is PK_DEBUG_BODY_compare — a kernel-native structural body comparison that is potentially a direct oracle primitive (diff a CADabra-produced body against a Parasolid-produced one) — plus session consistency checking and journal/report control.

- `fn compare_bodies(a:&Body, b:&Body) -> PsResult<BodyComparison>`
  &nbsp;&nbsp;↳ `PK_DEBUG_BODY_compare`, `PK_DEBUG_BODY_compare_r_f`, `PK_DEBUG_BODY_extract_data`, `PK_DEBUG_data_f`
  &nbsp;&nbsp;· HIGH potential oracle value — flag for a targeted signature/RE audit. Result struct opaque today.
- `fn session_check(partitions:&[Partition]) -> PsResult<Vec<CheckFault>>`
  &nbsp;&nbsp;↳ `PK_DEBUG_SESSION_check`
  &nbsp;&nbsp;· Whole-session consistency sweep; debug_session_check_o_t (32 B) recovered.
- `fn report_start(path:&str)->PsResult<()>; fn report_comment(text:&str)->PsResult<()>; fn report_stop()->PsResult<()>`
  &nbsp;&nbsp;↳ `PK_DEBUG_report_start`, `PK_DEBUG_report_comment`, `PK_DEBUG_report_stop`, `PK_DEBUG_transmit`, `PK_DEBUG_receive`
  &nbsp;&nbsp;· Diagnostic journal capture for reproducing oracle failures. report_start_o_t opaque (recoverable).

  **Blocker:** PK_DEBUG_BODY_compare / _extract_data / SESSION_check need signature+result audits and consume opaque stubs (PK_DEBUG_BODY_compare_r_f pairs with an opaque result; PK_DEBUG_check_fault_t, PK_debug_session_check_o_t (32 B, recovered) ). report_start/behaviours_start/shuffle_start o_t are opaque stubs. Worth an RE pass specifically on BODY_compare given its oracle value.

### [P3] Lattice (beam lattices)  (new type)
New-type over PK_LATTICE_t and its ball/rod topology (PK_LBALL/LROD/LTOPOL) — a graph of spheres joined by tapered rods, materialisable into a body. A genuine, distinct entity, but entirely outside CADabra's B-rep/analytic scope.

- `fn create_by_graph(balls:&[(Vec3,f64)], rods:&[(usize,usize,f64,f64)]) -> PsResult<Lattice>;  fn make_bodies(&self)->PsResult<Vec<Body>>;  fn part(&self)->PsResult<Part>`
  &nbsp;&nbsp;↳ `PK_LATTICE_create_by_graph`, `PK_LATTICE_make_bodies`, `PK_LATTICE_ask_part`, `PK_LATTICE_find_box`, `PK_LATTICE_find_nabox`, `PK_LATTICE_ask_n_lballs`, `PK_LATTICE_ask_n_lrods`
  &nbsp;&nbsp;· Construct from a ball/rod graph and realise into B-rep. Topology iterators (do_for_all_*) and clip/pattern deferred.
- `fn position(&self)->PsResult<Vec3>  //LBall;  fn radius(&self)->PsResult<f64>;  fn rods(&self)->..;  fn geometry(&self)->..  //LRod`
  &nbsp;&nbsp;↳ `PK_LBALL_ask_position`, `PK_LBALL_ask_radius`, `PK_LBALL_ask_lrods`, `PK_LBALL_ask_lballs_adj`, `PK_LROD_ask_geometry`, `PK_LROD_ask_lballs`, `PK_LTOPOL_ask_box`, `PK_LTOPOL_ask_class`, `PK_LTOPOL_is`
  &nbsp;&nbsp;· Ball/rod/topol read-side (plain asks); LBALL_ask_blend needs opaque o_t — defer.

  **Blocker:** Graph creation layouts ARE recovered (PK_lattice_create_by_graph_o_t 64 B, graph_cyl/cone o_t, make_bodies_o_t 16 B, find_box/nabox), so it is buildable, but it is off-roadmap. LBALL_ask_blend / clip / make_patterned consume opaque o_t/r_t stubs (PK_LBALL_ask_blend_o_t/_r_t). Deprioritise wholesale.

**Deferred groups:**
- _Threads (PK_THREAD_*) — SMP multi-threading (24)_ — Session is a !Send/!Sync single-threaded singleton by design (session.rs); the oracle runs one call at a time for determinism (TODO P9). All 24 thread fns (chain_start/stop, lock/unlock_partitions, exclusion, set_id, register_*_cbs, ask_last_error, tidy) are irrelevant to the oracle and several carry opaque stubs (PK_THREAD_id_t, _exclusion_t, _lock_t, _chain_type_t, lock_partitions_o_t/_r_t). Note: PK_THREAD_ask_last_error already faults inside the kernel (documented) — deliberately avoided.
- _Multi-partition rollback: cloning / guards / merge / copy / switching + partition-level transmit/receive/deltas (30)_ — Blocked by the documented frustrum limit: PK_PARTITION_set_current/_delete on a 2nd partition and multi-pmark navigation return error 10/5003 under the in-memory delta frustrum; a persistent delta store is required first. Covers PARTITION_merge/copy/start_cloning/stop_cloning/ask_cloning/is_clone/clone_pmark/set_guard/goto_guard/has_guard/clear_guard/find_pmark_by_id/make_pmark_2/reset_attribs/ask_transfs/ask_ki_lists/ask_appitems/ask_facet_geom, and all PARTITION/SESSION transmit/receive/receive_deltas/receive_version variants. Revisit when the persistent delta store lands.
- _Attribute & attdef callback registration + no-roll + group-closing (13)_ — Callback-driven (PK_ATTRIB_cb_f_t, PK_ATTDEF_callback_fns_t/_flags_t, PK_DEBUG_* cbs are opaque function-pointer-table stubs) and require the C callback ABI plus opaque option structs (PK_ATTRIB_ask/set_no_roll_o_t, PK_ATTDEF_is/set_group_closing_o_t, register_cb_o_t). Application-lifecycle hooks with no oracle role. Includes ATTDEF_register_callbacks/register_cb/ask_callbacks/ask_callback_flags/set_callback_flags, ATTRIB_ask/set_no_roll, ATTDEF_is/set_group_closing, ATTDEF_is_group_closing.
- _Custom frustrum / applio / indexio / polling registration (PK_SESSION_register_*) (8)_ — Advanced I/O-interception frustrum registration (register_applio/_2, register_fru_2, register_indexio, register_polling_cb, register, ask_applio/_2, ask_binding, ask_indexio) with opaque stubs (PK_SESSION_applio_t, _indexio_t, _binding_t). The default frustrum + PkArray already cover the oracle's needs (P0 done). Not needed unless custom streaming I/O is required.
- _Memory management frustrum (PK_MEMORY_*) (7)_ — alloc/free/block_size/register_callbacks are the low-level allocator frustrum. Kernel-array freeing is already handled by memory.rs (PkArray) and PK_MEMORY_free (the one wrapped). The rest is only for custom allocators — no oracle need.
- _Superseded '_2' / duplicate API variants (9)_ — Base variant should be wrapped and validated first; the _2 forms add options we don't yet need: PK_ATTDEF_ask_2, PK_GROUP_create_from_entities_2, PK_PMARK_goto_2, PK_MARK_goto_2 (already wrapped as the tracking form), PK_PART_ask_groups_2, PK_ENTITY_ask_owning_groups_2, PK_PARTITION_ask_pmarks_2, PK_INSTANCE (older forms). Wrap on demand once the base is proven.
- _AppItem (PK_APPITEM_*) application data items (5)_ — create/ask/delete/is/reset_pointers store opaque application pointer blobs on a partition; niche persistence feature with no geometric/oracle meaning. Defer indefinitely.
- _Convergent-mesh defect detection/repair, normals, hole-filling, imprinting (15)_ — Off CADabra's analytic path and blocked by opaque result/tolerance structs: PK_MESH_find_defects/_fix_defects/_find_laminar_mfins/_find_sharp_mfins/_find_sharp_mvxs/_fill_holes/_imprint_vectors/_is_loaded/_store_normals/_discard_normals/_has_unique_normals/_eval_with_mtopol plus MVERTEX_set_positions and MFIN_ask_mvx_curvature/_normal all return opaque *_r_t / consume opaque defect-tolerance o_t stubs. Wrap only if a convergent-mesh oracle becomes a goal.
