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

//! CDP mix files: one line per sound-file event in a `submix mix`
//! mix. legacy: `legacy/dev/submix/setupmix.c` (`set_up_mix`,
//! `get_mixdata_in_line`, `finalise_and_check_mixdata_in_line`, and
//! the four `check_*` field scanners), confirmed against the format
//! documentation printed by `legacy` `submix fileformat`:
//!
//! ```text
//! sndname starttime_in_mix  chans  level
//! sndname starttime_in_mix  1      level       pan
//! sndname starttime_in_mix  2      left_level  left_pan  right_level  right_pan
//! ```
//!
//! Each line is whitespace-split (legacy: `get_word_from_string`, no
//! quoting) into 4, 5 or 7 words -- filename, time, chans, and then
//! either nothing more, a mono level and pan, or a stereo pair of
//! level and pan. A line starting with `;` (after leading whitespace)
//! is a whole-line comment; blank lines are ignored; unlike
//! [`crate::breakpoint`] files, there is no per-token comment
//! stripping, so a `;` after real data on a line is just another
//! word, which almost always trips the line-length check.
//!
//! This module implements the mix-file *grammar*: given text already
//! known to be a mix file, it parses and validates each line's
//! fields, exactly as `setupmix.c` does, including its dB-aware level
//! syntax (`0.5` or `-6dB`) and `L`/`C`/`R` pan shorthand. It does not
//! implement the surrounding pieces of `set_up_mix`: opening the
//! referenced sound files, `-s`/`-e` start/end time windowing, buffer
//! allocation, or computing the actual per-channel gain scale factors
//! fed to the mixing engine (`d_assign_scaling`). Those belong to the
//! `submix` program itself (WP-2.7), which also does the file-type
//! auto-detection that decides a given command-line argument *is* a
//! mix file before ever calling into this grammar -- confirmed with
//! `legacy` `submix mix`: a structurally malformed mixfile is
//! rejected earlier, by that auto-detection layer, with a generic
//! "Application doesn't work with this type of infile" error rather
//! than reaching any of the specific messages below. Because of that
//! earlier layer, the field-level error messages here (ported
//! directly from `setupmix.c`) could not be independently
//! reproduced from a live erroring run the way the successful-parse
//! behaviour below was; they are still the literal C source strings.
//!
//! Verified against `legacy` `submix mix` with real files from
//! `docs/manual/sounds`: a plain run of `simplemix.mix` (an example
//! file already in `docs/manual/data`, using its mono `capm.wav`,
//! `bfrogcdtg.wav` and `clashmx.wav`) mixes without error; a
//! generated stereo file (`legacy` `synth wave 1 ... 44100 2 1 440`)
//! mixed with a 4-word line `name 0.0 2 0.5` produces a stereo file
//! with both channels at exactly the input level (`sndinfo props`
//! shows `-6.02 dB` on both channels for a `0.5` input level),
//! confirming the [`MixEvent`] four-word stereo derivation
//! (`right_level = left_level`, hard left/right pan, i.e. no
//! panning-induced attenuation); and a line with a scientific-notation
//! time field (`1e-1`) mixes without error, confirming that -- unlike
//! a breakpoint file's custom tokenizer -- mix-file numeric fields
//! are scanned with plain `sscanf`, which accepts it.

use crate::error::{DataError, Result};
use crate::tokenizer::{
    is_comment_or_blank_line, scan_c_double_prefix, scan_c_int_prefix, split_words,
};
use std::path::Path;

/// legacy: `MIX_MINLINE`/`MIX_MIDLINE`/`MIX_MAXLINE` and the
/// `MIX_*POS` word-index constants in
/// `legacy/dev/include/mixxcon.h`.
const MIX_TIMEPOS: usize = 1;
const MIX_CHANPOS: usize = 2;
const MIX_LEVELPOS: usize = 3;
const MIX_PANPOS: usize = 4;
const MIX_RLEVELPOS: usize = 5;
const MIX_RPANPOS: usize = 6;
const MIX_MINLINE: usize = 4;
const MIX_MIDLINE: usize = 5;
const MIX_MAXLINE: usize = 7;

