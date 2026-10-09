//! Developer console + cheat commands. The command table is the single source of truth (commands.txt is checked against it).
use crate::bullet::*;
use crate::elements::*;
use crate::enemy;
use crate::game::*;
use crate::hotspot::*;
use crate::missions::*;
use crate::planets::*;
use crate::ui::*;
use crate::weapons::{self, WEAPONS};
use macroquad::prelude::*;

/// (name, args, help)
pub const COMMANDS: &[(&str, &str, &str)] = &[
    ("help", "[cmd]", "Show all commands or help for one command"),
    ("clear", "", "Clear the console output"),
    ("echo", "<text>", "Print text"),
    ("fps", "", "Toggle FPS counter"),
    ("debug", "", "Toggle debug overlay (zones)"),
    ("list", "<what>", "List names: weapons | elements | planets | enemies | bosses | missions"),
    ("quit", "", "Exit the game"),
    ("give", "<weapon|all>", "Give a weapon (full ammo), e.g. give vainamoinen_gun"),
    ("ammo", "", "Refill all ammo"),
    ("heal", "", "Full health"),
    ("god", "", "Toggle invincibility"),
    ("ghost", "", "Toggle no-clip flight"),
    ("fly", "", "Toggle infinite jetpack fuel"),
    ("infammo", "", "Toggle infinite ammo"),
    ("speed", "<mult>", "Run speed multiplier"),
    ("jump", "<mult>", "Jump strength multiplier"),
    ("gravity", "<mult>", "Player gravity multiplier"),
    ("kill", "", "Kill yourself"),
    ("respawn", "", "Respawn at spawn point"),
    ("silver", "<n>", "Add silver"),
    ("pos", "", "Print player cell position"),
    ("tp", "<planet>", "Teleport to a planet (loads it)"),
    ("warp", "<x> <y>", "Move player to cell x,y"),
    ("regen", "", "Regenerate current planet"),
    ("clearworld", "", "Remove all loose powder/liquids/gases (terrain stays)"),
    ("unlock", "<all|planet>", "Unlock travel to planet(s)"),
    ("weather", "<element|off>", "Rain an element from the sky"),
    ("split", "<on|off>", "Toggle splitscreen (2 players)"),
    ("paint", "<element> [size]", "Select element and brush size, enter build mode"),
    ("drop", "<element> [radius]", "Drop a blob of element above the player"),
    ("boom", "[radius]", "Explosion at the cursor"),
    ("pause", "", "Pause/resume the powder simulation"),
    ("step", "", "Advance the simulation one frame while paused"),
    ("simspeed", "<n>", "Simulation steps per frame (1-4)"),
    ("spawn", "<enemy> [n]", "Spawn n enemies near cursor"),
    ("boss", "<boss>", "Spawn a boss"),
    ("killall", "", "Kill all enemies"),
    ("missions", "", "List missions of the current planet"),
    ("complete", "<id|all>", "Complete a mission (id like midgard_m1, m1, or all)"),
    ("shard", "", "Spawn a Sampo shard pickup next to you"),
    ("music", "<on|off>", "Toggle music"),
    ("volume", "<0-1>", "Master (music) volume"),
    ("zoom", "<n>", "Camera zoom (1-6)"),
];

fn names(what: &str) -> Vec<String> {
    match what {
        "weapons" => WEAPONS.iter().map(|w| w.key.to_string()).collect(),
        "elements" => ELEMENTS.iter().skip(1).map(|e| e.key.to_string()).collect(),
        "planets" => PLANETS.iter().map(|p| p.id.to_string()).collect(),
        "enemies" => enemy::ENEMIES.iter().filter(|e| !matches!(e.ai, enemy::Ai::Boss(_))).map(|e| e.key.to_string()).collect(),
        "bosses" => enemy::ENEMIES.iter().filter(|e| matches!(e.ai, enemy::Ai::Boss(_))).map(|e| e.key.to_string()).collect(),
        "missions" => MISSIONS.iter().map(|m| m.id.to_string()).collect(),
        _ => vec![],
    }
}

fn onoff(b: bool) -> &'static str { if b { "ON" } else { "off" } }

impl Game {
    fn out(&mut self, s: impl Into<String>) { self.console.out.push(s.into()); if self.console.out.len() > 200 { self.console.out.remove(0); } }

