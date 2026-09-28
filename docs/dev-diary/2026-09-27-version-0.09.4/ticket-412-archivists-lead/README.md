# The Archivists' reward for leading a Tech (ticket #412)

[Ticket #412](https://github.com/whaleyjoshua2/Dying-Earth/issues/412); the spec is §8 of
[`docs/spec/version-0.09.4.md`](../../../spec/version-0.09.4.md). Measured first (the sweep gained a
line, the Techs each Faction led: the Archivists 126 of 1,656 over 80 games), then decided in one
round (*"q1 a q2 ok q3 a"*).

## The red witness

`the_archivists_are_paid_for_leading_a_tech`, red before the rule (*"Seat(3): the Influence", left 0,
right 5*); green after, for the Archivists (+5 in the Allotment, every Region they hold eased by a
half, the line's tail) and for the Prospectors (nothing).

## The sweep

[`../sweeps/before-412.txt`](../sweeps/before-412.txt) and [`../sweeps/after-412.txt`](../sweeps/after-412.txt):
the Archivists' leads 126 to 130, their held Region-turns at 7+ 11% to 9%, their wins 5 and 5.

## No picture

The line shows only on a turn the Archivists lead, which the picture aids cannot stage without a
new one; the test pins its words.

## The review

An agent that did not build it found nothing blocking; fixed from it: **"at once" was not always
true** (a Tech finished by a card answered during the Orders, or by a player's pick at End Turn,
paid at the next Income); the Influence is now paid straight into the Allotment whenever the Tech
completes and never twice, the test driving a real End Turn and watched red with the double
payment let through (*left 17, right 12*). **The line no longer claims Unrest nothing eased**
(holding no Region: *"…: +5 Influence."*). The reward is logged. Left and noted: a Tech completed by
a player's own pick at End Turn has its line wiped with the old Report, as it always has.
