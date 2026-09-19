# Blame moderates the decay of a Standing

Ticket [#266](https://github.com/whaleyjoshua2/Dying-Earth/issues/266) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

No picture: the rule has no surface of its own. It shows in one place -- the challenger line's
hover on a held card now says *"theirs 3 for their Blame"* where it said *"theirs 2"* -- and a
hover cannot be photographed (ticket #242).

## The model the designer set out

Asked which way "moderate" runs, the designer answered *"both"* and then put the whole thing
straight: **Blame is a ledger.** It begins from physics -- ppm emitted less ppm removed -- and is
moved by deals and words: carbon credits bought or sold move ppm between ledgers, and a propaganda
campaign lays ppm on its target's. The share every rule reads is computed from the ledger, *"in
this way actual share will diverge from blame"*. That sentence is in the glossary's **Blame** entry
now, and it changes two tickets still open: the propaganda campaign adds to the ledger directly
(no separate "attributed" term), and a carbon credit is a ledger move.

## What was decided, in the designer's words

- *"both"* -- a dirty Faction's Standing erodes faster, a clean one's slower.
- *"step rule"* -- because the multiplier rounded to a whole number could only ever bite at the
  cap and never slow anything (2 x 0.75 rounds to 2): on a Region the Faction does not hold, a
  share **at or below an eighth** decays **1** a turn, **at or above a half** decays **3**, and
  anything between the plain 2. `decay_slow_below`, `decay_fast_from`, `decay_slow`, `decay_fast`
  in `influence.toml`.
- *"go with ti"* -- not-held Regions only, both directions; a held place keeps its 1.
- *"round to nearest"* -- moot under the step rule.
- *"same exclusion"* -- a Colony or a station is untouched; Blame is Earth's resentment.

This reverses, on purpose, the *"never on Standing decay"* that ticket #53 wrote into the Blame rule
in version 0.05. The data comment and the glossary say so.

## A consequence the suite found

With no Blame on the table at all -- turn 1, before the first Climate phase -- every seat's share
is nought, which is at or below the eighth, so **everyone's Standing on Regions they do not hold
decays 1 on the first turn**, not 2. An older test pinned the 2 and went red. The rule is read
as written and the test now pins the 1 with a note; from the first Climate phase on, shares exist
and the plain 2 returns for everyone between the steps.

## Witnessed red

`standing_decay_for` was written first knowing only held-or-not, and the test -- a seat at 0.6
loses 3 on a Region it does not hold, one at 0.05 loses 1, one at 0.3 loses 2, a Colony 2 and a
held place 1 whatever the share -- run against it: *"dirty, elsewhere: left 2, right 3"*. Then
the step rule went in; `324 passed`, clippy clean with `-D warnings`.
