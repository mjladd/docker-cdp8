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

//! `housekeep chans mode infile ...` (`HOUSE_CHANS`). Modes 1
//! (`HOUSE_CHANNEL`, `housekeep chans 1 infile channo`: extracts one
//! channel to a new, auto-named mono file), 3 (`HOUSE_ZCHANNEL`,
//! `housekeep chans 3 infile outfile channo`: zeroes one channel), 4
//! (`STOM`, `housekeep chans 4 infile outfile [-p]`: mixes a stereo
//! infile down to mono) and 5 (`MTOS`, `housekeep chans 5 infile
//! outfile`: doubles a mono infile's samples into a stereo outfile)
//! are ported so far. Mode 2 (extract all channels) reports a plain
//! `ProgramError`.
//!
//! legacy: `legacy/dev/houskeep/channels.c`'s `do_channels`,
//! `case(HOUSE_CHANNEL)`, `case(HOUSE_ZCHANNEL)` and (for modes 4/5)
//! `case(STOM)`/`case(MTOS)` (mode 2, `HOUSE_CHANNELS`, has its own,
//! not-yet-ported loop bounds change: `start_chan=0`/
//! `end_chan=dz->infile->channels`, but needs [`extract_channel`]'s
//! per-channel behaviour repeated once per channel, not yet confirmed
//! live for a real multi-channel file). `STOM`/`HOUSE_ZCHANNEL` also
//! have their own, not-yet-ported "thumbnail"/generalised multichannel
//! branches -- not confirmed live, since no corpus file has more than
//! two channels; `MTOS`'s own multichannel case is a plain error,
//! already ported (see [`mono_to_stereo`]'s doc).
//!
//! `HOUSE_ZCHANNEL`'s own mono-infile branch has a real, confirmed
//! legacy bug (`docs/migration/LEGACY-BUGS.md`): its in-place backward
//! expansion loop corrupts the first sample of every internal read
//! buffer to `0` instead of the real (non-zeroed) channel's value.
//! [`zero_channel`] ports the intended behavior (no corruption), not
//! the bug, per that file's own rule.
//!
//! Takes no outfile on the command line at all -- unlike `copy`, the
//! output filename is derived from the infile's own path, confirmed
//! live: `housekeep chans 1 marimba.wav 1` writes `marimba_c1.wav`
//! (legacy: `dz->wordstor[0]`, the infile's own path string as given
//! on the command line -- the same string this crate already has as
//! `ParsedCommand::infiles[0]`, needing no new "special data"
//! machinery). No zero-padding, unlike `copy` mode 2's `X_001` scheme:
//! confirmed live, channel 1 of a ten-or-more-channel file (not
//! reachable by any real corpus file, inferred from
//! `insert_new_number_at_filename_end`'s plain, unpadded `%d`) would
//! still write `_c1`, not `_c01`.
//!
//! `channo`'s valid range (`1..=infile.channels`) is infile-dependent,
//! the same "range is a parameter, not a literal" shape
//! `cdp_programs::sndinfo::smptime`/`timesmp` already established.
//! Confirmed live: an out-of-range `channo` reports the generic
//! `"Parameter[1] Value (...) out of range (1.000000 to
//! N.000000)"`, not the more specific `"There is no channel %d in the
//! input file"` text `param_preprocess` also carries for this mode --
//! that second check is unreachable, masked by the generic range
//! check firing first on the exact same condition, confirmed live
//! with a channel number one past a mono file's own count.
//!
//! Scope: [`cdp_sf::FileKind::Wave`] with
//! [`cdp_sf::SampleType::Short16`] or `Float32` sample data, the same
//! restriction `copy` already established (see that module's doc for
//! why).

use super::{check_outfile_does_not_exist, write_wave_file};
use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SampleType, SoundFile};

