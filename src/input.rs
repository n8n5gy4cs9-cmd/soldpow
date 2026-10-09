//! Keyboard / mouse input -> per-player Controls, hotkeys and the build (paint) tool.
use crate::elements::*;
use crate::game::*;
use crate::player::Controls;
use macroquad::prelude::*;

const QUICK: [u8; 10] = [SAND, WATER, OIL, LAVA, FIRE, WOOD, GUNPOWDER, ACID, SNOW, STONE];

impl Game {
    pub fn input_blocked(&self) -> bool { self.console.open || self.ui.paused || self.ui.journal || self.ui.cheat_menu }

    pub fn gather_input(&mut self, views: &[View]) {
        let (mx, my) = mouse_position();
        self.mouse_world = views[0].to_world(mx, my);
        let blocked = self.input_blocked();
        let build = self.brush.build;
        let wheel = mouse_wheel().1;
        for (i, p) in self.players.iter_mut().enumerate() {
            if blocked { p.ctl = Controls::default(); continue; }
            let c = &mut p.ctl;
            if i == 0 {
                c.left = is_key_down(KeyCode::A); c.right = is_key_down(KeyCode::D); c.down = is_key_down(KeyCode::S);
                c.up = is_key_down(KeyCode::W) || is_key_down(KeyCode::Space);
                c.up_pressed |= is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Space);
                c.fire = !build && is_mouse_button_down(MouseButton::Left);
                c.fire_pressed |= !build && is_mouse_button_pressed(MouseButton::Left);
                c.nade_pressed |= !build && is_mouse_button_pressed(MouseButton::Right);
                c.reload |= is_key_pressed(KeyCode::R); c.interact |= is_key_pressed(KeyCode::F);
                c.next |= is_key_pressed(KeyCode::E) || (!build && wheel < 0.0); c.prev |= is_key_pressed(KeyCode::Q) || (!build && wheel > 0.0);
                c.aim_world = Some(self.mouse_world); c.aim_adjust = 0.0;
            } else {
                c.left = is_key_down(KeyCode::Left); c.right = is_key_down(KeyCode::Right); c.down = is_key_down(KeyCode::Down);
                c.up = is_key_down(KeyCode::Up); c.up_pressed |= is_key_pressed(KeyCode::Up);
                c.fire = is_key_down(KeyCode::Enter); c.fire_pressed |= is_key_pressed(KeyCode::Enter);
                c.nade_pressed |= is_key_pressed(KeyCode::RightShift);
                c.reload |= is_key_pressed(KeyCode::Slash); c.interact |= is_key_pressed(KeyCode::Backslash);
                c.next |= is_key_pressed(KeyCode::Apostrophe); c.prev |= is_key_pressed(KeyCode::Semicolon);
                c.aim_world = None; c.aim_adjust = (is_key_down(KeyCode::Comma) as i32 - is_key_down(KeyCode::Period) as i32) as f32 * 0.045;
            }
        }
        // number keys select weapon slot of P1 when not building
        if !blocked && !build && !self.players.is_empty() { for (k, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8].iter().enumerate() { if is_key_pressed(*key) && k < self.players[0].slots.len() { self.players[0].cur = k; self.players[0].reload_t = 0; } } }
    }

    /// Global hotkeys (Play scene).
    pub fn handle_hotkeys(&mut self) {
        if self.console.open { return; }
        if is_key_pressed(KeyCode::GraveAccent) || is_key_pressed(KeyCode::F1) { self.console.open = true; self.console.bs_hold = 0; while get_char_pressed().is_some() {} return; }
        if is_key_pressed(KeyCode::F2) { self.ui.cheat_menu = !self.ui.cheat_menu; }
        if is_key_pressed(KeyCode::F3) { self.show_fps = !self.show_fps; }
        if is_key_pressed(KeyCode::Escape) { if self.ui.cheat_menu { self.ui.cheat_menu = false; } else if self.ui.journal { self.ui.journal = false; } else if self.ui.picker { self.ui.picker = false; } else { self.ui.paused = !self.ui.paused; } }
        if self.ui.paused { return; }
        if is_key_pressed(KeyCode::J) { self.ui.journal = !self.ui.journal; }
        if is_key_pressed(KeyCode::M) { self.in_game = true; self.scene = Scene::Map; self.ui.map_sys = self.pdef().system; self.ui.free_pick = self.mode != Mode::Campaign; }
        if is_key_pressed(KeyCode::B) { self.brush.build = !self.brush.build; }
        if is_key_pressed(KeyCode::Tab) { if self.ui.journal { } else { self.ui.picker = !self.ui.picker; if self.ui.picker { self.brush.build = true; } } }
        if is_key_pressed(KeyCode::P) { self.sim_paused = !self.sim_paused; self.toast(if self.sim_paused { "Powder sim paused (O = step)" } else { "Powder sim running" }); }
        if is_key_pressed(KeyCode::O) { self.step_once = true; }
        if is_key_pressed(KeyCode::R) && self.mode == Mode::Versus && self.versus_won.is_some() { let pl = self.planet; self.land(pl, Mode::Versus, true); }
        if self.brush.build {
            if is_key_pressed(KeyCode::LeftBracket) { self.brush.el = if self.brush.el <= 1 { (NUM_ELEMENTS - 1) as u8 } else { self.brush.el - 1 }; }
            if is_key_pressed(KeyCode::RightBracket) { self.brush.el = if self.brush.el as usize + 1 >= NUM_ELEMENTS { 1 } else { self.brush.el + 1 }; }
            for (k, key) in [KeyCode::Key1, KeyCode::Key2, KeyCode::Key3, KeyCode::Key4, KeyCode::Key5, KeyCode::Key6, KeyCode::Key7, KeyCode::Key8, KeyCode::Key9, KeyCode::Key0].iter().enumerate() { if is_key_pressed(*key) { self.brush.el = QUICK[k]; } }
            let w = mouse_wheel().1; if w != 0.0 && !self.ui.picker { self.brush.size = (self.brush.size + if w > 0.0 { 1 } else { -1 }).clamp(1, 24); }
        }
    }

    /// Mouse painting for P1 in build mode.
    pub fn build_tool(&mut self, v: &View) {
        if !self.brush.build || self.input_blocked() || self.ui.picker && { let (x, y, w, h) = self.picker_rect(); crate::ui::inside(x, y, w, h) } { self.brush.last = None; return; }
        let (mx, my) = mouse_position(); let (wx, wy) = v.to_world(mx, my); let (ix, iy) = (wx as i32, wy as i32);
        let down = is_mouse_button_down(MouseButton::Left); let erase = is_mouse_button_down(MouseButton::Right);
        if down || erase {
            let el = if erase { EMPTY } else { self.brush.el };
            let (lx, ly) = self.brush.last.unwrap_or((ix, iy));
            self.world.paint_line(lx, ly, ix, iy, el, self.brush.size);
            self.brush.last = Some((ix, iy));
        } else { self.brush.last = None; }
    }
}
