//! DATA_MODEL_DEEP probe: tag system (monotonic integer identity) + rollback
//! (partition delta replay) — grounds the DS__log / DS_roll_mark / partition RE.

use parasolid::*;
use parasolid_sys::*;

unsafe extern "C" {
    fn PK_MARK_create(mark: *mut i32) -> i32;
    fn PK_MARK_goto(mark: i32) -> i32;
}

fn class_of(tag: i32) -> (i32, i32) {
    let mut c: PK_CLASS_t = -1;
    let e = unsafe { PK_ENTITY_ask_class(tag, &mut c) };
    (e, c)
}
fn ident_of(tag: i32) -> (i32, i32) {
    let mut id: i32 = -1;
    let e = unsafe { PK_ENTITY_ask_identifier(tag, &mut id) };
    (e, id)
}

fn main() {
    let cfg = SessionConfig::new().check_arguments(false).frustrum(
        FrustrumConfig::new().base_dir("/tmp/claude-1000/-home-dev-projects-parasolid-re/ccf7881f-5d90-471c-9b56-208ecdb81733/scratchpad/xt"),
    );
    let _s = Session::start(cfg).expect("session");

    // 1. Tag allocation is monotonic integer identity.
    let a = Body::create_solid_block(10.0, 10.0, 10.0).expect("A");
    let at = a.tag();
    let faces = a.faces().unwrap();
    let ft: Vec<i32> = faces.iter().map(|f| f.tag()).collect();
    println!("A body tag = {at}  ident = {:?}", ident_of(at));
    println!("A face tags = {ft:?}  (monotonic, contiguous => index-allocated)");
    println!("A body class = {:?}", class_of(at));

    // 2. Set a mark (PK_MARK_create -> pmarks in every partition).
    let mut mark: i32 = 0;
    let mc = unsafe { PK_MARK_create(&mut mark) };
    println!("PK_MARK_create rc = {mc}  mark tag = {mark}");

    // 3. Create body B after the mark.
    let b = Body::create_solid_block(5.0, 5.0, 5.0).expect("B");
    let bt = b.tag();
    println!("B body tag = {bt}  (> A's tags => counter advanced)");
    println!("B valid before roll: class = {:?}", class_of(bt));

    // 4. Roll back to the mark: partition delta replay should destroy B and
    //    keep A (copy-on-write journal restore).
    let gr = unsafe { PK_MARK_goto(mark) };
    println!("PK_MARK_goto rc = {gr}");
    println!("A valid after roll: class = {:?} (expect ok, 0)", class_of(at));
    println!("B valid after roll: class = {:?} (expect error => B undone)", class_of(bt));

    // 5. Create C after rollback: does the tag counter rewind (tag reuse)?
    std::mem::forget(b); // B is dead now; don't let Drop double-free
    let c = Body::create_solid_block(3.0, 3.0, 3.0).expect("C");
    println!("C body tag = {}  (== B's {bt}? => tag counter rewound on roll)", c.tag());
    std::mem::forget(c);
    std::mem::forget(a);
}
