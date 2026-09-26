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

//! Gate 1 of `docs/migration/PLAN-V2.md`: usage-text conformance.
//!
//! For every sub-command in [`cdp_programs::registry::COMMANDS`], this
//! test runs the bare sub-command and compares the result against the
//! usage text captured from a real legacy run in
//! `spec/usage/<program>/<subcommand>.txt`.
//!
//! The gate exists because six sub-commands reached the main branch
//! carrying invented usage text (see PLAN-V2 section 3). Every one of
//! them declared an argument grammar that the legacy program rejects.
//! The captured text was already in the repository in each case.
//!
//! Three properties are checked, all confirmed against the legacy
//! image `cdp8-postmerge` for `sndinfo props` and `housekeep copy`:
//! usage text goes to standard output, standard error stays empty, and
//! the exit code is 255 (legacy's `FAILED` of -1, seen through a shell).

use cdp_programs::registry::{COMMANDS, CommandEntry, usage_spec_path};
use std::path::{Path, PathBuf};
use std::process::Command;

/// legacy: `report_and_exit` returns `FAILED` (-1) for every category,
/// including `USAGE_ONLY`, which a shell reports as 255.
const USAGE_EXIT_CODE: i32 = 255;

fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR is crates/cdp-cli.
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repository root above crates/cdp-cli")
        .to_path_buf()
}

/// The binary that runs `entry`, plus the arguments that reach its bare
/// sub-command. A sub-command with a launcher binary is invoked through
/// it (`sndinfo props`). One without is invoked through `cdp`
/// (`cdp pvoc anal`).
fn invocation(entry: &CommandEntry) -> (PathBuf, Vec<&str>) {
    match entry.launcher {
        Some("sndinfo") => (env!("CARGO_BIN_EXE_sndinfo").into(), vec![entry.subcommand]),
        Some("housekeep") => (
            env!("CARGO_BIN_EXE_housekeep").into(),
            vec![entry.subcommand],
        ),
        Some("synth") => (env!("CARGO_BIN_EXE_synth").into(), vec![entry.subcommand]),
        Some(other) => panic!(
            "registry names launcher {other:?} for {}/{}, but cdp-cli builds no such binary",
            entry.program, entry.subcommand
        ),
        None => (
            env!("CARGO_BIN_EXE_cdp").into(),
            vec![entry.program, entry.subcommand],
        ),
    }
}

/// Sub-commands that are known to fail this gate right now, with the
/// reason. Every one of them comes from the autonomous run described in
/// `docs/migration/PLAN-V2.md` section 3, and every one is scheduled
/// for repair or withdrawal in that plan's phase R.
///
/// This baseline keeps the main branch green while phase R runs. It is
/// deliberately tight in both directions. A sub-command that is not
/// listed and fails is a new regression and fails the test. A listed
/// sub-command that starts passing also fails the test, so that the
/// entry cannot outlive the defect it records.
const KNOWN_FAILURES: &[(&str, &str)] = &[
    (
        "sndinfo/prntsnd",
        "invented usage text; writes samples to standard output instead of the outtextfile argument",
    ),
    (
        "housekeep/bakup",
        "invented a splicelen argument; legacy takes none and uses the BAKUP_GAP constant",
    ),
    (
        "housekeep/extract",
        "usage text drops the trailing space legacy prints after the sub-command name",
    ),
    (
        "housekeep/remove",
        "prints an insufficient-parameters error when bare instead of the usage text; missing the -a flag",
    ),
    (
        "housekeep/sort",
        "invented usage text, mode numbering and an outfile argument; legacy has six modes and derives its output names",
    ),
];

fn known_failure(name: &str) -> Option<&'static str> {
    KNOWN_FAILURES
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, why)| *why)
}

