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

//! `sndinfo lens infile [infile2..]`: lists the duration of each of
//! two or more sound files, one per line. legacy:
//! `legacy/dev/sndinfo/compare.c`'s `case(INFO_TIMELIST)` block.
//!
//! `INFO_TIMELIST` is `MANY_SNDFILES` in `ap_sndinfo.c`'s
//! `assign_process_logic`, and (unlike [`super::sumlen`], see that
//! module's doc) `set_legal_infile_structure` also sets
//! `dz->has_otherfile = TRUE` for it. Confirmed live this means each
//! infile after the first is opened and measured entirely
//! independently, with no cross-file sample-rate or channel-count
//! consistency check at all: `sndinfo lens marimba.wav clip5-all.wav`
//! (a mono file followed by a stereo one) succeeds, printing each
//! file's own genuinely correct duration. `compare.c`'s loop confirms
//! this in the source too: for every infile after the first, it calls
//! `readhead`/`copy_to_fileptr` to refresh `dz->infile` to that file's
//! own header before computing its duration -- unlike `sumlen`, whose
//! loop never does this (see that module's doc for the consequence).
//!
//! At least two infiles are required, but the message differs
//! depending on exactly how the command line falls short of that,
//! confirmed live for both:
//! - Exactly one token after `lens` (`sndinfo lens infile`, real
//!   `argc==3`, regardless of whether that one infile is even valid):
//!   `legacy/dev/sndinfo/ap_sndinfo.c`'s `usage3` does not list
//!   `"lens"` among the sub-commands that `CONTINUE` past this case
//!   (unlike `props`/`len`), so it reports
//!   [`cdp_params::ParamsError::InsufficientParameters`] ("Insufficient
//!   parameters on command line.") instead of attempting to parse
//!   further.
//! - Two or more tokens, but fewer than two of them are actual infiles
//!   because one looks like a flag (`sndinfo lens infile -x`, real
//!   `argc==4`): `legacy/dev/cdp2k/mainfuncs.c`'s
//!   `count_and_allocate_for_infiles` reports
//!   [`super::INSUFFICIENT_INFILES`] instead, a different message from
//!   a different check.
//!
//! Argument handling is written by hand rather than through
//! `cdp_params::parse`, for the same underlying reason
//! [`super::timediff`]'s is (see that module's doc): the infile count
//! here is not fixed at all. legacy: `count_infiles`
//! (`legacy/dev/cdp2k/mainfuncs.c`) treats every remaining token that
//! does not look like a `-<letter>` flag as another infile, with no
//! upper bound -- this crate's `cdp_params::CommandSpec` does not
//! model that in general yet. [`super::split_infile_tokens`]
//! (shared with [`super::sumlen`]) reproduces the common case (all
//! infiles given up front, any flag-shaped token after them); it does
//! not attempt to reproduce infiles and flags genuinely interspersed
//! on the command line, a case this crate has no test coverage for.
//!
//! A leftover flag-shaped token once at least two infiles are already
//! accounted for reuses [`cdp_params::classify_trailing_token`] --
//! confirmed live for `sndinfo lens infile infile2 -x`
//! ("Unknown flag -x on command line.", the same
//! [`cdp_params::ParamsError::UnknownFlag`] text `sndinfo props`'s own
//! zero-flags-zero-variants tail check produces, see that function's
//! doc for the gap this slice found and fixed there).

use super::len::{format_duration, wave_duration_secs};
use cdp_core::CdpError;
use cdp_sf::SoundFile;

/// legacy: `legacy/dev/sndinfo/main.c`'s `make_initial_cmdline_check`,
/// unconditional whenever raw `argc<4` -- see `super::props::GREETING`'s
/// doc, which this mirrors exactly (confirmed live the same way, both
/// for `sndinfo lens` bare and `sndinfo lens infile`).
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo lens` (captured live via `spec/usage/sndinfo/
/// lens.txt`, WP-0.2). See `super::props::USAGE`'s doc for why this
/// bundles [`GREETING`] at the front and carries only one trailing
/// newline (not the fixture's own extra trailing blank lines).
pub const USAGE: &str = "CDP Release 7.1 2016
LIST DURATIONS OF SEVERAL SNDFILING-SYSTEM FILES

USAGE: sndreport lens infile [infile2..]
";

/// legacy: `SPACECNT` in `compare.c`, the column each filename is
/// padded to (with `.` characters) before its duration -- shared with
/// [`super::sumlen`]'s own doc reference, though `sumlen` never prints
/// per-file lines itself.
const SPACECNT: usize = 32;

/// Opens every path in `paths` in order: the first via
/// [`super::open_sound_infile`] (infile 1's own open/kind-check
/// wording), every subsequent one via
/// [`super::open_other_sound_infile`] (infile 2+'s different wording,
/// and -- confirmed live, see this module's doc -- no cross-file
/// sample-rate or channel-count check at all).
pub fn open_infiles(paths: &[&str]) -> Result<Vec<SoundFile>, CdpError> {
    let mut files = Vec::with_capacity(paths.len());
    for (i, &path) in paths.iter().enumerate() {
        let sf = if i == 0 {
            super::open_sound_infile(path)?
        } else {
            super::open_other_sound_infile(path)?
        };
        files.push(sf);
    }
    Ok(files)
}

