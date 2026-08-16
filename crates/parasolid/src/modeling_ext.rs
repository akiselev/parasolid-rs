//! Small high-level modeling operations built from validated PK primitives.
//!
//! These live in the safe `parasolid` crate so downstream CAD applications do
//! not need to reach through the wrapper into `parasolid-sys`.

use std::os::raw::{c_int, c_void};

use parasolid_sys::*;

use crate::{Axis2, Body, Edge, PkArray, PsError, PsResult, Surf};

impl Edge {
    /// Cover one connected, closed planar wire loop with a planar sheet face.
    ///
    /// The input edges may come from separate temporary wire bodies. They are
    /// first collected into one wire body, then one face is created for the
    /// connected loop and the supplied plane surface is attached to it.
    pub fn make_planar_sheet(edges: &[Edge], plane: Axis2) -> PsResult<Body> {
        if edges.is_empty() {
            return Err(PsError::Session(
                "make_planar_sheet requires at least one edge".into(),
            ));
        }

        let body = Edge::make_wire_body(edges)?;
        let body_edges = body.edges()?;
        let representative = body_edges.first().ok_or_else(|| {
            PsError::Session("wire body contained no edges".into())
        })?;

        // One representative edge identifies the one connected closed loop.
        let reps = [representative.tag()];
        let senses = [PK_LOGICAL_true];
        let shared_loop = [-1i32];
        let mut new_faces = [PK_ENTITY_null; 1];
        pk_call!(PK_EDGE_make_faces_from_wire(
            1,
            reps.as_ptr(),
            senses.as_ptr(),
            shared_loop.as_ptr(),
            new_faces.as_mut_ptr(),
        ));

        let surf = Surf::plane(plane)?;
        let surfs = [surf.tag()];
        let face_senses = [PK_LOGICAL_true];
        pk_call!(PK_FACE_attach_surfs(
            1,
            new_faces.as_ptr(),
            surfs.as_ptr(),
            face_senses.as_ptr(),
        ));
        Ok(body)
    }
}

impl Body {
    /// Chamfer selected edges with constant offsets, modifying this body in
    /// place. `width_2 = None` creates an equal-offset chamfer.
    ///
    /// This mirrors the validated fillet path: first attach chamfer blend
    /// attributes, then realize them with `PK_BODY_fix_blends`.
    pub fn chamfer_edges(
        &self,
        edges: &[Edge],
        width_1: f64,
        width_2: Option<f64>,
    ) -> PsResult<usize> {
        if edges.is_empty() {
            return Ok(0);
        }

        let edge_tags: Vec<PK_EDGE_t> = edges.iter().map(Edge::tag).collect();
        let mut n_set: c_int = 0;
        let mut set_edges: *mut PK_EDGE_t = std::ptr::null_mut();
        pk_call!(PK_EDGE_set_blend_chamfer(
            edge_tags.len() as c_int,
            edge_tags.as_ptr(),
            width_2.unwrap_or(width_1),
            width_1,
            std::ptr::null(),
            std::ptr::null(),
            &mut n_set,
            &mut set_edges,
        ));
        unsafe {
            let _ = PkArray::from_raw(set_edges, n_set);
        }

        let mut n_blends: c_int = 0;
        let mut blends: *mut PK_FACE_t = std::ptr::null_mut();
        let mut unders: *mut PK_FACE_array_t = std::ptr::null_mut();
        let mut topols: *mut c_int = std::ptr::null_mut();
        let mut fault: PK_blend_fault_t = 0;
        let mut fault_edge: PK_EDGE_t = PK_ENTITY_null;
        let mut fault_topol: PK_ENTITY_t = PK_ENTITY_null;
        pk_call!(PK_BODY_fix_blends(
            self.tag(),
            std::ptr::null(),
            &mut n_blends,
            &mut blends,
            &mut unders,
            &mut topols,
            &mut fault,
            &mut fault_edge,
            &mut fault_topol,
        ));

        unsafe {
            // PK_FACE_array_t owns one inner allocation per descriptor. Free
            // inner-first, then outer, matching the validated fillet probe.
            if !unders.is_null() {
                let descs = std::slice::from_raw_parts(unders, n_blends as usize);
                for descriptor in descs {
                    if descriptor.length > 0 && !descriptor.array.is_null() {
                        let _ = PK_MEMORY_free(descriptor.array as *mut c_void);
                    }
                }
                let _ = PK_MEMORY_free(unders as *mut c_void);
            }
            if !blends.is_null() {
                let _ = PK_MEMORY_free(blends as *mut c_void);
            }
            if !topols.is_null() {
                let _ = PK_MEMORY_free(topols as *mut c_void);
            }
        }

        Ok(n_blends as usize)
    }
}
