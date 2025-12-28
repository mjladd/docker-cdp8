# CDP System Software, Release 8.
### Full release as of 24 October 2023

#### Copyright (c) 2022 Composers Desktop Project

![The CDP logo]( http://composersdesktop.com/logo.gif) 

	The CDP System is free software; you can redistribute them and/or modify them  
	under the  terms of the GNU Lesser General Public License as published by   
	the Free Software Foundation; either version 2.1 of the License,   
	or (at your option) any later version.

	The CDP System is distributed in the hope that it will be useful, but WITHOUT  
	ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or 
	FITNESS FOR A PARTICULAR PURPOSE.  
	See the GNU Lesser General Public License for more details.

	You should have received a copy of the GNU Lesser General Public License  
	along with this software (see the top-level file COPYING); if not, write to  
	the Free Software  Foundation, Inc., 51 Franklin St, Fifth Floor,  
	Boston, MA 02110-1301 USA
	

What's New?

* approximately 80 new programs/processes by Trevor Wishart. See details in the [docs](./docs) folder.
*  majority already available for use in Soundloom 17.0.4.
*  includes new programs for multichannel (<= 8 channels) production, waveset distortion, speech/voice processing.
*  ***PVOCEX*** (.pvx) analysis file support for all pvoc-related programs. This is the standard phase vocoder analysis file format used in Csound. See also the downloadable utilities (Mac, Win32), with example files, in the companion PVXTOOLS distribution:
*  play program **pvplay** will play  mono/stereo .pvx files.
*  classic CDP directory utility **dirsf** now recognises .pvx files with format details.
*  See also Tabula Vigilans (TV): some bugs fixed, full MIDI device I/O now working on Linux.

What's needed?

* Developers to get involved (whether CDP8 programs or TV), especially to add new software, create new user interfaces, create libraries, and so many other things we have not thought of. Join the new **cdp-dev** mailing list (see *building.txt* for details); get immediate news of any updates pushed to github.


[updated 28/12/2023]

---

## Building CDP8

### Quick Start with Docker (Recommended)

The easiest way to build and run CDP8 is with Docker:

```bash
# Build the container (from the CDP8 root directory)
docker build -f docker-cdp8/Dockerfile -t cdp8 .

# List all available programs
docker run --rm cdp8

# Run a program (mounting current directory as /workspace)
docker run --rm -v $(pwd):/workspace cdp8 sndinfo props /workspace/myfile.wav
```

### Native Build

For native builds on macOS, Linux, or Windows, see [building.txt](building.txt) for full details.

**Prerequisites:**
- CMake 3.5+
- C/C++ compiler (GCC, Clang, or MSVC)
- PortAudio v19.7 (required for audio playback/recording programs)

**Build steps:**
```bash
# 1. Build and install PortAudio first (see dev/externals/pa*build.txt for details)

# 2. Create build directory
mkdir build && cd build

# 3. Generate makefiles
cmake ..                          # macOS/Linux/MSVC
cmake -G "MinGW Makefiles" ..     # Windows MinGW

# 4. Build
make                              # macOS/Linux
mingw32-make                      # Windows MinGW
```

Compiled binaries are written to the `NewRelease/` directory.

---

## Testing / Generating Example Sounds

Once built, test the installation by generating a simple audio file:

```bash
# Using Docker:
docker run --rm -v $(pwd):/workspace cdp8 synth wave 1 /workspace/test_sine.wav 44100 1 2 440

# Native (from NewRelease directory):
./synth wave 1 test_sine.wav 44100 1 2 440
```

This generates a 2-second, 440Hz sine wave. Verify it with:

```bash
# Docker:
docker run --rm -v $(pwd):/workspace cdp8 sndinfo props /workspace/test_sine.wav

# Native:
./sndinfo props test_sine.wav
```

### More synthesis examples

```bash
# Generate noise (white noise, 1 second)
synth noise 1 noise.wav 44100 1 1

# Generate a chord (C major, 3 seconds)
synth chord chord.wav 44100 1 3 261.63 329.63 392.00

# Generate clicks (10 clicks per second, 2 seconds)
synth clicks clicks.wav 44100 1 2 10
```

Run any program without arguments to see its usage:
```bash
docker run --rm cdp8 synth
docker run --rm cdp8 distort
```

---