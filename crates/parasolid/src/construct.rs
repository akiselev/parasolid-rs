//! Topology construction, geometry attachment, tolerant edges, SP-curves,
//! nominal curves and compound bodies — the calls CADabra's oracle uses to
//! build B-reps the modelling API refuses to make.
//!
//! Every entry point here is in parasolid-re `catalog/pk-validated-abi.md`
//! ("P9 oracle construction calls" and "Construction batch"; DLL
//! `c900fa3f…63073`). Option layouts live in `parasolid_sys::oracle_abi`.
//! Runtime tests are in `parasolid-test/src/oracle_wrappers.rs`.
//!
//! **Residual risk** (see also `docs/pskernel-solidworks.md`):
//! * `create_topology` reports create faults as data; a fault list containing
//!   only `PK_BODY_state_ok_c` is the success shape.
//! * `attach_fin_curves` accepts an interval that stops short of the vertices;
//!   the fault only shows up in `Edge::check` (`PK_EDGE_state_bad_vertex_c`).
//! * `Edge::attach_curve_nmnl` reports a too-short interval as 51
//!   `inconsistent_geom`, not the documented 5037 `bad_interval`.
//! * `PK_BCURVE_create` is validated only for polynomial (non-rational,
//!   non-periodic) curves of dimension 2 and 3.

use std::os::raw::c_int;

use parasolid_sys::*;

use crate::body::{Body, BodyType};
use crate::curve::Curve;
use crate::edge::Edge;
use crate::entity::PkClass;
use crate::error::PsResult;
use crate::face::Face;
use crate::memory::PkArray;
use crate::point::Point;
use crate::surf::Surf;
use crate::topology::Fin;
use crate::vertex::Vertex;

pub use parasolid_sys::{
    PK_BODY_make_compound_o_t as MakeCompoundOptions,
    PK_CURVE_make_spcurves_o_t as MakeSpcurvesOptions,
    PK_EDGE_attach_curves_o_t as AttachCurvesOptions,
    PK_EDGE_set_precision_o_t as SetPrecisionOptions,
};

/// Sense of a parent→child relation in [`create_topology`]
/// (`PK_TOPOL_sense_t`: none 18540, negative 18541, positive 18542).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopolSense {
    None,
    Positive,
    Negative,
}

impl TopolSense {
    fn token(self) -> PK_TOPOL_sense_t {
        match self {
            TopolSense::None => PK_TOPOL_sense_none_c,
            TopolSense::Positive => PK_TOPOL_sense_positive_c,
            TopolSense::Negative => PK_TOPOL_sense_negative_c,
        }
    }
}

fn body_type_token(t: BodyType) -> PK_BODY_type_t {
    match t {
        BodyType::Empty => PK_BODY_type_empty_c,
        BodyType::Acorn => PK_BODY_type_acorn_c,
        BodyType::Wire => PK_BODY_type_wire_c,
        BodyType::Sheet => PK_BODY_type_sheet_c,
        BodyType::Solid => PK_BODY_type_solid_c,
        BodyType::General => PK_BODY_type_general_c,
    }
}

/// One `PK_create_fault_t`: a state token and the indices (into the caller's
/// class list) it concerns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateFault {
    pub state: i32,
    pub indices: Vec<i32>,
}

/// Result of [`create_topology`].
#[derive(Debug, Clone)]
pub struct CreatedTopology {
    pub body: Body,
    /// One tag per entry of the class list, in order (index 0 is the body).
    pub topols: Vec<i32>,
    /// The kernel's create faults. A successful build reports a single
    /// `PK_BODY_state_ok_c` fault.
    pub faults: Vec<CreateFault>,
}

impl CreatedTopology {
    /// `true` when every create fault is `PK_BODY_state_ok_c`.
    pub fn is_ok(&self) -> bool {
        self.faults.iter().all(|f| f.state == PK_BODY_state_ok_c)
    }
}

fn copy_ints(p: *const c_int, n: c_int) -> Vec<i32> {
    if p.is_null() || n <= 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(p, n as usize) }.to_vec()
    }
}

