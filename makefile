OS ?= linux
#OS ?= $(shell uname)
PROFILE ?= release
GTK ?= gtk4


BUILD_DIR = target/build
APP_NAME = ethernaught

RESOURCE_XML = res/$(GTK)/$(OS).gresources.xml
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
ifeq ($(OS),linux)
    #else ifeq ($(OS),macos)

	@echo "Generating MacOS App Image"
	@rm -rf "$(BUILD_DIR)"
	@mkdir -p "$(BUILD_DIR)/dmg-pkg/$(APP_NAME).app/Contents/MacOS"
	@cp "target/$(PROFILE)/$(APP_NAME)" "$(BUILD_DIR)/dmg-pkg/$(APP_NAME).app/Contents/MacOS/";
	@printf '%s\n' '<?xml version="1.0" encoding="UTF-8"?>' \
    '<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">' \
    '<plist version="1.0">' \
    '<dict>' \
    '    <key>CFBundleExecutable</key>' \
    '    <string>ethernaught</string>' \
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



else
	@echo "Unknown OS. Skipping postbuild."
endif

clean:
	cargo clean
