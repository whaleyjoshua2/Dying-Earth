# Dying Earth — version 0.07.1, the reading-and-reaching version: a glyph for every figure, a corner that holds the controls, and a board that names its own rules

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.07.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/111), and the
pictures that decided almost all of it are in
[`docs/dev-diary/2026-09-12-version-0.07.1/`](../dev-diary/2026-09-12-version-0.07.1/).

**What the version is.** Version 0.07.0 gave the board its first icons and its first real computer
opponents. This one finishes the reading: every figure the game counts now has a glyph, the glyphs
have colours, the maps and the lists wear them, the figures that a rule governs say what the rule is,
and the controls a player reaches for every turn have a corner of their own instead of being scattered
between a top-bar row and the bottom of a scrolling card. One balance change rides with it.

---

## 1. Every figure has a glyph, and the glyph has a fill

*Ticket [#112](https://github.com/whaleyjoshua2/Dying-Earth/issues/112).*

**The three figures that had no icon now have one**, all Delapouite's, all game-icons.net under
CC BY 3.0 and all credited on the Credits screen: **population is a head-and-shoulders bust**
(*Character*), **Influence is a megaphone**, **Emissions is a chimney**. They are used the way the five
resources are: the glyph replaces the word on the top bar, sits beside the words elsewhere, and stands
in for the word inside a tooltip.

**Materials and Ducats swapped their art.** Materials is Delapouite's **Mine Wagon** and Ducats his
**Banknote**. The charting round had warned the designer's instruction might be backwards; the warning
was wrong, and wrong because it read filenames instead of the art. Rendered at the sixteen pixels the
bar draws, Materials was a cluster of gem shards nobody could name and Ducats a dozen stacked coins
that flattened into a mineral.

**All eight figures carry a colour**, decided in `icons::fill` and nowhere else:

| figure | fill |
|---|---|
| Materials | steel grey `168,176,186` |
| Fuel | red `226,88,62` |
| Energy | yellow `245,222,92` |
| Research | cyan `118,206,232` |
| Ducats | green `120,214,150` |
| population | warm tan `220,186,150` |
| Influence | lilac `188,146,236` |
| Emissions | brown `146,110,84` |

The palette was chosen off pictures of the real bar and a real Facility list under four candidates,
then **measured in CIELAB against itself and against the Faction colours**. Two collisions the
swatches hid: Ducats' green sat 12 from population's, inside the range two colours are mistaken for
each other, and the jerrycan shifted *redder* landed 19 from the Prospectors' orange — nearer than the
amber it started as, because amber is at hue 36 degrees and the Prospectors at 28, so a partial shift
red lands on top of them. Population moved to a warm tan and the jerrycan went past them to hue 13.
The worst remaining pair anywhere on the board is Materials against Research at 25.

**An icon's colour belongs to the icon.** Both icon constructors have lost their tint parameter: a
glyph's fill is decided by which figure it is and no call site can pass one. The first build of the
Blame block let a glyph take its line's tint, so the chimney came out teal on the Custodians' line and
orange on the Prospectors' — which quietly made an icon's colour mean *whose* on one screen and *which
figure* on every other.

**Two Faction colours moved to clear the two schemes apart.** The **Arkwrights** darken to
`[0.44, 0.275, 0.66]`, which opens the gap to Influence's lilac from 12 to 30, and the **Archivists**
leave pale blue-white for a **bright crimson** `[0.871, 0.322, 0.439]`.

**And every map label is lifted four tenths toward white before it is drawn.** A Faction's colour
tints its own territory *and* prints the label that sits on top of it, so a dark Faction colour costs
its own labels their legibility: the Archivists went from lightness L\* 83 to 55 and their labels all
but vanished into their own land. The lift answers that at the cause rather than by forbidding dark
Faction colours, and it improves the Prospectors' orange-on-orange, which was poor before this version
and nobody had complained about.

---

## 2. The Emissions glyph follows Emissions, and net Emissions join the top bar

*Ticket [#112](https://github.com/whaleyjoshua2/Dying-Earth/issues/112).*

The glyph goes everywhere Emissions appears: the Climate Panel's block heading, a Nation State card's
Emissions line, **every Facility's and Module's figure** in the two densest lists in the game, **every
ppm figure in the Blame block**, and a **new figure on the top bar** — this turn's net Emissions,
beside the Temperature. Until now the only way to learn whether the world went over or under the
Natural Sink this turn was to open the Climate Panel, and the Temperature moves far too slowly to
answer that question.

**`ppm` means Emissions inside the Blame block and nowhere else**, since four lines above it the same
three letters are the CO2 Stock and a Scrubber's pull on the Natural Sink.

---

## 3. One rule for turning a word into a glyph

*Tickets [#112](https://github.com/whaleyjoshua2/Dying-Earth/issues/112) and
[#116](https://github.com/whaleyjoshua2/Dying-Earth/issues/116).*

**A word is traded for its glyph only where it names a figure — only directly after a number — and a
full stop ends a figure.**

The rule arrived in two halves, and both halves were faults found by looking. Version 0.07.0 swapped a
resource word *anywhere* it stood, which was right for the tooltips it was written for. Pointed at the
Facility list it ate the word out of a building's own **name**: `Research Lab (inland): +2 Research`
came out as `[microscope] Lab (inland): +2 [microscope]`. Narrowed for lists and left alone for prose,
it then read as a **rebus** wherever a resource was a sentence's subject: `Fuel goes on transits` came
out as a jerrycan and a verb. The narrow rule now governs everything, and `draw_with_icons` has no flag.

---

## 4. A Colony Slot's yields wear the glyphs of what each Module makes

*Ticket [#113](https://github.com/whaleyjoshua2/Dying-Earth/issues/113).*

`M 1.22 G 0.80 R 1.86 H 1.63` — four capitals a player was never taught and could not look up on the
map — is now four glyphs and four figures. **No new art was needed**: the four are Modules rather than
resources, but the mapping is one to one, since a Mine makes Materials, a Generator Energy, a Refinery
Fuel and a Habitat people.

**The yield line alone is drawn at 14 points**, not the 11 the rest of a map label uses. At 11 the bolt
and the bust read, the mine cart is a blob and the jerrycan is mud; the fault is the size and the size
was ours. **It costs about forty per cent more width** than the capitals — charting expected the
opposite and was wrong — and nothing overlaps at that width on any Body.

**Where a glyph has not loaded the whole line falls back to the capitals.** A map label has no tooltip
behind it, so it must never be able to go mute; this was witnessed by moving `assets/icons` off disk
and watching the capitals come back.

Nothing else on any map was touched.

---

## 5. The roster is an index with a count on each heading

*Ticket [#115](https://github.com/whaleyjoshua2/Dying-Earth/issues/115).*

The designer's line *"organize liners"* meant the roster.

**It stays an index in a stable order**, and each heading now carries **the number of rows that still
want an order**, clickable to filter that group to them. The filter is off by default and works one
group at a time: a panel that re-sorts itself every turn is disorienting, and a panel that never says
what is outstanding makes you read twelve rows to find one.

**An Army carries no "no order" mark.** An Army keeps the stance it was last given until it is moved or
given another — the engine already worked that way and **nothing guarded it**, so a test does now — so
an Army standing where it was put is attended by definition, and the mark had been firing on twelve
rows out of twelve, which says exactly as much as firing on none. The row shows **the stance it is
parked in** instead.

**Army rows are sorted by place and shed the words the heading already says**:
`Standing Army at East Asia: strength 4, damage 0  - no order` is now `East Asia: strength 4  (Hold)`.
Ten of twelve rows wrapped to two lines before; none do now.

**Colonies and Nation States take the mark**, counting this turn's orders alone, so a Colony with a
Module three turns from done reads as attended.

---

## 6. A command cluster in the lower right

*Ticket [#114](https://github.com/whaleyjoshua2/Dying-Earth/issues/114).*

A strip along the foot of the side panel that **never scrolls away**: the Influence still unspent at a
size that can be read across the room, **Spend** on whatever place is selected, **Defence**, and **End
Turn**, which leaves the top-bar row so there is one of it and it is where a hand already is. A
spectator has no cluster, having no orders, and keeps End Turn beside the Auto box.

**There is no dropdown of countries.** Twelve Nation States plus every Colony and station is a long
list to open for one number, and the game already has a way of naming a place: click it.

**"Facilities" meant places.** A Facility is a building inside a Nation State and Influence is never
spent on one. Defence covers **every place held** — Nation States, Colonies and stations — since all
three are taken and decay the same way.

**Influence is a Nation State card's first business now, not its last.** It used to sit under the
Facility list, the Army orders and two paragraphs of help, which is below the fold on every card with
more than a few buildings.

---

## 7. What Defence splits by

*Ticket [#114](https://github.com/whaleyjoshua2/Dying-Earth/issues/114).*

A rival takes a place you hold when their Standing reaches **both** their own threshold **and** your
Standing plus the challenge margin. `Game::defence_needs` therefore counts, for each held place:

- the shortfall to out-stand the best rival by one,
- **plus the decay** a held place takes at every Resolution, so the spend still holds after it,
- and **nothing at all** where the best rival is below their own threshold, since such a rival cannot
  take the place at any Standing.

It does **not** guess what a rival will spend this turn. It assumes their Standing stays put, which
makes the figure a floor rather than a promise, and is the honest assumption: a rival's Allotment is
not something the player can see.

`Game::defence_split` then spends the budget **most threatened first and whole**. Taking a place is a
threshold and not a race, so a place funded most of the way is exactly as lost as one funded not at
all; no part of the budget reaches the second place until the first is safe. Where the budget cannot
cover the next place in full it walks **past** it to ones it can still save.

**"Every turn" is a standing order, not an automatic spend.** It is a pending order that sets a seat
flag, so it survives a save the way the Venture share and the Archive's funding do; each turn the
interface then places that turn's split as **ordinary orders that can be read and cancelled** before
ending the turn, and it stands aside on any turn Influence has already been placed by hand.

**The computer defends by the same rule.** Its own arithmetic ignored the threshold arm, so it spent on
places nobody could take, and ignored decay, so it stopped a point short. A rule belongs in the engine
and not in one caller; what stays the AI's own is the weighting that decides how much of the offered
spend it can afford.

---

## 8. The board names the rules behind its figures

*Ticket [#116](https://github.com/whaleyjoshua2/Dying-Earth/issues/116).*

**The rule: anything showing a bare number that a rule governs earns a tooltip naming the rule.** Not
restating the number, which is already on screen — naming what sets it, what it does at its thresholds,
and what happens when it runs out. A figure with no rule behind it earns nothing; a rule with no figure
on screen belongs in the prose. **The ceiling is six lines**, the length the Income breakdown already
proves works, and the first Unrest tooltip ran to ten and was cut.

Nine figures gained one: build slots, Unrest, Standing, every Facility figure, a Colony's Module cap, a
Ship stack's tank and what stranded means, an Army's stance, and a Body's orbital slots. Hovers went
from **27 to 35** against 231 labels. The Research shortlist and the Report's lines were deliberately
left for a version that looks at those screens properly.

---

## 9. Research costs a tenth more

*Ticket [#117](https://github.com/whaleyjoshua2/Dying-Earth/issues/117).* The version's one balance
change.

A **flat tenth, rounded to the nearest**. The rungs go 15, 25, 40 to **16, 28, 44**, and Coastal
Engineering, priced off the rungs, **10 to 11**. The whole tree goes **450 Research to 495**.

**Measured before it was taken**, since the costs are data:

| | before | after |
|---|---|---|
| the whole tree completes | **79 of 80** | **75 of 80** |
| median last turn, by seating | 30, 30, 27, 28 | 32, 33, 28, 28 |

**Research constrains nothing today.** The tree completes in 79 of 80 games with six to nine turns of
the thirty-six left over; it finishes and the game carries on without it. A tenth costs four
completions and adds nought to three turns. **It was taken as a nudge and not a lever**, which is what
it measures as; making finishing the tree a real choice would want something like a third, which is a
different change and a balance version's work.

**The Prospectors' provisional Lab weight of 8 was deliberately not re-fitted.** They lost three wins
in the tenth-more sweep, which is exactly the signal that tempts a re-fit; at twenty seeds that is
inside the noise, and fitting a provisional number against a noisy signal is how a number gets baked in
wrong.

---

## 10. Builder's calls, for the designer to veto

- **`icons::fill` is the only place an icon's colour exists**, and the tint parameter is gone from both
  constructors rather than merely unused. A convention can drift; a missing parameter cannot.
- **The Defence split walks past a place it cannot afford** rather than stopping dead at it, which is
  the one place the build reads *"and stop"* as *"and keep going down the list"*. Same money, more
  places held, and every place it funds is funded to safe.
- **Spend acts on the current selection** and says so when nothing is selected, rather than offering an
  empty control.
- **The old End Turn was removed from the top-bar row** rather than left beside the new one.
- **Three tests that pinned Tech costs were rewritten to pin the new figure beside the relationship
  that was their point** — Coastal Engineering is cheaper than the rung it shares, and no Faction's
  gate is dearer than another's — so the next rebalance breaks a number and not a meaning.
- **A test from ticket #75 was rewritten** rather than deleted or worked around; see section 12.

---

## 11. Building aids added this version

None of these are part of the game; all exist because something could not otherwise be looked at.

| aid | what it does | why it had to exist |
|---|---|---|
| `window:<w>x<h>` | sets the off-screen window's size | the Climate Panel is taller than 800 rows |
| `climate:top` | opens the Climate Panel at the top, full height | the panel settles at ~400 rows and scrolls, so the Blame block is below the fold |
| `roster:filter` | turns every roster filter on | it is otherwise a click |
| `threat:1` | stands a rival within reach of every state seat 0 holds | in a headless run all four seats are the computer, which now defends itself, so nothing is ever under threat |
| `tip:<word>` | shows the first tooltip containing that word without a hover | the window is off-screen and no pointer enters it, so **no tooltip could be photographed at all** |
| `icon_sheet ... <sizes>` | renders a folder of SVGs at named pixel sizes, magnified | a map label draws at 11 where the bar draws at 16 |
| `Techs: n of 17` | the simulation reports how far the tree got | ticket #117 could not otherwise be measured |

---

## 12. A test from ticket #75 was rewritten, and the rewrite is a finding

*Ticket [#114](https://github.com/whaleyjoshua2/Dying-Earth/issues/114).*

It asserted that an AI holder at Standing 30 answers a rival at 25 by spending 25. It was written when
the **challenge margin was 10** — which ticket #75 itself then raised to **20**, at which that rival
needs 50 to take anything. Under the shared Defence rule the holder spends nothing there, and that is
right: the old behaviour was not caution, it was Influence set on fire, out of the same Allotment the
seat takes new places with. The test now guards what it meant to guard — that a holder does not sit
still while a rival walks in — at the Standing where that is actually true.

---

## 13. What was measured

Every figure below is from a run on this branch, on the standing instrument: `simulate:<seed>
--player=<faction>`, seeds 1 to 20, in each of four seatings.

- **Three engine rules gained tests, each witnessed red**: an Army's stance surviving a Resolution
  (`left: Hold, right: Evade` when stances are reset); the Defence need counting the decay
  (`left: 41, right: 42` without it); and the split refusing to part-fund
  (`left: [(EastAsia, 61)], right: [(SouthAsia, 22)]` when it is allowed to).
- **A whole 36-turn game was played from seat 0 headlessly**, through
  `cargo run -p dying-earth-engine --example play`, giving no orders at all: it ran to a Collapse on
  turn 30 with all 17 Techs done and **no turn refused**.
- **The suite is 254 tests**, clippy clean with `-D warnings`.

---

## 14. The sweep at the end of the version

**The figure recorded at the end of version 0.07.0 does not reproduce on the code version 0.07.0
shipped, and that is worth knowing before reading anything below.** Its spec records Custodians
winning **49 of 80** with **16 Collapses**. Built from `main` at the 0.07.0 merge and run on the
standing instrument, that same code gives **Custodians 71 of 80** and **5 Collapses**. The
0.07.0 table was measured on something other than what landed — a different seed range, or a build
that did not merge — and the discrepancy predates this version entirely. **So the comparison here is
against a re-measured `main`, not against that table.**

| seat 0 | `main` (0.07.0 as merged) | version 0.07.1 |
|---|---|---|
| Custodians from Europe | Custodians 13, Prospectors 2, **Collapse 5** | Custodians 12, Prospectors 4, **Collapse 4** |
| Prospectors from North America | Custodians 20 | Custodians 20 |
| Arkwrights from South Asia | Custodians 19, Arkwrights 1 | Custodians 17, Arkwrights 3 |
| Archivists from Sub-Saharan Africa | Custodians 19, Archivists 1 | Custodians 20 |
| **totals** | **Custodians 71**, Prospectors 2, Arkwrights 1, Archivists 1, Collapses 5 | **Custodians 69**, Prospectors 4, Arkwrights 3, Archivists 0, Collapses 4 |

**What moved, honestly.** Custodian wins 71 to 69 and Collapses 5 to 4; Prospector wins 2 to 4 and
Arkwright wins 1 to 3. Every one of those differences is inside what twenty seeds can separate from
noise. The version's play changes were the AI defending by the split rule and Research costing a tenth
more, and **neither shows up as more than a wobble**.

**The Custodians remain the standing imbalance at 69 of 80, and the Archivists win none.** This version
does not address either, deliberately.

---

## 15. The kit

The **Windows kit** only, at the designer's word, as in version 0.07.0: `dying-earth.exe` built with a
statically linked CRT, `assets/`, and `PLAYTEST.txt`, zipped into `dist/`. `dist/` is gitignored, so
the kit is an artifact on the machine and not a commit.

---

## 16. What is left open

Carried onto the map as fog, none of it decided here:

- **Whether Defence should defend *early*.** As built it answers a rival who can take the place *next*
  turn and nobody sooner, so a rival jumping from 40 to 55 in one turn is never answered in time.
  Defending early needs a measure of how fast a rival can move, and the obvious one — their Allotment —
  is not something the player can see, so it is as much a question about what a card tells you as about
  the rule.
- **Research constrains nothing**, measured above. Whether the tree *should* complete every game is a
  balance question and belongs beside the two below.
- **The Archivists win none of 80, the Custodians 69, and the Victory bars are still unfitted.** Named
  at the end of version 0.07.0 and still the first thing a balance version should take.
- **The rest of the side bar.** *"Organize liners"* meant every populated side bar; this version took
  the roster alone.
- **Map crowding and the label plate.** Antarctica's three Colony Slot labels overlap at the default
  zoom — proved to predate the map icons by taking the art away and watching the capitals overlap
  identically — and an Archivist label reads weakest of the four because the plate behind it is black
  at only alpha 170 and that Faction's own pink tint leaks through.
- **Whether two Armies in one place should be one row.**
- **The 0.07.0 sweep discrepancy itself**, section 14. Something measured a table that its own code does
  not produce, and until that is understood no figure recorded against 0.07.0 should be quoted.
