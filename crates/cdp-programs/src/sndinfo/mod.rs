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
//! `smptime`, `timesmp`, `timediff`, `lens`, `sumlen`, `maxsamp` and
//! (modes 1/2 only) `units` sub-commands are ported so far.

mod ctime;
pub mod len;
pub mod lens;
pub mod maxsamp;
pub mod props;
pub mod smptime;
pub mod sumlen;
pub mod timediff;
pub mod timesmp;
pub mod units;

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

/// legacy: `"Insufficient input files for this process\n"`
/// (`legacy/dev/cdp2k/mainfuncs.c`'s `count_and_allocate_for_infiles`,
/// `MANY_SNDFILES`'s own `dz->infilecnt < 2` check) -- confirmed live
/// for both `sndinfo lens infile -x` and `sndinfo sumlen infile -x5`
/// (one real infile, one token that does not count as an infile
/// because it looks like a flag). Distinct from the plain
/// [`cdp_params::ParamsError::InsufficientParameters`] text a bare
/// one-token invocation (`sndinfo lens infile`) produces instead --
/// see [`lens`]'s module doc for why those two cases are not the
/// same check.
pub const INSUFFICIENT_INFILES: &str = "Insufficient input files for this process\n";

/// Opens infile 2 or later of a multi-infile command (legacy:
/// `handle_other_infile`/`open_checktype_getsize_and_compareheader`,
/// `legacy/dev/cdp2k/readfiles.c`), whose open-failure and wrong-kind
/// messages are both worded differently from infile 1's own
/// ([`open_sound_infile`]) -- see [`timediff::open_infiles`]'s module
/// doc, which established this same distinction for a fixed two-infile
/// command; [`lens`] and [`sumlen`] reuse it for an unbounded list.
pub(crate) fn open_other_sound_infile(path: &str) -> Result<SoundFile, CdpError> {
    std::fs::File::open(path).map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("cannot open input file {path} to read data."),
        )
    })?;
    let sf = SoundFile::open(path).map_err(CdpError::from)?;
    if !matches!(sf.file_kind, FileKind::Wave) {
        return Err(CdpError::new(
            ExitCategory::DataError,
            format!("{path} is not a sound file."),
        ));
    }
    Ok(sf)
}

/// legacy: `count_infiles`'s own unflagged-item test
/// (`legacy/dev/cdp2k/mainfuncs.c`): `*(argv[0])!='-' ||
/// !isalpha(argv[0][1])`. A token counts as flag-shaped (excluded from
/// the infile count) only when it starts with `-` *and* its second
/// character is alphabetic -- confirmed live a bare `-` is *not*
/// flag-shaped by this test: `sndinfo lens infile infile2 -` treats
/// the `-` as a third infile (whose name is literally `-`), not a
/// flag, and fails with the ordinary
/// [`open_other_sound_infile`]-style "cannot open input file - to
/// read data.", not a flag-related error. `sndinfo lens infile infile2
/// -x`, by contrast, correctly stops at `-x` (`x` is alphabetic).
fn looks_like_flag(token: &str) -> bool {
    let bytes = token.as_bytes();
    bytes.first() == Some(&b'-') && bytes.get(1).is_some_and(u8::is_ascii_alphabetic)
}

/// Splits `args` into the leading run of infile-shaped tokens (every
/// token up to but not including the first one [`looks_like_flag`]
/// considers flag-shaped) and whatever remains -- shared by [`lens`]
/// and [`sumlen`], the two `MANY_SNDFILES` commands ported so far. See
/// [`lens`]'s module doc for why this is a deliberate simplification
/// of legacy's own `count_infiles`, not a full reproduction of it (it
/// does not model infiles and flags genuinely interspersed).
pub fn split_infile_tokens<'a>(args: &'a [&'a str]) -> (&'a [&'a str], &'a [&'a str]) {
    let n = args
        .iter()
        .take_while(|token| !looks_like_flag(token))
        .count();
    args.split_at(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_infile_tokens_stops_at_the_first_flag_shaped_token() {
        let args = ["a.wav", "b.wav", "-x"];
        let (infiles, rest) = split_infile_tokens(&args);
        assert_eq!(infiles, ["a.wav", "b.wav"]);
        assert_eq!(rest, ["-x"]);
    }

    #[test]
    fn split_infile_tokens_takes_everything_when_no_flag_is_present() {
        let args = ["a.wav", "b.wav", "c.wav"];
        let (infiles, rest) = split_infile_tokens(&args);
        assert_eq!(infiles, args);
        assert!(rest.is_empty());
    }

    #[test]
    fn split_infile_tokens_treats_a_bare_dash_as_an_infile_not_a_flag() {
        // legacy: confirmed live -- see `looks_like_flag`'s own doc.
        let args = ["a.wav", "b.wav", "-"];
        let (infiles, rest) = split_infile_tokens(&args);
        assert_eq!(infiles, ["a.wav", "b.wav", "-"]);
        assert!(rest.is_empty());
    }

    #[test]
    fn split_infile_tokens_treats_a_dash_digit_token_as_an_infile_not_a_flag() {
        // legacy: `isalpha('5')` is false, so `-5` is not flag-shaped
        // either -- inferred from the same `count_infiles` source, not
        // independently confirmed live (no corpus file is named `-5`).
        let args = ["a.wav", "b.wav", "-5"];
        let (infiles, rest) = split_infile_tokens(&args);
        assert_eq!(infiles, ["a.wav", "b.wav", "-5"]);
        assert!(rest.is_empty());
    }
}
