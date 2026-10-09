//! Weapons firing, projectiles, explosions, damage and deaths.
use crate::bullet::*;
use crate::entity::Particle;
use crate::elements::*;
use crate::enemy::*;
use crate::game::*;
use crate::missions::Obj;
use crate::weapons::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::f32::consts::PI;

const RANDOM_ELEMENTS: [u8; 14] = [SAND, WATER, OIL, LAVA, ACID, SNOW, GUNPOWDER, SALT, DIRT, STEAM, METHANE, MERCURY, NITRO, PLANT];

impl Game {
    // ------------------------------------------------------------------ firing
    pub fn fire_weapon(&mut self, pi: usize) {
        let (wi, trigger, reload_t, cd) = {
            let p = &self.players[pi]; let s = p.slot(); let w = &WEAPONS[s.w];
            (s.w, (w.auto && p.ctl.fire) || p.ctl.fire_pressed, p.reload_t, p.cd)
        };
        if !trigger || reload_t > 0 || cd > 0 { return; }
        let w = &WEAPONS[wi];
        let inf = self.cheats.inf_ammo;
        if w.mag > 0 {
            let p = &mut self.players[pi]; let cur = p.cur.min(p.slots.len() - 1);
            if p.slots[cur].mag == 0 {
                if p.slots[cur].reserve > 0 { p.reload_t = w.reload; self.sfx("reload"); } else if p.ctl.fire_pressed { self.sfx("empty"); }
                return;
            }
            if !inf { p.slots[cur].mag -= 1; }
        }
        self.players[pi].cd = w.rate;
        let (mx, my) = self.players[pi].muzzle(); let aim = self.players[pi].aim;
        let owner = Owner::Player(pi);
        self.sfx_at(w.sfx, mx, my);
        let sp = w.spread;
        match w.fire {
            Fire::Melee => self.melee(pi, w.dmg),
            Fire::Bullet => {
                let a = aim + gen_range(-sp, sp);
                self.bullets.push(Bullet { x: mx, y: my, vx: a.cos() * w.speed, vy: a.sin() * w.speed, life: w.life, proj: Proj::Bullet, dmg: w.dmg, owner, radius: w.radius, grav: 0.0, bounce: 0, fuse: 0, pierce: w.key == "gungnir_sniper", color: Color::new(1.0, 0.9, 0.4, 1.0) });
                self.spark(mx, my, Color::new(1.0, 0.85, 0.4, 1.0), 3, 1.5, 0.12, 0.0);
            }
            Fire::Pellets(n) => for _ in 0..n {
                let a = aim + gen_range(-sp, sp);
                self.bullets.push(Bullet { x: mx, y: my, vx: a.cos() * w.speed * gen_range(0.85, 1.1), vy: a.sin() * w.speed * gen_range(0.85, 1.1), life: w.life, proj: Proj::Bullet, dmg: w.dmg, owner, radius: 0, grav: 0.0, bounce: 0, fuse: 0, pierce: false, color: Color::new(1.0, 0.8, 0.3, 1.0) });
            },
            Fire::Flame => { let a = aim + gen_range(-sp, sp); self.bullets.push(Bullet { x: mx, y: my, vx: a.cos() * w.speed, vy: a.sin() * w.speed, life: w.life, proj: Proj::Flame, dmg: w.dmg, owner, radius: 0, grav: -0.02, bounce: 0, fuse: 0, pierce: true, color: ORANGE }); }
            Fire::Frost => { let a = aim + gen_range(-sp, sp); self.bullets.push(Bullet { x: mx, y: my, vx: a.cos() * w.speed, vy: a.sin() * w.speed, life: w.life, proj: Proj::Frost, dmg: w.dmg, owner, radius: 0, grav: 0.0, bounce: 0, fuse: 0, pierce: true, color: Color::new(0.7, 0.9, 1.0, 1.0) }); }
            Fire::Spray(e) => {
                let el = match e { 0 => self.brush.el, 255 => RANDOM_ELEMENTS[gen_range(0, RANDOM_ELEMENTS.len())], x => x };
                let a = aim + gen_range(-sp, sp);
                self.bullets.push(Bullet { x: mx, y: my, vx: a.cos() * w.speed, vy: a.sin() * w.speed, life: w.life, proj: Proj::Spray(el), dmg: w.dmg, owner, radius: 0, grav: 0.07, bounce: 0, fuse: 0, pierce: true, color: { let c = def(el).color; Color::from_rgba(c.0, c.1, c.2, 255) } });
            }
            Fire::Beam(k) => self.fire_beam(pi, k, mx, my, aim, w),
            Fire::Rocket => self.bullets.push(Bullet { x: mx, y: my, vx: aim.cos() * w.speed, vy: aim.sin() * w.speed, life: w.life, proj: Proj::Rocket, dmg: w.dmg, owner, radius: w.radius, grav: 0.0, bounce: 0, fuse: 0, pierce: false, color: ORANGE }),
            Fire::Grenade => self.bullets.push(Bullet { x: mx, y: my, vx: aim.cos() * w.speed, vy: aim.sin() * w.speed, life: 400, proj: Proj::Grenade, dmg: w.dmg, owner, radius: w.radius, grav: 0.2, bounce: 3, fuse: w.life, pierce: false, color: GREEN }),
            Fire::Trick => self.bullets.push(Bullet { x: mx, y: my, vx: aim.cos() * w.speed, vy: aim.sin() * w.speed, life: w.life, proj: Proj::Trick, dmg: w.dmg, owner, radius: w.radius, grav: 0.1, bounce: 0, fuse: 0, pierce: false, color: YELLOW }),
        }
        if !matches!(w.fire, Fire::Melee | Fire::Flame | Fire::Frost | Fire::Spray(_)) { self.shake += 0.4 + w.dmg * 0.01; }
    }

