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

//! CDP breakpoint files: text files of `time value` pairs used
//! throughout CDP for time-varying parameters. legacy: parsing is
//! `get_brkpnt_data_from_file_and_test_it` in
//! `legacy/dev/cdp2k/readdata.c`; evaluating a table at a given time
//! is `read_value_from_brktable` in `legacy/dev/cdp2k/tklib3.c` (see
//! [`BreakpointTable::interpolate`]'s doc for why that one, and not
//! the similarly-named `interp_val` in `readdata.c`).

use crate::error::{DataError, Result};
use crate::tokenizer::{flteq, parse_line_floats};
use std::path::Path;

/// A parsed, range-checked breakpoint table: `(time, value)` pairs
/// with strictly increasing times.
#[derive(Debug, Clone, PartialEq)]
pub struct BreakpointTable {
    /// Flat `[time0, value0, time1, value1, ...]`, matching the
    /// legacy `dz->brk[paramno]` layout, which several call sites
    /// (`get_maxvalue_in_brktable` and friends) rely on directly.
    pairs: Vec<f64>,
}

impl BreakpointTable {
    /// Parses breakpoint data from `text` and checks every value is
    /// within `[lo, hi]`, snapping a value within [`crate::tokenizer::FLTERR`]
    /// of a bound to that exact bound. `source_name` is used only in
    /// error messages, to match the legacy text
    /// (`"...brkpntfile %s..."`).
    ///
    /// legacy: `get_brkpnt_data_from_file_and_test_it`. Does not
    /// implement the legacy function's line-length-driven
    /// reallocation (`BIGARRAY` chunks) since that is a C memory-
    /// management detail with no externally visible effect: parsing
    /// the same input produces the same pairs regardless of how the
    /// backing buffer grew.
    pub fn parse(text: &str, lo: f64, hi: f64, source_name: &str) -> Result<Self> {
        let mut pairs = Vec::new();
        let mut is_time = true;
        let mut last_time = 0.0f64;

        for line in text.lines() {
            for value in parse_line_floats(line) {
                if is_time {
                    if !pairs.is_empty() && value <= last_time {
                        return Err(DataError::TimesNotIncreasing {
                            path: source_name.to_string(),
                            lasttime: last_time,
                            newtime: value,
                        });
                    }
                    last_time = value;
                    pairs.push(value);
                } else {
                    let mut v = value;
                    if flteq(v, lo) {
                        v = lo;
                    } else if flteq(v, hi) {
                        v = hi;
                    }
                    if v < lo || v > hi {
                        return Err(DataError::OutOfRange {
                            path: source_name.to_string(),
                            value: v,
                            lo,
                            hi,
                        });
                    }
                    pairs.push(v);
                }
                is_time = !is_time;
            }
        }

        if pairs.is_empty() {
            return Err(DataError::NoData(source_name.to_string()));
        }
        if pairs.len() % 2 != 0 {
            return Err(DataError::Unpaired(source_name.to_string()));
        }
        Ok(BreakpointTable { pairs })
    }

    /// As [`Self::parse`], reading `path` first. legacy: the
    /// `fopen`/`"Can't open brkpntfile %s to read data.\n"` half of
    /// `get_brkpnt_data_from_file_and_test_it`.
    pub fn from_file(path: impl AsRef<Path>, lo: f64, hi: f64) -> Result<Self> {
        let path = path.as_ref();
        let text = std::fs::read_to_string(path).map_err(|source| DataError::CannotOpen {
            path: path.display().to_string(),
            source,
        })?;
        Self::parse(&text, lo, hi, &path.display().to_string())
    }

    pub fn len(&self) -> usize {
        self.pairs.len() / 2
    }

    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    pub fn point(&self, i: usize) -> (f64, f64) {
        (self.pairs[i * 2], self.pairs[i * 2 + 1])
    }

    pub fn first_time(&self) -> f64 {
        self.pairs[0]
    }

    pub fn last_time(&self) -> f64 {
        self.pairs[self.pairs.len() - 2]
    }

    pub fn min_value(&self) -> f64 {
        self.pairs
            .iter()
            .skip(1)
            .step_by(2)
            .cloned()
            .fold(f64::INFINITY, f64::min)
    }

