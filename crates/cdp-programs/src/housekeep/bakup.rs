// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep bakup`: concatenate sound files with silence gaps between them.
//!
//! Legacy: `legacy/dev/houskeep/dump.c`'s `house_bakup()` and the related
//! `legacy/dev/houskeep/ap_house.c` setup.
//!
//! Scope: [`cdp_sf::FileKind::Wave`] only. `Analysis`/`Envelope`/`Pitch`/
//! `Transposition`/`Formant` report a plain `ProgramError`.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::{FileKind, SoundFile};

const BAKUP_GAP: f32 = 1.0; // seconds of silence between files
const F_SECSIZE: usize = 256; // sector size in samples

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = "CDP Release 7.1 2016
CONCATENATE SOUNDFILES IN ONE BAKUP FILE, WITH SILENCES BETWEEN

USAGE: housekeep bakup infile1 [infile2 ...] outfile

";

pub fn bakup(parsed: &ParsedCommand) -> Result<(), CdpError> {
    let outfile = parsed.outfile.as_ref().ok_or(CdpError::new(
        ExitCategory::UsageOnly,
        "bakup requires an output file\n".to_string(),
    ))?;

    super::check_outfile_does_not_exist(outfile)?;

    // Open all input files
    let mut infiles: Vec<SoundFile> = Vec::new();
    for infile_path in &parsed.infiles {
        let sf = SoundFile::open(infile_path).map_err(|_| {
            CdpError::new(
                ExitCategory::DataError,
                format!("Can't open file {infile_path} to read data.\n"),
            )
        })?;

        // Verify it's a sound file type
        if !matches!(sf.file_kind, FileKind::Wave) {
            return Err(CdpError::new(
                ExitCategory::UsageOnly,
                "Application doesn't work with this type of infile.\n".to_string(),
            ));
        }

        infiles.push(sf);
    }

    if infiles.is_empty() {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "Insufficient input files for this process\n".to_string(),
        ));
    }

    // Verify all files have compatible sample rates and channel counts
    let sr = infiles[0].fmt.sample_rate;
    let chans = infiles[0].fmt.channels;

    for sf in infiles.iter().skip(1) {
        if sf.fmt.sample_rate != sr || sf.fmt.channels != chans {
            return Err(CdpError::new(
                ExitCategory::DataError,
                "Incompatible sample-rate/channel-count in input file\n".to_string(),
            ));
        }
    }

    // Calculate silence samples (in samples, not interleaved frames)
    let insert_samps = (sr as f32 * BAKUP_GAP * chans as f32) as usize;
    let insert_samps = insert_samps.div_ceil(F_SECSIZE) * F_SECSIZE;

    // Collect all samples
    let mut all_samples = Vec::new();
    let mut total_samples = 0.0;

    for (file_idx, sf) in infiles.iter().enumerate() {
        // Read this file's samples
        let file_samples = sf.samples_f32().map_err(CdpError::from)?;

        all_samples.extend_from_slice(&file_samples);
        total_samples += file_samples.len() as f64 / chans as f64 / sr as f64;

        // Add silence after each file except the last
        if file_idx < parsed.infiles.len() - 1 {
            let silence_count = insert_samps;
            all_samples.extend(vec![0.0; silence_count]);
            total_samples += insert_samps as f64 / chans as f64 / sr as f64;
        }
    }

    // Write output file
    super::write_wave_file(
        chans,
        sr,
        cdp_sf::SampleType::Short16,
        &all_samples,
        outfile,
    )?;

    super::print_virtual_time(total_samples);
    println!();

    Ok(())
}
