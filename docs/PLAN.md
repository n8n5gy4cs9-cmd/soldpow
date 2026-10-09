# nordSoldPow - Technical Plan

## Stack
Rust + macroquad 0.4 (OpenGL via miniquad, Metal-friendly on macOS; audio via quad-snd/CoreAudio).
Only dependency: `macroquad`. Python 3 (Pillow + numpy) used only in `tools/` to generate placeholder assets.

## Module map (src/)
| File | Responsibility |
|---|---|
| `main.rs` | window conf, app state machine, main loop, CLI flags (`--selftest`, `--sandbox`, `--planet X`) |
| `elements.rs` | `El` enum (u8), table: name/key/colour/density/category/flags |
| `sim.rs` | `World { w,h,cells }`, per-cell update rules, reactions, explosions, painting, helpers (`is_solid`, `ray`) |
| `terrain.rs` | value-noise, planet terrain generator, structures, hotspot/spawn point output |
| `planets.rs` | `PlanetDef` table (14), systems, hazards, palettes, enemy rosters |
| `entity.rs` | `Body` physics (AABB vs grid), jetpack, damage, fire/acid/lava contact |
| `player.rs` | player state, input mapping (P1/P2), inventory (3 slots) |
| `weapons.rs` | `WeaponDef` table + `weapon_by_key` |
| `bullet.rs` | projectiles (bullet, pellet, flame, rocket, grenade, element-shot), impact effects |
| `enemy.rs` | enemy kinds + AI, spawners, waves |
| `boss.rs` | boss behaviours |
| `pickup.rs` | ammo/health/shard/silver/weapon pickups |
| `hotspot.rs` | interactive map points |
| `missions.rs` | mission defs, objective tracking, rewards |
| `story.rs` | dialogue scenes, intro/ending text |
| `map.rs` | solar-system map screens + travel |
| `render.rs` | cameras, world texture, parallax, sprite drawing, splitscreen viewports |
| `ui.rs` | HUD, element picker, journal, cheat menu |
| `menu.rs` | main/pause/settings/credits |
| `console.rs` | dev console, command registry, cheats |
| `audio.rs` | sfx + music |
| `assets.rs` | runtime asset loading with fallbacks |
| `save.rs` | save/load progress & settings (plain `key=value` text, no deps) |
| `input.rs` | input abstraction per player |

## Simulation design
* `Cell { el: u8, life: u8, aux: u8, var: u8 }` (4 bytes). `var` = random colour shade. `aux` bit7 = "updated this frame" parity.
* Iterate rows bottom-to-top (so falling is stable), alternating x direction each frame. Gases update top-to-bottom (rise).
* A frame parity bit prevents a moved cell from being processed twice.
* Rendering: world -> `Image` (RGBA) -> `Texture2D` (nearest filter) once per frame; each viewport draws a sub-rect.
* Explosions: `World::explode(x,y,r,power)` clears cells in radius, sets fire/smoke, ignites neighbours, damages entities (via event list drained by game).
* Entities query `World::solid_for_body(x,y)` (solid + powder count as solid; liquids no).

## Entities & combat
* 1 sprite px == 1 world cell. Hitbox player 6x20 inside 16x24 sprite. Default zoom 3 (2 in splitscreen).
* Bullets are advanced in sub-steps (<=1 cell each) to avoid tunnelling; hit test vs creatures AABB and grid.
* Terrain damage by bullet depends on `hardness` of element (sand/dirt: always, stone: chance, metal/wall: never).
* Gravity per planet (scale), applies to bodies, bullets (slight), grenades. Sim gravity always down.

## Worlds
`terrain::generate(planet, w, h) -> Generated { cells, spawn_points, hotspots, structures }`.
Fixed seed per planet. Layers: bedrock WALL border, surface noise, stone core, caves (2-D noise), ores, planet biome
fill (see planets.rs), liquid pools, structures (huts, ruins, forge, basin).

## Missions
`Mission { id, planet, title, brief, kind, objective, reward, optional }`. Objective enum drives checks each frame:
`Kill(n) | Collect(n) | Reach(zone) | ZoneCount{el,zone,min|max} | Survive(sec) | Boss | Interact(kind)`.
Progress is stored in `Save { completed: Vec<id>, silver, unlocked_weapons, systems_unlocked }`.

## Console
Single registry `COMMANDS: &[CmdDef { name, args, help }]`. `commands.txt` generated from it by `cargo run -- --dump-commands`
and verified by a unit test.

## Milestones
M0 Plan docs -> M1 skeleton+assets -> M2 sim -> M3 player/weapons -> M4 worlds/enemies -> M5 missions/story/map ->
M6 console/cheats/audio -> M7 splitscreen/UI polish -> M8 tests/zip/push.

## Cross-platform notes
No platform APIs. macOS: `tools/bundle_macos.sh` builds .app (arm64). Windows: `cargo build --release` (MSVC).
Linux: needs libasound2-dev, libx11/libgl dev packages for build of audio/window crates.

## Risk / fallback
* No GitHub access to clone references -> generated assets (documented).
* Sandbox has no display -> logic is unit-testable; `--selftest` runs headless sim; rendering verified by eye on Mac.
