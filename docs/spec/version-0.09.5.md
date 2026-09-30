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

## 3. Commodity Finance, and every Faction building carries its base building

The authority is [ticket #426](https://github.com/whaleyjoshua2/Dying-Earth/issues/426).

**Commodity Finance** is the twenty-fifth Tech: Extraction rung 2 at **32 Research**, needing Deep
Mining. The whole tree now costs **821**, where it cost 789.
- **Its effect:** it multiplies by **1.15** the Ducats of every **Bank, Investment Bank, Trade Post
  and Exchange**, after the Faction's output multiplier. The Trade Post is in at the designer's
  word.
  - The Exchange's extra Ducat stays flat on top.
  - The Investment Bank's 1% interest into the Venture Capital Fund does not change.
- **Who gets it:** everyone, once it completes, as with every Tech.
- **The Extraction Charter** needs Automated Refining and Commodity Finance, where it needed
  Beneficiation. **Beneficiation** stays in the tree and leads nowhere.
  - The Prospectors' road stays **130**: Deep Mining, Automated Refining, Commodity Finance, the
    Charter.
  - Their pick list opens with those three.
- **Where it is drawn:** in the middle of Extraction's rung-2 stack, level with the Charter.
  Automated Refining sits above it and Beneficiation below.

**A Faction building carries everything its base building does**, at the designer's word:
*"please make sure all faction specific buildings also carry the base yeilds"*. This was already
true of every building's yield, Tech bonus, event card and job. The Investment Bank pays a full
Bank's Ducats and adds its interest on top. Seven places matched a building by name or kind and
now match it by the job it does:
- **The Custodians' mothball bonus:** a mothballed Reactor they hold doubles a Generator, as a
  Power Plant does.
- **The computer seats, four places:**
  - a Reactor answers an Energy shortage;
  - a Chorus is a Colony's first Relay;
  - an Exchange counts as a producing building;
  - an Exchange is weighed with its extra Ducat.
- **The sweep:** it counts Investment Banks, Exchanges, Choruses and Heliostats with their base
  buildings.
- **The Region card:** it names the player's own Bank ("an Investment Bank here would add") at its
  real figure.

Three stray Mass Driver figures on the Exchange's data row, which nothing read, are removed.

**What would show this wrong**:
- a Bank, Trade Post, Investment Bank or Exchange that pays the same with Commodity Finance as
  without;
- an Exchange whose extra Ducat grows with it;
- an Investment Bank's interest that moves;
- a place where a Faction building does less than its base.

## 4. Orbital Data Centers

The authority is [ticket #433](https://github.com/whaleyjoshua2/Dying-Earth/issues/433). The
designer added it on 2026-09-29, reopening ticket #424's "Society's rung 2 left as it is".

**Orbital Data Centers** is the twenty-sixth Tech: Society rung 2 at **32 Research**, needing Large
Language Models. The whole tree now costs **853**.

- **Its effect:** it multiplies an **Observatory's Research by 1.5**. This comes on top of Large
  Language Models' x1.5 and The Upload's x1.25, and the figure is rounded down once, at the end.
  It is the first Tech aimed at Research made off Earth alone.
- **The Upload needs Civil Defense and Orbital Data Centers**, at the designer's word ("q3 B"). The
  Archivists' road is Large Language Models, Civil Defense, Orbital Data Centers and The Upload:
  **130**, where it was 98. Their pick list opens with those three.
- **Where it is drawn:** under Civil Defense in Society's rung-2 stack.
- **Large Language Models' box text** now says what it has always done: *"Region, Lab and
  Observatory Research x1.5"*.

**What would show this wrong**: an Observatory that makes the same Research with it as without; a
Lab or Region lifted by it; The Upload open without it.
