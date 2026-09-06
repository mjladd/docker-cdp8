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

//! Error type for `cdp-sf`.
//!
//! Message text mirrors the corresponding legacy error where one
//! exists, so golden tests comparing stderr text against
//! `legacy/dev/sndinfo` and friends can match it. See
//! `legacy/dev/newsfsys/sfsys.c` for the originals (`rsferrstr`
//! assignments) and `legacy/dev/newinclude/sfsys.h` for the `ESF*`
//! error codes this crate's variants correspond to.

use std::io;

#[derive(Debug, thiserror::Error)]
pub enum SfError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// legacy: ESFBADMAG / "not a valid RIFF file"
    #[error("not a recognised sound file: missing or invalid RIFF header")]
    NotRiff,

    /// legacy: "not a WAVE file" checks in sf_headread / rdwavhdr
    #[error("not a WAVE file: RIFF form type is not \"WAVE\"")]
    NotWave,

    #[error("missing required chunk: {0:?}")]
    MissingChunk(FourCc),

    #[error("truncated or malformed chunk: {0:?}")]
    MalformedChunk(FourCc),

    /// legacy: props.c sf_headread, "unrecognised integer sample format"
    #[error("unrecognised sample format (wFormatTag={format_tag}, bits={bits_per_sample})")]
    UnsupportedSampleFormat {
        format_tag: u16,
        bits_per_sample: u16,
    },

    /// This crate does not yet decode this sample type's data (props
    /// can still be read). See docs/migration/STATUS.md for what
    /// `cdp-sf` currently supports.
    #[error("sample data decoding not yet implemented for {0:?}")]
    UnsupportedSampleDataDecoding(crate::SampleType),

    #[error("property not defined in file: {0:?}")]
    PropertyNotFound(String),

    #[error("property {name:?} has the wrong size: expected {expected}, found {found}")]
    PropertySize {
        name: String,
        expected: usize,
        found: usize,
    },
}

pub type Result<T> = std::result::Result<T, SfError>;

/// A 4-byte RIFF chunk identifier ("RIFF", "fmt ", "data", "PEAK", ...),
/// kept as raw bytes rather than a `String` since it is not always
/// valid UTF-8 and never needs locale-aware comparison.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FourCc(pub [u8; 4]);

impl FourCc {
    pub const fn new(tag: &[u8; 4]) -> Self {
        FourCc(*tag)
    }
}

impl std::fmt::Debug for FourCc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.0) {
            Ok(s) => write!(f, "{s:?}"),
            Err(_) => write!(f, "{:?}", self.0),
        }
    }
}
