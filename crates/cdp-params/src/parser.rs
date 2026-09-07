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

use crate::error::{ParamsError, Result};
use crate::spec::{CommandSpec, ParamType};
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
    Breakpoint(BreakpointTable),
}

#[derive(Debug, Clone)]
pub struct ParsedCommand {
    pub infiles: Vec<String>,
    pub outfile: String,
    /// The required positional parameters, in order (empty for a mode
    /// whose [`CommandSpec::params`] is empty -- see
    /// [`CommandSpec::flags`]'s doc on why this crate does not yet
    /// support a mode with both).
    pub params: Vec<ParamValue>,
    /// The optional flags that were actually present on the command
    /// line, keyed by letter. A flag [`CommandSpec::flags`] lists but
    /// the caller did not supply is simply absent here, matching
    /// legacy leaving the parameter at its own default (not modelled
    /// yet -- see `docs/migration/STATUS.md`).
    pub flags: BTreeMap<char, ParamValue>,
}

/// Parses `args` (everything after the process name, subcommand and
/// mode number) against `spec`.
pub fn parse(spec: &CommandSpec, args: &[&str]) -> Result<ParsedCommand> {
    let min_needed = spec.infile_count + 1; // + the output filename
    if args.len() < min_needed {
        return Err(ParamsError::InsufficientCmdlineParameters);
    }

    let mut infiles = Vec::with_capacity(spec.infile_count);
    for &arg in &args[..spec.infile_count] {
        check_file_openable(arg)?;
        infiles.push(arg.to_string());
    }
    let outfile = args[spec.infile_count].to_string();
    let rest = &args[spec.infile_count + 1..];

    if spec.params.is_empty() {
        let flags = parse_flags(spec, rest)?;
        return Ok(ParsedCommand {
            infiles,
            outfile,
            params: vec![],
            flags,
        });
    }

    if rest.len() < spec.params.len() {
        return Err(ParamsError::InsufficientParameters);
    }
    if rest.len() > spec.params.len() {
        return Err(ParamsError::TooManyParameters);
    }

    let mut params = Vec::with_capacity(spec.params.len());
    for (i, (&token, param_type)) in rest.iter().zip(&spec.params).enumerate() {
        let paramno = i + 1; // legacy: 1-based in "Parameter[%d]"
        params.push(parse_param(token, *param_type, paramno)?);
    }

    Ok(ParsedCommand {
        infiles,
        outfile,
        params,
        flags: BTreeMap::new(),
    })
}

/// legacy: the optional-flag scanning loop for a mode with no
/// required positional parameters -- see the module doc's mode 3/4
/// examples for every error case confirmed here.
fn parse_flags(spec: &CommandSpec, rest: &[&str]) -> Result<BTreeMap<char, ParamValue>> {
    let mut flags = BTreeMap::new();
    for &token in rest {
        let Some(after_dash) = token.strip_prefix('-') else {
            return Err(ParamsError::UnknownParameter(token.to_string()));
        };
        let letter = after_dash
            .chars()
            .next()
            .ok_or_else(|| ParamsError::UnknownParameter(token.to_string()))?;
        let flag_spec = spec
            .flags
            .iter()
            .find(|f| f.letter == letter)
            .ok_or(ParamsError::UnknownFlag(letter))?;
        let value_str = &after_dash[letter.len_utf8()..];
        if value_str.is_empty() {
            return Err(ParamsError::OptionValueMissing(letter));
        }
        let value = parse_param(
            value_str,
            flag_spec.value_type,
            flag_spec.range_check_paramno,
        )?;
        flags.insert(letter, value);
    }
    Ok(flags)
}

fn parse_param(token: &str, param_type: ParamType, paramno: usize) -> Result<ParamValue> {
    match param_type {
        ParamType::DoubleOrBreakpoint { lo, hi } => match token.parse::<f64>() {
            Ok(value) => Ok(check_range(value, lo, hi, paramno)?),
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
            let value: f64 = token
                .parse()
                .map_err(|_| ParamsError::CannotReadParameter {
                    legacy_index,
                    token: token.to_string(),
                })?;
            check_range(value, lo, hi, paramno)
        }
        ParamType::File => unreachable!(
            "ParamType::File describes a leading input file, consumed before any numeric \
             parameter or flag"
        ),
    }
}

fn check_range(value: f64, lo: f64, hi: f64, paramno: usize) -> Result<ParamValue> {
    if value < lo || value > hi {
        return Err(ParamsError::ValueOutOfRange {
            paramno,
            value,
            lo,
            hi,
        });
    }
    Ok(ParamValue::Number(value))
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
        assert_eq!(parsed.outfile, "out.wav");
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
}
