# A building shown in its box the moment it is ordered, and marked while it is built

Ticket [#291](https://github.com/whaleyjoshua2/Dying-Earth/issues/291) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

| picture | what it shows |
|---|---|
| [`china-factory-ordered.png`](china-factory-ordered.png) | `shot: select:eastasia order:factory panel:0 window:1280x1100`. China's card on turn 1 with a Factory **ordered and not yet committed**: the fifth box is hatched and dimmed, **ordered** in its bottom-left corner and **1 turn** in its top-right, the name beneath; the header reads **4 of 9 slots free, 1 ordered this turn**, where it read 5 of 9 over a bare dashed box before this ticket. |
| [`china-power-plant-building.png`](china-power-plant-building.png) | `shot: order:powerplant commit:1 select:eastasia panel:0 window:1280x1100`. The turn after a Power Plant was ordered: the same box, now **building** with **1 turn** to go, and the header at 4 of 9 with no order pending. Before this ticket the box was bright under the hatch and carried no count. |
| [`iss-habitat-ordered.png`](iss-habitat-ordered.png) | `shot: hab:1 morder:habitat panel:0`. The ISS's card with a Habitat ordered: the tile hatched and dimmed, **ordered**, **1 turn**, beside the one free tile left; the orders list beneath carries the same order with its cancel. |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

- *"q1 both hatch stays but greyed out with turns to complete indicated"* -- the ordered building
  **shows in its box at once**, and both the ordered and the building box keep the **hatch, greyed**,
  with the **turns to complete** on the face.
- *"q2 yes"* -- the count is on the face; the hover keeps the ready turn.
- *"q3 right click cancels"* -- a right-click on an ordered box cancels the order.
- *"q4 yes as well as stations"* -- the same on the Colony and station cards' Module tiles.

## Settled by the builder, to be corrected if wrong

- The word on a box whose order is not yet committed reads **ordered**; a queued one reads
  **building**. The count stands in the opposite corner from the word, so neither crosses the
  picture.
- An ordered Facility is drawn in the coastal or inland group its slot will take, decided the way
  the rule decides it at End Turn, the pending orders ahead of it taking theirs first; and it is
  drawn as the Faction's own kind, which is what the order raises.
- The **Facilities header** subtracts the ordered slots and says how many were ordered this turn;
  the Module cap line on a Colony card is unchanged, since it counts what stands.
- The right-click goes through the same cancel the orders list's button uses, so a cancelled Max
  still ends its standing order and later orders are re-checked.
- The hover on an ordered box says *right-click to cancel the order*; a building box cannot be
  clicked, as before.
- Two building aids for the screenshot harness: `order:<facility>` and `morder:<module>` place an
  order and leave it pending; `commit:1` ends the turn with it, which `turns:` cannot do.

## Looked at, not tested

An interface change; `346 passed`, `6 passed`, clippy clean with `-D warnings`. The three pictures
are the check, each looked at before filing. The first capture of the ordered box showed the header
still saying 5 of 9 free over four free boxes, which is why the header now counts the order.
