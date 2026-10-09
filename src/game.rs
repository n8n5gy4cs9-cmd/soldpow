//! Central game state + landing + fixed-step tick. Other modules add `impl Game` blocks.
use crate::assets::Assets;
use crate::audio::Audio;
use crate::bullet::*;
use crate::elements::*;
use crate::enemy::*;
use crate::entity::*;
use crate::hotspot::*;
use crate::missions::*;
use crate::planets::*;
use crate::player::*;
use crate::save::Save;
use crate::sim::*;
use crate::terrain::{self, Generated};
use crate::weapons;
use macroquad::prelude::*;
use macroquad::rand::gen_range;
use std::collections::HashMap;

pub const WORLD_W: usize = 1280;
pub const WORLD_H: usize = 640;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Scene { Title, Settings, Credits, Controls, Map, Play, Story }
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Mode { Campaign, Sandbox, Versus }
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum After { Map, Play, Title }

pub struct Cheats { pub unlock_all: bool, pub god: bool, pub ghost: bool, pub fly: bool, pub inf_ammo: bool, pub speed: f32, pub jump: f32, pub grav: f32 }
impl Default for Cheats { fn default() -> Self { Cheats { unlock_all: false, god: false, ghost: false, fly: false, inf_ammo: false, speed: 1.0, jump: 1.0, grav: 1.0 } } }
pub struct Brush { pub el: u8, pub size: i32, pub build: bool, pub last: Option<(i32, i32)> }
pub struct Radio { pub who: String, pub text: String, pub t: f32 }
pub struct StoryState { pub pages: Vec<(String, String)>, pub idx: usize, pub chars: f32, pub after: After }
#[derive(Default)]
pub struct Console { pub open: bool, pub input: String, pub history: Vec<String>, pub hist_idx: usize, pub out: Vec<String>, pub bs_hold: u32 }
#[derive(Default)]
pub struct UiState { pub free_pick: bool, pub picker: bool, pub journal: bool, pub cheat_menu: bool, pub paused: bool, pub map_sys: u8, pub hover: Option<usize>, pub settings_tab: usize, pub flash: f32, pub tracked: usize }

#[derive(Clone, Copy)]
pub struct View { pub x: f32, pub y: f32, pub w: f32, pub h: f32, pub cx: f32, pub cy: f32, pub z: f32 }
impl View {
    pub fn to_screen(&self, wx: f32, wy: f32) -> (f32, f32) { (self.x + self.w / 2.0 + (wx - self.cx) * self.z, self.y + self.h / 2.0 + (wy - self.cy) * self.z) }
    pub fn to_world(&self, sx: f32, sy: f32) -> (f32, f32) { (self.cx + (sx - self.x - self.w / 2.0) / self.z, self.cy + (sy - self.y - self.h / 2.0) / self.z) }
    pub fn visible(&self, wx: f32, wy: f32, pad: f32) -> bool { (wx - self.cx).abs() * self.z < self.w / 2.0 + pad * self.z && (wy - self.cy).abs() * self.z < self.h / 2.0 + pad * self.z }
}

pub struct Game {
    pub assets: Assets,
    pub audio: Audio,
    pub scene: Scene,
    pub mode: Mode,
    pub world: World,
    pub img: Image,
    pub tex: Texture2D,
    pub planet: usize,
    pub gen: Generated,
    pub players: Vec<Player>,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub pickups: Vec<Pickup>,
    pub particles: Vec<Particle>,
    pub hotspots: Vec<Hotspot>,
    pub runs: Vec<MissionRun>,
    pub save: Save,
    pub tick: u64,
    pub cheats: Cheats,
    pub brush: Brush,
    pub console: Console,
    pub ui: UiState,
    pub toasts: Vec<(String, f32)>,
    pub radio: Vec<Radio>,
    pub kills: u32,
    pub kind_kills: HashMap<String, u32>,
    pub shake: f32,
    pub sim_paused: bool,
    pub sim_speed: u32,
    pub step_once: bool,
    pub weather: Option<u8>,
    pub split: bool,
    pub zoom: f32,
    pub cams: [(f32, f32); 2],
    pub story: Option<StoryState>,
    pub boss_spawned: bool,
    pub boss_alive: bool,
    pub strikes: Vec<(f32, u32)>,
    pub show_fps: bool,
    pub debug: bool,
    pub banner: Option<(String, f32)>,
    pub versus_target: u32,
    pub versus_won: Option<usize>,
    pub screen_w: f32,
    pub screen_h: f32,
    pub quit: bool,
    pub acc: f32,
    pub sandbox_planet: usize,
    pub audio_jump: bool,
    pub prev_scene: Scene,
    pub pending: (Mode, bool),
    pub in_game: bool,
    pub mouse_world: (f32, f32),
    pub fullscreen: bool,
    pub confirm_reset: bool,
}

