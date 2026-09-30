# Dying Earth — version 0.09.5, the fog version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/423), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-29-version-0.09.5/`](../dev-diary/2026-09-29-version-0.09.5/).

## 1. Each Victory Tech at the end of a line

The authority is [ticket #424](https://github.com/whaleyjoshua2/Dying-Earth/issues/424).

**Before**, the Custodians' Planetary Stewardship and the Archivists' The Upload were both filed under
Society, and The Upload hung off Closed-Loop Colonies in Off-world Living. **After**, each Faction's
Victory Tech ends a line of its own:

| Line (top to bottom) | Victory Tech | Its chain |
|---|---|---|
| Society | The Upload (Archivists) | Large Language Models 18 -> Civil Defense 32 -> The Upload 48 = **98** |
| **Stewardship** (was Industry) | Planetary Stewardship (Custodians) | Efficient Grids 18 -> Clean Power 32 and Green Consensus 32 -> Planetary Stewardship 48 = **130** |
| Off-world Living | Generation Ships (Arkwrights) | unchanged here |
| Extraction | The Extraction Charter (Prospectors) | unchanged here |
| Propulsion | none | |

- **The Industry line is renamed Stewardship** and drawn **second**, directly above Off-world Living:
  Society, Stewardship, Off-world Living, Extraction, Propulsion. It holds Efficient Grids and
  Coastal Engineering (rung 1), Clean Power and Green Consensus (rung 2), Clean Manufacturing and
  Planetary Stewardship (rung 3), each pair stacked in its cell, **Planetary Stewardship in the lower
  box**.
- **Green Consensus moves to Stewardship and needs Efficient Grids** (was Public Science).
- **Planetary Stewardship moves to Stewardship and needs Efficient Grids, Green Consensus and Clean
  Power** (was Green Consensus alone), in the designer's list's words.
- **The Upload needs Civil Defense** (was Closed-Loop Colonies). Civil Defense needs Large Language
  Models, so the chain is the ticket's "Public Science and Civil Defense". The Upload names only Civil
  Defense because a second line from Large Language Models would run straight behind the Civil
  Defense box and read as passing through it, the fault ticket #245 took out of this same row. The
  Archive order still waits on The Upload; the Archive still stands at a Colony off Earth.
- **Public Science is renamed Large Language Models.** Its effect is unchanged. Its id stays
  `public_science`, so a saved game loads with the Tech it had.
- **Society's rung 2 is Civil Defense alone.** The designer judges it on the shot.
- **The computer seats follow**: each pick list opens with its Faction's new Victory gate chain,
  cheapest first, then the Propulsion chain, then nothing. The Custodians' list opens Efficient
  Grids, Clean Power, Green Consensus; the Archivists' opens Large Language Models, Civil Defense.

No Tech's cost or effect moves, and the whole tree still costs what it did. What moves is the price
of each Faction's road: **the Custodians' rises from 98 to 130**, the dearest in the tree, and **the
Archivists' falls from 148 to 98** (Expanded Habitats, Efficient Grids, Clean Power and Closed-Loop
Colonies are no longer on it). The version's closing sweep reports the win column.

**What would show this wrong**: a Tech reaching Planetary Stewardship through Large Language Models; a
Tech outside Society on The Upload's chain; a pick list that does not open with its gate chain; a
saved game losing its Public Science; the Stewardship row drawn anywhere but second.
