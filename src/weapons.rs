//! Weapon table. Console names double as asset names (assets/weapons/<key>.png).
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Fire {
    Melee,
    Bullet,
    Pellets(u8),
    Flame,
    Frost,
    /// 0 = selected brush element, 255 = random element, else fixed element id
    Spray(u8),
    /// 0 = thunder, 1 = rail
    Beam(u8),
    Rocket,
    Grenade,
    Trick,
}

pub struct WeaponDef {
    pub key: &'static str,
    pub name: &'static str,
    pub desc: &'static str,
    pub fire: Fire,
    pub dmg: f32,
    pub rate: u32,
    pub speed: f32,
    pub spread: f32,
    pub mag: u32,
    pub reserve: u32,
    pub reload: u32,
    pub sfx: &'static str,
    pub auto: bool,
    /// projectile lifetime in ticks (beam range for beams)
    pub life: i32,
    /// explosion radius (rocket/grenade/trick) or crater radius for bullets
    pub radius: i32,
    pub tier: u8,
}

macro_rules! w { ($k:expr,$n:expr,$d:expr,$f:expr,$dmg:expr,$rate:expr,$sp:expr,$spread:expr,$mag:expr,$res:expr,$rel:expr,$sfx:expr,$auto:expr,$life:expr,$rad:expr,$tier:expr) => {
    WeaponDef { key: $k, name: $n, desc: $d, fire: $f, dmg: $dmg, rate: $rate, speed: $sp, spread: $spread, mag: $mag, reserve: $res, reload: $rel, sfx: $sfx, auto: $auto, life: $life, radius: $rad, tier: $tier } } }

pub static WEAPONS: [WeaponDef; 18] = [
    w!("puukko", "Puukko", "Finnish knife. Silent, free, personal.", Fire::Melee, 30.0, 22, 0.0, 0.0, 0, 0, 0, "melee", false, 0, 0, 1),
    w!("hugin", "Hugin 9mm", "Odin's raven, in pistol form.", Fire::Bullet, 15.0, 14, 14.0, 0.02, 12, 96, 70, "shot_pistol", false, 60, 1, 1),
    w!("munin", "Munin Revolver", "Remembers every shot.", Fire::Bullet, 40.0, 34, 16.0, 0.012, 6, 36, 100, "shot_revolver", false, 70, 1, 1),
    w!("lemminkainen_smg", "Lemminkainen SMG", "Reckless fire rate.", Fire::Bullet, 8.0, 4, 15.0, 0.07, 30, 240, 80, "shot_smg", true, 55, 1, 2),
    w!("vainamoinen_gun", "Vainamoinen's Kantele Rifle", "Sings 5.56 mm runes.", Fire::Bullet, 12.0, 6, 18.0, 0.035, 30, 210, 90, "shot_rifle", true, 70, 1, 2),
    w!("kullervo_shotgun", "Kullervo Shotgun", "Tragic, loud, effective.", Fire::Pellets(8), 7.0, 38, 13.0, 0.16, 6, 36, 110, "shot_shotgun", false, 28, 1, 2),
    w!("gungnir_sniper", "Gungnir Sniper", "Never misses (if you don't).", Fire::Bullet, 95.0, 70, 40.0, 0.002, 5, 25, 120, "shot_sniper", false, 40, 2, 3),
    w!("mjolnir_rpg", "Mjolnir RPG", "Thor's hammer, delivered.", Fire::Rocket, 90.0, 90, 7.0, 0.0, 1, 8, 120, "shot_rocket", false, 240, 16, 3),
    w!("gjallar_grenade", "Gjallar Grenade Launcher", "Heimdall's horn goes bang.", Fire::Grenade, 70.0, 50, 8.0, 0.0, 4, 16, 100, "shot_grenade", false, 80, 12, 3),
    w!("surtr_flame", "Surtr Flamethrower", "Muspell's finest export.", Fire::Flame, 1.4, 2, 5.0, 0.13, 100, 300, 120, "shot_flame", true, 26, 0, 3),
    w!("louhi_frost", "Louhi Frost Gun", "Freezes foes and water alike.", Fire::Frost, 2.0, 3, 6.0, 0.10, 80, 240, 110, "shot_ice", true, 30, 0, 3),
    w!("tuoni_acid", "Tuoni Acid Sprayer", "Dissolves problems.", Fire::Spray(crate::elements::ACID), 1.0, 3, 5.5, 0.10, 80, 240, 110, "shot_acid", true, 40, 0, 4),
    w!("ukko_thunder", "Ukko Thunder Gun", "Chain lightning from the sky god.", Fire::Beam(0), 32.0, 26, 0.0, 0.0, 10, 50, 100, "shot_thunder", false, 170, 0, 4),
    w!("ilmarinen_rail", "Ilmarinen Railgun", "Forged, not bought. Pierces anything soft.", Fire::Beam(1), 130.0, 80, 0.0, 0.0, 4, 20, 140, "shot_rail", false, 420, 0, 5),
    w!("sampo_gun", "Sampo Gun", "Grinds out a random element.", Fire::Spray(255), 1.0, 4, 5.0, 0.12, 60, 240, 90, "shot_powder", true, 45, 0, 4),
    w!("powder_gun", "Powder Gun", "Shoots your selected powder element (B menu).", Fire::Spray(0), 0.5, 2, 6.0, 0.10, 200, 999, 60, "shot_powder", true, 45, 0, 1),
    w!("fenrir_minigun", "Fenrir Minigun", "Bites back.", Fire::Bullet, 7.0, 2, 17.0, 0.09, 150, 450, 150, "shot_minigun", true, 60, 1, 4),
    w!("loki_trick", "Loki's Trick", "Explodes into a random mess.", Fire::Trick, 22.0, 30, 9.0, 0.02, 8, 40, 90, "shot_trick", false, 90, 6, 4),
];

pub fn index(key: &str) -> Option<usize> { let k = key.to_ascii_lowercase(); WEAPONS.iter().position(|w| w.key == k) }
