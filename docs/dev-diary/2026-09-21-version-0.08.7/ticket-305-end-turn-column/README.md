# End Turn in its own column at the right of the command cluster, at the bottom

Ticket [#305](https://github.com/whaleyjoshua2/Dying-Earth/issues/305) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`cluster-china-selected.png`](cluster-china-selected.png) | `shot: select:eastasia panel:0`, 1280x800. The cluster in two columns with China selected: on the left the Allotment line, the rail, **Spend 5 on China**, then **Max** and **every turn**; on the right the **sun** alone, its word *End Turn* level with the Max row and nothing above it. |
| [`cluster-nothing-selected.png`](cluster-nothing-selected.png) | `shot: panel:0`, 1280x800, at the side panel's default width. Nothing selected: *Click a Region or a Colony to spend on it* wrapped onto two lines inside the left column, Max and the tick greyed, the sun whole at the right edge. The first cut of this picture had the sentence overflow the column and push the sun half off the panel; the column now wraps its text. |
| [`cluster-pick-owed-embers.png`](cluster-pick-owed-embers.png) | `shot: select:eastasia pick:0 panel:0`. A Tech pick owed: the sun dimmed to embers and its word greyed in the same place, the bar carrying **Pick a Tech**. |

No batch was run: an interface change; the engine is untouched.

## What was decided, in the designer's words

*"q1 you got it q2 the suns own width q3 empty q4 unchanged"*: two columns, the sun's column at
the sun's own width, nothing above the sun, and End Turn's key, hover, embers and the spectator's
top-bar rectangle as 0.08.6 left them.

## Settled by the builder, to be corrected if wrong

- **The left column's width is fixed before the rail is drawn**, since the rail takes all the
  width it is given; the right column is the sun's allocation (diameter plus its margin) and
  nothing more.
- **The sun is pushed to the bottom by measured space**, the left column's height less the sun's,
  rather than by a bottom-up layout, because the cluster's panel sizes itself from its content and
  a bottom-up layout in it has no bottom to sit on (the trap ticket #294's comment recorded).
- **Text wraps inside the left column.** Without it the *Click a Region or a Colony* line at the
  cluster's scale was wider than the column at the panel's default width.

## Looked at, not tested

The three pictures above, each opened and read before this was committed.
