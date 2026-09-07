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

//! Reading a WAVE or AIFF/AIFC sound file. legacy:
//! `legacy/dev/newsfsys/sfsys.c` (header parsing), `snd.c`
//! (`fgetfbufEx`, the sample-to-float conversion), `props.c`
//! (`sf_headread`).

use crate::aiff::{self, AiffForm};
use crate::error::{Result, SfError};
use crate::props::{self, ChannelPeak, FmtInfo, PropertyBlock, SampleType};
use crate::riff::{self, Chunk};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

/// legacy: `MAXSHORT` in `legacy/dev/newinclude/sfsys.h` -- "maxint
/// for shorts (as a float); used EVERYWHERE!". Both the int16<->f32
/// conversion in this crate's reader and writer use this exact
/// constant, matching `fgetshortEx`/`fputshortEx` in
/// `legacy/dev/newsfsys/snd.c`.
pub const MAXSHORT: f32 = 32767.0;

/// A WAVE or AIFF/AIFC file opened for reading: its format, PEAK data
/// (if any), named properties (if any), and raw sample bytes.
#[derive(Debug)]
pub struct SoundFile {
    pub fmt: FmtInfo,
    /// legacy: `props->type` -- always [`props::FileKind::Wave`]
    /// unless `fmt.sample_type` is [`SampleType::Float32`] and the
    /// property block marks it as a CDP-derived analysis-family file.
    pub file_kind: props::FileKind,
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

    /// Peeks the first four bytes (`RIFF` or `FORM`) to decide which
    /// container this file uses, then parses it accordingly.
    pub fn from_reader<R: Read + Seek>(mut r: R) -> Result<Self> {
        let mut magic = [0u8; 4];
        r.read_exact(&mut magic)?;
        r.seek(SeekFrom::Start(0))?;
        if &magic == b"FORM" {
            let (form, chunks) = aiff::read_form(&mut r)?;
            Self::from_aiff_chunks(form, chunks)
        } else {
            let chunks = riff::read_riff_wave(&mut r)?;
            Self::from_chunks(chunks)
        }
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
        let file_kind = props::detect_file_kind(&fmt, &properties)?;

        Ok(SoundFile {
            fmt,
            file_kind,
            peak_timestamp,
            peaks,
            properties,
            data,
        })
    }

