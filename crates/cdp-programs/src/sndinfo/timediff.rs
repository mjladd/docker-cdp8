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

//! `sndinfo timediff infile1 infile2`: prints the absolute difference
//! in duration between two sound files. legacy:
//! `legacy/dev/sndinfo/compare.c`'s `case(INFO_TIMEDIFF)` block.
//!
//! `INFO_TIMEDIFF` is `TWO_SNDFILES` in `ap_sndinfo.c`'s
//! `setup_process_logic`, the first command in this crate needing two
//! independently-opened input files (`dz->infile`/`dz->otherfile`)
//! rather than one -- `set_legal_infile_structure`'s `has_otherfile`
//! lets the second file's sample rate and channel count differ from
//! the first's. Confirmed live with `marimba.wav` (a 44100 Hz mono
//! file) against `ws2/tsw1-2nd.aiff` (a different sample rate and
//! format): `DIFFERENCE IS 4.465488 secs`, exactly `len::
//! wave_duration_secs`'s two independently-confirmed values
//! subtracted.
//!
//! This command's argument handling is written by hand rather than
//! through `cdp_params::parse` (unlike every other command in this
//! crate so far). Live legacy validates the two infiles in a strict,
//! interleaved order: infile1 must exist, then infile1's kind must be
//! a plain sound file, then -- only after both of those pass --
//! infile2 must exist, then infile2's kind must be a plain sound file
//! too. Confirmed live in both directions: `sndinfo timediff capm.ana
//! nonexistent.wav` reports the infile1-kind error ("Application
//! doesn't work with this type of infile.") without ever trying to
//! open infile2, and `sndinfo timediff marimba.wav capm.ana` (a valid
//! infile1) reports a *different*, infile2-specific error instead ("
//! capm.ana is not a sound file."). A generic two-infile existence
//! pre-check -- the shape `cdp_params::CommandSpec::infile_count: 2`
//! would give this command, the same as every other multi-infile
//! command in `legacy/dev/cdp2k/mainfuncs.c`'s
//! `count_and_allocate_for_infiles` -- checks both files' existence
//! before either file's kind, which would report the wrong error for
//! the first live case above. No other command ported so far needs
//! this interleaving, so it is implemented directly here instead of
//! being added to `cdp_params`'s generic parser.
//!
//! legacy also reports a genuinely different message for each
//! infile's own open failure. infile1 uses the same `"Can't open file
//! %s to read data."` text every other command's first infile does
//! ([`super::open_sound_infile`], `legacy/dev/cdp2k/mainfuncs.c`'s
//! `parse_infile_and_hone_type`/`cdparse`). infile2 uses `"cannot open
//! input file %s to read data."` instead
//! (`legacy/dev/cdp2k/readfiles.c`'s
//! `open_checktype_getsize_and_compareheader`, the generic
//! second-infile-open path every `TWO_SNDFILES`-family command
//! shares). Confirmed live, and ported as observed rather than
//! unified into one text.
//!
//! legacy bug, not reproduced: real legacy segfaults (confirmed live,
//! exit code 139/SIGSEGV, no output at all -- not even the usual
//! `argc<4` greeting) when given exactly one infile instead of two.
//! `count_and_allocate_for_infiles` sets `dz->infilecnt = 2` for
//! `TWO_SNDFILES`, and `handle_extra_infiles`
//! (`legacy/dev/cdp2k/mainfuncs.c`) then reads `filename =
//! (*cmdline)[0]` in its `n=1..dz->infilecnt` loop with no check that
//! `*cmdlinecnt` still has a token left -- a plain out-of-bounds
//! `argv` read. Logged in `docs/migration/LEGACY-BUGS.md`. This port
//! reports a clean [`cdp_params::ParamsError::InsufficientCmdlineParameters`]
//! instead of crashing. There is no real legacy output to match here
//! (legacy prints nothing before it segfaults), so this choice of
//! text is this crate's own, not a captured legacy string -- picked
//! because it is the same text every other command in this crate uses
//! for "too few tokens for the infiles this command needs".

use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SoundFile};

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo timediff` (captured live via `spec/usage/sndinfo/
/// timediff.txt`, WP-0.2). Bundles the greeting itself, the same
/// self-contained-fixture style `props::USAGE`/`len::USAGE` use.
/// Unlike those commands, this is the *only* case that ever prints
/// the greeting for `timediff`: a single-infile invocation crashes
/// real legacy before it would reach the `argc<4` greeting check (see
/// this module's doc), and a two-infile invocation is `argc==4`, not
/// `<4`, so the greeting never applies there either -- confirmed live.
pub const USAGE: &str = "CDP Release 7.1 2016
FIND DIFFERENCE IN DURATION OF TWO SOUND FILES

