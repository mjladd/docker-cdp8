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

//! Error type for `cdp-params`.
//!
//! Message text matches the real `legacy` `modify loudness` CLI,
//! confirmed with live runs via the `cdp8-postmerge` Docker image
//! (see the module doc in `crate::parser` for the exact commands).

use std::io;

#[derive(Debug, thiserror::Error)]
pub enum ParamsError {
    /// legacy: the cmdline-word-count check that fires before the
    /// file arguments (infile(s)/outfile) are even separated from the
    /// numeric parameter list -- confirmed live: `modify loudness 1
    /// infile` (missing outfile and the gain parameter) produces
    /// exactly this text.
    #[error("Insufficient cmdline parameters.")]
    InsufficientCmdlineParameters,

    /// legacy: `read_parameters_and_flags`'s own pre-check
    /// (`legacy/dev/cdp2k/readdata.c`), fired when *zero* required
    /// parameter tokens are left, before `get_params` is even called --
    /// confirmed live: `modify loudness 1 infile outfile` (missing the
    /// one and only gain parameter) and `synth wave 1 outfile` (missing
    /// all four required params).
    #[error("Insufficient parameters on command line.")]
    InsufficientParameters,

    /// legacy: `get_params`'s own mid-loop check (same file), fired
    /// instead of [`Self::InsufficientParameters`] when *some but not
    /// all* required parameter tokens are present -- a distinction
    /// invisible with only one required parameter (every mode ported
    /// before `synth wave` has exactly one), since "some but not all"
    /// is then impossible. Confirmed live: `synth wave 1 outfile 44100`
    /// (1 of 4), `... 44100 1` (2 of 4), and `... 44100 1 1.0` (3 of 4)
    /// all produce this text, not [`Self::InsufficientParameters`].
    #[error("Insufficient parameters on cmdline.")]
    InsufficientParametersOnCmdline,

    /// legacy: as above, too many words left over -- confirmed live:
    /// `modify loudness 1 infile outfile 0.5 extra`.
    #[error("Too many parameters on command line.")]
    TooManyParameters,

    /// legacy: `"Can't open file %s to read data.\n"` -- the generic
    /// file-arg open check, confirmed live for a missing `infile`.
    /// Distinct wording from [`cdp_data::DataError::CannotOpen`]
    /// (breakpoint-file specific) and
    /// [`cdp_data::DataError::CannotOpenDataFile`] (word-list-file
    /// specific): this one is for a plain sound-file positional
    /// argument.
    #[error("Can't open file {path} to read data.")]
    CannotOpenFile { path: String, source: io::Error },

