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

//! `synth wave`: generate a sine, square, sawtooth or ramp waveform.
//! legacy: `legacy/dev/synth/wave.c` (`do_synth`'s non-silence,
//! non-noise, non-click branch, i.e. `gentable`/`gen_wave`).
//!
//! This is the one program wired all the way through for WP-1.5 (see
//! `docs/migration/PLAN.md`'s done-when for that WP), chosen because
//! it needs no FFT or phase vocoder (`cdp-dsp`, WP-1.4, not built
//! yet): it is a table-lookup oscillator with linear start/end
//! splices and, when `freq`/`amp` are breakpoint files rather than
//! plain numbers, 10-millisecond-block envelope stepping (linear for
//! amplitude, logarithmic for frequency) -- all ported directly from
//! `wave.c`, not approximated.

use cdp_core::{CdpError, ExitCategory};
use cdp_data::BreakpointTable;
use cdp_params::{ParamValue, ParsedCommand};
use cdp_sf::{PropertyBlock, SampleType, SoundFileWriter, WriteSpec};
use std::f64::consts::PI;

/// legacy: `legacy/dev/include/modeno.h`'s `WAVE_SINE`/`WAVE_SQUARE`/
/// `WAVE_SAW`/`WAVE_RAMP` (`0`-`3`, internal/0-based); the discriminants
/// here are the 1-based numbers a user types (`synth wave 1 ...`),
/// i.e. `legacy_mode() as u32 == internal + 1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Sine = 1,
    Square = 2,
    Sawtooth = 3,
    Ramp = 4,
}

/// legacy: `dz->maxmode` for `SYNTH_WAVE`, from `get_maxmode`.
pub const MAX_MODE: u32 = 4;

/// legacy: the usage text `legacy/dev/synth/main.c` prints for `synth
/// wave` with no further arguments at all (captured live, byte for
/// byte, via `docker run cdp8-postmerge synth wave`). General
/// usage-text *generation* from a `CommandSpec` is out of scope for
/// now (`cdp-params`'s own module doc defers it, blocked on WP-0.2),
/// so this is a literal fixture, not the output of a formatter --
/// revisit once that formatter exists.
// A raw string, not the usual `"...\n\` line-continuation style this
// codebase's other multi-line constants use: `\` at a string
// literal's end-of-line also strips the *next* line's leading
// whitespace, which would silently eat the indentation on the
// "defaults to 256" line below -- confirmed the hard way, via a live
// `diff` against `docker run cdp8-postmerge synth wave` that failed
// on exactly that line until this was switched to `r"..."`.
pub const USAGE: &str = r"CDP Release 7.1 2016
GENERATE SIMPLE WAVEFORMS

USAGE: synth wave mode outfile sr chans dur freq [-aamp] [-ttabsize]

MODES ARE
1) sine wave
2) square wave
3) sawtooth wave
4) ramp wave

SR      (sample rate) can be 48000, 24000, 44100, 22050, 32000, or 16000
CHANS   can be 1, 2 or 4
DUR     is duration of output snd, in seconds.
FREQ    is frq of output sond, in Hz
AMP     is amplitude of output sound: 0.0 < Range <= 1.0 (max & default).
TABSIZE is size of table storing waveform.
        defaults to 256: input value always rounded to multiple of 4.

Frq and Amp may vary through time.
";

impl Mode {
    pub fn from_number(mode: u32) -> Option<Self> {
        match mode {
            1 => Some(Mode::Sine),
            2 => Some(Mode::Square),
            3 => Some(Mode::Sawtooth),
            4 => Some(Mode::Ramp),
            _ => None,
        }
    }
}

/// legacy: `F_MAXSAMP` in `legacy/dev/include/globcon.h` -- this is
/// the "floatsam" build, where full scale is always `1.0` regardless
/// of the eventual on-disk sample type (short or float); `cdp-sf`'s
/// writer does the short conversion.
const F_MAXSAMP: f64 = 1.0;

