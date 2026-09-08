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

//! `pvoc anal`: analyse a sound file into a phase-vocoder `.ana` file.
//! legacy: `legacy/dev/pv/pvoc.c` (`pvoc_process`'s analysis path,
//! `sndwrite_header`) and `legacy/dev/pv/ap_pvoc.c`
//! (`check_param_validity_and_consistency`, `pvoc_preprocess`), which
//! together resolve the `-c`/`-o` command-line flags into the window
//! length `M`, channel count `N` and decimation factor `D` this
//! module needs before it can call [`cdp_dsp::pvoc::analyze`].
//!
//! Scope of this slice: mode 1 (`STANDARD ANALYSIS`) and mono input
//! only. Modes 2/3 (`OUTPUT SPECTRAL ENVELOPE/MAGNITUDE VALS ONLY`)
//! write a different, smaller frame shape that
//! [`cdp_dsp::pvoc::analyze`] does not produce yet (see its own module
//! doc), and multi-channel input needs a per-channel analysis loop not
//! yet confirmed against `legacy/dev/pv/pvoc.c`. Both report a plain
//! `ProgramError` rather than silently doing the wrong thing.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::{ParamValue, ParsedCommand};
use cdp_sf::{PropertyBlock, SampleType, SoundFile, SoundFileWriter, WriteSpec};

/// legacy: `PVOC_ANAL`'s three modes (`legacy/dev/include/modeno.h`'s
/// `STANDARD_ANAL`/`ENVEL_ONLY`/`MAG_ONLY`, `0`-based internally; the
/// discriminants here are the 1-based numbers a user types).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Standard = 1,
    EnvelopeOnly = 2,
    MagnitudeOnly = 3,
}

/// legacy: `dz->maxmode` for `PVOC_ANAL` (`parstruct.c`'s
/// `case(PVOC_ANAL): maxmode = 3`).
pub const MAX_MODE: u32 = 3;

/// legacy: the usage text `legacy/dev/pv/main.c` prints for `pvoc
/// anal` with no further arguments at all (captured live via
/// `spec/usage/pvoc/anal.txt`, WP-0.2). See `synth::wave::USAGE`'s own
/// doc for why this is a literal fixture, and why a raw string
/// literal, not `"...\` line continuation.
pub const USAGE: &str = r"CDP Release 7.1 2016
CONVERT SOUNDFILE TO SPECTRAL FILE

USAGE: pvoc anal  mode infile outfile [-cpoints] [-ooverlap]

MODES ARE....
1) STANDARD ANALYSIS
2) OUTPUT SPECTRAL ENVELOPE VALS ONLY
3) OUTPUT SPECTRAL MAGNITUDE VALS ONLY
POINTS   No of analysis points (2-32768 (power of 2)): default 1024
         More points give better freq resolution
         but worse time-resolution (e.g. rapidly changing spectrum).
OVERLAP  Filter overlap factor (1-4): default 3
";

/// legacy: `DEFAULT_PVOC_CHANS` (`legacy/dev/include/pvoc.h`) -- `-c`'s
/// value when the flag is absent.
const DEFAULT_CHANS: i64 = 1024;
/// legacy: `DEFAULT_WIN_OVERLAP`, same header -- `-o`'s value when the
/// flag is absent.
const DEFAULT_OVERLAP: i64 = 3;
/// legacy: `PVOC_CONSTANT_A` (`legacy/dev/include/pvoc.h`).
const PVOC_CONSTANT_A: i64 = 8;

impl Mode {
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Mode::Standard),
            2 => Some(Mode::EnvelopeOnly),
            3 => Some(Mode::MagnitudeOnly),
            _ => None,
        }
    }
}

/// Resolves `-c`/`-o` into `(M, N, D)`. legacy:
/// `check_param_validity_and_consistency`'s `PVOC_ANAL` case together
/// with `pvoc_preprocess`'s matching, independently-computed
/// `PVOC_CHANS` even-rounding -- both derive the same `chancnt` from
/// the same input, confirmed by reading both functions side by side.
/// `M/PVOC_CONSTANT_A` in the C source is a `double` division
/// truncated by an `(int)` cast; for the always-non-negative `M` this
/// crate ever computes, plain integer division gives the identical
/// result.
fn resolve_m_n_d(chans_flag: Option<i64>, overlap_flag: Option<i64>) -> (usize, usize, usize) {
    let mut chancnt = chans_flag.unwrap_or(DEFAULT_CHANS);
    chancnt += chancnt % 2; // legacy: "Force PVOC_CHANS to be even"
    let overlap_cli = overlap_flag.unwrap_or(DEFAULT_OVERLAP);
    let win_overlap = overlap_cli - 1; // legacy: `PVOC_WINOVLP_INPUT - 1`
    let m = match win_overlap {
        0 => 4 * chancnt,
        1 => 2 * chancnt,
        2 => chancnt,
        3 => chancnt / 2,
        _ => unreachable!("cdp_params::CommandSpec::pvoc_anal already range-checks -o to 1..=4"),
    };
    let mut d = m / PVOC_CONSTANT_A;
    if d == 0 {
        // legacy: "WARNING: Decimation too low: adjusted."
        d = 1;
    }
    (m as usize, chancnt as usize, d as usize)
}

