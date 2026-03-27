# Geometry, Operations, and API Surface Patterns

## 1. Geometry Creation Patterns

### Standard Forms Pattern

Geometry creation follows a consistent "standard form" (SF) pattern:
- Application populates a `PK_*_sf_t` struct with parameters
- Calls create function with const pointer to SF
- Receives entity tag via mutable out-pointer
- Error code returned as `PK_ERROR_code_t`

Examples: `PK_PLANE_create()`, `PK_SPHERE_create()`, `PK_CYLINDER_create()`

Reverse queries: `PK_PLANE_ask()`, `PK_SPHERE_ask()` extract parameters from created entities.

### Memory Ownership

- Application allocates SF structs on the stack
- Parasolid receives **const pointers** — does NOT take ownership
- After creation, Parasolid copies data internally; application can drop originals

### Array Output Convention

Functions returning arrays:
- Allocate memory via Parasolid's internal allocator
- Output signature: `n_results: *mut c_int`, `results: *mut *mut PK_ENTITY_t`
- Application must call `PK_MEMORY_free()` on output arrays when done

## 2. B-Geometry (NURBS) — Creation, Interrogation, Memory

### B-Curve Standard Form (`PK_BCURVE_sf_t`)

| Field | Type | Description |
|-------|------|-------------|
| `degree` | `c_int` | Degree of the curve (typically 2-5) |
| `n_vertices` | `c_int` | Number of control points |
| `vertex_dim` | `c_int` | 3 (non-rational) or 4 (rational homogeneous) |
| `vertices` | `*const c_double` | Control points, length = `n_vertices * vertex_dim` |
| `n_knots` | `c_int` | = `n_vertices + degree + 1` |
| `knots` | `*const c_double` | Knot vector, length = `n_knots` |
| `is_rational` | `PK_LOGICAL_t` | Whether weights are used |
| `is_periodic` | `PK_LOGICAL_t` | Periodic curve |
| `is_closed` | `PK_LOGICAL_t` | Closed curve |

### Memory Ownership for B-Geometry

- **Application allocates and owns** control point and knot arrays
- Parasolid receives **const pointers** — does NOT take ownership
- Application **must keep arrays alive** for the duration of the create call
- After creation, Parasolid copies internally; application can deallocate

### Rational NURBS

For rational curves, control points stored in 4D homogeneous form: `(x*w, y*w, z*w, w)`
- Weight extraction: `w = vertices[i*4 + 3]`
- Position: `(vertices[i*4]/w, vertices[i*4+1]/w, vertices[i*4+2]/w)`

### B-Surface (`PK_BSURF_sf_t`)

- `u_degree`, `v_degree`: separate degrees in each parametric direction
- `n_u_vertices`, `n_v_vertices`: grid dimensions
- `vertices`: column-major order, length = `n_u_vertices * n_v_vertices * vertex_dim`
- Separate knot vectors: `u_knots` (length `n_u_knots`), `v_knots` (length `n_v_knots`)

## 3. Enquiry Functions — Property Querying & Array Conventions

### Standard Enquiry Pattern

```c
pub fn PK_BODY_ask_faces(
    body: PK_BODY_t,
    n_faces: *mut c_int,        // OUT: count
    faces: *mut *mut PK_FACE_t, // OUT: pointer to PK-allocated array
) -> PK_ERROR_code_t;
```

- Count pointer is output only
- Array pointer is allocated by Parasolid — must free with `PK_MEMORY_free()`
- Null returned for empty results: `*n_faces = 0`, `*faces = nullptr`
- All topological traversals follow this pattern

### Oriented Output Pattern

Some queries return oriented pairs:
- `PK_FACE_oriented_surf_t { surf: PK_SURF_t, sense: PK_LOGICAL_t }`
- `PK_oriented_curve_t { curve: PK_CURVE_t, sense: PK_LOGICAL_t }`
- Sense indicates forward (true) or reversed (false) relative to topological entity

### Evaluation Pattern

```c
PK_CURVE_eval(curve, t, n_deriv, position: *mut c_double)
```
- Requires pre-allocated output buffer
- Buffer size: `3 * (n_deriv + 1)` doubles (position + derivatives)

```c
PK_SURF_eval(surf, uv, n_u_deriv, n_v_deriv, position: *mut c_double)
```
- Similar buffer semantics

Inverse: `PK_CURVE_parameterise_vector()`, `PK_SURF_parameterise_vector()` find parameters given points.

## 4. Boolean Operations — Input/Output Patterns & Body Consumption

### Global Boolean Pattern

