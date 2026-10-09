//! Element table for the powder simulation. One row per element; ids are stable (saved worlds rely on them).

pub const EMPTY: u8 = 0;
pub const SAND: u8 = 1;
pub const DIRT: u8 = 2;
pub const SALT: u8 = 3;
pub const SNOW: u8 = 4;
pub const ASH: u8 = 5;
pub const GUNPOWDER: u8 = 6;
pub const COAL: u8 = 7;
pub const WATER: u8 = 8;
pub const OIL: u8 = 9;
pub const LAVA: u8 = 10;
pub const ACID: u8 = 11;
pub const NITRO: u8 = 12;
pub const BLOOD: u8 = 13;
pub const MERCURY: u8 = 14;
pub const SMOKE: u8 = 15;
pub const STEAM: u8 = 16;
pub const METHANE: u8 = 17;
pub const FIRE: u8 = 18;
pub const WALL: u8 = 19;
pub const STONE: u8 = 20;
pub const WOOD: u8 = 21;
pub const METAL: u8 = 22;
pub const GLASS: u8 = 23;
pub const ICE: u8 = 24;
pub const PLANT: u8 = 25;
pub const TNT: u8 = 26;
pub const GOLD: u8 = 27;
pub const RUNE: u8 = 28;
pub const BRICK: u8 = 29;
pub const CLONE: u8 = 30;
pub const VOID: u8 = 31;
pub const THUNDER: u8 = 32;
pub const DUST: u8 = 33;
pub const BCOAL: u8 = 34;
pub const SPONGE: u8 = 35;
pub const SALTWATER: u8 = 36;
pub const CO2: u8 = 37;
pub const LN2: u8 = 38;
pub const WAX: u8 = 39;
pub const MWAX: u8 = 40;
pub const C4: u8 = 41;
pub const FIREWORKS: u8 = 42;
pub const BOMB: u8 = 43;
pub const FUSE: u8 = 44;
pub const FUSEPOWDER: u8 = 45;
pub const BGLASS: u8 = 46;
pub const HYDROGEN: u8 = 47;
pub const OXYGEN: u8 = 48;
pub const LIFE: u8 = 49;
pub const WIRE: u8 = 50;
pub const SPARK: u8 = 51;
pub const BATTERY: u8 = 52;
pub const DIAMOND: u8 = 53;
pub const SEED: u8 = 54;
pub const CAUSTIC: u8 = 55;
pub const EMBER: u8 = 56;
pub const PACKED: u8 = 57;
pub const TURF: u8 = 58;
pub const SANDSTONE: u8 = 59;
pub const NUM_ELEMENTS: usize = 60;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cat { Empty, Powder, Liquid, Gas, Solid, Special }

pub struct ElDef {
    pub key: &'static str,
    pub name: &'static str,
    pub cat: Cat,
    pub color: (u8, u8, u8),
    pub alpha: u8,
    /// Sinks through lighter fluids; gases are negative.
    pub density: i16,
    /// 0 = bullets/explosions erase it freely, 255 = indestructible.
    pub hardness: u8,
}

macro_rules! e { ($k:expr,$n:expr,$c:ident,($r:expr,$g:expr,$b:expr),$a:expr,$d:expr,$h:expr) => {
    ElDef { key: $k, name: $n, cat: Cat::$c, color: ($r, $g, $b), alpha: $a, density: $d, hardness: $h } }; }

