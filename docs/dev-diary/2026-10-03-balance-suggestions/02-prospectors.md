# The Prospectors' view: balance suggestions after 0.09.5

Line numbers (l.N) are lines of the 0.09.5 closing sweep,
[`final-0.09.5.txt`](../2026-09-29-version-0.09.5/sweeps/final-0.09.5.txt).

## Where the Prospectors stand

**Measured** (`final-0.09.5.txt`, per-Faction totals):

- Wins 10 of 80 (l.284), but only **4** are outright (l.40, l.110). The Fund reached its bar in
  3 + 1 + 0 + 0 = **4 of 80** games (l.48, 118, 188, 258). The other 6 wins are on score at the last
  turn.
- Their score is the best of any Faction: median 0.38, placed 1st in 29 games (l.325). They often
  lead when the game times out or collapses, but rarely finish their own condition.
- Their Victory gate comes last: 70 of 80 games, median turn 25 (l.289).
- Their Fund depends heavily on seat. Median at the end:
  - 1851 as seat 1 (l.35);
  - 1340 as seat 0 (l.105);
  - only **718 and 890 as seat 2** (l.175, l.245). From seat 2 they never reach the bar.
- They hold the least land: 4202 Region-turns, against the Custodians' 15314. 23% of those turns
  are at Unrest 4 or more, the highest share. As seat 0 they lost their start state in all 20 games:
  3 to a throw-off, 17 to a taking (l.330).
- They produce the most Materials: 1492 a game as seat 1, against about 450 for the others (l.23).
  No seat sold anything on the Trading window: `sold [0, 0, 0, 0]` in every seating (l.52, l.262).

**Rules:**

- The Fund bar is 2500 Ducats (`factions.toml:103`). Only Ducats are banked, at up to 80% of
  income (`factions.toml:346`).
- The score is the smaller of the two Victory parts' fractions (`CONTEXT.md:792`). A game lasts 36
  turns (`victory.toml`).
- **A stale figure:** the computer seats' Fund pace is still
  `first = [[9, 133], [18, 333], [27, 600], [34, 1000]]` (`ai.toml:368`). It was sized for the old
  1000 bar and never rescaled when the bar moved to 2000 and then 2500. Two things read it:
  - the "behind on Victory" multiplier (`ai.rs:470`);
  - the Strip Permit trigger (`ai.rs:1068-1072`).

  How much to bank each turn (`ai.rs:3263-3288`) reads only the true bar and the pace's turns 9 and
  34, so it is not affected.

## Experiments (two runs, the shipped cell, not committed)

| Run | Cust | Pros | Ark | Arch | Collapses |
|---|---|---|---|---|---|
| Baseline 0.09.5 | 10 | 10 | 1 | 17 | 42 |
| A: pace rescaled to 2500 (`[[9,333],[18,833],[27,1500],[34,2500]]`), computer seats only | 9 | 11 | 1 | 20 | 39 |
| B: bar 2000 and pace rescaled to 2000 | 7 | 14 | 1 | 20 | 38 |

Outputs: [`sweeps/prospectors-pace-2500.txt`](sweeps/prospectors-pace-2500.txt) (A),
[`sweeps/prospectors-bar-2000.txt`](sweeps/prospectors-bar-2000.txt) (B).

- **Run A** did not meet my expectation. I hoped for 3 to 5 more Prospector wins.
  - They won 5 outright instead of 4.
  - The Fund medians barely moved: 1903 / 1291 / 718 / 842.
  - The Archivists gained 3 though nothing of theirs changed. Read that as the sweep's noise
    floor: a shift of about 3 wins is not a signal.
- **Run B** met it.
  - The Prospectors' median score rose from 0.38 to 0.46, and 1st places from 29 to 38.
  - As seat 1 they won 6 instead of 4, five of them outright (l.40). The Fund reached its bar in
    6 of 80 games.
  - **From seat 2 nothing changed:** the Fund ended at 718 and 842, with no wins.
  - The gain came out of the Custodians (−3), not the Archivists.

