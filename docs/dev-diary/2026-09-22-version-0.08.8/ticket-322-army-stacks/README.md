# Armies that stack for orders

Ticket [#322](https://github.com/whaleyjoshua2/Dying-Earth/issues/322) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`china-stack-row.png`](china-stack-row.png) | `shot: select:eastasia army:1 panel:0 window:1280x1080`. China's Armies block with two Armies, its own and one raised: under the stance row the **stack row**, *All 2 (8):* with **attack Russia**, **attack India**, **attack Indonesia**, **attack Japan**, one click moving both; beneath it each Army's own row with its own buttons, kept for a split. |
| [`china-stack-hover.png`](china-stack-hover.png) | `shot: ... tip:chance to win the first exchange: your 8`. The stack row's *attack Russia* hover: *61% is the chance to win the first exchange: your 8 against their 6*, the stack's summed strength where a single Army's row reads *your 4* at 35%. |
| [`ships-all-that-can.png`](ships-all-that-can.png) | `shot: battle:1 stack:1 panel:0`. The Custodians' Ship stack card at Mars with two Frigates: under *Transits (whole stack)* each destination now carries **All 2 that can** (Phobos, Deimos, whose leg the tanks pay) or a greyed **All 0 that can** (Earth, the Moon, Venus, at 39 or 20 Fuel against 30 in the tank), beside the per-Ship buttons that stay for a split. |

## What was decided, in the designer's words

*"q1 that q2 yes q3 yes q4 yes q5 yeah good point"*: a stack is every Army of one seat at one
place; one row of neighbour buttons moves it, the per-Army rows kept only when it has more than
one; the arrival as one Army's; Repair all; the Ship stack's Transits made what the heading says.
The sixth question, the computer's stack march, was not answered and was built as recommended
under the map's standing rule that a rule change reaches the computer seats in the same ticket.

## What was built

- **No new engine order.** A stack's march, transit or repair is the per-unit orders placed
  together: `orders_button` places a `Vec<Order>` when every one is legal on its own, priced at
  their sum, refused for the first one's reason. So the save format is untouched and the arrival
  rule is the one each Army already has.
- **The card**: the stack row under the stance row when two or more Armies of the player's may
  march (not dug in), the odds hover from the summed strength; *Repair all* and *Repair all with
  Ducats* when two or more are damaged; with one Army, its own row is the stack and nothing is
  drawn twice.
- **The Ship stack card**: *All N that can* per destination when the stack has two or more Ships,
  N being those whose tank pays the leg.
- **The computer seats** (`ai.rs`): where two or more Armies of a seat at a place may march, one
  candidate moves them all, its odds from the summed strength, offered before the single marches
  (the one-war-a-turn cap goes to the first candidate pushed) and weighed as a raised Army's march
  when any of the stack is raised, at half when it is all the Region's own.

## Tested

`the_computer_marches_a_stack_where_one_army_alone_would_not_clear_the_bar` (one Army of 4 against
Russia's 6 is under the bar and does not march; two of 4 are over it and march together) was watched
to fail with the stack candidate disabled, then to pass; the suite is green at 359.

## The batch: the computer masses

`batch-after.txt`, 20 seeds x four seatings, against the batch after ticket #321:

| | after #321 | after #322 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 45 / 20 / 1 / 0 | **43 / 23 / 0 / 0** |
| collapses | 14 of 80 | 14 of 80 |
| marches on held Regions | 120 | **332** |
| Occupations begun / broken | 122 / 9 | **229 / 31** |
| places taken by force | 95 | **165** |
| Standing Armies lost | 80 | **153** |
| Battles opened | 142 | **216** |

The playtest note's *"it never masses"* is false now: the marches on held Regions nearly tripled
with Armies built barely changed (90 to 111), which is stacks marching where singles would not
have cleared the bar. The Arkwrights-first seating carries most of it (196 marches, 87 Standing
Armies lost) and its collapses went 12 to 13 of 20. The column moved two wins toward the
Prospectors and the Arkwrights lost their one. Measured behaviour under the computer's habits; the
closing sweep judges the whole.
