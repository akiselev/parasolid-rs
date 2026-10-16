//! Faceting / tessellation — `PK_TOPOL_facet_2` with the version-26 options.
//!
//! [`Body::facet_with`] and [`Face::facet_with`] return a [`Mesh`]: a shared point
//! table, triangles as point indices, the face of each triangle, and one polyline
//! per model edge (indices into the same point table, so a watertightness check
//! can compare both sides of an edge). [`FacetOptions`] exposes the tolerances;
//! [`Mesh::tolerances`] records the tolerances after the kernel's defaulting,
//! computed by the recovered rule (the kernel has no query for them). Any
//! non-zero return code is an error.
//!
//! # ABI (parasolid-re `port/P3-faceting`, pskernel.dll `c900fa3f…63073`)
//!
//! Both option sub-structs are stamped version 26 (mesh 0x198 B, choice 0x50 B;
//! see `parasolid_sys::PK_TOPOL_facet_mesh_2_o_t`). The result is 24 bytes
//! `{n_facets, n_strips, n_fins, n_tables, tables*}`; each table entry is
//! `{fctab, table*}` and each table `{data*, length}`. Until 2026-10-16 this module
//! stamped version 5, whose prefix has no table flags, and read a fictional
//! by-value table struct, so no vertex was ever returned.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::os::raw::c_int;

use parasolid_sys::*;

use crate::body::Body;
use crate::edge::Edge;
use crate::entity::Entity;
use crate::error::PsResult;
use crate::face::Face;
use crate::geom::Vec3;

/// Default curve-chord and surface-plane angle (rad) the kernel applies when no
/// tolerance of that group is given (`FCT_facet` @0x180e6d46e loads 0.4 from
/// 0x183fd5188). Probed: a cylinder of any radius gets 16 chords per circle.
pub const FACET_DEFAULT_ANGLE: f64 = 0.4;

/// How the meshes of neighbouring faces are joined (`match` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FacetMatch {
    /// One point set per model edge, shared by both faces, with co-fins across
    /// faces (`PK_facet_match_topol_c`, the kernel default).
    #[default]
    Topology,
    /// Shared points but no co-fins across faces (`PK_facet_match_geom_c`).
    Geometry,
    /// Separate points per face (`PK_facet_match_trimmed_c`; probed in
    /// parasolid-re `port/P3-faceting` C15, the token's exact meaning is open).
    Trimmed,
}

impl FacetMatch {
    fn token(self) -> PK_facet_match_t {
        match self {
            FacetMatch::Topology => PK_facet_match_topol_c,
            FacetMatch::Geometry => PK_facet_match_geom_c,
            FacetMatch::Trimmed => PK_facet_match_trimmed_c,
        }
    }
}

/// Faceting tolerances. `None` leaves a field to the kernel (`is_X = false`).
///
/// Lengths are model units, angles radians. The kernel refuses (1043
/// `PK_ERROR_bad_tolerance`) a length at or below session precision, a negative
/// value, or an angle above π/2; it refuses widths at or below precision (5026)
/// and `min_facet_width > max_facet_width` (5145).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct FacetOptions {
    /// Max distance from an edge chord to its curve.
    pub curve_chord_tol: Option<f64>,
    /// Max edge chord length.
    pub curve_chord_max: Option<f64>,
    /// Max chord angle: the sum of the two angles between a chord and the curve
    /// tangents at its ends (the arc angle of a circle chord).
    pub curve_chord_ang: Option<f64>,
    /// Max distance from a facet to its surface.
    pub surface_plane_tol: Option<f64>,
    /// Max angle between facet and surface normals.
    pub surface_plane_ang: Option<f64>,
    /// Facets narrower than this are not refined further.
    pub min_facet_width: Option<f64>,
    /// Max facet side length.
    pub max_facet_width: Option<f64>,
    /// Matching between neighbouring faces.
    pub facet_match: FacetMatch,
}

