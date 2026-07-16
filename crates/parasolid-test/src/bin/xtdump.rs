// XT persistence-format oracle: emit known bodies in text + binary to separate keys.
use parasolid::*;
use parasolid_sys::*;
use std::ffi::CString;
use std::os::raw::c_int;

fn xmit(tag: i32, key: &str, fmt: PK_transmit_format_t) {
    let key_c = CString::new(key).unwrap();
    let mut opts = PK_PART_transmit_o_t::default();
    opts.transmit_format = fmt;
    let parts = [tag];
    let e = unsafe {
        PK_PART_transmit(parts.len() as c_int, parts.as_ptr(), key_c.as_ptr(), &opts)
    };
    println!("transmit key={key} fmt={fmt} -> err={e}");
}

fn dump(b: &Body, name: &str) {
    let f = b.faces().map(|v| v.len()).unwrap_or(0);
    let e = b.edges().map(|v| v.len()).unwrap_or(0);
    let v = b.vertices().map(|v| v.len()).unwrap_or(0);
    let vol = b.volume().unwrap_or(f64::NAN);
    println!("{name}: faces={f} edges={e} vertices={v} vol={vol}");
}

fn emit(b: &Body, name: &str) {
    dump(b, name);
    xmit(b.tag(), name, PK_transmit_format_text_c);
    xmit(b.tag(), name, PK_transmit_format_binary_c);
}

fn main() {
    let cfg = SessionConfig::new().check_arguments(false).frustrum(
        FrustrumConfig::new().base_dir(
            "/tmp/claude-1000/-home-dev-projects-parasolid-re/ccf7881f-5d90-471c-9b56-208ecdb81733/scratchpad/xt",
        ),
    );
    let _s = Session::start(cfg).unwrap();

    // 1) block 10x20x30 -> half-widths 5/10/15, 6 faces/12 edges/8 vertices
    let block = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    emit(&block, "block");

    // 2) unit cube 2x2x2 -> half-widths 1/1/1
    let cube = Body::create_solid_block(2.0, 2.0, 2.0).unwrap();
    emit(&cube, "cube");

    // 3) solid sphere r=5 -> 1 spherical face
    let sphere = Body::create_solid_sphere(5.0).unwrap();
    emit(&sphere, "sphere");

    // 4) solid cylinder r=5 h=10
    let cyl = Body::create_solid_cylinder(5.0, 10.0).unwrap();
    emit(&cyl, "cyl");

    // 5) solid cone r=5 h=10 semi_angle derived
    match Body::create_solid_cone(5.0, 10.0, 0.4636476) {
        Ok(cone) => emit(&cone, "cone"),
        Err(e) => println!("cone err: {e}"),
    }

    // 6) torus major=10 minor=3
    match Body::create_solid_torus(10.0, 3.0) {
        Ok(t) => emit(&t, "torus"),
        Err(e) => println!("torus err: {e}"),
    }

    println!("DONE");
}
