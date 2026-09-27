# Ticket #399: Attack from a Ship's own card

The designer's item: *"would appear there is no actual way to order a frigate or battle ship to
attack now that each ship has its own card."* Decided in one round of two (*"q1 a q2 b"*): the
stack's stance and Attack on every warship's own card, labelled as the stack's; nothing on a Ship
that cannot fight. [The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/399) is
the authority, [§15 of the spec](../../../spec/version-0.09.3.md#15-attack-from-a-ships-own-card)
records it.

## What was built

The stack card's stance row and Attack block are one function, `attack_block`, drawn on the stack
card as before and on a Frigate's, Battleship's or Missile Carrier's own card under its orbit line,
headed as the stack's. The Attack button asks the engine's `attack_has_a_target`, made public, so
it is greyed with the refusal on its hover where the stack card's Body-wide test showed it live and
left the confirm to be refused. A picture aid, `foe:low` and `foe:ring`, puts a Frigate of the
player's in Mars's low orbit and a rival's in the same orbit or in a station's ring.

## Before

**The stack card with the rival in another orbit**, `seed:7 foe:ring stack:1 panel:0
window:1400x1400`: the card's own odds line says *No rival stands in an orbit of yours here, so an
Attack ordered here fights nobody*, and the *Attack this turn* button under it is live all the same,
its confirm bound to be refused.

![The stack card before: a live Attack button above a line saying it fights nobody](before-stack-card.png)

**The Frigate's own card**, `seed:7 foe:low stack:1 ship:1`, with a rival Frigate in its orbit: no
stance row and no Attack, the stance a line of text with a link back to the stack card, which is
the designer's complaint.

![The Frigate's card before: no way to attack](before-ship-card.png)

## After

**The Frigate's own card with a rival in its orbit**, `seed:7 foe:low stack:1 ship:1`: under its
orbit line, *The stack's stance and Attack: all 1 Ship(s) of yours at Mars*, the stance row, the
odds, and *Attack with all 1 Ship(s) at Mars*, live.

![The Frigate's card after: the stack's stance and a live Attack](after-ship-card-live.png)

**The same card with the rival in the station's ring**, `foe:ring`, the button's hover forced
(`tip:no rival Ship`): the button greyed, its hover *Attack: no rival Ship or Battery in any orbit
your Ships hold there*, the engine's own refusal.

![The Frigate's card after: the Attack greyed with the reason](after-ship-card-refused.png)

**The stack card in the same case**: the button that was live above a line saying it fights nobody
is greyed with the same reason.

![The stack card after: the Attack greyed with the reason](after-stack-card-refused.png)

## The red witness

`an_attack_has_a_target_only_in_an_orbit_the_seat_shares_with_a_rival`: the engine's predicate
and the order's check agree, a rival in the ring against the player's Frigate in low orbit being
no target, the same orbit a target. The defect was the card's, not the engine's, so its witness
is the pictures: the stack card's button live above a refusal before, greyed with the reason after.

No rule moved; no sweep.
