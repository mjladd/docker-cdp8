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

//! A generic IFF/FORM chunk reader for AIFF and AIFC.
//!
//! legacy: `legacy/dev/newsfsys/sfsys.c`'s `rdaiffhdr`/`rdaifchdr`.
//! Structurally this is [`crate::riff`]'s RIFF reader with two
//! differences: the container tag is `FORM<size><AIFF|AIFC>` rather
//! than `RIFF<size>WAVE`, and every size (the FORM size and each
//! chunk's size) is big-endian rather than little-endian -- legacy
//! reads them with `read_dw_msf` ("most significant byte first"),
//! the same function it uses for the four-byte tags themselves. Odd-
//! sized chunks are padded to an even boundary exactly as in RIFF.
//!
//! Like [`crate::riff::read_riff_wave`], this collects every chunk
//! into memory up front rather than assuming a fixed order --
//! confirmed necessary here too: legacy's own comment about "the PEAK
//! chunk is after data chunk in some naff but otherwise legal files"
//! is a RIFF-specific comment, but `rdaiffhdr`'s chunk loop is
//! equally order-independent (it is a single `switch` inside a `while
//! remain > 0`, matching on whichever tag comes next), so this crate
//! does not assume `COMM` precedes `SSND` either.

use crate::error::{FourCc, Result, SfError};
use std::io::{Read, Seek};

pub const FORM: FourCc = FourCc::new(b"FORM");
pub const AIFF: FourCc = FourCc::new(b"AIFF");
pub const AIFC: FourCc = FourCc::new(b"AIFC");
pub const COMM: FourCc = FourCc::new(b"COMM");
pub const SSND: FourCc = FourCc::new(b"SSND");
pub const APPL: FourCc = FourCc::new(b"APPL");
pub const FVER: FourCc = FourCc::new(b"FVER");

/// legacy: `eaaiff` vs `aiffc` in the `filetype` enum in `sfsys.c` --
/// which of the two AIFF form types a file declares itself as. Plain
/// `AIFF`'s `COMM` chunk has no compression-type field and is always
/// integer PCM; `AIFC`'s `COMM` chunk has one, which is what lets it
/// declare 32-bit float samples (`compressionType == "FL32"/"fl32"`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiffForm {
    Aiff,
    Aifc,
}

pub struct Chunk {
    pub tag: FourCc,
    pub data: Vec<u8>,
}

/// Reads the `FORM<size><AIFF|AIFC>` header and every following
/// top-level chunk.
pub fn read_form<R: Read + Seek>(r: &mut R) -> Result<(AiffForm, Vec<Chunk>)> {
    let tag = read_fourcc(r)?;
    if tag != FORM {
        return Err(SfError::NotAiff);
    }
    let _form_size = read_u32_be(r)?;
    let form = read_fourcc(r)?;
    let form = if form == AIFF {
        AiffForm::Aiff
    } else if form == AIFC {
        AiffForm::Aifc
    } else {
        return Err(SfError::NotAiffForm);
    };

    let mut chunks = Vec::new();
    loop {
        let tag = match read_fourcc(r) {
            Ok(t) => t,
            Err(SfError::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        };
        let size = read_u32_be(r)? as usize;
        let mut data = vec![0u8; size];
        r.read_exact(&mut data)
            .map_err(|_| SfError::MalformedChunk(tag))?;
        if size % 2 == 1 {
            let mut pad = [0u8; 1];
            let _ = r.read(&mut pad);
        }
        chunks.push(Chunk { tag, data });
    }
    Ok((form, chunks))
}

fn read_fourcc<R: Read>(r: &mut R) -> Result<FourCc> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(FourCc(buf))
}

fn read_u32_be<R: Read>(r: &mut R) -> Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
}

/// Decodes an 80-bit IEEE extended-precision float (the on-disk
/// format of an AIFF `COMM` chunk's sample rate). legacy:
/// `ieee_80_to_double` in `legacy/dev/newsfsys/ieee80.c`, ported
/// straight across since it is already written in a portable,
/// bit-twiddling style with no platform-specific casts to translate.
/// legacy then truncates the result to a `DWORD` with a plain C cast
/// (`(DWORD) Csound_res`, i.e. truncation toward zero, not rounding);
/// [`read_sample_rate`] reproduces that truncation.
pub fn ieee80_to_f64(p: &[u8; 10]) -> f64 {
    let exp_raw = u16::from_be_bytes([p[0], p[1]]);
    let sign = exp_raw & 0x8000 != 0;
    let exp = (exp_raw & 0x7FFF) as i32;
    let mant1 = u32::from_be_bytes([p[2], p[3], p[4], p[5]]);
    let mant0 = u32::from_be_bytes([p[6], p[7], p[8], p[9]]);

    if mant1 == 0 && mant0 == 0 && exp == 0 && !sign {
        return 0.0;
    }
    let mut val = mant0 as f64 * 2f64.powi(-63) + mant1 as f64 * 2f64.powi(-31);
    val *= 2f64.powi(exp - 16383);
    if sign { -val } else { val }
}

/// legacy: `read_ex_todw`'s `(DWORD) Csound_res` truncation.
pub fn read_sample_rate(bytes: &[u8]) -> Result<u32> {
    let ext: [u8; 10] = bytes
        .try_into()
        .map_err(|_| SfError::MalformedChunk(COMM))?;
    Ok(ieee80_to_f64(&ext) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 44100.0 as a Sound Designer II / AIFF 80-bit extended float,
    /// bytes confirmed against a real file (`docs/manual/sounds/ws2/
    /// tsw1-2nd.aiff`'s `COMM` chunk, byte-for-byte).
    #[test]
    fn decodes_44100_sample_rate() {
        let bytes = [0x40, 0x0E, 0xAC, 0x44, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(read_sample_rate(&bytes).unwrap(), 44100);
    }

    #[test]
    fn decodes_zero() {
        assert_eq!(ieee80_to_f64(&[0u8; 10]), 0.0);
    }
}
