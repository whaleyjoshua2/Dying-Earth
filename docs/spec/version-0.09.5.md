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

## 5. Two Pioneers a state a turn

The authority is [ticket #427](https://github.com/whaleyjoshua2/Dying-Earth/issues/427).

- **Up to 2 Pioneers a turn from each state a Faction directs**, in as many states as it likes, with
  one recruitment in each state a turn. Before, it was 4 a turn per Faction, all from one state.
  Two states at their cap make four.
- **Coach Class keeps its double**: the Arkwrights recruit 4 a state, at twice the population each.
- **An Exodus Call** doubles the figure in its state (4, or 8 for the Arkwrights) and waives the
  double charge while it runs, as before.
- **The computer seats** recruit only as far as their plan needs, from as many states as that takes.
  The plan is what their Colony Ships carry, plus Antarctica once the ice is open, plus the room in
  their places off Earth, less who already waits. States with a working Launch Site go first, the
  most populous first.
- **Each Pioneer recruited takes 0.125 Unrest off its state**, at the designer's word ("let's just set such that each pioneer is .125"): a batch of 2 takes 0.25 off, the Arkwrights' 4 take 0.5, and an Exodus Call's 8 take 1.0. Before, every batch took 0.5. The Recruit button's hover says the figure for the batch it offers.
- **During an Exodus Call** the Recruit button and the computer offer the Call's figure at the ordinary price in people.
- The Faction card, the tutorial and the glossary say the new figure.

**Measured, not yet**: how many Pioneers a game raises. The closing sweep reports it against 0.09.4.

**What would show this wrong**: three recruited in one state (five for the Arkwrights) without a
Call; a second state refused while the first recruits; a computer seat recruiting past its plan.

## 6. Pioneers moved by slider

The authority is [ticket #428](https://github.com/whaleyjoshua2/Dying-Earth/issues/428). No rule
moves.

**Every count of people is a slider**, the 0.09.4 Unload slider made general. It runs from 1 to the
most in steps of one, starts at the most, keeps a count you set while the card is open, falls to a
new most that is smaller, and is hidden where only one is possible.
- **Recruit** (Region card): up to what the state may recruit this turn.
- **One shared slider above the Region card's doors**: the sea to Antarctica, the lift to your
  station over Earth, and a Colony Ship at Earth. Each door sends that many, or as many as its room
  takes.
- **The Ship card at Earth**: the Region drop-down, with a slider under it.
- **The Ship card off Earth**: a slider over each place's Load button.

**What would show this wrong**: a door that sends more than the slider says or more than its room;
a slider that offers nought; a slider shown where only one is possible.

## 7. A warning on moving an empty Colony Ship or Carrier

The authority is [ticket #429](https://github.com/whaleyjoshua2/Dying-Earth/issues/429). No rule
moves.

- **Empty** means a Colony Ship with no Colonists, or a Carrier with no Army. A warship is never
  empty.
- **The card** says **"Empty."** in amber over its moves, for one hull, or **"2 empty"** for a stack
  holding two empty hulls. It costs no click.
- **A right-click move** skips the card, so it asks once before sending an empty hull: **"Empty Colony
  Ship. Send?"** (or "Empty Carrier. Send?"; "2 empty Colony Ships. Send?"; "3 empty. Send?"), with
  **Send** and **Back**. This covers a transit on the System Map and a change of orbit on a Body.
  Back places nothing.

**What would show this wrong**: a loaded hull or a warship named empty; a right-click that sends an
empty hull without asking; a Back that places the move anyway.

## 8. Fog of war

The authority is [ticket #430](https://github.com/whaleyjoshua2/Dying-Earth/issues/430) and its two
resolution comments. **Nothing about the fog is saved**: what a seat sees is worked out from the
board each time it is asked (`engine/src/visibility.rs`).

**What a seat sees**:
- **A Body off Earth**, and Earth's orbits and Antarctica, when it has a Colony or station there, a
  Ship there, or a working Relay (or a Chorus) at a Colony it directs there.
- **On Earth**, the Regions it directs and their neighbours. A working Embassy in a Region it directs
  shows the whole of Earth.
- **A rival's books** (stockpiles, income totals, Ships in flight) when that rival is **Cordial or
  better toward it**, or the two stand under an Accord.
- **A rival's doings** (its Under way list and its Report lines) when that rival is **Friendly toward
  it**, or under an Accord.

**Always open**: who holds a place, and what stands there (buildings, Colonists); Research and
Victory progress, the Victory window's "(+N in transit)" included; the Moments.

**What the fog hides**:
- **Rival Ships at a Body you do not see** are shown only as whose and how many ("Prospectors: 3
  Ships"). The same goes for **rival Armies in a Region you do not see** ("Arkwrights: 2 Armies").
- **Rival Ships in flight** are shown only with their books; a Ship is seen again when it arrives
  somewhere you see.
- **A rival's build under way** is shown only where you see its place.
- **A rival's income at a place** is shown only where you see the place.
- **A rival's Report line** is shown if it happened somewhere you see, or the rival is Friendly
  toward you or under an Accord.
- **A Battle** is shown if it was somewhere you see, or you fought in it.
- **The Report's own lines** about a Battle, a Ship, an Army or a rival's completed build are dropped
  where you do not see their place, before the headline is chosen. Lines about who holds a place,
  the climate, the Techs, the Events and the card stay open. A rival's transit is read at where it
  is bound.
- **Orbital Control** at a Body you do not see is not shown, since it would say whose warships hold
  low orbit. Nor is what a rival's building earns on its tile's hover, or what a rival's Region pays
  it, at a place you do not see.

**The computer seats see only what a human in their seat would.** `reveal_all` lifts the fog for
testing only: the headless driver's flag and the `reveal:1` shot aid set it.

**Measured** (20 seeds x four seatings at the shipped cell, `sweeps/fog-430.txt` against
`sweeps/reveal-430.txt`, the same batch with `DYING_EARTH_REVEAL=1`): the fog costs the computer
seats almost nothing. The win column is **10 / 10 / 1 / 17** (Custodians / Prospectors /
Arkwrights / Archivists) with the fog and without it, collapses **42 of 80** both ways; one
Arkwright Victory gate (73 against 74) and one placing differ. Accords are common: **148 to 174
struck in a seating's 20 games**, and 24 to 26 still standing at the end. An Accord opens a rival's
books and doings, so between pairs under one the fog is lifted.

**What would show this wrong**: a rival Ship's kind or strength shown at a Body you do not see; a
rival build under way shown at a place you do not see; a computer seat reacting to a Ship it could
not see; a save that changes what is seen.

## 9. The Report organized and trimmed

The authority is [ticket #431](https://github.com/whaleyjoshua2/Dying-Earth/issues/431). Measured
first (10 games, fogged, as seat 0 reads them): about **30 lines a turn, 56 on the busiest**, plus
about 17 rival clauses.

- **What happened to you first**: the headline, then **Your works**, open.
- **Every other heading is collapsed** with its count ("The climate (18)"). The rival paragraphs go
  under **Rivals (N)**, collapsed.
- **Folded into one line each, opening to the full list:**
  - the sea at every Region ("The sea at 13 Regions");
  - the Refugees and the Unrest of Regions not the player's;
  - each rival's completed builds ("Arkwrights: 2 buildings completed").
  The player's own Regions keep their Unrest and Refugee lines.
- **A rival's own act is its doings** under the fog (§8): a decommission, a Smear, Research directed,
  an Exodus Call. Each Report line now records whose act it is. A Smear aimed at the player always
  shows. A Greenwash stays public, as ticket #277 made it.
- **Cut**: a line repeated word for word is written once with a count ("... (x2)"); "Regions in no
  one's hands added N Research" is gone; "Colonists wait aboard" is written only when it changes.
- An Exodus Call was filed under Your works whoever sounded it; it is the player's only when the
  player's.

**What would show this wrong**: a busy turn opening at more than a screen; a rival's Note out of
sight; a repeated line written twice; the waiting line written on a turn it did not change.
