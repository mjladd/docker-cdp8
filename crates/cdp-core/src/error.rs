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

//! The legacy exit-status categories and the message shape each one
//! prints. legacy: `legacy/dev/include/globcon.h`'s `FINISHED`/
//! `GOAL_FAILED`/`USER_ERROR`/`DATA_ERROR`/`MEMORY_ERROR`/
//! `SYSTEM_ERROR`/`PROGRAM_ERROR`/`USAGE_ONLY` constants, and
//! `legacy/dev/cdp2k/mainfuncs.c`'s `print_messages_and_close_sndfiles`,
//! which switches on them.
//!
//! Every one of the 27 legacy `main()` functions returns only
//! `FAILED` (`-1`) or `SUCCEEDED`/`FINISHED` (`0`) as its own process
//! exit code -- confirmed by reading every `legacy/dev/*/main.c` (all
//! `return` statements are one of those two, never a category value
//! directly) and live: `docker run cdp8-postmerge modify loudness 1
//! nonexistent.wav out.wav 0.5` exits `255` (`-1` as an 8-bit unix
//! exit code) regardless of which negative category triggered it. So
//! [`ExitCategory`] only controls what gets *printed*, not the actual
//! process exit code, which [`report_and_exit`] always makes `0` or
//! `1` (Rust's `process::exit` already truncates to 8 bits, matching
//! legacy's `255`).
//!
//! What differs per category, confirmed live via the `cdp8-postmerge`
//! Docker image (see `crate::error::CdpError`'s `From` impls for the
//! specific commands): [`ExitCategory::UsageOnly`] prints its message
//! text as-is, with no header line at all
//! (`print_messages_and_close_sndfiles`'s `case(USAGE_ONLY):` does
//! `fprintf(stdout,"%s",errstr)` verbatim, non-sloom); every other
//! category prints a fixed header line (`"ERROR: <category text>"`),
//! then the message with `"ERROR: "` in front of every line
//! (`splice_multiline_string(errstr,"ERROR:")`, confirmed by reading
//! `legacy/dev/cdp2k/writedata.c`: it re-prints each `'\n'`-terminated
//! segment of `errstr` as `"{prefix} {segment}\n"`). Both shapes are
//! followed, unconditionally, by one more blank line
//! (`fprintf(stdout,"\n\n")`, the non-sloom/unix branch of
//! `print_messages_and_close_sndfiles`'s own tail) -- confirmed live
//! byte-for-byte with `cat -A` on both a failing and a succeeding run.

use std::io::{self, Write};

/// legacy: one of `globcon.h`'s `-1` to `-8` category constants
/// (`FINISHED`/`0` has no [`CdpError`], so it is not a variant here).
/// `ProgramError`, `SystemError`, `MemoryError` and `GoalFailed` are
/// not produced by any conversion below yet -- no command ported so
/// far triggers them -- but are included because
/// `print_messages_and_close_sndfiles`'s header text for them is
/// already known from `legacy/dev/cdp2k/mainfuncs.c` and a future
/// program WP will need them without revisiting this module.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCategory {
    ProgramError,
    SystemError,
    MemoryError,
    UserError,
    DataError,
    GoalFailed,
    /// legacy: `USAGE_ONLY`. Confirmed live to cover every
    /// [`cdp_params::ParamsError`] variant that comes from
    /// `legacy/dev/cdp2k/readdata.c`'s `get_option_no` and the
    /// cmdline-word-count checks in `read_parameters_and_flags`/
    /// `mainfuncs.c` -- see `CdpError`'s `From<ParamsError>` impl for
    /// the exact list and the live command confirming each one.
    UsageOnly,
}

impl ExitCategory {
    /// The header line legacy prints ahead of the message, or `None`
    /// for [`Self::UsageOnly`] (which has no header at all -- see the
    /// module doc). legacy: the non-sloom text in each
    /// `print_messages_and_close_sndfiles` `case`.
    fn header(self) -> Option<&'static str> {
        match self {
            ExitCategory::ProgramError => Some("INTERNAL ERROR: (Bug?)"),
            ExitCategory::SystemError => Some("SYSTEM ERROR"),
            ExitCategory::MemoryError => Some("MEMORY ERROR"),
            ExitCategory::UserError => Some("INCORRECT USE"),
            ExitCategory::DataError => Some("INVALID DATA"),
            ExitCategory::GoalFailed => Some("CANNOT ACHIEVE TASK:"),
            ExitCategory::UsageOnly => None,
        }
    }
}

