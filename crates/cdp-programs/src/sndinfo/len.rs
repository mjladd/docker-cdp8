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

//! `sndinfo len infile`: prints a sound or analysis file's duration.
//! legacy: `legacy/dev/sndinfo/compare.c`'s `case(INFO_SFLEN)` block.
//!
//! Scope of this slice: [`cdp_sf::FileKind::Wave`],
//! [`cdp_sf::FileKind::Analysis`] and [`cdp_sf::FileKind::Envelope`]
//! only, the same subset `sndinfo::props` (WP-2.1's first slice)
//! already covers and for the same reason: these are the three kinds
//! this repository's corpus (`docs/manual`) actually has an example
//! of. `Pitch`/`Transposition`/`Formant` report a plain `ProgramError`.

use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SoundFile};

/// legacy: `legacy/dev/sndinfo/main.c`'s `make_initial_cmdline_check`,
/// unconditional whenever raw `argc<4` -- see `props::GREETING`'s doc,
/// which this mirrors exactly (confirmed live the same way).
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo len` (captured live via `spec/usage/sndinfo/
/// len.txt`, WP-0.2). See `props::USAGE`'s doc for why this bundles
/// [`GREETING`] at the front.
pub const USAGE: &str = "CDP Release 7.1 2016
DISPLAY DURATION OF A SNDFILING-SYSTEM FILE

USAGE: sndreport len infile
";

/// legacy: `make_time_display` (`legacy/dev/sndinfo/compare.c`).
/// Renders a duration as `"<hrs> hrs <mins> mins <secs>.dddddd secs
/// "` -- `hrs`/`mins` are omitted entirely when zero, and the trailing
/// space is always present (the caller's next token follows directly
/// after it, with no further space of its own).
///
/// legacy quirk, ported as-is rather than fixed: the C code checks
/// `if(mins > 60)`, not `>= 60`, before rolling minutes over into
/// hours. A duration of exactly 60 minutes therefore prints `"60 mins
/// ..."`, not `"1 hrs 0 mins ..."`. This is fully deterministic (not
/// the uninitialized-memory kind of bug `LEGACY-BUGS.md` exists for),
/// so it is preserved for byte-for-byte output compatibility rather
/// than logged there. Not confirmed live: no file in this repository's
/// corpus is long enough to reach the `mins`/`hrs` branches at all
/// (the longest, `clip5-all.wav`, is a little over 3 minutes, enough
/// to confirm the `mins` branch but not `hrs`).
pub(crate) fn format_duration(mut secs: f64) -> String {
    let mut mins: i64 = 0;
    let mut hrs: i64 = 0;
    if secs > 60.0 {
        mins = (secs / 60.0).floor() as i64;
        secs -= (mins * 60) as f64;
    }
    if mins > 60 {
        hrs = (mins as f64 / 60.0).floor() as i64;
        mins -= hrs * 60;
    }
    let mut out = String::new();
    if hrs != 0 {
        out.push_str(&format!("{hrs} hrs "));
    }
    if mins != 0 {
        out.push_str(&format!("{mins} mins "));
    }
    out.push_str(&format!("{secs:.6} secs "));
    out
}

/// legacy: `secs = (double)(dz->insams[0]/dz->infile->channels) *
/// inverse_sr` (`compare.c`'s `case(INFO_SFLEN)`, `SNDFILE` branch) --
/// integer division truncates *before* the cast to `double`. Shared
/// with `crate::sndinfo::timesmp`, whose `time` parameter's dynamic
/// range upper bound is this same value (`tklib1.c`'s
/// `ap->hi[INFO_TIME] = duration`).
pub fn wave_duration_secs(sf: &SoundFile) -> f64 {
    let channels = sf.fmt.channels as u64;
    (sf.sample_count() / channels) as f64 * (1.0 / sf.fmt.sample_rate as f64)
}

