# The rival's Moment

Ticket [#261](https://github.com/whaleyjoshua2/Dying-Earth/issues/261) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`rival-moment.png`](rival-moment.png) | `shot: menus:1 rival:1 moment:rival`. The Report picture's place, with the ninth kind of Moment open: **"9 of 12"** in the Prospectors' orange, and *"The Prospectors are three quarters of the way to their Victory Condition: Off-world Presence at 9 of 12."* The `rival:1` aid stands seat 1 at nine Colonists on the Moon and the Fund at 2000 of 2500 -- a score of 0.75 -- and runs a quiet turn so the Moment fires. |

## What was decided, in the designer's words

- *"as suggested"* -- **two steps**: the score (the lower of the two parts' fractions) reaching
  **three quarters**, and **one part met with the other short**. The suggestion's "one turn from"
  wants a projection the engine does not make; one part met is the step it can state exactly.
- *"once per step"* -- latched on the seat (`rival_steps_announced`), saved, so a seat that dips
  and recrosses is announced once.
- *"leave the gate tech as it is now"* -- no clause on the Tech-complete Moment.
- *"rivals only"* -- never seat 0.
- *"rank five"* -- between a decisive Battle (4) and a Tech completed (6).
- *"that sounds good"* -- the figure is **the part still short**, since that is what the player can
  still act on: *"...Off-world Presence at 9 of 12"*; and for the second step *"The Prospectors have
  met half of their Victory Condition, Venture Capital Fund. Only Off-world Presence stands between
  them and the game, at 9 of 12."*

Two smaller things settled while building: **a part held back by its gate Tech is not a part
met** (the Fund full with the Extraction Charter unresearched does not fire the second step), and
the figure **wears the rival's colour** -- a `seat` on the Moment, `None` for every other kind.

## Witnessed red

The test was written against the new rule -- three quarters fires once, the gate holds the
second step back, one part met fires once, the player's own seat never, rank 5, nine kinds -- and
run with the kind, the cards and the latch in place but the rule absent: *"three quarters: the
Moment fires: []"*, `left: 0, right: 1`. Then `rival_moments` went into the Victory check and it
passed; `318 passed`, clippy clean with `-D warnings`.

The share is `rival_moment_share = 0.75` in `victory.toml`.
