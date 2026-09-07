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

//! The `CommandSpec` data model: what [`crate::parser::parse`] needs
//! to know about one process/mode combination's positional argument
//! list. legacy: this is the runtime shape `set_param_data` (param
//! types, from `legacy/dev/cdp2k/parstruct.c`) and `set_param_ranges`
//! (numeric ranges, from `legacy/dev/cdp2k/tklib1.c`) build into
//! `struct applic` between them, for one process and mode.
//!
//! `docs/migration/PLAN.md`'s architecture section names five
//! positional parameter types: `Double`, `DoubleOrBreakpoint`, `Int`,
//! `IntOrBreakpoint`, `File`. Four of the five are now confirmed
//! against real commands, and turned out to line up exactly with
//! `legacy/dev/cdp2k/tklib3.c`'s `mark_parameter_types`, which reads
//! one of four letters (`i`, `I`, `d`, `D`) off `parstruct.c`'s
//! parameter-list string: lower-case sets `no_brk` (no breakpoint-file
//! fallback), `i`/`I` additionally set `is_int`. `modify loudness`'s
//! `-l<level>` optional flag (modes 3 and 4) is a plain `Double`
//! (lower-case `d`, no breakpoint fallback, confirmed by a live run of
//! `modify loudness 3 infile outfile -labc`, which fails with `"Cannot
//! read parameter 2 [abc]: brkpnt_files not permitted."` rather than
//! trying to open `abc` as a breakpoint file), while mode 1's `gain`
//! positional argument is [`ParamType::DoubleOrBreakpoint`] (upper-case
//! `D`, see `crate::parser`'s module doc). `pvoc anal`'s `-c<points>`
//! and `-o<overlap>` optional flags are [`ParamType::Int`] (lower-case
//! `i`): a live run of `pvoc anal 1 infile outfile -c512.9` completed
//! and produced an analysis file whose channel count reflects `512`,
//! not `513`, confirming the stored value is `(int)` cast (C
//! truncation toward zero, matching Rust's `as i64`) from the same
//! `%lf`-parsed double every other numeric type uses, not rejected for
//! having a fractional part and not rounded. `distort repeat`'s
//! required `multiplier` argument and its `-c<cyclecnt>` optional
//! flag are the fifth and last type, [`ParamType::IntOrBreakpoint`]
//! (upper-case `I`): confirmed live both by a real breakpoint file
//! (`distort repeat infile outfile brk_int.txt`, which runs to
//! completion) and by the same `(int)`-cast truncation `Int` shows
//! (an out-of-range check on a fractional value reports the
//! unrounded, untruncated double, e.g. `"Parameter[1] Value
//! (1.000000) ..."`, confirming the range check itself runs before
//! any truncation, exactly mirroring `Int`).

