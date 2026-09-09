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

//! Parses the positional arguments after a process/mode has already
//! been identified (e.g. after `modify loudness 1` is stripped off),
//! against a [`crate::CommandSpec`].
//!
//! legacy: the file/parameter-count checks in
//! `legacy/dev/cdp2k/mainfuncs.c`'s command-line reader, and the
//! per-parameter range check (`check_param_validity_and_consistency`,
//! `legacy/dev/cdp2k/tklib1.c`).
//!
//! Confirmed against 8 live runs of `legacy` `modify loudness`
//! (mode 1, `LOUDNESS_GAIN`: `modify loudness 1 infile outfile gain`)
//! via the `cdp8-postmerge` Docker image: a run with only `infile`
//! (missing outfile and gain) fails with `"Insufficient cmdline
//! parameters."`; one with `infile outfile` (missing just gain) fails
//! with `"Insufficient parameters on command line."`; one with an
//! extra trailing word fails with `"Too many parameters on command
//! line."`; a nonexistent `infile` fails with `"Can't open file %s to
//! read data."`; a `gain` of `99999` or `-1` fails with
//! `"Parameter[1] Value (...) out of range (0.000000 to
//! 32767.000000)"`; a `gain` of `abc` fails with `cdp_data`'s own
//! `"Can't open brkpntfile abc to read data."` (confirming the
//! breakpoint-file fallback); and `gain 0.5` runs to completion.
//!
//! And against a further 7 live runs of modes 3 and 4 (`LOUDNESS_NORM`/
//! `LOUDNESS_SET`: `modify loudness 3 infile outfile [-llevel]`), which
//! have no required positional parameters, only the optional `-l`
//! flag: a run with no `-l` at all, or `-l0.5` (attached, no space),
//! both parse successfully; `-l1.5` fails with `"Parameter[1] Value
//! (1.500000) out of range (0.000031 to 1.000000)"`; `-labc` fails
//! with `"Cannot read parameter 2 [abc]: brkpnt_files not
//! permitted."` (confirming no breakpoint-file fallback for this
//! parameter type); a bare trailing `-l`, or `-l 0.5` with a space,
//! both fail with `"option parameter missing with flag -l"`; an
//! unrecognised `-x0.5` fails with `"Unknown flag -x on command
//! line."`; and a plain leftover word with no `-` prefix (`modify
//! loudness 3 infile outfile 0.5`) fails with `"Unknown parameter
//! '0.5'"` -- confirmed, independently, for both mode 3 and mode 4.
//!
//! And against a further 9 live runs of `pvoc anal` (all three modes
//! share one spec, `crate::spec::CommandSpec::pvoc_anal`): with no
//! flags, and with both `-c<points>` and `-o<overlap>` given in either
//! order, all parse and run to completion; `-c1` and `-c99999` fail
//! with `"Parameter[1] Value (...) out of range (2.000000 to
//! 32768.000000)"`, `-o5` fails with the same shape at `"Parameter[2]
//! ... (1.000000 to 4.000000)"` (confirming errors are reported in
//! command-line order, not by paramno, since `-o5 -c1` reports
//! `Parameter[2]` first while `-c1 -o5` reports `Parameter[1]` first);
//! `-cabc`/`-oabc` fail with `"Cannot read parameter 1/2 [abc]:
//! brkpnt_files not permitted."`; and repeating a flag (`-c512 -c99999`,
//! and, retroactively, `modify loudness 3 infile outfile -l0.5 -l0.6`)
//! fails with `"Duplicate option c/l used on command line"` -- a
//! generic check this parser previously omitted, reporting the
//! duplicate even when the first occurrence's value was valid and the
//! second's was not.
//!
//! And against a further 15 live runs of `distort repeat` (the first
//! spec with a required param *and* flags at once, and the first with
//! any variants -- `crate::spec::CommandSpec::distort_repeat`): a run
//! with just the required `multiplier`, and one with `-c<cyclecnt>`
//! and `-s<skipcycles>` also given, both run to completion, confirming
//! `-s` is only reachable once `-c` (or the option phase's end) has
//! already been scanned. Giving `-s` before `-c` fails with `"option
//! flag -c out of order on cmdline."`, naming the *option* letter, not
//! the variant already read. `multiplier` retried as a real breakpoint
//! file succeeds (confirming `IntOrBreakpoint`'s fallback); as a
//! nonexistent one fails with the same `cdp_data` breakpoint-open text
//! `DoubleOrBreakpoint` uses. `multiplier` and `-c` (both
//! `IntOrBreakpoint`) out of range report `Parameter[1]`/`Parameter[2]`
//! respectively, continuing the same sequential paramno numbering into
//! `-s` (plain `Int`) at `Parameter[3]` -- unlike `pvoc anal`, whose
//! options start at `1` only because it has no required params ahead
//! of them. A leftover non-flag token after all three are given fails
//! with `"Unknown parameter 'extra'"`, the same shape `pvoc anal` and
//! `modify loudness` modes 3/4 use, now confirmed even when required
//! params are also present. Duplicating `-c` still fails with
//! `"Duplicate option c ..."`; duplicating `-s` fails with the
//! variant-specific `"Duplicate flag s used on command line"`; a bare
//! trailing `-s` fails with `"variant parameter missing with flag
//! -s"`, checked before the duplicate check exactly as
//! `OptionValueMissing` precedes `DuplicateOption` (confirmed live
//! with `-s1 -s`, which reports the missing-value error, not the
//! duplicate); an unrecognised `-z` during the variant phase fails
//! with `"Unknown variant flag -z"`; and `distort repeat infile` alone,
//! or `infile outfile` alone, both fail with `"Insufficient parameters
//! on command line."`, not `"Insufficient cmdline parameters."` --
//! `distort repeat`'s `UNEQUAL_SNDFILE` classification, unlike every
//! other command ported so far -- see
//! `crate::spec::CommandSpec::unequal_sndfile`'s doc.
//!
//! And against a further 13 live runs of `synth wave` (the first spec
//! with more than one required param, and the first with a
//! [`crate::Variant::Boolean`] -- `crate::spec::CommandSpec::synth_wave`):
//! a run with all four required params and no flags, and one with
//! `-a<amp>`, `-t<tabsize>` and the undocumented boolean `-f` also
//! given, both run to completion. `-f999` (an attached value on a
//! boolean variant) behaves identically to a bare `-f`, silently
//! discarding the `999`; `-f` before `-a` still fails with `"option
//! flag -a out of order on cmdline."`, the same as a value-carrying
//! variant; and `-f -f` still fails with `"Duplicate flag f used on
//! command line."` Zero of the four required params given fails with
//! `"Insufficient parameters on command line."`, but 1, 2, or 3 of 4
//! (some but not all) fails instead with `"Insufficient parameters on
//! cmdline."` -- a distinction invisible in every mode ported before
//! this one, since none had more than one required param (see
//! `crate::error::ParamsError::InsufficientParametersOnCmdline`'s
//! doc). Paramno numbering is confirmed to run 1 through 4 across
//! `sr`/`chans`/`dur`/`freq` in order, then continue at 5/6 for
//! `-a`/`-t` -- the first confirmation of sequential numbering across
//! *multiple* required params, not just from one required param into
//! the flags that follow it (`distort repeat` only had one). `freq`
//! retried as a real breakpoint file succeeds; `-a`'s own breakpoint
//! fallback failing on a nonexistent file produces the same `cdp_data`
//! text every other `DoubleOrBreakpoint`/`IntOrBreakpoint` case does. A
//! leftover non-flag token fails with `"Unknown parameter 'extra'"`,
//! the same shape already confirmed for every other flag-bearing mode.

