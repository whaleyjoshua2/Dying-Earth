# Dying Earth — version 0.05.5, the balance version: thirty-six turns, Emigrants, the Venture Capital Fund, the Archive Module, the home claim

**Status:** the destination of the map [Map: version 0.05.5, the balance version](https://github.com/whaleyjoshua2/Dying-Earth/issues/66). Every change here was decided on one of that map's tickets after the designer played version 0.05; each section names its ticket, and the ticket's resolution comment is the authority if this document and it ever disagree. Everything not amended here stands as written in [`first-playable.md`](first-playable.md), [`version-0.02.md`](version-0.02.md), [`version-0.03.md`](version-0.03.md), [`version-0.04.md`](version-0.04.md) and [`version-0.05.md`](version-0.05.md).

**Numbers** live in `assets/data/`, one file per table; this document names the file beside each change.

**A rule marked "builder's call"** was settled in the build rather than on the ticket. Every one of them is gathered again in section 12 for the designer to veto.

**The designer's list**, as given: thirty-six turns of two months; the Archivists' Archive as a Module with its fund capped until it stands; a neutral state's Research to the shared Tech; the Sea Wall's Tech on rung 1; Influence ties settled; a handful of coastal slots moved inland; the Prospectors' bar at 750 Materials on hand; the Moon's yields up a tenth; Colonists built four a turn, launched or sent to Antarctica, lowering Unrest. Dropped by the designer mid-map: the Climate cards at half strength and the Scrubber at 3.3, on the condition in section 12. Added mid-map: a 15% discount on the Prospectors' buildings, a deck for thirty-six turns, the undefended home state, and the Sea Wall's slot.

---

## 1. Thirty-six turns of two months

Decided on [Thirty-six turns of two months: the calendar, the sky, the climate clock and the Tech pace](https://github.com/whaleyjoshua2/Dying-Earth/issues/67). Amends section 15 of the First Playable and section 9 of version 0.05.

- **The game runs thirty-six turns, each two calendar months.** Turn 1 is January 2030 and turn 36 November 2035. A turn is named by its first month alone: the top bar, the dispatch and the save list say "March 2030" for turn 2 (`victory.toml`: `turns = 36`, `months_per_turn = 2`).
- **Every build time, transit time and per-turn figure stays what it was in turns.** Nothing is doubled because a turn is longer.
- **The sky follows the calendar.** Each turn's sky is the first instant of its first month, and a transit's days become turns at sixty days a turn (`ephemeris.toml`: `days_per_turn = 60`, the flight cap `max_turns = 9`, the same year and a half). The Hohmann flight of 259 days is five turns; the game holds three Mars windows, near turns 7, 20 and 33; the window cycle is thirteen turns.
- **A turn stands at the window when the phase angle crosses the Hohmann angle anywhere inside the turn.** Builder's call: a turn spans two months, over which the angle moves nearly thirty degrees; read at the turn's first instant alone no turn ever stood at the window and every crossing paid a Fuel over the card. The offset is the nearest the angle comes to the window between the turn's first instant and the next turn's, zero when it crosses inside.
- **The Tech costs stay** at 15, 25 and 40.
- The climate clock is set in section 10.

Measured on the ticket, over twenty seeds: the Mars system reached in every seed at a median turn 11, where version 0.05 reached it in none of seven batches; Techs a median 2 to 3 a game with rung 3 seen, where 0.05 never left rung 1.

## 2. The Archive as a Module

Decided on [The Archive as a Module of three turns, its fund capped at a quarter until it stands](https://github.com/whaleyjoshua2/Dying-Earth/issues/68). Amends section 2 of version 0.05.

- **The Archive is one Module**, one per Faction, only the Archivists, only at a Colony off Earth: **50 Materials, three turns**, from its own button on the Colony card. The four Materials-and-Research stages are retired, and with them the word Project (`modules.toml`: the Archive row; `[archive] research = 80`, `banked_before_built = 0.25`).
- **The 80 Research is still required**, paid into the Archive fund at any pace once the Module stands. **Until the Module stands the fund holds at most a quarter of it, 20.** At the cap a turn of funding is refused: the Fund box is greyed and the Research goes to the shared Tech; nothing is wasted, and what the fund has no room for stays with the shared Tech and still counts toward the Research Lead.
- **The Archive is complete when the Module stands and 80 Research has been paid.** From then the 12 Energy upkeep is charged and the Victory Condition reads as before: the Archive complete and running, with 12 Colonists at its Colony (`factions.toml`: `victory_first = { kind = "archive_research", bar = 80 }`). An Archive raised over a kept fund of 80 (the Colony was lost and re-founded) is complete the moment it stands.
- **Provisional Findings is unchanged**: off for the turn after a turn of funding.
- **The Archivist AI learns the whole path**: a Launch Site and a Shipyard count as advancing the Archive (they did not, so the AI never left Earth), and the Archive's own steps bank Materials over twelve turns of income rather than four.
- A Module lost its stage field, so the save stamp is 0.05.5 and a 0.05 save is refused by name.

Measured on the ticket: the Archivist AI, which founded no Colony in twenty seeds before, builds a Shipyard and Colony Ships in every seed and founds a Colony in half; the Archive stood in no seed, because the Custodians took East Asia from the Archivists in every seed at a median turn 20 (section 7).

## 3. Research: two start Labs and the neutral half; the Sea Wall's Tech on rung 1

Decided on [Research: two start Labs in North America and South-East Asia, half to the shared Tech while neutral, and the Sea Wall's Tech on rung 1](https://github.com/whaleyjoshua2/Dying-Earth/issues/69). The designer replaced the list's "neutral countries contribute 10% of their research yield" in the charting round, since no neutral state had a Lab. Amends section 12 of the First Playable and section 8 of version 0.05.

- **North America and South-East Asia each start with a Research Lab**, added to their start Facilities (`nation_states.toml`), and **a start Research Lab always stands inland** (builder's call: so the sea never takes the world's Research). Ticket #24's rule that start Facilities number as many as the Industry Level reads "plus a start Lab".
- **A Lab in a Nation State nobody holds, or one under Occupation, runs itself**, pays no Energy, and **pays half its yield, rounded down, into the Tech under research**, counting toward no Faction's Research Lead, as a Breakthrough does. The yield is a Faction-less one: the row's figure by the state's people and schooling, times Public Science once every Faction has it (3 in North America and 2 in South-East Asia, so 1 each a turn). A Lab idled by a Wildfire or mothballed pays nothing. **Under Occupation the occupier pays the Lab's upkeep and draws no Research from it**; the half is the world's. A Faction that takes the state gets the Lab whole. Any Lab that falls into neutral hands does the same.
- **A Faction starting in North America or South-East Asia keeps the Lab as its own** from turn 1.
- **One Report line** under On Earth on turns the world's Labs paid in; the state card's Lab line reads "in no one's hands: N Research a turn to the Tech under research".
- **Coastal Engineering moves to Industry rung 1 at 10 Research with no prerequisite**, beside Efficient Grids; Clean Power keeps Efficient Grids alone as its need (`techs.toml`).

Measured on the ticket: Coastal Engineering done by turn 16 to 18 in every seed; the Tech pace quadrupled (Techs a median 11 to 12 a game against 2 to 3), no seed collapsed at the provisional climate step, and the Custodians met Stabilization outright in 36 of 40 games, where no Custodian had in any version.

## 4. Influence ties, fewer coastal slots

Decided on [Influence ties and fewer coastal slots](https://github.com/whaleyjoshua2/Dying-Earth/issues/70). Amends section 8 of the First Playable and section 8 of version 0.05.

- **A held place is never tied with a challenger** (a challenger needs the holder's Standing plus the margin), and the holder keeps it when two challengers tie. **When two or more challengers qualify for a neutral place on the same turn at the same Standing, the lot decides** among them, drawn from the game's own generator as a contested orbital slot is, so a seed replays the same draw; the Report names the claimants and who it fell to (`report.toml`: `claim_lot`).
- **Coastal slots per point of Coastal Exposure go from 3 to 2** (`nation_states.toml`: `coastal_per_exposure = 2`): 34 coastal slots in the world where there were 49. Europe's Refinery and North America's Factory stand inland from turn 1, and a Faction's Launch Site stands inland in those two states since their coast is full.
- **While a Sea Level event is within 0.2 C, a Sea Wall takes the victory-gap and threat multipliers** in the AI's weighing (the sea is a threat to the state), so it competes with the Scrubber on even terms.

Measured on the ticket: the first Sea Walls ever built in any version, 3 and 5 a batch; the sea takes 34 slots a game (was 49) and drowns 24 Facilities (was 27 to 29).

## 5. The Prospectors: the Venture Capital Fund, a 15% discount, the Moon's yields

Decided on [The Prospectors' bar: 750 Materials on hand; the Moon's yields up a tenth](https://github.com/whaleyjoshua2/Dying-Earth/issues/72). The discount was added by the designer when the ticket was opened. Amends section 14 of the First Playable and section 1 of version 0.05.

- **The Venture Capital Fund.** The Prospectors' own pool beside the Stockpile, as the Archivists' Archive fund is. Each Income a **share of their Materials output** goes into it: the Materials their Factories and Mines pay (after a Strip Permit's doubling, rounded down); Materials bought with Ducats, refunded by a Decommission or found by a Rich Seam are not output. **The share is set on any turn, in steps of 10% from 0% to 80%**, starting at 0% (`factions.toml`: `[venture_capital] max_share = 0.8`, `share_step = 0.1`). Two orders, the Prospectors only: setting the share (refused off the steps) and a draw.
- **The first part of their Victory Condition is 750 Materials in the Fund** in one End phase, with 12 Colonists off Earth as before; the last-turn ranking reads the Fund over 750 (`victory_first = { kind = "venture_fund", bar = 750 }`).
- **The running Extraction Total is retired**: from the condition, the Victory panel and the AI's Strip Permit timing, which reads the Fund's pace instead.
- **A draw returns nine tenths**: Materials taken back out of the Fund return nine tenths to the Stockpile, rounded down (`draw_return = 0.9`).
- **The share is a strategic choice**, in the designer's words: "an AI/player may set it at 50% for five turns then down to 0% if they're trying to save; end game might try to max at 80% to reach the victory condition before others." The AI banks nothing before the pace's first waypoint (turn 9, it builds first), then the smallest step that reaches 750 by turn 34 at its current output, and 80% when nothing less will; its pace is 100 in the Fund by turn 9, 250 by 18, 450 by 27, 750 by 34 (`ai.toml`).
- **The share row and a Draw button sit on the Victory panel**, and the top bar shows "Fund N (S%)" beside the Prospectors' Materials. The banked Materials show among the Income sources.
- **Facilities and Colony Modules cost the Prospectors 15% less**, rounded down: a 20-Materials Factory is 17, a 25-Materials Habitat 21; a building bought with Ducats follows; Space Stations, Ships and Industry Level are not discounted (`factions.toml`: `facility_materials_multiplier` and `module_materials_multiplier` at 0.85).
- **The Moon's four yields up a tenth**: Mine 1.65, Generator 1.375, Refinery 0.55, Habitat 1.1 (`bodies.toml`).

Measured on the ticket: the Fund ended at 0 in every seed, because the Prospectors in East Asia lost it to the Custodians on turn 7 in every seed and earned nothing after (section 7). After section 7's rules the Fund fills to a median 219 of 750.

## 6. Colonists are built: Emigrants

Decided on [Colonists are built, four a turn, launched or sent to Antarctica, and building them lowers Unrest](https://github.com/whaleyjoshua2/Dying-Earth/issues/73). Amends sections 5 and 9 of the First Playable and section 2 of version 0.05.

- **Colonists are built.** A new Orders-phase order musters up to **four Emigrants a turn per Faction, in one Nation State the Faction directs**, at **0.1 population each** and nothing else; they stand on the state's card at End Turn, so nothing lifts them the turn they are ordered (a turn to muster: read from the designer's "one turn of production", the reading the designer did not correct). `factions.toml`: `[emigrants] per_turn = 4`, `population_each = 0.1`.
- **They wait on the state's card as Emigrants**, no upkeep, and stay with the state if it changes hands or throws its controller off.
- **A Launch Site lifts only the Emigrants waiting in its state**; a Ship at Earth can no longer draw straight from the population, and the lift takes no population. A lift still takes a turn and still counts as a launch for Emissions.
- **Antarctica by sea**: once the ice is open, waiting Emigrants are sent straight to an Antarctic slot from any state the Faction directs, founding a Colony or joining one of the Faction's own, with no Ship; **a turn to arrive; no launch Emissions** (`antarctica_turns = 1`). They land into their slot if it is still free, else into the Faction's own Antarctic Colony with room, else they come home.
- **Building a batch takes 0.5 off the state's Unrest**, once per turn it happens, damped by nothing (`unrest_fall = 0.5`).
- **Steerage**: the Arkwrights muster **eight a turn at twice the population per Colonist**, and their Colony Ships still carry double (`emigrants_multiplier = 2.0`).
- The state card shows "Emigrants waiting: N" under the Unrest line and carries the Muster button and, with the ice open, a Send button per free Antarctic slot and per own Colony there; the Ship's Load button lifts what waits. The AI musters in the state with a working Launch Site while fewer wait than two Ship loads (one more with the ice open), lifts what waits, and sends by sea.

Measured on the ticket: about fifty batches a game; all three Antarctic slots settled by sea in every seed; Colonists off Earth 20 to 32 at the end; the median peak Unrest down from 10.0 to 9.0, the first time it left the ceiling since Unrest arrived.

## 7. The home state: a claim from turn 1

Decided on [The undefended home state: the Custodian AI takes seat 0's start state by turn 7 in every seed](https://github.com/whaleyjoshua2/Dying-Earth/issues/75), opened from section 5's finding, in two rounds. Amends section 8 of the First Playable and section 1 of version 0.05.

The first round was an AI fix: **every AI holder pushes as many 5-Influence holds as it takes to stand two steps clear of a rival's Standing plus the challenge margin** once the rival comes within two steps, as many as its Allotment and Ducats allow; and **the state card warns**, in orange under the Unrest line, when a rival's Standing is within two steps of the player's own on a place the player holds, naming the rival, both Standings and the figure at which it changes hands. Measured, it did not hold the state: the holder's own Standing on its start state was zero, so a richer challenger needed only the threshold. The designer then chose the rule:

- **Every Faction begins with a Standing on its start state equal to that state's threshold**: a claim on its home from turn 1, so a challenger needs the threshold plus the margin at once.
- **The challenge margin goes from 10 to 20** (`influence.toml`: `challenge_margin = 20`).
- **A state another Faction holds counts 0.3 of a neutral one on the AI's Influence target list** (0.6 before, in the code; now `ai.toml`: `[thresholds] held_state_weight = 0.3`), so a held place is attacked only when no neutral one is worth having.

Measured on the ticket: the Prospectors keep East Asia in 9 of 20 seeds and hold it to turn 11 where they lost it on turn 8 in every seed; the Fund fills to a median 219; the Custodians' wins fall to 10 and 12 of 20 in two seatings; the Archivists still lose East Asia in every seed, at turn 22 (section 12).

## 8. A deck for thirty-six turns

Decided on [A deck for thirty-six turns: 40 cards, a third copy of four, a second of four, and four new Events](https://github.com/whaleyjoshua2/Dying-Earth/issues/76), added by the designer mid-map. Amends section 13 of the First Playable and section 2 of version 0.02.

- **The deck holds 40 cards**, never reshuffled, the draw chance unchanged (`events.toml`): the 28 of before, **a third copy of Heatwave, Wildfire, Rich Seam and Solar Storm**, **a second of Unrest, Methane Burst, Labour Dispute and Dust Storm**, and four new Events once each.
- **Drought** (climate, a Nation State): its Facilities make half at the next Income and its Unrest rises by 1; blunted by Green Consensus. Builder's call: the one Climate card the Temperature scale does not reach, since a halving cannot scale; the Unrest rise is a flat 1 as a climate source (`drought_output_multiplier = 0.5`, `drought_unrest = 1.0`).
- **Volcanic Eruption** (climate, everyone): 5 ppm leave the CO2 Stock at once, scaled like every Climate card, the one card that cools; blunted by nothing (`volcanic_co2 = 5.0`).
- **Moonquake** (failure, the Moon): every Module on the Moon is offline until the next Resolution, as Dust Storm does to Mars; blunted by Closed-Loop Colonies.
- **Helium-3 Vein** (discovery, the Moon): the Moon's Generators produce x2 for two turns, x3 with Efficient Grids, as Rich Seam does for a Body's Mines.

Measured on the ticket: 11 to 17 cards drawn a game, the deck never empty; the Climate Panel's line reads "40 cards left in the deck, 12 of them Climate".

## 9. The Sea Wall takes no build slot

Decided on [The Sea Wall takes no build slot, as the Scrubber does](https://github.com/whaleyjoshua2/Dying-Earth/issues/77), opened from a question of the designer's and decided over three rounds. Amends section 8 of version 0.05.

- **The Sea Wall takes no build slot**, as the Scrubber does, and costs **20 Materials** (`facilities.toml`: `no_slot = true`, `materials = 20`; it stood in a coastal slot at 35). It still needs Coastal Engineering, one per state, two turns, and still absorbs the state's next Sea Level threshold of any kind and is destroyed doing it. The AI is offered it while the state has a coast left to protect.
- **The AI picks Coastal Engineering early**: second on the Custodians' pick list and third on the Archivists' (`ai.toml`). Builder's call: it was on no list, so every AI took it around turn 17 as the cheapest Tech left, after the sea had taken every coastal slot (the first event around turn 9, the Ice Sheets Break around 13), and no wall was ever built whatever its slot.

The designer chose from a comparison over twenty seeds, seat 0 the Prospectors in East Asia and the Custodians from Europe, all rows but the first with the early Tech pick:

| rule | Sea Walls a batch | Collapses | Custodian wins | coastal slots lost a game | Facilities drowned | Scrubbers a batch |
| --- | --- | --- | --- | --- | --- | --- |
| before the ticket: coastal slot, 35, Tech late | 0 and 0 | | 10 and 12 | 34 and 34 | 24 and 24 | 366 and 606 |
| coastal slot, 35 | 22 and 6 | 11 and 9 | 9 and 11 | 34 and 34 | 24 and 24 | 438 and 646 |
| no slot, 35 | 220 and 372 | 13 and 11 | 7 and 9 | 34 and 29 | 24 and 19 | 365 and 571 |
| coastal slot, 20 | 30 and 48 | 14 and 10 | 6 and 10 | 34 and 34 | 24 and 25 | 329 and 606 |
| **no slot, 20 (chosen)** | **345 and 514** | **16 and 14** | **4 and 6** | **34 and 29** | **24 and 19** | **341 and 402** |

The wall is a trade against the Scrubber for the same Materials, and at no slot and 20 the Custodian AI takes it often.

## 10. The build ticket: the climate re-sweep and the four-way balance

Decided on [Write the 0.05.5 amendments and build them](https://github.com/whaleyjoshua2/Dying-Earth/issues/74).

### The climate clock

Version 0.05 chose `ppm_step = 180` for twenty-four turns. Section 1 raised it provisionally to 270 so the tickets between measured a game that reached its late turns; section 3's Tech pace then stopped the world collapsing at that step. With every change in, the step and the Sink were re-swept over five seatings (Custodians, Prospectors and Arkwrights in East Asia; Custodians and Archivists from Europe), `ppm_step` {240, 270, 300, 330} x `natural_sink` {6, 8}, twenty seeds a cell: **800 games**. The tables are in [the dev diary](../dev-diary/2026-09-10-version-0.05.5/sweep/climate-sweep.txt).

**Target, restated from 0.05:** every seating stays hot to the end (a median end Temperature of +2.5 to +2.9 C where it does not collapse), and Collapse is a real threat but not a certainty, with a median Collapse turn late in the game.

**Chosen: `ppm_step = 300`, the Sink at 6.0** (`climate.toml`). At 300 four of the five seatings end hot (+2.7 to +3.0) and their median Collapse turn is 22 to 34 of 36, with 9, 0, 7 and 3 Collapses in 20; the Custodians in East Asia still collapse 15 of 20 at a median turn 19, as they did at every step tried (13 of 20 at 330). At 270 three seatings collapse at a median turn 18 to 22, cutting the game's second half; at 330 two boards finish comfortable at +2.4. **No cell fits every seating**, as in 0.05; the designer may move this, 270 for a hotter world, 330 for a kinder one.

| seating | step 270, Sink 6 | step 300, Sink 6 | step 330, Sink 6 |
| --- | --- | --- | --- |
| Custodians in East Asia | 18/20 at 18, +3.11 | **15/20 at 19, +3.04** | 13/20 at 20, +3.03 |
| Prospectors in East Asia | 16/20 at 21, +3.00 | **9/20 at 22, +2.79** | 9/20 at 23, +2.54 |
| Arkwrights in East Asia | 9/20 at 30, +2.97 | **0/20, +2.74** | 0/20, +2.44 |
| Custodians from Europe | 14/20 at 22, +3.01 | **7/20 at 29, +2.93** | 4/20 at 32, +2.78 |
| Archivists from Europe | 7/20 at 31, +2.79 | **3/20 at 34, +2.37** | 1/20 at 34, +2.35 |

(Collapses in 20 at the median Collapse turn, and the median end Temperature.)

### The four-way balance

Eight batches of twenty seeds of four-seat `simulate` at the chosen cell, seat 0 each Faction in East Asia and from Europe, the other three seats the AI in the game's own seating (`--balance`): **160 games**. The full report is in [the dev diary](../dev-diary/2026-09-10-version-0.05.5/sweep/balance.txt).

| seat 0 | wins C / P / Ark / Arch | Collapses (median turn) | Techs (median) / rung | Sea Walls a batch | Archive stood / complete | Colonists off Earth (median) | Venture Fund at the end (median) | seat 0 lost its home |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Custodians in East Asia | 5 / 0 / 0 / 0 | 15/20 (19) | 9 / 3 | 273 | 0 / 0 | 16 | 391 | 0/20 |
| Prospectors in East Asia | 11 / 0 / 0 / 0 | 9/20 (22) | 9 / 3 | 428 | 0 / 0 | 16 | 165 | 11/20 (turn 11) |
| Arkwrights in East Asia | 19 / 0 / 1 / 0 | 0/20 | 13 / 3 | 372 | 0 / 0 | 50 | 160 | 20/20 (turn 22) |
| Archivists in East Asia | 20 / 0 / 0 / 0 | 0/20 | 13 / 3 | 542 | 0 / 0 | 46 | 203 | 19/20 (turn 21) |
| Custodians from Europe | 13 / 0 / 0 / 0 | 7/20 (29) | 13 / 3 | 518 | 0 / 0 | 23 | 81 | 0/20 |
| Prospectors from Europe | 1 / 0 / 0 / 0 | 19/20 (26) | 13 / 3 | 414 | 0 / 0 | 20 | 498 | 12/20 (turn 19) |
| Arkwrights from Europe | 20 / 0 / 0 / 0 | 0/20 | 12 / 3 | 631 | 0 / 0 | 48 | 124 | 20/20 (turn 17) |
| Archivists from Europe | 14 / 0 / 2 / 1 | 3/20 (34) | 13 / 3 | 873 | 6 / 5 | 66 | 141 | 14/20 (turn 19) |
| **all eight** | **103 / 0 / 3 / 1** | **53/160** | | **4051** | **6 / 5** | | | |

Every game that did not collapse had a winner (no draws). Where version 0.05's report read "the Prospectors win and nobody else ever does", this one reads **the Custodians win 103 of 160 and met Stabilization outright in 97 of them**; the Arkwrights won 3, the Archivists 1 (an Archive complete at a median turn 25 in 5 of 20 games from Europe, the first ever), and the Prospectors none (the Venture Capital Fund ends at a median 81 to 498 of the 750 the bar asks, its best seating the one it starts from Europe in). The Custodians' Stabilization is still reached, so the dropped cards-and-Scrubber ticket stays dropped (section 12). Techs run to a median 9 to 13 a game with rung 3 reached in every batch (0.05: one Tech, rung 1). The Mars system is reached in 6 to 20 of 20 seeds at a median turn 12 to 24; all three Antarctic slots are settled by sea in every seed. The deck is never empty. Every one of the five Breaks fires in every seed but Amazon Dieback.

## 11. Acceptance for 0.05.5

- `cargo build --release`; **clippy clean** across every target under `-- -D warnings`.
- **184 formula tests** and **6 save tests**, every new rule seen red first or watched red under a deliberate mutation of its figure, as the tickets record.
- `shot:` pictures of everything this version added, taken headlessly with the window off-screen, looked at, and in the dev diary (`docs/dev-diary/2026-09-10-version-0.05.5/`): the March 2030 dispatch, the January 2031 window with its five-turn flight, the Archive standing and on order, the Tech Tree with Coastal Engineering on rung 1, the North America card with its inland Lab and the dispatch's neutral-Research line, Europe's card with its Refinery inland, the Victory panel with the Venture Capital Fund, the Climate Panel's forty-card line, East Asia's card with Emigrants waiting, and North Africa's card with the home-state warning.
- **Twenty seeds of `simulate` per seating after every ticket**, in the diary, reported and not re-tuned except where a ticket said so; at the close, the five-seating climate re-sweep and the eight-batch balance report.
- The playtest kits: `dist/dying-earth-0.05.5/` and `dist/dying-earth-0.05.5-playtest.zip` built here for Windows, and a Linux kit from the `release-kits` workflow on GitHub's Ubuntu runner.

## 12. Open for the designer

Recorded on the map and the tickets, not decided here.

- **The Custodians win 103 of 160.** Version 0.05's imbalance has inverted: the Prospectors won everything then and nothing now. The pieces are all measured on the tickets: the Custodian AI takes rival home states (below), Stabilization is now reachable through the Tech pace (section 3), and the Scrubber plus the Sea Wall give it the Materials sink the others lack. The other three Factions' conditions are far off: the Fund at a median 81 to 498 of 750, the Archive standing in one seating only, the Arkwrights' 12 Colonists off Earth met but their second part not. Which knob to turn (the bars, the Custodian AI's Influence appetite, the Scrubber) is the designer's; nothing here was re-tuned after the tickets closed.
- **The home state holds for the Custodians alone.** Seat 0 keeps its start state in every seed when it is the Custodians and loses it in 11 to 20 of 20 as any other Faction, at a median turn 11 (Prospectors) to 22 (Arkwrights, Archivists), because the Custodian AI is the one Faction rich enough in Influence to pass the threshold plus the margin of 20. The Archivists' home question from the map (their Colony card, their own start state) is still open; section 7's rules made the loss later, not rarer.
- **The Sea Wall trades against the Scrubber.** At no slot and 20 Materials the Custodian AI builds eleven to forty-four walls a game (4051 in 160 games) and fewer Scrubbers, and its own seatings collapse more (section 9's table). The designer chose this rule seeing the comparison; whether the AI should weigh the two differently is open.
- **The climate cell.** No step fits every seating: at 300 the Custodians in East Asia collapse 15 of 20 and the Prospectors from Europe 19 of 20 (that seating's Custodian AI, in seat 1, builds the fewest Scrubbers of any batch, 210), while the Arkwrights and Archivists in East Asia never collapse and end at +2.5 to +2.7. 270 for a hotter world, 330 for a kinder one, both swept in the diary.
- **"One turn of production"** was read as a turn to muster (section 6): Emigrants ordered this turn stand on the card at End Turn and lift next turn. If the designer meant that a batch costs the state a turn's output, that is a different rule and was not built.
- **Coastal Engineering is on the Custodians' and Archivists' pick lists only.** The Prospector and Arkwright AIs still take it as the cheapest Tech left, so a Prospector seat 0 from Europe had it in 13 of 20 seeds. Adding it to their lists is a one-line change in `ai.toml` the designer may want.
- **The warning line's place.** The rival-Standing warning sits under the Unrest line on the state card, where a glance finds it, not in the Influence section below the fold. If the card is reorganised it should move with the Influence figures.
- **The dropped cards-and-Scrubber ticket** ([Halve the Climate cards' strength and raise the Scrubber to 3.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/71)) was dropped on the condition "note otherwise if the CO2 sweep changes it". The re-sweep did not: the Custodians reach Stabilization outright in 97 of 160 games at the chosen cell. It stays closed.

**Every builder's call, gathered**

- **Section 1:** the window offset over the whole two-month turn; the flight cap of nine turns is the same year and a half; a turn is named by its first month.
- **Section 2:** the Fund box greys at the cap and the excess stays with the shared Tech; an Archive raised over a kept fund of 80 is complete at once; "Project" retired from the glossary.
- **Section 3:** a start Lab stands inland; the world's Lab yield carries no Faction multiplier and takes Public Science; a Lab idled by a Wildfire or mothballed pays nothing; the card's Lab line; ticket #24's count rule reads "plus a start Lab".
- **Section 4:** the lot is drawn from the game's generator; the wall's threat multiplier is the sea itself; East Asia's four coastal slots are all gone after the +1.8 threshold and the Ice Sheets Break, so the scheduled +2.3 fires and finds nothing.
- **Section 5:** the Fund's orders land at Resolution and the share is read at the next Income; the banked Materials show among the Income sources; the AI banks nothing before turn 9.
- **Section 6:** "one turn of production" read as a turn to muster; the Unrest fall is once per turn a batch musters there, whatever its size; Emigrants stay with their state if it changes hands.
- **Section 7:** the AI's hold count is capped by what its Allotment and Ducats can buy in a turn; the warning line sits under the Unrest line, not in the Influence section below the fold.
- **Section 8:** a Drought is the one Climate card the Temperature scale does not reach; a Volcanic Eruption scales, so it cools more when the world is hotter; a Moonquake shares the Dust Storm's arm; a Helium-3 Vein is a discovery on Generators.
- **Section 9:** Coastal Engineering on the Custodians' and Archivists' pick lists; the wall is offered while the state has a coast left.

---

*Decisions recorded on the map's tickets remain the source of truth. This document assembles them; it does not amend them.*
