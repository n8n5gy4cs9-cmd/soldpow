//! Scene logic: title, settings, credits, controls, solar map, story pages, pause menu and the per-frame driver.
use crate::elements::*;
use crate::game::*;
use crate::missions::*;
use crate::planets::*;
use crate::ui::*;
use macroquad::prelude::*;

const TICK: f32 = 1.0 / 60.0;

impl Game {
    pub fn frame(&mut self) {
        self.screen_w = screen_width(); self.screen_h = screen_height();
        let dt = get_frame_time().min(0.1);
        if self.scene != self.prev_scene { self.on_scene_enter(); self.prev_scene = self.scene; }
        clear_background(Color::new(0.03, 0.03, 0.07, 1.0));
        match self.scene {
            Scene::Title => self.scene_title(),
            Scene::Settings => self.scene_settings(),
            Scene::Credits => self.scene_credits(),
            Scene::Controls => self.scene_controls(),
            Scene::Map => self.scene_map(),
            Scene::Story => self.scene_story(dt),
            Scene::Play => self.scene_play(dt),
        }
        self.update_ui_timers(dt);
    }

    fn on_scene_enter(&mut self) {
        let a = &self.assets;
        match self.scene { Scene::Title | Scene::Settings | Scene::Credits | Scene::Controls => self.audio.music(a, "menu"), Scene::Map => self.audio.music(a, "map"), Scene::Story => self.audio.music(a, "dark"),
            Scene::Play => { let m = if self.mode == Mode::Sandbox { "calm" } else { PLANETS[self.planet].music }; self.audio.music(a, m); } }
    }

    // ------------------------------------------------------------------ play
    fn scene_play(&mut self, dt: f32) {
        self.handle_hotkeys();
        self.update_console();
        let views = self.compute_views();
        self.gather_input(&views);
        if !self.ui.paused {
            self.build_tool(&views[0]);
            self.acc += dt; let mut n = 0;
            while self.acc >= TICK && n < 4 { self.tick(); self.acc -= TICK; n += 1; }
            if self.acc > TICK * 4.0 { self.acc = 0.0; }
            if let Some(c) = self.players.get(0) { let _ = c; }
        }
        if self.scene != Scene::Play { return; }
        self.draw_play();
        if self.ui.journal { self.draw_journal(); }
        self.draw_cheat_menu();
        if self.ui.paused { self.draw_pause(); }
        self.draw_console();
        if self.quit { std::process::exit(0); }
    }

    fn draw_pause(&mut self) {
        draw_rectangle(0.0, 0.0, self.screen_w, self.screen_h, Color::new(0.0, 0.0, 0.0, 0.6));
        let (cx, mut y) = (self.screen_w / 2.0, self.screen_h / 2.0 - 140.0);
        text_c("PAUSED", cx, y, 50.0, WHITE); y += 30.0;
        let (w, h) = (320.0, 40.0);
        if button(cx - w / 2.0, y, w, h, "Resume") { self.ui.paused = false; } y += 50.0;
        if button(cx - w / 2.0, y, w, h, "Journal (missions)") { self.ui.paused = false; self.ui.journal = true; } y += 50.0;
        if button(cx - w / 2.0, y, w, h, "World map (M)") { self.ui.paused = false; self.in_game = true; self.scene = Scene::Map; self.ui.map_sys = self.pdef().system; self.ui.free_pick = self.mode != Mode::Campaign; } y += 50.0;
        if button(cx - w / 2.0, y, w, h, "Controls") { self.in_game = true; self.scene = Scene::Controls; } y += 50.0;
        if button(cx - w / 2.0, y, w, h, "Settings") { self.in_game = true; self.scene = Scene::Settings; } y += 50.0;
        if button(cx - w / 2.0, y, w, h, "Quit to title") { self.save.write(); self.ui.paused = false; self.in_game = false; self.scene = Scene::Title; }
    }

