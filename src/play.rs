//! Player update, pickups, hotspots, hazards and mission runtime.
use crate::bullet::*;
use crate::entity::Particle;
use crate::elements::*;
use crate::enemy::*;
use crate::game::*;
use crate::hotspot::*;
use crate::missions::*;
use crate::planets::*;
use crate::player::Slot;
use crate::weapons::{self, WEAPONS};
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::f32::consts::PI;

impl Game {
    // ------------------------------------------------------------------ player
    pub fn update_player(&mut self, pi: usize) {
        if !self.players[pi].alive {
            self.players[pi].dead_t = self.players[pi].dead_t.saturating_sub(1);
            if self.players[pi].dead_t == 0 { self.respawn_player(pi); }
            return;
        }
        let ctl = self.players[pi].ctl.clone();
        let g = self.pdef().gravity * 0.30 * self.cheats.grav;
        let ghost = self.cheats.ghost && pi == 0;
        {
            let p = &mut self.players[pi];
            p.since_hit += 1; if p.since_hit > 360 && p.hp < p.max_hp { p.hp = (p.hp + 0.03).min(p.max_hp); }
            p.cd = p.cd.saturating_sub(1); p.invuln = p.invuln.saturating_sub(1); p.hurt_t = p.hurt_t.saturating_sub(1);
            if p.slow_t > 0 { p.slow_t -= 1; }
            // aim
            if let Some((ax, ay)) = ctl.aim_world { p.aim = (ay - (p.body.y - 14.0)).atan2(ax - p.body.x); }
            else { p.aim_rel = (p.aim_rel + ctl.aim_adjust).clamp(-1.3, 1.3); p.aim = if p.body.face >= 0.0 { -p.aim_rel } else { PI + p.aim_rel }; }
            if ctl.aim_world.is_some() { p.body.face = if p.aim.cos() >= 0.0 { 1.0 } else { -1.0 }; }
            // movement
            let run = 1.55 * self.cheats.speed * if p.slow_t > 0 { 0.6 } else { 1.0 } * if ctl.down { 0.6 } else { 1.0 };
            let dir = (ctl.right as i32 - ctl.left as i32) as f32;
            if ctl.aim_world.is_none() && dir != 0.0 { p.body.face = dir; }
            let accel = if p.body.on_ground { 0.4 } else { 0.14 };
            let target = dir * run;
            if p.body.vx < target { p.body.vx = (p.body.vx + accel).min(target.max(p.body.vx)); } else if p.body.vx > target { p.body.vx = (p.body.vx - accel).max(target.min(p.body.vx)); }
            if dir == 0.0 && p.body.on_ground { p.body.vx *= 0.7; }
            if ctl.up_pressed && p.body.on_ground { p.body.vy = -4.1 * self.cheats.jump; self.audio_jump = true; }
            p.jetting = false;
            if ctl.up && !p.body.on_ground && (p.fuel > 0.0 || self.cheats.fly) && !ghost {
                p.body.vy = (p.body.vy - g * 2.1).max(-3.3); p.jetting = true;
                if !self.cheats.fly { p.fuel -= 0.85; }
            } else if p.body.on_ground { p.fuel = (p.fuel + 1.2).min(100.0); } else { p.fuel = (p.fuel + 0.12).min(100.0); }
            if ghost { p.body.vy = if ctl.up { -2.5 } else if ctl.down { 2.5 } else { 0.0 }; p.body.vx = dir * 3.0 * self.cheats.speed; }
            // swim up
            if p.body.wet > 0.5 && ctl.up { p.body.vy -= 0.35; }
        }
        let wr = &mut self.players[pi];
        wr.body.step(&self.world, if ghost { 0.0 } else { g }, ghost);
        wr.anim += wr.body.vx.abs() * 0.12;
        let landed = wr.body.landed;
        if landed > 7.5 && !ghost && g > 0.0 { let dmg = (landed - 7.5) * 8.0 / (self.pdef().gravity).max(0.4); self.hurt_player(pi, dmg, 0.0, None); self.sfx_at("land", self.players[pi].body.x, self.players[pi].body.y); }
        if std::mem::take(&mut self.audio_jump) { let (x, y) = (self.players[pi].body.x, self.players[pi].body.y); self.sfx_at("jump", x, y); }
        if self.players[pi].body.on_ground && self.players[pi].body.vx.abs() > 0.9 && self.tick % 14 == 0 { let (x, y) = (self.players[pi].body.x, self.players[pi].body.y); self.sfx_at("step", x, y); }
        // jetpack effects
        if self.players[pi].jetting {
            let (x, y) = (self.players[pi].body.x, self.players[pi].body.y);
            for _ in 0..2 { self.particles.push(Particle { x: x - self.players[pi].body.face * 3.0 + gen_range(-1.0, 1.0), y, vx: gen_range(-0.4, 0.4), vy: gen_range(1.0, 2.5), life: 0.3, max: 0.3, col: if gen_range(0, 2) == 0 { ORANGE } else { YELLOW }, size: 2.0, grav: 0.0 }); }
            if self.tick % 5 == 0 { self.sfx_at("jet", x, y); }
        }
        // environment
        let env = self.players[pi].body.env(&self.world);
        if env.dmg > 0.0 { self.hurt_player(pi, env.dmg, 0.0, None); }
        { let (cx, cy) = (self.players[pi].body.x as i32, (self.players[pi].body.y - 10.0) as i32);
          if env.burn { self.players[pi].burn = 150; }
          if self.world.get(cx, cy) == WATER { self.players[pi].burn = 0; }
          if self.players[pi].burn > 0 { self.players[pi].burn -= 1; self.hurt_player(pi, 0.12, 0.0, None); if self.tick % 5 == 0 { let (x, y) = self.players[pi].body.center(); self.spark(x, y, ORANGE, 1, 0.5, 0.3, -0.03); } } }
        // weapons
        if ctl.next { let n = self.players[pi].slots.len(); self.players[pi].cur = (self.players[pi].cur + 1) % n; self.players[pi].reload_t = 0; }
        if ctl.prev { let n = self.players[pi].slots.len(); self.players[pi].cur = (self.players[pi].cur + n - 1) % n; self.players[pi].reload_t = 0; }
        {
            let inf = self.cheats.inf_ammo;
            let p = &mut self.players[pi]; let cur = p.cur.min(p.slots.len() - 1); let w = &WEAPONS[p.slots[cur].w];
            if ctl.reload && w.mag > 0 && p.slots[cur].mag < w.mag && p.slots[cur].reserve > 0 && p.reload_t == 0 { p.reload_t = w.reload; }
            if p.reload_t > 0 {
                p.reload_t -= 1;
                if p.reload_t == 0 { let need = w.mag - p.slots[cur].mag; let take = if inf { need } else { need.min(p.slots[cur].reserve) }; p.slots[cur].mag += take; if !inf { p.slots[cur].reserve -= take; } }
            }
        }
        if self.cheats.inf_ammo { let p = &mut self.players[pi]; let cur = p.cur.min(p.slots.len() - 1); let w = &WEAPONS[p.slots[cur].w]; if w.mag > 0 { p.slots[cur].mag = w.mag; } }
        if ctl.fire || ctl.fire_pressed { self.fire_weapon(pi); }
        if ctl.nade_pressed { self.throw_grenade(pi); }
        if ctl.interact { self.interact(pi); }
        // spring healing aura
        let (px, py) = (self.players[pi].body.x, self.players[pi].body.y);
        for h in &self.hotspots { if matches!(h.kind, HotKind::Spring | HotKind::Sauna) && (h.x - px).abs() < 14.0 && (h.y - py).abs() < 16.0 { let p = &mut self.players[pi]; if p.hp < p.max_hp && self.tick % 6 == 0 { p.hp = (p.hp + 1.5).min(p.max_hp); } } }
    }