/// legacy: `MINPAN`/`MAXPAN` in `mixxcon.h`. A vestige of a 16-bit
/// integer pan-scale representation; the practical range used by real
/// mixfiles is `-1.0` to `1.0` (values beyond that pan "past" hard
/// left/right with attenuation, per `legacy` `submix fileformat`),
/// but this is the range legacy actually range-checks against.
const MINPAN: f64 = -32767.0;
const MAXPAN: f64 = 32767.0;

/// legacy: `MIN_DB_ON_16_BIT`/`MAX_DB_ON_16_BIT` clamp, `dbtogain`
/// math -- `get_leveldb` in `legacy/dev/cdp2k/tklib1.c`.
fn parse_db_level(word: &str) -> Option<f64> {
    let mut db = scan_c_double_prefix(word)?;
    db = db.max(crate::breakpoint::MIN_DB_ON_16_BIT);
    db = db.min(crate::breakpoint::MAX_DB_ON_16_BIT);
    if crate::tokenizer::flteq(db, 0.0) {
        return Some(1.0);
    }
    let is_neg = db < 0.0;
    let mut gain = 10f64.powf(db.abs() / 20.0);
    if is_neg {
        gain = 1.0 / gain;
    }
    Some(gain)
}

/// legacy: `is_dB` in `tklib1.c` -- the *last two characters* of the
/// word, nothing more, so `"6dB"`, `"-6DB"` and `"0db"` all count,
/// but a shorter word never does.
fn is_db_suffixed(word: &str) -> bool {
    word.len() >= 2 && matches!(&word[word.len() - 2..], "dB" | "DB" | "db")
}

enum LevelScan {
    Value(f64),
    NotDb,
    Unparseable,
}

/// legacy: `check_left_level`/`check_right_level` share this logic in
/// `setupmix.c`; only the wording of the reported error differs by
/// channel.
fn scan_level(word: &str) -> LevelScan {
    if is_db_suffixed(word) {
        match parse_db_level(word) {
            Some(v) => LevelScan::Value(v),
            None => LevelScan::NotDb,
        }
    } else {
        match scan_c_double_prefix(word) {
            Some(v) => LevelScan::Value(v),
            None => LevelScan::Unparseable,
        }
    }
}

/// legacy: `check_left_level` in `setupmix.c`.
fn check_left_level(word: &str, line: usize) -> Result<f64> {
    let v = match scan_level(word) {
        LevelScan::Value(v) => v,
        LevelScan::NotDb => return Err(DataError::LeftLevelNotDb { line }),
        LevelScan::Unparseable => return Err(DataError::LeftLevelUnparseable { line }),
    };
    if v < 0.0 {
        return Err(DataError::LeftLevelNegative { line });
    }
    Ok(v)
}

/// legacy: `check_right_level` in `setupmix.c`.
fn check_right_level(word: &str, line: usize) -> Result<f64> {
    let v = match scan_level(word) {
        LevelScan::Value(v) => v,
        LevelScan::NotDb => return Err(DataError::RightLevelNotDb { line }),
        LevelScan::Unparseable => return Err(DataError::RightLevelUnparseable { line }),
    };
    if v < 0.0 {
        return Err(DataError::RightLevelNegative { line });
    }
    Ok(v)
}

/// legacy: `check_left_pan`/`check_right_pan` share the `L`/`R`
/// shorthand, but disagree on what `C` means (see
/// [`check_left_pan`]'s doc) -- `centre` carries that difference in.
fn scan_pan_letter(word: &str, centre: f64) -> Option<f64> {
    match word.as_bytes().first() {
        Some(b'L') => Some(-1.0),
        Some(b'R') => Some(1.0),
        Some(b'C') => Some(centre),
        _ => None,
    }
}

/// legacy: `check_left_pan` in `setupmix.c`. `C` means dead centre,
/// `0.0`.
fn check_left_pan(word: &str, line: usize) -> Result<f64> {
    if let Some(p) = scan_pan_letter(word, 0.0) {
        return Ok(p);
    }
    let p = scan_c_double_prefix(word).ok_or(DataError::LeftPanUnparseable { line })?;
    if !(MINPAN..=MAXPAN).contains(&p) {
        return Err(DataError::LeftPanOutOfRange { line });
    }
    Ok(p)
}

