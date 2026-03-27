# Missing PK_* Exports

221 of 1204 exported symbols from `pskernel.dll` have no Rust binding in `parasolid-sys`.
983 functions are already bound.

## Summary by Category

| Category | Count |
|----------|-------|
| Roll-forward variants (_r_f) | 90 |
| Lattice modeling | 31 |
| Body operations | 23 |
| Face operations | 14 |
| Curve operations | 10 |
| Frames | 9 |
| Surface operations | 7 |
| Region operations | 5 |
| Topology operations | 5 |
| Edge operations | 4 |
| Entity operations | 4 |
| Specialized geometry | 4 |
| Geometry operations | 4 |
| Part operations | 4 |
| Transform/Vector | 3 |
| Miscellaneous | 2 |
| Loop operations | 1 |
| Memory | 1 |

## Roll-forward variants (_r_f) (90)

- `PK_ASSEMBLY_check_r_f`
- `PK_BODY_create_implicit_r_f`
- `PK_BODY_create_topology_2_r_f`
- `PK_BODY_enlarge_r_f`
- `PK_BODY_is_cellular_r_f`
- `PK_BODY_is_disjoint_r_f`
- `PK_BODY_make_patterned_r_f`
- `PK_BODY_make_section_r_f`
- `PK_BODY_slice_cb_r_f`
- `PK_BODY_slice_r_f`
- `PK_BODY_tracked_loft_r_f`
- `PK_BODY_tracked_sweep_2_r_f`
- `PK_BODY_tracked_sweep_r_f`
- `PK_CURVE_find_vectors_r_f`
- `PK_CURVE_fix_degens_r_f`
- `PK_CURVE_fix_self_int_r_f`
- `PK_CURVE_project_r_f`
- `PK_EDGE_find_deviation_r_f`
- `PK_EDGE_offset_on_body_r_f`
- `PK_ENTITY_find_reparam_r_f`
- `PK_ENTITY_range_r_f`
- `PK_ENTITY_range_vector_r_f`
- `PK_ENTITY_track_r_f`
- `PK_FACE_ask_type_r_f`
- `PK_FACE_fix_mesh_defects_r_f`
- `PK_FACE_inst_tools_r_f`
- `PK_FACE_make_sect_with_sfs_r_f`
- `PK_FACE_pattern_2_r_f`
- `PK_FACE_pattern_r_f`
- `PK_FRAME_ask_geometry_r_f`
- `PK_FRAME_ask_owner_r_f`
- `PK_FRAME_attach_geoms_r_f`
- `PK_FRAME_reverse_r_f`
- `PK_GEOM_copy_r_f`
- `PK_GEOM_enlarge_r_f`
- `PK_GROUP_ask_controls_r_f`
- `PK_LATTICE_ask_bound_r_f`
- `PK_LATTICE_ask_cell_r_f`
- `PK_LATTICE_ask_connectivity_r_f`
- `PK_LATTICE_ask_core_r_f`
- `PK_LATTICE_ask_form_r_f`
- `PK_LATTICE_ask_n_lballs_r_f`
- `PK_LATTICE_ask_n_lrods_r_f`
- `PK_LATTICE_ask_regions_r_f`
- `PK_LATTICE_ask_type_r_f`
- `PK_LATTICE_clip_r_f`
- `PK_LATTICE_combine_r_f`
- `PK_LATTICE_create_by_core_r_f`
- `PK_LATTICE_create_by_graph_r_f`
- `PK_LATTICE_disjoin_r_f`
- `PK_LATTICE_find_box_r_f`
- `PK_LATTICE_find_nabox_r_f`
- `PK_LATTICE_make_bodies_r_f`
- `PK_LATTICE_make_patterned_r_f`
- `PK_LATTICE_offset_r_f`
- `PK_LBALL_ask_lballs_adj_r_f`
- `PK_LBALL_ask_lrods_r_f`
- `PK_LBALL_ask_position_r_f`
- `PK_LBALL_ask_radius_r_f`
- `PK_LROD_ask_geometry_r_f`
- `PK_LROD_ask_lballs_r_f`
- `PK_LTOPOL_ask_box_r_f`
- `PK_LTOPOL_ask_class_r_f`
- `PK_LTOPOL_is_r_f`
- `PK_REGION_ask_lattices_r_f`
- `PK_REGION_ask_type_r_f`
- `PK_REGION_embed_body_r_f`
- `PK_REGION_embed_lattices_r_f`
- `PK_REGION_remove_lattice_r_f`
- `PK_SURF_find_vectors_r_f`
- `PK_SURF_fix_self_int_r_f`
- `PK_THREAD_lock_partitions_r_f`
- `PK_THREAD_set_id_r_f`
- `PK_TOPOL_find_box_2_r_f`
- `PK_TOPOL_find_connected_r_f`
- `PK_TOPOL_find_frames_r_f`
- `PK_TOPOL_imprint_frames_r_f`
- `PK_TOPOL_is_connected_r_f`
- `PK_TOPOL_local_r_f`
- `PK_TOPOL_track_r_f`
- `PK_TRANSF_classify_r_f`
- `PK_TRANSF_enlarge_r_f`
- `PK_TRANSF_transform_r_f`
- `PK_boolean_r_f`
- `PK_identify_details_r_f`
- `PK_identify_facesets_r_f`
- `PK_identify_general_r_f`
- `PK_imprint_r_f`
- `PK_section_2_r_f`
- `PK_section_r_f`

