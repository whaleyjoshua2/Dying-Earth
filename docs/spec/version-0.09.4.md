# Dying Earth — version 0.09.4, the Custodians' version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/403), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-09-27-version-0.09.4/`](../dev-diary/2026-09-27-version-0.09.4/).

**What the version is.** Version 0.09.3 with the designer's list. The summary and the win column
are written when the version closes.

## 1. The defects the headless driver shows

The authority is [ticket #404](https://github.com/whaleyjoshua2/Dying-Earth/issues/404). Each item
is a thing the game said untruly or said badly, most of them in the headless driver; one rule moves
with them (item 1, the tribute).

1. **A confirm names everything the order pays.** The Stockpile's part first, then the Widgets a
   build takes from its place's rate, the people an Army takes, and the Fuel a move draws from a
   Ship's tank, the last two joined by *and*: *"costs 25 Materials, 4 Widgets and 1M people"* for
   an Army in a Region, *"… and one Colonist"* at a Colony, *"costs 20 Materials and 4 Widgets"*
   for a Factory. A build bought outright in Ducats starts done, so it names no Widgets. **A
   Launch costs the Warhead**; **a tribute costs its Materials or Ducats** (*"costs 15
   Materials"*); **a muster of emigrants costs its people** (*"costs 4M people"* for four
   Pioneers). The driver's confirm and the window's list of the turn's orders both say it. The
   driver's price list gives the Army's Widgets and people beside its Materials, as the Ships line
   gives theirs.

   **The one rule that moves**: a tribute is now paid from the Stockpile with the order's cost,
   where it was taken at the commit outside it. So the running total counts it, and a tribute
   beside other spending is refused when the Stockpile cannot cover both, where before it could
   overdraw. The computer seats pay no tribute, so the sweep is unmoved.
2. **One phrasing for a refused turn**: *"The turn did NOT end:"*, whichever of the three reasons
   stopped it.
3. **A seat prints as a number.** An Army row reads *seat 0*, or *seat -* for nobody's, where it
   read *Some(0)* and *None*; a Ship row's Army reads *army 2* or *army -* the same way.
4. **The Market every turn.** `show` prints the Market line under the Stockpile, and a price a
   Choice Card has set names the card and the turns it has left: *"Materials 1 (Cheap Ore Offer,
   1 more turn)"*, then *"(Cheap Ore Offer, its last turn)"*. A price moves at most a step a turn
   inside its band; a card is what swings it from 4 to 1. A save from before names *a card*.
