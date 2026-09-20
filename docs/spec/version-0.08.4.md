# Dying Earth — version 0.08.4, the rivalry version: Blame as a ledger that rivals can write on, carbon credits from the Custodians, the Smear and Agitate, a Sea Wall that stands, a Fund of 2500 behind a slider, and five ways of reading the other three seats

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.08.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/254), and the
pictures and measurements that decided it are in
[`docs/dev-diary/2026-09-19-version-0.08.4/`](../dev-diary/2026-09-19-version-0.08.4/).

**What the version is.** Version 0.08.3 made the four Factions different from one another and
ended with the Prospectors winning **30 games of 80**. This version is about what the seats can do
*to each other*, and what they can see of each other. Four of its rules act on a rival — a Smear
lays Blame on one, a carbon credit moves Blame off one, Agitate raises a rival-held Region's Unrest,
and Blame now sets how fast a Standing erodes — and five of its changes are ways of reading a rival:
a Moment when one nears its Victory, a challenger line on every held card, an *Under way* block, a
Victory history chart, and a motto. Underneath, **Blame became a ledger** (§11, §12, §13, §14): ppm
emitted less ppm removed, then moved by what rivals lay on it and what credits take off it, so the
share every rule reads can diverge from what a Faction physically put in the air. The rest is the
designer's housekeeping asked for in the same breath: a Sea Wall that is never destroyed, a Fund
bar of 2500 behind a proper slider, off-Earth Events held back until turn 12, glyphs on a founding,
a colour defect eleven versions old, and names for Armies.

**What it did to the win column**, 20 seeds across four seatings at the shipped climate cell
(sink 6, step 300), against the 0.08.3 baseline:

| Faction | 0.08.3 | 0.08.4 |
|---|---|---|
| Custodians | 26 | **26** |
| Prospectors | **30** | **28** |
| Arkwrights | 8 | **7** |
| Archivists | 1 | **0** |

**The column did not move.** Four tickets act on rivals; three of them landed, by the
measurements, on the Prospectors — a seventh of their Blame at a game's end is now Smear, they buy
back 37 to 70 ppm a game in credits, and their Fund's bar rose by a quarter — and the fourth,
Agitate, turned out to be *their* weapon. They won two fewer games of eighty, which is within the
noise of a reshuffled deck (§5). It was neither a correction nor a pile-on: the seat that runs away
still runs away, and the seat that had one win has none. The designer decides below whether that
ships.

Collapses rose from **15 of 80 to 19 of 80**. A rising collapse rate is the Collapse Line doing
its job. The likely cause, measured but not proven: a Sea Wall that stands keeps a state's coastal
Facilities working — the sea drowns 12 a game where it drowned 20 (§3) — and working Facilities
emit, so the median temperature at the end is a tenth of a degree higher in every seating (+2.98,
+2.96, +2.93 and +2.25 against +2.83, +2.85, +2.89 and +2.15). The whole Tech Tree completes in 20,
20, 14 and 15 of 20 seeds by seating at median turns of 29, 29, 36 and 31, against 20, 20, 16 and
17 at 29, 28, 36 and 26.

