"""Placeholder SFX + music synthesiser (numpy + wave). Replace the .wav/.ogg files in assets/ with real audio any time."""
import wave, numpy as np
SR = 22050
rng = np.random.default_rng(7)

def T(d): return np.arange(int(SR * d)) / SR
def noise(d): return rng.uniform(-1, 1, int(SR * d))
def expenv(d, k=8.0, a=0.002):
    t = T(d); e = np.exp(-k * t / d); n = int(a * SR)
    if n: e[:n] *= np.linspace(0, 1, n)
    return e
def lp(x, fc):
    s = np.fft.rfft(x); f = np.fft.rfftfreq(len(x), 1 / SR); s *= 1 / (1 + (f / fc) ** 4); return np.fft.irfft(s, len(x))
def hp(x, fc): return x - lp(x, fc)
def sweep(f0, f1, d, wave_='sin'):
    t = T(d); f = f0 * (f1 / f0) ** (t / d); ph = 2 * np.pi * np.cumsum(f) / SR
    if wave_ == 'sq': return np.sign(np.sin(ph))
    if wave_ == 'saw': return 2 * ((ph / (2 * np.pi)) % 1) - 1
    return np.sin(ph)
def mix(*xs):
    n = max(len(x) for x in xs); o = np.zeros(n)
    for x in xs: o[:len(x)] += x
    return o
def save(path, x, gain=0.9):
    x = np.asarray(x, dtype=np.float64); pk = np.max(np.abs(x)) or 1; x = np.clip(x / pk * gain, -1, 1)
    with wave.open(path, 'wb') as w:
        w.setnchannels(1); w.setsampwidth(2); w.setframerate(SR); w.writeframes((x * 32767).astype('<i2').tobytes())

# ---------------------------------------------------------------- SFX
def shot(d=.18, body=180, crack=.6, tone=1.0, hi=3500):
    return noise(d) * expenv(d, 14) * crack * 0.9 + lp(noise(d), hi) * expenv(d, 9) * .6 + sweep(body * 1.6, body * .4, d) * expenv(d, 11) * tone
