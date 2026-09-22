# The Prospectors' Fund figure out of the top bar's Materials group

Ticket [#308](https://github.com/whaleyjoshua2/Dying-Earth/issues/308) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`prospectors-top-bar.png`](prospectors-top-bar.png) | `shot: player:prospectors venture:900 panel:0`. The top bar as the Prospectors: after **Ducats 12 (+12)** a small bar in the Prospectors' orange, filled to 900 of 2500, with no figure of its own; the Materials group carries Materials alone. |
| [`fund-bar-hover.png`](fund-bar-hover.png) | `shot: player:prospectors venture:900 panel:0 tip:Venture Capital Fund:`. The bar's hover: *Venture Capital Fund: 900 of 2500 Ducats, banking 50% of Ducat income. Set it on the Victory panel.* |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

*"q1 move besides duckets q2 inverse bar only details on hover q3 yes"*, and on which bar,
*"progress bar as recommended"*: the Fund leaves the Materials group for the Ducats group, becomes a
progress bar with no figures of its own, and carries the balance, the 2500 and the share on its
hover; no separator of its own.

## Settled by the builder, to be corrected if wrong

- **The bar is 80 pixels wide and 14 tall**, drawn as the Research race bar is (a grey trough, the
  fill in the seat's colour), so the two bars on the row read as one family. The Research race bar
  is 150 wide; the Fund's is narrower because it carries one seat's figure, not four.
- **The hover goes through `rule_tip`**, so the `tip:` aid can photograph it.
- At 1280 pixels the Prospectors' figures row runs to the window's edge, as it did with the figure
  it replaces, which was about the same width.

## Looked at, not tested

The two pictures above, each opened and read before this was committed. A first pair was taken from
a binary that had not rebuilt (a float compared to an integer stopped the build) and was thrown
away before being looked at.
