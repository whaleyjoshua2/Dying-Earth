# Ticket #353: seven defects the playtest found. The build specification

Authority: the resolution comment on
[ticket #353](https://github.com/whaleyjoshua2/Dying-Earth/issues/353), decided by the designer on
2026-09-24. Where this file and that comment disagree, the comment wins and this file is wrong.

**Nothing here changes a rule.** No figure moves, `SAVE_VERSION` does not move, and **the closing
sweep must read identically on every line**. A sweep that moves is a defect in the fix.

## Prior art, searched before writing

Every location below was re-read out of the tree on 2026-09-24. They had drifted since the ticket
was charted, three tickets having landed on the same files.

- **D1**: `return fail("no Shipyard here")` at `orders.rs:1115`, guarded by `m.kind ==
  ModuleKind::Shipyard && m.working()`. The **Barracks** test at `orders.rs:1133` is
  `m.kind == ModuleKind::Barracks` with **no `working()` call at all**, so the two doors disagree
  about what a Module must be before it counts. `orders.rs:1417` already gets this right for the
  Rearm: it has `RearmSite::NoPlace` and `RearmSite::NoShipyard` as separate refusals so the message
  names what is missing. **That is the shape to copy.**
- **D2**: `(slot + 1).to_string()` at `resolution.rs:2207` (the Antarctic founding) and `:2441` (the
  Colony Ship's), and `slot + 1` inside a `format!` at `:2435`. The slot's name is already known:
  `self.tables.body(b).slots[slot as usize].name`, and `Game::place_name(Place::Colony(id))` gives
  the founded Colony's name, which **is** the slot's name.
- **D3**: `let moved = n.min(room);` at `resolution.rs:2192` (Antarctic founding), `:2237`
  (Antarctic join) and `:2428` (the Colony Ship's founding). Each reports `moved` and **nothing
  anywhere says what stayed aboard**. A Colony Ship carries 4 and a Core Module holds 4, so this
  fires on the first founding of most games.
- **D4**: `orders.rs:1814` computes `room` and then `if *n > room { return fail(...) }`, refusing the
  whole order. The driver's own help says *"as far as its Habitat room goes"*. The graphical client
  pre-clamps the button, so **the lie is only reachable from the driver**, which is where a
  playtester met it.
- **D5**: `fund_archive = "set its Labs to pay the Archive fund from the next Income"` at
  `report.toml:240`, used for all four Factions. The correct per-Faction wording **already exists**
  at `report.toml:67-69` as `directive_sink`, `directive_ducats` and `directive_fuel`.
- **D6**: `LineKind::Event => 7` in `headline_rank` (`report.rs:123`). `headline_index`
  (`report.rs:307`) picks the lowest rank and `headline()` returns it. A card answer is filed as
  `LineKind::Event` at `events.rs:413` and `:422`. The guard at `src/ui.rs:9810` is
  `game.report.headline().filter(|h| !is_card_answer(game, &h.text))`, which turns `Some` into
  `None` and so **drops the headline entirely instead of falling through**.
- **D7**: `engine/examples/play.rs:198`, the whole entry: `build archive <colony>  the Archivists
  only`. The engine's refusal is already good: *"the Archive waits on The Upload, which the world
  has not researched yet"*.

## The rules

### R1. A refusal names what is missing

- The Shipyard test splits, in the shape `rearm_site` already uses at `orders.rs:1417`: **no
  Shipyard at all**, **a Shipyard still building**, and **a Shipyard shut for Energy** are three
  refusals, each naming its own case.
- **Audit every other `working()` gate in `orders.rs` for the same lie** and fix each one found, or
  report that none was. A gate that tests `working()` and names absence is the defect; this ticket
  fixes the class, not one instance.
- The **Barracks** test is the opposite error: it never calls `working()`, so a shut Barracks builds
  an Army. **Do not change the Barracks rule** — whether a shut Barracks should work is a design
  question and not this ticket's. Report it for a ticket of its own.

### R2. A slot is named, not numbered

- Every place a slot is shown to a player uses **the slot's name**, not a number: the founding
  headline, the Report copy and any Moment.
- **The headless driver keeps its 0-based numbers**, where a slot is an argument typed by hand.
- No `slot + 1` survives anywhere a player reads.

### R3. A partial unload says so

- All three clamp sites report what **stayed aboard** when `moved < n`, in `report.toml`, naming the
  number left and why (Habitat room).
- It is one phrase used by all three, not three wordings.

### R4. `lift` fills as far as it can

- `orders.rs:1814` **clamps to the room** instead of refusing, matching the driver's own help.
- It still refuses when the room is **nought**, since an order that moves nobody is not an order.
- The graphical client pre-clamps and is unaffected; a test proves the engine now clamps.

### R5. The Research Directive line names what it bought

- The Archive-fund wording is used **only for the Archivists**. The other three read the existing
  `directive_sink`, `directive_ducats` and `directive_fuel` for what their own Directive buys.
- No new wording is invented where wording already exists.

### R6. The headline

Three changes, all three needed:

- **A card answer never headlines.** It is filed under a kind with **no headline rank**, which
  deletes the need for the `is_card_answer` guard in `src/ui.rs` entirely. Remove it.
- **The drawn card does not headline a turn its own modal has already shown it.** The player has
  read it once; the Report should not open by repeating it.
- **The headline falls through.** With the two above, `headline()` returns the next line by rank on
  its own and `src/ui.rs:9810` becomes a plain `if let Some(head)`. **A turn must never open with no
  headline because its loudest line was filtered.**

### R7. The driver's grammar says what the refusal says

- `play.rs:198` gains the three rules the engine already enforces: **The Upload must stand**, the
  Colony must be **off Earth**, and **four Colonists** must live there.
- Written in the grammar's existing register, matching the entries around it.

## Error cases

- A `report.toml` phrase referenced but absent: the load check refuses the table.
- `lift` with room for nobody: refused, naming the room.

## Out of scope

- Whether a shut Barracks should build an Army (R1). Report it, do not fix it.
- Any rule change, any figure, any balance. `SAVE_VERSION` does not move.

## Refutation

The specification is wrong if a build refusal still says "no Shipyard here" when a Shipyard stands
shut; if any player-facing line names a slot by number; if a partial unload leaves no line saying
what stayed aboard; if `lift` refuses an order it could partly fill; if a non-Archivist's Directive
still reads as paying the Archive fund; if a turn whose loudest line is a card answer opens with no
headline; if the drawn card headlines a turn its modal has shown; or if the closing sweep moves on
any line.

## Red witnesses owed

**One test per defect**, each watched red against the defect restored on purpose, with the failing
assertion quoted. These are messages, and a message is exactly the kind of thing a suite does not
notice, so a test that was never seen red certifies nothing here in particular.
