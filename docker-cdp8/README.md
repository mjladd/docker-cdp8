# CDP8 Docker Build

Build and run CDP8 audio processing tools in a Docker container without installing dependencies on your host machine.

## Quick Start

```bash
cd docker-cdp8
./build.sh
```

Or from the CDP8 root directory:

```bash
docker build -f docker-cdp8/Dockerfile -t cdp8 .
```

## Usage

### List available programs

```bash
docker run --rm cdp8
```

### Interactive shell

```bash
docker run -it --rm -v $(pwd):/workspace cdp8 /bin/bash
```

### Run a specific program

```bash
docker run --rm -v $(pwd):/workspace cdp8 sndinfo /workspace/myfile.wav
```

### Extract binaries to host

```bash
docker create --name cdp8-temp cdp8
docker cp cdp8-temp:/opt/cdp/bin ./cdp-binaries
docker rm cdp8-temp
```

## What's Included

The Docker image includes:
- Ubuntu 22.04 base
- PortAudio v19.7 (built with ALSA and JACK support)
- All 220+ CDP8 command-line tools

## Linux Compatibility Fixes

The `fix-linux-compat.sh` script applies the following fixes during build:

1. **psf_round()** - Makes this function available on all platforms (originally wrapped in MSVC-only preprocessor conditionals)
2. **__int64 typedef** - Adds a typedef for the `__int64` type on non-Windows platforms

These fixes are required because the CDP8 codebase was primarily developed for Windows/MSVC.

## Build Stages

The Dockerfile uses a multi-stage build:

1. **builder** - Full build environment with compilers and dev libraries
2. **runtime** - Minimal image with only runtime dependencies and compiled binaries

The final image is approximately 141MB.

## Files

- `Dockerfile` - Multi-stage Docker build configuration
- `build.sh` - Convenience script to build the image
- `fix-linux-compat.sh` - Script that patches source for Linux/GCC compatibility
