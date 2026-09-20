# Migration Plan V2: CDP Release 8 in Rust

Date: 2026-09-19. Author: planning pass by Claude Opus 5.

Status: this document replaces parts of [PLAN.md](PLAN.md). It keeps that
plan's decisions (section 1) and target architecture (section 2) unchanged.
It replaces that plan's porting method (section 3), phase sequencing
(section 4), testing strategy (section 5) and agent conventions (section 6).

Read this document together with [STATUS.md](STATUS.md) and
[LEGACY-BUGS.md](LEGACY-BUGS.md).

## Status at a glance

Updated 2026-09-19. Every box in this document is filled in by hand, so
treat the gate commands as the authority and this summary as the map.

```sh
cargo test --workspace                     # gates 1 and 2
cargo run -p cdp-oracle -- verify          # gate 3
cargo run -p cdp-oracle -- list            # every golden case and its state
```

### Phases

- [x] Phase V. Verification infrastructure. Five of six items done, V5 open.
- [ ] Phase R. Remediation. 0 of 9 sub-commands repaired.
- [ ] Phase 2. Core programs. WP-2.1 and WP-2.2 both open.
- [ ] Phase 3. Spectral programs. Blocked on WP-1.6 (`cdp-spectral`).
- [ ] Phase 4. About 150 standalone programs.
- [ ] Phase 5. Externals.
- [ ] Phase 6. Documentation, packaging, legacy retirement.

### Gates

| Gate | What it checks | Needs Docker | Scope | Passing | Baselined |
|---|---|---|---|---|---|
| 1 | usage text, stderr, exit code | no | 21 sub-commands | 14 | 7 |
| 2 | argument grammar against `parstruct.json` | no | 20 wired modes | 20 | 0 |
| 3 | exit code, stdout, stderr of real runs | no (replay) | 19 golden cases | 8 | 11 |

Gate 3 records against the legacy image but replays without it, so
continuous integration runs all three gates on every pull request.

### Sub-commands wired into `cdp-cli`

A box is checked when the sub-command passes all three gates. A
sub-command with no golden case cannot be called done, even when gates 1
and 2 pass, so the third column is the honest measure.

| Sub-command | Gate 1 | Gate 2 | Golden cases | Done |
|---|---|---|---|---|
| `sndinfo props` | pass | pass | 8 | [x] |
| `sndinfo len` | pass | pass | 0 | [ ] |
| `sndinfo lens` | pass | exempt | 0 | [ ] |
| `sndinfo sumlen` | pass | exempt | 0 | [ ] |
| `sndinfo timediff` | pass | exempt | 0 | [ ] |
| `sndinfo smptime` | pass | pass | 0 | [ ] |
| `sndinfo timesmp` | pass | pass | 0 | [ ] |
| `sndinfo maxsamp` | pass | pass | 0 | [ ] |
| `sndinfo units` | pass | pass (modes 1-2) | 0 | [ ] |
| `sndinfo prntsnd` | FAIL | exempt | 1, deviation | [ ] |
| `sndinfo findhole` | FAIL | exempt | 2, deviation | [ ] |
| `housekeep copy` | pass | pass (mode 1) | 2, both deviation | [ ] |
| `housekeep chans` | pass | pass (modes 1-5) | 0 | [ ] |
| `housekeep respec` | pass | pass (modes 2-3) | 0 | [ ] |
| `housekeep bakup` | FAIL | exempt | 1, deviation | [ ] |
| `housekeep bundle` | FAIL | exempt | 1, deviation | [ ] |
| `housekeep extract` | FAIL | exempt | 1, deviation | [ ] |
| `housekeep remove` | FAIL | exempt | 2, deviation | [ ] |
| `housekeep sort` | FAIL | exempt | 1, deviation | [ ] |
| `synth wave` | pass | pass (4 modes) | 0 | [ ] |
| `pvoc anal` | pass | pass (mode 1) | 0 | [ ] |

One sub-command of 21 currently meets the section 7 definition of done.
That number is the real measure of progress, and it is the number to
watch.

## 1. Why this plan exists

