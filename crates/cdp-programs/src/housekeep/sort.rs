// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep sort`: organize sound files by properties (sample rate, type, length, etc).
//!
//! Legacy: `legacy/dev/houskeep/sort.c`'s multiple sorting modes.
//!
//! Reads a text file of filenames, analyzes each sound file, and outputs
//! separate text files organized by the chosen sorting criterion.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use cdp_sf::{FileKind, SoundFile};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = "CDP Release 7.1 2016
SORT FILES INTO GROUPS BASED ON THEIR PROPERTIES

USAGE: housekeep sort mode infile outfile

MODES ARE
1) SORT BY SAMPLE RATE
2) SORT BY FILE TYPE (mono, stereo, analysis, etc)
4) SORT BY CHANNEL COUNT

";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    SampleRate = 1,
    FileType = 2,
    ChannelCount = 4,
}

impl Mode {
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Mode::SampleRate),
            2 => Some(Mode::FileType),
            4 => Some(Mode::ChannelCount),
            _ => None,
        }
    }
}

pub fn sort(parsed: &ParsedCommand, mode: Mode) -> Result<(), CdpError> {
    if parsed.infiles.len() < 2 {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "sort requires an input text file and output text file\n".to_string(),
        ));
    }

    let infile = &parsed.infiles[0];
    let outfile = &parsed.infiles[1];

    match mode {
        Mode::SampleRate => sort_by_sample_rate(infile, outfile),
        Mode::FileType => sort_by_file_type(infile, outfile),
        Mode::ChannelCount => sort_by_channel_count(infile, outfile),
    }
}

fn sort_by_sample_rate(infile: &str, outfile: &str) -> Result<(), CdpError> {
    let filenames = read_filenames(infile)?;

    // Group files by sample rate
    let mut by_srate: BTreeMap<u32, Vec<String>> = BTreeMap::new();

    for filename in filenames {
        match SoundFile::open(&filename) {
            Ok(sf) => {
                by_srate
                    .entry(sf.fmt.sample_rate)
                    .or_insert_with(Vec::new)
                    .push(filename);
            }
            Err(_) => {
                // Skip non-sound files
                continue;
            }
        }
    }

    // Write output files for each sample rate
    for (srate, filenames) in by_srate {
        let out_path = format!("{}_{}_Hz.txt", strip_extension(outfile), srate);
        write_filenames(&out_path, &filenames)?;
        eprintln!(
            "Created {} with {} files at {} Hz",
            out_path,
            filenames.len(),
            srate
        );
    }

    Ok(())
}

fn sort_by_file_type(infile: &str, outfile: &str) -> Result<(), CdpError> {
    let filenames = read_filenames(infile)?;

    // Group files by type
    let mut by_type: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for filename in filenames {
        match SoundFile::open(&filename) {
            Ok(sf) => {
                let type_name = file_type_name(&sf.file_kind);
                by_type
                    .entry(type_name)
                    .or_insert_with(Vec::new)
                    .push(filename);
            }
            Err(_) => {
                // Skip non-sound files
                continue;
            }
        }
    }

    // Write output files for each type
    for (type_name, filenames) in by_type {
        let out_path = format!("{}_{}.txt", strip_extension(outfile), type_name);
        write_filenames(&out_path, &filenames)?;
        eprintln!(
            "Created {} with {} {} files",
            out_path,
            filenames.len(),
            type_name
        );
    }

    Ok(())
}

fn sort_by_channel_count(infile: &str, outfile: &str) -> Result<(), CdpError> {
    let filenames = read_filenames(infile)?;

    // Group files by channel count
    let mut by_channels: BTreeMap<u16, Vec<String>> = BTreeMap::new();

    for filename in filenames {
        match SoundFile::open(&filename) {
            Ok(sf) => {
                by_channels
                    .entry(sf.fmt.channels)
                    .or_insert_with(Vec::new)
                    .push(filename);
            }
            Err(_) => {
                // Skip non-sound files
                continue;
            }
        }
    }

    // Write output files for each channel count
    for (channels, filenames) in by_channels {
        let channel_label = match channels {
            1 => "mono".to_string(),
            2 => "stereo".to_string(),
            n => format!("{}ch", n),
        };
        let out_path = format!("{}_{}.txt", strip_extension(outfile), channel_label);
        write_filenames(&out_path, &filenames)?;
        eprintln!(
            "Created {} with {} {} files",
            out_path,
            filenames.len(),
            channel_label
        );
    }

    Ok(())
}

fn file_type_name(kind: &FileKind) -> String {
    match kind {
        FileKind::Wave => "wave".to_string(),
        FileKind::Analysis(_) => "analysis".to_string(),
        FileKind::Pitch(_) => "pitch".to_string(),
        FileKind::Transposition(_) => "transposition".to_string(),
        FileKind::Formant { .. } => "formant".to_string(),
        FileKind::Envelope { .. } => "envelope".to_string(),
    }
}

fn read_filenames(filename: &str) -> Result<Vec<String>, CdpError> {
    let file = std::fs::File::open(filename).map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("Cannot open input file {}\n", filename),
        )
    })?;

    let reader = BufReader::new(file);
    let mut filenames = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                filenames.push(trimmed.to_string());
            }
        }
    }

    Ok(filenames)
}

fn write_filenames(filename: &str, names: &[String]) -> Result<(), CdpError> {
    let mut file = File::create(filename).map_err(|_| {
        CdpError::new(
            ExitCategory::DataError,
            format!("Cannot create output file {}\n", filename),
        )
    })?;

    for name in names {
        writeln!(file, "{}", name).map_err(|_| {
            CdpError::new(
                ExitCategory::DataError,
                "Failed to write to output file\n".to_string(),
            )
        })?;
    }

    Ok(())
}

fn strip_extension(filename: &str) -> String {
    if let Some(dot_pos) = filename.rfind('.') {
        filename[..dot_pos].to_string()
    } else {
        filename.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_extension_works() {
        assert_eq!(strip_extension("file.txt"), "file");
        assert_eq!(strip_extension("/path/to/file.txt"), "/path/to/file");
        assert_eq!(strip_extension("file"), "file");
    }

    #[test]
    fn file_type_name_works() {
        assert_eq!(file_type_name(&FileKind::Wave), "wave");
    }

    #[test]
    fn mode_from_number_works() {
        assert_eq!(Mode::from_number(1), Some(Mode::SampleRate));
        assert_eq!(Mode::from_number(2), Some(Mode::FileType));
        assert_eq!(Mode::from_number(4), Some(Mode::ChannelCount));
        assert_eq!(Mode::from_number(3), None);
        assert_eq!(Mode::from_number(5), None);
    }
}
