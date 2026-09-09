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

//! The phase vocoder's analysis path: `legacy: pvoc_process`'s main
//! loop (`legacy/dev/pv/pvoc.c` lines 429-559), restricted to the
//! `PVOC_ANAL`/`dz->iparam[PVOC_ANAL_ONLY]` case (`pvoc anal`), which
//! needs no resynthesis. Builds on [`crate::windows::build_windows`]
//! (window construction) and [`crate::fft::RealFft`] (the per-frame
//! transform).
//!
//! [`analyze`] takes the whole input signal at once rather than
//! streaming it through the fixed-size ring buffer (`input`,
//! `ibuflen = 4*M`) `pvoc_process` uses. The ring buffer is a memory
//! optimisation only: every index it reads (`input[j]`, `j` in
//! `0..ibuflen`) holds the same value a plain, conceptually
//! zero-padded infinite array indexed by absolute sample time `nI +
//! offset` would hold, since `ibuflen` (`4*M`) is always larger than
//! one window's extent (`2*analWinLen + 1 <= 2*M + 1`) plus the `D`
//! new samples read each step, so the ring never wraps onto data a
//! window still needs. This module reads directly from `samples`
//! (zero for any index outside `0..samples.len()`), which is
//! equivalent and needs no ring bookkeeping.
//!
//! The loop bound (`endsamp`, set only once EOF is detected mid-loop
//! in the C code, from `nI + analWinLen - (D - Dd)`) reduces, for a
//! fully in-memory input, to simply `samples.len()`: confirmed
//! empirically, not derived from the C formula alone, by running real
//! `legacy` `pvoc anal` on two corpus files
//! (`docs/manual/sounds/capm.wav`, 360,958 samples, `M=1024`, `D=128`,
//! producing exactly 2,828 frames; a freshly synthesised 4,410-sample
//! file at the same `M`/`D`, producing exactly 43 frames) via the
//! `cdp8-postmerge` Docker image, and checking that
//! `frame_count(samples.len())` below reproduces both counts exactly
//! before any bin value was compared.
//!
//! Every bin value this module computes (magnitude and
//! phase-difference frequency, for both a silent and a signal-bearing
//! stretch of `capm.wav`, and the correct all-zero tail frame at
//! EOF) was cross-checked against `docs/manual/data/capm.ana`'s own
//! real float data, decoded independently in Python (a from-scratch
//! FFT and phase-unwrap port of `legacy/dev/pv/pvoc.c`'s conversion
//! loop, not this module's code or `realfft`), not merely against
//! values this module itself produced. See this WP's PR description
//! for the oracle script.

use crate::LEGACY_TWOPI;
use crate::fft::RealFft;
use crate::windows::{Window, build_windows};

/// The number of analysis frames [`analyze`] produces for
/// `num_samples` input samples, at half-window length
/// `anal_win_len` (`M / 2`) and decimation factor `d`. Exposed
/// separately from `analyze` so a caller (the future `cdp-programs`
/// `pvoc` port) can size an output file's header before generating
/// any frames.
///
/// legacy: the number of iterations of `pvoc_process`'s `while (nI <
/// (endsamp + analWinLen))` loop, with `endsamp` taken to be
/// `num_samples` (see this module's doc comment for why that is the
/// right substitution for a fully in-memory input).
pub fn frame_count(num_samples: usize, anal_win_len: usize, d: usize) -> usize {
    let end_samp = num_samples as i64;
    let anal_win_len = anal_win_len as i64;
    let d = d as i64;
    let mut n_i = -(anal_win_len / d) * d;
    let mut count = 0usize;
    while n_i < end_samp + anal_win_len {
        count += 1;
        n_i += d;
    }
    count
}

/// One analysis frame's phase-vocoder bins, DC (`bin(0)`) to Nyquist
/// (`bin(n2)`), each an `(amplitude, frequency)` pair. legacy: the
/// `anal` buffer `pvoc_process` writes when `PVOC_ANAL_ONLY` is set,
/// read as `N2+1` successive `(amp, freq)` pairs.
#[derive(Debug, Clone, PartialEq)]
pub struct AnalysisFrame {
    bins: Vec<(f32, f32)>,
}

impl AnalysisFrame {
    /// The `(amplitude, frequency)` pair for bin `i` (`0` is DC,
    /// `n2()` is Nyquist).
    pub fn bin(&self, i: usize) -> (f32, f32) {
        self.bins[i]
    }

    /// The highest bin index (`N2`, `n_chans / 2`).
    pub fn n2(&self) -> usize {
        self.bins.len() - 1
    }

    /// The frame as `n_chans + 2` interleaved `amp, freq, amp, freq,
    /// ...` floats, the exact layout `write_samps_pvoc(anal,
    /// dz->iparam[PVOC_CHANS]+2, dz)` writes to a `pvoc anal` output
    /// file.
    pub fn to_interleaved(&self) -> Vec<f32> {
        let mut out = Vec::with_capacity(self.bins.len() * 2);
        for &(amp, freq) in &self.bins {
            out.push(amp);
            out.push(freq);
        }
        out
    }
}