use crate::error::{ParamsError, Result};
use crate::spec::{CommandSpec, ParamType, Variant};
use cdp_data::BreakpointTable;
use std::collections::BTreeMap;

/// One parsed parameter's value. legacy: `dz->param[paramno]` for a
/// plain number, or `dz->brk[paramno]` (plus `dz->brksize[paramno]`)
/// for a breakpoint file -- distinguished at parse time here, the
/// same as legacy distinguishes them at read time, rather than
/// carrying both slots as legacy's `struct dataline` does.
#[derive(Debug, Clone)]
pub enum ParamValue {
    Number(f64),
    /// legacy: an [`crate::ParamType::Int`] value, `(int)` cast (C
    /// truncation toward zero) from the same range-checked `f64` a
    /// [`Self::Number`] carries -- see that type's doc.
    Integer(i64),
    Breakpoint(BreakpointTable),
    /// legacy: a [`crate::Variant::Boolean`] flag was given -- no
    /// value at all, just `dz->vflag[flagno] = TRUE`.
    Present,
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub infiles: Vec<String>,
    /// `None` for a [`CommandSpec`] with `has_outfile: false` (an
    /// info-only command like `sndinfo props`, which takes no output
    /// filename at all) -- see that field's doc.
    pub outfile: Option<String>,
    /// The required positional parameters, in order (empty for a mode
    /// whose [`CommandSpec::params`] is empty).
    pub params: Vec<ParamValue>,
    /// The optional flags and variants that were actually present on
    /// the command line, keyed by letter (a letter can only belong to
    /// one of [`CommandSpec::flags`]/[`CommandSpec::variants`] --
    /// legacy keeps them in separate arrays specifically so it can
    /// detect one appearing where the other belongs, see that field's
    /// doc -- so merging both into one map here loses no information).
    /// A flag either list names but the caller did not supply is
    /// simply absent here, matching legacy leaving the parameter at
    /// its own default (not modelled yet -- see
    /// `docs/migration/STATUS.md`).
    pub flags: BTreeMap<char, ParamValue>,
}

