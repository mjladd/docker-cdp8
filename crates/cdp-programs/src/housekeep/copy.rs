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

//! `housekeep copy mode infile ...` (`HOUSE_COPY`). Only mode 1
//! (`COPYSF`, `housekeep copy 1 infile outfile`: writes an
//! unmodified copy of infile's samples to outfile) is ported so far.
//! Mode 2 (`DUPL`, `housekeep copy 2 infile count [-i]`: writes
//! `count` auto-named copies, `X_001.wav`, `X_002.wav`, ...) reports a
//! plain `ProgramError`.
//!
//! legacy: `legacy/dev/houskeep/dupl.c`'s `do_duplicates`,
//! `case(COPYSF)`.
//!
//! Scope of mode 1: [`cdp_sf::FileKind::Wave`] with
//! [`cdp_sf::SampleType::Short16`] or `Float32` sample data -- the
//! only two shapes [`cdp_sf::SoundFileWriter`] can write yet (see its
//! own module doc). `Analysis`/`Envelope`/`Pitch`/`Transposition`/
//! `Formant` and any other `SampleType` report a plain
//! `ProgramError`; legacy's `COPYSF` case has its own special-cased
//! read/write path for each of `Pitch`("`PITCHFILE`", copies
//! `dz->pitches`), `Transposition`("`TRANSPOSFILE`", copies
//! `dz->transpos`) and `Envelope`("`ENVFILE`", copies `window_size`
//! alongside the samples), and a from-scratch analysis-header write
//! for `Analysis` -- none of which this port attempts yet, since
//! `SoundFileWriter` has no equivalent of any of them.
//!
//! This is the first `cdp-programs` command that both reads and
//! writes real sample data (every `sndinfo` sub-command ported so far
//! only reads). Confirmed live, and ported as-is rather than treated
//! as a bug: legacy's copy is not a byte-for-byte file copy -- the
//! output is always written as `WAVE` (even when infile is `AIFF`,
//! confirmed live against `docs/manual/sounds/ws2/tsw1-2nd.aiff`),
//! always gains a freshly-computed `PEAK` chunk and a fresh `DATE`
//! property (`legacy/dev/cdp2k/mainfuncs.c`'s `create_sized_outfile`/
//! `establish_peak_status`, shared by every `SNDFILE_OUT` program),
//! and drops every other named property the infile had (confirmed
//! live: `marimba.wav`'s own `maxamp`/`maxloc`/`maxrep` properties --
//! see `cdp_programs::sndinfo::maxsamp`'s module doc -- do not survive
//! the copy). Sample data itself is bit-exact (confirmed live via
//! Python's own `wave` module on a real stereo file,
//! `docs/manual/sounds/clashmixtest.wav`).
//!
//! Also confirmed live, and a real, generic legacy behaviour this
//! slice's own regression tests do not exercise directly (no
//! in-process filesystem side effect in a unit test): every
//! `SNDFILE_OUT` program's `create_sized_outfile` refuses to
//! overwrite an already-existing outfile
//! (`"Cannot open output file %s\n"`, `DATA_ERROR`) -- confirmed live
//! against a real pre-existing outfile, not merely inferred from
//! source. `cdp_sf::SoundFileWriter::finalize` does not check this
//! itself (it always overwrites via `std::fs::write`), so
//! [`copy_once`] checks it directly rather than relying on the
//! writer. This same check likely applies to every other
//! `SNDFILE_OUT` program already shipped (`synth wave`, `pvoc anal`),
//! neither of which currently enforces it -- a real, pre-existing gap
//! this slice did not fix, since those two commands belong to
//! WP-1.4/WP-1.5, not this one.

use super::{check_outfile_does_not_exist, write_wave_file};
use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SampleType, SoundFile};

/// legacy: `legacy/dev/houskeep/main.c`'s `make_initial_cmdline_check`
/// -- see `cdp_programs::sndinfo::props::GREETING`'s doc, which this
/// mirrors exactly (confirmed live the same way: `housekeep copy 1`,
/// with no further tokens at all, prints it; `housekeep copy 1
/// infile`, one further token, does not).
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/houskeep/main.c` prints for a
/// bare `housekeep copy` (captured live via `spec/usage/housekeep/
/// copy.txt`, WP-0.2; re-confirmed live this slice). One fewer
/// trailing blank line than the raw capture -- see
/// `housekeep::USAGE`'s doc.
pub const USAGE: &str = "CDP Release 7.1 2016
MAKE COPIES OF THE INFILE