    /// legacy: `rdaiffhdr`/`rdaifchdr`'s `COMM`/`SSND`/`PEAK`/`APPL`
    /// handling in `legacy/dev/newsfsys/sfsys.c`. Unlike those
    /// functions (which seek around a single open file descriptor,
    /// and carefully avoid trusting `SSND`'s own declared size --
    /// see the "RWD98 BUG" comment there about it disagreeing with
    /// the true data length), this works from chunks already fully
    /// read into memory by [`aiff::read_form`], so the audio data is
    /// simply everything in the `SSND` chunk from `8 + ssnd_offset`
    /// (past its own `offset`/`blockSize` header fields) to the end
    /// of that chunk's own (correctly parsed) size -- there is no
    /// separate declared length to disagree with.
    fn from_aiff_chunks(form: AiffForm, chunks: Vec<aiff::Chunk>) -> Result<Self> {
        let comm = chunks
            .iter()
            .find(|c| c.tag == aiff::COMM)
            .ok_or(SfError::MissingCommChunk)?;
        let fmt = props::parse_aiff_comm(&comm.data, form)?;

        let ssnd = chunks
            .iter()
            .find(|c| c.tag == aiff::SSND)
            .ok_or(SfError::MissingSsndChunk)?;
        if ssnd.data.len() < 8 {
            return Err(SfError::MalformedChunk(crate::error::FourCc::new(b"SSND")));
        }
        let ssnd_offset = u32::from_be_bytes(ssnd.data[0..4].try_into().unwrap()) as usize;
        let ssnd_block_size = u32::from_be_bytes(ssnd.data[4..8].try_into().unwrap()) as usize;
        if ssnd_offset > ssnd_block_size {
            return Err(SfError::FunnyAiffSsndOffset);
        }
        let start = 8 + ssnd_offset;
        let raw = ssnd.data.get(start..).ok_or(SfError::FunnyAiffSsndOffset)?;
        // legacy: AIFF/AIFC sample data is big-endian
        // (`REVDATAINFILE`); byte-swapped here, once, to little-endian
        // so the rest of this crate (writer included, since `-ffast-
        // math` aside this is a pure byte reordering) never needs to
        // know which container a `SoundFile` came from.
        let data = swap_endian(raw, (fmt.block_align / fmt.channels) as usize);

        let (peak_timestamp, peaks) = match chunks.iter().find(|c| c.tag == riff::PEAK) {
            Some(c) => {
                let (ts, pk) = props::parse_peak_chunk_be(&c.data, fmt.channels)?;
                (Some(ts), pk)
            }
            None => (None, Vec::new()),
        };

        let properties = chunks
            .iter()
            .find(|c| c.tag == aiff::APPL && c.data.len() >= 4 && &c.data[0..4] == b"sfif")
            .map(|c| PropertyBlock::parse(&c.data[4..]))
            .unwrap_or_default();
        let file_kind = props::detect_file_kind(&fmt, &properties)?;

        Ok(SoundFile {
            fmt,
            file_kind,
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

/// Reverses the byte order of every `width`-byte group in `data`
/// (e.g. `width = 2` for 16-bit samples). A trailing partial group
/// (a malformed file) is left as-is. `width <= 1` is a no-op.
fn swap_endian(data: &[u8], width: usize) -> Vec<u8> {
    if width <= 1 {
        return data.to_vec();
    }
    let mut out = Vec::with_capacity(data.len());
    for chunk in data.chunks(width) {
        out.extend(chunk.iter().rev());
    }
    out
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const SRATE_44100_EXT80: [u8; 10] = [0x40, 0x0E, 0xAC, 0x44, 0, 0, 0, 0, 0, 0];

    fn be_chunk(tag: &[u8; 4], data: &[u8]) -> Vec<u8> {
        let mut c = Vec::new();
        c.extend_from_slice(tag);
        c.extend_from_slice(&(data.len() as u32).to_be_bytes());
        c.extend_from_slice(data);
        if data.len() % 2 == 1 {
            c.push(0);
        }
        c
    }

    /// A minimal, hand-built one-channel 16-bit AIFF file: `FORM`
    /// containing just `COMM` and `SSND` (no `PEAK`/`APPL`), with two
    /// sample frames. Verifies the container-dispatch and byte-swap
    /// path end to end without depending on a real corpus file.
    fn minimal_aiff(samples_be: &[i16]) -> Vec<u8> {
        let mut comm = Vec::new();
        comm.extend_from_slice(&1u16.to_be_bytes()); // channels
        comm.extend_from_slice(&(samples_be.len() as u32).to_be_bytes()); // frames
        comm.extend_from_slice(&16u16.to_be_bytes()); // bits
        comm.extend_from_slice(&SRATE_44100_EXT80);

        let mut ssnd = Vec::new();
        ssnd.extend_from_slice(&0u32.to_be_bytes()); // offset
        ssnd.extend_from_slice(&0u32.to_be_bytes()); // block size
        for s in samples_be {
            ssnd.extend_from_slice(&s.to_be_bytes());
        }

        let mut body = Vec::new();
        body.extend_from_slice(b"AIFF");
        body.extend_from_slice(&be_chunk(b"COMM", &comm));
        body.extend_from_slice(&be_chunk(b"SSND", &ssnd));

        let mut file = Vec::new();
        file.extend_from_slice(b"FORM");
        file.extend_from_slice(&(body.len() as u32).to_be_bytes());
        file.extend_from_slice(&body);
        file
    }

    #[test]
    fn opens_a_minimal_aiff_file_and_decodes_its_samples() {
        let bytes = minimal_aiff(&[1000, -1000, 32767]);
        let sf = SoundFile::from_reader(Cursor::new(bytes)).unwrap();
        assert_eq!(sf.fmt.channels, 1);
        assert_eq!(sf.fmt.sample_rate, 44100);
        assert_eq!(sf.frame_count(), 3);
        let samples = sf.samples_f32().unwrap();
        assert_eq!(samples.len(), 3);
        assert!((samples[0] - 1000.0 / MAXSHORT).abs() < 1e-6);
        assert!((samples[1] - (-1000.0 / MAXSHORT)).abs() < 1e-6);
        assert!((samples[2] - 32767.0 / MAXSHORT).abs() < 1e-6);
    }

    #[test]
    fn missing_comm_chunk_is_an_error() {
        let mut ssnd = Vec::new();
        ssnd.extend_from_slice(&0u32.to_be_bytes());
        ssnd.extend_from_slice(&0u32.to_be_bytes());
        let mut body = Vec::new();
        body.extend_from_slice(b"AIFF");
        body.extend_from_slice(&be_chunk(b"SSND", &ssnd));
        let mut file = Vec::new();
        file.extend_from_slice(b"FORM");
        file.extend_from_slice(&(body.len() as u32).to_be_bytes());
        file.extend_from_slice(&body);

        let err = SoundFile::from_reader(Cursor::new(file)).unwrap_err();
        assert!(matches!(err, SfError::MissingCommChunk));
    }

    #[test]
    fn missing_ssnd_chunk_is_an_error() {
        let mut comm = Vec::new();
        comm.extend_from_slice(&1u16.to_be_bytes());
        comm.extend_from_slice(&0u32.to_be_bytes());
        comm.extend_from_slice(&16u16.to_be_bytes());
        comm.extend_from_slice(&SRATE_44100_EXT80);
        let mut body = Vec::new();
        body.extend_from_slice(b"AIFF");
        body.extend_from_slice(&be_chunk(b"COMM", &comm));
        let mut file = Vec::new();
        file.extend_from_slice(b"FORM");
        file.extend_from_slice(&(body.len() as u32).to_be_bytes());
        file.extend_from_slice(&body);

        let err = SoundFile::from_reader(Cursor::new(file)).unwrap_err();
        assert!(matches!(err, SfError::MissingSsndChunk));
    }

    #[test]
    fn a_wave_file_still_opens_after_the_format_dispatch_was_added() {
        let bytes = std::fs::read(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/synth_stereo_16bit.wav"
        ))
        .unwrap();
        let sf = SoundFile::from_reader(Cursor::new(bytes)).unwrap();
        assert_eq!(sf.fmt.channels, 2);
    }
}
