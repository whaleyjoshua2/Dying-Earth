# Ticket #249: the band order settled

Four turns of the designer looking at the tree and moving bands:

- [#247](https://github.com/whaleyjoshua2/Dying-Earth/issues/247) put Extraction under Industry and
  The Upload above Generation Ships.
- [#248](https://github.com/whaleyjoshua2/Dying-Earth/issues/248) lifted Off-world Living to the top,
  which cost the second of those and was accepted.
- This one is the order asked for last: *"keep off world living first than I want industry followed
  extraction and then propulsion and finally society"*.

```
Off-world Living, Industry, Extraction, Propulsion, Society
```

## The price, counted

Society is at the **bottom** and Off-world Living at the **top** — and those are precisely the two
bands the **Closed-Loop Colonies → The Upload** edge runs between, since [#246](https://github.com/whaleyjoshua2/Dying-Earth/issues/246)
made that a prerequisite.

So the edge now spans the entire height of the tree, and because lines are painted before boxes it
passes **behind four boxes** on the way down, in the column it shares with them:

| box it passes behind |
|---|
| Generation Ships |
| Clean Manufacturing |
| The Extraction Charter |
| Hardened Hulls |

That is worse than the two boxes #246 shipped with, and it is the third time this defect has been
looked at in one version. **Reading order won over the line, knowingly.**

| picture | what it shows |
|---|---|
| [`final-order.png`](final-order.png) | The order as asked, and the edge running the full height behind four boxes. |

## The fix this keeps arguing for

It is a **drawing** problem with a drawing answer. `tech_tree`'s elbow routing (ticket #133) sends
an edge down the gap to the left of the needing box's column, which stops it crossing boxes
*sideways* but does nothing about crossing them *vertically*. Routing around, offsetting, or
dimming a passing edge would end it for every layout, not just this one.

It has been offered on #243, on #246 and again here, and declined or deferred each time. It stays
on the map as fog. Nothing in the rules depends on it.

## No test guards this

Presentation with no mechanic behind it, and `src/ui.rs` has no test harness, so every test stayed
green and witnessed nothing. The picture is the verification.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 turns:14 window:1920x1080`,
off-screen, exit 0.
