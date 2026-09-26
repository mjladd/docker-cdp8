// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep bundle`: filter and list sound files by properties.
//!
//! Legacy: `legacy/dev/houskeep/sort.c`'s `do_bundle()`.
//!
//! Filters input files by mode and writes matching filenames to an output text file.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::{FileKind, SoundFile};
use std::fs::File;
use std::io::Write;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = r"CDP Release 7.1 2016
LIST FILENAMES IN TEXTFILE FOR SORTING, OR MIXDUMMY.

USAGE: housekeep bundle mode infile [infile2....] outtextfile

MODES ARE
1) BUNDLE ALL ENTERED FILES
2) BUNDLE ALL NON-TEXT FILES ENTERED
3) BUNDLE ALL NON-TEXT FILES OF SAME TYPE AS FIRST NON-TEXT FILE
           e.g. all sndfiles, or all analysis files....
4) AS (3), BUT ONLY FILES WITH SAME PROPERTIES
5) AS (4), BUT IF FILE1 IS SNDFILE, FILES WITH SAME CHAN COUNT ONLY
";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    All,
    NonText,
    Type,
    SameProperties,
    SameChanCount,
}

impl Mode {
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Mode::All),
            2 => Some(Mode::NonText),
            3 => Some(Mode::Type),
            4 => Some(Mode::SameProperties),
            5 => Some(Mode::SameChanCount),
            _ => None,
        }
    }
}

fn filekind_discriminant(fk: &FileKind) -> &'static str {
    match fk {
        FileKind::Wave => "Wave",
        FileKind::Analysis(_) => "Analysis",
        FileKind::Pitch(_) => "Pitch",
        FileKind::Transposition(_) => "Transposition",
        FileKind::Formant { .. } => "Formant",
        FileKind::Envelope { .. } => "Envelope",
    }
}

pub fn bundle(parsed: &ParsedCommand, mode: Mode) -> Result<(), CdpError> {
    if parsed.infiles.is_empty() {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "Insufficient input files for this process\n".to_string(),
        ));
    }

    let outfile = parsed.outfile.as_ref().ok_or(CdpError::new(
        ExitCategory::UsageOnly,
        "bundle requires an output text file\n".to_string(),
    ))?;

    // Check that output filename is not in the infiles list
    for infile in &parsed.infiles {
        if infile == outfile {
            return Err(CdpError::new(
                ExitCategory::DataError,
                format!(
                    "Name of out-listfile [{}] cannot be included in the listing!!\n",
                    outfile
                ),
            ));
        }
    }

    let mut output = File::create(outfile).map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("Cannot open output file {}\n", outfile),
        )
    })?;

    let mut first_sound_file: Option<FileProperties> = None;
    let mut first_sound_type: Option<String> = None;

    for infile_path in parsed.infiles.iter() {
        // For Mode 1, include all files (even non-sound files)
        if mode == Mode::All {
            writeln!(output, "{}", infile_path).map_err(|_| {
                CdpError::new(
                    ExitCategory::DataError,
                    "Failed to write to output file\n".to_string(),
                )
            })?;
            println!("BUNDLED {infile_path}");
            continue;
        }

        // Try to open as sound file
        let sf = match SoundFile::open(infile_path) {
            Ok(file) => file,
            Err(_) => {
                // Not a sound file; skip for modes 2-5
                continue;
            }
        };

        // For mode 2, include all non-text files
        if mode == Mode::NonText {
            writeln!(output, "{}", infile_path).map_err(|_| {
                CdpError::new(
                    ExitCategory::DataError,
                    "Failed to write to output file\n".to_string(),
                )
            })?;
            println!("BUNDLED {infile_path}");
            continue;
        }

        // For modes 3-5, we need to check against the first sound file
        if first_sound_type.is_none() {
            // This is the first sound file we've encountered
            first_sound_type = Some(filekind_discriminant(&sf.file_kind).to_string());
            first_sound_file = Some(FileProperties::from_sound_file(&sf));
            writeln!(output, "{}", infile_path).map_err(|_| {
                CdpError::new(
                    ExitCategory::DataError,
                    "Failed to write to output file\n".to_string(),
                )
            })?;
            println!("BUNDLED {infile_path}");
            continue;
        }

        // Check if this file matches the criteria for inclusion
        let should_include = match mode {
            Mode::Type => {
                // Include if same file type as first sound file
                filekind_discriminant(&sf.file_kind) == first_sound_type.as_ref().unwrap().as_str()
            }
            Mode::SameProperties => {
                // Include if same type AND same properties
                filekind_discriminant(&sf.file_kind) == first_sound_type.as_ref().unwrap().as_str()
                    && properties_match(&sf, first_sound_file.as_ref().unwrap())
            }
            Mode::SameChanCount => {
                // Include if same type, properties, and channel count
                filekind_discriminant(&sf.file_kind) == first_sound_type.as_ref().unwrap().as_str()
                    && properties_match(&sf, first_sound_file.as_ref().unwrap())
                    && sf.fmt.channels == first_sound_file.as_ref().unwrap().channels
            }
            _ => false, // Already handled above
        };

        if should_include {
            writeln!(output, "{}", infile_path).map_err(|_| {
                CdpError::new(
                    ExitCategory::DataError,
                    "Failed to write to output file\n".to_string(),
                )
            })?;
            println!("BUNDLED {infile_path}");
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct FileProperties {
    sample_rate: u32,
    channels: u16,
    // For analysis files, store additional properties
    orig_sample_rate: Option<u32>,
    arate: Option<u32>,
    window_size: Option<f32>,
}

impl FileProperties {
    fn from_sound_file(sf: &SoundFile) -> Self {
        FileProperties {
            sample_rate: sf.fmt.sample_rate,
            channels: sf.fmt.channels,
            orig_sample_rate: sf
                .properties
                .get_i32("original sample rate")
                .ok()
                .map(|x| x as u32),
            arate: sf.properties.get_i32("arate").ok().map(|x| x as u32),
            window_size: sf.properties.get_f32("analwinlen").ok(),
        }
    }
}

fn properties_match(sf: &SoundFile, first_props: &FileProperties) -> bool {
    // Check basic properties that apply to all files
    if sf.fmt.sample_rate != first_props.sample_rate {
        return false;
    }
    if sf.fmt.channels != first_props.channels {
        return false;
    }

    // For wave files, just check sample rate and channels
    if matches!(sf.file_kind, FileKind::Wave) {
        return true;
    }

    // For analysis files, check additional properties
    let sf_orig_sr = sf
        .properties
        .get_i32("original sample rate")
        .ok()
        .map(|x| x as u32);
    let sf_arate = sf.properties.get_i32("arate").ok().map(|x| x as u32);
    let sf_window = sf.properties.get_f32("analwinlen").ok();

    if sf_orig_sr != first_props.orig_sample_rate {
        return false;
    }
    if sf_arate != first_props.arate {
        return false;
    }
    if sf_window != first_props.window_size {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_from_number_works() {
        assert_eq!(Mode::from_number(1), Some(Mode::All));
        assert_eq!(Mode::from_number(2), Some(Mode::NonText));
        assert_eq!(Mode::from_number(3), Some(Mode::Type));
        assert_eq!(Mode::from_number(4), Some(Mode::SameProperties));
        assert_eq!(Mode::from_number(5), Some(Mode::SameChanCount));
        assert_eq!(Mode::from_number(0), None);
        assert_eq!(Mode::from_number(6), None);
    }
}
