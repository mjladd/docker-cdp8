# Port Status

Agents update this file at the end of every work package.

## Infrastructure work packages

| WP | State | Notes |
|---|---|---|
| 0.1 Sync with upstream and restructure | done | Merged upstream `ComposersDesktop/CDP8@28bc42c` (15 commits: same Linux/GCC fixes our patch script applied, `specfnu` memory-allocation fixes, `aaio` relicensed to LGPL 2.1). Removed `docker-cdp8/fix-linux-compat.sh`. Moved `dev/`, `include/`, `cmake/`, `CMakeLists.txt`, `building.txt` to `legacy/`. Removed the stale `dev/sfsys` (an unbuilt duplicate of `newsfsys`). Also removed the top-level `include/` copies of `sfsys.h`, `props.h`, `osbind.h`, `chanmask.h` and `cdplib.h`, since `dev/newinclude/` already shadowed them. Moved the one non-duplicate header, `aaio.h`, into `dev/newinclude/`. Fixed `CPACK_RESOURCE_FILE_LICENSE` (broke `cmake` configure after the move). Updated Dockerfile/README build paths to `cmake ../legacy` and `legacy/NewRelease`. Verified in Docker: same 220 executables, same 3 warnings, 0 errors, core commands (`synth`, `sndinfo`, `pvoc`, `modify`) run and match pre-move output. |
| 0.2 Capture the specification | not started | Usage-text dump and `spec/commands/*.toml` drafts from `parstruct.c`/`validate.c`/`ap_*.c`. |
| 0.3 Continuous integration | not started | fmt/clippy/test workflow plus the legacy Docker build and golden suite. |

## Programs
