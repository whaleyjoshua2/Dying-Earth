# Version 0.07.5, the reading-and-founding version: the pictures

The map is [Map: version 0.07.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/160). Every
picture here was taken headlessly in `shot:` mode on `version-0.07.5`; nothing was opened on the
designer's desktop.

## Every word of the Influence section explained on hover

Ticket [#161](https://github.com/whaleyjoshua2/Dying-Earth/issues/161). The designer's line:
*"influence mouse over on all words in the influence portion of the nation card."* One line of that
section had a hover; eight things did not. Built as the ticket's recommendation for the designer to
react to: hovers on all eight, each naming the rule behind its figure rather than restating it, as
the tooltip rule set on ticket #116 requires.

![The Influence heading: what Influence is, which nothing on the card said](influence-hover-heading.png)

![The Threshold figure: what sets it, and what lowers and raises it](influence-hover-threshold.png)

![Your own chip: what it would take a rival to reach you, and what spending here does](influence-hover-own-chip.png)

![The holder line: by what rule the holder keeps it, or an occupier takes it](influence-hover-holder.png)

The other four are the **within-reach warning** (which never named the challenge margin behind its
own figure), the **Blame note** (four rule-governed figures and no hover at all), the **Influence
value** line (which names the Allotment and never says what an Allotment is), and **`nobody has any
yet`**. The Colony's card followed, and its three controls -- `Influence:`, `Spend` and `Buy more
Influence in the Trading window`, which the Nation card lost on ticket #121 -- gained hovers of
their own.

**Decided by the designer** off those pictures: *"revise verbage to be the most efficient possible
in conveying information"*, so every one of the new hovers was tightened -- the heading's went from
six rendered lines to four with no fact dropped; **your own chip says what it would take to be safe
from the nearest rival** rather than repeating the rule; **Threshold gains an entry in
`CONTEXT.md`**, which it never had, saying plainly that this is the Influence sense and not the
climate one; and the Colony card **follows the same choices as the Nation card** rather than being
special-cased.

`icon_word` now hands back its response so a heading drawn with a glyph can carry a hover, which is
what the Influence heading and the Influence value line needed.

## The Module grid moves onto the card, and the Hab View window retires

Ticket [#162](https://github.com/whaleyjoshua2/Dying-Earth/issues/162). The designer's line: *"Move
the station module grid to its in the card like nation states and retire the window."* Built as the
ticket's recommendation, with one change the pictures forced: **six columns, not three.**

![A station's card: its Modules as tiles, a free one clicked, the build buttons in the strip](modules-on-the-card-six-columns.png)

![A Module clicked: its figures and its Mothball and Decommission buttons in the strip](modules-on-the-card-tile-clicked.png)

![A bare station at turn 1: three free places and the strip's hint](modules-on-the-card-bare-station.png)

**Why six and not three.** Three columns fit the side panel at its default width, which is what the
ticket recommended. Built, they push the strip below the fold on any station with room to grow: the
ISS at turn 13 has thirteen places, which is five rows of tiles, and the build buttons a free tile
is clicked for are off the bottom of the screen.

![Three columns: the first free tile is selected and its build buttons are below the fold](modules-on-the-card-three-columns.png)

Six columns is also what *"like nation states"* means literally -- the Region card's boxes are six
across -- and it widens the card to the width the Region card already takes, so the two read as one
thing. The cost is that a Colony's card covers more of the globe than it did.

**What retired with the window**: the `Modules (M)` button, the list of Module names (the tiles say
it better), the **M** key, the Esc branch that closed the window, the roster row's auto-open, and
the window's header and `Esc closes.` tail. `view.hab_view` is gone from the view state. Esc now
clears a clicked tile, which is what it did by closing the window. The `hab:1` picture aid
**selects** seat 0's first station or Colony instead of opening a window, and `habtile:` stands on
its own as `slotbox:` does. **The Module build buttons live only in a free tile's strip**, as a
Facility that takes a slot has been buildable only from a free box since ticket #154; the card's
Build section keeps the Army, the Ships and the Archivists' Archive, which take no Module slot, and
the Build-Where-You-Dig discount note moved up to stand above the tiles where it says what a Module
here will cost.
