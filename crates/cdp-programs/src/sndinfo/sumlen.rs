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

//! `sndinfo sumlen infile infile2 [infile3..] [-ssplicelen]`: sums the
//! duration of two or more sound files, subtracting one splice length
//! per file. legacy: `legacy/dev/sndinfo/compare.c`'s
//! `case(INFO_TIMESUM)` block.
//!
//! `INFO_TIMESUM` is `MANY_SNDFILES`, like [`super::lens`], but --
//! unlike `lens` -- is *not* one of the processes
//! `set_legal_infile_structure` (`ap_sndinfo.c`) sets
//! `dz->has_otherfile = TRUE` for. Confirmed live this means every
//! infile after the first goes through the strict, generic
//! property-comparison check every other `has_otherfile == FALSE`
//! multi-infile command shares
//! (`open_checktype_getsize_and_compareheader`/`handle_other_infile`,
//! `legacy/dev/cdp2k/readfiles.c`): its sample rate and channel count
//! must match infile 1's exactly, or the run fails with
//! `"Incompatible sample-rate in input file %s."` / `"Incompatible
//! channel-count in input file %s."` (`DATA_ERROR`) before any
//! duration is even computed. Channel-count is confirmed live
//! (`sndinfo sumlen marimba.wav clip5-all.wav`, mono then stereo);
//! sample-rate is read from the same source code block (`readfiles.c`
//! lines checking `srate` immediately before `channels`) but not
//! independently confirmed live, since every sound file in this
//! repository's corpus (`docs/manual`) shares one sample rate
//! (44100Hz) -- ported as observed rather than assumed, matching this
//! crate's usual practice for source-derived-but-corpus-unconfirmable
//! branches (e.g. `len::format_duration`'s `hrs` branch).
//!
//! This equal-properties guarantee is also why `case(INFO_TIMESUM)`'s
//! duration sum can safely reuse `dz->infile`'s *own* (i.e. infile 1's)
//! `channels`/`srate` for every term in the loop, rather than
//! re-reading each file's own header the way `case(INFO_TIMELIST)`
//! does (see [`super::lens`]'s doc) -- by the time the loop runs,
//! every infile's `channels`/`srate` are already known to be identical
//! to infile 1's, so [`format_sumlen`] computing each file's own
//! duration via [`super::len::wave_duration_secs`] produces the same
//! result either way. Confirmed live across three identical-file sums
//! (`sndinfo sumlen marimba.wav marimba.wav marimba.wav`) that the
//! splice length (`TIMESUM_SPLEN`, default 15ms, `MS_TO_SECS ==
//! 0.001`) is subtracted once *per infile*, not once total: 3 ×
//! `1.001678` − 3 × `0.015` = `2.960034`, the real legacy output.
//!
//! `-ssplicelen` is a genuine [`cdp_params::CommandSpec`] flag
//! (`ap_sndinfo.c`: `set_vflgs(ap,"s",1,"d",...)`, range `[0.0,
//! 2000.0]` ms, default `15.0`ms -- `legacy/dev/include/sndinfo.h`,
//! `legacy/dev/cdp2k/tklib1.c`), so [`parse_splice_ms`] reuses
//! `cdp_params::parse` directly rather than hand-rolling flag parsing
//! the way [`super::timediff`]/[`super::lens`] do for their own
//! infile-list quirks: every one of its error cases (out-of-range,
//! unparseable, duplicate, missing value, unrecognised flag) is
//! confirmed live to already match this crate's existing
//! `cdp_params::ParamsError` text exactly, with `infile_count: 0` and
//! `has_outfile: false` so `parse` only ever sees the flag-region
//! tokens this module hands it, never the infile list itself (handled
//! separately, the same way [`super::split_infile_tokens`] is --
//! see that function's doc for why the infile count itself cannot go
//! through `cdp_params::parse`).
//!
//! At least two infiles are required, with the exact same two-message
//! split [`super::lens`] has (`sndinfo sumlen infile` → `"Insufficient
//! parameters on command line."`; `sndinfo sumlen infile -s5` →
//! [`super::INSUFFICIENT_INFILES`]) -- confirmed live for both.

use super::len::{format_duration, wave_duration_secs};
use cdp_core::{CdpError, ExitCategory};
use cdp_params::{CommandSpec, OptionFlag, ParamType, ParamValue, parse};
use cdp_sf::SoundFile;

/// legacy: same `argc<4` greeting rule as `sndinfo lens` -- see that
/// module's [`super::lens::GREETING`] doc.
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo sumlen` (captured live via `spec/usage/sndinfo/
/// sumlen.txt`, WP-0.2). See `super::props::USAGE`'s doc for why this
/// bundles [`GREETING`] at the front and carries only one trailing
/// newline (not the fixture's own extra trailing blank lines).
pub const USAGE: &str = "CDP Release 7.1 2016
SUM DURATIONS OF SEVERAL SNDFILING-SYSTEM FILES

USAGE: sndreport sumlen infile infile2 [infile3..] [-ssplicelen]

      SPLICELEN is in milliseconds. (Default: 15ms)
";

/// legacy: `TIMESUM_DEFAULT_SPLEN` (`legacy/dev/include/sndinfo.h`).
pub const DEFAULT_SPLICE_MS: f64 = 15.0;

/// legacy: `ap->lo[TIMESUM_SPLEN]`/`ap->hi[TIMESUM_SPLEN]`
/// (`legacy/dev/cdp2k/tklib1.c`) -- a fixed range, unlike
/// `super::smptime`/`super::timesmp`'s dynamic per-infile bound, so
/// this needs no infile open first.
fn splice_flag_spec() -> CommandSpec {
    CommandSpec {
        infile_count: 0,
        has_outfile: false,
        params: vec![],
        flags: vec![OptionFlag {
            letter: 's',
            value_type: ParamType::Double {
                lo: 0.0,
                hi: 2000.0,
                legacy_index: 1,
            },
            range_check_paramno: 1,
        }],
        variants: vec![],
        unequal_sndfile: false,
    }
}

