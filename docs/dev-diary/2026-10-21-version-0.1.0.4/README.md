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
