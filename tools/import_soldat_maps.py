#!/usr/bin/env python3
"""Convert Soldat .pms polygon maps (OpenSoldat 'base' content, CC BY 4.0) into nordSoldPow arena maps.

  git clone --depth 1 https://github.com/opensoldat/base /tmp/soldat-base
  python3 tools/import_soldat_maps.py /tmp/soldat-base

For every shared/maps/*.pms writes into assets/maps/:
  <stem>.png        element map  (R = element id, G = 255 where a rock backdrop shows, B = 0, A = 255)  <- loaded by the game
  <stem>.txt        metadata     (name, sky colours, spawn points, ...)
  <stem>_thumb.png  coloured thumbnail for the arena picker
Polygons are rasterised into the 1280x640 cell grid (scaled by <= 0.5 so players have the right proportions);
colour -> material is guessed from the map texture's average colour.
"""
import sys, struct, pathlib, math, re
import numpy as np
from PIL import Image

ROOT = pathlib.Path(__file__).resolve().parent.parent
W, H = 1280, 640
# element ids (must match src/elements.rs)
EL = dict(WALL=19, STONE=20, WOOD=21, METAL=22, GLASS=23, ICE=24, TNT=26, BRICK=29, LAVA=10, WATER=8, PACKED=57, TURF=58, SANDSTONE=59)
PALETTE = {  # element -> reference colour used to classify polygons
    EL['PACKED']: (116, 84, 56), EL['TURF']: (70, 150, 70), EL['SANDSTONE']: (214, 186, 120), EL['STONE']: (122, 122, 130), EL['BRICK']: (152, 72, 60),
    EL['METAL']: (150, 156, 168), EL['WOOD']: (124, 82, 42), EL['WALL']: (60, 60, 80)}
DRAWCOL = {19: (84, 84, 104), 20: (122, 122, 130), 21: (124, 82, 42), 22: (150, 156, 168), 23: (172, 222, 238), 24: (170, 215, 255), 26: (205, 42, 42), 29: (152, 72, 60), 10: (255, 92, 20), 8: (40, 100, 220), 57: (116, 84, 56), 58: (70, 150, 70), 59: (214, 186, 120)}

class R:
    def __init__(s, b): s.b, s.p = b, 0
    def u8(s): v = s.b[s.p]; s.p += 1; return v
    def u16(s): v = struct.unpack_from('<H', s.b, s.p)[0]; s.p += 2; return v
    def i32(s): v = struct.unpack_from('<i', s.b, s.p)[0]; s.p += 4; return v
    def f32(s): v = struct.unpack_from('<f', s.b, s.p)[0]; s.p += 4; return v
    def string(s, mx):
        n = s.u8(); raw = s.b[s.p:s.p + mx]; s.p += mx; return raw[:n].decode('latin-1')
    def color(s): b, g, r, a = s.b[s.p:s.p + 4]; s.p += 4; return (r, g, b, a)

def parse(path):
    r = R(path.read_bytes()); m = {}
    m['version'] = r.i32(); m['name'] = r.string(38); m['texture'] = r.string(24); m['top'] = r.color(); m['btm'] = r.color()
    r.i32(); m['grenades'] = r.u8(); m['medikits'] = r.u8(); m['weather'] = r.u8(); r.u8(); r.i32()
    n = r.i32(); polys = []
    if not (0 <= n < 20000): raise ValueError(f"bad polygon count {n}")
    for _ in range(n):
        verts = []
        for _ in range(3):
            x, y, z, rhw = r.f32(), r.f32(), r.f32(), r.f32(); col = r.color(); u, v = r.f32(), r.f32(); verts.append((x, y, col))
        r.p += 36; t = r.u8(); polys.append((verts, t))
    m['polys'] = polys
    r.i32(); num = r.i32(); cnt = (2 * num + 1) ** 2
    for _ in range(cnt): k = r.u16(); r.p += 2 * k
    k = r.i32(); r.p += 44 * k          # props
    k = r.i32(); r.p += 55 * k          # scenery
    k = r.i32(); r.p += 16 * k          # colliders
    k = r.i32(); sp = []
    for _ in range(k):
        act = r.u8(); r.p += 3; x, y, team = r.i32(), r.i32(), r.i32()
        if act: sp.append((x, y, team))
    m['spawns'] = sp
    return m

def tex_avg(base, name):
    d = base / 'shared' / 'textures'
    for p in [d / name] + list(d.glob('*')):
        if p.name.lower() == name.lower():
            try:
                im = Image.open(p).convert('RGBA'); a = np.asarray(im).reshape(-1, 4).astype(float); a = a[a[:, 3] > 128]
                return tuple(a[:, :3].mean(axis=0)) if len(a) else None
            except Exception: return None
    return None

def classify(rgb):
    best, bd = None, 1e18
    for el, c in PALETTE.items():
        d = sum((a - b) ** 2 * w for a, b, w in zip(rgb, c, (0.30, 0.59, 0.11)))
        if d < bd: best, bd = el, d
    return best

