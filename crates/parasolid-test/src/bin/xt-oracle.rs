//! Versioned, machine-readable Parasolid XT semantic oracle.
//!
//! This executable is intentionally separate from `xt-parser`. It receives an
//! arbitrary text XT file through the validated public PK API and emits coarse
//! semantic fingerprints without making the native parser depend on Parasolid.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

use parasolid::{BodyType, FrustrumConfig, Session, SessionConfig, fileio};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Args {
    input: PathBuf,
    dll_sha256: String,
    parasolid_rs_commit: String,
    source_state: SourceState,
    keep_staging: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
enum SourceState {
    Clean,
    Dirty,
}

#[derive(Debug, Serialize)]
struct Report {
    format_version: u32,
    input_path: PathBuf,
    input_sha256: String,
    input_bytes: u64,
    parasolid_dll_sha256: String,
    parasolid_rs_commit: String,
    parasolid_rs_source_state: SourceState,
    kernel_version: Option<KernelVersion>,
    #[serde(flatten)]
    result: OracleResult,
}

#[derive(Debug, Serialize)]
struct KernelVersion {
    major: i32,
    minor: i32,
    patch: i32,
}

#[derive(Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum OracleResult {
    Accepted { bodies: Vec<BodyFingerprint> },
    Rejected { stage: &'static str, error: String },
}

#[derive(Debug, Serialize)]
struct BodyFingerprint {
    ordinal: usize,
    body_type: String,
    topology: Topology,
    bounding_box: Box3,
    mass_properties: MassProperties,
}

#[derive(Debug, Serialize)]
struct Topology {
    regions: usize,
    solid_regions: usize,
    shells: usize,
    faces: usize,
    loops: usize,
    edges: usize,
    vertices: usize,
}

#[derive(Debug, Serialize)]
struct Box3 {
    min: Vector3,
    max: Vector3,
}

#[derive(Debug, Serialize)]
struct MassProperties {
    amount: f64,
    mass: f64,
    center_of_gravity: Vector3,
    inertia: [f64; 9],
    periphery: f64,
}

#[derive(Debug, Serialize)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(Some(args)) => args,
        Ok(None) => return ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("xt-oracle: {error}");
            return ExitCode::from(2);
        }
    };

    match run(&args) {
        Ok(report) => {
            let accepted = matches!(report.result, OracleResult::Accepted { .. });
            if let Err(error) = serde_json::to_writer_pretty(std::io::stdout(), &report) {
                eprintln!("xt-oracle: serialize report: {error}");
                return ExitCode::from(2);
            }
            println!();
            if accepted {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(3)
            }
        }
        Err(error) => {
            eprintln!("xt-oracle: {error}");
            ExitCode::from(2)
        }
    }
}

fn parse_args() -> Result<Option<Args>, String> {
    let mut input = None;
    let mut dll_sha256 = None;
    let mut parasolid_rs_commit = None;
    let mut source_state = None;
    let mut keep_staging = false;
    let mut arguments = env::args_os().skip(1);

    while let Some(argument) = arguments.next() {
        if argument == "--help" || argument == "-h" {
            println!(
                "Usage: xt-oracle INPUT --dll-sha256 HASH --parasolid-rs-commit COMMIT --source-state clean|dirty [--keep-staging]\n\
                 \n\
                 Receives one text XT file through public PK APIs and writes a JSON fingerprint.\n\
                 Exit 0: accepted; exit 3: Parasolid rejected/query failed; exit 2: tool error."
            );
            return Ok(None);
        } else if argument == "--dll-sha256" {
            dll_sha256 = Some(
                arguments
                    .next()
                    .ok_or("--dll-sha256 requires a value")?
                    .into_string()
                    .map_err(|_| "DLL hash is not valid Unicode")?,
            );
        } else if argument == "--parasolid-rs-commit" {
            parasolid_rs_commit = Some(
                arguments
                    .next()
                    .ok_or("--parasolid-rs-commit requires a value")?
                    .into_string()
                    .map_err(|_| "commit is not valid Unicode")?,
            );
        } else if argument == "--source-state" {
            let value = arguments
                .next()
                .ok_or("--source-state requires clean or dirty")?
                .into_string()
                .map_err(|_| "source state is not valid Unicode")?;
            source_state = Some(match value.as_str() {
                "clean" => SourceState::Clean,
                "dirty" => SourceState::Dirty,
                _ => return Err("--source-state must be clean or dirty".to_owned()),
            });
        } else if argument == "--keep-staging" {
            keep_staging = true;
        } else if argument.to_string_lossy().starts_with('-') {
            return Err(format!("unknown option {}", argument.to_string_lossy()));
        } else if input.replace(PathBuf::from(argument)).is_some() {
            return Err("provide exactly one input path".to_owned());
        }
    }

    let dll_sha256 = dll_sha256.ok_or("missing required --dll-sha256")?;
    if !is_sha256(&dll_sha256) {
        return Err("--dll-sha256 must be exactly 64 hexadecimal characters".to_owned());
    }
    let parasolid_rs_commit =
        parasolid_rs_commit.ok_or("missing required --parasolid-rs-commit")?;
    if !is_git_commit(&parasolid_rs_commit) {
        return Err("--parasolid-rs-commit must be a 40-character hexadecimal commit".to_owned());
    }

    Ok(Some(Args {
        input: input.ok_or("missing input path")?,
        dll_sha256: dll_sha256.to_ascii_lowercase(),
        parasolid_rs_commit: parasolid_rs_commit.to_ascii_lowercase(),
        source_state: source_state.ok_or("missing required --source-state")?,
        keep_staging,
    }))
}

