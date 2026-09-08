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

//! The real-signal FFT the phase vocoder's per-frame loop needs.
//!
//! `pvoc_process` (`legacy/dev/pv/pvoc.c`) transforms one windowed,
//! real-valued frame of `Nchans` samples (`PVOC_CHANS`) into
//! `Nchans/2 + 1` complex spectral bins (DC to Nyquist) with
//! `fft_(anal, banal, 1, N2, 1, -2)` (`N2 = Nchans/2`) followed by
//! `reals_(anal, banal, N2, -2)`, both in `legacy/dev/pv/mxfft.c`. The
//! pair implements one classic trick: pack the `Nchans` real samples
//! as `N2` complex numbers (`anal[2k], anal[2k+1]` as one complex
//! entry), run one complex FFT of length `N2` on that, then unpack
//! the result into the `N2 + 1` bins a real-input transform actually
//! has. Synthesis (`pvoc_process`'s reconversion block) runs the same
//! pair in reverse order and negated `isn`.
//!
//! Per `docs/migration/PLAN.md`'s crate-responsibilities section,
//! this crate does not port `mxfft.c` itself; it wraps `realfft`
//! (built on `rustfft`), which computes the same mathematical
//! transform (forward: `X[k] = sum_n x[n] * exp(-2*pi*i*k*n/len)` for
//! `k` in `0..=len/2`, unnormalized) as a mixed-radix FFT that is not
//! restricted to the sizes `mxfft.c`'s own factoring handles. [`RealFft::analyze`]
//! and [`RealFft::resynthesize`] below reproduce `fft_`/`reals_`'s two
//! call shapes exactly, including the one difference between them:
//! legacy's *inverse* pair (`reals_` then `fft_`, positive `isn`)
//! already divides its result by `len` before returning, so
//! [`RealFft::resynthesize`] applies that same `1/len` scale itself,
//! where `realfft`'s own inverse does not (see its crate
//! documentation: "RealFFT matches the behaviour of RustFFT and does
//! not normalize the output of either forward or inverse FFT").
//!
//! This was confirmed, not assumed, against `legacy/dev/pv/mxfft.c`
//! itself: `fft_` and `reals_` need only `globcon.h`'s `FINISHED`,
//! `DATA_ERROR` and the `errstr` buffer to link standalone, with no
//! dependency on the rest of `legacy`'s sound-file or option-parsing
//! machinery. A small standalone harness (not part of this crate,
//! see this WP's PR description for the harness source) compiled
//! `mxfft.c` alone and called `fft_`/`reals_` directly on known real
//! signals (an impulse, a DC signal, single sinusoids at a known bin,
//! a ramp, and a round trip on arbitrary data) at `Nchans` 8, 16, 256,
//! 1024 and 4096, confirming: the forward pair's sign convention and
//! scale (a ramp's known closed-form DFT, and a sinusoid's expected
//! `+-len/2` peak, both matched exactly up to float32 rounding); that
//! forward-then-inverse round-trips to the original signal with no
//! extra scale factor needed (ratio exactly `1.0`, not `len` or
//! `1/len`); and that every one of `analyze`'s test fixtures below is
//! the literal legacy output for that input, not a value computed by
//! this module or by `realfft`.

use std::sync::Arc;

use realfft::num_complex::Complex32;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};

/// A forward/inverse real-FFT pair for one fixed frame length,
/// matching `fft_`/`reals_`'s two call shapes in `pvoc_process`.
///
/// `len` is `Nchans` (`PVOC_CHANS`): [`analyze`](Self::analyze) takes
/// `len` real samples and returns `len/2 + 1` complex bins;
/// [`resynthesize`](Self::resynthesize) is its inverse. Legacy
/// requires `Nchans` even (`N2 = Nchans/2` must be a whole number of
/// complex pairs); this wrapper does not re-check that itself, since
/// every caller in `cdp-programs`'s future `pvoc` port derives `len`
/// from an already-validated channel count.
pub struct RealFft {
    len: usize,
    forward: Arc<dyn RealToComplex<f32>>,
    inverse: Arc<dyn ComplexToReal<f32>>,
}