/// legacy: `check_right_pan` in `setupmix.c`. `C` means `0.5`, not
/// `0.0` -- ported as-is; this asymmetry with [`check_left_pan`] is
/// in the original source, not a transcription slip.
fn check_right_pan(word: &str, line: usize) -> Result<f64> {
    if let Some(p) = scan_pan_letter(word, 0.5) {
        return Ok(p);
    }
    let p = scan_c_double_prefix(word).ok_or(DataError::RightPanUnparseable { line })?;
    if !(MINPAN..=MAXPAN).contains(&p) {
        return Err(DataError::RightPanOutOfRange { line });
    }
    Ok(p)
}

/// One parsed, validated mix-file line. legacy: the fields
/// `set_up_mix` extracts per line via `get_mixdata_in_line` and
/// `finalise_and_check_mixdata_in_line` in `setupmix.c`.
///
/// `right_level` and `right_pan` are [`f64::NAN`] for a mono event (a
/// line whose effective `chans` is `1`): legacy never reads them in
/// that case (`assign_stereo_sense`'s `MONO` branch only looks at
/// `lpan`), so there is no real value to report, and `NaN` marks that
/// plainly rather than defaulting to `0.0`, which would look like a
/// meaningful centre pan.
#[derive(Debug, Clone, PartialEq)]
pub struct MixEvent {
    pub filename: String,
    pub time: f64,
    pub chans: i32,
    pub left_level: f64,
    pub left_pan: f64,
    pub right_level: f64,
    pub right_pan: f64,
}

impl MixEvent {
    /// legacy: `get_mixdata_in_line` followed by
    /// `finalise_and_check_mixdata_in_line`, both in `setupmix.c`.
    /// `words` is one already comment/blank-filtered line, already
    /// split by [`split_words`]; `line` is the 1-based count of such
    /// lines seen so far (legacy: `filecnt+1` -- see the module doc
    /// on why that coincides with the plain line count here, since
    /// this parser does not implement `MIX_START`/`MIX_END`
    /// windowing).
    fn parse_line(words: &[&str], line: usize) -> Result<MixEvent> {
        let filename = words[0].to_string();
        let mut left_pan = f64::NAN;
        let mut right_level = f64::NAN;
        let mut right_pan = f64::NAN;

        match words.len() {
            MIX_MAXLINE => {
                right_level = check_right_level(words[MIX_RLEVELPOS], line)?;
                right_pan = check_right_pan(words[MIX_RPANPOS], line)?;
                left_pan = check_left_pan(words[MIX_PANPOS], line)?;
            }
            MIX_MIDLINE => {
                left_pan = check_left_pan(words[MIX_PANPOS], line)?;
            }
            MIX_MINLINE => {}
            _ => return Err(DataError::IllegalMixLineLength),
        }

        let time =
            scan_c_double_prefix(words[MIX_TIMEPOS]).ok_or(DataError::CannotScanMixTimeOrChans)?;
        let chans =
            scan_c_int_prefix(words[MIX_CHANPOS]).ok_or(DataError::CannotScanMixTimeOrChans)?;
        let left_level = check_left_level(words[MIX_LEVELPOS], line)?;

        match words.len() {
            MIX_MINLINE => match chans {
                1 => left_pan = 0.0,
                2 => {
                    right_level = left_level;
                    left_pan = -1.0;
                    right_pan = 1.0;
                }
                _ => {
                    return Err(DataError::MinLineChansMustBeMonoOrStereo { line, chans });
                }
            },
            MIX_MIDLINE if chans != 1 => return Err(DataError::MixChansLineLengthMismatch),
            MIX_MAXLINE if chans != 2 => return Err(DataError::MixChansLineLengthMismatch),
            _ => {}
        }

        Ok(MixEvent {
            filename,
            time,
            chans,
            left_level,
            left_pan,
            right_level,
            right_pan,
        })
    }
}

