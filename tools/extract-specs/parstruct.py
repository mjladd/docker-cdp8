#!/usr/bin/env python3
"""Extracts the parameter and option/variant tables from
legacy/dev/cdp2k/parstruct.c into structured JSON.

This is the machine-readable core of every group program's command-line
interface: for each (process, mode) pair, set_legal_param_structure gives
the parameter count and per-parameter type string (passed to
set_param_data), and set_legal_option_and_variant_structure gives the
option and variant flags (passed to set_vflgs). See
dev/cdp2k/readdata.c and dev/cdp2k/tklib3.c for how these type strings are
interpreted at parse time, and dev/cdparams/parnames.c for the matching
human-readable parameter names.

This script does NOT map process/mode symbols to the CLI names a user
types (e.g. MOD_LOUDNESS -> "modify loudness"). That mapping lives in
each group's ap_*.c dispatch table (get_process_no, get_the_mode_from_cmdline)
and is cross-referenced by hand in each program's porting work package
(see docs/migration/PLAN.md section 3, step 2). Use spec/usage/<program>/
alongside this file's output to do that.

Output: spec/commands/_raw/parstruct.json, a dict:
  {
    "<PROCESS_SYMBOL>": {
      "modes": {
        "<MODE_SYMBOL_or_null>": {
          "param": {"special_data": int, "max_param_cnt": int,
                     "param_cnt": int, "param_list": str},
          "vflags": {"opt_flags": str, "opt_cnt": int, "opt_list": str,
                      "var_flags": str, "vflag_cnt": int,
                      "vparam_cnt": int, "var_list": str}
        }, ...
      }
    }, ...
  }

Usage:
  tools/extract-specs/parstruct.py [path/to/parstruct.c] [output.json]
Defaults: legacy/dev/cdp2k/parstruct.c, spec/commands/_raw/parstruct.json
"""
import json
import re
import sys
from pathlib import Path

CASE_RE = re.compile(r"case\s*\(\s*([A-Za-z_][A-Za-z0-9_]*)\s*\)\s*:")
DEFAULT_RE = re.compile(r"\bdefault\s*:")
SWITCH_MODE_RE = re.compile(r"switch\s*\(\s*mode\s*\)")
RETURN_CALL_RE = re.compile(
    r"return\s+(set_param_data|set_vflgs)\s*\(\s*ap\s*,(.*)\)\s*;\s*$"
)


def strip_comments(text: str) -> str:
    text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
    text = re.sub(r"//[^\n]*", "", text)
    return text


def split_args(s: str):
    """Splits a C argument list on top-level commas (no nested parens or
    string commas appear in these calls, so a simple depth counter is
    enough)."""
    args, depth, cur = [], 0, ""
    for ch in s:
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        if ch == "," and depth == 0:
            args.append(cur.strip())
            cur = ""
        else:
            cur += ch
    if cur.strip():
        args.append(cur.strip())
    return args


def unquote(s: str) -> str:
    s = s.strip()
    if s.startswith('"') and s.endswith('"'):
        return s[1:-1]
    return s


