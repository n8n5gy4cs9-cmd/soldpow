//! Falling-sand simulation (Powder Toy side). Original implementation, rules inspired by the genre.
use crate::elements::*;

#[derive(Clone, Copy, Default)]
pub struct Cell { pub el: u8, pub life: u8, pub aux: u8, pub var: u8, pub upd: u8 }

#[derive(Clone, Copy, Debug)]
pub enum SimEvent { Explosion { x: i32, y: i32, r: i32 }, Splash { x: i32, y: i32 }, Sizzle { x: i32, y: i32 } }

pub struct World {
    pub w: usize,
    pub h: usize,
    pub cells: Vec<Cell>,
    /// 1 = rock backdrop (cave wall) shown behind empty cells.
    pub bg: Vec<u8>,
    pub frame: u32,
    pub events: Vec<SimEvent>,
    rng: u32,
    cur: u8,
    pending: Vec<(i32, i32, i32)>,
}

impl World {
    pub fn new(w: usize, h: usize) -> Self {
        let mut world = World { w, h, cells: vec![Cell::default(); w * h], bg: vec![0; w * h], frame: 0, events: Vec::new(), rng: 0x2545F491, cur: 0, pending: Vec::new() };
        world.border();
        world
    }
    pub fn border(&mut self) {
        let (w, h) = (self.w as i32, self.h as i32);
        for x in 0..w { for y in [0, 1, h - 2, h - 1] { self.set(x, y, WALL); } }
        for y in 0..h { for x in [0, 1, w - 2, w - 1] { self.set(x, y, WALL); } }
    }
    pub fn clear_all(&mut self) { for c in self.cells.iter_mut() { *c = Cell::default(); } for b in self.bg.iter_mut() { *b = 0; } self.border(); }
    /// Remove all powder/fluids but keep solid terrain.
    pub fn clear_loose(&mut self) {
        for c in self.cells.iter_mut() { if matches!(cat(c.el), Cat::Powder | Cat::Liquid | Cat::Gas) || c.el == THUNDER { *c = Cell::default(); } }
    }
    #[inline] pub fn rnd(&mut self) -> u32 { let mut x = self.rng; x ^= x << 13; x ^= x >> 17; x ^= x << 5; self.rng = x; x }
    pub fn seed(&mut self, s: u32) { self.rng = s | 1; }
    #[inline] pub fn inb(&self, x: i32, y: i32) -> bool { x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h }
    #[inline] pub fn idx(&self, x: i32, y: i32) -> usize { y as usize * self.w + x as usize }
    #[inline] pub fn get(&self, x: i32, y: i32) -> u8 { if self.inb(x, y) { self.cells[self.idx(x, y)].el } else { WALL } }
    #[inline] pub fn cell(&self, x: i32, y: i32) -> Cell { if self.inb(x, y) { self.cells[self.idx(x, y)] } else { Cell { el: WALL, ..Default::default() } } }
    pub fn new_cell(&mut self, el: u8) -> Cell {
        let r = self.rnd();
        let life = match el {
            FIRE => 30 + (r >> 8) % 40,
            SMOKE => 40 + (r >> 8) % 60,
            STEAM => 120 + (r >> 8) % 120,
            THUNDER => 5 + (r >> 8) % 5,
            _ => 0,
        } as u8;
        Cell { el, life, aux: 0, var: (r >> 16) as u8, upd: self.cur }
    }
    pub fn set(&mut self, x: i32, y: i32, el: u8) {
        if !self.inb(x, y) { return; }
        let c = self.new_cell(el); let i = self.idx(x, y); self.cells[i] = c;
    }
    pub fn fire_at(&mut self, x: i32, y: i32, base: u32, range: u32) { let l = (base + self.rnd() % range.max(1)).min(255) as u8; self.set_aux(x, y, FIRE, 0, Some(l)); }
    pub fn set_aux(&mut self, x: i32, y: i32, el: u8, aux: u8, life: Option<u8>) {
        self.set(x, y, el);
        if self.inb(x, y) { let i = self.idx(x, y); self.cells[i].aux = aux; if let Some(l) = life { self.cells[i].life = l; } }
    }
    #[inline] fn swap_i(&mut self, a: usize, b: usize) { self.cells.swap(a, b); self.cells[a].upd = self.cur; self.cells[b].upd = self.cur; }
    fn swap_xy(&mut self, x: i32, y: i32, nx: i32, ny: i32) { let (a, b) = (self.idx(x, y), self.idx(nx, ny)); self.swap_i(a, b); }

