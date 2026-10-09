"""Pixel-art placeholder generator (Starbound-ish chunky style). Part of tools/gen_assets.py.
Everything here is procedural so it can be regenerated; replace the PNGs in assets/ with real art any time."""
import math, random
from PIL import Image, ImageDraw

OUT = (14, 10, 22, 255)

def C(h, a=255):
    h = h.lstrip('#'); return (int(h[0:2], 16), int(h[2:4], 16), int(h[4:6], 16), a)

def lighten(c, f): return tuple(min(255, int(v + (255 - v) * f)) for v in c[:3]) + (c[3],)
def darken(c, f): return tuple(int(v * (1 - f)) for v in c[:3]) + (c[3],)

def shade(img, hi=0.22, lo=0.28):
    """top-lit shading: lighten pixels with empty above, darken pixels with empty below."""
    w, h = img.size; src = img.copy(); px = src.load(); out = img.load()
    for y in range(h):
        for x in range(w):
            p = px[x, y]
            if p[3] == 0: continue
            up = px[x, y - 1][3] if y > 0 else 0
            dn = px[x, y + 1][3] if y < h - 1 else 0
            if up == 0: out[x, y] = lighten(p, hi)
            elif dn == 0: out[x, y] = darken(p, lo)
    return img

def outline(img, col=OUT):
    w, h = img.size; src = img.copy(); px = src.load(); out = img.load()
    for y in range(h):
        for x in range(w):
            if px[x, y][3] != 0: continue
            for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                nx, ny = x + dx, y + dy
                if 0 <= nx < w and 0 <= ny < h and px[nx, ny][3] > 100:
                    out[x, y] = col; break
    return img

def canvas(w, h): return Image.new("RGBA", (w, h), (0, 0, 0, 0))
def R(d, x0, y0, x1, y1, c):
    if x1 < x0 or y1 < y0: return
    d.rectangle([x0, y0, x1, y1], fill=c)

# ------------------------------------------------------------------ humanoids
FW, FH = 16, 24
HUMAN_FRAMES = 8   # idle, run1..4, jump, fall, dead

def human_frame(P, frame):
    im = canvas(FW, FH); d = ImageDraw.Draw(im)
    skin, coat, coat2, pants, boots = P['skin'], P['coat'], darken(P['coat'], .3), P['pants'], P['boots']
    legs = {0: (0, 0, 0, 0), 1: (2, 0, -2, 1), 2: (0, 0, 0, 2), 3: (-2, 1, 2, 0), 4: (0, 2, 0, 0), 5: (1, 2, -1, 2), 6: (2, 0, -2, 0)}.get(frame, (0, 0, 0, 0))
    bob = 1 if frame in (2, 4) else 0
    ghost = P.get('ghost')
    # back leg (darker) then front leg
    if not ghost:
        for (x, dx, lift, col) in ((5, legs[2], legs[3], darken(pants, .25)), (8, legs[0], legs[1], pants)):
            R(d, x, 15 - bob, x + 2, 18, col)
            R(d, x + dx, 19 - bob, x + dx + 2, 22 - lift, col)
            R(d, x + dx, 21 - lift, x + dx + 3, 22 - lift, boots)
    else:
        for i, y in enumerate(range(15, 22)):
            w = 6 - i // 2; R(d, 8 - w // 2, y, 8 + w // 2, y, pants)
    if P.get('wings'):
        wc = P['wings']
        d.polygon([(5, 9), (0, 5), (0, 14), (5, 14)], fill=wc); d.polygon([(5, 9), (2, 3), (1, 8)], fill=lighten(wc, .4))
    if P.get('pack'):  R(d, 3, 9 - bob, 5, 15 - bob, P['pack']); R(d, 3, 15 - bob, 4, 15 - bob, (240, 130, 40, 255))
    # torso
    R(d, 5, 8 - bob, 10, 14 - bob, coat); R(d, 5, 8 - bob, 6, 14 - bob, coat2)
    R(d, 5, 14 - bob, 10, 14 - bob, P.get('belt', darken(coat, .6)))
    if P.get('plate'): R(d, 7, 9 - bob, 10, 12 - bob, P['plate'])
    if P.get('scarf'):
        R(d, 5, 8 - bob, 10, 9 - bob, P['scarf']); R(d, 3, 9 - bob, 4, 9 - bob, P['scarf']); R(d, 2, 10 - bob, 3, 10 - bob, darken(P['scarf'], .2))
    # arms
    R(d, 9, 9 - bob, 10, 12 - bob, darken(coat, .1)); R(d, 9, 13 - bob, 10, 13 - bob, skin)
    R(d, 6, 9 - bob, 7, 12 - bob, coat2); R(d, 6, 13 - bob, 7, 13 - bob, darken(skin, .2))
    # head
    hy = 2 - bob
    R(d, 6, hy + 1, 10, hy + 5, skin)
    hg = P.get('head', 'bare'); hair = P.get('hair', C('#5a3a1a')); eye = P.get('eye', C('#101018'))
    if hg == 'bare':
        R(d, 5, hy, 10, hy + 1, hair); R(d, 5, hy + 1, 6, hy + 4, hair)
    elif hg in ('helm', 'horn'):
        R(d, 5, hy, 10, hy + 2, P['helm']); R(d, 5, hy + 2, 6, hy + 4, P['helm']); R(d, 9, hy + 1, 9, hy + 3, darken(P['helm'], .3))
        if hg == 'horn':
            R(d, 4, hy - 1, 4, hy, C('#e8e0c8')); R(d, 11, hy - 1, 11, hy, C('#e8e0c8')); R(d, 3, hy - 2, 3, hy - 1, C('#e8e0c8')); R(d, 12, hy - 2, 12, hy - 1, C('#e8e0c8'))
    elif hg == 'hood':
        R(d, 5, hy, 10, hy + 6, P['helm']); R(d, 8, hy + 2, 10, hy + 5, skin)
    elif hg == 'fur':
        R(d, 5, hy - 1, 11, hy + 1, P['helm']); R(d, 5, hy + 1, 6, hy + 4, P['helm'])
    elif hg == 'skull':
        R(d, 6, hy + 1, 10, hy + 5, C('#d8d4c0')); R(d, 8, hy + 2, 9, hy + 3, C('#101018')); R(d, 7, hy + 5, 10, hy + 5, C('#a8a490'))
    elif hg == 'crown':
        R(d, 5, hy, 10, hy + 1, hair)
        for x in (5, 7, 9, 10): R(d, x, hy - 2, x, hy - 1, C('#bfefff'))
    elif hg == 'goblin':
        R(d, 5, hy + 1, 5, hy + 3, skin); R(d, 11, hy + 1, 12, hy + 2, skin); R(d, 6, hy, 10, hy, darken(skin, .3))
    elif hg == 'wing':
        R(d, 5, hy, 10, hy + 2, P['helm']); R(d, 4, hy, 4, hy - 1, C('#ffffff')); R(d, 11, hy, 11, hy - 1, C('#ffffff'))
    elif hg == 'visor':
        R(d, 5, hy, 10, hy + 3, P['helm']); R(d, 8, hy + 2, 10, hy + 2, P.get('eye', C('#40e0ff')))
    if hg not in ('skull', 'visor'):
        R(d, 9, hy + 3, 9, hy + 3, eye)
    elif hg == 'skull': pass
    if P.get('beard'): R(d, 8, hy + 5, 10, hy + 7, P['beard']); R(d, 7, hy + 5, 7, hy + 6, P['beard'])
    if P.get('glow'): R(d, 9, hy + 3, 10, hy + 3, P['glow'])
    shade(im); outline(im)
    if ghost:
        a = im.split()[3].point(lambda v: int(v * 0.72)); im.putalpha(a)
    return im