impl FacetOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn curve_chord_tol(mut self, v: f64) -> Self {
        self.curve_chord_tol = Some(v);
        self
    }
    pub fn curve_chord_max(mut self, v: f64) -> Self {
        self.curve_chord_max = Some(v);
        self
    }
    pub fn curve_chord_ang(mut self, v: f64) -> Self {
        self.curve_chord_ang = Some(v);
        self
    }
    pub fn surface_plane_tol(mut self, v: f64) -> Self {
        self.surface_plane_tol = Some(v);
        self
    }
    pub fn surface_plane_ang(mut self, v: f64) -> Self {
        self.surface_plane_ang = Some(v);
        self
    }
    pub fn min_facet_width(mut self, v: f64) -> Self {
        self.min_facet_width = Some(v);
        self
    }
    pub fn max_facet_width(mut self, v: f64) -> Self {
        self.max_facet_width = Some(v);
        self
    }
    pub fn facet_match(mut self, m: FacetMatch) -> Self {
        self.facet_match = m;
        self
    }

    fn to_pk(self) -> PK_TOPOL_facet_2_o_t {
        let mut o = PK_TOPOL_facet_2_o_t::default();
        let c = &mut o.control;
        c.match_ = self.facet_match.token();
        let set = |flag: &mut PK_LOGICAL_t, val: &mut f64, v: Option<f64>| {
            if let Some(v) = v {
                *flag = PK_LOGICAL_true;
                *val = v;
            }
        };
        set(
            &mut c.is_min_facet_width,
            &mut c.min_facet_width,
            self.min_facet_width,
        );
        set(
            &mut c.is_max_facet_width,
            &mut c.max_facet_width,
            self.max_facet_width,
        );
        set(
            &mut c.is_curve_chord_tol,
            &mut c.curve_chord_tol,
            self.curve_chord_tol,
        );
        set(
            &mut c.is_curve_chord_max,
            &mut c.curve_chord_max,
            self.curve_chord_max,
        );
        set(
            &mut c.is_curve_chord_ang,
            &mut c.curve_chord_ang,
            self.curve_chord_ang,
        );
        set(
            &mut c.is_surface_plane_tol,
            &mut c.surface_plane_tol,
            self.surface_plane_tol,
        );
        set(
            &mut c.is_surface_plane_ang,
            &mut c.surface_plane_ang,
            self.surface_plane_ang,
        );
        let t = &mut o.choice;
        t.facet_fin = PK_LOGICAL_true;
        t.fin_data = PK_LOGICAL_true;
        t.data_point_idx = PK_LOGICAL_true;
        t.point_vec = PK_LOGICAL_true;
        t.facet_face = PK_LOGICAL_true;
        t.fin_edge = PK_LOGICAL_true;
        t.error_object = PK_LOGICAL_true;
        o
    }

    /// The tolerances the kernel uses for this request.
    ///
    /// Not read back from the kernel (it exposes no query): computed with the
    /// rules recovered in parasolid-re `port/P3-faceting` (cases C2–C5):
    /// - no curve tolerance given → `curve_chord_ang = 0.4` (tol and max off);
    ///   no surface tolerance given → `surface_plane_ang = 0.4` (tol off).
    ///   Giving any member of a group switches that group's default off
    ///   (`FCT_facet` @0x180e6cdc0, current-version branch; probed);
    /// - chord tol, chord max, surface tol and max width below 2 × session
    ///   precision are raised to it; min width below 1 × precision is raised to it;
    ///   angles below 2e-6 are raised to 2e-6 (`FUN_1814cedb0`, static only);
    /// - a chord max longer than the model size (+ precision) is switched off
    ///   (`FCT_facet`; behaviour-neutral, static only). The kernel's model size is
    ///   the largest box diagonal; this uses the box of each faceted topology,
    ///   which for a single face may be smaller than the kernel's.
    fn effective(&self, precision: f64, model_size: f64) -> FacetTolerances {
        let floor = |v: Option<f64>, f: f64| v.map(|x| if x > 0.0 && x < f { f } else { x });
        let ang = |v: Option<f64>| v.map(|x| if x > 0.0 && x < 2e-6 { 2e-6 } else { x });
        let mut t = FacetTolerances {
            curve_chord_tol: floor(self.curve_chord_tol, 2.0 * precision),
            curve_chord_max: floor(self.curve_chord_max, 2.0 * precision),
            curve_chord_ang: ang(self.curve_chord_ang),
            surface_plane_tol: floor(self.surface_plane_tol, 2.0 * precision),
            surface_plane_ang: ang(self.surface_plane_ang),
            min_facet_width: floor(self.min_facet_width, precision),
            max_facet_width: floor(self.max_facet_width, 2.0 * precision),
            facet_match: self.facet_match,
        };
        if self.curve_chord_tol.is_none()
            && self.curve_chord_max.is_none()
            && self.curve_chord_ang.is_none()
        {
            t.curve_chord_ang = Some(FACET_DEFAULT_ANGLE);
        }
        if self.surface_plane_tol.is_none() && self.surface_plane_ang.is_none() {
            t.surface_plane_ang = Some(FACET_DEFAULT_ANGLE);
        }
        if let Some(m) = t.curve_chord_max {
            if m - model_size > precision {
                t.curve_chord_max = None;
            }
        }
        t
    }
}

