//! Enemy and boss definitions + runtime struct (AI lives in ai.rs).
use crate::entity::Body;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Ai { Ranged, Melee, Flyer, Ghost, Quad, Boss(Boss) }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Boss { Fenrir, Surtr, Ymir, Jormungandr, Hel, Louhi, Tuoni, RautaGolem, UkkoEagle }

pub struct EnemyDef {
    pub key: &'static str,
    pub name: &'static str,
    pub hp: f32,
    pub speed: f32,
    pub jump: f32,
    pub w: f32,
    pub h: f32,
    /// sprite draw scale
    pub scale: f32,
    pub ai: Ai,
    /// ranged: bullet dmg, speed, spread, ticks between shots/bursts
    pub shot: (f32, f32, f32, u32),
    pub melee: f32,
    pub sight: f32,
    pub fire_immune: bool,
    pub cold_immune: bool,
    pub silver: u32,
    pub frames: u32,
    pub fw: f32,
    pub fh: f32,
}

macro_rules! en { ($k:expr,$n:expr,$hp:expr,$sp:expr,$j:expr,$w:expr,$h:expr,$sc:expr,$ai:expr,$shot:expr,$mel:expr,$sight:expr,$fi:expr,$ci:expr,$si:expr,$fr:expr,$fw:expr,$fh:expr) => {
    EnemyDef { key: $k, name: $n, hp: $hp, speed: $sp, jump: $j, w: $w, h: $h, scale: $sc, ai: $ai, shot: $shot, melee: $mel, sight: $sight, fire_immune: $fi, cold_immune: $ci, silver: $si, frames: $fr, fw: $fw, fh: $fh } } }

