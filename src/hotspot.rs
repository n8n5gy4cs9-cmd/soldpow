//! Interactive map points.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HotKind { Spring, Rune, Chest, Forge, Gate, Sauna, Bunker }

impl HotKind {
    pub fn key(self) -> &'static str { match self { HotKind::Spring => "spring", HotKind::Rune => "rune", HotKind::Chest => "chest", HotKind::Forge => "forge", HotKind::Gate => "gate", HotKind::Sauna => "sauna", HotKind::Bunker => "bunker" } }
    pub fn label(self) -> &'static str {
        match self { HotKind::Spring => "Aino's Well (heals)", HotKind::Rune => "Rune Stone", HotKind::Chest => "Supply Chest", HotKind::Forge => "Ilmarinen's Forge (restock)", HotKind::Gate => "Bifrost / Aurora Gate", HotKind::Sauna => "Sauna (full heal)", HotKind::Bunker => "Hidden Bunker Cache" }
    }
    pub fn from_key(k: &str) -> Option<HotKind> {
        [HotKind::Spring, HotKind::Rune, HotKind::Chest, HotKind::Forge, HotKind::Gate, HotKind::Sauna, HotKind::Bunker].into_iter().find(|h| h.key() == k)
    }
}

#[derive(Clone, Debug)]
pub struct Hotspot { pub kind: HotKind, pub x: f32, pub y: f32, pub used: bool }
