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

//! Sample type detection and the CDP named-property block.
//!
//! legacy: `legacy/dev/newsfsys/sfsys.c` (`sfgetprop`, the
//! `"sample type"` case) and `legacy/dev/newsfsys/props.c`
//! (`sf_headread`, which maps the raw `SAMP_*` code `sfgetprop`
//! returns onto the higher-level `sampletype` enum stored in
//! `SFPROPS`).

use crate::error::{Result, SfError};
use std::collections::BTreeMap;

const WAVE_FORMAT_PCM: u16 = 0x0001;
const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;

/// The 16-byte on-disk `SubFormat` GUID for PCM, as written by
/// `legacy/dev/newsfsys/sfsys.c`'s `KSDATAFORMAT_SUBTYPE_PCM`:
/// `DWORD Data1` (little-endian), `WORD Data2`, `WORD Data3`, then 8
/// raw `Data4` bytes.
const GUID_PCM: [u8; 16] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71,
];
/// As above, for `KSDATAFORMAT_SUBTYPE_IEEE_FLOAT`.
const GUID_IEEE_FLOAT: [u8; 16] = [
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xAA, 0x00, 0x38, 0x9B, 0x71,
];

/// legacy: `sampletype` in `legacy/dev/newinclude/sfsys.h`, as
/// narrowed by `sf_headread` in `props.c`. `SHORT8` is part of the C
/// enum but is never produced by `sf_headread`: the lower-level
/// `sfgetprop` "sample type" lookup maps both an 8-bit and a 16-bit
/// PCM container to `SAMP_SHORT`, and `sf_headread` maps `SAMP_SHORT`
/// to `Short16` unconditionally. That is a real quirk of the legacy
/// code, not an oversight in this port: an 8-bit PCM file is reported
/// (and, in the legacy code, decoded) as if it were 16-bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleType {
    Short16,
    Float32,
    Int32,
    Int2424,
    Int2432,
    Int2024,
    Masked,
}

impl SampleType {
    /// legacy: the `sample type:` line printed by
    /// `legacy/dev/sndinfo/compare.c`'s `prntprops`.
    pub fn describe(self) -> &'static str {
        match self {
            SampleType::Short16 => "16bit",
            SampleType::Float32 => "32bit floats",
            SampleType::Int2424 => "24bit packed",
            SampleType::Int2432 => "24bit in 32bit frames",
            SampleType::Int32 => "32bit integer",
            SampleType::Masked | SampleType::Int2024 => "unknown (custom format)",
        }
    }
}

/// The fields of a WAVE `fmt ` chunk this crate uses, plus the
/// derived `SampleType`. legacy: `WAVEFORMATEX` /
/// `WAVEFORMATEXTENSIBLE` in `legacy/dev/newinclude/pvfileio.h`, and
/// the `sample type` detection in `sfgetprop`.
#[derive(Debug, Clone, Copy)]
pub struct FmtInfo {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub block_align: u16,
    pub sample_type: SampleType,
}