/// The tolerances a faceting call used, after the kernel's defaulting. `None`
/// means the control is off. See [`Mesh::tolerances`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FacetTolerances {
    pub curve_chord_tol: Option<f64>,
    pub curve_chord_max: Option<f64>,
    pub curve_chord_ang: Option<f64>,
    pub surface_plane_tol: Option<f64>,
    pub surface_plane_ang: Option<f64>,
    pub min_facet_width: Option<f64>,
    pub max_facet_width: Option<f64>,
    pub facet_match: FacetMatch,
}

/// The facet points along one model edge, as indices into [`Mesh::points`].
///
/// Built from the fins of the first face (in `fin_edge` order) that lies on the
/// edge. With [`FacetMatch::Topology`] the other face uses the same points.
#[derive(Debug, Clone, PartialEq)]
pub struct EdgePolyline {
    pub edge: Edge,
    /// Points in the sense of that face's fins along the edge. A closed edge (a
    /// ring) does not repeat its first point.
    pub points: Vec<u32>,
    pub closed: bool,
}

/// One facet fin lying on a model edge: `points` are its tail and head.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundaryFin {
    pub edge: Edge,
    pub face: Face,
    pub points: [u32; 2],
}

/// A triangle mesh from `PK_TOPOL_facet_2`.
#[derive(Debug, Clone, PartialEq)]
pub struct Mesh {
    /// Facet count reported by the kernel (= `triangles.len()`).
    pub n_facets: usize,
    pub n_strips: usize,
    pub n_fins: usize,
    /// Shared point table.
    pub points: Vec<Vec3>,
    /// Triangles as indices into `points`, in fin order.
    pub triangles: Vec<[u32; 3]>,
    /// The face of each triangle (parallel to `triangles`).
    pub facet_faces: Vec<Face>,
    /// One polyline per model edge that has fins, in edge-tag order.
    pub edges: Vec<EdgePolyline>,
    /// Every fin on a model edge. With [`FacetMatch::Topology`] both faces'
    /// fins on an edge use the points of that edge's polyline.
    pub boundary_fins: Vec<BoundaryFin>,
    /// `error_object` records `{error, object}`. Seen once in parasolid-re
    /// `port/P3-faceting` (C18: a body beyond the size box returns no error
    /// code, no facets and one row); the error values are not decoded.
    pub faults: Vec<[i32; 2]>,
    /// The tolerances used, after the kernel's defaulting (computed, see
    /// [`FacetTolerances`]).
    pub tolerances: FacetTolerances,
}

impl Mesh {
    /// The polyline of `edge`, if any fin lies on it.
    pub fn edge_polyline(&self, edge: Edge) -> Option<&EdgePolyline> {
        self.edges
            .binary_search_by_key(&edge.tag(), |p| p.edge.tag())
            .ok()
            .map(|i| &self.edges[i])
    }
}

/// Copy the kernel's tables out of a facet result (records as flat `i32`/`f64`).
struct RawTables {
    ints: HashMap<PK_TOPOL_fctab_t, Vec<i32>>,
    dbls: HashMap<PK_TOPOL_fctab_t, Vec<f64>>,
}

