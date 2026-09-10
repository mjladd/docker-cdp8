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

//! `sndinfo maxsamp infile [-f]`: reports the largest-magnitude
//! sample (or, for a non-sound file, the largest and smallest raw
//! float values) in a file. legacy: `legacy/dev/sndinfo/compare.c`'s
//! `case(INFO_MAXSAMP)` block, `try_header`, `get_and_set_float_maxmin`
//! and `get_and_set_float_maxmina`.
//!
//! Scope of this slice: [`cdp_sf::FileKind::Wave`],
//! [`cdp_sf::FileKind::Analysis`] and [`cdp_sf::FileKind::Envelope`]
//! only, the same subset every other `sndinfo` sub-command ported so
//! far covers (see `props`'s module doc for why). `Pitch`/
//! `Transposition`/`Formant` report a plain `ProgramError`.
//! `is_a_text_input_filetype`'s check (a plain `GOAL_FAILED` "This is
//! a text file." for a genuine text infile) is also out of scope:
//! `cdp-sf` does not classify a file that way at all yet, and this
//! crate's [`cdp_sf::FileKind`] has no variant a text file could
//! reach in the first place.
//!
//! Unlike every other `sndinfo` sub-command ported so far, legacy
//! tries a header-only shortcut first (`try_header`): several of its
//! own programs write a `maxamp`/`maxpfamp`/`maxloc`/`maxrep` (or, for
//! a non-sound file, `maxpfamp`/`maxnfamp`/`maxprep`/`maxnrep`)
//! property block into a file they process, recording that file's own
//! peak already found during that earlier pass. When every property
//! the shortcut needs is present, `sndinfo maxsamp` trusts them
//! verbatim rather than re-scanning the file's sample data, which can
//! report a stale answer if the file was edited since -- confirmed
//! live: `docs/manual/sounds/marimba.wav` carries an old-format
//! `maxamp`/`maxloc`/`maxrep` block reporting its peak at sample
//! 44310, but forcing a real scan (`-f`) finds the same peak
//! magnitude recurring earlier, at sample 136, with a different
//! repeat count -- both runs agree on the magnitude itself (legacy's
//! own `maxamp`-writing code and this file's actual samples do agree
//! on what the true peak *value* is), but not on where it first
//! occurs or how many times, since the stored `maxloc`/`maxrep` are
//! whatever an earlier CDP run happened to record. This is ported
//! as-observed, not treated as a bug: `sndinfo maxsamp`'s whole
//! purpose for a file carrying this header is to answer instantly
//! from it, trusting it exactly as far as legacy itself does.

use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SoundFile};

/// legacy: `legacy/dev/sndinfo/main.c`'s `make_initial_cmdline_check`
/// -- see `props::GREETING`'s doc, which this mirrors exactly
/// (confirmed live the same way: `sndinfo maxsamp infile`, argc 3,
/// prints it; `sndinfo maxsamp infile -f`, argc 4, does not).
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo maxsamp` (captured live via `spec/usage/sndinfo/
/// maxsamp.txt`, WP-0.2; byte-for-byte re-confirmed live this slice).
/// One fewer trailing blank line than the raw capture, same reason as
/// every other `USAGE` constant in this crate --
/// `ExitCategory::UsageOnly` already appends its own tail blank line.
pub const USAGE: &str = "CDP Release 7.1 2016
FIND MAXIMUM SAMPLE IN SOUNDFILE OR BINARY DATA FILE

USAGE: sndreport maxsamp infile [-f]
-f   Force file to be scanned
     (Ignore any header info about max sample)
";

/// legacy: `globcon.h`'s `MAXSAMP` -- deliberately `32767.0`, not
/// `32768.0`: confirmed live, the header-shortcut path's reported
/// value for `marimba.wav` (raw stored `maxamp` property `30408`)
/// only matches `sndinfo maxsamp`'s real `0.928007` when divided by
/// `32767.0` (dividing by `32768.0` gives `0.927979` instead).
const MAXSAMP: f64 = 32767.0;

/// legacy: `columns.h`'s `F_MAXSAMP` (the gain formula's numerator in
/// [`scan_wave`]) and `F_MINSAMP` (the scan's own starting sentinel).
const F_MAXSAMP: f64 = 1.0;
const F_MINSAMP: f32 = -1.0;

