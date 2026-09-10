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

//! `housekeep` (legacy: `legacy/dev/houskeep`, WP-2.2). Only `copy`
//! mode 1 (`housekeep copy 1 infile outfile`) is ported so far, of
//! the thirteen sub-commands `get_process_no`
//! (`legacy/dev/houskeep/ap_house.c`) recognises: `copy`, `remove`,
//! `chans`, `bundle`, `sort`, `respec`, `extract`, `bakup`, `gate`,
//! `disk`, `batchexpand`, `endclicks`, `deglitch`.

pub mod copy;

/// legacy: the usage text `legacy/dev/houskeep/main.c` prints for a
/// bare `housekeep` (captured live via `spec/usage/housekeep.txt`,
/// WP-0.2; re-confirmed live this slice). One fewer trailing blank
/// line than the raw capture, same reason as every `USAGE` constant
/// elsewhere in this crate -- `cdp_core::ExitCategory::UsageOnly`
/// already appends its own tail blank line.
pub const USAGE: &str = "CDP Release 7.1 2016
USAGE: housekeep NAME (mode) infile(s) (outfile) (parameters)

where NAME can be any one of

extract  copy   remove   chans   respec   bundle   sort   disk
bakup    gate    batchexpand    endclicks    deglitch

Type 'housekeep chans'  for more info on housekeep chans option... ETC.
";
