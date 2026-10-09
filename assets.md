# nordSoldPow - Assets

Most art and all music are **generated placeholders** (`tools/gen_assets.py`). **Sound effects marked `OpenSoldat base` are real Soldat sounds** (https://github.com/opensoldat/base, licence **CC BY 4.0** - attribution required, included here and in README/credits; converted to 16-bit 22.05 kHz mono). Soldat itself ships no music; The Powder Toy ships no sound or art assets (elements are just colours).
Everything is loaded **at runtime** from `assets/`; replace a file (same path, same name) and restart. Missing files never crash the game (fallback shape / silence).
Formats: sprites = PNG (RGBA, nearest-neighbour, 1 px = 1 world cell), audio = WAV or OGG (use the same file name; keep `.wav` unless you also edit `src/assets.rs` extension list - `.ogg` is tried first automatically).
Regenerate everything: `python3 tools/gen_assets.py` (overwrites!).  Machine list: `assets/manifest.json`.

## Quick list (use | name | path | source)

| Use | Name | Path | Source |
|---|---|---|---|
| Player 1 sheet (blue scarf) | player1 | `assets/sprites/player1.png` | generated placeholder (tools/gen_assets.py) |
| Player 2 sheet (red scarf, splitscreen) | player2 | `assets/sprites/player2.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: draugr | enemy_draugr | `assets/sprites/enemy_draugr.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: berserker | enemy_berserker | `assets/sprites/enemy_berserker.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: eldjotunn | enemy_eldjotunn | `assets/sprites/enemy_eldjotunn.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: frostjotunn | enemy_frostjotunn | `assets/sprites/enemy_frostjotunn.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: valkyrie | enemy_valkyrie | `assets/sprites/enemy_valkyrie.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: hiisi | enemy_hiisi | `assets/sprites/enemy_hiisi.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: kalma | enemy_kalma | `assets/sprites/enemy_kalma.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: louhi_guard | enemy_louhi_guard | `assets/sprites/enemy_louhi_guard.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: svartalf | enemy_svartalf | `assets/sprites/enemy_svartalf.png` | generated placeholder (tools/gen_assets.py) |
| Enemy: fenrir_pup (wolf) | enemy_fenrir_pup | `assets/sprites/enemy_fenrir_pup.png` | generated placeholder (tools/gen_assets.py) |
| Boss: fenrir | boss_fenrir | `assets/sprites/boss_fenrir.png` | generated placeholder (tools/gen_assets.py) |
| Boss: surtr | boss_surtr | `assets/sprites/boss_surtr.png` | generated placeholder (tools/gen_assets.py) |
| Boss: ymir | boss_ymir | `assets/sprites/boss_ymir.png` | generated placeholder (tools/gen_assets.py) |
| Boss: jormungandr | boss_jormungandr | `assets/sprites/boss_jormungandr.png` | generated placeholder (tools/gen_assets.py) |
| Boss: hel | boss_hel | `assets/sprites/boss_hel.png` | generated placeholder (tools/gen_assets.py) |
| Boss: louhi | boss_louhi | `assets/sprites/boss_louhi.png` | generated placeholder (tools/gen_assets.py) |
| Boss: tuoni | boss_tuoni | `assets/sprites/boss_tuoni.png` | generated placeholder (tools/gen_assets.py) |
| Boss: rauta_golem | boss_rauta_golem | `assets/sprites/boss_rauta_golem.png` | generated placeholder (tools/gen_assets.py) |
| Boss: ukko_eagle | boss_ukko_eagle | `assets/sprites/boss_ukko_eagle.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: puukko | puukko | `assets/weapons/puukko.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: hugin | hugin | `assets/weapons/hugin.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: munin | munin | `assets/weapons/munin.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: lemminkainen_smg | lemminkainen_smg | `assets/weapons/lemminkainen_smg.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: vainamoinen_gun | vainamoinen_gun | `assets/weapons/vainamoinen_gun.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: kullervo_shotgun | kullervo_shotgun | `assets/weapons/kullervo_shotgun.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: gungnir_sniper | gungnir_sniper | `assets/weapons/gungnir_sniper.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: mjolnir_rpg | mjolnir_rpg | `assets/weapons/mjolnir_rpg.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: gjallar_grenade | gjallar_grenade | `assets/weapons/gjallar_grenade.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: surtr_flame | surtr_flame | `assets/weapons/surtr_flame.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: louhi_frost | louhi_frost | `assets/weapons/louhi_frost.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: tuoni_acid | tuoni_acid | `assets/weapons/tuoni_acid.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: ukko_thunder | ukko_thunder | `assets/weapons/ukko_thunder.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: ilmarinen_rail | ilmarinen_rail | `assets/weapons/ilmarinen_rail.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: sampo_gun | sampo_gun | `assets/weapons/sampo_gun.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: powder_gun | powder_gun | `assets/weapons/powder_gun.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: fenrir_minigun | fenrir_minigun | `assets/weapons/fenrir_minigun.png` | generated placeholder (tools/gen_assets.py) |
| Weapon sprite: loki_trick | loki_trick | `assets/weapons/loki_trick.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: midgard | midgard | `assets/planets/midgard.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: mani | mani | `assets/planets/mani.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: muspell | muspell | `assets/planets/muspell.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: vanaheim | vanaheim | `assets/planets/vanaheim.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: jotunheim | jotunheim | `assets/planets/jotunheim.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: asgard | asgard | `assets/planets/asgard.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: niflheim | niflheim | `assets/planets/niflheim.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: svartalfheim | svartalfheim | `assets/planets/svartalfheim.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: helheim | helheim | `assets/planets/helheim.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: kalevala | kalevala | `assets/planets/kalevala.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: pohjola | pohjola | `assets/planets/pohjola.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: tuonela | tuonela | `assets/planets/tuonela.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: sampola | sampola | `assets/planets/sampola.png` | generated placeholder (tools/gen_assets.py) |
| Planet icon: ukkola | ukkola | `assets/planets/ukkola.png` | generated placeholder (tools/gen_assets.py) |
| Star Sol (Norse system centre) | sol | `assets/planets/sol.png` | generated placeholder (tools/gen_assets.py) |
| Star Paivatar (Kalevala system centre) | paivatar | `assets/planets/paivatar.png` | generated placeholder (tools/gen_assets.py) |
| Bifrost / Aurora gate on the map | gate | `assets/planets/gate.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: midgard | midgard | `assets/bg/midgard.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: mani | mani | `assets/bg/mani.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: muspell | muspell | `assets/bg/muspell.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: vanaheim | vanaheim | `assets/bg/vanaheim.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: jotunheim | jotunheim | `assets/bg/jotunheim.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: asgard | asgard | `assets/bg/asgard.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: niflheim | niflheim | `assets/bg/niflheim.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: svartalfheim | svartalfheim | `assets/bg/svartalfheim.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: helheim | helheim | `assets/bg/helheim.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: kalevala | kalevala | `assets/bg/kalevala.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: pohjola | pohjola | `assets/bg/pohjola.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: tuonela | tuonela | `assets/bg/tuonela.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: sampola | sampola | `assets/bg/sampola.png` | generated placeholder (tools/gen_assets.py) |
| Parallax background silhouette: ukkola | ukkola | `assets/bg/ukkola.png` | generated placeholder (tools/gen_assets.py) |
| Title logo | logo | `assets/ui/logo.png` | generated placeholder (tools/gen_assets.py) |
| Crosshair cursor | crosshair | `assets/ui/crosshair.png` | generated placeholder (tools/gen_assets.py) |
| Pickup: health | pickup_health | `assets/ui/pickup_health.png` | generated placeholder (tools/gen_assets.py) |
| Pickup: ammo | pickup_ammo | `assets/ui/pickup_ammo.png` | generated placeholder (tools/gen_assets.py) |
| Pickup: shard | pickup_shard | `assets/ui/pickup_shard.png` | generated placeholder (tools/gen_assets.py) |
| Pickup: silver | pickup_silver | `assets/ui/pickup_silver.png` | generated placeholder (tools/gen_assets.py) |
| Pickup: weapon | pickup_weapon | `assets/ui/pickup_weapon.png` | generated placeholder (tools/gen_assets.py) |
| HUD icon: grenade | icon_grenade | `assets/ui/icon_grenade.png` | generated placeholder (tools/gen_assets.py) |
| HUD icon: fuel | icon_fuel | `assets/ui/icon_fuel.png` | generated placeholder (tools/gen_assets.py) |
| Hotspot marker: spring | hotspot_spring | `assets/ui/hotspot_spring.png` | generated placeholder (tools/gen_assets.py) |
| Hotspot marker: rune | hotspot_rune | `assets/ui/hotspot_rune.png` | generated placeholder (tools/gen_assets.py) |
| Hotspot marker: chest | hotspot_chest | `assets/ui/hotspot_chest.png` | generated placeholder (tools/gen_assets.py) |
| Hotspot marker: forge | hotspot_forge | `assets/ui/hotspot_forge.png` | generated placeholder (tools/gen_assets.py) |
| Hotspot marker: gate | hotspot_gate | `assets/ui/hotspot_gate.png` | generated placeholder (tools/gen_assets.py) |
| Hotspot marker: sauna | hotspot_sauna | `assets/ui/hotspot_sauna.png` | generated placeholder (tools/gen_assets.py) |
| Hotspot marker: bunker | hotspot_bunker | `assets/ui/hotspot_bunker.png` | generated placeholder (tools/gen_assets.py) |
| Projectile sprite: rocket | proj_rocket | `assets/ui/proj_rocket.png` | generated placeholder (tools/gen_assets.py) |
| Projectile sprite: grenade | proj_grenade | `assets/ui/proj_grenade.png` | generated placeholder (tools/gen_assets.py) |
| Projectile sprite: bullet | proj_bullet | `assets/ui/proj_bullet.png` | generated placeholder (tools/gen_assets.py) |
| Projectile sprite: particle | proj_particle | `assets/ui/proj_particle.png` | generated placeholder (tools/gen_assets.py) |
| SFX: shot_pistol | shot_pistol | `assets/sfx/shot_pistol.wav` | OpenSoldat base: shared/sfx/deserteagle-fire.wav (CC BY 4.0), converted |
| SFX: shot_revolver | shot_revolver | `assets/sfx/shot_revolver.wav` | OpenSoldat base: shared/sfx/colt1911-fire.wav (CC BY 4.0), converted |
| SFX: shot_smg | shot_smg | `assets/sfx/shot_smg.wav` | OpenSoldat base: shared/sfx/mp5-fire.wav (CC BY 4.0), converted |
| SFX: shot_rifle | shot_rifle | `assets/sfx/shot_rifle.wav` | OpenSoldat base: shared/sfx/ak74-fire.wav (CC BY 4.0), converted |
| SFX: shot_shotgun | shot_shotgun | `assets/sfx/shot_shotgun.wav` | OpenSoldat base: shared/sfx/spas12-fire.wav (CC BY 4.0), converted |
| SFX: shot_sniper | shot_sniper | `assets/sfx/shot_sniper.wav` | OpenSoldat base: shared/sfx/barretm82-fire.wav (CC BY 4.0), converted |
| SFX: shot_minigun | shot_minigun | `assets/sfx/shot_minigun.wav` | OpenSoldat base: shared/sfx/minigun-fire.wav (CC BY 4.0), converted |
| SFX: shot_flame | shot_flame | `assets/sfx/shot_flame.wav` | OpenSoldat base: shared/sfx/flamer.wav (CC BY 4.0), converted |
| SFX: shot_ice | shot_ice | `assets/sfx/shot_ice.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: shot_acid | shot_acid | `assets/sfx/shot_acid.wav` | OpenSoldat base: shared/sfx/spit.wav (CC BY 4.0), converted |
| SFX: shot_thunder | shot_thunder | `assets/sfx/shot_thunder.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: shot_rail | shot_rail | `assets/sfx/shot_rail.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: shot_rocket | shot_rocket | `assets/sfx/shot_rocket.wav` | OpenSoldat base: shared/sfx/law.wav (CC BY 4.0), converted |
| SFX: shot_grenade | shot_grenade | `assets/sfx/shot_grenade.wav` | OpenSoldat base: shared/sfx/m79-fire.wav (CC BY 4.0), converted |
| SFX: shot_powder | shot_powder | `assets/sfx/shot_powder.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: shot_trick | shot_trick | `assets/sfx/shot_trick.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: melee | melee | `assets/sfx/melee.wav` | OpenSoldat base: shared/sfx/knife.wav (CC BY 4.0), converted |
| SFX: explosion | explosion | `assets/sfx/explosion.wav` | OpenSoldat base: shared/sfx/grenade-explosion.wav (CC BY 4.0), converted |
| SFX: explosion_big | explosion_big | `assets/sfx/explosion_big.wav` | OpenSoldat base: shared/sfx/cluster-explosion.wav (CC BY 4.0), converted |
| SFX: hit_flesh | hit_flesh | `assets/sfx/hit_flesh.wav` | OpenSoldat base: shared/sfx/bonecrack.wav (CC BY 4.0), converted |
| SFX: hit_wall | hit_wall | `assets/sfx/hit_wall.wav` | OpenSoldat base: shared/sfx/ric3.wav (CC BY 4.0), converted |
| SFX: hurt | hurt | `assets/sfx/hurt.wav` | OpenSoldat base: shared/sfx/hit-arg2.wav (CC BY 4.0), converted |
| SFX: die | die | `assets/sfx/die.wav` | OpenSoldat base: shared/sfx/death.wav (CC BY 4.0), converted |
| SFX: jump | jump | `assets/sfx/jump.wav` | OpenSoldat base: shared/sfx/jump.wav (CC BY 4.0), converted |
| SFX: land | land | `assets/sfx/land.wav` | OpenSoldat base: shared/sfx/fall.wav (CC BY 4.0), converted |
| SFX: step | step | `assets/sfx/step.wav` | OpenSoldat base: shared/sfx/step.wav (CC BY 4.0), converted |
| SFX: jet | jet | `assets/sfx/jet.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: pickup | pickup | `assets/sfx/pickup.wav` | OpenSoldat base: shared/sfx/takegun.wav (CC BY 4.0), converted |
| SFX: pickup_weapon | pickup_weapon | `assets/sfx/pickup_weapon.wav` | OpenSoldat base: shared/sfx/pickupgun.wav (CC BY 4.0), converted |
| SFX: reload | reload | `assets/sfx/reload.wav` | OpenSoldat base: shared/sfx/mp5-reload.wav (CC BY 4.0), converted |
| SFX: empty | empty | `assets/sfx/empty.wav` | OpenSoldat base: shared/sfx/minigun-empty.wav (CC BY 4.0), converted |
| SFX: mission_done | mission_done | `assets/sfx/mission_done.wav` | OpenSoldat base: shared/sfx/capture.wav (CC BY 4.0), converted |
| SFX: ui_click | ui_click | `assets/sfx/ui_click.wav` | OpenSoldat base: shared/sfx/menuclick.wav (CC BY 4.0), converted |
| SFX: ui_open | ui_open | `assets/sfx/ui_open.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: splash | splash | `assets/sfx/splash.wav` | OpenSoldat base: shared/sfx/bryzg.wav (CC BY 4.0), converted |
| SFX: sizzle | sizzle | `assets/sfx/sizzle.wav` | OpenSoldat base: shared/sfx/burn.wav (CC BY 4.0), converted |
| SFX: boss_roar | boss_roar | `assets/sfx/boss_roar.wav` | OpenSoldat base: shared/sfx/roar.wav (CC BY 4.0), converted |
| SFX: thunder_strike | thunder_strike | `assets/sfx/thunder_strike.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: gate | gate | `assets/sfx/gate.wav` | generated placeholder (tools/gen_assets.py) |
| SFX: heal | heal | `assets/sfx/heal.wav` | OpenSoldat base: shared/sfx/regenerate.wav (CC BY 4.0), converted |
| Music loop: menu | menu | `assets/music/menu.wav` | generated placeholder (tools/gen_assets.py) |
| Music loop: map | map | `assets/music/map.wav` | generated placeholder (tools/gen_assets.py) |
| Music loop: calm | calm | `assets/music/calm.wav` | generated placeholder (tools/gen_assets.py) |
| Music loop: norse | norse | `assets/music/norse.wav` | generated placeholder (tools/gen_assets.py) |
| Music loop: kalevala | kalevala | `assets/music/kalevala.wav` | generated placeholder (tools/gen_assets.py) |
| Music loop: dark | dark | `assets/music/dark.wav` | generated placeholder (tools/gen_assets.py) |
| Music loop: boss | boss | `assets/music/boss.wav` | generated placeholder (tools/gen_assets.py) |
| Element colour overrides (optional) | elements | `assets/data/elements.txt` | generated by `nordsoldpow --dump-docs` |

## Details & how to replace

### Sprites (`assets/sprites/`)
* `player1.png`, `player2.png`, `enemy_<kind>.png` (9 humanoids): sheet of **8 frames, each 16x24**, left->right: idle, run1, run2, run3, run4, jump, fall, dead. Character faces **right**; the game mirrors it. Feet touch bottom row-1. Arms are baked in; the weapon sprite is drawn on top rotated around the shoulder (x=8,y=10).
* `enemy_fenrir_pup.png`: 4 frames of 24x16 (idle, run1, run2, dead).
* `boss_<name>.png`: one 48x48 frame, faces right. Names: fenrir, surtr, ymir, jormungandr, hel, louhi, tuoni, rauta_golem, ukko_eagle.
### Weapons (`assets/weapons/<weapon_key>.png`)
* 24x12, barrel points right, grip/hand pivot at about (5,7). Key = console name (e.g. `vainamoinen_gun`).
### Planets / map (`assets/planets/`)
* 64x64 planet icons, `sol.png`/`paivatar.png` 96x96 stars, `gate.png` 48x48.
### Backgrounds (`assets/bg/<planet>.png`)
* 320x180 transparent parallax silhouette (drawn twice, scrolling at different speeds). Sky gradient + stars are drawn by code using colours in `src/planets.rs`.
### UI (`assets/ui/`)
* logo, crosshair, pickups (12x12), HUD icons, hotspot markers (16x16), projectile sprites.
### Audio
* `assets/sfx/<event>.wav` - weapon entries in `src/weapons.rs` reference these names (`sfx: "shot_rifle"`).
* `assets/music/<track>.wav` - tracks: menu, map, calm, norse, kalevala, dark, boss. Planet -> track mapping is in `src/planets.rs` (`music` field). Must loop cleanly.
### Optional data
* `assets/data/elements.txt` (generated by `cargo run --release -- --dump-docs`): optional colour override for powder elements, one line per element: `key r g b`.
### Fonts
* The default built-in font is ASCII-only. To use a custom font put `assets/fonts/ui.ttf` (optional task 13.6).
### Where to get real assets
* Soldat (MIT code, assets under their own terms): https://github.com/soldat/soldat  - check licence before copying.
* The Powder Toy has almost no art assets (elements are colours), see https://github.com/The-Powder-Toy/The-Powder-Toy.
* Free options: OpenGameArt.org, Kenney.nl, freesound.org (check licences).
