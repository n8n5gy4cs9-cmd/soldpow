//! World + entity rendering for each viewport.
use crate::bullet::*;
use crate::elements::*;
use crate::enemy::*;
use crate::game::*;
use crate::ui::*;
use crate::weapons::WEAPONS;
use macroquad::prelude::*;

const FOOT: f32 = 1.0;

fn sky_text_hash(i: i32) -> f32 { let mut h = (i as u32).wrapping_mul(2654435761); h ^= h >> 15; h = h.wrapping_mul(2246822519); h ^= h >> 13; (h & 0xffff) as f32 / 65535.0 }

impl Game {
    pub fn draw_play(&mut self) {
        self.world.render(&mut self.img.bytes, &self.assets.colors);
        self.tex.update(&self.img);
        let views = self.compute_views();
        let split = views.len() > 1;
        for (i, v) in views.iter().enumerate() {
            if split { self.scissor(Some(*v)); }
            self.draw_view(i, v);
            self.draw_hud(i, v);
            if split { self.scissor(None); }
        }
        if split { draw_rectangle(self.screen_w / 2.0 - 1.0, 0.0, 2.0, self.screen_h, Color::new(0.0, 0.0, 0.0, 1.0)); }
        self.draw_toasts();
        self.draw_boss_bar();
        if self.ui.picker { self.draw_picker(); }
        if self.show_fps { text(&format!("{} fps  cells {}  ents {} bul {} part {}", get_fps(), self.world.w * self.world.h, self.enemies.len(), self.bullets.len(), self.particles.len()), 10.0, self.screen_h - 4.0, 16.0, GREEN); }
    }

    fn scissor(&self, v: Option<View>) {
        let mut gl = unsafe { get_internal_gl() };
        gl.flush();
        let dpi = screen_dpi_scale();
        match v { Some(v) => gl.quad_gl.scissor(Some(((v.x * dpi) as i32, (v.y * dpi) as i32, (v.w * dpi) as i32, (v.h * dpi) as i32))), None => gl.quad_gl.scissor(None) }
    }

