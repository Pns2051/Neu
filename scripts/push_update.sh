#!/bin/bash
set -e

echo "🚀 Starting Neu Music Platform Update Cycle..."

# 1. Version Management
VERSION_FILE="VERSION"
CURRENT_VERSION=$(cat $VERSION_FILE 2>/dev/null || echo "1.0.0")
echo "Current Version: $CURRENT_VERSION"

# Use argument if provided, otherwise increment minor version
if [ -z "$1" ]; then
    IFS='.' read -r -a parts <<< "$CURRENT_VERSION"
    parts[2]=$((parts[2] + 1))
    NEW_VERSION="${parts[0]}.${parts[1]}.${parts[2]}"
else
    NEW_VERSION=$1
fi

echo "🚀 Releasing Neu $NEW_VERSION..."
echo $NEW_VERSION > $VERSION_FILE

# 2. Build C++/Qt Project
mkdir -p build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)

# 3. Package for Linux (.deb, .rpm, AppImage)
cpack

# 4. Snap Package
# snapcraft --use-lxd

# 5. Flatpak Package
echo "📦 Building Flatpak bundle..."
# flatpak-builder --force-clean build-dir org.neu.Neu.yml
# flatpak build-bundle repo org.neu.Neu.flatpak org.neu.Neu

echo "✅ Build Complete! Packages generated in $(pwd)/"

# 5. Push to GitHub (Triggers Automated CI/CD Release)
echo "📤 Pushing updates to GitHub..."
git add .
git commit -m "chore: release $NEW_VERSION"
git tag v$NEW_VERSION
git push origin main --tags

echo "🎉 Update $NEW_VERSION is now live for all users! (Builds starting on GitHub Actions)"
