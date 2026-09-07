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

//! CDP texture note-data files: the `notedata` input to every
//! `texture` sub-command. legacy: `legacy/dev/texture/texprepro.c`
//! (`get_the_notedata`, `get_sample_pitches`, `get_motifs`,
//! `read_a_note_from_notedata_file`), confirmed against the format
//! documentation in `legacy/dev/texture/ap_texture.c`'s usage text:
//!
//! ```text
//! assumed MIDI 'pitch' of each input snd, specified on 1st line.
//!
//! FOLLOWED BY, where ness, NOTELIST(S), SPECIFIED THUS:-
//!
//! #N (where N = no. of notes in notelist: follows by N lines of...)
//! time(SECS)   infile_no    pitch(MIDI)    amp(MIDI)   dur(SECS)
//! ```
//!
//! A file is: a sample-pitches line (one float per input sound file,
//! on the first line that is not a whole-line `;` comment -- a
//! comment here must start with `;` as the line's literal first
//! character, with no leading whitespace tolerated, unlike
//! [`crate::tokenizer::is_comment_or_blank_line`]), followed by zero
//! or more motifs. Each motif is a `#N` header line
//! (`N` the note count) followed by `N` note lines of five
//! whitespace-separated fields (`time instr_no pitch amp dur`); a
//! blank line between or within motifs is skipped without counting
//! against `N`, but -- unlike the sample-pitches line -- there is no
//! `;`-comment support at all here, confirmed against `legacy`
//! `texture`: a `;`-prefixed line where a note or header is expected
//! is parsed as data and fails.
//!
//! This module implements the notedata *grammar* only: it parses the
//! sample pitches and the motifs' note fields, and enforces the
//! checks that do not depend on which `texture` sub-command or mode
//! is asking (missing/malformed fields, blank-line skipping, and
//! strictly non-decreasing note times within a motif). It does not
//! decide how many motifs a given sub-command and mode need, since
//! that depends on `texture`'s own flag combination (timing line,
//! harmonic field, ornament/motif list); [`NoteDataFile::check_motif_count`]
//! exposes that one check, parameterised, for the `texture` program
//! itself (WP-3.15) to call once it knows the expected count. The
//! `instr_no` field is parsed and kept on [`Note`] for completeness,
//! but legacy itself never stores it (`read_a_note_from_notedata_file`
//! computes then discards it -- see the field's own doc) or range-
//! checks it.
//!
//! Verified against `legacy` `texture simple`/`texture motifs` (via
//! the `cdp8-postmerge` Docker image), which reaches notedata parsing
//! before any audio processing: every corpus file's structure --
//! `docs/manual/data/ndf62hs.txt` (1 pitch, 1 five-note harmonic-field
//! motif), `ndfPO1.txt` (4 pitches, two motifs separated by a blank
//! line), `tmotifsinhf.txt` (1 pitch, three motifs) -- parses and runs
//! to completion; and each error message below (insufficient pitches,
//! a motif header missing `#`, a non-digit datalength, a zero
//! datalength, a missing note line at end of file, an out-of-order
//! note time, each of the five per-field "missing"/"no data after"
//! cases, and both the exact-count and at-least-count motif-count
//! mismatches) was reproduced character-for-character from a live
//! erroring run with a small crafted notedata file.

use crate::error::{DataError, Result};
use crate::tokenizer::{is_space_char, next_float, scan_c_double_prefix, scan_c_int_prefix};
use std::path::Path;

/// One note in a motif's notelist. legacy: `struct nnote` in
/// `legacy/dev/include/structures.h`, restricted to the fields the
/// notedata file itself supplies (`spacepos` and `motioncentre` are
/// computed later, by the `texture` program).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Note {
    /// Seconds from the start of the motif (or, for the first motif
    /// of a timed texture, from the start of the timing set).
    pub time: f64,
    /// Parsed from the file's second field, but not range-checked
    /// and -- in the current `texture` source -- not stored anywhere
    /// once parsed: `read_a_note_from_notedata_file` computes
    /// `instr_no` into a local and never assigns it to
    /// `thisnote->instr`. Kept here for fidelity; a future `texture`
    /// WP can decide whether it has a real use.
    pub instr_no: f64,
    /// MIDI pitch (0-127; not enforced by this parser -- see the
    /// module doc).
    pub pitch: f64,
    /// MIDI-range amplitude (0-127; not enforced by this parser).
    pub amp: f64,
    /// Duration in seconds.
    pub dur: f64,
}

