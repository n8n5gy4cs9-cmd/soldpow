#!/usr/bin/env bash
# Portable package for the current platform: dist/nordSoldPow-<os>/ + zip.  Run after `cargo build --release`.
set -euo pipefail
cd "$(dirname "$0")/.."
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"; OUT="dist/nordSoldPow-$OS"
[ -f target/release/nordsoldpow ] || [ -f target/release/nordsoldpow.exe ] || cargo build --release
rm -rf "$OUT"; mkdir -p "$OUT"
cp target/release/nordsoldpow* "$OUT"/ 2>/dev/null || true; rm -f "$OUT"/*.d "$OUT"/*.pdb
cp -R assets "$OUT/assets"; cp README.md commands.txt assets.md LICENSE "$OUT"/
(cd dist && zip -qry "nordSoldPow-$OS.zip" "nordSoldPow-$OS")
echo "Packaged dist/nordSoldPow-$OS.zip"
