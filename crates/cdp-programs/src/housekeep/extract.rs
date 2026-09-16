// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep extract mode 4`: rectify signal to eliminate DC drift.
//!
//! Legacy: `legacy/dev/houskeep/extract.c`'s rectify mode.
//!
//! Mode 4 only (Rectify): shift entire signal to eliminate DC drift by
//! subtracting the mean sample value from all samples. Other modes deferred.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::SoundFile;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = "CDP Release 7.1 2016
USAGE: housekeep extract
       1 inf [-ggate] [-sS] [-eE] [-tT] [-hH] [-bB] [-iI] [-lL] [-wW] [-n]
OR:    2 inf outf             OR: 3 inf outf [-ggate] [-ssplice] [-b] [-e]
OR:    4 inf outf shift       OR: 5 inf outf valsfile
OR:    6 inf gate endgate threshold baktrak initlevel minsize gatewin
MODE 1) CUT OUT & KEEP SIGNIFICANT EVENTS FROM INPUT SNDFILE.
MODE 2) PREVIEW: make envel as 'sound'. View to get params for CUT OUT & KEEP.
MODE 3) TOP AND TAIL: REMOVE LOW LEVEL SIGNAL FROM START & END OF SOUND.
MODE 4) RECTIFY: SHIFT ENTIRE SIGNAL TO ELIMINATE DC DRIFT.
MODE 5) MODIFY 'BY HAND'. This process is no longer available.
MODE 6) GET ONSET TIMES.

";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Rectify = 4,
}

impl Mode {
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            4 => Some(Mode::Rectify),
            _ => None,
        }
    }
}

pub fn extract(parsed: &ParsedCommand, mode: Mode) -> Result<(), CdpError> {
    match mode {
        Mode::Rectify => rectify(parsed),
    }
}

fn rectify(parsed: &ParsedCommand) -> Result<(), CdpError> {
    if parsed.infiles.len() < 2 {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "extract mode 4: requires infile and outfile\n".to_string(),
        ));
    }

    let infile_path = &parsed.infiles[0];
    let outfile_path = &parsed.infiles[1];

    // Check that outfile doesn't already exist
    super::check_outfile_does_not_exist(outfile_path)?;

    let sf = SoundFile::open(infile_path).map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("Can't open file {infile_path} to read data.\n"),
        )
    })?;

    // Rectify: shift signal to eliminate DC drift
    // Calculate DC offset (mean of all samples)
    let samples = sf.samples_f32().map_err(CdpError::from)?;

    if samples.is_empty() {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "No samples in input file\n".to_string(),
        ));
    }

    let dc_offset: f32 = samples.iter().sum::<f32>() / samples.len() as f32;

    // Subtract DC offset from all samples
    let rectified: Vec<f32> = samples.iter().map(|&s| s - dc_offset).collect();

    // Write output file
    super::write_wave_file(
        sf.fmt.channels,
        sf.fmt.sample_rate,
        sf.fmt.sample_type,
        &rectified,
        outfile_path,
    )?;

    super::print_virtual_time(
        rectified.len() as f64 / sf.fmt.channels as f64 / sf.fmt.sample_rate as f64,
    );
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_from_number_works() {
        assert_eq!(Mode::from_number(4), Some(Mode::Rectify));
        assert_eq!(Mode::from_number(1), None);
        assert_eq!(Mode::from_number(2), None);
        assert_eq!(Mode::from_number(3), None);
        assert_eq!(Mode::from_number(5), None);
        assert_eq!(Mode::from_number(6), None);
    }
}
