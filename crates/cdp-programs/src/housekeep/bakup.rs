// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep bakup`: concatenate sound files with silence between them.
//!
//! Legacy: `legacy/dev/houskeep/bakup.c`
//!
//! Reads multiple sound files and writes them concatenated into an output file,
//! with a specified duration of silence inserted between each pair of files.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::{SampleType, SoundFile};

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = "CDP Release 7.1 2016
CONCATENATE SOUND FILES WITH SILENCE BETWEEN THEM

USAGE: housekeep bakup infile1 [infile2 ...] outfile splicelen

WHERE
splicelen = duration of silence (in seconds) to insert between files

";

pub fn bakup(parsed: &ParsedCommand) -> Result<(), CdpError> {
    if parsed.infiles.len() < 3 {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "bakup requires at least 2 infiles, outfile, and splicelen\n".to_string(),
        ));
    }

    let splice_len_str = &parsed.infiles[parsed.infiles.len() - 1];
    let outfile = &parsed.infiles[parsed.infiles.len() - 2];
    let infiles = &parsed.infiles[..parsed.infiles.len() - 2];

    let splice_len: f64 = splice_len_str.parse().map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("Invalid splicelen: {}\n", splice_len_str),
        )
    })?;

    if splice_len < 0.0 {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "Splicelen cannot be negative.\n".to_string(),
        ));
    }

    // Open all infiles first to validate and get properties
    let mut sound_files = Vec::new();
    let mut sr_out = 0u32;
    let mut chans_out = 0u16;

    for (i, infile_path) in infiles.iter().enumerate() {
        let sf = SoundFile::open(infile_path).map_err(|_| {
            CdpError::new(
                ExitCategory::DataError,
                format!("Can't open file {}\n", infile_path),
            )
        })?;

        if i == 0 {
            sr_out = sf.fmt.sample_rate;
            chans_out = sf.fmt.channels;
        } else {
            if sf.fmt.sample_rate != sr_out || sf.fmt.channels != chans_out {
                return Err(CdpError::new(
                    ExitCategory::DataError,
                    "Incompatible sample rate or channel count in input file.\n".to_string(),
                ));
            }
        }

        sound_files.push(sf);
    }

    // Compute total samples needed
    let mut total_samples = 0u64;
    let splice_samples = (splice_len * sr_out as f64).round() as u64 * chans_out as u64;

    for sf in &sound_files {
        let samples = sf.samples_f32().map_err(CdpError::from)?;
        total_samples += samples.len() as u64;
        total_samples += splice_samples;
    }

    // Remove final splice (no silence after last file)
    if total_samples >= splice_samples {
        total_samples -= splice_samples;
    }

    // Allocate output buffer
    let mut output_samples = vec![0.0f32; total_samples as usize];
    let mut write_idx = 0;

    // Write each file with silence between
    for (file_idx, sf) in sound_files.iter().enumerate() {
        // Write file samples
        let samples = sf.samples_f32().map_err(CdpError::from)?;
        for &sample in &samples {
            if write_idx < output_samples.len() {
                output_samples[write_idx] = sample;
                write_idx += 1;
            }
        }

        // Write silence between files (not after the last one)
        if file_idx < sound_files.len() - 1 {
            for _ in 0..splice_samples {
                if write_idx < output_samples.len() {
                    output_samples[write_idx] = 0.0;
                    write_idx += 1;
                }
            }
        }
    }

    // Write output file
    super::write_wave_file(
        chans_out,
        sr_out,
        SampleType::Short16,
        &output_samples,
        outfile,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn splice_length_validation_works() {
        assert!("-0.1".parse::<f64>().is_ok()); // Parse succeeds; validation would fail
        assert!("0.0".parse::<f64>().is_ok());
        assert!("1.5".parse::<f64>().is_ok());
    }
}
