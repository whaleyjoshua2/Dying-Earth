# The two measured defects and the two-turn respawn: built in 0.08.5; what remains

Ticket [#318](https://github.com/whaleyjoshua2/Dying-Earth/issues/318) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

No picture: nothing on a card or the map changes.

## What was on the list, and where it already stood

The strategy review's proposals 4 and 5, written against 0.08.4. **Both were built in 0.08.5**:
ticket #284 made the Battle line and the Occupation read one predicate (`alone_at`, an attacker
that did not escape and no defender left), so an escaped attacker is neither promised an Occupation
nor given one, and made an occupier stay (Dig In at three times the weight, marching nowhere);
ticket #282 made a destroyed Standing Army return two Incomes after it died, at strength one.

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes"*: both items are done and this ticket adds no rule; the residue is fixed;
the delay becomes a data figure.

## What was built

- **An Occupation is held by the presence that begins it.** `present_at(place, seat)`: an Army of
  the seat at the place that did not escape and does not stand down. The holding check read every
  Army of the occupier there, escaped or not, so an Army that ran kept an Occupation alive until
  the next Resolution cleared its flag. `an_army_that_escaped_holds_no_occupation` in
  `engine/tests/formulas.rs` was watched to fail against the old check and to pass with the new;
  the whole engine suite, 355 tests, is green.
- **`respawn_incomes = 2`** in `assets/data/units.toml [standing_army]`, read where a literal `1`
  (the Incomes to wait) was written; the existing respawn test reads the table through it.
- The **Occupation** and **Standing Army** glossary entries say so.

## The batch

`batch-after.txt`, 20 seeds x four seatings: **line for line the batch after ticket #319**. The
escaped-holder case never arose in eighty games (Occupations broken 0 in every seating), and the
delay is the figure it was, so nothing moved. That is the expected result of a residue fix.
