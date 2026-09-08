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

//! Phase vocoder analysis and synthesis window construction.
//!
//! Ported from the window-setup block of `pvoc_process`
//! (`legacy/dev/pv/pvoc.c` lines 53-73 and 272-358), which runs once
//! per `pvoc`/`pvoc anal`/`pvoc synth`/`pvoc extract` invocation,
//! before the per-frame analysis/synthesis loop this crate does not
//! implement yet. Takes the already-resolved window length `M`,
//! channel count `N` (`PVOC_CHANS`, always even, see
//! `pvoc_preprocess`) and interpolation factor `I` as plain
//! parameters; computing `M`, `N` and `I` from a command line's
//! `-c`/`-o` flags belongs to the future program port that calls this
//! module (`cdp-programs`'s `pvoc` module, not yet written).
//!
//! All arithmetic mirrors the C code's exact mix of `float` and
//! `double` operations, including which multiplications happen
//! before a value is promoted to `double` for a trig call, since that
//! affects the last bit or two of every coefficient. [`LEGACY_PI`]
//! (not [`std::f64::consts::PI`]) is used throughout for the same
//! reason.
//!
//! A genuine legacy quirk is ported as observed rather than smoothed
//! over: when `M > N` (window overlap factors 0 and 1), the synthesis
//! window's final scale factor is `1.0 / anal_scale`, reusing the
//! *analysis* window's normalisation factor left over in pvoc.c's
//! `sum` variable, not a fresh sum computed from the synthesis window
//! itself (contrast the `M <= N` branch, which does compute a fresh
//! sum). See [`build_windows`]'s doc comment for the exact C lines.

use crate::LEGACY_PI;

/// A symmetric window of `2 * half_len + 1` coefficients, indexed by
/// signed offset from its center (`-half_len..=half_len`), matching
/// how `pvoc_process` addresses `analWindow`/`synWindow` via a
/// pointer advanced to the window's midpoint.
#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    half_len: usize,
    coeffs: Vec<f32>,
}

impl Window {
    /// `analWinLen`/`synWinLen` in `pvoc_process`: half the window's
    /// true length, rounded down (`M / 2`).
    pub fn half_len(&self) -> usize {
        self.half_len
    }

    /// The coefficient at signed `offset` from the window's center.
    /// Panics if `offset` is outside `-half_len..=half_len`.
    pub fn at(&self, offset: i64) -> f32 {
        self.coeffs[(offset + self.half_len as i64) as usize]
    }

    /// The full window, left edge (`-half_len`) first.
    pub fn as_slice(&self) -> &[f32] {
        &self.coeffs
    }
}

/// The analysis and synthesis windows `pvoc_process` builds once per
/// run, from [`build_windows`].
#[derive(Debug, Clone, PartialEq)]
pub struct WindowPair {
    pub analysis: Window,
    pub synthesis: Window,
}

/// Ports `hamming()` (`legacy/dev/pv/pvoc.c` lines 53-73).
///
/// Returns `win_len + 1` coefficients, `result[i]` holding the value
/// at offset `i` from the window's center for `i` in `0..=win_len`.
/// `even` is `pvoc_process`'s `Mf` flag (`1 - M % 2`): true when the
/// full window length `M` is even.
///
/// When `even`, `result[0..win_len]` follow the raised-cosine formula
/// and `result[win_len]` is forced to exactly `0.0` (the C code sets
/// it directly, sidestepping the formula's `i + 0.5` offset landing
/// exactly on `M`'s edge). When not `even`, `result[0]` is forced to
/// exactly `1.0` (the window's true center sample) and
/// `result[1..=win_len]` follow the formula.
pub fn hamming(win_len: usize, even: bool) -> Vec<f32> {
    let pi = LEGACY_PI as f32;
    let ftmp = pi / win_len as f32;
    let mut win = vec![0.0f32; win_len + 1];
    if even {
        for (i, w) in win.iter_mut().enumerate().take(win_len) {
            // `ftmp*((float)i+.5)`: the `.5` double literal promotes
            // the whole product to double arithmetic before `cos`.
            let arg = ftmp as f64 * (i as f64 + 0.5);
            *w = (0.54_f64 + 0.46_f64 * arg.cos()) as f32;
        }
        win[win_len] = 0.0;
    } else {
        win[0] = 1.0;
        for (i, w) in win.iter_mut().enumerate().skip(1) {
            // `ftmp*(float)i`: both operands float, multiplied in
            // float32 before the explicit cast to double for `cos`.
            let prod = ftmp * i as f32;
            *w = (0.54_f64 + 0.46_f64 * (prod as f64).cos()) as f32;
        }
    }
    win
}