def parse_switch_process(body: str, kind: str):
    """kind is 'param' or 'vflags'. Returns list of
    {process, mode, fields}."""
    results = []
    depth = 0  # 0 = inside switch(process), 1 = inside nested switch(mode)
    pending_process_cases = []
    pending_mode_cases = []

    for raw_line in body.split("\n"):
        line = raw_line.strip()
        if not line:
            continue

        # A line can carry one or more case labels, and/or a return
        # statement, and/or the switch(mode) opener. Peel them off in
        # the order they can appear.
        working = line

        cases_here = CASE_RE.findall(working)
        has_default = bool(DEFAULT_RE.search(working))
        if cases_here or has_default:
            target = pending_process_cases if depth == 0 else pending_mode_cases
            target.extend(cases_here)
            if has_default:
                target.append("DEFAULT")
            working = CASE_RE.sub("", working)
            working = DEFAULT_RE.sub("", working)

        if SWITCH_MODE_RE.search(working):
            depth = 1
            continue

        m = RETURN_CALL_RE.search(working)
        if m:
            fn, argstr = m.group(1), m.group(2)
            args = [unquote(a) for a in split_args(argstr)]
            if kind == "param" and fn == "set_param_data":
                fields = {
                    "special_data": args[0],
                    "max_param_cnt": args[1],
                    "param_cnt": args[2],
                    "param_list": args[3] if len(args) > 3 else "",
                }
            elif kind == "vflags" and fn == "set_vflgs":
                fields = {
                    "opt_flags": args[0],
                    "opt_cnt": args[1],
                    "opt_list": args[2],
                    "var_flags": args[3],
                    "vflag_cnt": args[4],
                    "vparam_cnt": args[5],
                    "var_list": args[6] if len(args) > 6 else "",
                }
            else:
                continue

            if depth == 0:
                for p in pending_process_cases:
                    results.append({"process": p, "mode": None, "fields": fields})
                pending_process_cases = []
            else:
                for p in pending_process_cases:
                    for md in pending_mode_cases:
                        results.append({"process": p, "mode": md, "fields": fields})
                pending_mode_cases = []
            continue

        if working == "break;" or working.startswith("break;"):
            if depth == 1:
                depth = 0
                pending_mode_cases = []
                pending_process_cases = []
            continue

        # Anything else (bare '{', '}', blank after stripping, stray
        # comments already removed) is structural noise we can ignore
        # given this file's consistent style.

    return results


def extract_function_body(text: str, signature_re: str) -> str:
    m = re.search(signature_re, text)
    if not m:
        raise SystemExit(f"could not find function matching {signature_re!r}")
    start = text.index("{", m.end())
    depth = 0
    i = start
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return text[start + 1 : i]
        i += 1
    raise SystemExit("unbalanced braces")


def main():
    src = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(
        "legacy/dev/cdp2k/parstruct.c"
    )
    out = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(
        "spec/commands/_raw/parstruct.json"
    )

    text = strip_comments(src.read_text())

    param_body = extract_function_body(
        text, r"int\s+set_legal_param_structure\s*\("
    )
    vflag_body = extract_function_body(
        text, r"int\s+set_legal_option_and_variant_structure\s*\("
    )

    param_rows = parse_switch_process(param_body, "param")
    vflag_rows = parse_switch_process(vflag_body, "vflags")

    merged = {}
    for row in param_rows:
        proc = merged.setdefault(row["process"], {"modes": {}})
        mode_key = row["mode"] or "null"
        merged[row["process"]]["modes"].setdefault(mode_key, {})["param"] = row[
            "fields"
        ]
    for row in vflag_rows:
        proc = merged.setdefault(row["process"], {"modes": {}})
        mode_key = row["mode"] or "null"
        merged[row["process"]]["modes"].setdefault(mode_key, {})["vflags"] = row[
            "fields"
        ]

    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(merged, indent=2, sort_keys=True) + "\n")

    n_pairs = sum(len(v["modes"]) for v in merged.values())
    print(
        f"{len(merged)} processes, {n_pairs} (process, mode) entries "
        f"-> {out}",
        file=sys.stderr,
    )
    both = sum(
        1
        for v in merged.values()
        for m in v["modes"].values()
        if "param" in m and "vflags" in m
    )
    only_param = sum(
        1
        for v in merged.values()
        for m in v["modes"].values()
        if "param" in m and "vflags" not in m
    )
    only_vflags = sum(
        1
        for v in merged.values()
        for m in v["modes"].values()
        if "vflags" in m and "param" not in m
    )
    print(
        f"  {both} entries have both param+vflags data, "
        f"{only_param} param-only, {only_vflags} vflags-only "
        "(the last two usually mean a case falls through to a shared "
        "'default' entry in the other table -- check by hand)",
        file=sys.stderr,
    )


if __name__ == "__main__":
    main()