    pub fn respawn_player(&mut self, pi: usize) {
        let (sx, sy) = self.gen.spawn;
        let mut pos = (sx as f32 + pi as f32 * 10.0, sy as f32);
        if self.arena.is_some() { pos = self.arena_spawn(pi); }
        if self.mode == Mode::Versus { let x = if pi == 0 { 0.07 } else { 0.9 } * WORLD_W as f32; pos = (x, self.gen.surface[x as usize] as f32 - 2.0); }
        else if self.players.len() > 1 { let o = 1 - pi; if self.players[o].alive { pos = (self.players[o].body.x, self.players[o].body.y); } }
        let p = &mut self.players[pi];
        p.alive = true; p.hp = p.max_hp; p.fuel = 100.0; p.body.x = pos.0; p.body.y = pos.1; p.body.vx = 0.0; p.body.vy = 0.0; p.invuln = 120; p.burn = 0;
        if self.mode != Mode::Campaign { p.refill(); }
    }

    // ------------------------------------------------------------------ pickups
    pub fn update_pickups(&mut self) {
        let mut list = std::mem::take(&mut self.pickups);
        for k in list.iter_mut() {
            if k.life != u32::MAX { k.life = k.life.saturating_sub(1); }
            if !self.world.solid_at(k.x as i32, k.y as i32 + 3) { k.vy = (k.vy + 0.2).min(4.0); k.y += k.vy; } else { k.vy = 0.0; if self.world.solid_at(k.x as i32, k.y as i32 + 1) { k.y -= 1.0; } }
        }
        let mut keep = Vec::with_capacity(list.len());
        'outer: for k in list.drain(..) {
            if k.life == 0 { continue; }
            for pi in 0..self.players.len() {
                let p = &self.players[pi]; if !p.alive { continue; }
                let (px, py) = p.body.center();
                if (px - k.x).abs() < 9.0 && (py - k.y).abs() < 14.0 {
                    if self.take_pickup(pi, k) { continue 'outer; }
                }
            }
            keep.push(k);
        }
        keep.extend(self.pickups.drain(..));
        self.pickups = keep;
    }

