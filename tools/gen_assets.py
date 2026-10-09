#!/usr/bin/env python3
"""Generate ALL placeholder assets for nordSoldPow + manifest + assets.md.
   python3 tools/gen_assets.py [--only sprites|audio]
Requires: pillow, numpy.  Output: assets/**  assets/manifest.json  assets.md
To replace an asset: overwrite the file at the same path (same size/format).  The game loads from disk at startup.
Running this script again would overwrite your replacements!  Use --only or delete entries from MANIFEST if you want to keep custom files."""
import os, sys, json, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import gen_sprites as S
ROOT = pathlib.Path(__file__).resolve().parent.parent
A = ROOT / "assets"
M = []   # manifest rows

def add(use, path, kind, size, note=""):
    M.append(dict(use=use, name=pathlib.Path(path).stem, path="assets/" + path, kind=kind, size=size, source="generated placeholder (tools/gen_assets.py)", note=note))

def save_img(img, rel, use, kind="sprite", note=""):
    p = A / rel; p.parent.mkdir(parents=True, exist_ok=True); img.save(p); add(use, rel, kind, f"{img.size[0]}x{img.size[1]} PNG", note)

def sprites():
    save_img(S.human_sheet(S.PLAYER1), "sprites/player1.png", "Player 1 sheet (blue scarf)", note="8 frames of 16x24: idle, run1-4, jump, fall, dead. Faces right (code mirrors).")
    save_img(S.human_sheet(S.PLAYER2), "sprites/player2.png", "Player 2 sheet (red scarf, splitscreen)", note="same layout as player1")
    for k, P in S.ENEMIES.items():
        save_img(S.human_sheet(P), f"sprites/enemy_{k}.png", f"Enemy: {k}", note="8 frames 16x24 (same layout as player)")
    save_img(S.wolf_sheet(), "sprites/enemy_fenrir_pup.png", "Enemy: fenrir_pup (wolf)", note="4 frames of 24x16: idle, run1, run2, dead")
    for k, fn in S.BOSSES.items():
        save_img(S.boss_img(fn), f"sprites/boss_{k}.png", f"Boss: {k}", note="single 48x48 frame, faces right")
    for k in S.WEAPON_KEYS:
        save_img(S.weapon_img(k), f"weapons/{k}.png", f"Weapon sprite: {k}", "weapon", "24x12, barrel points +x, grip pivot ~(5,7)")
    for k, (b, a, kind) in S.PLANETS.items():
        save_img(S.planet_img(k, b, a, kind), f"planets/{k}.png", f"Planet icon: {k}", "map")
    save_img(S.star_img('sol', (255, 250, 200), (255, 170, 40)), "planets/sol.png", "Star Sol (Norse system centre)", "map")
    save_img(S.star_img('paivatar', (240, 255, 255), (120, 200, 255)), "planets/paivatar.png", "Star Paivatar (Kalevala system centre)", "map")
    gate = S.canvas(48, 48); from PIL import ImageDraw; d = ImageDraw.Draw(gate)
    for i, c in enumerate(((255, 80, 120), (255, 220, 80), (80, 220, 255), (150, 100, 255))): d.arc([2 + i * 3, 2 + i * 3, 45 - i * 3, 60 - i * 3], 180, 360, fill=c + (255,), width=3)
    save_img(gate, "planets/gate.png", "Bifrost / Aurora gate on the map", "map")
    for k, (style, col) in S.BGS.items():
        save_img(S.bg_img(k, style, col), f"bg/{k}.png", f"Parallax background silhouette: {k}", "background", "320x180 transparent; sky colours come from planets.rs")
    save_img(S.logo_img(), "ui/logo.png", "Title logo", "ui")
    save_img(S.crosshair(), "ui/crosshair.png", "Crosshair cursor", "ui", "16x16 centre-aligned")
    for n in ("health", "ammo", "shard", "silver", "weapon", "grenade", "fuel"):
        save_img(S.icon(n), f"ui/pickup_{n}.png" if n in ("health", "ammo", "shard", "silver", "weapon") else f"ui/icon_{n}.png", f"{'Pickup' if n in ('health','ammo','shard','silver','weapon') else 'HUD icon'}: {n}", "ui", "12x12")
    for n in ("spring", "rune", "chest", "forge", "gate", "sauna", "bunker"):
        save_img(S.hotspot(n), f"ui/hotspot_{n}.png", f"Hotspot marker: {n}", "ui", "16x16")
    for n in ("rocket", "grenade", "bullet", "particle"):
        save_img(S.small(n), f"ui/proj_{n}.png", f"Projectile sprite: {n}", "sprite")

