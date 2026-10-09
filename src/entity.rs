//! Physics body shared by players, enemies; visual particles.
use crate::elements::*;
use crate::sim::World;
use macroquad::prelude::Color;

#[derive(Clone, Debug)]
pub struct Body {
    pub x: f32, // centre-bottom (feet)
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub w: f32,
    pub h: f32,
    pub on_ground: bool,
    pub wet: f32,
    pub face: f32,
    pub blocked: bool,
    pub landed: f32,
}

#[derive(Default, Clone, Copy)]
pub struct Env { pub dmg: f32, pub burn: bool, pub lava: bool, pub acid: bool, pub thunder: bool }

impl Body {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self { Body { x, y, vx: 0.0, vy: 0.0, w, h, on_ground: false, wet: 0.0, face: 1.0, blocked: false, landed: 0.0 } }
    pub fn collides(&self, world: &World, x: f32, y: f32) -> bool {
        let x0 = (x - self.w / 2.0).floor() as i32; let x1 = (x + self.w / 2.0 - 0.01).floor() as i32;
        let y0 = (y - self.h).floor() as i32; let y1 = (y - 0.01).floor() as i32;
        for yy in y0..=y1 { for xx in x0..=x1 { if world.solid_at(xx, yy) { return true; } } }
        false
    }
    pub fn rect(&self) -> (f32, f32, f32, f32) { (self.x - self.w / 2.0, self.y - self.h, self.x + self.w / 2.0, self.y) }
    pub fn center(&self) -> (f32, f32) { (self.x, self.y - self.h * 0.5) }
    pub fn overlaps(&self, x: f32, y: f32, r: f32) -> bool { let (l, t, rr, b) = self.rect(); x + r > l && x - r < rr && y + r > t && y - r < b }

    fn sample_wet(&mut self, world: &World) {
        let mut n = 0; let mut tot = 0;
        let x = self.x.floor() as i32; let mut yy = (self.y - self.h).floor() as i32;
        while yy < self.y.floor() as i32 { tot += 1; if cat(world.get(x, yy)) == Cat::Liquid { n += 1; } yy += 2; }
        self.wet = if tot > 0 { n as f32 / tot as f32 } else { 0.0 };
    }

    /// One physics tick. `g` = gravity accel (cells/tick^2).
    pub fn step(&mut self, world: &World, g: f32, noclip: bool) {
        self.sample_wet(world);
        let wet = self.wet > 0.4;
        self.vy += if wet { g * 0.35 } else { g };
        if self.vy > 9.0 { self.vy = 9.0; }
        if wet { self.vx *= 0.9; self.vy *= 0.92; }
        self.blocked = false; self.landed = 0.0;
        let (ww, wh) = (world.w as f32, world.h as f32);
        if noclip {
            self.x = (self.x + self.vx).clamp(4.0, ww - 4.0); self.y = (self.y + self.vy).clamp(self.h + 3.0, wh - 3.0); self.on_ground = false; return;
        }
        // horizontal, one cell at a time with step-up
        let mut dx = self.vx; let sgn = dx.signum();
        while dx.abs() > 0.0 {
            let s = if dx.abs() > 1.0 { sgn } else { dx };
            if !self.collides(world, self.x + s, self.y) { self.x += s; }
            else {
                let mut stepped = false;
                if self.on_ground || wet { for up in 1..=4 { if !self.collides(world, self.x + s, self.y - up as f32) && !self.collides(world, self.x, self.y - up as f32) { self.x += s; self.y -= up as f32; stepped = true; break; } } }
                if !stepped { self.blocked = true; self.vx = 0.0; break; }
            }
            dx -= s;
            if s.abs() < 1.0 { break; }
        }
        // vertical
        let mut dy = self.vy; let vs = dy.signum(); let impact = self.vy;
        while dy.abs() > 0.0 {
            let s = if dy.abs() > 1.0 { vs } else { dy };
            if !self.collides(world, self.x, self.y + s) { self.y += s; }
            else { if self.vy > 0.0 { self.landed = impact; } self.vy = 0.0; break; }
            dy -= s; if s.abs() < 1.0 { break; }
        }
        self.on_ground = self.collides(world, self.x, self.y + 1.0);
        self.x = self.x.clamp(4.0, ww - 4.0); self.y = self.y.clamp(self.h + 3.0, wh - 2.0);
    }

    pub fn env(&self, world: &World) -> Env {
        let mut e = Env::default();
        let (l, t, r, b) = self.rect();
        for yy in (t.floor() as i32)..=(b.floor() as i32 - 1) { for xx in (l.floor() as i32)..=(r.floor() as i32 - 1) {
            match world.get(xx, yy) {
                FIRE => { e.dmg += 0.015; e.burn = true; }
                LAVA => { e.dmg += 0.09; e.burn = true; e.lava = true; }
                ACID => { e.dmg += 0.04; e.acid = true; }
                THUNDER => { e.dmg += 0.5; e.thunder = true; e.burn = true; }
                STEAM => { e.dmg += 0.004; }
                _ => {}
            }
        } }
        e
    }
}

#[derive(Clone, Copy)]
pub struct Particle { pub x: f32, pub y: f32, pub vx: f32, pub vy: f32, pub life: f32, pub max: f32, pub col: Color, pub size: f32, pub grav: f32 }
