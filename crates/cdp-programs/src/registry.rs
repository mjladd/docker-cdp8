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

//! The registry of every sub-command wired into `cdp-cli`.
//!
//! This table exists so that the conformance gates of
//! `docs/migration/PLAN-V2.md` can enumerate the ported commands
//! instead of trusting an agent to remember to test a new one.
//!
//! Gate 1 (`crates/cdp-cli/tests/usage_conformance.rs`) runs every
//! entry's bare sub-command invocation and compares the output against
//! the captured legacy text in `spec/usage/<program>/<subcommand>.txt`.
//!
//! Gate 2 (`crates/cdp-programs/tests/grammar_conformance.rs`) compares
//! every entry's [`CommandEntry::legacy_process`] against the recorded
//! argument grammar in `spec/commands/_raw/parstruct.json`.
//!
//! Adding a dispatch arm to `cdp-cli` without adding an entry here
//! fails `registry_covers_every_dispatched_subcommand`, so the gates
//! cannot be skipped by omission.

/// One sub-command of one legacy program.
pub struct CommandEntry {
    /// Legacy program name, as typed (`sndinfo`, `housekeep`, ...).
    pub program: &'static str,
    /// Sub-command name, as typed (`props`, `chans`, ...).
    pub subcommand: &'static str,
    /// The usage text this sub-command prints when invoked bare.
    pub usage: &'static str,
    /// The `parstruct.json` top-level process symbol, taken from the
    /// program's own `ap_*.c` dispatch (`get_process_no`).
    pub legacy_process: &'static str,
    /// The legacy-name launcher binary, when one is built. `None`
    /// means the sub-command is reachable only as `cdp <program>
    /// <subcommand>`.
    pub launcher: Option<&'static str>,
}

/// Every sub-command `cdp-cli` dispatches.
pub const COMMANDS: &[CommandEntry] = &[
    // sndinfo (legacy: docs/legacy/dev/sndinfo/ap_sndinfo.c).
    entry(
        "sndinfo",
        "props",
        crate::sndinfo::props::USAGE,
        "INFO_PROPS",
    ),
    entry("sndinfo", "len", crate::sndinfo::len::USAGE, "INFO_SFLEN"),
    entry(
        "sndinfo",
        "lens",
        crate::sndinfo::lens::USAGE,
        "INFO_TIMELIST",
    ),
    entry(
        "sndinfo",
        "sumlen",
        crate::sndinfo::sumlen::USAGE,
        "INFO_TIMESUM",
    ),
    entry(
        "sndinfo",
        "timediff",
        crate::sndinfo::timediff::USAGE,
        "INFO_TIMEDIFF",
    ),
    entry(
        "sndinfo",
        "smptime",
        crate::sndinfo::smptime::USAGE,
        "INFO_SAMPTOTIME",
    ),
    entry(
        "sndinfo",
        "timesmp",
        crate::sndinfo::timesmp::USAGE,
        "INFO_TIMETOSAMP",
    ),
    entry(
        "sndinfo",
        "maxsamp",
        crate::sndinfo::maxsamp::USAGE,
        "INFO_MAXSAMP",
    ),
    entry(
        "sndinfo",
        "units",
        crate::sndinfo::units::USAGE,
        "INFO_MUSUNITS",
    ),
    entry(
        "sndinfo",
        "prntsnd",
        crate::sndinfo::prntsnd::USAGE,
        "INFO_PRNTSND",
    ),
    entry(
        "sndinfo",
        "findhole",
        crate::sndinfo::findhole::USAGE,
        "INFO_FINDHOLE",
    ),
    // housekeep (legacy: docs/legacy/dev/houskeep/ap_house.c).
    entry(
        "housekeep",
        "copy",
        crate::housekeep::copy::USAGE,
        "HOUSE_COPY",
    ),
    entry(
        "housekeep",
        "chans",
        crate::housekeep::chans::USAGE,
        "HOUSE_CHANS",
    ),
    entry(
        "housekeep",
        "respec",
        crate::housekeep::respec::USAGE,
        "HOUSE_SPEC",
    ),
    entry(
        "housekeep",
        "bakup",
        crate::housekeep::bakup::USAGE,
        "HOUSE_BAKUP",
    ),
    entry(
        "housekeep",
        "bundle",
        crate::housekeep::bundle::USAGE,
        "HOUSE_BUNDLE",
    ),
    entry(
        "housekeep",
        "extract",
        crate::housekeep::extract::USAGE,
        "HOUSE_EXTRACT",
    ),
    entry(
        "housekeep",
        "remove",
        crate::housekeep::remove::USAGE,
        "HOUSE_DEL",
    ),
    entry(
        "housekeep",
        "sort",
        crate::housekeep::sort::USAGE,
        "HOUSE_SORT",
    ),
    // synth (legacy: docs/legacy/dev/synth/ap_synthesis.c).
    entry("synth", "wave", crate::synth::wave::USAGE, "SYNTH_WAVE"),
    // pvoc (legacy: docs/legacy/dev/pv/ap_pvoc.c). No launcher binary
    // is built for `pvoc` yet, so this one is `cdp pvoc anal` only.
    CommandEntry {
        program: "pvoc",
        subcommand: "anal",
        usage: crate::pvoc::anal::USAGE,
        legacy_process: "PVOC_ANAL",
        launcher: None,
    },
];

/// Builds an entry whose launcher binary shares the program's name.
const fn entry(
    program: &'static str,
    subcommand: &'static str,
    usage: &'static str,
    legacy_process: &'static str,
) -> CommandEntry {
    CommandEntry {
        program,
        subcommand,
        usage,
        legacy_process,
        launcher: Some(program),
    }
}

/// The path of a sub-command's captured legacy usage text, relative to
/// the repository root.
pub fn usage_spec_path(entry: &CommandEntry) -> String {
    format!("spec/usage/{}/{}.txt", entry.program, entry.subcommand)
}
