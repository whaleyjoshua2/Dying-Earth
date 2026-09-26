# Dying Earth — version 0.09.2, the tidy-up version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/365), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-25-version-0.09.2/`](../dev-diary/2026-09-25-version-0.09.2/).

**What the version is.** Version 0.09.1 with the designer's list, which is mostly the game saying
what it does, where the player can see it. Sections are added here as each ticket closes; the
closing ticket writes the summary and the win column.

---

## 1. No card on the first turn

The authority is [ticket #367](https://github.com/whaleyjoshua2/Dying-Earth/issues/367).

**No card of either kind is drawn on the first turn.** The Draw Chance is not rolled before
`first_draw_turn` (2, in `events.toml`), and the Event Deck is **not touched** before it: nothing is
rolled and nothing is spent, so the card on top waits for turn 2's ordinary roll and no card is
lost. Both kinds are held off, the Choice Card that would ask before the first order and the
ordinary Event that would land in the first Resolution. The loader refuses a figure of nought. The
off-Earth join on turn 12 is unchanged.

**Why.** The first turn is where a player reads their Condition and gives their first orders; before
this a card came on turn 1 half the time, and a question before the first order was the interruption
the designer removed. Measured before the rule over sixty seeds, thirty drew on turn 1.

**Reaches the computer seats** by the same phase, so it is the same game for all four.

## 2. The first-to-land line off the system map

The authority is [ticket #368](https://github.com/whaleyjoshua2/Dying-Earth/issues/368).

The Solar System Map's Body label **no longer carries the first-to-a-Body line** in either state,
*"first to land: 15 Influence"* or *"first settled by the Prospectors"*. The line **lives in the
Body's hover**, the one that unfolds the orbital-slot list while the pointer is near the Body, under
that list. The **surface card's own line is unchanged**: the glyph, the settler's colour and the
windfall hover stay. No rule moves; the bonus itself (§2 of 0.09.1) is untouched.

**Why.** The designer: *"do not display 1st founding bonus on system map."* Kept in the hover rather
than removed so that #345's point stands, that a voyage is chosen from this map.

## 3. Glyphs in the Trading window

The authority is [ticket #369](https://github.com/whaleyjoshua2/Dying-Earth/issues/369).

The Trading window **reads as the top bar does**: a glyph at the head of each row before the good's
name, and a glyph on every figure -- the header's Ducats, the price, the sell price, and the faces of
the Buy and Sell buttons -- by the game's one rule for glyphs, a word traded for its glyph only
directly after a number. The Buildings sentence and the Trades-this-turn list are drawn by the same
rule. No word is dropped: a row still carries the good's name. No price, rule or figure moves.

**Why.** The designer: *"glyphs in market window."* Every good the window trades and the currency it
trades in already had a glyph; the window alone drew none.

## 4. Colonists waiting aboard off Earth, and a Ships heading in the Report

The authority is [ticket #370](https://github.com/whaleyjoshua2/Dying-Earth/issues/370).

**The player's own Colonists still aboard a Ship in any orbit off Earth are reported every turn
they wait**, one line a Body: *"4 Colonists wait aboard in low orbit of Mars."*, or *"at Ares over
Mars"*, or, in more than one orbit, *"at Mars: 2 in low orbit, 2 at Ares"*. When a rival's Orbital
Control shuts the ground, or a rival blockade shuts the station they are docked at, the line ends
*", blocked by rivals' control of the orbit"*. A Ship in transit is not reported; a rival's Ships
are never reported; a spectated game, having no seat of its own, has no line. It is written after
every landing the turn has made, so a Ship that unloaded this turn is not said to wait. It never
takes the headline.

**The Report has a fifth heading, Ships, above Your works**, and every Ship line reads under it --
arrivals, orbit changes, losses, holds, the waiting line -- whoever's Ship it is. Until this version
a Ship line read under In space. The order of headings is In space, On Earth, The climate, Ships,
Your works.

**Why.** The designer: *"add report line about colonist waiting to be settled for those still in
ships in low orbit of any body but earth"*, then, on the ticket: any orbit, the player's own, and
*"above your works, move the rest of the ship lines there too."* Colonists aboard count for nothing
until landed, and nothing said so.

## 5. One net Unrest line a Region

The authority is [ticket #371](https://github.com/whaleyjoshua2/Dying-Earth/issues/371).

**The Report says one line per Region about its Unrest**, at the end of the Resolution, by Region in
the board's order, after the migration lines: *"India: Unrest from 3 to 5.5 (a Heatwave, agitation by
the Prospectors, a Mothball), past the first threshold: the Standing Army no longer replenishes."* The
figures are the Region's Unrest as the Resolution opened and as it closed; the causes are named in
the order they landed; a threshold crossed, up or down, is the line's ending. It is said **only where
a cause moved it**: the natural fall alone is not news. It is said for **the player's own Regions**,
held or occupied, and for **any Region the player Agitated or Relieved** this turn; a rival's Region
a rival acted in is silent, unless its holder is thrown off, which has its own line. A spectated
game, having no player, says every Region's.

**Six lines are gone**, each now a cause of that one line: a Mothball's or Decommission's, a Climate
card's (*"a Heatwave"*, *"a Drought"*, *"the Unrest card"*), a Strip Permit's (its line keeps the
Emissions half, which is its own news), an Agitate's (*"agitation by the Prospectors"*, or *"held to
nothing by the Constabulary"*), a Relief's, and the threshold line. **Two movers that never had a
line are causes too**: an Occupation (*"the Occupation beginning"*, *"the Occupation"* each turn,
*"the Occupation breaking"*) and a Choice Card's Unrest effect (*"the Refugee Convoy card"*), so
that every change inside the Resolution is named. The heat's rise is the Climate phase's, before the
Resolution, and keeps its own line under The climate. **The migration line's Unrest
clause is gone too**: *"India took in 0.6 people"* says only the migration, and *"5.0 people
arriving"* is a cause on the Unrest line. The log keeps every line as it was.

**Why.** The designer: *"quiet unrest spam"*, and the playtest's *"Unrest lines in the Report run out
of order."* The six sources wrote in phase order, so one Region's lines lay scattered among
another's, and a Region could take six in a turn.

## 6. Seven defects from the list

The authority is [ticket #366](https://github.com/whaleyjoshua2/Dying-Earth/issues/366). Nothing
here is a design decision; each is a thing the game said or did that was untrue or unreachable.

1. **The Exchange stands on a station, and the Heliostat too.** A station's build check reads the one
   predicate that answers by the job, so a Faction's own kind of a common Module follows it; the
   check was a second list of kinds that left both out, while the build buttons offered them. The
   Exchange is under the one-Trade-Post-per-Body cap, being the Trade Post it is.
2. **The driver takes a Region by its name or its id**, case and separators ignored (`india`,
   `"the united states"`, `southasia`), and its error lists the names the board prints.
3. **A place that passed to a Faction and threw them off in the same Resolution says both in one
   line**, *"India passed to the Custodians and threw them off at once; it stands neutral."*, in the
   transfer's place; and the driver's note on an occupied place reads *"you hold it in name on 47
   Standing; the Custodians occupy it."*
4. **A rival's founding names the slot**, as the player's own has since 0.09.1.
5. **A closed card side names the good and the shortfall**, *"you have 12 Fuel of the 20 it asks"*,
   in the engine's refusal, the driver and the card modal alike, from one place.
6. **The lift line says what the Spaceport will pay and when**, *"(+2 Influence next turn from the
   Spaceport)"*, and **the Energy line names where each shut building stands**, *"Spaceport in
   China"*. The playtest's guess, that the Spaceport was shut for Energy, cannot be the cause: a
   shut Spaceport refuses the lift outright; the pay lands a turn late and was itemised nowhere.
7. **The unanswered Tech pick is not reproduced.** The refusal yields only when no Tech is left to
   research, not on an empty shortlist, and the suite holds a test of it; a Tech completing inside
   End Turn owes a fresh pick that blocks the *next* End Turn, which reads as "did not block" to a
   player who saw the prompt appear mid-turn. Nothing changed.

## 7. The Influence figure on a card reads the holder

The authority is [ticket #372](https://github.com/whaleyjoshua2/Dying-Earth/issues/372).

On a Region, Colony or station card, **the standings row's figure on a held place is the price to
take it**, read by the rule that takes it -- the greater of the challenger's threshold and the
holder's Standing plus the margin: *"Take at 67"* on a rival's place, and on the player's own place
*"A rival takes it at 67"*, the lowest price any rival pays. *"Threshold 50"* stays on a place nobody
holds. **Every hover that explains the margin uses the real one**, from the function the rule runs:
*"a margin of 27 (20, +2 for cold relations, +5 for the Constabulary)"*. No rule moves.

**Why.** The designer: *"update influence thresholds on cards to reflect current holders influence."*
The row printed the fixed formula whatever the holder stood at, so a place held on 47 read
*"Threshold 50"* when the price was 67, and its hovers quoted the flat base of 20 where a
Constabulary and cold relations had raised it by up to 12.

## 8. The Victory bar counts settled Colonists, with a band for those in transit

The authority is [ticket #373](https://github.com/whaleyjoshua2/Dying-Earth/issues/373).

**Settled is as it was**: a Colonist counts toward a Victory bar when they live in a Colony off Earth,
a station over Earth included; nobody aboard a Ship counts. **Every bar that counts Colonists** --
the Custodians' and Prospectors' twelve off Earth, the Arkwrights' thirty -- **draws a band beyond
its settled fill for the Colonists in transit**: everyone aboard the Faction's Ships, anywhere, a
Ship loaded and sitting over Earth included. The band is the fill's hue darker, with thin lines at
45 degrees, and stops at the bar's end; the label reads *"9 of 12 (+3 in transit)"*. **The band
never counts toward winning.** The Arkwrights' per-Body bar carries no band, a Ship in flight having
no one Body to count toward. The temperature bar's committed band stays flat.

**Why.** The designer: *"victory bar should only count settled colonist but add a extra area to
represent those in transit, similar to the temperature bar, make the in transit darker and add lines
at 45 degrees."* Colonists aboard counted for nobody and were shown nowhere on the bar, so a Colony
Ship on its way looked like no progress at all.
