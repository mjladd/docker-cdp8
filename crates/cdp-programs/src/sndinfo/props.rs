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

//! `sndinfo props infile`: prints a sound or analysis file's
//! properties. legacy: `legacy/dev/sndinfo/compare.c`'s `case
//! (INFO_PROPS)` block and its `prntprops` (the `PEAK`-chunk and
//! `DATE`-property tail every non-text-file case shares).
//!
//! Scope of this slice: [`cdp_sf::FileKind::Wave`],
//! [`cdp_sf::FileKind::Analysis`] and [`cdp_sf::FileKind::Envelope`]
//! only -- the three kinds this repository's corpus (`docs/manual`)
//! actually has an example of, and the three
//! `crates/cdp-sf/tests/oracle_fixtures.rs` already independently
//! confirms field values for. `Pitch`/`Transposition`/`Formant` report
//! a plain `ProgramError`: no real file of any of those three kinds
//! exists anywhere in this repository's corpus (see
//! `docs/migration/STATUS.md`'s WP-1.1 notes), so their branch below
//! would be confirmed against a hand-built fixture only, not a live
//! run. Text-file/breakpoint-file/number-list detection (`legacy`'s
//! `is_a_textfile_type` branch) is also out of scope: `cdp-sf`/
//! `cdp-data` do not classify a file that way at all yet.

use crate::sndinfo::ctime;
use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SoundFile};

/// legacy: `legacy/dev/sndinfo/main.c`'s `make_initial_cmdline_check`
/// (`legacy/dev/cdp2k/mainfuncs.c`), which prints this unconditionally
/// whenever raw `argc<4` -- true for every `sndinfo props` invocation
/// this module handles (`sndinfo`, `props`, at most one further
/// token), regardless of whether that invocation goes on to succeed.
/// Confirmed live: `sndinfo props infile` (argc 3) prints it before
/// real output; `sndinfo props infile extra` (argc 4) does not print
/// it at all, going straight to `"Too many parameters on command
/// line."` with no greeting.
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// legacy: the usage text `legacy/dev/sndinfo/main.c` prints for a
/// bare `sndinfo props` (captured live via `spec/usage/sndinfo/
/// props.txt`, WP-0.2). Bundles [`GREETING`] at the front, the same
/// self-contained-fixture style `synth::wave::USAGE`/`pvoc::anal::USAGE`
/// use, since this is the one case where the greeting is followed
/// immediately by an error rather than real output.
pub const USAGE: &str = "CDP Release 7.1 2016
DISPLAY PROPERTIES OF A SNDFILING-SYSTEM FILE

USAGE: sndreport props infile
";

/// legacy: `prntprops`'s `sndreadpeaks`/`ctime` block, gated on
/// `props.type==wt_wave` -- i.e. only for [`FileKind::Wave`], never
/// for the analysis-family kinds (confirmed live: `capm.ana`'s real
/// `sndinfo props` output has no `sample type:`/`PEAK` lines at all,
/// only the `Date:` line below, which is not gated the same way).
fn format_peak_block(sf: &SoundFile) -> String {
    let mut out = String::new();
    out.push_str("sample type:  ");
    out.push_str(sf.fmt.sample_type.describe());
    out.push('\n');
    if sf.peaks.is_empty() {
        // legacy: `print_outmessage("No PEAK chunk in this file")` --
        // no trailing newline at all, confirmed live (the very next
        // line, `Date: ...`, starts on the same output line).
        out.push_str("No PEAK chunk in this file");
    } else {
        let timestamp = sf.peak_timestamp.unwrap_or(0) as i32;
        out.push_str("PEAK data at: ");
        out.push_str(&ctime::format(timestamp));
        out.push('\n');
        for (i, peak) in sf.peaks.iter().enumerate() {
            let db = 20.0 * peak.value.log10();
            out.push_str(&format!(
                "CH {}:\tamp = {:.4} ({:.2} dB)\tFrame {}\n",
                i + 1,
                peak.value,
                db,
                peak.position
            ));
        }
    }
    out
}

/// legacy: `prntprops`'s `DATE`/`date` property check, unconditional
/// (unlike the PEAK block above, this runs for every file kind
/// [`format_props`] calls it for).
fn format_date_line(sf: &SoundFile) -> Option<String> {
    let timestamp = sf
        .properties
        .get_i32("date")
        .or_else(|_| sf.properties.get_i32("DATE"))
        .ok()?;
    Some(format!("Date: {}\n", ctime::format(timestamp)))
}

