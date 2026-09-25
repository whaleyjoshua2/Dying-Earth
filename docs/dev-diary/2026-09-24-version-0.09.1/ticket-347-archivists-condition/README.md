# The Archivists' Condition: 125 Research, and the quarter-cap gone

Ticket [#347](https://github.com/whaleyjoshua2/Dying-Earth/issues/347) on version 0.09.1. The
designer's line was *"get rid of the 20 of 80 cap for archivists instead they now require 125
research"*.

`SPEC.md` beside this file is the build specification and the
[resolution comment](https://github.com/whaleyjoshua2/Dying-Earth/issues/347#issuecomment-5834465393)
on the ticket is the authority over it. `sweep-before.txt` and `sweep-after.txt` are the raw runs.

## Said first: this did not unlock the Archivists

**The Archive still stands in 5 games of 80.** The money is fixed, the interface no longer lies, and
the Module is where they die. The closing sweep says so and this file says so rather than dressing
a bar change up as a fix.

| | before | after |
|---|---|---|
| **Archivists, wins of 80** | **0** | **1** |
| Archive **standing** | 3 of 80 | **5 of 80** |
| Archive **complete** | 1 of 80 | **4 of 80** |
| median fund at the end | **20 in every seating** | **125, 20, 125, 62** |
| Custodians / Prospectors / Arkwrights | 3 / 40 / 2 | 4 / 37 / 2 |
| collapses | 35 of 80 | 36 of 80 |

**Their second part is the wall, and it is structural.** Twelve Colonists uploaded needs the Archive
**complete**, so their wins can never exceed the complete count, which is 4 of 80. No change to the
money can move that.

**The one win is a ranking win, not a Condition met.** That seating's outright list names two seeds,
both Prospectors. The Archivists took a win from the end-of-last-turn ranking, which is exactly what
removing the clamp should do: their score is now the lesser of two real fractions instead of a first
part nailed at 0.25, so they out-rank seats they used to lose to.

**The median fund of 20 in the second seating is a coincidence, not the old cap.** The cap is off
the struct and the build would not compile with it. That seating starves the Archivists — median
off-Earth Research nought, median Ducats 32 — and 16 of its 20 games collapse by turn 26. The other
three seatings end at 125, 125 and 62.

## What changed

- **125 is one figure**, replacing 80 as both the Archive's running cost and the first Victory bar.
  They were always the same number in two files; now the **loader refuses to start the game** if
  they disagree, naming both.
- **The quarter-cap is gone entirely.** `banked_before_built` is off the card, so Research may be
  banked freely from turn 1.
- **The Victory figure is no longer clamped.** `victory.rs` reads the fund itself. **This is the
  part that unlocked anything at all**: raising the bar to 125 while leaving the score capped at a
  quarter would have made them strictly worse off.
- **The Module's gates did not move**: The Upload, a Body off Earth, four Colonists, 50 Materials,
  12 Widgets. A test pins them so the ticket could not quietly change the journey while changing the
  money.
- **The interface stopped lying.** The Archive block is one line, `Archive fund 31 of 125`, with no
  cap clause. The playtest's complaint that *"20 of a cap of 20"* read like a bug goes with the cap.

## The pictures

| picture | what it shows |
|---|---|
| [`victory-panel-31-of-125-no-module.png`](victory-panel-31-of-125-no-module.png) | *"The Archive: 31 of 125 — needs The Upload, not yet researched"*, with **no Archive standing** and the bar a quarter along. Under the old rule this read *20 of 80* and could never move. |
| [`colony-card-fund-31-of-125.png`](colony-card-fund-31-of-125.png) | The Colony card: *"The Archive / Archive fund 31 of 125"*, one line, no cap clause. |
| [`victory-panel-62-of-125-module-standing.png`](victory-panel-62-of-125-module-standing.png) | 62 of 125 with the Module up. |

Both of the first two carry a Choice Card modal over part of the globe, an artifact of the scene
that draws one; the lines that matter are unobstructed in each.

## Six things the specification got wrong

The build found all six and said so plainly rather than working around them.

1. **The error case did not exist.** The spec said a stale `banked_before_built` would be refused by
   the loader or caught by a test. **Neither.** No struct carried `deny_unknown_fields`, so serde
   would have ignored the dead key silently with every test green — the exact drift the ticket
   exists to stop. The build added `#[serde(deny_unknown_fields)]` to the Archive card and witnessed
   the refusal.
2. **A seventh use of the figure.** `src/shot.rs` read `banked_before_built` **directly**, not
   through the cap function, so the tree would not compile without touching it. The spec claimed six
   sites, all traced.
3. **The red figure in R3 was wrong.** The spec predicted an Archivist scoring 0.16 under the old
   clamp. That is 20/125, a state this ticket never passes through. The real reading is 31 of 125,
   0.248. The direction was right and the number was not.
4. **The baseline was wrong.** The spec and the brief both said the Archive stands in 4 of 80 on the
   current build; measured on the untouched tree, it is **3 of 80 standing and 1 complete**. Every
   other baseline figure matched exactly.
5. **A glossary entry was misnamed.** The cap is described in **Fund the Archive**, not **Research
   Directive**.
6. **`ai.toml` named the dead field** as "the lever held in reserve" for lifting the Archivists. The
   lever has now been pulled and the comment says so.

## The gate

Clippy clean with `--workspace` and the trailing denial. **469 tests pass**, none ignored, none
skipped. `SAVE_VERSION` untouched: an old save's fund simply stops being read through a cap.

**Five rules, eight red witnesses**, each watched fail against its own rule reverted on purpose.
R1 owes three, because the two figures and the dead field fail in three different ways; R3 owes two,
because the clamp and the cap are separable and only one of them is the unlock.

**One check was wrong and was fixed rather than worked around**: R4's Earth clause asserted the
refusal contains *"off Earth"*, and the real refusal has said *"at a Colony on another Body"* since
ticket #209. The assertion was wrong, not the rule.

**Six pre-existing tests were amended**, every one because it hard-coded 80. Each now reads
`tables.archive.research`, so they follow the data rather than pinning a figure the designer can
move.
