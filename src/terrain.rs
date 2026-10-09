//! Procedural planet generator: terrain, caves, ores, seas, trees, structures, zones and hotspots.
use crate::elements::*;
use crate::hotspot::{HotKind, Hotspot};
use crate::planets::PlanetDef;
use crate::sim::World;

#[derive(Clone, Debug)]
pub struct Zone { pub name: &'static str, pub x0: i32, pub y0: i32, pub x1: i32, pub y1: i32, pub initial: usize }
impl Zone {
    pub fn contains(&self, x: f32, y: f32) -> bool { x >= self.x0 as f32 && x <= self.x1 as f32 && y >= self.y0 as f32 && y <= self.y1 as f32 }
    pub fn center(&self) -> (f32, f32) { ((self.x0 + self.x1) as f32 / 2.0, (self.y0 + self.y1) as f32 / 2.0) }
}

pub struct Generated {
    pub spawn: (i32, i32),
    pub hotspots: Vec<Hotspot>,
    pub zones: Vec<Zone>,
    pub enemy_spawns: Vec<(i32, i32)>,
    pub shard_spots: Vec<(i32, i32)>,
    pub surface: Vec<i32>,
}
impl Generated { pub fn zone(&self, name: &str) -> Option<&Zone> { self.zones.iter().find(|z| z.name == name) } }

// ---------------------------------------------------------------- noise
fn hash(x: i32, y: i32, seed: u32) -> f32 {
    let mut h = (x as u32).wrapping_mul(0x27d4eb2d) ^ (y as u32).wrapping_mul(0x165667b1) ^ seed.wrapping_mul(0x9e3779b9);
    h ^= h >> 15; h = h.wrapping_mul(0x85ebca6b); h ^= h >> 13; h = h.wrapping_mul(0xc2b2ae35); h ^= h >> 16;
    (h & 0xffff) as f32 / 65535.0
}
fn vnoise(x: f32, y: f32, seed: u32) -> f32 {
    let (xi, yi) = (x.floor() as i32, y.floor() as i32); let (fx, fy) = (x - xi as f32, y - yi as f32);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let a = hash(xi, yi, seed); let b = hash(xi + 1, yi, seed); let c = hash(xi, yi + 1, seed); let d = hash(xi + 1, yi + 1, seed);
    (a + (b - a) * sx) + ((c + (d - c) * sx) - (a + (b - a) * sx)) * sy
}
fn fbm(x: f32, y: f32, seed: u32, oct: u32) -> f32 {
    let (mut v, mut a, mut f, mut n) = (0.0, 0.5, 1.0, 0.0);
    for o in 0..oct { v += vnoise(x * f, y * f, seed + o * 31) * a; n += a; a *= 0.5; f *= 2.0; }
    v / n
}

struct Rng(u32);
impl Rng {
    fn next(&mut self) -> u32 { let mut x = self.0; x ^= x << 13; x ^= x >> 17; x ^= x << 5; self.0 = x; x }
    fn range(&mut self, a: i32, b: i32) -> i32 { if b <= a { a } else { a + (self.next() % (b - a + 1) as u32) as i32 } }
    fn chance(&mut self, pct: u32) -> bool { self.next() % 100 < pct }
}

fn rect(w: &mut World, x0: i32, y0: i32, x1: i32, y1: i32, el: u8) { for y in y0..=y1 { for x in x0..=x1 { w.set(x, y, el); } } }
fn rect_bg(w: &mut World, x0: i32, y0: i32, x1: i32, y1: i32) { for y in y0..=y1 { for x in x0..=x1 { if w.inb(x, y) { let i = w.idx(x, y); w.bg[i] = 1; } } } }

pub fn generate(w: &mut World, p: &PlanetDef) -> Generated {
    let g = &p.gen; let (ww, hh) = (w.w as i32, w.h as i32);
    w.clear_all(); w.seed(p.seed.wrapping_mul(7919) | 1);
    let seed = p.seed; let mut rng = Rng(seed.wrapping_mul(2654435761) | 1);
    let mut surface = vec![0i32; ww as usize];
    for x in 0..ww {
        let n = fbm(x as f32 * 0.006, 0.5, seed, 4) - 0.5; let m = fbm(x as f32 * 0.04, 3.3, seed + 5, 2) - 0.5;
        surface[x as usize] = (g.base * hh as f32 + n * 2.0 * g.amp + m * g.amp * 0.35) as i32;
    }
    // --- fill ground
    for x in 2..ww - 2 {
        let s = surface[x as usize];
        for y in s.max(2)..hh - 2 {
            let depth = y - s; let deep_start = 38 + (fbm(x as f32 * 0.02, 9.0, seed + 2, 2) * 24.0) as i32;
            let el = if depth < g.top_depth { g.top } else if depth < deep_start { g.sub } else { g.deep };
            w.set(x, y, el);
            if depth > 1 { let i = w.idx(x, y); w.bg[i] = 1; }
        }
    }
    // --- islands: floating chunks above the floor
    if g.islands {
        for x in 2..ww - 2 { for y in (hh as f32 * 0.18) as i32..(hh as f32 * 0.78) as i32 {
            let n = fbm(x as f32 * 0.012, y as f32 * 0.035, seed + 70, 3);
            let band = 1.0 - ((y as f32 / hh as f32 - 0.48).abs() * 2.2).min(1.0);
            if n * (0.55 + band * 0.7) > 0.56 && w.get(x, y) == EMPTY {
                let above_empty = (1..5).all(|k| fbm(x as f32 * 0.012, (y - k) as f32 * 0.035, seed + 70, 3) * (0.55 + band * 0.7) <= 0.56);
                w.set(x, y, if above_empty { g.top } else { g.sub });
                let i = w.idx(x, y); w.bg[i] = 1;
            }
        } }
    }
    // --- caves
    if g.caves > 0.0 {
        let thr = 0.735 - g.caves * 0.26;
        for x in 6..ww - 6 { for y in 6..hh - 8 {
            let depth = y - surface[x as usize];
            if depth < 12 { continue; }
            let n = fbm(x as f32 * 0.028, y as f32 * 0.04, seed + 91, 3);
            if n > thr && w.get(x, y) != EMPTY { w.set(x, y, EMPTY); }
        } }
    }
    // --- craters
    if g.craters {
        for _ in 0..14 {
            let cx = rng.range(40, ww - 40); let r = rng.range(8, 24); let cy = surface[cx as usize];
            for dy in -r..=r { for dx in -r..=r { if dx * dx + dy * dy * 2 <= r * r && dy <= r / 3 { let (x, y) = (cx + dx, cy + dy + r / 4); if w.inb(x, y) && x > 2 && x < ww - 2 && y > 2 { w.set(x, y, EMPTY); } } } }
        }
        for x in 2..ww - 2 { for y in 2..hh - 3 { if w.get(x, y) == EMPTY && w.get(x, y + 1) != EMPTY && y > surface[x as usize] - 30 { /* keep */ } } }
    }
    // --- ores
    for &(el, per) in g.ores {
        let n = (ww as u32 * hh as u32 * per / 10000) / 12;
        for _ in 0..n {
            let cx = rng.range(10, ww - 10); let cy = rng.range(10, hh - 12); let r = rng.range(2, 5);
            if cy < surface[cx as usize] + 12 && !g.islands { continue; }
            for dy in -r..=r { for dx in -r..=r {
                if dx * dx + dy * dy <= r * r + 1 { let (x, y) = (cx + dx, cy + dy); let e = w.get(x, y); if x > 2 && y > 2 && x < ww - 2 && y < hh - 2 && matches!(cat(e), Cat::Solid) && e != WALL { w.set(x, y, el); } else if (el == WATER || el == SALT) && e == EMPTY && w.bg[w.idx(x, y)] == 1 && (y - surface[x as usize]) > 14 { w.set(x, y, el); } }
            } }
        }
    }
    // --- sea / open-air liquid
    if let Some((el, lvl)) = g.sea {
        let ly = (lvl * hh as f32) as i32;
        for x in 2..ww - 2 { for y in ly..surface[x as usize].min(hh - 2) { if w.get(x, y) == EMPTY { w.set(x, y, el); } } }
    }
    // --- cave pools / gas pockets
    if let Some(el) = g.cave_pool {
        let ly = (hh as f32 * 0.78) as i32;
        for x in 6..ww - 6 { for y in ly..hh - 3 { if w.get(x, y) == EMPTY && y > surface[x as usize] + 10 { w.set(x, y, el); } } }
    }
    if g.cave_gas {
        for x in 6..ww - 6 { for y in 6..hh - 8 { if w.get(x, y) == EMPTY && y > surface[x as usize] + 14 && w.bg[w.idx(x, y)] == 1 && fbm(x as f32 * 0.05, y as f32 * 0.07, seed + 8, 2) > 0.62 { w.set(x, y, METHANE); } } }
    }
    // --- layout
    let fx = |f: f32| -> i32 { (f * ww as f32) as i32 };
    let mut hotspots: Vec<Hotspot> = Vec::new(); let mut zones: Vec<Zone> = Vec::new();
    let sea_y = g.sea.map(|(_, l)| (l * hh as f32) as i32).unwrap_or(hh);

    // flatten a stretch and return the ground y
    let flatten = |w: &mut World, surface: &mut Vec<i32>, x0: i32, x1: i32, rise: i32| -> i32 {
        let mut sum = 0; for x in x0..=x1 { sum += surface[x as usize]; }
        let mut y = sum / (x1 - x0 + 1) - rise;
        if g.sea.is_some() { y = y.min(sea_y - 5); }
        if g.islands { y = y.max(hh / 2); }
        for x in x0..=x1 {
            for yy in (y - 60).max(3)..y { w.set(x, yy, EMPTY); }
            for yy in y..surface[x as usize].max(y) + 1 { if yy < hh - 2 { let d = yy - y; w.set(x, yy, if d < g.top_depth { g.top } else { g.sub }); } }
            if surface[x as usize] > y + 0 { for yy in y..=surface[x as usize] { if yy < hh - 2 { let i = w.idx(x, yy); w.bg[i] = if yy > y + 1 { 1 } else { 0 }; } } }
            surface[x as usize] = y;
        }
        y
    };
    // spawn
    let sx0 = fx(0.04); let sx1 = fx(0.075); let sy = flatten(w, &mut surface, sx0, sx1, 0);
    let spawn = ((sx0 + sx1) / 2, sy - 2);
    let mut place = |name: &'static str, f: f32, half: i32, w: &mut World, surface: &mut Vec<i32>| -> (i32, i32) { let cx = fx(f); let y = flatten(w, surface, cx - half, cx + half, 0); let _ = name; (cx, y) };

    // gate (asgard / kalevala)
    if p.gate { let (cx, y) = place("gate", 0.10, 14, w, &mut surface); build_gate(w, cx, y); hotspots.push(Hotspot { kind: HotKind::Gate, x: cx as f32, y: (y - 1) as f32, used: false }); }
    // spring
    { let (cx, y) = place("spring", 0.17, 10, w, &mut surface); rect(w, cx - 8, y, cx + 8, y + 5, STONE); rect(w, cx - 6, y, cx + 6, y + 3, EMPTY); rect(w, cx - 6, y + 1, cx + 6, y + 3, WATER); rect(w, cx - 8, y - 2, cx - 8, y - 1, STONE); rect(w, cx + 8, y - 2, cx + 8, y - 1, STONE); hotspots.push(Hotspot { kind: HotKind::Spring, x: cx as f32, y: (y - 1) as f32, used: false }); }
    // forge (or sauna on kalevala)
    { let (cx, y) = place("forge", 0.24, 12, w, &mut surface); rect(w, cx - 9, y - 6, cx + 9, y, STONE); rect(w, cx - 3, y - 6, cx + 3, y - 4, LAVA); rect(w, cx - 9, y - 7, cx + 9, y - 7, METAL); rect(w, cx + 6, y - 14, cx + 8, y - 8, BRICK);
      hotspots.push(Hotspot { kind: HotKind::Forge, x: cx as f32 - 6.0, y: (y - 8) as f32, used: false }); }
    if p.sauna { let (cx, y) = place("sauna", 0.30, 12, w, &mut surface); build_hut(w, cx, y, 22, 14); hotspots.push(Hotspot { kind: HotKind::Sauna, x: cx as f32, y: (y - 1) as f32, used: false }); }
    // chest hut
    { let (cx, y) = place("chest", 0.35, 14, w, &mut surface); build_hut(w, cx, y, 22, 13); hotspots.push(Hotspot { kind: HotKind::Chest, x: cx as f32, y: (y - 1) as f32, used: false }); }
    // rune stones
    for f in [0.41, 0.63] { let (cx, y) = place("rune", f, 6, w, &mut surface); rect(w, cx - 2, y - 11, cx + 2, y, STONE); rect(w, cx - 1, y - 9, cx + 1, y - 3, RUNE); hotspots.push(Hotspot { kind: HotKind::Rune, x: cx as f32 + 5.0, y: (y - 1) as f32, used: false }); }
    // basin (powder puzzle: fill with water)
    { let (cx, y) = place("basin", 0.50, 26, w, &mut surface);
      let (bx0, bx1) = (cx - 20, cx + 20); rect(w, bx0, y - 8, bx1, y + 18, STONE); rect(w, bx0 + 4, y - 8, bx1 - 4, y + 14, EMPTY);
      rect_bg(w, bx0 + 4, y + 2, bx1 - 4, y + 14);
      zones.push(Zone { name: "basin", x0: bx0 + 4, y0: y - 6, x1: bx1 - 4, y1: y + 14, initial: 0 }); }
    // bunker (underground cache; dig down!)
    { let cx = fx(0.56); let y = surface[cx as usize]; let ry = (y + 36).min(hh - 24);
      rect(w, cx - 14, ry - 1, cx + 14, ry + 13, BRICK); rect(w, cx - 12, ry + 1, cx + 12, ry + 11, EMPTY); rect_bg(w, cx - 12, ry + 1, cx + 12, ry + 11);
      rect(w, cx - 2, y, cx + 2, ry, DIRT); rect(w, cx - 1, ry - 1, cx + 1, ry + 2, EMPTY);
      for yy in y - 1..ry { let i = w.idx(cx, yy); w.bg[i] = 1; }
      rect(w, cx - 1, y - 1, cx + 1, y + 2, DIRT);
      hotspots.push(Hotspot { kind: HotKind::Bunker, x: cx as f32 + 8.0, y: (ry + 11) as f32, used: false });
      zones.push(Zone { name: "bunker", x0: cx - 12, y0: ry + 1, x1: cx + 12, y1: ry + 11, initial: 0 }); }
    // nest: flammable hive for burn missions
    { let (cx, y) = place("nest", 0.70, 24, w, &mut surface);
      for k in 0..3 { build_hut(w, cx - 16 + k * 16, y, 14, 12 - k); }
      for x in cx - 22..=cx + 22 { if rng.chance(55) { let h = rng.range(2, 6); rect(w, x, y - h, x, y - 1, PLANT); } }
      rect(w, cx - 22, y - 20, cx + 22, y - 18, WOOD); // beam across the roofs
      let (x0, y0, x1, y1) = (cx - 24, y - 24, cx + 24, y);
      let n = w.count_in(x0, y0, x1, y1, |e| e == WOOD || e == PLANT);
      zones.push(Zone { name: "nest", x0, y0, x1, y1, initial: n }); }
    // second chest + arena
    { let (cx, y) = place("chest2", 0.76, 10, w, &mut surface); rect(w, cx - 6, y - 2, cx + 6, y, STONE); hotspots.push(Hotspot { kind: HotKind::Chest, x: cx as f32, y: (y - 3) as f32, used: false }); }
    { let (cx, y) = place("arena", 0.84, 50, w, &mut surface);
      rect(w, cx - 52, y - 40, cx - 49, y, STONE); rect(w, cx + 49, y - 40, cx + 52, y, STONE);
      zones.push(Zone { name: "arena", x0: cx - 48, y0: y - 60, x1: cx + 48, y1: y, initial: 0 }); }
    // goal: rune beacon
    { let (cx, y) = place("goal", 0.95, 14, w, &mut surface);
      rect(w, cx - 4, y - 3, cx + 4, y, BRICK); rect(w, cx - 2, y - 26, cx + 2, y - 4, STONE); rect(w, cx - 1, y - 24, cx + 1, y - 6, RUNE);
      zones.push(Zone { name: "goal", x0: cx - 22, y0: y - 40, x1: cx + 22, y1: y, initial: 0 }); }
    // --- trees on the surface
    if g.trees > 0 {
        let mut x = 90;
        while x < ww - 90 {
            let s = surface[x as usize];
            let ok = (g.sea.is_none() || s < sea_y - 1) && matches!(w.get(x, s), DIRT | SNOW | ASH | STONE | SAND) && w.get(x, s - 1) == EMPTY && !zones.iter().any(|z| x > z.x0 - 30 && x < z.x1 + 30) && !hotspots.iter().any(|h| (h.x - x as f32).abs() < 24.0);
            if ok { build_tree(w, x, s, g.trees, &mut rng); }
            x += rng.range(34, 70);
        }
    }
    // --- spawns
    let mut enemy_spawns = Vec::new();
    for _ in 0..60 {
        let x = rng.range(fx(0.14), ww - 40); let s = surface[x as usize];
        if g.sea.is_some() && s >= sea_y - 2 { continue; }
        if zones.iter().any(|z| z.name == "basin" && x > z.x0 - 6 && x < z.x1 + 6) { continue; }
        enemy_spawns.push((x, s - 14));
    }
    for _ in 0..400 { // cave spots
        if enemy_spawns.len() > 90 { break; }
        let x = rng.range(20, ww - 20); let y = rng.range(surface[x as usize] + 20, hh - 20);
        if w.get(x, y) == EMPTY && w.get(x, y - 1) == EMPTY && w.get(x, y - 2) == EMPTY && w.get(x, y + 1) != EMPTY && w.get(x, y + 1) != WATER && is_solid_for_body(w.get(x, y + 1)) { enemy_spawns.push((x, y)); }
    }
    if g.islands { // enemies on islands
        for _ in 0..200 { if enemy_spawns.len() > 110 { break; } let x = rng.range(60, ww - 60); let y = rng.range(hh / 5, hh * 3 / 4); if w.get(x, y) == EMPTY && w.get(x, y + 1) != EMPTY && w.get(x, y - 8) == EMPTY && is_solid_for_body(w.get(x, y + 1)) { enemy_spawns.push((x, y - 10)); } }
    }
    let mut shard_spots = Vec::new();
    for f in [0.22, 0.31, 0.45, 0.58, 0.66, 0.78, 0.88] { let x = fx(f) + rng.range(-6, 6); shard_spots.push((x, surface[x as usize] - 6)); }
    // cave shard spots
    for _ in 0..600 { if shard_spots.len() >= 11 { break; } let x = rng.range(30, ww - 30); let y = rng.range(surface[x as usize] + 25, hh - 20); if w.get(x, y) == EMPTY && w.get(x, y + 1) != EMPTY && is_solid_for_body(w.get(x, y + 1)) { shard_spots.push((x, y - 3)); } }
    // keep spawn area safe
    enemy_spawns.retain(|&(x, _)| (x - spawn.0).abs() > 80);
    w.border();
    Generated { spawn, hotspots, zones, enemy_spawns, shard_spots, surface }
}

fn build_hut(w: &mut World, cx: i32, y: i32, wd: i32, h: i32) {
    let (x0, x1) = (cx - wd / 2, cx + wd / 2);
    rect(w, x0, y - h, x1, y - 1, WOOD); rect(w, x0 + 2, y - h + 2, x1 - 2, y - 1, EMPTY); rect_bg(w, x0 + 2, y - h + 2, x1 - 2, y - 1);
    rect(w, x0, y - 8, x0 + 1, y - 1, EMPTY); // door
    for k in 0..(wd / 2 + 2) { let yy = y - h - 1 - k / 2; rect(w, x0 - 1 + k, yy, x1 + 1 - k, yy, WOOD); if x0 - 1 + k > x1 + 1 - k { break; } }
}
fn build_gate(w: &mut World, cx: i32, y: i32) {
    rect(w, cx - 10, y - 2, cx + 10, y, METAL); rect(w, cx - 10, y - 24, cx - 8, y - 3, METAL); rect(w, cx + 8, y - 24, cx + 10, y - 3, METAL); rect(w, cx - 10, y - 26, cx + 10, y - 25, METAL);
    rect(w, cx - 7, y - 21, cx + 7, y - 3, EMPTY); rect(w, cx - 1, y - 27, cx + 1, y - 27, RUNE); rect(w, cx - 12, y - 4, cx - 11, y - 3, RUNE); rect(w, cx + 11, y - 4, cx + 12, y - 3, RUNE);
}
fn build_tree(w: &mut World, x: i32, s: i32, kind: u8, rng: &mut Rng) {
    match kind {
        1 | 4 => { let tall = if kind == 4 { rng.range(26, 40) } else { rng.range(14, 24) }; let th = if kind == 4 { 2 } else { 1 };
            rect(w, x - th, s - tall, x + th, s - 1, WOOD); let r = if kind == 4 { rng.range(9, 13) } else { rng.range(6, 10) };
            for dy in -r..=r { for dx in -r..=r { if dx * dx + dy * dy * 2 <= r * r + 4 && w.get(x + dx, s - tall + dy) == EMPTY { w.set(x + dx, s - tall + dy, PLANT); } } } }
        2 => { let tall = rng.range(18, 32); rect(w, x, s - tall, x, s - 1, WOOD);
            let mut wd = 1; let mut y = s - tall; while y < s - 5 { rect(w, x - wd, y, x + wd, y, PLANT); y += 1; if (y - (s - tall)) % 3 == 0 { wd += 1; } if wd > 7 { wd = 7; } } }
        _ => { let tall = rng.range(12, 22); rect(w, x, s - tall, x, s - 1, WOOD);
            for k in 0..3 { let by = s - tall + k * 4; let dir = if k % 2 == 0 { 1 } else { -1 }; for j in 1..=5 { w.set(x + dir * j, by - j / 2, WOOD); } } }
    }
}
