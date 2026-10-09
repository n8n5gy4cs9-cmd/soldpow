//! The 14 worlds: 9 Norse (Sol system) + 5 Kalevala (Paivatar system).
use crate::elements::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Hazard { None, EmberRain, AcidRain, Snow, Lightning, Sleet }

pub struct GenParams {
    pub base: f32,
    pub amp: f32,
    pub top: u8,
    pub sub: u8,
    pub deep: u8,
    pub top_depth: i32,
    pub caves: f32,
    /// (element, per-ten-thousand cells)
    pub ores: &'static [(u8, u32)],
    /// open-air fill: (element, level as fraction of height)
    pub sea: Option<(u8, f32)>,
    pub cave_pool: Option<u8>,
    pub islands: bool,
    pub craters: bool,
    /// 0 none, 1 oak, 2 pine, 3 dead, 4 jungle
    pub trees: u8,
    pub cave_gas: bool,
}

pub struct PlanetDef {
    pub id: &'static str,
    pub name: &'static str,
    pub system: u8,
    pub analog: &'static str,
    pub gravity: f32,
    pub sky: [(u8, u8, u8); 2],
    pub hazard: Hazard,
    pub gen: GenParams,
    pub enemies: &'static [&'static str],
    pub enemy_count: u32,
    pub boss: Option<&'static str>,
    pub music: &'static str,
    /// orbit radius (0..1 of map radius) and starting angle in degrees
    pub orbit: (f32, f32),
    pub after: &'static [&'static str],
    pub tier: u8,
    pub seed: u32,
    pub gate: bool,
    pub sauna: bool,
    pub desc: &'static str,
}

macro_rules! gp { ($($k:ident : $v:expr),* $(,)?) => { GenParams { $($k: $v),* } } }

