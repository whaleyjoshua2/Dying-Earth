# Ground Armies that can actually be moved: the playtest report

Ticket [#321](https://github.com/whaleyjoshua2/Dying-Earth/issues/321) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`china-standing-army-marches.png`](china-standing-army-marches.png) | `shot: select:eastasia panel:0 window:1280x1080`. China's card on a fresh board: under **the 1st Chinese Army (Custodians, standing)** the four buttons **attack Russia**, **attack India**, **attack Indonesia**, **attack Japan**. On 0.08.7's board this row had no buttons at all, and a player who had raised nothing could move nothing, which is the playtest report. |

## What was decided, in the designer's words

*"q1 no they can march q2 no those raised armies change hands q3 yes q4 sure q5 yes"*: a Region's
own Army marches again, reversing 0.08.6's rule (#302) that it stays at home; a raised Army keeps
changing hands with its home Region; the turn of raising and a Colony's Army as they were; the
playtest note says the path.

## What was built

- **The refusal is gone** (`orders.rs`): the holder may march a Region's own Army like any Army.
  Its people's defence, the Constabulary and the calm, is its **only while it stands at home**
  (`army_at_home`, `army_defence`); marched out it is an Army like any other, and a threat next
  door like any other (`nearest_army_threat`, the computer's `enemy_army_near`).
- **The card** draws march buttons under every Army of the player's; the Standing Army's row hover
  says it may march and what it loses away from home; the threat line's hover says a Region's own
  Army at home is no threat and marched out it is.
- **The computer seats** march their own Armies too, **last and at half weight**: raised Armies
  are offered first, since one new war a turn is the cap and the first candidate pushed takes it,
  and a Standing Army's march is weighed at half, so a computer seat empties a Region of its own
  defence only when nothing else will serve. Without the ordering an existing test caught the
  computer marching its Standing Army where the raised one was meant to go.
- **A Carrier still does not carry a Region's own Army** (`orders.rs` Load): the designer's word
  was *march*, and lifting a Region's whole defence off the planet is a different act; said here so
  it can be corrected.
- The **Army** and **Standing Army** glossary entries and the playtest note's *How to play* say the
  rule.

## Tested

`a_regions_own_army_marches_again_and_defends_only_at_home` (the holder's march allowed; the
defence bonus at home and none away; the Army marched out a threat next door) was watched to fail
with the refusal put back, then to pass; the workspace suite is green at 358.

## The batch: the ground war returns

`batch-after.txt`, 20 seeds x four seatings, against the batch before this ticket:

| | before | after |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 21 / 2 / 0 | **45 / 20 / 1 / 0** |
| collapses | 14 of 80 | 14 of 80 |
| marches on held Regions | 5 | **120** |
| Occupations begun / broken | 2 / 0 | **122 / 9** |
| places taken by force | 2 | **95** |
| Standing Armies lost | 2 | **80** |
| Battles opened | 60 (55 in orbit) | **142** (47 in orbit) |

**The ground war 0.08.6 had shut is open again**, and it is the Regions' own Armies fighting it:
Armies built barely moved (89 to 90 over the batch) while marches on held Regions went from 5 to
120 and Standing Armies lost from 2 to 80. The computer marches its own Army out at half weight
and still finds the odds, since a Region emptied by its neighbour's march is undefended. The
column barely moved, because the seats that fight (the Custodians and the Prospectors in the
first three seatings) trade places by force and the two that do not (the Arkwrights and the
Archivists) win as little as before. **Measured behaviour, not a rule**: the computer's half
weight and its 60% bar are its habits. This is the version's first large move of the military
block, as the map's Notes said this list would bring; the closing sweep will say where it settles
once the stack, escort, rolls and Bombard tickets are in.
