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

//! The number tokenizer every CDP text data format is built on.
//! legacy: `get_float_from_within_string` in
//! `legacy/dev/cdp2k/tklib1.c`.
//!
//! This is a faithful, byte-for-byte port, including two behaviours
//! that look like bugs but are load-bearing for matching legacy
//! output exactly:
//!
//! - **No scientific notation.** The character scan only accepts
//!   digits, one `.`, and a leading `-`. A token like `1e-5` fails at
//!   the `e`, which aborts the *rest of the current line* (see
//!   below), not just that one token.
//! - **One bad token abandons the rest of the line.** The legacy
//!   caller reads one line at a time (`fgets`) and calls this
//!   tokenizer in a `while` loop until it returns false, then moves
//!   to the next line. A single invalid character -- whether it
//!   starts a real comment (`;a comment`) or is just malformed data
//!   -- stops token extraction for that line, silently discarding
//!   anything after it on the same line. [`parse_line_floats`]
//!   reproduces this per line, since that is the granularity legacy
//!   callers see it at.

/// Attempts to read one float from the start of `s`, after skipping
/// leading whitespace. Returns the parsed value and the remainder of
/// `s` starting right after the consumed token, or `None` if `s`
/// (after skipping whitespace) does not start a valid numeric token --
/// in which case `s` is unconsumed, matching the legacy function
/// leaving `*str` unchanged on a false return.
pub fn next_float(s: &str) -> Option<(f64, &str)> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() && is_space(bytes[i]) {
        i += 1;
    }
    let start = i;
    let mut decimal_point_cnt = 0u32;
    let mut has_digits = false;

    if i >= bytes.len() {
        return None;
    }
    match bytes[i] {
        b'-' => {}
        b'.' => decimal_point_cnt = 1,
        b if b.is_ascii_digit() => has_digits = true,
        _ => return None,
    }
    i += 1;

    while i < bytes.len() && !is_space(bytes[i]) && bytes[i] != b'\n' {
        match bytes[i] {
            b if b.is_ascii_digit() => has_digits = true,
            b'.' => {
                decimal_point_cnt += 1;
                if decimal_point_cnt > 1 {
                    return None;
                }
            }
            _ => return None,
        }
        i += 1;
    }

    if !has_digits {
        return None;
    }
    let token = &s[start..i];
    let value: f64 = token.parse().ok()?;
    Some((value, &s[i..]))
}

/// legacy: `isspace()` under the C locale -- space, tab, newline,
/// vertical tab, form feed, carriage return.
fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r')
}

/// Extracts every float from one line, stopping at the first
/// character that cannot start or continue a token (see the module
/// doc: this includes a `;` comment marker, and discards the rest of
/// the line when it happens, matching the legacy per-line behaviour).
pub fn parse_line_floats(line: &str) -> Vec<f64> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some((value, remainder)) = next_float(rest) {
        out.push(value);
        rest = remainder;
    }
    out
}

/// legacy: `FLTERR` in `legacy/dev/include/globcon.h`.
pub const FLTERR: f64 = 0.000002;

/// legacy: `flteq` in `legacy/dev/cdp2k/tklib1.c`.
pub fn flteq(a: f64, b: f64) -> bool {
    (a - b).abs() <= FLTERR
}

/// legacy: `isspace()` under the C locale, as seen by
/// `is_an_empty_line_or_a_comment` and `get_word_from_string` in
/// `legacy/dev/cdp2k/tklib1.c`. A `char`-based twin of [`is_space`]
/// for the word-list formats below, which work on `str` rather than
/// bytes.
fn is_space_char(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
}

/// legacy: `is_an_empty_line_or_a_comment` in `tklib1.c`, called by
/// `store_wordlist` in `readfiles.c` before a line is word-split.
/// Every CDP word-list format (mix files, and -- in later slices --
/// texture note-data and tuning files) uses this: a line is skipped
/// entirely, not just trimmed, when it is blank or starts with `;`
/// after leading whitespace. Unlike [`parse_line_floats`]'s per-token
/// comment handling for breakpoint files, a `;` appearing after real
/// data on the same line is not a comment marker here -- see
/// [`split_words`].
pub fn is_comment_or_blank_line(line: &str) -> bool {
    matches!(
        line.trim_start_matches(is_space_char).chars().next(),
        None | Some(';')
    )
}

/// legacy: `get_word_from_string` in `tklib1.c`, called in a loop by
/// `store_wordlist` in `readfiles.c` to split one already-non-comment
/// line into its whitespace-separated words. There is no quoting and
/// no per-word comment stripping: a `;` in the middle of a line is
/// just another word, which is why a trailing comment on a data line
/// (fine in a breakpoint file) instead trips the word-count check of
/// whichever format is reading the line.
pub fn split_words(line: &str) -> Vec<&str> {
    line.split(is_space_char)
        .filter(|w| !w.is_empty())
        .collect()
}

/// legacy: mimics the prefix-parsing behaviour of `sscanf(str, "%lf",
/// ...)` -- an optional sign, digits, an optional `.` and more
/// digits, and an optional exponent, reading only as much of a
/// leading numeric prefix as is valid rather than requiring the whole
/// token to be numeric. This is what several mix-file fields are
/// actually scanned with in `legacy/dev/submix/setupmix.c` (time,
/// and, when not a `dB`-suffixed level, the level and pan fields),
/// unlike [`next_float`] (CDP's own tokenizer, used for breakpoint
/// files), which rejects scientific notation outright.
pub fn scan_c_double_prefix(s: &str) -> Option<f64> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let mut has_digits = false;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
        has_digits = true;
    }
    if i < bytes.len() && bytes[i] == b'.' {
        i += 1;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            i += 1;
            has_digits = true;
        }
    }
    if !has_digits {
        return None;
    }
    let mut exp_end = i;
    if i < bytes.len() && (bytes[i] == b'e' || bytes[i] == b'E') {
        let mut j = i + 1;
        if j < bytes.len() && (bytes[j] == b'+' || bytes[j] == b'-') {
            j += 1;
        }
        let exp_digits_start = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j > exp_digits_start {
            exp_end = j;
        }
    }
    s[..exp_end].parse::<f64>().ok()
}