impl RealFft {
    /// Builds the forward/inverse plan pair for `len`-sample frames.
    pub fn new(len: usize) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        Self {
            len,
            forward: planner.plan_fft_forward(len),
            inverse: planner.plan_fft_inverse(len),
        }
    }

    /// The real-signal frame length this plan was built for.
    pub fn frame_len(&self) -> usize {
        self.len
    }

    /// The spectrum length `analyze` returns: `frame_len() / 2 + 1`
    /// complex bins, DC (`spectrum[0]`) to Nyquist
    /// (`spectrum[frame_len() / 2]`).
    pub fn spectrum_len(&self) -> usize {
        self.len / 2 + 1
    }

    /// `fft_(anal, banal, 1, N2, 1, -2)` then `reals_(anal, banal,
    /// N2, -2)`: the forward transform `pvoc_process`'s analysis path
    /// runs on one windowed frame. `samples.len()` must equal
    /// [`frame_len`](Self::frame_len).
    pub fn analyze(&self, samples: &[f32]) -> Vec<Complex32> {
        let mut input = samples.to_vec();
        let mut spectrum = self.forward.make_output_vec();
        self.forward
            .process(&mut input, &mut spectrum)
            .expect("samples.len() must equal RealFft::frame_len()");
        spectrum
    }

    /// `reals_(syn, bsyn, N2, 2)` then `fft_(syn, bsyn, 1, N2, 1, 2)`:
    /// the inverse transform `pvoc_process`'s resynthesis path runs
    /// on one frame's reconverted spectrum, including the `1/len`
    /// scale legacy's inverse pair applies internally (see this
    /// module's doc comment). `spectrum.len()` must equal
    /// [`spectrum_len`](Self::spectrum_len).
    pub fn resynthesize(&self, spectrum: &[Complex32]) -> Vec<f32> {
        let mut input = spectrum.to_vec();
        let mut output = self.inverse.make_output_vec();
        self.inverse
            .process(&mut input, &mut output)
            .expect("spectrum.len() must equal RealFft::spectrum_len()");
        let scale = 1.0 / self.len as f32;
        for sample in &mut output {
            *sample *= scale;
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every expected value below is the literal output of a
    // standalone-compiled `legacy/dev/pv/mxfft.c` (`fft_` and
    // `reals_` called directly, not through the rest of `legacy`),
    // not a value computed by this module or by `realfft`. See the
    // module doc comment for how the harness was built and what it
    // confirmed structurally.

    fn assert_close(label: &str, got: f32, expected: f32) {
        assert!(
            (got - expected).abs() < 1e-4,
            "{label}: got {got}, expected {expected}"
        );
    }

    #[test]
    fn analyze_impulse_is_flat_unity_spectrum() {
        // x[0]=1, rest 0, Nchans=8: every bin is 1+0i.
        let fft = RealFft::new(8);
        let mut samples = [0.0f32; 8];
        samples[0] = 1.0;
        let spectrum = fft.analyze(&samples);
        assert_eq!(spectrum.len(), 5);
        for (k, bin) in spectrum.iter().enumerate() {
            assert_close(&format!("bin {k} re"), bin.re, 1.0);
            assert_close(&format!("bin {k} im"), bin.im, 0.0);
        }
    }

    #[test]
    fn analyze_dc_signal_is_sum_at_bin_zero() {
        // x[n]=1 for all n, Nchans=8: bin 0 is 8, every other bin 0.
        let fft = RealFft::new(8);
        let samples = [1.0f32; 8];
        let spectrum = fft.analyze(&samples);
        assert_close("bin 0 re", spectrum[0].re, 8.0);
        for (k, bin) in spectrum.iter().enumerate().skip(1) {
            assert_close(&format!("bin {k} re"), bin.re, 0.0);
            assert_close(&format!("bin {k} im"), bin.im, 0.0);
        }
    }

    #[test]
    fn analyze_single_cosine_peaks_at_its_bin() {
        // x[n] = cos(2*pi*n/8), Nchans=8: energy concentrates at bin
        // 1 as a purely real len/2 = 4.
        let fft = RealFft::new(8);
        let mut samples = [0.0f32; 8];
        for (n, s) in samples.iter_mut().enumerate() {
            *s = (2.0 * std::f64::consts::PI * n as f64 / 8.0).cos() as f32;
        }
        let spectrum = fft.analyze(&samples);
        assert_close("bin 1 re", spectrum[1].re, 4.0);
        assert_close("bin 1 im", spectrum[1].im, 0.0);
    }

    #[test]
    fn analyze_ramp_matches_legacy_closed_form() {
        // x[n]=n, Nchans=8: bin 0 is the sum (28); bin k>0 is
        // -len/2 + i*(len/2)*cot(pi*k/len), the closed form for a
        // ramp's DFT under legacy's sign convention.
        let fft = RealFft::new(8);
        let mut samples = [0.0f32; 8];
        for (n, s) in samples.iter_mut().enumerate() {
            *s = n as f32;
        }
        let spectrum = fft.analyze(&samples);
        assert_close("bin 0 re", spectrum[0].re, 28.0);
        assert_close("bin 1 re", spectrum[1].re, -4.0);
        assert_close("bin 1 im", spectrum[1].im, 9.656_855);
        assert_close("bin 2 re", spectrum[2].re, -4.0);
        assert_close("bin 2 im", spectrum[2].im, 4.0);
        assert_close("bin 3 re", spectrum[3].re, -4.0);
        assert_close("bin 3 im", spectrum[3].im, 1.656_854_2);
        assert_close("bin 4 re", spectrum[4].re, -4.0);
        assert_close("bin 4 im", spectrum[4].im, 0.0);
    }

    #[test]
    fn analyze_arbitrary_signal_matches_legacy_bin_for_bin() {
        // Nchans=8, arbitrary (non-symmetric, non-trivial) samples.
        let fft = RealFft::new(8);
        let samples = [3.0f32, -1.5, 2.0, 0.5, -2.5, 1.0, 4.0, -0.75];
        let spectrum = fft.analyze(&samples);
        assert_close("bin 0 re", spectrum[0].re, 5.75);
        assert_close("bin 0 im", spectrum[0].im, 0.0);
        assert_close("bin 1 re", spectrum[1].re, 2.848_349_6);
        assert_close("bin 1 im", spectrum[1].im, 2.883_883_5);
        assert_close("bin 2 re", spectrum[2].re, -5.5);
        assert_close("bin 2 im", spectrum[2].im, 0.25);
        assert_close("bin 3 re", spectrum[3].re, 8.151_65);
        assert_close("bin 3 im", spectrum[3].im, -1.116_116_5);
        assert_close("bin 4 re", spectrum[4].re, 7.25);
        assert_close("bin 4 im", spectrum[4].im, 0.0);
    }

    #[test]
    fn round_trip_recovers_the_original_signal_with_no_extra_scale() {
        // Confirms resynthesize's 1/len scale is exactly right: with
        // it, forward-then-inverse recovers the original samples
        // (not the original scaled by len or 1/len), matching legacy
        // fft_/reals_'s own round trip on the same data.
        let fft = RealFft::new(8);
        let samples = [3.0f32, -1.5, 2.0, 0.5, -2.5, 1.0, 4.0, -0.75];
        let spectrum = fft.analyze(&samples);
        let round_tripped = fft.resynthesize(&spectrum);
        for (n, (&got, &want)) in round_tripped.iter().zip(samples.iter()).enumerate() {
            assert_close(&format!("x[{n}]"), got, want);
        }
    }

    #[test]
    fn analyze_matches_legacy_at_nchans_1024_a_real_pvoc_window_size() {
        // x[n] = 0.5*sin(2*pi*3*n/N) + 0.25*cos(2*pi*17*n/N) + 0.1,
        // Nchans=1024 (a window size PLAN.md's WP-1.4 done-when
        // actually names, unlike the toy Nchans=8 cases above).
        let len = 1024;
        let fft = RealFft::new(len);
        let mut samples = vec![0.0f32; len];
        for (n, s) in samples.iter_mut().enumerate() {
            let n = n as f64;
            let v = 0.5 * (2.0 * std::f64::consts::PI * 3.0 * n / len as f64).sin()
                + 0.25 * (2.0 * std::f64::consts::PI * 17.0 * n / len as f64).cos()
                + 0.1;
            *s = v as f32;
        }
        let spectrum = fft.analyze(&samples);
        assert_close("bin 0 re", spectrum[0].re, 102.399_994);
        assert_close("bin 3 re", spectrum[3].re, 0.0);
        assert_close("bin 3 im", spectrum[3].im, -256.0);
        assert_close("bin 17 re", spectrum[17].re, 128.0);
        assert_close("bin 17 im", spectrum[17].im, 0.0);
        assert_close("bin 512 re", spectrum[512].re, 0.0);
        assert_close("bin 512 im", spectrum[512].im, 0.0);

        let round_tripped = fft.resynthesize(&spectrum);
        for (n, (&got, &want)) in round_tripped.iter().zip(samples.iter()).enumerate() {
            assert!(
                (got - want).abs() < 1e-4,
                "x[{n}]: got {got}, expected {want}"
            );
        }
    }

    #[test]
    fn round_trips_at_the_window_size_boundaries_256_and_4096() {
        // PLAN.md's WP-1.4 done-when names window sizes 256 to 4096;
        // confirm the wrapper round-trips cleanly at both boundaries,
        // not just the 1024 case checked bin-for-bin above.
        for len in [256usize, 4096] {
            let fft = RealFft::new(len);
            let mut samples = vec![0.0f32; len];
            for (n, s) in samples.iter_mut().enumerate() {
                let n = n as f64;
                let v = (2.0 * std::f64::consts::PI * 5.0 * n / len as f64).sin()
                    - 0.3 * (2.0 * std::f64::consts::PI * 40.0 * n / len as f64).cos();
                *s = v as f32;
            }
            let spectrum = fft.analyze(&samples);
            assert_eq!(spectrum.len(), len / 2 + 1);
            let round_tripped = fft.resynthesize(&spectrum);
            for (n, (&got, &want)) in round_tripped.iter().zip(samples.iter()).enumerate() {
                assert!(
                    (got - want).abs() < 1e-4,
                    "len={len} x[{n}]: got {got}, expected {want}"
                );
            }
        }
    }
}
