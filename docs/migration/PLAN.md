# Implementation Plan: CDP Release 8 in Rust

Date: 2026-09-05. Companion to [ANALYSIS.md](ANALYSIS.md) and
[PROGRAMS.md](PROGRAMS.md).

This plan is written for implementation agents. Each work package (WP) names
its inputs, its outputs, and the condition that marks it as done. Work
packages inside one phase can run in parallel unless the text says otherwise.

## 1. Decisions and assumptions

The plan proceeds under the defaults below. Each item is a decision for the
project owner. If a decision changes, the affected work packages are listed.

Confirmed by the project owner on 2026-09-05: D1 (Rust), D2 (full parity,
core first), D3 (same argument grammar, GUI protocol dropped in version 1),
D4 (tolerance-based fidelity), and D9 (keep the C tree as the test oracle).
D5 to D8, D10 and D11 remain defaults.

| ID | Question | Default assumed by this plan | Affects |
|---|---|---|---|
| D1 | Language | Rust, stable toolchain, edition 2024, no nightly features | all |
| D2 | Scope | Full parity with the 220 Linux executables, delivered in priority order (Phase 2 core first) | Phases 2 to 5 |
| D3 | Command-line compatibility | One `cdp` binary with sub-commands (`cdp modify loudness 1 in.wav out.wav 0.5`). Legacy names (`modify`, `pvoc`, ...) work through thin launcher binaries or symlinks that read `argv[0]`. Argument grammar per command is identical to the C programs so that existing batch files and the Learning Manual keep working. | WP-1.5, every program WP |
| D4 | Output fidelity | Sample data equal to the C output within a tolerance (default: maximum absolute difference 1e-4 for 16-bit output, 1e-6 for float, and RMS difference below -80 dBFS). Bit-exact output is not a goal because the C build uses `-ffast-math`. | WP-1.7 and all program WPs |
| D5 | Random processes | Every process that uses random numbers gets a documented `-s<seed>` style option. With a seed, output is reproducible. Default without a seed stays random. Golden tests only run with seeds. | WP-1.4, random program WPs |
| D6 | GUI protocol | The Sound Loom `#`/`##` argument protocol, `cdparams`, `cdparams_other`, `tkusage` and `tkusage_other` are not ported in version 1. The message prefixes `INFO:`, `WARNING:`, `ERROR:` are kept behind a `--gui` flag so that a later WP can restore GUI support. | WP-1.5, Phase 6 |
| D7 | File formats | Read and write every format the C code handles: WAVE, WAVE_EXTENSIBLE, AIFF, AIFC, PVOC-EX, and the CDP derived files with the `sfif` property block, `PEAK` and `cue ` chunks. Existing files from the C tools and from the GUIs stay usable. | WP-1.1 |
| D8 | Audio playback and recording | Not in version 1 (upstream already removed them from the default build). A later WP can add `cdp play` with the `cpal` crate. | Phase 6 |
| D9 | Upstream relationship | Sync the fork with upstream `ComposersDesktop/CDP8` HEAD (28bc42c or newer) before any other work, keep the C tree in the repository as the reference implementation, and stop patching it. | WP-0.1 |
| D10 | Platforms | Linux x86_64 and aarch64, macOS arm64, Windows x86_64. Container image for Linux. | WP-0.3, WP-6.2 |
| D11 | License | LGPL 2.1 or later for the Rust code, because it is a translation of LGPL code. | WP-0.1 |

## 2. Target architecture

A Cargo workspace at the repository root. The C code moves to `legacy/` and
keeps its CMake build so that it can serve as the test oracle.