/// legacy: `SYNTH_SPLICELEN` in `legacy/dev/include/synth.h`. The
/// number of sample *frames* linearly faded in at the start and out
/// at the end.
const SYNTH_SPLICELEN: i64 = 256;

/// legacy: `MS_TO_SECS` in `globcon.h`, times the `10.0` literal
/// `gen_wave`'s own `timestep` uses -- ten milliseconds, in seconds.
const BLOCK_SECONDS: f64 = 0.01;

/// A `freq`/`amp` argument: either a plain number (legacy: `dz->param`
/// with `dz->brksize[paramno] == 0`) or a breakpoint file (`dz->brk`).
/// legacy: `read_values_from_all_existing_brktables` re-evaluates
/// every *breakpoint* parameter at a given time and leaves a plain
/// number's `dz->param` slot untouched, since it was already set,
/// once, at argument-parsing time -- [`Self::value_at`] reproduces
/// both halves of that by simply returning the constant regardless of
/// `time` for [`Self::Constant`].
enum ValueSource {
    Constant(f64),
    Breakpoint(BreakpointTable),
}

impl ValueSource {
    fn from_param(value: &ParamValue) -> Self {
        match value {
            ParamValue::Number(n) => ValueSource::Constant(*n),
            ParamValue::Breakpoint(table) => ValueSource::Breakpoint(table.clone()),
            _ => unreachable!("synth wave's freq/amp are always DoubleOrBreakpoint"),
        }
    }

    fn is_breakpoint(&self) -> bool {
        matches!(self, ValueSource::Breakpoint(_))
    }

    fn value_at(&self, time: f64, cursor: &mut usize) -> Result<f64, CdpError> {
        match self {
            ValueSource::Constant(v) => Ok(*v),
            ValueSource::Breakpoint(table) => {
                let (value, next_cursor) = table.interpolate_from_cursor(time, *cursor)?;
                *cursor = next_cursor;
                Ok(value)
            }
        }
    }
}

fn expect_integer(value: &ParamValue) -> i64 {
    match value {
        ParamValue::Integer(n) => *n,
        _ => unreachable!("synth wave's sr/chans are always Int"),
    }
}

fn expect_number(value: &ParamValue) -> f64 {
    match value {
        ParamValue::Number(n) => *n,
        _ => unreachable!("synth wave's dur is always Double"),
    }
}

/// legacy: `gentable`. `tab` is `tabsize + 1` long; the extra slot at
/// `tab[tabsize]` is what [`getval`] reads as the "next" point when
/// interpolating from `tab[tabsize - 1]`, and is set explicitly by
/// every mode (not left at its `vec![]`-initialised `0.0` by
/// coincidence, except where that genuinely is the right value --
/// ported per mode exactly as `wave.c` writes it, not assumed).
fn gentable(mode: Mode, tabsize: usize) -> Vec<f64> {
    let mut tab = vec![0.0f64; tabsize + 1];
    match mode {
        Mode::Sine => {
            let step = (2.0 * PI) / tabsize as f64;
            for (n, slot) in tab.iter_mut().enumerate().take(tabsize) {
                *slot = (step * n as f64).sin() * F_MAXSAMP;
            }
            tab[tabsize] = 0.0;
        }
        Mode::Square => {
            let k0 = tabsize / 2;
            for slot in tab.iter_mut().take(k0) {
                *slot = F_MAXSAMP;
            }
            for slot in &mut tab[k0..=tabsize] {
                *slot = -F_MAXSAMP;
            }
        }
        Mode::Sawtooth => {
            let k0 = tabsize / 4;
            let k1 = tabsize / 2;
            let minstep = F_MAXSAMP / k0 as f64;
            tab[0] = 0.0;
            for n in 1..=k0 {
                tab[n] = tab[n - 1] + minstep;
            }
            let mut m = k0 as i64 - 1;
            for n in k0..k1 {
                tab[n] = tab[m as usize];
                m -= 1;
            }
            for (m, n) in (k1..tabsize).enumerate() {
                tab[n] = -tab[m];
            }
            tab[tabsize] = 0.0;
        }
        Mode::Ramp => {
            let k0 = tabsize / 2;
            let minstep = F_MAXSAMP / k0 as f64;
            tab[0] = F_MAXSAMP;
            for n in 1..k0 {
                tab[n] = tab[n - 1] - minstep;
            }
            tab[k0] = 0.0;
            let mut m = k0 as i64 - 1;
            for n in (k0 + 1)..=tabsize {
                tab[n] = -tab[m as usize];
                m -= 1;
            }
        }
    }
    tab
}

