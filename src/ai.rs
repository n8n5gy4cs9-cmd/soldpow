//! Enemy + boss AI.
use crate::bullet::*;
use crate::elements::*;
use crate::enemy::*;
use crate::game::*;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::f32::consts::PI;

fn approach(v: f32, target: f32, a: f32) -> f32 { if v < target { (v + a).min(target) } else { (v - a).max(target) } }

impl Game {
    pub fn update_enemies(&mut self) {
        let mut list = std::mem::take(&mut self.enemies);
        for e in list.iter_mut() { if !e.dead { self.ai_step(e); } }
        list.retain(|e| !e.dead);
        list.extend(self.enemies.drain(..));
        self.enemies = list;
        for i in 0..self.enemies.len() { if self.enemies[i].hp <= 0.0 && !self.enemies[i].dead { self.kill_enemy(i, Owner::World); } }
        self.enemies.retain(|e| !e.dead);
    }

    fn enemy_shoot(&mut self, e: &Enemy, angle: f32, dmg: f32, speed: f32, color: Color, proj: Proj) {
        let (x, y) = (e.body.x + angle.cos() * 8.0, e.body.y - e.body.h * 0.6 + angle.sin() * 8.0);
        let rad = if matches!(proj, Proj::Rocket) { 10 } else { 1 };
        self.bullets.push(Bullet { x, y, vx: angle.cos() * speed, vy: angle.sin() * speed, life: 220, proj, dmg, owner: Owner::Enemy, radius: rad, grav: if matches!(proj, Proj::Spray(_)) { 0.06 } else { 0.0 }, bounce: 0, fuse: 0, pierce: false, color });
        self.sfx_at(if dmg > 12.0 { "shot_rifle" } else { "shot_pistol" }, x, y);
    }

    fn ai_step(&mut self, e: &mut Enemy) {
        let d = e.d(); e.t += 1; if e.cd > 0 { e.cd -= 1; } if e.hurt > 0 { e.hurt -= 1; }
        let slow = if e.slow > 0 { e.slow -= 1; 0.45 } else { 1.0 };
        let boss = matches!(d.ai, Ai::Boss(_));
        // environment damage
        let env = e.body.env(&self.world);
        if env.dmg > 0.0 { let mut dm = env.dmg; if d.fire_immune && !env.acid { dm = 0.0; } if boss { dm *= 0.3; } e.hp -= dm; }
        if env.burn && !d.fire_immune { e.burn = 200; }
        if e.burn > 0 { e.burn -= 1; e.hp -= if boss { 0.05 } else { 0.14 }; if e.t % 6 == 0 { let (bx, by) = e.body.center(); self.spark(bx, by - 4.0, ORANGE, 1, 0.5, 0.35, -0.03); } if self.world.get(e.body.x as i32, (e.body.y - e.body.h / 2.0) as i32) == WATER { e.burn = 0; } }
        if e.body.landed > 9.5 && d.ai != Ai::Flyer { e.hp -= (e.body.landed - 9.5) * 5.0; }
        // target selection
        let mut tgt: Option<usize> = None;
        if let Some(ti) = self.nearest_player(e.body.x, e.body.y) {
            let (tx, ty) = self.players[ti].body.center(); let dist = (tx - e.body.x).hypot(ty - e.body.y);
            let aggro = e.charge > 0 || boss;
            if dist < d.sight && (aggro || self.line_clear(e.body.x, e.body.y - d.h * 0.6, tx, ty)) { tgt = Some(ti); e.charge = 240; }
        }
        if e.charge > 0 && tgt.is_none() { e.charge -= 1; }
        let grav = self.pdef().gravity * 0.30;
        let mut g = grav;
        match d.ai {
            Ai::Ranged => self.ai_ranged(e, tgt, slow),
            Ai::Melee | Ai::Quad => self.ai_melee(e, tgt, slow),
            Ai::Flyer => { g = 0.0; self.ai_flyer(e, tgt, slow); }
            Ai::Ghost => { g = 0.0; self.ai_ghost(e, tgt, slow); }
            Ai::Boss(k) => { if !matches!(k, Boss::Louhi | Boss::UkkoEagle | Boss::Tuoni) { } else { g = 0.0; } self.ai_boss(e, k, tgt, slow); }
        }
        let noclip = d.ai == Ai::Ghost;
        e.body.step(&self.world, g, noclip);
        e.anim += e.body.vx.abs() * 0.12;
        if e.body.vx.abs() > 0.1 { e.body.face = e.body.vx.signum(); }
    }

