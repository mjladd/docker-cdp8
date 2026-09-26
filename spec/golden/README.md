# Golden cases

Gate 3 of [../../docs/migration/PLAN-V2.md](../../docs/migration/PLAN-V2.md).

One case is one TOML file at
`spec/golden/<program>/<subcommand>/<case>.toml`. It holds what to run,
and what the legacy program did when it ran.

## Add a case

1. Write the case file. Give it a `description`, the `program` and
   `subcommand`, the `argv` as typed after the launcher name, and one
   `[[inputs]]` block per input file.
2. Record it against the legacy image:

   ```sh
   cargo run -p cdp-oracle -- record spec/golden/sndinfo/props/my-case.toml
   ```

3. Replay it against the Rust build:

   ```sh
   cargo run -p cdp-oracle -- verify sndinfo props
   ```

Recording needs Docker and the legacy image. Replaying needs neither, so
continuous integration replays every case on every pull request.

## Rules

Never write an `[expected]` block by hand. Record it. A hand-written
expectation fails the `golden-drift` job, which re-records every case
against the real legacy image and compares.

Every path in `from` is relative to the repository root. The file is
copied into a sandbox under the name in `name`, and the command runs with
the sandbox as its working directory. So `argv` never carries an absolute
path, and a case stays portable.

Prefer a small input. A case that needs real material can name a corpus
file by path, which costs nothing, because the case stores the path
rather than a copy.

## Progress output is normalised

A recorded stream holds what a terminal would show: for each line, only
the text after the last carriage return.

legacy reports progress with `display_virtual_time`, which prints
`"\r%d min %5.2lf sec"` once per internal write buffer and no newline, so
each tick overwrites the one before it. The number of ticks is not
reproducible, because the buffer size comes from a request for the largest
free block of memory at allocation time, which depends on the machine.

This was found the hard way. Two `housekeep copy` cases recorded on a
workstation drifted when the `golden-drift` job re-recorded them on a
continuous-integration runner. Recording the last tick only is both
portable and faithful to what a user sees.

## Known deviations

A case whose Rust output does not match legacy yet carries a
`known_deviation` line that says why. `verify` reports such a case and
does not fail the run.

The marker is tight in both directions. A case that carries it and then
starts matching fails the run, so the marker cannot outlive the defect.
Remove the marker in the same change that fixes the defect.

## Output files

A case records the files a run left in the sandbox, so you never list them
yourself. The harness snapshots the sandbox before and after the run and
records what was created, changed or deleted. Auto-named outputs come for
free, which matters because `housekeep chans` writes `marimba_c1.wav` and
`housekeep copy` mode 2 writes `in_001.wav`. A command that writes a file it
must not write fails the case.

A sound output records its header fields, its property names, its `PEAK`
values and positions, a SHA-256 of the decoded samples, and a fixed-size
fingerprint: the first and last 8 samples, the extremes with their
positions, and the root-mean-square value of each of 64 equal blocks.

Two fields are never recorded, because both hold the time of the run: the
`DATE` property and the `PEAK` chunk timestamp. Property names are recorded,
because presence is deterministic and meaningful.

## Samples must match exactly

A case fails if an output's samples differ at all. To accept a difference,
set both fields:

```toml
sample_tolerance = 1e-4
tolerance_reason = "why this case cannot match exactly"
```

A tolerance with no reason is refused. This is PLAN.md section 5's rule, and
there is a concrete reason for it: one bit of a 16-bit sample is about
3.05e-5, so PLAN.md decision D4's 1e-4 permits roughly three bits of error
and would hide a quantisation bug. One such bug was real, in
`cdp_sf::writer::encode_pcm16`. See PLAN-V2 section 6.1.

## After editing a case by hand

Run `record` once. The harness writes fields in a fixed order, so a
hand-edited file needs one pass to settle. Without it the `golden-drift`
job reports a change that is only field order.
