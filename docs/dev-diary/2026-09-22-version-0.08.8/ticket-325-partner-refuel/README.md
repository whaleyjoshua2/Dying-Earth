# Refuelling at a partner's station, negotiated

Ticket [#325](https://github.com/whaleyjoshua2/Dying-Earth/issues/325) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`refuel-partner-button.png`](refuel-partner-button.png) | `shot: refuel:1 stack:1 panel:0 window:1600x900`. The Custodians' Ship stack card at Mars, where the only station is the Prospectors' *Mars Base Camp* and a Refuel Accord stands between the two. Under *Tanks*, TSV Vanguard at 0/30 Fuel carries **Refuel from the Stockpile 30**, live; TSV Valiant's is greyed at a full tank. Before this ticket the row read *no station of yours here to refuel at* and the Ship was stranded. |
| [`refuel-partner-hover.png`](refuel-partner-hover.png) | `shot: ... tip:under your Refuel Accord`. The button's hover: *At the station of the Prospectors, under your Refuel Accord: the Fuel is your own Stockpile's, drawn there.* |

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes q4 the fuel still comes from their stock pile they may simply draw on it
from another factions station q5 yes"*: under a Refuel Accord a Ship refuels at the partner's
station as at its own; no fee, the Accord is the negotiation; the computer offers Refuel in its
non-aggression offer where the other holds a station at a Body it has Ships or a Colony at and no
station of its own; the Fuel is the refueller's own Stockpile's, drawn through the other Faction's
station, and the acceptance gate stays at Wary; the sweep counts refuels at a partner's station.

## What was built

- **One predicate, three readers.** `fuels_for(station, seat)`: the station is the seat's own, or
  its director is a partner under a Refuel Accord. `refuel_station_at` (is there one at the Body
  at all) and `refuelling_station` (is one open, that is not blockaded against its holder) read
  it; so do the Refuel order's two checks, the stranded rule, the card's button and the computer's
  Refuel candidate. A station blockaded against its holder fuels the partner no more than its
  holder. The Fuel comes out of the refueller's Stockpile as it always did.
- **The computer offers it**: `refuel_worth_offering` is true where the other seat holds a station
  at a Body this seat has Ships or a Colony at and no station of its own; the term goes into the
  same offer as non-aggression (and Passage), for the reason ticket #320 found: one Accord stands
  per pair, and a non-aggression Accord struck first shuts every later term out.
- **Said and counted**: a refuel at a partner's station logs *at a partner's station*; the sim
  counts those apart and the sweep's Tanks line prints them.
- **The interface**: the Refuel button appears at a partner's station, with a hover naming whose
  station it is and whose Fuel; the stranded line and the *no station* line name the Refuel
  partner as the other rescue.
- `CONTEXT.md`: **Refuel** and the Refuel term under **Terms**.

## Tested

`a_refuel_accord_opens_a_partners_station`: a Custodian Frigate with an empty tank at Mars, where
only the Prospectors hold a station, cannot Refuel and is stranded; the computer, playing the
Custodians, offers the Prospectors an Accord carrying Refuel; struck, the Refuel is legal and the
Ship is not stranded; resolved, the tank is full and the Custodians' own Stockpile paid; a third
seat blockading the station's slot shuts it again. Watched to fail with the Accord read dropped
from `fuels_for` (the Refuel refused under the Accord), then to pass restored. The suite is green
at 362; clippy clean under `-D warnings`.

## The batch: struck, and used a little

`batch-after.txt`, 20 seeds x four seatings, against the batch after ticket #324:

| | after #324 | after #325 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 22 / 1 / 0 | 43 / 22 / 1 / 0 |
| collapses | 14 of 80 | 14 of 80 |
| refuel Accords standing at the end, four seatings | 0, 0, 0, 0 | **1, 2, 0, 2** |
| Refuel orders, of them at a partner's station | 421, none possible | 423, **2** |
| Ships stranded at the end, all seats | 30 | 29 |

The term is struck now, five times across eighty games where it was never struck before, and used
twice; one stranding fewer. Small, and honest: the computer offers Refuel only where it already
has Ships or a Colony at a Body with a rival's station and none of its own, and its transit
planning does not yet fly toward a partner's station as a refuelling point, which the ticket's
recommendation named and which is a further habit, not this rule. Measured behaviour under the
computer's habits; the closing sweep judges the whole.

## Looked at

The two pictures above, opened and read before this was committed. The first take of the hover
read *the Prospectors's station*, a wrong possessive; the sentence was rewritten and retaken.
