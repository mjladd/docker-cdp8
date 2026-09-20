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

/// One mode of one sub-command.
///
/// A mode is checked by gate 2 only when [`Self::spec`] is present.
/// Several ported sub-commands parse their own arguments instead of
/// calling [`cdp_params::parse`], for reasons recorded in their own
/// module docs, so they carry an [`Self::exempt_reason`] instead.
pub struct ModeEntry {
    /// Mode number as typed on the command line, or `None` when the
    /// sub-command takes no mode number at all.
    pub cli_mode: Option<u32>,
    /// The key of this mode inside its process's `modes` map in
    /// `spec/commands/_raw/parstruct.json`. Mode numbers come from
    /// `docs/legacy/dev/include/modeno.h`, which is zero-based, so
    /// `cli_mode` is one higher than the constant there.
    pub legacy_mode: &'static str,
    /// Builds the spec whose grammar gate 2 compares. Dynamic ranges
    /// are irrelevant to that comparison, so a builder that needs a
    /// bound is called with a placeholder.
    pub spec: Option<fn() -> cdp_params::CommandSpec>,
    /// Why no spec exists, when none does.
    pub exempt_reason: Option<&'static str>,
}

/// A mode whose grammar gate 2 checks.
const fn mode(
    cli_mode: u32,
    legacy_mode: &'static str,
    spec: fn() -> cdp_params::CommandSpec,
) -> ModeEntry {
    ModeEntry {
        cli_mode: Some(cli_mode),
        legacy_mode,
        spec: Some(spec),
        exempt_reason: None,
    }
}

/// A sub-command with no mode number, whose grammar gate 2 checks.
const fn unmoded(spec: fn() -> cdp_params::CommandSpec) -> ModeEntry {
    ModeEntry {
        cli_mode: None,
        legacy_mode: "null",
        spec: Some(spec),
        exempt_reason: None,
    }
}

/// A mode that gate 2 cannot check, with the reason.
const fn exempt(cli_mode: Option<u32>, legacy_mode: &'static str, why: &'static str) -> ModeEntry {
    ModeEntry {
        cli_mode,
        legacy_mode,
        spec: None,
        exempt_reason: Some(why),
    }
}

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
    /// Every mode of this sub-command that is wired up.
    pub modes: &'static [ModeEntry],
}

