# Version 0.07.3, the depth version

The map is [Map: version 0.07.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/131). Branch
`version-0.07.3`, cut from `main` after version 0.07.2 merged as
[pull request #130](https://github.com/whaleyjoshua2/Dying-Earth/pull/130).

## Resource glyphs on the start screen

Ticket [#132](https://github.com/whaleyjoshua2/Dying-Earth/issues/132). The designer's line: *"Use
resource icons for faction selection cards on start screen and for territory cards when selecting
starting location."* Decided in one round and a half: the Multipliers line as a **compact glyph row**
(option c), every multiplier shown so the four cards line up; a mixed line where a piece has no
glyph; every price in the paragraphs by the existing rule, `12 Colonists` staying words; `leans 🛒`
on the start globe's Region panel, and a second line there of the three figures a start is chosen on.

![The four Faction cards](faction-cards-glyph-rows.png)

**The Multipliers line is a row of glyphs.** `Output x1 · 🏭 x0.75 · 🔬 x1.25 · 📣 x1.2` where it read
`Facility and Module output x1; Emissions from Earth sources it controls x0.75; Research x1.25;
Influence Allotment x1.2`. Every multiplier is drawn, x1 included, so the same glyph sits in the same
place on all four cards. This is the one place the glyph rule bends -- the glyph *heads* a
multiplier instead of following a number -- and `CONTEXT.md`'s **Figure** entry now says so. The
phrase each glyph replaced is on its hover, through `rule_tip` so the `tip:<word>` aid can reach it:

![The hover on a multiplier](faction-card-multiplier-hover.png)

**The Arkwrights' extras are a mixed row**: `Habitat capacity x1.5 · transit ⛽ x0.75 · Colony Ship
capacity x2 · 👤 per lifted Colonist x2 · every Ship x0.85 🛒 …`. Only the eight Figures have glyphs;
a Habitat or a Ship is a piece and stays a word. **The paragraphs** take their prices by the existing
rule: `30 🛒, 2 turns, 4 ⚡ upkeep`, `Leapfrog costs 50 💵`, `750 🛒 in the Venture Capital Fund`, and
`(a Colony Ship 25 🛒)` now that a closing bracket rides on the word like a comma does.

**The first picture caught a layout fault** the suite could not: a row part was a nested
`horizontal` inside a `horizontal_wrapped`, laid out at the cursor and simply overrunning the card's
edge, so the Arkwrights' last part crossed into the Archivists' card. The part is now measured first
and the row broken before it, and a part that opens a new line takes no separator.

![The first try, overrunning](faction-cards-first-try-overrun.png)

**The Region panel on the start globe** reads `👤 Population 14.4 (hundreds of millions) · Industry
Level 3 · leans 🛒`, and beneath it `📣 3 · 💵 5 a turn · 🏭 5.5 · Education Level 1.1`: the Influence
value, the Ducats a turn and the Emissions as the game opens, each with a hover saying what it is.
No game exists yet on that screen, so the engine grew `Tables::start_ducats` and
`Tables::start_emissions`, computed from the cards as turn 1 will compute them -- and the Ducats
formula moved into `Tables::base_ducats`, one place that `Game::state_ducats` and the panel both
read, so the ticket that changes the formula changes one line.

![The Region panel](start-panel-with-figures.png)

![The Emissions hover](start-panel-emissions-hover.png)

Clippy clean with `-D warnings`, 255 tests passing.

## The Tech Tree transposed

Ticket [#133](https://github.com/whaleyjoshua2/Dying-Earth/issues/133). The designer's line: *"Transpose
tech tree."* Decided in one round: branch names as row headings on the left; two Techs on one rung
side by side; no rung headings; elbowed prerequisite lines; the effect staying on hover.

![Before: branches as columns](tech-tree-before.png)

**One row per branch, one column per rung**, so time runs left to right the way a tree is read.
Industry and Society each hold two Techs on one rung; they sit **side by side** and the column widens
for that rung, because stacked the tree would be seven box-rows tall (about 700 pixels) and would
not fit under the top bar, where side by side keeps it five rows and about 640 wide.

![After: branches as rows](tech-tree-transposed.png)

**The lines are elbowed.** A line leaves the needed box, runs along the gap to the left of the
needing box's column, and enters the needing box's left edge, so it never crosses a box; a Tech that
needs one on its own rung (Closed-Loop Colonies needs Clean Power) is reached the same way, out of
the needed box's left edge and down the same gap. Two things the first pictures caught that the
decision had not said: **every line into a column merged into one trunk**, so nobody could tell
which Tech fed which, and **a line from Efficient Grids to Clean Power ran behind Coastal
Engineering**, which then seemed to be the source. Each source row now takes its own lane in the
gap, and a line whose source has a neighbour in the way leaves the box's bottom, runs along the
row gap, and only then climbs the lane.

![The first try: one trunk](tech-tree-first-try-one-trunk.png)

Clippy clean with `-D warnings`.

## Defence gives way to Max

Ticket [#134](https://github.com/whaleyjoshua2/Dying-Earth/issues/134). The designer's line: *"Get rid
of defense button replace with a max spend button that just say Max."* Decided in two rounds: one
press places the order; Max exactly where Defence stood; the computer players lose the rule too and
go back to their own arithmetic; `every turn` survives as Max's standing order, ending on untick, on
the placed order being cancelled, or on the place being lost; Defence retired from the vocabulary.

![Before: the Defence row](command-cluster-defence-before.png)

**Max, in Defence's row.** With a Region or Colony selected and Influence left, one press places one
order spending all of it there; with nothing selected it is greyed out and its hover says to click a
place. The Spend box and button stay above it for a lesser amount.

![After: Max with China selected](command-cluster-max.png)

**`every turn` is now Max's standing order.** Ticking it remembers the place selected at that moment
(`every turn on China`), and at the start of each turn the whole Allotment is placed on that place as
an ordinary pending order, readable and cancellable like any other. It ends when the box is unticked,
**when the player cancels the placed order** (the designer's addition on the ticket), or when the place
is no longer theirs to spend on, which the Report says. The order behind it is `SetMaxStanding`, with
the place inside it, where `SetDefenceStanding` carried a flag.

![The standing order on, at turn 1](command-cluster-max-every-turn.png)

**The computer players lost the rule with the button.** Ticket #114 had given them `defence_needs`,
the same rule the player's button split by; the designer's answer was *"computer players lose it too"*,
and they are back on ticket #75's arithmetic -- once a rival's Standing comes within two steps of
their own, push as many holds as it takes to stand two steps clear of the rival plus the margin.
`defence_needs` and `defence_split` are gone from the engine with their two tests, and the ticket-#114
AI test is the ticket-#75 one again.

**Measured, twenty seeds in each of four seatings, after the change, against the 0.07.2 baseline:**

| seat 0 | wins | Collapses | tree completes | 0.07.2 baseline |
|---|---|---|---|---|
| Custodians | Custodians 12 | 8 | 20 of 20 | Custodians 12, Collapses 8, 20 of 20 |
| Prospectors | Custodians 19, Arkwrights 1 | 0 | 20 of 20 | Custodians 20, 20 of 20 |
| Arkwrights | Custodians 20 | 0 | 15 of 20 | Custodians 19, Arkwrights 1, 13 of 20 |
| Archivists | Custodians 19, Archivists 1 | 0 | 20 of 20 | Custodians 19, Archivists 1, 20 of 20 |
| **totals** | **Custodians 70**, Arkwrights 1, Archivists 1 | **8** | **75 of 80** | Custodians 70, Arkwrights 1, Archivists 1, Collapses 8, 73 of 80 |

Two games in eighty changed hands and the tree completed twice more: the cruder holding rule costs
the computer nothing the sweep can see. The Custodians' seventy stays exactly seventy, which is the
imbalance the map carries as out of scope, untouched. (The instrument is `simulate:<seed>
--player=<faction>` on the dev build; a game runs in under a second.)

Clippy clean with `-D warnings`, 253 tests passing.

## The station glyph is replaced: the candidates

Ticket [#135](https://github.com/whaleyjoshua2/Dying-Earth/issues/135). The designer's line: *"Replace
station icon."* Twelve columns from game-icons.net, rendered by the `icon_sheet` aid at full size and
at 13, 16 and 20 pixels (the map label, the roster row and the card title), left to right:

![Station glyph candidates](station-glyph-candidates.png)

1. **Defense Satellite** (Delapouite), the glyph today, for comparison.
2. **Apollo Capsule** (Delapouite): a capsule at every size, but a capsule is a ship.
3. **Beam Satellite** (Delapouite): the panels dissolve at 13.
4. **Death Star** (Delapouite): the strongest shape at 13, but it reads as a moon with a crater.
5. **Double Ringed Orb** (Lorc): a body inside a tilted ring; survives at 13 and reads as a thing in orbit.
6. **Mars Pathfinder** (Delapouite): a lander on legs, thin at 13.
7. **Moon Orbit** (Delapouite): a planet with a small moon on a ring; clear at 13, mistakable for a Body.
8. **Radar Dish** (Lorc): clear at 13, but a dish on the ground.
9. **Satellite Communication** (Delapouite): the same dish family as today's; the waves go to mush.
10. **Spoutnik** (Lorc): the antennae thin to nothing at 13.
11. **Star Satellites** (Lorc): a ring round a sun, a blur at 13.
12. **Transportation Rings** (Lorc): busy at every small size.

The seven 0.07.2 offered (lunar module, observatory, orbital, satellite, solar system, space needle
and today's) are not repeated; the site has nothing that draws an ISS truss. Recommended to the
designer: **5, the Double Ringed Orb**, the one that reads as a thing in orbit rather than a dish, a
ship or a world, with Lorc already on the Credits screen.

**A second set**, after the designer passed on all twelve: game-icons.net has no station drawn as a
station, so this set comes from four other open sets and from three shapes drawn for the purpose,
which, like the Army's shield, would need no art file and no credit. Left to right:

![Station glyph candidates, second set](station-glyph-candidates-2.png)

1. **Satellite**, Font Awesome Free (CC BY 4.0): a blob at 13.
2. **Orbit**, Lucide (ISC): a ring with two moons; holds at 13.
3. **Satellite**, Lucide (ISC): stroked and thin; fades at 13.
4. **Orbit**, Material Design Icons (Apache 2.0): a ball inside a ring; clear at every size.
5. **Satellite Variant**, Material Design Icons: a satellite with dish and panels; a blob with a tail at 13.
6. **Space Station**, Material Design Icons: **the ISS truss** -- a core with panel pairs either side. Reads as a station at 20 and 16 and as an H-shaped truss at 13. The only icon in any of these sets that draws a station.
7. **Satellite**, Tabler filled (MIT): a blob at 13.
8. **Satellite**, Tabler outline (MIT): stroked; fades at 13.
9. **Drawn: a rotating station**, a ring seen edge-on with an axis rod and a hub; clear at 13.
10. **Drawn: panels**, two solar panels on a bar with a central module; clear at 13, the ISS reduced to its silhouette.
11. **Drawn: a wheel station**, a ring with a hub and four spokes; clear at 13.

Recommended to the designer: **6, Material Design's Space Station**, the one glyph that is the thing
itself, with **10, the drawn panels**, as the runner-up if a silhouette that owes nothing is preferred.

**The designer chose 10, the drawn panels.** Two solar panels on a bar with a central module, the ISS
reduced to its silhouette, drawn for the game in four rectangles and a circle. It ships as
`assets/icons/station.svg` and loads and tints through `Icons` like every other Kind Glyph, so
nothing in the interface changed to wear it; Delapouite's *Defense Satellite* is gone from the tree.
The Credits screen gained a `DRAWN` list beside `CREDITS` -- a glyph that is the game's own owes no
credit but must still exist, and the credits test now checks both -- and names the station as
*drawn for Dying Earth, no credit owed*, so the list of icons on screen stays complete.

![The glyph on the roster](station-glyph-on-the-roster.png)

![The glyph on the Report's lines](station-glyph-in-the-report.png)

![The Credits screen](credits-with-a-drawn-glyph.png)

Clippy clean with `-D warnings`, 253 tests passing.

## Orbital Slots drawn as orbits round their Bodies

Ticket [#136](https://github.com/whaleyjoshua2/Dying-Earth/issues/136). The designer's lines: *"Want to
see icons representative of orbitals orbiting their parent bodies each slot a separate orbit"* and
*"List orbital slots in the body card."* Decided in one round, every answer (a): both maps; an empty
slot a dashed ring; glyphs fixed; tilted rings in the 3D scene; a blockading warship drawn on its
ring; the Solar System Map's label listing every slot by name and holder.

![Earth: three stations on their rings, two empty rings dashed](orbits-round-earth.png)

**One ring per Orbital Slot round the globe** on a Body Surface Map, a step further out and a little
more inclined than the last, the part behind the globe not drawn. A built station wears the drawn
panels glyph at a fixed point on its ring in its holder's colour, with its name beneath, and is
clickable; an empty slot is a dashed ring; a warship blockading a slot is drawn beside the station's
place in its Faction's colour -- the first time Blockade has been visible on a map.

![The Moon: two empty rings](orbits-round-the-moon-empty.png)

**The first picture caught the rings edge-on.** They were drawn in the globe's own equatorial frame,
which from where this camera sits is a line, so five near-vertical lines ran off the screen. The
rings now live in the **camera's frame**, not the globe's: an orbit is not fixed to the ground (the
ISS does not turn with China), and drawn this way every ring is an ellipse round the globe whatever
the globe's spin. That is a refinement of the decision's "turn with the globe", made by the picture.

![The first try: rings edge-on](orbits-first-try-edge-on.png)

**On the Solar System Map** each Body has **one orbit**, dashed while nothing is in it, carrying the
station glyphs at spaced positions -- five rings will not fit round an eighteen-pixel Earth without
swallowing the Moon. **The slot list on the label unfolds while the Body is under the pointer**
(`ISS: Custodians · Tiangong: Prospectors · Axiom: Archivists · Orbital Reef: free · Starlab: free`);
always open, Earth's six lines lay over the Moon and Mars's over its moons in the first picture, so
the resting label keeps its count and the list is one hover away. The `hover:<body>` aid photographs it.

![The Solar System Map with Earth hovered](orbits-on-the-solar-map-earth-hovered.png)

![The first try: the open list over the Moon](orbits-first-try-label-over-the-moon.png)

Clippy clean with `-D warnings`, 253 tests passing.

## The Nation card's Influence paragraph becomes a hover

Ticket [#137](https://github.com/whaleyjoshua2/Dying-Earth/issues/137). The designer's line: *"remove
this language from the nation cards … replace with mouse over that relays the same information in
far fewer words."* Decided in one round: one line stays, the explanation is a hover on the whole
line, one sentence and a number per case, and the Colony's card follows.

![The threshold hover on China's card](threshold-hover-on-the-nation-card.png)

**Two sentences became one line and a hover.** `Threshold 50; a place already held changes hands only
at the holder's Standing plus the challenge margin of 20.` and `Yours. A rival takes it with a
standing above yours and at least the threshold: 70 now. Spending here raises your standing; it
decays 1 a turn.` are now `Standings: Custodians 50 · Threshold 50`, and under the pointer: *A rival
needs 70: your Standing plus 20, and at least the threshold. Decays 1 a turn.* A Region held by a
rival says *You need 70: their Standing plus 20, and at least your threshold. Decays 2 a turn*, and a
neutral one *First to 50 takes it. Decays 2 a turn*. Every figure in the hover is the engine's own,
as before; the red within-reach warning and the Blame note are untouched. The spectator's cards
carry the same line with a hover that names the holder. A Colony's card goes through the same
function, so it follows without a second change.

Clippy clean with `-D warnings`, 253 tests passing.

## Mothball and Decommission on the line, and the Army's shield inside its button

Ticket [#138](https://github.com/whaleyjoshua2/Dying-Earth/issues/138). The designer's lines: *"Mothball
and decommission buttons moved next to facility name not under (after yields and upkeep)"* and *"The
army icon on the cards left of buttons when other icons are not."* Decided in one round: the buttons
right-aligned on the name line, Restart in Mothball's place, the Module rows following -- and the
second line, which the charting had read as a card question, corrected by the designer: *"on roster
every other glyph is a part of the button while armies stands apart."*

![Before: the buttons beneath each Facility](facility-buttons-beneath-before.png)

![After: the buttons on the line, right-aligned](facility-buttons-on-the-line.png)

**The two buttons ride on the building's own line**, right-aligned after its figures, so they make a
column down the card and every row is one line shorter. The line keeps its hover on the figures. The
buttons drop beneath, as they were, only when the panel is too narrow to hold both after the figures
(about 196 pixels' worth); a building under a pending change keeps its weak "ordered, lands at turn
N" line. A Colony's Module rows do the same.

![Before: the shield beside the button](roster-army-shield-outside-before.png)

![After: the shield inside it](roster-army-shield-inside.png)

**The Army's shield is inside its roster button.** Every other kind's glyph is loaded art and rides
inside the row's button through egui's own image-and-text button; the shield is drawn, and had no
way into one, so 0.07.2 drew it beside the button. The designer saw the difference at a glance. The
Army's row is now a hand-drawn button in the button's own visuals -- the shape `priced_button`
already uses -- with the shield painted inside, so it looks and behaves like the rest.

Clippy clean with `-D warnings`, 253 tests passing.

## Building art: candidate sheets (research, ticket #144)

Sixty-two game-icons.net candidates for the twenty-two building kinds, drawn with the game's own
`resvg` at 64, 48, 32 and 28 pixels and the 28 magnified three times, one sheet per group
(`facilities`, `modules`) and per tint (`untinted`, `offwhite` = the kind fill, `tint1` = the
Custodians' teal, `tint2` = the Prospectors' orange), plus `worn` (the thirteen glyphs already on
the board, as the control for collisions). Row numbers match the table in
[`docs/research/building-art.md`](../../research/building-art.md).
