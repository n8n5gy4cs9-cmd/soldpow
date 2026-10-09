# nordSoldPow - Lore & Story

*Modern-day Nordic myth: Viking sagas + the Finnish Kalevala, with assault rifles, jetpacks and a falling-sand universe.*
(In-game text is ASCII: Jotunheim, Muspell, Vainamoinen...)

## Premise
In the present day the Nine Worlds are not a tree - they are **our Solar System**, and the old gods never left; they
retired to run private security firms. **Sol** (the Sun) is the great hearth. Across the dark lives a *second* sun,
**Paivatar**, the Kalevala star, with five worlds of its own, where Vainamoinen, Ilmarinen and Lemminkainen
still argue about whose turn it is to sing the world into shape.

The two pantheons were bound together by the **Sampo** - Ilmarinen's wonder-mill that grinds out grain, salt and gold, and
(it turns out) holds the laws of matter themselves: why sand falls, water flows, fire spreads. When the witch-queen
**Louhi of Pohjola** stole it and the sea-serpent **Jormungandr** thrashed in Midgard's oceans, the Sampo **shattered into 14 shards**,
one for every world. Without it matter becomes *unruly*: lava rains, acid pools spread, the dead walk (the **Draugr**
in Norse space, the **Kalma** ghosts in Kalevala space).

Odin (retired, eyepatch, now runs "Huginn & Muninn Logistics") and Vainamoinen (ancient singer, plays a kantele that
also fires 5.56 mm) hire **you** - a mercenary the sagas call the **Rune-Runner** - to recover the shards, reforge the Sampo at
Ilmarinen's forge on **Sampola**, and stop Ragnarok from landing in Q4.

## Protagonist
**Rune-Runner** (P1, blue scarf) and **Her Shadow / Kaiku** (P2, red scarf). Jetpack forged by Ilmarinen, "legally a
lawnmower". Wields modern guns blessed (cursed?) with god names.

## Allies (voice of briefings)
| Name | Role |
|---|---|
| Odin | Employer. Cryptic. Always loses his keys. |
| Vainamoinen | Eternal singer, mentor, gives you `vainamoinen_gun` |
| Ilmarinen | The smith. Upgrades, forges. Brags about the Sampo. |
| Freya | Vanaheim quartermaster, supplies |
| Heimdall | Gatekeeper of Bifrost / Aurora Gate (can hear grass grow, hates loud music) |
| Lemminkainen | Reckless friend. Drops in via cheats of reality. Gives shotgun |
| Aino | Water spirit, Kalevala guide, springs heal |

## Enemies
| Kind | Where | Notes |
|---|---|---|
| Draugr | everywhere Norse | undead soldier, assault rifle |
| Berserker | Midgard, Jotunheim | fast melee axe, high HP |
| Eldjotunn | Muspell | fire giant trooper, immune to fire |
| Frostjotunn | Jotunheim, Niflheim | ice giant, immune to cold |
| Valkyrie | Asgard | flies, SMG |
| Hiisi | Kalevala | goblin raider, fast, knife |
| Kalma | Tuonela | ghost, slow, passes through powder |
| Louhi Guard | Pohjola | SMG, shield |
| Fenrir Pup | wolves | quadruped biter |
| Svartalf | Svartalfheim | dark elf sniper |

## Bosses (9)
Fenrir (Midgard), Surtr (Muspell), Ymir (Jotunheim), Jormungandr (Vanaheim swamps, sea serpent), Hel (Helheim),
Louhi (Pohjola), Tuoni (Tuonela), Rauta Golem (Sampola, Ilmarinen's rogue forge-bot), Ukko Thunder-Eagle (Ukkola).

## Worlds
### Sol system (Norse)
| id | Name | Real analog | Gist |
|---|---|---|---|
| midgard | Midgard | Earth | Home, green hills, lakes. Tutorial. |
| mani | Mani | Moon | Low gravity regolith, salt flats, Draugr in suits |
| muspell | Muspell | Mercury | Lava seas, ember rain. Surtr's smithy |
| vanaheim | Vanaheim | Venus | Acid swamps, jungle, Freya's gardens |
| jotunheim | Jotunheim | Mars | Red rock, frozen caverns, giants |
| asgard | Asgard | Jupiter | Sky-fortress on gas clouds, Bifrost gate |
| niflheim | Niflheim | Saturn | Eternal snow & ice, whiteouts |
| svartalfheim | Svartalfheim | Uranus | Dark caves, glowing runes, dwarven forges |
| helheim | Helheim | Neptune | Black oceans, dead realm, Hel's court |

### Paivatar system (Kalevala)
| id | Name | Gist |
|---|---|---|
| kalevala | Kalevala | Birch forests, lakes, saunas (healing). Vainamoinen's home |
| pohjola | Pohjola | Frozen north, Louhi's fortress |
| tuonela | Tuonela | Black river, swan of Tuonela, land of the dead |
| sampola | Sampola | Ilmarinen's forge world: metal, lava, gunpowder |
| ukkola | Ukkola | Storm-sky islands; Ukko's thunder (lightning events) |

## Acts
1. **Act I - The Shards of Sol**: Midgard, Mani, Muspell, Vanaheim. Learn the controls and the elements.
2. **Act II - Bifrost**: Jotunheim, Asgard, Niflheim, Svartalfheim, Helheim. The **Bifrost-Aurora Gate** at Asgard opens
   once 4 Norse worlds are cleared.
3. **Act III - The Singing Star**: Kalevala system: Kalevala, Pohjola, Tuonela, Ukkola, then Sampola (finale).
4. **Ending**: reforge the Sampo (fill the Sampo basin with gold + water + fire - a powder puzzle), gods toast, Odin finds his keys.

Mission lists are generated into `docs/MISSIONS.md` (`cargo run --release -- --dump-docs`).

## Hotspots
* **Spring (Aino's Well)** - heals.  * **Rune stone** - lore page + silver.  * **Chest** - ammo / weapon.
* **Forge (Ilmarinen)** - restocks all ammo.  * **Gate** - Bifrost / Aurora travel.  * **Sauna** - full heal + jetpack refuel (Kalevala).

## Weapon names (modern guns, god-blessed)
puukko (knife), hugin (pistol), munin (revolver), lemminkainen_smg, vainamoinen_gun (assault rifle, kantele-stock),
kullervo_shotgun, gungnir_sniper, mjolnir_rpg, gjallar_grenade (launcher), surtr_flame (flamethrower),
louhi_frost (ice gun), tuoni_acid (acid sprayer), ukko_thunder (lightning gun), ilmarinen_rail (railgun), sampo_gun (random element),
powder_gun (shoots selected element), fenrir_minigun, loki_trick (confusing pellets/mines).
