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

//! `sndinfo units mode value` (`INFO_MUSUNITS`): converts between
//! musical units -- MIDI note numbers, frequencies, note names,
//! interval ratios, semitones, octaves, time-stretch ratios, gain/dB,
//! and delay times/tempos. legacy:
//! `legacy/dev/sndinfo/musunit.c`'s `do_musunits`, a dispatch over
//! `dz->mode`. `spec/usage/sndinfo/units.txt` documents 25 modes, but
//! the real mode range is 1-31 (confirmed live: `sndinfo units 31 1`
//! runs -- reporting its own, unrelated range error -- while `sndinfo
//! units 32 1` reports `"Program mode value [32] is out of range [1 -
//! 31]."`; modes 26-31 are undocumented DELAY/TEMPO conversions).
//! `INFO_MUSUNITS` is `NO_FILE_AT_ALL` in `ap_sndinfo.c`'s
//! `assign_process_logic` -- unlike every other `sndinfo` sub-command
//! ported so far, this one takes no infile or outfile at all, just a
//! mode number and one value.
//!
//! Scope of this slice: modes 1 (`MIDI to FRQ`) and 2 (`FRQ to MIDI`)
//! only, the two purely numeric PITCH-section modes. The rest of the
//! PITCH section (modes 3-6) needs `ap_sndinfo.c`'s
//! `readnote_as_midi` note-name grammar; the INTERVAL/SPEED sections
//! (7-23) need `interval_to_semitones`'s interval-name grammar;
//! LOUDNESS (24-25) and the undocumented DELAY/TEMPO modes (26-31)
//! are each their own formula. All deferred to future slices.
//!
//! Both modes 1 and 2 are a plain `cdp_params::ParamType::Double`
//! (`parstruct.json`'s `INFO_MUSUNITS` entry: `param_list: "d"`,
//! `special_data: "0"` for both), whose range is not a fixed literal
//! but mode-dependent (`tklib1.c`'s `set_param_ranges`, `case
//! (INFO_MUSUNITS): switch(mode)`): mode 1 is `[MIDIMIN, MIDIMAX]` =
//! `[0.0, 127.0]`; mode 2 is `[miditohz(MIDIMIN), miditohz(MIDIMAX)]`
//! = `[8.175799, 12543.853951]` (`miditohz` is the exact formula
//! [`midi_to_frq`] below also uses for mode 1's own output).
//! [`Mode::range`] carries this, the same "range is a parameter,
//! not a literal" shape `cdp_params::CommandSpec::sndinfo_smptime`/
//! `sndinfo_timesmp` already established for an infile-dependent
//! bound.
//!
//! A genuine legacy quirk, ported as observed rather than smoothed
//! over: `do_musunits` always prints its result text once,
//! unconditionally, via `print_outmessage_flush(errstr)`, *before*
//! returning its exit status to the caller. For a successful mode
//! this is the only place the result text is printed. But mode 2 also
//! has its own internal range check (`frq_to_midi`'s `MINPITCH`/
//! `FRQMAX` literals, `9.0`/`12543`), narrower than the generic
//! `cdp_params` range check above at both ends (`8.175799 < 9.0` and
//! `12543 < 12543.853951`), so it is genuinely reachable: a value in
//! `(8.175799, 9.0)` or `(12543.0, 12543.853951]` passes the outer
//! check but fails this inner one, returning `GOAL_FAILED`. When that
//! happens, `errstr` is printed *twice*: once plain by the
//! unconditional call above, then again with the usual `"ERROR:
//! CANNOT ACHIEVE TASK:"` header by `report_and_exit`'s normal error
//! path. Confirmed live byte-for-byte (`sndinfo units 2 8.5`; see
//! [`run`]'s own doc for how this is reproduced). Mode 1's equivalent
//! internal check (`midi_to_frq`'s `MIDIMIN`/`MIDIMAX`, `0`/`127`) is
//! ported for the same fidelity but is confirmed unreachable: those
//! literals are identical to the outer check's own bounds for mode 1,
//! so nothing can pass the outer check and fail the inner one.

use cdp_core::{CdpError, ExitCategory};

/// legacy: `main.c`'s `make_initial_cmdline_check`, same rule as every
/// other `sndinfo` sub-command -- see `super::props::GREETING`'s doc.
pub const GREETING: &str = "CDP Release 7.1 2016\n";

/// The real mode range (see this module's doc for why it is 31, not
/// the 25 `USAGE` documents).
pub const MAX_MODE: u32 = 31;

/// legacy: the usage text for a bare `sndinfo units` (captured live,
/// byte for byte, via `docker run cdp8-postmerge sndinfo units`;
/// `spec/usage/sndinfo/units.txt`, WP-0.2), minus the two trailing
/// blank lines `ExitCategory::UsageOnly`'s own tail already supplies
/// -- see `super::props::USAGE`'s doc for why this is a literal
/// fixture, and `crate::synth::wave::USAGE`'s doc for why it is a raw
/// string rather than the usual `"...\` style.
pub const USAGE: &str = r"CDP Release 7.1 2016
CONVERT BETWEEN DIFFERENT UNITS