/// legacy: `legacy/dev/houskeep/main.c`'s `make_initial_cmdline_check`
/// -- see `cdp_programs::housekeep::copy::GREETING`'s doc, which this
/// mirrors exactly.
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/houskeep/main.c` prints for a
/// bare `housekeep chans` (captured live via `spec/usage/housekeep/
/// chans.txt`, WP-0.2; re-confirmed live this slice). One fewer
/// trailing blank line than the raw capture -- see
/// `housekeep::USAGE`'s doc.
pub const USAGE: &str = "CDP Release 7.1 2016
EXTRACT OR CONVERT CHANNELS OF SOUNDFILE.

USAGE: housekeep chans 1 infile channo
OR:    housekeep chans 2 infile 
OR:    housekeep chans 3 infile outfile channo
OR:    housekeep chans 4 infile outfile [-p]
OR:    housekeep chans 5 infile outfile

MODES ARE
1) EXTRACT A CHANNEL
         channo is channel to extract
         outfile is named inname_c1 (for channel 1) etc.
2) EXTRACT ALL CHANNELS
         outfiles are named inname_c1 (for channel 1) etc.
3) ZERO ONE CHANNEL
         channo is channel to zero
         mono file goes to just one side of stereo outfile.
         multichannel files have one channel zeroed out.
4) MIX DOWN TO MONO
         -p inverts phase of 2nd stereo channel before mixing.
         (phase inversion has no effect with non-stereo files).
5) MONO TO STEREO
         Creates a 2-channel equivalent of mono infile.
";

pub const MAX_MODE: u32 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// legacy: `HOUSE_CHANNEL`.
    ExtractChannel,
    /// legacy: `HOUSE_ZCHANNEL`.
    ZeroChannel,
    /// legacy: `STOM`.
    MixToMono,
    /// legacy: `MTOS`.
    MonoToStereo,
}

impl Mode {
    /// `None` for mode 2 (not implemented yet) -- the caller maps
    /// that to a plain `ProgramError`, the same shape
    /// `cdp_programs::sndinfo::units::Mode::from_number` already
    /// established for a mode `1..=MAX_MODE` in range but not yet
    /// ported.
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Mode::ExtractChannel),
            3 => Some(Mode::ZeroChannel),
            4 => Some(Mode::MixToMono),
            5 => Some(Mode::MonoToStereo),
            _ => None,
        }
    }
}

/// legacy: `insert_new_chars_at_filename_end(outfilename,"_c")` then
/// `insert_new_number_at_filename_end(outfilename,k+1,0)`
/// (`legacy/dev/houskeep/channels.c`), inserted before the infile's
/// own extension (or appended at the end when there is none), with
/// the channel number un-padded -- confirmed live: `marimba.wav`
/// channel 1 writes `marimba_c1.wav`. Only considers a `.` after the
/// last path separator an extension, matching legacy's own backward
/// scan (`legacy/dev/cdp2k/mainfuncs.c`'s helpers stop at the first
/// `/`/`\`/`:` encountered).
fn numbered_channel_filename(infile_path: &str, channo: i64) -> String {
    let sep = infile_path
        .rfind(['/', '\\', ':'])
        .map(|i| i + 1)
        .unwrap_or(0);
    let (dir, base) = infile_path.split_at(sep);
    match base.rfind('.') {
        Some(dot) if dot > 0 => {
            format!("{dir}{}_c{channo}{}", &base[..dot], &base[dot..])
        }
        _ => format!("{infile_path}_c{channo}"),
    }
}

/// legacy: `do_channels`'s `case(HOUSE_CHANNEL)` (`legacy/dev/
/// houskeep/channels.c`). `channo` is the 1-based channel number
/// already range-checked by `cdp_params::parse` against
/// `1..=sf.fmt.channels` -- see this module's doc.
pub fn extract_channel(sf: &SoundFile, infile_path: &str, channo: i64) -> Result<String, CdpError> {
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: only sound files are implemented yet",
        ));
    }
    if !matches!(
        sf.fmt.sample_type,
        SampleType::Short16 | SampleType::Float32
    ) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: this sample format is not implemented yet",
        ));
    }
    let outfile_path = numbered_channel_filename(infile_path, channo);
    check_outfile_does_not_exist(&outfile_path)?;
    let samples = sf.samples_f32().map_err(CdpError::from)?;
    let channels = sf.fmt.channels as usize;
    let channel_index = (channo - 1) as usize;
    let mono: Vec<f32> = samples
        .iter()
        .skip(channel_index)
        .step_by(channels)
        .copied()
        .collect();
    write_wave_file(
        1,
        sf.fmt.sample_rate,
        sf.fmt.sample_type,
        &mono,
        &outfile_path,
    )?;
    Ok(outfile_path)
}