/// legacy: `case(INFO_MAXSAMP)` in `compare.c`. Returns the complete
/// output text, not including [`GREETING`] (the caller prints that
/// separately -- see this module's doc).
pub fn format_maxsamp(sf: &SoundFile, force_scan: bool) -> Result<String, CdpError> {
    match sf.file_kind {
        FileKind::Wave => {
            let chans = sf.fmt.channels as i64;
            let inverse_sr = if sf.fmt.sample_rate > 0 {
                1.0 / sf.fmt.sample_rate as f64
            } else {
                1.0
            };
            if !force_scan && let Some(text) = try_header_wave(sf, chans, inverse_sr) {
                return Ok(text);
            }
            let samples = sf.samples_f32().map_err(CdpError::from)?;
            Ok(scan_wave(&samples, chans, inverse_sr))
        }
        FileKind::Analysis(_) | FileKind::Envelope { .. } => {
            if !force_scan && let Some(text) = try_header_float(sf) {
                return Ok(text);
            }
            let samples = sf.samples_f32().map_err(CdpError::from)?;
            Ok(scan_float(&samples))
        }
        FileKind::Pitch(_) | FileKind::Transposition(_) | FileKind::Formant { .. } => {
            Err(CdpError::new(
                ExitCategory::ProgramError,
                "sndinfo maxsamp: pitch/transposition/formant files are not implemented yet",
            ))
        }
    }
}

/// legacy: `try_header`'s `SNDFILE` branch. Returns `None` (falls
/// through to [`scan_wave`]) whenever any property the shortcut needs
/// is missing, exactly mirroring `try_header`'s own `return(CONTINUE)`
/// points.
fn try_header_wave(sf: &SoundFile, chans: i64, inverse_sr: f64) -> Option<String> {
    let maxamp_raw = sf.properties.get_i32("maxamp").ok(); // legacy: old files
    let maxpfamp = sf.properties.get_f32("maxpfamp").ok(); // legacy: new files
    if maxamp_raw.is_none() && maxpfamp.is_none() {
        return None;
    }
    let maxloc = sf.properties.get_i32("maxloc").ok()?;
    let maxrep = sf.properties.get_i32("maxrep").ok()?;
    // legacy: `if(maxamp>=0) newmaxamp = (float)(maxamp/(double)MAXSAMP);`
    // -- `maxamp` (the raw int) starts at `-1` and is only ever
    // written by a successful "maxamp" property read, so a
    // new-format file (only "maxpfamp" present) leaves it at `-1`
    // and keeps `newmaxamp` as read directly from "maxpfamp" instead.
    let newmaxamp = match maxamp_raw {
        Some(maxamp) if maxamp >= 0 => (maxamp as f64 / MAXSAMP) as f32,
        _ => maxpfamp.unwrap_or(0.0),
    };
    let mut out = String::new();
    out.push_str(&format!("maximum abs value:.. {newmaxamp:.6}\n"));
    out.push_str(&format!("at: ................ {maxloc} samples\n"));
    let secs = (maxloc as i64 / chans) as f64 * inverse_sr;
    out.push_str(&format!("time: .............. {secs:.4} secs\n"));
    out.push_str(&format!("repeated: .......... {maxrep} times\n"));
    // legacy: gated on `newmaxamp>=0.0` (not `>0.0`, unlike
    // `scan_wave`'s equivalent gate below -- ported as observed, not
    // unified), and the gain itself divides by the raw `maxamp` int,
    // not `newmaxamp` -- for a new-format file with no "maxamp"
    // property this is `1.0/(double)(-1)`, a real legacy quirk not
    // confirmed live (no corpus file has "maxpfamp" without "maxamp").
    if newmaxamp >= 0.0 {
        let maxamp_for_gain = maxamp_raw.unwrap_or(-1) as f64;
        let gain = 1.0 / maxamp_for_gain;
        let dbgain = gain.log10() * 20.0;
        out.push_str(&format!("max possible gain:.. {gain:.4}\n"));
        out.push_str(&format!("max dB gain:........ {dbgain:.4}\n"));
    }
    Some(out)
}

