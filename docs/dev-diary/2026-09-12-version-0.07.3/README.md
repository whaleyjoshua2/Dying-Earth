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

## Building art: candidate sheets (research, ticket #144)

Sixty-two game-icons.net candidates for the twenty-two building kinds, drawn with the game's own
`resvg` at 64, 48, 32 and 28 pixels and the 28 magnified three times, one sheet per group
(`facilities`, `modules`) and per tint (`untinted`, `offwhite` = the kind fill, `tint1` = the
Custodians' teal, `tint2` = the Prospectors' orange), plus `worn` (the thirteen glyphs already on
the board, as the control for collisions). Row numbers match the table in
[`docs/research/building-art.md`](../../research/building-art.md).
