# Faction window mockups — ticket #203

Four mockups made while grilling [A Faction window: a selector, a glyph worn large, Relations moved off the
Victory window, and income beside them](https://github.com/whaleyjoshua2/Dying-Earth/issues/203), on map
[#202](https://github.com/whaleyjoshua2/Dying-Earth/issues/202).

**These are drawings, not the game.** Nothing here is built. They are HTML rendered headlessly in Edge and
styled to match the game's own dark panels, so that a layout can be argued about by looking rather than by
describing. The faction symbols are the real `assets/icons/*.svg` files at their real colours; the figures
are a plausible turn 18, invented.

The designer's decisions they were made against, from the grilling round:

- The window holds **both halves**: the Faction card brought in-game (the rulebook) *and* the live figures.
- A rival shows **income totals only**; the building-by-building breakdown stays yours alone.
- Relations shows the selected Faction's **two rows** — what they think of the others, and what the others
  think of them — and the 4x4 grid leaves the Victory window entirely.
- The Victory window **keeps the climate projection and the four-way progress bars** and gives up **Blame
  and Relations**.
- The selector is a **dropdown inside the window, top right**, defaulting to your own Faction.

## The files

| file | what it shows |
|---|---|
| `a-rulebook-first.png` | Layout **A**: the rulebook at the top, the live figures under it. Your own Faction (Custodians) selected. |
| `b-live-first.png` | Layout **B**: the same content with the live figures first and the rulebook below. |
| `c-rival.png` | Layout A with a **rival** selected (Archivists), showing the disclosure line: income totals with the breakdown withheld. |
| `d-glyph-sizes.png` | The header at **48, 64 and 96 pixels**, to pick a size by looking. The Faction card on the setup screen draws it at 28. |

`make-mockups.py` regenerates the HTML; render it with headless Edge (see the `eyes-on` skill for the exact
invocation on this host). Nothing was opened on the designer's desktop.

## What is visible in them, which the descriptions did not say

- **Layout B buries the Faction.** With the live figures first, the glyph and the name land in the middle of
  the window, and the page opens with nothing but a small dropdown saying whose it is. Against a line that
  asks for the glyph "prominently", that reads as an argument for layout A.
- **The window is tall** — about 950 pixels of content at 524 wide, with the signature rules being the long
  part. On a 1080-line screen that very nearly fills the height.
- **The Archivists' page is the long one**: theirs is the only Faction carrying a fifth multiplier (Research
  off Earth x1.75) and the longest Victory Condition.