/// legacy: `try_header`'s non-`SNDFILE` branch. All four properties
/// are required; `None` falls through to [`scan_float`]. Not
/// confirmed live: no corpus file (`docs/manual`) carries this
/// property set on an analysis or envelope file, only [`scan_float`]
/// is live-confirmed for those file kinds.
fn try_header_float(sf: &SoundFile) -> Option<String> {
    let maxpfamp = sf.properties.get_f32("maxpfamp").ok()?;
    let maxnfamp = sf.properties.get_f32("maxnfamp").ok()?;
    let pos_repeats = sf.properties.get_i32("maxprep").ok()?;
    let neg_repeats = sf.properties.get_i32("maxnrep").ok()?;
    let mut out = String::new();
    // legacy: `"occurs: ............ %d times\n"` -- twelve dots, one
    // space, distinct from `scan_float`'s own "occurs" line (see its
    // doc) -- a genuine, confirmed-from-source difference between
    // these two functions, not a typo in this port.
    out.push_str(&format!("maximum float value: {maxpfamp:.6}\n"));
    out.push_str(&format!("occurs: ............ {pos_repeats} times\n"));
    out.push_str(&format!("minimum float value: {maxnfamp:.6}\n"));
    out.push_str(&format!("occurs: ............ {neg_repeats} times\n"));
    Some(out)
}

/// legacy: `get_and_set_float_maxmin`, the `SNDFILE` full scan.
/// `samples` is already decoded to the same -1.0..=1.0 range legacy's
/// `dz->sampbuf` uses (see [`SoundFile::samples_f32`]).
fn scan_wave(samples: &[f32], chans: i64, inverse_sr: f64) -> String {
    let mut maxamp = F_MINSAMP;
    let mut maxloc: i64 = 0;
    let mut maxrep: i64 = 1;
    for (n, &sample) in samples.iter().enumerate() {
        let mag = sample.abs();
        if mag > maxamp {
            maxloc = n as i64;
            maxamp = mag;
            maxrep = 1;
        } else if mag == maxamp {
            maxrep += 1;
        }
    }
    let mut out = String::new();
    out.push_str(&format!("maximum abs value:.. {maxamp:.6}\n"));
    out.push_str(&format!("at: ................ {maxloc} samples\n"));
    let secs = (maxloc / chans) as f64 * inverse_sr;
    out.push_str(&format!("time: .............. {secs:.4} secs\n"));
    out.push_str(&format!("repeated: .......... {maxrep} times\n"));
    // legacy: gated on `maxamp>0.0f`, unlike `try_header_wave`'s
    // `>=0.0` -- see that function's doc.
    if maxamp > 0.0 {
        let gain = F_MAXSAMP / maxamp as f64;
        let dbgain = gain.log10() * 20.0;
        out.push_str(&format!("max possible gain:.. {gain:.4}\n"));
        out.push_str(&format!("max dB gain:........ {dbgain:.4}\n"));
    }
    out
}

