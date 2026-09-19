# Ticket #237: the Exodus Call

The Arkwrights' own order: on a Region they control, for the price of a Leapfrog, **two turns of a
doubled muster** — sixteen Pioneers a turn where they otherwise recruit eight. **Once per Region,
ever**, the shape the Strip Permit has had since version 0.05.

## The measurement changed what the order does

The ticket was charted worrying that a Call would empty a small Region. That worry was built on
population figures I had lifted from a 2026-09-11 playtest, which predate version 0.07.3's
rescaling — **they were wrong by a factor of twenty**, and the ticket said so before anything was
built.

The real measurement was worse, not better. Home-Region population over a game, median of 20 seeds:

| | Custodians | Prospectors | **Arkwrights** | Archivists |
|---|---|---|---|---|
| start | 121 | 290 | **20** | 10 |
| end | 88 | 197 | **1** | 1 |

**Coach Class already empties their Region**, because it charges them twice the population a head.
They run out of people — on the Faction whose whole Victory Condition is moving people.

So an order that doubled only the *count* would have deepened the very thing capping them at 3
wins of 80. The designer took the other reading: **a Call suspends the double charge while it
runs**. Sixteen Pioneers at the ordinary price in population, not their double.

## What else was settled

- **Double their eight, not the base four** — sixteen a turn.
- **The turn it is ordered and the turn after.**
- **Once per Region, ever**, Arkwrights alone, priced as a Leapfrog.
- **Unrest is left alone for now.** A doubled muster still takes half a point off, which reads
  strangely at four times the drain; the designer's note is that it *"should eventually scale by
  pioneer"*, which is a ticket rather than a silent oddity.

## The thing this ticket could not solve

**The Arkwrights cannot afford it.** Their starting Region pays **0.8 Ducats a turn**:

| Region | starting seat | Ducats/turn |
|---|---|---|
| The European Union | Prospectors | 12.0 |
| China | Custodians | 10.2 |
| **Saudi Arabia** | **Arkwrights** | **0.8** |
| Australia | Archivists | 0.8 |

A 50-Ducat order is sixty turns of that Region's entire income, and in a full headless game the
Call fired **zero times** even after the AI's gating was stripped back to almost nothing. For
contrast the Custodians ran **703 Leapfrogs** in a 20-seed batch at the same price.

The designer kept the price: *"keep it duckets gdp can always be increased"*. So it ships at 50,
and the closing sweep should be expected to show it firing rarely until a Region's GDP moves.

## Not photographed, and why

The button is guarded by Faction **and** control, and no combination of `shot:` aids got an
Arkwright seat and a Region they hold into one capture — `player:`, `start:` and `select:` did not
compose the way the flag names suggest, and two of them fail **silently** on a name that does not
match. The rule is covered instead by two tests that pin the exact figures, including that sixteen
Pioneers cost strictly less under a Call than Coach Class charges.

Red witnessed: the muster multiplier was set to 1 and the test that guards the doubling watched to
fail. `309 passed; 0 failed`, `6 passed; 0 failed`, clippy clean with the denial.
