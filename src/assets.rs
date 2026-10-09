//! Runtime asset loader. Everything is looked up by relative path without extension, e.g. "sprites/player1".
//! Missing files never crash the game: `tex()` returns None and callers draw fallback shapes; sounds are silent.
use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Assets {
    pub dir: PathBuf,
    pub textures: HashMap<String, Texture2D>,
    pub sounds: HashMap<String, Sound>,
    pub colors: [(u8, u8, u8); crate::elements::NUM_ELEMENTS],
    pub sound_enabled: bool,
    pub arenas: Vec<ArenaInfo>,
}

/// Metadata of an imported Soldat map (assets/maps/<stem>.txt).
#[derive(Clone, Debug, Default)]
pub struct ArenaInfo { pub stem: String, pub name: String, pub title: String, pub sky: [(u8, u8, u8); 2], pub spawns: Vec<(i32, i32, i32)>, pub size: (i32, i32), pub medikits: u32, pub grenades: u32 }

fn parse_arena(stem: &str, text: &str) -> ArenaInfo {
    let mut a = ArenaInfo { stem: stem.to_string(), name: stem.to_string(), title: String::new(), sky: [(60, 90, 150), (170, 200, 230)], ..Default::default() };
    for line in text.lines() {
        let Some((k, v)) = line.split_once('=') else { continue };
        let nums = |s: &str| s.split(',').filter_map(|x| x.trim().parse::<i32>().ok()).collect::<Vec<_>>();
        match k { "name" => a.title = v.trim().to_string(), "sky" => { let n = nums(v); if n.len() >= 6 { a.sky = [(n[0] as u8, n[1] as u8, n[2] as u8), (n[3] as u8, n[4] as u8, n[5] as u8)]; } } "size" => { let n = nums(v); if n.len() == 2 { a.size = (n[0], n[1]); } } "medikits" => a.medikits = v.trim().parse().unwrap_or(0), "grenades" => a.grenades = v.trim().parse().unwrap_or(0),
            "spawns" => { for part in v.split(';') { let n = nums(part); if n.len() == 3 { a.spawns.push((n[0], n[1], n[2])); } } } _ => {} }
    }
    a
}

pub fn find_dir() -> PathBuf {
    if let Ok(p) = std::env::var("NORDSOLDPOW_ASSETS") { return PathBuf::from(p); }
    let mut cands = vec![PathBuf::from("assets")];
    if let Ok(exe) = std::env::current_exe() {
        if let Some(d) = exe.parent() {
            cands.push(d.join("assets"));
            cands.push(d.join("../Resources/assets"));
            cands.push(d.join("../../assets"));
            cands.push(d.join("../../../assets"));
        }
    }
    cands.into_iter().find(|p| p.is_dir()).unwrap_or_else(|| PathBuf::from("assets"))
}

fn walk(dir: &Path, base: &Path, out: &mut Vec<(String, PathBuf)>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().collect();
    entries.sort_by_key(|e| e.path());
    for e in entries {
        let p = e.path();
        if p.is_dir() { walk(&p, base, out); continue; }
        let rel = p.strip_prefix(base).unwrap_or(&p).with_extension("");
        out.push((rel.to_string_lossy().replace('\\', "/"), p));
    }
}

impl Assets {
    pub async fn load(sound_enabled: bool) -> Assets {
        let dir = find_dir();
        let mut a = Assets { dir: dir.clone(), textures: HashMap::new(), sounds: HashMap::new(), colors: [(0, 0, 0); crate::elements::NUM_ELEMENTS], sound_enabled, arenas: Vec::new() };
        for (i, d) in crate::elements::ELEMENTS.iter().enumerate() { a.colors[i] = d.color; }
        if let Ok(t) = std::fs::read_to_string(dir.join("data/elements.txt")) { crate::elements::load_overrides(&t, &mut a.colors); }
        let mut files = Vec::new(); walk(&dir, &dir, &mut files);
        let total = files.len().max(1);
        for (n, (key, path)) in files.iter().enumerate() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
            let ps = path.to_string_lossy().to_string();
            if key.starts_with("maps/") {   // arena maps are decoded on demand (they are element maps, not images to draw)
                if ext == "txt" { if let Ok(t) = std::fs::read_to_string(path) { let stem = key.trim_start_matches("maps/"); a.arenas.push(parse_arena(stem, &t)); } }
                else if ext == "png" && key.ends_with("_thumb") { if let Ok(t) = load_texture(&ps).await { t.set_filter(FilterMode::Nearest); a.textures.insert(key.clone(), t); } }
                continue;
            }
            match ext.as_str() {
                "png" => { if let Ok(t) = load_texture(&ps).await { t.set_filter(FilterMode::Nearest); a.textures.insert(key.clone(), t); } }
                "wav" | "ogg" if sound_enabled => {
                    // .ogg wins over .wav if both exist (files are walked sorted; later insert would override, so only insert if absent or ogg)
                    if ext == "ogg" || !a.sounds.contains_key(key) { if let Ok(s) = load_sound(&ps).await { a.sounds.insert(key.clone(), s); } }
                }
                _ => {}
            }
            if n % 12 == 0 {
                clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
                draw_text("nordSoldPow - loading assets...", 40.0, screen_height() / 2.0, 32.0, WHITE);
                draw_rectangle(40.0, screen_height() / 2.0 + 20.0, 400.0 * n as f32 / total as f32, 10.0, ORANGE);
                next_frame().await;
            }
        }
        a.arenas.sort_by(|x, y| x.name.to_lowercase().cmp(&y.name.to_lowercase()));
        a
    }
    pub fn tex(&self, key: &str) -> Option<&Texture2D> { self.textures.get(key) }
}
