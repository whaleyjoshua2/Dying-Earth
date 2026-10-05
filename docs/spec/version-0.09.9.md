# Dying Earth — version 0.09.9, the settlement version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.9](https://github.com/whaleyjoshua2/Dying-Earth/issues/488), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-05-version-0.09.9/`](../dev-diary/2026-10-05-version-0.09.9/).

**What the version is.** Version 0.09.8 with the designer's list. *(Summary and win column written
when the version closes.)*

## 1. Four Colonists to found

The authority is [ticket #489](https://github.com/whaleyjoshua2/Dying-Earth/issues/489).

**Every new Colony or station opens with four Colonists**, however it is made. The figure is
`found_with = 4` under `[emigrants]` in `factions.toml`. Materials prices are unchanged.

| How it is made | Where the four come from | Before |
|---|---|---|
| A Colony Ship unloads onto a free ground slot, or into a free ring off Earth or at Earth L4 or L5 | the Ship: the Unload must put down at least 4 | at least 1 |
| Pioneers sent to Antarctica by sea, into a free slot | the Region they sail from: at least 4 sent | at least 1 |
| A station over Earth, built from a Launch Site | 4 Pioneers waiting in a Region of the builder's with a working Launch Site | nobody |
| A station over another Body, built from a ground Colony there | 4 Colonists from that Colony | nobody |
| A ground Colony built from a station over that Body | 4 Colonists from that station | nobody |

- **A Colony Ship lands exactly four**, as it already could not land more: a new Colony's Core
  holds four and the rest stay aboard.
- **A Colony or station that builds one keeps enough people.** After the four leave it must
  still hold at least as many Colonists as its Modules in slots (Core and Archive not counted,
  a Module mothballed or building counted, as on the card's "Modules {used}"), and never fewer
  than four. So a source needs that number plus four.
- **Which source.** A station order and a station-built Colony name the place their four come from.
  The game chooses for the player: the qualifying Region or Colony with the most to spare.
- **People ordered once are ordered once.** A source's spare counts what the turn's other
  orders already take from it: Loads, lifts, sends by sea or down, and other builds.
- **When they leave.** Builders leave their source when the turn is ended and arrive as the
  place is built at Resolution, as Pioneers sent by sea do. If the place is not built (another
  Faction took the slot), they go back where they came from.
- **A lift from a Launch Site** carrying the four is the station build itself: no second launch,
  no second emission.
- **The hover.** Every founding button's price reads "4 Colonists" beside the Materials. A
  refusal says what is short: "needs 4 Colonists", or "needs 4 Colonists to spare".
- **The computer seats** found only with four ready, by the same rule: a Colony Ship sets down
  four or does not found, and a Colony or station builds only with four to spare. A Colony Ship
  of theirs holding fewer than four sails only to a place of their own with room, or stays to
  load. **A seat with no station over Earth recruits the four a station takes**: their recruiting
  wanted room to put people, and with no station there was none, so a seat without one never
  recruited and never built one.
- **The Arkwrights start with 4 Pioneers waiting** in their start Region, where it was 2, so their
  first station can still be ordered on turn 1. The gift takes no population, as before.
- **Not changed:** unloading or sending into a Colony that already stands, the Arkwrights' Colony
  Ship carrying 8, and the stations every other Faction starts with.
- **Saves:** the source rides on the order and on the pending build; `SAVE_VERSION` stays at 9.

**Measured** (80 games, the standing cell,
[`after-489.txt`](../dev-diary/2026-10-05-version-0.09.9/sweeps/after-489.txt)):

| | 0.09.8 | after this | the ideal |
|---|---|---|---|
| Custodians | 9 | 0 | 15 |
| Prospectors | 27 | 24 | 15 |
| Arkwrights | 18 | 33 | 15 |
| Archivists | 7 | 6 | 15 |
| collapses | 17 | 16 | 20 |
| Arkwrights meet their Opening Objective | 29 of 80 | 55 of 80 | |
| stations off Earth at the end | 94 | 36 | |

The column moves far past noise. Stations off Earth fall to a third: a Colony or station must
hold eight before it builds one. The Arkwrights, whose Colony Ship carries eight and who start
with their four, reach the Moon first far more often, and their Opening Objective, a Colony on the
Moon, is met in 55 games where it was 29. The Custodians win none: traced on one game they still
had their twelve off Earth by turn 8, but never held Stabilization; the game diverged through Tech
picks and cards, not through any one rule, so their loss is **not traced** past that.
The dev diary's README has the trace.

With the Arkwrights starting at two Pioneers and the computer's recruiting fixed, the column read
5 / 23 / 24 / 5 and 23 collapses; at two before the fix, 8 / 45 / 1 / 2 and 24.