    fn patrol(&mut self, e: &mut Enemy, slow: f32) {
        let d = e.d();
        if e.t % 240 == 0 && gen_range(0, 3) == 0 { e.dir = -e.dir; }
        if e.body.blocked { e.dir = -e.dir; }
        let ax = e.body.x + e.dir * (e.body.w / 2.0 + 5.0);
        if e.body.on_ground && !self.world.solid_at(ax as i32, e.body.y as i32 + 2) && !self.world.solid_at(ax as i32, e.body.y as i32 + 9) { e.dir = -e.dir; }
        e.body.vx = approach(e.body.vx, e.dir * d.speed * 0.4 * slow, 0.1);
        e.body.face = e.dir;
    }

    fn ai_ranged(&mut self, e: &mut Enemy, tgt: Option<usize>, slow: f32) {
        let d = e.d();
        let Some(ti) = tgt else { self.patrol(e, slow); return };
        let (tx, ty) = self.players[ti].body.center(); let dx = tx - e.body.x; let dist = (dx * dx + (ty - e.body.y).powi(2)).sqrt();
        e.dir = if dx >= 0.0 { 1.0 } else { -1.0 }; e.body.face = e.dir;
        let want = if dist > 140.0 { e.dir } else if dist < 60.0 { -e.dir } else if (e.t / 80) % 2 == 0 { e.dir * 0.3 } else { -e.dir * 0.3 };
        e.body.vx = approach(e.body.vx, want * d.speed * slow, 0.15);
        if e.body.on_ground && (e.body.blocked || (ty < e.body.y - 30.0 && e.t % 50 == 0)) { e.body.vy = -d.jump; }
        let sy = e.body.y - d.h * 0.6; e.aim = (ty - sy).atan2(dx);
        let los = self.line_clear(e.body.x, sy, tx, ty);
        let (dmg, spd, spread, rate) = d.shot;
        if e.burst > 0 { if e.t % 7 == 0 { let a = e.aim + gen_range(-spread, spread); self.enemy_shoot(e, a, dmg, spd, Color::new(1.0, 0.5, 0.2, 1.0), Proj::Bolt); e.burst -= 1; } }
        else if e.cd == 0 && los && dist < d.sight { e.cd = rate; e.burst = if d.key == "svartalf" { 1 } else { 3 }; }
    }

    fn ai_melee(&mut self, e: &mut Enemy, tgt: Option<usize>, slow: f32) {
        let d = e.d();
        let Some(ti) = tgt else { self.patrol(e, slow); return };
        let (tx, ty) = self.players[ti].body.center(); let dx = tx - e.body.x; let dy = ty - (e.body.y - d.h / 2.0);
        e.dir = if dx >= 0.0 { 1.0 } else { -1.0 }; e.body.face = e.dir;
        e.body.vx = approach(e.body.vx, e.dir * d.speed * slow, 0.2);
        if e.body.on_ground && (e.body.blocked || (dy < -20.0 && e.t % 30 == 0) || (dx.abs() < 70.0 && e.cd == 0 && d.ai == Ai::Quad)) { e.body.vy = -d.jump; }
        if dx.abs() < d.w / 2.0 + 10.0 && dy.abs() < d.h / 2.0 + 8.0 && e.cd == 0 {
            e.cd = 40; let mult = 1.0 + 0.1 * self.pdef().tier as f32;
            self.hurt_player(ti, d.melee * mult, e.dir, None); self.sfx_at("melee", e.body.x, e.body.y);
        }
    }

