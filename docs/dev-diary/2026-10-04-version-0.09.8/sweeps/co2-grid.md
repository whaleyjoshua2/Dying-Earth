# The CO2 sweep of version 0.09.8 (ticket #483)

Fifteen cells and one alternative, each 80 games (twenty seeds by four seatings) on the
eighteen-Region board, at commit `808464a` with only the two climate figures varied. Wins are
Custodians / Prospectors / Arkwrights / Archivists. The designer's ideal is about 15 each and 20
collapses.

**The Sink** is `natural_sink`, the ppm the world takes back each turn. **The step** is `ppm_step`,
the ppm it takes to add half a degree: higher is more forgiving.

## What the Regions emit at the start

The industry line (baseline x Industry Level) and the per-head line, summed over the Regions, with
no building counted:

| Version | Regions | Industry | People | Sum |
|---|---|---|---|---|
| 0.09.5 | 14 | 10.30 | 8.03 | 18.33 |
| 0.09.7 | 16 | 11.60 | 7.90 | 19.50 |
| 0.09.8 | 18 | 13.20 | 8.05 | 21.25 |

The world's people are 7,860 in all three. The 2.9 the industry line gained is exactly the four
added Regions' own (the United Kingdom 0.9, Kazakhstan 0.4, Turkey 0.8, South Africa 0.8): no
parent's was reduced.

## The grid

| Sink | Step | Wins | Collapses |
|---|---|---|---|
| 6 | 300 (the cell before this ticket) | 4 / 20 / 11 / 6 | 39 |
| 6 | 330 | 8 / 23 / 9 / 8 | 32 |
| 6 | 360 | 7 / 36 / 18 / 7 | 11 |
| 7 | 300 | 6 / 20 / 13 / 6 | 34 |
| 7 | 315 | 6 / 24 / 12 / 5 | 31 |
| 7 | 330 | 6 / 25 / 15 / 7 | 27 |
| 7 | 345 | 8 / 31 / 19 / 6 | 16 |
| 7 | 360 | 7 / 36 / 19 / 8 | 9 |
| 8 | 300 | 8 / 19 / 10 / 5 | 37 |
| 8 | 315 | 5 / 24 / 13 / 8 | 28 |
| **8** | **330 (chosen)** | **9 / 27 / 18 / 7** | **17** |
| 8 | 345 | 9 / 36 / 19 / 6 | 10 |
| 8 | 360 | 7 / 32 / 17 / 9 | 11 |
| 9 | 330 | 9 / 34 / 18 / 4 | 14 |

## The alternative: square the Regions

Every Region's baseline emissions scaled by 10.3 / 13.2, so the industry line is 0.09.5's again,
with the climate left at Sink 6 and step 300: **10 / 20 / 11 / 7 and 30 collapses**. It undoes the
new countries and no more.

## What the grid shows

- **The dials set the collapses, not the win column.** As collapses fall the games saved go mostly
  to the Prospectors: 20 wins at 39 collapses, 27 at 17, 36 at 11.
- **The chosen cell's collapses are nearly all one seating's.** By seating, with the Faction in seat
  0 starting in China: Custodians 1 of 20, Prospectors 14, Arkwrights 2, Archivists 0.
- The step does more than the Sink across this range: Sink 8 at step 300 collapses 37.