/// legacy: `do_channels`'s `case(HOUSE_ZCHANNEL)` (`legacy/dev/
/// houskeep/channels.c`). `channo` is the 1-based channel to zero,
/// already range-checked by `cdp_params::parse` against
/// `1..=sf.fmt.channels` (the same dynamic bound [`extract_channel`]
/// uses). Scope: mono and stereo infiles only, matching every other
/// mode in this module -- more than two channels would take this
/// mode's own generalised `default:` branch (zero every `channo`th
/// sample across an arbitrary channel count), not confirmed live
/// since no corpus file has more than two channels, so it reports a
/// plain `ProgramError` instead.
///
/// Mono infile: output is stereo, one side zero and the other the
/// original mono signal -- `channo` picks which side (legacy:
/// `zeroed`/`not_zeroed` are `0`/`1` swapped by `channo`, but a mono
/// infile's own range check only ever allows `channo == 1`, confirmed
/// live, so `not_zeroed` is always `1` in practice: the real signal
/// always lands on the second, right channel). legacy's own in-place
/// backward-expansion loop for this case has a real, confirmed bug
/// (`docs/migration/LEGACY-BUGS.md`) that corrupts the first sample of
/// every internal read buffer to `0`; this port builds the expanded
/// buffer fresh instead, avoiding it entirely (the intended behavior,
/// not the bug).
///
/// Stereo infile: output stays stereo, with the named channel zeroed
/// throughout and the other left untouched -- confirmed live,
/// byte-for-byte, against `docs/manual/sounds/clashmixtest.wav`, no
/// equivalent bug (a plain forward loop, no in-place aliasing).
///
/// legacy quirk, confirmed live, the same one [`mono_to_stereo`]'s own
/// doc describes: an already-existing outfile reports a bare
/// `"ERROR: INVALID DATA"` with an *empty* detail message, not
/// [`check_outfile_does_not_exist`]'s `"Cannot open output file
/// %s\n"`.
pub fn zero_channel(sf: &SoundFile, outfile_path: &str, channo: i64) -> Result<(), CdpError> {
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: only sound files are implemented yet",
        ));
    }
    if !matches!(
        sf.fmt.sample_type,
        SampleType::Short16 | SampleType::Float32
    ) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: this sample format is not implemented yet",
        ));
    }
    if sf.fmt.channels > 2 {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: zeroing a channel in more than two is not implemented yet",
        ));
    }
    if std::path::Path::new(outfile_path).exists() {
        return Err(CdpError::new(ExitCategory::DataError, ""));
    }
    let zero_index = (channo - 1) as usize;
    let (out_channels, samples) = if sf.fmt.channels == 1 {
        let mono = sf.samples_f32().map_err(CdpError::from)?;
        let mut stereo = Vec::with_capacity(mono.len() * 2);
        for &s in &mono {
            if zero_index == 0 {
                stereo.push(0.0);
                stereo.push(s);
            } else {
                stereo.push(s);
                stereo.push(0.0);
            }
        }
        (2u16, stereo)
    } else {
        let mut stereo = sf.samples_f32().map_err(CdpError::from)?;
        for (i, sample) in stereo.iter_mut().enumerate() {
            if i % 2 == zero_index {
                *sample = 0.0;
            }
        }
        (2u16, stereo)
    };
    write_wave_file(
        out_channels,
        sf.fmt.sample_rate,
        sf.fmt.sample_type,
        &samples,
        outfile_path,
    )?;
    // legacy: `display_virtual_time`'s `SNDFILE_OUT` branch -- `secs =
    // samps_sent/(infile->srate * infile->channels)`. For a mono
    // infile this doubles the real duration (the expanded stereo
    // sample count divided by the mono infile's own channel count,
    // `1`), the same skew `mono_to_stereo` already established; for a
    // stereo infile the counts already match, giving the real
    // duration exactly -- both confirmed live.
    super::print_virtual_time(
        samples.len() as f64 / (sf.fmt.sample_rate as f64 * sf.fmt.channels as f64),
    );
    Ok(())
}