pub static ELEMENTS: [ElDef; NUM_ELEMENTS] = [
    e!("empty", "Empty", Empty, (0, 0, 0), 0, 0, 0),
    e!("sand", "Sand", Powder, (222, 196, 120), 255, 15, 0),
    e!("dirt", "Dirt", Powder, (112, 78, 50), 255, 14, 0),
    e!("salt", "Salt", Powder, (238, 238, 240), 255, 13, 0),
    e!("snow", "Snow", Powder, (240, 248, 255), 255, 6, 0),
    e!("ash", "Ash", Powder, (112, 112, 118), 255, 5, 0),
    e!("gunpowder", "Gunpowder", Powder, (74, 74, 64), 255, 12, 0),
    e!("coal", "Coal", Powder, (38, 38, 44), 255, 14, 10),
    e!("water", "Water", Liquid, (40, 100, 220), 235, 10, 0),
    e!("oil", "Oil", Liquid, (92, 62, 30), 245, 8, 0),
    e!("lava", "Lava", Liquid, (255, 92, 20), 255, 20, 0),
    e!("acid", "Acid", Liquid, (120, 235, 40), 235, 11, 0),
    e!("nitro", "Nitroglycerin", Liquid, (40, 130, 100), 240, 12, 0),
    e!("blood", "Blood", Liquid, (170, 10, 22), 245, 10, 0),
    e!("mercury", "Mercury", Liquid, (190, 192, 204), 255, 30, 0),
    e!("smoke", "Smoke", Gas, (64, 64, 70), 150, -2, 0),
    e!("steam", "Steam", Gas, (205, 215, 230), 140, -3, 0),
    e!("methane", "Methane", Gas, (130, 150, 90), 120, -1, 0),
    e!("fire", "Fire", Gas, (255, 160, 30), 230, -4, 0),
    e!("wall", "Wall", Solid, (84, 84, 104), 255, 100, 255),
    e!("stone", "Stone", Solid, (122, 122, 130), 255, 100, 60),
    e!("wood", "Wood", Solid, (124, 82, 42), 255, 100, 20),
    e!("metal", "Metal", Solid, (150, 156, 168), 255, 100, 200),
    e!("glass", "Glass", Solid, (172, 222, 238), 220, 100, 40),
    e!("ice", "Ice", Solid, (170, 215, 255), 235, 100, 30),
    e!("plant", "Plant", Solid, (40, 152, 52), 255, 100, 5),
    e!("tnt", "TNT", Solid, (205, 42, 42), 255, 100, 10),
    e!("gold", "Gold", Solid, (242, 202, 60), 255, 100, 180),
    e!("rune", "Rune stone", Solid, (80, 222, 255), 255, 100, 255),
    e!("brick", "Brick", Solid, (152, 72, 60), 255, 100, 100),
    e!("clone", "Clone", Special, (204, 164, 40), 255, 100, 150),
    e!("void", "Void", Special, (24, 8, 36), 255, 100, 150),
    e!("thunder", "Thunder", Special, (255, 255, 130), 255, -5, 0),
    e!("dust", "Dust", Powder, (255, 224, 160), 255, 4, 0),
    e!("bcoal", "Broken coal", Powder, (60, 60, 60), 255, 14, 5),
    e!("sponge", "Sponge", Solid, (255, 190, 48), 255, 100, 20),
    e!("saltwater", "Salt water", Liquid, (64, 80, 240), 235, 11, 0),
    e!("co2", "Carbon dioxide", Gas, (150, 150, 150), 120, 3, 0),
    e!("ln2", "Liquid nitrogen", Liquid, (128, 160, 223), 240, 9, 0),
    e!("wax", "Wax", Solid, (240, 240, 187), 255, 100, 10),
    e!("moltenwax", "Molten wax", Liquid, (224, 224, 170), 250, 7, 0),
    e!("c4", "C-4", Solid, (208, 128, 224), 255, 100, 10),
    e!("fireworks", "Fireworks", Powder, (255, 160, 64), 255, 12, 0),
    e!("bomb", "Bomb", Powder, (255, 190, 40), 255, 13, 0),
    e!("fuse", "Fuse", Solid, (10, 100, 6), 255, 100, 19),
    e!("fusepowder", "Fuse powder", Powder, (99, 173, 95), 255, 10, 0),
    e!("brokenglass", "Broken glass", Powder, (150, 170, 180), 255, 15, 0),
    e!("hydrogen", "Hydrogen", Gas, (80, 112, 255), 120, -6, 0),
    e!("oxygen", "Oxygen", Gas, (128, 160, 255), 110, -2, 0),
    e!("life", "Game of Life", Special, (12, 172, 0), 255, 100, 0),
    e!("wire", "Wire", Solid, (200, 110, 60), 255, 100, 120),
    e!("spark", "Spark", Special, (255, 255, 128), 255, 100, 0),
    e!("battery", "Battery", Solid, (140, 140, 20), 255, 100, 100),
    e!("diamond", "Diamond", Solid, (190, 240, 250), 255, 100, 255),
    e!("seed", "Seed", Powder, (136, 231, 136), 255, 9, 0),
    e!("caustic", "Caustic gas", Gas, (128, 255, 160), 140, -2, 0),
    e!("ember", "Ember", Powder, (255, 242, 136), 255, 3, 0),
    e!("packed", "Packed earth", Solid, (116, 84, 56), 255, 100, 30),
    e!("turf", "Turf", Solid, (70, 150, 70), 255, 100, 10),
    e!("sandstone", "Sandstone", Solid, (214, 186, 120), 255, 100, 40),
];

