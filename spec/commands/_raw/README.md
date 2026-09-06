# Raw extracted parameter tables

Files in this directory are extracted mechanically by
`tools/extract-specs/parstruct.py` from
`legacy/dev/cdp2k/parstruct.c`. They are a starting point, not a
finished specification. See `docs/migration/PLAN.md`, section 3, for
how a porting work package turns one of these entries into a
`spec/commands/<program>/<subcommand>.toml` file.

## `parstruct.json`

The top level key is a process symbol, from
`legacy/dev/include/processno.h`. The second level key is a mode
symbol, from `legacy/dev/include/modeno.h`. If a process does not
switch on mode for a given table, the second-level key is the literal
string `"null"` instead.

Each mode entry can carry a `param` key, a `vflags` key, or both:

- `param` comes from `set_legal_param_structure` (`set_param_data` calls):
  `special_data`, `max_param_cnt`, `param_cnt`, and `param_list`. The
  list has one type character per parameter. Two files explain the
  meaning of each character: the column-header comments above
  `set_legal_param_structure` in `legacy/dev/cdp2k/parstruct.c`, and
  the parsing code in `legacy/dev/cdp2k/readdata.c`.
- `vflags` comes from `set_legal_option_and_variant_structure`
  (`set_vflgs` calls): `opt_flags`/`opt_cnt`/`opt_list` for command-line
  options, `var_flags`/`vflag_cnt`/`vparam_cnt`/`var_list` for mode
  variants.

**A mode entry missing `param` or `vflags` is usually not an error.**
The two source functions switch on mode independently. One process can
have per-mode parameters but one shared option and variant set for
every mode. Or the reverse can hold. When that happens, the shared
side's data sits under the `"null"` mode key. It does not sit under
each mode symbol. Always check the `"null"` entry for a process first.
Do this before you treat data as missing. `parstruct.py` prints a
count of param-only, vflags-only, and both-present entries as a sanity
check. That count stays in the same range on every run, because the
source file's shape is stable. A param-only or vflags-only count of
zero is not the goal.

## What this does not give you

- **The CLI name.** `MOD_LOUDNESS` does not tell you the command is
  `modify loudness`. That mapping lives in each group's `ap_*.c`
  dispatch (`get_process_no`, `get_the_mode_from_cmdline`). Cross-check
  it against the group's usage text in `spec/usage/<program>/`, which
  gives the human-readable sub-command name. Do this by hand.
- **Ranges and defaults.** Those are set per-process in each
  `setup_*_param_ranges_and_defaults` function, not in `parstruct.c`.
- **Parameter names.** Human-readable names (`DECAY_RATE`,
  `GLISS_RATE`, ...) come from `legacy/dev/cdparams/parnames.c`, keyed
  the same way. A future work package can extract these alongside this
  table. No tool does that extraction yet.
- **Standalone programs.** `parstruct.c` only covers the group
  programs that share the `cdp2k` framework tables. Standalone programs
  (`dev/standalone`, `dev/standnew`, `dev/science`, `dev/new`) define
  their own parameter setup per file. No central table exists for them
  to extract.
