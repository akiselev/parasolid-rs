//! Dynamic probe: which behaviour value flips the PK_BODY_contains_vector "on"
//! band observed by behaviour_probe (face +X of a 10x20x30 block, outward offsets).
//! Sweeps PK_SESSION_set_behaviour(value) over MMmmpp candidates 280100..370115 and
//! reports the value actually set plus the enclosure code at each offset.

use parasolid::*;
use parasolid_sys as sys;

fn main() {
    let base_dir = "/tmp/claude-1000/-home-dev-projects-parasolid-re/82611b04-ad9e-4210-a557-7db62fa778dd/scratchpad/dynamic-session/xt";
    let _s = Session::start(
        SessionConfig::new()
            .check_arguments(true)
            .frustrum(FrustrumConfig::new().base_dir(base_dir)),
    )
    .unwrap();
    let blk = Body::create_solid_block(10.0, 20.0, 30.0).unwrap();
    let sph = Body::create_solid_sphere(10.0).unwrap();
    let offs = [3e-9, 4e-9, 4.5e-9, 5e-9, 7e-9, 9e-9, -9e-9, -1e-8, -1.1e-8];
    println!(
        "columns: requested set_value status | block face+X offsets {:?} | sphere radial offsets same",
        offs
    );
    let mut last = String::new();
    for major in 28..=37 {
        for minor in 0..=1 {
            for patch in 0..=15 {
                let v = major * 10000 + minor * 100 + patch;
                if v < 280100 {
                    continue;
                }
                let req = sys::PK_SESSION_behaviour_t {
                    behaviour_type: 25842,
                    behaviour_value: v,
                };
                let mut s = sys::PK_SESSION_behaviour_t {
                    behaviour_type: -1,
                    behaviour_value: -1,
                };
                let mut p = s;
                let mut st = -1;
                let rc = unsafe {
                    sys::PK_SESSION_set_behaviour(req, std::ptr::null(), &mut s, &mut p, &mut st)
                };
                let mut codes = Vec::new();
                for (b, base, dir) in [
                    (blk.tag(), [5.0, 1.3, 15.7], [1.0, 0.0, 0.0]),
                    (sph.tag(), [6.0, 0.0, 8.0], [0.6, 0.0, 0.8]),
                ] {
                    for k in offs {
                        let pt = [
                            base[0] + k * dir[0],
                            base[1] + k * dir[1],
                            base[2] + k * dir[2],
                        ];
                        let mut enc = 0;
                        let mut t = 0;
                        let r = unsafe { sys::PK_BODY_contains_vector(b, &pt, &mut enc, &mut t) };
                        codes.push(if r != 0 {
                            format!("E{r}")
                        } else {
                            match enc {
                                5701 => "I".into(),
                                5702 => "O".into(),
                                5703 => "N".into(),
                                x => format!("{x}"),
                            }
                        });
                    }
                    codes.push("|".into());
                }
                let c = codes.join("");
                let mark = if c != last { "  <== change" } else { "" };
                println!(
                    "{v} rc={rc} set={} status={st} {c}{mark}",
                    s.behaviour_value
                );
                last = c;
            }
        }
    }
    println!("done");
}
