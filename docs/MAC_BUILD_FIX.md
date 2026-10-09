# If the build does not work on your Mac - guide for an AI assistant

**How to use:** open Claude / ChatGPT / any coding AI, paste this whole file plus the **exact error text** from your terminal, and say
"fix my nordSoldPow build on macOS". The checklist below tells the AI what to try, in order.

## Facts about the project
* Rust (edition 2021) + one dependency: `macroquad 0.4` (OpenGL window + audio). No C libraries to install on macOS.
* Entry points: `./build.sh` (makes `dist/nordSoldPow.app`), `./run.sh` (runs in release mode). Always build with `--release`.
* Assets are loaded at runtime from the `assets/` folder next to the binary (or `Contents/Resources/assets` in the .app). Missing assets never crash the game.
* Tested logic on Linux (unit tests + headless self-test). The macOS GUI path was NOT tested by the author, so window/audio issues are possible.

## Checklist (stop at the first step that fixes it)
1. **Rust missing** (`cargo: command not found`): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`, open a new terminal.
2. **Xcode tools** (`linker 'cc' not found`, `xcrun: error`): `xcode-select --install`, accept the licence (`sudo xcodebuild -license accept`).
3. **Old Rust** (errors about unstable features / edition): `rustup update stable`, then `cargo clean && ./build.sh`.
4. **Wrong architecture** on Apple Silicon: `rustup target add aarch64-apple-darwin`, `./build.sh aarch64-apple-darwin` (or `tools/bundle_macos.sh aarch64-apple-darwin`).
5. **Dependency download fails** (network/proxy): `cargo fetch` to see the error; retry; if behind a proxy set `HTTPS_PROXY`. `rm -rf ~/.cargo/registry/cache` then retry if a crate is corrupt.
6. **Compile error in our code**: run `cargo build --release 2>&1 | head -60`, read the first `error[...]` only. Fix the file/line it names (src/*.rs). Run `cargo test --release` afterwards (all tests must pass). Keep `commands.txt` in sync if you edit `src/console.rs` (a test checks it).
7. **App opens then closes / "damaged" / "unidentified developer"**: unsigned app. `xattr -dr com.apple.quarantine dist/nordSoldPow.app` or right-click -> Open. Run the binary directly to see errors: `dist/nordSoldPow.app/Contents/MacOS/nordsoldpow`.
8. **Black window or crash at start about GL/context**: run `./run.sh` from Terminal (not the .app) to see the panic text. macroquad uses OpenGL (deprecated but working on macOS). Try `./run.sh --nosound` to rule out audio.
9. **No sound / audio panic**: `./run.sh --nosound`. Check System Settings -> Sound output device exists. If audio crashes only in the .app, run the binary from Terminal for the message.
10. **"assets not found" / pink or missing sprites**: assets must sit next to the exe: `ls dist/nordSoldPow.app/Contents/Resources/assets`. Or `NORDSOLDPOW_ASSETS=/full/path/to/assets ./run.sh`.
11. **Slow / low FPS**: you must use `--release` (debug is 10x slower). Press F3 for FPS. Lower zoom in Settings.
12. **Retina scaling looks odd / wrong splitscreen clipping**: splitscreen uses macroquad's scissor with `screen_dpi_scale()` in `src/render.rs` (`fn scissor`). If the left/right halves are clipped wrongly, swap y handling there (macroquad scissor origin is top-left in 0.4, verified on Linux).
13. **Last resort**: `cargo clean && rm -rf dist && ./build.sh`. Still failing? Send the AI: output of `rustc --version`, `cargo --version`, `uname -m`, `sw_vers`, and the full error.

## Useful commands
```bash
./build.sh                       # build .app
./run.sh --nosound               # run without audio
./run.sh --planet midgard --console "god; give all"   # jump straight in with cheats
cargo test --release             # unit tests (sim rules, missions, console, save)
xvfb-run ./target/release/nordsoldpow --nosound --selftest   # headless smoke test (Linux)
python3 tools/gen_assets.py      # regenerate placeholder assets (Pillow + numpy)
```
