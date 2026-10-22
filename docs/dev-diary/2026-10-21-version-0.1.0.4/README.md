# Version 0.1.0.4, the storyteller version

The map is [Map: version 0.1.0.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/583); the spec
is [`docs/spec/version-0.1.0.4.md`](../../spec/version-0.1.0.4.md). The standing baseline is 0.1.0.3's
closing board, **26 / 41 / 5 / 8 and 0 collapses**
([`ticket-582.txt`](../2026-10-19-version-0.1.0.3/sweeps/ticket-582.txt)), against the designer's 15
each and 20.

## The Prospectors' Venture Fund at 5,000

Ticket [#584](https://github.com/whaleyjoshua2/Dying-Earth/issues/584), spec §1.

Two data rows moved: the bar (`factions.toml`, `victory_first.bar = 5000`) and the computer
Prospectors' pace (`ai.toml`, every figure x 5000/4250 on the same turns, the designer's choice of
the two offered). The bar test was run first and watched red on the new figure (left 5000, right
4250), then every pin that read the old bar moved with it: the Victory margins, the rival-Moment
figure (3,800 of 5,000), the headless shot's `rival:1` aid, the turn-one line, the glossary, the
playtest note. No picture: a figure, nothing on the screen changes shape.

- **The sweep** ([`sweeps/ticket-584.txt`](sweeps/ticket-584.txt)): **30 / 36 / 3 / 9, 1 collapse
  and 1 draw**, from 26 / 41 / 5 / 8 and 0. The Prospectors' five off and the Custodians' four on
  are at the edge of noise; the rest is noise. **The bar did not bite**, as 4,250 did not: the
  Prospectors' Fund at the end by seating is median 5,200 / 5,064 / 4,112 / 4,808 of 5,000, from
  4,425 / 4,298 / 4,075 / 4,308 of 4,250, up by 500 to 775 in three seatings and unmoved in the
  third; their gate completes in 79 of 80 at a median turn 27, as before. The computer sets its
  banking share from the bar each turn, so a higher bar is met by a higher share. Reported and left
  at the designer's word.

## The Arkwrights and the Archivists open with 10 Ducats

