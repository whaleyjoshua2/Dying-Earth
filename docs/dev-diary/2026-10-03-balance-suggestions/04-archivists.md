# The Archivists' view: balance suggestions after 0.09.5

Line numbers are lines of the 0.09.5 closing sweep,
[`final-0.09.5.txt`](../2026-09-29-version-0.09.5/sweeps/final-0.09.5.txt).

## Why the Archivists lead (17 of 80)

**Rules (code and data):**

- **Their Condition:** the Archive at a Colony off Earth, **125 Research** paid into its fund, and
  **12 Colonists Uploaded** (`factions.toml` Archivist card; `modules.toml [archive] research = 125`).
  Their gate is The Upload (`techs.toml`, `the_upload`).
- **They alone may divert 100% of their Research** (`factions.toml [research_directive]
  archivists_max = 100`). An Uploaded Colonist counts for good, and the fund is kept if the Archive
  is lost (CONTEXT.md, *Upload*, *The Archive*).
- **No computer seat acts against a rival's progress.** Ticket #50 removed the denial multiplier
  (`engine/src/ai.rs:78`).
- **At turn 36 the seats are ranked by score**, the lower of their two parts' fractions
  (`engine/src/victory.rs:61`, `end_phase`).
- **A computer Research Lead whose own list is used up picks the cheapest Tech left**
  (`ai.toml [tech_picks]`), and that includes rivals' gates.

**Measured behaviour (`final-0.09.5.txt`):**

- **Archivist wins follow the games that do not collapse.** By seating, Archivist wins were
  6 / 1 / 0 / 10 against collapses of 8 / 12 / 16 / 6 (lines 3, 73, 143, 213). They won **17 of
  the 38 games that did not collapse**.
- **Others pay for their gate.**
  - In every seating the Archivists kept back 100% of their Research (lines 24, 94, 164, 234), for
    about 30 turns a game.
  - They led 173 Techs over the batch, against the Custodians' 1,348 (line 333).
  - Even so, The Upload was completed in 71 of 80 games, median turn 23 (line 291).
- **The timeline, from a per-game trace.** I re-ran the 80 games with a throwaway probe. It
  reproduces the sweep's wins exactly and has been deleted.
  - The Archive is ordered around turn 17. Its fund reaches 125 at a median of about turn 21, and
    it completes then.
  - Uploads then arrive in fours (4, 8, 12).
  - **13** of their wins were the Condition met outright, between turns 24 and 35.
  - **4** were on score at turn 36, with scores of 0.67 to 0.83 against rivals at 0.42 or below.
- **Nothing contests them.** The Archivists opened no orbital Battles, and no seat targets the
  Archive (lines 337-341).

So the Archivists' Condition is the only one that finishes itself whenever the climate holds. Its
research half is nearly free: the fund fills by about turn 21 in most games. Their turn-36 score is
therefore just their Uploads, and Uploads come in big steps.

## Experiments (the shipped cell, against 10 / 10 / 1 / 17, 42 collapses)

| Change | Custodians | Prospectors | Arkwrights | Archivists | Collapses |
|---|---|---|---|---|---|
| Archive at **150** Research (both files) | 12 | 9 | 1 | **15** | 42 |
| Upload bar **16** | 10 | 10 | 1 | **17** | 42 |

Outputs: [`sweeps/archivists-archive-150.txt`](sweeps/archivists-archive-150.txt),
[`sweeps/archivists-upload-16.txt`](sweeps/archivists-upload-16.txt).

- **Archive at 150 fell short of my expectation** of about 12. The Archive's median completion moved
  only one turn, from 22 to 23 (line 242). A fall of 2 wins is within the noise.
- **Upload bar 16 left the win column unchanged.**
  - Outright Archivist wins fell from 13 to 9, but turn-36 score wins rose from 4 to 8.
  - Colonists pile up at the Archive and are uploaded in one go, so a higher bar costs a turn or two
    at most.
  - 12 of 16 still tops the turn-36 ranking.

**So the Upload bar is not a lever, and the research bar moves things only a little.**

## Suggestions, best first

**1. Rivals research other Factions' Victory gates last, and the Archivist computer seat stops
diverting once its fund is full.** *Computer seats only, small.*

- **What:** in `ai.toml [tech_picks.*]`, give each Faction `last = [the three rivals' gates]`. In
  `ai.rs` (about line 1927), set the Archivists' directive to 0 once the fund is full.
- **Why:**
  - The Archivists get their gate for free today.
  - The Lead's own gate is always on its shortlist, so they can still get The Upload by leading.
  - That makes real the "monument or the tree" choice the glossary describes, and it brings
    Provisional Findings and the +5 Influence for leading into their game.
- **Expected:** the Archivists fall to about 12. Not measured.
- **Risk:** every gate slips. Ticket #348 removed `last` because it blocked a rival's only road;
  here the target is the gate itself, and its owner can still lead it.
- **Rule figures:** none change.

**2. The Archive costs 200 Research.** *Data, small.*

- **What:** `modules.toml [archive] research` and the Archivists' `victory_first` bar on their card,
  together (the game refuses to load them out of step).
- **Why:** the 150 run moved the Archive's completion about one turn per 25 Research. At 200 the fund
  should fill around turn 26 or 27, near the collapse median of 28 to 32, so research becomes a real
  half of their score.
- **Expected:** from 17 to about 11. Extrapolated, not measured.
- **Computer seats:** `[pace.archivists] first` must change too. It still reads 80 by turn 32
  (`ai.toml:384-386`) and was never moved to 125, so the computer thinks it is ahead of schedule.
  Rescaling it alone may make them stronger, so measure both together.

**3. A running Archive emits.** *Rule plus data, small to medium.*

- **What:** for example 1 ppm a turn while the Archive is online, with the Blame laid on the
  Archivists.
- **Why:** they win exactly when the climate holds. This ties their engine to the climate clock the
  other three race against, and feeds the Custodians' contest.
- **Expected:** about −3 wins and more collapses (which the designer accepts). Not measured.
- **Computer seats:** no change.

**4. Counterplay: the fund is lost when the Archive's Colony changes hands.** *Rule, small.*

- **Why:** today a capture costs the Archivists only the Module.
- **Expected:** almost nothing among computer seats, which never target the Archive. It matters for
  a human rival.
- **Note:** a computer response would reopen ticket #50's denial question; that is the designer's
  call.

**5. Lift the Arkwrights' turn-36 score.** *Four-way, medium.*

- **Why:** the Arkwrights score nought in 58 of 80 and place 4th in 48 (line 326). A real rival in
  the turn-36 ranking takes wins straight from the Archivists.
- **For example:** their per-Body part could count a Body at half credit with 2 Colonists. This is
  for the Arkwrights' advocate to set.

**6. Do not raise the Upload bar.** Measured above: it leaves the win column unchanged.

## Notes

- **The Archivist computer seat keeps 100% diverted after its fund is full.** At 100 it loses
  Provisional Findings, its own signature rule, and every rival thinks a point worse of it all game,
  for no gain. Suggestion 1 fixes this.
- **My worktree:** it was moved to 0.09.5 (9fa4eee) before anything was measured. The data edits are
  reverted and the probe file deleted; nothing was committed.
