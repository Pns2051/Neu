# Neu (C++/Qt Edition)

High-performance music client for Linux.

## Requirements
- **Qt 6.6+** (Core, Gui, Qml, Quick, Network, Multimedia, Sql, OAuth)
- **CMake 3.16+**
- **GStreamer** (for Qt Multimedia backend)
- **libpulse-dev / libasound2-dev** (for optimized audio)

## Build Instructions
```bash
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)
./neu-cpp
```

## Packaging
Use the provided `push_update.sh` script to build DEB, RPM, AppImage, Snap, and Flatpak packages simultaneously.

## Features
- **Social Login**: YouTube & Spotify OAuth2.
- **Lyrics**: LRCLIB integration.
- **High Performance**: Native C++ processing.
- **Glassmorphism UI**: Modern MD3 design.
