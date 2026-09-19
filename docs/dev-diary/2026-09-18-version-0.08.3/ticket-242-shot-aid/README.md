# The Tech Tree, found not fitting and made to fit

These pictures belong to two tickets that ran together: [A shot aid that opens a window and parks
a pointer](https://github.com/whaleyjoshua2/Dying-Earth/issues/242), and the regression its first
capture exposed, [The Tech Tree scrolls, and the tree is a tenth
smaller](https://github.com/whaleyjoshua2/Dying-Earth/issues/243).

## The correction that started it

The shot aid was charted on the claim that **no `shot:` flag opens a panel**. That claim was wrong.
`tech:1`, `factions:1`, `trade:1`, `victory:1` and `climate:toggle` have always opened their
windows, and `window:<w>x<h>` has always set the capture size; they are parsed by equality rather
than by `strip_prefix`, and the survey that missed them only searched for the latter.

The cost of the error was real: **tickets #231 and #232 were both told the Tech Tree could not be
photographed**, and both verified its layout by reading `box_of` in `src/ui.rs` instead of looking
at it. The first picture actually taken found a defect that reading had missed.

So the shot aid shrank to its one genuinely missing half — parking a pointer so a tooltip renders —
and this entry is about what the pictures showed.

## What the pictures showed

A branch's band in the tree is as tall as its fullest rung-2 cell. Ticket #232 gave **Off-world
Living** and **Extraction** a second rung-2 Tech each, taking the tree from **seven rows to nine —
672 pixels to 864**.

| picture | what it shows |
|---|---|
| [`before-overflow-800.png`](before-overflow-800.png) | 1280x800, before the fix. **The whole Society branch is off the bottom of the screen** — Public Science, Green Consensus, Civil Defense and two Victory gates — with no scrollbar and the window fixed at `resizable(false)`. |
| [`before-fits-1080.png`](before-fits-1080.png) | 1920x1080, before the fix. All five branches fit, **with about thirty pixels to spare**. This is the size the game opens maximised into, which is why the defect was invisible to anyone playing on a large screen. |
| [`after-scroll-800.png`](after-scroll-800.png) | 1280x800, with the tree scrolling inside a screen-bounded window: four branches and the head of the fifth, the rest a scroll away. |
| [`after-shrink-1080.png`](after-shrink-1080.png) | 1920x1080, every layout figure and box font a tenth smaller. All five branches with real headroom below Civil Defense, and the two longest names -- **Closed-Loop Colonies** and **The Extraction Charter** -- still inside their boxes, which is what had to be checked since box text is not clipped. |
| [`after-shrink-800.png`](after-shrink-800.png) | 1280x800 at nine tenths: four branches and the head of Society, against two branches before the scroll bound was fixed. |
| [`after-edges-1080.png`](after-edges-1080.png) | The tree with the `Research share` subheading over the bar, **Green Consensus hanging off Public Science alone**, and **Beneficiation feeding The Extraction Charter** (drawn `locked`, since Beneficiation is not yet done in that game). |

## What was done

**It scrolls**, bounded by the screen, the scrollbar showing only when needed — the same answer
ticket #217 gave for the Faction selection screen, so the game solves this problem the same way
twice.

**And the whole tree is a tenth smaller**: `COL` 160 → 144, `ROW` 96 → 86, `BOX_W` 136 → 122,
`BOX_H` 64 → 58, box fonts 13 → 12 and 11 → 10, text offsets and Pick button to match. The spacing
came down with the boxes on purpose: the tree's height is `rows x ROW`, so shrinking the boxes alone
would have saved nothing and merely put air around smaller boxes. At nine tenths the tree is **774
pixels**.

Box text is centred and **not clipped**, so a name too long for its box spills over the edge rather
than being cut. That is why the fonts came down with the boxes, and why **Closed-Loop Colonies** and
**The Extraction Charter** — the two longest — were the names to check in the captures. Both sit
inside their boxes.

## Two wrong turns, recorded because they are easy to repeat

- A `max_height` computed from **`ui.cursor().top()`** left the window **535 pixels tall on an
  800-pixel screen**, showing two branches where four had fitted — worse than the bug being fixed.
  The cursor does not return what it appears to. The bound is computed from `ctx.content_rect()`
  and the window's own top instead, and that was settled by **printing the rect** rather than
  guessing a fourth time. One debug run would have been cheaper than three attempts.
- A **`max_height` on the Window does nothing**: a window sizes itself to its content, and
  `max_height` is a cap rather than a floor. The bound belongs on the `ScrollArea`, with
  `min_scrolled_height` so it fills the room it is given.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 turns:14 [window:1920x1080]`,
off-screen, exit 0. Nothing was opened on the designer's desktop.
