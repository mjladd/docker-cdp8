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

//! The golden-case file format (V4 of `docs/migration/PLAN-V2.md`).
//!
//! One case is one TOML file under
//! `spec/golden/<program>/<subcommand>/<case>.toml`. It holds two parts:
//! what to run, written by whoever adds the case, and what legacy did,
//! written by `cdp-oracle record`.
//!
//! TOML is the format rather than JSON because an expectation file earns
//! its keep in code review, and TOML writes multi-line output as a
//! readable block. The same text in JSON is one line of escapes.
//!
//! ```toml
//! description = "a real corpus file with a DATE property and no PEAK chunk"
//! program = "sndinfo"
//! subcommand = "props"
//! argv = ["props", "in.wav"]
//!
//! [[inputs]]
//! name = "in.wav"
//! from = "docs/manual/sounds/marimba.wav"
//!
//! [expected]
//! exit_code = 0
//! stdout = """
//! ...
//! """
//! stderr = ""
//! ```
//!
//! Every path in `from` is relative to the repository root, and the file
//! is copied into a sandbox directory under the name in `name`. The
//! command runs with the sandbox as its working directory, so `argv`
//! never carries an absolute path and a recorded case stays portable.

use serde::{Deserialize, Serialize};

/// One golden case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Case {
    /// Why this case exists: the mode, flag or edge it covers.
    pub description: String,
    /// Legacy program name, which must match a
    /// [`cdp_programs::registry`] entry.
    pub program: String,
    /// Sub-command name, which must match the same entry.
    pub subcommand: String,
    /// Arguments as typed after the launcher binary name, starting with
    /// the sub-command. For a program with no launcher the runner
    /// prepends `cdp <program>` itself.
    pub argv: Vec<String>,
    /// Files copied into the sandbox before the run.
    #[serde(default)]
    pub inputs: Vec<Input>,
    /// The largest absolute sample difference this case accepts in an
    /// output sound file. Absent means the samples must match exactly,
    /// which is the default.
    ///
    /// legacy: PLAN.md section 5, "Tolerance rules are per case and can be
    /// tightened. A case that only passes at a wider tolerance must say why
    /// in its notes." Setting this without [`Self::tolerance_reason`] is
    /// refused, so a tolerance always carries its justification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_tolerance: Option<f64>,
    /// Why this case needs a tolerance at all.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tolerance_reason: Option<String>,
    /// Why this case is known not to match yet, when it is. A case
    /// carrying this is expected to fail, so `verify` reports it without
    /// failing the run.
    ///
    /// The marker is tight in both directions, exactly as gate 1's
    /// baseline is: a case that carries it and then starts passing fails
    /// the run, so the marker cannot outlive the defect it records.
    /// legacy: PLAN.md section 3 step 5 calls this a `known-deviation`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub known_deviation: Option<String>,
    /// What legacy produced. Absent until `cdp-oracle record` fills it.
    pub expected: Option<Expected>,
}

/// One input file, copied from the repository into the sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    /// The name the command refers to, inside the sandbox.
    pub name: String,
    /// Where the bytes come from, relative to the repository root.
    pub from: String,
}

/// What a recorded run produced.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expected {
    /// The process exit code. legacy returns only 0 or -1, which a shell
    /// reports as 0 or 255 (see `cdp_core::report_and_exit`).
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    /// The files the run left in the sandbox. See
    /// [`crate::output`] for what is recorded and what is deliberately
    /// left out.
    #[serde(default)]
    pub outputs: crate::output::Outputs,
}

impl Case {
    pub fn load(path: &std::path::Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        toml::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))
    }

    pub fn save(&self, path: &std::path::Path) -> Result<(), String> {
        let text = toml::to_string_pretty(self)
            .map_err(|e| format!("cannot serialise {}: {e}", path.display()))?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
        std::fs::write(path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))
    }

    /// Checks the case against the registry, so that a typo in a program
    /// or sub-command name fails when the case is read rather than
    /// producing a confusing run failure later.
    /// Refuses a tolerance with no stated reason, so that a loosened
    /// comparison cannot enter the repository unexplained.
    pub fn check_tolerance(&self) -> Result<(), String> {
        match (self.sample_tolerance, &self.tolerance_reason) {
            (Some(_), None) => Err(format!(
                "{}/{}: sample_tolerance is set with no tolerance_reason. \
                 PLAN.md section 5 requires a case that passes only at a wider \
                 tolerance to say why.",
                self.program, self.subcommand
            )),
            (None, Some(why)) => Err(format!(
                "{}/{}: tolerance_reason is set ({why:?}) but sample_tolerance is not",
                self.program, self.subcommand
            )),
            _ => Ok(()),
        }
    }

    pub fn registry_entry(&self) -> Result<&'static cdp_programs::registry::CommandEntry, String> {
        cdp_programs::registry::COMMANDS
            .iter()
            .find(|c| c.program == self.program && c.subcommand == self.subcommand)
            .ok_or_else(|| {
                format!(
                    "{}/{} is not in the cdp-programs registry, so it is not wired into cdp-cli",
                    self.program, self.subcommand
                )
            })
    }
}