    fn take_pickup(&mut self, pi: usize, k: Pickup) -> bool {
        match k.kind {
            PickKind::Health => { let p = &mut self.players[pi]; if p.hp >= p.max_hp { return false; } p.hp = (p.hp + 40.0).min(p.max_hp); self.sfx("heal"); self.toast("+ Health"); }
            PickKind::Ammo => { let p = &mut self.players[pi]; for s in p.slots.iter_mut() { let w = &WEAPONS[s.w]; s.reserve = (s.reserve + w.mag.max(10) * 2).min(w.reserve.max(1)); } p.nades = (p.nades + 2).min(9); self.sfx("pickup"); self.toast("+ Ammo"); }
            PickKind::Shard => { self.sfx("pickup_weapon"); self.toast("Sampo shard fragment!"); self.mission_collect(); }
            PickKind::Silver(n) => { self.save.silver += n; self.sfx("pickup"); }
            PickKind::Weapon(wi) => { self.players[pi].give(wi, true); self.save.unlocked.insert(WEAPONS[wi].key.to_string()); self.sfx("pickup_weapon"); self.toast(format!("Got {}", WEAPONS[wi].name)); }
        }
        true
    }

    // ------------------------------------------------------------------ hotspots
    pub fn interact(&mut self, pi: usize) {
        let (px, py) = (self.players[pi].body.x, self.players[pi].body.y);
        let Some(hi) = self.hotspots.iter().enumerate().filter(|(_, h)| (h.x - px).abs() < 24.0 && (h.y - py).abs() < 24.0).min_by(|a, b| ((a.1.x - px).abs()).partial_cmp(&(b.1.x - px).abs()).unwrap()).map(|(i, _)| i) else { return };
        let kind = self.hotspots[hi].kind; let (hx, hy) = (self.hotspots[hi].x, self.hotspots[hi].y); let used = self.hotspots[hi].used;
        let tier = self.pdef().tier;
        match kind {
            HotKind::Spring => { let p = &mut self.players[pi]; p.hp = p.max_hp; p.fuel = 100.0; self.sfx("heal"); self.toast("Aino's Well: fully healed"); }
            HotKind::Sauna => { let p = &mut self.players[pi]; p.hp = p.max_hp; p.fuel = 100.0; self.sfx("heal"); self.toast("Sauna: steam, peace, full health"); }
            HotKind::Rune => { if used { self.toast("The runes have gone quiet."); return; } let planet = self.pdef().id; let n = self.hotspots.iter().filter(|h| h.kind == HotKind::Rune && h.used).count(); self.toast(crate::story::rune_lore(planet, n)); self.save.silver += 30; self.sfx("ui_open"); self.hotspots[hi].used = true; }
            HotKind::Chest | HotKind::Bunker => {
                if used { self.toast("Already looted."); return; }
                self.hotspots[hi].used = true; self.sfx("pickup_weapon");
                let t = if kind == HotKind::Bunker { tier + 1 } else { tier };
                let cands: Vec<usize> = WEAPONS.iter().enumerate().filter(|(_, w)| w.tier <= t && w.key != "puukko" && w.key != "hugin").map(|(i, _)| i).collect();
                self.pickups.push(Pickup { kind: PickKind::Weapon(cands[gen_range(0, cands.len())]), x: hx, y: hy - 6.0, vy: -2.5, life: u32::MAX });
                self.pickups.push(Pickup { kind: PickKind::Ammo, x: hx + 6.0, y: hy - 6.0, vy: -2.0, life: u32::MAX });
                self.pickups.push(Pickup { kind: PickKind::Silver(if kind == HotKind::Bunker { 150 } else { 50 }), x: hx - 6.0, y: hy - 6.0, vy: -2.0, life: u32::MAX });
                self.pickups.push(Pickup { kind: PickKind::Health, x: hx - 12.0, y: hy - 6.0, vy: -2.0, life: u32::MAX });
                self.toast("Chest opened!");
            }
            HotKind::Forge => {
                self.players[pi].refill(); self.players[pi].hp = self.players[pi].max_hp;
                let mut list: Vec<usize> = WEAPONS.iter().enumerate().filter(|(_, w)| self.save.unlocked.contains(w.key) || self.mode != Mode::Campaign).map(|(i, _)| i).filter(|&i| !matches!(WEAPONS[i].key, "puukko" | "hugin" | "powder_gun")).collect();
                list.sort();
                if !list.is_empty() {
                    let cur_w = self.players[pi].slots.get(2).map(|s| s.w);
                    let next = match cur_w.and_then(|c| list.iter().position(|&x| x == c)) { Some(p) => list[(p + 1) % list.len()], None => list[0] };
                    let p = &mut self.players[pi]; if p.slots.len() > 2 { p.slots[2] = Slot { w: next, mag: WEAPONS[next].mag, reserve: WEAPONS[next].reserve }; p.cur = 2; } else { p.give(next, true); }
                    self.toast(format!("Forge: restocked. Primary = {} (press F again to cycle)", WEAPONS[next].name));
                } else { self.toast("Forge: restocked."); }
                self.sfx("reload");
            }
            HotKind::Gate => {
                let sys = self.pdef().system;
                if sys == 0 && !self.save.gate_open && !self.cheats.unlock_all { self.toast("The Bifrost gate is dormant. Complete Asgard's missions."); return; }
                self.ui.map_sys = 1 - sys; self.scene = Scene::Map; self.sfx("gate");
            }
        }
        self.mission_interact(kind);
    }

