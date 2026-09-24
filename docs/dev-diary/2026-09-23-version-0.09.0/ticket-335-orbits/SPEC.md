# Ticket #335: orbits. The build specification

Authority: the resolution comment on
[ticket #335](https://github.com/whaleyjoshua2/Dying-Earth/issues/335), decided by the designer on
2026-09-23. Where this file and that comment disagree, the comment wins and this file is wrong.

## Prior art, searched before writing

- **This tree**: a Ship already carries `slot: Option<u32>` (`engine/src/state.rs:480`), *the Orbital
  Slot this Ship sits in, chosen with the leg that brought it*, set only by
  `Order::Transit { ship, to, slot }` (`orders.rs:59`). **Every transit the interface sends passes
  `slot: None`** (`src/ui.rs:3043, 6184, 6192`), so no human player has ever been able to Blockade:
  the order demands a warship already sitting in a rival's slot (`orders.rs:1201-1210`). Battle
  parties form per Body (`resolution.rs:201-231`). Orbital Control is one seat alone with a warship
  at the Body and no rival Battery (`state.rs:3156-3165`). `may_land` reads Control
  (`state.rs:3197`); `may_unload_into` reads the slot for a station and `may_land` for the ground
  (`state.rs:3270`); `refuelling_station` is Body-wide (`state.rs:3262`); `starved_by` reads the
  slot for a station and Control plus any blockading stack for the ground (`state.rs:3242`);
  `battery_stands_against` is Body-wide (`state.rs:3186`). Bombard wants Control outright and does
  **not** exclude `in_orbit` (`orders.rs:1165-1191`), so a rival station off Earth can be bombarded
  today. A Battle keys on `ReportPlace::Body` (`report.rs:16-20`).
- **The figures**: a tank holds 30 Fuel; a sibling hop between two moons is 1 Fuel
  (`bodies.toml:9`), a local hop 2 to 6, Earth to Mars 20 to 24, Venus 16.
- Nothing found that does what is specified; the finding is a negative.

## The model

**A Body's orbits** are **low orbit** plus one per **Orbital Slot** (Earth 5, Moon 2, Mars 3,
Phobos 1, Deimos 1, Venus 3: twenty-one orbits on the board). Every Ship at a Body sits in exactly
one of them. There is no longer a Body at large: what `slot: None` meant is now **low orbit**, named
and drawn.

### R1. Where a Ship is

- `Ship.slot: Option<u32>` keeps its shape, `None` being low orbit, and the tree stops calling that
  "the Body at large" anywhere a player can read. A helper names an orbit for the interface and the
  Report (`Mars, low orbit` / `Mars, at Tiangong-2`).
- **A transit names its destination orbit before leaving**: `Order::Transit { ship, to, slot }` is
  unchanged in shape and the interface now fills `slot`.
- **A new Ship starts in the orbit of the Shipyard that built it**: a station's yard puts it in that
  station's orbit, a ground Colony's yard in low orbit.
- **Starting Ships** begin in their Faction's station's orbit where it holds one at that Body, and
  in low orbit otherwise.
- **A Ship with no orbit named arrives in low orbit.**

### R2. Changing orbit

- A new order, `Order::ChangeOrbit { ship, slot }`, legal for a Ship at a Body (not in transit) with
  an order-free turn, to an orbit at that Body that exists. It costs **`orbit_change_fuel = 1`**
  (`bodies.toml`) from the Ship's own tank and is refused below it.
- Resolved with the transits, before the Battles, so a Ship that changes orbit fights in its new one.

### R3. What each orbit is for

- **Low orbit** is needed to touch the ground: landing an Army at a ground Colony, unloading
  Colonists into one, founding a ground Colony, Bombarding one, and receiving a lift from a Launch
  Site.
- **A station's own orbit** is needed to touch that station: unloading into it, **refuelling at it**,
  blockading it, attacking it, Bombarding it.
- Founding a station is unchanged: a foothold and an unblockaded slot, with no Ship in the slot
  (Venus still wants a Ship somewhere at the Body).

### R4. Orbital Control, and the Battery

- **Orbital Control is of low orbit**: the one seat with a non-escaped warship in low orbit and no
  rival Battery denying it. It gates the ground as before.
- **A Battery covers its own orbit**: a station's Battery denies its station's orbit, a ground
  Colony's Battery denies low orbit. This narrows ticket #324, at the designer's word.

### R5. Battles, Intercept, Blockade

- **Battle parties form per orbit.** An Attack fights the orbit the stack sits in: rival Ships in
  that orbit and, in a station's orbit, that station's working Batteries; in low orbit, the ground
  Colonies' Batteries. Two Battles at one Body are two records.
- A Battle keys on the **orbit**: `ReportPlace` gains an orbit form, so the map marks each.
- **An Attack on a station that cannot fight back does nothing beyond the Battle.** Bombard is the
  second action that breaks it.
- **Intercept catches only arrivals into its own orbit.**
- **A Blockade shuts the orbit it is given in.** A station starves under a Blockade of its own
  orbit. A ground Colony starves while a rival holds Orbital Control outright **and** has a stack on
  Blockade **in low orbit**.
- Passage is unchanged: a partner is no target for Intercept and is not shut out by a Blockade.

### R6. Bombard

- **The orbit you are in is the orbit you must hold.** To Bombard a ground Colony, the Battleship
  is in low orbit and its Faction holds Orbital Control there outright. To Bombard a station, the
  Battleship is in that station's orbit with no rival warship and no rival working Battery in it.
- Everything else about Bombard stands: a Battleship, one per Battleship a turn, never over Earth's
  ground, one Module drawn at random rolling the destruction chance, rung 3, the Report line and
  the Battle mark. A station off Earth is a lawful target, as it is today.

### R7. The computer seats

- Transits to low orbit by default; to a rival station's orbit to blockade or attack it; wants
  Control of low orbit where it wants the ground. Changes orbit rather than flying away when what it
  wants is at the same Body. The sweep counts orbit changes and Blockades by a human-reachable path.

### R8. The maps

- The Body Surface Map draws low orbit as its own ring, innermost, with every stack in it; the orbit
  band names each stack's orbit. The Solar map's stack label says the orbit. A right-click on a
  station glyph sends the armed stack to that station's orbit; on the Body, to low orbit.

## Error cases

- `ChangeOrbit` to the orbit the Ship is already in: refused.
- `ChangeOrbit` to a slot that does not exist at that Body: refused.
- `ChangeOrbit` with fewer than `orbit_change_fuel` in the tank: refused, naming the figure.
- A transit naming a slot that does not exist at the destination: refused.

## Out of scope

- Any change to what a Blockade or a Battle does once it lands, beyond the orbit it reaches.
- Arriving with a stance, and a prize rule for stranded Ships: both on the map's fog.

## Refutation

The specification is wrong if a Ship built at a station's Shipyard does not start in that station's
orbit; if a stack on Attack in low orbit fights a station's Battery in a high orbit; if a Ship in low
orbit can refuel at a station in another orbit; or if a human player still cannot give a Blockade
after a transit that names a rival station.

## Red witnesses owed

One test per rule R1 to R5 and R7 in `engine/tests/formulas.rs`, each watched red against the 0.08.8
rule or the rule reverted on purpose, with the failing assertion quoted in the lane's report.
