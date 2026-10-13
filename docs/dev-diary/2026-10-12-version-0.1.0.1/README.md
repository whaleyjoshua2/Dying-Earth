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
