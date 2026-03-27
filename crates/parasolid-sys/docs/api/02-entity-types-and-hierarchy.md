# Entity Type System, Class Hierarchy, and Topology Model

## 1. The PK Entity Tag System

### What Tags Are

**Tags** (type `PK_ENTITY_t`, which is `c_int`) are opaque integer handles that uniquely identify entities within a Parasolid session.

- **Session-unique**: Each tag is unique within a single Parasolid session; tag numbers are never reused
- **Kernel-created**: Tags are created only by the Parasolid kernel when entities are created
- **Non-persistent**: Tags are NOT consistent across different sessions (the same model loaded in different sessions gets different tags)
- **Null tag**: `PK_ENTITY_null` (value 0) represents "no entity"
- **Arbitrary arithmetic**: Tags have no meaning if subjected to arithmetic operations — they must be obtained from kernel functions

### Lifetime Rules (CRITICAL for Safe Wrapper)

A tag is **alive** (has meaning) only while the entity to which it refers still exists in internal memory.

Tags become **dead** (invalid) when the entity is:
- Explicitly deleted via `PK_ENTITY_delete()`
- Implicitly deleted (e.g., superfluous edges merged away during model simplification)
- Deleted by rollback (when rolling back to a point before the entity was created)
- Lost when the modeling session stops via `PK_SESSION_stop()`

Using dead tags causes the error `PK_ERROR_not_an_entity` (error code 504) if argument checking is enabled.

### Tag Validation Functions

- `PK_ENTITY_is()` — test if a tag is alive
- `PK_ENTITY_is_topol()` — test if alive AND topological
- `PK_ENTITY_is_geom()` — test if alive AND geometric

### Tag Persistence Rules

When modeling operations create, delete, or modify entities:

- **Face/Edge shrinking**: If a face/edge is truncated, its tag persists (original entity becomes smaller)
- **Face/Edge splitting**: When split into multiple faces/edges, one result keeps the original tag; all others get new tags
- **Entity merging**: When multiple entities merge into one, the result keeps the tag of one original entity (operation-dependent)
- **New entity creation**: Genuinely new entities always get new, unique tags

**Design note**: Applications should NOT rely on tag persistence for tracking; instead, use explicit tracking returns from functions, attributes, or the Parasolid report mechanism.

## 2. The Class Hierarchy

### Top-Level Hierarchy

```
CLASS (root)
├── PARTITION, PMARK, APPITEM, MARK (items with tags, non-model)
├── ENTITY (model data with tags)
│   ├── ATTDEF, ATTRIB, GROUP (attributes & grouping)
│   ├── GEOM (geometric entities)
│   └── TOPOL (topological entities)
├── MEMORY, SESSION, ERROR (system)
└── PRIMITIVE (base types)
```

### The TOPOL (Topological) Hierarchy

```
TOPOL
├── INSTANCE (references another part with transform)
├── LOOP (connected face boundary component)
├── FACE (bounded 2D surface subset)
├── FIN (oriented edge usage by a loop)
├── SHELL (connected collection of faces/edges)
├── EDGE (bounded 1D curve)
├── REGION (connected 3D spatial subset)
└── PART (body or assembly)
    ├── ASSEMBLY (collection of instances)
    └── BODY (primary modeling entity)
```

### The GEOM (Geometric) Hierarchy

```
GEOM
├── SURF (surface)
│   ├── PLANE, SPHERE, CONE, CYL, TORUS
│   ├── BSURF (B-spline surface)
│   ├── OFFSET, SWEPT, SPUN
│   ├── FSURF (fitted surface)
│   ├── MESH (faceted/convergent geometry)
│   └── BLENDSF, SSURF
├── CURVE (curve)
│   ├── LINE, CIRCLE, ELLIPSE
│   ├── BCURVE (B-spline curve)
│   ├── ICURVE (intersection curve)
│   ├── FCURVE (fitted curve)
│   ├── SCURVE (surface-parameter curve)
│   ├── TCURVE (trimmed curve)
│   ├── CPCURVE (contact-point curve)
│   └── PLINE (polyline)
└── POINT (Cartesian point)
```