    // ------------------------------------------------------------------ title
    fn scene_title(&mut self) {
        let (w, h) = (self.screen_w, self.screen_h);
        // starfield + drifting planets backdrop
        for i in 0..120 { let x = (i as f32 * 97.31 + get_time() as f32 * (3.0 + (i % 5) as f32)).rem_euclid(w); let y = (i as f32 * 53.77).rem_euclid(h); draw_rectangle(x, y, 2.0, 2.0, Color::new(1.0, 1.0, 1.0, 0.3 + (i % 7) as f32 * 0.1)); }
        for (k, id) in ["midgard", "muspell", "niflheim", "kalevala"].iter().enumerate() { let a = get_time() as f32 * 0.1 + k as f32 * 1.7; self.draw_icon(&format!("planets/{}", id), w * 0.5 + a.cos() * w * 0.42 - 32.0, h * 0.62 + a.sin() * h * 0.28 - 32.0, 1.0 + (k % 2) as f32 * 0.5); }
        if let Some(t) = self.assets.tex("ui/logo") { let s = (w * 0.7 / t.width()).min(2.4); draw_texture_ex(t, w / 2.0 - t.width() * s / 2.0, h * 0.06, WHITE, DrawTextureParams { dest_size: Some(vec2(t.width() * s, t.height() * s)), ..Default::default() }); } else { text_c("nordSoldPow", w / 2.0, h * 0.18, 80.0, COL_GOLD); }
        text_c("Powder Toy x Soldat  -  Viking & Kalevala myth, modern guns", w / 2.0, h * 0.06 + 96.0 * 2.0 + 14.0, 22.0, Color::new(0.8, 0.85, 1.0, 1.0));
        let (bw, bh) = (360.0, 40.0); let x = w / 2.0 - bw / 2.0; let mut y = h * 0.40;
        let has = !self.save.completed.is_empty();
        if button(x, y, bw, bh, if has { "Campaign - Continue (1P)" } else { "Campaign - New game (1P)" }) { self.begin(Mode::Campaign, false); } y += bh + 8.0;
        if button(x, y, bw, bh, "Campaign Co-op (2P splitscreen)") { self.begin(Mode::Campaign, true); } y += bh + 8.0;
        if button(x, y, bw, bh, "Sandbox (paint the universe)") { self.begin(Mode::Sandbox, false); } y += bh + 8.0;
        if button(x, y, bw, bh, "Versus (2P splitscreen)") { self.begin(Mode::Versus, true); } y += bh + 8.0;
        if button(x, y, bw / 2.0 - 4.0, bh, "Controls") { self.in_game = false; self.scene = Scene::Controls; }
        if button(x + bw / 2.0 + 4.0, y, bw / 2.0 - 4.0, bh, "Settings") { self.in_game = false; self.scene = Scene::Settings; } y += bh + 8.0;
        if button(x, y, bw / 2.0 - 4.0, bh, "Credits") { self.scene = Scene::Credits; }
        if button(x + bw / 2.0 + 4.0, y, bw / 2.0 - 4.0, bh, "Quit") { self.save.write(); std::process::exit(0); }
        text_c(&format!("Sampo shards {}/14   Silver {}   Missions {}/{}", self.save.shards, self.save.silver, self.save.completed.len(), MISSIONS.len()), w / 2.0, h - 40.0, 18.0, LIGHTGRAY);
        text_c("by Crowelian 2026 + Sonnet", w / 2.0, h - 18.0, 16.0, GRAY);
    }

    fn begin(&mut self, mode: Mode, two: bool) {
        self.pending = (mode, two); self.in_game = false; self.ui.free_pick = mode != Mode::Campaign; self.ui.map_sys = 0;
        if mode == Mode::Campaign && !self.save.intro_seen { self.save.intro_seen = true; self.save.write(); self.start_story(crate::story::INTRO, After::Map); } else { self.scene = Scene::Map; }
    }