/// Parses `args` (everything after the process name, subcommand and
/// mode number) against `spec`.
pub fn parse(spec: &CommandSpec, args: &[&str]) -> Result<ParsedCommand> {
    let outfile_slot = if spec.has_outfile { 1 } else { 0 };
    let min_needed = spec.infile_count + outfile_slot;
    if args.len() < min_needed {
        // legacy: `crate::spec::CommandSpec::unequal_sndfile`'s doc.
        return Err(if spec.unequal_sndfile {
            ParamsError::InsufficientParameters
        } else {
            ParamsError::InsufficientCmdlineParameters
        });
    }

    let mut infiles = Vec::with_capacity(spec.infile_count);
    for &arg in &args[..spec.infile_count] {
        check_file_openable(arg)?;
        infiles.push(arg.to_string());
    }
    let outfile = if spec.has_outfile {
        Some(args[spec.infile_count].to_string())
    } else {
        None
    };
    let rest = &args[spec.infile_count + outfile_slot..];

    if rest.len() < spec.params.len() {
        // legacy: zero tokens left is `read_parameters_and_flags`'s own
        // pre-check; some but not all is `get_params`'s mid-loop check
        // -- see `ParamsError::InsufficientParametersOnCmdline`'s doc.
        return Err(if rest.is_empty() {
            ParamsError::InsufficientParameters
        } else {
            ParamsError::InsufficientParametersOnCmdline
        });
    }
    let (param_tokens, after_params) = rest.split_at(spec.params.len());
    let mut params = Vec::with_capacity(spec.params.len());
    for (i, (&token, param_type)) in param_tokens.iter().zip(&spec.params).enumerate() {
        let paramno = i + 1; // legacy: 1-based in "Parameter[%d]"
        params.push(parse_param(token, *param_type, paramno)?);
    }

    let flags = if spec.flags.is_empty() && spec.variants.is_empty() {
        // legacy: a mode with no options or variants never reaches
        // `get_options`/`get_variants_and_flags` at all, so a leftover
        // word falls to `read_parameters_and_flags`'s own final check
        // -- see the module doc's `modify loudness` mode 1 example.
        if !after_params.is_empty() {
            return Err(ParamsError::TooManyParameters);
        }
        BTreeMap::new()
    } else {
        parse_flags_and_variants(spec, after_params)?
    };

    Ok(ParsedCommand {
        infiles,
        outfile,
        params,
        flags,
    })
}

/// legacy: `get_options` then, if any are declared,
/// `get_variants_and_flags` -- two ordered phases over the same
/// remaining tokens. See [`CommandSpec`]'s doc for the order
/// constraint this creates and every error case confirmed here.
fn parse_flags_and_variants(
    spec: &CommandSpec,
    rest: &[&str],
) -> Result<BTreeMap<char, ParamValue>> {
    let mut flags = BTreeMap::new();
    let mut i = 0;

    // Phase 1: options (`spec.flags`), a contiguous prefix. legacy:
    // `get_options`/`get_option_no`. A non-dash token errors
    // immediately (unconditionally, whether or not the mode has any
    // variants); a dash token whose letter isn't a known option stops
    // this phase without erroring, leaving it for phase 2 (if any) or
    // the trailing check below.
    if !spec.flags.is_empty() {
        while i < rest.len() {
            let token = rest[i];
            let Some(after_dash) = token.strip_prefix('-') else {
                return Err(ParamsError::UnknownParameter(token.to_string()));
            };
            let letter = after_dash
                .chars()
                .next()
                .ok_or_else(|| ParamsError::UnknownParameter(token.to_string()))?;
            let Some(flag_spec) = spec.flags.iter().find(|f| f.letter == letter) else {
                break;
            };
            let value_str = &after_dash[letter.len_utf8()..];
            if value_str.is_empty() {
                return Err(ParamsError::OptionValueMissing(letter));
            }
            // legacy: `get_options` checks this (`options_got[option_no]`)
            // right after resolving the flag letter and its missing-value
            // check, but before reading or range-checking its value -- see
            // the module doc's `pvoc anal -c512 -c99999` example.
            if flags.contains_key(&letter) {
                return Err(ParamsError::DuplicateOption(letter));
            }
            let value = parse_param(
                value_str,
                flag_spec.value_type,
                flag_spec.range_check_paramno,
            )?;
            flags.insert(letter, value);
            i += 1;
        }
    }

    // Phase 2: variants (`spec.variants`), consuming everything left.
    // legacy: `get_variants_and_flags`/`get_variant_no`.
    if !spec.variants.is_empty() {
        while i < rest.len() {
            let token = rest[i];
            let Some(after_dash) = token.strip_prefix('-') else {
                return Err(ParamsError::UnknownParameter(token.to_string()));
            };
            let letter = after_dash
                .chars()
                .next()
                .ok_or_else(|| ParamsError::UnknownParameter(token.to_string()))?;
            if let Some(variant) = spec.variants.iter().find(|v| v.letter() == letter) {
                match variant {
                    Variant::Value(variant_spec) => {
                        let value_str = &after_dash[letter.len_utf8()..];
                        if value_str.is_empty() {
                            return Err(ParamsError::VariantValueMissing(letter));
                        }
                        // legacy: checked in the caller, after
                        // `get_variant_no` (which owns the missing-value
                        // check above) returns -- same order as the
                        // option phase's own duplicate check.
                        if flags.contains_key(&letter) {
                            return Err(ParamsError::DuplicateFlag(letter));
                        }
                        let value = parse_param(
                            value_str,
                            variant_spec.value_type,
                            variant_spec.range_check_paramno,
                        )?;
                        flags.insert(letter, value);
                    }
                    Variant::Boolean { .. } => {
                        // legacy: nothing after the letter is ever
                        // inspected -- see `Variant::Boolean`'s doc.
                        if flags.contains_key(&letter) {
                            return Err(ParamsError::DuplicateFlag(letter));
                        }
                        flags.insert(letter, ParamValue::Present);
                    }
                }
            } else if spec.flags.iter().any(|f| f.letter == letter) {
                return Err(ParamsError::OptionOutOfOrder(letter));
            } else {
                return Err(ParamsError::UnknownVariantFlag(letter));
            }
            i += 1;
        }
        return Ok(flags);
    }

    // legacy: `read_parameters_and_flags`'s final check, reached only
    // when the mode has options but no variants and phase 1 stopped
    // early on an unrecognised dash-prefixed token (any non-dash token
    // is already caught inside phase 1 itself).
    if let Some(&token) = rest.get(i) {
        let letter = token
            .strip_prefix('-')
            .and_then(|s| s.chars().next())
            .ok_or_else(|| ParamsError::UnknownParameter(token.to_string()))?;
        return Err(ParamsError::UnknownFlag(letter));
    }
    Ok(flags)
}

