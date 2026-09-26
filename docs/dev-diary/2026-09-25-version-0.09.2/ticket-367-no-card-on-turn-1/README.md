# Ticket #367: no card on the first turn

The designer: *"do not draw an event card on first turn."* Decided on the ticket in one round: no
card of either kind, the deck untouched, the turn a figure.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/367) is the authority;
[§1 of the spec](../../../spec/version-0.09.2.md#1-no-card-on-the-first-turn) records it.

## What was built

- `first_draw_turn = 2` in `events.toml`, refused by the loader at nought.
- The Question phase returns before rolling while the turn is below it. The deck is not touched,
  so the top card waits for turn 2's ordinary roll.
- The driver's help says so, and the glossary's **Draw Chance** entry carries the clause.

## The red witness

`no_card_of_either_kind_is_drawn_on_the_first_turn_and_the_deck_is_untouched`, run with the figure
loaded but the rule absent:

    assertion `left == right` failed: seed 1: turn 1 drew Choice(ConscriptionNotice)

and `the_loader_refuses_a_first_draw_turn_of_nought`, on a copy of the data folder with the one line
changed to 0:

    a first_draw_turn of 0 is refused

Both green after the rule. The rule test also checks that turn 2 still rolls a coin: over sixty
seeds some draw and some do not, so the rule holds off turn 1 alone.

## The sweep after it

[`../sweeps/after-367.txt`](../sweeps/after-367.txt), 20 seeds x four seatings at the shipped cell,
against 0.09.1's closing **6 / 28 / 3 / 2**, collapses 41:

| | 0.09.1 closing | after #367 |
|---|---|---|
| Custodians | 6 | **8** |
| Prospectors | 28 | **28** |
| Arkwrights | 3 | **0** |
| Archivists | 2 | **2** |
| collapses of 80 | 41 | **42** |

Gates completed 70 / 63 / 55 / 39 of 80, against 73 / 58 / 50 / 40. The Arkwrights' three wins went
to nought; whether that is the missing turn-1 card or the seeds is not measured here, and it is
reported as a figure, not chased. The rule reaches every seat the same way, since the phase is one
phase for all four.

One test fixture moved with the rule: three tests that force a card by rolling the Question phase
on a fresh game now stand it on turn 2 first, since turn 1 no longer rolls.