/// legacy: `getval`. Linear interpolation between `tab[i]` and
/// `tab[i + 1]`, scaled by `amp`.
fn getval(tab: &[f64], i: usize, fracstep: f64, amp: f64) -> f64 {
    let val = tab[i];
    let diff = tab[i + 1] - val;
    (val + diff * fracstep) * amp
}

/// One (re-)computation of the current 10ms block's start value and
/// per-sample step, for both `freq` and `amp` together. legacy:
/// `update_wave_params`. Returns `(lastfrq, lastamp, ampstep,
/// frqstep)`: `lastfrq`/`lastamp` are the block's *starting* value
/// (whatever `freq_state`/`amp_state` held before this call), and the
/// two states are advanced, in place, to the value at
/// `nextsamptime`.
#[allow(clippy::too_many_arguments)]
fn update_wave_params(
    freq_src: &ValueSource,
    amp_src: &ValueSource,
    freq_state: &mut f64,
    amp_state: &mut f64,
    freq_cursor: &mut usize,
    amp_cursor: &mut usize,
    nextsamptime: i64,
    inverse_sr: f64,
    blokstep: i64,
) -> Result<(f64, f64, f64, f64), CdpError> {
    let thistime = nextsamptime as f64 * inverse_sr;
    let lastfrq = *freq_state;
    let lastamp = *amp_state;
    let mut ampstep = 0.0;
    let mut frqstep = 1.0;

    *amp_state = amp_src.value_at(thistime, amp_cursor)?;
    *freq_state = freq_src.value_at(thistime, freq_cursor)?;

    if amp_src.is_breakpoint() {
        ampstep = (*amp_state - lastamp) / blokstep as f64;
    }
    if freq_src.is_breakpoint() {
        let ratio = *freq_state / lastfrq;
        frqstep = 10f64.powf(ratio.log10() / blokstep as f64);
    }
    Ok((lastfrq, lastamp, ampstep, frqstep))
}

