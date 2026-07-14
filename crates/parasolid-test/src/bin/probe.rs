//! Empirical token probe for pskernel.dll.
//!
//! The Parasolid header docs (see solidworks-notes/headers mirror) give token
//! NAMES but not numeric VALUES. This probe recovers the values from the
//! actual DLL. Values already confirmed against pskernel.dll V37.01.243
//! (SOLIDWORKS 2025) are hardcoded in parasolid-sys (marked [probed]).
//!
//! Run under wine:
//!   cargo build -p parasolid-test --target x86_64-pc-windows-gnu
//!   (cd target dir; copy pskernel.dll next to exe) wine probe.exe

use parasolid::*;
use parasolid_sys::*;

// Correct-by-docs signatures for probes (deprecated/simple variants).
unsafe extern "C" {
    #[link_name = "PK_SURF_make_sheet_body"]
    fn surf_make_sheet_body(surf: i32, uv_box: PK_UVBOX_t, body: *mut i32) -> i32;
    #[link_name = "PK_CURVE_make_wire_body"]
    fn curve_make_wire_body(curve: i32, range: PK_INTERVAL_t, body: *mut i32) -> i32;
    #[link_name = "PK_POINT_make_minimum_body"]
    fn point_make_minimum_body(point: i32, body: *mut i32) -> i32;
}

fn class_of(tag: i32) -> i32 {
    let mut c: PK_CLASS_t = -1;
    unsafe { PK_ENTITY_ask_class(tag, &mut c) };
    c
}

fn body_type(tag: i32) -> i32 {
    let mut t: PK_BODY_type_t = -1;
    unsafe { PK_BODY_ask_type(tag, &mut t) };
    t
}

fn main() {
    let _session = Session::start(SessionConfig::new()).expect("session");

    println!("== full class hierarchy (class -> superclass) ==");
    for c in 1..8000 {
        let mut sup: PK_CLASS_t = -1;
        if unsafe { PK_CLASS_ask_superclass(c, &mut sup) } == 0 {
            println!("class {c} -> {sup}");
        }
    }

    println!("== anchors ==");
    let block = Body::create_solid_block(10.0, 20.0, 30.0).expect("block");
    println!("BODY_type solid   = {}", body_type(block.tag()));
    println!("class body        = {}", class_of(block.tag()));
    let face_tag = block.faces().unwrap()[0].tag();
    println!("class face        = {}", class_of(face_tag));
    let mut surf: PK_SURF_t = 0;
    unsafe { PK_FACE_ask_surf(face_tag, &mut surf) };
    println!("class plane       = {}", class_of(surf));
    let edge_tag = block.edges().unwrap()[0].tag();
    println!("class edge        = {}", class_of(edge_tag));
    let mut curve: PK_CURVE_t = 0;
    unsafe { PK_EDGE_ask_curve(edge_tag, &mut curve) };
    println!("class line        = {}", class_of(curve));
    let vert_tag = block.vertices().unwrap()[0].tag();
    println!("class vertex      = {}", class_of(vert_tag));
    let mut point: PK_POINT_t = 0;
    unsafe { PK_VERTEX_ask_point(vert_tag, &mut point) };
    println!("class point       = {}", class_of(point));

    let mut loops = std::ptr::null_mut();
    let mut n = 0;
    unsafe { PK_FACE_ask_loops(face_tag, &mut n, &mut loops) };
    if n > 0 {
        let l = unsafe { *loops };
        println!("class loop        = {}", class_of(l));
        let mut fins = std::ptr::null_mut();
        let mut nf = 0;
        unsafe { PK_LOOP_ask_fins(l, &mut nf, &mut fins) };
        if nf > 0 {
            println!("class fin         = {}", class_of(unsafe { *fins }));
        }
    }
    let (mut shells, mut ns) = (std::ptr::null_mut(), 0);
    unsafe { PK_BODY_ask_shells(block.tag(), &mut ns, &mut shells) };
    if ns > 0 {
        println!("class shell       = {}", class_of(unsafe { *shells }));
    }
    let (mut regions, mut nr) = (std::ptr::null_mut(), 0);
    unsafe { PK_BODY_ask_regions(block.tag(), &mut nr, &mut regions) };
    if nr > 0 {
        println!("class region      = {}", class_of(unsafe { *regions }));
    }

    let cyl = Body::create_solid_cylinder(5.0, 20.0).expect("cyl");
    for f in cyl.faces().unwrap() {
        let mut s: PK_SURF_t = 0;
        unsafe { PK_FACE_ask_surf(f.tag(), &mut s) };
        println!("class cyl-surf    = {}", class_of(s));
    }
    for e in cyl.edges().unwrap() {
        let mut c: PK_CURVE_t = 0;
        unsafe { PK_EDGE_ask_curve(e.tag(), &mut c) };
        println!("class circle      = {}", class_of(c));
    }
    let sph = Body::create_solid_sphere(25.0).expect("sphere");
    let mut s: PK_SURF_t = 0;
    unsafe { PK_FACE_ask_surf(sph.faces().unwrap()[0].tag(), &mut s) };
    println!("class sphere      = {}", class_of(s));
    let tor = Body::create_solid_torus(20.0, 5.0).expect("torus");
    let mut s: PK_SURF_t = 0;
    unsafe { PK_FACE_ask_surf(tor.faces().unwrap()[0].tag(), &mut s) };
    println!("class torus       = {}", class_of(s));

    // --- Body types beyond solid ---
    let uvbox = PK_UVBOX_t { param: [0.0, 0.0, 5.0, 5.0] };
    let mut sheet = 0;
    unsafe { PK_FACE_ask_surf(block.faces().unwrap()[0].tag(), &mut surf) };
    if unsafe { surf_make_sheet_body(surf, uvbox, &mut sheet) } == 0 {
        println!("BODY_type sheet   = {}", body_type(sheet));
    }
    let mut wire = 0;
    let range = PK_INTERVAL_t { low: 0.0, high: 5.0 };
    if unsafe { curve_make_wire_body(curve, range, &mut wire) } == 0 {
        println!("BODY_type wire    = {}", body_type(wire));
    }
    let sf = PK_POINT_sf_t { position: [1.0, 2.0, 3.0] };
    let mut pt: PK_POINT_t = 0;
    unsafe { PK_POINT_create(&sf, &mut pt) };
    let mut min_body = 0;
    if unsafe { point_make_minimum_body(pt, &mut min_body) } == 0 {
        println!("BODY_type minimum = {}", body_type(min_body));
    }

    println!("done");
}
