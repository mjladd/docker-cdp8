// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `sndinfo prntsnd`: write the sample values in a time range to a text
//! file.
//!
//! legacy: `case(INFO_PRNTSND)` in `legacy/dev/sndinfo/compare.c`.
//!
//! The destination is a text file named on the command line, not standard
//! output. An earlier version of this module printed to standard output and
//! took three arguments instead of four, so it read the text file's name as
//! a start time.
//!
//! One line per frame, of the shape
//!
//! ```text
//! [<frame>]\t<channel 0>\t<channel 1>\t...\t\n
//! ```
//!
//! with each value written as `%.6f`. Note the tab after the last channel:
//! legacy writes one per channel and then the newline, so every line ends
//! with a tab before its newline. The frame number is absolute, counted
//! from the start of the file rather than from the start of the range.
//!
//! legacy's own usage text warns "CARE!!! large quantities of data.", which
//! is why the golden cases for this command all use a short range.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::{ParamValue, ParsedCommand};
use std::io::Write;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = r"CDP Release 7.1 2016
PRINT SOUND SAMPLE DATA TO A TEXTFILE

USAGE: sndreport prntsnd infile outtextfile starttime endtime

CARE!!! large quantities of data.
";

pub fn prntsnd(parsed: &ParsedCommand) -> Result<(), CdpError> {
    let infile_path = &parsed.infiles[0];
    let outfile_path = parsed
        .outfile
        .as_deref()
        .expect("sndinfo_prntsnd sets has_outfile");

    let sf = super::open_sound_infile(infile_path)?;
    let samples = sf.samples_f32().map_err(CdpError::from)?;
    let chans = sf.fmt.channels as usize;
    let sample_rate = f64::from(sf.fmt.sample_rate);

    let (start_time, end_time) = match (parsed.params.first(), parsed.params.get(1)) {
        (Some(ParamValue::Number(a)), Some(ParamValue::Number(b))) => (*a, *b),
        _ => unreachable!("sndinfo_prntsnd declares two Double parameters"),
    };

    // legacy: `round(param * sr) * chans`, so each time becomes a sample
    // index aligned to a frame boundary.
    let start_samp = (start_time * sample_rate).round() as usize * chans;
    let end_samp = (end_time * sample_rate).round() as usize * chans;

    if start_samp >= end_samp {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "Incompatible start and end times.\n",
        ));
    }
    if start_samp >= samples.len() {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "Start time is at end of file: can't proceed.\n",
        ));
    }

    let mut out = std::fs::File::create(outfile_path).map_err(|e| {
        CdpError::new(
            ExitCategory::DataError,
            format!("Cannot open output file {outfile_path}: {e}\n"),
        )
    })?;

    // legacy's own loop streams through buffers and stops at end of file.
    // Reading the whole file makes the same bound explicit.
    let end_samp = end_samp.min(samples.len());
    for (frame, samp) in (start_samp / chans..).zip((start_samp..end_samp).step_by(chans)) {
        write!(out, "[{frame}]\t").map_err(write_error)?;
        for channel in 0..chans {
            if let Some(value) = samples.get(samp + channel) {
                write!(out, "{value:.6}\t").map_err(write_error)?;
            }
        }
        writeln!(out).map_err(write_error)?;
    }

    super::super::housekeep::print_virtual_time(
        (end_samp - start_samp) as f64 / (sample_rate * chans as f64),
    );
    Ok(())
}

fn write_error(e: std::io::Error) -> CdpError {
    CdpError::new(
        ExitCategory::SystemError,
        format!("Cannot write the output text file: {e}\n"),
    )
}

#[cfg(test)]
mod tests {
    /// legacy: `round(param * sr) * chans`.
    fn sample_index(time: f64, sample_rate: f64, chans: usize) -> usize {
        (time * sample_rate).round() as usize * chans
    }

    #[test]
    fn a_time_becomes_a_frame_aligned_sample_index() {
        assert_eq!(sample_index(1.0, 44100.0, 1), 44100);
        assert_eq!(sample_index(1.0, 44100.0, 2), 88200);
        // The rounding happens before the channel multiply, so the result
        // always lands on a frame boundary.
        assert_eq!(sample_index(0.00001, 44100.0, 2) % 2, 0);
    }

    #[test]
    fn rounding_is_to_nearest_not_truncation() {
        // 0.0000115 * 44100 is 0.50715, which rounds up to 1.
        assert_eq!(sample_index(0.0000115, 44100.0, 1), 1);
        // 0.0000105 * 44100 is 0.46305, which rounds down to 0.
        assert_eq!(sample_index(0.0000105, 44100.0, 1), 0);
    }

    /// The expected lines come from a live `legacy` run through
    /// `cdp8-postmerge`, not from this module's own output:
    ///
    /// ```text
    /// $ sndinfo prntsnd marimba.wav o.txt 0.0 0.0002
    /// $ head -3 o.txt
    /// [0]\t0.004303\t
    /// [1]\t0.003510\t
    /// [2]\t0.004883\t
    /// ```
    ///
    /// Note the tab after the last channel on every line, which legacy
    /// writes because it emits one per channel and then the newline.
    #[test]
    fn formats_lines_the_way_a_live_legacy_run_did() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../docs/manual/sounds/marimba.wav"
        );
        let sf = cdp_sf::SoundFile::open(path).expect("open marimba.wav");
        let samples = sf.samples_f32().expect("decode marimba.wav");
        let chans = sf.fmt.channels as usize;
        assert_eq!(chans, 1, "marimba.wav is mono");

        let lines: Vec<String> = (0..3)
            .map(|frame| {
                let mut line = format!("[{frame}]\t");
                for channel in 0..chans {
                    line.push_str(&format!("{:.6}\t", samples[frame * chans + channel]));
                }
                line
            })
            .collect();

        assert_eq!(lines[0], "[0]\t0.004303\t");
        assert_eq!(lines[1], "[1]\t0.003510\t");
        assert_eq!(lines[2], "[2]\t0.004883\t");
    }
}
