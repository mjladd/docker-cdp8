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

//! `sndinfo smptime infile samplecnt [-g]`: converts a sample count to
//! a duration, in the infile's own sample rate and (unless `-g` is
//! given) channel count. legacy: `legacy/dev/sndinfo/compare.c`'s
//! `case(INFO_SAMPTOTIME)` block.
//!
//! Scope: [`cdp_sf::FileKind::Wave`] infiles only -- `INFO_SAMPTOTIME`
//! is `SNDFILES_ONLY` in `ap_sndinfo.c`'s `setup_process_logic`
//! (unlike `props`/`len`'s `ALL_FILES`), confirmed live: a real
//! analysis-family infile (`docs/manual/data/capm.ana`) fails with
//! [`super::WRONG_FILETYPE`] rather than any per-kind branch.
//!
//! `-g` ("sample count is count of GROUPED samples") means `samplecnt`
//! is already a per-channel frame count; without it, `samplecnt` is a
//! raw total-samples count and gets divided by the channel count
//! first (integer division, confirmed live against `clip5-all.wav`,
//! a stereo file: the same `samplecnt` produces a different `TIME`
//! with and without `-g`, in the ratio of its channel count).

use crate::sndinfo::len::format_duration;
use cdp_params::{ParamValue, ParsedCommand};
use cdp_sf::SoundFile;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo smptime` (captured live via `spec/usage/sndinfo/
/// smptime.txt`, WP-0.2). Bundles [`GREETING`] at the front, the same
/// self-contained-fixture style `props::USAGE`/`len::USAGE` use.
pub const USAGE: &str = "CDP Release 7.1 2016
CONVERT SAMPLE COUNT TO TIME IN SOUNDFILE

USAGE: sndreport smptime infile samplecnt [-g]

-g   sample count is count of GROUPED samples
     e.g. stereo file: sample-PAIRS counted.
";

/// legacy: `case(INFO_SAMPTOTIME)` in `compare.c`. `sf` is the already
/// -open infile (see [`super::open_sound_infile`]); `parsed`'s single
/// required parameter is `samplecnt` ([`cdp_params::CommandSpec::
/// sndinfo_smptime`]'s `Int`), and its `-g` flag (if present) is
/// [`ParamValue::Present`]. Returns the complete output text,
/// including its own trailing newline (legacy: `strcat(errstr,"\n")`
/// after the [`format_duration`] text, which does not end in one
/// itself).
pub fn format_smptime(sf: &SoundFile, parsed: &ParsedCommand) -> String {
    let ParamValue::Integer(raw_samples) = parsed.params[0] else {
        unreachable!("CommandSpec::sndinfo_smptime's one param is ParamType::Int")
    };
    let grouped = parsed.flags.contains_key(&'g');
    let samples = if grouped {
        raw_samples
    } else {
        // legacy: `dz->iparam[INFO_SAMPS] /= chans` -- integer
        // division, truncating.
        raw_samples / sf.fmt.channels as i64
    };
    let secs = samples as f64 * (1.0 / sf.fmt.sample_rate as f64);
    format!("TIME {}\n", format_duration(secs))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdp_params::{CommandSpec, parse};

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    fn parsed_for(sf: &SoundFile, args: &[&str]) -> ParsedCommand {
        let spec = CommandSpec::sndinfo_smptime(sf.sample_count() as f64);
        parse(&spec, args).unwrap()
    }

    #[test]
    fn marimba_wav_matches_the_real_legacy_sndinfo_smptime_output() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let sf = SoundFile::open(&path).unwrap();
        let parsed = parsed_for(&sf, &[&path, "44174"]);
        assert_eq!(format_smptime(&sf, &parsed), "TIME 1.001678 secs \n");
    }

    #[test]
    fn clip5_all_wav_stereo_ungrouped_matches_the_real_legacy_sndinfo_smptime_output() {
        let path = repo_path("docs/manual/sounds/clip5-all.wav");
        let sf = SoundFile::open(&path).unwrap();
        let parsed = parsed_for(&sf, &[&path, "18610752"]);
        assert_eq!(
            format_smptime(&sf, &parsed),
            "TIME 3 mins 31.006259 secs \n"
        );
    }

    #[test]
    fn clip5_all_wav_stereo_grouped_matches_the_real_legacy_sndinfo_smptime_output() {
        let path = repo_path("docs/manual/sounds/clip5-all.wav");
        let sf = SoundFile::open(&path).unwrap();
        let parsed = parsed_for(&sf, &[&path, "18610752", "-g"]);
        assert_eq!(format_smptime(&sf, &parsed), "TIME 7 mins 2.012517 secs \n");
    }

    #[test]
    fn samplecnt_out_of_range_is_rejected_against_the_infiles_own_raw_sample_total() {
        let path = repo_path("docs/manual/sounds/clip5-all.wav");
        let sf = SoundFile::open(&path).unwrap();
        let spec = CommandSpec::sndinfo_smptime(sf.sample_count() as f64);
        let err = parse(&spec, &[&path, "99999999999"]).unwrap_err();
        assert!(matches!(
            err,
            cdp_params::ParamsError::ValueOutOfRange { hi, .. } if hi == 18_610_752.0
        ));
    }

    #[test]
    fn unrecognised_flag_reports_the_no_options_message() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let sf = SoundFile::open(&path).unwrap();
        let spec = CommandSpec::sndinfo_smptime(sf.sample_count() as f64);
        let err = parse(&spec, &[&path, "44174", "-z"]).unwrap_err();
        assert!(matches!(
            err,
            cdp_params::ParamsError::UnknownFlagNoOptions('z')
        ));
    }

    #[test]
    fn trailing_plain_word_reports_unknown_parameter_in_variant_phase() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let sf = SoundFile::open(&path).unwrap();
        let spec = CommandSpec::sndinfo_smptime(sf.sample_count() as f64);
        let err = parse(&spec, &[&path, "44174", "extra"]).unwrap_err();
        assert!(matches!(
            err,
            cdp_params::ParamsError::UnknownParameterInVariantPhase(ref s) if s == "extra"
        ));
    }
}