/// Ints / doubles per record for the tables this module requests.
fn record_shape(fctab: PK_TOPOL_fctab_t) -> (usize, usize) {
    match fctab {
        PK_TOPOL_fctab_facet_fin_c | PK_TOPOL_fctab_fin_edge_c | PK_TOPOL_fctab_error_object_c => {
            (2, 0)
        }
        PK_TOPOL_fctab_fin_data_c | PK_TOPOL_fctab_data_point_c | PK_TOPOL_fctab_facet_face_c => {
            (1, 0)
        }
        PK_TOPOL_fctab_point_vec_c => (0, 3),
        _ => (0, 0),
    }
}

fn read_tables(r: &PK_TOPOL_facet_2_r_t) -> RawTables {
    let mut out = RawTables {
        ints: HashMap::new(),
        dbls: HashMap::new(),
    };
    if r.tables.is_null() {
        return out;
    }
    for i in 0..r.n_tables.max(0) as usize {
        // SAFETY: the kernel allocated `n_tables` entries; each `table` points to
        // a `{data*, length}` whose data holds `length` records of the shape the
        // fctab token names (return_tables_2 @0x180458940).
        unsafe {
            let ent = &*r.tables.add(i);
            if ent.table.is_null() {
                continue;
            }
            let t = &*ent.table;
            let (ni, nd) = record_shape(ent.fctab);
            if t.data.is_null() || t.length <= 0 {
                continue;
            }
            let n = t.length as usize;
            if ni > 0 {
                out.ints.insert(
                    ent.fctab,
                    std::slice::from_raw_parts(t.data as *const i32, n * ni).to_vec(),
                );
            } else if nd > 0 {
                out.dbls.insert(
                    ent.fctab,
                    std::slice::from_raw_parts(t.data as *const f64, n * nd).to_vec(),
                );
            }
        }
    }
    out
}