USAGE: sndreport units mode value

                           MODES ARE

       PITCH                  INTERVAL                       SPEED
       -----                  --------                       -----
(1) MIDI to FRQ   (7)  FRQ RATIO  to SEMITONES (16) FRQ RATIO  to TIME RATIO
(2) FRQ  to MIDI  (8)  FRQ RATIO  to INTERVAL  (17) SEMITONES  to TIME RATIO
(3) NOTE to FRQ   (9)  INTERVAL   to FRQ RATIO (18) OCTAVES    to TIME RATIO
(4) NOTE to MIDI  (10) SEMITONES to FRQ RATIO  (19) INTERVAL   to TIME RATIO
(5) FRQ to NOTE   (11) OCTAVES    to FRQ RATIO (20) TIME RATIO to FRQ RATIO
(6) MIDI to NOTE  (12) OCTAVES   to SEMITONES  (21) TIME RATIO to SEMITONES
                  (13) FRQ RATIO to OCTAVES    (22) TIME RATIO to OCTAVES
                  (14) SEMITONES to OCTAVES    (23) TIME RATIO to INTERVAL
                  (15) SEMITONES to INTERVAL

                                LOUDNESS
                                --------
                  (24) GAIN FACTOR to DB GAIN
                  (25) DB GAIN     to GAIN FACTOR

NOTE REPRESENTATION ..... A1 = A in octave 1
                          Ebu4 is E flat,   + (Up) quartertone  in octave 4
                          F#d-2 is F sharp, - (Dn) quartertone, in octave -2

INTERVAL REPRESENTATION.. 3 = a 3rd     -m3 = minor 3rd DOWN
                          m3u = minor 3rd + (Up) quartertone
                          #4d = tritone   - (Dn) quartertone
                          15  = a fifteenth (max permissible interval)
";

/// legacy: `globcon.h`'s `MIDIMIN`/`MIDIMAX`.
const MIDIMIN: f64 = 0.0;
const MIDIMAX: f64 = 127.0;
/// legacy: `globcon.h`'s `MINPITCH`/`musunit.c`'s `FRQMAX`.
const MINPITCH: f64 = 9.0;
const FRQMAX: f64 = 12543.0;
/// legacy: `globcon.h`'s `LOW_A`/`SEMITONES_PER_OCTAVE`.
const LOW_A: f64 = 6.875;
const SEMITONES_PER_OCTAVE: f64 = 12.0;
/// legacy: `musunit.c`'s `CONVERT_LOG10_TO_LOG2`, a literal constant
/// (not a computed `log2`) that `my_LOG2` multiplies a `log10` result
/// by -- ported as the same literal, not `f64::log2`, since the two
/// differ in the low decimal digits and this crate's byte-for-byte
/// live comparisons print to 6 decimal places.
#[allow(clippy::approx_constant)]
const CONVERT_LOG10_TO_LOG2: f64 = 3.321928;

/// The two modes this slice implements. legacy: `modeno.h`'s
/// `MU_MIDI_TO_FRQ`/`MU_FRQ_TO_MIDI`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    MidiToFrq,
    FrqToMidi,
}

impl Mode {
    /// `None` for a mode number this slice does not implement yet --
    /// the caller reports that as a plain `ProgramError`, distinct
    /// from an out-of-range mode number (`1..=MAX_MODE` is already
    /// enforced by `cdp_params::parse_mode` before this is called).
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Self::MidiToFrq),
            2 => Some(Self::FrqToMidi),
            _ => None,
        }
    }

    /// legacy: `tklib1.c`'s `set_param_ranges`, `case(INFO_MUSUNITS)`
    /// -- see this module's doc for the exact values and how they
    /// were confirmed.
    pub fn range(self) -> (f64, f64) {
        match self {
            Mode::MidiToFrq => (MIDIMIN, MIDIMAX),
            Mode::FrqToMidi => (midi_to_hz(MIDIMIN), midi_to_hz(MIDIMAX)),
        }
    }
}

/// legacy: `miditohz` (`legacy/dev/cdp2k/tklib1.c`) and `midi_to_frq`'s
/// own arithmetic (`legacy/dev/sndinfo/musunit.c`) -- the identical
/// formula, confirmed by reading both side by side.
fn midi_to_hz(midi: f64) -> f64 {
    let mut frq = midi;
    frq += 3.0;
    frq /= SEMITONES_PER_OCTAVE;
    frq = 2.0_f64.powf(frq);
    frq *= LOW_A;
    frq
}

/// legacy: `musunit.c`'s `my_LOG2`.
fn log2_via_log10(x: f64) -> f64 {
    x.log10() * CONVERT_LOG10_TO_LOG2
}

