//! nordSoldPow - Powder Toy x Soldat in Nordic / Kalevala myth.  by Crowelian 2026 + Sonnet
mod ai; mod arena; mod assets; mod audio; mod bullet; mod combat; mod console; mod elements; mod enemy; mod entity; mod game; mod hotspot; mod input;
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
    if flag("--dump-docs") { dump_docs(); return; }
    if flag("--help") { println!("nordSoldPow options: --nosound  --planet <id>  --mode campaign|sandbox|versus  --split  --console \"cmd; cmd\"  --shot <file.png>  --frames <n>  --selftest"); return; }
    if flag("--selftest") { std::env::set_var("NORDSOLDPOW_SAVE", std::env::temp_dir().join("nordsoldpow_selftest_save.txt")); }
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
    match val("--ui").as_deref() { Some("journal") => game.ui.journal = true, Some("cheats") => game.ui.cheat_menu = true, Some("console") => { game.console.open = true; game.exec("help"); game.exec("give vainamoinen_gun"); } Some("picker") => { game.ui.picker = true; game.brush.build = true; } Some("pause") => game.ui.paused = true, _ => {} }
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
        let t0 = std::time::Instant::now();
        for k in 0..600 { game.tick(); if k == 100 { game.exec("spawn draugr 5"); } }
        let ms = t0.elapsed().as_secs_f64() * 1000.0 / 600.0;
        print!("[{:.2} ms/tick] ", ms);
        println!("{:<13} spawn={:?} enemies={} players_alive={} cells_nonempty={}", planets::PLANETS[pi].id, game.gen.spawn, game.enemies.len(), game.alive_players().len(), game.world.cells.iter().filter(|c| c.el != 0).count());
    }
    // bot playthrough on every planet: two bots run east, jump, jetpack and shoot
    for pi in 0..planets::PLANETS.len() {
        game.land(pi, Mode::Campaign, true); game.cheats.inf_ammo = true; game.exec("give vainamoinen_gun");
        let (mut maxx, mut deaths0) = (0.0f32, 0);
        let t0 = std::time::Instant::now();
        for k in 0..4000u32 {
            for i in 0..game.players.len() {
                let target = game.enemies.iter().filter(|e| (e.body.x - game.players[i].body.x).abs() < 220.0).min_by(|a, b| (a.body.x - game.players[i].body.x).abs().partial_cmp(&(b.body.x - game.players[i].body.x).abs()).unwrap()).map(|e| e.body.center());
                let p = &mut game.players[i]; let blocked = p.body.blocked;
                p.ctl.right = target.is_none(); p.ctl.left = false; p.ctl.up = blocked || (k / 90) % 7 == 0; p.ctl.up_pressed = blocked && p.body.on_ground;
                if let Some((tx, ty)) = target { p.ctl.fire = true; p.ctl.fire_pressed = k % 5 == 0; if i == 0 { p.ctl.aim_world = Some((tx, ty)); } else { p.aim_rel = 0.0; p.ctl.aim_world = Some((tx, ty)); } p.ctl.nade_pressed = k % 200 == 0; } else { p.ctl.fire = false; p.ctl.aim_world = Some((p.body.x + 50.0, p.body.y - 14.0)); }
                p.ctl.reload = k % 400 == 0;
            }
            game.tick();
            maxx = maxx.max(game.players[0].body.x);
            if k % 1000 == 999 { for i in 0..game.players.len() { if !game.players[i].alive { deaths0 += 1; } } }
        }
        println!("bot {:<13} reached x={:>4.0}  kills={:<3} deaths={} bullets={} particles={} [{:.0} ms]", planets::PLANETS[pi].id, maxx, game.kills, game.players.iter().map(|p| p.deaths).sum::<u32>(), game.bullets.len(), game.particles.len(), t0.elapsed().as_millis());
        let _ = deaths0;
    }
    // idle-survival diagnostics
    for pi in [0usize, 2, 3] { game.land(pi, Mode::Campaign, false); game.enemies.clear(); let mut min_hp = 100.0f32; for _ in 0..900 { game.tick(); min_hp = min_hp.min(game.players[0].hp); } println!("idle no-enemies {}: min hp {:.1}", planets::PLANETS[pi].id, min_hp); }
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
    // powder interaction: explosions carve
    game.exec("boom 14");
    // ---- mission chain on Midgard
    game.land(0, Mode::Campaign, false); game.enemies.clear(); game.save.completed.clear(); game.init_missions();
    game.cheats.god = true;
    let shards0 = game.save.shards;
    let goal = game.gen.zone("goal").unwrap().clone(); { let b = &mut game.players[0].body; b.x = goal.center().0; b.y = goal.y1 as f32 - 2.0; }
    for _ in 0..5 { game.tick(); }
    assert!(game.runs[0].done, "m1 reach");
    let basin = game.gen.zone("basin").unwrap().clone();
    for y in basin.y0 + 4..basin.y1 { for x in basin.x0..basin.x1 { if game.world.get(x, y) == elements::EMPTY { game.world.set(x, y, elements::WATER); } } }
    for _ in 0..60 { game.tick(); }
    assert!(game.runs[1].done, "m2 fill (progress {})", game.runs[1].progress);
    let arena = game.gen.zone("arena").unwrap().clone(); { let b = &mut game.players[0].body; b.x = arena.center().0 - 10.0; b.y = arena.y1 as f32 - 2.0; }
    for _ in 0..30 { game.tick(); }
    assert!(game.boss_spawned && game.enemies.iter().any(|e| enemy::is_boss(e.def)), "boss spawn");
    for k in 0..900 { let p = &mut game.players[0]; if p.hp < 50.0 { p.hp = 100.0; } game.exec(if k == 5 { "killall" } else { "echo" }); game.tick(); if game.runs[2].done { break; } }
    assert!(game.runs[2].done, "m3 boss");
    assert_eq!(game.save.shards, shards0 + 1);
    println!("missions: midgard chain OK (shards {}, silver {})", game.save.shards, game.save.silver);
    // ---- burn mission on Muspell
    game.land(2, Mode::Campaign, false); game.enemies.clear(); game.save.completed.clear(); game.init_missions(); game.cheats.god = true;
    let nest = game.gen.zone("nest").unwrap().clone(); println!("nest fuel {}", nest.initial);
    for y in nest.y0..=nest.y1 { for x in nest.x0..=nest.x1 { let e = game.world.get(x, y); if e == elements::WOOD || e == elements::PLANT { game.world.ignite(x, y); } } }
    game.runs[0].done = true; game.runs[0].progress = 1;
    for _ in 0..1500 { game.tick(); if game.runs[1].done { break; } }
    assert!(game.runs[1].done, "burn mission (progress {})", game.runs[1].progress);
    println!("missions: burn OK");
    // every imported Soldat arena must load and give valid spawns
    let n = game.assets.arenas.len(); let mut bad = 0;
    for i in 0..n {
        game.land_arena(i, Mode::Versus, true);
        for _ in 0..30 { game.tick(); }
        for p in &game.players { if p.body.collides(&game.world, p.body.x, p.body.y) { bad += 1; println!("arena {} spawn inside terrain at ({:.0},{:.0})", game.assets.arenas[i].stem, p.body.x, p.body.y); } }
    }
    println!("arenas: {} loaded, {} bad spawns", n, bad);
    assert!(n >= 90, "expected the imported Soldat arenas in assets/maps");
    println!("selftest OK");
}