/// legacy: `do_channels`'s `case(STOM)`, `case(STEREO)` branch (the
/// only branch this port implements -- see this module's doc). A mono
/// infile reports `"This file is already mono!!"` (`GOAL_FAILED`,
/// confirmed live); any other channel count would take the
/// "thumbnail" multichannel-downmix branch (a two-pass, normalising
/// algorithm), not ported -- reports a plain `ProgramError` instead of
/// the (unconfirmed) source text, since this is a real port gap, not
/// an observed legacy behaviour. `invert_phase` is legacy's `-p`
/// variant (`CHAN_INVERT_PHASE`): averages the two channels when
/// `false` (`(L+R)/2.0`), takes their half-difference when `true`
/// (`(L-R)/2.0`) -- both confirmed live, bit-exact, against
/// `docs/manual/sounds/clashmixtest.wav`.
///
/// legacy quirk, confirmed live, the same one [`mono_to_stereo`]'s own
/// doc describes: an already-existing outfile reports a bare
/// `"ERROR: INVALID DATA"` with an *empty* detail message, not
/// [`check_outfile_does_not_exist`]'s `"Cannot open output file
/// %s\n"`.
pub fn mix_to_mono(sf: &SoundFile, outfile_path: &str, invert_phase: bool) -> Result<(), CdpError> {
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: only sound files are implemented yet",
        ));
    }
    if !matches!(
        sf.fmt.sample_type,
        SampleType::Short16 | SampleType::Float32
    ) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: this sample format is not implemented yet",
        ));
    }
    match sf.fmt.channels {
        1 => {
            return Err(CdpError::new(
                ExitCategory::GoalFailed,
                "This file is already mono!!\n",
            ));
        }
        2 => {}
        _ => {
            return Err(CdpError::new(
                ExitCategory::ProgramError,
                "housekeep chans: mixing down more than two channels is not implemented yet",
            ));
        }
    }
    if std::path::Path::new(outfile_path).exists() {
        return Err(CdpError::new(ExitCategory::DataError, ""));
    }
    let stereo = sf.samples_f32().map_err(CdpError::from)?;
    let mono: Vec<f32> = stereo
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            if invert_phase {
                (pair[0] - pair[1]) / 2.0
            } else {
                (pair[0] + pair[1]) / 2.0
            }
        })
        .collect();
    write_wave_file(
        1,
        sf.fmt.sample_rate,
        sf.fmt.sample_type,
        &mono,
        outfile_path,
    )?;
    // legacy: `display_virtual_time`'s `SNDFILE_OUT` branch -- `secs =
    // samps_sent/(infile->srate * infile->channels)`, using the
    // *mono* output sample count written but the *stereo* infile's
    // own channel count (2) as the divisor, confirmed live to report
    // exactly half the file's real duration (`"\r0 min  1.25 sec"`
    // for `clashmixtest.wav`, a real 2.49-second file) -- see
    // `super::print_virtual_time`'s own doc for what this simplifies.
    super::print_virtual_time(
        mono.len() as f64 / (sf.fmt.sample_rate as f64 * sf.fmt.channels as f64),
    );
    Ok(())
}

