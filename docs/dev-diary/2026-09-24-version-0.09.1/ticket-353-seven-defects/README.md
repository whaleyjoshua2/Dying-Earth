# Seven defects the playtest found

Ticket [#353](https://github.com/whaleyjoshua2/Dying-Earth/issues/353) on version 0.09.1. Seven
things the game said that were untrue, found together by four playtesters in
[`docs/dev-diary/2026-09-24-playtest-0.09.0/README.md`](../../2026-09-24-playtest-0.09.0/README.md).

`SPEC.md` beside this file is the build specification, and the
[resolution comment](https://github.com/whaleyjoshua2/Dying-Earth/issues/353#issuecomment-5825574206)
on the ticket is the authority over it.

## The seven, before and after

| # | before | after |
|---|---|---|
| **1** | `no Shipyard here`, whether it was absent, still building or shut | three refusals: `no Shipyard here` / `the Shipyard here is still building` / `the Shipyard here is shut: mothballed, or dark for want of Energy` |
| **2** | *"founded a Colony in slot 1 on the Moon"* | *"founded a Colony at **Mare Tranquillitatis** on the Moon"* |
| **3** | nothing, anywhere | *"3 more found no Habitat room at Mare Tranquillitatis on the Moon and did not land."* |
| **4** | `REFUSED lift east-asia 6 15: ISS over Earth has Habitat room for 2 more` | it fills: *"2 Pioneers lifted from China to ISS over Earth."*, and *"4 more found no Habitat room at ISS over Earth and are still waiting in China."* |
| **5** | *"set its Labs to pay the Archive fund"*, for all four Factions | *"directed its Research into its coffers"* / *"into propellant"* / *"into the Natural Sink"*, and the Archive fund for the Archivists alone |
| **6** | a quiet turn opened with **no headline at all** | it falls through: *"Arkwrights completed Research Lab at Saudi Arabia."* |
| **7** | `build archive <colony>  the Archivists only` | the three rules the engine enforces: off Earth, four Colonists, and The Upload standing |

**Defect 1 was the worst of them** and the reason this ticket led the build order: all four
playtesters hit it and one lost six turns to it. Version 0.09.0's headline improvement was that a
refusal names the rule, and this one folded three cases into a word that was false in two of them.

[`headline-falls-through-to-the-build.png`](headline-falls-through-to-the-build.png) is defect 6
demonstrated: the four answers to the turn's Choice Card are the loudest thing that happened, and
the Report now opens on the build below them instead of opening on nothing. The answers stand once,
together, under *The Auditors: what the table answered*, each in its Faction's colour.

## The sweep moved, and the designer ruled on it

**This ticket was specified to change nothing measurable**, and it did not hold. The closing sweep
reads **Custodians 3 of 80 against a baseline of 4, and Prospectors 30 against 29**. Collapses are
unchanged at 45 of 80, as are every other line.

**The cause was isolated rather than guessed.** Reverting defect 4's gate alone, leaving the other
six fixes in, returns the baseline exactly. The mechanism was then counted: **38 lift orders over
eighty games are now accepted-and-clamped where they used to be refused outright**, because several
Regions can nominate the same station in one turn and the second order finds less room than it
asked for. Those 38 partial lifts move people, emissions and Influence, and one win with them.

**The ticket's premise was wrong.** It said the `lift` lie was *"only reachable from the driver"*.
It is not: `check_order` is the one door both the player and the computer go through, so a refusal
that becomes a clamp cannot be made visible to one and invisible to the other without a per-seat
rule, which this tree does not have.

Put to the designer with both options, they chose to **keep the fix**: one win of eighty traded,
collapses untouched, in exchange for the driver finally doing what its own help has promised since
version 0.07.3. The cost is worth naming: the win comes off the **Custodians**, already the weakest
Faction at three or four of eighty.

## Fixed beyond the seven

- **Defect 1 was a class, not an instance.** Ten gates in `orders.rs` ask whether a building is
  standing. Only the Shipyard asserted an absence outright, but **two more understated the same
  gate** and were fixed with a word each: building a station over Earth, and a Repair, both of which
  read `working()` and then told a seat to get a Launch Site it already had, mothballed.
- **Defect 3 named three clamp sites and there are four.** The fourth is a disembarkation into a
  Colony that already stands, which clamped silently too.
- **Defect 2 left one number behind.** An empty slot's card header read *"Mare Tranquillitatis,
  Colony Slot 3 on the Moon"* — the name **and** a number, and the number was `slot + 1` while the
  driver counts from nought, so the two surfaces disagreed about which slot it was. The number is
  gone; the name was always enough.
- **Defect 4 gained a second line**, at the designer's word when asked whether a clamped lift should
  say who stayed: *"yes"*. A partial unload got that line in this ticket, and a partial lift was the
  same silence one step earlier in the journey.

## What was found and NOT fixed

**A mothballed Barracks still builds an Army.** Nine of the ten gates call `working()`; the Barracks
is the tenth and does not, so a Colony that mothballs its Barracks to save Energy still raises
Armies from it while the Shipyard beside it builds nothing. That is the **opposite** error to defect
1 and it is a rule rather than a message, so it was deliberately left alone and charted as
[A mothballed Barracks still builds an Army](https://github.com/whaleyjoshua2/Dying-Earth/issues/359).

## The gate

Clippy clean with `--workspace` and the trailing denial. **460 tests pass**, none ignored, none
skipped, no pre-existing test amended. `SAVE_VERSION` untouched at 5.

**Every one of the eight fixes was watched red on its own defect**, twice: once before any fix
existed, and again by restoring each defect one at a time against the finished tree. These are all
messages, and a suite does not notice a message, so a test never seen red certifies nothing here in
particular.
