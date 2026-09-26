// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep remove`: delete the numbered copies `housekeep copy` mode 2
//! made.
//!
//! legacy: `do_deletes` in `legacy/dev/houskeep/dupl.c`.
//!
//! **Legacy cannot run this command at all.** Every invocation aborts with
//! `"Invalid process_type 20: create_sized_outfile()"` and deletes nothing,
//! confirmed for the plain form, for a run with copies present, and for the
//! documented `-a` flag. See `docs/migration/LEGACY-BUGS.md`.
//!
//! So there is no legacy behavior to compare against, and this port follows
//! `docs/migration/PLAN.md` section 6: port the intended behavior. The
//! intent is taken from `do_deletes` itself rather than from the usage
//! prose, since the code is the more precise of the two. Every golden case
//! for this command is a `known_deviation` by design, and will stay that
//! way unless the legacy bug is ever fixed upstream.
//!
//! One detail of `do_deletes` is easy to misread. Its `unopen` counter is a
//! *total* count of names that were not found, and is never reset when one
//! is found. So the standard form stops once ten names in the whole run
//! have been missing, not ten in a row. The usage text's wording ("once a
//! numbered file is missing, program checks for 10 more named files")
//! suggests consecutive misses, and it is wrong.

use cdp_core::CdpError;
use cdp_params::ParsedCommand;
use std::path::Path;

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = r"CDP Release 7.1 2016
REMOVE EXISTING COPIES OF A FILE 

USAGE: housekeep remove filename [-a]

         Deletes any copies of filename X, having names X_001,X_002...
         No checks are made that these ARE COPIES of file X!!
     -a  Program checks all names in numbered sequence.
         In standard case, once a numbered file is missing,
         program checks for 10 more named files before halting.
         Setting -a flag forces program to search for all possible
         duplicate filenames. This may take some time.
";

/// legacy: `MAXDUPL` (`house.h`), the highest numbered copy considered.
const MAX_DUPLICATES: u32 = 999;

/// legacy: `COPYDEL_OVERMAX` (`house.h`). Without `-a`, the scan stops once
/// this many names in total have been missing.
const MISSES_BEFORE_STOPPING: u32 = 10;

pub fn remove(parsed: &ParsedCommand, all_copies: bool) -> Result<(), CdpError> {
    let base = &parsed.infiles[0];
    let mut misses = 0u32;
    let mut removed = 0u32;

    for n in 1..=MAX_DUPLICATES {
        // legacy tests this at the top of the loop, before building the
        // name for this n.
        if !all_copies && misses >= MISSES_BEFORE_STOPPING {
            break;
        }

        let name = numbered_filename(base, n);
        if !Path::new(&name).exists() {
            misses += 1;
            continue;
        }
        match std::fs::remove_file(&name) {
            Ok(()) => removed += 1,
            Err(_) => {
                // legacy prints its warnings on standard output, not
                // standard error, and does not count the file as removed.
                println!("WARNING: Can't set output soundfile {name} for deletion.");
            }
        }
    }

    println!("INFO: {removed} duplicate files removed.");
    Ok(())
}

/// legacy: `insert_new_chars_at_filename_end` plus
/// `insert_new_number_at_filename_end` (`legacy/dev/cdp2k/mainfuncs.c`),
/// called with `"_00"`, `"_0"` or `"_"` by magnitude, so the number is
/// zero-padded to three digits.
fn numbered_filename(source: &str, number: u32) -> String {
    let suffix = match number {
        1..=9 => format!("_00{number}"),
        10..=99 => format!("_0{number}"),
        _ => format!("_{number}"),
    };

    // Only a dot after the last path separator counts as an extension,
    // matching the backward scan those two legacy helpers perform.
    let stem_end = source
        .rfind('.')
        .filter(|dot| !source[dot + 1..].contains('/') && !source[dot + 1..].contains('\\'));
    match stem_end {
        Some(dot) => {
            let (base, ext) = source.split_at(dot);
            format!("{base}{suffix}{ext}")
        }
        None => format!("{source}{suffix}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_are_padded_to_three_digits() {
        assert_eq!(numbered_filename("file.wav", 1), "file_001.wav");
        assert_eq!(numbered_filename("file.wav", 9), "file_009.wav");
        assert_eq!(numbered_filename("file.wav", 10), "file_010.wav");
        assert_eq!(numbered_filename("file.wav", 99), "file_099.wav");
        assert_eq!(numbered_filename("file.wav", 100), "file_100.wav");
        assert_eq!(numbered_filename("file.wav", 999), "file_999.wav");
    }

    #[test]
    fn a_directory_component_is_not_mistaken_for_an_extension() {
        assert_eq!(
            numbered_filename("/path/to/file.wav", 1),
            "/path/to/file_001.wav"
        );
        assert_eq!(numbered_filename("/a.b/file", 1), "/a.b/file_001");
        assert_eq!(numbered_filename("file", 1), "file_001");
    }

    /// legacy stops once ten names *in total* have been missing, because
    /// `do_deletes`' `unopen` counter is never reset. Ten in a row is a
    /// different rule, and it is the one the usage text implies.
    #[test]
    fn the_standard_form_counts_total_misses_not_consecutive_ones() {
        // Names 1 to 10 missing, 11 present: the standard form never
        // reaches 11, because the tenth miss stops it.
        let present: Vec<u32> = vec![11];
        assert_eq!(scan(&present, false), 0);
        // With -a there is no stopping rule, so it is found.
        assert_eq!(scan(&present, true), 1);
    }

    #[test]
    fn a_hit_does_not_give_the_standard_form_a_fresh_allowance() {
        // Copies at 10, 20 and 30. Names 1 to 9 are nine misses, 10 is a
        // hit, and 11 is the tenth miss overall, so the loop stops at 12
        // and never reaches 20. A consecutive-miss rule would have found
        // all three, which is the difference the total count makes.
        let present: Vec<u32> = vec![10, 20, 30];
        assert_eq!(
            scan(&present, false),
            1,
            "only the copy at 10 is reached before the tenth miss overall"
        );
        assert_eq!(scan(&present, true), 3, "-a has no stopping rule");
    }

    /// Counts how many of `present` the loop would reach, applying the
    /// same stopping rule as [`remove`] without touching the filesystem.
    fn scan(present: &[u32], all_copies: bool) -> u32 {
        let mut misses = 0u32;
        let mut removed = 0u32;
        for n in 1..=MAX_DUPLICATES {
            if !all_copies && misses >= MISSES_BEFORE_STOPPING {
                break;
            }
            if present.contains(&n) {
                removed += 1;
            } else {
                misses += 1;
            }
        }
        removed
    }
}