```c
pub fn PK_BODY_boolean_2(
    target: PK_BODY_t,          // IN: modified in-place
    n_tools: c_int,
    tools: *const PK_BODY_t,    // IN: destroyed after operation
    function: PK_boolean_function_t,
    options: *const PK_BODY_boolean_2_o_t,
    result_bodies: *mut *mut PK_BODY_t,
    n_result_bodies: *mut c_int,
) -> PK_ERROR_code_t;
```

### Consumption Semantics

- **Target body**: modified in-place and **remains valid** (not consumed)
- **Tool bodies**: destroyed/consumed — application must NOT access afterward
- **Result bodies**: newly allocated array

### Specialized Manifold Booleans

- `PK_BODY_unite_bodies()`, `PK_BODY_subtract_bodies()`, `PK_BODY_intersect_bodies()`
- Accept arrays of tool bodies — all destroyed after operation
- Return single result body or array of results

### Boolean Options

- `match_style`: `basic_c`, `auto_c`, `relax_c` — how to match topologies
- `tool_material_side`, `target_material_side`: solid/sheet/none semantics
- `resulting_body_type`: prefer original, solid, sheet, wire, general, or simplest result
- `keep_tools`: if true, tools are NOT destroyed (copied first)

## 5. File I/O — Transmit/Receive with Frustrum Callbacks

### Transmit (Writing)

```c
pub fn PK_PART_transmit(
    n_parts: c_int,
    parts: *const PK_PART_t,
    key: *const c_char,       // File path or frustrum identifier
    options: *const PK_PART_transmit_o_t,
) -> PK_ERROR_code_t;
```

### Receive (Reading)

```c
pub fn PK_PART_receive(
    key: *const c_char,
    options: *const PK_PART_receive_o_t,
    n_parts: *mut c_int,
    parts: *mut *mut PK_PART_t,  // PK-allocated array
) -> PK_ERROR_code_t;
```

### Memory I/O Variants

- `PK_PART_transmit_b()` — transmit to application memory buffer
- `PK_PART_receive_b()` — receive from application memory buffer
- Output: `n_bytes: *mut c_int`, `buffer: *mut *mut c_void`

### Partition vs. Part I/O

- **Parts**: individual modeling data with assembly structure
- **Partitions**: entire session state (all bodies, attributes, history)
- `PK_PARTITION_transmit()` / `PK_PARTITION_receive()` for full snapshots

## 6. Faceting — Mesh Output

### Tabular Faceting

```c
pub fn PK_TOPOL_facet_2(
    body: PK_BODY_t,
    options: *const PK_TOPOL_facet_2_o_t,
    result: *mut PK_TOPOL_facet_2_r_t,
) -> PK_ERROR_code_t;
```

Result contains arrays of facet indices, fin indices, vertex coordinates, etc. All PK-allocated — must free with `PK_MEMORY_free()`.

### GO-Based Faceting (Callback)

- Output through callback functions, not return arrays
- Application registers GO callbacks: `GOOPSG()`, `GOSGMT()`, `GOCLSG()`
- Parasolid calls callbacks as it generates facet data

### Faceting Options

```c
pub struct PK_TOPOL_facet_mesh_2_o_t {
    pub surface_tolerance: c_double,
    pub curve_tolerance: c_double,
    pub max_facet_size: c_double,
    pub shape: PK_facet_shape_t,     // convex, cut, any
    pub match_type: PK_facet_match_t, // topol, geom, trimmed
}
```

## 7. Assembly & Instance Model

### Instance Creation

```c
pub fn PK_INSTANCE_create(
    assembly: PK_ASSEMBLY_t,
    part: PK_PART_t,        // Body or assembly
    transf: PK_TRANSF_t,    // Null = identity
    instance: *mut PK_INSTANCE_t,
) -> PK_ERROR_code_t;
```

### Key Properties

- Instances reference parts (bodies or sub-assemblies) within a parent assembly
- Each instance has a rigid transform (translation, rotation, reflection — no scaling)
- Instance traversal: `PK_ASSEMBLY_ask_instances()`, `PK_INSTANCE_ask()` returns `(assembly, part, transf)` triple
- Instances cannot exist outside an assembly; the part must be non-null
- Assembly graphs must be **acyclic** (no self-referencing)

### Transmit/Receive

- Top-level assemblies transmitted with all sub-parts down to bodies
- Instances and transforms automatically included
- On receive, assembly structure fully reconstructed

## 8. Common API Patterns for Safe Rust Wrapper

### Pattern 1: Output Array RAII