```
Cargo.toml                 workspace
crates/
  cdp-sf/                  sound file I/O
  cdp-data/                text data formats: breakpoint, mix, note data, tuning, lists
  cdp-params/              declarative command specs, argument parsing, usage text
  cdp-dsp/                 FFT, windows, phase vocoder, interpolation, splices, filters, RNG
  cdp-spectral/            analysis-window iteration helpers shared by spectral programs
  cdp-core/                the Context struct that replaces `dz`, error types, progress reporting
  cdp-programs/            one module per legacy program directory (see section 5)
  cdp-cli/                 the `cdp` binary and the legacy-name launchers
tools/
  oracle/                  golden test harness (runs legacy binaries in Docker or from PATH)
  extract-specs/           scripts that extract parameter tables and usage text from the C code
spec/
  usage/                   captured usage text per legacy program (generated)
  commands/                one TOML file per command: params, modes, flags, ranges, defaults
  golden/                  golden test cases: command line, inputs, expected hashes and tolerances
legacy/                    the C tree (dev/, include/, cmake/, CMakeLists.txt)
docs/
```

### 2.1 Crate responsibilities

`cdp-sf`. Streaming reader and writer for all formats in D7. `u64` sizes.
Sample data is exposed as `f32` frames regardless of the stored type. The
writer produces the `fmt `, `data`, `PEAK`, `cue ` and `LIST/adtl/note`
chunks in the same layout as the C code so that the C tools accept the files.
A `SoundProps` struct mirrors `SFPROPS` in `dev/newinclude/sfsys.h` with an
enum for the file kind (sound, analysis, pitch, transposition, formant,
envelope). No global file table.

`cdp-data`. Parsers and writers for text formats. Breakpoint tables with
linear interpolation, dB conversion, range checks and the same error messages
as `dev/cdp2k/readdata.c`. Mix files, note data files for `texture`, tuning
files, number lists, sound file lists, and the `columns` data model.

`cdp-params`. A `CommandSpec` type: program name, sub-command, modes,
positional parameters with type (`Double`, `DoubleOrBreakpoint`, `Int`,
`IntOrBreakpoint`, `File`), range, default, option flags, variant flags,
special data files, input file kinds, output file kind. Specs are loaded from
`spec/commands/*.toml` at build time (a build script turns them into Rust
tables). One parser handles the legacy argument grammar for all commands. One
formatter prints usage text in the legacy layout.

`cdp-dsp`. `realfft` and `rustfft` for transforms. The phase vocoder
analysis and synthesis from `dev/pv/pvoc.c` with Hamming window, overlap
factors 1 to 4 and the same channel layout (`amp, freq` pairs). Splice
functions, interpolators, envelope followers, biquad and FIR filters,
waveset detection, and a seedable random generator with the same
distributions the C code uses (`drand48` uniform).

`cdp-spectral`. The window iteration pattern of `outer_loop` in
`dev/cdp2k/tklib3.c`: read a window, apply per-window parameters from
breakpoint tables, write a window. Helpers for channel to frequency mapping,
partial tracking, formant extraction (`dev/cdp2k/formantsg.c`).

`cdp-core`. `Context` replaces the `datalist` god-struct. It holds the parsed
command, the open inputs and outputs, the parameter tables, and a progress
reporter. Errors are an enum with the legacy exit codes. No `unsafe`. No
global mutable state.

`cdp-programs`. Every legacy program becomes a module that registers its
`CommandSpec`s and a `run(ctx: &mut Context) -> Result<()>`. Modules are
grouped in directories that mirror the legacy directory names so that an
agent can find the C source quickly.

`cdp-cli`. Builds the sub-command tree from the registered specs. Legacy
launcher: when `argv[0]` is a legacy program name, the arguments are passed
unchanged to that program's parser. Supports `--version`, `--help`, and
`--gui` (D6).

## 3. Porting method for one program

Every program WP follows the same recipe. Agents must not skip steps.

1. Read `spec/usage/<program>.txt` and the C source directory named in the
   WP. Read the `ap_*.c` file first for group programs. Read the whole file
   for standalone programs.
2. Write or complete `spec/commands/<program>/<subcommand>.toml`. Take
   parameter types from the `set_param_data` string, ranges and defaults from
   `setup_*_param_ranges_and_defaults`, flags from `set_vflgs`, file kinds
   from `set_legal_infile_structure`. Record every deviation from the usage
   text in a `notes` field.
