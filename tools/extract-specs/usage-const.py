#!/usr/bin/env python3
"""Emit or check the Rust USAGE constant for a ported sub-command.

Gate 1 of docs/migration/PLAN-V2.md requires that a sub-command print
the usage text captured from a real legacy run. Six sub-commands reached
the main branch with invented text. This tool removes the need to type
that text by hand.

The rule it applies is verified against all 14 conforming sub-commands:

    USAGE const == spec/usage/<program>/<sub>.txt minus its last 2 bytes

The legacy exit path (cdp_core::report_and_exit, mirroring
print_messages_and_close_sndfiles in legacy/dev/cdp2k/mainfuncs.c)
prints its own trailing blank line, which accounts for those 2 bytes.

Usage:
    usage-const.py emit  <program> <subcommand>   # print the Rust const
    usage-const.py check                          # check every module
"""

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TAIL = 2  # newlines that report_and_exit adds

def captured(program, sub):
    path = os.path.join(ROOT, "spec", "usage", program, f"{sub}.txt")
    if not os.path.exists(path):
        sys.exit(f"no captured usage text at {path}\n"
                 f"Capture it from the legacy image first:\n"
                 f"  docker run --rm cdp8-postmerge {program} {sub} > {path}")
    with open(path) as handle:
        return handle.read()

def const_text(program, sub):
    text = captured(program, sub)
    if not text.endswith("\n" * TAIL):
        sys.exit(f"{program}/{sub}: captured text does not end in {TAIL} newlines. "
                 f"Re-capture it, or handle this sub-command by hand.")
    return text[:-TAIL]

def emit(program, sub):
    body = const_text(program, sub)
    # A raw string keeps backslashes literal. Widen the hashes if the
    # text itself contains the closing delimiter.
    hashes = ""
    while f'"{hashes}' in body:
        hashes += "#"
    print(f'pub const USAGE: &str = r{hashes}"{body}"{hashes};')

def check():
    bad = []
    checked = 0
    src = os.path.join(ROOT, "crates", "cdp-programs", "src")
    for program in sorted(os.listdir(src)):
        pdir = os.path.join(src, program)
        if not os.path.isdir(pdir):
            continue
        for name in sorted(os.listdir(pdir)):
            if not name.endswith(".rs") or name == "mod.rs":
                continue
            sub = name[:-3]
            spec = os.path.join(ROOT, "spec", "usage", program, f"{sub}.txt")
            if not os.path.exists(spec):
                continue
            with open(os.path.join(pdir, name)) as handle:
                module = handle.read()
            if "pub const USAGE" not in module:
                continue
            checked += 1
            want = const_text(program, sub)
            if want not in module:
                bad.append(f"{program}/{sub}")
    print(f"checked {checked} module(s)")
    if bad:
        print(f"{len(bad)} do not contain the captured usage text verbatim:")
        for name in bad:
            program, sub = name.split("/")
            print(f"  {name}  -> fix with: "
                  f"tools/extract-specs/usage-const.py emit {program} {sub}")
        sys.exit(1)
    print("every module matches its captured usage text")

if __name__ == "__main__":
    if len(sys.argv) == 4 and sys.argv[1] == "emit":
        emit(sys.argv[2], sys.argv[3])
    elif len(sys.argv) == 2 and sys.argv[1] == "check":
        check()
    else:
        sys.exit(__doc__)
