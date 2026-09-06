# CDP Release 8: Analysis for a Modern Reimplementation

Date: 2026-09-05. Analysed commit: 94dbfc1 on branch main.

This document records what the current code is, how it works, and where the
risks are. The companion document [PLAN.md](PLAN.md) turns these findings into
work packages for implementation agents.

## 1. What the project is

The repository is a fork of the Composers Desktop Project (CDP) Release 8
command-line sound-processing suite, published by Richard Dobson at
`ComposersDesktop/CDP8` on GitHub. The programs were written by Trevor Wishart,
Richard Dobson, Martin Atkins and others between 1983 and 2025. The code is C
with a small amount of C++ and is licensed under the LGPL 2.1 or later.

The fork base is upstream commit d347dfb (2025-11-30). The fork adds:

- a multi-stage Dockerfile and a devcontainer,
- a build-time patch script (`docker-cdp8/fix-linux-compat.sh`),
- the Learning Manual converted to Markdown with 933 MB of example sounds,
- README updates.

The fork changes only two source files, and both changes are build fixes.

Upstream moved on after the fork. As of 2026-06-08 upstream HEAD is 28bc42c,
15 commits ahead. Those commits include the same two Linux fixes that the fork
applies with the patch script, memory-allocation bug fixes in `specfnu`, and a
relicense of the `aaio` library to LGPL 2.1 with the author's permission.
Syncing with upstream removes the need for the patch script.

## 2. Size and shape

| Measure | Value |
|---|---|
| C source files | 402 |
| C++ source files | 25 |
| Header files | 111 |
| Lines of C and C++ | 557,599 |
| Executables built on Linux | 220 |
| Group programs with sub-commands | 45 |
| Sub-command names inside group programs | 388 |
| Single-purpose programs | 175 |
| Process identifiers (`processno.h`) | 352 |
| Mode identifiers (`modeno.h`) | 410 |
| (process, mode) parameter specs in `parstruct.c` | 452 |
| Standalone programs that carry a private copy of the framework setup | 112 |
| Help text lines (`fprintf` calls) | about 14,200 |

The user-facing command surface is therefore about 560 named commands, most of
which have several numbered modes. Every command is a separate process that
reads one or more files and writes one or more files.

Lines of code by directory, largest first:

| Directory | Lines | Content |
|---|---|---|
| dev/science | 94,400 | 46 newer Wishart programs (bounce, brownian, specfnu, pulser, ...) |
| dev/standalone | 84,590 | 42 older standalone programs (psow, retime, mchanpan, fofex, ...) |
| dev/new | 60,766 | 33 programs (newtex, fracture, cantor, ...) |
| dev/standnew | 47,320 | 32 programs (clip, distcut, envspeak, fturanal, ...) |
| dev/externals | 27,579 | portsf, fastconv, reverb, mctools, PortAudio players |
| dev/cdp2k | 20,384 | the shared framework library |
| dev/misc | 19,053 | utilities, `tkusage` help tables |
| dev/tabedit | 15,413 | `columns`, `vectors`: text and number list tools |
| dev/cdparams_other | 13,178 | GUI parameter tables for standalone programs |
| dev/modify | 13,042 | modify group |
| dev/sfsys | 11,038 | old sound file system, not built |
| dev/newsfsys | 10,865 | sound file system in use |
| others | about 140,000 | 30 group programs (blur, distort, env, filter, grain, submix, texture, ...) |

## 3. Architecture

### 3.1 Two program lineages

Group programs (`modify`, `blur`, `submix`, `texture`, and so on) share the
`cdp2k` library. The library holds central tables that describe every process:
`parstruct.c` gives parameter counts and types, `validate.c` gives which
process is valid for which program, `tklib3.c` sets up buffers, and the
`ap_*.c` file in each group directory maps process numbers to the setup and
processing functions. `tkusage.c` holds the help text for these programs.

Standalone programs (directories `standalone`, `standnew`, `science`, `new`,
and parts of `misc`) follow the same pattern but copy the setup code into each
file as static functions. 112 files define their own `setup_*_application`,
and 138 files carry a private copy of `get_tk_cmdline_word`. Each standalone
program is therefore self-contained, which makes it a natural unit of work for
a port.

### 3.2 Lifecycle of every program

Every `main` follows this sequence, with the same function names in both
lineages:

1. `sound_loom_in_use` checks for the GUI sentinel `#` or `##` in `argv[1]`.
2. `sflinit` initialises the sound file system.
3. `establish_datastructure` allocates `dz`, a single `struct datalist` with
   about 200 fields that carries all state (see `dev/include/structures.h`).
4. The process number and mode are read from the command line
   (`get_process_and_mode_from_cmdline`) or from the GUI stream
   (`parse_tk_data`).
5. `setup_particular_application` fills the `applic` struct: parameter count,
   parameter type string (for example `"D0"`), option flags, variant flags,
   special data type, and whether formant data is used.
6. Input files are opened and typed (`parse_infile_and_hone_type`,
   `open_first_infile`, `handle_extra_infiles`). Sound, analysis, pitch,
   transposition, formant, envelope and text files are distinguished.