5. **A move that spends the tank says so**: a transit *"costs 6 Fuel from the tank"*, an orbit
   change its orbit-change Fuel likewise, where both said *free*. A Launch and a Bombard draw no
   Fuel (the ticket's first comment said they did; the code says not); a Bombard says *free*.
6. **A place changing hands to or from the player is also listed under Your works**, as well as
   under its place: taken by force, by Influence or Pacified, won or lost, and a throw-off of the
   player; and **an Occupation begun or broken** by the player or on the player's place, which
   changes who directs it though not who controls it, including an Occupation of the player's
   ended by a third Faction's Influence. The headline's ranking is unchanged. A
   change of hands or an Occupation between two rivals is not listed there. The driver's Report
   prints under the window's headings, so it shows the same.

**What would show this wrong**: a Build Army confirm without its Widgets or people; a transit
confirm reading *free*; a Market line with no card named on the turns a Cheap Ore Offer's price
stands; a player's lost Region, or a player's Occupation begun, broken or taken by a third Faction,
missing from Your works; a Launch confirm reading *free*; a tribute missing from the running total.

## 2. The first turn under the Sink eases Unrest everywhere

The authority is [ticket #405](https://github.com/whaleyjoshua2/Dying-Earth/issues/405).

**The first Climate phase the world is ever under the Natural Sink eases every Region's Unrest,
once a game.** "Under the Sink" is the Stabilization test: the Emissions that count (everything
but the Climate cards and the Permafrost) against the Natural Sink and the Scrubbers, so the ease
and the Custodians' run never disagree. It fires on that phase whatever the next does; it does not
wait for a run of three.

- **Every Region on Earth, held or nobody's, eases by a half** (`under_sink_ease = 0.5`); **a
  Region the Custodians hold eases by a whole point instead** (`under_sink_ease_custodians = 1.0`),
  in `unrest.toml`. Nothing falls below nought. **An occupied Region is still its holder's** until
  the Occupation completes: a Custodian Region under a rival's Occupation eases by a whole point, a
  Region the Custodians are occupying by a half.
- The Climate phase is the second phase of a turn and the throw-off comes in that turn's
  Resolution, so the ease lands **before this turn's throw-off**, as the first Colony's does.
- A save from before this version, written while the world stood under the Sink, counts the ease
  as spent, so it is never announced twice.
- **One line for the whole Earth** under On Earth: *"The world is under the Natural Sink for the
  first time: Unrest eased by a half in every Region on Earth, a whole point in the Custodians'."*
- **A Moment of its own, *Under the Sink***, switchable in the Moments corner like the others,
  ranked with a Break, in the designer's words: *"For the first time, the world takes more carbon
  out of the air than it puts in. Unrest eases by a half in every Region, a whole point in the
  Custodians'."*
- **The computer seats** have no new weight; the ease falls where the Custodians' Scrubber and
  Leapfrog weights already push.

**Measured** (20 seeds x four seatings, the shipped cell; the sweep gained a line counting the games
in which the world was ever under the Sink): the world got under in **10 of 80 games, median first
turn 28**, before and after, since the ease follows the fact; the win column **7 / 5 / 1 / 7,
collapses 60**, unmoved. The ease moved small things (Constabularies built in the first seating 79
to 77, Agitates landed a few either way) and nothing a Faction's result turns on.

**What would show this wrong**: a Region's Unrest unchanged on the first turn under the Sink; a
second ease on a later turn under it; a Custodian Region eased by a half, or a rival's by a whole
point.

## 3. The Custodians' Victory line in ppm, and partial credit for the run

The authority is [ticket #406](https://github.com/whaleyjoshua2/Dying-Earth/issues/406).

**The row says the gap.** The Custodians' first Victory row reads *"Stabilization: 21.3 ppm over
the Sink (counted 27.6, Sink 6.3), run 0 of 3"*, and *"… ppm under the Sink …, run 2 of 3"* once
under, off the last Climate phase and by the Stabilization test (counted Emissions against the
Natural Sink and the Scrubbers). The Climate Panel's Stabilization line and the headless driver's
Victory row print the same words, so no two screens disagree. A game opens **30.9 ppm over** (36.9
counted against a Sink of 6).

**The run earns partial credit.** The Custodians' first part is worth the greater of:

- **the best run of the game** over its bar of 3, never falling back; and
- **three tenths of the best share of the opening gap ever closed** (`stabilization_gap_cap = 0.3`
  in `victory.toml`), the opening gap being the counted gap at the game's first Climate phase, and
  never falling back. **The opening figure, not the worst the gap has been**, at the designer's
  word: a gap that grows through the middle game and is cut back earns nothing until it is under
  where the game began, since measuring from the peak would pay for letting Emissions climb first.

Three tenths is under the third one turn of run is worth, so a real run always beats any cut. **The
Condition does not change**: a win is still three phases in a row. The score (the lower of the two
parts) orders the final ranking and picks the turn-36 winner when nobody has met a Condition, as it
does for the other three Factions, so **the Custodians can now win at turn 36 on score**. A save
from before opens its gap at the next Climate phase.

**The computer seats** play as before: their Custodian pace already reads the gap in ppm, and
where they read how near a Faction is to its Condition (the Lead's pick of its gate Tech past half
the first part, and refusing an Accord to a Faction at the door) they read the part **as it
stands**, not the partial credit. The Rival Moment and the Victory chart carry the credit, as the
score does.

**Measured** (20 seeds x four seatings, the shipped cell; the sweep gained a line per Faction, its
score at the end and its places in the final ranking):

| Custodians | before | after |
|---|---|---|
| score at the end, median | 0.00 | 0.13 |
| games at nought | 73 | 12 |
| placed 1st / 2nd / 3rd / 4th | 7 / 0 / 6 / 67 | 8 / 14 / 43 / 15 |

The win column moved **7 / 5 / 1 / 7 to 8 / 4 / 1 / 7**, one turn-36 win passing from the
Prospectors to the Custodians; collapses 60, unmoved. The Arkwrights, whose score is nought in 75 of
80, now take the last place in 54 games where they took it in 11.

**What would show this wrong**: a Custodian score falling when the gap widens again; a run of two
worth less than two thirds once broken; the gap credit reaching a third; the Victory row and the
Climate Panel quoting different figures.

## 4. Say Conquer-and-Mothball, and its price

The authority is [ticket #407](https://github.com/whaleyjoshua2/Dying-Earth/issues/407). No rule moves.

**The price is the order's, not conquest's**: a Mothball in a Region costs +1 Unrest there and a
Decommission +2, whoever holds it, and nothing damps either; in a Colony neither costs Unrest.

- **The buttons say it, in as few words as carry it** (the designer: *"way way fewer words"*): the
  Mothball button's hover *"+1 Unrest"*, the Decommission button's *"+2 Unrest, 10 Materials back"*,
  half the building's Materials, the figure its own. In a Colony, no hover.
- **The headless driver's confirm** says `costs +1 Unrest`, or `+2 Unrest`.
- **The Custodians' Faction text says the idea**, as its last sentence: *"Their lever on Earth is
  switching industry off, at home or in a Region they have taken: each Mothball there costs +1
  Unrest."* Its Scrubber and Leapfrog figures are read from the data, so it cannot go stale: it said
  4 Energy upkeep and 2 turns, where the Scrubber costs 3 and is paid in 8 Widgets.
- **The playtest note** carries a short *Playing the Custodians* section saying the same.
- **The other three texts were audited against the data**; two were stale and are corrected: the
  Arkwrights' Colony Ship carries 16 with Generation Ships too and costs them 25.5 Materials; the
  Archivists' Provisional Findings holds while at least 75% of their Research went to the shared
  Tech last turn (the figure read from the data, as the Scrubber's cap now is in the Custodians'
  text), and keeping back more switches it off for the turn after. **The Reactor takes a quarter
  off** its holder's Energy upkeep, as the code and its own line have it; the Archivists' text said
  75% off, and the designer ruled the code right.

**What would show this wrong**: a Mothball button with no hover in a Region; a confirm reading
`free` for a Mothball; a figure in any Faction's text that the data contradicts.

## 5. Cut the language

The authority is [ticket #408](https://github.com/whaleyjoshua2/Dying-Earth/issues/408). No rule moves.

Five Report lines cut to what they must say, in the designer's words:

| Line | Now |
|---|---|
| A Tech completed | *"Coastal Engineering is complete. The Prospectors led and pick the next Tech."* |
| An Occupation broken | *"The Prospectors' Occupation of Egypt broke: +2 Unrest."* |
| First to a Body | *"The Archivists are first to settle the Moon: +5 Influence."* |
| A throw-off | *"Egypt threw off the Custodians."* |
| A Sea Wall holding | *"China's Sea Wall held the sea; keep now 4 Materials."* |

The log keeps the Tech's shares; the Report does not. The first-to-a-Body line no longer names the
Colony; it points at it, and the Moment names it. **The Constabulary's and the Stadium's sentences
on the Region card are dropped**, nothing in their place; each building's own box says what it does,
and the Constabulary's now says it as the engine does it (*"takes 1 off this state's Unrest a turn
and half a point off each rise from the climate, refugees or an Agitate"*), where it said *halves*
and left refugees out. The Unrest figure's hover says Unrest moves *in quarters*, where it said *in
halves*.

Two cases the cut exposed: **an Occupation that breaks where nothing rose** (a Colony, which has no
Unrest, or a Region already at the top) says only *"The Prospectors' Occupation of Tycho on the Moon
broke."*, where the figure had been claimed regardless; and **a Region whose name ends in s** takes
an apostrophe alone (*"The United States' Sea Wall"*). The Sea Wall's line keeps the coast's suffix
when a rise turns an inland slot coastal (*"… The coast now reaches one slot further in."*).

**What would show this wrong**: any of the five lines in its old words; a Colony's broken
Occupation claiming Unrest; a Constabulary's box saying *halves*.

## 6. A Colony Ship unloads any count of Colonists

The authority is [ticket #409](https://github.com/whaleyjoshua2/Dying-Earth/issues/409). No rule moves;
the engine already took any count.

- **A slider picks how many**, from 1 to the most the place will take (the designer: *"up to max
  number the outpost will take"*): into a Colony, the Colonists aboard or the room left in its
  Habitats, whichever is fewer; onto a free slot, what a new Colony's Core holds (4). No slider
  where only one fits.
- **Founding too**: one slider over a Ship's founding buttons, the same on every slot, the button
  reading *"Found a Colony at Tycho with 2"*; an Army aboard lands with the founding, as before.
  The rest stay aboard.
- **The computer seats unload any whole number**, as many as fit, founding or disembarking, so
  they never stick on a count; a founding now asks for what lands (it asked for the whole load and
  the Resolution landed what fitted, so nothing changes in play).
- The turn's order list names the count (*"Found a Colony at Tycho on the Moon with 2 from …"*), and
  one Colonist is *"1 Colonist"*. The founding slider remembers its count per Ship.

**What would show this wrong**: a slider running past the room left; a founding that lands more
than the Core holds; a computer seat's Unload asking for more than fits.

## 7. How the computer seats manage Unrest

The authority is [ticket #410](https://github.com/whaleyjoshua2/Dying-Earth/issues/410). **The
computer's play moves; no rule does.** Measured first, the sweep gaining a line per Faction (the
Region-turns it held, those at Unrest 4 or more and at 7 or more, the throw-offs it suffered, and
seat 0's start Region lost to a throw-off or to a taking).

Four changes, their figures in `ai.toml` `[thresholds]`:

- **A Constabulary from Unrest 4**, where the Standing Army stops replenishing, not 5.
- **A Mothball priced**: in a Region at Unrest 5 or more its weight is a quarter, since the point of
  Unrest it adds is damped by nothing.
- **Relief from 5**, its weight rising from one at 5 to double at 9, where it stood at one from 6 and
  double from 9.
- **A Stadium without a Constabulary** where the Region has one slot left and no Constabulary on
  order to take it, from Unrest 5 as before (`stadium_from`).

**Measured** (20 seeds x four seatings, the shipped cell):

| | held at 7+ before | after | throw-offs before | after |
|---|---|---|---|---|
| Custodians | 14% | 9% | 61 | 57 |
| Prospectors | 19% | 14% | 20 | 12 |
| Arkwrights | 13% | 5% | 13 | 10 |
| Archivists | 17% | 11% | 9 | 4 |

Held at 4+: 38 / 39 / 35 / 44% to 32 / 37 / 25 / 41%. Start Regions lost to a throw-off 17 of 45
to 11 of 44. Wins **8 / 4 / 1 / 7 to 7 / 6 / 1 / 5**, collapses 60 to 61; the Archivists' and the
Custodians' falls are within the noise of twenty seeds a seating. Constabularies built fell (77 to
52 in the Custodians' seating, 148 to 122 in the Prospectors'); why is not measured.

**What would show this wrong**: a computer seat offering no Constabulary at Unrest 4; a Stadium
alone in a Region with slots to spare; the share at 7+ rising in the closing sweep.

## 8. The Archivists' reward for leading a Tech

The authority is [ticket #412](https://github.com/whaleyjoshua2/Dying-Earth/issues/412).

**Each time the Archivists lead a Tech to completion they win 5 Influence and half a point of Unrest
off every Region they hold** (`lead_influence`, `lead_unrest_ease` on their card in
`factions.toml`; nought for the other three). The Influence is paid into the Allotment **at once**,
whenever the Tech completes, and never twice. The Tech's line says it, and says only what happened
(holding no Region, *"…: +5 Influence."*): *"Coastal Engineering is
complete. The Archivists led and pick the next Tech: +5 Influence, -0.5 Unrest."* Their computer
seat's Research Directive is unchanged.

**Measured** (the sweep gained a line, the Techs each Faction led): the Archivists led **126 of
1,656** Techs over 80 games before, **130** after; their held Region-turns at Unrest 7+ fell 11% to
9%; their wins 5 and 5. The win column 7 / 6 / 1 / 5 to 8 / 4 / 0 / 5, collapses 61 to 63, within
the noise of twenty seeds a seating.

**What would show this wrong**: a Lead other than the Archivists paid; the Influence missing from
the next Allotment; a Region they hold not eased.

## 9. Orbital Refuelling, and Clean Propellant's bigger tanks

The authority is [ticket #413](https://github.com/whaleyjoshua2/Dying-Earth/issues/413).

**Orbital Refuelling** is the twenty-third Tech: Propulsion, **rung 1 beside Clean Propellant,
needing nothing and needed by nothing**, priced at **22 Research** where its rung costs 18, on
purpose. It **cuts a crossing's days by a tenth before the rounding up**, and where Nuclear Rockets
stands too the two multiply (0.72). At the Launch Window:

| | neither | Orbital Refuelling | Nuclear Rockets | both |
|---|---|---|---|---|
| Earth to Mars | 5 turns | 4 | 4 | 4 |
| Earth to Venus | 3 | 3 | 2 | 2 |

So the stacking gains only off the window, where a crossing is longer. The Fuel of a leg is
unchanged. **Clean Propellant adds 5 Fuel to every Ship's tank** (30 to 35), a Ship already flying
gaining the room empty and a new Ship built full at 35; at half under Provisional Findings, as every
addition is. **The computer seats pick Orbital Refuelling first in the Propulsion chain**, after
each Victory gate chain. The whole tree now costs **757 Research**, where it cost 735.

**Measured** (20 seeds x four seatings, against the sweep after §8): wins **8 / 4 / 0 / 5 to
9 / 5 / 0 / 9**, collapses **63 to 57**; Victory gates completed 66 / 61 / 58 to 60 / 51 / 52 of
80 (Prospectors, Arkwrights, Archivists; the Custodians' 70 to 69), a cause not measured; Missile
Carriers built 40 to 32, games with an orbital Battle 28 to 21.

**What would show this wrong**: a crossing slower with both Techs than with Nuclear Rockets alone; a
new Ship built at 30 once Clean Propellant stands; a Tech that needs Orbital Refuelling.

## 10. The Tech tree: the path lit on a hover, the boxes a tenth smaller

The authority is [ticket #414](https://github.com/whaleyjoshua2/Dying-Earth/issues/414). No rule moves.

- **A hover lights the path back to the root**: the hovered Tech and every Tech it needs, all the
  way down, and the lines between them, drawn at full strength with a bright border and a thicker
  line; everything else fades to a third. A Tech done on the path keeps its green line; a Victory
  gate on the path keeps its Faction's border, thicker. Nothing changes until the hover.
- **The whole tree a tenth smaller**: boxes 110 x 52 on a 130 x 77 grid, where they were 122 x 58
  on 144 x 86. The type stays 12 point; a name that no longer fits its box (*Planetary
  Stewardship*, *The Extraction Charter*) comes down only as far as it must.

**What would show this wrong**: a hover lighting a Tech the hovered one does not need; a lit path
missing a Tech it does; a name spilling out of its box.

## 11. The Victory window's bars in the Faction's colour

The authority is [ticket #418](https://github.com/whaleyjoshua2/Dying-Earth/issues/418), added to the
map by the designer on 2026-09-28. No rule moves.

**Each Faction's two Victory bars are filled in its own colour**, its heading's, where every bar was
the theme's one blue (as the glossary's *Victory bar* entry had always said they were); **the
Colonists-in-transit band is that colour darkened**, hatched as before.

## 12. The Arkwrights' Unrest eased by every place they found off Earth

The authority is [ticket #419](https://github.com/whaleyjoshua2/Dying-Earth/issues/419), added to the
map by the designer on 2026-09-28.

**Each ground Colony the Arkwrights found off Earth takes a point off the Unrest of every Region they
hold, and each Space Station they build half a point** (`found_colony_unrest_ease`,
`found_station_unrest_ease` on their card; nought for the other three). Antarctica, being on Earth,
eases nothing; a place taken rather than founded counts for nothing. The founding's own line says it
(*"… with 4 Colonists: -1 Unrest."*, *"… built Starlab: -0.5 Unrest."*), and only where something
eased.

**It is part of their computer seat's Unrest management**: founding a ground Colony and building a
station weigh more as their most restive Region's Unrest rises, one at 4 and double at 9
(`founding_pull_from`, `founding_pull_double_at` in `ai.toml`), as Relief does since §7.

**Measured** (against the sweep after §9): the Arkwrights' held Region-turns at Unrest 7+ 360 to
355, their throw-offs 7 and 7; stations on the Moon 15 to 20 (all seats; the sweep does not split
them by Faction); ground Colonies unchanged to the Body; wins **9 / 5 / 0 / 9** and collapses
**57**, unmoved. The ease fires seldom. **The pull on a ground Colony does nothing measurable**: it
sits on the landing itself, which is offered only to a loaded Ship already in low orbit and was
chosen anyway; the stations are what moved. The designer left it so (Q6, A): stations answer Unrest,
Colonies are founded as they were. The line states the rule's figure; a Region already
nearly calm eases by less.

**What would show this wrong**: a rival's founding easing the Arkwrights; an Antarctic founding or a
taken place easing anything; the line claiming an ease that did not land.
