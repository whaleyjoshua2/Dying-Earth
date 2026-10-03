# The Custodians' view: balance suggestions after 0.09.5

Line numbers are lines of the 0.09.5 closing sweep,
[`final-0.09.5.txt`](../2026-09-29-version-0.09.5/sweeps/final-0.09.5.txt).

## Where the Custodians stand

**Rules.** The Custodians win on two things together:

- **The climate:** a run of three Climate phases in a row with the counted Emissions under the
  Natural Sink plus the Scrubbers (`victory.toml` `stabilization_turns = 3`; `state.rs:615-623`).
  Event cards and the permafrost do not count.
- **People off Earth:** 12 Colonists living off Earth (`factions.toml:50-51`).

Planetary Stewardship opens the Condition and adds a flat +1.0 ppm to the Sink (`techs.toml:265-266`,
`climate.rs:339-341`). That is a third of one Scrubber, which adds 3.0 (`facilities.toml:110`). A game
opens about 31 ppm over the Sink (`spec/version-0.09.4.md` §3).

**Measured** (`final-0.09.5.txt`):

- **They are at par.** 10 wins of 80; the 38 games that do not collapse share out to about 9.5 a
  Faction (lines 283-287). 6 of the 10 are outright (the "met outright" lines); 4 are on score at turn
  36.
- **They are the strongest power on Earth, and the win column barely shows it.**
  - They hold Regions for 15,314 Region-turns, against 4,202, 4,136 and 2,952 (lines 329-332).
  - They lead the Tech for 1,348 Tech-turns, against 370, 145 and 173 (line 333).
  - Yet their median score is 0.19, and they place 3rd in 41 games (line 324).
- **Their win waits on the world.** The world gets under the Sink at least once in 16 of 80 games,
  and late: median first turn 31 (line 334). Planetary Stewardship stands at a median turn 24
  (line 316).
- **The Archivists finish first.** In the seating with the Archivists in seat 0, the world reaches
  the Sink in 6 of 20 games. The Custodians win 2 of them and the Archivists 10 (lines 213-216, 278).

So the Custodians' problem is not strength. Their win comes late and depends on the climate, and the
Archivists' comes earlier.

## Experiments (two runs, the shipped cell, at 9fa4eee with an edited copy of `assets/data`)

| | Cust | Pros | Ark | Arch | collapses | outright C / P / Ar |
|---|---|---|---|---|---|---|
| Baseline 0.09.5 | 10 | 10 | 1 | 17 | 42 | 6 / 4 / 13 |
| A: Archive 125 → 160 | **12** | **12** | 1 | **11** | 43 | 10 / 3 / 10 |
| B: Stewardship +1.0 → +3.0 | **13** | **7** | 1 | **20** | 39 | 10 / 3 / 13 |

Outputs: [`sweeps/custodians-archive-160.txt`](sweeps/custodians-archive-160.txt) (A),
[`sweeps/custodians-stewardship-3.txt`](sweeps/custodians-stewardship-3.txt) (B).

- **A met my expectation.**
  - The Archivists fall 6; the Custodians and Prospectors gain 2 each.
  - In the seating with the Archivists in seat 0, the Custodians rise from 2 wins to 7.
  - Custodian outright wins rise from 6 to 10: the extra time became real climate wins.
  - Collapses barely move.
- **B did not.** It feeds the Archivists (+3) and costs the Prospectors (−3). With fewer collapses,
  more games last long enough for the Archive to finish.

I did not re-run the plain baseline, so both rows rely on the sweep being deterministic.

## Suggestions, best first

### 1. Raise the Archivists' Archive bar from 125 to 160 Research (measured, A)

- **What:** `modules.toml:144` `research = 160` and `factions.toml:217` `bar = 160`, together; a test
  checks that they match. Change the "125 Research" text at `factions.toml:215` too.
- **Why:** Research roughly doubled in 0.09.4 §15, and the Archivists went from 7 wins to 17 on it
  (`after-416.txt`, `final-0.09.4.txt`). The bar was set before that. 13 of their 17 wins are
  outright, and they finish before the world can reach the Sink.
- **Expected effect:** about 12 / 12 / 1 / 11, collapses 43 (measured). The win column flattens
  across three Factions.
- **Risk:** the Archive finishes later (median turn 23-25, from 21-23), so the Archivists may feel
  slow in a human game. A bar of 145 is a softer middle step.
- **Computer seats:** no change needed; they pay into the fund until it is full (the comment at
  `ai.toml:305-306`). The pace table `[pace.archivists]` already ends at 80 and was never moved to
  125; fixing it is optional cleanup.
