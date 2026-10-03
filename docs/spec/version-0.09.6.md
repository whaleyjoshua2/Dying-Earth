# Dying Earth — version 0.09.6, the growth version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/435), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-03-version-0.09.6/`](../dev-diary/2026-10-03-version-0.09.6/).

## 1. The crash on a Colony Ship at a Venus station

The authority is [ticket #436](https://github.com/whaleyjoshua2/Dying-Earth/issues/436). No rule
moves.

**Before:** a Colony Ship's card offered "Unload N into <place>" for each of your places at its Body
that had room. It named each place by its **ground** slot, and an Army landing did the same. For a
station the slot number counts stations, so:

- every station over **Venus**, which has no ground slots, crashed the game when the card opened;
- Earth's 4th and 5th stations would have crashed it too;
- every other station was named for a ground site on its Body.

**After:** a station is named for its station slot and a ground Colony for its ground slot, on both
buttons. The designer's save (turn 33, a Colony Ship at Aphrodite over Venus) opens its card and
offers "Unload 4 Colonists into Aphrodite".

Every other ground-slot lookup in the window was checked; each one is on a path that only ever
names a ground slot.

**What would show this wrong:** a Ship card that panics at any station, or a button that names a
ground site for a station.

## 2. Moving Colonists from a Colony Ship onto a station

The authority is [ticket #437](https://github.com/whaleyjoshua2/Dying-Earth/issues/437).

**Unchanged rules, now seen working:**

- **Onto a station.** A Colony Ship unloads into one of your stations from that station's own
  orbit, and changing orbit still costs a turn and 1 Fuel. In any other orbit the Unload button is
  greyed, and its hover says where to move: "Move this Ship to Venus, at Aphrodite, then unload next
  turn". The crash (§1) had hidden this.
- **Back off a station.** Colonists may go back from any place of yours, station or ground, onto a
  Colony Ship in that place's orbit.

**Changed:**

- **Over Earth, the Ship card lists your stations too.** Under the Pioneers drop-down, each station
  of yours over Earth with Colonists aboard offers "Load N Colonists from <station>" on a slider.
  Until now the card at Earth offered only the Regions' Pioneers, so a Colonist lifted to the ISS
  could not leave it. Antarctic Colonies are not listed, as decided: stations only.
- **Unloading brings Education.** Colonists unloaded onto a place that already stands blend their
  Education into the place's, as every other arrival does. Before, they were added to the count
  alone.

**What would show this wrong:**

- a station over Earth with Colonists that a Colony Ship in its ring cannot load from;
- a greyed Unload button with no orbit named;
- four Colonists at Education 2.0 unloaded onto four at 1.0 leaving anything but 1.5.

## 3. Generation Ships needs Relay Networks

The authority is [ticket #438](https://github.com/whaleyjoshua2/Dying-Earth/issues/438).

- **Generation Ships needs Closed-Loop Colonies and Relay Networks**, where it needed Closed-Loop
  Colonies alone. Both lines are drawn: one into the box's upper quarter, one into its lower.
- **Relay Networks stays at 32.** The Arkwrights' road is Expanded Habitats 18, Closed-Loop
  Colonies 32, Relay Networks 32 and Generation Ships 48: **130**, where it was 98. **All four
  Victory roads now cost 130.**
- **The computer Arkwrights** research Relay Networks straight after Closed-Loop Colonies, before
  the Propulsion line.

**What would show this wrong:**

- Generation Ships available with Relay Networks unresearched;
- a Victory road at any price but 130;
- the Arkwrights' pick list reaching Propulsion before Relay Networks.

## 4. The Archive fund at 150

The authority is [ticket #439](https://github.com/whaleyjoshua2/Dying-Earth/issues/439).

- **The Archive costs 150 Research**, where it cost 125. That figure is also the Archivists'
  first Victory part. Both stored figures moved (`modules.toml`, `factions.toml`), along with the
  Archivists' card text and the glossary. The turn-one line and the Victory window read the figure,
  so they follow it.
- **The computer Archivists' pace** runs to 150 on the same turns: 38 by turn 10, 75 by turn 18,
  113 by turn 26, 150 by turn 32. It had stayed at 80 since version 0.05.5.
- **Measured before** (balance suggestions, Archive at 150 alone): 12 / 9 / 1 / 15 with 42
  collapses, against 0.09.5's 10 / 10 / 1 / 17. The version's sweep reads it with the other figures.

**What would show this wrong:** an Archive that completes, or a first Victory part that fills, at
any figure but 150; or a computer Archivist pace that ends anywhere else.

## 5. The Custodians' Influence at ×1.15

The authority is [ticket #440](https://github.com/whaleyjoshua2/Dying-Earth/issues/440).

- **The Custodians' Influence multiplier is 1.15**, where it was 1.2. The card shows "x1.15" and
  its hover "Influence Allotment x1.15", both read from the data.
- **Rounding is unchanged:** the Allotment is multiplied, then rounded down to a whole point, as
  every Faction's is. From China at the start, (10 + 3) × 1.15 = 14.95 gives **14** a turn, where
  ×1.2 gave 15.
- **The computer Custodians** read the Allotment, not the figure, so nothing of theirs moves.

**What would show this wrong:** a Custodian Allotment of 15 from China alone on turn 1, or a card
that reads x1.2.

## 6. A Shipyard wants one Factory, not one every turn

The authority is [ticket #441](https://github.com/whaleyjoshua2/Dying-Earth/issues/441). No rule
moves; this is how the computer seats spend.

- **Before:** once a Colony had a Shipyard, every computer seat wanted another Factory Module there
  each turn "because a Ship is wanted". Nothing counted the Factories already standing, and the want
  carried the full victory-gap boost. A yard Colony filled its free slots with Factories. The
  Arkwrights' one station over Earth spent its Materials on them and never launched a Colony Ship.
- **After:** a yard Colony wants **one** Factory on the yard's account, counting those standing and
  on order. A Colony whose build queue is deep still wants one on its own account, as before.
- **All four computer seats.**
- **The designer's ideal**, given on this ticket: about **15 / 15 / 15 / 15 wins and 20 collapses**
  over the 80-game sweep. The trial (balance suggestions, Ar1) moved collapses from 42 to 32, toward
  it.

**What would show this wrong:** a computer yard Colony with two Factory Modules and a short queue,
or one with none while it builds Ships.

## 7. Colony Ships hold 35 Fuel, and the build price matches the tank

The authority is [ticket #455](https://github.com/whaleyjoshua2/Dying-Earth/issues/455), added by the
designer after charting.

- **The Colony Ship's base tank is 35**, where it was 40. Clean Propellant's +5 and Cryogenic
  Tanks' +15 are unchanged: 40 with the first, 55 with both. Warships stay at 30.
- **A Ship is built with the tank it was paid for.** The Fuel is paid at the order, as before, and
  the build now records it.
  - The Ship comes out holding exactly that.
  - Room a tank Tech added while it was building comes empty, as it does for a Ship already flying.
  - Before, the Ship was filled to the tank on the turn it finished, so a Tech completing mid-build
    gave up to 15 Fuel nobody paid for.
  - A build in a save older than the record fills as it always did.
- **Cancelling a Ship build refunds its tank's Fuel** with the Materials. Before, the Fuel was lost.
- **The build button's hover** ends "Built full: N Fuel."
- **The computer seats buy the Fuel ahead of every Ship build**, warships included, as they did for
  a Colony Ship alone. Where they cannot pay this turn, the build waits like any other want.

**What would show this wrong:**

- a new Colony Ship holding anything but 35 before Clean Propellant;
- a Ship holding more Fuel than its order paid;
- a cancel that leaves the Stockpile short of the tank;
- a computer warship build refused for Fuel it never planned to buy.

## 8. A captured Scrubber runs at half

The authority is [ticket #445](https://github.com/whaleyjoshua2/Dying-Earth/issues/445).

**Before:** every Scrubber in a Region was destroyed when the Region changed hands (by Influence,
Occupation, Pacification or a throw-off). **After:** it stands, and runs at a share set by who holds
the Region (`facilities.toml` `[scrubber]`):

| Held by | Sink | Unrest relief | Blame credit | Energy upkeep |
|---|---|---|---|---|
| the Custodians | 3.0 ppm | 1 a turn | the holder's | 3 |
| any other Faction | 1.5 ppm | 0.5 a turn | the holder's | 3 |
| nobody (neutral, after a throw-off) | 0.75 ppm | 0.25 a turn | nobody's | none |

- **Taken back by the Custodians**, a Scrubber runs whole again.
- **A Scrubber still on order** is cancelled when its Region changes hands, as before; only the
  Custodians may build one.
- **A neutral Scrubber shut for want of Energy** runs again at the throw-off, since a neutral Region
  pays no upkeep. A mothballed one stays mothballed.
- **A Region occupied from neutral** has no controller, and its Scrubber still adds nothing, as since
  ticket #351. The quarter is a *neutral* Region's. This case was not put to the designer; it keeps
  the rule it had.
- **The Report** says "N Scrubber(s) in <Region> now run at half" (or "a quarter", or "full
  strength"), where it said they were destroyed.
- **The Region card's Scrubber line** gives the shared figures, e.g. "+1.5 ppm Sink, 0.5 off Unrest a
  turn, 3 Energy upkeep (x0.5)". The Energy-shortfall line's "the Natural Sink loses N ppm" counts
  each shut Scrubber at its share.
- **The transfer's destruction roll** (spec 8.3) still applies to a Scrubber as to any building.
- **The computer seats:** no new want.

**What would show this wrong:** a Scrubber destroyed by a change of hands (other than by the
destruction roll); a Prospector-held Scrubber crediting 3.0 ppm or none; a neutral one charging
upkeep or crediting a seat.

## 9. A Colony Ship takes Pioneers from several countries

The authority is [ticket #443](https://github.com/whaleyjoshua2/Dying-Earth/issues/443).

- **Several Regions in one turn.** A Colony Ship at Earth may take one Load from **each** Region its
  seat directs, in one turn.
  - The Loads count together against the Ship's room, crowding included.
  - Two Loads never come from one Region ("this Ship already loads from that Region this turn").
  - Loads stand beside one another and nothing else: no Transit, Unload or change of orbit that turn.
- **Low orbit needs no Launch Site** (Q5, asked mid-build). A Colony Ship in **low orbit** over
  Earth takes Pioneers from any Region its seat directs, Launch Site or none. A station's ring still
  needs a working Launch Site in the Region, and so does a lift straight onto a station. Before,
  every load from a Region needed one, in any orbit (ticket #46).
- **The Ship card** at Earth lists one row per Region with Pioneers waiting, each with its own slider
  and a "Load N Pioneers from <Region>" button, where a drop-down chose one Region. Each slider runs
  to the room left after the Loads already placed.
- **The Region card's "Send N to <Ship>" door** opens for a Colony Ship in low orbit even without a
  Launch Site. A Region without one says "only a Colony Ship in low orbit takes Pioneers from here".
- **The computer seats** fill a Ship from their Regions, most Pioneers waiting first, until full. In
  low orbit that includes Regions with no Launch Site.

**What would show this wrong:**

- a Ship refused a second Region's Load in the same turn;
- two Loads from one Region accepted;
- the Loads together passing the room;
- a low-orbit Load refused for want of a Launch Site;
- a station-ring Load accepted without one.

## 10. Founding a station from a ground Colony, and a ground Colony from a station

The authority is [ticket #442](https://github.com/whaleyjoshua2/Dying-Earth/issues/442).

| To found | Before | After |
|---|---|---|
| a station, off Earth | built for Materials, with a ground Colony of yours on that Body; at Venus, with any Ship of yours there | still built that way with a ground Colony; **or founded** by a Colony Ship in that station's own ring, a Core and the Colonists aboard, no Materials. At Venus only founded |
| a ground Colony | a Colony Ship in low orbit unloads | still; **or built** from a working station of yours over that Body, for the station's Materials price, opening with a Core and nobody |
| over Earth | a station from a Launch Site; Antarctica by Colony Ship or by sea | unchanged |

- **Sending people down.** A station of yours sends Colonists down to a ground Colony of yours on the
  same Body.
  - Free, any number within the Colony's room, on a slider.
  - It lands at the Resolution, with their Education blended.
  - One order a station a turn, and not while the station is under Blockade.
  - Down only.
  - A Colony Ship can still unload into a ground Colony, as before.
- **Venus has no low orbit.** It has no ground, so the orbit that reaches the ground has no purpose.
  - Its Ships sit in its three station orbits. A move to Venus names one; the default is the seat's
    own station's orbit, else the first free one.
  - A Ship an older save left in Venus low orbit loads into the first station orbit.
  - Venus low-orbit Control and Blockades are gone, and the map no longer labels them.
- **The computer seats:**
  - they found a station from a loaded Colony Ship's ring where they have none at that Body;
  - they no longer build one at Venus for Materials;
  - they build a ground Colony from a station where they have none on the ground;
  - they send a station's people down to a ground Colony with room.
- **Two foundings in one ring** in one Resolution: the first takes it, and the second stays aboard.

**What would show this wrong:**

- a station built at Venus for Materials;
- a station founded over Earth;
- a ground Colony built with no station of yours above it;
- a Ship in Venus low orbit;
- Colonists sent up, or past the Colony's room, or from a blockaded station.

## 11. Natural population growth and decline, in Colonies and on Earth

The authority is [ticket #444](https://github.com/whaleyjoshua2/Dying-Earth/issues/444).

- **Earth's rule is kept:** every Region grows 1% a turn, less 0.15% for each tenth of a degree above
  +1.2 C; a fall raises Unrest and sends Refugees. It is now **shown**: the Region card's population
  line has a hover giving the rule and this turn's figure ("This turn +1.00%: +14.5 million").
- **Colonies and stations grow.** Each grows by **2%** of its Colonists a turn (`climate.toml`
  `colony_growth`).
  - The fraction is kept on the place and lands as a whole Colonist when it reaches one: a Colony of
    ten gains one every five turns.
  - It stops at the room its Habitats give. A full Colony banks nothing.
  - It grows only while it is not under Blockade and its Core works.
- **And decline.** Under Blockade, or with its Core offline, a Colony loses **1** Colonist a turn
  (`colony_decline`) and banks nothing. Nothing else shrinks a Colony.
- **Born Colonists** know what the place knows, so Education does not move. They count for every
  Victory figure.
- **The Report:** one line a seat under Your works, "Colonies grew by N: <place +1, …>" (or "lost N",
  or both).
- **The card:** the Colonists line has a hover:
  - "Growing: +0.1 a turn, next Colonist in 9 turns";
  - or "Full: no room to grow";
  - or "Shrinking: 1 a turn, under Blockade" / "its Core offline".
- **The save** carries each Colony's fraction (defaulted, so an older save loads).

**What would show this wrong:**

- a Colony of ten with room not gaining one in five turns;
- a Colony past its room;
- a blockaded or dark Colony growing;
- an Earth Region's growth not matching its hover.

## 12. The Custodians' aggression follows a rival's CO2

The authority is [ticket #446](https://github.com/whaleyjoshua2/Dying-Earth/issues/446). Computer
seats only; no rule moves for a human Custodian.

- **The measure** is a rival's share of **this turn's emissions**, every seat's sources counted
  (`emissions_share`).
- **The lift:** against a rival above a fair quarter, the computer Custodians weigh every hostile act
  by `1 + emitter_k × (share − 0.25)`, at most `emitter_cap` (`ai.toml`: 2.0 and 2.0). A rival at
  40% is ×1.3; one at 75% or more is ×2. The acts:
  - Smear and Agitate;
  - Influence spent on that rival's Regions and Colonies;
  - an Army's attack and a march on that rival's Region;
  - a warship's Attack at a Body where that rival has Ships, and a Blockade of that rival's station.
- **Cause:** against such a rival the Custodians have cause at Relations **−3** (`emitter_cause`),
  where every computer seat needs −5 (`war_cause`). Every hostile act's gate reads one test,
  `has_cause`.
- **Wording:** the code's comments called −5 "Cold"; the game's scale calls it **Wary**, and the
  comments now say so.
- A Smear still needs the rival's Blame share above a fair quarter, as since ticket #267.

**What would show this wrong:**

- a computer Custodian Smearing or marching on a rival at −4 whose emissions share is at or under a
  quarter;
- any other computer seat acting at −3;
- a lift above ×2.

## 13. Two texts made shorter

The authority is [ticket #452](https://github.com/whaleyjoshua2/Dying-Earth/issues/452). No rule moves.

- **The Blame hover**, in the Faction window and the Climate Panel, is one shared text where each
  carried four long lines, word for word:
  > Blame: CO2 this Faction's places emitted, less what it removed (Scrubbers, Nature Reserves, the
  > Custodians' Sink Directive).
  > Above a quarter share: Influence thresholds rise up to +50% where it doesn't hold, and every
  > rival likes it a point less per step.
- **The Region's slots hover:**
  > Slots: Size 3 + 3 + starting Industry 3, +1 inland per raise.
  > 4 coastal: each sea threshold takes one (with its oldest Facility) and turns an inland slot
  > coastal. A Sea Wall stops the taking, not the turning.
  > Mothballed and building each keep a slot.
- Every rule the old texts stated is kept.

## 14. The Colonists line: 8/12, a glyph, coloured by fullness

The authority is [ticket #450](https://github.com/whaleyjoshua2/Dying-Earth/issues/450). No rule moves.

- **The form:** the Colony and station card's Colonists line reads `[people glyph] 8/12`, where it
  read "Colonists 8 of 12 room". The words move to its hover: "Colonists 8 of 12 room.", then the
  growth line (§11).
- **The colours:**
  - plain under three quarters full;
  - **amber** from three quarters;
  - **red** when full, since a full place takes no more Colonists and grows no further.
- **Where:** the Colony and station card only. Map labels and roster lines keep their words.

## 15. The computer seats use the market

The authority is [ticket #448](https://github.com/whaleyjoshua2/Dying-Earth/issues/448). Computer seats
only; no rule moves.

- **Energy bought where the seat would be short at Income**, the shortfall that switches buildings
  off. It is weighed ahead of the turn's other spending, at a producer's weight and the opportunity
  multiplier.
- **Selling, at the end of the turn, after everything else is chosen:**
  - **Materials** beyond 20, plus three turns of the larger of the seat's Materials income and what
    this turn's own orders spend (the designer's word on the measured miss: "a but make 3x");
  - **Fuel** beyond its Ships' empty tank room, plus 10.
  - Each is sold only at or above the midpoint price.
  - Never in a turn the seat bought that good.
  - No Materials while they are held for a dearer build.
  - No Fuel while Fuel is held for the Mars window (ticket #57).
- **The Prospectors** get no special rule: their Fund banks the Ducats from sales as from any income.
- **Energy** stays unsellable.
- **Measured,** and accepted by the designer as built (option E):

| | Cust | Pros | Ark | Arch | collapses |
|---|---|---|---|---|---|
| before | 10 | 8 | 8 | 8 | 46 |
| as shipped | 9 | 6 | 4 | 19 | 42 |

## 16. The computer pace tables checked and brought up to date

The authority is [ticket #449](https://github.com/whaleyjoshua2/Dying-Earth/issues/449). Computer seats
only; no rule moves. A pace table says how far along its Victory a computer seat should be by a turn;
behind it, the seat weighs what advances that Victory more heavily.

- **The Prospectors' pace** runs to the real bar: 333 by turn 9, 833 by 18, 1500 by 27, 2500 by 34.
  It had stood at 1000.
- **The Arkwrights are paced in Bodies**, which nothing read:
  - a Bodies schedule, 1 by turn 18, 2 by 24, 3 by 30, read as every schedule is, so a seat with no
    Body is behind from the start;
  - while Bodies are short, a lift onto the station over Earth is a foothold at half weight with no
    "behind" boost, as ticket #94 had it for a Ship's disembark there;
  - a Habitat is not progress on a Bodies part.
- **The Archivists are paced in Uploads**, at the designer's word: 4 by turn 26, 8 by 30, 12 by 34.
  The pace is read **only once the Archive stands complete**, since there is nothing to Upload into
  before.
- **The Custodians' pace** is left as it is. It is never met, so they push on the climate all game.
- **A guard test** holds every pace table's last figure to its bar: the first part, the Bodies and
  the Uploads.
- **Measured,** against the sweep before this ticket:

| | Cust | Pros | Ark | Arch | collapses |
|---|---|---|---|---|---|
| before | 9 | 6 | 4 | 19 | 42 |
| after | 8 | 3 | 6 | 22 | 41 |

The Arkwrights end at nought in 23 games, where it was 49.
