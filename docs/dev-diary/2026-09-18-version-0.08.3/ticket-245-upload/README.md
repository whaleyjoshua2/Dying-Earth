# Ticket #245: the line that was not an edge

The designer asked for a line removed from the Tech Tree — the one that appeared to run from
**Expanded Habitats** to **Generation Ships** — reasoning that it was redundant, since the path
already goes through Closed-Loop Colonies.

**There was no such edge.** Generation Ships needs only Closed-Loop Colonies. What was drawn was
**Expanded Habitats → The Upload**: it left Expanded Habitats, ran right, passed *underneath* the
Generation Ships box — lines are painted before boxes, so the box hid the middle of it — and turned
down to The Upload.

That made the redundancy argument false: Expanded Habitats was not implied by anything else on The
Upload's path, so removing the edge would lower the bar on the **Archivists' Victory gate** rather
than tidy a duplicate. Offered a drawing fix instead, the designer took the rules change with the
consequence stated plainly: *"remove expanded habitats from the upload I accept the weakend gate"*.

| picture | what it shows |
|---|---|
| [`tree-without-the-line.png`](tree-without-the-line.png) | The tree with the edge gone. Expanded Habitats now reaches only Closed-Loop Colonies and Relay Networks; nothing runs past Generation Ships; The Upload hangs off Public Science alone. |

## The sum of three tickets, which none of them intended

Version 0.08.3 moved three of the four Victory gates, in three separate tickets, and nothing was
watching the result. Counted as Techs that must stand before a gate is reachable:

| Faction | gate | depth | wins of 80 at the 0.08.2 baseline |
|---|---|---|---|
| Archivists | The Upload | **1** | 7 |
| Custodians | Planetary Stewardship | **2** | **37** |
| Arkwrights | Generation Ships | **4** | **3** |
| Prospectors | The Extraction Charter | **4** | 10 |

**The two thinnest seats now have the deepest gates, and the runaway has one of the shallowest.**
No ticket set out to do that — #232 added Beneficiation to the Prospectors' path, #242 freed the
Custodians' from Industry, and this one freed the Archivists' — but it is where they add up, and
this version is supposed to be about closing that gap rather than widening it.

A test now pins all four depths, so the next change cannot move one silently. The sweep reports the
win column against the baseline above.

## A correction worth keeping

The Arkwrights' depth was first written into that test as **3** and the test caught it: Closed-Loop
Colonies itself needs Expanded Habitats *and* Clean Power, and Clean Power needs Efficient Grids,
so the true figure is **4**. A transitive count is easy to eyeball wrong, which is the argument for
computing it in the test rather than asserting a remembered number.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 turns:14 window:1920x1080`,
off-screen, exit 0.
