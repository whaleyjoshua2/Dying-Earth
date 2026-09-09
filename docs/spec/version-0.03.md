# Dying Earth — version 0.03, housekeeping and the Influence system

**Status:** the destination of the map [Map: version 0.03, housekeeping and the Influence system](https://github.com/whaleyjoshua2/Dying-Earth/issues/30). Every change here was decided on one of that map's tickets after the designer played version 0.02; each section names its ticket, and the ticket's resolution comment is the authority if this document and it ever disagree. Everything not amended here stands as written in [`first-playable.md`](first-playable.md) and [`version-0.02.md`](version-0.02.md).

**Numbers** live in `assets/data/`, one file per table; this document names the file beside each change.

---

## 1. Standings: Influence that persists

Decided on [Influence standings that persist](https://github.com/whaleyjoshua2/Dying-Earth/issues/33). Replaces section 8.3 of the First Playable.

- Every Faction keeps a **Standing** on every Nation State and Colony: the Influence it has spent there, plus what its buildings and Occupation add. **Standings are never wiped.**
- Influence is spent in Orders from the Allotment (and from Ducats, section 4) onto any place, including one the Faction controls, where it raises the Faction's Standing.
- **A neutral place** goes to the first Faction whose Standing reaches the threshold (20 + 10 × Size for a state; 10 × Colonists for a Colony; Green Consensus × 0.75), as before. **A controlled place** goes to a rival whose Standing is **above the controller's and at least the threshold**. A Standing of zero claims nothing. Both qualifying in one turn: the tiebreak of section 6.
- **Decay** each Resolution on every Standing that received nothing that turn: **1** on a place the Faction controls, **2** elsewhere, never below zero (`influence.toml`, `decay_controlled`, `decay`).
- **Occupation** gains go into the occupier's Standing; at transfer the old controller keeps its Standing and may contest the place back.
- The state card shows both Standings and the threshold, and what a rival needs now.

## 2. Each Nation State's Influence value

Decided on [Each Nation State's Influence value](https://github.com/whaleyjoshua2/Dying-Earth/issues/34). Amends the Allotment of section 8.3.

- Each state carries an **Influence value** on its card (`nation_states.toml`, `influence`), from 2023 GDP and military spending shares: North America 8, Asia 7, Europe 5, the Middle East 4, Russia 4, South America 2, Africa 2, Australia and Oceania 2, Antarctica 0. Raising Industry Level adds 1.
- **The Allotment** each turn is 10 plus the values of every controlled state plus the buildings of section 5, times the Faction multiplier (Custodians × 1.3). The flat 3 per state is retired.

## 3. Housekeeping

Decided on [Housekeeping](https://github.com/whaleyjoshua2/Dying-Earth/issues/31) and [Replace the two cards that single out a Faction](https://github.com/whaleyjoshua2/Dying-Earth/issues/32).

- **Only Climate cards scale with the Temperature**; the Event popup says whether the drawn card is one.
- **No card targets a Faction.** Equipment Failure and Launch Failure are gone. **Launch Pad Fire** (Failure, one Nation State with a Launch Site): its Launch Site is offline until the next Resolution and every Ship due there this turn completes next turn instead; Clean Propellant delays no Ship. **Labour Dispute** (Failure, one Nation State by population): its Facilities make nothing at the next Income; Public Science spares all but one. Once each: the deck is **twenty-eight cards** (`events.toml`).
- **No destruction when a place changes hands by Influence.** The 1-in-4 roll of section 8.5 fires only when a place is attacked and when Occupation transfers it.
- The Earth Map labels and the state card say how many build slots are free; hovering a resource in the top bar lists last Income by source; End Turn with Influence unspent asks once; Armies show as shields on the Earth Map, one per Faction present, strength written on, grey for a neutral Standing Army.

## 4. Ducats

Decided on [Ducats](https://github.com/whaleyjoshua2/Dying-Earth/issues/35). Extends sections 5 and 7 with a fourth resource.

- **Ducats** sit in the Stockpile beside Materials, Fuel and Energy, and in the top bar. The Stockpile starts with none (`factions.toml`, `[start]`).
- **A controlled Nation State pays GDP × Industry Level ÷ 10 Ducats** each Income, from a **GDP** figure on its card (`nation_states.toml`, `gdp`, 2023 world share in tenths: North America 25, Asia 30, Europe 20, the Middle East 5, Russia 3, South America 5, Africa 3, Australia and Oceania 2, Antarctica 0).
- **Bank**, a Facility (25 Materials, 1 turn, 2 Energy upkeep, no Emissions): 4 × GDP ÷ 10 Ducats a turn, times the Faction output multiplier. **Trade Post**, a Module (25 Materials, 1 turn, 2 Energy upkeep): 3 × the Body's Habitat yield a turn.
- **Ducats buy** (`factions.toml`, `[ducats]`): Influence at **2 for 1**, added to this turn's Allotment and spendable at once, without cap; a Restoration step for 10; a repair point for 5, under the same conditions as a Materials repair.
- Ducats never count toward the Extraction Total.

## 5. Embassies and Relays

Decided on [Buildings that raise Influence](https://github.com/whaleyjoshua2/Dying-Earth/issues/36).

| Building | Where | Materials | Turns | Energy upkeep | Adds to the Allotment | Raises its place's Standing |
| --- | --- | --- | --- | --- | --- | --- |
| Embassy | a Nation State (one slot) | 30 | 1 | 2 | +2 | +2 a turn |
| Relay | a Colony | 25 | 1 | 2 | +1 | +2 a turn |

Both act only while standing and online; the rise counts as Influence received, so the Standing does not decay that turn; any number may stand in one place. The AI weighs both 6 (`ai.toml`, `build_influence`).

## 6. The AI

The AI's Influence target becomes the place it is closest to passing the controller on; it holds its own places when a rival's Standing approaches its own; it buys Influence with Ducats in units of five; Banks, Trade Posts, Embassies and Relays are enumerated with their weights.

## 7. Acceptance for 0.03

- `cargo build --release`; clippy clean across every target under `-- -D warnings`.
- 63 formula tests, every new rule seen red first.
- `shot:` pictures of the top bar with Ducats, a state card with both Standings, and the Army shields, looked at and in the dev diary.
- Twenty seeds of `simulate` per seating reporting the Influence game: transfers per game, states held, Ducat buys, Embassies built.

## 8. Open for the designer

Recorded on the tickets, not decided here:

1. **Contested states ping-pong**: with Standings that persist and both AIs spending, 13 to 16 transfers a game, most of them one state going back and forth. Three one-number options on #33: a margin above the controller's Standing, a wider gap between the two decay rates, or leaving the unequal Allotments of section 2 to settle it.
2. **The AI builds no Bank, Trade Post, Embassy or Relay** at the chosen weights; Colony Ships and producers take the Materials first.
3. Ducat rates for Restoration (10 a step) and repairs (5 a point) are the builder's numbers.
4. No Faction meets its Victory Condition in an AI game (a 0.02 finding, unchanged).

---

*Decisions recorded on the map's tickets remain the source of truth. This document assembles them; it does not amend them.*
