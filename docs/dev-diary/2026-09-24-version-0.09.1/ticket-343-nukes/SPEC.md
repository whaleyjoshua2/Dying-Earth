# Ticket #343: the Missile Carrier. The build specification

Authority: the resolution comment on
[ticket #343](https://github.com/whaleyjoshua2/Dying-Earth/issues/343), decided by the designer on
2026-09-24. Where this file and that comment disagree, the comment wins and this file is wrong.

## Prior art, searched before writing

Every claim below was read out of the tree on 2026-09-24, at the line given.

- **Nothing in this game destroys a place.** `destruction_rolls` (`resolution.rs:943`) has exactly
  one caller, the Occupation's three-turn transfer (`resolution.rs:1331`), and rolls
  `influence.destruction_chance` = 0.25 per building. It already takes a `Place`, already handles a
  Region (its Facilities) and a Colony (its Modules), already trims Colonists to the Habitat room
  left, and already charges `war_ppm_per_building` through `charge_war`. **It does not exempt the
  Core Module or the Archive**, so the nuke cannot call it unchanged.
- **`charge_war` (`resolution.rs:454`) returns at once unless `on_earth`.** The Earth-only rule the
  designer asked for is therefore already the engine's rule, and a Launch off Earth needs no new
  gate to poison nothing. This is also why a Bombard's emissions are always discarded: Bombard is
  refused over Earth.
- **Bombard** (`orders.rs:1254`, `resolution.rs:1002`) burns **one** drawn Module at 0.25, Core and
  Archive left out, kills only Colonists a burned Habitat no longer houses, is **free**, and charges
  rung 3. Its gate is the shape R3 copies: your Ship, the right kind, not in transit, in the orbit
  that touches the target, holding it outright, a rival's place, one order per hull. The gate is
  **read again at resolution**, since the Battles just fought may have sunk the hull or taken the
  orbit.
- **The escort rule** (#326, `combat.rs:81-84` `escorted`, `combat.rs:129-138` `random_engaged`)
  already draws hits only among a party's engaged **armed** units while it has any, and strikes an
  unarmed hull only once none remain. A Combatant's `armed` flag is set from `s.kind.is_warship()`
  at `resolution.rs:540`. **This is the counter the designer asked for and it needs no new code.**
- **Offence** is one call, `offend_by(offender, victim, weight)` (`state.rs:4089`), which breaks a
  non-aggression Accord itself and adds the weight to `relations.owed`. Weights today are 1 to 3;
  the per-turn sum is clamped by `relations.turn_cap` (`state.rs:4328`), which is 8, so a weight of
  4 fits under it. **A rung is a weight, not an enum**, so rung 4 is data, not a new type.
- **The Sink** is `climate.natural_sink` = 6.0 (`climate.toml:64`). The Sink Weakens **assigns**
  `self.climate.natural_sink = b.sink_after` (`climate.rs:750`), with `sink_after` = 4.0
  (`climate.toml:149`), and the projection mirrors it as `sink = b.sink_after + e.scrubbers`
  (`climate.rs:827`), where its `sink` already carries the scrubbers from `climate.rs:802`.
- **No unit card carries a Tech gate.** `needs_tech` exists on `FacilityCard` (`data.rs:145`) and
  `ModuleCard` (`data.rs:280`) only; `UnitCard` (`data.rs:296-311`) has no such field, and
  `Order::BuildShip`'s gate (`orders.rs:1047`) checks only that the kind is a Ship and that the
  Colony has a working Shipyard. **The field is new.**
- **A Tech's `effect` string is display only**, read at `src/ui.rs:1856` and `src/ui.rs:7421` and
  nowhere else; behaviour is keyed by `TechId` in code. So a Tech that only unlocks a unit needs no
  effect handler.
- **Nothing is consumable.** No order consumes a unit and no unit carries a one-shot. A Warhead is
  the first.
- `UnitKind` (`ids.rs:457`) has five variants and `UnitKind::SHIPS` (`ids.rs:467`) lists the four
  Ships; `is_warship` (`ids.rs:477`) has nineteen callers across `ai.rs`, `orders.rs`,
  `resolution.rs`, `state.rs`, `shot.rs` and `ui.rs`.
- Nothing found that does what is specified; the finding is a negative.

## The rules

### R1. The Missile Carrier

- `UnitKind::MissileCarrier`, **appended last** (`ids.rs:457`), added to `UnitKind::SHIPS` so it is
  a Ship, with its row appended last in `units.toml` **in enum order**, since the tables index by
  discriminant.
- Figures from the resolution's table: 60 Materials, 16 Widgets, **strength 0, hit points 3,
  pursuit 0**, tank 30, carries no Colonists and no Army.
- **`is_warship()` is false.** It has no strength, it cannot hold Orbital Control, and the escort
  rule must treat it as the unarmed hull it is. Every one of the nineteen callers is read under that
  reading and the lane says so, naming any that surprised it: Orbital Control
  (`state.rs:3400, 3415`), the Blockade gate (`orders.rs:1995`, `state.rs:3498`), Intercept
  (`orders.rs:1311`), `armed` in the melee (`resolution.rs:540`), and the AI's ten.
- A hull carries `warhead: bool`, true at the build, false once fired. In the save.

### R2. The Tech

- A new `TechId` variant and a row in `techs.toml`: **rung 3, cost 48**, branch **Propulsion**,
  `needs = ["hardened_hulls"]`. Branch and prerequisite are an **implementation choice, not the
  designer's** (rung and cost are theirs), named in the Built comment for correction; Propulsion is
  where the tree's only other combat Tech sits. Its `effect` line says in plain words that it is the
  game's **first weapon Tech**, since the tree is shared and every seat sees it researched.
- **`needs_tech: Option<TechId>` is added to `UnitCard`** (`#[serde(default)]`, as the Facility and
  Module cards carry it) and checked in `Order::BuildShip`'s gate (`orders.rs:1047`), in the shape
  `orders.rs:842` and `orders.rs:1012` already use. Every other unit row leaves it absent, so
  nothing else changes.

### R3. Launch

- `Order::Launch { ship, target: Place }`, so a Region, a ground Colony and a station are all lawful
  targets, `Place` being exactly those two things (`ids.rs:761`).
- Gate, mirroring Bombard's (`orders.rs:1254-1293`): your Ship; a Missile Carrier; **carrying its
  Warhead**; not in transit; **in the orbit that touches the target** (`colony_orbit` for a Colony,
  low orbit for a Region, which is Earth's ground); **holding that orbit outright**
  (`orbital_control` for low orbit, `orbit_uncontested` for a slot); the target directed by a rival;
  one order per hull per turn. **Re-checked at resolution**, as Bombard's is, for the same reason.
- **A Region is a lawful target and Earth is not excepted.** Bombard's `no Bombard over Earth` rule
  is Bombard's, and does not carry over; a nuke on Earth is the point.
- Free to order; the cost was paid at the build and is paid again at the rearm.

### R4. What a Launch does

Resolved after the orbital Battles, beside `resolve_bombards`.

- **Every building at the place rolls `nuke.destruction_chance` = 0.60**, independently. At a Colony
  that is its Modules, **Core Module and Archive exempt**; at a Region its Facilities.
  `destruction_rolls` is the shape but not the function: it rolls one fixed chance and exempts
  nothing. Give it the chance and the exemption as parameters and let the Occupation pass its
  present values, so **the Occupation's behaviour is unchanged** and one function does both.
- **People**: a share between `nuke.people_min` 0.40 and `nuke.people_max` 0.60 dies, drawn once per
  strike from the game's own RNG. At a Colony that is its Colonists; at a Region its `population`
  (`state.rs:216`), which is a float in millions since #333. At a Colony the survivors are then
  trimmed to the Habitat room left, as `destruction_rolls` already trims them.
- **At a Region only**: the **Standing Army is destroyed**, counted into `standing_armies_lost`
  (`state.rs:1030`) as any other loss is; and the **Industry Level falls by `nuke.industry_lost`**
  = 1, floored at the state card's own `industry_level`, so a Region never falls below the board it
  started on. It may be raised again by `Order::RaiseIndustry`; this is a setback, not ruin.
- **Rung 4**: `offend_by(seat, holder, tables.nuke.offence)` with `offence` = 4 in data. The Accord
  break and the permanent scar come free with the call.
- **On Earth only**, which `charge_war` already enforces: `nuke.war_ppm` = 20.0 charged to the
  firing seat, and `natural_sink += nuke.sink_rise` = 0.25, permanently. The Sink rise is written
  beside the `charge_war` call and takes the same `on_earth` test, so the two can never disagree.
- The Warhead is spent whether or not anything burned: `warhead = false`.
- A Report line hit or miss, a Moment, and the Battle mark at the place, as a Bombard leaves.

### R5. Rearming

- `Order::Rearm { ship }`, legal for a Missile Carrier without its Warhead, at a Colony or station
  of its Faction **with a working Shipyard**, in that place's own orbit. Costs
  `nuke.rearm_materials` 40 and `nuke.rearm_widgets` 8. Resolved with the builds.

### R6. The Sink Weakens subtracts

- `climate.rs:750` becomes
  `self.climate.natural_sink = (self.climate.natural_sink - b.sink_cut).max(0.0)`, with `sink_cut`
  = 2.0 in `climate.toml` replacing `sink_after` = 4.0.
- The projection at `climate.rs:827` becomes `sink = (sink - b.sink_cut).max(0.0)`. **Note the
  scrubbers**: today's line re-adds `e.scrubbers` because it assigns; the subtraction must not,
  since the projection's `sink` already carries them from `climate.rs:802`. Getting this wrong
  double-counts the scrubbers and the forecast lies.
- On an untouched game the outcome is identical, 6.0 to 4.0, so **no existing measurement moves**.
  What changes is that a Sink which has been raised keeps what it was given.

### R7. The computer

- A seat with cause wants a Missile Carrier where it holds a place it cannot take, and fires at the
  rival holding the place it most wants and cannot take. Weights and thresholds in `ai.toml`, read
  beside the existing war appetites.
- The sweep counts, by seat: Missile Carriers built, Launches fired, buildings burned, people
  killed, Industry Levels lost; and the Natural Sink at the end of each game.

## Error cases

- Launch from a hull with no Warhead: refused, naming the rearm.
- Launch at a place the firing seat directs, or at a neutral: refused.
- Launch without the orbit, or without holding it: refused, naming which.
- Rearm at a place with no working Shipyard: refused, naming that.
- A Missile Carrier ordered before the Tech: refused, naming the Tech.
- A `nuke` figure missing from the table: the load check refuses the table.

## Out of scope

- Any change to what a Bombard, an Occupation or a Battle destroys. The Occupation must roll exactly
  what it rolls today after `destruction_rolls` gains its parameters, and a test says so.
- An anti-missile rule. The escort rule is the counter, by decision.
- Removing a Colony from the board. The Core Module is spared, so a place is gutted, never erased.

## Refutation

The specification is wrong if an unescorted Missile Carrier is not the first hull struck in a Battle
whose party has lost its warships; if a Launch off Earth changes the Natural Sink or the war bucket;
if a Launch on Earth changes neither; if a nuked Region cannot raise its Industry Level again; if
The Sink Weakens on an untouched game leaves a Sink other than 4.0; if an Occupation timing out
burns a different share of buildings than it does on main; or if a Missile Carrier holds Orbital
Control.

## Red witnesses owed

One test per rule R1 to R7 in `engine/tests/formulas.rs`, each watched red against the current rule
or the rule reverted on purpose, with the failing assertion quoted. The Refutation's seven scenarios
are the ones to write first.