    fn ai_flyer(&mut self, e: &mut Enemy, tgt: Option<usize>, slow: f32) {
        let d = e.d();
        let Some(ti) = tgt else { e.body.vx = approach(e.body.vx, e.dir * 0.4, 0.05); e.body.vy = approach(e.body.vy, (e.t as f32 * 0.05).sin() * 0.3, 0.05); if e.body.blocked { e.dir = -e.dir; } return };
        let (tx, ty) = self.players[ti].body.center(); let dx = tx - e.body.x;
        e.dir = if dx >= 0.0 { 1.0 } else { -1.0 }; e.body.face = e.dir;
        let side = if (e.t / 200) % 2 == 0 { 1.0 } else { -1.0 };
        let (gx, gy) = (tx - side * 80.0, ty - 45.0 + (e.t as f32 * 0.05).sin() * 10.0);
        e.body.vx = approach(e.body.vx, ((gx - e.body.x) * 0.03).clamp(-d.speed, d.speed) * slow, 0.06);
        e.body.vy = approach(e.body.vy, ((gy - e.body.y) * 0.03).clamp(-d.speed, d.speed) * slow, 0.06);
        let sy = e.body.y - d.h * 0.6; e.aim = (ty - sy).atan2(dx);
        let (dmg, spd, spread, rate) = d.shot;
        if e.cd == 0 && rate > 0 && self.line_clear(e.body.x, sy, tx, ty) { e.cd = rate; let a = e.aim + gen_range(-spread, spread); self.enemy_shoot(e, a, dmg, spd, Color::new(1.0, 0.9, 0.4, 1.0), Proj::Bolt); }
    }

    fn ai_ghost(&mut self, e: &mut Enemy, tgt: Option<usize>, slow: f32) {
        let d = e.d();
        let Some(ti) = tgt else { e.body.vx = approach(e.body.vx, (e.t as f32 * 0.02).sin() * 0.4, 0.03); e.body.vy = approach(e.body.vy, (e.t as f32 * 0.03).cos() * 0.3, 0.03); return };
        let (tx, ty) = self.players[ti].body.center(); let dx = tx - e.body.x; let dy = ty - (e.body.y - d.h / 2.0);
        let dist = dx.hypot(dy).max(1.0);
        e.body.vx = approach(e.body.vx, dx / dist * d.speed * slow, 0.04); e.body.vy = approach(e.body.vy, dy / dist * d.speed * slow, 0.04);
        e.body.face = if dx >= 0.0 { 1.0 } else { -1.0 };
        if dist < 14.0 && e.cd == 0 { e.cd = 45; let mult = 1.0 + 0.1 * self.pdef().tier as f32; self.hurt_player(ti, d.melee * mult, e.body.face, None); }
    }