/// `PK_BODY_create_topology_2`: build a body's topology (no geometry) from a
/// class list and `(parent, child, sense)` relations given as indices into
/// `classes`. `body_type` `None` lets the kernel choose (`general`).
pub fn create_topology(
    classes: &[PkClass],
    relations: &[(usize, usize, TopolSense)],
    body_type: Option<BodyType>,
) -> PsResult<CreatedTopology> {
    let cls: Vec<PK_CLASS_t> = classes.iter().map(|c| c.to_raw()).collect();
    let parents: Vec<c_int> = relations.iter().map(|r| r.0 as c_int).collect();
    let children: Vec<c_int> = relations.iter().map(|r| r.1 as c_int).collect();
    let senses: Vec<PK_TOPOL_sense_t> = relations.iter().map(|r| r.2.token()).collect();
    let opts = PK_BODY_create_topology_2_o_t {
        body_type: body_type.map_or(PK_BODY_type_general_c, body_type_token),
        ..Default::default()
    };
    let mut r: PK_BODY_create_topology_2_r_t = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        PK_BODY_create_topology_2(
            cls.len() as c_int,
            cls.as_ptr(),
            relations.len() as c_int,
            parents.as_ptr(),
            children.as_ptr(),
            senses.as_ptr(),
            &opts,
            &mut r,
        )
    };
    let out = CreatedTopology {
        body: Body::from_tag(r.body),
        topols: copy_ints(r.topols, r.n_topols),
        faults: (0..r.n_create_faults.max(0) as usize)
            .map(|i| {
                let f = unsafe { &*r.create_faults.add(i) };
                CreateFault { state: f.state, indices: copy_ints(f.indices, f.n_indices) }
            })
            .collect(),
    };
    unsafe { PK_BODY_create_topology_2_r_f(&mut r) };
    crate::error::pk_check(rc)?;
    Ok(out)
}

fn same_len(a: usize, b: usize, what: &str) -> PsResult<()> {
    if a != b {
        return Err(crate::error::PsError::Session(format!(
            "{what}: array lengths differ ({a} vs {b})"
        )));
    }
    Ok(())
}

/// `PK_FACE_attach_surfs`: attach one surface to each face, with the face's
/// sense relative to the surface normal (`true` = same direction).
pub fn attach_surfs(faces: &[Face], surfs: &[Surf], senses: &[bool]) -> PsResult<()> {
    same_len(faces.len(), surfs.len(), "attach_surfs")?;
    same_len(faces.len(), senses.len(), "attach_surfs")?;
    let f: Vec<i32> = faces.iter().map(|x| x.tag()).collect();
    let s: Vec<i32> = surfs.iter().map(|x| x.tag()).collect();
    let l: Vec<PK_LOGICAL_t> = senses.iter().map(|&b| b as PK_LOGICAL_t).collect();
    pk_call!(PK_FACE_attach_surfs(f.len() as c_int, f.as_ptr(), s.as_ptr(), l.as_ptr()));
    Ok(())
}

/// `PK_VERTEX_attach_points`: attach one point to each vertex.
pub fn attach_points(vertices: &[Vertex], points: &[Point]) -> PsResult<()> {
    same_len(vertices.len(), points.len(), "attach_points")?;
    let v: Vec<i32> = vertices.iter().map(|x| x.tag()).collect();
    let p: Vec<i32> = points.iter().map(|x| x.tag()).collect();
    pk_call!(PK_VERTEX_attach_points(v.len() as c_int, v.as_ptr(), p.as_ptr()));
    Ok(())
}

/// `PK_EDGE_attach_curves_2` (options v2): attach one curve to each edge. The
/// tracking result is freed, not returned.
pub fn attach_curves(
    edges: &[Edge],
    curves: &[Curve],
    options: &AttachCurvesOptions,
) -> PsResult<()> {
    same_len(edges.len(), curves.len(), "attach_curves")?;
    let mut e: Vec<i32> = edges.iter().map(|x| x.tag()).collect();
    let mut c: Vec<i32> = curves.iter().map(|x| x.tag()).collect();
    let mut o = *options;
    let mut track = PK_ENTITY_track_r_t::default();
    let rc = unsafe {
        PK_EDGE_attach_curves_2(e.len() as c_int, e.as_mut_ptr(), c.as_mut_ptr(), &mut o, &mut track)
    };
    unsafe { PK_ENTITY_track_r_f(&mut track) };
    crate::error::pk_check(rc)
}