The project shipped 21 sub-commands. Seven of them do not run the command
lines that the legacy programs accept. The defects reached the main branch
through green continuous integration (CI). No test could detect them.

This plan fixes the cause. The cause is not the architecture. The
architecture works. The cause is that PLAN.md made output checking a manual
step, and CI cannot see whether an agent performed a manual step.

## 2. Measured state of the project

The "before" column is the state that motivated this plan. The "now"
column is the state after Phase V.

| Item | Before Phase V | Now |
|---|---|---|
| Legacy C executables in scope | 220 | 220 |
| Legacy sub-commands in scope | about 368 | about 368 |
| Captured usage-text files in `spec/usage/` | 588 | 588 |
| Processes in `spec/commands/_raw/parstruct.json` | 299 | 299 |
| Process and mode pairs in that file | 645 | 645 |
| Rust crates | 7 | 7, plus `cdp-oracle` |
| Sub-commands wired into `cdp-cli` | 21 | 21 |
| Of those, passing gate 1 | unknown | 14 |
| Of those, failing gate 1 | unknown | 7 |
| Wired modes passing gate 2 | unknown | 20 of 20 |
| Tests in the workspace | about 125 in `cdp-programs` | 350 across the workspace |
| Golden cases in `spec/golden/` | 0 | 19 |
| Oracle harness in `tools/oracle/` | absent | `cdp-oracle`, record and verify |
| Sub-commands meeting the section 7 definition of done | 0 | 1 |
| Per-command TOML specs in `spec/commands/` | 0 | 0, and deferred, see section 4 |

Foundation crates `cdp-sf`, `cdp-data`, `cdp-params`, `cdp-dsp`, `cdp-core`
and `cdp-cli` all work and carry real tests. `cdp-spectral` does not exist
yet. Four launcher binaries exist: `cdp`, `sndinfo`, `housekeep` and `synth`.

The legacy oracle image `cdp8-postmerge` builds and runs. That image is the
reference implementation for every check in this plan.

## 3. The defects found

Two eras of work are visible in the git history. The boundary is commit
`ea425dc` (pull request 33), the last commit that updated STATUS.md.

Work before that boundary followed PLAN.md section 3. Each sub-command
carries corpus regression tests and a STATUS.md entry that records live
comparisons against the legacy image. That work is sound.

Work after that boundary ran in an autonomous loop. It produced nine commits
and eight sub-commands. It wrote no STATUS.md entry and ran no legacy
comparison. Two of those commits went to main without a pull request.

The table below shows real runs. The legacy column is the output of
`cdp8-postmerge`. The Rust column is the output of the same command line
against `target/debug`.

| Command line | Legacy result | Rust result |
|---|---|---|
| `sndinfo findhole a.wav` | `Maximum holesize is 0.000000 at time 0.000000` | rejects: `Insufficient parameters on command line.` |
| `sndinfo prntsnd a.wav o.txt 0.0 0.001` | writes the text file | rejects: `Invalid starttime: o.txt` |
| `housekeep bakup a.wav b.wav out.wav` | writes a 356432 byte file | rejects: `bakup requires ... splicelen` |
| `housekeep sort 1 list.txt` | creates `list.mot` | rejects: `sort requires an input text file and output text file` |
| `housekeep extract 4 a.wav e.wav 0.0` | fails: `NO CHANGE to original sound file.` | writes a file and ignores the `shift` argument |
| `housekeep remove a.wav -a` | fails inside legacy, see section 3.2 | ignores the `-a` flag |

Five of the six reject a command line that legacy accepts. The sixth accepts
the command line and computes a different result.

`housekeep bundle` and `housekeep copy` mode 2 come from the same era. No one
checked them. Treat them as suspect until section 5 clears them.

### 3.1 How each defect happened

Every defect has the same shape. The agent invented the argument grammar and
the output text instead of reading the captured specification.

The repository already holds the correct answer for each case. The file
`spec/usage/sndinfo/findhole.txt` records the real usage text, captured from
a real run:

```
USAGE: sndreport findhole infile [-tthreshold]
THRESHOLD  hole only if level falls and stays below threshold (default: 0).
```

