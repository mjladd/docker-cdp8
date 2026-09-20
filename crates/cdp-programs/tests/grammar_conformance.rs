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

//! Gate 2 of `docs/migration/PLAN-V2.md`: argument-grammar conformance.
//!
//! Legacy's own argument grammar was extracted from
//! `legacy/dev/cdp2k/parstruct.c` into
//! `spec/commands/_raw/parstruct.json` during WP-0.2: 299 processes and
//! 645 process-and-mode pairs, each recording the count of required
//! parameters, the type letter of each one, the option-flag letters with
//! their own type letters, the variant-flag letters, and the special-data
//! kind.
//!
//! This test compares every wired-up mode's [`cdp_params::CommandSpec`]
//! against that record. It catches the defect class that gate 1 cannot:
//! an invented positional parameter, a missing flag, or a wrong mode
//! number, in a sub-command whose usage text is correct. Gate 1 is blind
//! to those whenever one usage text covers several modes, as
//! `housekeep extract`'s does.
//!
//! The type letters are the ones `mark_parameter_types`
//! (`legacy/dev/cdp2k/tklib3.c`) reads:
//!
//! | letter | [`cdp_params::ParamType`] |
//! |---|---|
//! | `d` | `Double`, no breakpoint-file fallback |
//! | `D` | `DoubleOrBreakpoint` |
//! | `i` | `Int` |
//! | `I` | `IntOrBreakpoint` |
//! | `0` | an unused padding slot, ignored |
//!
//! This test lives in `cdp-programs` rather than in `cdp-params`, where
//! PLAN-V2 section 6 first placed it, because `cdp-params` cannot see
//! the registry: `cdp-programs` depends on `cdp-params`, not the other
//! way round.

use cdp_params::{CommandSpec, OptionFlag, ParamType, Variant};
use cdp_programs::registry::{COMMANDS, ModeEntry};
use serde_json::Value;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("repository root above crates/cdp-programs")
        .to_path_buf()
}

fn parstruct() -> Value {
    let path = repo_root().join("spec/commands/_raw/parstruct.json");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    serde_json::from_str(&text).expect("parstruct.json is valid JSON")
}

/// The type letter that `parstruct.c` records for a [`ParamType`].
fn type_letter(param: &ParamType) -> char {
    match param {
        ParamType::Double { .. } => 'd',
        ParamType::DoubleOrBreakpoint { .. } => 'D',
        ParamType::Int { .. } => 'i',
        ParamType::IntOrBreakpoint { .. } => 'I',
        // legacy records file arguments outside the parameter list, so
        // a File parameter has no letter of its own.
        ParamType::File => '?',
    }
}

fn flag_letters(flags: &[OptionFlag]) -> String {
    flags.iter().map(|f| f.letter).collect()
}

fn variant_letters(variants: &[Variant]) -> String {
    variants.iter().map(Variant::letter).collect()
}

/// Reads a field out of a mode's record, falling back to the process's
/// own `null` mode. `parstruct.c` often sets the parameter list once,
/// outside the per-mode switch, so a named mode legitimately carries
/// only its variant flags.
fn field<'a>(process: &'a Value, legacy_mode: &str, group: &str, key: &str) -> Option<&'a str> {
    let modes = process.get("modes")?;
    modes
        .get(legacy_mode)
        .and_then(|m| m.get(group))
        .and_then(|g| g.get(key))
        .or_else(|| {
            modes
                .get("null")
                .and_then(|m| m.get(group))
                .and_then(|g| g.get(key))
        })
        .and_then(Value::as_str)
}

fn checked_count(process: &Value, legacy_mode: &str, group: &str, key: &str) -> usize {
    field(process, legacy_mode, group, key)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}

