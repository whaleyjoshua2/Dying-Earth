# Dying Earth — version 0.06.0, the space version: the tank, Venus, the Observatory, the Solar Array, the Mass Driver, four cards retuned, every win behind a Tech

**Status:** the destination of the map [Map: version 0.06.0, the space version](https://github.com/whaleyjoshua2/Dying-Earth/issues/79). Every change here was decided on one of that map's tickets from the designer's list; each section names its ticket, and the ticket's resolution comment is the authority if this document and it ever disagree. Everything not amended here stands as written in [`first-playable.md`](first-playable.md), [`version-0.02.md`](version-0.02.md), [`version-0.03.md`](version-0.03.md), [`version-0.04.md`](version-0.04.md), [`version-0.05.md`](version-0.05.md) and [`version-0.05.5.md`](version-0.05.5.md).

**Numbers** live in `assets/data/`, one file per table; this document names the file beside each change.

**A rule marked "builder's call"** was settled in the build rather than on the ticket. Every one of them is gathered again in section 16 for the designer to veto.

**The designer's list**, as given: the Custodians' mothball doubling a space asset of the same kind; their Influence to 1.2; every Victory Condition behind a Tech; the Prospectors x1.2 on Ducats and 15% off the market; the Arkwrights 15% off Ships; the Archivists' 0.8 lifted for a research boon (settled mid-map as 1.5 on Earth and 1.75 off it); the Antarctic pop-up; S5 (a warming Earth fills the Ships, with deaths); S6 (a tank per Ship, filled only at a station); S7 (a discount where a Colony has Mines); S8 (a Solar Array scaled by distance from the Sun); S10 (the Trade Post's network); S11 and S12 (the Mass Driver and Venus); Habitats of eight with 1% Research per Colonist. The build ticket added an AI sweep at the designer's request: "make sure all their decision trees and weights make sense" (section 14).

---

## 1. The Observatory, and Habitats of eight