The shipped code declares a required positional `threshold` instead of the
optional flag `-t`. The file `spec/commands/_raw/parstruct.json` also records
the answer, as `INFO_FINDHOLE` with `param_cnt: 0` and `opt_flags: "t"`.

The same two files contradict every other defect in the table:

- `HOUSE_BAKUP` has `param_cnt: 0`. The shipped `splicelen` argument does not
  exist. The real gap is the constant `BAKUP_GAP`, which equals 1.0 second,
  rounded up to a whole disk sector.
- `HOUSE_SORT` has six named modes: `BY_FILETYPE`, `BY_SRATE`, `BY_DURATION`,
  `BY_LOG_DUR`, `IN_DUR_ORDER` and `FIND_ROGUES`. The shipped code maps mode
  1 to sample rate and mode 2 to file type. Legacy maps them the other way.
  The shipped mode 4, "channel count", does not exist in legacy.
- `HOUSE_SORT` is `WORDLIST_ONLY` and `NO_OUTPUTFILE`. The shipped code
  demands an output filename on the command line.
- `HOUSE_DEL` has the variant flag `a` and uses the `SNDFILENAME` special-data
  mechanism. The shipped code has neither.
- `INFO_PRNTSND` has `param_cnt: 2`. The two doubles are the times. The text
  file is an output file. The shipped code prints to standard output instead.

No defect required new research to find. Each one contradicts a file that was
already committed.

### 3.2 Why continuous integration did not stop this

Two separate failures let the defects merge.

First, the gates did not exist. The `golden-tests` job in
`.github/workflows/ci.yml` passes when `tools/oracle/run.sh` is absent,
and it is absent, so the job has never checked anything.

