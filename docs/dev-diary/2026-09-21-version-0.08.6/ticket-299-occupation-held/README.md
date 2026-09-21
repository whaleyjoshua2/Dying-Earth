# An Occupation that must be held

Ticket [#299](https://github.com/whaleyjoshua2/Dying-Earth/issues/299) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

No picture: the rule has no surface of its own beyond a Report line, *The Custodians' Occupation of
The European Union broke: it goes back to the Prospectors at +2 Unrest, an offence against them,
and the 17 Standing the Occupation had won is gone.* The measurement, twenty seeds with the
Custodians first: [`sweep-20-custodians-first.txt`](sweep-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 leave it but if they move control of the state reverts"* -- the march and the Carrier lift
  **stay legal**; the line's "may not move" was dropped. The moment the last Army leaves, the
  Occupation breaks and control reverts, with the cost.
- *"q2 any time it leaves or other break"* -- **any break costs**, by march, lift or death.
- *"q3 yes"* -- **+2 Unrest**, a figure in the Unrest data, undamped, Regions only.
- *"q4 yes"* -- **rung 2**, a figure in the Relations data, and an **Offence** entry in the
  glossary for the ladder.
- *"q5 whipe it"* -- the Standing the Occupation banked is **wiped** on a break.
- *"q6 yes"* -- the Report says so.

## Settled by the builder, to be corrected if wrong

- **"Wipe it" is read as the Standing the Occupation itself brought**, tracked on the Occupation
  as it runs (`banked`), and taken back on a break; Standing the seat held on the place before it
  marched is kept. A save from before this version carries nought banked. If all of the seat's
  Standing there was meant, say so.
- `occupation_break = 2.0` in `unrest.toml` beside the +3 and the +1; `occupation_broken_offence
  = 2` in the Relations table, the first weight-2 offence in the game, where the 1 and the 3 are
  still literals in the code.
- One Report line for both parties; the previous holder is named in it, or *nobody*.
- The computer is unchanged: it already stays by weight (#284) and digs in while it occupies
  (#297), so it pays this cost only when its occupier is destroyed.

## Measured

Twenty seeds with the Custodians first: 5 / 13 / 0 / 0, no Occupation begun and none broken, so
the rule was not exercised in this batch; the ground war on this seating stopped under the defence
ticket's rule. The closing sweep reads all four seatings; last version 67 Occupations began and 2
broke over eighty games.

## Witnessed

`a_broken_occupation_hands_the_place_back_at_a_cost` was watched red with the offence removed:
*a rung-2 offence from the previous holder: left 0, right 2*. `351 passed`, `6 passed`, clippy
clean with `-D warnings`.