pub fn parse_fmt_chunk(data: &[u8]) -> Result<FmtInfo> {
    if data.len() < 16 {
        return Err(SfError::MalformedChunk(crate::riff::FMT_));
    }
    let format_tag = u16::from_le_bytes([data[0], data[1]]);
    let channels = u16::from_le_bytes([data[2], data[3]]);
    let sample_rate = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    let block_align = u16::from_le_bytes([data[12], data[13]]);
    let bits_per_sample = u16::from_le_bytes([data[14], data[15]]);

    if channels == 0 {
        return Err(SfError::MalformedChunk(crate::riff::FMT_));
    }

    // legacy: sfgetprop's "sample type" case computes
    // containersize = 8 * (nBlockAlign / nChannels)
    // i.e. the storage width in bits of one channel's sample slot,
    // which is not always equal to wBitsPerSample (e.g. 24-in-32).
    let container_bits = 8 * (block_align as u32 / channels as u32);

    // For WAVE_FORMAT_EXTENSIBLE, the real format lives in the
    // SubFormat GUID at the end of the (40-byte) fmt chunk, and
    // wValidBitsPerSample (same offset as wBitsPerSample) replaces it
    // for container-size purposes only when the container itself is
    // ambiguous (32-bit container holding 24 valid bits, etc).
    let is_extensible = format_tag == WAVE_FORMAT_EXTENSIBLE;
    let valid_bits_per_sample = if is_extensible && data.len() >= 40 {
        u16::from_le_bytes([data[18], data[19]])
    } else {
        bits_per_sample
    };
    let sub_format: Option<[u8; 16]> = if is_extensible && data.len() >= 40 {
        Some(data[24..40].try_into().unwrap())
    } else {
        None
    };
    let is_ieee_float_subformat = sub_format == Some(GUID_IEEE_FLOAT);
    let is_pcm_subformat = sub_format == Some(GUID_PCM);

    let sample_type = match container_bits {
        32 => {
            if format_tag == WAVE_FORMAT_IEEE_FLOAT || (is_extensible && is_ieee_float_subformat) {
                SampleType::Float32
            } else if format_tag == WAVE_FORMAT_PCM || (is_extensible && is_pcm_subformat) {
                match valid_bits_per_sample {
                    32 => SampleType::Int32,
                    24 => SampleType::Int2432,
                    _ => SampleType::Masked,
                }
            } else {
                return Err(SfError::UnsupportedSampleFormat {
                    format_tag,
                    bits_per_sample,
                });
            }
        }
        24 => match valid_bits_per_sample {
            24 => SampleType::Int2424,
            20 => SampleType::Int2024,
            _ => SampleType::Masked,
        },
        16 | 8 => SampleType::Short16,
        _ => {
            return Err(SfError::UnsupportedSampleFormat {
                format_tag,
                bits_per_sample,
            });
        }
    };

    Ok(FmtInfo {
        channels,
        sample_rate,
        bits_per_sample,
        block_align,
        sample_type,
    })
}

/// A per-channel entry from the `PEAK` chunk. legacy: `CHPEAK` in
/// `legacy/dev/newinclude/sfsys.h` (`float value; unsigned int
/// position;`), a peak sample's absolute value and the frame position
/// it first occurred at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChannelPeak {
    pub value: f32,
    pub position: u32,
}

const CURRENT_PEAK_VERSION: u32 = 1;

/// Parses a `PEAK` chunk's payload (after the tag and size, which the
/// RIFF walker already consumed). legacy: `read_peak_lsf` and its
/// caller in `legacy/dev/newsfsys/sfsys.c`.
pub fn parse_peak_chunk(data: &[u8], channels: u16) -> Result<(u32, Vec<ChannelPeak>)> {
    if data.len() < 8 {
        return Err(SfError::MalformedChunk(crate::riff::PEAK));
    }
    let version = u32::from_le_bytes(data[0..4].try_into().unwrap());
    let timestamp = u32::from_le_bytes(data[4..8].try_into().unwrap());
    if version != CURRENT_PEAK_VERSION {
        // legacy: unrecognised PEAK version is silently dropped
        // (f->peaks freed, left NULL), not an error.
        return Ok((timestamp, Vec::new()));
    }
    let mut peaks = Vec::with_capacity(channels as usize);
    let mut offset = 8usize;
    for _ in 0..channels {
        if offset + 8 > data.len() {
            return Err(SfError::MalformedChunk(crate::riff::PEAK));
        }
        let value = f32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        let position = u32::from_le_bytes(data[offset + 4..offset + 8].try_into().unwrap());
        peaks.push(ChannelPeak { value, position });
        offset += 8;
    }
    Ok((timestamp, peaks))
}

/// Encodes a `PEAK` chunk's payload. legacy: `write_peak_lsf` and its
/// caller.
pub fn encode_peak_chunk(timestamp: u32, peaks: &[ChannelPeak]) -> Vec<u8> {
    let mut data = Vec::with_capacity(8 + peaks.len() * 8);
    data.extend_from_slice(&CURRENT_PEAK_VERSION.to_le_bytes());
    data.extend_from_slice(&timestamp.to_le_bytes());
    for p in peaks {
        data.extend_from_slice(&p.value.to_le_bytes());
        data.extend_from_slice(&p.position.to_le_bytes());
    }
    data
}

