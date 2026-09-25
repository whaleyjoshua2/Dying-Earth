# Ticket #346: Battles that cost Fuel. The build specification

Authority: the resolution comment on
[ticket #346](https://github.com/whaleyjoshua2/Dying-Earth/issues/346), decided by the designer on
2026-09-24. Where this file and that comment disagree, the comment wins and this file is wrong.

## Prior art, searched before writing

Every claim below was read out of the tree on 2026-09-24, at the line given.

- **A Battle spends nothing.** `ship_melee` (`resolution.rs:647`) charges offence, builds Combatants
  through `unit_combatant` (`:519`), runs the melee and writes the Report. The melee writes only
  `damage` and `escaped` back to a Ship: **no Fuel, no stance, no orbit, no cargo**.
- **A Ship has no defence stat.** `ship_combatant` (`resolution.rs:537`) builds one Combatant from
  `ship_strength`, the card's hit points, the hull's damage, the card's pursuit and Evade, and takes
  **no `defending` flag**. `ship_strength` (`state.rs:2534`) is the card's strength plus Hardened
  Hulls, and **returns 0 early when the card's strength is 0**.
- **An Army does have one, and it is the precedent for a side-dependent figure.**
  `army_combatant(id, defending)` (`resolution.rs:549`) picks `army_defended_strength` or
  `army_strength` from that flag, which `army_melee` passes as `!aggressor`. `unit_combatant` passes
  `false` for a Ship's party.
- **Strengths**: Frigate **3**, Battleship **7**, and **0** for the Colony Ship, the Carrier and the
  Missile Carrier (`units.toml`). Only two of the five hulls have any strength at all.
- **The Fuel model**: every hull's tank is **30**; a transit costs 6 to the Moon, 16 to Venus, 20 to
  Mars, 24 to Phobos or Deimos and 1 between sibling moons; an orbit change costs
  `tables.orbit_change_fuel` = 1 since ticket #335; a Refuel fills only at a station of the seat's
  Faction or a Refuel partner's, **in that station's own orbit**, and never at a blockaded one
  (`orders.rs:1280-1300`).
- **The three doors the Fuel bar touches**, all of which read `is_warship()` and none of which reads
  Fuel today:
  - `orbital_control` (`state.rs:3520`): exactly one seat with an unescaped warship in low orbit, and
    no rival Battery denying it.
  - `orbit_uncontested` (`state.rs:3537`): **no rival** unescaped warship in that orbit, and no rival
    Battery covering it. This is what a station's own ring asks.
  - `blockading` (`state.rs:3620`): an unescaped warship on the Blockade stance.
  - **Intercept** is a stance read at `resolution.rs:197`, filtering the seat's unescaped Ships in
    that orbit on `Stance::Intercept`.
- **`stranded` (`state.rs:2125`) is a live computation, not a state, and does nothing mechanically.**
  It is a warning on the interface and a figure in the sweep. A stranded Ship still fights, still
  blockades, still intercepts and still holds Orbital Control **today**; R4 below is what changes
  that, and it changes it through Fuel rather than through `stranded`.
- **Measured, eighty games on the current build** (`sweep -- 20 --seatings --balance --steps=300
  --sinks=6`): **one orbital Battle in the whole batch** (`attacks in orbit` reads `[1,0,0,0]` in the
  third seating and `[0,0,0,0]` in the other three); **no Bombard in any seating**; over three
  hundred Battles opened, essentially all on the ground; **stations standing off Earth 1, 2, 5 and 1**
  per seating; 198 Refuel orders, **none at a partner's station**; 2 Ships stranded at the end.
- Nothing found that does what is specified; the finding is a negative.

## The rules

### R1. The charge

- `units.toml [melee]` gains **`battle_fuel` = 2**.
- At the head of `ship_melee`, **every Ship named in any party pays `battle_fuel` from its own tank**,
  once per Battle, floored at 0. Whether it was struck, whether it was armed, whichever side it is
  on, and whether or not it was the aggressor.
- **A Battery pays nothing**: it has no tank, and a `UnitRef::Battery` is skipped.
- An Army is untouched: `army_melee` is not a space Battle and Armies have no tanks.

### R2. A hull that could not pay fights at half strength

- `units.toml [melee]` gains **`dry_strength_share` = 0.5**.
- A Ship whose tank held **less than `battle_fuel` when the charge was taken** fights this Battle at
  `floor(ship_strength * dry_strength_share)`, **whichever side of the Battle it is on**.
- **All or nothing**: a hull short by one Fuel takes the same penalty as one at nought.
- A hull whose strength is already 0 is unchanged, since half of nought is nought. **No test may
  assert a penalty on a Colony Ship, a Carrier or a Missile Carrier**, and one asserts the absence.
- **It follows the hull while it stays dry**, with no rule of its own: the penalty is computed from
  the tank at each Battle, so a hull that never refuels fights halved every turn.
- **Implementation note, not a rule**: `ship_combatant` must learn the hull's Fuel. The cleanest
  shape is to take the penalty as a parameter, computed by `ship_melee` from the tank **as it stood
  before the charge**, so the charge and the penalty can never disagree about which hulls were dry.

### R3. The Fuel bar on a warship's work

A warship must hold **at least `battle_fuel`** in its tank to do any of four things. Below that bar
it does none of them:

- **hold Orbital Control** (`orbital_control`, `state.rs:3520`);
- **contest an orbit** (`orbit_uncontested`, `state.rs:3537`) — a dry rival warship is **not** a
  rival warship for this test;
- **blockade** (`blockading`, `state.rs:3620`);
- **intercept** (`resolution.rs:197`, and the stance's own gate in `orders.rs`).

- **The bar is the Battle charge, not "more than nought"**, so a warship holds an orbit exactly as
  long as it could still fight for it.
- **Read live**, as Orbital Control already is: a hull that empties its tank in a Battle loses the
  orbit at that moment, not a turn later. **A fleet that spends its last Fuel winning a Battle can
  lose the orbit it just won to a fresh Frigate arriving next turn.** This is intended and a test
  says so.
- A Missile Carrier is not a warship and never was; it is untouched by R3 and pays R1 like any hull.

### R4. `stranded` is NOT changed

The zero-Fuel trap stays, at the designer's word: *"for now its stranded."* A Ship at exactly nought
with a station of its own in another orbit remains stranded, because the orbit change costs 1 Fuel.
**No line of `stranded` moves in this ticket**, and a test pins its behaviour so a later ticket can
change it deliberately.

### R5. The computer

- A seat **weighs** the Fuel a Battle would cost against the prize and will not open one that strands
  its fleet for nothing. A weight in `ai.toml`, not a prohibition.
- The sweep counts, by seat: **Fuel burned in Battle**, and **hulls left dry by a Battle**.

### R6. What the game says

- The Battle line in `report.toml` says what the Battle cost in Fuel and names any hull that fought
  dry.
- `src/ui.rs`: the attack hover says the cost **before** the player commits, and a warship at or
  under the bar says on its row what it has lost — it cannot hold an orbit, blockade or intercept,
  and it will fight at half strength.
- `CONTEXT.md`: **Battle**, **Tank**, **Stranded**, **Orbital Control**, **Blockade** all move.

## Error cases

- `battle_fuel` or `dry_strength_share` missing from `[melee]`: the load check refuses the table.
- `dry_strength_share` outside 0..1: the load check refuses the table.
- A Battery in a party: skipped by the charge, never panics.

## Out of scope

- Any change to `stranded` (R4), to a ground Battle, or to what a Battle destroys.
- Making the computer fight in orbit at all. That is
  [#355](https://github.com/whaleyjoshua2/Dying-Earth/issues/355), and this ticket will measure
  almost nothing until it is answered.
- `SAVE_VERSION` does not move: no field is added to any saved structure.

## Refutation

The specification is wrong if a Battery is charged Fuel; if a Ship that never fought in an orbital
Battle escapes the charge; if a Colony Ship, a Carrier or a Missile Carrier suffers a strength
penalty; if a dry hull fights at full strength when it opens the Battle; if a warship with less than
the charge in its tank holds Orbital Control, contests an orbit, blockades or intercepts; if a
warship with exactly the charge cannot do those things; if `stranded` behaves differently than it
does on `main`; or if the penalty and the charge disagree about which hulls were dry.

## Red witnesses owed

One test per rule R1 to R3, R5 and R6 in `engine/tests/formulas.rs`, each watched red against the
rule reverted on purpose, with the failing assertion quoted. R4 owes a pinning test that fails if
`stranded` changes. The Refutation's eight scenarios are the ones to write first.
