//! Immediate-mode UI helpers + HUD, element picker, journal, toasts.
use crate::elements::*;
use crate::enemy::*;
use crate::game::*;
use crate::missions::*;
use crate::weapons::WEAPONS;
use macroquad::prelude::*;

pub const FONT: f32 = 20.0;
pub const COL_GOLD: Color = Color::new(1.0, 0.84, 0.0, 1.0);
pub fn text(s: &str, x: f32, y: f32, size: f32, col: Color) { draw_text(s, x + 1.0, y + 1.0, size, Color::new(0.0, 0.0, 0.0, 0.8 * col.a)); draw_text(s, x, y, size, col); }
pub fn text_w(s: &str, size: f32) -> f32 { measure_text(s, None, size as u16, 1.0).width }
pub fn text_c(s: &str, cx: f32, y: f32, size: f32, col: Color) { text(s, cx - text_w(s, size) / 2.0, y, size, col); }
pub fn panel(x: f32, y: f32, w: f32, h: f32) { draw_rectangle(x, y, w, h, Color::new(0.04, 0.04, 0.09, 0.94)); draw_rectangle_lines(x, y, w, h, 2.0, Color::new(0.6, 0.5, 0.25, 1.0)); }
pub fn inside(x: f32, y: f32, w: f32, h: f32) -> bool { let (mx, my) = mouse_position(); mx >= x && mx <= x + w && my >= y && my <= y + h }
/// Returns true when clicked this frame.
static CLICKED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn take_click() -> bool { CLICKED.swap(false, std::sync::atomic::Ordering::Relaxed) }
pub fn button(x: f32, y: f32, w: f32, h: f32, label: &str) -> bool {
    let hov = inside(x, y, w, h);
    draw_rectangle(x, y, w, h, if hov { Color::new(0.35, 0.28, 0.12, 0.95) } else { Color::new(0.12, 0.12, 0.2, 0.92) });
    draw_rectangle_lines(x, y, w, h, 2.0, if hov { Color::new(1.0, 0.85, 0.4, 1.0) } else { Color::new(0.5, 0.45, 0.3, 1.0) });
    let fs = (h * 0.55).min(26.0); text_c(label, x + w / 2.0, y + h / 2.0 + fs * 0.3, fs, WHITE);
    let c = hov && is_mouse_button_pressed(MouseButton::Left); if c { CLICKED.store(true, std::sync::atomic::Ordering::Relaxed); } c
}
pub fn bar(x: f32, y: f32, w: f32, h: f32, frac: f32, col: Color) {
    draw_rectangle(x, y, w, h, Color::new(0.0, 0.0, 0.0, 0.6)); draw_rectangle(x + 1.0, y + 1.0, (w - 2.0) * frac.clamp(0.0, 1.0), h - 2.0, col); draw_rectangle_lines(x, y, w, h, 1.0, Color::new(1.0, 1.0, 1.0, 0.5));
}
pub fn el_color(a: &crate::assets::Assets, el: u8) -> Color { let c = a.colors[el as usize]; Color::from_rgba(c.0, c.1, c.2, 255) }

impl Game {
    pub fn draw_icon(&self, key: &str, x: f32, y: f32, scale: f32) {
        if let Some(t) = self.assets.tex(key) { draw_texture_ex(t, x, y, WHITE, DrawTextureParams { dest_size: Some(vec2(t.width() * scale, t.height() * scale)), ..Default::default() }); }
    }

