# nordSoldPow - TASKS

_Generated from PRD.json by `python3 tools/task.py sync` - do not hand-edit; use `tools/task.py done <id>`._

**Progress: 80/99 total | 79/84 required | updated 2026-10-09**

Legend: `[x]` done, `[~]` in progress, `[ ]` to do, `[!]` blocked, *(opt)* optional.

**Continue here: see HANDOFF.md**

## NEXT UP

- `6.4` Spawners / waves (survive missions)
- `7.6` Optional tasks tracker / journal screen
- `13.1` Unit tests (sim rules, console parser, mission engine)
- `13.2` Playtest headless smoke test (sim + frames)
- `13.4` Final zip of repo + commit + push

## P0 Planning & docs  (8/8)

- [x] `0.1` PRD.md written (product requirements)
- [x] `0.2` PRD.json written (machine-readable source of truth for tasks)
- [x] `0.3` task.md + tools/task.py (sync task.md from PRD.json)
- [x] `0.4` docs/PLAN.md architecture + milestones
- [x] `0.5` docs/LORE.md story, factions, planets, missions
- [x] `0.6` assets.md (simple list at top + detail below)
- [x] `0.7` commands.txt (console commands list)
- [x] `0.8` HANDOFF.md ("continue here" for next AI / next session)

## P1 Project foundation  (6/6)

- [x] `1.1` Cargo project (macroquad), release profile tuned for M1 Max
- [x] `1.2` Game state machine: Boot / Menu / SolarMap / Play / Pause / Story
- [x] `1.3` Asset loader with graceful fallbacks (missing file => placeholder, never crash)
- [x] `1.4` Settings + save file (progress, unlocks, silver, completed missions)
- [x] `1.5` macOS .app bundle script (tools/bundle_macos.sh) + Win/Linux build notes
- [x] `1.6` GitHub Actions CI building mac/win/linux *(opt)*

## P2 Asset generation (all placeholders, replaceable)  (10/10)

- [x] `2.1` Python pixel-art toolkit tools/gen_assets.py
- [x] `2.2` Player + enemy sprite sheets (Starbound-ish pixel style)
- [x] `2.3` Boss sprites (9 bosses)
- [x] `2.4` Weapon sprites (all guns)
- [x] `2.5` Planet icons (14 planets + 2 stars + gate)
- [x] `2.6` Parallax backgrounds per planet
- [x] `2.7` UI: logo, icons, pickups, hotspot markers, crosshair
- [x] `2.8` Placeholder SFX (synthesised wav)
- [x] `2.9` Placeholder music (synthesised kantele-style loops)
- [x] `2.10` assets.md auto-generated from manifest (tools/gen_assets.py)

## P3 Powder sim (Powder Toy side)  (12/15)

- [x] `3.1` Element table (id, name, console key, colour, density, flags)
- [x] `3.2` Grid world + double-buffer-free update with parity flag
- [x] `3.3` Powders: sand, dirt, salt, snow, ash, gunpowder, coal
- [x] `3.4` Liquids: water, oil, lava, acid, nitro, blood, mercury
- [x] `3.5` Gases + fire: smoke, steam, methane, fire
- [x] `3.6` Solids: wall, stone, wood, metal, glass, ice, plant, tnt, gold, rune
- [x] `3.7` Special: clone, void, spark/thunder
- [x] `3.8` Reactions (fire burns, water+lava, acid dissolves, ice melts, plant grows, sand+lava=glass)
- [x] `3.9` Explosions (radius, chain reactions, terrain throw)
- [x] `3.10` Brush painting tool (sizes, erase, line interpolation) + build mode
- [x] `3.11` Element picker UI (categories, hotkeys)
- [x] `3.12` Sim pause / step / speed
- [ ] `3.13` Sandbox save/load of world (png/bin) *(opt)*
- [ ] `3.14` Temperature simulation (Powder Toy heat model) *(opt)*
- [ ] `3.15` Electricity (spark/metal/battery) *(opt)*

## P4 Soldat gameplay  (9/11)

- [x] `4.1` Creature physics vs grid (walk, step-up, jump, swim, fall damage)
- [x] `4.2` Jetpack with fuel + particles
- [x] `4.3` Mouse aim, weapon rotation, flip
- [x] `4.4` Weapon table (16+ weapons incl vainamoinen_gun)
- [x] `4.5` Bullets: hitscan-step, tracer, terrain crater, ignite/freeze/acid effects
- [x] `4.6` Grenades + rocket + explosions that feed the powder sim
- [x] `4.7` Pickups: ammo, medkit, weapon crates, sampo shards, silver
- [x] `4.8` Health, death, respawn, blood particles
- [x] `4.9` Powder gun (shoots selected element) + sampo_gun (random element)
- [ ] `4.10` Crouch / prone *(opt)*
- [ ] `4.11` Ragdoll death *(opt)*