fn make_symmetric(half: &[f32], half_len: usize, even: bool) -> Vec<f32> {
    let mut coeffs = vec![0.0f32; 2 * half_len + 1];
    coeffs[half_len..].copy_from_slice(half);
    mirror(&mut coeffs, half_len, even);
    coeffs
}

/// `*(win - i) = *(win + i - Mf)` for `i` in `1..=half_len`.
fn mirror(coeffs: &mut [f32], half_len: usize, even: bool) {
    let mf = if even { 1 } else { 0 };
    for i in 1..=half_len as i64 {
        let src = (half_len as i64 + i - mf) as usize;
        coeffs[(half_len as i64 - i) as usize] = coeffs[src];
    }
}

/// The `if (M > chans)`/`if (M <= chans)` sinc-taper block shared,
/// with a different divisor, by the analysis window (divisor `N`,
/// `legacy/dev/pv/pvoc.c` lines 295-304) and the synthesis window's
/// `M > N` branch (divisor `IO`, lines 346-352).
fn apply_sinc_taper(coeffs: &mut [f32], half_len: usize, even: bool, denom: f64) {
    let half_mf = if even { 0.5 } else { 0.0 };
    if even {
        // `(double)PI*.5/denom` parses as `(PI * .5) / denom`.
        let x = (LEGACY_PI * 0.5) / denom;
        let factor = ((denom * x.sin()) / (LEGACY_PI * 0.5)) as f32;
        coeffs[half_len] *= factor;
    }
    for i in 1..=half_len {
        let x = (LEGACY_PI * (i as f64 + half_mf)) / denom;
        let factor = ((denom * x.sin()) / (LEGACY_PI * (i as f64 + half_mf))) as f32;
        coeffs[half_len + i] *= factor;
    }
    mirror(coeffs, half_len, even);
}

fn sum_all(coeffs: &[f32]) -> f32 {
    coeffs.iter().sum()
}

fn scale_all(coeffs: &mut [f32], scale: f32) {
    for v in coeffs.iter_mut() {
        *v *= scale;
    }
}

/// `for (i = -half_len; i <= half_len; i += stride) sum += w[i]*w[i]`
/// (`legacy/dev/pv/pvoc.c` lines 334-335).
fn sum_sq_strided(coeffs: &[f32], half_len: usize, stride: usize) -> f32 {
    let mut sum = 0.0f32;
    let mut i = -(half_len as i64);
    while i <= half_len as i64 {
        let v = coeffs[(i + half_len as i64) as usize];
        sum += v * v;
        i += stride as i64;
    }
    sum
}