/// Every sub-command `cdp-cli` dispatches.
pub const COMMANDS: &[CommandEntry] = &[
    // sndinfo (legacy: docs/legacy/dev/sndinfo/ap_sndinfo.c).
    entry(
        "sndinfo",
        "props",
        crate::sndinfo::props::USAGE,
        "INFO_PROPS",
        &[unmoded(cdp_params::CommandSpec::sndinfo_props)],
    ),
    entry(
        "sndinfo",
        "len",
        crate::sndinfo::len::USAGE,
        "INFO_SFLEN",
        &[unmoded(cdp_params::CommandSpec::sndinfo_len)],
    ),
    entry(
        "sndinfo",
        "lens",
        crate::sndinfo::lens::USAGE,
        "INFO_TIMELIST",
        &[exempt(
            None,
            "null",
            "MANY_SNDFILES: the infile list is consumed by hand before parsing, see the lens module doc",
        )],
    ),
    entry(
        "sndinfo",
        "sumlen",
        crate::sndinfo::sumlen::USAGE,
        "INFO_TIMESUM",
        &[exempt(
            None,
            "null",
            "MANY_SNDFILES: only the -s flag goes through cdp_params::parse, see the sumlen module doc",
        )],
    ),
    entry(
        "sndinfo",
        "timediff",
        crate::sndinfo::timediff::USAGE,
        "INFO_TIMEDIFF",
        &[exempt(
            None,
            "null",
            "TWO_SNDFILES: legacy validates the two infiles in an interleaved order that a generic infile_count check cannot reproduce, see the timediff module doc",
        )],
    ),
    entry(
        "sndinfo",
        "smptime",
        crate::sndinfo::smptime::USAGE,
        "INFO_SAMPTOTIME",
        &[unmoded(smptime_spec)],
    ),
    entry(
        "sndinfo",
        "timesmp",
        crate::sndinfo::timesmp::USAGE,
        "INFO_TIMETOSAMP",
        &[unmoded(timesmp_spec)],
    ),
    entry(
        "sndinfo",
        "maxsamp",
        crate::sndinfo::maxsamp::USAGE,
        "INFO_MAXSAMP",
        &[unmoded(cdp_params::CommandSpec::sndinfo_maxsamp)],
    ),
    entry(
        "sndinfo",
        "units",
        crate::sndinfo::units::USAGE,
        "INFO_MUSUNITS",
        &[
            mode(1, "MU_MIDI_TO_FRQ", units_spec),
            mode(2, "MU_FRQ_TO_MIDI", units_spec),
        ],
    ),
    entry(
        "sndinfo",
        "prntsnd",
        crate::sndinfo::prntsnd::USAGE,
        "INFO_PRNTSND",
        &[exempt(
            None,
            "null",
            "PLAN-V2 phase R: arguments are parsed by hand in cdp-cli and the grammar is wrong",
        )],
    ),
    entry(
        "sndinfo",
        "findhole",
        crate::sndinfo::findhole::USAGE,
        "INFO_FINDHOLE",
        &[exempt(
            None,
            "null",
            "PLAN-V2 phase R: arguments are parsed by hand in cdp-cli and the grammar is wrong",
        )],
    ),
    // housekeep (legacy: docs/legacy/dev/houskeep/ap_house.c).
    entry(
        "housekeep",
        "copy",
        crate::housekeep::copy::USAGE,
        "HOUSE_COPY",
        &[
            mode(1, "COPYSF", cdp_params::CommandSpec::housekeep_copy_once),
            exempt(
                Some(2),
                "DUPL",
                "PLAN-V2 phase R: needs the SNDFILENAME special-data mechanism, which cdp-params does not model",
            ),
        ],
    ),
    entry(
        "housekeep",
        "chans",
        crate::housekeep::chans::USAGE,
        "HOUSE_CHANS",
        &[
            mode(1, "HOUSE_CHANNEL", chans_channel_spec),
            mode(
                2,
                "HOUSE_CHANNELS",
                cdp_params::CommandSpec::housekeep_chans_channels,
            ),
            mode(3, "HOUSE_ZCHANNEL", chans_zchannel_spec),
            mode(4, "STOM", cdp_params::CommandSpec::housekeep_chans_stom),
            mode(5, "MTOS", cdp_params::CommandSpec::housekeep_chans_mtos),
        ],
    ),
    entry(
        "housekeep",
        "respec",
        crate::housekeep::respec::USAGE,
        "HOUSE_SPEC",
        &[
            exempt(
                Some(1),
                "HOUSE_RESAMPLE",
                "not ported: cubic-spline resampling, see PLAN-V2 section 8",
            ),
            mode(
                2,
                "HOUSE_CONVERT",
                cdp_params::CommandSpec::housekeep_respec_convert,
            ),
            mode(
                3,
                "HOUSE_REPROP",
                cdp_params::CommandSpec::housekeep_respec_reprop,
            ),
        ],
    ),
    entry(
        "housekeep",
        "bakup",
        crate::housekeep::bakup::USAGE,
        "HOUSE_BAKUP",
        &[exempt(
            None,
            "null",
            "PLAN-V2 phase R: withdraw and re-port, the shipped code invents a splicelen argument",
        )],
    ),
    entry(
        "housekeep",
        "bundle",
        crate::housekeep::bundle::USAGE,
        "HOUSE_BUNDLE",
        &[
            // parstruct.c sets HOUSE_BUNDLE's grammar once rather than per
            // mode, so all five modes share the one `null` record even
            // though modeno.h names them BUNDLE_ALL to BUNDLE_CHAN (0-4).
            exempt(
                Some(1),
                "null",
                "PLAN-V2 phase R: arguments are parsed by hand, grammar unchecked",
            ),
            exempt(
                Some(2),
                "null",
                "PLAN-V2 phase R: arguments are parsed by hand, grammar unchecked",
            ),
            exempt(
                Some(3),
                "null",
                "PLAN-V2 phase R: arguments are parsed by hand, grammar unchecked",
            ),
            exempt(
                Some(4),
                "null",
                "PLAN-V2 phase R: arguments are parsed by hand, grammar unchecked",
            ),
            exempt(
                Some(5),
                "null",
                "PLAN-V2 phase R: arguments are parsed by hand, grammar unchecked",
            ),
        ],
    ),
    entry(
        "housekeep",
        "extract",
        crate::housekeep::extract::USAGE,
        "HOUSE_EXTRACT",
        &[exempt(
            Some(4),
            "HOUSE_RECTIFY",
            "PLAN-V2 phase R: the shipped mode ignores the required shift argument",
        )],
    ),
    entry(
        "housekeep",
        "remove",
        crate::housekeep::remove::USAGE,
        "HOUSE_DEL",
        &[exempt(
            None,
            "null",
            "PLAN-V2 phase R: missing the -a variant flag and the SNDFILENAME mechanism",
        )],
    ),
    entry(
        "housekeep",
        "sort",
        crate::housekeep::sort::USAGE,
        "HOUSE_SORT",
        &[
            exempt(
                Some(1),
                "BY_FILETYPE",
                "PLAN-V2 phase R: withdraw and re-port, the shipped mode numbering does not match legacy",
            ),
            exempt(
                Some(2),
                "BY_SRATE",
                "PLAN-V2 phase R: withdraw and re-port, the shipped mode numbering does not match legacy",
            ),
            exempt(
                Some(4),
                "BY_LOG_DUR",
                "PLAN-V2 phase R: withdraw and re-port, the shipped mode 4 sorts by channel count, which is not a legacy mode",
            ),
        ],
    ),
    // synth (legacy: docs/legacy/dev/synth/ap_synthesis.c).
    entry(
        "synth",
        "wave",
        crate::synth::wave::USAGE,
        "SYNTH_WAVE",
        &[
            mode(1, "null", cdp_params::CommandSpec::synth_wave),
            mode(2, "null", cdp_params::CommandSpec::synth_wave),
            mode(3, "null", cdp_params::CommandSpec::synth_wave),
            mode(4, "null", cdp_params::CommandSpec::synth_wave),
        ],
    ),
    // pvoc (legacy: docs/legacy/dev/pv/ap_pvoc.c). No launcher binary
    // is built for `pvoc` yet, so this one is `cdp pvoc anal` only.
    CommandEntry {
        program: "pvoc",
        subcommand: "anal",
        usage: crate::pvoc::anal::USAGE,
        legacy_process: "PVOC_ANAL",
        launcher: None,
        modes: &[mode(1, "null", cdp_params::CommandSpec::pvoc_anal)],
    },
];

