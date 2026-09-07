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

//! Writing a WAVE sound file. legacy: `legacy/dev/newsfsys/sfsys.c`
//! (`wrwavhdr98`, header writing), `snd.c` (`fputshortEx`, the
//! float-to-sample conversion).
//!
//! Scope of this version: writes `fmt `+`data`(+`PEAK`)(+`cue
//! `+`LIST`/`adtl`/`note`/`sfif`) WAVE files in 16-bit PCM or 32-bit
//! float. It does not yet write AIFF/AIFC or WAVE_FORMAT_EXTENSIBLE /
//! multichannel formats beyond plain PCM. Tracked in
//! docs/migration/STATUS.md.

use crate::error::{Result, SfError};
use crate::props::{ChannelPeak, PROPCNKSIZE, PropertyBlock, SampleType};
use crate::reader::MAXSHORT;
use crate::riff;
use std::path::Path;

const WAVE_FORMAT_PCM: u16 = 0x0001;
const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;

/// What to create a [`SoundFileWriter`] with. legacy: the subset of
/// `SFPROPS` a writer needs before the first sample is written.
#[derive(Debug, Clone)]
pub struct WriteSpec {
    pub channels: u16,
    pub sample_rate: u32,
    pub sample_type: SampleType,
    /// Whether to write a `PEAK` chunk on finalize, computed from the
    /// samples actually written. legacy: gated on `min_header >=
    /// SFILE_PEAKONLY`; this crate always computes accurate peaks
    /// when true, rather than the legacy zero-then-defer-to-later
    /// behaviour used only for streaming writes that reopen the file.
    pub write_peaks: bool,
    /// The named properties to write as a `cue `(optional)+`LIST`/
    /// `adtl`/`note`/`sfif` chunk group, or an empty
    /// [`PropertyBlock`] to skip writing any of it (legacy:
    /// `min_header < SFILE_CDP`).
    pub properties: PropertyBlock,
    /// Whether to also write the `cue ` chunk that normally precedes
    /// the property `LIST` (legacy: `f->min_header==SFILE_CDP`,
    /// confirmed by `docs/manual/sounds/marimba.wav` having one).
    /// Meaningless when `properties` is empty. Real CDP-derived
    /// analysis files (`docs/manual/data/capm.ana`) have properties
    /// but no `cue ` chunk (legacy comment: "don't need cue for
    /// analysis files"), which is why this is the caller's choice
    /// rather than inferred from `sample_type`.
    pub write_cue_chunk: bool,
}

/// Accumulates `f32` sample frames in memory and writes a complete
/// WAVE file on [`SoundFileWriter::finalize`]. See the module doc for
/// what is not implemented yet. Buffering the whole file in memory
/// before writing (rather than streaming with a header backpatch, as
/// the legacy writer does) is a deliberate simplification for this
/// first version; revisit if a program needs to write files too
/// large to buffer.
pub struct SoundFileWriter {
    spec: WriteSpec,
    samples: Vec<f32>,
    peak_running: Vec<ChannelPeak>,
}

impl SoundFileWriter {
    pub fn new(spec: WriteSpec) -> Self {
        let peak_running = vec![
            ChannelPeak {
                value: 0.0,
                position: 0,
            };
            spec.channels as usize
        ];
        SoundFileWriter {
            spec,
            samples: Vec::new(),
            peak_running,
        }
    }

    /// Appends interleaved sample frames. `frames.len()` must be a
    /// multiple of `spec.channels`.
    pub fn write_frames(&mut self, frames: &[f32]) {
        let channels = self.spec.channels as usize;
        debug_assert_eq!(
            frames.len() % channels,
            0,
            "write_frames: frame data not a whole number of channel-frames"
        );
        let base_frame = (self.samples.len() / channels) as u32;
        for (i, &s) in frames.iter().enumerate() {
            let ch = i % channels;
            let mag = s.abs();
            // legacy: CHPEAK stores the first position a channel's
            // peak absolute value occurred at (see
            // `reset_peak_finder`/PEAK-tracking in
            // `legacy/dev/cdp2k/mainfuncs.c`); a later sample with the
            // same magnitude does not move the position.
            if mag > self.peak_running[ch].value {
                self.peak_running[ch].value = mag;
                self.peak_running[ch].position = base_frame + (i / channels) as u32;
            }
        }
        self.samples.extend_from_slice(frames);
    }

