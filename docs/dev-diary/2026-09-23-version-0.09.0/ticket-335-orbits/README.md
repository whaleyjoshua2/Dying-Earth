# Orbits: the maps, the doors and the pictures

Ticket [#335](https://github.com/whaleyjoshua2/Dying-Earth/issues/335) on version 0.09.0. The engine
half -- `Orbit::Low | Orbit::Slot(n)`, a transit that names its orbit, `Order::ChangeOrbit`, Orbital
Control of low orbit, a Battery covering its own orbit, Battle parties per orbit, Bombard holding the
orbit it acts in -- is commit `7450d1a`. This file is the **interface** half: rule **R8** of
`SPEC.md` beside it, the maps and every door and hover that used to say "at the Body". `sweeps/`
holds the computer lane's closing sweep.

| picture | what it shows |
|---|---|
| [`orbits-rings-mars.png`](orbits-rings-mars.png) | `shot:orbits-rings orbits:1 window:1600x1000`. **Low orbit's own ring**, innermost and pale blue-white, hugging the globe and named *low orbit* on its own right-hand side, crossing the Orbital Slots' rings (Mars Base Camp's teal, Ares's orange, Hermes's empty grey). Every stack is drawn **on the ring it sits in**: four glyphs on the low-orbit ring (Custodians, Prospectors, Arkwrights, Archivists) and two on Mars Base Camp's (Custodians, Prospectors). The orbit band names each: *Custodians: 1 Ship(s), strength 3, in low orbit* / *Custodians: 1 Ship(s), strength 0, at Mars Base Camp* / *Prospectors: ... at Mars Base Camp*, then the two stations, then *Orbital Control of low orbit: Custodians*. The Mars card reads *Mars Base Camp ... blockaded by the Prospectors: producing nothing* -- a Blockade of a station's own ring. |
| [`orbits-solar.png`](orbits-solar.png) | `shot:orbits-rings orbits:1 window:1600x1000`, the Solar System Map of the same run. Each stack label says the orbit: at Earth *Custodians x1 str 0, at ISS*; at Mars *Archivists x1 str 0, in low orbit*, *Arkwrights x1 str 0, in low orbit*, *Prospectors x2 str 3, across 2 orbits*, *Custodians x2 str 3, across 2 orbits*, and the flag *Orbital Control of low orbit: Custodians*. |
| [`orbits-transits-mars.png`](orbits-transits-mars.png) | `shot:orbits-transits orbits:1 stack:1 scroll:transits window:1400x1100`. The Transits row, now **a line per destination orbit**: *To Earth: 8 turn(s), 43 Fuel each from the tank, whichever orbit it ends in*, then `Earth, low orbit`, `Earth, at ISS`, `Earth, at Tiangong`, `Earth, at Axiom`, `Earth, at Orbital Reef`, `Earth, at Starlab`, each with its own *All N that can* and per-Ship buttons; the same under the Moon, Phobos and Deimos. Earth's are greyed (43 Fuel against a 30 tank), Phobos's and Deimos's live at *All 2 that can*. |
| [`orbits-door-mars.png`](orbits-door-mars.png) | `shot:orbits-door orbits:1 stack:1 scroll:orbits window:1400x1100`. **The `ChangeOrbit` door**: *Moving between two orbits of Mars costs 1 Fuel from the Ship's own tank, and lands with the transits, before the Battles*, then `To Mars, low orbit (1 Fuel)` offering only the Endeavour (the Valiant is already there), `To Mars, at Mars Base Camp (1 Fuel)` only the Valiant, and `To Mars, at Ares (1 Fuel)` and `To Mars, at Hermes (1 Fuel)` offering both at *All 2 that can*. Under it the Tanks rows say where each Ship is -- *TSV Valiant: 30/30 Fuel, in low orbit*, *TSV Endeavour: 30/30 Fuel, at Mars Base Camp* -- with both Refuel buttons greyed, one for the wrong orbit and one for the Blockade. |
| [`orbits-battle-mars.png`](orbits-battle-mars.png) | `shot:orbits-battle orbits:1 battle:1 window:1600x1000`. **Two Battle marks at one Body**, each beside the orbit that fought: one on the pale low-orbit ring, one on Mars Base Camp's teal ring (enlarged and checked pixel by pixel before this was written). The band carries a row for each: *A Battle in low orbit last turn: the Custodians attacked* and *A Battle at Mars Base Camp last turn: the Custodians attacked*, both clickable through to the Report. *Orbital Control of low orbit: Custodians* -- one seat's warships left down there. Under the band, clear of it, *Chryse Planitia: empty* and its four yields: the band grew a row per orbit this ticket and began landing on the globe's site labels, so a label whose block would fall inside the band is now dropped below it, whole. |
| [`orbits-lift-earth.png`](orbits-lift-earth.png) | `shot:orbits-lift orbits:1 emigrants:8 select:eastasia panel:0 "tip:a lift from a Launch Site reaches" window:1400x1600`. **A door shut because the Ship is in the wrong orbit, saying so in the engine's words.** China's card: `Send 2 to ISS over Earth by lift` live, and `Send 4 to TSV Beagle (at ISS)` greyed, its hover reading *a lift from a Launch Site reaches Earth, low orbit alone*. The button's own face names the orbit the Ship is in, so the refusal is legible before it is hovered. |

## What was built

- **The Body Surface Map draws low orbit.** `orbit_rings_on_globe` loops `Game::orbits_of` rather
  than the slot count: low orbit at 1.035 globe radii (the slots keep ticket #151's 1.08 + 0.04n to
  the pixel), on a shallower plane of its own, in `LOW_ORBIT` pale blue-white at a 2.2 stroke where a
  slot's is 1.2, with its name on the ring. **Every stack in any orbit is drawn on that orbit's
  ring**, one glyph a Faction in its colour with its count, each a hotspot that selects the stack;
  where the globe stands in front of a stack's place on the ring it is carried round until it is in
  sight. The old lone `warship_in_slot` glyph is gone -- the per-Faction stacks say strictly more.
- **The Battle mark per orbit.** `battle_last_turn_at(ReportPlace::Orbit(body, orbit))` is read once
  per ring, so two fights at one Body put two marks up, each on its own ring, each clicking through
  to its record. The Body's label on the Solar map keeps the single mark ticket #317 gave it.
- **The orbit band** is a `BandRow` struct now, not a tuple: a Faction's Ships are a row **per orbit**
  (`Custodians: 1 Ship(s), strength 3, in low orbit`), a Battery's row names the orbit it covers, the
  Control row says *Orbital Control of low orbit*, and there is a Battle row per orbit that fought,
  each carrying the record index its hotspot opens.
- **The Solar System Map** stack label says the orbit, or *across N orbits* where the Faction's
  Ships at that Body are spread; the Orbital Control flag says *of low orbit*.
- **Choosing an orbit.** The Transits row lists every destination orbit as its own line with the same
  Fuel, filling `Order::Transit { slot }` -- the three sites that passed `slot: None` were why no
  human player had ever been able to blockade. The Solar map's right-click still sends the stack to
  the Body's **low orbit**, which is what `slot: None` has always meant; on a Body Surface Map, with
  the player's stack there selected, a right-click on a station's glyph moves it to that station's
  orbit and a right-click on the globe to low orbit, cancelling on a second click as every other
  right-click does (`orbit_right_click`).
- **The `ChangeOrbit` door**: a block on the Ship stack's card, one line per other orbit at the Body
  with the Fuel on its face, an *All N that can* and a button per Ship not already there, the hover
  naming where that Ship is now, and the engine's refusal on a greyed button.
- **Words**, everywhere a rule used to be said of a whole Body: the Blockade stance hover (*it shuts
  the orbit the stack sits in and no other*), Intercept's (*only what arrives into the orbit the
  stack sits in*), the starved-Colony hover, the Battery tile's rules (which now name the one orbit
  that Battery covers and what denying it is worth, differently for a station and for the ground),
  the Bombard hover (*the orbit you are in is the orbit you must hold*, with the Battleship's own
  orbit named), the Body card's station paragraph, the Passage Accord's line, the Tanks rows and each
  Ship's line on the stack card.
