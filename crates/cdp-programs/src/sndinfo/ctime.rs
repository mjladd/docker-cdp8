// This file is part of a Rust reimplementation of the CDP System, a
// Composers Desktop Project (CDP) command-line sound-processing suite
// originally written by Trevor Wishart, Richard Dobson, Martin Atkins
// and others (see legacy/dev for the original C source and its
// per-file copyright notices).
//
// SPDX-License-Identifier: LGPL-2.1-or-later
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 2.1 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

//! A from-scratch port of C's `ctime`/`asctime`, the exact text shape
//! `legacy/dev/sndinfo/compare.c`'s `prntprops` uses for both the
//! `DATE` property and the `PEAK` chunk timestamp
//! (`"Www Mmm dd hh:mm:ss yyyy\n"`, day-of-month space-padded to width
//! 2, `\n`-terminated). `legacy` and this crate's own live-verification
//! Docker image both run with no `TZ` set, which glibc treats as UTC,
//! so this is a UTC conversion, not a local-time one -- consistent
//! with `cdp-sf`'s own `DATE`-property doc comments, which independently
//! confirm the same corpus timestamps against UTC.
//!
//! No date/time crate is a workspace dependency (checked before
//! writing this), so the calendar conversion is implemented directly:
//! Howard Hinnant's `civil_from_days` (a standard, widely-used
//! constant-time proleptic-Gregorian day-count-to-date algorithm), not
//! a loop over years/months.

const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Floor division, for the day-count conversion below (`/` in Rust
/// truncates toward zero, but this algorithm needs floor division to
/// stay correct for negative inputs, i.e. dates before 1970).
fn div_floor(a: i64, b: i64) -> i64 {
    let q = a / b;
    let r = a % b;
    if (r != 0) && ((r < 0) != (b < 0)) {
        q - 1
    } else {
        q
    }
}

/// Days since the Unix epoch (1970-01-01) to a proleptic-Gregorian
/// `(year, month, day)`, all 1-based except `year`. Howard Hinnant's
/// `civil_from_days` algorithm, ported as published (see
/// <https://howardhinnant.github.io/date_algorithms.html>).
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = div_floor(if z >= 0 { z } else { z - 146_096 }, 146_097);
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365; // [0, 399]
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2) / 153; // [0, 11]
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32; // [1, 31]
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32; // [1, 12]
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// legacy: `ctime`/`asctime`'s exact text, for a Unix timestamp
/// already read as a plain `i32` (the `DATE` property's and the
/// `PEAK` chunk's on-disk type).
pub fn format(timestamp: i32) -> String {
    let timestamp = timestamp as i64;
    let days = div_floor(timestamp, 86_400);
    let secs_of_day = timestamp - days * 86_400;
    let hour = secs_of_day / 3600;
    let minute = (secs_of_day % 3600) / 60;
    let second = secs_of_day % 60;
    // legacy: 1970-01-01 was a Thursday (index 4, Sunday-indexed).
    let weekday = WEEKDAYS[(days + 4).rem_euclid(7) as usize];
    let (year, month, day) = civil_from_days(days);
    format!(
        "{weekday} {} {day:2} {hour:02}:{minute:02}:{second:02} {year}\n",
        MONTHS[(month - 1) as usize]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every value here is an independent `date -u -d @<ts>` run, not
    /// derived from this module's own code.
    #[test]
    fn matches_date_dash_u_for_a_spread_of_real_and_edge_timestamps() {
        let cases: &[(i32, &str)] = &[
            (963_996_604, "Wed Jul 19 08:50:04 2000\n"), // docs/manual/sounds/marimba.wav's DATE
            (1_151_268_522, "Sun Jun 25 20:48:42 2006\n"), // docs/manual/sounds/ws2/tsw1-2nd.aiff's PEAK/DATE
            (1_641_317_196, "Tue Jan  4 17:26:36 2022\n"), // docs/manual/data/capm.ana's DATE
            (0, "Thu Jan  1 00:00:00 1970\n"),             // the epoch itself
            (86_399, "Thu Jan  1 23:59:59 1970\n"),        // last second of day 0
            (86_400, "Fri Jan  2 00:00:00 1970\n"),        // first second of day 1
            (1_000_000_000, "Sun Sep  9 01:46:40 2001\n"),
            (1_700_000_000, "Tue Nov 14 22:13:20 2023\n"),
            (951_782_400, "Tue Feb 29 00:00:00 2000\n"), // century leap year (div by 400)
            (1_582_934_400, "Sat Feb 29 00:00:00 2020\n"), // ordinary leap year
        ];
        for &(ts, expected) in cases {
            assert_eq!(format(ts), expected, "timestamp {ts}");
        }
    }
}
