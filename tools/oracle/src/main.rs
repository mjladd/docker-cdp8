// This file is part of a Rust reimplementation of the CDP System, a
// Composers Desktop Project (CDP) command-line sound-processing suite
// originally written by Trevor Wishart, Richard Dobson, Martin Atkins
// and others (see legacy/dev for the original C source and its
// per-file copyright notices).
//
// SPDX-License-Identifier: LGPL-2.1-or-later
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 2.1 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

//! `cdp-oracle`: the golden-test harness (gate 3 of
//! `docs/migration/PLAN-V2.md`).
//!
//! ```text
//! cdp-oracle record <case.toml>...     run legacy, write the expectation
//! cdp-oracle verify [<program> [<sub>]] replay every matching case
//! cdp-oracle list                      show every case and its state
//! ```
//!
//! Recording needs Docker, because the expectation comes from the legacy
//! image. Verifying does not: it replays the committed expectation, so
//! the fast continuous-integration job can run it.
//!
//! The split matters. Gate 1 and gate 2 check that a command declares the
//! right argument grammar and prints the right usage text. Neither one
//! checks what the command actually does. This gate does.

mod case;
mod run;

use case::Case;
use run::{Sandbox, run_legacy, run_rust};
use std::path::{Path, PathBuf};

const DEFAULT_IMAGE: &str = "cdp8-postmerge";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("record") => record(&args[1..]),
        Some("verify") => verify(&args[1..]),
        Some("list") => list(),
        _ => {
            eprintln!("{}", usage());
            std::process::exit(2);
        }
    };
    if let Err(message) = result {
        eprintln!("cdp-oracle: {message}");
        std::process::exit(1);
    }
}

fn usage() -> String {
    "cdp-oracle: golden-test harness for the CDP System port\n\n\
     USAGE:\n  \
       cdp-oracle record <case.toml>...      run legacy and write the expectation (needs Docker)\n  \
       cdp-oracle verify [program [sub]]     replay cases against the Rust build\n  \
       cdp-oracle list                       list every case and whether it is recorded\n\n\
     The legacy image defaults to \"cdp8-postmerge\"; set CDP_ORACLE_IMAGE to change it.\n\
     The Rust binaries default to target/debug; set CDP_ORACLE_BIN_DIR to change it.\n"
        .to_string()
}

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is tools/oracle.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repository root above tools/oracle")
        .to_path_buf()
}

fn image() -> String {
    std::env::var("CDP_ORACLE_IMAGE").unwrap_or_else(|_| DEFAULT_IMAGE.to_string())
}

