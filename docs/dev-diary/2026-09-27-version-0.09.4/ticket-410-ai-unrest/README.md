# How the computer seats manage Unrest (ticket #410)

[Ticket #410](https://github.com/whaleyjoshua2/Dying-Earth/issues/410); the spec is §7 of
[`docs/spec/version-0.09.4.md`](../../../spec/version-0.09.4.md). Measured first, then decided in one
round (*"q1 a b c d q2 ok"*).

## The measure

The sweep gained a line per Faction: Region-turns held, those at Unrest 4+ and 7+, throw-offs, and
seat 0's start Region lost to a throw-off or a taking. [`../sweeps/before-410.txt`](../sweeps/before-410.txt)
is the baseline the designer decided on: a third or more of every Faction's held Region-turns at
4+, one in seven at 7+, 103 throw-offs in 80 games, 17 of 45 start Regions lost to a throw-off.

## The change and the sweep

[`../sweeps/after-410.txt`](../sweeps/after-410.txt): the share at 7+ falls for all four (14 / 19 / 13
/ 17% to 9 / 14 / 5 / 11%), throw-offs 103 to 83, start Regions thrown off 17 to 11. Wins 8 / 4 /
1 / 7 to 7 / 6 / 1 / 5, collapses 60 to 61: the designer's bar met, with no Faction's wins
collapsing. One round, no tuning.

## The red witness

`a_computer_seat_answers_unrest_from_four_and_raises_a_stadium_alone_where_slots_are_short`, on the
gate the candidate list reads (`ai_offers_calming`), red with `constabulary_from` put back to 5
(*"a Constabulary at four"*). A first form of the test read the seat's orders, where a Constabulary
offered at 4.5 lost to better-weighted builds; the gate is what the rule is.

## The review

An agent that did not build it reproduced both sweeps byte for byte and found the four changes as
decided. Fixed from it: the spec's *"of 45"* after is 44; a Stadium could be offered *"alone"* while
a Constabulary on order already had the last slot (never seen in 80 games; the rerun is identical);
the Stadium's floor and Relief's double are read from `ai.toml`; a doc line had slipped onto the new
function; two comments still named the old thresholds; and the Mothball price and the Relief curve
now have a test (`a_computer_seat_prices_a_mothball_and_weighs_relief_by_unrest`, red with the price
set to one).
