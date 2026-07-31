//! Twin-corpus probe campaign: PK_EDGE_optimise + PK_BODY_simplify_geom over a
//! seeded random sample of the xt-parser-validation corpus.
//!
//! Protocol: cadabra2 `docs/re/15-probe-protocol.md` (pinned session: linear
//! 1e-8 / angular 1e-11, SMP off, argument checking on). Per part:
//!
//! 1. receive → census (`orig`) → transmit bare round-trip twin (`.bare`)
//! 2. fresh receive → `PK_EDGE_optimise` on every edge (per-edge outcome
//!    recorded) → `PK_BODY_simplify_geom(local=false)` → census (`afterop`) →
//!    transmit op twin (`.op`)
//! 3. fresh receive → same ops but `simplify_geom(local=true)` → census
//!    (`afterlocal`) → transmit `.oplocal` twin
//! 4. re-receive the `.bare` twin → census (`bare_rt`); re-receive the `.op`
//!    twin → census (`op_rt`)
//!
//! Output is line-oriented TSV-ish records on stdout, analysed host-side:
//!   PART <key> <src>
//!   BODY <key> <phase> <i> type=<..> faces=<n> edges=<n> vertices=<n>
//!   EDGE <key> <phase> <body> <j> class=<..> deg=<..> rat=<..> null=<0|1> prec=<..>
//!   OPT  <key> <body> <j> null=<0|1> before=<prec> result=<mod|nomod|err:..> achieved=<..> after=<prec>
//!   SIMP <key> <local> n_geoms=<n> classes=<..>
//!   PARTFAIL <key> <stage> <error>
//!
//! Usage (cwd = campaign root containing `twin-campaign/` and `twin-corpus/`):
//!   twin_corpus.exe twin-campaign/manifest.tsv

use parasolid::*;
use parasolid_sys::PK_ENTITY_null;

/// The lane-standard pinned session (probe protocol step 3).
fn pinned_config() -> SessionConfig {
    SessionConfig::new()
        .precision(1.0e-8)
        .angle_precision(1.0e-11)
        .smp_threads(0)
        .check_arguments(true)
}

/// One edge's census row.
struct EdgeRow {
    class: String,
    degree: Option<i32>,
    rational: Option<bool>,
    null_curve: bool,
    precision: Option<f64>,
}

fn edge_census(edge: &Edge) -> EdgeRow {
    let (class, degree, rational, null_curve) = match edge.curve_tag() {
        Ok(tag) if tag == PK_ENTITY_null => ("NULL".to_string(), None, None, true),
        Ok(_) => match edge.curve() {
            Ok(curve) => match curve.curve_type() {
                Ok(ct) => {
                    let (deg, rat) = if matches!(ct, CurveType::Bcurve) {
                        match curve.ask_bcurve() {
                            Ok(d) => (Some(d.degree), Some(d.is_rational)),
                            Err(_) => (None, None),
                        }
                    } else {
                        (None, None)
                    };
                    (format!("{ct:?}"), deg, rat, false)
                }
                Err(_) => ("UNKNOWN".to_string(), None, None, false),
            },
            Err(_) => ("ERR".to_string(), None, None, false),
        },
        Err(_) => ("ERR".to_string(), None, None, true),
    };
    let precision = edge.precision().ok();
    EdgeRow {
        class,
        degree,
        rational,
        null_curve,
        precision,
    }
}

fn print_census(key: &str, phase: &str, bodies: &[Body]) {
    for (i, body) in bodies.iter().enumerate() {
        let btype = body
            .body_type()
            .map(|t| format!("{t:?}"))
            .unwrap_or_else(|e| format!("ERR({e})"));
        let faces = body.faces().map(|f| f.len()).unwrap_or(usize::MAX);
        let edges = body.edges().unwrap_or_default();
        let vertices = body.vertices().map(|v| v.len()).unwrap_or(usize::MAX);
        println!(
            "BODY\t{key}\t{phase}\t{i}\ttype={btype}\tfaces={faces}\tedges={}\tvertices={vertices}",
            edges.len()
        );
        for (j, edge) in edges.iter().enumerate() {
            let r = edge_census(edge);
            println!(
                "EDGE\t{key}\t{phase}\t{i}\t{j}\tclass={}\tdeg={}\trat={}\tnull={}\tprec={}",
                r.class,
                r.degree.map(|d| d.to_string()).unwrap_or("-".into()),
                r.rational
                    .map(|b| if b { "1" } else { "0" }.to_string())
                    .unwrap_or("-".into()),
                if r.null_curve { 1 } else { 0 },
                r.precision
                    .map(|p| format!("{p:.9e}"))
                    .unwrap_or("-".into()),
            );
        }
    }
}

