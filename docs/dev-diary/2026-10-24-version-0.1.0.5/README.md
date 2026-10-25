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
then the texts. The Refinery's sentence was first written into the Module row's `does` field, which
turned out to be read by nothing in the window or the tool -- every `does` in the two tables is dead
data -- so it was taken back out and drawn in code, on the tile's line and the build hover, with a
window test. No rule moves; no sweep. No picture: no Colony with a Refinery stands on turn one's board and a station offers none, so the headless aid has nothing to hover; the two strings are held by the test.

The top-bar ticket's review (#609) found the Reactor's relief missing from the per-place lines, so they
could not sum to the Income's Energy while one stood, and the hover unbounded on a wide board; both
fixed, with the window test grown to cover the relief line, the fold past the fourth place and the
five-place case, and the three comments the review found stale.
