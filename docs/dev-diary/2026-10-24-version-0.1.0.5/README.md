# Version 0.1.0.5, the pushback version

The map is [Map: version 0.1.0.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/606); the spec
is [`docs/spec/version-0.1.0.5.md`](../../spec/version-0.1.0.5.md). The standing baseline is 0.1.0.4's
closing board, **23 / 42 / 4 / 10 and 1 collapse**
([`final-0.1.0.4.txt`](../2026-10-21-version-0.1.0.4/sweeps/final-0.1.0.4.txt)), against the designer's
15 each and 20.

## Every off-world population bar up by 2, and the Archive at a place of Colony tier

Ticket [#625](https://github.com/whaleyjoshua2/Dying-Earth/issues/625), spec §1.

Two bars moved in the data (Off-world Presence 20, the Diaspora 44; its four on each of four Bodies
kept, after a first build at six on a misread answer was put back at the designer's word) and the
Upload stayed at 16, at the designer's word; in its place the Archive asks a place of Colony tier,
`[archive] tier_to_order = "Colony"` in place of `colonists_to_order = 4`. Three tests were written
first and watched red: the bars (left 18, right 20), the order's refusal (an `Ok` where the Outpost of
eighteen should be refused), and the computer naming only a Colony-tier place -- whose first half
had passed for the wrong reason until the Outpost's slots were filled, so the Archive was the one
thing left to order there. Then the twenty-two tests that pinned 18, 42 and four moved with them. No
picture: figures and a refusal line, nothing on the screen changes shape.

- **The first sweep** ([`sweeps/ticket-625-ferry-paused.txt`](sweeps/ticket-625-ferry-paused.txt)),
  the rule alone: **25 / 52 / 2 / 0 and 1**. No computer Archive stood in any of 80 games (43 of 80
  on the baseline): the Archivists' ferry pauses its spending lift the moment a place that may hold
  the Archive exists (ticket #361), and under the tier an Outpost of four is such a place but not a
  site, so nobody was carried there to climb it. Their fund sat full at 250 in every game.
- **The fix**: a site is a place that may hold the Archive AND stands at the Colony tier; the lift
  stays on until one does. A fourth test, red first (the Habitat at the Outpost weighed as at a
  Colony), then green.
- **The second sweep** ([`sweeps/ticket-625-six-each.txt`](sweeps/ticket-625-six-each.txt)), with the
  ferry fixed but the per-Body figure still at six: 25 / 50 / 2 / 3 and 0, the Archive in 13 of 80.
- **The sweep** ([`sweeps/ticket-625.txt`](sweeps/ticket-625.txt)), the four on each Body back:
  **26 / 48 / 2 / 2 and 2 collapses**. The computer's Archive stands in 11 of 80 at a median turn 35
  (43 at about 30 before); the Archivists win 2 from 10, the Arkwrights 2 from 4, and the Prospectors
  take what both lose, 48 from 42, six past noise. Colony-tier places at the end 34 from 23, Capitals
  28 from 16. **Reported and left**: the tier is the designer's word, and what more the computer
  should do to climb -- or whether the tier should be the Settlement's -- is theirs to say.

## The pack on a runaway leader forms at 40% of Victory

Ticket [#621](https://github.com/whaleyjoshua2/Dying-Earth/issues/621), spec §2.

One figure moved: `gang_from` 0.5 -> 0.4 in `ai.toml`, at the designer's word; `gang_ahead` and
`gang_full` as they were. The pressure test was moved to the new ramp first (a leader at 80% reads two
thirds, not 0.6; 45% is a leader, 39% not) and watched red on the old figure, then the pack test's
x1.6 lift moved with it. No picture: a figure the screen never shows.

- **The sweep** ([`sweeps/ticket-621.txt`](sweeps/ticket-621.txt)): **25 / 50 / 2 / 2 and 1
  collapse**, from 26 / 48 / 2 / 2 and 2: noise. Accords struck over the batch 620 from 620, games
  with an orbital Battle 24 from 27, the computer's Archive 9 of 80 from 11. The earlier pack did not
  bite on the Prospectors' 50; reported and left.

## An Occupation halves the prior holder's Standing

Ticket [#615](https://github.com/whaleyjoshua2/Dying-Earth/issues/615), spec §3.

One data key (`occupation_standing_kept = 0.5`, the share kept), one rule at the Occupation's start in
`resolution.rs`, one Report phrase that names the holder and the figure. The test was written first
and watched red (41 stayed 40, the turn's decay alone), then green at 20 less the decay; the old test
that had the prior holder keep 40 through a transfer now reads 20, and the Report's word ceiling rose
by the phrase's nine words. No picture: a figure on the Region card's Standing line.

- **The sweep** ([`sweeps/ticket-615.txt`](sweeps/ticket-615.txt)): **26 / 49 / 2 / 2 and 1
  collapse**, from 25 / 50 / 2 / 2 and 1: noise. Occupations begun over the batch 204 from 197, broken
  37 from 36. The halving changes what a retake costs, not how often the computer occupies; reported
  and left.

## One more Colony Slot on the Moon, Phobos and Deimos

Ticket [#613](https://github.com/whaleyjoshua2/Dying-Earth/issues/613), spec §4.

Three rows in `bodies.toml`, at the designer's word on the names: Oceanus Procellarum on the Moon, Todd
on Phobos, Voltaire on Deimos. The test was written first (five, three and two slots, the new names
last, no two sites within five degrees) and watched red on the Moon's four, then green; the old pin of
Phobos's two and Deimos's one moved with it.

**The pictures** ([`ticket-613/slots-moon.png`](ticket-613/slots-moon.png),
[`slots-phobos.png`](ticket-613/slots-phobos.png), [`slots-deimos.png`](ticket-613/slots-deimos.png)):
each globe from the headless shot: Oceanus Procellarum at the Moon's left, labelled with its yields, beside Mare Imbrium and Tycho; Voltaire low on Deimos under Swift; Todd on Phobos's far side, so a second shot turned to face it ([`todd-phobos.png`](ticket-613/todd-phobos.png), `look:-153,-9`) shows it centred. Looked at, all four; the prescreen's SUSPECT is the known full-bleed false positive.

- **The sweep** ([`sweeps/ticket-613.txt`](sweeps/ticket-613.txt)): **29 / 51 / 0 / 0 and 0 collapses**, from 26 / 49 / 2 / 2 and 1. The landings moved: ground Colonies at the end over the batch on Phobos 165 from 127, Deimos 57 from 40, the Moon 290 from 270. The Arkwrights' and Archivists' nought is within noise of their 2, and reported.

## The Strip Permit: more CO2, a point off with every rival, no growth while it runs

Ticket [#622](https://github.com/whaleyjoshua2/Dying-Earth/issues/622), spec §5.

Two figures in `nation_states.toml` (the rise 0.3, `relations = 1`), the point taken off the deeds
score at the permit's commit the way a card's `RelationsAllRivals` takes it (never `offend_by`, so no
Accord breaks), and the Region's growth rate clamped to nought while the permit runs. The test was
written first and watched red (every rival's view unmoved), then green; the old permit test's 0.2
moved to 0.3, and the Report's word ceiling rose by the phrase's ten words. No picture: a Report
line and three figures.

- **The sweep** ([`sweeps/ticket-622.txt`](sweeps/ticket-622.txt)): **27 / 49 / 3 / 0 and 0
  collapses**, from 29 / 51 / 0 / 0 and 0: noise. Accords struck over the batch 616 from 613: the point
  off with every rival breaks nothing and costs the Prospectors no partner. Reported and left.

## Every Facility and Module costs a fifth more Widgets, and a Frigate 6

Ticket [#610](https://github.com/whaleyjoshua2/Dying-Earth/issues/610), spec §6.

Thirty-nine rows in `facilities.toml` and `modules.toml` moved (4 to 5, 5 to 6, 8 to 10, 12 to 14), by
a pass over the two tables that touched only `[[facility]]` and `[[module]]` rows, so the tiers and
Industry Level stayed; the Frigate's row alone in `units.toml`. The test was written first (every row
in {5, 6, 10, 14}, the Core at nought, the Frigate 6, the rest pinned) and watched red on the
Factory's 4, then green; twenty-two pins in older tests moved with the rows, the Archive's build
loop among them (four Resolutions now). No picture: figures every hover reads from the table.

- **The sweep** ([`sweeps/ticket-610.txt`](sweeps/ticket-610.txt)): **25 / 51 / 2 / 0 and 2
  collapses**, from 27 / 49 / 3 / 0 and 0: noise. The fifth shows where it should: Widgets applied a
  game up by about a sixth for every seat, Materials and Energy made over the batch down six per cent
  for the Custodians and the Prospectors, within two for the other two. The computer's Archive 3 of
  80 from 9, at the edge of its noise. Reported and left.

## Turn one's signature moves: one sentence, the player's Faction alone

Ticket [#607](https://github.com/whaleyjoshua2/Dying-Earth/issues/607), spec §7.

Four sentences on the Faction rows at the designer's word (the drafts with the turn labels cut), the
window cut to the player's own row, the tool's `new` to one line, the `{your}` placeholder gone with
its only rival reader. The test was moved to the new sentences first and watched red on the old
(a first sed had folded it into one commented line, which "passed" with nothing in it, and was put
right before the red was believed). No rule moves; no sweep.

**The pictures** ([`ticket-607/custodians-report.png`](ticket-607/custodians-report.png) and the three
beside it, with [`tool-new.txt`](ticket-607/tool-new.txt)): the window over the Earth map, "Signature
moves", the Faction's glyph and name in its colour, the two sentences wrapped on two lines, "Go on".
Looked at, all four; a first run had kept the solar view, where the window never stands, and was
reshot through the menu captures.

## The Region card's threshold lines cut to the hover, and one hover per element

Ticket [#608](https://github.com/whaleyjoshua2/Dying-Earth/issues/608), spec §8.

The two weak lines under the Standings row were cut and their terms became `threshold_terms` and
`held_price_terms`, sums on the threshold label's hover through one `threshold_label` the row and
the test both read; the row's own `rule_tip`, the second hover, went. A window test holds every
label's tip under six lines, and it bit twice before the shape settled: with the sums appended to the
old sentences the own-Region tip estimated at eight lines, and the neutral tip at seven, so the
sentence that repeated the sums and the neutral tip's held-place sentence were cut. No sweep.

**The pictures** ([`ticket-608/neutral-earth.png`](ticket-608/neutral-earth.png),
[`own-earth.png`](ticket-608/own-earth.png)), looked at: Egypt's card with "Threshold 40 = 20 + 20
for size 2" as the hover's first line, and China's with the price sum. A first own-Region shot had
caught the Influence heading's tip instead, the `tip:` aid matching a word that tip also carries.

## The top bar's Widgets hover in the Fuel's shape, and the Energy hover by place

Ticket [#609](https://github.com/whaleyjoshua2/Dying-Earth/issues/609), spec §9.

The Income now keeps Energy by place beside its source lines, summed where the lines are made rather
than parsed back out of their names; an engine test holds each place's figures to its source lines
and the whole to the Income's Energy (it found the test fixture's home Region holds no Power Plant,
and was given one). The window's two hovers were rewritten with a window test beside them: the Widgets
hover opens on "Made a turn" and closes on what went into builds, the Energy hover is one net line a
place and appends the shortfall's dark list instead of replacing the lines. No sweep.

**The pictures** ([`ticket-609/widgets-earth.png`](ticket-609/widgets-earth.png),
[`energy-earth.png`](ticket-609/energy-earth.png)), looked at: the Widgets hover "Made a turn: 15" with four
makers and "0 went into builds last turn", the Energy hover "-3  China (6 made, 9 upkeep)" and "+5  ISS
over Earth (6 made, 1 upkeep)", summing to the bar's +2. The Energy figure had to be drawn through
`rule_tip` as the Widgets figure is before the `tip:` aid could photograph it; a shortfall picture
was not taken, since the aid cannot place an order that overspends the Energy.

## Three texts: the Cheap Ore refusal, "breaks Accord", the Refinery's refuelling

Ticket [#611](https://github.com/whaleyjoshua2/Dying-Earth/issues/611), spec §10.

Three pins moved first and watched red (the window's armed-button test, the engine's two Accord lines),
then the texts. The first build went wrong twice and the review caught both: the Cheap Ore change
touched a card field that is drawn nowhere (the duplicate was the refuse button's own face over the
line the card generates under it, so the face is now bare), and the Refinery's sentence was drawn in
code on a wrong belief that a Module row's `does` field is dead, when the yield's text reads it; it is
the row's `does` now, owner-neutral, with the window test holding the drawn line. The tool's refuel
help was also made to contradict itself and was rewritten.

**The pictures** ([`ticket-611/ore-earth.png`](ticket-611/ore-earth.png),
[`accord-earth.png`](ticket-611/accord-earth.png)), looked at, taken after the designer asked where the
screenshots were: the Cheap Ore card with the refuse button bare, "Let it go", over its generated
"Materials a third off for you next turn"; and the European Union's card under a Non-Aggression Accord
with its holder, "breaks Accord" in red beside the Spend and Max buttons. The Refinery's line has no
picture: no aid plants a Refinery Colony on turn one's board.

The top-bar ticket's review (#609) found the Reactor's relief missing from the per-place lines, so they
could not sum to the Income's Energy while one stood, and the hover unbounded on a wide board; both
fixed, with the window test grown to cover the relief line, the fold past the fourth place and the
five-place case, and the three comments the review found stale.

## The Scrubber cap: one per 200 million

Ticket [#612](https://github.com/whaleyjoshua2/Dying-Earth/issues/612), spec §11.

One figure, `per_population` 300 -> 200, at the designer's word from a table of what 300, 200 and 150
give every Region. The cap test's pins moved first (India eight, a Region of 380 two) and were watched
red on the 300, then green.

**The picture** ([`ticket-612/cap-factions.png`](ticket-612/cap-factions.png)), looked at, taken after
the designer asked: the Custodians' Faction card, whose signature rule reads the cap from the table,
"one per 200M people, 1 to 10". The Region card's "Scrubber (2 of 7)" button sits below the fold at
the shot's 800 pixels, so the Faction card is the picture.

- **The sweep** ([`sweeps/ticket-612.txt`](sweeps/ticket-612.txt)): **23 / 53 / 1 / 0 and 3
  collapses**, from 25 / 51 / 2 / 0 and 2: noise. Scrubbers built over the batch 2,262 from 2,113,
  seven per cent more, the end Temperatures and the Sink within noise; that reads as the cap binding in few
  Regions, an inference from the count rather than a measured one. Reported and left.

## The Battle Moment says who won

Ticket [#614](https://github.com/whaleyjoshua2/Dying-Earth/issues/614), spec §12.

The test was written first from the designer's own case -- the Defiant sinking the Valiant and losing
nothing -- and watched red on the card as it was, "A Battle at Mars low orbit: PMV Valiant destroyed"
over "1 lost"; then the parties were given their units' names and the Moment's sentence and figure
rebuilt from the losses. Five cases held: won, lost, both lost, the player out of it, and a bloodless
Battle with no Moment. The picture, [ticket-614/battle-report.png](ticket-614/battle-report.png), came
after the ticket closed, at the designer's word: the new `battle:two` aid brings seats 0 and 1 alone
to Mars (the old `battle:1` brings three, and a three-sided Battle reads the losses alone), and the
card reads *"you won"* over *"Battle at Mars orbit: your TSV Valiant and TSV Vanguard destroyed the
Prospectors' PMV Magellan and PMV Indomitable."* The first shot read *"your  destroyed"*: both
Frigates had rolled a disengage in the round their blows landed, so no unit "stayed" and the
destroyer's name was empty. A side that left whole is now named whole, held by a third test written
red first.

## Nine hovers cut to the designer's lines

Ticket [#616](https://github.com/whaleyjoshua2/Dying-Earth/issues/616), spec §13.

Seven questions in one round (two cuts given for the Accord's notice and the Directive, the first
landing's second clause, the Upload already cut by #587, what the Allotment's `{breakdown}` is, a
Blockade hover that repeated its own line, Unrest's first figure), answered *"q1 c q2 that q3 yes q4
keep 587s q5 b q6 b q7 yes"*. The texts went into constants and three small functions beside the
Directive's sentence, pinned by a test mod watched red on the missing names; the six that hung on
`on_hover_text` were moved onto `rule_tip` so the `tip:` aid could photograph them. The Allotment's
hover reads the recorded parts and, on turn one before any Income, the live ones: the test caught the
empty case, *"Influence each turn. Unspent is lost."*, on a fresh game.

**The pictures** ([`ticket-616/`](ticket-616/)), each looked at: China's card with the Pioneers line
([`pioneers-earth.png`](ticket-616/pioneers-earth.png)), the Unrest line
([`unrest-earth.png`](ticket-616/unrest-earth.png), "Unrest to 10: Army stops at 4, Facilities halve
at 7, revolt at 10. Falls 1.5 a turn.") and the Influence value
([`allotment-earth.png`](ticket-616/allotment-earth.png), "Influence each turn: Regions and buildings
14. Unspent is lost."); the Moon's card ([`landing-moon.png`](ticket-616/landing-moon.png), "First to
land here: +1 Influence a turn while held." over "A station claims nothing."); the Prospectors' page
with the Accord's notice ([`accord-factions-mars.png`](ticket-616/accord-factions-mars.png), on the
Mars view, where no Climate Panel covers the Factions window); the Directive in the Tech Tree window
([`directive-tech-mars.png`](ticket-616/directive-tech-mars.png)); and the Climate Panel's Blame
heading ([`blame-climate-earth.png`](ticket-616/blame-climate-earth.png)), drawn bare, the
"[BLAME_HOVER]" tag being the dump's. Three aids were tried before the right ones: `menus:1`
photographs the Faction CHOICE screen, `factions:1` opens the Factions window under the Earth view's
Climate Panel, and the Directive is in the Tech Tree window (`tech:1`), not the Faction window. No
picture of the starved or overcrowded lines: no aid starves or overfills a Colony on turn one's board.

**The review** (an agent that did not build it) found the standing order's checkbox still on
`on_hover_text`, kept there for its greyed words; it is on `rule_tip` now with a branch for them, and
photographed ([`order-earth.png`](ticket-616/order-earth.png)). It found the turn-one fallback a copy
of the engine's `allotment_lines` closure, moved into the engine, so the top bar's "of N left" hover
has its lines on turn one too; the Unrest test a tautology (built from the format it was checked
against), now the shipped line; the Allotment's ordinary branch unpinned, now pinned; a stale comment
above the Region card's Allotment line, rewritten; and the new block wedged between the two sentences
ticket #382's doc comment introduced, moved after them.

## The text tool shows the first landing, the race to the Moon and the Tech tree

Ticket [#617](https://github.com/whaleyjoshua2/Dying-Earth/issues/617), spec §14.

Four questions, answered *"q1 a q2 yes q3 a q4 a"*: the race mirrors the window's ladder rather than
the ticket's per-Body draft, every Body gets its first-landing line, the tree sits behind `show
--techs`, and runs by branch with every Tech's effect. The three texts went into the engine, tested
red first on the missing functions, so the tool and the tests read one line; the step's words moved
from the window onto `MoonStep` and the band order onto `TECH_BRANCH_ORDER`, the window reading both.
The tree test's first failure was its own: it left the progress at 10 after moving the pick, and the
engine's *"chosen this turn, 10/32"* was right.

**The artifacts** ([`ticket-617/bodies.txt`](ticket-617/bodies.txt),
[`tech-tree.txt`](ticket-617/tech-tree.txt)): the tool's Bodies block and tree on a fresh seed-7
game, looked at: the Moon carries *"first landing: nobody yet, 5 Influence to the first"* over
*"race to the Moon: Custodians no Shipyard, ..."*, Earth carries neither, and the tree opens on
Society with Large Language Models available and The Upload locked behind two Techs with the
Archivists' gate marked.

**The review** (an agent that did not build it) found the tree's in-rung order departing from the
window's (a card's `stack` ignored: Commodity Finance drawn above Automated Refining in the window,
printed below it), the tool's "available" wider than the window's (every unlocked Tech, where the
window reads the drawn shortlist), the needs joined "A and B and C", the spec's premise that no turn
is kept for a first wrong (the first's Colony stays and carries its founding turn, so the line reads
"turn 13" as the designer had said yes to), the race test's second arm dead (the fixture strips
every Shipyard), the race computed for all nine Bodies and dropped for eight, the branch list copied
from the window, the order constant in `ids.rs` rather than beside the Tech card, the playtest sheet
and the module doc silent on the flag, and one effect with a full stop. All fixed: the tree sorts by
(rung, stack), reads the shortlist and says "unlocked, not on this pick's shortlist" for the rest,
joins needs as an and-list; the race test pins the exact line and then a Shipyard leading it; the
window and the tool read one `tech_branches`.

## The computer contests a rival's climb on a neutral Region

Ticket [#618](https://github.com/whaleyjoshua2/Dying-Earth/issues/618), spec §15.

Four questions, answered *"q1 yes q2 yes q3 a q4 yes"*. The test was written first on a neutral
Egypt (a contester with a little Standing, the human climbing to half the threshold then to it, a
computer rival climbing instead, an aloof seat, a neighbour's holder, the orders and the count) and
watched red on the three missing names; its first failure after the build was its own premise, a
named seat held Europe, Egypt's neighbour, so the aloof seat is now found rather than named. Agitate
dropped out of the answer on a read of the order: it wants a holder.

- **The sweep** ([`sweeps/ticket-618.txt`](sweeps/ticket-618.txt)): **41 / 36 / 2 / 0 and 1
  collapse**, from 23 / 53 / 1 / 0 and 3. Not noise. The whole swing is in the three seatings where
  the Custodians do not sit first; the Custodians' own seating is unchanged. Contests run about 25 a
  game across the table. The reading is that the Prospectors were the snipers and a contest costs
  the sniper most. Reported and left for the designer: `contest_from` and the lift are the levers.

**The review** (an agent that did not build it) found the spend not sized: the designer's yes was
to "outbids where the Allotment allows", and the build had lifted every step alike and sized nothing.
The steps that would pass the climber's Standing now take the multiplier, enough to outbid, else all
the Allotment; the test outbids a climber at the threshold from 3 in several steps and counts one
contest. Also fixed: the climb read against the climber's own threshold (Blame, Green Consensus, the
Regions-held term) rather than the board's; the #394 doc comment left over the new function; the
multiplier named `ai_contest_lift` with `ai_climb_on` and `ai_contested` beside it; the spec's
contest figures (by seat they run 76 to 213, a seating 466 to 609, not "380 to 610 by seat"); ties
and the figure's edge pinned in the test, a vacuous assertion dropped. The sized build re-swept
([`sweeps/ticket-618-sized.txt`](sweeps/ticket-618-sized.txt)): 37 / 39 / 1 / 2 and 1, within noise
of the first build's 41 / 36 / 2 / 0 and 1; the swing is the rule's.

## The computer defends its Regions, and keeps a floor at home

Ticket [#619](https://github.com/whaleyjoshua2/Dying-Earth/issues/619), spec §16.

Four questions, answered *"q1 yes q2 yes q3 b q4 yes"*. The test was written first (the floor, a
held Region quiet then climbed to under and at half its price, the home at a quarter, the orders
sized past the climber's Standing plus their Allotment, a home lost counted by loser and taker) and
watched red on the eight missing names. Its first failure after the build was the premise: a home
starts with the threshold banked (ticket #75), not nothing, so the floor of 20 first bites around
turn 20; the spec says so, and that the question put to the designer had it wrong. One old test
wiped every Region Standing and then found seat 0 pouring its Allotment into its own home: the new
rule working in a fixture that had taken the home under the floor; the fixture keeps the home at
its floor now.

- **The sweep** ([`sweeps/ticket-619.txt`](sweeps/ticket-619.txt)): **19 / 58 / 2 / 1 and 0**,
  from 37 / 39 / 1 / 2 and 1. Not noise, and the contest ticket's swing reversed and then some.
  Homes lost fall for seat 0 (7 / 13 / 4 / 5 of 20 seeds from 11 / 18 / 11 / 9), the Prospectors
  lose theirs most and win regardless, the Custodians take the most homes and win least. The
  reading is that the Allotment is one budget and a seat that defends takes fewer Regions.
  Reported and left for the designer: the triggers, the lifts and the floor are the levers.

**The review** (an agent that did not build it) found the wanted figure spending against the
Standing wall while the threshold alone still held (a holder decayed to 10 with a climber at 25
and ten to spend reached 35 against a threshold of 50, and bought six for nothing), and the test's
orders clause asserting on a case that wanted nothing. Both fixed: nothing is wanted where the
threshold stands above the climber's reach, and the test puts the climber at the price less one and
sees ten bought. Also from the review: the ticket's draft had sized the answer by the rival's last
turn's gain, which no record keeps, and the question the designer said yes to sized it by the
rival's Allotment; the spec says so now, and that the doubling is on the list's weight. The
top-rival line and the lift were pulled into helpers shared with the contest. The fixed build was
re-swept and came out identical to the figure, so the duplicate file was not kept.
