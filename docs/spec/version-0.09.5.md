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
- **Planetary Stewardship moves to Stewardship; its road is Efficient Grids, Green Consensus and Clean
  Power** (was Green Consensus alone), in the designer's list's words. It lists Clean Power and Green
  Consensus only: both need Efficient Grids, so a line straight from Efficient Grids is not drawn, at
  the designer's word.
- **The Upload needs Civil Defense** (was Closed-Loop Colonies). Civil Defense needs Large Language
  Models, so the chain is the ticket's "Public Science and Civil Defense". The Upload names only Civil
  Defense because a second line from Large Language Models would run straight behind the Civil
  Defense box and read as passing through it, the fault ticket #245 took out of this same row. The
  Archive still stands at a Colony off Earth; The Upload gates the Archivists' win, not the Archive
  order (ticket #361).
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

## 2. Four prerequisites moved, and the lines redrawn

The authority is [ticket #425](https://github.com/whaleyjoshua2/Dying-Earth/issues/425).

| Tech | Needs, before | Needs, after |
|---|---|---|
| Missile Technology | Hardened Hulls | **Nuclear Rockets** |
| Automated Refining | Deep Mining, Efficient Grids | **Deep Mining** |
| Closed-Loop Colonies | Expanded Habitats, Clean Power | **Expanded Habitats** |
| Efficient Transit | Clean Propellant | **Orbital Refuelling** |

So **no Victory road leaves its own row**: the Prospectors' is Deep Mining, Automated Refining,
Beneficiation and the Charter, **130** (was 148); the Arkwrights' is Expanded Habitats, Closed-Loop
Colonies and Generation Ships, **98** (was 148). Their pick lists drop the Techs that left their
roads (Efficient Grids; Efficient Grids and Clean Power). No cost or effect moves.

**The lines, redrawn** at the designer's word ("adjust that branch to look better"; and on the
Stewardship row, "what with the vertical line between clean manufacturing and plantary stwardship"):

- **Orbital Refuelling is drawn above Clean Propellant**, level with the Efficient Transit it feeds,
  so Propulsion reads as two lanes: Orbital Refuelling -> Efficient Transit -> Hardened Hulls, and
  Clean Propellant -> Nuclear Rockets and Cryogenic Tanks, Nuclear Rockets -> Hardened Hulls and
  Missile Technology. A Tech's place in a stacked cell is `stack` in `techs.toml`, lower first;
  absent is the tree's order.
- **A line from another row enters its box a quarter from the edge it comes from**, not at the
  middle, so two lines into one box never meet at its door.
- **A line that changes rows climbs on the source's side of the gap**, so a fork reads as leaving the
  Tech that feeds it, not as a line between the two boxes on the right.

**What would show this wrong**: a road that reaches another row; two lines meeting at one door; a
vertical beside the door of a box the line does not enter.