    pub fn throw_grenade(&mut self, pi: usize) {
        if self.players[pi].nades == 0 && !self.cheats.inf_ammo { return; }
        if !self.cheats.inf_ammo { self.players[pi].nades -= 1; }
        let (sx, sy) = self.players[pi].shoulder(); let a = self.players[pi].aim;
        self.bullets.push(Bullet { x: sx + a.cos() * 6.0, y: sy + a.sin() * 6.0, vx: a.cos() * 5.5 + self.players[pi].body.vx * 0.5, vy: a.sin() * 5.5 - 0.6, life: 400, proj: Proj::Grenade, dmg: 70.0, owner: Owner::Player(pi), radius: 10, grav: 0.22, bounce: 4, fuse: 85, pierce: false, color: GREEN });
        self.sfx_at("shot_grenade", sx, sy);
    }

    fn melee(&mut self, pi: usize, dmg: f32) {
        let (px, py) = self.players[pi].shoulder(); let face = if self.players[pi].aim.cos() >= 0.0 { 1.0 } else { -1.0 };
        for ei in 0..self.enemies.len() {
            let e = &self.enemies[ei]; if e.dead { continue; }
            let (ex, ey) = e.body.center(); let dx = (ex - px) * face;
            if dx > -4.0 && dx < 18.0 + e.body.w / 2.0 && (ey - py).abs() < 14.0 + e.body.h / 2.0 { self.hurt_enemy(ei, dmg, Owner::Player(pi), face); }
        }
        if self.mode == Mode::Versus {
            for oj in 0..self.players.len() { if oj == pi || !self.players[oj].alive { continue; } let (ox, oy) = self.players[oj].body.center(); if ((ox - px) * face) > -4.0 && ((ox - px) * face) < 18.0 && (oy - py).abs() < 16.0 { self.hurt_player(oj, dmg, face, Some(pi)); } }
        }
        // knife also cuts soft material ahead
        for k in 3..14 { let (cx, cy) = ((px + face * k as f32) as i32, py as i32); let e = self.world.get(cx, cy); if matches!(e, PLANT | SNOW | WOOD) && gen_range(0, 3) == 0 { self.world.set(cx, cy, EMPTY); } }
        self.spark(px + face * 12.0, py, WHITE, 4, 1.4, 0.15, 0.0);
    }