3. Produce golden cases with the oracle tool: at least three cases per mode,
   at least one with a breakpoint file where a parameter accepts one, at least
   one with a stereo input, and one error case. Use the corpus in
   `docs/manual/sounds`. Random processes use a fixed seed (D5).
4. Port the processing code. Keep the algorithm structure of the C code so
   that a reviewer can compare function by function. Replace the `dz` field
   accesses with `Context` fields. Replace `goto` error paths with `?`.
   Replace whole-file buffers with streaming where the C code streams, and
   keep whole-file buffers where the algorithm needs random access.
5. Run `cargo test -p cdp-programs --test golden -- <program>` until every
   case passes at the tolerance in D4. If a case cannot pass, document the
   reason in the TOML `notes` and tag the case `known-deviation`.
6. Run `cargo clippy --all-targets -- -D warnings` and `cargo fmt`.
7. Update `docs/migration/STATUS.md` with the program state
   (`ported`, `partial`, `deviation`, `dropped`).

## 4. Phases and work packages

### Phase 0: Repository preparation (sequential)

WP-0.1 Sync with upstream and restructure.
Inputs: this repository, upstream `ComposersDesktop/CDP8`.
Steps: merge upstream main, delete `docker-cdp8/fix-linux-compat.sh` and
its call in the Dockerfile, move `dev/`, `include/`, `cmake/`,
`CMakeLists.txt`, `building.txt` and `libaaio/` to `legacy/`, remove
`legacy/dev/sfsys` (not built) and `legacy/include` duplicates after
confirming the build still passes, add the workspace `Cargo.toml`, add the
LGPL header template for Rust files.
Done when: `docker build` of the legacy image produces 220 executables from
`legacy/`, and `cargo build` of an empty workspace succeeds.

WP-0.2 Capture the specification.
Steps: write `tools/extract-specs/dump-usage.sh` that runs every legacy
executable with no arguments and with each sub-command name and stores the
text in `spec/usage/`. Write `tools/extract-specs/parstruct.py` that parses
`legacy/dev/cdp2k/parstruct.c`, `validate.c`, `dev/cdparams/parnames.c` and
each `ap_*.c` into a first draft of `spec/commands/*.toml` for the 452 group
process and mode pairs. Standalone programs are drafted by hand in their own
WPs.
Done when: `spec/usage/` has one file per program and sub-command and
`spec/commands/` has a draft for every group program.

WP-0.3 Continuous integration.
Steps: GitHub Actions workflow with `cargo fmt --check`, `clippy`, `test` on
Linux, macOS and Windows, plus a Linux job that builds the legacy image and
runs the golden suite. Cache the legacy image.
Done when: the workflow is green on an empty workspace.

### Phase 1: Foundation crates (parallel after Phase 0)

WP-1.1 `cdp-sf`.
Inputs: `legacy/dev/newsfsys/sfsys.c`, `snd.c`, `props.c`, `ieee80.c`,
`legacy/dev/pvxio2/pvfileio.c`, `legacy/dev/newinclude/*.h`, ANALYSIS.md
section 3.4.
Outputs: reader and writer, `SoundProps`, chunk model, property block codec.
Tests: round trip of every sample type and format, byte-level comparison of
headers against files written by `legacy` `synth`, `pvoc anal`,
`repitch getpitch`, `formants get`, `envel extract`, and `pvoc anal` with
`.pvx` output. Reading of the 553 WAV and 120 AIFF files in `docs/manual`.
Done when: `sndinfo props` implemented on top of it prints the same values as
the legacy program for the whole corpus.

WP-1.2 `cdp-data`.
Inputs: `legacy/dev/cdp2k/readdata.c`, `readfiles.c`, `tklib1.c`
(breakpoint helpers), `legacy/dev/submix/setupmix.c` (mix file grammar),
`legacy/dev/texture/*.c` (note data grammar), example files in
`docs/manual/data`.
Done when: every example data file in `docs/manual/data` parses, and error
messages for the error cases in `readdata.c` match the legacy text.

