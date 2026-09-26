// This file is part of a Rust reimplementation of the CDP System.
// SPDX-License-Identifier: LGPL-2.1-or-later

//! `housekeep extract`: cut, top-and-tail, rectify and onset-time modes.
//!
//! Mode 4 (`HOUSE_RECTIFY`) is ported. legacy: `do_rectify` in
//! `legacy/dev/houskeep/clean.c`. The other five modes need gate and
//! envelope detection, which this crate does not have yet, so they report
//! a plain `ProgramError`.
//!
//! Mode 4 adds a constant offset to every sample, which is how legacy
//! removes DC drift. It does not compute the offset itself: the offset is
//! the required `shift` argument. An earlier version of this module
//! computed the mean of the samples and subtracted it, which is a
//! different operation and ignored the argument entirely.
//!
//! The order of legacy's own checks is reproduced, because each one has
//! its own message and they are observable:
//!
//! 1. A shift of zero (within `FLTERR`) fails with `"NO CHANGE to
//!    original sound file."`, before the file is read at all.
//! 2. A positive shift scans for the largest sample and refuses if the
//!    result would exceed `F_MAXSAMP`. A negative shift scans for the
//!    smallest and refuses below `F_MINSAMP`. Both refusals share the
//!    message `"This rectification will distort the sound."`.
//!
//! legacy prints its progress as three lines on standard output, then the
//! usual `display_virtual_time` tick, all confirmed live.

use cdp_core::{CdpError, ExitCategory};
use cdp_params::{ParamValue, ParsedCommand};

pub const GREETING: &str = "CDP Release 7.1 2016\n";

pub const USAGE: &str = r"CDP Release 7.1 2016
USAGE: housekeep extract 
       1 inf [-ggate] [-sS] [-eE] [-tT] [-hH] [-bB] [-iI] [-lL] [-wW] [-n]
OR:    2 inf outf             OR: 3 inf outf [-ggate] [-ssplice] [-b] [-e]
OR:    4 inf outf shift       OR: 5 inf outf valsfile
OR:    6 inf gate endgate threshold baktrak initlevel minsize gatewin
MODE 1) CUT OUT & KEEP SIGNIFICANT EVENTS FROM INPUT SNDFILE.
-g  GATE level (G) above which sounds accepted : Range:0-1 (default 0).
-s  SPLICE LENGTH in mS (default 15mS)
-e  END-CUTOFF level (E) below which END of sound cut.(If 0, defaults to GATE).
-t  THRESHOLD (T) If segment level never exceeds threshold,not kept(default 0)
-h  HOLD sound till S sectors BEFORE START of next segment.(default 0)
-b  KEEP B sects prior to gate-on, if level there > I (see below). Max B is 64.
-i  INITIAL level (I). Use with -b flag.
-l  Min LENGTH of events to keep (secs).
-w  GATE_WINDOW Gates off only if level < gate for W+1 sectors. (default 0).
-n  STOP if NAME of EXISTING sndfile generated. Default: ignore and continue.
MODE 2) PREVIEW: make envel as 'sound'. View to get params for CUT OUT & KEEP.
MODE 3) TOP AND TAIL: REMOVE LOW LEVEL SIGNAL FROM START & END OF SOUND.
 GATE   level ABOVE which signal accepted : (Range 0-1 : default 0).
 SPLICE length in mS (default 15mS) : -b Don't trim start : -e Don't trim end.
MODE 4) RECTIFY: SHIFT ENTIRE SIGNAL TO ELIMINATE DC DRIFT.
MODE 5) MODIFY 'BY HAND'. This process is no longer available.
MODE 6) GET ONSET TIMES:gate,end-cutoff,thresh,keep,init-lvl,minlen,gate-windw.
";

/// legacy: `globcon.h`'s `F_MAXSAMP` and `F_MINSAMP`, the bounds a
/// rectified sample must stay inside.
const F_MAXSAMP: f64 = 1.0;
const F_MINSAMP: f64 = -1.0;

