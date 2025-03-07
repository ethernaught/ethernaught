#!/bin/bash

set -e  # Exit on error

APP_NAME="ethernaut"
VERSION="0.1.0"
ARCH="amd64"  # Change if targeting different architectures
BUILD_DIR="target/release"
DEB_DIR="deb-pkg"

# Ensure cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found! Install Rust and Cargo first."
    exit 1
fi

# Build Rust project
echo "Building Rust project..."
cargo build --release

# Remove old package directory if exists
rm -rf "$DEB_DIR"
mkdir -p "$DEB_DIR/DEBIAN"
mkdir -p "$DEB_DIR/usr/local/bin"
mkdir -p "$DEB_DIR/usr/share/$APP_NAME/assets"
mkdir -p "$DEB_DIR/etc/$APP_NAME"

# Copy binary
cp "$BUILD_DIR/$APP_NAME" "$DEB_DIR/usr/local/bin/"

# Copy resources (if any)
cp -r res/etc/* "$DEB_DIR/etc/$APP_NAME/" || true  # Config files
cp -r res/assets/* "$DEB_DIR/usr/share/$APP_NAME/assets/" || true  # Assets

# Create control file
cat > "$DEB_DIR/DEBIAN/control" <<EOF
Package: $APP_NAME
Version: $VERSION
Architecture: $ARCH
Maintainer: DrBrad <brad@bradeagle.com>
Description: Ethernaut - A networking tool
EOF

# Build the .deb package
dpkg-deb --build "$DEB_DIR" "${APP_NAME}_${VERSION}_${ARCH}.deb"

echo "Deb package created: ${APP_NAME}_${VERSION}_${ARCH}.deb"
