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

/// The CDP-derived file types stored inside a float32 sound file's
/// property block. legacy: `wt_wave`/`wt_binenv`/`wt_pitch`/
/// `wt_transposition`/`wt_formant`/`wt_analysis` in
/// `legacy/dev/newinclude/sfsys.h`, as detected by `sf_headread`/
/// `snd_headread` in `legacy/dev/newsfsys/props.c`. Applies equally
/// to WAVE and AIFC (the only AIFF form that can carry float32
/// samples at all): the detection reads named properties from the
/// property block regardless of which container holds it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FileKind {
    /// An ordinary sound file (or, if not float32, always this).
    Wave,
    /// legacy: `wt_binenv`, detected by the `"is an envelope"` marker
    /// property. `window_size` (ms) is legacy's `props->window_size`.
    Envelope {
        window_size: f32,
    },
    Pitch(OriginalChannelInfo),
    Transposition(OriginalChannelInfo),
    Formant {
        channel_info: OriginalChannelInfo,
        /// legacy: `specenvcnt`, the `"specenvcnt"` property.
        spectral_envelope_count: i32,
    },
    /// legacy: `wt_analysis`, the default when none of the `"is a
    /// ... file"` marker properties are present.
    Analysis(SpectralInfo),
}

/// The five analysis properties every non-`Wave`, non-`Envelope`
/// [`FileKind`] carries: `original sampsize`, `original sample
/// rate`, `arate`, `analwinlen`, `decfactor`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpectralInfo {
    pub original_sample_size: i32,
    pub original_sample_rate: i32,
    pub analysis_rate: f32,
    pub analysis_window_length: i32,
    pub decimation_factor: i32,
}

/// [`SpectralInfo`] plus the `"orig channels"` property, which
/// `sf_headread` only reads for [`FileKind::Pitch`],
/// [`FileKind::Transposition`] and [`FileKind::Formant`] -- these
/// collapse the parent analysis file's many channels (amplitude/
/// frequency pairs per band) down to one, so `original_channels`
/// records how many the parent had.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OriginalChannelInfo {
    pub spectral: SpectralInfo,
    pub original_channels: i32,
}

/// legacy: `sf_headread`/`snd_headread`'s `props->samptype==FLOAT32`
/// branch in `legacy/dev/newsfsys/props.c`. Any other sample type is
/// always [`FileKind::Wave`] (that branch is skipped entirely, and
/// `props->type` is left at its `wt_wave` initial value).
pub fn detect_file_kind(fmt: &FmtInfo, properties: &PropertyBlock) -> Result<FileKind> {
    use crate::error::SfError;

    if fmt.sample_type != SampleType::Float32 {
        return Ok(FileKind::Wave);
    }

    let origsize = properties.get_i32("original sampsize");
    let origrate = properties.get_i32("original sample rate");
    let arate = properties.get_f32("arate");
    let wlen = properties.get_i32("analwinlen");
    let dfac = properties.get_i32("decfactor");
    let any_missing =
        origsize.is_err() || origrate.is_err() || arate.is_err() || wlen.is_err() || dfac.is_err();
    let origsize = origsize.unwrap_or(0);
    let origrate = origrate.unwrap_or(0);
    let arate = arate.unwrap_or(0.0);
    let wlen = wlen.unwrap_or(0);
    let dfac = dfac.unwrap_or(0);

    // legacy: checksum = origsize + origrate + wlen + dfac + (int)arate
    let checksum = origsize + origrate + wlen + dfac + (arate as i32);

    if checksum == 0 {
        // legacy: "its a wave file, or an envelope file" -- a missing
        // read here (any_missing) is not an error, since a genuine
        // plain WAVE file has none of the five properties at all.
        let is_envelope = properties.get_i32("is an envelope").unwrap_or(0) != 0;
        if !is_envelope {
            return Ok(FileKind::Wave);
        }
        let window_size = properties
            .get_f32("window size")
            .map_err(|_| SfError::MissingEnvelopeWindowSize)?;
        return Ok(FileKind::Envelope { window_size });
    }

    if any_missing {
        return Err(SfError::InconsistentAnalysisProperties);
    }
    let spectral = SpectralInfo {
        original_sample_size: origsize,
        original_sample_rate: origrate,
        analysis_rate: arate,
        analysis_window_length: wlen,
        decimation_factor: dfac,
    };

    // legacy: presence, not value, of these three marker properties
    // (`sfgetprop(...) >= 0`) -- checked in this order, pitch first.
    let is_pitch = properties.get_raw("is a pitch file").is_some();
    let is_transpos = properties.get_raw("is a transpos file").is_some();
    let is_formant = properties.get_raw("is a formant file").is_some();

    if !(is_pitch || is_transpos || is_formant) {
        return Ok(FileKind::Analysis(spectral));
    }

    // legacy: pitch, transposition and formant files all fall
    // through to the same "channels must be 1, then read orig
    // channels" handling.
    if fmt.channels != 1 {
        return Err(SfError::AnalysisFileChannelCountNotOne);
    }
    let original_channels = properties
        .get_i32("orig channels")
        .map_err(|_| SfError::MissingOriginalChannels)?;
    let channel_info = OriginalChannelInfo {
        spectral,
        original_channels,
    };

    if is_pitch {
        Ok(FileKind::Pitch(channel_info))
    } else if is_transpos {
        Ok(FileKind::Transposition(channel_info))
    } else {
        let spectral_envelope_count = properties
            .get_i32("specenvcnt")
            .map_err(|_| SfError::MissingSpectralEnvelopeCount)?;
        Ok(FileKind::Formant {
            channel_info,
            spectral_envelope_count,
        })
    }
}