impl Game {
    pub fn new(assets: Assets, sound: bool) -> Game {
        let world = World::new(WORLD_W, WORLD_H);
        let img = Image::gen_image_color(WORLD_W as u16, WORLD_H as u16, Color::new(0.0, 0.0, 0.0, 0.0));
        let tex = Texture2D::from_image(&img); tex.set_filter(FilterMode::Nearest);
        let save = Save::load();
        let mut audio = Audio::new(save.sfx_vol, save.music_vol); if !sound { audio.music_on = false; }
        let zoom = save.zoom;
        Game {
            assets, audio, scene: Scene::Title, mode: Mode::Campaign, world, img, tex, planet: 0,
            gen: terrain::Generated { spawn: (60, 60), hotspots: vec![], zones: vec![], enemy_spawns: vec![], shard_spots: vec![], surface: vec![0; WORLD_W] },
            players: vec![], enemies: vec![], bullets: vec![], pickups: vec![], particles: vec![], hotspots: vec![], runs: vec![], save, tick: 0,
            cheats: Cheats::default(), brush: Brush { el: SAND, size: 4, build: false, last: None }, console: Console::default(), ui: UiState::default(), toasts: vec![], radio: vec![],
            kills: 0, kind_kills: HashMap::new(), shake: 0.0, sim_paused: false, sim_speed: 1, step_once: false, weather: None, split: false, zoom, cams: [(0.0, 0.0); 2],
            story: None, boss_spawned: false, boss_alive: false, strikes: vec![], show_fps: false, debug: false, banner: None, versus_target: 15, versus_won: None,
            screen_w: 1280.0, screen_h: 720.0, quit: false, acc: 0.0, sandbox_planet: 0, audio_jump: false, prev_scene: Scene::Credits, pending: (Mode::Campaign, false), in_game: false, mouse_world: (0.0, 0.0), fullscreen: false, confirm_reset: false,
        }
    }

    pub fn pdef(&self) -> &'static PlanetDef { &PLANETS[self.planet] }
    pub fn toast(&mut self, s: impl Into<String>) { let s = s.into(); if self.toasts.last().map_or(true, |t| t.0 != s) { self.toasts.push((s, 4.0)); } if self.toasts.len() > 6 { self.toasts.remove(0); } }
    pub fn say(&mut self, who: &str, text: &str) { self.radio.push(Radio { who: who.into(), text: text.into(), t: 9.0 }); if self.radio.len() > 3 { self.radio.remove(0); } }
    pub fn sfx(&mut self, name: &str) { let a = &self.assets; self.audio.sfx(a, name, 1.0); }
    pub fn sfx_at(&mut self, name: &str, x: f32, y: f32) {
        let mut d = f32::MAX;
        for p in &self.players { d = d.min(((p.body.x - x).powi(2) + (p.body.y - y).powi(2)).sqrt()); }
        if d == f32::MAX { d = 0.0; }
        let gain = 1.0 / (1.0 + d / 160.0); let a = &self.assets; self.audio.sfx(a, name, gain);
    }
    pub fn shake_at(&mut self, x: f32, y: f32, amt: f32) {
        let mut d = f32::MAX; for p in &self.players { d = d.min(((p.body.x - x).powi(2) + (p.body.y - y).powi(2)).sqrt()); }
        self.shake = (self.shake + amt / (1.0 + d / 120.0)).min(12.0);
    }

