# Port Status

Agents update this file at the end of every work package.

## Infrastructure work packages

| WP | State | Notes |
|---|---|---|
| 0.1 Sync with upstream and restructure | done | Merged upstream `ComposersDesktop/CDP8@28bc42c` (15 commits: same Linux/GCC fixes our patch script applied, `specfnu` memory-allocation fixes, `aaio` relicensed to LGPL 2.1). Removed `docker-cdp8/fix-linux-compat.sh`. Moved `dev/`, `include/`, `cmake/`, `CMakeLists.txt`, `building.txt` to `legacy/`. Removed the stale `dev/sfsys` (an unbuilt duplicate of `newsfsys`). Also removed the top-level `include/` copies of `sfsys.h`, `props.h`, `osbind.h`, `chanmask.h` and `cdplib.h`, since `dev/newinclude/` already shadowed them. Moved the one non-duplicate header, `aaio.h`, into `dev/newinclude/`. Fixed `CPACK_RESOURCE_FILE_LICENSE` (broke `cmake` configure after the move). Updated Dockerfile/README build paths to `cmake ../legacy` and `legacy/NewRelease`. Verified in Docker: same 220 executables, same 3 warnings, 0 errors, core commands (`synth`, `sndinfo`, `pvoc`, `modify`) run and match pre-move output. |
| 0.2 Capture the specification | in progress | `tools/extract-specs/dump-usage.sh` captured usage text for all 220 programs and 368 sub-commands into `spec/usage/` (one empty file, `newtex.txt`, from a pre-existing stdin-hang bug). `tools/extract-specs/parstruct.py` extracted `legacy/dev/cdp2k/parstruct.c` into `spec/commands/_raw/parstruct.json` (299 processes, 645 process/mode entries), validated by hand against five processes. Remaining: extract `legacy/dev/cdparams/parnames.c` (human-readable parameter names) and draft the per-program `spec/commands/<program>/<subcommand>.toml` files by cross-referencing this raw data against each group's `ap_*.c` dispatch table. Standalone programs (`dev/standalone`, `dev/standnew`, `dev/science`, `dev/new`) have no central table and are drafted by hand per program. |
| 0.3 Continuous integration | not started | fmt/clippy/test workflow plus the legacy Docker build and golden suite. |

## Programs
