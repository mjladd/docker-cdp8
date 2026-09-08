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

//! Shared dispatch for the `cdp` binary and the legacy-name launchers
//! (`docs/migration/PLAN.md`, decision D3). One real binary's worth of
//! logic, built twice: once as `cdp` (`src/bin/cdp.rs`, the unified
//! sub-command tree) and once as `synth` (`src/bin/synth.rs`, a
//! launcher that behaves as the legacy `synth` executable would).
//!
//! Current scope (see `docs/migration/STATUS.md`): only `synth wave`
//! is registered ([`cdp_programs::synth::wave`], WP-1.5's one proof-
//! of-life program). Every other program name reports "not
//! implemented yet" rather than legacy's own usage text, since this
//! crate does not (yet) know the full set of legacy program/
//! sub-command names -- that comes from `spec/usage/`, captured by
//! WP-0.2, one program at a time as each is wired up here.

use cdp_core::{CdpError, ExitCategory, report_and_exit};
use cdp_params::{CommandSpec, parse, parse_mode};
use cdp_programs::synth::wave::{self, Mode};

/// legacy: each program's own `cdp_version` constant in its
/// `main.c` (e.g. `legacy/dev/synth/main.c`) -- confirmed live via
/// `docker run cdp8-postmerge synth --version`. Grows as more
/// launchers are added; there is no single "legacy version" to fall
/// back on; a program not listed here has no launcher yet.
fn legacy_program_version(program: &str) -> Option<&'static str> {
    match program {
        "synth" => Some("7.1.1"),
        _ => None,
    }
}

/// Entry point for a legacy-name launcher binary (e.g. `src/bin/
/// synth.rs`). `args` is everything after the binary's own name
/// (`std::env::args().skip(1)`, already collected by the caller).
/// legacy: `argv[0]` tells the real `synth`/`modify`/... binary which
/// program it is; the equivalent fact here is simply which launcher
/// binary got built and run, so `program` is passed in rather than
/// read back out of `argv[0]`.
pub fn run_launcher(program: &str, args: &[String]) -> ! {
    if args.len() == 1 && args[0] == "--version" {
        match legacy_program_version(program) {
            Some(version) => {
                println!("{version}");
                std::process::exit(0);
            }
            None => {
                eprintln!("cdp: no recorded legacy version for '{program}'");
                std::process::exit(1);
            }
        }
    }
    dispatch(program, args)
}

/// Entry point for the unified `cdp` binary. `args` is everything
/// after `cdp` itself.
pub fn run_unified(args: &[String]) -> ! {
    match args.first().map(String::as_str) {
        Some("--version") => {
            println!("cdp {}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        Some("--help") | None => {
            print_top_level_help();
            std::process::exit(if args.is_empty() { 1 } else { 0 });
        }
        Some(program) => dispatch(program, &args[1..]),
    }
}

fn print_top_level_help() {
    println!("USAGE: cdp <program> <sub-command> [mode] [arguments...]");
    println!();
    println!("Programs implemented so far:");
    println!("  synth wave    generate a sine, square, sawtooth or ramp waveform");
}

fn dispatch(program: &str, args: &[String]) -> ! {
    match program {
        "synth" => dispatch_synth(args),
        _ => {
            eprintln!("cdp: '{program}' is not implemented yet");
            std::process::exit(1);
        }
    }
}

fn dispatch_synth(args: &[String]) -> ! {
    match args.split_first() {
        Some((subcommand, rest)) if subcommand == "wave" => dispatch_synth_wave(rest),
        Some((subcommand, _)) => {
            eprintln!("synth: '{subcommand}' is not implemented yet (only 'wave' is)");
            std::process::exit(1);
        }
        None => {
            eprintln!("synth: missing sub-command (only 'wave' is implemented)");
            std::process::exit(1);
        }
    }
}

fn dispatch_synth_wave(args: &[String]) -> ! {
    let Some((mode_token, rest)) = args.split_first() else {
        // legacy: confirmed live, `synth wave` with no further
        // arguments at all prints its usage text with no "ERROR:"
        // header -- see `wave::USAGE`'s doc for why this is a literal
        // fixture rather than a generated usage message.
        // `ExitCategory::UsageOnly` already prints a message exactly
        // this way (no header, still followed by the unconditional
        // tail blank line), so route through it rather than
        // duplicating that shape here.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, wave::USAGE)));
    };
    report_and_exit(run_synth_wave(mode_token, rest))
}

fn run_synth_wave(mode_token: &str, args: &[String]) -> Result<(), CdpError> {
    let mode_number = parse_mode(mode_token, wave::MAX_MODE)?;
    let mode = Mode::from_number(mode_number)
        .expect("parse_mode already checked mode_number is in 1..=MAX_MODE");
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let parsed = parse(&CommandSpec::synth_wave(), &args)?;
    let outfile = parsed.outfile.clone();
    let writer = wave::synthesize(mode, &parsed)?;
    writer.finalize(&outfile).map_err(CdpError::from)
}