    fn fire_beam(&mut self, pi: usize, kind: u8, mx: f32, my: f32, aim: f32, w: &WeaponDef) {
        let owner = Owner::Player(pi);
        let (dx, dy) = (aim.cos(), aim.sin()); let range = w.life as f32;
        if kind == 0 { // thunder: jagged line stopping at terrain, damages along
            let (mut x, mut y) = (mx, my); let mut hit_any: Vec<usize> = vec![];
            let mut t = 0.0;
            while t < range {
                x += dx * 2.0 + gen_range(-1.2, 1.2); y += dy * 2.0 + gen_range(-1.2, 1.2); t += 2.0;
                let (ix, iy) = (x as i32, y as i32);
                if self.world.solid_at(ix, iy) { self.explode_at(x, y, 5, 30.0, owner); break; }
                if self.world.get(ix, iy) == EMPTY { self.world.set(ix, iy, THUNDER); }
                for ei in 0..self.enemies.len() { if !self.enemies[ei].dead && !hit_any.contains(&ei) && self.enemies[ei].body.overlaps(x, y, 5.0) { hit_any.push(ei); } }
            }
            // chain to nearby enemies
            let mut extra = vec![];
            for &ei in &hit_any { let (ex, ey) = self.enemies[ei].body.center(); for oj in 0..self.enemies.len() { if !self.enemies[oj].dead && !hit_any.contains(&oj) && !extra.contains(&oj) { let (ox, oy) = self.enemies[oj].body.center(); if (ox - ex).hypot(oy - ey) < 60.0 && extra.len() < 3 { extra.push(oj); } } } }
            for ei in hit_any.into_iter().chain(extra) { self.hurt_enemy(ei, w.dmg, owner, dx.signum()); let (ex, ey) = self.enemies[ei].body.center(); self.spark(ex, ey, Color::new(1.0, 1.0, 0.5, 1.0), 8, 2.0, 0.3, 0.0); }
            self.spark(mx, my, YELLOW, 6, 2.0, 0.2, 0.0);
        } else { // rail: drills through soft terrain, pierces everyone
            let (mut x, mut y) = (mx, my); let mut t = 0.0; let mut pen = 0;
            while t < range {
                x += dx; y += dy; t += 1.0; let (ix, iy) = (x as i32, y as i32);
                let e = self.world.get(ix, iy);
                if is_solid_for_body(e) {
                    if def(e).hardness >= 150 { self.spark(x, y, WHITE, 8, 2.0, 0.3, 0.0); break; }
                    pen += 1; if pen > 40 { break; }
                    for oy in -2..=2 { for ox in -2..=2 { if ox * ox + oy * oy <= 5 { let c = self.world.get(ix + ox, iy + oy); if def(c).hardness < 150 && c != EMPTY { self.world.set(ix + ox, iy + oy, EMPTY); } } } }
                }
                if t as i32 % 3 == 0 { self.particles.push(Particle { x, y, vx: 0.0, vy: 0.0, life: 0.25, max: 0.25, col: Color::new(0.3, 0.9, 1.0, 1.0), size: 2.0, grav: 0.0 }); }
                for ei in 0..self.enemies.len() { if !self.enemies[ei].dead && self.enemies[ei].body.overlaps(x, y, 2.0) && t as i32 % 4 == 0 { self.hurt_enemy(ei, w.dmg / 2.0, owner, dx.signum()); } }
                if x < 3.0 || y < 3.0 || x > WORLD_W as f32 - 3.0 || y > WORLD_H as f32 - 3.0 { break; }
            }
            self.shake += 2.0;
        }
    }

