//! Imported Soldat maps ("arenas"): element maps produced by tools/import_soldat_maps.py.
use crate::bullet::*;
use crate::elements::*;
use crate::game::*;
use crate::player::Player;
use crate::terrain::Generated;
use crate::weapons;
use macroquad::prelude::*;
use macroquad::rand::gen_range;

impl Game {
    pub fn land_arena(&mut self, idx: usize, mode: Mode, two_players: bool) {
        let info = self.assets.arenas[idx].clone();
        let path = self.assets.dir.join("maps").join(format!("{}.png", info.stem));
        let Ok(bytes) = std::fs::read(&path) else { self.toast(format!("Missing arena file {}", path.display())); return };
        let Ok(img) = Image::from_file_with_format(&bytes, None) else { self.toast("Could not decode arena map"); return };
        if img.width() as usize != WORLD_W || img.height() as usize != WORLD_H { self.toast("Arena map has the wrong size (expected 1280x640)"); return; }
        self.arena = Some(idx); self.planet = 0; self.mode = mode; self.split = two_players;
        self.world.clear_all();
        for y in 0..WORLD_H { for x in 0..WORLD_W {
            let p = (y * WORLD_W + x) * 4; let (e, b) = (img.bytes[p], img.bytes[p + 1]);
            if e != 0 { self.world.set(x as i32, y as i32, e.min((NUM_ELEMENTS - 1) as u8)); }
            if b != 0 { let i = self.world.idx(x as i32, y as i32); self.world.bg[i] = 1; }
        } }
        self.world.border();
        // surface profile for the parallax horizon
        let mut surface = vec![WORLD_H as i32 / 2; WORLD_W];
        for x in 0..WORLD_W { if let Some(y) = (2..WORLD_H as i32 - 2).find(|&y| is_solid_for_body(self.world.get(x as i32, y))) { surface[x] = y; } }
        let spawn = info.spawns.first().map(|s| (s.0, s.1)).unwrap_or((WORLD_W as i32 / 2, WORLD_H as i32 / 2));
        self.gen = Generated { spawn, hotspots: vec![], zones: vec![], enemy_spawns: vec![], shard_spots: vec![], surface };
        self.enemies.clear(); self.bullets.clear(); self.pickups.clear(); self.particles.clear(); self.strikes.clear(); self.radio.clear(); self.hotspots.clear(); self.runs.clear();
        self.kills = 0; self.kind_kills.clear(); self.boss_spawned = false; self.boss_alive = false; self.tick = 0; self.versus_won = None;
        self.players.clear();
        for i in 0..(if two_players { 2 } else { 1 }) { let mut pl = Player::new(i, 0.0, 0.0); pl.loadout(&self.save.unlocked, true); self.players.push(pl); }
        for i in 0..self.players.len() { let (x, y) = self.arena_spawn(i); self.players[i].body.x = x; self.players[i].body.y = y; }
        // crates and medikits on spawn points
        let tier = 5u8; let cands: Vec<usize> = weapons::WEAPONS.iter().enumerate().filter(|(_, w)| w.tier <= tier && w.key != "puukko").map(|(i, _)| i).collect();
        let n = info.spawns.len();
        if n > 0 { for k in 0..(n.min(8)) { let s = info.spawns[gen_range(0, n)]; let kind = if k % 3 == 0 { PickKind::Health } else if k % 3 == 1 { PickKind::Ammo } else { PickKind::Weapon(cands[gen_range(0, cands.len())]) }; self.pickups.push(Pickup { kind, x: s.0 as f32, y: s.1 as f32 - 4.0, vy: 0.0, life: u32::MAX }); } }
        self.brush.build = mode == Mode::Sandbox; self.cheats.inf_ammo = mode == Mode::Sandbox;
        self.cams = [(self.players[0].body.x, self.players[0].body.y); 2];
        self.scene = Scene::Play; self.ui.paused = false; self.ui.picker = false; self.in_game = true;
        let a = &self.assets; self.audio.music(a, "norse");
        self.say("", &format!("Soldat map '{}' - {}  (original game content, CC BY 4.0)", info.stem, info.title));
        self.banner = Some((info.stem.to_uppercase(), 3.0));
    }

    /// A free spawn position for player `pi` (P2 prefers the spawn farthest from P1).
    pub fn arena_spawn(&mut self, pi: usize) -> (f32, f32) {
        let Some(ai) = self.arena else { return (self.gen.spawn.0 as f32, self.gen.spawn.1 as f32) };
        let spawns = self.assets.arenas[ai].spawns.clone();
        if spawns.is_empty() { return (WORLD_W as f32 / 2.0, WORLD_H as f32 / 2.0); }
        let other = self.players.iter().enumerate().find(|(i, p)| *i != pi && p.alive).map(|(_, p)| (p.body.x, p.body.y));
        let mut best = spawns[gen_range(0, spawns.len())]; let mut bd = -1.0f32;
        for _ in 0..6 {
            let s = spawns[gen_range(0, spawns.len())]; let d = other.map_or(gen_range(0.0, 1.0), |(ox, oy)| (ox - s.0 as f32).hypot(oy - s.1 as f32));
            if d > bd { bd = d; best = s; }
        }
        // find the nearest spot where a full-size body fits (try the spawn and then a few other spawn points)
        let probe = crate::entity::Body::new(0.0, 0.0, crate::player::BODY_W, crate::player::BODY_H);
        let mut order: Vec<(i32, i32, i32)> = vec![best]; for _ in 0..8 { order.push(spawns[gen_range(0, spawns.len())]); }
        for (sx, sy, _) in order {
            for dy in 0..140 { for dx in [0, -4, 4, -8, 8, -14, 14] {
                let (x, y) = ((sx + dx) as f32, (sy - dy) as f32);
                if y > 22.0 && !probe.collides(&self.world, x, y) && !probe.collides(&self.world, x, y - 1.0) { return (x, y); }
            } }
        }
        (WORLD_W as f32 / 2.0, WORLD_H as f32 / 2.0)
    }
}
