//! SFX + music playback on top of macroquad::audio. All calls are no-ops when sound is disabled / assets are missing.
use crate::assets::Assets;
use macroquad::audio::{play_sound, set_sound_volume, stop_sound, PlaySoundParams};
use std::collections::HashMap;

pub struct Audio {
    pub sfx_vol: f32,
    pub music_vol: f32,
    pub music_on: bool,
    pub current: Option<String>,
    last: HashMap<String, u64>,
    pub tick: u64,
}

impl Audio {
    pub fn new(sfx_vol: f32, music_vol: f32) -> Self { Audio { sfx_vol, music_vol, music_on: true, current: None, last: HashMap::new(), tick: 0 } }
    /// Play a one-shot effect. `gain` 0..1 (distance falloff done by the caller).
    pub fn sfx(&mut self, a: &Assets, name: &str, gain: f32) {
        if !a.sound_enabled || gain <= 0.01 { return; }
        let key = format!("sfx/{}", name);
        if let Some(&t) = self.last.get(&key) { if self.tick < t + 2 { return; } }
        if let Some(s) = a.sounds.get(&key) {
            self.last.insert(key, self.tick);
            play_sound(s, PlaySoundParams { looped: false, volume: (self.sfx_vol * gain).clamp(0.0, 1.0) });
        }
    }
    pub fn music(&mut self, a: &Assets, track: &str) {
        if !a.sound_enabled { return; }
        if self.current.as_deref() == Some(track) { return; }
        if let Some(cur) = self.current.take() { if let Some(s) = a.sounds.get(&format!("music/{}", cur)) { stop_sound(s); } }
        if !self.music_on { self.current = None; return; }
        if let Some(s) = a.sounds.get(&format!("music/{}", track)) {
            play_sound(s, PlaySoundParams { looped: true, volume: self.music_vol });
            self.current = Some(track.to_string());
        }
    }
    pub fn stop_music(&mut self, a: &Assets) {
        if let Some(cur) = self.current.take() { if let Some(s) = a.sounds.get(&format!("music/{}", cur)) { stop_sound(s); } }
    }
    pub fn apply_volume(&self, a: &Assets) {
        if let Some(cur) = &self.current { if let Some(s) = a.sounds.get(&format!("music/{}", cur)) { set_sound_volume(s, self.music_vol); } }
    }
}