    // ------------------------------------------------------------------ hazards / weather
    pub fn hazards(&mut self) {
        let alive = self.alive_players();
        if alive.is_empty() { return; }
        let hz = self.pdef().hazard;
        let rate = match hz { Hazard::EmberRain => 6, Hazard::AcidRain => 5, Hazard::Snow => 2, Hazard::Sleet => 3, _ => 0 };
        if rate > 0 && self.tick % rate == 0 && self.mode != Mode::Sandbox {
            let el = match hz { Hazard::EmberRain => LAVA, Hazard::AcidRain => ACID, Hazard::Snow => SNOW, _ => WATER };
            for &pi in &alive { let x = self.players[pi].body.x + gen_range(-220.0, 220.0); let y = (self.players[pi].body.y - 150.0).max(5.0); self.drop_cell(x, y, el); }
        }
        if hz == Hazard::Lightning && self.tick % 200 == 0 && gen_range(0, 100) < 55 {
            let pi = alive[gen_range(0, alive.len())]; self.strikes.push((self.players[pi].body.x + gen_range(-160.0, 160.0), 50));
        }
        if let Some(el) = self.weather {
            if self.tick % 2 == 0 { for &pi in &alive { let x = (self.players[pi].body.x + gen_range(-150.0, 150.0)) as i32; let y = ((self.players[pi].body.y - 60.0).max(4.0)) as i32; if self.world.get(x, y) == EMPTY { self.world.set(x, y, el); } } }
        }
        // strikes
        let mut strikes = std::mem::take(&mut self.strikes); let mut next = vec![];
        for (x, t) in strikes.drain(..) {
            if t > 1 { next.push((x, t - 1)); continue; }
            let ix = (x as i32).clamp(4, WORLD_W as i32 - 5); let mut y = 3; while y < WORLD_H as i32 - 3 && !self.world.solid_at(ix, y) { y += 1; }
            let mut xx = ix; for yy in 3..y { xx = (xx + gen_range(-1, 2)).clamp(3, WORLD_W as i32 - 3); if self.world.get(xx, yy) == EMPTY { self.world.set(xx, yy, THUNDER); } }
            self.explode_at(ix as f32, y as f32 - 2.0, 6, 40.0, Owner::World);
            self.sfx_at("thunder_strike", ix as f32, y as f32);
        }
        self.strikes = next;
    }