    pub fn exec(&mut self, line: &str) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { return; }
        let cmd = parts[0].to_ascii_lowercase(); let a1 = parts.get(1).copied().unwrap_or(""); let a2 = parts.get(2).copied().unwrap_or("");
        let num = |s: &str, d: f32| s.parse::<f32>().unwrap_or(d);
        let has_player = !self.players.is_empty();
        if !COMMANDS.iter().any(|c| c.0 == cmd) { self.out(format!("Unknown command '{}'. Type help.", cmd)); return; }
        match cmd.as_str() {
            "help" => {
                if let Some(c) = COMMANDS.iter().find(|c| c.0 == a1) { self.out(format!("{} {}  -  {}", c.0, c.1, c.2)); }
                else { let mut row = String::new(); for c in COMMANDS { row.push_str(c.0); row.push(' '); if row.len() > 90 { self.out(row.clone()); row.clear(); } } if !row.is_empty() { self.out(row); } self.out("help <cmd> for details. Example: give vainamoinen_gun"); }
            }
            "clear" => self.console.out.clear(),
            "echo" => { let t = parts[1..].join(" "); self.out(t); }
            "fps" => { self.show_fps = !self.show_fps; }
            "debug" => { self.debug = !self.debug; self.out(format!("debug {}", onoff(self.debug))); }
            "list" => { let n = names(a1); if n.is_empty() { self.out("list weapons|elements|planets|enemies|bosses|missions"); } else { let mut row = String::new(); for s in n { row.push_str(&s); row.push_str("  "); if row.len() > 90 { self.out(row.clone()); row.clear(); } } if !row.is_empty() { self.out(row); } } }
            "quit" => self.quit = true,
            "give" => {
                if !has_player { return; }
                if a1 == "all" { for (i, w) in WEAPONS.iter().enumerate() { self.save.unlocked.insert(w.key.to_string()); self.players[0].give(i, true); } self.out("Gave all weapons."); }
                else if let Some(i) = weapons::index(a1) { self.save.unlocked.insert(WEAPONS[i].key.to_string()); self.players[0].give(i, true); self.out(format!("Gave {}", WEAPONS[i].name)); }
                else { self.out(format!("Unknown weapon '{}'. Try: list weapons", a1)); }
            }
            "ammo" => { for p in self.players.iter_mut() { p.refill(); } self.out("Ammo refilled."); }
            "heal" => { for p in self.players.iter_mut() { p.hp = p.max_hp; p.fuel = 100.0; } self.out("Healed."); }
            "god" => { self.cheats.god = !self.cheats.god; self.out(format!("god {}", onoff(self.cheats.god))); }
            "ghost" => { self.cheats.ghost = !self.cheats.ghost; self.out(format!("ghost {}", onoff(self.cheats.ghost))); }
            "fly" => { self.cheats.fly = !self.cheats.fly; self.out(format!("infinite fuel {}", onoff(self.cheats.fly))); }
            "infammo" => { self.cheats.inf_ammo = !self.cheats.inf_ammo; self.out(format!("infinite ammo {}", onoff(self.cheats.inf_ammo))); }
            "speed" => { self.cheats.speed = num(a1, 1.0).clamp(0.1, 6.0); self.out(format!("speed x{}", self.cheats.speed)); }
            "jump" => { self.cheats.jump = num(a1, 1.0).clamp(0.1, 4.0); self.out(format!("jump x{}", self.cheats.jump)); }
            "gravity" => { self.cheats.grav = num(a1, 1.0).clamp(0.0, 5.0); self.out(format!("gravity x{}", self.cheats.grav)); }
            "kill" => { if has_player { self.kill_player(0, None); } }
            "respawn" => { if has_player { self.respawn_player(0); } }
            "silver" => { let n = a1.parse::<i64>().unwrap_or(0); self.save.silver = (self.save.silver as i64 + n).max(0) as u32; self.out(format!("silver = {}", self.save.silver)); }
            "pos" => { if has_player { let b = &self.players[0].body; self.out(format!("x={:.0} y={:.0}  planet {}", b.x, b.y, self.pdef().id)); } }
            "tp" => {
                if let Some(pi) = crate::planets::by_id(a1) { let (m, s) = (self.mode, self.split); self.out(format!("Teleporting to {}...", PLANETS[pi].name)); self.land(pi, m, s); self.console.open = true; }
                else { self.out(format!("Unknown planet '{}'. Try: list planets", a1)); }
            }
            "warp" => { if has_player { let (x, y) = (num(a1, 100.0), num(a2, 100.0)); let b = &mut self.players[0].body; b.x = x.clamp(5.0, WORLD_W as f32 - 5.0); b.y = y.clamp(25.0, WORLD_H as f32 - 5.0); b.vx = 0.0; b.vy = 0.0; } }
            "regen" => { let (p, m, s) = (self.planet, self.mode, self.split); self.land(p, m, s); self.console.open = true; }
            "clearworld" => { self.world.clear_loose(); self.out("Loose matter cleared."); }
            "unlock" => {
                if a1 == "all" { self.cheats.unlock_all = true; self.save.gate_open = true; self.out("All worlds unlocked."); }
                else if let Some(pi) = crate::planets::by_id(a1) {
                    if PLANETS[pi].system == 1 { self.save.gate_open = true; }
                    for a in PLANETS[pi].after { for m in MISSIONS.iter().filter(|m| m.planet == *a && !m.optional) { self.save.completed.insert(m.id.to_string()); } }
                    self.out(format!("{} unlocked.", PLANETS[pi].name));
                } else { self.out("unlock all | <planet>"); }
                self.save.write();
            }
            "weather" => { if a1 == "off" || a1.is_empty() { self.weather = None; self.out("weather off"); } else if let Some(e) = by_key(a1) { self.weather = Some(e); self.out(format!("raining {}", def(e).name)); } else { self.out("unknown element"); } }
            "split" => { let on = a1 != "off"; if on && self.players.len() < 2 { let (sx, sy) = self.gen.spawn; let mut p = crate::player::Player::new(1, sx as f32 + 10.0, sy as f32); p.loadout(&self.save.unlocked, true); self.players.push(p); } self.split = on && self.players.len() > 1; self.out(format!("splitscreen {}", onoff(self.split))); }
            "paint" => { if let Some(e) = by_key(a1) { self.brush.el = e; if !a2.is_empty() { self.brush.size = (num(a2, 4.0) as i32).clamp(1, 24); } self.brush.build = true; self.out(format!("brush: {} size {}", def(e).name, self.brush.size)); } else { self.out("paint <element> [size]. Try: list elements"); } }
            "drop" => { if has_player { if let Some(e) = by_key(a1) { let r = num(a2, 6.0).clamp(1.0, 24.0) as i32; let (x, y) = (self.players[0].body.x as i32 + 14 * self.players[0].body.face as i32, self.players[0].body.y as i32 - 40); self.world.paint(x, y, e, r); } else { self.out("drop <element> [radius]"); } } }
            "boom" => { let r = num(a1, 12.0).clamp(2.0, 60.0) as i32; let (x, y) = self.mouse_world; self.explode_at(x, y, r, r as f32 * 6.0, Owner::World); }
            "pause" => { self.sim_paused = !self.sim_paused; self.out(format!("sim paused: {}", onoff(self.sim_paused))); }
            "step" => { self.step_once = true; }
            "simspeed" => { self.sim_speed = (num(a1, 1.0) as u32).clamp(1, 4); self.out(format!("simspeed {}", self.sim_speed)); }
            "spawn" => {
                if let Some(di) = enemy::index(a1) { let n = a2.parse::<u32>().unwrap_or(1).min(30); let (mx, my) = self.mouse_world; for _ in 0..n { self.spawn_enemy(di, (mx + macroquad::rand::gen_range(-20.0, 20.0)).clamp(10.0, WORLD_W as f32 - 10.0), my.min(WORLD_H as f32 - 20.0)); } self.out(format!("spawned {} {}", n, a1)); }
                else { self.out("spawn <enemy> [n]. Try: list enemies"); }
            }
            "boss" => { if let Some(di) = enemy::index(a1) { let (mx, my) = self.mouse_world; self.spawn_enemy(di, mx, my); self.boss_alive = true; } else { self.out("boss <name>. Try: list bosses"); } }
            "killall" => { for i in 0..self.enemies.len() { self.enemies[i].hp = -1.0; } self.out("All enemies slain."); }
            "missions" => {
                let lines: Vec<String> = self.runs.iter().enumerate().map(|(i, r)| { let m = &MISSIONS[r.def]; format!("{} [{}] {}  {}", m.id, if r.done { "done" } else if self.mission_active(i) { "ACTIVE" } else { "locked" }, m.title, describe(&m.obj)) }).collect();
                for l in lines { self.out(l); }
            }
            "complete" => {
                let n = self.runs.len();
                for ri in 0..n {
                    let id = MISSIONS[self.runs[ri].def].id; let short = id.rsplit('_').next().unwrap_or("");
                    if a1 == "all" || a1 == id || a1 == short { self.complete_mission(ri); }
                }
            }
            "shard" => { if has_player { let b = &self.players[0].body; self.pickups.push(Pickup { kind: PickKind::Shard, x: b.x + 20.0, y: b.y - 10.0, vy: 0.0, life: u32::MAX }); } }
            "music" => { self.audio.music_on = a1 != "off"; if !self.audio.music_on { let a = &self.assets; self.audio.stop_music(a); } else { let m = self.pdef().music; let a = &self.assets; self.audio.music(a, m); } }
            "volume" => { self.audio.music_vol = num(a1, 0.5).clamp(0.0, 1.0); self.save.music_vol = self.audio.music_vol; let a = &self.assets; self.audio.apply_volume(a); }
            "zoom" => { self.zoom = num(a1, 3.0).clamp(1.0, 6.0); self.save.zoom = self.zoom; }
            _ => {}
        }
    }

    fn complete_word(&self, input: &str) -> Option<String> {
        let parts: Vec<&str> = input.split(' ').collect();
        let (cands, prefix, head): (Vec<String>, &str, String) = if parts.len() <= 1 { (COMMANDS.iter().map(|c| c.0.to_string()).collect(), parts.first().copied().unwrap_or(""), String::new()) } else {
            let list = match parts[0] { "give" => { let mut v = names("weapons"); v.push("all".into()); v } "paint" | "drop" | "weather" => names("elements"), "tp" | "unlock" => names("planets"), "spawn" => names("enemies"), "boss" => names("bosses"), "complete" => names("missions"), "list" => vec!["weapons".into(), "elements".into(), "planets".into(), "enemies".into(), "bosses".into(), "missions".into()], _ => vec![] };
            (list, parts[parts.len() - 1], parts[..parts.len() - 1].join(" ") + " ")
        };
        let m: Vec<&String> = cands.iter().filter(|c| c.starts_with(prefix)).collect();
        if m.is_empty() { return None; }
        let mut common = m[0].clone(); for c in &m { while !c.starts_with(&common) { common.pop(); } }
        Some(format!("{}{}", head, if m.len() == 1 { format!("{} ", m[0]) } else { common }))
    }

    pub fn update_console(&mut self) {
        if !self.console.open { return; }
        while let Some(c) = get_char_pressed() { if c == '`' || c == '~' || c == '§' || c.is_control() { continue; } if self.console.input.len() < 120 { self.console.input.push(c); } }
        if is_key_down(KeyCode::Backspace) { self.console.bs_hold += 1; if self.console.bs_hold == 1 || (self.console.bs_hold > 24 && self.console.bs_hold % 2 == 0) { self.console.input.pop(); } } else { self.console.bs_hold = 0; }
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::GraveAccent) || is_key_pressed(KeyCode::F1) { self.console.open = false; return; }
        if is_key_pressed(KeyCode::Tab) { if let Some(s) = self.complete_word(&self.console.input.clone()) { self.console.input = s; } }
        if is_key_pressed(KeyCode::Up) && !self.console.history.is_empty() { self.console.hist_idx = self.console.hist_idx.saturating_sub(1); self.console.input = self.console.history[self.console.hist_idx].clone(); }
        if is_key_pressed(KeyCode::Down) { if self.console.hist_idx + 1 < self.console.history.len() { self.console.hist_idx += 1; self.console.input = self.console.history[self.console.hist_idx].clone(); } else { self.console.hist_idx = self.console.history.len(); self.console.input.clear(); } }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
            let line = std::mem::take(&mut self.console.input);
            if !line.trim().is_empty() { self.out(format!("> {}", line)); self.console.history.push(line.clone()); self.console.hist_idx = self.console.history.len(); self.exec(line.trim()); }
        }
    }

    pub fn draw_console(&self) {
        if !self.console.open { return; }
        let h = (self.screen_h * 0.45).max(200.0);
        draw_rectangle(0.0, 0.0, self.screen_w, h, Color::new(0.02, 0.02, 0.06, 0.92)); draw_line(0.0, h, self.screen_w, h, 2.0, Color::new(0.8, 0.6, 0.2, 1.0));
        let lines = ((h - 40.0) / 18.0) as usize; let start = self.console.out.len().saturating_sub(lines);
        for (i, l) in self.console.out[start..].iter().enumerate() { text(l, 10.0, 20.0 + i as f32 * 18.0, 17.0, if l.starts_with('>') { Color::new(0.6, 0.9, 1.0, 1.0) } else { LIGHTGRAY }); }
        let cur = if (get_time() * 2.0) as i32 % 2 == 0 { "_" } else { " " };
        text(&format!("> {}{}", self.console.input, cur), 10.0, h - 10.0, 20.0, YELLOW);
        text("nordSoldPow console - Tab=complete, Up/Down=history, Esc/` close, 'help'", self.screen_w - 560.0, h - 10.0, 14.0, GRAY);
    }

    pub fn draw_cheat_menu(&mut self) {
        if !self.ui.cheat_menu { return; }
        let items: &[(&str, &str)] = &[("God mode", "god"), ("Ghost (no-clip)", "ghost"), ("Infinite jetpack", "fly"), ("Infinite ammo", "infammo"), ("Heal", "heal"), ("Refill ammo", "ammo"), ("Give ALL weapons", "give all"), ("Unlock all worlds", "unlock all"),
            ("Complete all missions here", "complete all"), ("Spawn 5 draugr", "spawn draugr 5"), ("Spawn boss here", "boss fenrir"), ("Kill all enemies", "killall"), ("+1000 silver", "silver 1000"), ("Speed x2", "speed 2"), ("Speed x1", "speed 1"), ("Jump x2", "jump 2"),
            ("Moon gravity", "gravity 0.3"), ("Normal gravity", "gravity 1"), ("Rain lava", "weather lava"), ("Rain water", "weather water"), ("Weather off", "weather off"), ("Clear loose matter", "clearworld"), ("Toggle splitscreen", "split on"), ("Spawn shard", "shard")];
        let cols = 3; let (bw, bh) = (200.0, 30.0); let rows = (items.len() + cols - 1) / cols;
        let (w, h) = (cols as f32 * (bw + 8.0) + 16.0, rows as f32 * (bh + 6.0) + 60.0); let (x, y) = ((self.screen_w - w) / 2.0, 70.0);
        panel(x, y, w, h); text("CHEAT MENU (F2)   -   same as console commands, see commands.txt", x + 10.0, y + 24.0, 18.0, YELLOW);
        let mut clicked: Option<&str> = None;
        for (i, (label, cmd)) in items.iter().enumerate() { let (cx, cy) = (x + 10.0 + (i % cols) as f32 * (bw + 8.0), y + 40.0 + (i / cols) as f32 * (bh + 6.0)); if button(cx, cy, bw, bh, label) { clicked = Some(cmd); } }
        if let Some(c) = clicked { self.exec(c); self.toast(format!("cheat: {}", c)); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn commands_txt_matches_registry() {
        let txt = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/commands.txt")).unwrap();
        for (n, _, _) in COMMANDS { assert!(txt.lines().any(|l| l.split_whitespace().next() == Some(*n)), "commands.txt missing '{}'", n); }
        // and no documented command is missing from the registry
        for l in txt.lines() { if l.starts_with(|c: char| c.is_ascii_lowercase()) && !l.starts_with("nordSold") { let w = l.split_whitespace().next().unwrap(); if w.chars().all(|c| c.is_ascii_lowercase()) && l.contains("  ") { assert!(COMMANDS.iter().any(|c| c.0 == w), "registry missing documented '{}'", w); } } }
    }
    #[test] fn weapon_and_element_names_unique() { let mut s = std::collections::HashSet::new(); for w in WEAPONS.iter() { assert!(s.insert(w.key)); } let mut e = std::collections::HashSet::new(); for d in ELEMENTS.iter() { assert!(e.insert(d.key)); } }
}
