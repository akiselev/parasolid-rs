//! Batch public-PK oracle for producing matched XT binary fixtures from text.

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use parasolid::{
    FrustrumConfig, Session, SessionConfig,
    fileio::{self, TransmitFormat},
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Args {
    input: PathBuf,
    output: PathBuf,
    limit: Option<usize>,
    report: Option<PathBuf>,
}

#[derive(Serialize)]
struct Report {
    format_version: u32,
    input_root: PathBuf,
    output_root: PathBuf,
    files_discovered: usize,
    files_selected: usize,
    files_reencoded: usize,
    binary_files_written: usize,
    fixtures: Vec<Fixture>,
    errors: Vec<FixtureError>,
}

#[derive(Serialize)]
struct Fixture {
    input: PathBuf,
    input_sha256: String,
    input_bytes: u64,
    body_count: usize,
    outputs: Vec<Output>,
}

#[derive(Serialize)]
struct Output {
    format: &'static str,
    path: PathBuf,
    sha256: String,
    bytes: u64,
}

#[derive(Serialize)]
struct FixtureError {
    input: PathBuf,
    stage: &'static str,
    error: String,
}

fn main() -> ExitCode {
    match parse_args().and_then(run) {
        Ok(report) => {
            let success = report.errors.is_empty();
            if let Err(error) = serde_json::to_writer_pretty(std::io::stdout(), &report) {
                eprintln!("xt-reencode: serialize report: {error}");
                return ExitCode::from(2);
            }
            println!();
            if success {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(3)
            }
        }
        Err(error) => {
            eprintln!("xt-reencode: {error}");
            ExitCode::from(2)
        }
    }
}

fn parse_args() -> Result<Args, String> {
    let mut input = None;
    let mut output = None;
    let mut limit = None;
    let mut report = None;
    let mut arguments = env::args_os().skip(1);
    while let Some(argument) = arguments.next() {
        if argument == "--out" {
            output = Some(PathBuf::from(
                arguments.next().ok_or("--out requires a directory")?,
            ));
        } else if argument == "--report" {
            report = Some(PathBuf::from(
                arguments.next().ok_or("--report requires a path")?,
            ));
        } else if argument == "--limit" {
            let value = arguments
                .next()
                .ok_or("--limit requires a positive integer")?
                .into_string()
                .map_err(|_| "--limit is not valid Unicode")?;
            let value = value
                .parse::<usize>()
                .map_err(|_| "--limit must be a positive integer")?;
            if value == 0 {
                return Err("--limit must be positive".to_owned());
            }
            limit = Some(value);
        } else if argument.to_string_lossy().starts_with('-') {
            return Err(format!("unknown option {}", argument.to_string_lossy()));
        } else if input.replace(PathBuf::from(argument)).is_some() {
            return Err("provide exactly one input file or directory".to_owned());
        }
    }
    Ok(Args {
        input: input.ok_or("missing input file or directory")?,
        output: output.ok_or("missing required --out directory")?,
        limit,
        report,
    })
}

fn run(args: Args) -> Result<Report, String> {
    let mut discovered = Vec::new();
    collect_paths(&args.input, &mut discovered)?;
    discovered.sort();
    discovered.dedup();
    let selected = select_evenly(&discovered, args.limit);
    fs::create_dir_all(&args.output)
        .map_err(|error| format!("create {}: {error}", args.output.display()))?;

    let _session = Session::start(
        SessionConfig::new()
            .frustrum(FrustrumConfig::new().base_dir(&args.output))
            .check_arguments(true)
            .general_topology(true),
    )
    .map_err(|error| error.to_string())?;

    let mut report = Report {
        format_version: 1,
        input_root: args.input,
        output_root: args.output.clone(),
        files_discovered: discovered.len(),
        files_selected: selected.len(),
        files_reencoded: 0,
        binary_files_written: 0,
        fixtures: Vec::new(),
        errors: Vec::new(),
    };

    for (ordinal, input_path) in selected.into_iter().enumerate() {
        match reencode_one(&args.output, input_path, ordinal) {
            Ok(fixture) => {
                report.files_reencoded += 1;
                report.binary_files_written += fixture.outputs.len();
                report.fixtures.push(fixture);
            }
            Err(error) => report.errors.push(error),
        }
    }
    if let Some(path) = args.report {
        let json = serde_json::to_vec_pretty(&report).map_err(|error| error.to_string())?;
        fs::write(&path, json).map_err(|error| format!("write {}: {error}", path.display()))?;
    }
    Ok(report)
}

fn reencode_one(
    output_root: &Path,
    input_path: &Path,
    ordinal: usize,
) -> Result<Fixture, FixtureError> {
    let input = fs::read(input_path).map_err(|error| FixtureError {
        input: input_path.to_owned(),
        stage: "read",
        error: error.to_string(),
    })?;
    let input_sha256 = format!("{:x}", Sha256::digest(&input));
    let identity = format!("{ordinal:04}_{}", &input_sha256[..16]);
    let input_key = format!("input_{identity}");
    let staged_path = output_root.join(format!("{input_key}.xmt_txt"));
    fs::write(&staged_path, &input).map_err(|error| FixtureError {
        input: input_path.to_owned(),
        stage: "stage",
        error: error.to_string(),
    })?;

    let bodies = fileio::receive(&input_key).map_err(|error| FixtureError {
        input: input_path.to_owned(),
        stage: "receive",
        error: error.to_string(),
    })?;
    let formats = [
        ("bare", TransmitFormat::BareBinary),
        ("neutral", TransmitFormat::NeutralBinary),
        ("typed", TransmitFormat::TypedBinary),
    ];
    let mut outputs = Vec::new();
    for (name, format) in formats {
        let key = format!("{identity}_{name}");
        fileio::transmit_with_format(&bodies, &key, format).map_err(|error| FixtureError {
            input: input_path.to_owned(),
            stage: "transmit",
            error: format!("{name}: {error}"),
        })?;
        let path = output_root.join(format!("{key}.xmt_bin"));
        let bytes = fs::read(&path).map_err(|error| FixtureError {
            input: input_path.to_owned(),
            stage: "read_output",
            error: format!("{}: {error}", path.display()),
        })?;
        outputs.push(Output {
            format: name,
            path,
            sha256: format!("{:x}", Sha256::digest(&bytes)),
            bytes: bytes.len() as u64,
        });
    }
    fs::remove_file(&staged_path).map_err(|error| FixtureError {
        input: input_path.to_owned(),
        stage: "cleanup",
        error: error.to_string(),
    })?;
    Ok(Fixture {
        input: input_path.to_owned(),
        input_sha256,
        input_bytes: input.len() as u64,
        body_count: bodies.len(),
        outputs,
    })
}

fn collect_paths(path: &Path, output: &mut Vec<PathBuf>) -> Result<(), String> {
    if path.is_file() {
        if is_text_xt(path) {
            output.push(path.to_owned());
        }
        return Ok(());
    }
    if !path.is_dir() {
        return Err(format!("{} is not a file or directory", path.display()));
    }
    for entry in fs::read_dir(path).map_err(|error| format!("{}: {error}", path.display()))? {
        let child = entry
            .map_err(|error| format!("{}: {error}", path.display()))?
            .path();
        if child.is_dir() {
            collect_paths(&child, output)?;
        } else if is_text_xt(&child) {
            output.push(child);
        }
    }
    Ok(())
}

fn is_text_xt(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("x_t") || extension.eq_ignore_ascii_case("xmt_txt")
        })
}

fn select_evenly<'a>(paths: &'a [PathBuf], limit: Option<usize>) -> Vec<&'a Path> {
    let Some(limit) = limit.filter(|limit| *limit < paths.len()) else {
        return paths.iter().map(PathBuf::as_path).collect();
    };
    if limit == 1 {
        return vec![paths[0].as_path()];
    }
    (0..limit)
        .map(|position| {
            let index = position * (paths.len() - 1) / (limit - 1);
            paths[index].as_path()
        })
        .collect()
}