/// Compares one mode's spec against its recorded grammar, returning one
/// message per disagreement.
fn check_mode(name: &str, process: &Value, entry: &ModeEntry, spec: &CommandSpec) -> Vec<String> {
    let mut out = Vec::new();
    let m = entry.legacy_mode;
    let label = match entry.cli_mode {
        Some(n) => format!("{name} mode {n} ({m})"),
        None => format!("{name} ({m})"),
    };

    // Required positional parameters: count, then type letter by
    // position. Trailing `0` entries in param_list are padding.
    let want_count = checked_count(process, m, "param", "param_cnt");
    let want_list: Vec<char> = field(process, m, "param", "param_list")
        .unwrap_or("")
        .chars()
        .filter(|c| *c != '0')
        .collect();
    let got_list: Vec<char> = spec.params.iter().map(type_letter).collect();

    if spec.params.len() != want_count {
        out.push(format!(
            "{label}: spec declares {} required parameter(s), parstruct records {want_count}",
            spec.params.len()
        ));
    }
    if got_list != want_list && spec.params.len() == want_count {
        out.push(format!(
            "{label}: parameter types are {got_list:?}, parstruct records {want_list:?}"
        ));
    }

    // Option flags: letters, in the order parstruct records them.
    let want_opts = field(process, m, "vflags", "opt_flags").unwrap_or("");
    let got_opts = flag_letters(&spec.flags);
    if got_opts != want_opts {
        out.push(format!(
            "{label}: option flags are {got_opts:?}, parstruct records {want_opts:?}"
        ));
    }

    // Option-flag value types, positionally aligned with the letters.
    let want_opt_types: Vec<char> = field(process, m, "vflags", "opt_list")
        .unwrap_or("")
        .chars()
        .filter(|c| *c != '0')
        .collect();
    let got_opt_types: Vec<char> = spec
        .flags
        .iter()
        .map(|f| type_letter(&f.value_type))
        .collect();
    if got_opts == want_opts && got_opt_types != want_opt_types {
        out.push(format!(
            "{label}: option flag value types are {got_opt_types:?}, parstruct records {want_opt_types:?}"
        ));
    }

    // Variant flags: letters only. A variant carries a value or not,
    // which parstruct records separately as vparam_cnt.
    let want_vars = field(process, m, "vflags", "var_flags").unwrap_or("");
    let got_vars = variant_letters(&spec.variants);
    if got_vars != want_vars {
        out.push(format!(
            "{label}: variant flags are {got_vars:?}, parstruct records {want_vars:?}"
        ));
    }

    // Special data: a mode that legacy reads a special-data argument
    // for needs a mechanism cdp-params does not model at all, so a spec
    // that claims to cover such a mode is incomplete by construction.
    if let Some(kind) = field(process, m, "param", "special_data")
        && kind != "0"
    {
        out.push(format!(
            "{label}: parstruct records special_data {kind:?}, which cdp-params does not model. \
             Mark the mode exempt in the registry, or add the mechanism."
        ));
    }

    out
}

#[test]
fn every_wired_mode_matches_the_recorded_legacy_grammar() {
    let data = parstruct();
    let mut failures = Vec::new();
    let mut checked = 0usize;

    for command in COMMANDS {
        let name = format!("{}/{}", command.program, command.subcommand);

        let process = match data.get(command.legacy_process) {
            Some(p) => p,
            None => {
                failures.push(format!(
                    "{name}: parstruct.json has no process {:?}. \
                     Take the symbol from the program's own ap_*.c get_process_no.",
                    command.legacy_process
                ));
                continue;
            }
        };

        assert!(
            !command.modes.is_empty(),
            "{name}: registry lists no modes at all"
        );

        for entry in command.modes {
            // A named mode must exist in the record, so that a wrong
            // mode name cannot pass by silently falling back to `null`.
            let known = entry.legacy_mode == "null"
                || process
                    .get("modes")
                    .and_then(|m| m.get(entry.legacy_mode))
                    .is_some();
            if !known {
                failures.push(format!(
                    "{name}: mode {:?} is not a mode of {}. Its real modes are {:?} \
                     (numbers come from docs/legacy/dev/include/modeno.h).",
                    entry.legacy_mode,
                    command.legacy_process,
                    process
                        .get("modes")
                        .and_then(Value::as_object)
                        .map(|o| o.keys().cloned().collect::<Vec<_>>())
                        .unwrap_or_default()
                ));
                continue;
            }

            match entry.spec {
                Some(build) => {
                    checked += 1;
                    failures.extend(check_mode(&name, process, entry, &build()));
                }
                None => assert!(
                    entry.exempt_reason.is_some(),
                    "{name}: mode {:?} has neither a spec nor an exemption reason",
                    entry.legacy_mode
                ),
            }
        }
    }

    assert!(
        failures.is_empty(),
        "\n{} grammar disagreement(s) across {checked} checked mode(s) (PLAN-V2 gate 2):\n\n{}\n\n\
         parstruct.json is the extracted record of legacy's own parstruct.c. \
         When it disagrees with the usage text, the record wins, because the record \
         is what legacy's parser runs.\n",
        failures.len(),
        failures.join("\n")
    );
}