def human_sheet(P):
    sheet = canvas(FW * HUMAN_FRAMES, FH)
    for f in range(7): sheet.paste(human_frame(P, f), (f * FW, 0))
    idle = human_frame(P, 0)
    lying = idle.rotate(90, expand=True).resize((16, 11), Image.NEAREST)
    dead = canvas(FW, FH); dead.paste(lying, (0, 13)); sheet.paste(dead, (7 * FW, 0))
    return sheet

PLAYER1 = dict(skin=C('#e8b890'), coat=C('#2f5aa8'), pants=C('#3a3a48'), boots=C('#2a2018'), head='horn', helm=C('#8a909c'),
               scarf=C('#3ab0e8'), pack=C('#707888'), beard=C('#c8923a'), plate=C('#4a4a58'))
PLAYER2 = dict(PLAYER1, coat=C('#a83030'), scarf=C('#f0a030'), beard=C('#403028'), head='helm', helm=C('#a09078'), plate=C('#5a3a30'))
ENEMIES = {
 'draugr':      dict(skin=C('#8fb0a8'), coat=C('#3c4a44'), pants=C('#2c3430'), boots=C('#1c201e'), head='skull', glow=C('#60ffd0'), plate=C('#586058')),
 'berserker':   dict(skin=C('#e0a880'), coat=C('#7a3a22'), pants=C('#4a3828'), boots=C('#2a1c14'), head='horn', helm=C('#6a6a70'), beard=C('#d05020'), belt=C('#c8a040')),
 'eldjotunn':   dict(skin=C('#e0602a'), coat=C('#4a1a10'), pants=C('#321410'), boots=C('#201010'), head='crown', hair=C('#ffb020'), glow=C('#ffe040'), plate=C('#ff7020')),
 'frostjotunn': dict(skin=C('#a8d4f0'), coat=C('#4a6c9c'), pants=C('#34486c'), boots=C('#22304a'), head='crown', hair=C('#e8f8ff'), beard=C('#e8f8ff'), plate=C('#8ab0d8')),
 'valkyrie':    dict(skin=C('#f0c8a8'), coat=C('#d8d8e8'), pants=C('#8890b0'), boots=C('#505878'), head='wing', helm=C('#e0c050'), wings=C('#f4f4ff'), plate=C('#e0c050')),
 'hiisi':       dict(skin=C('#6ea04a'), coat=C('#5a4228'), pants=C('#3c3020'), boots=C('#241a10'), head='goblin', eye=C('#ffe030'), belt=C('#a02020')),
 'kalma':       dict(skin=C('#b8c0e8'), coat=C('#6068a0'), pants=C('#4a5088'), boots=C('#383c68'), head='hood', helm=C('#2c3050'), ghost=True, glow=C('#e0f0ff')),
 'louhi_guard': dict(skin=C('#d8b090'), coat=C('#30344c'), pants=C('#242838'), boots=C('#14161e'), head='visor', helm=C('#50587c'), eye=C('#ff4060'), plate=C('#6c78a8')),
 'svartalf':    dict(skin=C('#585078'), coat=C('#241c3c'), pants=C('#1a1430'), boots=C('#0e0a1c'), head='hood', helm=C('#2e2448'), glow=C('#c060ff')),
}