    // ------------------------------------------------------------------ projectiles
    pub fn update_bullets(&mut self) {
        let mut list = std::mem::take(&mut self.bullets);
        let mut keep = Vec::with_capacity(list.len());
        for mut b in list.drain(..) { if self.step_bullet(&mut b) { keep.push(b); } }
        keep.extend(self.bullets.drain(..));
        self.bullets = keep;
        if self.bullets.len() > 1200 { self.bullets.truncate(1200); }
    }

    fn step_bullet(&mut self, b: &mut Bullet) -> bool {
        let g = self.pdef().gravity;
        b.vy += b.grav * g; b.life -= 1;
        if matches!(b.proj, Proj::Grenade) { b.fuse -= 1; b.vx *= 0.995; if b.fuse <= 0 { self.explode_at(b.x, b.y, b.radius, b.dmg, b.owner); return false; } }
        if b.life <= 0 {
            return match b.proj { Proj::Rocket => { self.explode_at(b.x, b.y, b.radius, b.dmg, b.owner); false } Proj::Trick => { self.trick_burst(b); false } _ => false };
        }
        let speed = (b.vx * b.vx + b.vy * b.vy).sqrt(); let steps = speed.ceil().max(1.0) as i32;
        let (sx, sy) = (b.vx / steps as f32, b.vy / steps as f32);
        let mut dmg_applied = false;
        for _ in 0..steps {
            b.x += sx; b.y += sy;
            if b.x < 2.0 || b.y < 1.0 || b.x >= WORLD_W as f32 - 2.0 || b.y >= WORLD_H as f32 - 2.0 { return false; }
            let (ix, iy) = (b.x as i32, b.y as i32); let el = self.world.get(ix, iy);
            // creatures
            let per_tick = matches!(b.proj, Proj::Flame | Proj::Frost | Proj::Spray(_));
            if !(per_tick && dmg_applied) {
                let hit = self.bullet_vs_creatures(b);
                if hit { dmg_applied = true;
                    match b.proj {
                        Proj::Bullet | Proj::Bolt => { if !b.pierce { return false; } }
                        Proj::Rocket | Proj::Fireball => { self.explode_at(b.x, b.y, b.radius.max(5), b.dmg, b.owner); return false; }
                        Proj::Grenade => { if b.fuse > 8 { b.fuse = 8; } }
                        Proj::Trick => { self.trick_burst(b); return false; }
                        _ => {}
                    }
                }
            }
            if is_solid_for_body(el) { if !self.bullet_terrain(b, ix, iy, el, sx, sy) { return false; } continue; }
            match b.proj {
                Proj::Flame | Proj::Fireball => {
                    if el == WATER { self.sfx_at("sizzle", b.x, b.y); return false; }
                    if el == EMPTY && gen_range(0, 100) < 40 { self.world.fire_at(ix, iy, 12, 14); }
                    if is_flammable(el) && gen_range(0, 100) < 40 { self.world.ignite(ix, iy); }
                }
                Proj::Frost => {
                    if el == WATER && gen_range(0, 100) < 50 { self.world.set(ix, iy, ICE); }
                    if el == EMPTY && gen_range(0, 100) < 25 { self.world.set(ix, iy, SNOW); }
                    if el == FIRE || el == LAVA { self.world.set(ix, iy, if el == LAVA { STONE } else { SMOKE }); }
                }
                Proj::Spray(e) => { if el == EMPTY && gen_range(0, 100) < 60 { self.world.set(ix, iy, e); } }
                Proj::Bullet => { if is_explosive(el) && gen_range(0, 100) < 40 { self.world.ignite(ix, iy); } if el == FIRE { } }
                Proj::Rocket => { if gen_range(0, 2) == 0 { self.particles.push(Particle { x: b.x, y: b.y, vx: gen_range(-0.3, 0.3), vy: gen_range(-0.3, 0.3), life: 0.5, max: 0.5, col: Color::new(0.6, 0.6, 0.6, 1.0), size: 2.0, grav: -0.02 }); } if gen_range(0, 3) == 0 && el == EMPTY { self.world.set(ix, iy, SMOKE); } }
                _ => {}
            }
        }
        true
    }

