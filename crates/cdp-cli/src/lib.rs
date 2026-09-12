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
//! `pvoc anal` ([`cdp_programs::pvoc::anal`], WP-1.4, mode 1/mono only),
//! `sndinfo props`/`len`/`smptime`/`timesmp`/`timediff`/`lens`/`sumlen`/
//! `maxsamp`/`units` (modes 1/2 only) ([`cdp_programs::sndinfo`], WP-2.1)
//! and `housekeep copy`/`chans`/`respec` ([`cdp_programs::housekeep`],
//! WP-2.2). Every other program name reports "not implemented yet" rather than
//! legacy's own usage text, since this crate does not (yet) know the
//! full set of legacy program/sub-command names -- that comes from
//! `spec/usage/`, captured by WP-0.2, one program at a time as each is
//! wired up here.

use cdp_core::{CdpError, ExitCategory, report_and_exit};
use cdp_params::{CommandSpec, ParamValue, ParamsError, parse, parse_mode};
use cdp_programs::housekeep::{self, chans, copy, respec};
use cdp_programs::pvoc::anal;
use cdp_programs::sndinfo::{len, lens, maxsamp, props, smptime, sumlen, timediff, timesmp, units};
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
        "housekeep" => Some("7.1.1"),
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
    println!("  sndinfo units  convert between musical units (modes 1/2 only)");
    println!("  sndinfo maxsamp find the maximum sample in a sound or binary data file");
    println!("  housekeep copy 1  write an unmodified copy of a sound file");
    println!("  housekeep chans 1 extract one channel of a sound file");
    println!("  housekeep respec 2 toggle a sound file between 16-bit and float");
    println!("  housekeep respec 3 change a sound file's declared srate/channels");
}

fn dispatch(program: &str, args: &[String]) -> ! {
    match program {
        "synth" => dispatch_synth(args),
        "pvoc" => dispatch_pvoc(args),
        "sndinfo" => dispatch_sndinfo(args),
        "housekeep" => dispatch_housekeep(args),
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
        Some((subcommand, rest)) if subcommand == "units" => dispatch_sndinfo_units(rest),
        Some((subcommand, rest)) if subcommand == "maxsamp" => dispatch_sndinfo_maxsamp(rest),
        Some((subcommand, _)) => {
            eprintln!(
                "sndinfo: '{subcommand}' is not implemented yet (only 'props'/'len'/'smptime'/'timesmp'/'timediff'/'lens'/'sumlen'/'units'/'maxsamp' are)"
            );
            std::process::exit(1);
        }
        None => {
            eprintln!(
                "sndinfo: missing sub-command (only 'props'/'len'/'smptime'/'timesmp'/'timediff'/'lens'/'sumlen'/'units'/'maxsamp' are implemented)"
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

fn dispatch_sndinfo_units(args: &[String]) -> ! {
    let Some((mode_token, rest)) = args.split_first() else {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, units::USAGE)));
    };
    if rest.is_empty() {
        // legacy: same `argc<4` greeting rule as `sndinfo props`
        // (`sndinfo units <mode>`, with no value, is argc 3) -- see
        // `dispatch_sndinfo_props`'s own comment.
        print!("{}", units::GREETING);
    }
    let rest: Vec<&str> = rest.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_units(mode_token, &rest));
}

fn run_sndinfo_units(mode_token: &str, args: &[&str]) -> Result<(), CdpError> {
    let mode_number = parse_mode(mode_token, units::MAX_MODE)?;
    let Some(mode) = units::Mode::from_number(mode_number) else {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            format!("sndinfo units: mode {mode_number} is not implemented yet"),
        ));
    };
    let (lo, hi) = mode.range();
    let parsed = parse(&CommandSpec::sndinfo_units(lo, hi), args)?;
    let value = match parsed.params[0] {
        ParamValue::Number(n) => n,
        _ => unreachable!("CommandSpec::sndinfo_units's only param is ParamType::Double"),
    };
    units::run(mode, value)
}

fn dispatch_sndinfo_maxsamp(args: &[String]) -> ! {
    if args.is_empty() {
        // legacy: same bare-subcommand usage-text shape as `sndinfo
        // props` -- see `dispatch_sndinfo_props`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, maxsamp::USAGE)));
    }
    if args.len() == 1 {
        // legacy: same `argc<4` greeting rule as `sndinfo props` --
        // see `dispatch_sndinfo_props`'s own comment (`sndinfo maxsamp
        // infile`, with no `-f`, is argc 3).
        print!("{}", maxsamp::GREETING);
    }
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    report_and_exit(run_sndinfo_maxsamp(&args));
}

fn run_sndinfo_maxsamp(args: &[&str]) -> Result<(), CdpError> {
    let parsed = parse(&CommandSpec::sndinfo_maxsamp(), args)?;
    let sf = SoundFile::open(&parsed.infiles[0])?;
    let force_scan = parsed.flags.contains_key(&'f');
    let text = maxsamp::format_maxsamp(&sf, force_scan)?;
    print!("{text}");
    Ok(())
}

