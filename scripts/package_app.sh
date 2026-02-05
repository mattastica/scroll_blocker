#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Scroll Blocker"
BUNDLE_DIR="${APP_NAME}.app"
CONTENTS_DIR="${BUNDLE_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"

# Build release binary
cargo build --release

# Create bundle structure
mkdir -p "${MACOS_DIR}"

# Copy binary
cp -f "target/release/scroll_blocker" "${MACOS_DIR}/scroll_blocker"

# Write Info.plist
cat > "${CONTENTS_DIR}/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>scroll_blocker</string>
  <key>CFBundleIdentifier</key>
  <string>com.grace.scrollblocker</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>Scroll Blocker</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleVersion</key>
  <string>1.0</string>
  <key>CFBundleShortVersionString</key>
  <string>1.0</string>
  <key>LSUIElement</key>
  <true/>
</dict>
</plist>
PLIST

# Ad-hoc sign
codesign --force --deep --sign - "${BUNDLE_DIR}"

echo "Built and signed: ${BUNDLE_DIR}"