/// legacy: `globcon.h`'s `FLTERR`, the tolerance `flteq` uses.
const FLTERR: f64 = 0.000002;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Rectify = 4,
}

impl Mode {
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            4 => Some(Mode::Rectify),
            _ => None,
        }
    }
}

pub fn extract(parsed: &ParsedCommand, mode: Mode) -> Result<(), CdpError> {
    match mode {
        Mode::Rectify => rectify(parsed),
    }
}

/// legacy: `flteq` (`legacy/dev/cdp2k/tklib1.c`).
fn flteq(a: f64, b: f64) -> bool {
    (a - b).abs() < FLTERR
}

fn rectify(parsed: &ParsedCommand) -> Result<(), CdpError> {
    let infile_path = &parsed.infiles[0];
    let outfile_path = parsed
        .outfile
        .as_deref()
        .expect("housekeep_extract_rectify sets has_outfile");

    // legacy: `rectify_shift = dz->param[RECTIFY_SHIFT] *
    // (double)F_MAXSAMP`, and `F_MAXSAMP` is 1.0, so the argument is
    // used directly against normalised sample values.
    let shift = match parsed.params.first() {
        Some(ParamValue::Number(value)) => *value * F_MAXSAMP,
        _ => unreachable!("housekeep_extract_rectify declares one Double parameter"),
    };

    // legacy checks this before opening or reading anything.
    if flteq(shift, 0.0) {
        return Err(CdpError::new(
            ExitCategory::GoalFailed,
            "NO CHANGE to original sound file.\n",
        ));
    }

    let sf = super::super::sndinfo::open_sound_infile(infile_path)?;
    super::check_outfile_does_not_exist(outfile_path)?;
    let samples = sf.samples_f32().map_err(CdpError::from)?;

    if shift > 0.0 {
        println!("INFO: Finding maximum sample in file.");
        let largest = samples
            .iter()
            .fold(F_MINSAMP, |acc, &s| acc.max(f64::from(s)));
        if largest + shift > F_MAXSAMP {
            return Err(CdpError::new(
                ExitCategory::GoalFailed,
                "This rectification will distort the sound.\n",
            ));
        }
    } else {
        println!("INFO: Finding minimum sample in file.");
        let smallest = samples
            .iter()
            .fold(F_MAXSAMP, |acc, &s| acc.min(f64::from(s)));
        if smallest + shift < F_MINSAMP {
            return Err(CdpError::new(
                ExitCategory::GoalFailed,
                "This rectification will distort the sound.\n",
            ));
        }
    }

    println!("INFO: Rectifying.");

    // legacy: `buf[n] = (float)(buf[n] + rectify_shift)`. The sample
    // promotes to double for the addition, then narrows back.
    let shifted: Vec<f32> = samples
        .iter()
        .map(|&s| (f64::from(s) + shift) as f32)
        .collect();

    super::write_wave_file(
        sf.fmt.channels,
        sf.fmt.sample_rate,
        sf.fmt.sample_type,
        &shifted,
        outfile_path,
    )?;
    super::print_virtual_time(
        shifted.len() as f64 / (f64::from(sf.fmt.sample_rate) * f64::from(sf.fmt.channels)),
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{FLTERR, Mode, flteq};

    #[test]
    fn mode_4_is_the_only_one_ported() {
        assert_eq!(Mode::from_number(4), Some(Mode::Rectify));
        for other in [1, 2, 3, 5, 6] {
            assert_eq!(Mode::from_number(other), None);
        }
    }

    #[test]
    fn flteq_uses_legacy_flterr() {
        // legacy: `FLTERR` is 0.000002, so a shift just inside it counts
        // as zero and one just outside it does not.
        assert!(flteq(FLTERR / 2.0, 0.0));
        assert!(!flteq(FLTERR * 2.0, 0.0));
        assert!(flteq(0.0, 0.0));
    }

    #[test]
    fn a_shift_narrower_than_flterr_counts_as_no_change() {
        assert!(flteq(0.000001, 0.0), "legacy refuses this as no change");
        assert!(!flteq(0.00001, 0.0), "legacy accepts this one");
    }
}
