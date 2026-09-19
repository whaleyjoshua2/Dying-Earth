# What this Faction has under way

Ticket [#263](https://github.com/whaleyjoshua2/Dying-Earth/issues/263) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

`underway:1` is a new shot aid: seat 0 with a Factory on order in its start state, a Mine on order
at a Colony on the Moon, and a Frigate three turns out on the road to Mars.

| picture | what it shows |
|---|---|
| [`faction-window-under-way.png`](faction-window-under-way.png) | `shot: factions:1 underway:1 panel:0`. The Faction window on the player's page, the new **Under way** block under Holdings: *Building: a Mine at Mare Tranquillitatis on the Moon (1 turn), a Factory in China (3 turns)* and *In transit: TSV Vanguard, Earth to Mars, 3 turns*. |
| [`under-way-block.png`](under-way-block.png) | The block, magnified. |
| [`first-build-arrow-box.png`](first-build-arrow-box.png) | The first build, kept as a lesson: *Earth □ Mars* -- the interface font has no arrow, and *a Factory at China*, where a Region wants *in*. Both found by the picture, neither by the suite. |

## What was decided, in the designer's words

*"yeah that"* -- the list in full on every page, a rival's included: a build stands hatched on its
card and a transit is drawn on the Solar System Map for anyone to see, so the disclosure rule
hides nothing here. *"looks good"* -- two lines under a bold heading, *Nothing under way* when there
is nothing. *"no"* -- Pioneers waiting or at sea are not listed. *"soonest first"*.

## What it reads

`Game::under_way(seat)` -- every build the seat **ordered** (`Build.seat`, so a build begun in a
Region since lost stays with whoever paid for it), with the turns until it lands, and every Ship
of the seat's in transit with its name, its road and the turns left; both sorted soonest first,
name breaking a tie. The Report or the AI could read the same list.

## Witnessed red

The helper was written first in discovery order and the test -- a Mine landing next turn queued
after a Factory landing in three -- run against it: `left: ("Factory", …, 3), right: ("Mine", …,
1)`. Then the two sorts went in and it passed; `320 passed`, clippy clean.
