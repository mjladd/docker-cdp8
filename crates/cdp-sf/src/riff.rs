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

//! A generic RIFF chunk reader.
//!
//! legacy: `legacy/dev/newsfsys/sfsys.c` reads chunks with a
//! big-endian-composed tag (`read_dw_msf`, so the four ASCII bytes of
//! a tag land in file order) and a little-endian `DWORD` size
//! (`read_dw_lsf`). RIFF pads each chunk's data to an even number of
//! bytes; a chunk with an odd `size` is followed by one pad byte that
//! is not part of `size`.
//!
//! This reader does not assume a fixed chunk order. The legacy reader
//! does not either: it scans every top-level chunk in whatever order
//! it appears, because "the PEAK chunk is after data chunk in some
//! naff but otherwise legal files" (a comment left in `sfsys.c`).

use crate::error::{FourCc, Result, SfError};
use std::io::{Read, Seek, SeekFrom};

pub const RIFF: FourCc = FourCc::new(b"RIFF");
pub const WAVE: FourCc = FourCc::new(b"WAVE");
pub const FMT_: FourCc = FourCc::new(b"fmt ");
pub const DATA: FourCc = FourCc::new(b"data");
pub const PEAK: FourCc = FourCc::new(b"PEAK");
pub const LIST: FourCc = FourCc::new(b"LIST");
pub const ADTL: FourCc = FourCc::new(b"adtl");
pub const NOTE: FourCc = FourCc::new(b"note");
pub const SFIF: FourCc = FourCc::new(b"sfif");
pub const CUE_: FourCc = FourCc::new(b"cue ");

/// One top-level chunk: its tag, its data (already read into memory),
/// and the file offset its data starts at (some callers, e.g. writers
/// doing an in-place PEAK update, need this; readers usually do not).
pub struct Chunk {
    pub tag: FourCc,
    pub data: Vec<u8>,
}

/// Reads the RIFF header (`RIFF<size>WAVE`) and returns every
/// following top-level chunk. `size` in the RIFF header itself is not
/// checked against the actual file length: legacy CDP files often
/// leave it referring to a size written before later chunks (like
/// PEAK) were appended, and the legacy reader does not check it
/// either.
pub fn read_riff_wave<R: Read + Seek>(r: &mut R) -> Result<Vec<Chunk>> {
    let tag = read_fourcc(r)?;
    if tag != RIFF {
        return Err(SfError::NotRiff);
    }
    let _riff_size = read_u32_le(r)?;
    let form = read_fourcc(r)?;
    if form != WAVE {
        return Err(SfError::NotWave);
    }

    let mut chunks = Vec::new();
    loop {
        let tag = match read_fourcc(r) {
            Ok(t) => t,
            Err(SfError::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };
        let size = read_u32_le(r)? as usize;
        let mut data = vec![0u8; size];
        r.read_exact(&mut data)
            .map_err(|_| SfError::MalformedChunk(tag))?;
        if size % 2 == 1 {
            // RIFF pad byte; ignore its value, but it must be there
            // unless we are exactly at EOF (some encoders omit the
            // final pad byte on the last chunk).
            let mut pad = [0u8; 1];
            let _ = r.read(&mut pad);
        }
        chunks.push(Chunk { tag, data });
    }
    Ok(chunks)
}

pub fn read_fourcc<R: Read>(r: &mut R) -> Result<FourCc> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(FourCc(buf))
}

pub fn read_u32_le<R: Read>(r: &mut R) -> Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

pub fn read_u16_le<R: Read>(r: &mut R) -> Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

/// Writes a chunk tag, its little-endian size, its data, and the RIFF
/// pad byte if `data.len()` is odd.
pub fn write_chunk<W: std::io::Write>(w: &mut W, tag: FourCc, data: &[u8]) -> Result<()> {
    w.write_all(&tag.0)?;
    w.write_all(&(data.len() as u32).to_le_bytes())?;
    w.write_all(data)?;
    if data.len() % 2 == 1 {
        w.write_all(&[0u8])?;
    }
    Ok(())
}

/// Seeks to `pos` and returns the previous position, for the small
/// number of callers (PEAK-chunk backpatching, `data`-chunk-size
/// backpatching) that need to fix up a header field after writing
/// more data than was known when the header was first written.
pub fn seek_and_return<S: Seek>(s: &mut S, pos: u64) -> Result<u64> {
    let prev = s.stream_position()?;
    s.seek(SeekFrom::Start(pos))?;
    Ok(prev)
}
