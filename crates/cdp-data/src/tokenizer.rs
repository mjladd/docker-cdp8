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
}
