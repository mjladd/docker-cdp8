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

//! DSP primitives for the CDP System.
//!
//! This crate is WP-1.4 of the migration plan
//! (`docs/migration/PLAN.md`). It replaces `legacy/dev/pv/pvoc.c` and
//! `mxfft.c`'s numeric core, and the splice/interpolation/filter/RNG
//! helpers in `legacy/dev/cdp2k/tklib3.c`.
//!
//! Current scope (see `docs/migration/STATUS.md` for the live list):
//! the phase vocoder's analysis and synthesis window construction
//! ([`windows`]), ported from the window-setup block in
//! `pvoc_process` (`legacy/dev/pv/pvoc.c` lines 272-358); and the
//! real-signal FFT wrapper ([`fft`]) `pvoc_process`'s per-frame loop
//! will call. The per-frame analysis/synthesis loop itself, splices,
//! interpolators, filters and the seedable RNG are not implemented
//! yet.

/// The literal value of `PI` in `legacy/dev/include/globcon.h`: a
/// truncated decimal (`3.141592654`), not full double-precision pi
/// (`std::f64::consts::PI` is `3.14159265358979...`). The phase
/// vocoder's window math is ported against this exact constant, since
/// legacy's own arithmetic used it, not the mathematically precise
/// value.
#[allow(clippy::approx_constant)]
pub const LEGACY_PI: f64 = 3.141592654;

/// `TWOPI` from `legacy/dev/include/globcon.h`.
pub const LEGACY_TWOPI: f64 = 2.0 * LEGACY_PI;

pub mod fft;
pub mod windows;