    fn bullet_terrain(&mut self, b: &mut Bullet, ix: i32, iy: i32, el: u8, sx: f32, sy: f32) -> bool {
        let (px, py) = (b.x - sx, b.y - sy);
        match b.proj {
            Proj::Bullet | Proj::Bolt => {
                let r = b.radius.max(1);
                for oy in -r..=r { for ox in -r..=r { if ox * ox + oy * oy <= r * r { let (cx, cy) = (ix + ox, iy + oy); let c = self.world.get(cx, cy); let h = def(c).hardness as i32; if c != EMPTY && h < 100 && gen_range(0, 100) >= h { if is_explosive(c) { self.world.ignite(cx, cy); } else if cx > 2 && cy > 2 { self.world.set(cx, cy, EMPTY); } } } } }
                if is_explosive(el) { self.world.ignite(ix, iy); }
                let c = def(el).color; self.spark(b.x, b.y, Color::from_rgba(c.0, c.1, c.2, 255), 4, 1.6, 0.25, 0.1);
                if matches!(b.owner, Owner::Player(_)) && gen_range(0, 3) == 0 { self.sfx_at("hit_wall", b.x, b.y); }
                false
            }
            Proj::Flame | Proj::Fireball => { if is_flammable(el) { self.world.ignite(ix, iy); } if matches!(b.proj, Proj::Fireball) { self.explode_at(px, py, 7, b.dmg, b.owner); } false }
            Proj::Frost => { if self.world.get(px as i32, py as i32) == EMPTY { self.world.set(px as i32, py as i32, SNOW); } false }
            Proj::Spray(e) => { for k in 0..3 { let (cx, cy) = ((px - sx * k as f32) as i32, (py - sy * k as f32) as i32); if self.world.get(cx, cy) == EMPTY { self.world.set(cx, cy, e); break; } } false }
            Proj::Rocket => { self.explode_at(px, py, b.radius, b.dmg, b.owner); false }
            Proj::Trick => { self.trick_burst(b); false }
            Proj::Grenade => {
                if b.bounce == 0 { b.vx *= 0.3; b.vy = 0.0; b.x = px; b.y = py - 0.5; return true; }
                b.bounce -= 1;
                let hit_x = self.world.solid_at(ix, py as i32); let hit_y = self.world.solid_at(px as i32, iy);
                if hit_x || !hit_y { b.vx = -b.vx * 0.5; } if hit_y || !hit_x { b.vy = -b.vy * 0.5; }
                b.vx *= 0.85; b.x = px; b.y = py;
                self.sfx_at("hit_wall", b.x, b.y); true
            }
            Proj::Thunder => false,
        }
    }

    fn trick_burst(&mut self, b: &Bullet) {
        self.explode_at(b.x, b.y, b.radius, b.dmg, b.owner);
        for _ in 0..30 { let a = gen_range(0.0, PI * 2.0); let r = gen_range(0.0, 7.0); let (x, y) = ((b.x + a.cos() * r) as i32, (b.y + a.sin() * r) as i32); if self.world.get(x, y) == EMPTY { let el = RANDOM_ELEMENTS[gen_range(0, RANDOM_ELEMENTS.len())]; self.world.set(x, y, el); } }
    }

