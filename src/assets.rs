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
        let mut a = Assets { dir: dir.clone(), textures: HashMap::new(), sounds: HashMap::new(), colors: [(0, 0, 0); crate::elements::NUM_ELEMENTS], sound_enabled };
        for (i, d) in crate::elements::ELEMENTS.iter().enumerate() { a.colors[i] = d.color; }
        if let Ok(t) = std::fs::read_to_string(dir.join("data/elements.txt")) { crate::elements::load_overrides(&t, &mut a.colors); }
        let mut files = Vec::new(); walk(&dir, &dir, &mut files);
        let total = files.len().max(1);
        for (n, (key, path)) in files.iter().enumerate() {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
            let ps = path.to_string_lossy().to_string();
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
        a
    }
    pub fn tex(&self, key: &str) -> Option<&Texture2D> { self.textures.get(key) }
}