## Lattice modeling (31)

Lattices represent collections of lballs (spheres) joined by lrods (cylindrical or conical rods). They are orphan geometry that cannot be attached to a body nor transmitted. Solid convergent bodies can be created from them. See Ch. 17 for full details.

- `PK_LATTICE_ask_bound`
- `PK_LATTICE_ask_cell`
- `PK_LATTICE_ask_connectivity`
- `PK_LATTICE_ask_core`
- `PK_LATTICE_ask_form`
- `PK_LATTICE_ask_n_lballs` — Returns the number of lballs in the given lattice. (Ch. 17)
- `PK_LATTICE_ask_n_lrods` — Returns the number of lrods in the given lattice. (Ch. 17)
- `PK_LATTICE_ask_part`
- `PK_LATTICE_ask_regions`
- `PK_LATTICE_ask_type`
- `PK_LATTICE_clip`
- `PK_LATTICE_combine`
- `PK_LATTICE_create_by_core`
- `PK_LATTICE_create_by_graph` — Creates an orphan lattice geometry from data supplied by a user-defined callback function (graph_reader). Receives graph_reader callback, context pointer, and options (lballs_estimate, lrods_estimate, graph_free, thread_safe, lball_radius, lrod_shape_opts). (Ch. 17)
- `PK_LATTICE_disjoin`
- `PK_LATTICE_do_for_all_lballs` — Calls the given callback function for every lball in the given lattice. (Ch. 17)
- `PK_LATTICE_do_for_all_lrods` — Calls the given callback function for every lrod in the given lattice. (Ch. 17)
- `PK_LATTICE_find_box`
- `PK_LATTICE_find_nabox`
- `PK_LATTICE_make_bodies` — Creates a solid body with a single mesh face from lattice geometry. (Ch. 17)
- `PK_LATTICE_make_patterned`
- `PK_LATTICE_offset`
- `PK_LBALL_ask_lballs_adj` — Returns the adjacent lballs of a given lball. (Ch. 17)
- `PK_LBALL_ask_lrods` — Queries the lrods of an lball in a lattice. (Ch. 17)
- `PK_LBALL_ask_position` — Queries the position of the given lball. (Ch. 17)
- `PK_LBALL_ask_radius` — Returns the radius of the given lball. (Ch. 17)
- `PK_LROD_ask_geometry`
- `PK_LROD_ask_lballs` — Queries the lballs of a given lrod. (Ch. 17)
- `PK_LTOPOL_ask_box` — Returns the bounding box of the given ltopols. (Ch. 17)
- `PK_LTOPOL_ask_class` — Returns the class of an element of lattice topology. (Ch. 17)
- `PK_LTOPOL_is` — Queries the existence of an ltopol. (Ch. 17)