/// One parameter's type and (for the numeric types) valid range.
/// legacy: one `char` of `parstruct.c`'s `param_list`/`opt_list`
/// string, paired with the `ap->lo[paramno]`/`ap->hi[paramno]` pair
/// `set_param_ranges` fills in for that same `paramno`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParamType {
    /// legacy: an upper-case `D` entry. A token that does not parse
    /// as a plain number is retried as a breakpoint file
    /// (`legacy/dev/cdp2k/readdata.c`'s
    /// `get_brkpnt_data_from_file_and_test_it`); [`crate::ParamValue::Breakpoint`]
    /// carries the result, range-checked point-by-point exactly as
    /// [`crate::ParamValue::Number`] is range-checked as a whole.
    DoubleOrBreakpoint { lo: f64, hi: f64 },
    /// legacy: a lower-case `d` entry -- no breakpoint-file fallback.
    /// An unparseable token is rejected outright with `"Cannot read
    /// parameter {legacy_index} [{token}]: brkpnt_files not
    /// permitted."`, confirmed live for `modify loudness 3`/`4`'s
    /// `-l` flag. `legacy_index` is that message's own parameter
    /// number, which -- confirmed by the same live run -- is *not*
    /// the same number [`crate::error::ParamsError::ValueOutOfRange`]
    /// uses for the very same parameter (that one shows `Parameter[1]`
    /// for `-l`, this shows `parameter 2`): legacy's own value-range
    /// check and its unparseable-token check are evidently two
    /// separate code paths numbering parameters two different ways,
    /// not a single scheme this crate could compute generally. Until
    /// a second real `Double` parameter is confirmed, `legacy_index`
    /// is carried as a literal per-parameter fact, not derived.
    Double {
        lo: f64,
        hi: f64,
        legacy_index: usize,
    },
    /// legacy: a lower-case `i` entry. Parsed exactly like
    /// [`Self::Double`] (the same `%lf`-style tokenizer, the same
    /// unparseable-token and range-check errors, and the same
    /// `legacy_index` caveat) since `mark_parameter_types` only turns
    /// `is_int`/`no_brk` into a later `(int)` cast, not a different
    /// read function; [`crate::ParamValue::Integer`] carries that cast
    /// applied to the already range-checked value. Confirmed live by
    /// `pvoc anal 1 infile outfile -c512.9` and `-o` (see this
    /// module's doc).
    Int {
        lo: f64,
        hi: f64,
        legacy_index: usize,
    },
    /// legacy: an upper-case `I` entry -- [`Self::Int`]'s breakpoint-
    /// capable counterpart, exactly as [`Self::DoubleOrBreakpoint`] is
    /// to [`Self::Double`]. No `legacy_index` (like
    /// `DoubleOrBreakpoint`, an unparseable token is always retried as
    /// a breakpoint filename, so the "cannot read as a number" error
    /// this crate's `Int`/`Double` produce never applies here).
    /// Confirmed live by `distort repeat`'s required `multiplier`
    /// argument and its `-c<cyclecnt>` flag -- see this module's doc.
    IntOrBreakpoint { lo: f64, hi: f64 },
    /// A positional file argument (an input sound file). legacy: the
    /// generic `"Can't open file %s to read data.\n"` check that
    /// runs on every input file argument before any numeric
    /// parameter is parsed.
    File,
}

/// One `-<letter><value>` flag: either an *option* (legacy:
/// `opt_flags`/`opt_list` in `parstruct.c`, e.g. `LOUDNESS_NORM`/
/// `LOUDNESS_SET`'s `opt_flags = "l"`, `opt_list = "d"`) via
/// [`CommandSpec::flags`], or a value-carrying *variant* (legacy:
/// `varflags`/`varlist` in `set_vflgs`, confirmed live by `distort
/// repeat`'s `-s<skipcycles>`) via [`CommandSpec::variants`] -- the
/// same shape either way, since only *where* [`crate::parser::parse`]
/// looks for it, and which errors it reports, differ (see
/// [`CommandSpec`]'s doc). The value is attached directly after the
/// letter with no space (confirmed live: `-l 0.5`, with a space,
/// fails with `"option parameter missing with flag -l"` -- the same
/// message a bare `-l` with nothing after it produces). A pure
/// boolean variant flag with no value at all (legacy: `vparamcnt`
/// entries beyond the value-carrying ones, e.g. `blur scatter`'s
/// `-r`/`-n`) is not implemented yet -- no real command has confirmed
/// one live.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OptionFlag {
    pub letter: char,
    /// legacy: only `Double`, `DoubleOrBreakpoint`, `Int` and
    /// `IntOrBreakpoint` are confirmed as flag value types so far;
    /// [`ParamType::File`] is meaningless here and never used.
    pub value_type: ParamType,
    /// legacy: the position [`crate::error::ParamsError::ValueOutOfRange`]
    /// reports this flag's value at when it is out of range (see
    /// [`ParamType::Double`]'s doc on why this cannot be computed
    /// generally yet). 1-based, matching the legacy message.
    pub range_check_paramno: usize,
}