/// A parsed notedata file: the sample pitches (one per input sound
/// file) followed by zero or more motifs, each a list of notes in
/// file order.
#[derive(Debug, Clone, PartialEq)]
pub struct NoteDataFile {
    sample_pitches: Vec<f64>,
    motifs: Vec<Vec<Note>>,
}

impl NoteDataFile {
    /// Parses `text`. `infilecnt` is the number of input sound files
    /// given on the command line (external to the file itself; it is
    /// what tells legacy how many sample-pitch values to expect on
    /// the first line).
    pub fn parse(text: &str, infilecnt: usize) -> Result<Self> {
        let lines: Vec<&str> = text.lines().collect();
        let mut i = 0usize;
        let sample_pitches = parse_sample_pitches(&lines, &mut i, infilecnt)?;
        let motifs = parse_motifs(&lines, &mut i)?;
        Ok(NoteDataFile {
            sample_pitches,
            motifs,
        })
    }

    /// As [`Self::parse`], reading `path` first. legacy: the
    /// `fopen`/`"Failed to open notedata file %s"` half of
    /// `get_the_notedatafile` in `legacy/dev/texture/ap_texture.c`.
    pub fn from_file(path: impl AsRef<Path>, infilecnt: usize) -> Result<Self> {
        let path = path.as_ref();
        let text =
            std::fs::read_to_string(path).map_err(|source| DataError::CannotOpenNotedataFile {
                path: path.display().to_string(),
                source,
            })?;
        Self::parse(&text, infilecnt)
    }

    pub fn sample_pitches(&self) -> &[f64] {
        &self.sample_pitches
    }

    pub fn motifs(&self) -> &[Vec<Note>] {
        &self.motifs
    }

    /// legacy: `get_the_notedata`'s motif-count check, run by the
    /// caller once it knows how many motifs its `texture` sub-command
    /// and mode need. `at_least` is legacy's `IS_ORN_OR_MTF` branch
    /// (an ornament or motif list may be followed by more alternative
    /// lists than the minimum); otherwise the count must match
    /// exactly.
    pub fn check_motif_count(&self, expected: usize, at_least: bool) -> Result<()> {
        let motifcnt = self.motifs.len();
        if at_least {
            if motifcnt < expected {
                return Err(DataError::NotedataInsufficientMotifs);
            }
        } else if motifcnt != expected {
            return Err(DataError::NotedataIncorrectMotifCount { motifcnt, expected });
        }
        Ok(())
    }
}

/// legacy: `get_sample_pitches`. A comment here is a line whose
/// literal first character is `;` -- no leading whitespace is
/// skipped first, unlike every other comment check in this crate.
/// The first line that is not such a comment must supply at least
/// `infilecnt` values by itself: legacy never continues gathering
/// pitches onto a following line, so a line that runs out of valid
/// tokens first is an error even if more lines follow.
fn parse_sample_pitches(lines: &[&str], i: &mut usize, infilecnt: usize) -> Result<Vec<f64>> {
    if infilecnt == 0 {
        return Ok(Vec::new());
    }
    while *i < lines.len() {
        let line = lines[*i];
        *i += 1;
        if line.starts_with(';') {
            continue;
        }
        let mut values = Vec::with_capacity(infilecnt);
        let mut rest = line;
        while let Some((value, remainder)) = next_float(rest) {
            values.push(value);
            rest = remainder;
            if values.len() >= infilecnt {
                return Ok(values);
            }
        }
        return Err(DataError::NotedataInsufficientPitches);
    }
    Err(DataError::NotedataInsufficientPitches)
}

/// A line is blank, for the header- and note-line skipping below, if
/// it is empty once leading *and* trailing legacy whitespace is
/// trimmed -- there is no `;`-comment support at this level (see the
/// module doc).
fn is_blank(line: &str) -> bool {
    line.trim_matches(is_space_char).is_empty()
}