fn bin_dir() -> PathBuf {
    std::env::var("CDP_ORACLE_BIN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| repo_root().join("target/debug"))
}

/// Every case file under `spec/golden`, sorted, optionally filtered by
/// program and sub-command.
fn all_cases(program: Option<&str>, subcommand: Option<&str>) -> Result<Vec<PathBuf>, String> {
    let root = repo_root().join("spec/golden");
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut found = Vec::new();
    let mut stack = vec![root];
    while let Some(dir) = stack.pop() {
        let entries =
            std::fs::read_dir(&dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
        for entry in entries {
            let path = entry
                .map_err(|e| format!("cannot read an entry of {}: {e}", dir.display()))?
                .path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "toml") {
                found.push(path);
            }
        }
    }
    found.sort();

    if program.is_none() {
        return Ok(found);
    }
    let mut kept = Vec::new();
    for path in found {
        let case = Case::load(&path)?;
        let program_matches = program.is_none_or(|p| case.program == p);
        let sub_matches = subcommand.is_none_or(|s| case.subcommand == s);
        if program_matches && sub_matches {
            kept.push(path);
        }
    }
    Ok(kept)
}

fn record(paths: &[String]) -> Result<(), String> {
    if paths.is_empty() {
        return Err("record needs at least one case file".to_string());
    }
    let root = repo_root();
    let image = image();

    for raw in paths {
        let path = PathBuf::from(raw);
        let mut case = Case::load(&path)?;
        case.registry_entry()?;

        let sandbox = Sandbox::new(&format!("{}-{}", case.program, case.subcommand))?;
        sandbox.populate(&case, &root)?;
        let observed = run_legacy(&case, &sandbox, &image)?;

        let changed = case.expected.as_ref() != Some(&observed);
        case.expected = Some(observed.clone());
        case.save(&path)?;

        let state = if changed { "recorded" } else { "unchanged" };
        println!(
            "{state}: {} ({}/{}) exit {}",
            path.display(),
            case.program,
            case.subcommand,
            observed.exit_code
        );
    }
    Ok(())
}

fn verify(filter: &[String]) -> Result<(), String> {
    let program = filter.first().map(String::as_str);
    let subcommand = filter.get(1).map(String::as_str);
    let cases = all_cases(program, subcommand)?;

    if cases.is_empty() {
        return Err(format!(
            "no cases found under spec/golden{}. Add one, then record it.",
            match (program, subcommand) {
                (Some(p), Some(s)) => format!(" for {p}/{s}"),
                (Some(p), None) => format!(" for {p}"),
                _ => String::new(),
            }
        ));
    }

    let root = repo_root();
    let bin_dir = bin_dir();
    let mut passed = 0usize;
    let mut failures = Vec::new();
    let mut unrecorded = Vec::new();
    let mut known = Vec::new();
    let mut stale_markers = Vec::new();

    for path in &cases {
        let case = Case::load(path)?;
        let name = format!("{}/{} [{}]", case.program, case.subcommand, stem(path));

        let Some(expected) = case.expected.clone() else {
            unrecorded.push(format!(
                "{name}: no expectation recorded. Run: cdp-oracle record {}",
                path.display()
            ));
            continue;
        };

        let sandbox = Sandbox::new(&format!("{}-{}", case.program, case.subcommand))?;
        sandbox.populate(&case, &root)?;
        let observed = run_rust(&case, &sandbox, &bin_dir)?;

        let mut differences = Vec::new();
        if observed.exit_code != expected.exit_code {
            differences.push(format!(
                "  exit code: got {}, legacy gave {}",
                observed.exit_code, expected.exit_code
            ));
        }
        if observed.stdout != expected.stdout {
            differences.push(format!(
                "  standard output:\n{}",
                indent(&diff_lines(&observed.stdout, &expected.stdout))
            ));
        }
        if observed.stderr != expected.stderr {
            differences.push(format!(
                "  standard error:\n{}",
                indent(&diff_lines(&observed.stderr, &expected.stderr))
            ));
        }

        match (differences.is_empty(), case.known_deviation.as_deref()) {
            (true, None) => passed += 1,
            (true, Some(why)) => stale_markers.push(format!(
                "{name}: now matches legacy, but is still marked known_deviation = {why:?}\n  \
                 Remove the marker from {}.",
                path.display()
            )),
            (false, Some(why)) => known.push(format!("{name}: {why}")),
            (false, None) => failures.push(format!("{name}\n{}", differences.join("\n"))),
        }
    }

    for note in &unrecorded {
        println!("SKIP  {note}");
    }
    for note in &known {
        println!("KNOWN {note}");
    }
    for failure in &failures {
        println!("FAIL  {failure}");
    }
    for stale in &stale_markers {
        println!("STALE {stale}");
    }
    println!(
        "\n{passed} passed, {} failed, {} known deviation(s), {} unrecorded, \
         {} stale marker(s), {} case(s) total",
        failures.len(),
        known.len(),
        unrecorded.len(),
        stale_markers.len(),
        cases.len()
    );

    match (failures.len(), stale_markers.len()) {
        (0, 0) => Ok(()),
        (0, n) => Err(format!(
            "{n} case(s) carry a known_deviation marker that is no longer true"
        )),
        (n, 0) => Err(format!("{n} case(s) do not match legacy")),
        (n, m) => Err(format!(
            "{n} case(s) do not match legacy and {m} carry a stale known_deviation marker"
        )),
    }
}

fn list() -> Result<(), String> {
    let cases = all_cases(None, None)?;
    if cases.is_empty() {
        println!("no cases under spec/golden");
        return Ok(());
    }
    for path in &cases {
        let case = Case::load(path)?;
        let state = if case.expected.is_some() {
            "recorded"
        } else {
            "NOT recorded"
        };
        println!(
            "{:<12} {}/{} [{}]  {}",
            state,
            case.program,
            case.subcommand,
            stem(path),
            case.description
        );
    }
    println!("\n{} case(s)", cases.len());
    Ok(())
}

fn stem(path: &Path) -> String {
    path.file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Reports the first differing line, then the shape of the rest, so a
/// failure names the defect instead of printing two whole outputs.
fn diff_lines(actual: &str, expected: &str) -> String {
    let (a, e): (Vec<_>, Vec<_>) = (actual.lines().collect(), expected.lines().collect());
    for i in 0..a.len().max(e.len()) {
        let got = a.get(i);
        let want = e.get(i);
        if got != want {
            return format!(
                "first difference at line {}:\n  actual  : {:?}\n  expected: {:?}\n\
                 ({} line(s) produced, {} expected)",
                i + 1,
                got.unwrap_or(&"<end of output>"),
                want.unwrap_or(&"<end of expectation>"),
                a.len(),
                e.len()
            );
        }
    }
    format!(
        "lines match; trailing bytes differ ({} bytes produced, {} expected)",
        actual.len(),
        expected.len()
    )
}

fn indent(text: &str) -> String {
    text.lines()
        .map(|l| format!("    {l}"))
        .collect::<Vec<_>>()
        .join("\n")
}
