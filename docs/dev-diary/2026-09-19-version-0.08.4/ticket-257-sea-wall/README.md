# A Sea Wall that stands

Ticket [#257](https://github.com/whaleyjoshua2/Dying-Earth/issues/257) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`china-card-walled.png`](china-card-walled.png) | `shot: walls:1 select:eastasia panel:0`, 1280x800. China's card with the sea through two of its coastal slots (a Factory and a Power Plant drowned, boxed in blue and marked *lost to the sea*), a Launch Site and a Research Lab still standing, and beneath the boxes the wall's own row: **"Sea Wall: no output, 1 Energy upkeep; has held back 1 rise: 0.5 Materials a turn to keep."** The `walls:1` aid was taught to give its wall one rise held, so the row has something to say. |

## What was decided, in the designer's words

- *"1"* -- of the three readings offered, the literal one: **the wall stands and holds every
  threshold**, not a breach needing a repair, not a half-measure. The recommendation (a breach
  repaired by a Restart) was overruled.
- *"unchanged"* -- the displacement and the Unrest a threshold brings still pass through the wall,
  as they have since ticket #52.
- *"yes add an upkeep cost of .5 material every sea rise"* -- each rise the wall has held adds
  **half a Material a turn** to its keep, on top of the Energy. Paid at Income out of the Materials
  the seat makes, the half carried between turns so nothing is lost to rounding: one rise pays one
  Material every other turn, two pay one a turn, three pay one then two.
- *"that shouldn't be an issue based on my answer to q1"* -- no AI change. `sea_is_close` still
  raises a wall when a threshold is within 0.2 C, and now stops there, since the wall stays.
- *"change it reduce by 30% the output of coastal slots for the next turn"* -- a **Storm Surge**
  that breaks on a standing wall no longer brings the next threshold forward. The wall holds, and
  the Facilities in the state's coastal slots make **30% less at the next Income**, once. An
  unwalled state takes the threshold early, as it always has.
- The Report line, from the designer's *"The Sea Wall in China took the sea, rising sea levels
  have increased the cost to maintain this Sea Wall"*: **"The Sea Wall in China took the sea at
  +1.8 C; the risen water makes it dearer to keep, 0.5 Materials a turn now."**

## Settled by the builder, to be corrected if wrong

Three cases the decisions did not reach:

- **The count is per wall, not per state.** A wall built after two thresholds have already taken
  the coast starts at nothing and pays for the rises *it* holds. The row says how many.
- **A keep the seat cannot pay leaves the wall unkept that turn**, standing but holding nothing --
  the Energy shortfall rule's shape, for Materials. Nothing is paid, the fraction owed is dropped,
  and the Report says *"Materials ran short: the Sea Wall in China stands unkept this turn and
  holds nothing."* The alternative, a debt that accrues, would have let a seat run a wall it never
  paid for.
- **A Storm Surge is weather, not a rise.** It does not add to the wall's keep; only a threshold --
  the three scheduled ones and the Ice Sheets Break -- does.

## Witnessed red

Three tests were written against the new rule and run against the old one first: the wall test
failed at *"and it was destroyed doing it"* (the assertion inverted), the keep test at `0 != -1`,
the surge test at *"no threshold brought forward"*. Then the engine changed and all three passed;
`316 passed`, clippy clean with `-D warnings`.

## What the sweep should show

The computer built **1047 walls in one seating of 20 games** because each died absorbing a
threshold. Expect that to fall to about one per coastal state, the median **32 coastal slots lost a
game** to fall with it, and 20 drowned Facilities a game to fall too. The wall's keep is the price:
a seat with walls on ten coastal states through three thresholds pays 15 Materials a turn.
