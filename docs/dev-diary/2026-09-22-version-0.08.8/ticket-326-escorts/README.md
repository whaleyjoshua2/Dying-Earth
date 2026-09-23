# Escorts take the fire

Ticket [#326](https://github.com/whaleyjoshua2/Dying-Earth/issues/326) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

No picture: the rule has no surface of its own. It shows in a Battle line in the Report, where
a Colony Ship beside a Frigate now reads *took 0 hits* while the Frigate takes them, and the
Report's line is the one ticket #281 drew.

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes q4 yes"*, every one as recommended: the strict rule; Ships only, by a
flag the caller sets from the hull; the retreat covered too; the computer seats' habits left
alone this ticket.

## What was built

- **`Combatant.armed`**, true unless the caller says otherwise: the resolution sets it from the
  hull for a Ship (a Frigate or Battleship armed, a Colony Ship or Carrier not); a Battery and an
  Army are armed by default. The melee itself still knows no kinds.
- **The draw**: `random_engaged` draws uniformly among the party's engaged ARMED units while it
  has any, and among its unarmed ones only when none does. So a Colony Ship beside one Frigate
  takes no hit until the Frigate is destroyed or has disengaged, where before it took half.
- **The retreat**: in `pursue`, an unarmed leaver whose party still has an armed unit engaged is
  marked pursued and skipped, so the standard escort play, the Frigate holds and the Colony Ship
  runs, is the play it says it is. An escaping warship is pursued as before.
- The Carrier's card comment, *"Unarmed, so it needs an escort"*, is true now and says since
  when; `CONTEXT.md` **Battle** carries the rule.
- The test helper `colony_ship` builds an unarmed hull, so every older Battle test that uses it
  now runs under the rule; all of them still pass, since none stood a Colony Ship beside a
  warship.

## Tested

`escorts_take_the_fire_and_cover_the_retreat`, with scripted dice: a Frigate attacks a Colony Ship
that stands FIRST in its party beside an escort Frigate, so a uniform draw with the scripted pick
would strike the Colony Ship first. All seven landed hits go to the escort until it falls at
four, then three to the Colony Ship. Then the same with the Colony Ship on Evade: it escapes at
the start, no pursuit die is rolled for it (the d6 script is empty and would panic if one were),
it takes no hit, and its escort takes the fight. Watched to fail with the cover switched off (the fight took a
different course and ran the scripted dice dry), then to pass restored. The suite is green at 363;
clippy clean under `-D warnings`.

## The batch: nothing to see yet

`batch-after.txt`, 20 seeds x four seatings, against the batch after ticket #325:

| | after #325 | after #326 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 22 / 1 / 0 | 43 / 22 / 1 / 0 |
| collapses | 14 of 80 | 14 of 80 |
| warships lost | 2 | 2 |
| Battles opened | 223 | 226 |

The rule fires only where an unarmed hull stands in a Battle beside a warship, and the computer
carries no unarmed hull into one on today's board (measured on ticket #319: no Carrier built, no
Army landed), so the batch is the batch it was. Measured behaviour under the computer's habits;
the rolls-per-warship ticket next changes the odds and is where the escort habit is looked at
again, as decided.
