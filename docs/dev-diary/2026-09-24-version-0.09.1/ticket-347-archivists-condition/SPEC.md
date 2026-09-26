# Ticket #347: the Archivists' Condition. The build specification

Authority: the resolution comment on
[ticket #347](https://github.com/whaleyjoshua2/Dying-Earth/issues/347), decided by the designer on
2026-09-25. Where this file and that comment disagree, the comment wins and this file is wrong.

## Prior art, searched before writing

Read out of the tree on 2026-09-25, with
[Every Faction's gate chain in its own pick list](https://github.com/whaleyjoshua2/Dying-Earth/issues/348)
already landed.

- **`archive_fund_cap` (`state.rs:1927`)**: `if archive_built { a.research } else { floor(a.research
  * a.banked_before_built) }`. With `research = 80` and `banked_before_built = 0.25`
  (`modules.toml:137-139`) that is **20** until the Module stands.
- **The Victory figure is clamped by it** (`victory.rs:116`):
  `VictoryFirstKind::ArchiveResearch => s.archive_fund.min(self.archive_fund_cap(seat)) as f64`.
  The score is a fraction of `victory_first.bar`, which is **80** (`factions.toml:189`). So before
  the Module stands the Archivists' first part **cannot exceed 0.25**, whatever they bank.
- **Six call sites of the cap**, all traced: `state.rs:1927` (the definition), `victory.rs:116` (the
  clamp), `orders.rs:761` and `:765` (the refusal and its message), `research.rs:344` (the banking
  clamp), `ai.rs:1633` (the seat stops directing at the cap), `src/ui.rs:6880` (the display).
- **`archive_complete` (`state.rs:1933`)** is `archive_built && archive_fund >= tables.archive.research`.
- **The Archive's other gates**: 50 Materials, 12 Widgets, 12 Energy upkeep charged only once
  complete, the Archivists alone, a Body off Earth, **The Upload** researched, and **four Colonists
  living there**, checked once at the order.
- **The Archivists may divert up to 100%** (`factions.toml:390`, `archivists_max = 100`) where every
  other Faction is capped lower; `research_directive_cap` (`research.rs:433`) is the one door.
  `research_multiplier` 1.5 and `research_multiplier_off_earth` 1.75 (`factions.toml:180-181`).
  `provisional_min_contribution = 75` (`factions.toml:395`).
- **Measured, eighty games, on the current build**: Archivists **0 wins of 80**; the Archive
  **standing in 4 of 80, complete in 1 of 80**; **median fund at the end exactly 20 in every
  seating**; their Victory gate completed in 44 of 80 at median turn 28.
- Nothing found that does what is specified; the finding is a negative.

## The rules

### R1. One figure, 125

- `[archive] research` becomes **125** in `modules.toml`.
- `victory_first.bar` for the Archivists becomes **125** in `factions.toml`, and the Victory sentence
  is rewritten to say 125 rather than 80.
- **They must stay one figure.** A test asserts `tables.archive.research` equals the Archivists'
  `victory_first.bar`, so the two cannot drift apart in data.

### R2. The quarter-cap goes entirely

- `banked_before_built` is **removed** from `modules.toml` and from its card in `data.rs`.
- `archive_fund_cap` **survives** as a cap at the Archive's own figure — nobody may bank more
  Research than the Archive needs — but loses its pre-build branch. It returns
  `tables.archive.research` always.
- `research.rs:344`'s banking clamp stands, now at 125.
- **`orders.rs:761-765`**: the refusal stands when the fund is **full**, but its message — *"the
  Archive fund holds its quarter (20) until the Archive stands at a Colony off Earth"* — is now
  false in every clause and must be rewritten to say the fund is full at 125.
- `ai.rs:1633` is unchanged in shape; the seat now directs until 125 rather than 20.

### R3. The Victory figure is not clamped

- `victory.rs:116` reads **`s.archive_fund as f64`**. The `.min(archive_fund_cap(seat))` goes.
- It is redundant as well as wrong once R2 lands, since the fund can never exceed the cap at
  banking — but it is **removed rather than left**, because a clamp that looks live and is not is
  the thing a reader trusts and a future change breaks.
- **This is the rule that unlocks them.** A test proves an Archivist who has banked 60 with no
  Archive standing scores 60/125 and not 0.16.

### R4. The Module's other gates do not move

- The Upload, off Earth, four Colonists, 50 Materials, 12 Widgets, 12 Energy: **all unchanged**.
- A test pins them, so this ticket cannot quietly move the journey while changing the money.

### R5. What the game says

- `src/ui.rs:6880` reads **`fund / 125`** with **no cap clause**. The playtest's complaint that
  *"20 of a cap of 20"* reads like a bug goes with the cap.
- `CONTEXT.md`: **Archive** and **Research Directive** both carry the figure and the cap; both move.
  Glossary register only.
- The Victory panel and the Faction window read 125 wherever they read 80.

### R6. What the sweep must say

The sweep already reports the Archive line. It must report, and the closing note must quote:

- the Archivists' **wins of 80**, against a baseline of **0**;
- the Archive **standing** and **complete** counts, against **4 of 80** and **1 of 80**;
- the **median fund at the end**, against **20**.

**If the Archive still stands in about 4 games of 80, this ticket did not unlock the Archivists**
and the report must say so in those words. The money and the Module are separable problems; the
money is what was asked for, and a bar change dressed up as a fix would be worse than no fix.

**A rising collapse rate is a figure on this project, not an alarm.** Report it as a number.

## Error cases

- `banked_before_built` left in the data file after the field is gone: the loader must refuse an
  unknown key, or the test that asserts the two figures are one must fail. Say which happens.
- `[archive] research` and the Archivists' `victory_first.bar` out of step: refused by R1's test.

## Out of scope

- **Why the Archive is nearly unbuildable.** If the closing sweep shows it still is, that is a
  second ticket, charted after this one reports.
- The second Victory part, twelve Colonists uploaded.
- Any change to The Upload, to the Colonist gate, or to the Research Directive's own costs.
- `SAVE_VERSION` does not move.

## Refutation

The specification is wrong if an Archivist's first Victory part is still clamped at a quarter before
the Module stands; if the Archive's cost and the Victory bar are two different figures; if a Faction
can bank more Research than the Archive needs; if the refusal still speaks of a quarter; if any gate
on the Module moved; or if the interface still shows a cap.

## Red witnesses owed

One test per rule R1 to R5, each watched red against the rule reverted on purpose, with the failing
assertion quoted. R3's is the one that matters: it must fail on the old clamp with an Archivist
scoring 0.16 where it should read 0.48.
