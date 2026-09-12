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

//! `housekeep respec mode infile outfile ...` (`HOUSE_SPEC`). Modes 2
//! (`HOUSE_CONVERT`, `housekeep respec 2 infile outfile`: toggles
//! sample type between 16-bit integer and 32-bit float) and 3
//! (`HOUSE_REPROP`, `housekeep respec 3 infile outfile [-ssrate]
//! [-cchannels]`: changes the declared sample rate and/or channel
//! count, without resampling or rechanneling the data) are ported so
//! far. Mode 1 (`HOUSE_RESAMPLE`, real cubic-spline-interpolated
//! resampling, `legacy/dev/houskeep/respec.c`'s `cubic_spline`) is a
//! substantial DSP task of its own, not attempted in this slice --
//! reports a plain `ProgramError`.
//!
//! legacy: `legacy/dev/houskeep/respec.c`'s `do_convert_process`
//! (mode 2) and `reprop_process` (mode 3).
//!
//! Both modes are `SNDFILES_ONLY` (`ap_house.c`'s
//! `assign_process_logic`); scope is [`cdp_sf::FileKind::Wave`] with
//! [`cdp_sf::SampleType::Short16`] or `Float32` sample data, the same
//! restriction every other `housekeep` sub-command in this crate
//! already established.
//!
//! Both modes share a real, confirmed-live overwrite-refusal quirk
//! genuinely different from every other `housekeep` command ported so
//! far: `reprop_process`/`do_convert_process` create their outfile
//! via `sndcreat_formatted` directly, not the generic
//! `create_sized_outfile` every other command in this crate goes
//! through, so an already-existing outfile reports `"ERROR: SYSTEM
//! ERROR"`/`"Unable to open outfile %s\n"` (`SystemError`), not
//! `copy`/`chans`'s own `"ERROR: INVALID DATA"`/`"Cannot open output
//! file %s\n"` (`DataError`) -- confirmed live for both modes.
//!
//! Mode 3's own real semantics were confirmed live, not merely
//! inferred from the (misleadingly worded) usage text or source
//! comments, since they turned out to be genuinely surprising:
//!
//! - `-s`/`-c` never resample or rechannel the actual sample stream
//!   at all -- every raw (already-decoded, interleaved) sample value
//!   is written out completely unchanged, in the same order, just
//!   under a new declared sample rate and/or channel count. Confirmed
//!   live, bit-exact, for both a sample-rate-only change (a mono
//!   file's own samples, unchanged, under a different `srate`) and a
//!   channel-count-only change (a stereo file's own raw interleaved
//!   samples, unchanged, now declared mono, doubling its own apparent
//!   duration).
//! - The output sample type is unconditionally 16-bit integer,
//!   regardless of the infile's own type or of anything on the
//!   command line: legacy's `-t` (sample-type-selecting) flag was
//!   removed from this build (`legacy/dev/cdp2k/parstruct.c`'s own
//!   `#else`/`/*RWD May 2005 removed -t option */` comment), but
//!   `tklib1.c`'s `default_val[HSPEC_TYPE]` still unconditionally
//!   defaults to `SAMP_SHORT` for any real sound file, and nothing
//!   can override it. Confirmed live: a genuine synthetic 32-bit
//!   float infile, respecified with `-s` alone, writes a 16-bit
//!   integer outfile whose samples are the infile's own real values,
//!   properly decoded and re-encoded (not a raw byte reinterpretation
//!   -- `0.5` decodes and rounds to `16384`, `1.0` to `32767`,
//!   matching every other 16-bit encode in this codebase).
//! - legacy's own "no genuine change requested" check
//!   (`reprop_process`'s own `if(...) sprintf(errstr,"NO CHANGE to
//!   input file.\n")`) is built on a comparison that is *not* what it
//!   looks like: it compares the *default* value of an internal
//!   "requested type" variable (always `SAMP_SHORT`, per the point
//!   above) against a *second* variable computed as the *opposite* of
//!   the infile's own actual on-disk type. This means the "no genuine
//!   change" check's own type comparison is satisfied (contributing
//!   to a "no change" failure) precisely when the infile is *already*
//!   32-bit float, and never satisfied when the infile is already
//!   16-bit -- confirmed live at all four sampled corners: a 16-bit
//!   infile with no `-s`/`-c` at all succeeds (bit-exact passthrough,
//!   since the forced 16-bit target already matches); a 32-bit float
//!   infile with no `-s`/`-c` (or with `-s`/`-c` explicitly repeating
//!   its own already-current values) reports `"NO CHANGE to input
//!   file.\n"` (`GoalFailed`); and a 32-bit float infile given a
//!   genuinely different `-s` or `-c` value succeeds. `is_no_change`
//!   below reproduces this exact, confirmed-surprising formula
//!   directly, not a more "sensible" one.