/// The CDP named-property block: arbitrary name/value pairs stored as
/// raw bytes, read from (or written to) the `cue `/`LIST`/`adtl`/
/// `note`/`sfif` chunk group. legacy: `struct property` and
/// `parseprops`/`writeprops` in `legacy/dev/newsfsys/sfsys.c`.
///
/// This does not cover `"channels"`, `"sample rate"` or `"sample
/// type"`: those three names are special-cased in the legacy
/// `sfgetprop`/`sfputprop` to read and write the `fmt ` chunk
/// directly rather than this block, and this crate's [`FmtInfo`]
/// covers them instead.
#[derive(Debug, Clone, Default)]
pub struct PropertyBlock {
    entries: BTreeMap<String, Vec<u8>>,
}

impl PropertyBlock {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parses the text payload inside the `note` chunk's `sfif`
    /// sub-block (i.e. the bytes after the `"sfif"` tag). legacy:
    /// `parseprops`. Format: `name '\n' HEXVALUE '\n'` pairs,
    /// repeated, where `HEXVALUE` is the property's raw bytes as
    /// upper-case hex, two hex digits per byte in the same order as
    /// the original in-memory bytes (so little-endian for anything
    /// the legacy code stored as a native `int`/`float`, since it was
    /// authored on little-endian machines). The block ends at the
    /// first line that is empty (a `\n` with nothing before it), and
    /// everything from there to the end of the chunk's reserved space
    /// is unused padding.
    pub fn parse(payload: &[u8]) -> Self {
        let mut entries = BTreeMap::new();
        let mut pos = 0usize;
        while pos < payload.len() && payload[pos] != b'\n' {
            let Some(name_end) = payload[pos..].iter().position(|&b| b == b'\n') else {
                break;
            };
            let name_end = pos + name_end;
            let value_start = name_end + 1;
            let Some(value_len) = payload[value_start..].iter().position(|&b| b == b'\n') else {
                break;
            };
            let value_end = value_start + value_len;
            if value_len % 2 != 0 {
                break; // legacy: parseprops bails on an odd-length hex value too
            }
            let name = String::from_utf8_lossy(&payload[pos..name_end]).into_owned();
            let mut value = Vec::with_capacity(value_len / 2);
            let hex = &payload[value_start..value_end];
            for pair in hex.as_chunks::<2>().0 {
                let hi = hex_digit(pair[0]);
                let lo = hex_digit(pair[1]);
                value.push((hi << 4) | lo);
            }
            entries.insert(name, value);
            pos = value_end + 1;
        }
        PropertyBlock { entries }
    }

    /// Encodes this block back into the `name\nHEXVALUE\n` text form,
    /// terminated by a blank line, but does not pad to the legacy
    /// 2000-byte reservation (`PROPCNKSIZE`) -- the writer adds that
    /// padding, since only it knows whether it needs to match the
    /// legacy layout byte-for-byte or can size the chunk to fit.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for (name, value) in &self.entries {
            out.extend_from_slice(name.as_bytes());
            out.push(b'\n');
            for byte in value {
                out.push(hex_char(byte >> 4));
                out.push(hex_char(byte & 0x0F));
            }
            out.push(b'\n');
        }
        out.push(b'\n');
        out
    }

    pub fn get_raw(&self, name: &str) -> Option<&[u8]> {
        self.entries.get(name).map(Vec::as_slice)
    }

    pub fn get_i32(&self, name: &str) -> Result<i32> {
        let raw = self
            .get_raw(name)
            .ok_or_else(|| SfError::PropertyNotFound(name.to_string()))?;
        let bytes: [u8; 4] = raw.try_into().map_err(|_| SfError::PropertySize {
            name: name.to_string(),
            expected: 4,
            found: raw.len(),
        })?;
        Ok(i32::from_le_bytes(bytes))
    }

    pub fn get_f32(&self, name: &str) -> Result<f32> {
        let raw = self
            .get_raw(name)
            .ok_or_else(|| SfError::PropertyNotFound(name.to_string()))?;
        let bytes: [u8; 4] = raw.try_into().map_err(|_| SfError::PropertySize {
            name: name.to_string(),
            expected: 4,
            found: raw.len(),
        })?;
        Ok(f32::from_le_bytes(bytes))
    }

    pub fn set_i32(&mut self, name: &str, value: i32) {
        self.entries
            .insert(name.to_string(), value.to_le_bytes().to_vec());
    }

    pub fn set_f32(&mut self, name: &str, value: f32) {
        self.entries
            .insert(name.to_string(), value.to_le_bytes().to_vec());
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

fn hex_digit(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'A'..=b'F' => b - b'A' + 10,
        b'a'..=b'f' => b - b'a' + 10,
        // legacy: xtoi() returns 0 for any other character rather
        // than treating it as an error.
        _ => 0,
    }
}