WP-1.3 `cdp-params` and the spec loader.
Inputs: `spec/commands/*.toml` drafts, `legacy/dev/cdp2k/parstruct.c`,
`readdata.c`, `tklib3.c` (`setup_particular_application`), the usage layout
in `legacy/dev/misc/tkusage.c`.
Done when: the parser accepts every golden command line for `modify loudness`
and `pvoc anal` and rejects the same inputs the legacy parser rejects with
the same `ERROR:` text.

WP-1.4 `cdp-dsp`.
Inputs: `legacy/dev/pv/pvoc.c`, `mxfft.c`, `legacy/dev/cdp2k/tklib3.c`
(splices, interpolation, ring buffers), `legacy/dev/newsfsys/osbind.c`
(`initrand48`).
Done when: `pvoc anal` and `pvoc synth` round trips match legacy output at
the D4 tolerance for all three analysis modes, four overlap values and
window sizes 256 to 4096, on mono and stereo inputs.

WP-1.5 `cdp-core` and `cdp-cli`.
Inputs: `legacy/dev/modify/main.c` (the canonical lifecycle),
`legacy/dev/cdp2k/mainfuncs.c`, `writedata.c`, `tklib1.c`
(`sound_loom_in_use`, `print_outmessage`).
Done when: `cdp synth wave ...`, `synth wave ...` through the launcher,
`--version`, and `--help` work and exit codes match legacy.

WP-1.6 `cdp-spectral`.
Inputs: `legacy/dev/cdp2k/tklib3.c` (`outer_loop` and helpers),
`formantsg.c`, `legacy/dev/spec/simple.c`.
Done when: `spec gain` and `spec gate` match legacy.

WP-1.7 Golden test harness.
Inputs: ANALYSIS.md section 6, `tools/oracle` design in section 5 below.
Done when: a `spec/golden/*.toml` case can be recorded from the legacy image
and replayed against the Rust binary with chunk-level comparison and the
tolerance rules of D4, and the CI job runs it.

### Phase 2: Core time-domain and utility programs

Each WP is one legacy directory. Effort in the table is legacy C lines.

| WP | Program(s) | Legacy dir | Lines | Notes |
|---|---|---|---|---|
| 2.1 | `sndinfo` | dev/sndinfo | 3,125 | first end-to-end program, mostly done in WP-1.1 |
| 2.2 | `housekeep` | dev/houskeep | 5,705 | copy, chans, respec, bundle, gate, deglitch |
| 2.3 | `synth`, `newsynth` | dev/synth | 4,120 | contains its own pvoc copy (`pvoc_addon.c`) |
| 2.4 | `sfedit` | dev/editsf | 6,299 | cut, join, insert, twixt |
| 2.5 | `modify` | dev/modify | 13,042 | 13 sub-commands; brassage is random (D5) |
| 2.6 | `envel` | dev/env | 7,361 | 22 sub-commands, envelope file format |
| 2.7 | `submix` | dev/submix | 9,576 | mix file grammar, 23 sub-commands |
| 2.8 | `extend` | dev/extend | 7,238 | loop, drunk, zigzag; random |
| 2.9 | `distort` | dev/distort | 9,248 | waveset engine shared by later `dist*` programs |
| 2.10 | `filter` | dev/filter | 6,293 | filter bank data files |
| 2.11 | `grain` | dev/grain | 6,457 | grain detection engine |
| 2.12 | `columns`, `getcol`, `putcol`, `vectors` | dev/tabedit | 15,413 | text tools, no audio |
| 2.13 | `dirsf`, `diskspace`, `listdate`, `logdate`, `brkdur`, `maxsamp2`, `pdisplay`, `progmach`, `stretcha`, `vuform`, `histconv`, `gobo` family, `pmodify`, `paudition` | dev/sfutils, dev/misc | 19,800 | small utilities, several are GUI helpers and can be marked `dropped` under D6 |
| 2.14 | `mctools`: `abfpan`, `abfpan2`, `abfdcode`, `interlx`, `njoin`, `nmix`, `chxformat`, `chorder`, `channelx`, `copysfx`, `rmsinfo`, `sfprops`, `fmdcode` | dev/externals/mctools | 7,000 | use `portsf` in C, use `cdp-sf` in Rust |

