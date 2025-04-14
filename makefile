OS ?= debian
#OS ?= $(shell uname)
PROFILE ?= release
GTK ?= gtk4


BUILD_DIR = target/build
VERSION = 0.1.0
APP_NAME = ethernaught
OS_TYPE := $(if $(filter $(OS),rpm debian),linux,$(OS))

RESOURCE_XML = res/$(GTK)/$(OS_TYPE).gresources.xml
RESOURCE_TARGET = res/resources.gresources

CARGO_ARGS := --profile $(PROFILE) --no-default-features --features $(GTK)

.PHONY: all build resources clean

all: build postbuild

build: resources
	@echo "Building for OS=$(OS), Profile=$(PROFILE), GTK=$(GTK)"
	cargo build $(CARGO_ARGS)

resources:
	@echo "Compiling resources for GTK=$(GTK) on OS=$(OS)"
ifeq ($(GTK),gtk4)
	glib-compile-resources $(RESOURCE_XML) --target=$(RESOURCE_TARGET)
else ifeq ($(GTK),gtk3)
	glib-compile-resources $(RESOURCE_XML) --target=$(RESOURCE_TARGET)
else
	$(error GTK must be 'gtk3' or 'gtk4')
endif

postbuild:
ifeq ($(OS),debian)
	@echo "Generating Linux DEB"
	@rm -rf "$(BUILD_DIR)"
	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/local/bin"
	@cp "target/$(PROFILE)/$(APP_NAME)" "$(BUILD_DIR)/deb-pkg/usr/local/bin/"
	@mkdir -p "$(BUILD_DIR)/deb-pkg/DEBIAN"

	@printf '%s\n' 'Package: $(APP_NAME)' \
        'Version: $(VERSION)' \
        'Architecture: amd64' \
        'Maintainer: DrBrad <brad@bradeagle.com>' \
        'Description: Ethernaught - Packet sniffer' > "$(BUILD_DIR)/deb-pkg/DEBIAN/control"

	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/applications"
	@printf '%s\n' '[Desktop Entry]' \
        'Name=Ethernaught' \
        'GenericName=Ethernaught' \
        'Comment=Ethernaught - Packet sniffer' \
        'Keywords=packet;sniffer;capture;network;ethernet' \
        'Exec=$(APP_NAME) %f' \
        'Icon=$(APP_NAME)' \
        'MimeType=application/vnd.tcpdump.pcap;application/x-pcapng;application/x-snoop;application/x-iptrace;application/x-lanalyzer;application/x-nettl;application/x-radcom;application/x-etherpeek;application/x-visualnetworks;application/x-netinstobserver;application/x-5view;application/x-tektronix-rf5;application/x-micropross-mplog;application/x-apple-packetlogger;application/x-endace-erf;application/ipfix;application/x-ixia-vwr;' \
        'Terminal=false' \
        'Type=Application' \
        'Categories=Network;Monitor;Qt;' > "$(BUILD_DIR)/deb-pkg/usr/share/applications/$(APP_NAME).desktop"

	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor"

	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/16x16/apps"
	@cp res/icons/app/icon_16x16.png "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/16x16/apps/$(APP_NAME).png"
	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/32x32/apps"
	@cp res/icons/app/icon_32x32.png "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/32x32/apps/$(APP_NAME).png"
	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/64x64/apps"
	@cp res/icons/app/icon_64x64.png "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/64x64/apps/$(APP_NAME).png"
	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/128x128/apps"
	@cp res/icons/app/icon_128x128.png "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/128x128/apps/$(APP_NAME).png"
	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/256x256/apps"
	@cp res/icons/app/icon_256x256.png "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/256x256/apps/$(APP_NAME).png"
	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/512x512/apps"
	@cp res/icons/app/icon_512x512.png "$(BUILD_DIR)/deb-pkg/usr/share/icons/hicolor/512x512/apps/$(APP_NAME).png"

	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/share/fonts/truetype/$(APP_NAME)"
	@cp -r res/fonts/* "$(BUILD_DIR)/deb-pkg/usr/share/fonts/truetype/$(APP_NAME)/" || true

	@mkdir -p "$(BUILD_DIR)/deb-pkg/usr/var/lib/$(APP_NAME)"
	@cp database.db "$(BUILD_DIR)/deb-pkg/usr/var/lib/$(APP_NAME)/database.db"

	@dpkg-deb --build "$(BUILD_DIR)/deb-pkg" "$(BUILD_DIR)/$(APP_NAME)_$(VERSION).deb"

	@echo "Linux DEB package created: $(BUILD_DIR)/$(APP_NAME)_$(VERSION).deb"

else ifeq ($(OS),rpm)
	@echo "Generating Linux RPM"
	@rm -rf "$(BUILD_DIR)"
	@mkdir -p "$(BUILD_DIR)/rpm-pkg/"

else ifeq ($(OS),macos)
	@echo "Generating MacOS App Image"
	@rm -rf "$(BUILD_DIR)"
	@mkdir -p "$(BUILD_DIR)/dmg-pkg/$(APP_NAME).app/Contents/MacOS"
	@cp "target/$(PROFILE)/$(APP_NAME)" "$(BUILD_DIR)/dmg-pkg/$(APP_NAME).app/Contents/MacOS/";
	@printf '%s\n' '<?xml version="1.0" encoding="UTF-8"?>' \
        '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">' \
        '<plist version="1.0">' \
        '<dict>' \
        '    <key>CFBundleExecutable</key>' \
        '    <string>$(APP_NAME)</string>' \
        '    <key>CFBundleIdentifier</key>' \
        '    <string>net.ethernaught.rust</string>' \
        '    <key>CFBundleName</key>' \
        '    <string>Ethernaught</string>' \
        '    <key>CFBundleVersion</key>' \
        '    <string>1.0</string>' \
        '    <key>CFBundlePackageType</key>' \
        '    <string>APPL</string>' \
        '    <key>CFBundleIconFile</key>' \
        '    <string>icon</string>' \
        '</dict>' \
        '</plist>' > "$(BUILD_DIR)/dmg-pkg/$(APP_NAME).app/Contents/Info.plist"
	@mkdir -p "$(BUILD_DIR)/icon.iconset"

	@cp res/icons/app/icon_16x16.png "$(BUILD_DIR)/icon.iconset/icon_16x16.png"
	@cp res/icons/app/icon_32x32.png "$(BUILD_DIR)/icon.iconset/icon_16x16@2x.png"
	@cp res/icons/app/icon_128x128.png "$(BUILD_DIR)/icon.iconset/icon_128x128.png"
	@cp res/icons/app/icon_256x256.png "$(BUILD_DIR)/icon.iconset/icon_128x128@2x.png"
	@cp res/icons/app/icon_512x512.png "$(BUILD_DIR)/icon.iconset/icon_256x256@2x.png"
	#@cp res/icons/app/icon_1024x1024.png "$(BUILD_DIR)/icon.iconset/icon_512x512@2x.png"
	@iconutil -c icns "$(BUILD_DIR)/icon.iconset"

	@mkdir -p "$(BUILD_DIR)/dmg-pkg/$(APP_NAME).app/Contents/Resources"
	@mv "$(BUILD_DIR)/icon.icns" "$(BUILD_DIR)/dmg-pkg/$(APP_NAME).app/Contents/Resources/"

	@echo "Generating MacOS DMG"
	@ln -s /Applications "$(BUILD_DIR)/dmg-pkg/Applications"
	@hdiutil create -volname "$(APP_NAME) Installer" \
        -srcfolder "$(BUILD_DIR)/dmg-pkg" \
        -ov -format UDZO "$(BUILD_DIR)/$(APP_NAME)_$(VERSION).dmg"
	@echo "MacOS DMG package created: $(BUILD_DIR)/$(APP_NAME)_$(VERSION).dmg"

else ifeq ($(OS),windows)
	@echo "Generating Windows EXE"
	@rm -rf "$(BUILD_DIR)"
	@mkdir -p "$(BUILD_DIR)/exe-pkg/"

else
	@echo "Unknown OS. Skipping postbuild."
endif

clean:
	cargo clean