Ticket [#585](https://github.com/whaleyjoshua2/Dying-Earth/issues/585), spec §2.

A `start_ducats` beside the Prospectors' `start_materials` on the Faction's row, read by the one
function `Game::new` and the card both use; the Arkwrights' and Archivists' rows say 10. Two tests
were written first and watched red (the seats at 0 Ducats; the card's glyph line with no Ducats),
then green. The designer chose the glyph line alone over a signature clause.

**The picture** ([`ticket-585/faction-cards-low.png`](ticket-585/faction-cards-low.png)): the Faction
screen's second row, the Arkwrights and the Archivists, each opening line reading *"Opens with 80 · 20
· 20 · 10"* with the Ducat glyph; the signature rules unchanged. The second row sits below the fold at
1280x800, so the shot needed a new building aid, `cards:low`, which scrolls the cards down one row.

- **The sweep** ([`sweeps/ticket-585.txt`](sweeps/ticket-585.txt)): **28 / 37 / 5 / 10 and no
  collapse**, from 30 / 36 / 3 / 9, 1 collapse and 1 draw. Noise throughout: the Arkwrights +2,
  the Archivists +1. In a logged game neither seat spends the Ducats on turn one. Reported and left.

## The Arkwrights draw +1 Research for every Body settled

Ticket [#589](https://github.com/whaleyjoshua2/Dying-Earth/issues/589), spec §3.

A `research_per_body_settled` on the Faction's row, 1 on the Arkwrights', paid flat at Income for
each Body off Earth holding at least one of their Colonists. The designer chose any Colonist over the
Diaspora's four and over ground Colonies only, and a signature clause. The test was written first and
watched red (no "Bodies settled" source, the Research unmoved), then green.

**The picture** ([`ticket-589/arkwrights-card.png`](ticket-589/arkwrights-card.png)): the Arkwrights'
card, the signature ending *"They draw +1 [Research] a turn for each Body they have settled."*

- **The sweep** ([`sweeps/ticket-589.txt`](sweeps/ticket-589.txt)): **28 / 38 / 4 / 9 and 1
  collapse**, from 28 / 37 / 5 / 10 and none: noise. The Arkwrights' Research is 325 a game from 285
  and the Techs they led 193 from 168; the win column does not move. Reported and left.

## Orbital Data Centers at x1.25, and a fourth Tech to choose from

Ticket [#592](https://github.com/whaleyjoshua2/Dying-Earth/issues/592), spec §4.

The Tech's `value` to 1.25 and a `shortlist_bonus = 1` that the draw sums over every Tech done. The
designer chose "whoever leads a pick" over "the seat that led Orbital Data Centers", so no seat owns
it and nothing is saved. A test was written first and watched red (the bonus 0, the list three),
then green; two older pins moved with it: the Observatory test now reads the x1.25 step itself,
since the floored ratio of two whole figures is only exact at x1.5, and the Archivists' chain walk
expects four once the Tech is done.

**The picture** ([`ticket-592/tech-tree-shortlist.png`](ticket-592/tech-tree-shortlist.png)): the
Tech Tree with Orbital Data Centers done and a pick owed, three of the four Pick buttons in view
(Coastal Engineering, Green Consensus, Expanded Habitats). The tree's fourth branch is cut at the
window's fold at 1280x800, so the picture cannot show the fourth button; the test that draws four for
every seat is the proof of it. A new building aid, `shortlist:1` with `pick:0`, draws the owed pick
over the `researched:` Techs for it.

- **The sweep** ([`sweeps/ticket-592.txt`](sweeps/ticket-592.txt)): **24 / 40 / 5 / 11 and no
  collapse**, from 28 / 38 / 4 / 9 and 1. The Custodians' four off is at the edge of noise.
  Observatories 18.9 Research a world-turn from 21.1. Reported and left.
- **Production over the batch, totals**, gross before upkeep (the sweep's new block):

| | Materials | Fuel | Energy | Ducats | Research | Widgets |
|---|---|---|---|---|---|---|
| Custodians | 167,093 | 80,720 | 315,881 | 137,927 | 107,896 | 235,017 |
| Prospectors | 180,403 | 47,464 | 220,737 | 432,185 | 30,556 | 141,465 |
| Arkwrights | 129,760 | 43,150 | 207,976 | 281,050 | 25,860 | 123,749 |
| Archivists | 38,032 | 22,546 | 87,593 | 40,181 | 52,086 | 76,683 |
| all four | 515,289 | 193,880 | 832,188 | 891,343 | 216,398 | 576,914 |
| net of upkeep | 509,381 | 153,752 | 121,381 | 529,675 | 216,398 | |

## The market's price rises as lots are bought and falls as they are sold

Ticket [#593](https://github.com/whaleyjoshua2/Dying-Earth/issues/593), spec §5.

Every unit is priced at the table's running net for its good this turn, a Ducat for every 20 bought
ahead of it and a Ducat off for every 20 sold; the band is 2 either side of the card figure, never
under 1; the step stays 20. The designer chose all three as recommended. One door, `trade_cost`,
prices a trade after the seat's own earlier pending trades, and the window, the orders list,
`remaining`, the order check and the computer's Fuel buy all read it; commit reads the table's net
as it moves. The settle banks where the trading left the price. A test was written first and watched
red (the band at 1), then green; one older pin moved: 22 Materials bought from 100 Ducats leave 32,
the 21st and 22nd at 4.

**The picture** ([`ticket-593/trading-window.png`](ticket-593/trading-window.png)): two pending Buys
of 20 Materials, *"Buy 20 for 60"* then *"Buy 20 for 80"*, and the next Materials quoted at 5. A new
building aid, `buy:<n>`, places the two Buys. The Sell button reads 20.5 for 10, not 25: the sale's
own units walk the price down from 5.

- **The sweep** ([`sweeps/ticket-593.txt`](sweeps/ticket-593.txt)): **26 / 37 / 3 / 12 and 2
  collapses**, from §4's 24 / 40 / 5 / 11 and none: noise. Units bought over the batch 58,018 from
  63,624, sold 77,827 from 73,384. Reported and left.

| | Materials | Fuel | Energy | Ducats | Research | Widgets |
|---|---|---|---|---|---|---|
| Custodians | 183,836 | 86,100 | 339,731 | 145,393 | 115,623 | 248,865 |
| Prospectors | 179,203 | 43,457 | 215,425 | 424,505 | 27,552 | 136,754 |
| Arkwrights | 133,932 | 42,606 | 196,351 | 283,342 | 25,236 | 124,520 |
| Archivists | 42,800 | 24,522 | 91,965 | 47,029 | 54,711 | 80,615 |
| all four | 539,772 | 196,685 | 843,473 | 900,269 | 223,122 | 590,754 |
| net of upkeep | 532,732 | 153,713 | 110,459 | 544,790 | 223,122 | |

**The planning order**, the designer's follow-up: a computer seat now plans after the seats before it
have committed, where every seat planned first and committed after. The review had found that a
later seat was quoted the turn's opening price and paid the price the seats ahead of it had moved;
the sweep, given a count of order lists whose first dropped order was refused for Ducats (the turn
logs only a list's first, so a floor), put it at 216 over the batch with the old order (the same 26 / 37 / 3 / 12 re-run) and 0 with the new. A test was written first and
watched red (the Prospectors scored their actions before the player's Buy was logged), then green.

- **The sweep** ([`sweeps/ticket-593-planning.txt`](sweeps/ticket-593-planning.txt)): **21 / 25 / 8
  / 24 and 2 collapses**, from 26 / 37 / 3 / 12 and 2. The Prospectors' twelve off and the
  Archivists' twelve on are past noise, the nearest board yet to the designer's 15 each. A later
  seat also sees an earlier seat's queued builds and departed Ships as it plans; which of that and
  the prices moved the column is not measured. Reported and left.

| | Materials | Fuel | Energy | Ducats | Research | Widgets |
|---|---|---|---|---|---|---|
| Custodians | 157,583 | 73,561 | 289,496 | 128,069 | 95,139 | 227,545 |
| Prospectors | 169,569 | 40,104 | 214,737 | 438,889 | 30,263 | 136,688 |
| Arkwrights | 120,806 | 34,339 | 193,136 | 305,017 | 25,704 | 120,344 |
| Archivists | 36,441 | 22,810 | 87,553 | 29,456 | 53,501 | 82,623 |
| all four | 484,400 | 170,815 | 784,922 | 901,432 | 204,607 | 567,200 |
| net of upkeep | 478,687 | 132,692 | 111,272 | 532,738 | 204,607 | |

**Taken back: the planning order.** The designer suspected the better board came from seat order
and what a later seat could see. Counted by seat position it did: wins by seat 14 / 38 / 16 / 10
with the old order became 10 / 30 / 13 / 25, and the Archivists' twelve all came from seat 3, where
three of the sweep's four seatings put them. At the designer's word every seat plans before any
commits again, and each seat's trades are priced by its own trades this turn alone; the whole
table's net is banked at the settle and moves the next turn's price. Two tests were written first
and watched red on the planning-order build (the rival quoted the table's moved price; the
Prospectors planned after the player's trade was paid), then green.

- **The sweep** ([`sweeps/ticket-593-own-trades.txt`](sweeps/ticket-593-own-trades.txt)): **25 / 40
  / 3 / 9 and 2 collapses**; wins by seat 15 / 39 / 17 / 6; order lists with a Ducat drop **0**. The
  board is back where the first build left it (26 / 37 / 3 / 12): the seat-order edge was the whole
  of the better spread. Units bought 61,087, sold 70,742.

Income per Faction, mean a game, net of upkeep:

| | Materials | Fuel | Energy | Ducats | Research |
|---|---|---|---|---|---|
| Custodians | 2,166 | 840 | 671 | 1,637 | 1,470 |
| Prospectors | 2,332 | 468 | 290 | 1,176 | 405 |
| Arkwrights | 1,758 | 433 | 366 | 3,795 | 341 |
| Archivists | 521 | 235 | 75 | 512 | 659 |

## The Cheap Ore Offer's refusal: a third off the refusing seat's Materials

Ticket [#597](https://github.com/whaleyjoshua2/Dying-Earth/issues/597), spec §6.

The designer chose the refusing seat's own price over the market's for everyone, one turn, flat while
it stands, and the card's words "a third off". The card's multiplier is two thirds; `lot_ducats`
prices a seat's lots flat at the opening price while it holds a cut on the good; the cut's note reads
"a third off". A test was written first and watched red (the multiplier 0.5), then green; the 0.1.0.3
test and the Market-line test moved from half to a third off.

**The picture** ([`ticket-597/trading-window.png`](ticket-597/trading-window.png)): the Materials row
*"3 each; sells for 1.5 (a third off for you, this turn)"*, and two pending Buys of 20 at 40 Ducats
each, flat. A new building aid, `orecut:1`, gives seat 0 the cut.

**The review** found two defects, both fixed with a test watched red first: with only the Buys flat,
a seat holding the cut sold its cheap Buys back up the price they raised (100 bought for 200, sold
for 221: the test read 221 against 150), so its Sales are flat too; and the card's refuse button
would have read "x0.6666666666666666", so the cut's words come from one engine function. **The
card** ([`ticket-597/card.png`](ticket-597/card.png)): *"Let it go: Materials a third off next turn
/ Materials a third off for you next turn"*.

- **The sweep** ([`sweeps/ticket-597.txt`](sweeps/ticket-597.txt)): **25 / 40 / 3 / 9 and 2
  collapses**, as before: the computer takes the offer whenever it holds 20 Ducats, so the refusal
  seldom fires in a sweep. Reported and left.

| | Materials | Fuel | Energy | Ducats | Research |
|---|---|---|---|---|---|
| Custodians | 2,167 | 841 | 672 | 1,637 | 1,469 |
| Prospectors | 2,332 | 468 | 290 | 1,176 | 405 |
| Arkwrights | 1,757 | 432 | 366 | 3,797 | 341 |
| Archivists | 520 | 236 | 74 | 508 | 658 |

## Every Victory gate is 6% of the whole

Ticket [#601](https://github.com/whaleyjoshua2/Dying-Earth/issues/601), spec §7.

The designer chose all four as recommended: 6% for all four Factions (`gate_weight` in
`victory.toml`), the Archivists' offline Archive costing the same 6%, today's short words, no
partial credit. `Progress` carries `gate_open` and `gate_weight`; `score` and `score_as_it_stands`
both take the gate's share, so the computer reads what the player sees. A test was written first and
watched red (no such field), then green; nine older pins moved by the one rule -- the parts are 94%
of the score, the gate the rest -- and the rival-Moment test's premise rose to 15 of 18 and 4,000 of
5,000 (14 and 3,800 read 72%, under its three quarters). No picture: staging both bars met needs a
shot aid this ticket did not warrant; the Report-head test pins "94% - needs Planetary Stewardship".

**The review** found the computer's door had moved: no seat helps a Faction at 95%, and a seat with
both bars met and no gate, refused at 100% before, read 94% and got through. The door now reads the
parts without the gate (`parts_as_it_stands`); a test isolating it (the runaway-leader rule kept out
of the way) was watched red, then green. It also found the Custodians can still read 100% unwon --
the gate standing, the best run at the bar, the run since broken -- which predates this ticket and
is put to the designer; and stale "90%" comments, corrected.

- **The sweep** ([`sweeps/ticket-601.txt`](sweeps/ticket-601.txt)): **26 / 39 / 7 / 8 and no
  collapse**, from 25 / 40 / 3 / 9 and 2: noise. Reported and left.

| | Materials | Fuel | Energy | Ducats | Research |
|---|---|---|---|---|---|
| Custodians | 2,092 | 795 | 603 | 1,528 | 1,383 |
| Prospectors | 2,255 | 448 | 323 | 1,181 | 391 |
| Arkwrights | 1,785 | 489 | 410 | 3,812 | 381 |
| Archivists | 497 | 234 | 73 | 538 | 672 |

## The Report's lines

Ticket [#600](https://github.com/whaleyjoshua2/Dying-Earth/issues/600), spec §8.

The designer chose the sea's one line with their losses named and rivals' counted, a Your works line
per building, "revolted against", both invasions as the headline, and Planetary Stewardship's effect
as "Natural Sink +1.5 ppm" alone. The sea's loss now returns what each Region lost (`SeaLoss`) and
one function writes the rise's line; `LineKind::Invasion` heads the ranks. The new sea test was
written before the build but first ran after it; the old pins that moved (the ranks, the revolt
words, the sea's per-Region lines, an invasion's kind) each failed on the new code and were moved.
The revolt drops the designer's "the" by the Report's standing rule, which a test enforces.

**The picture** ([`ticket-600/report.png`](ticket-600/report.png)): the Report at the sea's first
threshold, headlined *"Sea rose at +1.8 C: you lost a Power Plant, a Factory and a Mine (China);
rivals lost 5 Facilities."*, Your works listing each of the three. A new building aid, `searise:1`.

**The review** found the Storm Surge's one Region written as a world's rise with no Region named,
"you" read as the director by the sea and the holder by an invasion, "in The United States" in Your
works, and the Ice Sheets Break raising two Moments; all fixed, a test watched red on the first three
(the surge, the name, an occupied rival's Region), then green. The dead `slot`/`slots` phrases went.

No rule moves, so no sweep.

## The help without ticket numbers, and three lines

Ticket [#599](https://github.com/whaleyjoshua2/Dying-Earth/issues/599), spec §9. The first ticket
worked with the `grilling` and `domain-modeling` skills loaded: one round of four questions, the
designer taking the recommendation on each but the tie line, where a tie reads "random among the
tied" and names nobody. Three tests were written first and watched red (the help's history, the old
tie line, "needs 4 Pioneers here"), then green; one older pin moved to "in China". No new term, so
no glossary entry.

**The review** found "in The United States", a lone leader shown a made-up 25% when nothing had
been contributed, and two help sentences left without the colon the cut ticket tag had carried; all
fixed, the first two with tests watched red, and the help test now catches any "#" before a digit.

**The help as printed** ([`ticket-599/help.txt`](ticket-599/help.txt)): no ticket or version
reference, the `-ducats` sentence in the designer's words. No rule moves, so no sweep.