// Wrappers for the spec builders that take a dynamic range bound. Gate
// 2 compares parameter counts, types and flag letters, never ranges, so
// the placeholder bound does not affect the comparison.
fn smptime_spec() -> cdp_params::CommandSpec {
    cdp_params::CommandSpec::sndinfo_smptime(PLACEHOLDER_BOUND)
}
fn timesmp_spec() -> cdp_params::CommandSpec {
    cdp_params::CommandSpec::sndinfo_timesmp(PLACEHOLDER_BOUND)
}
fn units_spec() -> cdp_params::CommandSpec {
    cdp_params::CommandSpec::sndinfo_units(0.0, PLACEHOLDER_BOUND)
}
fn chans_channel_spec() -> cdp_params::CommandSpec {
    cdp_params::CommandSpec::housekeep_chans_channel(PLACEHOLDER_BOUND)
}
fn chans_zchannel_spec() -> cdp_params::CommandSpec {
    cdp_params::CommandSpec::housekeep_chans_zchannel(PLACEHOLDER_BOUND)
}

/// Stands in for a range bound that a real run reads from an open
/// infile or a mode number.
const PLACEHOLDER_BOUND: f64 = 1.0;

/// Builds an entry whose launcher binary shares the program's name.
const fn entry(
    program: &'static str,
    subcommand: &'static str,
    usage: &'static str,
    legacy_process: &'static str,
    modes: &'static [ModeEntry],
) -> CommandEntry {
    CommandEntry {
        program,
        subcommand,
        usage,
        legacy_process,
        launcher: Some(program),
        modes,
    }
}

/// The path of a sub-command's captured legacy usage text, relative to
/// the repository root.
pub fn usage_spec_path(entry: &CommandEntry) -> String {
    format!("spec/usage/{}/{}.txt", entry.program, entry.subcommand)
}
