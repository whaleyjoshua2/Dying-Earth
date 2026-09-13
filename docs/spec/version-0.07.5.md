# Dying Earth — version 0.07.5, the reading-and-founding version: every Influence word explained, the Module grid on the card, a Core Module at founding, a tutorial, and a symbol for each Faction

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.07.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/160), and the
pictures that decided it are in
[`docs/dev-diary/2026-09-13-version-0.07.5/`](../dev-diary/2026-09-13-version-0.07.5/).

**What the version is.** Version 0.07.4 answered the designer's reactions to 0.07.3. This one takes
nine more, and two of them are not tidyings: **the Core Module** changes how every Colony and every
Space Station is founded, which is the version's one rules change and moves the balance a long way;
and **the tutorial** is the first the game has ever had, against a standing design that said *no
tutorial beyond the first Report* from the first playable. The rest is reading: every word of the
Influence section explained, the Modules moved onto their own card, a third history chart, a red
button, a window that opens the size it should, and a symbol for each Faction. **The sweep baseline
restarts** (section 11).

---

## 1. Every word of the Influence section explained on hover

*Ticket [#161](https://github.com/whaleyjoshua2/Dying-Earth/issues/161).*

Eight things in that section had no hover: the **Influence** heading, each Faction's **chip**, the
**Threshold**, **`nobody has any yet`**, the **Blame** note, the red **within-reach** warning, the
**Influence value** line and the **holder** line. All eight have one now, each naming the rule behind
its figure rather than restating it, as the rule set on ticket #116 requires. The wording is as dense
as it can be made, at the designer's word: the heading's went from six rendered lines to four with no
fact dropped.

**Your own chip says what the others do not.** Where a rival's chip gives the rule, yours on a place
you hold says what it would take the nearest rival to reach you: *A rival takes it at 179, your
Standing plus 20. The Prospectors are nearest at 128, 51 short. Every point you add here adds one to
that.* Where no rival has spent, it says what one would need.

**Threshold gains an entry in `CONTEXT.md`**, which it never had -- the rule lived inside Influence.
It says what sets it on a Region, a Colony and a station, what Green Consensus and Blame do to it,
that it is a floor and never the whole price, and plainly that this is the Influence sense and not
the climate one.

**The Colony's card follows the Nation card** rather than being special-cased: its `Influence:`,
`Spend` and `Buy more Influence in the Trading window` controls carry hovers of their own.

`icon_word` hands back its response now, so a heading drawn with a glyph can carry a hover.

---

## 2. The Module grid moves onto the Colony's card, and the Hab View window retires

*Ticket [#162](https://github.com/whaleyjoshua2/Dying-Earth/issues/162).*

A Colony's or station's Modules are tiles **on its own card**, laid out in **the same grid a Region's
build slots use** -- one constant for both, at the designer's word, so the two cards cannot drift
apart. Three columns fit the panel at its default width and were built first; they push the strip
below the fold on any station with room to grow, and the picture of that is in the diary.

**The Hab View window retires**, and with it the `Modules (M)` button, the list of Module names, the
Esc branch that closed it, the roster row's auto-open, the window's header and its `Esc closes.`
tail. `view.hab_view` is gone. Esc clears a clicked tile, which is what closing the window did. The
name retires too: `CONTEXT.md`'s **Hab View** entry becomes a retired term with its avoid-list, as
Defence and Restoration did, and the tiles are written into **Module** the way a Region's boxes live
in **Build Slot**.

**M swaps to the Solar System Map and back**, the job Tab held; the bar's button reads
`Solar System Map (M)`. **A Module is ordered from a free tile and nowhere else**, as a Facility that
takes a slot has been since #154, so the card's bottom header reads `Orders (hover a button for what
it does)`, matching the Nation card's, and keeps only the Army, the Ships and the Archivists' Archive.
The Build-Where-You-Dig discount note moved above the tiles, where it says what a Module here costs.

The `hab:1` picture aid **selects** seat 0's first station or Colony instead of opening a window;
`habtile:` stands on its own as `slotbox:` does.

---

## 3. The Pick a Tech button is red, and Tab brings the roster back

*Ticket [#163](https://github.com/whaleyjoshua2/Dying-Earth/issues/163).*

The top bar's `Pick a Tech` button wears **the red End Turn wears** (`120, 40, 30`) with light text,
since the two are the pair that gate a turn. It was black text on the default dark fill, the only
styled button on the bar and the least legible of the seven places the game asks for a Tech. **The
refused modal's button follows.** The Tech Tree's own `Pick` buttons and its yellow prompt are left
alone: six red buttons inside one window is a wall rather than a signal. The red is a named constant
now, `TURN_RED`, where it had been a bare literal twice.

**Tab brings the roster back.** The side panel shows the roster whenever nothing is selected, so Tab
clears the selection; the heading reads `Your roster (Tab)`, the rule every bar button follows.

A **`pick:0`** picture aid leaves the opening Tech pick unmade, since the harness makes it so it can
drive turns and the button could not otherwise be photographed.

---

## 4. The Core Module: how a Colony or a station is founded

*Ticket [#164](https://github.com/whaleyjoshua2/Dying-Earth/issues/164). The version's one rules
change, decided before it was built and measured before and after.*

Every Colony and every Space Station is founded with a **Core Module**:

- It **holds four Colonists**, a flat four. Neither Expanded Habitats nor the Arkwrights' capacity
  multiplier reaches it; the Habitat stays the thing those rules improve.
- It **draws 1 Energy a turn**, is **never ordered, never mothballed, never decommissioned**, and is
  **never shut for want of Energy**: it is the walls of the place rather than a building in it.
- It **stands outside the Module count**, as the Archive does.
- It **replaced the free Habitat** a ground Colony was founded with, so a Colony Ship's four fit it
  exactly and a Habitat is the first thing anyone builds.

**The base allowance drops from three to zero.** A place's Module cap is exactly the number of
Colonists living there, so **a place nobody lives in builds nothing**. A Colony founded with four has
four slots; a station built this turn has none.

**This ends a deadlock that had been in the game since stations existed.** A bare station needed a
Habitat to hold anybody and Colonists to earn the slot to build one. A station is settleable the turn
it stands, because its Core Module holds four.

**Twenty-one tests were repinned** to the new truth, each with its reason beside it, and the test
helper founds its Colonies with a Core Module as the game does.

---

## 5. The game opens full size, and the Climate Panel opens tall enough to read

*Ticket [#165](https://github.com/whaleyjoshua2/Dying-Earth/issues/165).*

The game opens **maximised**, filling the screen with its title bar and buttons where they can be
found; borderless fullscreen was offered and passed over, since a game people leave and come back to
should not seize the display. A `shot:` run is never maximised: it wants the exact off-screen size it
was given.

**The Climate Panel opens at the top, under the bar, and as tall as the window leaves room for.** It
opened four hundred rows tall and four hundred rows off the bottom -- about a third of its thousand
rows of content, the whole Blame block below the fold before the player touched it. Maximised it now
shows everything; at 1280x800 it reaches the Stabilization line and scrolls for the rest.

**The Core Module leaves the lists.** Every place has one, so naming it said nothing about this one:
it is out of the station list on a Surface Map and out of the roster's Module count, which now matches
the card's. *"A bare core module"*, the placeholder those station lines have shown since version 0.04,
is literally true again.

---

## 6. A population history, and a date on every chart's axis

*Tickets [#166](https://github.com/whaleyjoshua2/Dying-Earth/issues/166).*

The top bar's Population figure draws a history on its hover and on the Climate Panel: **two lines on
two scales**, Earth read against the left and space against the right, each with its own ends written
small at its own side. One scale would not do -- Earth is counted in the hundreds of units and space
in single figures, so a shared axis lays the space line flat for the whole game. The Breaks are
ticked red on the time axis, because the heat is what takes the people. The hover's list of Regions
is cut to the four largest and a count of the rest.

**A chart's time axis carries the in-game date, never the turn number**, at the designer's word, *"a
rule for all charts."* All three read `January 2030` to `September 2032`, the form the top bar has
always used. The Break ticks were shortened and the plot floor lifted in the same pass, since a tick
landing near the last turn sat on the label.

**Two pieces of engine work.** The per-turn record carries **Earth's people and space's**, and **the
record is written later in the Climate phase**: it was pushed before the heat had taken its losses and
the Refugees had moved, so a population read there would have lagged a turn behind the Emissions and
Temperature on the same record. Nothing else on the record moves in between.

---

## 7. A symbol for each Faction

*Tickets [#167](https://github.com/whaleyjoshua2/Dying-Earth/issues/167) (research) and
[#168](https://github.com/whaleyjoshua2/Dying-Earth/issues/168).*

Forty-eight candidates were found, measured at the sizes a card draws and checked against every
picture the board already wears; the findings are in
[`docs/research/faction-symbols.md`](../research/faction-symbols.md) on its own branch. The designer
chose four off the sheets:

| Faction | symbol | author |
|---|---|---|
| Custodians | **Ecology**, two hands cupped under a globe | Delapouite |
| Prospectors | **Mining Helmet** | Delapouite |
| Arkwrights | **Moon Orbit**, a ringed disc | Delapouite |
| Archivists | **CPU** | Delapouite |

All four are game-icons.net under CC BY 3.0, the licence the game already carries, and the author was
already credited, so the Credits screen gains four rows and no new name. None collides with a picture
the board draws: `miner` and `war-pick` are the Mine Module's, `open-book` is the Archive Module's.

**The symbol stands where the colour swatch stood**, drawn in the Faction's own colour at **28
pixels** -- the research measured that 28 is the size and not the swatch's 24 -- so the card says
which Faction and which colour in one mark and grows by nothing. **The symbols appear on the Faction
screen and nowhere else.**

**One standing rule is bent here and nowhere else.** An icon's colour is decided in `icons::fill` and
never by its caller, because a colour on this board means *whose*; a Faction symbol is exactly a
statement of whose, so the card picks the tint, with the reason written where the bending happens.

---

## 8. A quick tutorial, played as the Custodians

*Ticket [#169](https://github.com/whaleyjoshua2/Dying-Earth/issues/169).*

The game's first tutorial, against the standing design that said *no tutorial beyond the first
Report*. The designer chose **a guided free game** over a scripted step panel: nothing is forced and
nothing is checked. It is an ordinary game on an ordinary board, and a **tutorial note** opens each of
its first five turns to say what that turn is for -- what the Custodians are for, read the board,
build a Scrubber, spend your Influence, get off Earth.

A **Tutorial button on the title screen**, above New Game, starts it in one click: as the Custodians
at their own home, the Faction and the start not asked for. The last note says it is the last, the
tutorial ends itself, and the game carries on with nothing thrown away. **Stateless**: the game does
not remember it has been played and the button never changes.

A note is drawn the way a Moment is drawn, because it is the same thing to the player: the turn stops
for one short thought. It leads the turn's popups and hands on to the Event, the Moments and the
Report. Every word lives in **`assets/data/tutorial.toml`**, and the turn a note opens is a field on
the note rather than its place in the file.

**Written knowing the seat is the hard one.** Over eighty computer-played games the Custodian seat
wins 3 in 20 and collapses the world in 11, the worst record of the four. It is the seat that teaches
the climate, which is why the designer chose it; the notes do not promise an easy win.

---

## 9. Builder's calls, for the designer to veto

- The Core Module wears the station glyph the game drew for itself on ticket #135; a ground Colony's
  wears it too, for want of a drawing of its own.
- The tutorial's start is the Custodians' own home state, since the Tutorial button asks for nothing.
- The window size and the start globe's period stay numbers in the code; `assets/data` has no
  view-tuning table and two numbers do not earn one.
- The population chart's colours: Earth in the population glyph's blue, space in off-white.

---

## 10. Building aids added or changed this version

- **`pick:0`** leaves the opening Tech pick unmade, so the `Pick a Tech` button can be photographed.
- **`tutorial:<turn>`** puts that tutorial note in the Report picture's place.
- **`hab:1`** selects seat 0's first station or Colony rather than opening a window, and
  **`habtile:`** stands on its own.

---

## 11. What was measured, and the baseline restarts here

- **The suite is 254 tests**, clippy clean with `-D warnings`. Twenty-one were repinned by the Core
  Module, each with its reason written beside it.
- **The Core Module was swept before and after**, twenty seeds in each of four seatings. The before
  table reproduced the standing baseline to the cell, which confirmed nothing earlier in the version
  had touched the rules.

**The baseline restarts here.** The Core Module changes how every Colony and station begins; nothing
measured before it is comparable to what follows.

---

## 12. The kit

The **Windows kit** only: `dying-earth.exe` built with a statically linked CRT, `assets/`, and the
playtest note as `README.txt`, zipped into `dist/dying-earth-0.07.5-playtest.zip`. `dist/` is
gitignored, so the kit is an artifact on the machine and not a commit.

---

## 13. What is left open

- **The balance version.** The Core Module moved the Factions a long way and the designer kept it,
  expecting later work to move them again. It is the fifth version to carry this.
- **Whether the Solar System Map's station rings should move** as the Surface Map's do.
- **A Faction symbol anywhere but the Faction cards.**
- **India and the United States the same green**, **the start globe under the side panel**, **French
  Guiana with the European Union**, **whether a Colony's card should keep its Influence controls at
  all**: as earlier versions left them.