USAGE: housekeep copy 1 infile outfile
OR:    housekeep copy 2 infile count [-i]
MODES ARE
1) COPY ONCE
2) COPY MANY
         Produces N copies of infile X, with names X_001,X_002...
         COUNT is number of duplicates to produce.
     -i  ignore existing duplicates (don't overwrite them).
         default, process halts on discovering a pre-existing file.
";

pub const MAX_MODE: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// legacy: `COPYSF`.
    Once,
}

impl Mode {
    /// `None` for mode 2 (`DUPL`, not implemented yet) -- the caller
    /// maps that to a plain `ProgramError`, the same shape
    /// `cdp_programs::sndinfo::units::Mode::from_number` already
    /// established for a mode `1..=MAX_MODE` in range but not yet
    /// ported.
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Mode::Once),
            _ => None,
        }
    }
}

/// legacy: `do_duplicates`'s `case(COPYSF)` default branch (plain
/// `SNDFILE`/`ANALFILE` sample data, read via `read_samps` and
/// written back via `write_exact_samps`, unchanged).
pub fn copy_once(sf: &SoundFile, outfile_path: &str) -> Result<(), CdpError> {
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep copy: only sound files are implemented yet",
        ));
    }
    if !matches!(
        sf.fmt.sample_type,
        SampleType::Short16 | SampleType::Float32
    ) {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "housekeep copy: this sample format is not implemented yet",
        ));
    }
    check_outfile_does_not_exist(outfile_path)?;
    let samples = sf.samples_f32().map_err(CdpError::from)?;
    write_wave_file(
        sf.fmt.channels,
        sf.fmt.sample_rate,
        sf.fmt.sample_type,
        &samples,
        outfile_path,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    fn scratch_path(name: &str) -> std::path::PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!(
            "cdp-housekeep-copy-test-{}-{name}",
            std::process::id()
        ));
        p
    }

    #[test]
    fn marimba_wav_copy_round_trips_sample_data_and_format_exactly() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_path("marimba.wav");
        let _ = std::fs::remove_file(&out_path);
        copy_once(&sf, out_path.to_str().unwrap()).unwrap();
        let copy = SoundFile::open(&out_path).unwrap();
        assert_eq!(copy.fmt.channels, sf.fmt.channels);
        assert_eq!(copy.fmt.sample_rate, sf.fmt.sample_rate);
        assert_eq!(copy.fmt.sample_type, sf.fmt.sample_type);
        assert_eq!(copy.samples_f32().unwrap(), sf.samples_f32().unwrap());
        // legacy: DATE survives (freshly stamped, not copied from
        // infile), but maxamp/maxloc/maxrep do not -- see this
        // module's doc.
        assert!(copy.properties.get_i32("DATE").is_ok());
        assert!(copy.properties.get_i32("maxamp").is_err());
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn tsw1_2nd_aiff_copy_is_written_as_wave_not_aiff() {
        // legacy: confirmed live -- see this module's doc.
        let sf = SoundFile::open(repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff")).unwrap();
        let out_path = scratch_path("tsw1.wav");
        let _ = std::fs::remove_file(&out_path);
        copy_once(&sf, out_path.to_str().unwrap()).unwrap();
        let bytes = std::fs::read(&out_path).unwrap();
        assert_eq!(&bytes[0..4], b"RIFF");
        let copy = SoundFile::open(&out_path).unwrap();
        assert_eq!(copy.samples_f32().unwrap(), sf.samples_f32().unwrap());
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn refuses_to_overwrite_an_existing_outfile() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out_path = scratch_path("exists.wav");
        std::fs::write(&out_path, b"not a real sound file").unwrap();
        let err = copy_once(&sf, out_path.to_str().unwrap()).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!("Cannot open output file {}\n", out_path.to_str().unwrap())
        );
        let _ = std::fs::remove_file(&out_path);
    }

    #[test]
    fn analysis_file_is_not_implemented_yet() {
        let sf = SoundFile::open(repo_path("docs/manual/data/capm.ana")).unwrap();
        let out_path = scratch_path("capm.ana.wav");
        let err = copy_once(&sf, out_path.to_str().unwrap()).unwrap_err();
        assert!(err.to_string().contains("implemented yet"));
    }
}
