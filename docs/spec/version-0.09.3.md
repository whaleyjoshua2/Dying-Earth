# Dying Earth — version 0.09.3, the frontier version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/385), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-26-version-0.09.3/`](../dev-diary/2026-09-26-version-0.09.3/).

**What the version is.** Version 0.09.2 with the designer's list. The sections are written as the
tickets close; the summary is written when the last one does.

**What it did to the win column** (80 games, per Faction, at the shipped climate cell):

| | 0.09.2 | 0.09.3 |
|---|---|---|
| Custodians | 2 | |
| Prospectors | 5 | |
| Arkwrights | 0 | |
| Archivists | 11 | |
| collapses | 62 | |

---

## 1. Defects from the list, and the Tech tree audit

The authority is [ticket #386](https://github.com/whaleyjoshua2/Dying-Earth/issues/386). No rule
moves in this section; each item is a thing the game said untruly or said badly.

1. **The Fund is Ducats everywhere it is named.** The headless driver's status line reads
   *"Venture Capital Fund: 0 Ducats, banking 0% of Ducat income (0 banked last turn)"*; the
   Report's line for setting the share reads *"bank 40% of its Ducat income"*; the Investment
   Bank's text reads *"never less than 1 Ducat in all"*. The window's own line already said
   Ducats. The comments in the code that still said Materials say Ducats.
2. **End Turn's refusal names everything owed at once.** When a human seat owes both this turn's
   card an answer and the table a Tech, the one message reads *"Two things before the turn can
   end:"* with one line each under it; when one thing is owed, the message is that thing's line
   with *"The turn cannot end until you do."* after it, as before. The sun's hover, Enter's
   refusal and the driver's *STILL OWED* all print the engine's message. The driver's own Tech
   banner says *MUST* only when the engine's End Turn would refuse without the pick, and *"A TECH
   IS OPEN TO PICK … the turn can end without it"* when the tree merely has nothing chosen.
3. **The Tech tree audit found nothing stale by name.** No Tech names a Choice Card, and every
   Event a Tech's effect line names exists and matches the code. Four Techs under-state what they
   blunt (Hardened Hulls also the Meteor Shower; Closed-Loop Colonies also the Reactor Leak, Dust
   Storm and Moonquake; Efficient Grids also doubles Solar Maximum and trebles the Helium-3 Vein;
   Green Consensus also halves the Methane Burst and cancels the Drought), and the designer chose
   to leave those lines as the headline effect. **The Launch Pad Fire's text says the live rule**:
   *"Its Launch Site is offline until the next Resolution, so nothing lifts to orbit from there,
   and the Region loses the turn's Widgets"*, where it had said a due Ship completes next turn.

## 2. Materials, Fuel, Energy and Ducats carried to a tenth

The authority is [ticket #387](https://github.com/whaleyjoshua2/Dying-Earth/issues/387). This is
the version's one change under everything else: every later figure is written on top of it.

**The four resources are carried to a tenth**, and with them the Venture Capital Fund, a Ship's
Fuel tank, last turn's income and every cost. A figure is settled to the nearest tenth whenever it
is written, so a tenth is the finest step a stockpile ever moves in and no drift accumulates.

**Nothing in the data changed, and nothing whole became partial.** A whole price, output or upkeep
is still whole, because nothing partial arises from it. **What was partial and floored away is now
kept**: a Faction's multiplier on an output, an upkeep or a price (the Prospectors' Factory costs
21.3 where it cost 21; a Refinery under Automated Refining makes 4.5 Fuel where it made 4); a
Region's economy, which was a fifth of its GDP figure times its Industry Level rounded down and
now keeps its tenth (East Asia pays 10.2 where it paid 10); the Bank's and the Investment Bank's
Ducats and interest; the Fund's banked share and its withdrawal; the Drought's, the Storm Surge's
and a card's halvings; the Solar Array's sun-scaled Energy, the one figure that was rounded rather
than floored; Closed-Loop and Reactor upkeeps; Build Where You Dig's and a station's and a Ship's
prices; transit Fuel; the market's price and the sale price; carbon credits; a decommission's
refund; the Research Directive's Ducats and Fuel. Two consequences to name: **the Sea Wall's keep
is paid each turn to the tenth** (half a Materials a rise) where it was saved up and paid when it
reached a whole, the same Materials over time; and **a Friendly ppm of carbon credits costs half a
Ducat** where its half was rounded up to one. A withdrawal from the Fund stays a whole number of
Ducats, being whole already.

**What stays whole.** Widgets, Research, Influence, Standing and every count of people and things.
The computer seats' own estimate of a Region's Energy, which is an estimate and not a rule, keeps
its floors, and their thresholds (*20 Ducats or more*) read the tenths as they are; no rule for
them moved.

**How a figure is shown**: whole when whole and one decimal otherwise, *80* and *80.4*, wherever a
stockpile, an income, a price or a cost is printed: the top bar and its hovers, the seat table, the
Faction window's income row, the Trading window, the fund bar, the yield hovers (which printed two
decimals and now print one), the Report and the headless driver.

