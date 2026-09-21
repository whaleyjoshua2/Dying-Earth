# Dig In: a third live stance for Armies

Ticket [#297](https://github.com/whaleyjoshua2/Dying-Earth/issues/297) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

| picture | what it shows |
|---|---|
| [`china-stance-row-dig-in.png`](china-stance-row-dig-in.png) | `shot: select:eastasia panel:0 window:1280x1500`. China's card down to the Army orders block: the stance row reads **Attack, Hold, Dig In, Evade**, Hold lit, with the four attack buttons beneath. The Standing Army's row above reads *strength 5, damage 0/5* and no "dug in", since a held Region's Standing Army takes its holder's order. |
| [`map-neutral-shields-trench-line.png`](map-neutral-shields-trench-line.png) | `shot: panel:0`, 1280x800. The Earth map: the neutral Regions' grey shields (Egypt 3, Nigeria 3, Brazil 3, Iran 4) each carry a **trench line beneath**, since a neutral's own Army is always dug in; the held Regions' shields (China 5, Europe 5, Saudi Arabia 4) carry none. |
| [`egypt-always-dug-in.png`](egypt-always-dug-in.png) | `shot: select:northafrica panel:0 window:1280x1100`. Egypt's card: **the 1st Egyptian Army (neutral, standing): strength 3, damage 0/3, dug in: +2 defending**. |

The measurement, twenty seeds with the Custodians first: [`sweep-20-custodians-first.txt`](sweep-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 that it can not move to attack without changing it stance and waiting a turn"* -- a dug-in
  Army cannot march or attack until its stance is changed and the turn has passed; and the
  recommended reading of the cut clause: **a neutral's Standing Army is always dug in**, a held
  Region's on its holder's order.
- *"q2 no bonus to hitpoints"* -- +2 strength while defending; hit points do not follow.
- *"q3 the first"* -- the march and the Carrier lift are refused while dug in; a stance order
  beside them this turn does not lift that, since it takes effect at the Resolution.
- *"q4 yes"* -- it takes effect at the Resolution of the turn it is ordered.
- *"q5 yes"* -- the computer digs in where a rival's Army stands next door and it has no cause to
  attack, and wherever it occupies.
- *"q6 yes"* -- a word on the stance row, the roster tooltip, a mark on the map shield, a Report
  line.

## Settled by the builder, to be corrected if wrong

- `[dig_in] defence = 2` in `units.toml`; a `stance_dig_in` weight in every Faction block of
  `ai.toml`, at Hold's weight, taking Hold's place when the seat is threatened or occupying (and
  the occupier's threefold weight with it).
- **A dug-in Army is never an aggressor**, since its stance is not Attack, so the bonus is a
  defender's by construction; a neutral's Army is never one either.
- **A Levy follows the Standing Army of its Region**: always dug in while neutral, which is the
  only time a Levy exists.
- The **mark on the map** is a short line in the shield's own colour beneath it. The Report line
  reads *The Custodians Armies dig in at China* and is written the turn the order commits.
- A Ship is refused Dig In outright.

## Measured

Identical to the defence ticket's figures: 5 / 13 / 0 / 0 wins, 22 Battles all in orbit, no
marches on held Regions. The ground war on this seating had already stopped under the defence
rule, so Dig In had nothing left to defend against; 19 Dig In orders were committed over the batch
(3 / 1 / 12 / 3 by seat), the Arkwrights digging in most where the Prospectors' Armies stand next
door. The closing sweep reads all four seatings.

## Witnessed

`a_dug_in_army_fights_two_stronger_never_disengages_and_cannot_march_until_it_digs_out` was
watched red twice: with the disengage skip removed (*dug in, it never rolls to disengage*) and
with the march refusal removed. `349 passed`, `6 passed`, clippy clean with `-D warnings`.

A note for the next builder: restoring a mutated file with `git checkout --` restored it to the
last **commit**, which wiped every uncommitted Dig In edit in that file; the mutations are now made
and undone by a script that touches only the lines under test.
