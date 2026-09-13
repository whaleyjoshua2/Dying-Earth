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