/// legacy: `dz->infile->stype`, the raw `SAMP_*` code
/// (`legacy/dev/newinclude/sfsys.h`) stored as the `"original
/// sampsize"` property. Only [`SampleType::Short16`] and
/// [`SampleType::Float32`] are reachable here: `cdp_sf::SoundFile`
/// does not decode any other sample type to `f32` yet (see its own
/// module doc), so [`SoundFile::samples_f32`] already fails first for
/// every other case.
fn original_sampsize_code(sample_type: SampleType) -> i32 {
    match sample_type {
        SampleType::Short16 => 0, // legacy: SAMP_SHORT
        SampleType::Float32 => 1, // legacy: SAMP_FLOAT
        _ => unreachable!("SoundFile::samples_f32 already rejects every other sample type"),
    }
}

fn flag_as_int(cmd: &ParsedCommand, letter: char) -> Option<i64> {
    match cmd.flags.get(&letter) {
        Some(ParamValue::Integer(n)) => Some(*n),
        Some(_) => unreachable!("pvoc anal's -c/-o are always Int"),
        None => None,
    }
}

/// legacy: `pvoc_process`'s analysis path, `PVOC_ANAL_ONLY` scope.
/// `cmd` is the already-validated
/// [`cdp_params::CommandSpec::pvoc_anal`] parse.
pub fn analyze(mode: Mode, cmd: &ParsedCommand) -> Result<SoundFileWriter, CdpError> {
    if mode != Mode::Standard {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "pvoc anal: modes 2 and 3 (spectral envelope/magnitude only) are not implemented yet",
        ));
    }

    let sf = SoundFile::open(&cmd.infiles[0])?;
    if sf.fmt.channels != 1 {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            "pvoc anal: multi-channel input is not implemented yet",
        ));
    }
    let samples = sf.samples_f32()?;

    let (m, n, d) = resolve_m_n_d(flag_as_int(cmd, 'c'), flag_as_int(cmd, 'o'));
    let frames = cdp_dsp::pvoc::analyze(&samples, m, n, d, sf.fmt.sample_rate as f32);

    let frame_width = n + 2;
    let mut out = Vec::with_capacity(frames.len() * frame_width);
    for frame in &frames {
        out.extend(frame.to_interleaved());
    }

    // legacy: `sndwrite_header`'s `PVOC_ANAL`/no-envelope-or-magnitude
    // branch: `arate = R/D` (single-precision, matching the C `float`
    // arithmetic exactly, not `f64`), `dz->outfile->srate = (int)
    // (*arate)` (a truncating cast, not a round).
    let arate: f32 = sf.fmt.sample_rate as f32 / d as f32;
    let out_sample_rate = arate as u32;

    let mut properties = PropertyBlock::new();
    properties.set_i32(
        "original sampsize",
        original_sampsize_code(sf.fmt.sample_type),
    );
    properties.set_i32("original sample rate", sf.fmt.sample_rate as i32);
    properties.set_f32("arate", arate);
    properties.set_i32("analwinlen", m as i32);
    properties.set_i32("decfactor", d as i32);

    let mut writer = SoundFileWriter::new(WriteSpec {
        channels: frame_width as u16,
        sample_rate: out_sample_rate,
        sample_type: SampleType::Float32,
        write_peaks: false,
        properties,
        write_cue_chunk: false, // legacy: "don't need cue for analysis files"
    });
    writer.write_frames(&out);
    Ok(writer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_m_n_d_matches_defaults_from_the_usage_text() {
        // legacy: default POINTS 1024, default OVERLAP 3 -- confirmed
        // live: `pvoc anal 1 sine1.wav ana1.ana` (no flags) produced
        // `analwinlen=1024, decfactor=128` (see
        // `cdp_dsp::pvoc::analyze`'s own corpus test, which reuses
        // this exact file pair).
        let (m, n, d) = resolve_m_n_d(None, None);
        assert_eq!((m, n, d), (1024, 1024, 128));
    }

    #[test]
    fn resolve_m_n_d_rounds_an_odd_chans_flag_up_to_even() {
        // legacy: "Force PVOC_CHANS to be even".
        let (_, n, _) = resolve_m_n_d(Some(511), Some(3));
        assert_eq!(n, 512);
    }

    #[test]
    fn resolve_m_n_d_covers_all_four_overlap_factors() {
        // legacy: `check_param_validity_and_consistency`'s switch.
        assert_eq!(resolve_m_n_d(Some(1024), Some(1)).0, 4096);
        assert_eq!(resolve_m_n_d(Some(1024), Some(2)).0, 2048);
        assert_eq!(resolve_m_n_d(Some(1024), Some(3)).0, 1024);
        assert_eq!(resolve_m_n_d(Some(1024), Some(4)).0, 512);
    }

    #[test]
    fn resolve_m_n_d_adjusts_a_zero_decimation_to_one() {
        // legacy: "WARNING: Decimation too low: adjusted." -- the
        // smallest legal chancnt (2) at the smallest M (overlap 4,
        // M = chancnt/2 = 1) gives D = 1/8 = 0 before the adjustment.
        let (_, _, d) = resolve_m_n_d(Some(2), Some(4));
        assert_eq!(d, 1);
    }

    fn repo_path(rel: &str) -> String {
        // This crate lives at `crates/cdp-programs`; the corpus is two
        // levels up, at the repository root.
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    /// End-to-end: `analyze` wired to a real `cdp_sf::SoundFile`, its
    /// output re-parsed by `cdp_sf::SoundFile` and compared against
    /// `docs/manual/data/capm.ana`, a real file `legacy` itself
    /// produced from `docs/manual/sounds/capm.wav` with default `-c`/
    /// `-o` (`M=1024, D=128`, confirmed by
    /// `cdp_dsp::pvoc::analyze`'s own corpus test, which this test's
    /// header/property checks complement rather than repeat -- that
    /// test already cross-checks bin values at three representative
    /// frames against an independent Python oracle; this one confirms
    /// the *wiring* around it: `-c`/`-o` resolution, the written
    /// header properties, and that the encoded file byte layout
    /// itself round-trips through `cdp_sf::SoundFile`. Also confirmed
    /// live, byte for byte, against `cdp8-postmerge`'s own `pvoc anal`
    /// output for this exact file (amplitude bit-exact; frequency
    /// exact to float32 noise for every bin above the noise floor --
    /// see this module's PR description for the comparison script).
    #[test]
    fn analyze_end_to_end_matches_the_real_capm_ana_corpus_file() {
        let cmd = ParsedCommand {
            infiles: vec![repo_path("docs/manual/sounds/capm.wav")],
            outfile: "capm.ana".to_string(),
            params: vec![],
            flags: std::collections::BTreeMap::new(),
        };
        let writer = analyze(Mode::Standard, &cmd).unwrap();
        let bytes = writer.encode().unwrap();
        let produced = SoundFile::from_reader(std::io::Cursor::new(bytes)).unwrap();
        let reference = SoundFile::open(repo_path("docs/manual/data/capm.ana")).unwrap();

        assert_eq!(produced.fmt.channels, reference.fmt.channels);
        assert_eq!(produced.fmt.channels, 1026);
        assert_eq!(produced.fmt.sample_rate, reference.fmt.sample_rate);
        assert_eq!(produced.frame_count(), reference.frame_count());
        assert_eq!(produced.frame_count(), 2828);

        for name in [
            "original sampsize",
            "original sample rate",
            "analwinlen",
            "decfactor",
        ] {
            assert_eq!(
                produced.properties.get_i32(name).unwrap(),
                reference.properties.get_i32(name).unwrap(),
                "property {name} mismatch"
            );
        }
        assert_eq!(
            produced.properties.get_f32("arate").unwrap(),
            reference.properties.get_f32("arate").unwrap()
        );

        let produced_samples = produced.samples_f32().unwrap();
        let reference_samples = reference.samples_f32().unwrap();
        assert_eq!(produced_samples.len(), reference_samples.len());
        // legacy: amplitude is bit-exact everywhere, and frequency is
        // exact to float32 noise at any bin with non-negligible
        // amplitude -- confirmed for the *entire* file live against
        // `cdp8-postmerge` (see this test's doc). A bin's reported
        // frequency below that noise floor is inherently unstable
        // (phase is meaningless when magnitude is ~0), and does
        // genuinely diverge between this crate's FFT (`realfft`) and
        // legacy's own (`mxfft.c`) there -- an already-documented
        // characteristic (`cdp_dsp::fft`'s module doc), not a wiring
        // bug, so this in-repo check (no Docker at test time) only
        // asserts amplitude, plus frequency where amplitude clears
        // the noise floor.
        let channels = produced.fmt.channels as usize;
        for frame in produced_samples
            .chunks_exact(channels)
            .zip(reference_samples.chunks_exact(channels))
        {
            let (produced_frame, reference_frame) = frame;
            for bin in 0..channels / 2 {
                let amp_p = produced_frame[bin * 2];
                let amp_r = reference_frame[bin * 2];
                assert!(
                    (amp_p - amp_r).abs() < 1e-6,
                    "bin {bin} amplitude: produced {amp_p} vs reference {amp_r}"
                );
                if amp_p > 0.02 {
                    let freq_p = produced_frame[bin * 2 + 1];
                    let freq_r = reference_frame[bin * 2 + 1];
                    assert!(
                        (freq_p - freq_r).abs() < 0.01,
                        "bin {bin} frequency (amp {amp_p}): produced {freq_p} vs reference {freq_r}"
                    );
                }
            }
        }
    }
}