7. `handle_outfile` creates the output file (or defers creation).
8. `read_parameters_and_flags` reads each parameter as a number or as the name
   of a breakpoint file, and checks the range.
9. `check_param_validity_and_consistency`, `allocate_large_buffers`,
   `param_preprocess`.
10. The process function runs (`groucho_process_file` or `spec_process_file`).
11. `complete_output` writes headers and PEAK data. `print_messages_and_close_sndfiles` reports.

Exit codes: 0 on success, 255 (`FAILED`, -1) on error. Messages go to stdout.
In GUI mode messages carry the prefixes `INFO:`, `WARNING:`, `ERROR:`, `TIME:`.

### 3.3 Parameters and time-varying data

A parameter is either a constant or a breakpoint file. A breakpoint file is a
text file of `time value` pairs, one pair per line, with `;` comments. Times
must increase. Values are interpolated linearly at each sample or analysis
window (`interp_val`, `read_value_from_brktable`). Some parameters accept dB
values (`convert_dB_at_or_below_zero_to_gain`). Flags are glued to their
values (`-r5`, `-s0.5`). Modes are positional integers.

The parameter type string encodes one character per parameter. Observed codes:
`D` double that accepts a breakpoint file, `d` double only, `I` and `i` for
integers, `0` for no parameter. Options (`set_vflgs`) carry their own flag
letters and types. These strings, together with ranges and defaults set in each
program's `setup_*_param_ranges_and_defaults`, are the complete machine-readable
specification of a command's interface.

### 3.4 File formats

The sound file system (`dev/newsfsys`) is a home-grown library that predates
libsndfile. It keeps a table of up to 1000 open files addressed by integer
descriptors, reads and writes through its own buffers, and converts all sample
types to 32-bit float in memory. Output defaults to 16-bit PCM unless a program
or the `CDP_NOCLIP_FLOATS` environment variable selects float output.

Formats read and written:

- RIFF WAVE (PCM 8, 16, 24, 32 bit, IEEE float) and WAVE_FORMAT_EXTENSIBLE
  with channel masks and Ambisonic B-format GUIDs.
- AIFF and AIFC (`ieee80.c` encodes the 80-bit sample rate).
- PVOC-EX (`.pvx`), a WAVE_EXTENSIBLE variant with an 80-byte format chunk
  (`dev/pvxio2/pvfileio.c`, `dev/newinclude/pvfileio.h`).
- CDP derived files: analysis (`.ana`), pitch (`.frq`), transposition (`.trn`),
  formant (`.for`), envelope (`.evl`). These are mono or multi-channel
  IEEE-float WAVE files whose meaning is carried in a proprietary property
  block.

The property block lives in a `LIST`/`adtl`/`note` chunk with the marker
`sfif`. Properties are name and value pairs. Names in use:
`sample type`, `sample rate`, `channels`, `original sampsize`,
`original sample rate`, `arate`, `analwinlen`, `decfactor`, `orig channels`,
`specenvcnt`, `window size`, `is a pitch file`, `is a transpos file`,
`is a formant file`, `is an envelope`, `is a fofbank file`, `maxamp`,
`maxloc`, `maxrep`, `DATE`. The library also writes a `PEAK` chunk and a
`cue ` chunk. The `DATE` property and the `PEAK` timestamp make two runs of the
same command produce different bytes even when the sample data is identical.

Text data formats are documented in the CDP reference manual (not in this
repository) and appear in `docs/manual/data`: breakpoint files, mix files
(one sound per line with time, channels, gain, pan), note data files for the
texture programs, tuning files, number lists, and sound file lists.

Environment variables: `CDP_SOUND_EXT`, `CDP_OVERWRITE_FILE`,
`CDP_NOCLIP_FLOATS`, `CDP_MEMORY_BBSIZE`.

### 3.5 Signal processing core

- Phase vocoder: `dev/pv/pvoc.c` is a CARL-style analysis and resynthesis
  engine with a Hamming window and the Mayer FFT (`mxfft.c`). Analysis files
  hold, per window, `amp, freq` pairs for `N/2 + 1` channels. Most spectral
  programs (blur, stretch, pitch, repitch, formants, morph, focus, hilite,
  strange, combine, spec, specinfo, pitchinfo, hfperm) iterate windows through
  the `outer_loop` helper in `tklib3.c`.
- Time-domain processes: granular reconstruction (`brassage`, `sausage`),
  waveset distortion (`distort` and the newer `dist*` programs), envelopes,
  filters (`filter`, `filtrage`), delays, mixing (`submix`), editing
  (`sfedit`), extension (`extend`), and the `texture` event generator that
  turns note lists into placed sound events.
- Externals: `fastconv` (FFT convolution), `reverb` and `rmverb`, and the
  multichannel tools (`abfpan`, `abfdcode`, `interlx`, `njoin`, `chxformat`,
  `copysfx`, `rmsinfo`, `sfprops`).
- Five copies of the Mayer FFT exist, and they differ from one another.

### 3.6 GUI integration

