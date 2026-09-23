# Passage as a rule for Armies

Ticket [#320](https://github.com/whaleyjoshua2/Dying-Earth/issues/320) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`china-move-to-partner-hover.png`](china-move-to-partner-hover.png) | `shot: select:eastasia army:1 threat:1 passage:1 panel:0 window:1280x1080 tip:a partner under Passage`. China's card with Russia held by the Prospectors and a Passage Accord standing: the 2nd Chinese Army's first button reads **move to Russia** where it read *attack Russia*, and its hover: *Russia: held by the Prospectors, a partner under Passage. Moving costs nothing; the Army arrives on Hold and fights nobody while the Accord stands.* The other three neighbours still read *attack*. |

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes q4 yes q5 yes"*: Passage lets an Army march into a partner's held Region on
Hold, without a Battle; no new offence; a partner's Army in your Region fights nobody, defends
nothing, and is on Attack at the next Resolution once the Accord ends; a partner is no target for
Intercept and not shut out by a Blockade; the computer offers Passage to a Friendly neighbour,
accepts at Neutral or better, and never targets a partner's Region.

## What was built

- **The march** (`resolution.rs`): a march into a Region held by a partner under Passage sets Hold
  and is not counted as a march on a held Region; otherwise Attack, as before.
- **The guest** (`guest_at`): an Army standing in a Region held by another seat under Passage, not
  on Attack, is no defender of the place and no party to a Battle there. At each Resolution's
  ground pass, a rival Army standing in a held Region with **no** Passage behind it, on Hold or Dig
  In, is put on Attack: the guest whose Accord ended, as any rival Army in a Region not its own.
- **Ships**: the Intercept loop skips a partner under Passage; `slot_blockaded_against` ignores a
  partner's blockader.
- **The computer seats**: Passage is accepted at Neutral or better (Wary before); it is offered
  **in the same Accord as non-aggression** to a Friendly seat that holds a Region next door to one
  of the offerer's (`passage_worth_offering`); a partner's Region under Passage is never a march
  target.
- **The card**: a partner's Region under Passage reads *move to* with a hover saying why; the term's
  description in the Faction window and the **Terms** glossary entry say the rule.
- A `passage:1` shot aid strikes the Accord for the picture.

## Tested

`passage_lets_an_army_march_into_a_partners_region_on_hold` (under Passage: Hold, no march
counted, no Battle, no offence; without: Attack, counted) was watched to fail against a march made
to ignore the Accord, then to pass; `a_blockade_does_not_shut_out_a_partner_under_passage` passes;
the workspace suite is green at 357.

## Measured

Two batches of 20 seeds x four seatings. **The first**, with Passage offered on its own to a
Friendly neighbour: `passage 0` standing at the end in every seating, and the war lines unchanged.
The cause was in the code: one Accord stands per pair, and a non-aggression Accord struck first at
Neutral shut a later Passage offer out. **The second** (`batch-after.txt`), with Passage folded into
the non-aggression offer when the pair is Friendly and adjacent: **`passage 0` again**, the war
lines and the column unchanged (5/13/0/0, 6/14/0/0, 2/5/1/0, 0/20/0/0). A seed simulated headlessly
says why: **no pair of computer seats is Friendly at any point in thirty-six turns** (median
Relations over the batch are -3 to -7), so the offer's bar, which the designer set at Friendly, is
never reached. The rule reaches the computer seats; today's table is too cold for them to use it.
Lowering the bar to Cordial is a one-word change and a designer's call, not made here.

## Settled by the builder, to be corrected if wrong

- **The guest turned to Attack** when its Accord ends also catches an Army standing in a Region
  that a rival has since taken by Influence, which sat idle on Hold before; it now attacks, as *any
  rival Army in a Region not its own* would. Said here because it is a second case the rule
  reaches.
- **Passage in the non-aggression offer** rather than a second offer, for the reason measured
  above.