/// legacy: `get_motifs`.
fn parse_motifs(lines: &[&str], i: &mut usize) -> Result<Vec<Vec<Note>>> {
    let mut motifs = Vec::new();
    let mut motifno = 0usize;
    loop {
        let mut header = None;
        while *i < lines.len() {
            let line = lines[*i];
            *i += 1;
            let trimmed = line.trim_start_matches(is_space_char);
            if is_blank(trimmed) {
                continue;
            }
            header = Some(trimmed);
            break;
        }
        let Some(header) = header else {
            break;
        };
        motifno += 1;
        let Some(rest) = header.strip_prefix('#') else {
            return Err(DataError::NotedataMissingHash { motifno });
        };
        if !rest.starts_with(|c: char| c.is_ascii_digit()) {
            return Err(DataError::NotedataNoDatalength { motifno });
        }
        let datalen = scan_c_int_prefix(rest).ok_or(DataError::NotedataNoDatalength { motifno })?;
        if datalen <= 0 {
            return Err(DataError::NotedataInvalidDatalen { datalen, motifno });
        }
        let datalen = datalen as usize;

        let mut notes = Vec::with_capacity(datalen);
        let mut lasttime: Option<f64> = None;
        let mut noteno = 1usize;
        while noteno <= datalen {
            loop {
                if *i >= lines.len() {
                    return Err(DataError::NotedataMissingNoteLine { noteno, motifno });
                }
                let line = lines[*i];
                *i += 1;
                if is_blank(line) {
                    continue;
                }
                notes.push(parse_note_line(line, noteno, motifno, &mut lasttime)?);
                break;
            }
            noteno += 1;
        }
        motifs.push(notes);
    }
    Ok(motifs)
}

