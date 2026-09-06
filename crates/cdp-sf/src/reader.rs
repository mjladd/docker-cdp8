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

//! Reading a WAVE sound file. legacy: `legacy/dev/newsfsys/sfsys.c`
//! (header parsing), `legacy/dev/newsfsys/snd.c` (`fgetfbufEx`, the
//! sample-to-float conversion), `legacy/dev/newsfsys/props.c`
//! (`sf_headread`).

use crate::error::{Result, SfError};
use crate::props::{self, ChannelPeak, FmtInfo, PropertyBlock, SampleType};
use crate::riff::{self, Chunk};
use std::fs::File;
use std::io::{BufReader, Read, Seek};
use std::path::Path;

/// legacy: `MAXSHORT` in `legacy/dev/newinclude/sfsys.h` -- "maxint
/// for shorts (as a float); used EVERYWHERE!". Both the int16<->f32
/// conversion in this crate's reader and writer use this exact
/// constant, matching `fgetshortEx`/`fputshortEx` in
/// `legacy/dev/newsfsys/snd.c`.
pub const MAXSHORT: f32 = 32767.0;

/// A WAVE file opened for reading: its format, PEAK data (if any),
/// named properties (if any), and raw sample bytes.
pub struct SoundFile {
    pub fmt: FmtInfo,
    pub peak_timestamp: Option<u32>,
    pub peaks: Vec<ChannelPeak>,
    pub properties: PropertyBlock,
    data: Vec<u8>,
}

impl SoundFile {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let file = File::open(path)?;
        Self::from_reader(BufReader::new(file))
    }

    pub fn from_reader<R: Read + Seek>(mut r: R) -> Result<Self> {
        let chunks = riff::read_riff_wave(&mut r)?;
        Self::from_chunks(chunks)
    }

    fn from_chunks(chunks: Vec<Chunk>) -> Result<Self> {
        let fmt_chunk = chunks
            .iter()
            .find(|c| c.tag == riff::FMT_)
            .ok_or(SfError::MissingChunk(riff::FMT_))?;
        let fmt = props::parse_fmt_chunk(&fmt_chunk.data)?;

        let data = chunks
            .iter()
            .find(|c| c.tag == riff::DATA)
            .ok_or(SfError::MissingChunk(riff::DATA))?
            .data
            .clone();

        let (peak_timestamp, peaks) = match chunks.iter().find(|c| c.tag == riff::PEAK) {
            Some(c) => {
                let (ts, pk) = props::parse_peak_chunk(&c.data, fmt.channels)?;
                (Some(ts), pk)
            }
            None => (None, Vec::new()),
        };

        let properties = find_sfif_payload(&chunks)
            .as_deref()
            .map(PropertyBlock::parse)
            .unwrap_or_default();

        Ok(SoundFile {
            fmt,
            peak_timestamp,
            peaks,
            properties,
            data,
        })
    }

    /// Number of complete sample frames (one frame = one sample per
    /// channel) in the `data` chunk. Any trailing partial frame
    /// (a malformed file) is ignored, matching how `insams` is
    /// derived from a whole-frame-aligned sample count in the legacy
    /// code.
    pub fn frame_count(&self) -> u64 {
        let bytes_per_frame = self.bytes_per_frame();
        if bytes_per_frame == 0 {
            0
        } else {
            self.data.len() as u64 / bytes_per_frame as u64
        }
    }

    /// Total sample count across all channels (`insams` in the
    /// legacy `datalist`, i.e. `dz->insams[0]`).
    pub fn sample_count(&self) -> u64 {
        self.frame_count() * self.fmt.channels as u64
    }

    fn bytes_per_frame(&self) -> u32 {
        self.fmt.block_align as u32
    }

    /// Decodes every sample to `f32`, interleaved by channel, in the
    /// same -1.0..=1.0 range the legacy code's `dz->sampbuf` uses.
    /// legacy: `fgetfbufEx` in `snd.c`.
    ///
    /// Only [`SampleType::Short16`] and [`SampleType::Float32`] are
    /// implemented so far (see docs/migration/STATUS.md); every other
    /// `SampleType` this crate can already *report* via [`FmtInfo`]
    /// returns [`SfError::UnsupportedSampleDataDecoding`] here rather
    /// than silently decoding it wrong.
    pub fn samples_f32(&self) -> Result<Vec<f32>> {
        match self.fmt.sample_type {
            SampleType::Short16 if self.fmt.bits_per_sample == 16 => Ok(self
                .data
                .as_chunks::<2>()
                .0
                .iter()
                .map(|b| i16::from_le_bytes(*b) as f32 / MAXSHORT)
                .collect()),
            SampleType::Float32 => Ok(self
                .data
                .as_chunks::<4>()
                .0
                .iter()
                .map(|b| f32::from_le_bytes(*b))
                .collect()),
            other => Err(SfError::UnsupportedSampleDataDecoding(other)),
        }
    }
}

/// Finds the `sfif` property payload inside `LIST`/`adtl`/`note`.
/// legacy: the chunk-scanning loop around `TAG('n','o','t','e')` in
/// `legacy/dev/newsfsys/sfsys.c`'s WAVE reader.
fn find_sfif_payload(chunks: &[Chunk]) -> Option<Vec<u8>> {
    for list in chunks.iter().filter(|c| c.tag == riff::LIST) {
        if list.data.len() < 4 || &list.data[0..4] != riff::ADTL.0.as_slice() {
            continue;
        }
        let mut pos = 4usize;
        while pos + 8 <= list.data.len() {
            let tag = &list.data[pos..pos + 4];
            let size = u32::from_le_bytes(list.data[pos + 4..pos + 8].try_into().unwrap()) as usize;
            let body_start = pos + 8;
            let body_end = body_start.saturating_add(size).min(list.data.len());
            if tag == riff::NOTE.0.as_slice()
                && body_end - body_start >= 4
                && &list.data[body_start..body_start + 4] == riff::SFIF.0.as_slice()
            {
                return Some(list.data[body_start + 4..body_end].to_vec());
            }
            pos = body_end + (size % 2); // RIFF sub-chunk padding
        }
    }
    None
}
