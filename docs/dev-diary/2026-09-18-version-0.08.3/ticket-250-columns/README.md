# Ticket #250: one column for every rung-3 Tech, and the bands settled

Three asks in one line, and read together they fit:

> now swap industry and extraction and move society to the top - there is no reason upload needs to
> be on the same line as society society and I want all level three techs to appear on the same
> column

## The bands

```
Society, Off-world Living, Extraction, Industry, Propulsion
```

## Every rung stacks now, the last one included

The rule was `stacked = |r| r + 1 < rungs`, so the **final** rung laid its boxes side by side. A
band with two rung-3 Techs therefore split the column between them, which is why Society's
**Planetary Stewardship** and **The Upload** sat at two x positions no other band's rung-3 box
shared.

Making every rung stack gives both halves of the ask at once:

- **The Upload takes a row of its own** inside the Society band, no longer sharing a line with
  Planetary Stewardship.
- **Every rung-3 box in the tree sits at `left[2] + COL / 2`** — one column, six Techs:
  Planetary Stewardship, The Upload, Generation Ships, The Extraction Charter, Clean Manufacturing,
  Hardened Hulls.

It costs nothing in height — a band is already as tall as its fullest stacked cell and Society's
rung 2 already held two — and it makes the tree **one COL narrower**, because the last rung no
longer claims width for the widest cell in it.

## The edge problem is gone, and this order is what ended it

Ticket #249 counted the Closed-Loop Colonies → The Upload edge running the whole height of the tree
and passing behind **four** boxes: Generation Ships, Clean Manufacturing, The Extraction Charter,
Hardened Hulls. The fix was offered three times and declined three times.

It did not need taking. **Society and Off-world Living are adjacent in this order**, and those are
the two bands that edge runs between, so it is a hop between neighbours that routes up the empty
gutter between the rung-2 and rung-3 columns and **crosses nothing at all**.

The routing fix stays on the map as fog, now genuinely optional rather than pressing: no edge in
the shipped layout passes behind a box.

| picture | what it shows |
|---|---|
| [`one-column.png`](one-column.png) | The five bands in order; all six rung-3 Techs in a single column; The Upload stacked under Planetary Stewardship on its own row; the Closed-Loop Colonies edge a short clean hop; the tree a column narrower. |

## No test guards this

Presentation with no mechanic behind it, and `src/ui.rs` has no test harness, so every test stayed
green and witnessed nothing. The picture is the verification — which for a layout change is the
right one, and it is the fifth time in this version that looking at the tree has found or settled
something a diff could not show.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 turns:14 window:1920x1080`,
off-screen, exit 0.