use super::write_wave_file;
use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SampleType, SoundFile};

/// legacy: `legacy/dev/houskeep/main.c`'s `make_initial_cmdline_check`
/// -- see `cdp_programs::housekeep::copy::GREETING`'s doc, which this
/// mirrors exactly.
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/houskeep/main.c` prints for a
/// bare `housekeep respec` (captured live via `spec/usage/housekeep/
/// respec.txt`, WP-0.2; re-confirmed live this slice). One fewer
/// trailing blank line than the raw capture -- see
/// `housekeep::USAGE`'s doc.
pub const USAGE: &str = "CDP Release 7.1 2016
ALTER THE SPECIFICATION OF A SOUNDFILE.

USAGE: housekeep respec 1 infile outfile new_samplerate
OR:    housekeep respec 2 infile outfile
OR:    housekeep respec 3 infile outfile [-ssrate] [-cchannels]
MODES ARE

1) RESAMPLE  at some different sampling rate.
           New sampling rate must be one of...
           96000,88200,48000,24000,44100,22050,32000,16000

2) CONVERT FROM INTEGER TO FLOAT SAMPLES, OR VICE VERSA

3) CHANGE PROPERTIES OF SOUND: (USE WITH CAUTION!!)
      SRATE is the new sample rate to impose.
           NB: this does NOT RESAMPLE the data.
           Simply causes original data to be read at different srate.
           Sound has same no of samples, but different duration,
           and will appear to be transposed in pitch.
      CHANNELS is the new channel count to impose.
           NB: this does NOT RECHANNEL the data.
           Hence e.g. a stereo file will appear twice as long.
";

pub const MAX_MODE: u32 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// legacy: `HOUSE_CONVERT`.
    Convert,
    /// legacy: `HOUSE_REPROP`.
    Reprop,
}

impl Mode {
    /// `None` for mode 1 (`HOUSE_RESAMPLE`, real cubic-spline
    /// resampling, not implemented yet) -- the caller maps that to a
    /// plain `ProgramError`, the same shape
    /// `cdp_programs::sndinfo::units::Mode::from_number` already
    /// established for a mode `1..=MAX_MODE` in range but not yet
    /// ported.
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            2 => Some(Mode::Convert),
            3 => Some(Mode::Reprop),
            _ => None,
        }
    }
}

/// legacy: `sndcreat_formatted`'s own failure text, confirmed live for
/// both `respec` modes -- see this module's doc for why it differs
/// from `super::check_outfile_does_not_exist`'s text.
fn check_outfile_does_not_exist_respec(path: &str) -> Result<(), CdpError> {
    if std::path::Path::new(path).exists() {
        return Err(CdpError::new(
            ExitCategory::SystemError,
            format!("Unable to open outfile {path}\n"),
        ));
    }
    Ok(())
}

fn check_scope(sf: &SoundFile) -> Result<(), CdpError> {
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep respec: only sound files are implemented yet",
        ));
    }
    if !matches!(
        sf.fmt.sample_type,
        SampleType::Short16 | SampleType::Float32
    ) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep respec: this sample format is not implemented yet",
        ));
    }
    Ok(())
}

/// legacy: `do_convert_process` (`legacy/dev/houskeep/respec.c`).
/// Toggles sample type, keeping srate/channels and every sample value
/// unchanged (decoded and re-encoded through the normalised float
/// pipeline every command in this crate already shares, not a raw
/// byte reinterpretation).
pub fn convert(sf: &SoundFile, outfile_path: &str) -> Result<(), CdpError> {
    check_scope(sf)?;
    check_outfile_does_not_exist_respec(outfile_path)?;
    let samples = sf.samples_f32().map_err(CdpError::from)?;
    let out_type = match sf.fmt.sample_type {
        SampleType::Short16 => SampleType::Float32,
        SampleType::Float32 => SampleType::Short16,
        _ => unreachable!("check_scope already restricted sample_type"),
    };
    write_wave_file(
        sf.fmt.channels,
        sf.fmt.sample_rate,
        out_type,
        &samples,
        outfile_path,
    )?;
    // legacy: `display_virtual_time`'s `SNDFILE_OUT` branch -- srate
    // and channels are unchanged by this mode, so this is simply the
    // infile's own real duration, confirmed live.
    super::print_virtual_time(
        samples.len() as f64 / (sf.fmt.sample_rate as f64 * sf.fmt.channels as f64),
    );
    Ok(())
}