## P5 Worlds  (6/6)

- [x] `5.1` Noise + terrain generator (surface, caves, ores, pools, bedrock)
- [x] `5.2` Planet data: 9 Norse + 5 Kalevala planets (gravity, palette, hazards, enemies)
- [x] `5.3` Environment hazards (ember rain, acid rain, snow, lightning, regolith dust)
- [x] `5.4` Structures (huts, ruins, forges, basins) + hotspot placement
- [x] `5.5` Parallax background renderer
- [x] `5.6` Camera (zoom, follow, clamp) per viewport

## P6 Enemies  (3/4)

- [x] `6.1` Enemy AI (patrol, chase, shoot, melee, flying)
- [x] `6.2` 10 enemy kinds
- [x] `6.3` 9 bosses with patterns
- [ ] `6.4` Spawners / waves (survive missions)

## P7 Story + missions  (5/6)

- [x] `7.1` Mission engine (Kill, Collect, Reach, ZoneCount, Survive, Boss, Interact)
- [x] `7.2` Mission data: 4-5 per planet (main + optional)
- [x] `7.3` Dialogue box + story scenes (intro, act breaks, ending)
- [x] `7.4` Hotspots (spring, rune stone, chest, forge, gate) interaction
- [x] `7.5` Rewards + unlocks + planet progression
- [ ] `7.6` Optional tasks tracker / journal screen

## P8 Solar maps  (3/4)

- [x] `8.1` Norse solar map screen (Sol + 9 worlds)
- [x] `8.2` Kalevala system map (5 planets)
- [x] `8.3` Aurora Gate / Bifrost travel between systems
- [ ] `8.4` Landing / travel transition *(opt)*

## P9 Cheats + console  (4/4)

- [x] `9.1` Dev console UI (` or F1), history, tab-complete
- [x] `9.2` Command set (give, tp, spawn, god, paint, ...) matching commands.txt
- [x] `9.3` Cheat menu (F2) clickable toggles
- [x] `9.4` commands.txt kept in sync with code (self-test)

## P10 Audio  (2/2)

- [x] `10.1` SFX manager with per-event mapping + volume
- [x] `10.2` Music per scene/planet with crossfade-less loop

## P11 Splitscreen  (3/4)

- [x] `11.1` Two viewports sharing one world
- [x] `11.2` P2 keyboard controls
- [x] `11.3` Co-op campaign + versus deathmatch
- [ ] `11.4` Gamepad support (gilrs) *(opt)*

## P12 UI / menus  (4/5)

- [x] `12.1` Main menu (campaign, sandbox, versus, settings, quit)
- [x] `12.2` HUD (hp, fuel, ammo, weapon, mission tracker, element)
- [x] `12.3` Pause menu + settings (volume, zoom, keys)
- [ ] `12.4` Minimap *(opt)*
- [x] `12.5` Credits screen: by Crowelian 2026 + Sonnet

## P13 Polish + ship  (1/8)

- [ ] `13.1` Unit tests (sim rules, console parser, mission engine)
- [ ] `13.2` Playtest headless smoke test (sim + frames)
- [x] `13.3` README with controls + build instructions
- [ ] `13.4` Final zip of repo + commit + push
- [ ] `13.5` Windows + Linux verified builds *(opt)*
- [ ] `13.6` Finnish/Norse font + localisation *(opt)*
- [ ] `13.7` Online multiplayer *(opt)*
- [ ] `13.8` Mod loader (assets/mods) *(opt)*

## P14 Reference-repo integration (Soldat / Powder Toy)  (4/6)

- [x] `14.1` Clone references; import 31 real Soldat SFX (CC BY 4.0) with provenance
- [x] `14.2` Soldat .pms map importer + 99 playable arenas + arena picker + `arena` command
- [x] `14.3` 24 extra Powder Toy-style elements (life, electricity, fireworks, bombs, sponge, wax, ln2, co2, seeds...) + tests
- [x] `14.4` build.sh / run.sh + docs/MAC_BUILD_FIX.md AI troubleshooting guide
- [ ] `14.5` Bots (AI opponents) in Soldat arenas *(opt)*
- [ ] `14.6` Port Soldat font (Play, OFL) for UI + proper diacritics *(opt)*