/// Builds the analysis and synthesis windows exactly as
/// `pvoc_process` does (`legacy/dev/pv/pvoc.c` lines 272-358).
///
/// - `m` is `M`, the window length (already resolved from the
///   overlap factor: `4*n`, `2*n`, `n` or `n/2` for overlap factors 0
///   through 3, per `pvoc_process`'s own `switch`).
/// - `n_chans` is `N` (`PVOC_CHANS`), always even.
/// - `interp` is `I`/`IO`, the interpolation factor (equal to the
///   decimation factor `D` unless a future WP adds true
///   time-stretching, which `pvoc_process` does not do either).
///
/// Both windows share `M`'s half-length. The analysis window gets a
/// sinc taper when `M > N` (so its impulse response fits inside the
/// window's true extent); otherwise it is a plain (renormalised)
/// Hamming window. The synthesis window mirrors that split, but on
/// the *opposite* condition (`M <= N` takes the plain path, `M > N`
/// takes the taper), per `pvoc_process`'s own two branches:
///
/// ```c
/// if (M <= dz->iparam[PVOC_CHANS]){
///     hamming(synWindow,synWinLen,Mf);
///     ...
///     for (i = -synWinLen; i <= synWinLen; i++)
///         *(synWindow + i) *= sum;               // sum == 2/anal_raw_sum
///     sum = 0.0f;
///     for (i = -synWinLen; i <= synWinLen; i+=I)
///         sum += *(synWindow + i) * *(synWindow + i);
///     sum = (float)(1.0/ sum);
///     for (i = -synWinLen; i <= synWinLen; i++)
///         *(synWindow + i) *= sum;
/// } else {
///     hamming(synWindow,synWinLen,Mf);
///     ...sinc taper using IO instead of N...
///     sum = (float)(1.0/sum);                    // sum is STILL 2/anal_raw_sum here
///     for (i = -synWinLen; i <= synWinLen; i++)
///         *(synWindow + i) *= sum;
/// }
/// ```
///
/// The `else` branch never recomputes `sum` from the synthesis
/// window; it reuses whatever the analysis window's normalisation
/// left in that variable. This is ported as observed, not corrected,
/// matching this project's practice for other confirmed legacy
/// quirks (see e.g. `cdp-data`'s mix-file `check_right_pan` note).
pub fn build_windows(m: usize, n_chans: usize, interp: usize) -> WindowPair {
    let even = m.is_multiple_of(2); // Mf = 1 - M%2
    let half_len = m / 2;

    let mut anal = make_symmetric(&hamming(half_len, even), half_len, even);
    if m > n_chans {
        apply_sinc_taper(&mut anal, half_len, even, n_chans as f64);
    }
    let anal_raw_sum = sum_all(&anal);
    let anal_scale = (2.0_f64 / anal_raw_sum as f64) as f32;
    scale_all(&mut anal, anal_scale);

    let mut syn = make_symmetric(&hamming(half_len, even), half_len, even);
    if m <= n_chans {
        scale_all(&mut syn, anal_scale);
        let sumsq = sum_sq_strided(&syn, half_len, interp);
        let scale2 = (1.0_f64 / sumsq as f64) as f32;
        scale_all(&mut syn, scale2);
    } else {
        apply_sinc_taper(&mut syn, half_len, even, interp as f64);
        // Legacy quirk: reuses `anal_scale`, not a fresh sum of `syn`.
        let scale2 = (1.0_f64 / anal_scale as f64) as f32;
        scale_all(&mut syn, scale2);
    }

    WindowPair {
        analysis: Window {
            half_len,
            coeffs: anal,
        },
        synthesis: Window {
            half_len,
            coeffs: syn,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Every expected value below was computed by an independent
    // Python port of the same C lines (struct.pack/unpack forcing
    // float32 rounding at each cast point, not this module's code).
    // See the oracle script referenced in this WP's PR description.

    #[test]
    fn hamming_even_forces_last_sample_to_zero() {
        let w = hamming(4, true);
        assert_eq!(w.len(), 5);
        assert_eq!(w[4], 0.0);
    }

    #[test]
    fn hamming_odd_forces_first_sample_to_one() {
        let w = hamming(4, false);
        assert_eq!(w[0], 1.0);
    }

    #[test]
    fn hamming_matches_independent_python_oracle() {
        let w = hamming(8, true);
        let expected = [
            0.991_161_2_f32,
            0.922_476,
            0.795_562_27,
            0.629_741_55,
            0.450_258_43,
            0.284_437_66,
            0.157_523_96,
            0.088_838_76,
            0.0,
        ];
        for (i, &e) in expected.iter().enumerate() {
            assert!(
                (w[i] - e).abs() < 1e-6,
                "even index {i}: got {}, expected {}",
                w[i],
                e
            );
        }

        let w = hamming(8, false);
        let expected = [
            1.0_f32,
            0.964_984_6,
            0.865_269_1,
            0.716_034_35,
            0.539_999_96,
            0.363_965_57,
            0.214_730_87,
            0.115_015_39,
            0.08,
        ];
        for (i, &e) in expected.iter().enumerate() {
            assert!(
                (w[i] - e).abs() < 1e-6,
                "odd index {i}: got {}, expected {}",
                w[i],
                e
            );
        }
    }

    fn assert_close(label: &str, got: f32, expected: f32) {
        let tol = 1e-4 * expected.abs().max(1.0);
        assert!(
            (got - expected).abs() < tol,
            "{label}: got {got}, expected {expected}"
        );
    }

    #[test]
    fn build_windows_n1024_overlap0_m_gt_n_taper_branch() {
        let wp = build_windows(4096, 1024, 512);
        assert_eq!(wp.analysis.half_len(), 2048);
        assert_close("anal[0]", wp.analysis.at(0), 0.0019455099);
        assert_close("anal[+1]", wp.analysis.at(1), 0.0019455018);
        assert_close("anal[+half]", wp.analysis.at(2048), 0.0);
        assert_close("syn[0]", wp.synthesis.at(0), 514.002_9);
        assert_close("syn[-half]", wp.synthesis.at(-2048), -0.010041584);
        assert_close("syn[+half]", wp.synthesis.at(2048), 0.0);
    }

    #[test]
    fn build_windows_n1024_overlap1_m_gt_n_taper_branch() {
        let wp = build_windows(2048, 1024, 256);
        assert_close("anal[0]", wp.analysis.at(0), 0.0023133089);
        assert_close("syn[0]", wp.synthesis.at(0), 432.2779);
        assert_close("syn[+1]", wp.synthesis.at(1), 432.2543);
    }

    #[test]
    fn build_windows_n1024_overlap2_m_eq_n_plain_branch() {
        let wp = build_windows(1024, 1024, 128);
        assert_close("anal[0]", wp.analysis.at(0), 0.0036168916);
        assert_close("anal[-half]", wp.analysis.at(-512), 0.00028935977);
        assert_close("syn[0]", wp.synthesis.at(0), 86.965065);
        assert_close("syn[-half]", wp.synthesis.at(-512), 6.957_408);
    }

    #[test]
    fn build_windows_n1024_overlap3_m_lt_n_plain_branch() {
        let wp = build_windows(512, 1024, 64);
        assert_close("anal[0]", wp.analysis.at(0), 0.0072337417);
        assert_close("syn[0]", wp.synthesis.at(0), 43.482_22);
        assert_close("syn[+1]", wp.synthesis.at(1), 43.479206);
    }

    #[test]
    fn build_windows_n256_boundary_small() {
        let wp = build_windows(256, 256, 32);
        assert_close("anal[0]", wp.analysis.at(0), 0.014467086);
        assert_close("syn[0]", wp.synthesis.at(0), 21.740574);
    }

    #[test]
    fn build_windows_n4096_boundary_large() {
        let wp = build_windows(16384, 4096, 2048);
        assert_close("anal[0]", wp.analysis.at(0), 0.00048637897);
        assert_close("syn[0]", wp.synthesis.at(0), 2056.0098);
    }

    #[test]
    fn build_windows_odd_m_center_is_true_center_sample() {
        // N=514 (even, per PVOC_CHANS always-even), overlap=3 gives
        // M = N/2 = 257 (odd), the real case where Mf=0 arises.
        let wp = build_windows(257, 514, 32);
        assert_eq!(wp.analysis.half_len(), 128);
        assert_close("anal[0]", wp.analysis.at(0), 0.014459229);
        // Odd M: edges are NOT forced to zero (contrast the even-M
        // cases above), and are symmetric since half_len steps by a
        // whole i-mf=i offset on both sides.
        assert_close("anal[-half]", wp.analysis.at(-128), 0.0011567383);
        assert_close("anal[+half]", wp.analysis.at(128), 0.0011567383);
        assert_eq!(wp.analysis.at(-128), wp.analysis.at(128));
        assert_close("syn[0]", wp.synthesis.at(0), 21.710_19);
    }

    #[test]
    fn analysis_window_renormalises_to_sum_two() {
        // Every configuration's analysis window is scaled so its sum
        // is exactly 2.0 before float rounding (`sum = 2.0/sum` then
        // rescaled by it) -- a property of the construction itself,
        // not read off the oracle, so worth asserting generically.
        for (m, n, i) in [
            (4096, 1024, 512),
            (2048, 1024, 256),
            (1024, 1024, 128),
            (512, 1024, 64),
            (256, 256, 32),
        ] {
            let wp = build_windows(m, n, i);
            let sum: f32 = wp.analysis.as_slice().iter().sum();
            assert!(
                (sum - 2.0).abs() < 1e-3,
                "M={m} N={n}: analysis window sum {sum}, expected ~2.0"
            );
        }
    }
}
