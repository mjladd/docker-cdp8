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
//! `IntOrBreakpoint`, `File`. `Int`/`IntOrBreakpoint` are not
//! implemented yet -- no real command has confirmed what they look
//! like -- but the other three now are, having turned out to be
//! distinguishable after all: `modify loudness`'s `-l<level>` optional
//! flag (modes 3 and 4) is a plain `Double` (no breakpoint fallback,
//! confirmed by a live run of `modify loudness 3 infile outfile
//! -labc`, which fails with `"Cannot read parameter 2 [abc]:
//! brkpnt_files not permitted."` rather than trying to open `abc` as a
//! breakpoint file), while mode 1's `gain` positional argument is
//! [`ParamType::DoubleOrBreakpoint`] (see `crate::parser`'s module
//! doc). In `parstruct.c`'s own source this is exactly the
//! upper-/lower-case distinction visible in `LOUDNESS_GAIN`'s
//! parameter-list letter (`"D0"`) versus `LOUDNESS_NORM`/
//! `LOUDNESS_SET`'s optional-flag value-type letter (`"d"`).

/// One parameter's type and (for the numeric types) valid range.
/// legacy: one `char` of `parstruct.c`'s `param_list`/`opt_list`
/// string, paired with the `ap->lo[paramno]`/`ap->hi[paramno]` pair
/// `set_param_ranges` fills in for that same `paramno`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamType {
    /// legacy: an upper-case `D` entry. A token that does not parse
    /// as a plain number is retried as a breakpoint file
    /// (`legacy/dev/cdp2k/readdata.c`'s
    /// `get_brkpnt_data_from_file_and_test_it`); [`crate::ParamValue::Breakpoint`]
    /// carries the result, range-checked point-by-point exactly as
    /// [`crate::ParamValue::Number`] is range-checked as a whole.
    DoubleOrBreakpoint { lo: f64, hi: f64 },
    /// legacy: a lower-case `d` entry -- no breakpoint-file fallback.
    /// An unparseable token is rejected outright with `"Cannot read
    /// parameter {legacy_index} [{token}]: brkpnt_files not
    /// permitted."`, confirmed live for `modify loudness 3`/`4`'s
    /// `-l` flag. `legacy_index` is that message's own parameter
    /// number, which -- confirmed by the same live run -- is *not*
    /// the same number [`crate::error::ParamsError::ValueOutOfRange`]
    /// uses for the very same parameter (that one shows `Parameter[1]`
    /// for `-l`, this shows `parameter 2`): legacy's own value-range
    /// check and its unparseable-token check are evidently two
    /// separate code paths numbering parameters two different ways,
    /// not a single scheme this crate could compute generally. Until
    /// a second real `Double` parameter is confirmed, `legacy_index`
    /// is carried as a literal per-parameter fact, not derived.
    Double {
        lo: f64,
        hi: f64,
        legacy_index: usize,
    },
    /// A positional file argument (an input sound file). legacy: the
    /// generic `"Can't open file %s to read data.\n"` check that
    /// runs on every input file argument before any numeric
    /// parameter is parsed.
    File,
}

/// One `-<letter><value>` optional flag (legacy: `opt_flags`/
/// `opt_list` in `parstruct.c`, e.g. `LOUDNESS_NORM`/`LOUDNESS_SET`'s
/// `opt_flags = "l"`, `opt_list = "d"`). The value is attached
/// directly after the letter with no space (confirmed live: `-l
/// 0.5`, with a space, fails with `"option parameter missing with
/// flag -l"` -- the same message a bare `-l` with nothing after it
/// produces).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptionFlag {
    pub letter: char,
    /// legacy: only `Double` and `DoubleOrBreakpoint` are confirmed
    /// as flag value types so far; [`ParamType::File`] is meaningless
    /// here and never used.
    pub value_type: ParamType,
    /// legacy: the position [`crate::error::ParamsError::ValueOutOfRange`]
    /// reports this flag's value at when it is out of range (see
    /// [`ParamType::Double`]'s doc on why this cannot be computed
    /// generally yet). 1-based, matching the legacy message.
    pub range_check_paramno: usize,
}

/// One process/mode's full argument list: some number of leading
/// [`ParamType::File`] input files, then one output filename (not
/// itself a [`ParamType`] entry -- legacy never range- or
/// existence-checks it during parsing, since it is being created),
/// then either a fixed list of required numeric [`Self::params`]
/// (`modify loudness` mode 1) *or* a set of optional [`Self::flags`]
/// (`modify loudness` modes 3/4) -- confirmed live that these two
/// shapes report a leftover unrecognised token differently
/// (`"Too many parameters on command line."` when `params` is
/// non-empty, `"Unknown parameter '<token>'"` when it is empty and
/// the token is not a recognised flag), so [`crate::parser::parse`]
/// does not yet handle a mode with both.
#[derive(Debug, Clone)]
pub struct CommandSpec {
    /// Number of leading input-file arguments, before the single
    /// output filename. legacy: `ap->max_infile_cnt` for a mode with
    /// a fixed input count (`modify loudness` always takes exactly
    /// one).
    pub infile_count: usize,
    pub params: Vec<ParamType>,
    pub flags: Vec<OptionFlag>,
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
            flags: vec![],
        }
    }

    /// `modify loudness 3` (`LOUDNESS_NORM`): `modify loudness 3
    /// infile outfile [-llevel]`. legacy: `parstruct.c`'s
    /// `opt_flags = "l"`, `opt_list = "d"`; `tklib1.c`'s
    /// `set_param_ranges`' `case(LOUDNESS_NORM): case(LOUDNESS_SET):
    /// ap->lo[LOUD_LEVEL] = 1.0/MAXSHORT; ap->hi[LOUD_LEVEL] = 1.0;`.
    /// `legacy_index` (`2`) and `range_check_paramno` (`1`) are both
    /// confirmed live -- see `crate::parser`'s module doc.
    pub fn modify_loudness_normalise() -> Self {
        CommandSpec {
            infile_count: 1,
            params: vec![],
            flags: vec![OptionFlag {
                letter: 'l',
                value_type: ParamType::Double {
                    lo: 1.0 / 32767.0, // legacy: 1.0/MAXSHORT
                    hi: 1.0,
                    legacy_index: 2, // legacy: LOUD_LEVEL(1) + 1
                },
                range_check_paramno: 1,
            }],
        }
    }

    /// `modify loudness 4` (`LOUDNESS_SET`): `modify loudness 4
    /// infile outfile [-llevel]`. legacy: structurally identical to
    /// [`Self::modify_loudness_normalise`] in `parstruct.c` and
    /// `tklib1.c` (the same `case(LOUDNESS_NORM): case(LOUDNESS_SET):`
    /// range-setting code handles both), confirmed independently live
    /// -- see `crate::parser`'s module doc.
    pub fn modify_loudness_force_level() -> Self {
        Self::modify_loudness_normalise()
    }
}