### Phase 3: Spectral programs

| WP | Program(s) | Legacy dir | Lines |
|---|---|---|---|
| 3.1 | `pvoc` (anal, synth, extract) | dev/pv | 2,534 |
| 3.2 | `spec` | dev/spec | 1,275 |
| 3.3 | `blur` | dev/blur | 2,699 |
| 3.4 | `stretch` | dev/stretch | 1,526 |
| 3.5 | `pitch`, `pitchinfo` | dev/pitch, dev/pitchinfo | 3,594 |
| 3.6 | `repitch` | dev/repitch | 6,143 |
| 3.7 | `formants` | dev/formants | 2,447 |
| 3.8 | `morph`, `newmorph` | dev/morph, dev/new | 1,790 + part |
| 3.9 | `focus` | dev/focus | 2,432 |
| 3.10 | `hilite` | dev/hilite | 3,424 |
| 3.11 | `strange` | dev/strange | 2,568 |
| 3.12 | `combine` | dev/combine | 1,604 |
| 3.13 | `specinfo` | dev/specinfo | 1,707 |
| 3.14 | `hfperm` | dev/hfperm | 3,299 |
| 3.15 | `texture`, `texmchan` | dev/texture, dev/standalone | 8,178 + part |

### Phase 4: Standalone programs (bulk, highly parallel)

Each of the roughly 150 standalone programs in `dev/standalone`,
`dev/standnew`, `dev/science` and `dev/new` is one WP. A WP is one C file
(occasionally two) and one Rust module. Suggested batching by size:

- Small (under 1,500 lines, about 60 programs): one agent, one day each.
- Medium (1,500 to 4,000 lines, about 70 programs): one agent, two to four
  days each.
- Large (over 4,000 lines: `specfnu` 9,805, `psow` 8,336, `mchanpan` 4,643,
  `retime` 4,427, `specanal` 4,186, `synthesis` 4,157, `fofex` 4,091): one
  agent each, one to two weeks, split by mode where modes are independent.

Priority inside Phase 4 follows the groups in `docs/r8groups.md`:
waveset distortion (`distcut`, `distmark`, `distmore`, `distrep`,
`distshift`, `partition`, `quirk`, `scramble`, `splinter`), then multichannel
tools, then pvoc tools, then synthesis, then speech tools, then the rest.
Programs marked `NOT IN SOUNDLOOM` in `docs/current-notes.md` go last.

### Phase 5: Externals

| WP | Program | Legacy dir | Notes |
|---|---|---|---|
| 5.1 | `fastconv` | dev/externals/fastconv | C++ with its own FFT, convolution reverb |
| 5.2 | `reverb`, `rmverb`, `rmresp`, `tapdelay` | dev/externals/reverb | C++ |
| 5.3 | `pvplay`, `paplay`, `recsf`, `listaudevs` | dev/externals/paprogs | optional under D8, use `cpal` |

### Phase 6: Retirement and release

WP-6.1 Documentation. Generate a reference page per command from the TOML
specs and the usage text. Update the README and the Learning Manual paths.

WP-6.2 Packaging. `cargo-dist` or equivalent for the platforms in D10, a
Docker image based on `debian:bookworm-slim` with only the `cdp` binary and
the launchers, and a Homebrew formula.

WP-6.3 Legacy retirement. When `STATUS.md` shows every program in scope as
`ported` or `deviation` with an accepted note, move `legacy/` to a tag and
delete it from main. Keep the oracle image published so the golden suite can
still run.

WP-6.4 Optional GUI protocol (only if D6 is revisited). Port
`legacy/dev/cdp2k/tkinput.c`, `cdparams`, and `cdparams_other` as a `--gui`
mode and a `cdp params` sub-command.

## 5. Testing strategy

Three test layers.

