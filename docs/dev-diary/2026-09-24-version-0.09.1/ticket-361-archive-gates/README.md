# Ticket #361: the Archive is nearly unbuildable

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/361#issuecomment-5842786169)
is the authority, and served as the spec.

## Measured first: which gate binds

A throwaway diagnostic (not committed) counted, over eighty games, the first turn the Archivist seat
met each of the order's gates:

| gate | before | after |
|---|---|---|
| The Upload researched | 36 of 80, turn 28 | 40, turn 28 |
| an Archivist Colony on another Body | **19**, turn 22 | **24**, turn 22 |
| every gate and the Materials | **6**, turn 26 | **12**, turn 26 |
| Archive ordered / standing / complete | 6 / 6 / 6 | **12 / 12 / 8** (complete at a median turn **33**) |
| most Colonists ever at an eligible Colony | 4 | 12 |

## What was built

- **Rule, at the designer's word: The Upload gates the Archivists' WIN, not the Archive's order**,
  as before #199, whose premise had reversed (The Upload at turn 28, the Colony at 22).
- **The computer's Archivists ferry** while their uploads are short of the bar: a loaded Colony Ship
  does not land in Antarctica or at a station over Earth, and crosses to the Archive's Body (or the
  best Body off Earth while there is none); Colony Ships, crossings and Habitats at the Archive's
  Colony take the homeless lift. **The spending lift pauses while a site is ready and the Archive is
  not yet ordered** -- the first build spent every Material on Colony Ships and the Archive was
  ordered in 0 games of 80 -- but the route does not.
- Two tests updated to the new rule and one added (`the_archivists_ferry_their_people_off_earth`,
  witnessed red on the old computer). Clippy gate clean; 482 + 8 + 6 pass.

## What it did not do: the twelve uploads

In the games where the Archive completed, uploads came to 4 almost every time. The Archive completes
at a median **turn 33 of 36**, and the computer uploads everyone living there the moment it does, so
three turns is all the ferry has. **That is a clock, and it is the designer's**: carried on as its
own ticket.

## The sweep

[`sweep-before.txt`](sweep-before.txt) is ticket #358's after; [`sweep-after.txt`](sweep-after.txt);
[`sweep-order-gate-only.txt`](sweep-order-gate-only.txt) is the rule change without the ferry.

| | before | order gate only | after |
|---|---|---|---|
| wins C / P / Ar / Ar | 3 / 36 / 3 / 3 | 3 / 37 / 3 / 2 | 6 / 28 / 3 / 2 |
| collapses | 35 | 35 | **41** |

The order-gate change alone leaves the board as it was; **the ferry moves it**. No single mechanism
was found: the Archivists spend differently and every seating's game diverges from there. Six
collapses in eighty is about 1.4 standard deviations of the spread. Reported as a figure.