fn facet_topols(topols: &[i32], opts: &FacetOptions) -> PsResult<Mesh> {
    let mut precision = 0.0;
    pk_call!(PK_SESSION_ask_precision(&mut precision));
    let mut model_size = 0.0f64;
    for &t in topols {
        let mut b = PK_BOX_t { coord: [0.0; 6] };
        pk_call!(PK_TOPOL_find_box(t, &mut b));
        let d = [
            b.coord[3] - b.coord[0],
            b.coord[4] - b.coord[1],
            b.coord[5] - b.coord[2],
        ];
        model_size = model_size.max((d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt());
    }
    let mut o = opts.to_pk();
    let mut t = topols.to_vec();
    // SAFETY: zeroed is a valid empty result (counts 0, null table pointer).
    let mut r: PK_TOPOL_facet_2_r_t = unsafe { std::mem::zeroed() };
    let code = crate::budget::guard(|| unsafe {
        PK_TOPOL_facet_2(
            t.len() as c_int,
            t.as_mut_ptr(),
            std::ptr::null_mut(),
            &mut o,
            &mut r,
        )
    });
    let code = match code {
        Ok(code) => code,
        Err(budget) => {
            if !r.tables.is_null() {
                // SAFETY: the kernel allocated these tables; free them before reporting.
                unsafe { PK_TOPOL_facet_2_r_f(&mut r) };
            }
            return Err(budget);
        }
    };
    if code != 0 {
        if !r.tables.is_null() {
            // SAFETY: the kernel allocated these tables; free them before reporting.
            unsafe { PK_TOPOL_facet_2_r_f(&mut r) };
        }
        return Err(crate::error::PsError::from_code(code));
    }
    let raw = read_tables(&r);
    let (n_facets, n_strips, n_fins) = (
        r.n_facets.max(0) as usize,
        r.n_strips.max(0) as usize,
        r.n_fins.max(0) as usize,
    );
    pk_call!(PK_TOPOL_facet_2_r_f(&mut r));

    let empty = Vec::new();
    let get = |k| raw.ints.get(&k).unwrap_or(&empty);
    let (facet_fin, fin_data, data_point, facet_face, fin_edge) = (
        get(PK_TOPOL_fctab_facet_fin_c),
        get(PK_TOPOL_fctab_fin_data_c),
        get(PK_TOPOL_fctab_data_point_c),
        get(PK_TOPOL_fctab_facet_face_c),
        get(PK_TOPOL_fctab_fin_edge_c),
    );
    let points: Vec<Vec3> = raw
        .dbls
        .get(&PK_TOPOL_fctab_point_vec_c)
        .map(|v| {
            v.chunks_exact(3)
                .map(|c| Vec3::new(c[0], c[1], c[2]))
                .collect()
        })
        .unwrap_or_default();
    // Each fin's point, through fin -> data -> point. Checked once here so the
    // assembly below can index freely.
    let fin_point: Vec<u32> = (0..n_fins)
        .map(|k| {
            let d = *fin_data.get(k)?;
            let p = *data_point.get(usize::try_from(d).ok()?)?;
            usize::try_from(p).ok().filter(|&p| p < points.len()).map(|p| p as u32)
        })
        .collect::<Option<_>>()
        .ok_or_else(|| {
            crate::error::PsError::Session(format!(
                "PK_TOPOL_facet_2 tables are inconsistent: {n_fins} fins, {} fin_data, {} data_point, {} points",
                fin_data.len(),
                data_point.len(),
                points.len()
            ))
        })?;

    // Fins of each facet in order, from the {facet, fin} lookup.
    let mut facet_fins: Vec<Vec<usize>> = vec![Vec::new(); n_facets];
    let mut fin_facet = vec![usize::MAX; n_fins];
    for rec in facet_fin.chunks_exact(2) {
        let (f, k) = (rec[0] as usize, rec[1] as usize);
        if f < n_facets && k < n_fins {
            facet_fins[f].push(k);
            fin_facet[k] = f;
        }
    }
    let face_of = |f: usize| Face::from_tag(facet_face.get(f).copied().unwrap_or(PK_ENTITY_null));
    // max_facet_sides is 3, so every facet is a triangle; fan anything else.
    let mut triangles = Vec::with_capacity(n_facets);
    let mut facet_faces = Vec::with_capacity(n_facets);
    for (f, fins) in facet_fins.iter().enumerate() {
        for j in 1..fins.len().saturating_sub(1) {
            triangles.push([
                fin_point[fins[0]],
                fin_point[fins[j]],
                fin_point[fins[j + 1]],
            ]);
            facet_faces.push(face_of(f));
        }
    }
    // A fin's point is its head: fin k runs from the head of the previous fin of
    // its facet to its own head (parasolid-re port/P3-faceting probe; its edge
    // chords measured on the curve to 2e-15).
    let tail = |k: usize| {
        let fins = &facet_fins[fin_facet[k]];
        let i = fins.iter().position(|&x| x == k).unwrap_or(0);
        fins[(i + fins.len() - 1) % fins.len()]
    };
    let mut boundary_fins = Vec::new();
    for rec in fin_edge.chunks_exact(2) {
        let k = rec[0] as usize;
        if k >= n_fins || fin_facet[k] == usize::MAX {
            continue;
        }
        boundary_fins.push(BoundaryFin {
            edge: Edge::from_tag(rec[1]),
            face: face_of(fin_facet[k]),
            points: [fin_point[tail(k)], fin_point[k]],
        });
    }
    let edges = edge_polylines(&boundary_fins);
    let faults = get(PK_TOPOL_fctab_error_object_c)
        .chunks_exact(2)
        .map(|c| [c[0], c[1]])
        .collect();
    Ok(Mesh {
        n_facets,
        n_strips,
        n_fins,
        points,
        triangles,
        facet_faces,
        edges,
        boundary_fins,
        faults,
        tolerances: opts.effective(precision, model_size),
    })
}

/// Chain the fins of the first face seen on each edge into a polyline.
///
/// The segments are walked as an undirected chain, so a face that has both fins
/// of an edge still gives one polyline. The walk starts at the chain's lowest
/// tail end (open) or at the first fin's tail, following its sense (closed).
fn edge_polylines(fins: &[BoundaryFin]) -> Vec<EdgePolyline> {
    let mut first_face: HashMap<i32, i32> = HashMap::new();
    let mut segs: BTreeMap<i32, Vec<[u32; 2]>> = BTreeMap::new();
    for b in fins {
        let face = *first_face.entry(b.edge.tag()).or_insert(b.face.tag());
        if face == b.face.tag() && b.points[0] != b.points[1] {
            segs.entry(b.edge.tag()).or_default().push(b.points);
        }
    }
    segs.into_iter()
        .map(|(tag, segs)| {
            let mut seen = HashSet::new();
            let mut adj: HashMap<u32, Vec<u32>> = HashMap::new();
            for &[a, b] in &segs {
                if seen.insert((a.min(b), a.max(b))) {
                    adj.entry(a).or_default().push(b);
                    adj.entry(b).or_default().push(a);
                }
            }
            let tails: HashSet<u32> = segs.iter().map(|s| s[0]).collect();
            let ends: Vec<u32> = adj
                .iter()
                .filter(|(_, n)| n.len() == 1)
                .map(|(&p, _)| p)
                .collect();
            let closed = ends.is_empty();
            let start = (ends.iter().filter(|p| tails.contains(p)).min())
                .or(ends.iter().min())
                .copied()
                .unwrap_or(segs[0][0]);
            let mut points = vec![start];
            let (mut prev, mut at) = (u32::MAX, start);
            if closed {
                (prev, at) = (start, segs[0][1]);
                points.push(at);
            }
            while points.len() <= seen.len() {
                let Some(&n) = adj[&at].iter().find(|&&n| n != prev) else {
                    break;
                };
                if n == start {
                    break;
                }
                points.push(n);
                (prev, at) = (at, n);
            }
            EdgePolyline {
                edge: Edge::from_tag(tag),
                points,
                closed,
            }
        })
        .collect()
}

impl Body {
    /// Facet this body with the kernel's default tolerances.
    pub fn facet(&self) -> PsResult<Mesh> {
        self.facet_with(&FacetOptions::default())
    }

    /// Facet this body (`PK_TOPOL_facet_2`, options version 26).
    pub fn facet_with(&self, opts: &FacetOptions) -> PsResult<Mesh> {
        facet_topols(&[self.tag], opts)
    }
}

impl Face {
    /// Facet this face alone with the kernel's default tolerances.
    pub fn facet(&self) -> PsResult<Mesh> {
        self.facet_with(&FacetOptions::default())
    }

    /// Facet this face alone. Topology matching does not apply: its edges get
    /// their own curve threads, without the neighbours' contributions.
    pub fn facet_with(&self, opts: &FacetOptions) -> PsResult<Mesh> {
        facet_topols(&[self.tag()], opts)
    }
}

// =============================================================================
// Convergent-modeling mesh (PK_MESH_t) built from facet data
// =============================================================================

/// A **convergent-modeling mesh** entity (`PK_MESH_t`), built from raw facet data
/// via the callback-based `PK_MESH_create_from_facets`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FacetMesh {
    tag: PK_MESH_t,
}

