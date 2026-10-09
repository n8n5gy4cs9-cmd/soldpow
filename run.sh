#!/usr/bin/env bash
# nordSoldPow - build (if needed) and run in release mode.  Usage: ./run.sh [game options]
# Examples:  ./run.sh     ./run.sh --planet helheim --console "give all; god"     ./run.sh --nosound
cd "$(dirname "$0")"
command -v cargo >/dev/null || { echo "Rust missing: https://rustup.rs"; exit 1; }
exec cargo run --release -- "$@"
