# Twice the Influence off Earth: the build, the measurement and the pictures

Ticket [#336](https://github.com/whaleyjoshua2/Dying-Earth/issues/336) on version 0.09.0. The
designer's line: *"double influence needed to take a space station or colony"*, taken literally and
with a floor under it. A place off Earth costs **40 + 20 a Colonist**, whether it is a ground Colony
or a Space Station; the **challenge margin stays 20**, shared with Earth, so a settled place whose
holder stands high costs exactly what it did and what got dearer is a place worth taking on its
threshold. `sweeps/` holds the closing eighty-game sweep.

| place | before | now |
|---|---|---|
| ground Colony, N Colonists | 10N | **40 + 20N** |
| Space Station, N Colonists | 20 + 10N | **40 + 20N** |

| picture | what it shows |
|---|---|
| [`threshold-colony-card-moon.png`](threshold-colony-card-moon.png) | `shot:… barracks:1 hab:ground`. A ground Colony of four Colonists on the Moon: *Standings: nobody has any yet · **Threshold 120***, and under it the breakdown in its new shape, *Threshold 120 - 40 for the Colony + 80 for 4 Colonists*. Under the old figures the same Colony was worth 40. |
| [`threshold-colony-hover-moon.png`](threshold-colony-hover-moon.png) | The same card with `tip:What a Standing must reach`: the Threshold label's hover, *What a Standing must reach to take this place: **40 for the Colony itself, plus 20 a Colonist living here.*** It named one figure for a ground Colony and two for a station; now it names the base the place actually used. |
| [`threshold-station-card-earth.png`](threshold-station-card-earth.png) | `shot:… hab:1 panel:0`. The ISS over Earth, two Colonists aboard: ***Threshold 80***, *Threshold 80 - 40 for the station + 40 for 2 Colonists*. The station base replaces the Colony base rather than stacking on it, so this is the same 80 a ground Colony of two costs. It was 40. |
| [`threshold-station-hover-earth.png`](threshold-station-hover-earth.png) | The station card with the same hover: *40 for the **station** itself, plus 20 a Colonist living here.* |

## What was built

- **Three figures in `assets/data/influence.toml`**: `colony_threshold_per_colonist` 10 to 20,
  `station_threshold_base` 20 to 40, and a new `colony_threshold_base` of 40 under every place off
  Earth. `challenge_margin` is untouched at 20. Both bases stay separate figures so they can
  diverge again without a rule change.
- **`engine/src/state.rs` `threshold_with`**: a place off Earth is `base + per_colonist x
  colonists`, the base being the station's in orbit and the Colony's on the ground -- one replaces
  the other. `engine/src/data.rs` loads `colony_threshold_base` and the load check refuses a
  non-positive one, since at nought an empty Colony is free to take again.
- **The computer seats** (`engine/src/ai.rs`) weigh a rival's Colony by **the price they would
  actually pay** (`influence_needed_for`, cheapest first) where the rule from ticket #50 ranked by
  fewest Colonists and never read the threshold at all. The weight is `colony_price_pivot / price`,
  capped at the old rule's own 3.0, with the pivot in `ai.toml` at 80 -- the price of a starting
  two-Colonist place -- so a Colony's worth against a Region's is where it was. Regions are weighed
  exactly as before.
- **The counter is counted, not scraped.** Places taken by Influence were counted by string-matching
  log lines ending `(Influence).`, one figure for Regions and Colonies together. Three counters now
  sit on `WarCounters` and are incremented in `transfer_control` at the transfer itself, by the kind
  of place. The sweep keeps its total line and prints the split under it.
- **The surface** (`src/ui.rs`): the breakdown line names the base it used (*40 for the Colony* /
  *40 for the station*), and the Threshold label's hover names one base and the per-Colonist figure
  instead of the old "10 a Colonist living here, and 20 for the station itself".
- **`CONTEXT.md`**: **Threshold**, **Influence** and **Space Station** carry the new figures and say
  what they were.

## The closing sweep, against ticket #335's

Twenty seeds a seating, all four seatings, `--balance --seatings --steps=300`.

| | #335 final | **#336 final** |
|---|---|---|
| wins, seating 1 / 2 / 3 / 4 | [0,2,0,0] / [9,0,0,0] / [3,3,4,0] / [0,3,12,1] | [0,2,0,0] / [9,0,0,0] / [0,3,7,0] / [1,1,14,0] |
| by Faction over 80 games | Cus 6, Pro 27, Ark 4, Arc 0 | Cus 4, Pro 32, Ark 0, Arc 1 |
| collapses | 43 of 80 | 43 of 80 |
| places taken by Influence | 300 / 408 / 322 / 298 (1,328) | 300 / 407 / 323 / 301 (1,331) |
| of those, Regions | not split | 300 / 403 / 316 / 286 (1,305) |
| of those, ground Colonies | not split | 0 / 4 / 4 / 13 (21) |
| of those, stations | not split | 0 / 0 / 3 / 2 (5) |

The doubling barely moves the aggregate because **a take off Earth is two per cent of all takes**:
26 of 1,331 over eighty games, which the old single counter could not have said. The win column is
unchanged in the first two seatings and moves within a seat or two in the other two; the collapse
count is identical at 43 of 80.

## Looked at

The four pictures above, opened and read before this was written. The twenty-eight other view
captures the four runs wrote were deleted.