    /// legacy: a `DoubleOrBreakpoint` parameter whose token did not
    /// parse as a plain number, so it was retried as a breakpoint
    /// filename -- confirmed live: `modify loudness 1 infile outfile
    /// abc` produces `cdp_data`'s own `CannotOpen` text verbatim
    /// (`"Can't open brkpntfile abc to read data."`), since legacy
    /// always tries `get_brkpnt_data_from_file_and_test_it` on a
    /// `DoubleOrBreakpoint` token it could not read as `%lf`.
    #[error(transparent)]
    Breakpoint(#[from] cdp_data::DataError),

    /// legacy: `"Parameter[%d] Value (%lf) out of range (%lf to
    /// %lf)\n"` (`check_param_validity_and_consistency` /
    /// `put_param_val_and_check_range` in `legacy/dev/cdp2k/
    /// tklib1.c`), confirmed live for `modify loudness 1 infile
    /// outfile 99999` and `... -1`. `paramno` is 1-based, matching
    /// the legacy message.
    #[error("Parameter[{paramno}] Value ({value:.6}) out of range ({lo:.6} to {hi:.6})")]
    ValueOutOfRange {
        paramno: usize,
        value: f64,
        lo: f64,
        hi: f64,
    },

    /// legacy: a [`crate::ParamType::Double`] token that failed to
    /// parse as a plain number -- confirmed live: `modify loudness 3
    /// infile outfile -labc`.
    #[error("Cannot read parameter {legacy_index} [{token}]: brkpnt_files not permitted.")]
    CannotReadParameter { legacy_index: usize, token: String },

    /// legacy: `"Unknown flag -%c on command line.\n"`, confirmed
    /// live: `modify loudness 3 infile outfile -x0.5`.
    #[error("Unknown flag -{0} on command line.")]
    UnknownFlag(char),

    /// legacy: `"Duplicate option %c used on command line\n"`
    /// (`get_options` in `legacy/dev/cdp2k/readdata.c`) -- the same
    /// flag letter given twice. This check runs generically for every
    /// command with optional flags, not just the one that surfaced it:
    /// confirmed live for both `pvoc anal 1 infile outfile -c512
    /// -c99999` (reporting the duplicate rather than the second
    /// value's own out-of-range error) and, retroactively, `modify
    /// loudness 3 infile outfile -l0.5 -l0.6`. Checked after a bare
    /// flag's own missing-value check (`-c512 -c` reports
    /// [`Self::OptionValueMissing`], not this), but before the second
    /// occurrence's value is parsed or range-checked at all.
    #[error("Duplicate option {0} used on command line")]
    DuplicateOption(char),

    /// legacy: `"variant parameter missing with flag -%c\n"`
    /// (`get_variant_no` in `legacy/dev/cdp2k/readdata.c`) -- the
    /// variant-flag counterpart of [`Self::OptionValueMissing`],
    /// confirmed live for `distort repeat infile outfile 3 -c2 -s`
    /// (a bare trailing `-s`). Checked before [`Self::DuplicateFlag`],
    /// the same order [`Self::OptionValueMissing`] takes relative to
    /// [`Self::DuplicateOption`] -- confirmed live with `-s1 -s`.
    #[error("variant parameter missing with flag -{0}")]
    VariantValueMissing(char),

    /// legacy: `"Duplicate flag %c used on command line\n"`
    /// (`get_variants_and_flags` in `legacy/dev/cdp2k/readdata.c`) --
    /// the variant-flag counterpart of [`Self::DuplicateOption`],
    /// distinct text ("flag" vs "option"), confirmed live for `distort
    /// repeat infile outfile 3 -s1 -s2`.
    #[error("Duplicate flag {0} used on command line")]
    DuplicateFlag(char),

    /// legacy: `"option flag -%c out of order on cmdline.\n"`
    /// (`get_variant_no`) -- a flag letter that belongs to
    /// [`crate::CommandSpec::flags`], not
    /// [`crate::CommandSpec::variants`], appearing after variant
    /// scanning has already started (so it is too late to be
    /// recognised as an option). Confirmed live for `distort repeat
    /// infile outfile 3 -s1 -c2` (`-c` is a real option, but appears
    /// after `-s`, a variant).
    #[error("option flag -{0} out of order on cmdline.")]
    OptionOutOfOrder(char),

    /// legacy: `"Unknown variant flag -%c\n"` (`get_variant_no`,
    /// reached only when the mode has at least one option -- see that
    /// function's `else` branch, `"Unknown flag '-%c'\n"`, for the
    /// no-options case, not yet implemented since no confirmed command
    /// exercises it). Confirmed live for `distort repeat infile
    /// outfile 3 -c2 -z5`.
    #[error("Unknown variant flag -{0}")]
    UnknownVariantFlag(char),

    /// legacy: `"option parameter missing with flag -%c\n"`,
    /// confirmed live both for a bare trailing `-l` and for `-l 0.5`
    /// (a space-separated value is not supported at all: the whole
    /// next word is a separate, unrecognised token, not part of this
    /// flag).
    #[error("option parameter missing with flag -{0}")]
    OptionValueMissing(char),

    /// legacy: `"Unknown parameter '%s'\n"` -- confirmed live for a
    /// leftover non-flag token in a mode whose `params` list is empty
    /// (`modify loudness 3 infile outfile 0.5`, an extra word neither
    /// consumed as a required positional parameter, since this mode
    /// has none, nor recognised as a `-`-prefixed flag). Distinct
    /// from [`Self::TooManyParameters`], which is what a mode with a
    /// non-empty `params` list uses for the same underlying "extra
    /// word" situation.
    #[error("Unknown parameter '{0}'")]
    UnknownParameter(String),

    /// legacy: `get_mode_from_cmdline` in `legacy/dev/cdp2k/tklib3.c`,
    /// `sscanf(str,"%d",&dz->mode)` finding no leading integer at all
    /// -- confirmed live: `synth wave abc outfile.wav 44100 1 1.0
    /// 440`.
    #[error("Cannot read mode of program.")]
    CannotReadModeOfProgram,

    /// legacy: `get_mode_from_cmdline`'s range check, once a mode
    /// number was read -- confirmed live: `synth wave 9 outfile.wav
    /// 44100 1 1.0 440`. `mode` is whatever `sscanf("%d")` read, which
    /// -- confirmed live -- truncates at the first non-digit rather
    /// than rejecting the token (`synth wave 3.5 ...` and `synth wave
    /// 3abc ...` both run mode 3 to completion): [`crate::parse_mode`]
    /// reproduces that truncation, not full-token validation.
    #[error("Program mode value [{mode}] is out of range [1 - {maxmode}].")]
    ModeOutOfRange { mode: i64, maxmode: u32 },
}

pub type Result<T> = std::result::Result<T, ParamsError>;