/// Every mode without a spec must say why, and the reason must point at
/// a real mechanism or plan item rather than being left blank.
#[test]
fn every_exempt_mode_records_a_reason() {
    let mut thin = Vec::new();
    for command in COMMANDS {
        for entry in command.modes {
            if entry.spec.is_none() {
                match entry.exempt_reason {
                    Some(why) if why.len() >= 20 => {}
                    other => thin.push(format!(
                        "{}/{} mode {:?}: exemption reason is {other:?}",
                        command.program, command.subcommand, entry.legacy_mode
                    )),
                }
            }
        }
    }
    assert!(
        thin.is_empty(),
        "exemptions must explain themselves:\n{}",
        thin.join("\n")
    );
}

/// Proves this gate would have blocked the defect it was built for.
///
/// The merged `sndinfo findhole` demanded one required positional
/// threshold. Legacy takes no parameter and one optional `-t` flag, so
/// `sndinfo findhole a.wav` runs under legacy and was rejected by this
/// build. That command line is the one the usage text documents.
#[test]
fn the_gate_rejects_the_grammar_that_findhole_actually_shipped() {
    let data = parstruct();
    let process = data
        .get("INFO_FINDHOLE")
        .expect("parstruct.json records INFO_FINDHOLE");

    let as_shipped = CommandSpec {
        infile_count: 1,
        has_outfile: false,
        params: vec![ParamType::Double {
            lo: 0.0,
            hi: 1.0,
            legacy_index: 1,
        }],
        flags: vec![],
        variants: vec![],
        unequal_sndfile: false,
    };

    let entry = ModeEntry {
        cli_mode: None,
        legacy_mode: "null",
        spec: None,
        exempt_reason: None,
    };
    let complaints = check_mode("sndinfo/findhole", process, &entry, &as_shipped);

    assert!(
        complaints
            .iter()
            .any(|c| c.contains("required parameter") && c.contains("parstruct records 0")),
        "gate 2 must reject the invented positional parameter, got: {complaints:?}"
    );
    assert!(
        complaints
            .iter()
            .any(|c| c.contains("option flags") && c.contains("\"t\"")),
        "gate 2 must report the missing -t option flag, got: {complaints:?}"
    );
}

/// The same proof for `housekeep bakup`, whose merged version invented a
/// `splicelen` argument. Legacy takes none: the gap is the compile-time
/// constant `BAKUP_GAP` (1.0 second) in
/// `docs/legacy/dev/include/house.h`.
#[test]
fn the_gate_rejects_the_grammar_that_bakup_actually_shipped() {
    let data = parstruct();
    let process = data
        .get("HOUSE_BAKUP")
        .expect("parstruct.json records HOUSE_BAKUP");

    let as_shipped = CommandSpec {
        infile_count: 1,
        has_outfile: true,
        params: vec![ParamType::Double {
            lo: 0.0,
            hi: 60.0,
            legacy_index: 1,
        }],
        flags: vec![],
        variants: vec![],
        unequal_sndfile: false,
    };

    let entry = ModeEntry {
        cli_mode: None,
        legacy_mode: "null",
        spec: None,
        exempt_reason: None,
    };
    let complaints = check_mode("housekeep/bakup", process, &entry, &as_shipped);

    assert!(
        complaints
            .iter()
            .any(|c| c.contains("required parameter") && c.contains("parstruct records 0")),
        "gate 2 must reject the invented splicelen argument, got: {complaints:?}"
    );
}