/// legacy: mimics the prefix-parsing behaviour of `sscanf(str, "%d",
/// ...)` -- an optional sign followed by one or more digits, stopping
/// at the first non-digit rather than requiring the whole token to be
/// an integer. legacy: the `chans` field in
/// `legacy/dev/submix/setupmix.c`'s `get_mixdata_in_line`.
pub fn scan_c_int_prefix(s: &str) -> Option<i32> {
    let bytes = s.as_bytes();
    let mut i = 0usize;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        i += 1;
    }
    let digits_start = i;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        i += 1;
    }
    if i == digits_start {
        return None;
    }
    s[..i].parse::<i32>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_plain_pair() {
        assert_eq!(parse_line_floats(" 0.0  0.15"), vec![0.0, 0.15]);
    }

    #[test]
    fn full_line_comment_yields_nothing() {
        assert_eq!(parse_line_floats(";balltv.brk"), Vec::<f64>::new());
    }

    #[test]
    fn trailing_comment_after_values_is_discarded_not_an_error() {
        assert_eq!(
            parse_line_floats(" 0.0  0.15   ;start fast"),
            vec![0.0, 0.15]
        );
    }

    #[test]
    fn blank_line_yields_nothing() {
        assert_eq!(parse_line_floats("   \t  "), Vec::<f64>::new());
    }

    #[test]
    fn negative_numbers_parse() {
        assert_eq!(parse_line_floats("-1.5 2.0"), vec![-1.5, 2.0]);
    }

    #[test]
    fn no_leading_zero_still_parses() {
        assert_eq!(parse_line_floats(".5 12.0"), vec![0.5, 12.0]);
    }

    /// legacy quirk, not a limitation of this port: scientific
    /// notation is not accepted. The 'e' invalidates the whole token
    /// it appears in -- the scan returns false as soon as it sees an
    /// unrecognised character, before ever extracting a value for the
    /// digits already scanned -- and that abandons the rest of the
    /// line too, since the per-line caller stops at the first failed
    /// token.
    #[test]
    fn scientific_notation_invalidates_the_whole_token_and_abandons_the_rest_of_the_line() {
        assert_eq!(parse_line_floats("1e-5 2.0"), Vec::<f64>::new());
    }

    /// legacy quirk: a second '.' inside one token invalidates that
    /// whole token (not just the part from the second '.' onward),
    /// and, as above, abandons the rest of the line.
    #[test]
    fn two_decimal_points_invalidates_the_whole_token() {
        assert_eq!(parse_line_floats("1.2.3 4.0"), Vec::<f64>::new());
    }

    #[test]
    fn indentation_amount_does_not_matter() {
        assert_eq!(
            parse_line_floats("12.0  0.10"),
            parse_line_floats("   12.0  0.10")
        );
    }

    #[test]
    fn flteq_matches_within_epsilon_only() {
        assert!(flteq(1.0, 1.0 + 0.0000015));
        assert!(!flteq(1.0, 1.0 + 0.0000025));
    }

    #[test]
    fn comment_and_blank_lines_are_recognised() {
        assert!(is_comment_or_blank_line(";a comment"));
        assert!(is_comment_or_blank_line("   ;indented comment"));
        assert!(is_comment_or_blank_line(""));
        assert!(is_comment_or_blank_line("   "));
        assert!(!is_comment_or_blank_line("capm.wav 0.0 1 0.5 C"));
    }

    #[test]
    fn split_words_splits_on_runs_of_whitespace() {
        assert_eq!(
            split_words("capm.wav  0.0\t1  0.5 C"),
            vec!["capm.wav", "0.0", "1", "0.5", "C"]
        );
    }

    #[test]
    fn split_words_does_not_strip_a_trailing_comment() {
        // legacy quirk (see the doc comment): a mid-line `;` is just
        // another word here, unlike parse_line_floats.
        assert_eq!(
            split_words("0.0 0.15 ;start fast"),
            vec!["0.0", "0.15", ";start", "fast"]
        );
    }

    #[test]
    fn scan_c_double_prefix_accepts_scientific_notation() {
        // legacy quirk: unlike next_float (the breakpoint-file
        // tokenizer), this accepts it, because it mimics raw
        // sscanf("%lf", ...) rather than get_float_from_within_string.
        assert_eq!(scan_c_double_prefix("1e-1"), Some(0.1));
        assert_eq!(scan_c_double_prefix("-6.5e0"), Some(-6.5));
    }

    #[test]
    fn scan_c_double_prefix_reads_only_the_leading_numeric_prefix() {
        // mimics sscanf("6dB", "%lf", &val) succeeding with 6.0 and
        // leaving "dB" unconsumed.
        assert_eq!(scan_c_double_prefix("6dB"), Some(6.0));
        assert_eq!(scan_c_double_prefix("xdB"), None);
    }

    #[test]
    fn scan_c_int_prefix_stops_at_the_first_non_digit() {
        assert_eq!(scan_c_int_prefix("1.5"), Some(1));
        assert_eq!(scan_c_int_prefix("-2"), Some(-2));
        assert_eq!(scan_c_int_prefix("abc"), None);
    }
}