/// Parses the optional `-ssplicelen` flag out of `args` (everything
/// left on the command line once every infile token has already been
/// removed by the caller -- see this module's doc), returning
/// [`DEFAULT_SPLICE_MS`] when it is absent.
pub fn parse_splice_ms(args: &[&str]) -> Result<f64, CdpError> {
    let parsed = parse(&splice_flag_spec(), args)?;
    Ok(match parsed.flags.get(&'s') {
        Some(ParamValue::Number(v)) => *v,
        Some(_) => unreachable!("splice_flag_spec's -s flag is ParamType::Double"),
        None => DEFAULT_SPLICE_MS,
    })
}

/// Opens every path in `paths` in order: the first via
/// [`super::open_sound_infile`], every subsequent one via
/// [`super::open_other_sound_infile`] plus the sample-rate/
/// channel-count consistency check against infile 1 -- see this
/// module's doc for both.
pub fn open_infiles(paths: &[&str]) -> Result<Vec<SoundFile>, CdpError> {
    let mut files: Vec<SoundFile> = Vec::with_capacity(paths.len());
    for (i, &path) in paths.iter().enumerate() {
        if i == 0 {
            files.push(super::open_sound_infile(path)?);
            continue;
        }
        let sf = super::open_other_sound_infile(path)?;
        let first = &files[0];
        if sf.fmt.sample_rate != first.fmt.sample_rate {
            return Err(CdpError::new(
                ExitCategory::DataError,
                format!("Incompatible sample-rate in input file {path}."),
            ));
        }
        if sf.fmt.channels != first.fmt.channels {
            return Err(CdpError::new(
                ExitCategory::DataError,
                format!("Incompatible channel-count in input file {path}."),
            ));
        }
        files.push(sf);
    }
    Ok(files)
}

/// legacy: `case(INFO_TIMESUM)` in `compare.c`. Returns the complete
/// output text, including its own trailing newline.
pub fn format_sumlen(files: &[SoundFile], splice_ms: f64) -> String {
    let splice_secs = splice_ms * 0.001; // legacy: MS_TO_SECS
    let mut secs = 0.0;
    for sf in files {
        secs += wave_duration_secs(sf);
        secs -= splice_secs;
    }
    format!("TOTAL TIME {}\n", format_duration(secs))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    #[test]
    fn no_flag_uses_the_default_splice_length() {
        assert_eq!(parse_splice_ms(&[]).unwrap(), DEFAULT_SPLICE_MS);
    }

    #[test]
    fn explicit_flag_overrides_the_default() {
        assert_eq!(parse_splice_ms(&["-s0"]).unwrap(), 0.0);
        assert_eq!(parse_splice_ms(&["-s5"]).unwrap(), 5.0);
    }

    #[test]
    fn out_of_range_splice_is_value_out_of_range() {
        assert!(matches!(parse_splice_ms(&["-s2500"]), Err(CdpError { .. })));
    }

    #[test]
    fn two_marimba_copies_with_default_splice_matches_the_real_legacy_sndinfo_sumlen_output() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let files = open_infiles(&[&path, &path]).unwrap();
        let splice_ms = parse_splice_ms(&[]).unwrap();
        assert_eq!(
            format_sumlen(&files, splice_ms),
            "TOTAL TIME 1.973356 secs \n"
        );
    }

    #[test]
    fn two_marimba_copies_with_zero_splice_is_exactly_double_the_single_duration() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let files = open_infiles(&[&path, &path]).unwrap();
        let splice_ms = parse_splice_ms(&["-s0"]).unwrap();
        assert_eq!(
            format_sumlen(&files, splice_ms),
            "TOTAL TIME 2.003356 secs \n"
        );
    }

    #[test]
    fn three_marimba_copies_with_default_splice_subtracts_the_splice_once_per_infile() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let files = open_infiles(&[&path, &path, &path]).unwrap();
        let splice_ms = parse_splice_ms(&[]).unwrap();
        assert_eq!(
            format_sumlen(&files, splice_ms),
            "TOTAL TIME 2.960034 secs \n"
        );
    }

    #[test]
    fn mismatched_channel_counts_are_rejected() {
        // legacy: confirmed live -- see this module's doc.
        let marimba = repo_path("docs/manual/sounds/marimba.wav"); // mono
        let clip5 = repo_path("docs/manual/sounds/clip5-all.wav"); // stereo
        let err = open_infiles(&[&marimba, &clip5]).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(
            err.message,
            format!("Incompatible channel-count in input file {clip5}.")
        );
    }

    #[test]
    fn open_infiles_rejects_a_non_sound_first_infile_with_the_generic_wrong_filetype_text() {
        let path1 = repo_path("docs/manual/data/capm.ana");
        let err = open_infiles(&[&path1, "/nonexistent/does-not-exist.wav"]).unwrap_err();
        assert_eq!(err.category, ExitCategory::UsageOnly);
        assert_eq!(err.message, super::super::WRONG_FILETYPE);
    }

    #[test]
    fn open_infiles_reports_the_infile2_specific_open_error_for_a_nonexistent_second_infile() {
        let path1 = repo_path("docs/manual/sounds/marimba.wav");
        let err = open_infiles(&[&path1, "/nonexistent/does-not-exist.wav"]).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(
            err.message,
            "cannot open input file /nonexistent/does-not-exist.wav to read data."
        );
    }
}