/// Writes assets/data/elements.txt (editable colour overrides) and docs/MISSIONS.md.
fn dump_docs() {
    let mut e = String::from("# element colour overrides: key r g b  (edit and restart; delete a line to use the built-in colour)\n");
    for d in elements::ELEMENTS.iter().skip(1) { e.push_str(&format!("{} {} {} {}\n", d.key, d.color.0, d.color.1, d.color.2)); }
    let _ = std::fs::create_dir_all("assets/data"); std::fs::write("assets/data/elements.txt", e).unwrap();
    let mut m = String::from("# nordSoldPow - Mission list (generated by `--dump-docs`)\n\nMain chain: m1 -> m2 -> m3 (m3 completion recovers a Sampo shard). Optional missions (o1, o2) are always available.\n");
    for p in planets::PLANETS.iter() {
        m.push_str(&format!("\n## {} ({}) - {}\n\nGravity {:.2}g, hazard {:?}, boss: {}.  Enemies: {}\n\n| id | title | objective | reward |\n|---|---|---|---|\n", p.name, p.analog, planets::system_name(p.system), p.gravity, p.hazard, p.boss.unwrap_or("none"), p.enemies.join(", ")));
        for i in missions::for_planet(p.id) { let x = &missions::MISSIONS[i]; m.push_str(&format!("| `{}` | {}{} | {} | {} silver{} |\n", x.id, x.title, if x.optional { " *(optional)*" } else { "" }, missions::describe(&x.obj), x.silver, x.weapon.map(|w| format!(", weapon `{}`", w)).unwrap_or_default())); }
    }
    m.push_str("\n## Weapons\n\n| key | name | type | dmg | tier |\n|---|---|---|---|---|\n");
    for w in weapons::WEAPONS.iter() { m.push_str(&format!("| `{}` | {} | {:?} | {} | {} |\n", w.key, w.name, w.fire, w.dmg, w.tier)); }
    m.push_str("\n## Enemies\n\n| key | name | hp | ai |\n|---|---|---|---|\n");
    for en in enemy::ENEMIES.iter() { m.push_str(&format!("| `{}` | {} | {} | {:?} |\n", en.key, en.name, en.hp, en.ai)); }
    std::fs::write("docs/MISSIONS.md", m).unwrap();
    println!("wrote assets/data/elements.txt and docs/MISSIONS.md");
}
