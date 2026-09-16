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

## The finding that wants the designer's eye

**The world collapses in 23 of 80 games where it collapsed in 14.** That is the largest movement in
the batch and it is not what any of the eleven decisions set out to do. The win table moved much less
than the collapse rate did.

The likeliest cause is the one substantive balance change in the version, and it was flagged as such
when it was decided: **every Trading window price rose by one** — Materials +50%, Fuel +33%, Energy
+100% — which is a broad cut to what a Ducat buys, on a currency that also pays for Relief, Resettle,
Leapfrogs and Influence. A Custodian with less buying power raises fewer Scrubbers and buys fewer
Leapfrogs, and the Natural Sink is what stands between the board and the Collapse Line.

The Arkwrights halving (7 to 3) is the other movement worth a look; they were already the thinnest
seat.

**Nothing here has been acted on.** The rule was shipped knowingly and the sweep is doing its job by
saying what it cost. Whether to soften the price rise, to compensate elsewhere, or to accept a
hotter world is a design decision and belongs to the designer.

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