/// legacy: `get_and_set_float_maxmina`, the non-`SNDFILE` full scan.
/// `samples` are the file's raw floats, undecoded (no -1.0..=1.0
/// normalisation -- confirmed live matching bin-for-bin against
/// `capm.ana`/`crklenv.evl`, both [`cdp_sf::props::SampleType::Float32`]
/// already, so [`SoundFile::samples_f32`] returns them unscaled).
///
/// legacy quirk, ported as-is: the running maximum starts at `DBL_MIN`
/// (the smallest *positive* representable `double`, effectively `~0`,
/// not `-DBL_MAX`), so a file whose every value is negative would
/// never update it at all and would report `DBL_MIN` itself as the
/// "maximum". Not reachable by either real corpus file this slice
/// confirms against (both have genuine positive values well above
/// zero), so this is not independently confirmed live, only
/// transcribed faithfully from `get_and_set_float_maxmina`'s own `double
/// maxpdamp = DBL_MIN;`.
fn scan_float(samples: &[f32]) -> String {
    let mut maxpdamp = f64::MIN_POSITIVE; // legacy: DBL_MIN
    let mut maxndamp = f64::MAX; // legacy: DBL_MAX
    let mut maxprep: i64 = 1;
    let mut maxnrep: i64 = 1;
    for &sample in samples {
        let v = sample as f64;
        if v < maxpdamp {
        } else if v > maxpdamp {
            maxpdamp = v;
            maxprep = 1;
        } else {
            maxprep += 1;
        }
        if v > maxndamp {
        } else if v < maxndamp {
            maxndamp = v;
            maxnrep = 1;
        } else {
            maxnrep += 1;
        }
    }
    let maxpfamp = maxpdamp as f32;
    let maxnfamp = maxndamp as f32;
    let mut out = String::new();
    // legacy: `"occurs: ...........  %d times\n"` -- eleven dots, two
    // spaces -- see [`try_header_float`]'s doc for the contrast.
    out.push_str(&format!("maximum float value: {maxpfamp:.6}\n"));
    out.push_str(&format!("occurs: ...........  {maxprep} times\n"));
    out.push_str(&format!("minimum float value: {maxnfamp:.6}\n"));
    out.push_str(&format!("occurs: ...........  {maxnrep} times\n"));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    #[test]
    fn marimba_wav_header_shortcut_matches_the_real_legacy_output() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out = format_maxsamp(&sf, false).unwrap();
        assert_eq!(
            out,
            "maximum abs value:.. 0.928007\n\
             at: ................ 44310 samples\n\
             time: .............. 1.0048 secs\n\
             repeated: .......... 1 times\n\
             max possible gain:.. 0.0000\n\
             max dB gain:........ -89.6598\n"
        );
    }

    #[test]
    fn marimba_wav_forced_scan_matches_the_real_legacy_output_and_disagrees_with_its_own_stale_header()
     {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out = format_maxsamp(&sf, true).unwrap();
        assert_eq!(
            out,
            "maximum abs value:.. 0.928007\n\
             at: ................ 136 samples\n\
             time: .............. 0.0031 secs\n\
             repeated: .......... 1 times\n\
             max possible gain:.. 1.0776\n\
             max dB gain:........ 0.6490\n"
        );
    }

    #[test]
    fn tsw1_2nd_aiff_full_scan_matches_the_real_legacy_output() {
        // legacy: no header shortcut properties at all on this file,
        // so `force_scan: false` already reaches `scan_wave`.
        let sf = SoundFile::open(repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff")).unwrap();
        let out = format_maxsamp(&sf, false).unwrap();
        assert_eq!(
            out,
            "maximum abs value:.. 0.299722\n\
             at: ................ 29914 samples\n\
             time: .............. 0.6783 secs\n\
             repeated: .......... 4 times\n\
             max possible gain:.. 3.3364\n\
             max dB gain:........ 10.4656\n"
        );
    }

    #[test]
    fn clip5_all_wav_stereo_full_scan_matches_the_real_legacy_output() {
        // legacy: exercises the multi-channel `maxloc/chans` division
        // this file's mono corpus siblings can't (chans == 2).
        let sf = SoundFile::open(repo_path("docs/manual/sounds/clip5-all.wav")).unwrap();
        let out = format_maxsamp(&sf, false).unwrap();
        assert_eq!(
            out,
            "maximum abs value:.. 0.393017\n\
             at: ................ 16772571 samples\n\
             time: .............. 190.1652 secs\n\
             repeated: .......... 1 times\n\
             max possible gain:.. 2.5444\n\
             max dB gain:........ 8.1118\n"
        );
    }

    #[test]
    fn capm_ana_matches_the_real_legacy_output() {
        let sf = SoundFile::open(repo_path("docs/manual/data/capm.ana")).unwrap();
        let out = format_maxsamp(&sf, false).unwrap();
        assert_eq!(
            out,
            "maximum float value: 22222.265625\n\
             occurs: ...........  466 times\n\
             minimum float value: -172.265610\n\
             occurs: ...........  150 times\n"
        );
    }

    #[test]
    fn crklenv_evl_matches_the_real_legacy_output() {
        let sf = SoundFile::open(repo_path("docs/manual/data/crklenv.evl")).unwrap();
        let out = format_maxsamp(&sf, false).unwrap();
        assert_eq!(
            out,
            "maximum float value: 2.480246\n\
             occurs: ...........  1 times\n\
             minimum float value: 0.000000\n\
             occurs: ...........  1 times\n"
        );
    }
}
