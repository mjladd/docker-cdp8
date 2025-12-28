#!/bin/bash
# Fix Linux/GCC compatibility issues in CDP8 source code

set -e

echo "Applying Linux/GCC compatibility fixes..."

# Fix 1: Make psf_round() available on all platforms
PORTSF_FILE="dev/externals/portsf/portsf.c"

awk '
/^#ifdef _MSC_VER$/ && !found {
    getline next_line
    if (next_line ~ /_MSC_VER <= 1200/) {
        found = 1
        depth = 1
        while (depth > 0) {
            if (getline <= 0) break
            if (/^#if/) depth++
            if (/^#endif/) depth--
        }
        print ""
        print "/* Portable convergent rounding function */"
        print "int psf_round(double val)"
        print "{"
        print "    long k;"
        print "    k = (long)(fabs(val)+0.5);"
        print "    if(val < 0.0)"
        print "        k = -k;"
        print "    return (int) k;"
        print "}"
        next
    } else {
        print "#ifdef _MSC_VER"
        print next_line
        next
    }
}
{ print }
' "$PORTSF_FILE" > "$PORTSF_FILE.tmp" && mv "$PORTSF_FILE.tmp" "$PORTSF_FILE"

echo "  Fixed: $PORTSF_FILE (psf_round)"

# Fix 2: Add __int64 typedef for non-Windows platforms
CERACU_FILE="dev/new/ceracu.c"

awk '
/#include <srates.h>/ {
    print
    print ""
    print "#ifndef WIN32"
    print "typedef long long __int64;"
    print "#endif"
    next
}
{ print }
' "$CERACU_FILE" > "$CERACU_FILE.tmp" && mv "$CERACU_FILE.tmp" "$CERACU_FILE"

echo "  Fixed: $CERACU_FILE (__int64 typedef)"

echo "All fixes applied successfully."