Second, the merge step misread the checks it waited for. The wait loop
tested the pull request's check output with `grep -q
'"conclusion":"SUCCESS"'`, which matches when any single check succeeds.
The `Rust` job was failing on `cargo clippy -D warnings` for every one of
the loop-era pull requests, and the loop merged them anyway. Phase V
repairs the first failure. Any autonomous runner must require that every
check succeeds, not that one does.

### 3.3 A legacy bug found during this analysis

The command `housekeep remove a.wav -a` fails inside legacy itself:

```
ERROR: INTERNAL ERROR: (Bug?)
ERROR: Invalid process_type 20: create_sized_outfile()
```

Add this to LEGACY-BUGS.md during Phase R. The `-a` flag appears in the usage
text and in `parstruct.json`, but the legacy code path aborts. Decide whether
to port the intended behavior, per PLAN.md section 6.

## 4. What changes

The porting method changes in one way. Machines check the work, not agents.

PLAN.md section 3 told agents to record golden cases and compare output. That
instruction is correct. It is also unenforceable, because CI never learned
which commands had golden cases and which did not. An agent that skipped
steps 3 and 5 produced a green build.

V2 replaces the instruction with three gates. A gate is a test that fails on
its own when work is missing or wrong. Two of the three gates need no Docker
and no audio comparison, so they run on every pull request in seconds.

### Gate 1: usage-text conformance

For every sub-command wired into `cdp-cli`, invoke it with no arguments.
Compare the output to `spec/usage/<program>/<subcommand>.txt` byte for byte,
after the documented trailing-newline rule.

The test enumerates the dispatch table. If a sub-command has no matching
usage file, the test fails. An agent cannot add a command and skip the gate.

This gate alone rejects the shipped `findhole`, `prntsnd`, `bakup` and `sort`,
because all four carry invented usage text.

### Gate 2: argument-grammar conformance

Load `spec/commands/_raw/parstruct.json`. For every ported process and mode,
compare the Rust `CommandSpec` against the recorded grammar:

- the count of required positional parameters,
- the type letter of each parameter,
- the set of option-flag letters and their type letters,
- the set of variant-flag letters,
- the `special_data` kind.

This gate rejects all six defects in the table, including `extract` mode 4,
which Gate 1 cannot catch because `extract` shares one usage text across six
modes.

Gate 2 also removes the blocker that stalled the TOML specification loader.
PLAN.md WP-1.3 deferred that loader because WP-0.2 never finished the mapping
from process symbols to command-line names. Gate 2 needs no such mapping.
Each ported command names its own process symbol in one line of Rust.

### Gate 3: golden output cases

Run recorded command lines and compare every output against recorded
expectations. This gate covers behavior that grammar cannot describe: sample
data, printed reports, exit codes and error text.

## 5. Phase R: remediation (do this first)

Goal: the main branch contains no unchecked command.

Phase V is done enough to start, so R4's precondition is met. Every
sub-command below has a recorded golden case, so the work is to turn
`cargo run -p cdp-oracle -- verify` from KNOWN lines into passes, then
delete the `known_deviation` marker and the `KNOWN_FAILURES` entry. Both
markers fail the build if they outlive their defect, so nothing needs to
be remembered.

- [x] **R1. Diagnose the two suspect commands.** Done. `housekeep bundle`
  prints one trailing newline more than legacy. `housekeep copy` mode 2
  produces the correct duplicate files but prints a progress tick that
  legacy does not print for that mode.
- [ ] **R2. Repair or withdraw each command**, per the table below.
- [ ] **R3. Write the STATUS.md entries the autonomous run never wrote.**
- [x] **R4. Wait for gates 1 and 2 before repairing.** Done, and gate 3
  as well, which turned out to matter more: it found a defect in
  `housekeep copy` mode 1 that the earlier prose-verified process had
  missed.

### R2 work list

| Sub-command | Action | Defect | Done |
|---|---|---|---|
| `sndinfo findhole` | repair | takes a required positional threshold. Legacy takes an optional `-t` flag and prints usage when bare | [ ] |
| `sndinfo prntsnd` | repair | takes three arguments and prints to standard output. Legacy takes four and writes a text file | [ ] |
| `housekeep extract` mode 4 | repair | ignores the required `shift` argument and computes its own mean | [ ] |
| `housekeep remove` | repair | missing the `-a` flag. Also needs a LEGACY-BUGS.md entry, see section 3.3 | [ ] |
| `housekeep bundle` | repair | prints one trailing newline more than legacy | [ ] |
| `housekeep copy` mode 1 | repair | prints one trailing newline more than legacy, 19 bytes against 18 | [ ] |
| `housekeep copy` mode 2 | repair | prints a progress tick legacy does not print for this mode | [ ] |
| `housekeep bakup` | withdraw, then re-port | invents a `splicelen` argument. The real gap is `BAKUP_GAP` (1.0 second) rounded up to a whole disk sector | [ ] |
| `housekeep sort` | withdraw, then re-port | wrong mode numbering, a mode that does not exist, and an outfile legacy has no argument for | [ ] |

Withdrawing means removing the dispatch arm, the module and the registry
entry, which leaves the sub-command unimplemented. Withdraw any command
whose repair needs a mechanism the workspace lacks, rather than narrowing
the command until it fits.

### The two trailing-newline defects

`housekeep copy` mode 1 and `housekeep bundle` each print one newline too
many. The `copy` defect matters beyond its own repair, because `copy` mode
1 was verified by hand against legacy and recorded in STATUS.md as
byte-for-byte correct. It was not.

The likely cause is the shell. A check written as `out=$(cmd)` strips
every trailing newline, so a difference that is only trailing newlines
cannot be seen that way. Gate 3 compares captured bytes and found it
immediately.

This does not overturn section 6's finding that the two eras differ in
kind. The disciplined era holds 20 of 20 on gate 2 and was wrong here by
one byte, where the autonomous era rejects command lines that legacy
accepts. It does mean the disciplined era is not beyond checking, and that
every sub-command needs golden cases regardless of how it was verified
before.

Confirmed by direct comparison, with no harness involved:

```sh
docker run --rm -v $W:/w -w /w cdp8-postmerge housekeep copy 1 in.wav out.wav | wc -c   # 18
./target/debug/housekeep copy 1 in.wav out.wav | wc -c                                  # 19
```

## 6. Phase V: verification infrastructure (before any new port)

State on 2026-09-19: V1, V2, V3, V4 and V6 are built and merged. V5, the
nightly full-file comparison, waits on output-file comparison, which
section 6.1 describes.

What the gates found. Counts move as cases are added, so re-run them
rather than trusting this table:

| Gate | Scope | Result |
|---|---|---|
| 1, usage text | 21 sub-commands | 7 fail, all from the autonomous run |
| 2, argument grammar | 20 wired modes | all 20 agree with the record |
| 3, golden cases | 19 cases | 8 pass, 11 known deviations |

Gate 2's clean result is worth stating plainly. Every mode it can check
comes from the disciplined work before pull request 33. That work agrees
with legacy's recorded grammar at every position. The two eras differ in
kind, not in degree.

- [x] **V1. Gate 1, the usage-text conformance test.** Put it in
`crates/cdp-cli/tests/usage_conformance.rs`. Drive it from the dispatch
table. Effort: one agent-day.

- [x] **V2. Gate 2, the argument-grammar conformance test.** Put it in
`crates/cdp-programs/tests/grammar_conformance.rs`, not in `cdp-params`:
`cdp-programs` depends on `cdp-params`, so only the former can see both
the specs and the registry. Name each mode's `parstruct.json` process
symbol and mode key in the registry. Effort: two agent-days.

- [x] **V3. The oracle harness, `tools/oracle/`.** Build it as a workspace crate named
`cdp-oracle` so that Cargo builds and lints it with everything else. Give it
two modes:

- `record`: run a case against the legacy Docker image and write the
  expectation file.
- `verify`: run the same case against the Rust binary and compare.

Recording needs Docker. Verifying must not need Docker. Commit the
expectations so that the fast CI job replays them.

- [x] **V4. The case format**, `spec/golden/<program>/<subcommand>/<case>.toml`. Each
case records the argument vector, the input files, the expected exit code,
the expected standard output and standard error, and one entry per output
file.

For a sound output, record the format fields, the parsed `sfif` properties,
the `PEAK` values, the SHA-256 of the data chunk, and a fixed-size sample
fingerprint. The fingerprint holds the first 4096 samples, the last 4096
samples, and the root-mean-square value of each 4096-sample block. The
fingerprint keeps the repository small and still supports the tolerance rules
of PLAN.md decision D4.

Recommendation on inputs: generate small synthetic inputs where the algorithm
does not need real material. A 2000-sample file keeps the expectation file
small. Use the corpus in `docs/manual/sounds` only where real material
matters, and record the corpus path rather than a copy.

- [x] **V4a. The `known_deviation` marker.** A case whose output does not match
legacy yet carries one line saying why. `verify` reports the case and does
not fail the run, so legacy's real behavior can be recorded for a
defective command before anyone repairs it. This turns phase R from a
reading exercise into a list of failing cases to turn green.

The marker is tight in both directions, like gate 1's baseline: a case
that carries it and then starts matching fails the run. A marker cannot
outlive its defect.

All 7 defective sub-commands now have recorded cases. An agent doing phase
R runs `cargo run -p cdp-oracle -- verify` and sees exactly what legacy
does for each one.

- [ ] **V5. A nightly CI job that compares full output files inside Docker.** The
fingerprint catches almost every regression. The nightly job catches the
rest. Effort for V3 to V5 together: five agent-days.

- [x] **V6. Wire the gates into `.github/workflows/ci.yml`.** The placeholder
`golden-tests` job is gone. Two jobs replace it:

- The `rust` job now builds the binaries and replays every golden case.
  This needs no Docker and runs in seconds, so it gates every pull
  request.
- A `golden-drift` job re-records every case against the real legacy image
  and fails if a committed expectation moved. This catches an expectation
  written by hand rather than recorded, and it catches legacy changing
  under an upstream sync. It also fails outright when the harness is
  absent, which the old job did not.

Re-recording is deterministic, confirmed across all 17 cases: every
timestamp in a recorded output comes from the input file, not from the
run.

### 6.1 Output-file comparison, still to build

A case currently compares the exit code, standard output and standard
error. That is enough to catch every defect found so far, because five of
the seven reject their own command line. It is not enough for a command
whose output is a sound file.

The next increment adds one `[[outputs]]` entry per written file. For a
sound output, record the format fields, the parsed `sfif` properties, the
`PEAK` values, the SHA-256 of the data chunk, and a fixed-size
fingerprint: total sample count, the first and last 8 samples, the
minimum and maximum sample with their positions, and the
root-mean-square value of each of 64 equal blocks.

The hash answers whether the output is identical. The fingerprint supports
the tolerance rules of PLAN.md decision D4 when it is not, which matters
because the legacy build uses `-ffast-math`. Both stay small enough to
read in a diff, which a whole sample array would not.

## 7. Definition of done for one sub-command

This checklist replaces PLAN.md section 3. Every item is machine-checkable.
An agent reports a sub-command as done only when every box is ticked.

Copy this block into the pull request description and tick it there. A
pull request that does not carry it does not merge.

```markdown
### Definition of done: <program> <subcommand>