    // ------------------------------------------------------------------ settings / controls / credits
    fn back_target(&mut self) { self.scene = if self.in_game { Scene::Play } else { Scene::Title }; if self.in_game { self.ui.paused = true; } }
    fn scene_settings(&mut self) {
        let (cx, mut y) = (self.screen_w / 2.0, 130.0);
        text_c("SETTINGS", cx, 80.0, 48.0, COL_GOLD);
        let (bw, bh) = (60.0, 34.0);
        let mut row = |g: &mut Game, label: &str, val: String, dec: &mut bool, inc: &mut bool, y: f32| { text(label, cx - 260.0, y + 24.0, 24.0, WHITE); *dec = button(cx + 40.0, y, bw, bh, "-"); text_c(&val, cx + 40.0 + bw + 50.0, y + 24.0, 24.0, YELLOW); *inc = button(cx + 40.0 + bw + 100.0, y, bw, bh, "+"); let _ = g; };
        let (mut d, mut i) = (false, false);
        row(self, "Music volume", format!("{}%", (self.audio.music_vol * 100.0).round()), &mut d, &mut i, y); if d { self.audio.music_vol = (self.audio.music_vol - 0.1).max(0.0); } if i { self.audio.music_vol = (self.audio.music_vol + 0.1).min(1.0); } y += 50.0;
        row(self, "Effects volume", format!("{}%", (self.audio.sfx_vol * 100.0).round()), &mut d, &mut i, y); if d { self.audio.sfx_vol = (self.audio.sfx_vol - 0.1).max(0.0); } if i { self.audio.sfx_vol = (self.audio.sfx_vol + 0.1).min(1.0); } y += 50.0;
        row(self, "Camera zoom", format!("{:.1}", self.zoom), &mut d, &mut i, y); if d { self.zoom = (self.zoom - 0.5).max(1.0); } if i { self.zoom = (self.zoom + 0.5).min(6.0); } y += 60.0;
        self.save.music_vol = self.audio.music_vol; self.save.sfx_vol = self.audio.sfx_vol; self.save.zoom = self.zoom;
        let a = &self.assets; self.audio.apply_volume(a);
        if button(cx - 150.0, y, 300.0, 40.0, "Toggle fullscreen") { set_fullscreen(!self.fullscreen); self.fullscreen = !self.fullscreen; } y += 50.0;
        if button(cx - 150.0, y, 300.0, 40.0, if self.confirm_reset { "CLICK AGAIN: erase save" } else { "Reset campaign progress" }) { if self.confirm_reset { let keep = (self.save.music_vol, self.save.sfx_vol, self.save.zoom); self.save = crate::save::Save::default(); self.save.music_vol = keep.0; self.save.sfx_vol = keep.1; self.save.zoom = keep.2; self.save.write(); self.confirm_reset = false; self.toast("Progress reset."); } else { self.confirm_reset = true; } } y += 60.0;
        if button(cx - 150.0, y, 300.0, 40.0, "Back") || is_key_pressed(KeyCode::Escape) { self.save.write(); self.confirm_reset = false; self.back_target(); }
    }

    fn scene_controls(&mut self) {
        let cx = self.screen_w / 2.0; text_c("CONTROLS", cx, 70.0, 48.0, COL_GOLD);
        let rows = [("Player 1", "Move A/D   Jump / jetpack W or Space   Crouch S   Aim mouse   Fire LMB   Grenade RMB"), ("", "Reload R   Next/prev weapon E/Q or wheel   Slots 1-8   Interact F"),
            ("Player 2", "Move Left/Right   Jump / jetpack Up   Crouch Down   Aim up/down  ,  and  .   Fire Enter"), ("", "Grenade Right Shift   Reload /   Next weapon '   Prev weapon ;   Interact \\"),
            ("Powder", "B build mode   LMB paint   RMB erase   Wheel brush size   [ ] element   1-0 quick elements   Tab picker"), ("", "P pause sim   O step sim   Clone: paint CLONE over a material to duplicate it"),
            ("Game", "M map   J journal   Esc pause   F3 fps   F2 cheat menu   ` or F1 console (see commands.txt)"), ("Tips", "Fire burns wood/plants/oil. Water + lava = stone. Acid eats ground. Gunpowder/nitro/TNT explode.")];
        let mut y = 130.0; for (a, b) in rows { text(a, 60.0, y, 24.0, Color::new(1.0, 0.8, 0.3, 1.0)); text(b, 190.0, y, 20.0, WHITE); y += 38.0; }
        if button(cx - 150.0, y + 20.0, 300.0, 40.0, "Back") || is_key_pressed(KeyCode::Escape) { self.back_target(); }
    }