**The save moved**: `SAVE_VERSION` 7; a 0.09.2 save is refused, as every save-version move has
been.

**The sweep** (20 seeds x four seatings at the shipped cell): **5 / 6 / 1 / 9, collapses 59**,
against 0.09.2's 2 / 5 / 0 / 11 and 62. Every seat earns slightly more Ducats, the floors on the
Bank's, the Trade Post's and every Region's economy having gone.

## 3. Nuclear Rockets, a rung-2 Tech that cuts transit times by a fifth

The authority is [ticket #393](https://github.com/whaleyjoshua2/Dying-Earth/issues/393).

**Nuclear Rockets** is the twenty-second Tech: Propulsion, rung 2, **beside Efficient Transit**,
each needing Clean Propellant alone, and **Hardened Hulls needs both**. It costs **38 Research**
where its rung costs 32, priced above its rung on purpose, as Coastal Engineering is priced below.

**What it does.** A crossing between systems is worked in days from the launch window and rounded
up to sixty-day turns; Nuclear Rockets **cuts the days by a fifth before the rounding up**. So a
Mars crossing at the window is four turns where it was five, and Venus at its window two where it
was three. Measured over the game's thirty-six turns, it takes a turn off the Mars crossing on every
turn (two on the worst), and off the Venus crossing on thirty-one of thirty-six. A one-turn hop
inside a system stays one turn. **The Fuel of a leg is unchanged**, and a flight already under way
keeps its turns; the Tech reaches the next order. The Archivists' Provisional Findings read it at
half, as they read every Tech. Like every Tech it is the whole table's once researched.

**The computer's pick lists** are now the designer's order for every Faction: **its Victory gate
chain, then the Propulsion chain** (Clean Propellant, Efficient Transit, Nuclear Rockets, Hardened
Hulls, Missile Technology), **then the cheapest Tech left**. That dropped the Custodians' Coastal
Engineering, Efficient Grids, Clean Power, Civil Defense and Clean Manufacturing, and the
Archivists' Public Science, Coastal Engineering, Green Consensus and Civil Defense, from the named
lists; each is now reached only as the cheapest left.

**The sweep** (20 seeds x four seatings at the shipped cell): **5 / 6 / 1 / 7, collapses 61**,
against 5 / 6 / 1 / 9 and 59 after §2. The gates completed in 67 / 58 / 50 / 49 of 80 against
71 / 69 / 66 / 63, and the orbital war quietened (Missile Carriers built 32 where 69 were); the
figures are in the ticket's diary page.

## 4. The first Colony on each Body eases Unrest everywhere

The authority is [ticket #395](https://github.com/whaleyjoshua2/Dying-Earth/issues/395).

**The first ground Colony ever founded on a Body takes half a point off every Region's Unrest at
once**, whoever holds the Region or nobody, the moment the founding is claimed. It happens **once
for each Body**: the Moon's first, then Mars's, Phobos's and Deimos's, each again; Venus has no
ground to claim. A second Colony on a Body that already has one eases nothing; a Space Station is
no settling; Antarctica is on Earth and does not count. The figure is `first_colony_ease` in
`unrest.toml`, the smallest step Unrest moves in. It lands in the Resolution's cargo step, before
the Unrest step, so a Region at the top eased to nine and a half does not throw its holder off that
turn.

**What is said.** One line for the whole Earth, filed under the Report's *On Earth*, *"The first
Colony on the Moon eased Unrest by 0.5 in every Region on Earth."*, and the First to a Body Moment
carries the clause; the per-Region net Unrest lines stay quiet, so the easing never writes
fourteen lines the same turn. A Region whose net line has a named cause that turn carries the half
unnamed inside its figures, as it carries the natural fall.

**The computer seats** are reached by the rule itself, their Regions easing with everyone's.

**The sweep** (20 seeds x four seatings at the shipped cell): **6 / 6 / 1 / 5, collapses 62**,
against 5 / 6 / 1 / 7 and 61 after §3; within a seed's noise, as half a point once a game would be.