def boom(d=.9, k=5): return lp(noise(d), 700) * expenv(d, k) + sweep(90, 28, d) * expenv(d, 4) * .9
def sfx_table():
    S = {}
    S['shot_pistol'] = shot(.16, 220)
    S['shot_revolver'] = shot(.3, 130, 1.0, 1.3, 2600)
    S['shot_smg'] = shot(.1, 260, .7)
    S['shot_rifle'] = shot(.2, 170, .8, 1.1)
    S['shot_shotgun'] = shot(.35, 100, 1.1, 1.0, 2000)
    S['shot_sniper'] = shot(.5, 90, 1.2, 1.3, 1800) + lp(noise(.5), 500) * expenv(.5, 3) * .3
    S['shot_minigun'] = shot(.07, 240, .6)
    S['shot_flame'] = lp(noise(.25), 1800) * expenv(.25, 3, .05) * .9
    S['shot_ice'] = hp(sweep(2400, 900, .3, 'sq'), 600) * expenv(.3, 6) * .35 + hp(noise(.3), 3000) * expenv(.3, 8) * .4
    S['shot_acid'] = lp(noise(.25), 900) * expenv(.25, 5, .03) * .5 + sweep(300, 900, .25) * expenv(.25, 6) * .4
    S['shot_thunder'] = hp(noise(.35), 1500) * expenv(.35, 9) + sweep(1800, 90, .35, 'saw') * expenv(.35, 7) * .5
    S['shot_rail'] = mix(sweep(3000, 200, .45, 'sq') * expenv(.45, 6) * .4, hp(noise(.2), 4000) * expenv(.2, 15))
    S['shot_rocket'] = lp(noise(.5), 900) * expenv(.5, 2.5, .08) * .8 + sweep(200, 90, .5, 'saw') * expenv(.5, 3) * .4
    S['shot_grenade'] = sweep(180, 70, .22) * expenv(.22, 9) + lp(noise(.22), 600) * expenv(.22, 12) * .6
    S['shot_powder'] = lp(noise(.18), 2500) * expenv(.18, 6, .02) * .6 + sweep(500, 250, .18) * expenv(.18, 8) * .3
    S['shot_trick'] = sweep(300, 1200, .2, 'sq') * expenv(.2, 7) * .5 + sweep(1200, 300, .2) * expenv(.2, 7) * .3
    S['melee'] = hp(noise(.14), 1500) * expenv(.14, 10) * .8 + sweep(600, 200, .14) * expenv(.14, 8) * .3
    S['explosion'] = boom(.8, 5)
    S['explosion_big'] = boom(1.6, 3) + lp(noise(1.6), 250) * expenv(1.6, 2) * .8
    S['hit_flesh'] = lp(noise(.12), 900) * expenv(.12, 10) + sweep(160, 70, .12) * expenv(.12, 8) * .6
    S['hit_wall'] = hp(noise(.08), 2500) * expenv(.08, 14) * .8 + sweep(1500, 700, .08, 'sq') * expenv(.08, 10) * .2
    S['hurt'] = sweep(420, 220, .22, 'saw') * expenv(.22, 7) * .6 + lp(noise(.22), 1200) * expenv(.22, 9) * .3
    S['die'] = sweep(380, 60, .7, 'saw') * expenv(.7, 4) * .7 + lp(noise(.7), 800) * expenv(.7, 5) * .3
    S['jump'] = sweep(180, 360, .14) * expenv(.14, 8) * .5 + lp(noise(.14), 800) * expenv(.14, 8) * .2
    S['land'] = lp(noise(.12), 500) * expenv(.12, 10) + sweep(110, 60, .12) * expenv(.12, 8) * .6
    S['step'] = lp(noise(.06), 700) * expenv(.06, 14) * .6
    S['jet'] = lp(noise(.3), 1400) * (0.7 + 0.3 * np.sin(2 * np.pi * 28 * T(.3))) * .7
    S['pickup'] = np.concatenate([sweep(600, 600, .06), sweep(900, 900, .09)]) * np.concatenate([expenv(.06, 5), expenv(.09, 5)]) * .6
    S['pickup_weapon'] = np.concatenate([sweep(f, f, .07, 'sq') * expenv(.07, 5) for f in (400, 500, 650, 800)]) * .35
    S['reload'] = np.concatenate([hp(noise(.04), 2000) * expenv(.04, 12), np.zeros(int(.1 * SR)), hp(noise(.05), 1500) * expenv(.05, 10) * 1.2])
    S['empty'] = hp(noise(.03), 3000) * expenv(.03, 15) * .6
    S['mission_done'] = np.concatenate([sweep(f, f, .16) * expenv(.16, 3) + sweep(f * 2, f * 2, .16) * expenv(.16, 4) * .3 for f in (392, 494, 587, 784)])
    S['ui_click'] = sweep(900, 700, .04, 'sq') * expenv(.04, 8) * .4
    S['ui_open'] = sweep(300, 700, .12) * expenv(.12, 5) * .4
    S['splash'] = lp(noise(.3), 1500) * expenv(.3, 6, .01) * .7
    S['sizzle'] = hp(noise(.35), 2500) * expenv(.35, 5) * .5
    S['boss_roar'] = sweep(140, 55, 1.4, 'saw') * expenv(1.4, 2.2) * .8 + lp(noise(1.4), 400) * expenv(1.4, 2.5) * .5
    S['thunder_strike'] = mix(hp(noise(.1), 800) * expenv(.1, 12), lp(noise(1.5), 300) * expenv(1.5, 2.5) * .8)
    S['gate'] = sum(sweep(f, f * 1.5, 1.2) * expenv(1.2, 2, .2) * .3 for f in (220, 330, 440, 660))
    S['heal'] = np.concatenate([sweep(f, f, .09) * expenv(.09, 4) for f in (523, 659, 784, 1046)]) * .5
    return S

# ---------------------------------------------------------------- music
def pluck(f, d, amp=1.0, decay=3.0):
    t = T(d); x = np.zeros_like(t)
    for h in range(1, 9):
        x += np.sin(2 * np.pi * f * h * (1 + .0005 * h * h) * t) / h ** 1.15 * np.exp(-t * decay * (1 + .35 * h))
    return x * amp * np.minimum(1, t * 400)
def pad(f, d, amp=1.0):
    t = T(d); x = sum(np.sin(2 * np.pi * f * m * t + p) for m, p in ((1, 0), (1.004, 1), (.996, 2), (2, 3))) / 4
    a = np.minimum(1, t / (d * .3)) * np.minimum(1, (d - t) / (d * .3)); return x * a * amp
def horn(f, d, amp=1.0):
    t = T(d); x = lp(2 * ((f * t) % 1) - 1, 700); a = np.minimum(1, t / .15) * np.minimum(1, (d - t) / .25); return x * a * amp