fn dispatch_housekeep(args: &[String]) -> ! {
    match args.split_first() {
        Some((subcommand, rest)) if subcommand == "copy" => dispatch_housekeep_copy(rest),
        Some((subcommand, rest)) if subcommand == "chans" => dispatch_housekeep_chans(rest),
        Some((subcommand, rest)) if subcommand == "respec" => dispatch_housekeep_respec(rest),
        Some((subcommand, _)) => {
            eprintln!(
                "housekeep: '{subcommand}' is not implemented yet (only 'copy'/'chans'/'respec' are)"
            );
            std::process::exit(1);
        }
        None => {
            // legacy: a bare `housekeep` prints its own top-level
            // usage text with no "ERROR:" header -- see
            // `run_synth_wave`'s own comment on
            // `ExitCategory::UsageOnly`.
            report_and_exit(Err(CdpError::new(
                ExitCategory::UsageOnly,
                housekeep::USAGE,
            )));
        }
    }
}

fn dispatch_housekeep_copy(args: &[String]) -> ! {
    let Some((mode_token, rest)) = args.split_first() else {
        // legacy: same bare-subcommand usage-text shape as `synth
        // wave` -- see `run_synth_wave`'s own comment.
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, copy::USAGE)));
    };
    if rest.is_empty() {
        // legacy: confirmed live, `housekeep copy <mode>` with no
        // further tokens at all always prints the greeting then
        // "Insufficient parameters on command line.", even for an
        // out-of-range or unparseable mode token (`housekeep copy 0`,
        // `housekeep copy abc`) -- this check runs before mode-number
        // validation even happens, unlike `run_synth_wave`/
        // `run_pvoc_anal`'s current order (a real, pre-existing gap
        // in those two commands this slice found but did not fix,
        // since they belong to WP-1.4/WP-1.5, not this one).
        print!("{}", copy::GREETING);
        report_and_exit(Err(CdpError::from(ParamsError::InsufficientParameters)));
    }
    report_and_exit(run_housekeep_copy(mode_token, rest));
}

fn run_housekeep_copy(mode_token: &str, args: &[String]) -> Result<(), CdpError> {
    let mode_number = parse_mode(mode_token, copy::MAX_MODE)?;
    let Some(mode) = copy::Mode::from_number(mode_number) else {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            format!("housekeep copy: mode {mode_number} is not implemented yet"),
        ));
    };
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match mode {
        copy::Mode::Once => {
            let parsed = parse(&CommandSpec::housekeep_copy_once(), &args)?;
            let sf = SoundFile::open(&parsed.infiles[0])?;
            let outfile = parsed
                .outfile
                .clone()
                .expect("CommandSpec::housekeep_copy_once has_outfile: true");
            copy::copy_once(&sf, &outfile)
        }
    }
}

fn dispatch_housekeep_chans(args: &[String]) -> ! {
    let Some((mode_token, rest)) = args.split_first() else {
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, chans::USAGE)));
    };
    if rest.is_empty() {
        // legacy: same argc<4 rule as `housekeep copy` -- see
        // `dispatch_housekeep_copy`'s own comment.
        print!("{}", chans::GREETING);
        report_and_exit(Err(CdpError::from(ParamsError::InsufficientParameters)));
    }
    report_and_exit(run_housekeep_chans(mode_token, rest));
}