/// One process/mode's full argument list: some number of leading
/// [`ParamType::File`] input files, then one output filename (not
/// itself a [`ParamType`] entry -- legacy never range- or
/// existence-checks it during parsing, since it is being created),
/// then [`Self::params`] (required, positional, in order), then
/// [`Self::flags`] (optional, `-<letter><value>`, any order among
/// themselves), then [`Self::variants`] (also optional and
/// `-<letter><value>`, but legacy: `get_options`
/// (`legacy/dev/cdp2k/readdata.c`) only scans a *prefix* of the
/// remaining tokens for known [`Self::flags`] letters, stopping
/// (without erroring) at the first it does not recognise; whatever is
/// left is then scanned entirely as [`Self::variants`] by
/// `get_variants_and_flags`/`get_variant_no`. This makes flags and
/// variants order-dependent relative to each other -- confirmed live
/// by `distort repeat infile outfile 3 -s1 -c2` (variant before
/// option) failing with `"option flag -c out of order on cmdline."`,
/// naming the *option* letter, even though `-s`, not `-c`, is the one
/// out of place -- but each group is still order-independent among
/// its own members ([`Self::flags`]' doc already established this for
/// `pvoc anal`'s two options; `distort repeat` only has one variant,
/// so this crate cannot yet confirm the same for
/// [`Self::variants`]). Confirmed live that a leftover token
/// unrecognised by either group reports differently depending on
/// whether the mode has any flags/variants at all: `"Too many
/// parameters on command line."` when both are empty (`modify
/// loudness` mode 1), `"Unknown parameter '<token>'"` otherwise
/// (`modify loudness` modes 3/4, `pvoc anal`, and `distort repeat`,
/// confirmed with a required param, an option, and a variant all
/// present at once).
#[derive(Debug, Clone)]
pub struct CommandSpec {
    /// Number of leading input-file arguments, before the single
    /// output filename. legacy: `ap->max_infile_cnt` for a mode with
    /// a fixed input count (`modify loudness` always takes exactly
    /// one).
    pub infile_count: usize,
    pub params: Vec<ParamType>,
    pub flags: Vec<OptionFlag>,
    pub variants: Vec<OptionFlag>,
    /// legacy: `UNEQUAL_SNDFILE` (`true`) vs `EQUAL_SNDFILE` (`false`,
    /// the default for every mode ported so far) in
    /// `setup_process_logic` (e.g. `legacy/dev/distort/ap_distort.c`).
    /// `legacy/dev/cdp2k/mainfuncs.c`'s `count_infiles` derives the
    /// actual infile count from the command line for an
    /// `UNEQUAL_SNDFILE` mode, rather than using a fixed
    /// [`Self::infile_count`]; this crate does not implement that in
    /// general (every mode ported so far, including `distort repeat`,
    /// only ever takes one infile in practice, whichever category it
    /// is in). The one confirmed, implemented difference this makes:
    /// when the command line is too short even for the infile,
    /// outfile and required params combined, an `EQUAL_SNDFILE` mode
    /// reports [`crate::error::ParamsError::InsufficientCmdlineParameters`]
    /// (`"Insufficient cmdline parameters."`), but an `UNEQUAL_SNDFILE`
    /// mode instead reports
    /// [`crate::error::ParamsError::InsufficientParameters`]
    /// (`"Insufficient parameters on command line."`) -- confirmed
    /// live for `distort repeat` with both just an infile, and an
    /// infile plus outfile, given.
    pub unequal_sndfile: bool,
}

impl CommandSpec {
    /// `modify loudness 1` (`LOUDNESS_GAIN`): `modify loudness 1
    /// infile outfile gain`. legacy: `parstruct.c`'s
    /// `set_param_data(ap, 0, 2, 1, "D0")` for the parameter list,
    /// `tklib1.c`'s `set_param_ranges`' `case(LOUDNESS_GAIN): ap->lo
    /// = 0.0; ap->hi = (double)MAXSHORT;` for the range. Confirmed
    /// against live `legacy` runs -- see `crate::parser`'s module
    /// doc.
    pub fn modify_loudness_gain() -> Self {
        CommandSpec {
            infile_count: 1,
            params: vec![ParamType::DoubleOrBreakpoint {
                lo: 0.0,
                hi: 32767.0, // legacy: MAXSHORT
            }],
            flags: vec![],
            variants: vec![],
            unequal_sndfile: false,
        }
    }

