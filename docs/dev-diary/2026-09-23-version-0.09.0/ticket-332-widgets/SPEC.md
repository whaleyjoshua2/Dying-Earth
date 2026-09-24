# Ticket #332: Widgets. The build specification

Authority: the resolution comment on
[ticket #332](https://github.com/whaleyjoshua2/Dying-Earth/issues/332#issuecomment-5804831814),
decided by the designer on 2026-09-23. This file says how that decision is built; where the two
disagree, the comment wins and this file is wrong.

## Prior art, searched before writing

- **The source document** (the resolution): every rule below traces to a sentence of it.
- **This tree**: there is no Production, no progress on a build, no Widgets. `Build` is
  `{ item, seat, due_turn, coastal }` (`engine/src/state.rs:194-201`); `commit_orders` sets
  `due_turn = turn + build_turns - 1` (`orders.rs:1656-1690`); `resolve_builds` pops whatever is
  due (`resolution.rs:1312-1364`). Only two buildings make Materials, the Factory
  (`facilities.toml:4-11`) and the Mine Module (`modules.toml:3-11`). `Resource` is Materials,
  Fuel, Energy, Ducats, Research (`ids.rs`). Production Moved pairs Facility kinds to Module
  kinds in `factions.toml:35` and doubles the paired Module's final figure
  (`economy.rs:491-503`). Nineteen readers of `build_turns` (`grep -n build_turns engine/src src`).
  Nothing found that does what is specified; the finding is a negative.
- **Libraries**: none apply; this is game rules in the engine crate.

## The rules, as inputs, outputs and invariants

### R1. Widgets are a rate per place

- **Input**: a place (a Region, or a Colony or station) at Resolution.
- **Output**: `widgets_at(place) -> i64`, the Widgets that place makes this turn:
  - a Region: `industry_level x widgets.per_industry_level` (data, 1) plus every working
    Factory's Widget yield through `facility_yield` (so the Prospectors' `output_multiplier`,
    the Strip Permit's double and the Unrest-7 half apply; the Region's Materials lean and Deep
    Mining do not, since they are Materials rules);
  - a Colony or station: every working Module's Widget yield through `module_yield_at` (so
    Production Moved's doubling applies), the Core Module's row carrying
    `produces = { resource = "widgets", amount = 1 }`.
- **Invariant**: Widgets never enter the Stockpile, are never traded, never saved as a stock.
  Whatever is not applied this turn is lost (counted).

### R2. A build carries a figure and a count

- `Build { item, seat, widgets: u32, done: u32, coastal }`; `due_turn` is retired.
- `widgets` at the order is the row's figure times the seat's Faction discount for that kind
  (`facility_materials_multiplier` for a Facility and an Industry raise, `module_materials_multiplier`
  for a Module, `ship_materials_multiplier` for a Ship, none for an Army), floored, never below 1.
- The row's figure is a `widgets` field that **replaces `build_turns`** in every row of
  `facilities.toml`, `modules.toml` (the Archive included), `units.toml` and `[industry_level]`,
  at **four per former build turn**.
- An outright buy in Ducats (`BuildFacilityWithDucats`, `BuildModuleWithDucats`) sets `done =
  widgets` at the order, so it completes at the next Resolution ahead of the queue.

### R3. The queue is served in order

- **Input**: a place's `queue` (order of insertion is the order given) and its `widgets_at`.
- **Process**, at Resolution step (e): `remaining = widgets_at(place)`; for each build in queue
  order, `take = min(remaining, widgets - done)`, `done += take`, `remaining -= take`; every build
  with `done >= widgets` completes (`complete_build`, unchanged) in queue order.
- A Launch Pad Fire at a Region (no Clean Propellant): that Region applies **no Widgets this
  turn**; the requeue-a-Ship rule goes, since nothing completes.
- **Invariant**: a build never completes with `done < widgets`; the sum applied at a place never
  exceeds `widgets_at`.

### R4. Four makers

- `FacilityKind::Mine` appended last; `ModuleKind::Factory` appended last.
- Factory (Facility): `produces = { resource = "widgets", amount = 4 }`; the rest of its row
  unchanged (20 Materials, 2 Energy, emissions 1.0, Clean Manufacturing on its emissions).
