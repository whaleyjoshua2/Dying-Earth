# Version 0.09.8, the Moon race version

The map is [#473](https://github.com/whaleyjoshua2/Dying-Earth/issues/473); the spec is
[`docs/spec/version-0.09.8.md`](../../spec/version-0.09.8.md). One entry per ticket, in the order
they were built.

## Card sub-headers (#474)

The designer: *"for the nation/outpost cards increase the size of each sub-header by 15%, e.g.
Influence, Pioneers, Policies. Influence should share the same white color. add a break between
policies and facilities and like influence I want to see a glyph leading each heading."*

Decided on the ticket: every sub-header on both cards, with the Colony card's Modules and Influence
lines promoted to headings; 14.5 points; white for all; the rule below the Policies block.

Each picture is the 0.09.7 build on the left and this build on the right, the same seed (7), turn 4,
at `window:1280x1500` so the whole card shows.

- **The Region card** (`select:EastAsia turns:3 seed:7`):
  ![the Region card, before and after](ticket-474/region-before-after.png)
- **The Colony card**, the ISS (`hab:1 turns:3 seed:7`):
  ![the Colony card, before and after](ticket-474/outpost-before-after.png)
- **The Archivists' Colony card**, for The Archive's heading (`hab:1 player:archivists archive:1`):
  ![the Archivists' Colony card](ticket-474/archivists-colony-card.png)
- **The headings at four times their size**, to judge the glyphs. Policies' gavel and Orders' list
  are new drawings (`assets/icons/policies.svg`, `orders.svg`):
  ![the headings magnified](ticket-474/headings-magnified.png)

**After the designer looked:** *"policies looks a bit wonky at its size - replace, also we need a line
before the armies section"*. Policies was a scroll first. Four replacements were drawn and previewed
at 17 pixels (a preview drawn by script, not by the game's own renderer); the gavel went into the
build. A rule now stands above the Armies block on both cards.

![four candidates for the Policies glyph](ticket-474/policies-candidates/sheet.png)

Seen in the pictures and left alone:

- The Region card is about 30 pixels taller for the larger headings and the new rule.
- On a station with no Shipyard the Orders heading has nothing under it. It was so before; the
  larger heading makes it plainer.

Not pictured: the Ships heading (no Shipyard on these boards), the Colony card's Armies heading, and
the full-place warning under Modules.

## The population and GDP lines on the cards (#475)

The designer gave both lines word for word. Decided on the ticket: the figures cut from the GDP line
go onto its hover; the Colony card's bracket counts the part-grown next Colonist; the amber and red
stay on the count.

The left of each pair is the build after the sub-headers ticket, the right this build, seed 7, turn 4.

- **The Region card** (`select:EastAsia turns:3 seed:7`):
  ![the Region card's lines, before and after](ticket-475/region-before-after.png)
- **The GDP line's hover** (`tip:Pays`):
  ![the GDP hover](ticket-475/gdp-hover.png)
- **The Colony card**, the ISS (`hab:1`): six Colonists and three tenths of a seventh.
  ![the Colony card's people line, before and after](ticket-475/outpost-before-after.png)

Not pictured: the hover on a rival's Region the player cannot see.

## Raise Industry Level as a tile (#476)

The designer: *"get rid of the raise industry level button, instead add an empty tile that says
raise industry level; when clicked it changes to click to build and procs another such box at the
end."* Decided on the ticket: the rule stands, so the tile is drawn *"just like any building being
built"* until the raise completes; last of the tiles; one click orders it.

China, seed 7, turn 4 (`select:EastAsia turns:3 seed:7`). A new aid, `raise:1`, gives the seat
Materials and places the order.

- **Free, and refused**: the seat holds 21 Materials of the 30, so the tile is greyed and the
  refusal leads its hover (`"tip:Adds an inland"`):
  ![the Raise tile and its hover](ticket-476/hover.png)
- **Ordered** (`raise:1`):
  ![the tile ordered](ticket-476/ordered.png)
- **Its hover while ordered** (`raise:1 "tip:ordered this turn"`):
  ![the ordered tile's hover](ticket-476/ordhover.png)
- **After End Turn** (`raise:1 commit:1`): a fourth free box, and a new Raise tile after it. A card
  of the new turn overlaps the left of the picture.
  ![the tile after the raise completes](ticket-476/done.png)

**After the designer looked:** *"can we shift the color maybe something closer to the mothballed blue
shade or a red version of the same tone"*. Both were built and shot, live and greyed; the designer
took the red. The tile's words and dashes wear it, dimmed where the order would be refused.

![the two colours, affordable and not](ticket-476/colours.png)

- **Live, in the red** (`raise:afford`):
  ![the tile live](ticket-476/live.png)

Not pictured: the raise under way across a turn ("building,
2 of 4"), which needs a Region making fewer than 4 Widgets a turn.

## The tutorial, 10% fewer words (#477)

462 words to 398. The designer approved the seven cuts as a before-and-after table on the ticket,
and declined a tested ceiling: *"nah we need some flexibility here"*.

The four notes that changed, as the game draws them (`menus:1 tutorial:<turn> seed:7`): turns 1 and
2 above, 3 and 5 below.

![the four changed tutorial notes](ticket-477/notes.png)

## The Prospectors' Opening Objective: 50 Ducats, three turns running (#478)

Decided on the ticket: three Incomes in a row, a turn with one Bank not working starts the count
again; still three different Regions; the Journal shows the count.

**Witnessed red:** the new test `the_prospectors_opening_objective_wants_three_turns_running` and
the amended `each_faction_has_an_opening_objective_met_once_and_rewarded`, written before the rule,
both failed with the objective met at the first Income (`left: (Some(1), 0)`). The first build then
failed the new test on its own premise: the Fund grows a little each Income by itself, so the test
now measures the 50 on top of that growth.

**The Journal's count** (`player:prospectors victory:1 journal:1 openingrun:2`; `openingrun:` is a
new aid that sets the count):

![the Journal reading 2 of 3 turns](ticket-478/journal-count.png)

**The sweep** ([`after-478.txt`](sweeps/after-478.txt)): 8 / 19 / 20 / 4 and 29 collapses, from
8 / 21 / 18 / 5 and 28. Within noise. The Prospectors meet their objective in 51 of 80 games, from
58, by seating 20 / 20 / 6 / 5 from 20 / 20 / 9 / 9, at a median turn one or two later.

**The sweep's command and time, for the record:** `cargo run --release -p dying-earth-engine
--example sweep -- 20 --seatings --steps=300 --balance`; the 80 games took 11 seconds, measured.