def audio():
    import gen_audio as G
    (A / "sfx").mkdir(parents=True, exist_ok=True); (A / "music").mkdir(parents=True, exist_ok=True)
    for k, x in G.sfx_table().items():
        G.save(str(A / "sfx" / f"{k}.wav"), x); add(f"SFX: {k}", f"sfx/{k}.wav", "sfx", f"{len(x)/G.SR:.2f}s 22.05kHz mono WAV")
    for k, x in G.music_table().items():
        G.save(str(A / "music" / f"{k}.wav"), x, .8); add(f"Music loop: {k}", f"music/{k}.wav", "music", f"{len(x)/G.SR:.0f}s 22.05kHz mono WAV, loops seamlessly")

def write_docs():
    (A / "manifest.json").write_text(json.dumps(M, indent=1))
    L = ["# nordSoldPow - Assets", "",
         "All assets are **generated placeholders** (the original Soldat / Powder Toy repos could not be cloned in the build sandbox - GitHub returned HTTP 403).",
         "Everything is loaded **at runtime** from `assets/`; replace a file (same path, same name) and restart. Missing files never crash the game (fallback shape / silence).",
         "Formats: sprites = PNG (RGBA, nearest-neighbour, 1 px = 1 world cell), audio = WAV or OGG (use the same file name; keep `.wav` unless you also edit `src/assets.rs` extension list - `.ogg` is tried first automatically).",
         "Regenerate everything: `python3 tools/gen_assets.py` (overwrites!).  Machine list: `assets/manifest.json`.", "",
         "## Quick list (use | name | path | source)", "", "| Use | Name | Path | Source |", "|---|---|---|---|"]
    for r in M: L.append(f"| {r['use']} | {r['name']} | `{r['path']}` | {r['source']} |")
    L += ["", "## Details & how to replace", "",
          "### Sprites (`assets/sprites/`)", "* `player1.png`, `player2.png`, `enemy_<kind>.png` (9 humanoids): sheet of **8 frames, each 16x24**, left->right: idle, run1, run2, run3, run4, jump, fall, dead. Character faces **right**; the game mirrors it. Feet touch bottom row-1. Arms are baked in; the weapon sprite is drawn on top rotated around the shoulder (x=8,y=10).",
          "* `enemy_fenrir_pup.png`: 4 frames of 24x16 (idle, run1, run2, dead).",
          "* `boss_<name>.png`: one 48x48 frame, faces right. Names: fenrir, surtr, ymir, jormungandr, hel, louhi, tuoni, rauta_golem, ukko_eagle.",
          "### Weapons (`assets/weapons/<weapon_key>.png`)", "* 24x12, barrel points right, grip/hand pivot at about (5,7). Key = console name (e.g. `vainamoinen_gun`).",
          "### Planets / map (`assets/planets/`)", "* 64x64 planet icons, `sol.png`/`paivatar.png` 96x96 stars, `gate.png` 48x48.",
          "### Backgrounds (`assets/bg/<planet>.png`)", "* 320x180 transparent parallax silhouette (drawn twice, scrolling at different speeds). Sky gradient + stars are drawn by code using colours in `src/planets.rs`.",
          "### UI (`assets/ui/`)", "* logo, crosshair, pickups (12x12), HUD icons, hotspot markers (16x16), projectile sprites.",
          "### Audio", "* `assets/sfx/<event>.wav` - weapon entries in `src/weapons.rs` reference these names (`sfx: \"shot_rifle\"`).",
          "* `assets/music/<track>.wav` - tracks: menu, map, calm, norse, kalevala, dark, boss. Planet -> track mapping is in `src/planets.rs` (`music` field). Must loop cleanly.",
          "### Optional data", "* `assets/data/elements.txt` (generated by `cargo run --release -- --dump-docs`): optional colour override for powder elements, one line per element: `key r g b`.",
          "### Fonts", "* The default built-in font is ASCII-only. To use a custom font put `assets/fonts/ui.ttf` (optional task 13.6).",
          "### Where to get real assets", "* Soldat (MIT code, assets under their own terms): https://github.com/soldat/soldat  - check licence before copying.", "* The Powder Toy has almost no art assets (elements are colours), see https://github.com/The-Powder-Toy/The-Powder-Toy.", "* Free options: OpenGameArt.org, Kenney.nl, freesound.org (check licences)."]
    (ROOT / "assets.md").write_text("\n".join(L) + "\n")

if __name__ == "__main__":
    only = sys.argv[2] if len(sys.argv) > 2 and sys.argv[1] == "--only" else None
    if only in (None, "sprites"): sprites()
    if only in (None, "audio"): audio()
    if only: # keep other rows from previous manifest
        old = json.loads((A / "manifest.json").read_text()) if (A / "manifest.json").exists() else []
        keep = [r for r in old if (r["kind"] == "sfx" or r["kind"] == "music") != (only == "audio")]
        M[:] = keep + M
    write_docs(); print(len(M), "assets")