/// legacy: `do_synth`'s wave branch, i.e. `gentable` followed by
/// `gen_wave`. `cmd` is the already-validated
/// [`cdp_params::CommandSpec::synth_wave`] parse: `params` is `[sr,
/// chans, dur, freq]` and `flags` may hold `'a'` (amp, default `1.0`)
/// and `'t'` (tabsize, default `256`) -- `'f'`'s presence has no
/// effect on the generated samples (legacy: `wave.c` never reads
/// `dz->vflag[0]` for `SYNTH_WAVE`; grep of `ap_synthesis.c` confirms
/// no other file does either), so it is accepted (already validated
/// by `cdp_params::parse`) and otherwise ignored, matching legacy
/// exactly rather than by omission.
pub fn synthesize(mode: Mode, cmd: &ParsedCommand) -> Result<SoundFileWriter, CdpError> {
    let sr = expect_integer(&cmd.params[0]) as u32;
    let chans = expect_integer(&cmd.params[1]) as u16;
    let dur = expect_number(&cmd.params[2]);
    let freq_src = ValueSource::from_param(&cmd.params[3]);
    let amp_src = match cmd.flags.get(&'a') {
        Some(value) => ValueSource::from_param(value),
        None => ValueSource::Constant(1.0), // legacy: usage text, "default"
    };
    let tabsize = match cmd.flags.get(&'t') {
        Some(ParamValue::Integer(n)) => *n as usize,
        Some(_) => unreachable!("synth wave's -t is always Int"),
        None => 256, // legacy: usage text, "defaults to 256"
    };

    let chans_i64 = chans as i64;
    // legacy: `do_synth`'s `sampdur = round(sr * dur) * chans`, total
    // interleaved samples across all channels.
    let sampdur = (sr as f64 * dur).round() as i64 * chans_i64;
    let synth_splicelen = SYNTH_SPLICELEN * chans_i64;
    if sampdur < synth_splicelen * 2 + chans_i64 {
        return Err(CdpError::new(
            ExitCategory::DataError,
            "Specified output duration is less then available splicing length.",
        ));
    }

    let table = gentable(mode, tabsize);
    let samples = gen_wave(
        &table,
        sr,
        chans,
        sampdur,
        tabsize,
        freq_src,
        amp_src,
        synth_splicelen,
    )?;

    let mut writer = SoundFileWriter::new(WriteSpec {
        channels: chans,
        sample_rate: sr,
        sample_type: SampleType::Short16, // legacy: do_synth's default (dz->floatsam_output is only set by the separate, not-yet-ported "-f"-before-outfile convention; see docs/migration/STATUS.md)
        write_peaks: true,
        properties: PropertyBlock::new(),
        write_cue_chunk: false,
    });
    writer.write_frames(&samples);
    Ok(writer)
}