USAGE: sndreport timediff infile1 infile2
";

/// legacy: `case(INFO_TIMEDIFF)` in `compare.c`: `secs = insams[0]/
/// chans*inverse_sr - insams[1]/other_chans/other_sr`, then
/// `fabs(secs)`. Returns the complete output text, including its own
/// trailing newline (legacy: `strcat(errstr,"\n")` after the
/// [`super::len::format_duration`] text, which does not end in one
/// itself).
pub fn format_timediff(sf1: &SoundFile, sf2: &SoundFile) -> String {
    let secs1 = super::len::wave_duration_secs(sf1);
    let secs2 = super::len::wave_duration_secs(sf2);
    let secs = (secs1 - secs2).abs();
    format!("DIFFERENCE IS {}\n", super::len::format_duration(secs))
}

/// Opens and validates both infiles in legacy's own confirmed order
/// -- see this module's doc for why this cannot just call
/// [`super::open_sound_infile`] twice through `cdp_params::parse`.
pub fn open_infiles(path1: &str, path2: &str) -> Result<(SoundFile, SoundFile), CdpError> {
    let sf1 = super::open_sound_infile(path1)?;
    let sf2 = open_second_sound_infile(path2)?;
    Ok((sf1, sf2))
}

/// legacy: `open_checktype_getsize_and_compareheader`'s
/// `TWO_SNDFILES` arm (open failure) and
/// `check_later_filetype_valid_for_process`'s `TWO_SNDFILES` arm
/// (wrong kind) -- both `legacy/dev/cdp2k/readfiles.c`, both
/// `DATA_ERROR`. See this module's doc for the exact confirmed text
/// and why it differs from [`super::open_sound_infile`]'s.
fn open_second_sound_infile(path: &str) -> Result<SoundFile, CdpError> {
    std::fs::File::open(path).map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("cannot open input file {path} to read data."),
        )
    })?;
    let sf = SoundFile::open(path).map_err(CdpError::from)?;
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::DataError,
            format!("{path} is not a sound file."),
        ));
    }
    Ok(sf)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    #[test]
    fn marimba_wav_vs_tsw1_2nd_aiff_matches_the_real_legacy_sndinfo_timediff_output() {
        let sf1 = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let sf2 = SoundFile::open(repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff")).unwrap();
        assert_eq!(
            format_timediff(&sf1, &sf2),
            "DIFFERENCE IS 4.465488 secs \n"
        );
    }

    #[test]
    fn difference_is_symmetric_regardless_of_infile_order() {
        let sf1 = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let sf2 = SoundFile::open(repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff")).unwrap();
        assert_eq!(format_timediff(&sf1, &sf2), format_timediff(&sf2, &sf1));
    }

    #[test]
    fn identical_infiles_have_zero_difference() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let sf2 = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        assert_eq!(format_timediff(&sf, &sf2), "DIFFERENCE IS 0.000000 secs \n");
    }

    #[test]
    fn open_infiles_rejects_a_non_sound_second_infile_with_the_infile2_specific_text() {
        let path1 = repo_path("docs/manual/sounds/marimba.wav");
        let path2 = repo_path("docs/manual/data/capm.ana");
        let err = open_infiles(&path1, &path2).unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(err.message, format!("{path2} is not a sound file."));
    }

    #[test]
    fn open_infiles_rejects_a_non_sound_first_infile_with_the_generic_wrong_filetype_text_before_ever_touching_infile2()
     {
        let path1 = repo_path("docs/manual/data/capm.ana");
        let err = open_infiles(&path1, "/nonexistent/does-not-exist.wav").unwrap_err();
        assert_eq!(err.category, ExitCategory::UsageOnly);
        assert_eq!(err.message, super::super::WRONG_FILETYPE);
    }

    #[test]
    fn open_infiles_reports_the_infile2_specific_open_error_for_a_nonexistent_second_infile() {
        let path1 = repo_path("docs/manual/sounds/marimba.wav");
        let err = open_infiles(&path1, "/nonexistent/does-not-exist.wav").unwrap_err();
        assert_eq!(err.category, ExitCategory::DataError);
        assert_eq!(
            err.message,
            "cannot open input file /nonexistent/does-not-exist.wav to read data."
        );
    }
}