    // ------------------------------------------------------------------ missions
    pub fn init_missions(&mut self) {
        let id = self.pdef().id;
        self.runs = for_planet(id).into_iter().map(|di| { let done = self.save.completed.contains(MISSIONS[di].id); MissionRun { def: di, progress: if done { target(&MISSIONS[di].obj) } else { 0 }, done, timer: 0, spawned: false } }).collect();
        self.ui.tracked = 0;
        if let Some(r) = self.runs.iter().find(|r| !r.done && !MISSIONS[r.def].optional) { let m = &MISSIONS[r.def]; let (w, b) = (m.who, m.brief); self.say(w, b); }
    }
    pub fn mission_active(&self, ri: usize) -> bool {
        let r = &self.runs[ri]; if r.done { return false; }
        let m = &MISSIONS[r.def]; if m.optional { return true; }
        self.runs.iter().take(ri).filter(|x| !MISSIONS[x.def].optional).all(|x| x.done)
    }
    pub fn tracked_mission(&self) -> Option<usize> {
        let act: Vec<usize> = (0..self.runs.len()).filter(|&i| self.mission_active(i)).collect();
        act.get(self.ui.tracked % act.len().max(1)).copied().or_else(|| None)
    }

    pub fn update_missions(&mut self) {
        let n = self.runs.len();
        for ri in 0..n {
            if !self.mission_active(ri) { continue; }
            let obj = MISSIONS[self.runs[ri].def].obj;
            let alive = self.alive_players();
            let mut finished = false;
            match obj {
                Obj::Reach(z) => { if let Some(zone) = self.gen.zone(z) { if alive.iter().any(|&p| zone.contains(self.players[p].body.x, self.players[p].body.y - 8.0)) { finished = true; } } }
                Obj::Fill(el, z, cnt) => { if self.tick % 20 == 0 { if let Some(zone) = self.gen.zone(z).cloned() { let c = self.world.count_in(zone.x0, zone.y0, zone.x1, zone.y1, |e| e == el) as u32; self.runs[ri].progress = c.min(cnt); if c >= cnt { finished = true; } } } }
                Obj::Burn(z) => { if self.tick % 20 == 0 { if let Some(zone) = self.gen.zone(z).cloned() { let rem = self.world.count_in(zone.x0, zone.y0, zone.x1, zone.y1, |e| e == WOOD || e == PLANT || e == FIRE) as u32; let init = zone.initial.max(1) as u32; self.runs[ri].progress = ((init.saturating_sub(rem)) * 100 / init).min(100); if rem * 100 <= init * 15 { finished = true; } } } }
                Obj::Survive(secs) => {
                    if let Some(zone) = self.gen.zone("goal").cloned() {
                        if alive.iter().any(|&p| zone.contains(self.players[p].body.x, self.players[p].body.y - 8.0)) {
                            self.runs[ri].timer += 1; self.runs[ri].progress = self.runs[ri].timer / 60;
                            if self.runs[ri].timer % 300 == 30 { for _ in 0..(2 + self.pdef().tier as usize / 2) { let (cx, cy) = zone.center(); let side = if gen_range(0, 2) == 0 { -1.0 } else { 1.0 }; let p = self.pdef(); let key = p.enemies[gen_range(0, p.enemies.len())]; if let Some(di) = index(key) { let x = (cx + side * gen_range(70.0, 130.0)).clamp(30.0, WORLD_W as f32 - 30.0); self.spawn_enemy(di, x, cy - 20.0); } } }
                            if self.runs[ri].progress >= secs { finished = true; }
                        }
                    }
                }
                Obj::Boss => {
                    if !self.boss_spawned { if let (Some(zone), Some(bk)) = (self.gen.zone("arena").cloned(), self.pdef().boss) {
                        if alive.iter().any(|&p| self.players[p].body.x > zone.x0 as f32 - 90.0 && self.players[p].body.x < zone.x1 as f32 + 40.0) {
                            if let Some(di) = index(bk) { let (cx, _) = zone.center(); self.spawn_enemy(di, cx + 20.0, zone.y1 as f32 - 40.0); self.boss_spawned = true; self.boss_alive = true; self.say(ENEMIES[di].name, "You dare enter my arena?!"); self.sfx("boss_roar"); let a = &self.assets; self.audio.music(a, "boss"); }
                        } } }
                }
                Obj::Collect(cnt) => {
                    if !self.runs[ri].spawned { self.runs[ri].spawned = true; let spots = self.gen.shard_spots.clone(); for k in 0..(cnt as usize).min(spots.len()) { self.pickups.push(Pickup { kind: PickKind::Shard, x: spots[k].0 as f32, y: spots[k].1 as f32, vy: 0.0, life: u32::MAX }); } }
                }
                _ => {}
            }
            if finished { self.complete_mission(ri); }
        }
    }