    pub fn draw_hud(&self, pi: usize, v: &View) {
        let p = &self.players[pi];
        let (x, y) = (v.x + 12.0, v.y + v.h - 78.0);
        panel(x - 6.0, y - 10.0, 230.0, 80.0);
        text(&format!("P{}", pi + 1), x, y + 6.0, 18.0, if pi == 0 { Color::new(0.5, 0.8, 1.0, 1.0) } else { Color::new(1.0, 0.5, 0.5, 1.0) });
        bar(x + 30.0, y - 4.0, 180.0, 14.0, p.hp / p.max_hp, Color::new(0.85, 0.15, 0.15, 1.0));
        text(&format!("{}", p.hp.ceil() as i32), x + 34.0, y + 8.0, 14.0, WHITE);
        self.draw_icon("ui/icon_fuel", x + 6.0, y + 14.0, 1.5);
        bar(x + 30.0, y + 16.0, 180.0, 8.0, p.fuel / 100.0, Color::new(1.0, 0.7, 0.2, 1.0));
        let w = p.weapon(); let s = p.slot();
        let ammo = if w.mag == 0 { "inf".to_string() } else if p.reload_t > 0 { "reloading...".to_string() } else { format!("{}/{}", s.mag, s.reserve) };
        let mut nm = w.name.to_string(); if nm.len() > 24 { nm.truncate(22); nm.push_str(".."); }
        text(&nm, x, y + 40.0, 15.0, WHITE);
        text(&ammo, x + 150.0, y + 40.0, 17.0, if p.reload_t > 0 { YELLOW } else { Color::new(1.0, 0.9, 0.5, 1.0) });
        text(&format!("Nades {}   Silver {}", p.nades, self.save.silver), x, y + 62.0, 15.0, LIGHTGRAY);
        // weapon slots
        for (i, sl) in p.slots.iter().enumerate() {
            let sx = v.x + v.w / 2.0 - 2.0 * 46.0 + i as f32 * 46.0; let sy = v.y + v.h - 36.0;
            draw_rectangle(sx, sy, 42.0, 22.0, Color::new(0.0, 0.0, 0.0, if i == p.cur { 0.7 } else { 0.4 }));
            draw_rectangle_lines(sx, sy, 42.0, 22.0, if i == p.cur { 2.0 } else { 1.0 }, if i == p.cur { YELLOW } else { GRAY });
            if let Some(t) = self.assets.tex(&format!("weapons/{}", WEAPONS[sl.w].key)) { draw_texture_ex(t, sx + 3.0, sy + 3.0, WHITE, DrawTextureParams { dest_size: Some(vec2(36.0, 18.0)), ..Default::default() }); }
        }
        if !p.alive { text_c(&format!("You died. Respawn in {}...", p.dead_t / 60 + 1), v.x + v.w / 2.0, v.y + v.h / 2.0, 30.0, Color::new(1.0, 0.3, 0.3, 1.0)); }
        if self.mode == Mode::Versus { text(&format!("Frags {}/{}", p.kills, self.versus_target), v.x + 14.0, v.y + 22.0, 24.0, WHITE); }
        if pi == 0 || self.split { self.draw_objective(pi, v); }
        if pi == 0 && self.brush.build { text(&format!("BUILD: {} (size {})  wheel=size  [ ]=element  Tab=picker  RMB=erase  B=exit", def(self.brush.el).name, self.brush.size), v.x + 14.0, v.y + v.h - 90.0, 18.0, Color::new(0.6, 1.0, 0.7, 1.0)); }
    }

    fn draw_objective(&self, pi: usize, v: &View) {
        if self.mode != Mode::Campaign { return; }
        let Some(ri) = self.tracked_mission() else { return };
        let r = &self.runs[ri]; let m = &MISSIONS[r.def];
        let (px, py) = (self.screen_w - 330.0, 12.0);
        if pi != 0 { return; }
        panel(px, py, 318.0, 74.0);
        text(&format!("{}{}", if m.optional { "(optional) " } else { "" }, m.title), px + 8.0, py + 20.0, 20.0, Color::new(1.0, 0.85, 0.4, 1.0));
        text(&describe(&m.obj), px + 8.0, py + 40.0, 15.0, WHITE);
        let t = target(&m.obj);
        let prog = match m.obj { Obj::Burn(_) => format!("{}%", r.progress), Obj::Reach(_) | Obj::Boss => String::new(), _ => format!("{}/{}", r.progress, t) };
        text(&prog, px + 8.0, py + 60.0, 16.0, GREEN);
        text("Tab/J journal", px + 190.0, py + 66.0, 13.0, GRAY);
        // compass to objective target
        let zone = match m.obj { Obj::Reach(_) | Obj::Survive(_) => self.gen.zone("goal"), Obj::Fill(_, z, _) | Obj::Burn(z) => self.gen.zone(z), Obj::Boss => self.gen.zone("arena"), _ => None };
        if let Some(z) = zone { let (cx, _) = z.center(); let dx = cx - self.players[pi].body.x; let arrow = if dx > 0.0 { "-->" } else { "<--" }; text(&format!("{} {}m", arrow, dx.abs() as i32), px + 214.0, py + 60.0, 16.0, Color::new(0.5, 0.9, 1.0, 1.0)); }
    }