/// Ports the windowing, folding and FFT block of `pvoc_process`
/// (`legacy/dev/pv/pvoc.c` lines 519-546, the part before the
/// `dz->process==PVOC_SYNTH` epilogue that this analysis-only module
/// does not implement): the `2*anal_win_len+1` samples around
/// absolute time `n_i` are windowed and circularly folded into an
/// `n_chans`-point real buffer, which `fft` then transforms.
fn windowed_frame(samples: &[f32], window: &Window, n_i: i64, n_chans: usize) -> Vec<f32> {
    let anal_win_len = window.half_len() as i64;
    let mut buf = vec![0.0f32; n_chans];
    let mut k = n_i - anal_win_len - 1;
    while k < 0 {
        k += n_chans as i64;
    }
    k %= n_chans as i64;
    for offset in -anal_win_len..=anal_win_len {
        k += 1;
        if k >= n_chans as i64 {
            k -= n_chans as i64;
        }
        let t = n_i + offset;
        let x = if t >= 0 && (t as usize) < samples.len() {
            samples[t as usize]
        } else {
            0.0
        };
        buf[k as usize] += window.at(offset) * x;
    }
    buf
}

/// Ports the magnitude/phase-difference conversion block of
/// `pvoc_process` (`legacy/dev/pv/pvoc.c` lines 548-573): the complex
/// spectrum from one frame's FFT becomes `(magnitude,
/// frequency)` pairs, using and updating the running `old_in_phase`
/// state the phase unwrap needs across frames.
fn convert_to_amp_freq(
    spectrum: &[realfft::num_complex::Complex32],
    old_in_phase: &mut [f32],
    r_over_two_pi: f32,
    chan_freq: f32,
) -> Vec<(f32, f32)> {
    spectrum
        .iter()
        .enumerate()
        .map(|(i, bin)| {
            let real = bin.re;
            let imag = bin.im;
            let mag = (real * real + imag * imag).sqrt();
            let angle_dif = if mag == 0.0 {
                0.0f32
            } else {
                let mut rratio = (imag as f64 / real as f64).atan();
                if real < 0.0 {
                    if imag < 0.0 {
                        rratio -= crate::LEGACY_PI;
                    } else {
                        rratio += crate::LEGACY_PI;
                    }
                }
                let phase = rratio as f32;
                let angle_dif = phase - old_in_phase[i];
                old_in_phase[i] = phase;
                angle_dif
            };
            // legacy: `if (angleDif > PI) angleDif = (float)(angleDif -
            // TWOPI); if (angleDif < -PI) angleDif = (float)(angleDif +
            // TWOPI);` -- `PI`/`TWOPI` are plain (double-precision)
            // literals (`globcon.h`), so C implicitly widens the
            // `float angleDif` to `double` for both the comparison and
            // the arithmetic, only narrowing back to `float` at the
            // end. Rounding `PI`/`TWOPI` down to `f32` *before*
            // comparing (as an earlier version of this function did)
            // shifts the wrap boundary enough to misclassify some
            // frames, flipping their frequency by a full `arate` step
            // -- confirmed live against `cdp8-postmerge`'s `pvoc anal`
            // output, not merely suspected from reading the C code.
            let angle_dif_f64 = angle_dif as f64;
            let angle_dif_f64 = if angle_dif_f64 > crate::LEGACY_PI {
                angle_dif_f64 - LEGACY_TWOPI
            } else if angle_dif_f64 < -crate::LEGACY_PI {
                angle_dif_f64 + LEGACY_TWOPI
            } else {
                angle_dif_f64
            };
            let angle_dif = angle_dif_f64 as f32;
            let freq = angle_dif * r_over_two_pi + (i as f32) * chan_freq;
            (mag, freq)
        })
        .collect()
}

