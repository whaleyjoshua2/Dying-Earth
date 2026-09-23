# Bombard

Ticket [#328](https://github.com/whaleyjoshua2/Dying-Earth/issues/328) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`bombard-button.png`](bombard-button.png) | `shot: bombard:1 stack:1 panel:0 window:1600x900`. The Custodians' Ship stack card at Mars, where they hold Orbital Control with a Frigate and the Battleship TSV Vanguard, and the Prospectors hold Olympus Mons on the ground. Under the stance row and the Attack line, the new **Bombard** heading with one button, **Bombard Olympus Mons on Mars from TSV Vanguard**, live. |
| [`bombard-hover.png`](bombard-hover.png) | `shot: ... tip:chance to burn`. The button's hover: *One Module of 4 at Olympus Mons on Mars, drawn at random, rolls a 25% chance to burn; when a Habitat burns, the Colonists beyond the room left die with it. A rung 3 offence against the Prospectors, breaking a non-aggression Accord if one stands. Needs Orbital Control here held outright; never over Earth.* |
| [`bombard-after.png`](bombard-after.png) | `shot: bombard:1 bombard:order commit:1 panel:0 window:1600x900`. The turn after the Bombard was placed and resolved: the Mars band's last row reads **A Battle here last turn: the Custodians attacked** with the Battle mark, the record a Bombard leaves as any Battle does. This one missed: Olympus Mons still lists 8 Colonists. |

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes q4 yes"*: the act as recommended, the Habitat rule for the people, no
Bombard over Earth, rung 3 with a broken non-aggression Accord. The fifth and sixth questions,
the Report and map and the computer seats, were not answered and were built as recommended: the
first is the act's interface, the second the map's standing rule that a rule change reaches the
computer seats in the same ticket.

## What was built

- **The order**: `Order::Bombard { ship, colony }`, costing nothing. Its checks: the player's
  Battleship, at a Body other than Earth, a rival's Colony at that Body, Orbital Control held
  outright by the bombarder, one order per Ship. Committed into the pending store.
- **The resolution**, a step of its own after the orbital Battles, with the checks made again
  since those Battles may have sunk the Battleship or taken the orbit: one Module drawn at random
  from the Colony's (the Core Module and the Archive never in the draw) rolls the table's
  destruction chance; a burned Module is removed, and the Colonists beyond the Habitat room left
  die with it, as the transfer rule already does. Rung 3 against the holder through `offend_by`,
  which breaks a non-aggression Accord; a burned building's war Emissions charged to the
  bombarder. The Report gets a line hit or miss, the Moment fires when anything burned, and a
  Battle record is pushed so the Battle mark and the band's row show it.
- **The computer seats**: a Battleship holding the orbit outright at a Body off Earth, with cause
  against a Colony's holder there (Relations at the war-cause bar or worse), bombards it at the
  orbital Attack's weight, one candidate per Battleship and Colony; a second order for the same
  Ship is dropped at the check.
- **The interface**: the Bombard heading on the Ship stack card with a button per Battleship per
  rival Colony, its hover naming the Module count, the chance, the people rule and the offence;
  the orders list's line; the rival paragraph's deed *bombarded X*.
- **The sweep** counts Bombards and Modules burned by the bombarder; `CONTEXT.md` gains
  **Bombard**.

## Tested

`a_battleship_bombards_a_rival_colony_from_an_orbit_it_holds`: the order is legal with the orbit
held outright, refused with it contested and refused over Earth; at a destruction chance forced
to certain, one Module burns and never the Core, the Habitat that burned takes the Colonists
beyond the room left, the holder is owed rung 3, the counters read one and one, the log names
the Habitat and the dead, and a Battle record stands at Mars with the bombarder as aggressor.
Watched to fail with the burn switched off inside the resolution, then to pass restored.
`the_computer_bombards_with_cause_and_the_orbit_held`: no cause, no Bombard; cause, a Bombard from
the Battleship at the Colony. The suite is green at 366; clippy clean under `-D warnings`.

## The batch: fired six times, burned nothing

`batch-after.txt`, 20 seeds x four seatings, against the batch after ticket #327:

| | after #327 | after #328 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 22 / 1 / 0 | 43 / 22 / 1 / 0 |
| collapses | 14 of 80 | 14 of 80 |
| Bombards, by seat | none existed | **[0, 5, 1, 0]** |
| Modules burned by them | none existed | **0** |
| Battles opened | 226 | 225 |
| warships lost | 2 | 2 |

The computer bombards: six times in eighty games, five of them the Prospectors', and every one
missed, which at a quarter a roll happens one batch in five. The act is rare because its
conditions are: a Battleship, the orbit held outright, a rival Colony at the Body and cause
against its holder, and Battleships are few (the Prospectors build most of the warships). The
column did not move. Measured behaviour under the computer's habits; the closing sweep judges
the whole.

## Looked at

The three pictures above, opened and read before this was committed. The after-picture shows a
miss; the mark and the band's row are the point of it, and they are there.
