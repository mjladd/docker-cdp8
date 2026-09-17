// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep remove`: delete numbered duplicate files created by copy mode 2.
//!
//! Legacy: `legacy/dev/houskeep/dupl.c`'s `do_deletes()`.
//!
//! Takes a base filename and deletes all numbered copies (X_001, X_002, ..., X_999),
//! stopping when a file does not exist or after trying MAX_COPIES (999).

use cdp_core::{CdpError, ExitCategory};
use cdp_params::ParsedCommand;
use std::fs;
use std::path::Path;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = "CDP Release 7.1 2016
DELETE NUMBERED COPIES OF A FILE CREATED BY COPY MODE 2

USAGE: housekeep remove filename

";

const MAX_DUPLICATES: u32 = 999;

pub fn remove(parsed: &ParsedCommand) -> Result<(), CdpError> {
    if parsed.infiles.is_empty() {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            "remove requires a filename\n".to_string(),
        ));
    }

    let base_filename = &parsed.infiles[0];

    // Remove numbered copies
    let mut removed_count = 0;
    let mut not_found_count = 0;

    for n in 1..=MAX_DUPLICATES {
        let numbered_filename = numbered_filename(base_filename, n);
        let path = Path::new(&numbered_filename);

        if path.exists() {
            match fs::remove_file(&numbered_filename) {
                Ok(_) => {
                    removed_count += 1;
                    eprintln!("REMOVED {}", numbered_filename);
                }
                Err(_) => {
                    eprintln!("WARNING: Can't remove file {}\n", numbered_filename);
                }
            }
        } else {
            not_found_count += 1;
            // Stop after a reasonable number of consecutive missing files
            if not_found_count > 10 {
                break;
            }
        }
    }

    if removed_count == 0 {
        eprintln!("No numbered copies found to remove.\n");
    }

    Ok(())
}

fn numbered_filename(source: &str, number: u32) -> String {
    let number_str = match number {
        1..=9 => format!("_00{number}"),
        10..=99 => format!("_0{number}"),
        _ => format!("_{number}"),
    };

    // Find the last dot for the extension
    if let Some(dot_pos) = source.rfind('.') {
        let (base, ext) = source.split_at(dot_pos);
        format!("{base}{number_str}{ext}")
    } else {
        format!("{source}{number_str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbered_filename_pads_single_digits() {
        assert_eq!(numbered_filename("file.wav", 1), "file_001.wav");
        assert_eq!(numbered_filename("file.wav", 9), "file_009.wav");
    }

    #[test]
    fn numbered_filename_pads_double_digits() {
        assert_eq!(numbered_filename("file.wav", 10), "file_010.wav");
        assert_eq!(numbered_filename("file.wav", 99), "file_099.wav");
    }

    #[test]
    fn numbered_filename_no_padding_for_triple_digits() {
        assert_eq!(numbered_filename("file.wav", 100), "file_100.wav");
        assert_eq!(numbered_filename("file.wav", 999), "file_999.wav");
    }

    #[test]
    fn numbered_filename_preserves_path() {
        assert_eq!(
            numbered_filename("/path/to/file.wav", 1),
            "/path/to/file_001.wav"
        );
    }

    #[test]
    fn numbered_filename_handles_no_extension() {
        assert_eq!(numbered_filename("file", 1), "file_001");
    }
}
