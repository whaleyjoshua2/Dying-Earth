# Ticket #247: the Tech Tree's bands reordered

The designer, with the tree open: *"can you move the upload box to above generation ships and move
the entier extraction tree to below industry"*.

Both were had by **reordering the bands alone** — no rule moved, no Tech changed branch. Until now
the band order simply fell out of the order Techs happen to sit in `TechId::ALL`: Industry,
Propulsion, Off-world Living, Extraction, Society. It is now an explicit constant:

```
Industry, Extraction, Propulsion, Society, Off-world Living
```

- **Extraction sits directly below Industry**, as asked.
- **Society sits directly above Off-world Living**, which puts The Upload (Society, rung 3) in the
  band immediately above Generation Ships (Off-world Living, rung 3).

A branch not named in the constant keeps its first-appearance place, after the named ones, so a
branch added later cannot vanish by being forgotten.

## The bonus, and it is the better half

Ticket #246 pointed The Upload at Closed-Loop Colonies, and the edge that drew ran the **whole
height of the tree**, passing underneath Generation Ships and The Extraction Charter — the very
defect ticket #245 had been raised to remove.

Putting Society directly above Off-world Living makes that edge **short**: Closed-Loop Colonies is
now one band below The Upload, so the line is a hop between neighbours and **passes under no box at
all**. The problem #246 created is gone, and by layout rather than by changing a prerequisite back.

| picture | what it shows |
|---|---|
| [`after-reorder.png`](after-reorder.png) | Industry, Extraction, Propulsion, Society, Off-world Living, top to bottom. The Upload above Generation Ships, and the Closed-Loop Colonies edge reduced to a short hop. |

## One thing not exactly as asked

**The Upload is above Generation Ships, but half a column to the right of it, not squarely over
it.** Society's rung 3 holds *two* boxes — Planetary Stewardship and The Upload — so they share
the rung's width, while Off-world Living's rung 3 holds Generation Ships alone and centres it.
Aligning them would mean either nudging a lone rung-3 box to a slot index rather than centring it,
or fixing the rung's columns so every band divides it the same way. Left as it is pending the
designer's word, since "above" may well be enough.

## No test guards this

It is a presentation change with no mechanic behind it, and `src/ui.rs` has no test harness, so
**every test stayed green throughout and witnessed nothing** — the same shape of gap recorded on
ticket #244. The picture is the verification here, which for a layout change is the right one.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 turns:14 window:1920x1080`,
off-screen, exit 0.
