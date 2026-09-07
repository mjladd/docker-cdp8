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

//! Program lifecycle for the CDP System.
//!
//! This crate is (part of) WP-1.5 of the migration plan
//! (`docs/migration/PLAN.md`). It replaces the outer-shell parts of
//! `legacy/dev/*/main.c` and `legacy/dev/cdp2k/mainfuncs.c`: the exit
//! status categories and the exact stdout shape
//! `print_messages_and_close_sndfiles` prints for each one.
//!
//! Current scope (see `docs/migration/STATUS.md` for the live list):
//! [`error::ExitCategory`], [`error::CdpError`] (with `From` impls for
//! every `cdp-params` and `cdp-data` error this crate's one wired-up
//! program, `synth wave` in `cdp-programs`, can produce), and
//! [`error::report_and_exit`]. Deliberately not attempted yet: a
//! `Context` struct replacing legacy's `dz`. `docs/migration/PLAN.md`
//! describes one holding "the parsed command, the open inputs and
//! outputs, the parameter tables, and a progress reporter", but with
//! only one program wired up so far there is no second real case to
//! confirm that shape against -- the same reasoning
//! `cdp-params::CommandSpec`'s module doc gives for deferring its
//! general TOML loader. `cdp-programs::synth::wave` holds its own
//! state directly instead. Revisit once a second program WP needs
//! shared lifecycle state.

pub mod error;

pub use error::{CdpError, ExitCategory, report_and_exit};
