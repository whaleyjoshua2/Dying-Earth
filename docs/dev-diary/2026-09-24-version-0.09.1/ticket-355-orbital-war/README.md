# Ticket #355: the computer never contests an orbit

Surfaced by #343's closing sweep, not the designer's list. Re-measured before the build: over 80
games **every act given from orbit read 0** -- attacks in orbit, Bombards, Blockades,
Interceptions, Launches -- with 66 warships built.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/355#issuecomment-5841630757)
is the authority, and served as the spec.

## What the diagnosis found

A throwaway example (not committed) played twenty games and counted, per seat-turn, cause, warships
and where they sat. Three breaks, one behind another:

1. **Warships barely existed.** Seats had cause against a rival in 710 of 2,456 seat-turns and a
   warship in 70. A warship was built at the threat's lift only when an enemy stood at the yard's
   own Body.
2. **A warship at a rival's ring never blockaded.** 33 warship-turns sat at a rival station's ring
   with cause, and every one was on Hold: the Blockade candidate competed with Hold for the stack's
   one stance and lost.
3. **The orbital Attack could not see the fight.** One stance covers every orbit of a Body, and the
   computer read it as ONE contest: low orbit wherever it wanted the ground, which over Earth was
   nearly always. A fleet at a rival's ring, facing that station's Battery, saw no enemy in 127 of
   141 readings. Elsewhere it summed every rival round the planet against the one fight in front of it.

## What was built (the computer only; no rule moved)

- **Cause builds a fleet.** A seat at `war_cause` or worse with a rival who holds a station or a
  Colony builds warships at the threat's lift.
- **A Blockade IS the stance** when one applies, with cause, as #284 made the Attack.
- **A defended ring is still worth going to** when the seat's warships at that Body together clear
  the Attack bar against its Battery.
- **A station starved by a Blockade buys a Battery with Ducats**, since it makes no Widgets and is
  often the seat's only yard. A warship at the Body goes to break a Blockade of its own station.
- **The Attack is read orbit by orbit**: the odds of each melee it would open, Batteries of that
  orbit included, the worst of them read against the bar, and cause against anyone in any of them.
- **Missile Technology** (behind Hardened Hulls) closes all four pick lists, after each Faction's
  Victory gate chain. **Warship weights**: Arkwrights 3 to 4, Archivists 1 to 3; the Archivists'
  "never a fleet" is gone.
- **The sweep's foot** prints the orbital war per Faction -- orbital Battles (off Earth apart),
  Blockades, Launches, Bombards, Interceptions -- and the games each happened in.
  `orbit_attacks_off_earth` is a new counter, `serde(default)`, so no save moves.

## The bar, and where it stands

The designer's bar: orbital Battles, Blockades and Launches each in **at least 10 games of 80**.

| | before | after | bar |
|---|---|---|---|
| games with a Blockade | 0 | **28** | met |
| games with an orbital Battle | 0 | **1** (2 Battles, both over Earth) | **not met** |
| games with a Launch | 0 | **0** | **not met** |
| warships built | 66 | 159 | |

**Not forced, as the designer asked.** What stands between the figures and the bar is not an
appetite any more but two settings that are the designer's: the computer's Attack bar (`attack_odds
= 0.6`, which a lone blockader facing a bought Battery missed at 0.58), and Missile Technology's
place behind every gate chain (no Missile Carrier was built in eighty games once it moved there).
Both are on the map as a new ticket.

## The rest of the sweep

[`sweep-before.txt`](sweep-before.txt) is ticket #352's after; [`sweep-after.txt`](sweep-after.txt).

| | before | after |
|---|---|---|
| wins C / P / Ar / Ar | 5 / 39 / 1 / 2 | 3 / 38 / 3 / 3 |
| collapses | 33 | 33 |
| gates completed C / P / Ar / Ar | 76 / 62 / 59 / 47 | 73 / 57 / 51 / **36** |

The Archivists' gate falls eleven games, and the Arkwrights' eight: measured behaviour, the Materials
and Widgets now going to warships at the weights the designer set.

## Tests

- `the_computer_attacks_a_defended_ring_it_can_beat`: a seat that wants Earth's ground, with cause,
  at a rival's ring with a Battery -- one Frigate holds, five hulls attack. **Red on the old
  computer** ("a fleet that beats the Battery takes the ring"), green on the new.
- `the_computer_orders_a_blockade_where_its_warship_sits_in_a_rival_stations_slot` now also checks a
  calm seat does NOT blockade, since the old candidate needed no cause.
- Clippy gate clean; 475 + 8 + 6 pass.
