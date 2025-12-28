# CDP8 Build Environment
# Multi-stage build: first build PortAudio, then build CDP
#
# Usage from CDP8 root directory:
#   docker build -f docker-cdp8/Dockerfile -t cdp8 .
#
# Or use the build.sh script:
#   cd docker-cdp8 && ./build.sh

FROM ubuntu:22.04 AS builder

# Avoid interactive prompts during package installation
ENV DEBIAN_FRONTEND=noninteractive

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    git \
    wget \
    # Audio development libraries
    libasound2-dev \
    libjack-jackd2-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Build PortAudio v19.7 from source
WORKDIR /tmp
RUN wget -q http://files.portaudio.com/archives/pa_stable_v190700_20210406.tgz \
    && tar xzf pa_stable_v190700_20210406.tgz \
    && cd portaudio \
    && ./configure --with-alsa --with-jack \
    && make -j$(nproc) \
    && make install \
    && ldconfig

# Copy CDP8 source code
WORKDIR /cdp8
COPY . .

# Apply fixes for Linux/GCC compatibility
RUN bash docker-cdp8/fix-linux-compat.sh

# Build CDP8
RUN mkdir -p build \
    && cd build \
    && cmake .. \
    && make -j$(nproc)

# The built binaries will be in /cdp8/NewRelease/

# Runtime image with just the binaries
FROM ubuntu:22.04 AS runtime

ENV DEBIAN_FRONTEND=noninteractive

# Install only runtime dependencies (no dev packages)
RUN apt-get update && apt-get install -y \
    libasound2 \
    libjack-jackd2-0 \
    && rm -rf /var/lib/apt/lists/*

# Copy PortAudio library
COPY --from=builder /usr/local/lib/libportaudio* /usr/local/lib/
RUN ldconfig

# Copy built CDP binaries
COPY --from=builder /cdp8/NewRelease /opt/cdp/bin

# Add CDP binaries to PATH
ENV PATH="/opt/cdp/bin:${PATH}"

WORKDIR /workspace

# Default command shows available programs
CMD ["ls", "/opt/cdp/bin"]
