# Ticket #394: the computer's appetite for a rival's Colony scales with its size

The designer's item: *"the AI now scales its desire for a station/colony on its size."* The first
round read it two ways (the Faction's size or the place's) and the designer set both aside with the
intent: *"a counter check to founding one colony or station and just loading people and building on
it ... I'd like the AIs to want to take possession of a colony with high population. Let me know if
this mechanic is already in play."* It was not (the ticket's resolution says what was), and the
second round decided it (*"q1 a q2 a q3 a q4 colonists + output"*). The first build's sweeps sent
two shapes back to the designer, who reshaped the prize (*"q5 a q6 a but make it 26 and give it a
factory too"*): rescaled to the board, the fattest outpost the 3.
[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/394) is the authority,
[§14 of the spec](../../../spec/version-0.09.3.md#14-the-computers-appetite-for-a-rivals-colony-scales-with-its-size)
records it.

## What was built

`Game::ai_place_size(colony)`: Colonists plus the place's output a turn (ticket #391's
`place_output`, Energy only where positive). `Game::ai_prize(colony)`: `prize_top` x size / the
largest size on the board, never dividing by less than `prize_floor`, the designer's shape. The
Influence targets, extracted into `ai_influence_targets` so their order can be witnessed, weigh a
rival's Colony by its price rank times its prize; the Army landing at a rival's Colony takes the
prize on its base weight. Two figures in `ai.toml`: `prize_top` 3.0 (the old price rank's
ceiling) and `prize_floor` 26.0 (the designer's: a Colony of eight with a Mine, a Generator and a
Factory).

## The two shapes set aside

The first build had the prize as 1 + 0.05 x size, capped at 3, multiplying the price rank. Its
witness stayed red: a Colony of eight people and three Modules (price 200, prize 2.25) weighed 0.9
against a lean one's 1.3 (price 80, prize 1.3), since the price term falls 2.5x faster than that
prize rises. Its sweep, since overwritten, read 6 / 7 / 0 / 5, collapses 62, Colonies taken by
Influence 11 / 12 / 17 / 28 by seating (6 / 5 / 7 / 22 before). The second had the prize alone,
the price no longer ranking: the witness went green and the sweep,
[`sweeps/after-394-prize-alone.txt`](../sweeps/after-394-prize-alone.txt), read **2 / 1 / 0 / 8,
collapses 69**, Colonies taken by Influence 0 / 0 / 3 / 2: the seats spent their Allotment on
places priced 200 and more while their Regions were taken from them. Both went to the designer,
whose answer was the rescaling: linear in the size, with no "1 +", so a fat place wins the rank
while the price still counts.

## The red witness

`a_computer_seat_wants_a_rivals_fat_colony_more_than_its_lean_one`: two Colonies of a rival's on
Mars, one of two Colonists and nothing built, one of eight with a Mine, a Generator, a Factory and
a Habitat; the fat one's price by Influence is higher, and it outranks the lean one among the
seat's Influence targets all the same; and once a place reaches the floor it is the board's top,
a fatter one measured at 3 and the fat one under it. Red with `prize_top` at 0, where every Colony
weighs nothing and none outranks another; green at 3.

## The sweep

[`sweeps/after-394.txt`](../sweeps/after-394.txt), the rescaled prize as decided, against the
Trade Post's [`sweeps/after-397.txt`](../sweeps/after-397.txt): **8 / 3 / 1 / 7, collapses 61**
(6 / 5 / 1 / 8, 60 before). Colonies taken by Influence, by seating: **1 / 0 / 5 / 5, against 6 / 5
/ 7 / 22 before**; stations 0 / 1 / 0 / 2 (1 / 0 / 2 / 2); places taken by force 23 / 40 / 5 / 0,
51 / 37 / 7 / 1, 16 / 46 / 16 / 1, 4 / 39 / 24 / 15 (21 / 39 / 5 / 0, 51 / 37 / 6 / 1, 17 / 44 /
13 / 2, 3 / 37 / 23 / 11 before). **The computer takes fewer Colonies than before**, and the reason
is in the shape: the prize is linear and measured against the board's fattest outpost, so a lean
Colony (a computer's usual, size six or so against a floor of 26) is a prize of about 0.7, under
the 1 it weighed before, while the fat place the counter is aimed at is the 3. In a sweep of
computer seats there is no fat turtle to covet, so the lean places lose weight and nothing gains
it. The witness proves the ordering the designer asked for; the sweep cannot see the player it is
aimed at. Reported to the designer, with a measured alternative: the same prize never under 1
(`prize_least`), so the lean places keep the weight they had and only the fat ones rise.

[`sweeps/after-394-least-one.txt`](../sweeps/after-394-least-one.txt), the same prize never under 1
(`prize_least` 1): **7 / 5 / 1 / 7, collapses 60**; Colonies taken by Influence 6 / 5 / 7 / 21,
stations 1 / 0 / 2 / 2, places taken by force 21 / 39 / 5 / 0, 51 / 37 / 7 / 1, 17 / 44 / 16 / 1,
3 / 37 / 24 / 11: the baseline to the Colony. With the least at 1 the prize touches nothing a
computer game contains and waits for the fat place it is aimed at. Which of the two the designer
wants is theirs to say; the tree carries the figure at 0, as decided, until they do.