/// legacy: `reprop_process` (`legacy/dev/houskeep/respec.c`). See
/// this module's own doc for the confirmed, surprising semantics:
/// `-s`/`-c` only change the outfile's declared header fields, never
/// resampling or rechanneling the actual sample stream; the output
/// sample type is unconditionally [`SampleType::Short16`].
pub fn reprop(
    sf: &SoundFile,
    outfile_path: &str,
    srate: Option<i64>,
    channels: Option<i64>,
) -> Result<(), CdpError> {
    check_scope(sf)?;
    let target_srate = srate.unwrap_or(sf.fmt.sample_rate as i64);
    let target_channels = channels.unwrap_or(sf.fmt.channels as i64);
    if is_no_change(sf, target_srate, target_channels) {
        return Err(CdpError::new(
            ExitCategory::GoalFailed,
            "NO CHANGE to input file.\n",
        ));
    }
    if !matches!(target_channels, 1 | 2 | 4 | 6 | 8 | 16) {
        return Err(CdpError::new(
            ExitCategory::DataError,
            format!("\nInvalid channel count [{target_channels}]"),
        ));
    }
    if !matches!(
        target_srate,
        96000 | 88200 | 48000 | 44100 | 32000 | 24000 | 22050 | 16000
    ) {
        return Err(CdpError::new(
            ExitCategory::DataError,
            format!("\nInvalid sample rate [{target_srate}]"),
        ));
    }
    check_outfile_does_not_exist_respec(outfile_path)?;
    let samples = sf.samples_f32().map_err(CdpError::from)?;
    write_wave_file(
        target_channels as u16,
        target_srate as u32,
        SampleType::Short16,
        &samples,
        outfile_path,
    )?;
    // legacy: `display_virtual_time`'s `SNDFILE_OUT` branch, using the
    // *new* (target) srate/channels as the divisor -- confirmed live,
    // this reports a skewed, not-the-real-duration value whenever
    // `-s`/`-c` actually changes anything (see this module's doc).
    super::print_virtual_time(
        samples.len() as f64 / (target_srate as f64 * target_channels as f64),
    );
    Ok(())
}

