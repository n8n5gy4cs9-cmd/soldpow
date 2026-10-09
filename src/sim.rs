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
    pub has_life: bool,
    lifemap: Vec<u8>,
    rng: u32,
    cur: u8,
    pending: Vec<(i32, i32, i32)>,
}

impl World {
    pub fn new(w: usize, h: usize) -> Self {
        let mut world = World { w, h, cells: vec![Cell::default(); w * h], bg: vec![0; w * h], frame: 0, events: Vec::new(), has_life: false, lifemap: Vec::new(), rng: 0x2545F491, cur: 0, pending: Vec::new() };
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
            EMBER => 25 + (r >> 8) % 25,
            MWAX => 200,
            SPARK => 3,
            _ => 0,
        } as u8;
        Cell { el, life, aux: 0, var: (r >> 16) as u8, upd: self.cur }
    }
    pub fn set(&mut self, x: i32, y: i32, el: u8) {
        if !self.inb(x, y) { return; }
        let c = self.new_cell(el); let i = self.idx(x, y); self.cells[i] = c; if el == LIFE { self.has_life = true; }
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
        if el == SPARK { if is_conductor(cur) { self.set_aux(x, y, SPARK, cur, Some(3)); } return; }
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
        match el { WALL | RUNE | DIAMOND => 0.0, WIRE | BATTERY => 0.5, METAL | GOLD => 0.4, CLONE | VOID => 0.4, BRICK => 0.6, STONE => 0.75, GLASS => 0.9, _ => 1.0 }
    }
    /// Instant blast: clears cells in a radius (hard materials survive near the rim), leaves fire/smoke.
    pub fn explode(&mut self, cx: i32, cy: i32, r: i32) { self.explode_ex(cx, cy, r, true) }
    pub fn explode_ex(&mut self, cx: i32, cy: i32, r: i32, emit: bool) {
        if emit { self.events.push(SimEvent::Explosion { x: cx, y: cy, r }); }
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
            if el == GLASS && self.rnd() % 100 < 55 { self.set(x, y, BGLASS); continue; }
            if is_explosive(el) && d > 1.0 { let rr = Self::blast_radius(el); self.pending.push((x, y, rr)); self.set(x, y, EMPTY); continue; }
            let roll = self.rnd() % 100;
            if d < rf * 0.5 { if roll < 30 { self.fire_at(x, y, 8, 20); } else if roll < 45 { self.set(x, y, SMOKE); } else { self.set(x, y, EMPTY); } }
            else if roll < 25 { self.fire_at(x, y, 8, 20); } else { self.set(x, y, EMPTY); }
        } }
    }
    pub fn blast_radius(el: u8) -> i32 { match el { GUNPOWDER => 3, METHANE => 3, NITRO => 7, TNT => 15, C4 => 12, HYDROGEN => 3, BOMB => 9, _ => 4 } }

    /// Ignite whatever is at (x,y) if flammable.
    pub fn ignite(&mut self, x: i32, y: i32) {
        let c = self.cell(x, y);
        match c.el {
            WOOD => { let l = (150 + self.rnd() % 90) as u8; self.set_aux(x, y, FIRE, WOOD, Some(l)); }
            PLANT => { let l = (70 + self.rnd() % 50) as u8; self.set_aux(x, y, FIRE, PLANT, Some(l)); }
            OIL => { let l = (50 + self.rnd() % 40) as u8; self.set_aux(x, y, FIRE, OIL, Some(l)); }
            COAL => { self.set_aux(x, y, FIRE, COAL, Some(240)); }
            DUST => { self.set_aux(x, y, FIRE, DUST, Some(22)); }
            BCOAL => { self.set_aux(x, y, FIRE, BCOAL, Some(200)); }
            WAX => { self.set_aux(x, y, FIRE, WAX, Some(90)); }
            MWAX => { self.set_aux(x, y, FIRE, MWAX, Some(60)); }
            FUSE => { self.set_aux(x, y, FIRE, FUSE, Some(40)); }
            FUSEPOWDER => { self.set_aux(x, y, FIRE, FUSEPOWDER, Some(40)); }
            FIREWORKS => { let l = (24 + self.rnd() % 24) as u8; self.set_aux(x, y, FIREWORKS, 1, Some(l)); }
            GUNPOWDER | NITRO | METHANE | TNT | C4 | HYDROGEN | BOMB => { self.set(x, y, EMPTY); self.pending.push((x, y, Self::blast_radius(c.el))); }
            SNOW | ICE => { self.set(x, y, WATER); }
            _ => {}
        }
    }
    fn burn_chance(el: u8) -> u32 { // out of 256 per adjacent-fire per frame
        match el { WOOD => 8, PLANT => 10, OIL => 70, COAL => 2, GUNPOWDER => 255, METHANE => 220, NITRO => 255, TNT => 80, SNOW => 14, ICE => 6, DUST => 45, BCOAL => 2, WAX => 3, MWAX => 6, C4 => 90, FUSE => 12, FUSEPOWDER => 14, HYDROGEN => 250, BOMB => 255, FIREWORKS => 200, _ => 0 }
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
        if self.has_life && self.frame % 5 == 0 { self.life_step(); }
        // chain reactions: process a bounded number of queued explosions per frame
        let mut n = 0;
        while let Some((x, y, r)) = self.pending.pop() { self.explode(x, y, r); n += 1; if n > 40 { break; } }
        if self.pending.len() > 400 { self.pending.truncate(400); }
    }

    /// Weather droplets: aux==2. They fall straight and resolve on landing.
    fn droplet(&mut self, x: i32, y: i32, c: Cell) {
        let below = self.get(x, y + 1);
        if below == EMPTY || (below != c.el && is_fluid(below) && density(below) < density(c.el)) { self.swap_xy(x, y, x, y + 1); return; }
        let r = self.rnd() % 100;
        match c.el {
            LAVA => { self.ignite(x, y + 1); if below == WATER { self.set(x, y + 1, STEAM); } self.fire_at(x, y, 14, 10); }
            ACID => { if r < 70 { self.set(x, y, EMPTY); } else { let i = self.idx(x, y); self.cells[i].aux = 0; } }
            SNOW => { if r < 80 { self.set(x, y, EMPTY); } else { let i = self.idx(x, y); self.cells[i].aux = 0; } }
            _ => { if r < 85 { self.set(x, y, EMPTY); } else { let i = self.idx(x, y); self.cells[i].aux = 0; } }
        }
    }

    fn update(&mut self, x: i32, y: i32, c: Cell) {
        if c.aux == 2 && matches!(c.el, LAVA | ACID | SNOW | WATER) { self.droplet(x, y, c); return; }
        match c.el {
            SAND | DIRT | ASH | COAL | GUNPOWDER => self.powder(x, y, c.el),
            SALT => { self.powder(x, y, SALT); if self.rnd() % 35 == 0 { self.dissolve_salt(x, y); } }
            DUST | BCOAL | FUSEPOWDER | BGLASS => self.powder(x, y, c.el),
            EMBER => {
                if c.life == 0 { self.set(x, y, EMPTY); return; }
                { let i = self.idx(x, y); self.cells[i].life -= 1; }
                self.powder(x, y, EMBER);
                let (dx, dy) = [(1, 0), (-1, 0), (0, 1), (0, -1)][(self.rnd() % 4) as usize];
                if Self::burn_chance(self.get(x + dx, y + dy)) > 0 && self.rnd() % 12 == 0 { self.ignite(x + dx, y + dy); }
            }
            BOMB => {
                self.powder(x, y, BOMB);
                let below = self.get(x, self.cell_y_after(x, y) + 1);
                if below != EMPTY && below != BOMB && !is_fluid(below) { let (bx, by) = (x, self.cell_y_after(x, y)); self.set(bx, by, EMPTY); self.explode(bx, by, 9); }
            }
            SEED => { self.powder(x, y, SEED); if self.rnd() % 30 == 0 { self.grow_tree(x, y); } }
            FIREWORKS => self.firework(x, y, c),
            SALTWATER => { self.liquid(x, y, SALTWATER, 5); }
            LN2 => self.react_ln2(x, y),
            MWAX => {
                self.liquid(x, y, MWAX, 2);
                let (nx, ny) = self.find_moved(x, y, MWAX);
                if let Some(i) = self.try_idx(nx, ny) { if self.cells[i].life == 0 { self.set(nx, ny, WAX); } else { self.cells[i].life -= 1; } }
            }
            CO2 => { self.liquid(x, y, CO2, 3); }
            HYDROGEN => { if self.rnd() % 2 == 0 { self.gas(x, y, HYDROGEN); } }
            OXYGEN => { for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if self.get(x + dx, y + dy) == FIRE && self.rnd() % 3 == 0 { self.fire_at(x, y, 18, 10); return; } } if self.rnd() % 2 == 0 { self.gas(x, y, OXYGEN); } }
            CAUSTIC => { if self.rnd() % 2 == 0 { self.gas(x, y, CAUSTIC); } self.react_acid(x, y); }
            SPONGE => self.react_sponge(x, y, c),
            WIRE | METAL => { if c.life > 0 { let i = self.idx(x, y); self.cells[i].life -= 1; } }
            BATTERY => { if self.frame % 10 == 0 { for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { let e = self.get(x + dx, y + dy); if is_conductor(e) && e != BATTERY && self.cell(x + dx, y + dy).life == 0 { self.set_aux(x + dx, y + dy, SPARK, e, Some(3)); } } } }
            SPARK => self.spark(x, y, c),
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
            if matches!(e, WOOD | PLANT | COAL | TNT | WAX | FUSE | C4) { clings = true; }
            match e {
                WATER => { if self.rnd() % 4 == 0 { self.events.push(SimEvent::Sizzle { x, y }); self.set(x, y, STEAM); return; } }
                SNOW | ICE => { if self.rnd() % 12 == 0 { self.set(x + dx, y + dy, WATER); } }
                SALTWATER => { if self.rnd() % 6 == 0 { self.events.push(SimEvent::Sizzle { x, y }); self.set(x, y, STEAM); return; } }
                CO2 | LN2 => { self.set(x, y, SMOKE); return; }
                WAX => { if self.rnd() % 20 == 0 { self.set(x + dx, y + dy, MWAX); } }
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
        let chance = match e { WAX => 40, DUST => 60, FUSE => 30, SPONGE => 30, C4 => 20, BGLASS => 5, WIRE => 20, BATTERY => 10, BCOAL => 20, DIRT => 60, SAND => 30, WOOD => 50, PLANT => 120, STONE => 20, METAL => 10, BRICK => 14, ICE => 40, COAL => 20, SALT => 90, SNOW => 90, TNT => 40, ASH => 80, GLASS => 3, _ => 0 };
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
                SAND | BGLASS => { if self.rnd() % 50 == 0 { self.set(x + dx, y + dy, GLASS); } }
                SALTWATER => { self.set(x + dx, y + dy, STEAM); self.set(x, y, STONE); return; }
                WAX => { if self.rnd() % 5 == 0 { self.set(x + dx, y + dy, MWAX); } }
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

    // ------------------------------------------------------------------ extra element behaviours
    #[inline] fn try_idx(&self, x: i32, y: i32) -> Option<usize> { if self.inb(x, y) { Some(self.idx(x, y)) } else { None } }
    /// After a move this frame a powder/liquid cell may have shifted; find our element in the 3x3 around (x,y) that was just updated.
    fn cell_y_after(&self, x: i32, y: i32) -> i32 { for dy in 0..=1 { if self.get(x, y + dy) == BOMB && self.cells[self.idx(x, y + dy)].upd == self.cur { return y + dy; } } y }
    fn find_moved(&self, x: i32, y: i32, el: u8) -> (i32, i32) {
        for dy in -1..=1 { for dx in -1..=5 { let (nx, ny) = (x + dx, y + dy); if self.get(nx, ny) == el && self.cells[self.idx(nx, ny)].upd == self.cur { return (nx, ny); } } }
        (x, y)
    }
    fn dissolve_salt(&mut self, x: i32, y: i32) {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] { if self.get(x + dx, y + dy) == WATER { self.set(x + dx, y + dy, SALTWATER); self.set(x, y, EMPTY); return; } }
    }
    fn grow_tree(&mut self, x: i32, y: i32) {
        let below = self.get(x, y + 1);
        if !matches!(below, DIRT | SAND | ASH) { return; }
        let Some((wx, wy)) = [(1, 0), (-1, 0), (1, 1), (-1, 1)].into_iter().map(|(dx, dy)| (x + dx, y + dy)).find(|&(a, b)| self.get(a, b) == WATER) else { return };
        self.set(wx, wy, EMPTY);
        let h = 7 + (self.rnd() % 8) as i32;
        for k in 0..h { if self.get(x, y - k) == EMPTY || k == 0 { self.set(x, y - k, WOOD); } else { break; } }
        let (cx, cy) = (x, y - h); let r = 4 + (self.rnd() % 3) as i32;
        for dy in -r..=r { for dx in -r..=r { if dx * dx + dy * dy <= r * r && self.get(cx + dx, cy + dy) == EMPTY { self.set(cx + dx, cy + dy, PLANT); } } }
    }
    fn firework(&mut self, x: i32, y: i32, c: Cell) {
        if c.aux != 1 { self.powder(x, y, FIREWORKS); return; }          // unlit: behaves like a powder
        let dir = (self.rnd() % 3) as i32 - 1;
        let (nx, ny) = (x + if self.rnd() % 4 == 0 { dir } else { 0 }, y - 1);
        if c.life > 0 && self.get(nx, ny) == EMPTY {
            self.swap_xy(x, y, nx, ny); let i = self.idx(nx, ny); self.cells[i].life -= 1;
            if self.rnd() % 2 == 0 && self.get(nx, ny + 1) == EMPTY { self.set(nx, ny + 1, EMBER); }
        } else {
            self.set(x, y, EMPTY); self.events.push(SimEvent::Explosion { x, y, r: 3 });
            for _ in 0..40 { let a = (self.rnd() % 628) as f32 / 100.0; let r = 3.0 + (self.rnd() % 6) as f32; let (sx, sy) = (x + (a.cos() * r) as i32, y + (a.sin() * r) as i32); if self.get(sx, sy) == EMPTY { self.set(sx, sy, EMBER); } }
        }
    }
    fn react_ln2(&mut self, x: i32, y: i32) {
        self.liquid(x, y, LN2, 4);
        let (nx, ny) = self.find_moved(x, y, LN2);
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let e = self.get(nx + dx, ny + dy);
            match e {
                WATER | SALTWATER => { self.set(nx + dx, ny + dy, ICE); if self.rnd() % 3 == 0 { self.set(nx, ny, EMPTY); return; } }
                LAVA => { self.set(nx + dx, ny + dy, STONE); self.set(nx, ny, EMPTY); return; }
                FIRE => { self.set(nx + dx, ny + dy, SMOKE); }
                STEAM => { self.set(nx + dx, ny + dy, SNOW); }
                _ => {}
            }
        }
        if self.rnd() % 160 == 0 { self.set(nx, ny, EMPTY); }
    }
    fn react_sponge(&mut self, x: i32, y: i32, c: Cell) {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let e = self.get(x + dx, y + dy);
            if (e == WATER || e == SALTWATER) && c.life < 240 { self.set(x + dx, y + dy, EMPTY); let i = self.idx(x, y); self.cells[i].life = c.life.saturating_add(20); return; }
        }
        // saturated sponges drip slowly
        if c.life >= 200 && self.rnd() % 90 == 0 && self.get(x, y + 1) == EMPTY { self.set(x, y + 1, WATER); let i = self.idx(x, y); self.cells[i].life -= 20; }
        // a burning neighbour dries it
        if c.life > 0 && self.near4(x, y, FIRE) { let i = self.idx(x, y); self.cells[i].life = 0; }
    }
    fn spark(&mut self, x: i32, y: i32, c: Cell) {
        let orig = c.aux;
        if c.life == 0 || !is_conductor(orig) { let el = if is_conductor(orig) { orig } else { EMPTY }; self.set(x, y, el); if el != EMPTY { let i = self.idx(x, y); self.cells[i].life = 6; } return; }
        { let i = self.idx(x, y); self.cells[i].life = c.life - 1; }
        if c.life == 3 {
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let e = self.get(x + dx, y + dy);
                if is_conductor(e) && e != BATTERY && self.cell(x + dx, y + dy).life == 0 { self.set_aux(x + dx, y + dy, SPARK, e, Some(3)); }
                else if Self::burn_chance(e) > 0 && self.rnd() % 4 == 0 { self.ignite(x + dx, y + dy); }
            }
        }
    }
    /// Conway's Game of Life (B3/S23), advanced every 5th simulation frame while any LIFE cells exist.
    fn life_step(&mut self) {
        let n = self.w * self.h;
        if self.lifemap.len() != n { self.lifemap = vec![0; n]; }
        let mut alive: Vec<usize> = Vec::new();
        for (i, c) in self.cells.iter().enumerate() { if c.el == LIFE { alive.push(i); } }
        if alive.is_empty() { self.has_life = false; return; }
        let w = self.w as i32;
        let mut touched: Vec<usize> = Vec::with_capacity(alive.len() * 8);
        for &i in &alive { let (x, y) = (i as i32 % w, i as i32 / w); for dy in -1..=1 { for dx in -1..=1 { if dx == 0 && dy == 0 { continue; } let (nx, ny) = (x + dx, y + dy); if nx > 1 && ny > 1 && nx < w - 2 && ny < self.h as i32 - 2 { let j = (ny * w + nx) as usize; if self.lifemap[j] == 0 { touched.push(j); } self.lifemap[j] += 1; } } } }
        let mut births = Vec::new(); let mut deaths = Vec::new();
        for &i in &alive { if self.lifemap[i] != 2 && self.lifemap[i] != 3 { deaths.push(i); } }   // alive cell: survives with 2 or 3 neighbours
        for &j in &touched { let cnt = self.lifemap[j]; if self.cells[j].el == EMPTY && cnt == 3 { births.push(j); } }
        for &j in &touched { self.lifemap[j] = 0; }
        for j in deaths { let (x, y) = (j as i32 % w, j as i32 / w); self.set(x, y, EMPTY); }
        for j in births { let (x, y) = (j as i32 % w, j as i32 / w); self.set(x, y, LIFE); }
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
                        SPARK => { let t = ((f / 2 + c.var as u32) & 7) as u8 * 12; r = 255; g = 255; b = 120 + t; }
                        EMBER => { let t = ((f + c.var as u32) & 7) as i32; r = 255; g = (170 + t * 10) as u8; b = 60; }
                        LIFE => { let t = (c.var / 16) as u8; r = 20 + t; g = 190 + t; b = 20; }
                        SPONGE => { let wet = (c.life as i32 / 4) as u8; r = (255u32.saturating_sub(wet as u32 * 2)).min(255) as u8; g = (190u32.saturating_sub(wet as u32)) as u8; b = 48u8.saturating_add(wet / 2); }
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

    #[test] fn sponge_absorbs_water() {
        let mut w = world(); for x in 30..50 { for y in 74..78 { w.set(x, y, WATER); } } for y in 74..78 { w.set(40, y, SPONGE); }
        let before = w.count_in(0, 0, 79, 79, |e| e == WATER); run(&mut w, 200); assert!(w.count_in(0, 0, 79, 79, |e| e == WATER) < before);
    }
    #[test] fn life_blinker_oscillates() {
        let mut w = world(); for x in 39..=41 { w.set(x, 40, LIFE); }
        run(&mut w, 5); // one generation: horizontal -> vertical
        assert_eq!(w.get(40, 39), LIFE); assert_eq!(w.get(40, 41), LIFE); assert_eq!(w.get(39, 40), EMPTY);
        run(&mut w, 5); assert_eq!(w.get(39, 40), LIFE); assert_eq!(w.get(41, 40), LIFE);
    }
    #[test] fn spark_runs_along_wire_and_ignites_gunpowder() {
        let mut w = world(); for x in 20..60 { w.set(x, 77, WIRE); } for x in 55..58 { for y in 74..77 { w.set(x, y, GUNPOWDER); } }
        w.set(19, 77, BATTERY); run(&mut w, 400);
        assert!(w.events.iter().any(|e| matches!(e, SimEvent::Explosion { .. })), "gunpowder next to the wire should explode");
        assert!(matches!(w.get(30, 77), WIRE | SPARK), "wire must survive sparks");
    }
    #[test] fn liquid_nitrogen_freezes_water() {
        let mut w = world(); for x in 30..50 { for y in 74..78 { w.set(x, y, WATER); } } for x in 38..42 { w.set(x, 70, LN2); } run(&mut w, 200);
        assert!(w.count_in(0, 0, 79, 79, |e| e == ICE) > 0);
    }
    #[test] fn seed_grows_tree_with_water() {
        let mut w = world(); for x in 20..60 { for y in 74..78 { w.set(x, y, DIRT); } } w.set(40, 73, SEED); w.set(41, 73, WATER); w.set(42, 73, STONE); w.set(42, 72, STONE);
        run(&mut w, 1500); assert!(w.count_in(0, 0, 79, 79, |e| e == WOOD) >= 5, "wood {}", w.count_in(0, 0, 79, 79, |e| e == WOOD));
    }
    #[test] fn bomb_explodes_on_landing() {
        let mut w = world(); for x in 30..50 { w.set(x, 77, STONE); } w.set(40, 60, BOMB); run(&mut w, 100);
        assert!(w.events.iter().any(|e| matches!(e, SimEvent::Explosion { .. })));
    }
    #[test] fn co2_extinguishes_fire() {
        let mut w = world(); for x in 30..50 { for y in 70..78 { w.set(x, y, WOOD); } } for x in 30..50 { w.set(x, 69, CO2); w.set(x, 68, CO2); } w.set_aux(40, 67, FIRE, 0, Some(60));
        run(&mut w, 300); assert!(w.count_in(0, 0, 79, 79, |e| e == WOOD) > 100);
    }
    #[test] fn salt_and_water_make_saltwater() {
        let mut w = world(); for x in 30..50 { for y in 74..78 { w.set(x, y, WATER); } } for x in 38..42 { w.set(x, 60, SALT); } run(&mut w, 400);
        assert!(w.count_in(0, 0, 79, 79, |e| e == SALTWATER) > 0);
    }
    #[test] fn c4_and_wax() {
        let mut w = world(); for x in 30..50 { for y in 74..78 { w.set(x, y, C4); } } w.ignite(40, 74); run(&mut w, 50);
        assert!(w.events.iter().any(|e| matches!(e, SimEvent::Explosion { r, .. } if *r >= 12)));
        let mut w = world(); for x in 30..50 { w.set(x, 77, WAX); } w.set_aux(40, 76, FIRE, WAX, Some(200)); run(&mut w, 200);
        assert!(w.count_in(0, 0, 79, 79, |e| e == MWAX || e == WAX) > 0);
    }
    #[test] fn fireworks_launch_and_burst() {
        let mut w = world(); for x in 30..50 { w.set(x, 77, STONE); } w.set(40, 76, FIREWORKS); w.ignite(40, 76); run(&mut w, 120);
        assert!(w.count_in(0, 0, 79, 79, |e| e == EMBER) > 5 || w.events.iter().any(|e| matches!(e, SimEvent::Explosion { .. })));
    }
}