/// legacy: `do_musunits`'s dispatch for [`Mode::MidiToFrq`]/
/// [`Mode::FrqToMidi`], plus the unconditional
/// `print_outmessage_flush(errstr)` call that follows it -- see this
/// module's doc for why a `GOAL_FAILED` result is printed twice.
/// `value` has already passed the generic `cdp_params` range check
/// for `mode` (see [`Mode::range`]); this only re-checks the
/// (sometimes narrower) internal bound `musunit.c` itself uses.
pub fn run(mode: Mode, value: f64) -> Result<(), CdpError> {
    let (text, failed) = match mode {
        Mode::MidiToFrq => midi_to_frq(value),
        Mode::FrqToMidi => frq_to_midi(value),
    };
    print!("{text}");
    if failed {
        Err(CdpError::new(ExitCategory::GoalFailed, text))
    } else {
        Ok(())
    }
}

/// legacy: `midi_to_frq` in `musunit.c`.
fn midi_to_frq(midi: f64) -> (String, bool) {
    if midi < MIDIMIN || midi > MIDIMAX {
        // legacy: confirmed unreachable via the generic range check
        // for this mode -- see this module's doc.
        return (
            format!(
                "MIDI value out of range {} - {}\n",
                MIDIMIN as i64, MIDIMAX as i64
            ),
            true,
        );
    }
    (format!("frequency = {:.6}\n", midi_to_hz(midi)), false)
}

/// legacy: `frq_to_midi` in `musunit.c`.
fn frq_to_midi(frq: f64) -> (String, bool) {
    if frq < MINPITCH || frq > FRQMAX {
        return (
            format!(
                "Frq value out of range {} - {}\n",
                MINPITCH.round() as i64,
                FRQMAX as i64
            ),
            true,
        );
    }
    let mut midi = frq;
    midi /= LOW_A;
    midi = log2_via_log10(midi);
    midi *= SEMITONES_PER_OCTAVE;
    midi -= 3.0;
    (
        format!(
            "MIDI value = {:.6} or approx {}\n",
            midi,
            midi.round() as i64
        ),
        false,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mode_from_number_covers_only_the_two_implemented_modes() {
        assert_eq!(Mode::from_number(1), Some(Mode::MidiToFrq));
        assert_eq!(Mode::from_number(2), Some(Mode::FrqToMidi));
        assert_eq!(Mode::from_number(3), None);
        assert_eq!(Mode::from_number(31), None);
    }

    /// legacy: `docker run cdp8-postmerge sndinfo units 1 60`.
    #[test]
    fn midi_to_frq_matches_the_real_legacy_output_for_middle_c() {
        let (text, failed) = midi_to_frq(60.0);
        assert!(!failed);
        assert_eq!(text, "frequency = 261.625565\n");
    }

    /// legacy: `docker run cdp8-postmerge sndinfo units 1 0` / `... 1
    /// 127`, the range's own inclusive endpoints.
    #[test]
    fn midi_to_frq_matches_the_real_legacy_output_at_both_range_endpoints() {
        let (low, low_failed) = midi_to_frq(0.0);
        assert!(!low_failed);
        assert_eq!(low, "frequency = 8.175799\n");
        let (high, high_failed) = midi_to_frq(127.0);
        assert!(!high_failed);
        assert_eq!(high, "frequency = 12543.853951\n");
    }

    /// legacy: `docker run cdp8-postmerge sndinfo units 2 440`.
    #[test]
    fn frq_to_midi_matches_the_real_legacy_output_for_concert_a() {
        let (text, failed) = frq_to_midi(440.0);
        assert!(!failed);
        assert_eq!(text, "MIDI value = 68.999998 or approx 69\n");
    }

    /// legacy: `docker run cdp8-postmerge sndinfo units 2 8.5` -- a
    /// value that passes the generic `cdp_params` range check
    /// (`8.175799..12543.853951`) but fails `frq_to_midi`'s own
    /// narrower internal check (`9.0..12543.0`). See this module's
    /// doc for the confirmed-live double-print this implies via
    /// [`run`].
    #[test]
    fn frq_to_midi_fails_its_own_narrower_internal_range_check() {
        let (text, failed) = frq_to_midi(8.5);
        assert!(failed);
        assert_eq!(text, "Frq value out of range 9 - 12543\n");
    }

    /// legacy: `docker run cdp8-postmerge sndinfo units 2 12543.5`,
    /// the same gap as the low-end case above but at the high end.
    #[test]
    fn frq_to_midi_fails_its_own_narrower_internal_range_check_at_the_high_end() {
        let (text, failed) = frq_to_midi(12543.5);
        assert!(failed);
        assert_eq!(text, "Frq value out of range 9 - 12543\n");
    }

    /// legacy: `docker run cdp8-postmerge sndinfo units 2
    /// 8.175799`/`... 2 12543.853951`, mode 2's own range endpoints
    /// (`miditohz(0)`/`miditohz(127)`).
    #[test]
    fn mode_range_matches_the_real_legacy_bounds() {
        let (lo, hi) = Mode::FrqToMidi.range();
        assert_eq!(format!("{lo:.6}"), "8.175799");
        assert_eq!(format!("{hi:.6}"), "12543.853951");
        assert_eq!(Mode::MidiToFrq.range(), (0.0, 127.0));
    }
}
