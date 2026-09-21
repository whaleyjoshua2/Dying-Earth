# The sea reaches inland

Ticket [#276](https://github.com/whaleyjoshua2/Dying-Earth/issues/276) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

| picture | what it shows |
|---|---|
| [`china-after-a-rise.png`](china-after-a-rise.png) | `shot: temp:1.85 select:eastasia panel:0`, 1280x800. China's card after the +1.8 threshold and no wall: **5 of 7 slots free**. The boxes read in the new order: three coastal (the Refinery, the Launch Site and one free), two *lost to the sea* (the Factory and the Power Plant, oldest first, as before), and four free inland. Before this ticket the coastal row would have been two boxes and the inland row five; the rise took two and turned one. |
| [`china-walled-two-rises.png`](china-walled-two-rises.png) | `shot: walls:2 select:eastasia panel:0`. The same state with a Sea Wall standing through the +2.3 threshold: the wall's row reads **"has held back 2 rises: 1.0 Materials a turn to keep"**, no slot was lost, and the boxes are four coastal (the Launch Site and three free), two lost, three inland (the Research Lab and two free). The wall held the taking; the coast still moved one slot further in. `walls:2` is a new form of the `walls:1` aid that fires the next threshold with the wall in place, because `temp:` and `walls:1` compose in the wrong order for this picture. |

The measurement, twenty seeds with the Custodians first: [`sim-20-custodians-first.txt`](sim-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 a"* -- **no landlocked flag.** Every Region on the board has a coast, so *"every nation not
  land locked"* is every nation; a Region at Coastal Exposure 0 would be spared, and none exists.
- *"q2 yes"* -- **the wall does not stop it.** With a working wall the rise takes nothing and still
  turns one inland slot coastal; the wall holds the taking off, not the turning.
- *"q3 after"* -- the slot turns **after** the rise has taken its coastal slots, so it faces the next
  rise and not the one that made it.
- *"q4 c empty first"* -- an **empty inland slot turns first**; when none is empty, the **oldest**
  inland Facility turns with its slot and can drown at the next rise. The recommendation was the
  newest; the designer chose the oldest, the same order the coast drowns in.
- *"q5 no ceiling"* -- a Region may turn all coast, one slot a rise, until nothing inland is left.
- *"q6 yes"* -- a slot a raise of the Industry Level adds turns like any other.
- *"q7 go with that"* -- the strip's coastal and inland boxes and the Report line are the whole of
  what the player sees; no date on a box.

## Settled by the builder, to be corrected if wrong

- **The count is a counter, not a list.** A Region has no slot objects; it gains a saved
  `converted` figure with a default of nought, so every old save loads with no slot turned. Coastal
  is `2 x Exposure + converted - lost`, inland is the rest less `converted`.
- **A slot a queued build has reserved is not empty.** If nothing stands inland but a build is
  queued there, the build turns with its slot and completes on the coast.
- **A Region with no inland slot left turns nothing**, and the Report says nothing about it.
- **The Report line.** After the rise's own sentence: *"The coast now reaches one slot further in."*,
  or *"… further in: a Research Lab stands on it now."* when a Facility turned with its slot. The
  held-rise line carries it too, after *"dearer to keep"*.
- **The three sea figures in the sim and the sweep are counters on the state now** (slots lost,
  Facilities drowned, slots turned), not sentences scraped from the log; the scrapers broke whenever
  a line was reworded, and this ticket reworded one.
- **The glossary's Coastal Slot entry said three per point of exposure**; it has said two since
  0.05.5 everywhere else, and now says so here.

## Witnessed red

The new test was written against the rule, then the conversion was switched off on purpose in the
engine and the test watched to fail at *"Exposure 2 took two, and one inland slot turned coastal
after the taking": left 2, right 3*. Restored, it passed, with the four older sea tests moved from
the rule they encoded (*"and then nothing"*, *"the coast is gone"*) to the new one. `333 passed`,
`6 passed`, clippy clean with `-D warnings` across the workspace.

## Measured, twenty seeds with the Custodians first

| figure | 0.08.4 (this seating) | this ticket |
|---|---|---|
| Prospector wins / collapses of 20 | 15 / 5 | **15 / 5** |
| coastal slots lost a game, median | 22 | **30** |
| Facilities drowned a game, median | 12 | **10** |
| inland slots turned coastal a game, median | 0 | **50** |
| Sea Walls built over the batch | 478 | **537** |

The win column and the collapses did not move. The sea takes more slots because the coast no
longer runs out after two rises: fifty slots a game turn coastal across fourteen Regions and four
or five rises, and the walls the computer builds are worth more, so it builds more. The closing
sweep measures all four seatings.
