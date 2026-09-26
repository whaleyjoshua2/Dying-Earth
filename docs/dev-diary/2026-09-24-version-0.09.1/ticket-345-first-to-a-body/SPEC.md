# Ticket #345: first to a Body. The build specification

Authority: the resolution comment on
[ticket #345](https://github.com/whaleyjoshua2/Dying-Earth/issues/345), decided by the designer on
2026-09-24. Where this file and that comment disagree, the comment wins and this file is wrong.

## Prior art, searched before writing

Every claim below was read out of the tree on 2026-09-24, at the line given.

- **Nothing records who settled a Body first.** `Colony` (`state.rs:414-438`) carries `body`, `slot`,
  `control`, `founded_turn` and `in_orbit`, and **no founder**. `Game` (`state.rs:1367`) has no
  per-Body ledger of any kind. A Colony taken by Influence or by force keeps no trace of who raised
  it.
- **A Colony is never removed from the board.** There is no `colonies.retain` and no
  `colonies.remove` anywhere in the engine. "Losing" a Colony can only ever mean it changed hands.
- **The Allotment is ASSIGNED, not added to.** `income_phase` does `s.allotment = allot`
  (`economy.rs:172`), so whatever is unspent is wiped every Income. **This is the whole reason R3
  below is built the way it is.**
- **The turn order is Orders (4), Event (5), Resolution (6), End (7), then turn+1 and Income (1)**
  (`turn.rs:158-228`). A Colony is founded in the **Resolution**. So a windfall written into
  `seat.allotment` at the founding is **wiped by the very next Income before the player can spend a
  single point of it**, and would pay exactly nothing.
- **There is already one term that survives that wipe, and it is the shape this needs.** The
  Spaceport's clause (`state.rs:3265` `pay_spaceport`) accumulates into `seat.spaceport_influence`
  during the turn; `influence_allotment` adds it **after the Faction multiplier, at face value**
  (`state.rs:3252`); and the next Income adds it into the new Allotment and zeroes it
  (`economy.rs:175`). Its comment records exactly the argument the designer used for this ticket:
  the Arkwrights' ×0.8 says they are bad at diplomacy, and a clause about moving people should not
  be reduced by it.
- **`building_allotment` (`state.rs:3219`) sits INSIDE the multiplier**, summing `influence_allotment`
  from working Facilities at controlled states and working Modules at owned Colonies, skipping a
  starved Colony's. The **Core Module's row in `modules.toml:208` carries no `influence_allotment`
  at all**, so the +1 cannot be a data row there: that would give every Core in the game +1.
- **Venus has no ground Colony Slots.** Read out of `bodies.toml`: Earth 3 (all Antarctic), the Moon
  4, Venus **0**, Mars 6, Phobos 2, Deimos 1. Venus has 3 Orbital Slots and nothing on the ground.
- **A ground Colony is founded at two sites**: `found_antarctic_colony` (`resolution.rs:2100`, by
  sea, Earth only) and the Colony Ship's unload into a slot (`resolution.rs:2322-2345`). A station
  is founded at a third (`resolution.rs:2203`), and is **not** a settling under this ticket.
- **The tie precedent is not a pure random draw.** Competing foundings into the *same* slot are
  decided by `tiebreak_at_body` (`resolution.rs:1613`), which takes the seat with the **greatest
  ship stack strength** at the Body and only calls `random_tie` (`state.rs:1735`) among those tied
  on strength. The comment at `resolution.rs:2209` says "ties drawn at random", which is what was
  put to the designer, and it is a simplification of what the function does. **See R5.**
- **Measured, eighty games, the sweep of 2026-09-24**: Antarctic Colonies founded 60, a Mars Colony
  in **0 of 20 seeds in every seating**, Phobos and Deimos never, Venus 2 stations with nobody
  living on them. Median turn of the first Colony anywhere: 10.
- Nothing found that does what is specified; the finding is a negative.

## The rules

### R1. The record of who was first

- A new record on `Game`: for each Body, **which seat was first and at which Colony**. A `Vec` of
  `{ body, seat, colony }` rows, one per Body at most, appended when a first is claimed. In the
  save.
- It is written **only** by R2 and is never rewritten: a Body's first is claimed once and for good.
- `Game::first_at(body) -> Option<(Seat, ColonyId)>` and `Game::firsts_of(seat)` for the interface,
  the AI and the sweep.

### R2. What claims it

- **Founding a ground Colony on a Body other than Earth, at a Body where no ground Colony has ever
  stood.** Both ground-founding sites must claim it, but only one of them can: the Antarctic landing
  is on Earth and Earth is excluded, so in practice the Colony Ship's unload
  (`resolution.rs:2322-2345`) is the site. **A test must prove an Antarctic founding claims
  nothing.**
- **A Space Station claims nothing** (`resolution.rs:2203` is untouched), and a station standing at a
  Body does **not** close that Body's first.
- **Venus can never be claimed**, having no ground slots. No code enforces this; it falls out of the
  board. A test says so, so that adding a Venus slot later turns the rule on by itself.
- The Body's other slots **stay open to everybody** exactly as they are today. Only the bonus is
  spent.

### R3. The windfall

- `bodies.toml` gains **`first_windfall`** per Body: the Moon **5**, Mars **15**, Phobos **20**,
  Deimos **20**, Earth and Venus **0**.
- At the founding it is added to a **new accumulator on the seat**, `first_windfall`, built exactly
  as `spaceport_influence` is:
  - `influence_allotment` adds it **after the Faction multiplier, at face value**, beside the
    Spaceport's;
  - the next `income_phase` therefore pays it into the new Allotment, and then **zeroes it**
    (`economy.rs:175`, the same line).
- **This means the windfall is spendable in the turn AFTER the landing, which is the first turn the
  player can spend anything at all.** Writing it into `seat.allotment` during the Resolution, which
  is what the resolution comment's words say, would see it wiped by the next Income before a single
  point could be spent, because the Income assigns. **Named in the Built comment for the designer,
  because it is a departure from the letter of the decision made to honour its intent.**
- Paid **once**. Losing and retaking the Colony never pays it again.

### R4. The Core's +1

- `influence.toml` gains **`first_settled_allotment` = 1**.
- `influence_allotment` adds, **after the multiplier, at face value**, beside the Spaceport's and the
  windfall's: `first_settled_allotment` for every row of R1's record whose `seat` is this seat **and
  whose `colony` this seat currently directs**.
- So it is **dormant** while a rival holds the Colony, **never pays the rival**, and **wakes** if the
  founder takes the place back.
- It **never hops**: it is keyed to the Colony id in the record, so a second Colony of the founder's
  on the same Body pays nothing.
- It **keeps paying while the Colony is starved of Energy**, unlike a Relay or a Chorus. It must
  therefore **not** go through `building_allotment`, which both sits inside the multiplier and skips
  a starved Colony.

### R5. A tie

- Two or more seats founding a ground Colony at the **same Body** in one Resolution, in **different
  slots**, all land; the first is decided by **`tiebreak_at_body`** (`resolution.rs:1613`), **the
  very function the contested-slot path already uses**. So the seat with the **greatest ship stack
  strength** at the Body takes it, and `random_tie` (`state.rs:1735`) breaks a tie only among seats
  level on strength.
- **This was corrected by the designer mid-build.** The tie was first put to them as "drawn at
  random, as a contested slot already is", which conflated two things: the contested-slot path
  weighs fleet strength first, and the comment at `resolution.rs:2209` that says otherwise is a
  simplification. Told that, the designer: *"let's keep the current system for ties."* So the
  existing function is the rule, and this is no longer a choice the build made.
- Same-slot contests are untouched: they are settled before this, and the loser never founds.

### R6. The computer

- The founding appetite (`ai.rs:2224`, `Cat::FoundColony`) reads **what an unclaimed Body would
  pay**, so a distant unclaimed Body becomes worth the voyage. Weights in `ai.toml`.
- This is the change that makes the rule exist in play: no computer seat has founded a Mars Colony in
  any of eighty measured games.
- The sweep reports **which firsts were claimed, by whom, and on what turn**.

### R7. What the game says

- A **Report line** and a **Moment** when a Body's first is claimed, naming the Faction, the Body and
  the Colony, in `report.toml`.
- The Colony's card and the Core Module's tile say the Colony was first to its Body and what it pays;
  the income-sources hover lists the +1 and the windfall as its own line, as the Spaceport's is.

### R8. Save

- `SAVE_VERSION` moves: the record of R1 and the seat's `first_windfall` accumulator are both new
  and an older file has neither.

## Error cases

- A first claimed twice for one Body: impossible by construction, and a test asserts the record
  never grows a second row for a Body.
- A `first_windfall` missing from a Body's row: the load check refuses the table.
- A record row naming a Colony that is a station: impossible by construction; a test asserts it.

## Out of scope

- Giving Venus ground slots. Venus is unclaimable and that is the honest consequence of the board.
- Any change to what a station founding does, to `building_allotment`, or to the same-slot contest.
- Any change to the Faction multiplier itself.

## Refutation

The specification is wrong if an Antarctic founding claims a first; if a station founding claims one
or closes a Body; if the windfall is wiped before the player can spend it; if a rival who takes the
Colony is paid the +1; if the founder is still paid the +1 while a rival holds it; if a second Colony
of the founder's on the same Body pays a second +1; if a starved Colony stops paying it; if either
part is reduced by the Arkwrights' ×0.8; if a Body's first can be claimed twice; or if two seats
founding at one Body in one Resolution do not give the first to the one with the stronger fleet
there.

## Red witnesses owed

One test per rule R1 to R7 in `engine/tests/formulas.rs`, each watched red against the rule reverted
on purpose, with the failing assertion quoted. The Refutation's nine scenarios are the ones to write
first.