1. Unit tests inside each foundation crate.
2. Golden tests. `tools/oracle` records a case by running the legacy command
   inside the Docker image with inputs from `docs/manual/sounds`, storing the
   command line, the SHA-256 of each input, and for each output the `fmt `
   chunk, the parsed `sfif` properties, the `PEAK` values, and a SHA-256 of
   the `data` chunk plus the data itself in a content-addressed cache. Replay
   runs the Rust command, compares `fmt ` and properties exactly, `PEAK`
   within tolerance, and sample data within the D4 tolerance. Text outputs are
   compared after normalising whitespace and dates. Exit code and the first
   line of any `ERROR:` message are compared exactly.
3. Fuzz tests for `cdp-sf` and `cdp-data` parsers with `cargo-fuzz`, because
   these replace the code with the buffer-overflow patterns listed in
   ANALYSIS.md section 5.

Tolerance rules are per case and can be tightened. A case that only passes
at a wider tolerance must say why in its `notes`.

## 6. Conventions for agents

- Rust 2024 edition, `#![forbid(unsafe_code)]` in every crate except a
  clearly justified SIMD kernel in `cdp-dsp`.
- `u64` or `usize` for sample and frame counts. Never `i32`.
- Process audio in `f32` frames and accumulate in `f64` where the C code uses
  `double`.
- Errors: `thiserror` enums in library crates, `anyhow` only in `cdp-cli`.
  Every legacy `sprintf(errstr, ...)` message becomes an error variant with
  the same text so that golden error cases pass.
- No global mutable state. No `static mut`. Progress and messages go through
  the `Context` reporter.
- Keep function names close to the C names in a `// legacy: name()` comment
  so reviewers can cross-reference.
- One commit per WP step (spec, golden cases, port, tests). Commit messages
  name the WP.
- An agent that finds a bug in the C code writes it to
  `docs/migration/LEGACY-BUGS.md` with a reproducer and ports the intended
  behaviour, then marks the golden case `known-deviation`.

## 7. Risks and mitigations

| Risk | Mitigation |
|---|---|
| Volume: 557,599 lines of C, likely 200,000 to 300,000 lines of Rust | Phase 4 is embarrassingly parallel. Priority order lets the project ship a useful subset early. |
| Behaviour only defined by code | WP-0.2 freezes usage text and parameter tables before any port starts. |
| Numeric differences from `-ffast-math`, `float` accumulation, `drand48` | Tolerance-based golden tests (D4), seeds (D5), and a per-case `notes` field. |
| Hidden bugs in the C code (upstream fixed `specfnu` memory errors in June 2026) | `LEGACY-BUGS.md` and `known-deviation` cases. Merge upstream fixes before the affected WP starts. |
| Legacy file compatibility with the two GUIs | WP-1.1 tests headers byte for byte against legacy-written files. |
| GUI users lose Sound Loom integration | D6 keeps the door open with `--gui` and WP-6.4. |
| Upstream keeps changing | WP-0.1 records the synced commit. Re-sync only between phases. |
| License confusion | LGPL headers on every Rust file, a `NOTICE` that names the C authors. |

## 8. Effort estimate

Agent-days assume one agent per WP working with the oracle available.

| Phase | WPs | Agent-days |
|---|---|---|
| 0 | 3 | 5 |
| 1 | 7 | 40 |
| 2 | 14 | 60 |
| 3 | 15 | 45 |
| 4 | about 150 | 300 |
| 5 | 3 | 15 |
| 6 | 4 | 15 |
| Total | | about 480 |

With ten agents in parallel during Phases 2 to 4 the calendar time is about
three to four months after Phase 1 completes.

## 9. Order of work

1. Confirm or change the decisions in section 1.
2. Run Phase 0.
3. Run Phase 1 with up to seven agents. WP-1.5 waits for WP-1.3.
   WP-1.6 waits for WP-1.4. WP-1.7 waits for WP-1.1.
4. Run Phase 2 and Phase 3 in parallel. `modify`, `housekeep`, `sndinfo`,
   `pvoc`, `spec` and `synth` first, because the Learning Manual and most
   batch files depend on them.
5. Run Phase 4 as a queue ordered by the priority in section 4.
6. Run Phase 5 and Phase 6.
