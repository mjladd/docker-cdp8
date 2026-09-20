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

## What a case does not yet compare

Output files. A case records the exit code, standard output and standard
error. A command that writes a sound file or a text file is compared only
on those three. Comparing output files is the next increment of gate 3,
and PLAN-V2 section 6 item V4 describes the fingerprint it will use.
