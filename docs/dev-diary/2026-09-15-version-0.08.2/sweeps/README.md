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

## What this sweep does NOT yet report

`docs/spec/version-0.08.2.md` section 12 names six figures this version's rules depend on, and the
sweep reports none of them yet:

- each seat's **Blame share** at the end and over the game;
- the **distribution of Relations scores and floors** — a version where every pair is floored at Wary
  is the failure mode the scar ticket named;
- **takes per batch**, before and after, since the challenge-margin term is the one change that can
  move the win table directly;
- **Accords struck, by term and by seat** — a system the computer seats never use is invisible;
- **buy and sell volume per seat**, and Ducat spending, since every price rose;
- **the turn the Tech Tree completes**, since two seats at +10% Research reach it sooner.

Adding them is part of the build ticket and has not been done.