## Body operations (23)

- `PK_BODY_boolean` — Performs global boolean operations (unite, subtract, intersect) on solid, sheet, wire, and general bodies. The _2 variant is the current API. Takes target body, tool bodies, boolean type, and extensive options (matched_region, merge_imprinted, instancing, material_side, tracking, etc.). (Ch. 49)
- `PK_BODY_create_implicit`
- `PK_BODY_create_minimum_topology`
- `PK_BODY_create_sheet_topology`
- `PK_BODY_create_solid_topology` — Simplified topology import for solid bodies. Edge senses in the `senses` argument control loop directions. See PK_BODY_create_topology_2 for the more general variant. (Ch. 88)
- `PK_BODY_create_topology` — Generic topology creation function for importing external data. Defines topological entities (body, region, shell, face, loop, edge, vertex) and their relationships. The first region must be the outer region. The _2 variant is current. (Ch. 88)
- `PK_BODY_create_wire_topology`
- `PK_BODY_enlarge`
- `PK_BODY_hollow` — Hollows a solid body by offsetting all faces by a specified distance, creating a shell. Takes body, offset, tolerance, and options (pierce_faces, offset_faces, blend_edges, hollow_local, grow, etc.). The _2 variant is current. (Ch. 57)
- `PK_BODY_imprint_curves_normal`
- `PK_BODY_imprint_curves_vector`
- `PK_BODY_imprint_cus_vector`
- `PK_BODY_is_cellular`
- `PK_BODY_is_disjoint`
- `PK_BODY_make_patterned`
- `PK_BODY_make_swept_profiles`
- `PK_BODY_offset` — Offsets all faces in a body by a specified distance. Positive offsets expand, negative offsets shrink. Options allow subsets of faces to be offset by different distances. The _2 variant is current. (Ch. 56)
- `PK_BODY_section_with_sheet` — Sections a body with a sheet tool (global section). Target can be solid or sheet. Tool may be disjoint. After operation, the tool is deleted. Returns result bodies in front of, behind, or both sides of the section. The _2 variant is current. (Ch. 53)
- `PK_BODY_slice`
- `PK_BODY_thicken` — Thickens a sheet body to form a solid body by offsetting in both directions (front_default and back_default). Options allow per-face offset overrides, blend_edges, check_fa_fa, ortho_vx_split, grow, etc. The _3 variant is current. (Ch. 58)
- `PK_BODY_thicken_2`
- `PK_BODY_transform` — Transforms a sheet or solid body using a 4x4 homogeneous matrix. Face surfaces and edge curves may be approximated. Intersection curves are recalculated unless exact. The _2 variant is current. (Ch. 20)
- `PK_BODY_trim_neutral_sheets` — Trims neutral sheets created by PK_FACE_make_neutral_sheet against a solid body. Scribes edges where sheets meet each other or the body boundary, then deletes unwanted faces. Options: ignore small faces, trim_method, extend_and_fill_holes. The _2 variant is current. (Ch. 45)

## Face operations (14)

