# Dying Earth — version 0.1.0.4, the storyteller version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/583), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-10-21-version-0.1.0.4/`](../dev-diary/2026-10-21-version-0.1.0.4/).

**The baseline** is 0.1.0.3's closing board,
[`ticket-582.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-582.txt):
**26 / 41 / 5 / 8 and 0 collapses**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.3 with the designer's list of 2026-10-21, most of it making the
game tell its own story in plain words, beside the figures that move. The summary and the win column
are written at the close-out.

## 1. The Prospectors' Venture Fund at 5,000

Ticket: [The Prospectors' Venture Fund at 5,000](https://github.com/whaleyjoshua2/Dying-Earth/issues/584).

The designer: *"Prospectors to fund to 5k"*. The Prospectors stood at 41 wins of 80 on the closing
board, the farthest seat from the ideal, and 0.1.0.3's move to 4,250 (§11 there) had not bitten:
the computer Prospectors bank to their pace, and the Fund at the end rose by about the bar's fifth.

- **The bar is 5,000** (`victory_first.bar`, 4,250 x 20/17). The card's long and short texts read
  the bar, so they say 5,000; the glossary and the playtest note say it.
- **The computer Prospectors' pace** scales with it on the same turns, at the designer's word:
  667 / 1,667 / 3,000 / 5,000 at turns 12 / 25 / 37 / 47 (`[pace.prospectors] first`); the test
  that holds every pace schedule's last figure to its bar passes. The banking share, read from the
  bar each turn, asks a sixth more Fund at every turn.
- **Pins that moved with the bar**: the Victory tests' margins (6,000 for a 1.2 margin, 2,500 for
  half, 850 for 17%), the rival-Moment figure (3,800 of 5,000, from 3,230 of 4,250) in the tests and
  in the headless shot's `rival:1` aid, and the turn-one line.
- **Swept** (`sweeps/ticket-584.txt`, 20 seeds x four seatings): **30 / 36 / 3 / 9, 1 collapse and
  1 draw**, from 26 / 41 / 5 / 8 and 0. The Prospectors' five off and the Custodians' four on sit at
  the edge of noise (a move of about 3 is noise); the Arkwrights at 3 from 5 and the Archivists at 9
  from 8 are noise. Their gate still completes in 79 of 80 games at a median turn 27 (80 at 27
  before). **The bar did not bite again**: their Fund at the end by seating is median 5,200 / 5,064
  / 4,112 / 4,808 of 5,000, from 4,425 / 4,298 / 4,075 / 4,308 of 4,250, up by 500 to 775 in three
  seatings and unmoved in the third, since the pace table rose with it and the computer banks
  harder to meet it. Reported and left; the Fund is a pace the computer sets to the bar, so a bar
  alone does not slow it.

## 2. The Arkwrights and the Archivists open with 10 Ducats

Ticket: [The Arkwrights and Archivists open with 10 Ducats](https://github.com/whaleyjoshua2/Dying-Earth/issues/585).

The designer: *"Arkwrights and Archivists start with 10 duckets"*. They stood at 3 and 9 wins of 80
on §1's board, the Arkwrights the farthest seat below the ideal.

- **A `start_ducats` on a Faction's row**, read by `Tables::start_stockpile` in place of the `[start]`
  table's 0 as `start_materials` is (0.1.0.3 §22), and refused when negative. The Arkwrights' and
  Archivists' rows say 10; the Custodians and the Prospectors open with the table's 0.
- **The card** says it on its opening glyph line alone, *"Opens with 80 · 20 · 20 · 10"* with the
  Ducat glyph, which already showed the Ducats wherever there were any; the signature texts do not
  repeat it, at the designer's word.
- **The computer** needs nothing: the stockpile is the seat's. In a logged game (seed 7) neither seat
  spends the Ducats on turn one; both sell Fuel and keep them.
- **Swept** (`sweeps/ticket-585.txt`): **28 / 37 / 5 / 10 and no collapse**, from §1's 30 / 36 / 3 /
  9, 1 collapse and 1 draw. The Arkwrights +2 and the Archivists +1 are noise; so is the rest. The
  gates: the Arkwrights' in 65 of 80 at a median turn 30 (70 at 31), the Archivists' 68 at 30 (70 at
  29). Ten Ducats on turn one do not move the board. Reported and left.

## 3. The Arkwrights draw +1 Research for every Body settled

Ticket: [The Arkwrights draw +1 Research for every Body settled](https://github.com/whaleyjoshua2/Dying-Earth/issues/589).

The designer: *"Arkwright's get +1 research for every body they've settled"*. Their Research was the
lowest of the four (285 a game on §2's board, against the Custodians' 1,372).

- **A Body settled** is one off Earth where the seat controls at least one Colonist,
  `bodies_settled(seat, 1)`: a station over a Body counts as there, Earth and its orbit never. At the
  designer's word, over the Diaspora's four a Body and over ground Colonies only.
- **The figure** is `research_per_body_settled` on the Faction's row, 1 on the Arkwrights', nothing
  elsewhere. It is flat, added to the turn's Research after every multiplier and before a Research
  agreement's lift, and the income's sources name it *"Bodies settled"*.
- **The card** says it in the signature, at the designer's word: *"They draw +1 Research a turn for
  each Body they have settled."*, the word drawn as its glyph.
- **The computer** needs nothing: the Research lands where it is read.
- **Swept** (`sweeps/ticket-589.txt`): **28 / 38 / 4 / 9 and 1 collapse**, from §2's 28 / 37 / 5 /
  10 and none: noise. The Arkwrights' Research is 325 a game from 285 and the Techs they led 193
  from 168; their gate completes in 66 of 80 at a median turn 30, from 65 at 30. Reported and left.

## 4. Orbital Data Centers at x1.25, and a fourth Tech to choose from

Ticket: [Orbital Data Centers at x1.25, and a fourth Tech to choose from](https://github.com/whaleyjoshua2/Dying-Earth/issues/592).

The designer: *"Orbital data centers at x1.25 and now provide 4 tech options to winner to choose from
going forward"*.

- **x1.25** on an Observatory's Research, from x1.5 (`value`).
- **Four to choose from**: once Orbital Data Centers stands, every shortlist is drawn at four,
  **whichever seat leads the pick**, at the designer's word (over the seat that led the Tech itself,
  since the tree is shared and every seat holds it). A `shortlist_bonus` on a Tech's row, 1 on this
  one, sums into the draw's size; the Lead's gate or next rung still takes one place, and a free
  choice still opens when no more than the size are available. No saved field.
- **The effect text**: *"Observatory Research x1.25; every pick from 4 Techs"*.
- **The computer** picks from whatever list it is drawn; nothing to add.
- **Swept** (`sweeps/ticket-592.txt`): **24 / 40 / 5 / 11 and no collapse**, from §3's 28 / 38 / 4 /
  9 and 1. The Custodians' four off is at the edge of noise, the rest noise. Observatories make 18.9
  Research a world-turn from 21.1; the Archivists' gate completes in 72 of 80 at a median turn 30,
  from 73 at 29. Reported and left.

## 5. The market's price rises as lots are bought and falls as they are sold

Ticket: [The market's price rises as lots are bought and falls as they are sold](https://github.com/whaleyjoshua2/Dying-Earth/issues/593).

The designer: *"Check the way the market floats price changes - it should increase when large lots
are bought and decrease when sold"*. It floated once a turn, one Ducat at most, inside a band of 1,
and every lot of a turn cost the same.

- **Every lot re-prices**, at the designer's word: each unit is priced at the **seat's own** running
  net for that good this turn (bought less sold by that seat), a Ducat for every `price_step_units`
  (20) from where the turn opened. A Buy of 40 Materials at 3 costs 20 x 3 + 20 x 4; a Sale of 40
  fetches half of 20 x 3 + 20 x 2.
- **Another seat's trading moves nothing a seat is quoted this turn**: every seat orders blind of
  the others, so the whole table's net is banked at the settle and moves the next turn's price.
  The first build priced each unit at the whole table's running net, and a seat that committed
  after a big buyer paid a price it had not been quoted (216 order lists over a sweep had their
  first dropped order refused for Ducats; the turn logs only a list's first). Planning each
  computer seat after the seats before it had committed cured the drops and handed the last seat
  to plan an edge: wins by seat 14 / 38 / 16 / 10 became 10 / 30 / 13 / 25, the Archivists' gain
  all from seat 3 (`sweeps/ticket-593-planning.txt`). Both were taken back, at the designer's word,
  for this rule: every seat plans before any commits, as before.
- **The seat's pending trades count**: the Trading window quotes the next unit and the next lot
  after them, the orders list prices each trade after the ones ahead of it, and `remaining`, the
  order check and the commit pay them as quoted (`trade_cost`).
- **The band is 2** either side of the card figure (`price_band`, from 1), **never under 1**:
  Materials and Fuel 1 to 5, Energy 1 to 4. **The step stays 20 units**.
- **The settle** banks where the table's trading left the price (a step per 20 of net, where it
  moved one step at most); a quiet turn still drifts it one step home. A card's override stands flat.
- **The Price column's hover**: *"+1 Ducat for every 20 you buy this turn, -1 for every 20 you
  sell; up to 2 either side, never under 1. Everyone's trading moves next turn's price."*
- **The computer** prices every trade the same way, and its Fuel buy is checked after its earlier
  trades. Its at-or-above-base sale check reads the opening price, and a big sale walks itself down
  from there. The sweep counts order lists with an order dropped for want of Ducats.
- **Swept** (`sweeps/ticket-593.txt`): **26 / 37 / 3 / 12 and 2 collapses**, from §4's 24 / 40 / 5
  / 11 and none: noise. Units bought 58,018 from 63,624, sold 77,827 from 73,384. Reported and left.
- **Swept again** with the planning order (`sweeps/ticket-593-planning.txt`): **21 / 25 / 8 / 24
  and 2 collapses**, from 26 / 37 / 3 / 12 and 2; order lists with a Ducat drop **0, from 216**.
  The Prospectors' twelve off and the Archivists' twelve on are past noise.
- **Swept with each seat priced by its own trades** (`sweeps/ticket-593-own-trades.txt`), every
  seat planning before any commits: **25 / 40 / 3 / 9 and 2 collapses**; wins by seat 15 / 39 / 17 /
  6; order lists with a Ducat drop **0**. The planning order's 21 / 25 / 8 / 24 was the seat-order
  edge, as the designer suspected: without it the board is back where §5's first build left it.

## 6. The Cheap Ore Offer's refusal: a third off the refusing seat's Materials

Ticket: [The Cheap Ore Offer's refusal cuts the market's Materials price by a third](https://github.com/whaleyjoshua2/Dying-Earth/issues/597).

The designer: *"Cheap ore refusal now cuts market price by 1/3"*.

- **The refusing seat's own price**, at the designer's word, over the market's for everyone: the
  card's `seat_price` multiplier is two thirds, where it was a half (#577). One turn, as before.
- **Flat while it stands**, both ways: the seat's Materials Buys and Sales do not move with its own
  trading that turn, as a price a card has set does not (`lot_ducats`, `sale_price_after`); a
  rival's do. The review found that with only the Buys flat, cheap Buys sold back up the price they
  raised: 100 bought for 200, sold for 221.
- **The words**: the refuse side *"Let it go: Materials a third off next turn"*; the effect line
  *"Buy the ore, or let it go for a third off Materials next turn"*; the refuse button's effect
  *"Materials a third off for you next turn"* (`seat_cut_phrase`, shared with the driver, where it
  would have printed x0.6666666666666666); the window and the Market line *"(a third off for you,
  this turn)"*.
- **The computer's** answer stands: it takes the lot when it holds 20 Ducats.
- **Swept** (`sweeps/ticket-597.txt`): **25 / 40 / 3 / 9 and 2 collapses**, as §5's last build; the
  computer takes the offer whenever it can pay, so the refusal seldom fires in a sweep.

## 7. Every Victory gate is 6% of the whole

Ticket: [Every Victory gate is 6% of the whole: never 100% while a gate is held](https://github.com/whaleyjoshua2/Dying-Earth/issues/601).

The designer: *"Never show 'Your Victory: 100%' while a gate is held ... every victory gate should
count for 6% of the victory"*.

- **The gate is 6% of the percentage** (`gate_weight = 0.06` in `victory.toml`, one figure for all
  four), whole or nothing, no partial credit for Research toward it. The parts share the 94% left in
  their old proportions (the Arkwrights' 0.6 / 0.3 / Capital 0.1 become 56.4 / 28.2 / 9.4).
- **What the gate is**: the Faction's gate Tech standing and, for the Archivists, no complete
  Archive standing offline; both cost the same 6%, at the designer's word. A missing Colonial
  Capital is still a part of its own.
- **The words**: today's short form on the lower figure, *"94% - needs Generation Ships"*; the
  Arkwrights with both bars and the gate but no Capital read *"91% - needs a Colonial Capital"*.
- **The computer** reads the same score (`score_as_it_stands` takes the gate too), except at **the
  door**: no seat helps a Faction whose parts stand at 95% (`parts_as_it_stands`, without the gate),
  so a seat with both bars met and no gate is refused a Trade or Accord as it was at 100%; the
  review found the 6% had let it through at 94%.
- **Swept** (`sweeps/ticket-601.txt`): **26 / 39 / 7 / 8 and no collapse**, from §6's 25 / 40 / 3
  / 9 and 2: noise.
- **Not changed, for the designer**: the Custodians' first part counts their best run, so with
  Planetary Stewardship standing, the best run at the bar and the run since broken, they read 100%
  unwon, with no hold to name. It predates this ticket.

## 8. The Report's lines

Ticket: [The Report's lines: the sea in one line and your loss in Your works, "Japan revolted", the invasion as headline, "Natural Sink +1.5 ppm"](https://github.com/whaleyjoshua2/Dying-Earth/issues/600).

- **The sea, one line a rise**: *"Sea rose at +1.8 C: you lost a Power Plant, a Factory and a Mine
  (China); rivals lost 5 Facilities."*, the player's losses named by Region, the rivals' counted, a
  Region nobody holds in neither; *"no buildings lost"* where nothing was. Built after every
  Region's loss at the threshold (`report_sea_rise`), one Moment with it; the per-Region lines and
  the window's sea fold are gone. A Sea Wall that held keeps its own line. A Region of the player's
  that lost only slots still makes the line theirs. **Whose** is the Region's holder, as an invasion
  reads it: a Region the player only occupies is not theirs, one a rival occupies still is. A Storm
  Surge's one Region reads *"Sea rose early in Australia: you lost a Factory."*; the Ice Sheets
  Break's rise carries no second Moment beside the Break's own (the review).
- **Your own losses**: a Your works line for each building, *"The sea took your Power Plant in
  China."*
- **The revolt**, both the passed-and-thrown-off and the plain throw-off: *"Japan revolted against
  Arkwrights: now neutral."* The designer chose "against the Arkwrights"; the Report's standing rule
  (a Faction's name drawn in its colour, never after "the") drops the article.
- **The invasion headline**: a rival taking a Region the player holds, or beginning an Occupation of
  it, is `LineKind::Invasion`, rank 1, above a Colony founded; every other rank a step down. A Region
  the player only occupies is not theirs to be invaded.
- **Planetary Stewardship's effect**: *"Natural Sink +1.5 ppm"*, alone, at the designer's word.
- No rule moves; no sweep.

## 9. The help without ticket numbers, the -ducats line, the tie line, the Pioneers' Region

Ticket: [The help without ticket numbers, the -ducats line, the tie line, and the Pioneers' Region named](https://github.com/whaleyjoshua2/Dying-Earth/issues/599).

- **The tool's help** carries no ticket or version reference (21 cut); the rules' words stand.
- **`-ducats`**: *"buy a building outright for Ducats, twice its Materials' worth"*.
- **The Research Lead line** (the tool every turn, the Tech Tree's hover): *"Next pick: random among
  the tied."* or *"Next pick: Custodians (41%)."*, the other shares dropped; the Archive-funding
  sentence after it where it stands. A leader with nothing contributed yet (just after a Tech
  completes) reads *"Next pick: Arkwrights."*, no share (the review). The Tech Tree's share row is
  unchanged.
- **The Pioneers' refusal**: *"needs 4 Pioneers in China"* ("in the United States" mid-sentence)
  where Pioneers wait in one of the seat's Regions; *"needs 4 Pioneers waiting in a Launch Site Region of yours"* where none does.
- No rule moves; no sweep.

## 10. Every computer seat races for Mars

Ticket: [Every computer seat races for Mars](https://github.com/whaleyjoshua2/Dying-Earth/issues/596).

The designer: *"Race to mars"*.

- **A second race, opening once the Moon is claimed** by anyone, ending with the first ground Colony
  on Mars itself (Phobos and Deimos are Bodies of their own), for **every computer seat**. While a
  seat holds no Colony Ship free or bound for Mars, built or on order, its Shipyard and Colony Ship
  are weighed at **2.5** times their base (`mars_race_weight`), and its **runner** -- one loaded
  Colony Ship, none of its others flying to Mars or loaded ahead of it -- is sent to Mars where its
  tank pays the crossing now or at the window and it lands before the game ends. Its other Ships go
  where the ranking sends them.
- **The window**: the computer's soft preference, at the designer's word -- full weight at the window,
  a quarter at the far side of the cycle; a crossing the tank cannot pay is refused and the Ship goes
  when it can. A hard hold to two turns of the window was built (the designer had been told, wrongly,
  that the computer already did this), swept, and taken back.
- **The Colony Ship's tank is 32**, from 35, for every seat: Earth to Mars at the window (18) is
  untouched; off it four Mars turns of fifty close, Earth to Vesta closes and the Arkwrights' Moon to
  Mercury, until a tank Tech stands.
- **The runner's send** to Mars is weighed at the race weight, 2.5, as its Shipyard and Colony Ship
  are, at the designer's word. At its own weight, falling off the window (4.9, 4.3, 3.7), the send
  lost every turn to the Ship's own 6.0 moves between low orbit and the station, and the runner
  never left.
- **The prize** is First to a Body's as it stands (Mars: 15 Influence, +1 a turn, the world eased).
- **Until the Moon is claimed every seat's Ship goes to the Moon**, the runners-up too, at the
  designer's word: a seat does not know it will lose the Moon.
- **Swept** (`sweeps/ticket-596-race-send.txt`): **32 / 33 / 4 / 10 and 1 collapse**, from 26 / 39 /
  7 / 8 and none before the ticket. **Mars founded in 80 of 80, from 28, at a median turn 22, from
  24** (15 / 17 / 23 / 23 by seating): the Mars windows fall on turns 7, 20, 33 and 46 and the Moon is
  claimed at a median turn 8. The other far Bodies settle less, accepted at the designer's word:
  Phobos 56 of 80 from 76, Deimos 16 from 44, Mercury 65 from 77, Ceres 10 from 30, Vesta 6 from 37.
- **Built and taken back**, at the designer's word:
  - *A hold to two turns of the window*: 27 / 42 / 4 / 7, Mars 80 at 24, Phobos 33, Deimos 9, Vesta 7.
  - *The race without the runner's send* (the first commit, `sweeps/ticket-596.txt`): 32 / 34 / 6 /
    7 and 1 collapse, Mars 80 at 23.
  - *The race open from the first launch for the Moon*: no seat ever sees one, since every seat
    plans before any commits and the Moon is a turn away; nothing moved.
  - *The race on from turn one*, each seat's first Ship to the Moon and its second lifted: no seat
    holds a Colony Ship before turn 7, so the turn-7 window went unmet; 23 / 40 / 8 / 8, Mars at 23.
  - *Second to the Moon goes to Mars*, a seat whose first Ship stood behind a rival's sending it to
    Mars: Mars at a median turn 15 and the far Bodies back (Phobos 71, Vesta 31), but the Custodians
    gave the Moon to the Prospectors, 19 / 51 / 3 / 7 and none.

## 11. A neutral Region's threshold rises for every Region the taker holds

Ticket: [A neutral Region's threshold rises 5 for every Region the taker holds](https://github.com/whaleyjoshua2/Dying-Earth/issues/591).

The designer: *"scale with the number of Regions you already hold"*.

- **A neutral Region's threshold rises 1 for every Region the taker holds past its first**
  (`threshold_per_region_held`), at the designer's word, for every Faction alike. The home Region
  counts as held; an Occupied Region does not. One held is +0, three +2.
- **Flat**, added after Green Consensus and Blame. **Neutral Regions only**: a held Region's price,
  and every Colony's and station's, is untouched.
- **The card** adds the term to the breakdown line, hidden at +0: *Threshold 42 - 20 + 20 for size 2
  + 2 for Regions held*. The definition hover reads *"20, plus 10 a size step and 1 a Region you hold
  past your first."* Blame's note on the card leaves the term out of Blame's gap.
- **Swept** (`sweeps/ticket-591.txt`): **18 / 45 / 5 / 11 and 1 collapse**, from 32 / 33 / 4 / 10
  and 1. The term falls on the Custodians' Victory: a Scrubber stands only in a Region they hold,
  and with fewer Regions the Sink no longer outruns the Emissions for the four phases of a
  Stabilization run. Scrubbers built 2,325, from 2,570.
- **Swept and taken back**, at the designer's word:
  - *+5* (`sweeps/ticket-591-plus5.txt`): 7 / 57 / 4 / 12 and none; Scrubbers 2,047.
  - *+2* and *+3*: 9 / 52 / 1 / 17 and 1; 8 / 51 / 5 / 16 and none.
  - *+5 with the Custodians left out*, my misreading of the designer's answer
    (`sweeps/ticket-591-custodians-exempt.txt`): 35 / 34 / 2 / 9 and none.

## 12. People in Earth orbit count for no Victory, and grow faster

Ticket: [People in Earth orbit count for no Victory, and grow a third faster](https://github.com/whaleyjoshua2/Dying-Earth/issues/588).

The designer: *"populations in orbit around earth no longer count toward victory conditions but
population growth there is 1/3rd faster"*.

- **Earth orbit** is every station over Earth but Earth L4 and L5. People there count for no
  Victory: the Custodians' and Prospectors' Presence (18), the Arkwrights' 42, the tiebreak at the
  last turn and the computer's pace all read people **beyond Earth orbit**. L4 and L5 count. Every
  other reader of the people off Earth -- the Observatory, Regolith Printing, the world's count, the
  end-of-game table -- is unchanged. The bars stay at 18 and 42, at the designer's word.
- **Growth in Earth orbit is 3% a turn** (`colony_growth_earth_orbit`), from 2%, at the designer's
  word ("an even .03"); still capped by Habitat room, still declining when the Core is off. L4, L5
  and every other Body grow at 2%.
- **The Archive stands on the ground of another Body**, at the designer's word: no station will do,
  where one over another Body did. Measured before, the computer had never put it on one (40 games).
  The refusal: *"the Archive stands on the ground of another Body"*.
- **The words**: "living beyond Earth orbit" on the three Victory lines, the Archive's lines ("on
  the ground beyond Earth orbit"), the Arkwrights' part ("Colonists beyond Earth orbit"), both
  tiebreak hovers, and the tutorial's turn-4 note (*"People in Earth orbit count for no Victory:
  eighteen living beyond it is half your Victory Condition."*).
- **The Archivists' computer has no people pace** (`colonists = []`), at the designer's word: their
  Victory counts no one living anywhere, and the pace, reading nobody beyond Earth orbit at turn 1,
  pulled them from the Archive.
- **Swept** (`sweeps/ticket-588.txt`): **35 / 32 / 2 / 11 and none**, from 18 / 45 / 5 / 11 and 1.
  The system fills faster: the Moon at a median turn 6 from 8, Mars at 13 from 22, Deimos in 42 of
  80 from 19, Ceres 23 from 10. The Arkwrights score a median 0.31 at the end, from 0.84, last in
  50 of 80; their 42 stands at the designer's word, their recovery a pass of its own. Archives
  standing: 44 of 80, from 60.
- **Swept and taken back**: *the Archivists paced in people beyond Earth orbit*
  (`sweeps/ticket-588-archivists-paced.txt`): 25 / 46 / 2 / 6 and 1; Archives 32 of 80.

## 13. The Region card's lines: "Standing 40 (yours)", "nobody has any yet" cut

Ticket: [The Region card's lines: "Standing 40 (yours)", the calm line, "nobody has any yet"](https://github.com/whaleyjoshua2/Dying-Earth/issues/598).

- **The tool's Standing**, on every Region and Colony row: *"Standing 40 (yours)"* on a place the
  seat holds, *"Standing 40 (yours, the Arkwrights occupy it)"* on one held in name, and *"Standing
  12, 60 to take it from the Prospectors"* / *"Standing 0, 40 to take it"* elsewhere, where it read
  "your Standing 40, you hold it on 40 Standing, over which a rival must climb by the challenge
  margin" and "... 60 needed to take it ...".
- **The tool's Standings line** lists every Faction as before, and is left out where nobody has any,
  where it read "Standings: nobody has any yet".
- **The window's Standings row**, where no Faction has Standing, is the threshold alone (*"Threshold
  40"*): the "Standings:" label, the dot and "nobody has any yet" with its hover go.
- **The calm line** was already gone (#573): nothing below the Army threshold, on the card, in the
  tool and in the Report. The designer's "show only when not calm" stands as the rule.
- No rule moves; no sweep.

## 14. The Custodians' Stabilization meter

Ticket: [The Custodians' Stabilization meter: your Emissions, the world's, the Sink](https://github.com/whaleyjoshua2/Dying-Earth/issues/595).

The designer: *"The Custodians need a visible plan. Show a 'Stabilization meter': your emissions,
the world's, the Sink same place the stock pile meter is shown for prospectors/archivists."*

- **The meter**, on the top bar's second row where the Archivists' and Prospectors' funds stand: a
  bar the funds' size in the Custodians' colour, filled by the Sink over the world's Emissions last
  turn (full when the world is under the Sink); then the run alone, *"0/4"*, at the designer's word
  after the review ("too many characters"). Built first as the Emissions glyph and *"41 (yours 6) ·
  Sink 8 · run 0 of 4"*, and before that with the word "Emissions", which did not fit at 1280.
- **The hover**, on the bar and the run: *"The world emitted 40.7 ppm last turn; you 5.8. /
  The Sink took 8.0 (Scrubbers 0). / Four turns in a row under the Sink is Stabilization: run 0 of
  4."*
- **The tool**: a Custodians' line after the Research Lead's, *"Stabilization: 32.7 ppm over the
  Sink (Emissions 40.7, yours 5.8, Sink 8.0), run 0 of 4"*. The Victory window's and the Climate
  Panel's Stabilization lines are unchanged.
- The Arkwrights still have nothing in that place; a line in the map's fog.
- No rule moves; no sweep.

## 15. Turn one opens on the signature moves

Ticket: [Turn one opens with each Faction's signature moves, in place of the Report](https://github.com/whaleyjoshua2/Dying-Earth/issues/594).

The designer: *"When game starts on turn one provide a popup Name each factions signature moves on
turn 1 ... replace turn one report with this overview"*.

- **The window**, *"Signature moves"*: the four Factions, each its glyph, its name in its colour and
  its line, the player's own first and its line strong; a "Go on". A new game opens on it in the
  Report's place; a tutorial game then shows its first note, which hands on to nothing. The Report
  is not raised on turn one; the Report button still opens it.
- **The lines** (`opening_moves` on each Faction's row), the designer's: Custodians *"Build Scrubbers
  and Nature Reserves; Leapfrog your Regions"* ("their Regions" on a rival's screen); Prospectors
  *"Investment Banks + Venture share"*; Arkwrights *"Colony Ships to four Bodies"*; Archivists
  *"Labs + Directive; Archive"* -- "on the Moon" cut at the designer's word, since the Archive may
  stand on any Body's ground beyond Earth orbit.
- **No home Region on the rows**, at the designer's word: a Faction's home is where the map opens,
  and any Faction may start anywhere.
- **The tool**: `new` prints the four lines, the player's marked "(you)", in the Report's place;
  `show` still prints the Report.
- The Faction selection card is unchanged.
- No rule moves; no sweep.

## 16. The Upload as a story moment

Ticket: [The Upload as a story moment: a line naming the place and its people, and "10 remembered here"](https://github.com/whaleyjoshua2/Dying-Earth/issues/587).

The designer: *"Make the Upload a story moment: a line naming the place and count ('Ten of Mare
Imbrium's people were read into the Archive'), and the colony card keeps '10 remembered here'."*

- **The Report line** names the place by its slot alone and the people read in, as a numeral:
  the player's *"4 of Olympus Mons's people were read into the Archive."*, under Your works; a
  rival's *"Archivists read 4 of Olympus Mons's people into the Archive."*, under In space, with no
  "The" before the Faction's name, as every Report line since ticket #463. The "{total} in all" is
  gone; the card and the Victory line carry it. It never headlines, and there is no Moment.
- **The card**: *"10 remembered here"* in place of "Uploaded: 10 of 16", *"0 remembered here"*
  before the first; its hover *"Of 16 for Victory."*
- **The Upload button's hover**: *"Read into the Archive; 8 stay. Cannot be undone."*, the figure
  the people living at the place now.
- **The rival paragraph** ("uploaded 4 Colonists at ...") is unchanged, at the designer's word, and
  so is the log line. **The tool**'s Archive line ends "10 remembered here".
- Once Colonies carry names, the line reads the name; that belongs to the naming ticket.
- No rule moves; no sweep.
