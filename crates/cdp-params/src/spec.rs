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

//! The `CommandSpec` data model: what [`crate::parser::parse`] needs
//! to know about one process/mode combination's positional argument
//! list. legacy: this is the runtime shape `set_param_data` (param
//! types, from `legacy/dev/cdp2k/parstruct.c`) and `set_param_ranges`
//! (numeric ranges, from `legacy/dev/cdp2k/tklib1.c`) build into
//! `struct applic` between them, for one process and mode.
//!
//! `docs/migration/PLAN.md`'s architecture section names five
//! positional parameter types: `Double`, `DoubleOrBreakpoint`, `Int`,
//! `IntOrBreakpoint`, `File`. Only [`ParamType::DoubleOrBreakpoint`]
//! and [`ParamType::File`] are implemented so far, because the one
//! real command this crate has verified against live `legacy`
//! behaviour (`modify loudness 1`, see `crate::parser`'s module doc)
//! only exercises those two: its `gain` parameter is a plain `D`
//! entry in `parstruct.c`, and a live run confirms an unparseable
//! token is retried as a breakpoint filename, not rejected outright
//! (`modify loudness 1 <infile> <outfile> abc` fails with `cdp_data`'s
//! own breakpoint "can't open" text). Whether a bare `Double` (no
//! breakpoint fallback) exists as a genuinely different CLI-visible
//! type, and what `Int`/`IntOrBreakpoint` look like, is not yet
//! confirmed against any real command, so they are deliberately left
//! out rather than guessed at.

/// One positional parameter's type and (for the numeric types) valid
/// range. legacy: one `char` of `parstruct.c`'s `param_list` string,
/// paired with the `ap->lo[paramno]`/`ap->hi[paramno]` pair
/// `set_param_ranges` fills in for that same `paramno`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamType {
    /// legacy: a `D` entry in `param_list`. A token that does not
    /// parse as a plain number is retried as a breakpoint file
    /// (`legacy/dev/cdp2k/readdata.c`'s
    /// `get_brkpnt_data_from_file_and_test_it`); [`crate::ParamValue::Breakpoint`]
    /// carries the result, range-checked point-by-point exactly as
    /// [`crate::ParamValue::Number`] is range-checked as a whole.
    DoubleOrBreakpoint { lo: f64, hi: f64 },
    /// A positional file argument (an input sound file). legacy: the
    /// generic `"Can't open file %s to read data.\n"` check that
    /// runs on every input file argument before any numeric
    /// parameter is parsed.
    File,
}

/// One process/mode's full positional argument list: some number of
/// leading [`ParamType::File`] input files, then one output filename
/// (not itself a [`ParamType`] entry -- legacy never range- or
/// existence-checks it during parsing, since it is being created),
/// then the numeric parameters in [`Self::params`].
#[derive(Debug, Clone)]
pub struct CommandSpec {
    /// Number of leading input-file arguments, before the single
    /// output filename. legacy: `ap->max_infile_cnt` for a mode with
    /// a fixed input count (`modify loudness 1` always takes exactly
    /// one).
    pub infile_count: usize,
    pub params: Vec<ParamType>,
}

impl CommandSpec {
    /// `modify loudness 1` (`LOUDNESS_GAIN`): `modify loudness 1
    /// infile outfile gain`. legacy: `parstruct.c`'s
    /// `set_param_data(ap, 0, 2, 1, "D0")` for the parameter list,
    /// `tklib1.c`'s `set_param_ranges`' `case(LOUDNESS_GAIN): ap->lo
    /// = 0.0; ap->hi = (double)MAXSHORT;` for the range. Confirmed
    /// against live `legacy` runs -- see `crate::parser`'s module
    /// doc.
    pub fn modify_loudness_gain() -> Self {
        CommandSpec {
            infile_count: 1,
            params: vec![ParamType::DoubleOrBreakpoint {
                lo: 0.0,
                hi: 32767.0, // legacy: MAXSHORT
            }],
        }
    }
}
