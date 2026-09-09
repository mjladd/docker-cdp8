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

//! `sndinfo timesmp infile time [-g]`: converts a duration to a sample
//! count -- [`super::smptime`]'s inverse. legacy: `legacy/dev/sndinfo/
//! compare.c`'s `case(INFO_TIMETOSAMP)` block.
//!
//! Scope and `-g` semantics are the same as [`super::smptime`]: see
//! its module doc. `time`'s dynamic range upper bound is the infile's
//! own duration in seconds ([`crate::sndinfo::len::wave_duration_secs`],
//! the same formula `sndinfo len`'s `Wave` branch uses), confirmed
//! live: `sndinfo timesmp marimba.wav 1.0017` (just over that file's
//! 1.001678-second duration) reports `"...out of range (0.000000 to
//! 1.001678)"`.

use cdp_params::{ParamValue, ParsedCommand};
use cdp_sf::SoundFile;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo timesmp` (captured live via `spec/usage/sndinfo/
/// timesmp.txt`, WP-0.2). Bundles [`GREETING`] at the front, the same
/// self-contained-fixture style `props::USAGE`/`len::USAGE` use.
pub const USAGE: &str = "CDP Release 7.1 2016
CONVERT TIME TO SAMPLE COUNT IN SOUNDFILE

USAGE: sndreport timesmp infile time [-g]

-g   sample count is count of GROUPED samples
     e.g. stereo file: sample-PAIRS counted.
";

/// legacy: `case(INFO_TIMETOSAMP)` in `compare.c`. `sf` is the
/// already-open infile (see [`super::open_sound_infile`]); `parsed`'s
/// single required parameter is `time` ([`cdp_params::CommandSpec::
/// sndinfo_timesmp`]'s `Double`), and its `-g` flag (if present) is
/// [`ParamValue::Present`]. Returns the complete output text,
/// including its own trailing newline.
pub fn format_timesmp(sf: &SoundFile, parsed: &ParsedCommand) -> String {
    let ParamValue::Number(time) = parsed.params[0] else {
        unreachable!("CommandSpec::sndinfo_timesmp's one param is ParamType::Double")
    };
    // legacy: `n = round(dz->param[INFO_TIME] * sr)` -- `round()` is
    // `lround()` (`readdata.c`'s own `#define`), round-half-away-from-
    // zero, matching `f64::round`.
    let mut n = (time * sf.fmt.sample_rate as f64).round() as i64;
    let grouped = parsed.flags.contains_key(&'g');
    if !grouped {
        n *= sf.fmt.channels as i64;
    }
    format!("SAMPLE {n}\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sndinfo::len::wave_duration_secs;
    use cdp_params::{CommandSpec, parse};

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    fn parsed_for(sf: &SoundFile, args: &[&str]) -> ParsedCommand {
        let spec = CommandSpec::sndinfo_timesmp(wave_duration_secs(sf));
        parse(&spec, args).unwrap()
    }

    #[test]
    fn marimba_wav_matches_the_real_legacy_sndinfo_timesmp_output() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let sf = SoundFile::open(&path).unwrap();
        let parsed = parsed_for(&sf, &[&path, "1.0"]);
        assert_eq!(format_timesmp(&sf, &parsed), "SAMPLE 44100\n");
    }

    #[test]
    fn marimba_wav_at_its_own_full_duration_matches_the_real_legacy_sndinfo_timesmp_output() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let sf = SoundFile::open(&path).unwrap();
        let parsed = parsed_for(&sf, &[&path, "1.001678"]);
        assert_eq!(format_timesmp(&sf, &parsed), "SAMPLE 44174\n");
    }

    #[test]
    fn clip5_all_wav_stereo_ungrouped_matches_the_real_legacy_sndinfo_timesmp_output() {
        let path = repo_path("docs/manual/sounds/clip5-all.wav");
        let sf = SoundFile::open(&path).unwrap();
        let parsed = parsed_for(&sf, &[&path, "211.006259"]);
        assert_eq!(format_timesmp(&sf, &parsed), "SAMPLE 18610752\n");
    }

    #[test]
    fn clip5_all_wav_stereo_grouped_matches_the_real_legacy_sndinfo_timesmp_output() {
        let path = repo_path("docs/manual/sounds/clip5-all.wav");
        let sf = SoundFile::open(&path).unwrap();
        let parsed = parsed_for(&sf, &[&path, "211.006259", "-g"]);
        assert_eq!(format_timesmp(&sf, &parsed), "SAMPLE 9305376\n");
    }

    #[test]
    fn time_out_of_range_is_rejected_against_the_infiles_own_duration() {
        let path = repo_path("docs/manual/sounds/marimba.wav");
        let sf = SoundFile::open(&path).unwrap();
        let spec = CommandSpec::sndinfo_timesmp(wave_duration_secs(&sf));
        let err = parse(&spec, &[&path, "1.0017"]).unwrap_err();
        let cdp_params::ParamsError::ValueOutOfRange { hi, .. } = err else {
            panic!("expected ValueOutOfRange, got {err:?}");
        };
        assert!((hi - 1.001678).abs() < 1e-6);
    }
}
