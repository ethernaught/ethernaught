#!/bin/bash

set -e  # Exit on error

APP_NAME="ethernaut"
VERSION="0.1.0"
BUILD_TYPE=${1:release}
ARCH="amd64"
BUILD_DIR="target/$BUILD_TYPE"
DEB_DIR="target/deb-pkg"

# Ensure cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found! Install Rust and Cargo first."
    exit 1
fi

# Build Rust project
echo "Building Rust project in $BUILD_TYPE mode..."
glib-compile-resources res/gresources.xml --target=res/resources.gresources
cargo build --profile "$BUILD_TYPE"

# Remove old package directory if exists
rm -rf "$DEB_DIR"

# Create control file
mkdir -p "$DEB_DIR/DEBIAN"
cat > "$DEB_DIR/DEBIAN/control" <<EOF
Package: $APP_NAME
Version: $VERSION
Architecture: $ARCH
Maintainer: DrBrad <brad@bradeagle.com>
Description: Ethernaut - Packet sniffer
EOF

# Copy binary
mkdir -p "$DEB_DIR/usr/local/bin"
cp "$BUILD_DIR/$APP_NAME" "$DEB_DIR/usr/local/bin/"

# Create .desktop file
mkdir -p "$DEB_DIR/usr/share/applications"
cat > "$DEB_DIR/usr/share/applications/ethernaut.desktop" <<EOF
[Desktop Entry]
Name=Ethernaut
GenericName=Ethernaut
Comment=Ethernaut - Packet sniffer
Keywords=packet;sniffer;capture;
Exec=ethernaut
Icon=ethernaut
Terminal=false
Type=Application
EOF

# Create icons
mkdir -p "$DEB_DIR/usr/share/icons/hicolor"
cp -r res/hicolor/* "$DEB_DIR/usr/share/icons/hicolor/" || true

# Create database file
mkdir -p "$DEB_DIR/usr/var/lib/$APP_NAME"
cp database.db "$DEB_DIR/usr/var/lib/$APP_NAME/database.db"

# Build the .deb package
dpkg-deb --build "$DEB_DIR" "target/${APP_NAME}_${VERSION}_${ARCH}.deb"

echo "Deb package created: target/${APP_NAME}_${VERSION}_${ARCH}.deb"