## Suggestions, best first

**1. Rescale the computer seats' Fund pace to the real bar** (`ai.toml:368`, as in Run A). *Small.*

- **Why:** the computer thinks it is on schedule when it has 40% of what it needs. That is a
  defect whatever the balance.
- **Effect:** roughly neutral (Run A: +1).
- **Risk:** slightly more Strip Permits.
- **Computer seats:** this change is to them alone. From now on, rescale the pace whenever the bar
  moves.

**2. Lower the Fund bar from 2500 to 2000** (`factions.toml:103`, with the pace scaled to match).
*Small.*

- **Why:** four outright wins in 80 is too few for a Faction that leads on score in 29. The race is
  lost at the bar, not on the way to it.
- **Effect:** about +4 Prospectors (Run B).
- **Risk:** it takes wins from the Custodians, who are not the problem. In 0.08.4 a 2000 bar gave
  30 of 80. That board was different, but watch for the old runaway.
- **Recommendation:** 2000, or 2250 to keep some of the "deliberate stretch".
- **Computer seats:** the pace must move with the bar.

**3. Let extraction fill the Fund.** *Medium (computer seats) or large (a rule change).*

- **Why:** the Faction built around extraction makes about 3x everyone's Materials, yet only
  Ducats count toward its win, and no computer seat ever sells a Material.
- **Two ways to do it:**
  - (a) Teach the computer seats to sell surplus Materials for Ducats. The selling price comes from
    `sell_divisor = 2` (`factions.toml:480`). Computer seats only, in `ai.rs` near line 3278.
  - (b) A rule change: the Extraction Charter lets the Fund bank Materials at 2 for 1 Ducat. The
    computer seats need the same change in the same place.
- **Effect:** not measured. The Fund grows in every seat, which could close the seat-2 gap that
  suggestion 2 leaves.
- **Risk:** in seat 1, where they are already strong, it could snowball.

**4. Strip Permit lasting Unrest from +3 to +2** (`nation_states.toml:73` Strip Permit table; the
card text in `factions.toml:88` changes with it). *Small.*

- **Why:** the Prospectors carry the most Unrest of any Faction and lose their start state in every
  game as seat 0, mostly to takings. Land is what pays their Banks.
- **Effect:** not measured; perhaps +1 or +2, mostly from the weak seats.
- **Risk:** it makes the polluting move cheaper. The +0.2 lasting emissions stays, so collapses
  should not fall.
- **Computer seats:** the figure is read from the data; nothing changes.

**5. Investment Bank interest from 1% to 1.5%** (`facilities.toml:282`). *Small.*

- **Why:** interest is paid per Bank, one per Region. The Prospectors average under two Regions
  (4202 Region-turns over 80 games), so the compounding their card promises barely starts.
- **Risk:** a seat that does hold land compounds faster. Pair it with 2 or 3, not with both.
- **Computer seats:** their appetite for Banks already scales with the Fund (`ai.toml:278`).

**6. Cut the computer Prospectors' carbon-credit buying** (`buy_credits` from 4 to 2, `ai.toml:42`).
*Small, computer seats only.*

- **Why:** they bought 1443 and 1067 ppm in two seatings (l.43, l.253). Those Ducats leave the Fund
  route and land in the Custodians' Stockpile.
- **Risk:** more Blame makes Regions harder to win over. Measure it before adopting.

**Not recommended now: a cheaper road to the Charter.** The gate comes last (turn 25), but the Fund
rarely reaches the bar before then anyway. The gate is not what holds them back.

## The four-way view

The Prospectors are not the problem Faction: they are joint second and the score leader. What needs
fixing is the Arkwrights at 1 win and the Archivists' 17 to 20 lead.

- Suggestion 2 helps the Prospectors at the Custodians' expense.
- If the designer wants the Prospectors' extra wins to come from the Archivists, suggestion 3 is the
  better candidate. It adds a source of income rather than lowering a bar, and it matters most in
  the seats where the Prospectors finish far behind.
