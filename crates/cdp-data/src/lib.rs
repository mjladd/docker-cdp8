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

//! Text data formats for the CDP System.
//!
//! This crate is WP-1.2 of the migration plan
//! (`docs/migration/PLAN.md`). It replaces the text-data-parsing
//! parts of `legacy/dev/cdp2k` (`readdata.c`, `tklib1.c`, `tklib3.c`).
//!
//! Current scope (see `docs/migration/STATUS.md` for the live list):
//! breakpoint files (time/value pairs) and mix files (one line per
//! sound-file event in a `submix mix` mix) -- parsing and range
//! validation, plus breakpoint evaluation, sharing the same number
//! tokenizer every CDP text format is built on. Texture note-data
//! files, tuning files, and plain number lists are not implemented
//! yet.

pub mod breakpoint;
pub mod error;
pub mod mix;
pub mod tokenizer;

pub use breakpoint::{
    BreakpointTable, MAX_DB_ON_16_BIT, MIN_DB_ON_16_BIT, db_to_gain, db_to_gain_allowing_boost,
};
pub use error::{DataError, Result};
pub use mix::{MixEvent, MixFile};
pub use tokenizer::{FLTERR, flteq, next_float, parse_line_floats};
