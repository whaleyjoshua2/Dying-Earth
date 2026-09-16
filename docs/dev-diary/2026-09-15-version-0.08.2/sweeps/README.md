# The first 0.08.2 sweep

Twenty seeds by four seatings at the shipped climate cell (`--steps=300 --balance`), against the
0.08.1 baseline. Raw output in `first-0.08.2.txt`.

| Faction | 0.08.1 | 0.08.2 | |
|---|---|---|---|
| Custodians | 42 | **37** | −5 |
| Prospectors | 11 | **10** | −1 |
| Arkwrights | 7 | **3** | −4 |
| Archivists | 6 | **7** | +1 |
| **Collapses** | **14 of 80** | **23 of 80** | **+9** |

## The collapse rate rose, and that is the point

**The world collapses in 23 of 80 games where it collapsed in 14.** That is the largest movement in
the batch.

It was first written up here as a problem wanting attention. **The designer's reading, and the one
that stands: a rise in collapses is a POSITIVE.** This is a game about colonizing the solar system
*before ecological collapse overtakes Earth*, so a world that collapses more often is one where the
climate genuinely bites — where the Collapse Line is a deadline being raced rather than a decoration.
By that reading the 0.08.1 figure of 14 in 80 was on the low side, not the right side.

The likeliest cause is the version's one substantive balance change: **every Trading window price rose
by one**, which cuts what a Ducat buys on a currency that also pays for Relief, Resettle and
Leapfrogs. Fewer Scrubbers and Leapfrogs means a smaller Natural Sink. That is recorded as the
mechanism, not as a fault to be corrected.

## The win table, which is the figure that does want watching

| Faction | 0.08.1 | 0.08.2 | |
|---|---|---|---|
| Custodians | 42 | **37** | −5 |
| Prospectors | 11 | **10** | −1 |
| Arkwrights | 7 | **3** | −4 |
| Archivists | 6 | **7** | +1 |

The spread is no better than it was: the Custodians still take nearly half the table, and **the
Arkwrights have halved, from 7 to 3**, which leaves the thinnest Faction thinner. Nothing in this
version set out to move either figure, and the Arkwrights' fall is the one worth a look — they were
already the seat with one signature rule, no faction-only order and no Unique Module.

## The second sweep, with the six figures the spec asked for

`second-0.08.2.txt`. The sweep now reports all six, and two of them earned their place immediately.

**The Accords were never struck at all.** The first run of these figures showed **zero standing at the
end of twenty games** — the computer seats had no way to propose one, so the whole subsystem was
invisible in a game they play among themselves. That is the risk the Accords ticket recorded as its
own largest. Giving the seats an appetite (`ai.toml: accord`, and a push that offers what a seat
would itself accept) fixed it: **157 Accords stand at the end across the 80 games**, all of them
non-aggression, which is the only term that means anything on today's board.

**Nearly half of all ordered pairs end the game carrying a scar floor** — 42% to 47% by seating, and
between 84 and 103 of 240 pairs sit at Cold or worse. That is close to the failure mode the scar
ticket named for itself: *"by turn 30 every pair is floored at the worst and the score stops carrying
information."* It is not there — the median pair ends at −1 to −4, so the scale is still saying
something — but it is nearer than the charting expected, and the step is one number in
`factions.toml` if it wants softening.

The other four, for the record:

| figure | reading |
|---|---|
| Blame share at the end | The dirtiest seat sits at **0.64–0.65** when it is the Prospectors, 0.33–0.39 otherwise. The rule fires hard and constantly. |
| Places taken by Influence | 269–372 a batch, which is the baseline the challenge-margin term should be watched against. |
| Trading window units | bought `[80, 7100, 0, 70]`, sold `[0, 0, 0, 0]` — one seat is the market, and nobody sells. |
| The whole Tech Tree | completes in 19–20 of 20 seeds, median turn **31–36**, where 0.08.1 had it done by the mid-game. |

## What this sweep does NOT yet report

All six are reported now. What remains unmeasured is narrower:

- **Ducat spending per seat**, which would say directly whether the price rise is what moved the
  collapse rate.
- **Accords by TERM struck over a game** rather than standing at its end, so a term struck and broken
  is not invisible.
- The figures are reported per seating; nothing yet aggregates them across the four.

