# Ticket #240: the Prospectors hoard Ducats

> Prospectors victory condition now require duckets rather than materials - maybe rename them and
> change their glyph

The Venture Capital Fund holds **Ducats**, filled by a share of **Ducat income**, and the bar is
**2000**. The name and the Faction's glyph are unchanged.

## The ticket's premise was false, and the measurement said so before a line was written

The ticket was titled *"a Victory Condition no computer seat has ever met"* and quoted a 0.08.2
sweep finding the Fund at 180 and 284 against a bar of 1000 — 72% to 82% short.

**It is met, outright, in 16 of 20 games.** Run on this branch at `--steps=300`:

```
Victory Conditions met outright: seed 1 Prospectors, seed 2 Prospectors, seed 5 Prospectors …
Prospectors (seat 1): 16 win(s)
The Prospectors' Venture Capital Fund at the end: median 1027
```

The 0.08.2 figure had been taken at collapse paces where **every game ends at turn 16**, and nothing
can be hoarded by then. The binding constraint was never the bar; it was how long the world lasts.
At the default pace the Fund still ends at a median of **130**; give the game to turn 30 and it ends
at **1027** against a bar of 1000, which is a bar sitting almost exactly on the median.

So this change is **thematic, not remedial**, and the job was to move the hoard to Ducats *without*
making a working Condition easier or harder. The designer, told that: *"proceed"*.

## What it is priced against

Measured per seat, median over a whole game, before anything changed:

| `--steps` (collapse near turn) | | Custodians | Prospectors | Arkwrights | Archivists |
|---|---|---|---|---|---|
| 140 (t16) | Ducat income | 198 | **1025** | 16 | 102 |
| 300 (t34) | Ducat income | 623 | **1993** | 31 | 443 |
| 300 (t34) | Materials income | 295 | **985** | 166 | 278 |
| — | **Ducats held at the end** | 5 | **6** | 6 | 4 |

That last row is the design fact. **Every seat ends every game holding about five Ducats.** They
spend the lot. A Materials hoard skimmed a resource they stockpile anyway; a Ducat hoard competes
with Influence bought, Relief paid and repairs made, every turn.

## The three decisions

**A share of Ducat income, not a relabelled share of output.** The designer chose *"a"*. Keeping the
old filling and merely counting the hoard in Ducats would have left the Victory Condition as "dig a
lot" with a currency sign painted on. Taking it at Income, before the seat can spend a coin, makes
it the decision a venture fund actually is: **bank it or spend it**.

**The bar is 2000, and it was derived rather than converted.** The tempting figure is 3000 — 1000
Materials at the market's 3 Ducats — and it would have been wrong, because Ducat income and
Materials output are different curves. Instead: Ducat income 1993 at `--steps=300`, times the 80%
share cap, times the ~30% the Investment Bank's interest adds on top (today's Fund reaches 1027 from
985 × 0.8 = 788), ≈ **2073**.

**The 80% cap stays**, at the designer's word, with the note that its top end is now a real trap
rather than nearly free — and with the requirement that *"the AI knows how to weigh its decision on
where to place their contribution."* It already does, and the rule needed only its resource
changed: a computer seat takes the **smallest share that still reaches the bar by its pace's last
turn**, so it banks the least it can and leaves the rest spendable.

## It landed where it was aimed

Same seeds, nothing else changed:

| | Fund at the end | bar | wins `[Cu, Pr, Ar, Ac]` | collapses |
|---|---|---|---|---|
| `--steps=300`, Materials, bar 1000 | 1027 | 1000 | [0, 16, 0, 1] | 3/20 |
| `--steps=300`, **Ducats, bar 2000** | **2049** | 2000 | **[0, 17, 0, 1]** | 2/20 |
| `--steps=420`, Materials, bar 1000 | 1026 | 1000 | [1, 18, 0, 0] | 0/20 |
| `--steps=420`, **Ducats, bar 2000** | **2145** | 2000 | **[1, 18, 0, 1]** | 0/20 |

**The median long game lands just across the line, exactly as it did before, and the win column is
unmoved.** The Condition now asks a different question and asks it just as hard.

## Two things that did not change, and one that quietly did

**The name stays.** "Venture Capital Fund" was always a finance word and fits Ducats better than it
ever fitted ore. **The glyph stays.** A Faction symbol is the most-reused image in the game and has
to hold at 16 pixels beside three others; the designer: *"keep both"*. The "plus 12 Colonists living
off Earth" half is untouched.

**A Bank now fills the Fund.** Ticket #72's test asserted the opposite in as many words — *"Ducats
are not Materials output, so a Bank banks nothing in the Venture Capital Fund however high the share
is set"* — and that assertion is now exactly inverted. The test was rewritten to say so rather than
deleted, because the inversion is the change.

The clause that kept the old Fund honest is kept, pointed at the new resource: **Ducats got by
selling are not income and never reach the Fund.** A hoard you can fill by trading is not a hoard.

## Looked at

| picture | what it shows |
|---|---|
| [`the-hoard.png`](the-hoard.png) | The Victory panel for a Prospector seat: *"2000 Ducats in the Venture Capital Fund, plus 12 Colonists living off Earth"*, the progress line **154 of 2000**, and the share control reading **"Banking 80% of Ducat income"** with **Draw 10 from the Fund** beneath. |

The first capture of this panel caught a line the code search had missed — the Condition's own
description still read *"1000 Materials in the Venture Capital Fund"* from `factions.toml`, beneath
a progress bar already counting Ducats out of 2000. Second time in this version a picture has found
a defect that reading did not.

## Tests

**The existing suite caught the change on its own**: nine tests failed the moment the resource
moved, which is the red witness arriving unasked. Eight were **aged** — the bar doubled in the
margin and ranking tests, the AI test fed Ducat income instead of Materials — and the ninth, the
Fund's own test, was rewritten with exact figures:

| | |
|---|---|
| Ducat income in the fixture | **14** a turn |
| at 50% | 7 banked, 7 landing, Fund **7** |
| at 80% | 11 banked, 3 landing, Fund **18** |
| a draw of 10 | Fund **8**, **9 Ducats** back to the Stockpile |

Then broken deliberately: banking put back on Materials output (**2 failed**), and the bar put back
to 1000 (**1 failed**). Restored: `314 passed; 0 failed`, `6 passed; 0 failed`, clippy clean with
the denial.

Removing the old filling left the `extraction` accumulator and the `extraction` flag on every
producer dead, and clippy said so. Both are gone: the flag existed for no other purpose.

Sweeps run with `target/release/examples/sweep.exe 20 --balance --steps=300,420`; the Fund's line
now names the bar it is being measured against. The capture is
`target/release/dying-earth.exe shot:<prefix> player:prospectors victory:1 turns:12 panel:0
window:1920x1080`, off-screen, exit 0.