    fn draw_view(&self, pi: usize, v: &View) {
        let p = self.pdef(); let z = v.z; let sky = self.sky();
        // sky gradient
        let bands = 24;
        for i in 0..bands {
            let t = i as f32 / (bands - 1) as f32; let (a, b) = (sky[0], sky[1]);
            let c = Color::from_rgba((a.0 as f32 + (b.0 as f32 - a.0 as f32) * t) as u8, (a.1 as f32 + (b.1 as f32 - a.1 as f32) * t) as u8, (a.2 as f32 + (b.2 as f32 - a.2 as f32) * t) as u8, 255);
            draw_rectangle(v.x, v.y + v.h * i as f32 / bands as f32, v.w, v.h / bands as f32 + 1.0, c);
        }
        let dark = sky[0].0 as u32 + sky[0].1 as u32 + sky[0].2 as u32;
        if dark < 200 || p.id == "svartalfheim" {
            for i in 0..90 { let sx = sky_text_hash(i * 2) * v.w; let sy = sky_text_hash(i * 2 + 1) * v.h * 0.8; let tw = ((self.tick as f32 * 0.03 + i as f32).sin() * 0.3 + 0.7).max(0.2); draw_rectangle(v.x + (sx - v.cx * 0.05 * z).rem_euclid(v.w), v.y + sy, 2.0, 2.0, Color::new(1.0, 1.0, 1.0, tw)); }
        }
        // parallax silhouettes
        if let Some(bg) = self.assets.tex(&format!("bg/{}", p.id)) {
            let horizon = self.gen.surface[WORLD_W / 2] as f32;
            for (layer, (par, tint)) in [(0.12f32, 0.55f32), (0.28, 0.85)].iter().enumerate() {
                let s = z * if layer == 0 { 0.5 } else { 0.8 }; let iw = bg.width() * s; let ih = bg.height() * s;
                let (_, by) = v.to_screen(0.0, horizon + 6.0 - layer as f32 * 6.0);
                let off = -(v.cx * par * z).rem_euclid(iw);
                let mut x = v.x + off - iw;
                while x < v.x + v.w { draw_texture_ex(bg, x, by - ih, Color::new(tint * 1.0, tint * 1.0, tint * 1.0, 1.0), DrawTextureParams { dest_size: Some(vec2(iw, ih)), ..Default::default() }); x += iw; }
                draw_rectangle(v.x, by - 1.0, v.w, v.h, Color::new(0.0, 0.0, 0.0, 0.0));
            }
        }
        // world texture
        let (sw, sh) = (v.w / z, v.h / z);
        let (ox, oy) = if self.shake > 0.2 { (macroquad::rand::gen_range(-1.0, 1.0) * self.shake * 0.5, macroquad::rand::gen_range(-1.0, 1.0) * self.shake * 0.5) } else { (0.0, 0.0) };
        draw_texture_ex(&self.tex, v.x + ox, v.y + oy, WHITE, DrawTextureParams { dest_size: Some(vec2(v.w, v.h)), source: Some(Rect::new(v.cx - sw / 2.0, v.cy - sh / 2.0, sw, sh)), ..Default::default() });
        // zones (debug / objective markers)
        if self.debug { for zn in &self.gen.zones { let (x0, y0) = v.to_screen(zn.x0 as f32, zn.y0 as f32); draw_rectangle_lines(x0, y0, (zn.x1 - zn.x0) as f32 * z, (zn.y1 - zn.y0) as f32 * z, 2.0, MAGENTA); text(zn.name, x0 + 4.0, y0 + 16.0, 16.0, MAGENTA); } }
        if self.mode == Mode::Campaign { self.draw_markers(v); }
        // hotspots
        for h in &self.hotspots {
            if !v.visible(h.x, h.y, 24.0) { continue; }
            let (sx, sy) = v.to_screen(h.x, h.y);
            let bob = (self.tick as f32 * 0.08).sin() * 1.5;
            if let Some(t) = self.assets.tex(&format!("ui/hotspot_{}", h.kind.key())) { draw_texture_ex(t, sx - 8.0 * z, sy - 15.0 * z + bob, if h.used { Color::new(0.6, 0.6, 0.6, 1.0) } else { WHITE }, DrawTextureParams { dest_size: Some(vec2(16.0 * z, 16.0 * z)), ..Default::default() }); }
            else { draw_rectangle(sx - 6.0 * z, sy - 12.0 * z, 12.0 * z, 12.0 * z, COL_GOLD); }
            let near = self.players.iter().any(|pl| pl.alive && (pl.body.x - h.x).abs() < 24.0 && (pl.body.y - h.y).abs() < 24.0);
            if near { text_c(&format!("[{}] {}", if self.split { "F / \\" } else { "F" }, h.kind.label()), sx, sy - 22.0 * z, 18.0, YELLOW); }
        }
        // pickups
        for k in &self.pickups {
            if !v.visible(k.x, k.y, 12.0) { continue; }
            let (sx, sy) = v.to_screen(k.x, k.y); let bob = (self.tick as f32 * 0.1 + k.x).sin() * 1.2;
            let key = match k.kind { PickKind::Health => "ui/pickup_health", PickKind::Ammo => "ui/pickup_ammo", PickKind::Shard => "ui/pickup_shard", PickKind::Silver(_) => "ui/pickup_silver", PickKind::Weapon(_) => "ui/pickup_weapon" };
            let s = if matches!(k.kind, PickKind::Shard) { 1.5 } else { 1.0 };
            if let Some(t) = self.assets.tex(key) { draw_texture_ex(t, sx - 6.0 * z * s, sy - 10.0 * z * s + bob, WHITE, DrawTextureParams { dest_size: Some(vec2(12.0 * z * s, 12.0 * z * s)), ..Default::default() }); } else { draw_circle(sx, sy - 4.0 * z, 3.0 * z, COL_GOLD); }
            if let PickKind::Weapon(wi) = k.kind { text_c(WEAPONS[wi].name, sx, sy - 14.0 * z, 15.0, WHITE); }
        }
        // enemies
        for e in &self.enemies { if v.visible(e.body.x, e.body.y, 60.0) { self.draw_enemy(e, v); } }
        // players
        for (i, pl) in self.players.iter().enumerate() { if v.visible(pl.body.x, pl.body.y, 40.0) { self.draw_player(i, v); } }
        // bullets
        for b in &self.bullets { if v.visible(b.x, b.y, 10.0) { self.draw_bullet(b, v); } }
        // particles
        for pt in &self.particles { if v.visible(pt.x, pt.y, 4.0) { let (sx, sy) = v.to_screen(pt.x, pt.y); let a = (pt.life / pt.max).clamp(0.0, 1.0); draw_rectangle(sx, sy, (pt.size * z * 0.6).max(1.5), (pt.size * z * 0.6).max(1.5), Color::new(pt.col.r, pt.col.g, pt.col.b, a)); } }
        // lightning warnings
        for (x, t) in &self.strikes { let (sx, _) = v.to_screen(*x, 0.0); if t % 6 < 3 { draw_line(sx, v.y, sx, v.y + v.h, 2.0, Color::new(1.0, 1.0, 0.4, 0.5)); } }
        // crosshair / brush
        if pi == 0 { self.draw_cursor(v); } else if self.players[1].alive { let q = &self.players[1]; let (sx, sy) = v.to_screen(q.body.x + q.aim.cos() * 60.0, q.body.y - 14.0 + q.aim.sin() * 60.0); self.draw_icon("ui/crosshair", sx - 16.0, sy - 16.0, 2.0); }
        if let Some(pl) = self.players.get(pi) { if pl.hurt_t > 0 { draw_rectangle(v.x, v.y, v.w, v.h, Color::new(1.0, 0.0, 0.0, 0.12)); } if pl.burn > 0 { draw_rectangle(v.x, v.y + v.h * 0.85, v.w, v.h * 0.15, Color::new(1.0, 0.4, 0.0, 0.15)); } }
    }

