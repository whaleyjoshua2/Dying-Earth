# Ticket #248: Off-world Living to the top

*"lets move off world living to the top"*, one turn after [the bands were first
reordered](https://github.com/whaleyjoshua2/Dying-Earth/issues/247). The order is now:

```
Off-world Living, Society, Industry, Extraction, Propulsion
```

## The trade, made explicit

**Nothing can sit above the top band.** With Off-world Living there, Society cannot be above it,
so The Upload cannot be above Generation Ships — the two swap, and The Upload sits directly below.
Put to the designer as the cost of the move, and accepted: *"upload can be below generation ships
now"*.

What was preserved is the adjacency: Society stays immediately beside Off-world Living, so the
Closed-Loop Colonies → The Upload edge stays a hop between neighbours rather than running the
height of the tree as ticket #246 left it. Extraction stays directly below Industry.

| picture | what it shows |
|---|---|
| [`after-off-world-top.png`](after-off-world-top.png) | Off-world Living, Society, Industry, Extraction, Propulsion, top to bottom. The Upload below Generation Ships. |

## A claim from ticket #247 that no longer holds

That ticket recorded the edge as passing **under no box at all**, which was true of *that* layout:
Society sat above Off-world Living, so the edge ran upward from Closed-Loop Colonies and cleared
the top of the Generation Ships box.

**Inverted, it does not clear it.** Closed-Loop Colonies is now above Generation Ships' row and The
Upload below it, so the edge descends straight past — and behind — that box. The crossing is back,
on one edge rather than the two of #246.

So the routing fix in the map's fog — route a passing edge around the boxes it crosses, or offset
it, so it stops reading as an arrival — is live again, and this is now the only edge that wants it.

## No test guards this

Presentation with no mechanic behind it, and `src/ui.rs` has no test harness, so every test stayed
green and witnessed nothing. The picture is the verification.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 turns:14 window:1920x1080`,
off-screen, exit 0.
