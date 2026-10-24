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
moved to 0.3, and the Report's word ceiling rose by the phrase's eight words. No picture: a Report
line and three figures.

- **The sweep** ([`sweeps/ticket-622.txt`](sweeps/ticket-622.txt)): **27 / 49 / 3 / 0 and 0
  collapses**, from 29 / 51 / 0 / 0 and 0: noise. Accords struck over the batch 616 from 613: the point
  off with every rival breaks nothing and costs the Prospectors no partner. Reported and left.
