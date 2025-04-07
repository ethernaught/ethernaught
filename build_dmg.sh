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

#mkdir -p "$APP_DIR/Contents/Resources"

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
</dict>
</plist>
EOF



#set -e

#APP_NAME="Ethernaught"
#APP_BUNDLE="${APP_NAME}.app"
#DMG_NAME="${APP_NAME}.dmg"
#VOLUME_NAME="${APP_NAME} Installer"
#STAGING_DIR="dmg_staging"
#FONT_NAME="EthernaughtMono-Regular.ttf"
#ICON_FILE="icon.icns"
#
## Cleanup from previous runs
#rm -rf "$STAGING_DIR" "$APP_BUNDLE" "$DMG_NAME"
#
#echo "[+] Creating .app bundle"
#mkdir -p "$APP_BUNDLE/Contents/MacOS"
#mkdir -p "$APP_BUNDLE/Contents/Resources"
#
## Copy binary
#cp "$APP_NAME" "$APP_BUNDLE/Contents/MacOS/"
#
## Set up Info.plist
#cat > "$APP_BUNDLE/Contents/Info.plist" <<EOF
#<?xml version="1.0" encoding="UTF-8"?>
#<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" \
#  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
#<plist version="1.0">
#<dict>
#    <key>CFBundleExecutable</key>
#    <string>$APP_NAME</string>
#    <key>CFBundleIdentifier</key>
#    <string>net.ethernaught.$APP_NAME</string>
#    <key>CFBundleName</key>
#    <string>$APP_NAME</string>
#    <key>CFBundleVersion</key>
#    <string>1.0</string>
#    <key>CFBundleIconFile</key>
#    <string>icon</string>
#</dict>
#</plist>
#EOF
#
## Copy icon and font into the app bundle (font optional for manual install)
##cp "$ICON_FILE" "$APP_BUNDLE/Contents/Resources/icon.icns"
#cp "$FONT_NAME" "$APP_BUNDLE/Contents/Resources/"
#
#echo "[+] Creating DMG staging directory"
#mkdir "$STAGING_DIR"
#cp -R "$APP_BUNDLE" "$STAGING_DIR/"
#
## Optional: Include a fonts install script or instructions
#cat > "$STAGING_DIR/InstallFont.command" <<EOF
##!/bin/bash
#cp "\$(dirname "\$0")/$APP_BUNDLE/Contents/Resources/$FONT_NAME" ~/Library/Fonts/
#echo "Font installed!"
#EOF
#chmod +x "$STAGING_DIR/InstallFont.command"
#
## Create the DMG
#echo "[+] Creating DMG..."
#hdiutil create -volname "$VOLUME_NAME" -srcfolder "$STAGING_DIR" -ov -format UDZO "$DMG_NAME"
#
#echo "[✓] DMG created: $DMG_NAME"
#