### Querying the Hierarchy

- `PK_ENTITY_ask_class(entity, &class_token)` — returns the class token
- `PK_CLASS_ask_superclass(class, &super)` — find parent class
- `PK_CLASS_is_subclass(class1, class2, &answer)` — test inheritance

### Geometric Entity Attachment

Geometric entities may be:
- **Principal geometry**: Attached to faces (surfaces), edges/fins (curves), vertices (points)
- **Construction geometry**: Attached to a body for reference
- **Orphan geometry**: Not attached to any topological entity; attached to the current partition

## 3. Body Types

### Four Manifold Body Types (Increasing Complexity)

**1. Acorn Bodies (0-dimensional)**
- Simplest type: isolated vertices (points in space)
- Minimum body: single void region + single shell + single acorn vertex (with point attached)

**2. Wire Bodies (1-dimensional)**
- Connected edges without faces
- Components: open wire (two end vertices) or closed wire (all vertices degree 2)
- Single void region, one shell per component
- No faces, no acorn vertices

**3. Sheet Bodies (2-dimensional)**
- Open surfaces or closed shells (hollow spheres, tori, etc.)
- Must contain at least one face; no solid regions
- Edge types:
  - **Normal**: exactly 2 fins with opposite senses (manifold)
  - **Laminar**: exactly 1 fin (boundary edge, open sheet only)
- No acorn vertices, no wireframe edges

**4. Solid Bodies (3-dimensional)**
- Enclose finite volume; continuous per component
- Strict topological closure:
  - Every face has solid region on one side, void region on other
  - Face normals point away from solid
- Every edge has exactly 2 fins with opposite senses (strictly manifold)
- No acorn vertices, no wireframe edges

### General Bodies (Mixed-Dimension)

- Can contain any combination of 0D, 1D, 2D, 3D components
- Allow non-manifold topology
- Single body can have disjoint components

### Compound Bodies

- Container for child bodies that share geometry
- Configuration returned by `PK_BODY_ask_config()`:
  - `PK_BODY_config_standard_c` — normal body
  - `PK_BODY_config_compound_c` — container
  - `PK_BODY_config_child_c` — member of compound

## 4. Topological Data Structure

### Full Hierarchy

```
BODY
 ├─ REGION (connected 3D space subset: solid or void)
 │   └─ SHELL (connected face & edge collection)
 │       ├─ FACE (bounded 2D surface)
 │       │   ├─ LOOP (face boundary component)
 │       │   │   └─ FIN (oriented edge usage)
 │       │   │       └─ EDGE (1D curve segment)
 │       │   │           ├─ VERTEX (start)
 │       │   │           └─ VERTEX (end)
 │       │   └─ SURFACE (principal geometry)
 │       └─ EDGE
 │           └─ CURVE (principal geometry)
 └─ VERTEX
     └─ POINT (principal geometry)
```

### Entity Definitions

**REGION**: A connected subset of 3D space bounded by shells.
- **Solid regions**: Bounded interior volume (finite)
- **Void regions**: Unbounded exterior (infinite, always one per body)

**SHELL**: A connected collection of oriented faces and edges.
- Face shells (normal), wire shells (edge-only), acorn shells (single vertex)

**FACE**: A bounded subset of a surface.
- Boundary is collection of loops (0 loops = closed surface like sphere)
- Oriented: normal direction specified

**LOOP**: A connected component of a face boundary.
- Types: outer, inner (holes), winding (cylindrical topology), vertex (isolated)

**FIN**: The oriented use of an edge by a loop.
- Has orientation (positive/negative sense)
- Can have associated local-precision curve

**EDGE**: A bounded piece of a curve.
- Types: wireframe (0 fins), laminar (1 fin), normal (2 fins opposite sense), general (2 fins same sense)

