# Every Faction's gate chain in its own pick list

Ticket [#348](https://github.com/whaleyjoshua2/Dying-Earth/issues/348) on version 0.09.1. The
designer's line was *"each faction should have all the antecedent techs for their victory tech in
their list"*.

`SPEC.md` beside this file is the build specification, and the
[resolution comment](https://github.com/whaleyjoshua2/Dying-Earth/issues/348#issuecomment-5827061021)
on the ticket is the authority over it. `sweep-before.txt` and `sweep-after.txt` are the raw runs.

## The asymmetry this fixes

| Faction | gate | chain | Research | antecedents missing from its own list |
|---|---|---|---|---|
| Custodians | Planetary Stewardship | **3 Techs** | **98** | **none** |
| Prospectors | The Extraction Charter | 5 Techs | 148 | Beneficiation |
| Arkwrights | Generation Ships | 5 Techs | 148 | Efficient Grids, Clean Power |
| Archivists | The Upload | 5 Techs | 148 | Expanded Habitats, Clean Power, Closed-Loop Colonies |

Only the Custodians could reach their own gate, and only they had a short chain.

**The shortlist made it worse.** The draw force-includes the Lead's own gate, but only
`if available.contains(&gate)`, and a Tech is available only once its prerequisites are done — so
the guarantee could not fire until the chain had already been climbed by luck. The playtest called
that lottery the thing that decides the game.

## What changed

- **Each pick list opens with its own chain**, cheapest first. Three lists changed; the Custodians'
  was already complete.
- **The shortlist carries the next rung.** Alongside the gate, the draw force-includes the cheapest
  unresearched antecedent of the Lead's gate that is itself available. **This is the half that
  reaches the human player**, because the Lead's draw is the Lead's draw whoever holds it.
- **The reserved place stays one slot of three.** The gate is available only when every antecedent
  is done; an unresearched antecedent exists only when one is not. The two cannot both fire, so the
  shortlist did not need to grow. A test asserts no draw ever forces both, and the build found the
  one condition on that claim: it holds for every state Research can actually reach, because
  `pick_tech` refuses anything unavailable, and the test skips unreachable states with the reason
  written down.
- **The Prospectors lost both denial levers**, `last = "clean_power"` and
  `never = "green_consensus"`.

## The specification was wrong, and it took two rounds to find out

`SPEC.md` kept `never = "green_consensus"` on the grounds that *"Green Consensus is nobody else's
antecedent"*. **It is the Custodians' only rung-2 antecedent**: Planetary Stewardship needs it and
nothing else in the tree does. The build lane caught it, left the line in place because it was not
its call, and corrected the false reason in `ai.toml` rather than repeating it. Put to the designer,
the answer was to drop it.

The two levers are not the same thing, and the difference is why `never` survived a round:

- **`last`** governs what a Faction **picks** while it holds the Research Lead, which stalls the
  whole table at once.
- **`never`** governs its **Research Directive**: for as long as the table researches that Tech, the
  Faction diverts its own Research out of the shared pot and into its coffers. It starves the Tech
  more slowly, costs that Faction its contribution to everything else, and arrives in the same
  place.

A Faction refusing to fund a rival is a fine idea and this is not a ruling against it. It is a
ruling that the target must not be the only road another Faction has to its Victory Condition.

The rule's test now checks **both** levers against **every** other Faction's chain, computed from
the tables rather than pinned, so it holds if the tree changes.

## The measurement

`sweep -- 20 --seatings --balance --steps=300 --sinks=6`, eighty games, per-Faction totals read at
the foot. The per-Faction gate line is new this ticket: the existing one reads by seat, and seat 0
rotates between Factions across seatings.

| | before | lists + shortlist | and `never` dropped |
|---|---|---|---|
| Custodians, wins of 80 | 3 | 4 | **3** |
| Prospectors | 30 | 40 | **40** |
| Arkwrights | 1 | 2 | **2** |
| Archivists | 0 | 0 | **0** |
| collapses | 45 of 80 | 34 | **35** |

**The figure this ticket is judged by moved the right way for every Faction.**

| gate completed | before | after |
|---|---|---|
| Custodians | 62 of 80, turn 25 | **69 of 80, turn 22** |
| Prospectors | 54 of 80, turn 27 | **63 of 80, turn 27** |
| Arkwrights | 52 of 80, turn 24 | **59 of 80, turn 23** |
| Archivists | 43 of 80, turn 28 | **44 of 80, turn 28** |

Every gate lands in more games and none lands later. Dropping `never` did exactly what it was meant
to: the Custodians' gate went from 68 games at turn 24 to **69 at turn 22**, the earliest of the
four.

**Three things worth saying plainly rather than dressing up.**

- **The wins table went the wrong way.** The Prospectors took ten of the eleven new wins and now
  take **half of all games**. Being able to reach a gate is worth most to the Faction that was
  already winning.
- **The Custodians won one fewer game despite reaching their gate sooner and more often.** One win
  in eighty is inside the noise of this sample, but it says their gate was not what was holding them
  back, and neither was the Prospectors' denial of it.
- **The Archivists are still on nought**, with their gate landing in 44 of 80 games. Their problem
  is downstream of the Tech — the Archive, or the bar itself — which is exactly the ticket this one
  unblocks.

**The collapse rate fell from 45 of 80 to 35**, which on this project is the figure moving the wrong
way: more gates landing means more Planetary Stewardship and more Clean Power, so the world burns in
ten fewer games. Accepted and recorded at the designer's word as this map's baseline for the balance
version, rather than re-fitted here against a twenty-seed signal.

Two knock-ons, reported as figures: Missile Carriers built went 1 to 8 and Fuel burned in Battle 4
to 64, because the tree now runs further up Propulsion.

## The gate

Clippy clean with `--workspace` and the trailing denial. **464 tests pass**, none ignored, none
skipped, **no pre-existing test amended**. `SAVE_VERSION` untouched.

Five red witnesses, each watched fail on its own rule. The build reports one check it had to fix
because it passed on a state broken on purpose: the first mutation for the mutual-exclusion test
broke the forcing clause rather than the premise, which certified nothing, so the premise was broken
instead and the assertion then fired.
