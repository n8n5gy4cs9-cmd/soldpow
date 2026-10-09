//! Projectiles and pickups (data only; logic in combat.rs / play.rs).
use macroquad::prelude::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Owner { Player(usize), Enemy, World }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Proj { Bullet, Flame, Frost, Spray(u8), Rocket, Grenade, Trick, Bolt, Fireball, Thunder }

#[derive(Clone, Copy, Debug)]
pub struct Bullet {
    pub x: f32, pub y: f32, pub vx: f32, pub vy: f32,
    pub life: i32,
    pub proj: Proj,
    pub dmg: f32,
    pub owner: Owner,
    pub radius: i32,
    pub grav: f32,
    pub bounce: u8,
    pub fuse: i32,
    pub pierce: bool,
    pub color: Color,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PickKind { Health, Ammo, Shard, Silver(u32), Weapon(usize) }

#[derive(Clone, Copy, Debug)]
pub struct Pickup { pub kind: PickKind, pub x: f32, pub y: f32, pub vy: f32, pub life: u32 }
