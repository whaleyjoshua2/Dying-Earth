# Ticket #394: the computer's appetite for a rival's Colony scales with its size

The designer's item: *"the AI now scales its desire for a station/colony on its size."* The first
round read it two ways (the Faction's size or the place's) and the designer set both aside with the
intent: *"a counter check to founding one colony or station and just loading people and building on
it ... I'd like the AIs to want to take possession of a colony with high population. Let me know if
this mechanic is already in play."* It was not (the ticket's resolution says what was), and the
second round decided it (*"q1 a q2 a q3 a q4 colonists + output"*). The first build's sweeps sent
two shapes back to the designer, who reshaped the bounty (*"q5 a q6 a but make it 26 and give it a
factory too"*): rescaled to the board, the fattest Colony or station the 3.
[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/394) is the authority,
[§14 of the spec](../../../spec/version-0.09.3.md#14-the-computers-appetite-for-a-rivals-colony-scales-with-its-size)
records it.

## What was built

`Game::ai_place_size(colony)`: Colonists plus the place's output a turn (ticket #391's
`place_output`, Energy only where positive). `Game::ai_bounty(colony)`: `bounty_top` x size / the
largest size on the board, never dividing by less than `bounty_floor`, the designer's shape. The
Influence targets, extracted into `ai_influence_targets` so their order can be witnessed, weigh a
rival's Colony by its price rank times its bounty; the Army landing at a rival's Colony takes the
bounty on its base weight. Two figures in `ai.toml`: `bounty_top` 3.0 (the old price rank's
ceiling) and `bounty_floor` 26.0 (the designer's, given as a Colony of eight with a Mine, a Generator
and a Factory; such a Colony sizes about 21 on the data, so the floor sits a little above it).

## The two shapes set aside

The first build had the bounty as 1 + 0.05 x size, capped at 3, multiplying the price rank. Its
witness stayed red: a Colony of eight people and three Modules (price 200, bounty 2.25) weighed 0.9
against a lean one's 1.3 (price 80, bounty 1.3), since the price term falls 2.5x faster than that
bounty rises. Its sweep, since overwritten, read 6 / 7 / 0 / 5, collapses 62, Colonies taken by
Influence 11 / 12 / 17 / 28 by seating (6 / 5 / 7 / 22 before). The second had the bounty alone,
the price no longer ranking: the witness went green and the sweep,
[`sweeps/after-394-bounty-alone.txt`](../sweeps/after-394-bounty-alone.txt), read **2 / 1 / 0 / 8,
collapses 69**, Colonies taken by Influence 0 / 0 / 3 / 2: the seats spent their Allotment on
places priced 200 and more while their Regions were taken from them. Both went to the designer,
whose answer was the rescaling: linear in the size, with no "1 +", so a fat place wins the rank
while the price still counts.

## The red witness

`a_computer_seat_wants_a_rivals_fat_colony_more_than_its_lean_one`: two Colonies of a rival's on
Mars, one of two Colonists and nothing built, one of eight with a Mine, a Generator, a Factory and
a Habitat; the fat one's price by Influence is higher, and it outranks the lean one among the
seat's Influence targets all the same; and once a place reaches the floor it is the board's top,
a fatter one measured at 3 and the fat one under it; and no bounty under the least, so the lean one
keeps the weight it had. Red with `bounty_top` at 0, where every Colony is the least and the price
ranks the lean one first again; green at 3. The fat Colony carries a Refinery as well, since the
witness's first board (a Mine, a Generator and a Factory on a Mars slot) came to a size of
twenty-one, under the floor, and with the least at 1 a bounty of 2.4 at a price of 200 weighed 0.97
against the lean one's 1.0: the same arithmetic that bounds the lift by Influence at nine
Colonists, named in the spec.

## The sweep

[`sweeps/after-394-least-zero.txt`](../sweeps/after-394-least-zero.txt), the rescaled bounty with
no least, against the Trade Post's [`sweeps/after-397.txt`](../sweeps/after-397.txt): **8 / 3 / 1 /
7, collapses 61** (6 / 5 / 1 / 8, 60 before); Colonies taken by Influence, by seating, **1 / 0 / 5 /
5 against 6 / 5 / 7 / 22 before**. The computer took fewer Colonies, and the reason is in the
shape: the bounty is linear and measured against the board's fattest Colony or station, so a lean Colony (a
computer's usual, size six or so against a floor of 26) weighed about 0.7, under the 1 it had,
while the fat place the counter is aimed at does not exist in a game of four computer seats. Put
to the designer with a measured alternative, the bounty never under 1; their answer, Q7 A.

[`sweeps/after-394.txt`](../sweeps/after-394.txt), **the ticket's sweep**, the bounty never under 1:
**7 / 5 / 1 / 7, collapses 60**; Colonies taken by Influence 6 / 5 / 7 / 21, stations 1 / 0 / 2 / 2,
places taken by force 21 / 39 / 5 / 0, 51 / 37 / 7 / 1, 17 / 44 / 16 / 1, 3 / 37 / 24 / 11: the
baseline to the Colony. The counter touches nothing a computer game contains and waits for the fat
place it is aimed at, which the witness proves it covets.