    pub fn draw_toasts(&self) {
        let mut y = 70.0;
        for (s, t) in self.toasts.iter().rev() { let a = (t / 1.0).min(1.0); let w = text_w(s, 20.0) + 20.0; draw_rectangle(self.screen_w / 2.0 - w / 2.0, y - 18.0, w, 26.0, Color::new(0.05, 0.05, 0.1, 0.8 * a)); text_c(s, self.screen_w / 2.0, y, 20.0, Color::new(1.0, 1.0, 1.0, a)); y += 30.0; }
        let mut ry = 14.0;
        for r in &self.radio {
            let a = (r.t / 1.0).min(1.0); let w = (self.screen_w * 0.5).min(640.0);
            let lines = wrap_lines(&r.text, 17.0, w - 20.0); let top = if r.who.is_empty() { 22.0 } else { 40.0 }; let h = top + lines.len() as f32 * 19.0 + 4.0;
            panel(14.0, ry, w, h); if !r.who.is_empty() { text(&r.who, 24.0, ry + 20.0, 20.0, Color::new(1.0, 0.8, 0.3, a)); }
            for (i, l) in lines.iter().enumerate() { text(l, 24.0, ry + top + i as f32 * 19.0, 17.0, Color::new(1.0, 1.0, 1.0, a)); }
            ry += h + 6.0;
        }
        if let Some((b, t)) = &self.banner { let a = (t / 0.8).min(1.0); text_c(b, self.screen_w / 2.0, self.screen_h * 0.22, 44.0, Color::new(1.0, 0.9, 0.6, a)); }
    }

    pub fn draw_boss_bar(&self) {
        if let Some(e) = self.enemies.iter().find(|e| is_boss(e.def) && !e.dead) {
            let w = self.screen_w * 0.5; let x = self.screen_w / 2.0 - w / 2.0;
            text_c(e.d().name, self.screen_w / 2.0, self.screen_h - 100.0, 24.0, Color::new(1.0, 0.5, 0.3, 1.0));
            bar(x, self.screen_h - 90.0, w, 16.0, e.hp / e.max_hp, Color::new(0.9, 0.2, 0.15, 1.0));
        }
    }

    pub fn draw_picker(&mut self) {
        let (w, h) = (360.0, 330.0); let (x, y) = (self.screen_w - w - 16.0, 100.0);
        panel(x, y, w, h); text("Element picker (click or Tab to close)", x + 10.0, y + 22.0, 17.0, YELLOW);
        let mut cy = y + 34.0;
        for (name, els) in PICKER {
            text(name, x + 10.0, cy + 14.0, 16.0, LIGHTGRAY); cy += 20.0;
            let mut cx = x + 10.0;
            for &e in els.iter() {
                let (bw, bh) = (80.0, 22.0);
                if cx + bw > x + w - 6.0 { cx = x + 10.0; cy += bh + 3.0; }
                let hov = inside(cx, cy, bw, bh);
                draw_rectangle(cx, cy, bw, bh, el_color(&self.assets, e));
                draw_rectangle_lines(cx, cy, bw, bh, if self.brush.el == e { 3.0 } else if hov { 2.0 } else { 1.0 }, if self.brush.el == e { WHITE } else { BLACK });
                let c = self.assets.colors[e as usize]; let lum = c.0 as f32 * 0.3 + c.1 as f32 * 0.6 + c.2 as f32 * 0.1;
                text(def(e).name, cx + 3.0, cy + 16.0, 14.0, if lum > 120.0 { BLACK } else { WHITE });
                if hov && is_mouse_button_pressed(MouseButton::Left) { self.brush.el = e; self.brush.build = true; }
                cx += bw + 4.0;
            }
            cy += 28.0;
        }
        text("Brush size: wheel | RMB erase | Clone: paint over a material", x + 10.0, y + h - 10.0, 14.0, GRAY);
    }