/// `PK_FIN_attach_curves`: attach an SP-curve to each fin of a tolerant edge,
/// with the parameter interval it covers.
pub fn attach_fin_curves(fins: &[Fin], curves: &[Curve], intervals: &[(f64, f64)]) -> PsResult<()> {
    same_len(fins.len(), curves.len(), "attach_fin_curves")?;
    same_len(fins.len(), intervals.len(), "attach_fin_curves")?;
    let mut f: Vec<i32> = fins.iter().map(|x| x.tag()).collect();
    let mut c: Vec<i32> = curves.iter().map(|x| x.tag()).collect();
    let mut iv: Vec<PK_INTERVAL_t> =
        intervals.iter().map(|&(low, high)| PK_INTERVAL_t { low, high }).collect();
    pk_call!(PK_FIN_attach_curves(
        f.len() as c_int,
        f.as_mut_ptr(),
        c.as_mut_ptr(),
        iv.as_mut_ptr()
    ));
    Ok(())
}

impl Curve {
    /// A non-rational 2-D B-curve (`vertex_dim = 2`), for use as the
    /// parameter-space curve of an SP-curve. Distinct knots + multiplicities,
    /// as for [`Curve::bcurve`].
    pub fn bcurve_2d(
        degree: i32,
        control_points: &[(f64, f64)],
        knots: &[f64],
        multiplicities: &[i32],
    ) -> PsResult<Curve> {
        let verts: Vec<f64> = control_points.iter().flat_map(|&(u, v)| [u, v]).collect();
        let sf = PK_BCURVE_sf_t {
            degree,
            n_vertices: control_points.len() as c_int,
            vertex_dim: 2,
            is_rational: PK_LOGICAL_false,
            vertices: verts.as_ptr(),
            form: PK_BCURVE_form_unset_c,
            n_knots: knots.len() as c_int,
            knot_mult: multiplicities.as_ptr(),
            knots: knots.as_ptr(),
            knot_type: PK_knot_unset_c,
            is_periodic: 0,
            is_closed: 0,
            _pad: [0; 2],
            self_intersecting: 0,
        };
        let mut tag: PK_BCURVE_t = PK_ENTITY_null;
        pk_call!(PK_BCURVE_create(&sf, &mut tag));
        Ok(Curve::from_tag(tag))
    }

    /// `PK_SPCURVE_create`: an SP-curve `surf(curve2d(t))` from a surface and
    /// an orphan 2-D B-curve. The B-curve is consumed into the SP-curve (a
    /// second embedding gives 1086 `not_in_same_partition`).
    pub fn spcurve(surf: &Surf, curve2d: &Curve) -> PsResult<Curve> {
        let sf = PK_SPCURVE_sf_t { surface: surf.tag(), bcurve: curve2d.tag() };
        let mut tag = PK_ENTITY_null;
        pk_call!(PK_SPCURVE_create(&sf, &mut tag));
        Ok(Curve::from_tag(tag))
    }

    /// `PK_SPCURVE_ask`: the surface and 2-D B-curve of an SP-curve.
    pub fn ask_spcurve(&self) -> PsResult<(Surf, Curve)> {
        let mut sf = PK_SPCURVE_sf_t { surface: 0, bcurve: 0 };
        pk_call!(PK_SPCURVE_ask(self.tag(), &mut sf));
        Ok((Surf::from_tag(sf.surface), Curve::from_tag(sf.bcurve)))
    }

    /// `PK_CURVE_make_spcurves_2`: SP-curves on `surf` approximating this curve
    /// over `range` to `tolerance`.
    pub fn make_spcurves(
        &self,
        range: (f64, f64),
        surf: &Surf,
        tolerance: f64,
        options: &MakeSpcurvesOptions,
    ) -> PsResult<Vec<Curve>> {
        let iv = PK_INTERVAL_t { low: range.0, high: range.1 };
        let mut o = *options;
        let mut n: c_int = 0;
        let mut p: *mut PK_SPCURVE_t = std::ptr::null_mut();
        pk_call!(PK_CURVE_make_spcurves_2(
            self.tag(),
            &iv,
            surf.tag(),
            tolerance,
            &mut o,
            &mut n,
            &mut p
        ));
        let a = unsafe { PkArray::from_raw(p, n) };
        Ok(a.iter().map(|&t| Curve::from_tag(t)).collect())
    }
}