/// A parsed mix file: every non-comment, non-blank line as a
/// [`MixEvent`], in file order (legacy: mix files need not be in
/// starttime order, per `legacy` `submix fileformat`, so this does
/// not sort them).
#[derive(Debug, Clone, PartialEq)]
pub struct MixFile {
    events: Vec<MixEvent>,
}

impl MixFile {
    /// legacy: `store_wordlist` (line splitting) followed by
    /// `set_up_mix`'s per-line loop (field parsing), minus the
    /// pieces listed in the module doc as out of scope.
    pub fn parse(text: &str) -> Result<Self> {
        let mut events = Vec::new();
        let mut line = 0usize;
        for raw_line in text.lines() {
            if is_comment_or_blank_line(raw_line) {
                continue;
            }
            line += 1;
            let words = split_words(raw_line);
            events.push(MixEvent::parse_line(&words, line)?);
        }
        if events.is_empty() {
            return Err(DataError::NoMixData);
        }
        Ok(MixFile { events })
    }

    /// As [`Self::parse`], reading `path` first. legacy: the
    /// `fopen`/`"Failed to open file %s for input."` half of
    /// `store_wordlist` in `legacy/dev/cdp2k/readfiles.c`.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let text =
            std::fs::read_to_string(path).map_err(|source| DataError::CannotOpenDataFile {
                path: path.display().to_string(),
                source,
            })?;
        Self::parse(&text)
    }

    pub fn events(&self) -> &[MixEvent] {
        &self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_real_simplemix_example_file() {
        // docs/manual/data/simplemix.mix, this repository's own
        // example file, verified end to end against `legacy` `submix
        // mix` with its referenced `docs/manual/sounds` files.
        let text = "capm.wav       0.0   1  0.5   C\n\
                     bfrogcdtg.wav  2.0   1  1.0  -1\n\
                     bfrogcdtg.wav  2.25  1  1.0   1\n\
                     clashmx.wav    6.5   1  1.0  -0.5\n\
                     clashmx.wav    6.7   1  1.0   0.5\n";
        let mix = MixFile::parse(text).unwrap();
        assert_eq!(mix.len(), 5);
        assert_eq!(mix.events()[0].filename, "capm.wav");
        assert_eq!(mix.events()[0].left_pan, 0.0); // 'C' -> centre
        assert_eq!(mix.events()[1].left_pan, -1.0); // plain -1 pan
    }

    #[test]
    fn four_word_stereo_line_derives_the_scaling_verified_against_legacy() {
        // Verified against `legacy` `submix mix`: a generated stereo
        // file mixed with `name 0.0 2 0.5` comes out with both
        // channels at exactly -6.02 dB (i.e. gain 0.5, no panning
        // attenuation), confirming this exact derivation.
        let mix = MixFile::parse("stereo_test.wav 0.0 2 0.5\n").unwrap();
        let ev = &mix.events()[0];
        assert_eq!(ev.left_level, 0.5);
        assert_eq!(ev.right_level, 0.5);
        assert_eq!(ev.left_pan, -1.0);
        assert_eq!(ev.right_pan, 1.0);
    }

    #[test]
    fn four_word_mono_line_leaves_right_fields_as_nan() {
        let mix = MixFile::parse("capm.wav 0.0 1 0.5\n").unwrap();
        let ev = &mix.events()[0];
        assert_eq!(ev.left_pan, 0.0);
        assert!(ev.right_level.is_nan());
        assert!(ev.right_pan.is_nan());
    }

    #[test]
    fn seven_word_stereo_line_parses_all_four_level_and_pan_fields() {
        let mix = MixFile::parse("name.wav 0.0 2 0.8 -1.0 0.8 -0.5\n").unwrap();
        let ev = &mix.events()[0];
        assert_eq!(ev.left_level, 0.8);
        assert_eq!(ev.left_pan, -1.0);
        assert_eq!(ev.right_level, 0.8);
        assert_eq!(ev.right_pan, -0.5);
    }

    #[test]
    fn db_suffixed_level_converts_to_gain() {
        let mix = MixFile::parse("name.wav 0.0 1 -6dB C\n").unwrap();
        let gain = mix.events()[0].left_level;
        // -6dB is close to a gain of 0.5011872336...
        assert!((gain - 0.501_187_233_6).abs() < 1e-9);
    }

    #[test]
    fn right_pan_c_means_half_not_centre() {
        // legacy quirk, ported as-is: see check_right_pan's doc.
        let mix = MixFile::parse("name.wav 0.0 2 0.5 C 0.5 C\n").unwrap();
        assert_eq!(mix.events()[0].left_pan, 0.0);
        assert_eq!(mix.events()[0].right_pan, 0.5);
    }

    #[test]
    fn comment_and_blank_lines_are_ignored_and_do_not_count_towards_line_numbers() {
        let mix =
            MixFile::parse(";a comment\n\ncapm.wav 0.0 1 0.5 C\n;another\ncapm.wav 1.0 1 xdB C\n")
                .unwrap_err();
        // the error is for the second data line ("line 2"), not the
        // fourth physical line.
        assert!(matches!(mix, DataError::LeftLevelNotDb { line: 2 }));
    }

    #[test]
    fn trailing_word_makes_the_line_illegal() {
        let err = MixFile::parse("name.wav 0.0 1 0.5 C extra\n").unwrap_err();
        assert!(matches!(err, DataError::IllegalMixLineLength));
    }

    #[test]
    fn unscannable_time_is_an_error() {
        let err = MixFile::parse("name.wav abc 1 0.5 C\n").unwrap_err();
        assert!(matches!(err, DataError::CannotScanMixTimeOrChans));
    }

    #[test]
    fn scientific_notation_time_is_accepted_verified_against_legacy() {
        // Verified against `legacy` `submix mix`: a "1e-1" time field
        // mixes without error, because setupmix.c scans it with raw
        // sscanf("%lf", ...), not the breakpoint-file tokenizer.
        let mix = MixFile::parse("capm.wav 1e-1 1 0.5 C\n").unwrap();
        assert!((mix.events()[0].time - 0.1).abs() < 1e-12);
    }

    #[test]
    fn negative_level_is_an_error() {
        let err = MixFile::parse("name.wav 0.0 1 -0.5 C\n").unwrap_err();
        assert!(matches!(err, DataError::LeftLevelNegative { line: 1 }));
    }

    #[test]
    fn pan_out_of_range_is_an_error() {
        let err = MixFile::parse("name.wav 0.0 1 0.5 99999\n").unwrap_err();
        assert!(matches!(err, DataError::LeftPanOutOfRange { line: 1 }));
    }

    #[test]
    fn mono_pan_at_maxpan_boundary_is_accepted() {
        let mix = MixFile::parse("name.wav 0.0 1 0.5 32767\n").unwrap();
        assert_eq!(mix.events()[0].left_pan, 32767.0);
    }

    #[test]
    fn five_word_line_with_chans_2_is_a_mismatch() {
        let err = MixFile::parse("name.wav 0.0 2 0.5 C\n").unwrap_err();
        assert!(matches!(err, DataError::MixChansLineLengthMismatch));
    }

    #[test]
    fn seven_word_line_with_chans_1_is_a_mismatch() {
        let err = MixFile::parse("name.wav 0.0 1 0.5 C 0.5 C\n").unwrap_err();
        assert!(matches!(err, DataError::MixChansLineLengthMismatch));
    }

    #[test]
    fn four_word_line_with_invalid_chans_is_a_deviation_error_not_ub() {
        // legacy leaves lpan/rlevel/rpan uninitialised here (see
        // docs/migration/LEGACY-BUGS.md); this port errors instead.
        let err = MixFile::parse("name.wav 0.0 3 0.5\n").unwrap_err();
        assert!(matches!(
            err,
            DataError::MinLineChansMustBeMonoOrStereo { line: 1, chans: 3 }
        ));
    }

    #[test]
    fn empty_file_is_an_error() {
        assert!(matches!(MixFile::parse(""), Err(DataError::NoMixData)));
        assert!(matches!(
            MixFile::parse(";only a comment\n"),
            Err(DataError::NoMixData)
        ));
    }
}
