# Dying Earth — version 0.07.3, the depth version: a Hab View, build slots as boxes, orbits drawn, people counted in real numbers, and a Ducat from every Region

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.07.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/131), and the
pictures that decided most of it are in
[`docs/dev-diary/2026-09-12-version-0.07.3/`](../dev-diary/2026-09-12-version-0.07.3/).

**What the version is.** Version 0.07.2 redrew the board. This one gives the places on it depth --
a window of tiles for every station and Colony, a Region's build slots drawn as boxes with the
buildings standing in them and the drowned ones under water, every Orbital Slot drawn as its own
orbit -- counts people in real numbers, on the cards and the top bar, and takes **four balance
changes** the designer named: a Ducat from every Region, Habitat yields traded for Research yields,
Emigrants lifted straight to a station over Earth, and the Tech prices rounded. Eight smaller
tidyings ride with it. Every balance change was measured by a twenty-seed sweep before it was final,
and for the first time every one of the four Factions wins games in the sweep. **The sweep baseline
restarts at the end** (section 18).

---

## 1. Resource glyphs on the start screen

*Ticket [#132](https://github.com/whaleyjoshua2/Dying-Earth/issues/132).*

The Faction cards' Multipliers line is a **compact glyph row**, `Output x1 · [emissions] x0.75 ·
[research] x1.25 · [influence] x1.2`, every multiplier shown (x1 included) so the same glyph sits in
the same place on all four cards, the phrase each glyph replaced on its hover. A line may **mix words
and glyphs**: only the eight Figures have glyphs, and a Habitat or a Ship is a piece and stays a
word. Every price in the Signature and Victory paragraphs is glyphed by the existing rule; `12
Colonists` stays words, Colonists being pieces and not the population figure. The start globe's
Region panel reads `leans [materials]` and carries a second line of the three figures a start is
chosen on: Influence value, Ducats a turn and Emissions, from new `Tables::start_ducats` and
`Tables::start_emissions`, since no game exists yet on that screen. **The Figure rule gains one
clause**: a word becomes a glyph directly after a number, *or where it heads a multiplier*.

---

## 2. The Tech Tree transposed

*Ticket [#133](https://github.com/whaleyjoshua2/Dying-Earth/issues/133).*

One row per branch, one column per rung, the branch names as row headings on the left, so time runs
left to right. Two Techs on one rung of one branch sit **side by side** (stacked, the tree would be
seven rows and not fit under the top bar). No rung headings. **The prerequisite lines are elbowed**:
a line leaves the needed box, runs along a lane in the gap before the needing box's column, and
enters the needing box's left edge, never crossing a box; each source row has its own lane, so lines
into one column stay distinguishable, and a line whose source has a neighbour beside it leaves the
box's bottom and runs along the row gap first. The effect stays on hover; boxes are unchanged.

---

## 3. Defence gives way to Max

*Ticket [#134](https://github.com/whaleyjoshua2/Dying-Earth/issues/134).*

**Max** stands exactly where Defence stood in the Command Cluster. One press places one order
spending everything left of the turn's Allotment on the selected place; with nothing selected it is
greyed out, with the hover *Click a Region or a Colony to spend on it*. **`every turn` is Max's
standing order**: ticking it remembers the place selected at that moment (`every turn on China`) and
places the whole Allotment there at the start of every turn as an ordinary pending order, until the
box is unticked, **the placed order is cancelled**, or the place is no longer the player's, which the
Report says. The order behind it is `SetMaxStanding { target: Option<place> }`.

**The computer players lose the Defence rule too.** They are back on ticket #75's holding arithmetic
(once a rival's Standing comes within two steps of their own, push as many holds as it takes to stand
two steps clear of the rival plus the challenge margin). `defence_needs` and `defence_split` leave
the engine with their tests. **Defence is a retired term** in `CONTEXT.md`, kept with its avoid-list.

Measured: Custodians 70 of 80 and Collapses 8 before and after; the tree completed 75 where 73.

---

## 4. The station glyph is replaced

*Ticket [#135](https://github.com/whaleyjoshua2/Dying-Earth/issues/135).*

The station's Kind Glyph is **a drawing of the game's own**: two solar panels on a bar with a central
module, the ISS reduced to its silhouette, chosen off a second candidate sheet after the designer
passed on every station game-icons.net has (none is drawn as a station). It ships as
`assets/icons/station.svg` and loads and tints through `Icons` like every other glyph. Delapouite's
*Defense Satellite* leaves the tree and the Credits screen, which gains a `DRAWN` list beside
`CREDITS` and names the station as *drawn for Dying Earth, no credit owed*; the credits test checks
that every SVG is credited or drawn, never both.

The Region glyph stays on the map labels: the designer kept it while the ticket was open (map, Out of
scope).

---

## 5. Orbital Slots drawn as orbits

*Ticket [#136](https://github.com/whaleyjoshua2/Dying-Earth/issues/136).*

**On a Body Surface Map, one ring per Orbital Slot round the globe**, a step further out and a little
more inclined than the last, the part behind the globe not drawn. A built station wears its glyph at
a fixed point on its ring in its holder's colour, with its name beneath, and is clickable; an empty
slot is a **dashed ring**; a warship blockading the slot is drawn beside the station's place in its
Faction's colour -- the first time Blockade has been visible on a map. The rings live in the
**camera's frame, not the globe's** (an orbit is not fixed to the ground, and an equatorial ring is
edge-on from where the camera sits), a refinement the first picture forced.

**On the Solar System Map, one orbit per Body**, dashed while nothing is in it, carrying the station
glyphs at spaced positions: five rings will not fit round an eighteen-pixel Earth without swallowing
the Moon. **The Body's label lists every Orbital Slot by name and holder while the Body is under the
pointer** (`ISS: Custodians · Tiangong: Prospectors · … · Starlab: free`); always open, Earth's six
lines lay over the Moon, so the resting label keeps its count.

---

## 6. The Nation card's Influence paragraph becomes a hover

*Ticket [#137](https://github.com/whaleyjoshua2/Dying-Earth/issues/137).*

One line stays: `Standings: Custodians 36 · Threshold 50`. The two sentences that explained it are a
hover on the whole line, one sentence and a number per case: *A rival needs 56: your Standing plus
20, and at least the threshold. Decays 1 a turn.* / *You need 56: their Standing plus 20, and at
least your threshold. Decays 2 a turn.* / *First to 50 takes it. Decays 2 a turn.* Every figure is
the engine's own. The Colony's card goes through the same function and follows; the red within-reach
warning and the Blame note stay.

---

## 7. Mothball and Decommission on the line, and the Army's shield inside its button

*Ticket [#138](https://github.com/whaleyjoshua2/Dying-Earth/issues/138).*

A building's Mothball and Decommission (Restart, once mothballed) ride **right-aligned on its own
line**, after its figures, and drop beneath only when the panel is too narrow to hold both; a
Colony's Module rows the same. **The Army's shield rides inside its roster button**: every other
kind's glyph rode inside the row's button through egui's image-and-text button, and the drawn shield
sat beside it; the designer saw the difference, and the Army's row is now a hand-drawn button with the
shield painted inside.

---

## 8. The Ducats formula: every Region pays something

*Ticket [#139](https://github.com/whaleyjoshua2/Dying-Earth/issues/139).*

A controlled Region's economy pays **GDP x Industry Level / 5, rounded down, never below 1** (times
the Prospectors' 1.2), in `Tables::base_ducats`, one line the Region card and the start panel both
read. Under the old `/ 10, rounded down`, ten of the fourteen Regions paid nothing at the start and
eighteen Ducats a turn left the table; now the United States pays 13, the European Union 12, China
10, Japan 3 and every other Region 1, forty-eight a turn in all. A small economy pays a flat one until
GDP x Industry reaches 10.

Measured against the baseline (70 / 8 / 73) and a gentler rule (`/ 10 rounded up`: 68 / 10 / 75):
Custodians 62, Collapses **14**, tree 76 of 80, five of the six new Collapses in the seating where a
richer computer-Prospector in China builds and emits more. The designer kept the rule -- *"looks most
balanced yet"* -- and the Collapse rise is written on the map for the balance version.

---

## 9. Habitat yields become Research yields

*Ticket [#140](https://github.com/whaleyjoshua2/Dying-Earth/issues/140).*

The fourth yield of every Body and every Colony Slot is **Research**: `research_yield` in
`bodies.toml` (Earth 1.0, the Moon 1.0, Mars 1.3, Venus 1.3, Phobos 0.8, Deimos 0.8), drawn per slot
near the Body's as the other three are, worn on the slot label with the Research glyph. It multiplies
an Observatory's output: its slot's figure on the ground, **its Body's on a station**
(`Game::research_yield_at`, the first Body yield a station has read). **A Habitat holds eight
everywhere** (ten with Expanded Habitats, half again for the Arkwrights); Mars lost its half-again
room and the moons their halving. The computer players multiply their Observatory weight by the
yield, and land by Energy where they landed by room. A Trade Post reads nothing.

Measured, cumulative: Custodians 72, Collapses 6, tree 62 of 80. In 42 of 80 games nobody built an
Observatory; the larger effect is the flat Habitat, fewer people off Earth for everyone. Kept.

---

## 10. Emigrants lifted straight to a station over Earth

*Ticket [#141](https://github.com/whaleyjoshua2/Dying-Earth/issues/141).*

`Order::LiftToStation { state, n, colony }`, on the Region's card as `Send N to ISS over Earth by
lift` under the muster, one button per station of the Faction's over Earth with Habitat room, for as
many as it has room for. It **wants a working Launch Site** and **is a launch** (it counts toward the
seat's launches, so it emits and goes on Blame); **no crowd**; the Faction's own stations only; a
warship blockading the station's slot refuses it; the Emigrants are aboard at that turn's Resolution
and the Report says so. One `emigrants_leaving` count, shared with the sea order, keeps the same
waiting people from being sent by sea and by lift both. The computer players lift to their own
station over Earth whenever it has room, at the unload's full weight. The lift lowers no Unrest of
its own, since the sea does not either: the muster does.

Measured, cumulative: **Custodians 47, Archivists 15, Arkwrights 8, Prospectors 1**, Collapses 9,
tree 67 of 80, Colonists off Earth across the eighty games nearly doubled (2,361 to 4,326). The
Archivists win ten of their own twenty where they won none before this version. Kept: *"q9 a."*

---

## 11. Research costs rounded to the nearest five, and Coastal Engineering 12

*Ticket [#142](https://github.com/whaleyjoshua2/Dying-Earth/issues/142).*

Charted as another tenth rounded up (20 / 35 / 50, a tree of 587), and redirected by the designer
after the balance changes before it: *"with everything so balanced let's make coastal engineering 12
and just round everything else to the nearest 5."* No tenth is added. Rung 1 **16 to 15**, rung 2
**28 to 30**, rung 3 **44 to 45**, Coastal Engineering **11 to 12**; the tree **495 to 507**. The
four Victory gates rise with their rung to 45.

Measured, cumulative: Custodians 40, Archivists 12, Arkwrights 12, Prospectors 3, Collapses 12, one
draw, tree 66 of 80 -- the lift's shape, within the noise of twenty seeds.

---

## 12. Population in real numbers

*Ticket [#143](https://github.com/whaleyjoshua2/Dying-Earth/issues/143).*

**The unit of population is five million people** (`Game::PEOPLE_PER_UNIT`). Every Region's figure
is twenty times what it was -- India 388, China 288, the Nigeria Region 228, Indonesia 136, the
European Union 120, Brazil 90, the United States 76, Egypt 52, Iran 50, Mexico 44, Japan 40, Russia
30, Saudi Arabia 20, Australia 10; 1,572 units, 7.86 billion -- and every rule that reads it is
rescaled by twenty and none retuned: the per-person Emissions coefficients (0.002 and 0.0015 per
unit; the cards still quote the rate per hundred million, `Game::UNITS_PER_HUNDRED_MILLION` being
twenty), the Scrubber cap (one per forty units), the Lab's population factor (a thousand units where
fifty hundred-millions stood). **One Emigrant is one unit**, five million, half the ten million it
took before; Steerage two.

**The card reads `Region population 228.0 (1.14B)`** -- one decimal, the real number in millions or
billions past a thousand, and the word *Region* because the figure is the whole territory's: the
designer read 11.4 as Nigeria's 1.14 billion, and since 0.07.2 named Regions for their Nations the
plain word invited that. The data was right. The start panel says the same, with a hover.

**The top bar carries `Earth 7.86B · Space 20M`** at its right end in the population glyph's fill,
with a hover listing every Region and every Body: Earth is the Regions' figures and the Colonists in
Antarctica; space is every Colonist living in Habitats off Earth, a station over Earth counting as
off, as Off-world Presence counts it. People aboard a Ship are not yet anywhere and are not counted.

Measured, cumulative: Custodians 36, Archivists 21, Arkwrights 6, Prospectors 1, Collapses 14, two
draws, tree 68 of 80. The one substantive change is the halved Emigrant drain; the Archivists-
Arkwrights swing is written for the balance version.

---

## 13. The Hab View

*Ticket [#145](https://github.com/whaleyjoshua2/Dying-Earth/issues/145).*

**A popup window** showing a station's or Colony's Modules as **five columns of tiles**: one per
Module with its picture and name, dimmed with the word while mothballed, hatched with the word while
building, a dashed empty tile for every free place under the Module cap, and the Archive on a row of
its own outside the count. **The Habitat wears the Colony's own dome glyph**; every other kind wears
a picture from the building-art research (section 15), off-white, since on this board a colour says
whose. **A click on a tile puts that Module's figures and its Mothball / Restart / Decommission
buttons in a strip under the grid; a click on a free tile puts the build buttons there.** Opened from
the card's `Modules (M)` button, by clicking a station's or Colony's roster row, or with `M` on the
selected place; closed with Esc or its cross, and Esc closes it before it leaves a Surface Map. The
card keeps its summary line (`Modules: Habitat, Shipyard`), the button, and its own build buttons,
drawn through the same function as the strip; its Module rows have moved into the strip. A spectator
opens a rival's Hab View and finds no buttons. Named after Terra Invicta's habitat screen, the
designer's reference; a `CONTEXT.md` entry.

---

## 14. Build slots drawn as boxes on the Nation card

*Ticket [#146](https://github.com/whaleyjoshua2/Dying-Earth/issues/146).*

A Region's build slots are **boxes in the Hab View's language**, drawn by the same tile function with
one more parameter: coastal boxes first, edged in the coast's blue -- standing, building, free, then
the ones the sea has taken -- and inland boxes after, edged in grey; six to a row. **A flooded box is
under water**, at the designer's word: three quarters of it under translucent blue, the top edge a
sine of two billows with a bright crest and a fainter one below, the drowned building dimmed beneath,
*lost to the sea* in the corner. A click on a box puts that Facility's line (figures, hover and the
two buttons) in a strip beneath; a click on a free box puts the build buttons there, drawn by the
same function as the Build section. The Facilities that take no slot -- the Sea Wall, the Scrubber --
keep their rows beneath. The `Coastal:` / `Inland:` word-rows and the `N coastal slot(s) lost to the
sea` line are gone; the boxes say all of it.

---

## 15. The building art

*Ticket [#144](https://github.com/whaleyjoshua2/Dying-Earth/issues/144) (research), chosen on #145 and
#146.*

Twenty-two kinds want a picture. game-icons.net (CC BY 3.0) covers all of them with sixty-two
candidates, measured on sheets at 64, 48, 32 and 28 pixels
(`docs/research/building-art.md`). The pictures in use are the first candidate of each kind except
where the research warned: Lorc's *Mining* for the Mine (*Gold Mine* collides with the Materials'
cart), a control tower for the Launch Site (the shuttle is the Colony Ship's rocket family), a
handshake for the Embassy (the capitol is the Bank's building twice), handcuffs for the Constabulary
(the badge is the Army's shield shape), a fan for the Scrubber (the gas mask says poison); the
Refinery shares one file; the Habitat wears the Colony's dome. Twenty new files in `assets/icons/`,
each credited on the Credits screen, which is two columns now. Three authors are new to it: Skoll,
Sbed and Lorc's Round Bottom Flask.

---

## 16. Builder's calls, for the designer to veto

Collected from the tickets; each is written beside its ticket's answer.

- **The Figure rule bends once**, where a glyph heads a multiplier (#132).
- **Each source row takes its own lane** in the Tech Tree's gaps, and a line with a neighbour in the
  way leaves the box's bottom first (#133).
- **Max is greyed with the hint when nothing is selected**; the `every turn` label names its place (#134).
- **A drawn glyph is listed on the Credits screen** as owing no credit, so the list stays complete (#135).
- **The orbit rings live in the camera's frame**, and **the slot list opens on hover** (#136).
- **The threshold hover hangs on the whole Standings line** (#137).
- **The buttons drop beneath their line** only below 196 pixels of room (#138).
- **The Region card's `GDP … / 5, never below 1` wording** on the line and the hover (#139).
- **The computer lands by Energy** where it landed by Habitat room (#140).
- **The lift lowers no Unrest of its own**, as the sea does not (#141).
- **The card's per-person rates stay quoted per hundred million**, twenty units (#143).
- **The first free tile stands for a free-slot click** in the Hab View and among the boxes (#145, #146).
- **The old "coastal slots lost to the sea" line is gone**, the water saying it (#146).

---

## 17. Building aids added or changed this version

None are part of the game; each exists because something could not otherwise be looked at.

| aid | what it does | why it had to exist |
|---|---|---|
| `attend:1` | now turns the standing Max order on for seat 0, on its start state | Defence is gone; the roster ring still wants photographing filled |
| `room:1` | seat 0's station over Earth has a Habitat on turn 1 | the lift button needs room to offer |
| `hab:1`, `habtile:<n>` / `habtile:free` | the Hab View open on seat 0's first station or Colony, with a tile clicked | a window and a click cannot be made in a headless picture |
| `slotbox:<n>` / `slotbox:free` | a slot box clicked on the selected Region's card | the strip beneath the boxes |
| `walls:1` | (existing) two of China's slots drowned and a Sea Wall standing | the water |

---

## 18. What was measured, and the sweep baseline restarts here

- **The suite is 254 tests**, clippy clean with `-D warnings`. Two tests are new (the lift; the
  Research yield on a slot and a station), two left with the Defence rule, and twenty-odd were
  repinned to the new unit, the new divisor, the new yields and the new costs, each with its reason
  written beside it.
- **Every balance change was swept before it was final**, twenty seeds in each of four seatings on
  the standing instrument `simulate:<seed> --player=<faction>`, and the cumulative table is in each
  ticket and in the dev diary.
- **A whole game was played from seat 0 headlessly** through `cargo run -p dying-earth-engine
  --example play`, seed 11, Custodians from Europe, giving no order but the owed Tech picks: 23 turn
  commands, **none refused**, one pick owed (the opening free choice; the computer Leads picked the
  rest), and the game over on turn 23 with **the Archivists winning on their Victory Condition**,
  every one of the seventeen Techs done, the world at +2.78 C and four turns from Collapse. The
  0.07.2 run of the same seed reached turn 29 with ten Techs done and nobody winning; the lift to a
  station is the difference. One fault in the `play` example was found and fixed on the way: its
  board printed *you must pick the next Tech* with nothing under it once the tree was complete, and
  a driver that trusted it wrote a `tech` line that could not parse.

**The baseline restarts here.** The unit of population, the Ducats formula, the Body yields, the
Emigrant lift and the Tech prices all moved; nothing measured before this table is comparable to it.

| seat 0 | wins | Collapses | draws | tree completes |
|---|---|---|---|---|
| Custodians | Archivists 6, Custodians 3 | **11** | 0 | 20 of 20 |
| Prospectors | Custodians 16, Archivists 2, Prospectors 1 | 1 | 0 | 20 of 20 |
| Arkwrights | Archivists 8, Arkwrights 6, Custodians 4 | 2 | 0 | 19 of 20 |
| Archivists | Custodians 13, Archivists 5 | 0 | 2 | 9 of 20 |
| **totals** | **Custodians 36**, Archivists 21, Arkwrights 6, Prospectors 1 | **14** | **2** | **68 of 80** |

**For the first time every Faction wins games.** The Custodians are still the most at 36 of 80, from
70; the Archivists win 21 where they won 1; the Arkwrights 6, the Prospectors 1. The Collapses are
14 where they were 8, eleven of them in the Custodian seating. The lift to a station did most of it.

---

## 19. The kit

The **Windows kit** only, at the designer's word: `dying-earth.exe` built with a statically linked
CRT, `assets/` (the twenty new icons among them), and the playtest note as `README.txt`, zipped into
`dist/dying-earth-0.07.3-playtest.zip`. `dist/` is gitignored, so the kit is an artifact on the
machine and not a commit.

---

## 20. What is left open

Carried onto the map as fog, none of it decided here:

- **The balance version**, now with real movement to read: the Custodians at 36, the Archivists at
  21, Collapses at 14 and the Archivists-Arkwrights swing under the five-million unit. The Ducats
  rise, the flat Habitat and the halved Emigrant drain are each written with its sweep.
- **The Region glyph on the map labels**, kept for now.
- **Which building pictures the designer keeps**: first picks, every one swappable.
- **The Solar System Map's slot list opens on hover**; always open, it covers the Moon.
- **Whether a Colony's card also loses its Influence controls**, **India and the United States the
  same green**, **the start globe under the side panel**, **French Guiana with the European Union**
  and **the third computer player in Saudi Arabia**: as 0.07.2 left them.