    pub fn finalize(self, path: impl AsRef<Path>) -> Result<()> {
        let bytes = self.encode()?;
        std::fs::write(path, bytes)?;
        Ok(())
    }

    /// Encodes the complete file into memory. Exposed separately from
    /// [`Self::finalize`] so tests (and, later, in-process
    /// round-tripping without a temp file) do not need the
    /// filesystem.
    pub fn encode(&self) -> Result<Vec<u8>> {
        let channels = self.spec.channels;
        let (bits_per_sample, format_tag, data_bytes) = match self.spec.sample_type {
            SampleType::Short16 => (16u16, WAVE_FORMAT_PCM, encode_pcm16(&self.samples)),
            SampleType::Float32 => (32u16, WAVE_FORMAT_IEEE_FLOAT, encode_float32(&self.samples)),
            other => {
                return Err(SfError::UnsupportedSampleDataDecoding(other));
            }
        };
        let block_align = channels * (bits_per_sample / 8);
        let byte_rate = self.spec.sample_rate * block_align as u32;

        let mut fmt = Vec::with_capacity(16);
        fmt.extend_from_slice(&format_tag.to_le_bytes());
        fmt.extend_from_slice(&channels.to_le_bytes());
        fmt.extend_from_slice(&self.spec.sample_rate.to_le_bytes());
        fmt.extend_from_slice(&byte_rate.to_le_bytes());
        fmt.extend_from_slice(&block_align.to_le_bytes());
        fmt.extend_from_slice(&bits_per_sample.to_le_bytes());

        let mut body = Vec::new();
        body.extend_from_slice(&riff::WAVE.0);
        riff::write_chunk(&mut body, riff::FMT_, &fmt)?;
        if self.spec.write_peaks {
            let peak_data =
                crate::props::encode_peak_chunk(now_unix_timestamp(), &self.peak_running);
            riff::write_chunk(&mut body, riff::PEAK, &peak_data)?;
        }
        if !self.spec.properties.is_empty() {
            if self.spec.write_cue_chunk {
                riff::write_chunk(&mut body, riff::CUE_, &encode_cue_chunk())?;
            }
            let list_data = encode_property_list_chunk(&self.spec.properties)?;
            riff::write_chunk(&mut body, riff::LIST, &list_data)?;
        }
        riff::write_chunk(&mut body, riff::DATA, &data_bytes)?;

        let mut out = Vec::with_capacity(8 + body.len());
        out.extend_from_slice(&riff::RIFF.0);
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        Ok(out)
    }
}

fn encode_pcm16(samples: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(samples.len() * 2);
    for &s in samples {
        // legacy: fputshortEx computes
        //   (short) cdp_round(sample * MAXSHORT)
        // where cdp_round is lround (round-half-away-from-zero) and
        // the (short) cast truncates on overflow rather than
        // saturating. This does not clip: a sample outside
        // -1.0..=1.0 wraps here exactly as the legacy (short) cast
        // does (both this cast and `as i16` on an out-of-range i32
        // take the low 16 bits). Callers that need clipping, e.g. the
        // legacy `gain` process's clip-before-write behaviour, must
        // do it before calling write_frames, matching where the
        // legacy code does it.
        let v = (s * MAXSHORT).round() as i32;
        let wrapped = v as i16;
        out.extend_from_slice(&wrapped.to_le_bytes());
    }
    out
}

fn encode_float32(samples: &[f32]) -> Vec<u8> {
    let mut out = Vec::with_capacity(samples.len() * 4);
    for &s in samples {
        out.extend_from_slice(&s.to_le_bytes());
    }
    out
}

