//! Plain `key=value` save file (no dependencies). Location: $NORDSOLDPOW_SAVE, else ~/.nordsoldpow/save.txt, else ./save.txt
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Save {
    pub completed: BTreeSet<String>,
    pub unlocked: BTreeSet<String>,
    pub silver: u32,
    pub shards: u32,
    pub intro_seen: bool,
    pub gate_open: bool,
    pub ending_seen: bool,
    pub sfx_vol: f32,
    pub music_vol: f32,
    pub zoom: f32,
    pub last_planet: String,
    pub secrets: BTreeSet<String>,
}

impl Default for Save {
    fn default() -> Self {
        let mut unlocked = BTreeSet::new();
        for k in ["puukko", "hugin", "powder_gun"] { unlocked.insert(k.to_string()); }
        Save { completed: BTreeSet::new(), unlocked, silver: 0, shards: 0, intro_seen: false, gate_open: false, ending_seen: false, sfx_vol: 0.8, music_vol: 0.5, zoom: 3.0, last_planet: "midgard".into(), secrets: BTreeSet::new() }
    }
}

pub fn path() -> PathBuf {
    if let Ok(p) = std::env::var("NORDSOLDPOW_SAVE") { return PathBuf::from(p); }
    if let Ok(h) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) { return PathBuf::from(h).join(".nordsoldpow").join("save.txt"); }
    PathBuf::from("save.txt")
}

impl Save {
    pub fn load() -> Save {
        let mut s = Save::default();
        if let Ok(t) = std::fs::read_to_string(path()) { s.parse(&t); }
        s
    }
    pub fn parse(&mut self, t: &str) {
        for line in t.lines() {
            let Some((k, v)) = line.split_once('=') else { continue };
            let list = || v.split(',').filter(|x| !x.is_empty()).map(|x| x.to_string()).collect::<BTreeSet<_>>();
            match k.trim() {
                "completed" => self.completed = list(),
                "unlocked" => { let mut l = list(); for d in ["puukko", "hugin", "powder_gun"] { l.insert(d.to_string()); } self.unlocked = l; }
                "secrets" => self.secrets = list(),
                "silver" => self.silver = v.trim().parse().unwrap_or(0),
                "shards" => self.shards = v.trim().parse().unwrap_or(0),
                "intro_seen" => self.intro_seen = v.trim() == "1",
                "gate_open" => self.gate_open = v.trim() == "1",
                "ending_seen" => self.ending_seen = v.trim() == "1",
                "sfx_vol" => self.sfx_vol = v.trim().parse().unwrap_or(0.8),
                "music_vol" => self.music_vol = v.trim().parse().unwrap_or(0.5),
                "zoom" => self.zoom = v.trim().parse().unwrap_or(3.0),
                "last_planet" => self.last_planet = v.trim().to_string(),
                _ => {}
            }
        }
    }
    pub fn to_text(&self) -> String {
        let j = |s: &BTreeSet<String>| s.iter().cloned().collect::<Vec<_>>().join(",");
        format!("completed={}\nunlocked={}\nsecrets={}\nsilver={}\nshards={}\nintro_seen={}\ngate_open={}\nending_seen={}\nsfx_vol={}\nmusic_vol={}\nzoom={}\nlast_planet={}\n",
            j(&self.completed), j(&self.unlocked), j(&self.secrets), self.silver, self.shards, self.intro_seen as u8, self.gate_open as u8, self.ending_seen as u8, self.sfx_vol, self.music_vol, self.zoom, self.last_planet)
    }
    pub fn write(&self) {
        let p = path();
        if let Some(d) = p.parent() { let _ = std::fs::create_dir_all(d); }
        let _ = std::fs::write(p, self.to_text());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn roundtrip() {
        let mut s = Save::default(); s.silver = 42; s.completed.insert("midgard_m1".into()); s.unlocked.insert("munin".into()); s.gate_open = true;
        let mut t = Save::default(); t.parse(&s.to_text());
        assert_eq!(t.silver, 42); assert!(t.completed.contains("midgard_m1")); assert!(t.unlocked.contains("munin")); assert!(t.gate_open);
    }
}
