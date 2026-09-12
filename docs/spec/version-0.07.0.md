# Dying Earth — version 0.07.0, the UI, AI and accessibility version: a Colony with a ceiling, a computer that researches, a blockade that is one slot's business, and a board you can read

**Status:** the destination of the map [Map: version 0.07.0, the UI, AI and accessibility version](https://github.com/whaleyjoshua2/Dying-Earth/issues/96). Every change here was decided on one of that map's tickets from the designer's list; each section names its ticket, and **the ticket's resolution comment is the authority** if this document and it ever disagree. Everything not amended here stands as written in [`first-playable.md`](first-playable.md), [`version-0.02.md`](version-0.02.md), [`version-0.03.md`](version-0.03.md), [`version-0.04.md`](version-0.04.md), [`version-0.05.md`](version-0.05.md), [`version-0.05.5.md`](version-0.05.5.md) and [`version-0.06.0.md`](version-0.06.0.md).

**Numbers** live in `assets/data/`, one file per table; this document names the file beside each change.

**A rule marked "builder's call"** was settled in the build rather than on the ticket. Every one is gathered again in section 13 for the designer to veto.

**Where this version came from.** Unlike its predecessors it was not opened by a list of wanted features but by **twelve agent playtests of version 0.06.0** — three games at each of the four seats, every one played from seat 0 by an agent trying to win, through a headless driver written for the purpose. Ten wins for seat 0 in twelve games, two Collapses, **no AI seat won once**, and no game reached turn 27 of thirty-six. The write-up is [`docs/dev-diary/2026-09-11-agent-playtests/`](../dev-diary/2026-09-11-agent-playtests/). The designer's list then addressed what those games found.

**The designer's list, as given:** limits on Colony and station Modules as a function of population; AI fixes, starred, to incentivize research; the funding bug fixed by deciding before Income; the start globe draggable; "Committed +n C" confirmed and clarified; permissively licensed icons for the resources; Antarctica's slots hidden until they open; panels not opening themselves; a warning when ending a turn with no Tech; the Climate Panel resizable downward; a tenth less text; and warships blockading Orbital Slots rather than whole worlds.

---

## 1. A Colony or a Space Station holds three Modules free and one more for each Colonist

Decided on [What limits the Modules a Colony or a Space Station may hold](https://github.com/whaleyjoshua2/Dying-Earth/issues/97). Amends section 7 of the First Playable (Modules) and section 8 of version 0.04 (Space Stations).

Before this a Colony had **no ceiling at all**: `build_slots` and its kin all take a `StateId` and there was no Colony equivalent. With Build Where You Dig making each further Module cheaper than the last, one playtested Moon Colony reached **51 Mines and 28 Generators on about twelve Colonists**, extracting 775 Materials a turn by turn 13.

- **A Colony or a Space Station may hold `base + one per Colonist living there` Modules** (`modules.toml`: `[slots] base = 3, per_colonist = 1`). One formula for the ground and for orbit.
- **Habitats count** against it. **The Archive is exempt**, on both sides of the sum.
- **A mothballed Module keeps its slot** and **one under construction reserves one**, exactly as a Facility does in a Nation State.
- **A cap that has fallen below what already stands destroys nothing and mothballs nothing**: there is simply no room until Colonists arrive.
- The free allowance is **per Colony**, not per Faction.
- A **Space Station founded bare holds the three**, which is exactly the Shipyard, Solar Array and Observatory one is built with, and grows only as its people arrive.
- The AI does not enumerate Modules at a Colony with no room, so it never spends its list on builds that would be dropped at commit.

**Why one per Colonist and not one per two.** The designer's constraint was that a filled Habitat must always hand back **at least two slots** after paying for its own. Measured over 95 Colony Slots across five seeds, a full Habitat holds **3 to 14 Colonists, median 8** — the floor being Phobos and Deimos at a Habitat yield of 0.5. At one slot per two Colonists the poorest Habitat hands back **nothing**; at one per Colonist it hands back exactly two. The constraint picked the ratio; nobody chose it by taste. A test walks every Colony Slot on every Body and proves it.

The playtested runaway caps at **15**. The AI's median ground Colony, 3.5 Modules on 4 Colonists, gets 7 and never touches the ceiling.

## 2. Research for the computer players, and a Research Lead that chooses from three

Decided on [Research for the computer players: the Prospectors' weight, and whether the Research Lead itself changes](https://github.com/whaleyjoshua2/Dying-Earth/issues/98), with a correction built ahead of the map (section 11).

- **The Prospectors' `build_research_lab` and `build_observatory` are 8** (were 3) (`ai.toml`). Their own producers earn the same pace multiplier a Lab does, so the multiplier cannot tell a Factory from a Lab and only the base weight can. Swept over four seatings, twenty seeds each: **2 Labs in 80 games at stock, 20 at weight 6, 220 at 8, 1222 at 10** — where the whole tech tree completes in every seating and Research stops being a constraint. Eight is the knee before the cliff. **Provisional**, and owed a re-check: the ticket asked for it to be fitted against Collapses too, and that half was blind while the Module cap dominated them.
- **The Research Lead chooses from a shortlist of three** rather than from everything available (`techs.toml`: `[shortlist] size = 3`, refused below two). The draw **always carries the Lead's own Victory gate** once its prerequisites are met, so a Faction can be denied a rival's gate but never its own; the rest are drawn from the game's own generator.
- **The opening is untouched.** The first Tech of the game is a free choice from the whole of rung 1 — an **empty shortlist is that free choice**, so the opening needs no special case in the rules or the code.
- The AI is held to the same list a human Lead is.

Measured over all four seatings, gates completed in 80 games: **89 without the shortlist, 114 with it**, the Extraction Charter rising from 25 to 37, with the Techs median barely moving. That is rivals reaching their own doors rather than more research being done.

## 3. A warship blockades the Orbital Slot it sits in, and nothing else

Decided on [A warship blockades the Orbital Slot it sits in and nothing more](https://github.com/whaleyjoshua2/Dying-Earth/issues/99). Amends section 9 of the First Playable (Orbital Control) and section 8 of version 0.04.

Before this, Orbital Control was Body-wide and gated **every** unloading, so one enemy Frigate anywhere at a Body shut the whole world to everyone else — a Faction's own Space Station included — and an orbit two rivals contested denied **everybody**, punishing the bystander hardest. A playtested Archivist had the Archive complete at 80 of 80 on turn 19 and could not put Colonists into their own station for nine turns.

- **A Ship carries the Orbital Slot it sits in**, chosen with the leg that brings it (`transit <ship> <body> [slot]`). The choice is made when the leg is ordered, so it is made **before the Ship can see who will be there**. A Ship built at a Shipyard starts at the Body at large.
- **Many Ships may share a Slot.** It is an address, not a berth: Phobos and Deimos have one Orbital Slot each, and exclusive berths would have made them single-ship worlds by accident.
- **A warship in a Slot shuts that Slot to every other Faction**: no unloading into the station standing there, **no refuelling from it**, and an **empty Slot under blockade cannot be built into**. It reaches no further — not to another Slot at the same Body, and never to the ground.
- **The ground answers to Orbital Control alone**, and only a rival holding it **outright** shuts it. A contested orbit now shuts out nobody.
- **A Faction is never barred from a place it holds** by a ship that never touched it.
- **Cargo resolves before builds**, so winning the battle for an orbit wins the turn rather than being undone by the loser's shipyard finishing a hull in the same Resolution.
- The AI parks a warship in the Slot of the richest rival station at its destination, by Modules standing, and names no Slot where no rival keeps one.

Measured over four seatings: Collapses unchanged, and **a landing is barred 13 times in 80 whole games** rather than whenever an enemy warship happened to be present.

**A consequence the designer accepted:** a Ship already at a Body cannot change Slot without flying away and back, since the Slot is chosen with the leg.

## 4. The start screen's globe turns, zooms and answers a click

Decided on [Turning the globe by hand when choosing a starting continent](https://github.com/whaleyjoshua2/Dying-Earth/issues/100).

The globe on "Choose your starting continent" took no drag, no click and no wheel, and spun on a fixed rate nobody could steer, so a continent facing away could not be brought round to be looked at.

- It **opens on the Faction's home**, turns under a drag at the Earth Map's rates and clamps, **zooms on the wheel** (0.45x to 2.2x), and **a click picks the continent** under the pointer. The list of twelve buttons stays beside it.
- The spin runs from the home until the **first drag and then stops for good**: a spin that carried the home back out of view would undo the point of opening on it.
- **Each Faction has a home** (`factions.toml`: `home`), the designer's: the **Custodians in Europe**, the **Prospectors in East Asia**, the **Arkwrights in South Asia**, the **Archivists in North America**. A home changes **nothing but where the globe opens**: no starting position, and any of the twelve may still be chosen.

The playing screen's picker reads a `Game` and none exists on this screen, so `start_pick` takes the nearest of the twelve by `geo::state_lonlat`.

## 5. "Already locked in", and a figure that was already there

Decided on [What "Committed +n C" actually is, and what the Climate Panel should call it](https://github.com/whaleyjoshua2/Dying-Earth/issues/101).

- **The figure was correct.** `target_temperature()` is `base 1.2 + 0.5 x (CO2 - 420) / 300` (`climate.toml`): the warming **already locked in by the CO2 now in the air**, which the Temperature drifts toward at half the remaining distance every Climate phase. It is not the end-of-game projection and not the lag itself. Its hover text was accurate too.
- **The wording becomes "Already locked in"** — the one word on a panel the playtests praised that a player might have had to look up, beside "Last turn to act", which is plain English doing precise work.
- The second figure the designer asked for — where the current course ends — **was already on the panel**, two rows down, with the Collapse comparison the proposed line lacked. Nothing was added.
- The projection **jitters**, by instruction: it extrapolates the current net and moves turn to turn. Nothing is rounded.

## 6. Icons for the resources, and the credit their licence requires

Read on [Icons and art for the resources, under licences that permit use](https://github.com/whaleyjoshua2/Dying-Earth/issues/102) ([`docs/research/icon-licences.md`](../research/icon-licences.md)); built on [Put the resource icons in, and the credits screen their licence requires](https://github.com/whaleyjoshua2/Dying-Earth/issues/109).

- **Five icons from game-icons.net**, fetched unmodified into `assets/icons/`: **Ore** (Faithtoken) for Materials, **Jerrycan** (Delapouite) for Fuel, **Electric** (Sbed) for Energy, **Microscope** (Lord Berandas) for Research, **Coins** (Delapouite) for Ducats.
- They are **rendered from SVG at runtime**, the designer's call over baking PNGs, so any future icon dropped into that folder is a drop-in and every one is crisp at whatever size is asked for. `resvg` is the new dependency; `tiny-skia`, which rasterises under it, was already in the tree.
- **On the top bar the icon replaces the word**; elsewhere the icon sits **beside** the words; and since the text cull (section 10) the icons are used **exclusively on mouse-overs** for resource names inside tooltips. Where the art fails to load the words come straight back.
- The licence is **CC BY 3.0** and its one condition is attribution, so the title screen gains a **Credits** screen naming each icon, its author and the licence. The menus screenshot run photographs it with every other menu, so a change that breaks the attribution shows up in a picture.

Every game-icons SVG opens with a **full-canvas black rectangle** behind its white glyph, which is stripped before the tree is parsed; the glyph left behind is white and tints to whatever colour a label wants.

## 7. Nothing of Antarctica is drawn until the ice opens

Decided on [Antarctica's Colony Slots stay off the map until the ice opens](https://github.com/whaleyjoshua2/Dying-Earth/issues/103). Amends section 4 of version 0.04 (Antarctica).

Earth's three Colony Slots were drawn from turn 1 with their names and their **four yields** readable, so a player could shop for the best Antarctic site three degrees of warming before they could reach it.

- **Nothing of them is drawn until the ice opens**: no marker, no name, no yields, nothing to click.
- **The fact of it stays.** The Solar System Map still reads "Antarctica: opens at +1.6 C" and the Climate Panel's bar carries an ice-blue notch at exactly that temperature, so an Antarctic Colony can still be planned for — and the game's grimmest trade stays legible: **the cheapest colony site in it is bought by warming the world a little further**.

The unload list already refused Earth's slots until the ice opened, and the AI reads `antarctica_open` and never plans ahead of it, so the change is purely what is **shown**.

## 8. Nothing opens itself, and the Climate Panel has a floor

Decided on [Two interface defects: panels that open themselves, and a Climate Panel that will not shrink](https://github.com/whaleyjoshua2/Dying-Earth/issues/104).

- **A new game opens on a clear map.** The Climate Panel no longer starts open, and **the Tech Tree no longer throws itself up** when a pick is owed — which, at turn 1, is always.
- The top bar gains a **"Pick a Tech"** button in its place, since the only prompt otherwise lived inside the Tech Tree. Its condition matches the one that disables End Turn, so it cannot appear once every Tech is researched.
- Panels keep remembering their state **between turns** and not between sessions.
- **The Climate Panel has a floor to shrink to**: an explicit `min_width` of 230 and a `min_height`, `resizable` stated rather than implied, and `vscroll`, since the panel is taller than the window on a small screen and grew instead of scrolling. Its content was proved not to be the blocker by rendering it at 230 wide and looking: everything wraps and nothing overflows.

**Unwitnessed:** a drag cannot be reproduced headlessly, so the shrink is the one change in this version that was not watched failing and then passing.

## 9. The turn will not end while a human Lead owes a Tech

Decided on [Ending a turn with no Tech chosen](https://github.com/whaleyjoshua2/Dying-Earth/issues/105). Amends section 12 of version 0.03 (Research).

**The ticket's premise was false, and proving it came first.** The game already refused — End Turn is disabled while a pick is owed, the "Influence unspent" modal sits behind that button, and auto-advance is spectator-only where seat 0 is an AI that picks instantly. The **headless driver** written for the playtests did not refuse, which is why twelve playtest games were played in which declining to pick froze the tech tree for good, reported as "the strongest strategy in the game". It was the harness.

- **The rule lives in the engine.** `end_turn` returns a result and **refuses, with a sentence saying why**, while a human Research Lead owes the table a Tech. The game, the headless driver, the tests and anything built later are bound alike; the interface keeps its greyed button as a **courtesy**, not as the enforcement.
- On a refusal **nothing is committed, the turn does not advance, the orders go back to the player unspent**, and a **popup says why**, with a button that opens the Tech Tree.
- **Research banked while no Tech is under research keeps its owner.** `unallocated` is per seat, with a separate bucket for what is genuinely nobody's: a Breakthrough card, and the spill past a completed Tech's cost. Before this it arrived unattributed and produced Lead lines reading "the Prospectors led (Archivists 0, Custodians 0, Prospectors 0, Arkwrights 0)".

Six tests failed when the rule landed, each having driven turns with seat 0 owing a pick — the same permissiveness the driver had. They pick first now, as a player does.

## 10. The Report stops reporting defaults

Decided on [A tenth less text: what goes](https://github.com/whaleyjoshua2/Dying-Earth/issues/106).

Measured over a whole 21-turn game played headlessly: the Report ran **431 words a turn and 617 late**, and the rival paragraphs the playtest blamed were **26%** of the late game against **48%** for the repetitive Climate, Sea Level and Refugee lines.

- **A line only where something happened**, applied to those three kinds, was worth **1.2%** — because **the engine was already silent when nothing moved**. What was genuinely empty did come out: a population fall rounding below 0.05% printed "population fell 0.0%", spending a line to report that a country was fine. Those now say only what moved.
- **A rival's paragraph does not report defaults.** Hold is what a stack does when nobody tells it otherwise, so a Hold stance earns **no sentence at all**; five of the nine clauses in one sampled paragraph were "set its Armies at X to Hold".

| | words | a turn | rival paragraphs |
|---|---|---|---|
| before | 9048 | 431 | 2820 |
| after | **7858** | **374** | **1739** |

**13.2% off the Report entire and 38% off the rival paragraphs, with nothing lost.** The tenth the designer first asked for is **no longer a target**, by their word; this took more than it from padding alone.

## 11. What the climate needed after the cap, and Leapfrog's second bite

Decided on [What holds the climate now that a Colony has a ceiling](https://github.com/whaleyjoshua2/Dying-Earth/issues/108). Amends section 11 of the First Playable (the Climate Model) and section 6 of version 0.05 (Leapfrog).

The Module cap took Collapses from 1 of 20 to **19 of 20**. Two hypotheses were tested and were wrong: **not** Energy starvation (8 shortfalls in twenty seeds) and **not** the Scrubber's price (cutting it from 4 Energy to 3 left the figures unmoved to the digit, though the cut stands on its own merits). What moved was Earth: **Factories 560 to 951 and Industry Levels 432 to 934.** Denied Modules off Earth, the computer industrialises the planet it cannot leave.

- **Only the Prospectors' weights move**: `build_producer` 8 to 5 and `raise_industry` 9 to 5 (`ai.toml`).
- **The Scrubber runs on 3 Energy** rather than 4 (`facilities.toml`).
- **A Leapfrog takes 0.1 off its Nation State's Baseline Emissions** as well as the 0.03 off its people's coefficient, floored at nothing, and is refused only when it can buy **neither** (`climate.toml`: `leapfrog_baseline_cut`). This answers two measured problems at once: `baseline x Industry Level` was a floor nothing in the game could lower, and Leapfrog cost about ten times a Scrubber per ppm and was **bought zero times in twelve playtested games**. It is bought **1151 times** in 80.

| seat 0 | Collapses | Leapfrogs | Scrubbers |
|---|---|---|---|
| Custodians from Europe | 0/20 | 482 | 233 |
| Prospectors from North America | 7/20 | 128 | 60 |
| Arkwrights from South Asia | 2/20 | 348 | 187 |
| Archivists from Sub-Saharan Africa | 7/20 | 193 | 161 |

**Sixty of 80 becomes sixteen.** The world's net Emissions at the end go from **+111.8 to −2.7**, and the Custodians' Stabilization run — **0 of 3 in every playtested game** — reaches a **median of 3, which is their bar**. Their Victory Condition works for the first time.

**The designer asked for about 25 and 25 is not reachable.** The response is a cliff on one integer: the Prospectors' `build_producer` at 6 burns 56 of 80, at 5 burns 12 to 16, with nothing between, and sweeping the other three Factions across their whole range moves it only 12 to 16. That the climate has so little margin is itself a finding.

## 12. Two changes built ahead of the map

Both were made on the designer's direct instruction before the map was charted, and belong in the record.

**Fund the Archive is a standing declaration.** Amends section 4 of version 0.05.5. The order used to run in the Orders phase and claw the Research back out of the shared Tech that Income had already paid it into; when that Research completed a Tech, the completion zeroed every seat's contribution first, so the Archivists banked **nothing** while the Report said they had funded the Archive. It is now set by an order and **read at the next Income**, before a point reaches the Tech, and holds until set again. Measured over twenty seeds, Archivists from Europe: the Archive **completed in 8 of 20 before and 20 of 20 after**, and of 328 funding events before, **169 banked nothing at all**, against 0 of 218 after.

**The AI's victory model learned that every win waits on a Tech.** Ticket #84 put all four Victory Conditions behind a rung-3 Tech and nothing told `advances_first`, the table deciding which builds earn the pace multiplier. Research Labs completed per Faction over twenty seeds: Custodians 50, Archivists 26, **Prospectors 0, Arkwrights 0** — exactly the two the table left out. Adding the Research Lab and Observatory to both takes the Arkwrights from 0 to 46 and Techs from a median 12 to 14.

## 13. Builder's calls, for the designer to veto

- **Section 1:** the AI skips enumerating Modules where a Colony has no room; the refusal names the allowance and the Colonist count.
- **Section 2:** the shortlist is drawn in tree order so the panel reads the same way twice running; a pick clears the list and the next completion draws a fresh one.
- **Section 3:** the AI chooses its blockade Slot by the rival station's Module count, ties by Colonists; a Ship's Slot is remembered while it is in transit.
- **Section 4:** the globe aims at the home's **latitude** as well as its longitude — aiming at longitude alone left Europe at 50 N sitting on the limb, which only a picture showed.
- **Section 6:** icons render at 64 pixels; the hover substitution keeps trailing punctuation on the word it rides.
- **Section 9:** an AI Lead is never asked, so simulate mode is untouched; the refusal's sentence names the Faction.
- **Section 10:** the shortened heat sentence is its own template (`heat_unrest_only`) rather than a conditional clause.

## 14. The kit

The Windows kit is `dist/dying-earth-0.07.0/` and `dist/dying-earth-0.07.0-playtest.zip` (37.3 MB), built from the release profile, whose `.cargo/config.toml` links the C runtime statically so the exe runs on a machine with no Visual C++ runtime installed. The tester's note is `docs/playtest/PLAYTEST.txt`, copied into the kit as `README.txt`.

**Asserted rather than assumed:** the kit's own exe was run from the kit folder in `shot:` mode and produced its pictures, so the folder carries everything it needs — the new `assets/icons/` included.

**No Linux kit**, at the designer's word; the `release-kits` workflow was not run.

## 15. What was measured

Every figure in this document came from a run, not an estimate. The four-seating twenty-seed sweep is the standing instrument; new in this version is a **headless driver for seat 0** (`cargo run -p dying-earth-engine --example play`), which plays a whole game from the outside and is what the twelve playtests used.

**A caution the driver earned:** it did not enforce the Tech pick (section 9), and twelve playtest games drew a false conclusion from it. What a tool measures is only as good as its fidelity to the game, which is the argument for rules living in the engine.

## 16. The sweep at the end of the version

Four seatings, twenty seeds each, the standing instrument. Seat 0 is named first.

| seat 0 | Collapses | wins by seat |
|---|---|---|
| Custodians from Europe | 0/20 | **Custodians 20**, Prospectors 0, Arkwrights 0, Archivists 0 |
| Prospectors from North America | 7/20 | **Prospectors 9**, Custodians 1, **Arkwrights 3**, Archivists 0 |
| Arkwrights from South Asia | 2/20 | Arkwrights 1, **Custodians 17**, Prospectors 0, Archivists 0 |
| Archivists from Sub-Saharan Africa | 7/20 | Archivists 0, **Custodians 11**, **Prospectors 2**, Arkwrights 0 |

**The computer plays now.** In version 0.06.0 no AI seat won a single one of the twelve playtested games, and the sweep had the Custodians taking almost everything a human did not. AI seats take **34 of the 80** games here, in three of the four seatings.

**And the Custodians are the standing imbalance.** They win **49 of 80** across the table, from any seat, and 20 of 20 from their own. Their Victory Condition went from unreachable — a Stabilization run of 0 in every playtested game — to met in every seed of their own seating, in one change (section 11). That is very likely too far the other way, and it is the first thing the next version should measure.

**The Archivists win nothing** in these 80, from any seat, though the Archive itself now completes (section 12). Their Colonists-at-the-Archive half is the half to look at.

## 17. What is left open

- **The Victory bars want re-fitting.** Every one was set against an AI that never contested them, and three of four were cleared before turn 17 by a player who was trying. Nothing here re-fits them.
- **The Custodians win 49 of 80 games across the four seatings** and 20 of 20 from their own, having been unable to meet their condition at all before section 11 (see section 15). The first thing the next version should measure.
- **The Archivists win none of the 80**, though the Archive completes: their Colonists-at-the-Archive half is the half to look at.
- **The gap from 16 Collapses to the designer's 25**, to be closed on the climate table rather than on an AI weight.
- **The Prospectors' Research weight of 8 is provisional** and now re-checkable, since Collapses can discriminate again.
- **Two changes are unwitnessed**: the Climate Panel's drag (section 8) and the glyph tooltips (section 6). Neither can be reproduced headlessly.
- **Merging the per-state Climate, Sea Level and Refugee lines** is the only way left to cut that block properly, worth perhaps a fifth of the Report, and untaken because it trades away per-state figures.
- **Where the resource icons appear beyond the top bar and the tooltips.** The loader serves anything in `assets/icons/` at any size and tint; the build and trade popups are the obvious next users.
- **Pricing Blame into the AI's own decision**, so producers self-limit as emissions rise rather than being capped by a flat weight.
