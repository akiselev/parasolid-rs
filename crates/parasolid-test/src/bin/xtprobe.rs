use parasolid::*;
fn main(){
    let cfg=SessionConfig::new().check_arguments(false).frustrum(FrustrumConfig::new().base_dir("/tmp/claude-1000/-home-dev-projects-parasolid-rs/938160ef-4201-4a69-8225-894cc231f0fe/scratchpad/xt"));
    let _s=Session::start(cfg).unwrap();
    let b=Body::create_solid_block(10.0,20.0,30.0).unwrap();
    println!("orig vol={} faces={} edges={}", b.volume().unwrap(), b.faces().unwrap().len(), b.edges().unwrap().len());
    match parasolid::fileio::transmit(&[b],"rt"){ Ok(())=>println!("transmit OK"), Err(e)=>{println!("transmit err: {e}");return;} }
    match parasolid::fileio::receive("rt"){
        Ok(bs)=>{ println!("received {} bodies", bs.len()); for rb in &bs { println!("  vol={} faces={} edges={} type={:?}", rb.volume().unwrap(), rb.faces().unwrap().len(), rb.edges().unwrap().len(), rb.body_type().unwrap()); } }
        Err(e)=>println!("receive err: {e}"),
    }
}
