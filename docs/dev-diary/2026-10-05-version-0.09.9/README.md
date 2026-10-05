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

**The sweep's command:** `cargo run --release -p dying-earth-engine --example sweep -- 20 --seatings
--balance`.