- **Size:** small.

### 2. Give the Arkwrights a reachable second part (not measured)

- **What:** `factions.toml:171`, either `bodies = 3` to 2 or `colonists_each = 4` to 3.
- **Why:**
  - The Arkwrights won 1 of 80, and their score is nought in 58 (line 326).
  - Phobos is settled first in only 9 of 80 games, Deimos in 49 (line 335).
  - From the Custodians' seat, a fourth rival that can actually finish takes games from whoever
    leads, which today is the Archivists.
- **Expected effect:** the Arkwrights gain a few wins, mostly from the score wins at turn 36. The
  size is unknown and needs one sweep.
- **Risk:** the game the designer may want from them is the long voyage, and two Bodies could be
  just the Moon and Deimos.
- **Computer seats:** their colonizing already targets Bodies, so the figure alone should do. Check
  that the computer's pace figures do not still assume three Bodies.
- **Size:** small.

### 3. Planetary Stewardship from +1.0 to +2.0, only together with #1 (+3.0 measured alone, B)

- **What:** `techs.toml:266` `value = 2.0`.
- **Why:** the Custodians' own Victory Tech is worth a third of a Scrubber. The 130-Research line
  (`spec/version-0.09.5.md` §1, the dearest in the tree) deserves to be felt.
- **Measured alone at +3.0:** the Custodians gained 3, the Archivists gained 3, the Prospectors lost
  3 and collapses fell 3. Hence: only after #1, and at the smaller +2.0.
- **Risk:** fewer collapses, against the designer's taste, and on its own it feeds the Archivists.
- **Computer seats:** no change; every seat already researches it (77 of 80 games).
- **Size:** small.

### 4. Make the Research Directive a real choice for the Custodians (not measured)

- **What:** `factions.toml` `[research_directive] custodians_ppm_per_point` from 0.01 to 0.03, and
  teach `ai.rs` to divert Research when the Sink gap is small (say within 5 ppm).
- **Why:** the computer Custodians keep back only 5-7% of their Research (lines 24, 94, 164, 234), so
  the lever is effectively dead. It was cut from 0.05 when the Custodians won 37 of 80; at 10 of 80
  there is room for a middle figure.
- **Expected effect:** Custodians +1 to +3. The shared tree slows a little, which also slows the
  other gates.
- **Risk:** it does what a Scrubber does, by another route.
- **Computer seats:** yes: `ai.rs` must change, or the new figure does nothing in the sweep.
- **Size:** medium.

### 5. Trim the Custodians' Influence multiplier from 1.2 to 1.1, only if #1 and #3 lift them past about 13 (not measured)

- **What:** `factions.toml:31`.
- **Why:**
  - They hold more than three times as many Region-turns as any rival (lines 329-332).
  - They take the most Regions by Influence in every seating (line 54 and its counterparts).
  - That grip squeezes the Prospectors' and Arkwrights' Earth bases without winning the Custodians
    anything: a steamroller on Earth with no win to show for it.
- **Risk:** a Region's Scrubber cap follows its population (`state.rs:4353`), and a Custodian
  Scrubber stands only where they hold the Region. Fewer Regions could mean fewer Scrubbers and
  fewer climate wins.
- **Computer seats:** no change.
- **Size:** small.

### 6. Measure the East Asia start before tuning any Faction against it (a measurement only)

- **Why:** seat 0 always starts in East Asia. From there:
  - the Archivists win 10 of 20 (line 214);
  - the Custodians 1 (line 4);
  - the Prospectors 2 and the Arkwrights 0.

  Part of the Archivists' 17 belongs to that start, not to their rules.
- **What:** sweep with `--start=` set to other Regions.
- **Size:** small (one sweep).

### Leave alone

- **The three-phase run and the 0.3 cap on partial credit** (`victory.toml`). Partial credit already
  lets the Custodians rank at turn 36. A cap above a third would pay coming close as much as a real
  phase under the Sink.
- **The computer Custodians' pace** (`ai.toml` `[pace.custodians]`): within 4 ppm by turn 12, under
  the Sink by turn 18.
  - It is never met in practice; the world first gets under at a median turn 31.
  - So the catch-up push (`victory_gap_max = 3.0`) sits almost always at its ceiling
    (`ai.rs:461-468`).
  - That is harmless, but the pace tells the computer nothing.

## Recommendation

Take #1 first: one number in two files, measured to bring the Archivists from 17 to 11 and the
Custodians and Prospectors to 12 each, with collapses unmoved. Then sweep #2 for the Arkwrights. Hold
#3 and #5 until both are measured.