Two GUIs drive these programs: Sound Loom (Tcl/Tk, by Wishart) and Soundshaper
(Windows). In GUI mode a program receives `#` or `##` as its first argument
and then reads a fixed sequence of file-property values before the normal
arguments (`dev/cdp2k/tkinput.c`). `cdparams`, `cdparams_other`, `tkusage` and
`tkusage_other` are separate programs that print parameter descriptions and
help text for the GUI to build dialogs. This protocol is only needed if a new
implementation must remain usable from those GUIs.

## 4. Build system

CMake 3.5 with one `CMakeLists.txt` per directory. Each directory sets its own
compiler flags with hard-coded platform defines (`-Dunix -Dlinux -D_X86_`,
`-DWIN32`, `-D__MAC__`). Global flags add `-ffast-math`, `-msse2`,
`-fomit-frame-pointer` and `-Wno-format`. 47 of 51 directories use `-Wall`.

The Docker build on Ubuntu 22.04 with GCC 11 completes with 33 warnings and no
errors and produces 220 executables. The PortAudio player programs (`paplay`,
`pvplay`, `recsf`, `listaudevs`) are commented out of the build since October
2025, so the PortAudio dependency in the Dockerfile and README is no longer
used by the build. The directory `dev/sfsys` is a stale copy of `newsfsys` and
is not built. The top-level `include/` directory duplicates `dev/newinclude/`
with older versions of `sfsys.h` and `props.h`.

Version strings are inconsistent. Programs print `CDP Release 7.1 2016` in
their usage banner and report `7.1.0` or `6.1.0` to `--version`, while CPack
says 8.0.1.

## 5. Quality and risk findings

Counts across `dev/`:

| Pattern | Count | Consequence |
|---|---|---|
| `sprintf` | 19,265 | unbounded formatting into fixed buffers |
| `strcpy` / `strcat` | 2,022 / 772 | overflow with long paths |
| `char temp[200]` style fixed buffers | about 400 | file names longer than 200 bytes overflow |
| `malloc` family / `free` | 6,290 / 946 | memory is rarely freed; process exit hides it |
| `goto` | 819 | control flow that is hard to translate mechanically |
| `drand48` users / seedable programs | 101 files / 4 | 37 programs seed from the clock, output is not reproducible |
| `int` sample counts | everywhere | files above 2^31 samples are not supported |
| `#ifdef unix` / `_WIN32` sites | 312 / 212 | platform logic scattered through algorithm code |
| global `errstr[2400]` definitions | 175 | every program declares the same globals |

Additional observations:

- `-ffast-math` is on for the whole build. Output therefore already depends on
  compiler and platform at the level of the last bits. A port cannot be
  bit-exact against one platform's binaries and must compare with a tolerance.
- 17 programs allocate buffers sized by the whole input file. All others
  stream through fixed buffers.
- The framework relies on a small set of mutable globals (`sloom`,
  `sloombatch`, `errstr`, `anal_infiles`, `is_converted_to_stereo`).
- The per-program reference manual (HTML, maintained by Robert Fraser) is not
  in this repository. The help text inside the programs and the source code are
  the only in-repo specification of behaviour. The Learning Manual in
  `docs/manual` covers workflows and gives 553 WAV and 120 AIFF example files
  that can serve as a test corpus.
- Upstream fixed memory errors in `specfnu` in June 2026. Similar latent
  errors are likely elsewhere in the 290,000 lines of standalone programs.

## 6. Baseline behaviour observed

Commands run inside the Docker image built from this commit:

| Command | Result |
|---|---|
| `synth wave 1 sine.wav 44100 1 2 440` | 2 s 16-bit mono WAV, exit 0 |
| `sndinfo props sine.wav` | reports 88200 samples, PEAK 1.0, exit 0 |
| `modify loudness 1 sine.wav loud.wav 0.5` | PEAK 0.5, exit 0 |
| `pvoc anal 1 sine.wav sine.ana` | 1026-channel float WAVE, arate 344.53, exit 0 |
| `pvoc synth sine.ana resyn.wav` | 47 samples clipped, exit 0 |
| `modify brassage 5 sine.wav out.wav 0.7` | exit 0, different output each run |
| wrong arguments | `ERROR: INCORRECT USE`, exit 255 |

Chunk-level comparison of two runs of `synth wave` with the same arguments:
`fmt `, `data` and `cue ` identical, `LIST` and `PEAK` differ. This confirms
that a golden-file test harness must compare the `data` chunk and the
properties it cares about, not whole files.

## 7. Licensing

- CDP code: LGPL 2.1 or later (`COPYING`, `LICENSE`).
- `portsf`: permissive (MIT-style) by Richard Dobson.
- `aaio`: Attribution Assurance License in this tree, LGPL 2.1 upstream since
  January 2026. It is only used for keyboard polling in five programs.
- `reverb`, `fastconv`, `mctools`: CDP copyright, LGPL.
- Learning Manual: Creative Commons (see `docs/manual/docs/index.md`).

A translation of the algorithms into Rust is a derivative work and stays under
the LGPL 2.1 or later unless the copyright holders agree otherwise.
