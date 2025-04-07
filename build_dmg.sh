#!/bin/bash

set -e  # Exit on error

APP_NAME="ethernaught"
VERSION="0.1.0"
BUILD_TYPE=${1:release}
ARCH="amd64"
BUILD_DIR="target/$BUILD_TYPE"
APP_DIR="target/ethernaught.app"

# Ensure cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo not found! Install Rust and Cargo first."
    exit 1
fi

# Build Rust project
echo "Building Rust project in $BUILD_TYPE mode..."
glib-compile-resources res/macos.gresources.xml --target=res/resources.gresources
cargo build --profile "$BUILD_TYPE"

# Remove old package directory if exists
rm -rf "$APP_DIR"

mkdir -p "$APP_DIR/Contents/MacOS"
cp "$BUILD_DIR/$APP_NAME" "$APP_DIR/Contents/MacOS/"

cat > "$APP_DIR/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" \
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>ethernaught</string>
    <key>CFBundleIdentifier</key>
    <string>net.ethernaught.rust</string>
    <key>CFBundleName</key>
    <string>MyApp</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string>icon</string>
</dict>
</plist>
EOF

mkdir -p target/icon.iconset

cp res/icons/app/icon_16x16.png       target/icon.iconset/icon_16x16.png
cp res/icons/app/icon_32x32.png       target/icon.iconset/icon_16x16@2x.png
cp res/icons/app/icon_128x128.png     target/icon.iconset/icon_128x128.png
cp res/icons/app/icon_256x256.png     target/icon.iconset/icon_128x128@2x.png
cp res/icons/app/icon_512x512.png     target/icon.iconset/icon_256x256@2x.png
#cp res/icons/app/icon_1024x1024.png   target/ethernaught.iconset/icon_512x512@2x.png

iconutil -c icns target/icon.iconset

mkdir -p "$APP_DIR/Contents/Resources"
mv target/icon.icns "$APP_DIR/Contents/Resources/"