    pub fn is_empty(&self, x: i32, y: i32) -> bool { self.get(x, y) == EMPTY }
    /// Obstacle for creature bodies.
    pub fn solid_at(&self, x: i32, y: i32) -> bool { is_solid_for_body(self.get(x, y)) }

    // ------------------------------------------------------------------ painting
    pub fn paint(&mut self, cx: i32, cy: i32, el: u8, r: i32) {
        for dy in -r..=r { for dx in -r..=r {
            if dx * dx + dy * dy > r * r + r / 2 { continue; }
            self.paint_cell(cx + dx, cy + dy, el);
        } }
    }
    pub fn paint_cell(&mut self, x: i32, y: i32, el: u8) {
        if x < 2 || y < 2 || x >= self.w as i32 - 2 || y >= self.h as i32 - 2 { return; }
        let cur = self.get(x, y);
        if el == EMPTY { if cur != EMPTY { self.set(x, y, EMPTY); } return; }
        if el == CLONE && cur != EMPTY && cur != CLONE && cur != WALL && cur != VOID { self.set_aux(x, y, CLONE, cur, None); return; }
        let loose = |e: u8| e == EMPTY || matches!(cat(e), Cat::Gas);
        let ok = match cat(el) {
            Cat::Solid | Cat::Special => loose(cur) || matches!(cat(cur), Cat::Powder | Cat::Liquid),
            _ => loose(cur) || (cat(el) == Cat::Gas && false),
        };
        if ok { self.set(x, y, el); }
    }
    pub fn paint_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, el: u8, r: i32) {
        let n = ((x1 - x0).abs().max((y1 - y0).abs())).max(1);
        for i in 0..=n { self.paint(x0 + (x1 - x0) * i / n, y0 + (y1 - y0) * i / n, el, r); }
    }
    pub fn count_in(&self, x0: i32, y0: i32, x1: i32, y1: i32, pred: impl Fn(u8) -> bool) -> usize {
        let mut n = 0;
        for y in y0.max(0)..=y1.min(self.h as i32 - 1) { for x in x0.max(0)..=x1.min(self.w as i32 - 1) { if pred(self.cells[self.idx(x, y)].el) { n += 1; } } }
        n
    }

    // ------------------------------------------------------------------ explosions
    pub fn queue_explosion(&mut self, x: i32, y: i32, r: i32) { self.pending.push((x, y, r)); }
    fn resist(el: u8) -> f32 {
        match el { WALL | RUNE => 0.0, METAL | GOLD => 0.4, CLONE | VOID => 0.4, BRICK => 0.6, STONE => 0.75, GLASS => 0.9, _ => 1.0 }
    }
    /// Instant blast: clears cells in a radius (hard materials survive near the rim), leaves fire/smoke.
    pub fn explode(&mut self, cx: i32, cy: i32, r: i32) {
        self.events.push(SimEvent::Explosion { x: cx, y: cy, r });
        let rf = r as f32;
        for dy in -r..=r { for dx in -r..=r {
            let d = ((dx * dx + dy * dy) as f32).sqrt();
            if d > rf { continue; }
            let (x, y) = (cx + dx, cy + dy);
            if !self.inb(x, y) { continue; }
            let el = self.get(x, y);
            if el == EMPTY { if d < rf * 0.7 && self.rnd() % 6 == 0 { self.fire_at(x, y, 10, 15); } continue; }
            if d > rf * Self::resist(el) { continue; }
            if x < 2 || y < 2 || x >= self.w as i32 - 2 || y >= self.h as i32 - 2 { continue; }
            if is_explosive(el) && d > 1.0 { let rr = Self::blast_radius(el); self.pending.push((x, y, rr)); self.set(x, y, EMPTY); continue; }
            let roll = self.rnd() % 100;
            if d < rf * 0.5 { if roll < 30 { self.fire_at(x, y, 8, 20); } else if roll < 45 { self.set(x, y, SMOKE); } else { self.set(x, y, EMPTY); } }
            else if roll < 25 { self.fire_at(x, y, 8, 20); } else { self.set(x, y, EMPTY); }
        } }
    }
    pub fn blast_radius(el: u8) -> i32 { match el { GUNPOWDER => 3, METHANE => 3, NITRO => 7, TNT => 15, _ => 4 } }

    /// Ignite whatever is at (x,y) if flammable.
    pub fn ignite(&mut self, x: i32, y: i32) {
        let c = self.cell(x, y);
        match c.el {
            WOOD => { let l = (150 + self.rnd() % 90) as u8; self.set_aux(x, y, FIRE, WOOD, Some(l)); }
            PLANT => { let l = (70 + self.rnd() % 50) as u8; self.set_aux(x, y, FIRE, PLANT, Some(l)); }
            OIL => { let l = (50 + self.rnd() % 40) as u8; self.set_aux(x, y, FIRE, OIL, Some(l)); }
            COAL => { self.set_aux(x, y, FIRE, COAL, Some(240)); }
            GUNPOWDER | NITRO | METHANE | TNT => { self.set(x, y, EMPTY); self.pending.push((x, y, Self::blast_radius(c.el))); }
            SNOW | ICE => { self.set(x, y, WATER); }
            _ => {}
        }
    }
    fn burn_chance(el: u8) -> u32 { // out of 256 per adjacent-fire per frame
        match el { WOOD => 8, PLANT => 10, OIL => 70, COAL => 2, GUNPOWDER => 255, METHANE => 220, NITRO => 255, TNT => 80, SNOW => 14, ICE => 6, _ => 0 }
    }

    // ------------------------------------------------------------------ main step
    pub fn step(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        self.cur ^= 1;
        let (w, h) = (self.w as i32, self.h as i32);
        let ltr = self.frame & 1 == 0;
        for y in (2..h - 2).rev() {
            for k in 2..w - 2 {
                let x = if ltr { k } else { w - 1 - k };
                let i = self.idx(x, y);
                let c = self.cells[i];
                if c.el == EMPTY || c.upd == self.cur { continue; }
                self.cells[i].upd = self.cur;
                self.update(x, y, c);
            }
        }
        // chain reactions: process a bounded number of queued explosions per frame
        let mut n = 0;
        while let Some((x, y, r)) = self.pending.pop() { self.explode(x, y, r); n += 1; if n > 40 { break; } }
        if self.pending.len() > 400 { self.pending.truncate(400); }
    }

    fn update(&mut self, x: i32, y: i32, c: Cell) {
        match c.el {
            SAND | DIRT | ASH | COAL | GUNPOWDER => self.powder(x, y, c.el),
            SALT => { self.powder(x, y, SALT); if self.rnd() % 35 == 0 && self.near4(x, y, WATER) { self.set(x, y, EMPTY); } }
            SNOW => { if self.rnd() % 4 != 0 { self.powder(x, y, SNOW); } }
            WATER => { self.liquid(x, y, WATER, 5); self.react_water(x, y); }
            OIL => self.liquid(x, y, OIL, 3),
            BLOOD => { self.liquid(x, y, BLOOD, 2); if self.rnd() % 4000 == 0 { self.set(x, y, EMPTY); } }
            MERCURY => self.liquid(x, y, MERCURY, 3),
            NITRO => self.liquid(x, y, NITRO, 3),
            ACID => { self.liquid(x, y, ACID, 4); self.react_acid(x, y); }
            LAVA => { if self.rnd() % 3 == 0 { self.liquid(x, y, LAVA, 2); } self.react_lava(x, y); }
            SMOKE => { if c.life == 0 { self.set(x, y, EMPTY); return; } { let i = self.idx(x, y); self.cells[i].life -= 1; } if self.rnd() % 4 != 0 { self.gas(x, y, SMOKE); } }
            STEAM => self.steam(x, y, c),
            METHANE => { if self.rnd() % 2 == 0 { self.gas(x, y, METHANE); } }
            FIRE => self.fire(x, y, c),
            PLANT => { if self.rnd() % 90 == 0 { self.grow(x, y); } }
            ICE => { if self.rnd() % 400 == 0 { self.freeze_neighbor(x, y); } }
            CLONE => self.clone_el(x, y, c),
            VOID => { for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { let e = self.get(x + dx, y + dy); if e != EMPTY && e != WALL && e != VOID && e != RUNE { self.set(x + dx, y + dy, EMPTY); } } }
            THUNDER => {
                if c.life == 0 { self.set(x, y, EMPTY); return; }
                { let i = self.idx(x, y); self.cells[i].life -= 1; }
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let e = self.get(x + dx, y + dy);
                    if Self::burn_chance(e) > 0 && self.rnd() % 3 == 0 { self.ignite(x + dx, y + dy); }
                    if e == WATER && self.rnd() % 8 == 0 { self.set(x + dx, y + dy, STEAM); }
                }
            }
            _ => {}
        }
    }

    fn near4(&self, x: i32, y: i32, el: u8) -> bool { [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().any(|(dx, dy)| self.get(x + dx, y + dy) == el) }
    #[inline] fn can_sink(&self, d: i16, target: u8) -> bool { target == EMPTY || (is_fluid(target) && density(target) < d) }
    #[inline] fn can_rise(&self, d: i16, target: u8) -> bool { target == EMPTY || (is_fluid(target) && density(target) > d) }

    fn powder(&mut self, x: i32, y: i32, el: u8) {
        let d = density(el);
        if self.can_sink(d, self.get(x, y + 1)) { self.swap_xy(x, y, x, y + 1); return; }
        let dir = if self.rnd() & 1 == 0 { -1 } else { 1 };
        for s in [dir, -dir] {
            let side = self.get(x + s, y);
            if (side == EMPTY || is_fluid(side)) && self.can_sink(d, self.get(x + s, y + 1)) { self.swap_xy(x, y, x + s, y + 1); return; }
        }
    }
    fn liquid(&mut self, x: i32, y: i32, el: u8, spread: i32) {
        let d = density(el);
        if self.can_sink(d, self.get(x, y + 1)) { self.swap_xy(x, y, x, y + 1); return; }
        let dir = if self.rnd() & 1 == 0 { -1 } else { 1 };
        for s in [dir, -dir] {
            if self.get(x + s, y) == EMPTY && self.can_sink(d, self.get(x + s, y + 1)) { self.swap_xy(x, y, x + s, y + 1); return; }
        }
        for s in [dir, -dir] {
            let mut last = 0;
            for k in 1..=spread { if self.get(x + s * k, y) == EMPTY { last = k; } else { break; } }
            if last > 0 { self.swap_xy(x, y, x + s * last, y); return; }
        }
    }
    fn gas(&mut self, x: i32, y: i32, el: u8) {
        let d = density(el);
        if self.can_rise(d, self.get(x, y - 1)) && self.rnd() % 8 != 0 { self.swap_xy(x, y, x, y - 1); return; }
        let dir = if self.rnd() & 1 == 0 { -1 } else { 1 };
        for s in [dir, -dir] { if self.can_rise(d, self.get(x + s, y - 1)) { self.swap_xy(x, y, x + s, y - 1); return; } }
        for s in [dir, -dir] { if self.get(x + s, y) == EMPTY { self.swap_xy(x, y, x + s, y); return; } }
    }
    fn steam(&mut self, x: i32, y: i32, c: Cell) {
        let i = self.idx(x, y);
        if c.life == 0 { if self.rnd() % 3 == 0 { self.set(x, y, WATER); } else { self.set(x, y, EMPTY); } return; }
        self.cells[i].life -= 1;
        if self.near4(x, y, ICE) || self.near4(x, y, SNOW) { self.set(x, y, WATER); return; }
        self.gas(x, y, STEAM);
    }

    fn fire(&mut self, x: i32, y: i32, c: Cell) {
        let i = self.idx(x, y);
        let fuel = c.aux != 0;
        if c.life == 0 {
            if c.aux == WOOD || c.aux == PLANT { let r = self.rnd() % 100; if r < 35 { self.set(x, y, ASH); return; } }
            if c.aux == COAL { self.set(x, y, ASH); return; }
            if self.rnd() % 6 == 0 { self.set(x, y, SMOKE); } else { self.set(x, y, EMPTY); }
            return;
        }
        let dec = if fuel { (self.rnd() % 3 == 0) as u8 } else { 1 };
        self.cells[i].life = c.life.saturating_sub(dec);
        let mut clings = false;
        for dy in -1..=1 { for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            let e = self.get(x + dx, y + dy);
            if matches!(e, WOOD | PLANT | COAL | TNT) { clings = true; }
            match e {
                WATER => { if self.rnd() % 4 == 0 { self.events.push(SimEvent::Sizzle { x, y }); self.set(x, y, STEAM); return; } }
                SNOW | ICE => { if self.rnd() % 12 == 0 { self.set(x + dx, y + dy, WATER); } }
                _ => { let bc = Self::burn_chance(e); if bc > 0 && self.rnd() % 256 < bc { self.ignite(x + dx, y + dy); } }
            }
        } }
        if fuel {
            if self.get(x, y - 1) == EMPTY && self.rnd() % 6 == 0 { self.fire_at(x, y - 1, 14, 12); }
            else if self.get(x, y - 1) == EMPTY && self.rnd() % 30 == 0 { self.set(x, y - 1, SMOKE); }
            // burning oil spreads sideways onto oil only through ignition; liquid oil falls even while burning? keep in place
        } else {
            if self.rnd() % 25 == 0 && self.get(x, y - 1) == EMPTY { self.set(x, y - 1, SMOKE); }
            if !clings && self.rnd() % 10 < 7 {
                let dir = if self.rnd() & 1 == 0 { -1 } else { 1 };
                if self.get(x, y - 1) == EMPTY { self.swap_xy(x, y, x, y - 1); }
                else if self.get(x + dir, y - 1) == EMPTY { self.swap_xy(x, y, x + dir, y - 1); }
            }
        }
    }

    fn react_water(&mut self, x: i32, y: i32) {
        if self.rnd() % 250 == 0 && (self.near4(x, y, ICE)) { self.set(x, y, ICE); }
    }
    fn react_acid(&mut self, x: i32, y: i32) {
        if self.rnd() % 2 == 0 { return; }
        let (dx, dy) = [(1, 0), (-1, 0), (0, 1), (0, -1)][(self.rnd() % 4) as usize];
        let e = self.get(x + dx, y + dy);
        let chance = match e { DIRT => 60, SAND => 30, WOOD => 50, PLANT => 120, STONE => 20, METAL => 10, BRICK => 14, ICE => 40, COAL => 20, SALT => 90, SNOW => 90, TNT => 40, ASH => 80, GLASS => 3, _ => 0 };
        if chance > 0 && self.rnd() % 256 < chance {
            if self.rnd() % 4 == 0 { self.set(x + dx, y + dy, SMOKE); } else { self.set(x + dx, y + dy, EMPTY); }
            if self.rnd() % 3 == 0 { self.set(x, y, EMPTY); }
        }
        if e == WATER && self.rnd() % 60 == 0 { self.set(x, y, WATER); }
    }
    fn react_lava(&mut self, x: i32, y: i32) {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let e = self.get(x + dx, y + dy);
            match e {
                WATER => { self.events.push(SimEvent::Sizzle { x, y }); self.set(x + dx, y + dy, STEAM); self.set(x, y, STONE); return; }
                SNOW | ICE => { self.set(x + dx, y + dy, WATER); if self.rnd() % 5 == 0 { self.set(x, y, STONE); return; } }
                SAND => { if self.rnd() % 50 == 0 { self.set(x + dx, y + dy, GLASS); } }
                _ => { if Self::burn_chance(e) > 0 && self.rnd() % 8 == 0 { self.ignite(x + dx, y + dy); } }
            }
        }
        if self.rnd() % 150 == 0 && self.get(x, y - 1) == EMPTY { self.set_aux(x, y - 1, FIRE, 0, Some(14)); }
    }
    fn grow(&mut self, x: i32, y: i32) {
        let (dx, dy) = [(1, 0), (-1, 0), (0, 1), (0, -1), (0, -1), (1, -1), (-1, -1)][(self.rnd() % 7) as usize];
        if self.get(x + dx, y + dy) == WATER { self.set(x + dx, y + dy, PLANT); }
    }
    fn freeze_neighbor(&mut self, x: i32, y: i32) {
        let (dx, dy) = [(1, 0), (-1, 0), (0, 1), (0, -1)][(self.rnd() % 4) as usize];
        if self.get(x + dx, y + dy) == WATER { self.set(x + dx, y + dy, ICE); }
    }
    fn clone_el(&mut self, x: i32, y: i32, c: Cell) {
        let i = self.idx(x, y);
        if c.aux == 0 {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let e = self.get(x + dx, y + dy);
                if e != EMPTY && e != CLONE && e != WALL && e != VOID && e != RUNE { self.cells[i].aux = e; return; }
            }
        } else if self.rnd() % 2 == 0 {
            let (dx, dy) = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)][(self.rnd() % 8) as usize];
            if self.get(x + dx, y + dy) == EMPTY { self.set(x + dx, y + dy, c.aux); }
        }
    }

    // ------------------------------------------------------------------ render
    /// Write RGBA bytes (w*h*4). `cols` = element colours (allows overrides).
    pub fn render(&self, out: &mut [u8], cols: &[(u8, u8, u8); NUM_ELEMENTS]) {
        let f = self.frame;
        for y in 0..self.h {
            for x in 0..self.w {
                let i = y * self.w + x; let c = self.cells[i]; let o = i * 4;
                let px: [u8; 4];
                if c.el == EMPTY {
                    px = if self.bg[i] != 0 { let n = ((x * 7 + y * 13) % 5) as u8; [28 + n, 24 + n, 36 + n, 238] } else { [0, 0, 0, 0] };
                } else {
                    let (mut r, mut g, mut b) = cols[c.el as usize]; let a = ELEMENTS[c.el as usize].alpha;
                    let v = c.var as i32;
                    match c.el {
                        FIRE => { let l = c.life as i32; if l > 36 { r = 255; g = 230; b = 90; } else if l > 18 { r = 255; g = (150 + v / 8) as u8; b = 30; } else { r = 210; g = (50 + v / 10) as u8; b = 20; } }
                        LAVA => { let t = ((f / 6 + c.var as u32) & 31) as i32; r = 255; g = (70 + t * 3).min(255) as u8; b = 10; }
                        RUNE => { let t = (((f / 4 + (x as u32 / 4)) & 31) as i32 - 16).abs() * 5; r = (60 + t / 2) as u8; g = (170 + t / 2).min(255) as u8; b = 255; }
                        THUNDER => { r = 255; g = 255; b = 190 + (v / 4) as u8; }
                        WATER => { let up = if y > 0 { self.cells[i - self.w].el } else { WALL }; if up == EMPTY || up == SMOKE { r = r.saturating_add(40); g = g.saturating_add(60); b = b.saturating_add(20); } let s = 232 + v * 23 / 255; r = (r as i32 * s / 255) as u8; g = (g as i32 * s / 255) as u8; b = (b as i32 * s / 255) as u8; }
                        _ => { let s = 214 + v * 41 / 255; r = (r as i32 * s / 255) as u8; g = (g as i32 * s / 255) as u8; b = (b as i32 * s / 255) as u8; }
                    }
                    px = [r, g, b, a];
                }
                out[o..o + 4].copy_from_slice(&px);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn run(w: &mut World, n: usize) { for _ in 0..n { w.step(); } }
    fn world() -> World { let mut w = World::new(80, 80); w.seed(12345); w }
    #[test] fn sand_falls_to_floor() {
        let mut w = world(); w.set(40, 10, SAND); run(&mut w, 100);
        assert_eq!(w.get(40, 77), SAND); assert_eq!(w.get(40, 10), EMPTY);
    }
    #[test] fn sand_sinks_in_water_oil_floats() {
        let mut w = world();
        for x in 20..60 { for y in 60..78 { w.set(x, y, WATER); } }
        w.set(40, 40, SAND); w.set(30, 40, OIL); run(&mut w, 200);
        // sand ends at the bottom, oil on top of the water column
        assert_eq!(w.get(40, 77), SAND);
        let oil_y = (0..78).find(|&y| (2..78).any(|x| w.get(x, y) == OIL)).unwrap();
        let water_top = (0..78).find(|&y| (20..60).any(|x| w.get(x, y) == WATER)).unwrap();
        assert!(oil_y <= water_top + 1, "oil {} water {}", oil_y, water_top);
    }
    #[test] fn water_levels_out() {
        let mut w = world(); for y in 50..60 { w.set(40, y, WATER); } run(&mut w, 300);
        let xs: Vec<i32> = (2..78).filter(|&x| w.get(x, 77) == WATER).collect();
        assert!(xs.len() >= 8, "spread {:?}", xs.len());
    }
    #[test] fn fire_burns_wood() {
        let mut w = world(); for x in 30..50 { for y in 70..78 { w.set(x, y, WOOD); } } w.set_aux(40, 69, FIRE, 0, Some(60));
        let before = w.count_in(0, 0, 79, 79, |e| e == WOOD); run(&mut w, 600);
        let after = w.count_in(0, 0, 79, 79, |e| e == WOOD); assert!(after < before / 2, "{} -> {}", before, after);
    }
    #[test] fn lava_and_water_make_stone() {
        let mut w = world(); for x in 30..40 { w.set(x, 77, LAVA); } for x in 30..40 { w.set(x, 70, WATER); } run(&mut w, 200);
        assert!(w.count_in(0, 0, 79, 79, |e| e == STONE) > 0);
    }
    #[test] fn gunpowder_explodes() {
        let mut w = world(); for x in 30..50 { for y in 70..78 { w.set(x, y, GUNPOWDER); } } w.set_aux(40, 69, FIRE, 0, Some(60));
        run(&mut w, 120);
        assert!(w.count_in(0, 0, 79, 79, |e| e == GUNPOWDER) < 20);
        assert!(w.events.iter().any(|e| matches!(e, SimEvent::Explosion { .. })));
    }
    #[test] fn walls_survive_explosion() {
        let mut w = world(); w.set(40, 40, WALL); w.explode(40, 41, 8); assert_eq!(w.get(40, 40), WALL);
    }
    #[test] fn plant_grows_in_water() {
        let mut w = world(); for x in 30..50 { for y in 60..78 { w.set(x, y, WATER); } } w.set(40, 77, PLANT);
        // plant needs support not required; just growth
        run(&mut w, 2000); assert!(w.count_in(0, 0, 79, 79, |e| e == PLANT) > 1);
    }
    #[test] fn clone_duplicates() {
        let mut w = world(); w.set_aux(40, 77, CLONE, WATER, None); run(&mut w, 400);
        assert!(w.count_in(0, 0, 79, 79, |e| e == WATER) > 20);
    }
    #[test] fn acid_dissolves_stone() {
        let mut w = world(); for x in 30..50 { w.set(x, 77, STONE); w.set(x, 76, STONE); } for x in 35..45 { for y in 60..75 { w.set(x, y, ACID); } }
        let b = w.count_in(0, 0, 79, 79, |e| e == STONE); run(&mut w, 800); assert!(w.count_in(0, 0, 79, 79, |e| e == STONE) < b);
    }
}