pub static PLANETS: [PlanetDef; 14] = [
    PlanetDef { id: "midgard", name: "Midgard", system: 0, analog: "Earth", gravity: 1.0, sky: [(70, 130, 210), (190, 220, 245)], hazard: Hazard::None,
        gen: gp!(base: 0.52, amp: 38.0, top: DIRT, sub: DIRT, deep: STONE, top_depth: 4, caves: 0.30, ores: &[(COAL, 14), (METAL, 6), (GOLD, 2)], sea: Some((WATER, 0.56)), cave_pool: Some(WATER), islands: false, craters: false, trees: 1, cave_gas: false),
        enemies: &["draugr", "berserker", "fenrir_pup"], enemy_count: 14, boss: Some("fenrir"), music: "calm", orbit: (0.30, 40.0), after: &[], tier: 1, seed: 11, gate: false, sauna: false,
        desc: "Home. Green hills, lakes, old burial mounds. The Sampo shard fell near Odin's workshop." },
    PlanetDef { id: "mani", name: "Mani", system: 0, analog: "Moon", gravity: 0.40, sky: [(4, 4, 14), (20, 22, 40)], hazard: Hazard::None,
        gen: gp!(base: 0.55, amp: 22.0, top: ASH, sub: STONE, deep: STONE, top_depth: 6, caves: 0.22, ores: &[(METAL, 10), (GOLD, 4), (SALT, 10)], sea: None, cave_pool: None, islands: false, craters: true, trees: 0, cave_gas: false),
        enemies: &["draugr", "svartalf"], enemy_count: 12, boss: None, music: "dark", orbit: (0.30, 75.0), after: &["midgard"], tier: 2, seed: 22, gate: false, sauna: false,
        desc: "Low gravity, grey dust, Draugr in vacuum suits. Jump high, fall slowly." },
    PlanetDef { id: "muspell", name: "Muspell", system: 0, analog: "Mercury", gravity: 0.90, sky: [(70, 14, 8), (230, 110, 30)], hazard: Hazard::EmberRain,
        gen: gp!(base: 0.5, amp: 55.0, top: STONE, sub: STONE, deep: STONE, top_depth: 5, caves: 0.40, ores: &[(METAL, 12), (COAL, 28), (GUNPOWDER, 4)], sea: Some((LAVA, 0.535)), cave_pool: Some(LAVA), islands: false, craters: false, trees: 0, cave_gas: false),
        enemies: &["eldjotunn", "draugr", "fenrir_pup"], enemy_count: 16, boss: Some("surtr"), music: "norse", orbit: (0.42, 120.0), after: &["midgard"], tier: 3, seed: 33, gate: false, sauna: false,
        desc: "Lava seas, ember rain, Surtr's smithy at the end of the world." },
    PlanetDef { id: "vanaheim", name: "Vanaheim", system: 0, analog: "Venus", gravity: 0.95, sky: [(80, 90, 30), (200, 210, 110)], hazard: Hazard::AcidRain,
        gen: gp!(base: 0.5, amp: 34.0, top: DIRT, sub: DIRT, deep: STONE, top_depth: 6, caves: 0.34, ores: &[(COAL, 8), (GOLD, 4), (WATER, 6)], sea: Some((ACID, 0.535)), cave_pool: Some(ACID), islands: false, craters: false, trees: 4, cave_gas: true),
        enemies: &["valkyrie", "draugr", "fenrir_pup"], enemy_count: 14, boss: Some("jormungandr"), music: "kalevala", orbit: (0.54, 200.0), after: &["mani"], tier: 3, seed: 44, gate: false, sauna: false,
        desc: "Freya's jungle gardens drowned in acid swamps. Methane pockets in the caves." },
    PlanetDef { id: "jotunheim", name: "Jotunheim", system: 0, analog: "Mars", gravity: 0.70, sky: [(120, 50, 30), (230, 150, 100)], hazard: Hazard::Sleet,
        gen: gp!(base: 0.52, amp: 48.0, top: BRICK, sub: STONE, deep: STONE, top_depth: 3, caves: 0.42, ores: &[(ICE, 20), (METAL, 8), (COAL, 6)], sea: None, cave_pool: Some(WATER), islands: false, craters: true, trees: 3, cave_gas: false),
        enemies: &["frostjotunn", "berserker", "draugr"], enemy_count: 16, boss: Some("ymir"), music: "norse", orbit: (0.66, 250.0), after: &["muspell"], tier: 4, seed: 55, gate: false, sauna: false,
        desc: "Red rock and frozen caverns. The giants are home, and they are not fond of visitors." },
    PlanetDef { id: "asgard", name: "Asgard", system: 0, analog: "Jupiter", gravity: 0.85, sky: [(110, 160, 235), (255, 235, 200)], hazard: Hazard::None,
        gen: gp!(base: 0.88, amp: 8.0, top: DIRT, sub: STONE, deep: STONE, top_depth: 4, caves: 0.0, ores: &[(GOLD, 14), (METAL, 8)], sea: None, cave_pool: None, islands: true, craters: false, trees: 1, cave_gas: false),
        enemies: &["valkyrie", "draugr", "berserker"], enemy_count: 16, boss: None, music: "norse", orbit: (0.78, 20.0), after: &["vanaheim", "jotunheim"], tier: 4, seed: 66, gate: true, sauna: false,
        desc: "Floating sky-fortress islands. The Bifrost gate to the Kalevala star stands here." },
    PlanetDef { id: "niflheim", name: "Niflheim", system: 0, analog: "Saturn", gravity: 1.10, sky: [(150, 175, 205), (235, 245, 252)], hazard: Hazard::Snow,
        gen: gp!(base: 0.5, amp: 50.0, top: SNOW, sub: ICE, deep: STONE, top_depth: 8, caves: 0.36, ores: &[(ICE, 24), (METAL, 6), (GOLD, 2)], sea: Some((WATER, 0.54)), cave_pool: Some(WATER), islands: false, craters: false, trees: 3, cave_gas: false),
        enemies: &["frostjotunn", "draugr", "fenrir_pup"], enemy_count: 16, boss: None, music: "dark", orbit: (0.88, 300.0), after: &["jotunheim"], tier: 4, seed: 77, gate: false, sauna: false,
        desc: "Eternal snow, frozen seas and howling wind. Heat is a weapon here." },
    PlanetDef { id: "svartalfheim", name: "Svartalfheim", system: 0, analog: "Uranus", gravity: 1.0, sky: [(10, 6, 24), (40, 20, 70)], hazard: Hazard::None,
        gen: gp!(base: 0.32, amp: 26.0, top: STONE, sub: STONE, deep: STONE, top_depth: 4, caves: 0.55, ores: &[(RUNE, 16), (METAL, 14), (GOLD, 8), (GUNPOWDER, 4)], sea: None, cave_pool: Some(LAVA), islands: false, craters: false, trees: 0, cave_gas: true),
        enemies: &["svartalf", "draugr", "berserker"], enemy_count: 18, boss: None, music: "dark", orbit: (0.96, 350.0), after: &["niflheim"], tier: 5, seed: 88, gate: false, sauna: false,
        desc: "Dark-elf caverns lit by glowing runes. Dynamite country." },
    PlanetDef { id: "helheim", name: "Helheim", system: 0, analog: "Neptune", gravity: 1.05, sky: [(8, 10, 28), (40, 50, 100)], hazard: Hazard::None,
        gen: gp!(base: 0.52, amp: 24.0, top: ASH, sub: STONE, deep: STONE, top_depth: 5, caves: 0.30, ores: &[(COAL, 14), (METAL, 6)], sea: Some((OIL, 0.545)), cave_pool: Some(OIL), islands: false, craters: false, trees: 3, cave_gas: true),
        enemies: &["draugr", "svartalf", "berserker"], enemy_count: 20, boss: Some("hel"), music: "boss", orbit: (1.0, 60.0), after: &["svartalfheim"], tier: 5, seed: 99, gate: false, sauna: false,
        desc: "Hel's court beyond a black, flammable ocean. Do not light a match." },
    // ---------------- Paivatar system
    PlanetDef { id: "kalevala", name: "Kalevala", system: 1, analog: "Homeland", gravity: 1.0, sky: [(90, 150, 220), (210, 235, 250)], hazard: Hazard::None,
        gen: gp!(base: 0.5, amp: 34.0, top: DIRT, sub: DIRT, deep: STONE, top_depth: 5, caves: 0.30, ores: &[(COAL, 10), (METAL, 6), (GOLD, 3)], sea: Some((WATER, 0.54)), cave_pool: Some(WATER), islands: false, craters: false, trees: 2, cave_gas: false),
        enemies: &["hiisi", "fenrir_pup", "louhi_guard"], enemy_count: 14, boss: None, music: "kalevala", orbit: (0.40, 30.0), after: &[], tier: 3, seed: 111, gate: true, sauna: true,
        desc: "Birch and pine, a thousand lakes, saunas. Vainamoinen's singing hill." },
    PlanetDef { id: "pohjola", name: "Pohjola", system: 1, analog: "The North", gravity: 1.0, sky: [(40, 60, 100), (190, 215, 240)], hazard: Hazard::Snow,
        gen: gp!(base: 0.50, amp: 55.0, top: SNOW, sub: STONE, deep: STONE, top_depth: 6, caves: 0.36, ores: &[(ICE, 16), (METAL, 10), (GOLD, 4)], sea: None, cave_pool: Some(WATER), islands: false, craters: false, trees: 2, cave_gas: false),
        enemies: &["louhi_guard", "frostjotunn", "hiisi"], enemy_count: 18, boss: Some("louhi"), music: "kalevala", orbit: (0.60, 130.0), after: &["kalevala"], tier: 4, seed: 122, gate: false, sauna: false,
        desc: "Louhi's frozen fortress. She stole the Sampo; she will not give it back politely." },
    PlanetDef { id: "tuonela", name: "Tuonela", system: 1, analog: "Land of the Dead", gravity: 0.90, sky: [(6, 12, 14), (40, 64, 60)], hazard: Hazard::None,
        gen: gp!(base: 0.52, amp: 28.0, top: ASH, sub: DIRT, deep: STONE, top_depth: 5, caves: 0.34, ores: &[(COAL, 16), (METAL, 4)], sea: Some((WATER, 0.545)), cave_pool: Some(OIL), islands: false, craters: false, trees: 3, cave_gas: true),
        enemies: &["kalma", "draugr", "hiisi"], enemy_count: 18, boss: Some("tuoni"), music: "dark", orbit: (0.80, 220.0), after: &["kalevala"], tier: 4, seed: 133, gate: false, sauna: false,
        desc: "The black river, the swan of Tuonela, quiet ghosts. Methane drifts over the marsh." },
    PlanetDef { id: "ukkola", name: "Ukkola", system: 1, analog: "Ukko's Sky", gravity: 0.80, sky: [(30, 36, 70), (130, 140, 190)], hazard: Hazard::Lightning,
        gen: gp!(base: 0.88, amp: 8.0, top: STONE, sub: STONE, deep: STONE, top_depth: 4, caves: 0.0, ores: &[(METAL, 12), (GOLD, 6), (WATER, 8)], sea: None, cave_pool: None, islands: true, craters: false, trees: 0, cave_gas: false),
        enemies: &["valkyrie", "kalma", "louhi_guard"], enemy_count: 16, boss: Some("ukko_eagle"), music: "kalevala", orbit: (0.90, 300.0), after: &["pohjola"], tier: 5, seed: 144, gate: false, sauna: false,
        desc: "Storm islands and Ukko's thunder. Lightning strikes metal first." },
    PlanetDef { id: "sampola", name: "Sampola", system: 1, analog: "The Forge", gravity: 1.0, sky: [(50, 20, 12), (200, 100, 50)], hazard: Hazard::None,
        gen: gp!(base: 0.5, amp: 32.0, top: STONE, sub: STONE, deep: METAL, top_depth: 5, caves: 0.40, ores: &[(GOLD, 24), (METAL, 20), (GUNPOWDER, 10), (COAL, 14)], sea: Some((LAVA, 0.535)), cave_pool: Some(LAVA), islands: false, craters: false, trees: 0, cave_gas: false),
        enemies: &["hiisi", "eldjotunn", "louhi_guard"], enemy_count: 20, boss: Some("rauta_golem"), music: "boss", orbit: (1.0, 90.0), after: &["tuonela", "ukkola"], tier: 5, seed: 155, gate: false, sauna: false,
        desc: "Ilmarinen's forge world, where the Sampo was made - and must be made again." },
];

pub fn by_id(id: &str) -> Option<usize> { PLANETS.iter().position(|p| p.id == id) }
pub fn system_name(s: u8) -> &'static str { if s == 0 { "Sol (Norse)" } else { "Paivatar (Kalevala)" } }
pub fn star_id(s: u8) -> &'static str { if s == 0 { "sol" } else { "paivatar" } }