/// Context passed to the facet-reader callback: a single independent-facet
/// "vector" block, delivered on the one reader call with `status = stop`.
struct FacetVectorContext {
    block: PK_MESH_facet_vector_t,
}

/// The facet-reader callback the kernel calls repeatedly. Emits one vector block
/// (all triangles, 3 positions each), then reports `stop`.
unsafe extern "C" fn facet_vector_reader(
    context: *mut std::os::raw::c_void,
    descriptor: *mut PK_MESH_facet_descriptor_t,
    status: *mut PK_MESH_cb_status_t,
) -> PK_ERROR_code_t {
    unsafe {
        let ctx = &mut *(context as *mut FacetVectorContext);
        // Deliver the single block AND signal `stop` on the SAME call: the
        // consumer (`FUN_1813f30a0`) dispatches the descriptor after every reader
        // call and treats `stop` as "this is the last block" (still processed),
        // NOT "no more data". `continue` means "another block follows"; emitting
        // a block on a continue-call and then stopping on the next call would
        // dispatch the block twice. Status is the out-param; return is ignored.
        (*descriptor).facet_type = PK_MESH_facet_type_vector_internal_c;
        (*descriptor).facet_data =
            (&mut ctx.block) as *mut PK_MESH_facet_vector_t as *mut std::os::raw::c_void;
        *status = PK_MESH_cb_status_stop_c;
    }
    PK_ERROR_no_errors
}