/// legacy: the single fixed cue point `wrwavhdr98` writes ahead of
/// the property `LIST` -- `struct cuepoint {name, position,
/// incchunkid, chunkoffset, blockstart, sampleoffset}`, always
/// pointing at the `sfif` property (by name) inside the `data` chunk
/// (by `incchunkid`), at offset zero. Confirmed byte-for-byte against
/// `docs/manual/sounds/marimba.wav`'s `cue ` chunk.
fn encode_cue_chunk() -> Vec<u8> {
    let mut out = Vec::with_capacity(28);
    out.extend_from_slice(&1u32.to_le_bytes()); // legacy: one cue point
    out.extend_from_slice(&riff::SFIF.0); // cue.name
    out.extend_from_slice(&0u32.to_le_bytes()); // cue.position
    out.extend_from_slice(&riff::DATA.0); // cue.incchunkid
    out.extend_from_slice(&0u32.to_le_bytes()); // cue.chunkoffset
    out.extend_from_slice(&0u32.to_le_bytes()); // cue.blockstart
    out.extend_from_slice(&0u32.to_le_bytes()); // cue.sampleoffset
    out
}

/// legacy: `wrwavhdr98`'s `LIST`("adtl")/`note`("sfif") chunk group,
/// containing [`PropertyBlock::encode_padded`]'s `PROPCNKSIZE`-byte
/// text. Returns the `LIST` chunk's payload (i.e. what
/// `riff::write_chunk` should wrap with the `LIST` tag), not the
/// `LIST` chunk itself.
fn encode_property_list_chunk(properties: &PropertyBlock) -> Result<Vec<u8>> {
    let padded = properties.encode_padded(PROPCNKSIZE)?;

    let mut note_chunk = Vec::with_capacity(8 + 4 + padded.len());
    note_chunk.extend_from_slice(&riff::NOTE.0);
    note_chunk.extend_from_slice(&((4 + padded.len()) as u32).to_le_bytes());
    note_chunk.extend_from_slice(&riff::SFIF.0);
    note_chunk.extend_from_slice(&padded);

    let mut list_data = Vec::with_capacity(4 + note_chunk.len());
    list_data.extend_from_slice(&riff::ADTL.0);
    list_data.extend_from_slice(&note_chunk);
    Ok(list_data)
}

