# Dying Earth — version 0.09.6, the growth version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/435), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-03-version-0.09.6/`](../dev-diary/2026-10-03-version-0.09.6/).

## 1. The crash on a Colony Ship at a Venus station

The authority is [ticket #436](https://github.com/whaleyjoshua2/Dying-Earth/issues/436). No rule
moves.

**Before:** a Colony Ship's card offered "Unload N into <place>" for each of your places at its Body
that had room. It named each place by its **ground** slot, and an Army landing did the same. For a
station the slot number counts stations, so:

- every station over **Venus**, which has no ground slots, crashed the game when the card opened;
- Earth's 4th and 5th stations would have crashed it too;
- every other station was named for a ground site on its Body.

**After:** a station is named for its station slot and a ground Colony for its ground slot, on both
buttons. The designer's save (turn 33, a Colony Ship at Aphrodite over Venus) opens its card and
offers "Unload 4 Colonists into Aphrodite".

Every other ground-slot lookup in the window was checked; each one is on a path that only ever
names a ground slot.

**What would show this wrong:** a Ship card that panics at any station, or a button that names a
ground site for a station.
