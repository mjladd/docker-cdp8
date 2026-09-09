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
//! Current scope (see `docs/migration/STATUS.md`): `synth wave`
//! ([`cdp_programs::synth::wave`], WP-1.5's one proof-of-life program),
//! `pvoc anal` ([`cdp_programs::pvoc::anal`], WP-1.4, mode 1/mono only)
//! and `sndinfo props`/`len`/`smptime`/`timesmp`/`timediff`/`lens`/`sumlen`
//! ([`cdp_programs::sndinfo`], WP-2.1). Every other program name reports "not implemented yet" rather than
//! legacy's own usage text, since this crate does not (yet) know the
//! full set of legacy program/sub-command names -- that comes from
//! `spec/usage/`, captured by WP-0.2, one program at a time as each is
//! wired up here.

use cdp_core::{CdpError, ExitCategory, report_and_exit};
use cdp_params::{CommandSpec, parse, parse_mode};
use cdp_programs::pvoc::anal;
use cdp_programs::sndinfo::{len, lens, props, smptime, sumlen, timediff, timesmp};
use cdp_programs::synth::wave::{self, Mode};
use cdp_sf::SoundFile;

/// legacy: each program's own `cdp_version` constant in its
/// `main.c` (e.g. `legacy/dev/synth/main.c`) -- confirmed live via
/// `docker run cdp8-postmerge synth --version`. Grows as more
/// launchers are added; there is no single "legacy version" to fall
/// back on; a program not listed here has no launcher yet.
fn legacy_program_version(program: &str) -> Option<&'static str> {
    match program {
        "synth" => Some("7.1.1"),
        "sndinfo" => Some("7.1.0"),
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
    println!("  pvoc anal     analyse a sound file into a phase-vocoder file (mode 1, mono only)");
    println!("  sndinfo props display a sound or analysis file's properties");
    println!("  sndinfo len   display a sound or analysis file's duration");
    println!("  sndinfo smptime convert a sample count to a duration");
    println!("  sndinfo timesmp convert a duration to a sample count");
    println!("  sndinfo timediff show the duration difference between two sound files");
    println!("  sndinfo lens   list the duration of two or more sound files");
    println!("  sndinfo sumlen sum the duration of two or more sound files");
}

