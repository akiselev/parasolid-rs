//! Adversarial-review probe: verify result-struct extents and PK_CLASS tokens.
//! Read-only w.r.t. the repo; builds only itself.

use parasolid::*;
use parasolid_sys::*;
use std::os::raw::c_int;

const PAD: usize = 128;

fn dump(label: &str, buf: &[u8; PAD], claimed: usize) {
    // print the first 64 bytes as qwords
    print!("  {label}: ");
    for i in (0..64).step_by(8) {
        let mut q = [0u8; 8];
        q.copy_from_slice(&buf[i..i + 8]);
        let v = u64::from_le_bytes(q);
        let mark = if i >= claimed { "*" } else { " " };
        print!("[{:#04x}]{}{:#018x} ", i, mark, v);
    }
    println!();
    let mut last = 0usize;
    for i in 0..PAD {
        if buf[i] != 0 {
            last = i + 1;
        }
    }
    println!(
        "    -> highest non-zero byte offset written = {last} (Rust struct claims {claimed} bytes){}",
        if last > claimed {
            "   <<< OUT OF BOUNDS"
        } else {
            ""
        }
    );
}

fn main() {
    let _s = Session::start(SessionConfig::new().check_arguments(true)).expect("session");

    // ---------------------------------------------------------------
    // 1. PK_boolean_r_t / PK_TOPOL_track_r_t real extents
    // ---------------------------------------------------------------
    println!("== PK_BODY_boolean_2 result/tracking struct extents ==");
    println!(
        "  sizeof(PK_boolean_r_t) in parasolid-sys      = {}",
        std::mem::size_of::<PK_boolean_r_t>()
    );
    println!(
        "  sizeof(PK_TOPOL_track_r_t) in parasolid-sys  = {}",
        std::mem::size_of::<PK_TOPOL_track_r_t>()
    );

    let a = Body::create_solid_block(10.0, 10.0, 10.0).expect("block a");
    let b = Body::create_solid_block(10.0, 10.0, 10.0).expect("block b");
    // move b so the union is a genuine boolean
    let t = Transform::translation(5.0, 0.0, 0.0).expect("transf");
    b.transform(&t).expect("transform b");

    let mut track_buf = [0u8; PAD];
    let mut res_buf = [0u8; PAD];

    let mut opts = PK_BODY_boolean_o_t::default();
    opts.function = PK_boolean_unite_c;

    let tools = [b.tag()];
    let code = unsafe {
        PK_BODY_boolean_2(
            a.tag(),
            1,
            tools.as_ptr(),
            &opts,
            track_buf.as_mut_ptr() as *mut PK_TOPOL_track_r_t,
            res_buf.as_mut_ptr() as *mut PK_boolean_r_t,
        )
    };
    println!("  PK_BODY_boolean_2 -> {code}");
    dump(
        "PK_boolean_r_t   ",
        &res_buf,
        std::mem::size_of::<PK_boolean_r_t>(),
    );
    dump(
        "PK_TOPOL_track_r_t",
        &track_buf,
        std::mem::size_of::<PK_TOPOL_track_r_t>(),
    );

    // what does the wrapper's struct view see?
    let rv = unsafe { &*(res_buf.as_ptr() as *const PK_boolean_r_t) };
    println!(
        "  result={} n_bodies={} bodies={:p}",
        rv.result, rv.n_bodies, rv.bodies
    );
    // raw ints at 0x10 / 0x18 (n_faults / faults) that the Rust struct cannot see
    let raw = res_buf.as_ptr();
    let n_faults = unsafe { *(raw.add(0x10) as *const c_int) };
    let faults = unsafe { *(raw.add(0x18) as *const usize) };
    println!("  RAW @0x10 (n_faults?) = {n_faults}   RAW @0x18 (faults ptr?) = {faults:#x}");

    unsafe { PK_TOPOL_track_r_f(track_buf.as_mut_ptr() as *mut PK_TOPOL_track_r_t) };
    unsafe { PK_boolean_r_f(res_buf.as_mut_ptr() as *mut PK_boolean_r_t) };

    // ---------------------------------------------------------------
    // 2. Tracking-enabled boolean (tracking=true option)
    // ---------------------------------------------------------------
    println!("\n== boolean with tracking requested ==");
    let a2 = Body::create_solid_block(10.0, 10.0, 10.0).expect("block a2");
    let b2 = Body::create_solid_block(10.0, 10.0, 10.0).expect("block b2");
    let t2 = Transform::translation(5.0, 0.0, 0.0).expect("transf2");
    b2.transform(&t2).expect("transform b2");
    let mut opts2 = PK_BODY_boolean_o_t::default();
    opts2.function = PK_boolean_unite_c;
    let mut track2 = [0u8; PAD];
    let mut res2 = [0u8; PAD];
    let tools2 = [b2.tag()];
    let code2 = unsafe {
        PK_BODY_boolean_2(
            a2.tag(),
            1,
            tools2.as_ptr(),
            &opts2,
            track2.as_mut_ptr() as *mut PK_TOPOL_track_r_t,
            res2.as_mut_ptr() as *mut PK_boolean_r_t,
        )
    };
    println!("  PK_BODY_boolean_2 (tracking) -> {code2}");
    dump(
        "PK_TOPOL_track_r_t",
        &track2,
        std::mem::size_of::<PK_TOPOL_track_r_t>(),
    );
    unsafe { PK_TOPOL_track_r_f(track2.as_mut_ptr() as *mut PK_TOPOL_track_r_t) };
    unsafe { PK_boolean_r_f(res2.as_mut_ptr() as *mut PK_boolean_r_t) };

    // ---------------------------------------------------------------
    // 3. PK_TOPOL_local_r_t extent from PK_BODY_extrude / hollow
    // ---------------------------------------------------------------
    println!("\n== PK_TOPOL_local_r_t extent (via PK_BODY_hollow_2) ==");
    let solid = Body::create_solid_block(10.0, 10.0, 10.0).expect("solid");
    let mut trk = [0u8; PAD];
    let mut loc = [0u8; PAD];
    let hc = unsafe {
        PK_BODY_hollow_2(
            solid.tag(),
            -1.0,
            1.0e-6,
            std::ptr::null(),
            trk.as_mut_ptr() as *mut PK_TOPOL_track_r_t,
            loc.as_mut_ptr() as *mut PK_TOPOL_local_r_t,
        )
    };
    println!("  PK_BODY_hollow_2 -> {hc}");
    dump("PK_TOPOL_local_r_t", &loc, 16);
    let status = unsafe { *(loc.as_ptr() as *const c_int) };
    println!("  local status token @0 = {status} (21450=ok, 21452=fail, 21456=cant_offset)");
    unsafe { PK_TOPOL_track_r_f(trk.as_mut_ptr() as *mut PK_TOPOL_track_r_t) };

    // ---------------------------------------------------------------
    // 4. PK_CLASS token probe for the [unknown] sentinels
    // ---------------------------------------------------------------
    println!("\n== PK_CLASS tokens for entities the wrapper maps to sentinel values ==");
    let blk = Body::create_solid_block(1.0, 1.0, 1.0).expect("blk");
    let mut c: PK_CLASS_t = -999;
    unsafe { PK_ENTITY_ask_class(blk.tag(), &mut c) };
    println!("  body                    -> {c}  (sys PK_CLASS_body={PK_CLASS_body})");

    let tr = Transform::translation(1.0, 0.0, 0.0).expect("tr");
    let mut c: PK_CLASS_t = -999;
    let e = unsafe { PK_ENTITY_ask_class(tr.tag(), &mut c) };
    println!("  transform (err {e})      -> {c}  (sys PK_CLASS_transf={PK_CLASS_transf}) ");

    // attribute: attach an int attribute to the body
    let mut attdef: PK_ATTDEF_t = PK_ENTITY_null;
    let name = std::ffi::CString::new("SDL/TYSA_COLOUR").unwrap();
    let e = unsafe { PK_ATTDEF_find(name.as_ptr(), &mut attdef) };
    println!("  PK_ATTDEF_find(colour) -> err {e}, attdef {attdef}");
    if attdef != PK_ENTITY_null {
        let mut c: PK_CLASS_t = -999;
        unsafe { PK_ENTITY_ask_class(attdef, &mut c) };
        println!("  attdef                  -> {c}  (sys PK_CLASS_attdef={PK_CLASS_attdef})");
    }

    // walk classes 6000..6010 to see which are valid via superclass query
    println!("  -- class hierarchy scan 5000..6020 --");
    for cls in 5000..6020 {
        let mut sup: PK_CLASS_t = -999;
        let e = unsafe { PK_CLASS_ask_superclass(cls, &mut sup) };
        if e == 0 {
            println!("     class {cls} -> superclass {sup}");
        }
    }

    println!("\n== PkClass::from_raw round-trip for sentinel classes ==");
    for (n, v) in [
        ("attrib", PK_CLASS_attrib),
        ("attdef", PK_CLASS_attdef),
        ("mesh", PK_CLASS_mesh),
        ("transf", PK_CLASS_transf),
    ] {
        println!("     {n} sentinel = {v}");
    }

    // ---------------------------------------------------------------
    // 6. BodyType catch-all collapses minimum/compound/unspecified -> General
    // ---------------------------------------------------------------
    println!("\n== Body::body_type() catch-all ==");
    let minb = Body::create_minimum(Vec3::new(0.0, 0.0, 0.0)).expect("min body");
    let mut raw: PK_BODY_type_t = -1;
    unsafe { PK_BODY_ask_type(minb.tag(), &mut raw) };
    println!(
        "  minimum body: raw PK_BODY_ask_type = {raw} (5603=minimum), wrapper says {:?}",
        minb.body_type()
    );
    let wire = Body::create_solid_block(1.0, 1.0, 1.0).expect("b");
    let mut raw2: PK_BODY_type_t = -1;
    unsafe { PK_BODY_ask_type(wire.tag(), &mut raw2) };
    println!(
        "  solid  body: raw = {raw2}, wrapper says {:?}",
        wire.body_type()
    );

    // ---------------------------------------------------------------
    // 7. Body::hollow / section drop kernel status + leak results
    // ---------------------------------------------------------------
    println!("\n== Body::hollow with an impossible wall thickness ==");
    let hb = Body::create_solid_block(2.0, 2.0, 2.0).expect("hb");
    println!(
        "  Body::hollow(wall=5.0 on a 2-cube) -> {:?}",
        hb.hollow(5.0)
    );
    let mut trk2 = [0u8; PAD];
    let mut loc2 = [0u8; PAD];
    let hb2 = Body::create_solid_block(2.0, 2.0, 2.0).expect("hb2");
    let hc2 = unsafe {
        PK_BODY_hollow_2(
            hb2.tag(),
            -5.0,
            1.0e-6,
            std::ptr::null(),
            trk2.as_mut_ptr() as *mut PK_TOPOL_track_r_t,
            loc2.as_mut_ptr() as *mut PK_TOPOL_local_r_t,
        )
    };
    let st2 = unsafe { *(loc2.as_ptr() as *const c_int) };
    let n2 = unsafe { *(loc2.as_ptr().add(4) as *const c_int) };
    let p2 = unsafe { *(loc2.as_ptr().add(8) as *const usize) };
    println!(
        "  raw PK_BODY_hollow_2 -> err {hc2}, local status={st2} (21450=ok,21452=fail,21456=cant_offset), n={n2}, ptr={p2:#x}"
    );
    unsafe { PK_TOPOL_track_r_f(trk2.as_mut_ptr() as *mut PK_TOPOL_track_r_t) };

    // ---------------------------------------------------------------
    // 8. ask_bcurve returns homogeneous coords for rational curves
    // ---------------------------------------------------------------
    println!("\n== Curve::ask_bcurve on a RATIONAL b-curve ==");
    let cps = [
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
    ];
    let ws = [1.0, 4.0, 1.0];
    let knots = [0.0, 1.0];
    let mults = [3, 3];
    match Curve::bcurve_rational(2, &cps, &ws, &knots, &mults) {
        Ok(c) => match c.ask_bcurve() {
            Ok(d) => {
                println!("  input control points  : {:?}", cps);
                println!("  input weights         : {:?}", ws);
                println!("  is_rational           : {}", d.is_rational);
                println!("  ask_bcurve returns    : {:?}", d.control_points);
                println!(
                    "  knots returned        : {:?} (input distinct knots {:?}, mults {:?} DROPPED)",
                    d.knots, knots, mults
                );
            }
            Err(e) => println!("  ask_bcurve err {e:?}"),
        },
        Err(e) => println!("  bcurve_rational err {e:?}"),
    }

    // ---------------------------------------------------------------
    // 9. Edge::is_planar leaks a kernel-created plane entity
    // ---------------------------------------------------------------
    println!("\n== Edge::is_planar orphan-plane leak ==");
    let cyl = Body::create_solid_cylinder(1.0, 2.0).expect("cyl");
    let ce = cyl.edges().expect("edges");
    let mut plane_tags = Vec::new();
    for _ in 0..5 {
        let mut ip: PK_LOGICAL_t = PK_LOGICAL_false;
        let mut pl: PK_PLANE_t = PK_ENTITY_null;
        unsafe { PK_EDGE_is_planar(ce[0].tag(), PK_LOGICAL_true, &mut ip, &mut pl) };
        plane_tags.push(pl);
    }
    println!(
        "  5 raw PK_EDGE_is_planar(want_plane=true) calls returned plane tags: {plane_tags:?}"
    );
    println!(
        "  -> distinct non-null tags means each call creates a NEW orphan PK_PLANE_t that is_planar() never deletes"
    );

    // ---------------------------------------------------------------
    // 10. PK_SESSION_ask_memory_usage really takes TWO qword out-params
    // ---------------------------------------------------------------
    println!("\n== PK_SESSION_ask_memory_usage arity/width ==");
    unsafe extern "C" {
        #[link_name = "PK_SESSION_ask_memory_usage"]
        fn mem_usage_2(a: *mut u64, b: *mut u64) -> c_int;
    }
    // Give each out-param a padded, poisoned buffer so we can see the real width.
    let mut buf_a = [0xAAu8; 32];
    let mut buf_b = [0xBBu8; 32];
    let e = unsafe {
        mem_usage_2(
            buf_a.as_mut_ptr() as *mut u64,
            buf_b.as_mut_ptr() as *mut u64,
        )
    };
    let a0 = u64::from_le_bytes(buf_a[0..8].try_into().unwrap());
    let a1 = u64::from_le_bytes(buf_a[8..16].try_into().unwrap());
    let b0 = u64::from_le_bytes(buf_b[0..8].try_into().unwrap());
    let b1 = u64::from_le_bytes(buf_b[8..16].try_into().unwrap());
    println!("  err={e}");
    println!(
        "  arg1 buffer: [0..8]={a0} (0x{a0:x})   [8..16]=0x{a1:x} (unchanged poison 0xAAAA.. means 8 bytes written)"
    );
    println!("  arg2 buffer: [0..8]={b0} (0x{b0:x})   [8..16]=0x{b1:x}");
    println!(
        "  parasolid-sys declares: PK_SESSION_ask_memory_usage(n_bytes: *mut c_int) -- ONE 4-byte out-param"
    );
    println!("  Session::memory_usage() passes &mut i32 (4 bytes) and NOTHING for arg2.");

    // ---------------------------------------------------------------
    // 5. Body::fillet_edges hides PK_BODY_fix_blends `fault`
    // ---------------------------------------------------------------
    println!("\n== fillet_edges with an impossible radius ==");
    let blk2 = Body::create_solid_block(10.0, 10.0, 10.0).expect("blk2");
    let edges = blk2.edges().expect("edges");
    println!("  block has {} edges", edges.len());
    // radius 20 on a 10-cube: geometrically impossible
    let r = blk2.fillet_edges(&edges, 6.0);
    println!("  Body::fillet_edges(all 12 edges, radius=6) -> {r:?}   <-- kernel fault is hidden");

    // now the raw call to see what `fault` really was
    let blk3 = Body::create_solid_block(10.0, 10.0, 10.0).expect("blk3");
    let e3 = blk3.edges().expect("edges3");
    let tags: Vec<PK_EDGE_t> = e3.iter().map(|e| e.tag()).collect();
    let mut n_set: c_int = 0;
    let mut set_edges: *mut PK_EDGE_t = std::ptr::null_mut();
    let sc = unsafe {
        PK_EDGE_set_blend_constant(
            tags.len() as c_int,
            tags.as_ptr(),
            6.0,
            std::ptr::null(),
            &mut n_set,
            &mut set_edges,
        )
    };
    println!("  PK_EDGE_set_blend_constant -> {sc}, n_set={n_set}");
    let mut n_blends: c_int = 0;
    let mut blends: *mut PK_FACE_t = std::ptr::null_mut();
    let mut unders: *mut PK_FACE_array_t = std::ptr::null_mut();
    let mut topols: *mut c_int = std::ptr::null_mut();
    let mut fault: PK_blend_fault_t = -1;
    let mut fault_edge: PK_EDGE_t = PK_ENTITY_null;
    let mut fault_topol: PK_ENTITY_t = PK_ENTITY_null;
    let fc = unsafe {
        PK_BODY_fix_blends(
            blk3.tag(),
            std::ptr::null(),
            &mut n_blends,
            &mut blends,
            &mut unders,
            &mut topols,
            &mut fault,
            &mut fault_edge,
            &mut fault_topol,
        )
    };
    println!(
        "  PK_BODY_fix_blends -> err {fc}, n_blends={n_blends}, fault={fault} (18391=no_fault), fault_edge={fault_edge}, unders={unders:p}"
    );

    println!("\ndone");
}
