#!/bin/bash

set -e  # Exit on error

APP_NAME="ethernaught"
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
glib-compile-resources res/linux.gresources.xml --target=res/resources.gresources
cargo build --profile "$BUILD_TYPE"

# Remove old package directory if exists
rm -rf "$DEB_DIR"

# Copy binary
mkdir -p "$DEB_DIR/usr/local/bin"
cp "$BUILD_DIR/$APP_NAME" "$DEB_DIR/usr/local/bin/"

# Create control file
mkdir -p "$DEB_DIR/DEBIAN"
cat > "$DEB_DIR/DEBIAN/control" <<EOF
Package: $APP_NAME
Version: $VERSION
Architecture: $ARCH
Maintainer: DrBrad <brad@bradeagle.com>
Description: Ethernaught - Packet sniffer
EOF

# Create .desktop file
mkdir -p "$DEB_DIR/usr/share/applications"
cat > "$DEB_DIR/usr/share/applications/ethernaught.desktop" <<EOF
[Desktop Entry]
Name=Ethernaught
GenericName=Ethernaught
Comment=Ethernaught - Packet sniffer
Keywords=packet;sniffer;capture;network;ethernet
Exec=ethernaught %f
Icon=ethernaught
MimeType=application/vnd.tcpdump.pcap;application/x-pcapng;application/x-snoop;application/x-iptrace;application/x-lanalyzer;application/x-nettl;application/x-radcom;application/x-etherpeek;application/x-visualnetworks;application/x-netinstobserver;application/x-5view;application/x-tektronix-rf5;application/x-micropross-mplog;application/x-apple-packetlogger;application/x-endace-erf;application/ipfix;application/x-ixia-vwr;
Terminal=false
Type=Application
Categories=Network;Monitor;Qt;
EOF

# Create icons
mkdir -p "$DEB_DIR/usr/share/icons/hicolor"

#cp -r res/hicolor/* "$DEB_DIR/usr/share/icons/hicolor/" || true
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/16x16/apps"
cp res/icons/app/icon_16x16.png       "$DEB_DIR/usr/share/icons/hicolor/16x16/apps/ethernaught.png"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/32x32/apps"
cp res/icons/app/icon_32x32.png       "$DEB_DIR/usr/share/icons/hicolor/32x32/apps/ethernaught.png"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/64x64/apps"
cp res/icons/app/icon_64x64.png       "$DEB_DIR/usr/share/icons/hicolor/64x64/apps/ethernaught.png"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/128x128/apps"
cp res/icons/app/icon_128x128.png       "$DEB_DIR/usr/share/icons/hicolor/128x128/apps/ethernaught.png"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/256x256/apps"
cp res/icons/app/icon_256x256.png       "$DEB_DIR/usr/share/icons/hicolor/256x256/apps/ethernaught.png"
mkdir -p "$DEB_DIR/usr/share/icons/hicolor/512x512/apps"
cp res/icons/app/icon_512x512.png       "$DEB_DIR/usr/share/icons/hicolor/512x512/apps/ethernaught.png"


# Create fonts
mkdir -p "$DEB_DIR/usr/share/fonts/truetype/$APP_NAME"
cp -r res/fonts/* "$DEB_DIR/usr/share/fonts/truetype/$APP_NAME/" || true

# Create database file
mkdir -p "$DEB_DIR/usr/var/lib/$APP_NAME"
cp database.db "$DEB_DIR/usr/var/lib/$APP_NAME/database.db"

# Build the .deb package
dpkg-deb --build "$DEB_DIR" "target/${APP_NAME}_${VERSION}_${ARCH}.deb"

echo "Deb package created: target/${APP_NAME}_${VERSION}_${ARCH}.deb"