/// legacy: `case(INFO_PROPS)` in `compare.c`, the non-text-file
/// branch. Returns the complete properties text, not including
/// [`GREETING`] (the caller prints that separately -- see this
/// module's doc).
pub fn format_props(sf: &SoundFile) -> Result<String, CdpError> {
    let mut out = String::new();
    let samples = sf.sample_count();

    match sf.file_kind {
        FileKind::Wave => {
            out.push_str("A SOUND file.\n");
            out.push_str(&format!("samples: ............ {samples}\n"));
            out.push_str("file type: ........... SOUND\n");
            out.push_str(&format!("sample rate: ........ {}\n", sf.fmt.sample_rate));
            out.push_str(&format!("channels: ........... {}\n", sf.fmt.channels));
            out.push_str(&format_peak_block(sf));
            if let Some(date) = format_date_line(sf) {
                out.push_str(&date);
            }
        }
        FileKind::Analysis(spectral) => {
            out.push_str("An ANALYSIS file.\n");
            out.push_str(&format!("samples: ............ {samples}\n"));
            out.push_str("file type: .......... ANALYSIS DATA\n");
            out.push_str(&format!("sample rate: ........ {}\n", sf.fmt.sample_rate));
            out.push_str(&format!("channels: ........... {}\n", sf.fmt.channels));
            out.push_str(&format!(
                "original sampsize: .. {}\n",
                spectral.original_sample_size
            ));
            out.push_str(&format!(
                "original sample rate: {}\n",
                spectral.original_sample_rate
            ));
            out.push_str(&format!(
                "analysis rate: ...... {:.6}\n",
                spectral.analysis_rate
            ));
            out.push_str(&format!(
                "analysis window len:  {}\n",
                spectral.analysis_window_length
            ));
            out.push_str(&format!(
                "decimation factor: .. {}\n",
                spectral.decimation_factor
            ));
            if let Some(date) = format_date_line(sf) {
                out.push_str(&date);
            }
        }
        FileKind::Envelope { window_size } => {
            out.push_str("A binary ENVELOPE file.\n");
            out.push_str(&format!("samples: ............ {samples}\n"));
            out.push_str("file type: .......... ENVELOPE DATA (binary)\n");
            out.push_str(&format!("sample rate: ........ {}\n", sf.fmt.sample_rate));
            out.push_str(&format!("channels: ........... {}\n", sf.fmt.channels));
            out.push_str(&format!("window size: ........ {window_size:.6} ms\n"));
            // legacy: `return(FINISHED)` right after the window-size
            // line -- no PEAK/DATE tail at all for envelope files,
            // confirmed live.
        }
        FileKind::Pitch(_) | FileKind::Transposition(_) | FileKind::Formant { .. } => {
            return Err(CdpError::new(
                ExitCategory::ProgramError,
                "sndinfo props: pitch/transposition/formant files are not implemented yet",
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
    fn marimba_wav_matches_the_real_legacy_sndinfo_props_output() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/marimba.wav")).unwrap();
        let out = format_props(&sf).unwrap();
        assert_eq!(
            out,
            "A SOUND file.\n\
             samples: ............ 44174\n\
             file type: ........... SOUND\n\
             sample rate: ........ 44100\n\
             channels: ........... 1\n\
             sample type:  16bit\n\
             No PEAK chunk in this fileDate: Wed Jul 19 08:50:04 2000\n\n"
        );
    }

    #[test]
    fn tsw1_2nd_aiff_matches_the_real_legacy_sndinfo_props_output() {
        let sf = SoundFile::open(repo_path("docs/manual/sounds/ws2/tsw1-2nd.aiff")).unwrap();
        let out = format_props(&sf).unwrap();
        assert_eq!(
            out,
            "A SOUND file.\n\
             samples: ............ 241102\n\
             file type: ........... SOUND\n\
             sample rate: ........ 44100\n\
             channels: ........... 1\n\
             sample type:  16bit\n\
             PEAK data at: Sun Jun 25 20:48:42 2006\n\n\
             CH 1:\tamp = 0.2997 (-10.47 dB)\tFrame 29914\n\
             Date: Sun Jun 25 20:48:42 2006\n\n"
        );
    }

    #[test]
    fn capm_ana_matches_the_real_legacy_sndinfo_props_output() {
        let sf = SoundFile::open(repo_path("docs/manual/data/capm.ana")).unwrap();
        let out = format_props(&sf).unwrap();
        assert_eq!(
            out,
            "An ANALYSIS file.\n\
             samples: ............ 2901528\n\
             file type: .......... ANALYSIS DATA\n\
             sample rate: ........ 344\n\
             channels: ........... 1026\n\
             original sampsize: .. 0\n\
             original sample rate: 44100\n\
             analysis rate: ...... 344.531250\n\
             analysis window len:  1024\n\
             decimation factor: .. 128\n\
             Date: Tue Jan  4 17:26:36 2022\n\n"
        );
    }

    #[test]
    fn crklenv_evl_matches_the_real_legacy_sndinfo_props_output() {
        let sf = SoundFile::open(repo_path("docs/manual/data/crklenv.evl")).unwrap();
        let out = format_props(&sf).unwrap();
        assert_eq!(
            out,
            "A binary ENVELOPE file.\n\
             samples: ............ 2845\n\
             file type: .......... ENVELOPE DATA (binary)\n\
             sample rate: ........ 344\n\
             channels: ........... 1\n\
             window size: ........ 2.902494 ms\n"
        );
    }
}
