# Rolls per engaged warship, not a flat three

Ticket [#327](https://github.com/whaleyjoshua2/Dying-Earth/issues/327) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

No picture: the rule has no surface of its own beyond the Battle line in the Report, which
already lists every unit's hits.

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes q4 yes"*, every one as recommended: one hit roll a round per engaged
armed unit across every party in a Ship melee, never fewer than three, a Battery counting; three
on the ground; three rounds; the odds figure unchanged; the computer seats' bar unchanged, the
batch to say what the deadlier melee does.

## What was built

- **The figures are data**: `units.toml [melee] rounds = 3, rolls = 3`, read into
  `tables.melee`, refused at load if either is nought. The First Playable's constants stay in
  `combat.rs` for the two-sided `fight` helper the tests use.
- **`melee` takes its rounds and rolls from the caller.** The Ship caller counts the engaged armed
  units across every party it hands in (warships and Batteries, not Colony Ships or Carriers) and
  passes the larger of that and the table's rolls; the ground caller passes the table's rolls.
- **Nothing else moved**: the hitter and the target party are still drawn by strength share each
  roll, the escort rule of ticket #326 still says which hull takes the hit, and the hover's odds
  are still the chance of winning the first exchange.
- `CONTEXT.md` **Battle** says the rule and what it replaced.

## Tested

`a_ship_melee_rolls_once_a_round_for_every_engaged_armed_unit`, with scripted dice: two Frigates
and a Battery against a Battleship roll four a round, and the Battleship's eight hit points are
gone in two rounds where a flat three needed three; and a fresh game reads 3 and 3 from the
table. Watched to fail with the rolls capped at three inside the melee (the fight ran a course the
scripted dice did not cover), then to pass restored. The suite is green at 364; clippy clean under `-D warnings`.

## The batch: the deadlier melee, measured

`batch-after.txt`, 20 seeds x four seatings, against the batch after ticket #326:

| | after #326 | after #327 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 22 / 1 / 0 | 43 / 22 / 1 / 0 |
| collapses | 14 of 80 | 14 of 80 |
| Battles opened | 226 | 226 |
| attacks in orbit | 52 | 52 |
| warships built / lost | 217 / 2 | 217 / 2 |
| Batteries lost | 8 | 8 |
| Batteries standing at the end | 36 | 39 |
| units escaped from a Battle | 55 | 60 |

The batch barely moved: the same column, the same Battles, the same two warships lost, with only
the escapes and the Batteries standing at the end a few apart. The reason is in the rule's floor:
the orbital Battles the computer fights on today's board are one or two hulls a side, under the
three the flat rule already rolled, so the extra rolls fire in the few fights with four or more
armed units present. The rule is there for the fleet action the game does not yet stage. Measured behaviour under the computer's habits, whose attack bar is where it was; the
closing sweep judges the whole.
