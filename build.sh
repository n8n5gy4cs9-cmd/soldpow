#!/usr/bin/env bash
# nordSoldPow - build the macOS .app (Apple Silicon native).  Usage: ./build.sh
# Output: dist/nordSoldPow.app  and  dist/nordSoldPow-macos.zip
# If anything fails, see docs/MAC_BUILD_FIX.md (written so you can paste it to an AI assistant).
set -euo pipefail
cd "$(dirname "$0")"
if ! command -v cargo >/dev/null; then
  echo "Rust is not installed. Install it with:"
  echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
  echo "then open a NEW terminal and run ./build.sh again."; exit 1
fi
if [ "$(uname -s)" = "Darwin" ] && ! xcode-select -p >/dev/null 2>&1; then
  echo "Xcode command line tools missing. Run:  xcode-select --install   then ./build.sh again."; exit 1
fi
if [ "$(uname -s)" = "Darwin" ]; then
  chmod +x tools/bundle_macos.sh && tools/bundle_macos.sh
  echo; echo "Done. Start with:  open dist/nordSoldPow.app    (first time: right-click -> Open)"
else
  echo "Not macOS: building a normal release binary + package."
  cargo build --release && chmod +x tools/package.sh && tools/package.sh
fi
