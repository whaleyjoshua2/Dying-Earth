# Ticket #246: The Upload waits on Closed-Loop Colonies

Superseding part of [ticket #245](https://github.com/whaleyjoshua2/Dying-Earth/issues/245), taken
minutes after it. The designer: *"Make the upload require closed loop colonies drop public science
all together"*.

Thematically it is the better parent. The Archive stands **at a Colony off Earth**, so the Tech
that makes an off-world Colony liveable is what should open its door; Public Science only ever
meant "you have done some research".

## The price

The Archivists' gate went **2 → 1 → 4 in one version**, and four is the deepest tier in the tree.
It is also not only a Victory gate: `orders.rs:580` refuses the **Archive order** until The Upload
stands, so the whole Archive chain — order, three turns, 80 Research, twelve Colonists uploaded —
moves back with it. The Archive already completed in just **28 of 80 games** at the 0.08.2
baseline.

| Faction | gate | depth | wins of 80 at baseline |
|---|---|---|---|
| Custodians | Planetary Stewardship | **2** | **37** |
| Archivists | The Upload | **4** | 7 |
| Arkwrights | Generation Ships | **4** | **3** |
| Prospectors | The Extraction Charter | **4** | 10 |

The runaway seat now has the only shallow gate.

## What the picture shows, and it is not only the edge

| picture | what it shows |
|---|---|
| [`tree-with-the-new-edge.png`](tree-with-the-new-edge.png) | The Upload hanging off Closed-Loop Colonies, Public Science feeding only Green Consensus and Civil Defense. |

The new edge runs as a long vertical from Closed-Loop Colonies down to The Upload, and **it passes
underneath the Generation Ships and The Extraction Charter boxes**, because lines are painted
before boxes. That is precisely the defect ticket #245 was raised to remove: an edge that reads as
though it terminates at a box it merely passes under.

**So this change deletes one such line and draws a longer one.** The drawing fix offered on #243
and declined there — route around, offset, or dim a passing edge — is now the more attractive
option, and it remains open. It touches `tech_tree`'s elbow routing in `src/ui.rs` and no rule.

Captured with `target/release/dying-earth.exe shot:<prefix> tech:1 turns:14 window:1920x1080`,
off-screen, exit 0.