def wolf_sheet():
    W, H = 24, 16; sheet = canvas(W * 4, H)
    for f in range(4):
        im = canvas(W, H); d = ImageDraw.Draw(im); fur = C('#8c8c98'); dk = darken(fur, .35)
        R(d, 5, 5, 18, 10, fur); R(d, 5, 9, 18, 10, dk)                   # body
        R(d, 17, 3, 22, 8, fur); R(d, 21, 5, 23, 7, dk); R(d, 20, 4, 20, 4, C('#ff3030'))  # head+eye
        R(d, 18, 1, 19, 3, fur); R(d, 21, 1, 21, 3, fur)                  # ears
        R(d, 2, 4, 5, 6, fur); R(d, 0, 3, 2, 4, fur)                      # tail
        a = [(0, 0), (2, -2), (0, 0), (-2, 2)][f]
        if f == 3: R(d, 5, 9, 18, 12, fur); R(d, 3, 11, 20, 12, dk); R(d, 21, 8, 23, 10, dk)
        else:
            for lx, off in ((6, a[0]), (9, a[1]), (14, -a[1]), (17, -a[0])):
                R(d, lx + off // 2, 11, lx + 1 + off // 2, 14, dk if lx in (9, 17) else fur)
        shade(im); outline(im); sheet.paste(im, (f * W, 0))
    return sheet

# ------------------------------------------------------------------ bosses (48x48)
def boss_img(fn):
    im = canvas(48, 48); d = ImageDraw.Draw(im); fn(d); shade(im, .18, .3); outline(im); return im

def giant(d, skin, body, accent, head='crown', glow=None, hair=None):
    R(d, 14, 20, 34, 38, body); R(d, 14, 20, 18, 38, darken(body, .3)); R(d, 14, 34, 34, 36, accent)
    R(d, 15, 38, 21, 46, darken(body, .2)); R(d, 27, 38, 33, 46, darken(body, .2)); R(d, 13, 45, 22, 47, darken(body, .5)); R(d, 26, 45, 35, 47, darken(body, .5))
    R(d, 6, 20, 13, 34, body); R(d, 35, 20, 42, 34, body); R(d, 6, 34, 13, 38, skin); R(d, 35, 34, 42, 38, skin)
    R(d, 12, 18, 36, 22, accent)
    R(d, 18, 6, 30, 19, skin)
    if hair: d.polygon([(16, 8), (18, 0), (22, 6), (24, -1), (27, 6), (31, 0), (32, 8)], fill=hair)
    R(d, 20, 11, 22, 12, glow or C('#101018')); R(d, 26, 11, 28, 12, glow or C('#101018')); R(d, 21, 16, 27, 17, darken(skin, .5))

def b_surtr(d): giant(d, C('#c0481c'), C('#3a1612'), C('#ff7a1a'), glow=C('#ffee40'), hair=C('#ff9020')); R(d, 40, 8, 42, 36, C('#ffcc40')); R(d, 38, 4, 44, 8, C('#ff4010'))
def b_ymir(d):  giant(d, C('#b8dcf4'), C('#5a80b0'), C('#e8f8ff'), glow=C('#2060ff'), hair=C('#e8f8ff')); R(d, 8, 8, 10, 16, C('#d0f0ff')); R(d, 38, 8, 40, 16, C('#d0f0ff'))
def b_hel(d):
    giant(d, C('#e8d0b8'), C('#14121e'), C('#6a2a8a'), glow=C('#a060ff'), hair=C('#201830')); R(d, 24, 6, 30, 19, C('#c8c4b0')); R(d, 26, 10, 28, 12, C('#101018'))
def b_louhi(d):
    giant(d, C('#d0b8c8'), C('#2a3050'), C('#80a0e0'), glow=C('#ff4060'), hair=C('#c0d0f0'))
    d.polygon([(14, 22), (0, 12), (2, 36), (14, 34)], fill=C('#8a98c0')); d.polygon([(34, 22), (47, 12), (46, 36), (34, 34)], fill=C('#8a98c0'))
def b_tuoni(d):
    giant(d, C('#d8d4c0'), C('#101014'), C('#40ff90'), glow=C('#40ff90')); R(d, 18, 2, 30, 6, C('#9a9a9a')); [R(d, x, 0, x + 1, 2, C('#c8c8c8')) for x in (18, 22, 26, 29)]
def b_golem(d):
    R(d, 12, 14, 36, 40, C('#7a808c')); R(d, 12, 14, 36, 17, C('#a0a8b4')); R(d, 4, 18, 11, 38, C('#626874')); R(d, 37, 18, 44, 38, C('#626874'))
    R(d, 14, 40, 22, 47, C('#4a4e58')); R(d, 26, 40, 34, 47, C('#4a4e58')); R(d, 18, 4, 30, 13, C('#8a909c')); R(d, 20, 7, 28, 9, C('#ff8020'))
    for (x, y) in ((16, 20), (32, 20), (16, 34), (32, 34)): R(d, x, y, x + 1, y + 1, C('#c0c8d0'))
    R(d, 21, 24, 27, 32, C('#ff6010')); R(d, 22, 25, 26, 31, C('#ffd040'))
def b_ukko(d):
    d.polygon([(24, 22), (0, 10), (4, 30), (18, 34)], fill=C('#3a4468')); d.polygon([(24, 22), (47, 10), (43, 30), (30, 34)], fill=C('#3a4468'))
    d.polygon([(0, 10), (6, 24), (10, 20)], fill=C('#ffe040')); d.polygon([(47, 10), (41, 24), (37, 20)], fill=C('#ffe040'))
    R(d, 18, 16, 30, 36, C('#5a4a38')); R(d, 19, 6, 29, 17, C('#f0f0f8')); R(d, 29, 10, 36, 13, C('#ffc030')); R(d, 25, 9, 27, 10, C('#ffe040'))
    R(d, 19, 36, 22, 44, C('#ffc030')); R(d, 26, 36, 29, 44, C('#ffc030'))
def b_fenrir(d):
    fur = C('#7c7c8c'); R(d, 6, 18, 36, 36, fur); R(d, 6, 30, 36, 36, darken(fur, .3)); R(d, 30, 8, 44, 24, fur); R(d, 40, 14, 47, 21, darken(fur, .3)); R(d, 38, 10, 39, 12, C('#ff3030'))
    d.polygon([(32, 8), (34, 0), (37, 8)], fill=fur); d.polygon([(40, 8), (42, 1), (44, 9)], fill=fur); d.polygon([(6, 18), (0, 8), (4, 24)], fill=fur)
    for x in (8, 15, 26, 32): R(d, x, 36, x + 3, 46, darken(fur, .25))
    R(d, 42, 20, 45, 22, C('#ffffff'))
def b_jorm(d):
    g = C('#2e7a5a')
    for i in range(7):
        a = i * 0.9; x = 6 + i * 5; y = 36 - int(math.sin(a) * 10); d.ellipse([x, y - 6, x + 11, y + 6], fill=g if i % 2 == 0 else darken(g, .2))
    d.ellipse([30, 4, 46, 20], fill=C('#3a9a70')); R(d, 40, 8, 42, 10, C('#ffe040')); R(d, 44, 14, 47, 15, C('#ff3040')); R(d, 30, 36, 40, 44, g)
BOSSES = dict(fenrir=b_fenrir, surtr=b_surtr, ymir=b_ymir, jormungandr=b_jorm, hel=b_hel, louhi=b_louhi, tuoni=b_tuoni, rauta_golem=b_golem, ukko_eagle=b_ukko)

# ------------------------------------------------------------------ weapons (24x12, barrel to +x, grip pivot ~ (5,7))
WW, WH = 24, 12
def weapon_img(key):
    im = canvas(WW, WH); d = ImageDraw.Draw(im)
    M = C('#4a4e5a'); M2 = C('#2c2e36'); W = C('#8a5a2c'); H = C('#a8adb8')
    def gun(stock=True, bl=8, mag=None, scope=False, body=M, wood=False):
        R(d, 4, 5, 13, 8, body)
        if stock: R(d, 0, 5, 4, 8, W if wood else M2)
        R(d, 13, 5, 13 + bl, 6, M2)
        R(d, 5, 8, 6, 11, M2)  # grip
        if mag: R(d, mag, 8, mag + 1, 10, M2)
        if scope: R(d, 7, 3, 12, 4, C('#202028')); R(d, 7, 4, 8, 4, M2)
    k = key
    if k == 'puukko':
        R(d, 3, 6, 6, 8, W); R(d, 7, 6, 8, 8, C('#c8a040')); d.polygon([(9, 5), (21, 6), (21, 7), (9, 8)], fill=H)
    elif k == 'hugin': gun(stock=False, bl=3, mag=None)
    elif k == 'munin': R(d, 4, 5, 10, 7, M); R(d, 10, 5, 17, 6, H); R(d, 4, 7, 5, 11, W); R(d, 6, 6, 8, 8, C('#c8a040'))
    elif k == 'lemminkainen_smg': gun(stock=False, bl=4, mag=7)
    elif k == 'vainamoinen_gun':
        gun(stock=True, bl=7, mag=8, wood=True, body=C('#6a4a2a')); R(d, 0, 4, 3, 9, W)
        for y in (5, 6, 7, 8): d.point((1, y), fill=C('#e8e0c8'))
        R(d, 14, 4, 15, 4, C('#e8c040'))
    elif k == 'kullervo_shotgun': gun(stock=True, bl=9, wood=True, body=C('#5a3c24')); R(d, 13, 7, 22, 7, M2)
    elif k == 'gungnir_sniper': gun(stock=True, bl=9, scope=True, body=C('#3c4a3c')); R(d, 14, 5, 22, 5, C('#c8a040'))
    elif k == 'mjolnir_rpg': R(d, 3, 4, 20, 8, C('#5a6a4a')); R(d, 19, 3, 23, 9, C('#c8c8d0')); R(d, 6, 8, 7, 11, M2); R(d, 12, 8, 12, 10, M2); R(d, 3, 5, 4, 7, C('#ffcc40'))
    elif k == 'gjallar_grenade': R(d, 3, 5, 14, 9, M); R(d, 14, 4, 21, 9, M2); R(d, 20, 4, 21, 9, H); R(d, 5, 9, 6, 11, M2); R(d, 0, 5, 3, 7, M2)
    elif k == 'surtr_flame': R(d, 2, 3, 8, 10, C('#c03020')); R(d, 8, 5, 18, 7, M); R(d, 18, 4, 21, 8, M2); R(d, 20, 5, 22, 7, C('#ff9020')); R(d, 5, 10, 6, 11, M2)
    elif k == 'louhi_frost': gun(stock=False, bl=6, mag=None, body=C('#5a90c0')); R(d, 13, 4, 19, 4, C('#d8f4ff')); R(d, 19, 5, 21, 6, C('#ffffff'))
    elif k == 'tuoni_acid': gun(stock=False, bl=5, body=C('#3a6a3a')); R(d, 8, 2, 12, 5, C('#60ff60')); R(d, 19, 5, 21, 6, C('#a0ff40'))
    elif k == 'ukko_thunder':
        gun(stock=False, bl=3, body=C('#5a5a70')); [R(d, x, 3, x + 1, 9, C('#ffe040')) for x in (14, 17, 20)]; R(d, 14, 6, 21, 6, C('#fff8c0'))
    elif k == 'ilmarinen_rail': gun(stock=True, bl=9, scope=True, body=C('#6a7088')); R(d, 13, 4, 22, 4, C('#40e0ff')); R(d, 13, 8, 22, 8, C('#40e0ff'))
    elif k == 'sampo_gun':
        R(d, 4, 4, 14, 9, C('#9a7a4a')); R(d, 14, 5, 20, 8, M2); R(d, 6, 1, 12, 4, C('#c8a040')); R(d, 5, 9, 6, 11, M2)
        for i, c in enumerate(('#ff4040', '#40ff40', '#4040ff', '#ffff40')): R(d, 5 + i * 2, 5, 6 + i * 2, 6, C(c))
    elif k == 'powder_gun': R(d, 3, 5, 14, 9, M); R(d, 14, 6, 20, 8, M2); R(d, 4, 1, 12, 4, C('#d8c070')); R(d, 5, 9, 6, 11, M2)
    elif k == 'fenrir_minigun': R(d, 3, 4, 10, 9, M); [R(d, 10, y, 22, y, H if y % 2 else M2) for y in (3, 4, 5, 6, 7, 8)]; R(d, 6, 9, 7, 11, M2); R(d, 2, 6, 3, 7, C('#c8a040'))
    elif k == 'loki_trick': R(d, 4, 5, 12, 8, C('#2a8a4a')); R(d, 12, 6, 18, 7, M2); R(d, 18, 4, 21, 9, C('#ffd040')); R(d, 5, 8, 6, 11, M2)
    else: gun()
    shade(im, .2, .3); outline(im); return im

WEAPON_KEYS = ['puukko', 'hugin', 'munin', 'lemminkainen_smg', 'vainamoinen_gun', 'kullervo_shotgun', 'gungnir_sniper', 'mjolnir_rpg',
               'gjallar_grenade', 'surtr_flame', 'louhi_frost', 'tuoni_acid', 'ukko_thunder', 'ilmarinen_rail', 'sampo_gun', 'powder_gun',
               'fenrir_minigun', 'loki_trick']

# ------------------------------------------------------------------ planets
def planet_img(name, base, accent, kind, size=64):
    rnd = random.Random(hash(name) & 0xffff); im = canvas(size, size); px = im.load(); c = size / 2 - 3
    ph = [rnd.random() * 6 for _ in range(4)]
    for y in range(size):
        for x in range(size):
            dx, dy = x - size / 2 + .5, y - size / 2 + .5; r = math.hypot(dx, dy)
            if r > c: continue
            nz = math.sqrt(max(0, 1 - (r / c) ** 2)); lit = max(0.15, (-dx * .5 - dy * .6) / c * .7 + nz * .6)
            t = (math.sin(dy * .35 + ph[0] + math.sin(dx * .1 + ph[1]) * 1.5) + 1) / 2 if kind in ('bands', 'gas') else (math.sin(dx * .21 + ph[0]) * math.sin(dy * .23 + ph[1]) + 1) / 2
            if kind == 'ice': t = (math.sin(dx * .4 + dy * .3 + ph[0]) + 1) / 2 * .5
            col = tuple(int(base[i] * (1 - t) + accent[i] * t) for i in range(3))
            if kind == 'craters' and rnd.random() < .004: pass
            col = tuple(min(255, int(v * (0.35 + lit * .85))) for v in col)
            px[x, y] = col + (255,)
    d = ImageDraw.Draw(im)
    if kind == 'craters':
        for _ in range(9):
            cx, cy, cr = rnd.randint(14, 50), rnd.randint(14, 50), rnd.randint(2, 5)
            if math.hypot(cx - 32, cy - 32) < c - cr: d.ellipse([cx - cr, cy - cr, cx + cr, cy + cr], outline=darken(base + (255,), .45), fill=darken(base + (255,), .2))
    if kind == 'lava':
        for _ in range(26):
            cx, cy = rnd.randint(12, 52), rnd.randint(12, 52)
            if math.hypot(cx - 32, cy - 32) < c - 3: d.line([cx, cy, cx + rnd.randint(-6, 6), cy + rnd.randint(-5, 5)], fill=(255, 150, 30, 255))
    if kind == 'ocean':
        for _ in range(7):
            cx, cy = rnd.randint(14, 50), rnd.randint(14, 50)
            if math.hypot(cx - 32, cy - 32) < c - 6: d.ellipse([cx - 5, cy - 3, cx + 6, cy + 3], fill=(70, 160, 70, 255))
    if kind == 'rings':
        d.ellipse([0, 24, 63, 40], outline=(230, 220, 190, 255)); d.ellipse([3, 26, 60, 38], outline=(190, 180, 150, 255))
        for x in range(64):
            for y in range(24, 41):
                p = im.getpixel((x, y))
                if p[3] and abs(math.hypot((x - 31.5) / 31.5, (y - 32) / 8) - 1) > .05 and False: pass
    # rim glow
    glow = canvas(size, size); gd = ImageDraw.Draw(glow); gd.ellipse([1, 1, size - 2, size - 2], outline=accent + (90,), width=1)
    im = Image.alpha_composite(glow, im); outline(im, (10, 8, 20, 255)); return im

def star_img(name, core, edge, size=96):
    im = canvas(size, size); px = im.load(); c = size / 2
    for y in range(size):
        for x in range(size):
            r = math.hypot(x - c + .5, y - c + .5)
            if r < c * .62: t = r / (c * .62); px[x, y] = tuple(int(core[i] * (1 - t) + edge[i] * t) for i in range(3)) + (255,)
            elif r < c: a = int(120 * (1 - (r - c * .62) / (c * .38)) ** 2); px[x, y] = edge + (a,)
    return im

PLANETS = {  # id: (base, accent, kind)
 'midgard': ((50, 120, 200), (60, 170, 70), 'ocean'), 'mani': ((170, 170, 180), (110, 110, 125), 'craters'),
 'muspell': ((150, 50, 30), (255, 140, 30), 'lava'), 'vanaheim': ((200, 190, 70), (70, 160, 70), 'gas'),
 'jotunheim': ((190, 90, 50), (120, 50, 30), 'blotch'), 'asgard': ((220, 170, 110), (255, 235, 190), 'bands'),
 'niflheim': ((200, 220, 235), (140, 180, 215), 'rings'), 'svartalfheim': ((60, 40, 110), (130, 70, 200), 'bands'),
 'helheim': ((20, 40, 80), (50, 30, 110), 'ice'),
 'kalevala': ((70, 160, 90), (150, 210, 120), 'ocean'), 'pohjola': ((200, 230, 250), (120, 160, 200), 'ice'),
 'tuonela': ((40, 50, 60), (90, 130, 110), 'blotch'), 'sampola': ((130, 90, 60), (255, 120, 30), 'lava'),
 'ukkola': ((90, 100, 160), (240, 230, 90), 'gas'),
}

# ------------------------------------------------------------------ backgrounds (320x180 transparent silhouette layers)
def bg_img(name, style, col):
    rnd = random.Random(sum(map(ord, name))); W, H = 320, 180; im = canvas(W, H); d = ImageDraw.Draw(im)
    def prof(base, amp, f1, f2):
        p1, p2 = rnd.random() * 9, rnd.random() * 9
        return [int(base + math.sin(x / f1 + p1) * amp + math.sin(x / f2 + p2) * amp * .5) for x in range(W)]
    layers = [(darken(col + (255,), .15), 120, 26, 40, 14), (darken(col + (255,), .45), 140, 18, 20, 8)]
    for lc, base, amp, f1, f2 in layers:
        p = prof(base, amp, f1, f2)
        for x in range(W):
            y = p[x]
            if style in ('peaks', 'spikes'): y = base - int(abs(((x * (0.9 if style == 'peaks' else 1.6)) % 60) - 30) * (1.3 if style == 'peaks' else 2.0)) + int(math.sin(x / 40) * 10)
            if style == 'dunes': y = int(base + math.sin(x / 33 + rnd.random() * .01) * amp)
            if style == 'waves': y = int(base + 20 + math.sin(x / 7) * 3 + math.sin(x / 31) * 6)
            if style == 'craters': y = int(base + 10 + math.sin(x / 50) * 8)
            d.line([x, y, x, H], fill=lc)
        if style in ('pines', 'trees'):
            for x in range(4, W, 11 if style == 'pines' else 19):
                y = p[x]; h = rnd.randint(14, 26)
                if style == 'pines': d.polygon([(x - 5, y), (x, y - h), (x + 5, y)], fill=lc); d.polygon([(x - 4, y - h // 2), (x, y - h - 4), (x + 4, y - h // 2)], fill=lc)
                else: d.line([x, y, x + rnd.randint(-3, 3), y - h], fill=lc, width=2); d.ellipse([x - 6, y - h - 6, x + 6, y - h + 4], fill=lc)
        if style == 'clouds':
            for _ in range(10):
                cx, cy = rnd.randint(0, W), rnd.randint(30, 140)
                for k in range(4): d.ellipse([cx + k * 9 - 12, cy - 5, cx + k * 9 + 12, cy + 7], fill=lc)
            break
        if style == 'stalactites':
            for x in range(0, W, 9):
                h = rnd.randint(8, 50); d.polygon([(x, 0), (x + 8, 0), (x + 4, h)], fill=lc)
        if style == 'chimneys':
            for x in range(10, W, 40):
                h = rnd.randint(30, 70); d.rectangle([x, 150 - h, x + 8, 160], fill=lc); d.rectangle([x - 2, 150 - h, x + 10, 154 - h], fill=lc)
        if style == 'storm':
            for _ in range(8):
                cx, cy = rnd.randint(0, W), rnd.randint(20, 100)
                for k in range(5): d.ellipse([cx + k * 10 - 14, cy - 6, cx + k * 10 + 14, cy + 9], fill=lc)
            break
    return im

BGS = {  # id: (style, silhouette colour)
 'midgard': ('hills', (60, 110, 80)), 'mani': ('craters', (130, 130, 150)), 'muspell': ('peaks', (150, 60, 40)),
 'vanaheim': ('trees', (80, 140, 70)), 'jotunheim': ('peaks', (160, 80, 60)), 'asgard': ('clouds', (255, 230, 190)),
 'niflheim': ('spikes', (170, 200, 230)), 'svartalfheim': ('stalactites', (110, 70, 170)), 'helheim': ('waves', (50, 60, 120)),
 'kalevala': ('pines', (70, 120, 80)), 'pohjola': ('spikes', (180, 210, 235)), 'tuonela': ('trees', (70, 90, 100)),
 'sampola': ('chimneys', (140, 90, 60)), 'ukkola': ('storm', (110, 120, 170)),
}

# ------------------------------------------------------------------ UI
def logo_img():
    W, H = 420, 96; im = canvas(W, H); d = ImageDraw.Draw(im)
    f = {'n': ['1001', '1101', '1011', '1001'], }
    # blocky 5x7 font drawn at scale 6 for "NORDSOLDPOW"
    G = {
     'N': ["10001", "11001", "10101", "10011", "10001", "10001", "10001"], 'O': ["01110", "10001", "10001", "10001", "10001", "10001", "01110"],
     'R': ["11110", "10001", "10001", "11110", "10100", "10010", "10001"], 'D': ["11110", "10001", "10001", "10001", "10001", "10001", "11110"],
     'S': ["01111", "10000", "10000", "01110", "00001", "00001", "11110"], 'L': ["10000", "10000", "10000", "10000", "10000", "10000", "11111"],
     'P': ["11110", "10001", "10001", "11110", "10000", "10000", "10000"], 'W': ["10001", "10001", "10001", "10101", "10101", "11011", "10001"]}
    x = 6; s = 5
    for i, ch in enumerate("NORDSOLDPOW"):
        col = C('#e8c860') if i < 4 else (C('#ff7040') if i < 7 else C('#7ad0ff'))
        if ch == 'P' and i == 8 or i >= 8: col = C('#7ad0ff') if i >= 8 else col
        for r, row in enumerate(G[ch]):
            for c, v in enumerate(row):
                if v == '1': R(d, x + c * s, 10 + r * s, x + c * s + s - 1, 10 + r * s + s - 1, col)
        x += 6 * s + 2
    d.line([6, 62, W - 6, 62], fill=C('#8a6a30')); [R(d, xx, 60, xx + 3, 64, C('#e8c860')) for xx in range(6, W - 6, 28)]
    shade(im); outline(im); return im

def icon(name, size=12):
    im = canvas(size, size); d = ImageDraw.Draw(im); n = name
    if n == 'health': R(d, 4, 1, 7, 10, C('#f0f0f0')); R(d, 1, 4, 10, 7, C('#f0f0f0')); R(d, 5, 2, 6, 9, C('#e02828')); R(d, 2, 5, 9, 6, C('#e02828'))
    elif n == 'ammo': [R(d, 2 + i * 3, 3, 3 + i * 3, 10, C('#d8a838')) or R(d, 2 + i * 3, 2, 3 + i * 3, 3, C('#b86030')) for i in range(3)]
    elif n == 'shard': d.polygon([(6, 0), (11, 5), (7, 11), (2, 7)], fill=C('#40e0ff')); d.polygon([(6, 0), (7, 11), (4, 6)], fill=C('#c8faff'))
    elif n == 'silver': d.ellipse([1, 1, 10, 10], fill=C('#c8ccd8')); d.ellipse([3, 3, 8, 8], outline=C('#7a8090'))
    elif n == 'weapon': R(d, 1, 4, 10, 7, C('#6a6e7a')); R(d, 2, 7, 4, 10, C('#4a4e5a')); R(d, 10, 5, 11, 5, C('#c8a040'))
    elif n == 'grenade': d.ellipse([2, 3, 9, 10], fill=C('#3a6a3a')); R(d, 5, 1, 6, 3, C('#a8a8b0'))
    elif n == 'fuel': R(d, 3, 2, 8, 10, C('#d83030')); R(d, 4, 1, 7, 2, C('#a8a8b0')); R(d, 4, 5, 7, 8, C('#ffb030'))
    shade(im); outline(im); return im

def hotspot(name):
    im = canvas(16, 16); d = ImageDraw.Draw(im); n = name
    if n == 'spring': d.ellipse([1, 8, 14, 14], fill=C('#3a90e0')); d.ellipse([3, 9, 12, 12], fill=C('#8ad0ff')); R(d, 7, 2, 8, 8, C('#bfefff')); d.point((5, 5), fill=C('#bfefff')); d.point((10, 4), fill=C('#bfefff'))
    elif n == 'rune': d.polygon([(4, 14), (4, 4), (8, 1), (12, 4), (12, 14)], fill=C('#7a7a88')); d.line([8, 4, 8, 11], fill=C('#40e0ff')); d.line([6, 6, 10, 9], fill=C('#40e0ff')); d.line([10, 6, 6, 9], fill=C('#40e0ff'))
    elif n == 'chest': R(d, 2, 6, 13, 13, C('#8a5a2c')); R(d, 2, 4, 13, 7, C('#a8743c')); R(d, 7, 7, 8, 9, C('#f0d060')); R(d, 2, 9, 13, 9, C('#c8a040'))
    elif n == 'forge': R(d, 2, 9, 13, 14, C('#4a4e5a')); R(d, 3, 6, 12, 9, C('#ff7020')); R(d, 5, 3, 10, 6, C('#ffb030')); R(d, 11, 8, 14, 9, C('#a8a8b0'))
    elif n == 'gate':
        d.arc([1, 1, 14, 20], 180, 360, fill=C('#ff60c0'), width=2); d.arc([3, 3, 12, 20], 180, 360, fill=C('#60e0ff'), width=2); d.arc([5, 5, 10, 20], 180, 360, fill=C('#ffe060'), width=1)
    elif n == 'sauna': R(d, 2, 6, 13, 14, C('#b88040')); d.polygon([(1, 6), (8, 1), (14, 6)], fill=C('#6a4020')); R(d, 6, 9, 9, 14, C('#402810')); R(d, 11, 2, 12, 5, C('#a0a0a8'))
    elif n == 'bunker': R(d, 1, 8, 14, 14, C('#5a5e6a')); R(d, 5, 10, 10, 14, C('#1c1e26')); R(d, 1, 8, 14, 9, C('#7a7e8a'))
    shade(im); outline(im); return im

def crosshair():
    im = canvas(16, 16); d = ImageDraw.Draw(im); c = C('#ffffff', 230); k = C('#101018', 200)
    for (a, b, e, f) in ((7, 1, 8, 4), (7, 11, 8, 14), (1, 7, 4, 8), (11, 7, 14, 8)): R(d, a, b, e, f, c)
    R(d, 7, 7, 8, 8, C('#ff4040')); return outline(im, k)

def small(name):
    if name == 'rocket': im = canvas(10, 5); d = ImageDraw.Draw(im); R(d, 1, 1, 7, 3, C('#c8c8d0')); R(d, 7, 1, 9, 3, C('#e04030')); R(d, 0, 0, 1, 4, C('#ff9030')); return outline(shade(im))
    if name == 'grenade': im = canvas(7, 7); d = ImageDraw.Draw(im); d.ellipse([1, 1, 5, 5], fill=C('#3c7a3c')); R(d, 3, 0, 3, 1, C('#d0d0d8')); return outline(im)
    if name == 'bullet': im = canvas(6, 2); d = ImageDraw.Draw(im); R(d, 0, 0, 3, 1, C('#ffd860')); R(d, 4, 0, 5, 1, C('#ffffff')); return im
    if name == 'particle': im = canvas(3, 3); d = ImageDraw.Draw(im); R(d, 0, 0, 2, 2, C('#ffffff')); return im
