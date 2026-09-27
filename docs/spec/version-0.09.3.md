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

## 5. A Choice Card the player cannot engage with

The authority is [ticket #388](https://github.com/whaleyjoshua2/Dying-Earth/issues/388). The
designer's complaint was three Ship cards in one game for a seat with no Ships, each printing
*"you had nothing to decide"* without saying why. The reading first: such a card **never held the
turn** in the engine; what the player saw was the headless driver printing both sides of a card it
could not answer, and a Report line with no reason.

**A card that reaches no seat at all is put back.** When the Question phase draws a Choice Card
that neither side can reach for any of the four seats, the card goes to the **bottom of the deck
unspent** and the next card is the turn's draw, whatever it is; once a turn, so the deck cannot
loop, and a second unreachable card in a row is drawn and passes everyone by. When the deck holds
only that one card it comes straight back up and is drawn the same way. The Report says so in one
line: *"Orbital Debris reached nobody at the table and went to the
bottom of the deck unspent; the next card was drawn."* A card that reaches a rival but not the
player is **still drawn**, and the rival is asked.

**A seat the card cannot reach is told why.** The Report line for such a seat names the reason in
the engine's own words, read off the first effect on either side with nothing of the seat's to land
on, and the outcome: *"Grounded Fleet: you have no Ship, so it passed you by; neither side
applied."* (*"the Prospectors have no Ship, so it passed them by"* for a rival). The reasons are
phrases in `report.toml`: no Ship, no Ship in orbit, no Region, no Region with people in it, no
Region making Widgets, no such Facility, no Colony with a given Module, no Colony with room for
one. **Neither side is applied** to such a seat, as before; the line now says it. The headless
driver prints the card's name, its question and that one line for a seat it passed by, and not
the two sides.

**The computer seats** see the put-back too, since it is keyed to the whole table. **The sweep**
(20 seeds x four seatings at the shipped cell): **6 / 4 / 1 / 7, collapses 62**, against
6 / 6 / 1 / 5 and 62 after §4; within a seed's noise.

## 6. The Unrest lines quieter again

The authority is [ticket #400](https://github.com/whaleyjoshua2/Dying-Earth/issues/400). Version
0.09.2 folded six sources of Unrest lines into one net line a Region; what it left was the heat,
which wrote a line for every Region on the board whose Unrest rose that Climate phase, whoever held
it, and the sea, which ended every threshold line with an Unrest clause. Measured over eighty games
played by the computer, the heat was twenty thousand of the thirty-two thousand lines that carried
an Unrest figure.

**The heat is a cause on the net line.** A Region whose people the heat took and whose Unrest rose
has *the heat* among the causes on its one net line (*"India: Unrest from 4 to 5 (the heat, 3.0
people arriving)"*), for the player's own Regions and any they acted in, as every cause is; and
the whole board is **one line**, *"The heat raised Unrest in 9 Regions and took people from 4."*,
under The climate. The per-Region heat lines are gone.

**The sea is a cause on the net line** in the same way, and the sea-level threshold line keeps the
slots the sea took with no Unrest clause after it; the people it moved are the migration line's.

**What keeps a line of its own**: the coral Break, a Region throwing its holder off, an Occupation
ending, and Pioneers mustered, each an event and not a figure. The Break's own rise in Unrest,
which the moved net line would otherwise fold silently, is a cause on the net line too, named for
the Break.

**Where the net line is written.** It was written at the Resolution's end, before the Climate
phase raised the heat and the sea; it is written after the Climate phase now, still from the
snapshot the Resolution opened with (or, on a first turn with no Resolution before it, from the
Climate phase's head), so its before and after span the whole turn, and the causes the Resolution
pushed (an Agitate, Relief, refugees, an Occupation) are carried across to it. A cause is named
once on the line however many times it pushed in the turn: three sea thresholds in one turn read
*the sea* once, where the build first read *the sea, the sea, the sea*. This dedupe was not in the
resolution and is named here for the designer's eye.

**The measure**, every Report line carrying an Unrest figure, eighty games (20 seeds x four
seatings, seat 0 played by the computer, so every Region's net line is written), 2,351 turns:

| | before | after |
|---|---|---|
| Unrest-bearing lines a turn | **13.54** (31,842) | **4.72** (11,099) |
| of them the Climate kind | 20,433 | 1,472 |
| of them the net line | 3,594 | 5,819 |
| most in one turn | 39 | 16 |

The net lines grew because the heat and the sea now write to them; a human player sees only the
net lines of their own Regions and of any they acted in, so their Report is quieter still. No rule
moved.

## 7. The Stadium, an entertainment building that lowers Unrest

The authority is [ticket #389](https://github.com/whaleyjoshua2/Dying-Earth/issues/389). Only
Regions carry Unrest, so the building is a Region Facility.

**The Stadium damps, it does not drain.** While it stands and is online it **halves what the
climate adds** to its Region's Unrest: the heat, the rising sea, a Break, a Climate card. It
touches neither an Agitate nor arriving Refugees. **It stacks with a Constabulary**: the
Constabulary takes its half point off a rise first (a flat half, whatever the rise), and the
Stadium halves what is left, so a heat rise of one lands as a quarter where both stand, and the
sea's rise of two as three quarters (`stadium_factor` in `unrest.toml`). **Unrest therefore moves
in quarters** from this version, at the designer's word after the review, where it moved in halves;
a quarter prints to two places (*3.25*) and every other figure as before.

**Its row**: 20 Materials, 4 Widgets, 1 Energy upkeep, a build slot, one to a Region, no Tech, no
Emissions; cheaper and lighter than the Constabulary's 25, 4 and 2, at the designer's word. **Every
Faction builds the same Stadium**; no Faction versions this version. A second in a Region is
refused at the door as a second Constabulary is.

**The computer seats** raise a Stadium only where a Constabulary already stands and Unrest is
still 5 or more, at the Constabulary's weight and, at the designer's word after the review, with
its multipliers (the threat multiplier in a Region just Occupied, the opportunity multiplier at
Unrest 9), without which the computer never chose one: the Constabulary is the first answer to a
restive Region, since it drains as well as damps; the Stadium is the second.

**The sweep** (20 seeds x four seatings at the shipped cell): **6 / 4 / 1 / 9, collapses 60**,
against 6 / 4 / 1 / 7 and 62 after §5, within noise; the computer built 42 Stadiums over the four
seatings against 404 Constabularies.

## 8. The Region card rearranged

The authority is [ticket #390](https://github.com/whaleyjoshua2/Dying-Earth/issues/390). Nothing
here moves a rule; it is where things stand on the card.

**The order of the card**: the header and its figures; **Influence**; **Pioneers**; **Policies**,
the block that was called Orders, holding the Custodians' Leapfrog, the Arkwrights' Exodus Call,
the Prospectors' Strip Permit, Raise Industry Level, and Relief and Resettle without the Unrest
subheading they stood under; Widgets and the queue; **Facilities**: the slot boxes, then the
completed Sea Wall and Scrubber rows, then the build strip, then the Scrubber's and the Sea Wall's
build buttons; **Armies**: the stance, the stack and the Army rows; and **Orders**, holding **Build
Army**, since *"all army/ship builds should be in orders sections for both"* cards. One departure
from the resolution's wording, which put the two slotless buttons "among" the strip's: the strip
shows its buttons only while a free box is clicked, and a Scrubber or Sea Wall takes no slot, so
their buttons stand under the strip whatever box is clicked; with a free box clicked they read as
the last two of the list. The Colony
card's Orders section stays as it is, holding Build Army (Barracks) and Ships (Shipyard).

**What a completed Sea Wall or Scrubber row says**: one glyph-rendered line, *Scrubber: +3.0 ppm
Sink, 1 off Unrest a turn, 3 Energy upkeep* and *Sea Wall: holds the sea off; 3 rises held, 1.5
Materials a turn to keep* (*no rise held yet*; *unkept this turn, holding nothing* when unkept);
the row's whole sentence is its hover. The separate *N Scrubbers here take …* note under Influence
is gone, the rows saying it.

**Their build buttons**: *Scrubber (2 of 3)*, the state's cap in the label where a sentence stood
after it, and *Sea Wall*, each priced as every build button is with its Ducat price beside it. The
resolution said "the *or* words go"; the *or* turned out to be the Ducat-price button every build
button carries (*or 60 Ducats*), which the ticket's charting had misread as filler, so it stays on
these two as on the rest. That is an override of the resolution's line and is said here for the
designer's eye.

## 9. Every place card says its output, and a Colony's founding date

The authority is [ticket #391](https://github.com/whaleyjoshua2/Dying-Earth/issues/391). No rule
moves; the total is a read of the engine.

**The Output row.** Under *Region population* on a Region's card and under *Colonists N of M room*
on a Colony's or station's, one glyph row, *Output:*, carrying Materials, Energy, Fuel, Ducats,
Widgets and Research as the place makes them this turn, in the top bar's order, a figure only
where the place makes any (*nothing this turn* when it makes none). **Energy is net of the place's
own upkeep**, so a home Region whose Facilities eat more than its Power Plant makes reads a minus;
the other figures are gross. **A Region's Ducats include its economy**, and **the Widgets are the
place's whole figure**, the Industry Level's or the Core Module's with the Factory's, the same
figure the card's *Widgets N a turn* line says. The row is what the place
made this turn, at this turn's multipliers (the Drought's, a Storm Surge's, a card's), summed from
the same per-building yields the Income pass uses: **a building shut for Energy or mothballed made
nothing**, and the hover says so. A place nobody directs shows no row.

**The founding date.** Under a Colony's or station's name in its card's header, small: *Founded
July 2032* on a ground Colony, *Built July 2032* on a station, since the glossary has a station
built and a Colony founded; a starting station reads the game's first date. No date on a Region.