    /// `modify loudness 3` (`LOUDNESS_NORM`): `modify loudness 3
    /// infile outfile [-llevel]`. legacy: `parstruct.c`'s
    /// `opt_flags = "l"`, `opt_list = "d"`; `tklib1.c`'s
    /// `set_param_ranges`' `case(LOUDNESS_NORM): case(LOUDNESS_SET):
    /// ap->lo[LOUD_LEVEL] = 1.0/MAXSHORT; ap->hi[LOUD_LEVEL] = 1.0;`.
    /// `legacy_index` (`2`) and `range_check_paramno` (`1`) are both
    /// confirmed live -- see `crate::parser`'s module doc.
    pub fn modify_loudness_normalise() -> Self {
        CommandSpec {
            infile_count: 1,
            params: vec![],
            flags: vec![OptionFlag {
                letter: 'l',
                value_type: ParamType::Double {
                    lo: 1.0 / 32767.0, // legacy: 1.0/MAXSHORT
                    hi: 1.0,
                    legacy_index: 2, // legacy: LOUD_LEVEL(1) + 1
                },
                range_check_paramno: 1,
            }],
            variants: vec![],
            unequal_sndfile: false,
        }
    }

    /// `modify loudness 4` (`LOUDNESS_SET`): `modify loudness 4
    /// infile outfile [-llevel]`. legacy: structurally identical to
    /// [`Self::modify_loudness_normalise`] in `parstruct.c` and
    /// `tklib1.c` (the same `case(LOUDNESS_NORM): case(LOUDNESS_SET):`
    /// range-setting code handles both), confirmed independently live
    /// -- see `crate::parser`'s module doc.
    pub fn modify_loudness_force_level() -> Self {
        Self::modify_loudness_normalise()
    }

    /// `pvoc anal` (`PVOC_ANAL`, all three modes -- `STANDARD ANALYSIS`,
    /// `OUTPUT SPECTRAL ENVELOPE VALS ONLY`, `OUTPUT SPECTRAL MAGNITUDE
    /// VALS ONLY`): `pvoc anal mode infile outfile [-cpoints]
    /// [-ooverlap]`. legacy: `parstruct.c`'s `set_param_data(ap,0,0,0,"")`
    /// (no required positional parameters at all -- confirmed live: a
    /// leftover positional word reports `"Unknown parameter '<word>'"`,
    /// the same as `modify loudness` modes 3/4, not `"Too many
    /// parameters..."`) and `set_vflgs(ap,"co",2,"ii","",0,0,"")` (two
    /// plain `Int` optional flags, no variant flags -- despite the name
    /// `set_vflgs`, `vflagcnt` is `0` here). Ranges from `tklib1.c`'s
    /// `set_param_ranges`: `ap->lo[PVOC_CHANS_INPUT]=2`,
    /// `ap->hi[PVOC_CHANS_INPUT]=MAX_PVOC_CHANS` (`32768`, from
    /// `legacy/dev/include/pvoc.h`); `ap->lo[PVOC_WINOVLP_INPUT]=1`,
    /// `ap->hi[PVOC_WINOVLP_INPUT]=4`. Both flags' `legacy_index` and
    /// `range_check_paramno` are confirmed live to be the same number
    /// as each other (`1` for `-c`, `2` for `-o`) -- unlike `modify
    /// loudness`'s `-l` flag, where those two numbers differ -- but
    /// each is still carried as its own literal fact, not derived from
    /// the other, for the reason [`ParamType::Double`]'s doc gives.
    pub fn pvoc_anal() -> Self {
        CommandSpec {
            infile_count: 1,
            params: vec![],
            flags: vec![
                OptionFlag {
                    letter: 'c',
                    value_type: ParamType::Int {
                        lo: 2.0,
                        hi: 32768.0, // legacy: MAX_PVOC_CHANS
                        legacy_index: 1,
                    },
                    range_check_paramno: 1,
                },
                OptionFlag {
                    letter: 'o',
                    value_type: ParamType::Int {
                        lo: 1.0,
                        hi: 4.0,
                        legacy_index: 2,
                    },
                    range_check_paramno: 2,
                },
            ],
            variants: vec![],
            unequal_sndfile: false,
        }
    }

