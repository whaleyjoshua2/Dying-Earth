# The Arkwrights' Unrest eased by every place they found off Earth (ticket #419)

[Ticket #419](https://github.com/whaleyjoshua2/Dying-Earth/issues/419), added by the designer during
the version; the spec is §12 of [`docs/spec/version-0.09.4.md`](../../../spec/version-0.09.4.md).
Decided in two rounds (*"q1 a q2 only arkwrights holdings and taken places count for nothing q3
sounds good q4 let's acutally make this part of their unrest management strategy"*, then *"q5 b"*).

## The red witness

`the_arkwrights_foundings_ease_their_unrest`, against a control game that makes the same turn
without the founding (the Moon's first Colony eases every Region by its own half point, #395, which
the test counts): red with the Arkwrights' two figures set to nought; green after, a Colony a point,
a station half, a rival's founding nothing, the lines' tails, and the computer's pull one at 3,
1.5 at 6.5, double at 10.

## The sweep

[`../sweeps/after-419.txt`](../sweeps/after-419.txt) against [`../sweeps/after-413.txt`](../sweeps/after-413.txt):
the Arkwrights' Regions at 7+ 360 to 355 held Region-turns, stations on the Moon 15 to 20, wins and
collapses unmoved. The ease fires seldom.

## No picture

The line's tail is pinned by the test.

## The review

An agent that did not build it found every founding door as decided (the Arkwrights' own, Antarctica
and a taken place excepted, not at game start) and nothing that changes what the game does. Fixed:
a doc comment had slid onto the new function; the test now founds in Antarctica and sees nothing
eased; the spec says plainly that **the pull on a ground Colony does nothing measurable** (it sits on
the landing, which a loaded Ship in low orbit takes anyway; the stations are what moved), which is
put to the designer. Noted: the line states the rule's figure, and a Region nearly calm eases by less.