/// Run PK_EDGE_optimise over every edge of every body, recording outcomes.
fn optimise_all(key: &str, bodies: &[Body]) {
    for (i, body) in bodies.iter().enumerate() {
        let Ok(edges) = body.edges() else { continue };
        for (j, edge) in edges.iter().enumerate() {
            let before = edge_census(edge);
            let outcome = edge.optimise(None, false);
            let after_prec = edge
                .precision()
                .map(|p| format!("{p:.9e}"))
                .unwrap_or("-".into());
            let result = match &outcome {
                Ok((true, _)) => "mod".to_string(),
                Ok((false, _)) => "nomod".to_string(),
                Err(e) => format!("err:{e}"),
            };
            let achieved = match &outcome {
                Ok((_, d)) => format!("{d:.9e}"),
                Err(_) => "-".into(),
            };
            println!(
                "OPT\t{key}\t{i}\t{j}\tnull={}\tbefore={}\tresult={result}\tachieved={achieved}\tafter={after_prec}",
                if before.null_curve { 1 } else { 0 },
                before
                    .precision
                    .map(|p| format!("{p:.9e}"))
                    .unwrap_or("-".into()),
            );
        }
    }
}

/// Run PK_BODY_simplify_geom on every body, recording new geometry classes.
fn simplify_all(key: &str, bodies: &[Body], local: bool) {
    for body in bodies {
        match body.simplify_geom(local) {
            Ok(geoms) => {
                let classes: Vec<String> = geoms
                    .iter()
                    .map(|g| {
                        g.class()
                            .map(|c| format!("{c:?}"))
                            .unwrap_or_else(|e| format!("ERR({e})"))
                    })
                    .collect();
                println!(
                    "SIMP\t{key}\tlocal={}\tn_geoms={}\tclasses={}",
                    if local { 1 } else { 0 },
                    geoms.len(),
                    classes.join(",")
                );
            }
            Err(e) => println!(
                "SIMP\t{key}\tlocal={}\tn_geoms=-\terr:{e}",
                if local { 1 } else { 0 }
            ),
        }
    }
}

/// One receive under a fresh pinned session; returns the session so it stays
/// alive while the bodies are used.
fn receive_fresh(stage_key: &str) -> PsResult<(Session, Vec<Body>)> {
    let session = Session::start(pinned_config())?;
    let bodies = fileio::receive(stage_key)?;
    Ok((session, bodies))
}

fn run_part(key: &str, src: &str) {
    println!("PART\t{key}\t{src}");
    let stage_key = format!("twin-campaign/staging/{key}");

    // Phase 1: bare round-trip twin.
    match receive_fresh(&stage_key) {
        Ok((session, bodies)) => {
            println!("RECV\t{key}\tn_parts={}", bodies.len());
            print_census(key, "orig", &bodies);
            if let Err(e) = fileio::transmit(&bodies, &format!("twin-corpus/{key}.bare")) {
                println!("PARTFAIL\t{key}\ttransmit_bare\t{e}");
            }
            drop(session);
        }
        Err(e) => {
            println!("PARTFAIL\t{key}\treceive_orig\t{e}");
            return;
        }
    }

    // Phase 2: optimise-all + global simplify → op twin.
    match receive_fresh(&stage_key) {
        Ok((session, bodies)) => {
            optimise_all(key, &bodies);
            simplify_all(key, &bodies, false);
            print_census(key, "afterop", &bodies);
            if let Err(e) = fileio::transmit(&bodies, &format!("twin-corpus/{key}.op")) {
                println!("PARTFAIL\t{key}\ttransmit_op\t{e}");
            }
            drop(session);
        }
        Err(e) => println!("PARTFAIL\t{key}\treceive_op\t{e}"),
    }

    // Phase 3: optimise-all + LOCAL simplify → oplocal twin (Q3 contrast).
    match receive_fresh(&stage_key) {
        Ok((session, bodies)) => {
            for body in &bodies {
                if let Ok(edges) = body.edges() {
                    for edge in edges {
                        let _ = edge.optimise(None, false);
                    }
                }
            }
            simplify_all(key, &bodies, true);
            print_census(key, "afterlocal", &bodies);
            if let Err(e) = fileio::transmit(&bodies, &format!("twin-corpus/{key}.oplocal")) {
                println!("PARTFAIL\t{key}\ttransmit_oplocal\t{e}");
            }
            drop(session);
        }
        Err(e) => println!("PARTFAIL\t{key}\treceive_oplocal\t{e}"),
    }

    // Phase 4: re-receive census of both twins.
    match receive_fresh(&format!("twin-corpus/{key}.bare")) {
        Ok((session, bodies)) => {
            print_census(key, "bare_rt", &bodies);
            drop(session);
        }
        Err(e) => println!("PARTFAIL\t{key}\trereceive_bare\t{e}"),
    }
    match receive_fresh(&format!("twin-corpus/{key}.op")) {
        Ok((session, bodies)) => {
            print_census(key, "op_rt", &bodies);
            drop(session);
        }
        Err(e) => println!("PARTFAIL\t{key}\trereceive_op\t{e}"),
    }
}

fn main() {
    let manifest_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "twin-campaign/manifest.tsv".to_string());
    let manifest = std::fs::read_to_string(&manifest_path)
        .unwrap_or_else(|e| panic!("cannot read manifest {manifest_path}: {e}"));

    println!("CAMPAIGN\ttwin-corpus\tkernel=V37.01.243\tsession=1e-8/1e-11/smp0/argcheck");
    for line in manifest.lines() {
        let mut it = line.splitn(2, '\t');
        let (Some(key), Some(src)) = (it.next(), it.next()) else {
            continue;
        };
        run_part(key, src);
    }
    println!("CAMPAIGN_DONE");
}
