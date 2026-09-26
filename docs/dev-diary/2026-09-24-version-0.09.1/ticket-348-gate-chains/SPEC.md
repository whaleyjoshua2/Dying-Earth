# Ticket #348: every Faction's gate chain in its own pick list. The build specification

Authority: the resolution comment on
[ticket #348](https://github.com/whaleyjoshua2/Dying-Earth/issues/348), decided by the designer on
2026-09-24. Where this file and that comment disagree, the comment wins and this file is wrong.

## Prior art, searched before writing

Computed from `assets/data/techs.toml` and `assets/data/ai.toml` on 2026-09-24.

- **The four gate chains**, each traced through `needs` to its roots:

| Faction | gate | chain | Research | antecedents, in dependency order | missing from its own `order` |
|---|---|---|---|---|---|
| Custodians | `planetary_stewardship` | 3 | 98 | `public_science`, `green_consensus` | **none** |
| Prospectors | `extraction_charter` | 5 | 148 | `deep_mining`, `efficient_grids`, `automated_refining`, `beneficiation` | `beneficiation` |
| Arkwrights | `generation_ships` | 5 | 148 | `expanded_habitats`, `efficient_grids`, `clean_power`, `closed_loop_colonies` | `efficient_grids`, `clean_power` |
| Archivists | `the_upload` | 5 | 148 | `expanded_habitats`, `efficient_grids`, `clean_power`, `closed_loop_colonies` | `expanded_habitats`, `clean_power`, `closed_loop_colonies` |

- **The pick lists today** (`ai.toml:436-457`): the Prospectors' `order` is
  `["deep_mining", "efficient_grids", "automated_refining", "clean_propellant", "efficient_transit"]`
  with `last = "clean_power"` and `never = "green_consensus"`; the Custodians' is seven long and
  complete; the Arkwrights' is
  `["expanded_habitats", "closed_loop_colonies", "efficient_transit"]`; the Archivists' is
  `["public_science", "efficient_grids", "coastal_engineering", "green_consensus", "civil_defense"]`.
- **`draw_shortlist` (`research.rs:140-161`)**: returns an empty shortlist when
  `available.len() <= size`, which the panel reads as a free choice of everything. Otherwise it
  pushes the Lead's gate **only** `if available.contains(&gate)`, then fills to `size` = 3 by
  uniform draw from the rest, then sorts by tree index.
- **`available_techs()`** is the set whose prerequisites are complete. **This is the crux**: the
  gate is in it only once every antecedent is researched, so the existing guarantee cannot fire
  until the chain has already been climbed.
- **`draw_shortlist` has exactly one caller**, `research.rs:116`, on a Tech completing.
- **Rung costs** are 18, 32, 48, and **every Faction's gate is on rung 3**.
- **Measured, eighty games**: wins Custodians 3, Prospectors 30, Arkwrights 1, Archivists 0;
  collapses 45 of 80; the Victory gates complete in `[15, 19, 18, 19]` seeds by seat at median turns
  30, 26, 30, 25; the whole tree completes in 9 to 13 seeds of 20 by median turn 34 to 36.
- Nothing found that does what is specified; the finding is a negative.

## The rules

### R1. Each pick list opens with its own chain

- Each `order` in `[tech_picks.*]` begins with **its gate's missing antecedents, cheapest first**,
  then everything that list held before, in its existing order.
- **Only the three incomplete lists change.** The Custodians' `order` is not touched.
- An antecedent already present in a list is **not duplicated**; it keeps its existing place if that
  place is ahead of the additions, and is moved to the front only if this rule puts it there.
- **A test computes each chain from the tables and asserts every antecedent appears in that
  Faction's `order`**, so the check survives a change to the tree rather than pinning today's list.

### R2. The shortlist carries the next rung of the Lead's chain

- In `draw_shortlist`, alongside the existing gate clause, force-include the **cheapest unresearched
  antecedent of the Lead's gate** that is itself available.
- **At most one of the two is ever forced.** The gate is available only when every antecedent is
  done; an unresearched antecedent exists only when some antecedent is not done. The conditions are
  mutually exclusive, so the reserved place stays **one** slot of three, exactly as today. **A test
  asserts this**: no draw ever contains both the gate and a forced antecedent.
- Ties on cost break by **tree index**, so a seeded game is unchanged by the choice.
- The forced antecedent must be **available** (its own prerequisites done); if the cheapest
  unresearched antecedent is not yet available, take the cheapest one that is. If none is, force
  nothing.
- Never duplicated into the random fill, as the gate already is not.
- The empty-shortlist case (`available.len() <= size`) is untouched: a free choice needs no help.
- **This reaches the human player, not only the computer**, because the Lead's draw is the Lead's
  draw whoever holds it. That is the point of the rule.

### R3. Clean Power stops being the Prospectors' deferred Tech

- `last = "clean_power"` is removed from `[tech_picks.prospectors]`.
- The reason belongs in the file: on a shared tree, deferring a Tech that is an antecedent of
  **both** the Arkwrights' and the Archivists' gates stalls two rivals' Victory Conditions without
  the Prospectors ever choosing to.
- `never = "green_consensus"` is untouched: that is a Faction saying what it will not have, and
  Green Consensus is nobody else's antecedent.

### R4. What the sweep must say

- **Gate Techs completed per Faction and the turn each landed** — the figure this ticket is judged
  by. The sweep already reports `Victory gates: completed in [..] seeds by seat, median turn [..]`;
  confirm it reads per **Faction** at the foot and not per seat, and add the foot line if it does
  not.
- **A wins table that moves is expected here**, unlike ticket #353. This is a change to how the
  computer plays and the whole point is that three Factions now climb chains they could not reach.
  Report the before and after and say plainly which way it went.
- **A rising collapse rate is a figure on this project, not an alarm.** Report it as a number.

## Error cases

- A Faction whose gate has no antecedents: R1 adds nothing, R2 forces only the gate. The Custodians
  are not this case (they have two) but the rule must not assume a non-empty chain.
- A gate already researched: nothing is forced.

## Out of scope

- **The cheapest-Tech reroll**, which is
  [#360](https://github.com/whaleyjoshua2/Dying-Earth/issues/360). It pulls against everything here
  and it is a separate rule.
- `[shortlist] size`. It stays 3, and R2's mutual exclusion is why it can.
- Any change to a Tech's cost, its `needs`, or the tree's shape.
- `SAVE_VERSION` does not move.

## Refutation

The specification is wrong if any Faction's `order` omits an antecedent of its own gate; if a draw
ever forces both the gate and an antecedent; if the reserved place grows beyond one slot; if the
Custodians' list changes; if Clean Power is still deferred by the Prospectors; if a forced antecedent
is offered whose own prerequisites are unmet; or if the sweep cannot say when each Faction's gate
landed.

## Red witnesses owed

One test per rule R1 to R3, each watched red against the rule reverted on purpose, with the failing
assertion quoted. R2 owes **two**: one that the next rung is offered while the gate is not yet
reachable, and one that the two are never forced together.
