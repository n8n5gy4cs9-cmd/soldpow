#!/usr/bin/env python3
"""Import real Soldat sound effects (OpenSoldat 'base' content, CC BY 4.0) over the generated placeholders.

  git clone --depth 1 https://github.com/opensoldat/base /tmp/soldat-base
  python3 tools/import_soldat.py /tmp/soldat-base

Converts to 16-bit 22.05 kHz mono WAV (the game's format), writes assets/sfx/<event>.wav, records provenance in
tools/imported.json (so tools/gen_assets.py will not overwrite them) and refreshes assets.md.
Attribution (required by CC BY 4.0): "Sound effects from OpenSoldat base content, https://github.com/opensoldat/base, CC BY 4.0, converted to 16-bit 22 kHz mono."
"""
import sys, json, wave, pathlib
import numpy as np
ROOT = pathlib.Path(__file__).resolve().parent.parent
OUT = 22050

# game event name -> (soldat file, gain 0..1)
MAP = {
 "shot_pistol": ("deserteagle-fire", .85), "shot_revolver": ("colt1911-fire", .9), "shot_smg": ("mp5-fire", .8), "shot_rifle": ("ak74-fire", .85),
 "shot_shotgun": ("spas12-fire", .95), "shot_sniper": ("barretm82-fire", .95), "shot_minigun": ("minigun-fire", .7), "shot_flame": ("flamer", .7),
 "shot_acid": ("spit", .8), "shot_rocket": ("law", .9), "shot_grenade": ("m79-fire", .85), "melee": ("knife", .8),
 "explosion": ("grenade-explosion", .95), "explosion_big": ("cluster-explosion", 1.0),
 "hit_flesh": ("bonecrack", .8), "hit_wall": ("ric3", .55), "hurt": ("hit-arg2", .8), "die": ("death", .9),
 "jump": ("jump", .55), "land": ("fall", .7), "step": ("step", .4), "pickup": ("takegun", .7), "pickup_weapon": ("pickupgun", .8),
 "reload": ("mp5-reload", .7), "empty": ("minigun-empty", .6), "mission_done": ("capture", .9), "ui_click": ("menuclick", .6),
 "splash": ("bryzg", .8), "sizzle": ("burn", .6), "boss_roar": ("roar", 1.0), "heal": ("regenerate", .8),
}

def read_wav(p):
    try: return read_wav_py(p)
    except wave.Error:   # e.g. MS-ADPCM (jump.wav): needs ffmpeg
        import subprocess, shutil
        if not shutil.which("ffmpeg"): raise SystemExit(f"{p.name}: unsupported WAV encoding and ffmpeg not found (install ffmpeg or drop this entry from MAP)")
        raw = subprocess.run(["ffmpeg", "-v", "error", "-i", str(p), "-f", "s16le", "-ac", "1", "-ar", str(OUT), "-"], capture_output=True, check=True).stdout
        return np.frombuffer(raw, dtype="<i2").astype(np.float64) / 32768, OUT

def read_wav_py(p):
    w = wave.open(str(p)); ch, sw, sr, n = w.getnchannels(), w.getsampwidth(), w.getframerate(), w.getnframes(); raw = w.readframes(n)
    if sw == 1: x = (np.frombuffer(raw, dtype=np.uint8).astype(np.float64) - 128) / 128
    elif sw == 2: x = np.frombuffer(raw, dtype="<i2").astype(np.float64) / 32768
    else: raise ValueError(f"unsupported sample width {sw}")
    if ch > 1: x = x.reshape(-1, ch).mean(axis=1)
    return x, sr

def convert(x, sr):
    if sr != OUT:
        t = np.arange(int(len(x) * OUT / sr)) * sr / OUT
        x = np.interp(t, np.arange(len(x)), x)
    x = x - x.mean()
    return x

def main():
    if len(sys.argv) < 2: sys.exit(__doc__)
    base = pathlib.Path(sys.argv[1]) / "shared" / "sfx"
    if not base.is_dir(): sys.exit(f"{base} not found")
    imported = {}
    (ROOT / "assets" / "sfx").mkdir(parents=True, exist_ok=True)
    for name, (src, gain) in MAP.items():
        p = base / f"{src}.wav"
        if not p.exists(): print("missing", p); continue
        x, sr = read_wav(p); x = convert(x, sr)
        pk = np.max(np.abs(x)) or 1; x = np.clip(x / pk * gain, -1, 1)
        with wave.open(str(ROOT / "assets" / "sfx" / f"{name}.wav"), "wb") as w:
            w.setnchannels(1); w.setsampwidth(2); w.setframerate(OUT); w.writeframes((x * 32767).astype("<i2").tobytes())
        imported[f"sfx/{name}"] = f"OpenSoldat base: shared/sfx/{src}.wav (CC BY 4.0), converted"
        print(f"{name:16s} <- {src}.wav  ({len(x)/OUT:.2f}s)")
    (ROOT / "tools" / "imported.json").write_text(json.dumps(imported, indent=1))
    print(f"imported {len(imported)} sound effects; now run: python3 tools/gen_assets.py --only sprites   (refreshes assets.md)")

main()
