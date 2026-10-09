# nordSoldPow - Product Requirements Document

**Author:** Crowelian 2026 + Sonnet  **Language:** Rust  **Engine:** macroquad 0.4
**Primary target:** macOS Apple Silicon (M1 Max). Then Windows, Linux (keep code portable; no platform-specific APIs).

> Machine-readable task list: `PRD.json` (source of truth). Human list: `task.md`. Resume info: `HANDOFF.md`.

## 1. Pitch
A 2D side-view run-and-gun (Soldat-style: jetpack, mouse aim, modern guns, fast deathmatch feel) played **inside a
falling-sand physics world** (The Powder Toy-style: sand, water, lava, fire, acid, gunpowder, plants...).
Every bullet, grenade and explosion interacts with the powder world; the player can also *paint* elements.
The setting is **Nordic / Viking / Finnish-Kalevala myth in the modern day, with modern guns**, played across
**our Solar System renamed with Norse lore** plus a **second "Kalevala" star system with 5 planets**.
Art direction: chunky pixel art reminiscent of *Starbound*.

## 2. Reference projects (inspiration, NOT code copies)
* The Powder Toy (https://github.com/The-Powder-Toy/The-Powder-Toy) - GPL-3. Element behaviours re-implemented from scratch.
* Soldat (https://github.com/soldat/soldat) - weapon feel/jetpack/aiming re-implemented from scratch.
* **Cloned for reference** (shallow clones, read-only, kept outside this repo): `soldat/soldat` (MIT, Pascal source only),
  `The-Powder-Toy/The-Powder-Toy` (GPL-3, C++; no sound/art assets), and `opensoldat/base` (Soldat's real game content, CC BY 4.0).
  * **Code is original** (no GPL code copied, so this project stays MIT). Powder Toy / Soldat were studied for element lists, colours and weapon stats.
  * **Sound effects**: 31 real Soldat SFX imported from `opensoldat/base` (CC BY 4.0, attribution in assets.md / README) by `tools/import_soldat.py`.
  * **Music**: neither reference ships music -> generated placeholder loops. **Sprites**: generated Starbound-style pixel art (as requested).

## 3. Goals
1. Fully playable single-player campaign across 14 worlds with missions, optional tasks, bosses.
2. Powder sandbox physics with >= 30 elements and reactions that matter for combat and missions.
3. Local **splitscreen** 2-player (co-op campaign + versus).
4. Developer console + cheat menu with memorable commands (`give vainamoinen_gun`) - listed in `commands.txt`.
5. All art/audio are **placeholders that are trivially replaceable** (`assets.md`, runtime-loaded files, no recompilation).
6. Any AI/human can resume: `HANDOFF.md`, `task.md`, `PRD.json`.

## 4. Non-goals (for v0.1)
Online multiplayer, mod workshop, mobile, controller (optional tasks exist for these).

## 5. Core features
### 5.1 Powder world
Grid of cells (1 cell = 1 world pixel). Categories: powder, liquid, gas, solid, special.
Elements (console keys): `sand dirt salt snow ash gunpowder coal | water oil lava acid nitro blood mercury |
smoke steam methane fire | wall stone wood metal glass ice plant tnt gold rune brick | clone void thunder`.
Reactions: fire burns flammables, water+lava -> stone+steam, acid dissolves, ice/snow melt near heat, plants grow
in water, sand+lava -> glass, nitro/tnt/gunpowder explode and chain. Pause/step/speed controls.
### 5.2 Soldat gameplay
WASD move, W/Space = jump then hold for **jetpack** (fuel), mouse aim, LMB fire, RMB grenade, R reload,
Q/E or wheel cycle weapons (carry 3 slots), F pick up / interact, B toggle **build mode** (paint elements with mouse).
Health, regen via springs/medkits, respawn. Bullets craters terrain; flame/acid/ice guns spawn elements.
### 5.3 Worlds
9 Norse worlds (Midgard, Mani, Muspell, Vanaheim, Jotunheim, Asgard, Niflheim, Svartalfheim, Helheim) around star **Sol**;
5 Kalevala worlds (Kalevala, Pohjola, Tuonela, Sampola, Ukkola) around star **Paivatar**. Procedurally generated
terrain from a fixed seed per world, with per-world gravity, palette, hazards, enemy roster, hotspots, missions.
### 5.4 Missions
Objective types: Kill, Collect (Sampo shards), Reach zone, ZoneCount (fill basin with water / burn the nest = powder
puzzles), Survive waves, Boss, Interact. 3 main + 2 optional per world. Rewards: silver, weapons, unlocks.
### 5.5 Hotspots
Healing springs, rune stones (lore), weapon chests, forges (refill + buy), Aurora Gate (Bifrost between systems), hidden bunkers.
### 5.6 Cheats / console
Backtick or F1 = console, F2 = cheat menu. All commands documented in `commands.txt`; a test asserts docs == code.
### 5.7 Splitscreen
P1 keyboard+mouse, P2 keyboard-only (arrows etc.), shared world, two viewports. Modes: Co-op campaign, Versus.
### 5.8 Assets
`assets/` loaded at runtime; missing file => procedural fallback (colored rect / silence). `assets.md` has a clean top
table (use, name, path, source) followed by details (size, format, how to replace).

## 6. Controls (default)
| Action | P1 | P2 (splitscreen) |
|---|---|---|
| Move | A / D | Left / Right |
| Jump / jetpack | W or Space | Up |
| Crouch | S | Down |
| Aim | Mouse | `,` up / `.` down (aims along facing) |
| Fire | LMB | Enter |
| Grenade | RMB | Right Shift |
| Reload | R | `/` |
| Next / prev weapon | E / Q, wheel | `'` / `;` |
| Interact | F | `\` |
| Build mode (paint) | B | - |
| Element select | `[` `]`, 1-0 hotkeys, Tab picker | - |
| Brush size | mouse wheel in build mode | - |
| Console / cheat menu | `` ` `` or F1 / F2 | |
| Pause | Esc | |
| Sim pause / step | P / O | |
| Journal | J | |
| Map | M | |

## 7. Technical constraints
Rust stable. Dependencies kept minimal (macroquad only). 60 FPS on M1 Max with a 1280x640 cell world in `--release`.
`cargo run --release`. No unsafe. Deterministic terrain per seed. Sim uses macroquad's quad-rand.

## 8. Acceptance (v0.1)
* `cargo build --release` clean on macOS arm64; `cargo test` passes.
* Menu -> map -> land on any unlocked world -> play -> complete missions -> unlock more worlds -> reach both systems.
* Console command list in commands.txt all work. Splitscreen toggle works in-game.
* Replace any file in `assets/` and it is picked up on restart.