/// legacy: `do_channels`'s `case(MTOS)`, `case(MONO)` branch (the
/// only branch this port implements -- see this module's doc). A
/// stereo infile reports `"This file is already stereo!!"`
/// (`GOAL_FAILED`, confirmed live); any other channel count reports
/// `"This process does not work with multichannel files!!"`, ported
/// from source but not confirmed live (no corpus file has more than 2
/// channels).
///
/// legacy quirk, confirmed live and ported as observed: this mode's
/// own outfile-creation path (`sndcreat_formatted`, not the `WAVE_EX`
/// branch `dz->outfile->channels > 2` would take) checks
/// `if(dz->ofd < 0) return DATA_ERROR;` with no `sprintf(errstr,...)`
/// call at all on that path, so an already-existing outfile reports a
/// bare `"ERROR: INVALID DATA"` header with an *empty* detail message
/// -- confirmed live, this is not the `"Cannot open output file
/// %s\n"` text [`check_outfile_does_not_exist`] produces for `copy`/
/// mode 1's own outfile-creation path, so this function does not use
/// that helper.
pub fn mono_to_stereo(sf: &SoundFile, outfile_path: &str) -> Result<(), CdpError> {
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: only sound files are implemented yet",
        ));
    }
    if !matches!(
        sf.fmt.sample_type,
        SampleType::Short16 | SampleType::Float32
    ) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep chans: this sample format is not implemented yet",
        ));
    }
    match sf.fmt.channels {
        1 => {}
        2 => {
            return Err(CdpError::new(
                ExitCategory::GoalFailed,
                "This file is already stereo!!\n",
            ));
        }
        _ => {
            return Err(CdpError::new(
                ExitCategory::GoalFailed,
                "This process does not work with multichannel files!!\n",
            ));
        }
    }
    if std::path::Path::new(outfile_path).exists() {
        return Err(CdpError::new(ExitCategory::DataError, ""));
    }
    let mono = sf.samples_f32().map_err(CdpError::from)?;
    let mut stereo = Vec::with_capacity(mono.len() * 2);
    for &s in &mono {
        stereo.push(s);
        stereo.push(s);
    }
    write_wave_file(
        2,
        sf.fmt.sample_rate,
        sf.fmt.sample_type,
        &stereo,
        outfile_path,
    )?;
    // legacy: `display_virtual_time`'s `SNDFILE_OUT` branch -- `secs =
    // samps_sent/(infile->srate * infile->channels)`, using the
    // *interleaved stereo* sample count written but the *mono*
    // infile's own channel count (1) as the divisor, confirmed live
    // to report double the file's real duration
    // (`"\r0 min  2.00 sec"` for `marimba.wav`, a real 1.00-second
    // file) -- see `super::print_virtual_time`'s own doc for what
    // this simplifies.
    super::print_virtual_time(
        stereo.len() as f64 / (sf.fmt.sample_rate as f64 * sf.fmt.channels as f64),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    /// legacy writes every output sample through a 16-bit PCM encode
    /// (`(short)cdp_round(sample*MAXSHORT)`, `cdp_sf::writer`'s
    /// `encode_pcm16`), so a value this test *computes* (rather than
    /// merely passes through unmodified, like
    /// `stereo_clashmixtest_channel_2_matches...` below) must be
    /// quantised the same way before comparing against the real
    /// written-then-read-back file.
    fn quantize_i16(sample: f32) -> f32 {
        // legacy: `cdp_sf::writer`'s `encode_pcm16` multiplies and
        // rounds in `f64` (matching `fputfloatEx`'s own `double`
        // promotion of `MAXSHORT`), not `f32` -- see its own doc.
        const MAXSHORT: f64 = 32767.0;
        (((sample as f64) * MAXSHORT).round() / MAXSHORT) as f32
    }

    #[test]
    fn zero_channel_mono_infile_zeros_left_and_keeps_right() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("zchan_mono").join("marimba_z1.wav");
        let _ = std::fs::remove_file(&out_path);
        zero_channel(&sf, out_path.to_str().unwrap(), 1).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.channels, 2);
        let mono = sf.samples_f32().unwrap();
        let stereo = out.samples_f32().unwrap();
        let left: Vec<f32> = stereo.iter().step_by(2).copied().collect();
        let right: Vec<f32> = stereo.iter().skip(1).step_by(2).copied().collect();
        // legacy bug (docs/migration/LEGACY-BUGS.md): the real C loop
        // corrupts the first sample of every internal read buffer to
        // 0. This port avoids that bug entirely, so `right` matches
        // `mono` bit-exactly, including at index 0.
        assert!(left.iter().all(|&v| v == 0.0));
        assert_eq!(right, mono);
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn zero_channel_stereo_infile_zeros_only_the_named_channel() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/clashmixtest.wav")).unwrap();
        let out_path = scratch_dir("zchan_stereo").join("clash_z2.wav");
        let _ = std::fs::remove_file(&out_path);
        zero_channel(&sf, out_path.to_str().unwrap(), 2).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.channels, 2);
        let orig = sf.samples_f32().unwrap();
        let zeroed = out.samples_f32().unwrap();
        let orig_left: Vec<f32> = orig.iter().step_by(2).copied().collect();
        let zeroed_left: Vec<f32> = zeroed.iter().step_by(2).copied().collect();
        let zeroed_right: Vec<f32> = zeroed.iter().skip(1).step_by(2).copied().collect();
        assert_eq!(zeroed_left, orig_left);
        assert!(zeroed_right.iter().all(|&v| v == 0.0));
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn zero_channel_existing_outfile_is_a_bare_data_error_with_no_message() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("zchan_exists").join("exists.wav");
        std::fs::write(&out_path, b"not a real sound file").unwrap();
        let err = zero_channel(&sf, out_path.to_str().unwrap(), 1).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(err.to_string(), "");
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn mix_to_mono_averages_left_and_right() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/clashmixtest.wav")).unwrap();
        let out_path = scratch_dir("stom").join("clash_mono.wav");
        let _ = std::fs::remove_file(&out_path);
        mix_to_mono(&sf, out_path.to_str().unwrap(), false).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.channels, 1);
        let stereo = sf.samples_f32().unwrap();
        let expected: Vec<f32> = stereo
            .as_chunks::<2>()
            .0
            .iter()
            .map(|p| quantize_i16((p[0] + p[1]) / 2.0))
            .collect();
        assert_eq!(out.samples_f32().unwrap(), expected);
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn mix_to_mono_with_invert_phase_takes_the_half_difference() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/clashmixtest.wav")).unwrap();
        let out_path = scratch_dir("stom_invert").join("clash_mono_p.wav");
        let _ = std::fs::remove_file(&out_path);
        mix_to_mono(&sf, out_path.to_str().unwrap(), true).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        let stereo = sf.samples_f32().unwrap();
        let expected: Vec<f32> = stereo
            .as_chunks::<2>()
            .0
            .iter()
            .map(|p| quantize_i16((p[0] - p[1]) / 2.0))
            .collect();
        assert_eq!(out.samples_f32().unwrap(), expected);
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn mix_to_mono_rejects_an_already_mono_infile() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let err = mix_to_mono(&sf, "irrelevant.wav", false).unwrap_err();
        assert_eq!(err.category, ExitCategory::GoalFailed);
        assert_eq!(err.to_string(), "This file is already mono!!\n");
    }

    #[test]
    fn mix_to_mono_existing_outfile_is_a_bare_data_error_with_no_message() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/clashmixtest.wav")).unwrap();
        let out_path = scratch_dir("stom_exists").join("exists.wav");
        std::fs::write(&out_path, b"not a real sound file").unwrap();
        let err = mix_to_mono(&sf, out_path.to_str().unwrap(), false).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(err.to_string(), "");
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn mono_to_stereo_doubles_marimba_into_both_channels() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("mtos").join("marimba_stereo.wav");
        let _ = std::fs::remove_file(&out_path);
        mono_to_stereo(&sf, out_path.to_str().unwrap()).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.channels, 2);
        let mono = sf.samples_f32().unwrap();
        let stereo = out.samples_f32().unwrap();
        let left: Vec<f32> = stereo.iter().step_by(2).copied().collect();
        let right: Vec<f32> = stereo.iter().skip(1).step_by(2).copied().collect();
        assert_eq!(left, mono);
        assert_eq!(right, mono);
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn mono_to_stereo_rejects_an_already_stereo_infile() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/clashmixtest.wav")).unwrap();
        let err = mono_to_stereo(&sf, "irrelevant.wav").unwrap_err();
        assert_eq!(err.category, ExitCategory::GoalFailed);
        assert_eq!(err.to_string(), "This file is already stereo!!\n");
    }

    #[test]
    fn mono_to_stereo_existing_outfile_is_a_bare_data_error_with_no_message() {
        // legacy quirk, confirmed live -- see `mono_to_stereo`'s own
        // doc: unlike `copy`/`extract_channel`, this mode's own
        // overwrite-refusal carries no detail text at all.
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_dir("mtos_exists").join("exists.wav");
        std::fs::write(&out_path, b"not a real sound file").unwrap();
        let err = mono_to_stereo(&sf, out_path.to_str().unwrap()).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(err.to_string(), "");
        let _ = std::fs::remove_file(&out_path);
    }

    fn scratch_dir(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "cdp-housekeep-chans-test-{}-{name}",
            std::process::id()
        ));
        let _ = std::fs::create_dir_all(&p);
        p
    }

    #[test]
    fn numbered_channel_filename_inserts_before_the_extension() {
        assert_eq!(
            numbered_channel_filename("marimba.wav", 1),
            "marimba_c1.wav"
        );
        assert_eq!(
            numbered_channel_filename("/dir/marimba.wav", 2),
            "/dir/marimba_c2.wav"
        );
        assert_eq!(numbered_channel_filename("noext", 1), "noext_c1");
    }

    #[test]
    fn mono_marimba_channel_1_matches_the_infile_exactly() {
        let dir = scratch_dir("marimba");
        let infile = dir.join("marimba.wav");
        std::fs::copy(repo_path("docs/manual/sounds/marimba.wav"), &infile).unwrap();
        let out_path = dir.join("marimba_c1.wav");
        let _ = std::fs::remove_file(&out_path);
        let sf = SoundFile::open(&infile).unwrap();
        let written = extract_channel(&sf, infile.to_str().unwrap(), 1).unwrap();
        assert_eq!(written, out_path.to_str().unwrap());
        let out = SoundFile::open(&out_path).unwrap();
        assert_eq!(out.fmt.channels, 1);
        assert_eq!(out.samples_f32().unwrap(), sf.samples_f32().unwrap());
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn stereo_clashmixtest_channel_2_matches_the_second_interleaved_channel() {
        let dir = scratch_dir("clash");
        let infile = dir.join("clashmixtest.wav");
        std::fs::copy(repo_path("docs/manual/sounds/clashmixtest.wav"), &infile).unwrap();
        let out_path = dir.join("clashmixtest_c2.wav");
        let _ = std::fs::remove_file(&out_path);
        let sf = SoundFile::open(&infile).unwrap();
        extract_channel(&sf, infile.to_str().unwrap(), 2).unwrap();
        let out = SoundFile::open(&out_path).unwrap();
        let expected: Vec<f32> = sf
            .samples_f32()
            .unwrap()
            .iter()
            .skip(1)
            .step_by(2)
            .copied()
            .collect();
        assert_eq!(out.fmt.channels, 1);
        assert_eq!(out.samples_f32().unwrap(), expected);
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn refuses_to_overwrite_an_existing_outfile() {
        let dir = scratch_dir("exists");
        let infile = dir.join("marimba.wav");
        std::fs::copy(repo_path("docs/manual/sounds/marimba.wav"), &infile).unwrap();
        let out_path = dir.join("marimba_c1.wav");
        std::fs::write(&out_path, b"not a real sound file").unwrap();
        let sf = SoundFile::open(&infile).unwrap();
        let err = extract_channel(&sf, infile.to_str().unwrap(), 1).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!("Cannot open output file {}\n", out_path.to_str().unwrap())
        );
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn analysis_file_is_not_implemented_yet() {
        let sf = SoundFile::open(repo_path("docs/manual/data/capm.ana")).unwrap();
        let err = extract_channel(&sf, &repo_path("docs/manual/data/capm.ana"), 1).unwrap_err();
        assert!(err.to_string().contains("implemented yet"));
    }
}