```rust
pub fn ask_faces(body: &Body) -> Result<Vec<Face>> {
    let mut n_faces = 0;
    let mut faces_ptr = std::ptr::null_mut();
    pk_call!(PK_BODY_ask_faces(body.tag, &mut n_faces, &mut faces_ptr));
    let slice = unsafe { std::slice::from_raw_parts(faces_ptr, n_faces as usize) };
    let result = slice.iter().map(|&tag| Face { tag }).collect();
    if n_faces > 0 {
        unsafe { PK_MEMORY_free(faces_ptr as *mut c_void); }
    }
    Ok(result)
}
```

### Pattern 2: Options Builder

```rust
pub struct BooleanOptions {
    pub tolerance: f64,
    pub keep_tools: bool,
    pub match_style: MatchStyle,
    // ...
}

impl Default for BooleanOptions {
    fn default() -> Self {
        // Mirror PK_BODY_boolean_2_o_m() defaults
        Self { tolerance: 1e-6, keep_tools: false, match_style: MatchStyle::Auto }
    }
}

impl BooleanOptions {
    pub fn tolerance(mut self, tol: f64) -> Self { self.tolerance = tol; self }
    pub fn keep_tools(mut self, keep: bool) -> Self { self.keep_tools = keep; self }
}
```

### Pattern 3: Standard Form Builder

```rust
pub struct PlaneBuilder {
    location: [f64; 3],
    axis: [f64; 3],
    ref_direction: [f64; 3],
}

impl PlaneBuilder {
    pub fn build(self) -> Result<Plane> {
        let sf = PK_PLANE_sf_t { /* fill from self */ };
        let mut tag = 0;
        pk_call!(PK_PLANE_create(&sf, &mut tag));
        Ok(Plane { tag })
    }
}
```

### Pattern 4: Body Consumption (Booleans)

```rust
pub fn boolean(target: &mut Body, tools: Vec<Body>, opts: &BooleanOptions) -> Result<Vec<Body>> {
    let tool_tags: Vec<_> = tools.iter().map(|b| b.tag).collect();
    // tools are moved in and dropped — cannot be used after
    let mut n_results = 0;
    let mut results_ptr = std::ptr::null_mut();
    pk_call!(PK_BODY_boolean_2(target.tag, ...));
    // Convert results...
    std::mem::forget(tools); // Prevent Drop from trying to delete consumed bodies
    Ok(results)
}
```

### Pattern 5: Evaluation Buffer

```rust
pub fn curve_eval(curve: &Curve, t: f64, n_deriv: usize) -> Result<Vec<[f64; 3]>> {
    let mut buffer = vec![0.0; 3 * (n_deriv + 1)];
    pk_call!(PK_CURVE_eval(curve.tag, t, n_deriv as c_int, buffer.as_mut_ptr()));
    Ok(buffer.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect())
}
```

### Pattern 6: Transmit/Receive

```rust
pub fn transmit_to_file(parts: &[&Part], path: &str) -> Result<()> {
    let tags: Vec<_> = parts.iter().map(|p| p.tag).collect();
    let c_path = CString::new(path)?;
    let opts = PK_PART_transmit_o_t::default();
    pk_call!(PK_PART_transmit(tags.len() as c_int, tags.as_ptr(), c_path.as_ptr(), &opts));
    Ok(())
}

pub fn receive_from_file(path: &str) -> Result<Vec<Part>> {
    let c_path = CString::new(path)?;
    let opts = PK_PART_receive_o_t::default();
    let mut n_parts = 0;
    let mut parts_ptr = std::ptr::null_mut();
    pk_call!(PK_PART_receive(c_path.as_ptr(), &opts, &mut n_parts, &mut parts_ptr));
    // Convert + free...
    Ok(parts)
}
```

## Summary: Key Wrapper Considerations

1. **Array allocation**: Parasolid allocates output arrays; must free with `PK_MEMORY_free()`
2. **Body consumption**: Boolean operations destroy tool bodies; enforce move semantics
3. **Standard forms**: Application owns SF structs; PK copies internally
4. **Control point memory**: B-geometry receives const pointers; application retains ownership
5. **Options defaults**: Must replicate `PK_*_o_m()` macro defaults
6. **Evaluation buffers**: Pre-allocated by application, sized by derivative count
7. **Oriented output**: Query functions may return (entity, orientation) pairs
8. **Error handling**: All FFI calls return error codes; wrap with `Result`
9. **Callback system**: GO faceting uses callbacks; provide Rust trait abstraction
10. **Const correctness**: Many FFI functions take const pointers; safe to borrow in Rust