fn dispatch(program: &str, args: &[String]) -> ! {
    match program {
        "synth" => dispatch_synth(args),
        "pvoc" => dispatch_pvoc(args),
        "sndinfo" => dispatch_sndinfo(args),
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
    let outfile = parsed
        .outfile
        .clone()
        .expect("CommandSpec::synth_wave has_outfile: true");
    let writer = wave::synthesize(mode, &parsed)?;
    writer.finalize(&outfile).map_err(CdpError::from)
}

fn dispatch_pvoc(args: &[String]) -> ! {
    match args.split_first() {
        Some((subcommand, rest)) if subcommand == "anal" => dispatch_pvoc_anal(rest),
        Some((subcommand, _)) => {
            eprintln!("pvoc: '{subcommand}' is not implemented yet (only 'anal' is)");
            std::process::exit(1);
        }
        None => {
            eprintln!("pvoc: missing sub-command (only 'anal' is implemented)");
            std::process::exit(1);
        }
    }
}

fn dispatch_sndinfo(args: &[String]) -> ! {
    match args.split_first() {
        Some((subcommand, rest)) if subcommand == "props" => dispatch_sndinfo_props(rest),
        Some((subcommand, rest)) if subcommand == "len" => dispatch_sndinfo_len(rest),
        Some((subcommand, rest)) if subcommand == "smptime" => dispatch_sndinfo_smptime(rest),
        Some((subcommand, rest)) if subcommand == "timesmp" => dispatch_sndinfo_timesmp(rest),
        Some((subcommand, rest)) if subcommand == "timediff" => dispatch_sndinfo_timediff(rest),
        Some((subcommand, rest)) if subcommand == "lens" => dispatch_sndinfo_lens(rest),
        Some((subcommand, rest)) if subcommand == "sumlen" => dispatch_sndinfo_sumlen(rest),
        Some((subcommand, _)) => {
            eprintln!(
                "sndinfo: '{subcommand}' is not implemented yet (only 'props'/'len'/'smptime'/'timesmp'/'timediff'/'lens'/'sumlen' are)"
            );
            std::process::exit(1);
        }
        None => {
            eprintln!(
                "sndinfo: missing sub-command (only 'props'/'len'/'smptime'/'timesmp'/'timediff'/'lens'/'sumlen' are implemented)"
            );
            std::process::exit(1);
        }
    }
}

fn dispatch_pvoc_anal(args: &[String]) -> ! {
    let Some((mode_token, rest)) = args.split_first() else {
        // legacy: same bare-subcommand usage-text shape as `synth
        // wave` -- see `run_synth_wave`'s own comment on
        // `ExitCategory::UsageOnly`.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, anal::USAGE)));
    };
    report_and_exit(run_pvoc_anal(mode_token, rest))
}

fn run_pvoc_anal(mode_token: &str, args: &[String]) -> Result<(), CdpError> {
    let mode_number = parse_mode(mode_token, anal::MAX_MODE)?;
    let mode = anal::Mode::from_number(mode_number)
        .expect("parse_mode already checked mode_number is in 1..=MAX_MODE");
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let parsed = parse(&CommandSpec::pvoc_anal(), &args)?;
    let outfile = parsed
        .outfile
        .clone()
        .expect("CommandSpec::pvoc_anal has_outfile: true");
    let writer = anal::analyze(mode, &parsed)?;
    writer.finalize(&outfile).map_err(CdpError::from)
}

fn dispatch_sndinfo_props(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: confirmed live, a bare `sndinfo props` prints its
        // usage text with no "ERROR:" header -- see
        // `run_synth_wave`'s own comment on `ExitCategory::UsageOnly`.
        // `props::USAGE` bundles the greeting itself (see its doc).
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, props::USAGE)));
    }
    if args.len() == 1 {
        // legacy: `make_initial_cmdline_check`'s `argc<4` greeting --
        // true for `sndinfo props infile` (argc 3) regardless of
        // whether the run goes on to succeed, confirmed live including
        // for a nonexistent infile. False once a further token makes
        // argc 4 or more (confirmed live: `sndinfo props infile extra`
        // prints no greeting at all, straight to `"Too many
        // parameters..."`), so `run_sndinfo_props`'s own `parse` call
        // below is what produces that error text for that case.
        print!("{}", props::GREETING);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_props(&args));
}

fn run_sndinfo_props(args: &[&str]) -> Result<(), CdpError> {
    let parsed = parse(&CommandSpec::sndinfo_props(), args)?;
    let sf = SoundFile::open(&parsed.infiles[0])?;
    let text = props::format_props(&sf)?;
    print!("{text}");
    Ok(())
}

fn dispatch_sndinfo_len(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, len::USAGE)));
    }
    if args.len() == 1 {
        // legacy: same `argc<4` greeting rule as `sndinfo props` -- see
        // `dispatch_sndinfo_props`'s own comment.
        print!("{}", len::GREETING);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_len(&args));
}

fn run_sndinfo_len(args: &[&str]) -> Result<(), CdpError> {
    let parsed = parse(&CommandSpec::sndinfo_len(), args)?;
    let sf = SoundFile::open(&parsed.infiles[0])?;
    let text = len::format_len(&sf)?;
    print!("{text}");
    Ok(())
}

fn dispatch_sndinfo_smptime(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, smptime::USAGE)));
    }
    if args.len() == 1 {
        // legacy: same `argc<4` greeting rule as `sndinfo props` -- see
        // `dispatch_sndinfo_props`'s own comment. Confirmed live even
        // though this case goes on to fail with "Insufficient
        // parameters on command line." (the required `samplecnt` is
        // missing), since the greeting prints before that check runs.
        print!("{}", smptime::GREETING);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_smptime(&args));
}

fn run_sndinfo_smptime(args: &[&str]) -> Result<(), CdpError> {
    // legacy: the infile must be open (to compute `samplecnt`'s
    // dynamic range) before `parse` itself can run -- see
    // `cdp_programs::sndinfo::open_sound_infile`'s doc.
    let sf = cdp_programs::sndinfo::open_sound_infile(args[0])?;
    let spec = CommandSpec::sndinfo_smptime(sf.sample_count() as f64);
    let parsed = parse(&spec, args)?;
    print!("{}", smptime::format_smptime(&sf, &parsed));
    Ok(())
}