Decided on [The Observatory: a research Module for Colonies and Space Stations, and Habitats of eight](https://github.com/whaleyjoshua2/Dying-Earth/issues/80). Amends section 7 of the First Playable (Modules) and section 5 of version 0.03 (Research).

- **The Observatory is a Module: 28 Materials, two turns, 3 Energy upkeep, 2 Research a turn**, plus **one per cent per Colonist living at its Colony**, uncapped, times the Faction's Research multiplier and Public Science, rounded down; no Body yield and no output multiplier (`modules.toml`: the `observatory` row, `[observatory] research_per_colonist = 0.01`). It stands on a Colony, a Space Station or in Antarctica, and its Research counts toward the Research Lead as a Lab's does.
- **A Habitat holds 8 Colonists** (the Arkwrights 12; Expanded Habitats still +2). The Colony Ship is unchanged (`modules.toml`: the Habitat's `holds_colonists`).
- The AI builds one Observatory per Colony once it holds `observatory_colonists` (8; the Archivists 4) (`ai.toml`).
- Builder's calls: one Observatory per Colony on the AI's side; the station's refusal names what it holds.

## 2. The Archivists' card, and what "off Earth" means

Decided on [The Archivists' card: the output nerf lifted, Research +15%, and +25% from Observatories off Earth](https://github.com/whaleyjoshua2/Dying-Earth/issues/81). Amends section 3 of version 0.05.

- **The Archivists' output multiplier is 1.0** (was 0.8). **Their Research is x1.5 on Earth and x1.75 off it**, two flat figures (`factions.toml`: `research_multiplier`, `research_multiplier_off_earth`; the other three cards carry one figure for both sides). Provisional Findings unchanged.
- **A Space Station over Earth is off Earth; Antarctica is not.** One rule answers for every place that asks: Off-world Presence, Diaspora's Colonist count, where the Archive may stand, the 1.75, and the Research-off-Earth measurement. Earth is still not one of Diaspora's three Bodies (builder's call).
- The Observatory has its own AI weight (`ai.toml` `build_observatory`: 3, 8, 6 and the Archivists' 12).

## 3. The Custodians' card: Production Moved, and Influence 1.2

Decided on [The Custodians' card: a mothballed Facility doubles a Module of its kind off Earth, and Influence eases to 1.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/82). Amends section 4 of the First Playable (Factions) and section 6 of version 0.04 (mothballing).

- **Production Moved**: four pairs, Factory to Mine, Power Plant to Generator, Refinery to Refinery, Research Lab to Observatory. **While a Facility of theirs on Earth is mothballed, one Module of the paired kind off Earth makes double**, one for one, the most productive undoubled Module first, on its final figure; none without an idle Facility; a Restart, a Decommission or losing the state ends it (`factions.toml`: `mothball_pairs`, the Custodians only). A station over Earth is off Earth, Antarctica is not (section 2). Unrest per mothball unchanged; no new Unrest rule.
- **The Custodians' Influence multiplier is 1.2** (was 1.25) (`factions.toml`: `influence_multiplier`).
- The Colony card's Module line reads "doubled by an idle Factory on Earth"; the Income names it.
- The AI idles a Facility for an even-or-better trade (section 14) and never restarts one whose doubling stands.
- Builder's calls: "most productive" reads the Module's Materials, Energy, Fuel or Research figure, ties in Colony order.

## 4. The Prospectors' Ducats and market; the Arkwrights' Ships

Decided on [The Prospectors' Ducats at x1.2 and 15% off the market; the Arkwrights' Ships 15% cheaper](https://github.com/whaleyjoshua2/Dying-Earth/issues/83). Amends section 4 of the First Playable and section 5 of version 0.05.5.

- **The Prospectors' states pay x1.2 Ducats** on their own GDP income (Banks and Trade Posts keep the general x1.25), and **the market sells to them at x0.85 over the lot** (Materials, Fuel, Energy, and a building bought outright), rounded down after the whole price; Influence and selling untouched (`factions.toml`: `ducats_multiplier`, `market_multiplier`).
- **The Arkwrights' 20-Materials Colony Ship is retired for the common 30, and every Ship of theirs costs x0.85**: Colony Ship 25, Frigate 21, Carrier 25, Battleship 42 (`factions.toml`: `ship_materials_multiplier`; `colony_ship_materials` gone). Read from "adjust their carrier cost to match everyone else's then 15% off" (builder's call, on the resolution for correction).

## 5. Every Victory Condition waits on a Tech

Decided on [Every Victory Condition waits on a Tech](https://github.com/whaleyjoshua2/Dying-Earth/issues/84). Amends section 15 of the First Playable (Victory) and section 5 of version 0.03 (the Tech Tree).

- **Four new rung-3 Techs at 40 Research, one per Faction, each the gate for its Victory Condition and a real Tech for everyone** (`techs.toml`: `gate_for` on each): **Planetary Stewardship** (Society, after Green Consensus; the Natural Sink +1.0 ppm for the world), **the Extraction Charter** (Extraction, after Automated Refining; Mines x1.25), **Generation Ships** (Off-world Living, after Closed-Loop Colonies; a Colony Ship +2), **the Upload** (Society, after Public Science and Expanded Habitats; Labs and Observatories x1.25). Seventeen Techs in all.
- **The gate holds back the win only.** Every part accrues as before, the end-of-game ranking is unchanged, Provisional Findings opens no gate. The Victory panel says "needs Planetary Stewardship, not yet researched" on the first part's line; the Faction card says what it waits on; the Tech Tree borders each gate in its Faction's colour.
- The AI picks its gate as Research Lead once its first part is past half or from turn 24 with the road standing (`ai.toml` `[thresholds]`: `gate_pick_fraction` 0.5, `gate_pick_turn` 24; builder's timing).

## 6. The Antarctic founding Moment

Decided on [The Antarctic founding Moment says "off Earth"](https://github.com/whaleyjoshua2/Dying-Earth/issues/85). Amends section 3 of version 0.05.

- An Antarctic founding says **"their first Colony in Antarctica"** and **"their second Colony in Antarctica"**, counting Antarctic Colonies alone; the off-Earth phrase counts the other Bodies alone and reads "their second Colony off Earth" (`report.toml`: `first_antarctic_colony`, `more_antarctic_colonies`, `more_colonies`). Ordinals in words to twelfth, figures past it.

## 7. A warming Earth fills the Colony Ships

Decided on [A warming Earth fills the Colony Ships, and crowding costs lives on the way](https://github.com/whaleyjoshua2/Dying-Earth/issues/86). Amends section 8 of the First Playable (Ships).

- **A Colony Ship lifting at Earth may take one Colonist beyond its capacity for every full 0.2 C above +1.8, capped at four** (`units.toml` `[crowding]`: `above` 1.8, `step` 0.2, `per_step` 1, `cap` 4). The player chooses the load; the Load panel shows the safe and crowded figures and the risk. Loading from a Colony elsewhere takes the capacity alone (builder's call).
- **At arrival, each Colonist above the safe capacity dies with a chance of 5% times the extras aboard**, rolled once on the game's dice (`death_chance_per_extra` 0.05). A Report line and a new Moment, "Colonists lost in transit", ranked with a place changing hands. Deaths count for nothing else. The sea to Antarctica carries no crowd and rolls nothing.
- The AI lifts the crowd only while behind on Off-world Presence.

## 8. Every Ship carries its own tank

Decided on [Every Ship carries its own tank, filled from the Stockpile only at a Space Station](https://github.com/whaleyjoshua2/Dying-Earth/issues/87). Amends section 8 of the First Playable (Ships, Fuel) and section 6 of version 0.05 (the Mars window's Fuel).

- **Every Ship has a tank of 30** (`units.toml`: `tank` on the Colony Ship, Frigate, Battleship and Carrier). A Ship is built with its tank full, the Fuel paid at the yard in the build's cost.
- **A transit spends the tank**: the leg's Fuel after the Faction's and the Tech's multipliers and the off-window surcharge, and the Mass Driver's cut (section 12). A leg the tank cannot pay is refused: "the tank holds 5 Fuel of 30; this leg needs 20". Lifts from Earth stay free.
- **Refuel is an order** at a Body where the Ship's Faction holds a station: it fills the tank from the Stockpile as far as the Stockpile can pay (builder's call: a partial fill rather than a refusal); refused with no station there, a full tank or an empty Stockpile. A Colony without a station fills nothing.
- **A Ship is stranded** when its tank cannot pay the cheapest leg from where it stands and no station of its own is there; **a station of its own built in orbit there rescues it**. The Solar System Map's stack line reads "tank 20/30" and "STRANDED"; the Ship panel has a Tanks section.
- A Ship in a save from before this version loads with an empty tank (builder's call).
- The AI refuels at any station of its own whenever a tank is short and the Stockpile has Fuel, and flies no leg its tank cannot pay; the Fuel bank of ticket #57 now holds nothing but the Refuel.

## 9. Build it where you dig

Decided on [Build it where you dig: Modules cost less at a Colony with working Mines](https://github.com/whaleyjoshua2/Dying-Earth/issues/88). Amends section 7 of the First Playable (Modules).

- **A Module's price at a Colony is the row, times the Faction's multiplier, times x0.75 with one working Mine there or x0.6 with two or more**, rounded down, **never below half the row** (`modules.toml` `[in_situ]`: `one_mine` 0.75, `two_mines` 0.6, `floor` 0.5). A mothballed or unfinished Mine counts for nothing. Modules only, the Archive included; Ships and stations untouched; Antarctica included. The Colony card says so and every button carries the cut price.
- Builder's call: the floor is applied before rounding down.

## 10. The Solar Array

Decided on [The Solar Array: a Space Station Module whose Energy scales with distance from the Sun](https://github.com/whaleyjoshua2/Dying-Earth/issues/89). Amends section 7 of the First Playable and section 4 of version 0.02 (Space Stations).

- **The Solar Array is a station-only Module: 25 Materials, two turns, no upkeep, 6 Energy at Earth's distance, scaled by the inverse square of its Body's mean distance from the Sun** (`modules.toml`: the `solar_array` row with `station_only` and `sun_scaled`; the distance is the ephemeris `a`: Venus x1.91, the Moon Earth's, Mars and its moons x0.43), rounded to the nearest whole. Any number per station. Efficient Grids and the Solar Maximum lift it as a Generator; a Solar Storm turn silences it; Production Moved pairs Generators only. Refused on the ground: "a Solar Array stands only on a station".
- **A station holds a Shipyard, Habitats, Observatories, Solar Arrays and a Trade Post.**
- The AI offers it as an Energy producer, raised by the Energy-shortage bonus.
- Builder's calls: the sun factor is one fixed figure per Body; the Faction's general output multiplier applies.

## 11. The Trade Post pays for the shape of the empire

Decided on [The Trade Post pays for the shape of the empire](https://github.com/whaleyjoshua2/Dying-Earth/issues/90). Amends section 7 of the First Playable and section 4 of version 0.03.

- **A Trade Post pays 2 Ducats per Colonist of its Faction at its Body** (stations overhead included) **plus 3 for every other Body where the Faction holds a Colony or a station** (Earth counts for a station over it or any Nation State directed; Antarctica is Earth; Venus counts); no Body yield; the Faction's output multiplier applies (`modules.toml`: the Trade Post's `produces.amount` 2, `[trade_post] per_other_body = 3`; `factions.toml` retires `trade_post_base`). The card shows the arithmetic.
- **One Trade Post per Faction per Body**, on a station or the ground; a second is refused ("one per Body").
- The AI offers one at every Body it holds and has none on, worth half again with two Bodies held (builder's call).

## 12. The Mass Driver

Decided on [The Mass Driver on the Moon, Phobos and Deimos](https://github.com/whaleyjoshua2/Dying-Earth/issues/92). Amends section 7 of the First Playable and section 8 (transits).

- **The Moon, Phobos and Deimos are low-gravity Bodies** (`bodies.toml`: `low_gravity = true`).
- **The Mass Driver is a Module of 35 Materials, two turns, 4 Energy, one per ground Colony on a low-gravity Body, behind Efficient Transit** (`modules.toml`: the `mass_driver` row, `needs_tech`, `low_gravity_only`; refused with "a Mass Driver needs Efficient Transit first" or "stands only on the ground of a low-gravity Body").
- **Every leg the owner's Ships fly from that Body costs 4 Fuel less, after the multipliers, never below 1; each Mine at its Colony makes +1 Materials** after everything (so Production Moved doubles it too) (`[mass_driver] fuel_off = 4, fuel_min = 1, mine_bonus = 1`). Owners only. A mothballed driver does neither.
- The AI builds one at a low-gravity ground Colony with a Mine once the Tech stands, at half again the producer weight, and weighs a Mine beside a working driver by the ratio of its yield with the driver to without (the designer's instruction on the ticket; the ratio the builder's call).

## 13. Venus, a Body of orbits only

Decided on [Venus, a Body of orbits only](https://github.com/whaleyjoshua2/Dying-Earth/issues/93), on the research of [Venus's orbital elements, the Earth-Venus windows 2030 to 2035, and every Body's distance from the Sun](https://github.com/whaleyjoshua2/Dying-Earth/issues/91). Amends section 6 of the First Playable (Bodies) and section 6 of version 0.05 (the Launch Window).

- **Venus is the sixth Body: no Colony Slots, three Orbital Slots (Ishtar, Aphrodite, Lada), three turns and 16 Fuel from Earth at its Launch Window** (`bodies.toml`: the `venus` row; `ephemeris.toml`: Venus's `[[planet]]` row and `[transit_venus]`, the Hohmann flight 146.08 days, phase angles -54.03 and -36.03, the synodic period 583.92 days). **There are two Launch Windows now**, Mars's and Venus's, each on the real sky; Venus's fall at turns 9, 18 and 28. **No leg runs between Venus and the Mars system**: fly by Earth.
- **Everything at Venus is built and held in orbit.** A station at a Body with no Colony Slots is built from a Ship of one's own in orbit there (builder's call; elsewhere still from a Colony below); a Colony Ship lands only into a station's Habitats; Orbital Control is the whole contest. Colonists on a Venus station are off Earth (section 2), and a station there is Venus for Diaspora's three Bodies. The Solar Array reads x1.91 there.
- The drawing: a cloud globe made by the asset step (`examples/prep_assets.rs --venus`, deterministic, no dependency), a third ring inside Earth's on the Solar System Map at the real sky's longitude, a Venus Surface Map with the three Orbital Slots and no Colony Slot, the window tooltip over Venus.
- The AI raises a station at Venus when a Ship of its own is there, and its Colony Ships weigh Venus as a slot with Venus's own yields (section 14).

## 14. The build ticket: the AI sweep, the four-way balance, the climate cell

Decided on [Write the 0.06.0 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/94), with the designer's addition: "this time let's include an AI sweep and make sure all their decision trees and weights make sense".

### The AI's decision tree, as it stands

Each turn a seat's AI enumerates every order it could give as a **candidate** with a category, a base weight from its Faction's row in `ai.toml`, and three multipliers: the **gap** (x1 to x3 with how far the seat is behind the pace of the Victory part it is furthest behind on, applied to the categories that advance that part), a **threat** figure (Influence and Armies where a rival presses) and an **opportunity** figure (a Shipyard when it has none, a crowded lift when behind on Presence, a station at Venus). The candidates are sorted by score and taken greedily while each is legal and affordable, with three reserves: **Materials are held for a higher-scored build that four turns of income would bring within reach** (twelve turns on the Archive's path), **Ducats for three turns**, and **Fuel near a Mars window for the crossing** (now only the Refuel). Stances are taken last, one per stack. The Tech pick is the Faction's list in order, then its gate once its first part is past half or from turn 24, then whatever is cheapest. Every scored line is written to the game's log ("take", "skip", "save", "wait"), which is how the sweep read it.

### What the sweep changed

Each change was seen red first and is pinned by a test; the balance batches were rerun after each.

1. **A loaded Colony Ship now disembarks into a Colony or station of its own with room at the Body it stands at.** The branch existed since ticket #46 but sat inside the at-Earth case and then asked for a Body that was not Earth, so it could never run: a second load never joined a Colony and nobody ever lived on a Venus station. **Over Earth the landing is a foothold at half weight with no gap, like Antarctica's**, taken when the Ship cannot go anywhere better. Offered at full weight the AI parked every load on the ISS's Habitats (they count for Presence since section 2) and Mars went unfounded: 0 to 9 seeds of 20 across the eight seatings, from 18 to 20 in 0.05.5. That batch is kept as `sweep/balance-iss-parked.txt`.
2. **Venus is worth a slot with Venus's own yields** to a Colony Ship choosing a Body (it was a flat 1), so it competes with the Moon and Mars on one scale.
3. **The Custodian AI idles a Facility for an even trade** (its output equal to the Module's undoubled figure), since the Emissions leave Earth with the output; it was strictly-better only.
4. **A Body whose leg from Earth no tank could pay is not a destination.** Mars off its window asks up to 47 Fuel of a 30 tank; the AI named it as its one choice, the Transit was refused at the check, and the Ship sat at Earth with its load. The Body the Ship stands at is no longer listed as a destination either.
5. **A Module off Earth that an idle Facility of the seat's could double is worth twice its base** (Production Moved, section 3), while the seat holds more Facilities of the paired kind than Modules already doubled. Until this the Custodian AI never built a Module off Earth in eight batches of twenty seeds: Earth's Facilities outscored them at the same base and the Materials reserve starved the rest, every turn's log reading "save 6.0 build Mine at Valles Marineris on Mars (holding Materials for build Power Plant in Europe)". Production Moved now fires: a median 30 to 55 doubled Module-turns a game for the Custodian seat in the five seatings where the world holds.
6. The `sim` example gains `--start=<state>`, so a logged game can be played from any seat at the table.

### What the sweep found and left

- **The Materials reserve** (ticket #56) holds everything below the first unaffordable candidate; with Earth's Facilities scored above off-Earth Modules, a Colony off Earth can stand bare for a whole game at +58 to +114 Materials a turn of income. The Production Moved weight lifts the Custodians' Modules over it; the other Factions' off-Earth Modules still sit under it.
- **The Prospector AI never flies to Venus** in twenty logged seeds from either start; the Archivist and Arkwright AIs do, raise the station on arrival and land (section 15's figures). The build picture shows the other case: two Prospector Colony Ships at Venus with twelve aboard, stranded until the station they are holding Materials for goes up.
- **Efficient Transit is on no Faction's Tech list**, so the Mass Driver is built only where an AI researches the whole tree (the Archivists' seatings: 13 and 26 drivers over twenty seeds; 0 to 5 elsewhere).
- **The Arkwright AI's forward station** still goes up after its Ships have flown (ticket #87's finding); with the payable-leg rule almost nothing strands (4 Ships at the end over 160 games).

### The four-way balance (`sweep/balance.txt`)

Eight batches of twenty seeds of four-seat `simulate`, the climate cell `ppm_step` 300 at Sink 6, permafrost and sink-after at +4.0 as 0.05.5 set them:

| Seat 0 | Wins by seat | Collapses (median turn) | End Temperature | Colonists off Earth (median) | Mars founded (seeds) | Venus stations / Colonists there | Custodian gate (seeds, median turn) | Doubled Module-turns (Custodians, median) |
|---|---|---|---|---|---|---|---|---|
| Custodians from East Asia | nobody | 20/20 (19) | +3.06 | 16 | 6 | 0 / 0 | 2, turn 28 | 0 |
| Custodians from Europe | Custodians 19 | 1/20 (20) | +2.70 | 31 | 19 | 0 / 0 | 19, turn 27 | 55 |
| Prospectors from East Asia | Custodians 2 | 18/20 (20) | +3.05 | 12 | 20 | 0 / 0 | 2, turn 25 | 0 |
| Prospectors from Europe | nobody | 20/20 (22) | +3.06 | 26 | 13 | 0 / 0 | none | 0 |
| Arkwrights from East Asia | Custodians 20 | 0/20 | +2.73 | 42 | 19 | 1 / 0 | 20, turn 28 | 30 |
| Arkwrights from Europe | Custodians 20 | 0/20 | +2.47 | 40 | 18 | 0 / 0 | 20, turn 27 | 39 |
| Archivists from East Asia | Custodians 20 | 0/20 | +2.68 | 43 | 20 | 15 / 69 | 20, turn 28 | 31 |
| Archivists from Europe | Custodians 15, Archivists 1 | 4/20 (34) | +2.46 | 73 | 18 | 28 / 218 | 20, turn 27 | 33 |

**Over 160 games the Custodians win 96, the Archivists 1, the Prospectors and Arkwrights none; 63 Collapses.** 0.05.5 measured 103, 1, 3 Arkwright wins and 53 Collapses. The other Factions' gates complete in about half the seeds only where the Archivists research the whole tree (9, 10 and 9 of 20 from Europe, at turns 18 to 28); the Archive is completed in 8 of 20 there (median turn 17) and nowhere else. The Fund ends at a median 437 and 481 in the Prospectors' own seatings, short of 750 in every seed. Refuel orders 41 to 165 a batch; Solar Arrays 96 to 383 standing at the end over a batch, nearly all over Earth; Trade Posts 0 to 67; Mass Drivers 0 to 26; Colonists lost in transit 101 over the 160 games, nearly all the Custodians' and Prospectors'; Research made off Earth a median 0 for every seat but the Archivists from Europe (12).

### The climate cell, reported and not retuned

**The cell holds where the Custodian AI holds its Scrubbers and fails where it does not.** From Europe the Custodians end at 560 Scrubbers over the batch and the world collapses once in twenty; from East Asia they end at 10 and it collapses every time at turn 19, and both Prospector seatings, with the Custodian AI beside them at 86 and 39 Scrubbers, collapse in 18 and 20 of 20. That is the Influence 1.2 finding of section 3 seen across every seating: East Asia's Allotment steps from 17 to 16 and the Custodian AI's Influence game turns on those steps. The Collapses rose from 53 to 63 of 160 and the Custodians' wins fell from 103 to 96; `ppm_step` 300 at Sink 6 stands, as the map's out-of-scope line says, and whether to move it, the Allotment or the Influence figure is the designer's.

## 15. Acceptance for 0.06.0

- `cargo clippy --release --workspace --all-targets -- -D warnings` clean; 229 formula tests and 6 save tests pass; every new rule was seen red first or, where the rule landed before its test could compile, mutated and watched to fail, as each ticket's build comment records.
- Eight batches of twenty seeds of four-seat `simulate` reporting the four-way win split, the Collapse rate, each gate's completion turn, Research made off Earth, stations off Earth, Ships stranded, Venus stations and Colonists, Trade Posts and Mass Drivers, Colonists lost in transit and Colonists off Earth: section 14 and `docs/dev-diary/2026-09-11-version-0.06.0/sweep/balance.txt`.
- `shot:` pictures in `docs/dev-diary/2026-09-11-version-0.06.0/`, one or two per ticket, the Solar System Map with Venus's ring and a tank gauge among them; every one opened before it was written about; none taken on the designer's desktop.
- `CONTEXT.md` amended: Fuel, Tank, Refuel, the Stockpile, the Space Station, the Observatory, the Solar Array, the Mass Driver, the Trade Post, Crowding, Build Where You Dig, Production Moved, the Body count (six), the Launch Window (two), the Module count (twelve), Off-world Presence, Diaspora, the Tech Tree, the Victory Condition, the Colony Ship, the Moment, Research, and the four Faction entries.
- The README and the playtest note at version 0.06.0; the kit in `dist/dying-earth-0.06.0/` and `dist/dying-earth-0.06.0-playtest.zip`; the `release-kits` workflow run on the branch for the Linux and Windows kits; the pull request from `version-0.06.0` against `main`.

## 16. Every builder's call, gathered

For the designer to veto, each on its ticket:

- The Observatory: one per Colony on the AI's side (#80).
- Earth is not one of Diaspora's three Bodies though a station over it is off Earth (#81).
- Production Moved's "most productive" reads the Module's figure, ties in Colony order; the AI's trade test compares the Facility's own figure with the Module's undoubled one, now even-or-better (#82, #94).
- "Carrier cost" read as the Colony Ship; the market discount taken over the lot so whole-Ducat prices survive (#83).
- The AI's gate timing (half of its first part, or turn 24); two gates in Society; the held-back line reuses the Archive's (#84).
- The crowd applies to a lift at Earth only; the safe capacity is read at arrival; the Moment ranked with a place changing hands (#86).
- The Refuel fills as far as the Stockpile can pay; the transit button reads "(free)" with the tank's price above it; a pre-0.06.0 save loads with empty tanks; the Refuel is exempt from the Fuel bank (#87).
- The in-situ floor applies before rounding; the AI's weights untouched by the discount (#88).
- The sun factor is one fixed figure per Body; the output multiplier applies to the array; the AI raises it through the Energy-shortage bonus (#89).
- A Trade Post over Earth counts Earth as its own Body; the AI values it x1.5 with two Bodies (#90).
- The driver's cut reads on the seat's figure; a mothballed driver does nothing; the AI's Mine bonus is the yield ratio (#92).
- Three turns at the Venus window (146 days rounded up as Mars's 259 is); the station foothold at a slotless Body is any Ship of one's own; the Venus ring at 0.72 of Earth's; the cloud map is deterministic sines (#93).
- The sweep's five changes of section 14: the ISS landing at half weight with no gap; Venus worth a slot of its own yields; the even trade; a leg no tank can pay excluded from destinations; the Production Moved weight x2 (#94).

## 17. Open for the designer

Carried from the map's fog and the sweep, none settled here:

- **The four-way balance.** The Custodians win 96 of 160; the Prospectors never reach 750 in the Fund (a median 437 to 481 in their own seatings); the Arkwrights never win; the gate delays the Custodians' win to turn 27 and denies nobody. What to turn next is the designer's.
- **Influence 1.2 and the Custodian AI's home seating.** From East Asia the Custodians go from 5 wins and 15 Collapses in 0.05.5 to none and 20; the rerun at 1.25 on ticket #82 restored the old figures. Whether 1.2 stands, the AI's Influence play is retuned, or the Allotment moves.
- **The climate cell.** As section 14 reports: holds from Europe, fails from East Asia and beside the Prospectors; not retuned.
- **Winning in orbit over Earth.** Off-world Presence and the Archive's twelve can both be met on a station over Earth with no transit; the sweep made the AI treat it as a last resort rather than the first, which is a weight and not a rule. Whether some part of a Victory Condition must leave the Earth system.
- **The Materials reserve and off-Earth Modules.** For three Factions a Colony off Earth can stand bare all game behind Earth's Facilities; whether off-Earth Modules should carry a weight of their own, as the Custodians' now do.
- **Efficient Transit on the AI's pick lists**, so the Mass Driver is seen in play outside the Archivists' seatings.
- **The Prospector AI and Venus**, which it never flies to; and the Arkwright AI's forward station, still raised after the crossing.
- **Solar Arrays over Earth**: 96 to 383 standing at the end over a batch of twenty, six clean Energy for 25 Materials with no upkeep; whether that is the intended shape of a station over Earth.
- **Venus wants something a player would cross for**: in AI play the Archivists live there in their hundreds for the Solar Array and the station's Habitats; whether a human will is the playtest's question.
- **The Arkwrights' Diaspora at Venus**: a station there is a Body for the three-Bodies count; whether Diaspora needs re-tuning now that a Body needs no landing.