pub static ENEMIES: [EnemyDef; 19] = [
    en!("draugr", "Draugr", 60.0, 0.9, 3.6, 6.0, 20.0, 1.0, Ai::Ranged, (7.0, 9.0, 0.08, 50), 0.0, 170.0, false, false, 6, 8, 16.0, 24.0),
    en!("berserker", "Berserker", 110.0, 1.7, 4.2, 6.0, 20.0, 1.1, Ai::Melee, (0.0, 0.0, 0.0, 0), 16.0, 190.0, false, false, 9, 8, 16.0, 24.0),
    en!("eldjotunn", "Eldjotunn", 150.0, 0.8, 3.2, 8.0, 26.0, 1.3, Ai::Ranged, (10.0, 8.0, 0.07, 70), 0.0, 180.0, true, false, 12, 8, 16.0, 24.0),
    en!("frostjotunn", "Frostjotunn", 150.0, 0.8, 3.2, 8.0, 26.0, 1.3, Ai::Melee, (0.0, 0.0, 0.0, 0), 20.0, 170.0, false, true, 12, 8, 16.0, 24.0),
    en!("valkyrie", "Valkyrie", 70.0, 1.3, 0.0, 6.0, 20.0, 1.0, Ai::Flyer, (6.0, 10.0, 0.06, 45), 0.0, 220.0, false, false, 10, 8, 16.0, 24.0),
    en!("hiisi", "Hiisi", 45.0, 1.9, 4.5, 6.0, 18.0, 0.9, Ai::Melee, (0.0, 0.0, 0.0, 0), 9.0, 170.0, false, false, 5, 8, 16.0, 24.0),
    en!("kalma", "Kalma", 80.0, 0.6, 0.0, 6.0, 20.0, 1.0, Ai::Ghost, (0.0, 0.0, 0.0, 0), 12.0, 200.0, true, true, 10, 8, 16.0, 24.0),
    en!("louhi_guard", "Louhi Guard", 90.0, 1.0, 3.6, 6.0, 20.0, 1.0, Ai::Ranged, (6.0, 12.0, 0.1, 22), 0.0, 190.0, false, false, 9, 8, 16.0, 24.0),
    en!("svartalf", "Svartalf", 55.0, 0.9, 3.8, 6.0, 20.0, 1.0, Ai::Ranged, (22.0, 18.0, 0.01, 110), 0.0, 260.0, false, false, 10, 8, 16.0, 24.0),
    en!("fenrir_pup", "Fenrir Pup", 50.0, 2.1, 4.4, 14.0, 9.0, 1.0, Ai::Quad, (0.0, 0.0, 0.0, 0), 8.0, 200.0, false, false, 5, 4, 24.0, 16.0),
    // bosses (48x48 sprites)
    en!("fenrir", "Fenrir", 1100.0, 1.6, 4.5, 26.0, 26.0, 1.0, Ai::Boss(Boss::Fenrir), (9.0, 11.0, 0.05, 60), 24.0, 400.0, false, false, 120, 1, 48.0, 48.0),
    en!("surtr", "Surtr", 1900.0, 0.7, 3.0, 18.0, 44.0, 1.0, Ai::Boss(Boss::Surtr), (14.0, 8.0, 0.04, 60), 30.0, 400.0, true, false, 200, 1, 48.0, 48.0),
    en!("ymir", "Ymir's Heir", 2100.0, 0.6, 3.0, 18.0, 44.0, 1.0, Ai::Boss(Boss::Ymir), (12.0, 10.0, 0.05, 60), 30.0, 400.0, false, true, 220, 1, 48.0, 48.0),
    en!("jormungandr", "Jormungandr", 2400.0, 0.5, 2.0, 30.0, 32.0, 1.0, Ai::Boss(Boss::Jormungandr), (10.0, 7.0, 0.06, 70), 30.0, 450.0, false, false, 260, 1, 48.0, 48.0),
    en!("hel", "Hel", 2600.0, 1.0, 3.5, 16.0, 44.0, 1.0, Ai::Boss(Boss::Hel), (12.0, 9.0, 0.05, 55), 26.0, 450.0, false, false, 300, 1, 48.0, 48.0),
    en!("louhi", "Louhi", 2800.0, 1.2, 0.0, 18.0, 40.0, 1.0, Ai::Boss(Boss::Louhi), (11.0, 11.0, 0.05, 50), 24.0, 450.0, false, true, 320, 1, 48.0, 48.0),
    en!("tuoni", "Tuoni", 3000.0, 0.9, 0.0, 18.0, 44.0, 1.0, Ai::Boss(Boss::Tuoni), (12.0, 8.0, 0.05, 55), 26.0, 450.0, true, true, 340, 1, 48.0, 48.0),
    en!("rauta_golem", "Rauta Golem", 4200.0, 0.55, 2.5, 26.0, 42.0, 1.0, Ai::Boss(Boss::RautaGolem), (16.0, 8.0, 0.03, 70), 40.0, 500.0, true, false, 500, 1, 48.0, 48.0),
    en!("ukko_eagle", "Ukko's Eagle", 3200.0, 1.5, 0.0, 22.0, 36.0, 1.0, Ai::Boss(Boss::UkkoEagle), (12.0, 12.0, 0.04, 45), 28.0, 500.0, false, false, 380, 1, 48.0, 48.0),
];

pub fn index(key: &str) -> Option<usize> { let k = key.to_ascii_lowercase(); ENEMIES.iter().position(|e| e.key == k) }
pub fn is_boss(i: usize) -> bool { matches!(ENEMIES[i].ai, Ai::Boss(_)) }

pub struct Enemy {
    pub def: usize,
    pub body: Body,
    pub hp: f32,
    pub max_hp: f32,
    pub cd: u32,
    pub dir: f32,
    pub anim: f32,
    pub hurt: u32,
    pub burn: u32,
    pub slow: u32,
    pub t: u32,
    pub phase: u32,
    pub burst: u32,
    pub aim: f32,
    pub tx: f32,
    pub ty: f32,
    pub dead: bool,
    pub charge: u32,
}

impl Enemy {
    pub fn new(def: usize, x: f32, y: f32, hp_mult: f32) -> Self {
        let d = &ENEMIES[def];
        Enemy { def, body: Body::new(x, y, d.w, d.h), hp: d.hp * hp_mult, max_hp: d.hp * hp_mult, cd: 60, dir: if macroquad::rand::gen_range(0, 2) == 0 { -1.0 } else { 1.0 }, anim: 0.0, hurt: 0, burn: 0, slow: 0, t: 0, phase: 0, burst: 0, aim: 0.0, tx: x, ty: y, dead: false, charge: 0 }
    }
    pub fn d(&self) -> &'static EnemyDef { &ENEMIES[self.def] }
}
