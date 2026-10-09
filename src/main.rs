//! nordSoldPow - Powder Toy x Soldat in Nordic / Kalevala myth.  by Crowelian 2026 + Sonnet
mod ai; mod assets; mod audio; mod bullet; mod combat; mod console; mod elements; mod enemy; mod entity; mod game; mod hotspot; mod input;
mod missions; mod planets; mod play; mod player; mod render; mod save; mod scenes; mod sim; mod story; mod terrain; mod ui; mod weapons;

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf { window_title: "nordSoldPow".to_owned(), window_width: 1280, window_height: 720, high_dpi: true, sample_count: 0, window_resizable: true, ..Default::default() }
}

#[macroquad::main(conf)]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flag = |f: &str| args.iter().any(|a| a == f);
    let val = |f: &str| args.iter().position(|a| a == f).and_then(|i| args.get(i + 1)).cloned();
    if flag("--help") { println!("nordSoldPow options: --nosound  --planet <id>  --mode campaign|sandbox|versus  --split  --console \"cmd; cmd\"  --shot <file.png>  --frames <n>  --selftest"); return; }
    macroquad::rand::srand(macroquad::miniquad::date::now() as u64);
    let sound = !flag("--nosound");
    let assets = assets::Assets::load(sound).await;
    let mut game = game::Game::new(assets, sound);
    if flag("--selftest") { selftest(&mut game); return; }
    // developer launch shortcuts (also used for automated screenshots)
    if let Some(id) = val("--planet") {
        let mode = match val("--mode").as_deref() { Some("sandbox") => game::Mode::Sandbox, Some("versus") => game::Mode::Versus, _ => game::Mode::Campaign };
        if let Some(pi) = planets::by_id(&id) { game.save.intro_seen = true; game.land(pi, mode, flag("--split") || mode == game::Mode::Versus); }
        if let Some(cmds) = val("--console") { for c in cmds.split(';') { game.exec(c.trim()); } game.console.open = false; }
    }
    match val("--scene").as_deref() { Some("map") => { game.ui.free_pick = true; game.scene = game::Scene::Map; } Some("settings") => game.scene = game::Scene::Settings, Some("controls") => game.scene = game::Scene::Controls, Some("credits") => game.scene = game::Scene::Credits, _ => {} }
    let shot = val("--shot"); let frames: u32 = val("--frames").and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut n = 0u32;
    loop {
        game.frame();
        n += 1;
        if frames > 0 && n == frames { if let Some(f) = &shot { get_screen_data().export_png(f); } if flag("--exit") || shot.is_some() { break; } }
        next_frame().await;
    }
}

/// Headless smoke test: generate every planet, run the simulation, spawn entities, tick the game logic.
fn selftest(game: &mut game::Game) {
    use game::*;
    for pi in 0..planets::PLANETS.len() {
        game.land(pi, Mode::Campaign, true);
        for k in 0..600 { game.tick(); if k == 100 { game.exec("spawn draugr 5"); } }
        println!("{:<13} spawn={:?} enemies={} players_alive={} cells_nonempty={}", planets::PLANETS[pi].id, game.gen.spawn, game.enemies.len(), game.alive_players().len(), game.world.cells.iter().filter(|c| c.el != 0).count());
    }
    // combat test: shoot a draugr with the rifle
    game.land(0, Mode::Campaign, false);
    game.exec("give vainamoinen_gun");
    for _ in 0..60 { game.tick(); }
    let (px, py) = (game.players[0].body.x, game.players[0].body.y);
    game.enemies.clear();
    game.spawn_enemy(enemy::index("draugr").unwrap(), px + 60.0, py);
    for _ in 0..30 { game.tick(); }
    let before = game.kills;
    for _ in 0..240 {
        let (ex, ey) = game.enemies.get(0).map(|e| e.body.center()).unwrap_or((px + 60.0, py - 10.0));
        let p = &mut game.players[0]; p.hp = p.max_hp; p.ctl.fire = true; p.ctl.aim_world = Some((ex, ey)); p.ctl.fire_pressed = true;
        game.tick();
    }
    println!("combat: kills {} -> {}  enemies {}", before, game.kills, game.enemies.len());
    assert!(game.kills > before, "rifle failed to kill a draugr");
    // powder interaction: bullets ignite, explosions carve
    game.exec("boom 14");
    println!("selftest OK");
}
