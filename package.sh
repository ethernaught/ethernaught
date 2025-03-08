#!/bin/bash

set -e  # Exit on error

APP_NAME="ethernaut"
VERSION="0.1.0"
ARCH="amd64"  # Change if targeting different architectures
BUILD_DIR="target/release"
DEB_DIR="target/deb-pkg"

# Ensure cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found! Install Rust and Cargo first."
    exit 1
fi

# Build Rust project
echo "Building Rust project..."
glib-compile-resources res/gresources.xml --target=res/resources.gresources
cargo build --release

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
Name=$APP_NAME
GenericName=Ethernaut
Comment=Ethernaut - Packet sniffer
Keywords=packet;sniffer;capture;
Exec=ethernaut
Icon=ethernaut
Terminal=false
Type=Application
EOF

#mkdir -p "$DEB_DIR/usr/share/$APP_NAME/icons/hicolor"
mkdir -p "$DEB_DIR/usr/var/lib/$APP_NAME"
cp "database.db" "$DEB_DIR/usr/var/lib/$APP_NAME/database.db"

# Build the .deb package
dpkg-deb --build "$DEB_DIR" "target/${APP_NAME}_${VERSION}_${ARCH}.deb"

echo "Deb package created: target/${APP_NAME}_${VERSION}_${ARCH}.deb"
