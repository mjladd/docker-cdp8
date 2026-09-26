// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep bakup`: concatenate sound files into one, with a gap between
//! them.
//!
//! legacy: `house_bakup` in `legacy/dev/houskeep/dump.c`.
//!
//! The gap is not a command-line argument. An earlier version of this module
//! invented a `splicelen` positional, which made the documented command line
//! `housekeep bakup a.wav b.wav out.wav` fail outright. The gap is the
//! compile-time constant `BAKUP_GAP` of 1.0 second (`house.h`), rounded up
//! to a whole disk sector.
//!
//! ## Why this does not simulate legacy's buffer loop
//!
//! `house_bakup` writes through an internal buffer whose size comes from
//! `Malloc(-1)`, the largest free block of memory at allocation time
//! (`create_bakup_sndbufs`), so it is machine-dependent. Its loop then
//! branches on how much of that buffer the last read left spare, which
//! looks as though the output would be machine-dependent too.
//!
//! It is not. Both branches write the same total, which is why this module
//! can compute the output directly:
//!
//! - `buflen` is always a whole number of sectors, so `spare_samps mod
//!   F_SECSIZE` is fixed by the sample count rather than by `buflen`. The
//!   first branch writes `read + tail + gap`, and `read + tail` is exactly
//!   `read` rounded up to a sector.
//! - The second branch writes the whole buffer, then `gap - spare` rounded
//!   up to a sector. Those two sum to the same value, because the buffer's
//!   own zero tail is counted in the first term instead of the second.
//!
//! So each input contributes `roundup(samples) + gap`, whatever the buffer
//! size. Confirmed against three live runs of `cdp8-postmerge`: one mono
//! file gives 88576 samples, two give 177152, and one stereo file
//! (`clashmixtest.wav`, 219840 samples) gives 308224. The formula predicts
//! all three exactly.
//!
//! Note also that legacy writes a gap after the **last** file as well, not
//! only between files. That falls out of the loop structure, and the sample
//! counts above confirm it.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::SoundFile;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = r"CDP Release 7.1 2016
CONCATENATE SOUNDFILES IN ONE BAKUP FILE, WITH SILENCES BETWEEN

USAGE: housekeep bakup infile1 [infile2 ...] outfile

";

/// legacy: `F_SECSIZE` (`tkglobals.h`), the sector size `house_bakup`'s own
/// arithmetic rounds to. Note that `create_bakup_sndbufs` uses
/// `F_SECSIZE * channels` for the buffer instead, which does not affect the
/// output.
const F_SECSIZE: u64 = 256;

/// legacy: `BAKUP_GAP` (`house.h`), the gap in seconds.
const BAKUP_GAP: f64 = 1.0;

/// Rounds `samples` up to a whole sector.
fn round_up_to_sector(samples: u64) -> u64 {
    samples.div_ceil(F_SECSIZE) * F_SECSIZE
}

/// The gap legacy writes after every input file, in samples.
fn gap_samples(sample_rate: u32, channels: u16) -> u64 {
    let one_second = f64::from(sample_rate) * f64::from(channels);
    round_up_to_sector((BAKUP_GAP * one_second).round() as u64)
}

pub fn bakup(parsed: &ParsedCommand) -> Result<(), CdpError> {
    let outfile_path = parsed
        .outfile
        .as_deref()
        .expect("bakup's dispatch always supplies an outfile");

    // legacy: the first infile sets the format every later one must match
    // (`open_checktype_getsize_and_compareheader`,
    // `legacy/dev/cdp2k/readfiles.c`).
    let mut opened: Vec<SoundFile> = Vec::with_capacity(parsed.infiles.len());
    for (index, path) in parsed.infiles.iter().enumerate() {
        let sf = if index == 0 {
            crate::sndinfo::open_sound_infile(path)?
        } else {
            crate::sndinfo::open_other_sound_infile(path)?
        };
        if index > 0 {
            let first = &opened[0];
            if sf.fmt.sample_rate != first.fmt.sample_rate {
                return Err(CdpError::new(
                    ExitCategory::DataError,
                    format!("Incompatible sample-rate in input file {path}.\n"),
                ));
            }
            if sf.fmt.channels != first.fmt.channels {
                return Err(CdpError::new(
                    ExitCategory::DataError,
                    format!("Incompatible channel-count in input file {path}.\n"),
                ));
            }
        }
        opened.push(sf);
    }

    let first = &opened[0];
    let sample_rate = first.fmt.sample_rate;
    let channels = first.fmt.channels;
    let sample_type = first.fmt.sample_type;
    let gap = gap_samples(sample_rate, channels);

    let mut out: Vec<f32> = Vec::new();
    for sf in &opened {
        let samples = sf.samples_f32().map_err(CdpError::from)?;
        let padded = round_up_to_sector(samples.len() as u64) as usize;
        out.extend_from_slice(&samples);
        // Legacy's buffer starts zeroed, so the padding to the sector
        // boundary and the gap itself are both silence.
        out.resize(out.len() + (padded - samples.len()) + gap as usize, 0.0);
    }

    super::check_outfile_does_not_exist(outfile_path)?;
    super::write_wave_file(channels, sample_rate, sample_type, &out, outfile_path)?;
    super::print_virtual_time(out.len() as f64 / (f64::from(sample_rate) * f64::from(channels)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_sector_boundary_is_left_alone() {
        assert_eq!(round_up_to_sector(0), 0);
        assert_eq!(round_up_to_sector(256), 256);
        assert_eq!(round_up_to_sector(512), 512);
    }

    #[test]
    fn anything_else_rounds_up() {
        assert_eq!(round_up_to_sector(1), 256);
        assert_eq!(round_up_to_sector(255), 256);
        assert_eq!(round_up_to_sector(257), 512);
        // marimba.wav's own length.
        assert_eq!(round_up_to_sector(44174), 44288);
    }

    #[test]
    fn the_gap_is_one_second_rounded_up() {
        // 44100 samples is 172.27 sectors, so 173.
        assert_eq!(gap_samples(44100, 1), 173 * 256);
        // Stereo doubles the sample count before rounding: 88200 is 344.5
        // sectors, so 345.
        assert_eq!(gap_samples(44100, 2), 345 * 256);
    }

    /// Every expected total here comes from a live `legacy` run through
    /// `cdp8-postmerge`, not from this module.
    #[test]
    fn totals_match_three_live_legacy_runs() {
        let per_file =
            |samples: u64, sr: u32, ch: u16| round_up_to_sector(samples) + gap_samples(sr, ch);
        // One mono marimba.wav (44174 samples).
        assert_eq!(per_file(44174, 44100, 1), 88576);
        // Two of them.
        assert_eq!(2 * per_file(44174, 44100, 1), 177152);
        // One stereo clashmixtest.wav (219840 samples).
        assert_eq!(per_file(219840, 44100, 2), 308224);
    }
}
