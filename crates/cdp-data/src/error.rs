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

//! Error type for `cdp-data`.
//!
//! Message text matches `legacy/dev/cdp2k/readdata.c`'s
//! `sprintf(errstr, ...)` calls exactly, `%lf`/`%f` formatting
//! included (C's default is six decimal places), so golden tests
//! comparing error text against the legacy programs can match it
//! verbatim.

use std::io;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Can't open brkpntfile {path} to read data.")]
    CannotOpen { path: String, source: io::Error },

    /// legacy: `get_brkpnt_data_from_file_and_test_it`, the check
    /// `*p != dz->brk[paramno] && *p <= lasttime` -- times must
    /// strictly increase; two equal times are also rejected, not
    /// just a decrease.
    #[error(
        "Times ({lasttime:.6} & {newtime:.6}) in brkpntfile {path} are not in increasing order."
    )]
    TimesNotIncreasing {
        path: String,
        lasttime: f64,
        newtime: f64,
    },

    /// legacy: `out_of_range_in_brkfile`.
    #[error("Value ({value:.6}) out of range ({lo:.6} to {hi:.6}) in brkpntfile {path}.")]
    OutOfRange {
        path: String,
        value: f64,
        lo: f64,
        hi: f64,
    },

    #[error("No data in brkpnt file {0}")]
    NoData(String),

    #[error("Data not paired correctly in file {0}")]
    Unpaired(String),

    /// legacy: `convert_dB_at_or_below_zero_to_gain`.
    #[error("dB value out of range (> 0dB)")]
    DbAboveZero,

    #[error("empty breakpoint table")]
    EmptyTable,
}

pub type Result<T> = std::result::Result<T, DataError>;