fn now_unix_timestamp() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as u32)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reader::SoundFile;

    fn make_sine(channels: u16, frames: usize, freq: f32, sample_rate: u32) -> Vec<f32> {
        let mut out = Vec::with_capacity(frames * channels as usize);
        for n in 0..frames {
            let t = n as f32 / sample_rate as f32;
            let s = (2.0 * std::f32::consts::PI * freq * t).sin();
            for _ in 0..channels {
                out.push(s);
            }
        }
        out
    }

    #[test]
    fn pcm16_round_trip_preserves_format_and_sample_count() {
        let spec = WriteSpec {
            channels: 1,
            sample_rate: 44100,
            sample_type: SampleType::Short16,
            write_peaks: true,
            properties: PropertyBlock::new(),
            write_cue_chunk: false,
        };
        let mut w = SoundFileWriter::new(spec);
        let sine = make_sine(1, 44100, 440.0, 44100);
        w.write_frames(&sine);
        let bytes = w.encode().unwrap();

        let sf = SoundFile::from_reader(std::io::Cursor::new(bytes)).unwrap();
        assert_eq!(sf.fmt.channels, 1);
        assert_eq!(sf.fmt.sample_rate, 44100);
        assert_eq!(sf.fmt.sample_type, SampleType::Short16);
        assert_eq!(sf.frame_count(), 44100);

        // 16-bit quantisation error bound: half a quantum, 1/32767.
        let decoded = sf.samples_f32().unwrap();
        for (original, round_tripped) in sine.iter().zip(decoded.iter()) {
            assert!((original - round_tripped).abs() < 1.0 / 32767.0 + 1e-6);
        }
    }

    #[test]
    fn float32_round_trip_is_bit_exact() {
        let spec = WriteSpec {
            channels: 2,
            sample_rate: 48000,
            sample_type: SampleType::Float32,
            write_peaks: false,
            properties: PropertyBlock::new(),
            write_cue_chunk: false,
        };
        let mut w = SoundFileWriter::new(spec);
        let sine = make_sine(2, 1000, 220.0, 48000);
        w.write_frames(&sine);
        let bytes = w.encode().unwrap();

        let sf = SoundFile::from_reader(std::io::Cursor::new(bytes)).unwrap();
        assert_eq!(sf.fmt.sample_type, SampleType::Float32);
        assert!(sf.peaks.is_empty(), "write_peaks was false");
        let decoded = sf.samples_f32().unwrap();
        assert_eq!(
            decoded, sine,
            "float32 is not lossy: round trip must be bit-exact"
        );
    }

    #[test]
    fn peak_tracks_first_occurrence_of_the_loudest_sample() {
        // legacy: CHPEAK records the position of the FIRST sample
        // that reaches the channel's peak magnitude; an equal later
        // sample must not move it.
        let spec = WriteSpec {
            channels: 1,
            sample_rate: 44100,
            sample_type: SampleType::Float32,
            write_peaks: true,
            properties: PropertyBlock::new(),
            write_cue_chunk: false,
        };
        let mut w = SoundFileWriter::new(spec);
        w.write_frames(&[0.1, 0.9, 0.2, 0.9, 0.3]);
        assert_eq!(w.peak_running[0].position, 1);
        assert_eq!(w.peak_running[0].value, 0.9);
    }

    #[test]
    fn peak_tracking_continues_correctly_across_multiple_write_calls() {
        let spec = WriteSpec {
            channels: 2,
            sample_rate: 44100,
            sample_type: SampleType::Float32,
            write_peaks: true,
            properties: PropertyBlock::new(),
            write_cue_chunk: false,
        };
        let mut w = SoundFileWriter::new(spec);
        w.write_frames(&[0.1, 0.2, 0.3, 0.4]); // frames 0,1
        w.write_frames(&[0.05, 0.9, 0.1, 0.1]); // frames 2,3: ch1 peaks at frame 2
        assert_eq!(w.peak_running[0].value, 0.3); // ch0 max stays at frame 1
        assert_eq!(w.peak_running[0].position, 1);
        assert_eq!(w.peak_running[1].value, 0.9); // ch1 max moves to frame 2
        assert_eq!(w.peak_running[1].position, 2);
    }

    #[test]
    fn pcm16_write_truncates_on_overflow_like_the_legacy_short_cast() {
        // legacy: fputshortEx computes (short) cdp_round(sample *
        // MAXSHORT), a truncating cast on overflow, not a saturating
        // one. 2.0 * 32767 = 65534, which as an i16 wraps to -2.
        let encoded = encode_pcm16(&[2.0]);
        let v = i16::from_le_bytes([encoded[0], encoded[1]]);
        assert_eq!(v, 65534i32 as i16);
    }

    fn find_chunk_offsets(bytes: &[u8]) -> Vec<(String, usize)> {
        // Walks top-level RIFF chunks (skipping "RIFF"+size+"WAVE"),
        // returning each chunk's tag and the file offset its tag
        // starts at, in order -- used to check chunk sequencing
        // without depending on `crate::riff`'s own chunk reader.
        let mut out = Vec::new();
        let mut pos = 12usize;
        while pos + 8 <= bytes.len() {
            let tag = String::from_utf8_lossy(&bytes[pos..pos + 4]).into_owned();
            let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
            out.push((tag, pos));
            pos += 8 + size + (size % 2);
        }
        out
    }

    /// legacy: `wrwavhdr98` writes `cue ` immediately before the
    /// property `LIST`, matching the exact chunk sequence in the real
    /// `docs/manual/sounds/marimba.wav` (`fmt `, `cue `, `LIST`,
    /// `data`, no `PEAK` in that particular file, but this crate
    /// always places `PEAK` -- when present -- before the property
    /// group, per legacy's write order).
    #[test]
    fn writing_properties_with_cue_chunk_matches_marimba_wav_chunk_order() {
        let mut properties = PropertyBlock::new();
        properties.set_i32("DATE", 963_996_604);
        let spec = WriteSpec {
            channels: 1,
            sample_rate: 44100,
            sample_type: SampleType::Short16,
            write_peaks: true,
            properties,
            write_cue_chunk: true,
        };
        let mut w = SoundFileWriter::new(spec);
        w.write_frames(&[0.1, 0.2, 0.3]);
        let bytes = w.encode().unwrap();

        let tags: Vec<String> = find_chunk_offsets(&bytes)
            .into_iter()
            .map(|(t, _)| t)
            .collect();
        assert_eq!(tags, vec!["fmt ", "PEAK", "cue ", "LIST", "data"]);

        let sf = SoundFile::from_reader(std::io::Cursor::new(bytes)).unwrap();
        assert_eq!(sf.properties.get_i32("DATE").unwrap(), 963_996_604);
    }

    /// legacy: real CDP-derived analysis files (`docs/manual/data/
    /// capm.ana`) go straight from `fmt ` to `LIST`, with no `cue `
    /// chunk at all ("don't need cue for analysis files").
    #[test]
    fn writing_properties_without_cue_chunk_matches_capm_ana_chunk_order() {
        let mut properties = PropertyBlock::new();
        properties.set_i32("original sampsize", 0);
        properties.set_i32("original sample rate", 44100);
        properties.set_f32("arate", 344.53125);
        properties.set_i32("analwinlen", 1024);
        properties.set_i32("decfactor", 128);
        let spec = WriteSpec {
            channels: 2,
            sample_rate: 344,
            sample_type: SampleType::Float32,
            write_peaks: false,
            properties,
            write_cue_chunk: false,
        };
        let mut w = SoundFileWriter::new(spec);
        w.write_frames(&[0.0, 0.0]);
        let bytes = w.encode().unwrap();

        let tags: Vec<String> = find_chunk_offsets(&bytes)
            .into_iter()
            .map(|(t, _)| t)
            .collect();
        assert_eq!(tags, vec!["fmt ", "LIST", "data"]);

        let sf = SoundFile::from_reader(std::io::Cursor::new(bytes)).unwrap();
        assert!(matches!(sf.file_kind, crate::props::FileKind::Analysis(_)));
    }

    /// legacy: `writeprops`'s padding loop, `while(op <
    /// &obuf[f->proplim]) *op++ = '\n';` -- confirmed against the real
    /// padding bytes in `docs/manual/sounds/marimba.wav`'s `note`
    /// chunk (`0x0A` repeating, not zero; see
    /// `PropertyBlock::encode_padded`'s doc).
    #[test]
    fn property_block_is_padded_to_propcnksize_with_newlines() {
        let mut properties = PropertyBlock::new();
        properties.set_i32("DATE", 1);
        let list_data = encode_property_list_chunk(&properties).unwrap();
        // list_data = "adtl" (4) + "note" (4) + size (4) + "sfif" (4) + padded properties
        let padded = &list_data[16..];
        assert_eq!(padded.len(), PROPCNKSIZE);
        assert_eq!(padded[padded.len() - 1], b'\n');
        assert!(padded[padded.len() - 100..].iter().all(|&b| b == b'\n'));
    }

    #[test]
    fn oversized_property_block_is_an_error_not_a_panic() {
        let mut properties = PropertyBlock::new();
        for n in 0..200 {
            properties.set_i32(&format!("property_number_{n}"), n);
        }
        assert!(
            properties.encode().len() > PROPCNKSIZE,
            "test fixture must actually exceed PROPCNKSIZE"
        );
        let spec = WriteSpec {
            channels: 1,
            sample_rate: 44100,
            sample_type: SampleType::Short16,
            write_peaks: false,
            properties,
            write_cue_chunk: false,
        };
        let w = SoundFileWriter::new(spec);
        assert!(matches!(
            w.encode(),
            Err(SfError::PropertyBlockTooLarge { .. })
        ));
    }
}