- [ ] 1. `spec/usage/<program>/<subcommand>.txt` exists, and gate 1 passes.
- [ ] 2. The registry names the `parstruct.json` process symbol and mode
         key, and gate 2 passes for every mode.
- [ ] 3. `spec/golden/<program>/<subcommand>/` holds, each recorded from
         the legacy image, never written by hand:
    - [ ] one success case per mode
    - [ ] one case with a stereo input, where the command takes sound input
    - [ ] one case per optional flag
    - [ ] one case with a breakpoint file, where a parameter accepts one
    - [ ] three error cases: a missing argument, an out-of-range value,
          and an unreadable input file
- [ ] 4. `cargo run -p cdp-oracle -- verify <program> <subcommand>` passes
         with no `known_deviation` marker left.
- [ ] 5. At least one in-repository regression test reads a real file and
         asserts a value that came from a legacy run, not from this
         crate's own output.
- [ ] 6. `cargo fmt --all -- --check`, `cargo clippy --workspace
         --all-targets -- -D warnings` and `cargo test --workspace` pass.
- [ ] 7. STATUS.md records the state, the scope limits, and every known
         deviation.
```

Item 3 is the one that the autonomous run skipped, and item 5 is the one
it faked. A test that asserts `"-0.1".parse::<f64>().unwrap() < 0.0`
satisfies neither.

## 8. Revised phase plan

Phase 0 and Phase 1 keep their PLAN.md definitions. Two changes apply.

WP-1.7, the golden harness, moves out of Phase 1 and becomes Phase V. It
gains a hard dependency: no program work package starts until Phase V ends.

WP-1.3's TOML specification loader stays deferred. Gate 2 supplies the
checking that the loader was going to supply. Revisit the loader only if
hand-written `CommandSpec` code becomes the bottleneck.

Remaining phase order:

- [x] 1. Phase V. Verification infrastructure. V5 remains, and it is not a
      blocker for Phase R.
- [ ] 2. Phase R. Remediation. About four agent-days.
- [ ] 3. Phase 2. Core time-domain and utility programs. Resume at WP-2.1
      and WP-2.2, which are both partly done. Then WP-2.3 to WP-2.14.
- [ ] 4. Phase 3. Spectral programs. Needs `cdp-spectral`, which WP-1.6
      must deliver first.
- [ ] 5. Phase 4. About 150 standalone programs. Highly parallel.
- [ ] 6. Phase 5 and Phase 6. Externals, packaging and legacy retirement.

Finish WP-2.1 and WP-2.2 before opening new programs. Both are close to
complete, and both hold the repaired commands from Phase R.

Remaining sub-commands in the two open work packages:

WP-2.1 `sndinfo`, 6 sub-commands and one mode range unstarted:

- [ ] `loudchan`, needs root-mean-square reporting
- [ ] `maxi`, needs root-mean-square reporting. Its process symbol is
      `INFO_LOUDLIST`, which is not a sub-command name.
- [ ] `diff`, needs two-file sample comparison
- [ ] `chandiff`, needs two-file sample comparison, stereo only
- [ ] `maxsamp2`
- [ ] `zcross`
- [ ] `units` modes 3 to 31, which need the note-name and interval-name
      grammars

WP-2.2 `housekeep`, 5 sub-commands and six modes unstarted:

- [ ] `respec` mode 1, needs cubic-spline resampling
- [ ] `extract` modes 1, 2, 3 and 6, need gate and envelope detection
- [ ] `extract` mode 5, which legacy reports as no longer available
- [ ] `gate`, needs gate and envelope detection
- [ ] `disk`, whose free-space figures are environment-dependent, so gate 3
      cannot compare them the way it compares everything else
- [ ] `batchexpand`, needs word-list input files
- [ ] `endclicks`
- [ ] `deglitch`

Order the remaining work by mechanism, not by apparent size. A sub-command
whose mechanism already exists costs a day. A sub-command that needs a new
mechanism costs a week. The missing mechanisms are:

| Mechanism | Blocks | Built |
|---|---|---|
| Output-file comparison in gate 3 | every command that writes a file, so every item below | [ ] |
| `SNDFILENAME` special data | `housekeep copy` mode 2, `housekeep remove` | [ ] |
| Word-list input files | `housekeep sort`, `housekeep batchexpand` | [ ] |
| Gate and envelope detection | `housekeep extract` modes 1, 2, 3, 6, `housekeep gate` | [ ] |
| Cubic-spline resampling | `housekeep respec` mode 1 | [ ] |
| Root-mean-square reporting | `sndinfo loudchan`, `maxi` | [ ] |
| Two-file sample comparison | `sndinfo diff`, `chandiff` | [ ] |

Output-file comparison comes first. Without it, gate 3 checks only the exit
code, standard output and standard error, so a command that writes a sound
file is not really checked. Section 6.1 specifies it. Do not start the
`housekeep bakup` or `housekeep sort` re-ports before it exists, because
both write files and both would otherwise be verified on their printed
output alone.

Build each mechanism once, in its own work package, before the sub-commands
that need it. Name the mechanism in the work package title.

## 9. Conventions for agents

These rules add to PLAN.md section 6. They exist because the autonomous run
broke each one. Read them as a checklist before opening a pull request.

- [ ] I copied the usage text from `spec/usage/` and did not write any.
- [ ] I read the `parstruct.json` entry and did not infer the grammar from
      the usage text.
- [ ] Every test I added compares against a value that came from a legacy
      run.
- [ ] I ran the legacy image and compared, rather than judging by my own
      output.
- [ ] I opened a pull request and did not commit to main.
- [ ] I updated STATUS.md in the same pull request as the code.
- [ ] If a mechanism was missing, I stopped and reported it rather than
      narrowing the command to fit.

The same rules in full:

Never write usage text. Copy it from `spec/usage/`. If the file is missing,
capture it from the legacy image first.

Never infer an argument grammar from the usage text alone. Read the
`parstruct.json` entry. The usage text and the grammar disagree in real
cases, and the grammar wins.

Never write a test that asserts the behavior of the Rust standard library.
The loop era shipped `assert!("-0.1".parse::<f64>().unwrap() < 0.0)` as the
only test for a sub-command. A test must compare against a value that came
from a legacy run.

Never report a command as done from its own output. Run the legacy image.
Compare. Then report.

Never commit to main. Open a pull request. Two loop-era commits went straight
to main, and one of them was silently overwritten later by a second
implementation of the same command.

Update STATUS.md in the same pull request as the code. A work package with no
STATUS.md entry is not done.

If a required mechanism is missing, stop and report it. Do not narrow the
command until it fits the mechanisms that exist. The loop era reduced
`housekeep sort` from six modes to three, renumbered the survivors, and
recorded none of this.

## 10. Autonomous operation

The loop era shows that autonomous runs amplify whatever the gates allow.
Weak gates produce fast, confident, wrong work.

Run agents autonomously only after Phase V ends. With the gates in place, an
autonomous agent cannot merge a fabricated port, because Gate 1 and Gate 2
fail without Docker and without any judgment call.

Set the per-iteration instruction to one sub-command, not one work package.
Require the section 7 checklist in the pull request description. A pull
request that does not list the checklist items does not merge.

## 11. Effort

| Phase | Work packages | Agent-days | State |
|---|---|---|---|
| V. Verification infrastructure | 6 | 8 | 5 of 6 done, V5 open |
| R. Remediation | 4 | 4 | R1 and R4 done, 0 of 9 commands repaired |
| 2. Core programs, remaining | 14 | 55 | WP-2.1 and WP-2.2 open |
| 3. Spectral programs | 15 | 45 | not started, blocked on WP-1.6 |
| 4. Standalone programs | about 150 | 300 | not started |
| 5. Externals | 3 | 15 | not started |
| 6. Release | 4 | 15 | not started |
| Total | | about 442 | |

Phase V and Phase R together cost 12 agent-days. They run before everything
else, and they run sequentially. That cost buys the ability to run Phase 4
with many parallel agents and trust the result.

## 12. Risks

| Risk | Mitigation |
|---|---|
| More unchecked commands exist than section 3 found | Gate 1 and Gate 2 run against every dispatched command at once, so Phase V measures the true count on its first run |
| Golden expectations make the repository large | Fingerprints instead of whole files, synthetic inputs where possible, corpus files by path |
| The legacy image stops building | Keep the image published, and record the synced upstream commit, per PLAN.md decision D9 |
| `parstruct.json` is wrong for some process | Gate 1 and Gate 3 are independent of it, so a disagreement between gates exposes the error |
| Usage text and grammar disagree | Record the disagreement in STATUS.md and follow the grammar, because the grammar is what the parser runs |
| Gate 2 cannot describe hand-written argument handling | Four commands already bypass `cdp_params::parse` for real reasons. Mark them exempt in one list, and require an extra golden case for each |

## 13. Quick start for an agent picking this up

1. Read section 5 (Phase R) and the dashboard at the top.
2. Run the three gate commands. Their output is the current state, and this
   document is only a map of it.
3. Pick one item from the R2 work list, or one unstarted sub-command from
   section 8.
4. Copy the section 7 checklist into your pull request and work through it.
5. Before you finish, make sure that no `known_deviation` marker and no
   `KNOWN_FAILURES` entry remains for what you repaired. Both fail the
   build if they outlive the defect, so the build will tell you.

What to read for one sub-command, in this order: its captured usage text,
its `parstruct.json` entry, its program's `ap_*.c` dispatch, then the
algorithm. Appendix B lists the paths.

## Appendix A: how to reproduce section 3

```sh
# Build the Rust binaries.
cargo build --workspace

