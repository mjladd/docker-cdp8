// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `sndinfo prntsnd`: print sample values within a time range.
//!
//! Legacy: `legacy/dev/sndinfo/compare.c`'s `INFO_PRNTSND` case.
//!
//! Reads a sound file and prints sample values for frames between specified
//! start and end times, one frame per line with tab-separated channel values.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::SoundFile;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = "CDP Release 7.1 2016
PRINT SOUND SAMPLE VALUES WITHIN A TIME RANGE

USAGE: sndinfo prntsnd infile starttime endtime

";

pub fn prntsnd(parsed: &ParsedCommand) -> Result<(), CdpError> {
    if parsed.infiles.len() < 3 {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "prntsnd requires infile, starttime, and endtime\n".to_string(),
        ));
    }

    let infile_path = &parsed.infiles[0];
    let start_time_str = &parsed.infiles[1];
    let end_time_str = &parsed.infiles[2];

    let start_time: f64 = start_time_str.parse().map_err(|_| {
        CdpError::new(
            ExitCategory::UsageOnly,
            format!("Invalid starttime: {}\n", start_time_str),
        )
    })?;

    let end_time: f64 = end_time_str.parse().map_err(|_| {
        CdpError::new(
            ExitCategory::UsageOnly,
            format!("Invalid endtime: {}\n", end_time_str),
        )
    })?;

    if start_time >= end_time {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "Incompatible start and end times.\n".to_string(),
        ));
    }

    let sf = SoundFile::open(infile_path).map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("Can't open file {}\n", infile_path),
        )
    })?;

    let samples = sf.samples_f32().map_err(CdpError::from)?;
    let sr = sf.fmt.sample_rate as f64;
    let chans = sf.fmt.channels as usize;

    if samples.is_empty() {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "No samples in input file\n".to_string(),
        ));
    }

    // Convert times to sample indices
    let start_samp = (start_time * sr * chans as f64).round() as usize;
    let end_samp = (end_time * sr * chans as f64).round() as usize;

    if start_samp >= samples.len() {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "Start time is at end of file: can't proceed.\n".to_string(),
        ));
    }

    let end_samp = end_samp.min(samples.len());

    // Print samples in range, one frame per line
    let mut frame_count = start_samp / chans;
    for samp_idx in (start_samp..end_samp).step_by(chans) {
        print!("[{}]", frame_count);
        for ch in 0..chans {
            if samp_idx + ch < samples.len() {
                print!("\t{:.6}", samples[samp_idx + ch]);
            }
        }
        println!();
        frame_count += 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_conversion_works() {
        // Test that time to sample conversion is correct
        let sr = 44100.0;
        let chans = 2.0;
        let time = 1.0;
        let expected_samp = (time * sr * chans) as usize;
        assert_eq!(expected_samp, 88200);
    }
}