    /// Returns true if the bullet hit a creature this step.
    fn bullet_vs_creatures(&mut self, b: &mut Bullet) -> bool {
        let mut hit = false;
        let dirx = if b.vx >= 0.0 { 1.0 } else { -1.0 };
        if !matches!(b.owner, Owner::Enemy) {
            for ei in 0..self.enemies.len() {
                if self.enemies[ei].dead || !self.enemies[ei].body.overlaps(b.x, b.y, 1.0) { continue; }
                let (fire_imm, cold_imm) = { let d = self.enemies[ei].d(); (d.fire_immune, d.cold_immune) };
                let mut dmg = b.dmg;
                match b.proj {
                    Proj::Flame | Proj::Fireball => { if fire_imm { dmg *= 0.1; } else { self.enemies[ei].burn = 240; } }
                    Proj::Frost => { if cold_imm { dmg *= 0.1; } else { self.enemies[ei].slow = 120; } }
                    Proj::Spray(ACID) => { dmg = 1.6; }
                    Proj::Spray(_) => { dmg = 0.2; }
                    _ => {}
                }
                self.hurt_enemy(ei, dmg, b.owner, dirx);
                self.enemies[ei].body.vx += b.vx * 0.04;
                hit = true; if !b.pierce && !matches!(b.proj, Proj::Flame | Proj::Frost | Proj::Spray(_)) { break; }
            }
        }
        let versus = self.mode == Mode::Versus;
        for pi in 0..self.players.len() {
            if !self.players[pi].alive || !self.players[pi].body.overlaps(b.x, b.y, 1.0) { continue; }
            match b.owner {
                Owner::Player(j) => { if !versus || j == pi { continue; } self.hurt_player(pi, b.dmg, dirx, Some(j)); hit = true; if !b.pierce { break; } }
                Owner::Enemy => { let mult = 1.0 + 0.1 * self.pdef().tier as f32; self.hurt_player(pi, b.dmg * mult * 0.7, dirx, None); hit = true; if !b.pierce { break; } }
                Owner::World => {}
            }
        }
        hit
    }

    // ------------------------------------------------------------------ explosions
    pub fn explode_at(&mut self, x: f32, y: f32, r: i32, dmg: f32, owner: Owner) {
        self.world.explode_ex(x as i32, y as i32, r, false);
        self.blast_effects(x, y, r as f32, dmg, owner);
        self.sfx_at(if r > 9 { "explosion_big" } else { "explosion" }, x, y); self.shake_at(x, y, r as f32 * 0.6);
    }

    pub fn blast_effects(&mut self, x: f32, y: f32, r: f32, dmg: f32, owner: Owner) {
        let reach = r + 6.0;
        if !matches!(owner, Owner::Enemy) {
            for ei in 0..self.enemies.len() {
                if self.enemies[ei].dead { continue; }
                let (ex, ey) = self.enemies[ei].body.center(); let d = (ex - x).hypot(ey - y);
                if d < reach { let f = 1.0 - d / reach; let imm = self.enemies[ei].d().fire_immune; self.hurt_enemy(ei, dmg * f * if imm && matches!(owner, Owner::World) { 0.2 } else { 1.0 }, owner, (ex - x).signum());
                    let b = &mut self.enemies[ei].body; b.vx += (ex - x) / d.max(1.0) * f * 4.0; b.vy -= f * 3.0; if self.enemies[ei].d().ai != Ai::Flyer { } }
            }
        }
        let versus = self.mode == Mode::Versus;
        for pi in 0..self.players.len() {
            if !self.players[pi].alive { continue; }
            let (px, py) = self.players[pi].body.center(); let d = (px - x).hypot(py - y);
            if d >= reach { continue; }
            let f = 1.0 - d / reach;
            let (scale, by) = match owner { Owner::Player(j) if j == pi => (0.4, Some(pi)), Owner::Player(j) => (if versus { 1.0 } else { 0.0 }, Some(j)), _ => (1.0, None) };
            if scale > 0.0 { self.hurt_player(pi, dmg * f * scale, (px - x).signum(), by); }
            let b = &mut self.players[pi].body; b.vx += (px - x) / d.max(1.0) * f * 5.0; b.vy -= f * 4.0;
        }
        for _ in 0..(r as i32 * 2).min(40) { let a = gen_range(0.0, PI * 2.0); let s = gen_range(0.5, 3.0); self.particles.push(Particle { x, y, vx: a.cos() * s, vy: a.sin() * s, life: gen_range(0.3, 0.8), max: 0.8, col: if gen_range(0, 2) == 0 { ORANGE } else { YELLOW }, size: gen_range(1.5, 3.0), grav: 0.05 }); }
    }