    pub fn mission_kill(&mut self, key: &str, boss: bool) {
        for ri in 0..self.runs.len() {
            if !self.mission_active(ri) { continue; }
            let obj = MISSIONS[self.runs[ri].def].obj;
            let done = match obj {
                Obj::Kill(n) => { self.runs[ri].progress += 1; self.runs[ri].progress >= n }
                Obj::KillKind(k, n) => { if k == key { self.runs[ri].progress += 1; } self.runs[ri].progress >= n }
                Obj::Boss => boss,
                _ => false,
            };
            if done { self.complete_mission(ri); }
        }
        if boss { let m = self.pdef().music; let a = &self.assets; self.audio.music(a, m); }
    }
    pub fn mission_collect(&mut self) {
        for ri in 0..self.runs.len() { if !self.mission_active(ri) { continue; } if let Obj::Collect(n) = MISSIONS[self.runs[ri].def].obj { self.runs[ri].progress += 1; if self.runs[ri].progress >= n { self.complete_mission(ri); } } }
    }
    pub fn mission_interact(&mut self, kind: HotKind) {
        for ri in 0..self.runs.len() { if !self.mission_active(ri) { continue; } if let Obj::Interact(k, n) = MISSIONS[self.runs[ri].def].obj { if k == kind { self.runs[ri].progress += 1; if self.runs[ri].progress >= n { self.complete_mission(ri); } } } }
    }

