# Dying Earth — version 0.08.8, the campaign version: Armies that can be moved, stacked and sent by right-click, Passage as a rule, the defects and the respawn checked, Carriers off Earth for every seat, a Module that defends a Colony or station, refuelling by Accord, escorts that take the fire, rolls per warship, Bombard, and a new mark for Battles on the map

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.8](https://github.com/whaleyjoshua2/Dying-Earth/issues/316), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-22-version-0.08.8/`](../dev-diary/2026-09-22-version-0.08.8/).

**What the version is.** Twelve tickets, six of them rules of war and one of them the largest
build since the war version. The playtest report of 0.08.7 said the ground war had frozen: a
Region's own Army could not march, the computer marched one Army at a time at 60 percent odds and
never massed, and nothing was landed from orbit. This version unfreezes it. A Region's own Army
marches again (§5); Armies stack for orders and the computer masses (§6); a click on a shield and
a right-click on the map moves the stack (§7); Passage is a rule for Armies and not a word (§4).
In orbit: every seat may want a Carrier and Intercept fires (§3); the Battery is the first Module
that fights and the first thing that denies Orbital Control (§8); Refuel Accords are read by the
rules (§9); escorts take the fire (§10); a big fight rolls as many dice as it has hulls (§11); and
a Battleship holding an orbit may Bombard (§12). The Battle mark replaces the ring on the map,
orbit included (§1), and the two measured defects were checked and one residue fixed (§2).

**What it did to the win column**, 20 seeds across four seatings at the shipped climate cell
(sink 6, step 300), against the 0.08.7 baseline: **Custodians 43, Prospectors 22, Arkwrights 1,
Archivists 0** where it was 43 / 21 / 2 / 0; collapses **14 of 80**, as before. The military block
moved as the map expected it to, and by a great deal: Battles opened 62 to **225**, marches on a
held Region 5 to **347**, places taken by force 2 to **169**, Standing Armies lost 2 to **161**.
See *What the closing sweep says* at the foot, which says which ticket did it.

---

## 1. A new way to signify Battles on the map, orbit included

*Ticket [#317](https://github.com/whaleyjoshua2/Dying-Earth/issues/317).*

The ring of 0.08.7 goes. **One mark** everywhere a Battle was fought last turn: a Region on
Earth, a Colony on its Body, and a Body in orbit on the Solar System Map and in the Body's surface
view. The mark is a drawn crossed-swords glyph, off-white as every kind glyph is, on a disc in the
aggressor's colour, beside the place's label; its hover reads the record and its click opens the
Report. The Solar System Map's page, when nothing is selected, carries a **Battles last turn**
list, one row per Battle in the aggressor's colour with a jump; the orbit band of a Body carries
the same row. The shields' outline is dropped; the red damage pip stays. The 3D stack cones are
untouched. The glyph is the game's own drawing and owes no credit.

## 2. The two measured defects and the two-turn respawn: built in 0.08.5; what remains

*Ticket [#318](https://github.com/whaleyjoshua2/Dying-Earth/issues/318).*

Both items the map named were already done: ticket #284 (0.08.5) made the Battle line and the
Occupation read one predicate and made an occupier stay; ticket #282 (0.08.5) made a destroyed
Standing Army return two Incomes after it died. This ticket adds no rule. One residue is fixed:
**holding an Occupation reads the same presence an Occupation needs to begin**, so an Army that
escaped neither begins nor holds one. The respawn delay is a data figure, `respawn_incomes = 2`
in `units.toml [standing_army]`, where it was a literal.

## 3. Carriers off Earth for every seat, and an Intercept that fires

*Ticket [#319](https://github.com/whaleyjoshua2/Dying-Earth/issues/319).*

**Any seat** wants a Carrier when it has cause, by the war-cause bar, against the holder of a
rival Colony off Earth, and the Prospectors as before; the loading gate opens the same way. The
Army lifts from a Region with a Launch Site; the targets are any rival Colony off Earth whose
holder the seat has cause against, nearest first. **Intercept fires**: the Orbital Control
condition goes; Intercept outscores Hold when an unarmed enemy hull is inbound; an armed stack is
never intercepted, since that would open a Battle against whoever arrives. The Ship stack card's
Intercept label says on hover when the computer uses it, as measured behaviour. The sweep counts
interceptions. Measured over eighty games: Intercept fired where it never had; no Carrier was
built by anybody, the appetite being a conjunction the board did not yet give any seat at once.

## 4. Passage as a rule for Armies

*Ticket [#320](https://github.com/whaleyjoshua2/Dying-Earth/issues/320), amended.*

The Passage term of an Accord is a rule for Armies as well as Ships. Either party's Armies may
**march into the other's held Regions without attacking**, arriving on Hold; while the Accord
stands they fight nobody and defend nothing there, and are not counted among the marches on a held
Region; once it ends they are on Attack at the next Resolution, as any rival Army would be.
Neither party intercepts the other's Ships, and a Blockade does not shut a partner out of a slot.
The computer offers Passage **in the same offer as non-aggression**, because one Accord stands per
pair and a non-aggression Accord struck first would shut Passage out for good; it offers it to a
seat it is **Cordial or better** toward, the designer's amendment (*"make it based on the level
below friendly"*), when it holds a Region next door to one that seat holds, and accepts at Neutral
or better. Measured: Passage was struck in no seating of eighty games, the pairs never reaching
Cordial at an offer moment; the fog on the map carries the question of when it should fire.

## 5. Ground Armies that can actually be moved: the playtest report

*Ticket [#321](https://github.com/whaleyjoshua2/Dying-Earth/issues/321).*

The holder **may march a Region's own Standing Army**, which 0.08.6 kept at home. At home it
defends with its people, the Constabulary and the calm; marched out it is an Army like any other,
with no such bonus, and a threat next door like any other. Raised Armies change hands with their
Region. The computer marches its raised Armies first and its Regions' own last, at half the
weight, so the one-war-a-turn cap is not spent on the Army the people rely on.

## 6. Armies that stack for orders

*Ticket [#322](https://github.com/whaleyjoshua2/Dying-Earth/issues/322).*

A **stack** is every Army of one seat at one place, and every Ship of one seat at one Body. On
the Region card, under the stance row, one row of neighbour buttons **moves the whole stack**,
*All 2 (8): attack Russia*, with the odds hover reading the summed strength; the per-Army rows
stay for a split, and with one Army its own row is the stack. *Repair all* and *Repair all with
Ducats* when two or more are damaged. The Ship stack card's Transits carry *All N that can* per
destination, N being the Ships whose tank pays the leg. **No new engine order**: a stack's march,
transit or repair is the per-unit orders placed together, each legal on its own, priced at their
sum, so the arrival and the Battle are each unit's. The computer, where two or more Armies of its
may march from a place, offers one candidate that moves them all, its odds from the summed
strength, before the single marches; the playtest note's *"it never masses"* is false now.

## 7. Click an Army, then right-click the map to move it

*Ticket [#323](https://github.com/whaleyjoshua2/Dying-Earth/issues/323).*

A click on the player's own shield selects the Region and **arms its stack**: the shield wears a
ring in the roster's ring colour, every Region the stack may reach is outlined in it, and the card
scrolls to its Armies block. A **right-click** on an outlined Region places the stack's march, as
the card's button places it; on a Region not next door, a notice on the panel's notice line; with
nothing armed, nothing. A left-click elsewhere, Escape, End Turn or a change of view disarms. On
the Solar System Map, with the player's Ship stack selected, a right-click on another Body places a
transit for every Ship whose tank pays the leg, as *All that can* does. A second right-click on the
same target takes the orders back. The right-click cannot be exercised headless and has no picture
of its own; the orders it places are the card's, which are tested.

## 8. A Module for Colonies and stations to defend themselves: the Battery

*Ticket [#324](https://github.com/whaleyjoshua2/Dying-Earth/issues/324).*

One Module, the **Battery**, buildable on a Colony and on a station, no Facility twin on Earth:
25 Materials, 1 turn, 3 Energy upkeep, **strength 4, hit points 6** in `modules.toml`, the first
Module with combat figures; Hardened Hulls does not reach it. It is a party in the orbital Battle
at its Body, on its owner's side, on Hold, never disengaging; a rival stack ordered Attack fights
it whether or not its owner has a Ship there, at rung 3 as any Battle; shot to its hit points it is
gone. Its damage is repaired with Materials at its own Colony, as a Ship's is at a yard. **While
one stands and works, no rival holds Orbital Control at the Body**: nobody lands against its owner
and no Blockade shuts or starves its station; the owner gains no Control by it, so a fleet that
wants the ground must shoot the Batteries down first. It is not a ground defender. Mothballed or
offline it does nothing, as no Module that is not working does. The computer wants one where a
rival warship stands at the Body or a rival Carrier is inbound, one per Colony, at the Barracks'
weight; it reads rival Batteries in its attack odds, may open a Battle to clear one, and does not
blockade a station whose holder has one. Measured: on a fresh station with two free places the
Battery loses to the opening Habitat and the Trade Post, and is built once those stand.

## 9. Refuelling at a partner's station, negotiated

*Ticket [#325](https://github.com/whaleyjoshua2/Dying-Earth/issues/325).*

The Refuel term of an Accord is read by the rules. A station fuels a Faction's Ships when it is
their own **or a partner's under a Refuel Accord**: the Refuel order, the stranded rule, the card's
button and the computer's Refuel candidate all read that one test. A station blockaded against its
holder fuels nobody. The Fuel is always the refueller's own Stockpile's, drawn through the other
Faction's station, in the designer's words. No fee: the Accord is the negotiation, ended with a
turn's notice. The computer offers Refuel inside its non-aggression offer where the other seat
holds a station at a Body it has Ships or a Colony at and no station of its own, and accepts at
Wary or better as before; it does not yet plan a transit toward a partner's station as a refuelling
point. The sweep counts refuels at a partner's station.

## 10. Escorts take the fire

*Ticket [#326](https://github.com/whaleyjoshua2/Dying-Earth/issues/326).*

A combatant carries an **armed** flag set from its hull: a Frigate, a Battleship, a Battery and
every Army are armed; a Colony Ship and a Carrier are not. **While a party has an armed unit
engaged, every hit on that party lands on one of its armed units, drawn uniformly**; its unarmed
hulls are struck only once the last armed unit is down or gone. An unarmed hull that runs while its
escort still stands engaged is **not pursued**. The melee itself still knows no kinds; the caller
sets the flag, so the one function serves ground and orbit. The Carrier's card line *"needs an
escort"* is true now. The computer seats' habits are unchanged this ticket, at the designer's
word: the rule changes who dies, not who wins the exchange.

## 11. Rolls per engaged warship, not a flat three

*Ticket [#327](https://github.com/whaleyjoshua2/Dying-Earth/issues/327).*

A Battle in orbit **rolls for a hit once a round for every engaged armed unit across every side**,
warships and Batteries alike, and never fewer than three; a Battle on the ground still rolls three;
three rounds, unchanged. The figures are data, `units.toml [melee] rounds = 3, rolls = 3`. The
hitter and target draws, the escort rule, the hover's odds (the chance of winning the first
exchange by strength share) and the computer's attack bar are as they were. Measured: the batch
barely moved, because the computer's orbital fights are one or two hulls a side, under the floor of
three; the rule is there for the fleet action the game does not yet stage.

## 12. Bombard

*Ticket [#328](https://github.com/whaleyjoshua2/Dying-Earth/issues/328).*

A **Battleship** at a Body other than Earth, whose Faction holds Orbital Control there **outright**,
may Bombard a rival's Colony at that Body, one order per Battleship a turn, from the Ship stack
card. It resolves after the orbital Battles, with the checks made again: **one Module drawn at
random** (the Core Module and the Archive never in the draw) rolls the destruction chance a taking
uses; a burned Module is removed, and when it is a Habitat the Colonists beyond the room left die
with it, which is the one way a Bombard kills people. Each Bombard is an offence at **rung 3**
against the Colony's holder, breaks a non-aggression Accord if one stands, and charges a burned
building's war Emissions to the bombarder. **Never over Earth**, where the opening stations stand.
It is a Battle's act: a Report line hit or miss, a Moment when anything burned, and a Battle
record, so the Battle mark and the orbit band's row show it. A standing Battery, which denies every
rival Orbital Control, is the shield against it. The computer bombards from a Battleship holding
the orbit outright when it has cause against the Colony's holder, at the orbital Attack's weight;
the sweep counts Bombards and Modules burned. The fifth and sixth questions on the ticket were not
answered and were built as recommended, under the map's standing rule that a rule change reaches
the computer seats in the same ticket.

---

## Shot aids added for the pictures

Building aids, not rules: `arm:1` arms seat 0's start Region's stack as a click on its shield
would; `passage:1` strikes a Passage Accord between seats 0 and 1; `battery:1` gives seat 0 a Mars
Colony with a damaged Battery and seat 1 a Frigate in Mars orbit; `refuel:1` gives seat 1 a station
over Mars, seat 0 an empty-tanked Frigate there and a Refuel Accord between them; `bombard:1` gives
seat 1 a Colony on Mars and seat 0 a Battleship holding the orbit, and `bombard:order` places the
Bombard for `commit:1` to resolve; `simulate:<seed>` writes a headless log of a computer-only game.

## What the closing sweep says

Run as `sweep 20 --balance --seatings --steps=300`; the output is
[`docs/dev-diary/2026-09-22-version-0.08.8/sweeps/final-0.08.8.txt`](../dev-diary/2026-09-22-version-0.08.8/sweeps/final-0.08.8.txt),
read against
[`final-0.08.7.txt`](../dev-diary/2026-09-21-version-0.08.7/sweeps/final-0.08.7.txt).

| | 0.08.7 | 0.08.8 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 21 / 2 / 0 | **43 / 22 / 1 / 0** |
| collapses, of 80 (by seating) | 14 (2, 0, 12, 0) | 14 (1, 0, 13, 0) |
| Battles opened, of them in orbit | 62, 57 | **225, 52** |
| marches on a held Region | 5 | **347** |
| Occupations begun / broken | 2 / 0 | **233 / 31** |
| places taken by force | 2 | **169** |
| Standing Armies lost | 2 | **161** |
| Armies built / lost | 128 / 3 | 139 / 6 |
| warships built / lost | 218 / 1 | 217 / 2 |
| Armies landed from a Carrier | 0 | 0 |
| interceptions | 0 | 1 |
| Batteries standing at the end / lost | none existed | 39 / 8 |
| Bombards / Modules burned | none existed | 6 / 0 |
| refuel Accords standing at the end, by seating | 0, 0, 0, 0 | 1, 2, 0, 2 |
| Ships stranded at the end | 22 | 29 |
| Blockade-turns suffered | 172 | 145 |
| Dig In orders | 151 | 477 |

**The ground war is open again, and two tickets did it.** The marches on a held Region went 5 to
120 when a Region's own Army marched again (§5), and 120 to 332 when the computer massed (§6);
everything else in the block follows from those two: Occupations begun, places taken by force,
Standing Armies lost, Dig Ins (the answer to a threat next door), Battles opened. The
Arkwrights-first seating carries most of the war and went from 12 collapses to 13; the column moved
one win from the Arkwrights to the Prospectors, within the noise of a twenty-seed batch. The orbit
moved little: attacks in orbit fell 57 to 52, warships lost 1 to 2, and the orbital rules built here
(the Battery, the escort, the rolls, Bombard) fired where their conditions held, which on today's
board is seldom: 39 Batteries stood at the end and 8 were shot down, 6 Bombards missed, 2 refuels
were drawn at a partner's station, 1 interception was fought, and no Army was landed from a Carrier
in eighty games, as before. The Blockade-turns fell 172 to 145 with the Batteries standing.

**The test suite**: 366 engine tests and the rest all green, clippy clean across the workspace with
warnings denied, `GAME_VERSION` 0.08.8 with `SAVE_VERSION` moved to 2, so a 0.08.7 save is refused
by name rather than played under rules it was not written for.
