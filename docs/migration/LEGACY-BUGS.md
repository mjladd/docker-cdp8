# Legacy bugs found during the port

`docs/migration/PLAN.md` section 6 sets the rule. An agent that finds
a bug in the C code writes it here. The agent adds a reproducer. The
agent ports the intended behavior instead of the bug. Once a golden
case exists for the affected code, the agent marks that case
`known-deviation`.

## Mix files: a 4-word line with an invalid `chans` value leaves data uninitialized

**Found while porting:** WP-1.2, `cdp-data` mix file parsing
(`crates/cdp-data/src/mix.rs`).

**Where:** `legacy/dev/submix/setupmix.c`,
`finalise_and_check_mixdata_in_line`.

**The bug:** a 4-word mix file line has this form: `sndname time
chans level`. It has no explicit pan field. When `chans` is `1` or
`2`, the function sets `lpan`, `rlevel`, and `rpan` for this line
form:

```c
case(MIX_MINLINE):
    switch(chans) {
    case(1):
        *lpan = 0.0;
        break;
    case(2):
        *rlevel = llevel;
        *lpan   = -1.0;
        *rpan   = 1.0;
    }
    break;
```

The switch statement has no `default` case. Take a `chans` value of
`3`, for example. The switch then leaves `lpan`, `rlevel`, and `rpan`
unset. `get_mixdata_in_line` does not set these three fields for a
4-word line either. So each variable holds whatever value was already
on the stack. `setupmix.c` does not reject this mixfile line on its
own. The code can add an event with a meaningless pan and level. A
later, unrelated check is the only thing that can still catch the
problem. For example, `open_file_and_get_props` checks the channel
count against the real sound file. On a typical mono or stereo
corpus, this later check happens to catch bad input by chance, not by
design.

**Reproducer:** a mix file line `name.wav 0.0 3 0.5`. This is a
4-word line that declares 3 channels.

**Ported behavior:** `MixEvent::parse_line` in
`crates/cdp-data/src/mix.rs` checks a 4-word line's `chans` value.
When that value is not `1` or `2`, it reports a data error
(`DataError::MinLineChansMustBeMonoOrStereo`) instead of an event
with undefined fields.

**Check performed:** a direct reading of
`finalise_and_check_mixdata_in_line` shows the bug. Its
`switch(chans)` block has no `default` arm. No other function in the
same path sets `lpan`, `rlevel`, or `rpan` for this line length
either.

A live run of the real binary did not reproduce this bug. The module
doc in `crates/cdp-data/src/mix.rs` explains why. `legacy` `submix
mix` runs a file-type auto-detection step first. In the normal CLI
path, that step rejects a malformed mixfile before `setupmix.c` runs
at all.
