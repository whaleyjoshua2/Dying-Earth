# 2026-10-03: balance suggestions after 0.09.5, one agent per Faction

The designer, after version 0.09.5 merged: *"fanout 4 opus agents (one for each faction) to make
suggestions about how to better ballance the game"*, then *"record the suggestions in the dev diary
and double check they used 0.09.5"*.

Four agents each argued for one Faction. Each read the glossary, the specs, the data and earlier
diaries, and could test its best idea with at most two sweeps in its own worktree. **Nothing here is
decided, built or ticketed.**

- [01-custodians.md](01-custodians.md)
- [02-prospectors.md](02-prospectors.md)
- [03-arkwrights.md](03-arkwrights.md)
- [04-archivists.md](04-archivists.md)

Each experiment's full output is in [`sweeps/`](sweeps/). Within a Faction's file, a suggestion is
named by its Faction and number (C1, P2, Ar1, Ac3, and so on), so a pick can name it.

**The baseline** is the 0.09.5 closing sweep,
[`final-0.09.5.txt`](../2026-09-29-version-0.09.5/sweeps/final-0.09.5.txt): 20 seeds x four
seatings at the shipped climate cell. It reads Custodians 10, Prospectors 10, Arkwrights 1,
Archivists 17, with 42 collapses of 80.

## Checked: every run was 0.09.5

- **A wrong start, caught.** The agents' worktrees opened on 0.09.4: the local `main` had not been
  updated after the merge on GitHub. The Prospectors' agent noticed and reset to `9fa4eee`. The other
  three were told to check and reset before measuring.
- **The check afterwards.** All eight experiment outputs list **Commodity Finance and Orbital Data
  Centers**, two Techs that exist only from 0.09.5. A 0.09.4 build cannot print them.
- **The Custodians' data copies** were compared against 0.09.5's `assets/data` with line endings
  ignored. Each differs only by the one change it claims: the Archive bar 125 → 160 in two files, and
  Planetary Stewardship +1.0 → +3.0.
- **Two agents' probes** each reproduced the 0.09.5 baseline exactly before changing anything: the
  Archivists' and the Arkwrights'.

## Every measured run

| Run | Cust | Pros | Ark | Arch | Collapses |
|---|---|---|---|---|---|
| **0.09.5 baseline** | 10 | 10 | 1 | 17 | 42 |
| Ar1: computer builds one Factory per Shipyard | 13 | 11 | **9** | 15 | 32 |
| Ar1 + Ar2: and the computer counts Bodies | 21 | 5 | **17** | 17 | 19 |
| C1: Archive bar 125 → 160 | 12 | 12 | 1 | **11** | 43 |
| Ac2 trial: Archive bar 125 → 150 | 12 | 9 | 1 | 15 | 42 |
| Ac6 trial: Upload bar 12 → 16 | 10 | 10 | 1 | 17 | 42 |
| C3 trial: Planetary Stewardship +1.0 → +3.0 | 13 | 7 | 1 | 20 | 39 |
| P1: Prospectors' computer pace rescaled to 2500 | 9 | 11 | 1 | 20 | 39 |
| P2: Fund bar 2500 → 2000, pace to match | 7 | 14 | 1 | 20 | 38 |

**Noise:** the Prospectors' P1 run moved the Archivists by 3 with nothing of theirs changed. Read a
move of about 3 wins as noise. **No two suggestions were measured together**, except Ar1 + Ar2.

## What the four found

**1. The Arkwrights' 1 win is mostly the computer's play, not their rules (Ar1).**
- Their computer seat buys Factory Modules for its one station over Earth again and again, and never
  launches a Colony Ship: none in any game by turn 18.
- The Factory has no cap in `ai.rs`. Capping it at one per Shipyard took them from 1 win to 9.
- It is a computer-seat change only, no rule moves, and it gave the most even column measured:
  13 / 11 / 9 / 15.
- **Cost:** collapses fell from 42 to 32.

**2. The Archivists lead because their win finishes itself whenever the climate holds.**
- They won 17 of the 38 games that did not collapse.
- Their research half is nearly free: the Archive's 125 fills by about turn 21.
- Raising the bar to 160 brought them from 17 to 11 (C1). Raising it to 150 brought them to 15,
  inside the noise (Ac2).
- Raising the Upload bar did nothing (Ac6).
- The Archivists' own agent proposes 200 and making rivals research other Factions' gates last (Ac1);
  neither is measured.

**3. Three Factions' computer pace tables were left behind when their rule moved.**
- The Prospectors' Fund pace still aims at 1000 where the bar is 2500 (P1).
- The Archivists' pace aims at 80 where the bar is 125 (Ac2).
- The Custodians' pace is never met in practice (C, "Leave alone").
- These are the same oversight each time: a figure moved and the computer's schedule did not. On its
  own, fixing the Prospectors' pace was within the noise.

**4. Where a Faction sits matters as much as its rules.**
- Seat 0 always starts in East Asia, and from there the Archivists win 10 of 20 (C6).
- The Prospectors never fill their Fund from seat 2 (P2), and no bar change fixed that.
- Measuring other starts (C6) would show how much of the 17 is the seat.

**5. The Prospectors lose at their bar, not on the way to it.**
- They lead on score in 29 games but fill the Fund in only 4.
- A 2000 bar (P2) added 4 wins, taken from the Custodians, not the Archivists.
- They make about three times anyone's Materials, but only Ducats fill the Fund, and no computer seat
  ever sells (P3).

## Where they pull against each other

- **Collapses.**
  - The Arkwrights' fixes cut them sharply: 42 → 32 with Ar1, → 19 with Ar2.
  - Stewardship +3.0 cut them to 39.
  - The designer treats collapses as good news, so each of these has a price.
- **Who pays.**
  - Ar1 + Ar2 costs the Prospectors (10 → 5).
  - P2 costs the Custodians (10 → 7).
  - Stewardship +3.0 alone feeds the Archivists (17 → 20).
  - C1 is the only measured change that took wins from the leader, and only from the leader.
- **The Archivists' bar.** Three figures are on the table: 150 (measured, inside the noise), 160
  (measured, 17 → 11) and 200 (Archivists, extrapolated).

## The pairing three agents point toward

Not measured together:
- **Ar1 (one Factory per Shipyard)**, which lifts the bottom without touching a rule;
- **C1 (the Archive at 160)**, which brings down the top.

Each moved its target by 6 to 8 wins with the other Factions inside the noise. The pace tables (P1
and the Archivists' pace) are clean-up any of these would carry. A combined sweep of Ar1 and C1 is
the obvious next measurement.

## Status

**2026-10-03:** filed for the designer to read. Nothing is on a map.