    fn scene_credits(&mut self) {
        let cx = self.screen_w / 2.0; text_c("CREDITS", cx, 90.0, 52.0, COL_GOLD);
        let lines = ["nordSoldPow", "", "Created by Crowelian 2026 + Sonnet", "", "Inspired by The Powder Toy (The Powder Toy Developers) and Soldat (Soldat Team).", "Code is an original Rust implementation; all art and audio are generated placeholders.", "", "Lore: Norse sagas + the Finnish Kalevala (Elias Lonnrot).", "Built with Rust + macroquad.", "", "Replace anything in assets/ - see assets.md"];
        let mut y = 160.0; for l in lines { text_c(l, cx, y, 24.0, WHITE); y += 34.0; }
        if button(cx - 150.0, y + 20.0, 300.0, 40.0, "Back") || is_key_pressed(KeyCode::Escape) { self.scene = Scene::Title; }
    }

    // ------------------------------------------------------------------ story pages
    fn scene_story(&mut self, dt: f32) {
        let (w, h) = (self.screen_w, self.screen_h);
        let Some(st) = self.story.as_mut() else { self.scene = Scene::Title; return };
        st.chars += dt * 55.0;
        let (who, txt) = st.pages[st.idx].clone();
        for i in 0..100 { let x = (i as f32 * 131.7).rem_euclid(w); let y = (i as f32 * 71.3).rem_euclid(h); draw_rectangle(x, y, 2.0, 2.0, Color::new(1.0, 1.0, 1.0, 0.4)); }
        let shown: String = txt.chars().take(st.chars as usize).collect();
        panel(w * 0.1, h * 0.55, w * 0.8, h * 0.3);
        if !who.is_empty() { text(&who, w * 0.1 + 20.0, h * 0.55 + 36.0, 28.0, Color::new(1.0, 0.8, 0.3, 1.0)); }
        crate::ui::wrap_text(&shown, w * 0.1 + 20.0, h * 0.55 + if who.is_empty() { 40.0 } else { 70.0 }, 26.0, w * 0.8 - 40.0, WHITE);
        text_c(&format!("SPACE / click to continue  ({}/{})   ESC skip", st.idx + 1, st.pages.len()), w / 2.0, h * 0.9, 18.0, GRAY);
        let adv = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter) || is_mouse_button_pressed(MouseButton::Left);
        let skip = is_key_pressed(KeyCode::Escape);
        if adv && (st.chars as usize) < txt.chars().count() { st.chars = 9999.0; }
        else if adv || skip {
            if skip || st.idx + 1 >= st.pages.len() { let after = st.after; self.story = None; self.scene = match after { After::Map => Scene::Map, After::Play => Scene::Play, After::Title => Scene::Title }; }
            else { st.idx += 1; st.chars = 0.0; }
        }
    }

    // ------------------------------------------------------------------ solar map
    fn scene_map(&mut self) {
        let (w, h) = (self.screen_w, self.screen_h);
        let sys = self.ui.map_sys; let (cx, cy) = (w / 2.0, h / 2.0 + 10.0); let r = (w * 0.5).min(h * 0.5) * 0.82;
        for i in 0..150 { let x = (i as f32 * 97.31).rem_euclid(w); let y = (i as f32 * 53.77).rem_euclid(h); draw_rectangle(x, y, 2.0, 2.0, Color::new(1.0, 1.0, 1.0, 0.2 + (i % 5) as f32 * 0.1)); }
        text_c(&format!("{} SYSTEM", if sys == 0 { "SOL - NORSE" } else { "PAIVATAR - KALEVALA" }), w / 2.0, 44.0, 40.0, COL_GOLD);
        text_c(if self.ui.free_pick { "Pick any world" } else { "Click a world to land.  Complete all 3 main missions on a world to unlock the next." }, w / 2.0, 70.0, 18.0, LIGHTGRAY);
        let starkey = format!("planets/{}", star_id(sys)); let ss = 1.0 + (get_time() as f32 * 2.0).sin() * 0.03;
        self.draw_icon(&starkey, cx - 48.0 * ss, cy - 48.0 * ss, ss);
        let mut hover: Option<usize> = None; let (mx, my) = mouse_position(); let t = get_time() as f32;
        for (pi, p) in PLANETS.iter().enumerate() {
            if p.system != sys { continue; }
            let rr = p.orbit.0 * r; draw_circle_lines(cx, cy, rr, 1.0, Color::new(1.0, 1.0, 1.0, 0.12));
            let a = (p.orbit.1 + t * (4.0 / (1.0 + p.orbit.0 * 6.0))).to_radians();
            let (px, py) = (cx + a.cos() * rr, cy + a.sin() * rr * 0.62);
            let unlocked = self.planet_unlocked(pi); let size = 64.0 * if rr > r * 0.7 { 1.0 } else { 0.85 };
            let is_hover = (mx - px).abs() < size / 2.0 && (my - py).abs() < size / 2.0;
            if is_hover { hover = Some(pi); }
            if let Some(tx) = self.assets.tex(&format!("planets/{}", p.id)) { let tint = if unlocked { WHITE } else { Color::new(0.25, 0.25, 0.3, 1.0) }; let s = if is_hover { 1.15 } else { 1.0 }; draw_texture_ex(tx, px - size * s / 2.0, py - size * s / 2.0, tint, DrawTextureParams { dest_size: Some(vec2(size * s, size * s)), ..Default::default() }); } else { draw_circle(px, py, size / 2.0, GRAY); }
            text_c(p.name, px, py + size / 2.0 + 18.0, 20.0, if unlocked { WHITE } else { GRAY });
            if !unlocked { text_c("LOCKED", px, py + 4.0, 16.0, Color::new(1.0, 0.4, 0.4, 1.0)); } else if self.cleared(p.id) { text_c("CLEARED", px, py + size / 2.0 + 34.0, 15.0, GREEN); }
            if is_hover && is_mouse_button_pressed(MouseButton::Left) && unlocked { self.land_selected(pi); return; }
        }
        // info panel
        if let Some(pi) = hover { let p = &PLANETS[pi]; let (x, y) = (20.0, h - 190.0); panel(x, y, 420.0, 170.0);
            text(&format!("{} ({})", p.name, p.analog), x + 12.0, y + 26.0, 24.0, COL_GOLD); text(&format!("Gravity {:.2}g   Hazard {:?}   Tier {}", p.gravity, p.hazard, p.tier), x + 12.0, y + 50.0, 17.0, WHITE);
            crate::ui::wrap_text(p.desc, x + 12.0, y + 74.0, 17.0, 396.0, LIGHTGRAY);
            let done = MISSIONS.iter().filter(|m| m.planet == p.id && self.save.completed.contains(m.id)).count();
            text(&format!("Missions {}/5   Boss: {}", done, p.boss.map(|b| crate::enemy::index(b).map(|i| crate::enemy::ENEMIES[i].name).unwrap_or("?")).unwrap_or("none")), x + 12.0, y + 150.0, 17.0, Color::new(1.0, 0.7, 0.4, 1.0));
            if !self.planet_unlocked(pi) { let need: Vec<&str> = if p.system == 1 && !self.save.gate_open { vec!["Open the Bifrost gate (clear Asgard)"] } else { p.after.to_vec() }; text(&format!("Needs: {}", need.join(", ")), x + 12.0, y + 130.0, 16.0, Color::new(1.0, 0.4, 0.4, 1.0)); } }
        // buttons
        if (self.save.gate_open || self.cheats.unlock_all || self.ui.free_pick) && button(w - 330.0, h - 130.0, 310.0, 40.0, if sys == 0 { "Aurora Gate -> Kalevala system" } else { "Bifrost Gate -> Sol system" }) { self.ui.map_sys = 1 - sys; self.sfx("gate"); }
        else if sys == 0 && !self.save.gate_open && !self.ui.free_pick { text("Aurora Gate: dormant (clear Asgard)", w - 330.0, h - 105.0, 16.0, GRAY); }
        if button(w - 330.0, h - 80.0, 310.0, 40.0, if self.in_game { "Back to game" } else { "Back to title" }) || is_key_pressed(KeyCode::Escape) { if self.in_game { self.scene = Scene::Play; } else { self.scene = Scene::Title; } }
        text(&format!("Shards {}/14   Silver {}", self.save.shards, self.save.silver), 20.0, 30.0, 22.0, WHITE);
    }

    fn land_selected(&mut self, pi: usize) {
        let (mode, two) = if self.in_game { (self.mode, self.split) } else { self.pending };
        self.in_game = true; self.sfx("gate");
        if mode == Mode::Campaign { self.save.write(); }
        self.land(pi, mode, two);
    }
}