- `PK_FACE_boolean` — Performs local boolean operations on selected face pairs from target and tool bodies. Quicker than global boolean but does not guarantee topological consistency. Takes target faces, tool faces, boolean type, and options (select_region, limit_target_faces, limit_tool_faces, extend_face_list, etc.). The _2 variant is current. (Ch. 49)
- `PK_FACE_delete` — Deletes faces from a body and heals the resulting wounds. Healing methods include capping (PK_FACE_heal_cap_c) and shrinkage (PK_FACE_heal_shrink_c). Returns tracking structure if body is split. The _2 variant is current. (Ch. 61)
- `PK_FACE_fix_mesh_defects`
- `PK_FACE_hollow` — Hollows a body by specifying exactly which faces to offset and by how much. Any unspecified faces are treated as pierce faces. Alternative to PK_BODY_hollow for cases with many pierce faces. The _3 variant is current. (Ch. 57)
- `PK_FACE_hollow_2`
- `PK_FACE_imprint_cus_vector`
- `PK_FACE_imprint_faces` — Imprints edges and vertices on target and tool face sets where they intersect. Returns paired sets of imprinted edges/vertices. Options: imprint_overlapping, extend_face_list, matched_region, imprint_complete_targ/tool, imprint_dir, tolerance. The _2 variant is current. (Ch. 48)
- `PK_FACE_install_surfs_isocline`
- `PK_FACE_make_neutral_sheet` — Creates a neutral sheet (mid-surface) between two arrays of faces (left_faces and right_faces). Used for thin-wall analysis. Options: placement, make_sheet, extend_and_fill_holes, construction method. The _2 variant is current. (Ch. 45)
- `PK_FACE_make_sheet_body`
- `PK_FACE_make_valid_faces`
- `PK_FACE_pattern_2`
- `PK_FACE_repair` — Repairs faces by splitting along G1 discontinuities and excluding regions of surface self-intersection. Used during data import after attaching geometry. (Ch. 88)
- `PK_FACE_section_with_sheet` — Sections particular faces of a target body with faces of a sheet body (local section). Similar to PK_FACE_boolean_2 for partial sectioning. Options include front/behind/both result selection and matched topology. The _2 variant is current. (Ch. 53)

## Curve operations (10)

- `PK_CURVE_ask_parm_different`
- `PK_CURVE_convert_parm_to_ki`
- `PK_CURVE_convert_parm_to_pk`
- `PK_CURVE_degens_f`
- `PK_CURVE_embed_in_surf` — Creates SP-curves by embedding 2D curves (analytic or B-spline) into a surface. Similar to PK_SPCURVE_create but supports 2D analytic curves, can extend surfaces for out-of-range SP-curves, and splits at parametric degeneracies and G1 discontinuities. The _2 variant is current. (Ch. 88)
- `PK_CURVE_find_length`
- `PK_CURVE_make_curve_reversed` — Negates (reverses) a curve, reversing its natural direction. The curve tangent always points in the direction of increasing parameter value; this function reverses that. (Ch. 14)
- `PK_CURVE_make_spcurves` — Creates SP-curves from 3D trimming curves and surfaces. Options control: creating zero-length SP-curves to connect disjoint segments at surface degeneracies, direction matching with trimmed curves, and C2 continuity in parameter space. The _2 variant is current. (Ch. 88)
- `PK_CURVE_make_wire_body` — Creates a wire body from an array of curves. The body can be open, closed, or disjoint. The _2 variant receives curves, bounds, and options (tolerance, allow_disjoint, allow_general, check, want_edges, want_indices, sequential). Returns body, new_edges, and edge_index mapping. (Ch. 42, Ch. 10)
- `PK_CURVE_self_ints_f`

## Frames (9)

- `PK_BODY_ask_frames`
- `PK_FRAME_ask_body`
- `PK_FRAME_ask_geometry`
- `PK_FRAME_ask_owner`
- `PK_FRAME_ask_sense`
- `PK_FRAME_attach_geoms`
- `PK_FRAME_reverse`
- `PK_TOPOL_find_frames`
- `PK_TOPOL_imprint_frames`

## Surface operations (7)

- `PK_SURF_degens_f`
- `PK_SURF_eval_grid`
- `PK_SURF_make_curve_u_isoparam`
- `PK_SURF_make_curve_v_isoparam`
- `PK_SURF_make_curves_isocline`
- `PK_SURF_offset` — Creates an offset surface. Listed as an orphan geometry creation function alongside PK_OFFSET_create. (Ch. 15)
- `PK_SURF_self_ints_f`

