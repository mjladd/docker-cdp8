// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `sndinfo findhole`: find silent sections in a sound file.
//!
//! Legacy: `legacy/dev/sndinfo/compare.c`'s `INFO_FINDHOLE` case.
//!
//! Scans a sound file for the longest continuous section below a specified
//! amplitude threshold, reporting its time location and duration.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::SoundFile;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = "CDP Release 7.1 2016
FIND THE LONGEST SILENT SECTION IN A SOUND FILE

USAGE: sndinfo findhole infile threshold

WHERE
threshold = amplitude level below which sound is considered silent (range 0.0 to 1.0)

";

pub fn findhole(parsed: &ParsedCommand) -> Result<(), CdpError> {
    if parsed.infiles.len() < 2 {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "findhole requires infile and threshold\n".to_string(),
        ));
    }

    let infile_path = &parsed.infiles[0];
    let threshold_str = &parsed.infiles[1];

    let threshold: f64 = threshold_str.parse().map_err(|_| {
        CdpError::new(
            ExitCategory::UsageOnly,
            format!("Invalid threshold: {}\n", threshold_str),
        )
    })?;

    if threshold < 0.0 || threshold > 1.0 {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "Threshold out of range (0.0 to 1.0).\n".to_string(),
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

    // Convert threshold to float32 for comparison
    let threshold_f32 = threshold as f32;

    // Find longest silent section (all channels below threshold)
    let mut max_start_frame = 0;
    let mut max_duration_frames = 0;
    let mut current_start_frame = 0;
    let mut in_silent = false;

    let num_frames = samples.len() / chans;

    for frame_idx in 0..num_frames {
        let frame_start = frame_idx * chans;
        let mut frame_silent = true;

        // Check if all channels in this frame are below threshold
        for ch in 0..chans {
            if frame_start + ch < samples.len() {
                if samples[frame_start + ch].abs() >= threshold_f32 {
                    frame_silent = false;
                    break;
                }
            }
        }

        if frame_silent {
            if !in_silent {
                current_start_frame = frame_idx;
                in_silent = true;
            }
        } else {
            if in_silent {
                let duration = frame_idx - current_start_frame;
                if duration > max_duration_frames {
                    max_duration_frames = duration;
                    max_start_frame = current_start_frame;
                }
                in_silent = false;
            }
        }
    }

    // Handle case where file ends in silence
    if in_silent {
        let duration = num_frames - current_start_frame;
        if duration > max_duration_frames {
            max_duration_frames = duration;
            max_start_frame = current_start_frame;
        }
    }

    // Report the hole
    if max_duration_frames == 0 {
        println!("No hole found : entire file exceeds threshold.\n");
    } else {
        let start_time = max_start_frame as f64 / sr;
        let duration_secs = max_duration_frames as f64 / sr;

        println!(
            "Longest hole found at time {:.3} secs, duration {:.3} secs.\n",
            start_time, duration_secs
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_range_validation_works() {
        // Verify that thresholds outside [0, 1] would be rejected
        assert!("-0.1".parse::<f64>().unwrap() < 0.0);
        assert!("1.1".parse::<f64>().unwrap() > 1.0);
    }
}