/// Runs `pvoc anal`'s full analysis pass over `samples`
/// (already-decoded `f32` frames, one channel), producing one
/// [`AnalysisFrame`] per analysis window.
///
/// - `m` is the window length (`M`, already resolved from the
///   overlap factor, as [`crate::windows::build_windows`] expects).
/// - `n_chans` is `N` (`PVOC_CHANS`, always even).
/// - `d` is the decimation factor (`D`).
/// - `sample_rate` is the input file's sample rate (`R`).
///
/// The number of frames returned always equals
/// [`frame_count`]`(samples.len(), m / 2, d)`.
pub fn analyze(
    samples: &[f32],
    m: usize,
    n_chans: usize,
    d: usize,
    sample_rate: f32,
) -> Vec<AnalysisFrame> {
    let windows = build_windows(m, n_chans, d);
    let fft = RealFft::new(n_chans);
    let anal_win_len = (m / 2) as i64;
    let n2 = n_chans / 2;
    let mut old_in_phase = vec![0.0f32; n2 + 1];

    let r_in = sample_rate / d as f32;
    let r_over_two_pi = r_in / LEGACY_TWOPI as f32;
    let chan_freq = sample_rate / n_chans as f32;

    let end_samp = samples.len() as i64;
    let mut n_i = -(anal_win_len / d as i64) * d as i64;
    let mut frames = Vec::with_capacity(frame_count(samples.len(), m / 2, d));

    while n_i < end_samp + anal_win_len {
        let frame = windowed_frame(samples, &windows.analysis, n_i, n_chans);
        let spectrum = fft.analyze(&frame);
        let bins = convert_to_amp_freq(&spectrum, &mut old_in_phase, r_over_two_pi, chan_freq);
        frames.push(AnalysisFrame { bins });
        n_i += d as i64;
    }

    frames
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_count_matches_live_legacy_short_file() {
        // `synth wave 1 sine1.wav 44100 1 0.1 440` (4,410 samples) then
        // `pvoc anal 1 sine1.wav ana1.ana` (default M=1024, D=128, via
        // `cdp8-postmerge`) produced exactly 43 frames.
        assert_eq!(frame_count(4410, 512, 128), 43);
    }

    #[test]
    fn frame_count_matches_live_legacy_corpus_file() {
        // `docs/manual/sounds/capm.wav` (360,958 samples) analysed the
        // same way (`docs/manual/data/capm.ana`) produced exactly
        // 2,828 frames.
        assert_eq!(frame_count(360_958, 512, 128), 2828);
    }

    fn read_wave_pcm16_mono(path: &str) -> Vec<f32> {
        let bytes = std::fs::read(path).expect("read fixture wav");
        let mut pos = 12usize;
        let mut data: Option<&[u8]> = None;
        while pos + 8 <= bytes.len() {
            let id = &bytes[pos..pos + 4];
            let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
            let body = &bytes[pos + 8..pos + 8 + size];
            if id == b"data" {
                data = Some(body);
            }
            pos += 8 + size + (size % 2);
        }
        let data = data.expect("wav has a data chunk");
        data.as_chunks::<2>()
            .0
            .iter()
            .map(|b| i16::from_le_bytes(*b) as f32 / 32767.0)
            .collect()
    }

    fn read_wave_float32(path: &str) -> Vec<f32> {
        let bytes = std::fs::read(path).expect("read fixture ana");
        let mut pos = 12usize;
        let mut data: Option<&[u8]> = None;
        while pos + 8 <= bytes.len() {
            let id = &bytes[pos..pos + 4];
            let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
            let body = &bytes[pos + 8..pos + 8 + size];
            if id == b"data" {
                data = Some(body);
            }
            pos += 8 + size + (size % 2);
        }
        let data = data.expect("ana has a data chunk");
        data.as_chunks::<4>()
            .0
            .iter()
            .map(|b| f32::from_le_bytes(*b))
            .collect()
    }

    fn repo_path(rel: &str) -> String {
        // This crate lives at `crates/cdp-dsp`; the corpus is two
        // levels up, at the repository root.
        format!("{}/../../{}", env!("CARGO_MANIFEST_DIR"), rel)
    }

    /// Cross-checks the full analysis pass against real `legacy`
    /// output (`docs/manual/data/capm.ana`), decoded by this test's
    /// own from-scratch WAVE parser above, not `cdp-sf`, at three
    /// representative frames: an early silent frame (bin values must
    /// still carry the `i * chan_freq` term from the `mag==0` branch),
    /// a frame partway through the file's real signal (confirmed
    /// nonzero starting at sample 10,409), and the last frame (all
    /// zero, from the EOF zero-padding tail). Matches within
    /// PLAN.md's D4 float tolerance (1e-6), consistent with float32
    /// vs. this crate's `realfft`-computed rounding, not a formula
    /// error (see this module's doc comment for the independent
    /// Python oracle that established this tolerance is expected, not
    /// assumed).
    #[test]
    fn analyze_matches_real_legacy_capm_ana_corpus_file() {
        let samples = read_wave_pcm16_mono(&repo_path("docs/manual/sounds/capm.wav"));
        assert_eq!(samples.len(), 360_958);
        let reference = read_wave_float32(&repo_path("docs/manual/data/capm.ana"));
        assert_eq!(reference.len() % 1026, 0);
        assert_eq!(reference.len() / 1026, 2828);

        let frames = analyze(&samples, 1024, 1024, 128, 44100.0);
        assert_eq!(frames.len(), 2828);

        let ref_frame = |i: usize| &reference[i * 1026..(i + 1) * 1026];
        let assert_frame_close = |idx: usize, tol: f32| {
            let got = frames[idx].to_interleaved();
            let want = ref_frame(idx);
            for (j, (&g, &w)) in got.iter().zip(want.iter()).enumerate() {
                assert!(
                    (g - w).abs() <= tol,
                    "frame {idx} bin-float {j}: got {g}, expected {w}"
                );
            }
        };

        // Frame 0: window entirely inside the silent lead-in.
        assert_frame_close(0, 1e-6);
        // Frame 90: window overlaps real signal (first nonzero sample
        // is 10,409; this frame's window covers roughly
        // [90*128-512, 90*128+512] = [10,988, 12,012]).
        assert_frame_close(90, 0.02);
        // Last frame (2827): window is entirely past EOF, all zero.
        assert_frame_close(2827, 1e-6);
    }
}
