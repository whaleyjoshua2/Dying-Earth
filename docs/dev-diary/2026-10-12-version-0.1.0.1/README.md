# Version 0.1.0.1, the counterweight version

The map is [Map: version 0.1.0.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/529); the spec
is [`docs/spec/version-0.1.0.1.md`](../../spec/version-0.1.0.1.md). The standing baseline is 0.1.0.0's
closing board after its playtest fixes, **2 / 24 / 15 / 17 and 22 collapses**
([`final-0.1.0.0-after-playtest.txt`](../2026-10-07-version-0.1.0.0/sweeps/final-0.1.0.0-after-playtest.txt)),
against the designer's 15 each and 20.

## Two panels' prose onto hovers, and Armies to low orbit without a Launch Site

Ticket [#532](https://github.com/whaleyjoshua2/Dying-Earth/issues/532), spec §1.

**The pictures** ([`ticket-532/`](ticket-532/)), taken headlessly in `shot:` mode:
- `launch-yes-earth.png`: China's card, "Launch Site: Yes" in green, its hover.
- `launch-dark-earth.png`: the same with the Launch Site dark (`offline:launch`, a new aid), "No" in
  yellow, the hover's second line "Its Launch Site is dark."
- `spaceport-earth.png`: an Arkwrights player's card, "Spaceport: Yes".
- `emissions-hover-earth.png`: the people's sentence gone from the card, its figures the hover of
  the Emissions line.
- `blame-hover-earth.png`: the Climate Panel's Blame lines cut to "Custodians: answerable for 6
  ppm", the hover's two lines on turn one (nothing moved yet, so no middle line).

**Found on the way.** No computer seat had loaded an Army onto an empty Carrier at Earth since at
least #319: the load stood inside the warship block, which a Carrier with no Army aboard never
enters. Moved out of it at the designer's word ("fix it").

**The sweep** ([`sweeps/ticket-532.txt`](sweeps/ticket-532.txt)): **2 / 24 / 15 / 17 and 22
collapses**, the baseline to the game. A few figures elsewhere moved (Sea Wall thresholds held 922
to 941, Ice Sheets Committed 19 to 20 of 20 in one seating), so games did diverge. The column did
not move because a computer seat seldom builds a Carrier at all: it wants one only with cause
(Wary or worse) against the holder of a rival Colony off Earth, every seat alike. Three
logged games (seeds 1-3, the Custodians' seating) built none. Armies landed at a Colony: 0 in every
seating, as before. Measured behaviour, not a rule.

**The review** (an agent that did not build it) found no rule defect and three gaps, all fixed: the
hover's first line showed "removed 0 in credit" at zero; with all four moved parts the middle line
wrapped to a fourth line as drawn (now said short past 55 characters, tested in rendered lines);
and nothing tested that a working Launch Site still lifts an Army into a station's ring or from a
rival's Region (now tested, each watched red by blinding the Launch Site check).

## A Ship's lifts from Regions and its refuel do not use its turn

Ticket [#533](https://github.com/whaleyjoshua2/Dying-Earth/issues/533), spec §2.

**The picture** ([`ticket-533/lift-and-go-earth.png`](ticket-533/lift-and-go-earth.png)), taken with
a new aid, `liftgo:1`: TSV Endeavour in Earth's low orbit with a lift of four Pioneers from China
and a flight to the Moon both ordered, its Move buttons greyed with "This Ship already has an
order" (a change of orbit is the turn).

**Three old tests moved with the rule**, each to the new rule rather than loosened: #443's "never
beside a move" (a lift now stands beside one), #363's "a Refuel after a Launch is refused" (a
refuel now stands beside a Launch or a Rearm; a move after one is still refused), and #87's "3 in
the tank flies nowhere" (the computer now refuels and leaves in one turn, on a leg the filled tank
pays, which the check of its whole order set confirms).

**The sweep** ([`sweeps/ticket-533.txt`](sweeps/ticket-533.txt)), run after the review fixes: **2 / 25 / 12 / 20 and 20
collapses**, from 2 / 24 / 15 / 17 and 22. The Arkwrights' 3 and the Archivists' 3 are at the edge
of noise. The settling moved past it: the first ground Colony on the Moon in 75 of 80 games at a
median turn 18, from 70 at turn 24; Phobos in 36 from 30, Deimos in 66 from 59, Ceres 72 ground
Colonies at the end from 60. Measured behaviour.

## One Launch Site to a Region

Ticket [#534](https://github.com/whaleyjoshua2/Dying-Earth/issues/534), spec §3.

**The picture** ([`ticket-534/second-site-earth.png`](ticket-534/second-site-earth.png)): China's
build list, its Launch Site standing, the Launch Site button greyed with "this Nation State already
has a Launch Site".

**Found in review and fixed:** the Arkwrights' free Launch Site (their Opening Objective's reward)
read only standing buildings, so it could land beside a Spaceport under construction; it passes such
a Region over now, and stays owed if no other will take it.

**The sweep** ([`sweeps/ticket-534.txt`](sweeps/ticket-534.txt), after the review fix): **2 / 26 / 14 /
16 and 20 collapses**, from 2 / 25 / 12 / 20 and 20 after #533. The Archivists' 4 is at the edge of
noise and not traced: the computer never built a second Launch Site before this ticket, and the one
change on its side is that it no longer offers one beside one under construction.

## Greenwash costs Relations

Ticket [#535](https://github.com/whaleyjoshua2/Dying-Earth/issues/535), spec §4.

**Measured before:** the computer greenwashes 5 Influence a time, 8 to 25 times a game (three logged
games), so 40 to 125 Influence; at the decided figures that is the Custodians -1.6 to -5 over a game.

**The pictures** ([`ticket-535/`](ticket-535/)): `hover-earth.png`, the Custodians' own page, the
Greenwash button's hover with "Relations: Prospectors -0.03, Arkwrights -0.1, Archivists -0.05";
`relations-earth.png`, the Prospectors' page with 50 Influence greenwashed (`greenwashed:50`, a new
aid), the Custodians' Neutral in the warning colour, its hover "-1 in all: +0 from what they have
done, +0 from their Blame. / Greenwash -2 / Research shared +1". The last line is new: the
shared-Research reward (#236) was a part of the score the hover never named, which the Greenwash line
made visible because the parts no longer added up.

**The sweep** ([`sweeps/ticket-535.txt`](sweeps/ticket-535.txt)): **0 / 28 / 15 / 16 and 21
collapses**, from 2 / 26 / 14 / 16 and 20, within noise.

**The review** (an agent that did not build it) found two gaps, both fixed: the "Research shared"
line printed the raw reward, where the score caps it at the top of Cordial, so the parts could still
fail to add up (the engine now reports the reward as applied, `directive_relations_applied`, watched
red by handing back the raw figure); and the "never heals" check set the quiet turns without
settling them, so it could not fail (it now settles twenty turns and reads the shown score).

## The Colony Ship card shows aboard and room

Ticket [#536](https://github.com/whaleyjoshua2/Dying-Earth/issues/536), spec §5.

**The pictures** ([`ticket-536/`](ticket-536/)): `earth-crowded-earth.png`, TSV Endeavour at Earth at
+2.6 C, "[glyph] 0/8" and its hover "4 room, +4 crowded at Earth"; `venus-loaded-venus.png`, the
same Ship at Ishtar with four aboard, "[glyph] 4/4"; `roster-earth.png`, the roster row's "0/8"
beside its button. The glyph goes BEFORE the figures, at the designer's word, where the game's
rule for a word after a figure would put it behind, so it is drawn as its own piece.

No rule moved; no sweep.

## The prices: Influence at 4, Strip Permits at 25, Mines and builds

Ticket [#537](https://github.com/whaleyjoshua2/Dying-Earth/issues/537), spec §6.

At the designer's word the figures themselves move, with no multiplier behind them: each price row
x1.1, kept to the tenth (a Habitat 27.5), so the rows now carry tenths; the Earth Mine's row 4.4;
each Body's Mine yield x0.85, Antarctica's among them.

**The pictures** ([`ticket-537/`](ticket-537/)): `china-builds-earth.png`, China's card, "Scrubber
33 / or 66" Ducats and "Build Army 27.5", its Output 6.6 Materials (the Mine's 4.4 x 1.5);
`strip-permit-earth.png`, a Prospectors player's China, "Strip Permit 25" in Ducats. The Influence
bar's hover is drawn with `on_hover_text`, which no headless picture reaches; a test reads it ("at 4
Ducats each.", watched red on the old words).

**Tests.** Fourteen pins were moved to the new figures first and watched fail on the old data; when
the data moved, thirty-nine more failed, each read and set to its new figure. Four fixtures had their premise moved by the figures and were mended, not
weakened: the Sea Wall, Factory Module and Capital fixtures' Materials x1.1 (the AI spent the old
amount on the dearer opening Mine first); the Custodians' idle-Mine tests (a Moon or Phobos Mine
no longer outproduces China's 6.6, so Vesta, and a Moon Mine beside a Mass Driver for the even
trade); the Colony Ship voyage test (at the new Moon yield the Mars system won all twelve boards
with no prize at all, so the test holds the Moon's old 1.65 to keep its question). The Strip Permit
test's refusal at 24 Ducats first passed for the wrong reason (Three Turns Held), found and fixed.

**The sweeps**, one after each move, wins Custodians / Prospectors / Arkwrights / Archivists:

| after | wins | collapses |
|---|---|---|
| before (#535) | 0 / 28 / 15 / 16 | 21 |
| Influence at 4 ([`sweeps/ticket-537-influence.txt`](sweeps/ticket-537-influence.txt)) | 2 / 32 / 11 / 12 | 23 |
| Strip Permit at 25 ([`sweeps/ticket-537-strip-permit.txt`](sweeps/ticket-537-strip-permit.txt)) | 2 / 31 / 13 / 13 | 21 |
| Mines and builds ([`sweeps/ticket-537-mines-builds.txt`](sweeps/ticket-537-mines-builds.txt)) | 2 / 20 / 20 / 22 | 15 |
| after the review's fix ([`sweeps/ticket-537-after-review.txt`](sweeps/ticket-537-after-review.txt)) | 1 / 20 / 20 / 20 | 19 |

Against the ideal of 15 each and 20: dearer Influence alone helped the Prospectors (measured: the
Arkwrights and Archivists lost about four wins each); the Permit's price moved nothing past noise;
the Mines and builds took eleven wins off the Prospectors and gave them to the Arkwrights and
Archivists. The Custodians are still at 2, and collapses fell to 15, five under the target.
After the review's fix, 1 / 20 / 20 / 20 and 19: three Factions even, the Custodians at 1.

**The review** (an agent that did not build it) found one defect and gaps, all fixed. The defect:
the computer's scarcest-resource reckoning floored its cheapest builds at the literal 20, the old
cheapest row; at 22, a seat under 22 Materials matched only the free Core Module and stopped
finding Materials scarce. The floor is the cheapest paid row now (tested, watched red: Fuel where
Materials was wanted). The gaps: the Strip Permit test never read that the 25 Ducats were taken
(now it does, watched red); the `strip:1` aid gave no Ducats; three comments said the old 25. It
also named the voyage test's held Moon yield, which is said above: at the shipped figures the Mars
system wins every board, prize or none. The market's 20-Material reserve (`ai.toml`) is a
keeping figure, not a price, and is left.

## A popup when an Accord ends, and a warning the turn before

Ticket [#538](https://github.com/whaleyjoshua2/Dying-Earth/issues/538), spec §7.

**The pictures** ([`ticket-538/`](ticket-538/)), Report menus in `shot:` mode with two new aids:
`ended-report.png` (`moment:accord`: the Prospectors call off an Accord with the player), the
Moment "Your Accord with Prospectors has ended: Prospectors called it off." under the Prospectors'
name in their colour; `warn-report.png` (`accord:warn turns:4`), Your works with "Your Accord with
Arkwrights ends after this turn." and two more: on this board each computer seat had struck an
Accord with the player on turn 1, so all three reach their last turn together.

The warning reads "ends after this turn", where the ticket's draft said "ends next turn": it is read
at the head of the last turn, and the Accord is gone by the next.

**Tests.** Four, each watched red by taking its piece out (no ending Moment, no warning, the cap of
two applied to it, no rival clause) and green restored. The Report's word ceiling rises by the 46
words this adds. No rule moved; no sweep.

**The review** (an agent that did not build it) found two small defects, both fixed: a partner's
refusal to renew was never forgotten, so an Accord renewed or added to after one, and later let run
out, read "would not renew" (now cleared wherever its run starts afresh; tested, watched red); and
an Accord declared over in a save from before this version credited its first seat with calling it
off (now it names nobody). Two tests added at its word: an ending fires once when it is called off
in its last turn, and two rivals' ending is under the fog (watched red with the fog lifted).

## The Report: a window next turn, your Regions lost to the sea, the clock and your Victory

Ticket [#544](https://github.com/whaleyjoshua2/Dying-Earth/issues/544), spec §8. Energy left was
dropped at the designer's word ("math is not hard").

**The picture** ([`ticket-544/`](ticket-544/)): `window-report.png` (`turns:5 menus:1 reportopen:1`,
seed 7), turn 6: under the date "On this path Collapse is not reached · Your Victory: 3%", and under
Ships "The Mars, Phobos and Deimos window is next turn: 5 turns, 18.1 Fuel."

**Measured, for the designer:** on seed 7 the turns before a window fall at Mars 6; Venus 8 and 17;
Mercury 2 and 4; Ceres and Vesta 3 and 11. Mercury's window comes about every other turn, so its
line will be a frequent one.

**Tests.** Three, each watched red by taking its piece out (the line on the window's own turn, the
sea line not marked the player's, the head with the clock alone) and green restored. A heading test
listed the five headings without Ships, which its board never had; Ships is listed in its place. The
word ceiling rises by the 33 words added. No rule moved; no sweep.

**The review** (an agent that did not build it) found one serious defect and two small, all fixed.
The window line pointed at its world, and the fog drops a Ship line at a world the player cannot
see, so the player with nothing at Mars, the one it is for, never read it; my test read the raw
Report. It is placeless now, and the test reads the Report under the fog (watched red: none
shown). The head's percentage rounded half up where the Victory windows round half to even; it
uses their rounding now. And the first Report never announced a window on turn 2; it does now
(untested: seed 7 has none on turn 2). Left as it is: on a game-over Report the clock reads "Cuts
alone no longer avoid Collapse", as the Climate Panel does.

## Before the order: the founding rule and the Pioneers' words

Ticket [#543](https://github.com/whaleyjoshua2/Dying-Earth/issues/543), spec §9.

**The pictures** ([`ticket-543/`](ticket-543/)): `short-moon.png` (`settler:moon aboard:3`, a new
aid), TSV Endeavour's "3/4" in yellow and its hover "4 found a Colony"; `recruit-earth.png`, the
Recruit hover "2M people, Unrest -0.25."

**After the pictures, at the designer's word.** The Recruit hover first said "No Launch Site here:
low orbit only" inside its old sentence; the designer: *"mouse over is too damn wordy"*, and the
Launch Site line already says it, so the hover is its two figures. The Influence preview ("Standing
4 of 30" beside Spend; the approved "→" was not in the font) was built and photographed, and the
designer changed their mind: it is gone from the window and the text tool.

**Found on the way.** The "Buy more Influence" hover still said "two a point", missed on the prices
ticket; it reads the table now.

**Tests.** The refusal's new words, the short flag and the hover's two figures, each watched red,
and the Ship card's own pins moved to the new hover. No rule moved; no sweep.

**The review** (an agent that did not build it; the tree changed under it) found no rule defect: a
doc comment spliced onto the wrong function, fixed; and a point for the designer, not changed: the
station's refusal "needs 4 Pioneers here" shows on the orbit's Build button, where "here" reads as
the slot, and with several Launch Site Regions it does not say which.

## A rival spending on yours, and your home at risk

Ticket [#542](https://github.com/whaleyjoshua2/Dying-Earth/issues/542), spec §10. At the designer's
word the home warning is a line, not a Moment.

**The picture** ([`ticket-542/`](ticket-542/)): `rival-report.png` (`rival:spend menus:1`, a new
aid: the Prospectors spend 12 on China, holding a Standing two spends short), Your works with
"Prospectors spent 12 on China: 60 of 69." and "China could fall to Prospectors next turn.", the
command cluster's "China is Pressed by a rival." beside it.

**Found on the way.** The game kept no home for a seat once it was seated; each seat now records the
Region it started in. And the bar a line reports is the one after the turn: the holder's own
Standing decays in the same Resolution, so 70 before the turn was 69 after it (the test first
expected 70).

**Tests.** One, watched red before the build (no line at all) and on its risk arm (the spend not
counted). The word ceiling rises by the 15 words added. No rule moved; no sweep.

**The review** (an agent that did not build it) found no blocking defect. Fixed: the test never
read the home on its own, since China was both home and the only Launch Site (now a second site
leaves the home alone to warn, watched red with the home unset, and a place taken that very turn
says nothing more); and the only-Launch-Site count read the Regions the player directs, where the
spend reads those it holds (now both read held). For the designer, not changed: "next turn" reads
the bar as it stands, so the holder's own Embassy rise or decay next turn can make it a false
alarm or a miss; and in the rare exact tie between two rivals a held-off rival reads "70 of 69".

## The text tool: offer Terms, every Standing, Leapfrog

Ticket [#541](https://github.com/whaleyjoshua2/Dying-Earth/issues/541), spec §11. The text tool alone;
the window had all three.

**Checked in the tool**, seed 7: `help` reads "leapfrog <state>  the Custodians only, 50 Ducats: the
Region's people emit 0.03 less per hundred million, for good, never below 0.04"; Japan's listing,
after a spend of 5, "Standings: Custodians 4"; and on the turn three Accords were offered, each
printed its Terms above its answer line ("Prospectors offer an Accord: non-aggression."). The first
build printed only the first offer's; every offer prints now. No picture: the tool is text. No rule
moved; no sweep.

**The review** (an agent that did not build it) found one defect and two small gaps, all fixed: a
renewal printed as a fresh Accord offer (now in the window's words, the dropped research agreement
said; unexercised, since a player is not sent a renewal since #509); the Leapfrog help left out the
Baseline cut of 0.1 (now said); and a comment named the wrong ticket.

## The Arkwrights' Victory text on stations over Earth

Ticket [#540](https://github.com/whaleyjoshua2/Dying-Earth/issues/540), spec §12. The ticket's draft,
"(Earth is not one of the four)", would have dropped the half that was true: Antarctica's people
live on Earth and count for nothing. The clause keeps it: "(Antarctica does not count; Earth is not
one of the four)".

**Tests.** One, reading the text and the engine side by side: watched red on the old text, and its
four-Bodies arm watched red with Earth let into `bodies_settled`. No picture: the words are the
data's, read by the Victory window as they stand. No rule moved; no sweep.

## The computer races for the Research Lead

Ticket [#530](https://github.com/whaleyjoshua2/Dying-Earth/issues/530), spec §13. At the designer's
word the race scales with how near the seat's Victory stands, in its builds and in what it sends to
the Sink.

**The sweep** ([`sweeps/ticket-530.txt`](sweeps/ticket-530.txt)), against 1 / 20 / 20 / 20 and 19
before: **6 / 23 / 20 / 18 and 13 collapses**. The bar is 15 each and 20; missed, reported, stopped.
Measured, not a rule: the gates moved most. Median turn and games completed, before and after:

| | before | after |
|---|---|---|
| Custodians | turn 26, 75 of 80 | turn 15, 76 of 80 |
| Prospectors | turn 25, 79 of 80 | turn 27, 80 of 80 |
| Arkwrights | turn 28, 73 of 80 | turn 32, 66 of 80 |
| Archivists | turn 26, 77 of 80 | turn 31, 69 of 80 |

The Custodians' road is the shortest, so they race first and take their gate eleven turns sooner;
the others, losing the Lead to them, come to theirs later and in fewer games. Collapses fell by six,
consistent with Planetary Stewardship's larger Sink standing earlier (not measured directly).

**Tests.** Three new, each watched red (the share held at nought, the Sink share unscaled, the old
wait put back); two old tests that pinned the turn-33 wait moved to the new rule. No rule moved.

**The review** (an agent that did not build it) found no blocking defect. Fixed: `gate_pick_turn`
was left a dead setting (removed, its comment reworded); the race's second way in, the gate shut
and the first part half way, was untested (now tested, watched red), and one check compared nought
with nought. Noted for the designer: the Lab weight reaches Schools and Institutes too, which the
computer has weighed as Labs since #185.

## The computer seats gang up on a runaway leader

Ticket [#531](https://github.com/whaleyjoshua2/Dying-Earth/issues/531), spec §14. The designer
redrew it at the start: *"I did not mean armies I meant seats"*. The Army gathering and the allied
melee rule the ticket asked about are dropped.

**The sweep** ([`sweeps/ticket-531.txt`](sweeps/ticket-531.txt)), against 6 / 23 / 20 / 18 and 13
before: **7 / 24 / 19 / 19 and 11 collapses**, within noise. The bar (15 each and 20) is missed;
reported, stopped.

**Measured, to say why** (a throwaway probe, twelve all-computer games, removed after): a seat stood
at half its Victory or more on 189 of 433 turns, and some seat pressed a leader on 162. The pack is
at work about a third of the game; as weighted -- x(1 + pressure), at most x2 -- its acts do not
turn the column.

**Tests.** Two new, each arm watched red (the cause, the lift, the Accord and Trade refusals, the
eased bar, the credit refusal); the Accord check first read a raw score of -5 that shows as -4, and
its -3 case could not fail, both mended to set the shown score.

**The review** (an agent that did not build it) found no defect that breaks correctness, and that no
computer judgement binds the player. Fixed: a doc comment spliced onto the wrong function; the
credit refusal matched on words the Hostile refusal shares; the two refusals had no control (now
the same offer from a seat not the leader is taken). For the designer, not changed: the eased bar
reaches Refuel too (Cold at full pressure; every offer carries Non-aggression, so it changes no
offer), and two rivals tied at the top are both leaders. Measured cost: the pressure reads every
seat's Victory each call, a constant factor; the sweep ran in 20 seconds.

## The close

Ticket [#545](https://github.com/whaleyjoshua2/Dying-Earth/issues/545). `GAME_VERSION` 0.1.0.1;
`SAVE_VERSION` stays at 12, and the old-save test now also cuts a seat's Greenwash spent and home
Region and an Accord's `ended_by` and `refused_by` (watched red with the Greenwash field's default
taken away; an `Option` reads None with or without one, so it could not be the witness). The seat's
home field is named `home_region`, where it was `home`: an Army's `home` shares the save's text, and
the old-save test's cut took both.

**The closing sweep** ([`sweeps/final-0.1.0.1.txt`](sweeps/final-0.1.0.1.txt)): **7 / 24 / 19 / 19 and
11 collapses**, against 0.1.0.0's 2 / 24 / 15 / 17 and 22. The spec's head carries the win column
after every ticket. A station's cap at 18 was dropped at the designer's word.