    pub fn draw_journal(&mut self) {
        let (w, h) = (self.screen_w.min(900.0) - 40.0, self.screen_h - 120.0); let (x, y) = ((self.screen_w - w) / 2.0, 60.0);
        panel(x, y, w, h);
        text(&format!("Journal - {}   (J to close, click a mission to track)", self.pdef().name), x + 14.0, y + 26.0, 22.0, YELLOW);
        text(&format!("Sampo shards: {}/14    Silver: {}    Kills here: {}", self.save.shards, self.save.silver, self.kills), x + 14.0, y + 50.0, 18.0, WHITE);
        let mut ly = y + 82.0; let mut idx = 0;
        let mut click: Option<usize> = None;
        for (i, r) in self.runs.iter().enumerate() {
            let m = &MISSIONS[r.def]; let act = self.mission_active(i);
            let col = if r.done { GREEN } else if act { WHITE } else { GRAY };
            let hov = inside(x + 8.0, ly - 18.0, w - 16.0, 50.0);
            if hov { draw_rectangle(x + 8.0, ly - 18.0, w - 16.0, 50.0, Color::new(1.0, 1.0, 1.0, 0.07)); if is_mouse_button_pressed(MouseButton::Left) && act { click = Some(idx); } }
            text(&format!("[{}] {}{}", if r.done { "x" } else { " " }, m.title, if m.optional { "  (optional)" } else { "" }), x + 16.0, ly, 20.0, col);
            text(&describe(&m.obj), x + 36.0, ly + 20.0, 15.0, if act || r.done { LIGHTGRAY } else { DARKGRAY });
            if act { text(&format!("{}/{}", r.progress, target(&m.obj)), x + w - 90.0, ly, 18.0, GREEN); idx += 1; }
            ly += 56.0;
        }
        if let Some(c) = click { self.ui.tracked = c; }
        text("Tip: B = build mode (paint powders). Missions like 'Fill the Well' need you to pour elements into the basin.", x + 14.0, y + h - 14.0, 15.0, GRAY);
    }

    pub fn update_ui_timers(&mut self, dt: f32) {
        for t in self.toasts.iter_mut() { t.1 -= dt; } self.toasts.retain(|t| t.1 > 0.0);
        for r in self.radio.iter_mut() { r.t -= dt; } self.radio.retain(|r| r.t > 0.0);
        if let Some((_, t)) = self.banner.as_mut() { *t -= dt; if *t <= 0.0 { self.banner = None; } }
    }
}

pub fn wrap_lines(s: &str, size: f32, maxw: f32) -> Vec<String> {
    let mut out = vec![]; let mut line = String::new();
    for word in s.split_whitespace() {
        let t = if line.is_empty() { word.to_string() } else { format!("{} {}", line, word) };
        if text_w(&t, size) > maxw && !line.is_empty() { out.push(std::mem::take(&mut line)); line = word.to_string(); } else { line = t; }
    }
    if !line.is_empty() { out.push(line); } out
}

pub fn wrap_text(s: &str, x: f32, y: f32, size: f32, maxw: f32, col: Color) {
    let mut line = String::new(); let mut ly = y;
    for word in s.split_whitespace() {
        let try_line = if line.is_empty() { word.to_string() } else { format!("{} {}", line, word) };
        if text_w(&try_line, size) > maxw && !line.is_empty() { text(&line, x, ly, size, col); ly += size + 2.0; line = word.to_string(); } else { line = try_line; }
    }
    if !line.is_empty() { text(&line, x, ly, size, col); }
}