def kick(): return sweep(130, 38, .35) * expenv(.35, 6)
def frame_drum(): return (sweep(95, 60, .3) * expenv(.3, 7) + lp(noise(.3), 600) * expenv(.3, 12) * .5)
def hat(): return hp(noise(.05), 6000) * expenv(.05, 14) * .5
def bell(f, d=1.5): t = T(d); return (np.sin(2 * np.pi * f * t) + .5 * np.sin(2 * np.pi * f * 2.76 * t)) * np.exp(-t * 2.2) * .5

def hz(m): return 440.0 * 2 ** ((m - 69) / 12)

def compose(name, bpm, beats, bars, scale, root, kind, seed):
    r = np.random.default_rng(seed); spb = 60.0 / bpm; L = bars * beats * spb; tail = 3.0
    buf = np.zeros(int((L + tail) * SR))
    def put(x, tsec, g=1.0):
        i = int(tsec * SR); n = min(len(x), len(buf) - i)
        if n > 0: buf[i:i + n] += x[:n] * g
    deg = lambda k, oct_=0: root + scale[k % len(scale)] + 12 * (k // len(scale)) + 12 * oct_
    for b in range(bars):
        t0 = b * beats * spb; chord = [0, 2, 4] if b % 4 != 3 else [3, 5, 7]
        if kind in ('menu', 'map', 'calm', 'dark', 'kalevala', 'norse'):
            for k in chord: put(pad(hz(deg(k, -1)), beats * spb * 1.05, 1), t0, .16 if kind != 'dark' else .22)
        if kind in ('norse', 'boss', 'dark'):
            put(horn(hz(deg(0 if b % 2 == 0 else 2, -2)), beats * spb * .9), t0, .3)
        # kantele arpeggio
        steps = beats * (2 if kind not in ('map', 'dark') else 1); last = int(r.integers(0, 5))
        for s in range(steps):
            if kind == 'map' and r.random() < .35: continue
            if kind == 'dark' and r.random() < .5: continue
            step = (last + int(r.integers(-2, 3))) % 8; last = step
            put(pluck(hz(deg(step, 0 if kind != 'boss' else -1)), 1.6, 1, 2.2 if kind != 'map' else 1.2), t0 + s * spb * beats / steps, .33 if kind != 'boss' else .38)
        for k in range(beats):
            tt = t0 + k * spb
            if kind in ('norse', 'kalevala', 'boss'):
                if k == 0 or (kind == 'boss' and k % 2 == 0): put(kick() if kind == 'boss' else frame_drum(), tt, .75)
                if kind == 'boss': put(hat(), tt + spb / 2, .5)
                if kind == 'kalevala' and k in (2, 4): put(frame_drum(), tt, .4)
            if kind == 'dark' and k == 0 and b % 2 == 0: put(bell(hz(deg(int(r.integers(2, 6)), 1))), tt, .5)
            if kind == 'calm' and k == 0: put(bell(hz(deg(4, 1)), 2.0), tt, .25)
    out = buf[:int(L * SR)].copy(); tl = buf[int(L * SR):]; out[:len(tl)] += tl   # wrap tail => seamless loop
    return out

MINOR_PENT = [0, 3, 5, 7, 10]; MAJOR_PENT = [0, 2, 4, 7, 9]; DORIAN = [0, 2, 3, 5, 7, 9, 10]; PHRYG = [0, 1, 3, 5, 7, 8, 10]; AEOL = [0, 2, 3, 5, 7, 8, 10]
KAL = [0, 2, 4, 7, 9]  # kantele pentachord feel
def music_table():
    return {
     'menu': compose('menu', 76, 4, 8, DORIAN, 57, 'menu', 1),
     'map': compose('map', 56, 4, 6, MINOR_PENT, 52, 'map', 2),
     'calm': compose('calm', 88, 4, 8, MAJOR_PENT, 55, 'calm', 3),
     'norse': compose('norse', 112, 4, 12, AEOL, 50, 'norse', 4),
     'kalevala': compose('kalevala', 104, 5, 10, KAL, 50, 'kalevala', 5),   # 5/4: Kalevala runo meter
     'dark': compose('dark', 64, 4, 8, PHRYG, 45, 'dark', 6),
     'boss': compose('boss', 150, 4, 14, PHRYG, 43, 'boss', 7),
    }
