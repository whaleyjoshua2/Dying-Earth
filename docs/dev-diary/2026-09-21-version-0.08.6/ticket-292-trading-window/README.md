# The Trading window opens below the top bar

Ticket [#292](https://github.com/whaleyjoshua2/Dying-Earth/issues/292) on
[map #289](https://github.com/whaleyjoshua2/Dying-Earth/issues/289).

| picture | what it shows |
|---|---|
| [`trading-under-the-bar.png`](trading-under-the-bar.png) | `shot: trade:1 panel:0`, 1280x800. The Trading window at its new home, under the bar's two rows and to the right of where the Faction window opens. It used to open sixteen pixels from the corner, over the figures. |
| [`victory-under-the-bar.png`](victory-under-the-bar.png) | `shot: victory:1 panel:0`. The Victory window at the same home. |
| [`trading-beside-factions.png`](trading-beside-factions.png) | `shot: trade:1 factions:1 panel:0`. Both open: the Faction window at the left, Trading to its right, neither over the other. The Faction window stands at the very top here because it is taller than an 800-pixel screen and egui pushes it up; on a taller window it sits under the bar, as before this ticket. |
| [`trading-with-pick-a-tech.png`](trading-with-pick-a-tech.png) | `shot: trade:1 pick:0 panel:0`. The bar carrying its **Pick a Tech** button, the window still clear of it. |
| [`tech-tree-800.png`](tech-tree-800.png) | `shot: tech:1 panel:0`. The Tech Tree under the measured bar on an 800-pixel window, its tree scrolling and its foot forty pixels above the screen's. With the old bound (top plus forty) and the guess retired, the window overshot the foot and egui shoved it up over the bar; the bound is now measured at the line the tree starts on. |
| [`tech-tree-1400.png`](tech-tree-1400.png) | `shot: tech:1 panel:0 window:1280x1400`. The same on a tall window: the whole tree, no scroll. |
| [`climate-panel-under-the-bar.png`](climate-panel-under-the-bar.png) | `shot:` with no aid. The Climate Panel at its home under the measured bar, where 104 stood. |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

- *"q1 that"* -- **under the bar and to the right of the Faction window's home**, so a player with
  both open sees both.
- *"q2 yes"* -- the **Victory window** too, in the same spot.
- *"yes that"* -- **the bar's height is measured** once a frame and every window that avoids the bar
  reads it; the three literals go.

## Settled by the builder, to be corrected if wrong

- The bar's foot is read from the panel it is drawn in and kept on the view; a window opens sixteen
  pixels under it, the gap egui itself leaves from an edge. Trading and Victory open at 560 across,
  past the Faction window's 16 plus its 524 width.
- The **Tech Tree's scroll bound** is measured inside the window at the line the tree starts on,
  rather than summed from the window's top; ticket #242's note that the cursor was tried and
  failed was written against the guessed top, and with the measured one the cursor reads right on
  the first frame. The pictures at 800 and 1400 are the check.
- A **top bar** glossary entry, which four entries used without defining.

## Looked at, not tested

An interface change; `346 passed`, `6 passed`, clippy clean with `-D warnings`. Seven pictures, each
looked at before filing; the first Tech Tree capture at 800 showed the window over the bar, which
is why its bound moved.