    // ------------------------------------------------------------------ particles
    pub fn spark(&mut self, x: f32, y: f32, col: Color, n: u32, speed: f32, life: f32, grav: f32) {
        for _ in 0..n {
            if self.particles.len() > 1800 { return; }
            let a = gen_range(0.0, std::f32::consts::TAU); let s = gen_range(0.2, 1.0) * speed;
            self.particles.push(Particle { x, y, vx: a.cos() * s, vy: a.sin() * s, life, max: life, col, size: gen_range(1.0, 2.2), grav });
        }
    }
    pub fn blood(&mut self, x: f32, y: f32, n: u32, dirx: f32) {
        for _ in 0..n {
            if self.particles.len() > 1800 { return; }
            self.particles.push(Particle { x, y, vx: dirx * gen_range(0.2, 1.5) + gen_range(-1.0, 1.0), vy: gen_range(-2.0, 0.5), life: 0.6, max: 0.6, col: Color::new(0.7, 0.04, 0.08, 1.0), size: 1.5, grav: 0.2 });
        }
        if gen_range(0, 3) == 0 { let (cx, cy) = (x as i32, y as i32); if self.world.get(cx, cy) == EMPTY { self.world.set(cx, cy, BLOOD); } }
    }

    // ------------------------------------------------------------------ landing
    pub fn land(&mut self, planet: usize, mode: Mode, two_players: bool) {
        self.planet = planet; self.mode = mode; self.split = two_players;
        let p = &PLANETS[planet];
        self.gen = terrain::generate(&mut self.world, p);
        self.world.seed(0x9E3779B9 ^ p.seed);
        self.enemies.clear(); self.bullets.clear(); self.pickups.clear(); self.particles.clear(); self.strikes.clear(); self.radio.clear();
        self.kills = 0; self.kind_kills.clear(); self.boss_spawned = false; self.boss_alive = false; self.tick = 0; self.versus_won = None;
        self.hotspots = self.gen.hotspots.clone();
        let (sx, sy) = self.gen.spawn;
        self.players.clear();
        let n = if two_players { 2 } else { 1 };
        let all = mode != Mode::Campaign;
        for i in 0..n {
            let mut pl = Player::new(i, sx as f32 + i as f32 * 10.0, sy as f32);
            pl.loadout(&self.save.unlocked, all); if i == 1 { pl.aim = 0.0; }
            self.players.push(pl);
        }
        if mode == Mode::Versus { self.gen_versus_spawns(); }
        if mode == Mode::Campaign { self.init_missions(); }
        else { self.runs.clear(); }
        if mode == Mode::Campaign {
            for _ in 0..p.enemy_count { self.spawn_random_enemy(true); }
            self.scatter_pickups(true);
        } else if mode == Mode::Versus { self.scatter_pickups(false); }
        if mode == Mode::Sandbox { self.brush.build = true; self.cheats.inf_ammo = true; } else { self.brush.build = false; }
        self.cams = [(sx as f32, sy as f32); 2];
        self.scene = Scene::Play; self.ui.paused = false; self.ui.picker = false;
        self.save.last_planet = p.id.to_string();
        let a = &self.assets; self.audio.music(a, p.music);
        self.say("", crate::story::arrival(p.id));
        self.banner = Some((format!("{} - {}", p.name.to_uppercase(), p.analog), 3.5));
    }

    fn gen_versus_spawns(&mut self) {
        // second player starts on the far side of the map
        let x = (WORLD_W as f32 * 0.9) as usize; let y = self.gen.surface[x] as f32 - 2.0;
        if self.players.len() > 1 { self.players[1].body.x = x as f32; self.players[1].body.y = y; }
    }