fn hex_char(nibble: u8) -> u8 {
    // legacy: itox() uses the fixed table "0123456789ABCDEF".
    b"0123456789ABCDEF"[(nibble & 0x0F) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact bytes `legacy/dev/newsfsys/sfsys.c`'s `parseprops`
    /// would produce for a file with a `DATE` and a `maxamp`
    /// property, independently confirmed by hand-decoding a real
    /// file's `sfif` block in Python (see
    /// `tests/oracle_fixtures.rs`): `963996604_i32` little-endian is
    /// `BC6B7539` in hex.
    #[test]
    fn parses_a_real_property_block_layout() {
        let payload = b"DATE\nBC6B7539\nmaxamp\nC8760000\nmaxloc\n16AD0000\nmaxrep\n01000000\n\n";
        let block = PropertyBlock::parse(payload);
        assert_eq!(block.get_i32("DATE").unwrap(), 963_996_604);
        assert_eq!(block.get_i32("maxloc").unwrap(), 0x0000AD16);
        assert_eq!(block.get_i32("maxrep").unwrap(), 1);
    }

    #[test]
    fn stops_at_the_first_blank_line_and_ignores_trailing_padding() {
        let mut payload = b"a\n01000000\n\n".to_vec();
        payload.extend(std::iter::repeat_n(0u8, 100)); // legacy: PROPCNKSIZE padding
        let block = PropertyBlock::parse(&payload);
        assert_eq!(block.get_i32("a").unwrap(), 1);
        assert_eq!(block.get_raw("padding-should-not-exist"), None);
    }

    #[test]
    fn missing_property_is_an_error_not_a_default() {
        let block = PropertyBlock::parse(b"\n");
        assert!(matches!(
            block.get_i32("nonexistent"),
            Err(SfError::PropertyNotFound(_))
        ));
    }

    #[test]
    fn encode_decode_round_trip() {
        let mut block = PropertyBlock::new();
        block.set_i32("DATE", 963_996_604);
        block.set_f32("arate", 344.53125);
        let encoded = block.encode();
        let decoded = PropertyBlock::parse(&encoded);
        assert_eq!(decoded.get_i32("DATE").unwrap(), 963_996_604);
        assert_eq!(decoded.get_f32("arate").unwrap(), 344.53125);
    }

    #[test]
    fn peak_chunk_round_trip() {
        let peaks = vec![
            ChannelPeak {
                value: 0.999_991_24,
                position: 877,
            },
            ChannelPeak {
                value: 0.5,
                position: 1200,
            },
        ];
        let encoded = encode_peak_chunk(1_788_702_037, &peaks);
        let (ts, decoded) = parse_peak_chunk(&encoded, 2).unwrap();
        assert_eq!(ts, 1_788_702_037);
        assert_eq!(decoded, peaks);
    }

    #[test]
    fn unrecognised_peak_version_is_dropped_not_an_error() {
        // legacy: an unknown PEAK version frees f->peaks and leaves it
        // NULL, rather than failing the whole file open.
        let mut data = Vec::new();
        data.extend_from_slice(&99u32.to_le_bytes()); // bogus version
        data.extend_from_slice(&0u32.to_le_bytes());
        let (_, peaks) = parse_peak_chunk(&data, 2).unwrap();
        assert!(peaks.is_empty());
    }

    /// legacy: `fmt ` chunk for 16-bit PCM mono, as `synth` writes it
    /// (confirmed against `tests/fixtures/synth_stereo_16bit.wav`'s
    /// own `fmt ` chunk, read independently with Python: `(1, 2,
    /// 44100, 176400, 4, 16)` for wFormatTag, nChannels,
    /// nSamplesPerSec, nAvgBytesPerSec, nBlockAlign, wBitsPerSample).
    #[test]
    fn parses_standard_pcm16_fmt_chunk() {
        let mut fmt = Vec::new();
        fmt.extend_from_slice(&1u16.to_le_bytes()); // WAVE_FORMAT_PCM
        fmt.extend_from_slice(&2u16.to_le_bytes()); // channels
        fmt.extend_from_slice(&44100u32.to_le_bytes());
        fmt.extend_from_slice(&176400u32.to_le_bytes());
        fmt.extend_from_slice(&4u16.to_le_bytes()); // block align
        fmt.extend_from_slice(&16u16.to_le_bytes()); // bits per sample

        let info = parse_fmt_chunk(&fmt).unwrap();
        assert_eq!(info.channels, 2);
        assert_eq!(info.sample_rate, 44100);
        assert_eq!(info.bits_per_sample, 16);
        assert_eq!(info.sample_type, SampleType::Short16);
    }

    #[test]
    fn parses_standard_float32_fmt_chunk() {
        let mut fmt = Vec::new();
        fmt.extend_from_slice(&3u16.to_le_bytes()); // WAVE_FORMAT_IEEE_FLOAT
        fmt.extend_from_slice(&1u16.to_le_bytes());
        fmt.extend_from_slice(&44100u32.to_le_bytes());
        fmt.extend_from_slice(&176400u32.to_le_bytes());
        fmt.extend_from_slice(&4u16.to_le_bytes());
        fmt.extend_from_slice(&32u16.to_le_bytes());
        fmt.extend_from_slice(&0u16.to_le_bytes()); // cbSize = 0

        let info = parse_fmt_chunk(&fmt).unwrap();
        assert_eq!(info.sample_type, SampleType::Float32);
    }

    #[test]
    fn extensible_ieee_float_subformat_is_detected() {
        // legacy: WAVEFORMATEXTENSIBLE with the IEEE_FLOAT SubFormat
        // GUID, as sfgetprop's "sample type" case checks via
        // compare_guids against KSDATAFORMAT_SUBTYPE_IEEE_FLOAT.
        let mut fmt = Vec::new();
        fmt.extend_from_slice(&0xFFFEu16.to_le_bytes()); // WAVE_FORMAT_EXTENSIBLE
        fmt.extend_from_slice(&2u16.to_le_bytes());
        fmt.extend_from_slice(&44100u32.to_le_bytes());
        fmt.extend_from_slice(&352800u32.to_le_bytes());
        fmt.extend_from_slice(&8u16.to_le_bytes());
        fmt.extend_from_slice(&32u16.to_le_bytes());
        fmt.extend_from_slice(&22u16.to_le_bytes()); // cbSize
        fmt.extend_from_slice(&32u16.to_le_bytes()); // wValidBitsPerSample
        fmt.extend_from_slice(&0u32.to_le_bytes()); // dwChannelMask
        fmt.extend_from_slice(&GUID_IEEE_FLOAT);

        let info = parse_fmt_chunk(&fmt).unwrap();
        assert_eq!(info.sample_type, SampleType::Float32);
    }

    /// A single-byte truncated `fmt ` chunk must be a
    /// `MalformedChunk` error, not a panic from an out-of-bounds
    /// slice index.
    #[test]
    fn truncated_fmt_chunk_is_an_error_not_a_panic() {
        let fmt = vec![1u8, 0, 2, 0, 0x44, 0xAC];
        assert!(parse_fmt_chunk(&fmt).is_err());
    }
}