/// legacy: `read_a_note_from_notedata_file`'s field-by-field scan via
/// `get_data_item`, which reads each of the five fields with the same
/// `sscanf("%lf", ...)` semantics as [`scan_c_double_prefix`] rather
/// than the breakpoint tokenizer, and reports a missing token and an
/// unparseable token with the same message (both are `get_data_item`
/// returning an error, with no distinction made by its caller).
fn parse_note_line(
    line: &str,
    noteno: usize,
    motifno: usize,
    lasttime: &mut Option<f64>,
) -> Result<Note> {
    let words: Vec<&str> = line
        .split(is_space_char)
        .filter(|w| !w.is_empty())
        .collect();

    let field = |idx: usize| words.get(idx).and_then(|w| scan_c_double_prefix(w));

    let time = field(0).ok_or(DataError::NotedataNoTimeData { noteno, motifno })?;
    if words.len() == 1 {
        return Err(DataError::NotedataNoDataAfterTime { noteno, motifno });
    }
    if noteno > 1
        && let Some(lt) = *lasttime
        && lt > time
    {
        return Err(DataError::NotedataReverseTimeOrder {
            motifno,
            noteno,
            prev_noteno: noteno - 1,
        });
    }
    *lasttime = Some(time);

    let instr_no = field(1).ok_or(DataError::NotedataNoInstrNo { noteno, motifno })?;
    if words.len() == 2 {
        return Err(DataError::NotedataNoDataAfterInstrNo { noteno, motifno });
    }

    let pitch = field(2).ok_or(DataError::NotedataNoPitchData { noteno, motifno })?;
    if words.len() == 3 {
        return Err(DataError::NotedataNoDataAfterPitch { noteno, motifno });
    }

    let amp = field(3).ok_or(DataError::NotedataNoAmpData { noteno, motifno })?;
    if words.len() == 4 {
        return Err(DataError::NotedataNoDataAfterAmp { noteno, motifno });
    }

    let dur = field(4).ok_or(DataError::NotedataNoDurationData { noteno, motifno })?;
    // legacy: there is no "no data after dur" check -- trailing words
    // (commonly a `;comment`, e.g. `docs/manual/data/rhythtempl.txt`)
    // are silently ignored.

    Ok(Note {
        time,
        instr_no,
        pitch,
        amp,
        dur,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn note(time: f64, instr_no: f64, pitch: f64, amp: f64, dur: f64) -> Note {
        Note {
            time,
            instr_no,
            pitch,
            amp,
            dur,
        }
    }

    #[test]
    fn parses_pitch_only_file_with_no_motifs() {
        let f = NoteDataFile::parse("50\n", 1).unwrap();
        assert_eq!(f.sample_pitches(), &[50.0]);
        assert!(f.motifs().is_empty());
    }

    #[test]
    fn leading_semicolon_comment_before_pitch_line_is_skipped() {
        let f = NoteDataFile::parse(";a comment\n60\n", 1).unwrap();
        assert_eq!(f.sample_pitches(), &[60.0]);
    }

    #[test]
    fn comment_marker_must_be_the_literal_first_character() {
        // Leading whitespace before ';' means this is NOT treated as
        // a comment (unlike is_comment_or_blank_line elsewhere in
        // this crate) -- it is parsed as the pitch line and fails.
        let err = NoteDataFile::parse("  ;not a comment\n60\n", 1).unwrap_err();
        assert!(matches!(err, DataError::NotedataInsufficientPitches));
    }

    #[test]
    fn excess_pitch_values_on_the_line_are_ignored() {
        let f = NoteDataFile::parse("60 62 64 67\n", 2).unwrap();
        assert_eq!(f.sample_pitches(), &[60.0, 62.0]);
    }

    #[test]
    fn pitches_split_across_two_lines_is_an_error_not_a_continuation() {
        // legacy quirk (documented behaviour, not a bug): pitches
        // must all fit on the first non-comment line.
        let err = NoteDataFile::parse("60 62\n64 67\n", 4).unwrap_err();
        assert!(matches!(err, DataError::NotedataInsufficientPitches));
    }

    #[test]
    fn missing_pitch_line_at_eof_is_an_error() {
        let err = NoteDataFile::parse(";only a comment\n", 1).unwrap_err();
        assert!(matches!(err, DataError::NotedataInsufficientPitches));
    }

    #[test]
    fn parses_one_motif() {
        let f = NoteDataFile::parse(
            "62\n#5\n0 1 62 0 0\n0 1 65 0 0\n0 1 69 0 0\n0 1 71 0 0\n0 1 74 0 0\n",
            1,
        )
        .unwrap();
        assert_eq!(f.sample_pitches(), &[62.0]);
        assert_eq!(
            f.motifs(),
            &[vec![
                note(0.0, 1.0, 62.0, 0.0, 0.0),
                note(0.0, 1.0, 65.0, 0.0, 0.0),
                note(0.0, 1.0, 69.0, 0.0, 0.0),
                note(0.0, 1.0, 71.0, 0.0, 0.0),
                note(0.0, 1.0, 74.0, 0.0, 0.0),
            ]]
        );
    }

    #[test]
    fn blank_line_between_motifs_is_skipped() {
        let f = NoteDataFile::parse(
            "60 60 60 60\n#2\n0.000 1 63 0 0\n0.001 1 66 0 0\n\n#1\n1.0 1 60 0 0\n",
            4,
        )
        .unwrap();
        assert_eq!(f.motifs().len(), 2);
    }

    #[test]
    fn blank_line_within_a_motif_is_skipped_and_does_not_count_as_a_note() {
        let f = NoteDataFile::parse("62\n#2\n0.000 1 63 0 0\n\n0.001 1 66 0 0\n", 1).unwrap();
        assert_eq!(f.motifs()[0].len(), 2);
    }

    #[test]
    fn header_missing_hash_is_an_error() {
        let err = NoteDataFile::parse("62\n5\n0 1 62 0 0\n", 1).unwrap_err();
        assert!(matches!(err, DataError::NotedataMissingHash { motifno: 1 }));
    }

    #[test]
    fn header_with_no_digit_after_hash_is_an_error() {
        let err = NoteDataFile::parse("62\n# 5\n0 1 62 0 0\n", 1).unwrap_err();
        assert!(matches!(
            err,
            DataError::NotedataNoDatalength { motifno: 1 }
        ));
    }

    #[test]
    fn zero_datalength_is_an_error() {
        let err = NoteDataFile::parse("62\n#0\n", 1).unwrap_err();
        assert!(matches!(
            err,
            DataError::NotedataInvalidDatalen {
                datalen: 0,
                motifno: 1
            }
        ));
    }

    #[test]
    fn missing_note_line_at_eof_is_an_error() {
        let err = NoteDataFile::parse("62\n#2\n0 1 62 0 0\n", 1).unwrap_err();
        assert!(matches!(
            err,
            DataError::NotedataMissingNoteLine {
                noteno: 2,
                motifno: 1
            }
        ));
    }

    #[test]
    fn each_missing_field_reports_its_own_error() {
        assert!(matches!(
            NoteDataFile::parse("62\n#1\n\n", 1).unwrap_err(),
            DataError::NotedataMissingNoteLine {
                noteno: 1,
                motifno: 1
            }
        ));
        assert!(matches!(
            NoteDataFile::parse("62\n#1\n1.0\n", 1).unwrap_err(),
            DataError::NotedataNoDataAfterTime {
                noteno: 1,
                motifno: 1
            }
        ));
        assert!(matches!(
            NoteDataFile::parse("62\n#1\n1.0 1\n", 1).unwrap_err(),
            DataError::NotedataNoDataAfterInstrNo {
                noteno: 1,
                motifno: 1
            }
        ));
        assert!(matches!(
            NoteDataFile::parse("62\n#1\n1.0 1 62\n", 1).unwrap_err(),
            DataError::NotedataNoDataAfterPitch {
                noteno: 1,
                motifno: 1
            }
        ));
        assert!(matches!(
            NoteDataFile::parse("62\n#1\n1.0 1 62 0\n", 1).unwrap_err(),
            DataError::NotedataNoDataAfterAmp {
                noteno: 1,
                motifno: 1
            }
        ));
        assert!(matches!(
            NoteDataFile::parse("62\n#1\nabc 1 62 0 0\n", 1).unwrap_err(),
            DataError::NotedataNoTimeData {
                noteno: 1,
                motifno: 1
            }
        ));
    }

    #[test]
    fn trailing_words_after_duration_are_ignored() {
        // matches docs/manual/data/rhythtempl.txt's `;quintuplet` note
        let f = NoteDataFile::parse("60\n#1\n0.0\t1  0  0  0   ;quintuplet\n", 1).unwrap();
        assert_eq!(f.motifs()[0][0].dur, 0.0);
    }

    #[test]
    fn reverse_time_order_within_a_motif_is_an_error() {
        let err = NoteDataFile::parse("62\n#2\n1.0 1 62 0 0\n0.5 1 65 0 0\n", 1).unwrap_err();
        assert!(matches!(
            err,
            DataError::NotedataReverseTimeOrder {
                motifno: 1,
                noteno: 2,
                prev_noteno: 1
            }
        ));
    }

    #[test]
    fn equal_times_within_a_motif_are_allowed_for_chords() {
        NoteDataFile::parse("62\n#2\n1.0 1 62 0 0\n1.0 1 65 0 0\n", 1).unwrap();
    }

    #[test]
    fn motif_count_exact_match_required_when_not_at_least() {
        let f = NoteDataFile::parse("62\n#1\n1.0 1 62 0 0\n#1\n1.0 1 65 0 0\n", 1).unwrap();
        let err = f.check_motif_count(1, false).unwrap_err();
        assert!(matches!(
            err,
            DataError::NotedataIncorrectMotifCount {
                motifcnt: 2,
                expected: 1
            }
        ));
    }

    #[test]
    fn motif_count_at_least_allows_more_than_expected() {
        let f = NoteDataFile::parse("62\n#1\n1.0 1 62 0 0\n#1\n1.0 1 65 0 0\n", 1).unwrap();
        f.check_motif_count(1, true).unwrap();
    }

    #[test]
    fn motif_count_at_least_rejects_fewer_than_expected() {
        let f = NoteDataFile::parse("62\n", 1).unwrap();
        let err = f.check_motif_count(1, true).unwrap_err();
        assert!(matches!(err, DataError::NotedataInsufficientMotifs));
    }

    #[test]
    fn from_file_reports_the_legacy_message_for_a_missing_file() {
        let err = NoteDataFile::from_file("/no/such/notedata.txt", 1).unwrap_err();
        assert!(matches!(err, DataError::CannotOpenNotedataFile { .. }));
    }
}
