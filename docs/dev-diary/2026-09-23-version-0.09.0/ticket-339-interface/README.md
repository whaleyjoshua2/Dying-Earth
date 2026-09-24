# Whole-battle odds, the eye, the Tech Tree edges, and the four adjustments

Ticket [#339](https://github.com/whaleyjoshua2/Dying-Earth/issues/339) on version 0.09.0, the
interface half. The designer's resolution comment on the ticket is the authority. This folder was
written in two passes: first **improvement N and all four adjustments**, then **improvements G and
L** -- the whole-Battle odds on screen, and the Relay or Embassy as an eye -- whose engine halves
were built in commit `6b4c12f` and whose interface halves are the sections that follow the pictures.
Improvements A and B are wholly the engine's and are in that commit.

Every picture below is cropped out of a headless `shot:` capture; the command that produced it is
named beside it. A `-was` picture in the first five rows is that same command run against the code
as it stood at `ad6f553` (prefix `shot:was`), rebuilt for the purpose and put back; a `-was`
picture in the two eye rows is the same build with the eye taken off the board (`eye:0`), which is
the comparison improvement L wants. Nothing was opened on the designer's desktop, and the extra view
captures each run writes were deleted.

## The pictures

| picture | what it shows |
|---|---|
| [`header-was.png`](header-was.png) / [`header-now.png`](header-now.png) | `shot:now settler:mars site:mars,0 seed:7 turns:8 window:1500x1000`, cropped to the middle of the top bar. **Both header adjustments in one strip.** Influence went from `20 of 20` to `20 / 20`; the temperature went from `+1.7 C, heading to +1.8` to `+1.7 C`. The bar got about a hundred pixels back, which is why net Emissions have moved left into the crop. |
| [`temperature-hover.png`](temperature-hover.png) | `shot:temp tip:Temperature seed:7 turns:12 window:1500x1000`. **Where the dropped figure went.** The temperature's own hover now says it in words -- *"The CO2 Stock already in the air is heading to +2.1 C on its own, with nothing else emitted"* -- above the chart that has always drawn it. |
| [`endturn-was.png`](endturn-was.png) / [`endturn-now.png`](endturn-now.png) | the same run, cropped to the foot of the command cluster. **End Turn's left padding.** The Influence rail's right end pulls back by 13 pixels and the sun stays exactly where it was, so the gap between the two goes from about four pixels to about seventeen. The cluster's own `of 20 left` is untouched, the designer having named the header. |
| [`found-site-was.png`](found-site-was.png) / [`found-site-now.png`](found-site-now.png) | the same run, cropped to the Colony Slot panel. **The founding door with the longest label in the game**, "Found a Colony here with the 8 Colonists aboard TSV Endeavour". Before: the four yields sat under the words, a second storey repeating the `Yields here:` line directly above it. Now they sit beside the words, which wrap to two lines inside the panel. |
| [`found-stack-now.png`](found-stack-now.png) | `shot:stack settler:mars seed:7 turns:8 window:1500x2200`, cropped to the Load and unload block of the Ship stack card. **The other founding door, six of them at once** -- one line each, the site's name and then its four figures. The tall window is what gets the block into a picture at all; the card is longer than a screen. |
| [`attack-odds.png`](attack-odds.png) | `shot:attack army:1 select:eastasia "tip:holding the field when" cardshut:1 seed:7 window:1500x1000`, cropped to the Armies block. **The attack button's hover, improvement G.** The stack's march on Russia: *"86% is the chance of holding the field when the Battle is over, not of winning its first exchange: your 8 against their 6."* The same board's first-exchange share, the figure this line carried until now, is 61%. |
| [`battle-report.png`](battle-report.png) | `shot:report menus:1 battle:region seed:7 window:1500x1000`, cropped to the Report. **The record, improvement G.** Both the headline and the Battle Report's party line read `9% odds of holding the field`, the same figure in the same words. The first exchange's share on that board is 30%, so the old record flattered a losing attack by better than three times. The four *refused it* lines are the aid answering the turn's Choice Card; see **Aids**. |
| [`orbit-odds.png`](orbit-odds.png) | `shot:orbit stack:1 cardshut:1 seed:7 window:1500x1000`, cropped to the head of the Ship stack card. **The orbit line, improvement G**, one per orbit your Ships sit in: *"Attacking in low orbit: 24% is your chance of holding the orbit when the Battle is over (your strength 3 against 0)."* A strength of 3 against 0 that reads 24% is not a defect; see below. |
| [`eye-region-now.png`](eye-region-now.png) / [`eye-region-was.png`](eye-region-was.png) | `shot:eyenow eye:1 select:russia cardshut:1 seed:7 window:1500x1000` and `shot:eyewas eye:0 ...`, cropped to the same part of the same rival card. **The eye on Earth, improvement L.** With an Embassy at home, Russia's card carries *"Your Embassy in China reads what the Prospectors draw here, building by building:"* and a line per Facility. Without one the block is not there at all and the card runs straight from Widgets to the slot boxes. Everything else on the two cards is identical. |
| [`eye-colony-now.png`](eye-colony-now.png) / [`eye-colony-was.png`](eye-colony-was.png) | `shot:eyecnow eye:1 hab:rival hab:ground cardshut:1 seed:7 window:1500x1000` and `shot:eyecwas eye:0 ...`, cropped the same way. **The eye off Earth.** A Relay at a Colony of ours at the Moon reads the Prospectors' Colony there Module by Module, and names itself and where it stands: *"Your Relay at Mare Imbrium on the Moon reads..."*. Without the Relay, nothing. |
| [`tech-tree.png`](tech-tree.png) | `shot:tall tech:1 seed:7 turns:12 window:1500x1400`, cropped to the tree. **The tree as it stands**, all five bands and all twenty Techs, with every edge clear of every box. This picture is **pixel-identical** to the same capture taken before the change, which is the finding, not a stale file: see below. |
| [`control-was.png`](control-was.png) / [`control-now.png`](control-now.png) | `shot:ctrlwas tech:1 seed:7 turns:12 window:1500x1400` and `shot:ctrlnow ...`, cropped to the Extraction band and enlarged three times. **The negative control**: a tree deliberately rearranged so that one edge has a box in its way. Before, the Deep Mining -> The Extraction Charter line goes in at Coastal Engineering's left edge and comes out of its right edge, so it reads as *Coastal Engineering feeds the Charter*, which is false. After, it leaves Deep Mining's top, runs the row gap above Coastal Engineering and drops into the lane. Both were taken with two temporary lines in `assets/data/techs.toml` (Coastal Engineering moved to Extraction rung 2, and Deep Mining added to the Charter's prerequisites), **which were reverted**; the shipped data is unchanged. |

## G. The odds shown are the whole Battle's

**The figure and the words moved together, at every site that carried them.** Four:

1. **The attack button's hover** (`attack_hover`), on a march on a Region, on a stack's march, and
   on a landing at a Colony. It read *"61% is the chance to win the first exchange"* and reads
   *"86% is the chance of holding the field when the Battle is over, not of winning its first
   exchange"*. The contrast clause is carried here, where a tooltip has the room for it.
2. **The Ship stack card, a rival's** -- *"A Battle here is a melee against every Faction present.
   Odds of winning the first round if you attack: N%"* -- and **the player's own**, which read
   *"Attack odds (first round): N%"*. Both now print `orbit_odds_lines`: **a line per orbit**, since
   ticket #335 made a Battle a thing of one orbit and a stack may be spread over several, so the one
   figure these lines carried could not be either honest or single. Each reads *"Attacking in low
   orbit: 24% is your chance of holding the orbit when the Battle is over (your strength 3 against
   0)"*, with the rule on its hover.
3. **The Battle Report's party line**, `, attacking at N% first-round odds` becomes `, attacking at
   N% odds of holding the field`.
4. **The Report's own `battle` line** in `assets/data/report.toml`, in the same words.

**The record moved with the words**, as the ticket directs: `BattleParty.odds` is set in `run_melee`
from `combat::whole_battle_odds` over the line as it stood **before the first round** -- the board
the attacker decided on -- and not from `first_round_odds`. It is measured from the figure's own
seed, so writing a record costs the game's dice nothing, and a Battle record lives one turn and is
wiped at the next, so no save carries a first-round figure under the new name for long. One pinned
value moved with it: `a_battle_is_a_report_line_at_its_place_by_name_with_odds_and_a_moment_when_a_unit_dies`
asserted the Report line read `N% first-round odds`, and now asserts the new words. The figure it
checks is read from the record either way, so nothing about that arm's strength changed.

**The threat line on a held Region's card was deliberately left alone**, and still says *"their
chance to win the first exchange"*. That figure is the **computer's own bar**: the same arithmetic
`nearest_army_threat` ranks threats by, and the same the amber compares against (*"the computer
seats attack at N% or better, so the line turns amber there"*). A whole-Battle figure there would be
measured against a first-round bar and the amber rule would stop being true. Moving both together is
a change to how the computer picks its attacks, which is a rule and the designer's to make.

**What it cost, and the one piece of machinery it needed.** The honest figure is measured, a
thousand melees a call and about **1.3 ms** of them, timed on this machine against the engine lane's
own fixture. A button's hover is built every frame whether or not anything is under the pointer, and
a Region card with three Armies of its own and four hostile neighbours asks for sixteen figures a
frame: twenty milliseconds, a third of the frame rate. So `memo_odds` keeps each answer against a
**fingerprint of the fight it was measured from** -- the place, the attackers, and every combatant's
strength, damage, hit points and stance for the ground; every hull in the orbit and every Battery
covering it for an orbit. Nothing that could move the figure can move without moving the key, so a
stale figure cannot be drawn, and within a turn the board does not move at all.

**A question for the designer, raised by `orbit-odds.png`.** One armed Frigate against three unarmed
rival hulls in the same orbit reads **24%**. That is the honest whole-Battle figure under the
engine's definition of a win -- holding the field means nothing of anyone else's is left standing,
and three hulls are hard to finish inside three rounds when they carry no strength to draw fire.
But in orbit a player may want **Orbital Control**, which is a different and easier thing than
clearing every hull out of the sky. Whether the orbit's figure should be the chance of holding the
orbit or the chance of taking Orbital Control is the designer's call; it is the engine's yardstick
either way, and this lane did not change it.

## L. The Relay or the Embassy as an eye

**Where it went: the rival place's own card**, under the Widgets line and above the buildings it
reads, on a Region card and a Colony card alike (`eye_block`). That is where a player wondering what
a place is worth to its holder is already looking, and the buildings named in the block are drawn as
boxes or tiles immediately beneath it.

**It names what is letting you see it** -- *"Your Embassy in China reads..."*, *"Your Relay at Mare
Imbrium on the Moon reads..."* -- so the rule is learnt from the thing itself. The building is named
by the kind actually standing, so an Arkwright's **Chorus** would say Chorus, as it does for every
other rule that reads a job rather than a kind. Without an eye there is no block at all, not an
empty one: `eye_income` answers `None`, and an empty block would read as a rival who makes nothing
rather than as a reading nobody has paid for. A spectated game draws none, having no "you".

**A finding the pictures turned up, for the designer.** A rival Region's card *already* discloses a
Facility's figures one at a time: every slot box's hover carries `facility_figures` read for the
place's director, and a Module tile's hover on a Colony card carries `module_line` the same way. So
what the eye adds today is the whole list at once, on the face of the card, rather than a thing that
could not be learnt at all. If the intent is that a Faction without an eye should **not** be able to
read a rival's building figures, those hovers want gating on `has_eye_at` as well -- which takes
away information players have today, so it is the designer's call and was not made here.

## N. Tech Tree edges routed around the boxes they cross

**The measurement first: with today's twenty Techs, no edge crosses a box.** Every edge was
replayed offline against `box_of`'s own arithmetic -- 144 by 86 grid, 122 by 58 boxes, the `#250`
band order -- and no leg of any of the seventeen edges meets any of the twenty boxes, with six
pixels of padding to spare. The defect issue #246 photographed was real and is gone: it was drawn
when the last rung laid its boxes side by side, and `#250`'s *every rung stacks* moved the rung-3
column back under one x. The tree picture therefore cannot change, and does not.

What changed is that the property is now **held by the code rather than by the data's good luck**.
The old routing had one detour and it fired only for a box whose middle sat within a pixel of the
needed box's middle (`(ob.center().y - from_box.center().y).abs() < 1.0`) and which lay wholly
inside the run -- a test that knows nothing about boxes in general. Adding one Tech to a rung-2 cell
is enough to put a box back in a line's way, which is exactly what `control-was.png` does.

**How it is routed**, in `tech_edge_path`: the elbow is unchanged -- out of the needed box, along
the lane in the gap left of the needing box's column, in at its left edge -- but the horizontal
**run**, the only leg long enough to cross a column, is tried at a series of heights and the first
clear one is taken. The heights are the needed box's own middle (the straight elbow, and what is
drawn today), then the air between box rows, below and above, stepping a whole row at a time out to
three rows either side, nearer side first. A run at one of those leaves the box by its top or bottom
and drops into the lane. With nothing clear it falls back to the straight elbow, because a line
drawn badly can still be read and a line not drawn cannot.

**What it cost**: three rows either side and no more, so this is a detour and not a graph-layout
engine; a detour spends one extra leg and leaves the box by a face rather than a side, which
`control-now.png` shows. Two lines sharing a lane still overlap, as they already did.

**Guarded by a test**, `a_tech_tree_edge_is_routed_around_the_boxes_in_its_way`, built on the tree's
own grid figures. It was **witnessed red**: with the height loop removed so the function always
returns the straight elbow, it fails at *the routed edge clears it*. The test also carries its own
control arm asserting that the straight elbow really does cross the fixture's box.

## The four adjustments

1. **`15 / 15`, not `15 of 15`.** `src/ui.rs`, the top bar's Influence and its bought-Influence
   variant. Every other paired figure on that bar is a slash -- Widgets `16 / 18`, Research
   `39 / 40`, `Turn 9 / 36` -- so Influence was the one figure there written in words. The command
   cluster's `of 20 left` is left alone.
2. **`+1.2 C` alone.** The top bar's temperature drops `, heading to +1.2`. The figure is not lost:
   the hover's first sentence now names it and says what it means, and the hover's chart has always
   drawn it. The Climate Panel's own line keeps both figures.
3. **End Turn gains 12.65 pixels on its left** (`10.0 * CLUSTER_SCALE`, scaled with everything else
   in the cluster as ticket #211 requires). The pad is taken off the left column's width rather than
   added to the strip, so the cluster is the width it always was and only the sun moves.
4. **The yields sit beside the words.** `found_button` lays label and yields in a row instead of a
   column. **The words wrap**, and the first picture is why: laid on one line the longest door came
   to about 610 pixels and widened the side panel by ninety, eating that much of the map, because
   the panel sizes itself to its content. The words are given the room left after the yields
   (`YIELD_ROW_W`, 220, measured off that capture at 216) and wrap inside it; a short door still
   puts the glyphs right beside its last word, because the words claim only the width they use.

## Looked at

Eighteen pictures now, every one opened and read before this was written. The seven of G and L
caught two defects:

- **The orbit line said nobody was there when three rival hulls were.** The first `orbit-odds.png`
  read *"No rival stands in an orbit of yours here, so an Attack ordered here fights nobody"* while
  the map behind it listed a Prospector, an Arkwright and an Archivist hull in the same low orbit.
  The line was skipping an orbit whose rivals summed to **strength** nought -- three unarmed hulls,
  which a Battle would still destroy. It tests presence now, not strength, and the picture in the
  table is the fixed one. Nothing but a picture would have found this: the board that produces it is
  the ordinary `stack:1` board every other capture has used.
- **`hab:rival` opened the wrong card.** Its first form picked the first Colony any rival directed,
  which on every board is the **station over Earth**, so the Moon's picture showed an Earth station
  and the Relay case went unphotographed. It honours `hab:ground` now, and the pair in the table is
  the Moon.

Two more things the eyes-on pass settled rather than caught: the Choice Card modal sat over the eye
block in the first captures, answered with `cardshut:1`; and the first eye pair was taken on two
different random seeds, so the two boards' slot yields disagreed and the pair proved less than it
claimed. `seed:7` makes the two boards identical but for the block.

The earlier eleven pictures, and three things caught by looking at them:

- **The founding button widened the side panel.** Caught on the first capture of adjustment 4, fixed
  by wrapping the words, and the fixed version is `found-site-now.png`. `docs/HANDOFF.md` records
  this button as built and never looked at by anybody; this is the first time it has been seen.
- **The Tech Tree picture did not change at all.** Decided rather than assumed: the tree-only crop
  of the before and after captures differs by zero pixels, and the offline sweep says why. The
  control pictures exist because a change nothing can show is a change nothing has tested.
- **The founding doors are ragged down their right edge** on the Ship stack card, since each button
  is as wide as its own label and the yields follow the last word. It is what *beside* costs, and it
  is the designer's call whether the figures should line up in a column instead.

A defect the temperature hover picture caught that is **not this ticket's**: the chart's caption
reads *"Temperature by turn; red is a Break"* while its axis is labelled with in-game dates. Left
alone; it belongs to the chart.

Not photographed: the clicks themselves (headless, no pointer), and a founding button on the
Antarctic sea door, which wants a board with the ice open and emigrants waiting.

## Aids

**`eye:1` and `eye:0`** (ticket #339): the pair of boards the eye is photographed on. Seat 1 takes
the first neighbour of seat 0's start Region and runs two Facilities in it, and holds a Colony on
the Moon with three Modules; `eye:1` also gives seat 0 a working Embassy at home and a Colony at the
Moon with a working Relay, and `eye:0` gives neither. So the same two rival cards can be
photographed watched and unwatched, and the difference is the block and nothing else. No AI game
offers a board of that shape on a turn anybody can choose.

**`hab:rival`** (the same ticket): the Hab selection picks the first Colony a **rival** directs
rather than seat 0's own, since the eye's whole point is what it reads on somebody else's card and
no aid could open one. It honours `hab:ground` beside it, which is what keeps the Moon's picture off
the station standing over Earth.

**A Choice Card no longer panics an aid that drives a turn.** Ticket #337 made `end_turn` refuse
while a card is unanswered, and every aid that drives a turn by hand (`battle:1`, `battle:region`,
`battle:earth`, `strip:1`, `found:1`, and `run_one_quiet_turn` with the half-dozen aids built on it)
ended the turn without answering it. On any seed whose first turn drew a card the run **panicked**:
`shot:report menus:1 battle:region seed:7` did, which is how it was found, and the run that first
produced `battle-report.png` survived only because its random seed drew no card. They call
`refuse_any_card` first now, which refuses the offer for every human seat -- the answer that buys
nothing and leaves the board the aid built where the aid put it. The four *refused it* lines in
`battle-report.png` are that refusal in the Report.

`site:<body id>,<slot>` is now applied **after** `settler:<body id>`, so the two can be given
together: `settler:` is what puts a Colony Ship at the Body, and without it the slot panel `site:`
opens has no founding door on it at all. Before this the stack selection won and the slot panel
could never be photographed with its door.