fn parse_param(token: &str, param_type: ParamType, paramno: usize) -> Result<ParamValue> {
    match param_type {
        ParamType::DoubleOrBreakpoint { lo, hi } => match token.parse::<f64>() {
            Ok(value) => {
                check_range(value, lo, hi, paramno)?;
                Ok(ParamValue::Number(value))
            }
            // legacy: a token that does not parse with `%lf` is
            // retried as a breakpoint filename, not rejected as a bad
            // number -- see the module doc's `abc` example.
            Err(_) => Ok(ParamValue::Breakpoint(BreakpointTable::from_file(
                token, lo, hi,
            )?)),
        },
        ParamType::Double {
            lo,
            hi,
            legacy_index,
        } => {
            let value = parse_as_f64_or_cannot_read(token, legacy_index)?;
            check_range(value, lo, hi, paramno)?;
            Ok(ParamValue::Number(value))
        }
        ParamType::Int {
            lo,
            hi,
            legacy_index,
        } => {
            let value = parse_as_f64_or_cannot_read(token, legacy_index)?;
            check_range(value, lo, hi, paramno)?;
            // legacy: `(int)` cast of the range-checked double --
            // truncates toward zero, matching Rust's `as i64` here --
            // see `ParamType::Int`'s doc.
            Ok(ParamValue::Integer(value as i64))
        }
        ParamType::IntOrBreakpoint { lo, hi } => match token.parse::<f64>() {
            Ok(value) => {
                check_range(value, lo, hi, paramno)?;
                Ok(ParamValue::Integer(value as i64))
            }
            Err(_) => Ok(ParamValue::Breakpoint(BreakpointTable::from_file(
                token, lo, hi,
            )?)),
        },
        ParamType::File => unreachable!(
            "ParamType::File describes a leading input file, consumed before any numeric \
             parameter or flag"
        ),
    }
}

/// legacy: the `%lf`-style read shared by [`ParamType::Double`] and
/// [`ParamType::Int`] (see that type's doc on why they parse
/// identically), producing `"Cannot read parameter {legacy_index}
/// [{token}]: brkpnt_files not permitted."` on failure.
fn parse_as_f64_or_cannot_read(token: &str, legacy_index: usize) -> Result<f64> {
    token.parse().map_err(|_| ParamsError::CannotReadParameter {
        legacy_index,
        token: token.to_string(),
    })
}

fn check_range(value: f64, lo: f64, hi: f64, paramno: usize) -> Result<()> {
    if value < lo || value > hi {
        return Err(ParamsError::ValueOutOfRange {
            paramno,
            value,
            lo,
            hi,
        });
    }
    Ok(())
}

