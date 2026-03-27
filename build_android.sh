#!/bin/bash
set -e

echo "Ensuring required targets and tools..."
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
cargo install cargo-apk || echo "cargo-apk already installed"

echo "Building Android APK..."
cd app
cargo apk build --release

echo "Android APK built successfully."