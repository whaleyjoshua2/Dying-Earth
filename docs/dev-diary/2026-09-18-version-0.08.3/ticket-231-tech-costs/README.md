# Ticket #231: the Tech Tree gets dearer

Two headless captures taken against `version-0.08.3` with the new costs in place, to look at the
one thing a diff cannot show: whether the changed figures still fit where they are drawn.

A Tech's cost is drawn in exactly three places, none of them the face of a Tech Tree box — the top
bar's `progress / cost` (`src/ui.rs:1941`), the hover on a Tech Tree box (`src/ui.rs:5369`), and
the "Waits on …" line on a Faction card (`src/ui.rs:1496`). **The Tech Tree window itself cannot be
photographed**: `shot:` mode captures the seven map views and the five menus, and nothing opens a
panel. That gap is recorded on the map as fog.

| picture | what it shows |
|---|---|
| [`t12-solar.png`](t12-solar.png) | Turn 13, the top bar reading **`38 / 48`** — a rung-3 Tech at its new cost of 48, with the Research race bar beside it. The figure fits its space and the bar is unaffected, which is the whole of what needed looking at: 30 → 32 and 45 → 48 are both two digits, so nothing could reflow, and nothing has. |
| [`t20-earth.png`](t20-earth.png) | Turn 21, the same element reading **`- (385 waiting)`** — no Tech is chosen and 385 Research is banked with nothing to spend it on. One frame of one seed, and not a measurement: a pick is owed at every completion, so this may be the instant before the Lead chooses rather than an exhausted tree. It is noted because it is the shape of the thing ticket #231 decided *not* to fix — the designer chose a nudge over making Research bite. |

Taken with:

```
target/release/dying-earth.exe shot:<prefix> turns:12
target/release/dying-earth.exe shot:<prefix> turns:20
```

Both off-screen, both exited 0, nothing opened on the designer's desktop. The other fourteen views
each run produced were deleted; these are the two that were looked at.
