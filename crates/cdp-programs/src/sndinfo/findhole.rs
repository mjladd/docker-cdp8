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

//! `sndinfo findhole`: the largest low-level hole in a sound file.
//!
//! legacy: `case(INFO_FINDHOLE)` in `legacy/dev/sndinfo/compare.c`.
//!
//! A hole is a run of consecutive samples whose magnitude stays below a
//! threshold. The command reports the longest such run, as a duration and
//! a start time.
//!
//! Two details of legacy's own loop are ported as written rather than
//! tidied, because both are observable:
//!
//! 1. The run is counted in raw samples, not frames, so an interleaved
//!    stereo file counts both channels' samples in one run. Only at the
//!    end is the count divided by the channel count, with integer
//!    division.
//! 2. A hole that runs to the end of the file is never counted. legacy
//!    updates `maxholesize` only when a hole *ends*, and the loop has no
//!    check after it finishes. A file that fades to silence and stops
//!    therefore reports the largest hole before that final silence, not
//!    the final silence itself.
//!
//! The threshold is `dz->param[HOLE_THRESH] * (double)F_MAXSAMP`, and
//! `F_MAXSAMP` is `1.0` in both `globcon.h` and `columns.h`, so the flag
//! value is used directly against normalised sample magnitudes.

use cdp_core::CdpError;
use cdp_params::{ParamValue, ParsedCommand};

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = r"CDP Release 7.1 2016
FIND LARGEST LOW LEVEL HOLE IN A SOUNDFILE

USAGE: sndreport findhole infile [-tthreshold]

THRESHOLD  hole only if level falls and stays below threshold (default: 0).
";

/// legacy: `default_val[HOLE_THRESH] = 0.0` (`tklib1.c`). A threshold of
/// zero means no sample is ever below it, so an unflagged run always
/// reports a hole size of zero.
const DEFAULT_THRESHOLD: f64 = 0.0;

pub fn findhole(parsed: &ParsedCommand) -> Result<(), CdpError> {
    let infile_path = &parsed.infiles[0];
    let sf = super::open_sound_infile(infile_path)?;

    let threshold = match parsed.flags.get(&'t') {
        Some(ParamValue::Number(value)) => *value,
        Some(_) => unreachable!("sndinfo_findhole's -t flag is ParamType::Double"),
        None => DEFAULT_THRESHOLD,
    };

    let samples = sf.samples_f32().map_err(CdpError::from)?;
    let chans = sf.fmt.channels as usize;
    let inverse_sr = 1.0 / f64::from(sf.fmt.sample_rate);

    let (max_size, max_samp) = largest_hole(&samples, threshold);

    // legacy: integer division by the channel count, then scaled by
    // 1/sr, exactly as `maxholelen`/`maxholetime` are computed.
    let hole_length = (max_size / chans) as f64 * inverse_sr;
    let hole_time = (max_samp / chans) as f64 * inverse_sr;

    println!("Maximum holesize is {hole_length:.6} at time {hole_time:.6}");
    Ok(())
}

/// The size and start of the longest run of samples below `threshold`,
/// in raw interleaved samples.
///
/// A run that reaches the end of `samples` is deliberately not counted,
/// matching legacy's loop. See this module's doc.
fn largest_hole(samples: &[f32], threshold: f64) -> (usize, usize) {
    let (mut hole_size, mut hole_samp) = (0usize, 0usize);
    let (mut max_size, mut max_samp) = (0usize, 0usize);

    for (index, &sample) in samples.iter().enumerate() {
        let below = f64::from(sample).abs() < threshold;
        if hole_size == 0 {
            if below {
                hole_size = 1;
                hole_samp = index;
            }
        } else if below {
            hole_size += 1;
        } else {
            if hole_size > max_size {
                max_size = hole_size;
                max_samp = hole_samp;
            }
            hole_size = 0;
        }
    }
    (max_size, max_samp)
}

#[cfg(test)]
mod tests {
    use super::{DEFAULT_THRESHOLD, largest_hole};

    #[test]
    fn a_threshold_of_zero_finds_nothing() {
        // legacy: |sample| < 0.0 is never true, so the default threshold
        // always reports a hole size of zero.
        assert_eq!(largest_hole(&[0.0, 0.0, 0.0], 0.0), (0, 0));
    }

    #[test]
    fn reports_the_longest_run_and_where_it_starts() {
        //          0    1    2    3    4    5    6    7
        let s = [0.9, 0.0, 0.9, 0.0, 0.0, 0.0, 0.9, 0.9];
        assert_eq!(largest_hole(&s, 0.5), (3, 3));
    }

    #[test]
    fn a_run_reaching_the_end_of_the_file_is_not_counted() {
        // legacy updates the maximum only when a hole ends, and its loop
        // has no check afterwards. Ported as written: see the module doc.
        let s = [0.9, 0.0, 0.9, 0.0, 0.0, 0.0, 0.0];
        assert_eq!(
            largest_hole(&s, 0.5),
            (1, 1),
            "the single-sample hole at index 1 wins, because the four-sample \
             run at the end never closes"
        );
    }

    #[test]
    fn the_first_of_two_equal_runs_wins() {
        // legacy uses a strict `>` comparison, so a later run of the same
        // length does not replace an earlier one.
        let s = [0.9, 0.0, 0.0, 0.9, 0.0, 0.0, 0.9];
        assert_eq!(largest_hole(&s, 0.5), (2, 1));
    }

    #[test]
    fn counts_raw_samples_rather_than_frames() {
        // An interleaved stereo pair below the threshold counts as two.
        let s = [0.9, 0.9, 0.0, 0.0, 0.9, 0.9];
        assert_eq!(largest_hole(&s, 0.5), (2, 2));
    }

    /// The expected values come from a live `legacy` run through
    /// `cdp8-postmerge`, not from this module's own output:
    ///
    /// ```text
    /// $ sndinfo findhole marimba.wav -t0.5
    /// Maximum holesize is 0.003696 at time 0.020476
    /// ```
    #[test]
    fn matches_a_live_legacy_run_against_a_real_corpus_file() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/manual/sounds/marimba.wav"
        );
        let sf = cdp_sf::SoundFile::open(path).expect("open marimba.wav");
        let samples = sf.samples_f32().expect("decode marimba.wav");
        let chans = sf.fmt.channels as usize;
        let inverse_sr = 1.0 / f64::from(sf.fmt.sample_rate);

        let (size, samp) = largest_hole(&samples, 0.5);
        let length = (size / chans) as f64 * inverse_sr;
        let time = (samp / chans) as f64 * inverse_sr;

        assert_eq!(format!("{length:.6}"), "0.003696");
        assert_eq!(format!("{time:.6}"), "0.020476");

        // The same file at the default threshold finds nothing, which is
        // also what legacy reports: "Maximum holesize is 0.000000 at time
        // 0.000000".
        assert_eq!(largest_hole(&samples, DEFAULT_THRESHOLD), (0, 0));
    }
}