#[inline] pub fn def(el: u8) -> &'static ElDef { &ELEMENTS[(el as usize).min(NUM_ELEMENTS - 1)] }
#[inline] pub fn cat(el: u8) -> Cat { def(el).cat }
#[inline] pub fn density(el: u8) -> i16 { def(el).density }
pub fn by_key(key: &str) -> Option<u8> {
    let k = key.to_ascii_lowercase();
    ELEMENTS.iter().position(|d| d.key == k || d.name.to_ascii_lowercase() == k).map(|i| i as u8)
}
/// Obstacle for creatures: solids and powders.
#[inline] pub fn is_solid_for_body(el: u8) -> bool { matches!(cat(el), Cat::Solid | Cat::Powder) || el == CLONE || el == VOID }
#[inline] pub fn is_fluid(el: u8) -> bool { matches!(cat(el), Cat::Liquid | Cat::Gas) }
pub fn is_flammable(el: u8) -> bool { matches!(el, WOOD | PLANT | OIL | COAL | GUNPOWDER | METHANE | NITRO | TNT | DUST | BCOAL | WAX | MWAX | C4 | FUSE | FUSEPOWDER | HYDROGEN | BOMB | FIREWORKS) }
pub fn is_explosive(el: u8) -> bool { matches!(el, GUNPOWDER | NITRO | METHANE | TNT | C4 | HYDROGEN | BOMB) }
pub fn is_conductor(el: u8) -> bool { matches!(el, WIRE | METAL | SALTWATER | BATTERY | GOLD) }

/// Picker categories for the UI.
pub const PICKER: &[(&str, &[u8])] = &[
    ("Powders", &[SAND, DIRT, SALT, SNOW, ASH, GUNPOWDER, COAL, DUST, BCOAL, FUSEPOWDER, BGLASS, SEED, FIREWORKS, BOMB, EMBER]),
    ("Liquids", &[WATER, OIL, LAVA, ACID, NITRO, BLOOD, MERCURY, SALTWATER, LN2, MWAX]),
    ("Gases", &[SMOKE, STEAM, METHANE, FIRE, CO2, HYDROGEN, OXYGEN, CAUSTIC]),
    ("Solids", &[WALL, STONE, WOOD, METAL, GLASS, ICE, PLANT, TNT, GOLD, RUNE, BRICK, WAX, C4, FUSE, SPONGE, DIAMOND, WIRE, BATTERY, PACKED, TURF, SANDSTONE]),
    ("Special", &[CLONE, VOID, THUNDER, LIFE, SPARK]),
];

/// Optional colour overrides: assets/data/elements.txt, lines `key r g b`.
pub fn load_overrides(text: &str, colors: &mut [(u8, u8, u8); NUM_ELEMENTS]) {
    for line in text.lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        if p.len() == 4 && !p[0].starts_with('#') {
            if let (Some(id), Ok(r), Ok(g), Ok(b)) = (by_key(p[0]), p[1].parse(), p[2].parse(), p[3].parse()) { colors[id as usize] = (r, g, b); }
        }
    }
}
