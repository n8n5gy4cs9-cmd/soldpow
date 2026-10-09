//! Player state + input abstraction.
use crate::entity::Body;
use crate::weapons::{self, WeaponDef, WEAPONS};

#[derive(Clone, Default)]
pub struct Controls {
    pub left: bool, pub right: bool, pub up: bool, pub up_pressed: bool, pub down: bool,
    pub fire: bool, pub fire_pressed: bool, pub nade_pressed: bool,
    pub reload: bool, pub next: bool, pub prev: bool, pub interact: bool,
    pub aim_world: Option<(f32, f32)>,
    /// keyboard aim change per tick (radians), used by player 2
    pub aim_adjust: f32,
}

#[derive(Clone, Copy)]
pub struct Slot { pub w: usize, pub mag: u32, pub reserve: u32 }

pub struct Player {
    pub id: usize,
    pub body: Body,
    pub hp: f32,
    pub max_hp: f32,
    pub fuel: f32,
    pub slots: Vec<Slot>,
    pub cur: usize,
    pub aim: f32,
    pub aim_rel: f32,
    pub cd: u32,
    pub reload_t: u32,
    pub nades: u32,
    pub dead_t: u32,
    pub alive: bool,
    pub burn: u32,
    pub anim: f32,
    pub kills: u32,
    pub deaths: u32,
    pub hurt_t: u32,
    pub ctl: Controls,
    pub jetting: bool,
    pub invuln: u32,
    pub step_t: u32,
    pub score: i32,
    pub slow_t: u32,
    pub since_hit: u32,
}

pub const MAX_SLOTS: usize = 8;
pub const BODY_W: f32 = 6.0;
pub const BODY_H: f32 = 20.0;

impl Player {
    pub fn new(id: usize, x: f32, y: f32) -> Self {
        Player { id, body: Body::new(x, y, BODY_W, BODY_H), hp: 100.0, max_hp: 100.0, fuel: 100.0, slots: Vec::new(), cur: 1, aim: 0.0, aim_rel: 0.0, cd: 0, reload_t: 0, nades: 3, dead_t: 0, alive: true, burn: 0,
            anim: 0.0, kills: 0, deaths: 0, hurt_t: 0, ctl: Controls::default(), jetting: false, invuln: 90, step_t: 0, score: 0, slow_t: 0, since_hit: 0 }
    }
    pub fn weapon(&self) -> &'static WeaponDef { &WEAPONS[self.slots[self.cur.min(self.slots.len() - 1)].w] }
    pub fn slot(&self) -> &Slot { &self.slots[self.cur.min(self.slots.len() - 1)] }
    pub fn shoulder(&self) -> (f32, f32) { (self.body.x, self.body.y - 14.0) }
    pub fn muzzle(&self) -> (f32, f32) {
        let (sx, sy) = self.shoulder(); let len = 13.0;
        (sx + self.aim.cos() * len, sy + self.aim.sin() * len)
    }
    pub fn give(&mut self, wi: usize, full: bool) {
        let w = &WEAPONS[wi];
        if let Some(i) = self.slots.iter().position(|s| s.w == wi) {
            self.slots[i].mag = w.mag; self.slots[i].reserve = if full { w.reserve } else { (self.slots[i].reserve + w.reserve / 2).min(w.reserve) };
            self.cur = i; return;
        }
        let s = Slot { w: wi, mag: w.mag, reserve: w.reserve };
        if self.slots.len() < MAX_SLOTS { self.slots.push(s); self.cur = self.slots.len() - 1; } else { let c = self.cur.max(1); self.slots[c] = s; self.cur = c; }
    }
    pub fn refill(&mut self) { for s in self.slots.iter_mut() { let w = &WEAPONS[s.w]; s.mag = w.mag; s.reserve = w.reserve; } self.nades = 5; }
    /// default start loadout from the unlocked set: knife, pistol, best unlocked primary, powder gun
    pub fn loadout(&mut self, unlocked: &std::collections::BTreeSet<String>, all: bool) {
        self.slots.clear();
        for k in ["puukko", "hugin"] { if let Some(i) = weapons::index(k) { self.give(i, true); } }
        let mut best: Option<usize> = None;
        for (i, w) in WEAPONS.iter().enumerate() {
            if matches!(w.key, "puukko" | "hugin" | "powder_gun") { continue; }
            if (all || unlocked.contains(w.key)) && best.map_or(true, |b| w.tier >= WEAPONS[b].tier) { best = Some(i); }
        }
        if let Some(b) = best { self.give(b, true); }
        if let Some(i) = weapons::index("powder_gun") { self.give(i, true); }
        self.cur = match best { Some(b) => self.slots.iter().position(|s| s.w == b).unwrap_or(1), None => 1 };
    }
}
