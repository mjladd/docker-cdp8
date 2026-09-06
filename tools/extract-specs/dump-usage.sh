#!/usr/bin/env bash
# Captures the usage text of every legacy CDP8 program and, for group
# programs, every sub-command, into spec/usage/. This is the frozen
# behavioural reference used by every porting work package (see
# docs/migration/PLAN.md, WP-0.2).
#
# Usage:
#   tools/extract-specs/dump-usage.sh [docker-image-tag]
#
# If no image tag is given, builds one from legacy/ via
# docker-cdp8/Dockerfile, tagged cdp8-spec-capture.
#
# Requires: docker.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
IMAGE="${1:-cdp8-spec-capture}"
OUT_DIR="$REPO_ROOT/spec/usage"
TIMEOUT_SECS=5

if [ "$#" -eq 0 ]; then
    echo "Building legacy image ($IMAGE)..." >&2
    docker build -f "$REPO_ROOT/docker-cdp8/Dockerfile" -t "$IMAGE" "$REPO_ROOT" >&2
fi

mkdir -p "$OUT_DIR"

# Get the full list of built programs.
mapfile -t PROGRAMS < <(docker run --rm "$IMAGE" ls /opt/cdp/bin | sort)
echo "Found ${#PROGRAMS[@]} programs." >&2

for prog in "${PROGRAMS[@]}"; do
    top_out="$OUT_DIR/${prog}.txt"
    docker run --rm "$IMAGE" bash -c "timeout ${TIMEOUT_SECS}s $prog < /dev/null 2>&1" > "$top_out" || true

    # If this looks like a group program ("where NAME can be ..."), capture
    # each sub-command's own usage text too. Every group program ends its
    # menu with a "Type '<prog> <sub>' for more info" line, which is a more
    # reliable stop marker than blank-line counting: some menus (e.g.
    # distort) have an uppercase section header with its own blank line
    # before the actual list.
    if grep -q "where NAME can be" "$top_out"; then
        subs=$(awk '/where NAME can be/{p=1;next} /Type .*for more info/{exit} p{print}' "$top_out" \
               | tr -s ' \t' '\n' | grep -E '^[a-z][a-z0-9_]*$' | sort -u || true)
        if [ -n "$subs" ]; then
            mkdir -p "$OUT_DIR/$prog"
            while IFS= read -r sub; do
                [ -z "$sub" ] && continue
                docker run --rm "$IMAGE" bash -c "timeout ${TIMEOUT_SECS}s $prog $sub < /dev/null 2>&1" \
                    > "$OUT_DIR/$prog/${sub}.txt" || true
            done <<< "$subs"
            echo "  $prog: $(echo "$subs" | wc -l) sub-commands" >&2
        fi
    fi
done

echo "Usage text captured under $OUT_DIR" >&2
