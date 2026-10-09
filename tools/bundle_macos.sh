#!/usr/bin/env bash
# Build a macOS .app bundle (native Apple Silicon by default). Run on a Mac:  tools/bundle_macos.sh [target-triple]
set -euo pipefail
cd "$(dirname "$0")/.."
TARGET="${1:-}"
if [ -z "$TARGET" ] && [ "$(uname -m)" = "arm64" ]; then TARGET="aarch64-apple-darwin"; fi
if [ -n "$TARGET" ]; then rustup target add "$TARGET" >/dev/null 2>&1 || true; cargo build --release --target "$TARGET"; BIN="target/$TARGET/release/nordsoldpow"; else cargo build --release; BIN="target/release/nordsoldpow"; fi
APP="dist/nordSoldPow.app"
rm -rf "$APP"; mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/nordsoldpow"
cp -R assets "$APP/Contents/Resources/assets"
# icon (optional, uses macOS sips + iconutil)
if command -v sips >/dev/null && command -v iconutil >/dev/null; then
  ICONSET="$(mktemp -d)/nord.iconset"; mkdir -p "$ICONSET"
  for s in 16 32 64 128 256 512; do sips -z $s $s assets/planets/midgard.png --out "$ICONSET/icon_${s}x${s}.png" >/dev/null 2>&1 || true; done
  iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/nordsoldpow.icns" 2>/dev/null || true
fi
cat > "$APP/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleName</key><string>nordSoldPow</string>
<key>CFBundleDisplayName</key><string>nordSoldPow</string>
<key>CFBundleIdentifier</key><string>dev.crowelian.nordsoldpow</string>
<key>CFBundleVersion</key><string>0.1.0</string>
<key>CFBundleShortVersionString</key><string>0.1.0</string>
<key>CFBundleExecutable</key><string>nordsoldpow</string>
<key>CFBundleIconFile</key><string>nordsoldpow</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>NSHighResolutionCapable</key><true/>
<key>LSMinimumSystemVersion</key><string>11.0</string>
</dict></plist>
PLIST
chmod +x "$APP/Contents/MacOS/nordsoldpow"
(cd dist && zip -qry nordSoldPow-macos.zip nordSoldPow.app)
echo "Built $APP and dist/nordSoldPow-macos.zip"
echo "First launch of an unsigned app: right-click -> Open (or: xattr -dr com.apple.quarantine $APP)"