#[test]
fn every_subcommand_prints_the_captured_legacy_usage_text() {
    let root = repo_root();
    let mut failures = Vec::new();

    for entry in COMMANDS {
        let name = format!("{}/{}", entry.program, entry.subcommand);
        let spec = root.join(usage_spec_path(entry));

        let expected = match std::fs::read_to_string(&spec) {
            Ok(text) => text,
            Err(err) => {
                failures.push(format!(
                    "{name}: cannot read {}: {err}\n  Capture it from the legacy image before porting.",
                    spec.display()
                ));
                continue;
            }
        };

        let (bin, args) = invocation(entry);
        let out = Command::new(&bin)
            .args(&args)
            .output()
            .unwrap_or_else(|e| panic!("{name}: cannot run {}: {e}", bin.display()));

        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        let code = out.status.code();

        if stdout != expected {
            failures.push(format!(
                "{name}: usage text does not match {}\n{}",
                spec.display(),
                first_difference(&stdout, &expected)
            ));
        }
        if !stderr.is_empty() {
            failures.push(format!(
                "{name}: wrote {} bytes to standard error, legacy writes none:\n  {stderr:?}",
                stderr.len()
            ));
        }
        if code != Some(USAGE_EXIT_CODE) {
            failures.push(format!(
                "{name}: exit code is {code:?}, legacy exits {USAGE_EXIT_CODE}"
            ));
        }
    }

    let mut regressions = Vec::new();
    let mut fixed = Vec::new();

    for entry in COMMANDS {
        let name = format!("{}/{}", entry.program, entry.subcommand);
        let failed = failures.iter().any(|f| f.starts_with(&format!("{name}:")));
        match (failed, known_failure(&name)) {
            (true, None) => regressions.extend(
                failures
                    .iter()
                    .filter(|f| f.starts_with(&format!("{name}:")))
                    .cloned(),
            ),
            (false, Some(why)) => fixed.push(format!("{name}: baseline says {why:?}")),
            _ => {}
        }
    }

    assert!(
        regressions.is_empty(),
        "\n{} sub-command(s) fail usage-text conformance and are not in the \
         KNOWN_FAILURES baseline (PLAN-V2 gate 1):\n\n{}\n\n\
         Copy the legacy text from spec/usage/ rather than writing your own.\n",
        regressions.len(),
        regressions.join("\n\n")
    );

    assert!(
        fixed.is_empty(),
        "\n{} sub-command(s) now pass this gate but are still listed in \
         KNOWN_FAILURES:\n\n{}\n\nRemove the entries from \
         crates/cdp-cli/tests/usage_conformance.rs.\n",
        fixed.len(),
        fixed.join("\n")
    );
}

/// Reports the first line that differs, so that a failure names the
/// defect instead of printing two whole usage texts.
fn first_difference(actual: &str, expected: &str) -> String {
    let (a, e): (Vec<_>, Vec<_>) = (actual.lines().collect(), expected.lines().collect());
    for i in 0..a.len().max(e.len()) {
        let got = a.get(i);
        let want = e.get(i);
        if got != want {
            return format!(
                "  line {}:\n    actual  : {:?}\n    expected: {:?}",
                i + 1,
                got.unwrap_or(&"<end of output>"),
                want.unwrap_or(&"<end of file>")
            );
        }
    }
    format!(
        "  lines match; trailing bytes differ (actual {} bytes, expected {} bytes)",
        actual.len(),
        expected.len()
    )
}

/// Makes the registry authoritative: a dispatch arm in `cdp-cli` that
/// has no registry entry fails here, so a new sub-command cannot skip
/// gate 1 by simply not being listed.
#[test]
fn registry_covers_every_dispatched_subcommand() {
    let lib = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let source = std::fs::read_to_string(&lib).expect("read cdp-cli/src/lib.rs");

    let mut dispatched: Vec<String> = source
        .match_indices("subcommand == \"")
        .filter_map(|(at, pat)| {
            let rest = &source[at + pat.len()..];
            rest.find('"').map(|end| rest[..end].to_string())
        })
        .collect();
    dispatched.sort();
    dispatched.dedup();

    let mut registered: Vec<String> = COMMANDS.iter().map(|c| c.subcommand.to_string()).collect();
    registered.sort();
    registered.dedup();

    let missing: Vec<_> = dispatched
        .iter()
        .filter(|s| !registered.contains(s))
        .collect();
    let extra: Vec<_> = registered
        .iter()
        .filter(|s| !dispatched.contains(s))
        .collect();

    assert!(
        missing.is_empty() && extra.is_empty(),
        "registry and cdp-cli dispatch disagree.\n  \
         dispatched but not in registry: {missing:?}\n  \
         in registry but not dispatched: {extra:?}\n  \
         Add the missing entries to crates/cdp-programs/src/registry.rs."
    );
}
