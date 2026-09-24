# Armies raised from people: the build, the tests and the pictures

Ticket [#334](https://github.com/whaleyjoshua2/Dying-Earth/issues/334) on version 0.09.0. The
designer's line: *"armies from people too"*. A raised Army takes people at the order, on top of its
Materials and Widgets: in a Region one unit of the Region's population, one million people
(`[army] population_each = 1.0` in `units.toml`), refused where the Region has not got it; at a
Colony one Colonist (`colonists_each = 1`), refused at fewer than two so the Core Module is never
emptied, and the Colonist's Module slot goes with them. The Standing Army takes nobody. Nobody
returns when an Army dies or marches. `sweeps/probe.txt` is the twenty-seed probe.

| picture | what it shows |
|---|---|
| [`army-region-earth.png`](army-region-earth.png) | `shot:army-region select:eastasia panel:0 tip:"and 1M people"`. China's card on turn one with the Build Army button's hover drawn: **25 [Materials], 4 [Widgets], and 1M people, ready next turn here.** The hover sits over the button itself, which is at the card's fold; the people are the third figure of the cost, beside the Materials and the Widgets, in the `people_text` form the Pioneer button under it uses (`1M people`). |
| [`army-colony-moon.png`](army-colony-moon.png) | `shot:army-colony hab:ground barracks:1 tip:"and one Colonist"`. Mare Tranquillitatis on the Moon, a Colony of four Colonists with a Barracks (the new `barracks:1` aid plants it): the **Build Army (Barracks) 25 [Materials] 4 [Widgets]** button and its hover, **25, 4, and one Colonist, ready next turn here.** Above it the card's own line, *Modules 4 of 4 (one for each Colonist)*, which is the slot the Colonist takes with them. |

## What was built

- **The figure in data.** `[army] population_each = 1.0`, `colonists_each = 1` in `units.toml`;
  `ArmyCard` in `data.rs`, validated positive and at least one.
- **The gate and the take.** `Game::army_people_refusal` (`orders.rs`) is the one door: a Region
  wants `population_each` for this raise and every raise already pending there this turn; a Colony
  wants `colonists_each` and one more. `check_order_inner` refuses through it after every other
  check, so the refusal a player reads is *China has not the people for an Army: it takes 1M people*
  or *a Colony keeps at least one Colonist; an Army takes one Colonist*. At commit the Region's
  population falls by the figure, or the Colony's Colonists by one, and the Army goes into the queue
  with its Materials and Widgets as before.
- **The Report.** A new line `army_ordered` in `report.toml` under Your Works, in the Pioneer
  line's shape: *An Army began at China: 1M people under arms.* / *... at Mare Tranquillitatis: one
  Colonist under arms.* The rival deed `build_army` says *began an Army at China, 1M people under
  arms*. No Report line was written at a raise before this ticket, only the rival deed.
- **The Standing Army** is untouched: `spawn_standing_army` and `replenish_standing_armies` take
  nobody, and `destroy_army` returns nobody, pinned by two tests.
- **The computer seats** get no new gate and no new weight: every raise candidate already goes
  through `check_order`, which now refuses for want of people. The chooser counts a raise whose
  refusal is the people one into `WarCounters::army_raises_refused_people`, by seat, and the sweep
  prints it on its own line after the Armies line.
- **The card.** `build_words` (`src/ui.rs`) puts the people after the Widgets on a raise's hover.
- **`CONTEXT.md`**: **Army** and **Barracks** carry the rule since 0.09.0.

## The Colony's slot

A Colony's Module slots are one per Colonist (`module_slots`), and a cap that has fallen below what
stands destroys and mothballs nothing (`free_module_slots`, ticket #97): a Barracks and two Mines
on a Colony of three, raised once, stand at three Modules on a cap of two, with no room for another
until a Colonist arrives. The test pins that.

## The four witnesses, each watched red first

| test | red | green |
|---|---|---|
| a raise in a Region takes one unit and is refused below one | `the raise took 0 units of population, not 1` (before the take was built) | ok |
| a Colony's raise takes one Colonist and is refused at one | `one Colonist went under arms: left: 3 right: 2` (before) | ok |
| the Standing Army's respawn takes nobody | `the Standing Army's return took 1 units of population; it takes nobody` (`spawn_standing_army` perturbed to take a unit) | ok, restored |
| a destroyed raised Army returns nobody | `nobody came home: left: 1439.0 right: 1438.0` (`destroy_army` perturbed to give a unit back) | ok, restored |

## The probe, against ticket #333's

Twenty seeds, Custodians-first seating, `--balance --steps=300`: **wins [0, 2, 0, 0], collapses
18/20, median collapse turn 27, Armies built [2, 12, 7, 0], lost [0, 1, 1, 0], Standing Armies lost
80**, every one the same figure as #333's probe; the only line that differs is the new one, **Army
raises refused for want of people, by seat [0, 0, 0, 0]**. A unit of population out of a Region of
hundreds moves no printed figure, and no computer seat came to a raise without the people. Four
logged games (`sim -- 3 --count=4 --log`) carry six *under arms* lines, so the computer's raises do
take the people.

## Looked at

The two pictures above, opened and read before this was written. The fourteen other view captures
the two runs wrote were deleted.