impl FacetMesh {
    pub(crate) fn from_tag(tag: PK_MESH_t) -> Self {
        Self { tag }
    }
    pub fn tag(&self) -> i32 {
        self.tag
    }
    pub fn entity(&self) -> Entity {
        Entity::from_tag(self.tag)
    }

    /// Build a convergent mesh from independent triangles (each three corner
    /// points). Wraps `PK_MESH_create_from_facets` with an independent-facet
    /// "vector" reader; the kernel groups every 3 positions into one facet.
    pub fn from_triangles(triangles: &[[Vec3; 3]]) -> PsResult<FacetMesh> {
        // Convergent Modeling (facet geometry) is DISABLED by default in every
        // session; `PK_MESH_create_from_facets` returns 5237 unless it is turned
        // on first (the kernel gates on a `DS_roll_data()+0x48` byte). Enable it
        // for the session before building the mesh — see ch082 §82.2.1.
        pk_call!(PK_SESSION_set_facet_geometry(PK_facet_geometry_all_c));

        // Flatten to `n_triangles * 3` vertex positions (kept alive for the call).
        let positions: Vec<f64> = triangles
            .iter()
            .flat_map(|t| t.iter().flat_map(|v| [v.x, v.y, v.z]))
            .collect();
        let n_positions = (triangles.len() * 3) as c_int;
        let mut ctx = FacetVectorContext {
            block: PK_MESH_facet_vector_t {
                // First field is the FACET count, not the vertex count (the
                // consumer reads 3 vertices per facet). Passing n_positions
                // over-reads the buffer 3× and fails with PK error 900.
                n_facets: triangles.len() as c_int,
                vertex_positions: positions.as_ptr(),
                vertex_normals: std::ptr::null(),
            },
        };
        let opts = PK_MESH_create_from_facets_o_t {
            o_t_version: 1,
            vertices_estimate: n_positions,
            facet_estimate: triangles.len() as c_int,
            facet_free: None,
            create: PK_MESH_create_now_c,
            have_box: PK_LOGICAL_false,
            box_: PK_BOX_t { coord: [0.0; 6] },
            thread_safe: PK_LOGICAL_false,
        };
        let mut mesh: PK_MESH_t = PK_ENTITY_null;
        let code = unsafe {
            PK_MESH_create_from_facets(
                Some(facet_vector_reader),
                (&mut ctx) as *mut FacetVectorContext as *mut std::os::raw::c_void,
                &opts,
                &mut mesh,
            )
        };
        // Surface the outcome faithfully: the mesh tag is written (non-null) only
        // when construction succeeds. When the convergent-modeling engine rejects
        // the facet data it returns a mild code (e.g. 5241 =
        // `PSM_mesh_create_result` 4/9) and leaves the tag null — that is a real
        // failure, not a tolerable warning, so report it.
        if mesh == PK_ENTITY_null {
            return Err(crate::error::PsError::from_code(code));
        }
        // Non-null tag: tolerate a mild warning, abort on serious/fatal.
        if code != 0 {
            match crate::error::PsError::from_code(code) {
                crate::error::PsError::Mild(_) => {}
                other => return Err(other),
            }
        }
        // `positions` is still alive here (dropped after the call returns).
        Ok(FacetMesh::from_tag(mesh))
    }

    /// Number of facets (mfacets) in the mesh.
    pub fn n_facets(&self) -> PsResult<i32> {
        let mut n = 0;
        pk_call!(PK_MESH_ask_n_mfacets(self.tag, &mut n));
        Ok(n)
    }

    /// Number of vertices (mvertices) in the mesh.
    pub fn n_vertices(&self) -> PsResult<i32> {
        let mut n = 0;
        pk_call!(PK_MESH_ask_n_mvertices(self.tag, &mut n));
        Ok(n)
    }
}