    // ------------------------------------------------------------------ bosses
    fn ai_boss(&mut self, e: &mut Enemy, k: Boss, tgt: Option<usize>, slow: f32) {
        let d = e.d(); let Some(ti) = tgt else { self.patrol(e, slow); return };
        let (tx, ty) = self.players[ti].body.center(); let dx = tx - e.body.x; let dy = ty - e.body.y; let dist = dx.hypot(dy);
        e.dir = if dx >= 0.0 { 1.0 } else { -1.0 }; e.body.face = e.dir;
        let frac = e.hp / e.max_hp; let sy = e.body.y - d.h * 0.7; e.aim = (ty - sy).atan2(dx);
        let mult = 1.0 + 0.1 * self.pdef().tier as f32;
        let walk = |e: &mut Enemy, s: f32| { e.body.vx = approach(e.body.vx, e.dir * s * slow, 0.1); if e.body.on_ground && (e.body.blocked || (dy < -40.0 && e.t % 60 == 0)) { e.body.vy = -d.jump; } };
        let hover = |e: &mut Enemy, gx: f32, gy: f32, s: f32| { e.body.vx = approach(e.body.vx, ((gx - e.body.x) * 0.03).clamp(-s, s) * slow, 0.05); e.body.vy = approach(e.body.vy, ((gy - e.body.y) * 0.03).clamp(-s, s) * slow, 0.05); };
        let touch = dx.abs() < d.w / 2.0 + 8.0 && dy.abs() < d.h + 6.0;
        if touch && e.cd % 30 == 0 && e.t % 30 == 0 { self.hurt_player(ti, d.melee * mult, e.dir, None); }
        match k {
            Boss::Fenrir => {
                let cyc = e.t % 260;
                if cyc < 130 { walk(e, d.speed); } else if cyc < 170 { e.body.vx = e.dir * 4.6; if e.body.on_ground && e.body.blocked { e.body.vy = -d.jump; } if cyc == 130 { self.sfx_at("boss_roar", e.body.x, e.body.y); } } else if cyc == 170 { self.explode_at(e.body.x + e.dir * 10.0, e.body.y - 4.0, 8, 22.0, Owner::Enemy); e.body.vx = 0.0; } else { e.body.vx *= 0.9; }
                if frac < 0.66 && e.t % 700 == 350 { for s in [-1.0f32, 1.0] { if let Some(p) = index("fenrir_pup") { self.spawn_enemy(p, e.body.x + s * 24.0, e.body.y - 6.0); } } }
            }
            Boss::Surtr => {
                walk(e, d.speed);
                if e.t % 80 == 0 { let a = e.aim + gen_range(-0.05, 0.05); self.enemy_shoot(e, a, 14.0, 6.0, ORANGE, Proj::Fireball); }
                if e.t % 330 == 100 { for i in 0..14 { let x = tx + (i as f32 - 7.0) * 9.0; self.drop_cell(x, (ty - 90.0).max(6.0), LAVA); } self.say("Surtr", "BURN!"); }
                if frac < 0.5 && e.t % 150 == 0 && dist < 60.0 { self.explode_at(e.body.x + e.dir * 18.0, e.body.y - 8.0, 12, 28.0, Owner::Enemy); }
            }
            Boss::Ymir => {
                walk(e, d.speed);
                if e.t % 100 == 0 { for i in -2..=2 { let a = e.aim + i as f32 * 0.14; self.enemy_shoot(e, a, 9.0, 7.0, Color::new(0.6, 0.9, 1.0, 1.0), Proj::Bolt); } }
                if e.t % 420 == 200 { for i in 0..20 { let x = tx + (i as f32 - 10.0) * 7.0; self.drop_cell(x, (ty - 90.0).max(6.0), SNOW); } }
                if frac < 0.5 && e.t % 800 == 400 { if let Some(p) = index("frostjotunn") { self.spawn_enemy(p, e.body.x - 30.0, e.body.y - 6.0); self.spawn_enemy(p, e.body.x + 30.0, e.body.y - 6.0); } }
            }
            Boss::Jormungandr => {
                walk(e, d.speed);
                if e.t % 80 == 0 { for i in -1..=2 { let a = e.aim - 0.35 + i as f32 * 0.12; self.enemy_shoot(e, a, 8.0, 5.5, Color::new(0.5, 1.0, 0.2, 1.0), Proj::Spray(ACID)); } }
                if e.t % 300 == 150 { for s in [-60.0f32, -30.0, 30.0, 60.0] { self.explode_at(e.body.x + s, e.body.y - 3.0, 6, 18.0, Owner::Enemy); } self.sfx_at("boss_roar", e.body.x, e.body.y); }
            }
            Boss::Hel => {
                walk(e, d.speed);
                if e.t % 70 == 0 { for i in -1..=1 { let a = e.aim + i as f32 * 0.2; self.enemy_shoot(e, a, 12.0, 6.0, Color::new(0.7, 0.3, 1.0, 1.0), Proj::Bolt); } }
                if e.t % 320 == 300 { let nx = (tx + gen_range(-110.0, 110.0)).clamp(40.0, WORLD_W as f32 - 40.0); self.spark(e.body.x, e.body.y - 20.0, Color::new(0.6, 0.2, 0.9, 1.0), 25, 2.0, 0.6, 0.0); e.body.x = nx; e.body.vy = -2.0; }
                if e.t % 600 == 450 { if let Some(p) = index("draugr") { self.spawn_enemy(p, e.body.x - 20.0, e.body.y - 6.0); self.spawn_enemy(p, e.body.x + 20.0, e.body.y - 6.0); } }
            }
            Boss::Louhi => {
                hover(e, tx - e.dir * 90.0, ty - 60.0 + (e.t as f32 * 0.04).sin() * 14.0, d.speed);
                if e.t % 55 == 0 { for i in -2..=2 { let a = e.aim + i as f32 * 0.16; self.enemy_shoot(e, a, 10.0, 6.5, Color::new(0.7, 0.85, 1.0, 1.0), Proj::Bolt); } }
                if frac < 0.5 && e.t % 400 == 200 { for i in 0..16 { self.drop_cell(tx + (i as f32 - 8.0) * 8.0, (ty - 90.0).max(6.0), SNOW); } if let Some(p) = index("louhi_guard") { self.spawn_enemy(p, e.body.x, e.body.y); } }
            }
            Boss::Tuoni => {
                hover(e, tx - e.dir * 70.0, ty - 30.0, d.speed);
                if e.t % 90 == 0 { for i in 0..8 { let a = i as f32 * PI / 4.0 + e.t as f32 * 0.01; self.enemy_shoot(e, a, 9.0, 3.5, Color::new(0.3, 1.0, 0.6, 1.0), Proj::Bolt); } }
                if e.t % 60 == 30 { self.enemy_shoot(e, e.aim, 12.0, 8.0, Color::new(0.3, 1.0, 0.6, 1.0), Proj::Bolt); }
                if e.t % 500 == 250 { if let Some(p) = index("kalma") { self.spawn_enemy(p, e.body.x - 20.0, e.body.y - 10.0); self.spawn_enemy(p, e.body.x + 20.0, e.body.y - 10.0); } }
            }
            Boss::RautaGolem => {
                walk(e, d.speed);
                if e.t % 110 == 0 { let a = e.aim; self.enemy_shoot(e, a, 30.0, 5.5, ORANGE, Proj::Rocket); }
                if e.t % 260 == 200 { for s in [-40.0f32, -20.0, 20.0, 40.0] { self.explode_at(e.body.x + s, e.body.y - 3.0, 8, 20.0, Owner::Enemy); } }
                if frac < 0.5 && e.t % 220 == 100 { for i in 0..6 { self.drop_cell(tx + (i as f32 - 3.0) * 10.0, (ty - 90.0).max(6.0), LAVA); } }
            }
            Boss::UkkoEagle => {
                hover(e, tx - e.dir * 60.0 + (e.t as f32 * 0.03).sin() * 60.0, ty - 70.0, d.speed);
                if e.t % 150 == 0 { self.strikes.push((tx, 45)); self.strikes.push((tx + 30.0, 60)); }
                if e.t % 70 == 0 { let a = e.aim; self.enemy_shoot(e, a, 12.0, 8.0, Color::new(1.0, 1.0, 0.4, 1.0), Proj::Bolt); }
                if frac < 0.5 && e.t % 300 == 150 { e.body.vy = 3.5; e.body.vx = e.dir * 3.0; }
            }
        }
    }

    /// Spawn a falling droplet (weather-style) at the top of the map.
    pub fn drop_cell(&mut self, x: f32, y: f32, el: u8) {
        let (ix, iy) = (x as i32, y as i32);
        if self.world.get(ix, iy) == EMPTY { self.world.set_aux(ix, iy, el, 2, None); }
    }
}