# Prepare a scratch directory with one corpus file.
W=$(mktemp -d); cp docs/manual/sounds/marimba.wav "$W/a.wav"
cp "$W/a.wav" "$W/b.wav"; printf 'a.wav\n' > "$W/list.txt"

# Run each command against legacy, then against Rust.
docker run --rm -v "$W:/w" cdp8-postmerge bash -c 'cd /w && sndinfo findhole a.wav'
./target/debug/sndinfo findhole "$W/a.wav"

docker run --rm -v "$W:/w" cdp8-postmerge bash -c 'cd /w && housekeep bakup a.wav b.wav out.wav'
./target/debug/housekeep bakup "$W/a.wav" "$W/b.wav" "$W/out2.wav"

docker run --rm -v "$W:/w" cdp8-postmerge bash -c 'cd /w && housekeep sort 1 list.txt'
(cd "$W" && /path/to/target/debug/housekeep sort 1 list.txt)
```

## Appendix B: the specification files an agent must read

| File | Holds |
|---|---|
| `spec/usage/<program>/<sub>.txt` | the exact usage text, captured from a real run |
| `spec/commands/_raw/parstruct.json` | parameter counts, type letters, option flags, variant flags, mode names |
| `docs/legacy/dev/<dir>/ap_*.c` | process dispatch, file kinds, buffer setup |
| `docs/legacy/dev/<dir>/*.c` | the algorithm |
| `docs/legacy/dev/include/*.h` | constants such as `BAKUP_GAP` |
| `docs/legacy/dev/cdp2k/parstruct.c` | the source of `parstruct.json` |
| `docs/legacy/dev/cdp2k/tklib1.c` | parameter ranges and defaults |