/// A command failure with the legacy category that decides how it is
/// printed. legacy: `errstr` (the message) plus whichever negative
/// `exit_status` a `dz`-taking function returned.
#[derive(Debug)]
pub struct CdpError {
    pub category: ExitCategory,
    pub message: String,
}

impl CdpError {
    pub fn new(category: ExitCategory, message: impl Into<String>) -> Self {
        CdpError {
            category,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for CdpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CdpError {}

/// legacy: every category this crate's `From` impls produce is
/// confirmed against a live `legacy` run reading
/// `legacy/dev/cdp2k/readdata.c` for the returned category alongside
/// it (see each arm). `get_option_no` and `get_variant_no` both
/// produce a `"Unknown parameter '%s'"` non-dash-token check, with two
/// different categories (`USAGE_ONLY`/`USER_ERROR` respectively) --
/// `cdp_params::ParamsError::UnknownParameter` (the former) and
/// `::UnknownParameterInVariantPhase` (the latter, first confirmed
/// live and made reachable by `sndinfo smptime`/`timesmp`, WP-2.1's
/// third slice) keep these apart for exactly this reason, closing a
/// gap this doc used to note as open.
impl From<cdp_params::ParamsError> for CdpError {
    fn from(err: cdp_params::ParamsError) -> Self {
        use cdp_params::ParamsError as E;
        let category = match &err {
            // legacy: `read_parameters_and_flags`'s and `mainfuncs.c`'s
            // own cmdline-word-count pre-checks, and `get_option_no`'s
            // checks -- all `USAGE_ONLY`. Confirmed live: `modify
            // loudness 1 infile.wav` (InsufficientCmdlineParameters),
            // `modify loudness 1 infile.wav outfile.wav`
            // (InsufficientParameters), `modify loudness 1 infile.wav
            // outfile.wav 0.5 extra` (TooManyParameters), `modify
            // loudness 3 infile.wav outfile.wav -x0.5` (UnknownFlag),
            // `pvoc anal 1 infile.wav outfile.ana -c512 -c99999`
            // (DuplicateOption), `modify loudness 3 infile.wav
            // outfile.wav -l` (OptionValueMissing), `modify loudness 3
            // infile.wav outfile.wav 0.5` (UnknownParameter): none of
            // these print an "ERROR: INCORRECT USE" header.
            // legacy: `get_mode_from_cmdline` in `tklib3.c` -- also
            // `USAGE_ONLY`, confirmed live (`synth wave abc
            // outfile.wav 44100 1 1.0 440` and `synth wave 9
            // outfile.wav 44100 1 1.0 440`, neither prints a header).
            E::InsufficientCmdlineParameters
            | E::InsufficientParameters
            | E::TooManyParameters
            | E::UnknownFlag(_)
            | E::DuplicateOption(_)
            | E::OptionValueMissing(_)
            | E::UnknownParameter(_)
            | E::CannotReadModeOfProgram
            | E::ModeOutOfRange { .. } => ExitCategory::UsageOnly,
            // legacy: `get_params`'s mid-loop check and
            // `read_param_as_value_or_brkfile_and_check_range`/
            // `out_of_range`, both in `readdata.c` -- `USER_ERROR`.
            // Confirmed live: `synth wave 1 outfile.wav 44100`
            // (InsufficientParametersOnCmdline), `modify loudness 1
            // infile.wav outfile.wav 99999` (ValueOutOfRange), `modify
            // loudness 3 infile.wav outfile.wav -labc`
            // (CannotReadParameter): all three print "ERROR: INCORRECT
            // USE" ahead of the message.
            E::InsufficientParametersOnCmdline
            | E::ValueOutOfRange { .. }
            | E::CannotReadParameter { .. } => ExitCategory::UserError,
            // legacy: `get_variant_no`'s own checks (a variant flag's
            // missing value, a duplicate, an option letter used after
            // variant scanning started, an unrecognised variant
            // letter) -- also `USER_ERROR`, but from a different
            // function than the arm above; kept as its own arm so a
            // future refactor does not silently merge two facts this
            // module doc keeps separate. Confirmed live: `distort
            // repeat infile.wav outfile.wav 3 -c2 -s`
            // (VariantValueMissing), `synth wave 1 outfile.wav 44100 1
            // 1.0 440 -f -f` (DuplicateFlag), `distort repeat
            // infile.wav outfile.wav 3 -s1 -c2` (OptionOutOfOrder),
            // `distort repeat infile.wav outfile.wav 3 -c2 -z5`
            // (UnknownVariantFlag).
            // legacy: `get_variant_no`'s own non-dash check --
            // `USER_ERROR`, confirmed live: `sndinfo smptime infile.wav
            // 44174 extra`.
            E::UnknownParameterInVariantPhase(_) => ExitCategory::UserError,
            // legacy: `get_variant_no`'s `else` branch, taken when the
            // mode has variants but no options at all -- `USER_ERROR`,
            // confirmed live: `sndinfo smptime infile.wav 44174 -z`.
            E::UnknownFlagNoOptions(_) => ExitCategory::UserError,
            E::VariantValueMissing(_)
            | E::DuplicateFlag(_)
            | E::OptionOutOfOrder(_)
            | E::UnknownVariantFlag(_) => ExitCategory::UserError,
            // legacy: the generic infile-open check -- `DATA_ERROR`.
            // Confirmed live: `modify loudness 1 nonexistent.wav
            // out.wav 0.5` prints "ERROR: INVALID DATA" then "ERROR:
            // Can't open file nonexistent.wav to read data."
            E::CannotOpenFile { .. } => ExitCategory::DataError,
            // legacy: every check inside
            // `get_brkpnt_data_from_file_and_test_it` (open failure,
            // non-increasing times, an out-of-range point, no data at
            // all, unpaired data) returns `DATA_ERROR` -- confirmed by
            // reading `legacy/dev/cdp2k/readdata.c` end to end (every
            // `return` in that function is `DATA_ERROR` or
            // `MEMORY_ERROR`, and `cdp_data::DataError` does not model
            // the memory-allocation-failure paths, which cannot happen
            // in this port).
            E::Breakpoint(_) => ExitCategory::DataError,
        };
        CdpError::new(category, err.to_string())
    }
}

/// legacy: every `DATA_ERROR` return inside
/// `get_brkpnt_data_from_file_and_test_it` -- see the
/// `From<ParamsError>` impl's last arm. Used directly (not only
/// through [`cdp_params::ParamsError::Breakpoint`]) once a program
/// reads a breakpoint file outside of `cdp-params`'s own argument
/// parsing (e.g. a special data file).
impl From<cdp_data::DataError> for CdpError {
    fn from(err: cdp_data::DataError) -> Self {
        CdpError::new(ExitCategory::DataError, err.to_string())
    }
}

/// legacy: `globcon.h`'s own comment on `SYSTEM_ERROR` --
/// "failure to write to or truncate outfile: usually means H/D is
/// full". No live legacy run confirms this mapping (there is no easy
/// way to force a real disk-full error through Docker), but it is the
/// only category `globcon.h` documents for an output-file I/O
/// failure, and every [`cdp_sf::SfError`] this crate's own programs
/// can produce from a *write* path is exactly that: an `io::Error`
/// from [`cdp_sf::SoundFileWriter::finalize`].
impl From<cdp_sf::SfError> for CdpError {
    fn from(err: cdp_sf::SfError) -> Self {
        CdpError::new(ExitCategory::SystemError, err.to_string())
    }
}

/// Prints `result` in the legacy shape (see the module doc) and exits
/// the process. legacy: the tail of every `main()`, from the first
/// `print_messages_and_close_sndfiles` call to the final `return`.
/// Never returns, matching every call site in `legacy/dev/*/main.c`
/// having nothing left to do afterwards.
pub fn report_and_exit(result: Result<(), CdpError>) -> ! {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let ok = result.is_ok();
    if let Err(err) = result {
        match err.category.header() {
            Some(header) => {
                let _ = writeln!(out, "ERROR: {header}");
                for line in err.message.lines() {
                    let _ = writeln!(out, "ERROR: {line}");
                }
            }
            None => {
                for line in err.message.lines() {
                    let _ = writeln!(out, "{line}");
                }
            }
        }
    }
    // legacy: `print_messages_and_close_sndfiles`'s unconditional,
    // unix-only `fprintf(stdout,"\n\n")` tail, printed whether the run
    // succeeded or failed.
    let _ = write!(out, "\n\n");
    let _ = out.flush();
    // legacy: `return(FAILED)` (`FAILED` is `-1`, `globcon.h`/
    // `tkglobals.h`), which becomes exit code `255` on Linux/macOS --
    // confirmed live (`docker run cdp8-postmerge modify loudness 1
    // nonexistent.wav out.wav 0.5` exits `255`). `-1`, not `1`,
    // reproduces that exactly the same way the C `return` does.
    std::process::exit(if ok { 0 } else { -1 });
}
