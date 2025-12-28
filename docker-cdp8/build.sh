#!/bin/bash
# Build CDP8 Docker image
#
# Run this script from the docker-cdp8 directory:
#   ./build.sh
#
# Or with a custom tag:
#   ./build.sh mytag

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CDP8_ROOT="$(dirname "$SCRIPT_DIR")"
TAG="${1:-cdp8}"

echo "Building CDP8 Docker image..."
echo "  Source: $CDP8_ROOT"
echo "  Tag: $TAG"
echo ""

cd "$CDP8_ROOT"
docker build -f docker-cdp8/Dockerfile -t "$TAG" .

echo ""
echo "Build complete! Image tagged as: $TAG"
echo ""
echo "Usage examples:"
echo "  docker run --rm $TAG                           # List available programs"
echo "  docker run -it --rm -v \$(pwd):/workspace $TAG /bin/bash  # Interactive shell"
echo "  docker run --rm -v \$(pwd):/workspace $TAG sndinfo /workspace/file.wav"