    fn draw_markers(&self, v: &View) {
        let Some(ri) = self.tracked_mission() else { return };
        let m = &crate::missions::MISSIONS[self.runs[ri].def];
        use crate::missions::Obj;
        let zone = match m.obj { Obj::Reach(_) | Obj::Survive(_) => self.gen.zone("goal"), Obj::Fill(_, z, _) | Obj::Burn(z) => self.gen.zone(z), Obj::Boss => self.gen.zone("arena"), _ => None };
        if let Some(z) = zone { let (cx, cy) = z.center(); let (sx, sy) = v.to_screen(cx, z.y0 as f32 - 12.0 + (self.tick as f32 * 0.06).sin() * 3.0); let (x0, y0) = v.to_screen(z.x0 as f32, z.y0 as f32); draw_rectangle_lines(x0, y0, (z.x1 - z.x0) as f32 * v.z, (z.y1 - z.y0) as f32 * v.z, 1.5, Color::new(1.0, 0.9, 0.3, 0.45)); draw_triangle(vec2(sx - 8.0, sy - 14.0), vec2(sx + 8.0, sy - 14.0), vec2(sx, sy), Color::new(1.0, 0.85, 0.2, 0.9)); let _ = cy; }
    }

    fn draw_cursor(&self, v: &View) {
        let (mx, my) = mouse_position();
        if self.brush.build && !self.ui.paused {
            let r = self.brush.size as f32 * v.z; draw_circle_lines(mx, my, r, 1.5, Color::new(1.0, 1.0, 1.0, 0.8)); draw_circle(mx, my, 2.0, el_color(&self.assets, self.brush.el));
            text(def(self.brush.el).name, mx + r + 6.0, my, 16.0, WHITE);
        } else if self.players.first().map_or(false, |p| p.alive) { self.draw_icon("ui/crosshair", mx - 16.0, my - 16.0, 2.0); }
    }

