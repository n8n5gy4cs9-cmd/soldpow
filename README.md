# nordSoldPow

**Powder Toy x Soldat** in a Nordic / Viking / Finnish-Kalevala setting - modern guns, jetpacks and a falling-sand universe.
Fly a jetpack through the **Solar System renamed with Norse lore** (9 worlds) and a second **Kalevala star system** (5 worlds),
recover the 14 shards of the shattered **Sampo**, and reforge it at Ilmarinen's forge.

*By Crowelian 2026 + Sonnet.* Written in Rust with [macroquad](https://macroquad.rs). Primary target: **macOS Apple Silicon (M1 Max)**;
Windows and Linux build from the same source.

## Run it

```bash
# install Rust: https://rustup.rs
cargo run --release          # always --release: the powder simulation is slow in debug builds
```

Options: `--nosound`, `--planet midgard`, `--mode campaign|sandbox|versus`, `--split`, `--console "give all; god"`,
`--selftest` (headless smoke test), `--dump-docs` (regenerates `docs/MISSIONS.md` and `assets/data/elements.txt`).

### macOS app bundle (Apple Silicon)
```bash
tools/bundle_macos.sh        # -> dist/nordSoldPow.app  (+ dist/nordSoldPow-macos.zip)
```
### Windows / Linux
```bash
cargo build --release        # Windows: MSVC toolchain.  Linux: sudo apt install libasound2-dev libx11-dev libxi-dev libgl1-mesa-dev pkg-config
tools/package.sh             # copies the binary + assets/ + docs into dist/ and zips them
```
The executable looks for an `assets/` folder next to itself (or in the working directory, or `$NORDSOLDPOW_ASSETS`).

## Game modes
* **Campaign** (1P or 2P co-op splitscreen): 14 worlds, 5 missions each (3 main + 2 optional), bosses, hotspots, story.
* **Sandbox**: pick any world, everything unlocked, infinite ammo, build mode on - paint the universe.
* **Versus** (2P splitscreen): first to 15 frags, weapon crates spawn over time.

## Controls
| | Player 1 | Player 2 (splitscreen) |
|---|---|---|
| Move | A / D | Left / Right |
| Jump, then hold for jetpack | W or Space | Up |
| Crouch (slower) | S | Down |
| Aim | mouse | `,` up  `.` down |
| Fire / grenade | LMB / RMB | Enter / Right Shift |
| Reload / next / prev weapon | R / E / Q (or wheel, keys 1-8) | `/` / `'` / `;` |
| Interact (springs, chests, forge, gate) | F | `\` |

**Powder mode:** `B` build mode - LMB paint, RMB erase, wheel = brush size, `[` `]` or `1-0` = element, `Tab` = element picker.
`P` pause simulation, `O` step. Paint **Clone** over a material to duplicate it forever.
**Game keys:** `M` map, `J` journal, `Esc` pause, `F3` fps, `F2` cheat menu, `` ` `` or `F1` console.

## Cheats & console
Open the console with `` ` `` / `F1`: `give vainamoinen_gun`, `tp helheim`, `god`, `spawn draugr 5`, `paint lava 6`, `weather water`, ...
Full list: [`commands.txt`](commands.txt) (checked against the code by a unit test). `F2` opens a clickable cheat menu.

## Docs for contributors / other AIs
* [`HANDOFF.md`](HANDOFF.md) - **start here when resuming work**
* [`task.md`](task.md) / [`PRD.json`](PRD.json) - task status (source of truth is PRD.json; `python3 tools/task.py done <id>`)
* [`PRD.md`](PRD.md), [`docs/PLAN.md`](docs/PLAN.md) (architecture), [`docs/LORE.md`](docs/LORE.md) (story), [`docs/MISSIONS.md`](docs/MISSIONS.md)
* [`assets.md`](assets.md) - every sprite / sound / music file with its path, easy to replace

## Assets
All art and audio are **generated placeholders** (`python3 tools/gen_assets.py`, needs Pillow + numpy). Replace any file in `assets/`
(same path) and restart - nothing is compiled in. See `assets.md`.

## Licence & credits
Game code: MIT (see LICENSE). Inspired by *The Powder Toy* (GPL-3) and *Soldat* (MIT) - no code from either project is included.
Lore: Norse sagas and Elias Lonnrot's *Kalevala* (public domain).