fn dispatch_sndinfo_timesmp(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, timesmp::USAGE)));
    }
    if args.len() == 1 {
        // legacy: same `argc<4` greeting rule as `sndinfo smptime` --
        // see `dispatch_sndinfo_smptime`'s own comment.
        print!("{}", timesmp::GREETING);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_timesmp(&args));
}

fn run_sndinfo_timesmp(args: &[&str]) -> Result<(), CdpError> {
    let sf = cdp_programs::sndinfo::open_sound_infile(args[0])?;
    let spec = CommandSpec::sndinfo_timesmp(cdp_programs::sndinfo::len::wave_duration_secs(&sf));
    let parsed = parse(&spec, args)?;
    print!("{}", timesmp::format_timesmp(&sf, &parsed));
    Ok(())
}

fn dispatch_sndinfo_timediff(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        // `timediff::USAGE`'s own doc explains why this is the *only*
        // case that ever prints the greeting for `timediff`, unlike
        // every other `sndinfo` sub-command ported so far.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, timediff::USAGE)));
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_timediff(&args));
}

fn run_sndinfo_timediff(args: &[&str]) -> Result<(), CdpError> {
    // legacy: `timediff::open_infiles`'s own module doc explains why
    // this command's argument handling is written by hand instead of
    // going through `cdp_params::parse` -- including why a single
    // infile reports a clean error here rather than reproducing a
    // real legacy segfault.
    match args.len() {
        1 => {
            return Err(CdpError::from(
                cdp_params::ParamsError::InsufficientCmdlineParameters,
            ));
        }
        2 => {}
        _ => return Err(CdpError::from(cdp_params::ParamsError::TooManyParameters)),
    }
    let (sf1, sf2) = timediff::open_infiles(args[0], args[1])?;
    print!("{}", timediff::format_timediff(&sf1, &sf2));
    Ok(())
}

fn dispatch_sndinfo_lens(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, lens::USAGE)));
    }
    if args.len() == 1 {
        // legacy: same `argc<4` greeting rule as `sndinfo props`, but
        // -- unlike `props`/`len` -- `sndinfo lens infile` (one token)
        // still fails, with "Insufficient parameters on command
        // line." rather than proceeding -- see `lens`'s own module
        // doc for why.
        print!("{}", lens::GREETING);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_lens(&args));
}

fn run_sndinfo_lens(args: &[&str]) -> Result<(), CdpError> {
    // legacy: `lens`'s own module doc explains why this command's
    // argument handling is written by hand instead of going through
    // `cdp_params::parse`, and why exactly one token is a different,
    // earlier error than two-or-more tokens that still add up to
    // fewer than two real infiles.
    if args.len() == 1 {
        return Err(CdpError::from(
            cdp_params::ParamsError::InsufficientParameters,
        ));
    }
    let (paths, trailing) = cdp_programs::sndinfo::split_infile_tokens(args);
    if paths.len() < 2 {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            cdp_programs::sndinfo::INSUFFICIENT_INFILES,
        ));
    }
    let files = lens::open_infiles(paths)?;
    if let Some(&token) = trailing.first() {
        return Err(CdpError::from(cdp_params::classify_trailing_token(token)));
    }
    print!("{}", lens::format_lens(paths, &files));
    Ok(())
}

fn dispatch_sndinfo_sumlen(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, sumlen::USAGE)));
    }
    if args.len() == 1 {
        // legacy: same `argc<4` greeting rule as `sndinfo lens` -- see
        // `dispatch_sndinfo_lens`'s own comment.
        print!("{}", sumlen::GREETING);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_sumlen(&args));
}

fn run_sndinfo_sumlen(args: &[&str]) -> Result<(), CdpError> {
    // legacy: same shape as `run_sndinfo_lens` -- see that function's
    // and `sumlen`'s own module doc.
    if args.len() == 1 {
        return Err(CdpError::from(
            cdp_params::ParamsError::InsufficientParameters,
        ));
    }
    let (paths, trailing) = cdp_programs::sndinfo::split_infile_tokens(args);
    if paths.len() < 2 {
        return Err(CdpError::new(
            ExitCategory::UsageOnly,
            cdp_programs::sndinfo::INSUFFICIENT_INFILES,
        ));
    }
    let files = sumlen::open_infiles(paths)?;
    let splice_ms = sumlen::parse_splice_ms(trailing)?;
    print!("{}", sumlen::format_sumlen(&files, splice_ms));
    Ok(())
}