    fn draw_bullet(&self, b: &Bullet, v: &View) {
        let z = v.z; let (sx, sy) = v.to_screen(b.x, b.y);
        match b.proj {
            Proj::Bullet | Proj::Bolt => { let l = 5.0; let sp = (b.vx * b.vx + b.vy * b.vy).sqrt().max(0.1); draw_line(sx, sy, sx - b.vx / sp * l * z, sy - b.vy / sp * l * z, (1.0 * z).max(1.5), b.color); }
            Proj::Rocket => { if let Some(t) = self.assets.tex("ui/proj_rocket") { draw_texture_ex(t, sx - 5.0 * z, sy - 2.5 * z, WHITE, DrawTextureParams { dest_size: Some(vec2(10.0 * z, 5.0 * z)), rotation: b.vy.atan2(b.vx), ..Default::default() }); } else { draw_circle(sx, sy, 3.0 * z, ORANGE); } }
            Proj::Grenade => { if let Some(t) = self.assets.tex("ui/proj_grenade") { draw_texture_ex(t, sx - 3.5 * z, sy - 3.5 * z, WHITE, DrawTextureParams { dest_size: Some(vec2(7.0 * z, 7.0 * z)), rotation: self.tick as f32 * 0.2, ..Default::default() }); } else { draw_circle(sx, sy, 3.0 * z, GREEN); } }
            Proj::Flame | Proj::Fireball => { let r = if matches!(b.proj, Proj::Fireball) { 4.0 } else { 1.5 + (b.life as f32 * 0.1).sin().abs() * 1.5 }; draw_circle(sx, sy, r * z, Color::new(1.0, 0.6, 0.1, 0.6)); }
            Proj::Frost => draw_circle(sx, sy, 2.0 * z, Color::new(0.7, 0.9, 1.0, 0.6)),
            Proj::Spray(_) => draw_rectangle(sx, sy, 1.5 * z, 1.5 * z, b.color),
            Proj::Trick => draw_circle(sx, sy, 2.5 * z, Color::new(1.0, 0.3 + (self.tick % 10) as f32 * 0.07, 0.8, 1.0)),
            Proj::Thunder => {}
        }
    }

    fn sprite(&self, key: &str, frame: usize, fw: f32, fh: f32, x: f32, y: f32, sc: f32, flip: bool, tint: Color, v: &View) {
        // (x,y) = feet position in world
        let (sx, sy) = v.to_screen(x, y);
        let (dw, dh) = (fw * sc * v.z, fh * sc * v.z);
        if let Some(t) = self.assets.tex(key) {
            draw_texture_ex(t, sx - dw / 2.0, sy - dh + FOOT * sc * v.z, tint, DrawTextureParams { dest_size: Some(vec2(dw, dh)), source: Some(Rect::new(frame as f32 * fw, 0.0, fw, fh)), flip_x: flip, ..Default::default() });
        } else { draw_rectangle(sx - dw / 3.0, sy - dh, dw * 0.66, dh, tint); }
    }

    fn frame_for(on_ground: bool, vy: f32, vx: f32, anim: f32) -> usize { if !on_ground { if vy < 0.0 { 5 } else { 6 } } else if vx.abs() > 0.25 { 1 + (anim as usize) % 4 } else { 0 } }

