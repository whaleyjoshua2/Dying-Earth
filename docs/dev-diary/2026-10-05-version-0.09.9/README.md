# Version 0.09.9, the settlement version

The map is [Map: version 0.09.9](https://github.com/whaleyjoshua2/Dying-Earth/issues/488). The
standing baseline is 0.09.8's closing sweep: 9 / 27 / 18 / 7 (Custodians / Prospectors / Arkwrights
/ Archivists) and 17 collapses in 80 games.

## Four Colonists to found (#489)

Every new Colony or station opens with four Colonists, however it is made: a Colony Ship's Unload,
Pioneers by sea, or a build from a Launch Site, a Colony or a station, which gives them up and keeps
as many as its Modules, never fewer than four. The Arkwrights start with four Pioneers, from two.
Spec section 1.

**The pictures.**

- [`ticket-489/arkwrights-station-turn-one.png`](ticket-489/arkwrights-station-turn-one.png): the
  Arkwrights on turn 1. Earth's card offers "Build Orbital Reef here 20 [Materials] 4 [people]",
  live.
- [`ticket-489/station-needs-four.png`](ticket-489/station-needs-four.png): the Custodians on turn 1,
  nobody waiting. The same button greyed, its hover "needs 4 [people]".

**The sweep** ([`after-489.txt`](sweeps/after-489.txt)): 0 / 24 / 33 / 6 and 16 collapses, from
9 / 27 / 18 / 7 and 17. Far past noise.

- **Stations off Earth at the end** over the batch: 36, from 94. The Moon 1 from 26, Deimos 6 from 40,
  Venus 29 from 25.
- **The Arkwrights' Opening Objective** (a Colony on the Moon) is met in 55 of 80 games, from 29.
- **The Custodians win none.** Traced on seed 1 with the Arkwrights in seat 0: on both builds the
  Custodians have twelve off Earth by turn 8; on 0.09.8 they hold Stabilization on turns 26 to 28 and
  win, here they never do and the Arkwrights win on turn 30. The two games part on turn 10 through a
  different Tech pick, then different cards and fewer Materials (47 against 77 on turn 12), fewer
  Regions (10 against 12 on turn 22) and fewer Scrubbers (6 against 9 on turn 28). A chain, not a
  single cause;
  **not traced further.**

**A lock found and fixed.** Run once with the Arkwrights starting at two Pioneers, they won 1 game in
80 and met their objective in 0 to 6 games a seating. The computer recruits only with room to put
people; with no station, a station-less seat had none, so it never recruited the four a station now
takes and never built one. The computer now counts the station it could build as room for four.
At two Pioneers with the fix: 5 / 23 / 24 / 5 and 23 collapses. At four, as shipped, the fix changes
no game: 0 / 24 / 33 / 6 and 16.

**The review** (two agents, standards and spec, neither of which built it) found: a Load or Send
Down could order people a build had already claimed, and fail quietly at the Resolution; a founding
settled four whatever had left; the UI decided which orders take people; a long refusal; the
computer recruiting four for a station with no Launch Site to build it from; a short founding
skipped with no line. All six fixed, the first two with tests watched failing first. Kept: four
from a Colony that changed hands in the turn are lost, not handed to its captor. The sweep after
the fixes is the same to the line.

**The sweep's command:** `cargo run --release -p dying-earth-engine --example sweep -- 20 --seatings
--balance`.

## Colony tiers: Outpost, Settlement, Colony (#490)

Every Colony and station stands at a tier, an Outpost to start; its Module slots are the lower of
its Colonists and the tier's cap, 6, 12 or 18; the next tier is a build through its Widgets queue,
30 Materials and 4 Widgets at 12 Colonists, 50 and 6 at 18. Spec section 2.

**The pictures.**

- [`ticket-490/outpost-card.png`](ticket-490/outpost-card.png): the ISS on turn 1, "ISS over Earth
  Outpost", Modules 1 of 2, and after the free box the dashed red Upgrade tile, "Settlement" under it.
- [`ticket-490/upgrade-needs-twelve.png`](ticket-490/upgrade-needs-twelve.png): its hover, greyed:
  "needs 12 [people]", then "Upgrade to a Settlement: 30 [Materials] and 4 [Widgets]. Up to 12 Modules."

**The sweep** ([`after-490.txt`](sweeps/after-490.txt)): 3 / 26 / 26 / 4 and 20 collapses, from
0 / 24 / 33 / 6 and 16 after the founding rule. The Arkwrights' fall of seven is past noise, not
traced. The sweep gained a line, places at the end by tier: 929 Outposts, 81 Settlements, 22
Colonies of 1,032.

The Report's word ceiling rose by two, to 1,783, for a rival's "upgraded {colony}".

**The review** (two agents, standards and spec): a Module refused at the tier's cap still blamed the
people ("one for each of its 14 Colonists"); it now says "full: upgrade to a Settlement for more",
test watched failing first. Also fixed: the free tile's hover stated the old rule; the Modules
hover wrote "one a Colonist" where the data holds the figure; the tiers check sat under another
ticket's comment; the queue test written three times. Kept: a tier build under way when the place
changes hands completes for the new holder, as a Module does. The sweep is the same to the line.

## The Prospectors' Bank paying interest off Earth (#491)

The Exchange pays an Investment Bank's interest, one share for each Colony or station with a working
one, as a Region with a Bank counts; captured, it pays its captor as a captured Bank does. Spec
section 3.

**The picture.** [`ticket-491/exchange-hover.png`](ticket-491/exchange-hover.png): Tiangong's build
list, the Exchange's hover "+1 Ducat over a Trade Post, and a Bank's interest". It wraps to seven
lines, as the old wording did.

**The sweep** ([`after-491.txt`](sweeps/after-491.txt)): 3 / 25 / 26 / 4 and 21 collapses, from
3 / 26 / 26 / 4 and 20, within noise. The Prospectors' median Fund at the end rises in every
seating, by 21 to 308 Ducats.