- Mine (Facility): 20 Materials, `widgets = 4`, 2 Energy, `produces = { resource = "materials",
  amount = 4 }`, emissions 1.0; the Materials lean applies by the existing `resource_lean == res`
  test; **Deep Mining moves from the Factory to the Mine** in `tech_output_multiplier_facility`.
- Factory (Module): 20 Materials, `widgets = 4`, 3 Energy, `produces = { resource = "widgets",
  amount = 4 }`, `earth_emissions = 1.0`, no Body or slot yield, no Tech.
- `Resource::Widgets` exists so `produces` parses and `Yield` carries it; the Income phase skips
  it (nothing adds it to a Stockpile; the income-sources hover does not list it). If adding the
  variant forces more than a handful of exhaustive matches, a `widgets: i64` on `Yield` beside
  `research` is the alternative; the data spelling stays `produces = { resource = "widgets" }`.
- The starting board: every `start_facilities` list holding `"factory"` gains `"mine"` beside it
  (`nation_states.toml`, eight Regions).

### R5. Production Moved

- `mothball_pairs = { factory = "factory", mine = "mine", power_plant = "generator", refinery =
  "refinery", research_lab = "observatory" }` for the Custodians.
- The doubling applies to the Factory Module's Widget yield as it does to a Mine's Materials.
- The signature string in `factions.toml` and the rulebook text move with it.

### R6. The outright buy, the discounts, the station

- Twice the Materials in Ducats buys a Facility or Module outright, as today; it completes at the
  next Resolution (R2).
- A Space Station founding costs Materials only and carries no Widgets; lifts stay free.

### R7. A place that changes hands; a cancel that is a prize

- `transfer_control` leaves the queue as it is: each build keeps its seat and finishes as usual.
- New `Order::CancelBuild { place, index }`: legal when the ordering seat directs the place and
  the build at `index` belongs to another seat. It removes the build and **refunds the item's
  Materials at the canceller's own price** to the canceller's Stockpile; the Widgets done are
  lost. A Report line names it (`build_cancelled` in `report.toml`). The computer seats never
  cancel this ticket.

### R8. Save

- `SAVE_VERSION = 3`, the doc comment extended in its shape: a Build carries a figure and a
  count in place of a due turn, two kinds appended, the tables' rows carry Widgets.

### R9. Counters

- `sim.rs` Result and `sweep.rs`: Widgets made, applied and lost a game by seat (a place's
  Widgets counted to its director), and the median queue depth at Resolution.

### R10. An estimate for the interface

- `Game::turns_to_build(seat, place, item) -> u32`: with this place's `widgets_at` and the
  Widgets still owed by everything ahead in its queue, the Resolutions until a fresh order of
  `item` would complete; at least 1; `u32::MAX` if the place makes nothing.

## Error cases

- `CancelBuild` on a build of the ordering seat's own: refused (it is not the rule; an own order
  this turn is cancelled by the existing right-click on the pending order).
- `CancelBuild` at an index past the queue: refused.
- A `widgets` row of 0: the load check refuses the table.
- A Faction discount that floors a figure to 0: the figure is 1.

## Out of scope

- The presentation (top bar, cards, tiles, buttons, the glyph): the interface lane.
- The computer seats' wants and placement: the AI lane; the engine only exposes `widgets_at`,
  `turns_to_build` and the queue.
- Any Module beyond the Factory making Widgets off Earth; the Trading window; repairs.

## Refutation

The specification is wrong if, on a fresh board with one Region holding one Factory and Industry
Level 1, a single 1-turn Facility ordered on turn N does not complete at turn N's Resolution
(5 Widgets against 4), or if two such Facilities ordered together complete in the same turn
(the second needs turn N+1). It is also wrong if a Mine Facility in China makes 4 rather than 6
Materials, or a Factory anywhere makes any Materials at all.

## Red witnesses owed

One test per rule R1 to R7 and R10 in `engine/tests/formulas.rs`, each watched red against the
0.08.8 rule (or the rule reverted on purpose) before green, with the failing assertion quoted in
the lane's report.