    // ------------------------------------------------------------------ damage
    pub fn hurt_enemy(&mut self, ei: usize, dmg: f32, owner: Owner, dir: f32) {
        let e = &mut self.enemies[ei]; if e.dead || dmg <= 0.0 { return; }
        e.hp -= dmg; e.hurt = 5;
        let (bx, by) = e.body.center(); let ghost = e.d().ai == Ai::Ghost;
        if dmg > 3.0 || gen_range(0, 6) == 0 { if ghost { self.spark(bx, by, Color::new(0.7, 0.8, 1.0, 1.0), 2, 1.0, 0.3, 0.0); } else { self.blood(bx, by, if dmg > 3.0 { 3 } else { 1 }, dir); } }
        if self.enemies[ei].hp <= 0.0 { self.kill_enemy(ei, owner); }
    }

    pub fn kill_enemy(&mut self, ei: usize, owner: Owner) {
        if self.enemies[ei].dead { return; }
        self.enemies[ei].dead = true;
        let (def_i, x, y) = { let e = &self.enemies[ei]; (e.def, e.body.x, e.body.y - e.body.h / 2.0) };
        let d = &ENEMIES[def_i]; let boss = is_boss(def_i);
        self.blood(x, y, if boss { 40 } else { 14 }, 0.0);
        if boss { self.explode_at(x, y, 14, 0.0, Owner::Enemy); self.sfx("boss_roar"); self.boss_alive = false; self.shake += 8.0; }
        else { self.sfx_at("die", x, y); }
        self.kills += 1; *self.kind_kills.entry(d.key.to_string()).or_insert(0) += 1;
        if let Owner::Player(i) = owner { self.players[i].kills += 1; self.players[i].score += d.silver as i32; }
        // drops
        let n = gen_range(1, 3) + if boss { 8 } else { 0 };
        for _ in 0..n { self.pickups.push(Pickup { kind: PickKind::Silver(((d.silver as f32 / n as f32).ceil() as u32).max(1)), x: x + gen_range(-6.0, 6.0), y, vy: -2.0, life: 1800 }); }
        let r = gen_range(0, 100); if r < 14 { self.pickups.push(Pickup { kind: PickKind::Health, x, y, vy: -2.0, life: 1800 }); } else if r < 30 { self.pickups.push(Pickup { kind: PickKind::Ammo, x, y, vy: -2.0, life: 1800 }); }
        if boss { for pi in 0..self.players.len() { self.players[pi].hp = self.players[pi].max_hp; } }
        self.mission_kill(d.key, boss);
    }

    pub fn hurt_player(&mut self, pi: usize, dmg: f32, dir: f32, by: Option<usize>) {
        if !self.players[pi].alive || self.players[pi].invuln > 0 || dmg <= 0.0 { return; }
        if self.cheats.god && (pi == 0 || self.players.len() == 1) { return; }
        let (x, y) = self.players[pi].body.center();
        self.players[pi].hp -= dmg; self.players[pi].hurt_t = 8; self.players[pi].since_hit = 0;
        if dmg > 1.5 { self.blood(x, y, 2, dir); if self.tick % 6 == 0 { self.sfx_at("hurt", x, y); } }
        if self.players[pi].hp <= 0.0 { self.kill_player(pi, by); }
    }

    pub fn kill_player(&mut self, pi: usize, by: Option<usize>) {
        let (x, y) = self.players[pi].body.center();
        { let p = &mut self.players[pi]; p.alive = false; p.dead_t = 150; p.deaths += 1; p.hp = 0.0; p.burn = 0; }
        self.blood(x, y, 30, 0.0); self.sfx_at("die", x, y);
        if let Some(j) = by { if j != pi { self.players[j].kills += 1; } } else if self.mode == Mode::Versus { /* suicide */ }
        if self.mode == Mode::Campaign { let lost = self.save.silver / 20; self.save.silver -= lost; if lost > 0 { self.toast(format!("You died. Lost {} silver.", lost)); } }
        let _ = Obj::Boss;
    }
}