def fill_tri(arr, tri, val, w, h):
    xs = [p[0] for p in tri]; ys = [p[1] for p in tri]
    x0, x1 = max(int(math.floor(min(xs))), 0), min(int(math.ceil(max(xs))), w - 1); y0, y1 = max(int(math.floor(min(ys))), 0), min(int(math.ceil(max(ys))), h - 1)
    if x1 < x0 or y1 < y0: return
    (ax, ay), (bx, by), (cx, cy) = tri
    den = (by - cy) * (ax - cx) + (cx - bx) * (ay - cy)
    gx, gy = np.meshgrid(np.arange(x0, x1 + 1) + 0.5, np.arange(y0, y1 + 1) + 0.5)
    if abs(den) < 1e-9:  # degenerate: draw its edges
        return
    l1 = ((by - cy) * (gx - cx) + (cx - bx) * (gy - cy)) / den; l2 = ((cy - ay) * (gx - cx) + (ax - cx) * (gy - cy)) / den; l3 = 1 - l1 - l2
    eps = -0.02
    mask = (l1 >= eps) & (l2 >= eps) & (l3 >= eps)
    arr[y0:y1 + 1, x0:x1 + 1][mask] = val

def convert(base, path, out):
    m = parse(path); polys = m['polys']
    if not polys: raise ValueError("no polygons")
    xs = [v[0] for p, _ in polys for v in p]; ys = [v[1] for p, _ in polys for v in p]
    minx, maxx, miny, maxy = min(xs), max(xs), min(ys), max(ys); mw, mh = maxx - minx, maxy - miny
    s = min(0.5, 1236 / mw, 600 / mh); gw, gh = int(mw * s) + 1, int(mh * s) + 1; ox, oy = (W - gw) // 2, (H - gh) // 2
    tavg = tex_avg(base, m['texture'])
    el = np.zeros((H, W), np.uint8); bg = np.zeros((H, W), np.uint8)
    # outside of the map: wall
    el[:] = EL['WALL']; el[oy:oy + gh, ox:ox + gw] = 0
    def T(v): return ((v[0] - minx) * s + ox, (v[1] - miny) * s + oy)
    inner_bg = np.zeros((H, W), np.uint8); sol = np.zeros((H, W), np.uint8)
    for verts, t in polys:   # pass 1: backdrop
        if t in (24, 25, 3, 1): fill_tri(inner_bg, [T(v) for v in verts], 255, W, H)
    for verts, t in polys:   # pass 2: solids
        if t in (24, 25, 3, 1): continue
        col = np.mean([v[2][:3] for v in verts], axis=0)
        if tavg is not None: col = np.array(tavg) * col / 255.0
        if t == 4: e = EL['ICE']
        elif t in (5, 6, 7): e = EL['LAVA']
        elif t == 8: e = EL['WATER']
        elif t == 18: e = EL['GLASS']
        elif t == 19: e = EL['TNT']
        else: e = classify(tuple(col))
        fill_tri(sol, [T(v) for v in verts], e, W, H)
    inside = np.zeros((H, W), bool); inside[oy:oy + gh, ox:ox + gw] = True
    el = np.where(inside & (sol > 0), sol, el)
    bg = np.where(inside & (el == 0), inner_bg, 0)
    bg = np.where(inside & (el != 0) & (el != EL['WALL']), 255, bg).astype(np.uint8)
    # liquids from polygons would flood the arena; keep them (they are hazards) but cap a thin border of wall
    el[:2, :] = EL['WALL']; el[-2:, :] = EL['WALL']; el[:, :2] = EL['WALL']; el[:, -2:] = EL['WALL']
    stem = path.stem; out.mkdir(parents=True, exist_ok=True)
    img = np.zeros((H, W, 4), np.uint8); img[..., 0] = el; img[..., 1] = bg; img[..., 3] = 255
    Image.fromarray(img, 'RGBA').save(out / f'{stem}.png', optimize=True)
    # thumbnail
    th = np.zeros((H, W, 3), np.uint8); top, btm = m['top'][:3], m['btm'][:3]
    for y in range(H): t = y / H; th[y, :] = [int(top[i] * (1 - t) + btm[i] * t) for i in range(3)]
    th[bg > 0] = (30, 26, 38)
    for e, c in DRAWCOL.items(): th[el == e] = c
    Image.fromarray(th).crop((ox, oy, ox + gw, oy + gh)).resize((192, max(48, int(192 * gh / gw))), Image.NEAREST).save(out / f'{stem}_thumb.png', optimize=True)
    spawns = [((x - minx) * s + ox, (y - miny) * s + oy, t) for x, y, t in m['spawns']]
    spawns = [(int(x), int(y), t) for x, y, t in spawns if 3 < x < W - 3 and 3 < y < H - 3]
    meta = [f"name={m['name'] or stem}", f"file={stem}", f"sky={','.join(map(str, top))},{','.join(map(str, btm))}", f"scale={s:.3f}", f"size={gw},{gh}", f"offset={ox},{oy}",
            f"medikits={m['medikits']}", f"grenades={m['grenades']}", "spawns=" + ";".join(f"{x},{y},{t}" for x, y, t in spawns)]
    (out / f'{stem}.txt').write_text("\n".join(meta) + "\n")
    return len(polys), gw, gh, len(spawns)

def main():
    if len(sys.argv) < 2: sys.exit(__doc__)
    base = pathlib.Path(sys.argv[1]); out = ROOT / 'assets' / 'maps'; ok = bad = 0
    for p in sorted((base / 'shared' / 'maps').glob('*.pms')):
        try:
            n, gw, gh, sp = convert(base, p, out); ok += 1; print(f"{p.stem:18s} polys={n:5d} grid={gw}x{gh} spawns={sp}")
        except Exception as e:
            bad += 1; print(f"{p.stem:18s} SKIPPED: {e}")
    print(f"{ok} maps converted, {bad} skipped")
main()
