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

//! Error type for `cdp-data`.
//!
//! Message text matches `legacy/dev/cdp2k/readdata.c`'s
//! `sprintf(errstr, ...)` calls exactly, `%lf`/`%f` formatting
//! included (C's default is six decimal places), so golden tests
//! comparing error text against the legacy programs can match it
//! verbatim.

use std::io;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Can't open brkpntfile {path} to read data.")]
    CannotOpen { path: String, source: io::Error },

    /// legacy: `get_brkpnt_data_from_file_and_test_it`, the check
    /// `*p != dz->brk[paramno] && *p <= lasttime` -- times must
    /// strictly increase; two equal times are also rejected, not
    /// just a decrease.
    #[error(
        "Times ({lasttime:.6} & {newtime:.6}) in brkpntfile {path} are not in increasing order."
    )]
    TimesNotIncreasing {
        path: String,
        lasttime: f64,
        newtime: f64,
    },

    /// legacy: `out_of_range_in_brkfile`.
    #[error("Value ({value:.6}) out of range ({lo:.6} to {hi:.6}) in brkpntfile {path}.")]
    OutOfRange {
        path: String,
        value: f64,
        lo: f64,
        hi: f64,
    },

    #[error("No data in brkpnt file {0}")]
    NoData(String),

    #[error("Data not paired correctly in file {0}")]
    Unpaired(String),

    /// legacy: `convert_dB_at_or_below_zero_to_gain`.
    #[error("dB value out of range (> 0dB)")]
    DbAboveZero,

    #[error("empty breakpoint table")]
    EmptyTable,

    /// legacy: `store_wordlist`'s `fopen` failure in
    /// `legacy/dev/cdp2k/readfiles.c`, shared by every word-list
    /// format that goes through it (mix files; tuning files in a
    /// later slice) -- distinct wording from [`DataError::CannotOpen`]
    /// (breakpoint-file specific) and [`DataError::CannotOpenNotedataFile`]
    /// (texture note-data has its own `fopen` call site with its own
    /// message).
    #[error("Failed to open file {path} for input.")]
    CannotOpenDataFile { path: String, source: io::Error },

    /// legacy: `get_mixdata_in_line` in
    /// `legacy/dev/submix/setupmix.c` -- a mix-file line must have 4,
    /// 5 or 7 words (see [`crate::mix`]).
    #[error("Illegal line length: get_mixdata_in_line()")]
    IllegalMixLineLength,

    /// legacy: `get_mixdata_in_line`, the `sscanf` call for the time
    /// and chans fields.
    #[error("Error scanning data: get_mixdata_in_line()")]
    CannotScanMixTimeOrChans,

    #[error("Error1 scanning (chan1) level: line {line}")]
    LeftLevelNotDb { line: usize },
    #[error("Error2 scanning (chan1) level: line {line}")]
    LeftLevelUnparseable { line: usize },
    #[error("Error3 scanning (chan1) level: line {line}")]
    LeftLevelNegative { line: usize },
    #[error("Error1 scanning (chan1) pan: line {line}")]
    LeftPanUnparseable { line: usize },
    #[error("Error2 scanning (chan1) pan: line {line}")]
    LeftPanOutOfRange { line: usize },
    #[error("Error1 scanning chan2 level: line {line}")]
    RightLevelNotDb { line: usize },
    #[error("Error2 scanning chan2 level: line {line}")]
    RightLevelUnparseable { line: usize },
    #[error("Error3 scanning chan2 level: line {line}")]
    RightLevelNegative { line: usize },
    #[error("Error1 scanning chan2 pan: line {line}")]
    RightPanUnparseable { line: usize },
    #[error("Error2 scanning chan2 pan: line {line}")]
    RightPanOutOfRange { line: usize },

    /// legacy: `finalise_and_check_mixdata_in_line` in
    /// `setupmix.c` -- a 5-word line's `chans` must be `1`, and a
    /// 7-word line's `chans` must be `2`.
    #[error("Error parsing data: finalise_and_check_mixdata_in_line()")]
    MixChansLineLengthMismatch,

    /// Not a legacy message: `finalise_and_check_mixdata_in_line`
    /// has no `default` arm for a 4-word line whose `chans` is
    /// neither `1` nor `2`, so the real C code leaves the pan and
    /// right-channel level fields uninitialised rather than
    /// reporting an error. See `docs/migration/LEGACY-BUGS.md`.
    #[error(
        "mix line {line}: a 4-word line's chans must be 1 or 2, found {chans} (legacy leaves pan and right-channel level undefined for any other value)"
    )]
    MinLineChansMustBeMonoOrStereo { line: usize, chans: i32 },

    /// legacy: the `filecnt==0` half of `set_up_mix`'s "No mixfile
    /// line is active..." check in `setupmix.c`, which also covers a
    /// mixfile with no data lines at all once no `MIX_START`/
    /// `MIX_END` windowing is applied (see [`crate::mix`]).
    #[error("No mixfile line is active within the time limits specified.")]
    NoMixData,

    /// legacy: `get_the_notedatafile` in
    /// `legacy/dev/texture/ap_texture.c`.
    #[error("Failed to open notedata file {path}")]
    CannotOpenNotedataFile { path: String, source: io::Error },

    /// legacy: `get_the_notedata` in
    /// `legacy/dev/texture/texprepro.c`, wrapping any failure from
    /// `get_sample_pitches` (missing first line, or a first
    /// non-comment line with fewer pitch values than there are input
    /// sound files) into this one message, regardless of the
    /// specific cause.
    #[error("Insufficient pitch values in notedata file.")]
    NotedataInsufficientPitches,

    /// legacy: `get_motifs`, a motif's header line must start with
    /// `#` (`TEXTURE_SEPARATOR`) once leading whitespace is skipped.
    #[error(
        "'#' missing before datacount in notedata file: motif {motifno} (or more notes listed than indicated by #N)\ncheck datalen is correct"
    )]
    NotedataMissingHash { motifno: usize },

    /// legacy: `get_motifs`, the character right after `#` must be a
    /// digit.
    #[error("No datalength given: motif {motifno}")]
    NotedataNoDatalength { motifno: usize },

    /// legacy: `get_motifs`, a `#N` datalength of zero or less.
    #[error("Invalid data length {datalen} in notedata: motif {motifno}")]
    NotedataInvalidDatalen { datalen: i32, motifno: usize },

    /// legacy: `read_a_note_from_notedata_file`, `fgets` returning
    /// `NULL` (end of file) while a motif still expects more notes.
    #[error("Note data line for note {noteno}, motif {motifno} missing in notedatafile")]
    NotedataMissingNoteLine { noteno: usize, motifno: usize },

    #[error("No time data for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoTimeData { noteno: usize, motifno: usize },
    #[error("No data after time for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoDataAfterTime { noteno: usize, motifno: usize },
    #[error("No instr_no for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoInstrNo { noteno: usize, motifno: usize },
    #[error("No data after instr_no for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoDataAfterInstrNo { noteno: usize, motifno: usize },
    #[error("No pitch data for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoPitchData { noteno: usize, motifno: usize },
    #[error("No data after pitch for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoDataAfterPitch { noteno: usize, motifno: usize },
    #[error("No amplitude data for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoAmpData { noteno: usize, motifno: usize },
    #[error("No data after amp for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoDataAfterAmp { noteno: usize, motifno: usize },
    #[error("No duration data for note {noteno}, motif {motifno} in notedatafile")]
    NotedataNoDurationData { noteno: usize, motifno: usize },

    /// legacy: `read_a_note_from_notedata_file`, `noteno > 1 &&
    /// *lasttime > thisnote->ntime`. `prev_noteno` is always `noteno
    /// - 1`, matching the legacy message, which names both notes.
    #[error(
        "Notes in reverse time order: notedata file : motif {motifno}: notes {noteno} & {prev_noteno}"
    )]
    NotedataReverseTimeOrder {
        motifno: usize,
        noteno: usize,
        prev_noteno: usize,
    },

    /// legacy: `get_the_notedata`, the `IS_ORN_OR_MTF` ("at least")
    /// branch of the motif-count check.
    #[error("Insufficient motifs in notedata file.")]
    NotedataInsufficientMotifs,

    /// legacy: `get_the_notedata`, the exact-count branch of the
    /// motif-count check.
    #[error("Incorrect number [{motifcnt}] of motifs in notedata file (expected {expected}).")]
    NotedataIncorrectMotifCount { motifcnt: usize, expected: usize },
}

pub type Result<T> = std::result::Result<T, DataError>;