/// legacy: `reprop_process`'s own "no genuine change requested" check
/// -- see this module's doc for the confirmed, surprising formula
/// this reproduces exactly (`type_no_change` is true precisely when
/// the infile is *already* [`SampleType::Float32`], not when the
/// requested type equals the infile's own type).
fn is_no_change(sf: &SoundFile, target_srate: i64, target_channels: i64) -> bool {
    let type_no_change = matches!(sf.fmt.sample_type, SampleType::Float32);
    target_srate == sf.fmt.sample_rate as i64
        && target_channels == sf.fmt.channels as i64
        && type_no_change
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "cdp-housekeep-respec-test-{}-{name}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&p);
        p
    }

    fn write_synthetic_float32_wav(path: &std::path::Path, samples: &[f32]) {
        let mut data = Vec::with_capacity(samples.len() * 4);
        for &s in samples {
            data.extend_from_slice(&s.to_le_bytes());
        }
        let block_align: u16 = 4;
        let sample_rate: u32 = 44100;
        let byte_rate = sample_rate * block_align as u32;
        let mut fmt = Vec::new();
        fmt.extend_from_slice(&3u16.to_le_bytes()); // IEEE float
        fmt.extend_from_slice(&1u16.to_le_bytes()); // mono
        fmt.extend_from_slice(&sample_rate.to_le_bytes());
        fmt.extend_from_slice(&byte_rate.to_le_bytes());
        fmt.extend_from_slice(&block_align.to_le_bytes());
        fmt.extend_from_slice(&32u16.to_le_bytes());
        let mut body = Vec::new();
        body.extend_from_slice(b"WAVE");
        body.extend_from_slice(b"fmt ");
        body.extend_from_slice(&(fmt.len() as u32).to_le_bytes());
        body.extend_from_slice(&fmt);
        body.extend_from_slice(b"data");
        body.extend_from_slice(&(data.len() as u32).to_le_bytes());
        body.extend_from_slice(&data);
        let mut out = Vec::new();
        out.extend_from_slice(b"RIFF");
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        std::fs::write(path, out).unwrap();
    }

    #[test]
    fn convert_short_to_float_round_trips_sample_values() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("convert").join("marimba_float.wav");
        let _ = std::fs::remove_file(&out_path);
        convert(&sf, out_path.to_str().unwrap()).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.sample_type, SampleType::Float32);
        assert_eq!(out.fmt.channels, sf.fmt.channels);
        assert_eq!(out.fmt.sample_rate, sf.fmt.sample_rate);
        assert_eq!(out.samples_f32().unwrap(), sf.samples_f32().unwrap());
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn reprop_short_infile_with_no_flags_is_a_bit_exact_passthrough() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("reprop_noop").join("marimba_reprop.wav");
        let _ = std::fs::remove_file(&out_path);
        reprop(&sf, out_path.to_str().unwrap(), None, None).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.sample_type, SampleType::Short16);
        assert_eq!(out.fmt.channels, sf.fmt.channels);
        assert_eq!(out.fmt.sample_rate, sf.fmt.sample_rate);
        assert_eq!(out.samples_f32().unwrap(), sf.samples_f32().unwrap());
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn reprop_short_infile_with_new_srate_keeps_every_raw_sample() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("reprop_srate").join("marimba_22050.wav");
        let _ = std::fs::remove_file(&out_path);
        reprop(&sf, out_path.to_str().unwrap(), Some(22050), None).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.sample_rate, 22050);
        assert_eq!(out.samples_f32().unwrap(), sf.samples_f32().unwrap());
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn reprop_float_infile_with_no_flags_reports_no_change() {
        let path = scratch_dir("reprop_float_noop").join("float.wav");
        write_synthetic_float32_wav(&path, &[0.5, -0.5, 0.25]);
        let sf = SoundFile::open(&path).unwrap();
        let err = reprop(&sf, "irrelevant.wav", None, None).unwrap_err();
        assert_eq!(err.category, ExitCategory::GoalFailed);
        assert_eq!(err.to_string(), "NO CHANGE to input file.\n");
    }

    #[test]
    fn reprop_float_infile_with_a_new_srate_converts_to_short16() {
        let path = scratch_dir("reprop_float_convert").join("float.wav");
        write_synthetic_float32_wav(&path, &[0.5, -0.5, 0.25, -0.25, 0.1, 0.0, 1.0, -1.0]);
        let sf = SoundFile::open(&path).unwrap();
        let out_path = scratch_dir("reprop_float_convert").join("out.wav");
        let _ = std::fs::remove_file(&out_path);
        reprop(&sf, out_path.to_str().unwrap(), Some(22050), None).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.sample_type, SampleType::Short16);
        assert_eq!(out.fmt.sample_rate, 22050);
        // legacy: confirmed live -- see this module's own doc. Values
        // taken directly from a real `cdp8-postmerge` run of the same
        // synthetic infile.
        let expected: Vec<i16> = vec![16384, -16384, 8192, -8192, 3277, 0, 32767, -32767];
        let actual: Vec<i16> = out
            .samples_f32()
            .unwrap()
            .iter()
            .map(|&s| (s as f64 * 32767.0).round() as i16)
            .collect();
        assert_eq!(actual, expected);
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn reprop_invalid_channel_count_reports_the_confirmed_text() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let err = reprop(&sf, "irrelevant.wav", None, Some(3)).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(err.to_string(), "\nInvalid channel count [3]");
    }

    #[test]
    fn reprop_invalid_srate_reports_the_confirmed_text() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let err = reprop(&sf, "irrelevant.wav", Some(44099), None).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(err.to_string(), "\nInvalid sample rate [44099]");
    }

    #[test]
    fn respec_existing_outfile_uses_the_system_error_text() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("respec_exists").join("exists.wav");
        std::fs::write(&out_path, b"pre-existing").unwrap();
        let err = convert(&sf, out_path.to_str().unwrap()).unwrap_err();
        assert_eq!(err.category, ExitCategory::SystemError);
        assert_eq!(
            err.to_string(),
            format!("Unable to open outfile {}\n", out_path.to_str().unwrap())
        );
        let _ = std::fs::remove_file(&out_path);
    }
}
