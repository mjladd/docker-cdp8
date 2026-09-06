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

//! Sound file I/O for the CDP System.
//!
//! This crate is WP-1.1 of the migration plan
//! (`docs/migration/PLAN.md`). It replaces the file-I/O parts of
//! `legacy/dev/newsfsys` (`sfsys.c`, `snd.c`, `props.c`, `ieee80.c`)
//! and `legacy/dev/pvxio2/pvfileio.c`.
//!
//! Current scope (see `docs/migration/STATUS.md` for the live list):
//! WAVE files, 16-bit PCM and 32-bit float sample data, the `PEAK`
//! chunk, and reading (not yet writing) the CDP named-property block.
//! AIFF/AIFC, PVOC-EX, WAVE_FORMAT_EXTENSIBLE multichannel formats,
//! and the CDP-derived file types (analysis, pitch, transposition,
//! formant, envelope) are not implemented yet.

pub mod error;
pub mod props;
pub mod reader;
pub mod riff;
pub mod writer;

pub use error::{FourCc, Result, SfError};
pub use props::{ChannelPeak, FmtInfo, PropertyBlock, SampleType};
pub use reader::SoundFile;
pub use writer::{SoundFileWriter, WriteSpec};
