# The Prospectors' Fund figure out of the top bar's Materials group

Ticket [#308](https://github.com/whaleyjoshua2/Dying-Earth/issues/308) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`prospectors-top-bar-cut.png`](prospectors-top-bar-cut.png) | `shot: player:prospectors venture:900 panel:0`. The top bar as the Prospectors with the Fund gone from it: Materials, Fuel, Energy, Ducats and Influence in their groups and nothing of the Fund's, which is 900 of 2500 in this picture and is read on the Victory window. |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

First, on the ticket: *"q1 move besides duckets q2 inverse bar only details on hover q3 yes"* and
*"progress bar as recommended"*. That was built as a small progress bar beside Ducats in the
Prospectors' colour, its figures on the hover, and shown to the designer in two pictures. Then, on
seeing it: *"yeah let's just cut it, the archivist bank is not shown on the bar"*. **The Fund is not
on the top bar at all**, as no other Faction's fund is; its figures are the Victory window's (the
progress line, the slider's heading, the Fund line) and the Faction window's chart.

## Settled by the builder, to be corrected if wrong

- Nothing of the bar remains in the code; the comment at its old place records the two moves.
- The **Top bar** and **Venture Capital Fund** glossary entries say the Fund is not on the bar and
  why.

## Looked at, not tested

The picture above, opened and read before this was committed. The two pictures of the progress
bar it replaced were looked at, shown to the designer, and removed with it.
