# At least two duplicated texts found and cut

Ticket [#306](https://github.com/whaleyjoshua2/Dying-Earth/issues/306) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304). Thirteen places where the game
said a thing twice were listed on the ticket; the designer cut eight of them. One picture per
surface, each showing the place where the words used to be.

| picture | what it shows |
|---|---|
| [`cluster-nothing-selected.png`](cluster-nothing-selected.png) | `shot: panel:0`. The command cluster with nothing selected: a greyed **Spend 5** where *Click a Region or a Colony to spend on it* stood; the Max button's hover keeps the sentence. The strip keeps its shape. |
| [`climate-panel.png`](climate-panel.png) | `shot: climate:toggle window:1280x1300`. The Climate Panel's Blame block ending at the Archivists' line; the sentence *A share above a fair quarter raises...*, a clause of the heading's hover, is gone from beneath it. |
| [`iss-shipyard-building.png`](iss-shipyard-building.png) | `shot: hab:1 morder:shipyard commit:1 panel:0`. The ISS's card the turn after a Shipyard was ordered: the hatched tile reads *building, 1 turn* and no text line beneath the grid repeats it. |
| [`china-orders-heading.png`](china-orders-heading.png) | `shot: select:eastasia panel:0 window:1280x1100`. China's card with the bottom heading reading **Orders**, no longer *Orders (hover a button for what it does)*. |
| [`victory-window.png`](victory-window.png) | `shot: victory:1`. The Victory window: each Faction's heading followed at once by its two labelled bars; the Victory Condition sentence that restated them is gone. |
| [`faction-window.png`](faction-window.png) | `shot: factions:1 panel:0`. The Faction window: the history chart directly under the heading, where the Victory progress lines and bars stood; *Income last turn* without the *Hover a figure...* line beneath its figures. |
| [`tech-tree.png`](tech-tree.png) | `shot: tech:1`. The Tech Tree's legend row without *Hover a box for its effect.* after it. |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

*"q1 as recommended plus the hover a button.. sign posts and 7 & 8 q2 keep q3 take those per Q1 q4
cut"*. Cut: the cluster's sentence (1), the Climate Panel's sentence (2), the Colony card's queued
Module lines (3), the offline suffix on a building's row (4), the Agitate hover's last sentence (5),
the Victory window's Victory Condition sentence (7), the Faction window's Victory progress block (8)
and the hover signposts (13). Kept: the Smear and Greenwash captions (6), the Influence figure on
the bar and in the cluster (9), last turn's income on the bar and the Faction window (10), the
Facilities heading's count (11), End Turn's hover (12).

## Settled by the builder, to be corrected if wrong

- **A greyed Spend button stands where the cluster's sentence was**, so the strip keeps its shape
  with nothing selected (ticket #294's rule that the rail is drawn either way).
- **Item 8 is read as the Faction window's copy going**, the Victory window being the victory's
  home; the Faction window keeps its history chart. The designer named 8 under Q1 to cut and
  answered Q2 *keep*, so Q2's keep is read as 9 and 10.
- **Item 7 leaves the Rulebook's copy** of the Victory Condition sentence, which is the Rulebook's
  own section and not a restatement of bars beside it.
- **Item 13 leaves the rival's-page line** *A rival's income is shown as totals only*, which states
  a rule rather than pointing at a hover.
- **Item 4 strips the suffix from the row and keeps it on the box's hover**, with the words in one
  constant, so the offline ticket on this map has one place to build on.
- The **Orders** heading's spec line in `docs/spec/version-0.07.4.md` is superseded by this
  ticket; the amendments document will say so.

## Looked at, not tested

The seven pictures above, each opened and read before this was committed. The first Faction
window picture was the Climate Panel over the window, and the first ISS picture a Habitat that had
finished in one turn; both were retaken.