## Region operations (5)

- `PK_REGION_ask_lattices`
- `PK_REGION_embed_body`
- `PK_REGION_embed_lattices`
- `PK_REGION_imprint_curve` — Imprints a curve onto a region. One of the three imprinting functions (alongside PK_BODY_imprint_curve and PK_FACE_imprint_curves_2). Used to create new faces/edges by scribing curves on a region or wire body, or to create profiles for sweep/spin operations. (Ch. 15)
- `PK_REGION_remove_lattice`

## Topology operations (5)

- `PK_TOPOL_ask_entities_by_attdef`
- `PK_TOPOL_find_box_2`
- `PK_TOPOL_find_connected`
- `PK_TOPOL_is_connected`
- `PK_TOPOL_make_new`

## Edge operations (4)

- `PK_EDGE_contains_vector`
- `PK_EDGE_find_deviation` — Calculates the distance between two edge geometries (maximal distance or multiple distance samples). Used before sewing to measure gaps between matching edges. Subtract edge precisions from the returned distance to get actual gap width. The _2 variant is current. (Ch. 44)
- `PK_EDGE_find_end_tangents`
- `PK_EDGE_reverse` — Reverses an edge and its associated geometry. PK_EDGE_reverse_2 is the current variant, which also supports sheet, solid, and general bodies (not just wire bodies). (Ch. 42)

## Entity operations (4)

- `PK_ENTITY_ask_description` — Listed in the C binding implementation as mapping to KI function PK.ENTITY.ask_description. One of the functions whose C binding name differs from the standard PK_CLASS_verb pattern. (Ch. 122)
- `PK_ENTITY_copy_2` — Copies any entity in a Parasolid session. Receives an entity and options structure (destination, want_user_fields, want_attribs, want_groups, want_tracking, track_classes). Returns the copy and optional tracking information mapping original components to copied components. (Ch. 2)
- `PK_ENTITY_range`
- `PK_ENTITY_range_vector`

## Specialized geometry (4)

- `PK_FCURVE_create` — Creates a foreign curve (FCURVE). Listed as a geometry creation function for data import alongside standard curve types (BCURVE, CIRCLE, etc.). (Ch. 88)
- `PK_FSURF_create` — Creates a foreign surface (FSURF). Listed as a geometry creation function for data import alongside standard surface types (BSURF, CONE, etc.). (Ch. 88)
- `PK_PLINE_ask`
- `PK_SPCURVE_eval_approx`

## Geometry operations (4)

- `PK_GEOM_copy` — Copies a collection of geometric entities while preserving geometric dependencies among them. Receives any mix of principal, construction, or orphan geometry. Creates copies together with referenced dependent geometries. Returns copies in input order with tracking info. Duplicates in input are copied once and referenced. (Ch. 2, Ch. 14)
- `PK_GEOM_delete_single`
- `PK_GEOM_enlarge`
- `PK_GEOM_transform` — Transforms an array of geometric entities (points, curves, surfaces). Can modify originals or create copies. Options: tolerance, modify, want_out_geoms, want_exact. The _2 variant is current. (Ch. 20)

## Part operations (4)

- `PK_PART_ask_attrib_owners`
- `PK_PART_ask_attribs_filter`
- `PK_PART_ask_con_lattices`
- `PK_PART_rectify_identifiers`

## Transform/Vector (3)

- `PK_VECTOR_make_lsq_plane` — Creates a least-squares plane from a set of vectors/points. Listed as an orphan geometry creation function. (Ch. 15)
- `PK_VECTOR_make_view_transf`
- `PK_VECTOR_perpendicular`

## Miscellaneous (2)

- `PK_acquire_KID_transfers`
- `PK_register_KID_callbacks`

## Loop operations (1)

- `PK_LOOP_offset_planar`

## Memory (1)

- `PK_MEMORY_block_f`