    pub fn cleared(&self, planet: &str) -> bool { MISSIONS.iter().filter(|m| m.planet == planet && !m.optional).all(|m| self.save.completed.contains(m.id)) }
    pub fn planet_unlocked(&self, pi: usize) -> bool {
        if self.cheats.unlock_all || self.mode_free() { return true; }
        let p = &PLANETS[pi];
        if p.system == 1 && !self.save.gate_open { return false; }
        p.after.iter().all(|a| self.cleared(a))
    }
    pub fn mode_free(&self) -> bool { self.ui.free_pick }

    pub fn complete_mission(&mut self, ri: usize) {
        if self.runs[ri].done { return; }
        let before: Vec<bool> = (0..PLANETS.len()).map(|i| self.planet_unlocked(i)).collect();
        let m = &MISSIONS[self.runs[ri].def];
        self.runs[ri].done = true; self.runs[ri].progress = target(&m.obj);
        self.save.completed.insert(m.id.to_string()); self.save.silver += m.silver;
        self.toast(format!("MISSION COMPLETE: {}  (+{} silver)", m.title, m.silver)); self.sfx("mission_done");
        self.say(m.who, m.done);
        if let Some(w) = m.weapon { self.save.unlocked.insert(w.to_string()); if let Some(wi) = weapons::index(w) { for p in self.players.iter_mut() { p.give(wi, true); } } self.toast(format!("New weapon: {}", WEAPONS[weapons::index(w).unwrap()].name)); }
        let main_idx = self.runs.iter().filter(|r| !MISSIONS[r.def].optional).position(|r| r.def == self.runs[ri].def);
        if !m.optional && main_idx == Some(2) {
            self.save.shards += 1; self.toast(format!("SAMPO SHARD {}/14 recovered!", self.save.shards));
            self.pickups.push(Pickup { kind: PickKind::Health, x: self.players[0].body.x, y: self.players[0].body.y - 10.0, vy: -2.0, life: 1800 });
        }
        if m.id == "asgard_m3" && !self.save.gate_open { self.save.gate_open = true; self.start_story(crate::story::ACT2, After::Play); }
        if m.id == "sampola_m3" && !self.save.ending_seen { self.save.ending_seen = true; self.start_story(crate::story::ENDING, After::Title); }
        for i in 0..PLANETS.len() { if !before[i] && self.planet_unlocked(i) { self.toast(format!("New world unlocked: {}", PLANETS[i].name)); } }
        self.save.write();
        // announce next main mission
        if let Some(r) = self.runs.iter().enumerate().find(|(i, r)| !r.done && !MISSIONS[r.def].optional && self.mission_active(*i)).map(|(_, r)| r.def) { let n = &MISSIONS[r]; let (w, b) = (n.who, n.brief); self.say(w, b); }
    }

    pub fn start_story(&mut self, pages: &[(&str, &str)], after: After) {
        self.story = Some(StoryState { pages: pages.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect(), idx: 0, chars: 0.0, after });
        self.scene = Scene::Story;
    }
}
