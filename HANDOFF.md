# HANDOFF - read this first when resuming (after tokens run out / next session / other AI)

**Project:** nordSoldPow - Rust + macroquad mashup of The Powder Toy and Soldat in Nordic/Kalevala setting.
**Author:** Crowelian 2026 + Sonnet.   **Branch:** `claude/nordsoldpow-game-prototype-5z39rh`

## How to resume in 60 seconds
1. `cat task.md` - the `NEXT UP` block at the top is where to continue. `[~]` = was in progress.
2. `python3 tools/task.py list` shows all open tasks. Mark progress: `python3 tools/task.py done 3.4` (updates PRD.json + task.md).
3. Read `PRD.md` (what), `docs/PLAN.md` (how / module map), `docs/LORE.md` (story). Assets: `assets.md`.
4. Build/run: `cargo run --release` (never debug - sim is slow). Tests: `cargo test --release`. Headless: `cargo run --release -- --selftest`.
5. Regenerate placeholder assets: `python3 tools/gen_assets.py` (needs Pillow + numpy). It also rewrites `assets.md`.

## Rules for any continuing AI
* Keep `PRD.json` and `task.md` updated after EVERY finished chunk (use tools/task.py). Commit + push often.
* Console commands live in ONE table in `src/console.rs`; keep `commands.txt` in sync (a unit test checks it).
* Assets must be loaded at runtime from `assets/` with fallbacks; never `include_bytes!` art you want users to swap.
* ASCII-only strings in the UI (default macroquad font). Accents only in docs.
* Reference repos CAN be cloned in cloud sessions via the `add_repo` tool + `git clone --depth 1` (proxy serves public repos). Soldat's real assets: opensoldat/base (CC BY 4.0). `python3 tools/import_soldat.py <path-to-base>` imports SFX. Code stays original/MIT.

## Status log (newest first)
* 2026-10-09: Planning docs complete (PRD.md, PRD.json, task.md, docs/PLAN.md, docs/LORE.md, commands.txt, HANDOFF.md). Next: P1 skeleton + P2 asset generator.