fn check_file_openable(path: &str) -> Result<()> {
    std::fs::File::open(path)
        .map(|_| ())
        .map_err(|source| ParamsError::CannotOpenFile {
            path: path.to_string(),
            source,
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::CommandSpec;
    use std::io::Write;

    fn existing_file() -> tempfile::NamedTempFile {
        tempfile::NamedTempFile::new().unwrap()
    }

    #[test]
    fn valid_command_parses() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [infile.path().to_str().unwrap(), "out.wav", "0.5"];
        let parsed = parse(&spec, &args).unwrap();
        assert_eq!(parsed.infiles, vec![infile.path().to_str().unwrap()]);
        assert_eq!(parsed.outfile.as_deref(), Some("out.wav"));
        assert!(matches!(parsed.params[0], ParamValue::Number(v) if v == 0.5));
    }

    #[test]
    fn missing_outfile_and_gain_is_insufficient_cmdline_parameters() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [infile.path().to_str().unwrap()];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::InsufficientCmdlineParameters)
        ));
    }

    #[test]
    fn missing_just_gain_is_insufficient_parameters() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [infile.path().to_str().unwrap(), "out.wav"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::InsufficientParameters)
        ));
    }

    #[test]
    fn extra_trailing_word_is_too_many_parameters() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [infile.path().to_str().unwrap(), "out.wav", "0.5", "extra"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::TooManyParameters)
        ));
    }

    #[test]
    fn nonexistent_infile_is_cannot_open_file() {
        let spec = CommandSpec::modify_loudness_gain();
        let args = ["/no/such/infile.wav", "out.wav", "0.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::CannotOpenFile { .. })
        ));
    }

    #[test]
    fn gain_above_range_is_value_out_of_range() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [infile.path().to_str().unwrap(), "out.wav", "99999"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange {
                paramno: 1,
                lo: 0.0,
                hi: 32767.0,
                ..
            })
        ));
    }

    #[test]
    fn negative_gain_is_value_out_of_range() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-1"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 1, .. })
        ));
    }

    #[test]
    fn unparseable_gain_is_retried_as_a_breakpoint_file_and_fails_to_open() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [infile.path().to_str().unwrap(), "out.wav", "abc"];
        let err = parse(&spec, &args).unwrap_err();
        assert!(matches!(err, ParamsError::Breakpoint(_)));
        assert_eq!(err.to_string(), "Can't open brkpntfile abc to read data.");
    }

    #[test]
    fn unparseable_gain_that_is_a_real_breakpoint_file_succeeds() {
        let infile = existing_file();
        let mut brk = tempfile::NamedTempFile::new().unwrap();
        writeln!(brk, "0.0 0.5\n1.0 0.8").unwrap();
        let spec = CommandSpec::modify_loudness_gain();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            brk.path().to_str().unwrap(),
        ];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.params[0], ParamValue::Breakpoint(_)));
    }

    #[test]
    fn normalise_with_no_flag_parses() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(parsed.flags.is_empty());
    }

    #[test]
    fn normalise_with_attached_flag_value_parses() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-l0.5"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.flags[&'l'], ParamValue::Number(v) if v == 0.5));
    }

    #[test]
    fn normalise_flag_above_range_is_value_out_of_range_with_paramno_1() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-l1.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange {
                paramno: 1,
                value: 1.5,
                ..
            })
        ));
    }

    #[test]
    fn normalise_flag_unparseable_value_is_cannot_read_parameter_not_a_breakpoint_attempt() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-labc"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::CannotReadParameter {
                legacy_index: 2,
                ..
            })
        ));
    }

    #[test]
    fn normalise_bare_flag_with_no_value_is_option_value_missing() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-l"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::OptionValueMissing('l'))
        ));
    }

    #[test]
    fn normalise_space_separated_flag_value_is_also_option_value_missing() {
        // legacy: "-l 0.5" is a bare "-l" (missing value) followed by
        // a separate, unrecognised "0.5" token -- but OptionValueMissing
        // is reported first, since "-l" is scanned before "0.5".
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-l", "0.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::OptionValueMissing('l'))
        ));
    }

    #[test]
    fn normalise_unknown_flag_is_unknown_flag() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-x0.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::UnknownFlag('x'))
        ));
    }

    #[test]
    fn normalise_leftover_non_flag_token_is_unknown_parameter() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "0.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::UnknownParameter(ref s)) if s == "0.5"
        ));
    }

    #[test]
    fn force_level_mode_4_behaves_the_same_as_normalise() {
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_force_level();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-l1.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 1, .. })
        ));
    }

    #[test]
    fn normalise_duplicate_flag_is_duplicate_option_even_when_second_value_is_out_of_range() {
        // legacy: confirmed live for `modify loudness 3 infile outfile
        // -l0.5 -l0.6` (both in range) and, separately, for the
        // pvoc-anal case this test's name describes -- see the module
        // doc.
        let infile = existing_file();
        let spec = CommandSpec::modify_loudness_normalise();
        let args = [infile.path().to_str().unwrap(), "out.wav", "-l0.5", "-l1.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::DuplicateOption('l'))
        ));
    }

    #[test]
    fn pvoc_anal_with_no_flags_parses() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(parsed.flags.is_empty());
    }

    #[test]
    fn pvoc_anal_with_both_flags_in_either_order_parses_as_integers() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();

        let args = [infile.path().to_str().unwrap(), "out.ana", "-c512", "-o2"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.flags[&'c'], ParamValue::Integer(512)));
        assert!(matches!(parsed.flags[&'o'], ParamValue::Integer(2)));

        let args = [infile.path().to_str().unwrap(), "out.ana", "-o2", "-c512"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.flags[&'c'], ParamValue::Integer(512)));
        assert!(matches!(parsed.flags[&'o'], ParamValue::Integer(2)));
    }

    #[test]
    fn pvoc_anal_fractional_points_truncates_toward_zero() {
        // legacy: confirmed live -- `pvoc anal 1 infile outfile
        // -c512.9` produced an analysis file reflecting channel count
        // 512, not 513 -- see `ParamType::Int`'s doc.
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-c512.9"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.flags[&'c'], ParamValue::Integer(512)));
    }

    #[test]
    fn pvoc_anal_points_out_of_range_reports_paramno_1() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-c99999"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange {
                paramno: 1,
                lo: 2.0,
                hi: 32768.0,
                ..
            })
        ));
    }

    #[test]
    fn pvoc_anal_overlap_out_of_range_reports_paramno_2() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-o5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange {
                paramno: 2,
                lo: 1.0,
                hi: 4.0,
                ..
            })
        ));
    }

    #[test]
    fn pvoc_anal_errors_report_in_command_line_order_not_by_paramno() {
        // legacy: confirmed live -- `-c1 -o5` reports Parameter[1]
        // first, `-o5 -c1` reports Parameter[2] first, since both
        // flags are scanned left to right and the first bad one wins.
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();

        let args = [infile.path().to_str().unwrap(), "out.ana", "-c1", "-o5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 1, .. })
        ));

        let args = [infile.path().to_str().unwrap(), "out.ana", "-o5", "-c1"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 2, .. })
        ));
    }

    #[test]
    fn pvoc_anal_unparseable_points_is_cannot_read_parameter_1() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-cabc"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::CannotReadParameter {
                legacy_index: 1,
                ..
            })
        ));
    }

    #[test]
    fn pvoc_anal_unparseable_overlap_is_cannot_read_parameter_2() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-oabc"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::CannotReadParameter {
                legacy_index: 2,
                ..
            })
        ));
    }

    #[test]
    fn pvoc_anal_bare_flag_with_no_value_is_option_value_missing() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-c"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::OptionValueMissing('c'))
        ));
    }

    #[test]
    fn pvoc_anal_unknown_flag_is_unknown_flag() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-x5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::UnknownFlag('x'))
        ));
    }

    #[test]
    fn pvoc_anal_leftover_non_flag_token_is_unknown_parameter() {
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "extra"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::UnknownParameter(ref s)) if s == "extra"
        ));
    }

    #[test]
    fn pvoc_anal_duplicate_flag_is_duplicate_option_even_when_second_value_is_out_of_range() {
        // legacy: confirmed live -- `pvoc anal 1 infile outfile -c512
        // -c99999` reports the duplicate, not the second value's own
        // out-of-range error.
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [
            infile.path().to_str().unwrap(),
            "out.ana",
            "-c512",
            "-c99999",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::DuplicateOption('c'))
        ));
    }

    #[test]
    fn pvoc_anal_duplicate_flag_missing_value_still_reports_option_value_missing_first() {
        // legacy: confirmed live -- `-c512 -c` (bare second occurrence)
        // reports the missing-value error, not DuplicateOption, since
        // legacy's own missing-value check runs inside `get_option_no`,
        // before `get_options`'s duplicate check ever sees it.
        let infile = existing_file();
        let spec = CommandSpec::pvoc_anal();
        let args = [infile.path().to_str().unwrap(), "out.ana", "-c512", "-c"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::OptionValueMissing('c'))
        ));
    }

    #[test]
    fn distort_repeat_with_just_multiplier_parses() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap(), "out.wav", "3"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.params[0], ParamValue::Integer(3)));
        assert!(parsed.flags.is_empty());
    }

    #[test]
    fn distort_repeat_with_option_and_variant_in_order_parses() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-c2",
            "-s1",
        ];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.params[0], ParamValue::Integer(3)));
        assert!(matches!(parsed.flags[&'c'], ParamValue::Integer(2)));
        assert!(matches!(parsed.flags[&'s'], ParamValue::Integer(1)));
    }

    #[test]
    fn distort_repeat_variant_before_option_is_option_out_of_order() {
        // legacy: confirmed live -- `-s1 -c2` fails naming `-c` (the
        // option), not `-s` (the variant already read), since option
        // scanning stops for good once a non-option token is seen.
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-s1",
            "-c2",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::OptionOutOfOrder('c'))
        ));
    }

    #[test]
    fn distort_repeat_missing_just_multiplier_is_insufficient_parameters() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap(), "out.wav"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::InsufficientParameters)
        ));
    }

    #[test]
    fn distort_repeat_just_infile_is_insufficient_parameters_not_insufficient_cmdline() {
        // legacy: confirmed live -- `distort repeat infile` alone fails
        // with "Insufficient parameters on command line.", not
        // "Insufficient cmdline parameters.", since `distort repeat` is
        // UNEQUAL_SNDFILE-classified, unlike `modify loudness`/`pvoc
        // anal` -- see `CommandSpec::unequal_sndfile`'s doc.
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap()];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::InsufficientParameters)
        ));
    }

    #[test]
    fn distort_repeat_multiplier_unparseable_is_retried_as_a_breakpoint_file() {
        let infile = existing_file();
        let mut brk = tempfile::NamedTempFile::new().unwrap();
        writeln!(brk, "0.0 3\n1.0 4").unwrap();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            brk.path().to_str().unwrap(),
        ];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.params[0], ParamValue::Breakpoint(_)));
    }

    #[test]
    fn distort_repeat_multiplier_unparseable_nonexistent_file_is_breakpoint_open_error() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap(), "out.wav", "abc"];
        let err = parse(&spec, &args).unwrap_err();
        assert_eq!(err.to_string(), "Can't open brkpntfile abc to read data.");
    }

    #[test]
    fn distort_repeat_multiplier_out_of_range_is_paramno_1() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap(), "out.wav", "1"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange {
                paramno: 1,
                lo: 2.0,
                hi: 32767.0,
                ..
            })
        ));
    }

    #[test]
    fn distort_repeat_option_out_of_range_is_paramno_2() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap(), "out.wav", "3", "-c99999"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange {
                paramno: 2,
                lo: 1.0,
                hi: 32767.0,
                ..
            })
        ));
    }

    #[test]
    fn distort_repeat_variant_out_of_range_is_paramno_3() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-c2",
            "-s99999",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange {
                paramno: 3,
                lo: 0.0,
                hi: 32767.0,
                ..
            })
        ));
    }

    #[test]
    fn distort_repeat_variant_unparseable_is_cannot_read_parameter_3() {
        // legacy: confirmed live -- `-s`, a plain `Int` (no breakpoint
        // fallback), unlike `multiplier`/`-c` which are `IntOrBreakpoint`.
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-c2",
            "-sabc",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::CannotReadParameter {
                legacy_index: 3,
                ..
            })
        ));
    }

    #[test]
    fn distort_repeat_leftover_non_flag_token_is_unknown_parameter() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-c2",
            "-s1",
            "extra",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::UnknownParameter(ref s)) if s == "extra"
        ));
    }

    #[test]
    fn distort_repeat_duplicate_option_is_duplicate_option() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-c2",
            "-c4",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::DuplicateOption('c'))
        ));
    }

    #[test]
    fn distort_repeat_duplicate_variant_is_duplicate_flag() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-s1",
            "-s2",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::DuplicateFlag('s'))
        ));
    }

    #[test]
    fn distort_repeat_duplicate_variant_missing_value_reports_missing_value_first() {
        // legacy: confirmed live -- `-s1 -s` (bare second occurrence)
        // reports the missing-value error, not DuplicateFlag, mirroring
        // the option phase's own check ordering.
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap(), "out.wav", "3", "-s1", "-s"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::VariantValueMissing('s'))
        ));
    }

    #[test]
    fn distort_repeat_bare_variant_with_no_value_is_variant_value_missing() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [infile.path().to_str().unwrap(), "out.wav", "3", "-c2", "-s"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::VariantValueMissing('s'))
        ));
    }

    #[test]
    fn distort_repeat_unrecognised_variant_flag_is_unknown_variant_flag() {
        let infile = existing_file();
        let spec = CommandSpec::distort_repeat();
        let args = [
            infile.path().to_str().unwrap(),
            "out.wav",
            "3",
            "-c2",
            "-z5",
        ];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::UnknownVariantFlag('z'))
        ));
    }

    #[test]
    fn synth_wave_with_no_flags_parses_all_four_required_params_in_order() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", "440"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(parsed.infiles.is_empty());
        assert!(matches!(parsed.params[0], ParamValue::Integer(44100)));
        assert!(matches!(parsed.params[1], ParamValue::Integer(1)));
        assert!(matches!(parsed.params[2], ParamValue::Number(v) if v == 1.0));
        assert!(matches!(parsed.params[3], ParamValue::Number(v) if v == 440.0));
    }

    #[test]
    fn synth_wave_with_options_and_boolean_variant_parses() {
        let spec = CommandSpec::synth_wave();
        let args = [
            "out.wav", "44100", "1", "1.0", "440", "-a0.5", "-t512", "-f",
        ];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.flags[&'a'], ParamValue::Number(v) if v == 0.5));
        assert!(matches!(parsed.flags[&'t'], ParamValue::Integer(512)));
        assert!(matches!(parsed.flags[&'f'], ParamValue::Present));
    }

    #[test]
    fn synth_wave_boolean_variant_with_attached_value_is_silently_discarded() {
        // legacy: confirmed live -- `-f999` behaves identically to a
        // bare `-f`, since `get_variant_no` never inspects what follows
        // a boolean variant's letter.
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", "440", "-f999"];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.flags[&'f'], ParamValue::Present));
    }

    #[test]
    fn synth_wave_boolean_variant_before_option_is_option_out_of_order() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", "440", "-f", "-a0.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::OptionOutOfOrder('a'))
        ));
    }

    #[test]
    fn synth_wave_duplicate_boolean_variant_is_duplicate_flag() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", "440", "-f", "-f"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::DuplicateFlag('f'))
        ));
    }

    #[test]
    fn synth_wave_zero_required_params_given_is_insufficient_parameters() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::InsufficientParameters)
        ));
    }

    #[test]
    fn synth_wave_no_arguments_at_all_is_insufficient_parameters() {
        // legacy: confirmed live, `synth wave 1` (no outfile, no
        // required params at all) prints "Insufficient parameters on
        // command line.", not "Insufficient cmdline parameters." --
        // found while wiring up WP-1.5's lifecycle, see
        // `CommandSpec::synth_wave`'s `unequal_sndfile` field doc.
        let spec = CommandSpec::synth_wave();
        let args: [&str; 0] = [];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::InsufficientParameters)
        ));
    }

    #[test]
    fn synth_wave_partial_required_params_is_insufficient_parameters_on_cmdline() {
        // legacy: confirmed live -- 1, 2, or 3 of the 4 required params
        // given (but not zero) fails with "Insufficient parameters on
        // cmdline.", not "Insufficient parameters on command line.",
        // since this distinction is invisible with only one required
        // param, as every mode ported before this one has.
        let spec = CommandSpec::synth_wave();
        for args in [
            vec!["out.wav", "44100"],
            vec!["out.wav", "44100", "1"],
            vec!["out.wav", "44100", "1", "1.0"],
        ] {
            assert!(matches!(
                parse(&spec, &args),
                Err(ParamsError::InsufficientParametersOnCmdline)
            ));
        }
    }

    #[test]
    fn synth_wave_required_params_are_numbered_1_through_4() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "999999", "1", "1.0", "440"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 1, .. })
        ));
        let args = ["out.wav", "44100", "99", "1.0", "440"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 2, .. })
        ));
        let args = ["out.wav", "44100", "1", "99999", "440"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 3, .. })
        ));
        let args = ["out.wav", "44100", "1", "1.0", "99999"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 4, .. })
        ));
    }

    #[test]
    fn synth_wave_options_continue_numbering_at_5_and_6() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", "440", "-a1.5"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 5, .. })
        ));
        let args = ["out.wav", "44100", "1", "1.0", "440", "-t99999"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::ValueOutOfRange { paramno: 6, .. })
        ));
    }

    #[test]
    fn synth_wave_freq_unparseable_is_retried_as_a_breakpoint_file() {
        let mut brk = tempfile::NamedTempFile::new().unwrap();
        writeln!(brk, "0.0 440\n1.0 880").unwrap();
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", brk.path().to_str().unwrap()];
        let parsed = parse(&spec, &args).unwrap();
        assert!(matches!(parsed.params[3], ParamValue::Breakpoint(_)));
    }

    #[test]
    fn synth_wave_amp_unparseable_nonexistent_file_is_breakpoint_open_error() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", "440", "-aabc"];
        let err = parse(&spec, &args).unwrap_err();
        assert_eq!(err.to_string(), "Can't open brkpntfile abc to read data.");
    }

    #[test]
    fn synth_wave_leftover_non_flag_token_is_unknown_parameter() {
        let spec = CommandSpec::synth_wave();
        let args = ["out.wav", "44100", "1", "1.0", "440", "-a0.5", "extra"];
        assert!(matches!(
            parse(&spec, &args),
            Err(ParamsError::UnknownParameter(ref s)) if s == "extra"
        ));
    }
}
