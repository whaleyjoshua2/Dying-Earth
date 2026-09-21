# Two Colonists on every starting station, and two Pioneers for the Arkwrights

Ticket [#290](https://github.com/whaleyjoshua2/Dying-Earth/issues/290) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

| picture | what it shows |
|---|---|
| [`iss-card-turn-one.png`](iss-card-turn-one.png) | `shot: hab:1 panel:0`, 1280x800. The ISS's card on turn 1: **Colonists 2 of 4 room**, **Modules 0 of 2 (one for each Colonist)**, the Core Module tile and **two free tiles** that say *Click to Build*. The threshold line reads *40 - 20 for the station + 20 for 2 Colonists*, where a bare ISS read 20. The top bar's *Space 30M* is the six people aboard the three stations. |
| [`china-as-the-arkwrights.png`](china-as-the-arkwrights.png) | `shot: player:arkwrights select:eastasia panel:0 window:1280x1500`. China's card as the Arkwrights on turn 1, tall enough to reach the orders block: **Pioneers waiting: 2** above *Recruit 8 Pioneers*, and the population at its card figure, 290.9, undebited. An 800-pixel window cuts the card above this block, which is why the first capture showed no Pioneers line at all. |
| [`tutorial-note-2-habitat.png`](tutorial-note-2-habitat.png) | `shot: menus:1 tutorial:2`. The second note, new: **Build a Habitat on the ISS**, with the two aboard and the one-slot-per-Colonist rule in its quieter line. The roster behind it reads *ISS over Earth: 2 Colonists, 0 Modules*. |
| [`tutorial-note-4-send-them.png`](tutorial-note-4-send-them.png) | `shot: menus:1 tutorial:4`. The fourth note, moved from third: **Send them to the ISS**, *the ISS holds eight now, two of them already there, so all four fit*. |

The measurements, twenty seeds with the Custodians first: [`sweep-20-custodians-first.txt`](sweep-20-custodians-first.txt)
and [`sim-20-custodians-first.txt`](sim-20-custodians-first.txt), and the control with the computer's
opening switched off, [`sweep-20-control-opening-off.txt`](sweep-20-control-opening-off.txt).

## What was decided, in the designer's words

- *"q1 adding build slots to start"* -- the two Colonists are there so a **starting station can
  build from turn one**: two aboard, two Module slots free, two berths left in the Core Module.
- *"q2 from nowhere"* -- they are the station's crew; **no Region is debited** for them.
- *"q3 starting region for the arkwrights"* -- the Arkwrights' two Pioneers **wait in their start
  Region** for the first Ship.
- *"q4 a gift"* -- outside Coach Class, population untouched.
- *"q5 shift the tutorial to have them build a habitat first then the four, you can add a turn of
  guidance if necessary and yes the computer should know this"* -- the tutorial chain becomes
  spend, **build a Habitat**, recruit four, send them up; and the computer gets an opening.
- *"q6 six notes"* -- one ask a turn, six notes.
- *"q7 opening rule"* -- the Habitat pushed first, not left to its ordinary weight.

## Settled by the builder, to be corrected if wrong

- **Two data figures on the Faction card**: `start_colonists = 2` beside each `start_station`,
  `start_emigrants = 2` on the Arkwrights' (spelled as the engine spells the field it fills). The
  loader refuses `start_colonists` without a station, or more than the Core Module holds.
- **The ISS is dearer to take**: its Influence threshold reads 40 (20 for the station, 10 a
  Colonist) where a bare one read 20. A consequence of the two aboard, not a rule of its own.
- **The opening is played once.** The computer's Habitat on a starting station is pushed at the
  opportunity weight and counted as advancing its Victory pace, the way its first Shipyard is, and
  only **until the station's first Habitat stands or is on order**, and **not while Energy is
  tight** (a Habitat draws Energy; with Energy at nought the Solar Array the station wants lost its
  slot to it, on the ticket #89 test).
- **The tutorial tick's hover** reads the count of notes from the table rather than saying "five".
- **The sim and the sweep print the opening**: Modules beyond the Core on a starting station at
  the end of turn 3, by seat.
- Ten older tests reasoned about the board with the ISS bare (people off Earth, a draw by equal
  margin, the schooling the four carry, the cap rule from nought); each now empties the starting
  stations first through one helper and says why, rather than carrying two strangers in its
  arithmetic.

## Measured

**The opening took.** In every one of twenty seeds the Custodians and the Archivists have two
Modules on their station by the end of turn 3 and the Prospectors one (the Habitat; their next is
the Trade Post): the sweep line reads `[40, 20, 0, 40]`, nought for the Arkwrights who have no
station. With the rule off, one a station: `[20, 20, 0, 20]`.

**The rule as first written was not an opening.** Gated only on "under four berths empty", it fired
again every time the muster filled the station, so the Prospectors put Habitat after Habitat on
Tiangong at the head of every list and never the Trade Post that earns their Ducats: over twenty
seeds **10 wins and 330 Ducats a game**, against **19 wins and 2287** with the rule off. The
once-only clause was written for that measurement.

**Against the 0.08.5 baseline, same seating** (`final-0.08.5.txt`, the Custodians first):

| | 0.08.5 | this ticket | the control (opening off) |
|---|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 0 / 16 / 0 / 2 | **5 / 13 / 0 / 0** | 0 / 19 / 0 / 0 |
| collapses of 20 | 2 | 2 | 1 |
| Ducats made a game, median by seat | 1099 / 1053 / 31 / 487 | 1251 / 1254 / 30 / 399 | 1030 / 2287 / 29 / 53 |

Two aboard alone (the control) hands the Prospectors three more wins and doubles their Ducats; the
opening gives some of that back to the Custodians. Twenty seeds of one seating; the closing sweep
reads all four.

## Witnessed

`a_starting_station_opens_with_two_aboard_and_the_arkwrights_with_two_pioneers` failed on the old
rules with *two aboard from the start: left 0, right 2*; `the_computer_opens_with_a_habitat_on_its_starting_station`
failed with the Prospectors opening on a Shipyard and a Research Lab and no Habitat, and failed
again with the opportunity weight alone before the Victory-pace clause. `346 passed`, `6 passed`,
clippy clean with `-D warnings`.