impl Edge {
    /// `PK_EDGE_set_precision_2` (options v2): set the edge tolerance. On an
    /// edge with no curve this makes a tolerant edge with no fin curves.
    /// Raises both end vertices to at least the edge tolerance. Returns any new
    /// edges.
    pub fn set_precision_2(&self, tol: f64, options: &SetPrecisionOptions) -> PsResult<Vec<Edge>> {
        let mut o = *options;
        let mut n: c_int = 0;
        let mut p: *mut PK_EDGE_t = std::ptr::null_mut();
        pk_call!(PK_EDGE_set_precision_2(self.tag(), tol, &mut o, &mut n, &mut p));
        let a = unsafe { PkArray::from_raw(p, n) };
        Ok(a.iter().map(|&t| Edge::from_tag(t)).collect())
    }

    /// `PK_EDGE_attach_curve_nmnl`: attach a nominal curve to a tolerant edge,
    /// optionally restricted to `interval`. The body's nominal-curve state must
    /// be on ([`Body::set_curve_nmnl_state`]), else 5077 `curve_nmnl_off`.
    pub fn attach_curve_nmnl(&self, curve: &Curve, interval: Option<(f64, f64)>) -> PsResult<()> {
        let mut o = PK_EDGE_attach_curve_nmnl_o_t::default();
        if let Some((low, high)) = interval {
            o.have_interval = PK_LOGICAL_true;
            o.t_int = PK_INTERVAL_t { low, high };
        }
        pk_call!(PK_EDGE_attach_curve_nmnl(self.tag(), curve.tag(), &mut o));
        Ok(())
    }

    /// `PK_EDGE_ask_curve_nmnl`: the nominal curve. On an accurate edge the
    /// kernel returns the edge curve. `None` if there is no curve.
    pub fn curve_nmnl(&self) -> PsResult<Option<Curve>> {
        let mut c = PK_ENTITY_null;
        pk_call!(PK_EDGE_ask_curve_nmnl(self.tag(), &mut c));
        Ok((c != PK_ENTITY_null).then(|| Curve::from_tag(c)))
    }
}

impl Body {
    /// `PK_BODY_set_curve_nmnl_state` (tokens 21200 no / 21201 yes).
    pub fn set_curve_nmnl_state(&self, on: bool) -> PsResult<()> {
        let t = if on { PK_BODY_curve_nmnl_yes_c } else { PK_BODY_curve_nmnl_no_c };
        pk_call!(PK_BODY_set_curve_nmnl_state(self.tag(), t));
        Ok(())
    }

    /// `PK_BODY_make_compound` (options v3): a compound body whose children
    /// are `bodies`.
    pub fn make_compound(bodies: &[Body], options: &MakeCompoundOptions) -> PsResult<Body> {
        let mut b: Vec<i32> = bodies.iter().map(|x| x.tag()).collect();
        let mut o = *options;
        let mut c = PK_ENTITY_null;
        pk_call!(PK_BODY_make_compound(b.len() as c_int, b.as_mut_ptr(), &mut o, &mut c));
        Ok(Body::from_tag(c))
    }

    /// `PK_BODY_ask_children` of a compound body.
    pub fn children(&self) -> PsResult<Vec<Body>> {
        let mut o = PK_BODY_ask_children_o_t::default();
        let mut n: c_int = 0;
        let mut p: *mut PK_BODY_t = std::ptr::null_mut();
        pk_call!(PK_BODY_ask_children(self.tag(), &mut o, &mut n, &mut p));
        let a = unsafe { PkArray::from_raw(p, n) };
        Ok(a.iter().map(|&t| Body::from_tag(t)).collect())
    }

    /// `PK_BODY_ask_parent`: the compound that owns this body, if any.
    pub fn parent(&self) -> PsResult<Option<Body>> {
        let mut o = PK_BODY_ask_parent_o_t::default();
        let mut p = PK_ENTITY_null;
        pk_call!(PK_BODY_ask_parent(self.tag(), &mut o, &mut p));
        Ok((p != PK_ENTITY_null).then(|| Body::from_tag(p)))
    }
}
