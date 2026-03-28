# Missing FFI Functions

**Total: 221 functions** (135 operational + 86 result-free `_r_f` variants)

These are Parasolid C API functions found in the pskernel.dll export table that
are not yet bound in parasolid-sys. Signatures are sourced from V35 online docs,
V12 docs, and reverse engineering (Ghidra).

## The `_r_f` Pattern

Functions ending in `_r_f` are **result-free** helpers. Every function that
returns a heap-allocated result struct `PK_xxx_r_t` has a companion:

```c
PK_ERROR_code_t PK_xxx_r_f(PK_xxx_r_t *results);
```

These free the memory allocated inside the result struct. They are trivial
wrappers around `PK_MEMORY_free` and share implementations when the result
struct layout is identical. All 86 `_r_f` functions are listed in
[Appendix A](#appendix-a-result-free-_r_f-functions) at the end.

---

## 1. Body Operations (26 functions)

### PK_BODY_create_implicit -- Create implicit/procedural body
  Sig: `PK_ERROR_code_t PK_BODY_create_implicit(const PK_BODY_create_implicit_o_t *options, PK_BODY_create_implicit_r_t *const results)`
  Source: RE (export addr 0x18010af70)

### PK_BODY_create_minimum_topology -- Create topology of a minimum body
  Sig: `PK_ERROR_code_t PK_BODY_create_minimum_topology(int n_topols, const PK_CLASS_t classes[], int n_relations, const int parents[], const int children[], const PK_TOPOL_sense_t senses[], PK_BODY_t *const body, PK_TOPOL_t *const topols, PK_BODY_fault_t *const fault, int *const fault_index)`
  Source: V35 docs

### PK_BODY_create_sheet_topology -- Create topology of a sheet body
  Sig: `PK_ERROR_code_t PK_BODY_create_sheet_topology(int n_topols, const PK_CLASS_t classes[], int n_relations, const int parents[], const int children[], const PK_TOPOL_sense_t senses[], PK_BODY_t *const body, PK_TOPOL_t *const topols, PK_BODY_fault_t *const fault, int *const fault_index)`
  Source: V35 docs

### PK_BODY_create_solid_topology -- Create topology of a solid body
  Sig: `PK_ERROR_code_t PK_BODY_create_solid_topology(int n_topols, const PK_CLASS_t classes[], int n_relations, const int parents[], const int children[], const PK_TOPOL_sense_t senses[], PK_BODY_t *const body, PK_TOPOL_t *const topols, PK_BODY_fault_t *const fault, int *const fault_index)`
  Source: V35 docs

### PK_BODY_create_wire_topology -- Create topology of a wire body
  Sig: `PK_ERROR_code_t PK_BODY_create_wire_topology(int n_topols, const PK_CLASS_t classes[], int n_relations, const int parents[], const int children[], const PK_TOPOL_sense_t senses[], PK_BODY_t *const body, PK_TOPOL_t *const topols, PK_BODY_fault_t *const fault, int *const fault_index)`
  Source: V35 docs

### PK_BODY_create_topology -- Create body topology with options
  Sig: `PK_ERROR_code_t PK_BODY_create_topology(int n_topols, const PK_CLASS_t classes[], int n_relations, const int parents[], const int children[], const PK_TOPOL_sense_t senses[], const PK_BODY_create_topology_o_t *options, PK_BODY_t *const body, PK_TOPOL_t *const topols, PK_BODY_fault_t *const fault, int *const fault_index)`
  Source: V35 docs

### PK_BODY_is_cellular -- Query whether body has cellular structure
  Sig: `PK_ERROR_code_t PK_BODY_is_cellular(PK_BODY_t body, const PK_BODY_is_cellular_o_t *options, PK_BODY_is_cellular_r_t *const results)`
  Source: RE (export addr 0x1800e0f00)

### PK_BODY_is_disjoint -- Query whether body contains disjoint shells
  Sig: `PK_ERROR_code_t PK_BODY_is_disjoint(PK_BODY_t body, const PK_BODY_is_disjoint_o_t *options, PK_BODY_is_disjoint_r_t *const results)`
  Source: RE (export addr 0x1800e17f0)

### PK_BODY_enlarge -- Scale body by factor with transform
  Sig: `PK_ERROR_code_t PK_BODY_enlarge(PK_BODY_t body, PK_scale_factor_t factor, PK_TRANSF_t transf, double tolerance, const PK_BODY_enlarge_o_t *options, PK_BODY_enlarge_r_t *const returns)`
  Source: V35 docs

### PK_BODY_slice -- Slice body with sheet tool
  Sig: `PK_ERROR_code_t PK_BODY_slice(PK_BODY_t body, PK_BODY_t tool, const PK_BODY_slice_o_t *options, PK_BODY_slice_r_t *const results)`
  Source: RE (export addr 0x18019fe40)

### PK_BODY_make_patterned -- Create patterned lattice-like mesh body from facet body
  Sig: `PK_ERROR_code_t PK_BODY_make_patterned(PK_BODY_t body, double tolerance, const PK_BODY_make_patterned_o_t *options, PK_BODY_make_patterned_r_t *const results)`
  Source: V35 docs

### PK_BODY_make_swept_profiles -- Create swept body from profiles along path
  Sig: `PK_ERROR_code_t PK_BODY_make_swept_profiles(int n_profiles, const PK_BODY_t profiles[], const PK_BODY_t path, const PK_BODY_make_swept_profiles_o_t *options, PK_BODY_tracked_sweep_2_r_t *const results)`
  Source: RE (export addr 0x18018edc0)

### PK_BODY_make_swept_body_2 -- Create swept body from profiles at path vertices
  Sig: `PK_ERROR_code_t PK_BODY_make_swept_body_2(int n_profiles, const PK_BODY_t profiles[], const PK_BODY_t path, const PK_VERTEX_t path_vertices[], const PK_BODY_make_swept_body_2_o_t *options, PK_BODY_tracked_sweep_2_r_t *const swept_body)`
  Source: V35 docs

### PK_BODY_make_swept_tool -- Create swept tool body
  Sig: `PK_ERROR_code_t PK_BODY_make_swept_tool(const PK_BODY_t tool, const PK_AXIS1_sf_t *tool_axis, const PK_BODY_t path, const PK_BODY_make_swept_tool_o_t *options, PK_TOPOL_track_r_t *const tracking, PK_BODY_sweep_tool_r_t *const swept_tool)`
  Source: V35 docs

### PK_BODY_boolean -- Boolean operation (obsolete, superseded by _2)
  Sig: `PK_ERROR_code_t PK_BODY_boolean(PK_BODY_t target, int n_tools, const PK_BODY_t tools[], const PK_BODY_boolean_o_t *options, int *const n_bodies, PK_BODY_t **const bodies)`
  Source: V35 docs

### PK_BODY_offset -- Offset faces of solid/sheet body
  Sig: `PK_ERROR_code_t PK_BODY_offset(PK_BODY_t body, double offset, double tolerance, PK_LOGICAL_t face_face_check)`
  Source: V35 docs

### PK_BODY_hollow -- Hollow solid body by offsetting all faces
  Sig: `PK_ERROR_code_t PK_BODY_hollow(PK_BODY_t body, double offset, double tolerance, PK_LOGICAL_t face_face_check, int *const n_faces, PK_FACE_t **const old_faces, PK_FACE_t **const new_faces)`
  Source: V35 docs

### PK_BODY_thicken -- Thicken sheet body into solid (obsolete, superseded by _2)
  Sig: `PK_ERROR_code_t PK_BODY_thicken(PK_BODY_t body, double front, double back, double tolerance, PK_LOGICAL_t face_face_check, int *const n_topols, PK_TOPOL_t **const old_topols, PK_TOPOL_t **const new_topols)`
  Source: V35 docs

### PK_BODY_thicken_2 -- Thicken sheet body into solid (current)
  Sig: `PK_ERROR_code_t PK_BODY_thicken_2(PK_BODY_t body, double front_default, double back_default, double tolerance, const PK_BODY_thicken_o_t *options, PK_TOPOL_track_r_t *const tracking, PK_BODY_thicken_r_t *const results)`
  Source: V35 docs

### PK_BODY_transform -- Transform body by transformation matrix
  Sig: `PK_ERROR_code_t PK_BODY_transform(PK_BODY_t body, PK_TRANSF_t transf, double tolerance, int *const n_replaces, PK_GEOM_t **const replaces, PK_LOGICAL_t **const exact)`
  Source: V35 docs

### PK_BODY_section_with_sheet -- Section body with sheet (obsolete, superseded by _2)
  Sig: `PK_ERROR_code_t PK_BODY_section_with_sheet(PK_BODY_t target, PK_BODY_t sheet, const PK_BODY_section_o_t *options, PK_section_r_t *const results)`
  Source: V35 docs

### PK_BODY_trim_neutral_sheets -- Trim neutral sheets by face-set pairs
  Sig: `PK_ERROR_code_t PK_BODY_trim_neutral_sheets(PK_BODY_t body, int n_pairs, const PK_FACE_set_pair_t pairs[], double tol, PK_BODY_t neutral_sheets[], PK_neutral_error_t errors[], PK_FACE_neutral_causes_array_t causes[])`
  Source: V35 docs

### PK_BODY_ask_frames -- Return all frames belonging to a body
  Sig: `PK_ERROR_code_t PK_BODY_ask_frames(PK_BODY_t body, int *n_frames, PK_FRAME_t **frames)`
  Source: RE (addr 0x1800f54e0)

---

## 2. Face Operations (15 functions)

### PK_FACE_boolean -- Boolean on face subsets (obsolete, superseded by _2)
  Sig: `PK_ERROR_code_t PK_FACE_boolean(int n_targets, const PK_FACE_t targets[], int n_tools, const PK_FACE_t tools[], const PK_FACE_boolean_o_t *options, int *const n_bodies, PK_BODY_t **const bodies)`
  Source: V35 docs

### PK_FACE_delete -- Delete faces and repair holes
  Sig: `PK_ERROR_code_t PK_FACE_delete(int n_faces, const PK_FACE_t faces[], PK_FACE_heal_t heal_action, PK_LOGICAL_t loops_independent, PK_LOGICAL_t local_check, int *const n_bodies, PK_BODY_t **const bodies, PK_local_check_t **const check_results)`
  Source: V35 docs

### PK_FACE_hollow -- Hollow body by offsetting specific faces (obsolete, superseded by _2)
  Sig: `PK_ERROR_code_t PK_FACE_hollow(int n_faces, PK_FACE_t faces[], double offsets[], double tolerance, PK_LOGICAL_t face_face_check, PK_FACE_t *const new_faces)`
  Source: V35 docs

### PK_FACE_hollow_2 -- Hollow body by offsetting specific faces (current)
  Sig: `PK_ERROR_code_t PK_FACE_hollow_2(int n_faces, PK_FACE_t faces[], double offsets[], double tolerance, PK_LOGICAL_t face_face_check, int *const n_new_faces, PK_FACE_t **const old_faces, PK_FACE_t **const new_faces)`
  Source: V35 docs

### PK_FACE_imprint_faces -- Imprint edges on target and tool faces
  Sig: `PK_ERROR_code_t PK_FACE_imprint_faces(int n_targets, const PK_FACE_t targets[], int n_tools, const PK_FACE_t tools[], const PK_FACE_imprint_o_t *options, PK_imprint_r_t *const results)`
  Source: V35 docs

### PK_FACE_make_neutral_sheet -- Create neutral sheet from two faces
  Sig: `PK_ERROR_code_t PK_FACE_make_neutral_sheet(PK_FACE_t faces[2], double placement, PK_BODY_t *const neutral_sheet)`
  Source: V35 docs

### PK_FACE_repair -- Repair invalid face (split G1 discontinuities, self-intersections)
  Sig: `PK_ERROR_code_t PK_FACE_repair(PK_FACE_t face, const PK_FACE_repair_o_t *options, PK_TOPOL_track_r_t *const tracking)`
  Source: V35 docs

### PK_FACE_make_valid_faces -- Create valid face topology from faces
  Sig: `PK_ERROR_code_t PK_FACE_make_valid_faces(int n_faces, const PK_FACE_t faces[], const PK_FACE_make_valid_faces_o_t *options, PK_FACE_make_valid_faces_r_t *const results)`
  Source: RE (export addr 0x180292d80)

### PK_FACE_make_sheet_body -- Create sheet body from faces
  Sig: `PK_ERROR_code_t PK_FACE_make_sheet_body(int n_faces, const PK_FACE_t faces[], PK_BODY_t *const body)`
  Source: V35 docs

### PK_FACE_section_with_sheet -- Section faces with sheet (obsolete, superseded by _2)
  Sig: `PK_ERROR_code_t PK_FACE_section_with_sheet(int n_targets, const PK_FACE_t targets[], int n_tools, const PK_FACE_t tools[], const PK_FACE_section_o_t *options, PK_section_r_t *const results)`
  Source: V35 docs

### PK_FACE_pattern -- Pattern faces by transforms (V1)
  Sig: `PK_ERROR_code_t PK_FACE_pattern(int n_pattern_faces, const PK_FACE_t pattern_faces[], int n_transforms, const PK_TRANSF_t transforms[], const PK_FACE_pattern_o_t *options, PK_FACE_pattern_r_t *const pattern_results)`
  Source: V35 docs

### PK_FACE_pattern_2 -- Pattern faces by transforms (V2)
  Sig: `PK_ERROR_code_t PK_FACE_pattern_2(int n_pattern_faces, const PK_FACE_t pattern_faces[], int n_transforms, const PK_TRANSF_t transforms[], const PK_FACE_pattern_2_o_t *options, PK_FACE_pattern_2_r_t *const results)`
  Source: RE (export addr 0x180306820)

### PK_FACE_fix_mesh_defects -- Fix mesh defects on facet faces
  Sig: `PK_ERROR_code_t PK_FACE_fix_mesh_defects(int n_faces, const PK_FACE_t faces[], const PK_FACE_fix_mesh_defects_o_t *options, PK_ENTITY_track_r_t *const tracking, PK_FACE_fix_mesh_defects_r_t *const results)`
  Source: V35 docs

### PK_FACE_install_surfs_isocline -- Replace face surfaces with isocline surfaces
  Sig: `PK_ERROR_code_t PK_FACE_install_surfs_isocline(int n_faces, const PK_FACE_t faces[], const PK_ENTITY_t references[], PK_VECTOR1_t direction, double angle, double tolerance, PK_LOGICAL_t face_face_check)`
  Source: V35 docs

### PK_FACE_imprint_cus_vector -- Imprint curves on faces by vector projection
  Sig: `PK_ERROR_code_t PK_FACE_imprint_cus_vector(int n_targets, const PK_FACE_t targets[], int n_curves, const PK_CURVE_t curves[], PK_INTERVAL_t intervals[], PK_VECTOR_t direction, double tol, const PK_FACE_imprint_cus_vector_o_t *options, PK_TOPOL_track_r_t *const tracking)`
  Source: V35 docs

---

## 3. Edge Operations (4 functions)

### PK_EDGE_reverse -- Reverse edge and its geometry
  Sig: `PK_ERROR_code_t PK_EDGE_reverse(PK_EDGE_t edge)`
  Source: V35 docs

### PK_EDGE_find_deviation -- Compute distances between two edges (deprecated by _2)
  Sig: `PK_ERROR_code_t PK_EDGE_find_deviation(PK_EDGE_t edge1, PK_EDGE_t edge2, int how_many, int *const n_distances, double **const distances, PK_VECTOR_t **const edge1_vecs, PK_VECTOR_t **const edge2_vecs)`
  Source: V35 docs

### PK_EDGE_contains_vector -- Test if position coincides with edge
  Sig: `PK_ERROR_code_t PK_EDGE_contains_vector(PK_EDGE_t edge, PK_VECTOR_t vector, PK_TOPOL_t *const topol)`
  Source: V35 docs

### PK_EDGE_find_end_tangents -- Find end positions and tangent directions
  Sig: `PK_ERROR_code_t PK_EDGE_find_end_tangents(PK_EDGE_t edge, PK_VECTOR_t *const start, PK_VECTOR_t *const start_tangent, PK_VECTOR_t *const end, PK_VECTOR_t *const end_tangent)`
  Source: V35 docs

---

## 4. Entity Operations (4 functions)

### PK_ENTITY_copy_2 -- Copy entity with tracking
  Sig: `PK_ERROR_code_t PK_ENTITY_copy_2(PK_ENTITY_t entity, const PK_ENTITY_copy_o_t *options, PK_ENTITY_t *const entity_copy, PK_ENTITY_track_r_t *const tracking)`
  Source: V35 docs

### PK_ENTITY_ask_description -- Return textual description of entity internals
  Sig: `PK_ERROR_code_t PK_ENTITY_ask_description(int tag, const PK_ENTITY_ask_description_o_t *options, char **const description)`
  Source: V35 docs

### PK_ENTITY_range -- Min/max separation between two entity arrays
  Sig: `PK_ERROR_code_t PK_ENTITY_range(int n_entities_1, const PK_ENTITY_t entities_1[], const PK_TRANSF_t tf_1[], int n_entities_2, const PK_ENTITY_t entities_2[], const PK_TRANSF_t tf_2[], const PK_ENTITY_range_o_t *options, PK_ENTITY_range_r_t *const results)`
  Source: V35 docs

### PK_ENTITY_range_vector -- Min separation between entities and positions
  Sig: `PK_ERROR_code_t PK_ENTITY_range_vector(int n_entities, const PK_ENTITY_t entities[], const PK_TRANSF_t tf[], int n_vectors, const PK_VECTOR_t vectors[], const PK_ENTITY_range_vector_o_t *options, PK_ENTITY_range_vector_r_t *const results)`
  Source: V35 docs

---

## 5. Curve Operations (10 functions)

### PK_CURVE_find_length -- Arc length over parametric interval
  Sig: `PK_ERROR_code_t PK_CURVE_find_length(PK_CURVE_t curve, PK_INTERVAL_t interval, double *const length, PK_INTERVAL_t *const range)`
  Source: V35 docs

### PK_CURVE_ask_parm_different -- Check if PK vs KI parametrisation differs
  Sig: `PK_ERROR_code_t PK_CURVE_ask_parm_different(PK_CURVE_t curve, PK_LOGICAL_t *const different)`
  Source: V35 docs

### PK_CURVE_embed_in_surf -- Embed curve in surface parameter space
  Sig: `PK_ERROR_code_t PK_CURVE_embed_in_surf(PK_CURVE_t curve, PK_SURF_t surf, int *const n_spcurves, PK_SPCURVE_t **const spcurves)`
  Source: V35 docs

### PK_CURVE_make_spcurves -- Create spcurve representation on surface
  Sig: `PK_ERROR_code_t PK_CURVE_make_spcurves(PK_CURVE_t curve, PK_INTERVAL_t range, PK_SURF_t surf, PK_LOGICAL_t degenerate, PK_LOGICAL_t sense, double tolerance, int *const n_spcurves, PK_SPCURVE_t **const spcurves)`
  Source: V35 docs

### PK_CURVE_make_wire_body -- Create wire body from curve (obsolete, superseded by _2)
  Sig: `PK_ERROR_code_t PK_CURVE_make_wire_body(PK_CURVE_t curve, PK_INTERVAL_t range, PK_BODY_t *const body)`
  Source: V35 docs

### PK_CURVE_make_curve_reversed -- Create reversed copy of curve
  Sig: `PK_ERROR_code_t PK_CURVE_make_curve_reversed(PK_CURVE_t curve, PK_CURVE_t *const reverse)`
  Source: V35 docs

### PK_CURVE_degens_f -- Free curve degeneracies result
  Sig: `PK_ERROR_code_t PK_CURVE_degens_f(PK_CURVE_degens_t *const result)`
  Source: V35 docs

### PK_CURVE_self_ints_f -- Free curve self-intersections result
  Sig: `PK_ERROR_code_t PK_CURVE_self_ints_f(PK_CURVE_self_ints_t *const result)`
  Source: V35 docs

### PK_CURVE_convert_parm_to_ki -- Convert PK parameter to KI parameter
  Sig: `PK_ERROR_code_t PK_CURVE_convert_parm_to_ki(PK_CURVE_t curve, double pk_t, double *const ki_t)`
  Source: V35 docs

### PK_CURVE_convert_parm_to_pk -- Convert KI parameter to PK parameter
  Sig: `PK_ERROR_code_t PK_CURVE_convert_parm_to_pk(PK_CURVE_t curve, double ki_t, double *const pk_t)`
  Source: V35 docs

---

## 6. Surface Operations (7 functions)

### PK_SURF_offset -- Create offset surface
  Sig: `PK_ERROR_code_t PK_SURF_offset(PK_SURF_t underlying_surf, double offset_distance, PK_SURF_t *const surf)`
  Source: V35 docs

### PK_SURF_eval_grid -- Evaluate surface points on rectangular parameter grid
  Sig: `PK_ERROR_code_t PK_SURF_eval_grid(PK_SURF_t surf, int n_u, const double u[], int n_v, const double v[], int n_u_derivs, int n_v_derivs, PK_LOGICAL_t triangular, PK_VECTOR_t p[])`
  Source: V35 docs

### PK_SURF_degens_f -- Free surface degeneracies result
  Sig: `PK_ERROR_code_t PK_SURF_degens_f(PK_SURF_degens_t *const result)`
  Source: V35 docs

### PK_SURF_self_ints_f -- Free surface self-intersections result
  Sig: `PK_ERROR_code_t PK_SURF_self_ints_f(PK_SURF_self_ints_t *const result)`
  Source: V35 docs

### PK_SURF_make_curves_isocline -- Create isocline curves on surface (obsolete)
  Sig: `PK_ERROR_code_t PK_SURF_make_curves_isocline(PK_SURF_t surf, PK_UVBOX_t uvbox, PK_VECTOR1_t direction, double angle, double tolerance, int *const n_curves, PK_CURVE_t **const curves)`
  Source: V35 docs

### PK_SURF_make_curve_u_isoparam -- Create curve at constant u parameter
  Sig: `PK_ERROR_code_t PK_SURF_make_curve_u_isoparam(const PK_SURF_t surf, const double param, PK_CURVE_t *const curve)`
  Source: V35 docs

### PK_SURF_make_curve_v_isoparam -- Create curve at constant v parameter
  Sig: `PK_ERROR_code_t PK_SURF_make_curve_v_isoparam(const PK_SURF_t surf, const double param, PK_CURVE_t *const curve)`
  Source: V35 docs

---

## 7. Topology Operations (6 functions)

### PK_TOPOL_find_box_2 -- Axis-aligned bounding box with per-topology boxes
  Sig: `PK_ERROR_code_t PK_TOPOL_find_box_2(int n_topols, const PK_TOPOL_t topols[], const PK_TRANSF_t transfs[], const PK_TOPOL_find_box_2_o_t *options, PK_TOPOL_find_box_2_r_t *const results)`
  Source: V35 docs

### PK_TOPOL_find_connected -- Find all connected topologies
  Sig: `PK_ERROR_code_t PK_TOPOL_find_connected(int n_topols, const PK_TOPOL_t topols[], const PK_TOPOL_find_connected_o_t *options, PK_TOPOL_find_connected_r_t *const results)`
  Source: RE (export addr 0x18045f340; not in V35 online docs)

### PK_TOPOL_is_connected -- Test mutual connectivity of topologies
  Sig: `PK_ERROR_code_t PK_TOPOL_is_connected(int n_topols, const PK_TOPOL_t topols[], const PK_TOPOL_is_connected_o_t *options, PK_TOPOL_is_connected_r_t *const results)`
  Source: RE (export addr 0x18045fc60; not in V35 online docs)

### PK_TOPOL_make_new -- Replace face with new tag (strips attributes/groups)
  Sig: `PK_ERROR_code_t PK_TOPOL_make_new(PK_TOPOL_t topol, const PK_TOPOL_make_new_o_t *options, PK_TOPOL_t *const new_topol)`
  Source: V35 docs

### PK_TOPOL_ask_entities_by_attdef -- Filter entities by class and attribute ownership
  Sig: `PK_ERROR_code_t PK_TOPOL_ask_entities_by_attdef(PK_TOPOL_t topol, PK_CLASS_t class, PK_LOGICAL_t have_attrib, PK_ATTDEF_t attdef, int *const n_entities, PK_ENTITY_t **const entities)`
  Source: V35 docs

### PK_TOPOL_find_frames -- Find frames on a topology entity
  Sig: `PK_ERROR_code_t PK_TOPOL_find_frames(PK_TOPOL_t topol, int *n_frames, PK_FRAME_t **frames)`
  Source: RE (addr 0x180438590)

---

## 8. Topology Imprinting (1 function)

### PK_TOPOL_imprint_frames -- Imprint frames onto topology entities
  Sig: `PK_ERROR_code_t PK_TOPOL_imprint_frames(int n_topols, const PK_TOPOL_t *topols, int *options, int *results)`
  Source: RE (addr 0x180439b60)

---

## 9. Vector Utilities (3 functions)

### PK_VECTOR_make_lsq_plane -- Least-squares plane fit to positions
  Sig: `PK_ERROR_code_t PK_VECTOR_make_lsq_plane(int n_positions, const PK_VECTOR_t *positions, const PK_VECTOR_make_lsq_plane_o_t *options, PK_PLANE_t *const plane)`
  Source: V35 docs

### PK_VECTOR_make_view_transf -- Create viewing transform from direction (deprecated)
  Sig: `PK_ERROR_code_t PK_VECTOR_make_view_transf(PK_VECTOR1_t direct, PK_TRANSF_t *const transf)`
  Source: V35 docs

### PK_VECTOR_perpendicular -- Compute perpendicular vector
  Sig: `PK_ERROR_code_t PK_VECTOR_perpendicular(PK_VECTOR1_t vector1, PK_VECTOR_t vector2, PK_VECTOR1_t *const perpendicular_vector)`
  Source: V35 docs

---

## 10. Geometry Operations (5 functions)

### PK_GEOM_copy -- Copy geometric entities with options
  Sig: `PK_ERROR_code_t PK_GEOM_copy(int n_geoms, const PK_GEOM_t geoms[], const PK_GEOM_copy_o_t *options, PK_GEOM_copy_r_t *const copies)`
  Source: V35 docs

### PK_GEOM_transform -- Transform geometric entity
  Sig: `PK_ERROR_code_t PK_GEOM_transform(PK_GEOM_t in_geom, PK_TRANSF_t transf, double tolerance, PK_GEOM_t *const out_geom, PK_LOGICAL_t *const exact)`
  Source: V35 docs

### PK_GEOM_delete_single -- Delete single geometric entity
  Sig: `PK_ERROR_code_t PK_GEOM_delete_single(PK_GEOM_t geom)`
  Source: V35 docs

### PK_GEOM_enlarge -- Enlarge geometries by scale factor
  Sig: `PK_ERROR_code_t PK_GEOM_enlarge(int n_geoms, const PK_GEOM_t geoms[], const PK_TRANSF_t transfs[], PK_scale_factor_t factor, const PK_GEOM_enlarge_o_t *options, PK_GEOM_enlarge_r_t *const results)`
  Source: V35 docs

### PK_GEOM_check -- Check geometry for invalidities (includes lattice)
  Sig: `PK_ERROR_code_t PK_GEOM_check(PK_GEOM_t geom, const PK_GEOM_check_o_t *options, int *const n_faults, PK_check_fault_t **const faults)`
  Source: V35 docs

---

## 11. Foreign Geometry (2 functions)

### PK_FCURVE_create -- Create foreign curve from standard form
  Sig: `PK_ERROR_code_t PK_FCURVE_create(const PK_FCURVE_sf_t *fcurve_sf, PK_FCURVE_t *const fcurve)`
  Source: V35 docs

### PK_FSURF_create -- Create foreign surface from standard form
  Sig: `PK_ERROR_code_t PK_FSURF_create(const PK_FSURF_sf_t *fsurf_sf, PK_FSURF_t *const fsurf)`
  Source: V35 docs

---

## 12. SP-Curve Operations (2 functions)

### PK_SPCURVE_eval_approx -- Approximate evaluation of surface-parametric curve
  Sig: `PK_ERROR_code_t PK_SPCURVE_eval_approx(PK_SPCURVE_t spcurve, double t, int n_derivs, PK_VECTOR_t p_derivs[])`
  Source: RE (export addr 0x1803fe7b0)

### PK_PLINE_ask -- Query polyline standard form
  Sig: `PK_ERROR_code_t PK_PLINE_ask(PK_PLINE_t pline, PK_PLINE_sf_t *const pline_sf)`
  Source: V35 docs

---

## 13. Loop Operations (1 function)

### PK_LOOP_offset_planar -- Offset planar loop by distance
  Sig: `PK_ERROR_code_t PK_LOOP_offset_planar(PK_LOOP_t loop, double distance, const PK_LOOP_offset_planar_o_t *options, PK_LOOP_offset_planar_r_t *const results)`
  Source: RE (export addr 0x180360fb0)

---

## 14. Region Operations (5 functions)

### PK_REGION_imprint_curve -- Imprint curve onto region
  Sig: `PK_ERROR_code_t PK_REGION_imprint_curve(PK_REGION_t region, PK_CURVE_t curve, PK_INTERVAL_t bounds, int *const n_new_edges, PK_EDGE_t **const new_edges, int *const n_new_faces, PK_FACE_t **const new_faces)`
  Source: V35 docs

### PK_REGION_embed_body -- Embed body into region (cellular topology)
  Sig: `PK_ERROR_code_t PK_REGION_embed_body(PK_REGION_t region, PK_BODY_t body, const PK_REGION_embed_body_o_t *options, PK_REGION_embed_body_r_t *const results)`
  Source: RE (export addr 0x1803d6500)

### PK_REGION_embed_lattices -- Embed lattices into regions
  Source: unknown (referenced in V35 lattice chapter but no header page found)

### PK_REGION_ask_lattices -- Query lattices in a region
  Source: unknown (referenced in V35 lattice chapter but no header page found)

### PK_REGION_remove_lattice -- Remove lattice from region
  Source: unknown (referenced in V35 lattice chapter but no header page found)

---

## 15. Part Operations (3 functions)

### PK_PART_rectify_identifiers -- Ensure no duplicate/invalid identifiers
  Sig: `PK_ERROR_code_t PK_PART_rectify_identifiers(PK_PART_t part, int *const n_entities, PK_ENTITY_t **const entities, int **const old_idents, int **const new_idents)`
  Source: V35 docs

### PK_PART_ask_attrib_owners -- Find entities with matching attribute fields
  Sig: `PK_ERROR_code_t PK_PART_ask_attrib_owners(PK_PART_t part, PK_ATTDEF_t attdef, int n_fields, const int fields[], const int indices[], const int *const values, PK_ATTRIB_filter_f_t filter, PK_POINTER_t context, int *const n_entities, PK_ENTITY_t **const entities)`
  Source: V35 docs

### PK_PART_ask_attribs_filter -- Find attributes matching field values
  Sig: `PK_ERROR_code_t PK_PART_ask_attribs_filter(PK_PART_t part, PK_ATTDEF_t attdef, int n_fields, const int fields[], const int indices[], const int *const values, PK_ATTRIB_filter_f_t filter, PK_POINTER_t context, int *const n_attribs, PK_ATTRIB_t **const attribs)`
  Source: V35 docs

---

## 16. Part-Lattice Queries (1 function)

### PK_PART_ask_con_lattices -- Construction lattices on a part
  Sig: `PK_ERROR_code_t PK_PART_ask_con_lattices(PK_PART_t part, int *const n_con_lattices, PK_LATTICE_t **const con_lattices)`
  Source: V35 docs

---

## 17. Memory (1 function)

### PK_MEMORY_block_f -- Free memory block
  Sig: `PK_ERROR_code_t PK_MEMORY_block_f(PK_MEMORY_block_t *const block)`
  Source: V35 docs

---

## 18. KID (Kernel Identifier) Functions (2 functions)

### PK_acquire_KID_transfers -- Acquire KID transfers
  Sig: unknown (internal function, no public documentation)
  Source: RE (export exists in pskernel.dll)

### PK_register_KID_callbacks -- Register KID management callbacks
  Sig: unknown (internal function, no public documentation)
  Source: RE (export exists in pskernel.dll)

---

## 19. Enquiry Additions from V35 (2 functions)

### PK_SWEPT_ask -- Query swept surface standard form (renamed from PK_SWEEP_ask)
  Sig: `PK_ERROR_code_t PK_SWEPT_ask(PK_SWEPT_t swept, PK_SWEPT_sf_t *const swept_sf)`
  Source: V35 docs

### PK_MTOPOL_ask_box -- Bounding box for mesh topology
  Sig: `PK_ERROR_code_t PK_MTOPOL_ask_box(PK_MTOPOL_t mtopol, PK_BOX_t *const bounding_box)`
  Source: V35 docs

---

## 20. Frame Operations (6 functions)

Frames (`PK_FRAME_t`, class `0xe6`) are coordinate frames attached to topology.
They carry geometry references (surf/curve/point) and an orientation sense.
Not serialized to XT format -- session-only entities.

### PK_FRAME_ask_body -- Get owning body of frame
  Sig: `PK_ERROR_code_t PK_FRAME_ask_body(PK_FRAME_t frame, PK_BODY_t *body)`
  Source: RE (addr 0x1803194f0)

### PK_FRAME_ask_geometry -- Get geometry attached to frame
  Sig: `PK_ERROR_code_t PK_FRAME_ask_geometry(PK_FRAME_t frame, int *options, int *results)`
  Source: RE (addr 0x1803199a0)

### PK_FRAME_ask_owner -- Get owning topology and its class
  Sig: `PK_ERROR_code_t PK_FRAME_ask_owner(PK_FRAME_t frame, int *owner_tag, int *owner_class)`
  Source: RE (addr 0x180319fa0)

### PK_FRAME_ask_sense -- Get frame orientation sense
  Sig: `PK_ERROR_code_t PK_FRAME_ask_sense(PK_FRAME_t frame, PK_LOGICAL_t *sense)`
  Source: RE (addr 0x18031a580)

### PK_FRAME_attach_geoms -- Attach surface/curve/point to frame
  Sig: `PK_ERROR_code_t PK_FRAME_attach_geoms(PK_FRAME_t frame, PK_SURF_t surface, PK_CURVE_t curve, PK_POINT_t point, int *options, int *results)`
  Source: RE (addr 0x18031aa00)

### PK_FRAME_reverse -- Reverse frame sense
  Sig: `PK_ERROR_code_t PK_FRAME_reverse(PK_FRAME_t frame, int *options, PK_FRAME_t *new_frame)`
  Source: RE (addr 0x18031b450)

---

## 21. Lattice Geometry (22 functions)

Lattices are V35 construction geometry: graphs of lballs (spherical nodes) and
lrods (cylindrical/conical struts). Requires `PK_SESSION_set_facet_geometry`.

### PK_LATTICE_create_by_graph -- Create lattice from streaming graph callback
  Sig: `PK_ERROR_code_t PK_LATTICE_create_by_graph(PK_LATTICE_graph_cb_f_t graph_reader, const PK_POINTER_t context, const PK_LATTICE_create_by_graph_o_t *options, PK_LATTICE_create_by_graph_r_t *const results)`
  Source: V35 docs

### PK_LATTICE_make_patterned -- Create patterned lattice from core cell
  Sig: `PK_ERROR_code_t PK_LATTICE_make_patterned(PK_LATTICE_t lattice, const PK_LATTICE_make_patterned_o_t *options, PK_LATTICE_make_patterned_r_t *const results)`
  Source: V35 docs

### PK_LATTICE_make_bodies -- Create solid facet body from lattice
  Sig: `PK_ERROR_code_t PK_LATTICE_make_bodies(PK_LATTICE_t lattice, const PK_LATTICE_make_bodies_o_t *options, PK_LATTICE_make_bodies_r_t *const results)`
  Source: V35 docs

### PK_LATTICE_clip -- Clip lattice against surfaces/faces
  Sig: `PK_ERROR_code_t PK_LATTICE_clip(PK_LATTICE_t lattice, const PK_LATTICE_clip_o_t *options, PK_LATTICE_clip_r_t *const results)`
  Source: V35 docs

### PK_LATTICE_ask_n_lballs -- Query number of lballs
  Sig: `PK_ERROR_code_t PK_LATTICE_ask_n_lballs(PK_LATTICE_t lattice, const PK_LATTICE_ask_n_lballs_o_t *options, PK_LATTICE_ask_n_lballs_r_t *const results)`
  Source: V35 docs

### PK_LATTICE_ask_n_lrods -- Query number of lrods
  Sig: `PK_ERROR_code_t PK_LATTICE_ask_n_lrods(PK_LATTICE_t lattice, const PK_LATTICE_ask_n_lrods_o_t *options, PK_LATTICE_ask_n_lrods_r_t *const results)`
  Source: V35 docs

### PK_LATTICE_ask_part -- Query owning part
  Sig: `PK_ERROR_code_t PK_LATTICE_ask_part(PK_LATTICE_t lattice, PK_PART_t *const part)`
  Source: V35 docs

### PK_LATTICE_do_for_all_lballs -- Iterate all lballs via callback
  Sig: `PK_ERROR_code_t PK_LATTICE_do_for_all_lballs(PK_LATTICE_t lattice, PK_LBALL_cb_f_t cb_fn, PK_POINTER_t data, PK_LOGICAL_t thread_safe)`
  Source: V35 docs

### PK_LATTICE_do_for_all_lrods -- Iterate all lrods via callback
  Sig: `PK_ERROR_code_t PK_LATTICE_do_for_all_lrods(PK_LATTICE_t lattice, PK_LROD_cb_f_t cb_fn, PK_POINTER_t data, PK_LOGICAL_t thread_safe)`
  Source: V35 docs

### PK_LATTICE_find_box -- Axis-aligned bounding box of lattice
  Sig: `PK_ERROR_code_t PK_LATTICE_find_box(PK_LATTICE_t lattice, const PK_LATTICE_find_box_o_t *options, PK_LATTICE_find_box_r_t *const results)`
  Source: V35 docs

### PK_LATTICE_find_nabox -- Non-axis-aligned bounding box of lattice
  Sig: `PK_ERROR_code_t PK_LATTICE_find_nabox(PK_LATTICE_t lattice, const PK_LATTICE_find_nabox_o_t *options, PK_LATTICE_find_nabox_r_t *const results)`
  Source: V35 docs

### PK_LBALL_ask_blend -- Query blend info on lball
  Sig: `PK_ERROR_code_t PK_LBALL_ask_blend(PK_LBALL_t lball, const PK_LBALL_ask_blend_o_t *options, PK_LBALL_ask_blend_r_t *const results)`
  Source: V35 docs

### PK_LBALL_ask_lballs_adj -- Query adjacent lballs
  Sig: `PK_ERROR_code_t PK_LBALL_ask_lballs_adj(PK_LBALL_t lball, const PK_LBALL_ask_lballs_adj_o_t *options, PK_LBALL_ask_lballs_adj_r_t *const results)`
  Source: V35 docs

### PK_LBALL_ask_lrods -- Query incident lrods on lball
  Sig: `PK_ERROR_code_t PK_LBALL_ask_lrods(PK_LBALL_t lball, const PK_LBALL_ask_lrods_o_t *options, PK_LBALL_ask_lrods_r_t *const results)`
  Source: V35 docs

### PK_LBALL_ask_position -- Query lball 3D position
  Sig: `PK_ERROR_code_t PK_LBALL_ask_position(PK_LBALL_t lball, const PK_LBALL_ask_position_o_t *options, PK_LBALL_ask_position_r_t *const results)`
  Source: V35 docs

### PK_LBALL_ask_radius -- Query lball radius
  Sig: `PK_ERROR_code_t PK_LBALL_ask_radius(PK_LBALL_t lball, const PK_LBALL_ask_radius_o_t *options, PK_LBALL_ask_radius_r_t *const results)`
  Source: V35 docs

### PK_LROD_ask_geometry -- Query lrod geometric form
  Sig: `PK_ERROR_code_t PK_LROD_ask_geometry(PK_LROD_t lrod, const PK_LROD_ask_geometry_o_t *options, PK_LROD_ask_geometry_r_t *const results)`
  Source: V35 docs

### PK_LROD_ask_lballs -- Query lrod endpoint lballs
  Sig: `PK_ERROR_code_t PK_LROD_ask_lballs(PK_LROD_t lrod, const PK_LROD_ask_lballs_o_t *options, PK_LROD_ask_lballs_r_t *const results)`
  Source: V35 docs

### PK_LTOPOL_ask_box -- Bounding box of lattice topology entities
  Sig: `PK_ERROR_code_t PK_LTOPOL_ask_box(int n_ltopols, PK_LTOPOL_t ltopols[], const PK_LTOPOL_ask_box_o_t *options, PK_LTOPOL_ask_box_r_t *const results)`
  Source: V35 docs

### PK_LTOPOL_ask_class -- Query class of ltopol element
  Sig: `PK_ERROR_code_t PK_LTOPOL_ask_class(PK_LTOPOL_t ltopol, const PK_LTOPOL_ask_class_o_t *options, PK_LTOPOL_ask_class_r_t *const results)`
  Source: V35 docs

### PK_LTOPOL_is -- Test if entity is an ltopol
  Sig: `PK_ERROR_code_t PK_LTOPOL_is(PK_LTOPOL_t may_be_ltopol, const PK_LTOPOL_is_o_t *options, PK_LTOPOL_is_r_t *const results)`
  Source: V35 docs

---

## Appendix A: Result-Free `_r_f` Functions

All follow the pattern `PK_ERROR_code_t PK_xxx_r_f(PK_xxx_r_t *results)`.
Many share the same underlying implementation when result struct layouts match.

### Body
- `PK_BODY_create_implicit_r_f`
- `PK_BODY_is_cellular_r_f`
- `PK_BODY_is_disjoint_r_f`
- `PK_BODY_enlarge_r_f`
- `PK_BODY_slice_r_f`
- `PK_BODY_slice_cb_r_f`
- `PK_BODY_make_patterned_r_f`
- `PK_BODY_make_swept_profiles_r_f`
- `PK_BODY_make_swept_body_2_r_f`
- `PK_BODY_make_swept_tool_r_f`
- `PK_BODY_thicken_r_f`

### Face
- `PK_FACE_make_valid_faces_r_f`
- `PK_FACE_pattern_r_f`
- `PK_FACE_pattern_2_r_f`
- `PK_FACE_fix_mesh_defects_r_f`
- `PK_FACE_imprint_faces_r_f`

### Entity
- `PK_ENTITY_copy_r_f`
- `PK_ENTITY_range_r_f`
- `PK_ENTITY_range_vector_r_f`
- `PK_ENTITY_ask_description_r_f`

### Geometry
- `PK_GEOM_copy_r_f`
- `PK_GEOM_enlarge_r_f`

### Topology
- `PK_TOPOL_find_box_2_r_f`
- `PK_TOPOL_find_connected_r_f`
- `PK_TOPOL_is_connected_r_f`
- `PK_TOPOL_make_new_r_f`
- `PK_TOPOL_track_r_f`

### Vector
- `PK_VECTOR_make_lsq_plane_r_f`

### Region
- `PK_REGION_embed_body_r_f`

### Loop
- `PK_LOOP_offset_planar_r_f`

### Frame
- `PK_FRAME_attach_geoms_r_f` (addr 0x1800e17d0, shares impl with PK_LATTICE_ask_type_r_f)
- `PK_FRAME_ask_geometry_r_f` (addr 0x180319f80, shares impl with PK_FRAME_ask_owner_r_f)
- `PK_FRAME_ask_owner_r_f` (addr 0x180319f80)
- `PK_FRAME_reverse_r_f` (addr 0x18031ba00, shares impl with PK_THREAD_set_id_r_f)
- `PK_TOPOL_find_frames_r_f` (addr 0x180439030)
- `PK_TOPOL_imprint_frames_r_f` (addr 0x18043aa20)

### Lattice
- `PK_LATTICE_create_by_graph_r_f`
- `PK_LATTICE_make_patterned_r_f`
- `PK_LATTICE_make_bodies_r_f`
- `PK_LATTICE_clip_r_f`
- `PK_LATTICE_ask_n_lballs_r_f`
- `PK_LATTICE_ask_n_lrods_r_f`
- `PK_LATTICE_find_box_r_f`
- `PK_LATTICE_find_nabox_r_f`
- `PK_LBALL_ask_blend_r_f`
- `PK_LBALL_ask_lballs_adj_r_f`
- `PK_LBALL_ask_lrods_r_f`
- `PK_LBALL_ask_position_r_f`
- `PK_LBALL_ask_radius_r_f`
- `PK_LROD_ask_geometry_r_f`
- `PK_LROD_ask_lballs_r_f`
- `PK_LTOPOL_ask_box_r_f`
- `PK_LTOPOL_ask_class_r_f`
- `PK_LTOPOL_is_r_f`

### Enquiry
- `PK_SWEPT_ask_r_f`
- `PK_MTOPOL_ask_box_r_f`

### Section/Imprint
- `PK_BODY_section_with_sheet_r_f`
- `PK_FACE_section_with_sheet_r_f`
- `PK_FACE_imprint_cus_vector_r_f`

### Memory
- `PK_MEMORY_block_f` (listed above in operational functions -- this IS a free function)

---

## Summary by Source

| Source | Count |
|--------|-------|
| V35 online docs | 88 |
| V12 online docs | 3 |
| Reverse engineering (Ghidra) | 30 |
| Unknown (referenced but no sig) | 3 |
| Result-free `_r_f` (trivial) | 86 |
| **Total** | **210** |

Note: 11 functions marked as obsolete/deprecated have documented successors
(PK_BODY_boolean -> _2, PK_BODY_thicken -> _2, PK_FACE_boolean -> _2,
PK_FACE_hollow -> _2, PK_FACE_section_with_sheet -> _2,
PK_BODY_section_with_sheet -> _2, PK_CURVE_make_wire_body -> _2,
PK_SURF_make_curves_isocline -> PK_SURF_make_cus_isocline,
PK_EDGE_find_deviation -> _2, PK_VECTOR_make_view_transf -> PK_TRANSF_create_view,
PK_CURVE_ask_parm_different -> will be withdrawn with KI).
