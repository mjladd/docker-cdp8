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

## `sndinfo timediff` with one infile segfaults instead of reporting an error

**Found while porting:** WP-2.1, `cdp-programs` `sndinfo timediff`
(`crates/cdp-programs/src/sndinfo/timediff.rs`).

**Where:** `legacy/dev/cdp2k/mainfuncs.c`, `handle_extra_infiles`.

**The bug:** `INFO_TIMEDIFF` is a `TWO_SNDFILES` process.
`count_and_allocate_for_infiles` sets `dz->infilecnt = 2` for it no
matter how many file names the user actually typed. `handle_extra_
infiles` then reads the second file name like this:

```c
if(dz->infilecnt > 1) {
    for(n=1;n<dz->infilecnt;n++) {
        filename = (*cmdline)[0];
        switch(dz->process) {
        ...
        default:
            if((exit_status = handle_other_infile(n,filename,dz))<0)
                return(exit_status);
            break;
        }
        (*cmdline)++;
        (*cmdlinecnt)--;
    }
```

The loop reads `(*cmdline)[0]` without a bounds check against
`*cmdlinecnt`. With only one infile on the command line, `*cmdline`
already points past the end of `argv`. The read is out of bounds.

**Reproducer:** `sndinfo timediff infile.wav` (one infile, not two).
A live run shows the crash. The process exits with signal 11
(SIGSEGV, exit code 139). It prints nothing at all, not even the
usual `argc<4` greeting line every other `sndinfo` sub-command prints
first.

**Ported behavior:** `run_sndinfo_timediff` in
`crates/cdp-cli/src/lib.rs` counts the command-line tokens before it
opens any file. With exactly one token, it returns
`cdp_params::ParamsError::InsufficientCmdlineParameters`
("Insufficient cmdline parameters."). This is the same clean error
text every other too-few-infiles case in this crate already uses.
Real legacy prints no text at all for this case, so this wording is
this crate's own choice, not a captured legacy string.

**Check performed:** a direct reading of `handle_extra_infiles` shows
the missing bounds check. Rust's array indexing cannot reproduce this
bug. An out-of-bounds slice read panics. It does not read adjacent
memory the way the C code does. `run_sndinfo_timediff`'s own
argument-count check also runs first, so this class of failure never
reaches the unsafe read at all.

## `housekeep chans 3` (zero one channel, mono infile) corrupts the first sample of every internal buffer

**Found while porting:** WP-2.2, `cdp-programs` `housekeep chans`
(`crates/cdp-programs/src/housekeep/chans.rs`).

**Where:** `legacy/dev/houskeep/channels.c`, `do_channels`,
`case(HOUSE_ZCHANNEL)`, the `case(MONO)` branch (line 310).

**The bug:** for a mono infile, this mode expands each sample into a
stereo pair, one side zero and the other side the original sample,
in place in the same buffer (so the buffer must grow from `N` floats
to `2N` without a second buffer to copy into):

```c
for(n=dz->ssampsread-MONO,m=(dz->ssampsread*STEREO)-STEREO;n>=0;n--,m-=2) {
    buf[m+zeroed]     = (float)0;
    buf[m+not_zeroed] = buf[n];
}
```

`zeroed`/`not_zeroed` are `0`/`1` or `1`/`0` depending on which side
`channo` names. The loop runs backward (`n`/`m` decreasing) so that
each pair is written only after its own source sample has already
been consumed by an earlier iteration -- except at the very last one,
`n==0, m==0`. When `zeroed==0` (`channo` names the left channel, the
only value a mono infile's own range check allows: `ap->hi[CHAN_NO]`
is the infile's channel count, `1`, for this mode too -- see
`cdp_params::CommandSpec::housekeep_chans_channel`'s doc for the same
formula on a different mode), that final iteration writes `buf[0] =
0` *before* reading `buf[n] == buf[0]` into `buf[1]`. The zero write
clobbers the very sample the next statement still needs, so `buf[1]`
(the real channel) receives `0` instead of the original sample.

This is not a one-off edge case: `do_channels` is called once per
internal read buffer (see `docs/migration/STATUS.md`'s WP-2.2 notes on
`display_virtual_time` for the same buffer mechanism, sized from
`Malloc(-1)`, the largest available block of free memory), and this
loop runs, and hits its own `n==0` case, once per buffer. A file
short enough to fit in one buffer loses exactly its first sample; a
larger file loses the first sample of every internal chunk, at
buffer-size-dependent (and so not reproducible in general) positions.

**Reproducer:** a synthetic 2000-sample mono WAV (well under any
plausible buffer size), `housekeep chans 3 short_mono.wav out.wav 1`.
The real channel's first sample is `0` in the output; every other
sample matches the infile exactly. `docs/manual/sounds/marimba.wav`
(44174 samples) reproduces the same corruption three times, at
samples `0`, `16384`, and `32768` -- consistent with, but not
confirmed to be exactly, a 16384-sample buffer in this environment.

**Ported behavior:** `crates/cdp-programs/src/housekeep/chans.rs`'s
`zero_channel` builds the expanded stereo buffer as a fresh `Vec`
(reading every mono sample into its own untouched slot), so there is
no in-place aliasing and no first-sample corruption at all, for a
mono infile of any size. The stereo-infile branch of the same mode
(a simple forward loop zeroing one interleaved channel, no in-place
expansion) has no equivalent bug, confirmed live against
`docs/manual/sounds/clashmixtest.wav` with zero byte differences.

**Check performed:** a direct reading of the loop above, and live
confirmation with both the 2000-sample synthetic file (single-buffer,
one corrupted sample at index `0`) and `marimba.wav` (multi-buffer,
three corrupted samples at buffer-sized intervals). This case is
`known-deviation`: `crates/cdp-programs/src/housekeep/chans.rs`'s own
regression tests mask the confirmed-corrupted index rather than
asserting exact equality with a live `legacy` run for the mono
branch, since the corruption's positions for a file spanning more
than one buffer are not reproducible in general (see this crate's own
`print_virtual_time` doc for the same non-determinism affecting a
different piece of legacy output).