**VERTEX**: A point in 3D space.
- Has attached POINT (principal geometry) or null

### Connectivity Navigation

From BODY:
- `PK_BODY_ask_faces()`, `ask_edges()`, `ask_vertices()`, `ask_shells()`, `ask_regions()`, `ask_loops()`, `ask_fins()`

From FACE:
- `PK_FACE_ask_body()`, `ask_edges()`, `ask_loops()`, `ask_vertices()`, `ask_shells()`, `ask_surf()`, `ask_first_loop()`

From EDGE:
- `PK_EDGE_ask_body()`, `ask_vertices()`, `ask_faces()`, `ask_fins()`, `ask_curve()`, `ask_type()`

From FIN:
- `PK_FIN_ask_edge()`, `ask_loop()`, `ask_face()`, `ask_body()`, `ask_next_in_loop()`, `ask_next_of_edge()` (radial)

From LOOP:
- `PK_LOOP_ask_body()`, `ask_face()`, `ask_fins()`, `ask_edges()`, `ask_vertices()`, `ask_type()`

## 5. Entity Relationships to Partitions

### Ownership Rules

- Every entity belongs to exactly one partition
- Created entities enter the "current partition"
- Can be moved to different partitions
- Partition deletion cascades to contained entities

### Orphan Geometry

- Geometric entities created without attachment to topology are initially attached to current partition
- Can later be attached to a body as construction geometry

### Copying Behavior

`PK_ENTITY_copy_2()` options for destination:
- `PK_ITEM_null` — same partition as original (default)
- `PK_PARTITION_t` — specific destination partition
- `PK_PART_t` — attach to specific body

## 6. Tracking and Labelling

### Tracking Mechanisms

1. **Explicit tracking returns** from modeling functions (primary approach)
2. **Attributes with callbacks**: Custom attributes on entities + callbacks on modification
3. **Identifiers**: Unique integers per-part (NOT tags), persist in archives
4. **Report mechanism**: Some functions use Parasolid's report system
5. **User fields**: Application-provided storage on entities

### Tag Persistence Strategy

Applications should NOT rely on:
- Tag persistence across operations
- Tag identity for entity matching

Instead use:
- **Tracking returns** from functions
- **Identifiers** (`PK_ENTITY_ask_identifier()`)
- **Attributes** for marking entities
- **Connectivity queries** before and after operations

## 7. Options-Structure Pattern (PK_*_o_t)

### Pattern Overview

Many Parasolid functions accept optional arguments via options structures:
- Struct name convention: `PK_<FUNCTION>_o_t`
- Initialization macro: `PK_<FUNCTION>_o_m(options)` — sets defaults
- Every field must be specified explicitly
- Version field `o_t_version` is auto-managed (do NOT modify)

### Example

```c
PK_ENTITY_copy_2_o_t options;
PK_ENTITY_copy_2_o_m(options);  // Initialize with defaults
options.want_tracking = PK_LOGICAL_true;
options.want_attribs = PK_LOGICAL_true;
PK_ENTITY_t copy;
PK_ENTITY_copy_2(original, options, &copy, &tracking_info);
```

## 8. Key Type-Safety Invariants for Rust Wrapper

| Invariant | Mechanism |
|-----------|-----------|
| Tags only used while alive | Session lifetime bounds all entities via `PhantomData<&'s Session>` |
| Class hierarchy respected | Newtype per concrete class (Body, Face, Edge, etc.) |
| Every option field initialized | Builder pattern / `Default` impl matching `PK_*_o_m()` |
| Memory freed properly | RAII wrappers around PK-allocated arrays |
| Error codes checked | `Result<T, Error>` return type on all operations |
| Topological consistency | Type parameters for body types (Solid, Sheet, Wire) |
| Entity references valid | Partition lifetime parameter |
| Tracking information preserved | Returned in structured `TrackingInfo` types |
