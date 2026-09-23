# Click an Army, then right-click the map to move it

Ticket [#323](https://github.com/whaleyjoshua2/Dying-Earth/issues/323) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`china-armed-stack.png`](china-armed-stack.png) | `shot: select:eastasia army:1 arm:1 panel:0 window:1280x1080 look:110.0,38.0`. China with its stack armed, as a click on its shield leaves it: the shield (8, the stack's summed strength) wears a **gold ring**, and the four Regions the stack may reach, **Russia, Japan, India and Indonesia**, are **outlined** in the same gold; the card at the right has scrolled to its Armies block. A right-click on any outlined Region would place the stack's march there; the right-click itself cannot be photographed headless. |

## What was decided, in the designer's words

*"q1 we'll try it q2 yes q3 yes q4 do the ships too q5 yes"*: a click on the shield selects the
Region and arms its stack, with a ring, the reachable Regions outlined and the card scrolled; a
right-click on a neighbour places the stack's march, elsewhere a notice, with nothing armed nothing;
a left-click elsewhere, Escape or End Turn disarms; the Ships too; a second right-click on the same
target takes the orders back.

## What was built

- **The armed stack** is a field on the view (`armed_stack`), set by a click on the player's own
  shield (`Hit::Shield`), cleared by any other left-click (at the top of `pick`), by Escape, by End
  Turn and by a change of view. The shield draws a ring in the roster's ring colour; every
  neighbour of the armed Region draws the same ring round its point; the Armies heading scrolls
  into view once.
- **The right-click** (`right_click`): on Earth, resolved to the Region under the pointer by the
  left-click's own ray-cast; a neighbour of the armed Region takes the stack's march, every Army of
  the player's there whose order is legal, placed as the card's *attack X* button places them; a
  Region not next door puts a notice on the panel's notice line (*X is not next to Y: the stack can
  reach only the outlined Regions*); with nothing armed, nothing. On the Solar System Map, with the
  player's Ship stack selected, another Body takes a transit for every Ship whose tank pays the leg,
  as *All that can* does, and a Body none can reach says so.
- **The cancel**: when every order the target would take is already pending, the same right-click
  takes them all back, cancelled from the highest index down so the earlier removals do not shift
  the later ones.
- A `Notice` action carries a sentence to the panel's notice line; an `arm:1` shot aid arms the
  start Region for the picture.

## Looked at, not tested

The picture above, opened and read before this was committed; the first take faced Africa with
China off the globe's edge and was retaken with `look:`. The right-click and the cancel were not
exercised headless and have no test: the orders they place are the card's own, which are tested,
and the resolving of a click to a Region is the left-click's, which the game has used since the
first playable.