**One ticket on the designer's list was cut** before it was built: renaming Ships, Colonies and
Space Stations ([#271](https://github.com/whaleyjoshua2/Dying-Earth/issues/271)), *"just a huge lift
for a small rp bump"* — it would have been the game's first text field, with keyboard focus, a
Colony name distinct from its slot, uniqueness and the save all hanging off it. It is ruled out of
scope on the map and may come back as a fresh ticket.

---

## 1. A held Colony wears its holder's colour

*Ticket [#255](https://github.com/whaleyjoshua2/Dying-Earth/issues/255).*

A defect eleven versions old, fixed in one file. The scene's marker materials are built at start-up
in Faction order and were looked up **by seat**, so seat 0's Colonies, Control ring and Ship stack
markers were always Custodian teal, whoever sat there — invisible only when the player is the
Custodians, which the default, the tutorial and every capture without a `player:` aid are. Every
lookup is by the seat's Faction now, and the stack markers are coloured each frame rather than at
spawn. Photographed before and after on one shot line: a Prospector Colony at Olympus Mons,
(52, 150, 140) teal and then (189, 118, 57) orange.

---

## 2. The Prospectors' Fund: a slider in whole percents, a Withdraw button with a field, a bar of 2500

*Ticket [#256](https://github.com/whaleyjoshua2/Dying-Earth/issues/256).*

The Fund's share is a full-width **slider in whole percents**, the Research Directive's rail, with
the top fifth painted over at the 80% cap — *"the entire point of the slider is to allow finer
control"* — so `share_step` is a hundredth where it was a tenth. The draw is a button reading
**Withdraw** beside a field for the amount, 1 to the Fund's balance; a tenth is still lost on the
way out. The bar is **2500 Ducats**, a deliberate stretch past the measured median of 2049 — *"other
things will bolster their output"*.

*"Adjust the AI as well — the whole victory condition should be 2500, not just a graphical change
to the bar."* The computer Prospectors read the bar from the card and walk the finer step, so they
moved with the data, and a test proved it: with the bar at 2500 and an income of 120 a turn they max
their share where they used to pick a modest one. Seven tests went red on the data change alone and
were rewritten against 2500. A stale Report card that said the Fund drew *Materials* — a version
after the Fund moved to Ducats — now says *withdrew N Ducats*.

Measured at the close: the Prospectors' Fund ends at a median **2588 and 2604 Ducats** in the two
seatings where they hold East Asia, meeting the bar in 15 and 16 of 20 seeds, and at 503 and 293 in
the two where they do not, meeting it in none.

---

## 3. A Sea Wall stands, and grows dearer to keep

*Ticket [#257](https://github.com/whaleyjoshua2/Dying-Earth/issues/257).*

*"Raising sea levels no longer destroy sea walls."* The literal reading, over the breach-and-repair
recommended: a working Sea Wall **holds every threshold and is never destroyed**. Each rise it has
held adds **half a Material a turn** to its keep, paid at Income out of the Materials the seat
makes, the half carried between turns so nothing is lost to rounding; a seat that cannot pay leaves
the wall standing but **unkept that turn, holding nothing**, and the Report says so. The count is
per wall: a wall built after two thresholds have already taken the coast starts at nothing.
Displacement and Unrest still pass through the wall, as they have since ticket #52.

A **Storm Surge** that breaks on a standing wall no longer brings the next threshold forward: the
wall holds, and the state's coastal Facilities make **30% less at the next Income**, once. An
unwalled state takes the threshold early, as it always has. A surge is weather, not a rise, and adds
nothing to the keep. The Report line, from the designer's words: *"The Sea Wall in China took the
sea at +1.8 C; the risen water makes it dearer to keep, 0.5 Materials a turn now."* No AI change:
the computer raises a wall when a threshold is within 0.2 C and now stops there, since the wall
stays.

Measured at the close: walls built over a seating fell from **1047, 698, 697 and 694 to 478, 330,
312 and 397**, of which 184, 134, 148 and 168 stand at the end, having held 641, 468, 380 and 257
thresholds between them. The sea takes a median 22, 25, 29 and 17 coastal slots a game against 32,
33, 31 and 15, and drowns 12, 13, 17 and 6 Facilities against 20, 22, 19 and 6.

---

## 4. The yields at a founding go to glyphs, on the panel and on the button

*Ticket [#258](https://github.com/whaleyjoshua2/Dying-Earth/issues/258).*

Both lines on the slot panel — the slot's yields and the Body's beneath it, the Body's kept weak —
draw a glyph and then a figure. So does the founding button on a Ship's card, which *"needs
yeilds"*: the ticket had wrongly said it already had them. The one glyph rule trades a word for its
glyph only where the word follows a figure, and the yield line put the figure after the word, so the
button ticket #218 built had drawn **words** since 0.08.2 and nobody had photographed it. A new
`slot_yield_row` draws glyph then figure directly, shared by the panel and both founding doors, so
it cannot fall through the rule again. Two shot aids, `site:` and `settler:`, so both can be seen.

---

## 5. Events that can only land off Earth join the deck on turn 12

*Ticket [#259](https://github.com/whaleyjoshua2/Dying-Earth/issues/259).*

Neither of the shapes offered. *"Those seven, flag them as off earth and add them to the deck on
turn 12."* The seven cards that can only land off Earth — Grid Failure, Reactor Leak, Dust Storm,
Moonquake, Helium-3 Vein, Rich Seam and Ice Deposit, twelve with their copies — carry
`off_earth = true`, are **not dealt at the start**, and on **turn 12** are shuffled once into
whatever remains of the deck, with a Report line saying so. A card drawn with nowhere to land is
still spent, as it always was. A save from before this version joins them on its next Event phase
past turn 12.

Measured over five games before the change and five after: draws that found nowhere to land fell
from **4.2 a game to 2.4**, and what is left is mostly Dust Storm, which wants somebody living on
Mars. At the close, over 80 games, **2.8 a game**. **The same seeds are no longer the same games**:
the deck's shuffle comes off the seeded dice, and a deck of 28 shuffled at the start and 40
reshuffled on turn 12 draws differently from a deck of 40 shuffled once, so every later roll moves
and the closing sweep's seed-for-seed comparison with 0.08.3 carries that noise.

---

## 6. A motto for every Faction

*Ticket [#260](https://github.com/whaleyjoshua2/Dying-Earth/issues/260).*

The Custodians: **Leave it better than we found it.** The Prospectors: **Everything has a price. We
find it.** The Arkwrights: **Nothing left behind but the Earth.** The Archivists: **Everyone
remembered.** Two from the proposed column, two from the alternatives; none is a quotation, so none
carries a credit. It sits under the name, italic, in the Faction's colour, before the blurb, on the
selection card and the Faction window's rulebook — *"card/rule book only"* — which one shared
function gives for nothing. It is a field on the Faction's card in `factions.toml`, and a guard test
fails with it absent.

---

## 7. The rival's Moment

*Ticket [#261](https://github.com/whaleyjoshua2/Dying-Earth/issues/261).*

A ninth kind of Moment, for **rivals only**, in **two steps**: a rival's score reaching **three
quarters**, and **one part of its Victory Condition met with the other short** — the step the engine
can state exactly, where the suggestion's "one turn from" wanted a projection it does not make. Each
step fires **once per seat**, latched and saved. **Rank five**, between a decisive Battle and a Tech
completed. The gate Tech's own Moment is left as it was. The figure is **the part still short**,
since that is what the player can still act on, in the rival's colour: *"9 of 12 — The Prospectors
are three quarters of the way to their Victory Condition: Off-world Presence at 9 of 12."* A part
held back by its gate Tech is not a part met: the Fund full with the Extraction Charter unresearched
does not fire the second step. The share is `rival_moment_share = 0.75` in `victory.toml`.

---

## 8. The challenger line

*Ticket [#262](https://github.com/whaleyjoshua2/Dying-Earth/issues/262).*

On every Region, Colony and station the player **holds**, one line in the Standings block naming
the rival **nearest its own price** — not the highest Standing — and how far it is from taking the
place: *"The Prospectors stand at 47; they take this at 70."* The arithmetic is on the hover: the
greater of the challenger's own threshold, Blame's multiplier inside it, and the holder's Standing
plus the challenge margin that pair faces, Relations and Constabulary included. *"No rival has a
Standing here"* when nobody does.

**The picture found a line already there.** Ticket #75's top-of-card warning named the rival with
the highest Standing and priced the place at Standing plus margin, which has not been the rule since
ticket #60; where Blame or Relations move a rival's price it was wrong, and could name the wrong
rival. Its two virtues — the amber, and *Spend here to stay ahead* when the rival is within two
steps — were folded into the new line, ticket #161's decay sentence into the hover, and the old line
removed. One line, one arithmetic, on every held card.

---

## 9. What this Faction has under way

*Ticket [#263](https://github.com/whaleyjoshua2/Dying-Earth/issues/263).*

An **Under way** block under Holdings on every Faction's page, the list **in full for rivals too**,
since a build stands hatched on its card and a transit is drawn on the Solar System Map for anyone
to see. Two lines under a bold heading — *Building: a Mine at Mare Tranquillitatis on the Moon
(1 turn), a Factory in China (3 turns)* and *In transit: TSV Vanguard, Earth to Mars, 3 turns* —
**soonest first**, name breaking a tie; *Nothing under way* when there is nothing. Pioneers waiting
or at sea are not listed. `Game::under_way` reads every build the seat **ordered**, so a build begun
in a Region since lost stays with whoever paid for it. The picture found the interface font has no
arrow, so the road reads *Earth to Mars*.

---

## 10. The Victory history

*Ticket [#264](https://github.com/whaleyjoshua2/Dying-Earth/issues/264).*

The first per-Faction history the game keeps: one record a seat per Climate phase — the score, the
Blame share, whether the gate Tech is done, the Archive complete and Antarctica open — written
beside the Emissions record so the two charts share an axis, saved with the game, empty on a save
from before. Drawn on the Faction window under Victory progress on **every Faction's page**, at the
population chart's size: **one line** for the score, and Blame as **the share** on its own scale at
the right with the fair quarter marked, both on the whole 0-to-1 range so a height means the same
thing on turn 3 as on turn 30. Ticks on the axis: the Breaks in red as every sibling has them,
Antarctica's opening in blue on every chart, the gate Tech on the Faction's own, the Archive on the
Archivists'. The axis is labelled with in-game dates, as every chart's is.

---

## 11. Blame and Blame credit: what each is

*Ticket [#265](https://github.com/whaleyjoshua2/Dying-Earth/issues/265).*

Measured first: **no seat held a Blame credit in ten games of ten** under the old definition, a
surplus of removed over emitted — the Custodians scrub a third to a half of what they emit, never
more. So **the credit is now what a Faction has removed**, full stop, and the word has a figure in
every game and a carbon credit (§13) has a supply. Blame is unchanged in itself: emitted less
removed, never below nothing — and then a ledger, §12. What the Custodians' Research Directive adds
to the Natural Sink **counts as their removal** every Climate phase it stands, since that
enlargement takes that much out of the air every turn; the recommendation to leave the Sink the
world's was overruled. The panels read *answerable for N ppm (emitted E, removed R in credit)*, and
one hover on the Blame heading names what Blame is, what the credit is, and the two rules that read
Blame. The ticket #53 test that pinned the old surplus was rewritten.

Measured at the close: the Custodians hold a median 9, 57, 119 and 176 ppm in credit by seating.

---

## 12. Blame moderates the decay of a Standing — and Blame is a ledger

*Ticket [#266](https://github.com/whaleyjoshua2/Dying-Earth/issues/266).*

**The designer set out the model on this ticket, and it governs §13 and §14 too:** Blame is a
ledger — *"ppm emitted − ppm removed ± credits bought and sold (in ppm) ± propaganda campaign. In
this way actual share will diverge from blame."* The share every rule reads is computed from the
ledger, not from the physics. It is in the glossary under **Blame**.

On the decay: *"both"* — a dirty Faction's Standing erodes faster and a clean one's slower — as a
**step rule**, because a multiplier rounded to a whole number could only ever bite at the cap and
never slow anything. On a Region the Faction **does not hold**, a Blame share **at or below an
eighth** decays **1** a turn, **at or above a half** decays **3**, and anything between the plain 2.
A held place keeps its 1. A Colony or a station is untouched: Blame is Earth's resentment. The four
figures are `decay_slow_below`, `decay_fast_from`, `decay_slow` and `decay_fast` in
`influence.toml`. This reverses, on purpose, the *"never on Standing decay"* that ticket #53 wrote
into the Blame rule in version 0.05; the data comment says so.

A consequence the suite found: with no Blame on the table at all — turn 1, before the first Climate
phase — every seat's share is nought, so everyone's Standing on Regions they do not hold decays 1 on
the first turn, not 2. The rule is read as written.

---

## 13. The Smear: Influence lays Blame on a rival

*Ticket [#267](https://github.com/whaleyjoshua2/Dying-Earth/issues/267).*

**Smear**, over the Denounce recommended — *"because it inflates blame above strictly ppm produced
and thus kind of a lie"*, which is exactly why the ledger and not the physics is what every rule
reads. **1 Influence lays 2 ppm** on the target's ledger (`[smear] ppm_per_influence` in
`influence.toml`), **for good**, shown as its own figure so nobody is told they put it in the air:
*…removed 0 in credit, 80 laid on by rivals*. One campaign a turn per target, any amount the
Allotment covers, never oneself, from the rival's page in the Faction window. An offence at an
Influence push's weight, and the target's Report names who paid: *The Custodians smeared the
Prospectors: 10 ppm laid on their Blame.* The computer seats use it: a seat Cold or Hostile toward
a rival whose share stands above the fair quarter proposes one step of Influence against it,
competing with a place for the same Allotment.

Measured at the close: the Prospectors end a game with a median **140 and 170 ppm** of smear on a
ledger of 813 and 815 in the seatings where they hold East Asia, and 90 and 110 on 560 and 280
elsewhere; the Arkwrights carry 60 and 40 in the two seatings where they are dirty enough to draw
it. Nobody smeared the Custodians or the Archivists.

---

## 14. Carbon credits from the Custodians

*Ticket [#268](https://github.com/whaleyjoshua2/Dying-Earth/issues/268).*

A fourth line in the Trading window. A ppm bought **comes off the buyer's ledger for good and off
the Custodians' credit**; **unbacked** — *"any amount and take the blame"* — so the Custodians may
sell more than they hold, and the excess goes onto their own ledger as Blame taken. *Forcing them to
sell only what they hold would deprive them of any reason to get credit, and forbidding overselling
would gimp a strategic choice.* **One Ducat a ppm**, multiplied by the Custodians' view of the buyer:
Friendly ×½, Cordial ×¾, Neutral ×1, Wary ×1½, Cold ×2, **Hostile refuses**. **10 ppm a turn per
buyer**, and the cap never reads the seller's credit. A purchase is an act of friendship both ways.
The figures are `[carbon_credits]` in `factions.toml`.

*"I want them to refuse sometimes."* The computer Custodians set a **standing offer**: their whole
credit while their own share of the table's Blame is under the fair quarter, **nought when it is
not**, and the cap on top when they are clean and short of Ducats. A Custodian player sets the offer
in the same window with a field, and may oversell as the computer may. An offer is shared first come
first served; a buyer left short gets its Ducats back for what it did not get, and the rest land in
the Custodians' Stockpile. The computer seats buy with an appetite keyed on their share above the
quarter and their Ducats.

An older test, *relations do nothing mechanical in this version* (ticket #191), went red: with every
pair Hostile the Custodians refuse to sell. Its premise had been dead since 0.08.2; it is kept
inverted, as the record of the first version in which Relations were mechanical to the computer
seats.

Measured at the close: the Custodians sold **733, 1408, 1448 and 630 ppm** over a seating of 20
games, every ppm of it bought — by the Prospectors in every seating (37 to 70 ppm a game), and by the
Arkwrights in the one seating where they start rich enough to be dirty.

---

## 15. Agitate

*Ticket [#269](https://github.com/whaleyjoshua2/Dying-Earth/issues/269).*

An order on a Region a rival holds: **15 Ducats and 5 Influence** — *"both"*, dearer than Relief's
10 so a duel is not a coin flip settled by income — raise its Unrest by **one point**, halved by a
working Constabulary as a climate card's rise is. **One a turn per Region per Faction**, so a Region
cannot be bought off its holder in one turn. An offence at an Influence push's weight, and the
holder's Report names who paid: *The Prospectors agitated in Nigeria: Unrest rose by 1 to 4.5.* The
figures are `agitate_ducats`, `agitate_influence` and `agitate_points` in `unrest.toml`. The
computer seats use it against holders they are Cold or Hostile toward, most eagerly on a Region
past its second threshold, competing with Relief, Influence and the rest for the same Ducats.

**A consequence recorded on the ticket:** Unrest falls 1.5 a turn on its own, after every rise, so
one Faction's single Agitate a turn on a *calm* Region nets out to nothing but the Relations it
costs. Agitate moves a Region that is already restive, or one that two or three rivals lean on
together. Whether the natural fall should spare an agitated Region that turn is in the map's fog.

Measured at the close: **428 Agitates landed over 80 games**, 5 a game — 371 of them by the
Prospectors, 97 by the Arkwrights in the one seating where they start rich, a handful by anyone
else. The seat with the Ducats is the seat that agitates.

---

## 16. Names for Armies

*Ticket [#270](https://github.com/whaleyjoshua2/Dying-Earth/issues/270).*

Every Army is named as it is raised, **from its home** rather than from a list: an ordinal and the
Region's demonym — *the 1st Chinese Army*, *the 2nd Chinese Army* — or for a Colony's, its Garrison
— *the Tycho Garrison*, *the 2nd Tycho Garrison*. No randomness is drawn. *"Standing armies too"*:
the one the game begins with is the 1st, and a Standing Army raised again after it is destroyed
takes the next number, since a Region counts every Army it has ever raised. The name is the Army's,
not any Faction's, and survives every change of hands. A `demonym` sits on every Region's card in
`nation_states.toml`. The roster, both cards' Army lists, the Army orders block and a Carrier's cargo
line all read the name. The Report's own lines still say *Army*; they are the next place to reach.
The name is a field, so a rename control could reach it; none does, with #271 cut.

---

## 17. Eleven figures the sweep could not say

*Ticket [#272](https://github.com/whaleyjoshua2/Dying-Earth/issues/272).*

The `sim` example prints, per seat, Blame in ppm, its removal, its credit, the ppm laid on by
Smear, and carbon credits bought and sold. The sweep aggregates them — Blame ppm, credit and smear
as medians by seat, credits bought and sold as totals — and adds Agitates landed per seat, Sea Walls
standing at the end beside walls built and the thresholds they held, Events drawn with nowhere to
land, and the seeds in which the Fund meets its bar. *Sea Walls spent* had counted the line *"was
destroyed"*, which nothing writes now; it counts *"took the sea"*.

---

## What the closing sweep says

Run as `sweep 20 --balance --seatings --steps=300`; the output is
[`docs/dev-diary/2026-09-19-version-0.08.4/sweeps/final-0.08.4.txt`](../dev-diary/2026-09-19-version-0.08.4/sweeps/final-0.08.4.txt).

**The win column is in the header of this document.** Beside it:

- **Three of the four rival-facing rules land on the Prospectors, the fourth is theirs, and none
  dented them.** The Smear adds a seventh to their ledger and the credits they buy take a twentieth
  to a twelfth off it, so their Blame share at the end is 0.71 and 0.73 where they hold East Asia
  against 0.68 and 0.68 — the share every rule reads has barely moved. Agitate is *their* weapon:
  of 428 landed over the batch, they landed 371, against the Custodians and Archivists who resent
  them. Their Ducats rose — a median 1575 and 4573 made a game against 1361 and 4057 — and the bar
  at 2500 is met in 31 of the 40 seeds where they hold East Asia, where the median stood at 2049
  and 2082 against 2000 before. What was aimed at them, they paid for.
- **The Archivists' one win became none**, for 0.08.3's reasons and no new one: they keep back 96
  to 97% of their Research and sit under the 85% line for 531 to 620 turns of a batch.
- **Collapses rose, 15 of 80 to 19 of 80**, and the world ends a tenth of a degree hotter in every
  seating. The likely cause is §3's: walls that stand keep coastal Facilities working, and they
  emit. Measured, not proven — nothing was run with the wall rule alone reverted.
- **Relations run colder.** 117, 106, 133 and 109 of 240 ordered pairs end at Cold or worse against
  107, 106, 109 and 87, and 40 to 59% carry a scar against 41 to 48% — the Smear and Agitate are
  offences, and the computer seats use both.
- **Nothing was re-fitted on this sweep.** The candidates, if the designer wants one: Agitate's
  price (5 a game, all of it from the two rich seats), the Smear rate (a seventh of a dirty ledger),
  the credit price (every offer sells out). Each is one figure in `assets/data`.
