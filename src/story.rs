//! Story text: intro, act breaks, ending, rune lore. ASCII only (default font).
pub const INTRO: &[(&str, &str)] = &[
    ("", "Once, the gods sang the world into shape. Odin from his high seat, Vainamoinen from his kantele, Ilmarinen at his anvil."),
    ("", "Their greatest song was the SAMPO - a wonder-mill that grinds out grain, salt, gold... and the laws of matter itself."),
    ("", "Why sand falls. Why water flows. Why fire spreads. All of it, ground out slowly, by the Sampo."),
    ("", "Then Louhi of Pohjola stole it. Jormungandr thrashed in the black sea. The Sampo shattered into FOURTEEN SHARDS."),
    ("", "Now matter is unruly. Lava rains. Acid spreads. The dead walk. Ragnarok has a delivery date."),
    ("Odin", "That is why I hired you, Rune-Runner. Modern guns, a jetpack legally classified as a lawnmower, and no sense of self-preservation."),
    ("Vainamoinen", "Collect the shards from Sol's nine worlds, then cross the Aurora Gate to my star. Ilmarinen will reforge the Sampo at his forge."),
    ("Odin", "Press M for the map. F to interact. B to paint the world with your own hands. Try not to set Midgard on fire. Again."),
];
pub const ACT2: &[(&str, &str)] = &[
    ("Heimdall", "The Bifrost is open. I can hear the grass growing on Kalevala, and someone singing off-key."),
    ("Vainamoinen", "That was me. Welcome to the Paivatar system, Rune-Runner. Louhi's fortress waits in the north."),
];
pub const ENDING: &[(&str, &str)] = &[
    ("Ilmarinen", "The mold is poured, the last shard seated. Listen... the Sampo grinds again."),
    ("Vainamoinen", "Sand falls. Water flows. Fire spreads. The old song is sung."),
    ("Odin", "Ragnarok postponed. Also, I found my keys. They were in the other jacket."),
    ("", "THANK YOU FOR PLAYING nordSoldPow - by Crowelian 2026 + Sonnet. The sandbox is yours. Press ESC."),
];

pub fn rune_lore(planet: &str, n: usize) -> &'static str {
    const POOL: [&str; 8] = [
        "RUNE: 'Fire loves wood, water loves fire's end.'",
        "RUNE: 'Dig deep: dwarves hide what giants cannot reach.'",
        "RUNE: 'Gunpowder is patient. Do not be.'",
        "RUNE: 'Acid eats all but gold and wall.'",
        "RUNE: 'Plants drink water and grow toward the sky.'",
        "RUNE: 'Clone a thing, and the thing will never end.'",
        "RUNE: 'Ice melts. Lava cools. Nothing stays.'",
        "RUNE: 'A hundred small grains make a mountain fall.'",
    ];
    let seed = planet.bytes().fold(0usize, |a, b| a.wrapping_mul(31).wrapping_add(b as usize));
    POOL[(seed + n * 3) % POOL.len()]
}

pub fn arrival(planet: &str) -> &'static str {
    match planet {
        "midgard" => "Midgard. Green hills, burial mounds, and Draugr who forgot they were dead.",
        "mani" => "Mani. Low gravity - your jetpack will love it.",
        "muspell" => "Muspell. Embers fall like rain. Surtr's forge-smoke fills the sky.",
        "vanaheim" => "Vanaheim. Freya's gardens, drowned in acid. Do not lick anything.",
        "jotunheim" => "Jotunheim. The giants are home. They are not fond of visitors.",
        "asgard" => "Asgard. Sky islands and Bifrost light. Watch your fuel.",
        "niflheim" => "Niflheim. Eternal snow. Fire is your friend.",
        "svartalfheim" => "Svartalfheim. Runes light the caves. Dynamite sleeps in the walls.",
        "helheim" => "Helheim. A black ocean of oil. Hel's court awaits.",
        "kalevala" => "Kalevala. Birches, lakes, and the smell of a sauna.",
        "pohjola" => "Pohjola. Louhi's frozen north. She is expecting you.",
        "tuonela" => "Tuonela. The river of the dead. Be polite.",
        "ukkola" => "Ukkola. Ukko's sky. Lightning loves metal.",
        "sampola" => "Sampola. Ilmarinen's forge world. The finale starts here.",
        _ => "A new world.",
    }
}
