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

//! Parsing the mode number: the command-line word right after a
//! group program's sub-command name and before [`crate::parse`]'s own
//! `args` (e.g. `1` in `synth wave 1 outfile.wav ...`). legacy:
//! `get_mode_from_cmdline` in `legacy/dev/cdp2k/tklib3.c`. This is
//! generic across every group program (confirmed by reading the
//! function: it only ever consults `dz->maxmode`, a per-process
//! constant, never anything mode-list-specific), so it lives here
//! rather than being duplicated per `CommandSpec`.

use crate::error::{ParamsError, Result};

/// Parses `token` as a mode number and checks it against `maxmode`
/// (1-based, inclusive -- e.g. `synth wave`'s four modes pass `4`).
/// legacy: `sscanf(str,"%d",&dz->mode)` followed by `dz->mode <= 0 ||
/// dz->mode > dz->maxmode`. The `sscanf("%d")` truncation quirk is
/// deliberate, not a shortcut: confirmed live, see
/// [`ParamsError::ModeOutOfRange`]'s doc.
pub fn parse_mode(token: &str, maxmode: u32) -> Result<u32> {
    let mode = scan_leading_int(token).ok_or(ParamsError::CannotReadModeOfProgram)?;
    if mode <= 0 || mode as u64 > maxmode as u64 {
        return Err(ParamsError::ModeOutOfRange { mode, maxmode });
    }
    Ok(mode as u32)
}

/// legacy: `sscanf(str,"%d",...)` -- an optional sign then one or
/// more decimal digits, read as a prefix of `s` (anything after the
/// last digit, valid or not, is silently ignored). Returns `None`
/// only when `s` has no leading digits at all (`sscanf` itself
/// returning `0`, not `1`).
fn scan_leading_int(s: &str) -> Option<i64> {
    let bytes = s.as_bytes();
    let mut i = 0;
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
    s[..i].parse::<i64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_mode_in_range() {
        assert_eq!(parse_mode("3", 4).unwrap(), 3);
    }

    #[test]
    fn truncates_at_first_non_digit() {
        // legacy: confirmed live, `synth wave 3.5 outfile.wav 44100 1
        // 1.0 440` and `synth wave 3abc ...` both run mode 3.
        assert_eq!(parse_mode("3.5", 4).unwrap(), 3);
        assert_eq!(parse_mode("3abc", 4).unwrap(), 3);
    }

    #[test]
    fn no_leading_digits_is_unreadable() {
        assert!(matches!(
            parse_mode("abc", 4),
            Err(ParamsError::CannotReadModeOfProgram)
        ));
    }

    #[test]
    fn zero_and_negative_are_out_of_range() {
        assert!(matches!(
            parse_mode("0", 4),
            Err(ParamsError::ModeOutOfRange {
                mode: 0,
                maxmode: 4
            })
        ));
        assert!(matches!(
            parse_mode("-1", 4),
            Err(ParamsError::ModeOutOfRange {
                mode: -1,
                maxmode: 4
            })
        ));
    }

    #[test]
    fn above_maxmode_is_out_of_range() {
        assert!(matches!(
            parse_mode("9", 4),
            Err(ParamsError::ModeOutOfRange {
                mode: 9,
                maxmode: 4
            })
        ));
    }
}
