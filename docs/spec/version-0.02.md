# Dying Earth — version 0.02, amendments to the First Playable

**Status:** the destination of the map [Map: version 0.02 of the First Playable](https://github.com/whaleyjoshua2/Dying-Earth/issues/21). Every change here was decided on one of that map's tickets after the designer played the First Playable; each section names its ticket, and the ticket's resolution comment is the authority if this document and it ever disagree. Everything not amended here stands as written in [`first-playable.md`](first-playable.md).

**Numbers** live in `assets/data/`, one file per table; this document names the file beside each change.

---

## 1. Twenty-four turns and the re-paced CO2 clock

Decided on [Twenty-four turns and the re-paced CO2 clock](https://github.com/whaleyjoshua2/Dying-Earth/issues/27). Amends sections 1, 6, 11.1, 16.3 and 19.3 of the First Playable.

- **A turn is still one month. The game runs twenty-four turns** (`victory.toml`, `turns = 24`). Every per-turn number in every table is unchanged.
- **Factions must be unique.** A game is one Custodian seat against one Prospector seat. The First Playable's two-Prospector run and its anchor ("Collapse around turn ten when both AIs are Prospectors") are retired; `simulate` refuses a same-Faction pairing.
- **The clock:** the conversion becomes **+0.5 °C per 90 ppm above 420** (`climate.toml`, `ppm_step = 90`; it was 40). The Natural Sink stays 6.0. Chosen by a sweep, twenty seeds per cell, so that the Custodian-versus-Prospector game (both AI) collapses by about turn 22. The sweep was run twice: with seven Nation States an 80 step met the target; the two states of section 2 added four emitting start Facilities and pulled Collapse to turns 13 to 17, so the sweep was re-run on the build ticket and 90 chosen. With nine states it collapses in 17 of 20 seeds between turns 15 and 21, the typical one on turn 16, and the three survivors end at +2.7 to +3.0 °C. A 100 step, one number away, gives 10 of 20 with a median of 19. The sweep and both tables are in `engine/examples/sweep.rs` and the dev diary.
- **The AI's pace** (16.3) stretches with the game (`ai.toml`): Prospectors hold 40 Extraction Total by turn 6, 150 by 12, 320 by 18, 500 by 24; Custodians are within 4 ppm of the Sink by turn 12 and under it by 18; both hold 4 Colonists off Earth by turn 12 and 12 by turn 22.
- **Unchanged:** the Stabilization run (3 turns), the Extraction Total (500), the Off-world Presence (12), the Occupation cap, build and transit times, the sea-level thresholds.

## 2. Two more Nation States

Decided on [Two more Nation States](https://github.com/whaleyjoshua2/Dying-Earth/issues/26). Amends section 4.2. Nine Nation States (`nation_states.toml`):

| State | Population | Industry Level | Resource Lean | Baseline Emissions | Education Level | Size | Coastal Exposure |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Africa | 14.0 | 1 | Materials | 0.3 | 0.8 | 3 | 1 |
| Antarctica | 0.0 | 0 | Fuel | 0.0 | 1.0 | 2 | 0 |
| Asia | 43.5 | 3 | Materials | 0.4 | 0.9 | 3 | 2 |
| Australia and Oceania | 0.5 | 2 | Energy | 0.2 | 1.35 | 2 | 2 |
| Europe | 6.0 | 3 | Energy | 0.3 | 1.45 | 2 | 1 |
| North America | 6.0 | 3 | Fuel | 0.4 | 1.5 | 3 | 1 |
| South America | 4.5 | 1 | Fuel | 0.2 | 1.1 | 3 | 1 |
| **Russia** | 1.5 | 2 | Fuel | 0.4 | 1.35 | 3 | 1 |
| **The Middle East** | 3.5 | 2 | Fuel | 0.5 | 1.0 | 2 | 1 |

Russia is cut out of Europe and the Middle East out of Asia; each parent lost the population that left and one Size. On the globe, Russia is everything east of Finland and the Baltics above 55 °N, east of Belarus to 55 °N, east of Ukraine down to the Caucasus, and Siberia north of the Kazakh, Mongolian and Manchurian borders, out to the Pacific and across the Bering Strait; the Middle East runs from the Bosporus to Iran's eastern border, with the Caucasus below 44 °N, everything east of the Red Sea, and the Arabian Peninsula.

**Continent adjacency**, edges symmetric:

| From | Neighbours |
| --- | --- |
| North America | South America, Europe, Russia |
| South America | North America, Antarctica |
| Europe | North America, Africa, Russia, the Middle East |
| Asia | Russia, the Middle East, Australia and Oceania |
| Africa | Europe, the Middle East, Antarctica |
| Australia and Oceania | Asia, Antarctica |
| Antarctica | South America, Africa, Australia and Oceania |
| Russia | Europe, Asia, North America |
| The Middle East | Europe, Asia, Africa |

## 3. Start buildings and the starting Stockpile

Decided on [Start buildings in every Nation State](https://github.com/whaleyjoshua2/Dying-Earth/issues/24). Amends sections 8.1 and 14.3.

- **Every Nation State starts with as many Facilities as its Industry Level**, chosen by its Resource Lean: a Materials-leaning state gets Factory, Power Plant, Refinery in that order; an Energy-leaning state Power Plant, Factory, Refinery; a Fuel-leaning state Refinery, Power Plant, Factory; the first N of its list (`nation_states.toml`, `start_facilities`). Antarctica starts empty.
- **They come with the state whoever takes it.** A neutral state's Facilities are there for whoever wins it by Influence or Occupation.
- **The Faction start states keep their Launch Site** on top of the list.
- **The Stockpile starts at 80 Materials**, 20 Fuel, 20 Energy (`factions.toml`, `[start]`).
- **A Facility nobody directs stands idle:** in a neutral state it makes nothing and emits nothing. (The builder's reading; the designer may veto.)

## 4. The Event Deck: no Calm Cards, a Draw Chance, thirty cards, six new Events

Decided on [Events ten percent rarer](https://github.com/whaleyjoshua2/Dying-Earth/issues/25). Replaces section 13.1 and extends 13.2 (`events.toml`).

- **There are no Calm Cards.** The deck holds only Events.
- **The Draw Chance.** Each turn, in phase 5, a card is drawn with probability **50% at +1.2 °C, plus 2.5 points for every full 0.2 °C the Temperature stands above it** (72.5% at the Collapse Line). On a turn with no draw, the Report says so. The Climate Panel states the chance.
- **Thirty cards:** the twelve First Playable Events **twice each** and the six new Events **once each** (`copies` per row). Shuffled with the game seed; never reshuffled. About fifteen cards come in twenty-four turns.
- **Climate scaling** (1 + (Temperature − 1.2) ÷ 2 on Climate cards) is unchanged, so warming raises both the chance of a card and the damage of a Climate card.
- **The six new Events:**

| Kind | Card | Target | Effect in Resolution | Blunted by |
| --- | --- | --- | --- | --- |
| Solar | Solar Maximum | everyone | every Power Plant and Generator makes ×1.5 at the next Income | Efficient Grids: ×2 |
| Solar | Meteor Shower | everyone | every Ship in orbit around a Body takes 1 damage | Hardened Hulls: none |
| Failure | Dust Storm | Mars | every Module on Mars is offline until the next Resolution | Closed-Loop Colonies: none |
| Failure | Unrest | one Nation State, by population | its Standing Army takes 2 damage; every Faction's Influence there drops by 5 | — |
| Failure | Reactor Leak | one Colony with a Generator | its Generators are offline until the next Resolution; its holder loses 5 Energy | Closed-Loop Colonies: none |
| Climate | Permafrost Thaw | everyone | +3.0 Emissions next turn, scaled by the Temperature; never counted against Stabilization | Green Consensus: halved |

## 5. The roster

Decided on [The roster: every unit and Ship the player controls, in one list](https://github.com/whaleyjoshua2/Dying-Earth/issues/23). Extends section 17.3.

When nothing is selected, the side panel is the **roster**: every Ship stack (and every Ship in transit), every Army, every Colony and every Nation State the player directs, grouped in that order, each with its key numbers. Clicking a row selects the item and switches to the view it is on. A Ship stack or Army with nothing pending this turn is marked "no order". Escape clears a selection and brings the roster back.

## 6. Income on the card and on hover

Decided on [Show each building's income in the territory list and on hover before building](https://github.com/whaleyjoshua2/Dying-Earth/issues/22). Extends section 17.3.

Every Facility and Module on a Nation State's or Colony's card shows its per-turn output, Energy upkeep and Emissions at today's multipliers, and "offline, making nothing" when the shortfall rule shut it. Every build button shows the same three figures on hover for the building it would make. The figures are the Income phase's own: one engine function feeds both, pinned by a formula test.

## 7. Acceptance for 0.02

- `cargo build --release`; clippy clean across every target under `-- -D warnings`.
- `dying-earth.exe shot:check` writes the four views off-screen, the Earth Map with nine tinted states; the pictures are looked at.
- `dying-earth.exe simulate:<seed>` refuses a same-Faction pairing and plays Custodians against Prospectors for twenty-four turns. Over twenty seeds the anchors are reported at twenty-four turns; misses reported, not retuned.
- Formula tests for every rule above, each seen red first; 52 in all at the close of the map.

## 8. Open for the designer

Recorded on the tickets, not decided here:

1. **No Faction ever meets its Victory Condition** in an AI-versus-AI game: the Custodians never hold three straight turns under the Sink, and the Prospectors never reach 500 Extraction because they lose their states.
2. **The Custodian AI takes every Nation State by Influence** over twenty-four turns; its 1.3× Allotment and a weight of 8 against 5 compound.
3. The First Playable's fourteen builder's calls on pull request #20 still stand, including the AI's bootstrap and saving rules.
4. The Russia line through northern Kazakhstan on the mask.

---

*Decisions recorded on the map's tickets remain the source of truth. This document assembles them; it does not amend them.*
