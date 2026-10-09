//! Mission definitions (5 per world: 3 main chain + 2 optional) and runtime progress.
use crate::elements::*;
use crate::hotspot::HotKind;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Obj {
    Kill(u32),
    KillKind(&'static str, u32),
    Collect(u32),
    Reach(&'static str),
    /// element, zone, count
    Fill(u8, &'static str, u32),
    Burn(&'static str),
    Survive(u32),
    Boss,
    Interact(HotKind, u32),
}

pub struct MissionDef {
    pub id: &'static str,
    pub planet: &'static str,
    pub title: &'static str,
    pub who: &'static str,
    pub brief: &'static str,
    pub done: &'static str,
    pub obj: Obj,
    pub silver: u32,
    pub weapon: Option<&'static str>,
    pub optional: bool,
}

macro_rules! m { ($id:expr,$pl:expr,$t:expr,$who:expr,$b:expr,$d:expr,$o:expr,$s:expr,$w:expr,$opt:expr) => {
    MissionDef { id: $id, planet: $pl, title: $t, who: $who, brief: $b, done: $d, obj: $o, silver: $s, weapon: $w, optional: $opt } } }

use HotKind::*;
pub static MISSIONS: &[MissionDef] = &[
    // ---------------- MIDGARD
    m!("midgard_m1", "midgard", "First Steps", "Odin", "Welcome home, Rune-Runner. Run east to the rune beacon. Jetpack: hold W. The Draugr are rude today.", "Good. The beacon hums. Take this Munin revolver.", Obj::Reach("goal"), 120, Some("munin"), false),
    m!("midgard_m2", "midgard", "Fill the Well", "Odin", "Ilmarinen's cistern is dry and the Sampo shard sulks. Press B, pick Water, and fill the stone basin (350 cells).", "The shard glows. Lemminkainen sent an SMG - don't ask.", Obj::Fill(WATER, "basin", 350), 200, Some("lemminkainen_smg"), false),
    m!("midgard_m3", "midgard", "Fenrir Unleashed", "Odin", "Fenrir slipped his chain and is waiting in the arena. Bring the thunder.", "Shard secured. Vainamoinen insists you take his gun.", Obj::Boss, 400, Some("vainamoinen_gun"), false),
    m!("midgard_o1", "midgard", "Rune Reader", "Heimdall", "OPTIONAL: Two rune stones stand on this world. Read both (press F).", "You read the sagas. Heimdall is mildly impressed.", Obj::Interact(Rune, 2), 100, None, true),
    m!("midgard_o2", "midgard", "Draugr Cull", "Odin", "OPTIONAL: Thin out the undead. Twelve will do.", "Tidy.", Obj::Kill(12), 150, None, true),
    // ---------------- MANI
    m!("mani_m1", "mani", "Moon Dust Shards", "Heimdall", "Three shard fragments tumbled across the regolith. Gravity is low - jump far!", "Collected. The moon feels heavier already.", Obj::Collect(3), 150, Some("kullervo_shotgun"), false),
    m!("mani_m2", "mani", "Fill the Crater", "Ilmarinen", "My old furnace-basin needs 300 cells of ash. Use the powder gun on ash.", "Fine work. A sniper for the long moonlight.", Obj::Fill(ASH, "basin", 300), 220, Some("gungnir_sniper"), false),
    m!("mani_m3", "mani", "Hold the Beacon", "Odin", "Stand at the beacon for 60 seconds. Draugr will come.", "The shard is safe. Here is a grenade launcher.", Obj::Survive(60), 400, Some("gjallar_grenade"), false),
    m!("mani_o1", "mani", "Dark Elf Hunt", "Odin", "OPTIONAL: Four Svartalf snipers lurk in the craters.", "Quiet again.", Obj::KillKind("svartalf", 4), 200, None, true),
    m!("mani_o2", "mani", "Moon Chests", "Freya", "OPTIONAL: Open both supply chests (press F).", "Freya approves of shopping.", Obj::Interact(Chest, 2), 100, None, true),
    // ---------------- MUSPELL
    m!("muspell_m1", "muspell", "Cross the Ember Sea", "Odin", "Embers rain from the sky. Cross to the rune beacon. Lava is not a floor.", "You made it. Take the flamethrower; Surtr will not mind.", Obj::Reach("goal"), 180, Some("surtr_flame"), false),
    m!("muspell_m2", "muspell", "Burn the Hive", "Odin", "Louhi's wooden hive blocks the pass. Burn it down (85% or more).", "Smoke on the horizon. Beautiful.", Obj::Burn("nest"), 260, None, false),
    m!("muspell_m3", "muspell", "Surtr", "Odin", "Surtr guards the shard in the arena. Fire will not hurt him. Try water.", "Surtr falls. Here is the Mjolnir RPG.", Obj::Boss, 600, Some("mjolnir_rpg"), false),
    m!("muspell_o1", "muspell", "Shard Fragments", "Ilmarinen", "OPTIONAL: Gather three fragments before they melt.", "Ilmarinen is smug.", Obj::Collect(3), 200, None, true),
    m!("muspell_o2", "muspell", "Giant Slayer", "Odin", "OPTIONAL: Defeat fifteen enemies.", "Counted.", Obj::Kill(15), 200, None, true),
    // ---------------- VANAHEIM
    m!("vanaheim_m1", "vanaheim", "Garden Fragments", "Freya", "Four shard fragments are in my gardens. Mind the acid; it eats boots.", "My roses thank you.", Obj::Collect(4), 200, None, false),
    m!("vanaheim_m2", "vanaheim", "Clear the Thicket", "Freya", "The thorn-hut hive must burn. Methane in the caves is, shall we say, enthusiastic.", "Lovely fire. Take Loki's Trick.", Obj::Burn("nest"), 280, Some("loki_trick"), false),
    m!("vanaheim_m3", "vanaheim", "Jormungandr Rises", "Freya", "The world serpent surfaces in the swamp arena. Acid? He has seen worse.", "The serpent sinks. Shard is yours.", Obj::Boss, 650, None, false),
    m!("vanaheim_o1", "vanaheim", "Drink the Spring", "Freya", "OPTIONAL: Drink from the spring (press F).", "Refreshing.", Obj::Interact(Spring, 1), 80, None, true),
    m!("vanaheim_o2", "vanaheim", "Valkyrie Trouble", "Freya", "OPTIONAL: Five Valkyries are on patrol.", "My sisters were rude anyway.", Obj::KillKind("valkyrie", 5), 220, None, true),
    // ---------------- JOTUNHEIM
    m!("jotunheim_m1", "jotunheim", "Giants' Road", "Odin", "Sleet, red rock, frost giants. Reach the beacon.", "On to the frost. Louhi's frost gun is yours.", Obj::Reach("goal"), 220, Some("louhi_frost"), false),
    m!("jotunheim_m2", "jotunheim", "Pack the Snow", "Ilmarinen", "My cooling basin needs 300 cells of snow. Freeze things, then dig.", "Chilled to perfection.", Obj::Fill(SNOW, "basin", 300), 300, None, false),
    m!("jotunheim_m3", "jotunheim", "Ymir's Heir", "Odin", "A frost giant guards the shard. Fire works, as do explosives.", "He shatters. A minigun for the road.", Obj::Boss, 700, Some("fenrir_minigun"), false),
    m!("jotunheim_o1", "jotunheim", "Dig for Treasure", "Heimdall", "OPTIONAL: A bunker lies buried near the middle. Dig and take its cache.", "Treasure.", Obj::Interact(Bunker, 1), 250, None, true),
    m!("jotunheim_o2", "jotunheim", "Frost Hunter", "Odin", "OPTIONAL: Four frost giants.", "Cold work.", Obj::KillKind("frostjotunn", 4), 250, None, true),
    // ---------------- ASGARD
    m!("asgard_m1", "asgard", "Sky Fragments", "Heimdall", "Four fragments float between the islands. Jetpack fuel is life.", "Gathered. The Bifrost stirs.", Obj::Collect(4), 250, None, false),
    m!("asgard_m2", "asgard", "Fill the Fountain", "Freya", "Idun's fountain on the floor needs 300 water.", "Apples for everyone.", Obj::Fill(WATER, "basin", 300), 320, None, false),
    m!("asgard_m3", "asgard", "Guard the Gate", "Heimdall", "Hold the beacon for 90 seconds while I tune the Bifrost.", "The Aurora Gate is open! Take Thor's lightning.", Obj::Survive(90), 700, Some("ukko_thunder"), false),
    m!("asgard_o1", "asgard", "Use the Bifrost", "Heimdall", "OPTIONAL: Walk up to the gate and press F.", "Heimdall nods.", Obj::Interact(Gate, 1), 100, None, true),
    m!("asgard_o2", "asgard", "Valkyrie Practice", "Freya", "OPTIONAL: Eight Valkyries.", "Practice complete.", Obj::KillKind("valkyrie", 8), 260, None, true),
    // ---------------- NIFLHEIM
    m!("niflheim_m1", "niflheim", "Whiteout", "Odin", "Cross the snow to the rune beacon. Bring fire.", "Frostbite avoided. Mostly.", Obj::Reach("goal"), 260, None, false),
    m!("niflheim_m2", "niflheim", "Thaw the Hive", "Odin", "A frozen hive of dead wood. Burn it.", "Warm at last.", Obj::Burn("nest"), 340, None, false),
    m!("niflheim_m3", "niflheim", "The Long Night", "Odin", "Survive 90 seconds at the beacon.", "Dawn comes. Here: the Sampo Gun.", Obj::Survive(90), 750, Some("sampo_gun"), false),
    m!("niflheim_o1", "niflheim", "Rune Reader II", "Heimdall", "OPTIONAL: Read both rune stones.", "Cold, ancient and true.", Obj::Interact(Rune, 2), 120, None, true),
    m!("niflheim_o2", "niflheim", "Cull the Cold", "Odin", "OPTIONAL: Eighteen enemies.", "Counted.", Obj::Kill(18), 280, None, true),
    // ---------------- SVARTALFHEIM
    m!("svartalfheim_m1", "svartalfheim", "Dark Shards", "Ilmarinen", "Five fragments glitter in the caves. Beware dynamite pockets.", "Sparkly.", Obj::Collect(5), 300, None, false),
    m!("svartalfheim_m2", "svartalfheim", "Stock the Magazine", "Ilmarinen", "The dwarves need 200 gunpowder in their basin. Do NOT smoke.", "Dynamite delivered. Not lit. Well done.", Obj::Fill(GUNPOWDER, "basin", 200), 380, None, false),
    m!("svartalfheim_m3", "svartalfheim", "Last Light", "Odin", "Hold the rune beacon for 75 seconds.", "The runes are quiet. Take the acid sprayer.", Obj::Survive(75), 800, Some("tuoni_acid"), false),
    m!("svartalfheim_o1", "svartalfheim", "Dwarven Cache", "Ilmarinen", "OPTIONAL: Find the buried bunker.", "Hoard acquired.", Obj::Interact(Bunker, 1), 300, None, true),
    m!("svartalfheim_o2", "svartalfheim", "Dark Elf Purge", "Odin", "OPTIONAL: Eight Svartalfs.", "Done.", Obj::KillKind("svartalf", 8), 300, None, true),
    // ---------------- HELHEIM
    m!("helheim_m1", "helheim", "Black Shore", "Odin", "Do not light a match. Reach the beacon.", "You smell of oil. Move on.", Obj::Reach("goal"), 300, None, false),
    m!("helheim_m2", "helheim", "Burn the Bone Hive", "Odin", "I said no matches. Except there: burn the hive.", "Oil, bones and smoke. Hel will be furious.", Obj::Burn("nest"), 400, None, false),
    m!("helheim_m3", "helheim", "Hel", "Odin", "Hel waits in her arena. Show her the modern world.", "Hel yields the shard. Here, a railgun.", Obj::Boss, 1000, Some("ilmarinen_rail"), false),
    m!("helheim_o1", "helheim", "Ghost Fragments", "Hel", "OPTIONAL: Four fragments, scattered by my guests.", "Hel grins.", Obj::Collect(4), 320, None, true),
    m!("helheim_o2", "helheim", "Grave Cleaning", "Odin", "OPTIONAL: Twenty enemies.", "Counted.", Obj::Kill(20), 320, None, true),
    // ---------------- KALEVALA
    m!("kalevala_m1", "kalevala", "Sauna First", "Vainamoinen", "Welcome, young one. Steam in the sauna first (press F inside), then we sing.", "Cleansed. Now the real work.", Obj::Interact(Sauna, 1), 150, None, false),
    m!("kalevala_m2", "kalevala", "Fill the Lake Basin", "Vainamoinen", "The singing basin wants 350 water. Pour it in.", "The basin hums a minor third.", Obj::Fill(WATER, "basin", 350), 330, None, false),
    m!("kalevala_m3", "kalevala", "Hold the Song", "Vainamoinen", "Hold the beacon for 60 seconds while I sing the shard free.", "The song is done. Pohjola awaits.", Obj::Survive(60), 700, None, false),
    m!("kalevala_o1", "kalevala", "Rune Songs", "Vainamoinen", "OPTIONAL: Read both rune stones.", "Old songs. Good songs.", Obj::Interact(Rune, 2), 150, None, true),
    m!("kalevala_o2", "kalevala", "Goblin Season", "Lemminkainen", "OPTIONAL: Eight hiisi. Reckless fun!", "Ha!", Obj::KillKind("hiisi", 8), 300, None, true),
    // ---------------- POHJOLA
    m!("pohjola_m1", "pohjola", "The Frozen North", "Vainamoinen", "Louhi's guards are everywhere. Reach the rune beacon.", "Louhi knows you are here.", Obj::Reach("goal"), 350, None, false),
    m!("pohjola_m2", "pohjola", "Burn the Fortress Stores", "Ilmarinen", "Louhi's timber stores must burn. 85% should do.", "That will slow her.", Obj::Burn("nest"), 420, None, false),
    m!("pohjola_m3", "pohjola", "Louhi", "Vainamoinen", "Louhi herself. She has an eagle form. Aim high.", "Louhi falls. The Sampo shard is ours.", Obj::Boss, 1100, None, false),
    m!("pohjola_o1", "pohjola", "Frozen Fragments", "Ilmarinen", "OPTIONAL: Four fragments in the ice.", "Fine.", Obj::Collect(4), 350, None, true),
    m!("pohjola_o2", "pohjola", "Guard Duty", "Vainamoinen", "OPTIONAL: Eighteen enemies.", "Counted.", Obj::Kill(18), 350, None, true),
    // ---------------- TUONELA
    m!("tuonela_m1", "tuonela", "The Black River", "Vainamoinen", "Four shard fragments lie along the river. Methane drifts; leave your lighter.", "Quiet river, quiet dead.", Obj::Collect(4), 350, None, false),
    m!("tuonela_m2", "tuonela", "Fill the Urn", "Vainamoinen", "Tuoni's urn needs 300 water to quiet the dead.", "The ghosts calm.", Obj::Fill(WATER, "basin", 300), 440, None, false),
    m!("tuonela_m3", "tuonela", "Tuoni", "Vainamoinen", "The lord of the dead guards his shard. Dodge the wisps.", "Tuoni bows. Take what you came for.", Obj::Boss, 1200, None, false),
    m!("tuonela_o1", "tuonela", "Rune Songs of the Dead", "Vainamoinen", "OPTIONAL: Both rune stones.", "Sad songs.", Obj::Interact(Rune, 2), 160, None, true),
    m!("tuonela_o2", "tuonela", "Ghost Hunt", "Lemminkainen", "OPTIONAL: Six Kalma ghosts.", "Boo!", Obj::KillKind("kalma", 6), 380, None, true),
    // ---------------- UKKOLA
    m!("ukkola_m1", "ukkola", "Storm Islands", "Vainamoinen", "Lightning strikes metal first. Reach the beacon.", "Ukko is watching.", Obj::Reach("goal"), 380, None, false),
    m!("ukkola_m2", "ukkola", "Sky Fragments", "Ilmarinen", "Four fragments float on the storm. Jetpack fuel!", "Lovely.", Obj::Collect(4), 450, None, false),
    m!("ukkola_m3", "ukkola", "Ukko's Eagle", "Vainamoinen", "Ukko's thunder-eagle guards the last sky shard.", "The eagle falls. Ukko applauds in thunder.", Obj::Boss, 1300, None, false),
    m!("ukkola_o1", "ukkola", "Sky Chests", "Ilmarinen", "OPTIONAL: Open both chests.", "Loot.", Obj::Interact(Chest, 2), 200, None, true),
    m!("ukkola_o2", "ukkola", "Storm Cull", "Vainamoinen", "OPTIONAL: Fourteen enemies.", "Counted.", Obj::Kill(14), 400, None, true),
    // ---------------- SAMPOLA (finale)
    m!("sampola_m1", "sampola", "The Forge Road", "Ilmarinen", "My forge world. Reach the beacon, mind the lava seas.", "Home sweet inferno.", Obj::Reach("goal"), 450, None, false),
    m!("sampola_m2", "sampola", "Pour the Mold", "Ilmarinen", "The Sampo mold needs 250 cells of lava. You know how.", "The mold glows. One more thing.", Obj::Fill(LAVA, "basin", 250), 600, None, false),
    m!("sampola_m3", "sampola", "The Rogue Golem", "Ilmarinen", "My forge-golem went rogue and guards the Sampo anvil. Smash him.", "The Sampo is whole again!", Obj::Boss, 2000, None, false),
    m!("sampola_o1", "sampola", "Anvil Fragments", "Ilmarinen", "OPTIONAL: Five last fragments.", "Polished.", Obj::Collect(5), 500, None, true),
    m!("sampola_o2", "sampola", "Forge Sweep", "Ilmarinen", "OPTIONAL: Twenty enemies.", "Counted.", Obj::Kill(20), 500, None, true),
];

pub fn for_planet(planet: &str) -> Vec<usize> { MISSIONS.iter().enumerate().filter(|(_, m)| m.planet == planet).map(|(i, _)| i).collect() }
pub fn by_id(id: &str) -> Option<usize> { MISSIONS.iter().position(|m| m.id == id) }

pub fn describe(o: &Obj) -> String {
    match o {
        Obj::Kill(n) => format!("Defeat {} enemies", n),
        Obj::KillKind(k, n) => format!("Defeat {} {}", n, k),
        Obj::Collect(n) => format!("Collect {} shard fragments", n),
        Obj::Reach(_) => "Reach the rune beacon (far east)".into(),
        Obj::Fill(e, _, n) => format!("Fill the stone basin with {} {}", n, def(*e).name),
        Obj::Burn(_) => "Burn the wooden hive (85%)".into(),
        Obj::Survive(s) => format!("Hold the beacon for {} s", s),
        Obj::Boss => "Defeat the boss in the arena".into(),
        Obj::Interact(k, n) => format!("Use {} x{}", k.label(), n),
    }
}

#[derive(Clone, Debug)]
pub struct MissionRun { pub def: usize, pub progress: u32, pub done: bool, pub timer: u32, pub spawned: bool }

pub fn target(o: &Obj) -> u32 {
    match o { Obj::Kill(n) | Obj::KillKind(_, n) | Obj::Collect(n) | Obj::Fill(_, _, n) | Obj::Interact(_, n) => *n, Obj::Survive(s) => *s, Obj::Reach(_) | Obj::Burn(_) | Obj::Boss => 1 }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn every_planet_has_five_missions() {
        for p in crate::planets::PLANETS.iter() {
            let v = for_planet(p.id);
            assert_eq!(v.len(), 5, "{}", p.id);
            assert_eq!(v.iter().filter(|&&i| !MISSIONS[i].optional).count(), 3);
            for &i in &v { if let Some(w) = MISSIONS[i].weapon { assert!(crate::weapons::index(w).is_some(), "weapon {}", w); } }
            if MISSIONS[v[2]].obj == Obj::Boss { assert!(p.boss.is_some(), "{} boss", p.id); }
            if p.boss.is_some() { assert_eq!(MISSIONS[v[2]].obj, Obj::Boss, "{} main3 should be boss", p.id); }
        }
    }
    #[test] fn ids_unique() { let mut s = std::collections::HashSet::new(); for m in MISSIONS { assert!(s.insert(m.id)); } }
}