    pub fn spawn_random_enemy(&mut self, far: bool) {
        let p = &PLANETS[self.planet]; if self.gen.enemy_spawns.is_empty() || p.enemies.is_empty() { return; }
        let mut pick = self.gen.enemy_spawns[gen_range(0, self.gen.enemy_spawns.len())];
        for _ in 0..8 {
            let c = self.gen.enemy_spawns[gen_range(0, self.gen.enemy_spawns.len())];
            let d = self.players.iter().map(|pl| (pl.body.x - c.0 as f32).abs()).fold(f32::MAX, f32::min);
            if !far || d > 200.0 { pick = c; break; }
        }
        let key = p.enemies[gen_range(0, p.enemies.len())];
        if let Some(di) = index(key) { self.spawn_enemy(di, pick.0 as f32, pick.1 as f32); }
    }
    pub fn spawn_enemy(&mut self, di: usize, x: f32, y: f32) {
        let mult = 1.0 + 0.12 * (self.pdef().tier as f32 - 1.0) + if self.players.len() > 1 { 0.5 } else { 0.0 };
        let mut e = Enemy::new(di, x, y, mult);
        if matches!(e.d().ai, Ai::Flyer | Ai::Ghost) { e.body.y = y.min(self.gen.surface[(x as usize).min(WORLD_W - 1)] as f32 - 30.0); }
        self.enemies.push(e);
    }

    fn scatter_pickups(&mut self, with_shards: bool) {
        let _ = with_shards;
        for k in 0..6 {
            let x = (WORLD_W as f32 * (0.12 + 0.13 * k as f32)) as usize + gen_range(0, 30);
            let y = self.gen.surface[x.min(WORLD_W - 1)] as f32 - 6.0;
            let kind = if k % 2 == 0 { PickKind::Health } else { PickKind::Ammo };
            self.pickups.push(Pickup { kind, x: x as f32, y, vy: 0.0, life: u32::MAX });
        }
        for k in 0..2 {
            let x = (WORLD_W as f32 * (0.3 + 0.35 * k as f32)) as usize; let y = self.gen.surface[x] as f32 - 6.0;
            let tier = self.pdef().tier.max(1);
            let cands: Vec<usize> = weapons::WEAPONS.iter().enumerate().filter(|(_, w)| w.tier <= tier && w.key != "puukko").map(|(i, _)| i).collect();
            if !cands.is_empty() { self.pickups.push(Pickup { kind: PickKind::Weapon(cands[gen_range(0, cands.len())]), x: x as f32, y, vy: 0.0, life: u32::MAX }); }
        }
    }

    pub fn alive_players(&self) -> Vec<usize> { self.players.iter().enumerate().filter(|(_, p)| p.alive).map(|(i, _)| i).collect() }
    pub fn nearest_player(&self, x: f32, y: f32) -> Option<usize> {
        self.players.iter().enumerate().filter(|(_, p)| p.alive).min_by(|a, b| {
            let da = (a.1.body.x - x).powi(2) + (a.1.body.y - y).powi(2); let db = (b.1.body.x - x).powi(2) + (b.1.body.y - y).powi(2); da.partial_cmp(&db).unwrap()
        }).map(|(i, _)| i)
    }

    /// Straight-line sight test through non-solid cells.
    pub fn line_clear(&self, x0: f32, y0: f32, x1: f32, y1: f32) -> bool {
        let dx = x1 - x0; let dy = y1 - y0; let n = (dx.abs().max(dy.abs()) / 2.0).ceil() as i32;
        for i in 1..n { let t = i as f32 / n as f32; if self.world.solid_at((x0 + dx * t) as i32, (y0 + dy * t) as i32) { return false; } }
        true
    }

    // ------------------------------------------------------------------ views
    pub fn compute_views(&mut self) -> Vec<View> {
        self.screen_w = screen_width(); self.screen_h = screen_height();
        let n = if self.split { 2 } else { 1 };
        let mut v = Vec::new();
        for i in 0..n {
            let (w, x) = if n == 2 { (self.screen_w / 2.0 - 1.0, if i == 0 { 0.0 } else { self.screen_w / 2.0 + 1.0 }) } else { (self.screen_w, 0.0) };
            let h = self.screen_h;
            let mut z = if n == 2 { (self.zoom * 0.75).max(1.5) } else { self.zoom };
            z = z.max(h / WORLD_H as f32).max(w / WORLD_W as f32);
            let (mut cx, mut cy) = self.cams[i];
            let (hw, hh) = (w / z / 2.0, h / z / 2.0);
            cx = cx.clamp(hw, WORLD_W as f32 - hw); cy = cy.clamp(hh, WORLD_H as f32 - hh);
            v.push(View { x, y: 0.0, w, h, cx, cy, z });
        }
        v
    }
    pub fn update_cams(&mut self) {
        for i in 0..self.players.len().min(2) {
            let p = &self.players[i]; let (px, py) = p.body.center();
            let lead = if p.alive { (p.aim.cos() * 30.0, p.aim.sin() * 20.0) } else { (0.0, 0.0) };
            let (tx, ty) = (px + lead.0 * 0.6, py + lead.1 * 0.6 - 4.0);
            let c = &mut self.cams[i]; c.0 += (tx - c.0) * 0.12; c.1 += (ty - c.1) * 0.12;
        }
    }

