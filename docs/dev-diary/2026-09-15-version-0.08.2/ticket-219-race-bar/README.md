# The Research race bar on the Tech Tree — ticket #219

Two mockups made while grilling [The Research race bar on the Tech Tree: percentages, Faction names, and a
place below the legend](https://github.com/whaleyjoshua2/Dying-Earth/issues/219), on map
[#215](https://github.com/whaleyjoshua2/Dying-Earth/issues/215).

**The window, the legend and the Faction symbols are the game.** They are lifted from real headless
captures (in `caps/`, taken against `main` at the merge of version 0.08.1); only the bar and the
percentages line are drawn. The figures are a plausible mid-game Tech — 24 of 30 paid, with the
Arkwrights having contributed nothing — invented so that the zero case is visible.

Both pictures show **layout (b)**, which the designer chose: the bar at the top of the window, the legend
directly beneath it, the tree below that. They differ in one thing only.

| file | what it shows |
|---|---|
| `a-quiet.png` | **A Faction that has contributed nothing is not shown.** Three entries; the Arkwrights are absent. |
| `b-dimmed.png` | **A Faction that has contributed nothing is shown dimmed at 0%.** Four entries always, so the line never changes length. |

`make-mockups.py` regenerates them: `py -3.12 make-mockups.py caps .`

## Why layout (b) rather than the foot of the window

The ticket read the designer's *"move below legend"* as the bar going to the bottom of the window, under
the legend that sits there today. The captures ruled that out: the **Tech Tree window does not scroll** —
it is `resizable(false)` around a fixed-size tree — so putting anything below the legend makes the
**window itself taller**. At 1600x900 there are about 38 rows to spare beneath it; at the default
**1280x800 the window already reaches the bottom of the screen**. A bar plus a percentages line is about
40 rows, so at the default size the bar would have been pushed off a window that cannot be resized or
scrolled to reach it.

Moving the **legend up** instead satisfies "the bar above, the legend below it" literally, costs no
height at all, and keeps the bar where the eye starts.

## What is visible in them that the words did not say

- **The symbols carry the line on their own.** At 15 pixels each the four are distinguishable without
  reading the names, which is the argument for the symbol over a plain colour swatch — and it keeps the
  legend's swatches meaning *states* rather than *owners*.
- **The line and the legend read as two different kinds of thing**, which is what makes stacking them
  work: coloured names and percentages above, grey state swatches below.
- **In `a-quiet.png` nothing marks the Arkwrights' absence.** A player who does not already know there are
  four Factions cannot tell whether the fourth is missing or does not exist. That is the cost of the quiet
  version, and it is the whole of the choice between these two pictures.