/// legacy: `case(INFO_TIMELIST)` in `compare.c`. `paths` and `files`
/// must be the same length and in the same order (as
/// [`open_infiles`] returns them). Returns the complete output text,
/// each line already carrying its own trailing newline.
pub fn format_lens(paths: &[&str], files: &[SoundFile]) -> String {
    let mut out = String::new();
    for (path, sf) in paths.iter().zip(files) {
        out.push_str(path);
        let dots = SPACECNT.saturating_sub(path.len()).max(2);
        for _ in 0..dots {
            out.push('.');
        }
        out.push_str(&format_duration(wave_duration_secs(sf)));
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_path(rel: &str) -> String {
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    // `format_lens` treats `paths` purely as display text, decoupled
    // from whatever path actually opened each `SoundFile` (legacy:
    // `dz->wordstor[n]` is simply whatever token appeared on the
    // command line -- see this module's doc). The tests below open
    // each file via `repo_path` (an absolute path, since these tests
    // run from an arbitrary working directory) but format with the
    // exact short, relative-style names `sndinfo lens` was actually
    // invoked with when this output was captured live via
    // `cdp8-postmerge`, so the confirmed `SPACECNT` dot-padding counts
    // (14, 8, 12) apply exactly as they did live.

    #[test]
    fn marimba_and_tsw1_match_the_real_legacy_sndinfo_lens_output() {
        let files = open_infiles(&[
            &repo_path("docs/manual/sounds/marimba.wav"),
            &repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff"),
        ])
        .unwrap();
        let display_paths = ["sounds/marimba.wav", "sounds/ws2/tsw1-2nd.aiff"];
        assert_eq!(
            format_lens(&display_paths, &files),
            "sounds/marimba.wav..............1.001678 secs \n\
             sounds/ws2/tsw1-2nd.aiff........5.467166 secs \n"
        );
    }

    #[test]
    fn three_infiles_including_a_stereo_file_match_the_real_legacy_sndinfo_lens_output() {
        let files = open_infiles(&[
            &repo_path("docs/manual/sounds/marimba.wav"),
            &repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff"),
            &repo_path("docs/manual/sounds/clip5-all.wav"),
        ])
        .unwrap();
        let display_paths = [
            "sounds/marimba.wav",
            "sounds/ws2/tsw1-2nd.aiff",
            "sounds/clip5-all.wav",
        ];
        assert_eq!(
            format_lens(&display_paths, &files),
            "sounds/marimba.wav..............1.001678 secs \n\
             sounds/ws2/tsw1-2nd.aiff........5.467166 secs \n\
             sounds/clip5-all.wav............3 mins 31.006259 secs \n"
        );
    }

    #[test]
    fn mismatched_channel_counts_are_allowed_and_each_infile_uses_its_own_duration() {
        // legacy: confirmed live -- see this module's doc on
        // `has_otherfile`.
        let files = open_infiles(&[
            &repo_path("docs/manual/sounds/marimba.wav"),   // mono
            &repo_path("docs/manual/sounds/clip5-all.wav"), // stereo
        ])
        .unwrap();
        let display_paths = ["sounds/marimba.wav", "sounds/clip5-all.wav"];
        assert_eq!(
            format_lens(&display_paths, &files),
            "sounds/marimba.wav..............1.001678 secs \n\
             sounds/clip5-all.wav............3 mins 31.006259 secs \n"
        );
    }

    #[test]
    fn dot_padding_falls_back_to_the_two_dot_minimum_for_a_long_filename() {
        // legacy: `thisspace = max(SPACECNT - strlen(...), 2)` --
        // confirmed by source reading only (every corpus filename is
        // short enough that live runs always land in the ordinary,
        // non-floored branch).
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let long_name = "a".repeat(40);
        let out = format_lens(&[&long_name], std::slice::from_ref(&sf));
        assert_eq!(out, format!("{long_name}..1.001678 secs \n"));
    }

    #[test]
    fn open_infiles_rejects_a_non_sound_first_infile_with_the_generic_wrong_filetype_text() {
        let path1 = repo_path("docs/manual/data/capm.ana");
        let err = open_infiles(&[&path1, "/nonexistent/does-not-exist.wav"]).unwrap_err();
        assert_eq!(err.category, cdp_core::ExitCategory::UsageOnly);
        assert_eq!(err.message, super::super::WRONG_FILETYPE);
    }

    #[test]
    fn open_infiles_reports_the_infile2_specific_open_error_for_a_nonexistent_second_infile() {
        let path1 = repo_path("docs/manual/sounds/marimba.wav");
        let err = open_infiles(&[&path1, "/nonexistent/does-not-exist.wav"]).unwrap_err();
        assert_eq!(err.category, cdp_core::ExitCategory::DataError);
        assert_eq!(
            err.message,
            "cannot open input file /nonexistent/does-not-exist.wav to read data."
        );
    }

    #[test]
    fn open_infiles_reports_the_infile2_specific_wrong_kind_error() {
        let path1 = repo_path("docs/manual/sounds/marimba.wav");
        let path2 = repo_path("docs/manual/data/capm.ana");
        let err = open_infiles(&[&path1, &path2]).unwrap_err();
        assert_eq!(err.category, cdp_core::ExitCategory::DataError);
        assert_eq!(err.message, format!("{path2} is not a sound file."));
    }
}