    fn draw_enemy(&self, e: &Enemy, v: &View) {
        let d = e.d(); let z = v.z;
        let key = if is_boss(e.def) { format!("sprites/boss_{}", d.key) } else { format!("sprites/enemy_{}", d.key) };
        let tint = if e.hurt > 0 { Color::new(1.0, 0.6, 0.6, 1.0) } else if e.slow > 0 { Color::new(0.7, 0.85, 1.0, 1.0) } else if e.burn > 0 { Color::new(1.0, 0.75, 0.5, 1.0) } else { WHITE };
        let frame = match d.ai { Ai::Boss(_) => 0, Ai::Quad => if e.body.on_ground && e.body.vx.abs() > 0.3 { 1 + (e.anim as usize) % 2 } else { 0 }, Ai::Ghost => 0, Ai::Flyer => 5, _ => Self::frame_for(e.body.on_ground, e.body.vy, e.body.vx, e.anim) };
        let bob = if matches!(d.ai, Ai::Ghost | Ai::Flyer) { (self.tick as f32 * 0.07 + e.body.x).sin() * 1.5 } else { 0.0 };
        self.sprite(&key, frame, d.fw, d.fh, e.body.x, e.body.y + bob, d.scale, e.body.face < 0.0, tint, v);
        if matches!(d.ai, Ai::Ranged | Ai::Flyer) {
            let gun = match d.key { "draugr" => "hugin", "eldjotunn" => "surtr_flame", "svartalf" => "gungnir_sniper", _ => "lemminkainen_smg" };
            let (sx, sy) = v.to_screen(e.body.x, e.body.y - d.h * 0.6 + bob);
            let a = if e.charge > 0 { e.aim } else if e.body.face >= 0.0 { 0.3 } else { std::f32::consts::PI - 0.3 };
            self.draw_gun(gun, sx, sy, a, z);
        }
        if !is_boss(e.def) && e.hp < e.max_hp { let (sx, sy) = v.to_screen(e.body.x, e.body.y - d.h * d.scale - 4.0); bar(sx - 10.0 * z / 1.5, sy, 20.0 * z / 1.5, 3.0, e.hp / e.max_hp, Color::new(0.9, 0.2, 0.2, 1.0)); }
    }

    fn draw_gun(&self, key: &str, sx: f32, sy: f32, aim: f32, z: f32) {
        let left = aim.cos() < 0.0;
        if let Some(t) = self.assets.tex(&format!("weapons/{}", key)) {
            let (w, h) = (24.0 * z, 12.0 * z); let top = if left { sy - 5.0 * z } else { sy - 7.0 * z };
            draw_texture_ex(t, sx - 5.0 * z, top, WHITE, DrawTextureParams { dest_size: Some(vec2(w, h)), rotation: aim, flip_y: left, pivot: Some(vec2(sx, sy)), ..Default::default() });
        } else { draw_line(sx, sy, sx + aim.cos() * 14.0 * z, sy + aim.sin() * 14.0 * z, 2.0 * z, DARKGRAY); }
    }

    fn draw_player(&self, i: usize, v: &View) {
        let p = &self.players[i]; let z = v.z;
        let key = format!("sprites/player{}", i + 1);
        if !p.alive { self.sprite(&key, 7, 16.0, 24.0, p.body.x, p.body.y, 1.0, p.body.face < 0.0, WHITE, v); return; }
        if p.invuln > 0 && (self.tick / 4) % 2 == 0 { return; }
        let frame = Self::frame_for(p.body.on_ground, p.body.vy, p.body.vx, p.anim);
        let tint = if p.hurt_t > 0 { Color::new(1.0, 0.6, 0.6, 1.0) } else if p.burn > 0 { Color::new(1.0, 0.8, 0.6, 1.0) } else { WHITE };
        self.sprite(&key, frame, 16.0, 24.0, p.body.x, p.body.y, 1.0, p.body.face < 0.0, tint, v);
        let (sx, sy) = v.to_screen(p.shoulder().0, p.shoulder().1);
        self.draw_gun(WEAPONS[p.slot().w].key, sx, sy, p.aim, z);
        if self.split { let (nx, ny) = v.to_screen(p.body.x, p.body.y - 28.0); text_c(&format!("P{}", i + 1), nx, ny, 16.0, if i == 0 { Color::new(0.5, 0.8, 1.0, 1.0) } else { Color::new(1.0, 0.5, 0.5, 1.0) }); }
    }
}
