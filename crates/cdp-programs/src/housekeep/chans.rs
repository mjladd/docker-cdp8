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
//! channel to a new, auto-named mono file) and 5 (`MTOS`, `housekeep
//! chans 5 infile outfile`: doubles a mono infile's samples into a
//! stereo outfile) are ported so far. Modes 2-4 (extract all
//! channels, zero one channel, mix down to mono) report a plain
//! `ProgramError`.
//!
//! legacy: `legacy/dev/houskeep/channels.c`'s `do_channels`,
//! `case(HOUSE_CHANNEL)` and (for mode 5) `case(MTOS)` (shared with
//! `HOUSE_ZCHANNEL`/`STOM` -- mode 2, `HOUSE_CHANNELS`, has its own,
//! not-yet-ported loop bounds change: `start_chan=0`/
//! `end_chan=dz->infile->channels`, but needs [`extract_channel`]'s
//! per-channel behaviour repeated once per channel, not yet confirmed
//! live for a real multi-channel file).
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
    /// legacy: `MTOS`.
    MonoToStereo,
}

impl Mode {
    /// `None` for modes 2-4 (not implemented yet) -- the caller maps
    /// that to a plain `ProgramError`, the same shape
    /// `cdp_programs::sndinfo::units::Mode::from_number` already
    /// established for a mode `1..=MAX_MODE` in range but not yet
    /// ported.
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Mode::ExtractChannel),
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
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
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
