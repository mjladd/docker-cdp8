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

//! `housekeep` (legacy: `legacy/dev/houskeep`, WP-2.2). `copy` mode 1
//! and `chans` mode 1 are ported so far, of the thirteen sub-commands
//! `get_process_no` (`legacy/dev/houskeep/ap_house.c`) recognises:
//! `copy`, `remove`, `chans`, `bundle`, `sort`, `respec`, `extract`,
//! `bakup`, `gate`, `disk`, `batchexpand`, `endclicks`, `deglitch`.

pub mod chans;
pub mod copy;

use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{PropertyBlock, SampleType, SoundFileWriter, WriteSpec};

/// legacy: `Cannot open output file %s\n` (`DATA_ERROR`) -- the
/// refusal-to-overwrite check every `SNDFILE_OUT` program's
/// `create_sized_outfile` shares (`legacy/dev/cdp2k/mainfuncs.c`),
/// confirmed live for both `copy` and `chans`. `cdp_sf::
/// SoundFileWriter::finalize` does not check this itself (it always
/// overwrites via `std::fs::write`), so every command in this module
/// checks it directly, before reading any sample data, matching
/// legacy's own order (`create_sized_outfile` runs before the
/// sample-copying loop begins). This same check likely applies to
/// `synth wave`/`pvoc anal` too, neither of which currently enforces
/// it -- a real, pre-existing gap `copy`'s own slice found but did
/// not fix, since those two commands belong to WP-1.4/WP-1.5, not
/// this one.
pub(crate) fn check_outfile_does_not_exist(path: &str) -> Result<(), CdpError> {
    if std::path::Path::new(path).exists() {
        return Err(CdpError::new(
            ExitCategory::DataError,
            format!("Cannot open output file {path}\n"),
        ));
    }
    Ok(())
}

/// legacy: the fresh `DATE` property every `SNDFILE_OUT` program's
/// output gains, encoded the same way `cdp_sf::SoundFileWriter`'s own
/// `PEAK`-chunk timestamp already is. Not a fixed value: every live
/// comparison this crate's tests run has to compare everything else
/// byte-for-byte and only sanity-check that `DATE` decodes to a
/// plausible, current timestamp, since legacy's own output produces a
/// different exact `DATE` on every run too.
pub(crate) fn now_unix_timestamp() -> i32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i32)
        .unwrap_or(0)
}

/// Writes `samples` (already-decoded, interleaved `f32` frames) as a
/// complete `WAVE` file at `outfile_path`, with a freshly-computed
/// `PEAK` chunk and a fresh `DATE` property -- the shape every
/// `HOUSE_COPY`/`HOUSE_CHANS` output shares (see `copy`'s own module
/// doc for what is confirmed live about this shape: always `WAVE`
/// regardless of the infile's own container format, and every other
/// named property from the infile is dropped). Callers must call
/// [`check_outfile_does_not_exist`] themselves first, before reading
/// any sample data, to match legacy's own ordering -- this function
/// does not check it again.
pub(crate) fn write_wave_file(
    channels: u16,
    sample_rate: u32,
    sample_type: SampleType,
    samples: &[f32],
    outfile_path: &str,
) -> Result<(), CdpError> {
    let mut properties = PropertyBlock::new();
    properties.set_i32("DATE", now_unix_timestamp());
    let mut writer = SoundFileWriter::new(WriteSpec {
        channels,
        sample_rate,
        sample_type,
        write_peaks: true,
        properties,
        write_cue_chunk: true,
    });
    writer.write_frames(samples);
    writer.finalize(outfile_path).map_err(CdpError::from)
}

/// legacy: the usage text `legacy/dev/houskeep/main.c` prints for a
/// bare `housekeep` (captured live via `spec/usage/housekeep.txt`,
/// WP-0.2; re-confirmed live this slice). One fewer trailing blank
/// line than the raw capture, same reason as every `USAGE` constant
/// elsewhere in this crate -- `cdp_core::ExitCategory::UsageOnly`
/// already appends its own tail blank line.
pub const USAGE: &str = "CDP Release 7.1 2016
USAGE: housekeep NAME (mode) infile(s) (outfile) (parameters)

where NAME can be any one of

extract  copy   remove   chans   respec   bundle   sort   disk
bakup    gate    batchexpand    endclicks    deglitch

Type 'housekeep chans'  for more info on housekeep chans option... ETC.
";