fn run_housekeep_chans(mode_token: &str, args: &[String]) -> Result<(), CdpError> {
    let mode_number = parse_mode(mode_token, chans::MAX_MODE)?;
    let Some(mode) = chans::Mode::from_number(mode_number) else {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            format!("housekeep chans: mode {mode_number} is not implemented yet"),
        ));
    };
    match mode {
        chans::Mode::ExtractChannel => {
            // legacy: `channo`'s range depends on the infile's own
            // channel count, so the infile must be open first -- see
            // `cdp_params::CommandSpec::housekeep_chans_channel`'s
            // doc. `SNDFILES_ONLY`, confirmed live: a real analysis
            // file reports the shared
            // `"Application doesn't work with this type of infile."`
            // text `cdp_programs::sndinfo::open_sound_infile` already
            // produces.
            let infile_path = args.first().ok_or(ParamsError::InsufficientParameters)?;
            let sf = cdp_programs::sndinfo::open_sound_infile(infile_path)?;
            let spec = CommandSpec::housekeep_chans_channel(sf.fmt.channels as f64);
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let parsed = parse(&spec, &args)?;
            let channo = match parsed.params[0] {
                ParamValue::Integer(n) => n,
                _ => unreachable!(
                    "CommandSpec::housekeep_chans_channel's only param is ParamType::Int"
                ),
            };
            // legacy: confirmed live, a successful `housekeep chans
            // 1` prints nothing to stdout at all.
            chans::extract_channel(&sf, &parsed.infiles[0], channo)?;
            Ok(())
        }
        chans::Mode::ExtractAllChannels => {
            // legacy: `SNDFILES_ONLY`, the same infile-kind
            // restriction mode 1 has -- see `ExtractChannel`'s own
            // comment above.
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let parsed = parse(&CommandSpec::housekeep_chans_channels(), &args)?;
            let sf = cdp_programs::sndinfo::open_sound_infile(&parsed.infiles[0])?;
            chans::extract_all_channels(&sf, &parsed.infiles[0])?;
            Ok(())
        }
        chans::Mode::MonoToStereo => {
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let parsed = parse(&CommandSpec::housekeep_chans_mtos(), &args)?;
            let sf = SoundFile::open(&parsed.infiles[0])?;
            let outfile = parsed
                .outfile
                .clone()
                .expect("CommandSpec::housekeep_chans_mtos has_outfile: true");
            chans::mono_to_stereo(&sf, &outfile)
        }
        chans::Mode::MixToMono => {
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let parsed = parse(&CommandSpec::housekeep_chans_stom(), &args)?;
            let sf = SoundFile::open(&parsed.infiles[0])?;
            let outfile = parsed
                .outfile
                .clone()
                .expect("CommandSpec::housekeep_chans_stom has_outfile: true");
            let invert_phase = parsed.flags.contains_key(&'p');
            chans::mix_to_mono(&sf, &outfile, invert_phase)
        }
        chans::Mode::ZeroChannel => {
            // legacy: `channo`'s range depends on the infile's own
            // channel count, so the infile must be open first -- see
            // `cdp_params::CommandSpec::housekeep_chans_zchannel`'s
            // doc, the same shape `ExtractChannel` above already
            // established.
            let infile_path = args.first().ok_or(ParamsError::InsufficientParameters)?;
            let sf = cdp_programs::sndinfo::open_sound_infile(infile_path)?;
            let spec = CommandSpec::housekeep_chans_zchannel(sf.fmt.channels as f64);
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let parsed = parse(&spec, &args)?;
            let outfile = parsed
                .outfile
                .clone()
                .expect("CommandSpec::housekeep_chans_zchannel has_outfile: true");
            let channo = match parsed.params[0] {
                ParamValue::Integer(n) => n,
                _ => unreachable!(
                    "CommandSpec::housekeep_chans_zchannel's only param is ParamType::Int"
                ),
            };
            chans::zero_channel(&sf, &outfile, channo)
        }
    }
}

fn dispatch_housekeep_respec(args: &[String]) -> ! {
    let Some((mode_token, rest)) = args.split_first() else {
        report_and_exit(Err(CdpError::new(ExitCategory::UsageOnly, respec::USAGE)));
    };
    if rest.is_empty() {
        // legacy: same argc<4 rule as every other `housekeep`
        // sub-command -- see `dispatch_housekeep_copy`'s own comment.
        print!("{}", respec::GREETING);
        report_and_exit(Err(CdpError::from(ParamsError::InsufficientParameters)));
    }
    report_and_exit(run_housekeep_respec(mode_token, rest));
}

fn run_housekeep_respec(mode_token: &str, args: &[String]) -> Result<(), CdpError> {
    let mode_number = parse_mode(mode_token, respec::MAX_MODE)?;
    let Some(mode) = respec::Mode::from_number(mode_number) else {
        return Err(CdpError::new(
            ExitCategory::ProgramError,
            format!("housekeep respec: mode {mode_number} is not implemented yet"),
        ));
    };
    match mode {
        respec::Mode::Convert => {
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let parsed = parse(&CommandSpec::housekeep_respec_convert(), &args)?;
            let sf = cdp_programs::sndinfo::open_sound_infile(&parsed.infiles[0])?;
            let outfile = parsed
                .outfile
                .clone()
                .expect("CommandSpec::housekeep_respec_convert has_outfile: true");
            respec::convert(&sf, &outfile)
        }
        respec::Mode::Reprop => {
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let parsed = parse(&CommandSpec::housekeep_respec_reprop(), &args)?;
            let sf = cdp_programs::sndinfo::open_sound_infile(&parsed.infiles[0])?;
            let outfile = parsed
                .outfile
                .clone()
                .expect("CommandSpec::housekeep_respec_reprop has_outfile: true");
            let srate = match parsed.flags.get(&'s') {
                Some(ParamValue::Integer(n)) => Some(*n),
                Some(_) => unreachable!("-s is ParamType::Int"),
                None => None,
            };
            let channels = match parsed.flags.get(&'c') {
                Some(ParamValue::Integer(n)) => Some(*n),
                Some(_) => unreachable!("-c is ParamType::Int"),
                None => None,
            };
            respec::reprop(&sf, &outfile, srate, channels)
        }
    }
}
