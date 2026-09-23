# A Module for Colonies and stations to defend themselves

Ticket [#324](https://github.com/whaleyjoshua2/Dying-Earth/issues/324) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`mars-battery-band.png`](mars-battery-band.png) | `shot: battery:1 hab:ground panel:0 window:1600x900`. Olympus Mons on Mars, a Custodian Colony with a **Battery** standing among its Modules, two hits on it: the tile wears the new turret glyph and the label *Battery 4/6*. In the orbit band at the top left, under the four stacks, the new rows: **Custodians: 1 Battery, strength 4** and **Orbital Control: nobody, a Battery stands**. |
| [`mars-battery-strip.png`](mars-battery-strip.png) | `shot: ... habtile:3`. The Battery's tile clicked: the strip reads *Battery: strength 4, 4 of 6 hit points, 3 Energy upkeep*, with Mothball and Decommission as any Module has, and beneath them **Repair fully 10** Materials and *Repair fully with Ducats 20*, greyed at ten Ducats in the purse. |
| [`battery-tile-hover.png`](battery-tile-hover.png) | `shot: ... tip:strength 4`. The tile's hover: the figures, the Module rules every tile carries, and the Battery's own paragraph: in the line of any Battle in this orbit, on Hold, never disengaging; while it stands no rival holds Orbital Control here; repaired with Materials here; at 6 hits destroyed. |

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes q4 yes q5 yes q6 yes"*, every one as recommended: one Module, the Battery,
on a Colony or a station and no Facility twin on Earth; a party in the orbital melee at its Body
on Hold, not a ground defender; while one stands no rival holds Orbital Control there and the owner
gains none by it; the figures from the 0.08.5 file; any rival stack on Attack fights it at rung 3;
the computer wants one where a rival warship stands or a rival Carrier is inbound, at the
Barracks' weight, and reads its strength in its odds.

## What was built

- **The kind and its card**: `ModuleKind::Battery`, appended last as the Core Module was, with
  `strength` and `hit_points` fields new to the Module card (nought for every other kind, and a
  card with strength and no hit points is refused at load). The station's allow-list takes it.
  25 Materials, 1 turn, 3 Energy, strength 4, hit points 6 in `modules.toml`.
- **Damage on a Module**: a Battery carries `damage` (a save field with a default, so old saves
  load). It is repaired with the ordinary Repair order through a new `UnitRef::Battery`, the
  owner's alone, at its own Colony with no yard needed, for no more than its damage; Ducats work
  as they do for a Ship.
- **In the line**: the orbital melee's parties are units now, not Ships, and a seat's working
  Batteries at the Body stand after its Ships. A rival stack on Attack at a Body with nothing but
  a Battery in it fights the Battery. It never rolls to disengage (the flag Dig In sets on an
  Army says exactly that), has no pursuit, and Hardened Hulls does not reach it. Shot to its hit
  points it is removed from its Colony, counted against its holder, and said in the log; the
  Battle line names it *the Battery at Olympus Mons on Mars* and, when nobody holds Orbital
  Control after the fight and a Battery is why, says so. An Interception is not a Battle it
  joins: that is a strike on arrivals, not an Attack at the Body.
- **What it denies**: `orbital_control` returns nobody while a working Battery of any other seat
  stands at the Body, so `may_land` and the ground Blockade's starving follow without a change;
  `slot_blockaded_against` and `starved_by` return nothing against a seat with a Battery at the
  Body. Mothballed or offline, it neither fires nor denies, as no Module that is not working does
  anything.
- **The computer seats**: a Battery candidate at a Colony where a rival warship stands at the
  Body or a rival Carrier is inbound, one per Colony, at the Barracks' category and the threat
  term; a rival's Batteries are in `enemy_ship_strength`, so the orbital Attack gate's odds read
  them; a rival's Battery counts as an enemy in orbit, so a stack may open a Battle to clear it,
  with cause from the Battery's holder; and a stack does not order a Blockade against a station
  whose holder has a Battery at the Body, since it would shut nothing.
- **The interface**: a hand-drawn turret glyph (`module_battery.svg`, in `DRAWN`, no credit
  owed), the tile's label with its hit points when damaged, the hover's rule paragraph, the strip's
  Repair buttons, the orbit band's Battery rows and the reason on the Control line, and the Repair
  order's line naming *the Battery at ...*.
- **The sweep** counts Batteries lost in a Battle by seat and Batteries standing at the end.

## Tested

`a_battery_denies_orbital_control_and_the_blockade_and_falls_in_a_battle` (a lone rival warship
holds Control and blockades the slot; a Battery denies both and starves nothing; mothballed it
denies nothing; the owner may repair it and a rival may not; three rival Frigates on Attack fight
it with no Ship of the owner's present, it is named in the line at its card's strength, one hit
from gone it is gone and counted, and Control is the rival's again) was watched to fail with the
Battery dropped from the line (no Battle was fought at all, there being one party) and to pass
restored. `the_computer_wants_a_battery_where_a_rival_warship_stands` (no rival warship, no
Battery; a rival Frigate in orbit, a Battery ordered; the rival's odds read its strength). The
suite is green at 361; clippy clean under `-D warnings`.

One measured fact from writing the second test: on a fresh station with two free places, a
Battery at the Barracks' weight loses to the opening Habitat and the Trade Post, and is built
once those stand and a place is free. That is the weight as decided, not a defect; the test gives
the station room.

## The batch: they are built, and shot down

`batch-after.txt`, 20 seeds x four seatings, against the batch after ticket #322 (ticket #323 had
none, being interface alone):

| | after #322 | after #324 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 23 / 0 / 0 | **43 / 22 / 1 / 0** |
| collapses | 14 of 80 | 14 of 80 |
| Batteries standing at the end, by seat | none existed | **[29, 2, 5, 2]**, 38 in all |
| Batteries lost in a Battle, by seat | none existed | **[2, 1, 0, 2]** |
| Battles opened | 216 | 223 |
| attacks in orbit | 48 | 52 |
| Blockade-turns suffered | 158 | 154 |
| marches on held Regions | 332 | 347 |
| places taken by force | 165 | 170 |

The computer builds Batteries and rivals shoot them down: thirty-eight stood at the end of eighty
games, twenty-seven of them the Custodians' in the Arkwrights-first seating, and five fell, across
three seats. The Blockade-turns barely moved, which says the Batteries stood mostly where nobody was
blockading, or fell before a Blockade was lifted by them; the column moved one win from the
Prospectors to the Arkwrights, within the noise of a twenty-seed batch. Measured behaviour under
the computer's habits; the closing sweep judges the whole.

## Looked at

The three pictures above, opened and read before this was committed. The turret glyph reads as a
gun on a mount at the tile's 48 pixels; it is the game's own drawing and can be replaced if the
designer wants a better one.
