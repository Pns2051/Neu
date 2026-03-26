<p align="center">
  <img src="app/assets/logo.jpg" width="120" alt="Neu Logo" />
</p>

# Neu

> A production-grade, cross-platform, modular music platform built with Rust and Slint.

![License](https://img.shields.io/badge/license-GPLv3-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)
![Slint](https://img.shields.io/badge/ui-Slint-brightgreen.svg)

Music Client is a high-performance audio player designed from the ground up for extensibility. By leveraging a custom streaming buffer on top of Rodio, and a strict Plugin SDK, this app seamlessly unifies local libraries, YouTube streams, and algorithmic recommendations into a single, beautiful MD3 interface.

## 🌟 Features

- **Unified Data Model**: One interface for YouTube, Local Files, Spotify, and Apple Music (Extensible via Plugins).
- **Custom Playback Engine**: Asynchronous chunk buffering integrated with `rodio` for gapless, HTTP-streamed audio.
- **YouTube Innertube SDK**: Bypasses typical scrape limits with dynamic signature deciphering using AST-like parsing limits.
- **Smart Recommendations**: A local, privacy-respecting Cosine-Similarity engine that recommends tracks based on your listening history embeddings.
- **Tiered Distributed Cache**: Intelligent `Memory → Disk → Remote` caching ensures offline capability and minimal API usage.
- **Cross-Platform MD3 UI**: A sleek, performant interface written in Slint, compiling natively to both Desktop and Android.

## 🏗️ Architecture

The workspace is divided into specific, decoupled crates:
- `app`: Slint UI, State config, Walkdir Local Plugin, and Cache.
- `plugin_sdk`: The foundational traits (`MusicPlugin`, `UnifiedTrack`).
- `playback`: Custom `StreamBuffer` and `rodio` wrapper.
- `yt_cipher`: YouTube streaming URL extractor and cipher descrambler.
- `recommender`: High-speed vector similarity engine.

## 🚀 Getting Started

### Prerequisites
- **Rust** (1.75+)
- **Slint Build Dependencies** (CMake, C++ Compiler for some backends)

### Desktop Build
1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/music-client.git
   cd music-client
   ```
2. Run the application:
   ```sh
   cargo run -p app --release
   ```

### Android Build
You can build the native Android APK using the provided script (requires `cargo-apk` and `cross`):
```sh
./build_android.sh
```

## 📦 Packaging for Release (.exe, .deb, macOS, .apk)

To distribute Neu to end users without requiring them to install Rust, you can build native app bundles. We recommend using `cargo-packager`:

### 1. Windows (`.exe` / `.msi`)
Build from a Windows machine or via GitHub Actions.
```shell
cargo install cargo-packager
cargo packager --release
```
*This generates a standalone executable and `.msi` installer in `target/release/bundles/`.*

### 2. Linux (`.deb` / `.AppImage`)
Ensure you have `dpkg-dev` installed.
```shell
cargo install cargo-packager
cargo packager --release
```
*Produces a `.deb` Debian package and an independent `.AppImage`.*

### 3. macOS (`.app` / `.dmg`)
Build from a Mac (or macOS CI runner).
```shell
cargo install cargo-packager
cargo packager --release
```
*Creates a `.dmg` disk image ready for distribution.*

### 4. Android (`.apk`)
Run the included shell script to generate the Android application package:
```shell
./build_android.sh
```
*This cross-compiles the Slint UI into an APK located in `target/android/release/apk/`.*


## 🤝 Contributing

We welcome contributions! Please see our [CONTRIBUTING.md](./CONTRIBUTING.md) for detailed guidelines. **Note**: We allow AI-assisted development, provided the code is thoroughly supervised by an experienced programmer.

## 📜 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