    /// `distort repeat` (`DISTORT_RPT`, single-mode, `maxmode=0`):
    /// `distort repeat infile outfile multiplier [-ccyclecnt]
    /// [-sskipcycles]`. legacy: `parstruct.c`'s `set_param_data(ap,0,1,1,"I")`
    /// (one required `IntOrBreakpoint` positional, `multiplier`) and
    /// `set_vflgs(ap,"c",1,"I","s",1,1,"i")` (one `IntOrBreakpoint`
    /// option `-c` = `cyclecnt`, and one plain `Int` variant `-s` =
    /// `skipcycles`). This is the first `CommandSpec` with a required
    /// param *and* flags at once, and the first with any
    /// [`CommandSpec::variants`] at all -- see both those fields' docs
    /// for what's confirmed and what's not. Ranges from `tklib1.c`'s
    /// `set_param_ranges`: `ap->lo[DISTRPT_MULTIPLY]=2`,
    /// `ap->hi[DISTRPT_MULTIPLY]=BIG_VALUE`; `ap->lo[DISTRPT_CYCLECNT]=1`,
    /// `ap->hi[DISTRPT_CYCLECNT]=MAX_CYCLECNT`; `ap->lo[DISTRPT_SKIPCNT]=0`,
    /// `ap->hi[DISTRPT_SKIPCNT]=MAX_CYCLECNT` (`BIG_VALUE` and
    /// `MAX_CYCLECNT` are both `32767.0`, from
    /// `legacy/dev/include/globcon.h`). `multiplier`'s paramno is `1`
    /// as usual; `-c`'s is `2` and `-s`'s is `3`, both confirmed live
    /// to continue the same global, sequential numbering (params, then
    /// options, then variants, in declaration order) rather than
    /// restarting at `1` for the flags -- unlike `pvoc anal`, whose
    /// options *did* start at `1` only because it has no required
    /// params to occupy that slot first. `-s`'s `legacy_index` (`3`)
    /// happens to equal its own paramno, unlike `modify loudness`'s
    /// `-l` flag -- again carried as its own literal fact, not
    /// derived, for the reason [`ParamType::Double`]'s doc gives.
    /// `setup_process_logic` in `legacy/dev/distort/ap_distort.c`
    /// classifies this mode `UNEQUAL_SNDFILE`, hence
    /// `unequal_sndfile: true` -- see that field's doc.
    pub fn distort_repeat() -> Self {
        CommandSpec {
            infile_count: 1,
            params: vec![ParamType::IntOrBreakpoint {
                lo: 2.0,
                hi: 32767.0, // legacy: BIG_VALUE
            }],
            flags: vec![OptionFlag {
                letter: 'c',
                value_type: ParamType::IntOrBreakpoint {
                    lo: 1.0,
                    hi: 32767.0, // legacy: MAX_CYCLECNT
                },
                range_check_paramno: 2,
            }],
            variants: vec![OptionFlag {
                letter: 's',
                value_type: ParamType::Int {
                    lo: 0.0,
                    hi: 32767.0, // legacy: MAX_CYCLECNT
                    legacy_index: 3,
                },
                range_check_paramno: 3,
            }],
            unequal_sndfile: true,
        }
    }
}
