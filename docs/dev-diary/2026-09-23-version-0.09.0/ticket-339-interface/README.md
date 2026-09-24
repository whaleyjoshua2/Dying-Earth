# Tech Tree edges routed around the boxes, and the four adjustments

Ticket [#339](https://github.com/whaleyjoshua2/Dying-Earth/issues/339) on version 0.09.0, the
interface half: **improvement N** and **all four adjustments**. The designer's resolution comment on
the ticket is the authority. Improvements A, B, G and L are another lane's work and are not here.

Every picture below is cropped out of a headless `shot:` capture; the command that produced it is
named beside it. A `-was` picture is that same command run against the code as it stood at `ad6f553`
(prefix `shot:was`), rebuilt for the purpose and put back. Nothing was opened on the designer's desktop, and the extra view captures each run
writes were deleted.

## The pictures

| picture | what it shows |
|---|---|
| [`header-was.png`](header-was.png) / [`header-now.png`](header-now.png) | `shot:now settler:mars site:mars,0 seed:7 turns:8 window:1500x1000`, cropped to the middle of the top bar. **Both header adjustments in one strip.** Influence went from `20 of 20` to `20 / 20`; the temperature went from `+1.7 C, heading to +1.8` to `+1.7 C`. The bar got about a hundred pixels back, which is why net Emissions have moved left into the crop. |
| [`temperature-hover.png`](temperature-hover.png) | `shot:temp tip:Temperature seed:7 turns:12 window:1500x1000`. **Where the dropped figure went.** The temperature's own hover now says it in words -- *"The CO2 Stock already in the air is heading to +2.1 C on its own, with nothing else emitted"* -- above the chart that has always drawn it. |
| [`endturn-was.png`](endturn-was.png) / [`endturn-now.png`](endturn-now.png) | the same run, cropped to the foot of the command cluster. **End Turn's left padding.** The Influence rail's right end pulls back by 13 pixels and the sun stays exactly where it was, so the gap between the two goes from about four pixels to about seventeen. The cluster's own `of 20 left` is untouched, the designer having named the header. |
| [`found-site-was.png`](found-site-was.png) / [`found-site-now.png`](found-site-now.png) | the same run, cropped to the Colony Slot panel. **The founding door with the longest label in the game**, "Found a Colony here with the 8 Colonists aboard TSV Endeavour". Before: the four yields sat under the words, a second storey repeating the `Yields here:` line directly above it. Now they sit beside the words, which wrap to two lines inside the panel. |
| [`found-stack-now.png`](found-stack-now.png) | `shot:stack settler:mars seed:7 turns:8 window:1500x2200`, cropped to the Load and unload block of the Ship stack card. **The other founding door, six of them at once** -- one line each, the site's name and then its four figures. The tall window is what gets the block into a picture at all; the card is longer than a screen. |
| [`tech-tree.png`](tech-tree.png) | `shot:tall tech:1 seed:7 turns:12 window:1500x1400`, cropped to the tree. **The tree as it stands**, all five bands and all twenty Techs, with every edge clear of every box. This picture is **pixel-identical** to the same capture taken before the change, which is the finding, not a stale file: see below. |
| [`control-was.png`](control-was.png) / [`control-now.png`](control-now.png) | `shot:ctrlwas tech:1 seed:7 turns:12 window:1500x1400` and `shot:ctrlnow ...`, cropped to the Extraction band and enlarged three times. **The negative control**: a tree deliberately rearranged so that one edge has a box in its way. Before, the Deep Mining -> The Extraction Charter line goes in at Coastal Engineering's left edge and comes out of its right edge, so it reads as *Coastal Engineering feeds the Charter*, which is false. After, it leaves Deep Mining's top, runs the row gap above Coastal Engineering and drops into the lane. Both were taken with two temporary lines in `assets/data/techs.toml` (Coastal Engineering moved to Extraction rung 2, and Deep Mining added to the Charter's prerequisites), **which were reverted**; the shipped data is unchanged. |

## N. Tech Tree edges routed around the boxes they cross

**The measurement first: with today's twenty Techs, no edge crosses a box.** Every edge was
replayed offline against `box_of`'s own arithmetic -- 144 by 86 grid, 122 by 58 boxes, the `#250`
band order -- and no leg of any of the seventeen edges meets any of the twenty boxes, with six
pixels of padding to spare. The defect issue #246 photographed was real and is gone: it was drawn
when the last rung laid its boxes side by side, and `#250`'s *every rung stacks* moved the rung-3
column back under one x. The tree picture therefore cannot change, and does not.

What changed is that the property is now **held by the code rather than by the data's good luck**.
The old routing had one detour and it fired only for a box whose middle sat within a pixel of the
needed box's middle (`(ob.center().y - from_box.center().y).abs() < 1.0`) and which lay wholly
inside the run -- a test that knows nothing about boxes in general. Adding one Tech to a rung-2 cell
is enough to put a box back in a line's way, which is exactly what `control-was.png` does.

**How it is routed**, in `tech_edge_path`: the elbow is unchanged -- out of the needed box, along
the lane in the gap left of the needing box's column, in at its left edge -- but the horizontal
**run**, the only leg long enough to cross a column, is tried at a series of heights and the first
clear one is taken. The heights are the needed box's own middle (the straight elbow, and what is
drawn today), then the air between box rows, below and above, stepping a whole row at a time out to
three rows either side, nearer side first. A run at one of those leaves the box by its top or bottom
and drops into the lane. With nothing clear it falls back to the straight elbow, because a line
drawn badly can still be read and a line not drawn cannot.

**What it cost**: three rows either side and no more, so this is a detour and not a graph-layout
engine; a detour spends one extra leg and leaves the box by a face rather than a side, which
`control-now.png` shows. Two lines sharing a lane still overlap, as they already did.

**Guarded by a test**, `a_tech_tree_edge_is_routed_around_the_boxes_in_its_way`, built on the tree's
own grid figures. It was **witnessed red**: with the height loop removed so the function always
returns the straight elbow, it fails at *the routed edge clears it*. The test also carries its own
control arm asserting that the straight elbow really does cross the fixture's box.

## The four adjustments

1. **`15 / 15`, not `15 of 15`.** `src/ui.rs`, the top bar's Influence and its bought-Influence
   variant. Every other paired figure on that bar is a slash -- Widgets `16 / 18`, Research
   `39 / 40`, `Turn 9 / 36` -- so Influence was the one figure there written in words. The command
   cluster's `of 20 left` is left alone.
2. **`+1.2 C` alone.** The top bar's temperature drops `, heading to +1.2`. The figure is not lost:
   the hover's first sentence now names it and says what it means, and the hover's chart has always
   drawn it. The Climate Panel's own line keeps both figures.
3. **End Turn gains 12.65 pixels on its left** (`10.0 * CLUSTER_SCALE`, scaled with everything else
   in the cluster as ticket #211 requires). The pad is taken off the left column's width rather than
   added to the strip, so the cluster is the width it always was and only the sun moves.
4. **The yields sit beside the words.** `found_button` lays label and yields in a row instead of a
   column. **The words wrap**, and the first picture is why: laid on one line the longest door came
   to about 610 pixels and widened the side panel by ninety, eating that much of the map, because
   the panel sizes itself to its content. The words are given the room left after the yields
   (`YIELD_ROW_W`, 220, measured off that capture at 216) and wrap inside it; a short door still
   puts the glyphs right beside its last word, because the words claim only the width they use.

## Looked at

The eleven pictures above, every one opened and read before this was written. Three things were
caught by looking:

- **The founding button widened the side panel.** Caught on the first capture of adjustment 4, fixed
  by wrapping the words, and the fixed version is `found-site-now.png`. `docs/HANDOFF.md` records
  this button as built and never looked at by anybody; this is the first time it has been seen.
- **The Tech Tree picture did not change at all.** Decided rather than assumed: the tree-only crop
  of the before and after captures differs by zero pixels, and the offline sweep says why. The
  control pictures exist because a change nothing can show is a change nothing has tested.
- **The founding doors are ragged down their right edge** on the Ship stack card, since each button
  is as wide as its own label and the yields follow the last word. It is what *beside* costs, and it
  is the designer's call whether the figures should line up in a column instead.

A defect the temperature hover picture caught that is **not this ticket's**: the chart's caption
reads *"Temperature by turn; red is a Break"* while its axis is labelled with in-game dates. Left
alone; it belongs to the chart.

Not photographed: the clicks themselves (headless, no pointer), and a founding button on the
Antarctic sea door, which wants a board with the ice open and emigrants waiting.

## Aids

`site:<body id>,<slot>` is now applied **after** `settler:<body id>`, so the two can be given
together: `settler:` is what puts a Colony Ship at the Body, and without it the slot panel `site:`
opens has no founding door on it at all. Before this the stack selection won and the slot panel
could never be photographed with its door.