- **Building aids**: `orbits:1` fills three of Mars's four orbits at once -- two stations, seat 0's
  Frigate in low orbit and its Colony Ship at its own station's ring, seat 1's Frigate blockading
  that ring -- and parks a Colony Ship of seat 0's at the ISS's ring over Earth for the shut lift
  door; it runs before `battle:1`, so the two together fight a Battle in each of two orbits.
  `scroll:transits` and `scroll:orbits` scroll the stack card to a block, which a headless capture
  cannot do with a scrollbar.

## Looked at

All six pictures above, opened and read before this was written, two of them enlarged. Four things
were caught by looking and fixed:

1. Low orbit's first plane (incline 0.95, node 2.35) stood nearly **edge-on** from this camera and
   drew as two thin arcs -- the exact fault ticket #151 redrew the slots to cure. Reset to 0.40 and
   0.60, and the ring now reads as an open ellipse.
2. The *low orbit* name label, placed at a fixed angle, landed **under the orbit band**; moved to the
   ring's lowest point it landed under a station's glyph. It now sits high on the ring's right-hand
   side, where neither the band nor the slots' glyphs reach.
3. Of four stacks in Mars's low orbit the first picture drew **one**: the other three sat behind the
   globe. A stack whose place on the ring is hidden is now carried round the ring until it is in
   sight, and all six stacks in the picture are drawn.
4. A pre-existing defect the door picture exposed: the *Load N Colonists from ...* button read the
   Body's **ground**-slot table with the Colony's slot number, so a station in Orbital Slot 0 was
   labelled with the ground site numbered 0 -- *Load 2 Colonists from Olympus Mons* for a station
   called Mars Base Camp. It reads `place_name` now.

Not photographed: the right-click itself (headless, no pointer, as ticket #323 found); the Change
orbit refusals other than an empty tank, which are the engine's own words on a greyed button.

## Left undone

`assets/data/modules.toml` still says of the Battery that *while it stands no rival holds Orbital
Control* and that it *fires at rival Ship stacks in orbit here* -- true only of a ground Colony's
Battery since this ticket, and the whole-Body reading is now false for a station's. The file was
outside this lane's hands; the two lines are `modules.toml:291` (the comment) and `:302` (`does`).
The card's own hover beside it says the narrowed rule correctly.