    pub fn max_value(&self) -> f64 {
        self.pairs
            .iter()
            .skip(1)
            .step_by(2)
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Linearly interpolates the value at `time`. legacy:
    /// `read_value_from_brktable` in `legacy/dev/cdp2k/tklib3.c`,
    /// the function every group program actually calls to evaluate a
    /// breakpoint parameter each sample or window (not the
    /// similarly-named `interp_val` in `readdata.c`, which is dead
    /// code -- declared and defined, but never called from any
    /// program in this codebase; do not port it, it is not part of
    /// CDP's real behaviour).
    ///
    /// A time at or before the table's first point returns the first
    /// value; at or after the last point, the last value. Between
    /// points, the value is linearly interpolated. This finds the
    /// correct bracketing segment with a binary search rather than
    /// `read_value_from_brktable`'s linear scan from a cached cursor
    /// position, but computes the identical result: both are exact
    /// linear interpolation between the correct pair of points,
    /// regardless of the query time's position relative to any
    /// previous query. [`Self::interpolate_from_cursor`] also ports
    /// `read_value_from_brktable`, keeping its O(1)-amortised cursor
    /// for a caller that queries strictly increasing times, such as
    /// a program advancing sample by sample.
    pub fn interpolate(&self, time: f64) -> Result<f64> {
        if self.is_empty() {
            return Err(DataError::EmptyTable);
        }
        if time <= self.first_time() {
            return Ok(self.point(0).1);
        }
        let last = self.len() - 1;
        if time >= self.last_time() {
            return Ok(self.point(last).1);
        }
        // Binary search for the first point whose time is >= `time`.
        let mut lo = 0usize;
        let mut hi = last;
        while lo + 1 < hi {
            let mid = (lo + hi) / 2;
            if self.point(mid).0 <= time {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let (lo_t, lo_v) = self.point(lo);
        let (hi_t, hi_v) = self.point(hi);
        Ok(lo_v + (time - lo_t) / (hi_t - lo_t) * (hi_v - lo_v))
    }

    /// A faithful port of `read_value_from_brktable`'s cursor
    /// optimisation: `cursor` is a point index (pass `0` on the first
    /// call) that this scans forward *or backward* from to find the
    /// segment bracketing `time`, then advances to sit at the
    /// segment's low point for next time. Unlike the dead
    /// `interp_val` in `readdata.c` (see [`Self::interpolate`]'s
    /// doc), the real, actually-used function handles a query time
    /// moving in either direction correctly -- it is an O(1)-amortised
    /// optimisation for sequential access, not a source of the
    /// wrong-segment bug a naive forward-only cursor would have.
    /// Returns the same value [`Self::interpolate`] would for the
    /// same `time`, plus the cursor to pass next time.
    pub fn interpolate_from_cursor(&self, time: f64, cursor: usize) -> Result<(f64, usize)> {
        if self.is_empty() {
            return Err(DataError::EmptyTable);
        }
        let last = self.len() - 1;
        if time <= self.first_time() {
            return Ok((self.point(0).1, cursor.min(last)));
        }
        if time >= self.last_time() {
            return Ok((self.point(last).1, cursor.min(last)));
        }

        let mut p = cursor.min(last);
        if time > self.point(p).0 {
            while self.point(p).0 < time {
                p += 1;
            }
        } else {
            while self.point(p).0 >= time {
                p -= 1;
            }
            p += 1;
        }
        let (hi_t, hi_v) = self.point(p);
        let (lo_t, lo_v) = self.point(p - 1);
        let value = lo_v + (time - lo_t) / (hi_t - lo_t) * (hi_v - lo_v);
        // legacy: dz->brkptr[paramno] is left pointing at the high
        // bound's time slot, i.e. point index p, not p - 1.
        Ok((value, p))
    }
}

/// legacy: `MIN_DB_ON_16_BIT` in `legacy/dev/include/globcon.h`.
pub const MIN_DB_ON_16_BIT: f64 = -96.0;

/// legacy: `convert_dB_at_or_below_zero_to_gain` in
/// `legacy/dev/cdp2k/readdata.c`. A dB value of `0` or above negative
/// infinity converts to a linear gain; anything at or below
/// [`MIN_DB_ON_16_BIT`] converts to `0.0` (silence) rather than a
/// vanishingly small gain, and a positive dB value is an error (this
/// function only accepts attenuation, never boost).
pub fn db_to_gain(db: f64) -> Result<f64> {
    if db > 0.0 {
        Err(DataError::DbAboveZero)
    } else if db <= MIN_DB_ON_16_BIT {
        Ok(0.0)
    } else if flteq(db, 0.0) {
        Ok(1.0)
    } else {
        Ok(1.0 / 10f64.powf(-db / 20.0))
    }
}

/// legacy: `dbtogain` in `legacy/dev/cdp2k/tklib1.c`, the general
/// dB-to-gain conversion used across the codebase (9 call sites).
/// Unlike [`db_to_gain`] (`convert_dB_at_or_below_zero_to_gain`),
/// this accepts a positive `db` (boost, gain > 1.0) rather than
/// treating it as an error; it is otherwise the same conversion.
pub fn db_to_gain_allowing_boost(db: f64) -> f64 {
    if db <= MIN_DB_ON_16_BIT {
        0.0
    } else if flteq(db, 0.0) {
        1.0
    } else if db < 0.0 {
        1.0 / 10f64.powf(-db / 20.0)
    } else {
        10f64.powf(db / 20.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `docs/manual/data/balltv.brk`, this repository's own example
    /// file (see docs/migration/PLAN.md section 5's test-corpus
    /// note).
    const BALLTV: &str = concat!(
        ";balltv.brk\n",
        " 0.0  0.15   ;start fast\n",
        " 3.0  0.40   ;slow down\n",
        " 6.0  0.07   ;very fast\n",
        " 9.0  0.30   ;slow down again\n",
        "12.0  0.10   ;to a little faster than original tempo\n",
    );

    #[test]
    fn parses_the_real_balltv_example_file() {
        let table = BreakpointTable::parse(BALLTV, 0.0, 1.0, "balltv.brk").unwrap();
        assert_eq!(table.len(), 5);
        assert_eq!(table.point(0), (0.0, 0.15));
        assert_eq!(table.point(2), (6.0, 0.07));
        assert_eq!(table.point(4), (12.0, 0.10));
        assert_eq!(table.min_value(), 0.07);
        assert_eq!(table.max_value(), 0.40);
    }

    #[test]
    fn rejects_non_increasing_times() {
        let err = BreakpointTable::parse("0.0 1.0\n0.0 2.0\n", 0.0, 10.0, "t.brk").unwrap_err();
        assert!(matches!(err, DataError::TimesNotIncreasing { .. }));
    }

    #[test]
    fn equal_times_are_rejected_not_just_decreasing_ones() {
        // legacy: the check is `<=`, not `<`.
        let err = BreakpointTable::parse("1.0 1.0\n1.0 2.0\n", 0.0, 10.0, "t.brk").unwrap_err();
        assert!(matches!(err, DataError::TimesNotIncreasing { .. }));
    }

    #[test]
    fn rejects_out_of_range_values() {
        let err = BreakpointTable::parse("0.0 5.0\n", 0.0, 1.0, "t.brk").unwrap_err();
        assert!(matches!(err, DataError::OutOfRange { .. }));
    }

    #[test]
    fn snaps_values_within_flterr_of_a_bound() {
        // 1.0000015 is within FLTERR (0.000002) of 1.0.
        let table = BreakpointTable::parse("0.0 1.0000015\n", 0.0, 1.0, "t.brk").unwrap();
        assert_eq!(table.point(0).1, 1.0);
    }

    #[test]
    fn empty_input_is_an_error() {
        assert!(matches!(
            BreakpointTable::parse("", 0.0, 1.0, "t.brk"),
            Err(DataError::NoData(_))
        ));
        assert!(matches!(
            BreakpointTable::parse(";only a comment\n", 0.0, 1.0, "t.brk"),
            Err(DataError::NoData(_))
        ));
    }

    #[test]
    fn odd_number_of_values_is_unpaired() {
        let err = BreakpointTable::parse("0.0 1.0 2.0\n", 0.0, 10.0, "t.brk").unwrap_err();
        assert!(matches!(err, DataError::Unpaired(_)));
    }

    /// Cross-checked against a real run of `legacy` `modify loudness
    /// 1 in.wav out.wav sci.brk` with this exact file content, which
    /// fails with "ERROR: Data not paired correctly in file
    /// sci.brk" -- confirming that `1e-5` really does contribute
    /// zero values (not a partial `1.0`), since the file has 3
    /// surviving values total (`0.0`, `1.0`, `0.5`), an odd count.
    #[test]
    fn scientific_notation_in_a_real_file_reproduces_the_legacy_unpaired_error() {
        let err = BreakpointTable::parse("0.0 1e-5\n1.0 0.5\n", 0.0, 10.0, "sci.brk").unwrap_err();
        assert!(matches!(err, DataError::Unpaired(_)));
    }

    #[test]
    fn interpolate_matches_hand_computed_linear_interpolation() {
        let table = BreakpointTable::parse("0.0 0.0\n10.0 100.0\n", 0.0, 200.0, "t.brk").unwrap();
        assert_eq!(table.interpolate(5.0).unwrap(), 50.0);
        assert_eq!(table.interpolate(2.5).unwrap(), 25.0);
    }

    #[test]
    fn interpolate_clamps_outside_the_table_range() {
        let table = BreakpointTable::parse("1.0 10.0\n2.0 20.0\n", 0.0, 100.0, "t.brk").unwrap();
        assert_eq!(table.interpolate(0.0).unwrap(), 10.0);
        assert_eq!(table.interpolate(5.0).unwrap(), 20.0);
    }

    #[test]
    fn cursor_interpolation_matches_plain_interpolation_when_moving_forward() {
        let table = BreakpointTable::parse(BALLTV, 0.0, 1.0, "balltv.brk").unwrap();
        let mut cursor = 0;
        for t in [0.5, 1.0, 4.0, 6.0, 10.0, 11.9] {
            let (from_cursor, next_cursor) = table.interpolate_from_cursor(t, cursor).unwrap();
            cursor = next_cursor;
            assert_eq!(from_cursor, table.interpolate(t).unwrap(), "at t={t}");
        }
    }

    #[test]
    fn db_to_gain_matches_known_values() {
        assert_eq!(db_to_gain(0.0).unwrap(), 1.0);
        assert_eq!(db_to_gain(-96.0).unwrap(), 0.0);
        assert_eq!(db_to_gain(-200.0).unwrap(), 0.0); // at or below MIN_DB_ON_16_BIT
        assert!(db_to_gain(1.0).is_err());
        // -20dB is a gain of 0.1.
        assert!((db_to_gain(-20.0).unwrap() - 0.1).abs() < 1e-9);
    }
}