fn run(args: &Args) -> Result<Report, String> {
    let input =
        fs::read(&args.input).map_err(|error| format!("read {}: {error}", args.input.display()))?;
    let input_sha256 = format!("{:x}", Sha256::digest(&input));
    let input_bytes = input.len() as u64;
    let staging = create_staging_dir()?;
    let staged_input = staging.join("input.xmt_txt");
    fs::write(&staged_input, &input)
        .map_err(|error| format!("stage {}: {error}", staged_input.display()))?;

    let mut report = Report {
        format_version: 1,
        input_path: args.input.clone(),
        input_sha256,
        input_bytes,
        parasolid_dll_sha256: args.dll_sha256.clone(),
        parasolid_rs_commit: args.parasolid_rs_commit.clone(),
        parasolid_rs_source_state: args.source_state,
        kernel_version: None,
        result: OracleResult::Rejected {
            stage: "session",
            error: "session did not start".to_owned(),
        },
    };

    let session = match Session::start(
        SessionConfig::new()
            .frustrum(FrustrumConfig::new().base_dir(&staging))
            .check_arguments(true)
            .general_topology(true),
    ) {
        Ok(session) => session,
        Err(error) => {
            report.result = OracleResult::Rejected {
                stage: "session",
                error: error.to_string(),
            };
            cleanup(&staging, args.keep_staging)?;
            return Ok(report);
        }
    };

    match session.kernel_version() {
        Ok((major, minor, patch)) => {
            report.kernel_version = Some(KernelVersion {
                major,
                minor,
                patch,
            });
        }
        Err(error) => {
            report.result = OracleResult::Rejected {
                stage: "kernel_version",
                error: error.to_string(),
            };
            drop(session);
            cleanup(&staging, args.keep_staging)?;
            return Ok(report);
        }
    }

    report.result = match fileio::receive("input") {
        Ok(bodies) => match fingerprint_bodies(&bodies) {
            Ok(bodies) => OracleResult::Accepted { bodies },
            Err(error) => OracleResult::Rejected {
                stage: "fingerprint",
                error,
            },
        },
        Err(error) => OracleResult::Rejected {
            stage: "receive",
            error: error.to_string(),
        },
    };

    drop(session);
    cleanup(&staging, args.keep_staging)?;
    Ok(report)
}

fn fingerprint_bodies(bodies: &[parasolid::Body]) -> Result<Vec<BodyFingerprint>, String> {
    bodies
        .iter()
        .enumerate()
        .map(|(ordinal, body)| {
            let body_type = body.body_type().map_err(|error| error.to_string())?;
            let topology = body.topology_summary().map_err(|error| error.to_string())?;
            let bounding_box = body.bounding_box().map_err(|error| error.to_string())?;
            let mass = body.mass_props().map_err(|error| error.to_string())?;

            Ok(BodyFingerprint {
                ordinal,
                body_type: body_type_name(body_type).to_owned(),
                topology: Topology {
                    regions: topology.regions,
                    solid_regions: topology.solid_regions,
                    shells: topology.shells,
                    faces: topology.faces,
                    loops: topology.loops,
                    edges: topology.edges,
                    vertices: topology.vertices,
                },
                bounding_box: Box3 {
                    min: vector(bounding_box.min),
                    max: vector(bounding_box.max),
                },
                mass_properties: MassProperties {
                    amount: mass.amount,
                    mass: mass.mass,
                    center_of_gravity: vector(mass.center_of_gravity),
                    inertia: mass.inertia,
                    periphery: mass.periphery,
                },
            })
        })
        .collect()
}

fn vector(value: parasolid::Vec3) -> Vector3 {
    Vector3 {
        x: value.x,
        y: value.y,
        z: value.z,
    }
}

fn body_type_name(value: BodyType) -> &'static str {
    match value {
        BodyType::Empty => "empty",
        BodyType::Acorn => "acorn",
        BodyType::Wire => "wire",
        BodyType::Sheet => "sheet",
        BodyType::Solid => "solid",
        BodyType::General => "general",
    }
}

fn create_staging_dir() -> Result<PathBuf, String> {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("system clock: {error}"))?
        .as_nanos();
    let path = env::temp_dir().join(format!("xt-oracle-{}-{nonce}", std::process::id()));
    fs::create_dir(&path).map_err(|error| format!("create {}: {error}", path.display()))?;
    Ok(path)
}

fn cleanup(path: &Path, keep: bool) -> Result<(), String> {
    if keep {
        eprintln!("xt-oracle: kept staging directory {}", path.display());
        return Ok(());
    }
    fs::remove_dir_all(path).map_err(|error| format!("remove {}: {error}", path.display()))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_git_commit(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
