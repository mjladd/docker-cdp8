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

//! `sndinfo` (legacy: `legacy/dev/sndinfo`). The `props`, `len`,
//! `smptime`, `timesmp` and `timediff` sub-commands are ported so far.

mod ctime;
pub mod len;
pub mod props;
pub mod smptime;
pub mod timediff;
pub mod timesmp;

use cdp_core::{CdpError, ExitCategory};
use cdp_sf::{FileKind, SoundFile};

/// legacy: `test_application_validity`'s generic infile-kind check
/// (`legacy/dev/cdp2k/mainfuncs.c`), confirmed live for `sndinfo
/// smptime`/`sndinfo timesmp` against `docs/manual/data/capm.ana` --
/// see [`smptime`]'s module doc. Also the check `timediff`'s infile1
/// uses (`INFO_TIMEDIFF` is `TWO_SNDFILES`, a different
/// `input_data_type` from `SNDFILES_ONLY`, but both route through this
/// same generic per-filetype validity table before either command's
/// own logic runs) -- see [`timediff`]'s module doc for infile2's own,
/// different check.
pub const WRONG_FILETYPE: &str = "Application doesn't work with this type of infile.\n";

/// Opens `path` as a [`SoundFile`], reproducing the same
/// `"Can't open file %s to read data."` (`DATA_ERROR`) text
/// `cdp_params::parser::parse`'s own infile-existence check would
/// produce, then rejects anything other than [`FileKind::Wave`] with
/// [`WRONG_FILETYPE`] -- both `smptime` and `timesmp` are
/// `SNDFILES_ONLY` in `ap_sndinfo.c`'s `setup_process_logic`, unlike
/// `props`/`len`'s `ALL_FILES`.
///
/// Needed because [`cdp_params::CommandSpec::sndinfo_smptime`]/
/// [`cdp_params::CommandSpec::sndinfo_timesmp`]'s dynamic range bound
/// requires the infile to already be open before
/// [`cdp_params::parse`] itself can run, unlike every other command in
/// this crate so far -- this duplicates
/// [`cdp_params::ParamsError::CannotOpenFile`]'s own check (not
/// exported from that crate) rather than calling `parse` twice with a
/// placeholder range. Reused by [`timediff::open_infiles`] for its
/// infile1 for the same reason `timediff` bypasses `cdp_params::parse`
/// entirely -- see that module's doc.
pub fn open_sound_infile(path: &str) -> Result<SoundFile, CdpError> {
    std::fs::File::open(path).map_err(|source| {
        CdpError::from(cdp_params::ParamsError::CannotOpenFile {
            path: path.to_string(),
            source,
        })
    })?;
    let sf = SoundFile::open(path).map_err(CdpError::from)?;
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(ExitCategory::UsageOnly, WRONG_FILETYPE));
    }
    Ok(sf)
}