    // ------------------------------------------------------------------ main fixed tick
    pub fn tick(&mut self) {
        self.tick += 1; self.audio.tick = self.tick;
        // powder world
        if !self.sim_paused || self.step_once {
            for _ in 0..self.sim_speed { self.world.step(); }
            self.step_once = false;
        }
        self.process_sim_events();
        self.hazards();
        for i in 0..self.players.len() { self.update_player(i); }
        self.update_enemies();
        self.update_bullets();
        self.update_pickups();
        self.update_particles();
        if self.mode == Mode::Campaign { self.update_missions(); self.upkeep_enemies(); }
        if self.mode == Mode::Versus { self.update_versus(); }
        self.update_cams();
        self.shake *= 0.9;
        for p in self.players.iter_mut() { p.ctl.up_pressed = false; p.ctl.fire_pressed = false; p.ctl.nade_pressed = false; p.ctl.interact = false; p.ctl.next = false; p.ctl.prev = false; p.ctl.reload = false; }
    }

    fn update_particles(&mut self) {
        let dt = 1.0 / 60.0;
        for p in self.particles.iter_mut() { p.x += p.vx; p.y += p.vy; p.vy += p.grav; p.vx *= 0.98; p.life -= dt; }
        self.particles.retain(|p| p.life > 0.0);
    }

    fn upkeep_enemies(&mut self) {
        if self.tick % 600 == 0 && self.enemies.iter().filter(|e| !is_boss(e.def)).count() < (self.pdef().enemy_count / 2) as usize { self.spawn_random_enemy(true); }
    }

    fn update_versus(&mut self) {
        if self.tick % 1800 == 0 {
            let x = gen_range(150, WORLD_W - 150); let y = self.gen.surface[x] as f32 - 6.0;
            let tier = 5u8; let cands: Vec<usize> = weapons::WEAPONS.iter().enumerate().filter(|(_, w)| w.tier <= tier && w.key != "puukko").map(|(i, _)| i).collect();
            self.pickups.push(Pickup { kind: PickKind::Weapon(cands[gen_range(0, cands.len())]), x: x as f32, y, vy: 0.0, life: 3600 });
            self.pickups.push(Pickup { kind: PickKind::Health, x: x as f32 + 8.0, y, vy: 0.0, life: 3600 });
        }
        if self.versus_won.is_none() {
            for (i, p) in self.players.iter().enumerate() { if p.kills >= self.versus_target { self.versus_won = Some(i); self.banner = Some((format!("PLAYER {} WINS!  (R = rematch)", i + 1), 99.0)); } }
        }
    }

    pub fn process_sim_events(&mut self) {
        let evs: Vec<SimEvent> = self.world.events.drain(..).collect();
        for ev in evs {
            match ev {
                SimEvent::Explosion { x, y, r } => {
                    self.blast_effects(x as f32, y as f32, r as f32, r as f32 * 6.0, Owner::World);
                    self.sfx_at(if r > 8 { "explosion_big" } else { "explosion" }, x as f32, y as f32); self.shake_at(x as f32, y as f32, r as f32 * 0.5);
                }
                SimEvent::Sizzle { x, y } => { if self.tick % 5 == 0 { self.sfx_at("sizzle", x as f32, y as f32); } }
                SimEvent::Splash { x, y } => self.sfx_at("splash", x as f32, y as f32),
            }
        }
    }
}