/// legacy: `rdaiffhdr`/`rdaifchdr` in `legacy/dev/newsfsys/sfsys.c`,
/// the `COMM` chunk fields. `data` is the chunk payload with the tag
/// and size already stripped, as `crate::aiff::read_form` returns it.
/// Plain AIFF's `COMM` is a fixed 18 bytes (channels, frame count,
/// bits per sample, then the 10-byte extended-float sample rate) and
/// is always integer PCM; AIFC's is at least 22 bytes, adding a
/// 4-byte compression-type tag right after, which is what lets it
/// declare 32-bit float samples (`FL32`/`fl32`) or Steinberg's packed
/// 24-in-32 (`in24`) -- the rest of the chunk (a Pascal string naming
/// the compression) is not needed here and is skipped by the caller,
/// since [`crate::aiff::read_form`] already captured the whole chunk.
pub fn parse_aiff_comm(data: &[u8], form: crate::aiff::AiffForm) -> Result<FmtInfo> {
    use crate::aiff::AiffForm;
    use crate::error::SfError;

    let min_len = match form {
        AiffForm::Aiff => 18,
        AiffForm::Aifc => 22,
    };
    if data.len() < min_len || (form == AiffForm::Aiff && data.len() != 18) {
        return Err(SfError::MalformedAiffCommChunk);
    }

    let channels = u16::from_be_bytes([data[0], data[1]]);
    if channels == 0 {
        return Err(SfError::MalformedAiffCommChunk);
    }
    let mut bits_per_sample = u16::from_be_bytes([data[6], data[7]]);
    let sample_rate = crate::aiff::read_sample_rate(&data[8..18])?;

    let is_float = if form == AiffForm::Aifc {
        let compression = &data[18..22];
        match compression {
            b"NONE" => false,
            b"FL32" | b"fl32" => {
                // legacy: QuickTime writes size = 16 (i.e.
                // bits_per_sample 16) for a float AIFC COMM chunk;
                // legacy silently corrects it to 32 rather than
                // erroring, so this does too.
                if bits_per_sample == 16 {
                    bits_per_sample = 32;
                } else if bits_per_sample != 32 {
                    return Err(SfError::MalformedAiffCommChunk);
                }
                true
            }
            b"in24" => {
                if bits_per_sample != 24 {
                    return Err(SfError::MalformedAiffCommChunk);
                }
                false
            }
            _ => return Err(SfError::UnknownAifcCompressionType),
        }
    } else {
        false
    };

    let (block_align, sample_type) = match bits_per_sample {
        32 if is_float => (4u16, SampleType::Float32),
        32 => (4, SampleType::Int32),
        24 => (3, SampleType::Int2424),
        20 => (3, SampleType::Int2024),
        16 => (2, SampleType::Short16),
        // legacy quirk, matching the WAVE SHORT8 case documented on
        // `SampleType`: 8-bit AIFF is classified the same as 16-bit
        // for reporting, though its on-disk container is genuinely 1
        // byte per sample (`block_align` reflects that truthfully),
        // and -- as for WAVE -- this crate does not yet decode it;
        // see the `samples_f32` guard in `reader.rs`.
        8 => (1, SampleType::Short16),
        other => return Err(SfError::UnsupportedAiffSampleSize(other)),
    };

    Ok(FmtInfo {
        channels,
        sample_rate,
        bits_per_sample,
        block_align: block_align * channels,
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

/// As [`parse_peak_chunk`], for AIFF/AIFC: legacy's `read_peak_msf`
/// reads the same version/timestamp/per-channel layout, but every
/// field big-endian (`_msf` = "most significant byte first") rather
/// than little-endian.
pub fn parse_peak_chunk_be(data: &[u8], channels: u16) -> Result<(u32, Vec<ChannelPeak>)> {
    if data.len() < 8 {
        return Err(SfError::MalformedChunk(crate::riff::PEAK));
    }
    let version = u32::from_be_bytes(data[0..4].try_into().unwrap());
    let timestamp = u32::from_be_bytes(data[4..8].try_into().unwrap());
    if version != CURRENT_PEAK_VERSION {
        return Ok((timestamp, Vec::new()));
    }
    let mut peaks = Vec::with_capacity(channels as usize);
    let mut offset = 8usize;
    for _ in 0..channels {
        if offset + 8 > data.len() {
            return Err(SfError::MalformedChunk(crate::riff::PEAK));
        }
        let value = f32::from_be_bytes(data[offset..offset + 4].try_into().unwrap());
        let position = u32::from_be_bytes(data[offset + 4..offset + 8].try_into().unwrap());
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

    /// 44100.0 as a 10-byte IEEE-80 extended float, confirmed
    /// byte-for-byte against `docs/manual/sounds/ws2/tsw1-2nd.aiff`'s
    /// real `COMM` chunk (see `tests/oracle_fixtures.rs`).
    const SRATE_44100_EXT80: [u8; 10] = [0x40, 0x0E, 0xAC, 0x44, 0, 0, 0, 0, 0, 0];

    fn plain_aiff_comm(channels: u16, frames: u32, bits: u16) -> Vec<u8> {
        let mut d = Vec::new();
        d.extend_from_slice(&channels.to_be_bytes());
        d.extend_from_slice(&frames.to_be_bytes());
        d.extend_from_slice(&bits.to_be_bytes());
        d.extend_from_slice(&SRATE_44100_EXT80);
        d
    }

    #[test]
    fn parses_plain_aiff_comm_chunk() {
        let comm = plain_aiff_comm(1, 241_102, 16);
        let info = parse_aiff_comm(&comm, crate::aiff::AiffForm::Aiff).unwrap();
        assert_eq!(info.channels, 1);
        assert_eq!(info.sample_rate, 44100);
        assert_eq!(info.bits_per_sample, 16);
        assert_eq!(info.sample_type, SampleType::Short16);
        assert_eq!(info.block_align, 2);
    }

    #[test]
    fn plain_aiff_comm_of_wrong_size_is_an_error() {
        let mut comm = plain_aiff_comm(1, 100, 16);
        comm.push(0); // legacy: plain AIFF COMM must be exactly 18 bytes
        assert!(parse_aiff_comm(&comm, crate::aiff::AiffForm::Aiff).is_err());
    }

    #[test]
    fn plain_aiff_never_reports_float_even_at_32_bits() {
        // legacy: plain AIFF's COMM has no compression-type field, so
        // it is always PCM -- only AIFC can declare float samples.
        let comm = plain_aiff_comm(1, 100, 32);
        let info = parse_aiff_comm(&comm, crate::aiff::AiffForm::Aiff).unwrap();
        assert_eq!(info.sample_type, SampleType::Int32);
    }

    /// No AIFC file exists in this repository's corpus (all 120 real
    /// AIFF files under `docs/manual/sounds` are plain `AIFF`), so
    /// unlike `parses_plain_aiff_comm_chunk`, this is a hand-built
    /// fixture rather than bytes lifted from a real file. The
    /// `COMM` layout (fixed fields, then `compressionType`, then a
    /// Pascal string this parser does not need to read) is ported
    /// from `rdaifchdr` in `legacy/dev/newsfsys/sfsys.c`.
    #[test]
    fn parses_aifc_float_comm_chunk() {
        let mut comm = plain_aiff_comm(2, 1000, 32);
        comm.extend_from_slice(b"FL32");
        comm.extend_from_slice(&[4, b't', b'e', b's', b't']); // pascal string, ignored
        let info = parse_aiff_comm(&comm, crate::aiff::AiffForm::Aifc).unwrap();
        assert_eq!(info.sample_type, SampleType::Float32);
        assert_eq!(info.block_align, 8); // 4 bytes * 2 channels
    }

    /// legacy: "F***** Quicktime writes size = 16, for floats!" --
    /// `rdaifchdr` silently corrects a `FL32`-compressed COMM chunk
    /// that declares 16-bit samples to 32-bit, rather than erroring.
    #[test]
    fn aifc_float_quicktime_16bit_quirk_is_corrected_to_32() {
        let mut comm = plain_aiff_comm(1, 100, 16);
        comm.extend_from_slice(b"fl32");
        let info = parse_aiff_comm(&comm, crate::aiff::AiffForm::Aifc).unwrap();
        assert_eq!(info.bits_per_sample, 32);
        assert_eq!(info.sample_type, SampleType::Float32);
    }

    #[test]
    fn aifc_unknown_compression_type_is_an_error() {
        let mut comm = plain_aiff_comm(1, 100, 16);
        comm.extend_from_slice(b"ZZZZ");
        assert!(matches!(
            parse_aiff_comm(&comm, crate::aiff::AiffForm::Aifc),
            Err(SfError::UnknownAifcCompressionType)
        ));
    }

    #[test]
    fn peak_chunk_be_round_trip_matches_le_values() {
        let mut data = Vec::new();
        data.extend_from_slice(&1u32.to_be_bytes()); // version
        data.extend_from_slice(&1_151_268_522u32.to_be_bytes()); // timestamp
        data.extend_from_slice(&0.299_722_28f32.to_be_bytes());
        data.extend_from_slice(&29_914u32.to_be_bytes());
        let (ts, peaks) = parse_peak_chunk_be(&data, 1).unwrap();
        assert_eq!(ts, 1_151_268_522);
        assert_eq!(peaks[0].position, 29_914);
        assert!((peaks[0].value - 0.299_722_28).abs() < 1e-9);
    }

    fn float32_fmt(channels: u16) -> FmtInfo {
        FmtInfo {
            channels,
            sample_rate: 44100,
            bits_per_sample: 32,
            block_align: 4 * channels,
            sample_type: SampleType::Float32,
        }
    }

    #[test]
    fn non_float32_is_always_wave_even_with_analysis_properties_present() {
        // legacy: sf_headread's float32 branch is skipped entirely
        // for any other sample type, so props->type stays wt_wave.
        let mut fmt = float32_fmt(1);
        fmt.sample_type = SampleType::Short16;
        let mut props = PropertyBlock::new();
        props.set_i32("is a pitch file", 1);
        assert_eq!(detect_file_kind(&fmt, &props).unwrap(), FileKind::Wave);
    }

    #[test]
    fn float32_with_no_analysis_properties_is_plain_wave() {
        let props = PropertyBlock::new();
        assert_eq!(
            detect_file_kind(&float32_fmt(2), &props).unwrap(),
            FileKind::Wave
        );
    }

    /// `docs/manual/sounds/crklenv.evl`'s real properties (see
    /// `tests/oracle_fixtures.rs` for the byte-for-byte version):
    /// `is an envelope` (any nonzero value) plus `window size`.
    #[test]
    fn envelope_marker_with_window_size() {
        let mut props = PropertyBlock::new();
        props.set_i32("is an envelope", 1);
        props.set_f32("window size", 2.902_494);
        let kind = detect_file_kind(&float32_fmt(1), &props).unwrap();
        assert_eq!(
            kind,
            FileKind::Envelope {
                window_size: 2.902_494
            }
        );
    }

    #[test]
    fn envelope_marker_without_window_size_is_an_error() {
        let mut props = PropertyBlock::new();
        props.set_i32("is an envelope", 1);
        assert!(matches!(
            detect_file_kind(&float32_fmt(1), &props),
            Err(SfError::MissingEnvelopeWindowSize)
        ));
    }

    fn spectral_props() -> PropertyBlock {
        let mut props = PropertyBlock::new();
        props.set_i32("original sampsize", 0);
        props.set_i32("original sample rate", 44100);
        props.set_f32("arate", 344.53125);
        props.set_i32("analwinlen", 1024);
        props.set_i32("decfactor", 128);
        props
    }

    /// `docs/manual/data/capm.ana`'s real property values (see
    /// `tests/oracle_fixtures.rs`): no marker property present, so
    /// this is the `wt_analysis` default.
    #[test]
    fn spectral_properties_with_no_marker_is_analysis() {
        let props = spectral_props();
        let kind = detect_file_kind(&float32_fmt(1026), &props).unwrap();
        assert_eq!(
            kind,
            FileKind::Analysis(SpectralInfo {
                original_sample_size: 0,
                original_sample_rate: 44100,
                analysis_rate: 344.53125,
                analysis_window_length: 1024,
                decimation_factor: 128,
            })
        );
    }

    /// `docs/manual/data/crklptrace.frq`'s real properties.
    #[test]
    fn pitch_marker_with_orig_channels() {
        let mut props = spectral_props();
        props.set_i32("is a pitch file", 1);
        props.set_i32("orig channels", 1026);
        let kind = detect_file_kind(&float32_fmt(1), &props).unwrap();
        assert!(matches!(kind, FileKind::Pitch(info) if info.original_channels == 1026));
    }

    /// `docs/manual/data/ssbcrkl.trn`'s real properties.
    #[test]
    fn transposition_marker_with_orig_channels() {
        let mut props = spectral_props();
        props.set_i32("is a transpos file", 1);
        props.set_i32("orig channels", 1026);
        let kind = detect_file_kind(&float32_fmt(1), &props).unwrap();
        assert!(matches!(kind, FileKind::Transposition(info) if info.original_channels == 1026));
    }

    /// No formant file exists in this repository's corpus, so unlike
    /// the pitch/transposition/analysis cases above, this is a
    /// hand-built fixture (see the module doc's note on AIFC for the
    /// same caveat).
    #[test]
    fn formant_marker_with_spectral_envelope_count() {
        let mut props = spectral_props();
        props.set_i32("is a formant file", 1);
        props.set_i32("orig channels", 1026);
        props.set_i32("specenvcnt", 80);
        let kind = detect_file_kind(&float32_fmt(1), &props).unwrap();
        assert!(matches!(
            kind,
            FileKind::Formant { channel_info, spectral_envelope_count: 80 }
                if channel_info.original_channels == 1026
        ));
    }

    /// legacy: pitch, in this order, wins over transposition and
    /// formant if (illegally) more than one marker is present.
    #[test]
    fn pitch_marker_takes_precedence_when_multiple_markers_present() {
        let mut props = spectral_props();
        props.set_i32("is a pitch file", 1);
        props.set_i32("is a transpos file", 1);
        props.set_i32("orig channels", 1);
        let kind = detect_file_kind(&float32_fmt(1), &props).unwrap();
        assert!(matches!(kind, FileKind::Pitch(_)));
    }

    #[test]
    fn pitch_file_with_more_than_one_channel_is_an_error() {
        let mut props = spectral_props();
        props.set_i32("is a pitch file", 1);
        props.set_i32("orig channels", 1);
        assert!(matches!(
            detect_file_kind(&float32_fmt(2), &props),
            Err(SfError::AnalysisFileChannelCountNotOne)
        ));
    }

    #[test]
    fn pitch_file_missing_orig_channels_is_an_error() {
        let mut props = spectral_props();
        props.set_i32("is a pitch file", 1);
        assert!(matches!(
            detect_file_kind(&float32_fmt(1), &props),
            Err(SfError::MissingOriginalChannels)
        ));
    }

    #[test]
    fn formant_file_missing_specenvcnt_is_an_error() {
        let mut props = spectral_props();
        props.set_i32("is a formant file", 1);
        props.set_i32("orig channels", 1);
        assert!(matches!(
            detect_file_kind(&float32_fmt(1), &props),
            Err(SfError::MissingSpectralEnvelopeCount)
        ));
    }

    #[test]
    fn partially_present_analysis_properties_is_an_error() {
        // Only "arate" set (nonzero, so checksum != 0), the other
        // four genuinely missing rather than present-and-zero.
        let mut props = PropertyBlock::new();
        props.set_f32("arate", 344.53125);
        assert!(matches!(
            detect_file_kind(&float32_fmt(1), &props),
            Err(SfError::InconsistentAnalysisProperties)
        ));
    }
}