/// legacy: `case(INFO_SFLEN)` in `compare.c`. Returns the complete
/// duration text, not including [`GREETING`] (the caller prints that
/// separately -- see this module's doc).
pub fn format_len(sf: &SoundFile) -> Result<String, CdpError> {
    let samples = sf.sample_count();
    let mut out = String::new();

    match sf.file_kind {
        FileKind::Wave => {
            out.push_str("A soundfile\n");
            let secs = wave_duration_secs(sf);
            out.push_str("DURATION: ");
            out.push_str(&format_duration(secs));
            out.push_str(&format!("samples {samples}\n"));
        }
        FileKind::Analysis(spectral) => {
            out.push_str("An analysis data file\n");
            // legacy: `dz->wlength` ("length in windows",
            // `structures.h`) is `dz->insams[0]/dz->wanted`, and
            // `dz->wanted` ("floats per window") is this file's
            // reported channel count for an analysis file -- confirmed
            // by cross-checking against WP-1.4's independently
            // established frame count for `capm.wav`/`capm.ana` (same
            // `M`/`D`): `2901528/1026 == 2828` exactly.
            let channels = sf.fmt.channels as u64;
            let wlength = samples / channels;
            // legacy: `dz->frametime`, "duration of window"
            // (`structures.h`), computed here as
            // `decimation_factor/original_sample_rate` in `f32` (the
            // struct's own field type) before widening to `f64` for
            // the `secs` multiply, matching the C code's implicit
            // float-to-double promotion.
            let frametime =
                spectral.decimation_factor as f32 / spectral.original_sample_rate as f32;
            let secs = wlength as f64 * frametime as f64;
            out.push_str("DURATION: ");
            out.push_str(&format_duration(secs));
            out.push_str(&format!("windows {wlength} : floats {samples}\n"));
        }
        FileKind::Envelope { window_size } => {
            out.push_str("A binary envelope file\n");
            let secs = samples as f64 * window_size as f64 * 0.001;
            out.push_str("DURATION: ");
            out.push_str(&format_duration(secs));
            // legacy: `dz->wlength` is never set when opening an
            // envelope file (no per-window count exists for this file
            // kind), so the real `sndinfo len` always prints a literal
            // `0` here -- confirmed live against `crklenv.evl`.
            out.push_str(&format!("windows 0 : floats {samples}\n"));
        }
        FileKind::Pitch(_) | FileKind::Transposition(_) | FileKind::Formant { .. } => {
            return Err(CdpError::new(
                ExitCategory::ProgramError,
                "sndinfo len: pitch/transposition/formant files are not implemented yet",
            ));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    #[test]
    fn marimba_wav_matches_the_real_legacy_sndinfo_len_output() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out = format_len(&sf).unwrap();
        assert_eq!(out, "A soundfile\nDURATION: 1.001678 secs samples 44174\n");
    }

    #[test]
    fn tsw1_2nd_aiff_matches_the_real_legacy_sndinfo_len_output() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff")).unwrap();
        let out = format_len(&sf).unwrap();
        assert_eq!(out, "A soundfile\nDURATION: 5.467166 secs samples 241102\n");
    }

    #[test]
    fn clip5_all_wav_stereo_matches_the_real_legacy_sndinfo_len_output_and_exercises_the_mins_branch()
     {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/clip5-all.wav")).unwrap();
        let out = format_len(&sf).unwrap();
        assert_eq!(
            out,
            "A soundfile\nDURATION: 3 mins 31.006259 secs samples 18610752\n"
        );
    }

    #[test]
    fn capm_ana_matches_the_real_legacy_sndinfo_len_output() {
        let sf = SoundFile::open(repo_path("docs/manual/data/capm.ana")).unwrap();
        let out = format_len(&sf).unwrap();
        assert_eq!(
            out,
            "An analysis data file\nDURATION: 8.208254 secs windows 2828 : floats 2901528\n"
        );
    }

    #[test]
    fn crklenv_evl_matches_the_real_legacy_sndinfo_len_output() {
        let sf = SoundFile::open(repo_path("docs/manual/data/crklenv.evl")).unwrap();
        let out = format_len(&sf).unwrap();
        assert_eq!(
            out,
            "A binary envelope file\nDURATION: 8.257596 secs windows 0 : floats 2845\n"
        );
    }
}