/// legacy: `gen_wave`, minus the buffer-at-a-time `write_samps` calls
/// (this crate's writer buffers the whole file, so every sample is
/// generated directly into one `Vec`).
#[allow(clippy::too_many_arguments)]
fn gen_wave(
    table: &[f64],
    sr: u32,
    chans: u16,
    sampdur: i64,
    tabsize: usize,
    freq_src: ValueSource,
    amp_src: ValueSource,
    synth_splicelen: i64,
) -> Result<Vec<f32>, CdpError> {
    let inverse_sr = 1.0 / sr as f64;
    let dtabsize = tabsize as f64;
    let convertor = dtabsize * inverse_sr;
    let blokstep = (BLOCK_SECONDS * sr as f64).round() as i64;

    let mut freq_cursor = 0usize;
    let mut amp_cursor = 0usize;
    // legacy: `read_values_from_all_existing_brktables(0.0,dz)`,
    // seeding `dz->param` at time zero before the first
    // `update_wave_params` call.
    let mut freq_state = freq_src.value_at(0.0, &mut freq_cursor)?;
    let mut amp_state = amp_src.value_at(0.0, &mut amp_cursor)?;

    let mut nextsamptime: i64 = blokstep;
    let (lastfrq, lastamp, mut ampstep, mut frqstep) = update_wave_params(
        &freq_src,
        &amp_src,
        &mut freq_state,
        &mut amp_state,
        &mut freq_cursor,
        &mut amp_cursor,
        nextsamptime,
        inverse_sr,
        blokstep,
    )?;
    // legacy: `samptime = blokstep` (i.e. `nextsamptime`'s
    // pre-increment value) after the first `update_wave_params` call.
    let mut samptime: i64 = nextsamptime;
    nextsamptime += blokstep;

    let mut amp = lastamp;
    let mut frq = lastfrq;
    let mut step = 0.0f64;
    let mut i = 0usize;
    let mut fracstep = 0.0f64;

    let mut do_start = true;
    let mut startj: i64 = 0;
    let mut endj: i64 = SYNTH_SPLICELEN;
    let endsplicestart = sampdur - synth_splicelen;
    let mut total_samps: i64 = 0;

    let mut out = Vec::with_capacity(sampdur as usize);
    while total_samps < sampdur {
        let mut val = getval(table, i, fracstep, amp);
        if do_start {
            val *= startj as f64 / SYNTH_SPLICELEN as f64;
            startj += 1;
            if startj >= SYNTH_SPLICELEN {
                do_start = false;
            }
        }
        if total_samps >= endsplicestart {
            val *= endj as f64 / SYNTH_SPLICELEN as f64;
            endj -= 1;
        }
        for _ in 0..chans {
            out.push(val as f32);
            total_samps += 1;
        }

        // legacy: `advance_in_table`.
        step += frq * convertor;
        while step >= dtabsize {
            step -= dtabsize;
        }
        i = step as usize;
        fracstep = step - i as f64;
        amp += ampstep;
        frq *= frqstep;
        samptime += 1;
        if samptime >= nextsamptime {
            let (new_lastfrq, new_lastamp, new_ampstep, new_frqstep) = update_wave_params(
                &freq_src,
                &amp_src,
                &mut freq_state,
                &mut amp_state,
                &mut freq_cursor,
                &mut amp_cursor,
                nextsamptime,
                inverse_sr,
                blokstep,
            )?;
            ampstep = new_ampstep;
            frqstep = new_frqstep;
            nextsamptime += blokstep;
            amp = new_lastamp;
            frq = new_lastfrq;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cdp_data::BreakpointTable;
    use cdp_params::ParsedCommand;
    use std::collections::BTreeMap;

    fn command(params: Vec<ParamValue>, flags: BTreeMap<char, ParamValue>) -> ParsedCommand {
        ParsedCommand {
            infiles: vec![],
            outfile: "out.wav".to_string(),
            params,
            flags,
        }
    }

    #[test]
    fn gentable_length_is_tabsize_plus_one() {
        for mode in [Mode::Sine, Mode::Square, Mode::Sawtooth, Mode::Ramp] {
            assert_eq!(gentable(mode, 256).len(), 257);
        }
    }

    #[test]
    fn gentable_sine_starts_and_wraps_at_zero_and_peaks_at_a_quarter() {
        // legacy: `gentable`'s `WAVE_SINE` case.
        let tab = gentable(Mode::Sine, 256);
        assert_eq!(tab[0], 0.0);
        assert!((tab[64] - 1.0).abs() < 1e-9); // a quarter turn: sin(pi/2)
        assert_eq!(tab[256], 0.0); // the explicit extra endpoint
    }

    #[test]
    fn gentable_square_is_high_then_low_including_the_extra_endpoint() {
        // legacy: `gentable`'s `WAVE_SQUARE` case -- `tab[tabsize]` is
        // `-F_MAXSAMP`, not `0.0` (unlike sine/sawtooth), since the
        // low half's fill loop runs through `n <= tabsize` inclusive.
        let tab = gentable(Mode::Square, 256);
        assert_eq!(tab[0], 1.0);
        assert_eq!(tab[127], 1.0);
        assert_eq!(tab[128], -1.0);
        assert_eq!(tab[256], -1.0);
    }

    #[test]
    fn gentable_ramp_descends_from_peak_through_zero_to_trough() {
        // legacy: `gentable`'s `WAVE_RAMP` case.
        let tab = gentable(Mode::Ramp, 256);
        assert_eq!(tab[0], 1.0);
        assert_eq!(tab[128], 0.0);
        assert!((tab[256] - (-1.0)).abs() < 1e-9);
    }

    #[test]
    fn gentable_sawtooth_endpoints() {
        // legacy: `gentable`'s `WAVE_SAW` case. `tab[k0]` (`64`) is
        // deliberately overwritten by the second fill loop's first
        // iteration (`n = k0`), not left at the first loop's ramp
        // value -- confirmed by hand-tracing `wave.c` (see this
        // module's `gentable` doc).
        let tab = gentable(Mode::Sawtooth, 256);
        assert_eq!(tab[0], 0.0);
        assert!((tab[63] - 63.0 / 64.0).abs() < 1e-9);
        assert_eq!(tab[256], 0.0);
    }

    #[test]
    fn synthesize_sine_produces_the_requested_frame_count() {
        let cmd = command(
            vec![
                ParamValue::Integer(44100),
                ParamValue::Integer(1),
                ParamValue::Number(0.1),
                ParamValue::Number(440.0),
            ],
            BTreeMap::new(),
        );
        let writer = synthesize(Mode::Sine, &cmd).unwrap();
        let bytes = writer.encode().unwrap();
        // 16-bit mono PCM: 2 bytes per sample, round(44100 * 0.1) frames.
        let data_chunk = find_data_chunk(&bytes);
        assert_eq!(data_chunk.len(), 4410 * 2);
    }

    #[test]
    fn synthesize_amp_flag_scales_the_waveform_peak() {
        let full = synthesize(
            Mode::Sine,
            &command(
                vec![
                    ParamValue::Integer(44100),
                    ParamValue::Integer(1),
                    ParamValue::Number(0.1),
                    ParamValue::Number(440.0),
                ],
                BTreeMap::new(),
            ),
        )
        .unwrap();
        let mut half_flags = BTreeMap::new();
        half_flags.insert('a', ParamValue::Number(0.5));
        let half = synthesize(
            Mode::Sine,
            &command(
                vec![
                    ParamValue::Integer(44100),
                    ParamValue::Integer(1),
                    ParamValue::Number(0.1),
                    ParamValue::Number(440.0),
                ],
                half_flags,
            ),
        )
        .unwrap();
        let full_peak = peak_sample(&full.encode().unwrap());
        let half_peak = peak_sample(&half.encode().unwrap());
        assert!((full_peak as f64 - 2.0 * half_peak as f64).abs() <= 1.0);
    }

    #[test]
    fn synthesize_freq_as_a_breakpoint_file_does_not_error() {
        let table = BreakpointTable::parse("0.0 200\n1.0 800\n", 0.1, 22000.0, "test").unwrap();
        let cmd = command(
            vec![
                ParamValue::Integer(44100),
                ParamValue::Integer(1),
                ParamValue::Number(1.0),
                ParamValue::Breakpoint(table),
            ],
            BTreeMap::new(),
        );
        let writer = synthesize(Mode::Sine, &cmd).unwrap();
        let bytes = writer.encode().unwrap();
        assert_eq!(find_data_chunk(&bytes).len(), 44100 * 2);
    }

    #[test]
    fn synthesize_rejects_a_duration_shorter_than_the_splice_length() {
        // legacy: `gen_wave`'s own guard, ported even though the
        // ranges `cdp-params`'s `CommandSpec::synth_wave` already
        // enforces (`dur >= MIN_SYN_DUR`, `sr >= 16000`) make it
        // unreachable through the normal CLI path -- see this
        // module's `synthesize` doc.
        let cmd = command(
            vec![
                ParamValue::Integer(16000),
                ParamValue::Integer(1),
                ParamValue::Number(0.001),
                ParamValue::Number(440.0),
            ],
            BTreeMap::new(),
        );
        match synthesize(Mode::Sine, &cmd) {
            Err(err) => assert_eq!(err.category, ExitCategory::DataError),
            Ok(_) => panic!("expected a splice-length DataError"),
        }
    }

    fn find_data_chunk(bytes: &[u8]) -> &[u8] {
        let mut pos = 12usize;
        while pos + 8 <= bytes.len() {
            let id = &bytes[pos..pos + 4];
            let size = u32::from_le_bytes(bytes[pos + 4..pos + 8].try_into().unwrap()) as usize;
            if id == b"data" {
                return &bytes[pos + 8..pos + 8 + size];
            }
            pos += 8 + size + (size % 2);
        }
        panic!("no data chunk found");
    }

    fn peak_sample(bytes: &[u8]) -> i16 {
        let data = find_data_chunk(bytes);
        data.as_chunks::<2>()
            .0
            .iter()
            .map(|b| i16::from_le_bytes(*b).unsigned_abs())
            .max()
            .unwrap() as i16
    }
}
