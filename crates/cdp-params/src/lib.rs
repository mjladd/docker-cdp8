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

//! Command-line argument parsing for the CDP System.
//!
//! This crate is WP-1.3 of the migration plan
//! (`docs/migration/PLAN.md`). It replaces the command-line-argument
//! parts of `legacy/dev/cdp2k` (`parstruct.c`'s parameter-type
//! declarations, `tklib1.c`'s `set_param_ranges`, and the count/range
//! checks in `mainfuncs.c`/`tklib1.c`).
//!
//! Current scope (see `docs/migration/STATUS.md` for the live list):
//! four hand-written [`CommandSpec`]s -- all `modify loudness` modes
//! (mode 1 `LOUDNESS_GAIN`, modes 3/4 `LOUDNESS_NORM`/`LOUDNESS_SET`)
//! and `pvoc anal` (all three modes share one spec) -- and a parser
//! handling four of PLAN.md's five parameter types
//! ([`ParamType::DoubleOrBreakpoint`], [`ParamType::Double`],
//! [`ParamType::Int`], [`ParamType::File`]) plus multiple simultaneous
//! optional `-<letter><value>` flags, including duplicate-flag
//! detection, all confirmed against live `legacy` runs (see `parser`'s
//! module doc). The general `CommandSpec`-from-TOML loader,
//! `IntOrBreakpoint`, variant flags, a mode with both required
//! positional parameters and optional flags, and usage-text formatting
//! are not implemented yet.

pub mod error;
pub mod parser;
pub mod spec;

pub use error::{ParamsError, Result};
pub use parser::{ParamValue, ParsedCommand, parse};
pub use spec::{CommandSpec, OptionFlag, ParamType};
