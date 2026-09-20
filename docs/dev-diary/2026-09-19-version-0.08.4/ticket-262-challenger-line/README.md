# The challenger line, and the warning it replaced

Ticket [#262](https://github.com/whaleyjoshua2/Dying-Earth/issues/262) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

All three pictures are China's card, held by the player; `challenger:1` is a new shot aid that
stands seat 1 on the player's start state 23 short of its price -- the designer's own example.

| picture | what it shows |
|---|---|
| [`before-two-lines.png`](before-two-lines.png) | **The first build, and the surprise.** The new line in the Standings block -- *The Prospectors stand at 47; they take this at 70.* -- and, above the Influence heading, **a line that was already there**: *"The Prospectors stand at 47 here against your 50: they take it at 70. Spend here to stay ahead."* Ticket #75 (version 0.05.5) put it at the top of the card, and the charting round missed it. |
| [`china-card-challenger.png`](china-card-challenger.png) | **After.** One line, in the Standings block, amber because the rival is within two steps: *The Prospectors stand at 47; they take this at 70. Spend here to stay ahead.* The old line is gone. |
| [`no-rival-standing.png`](no-rival-standing.png) | The same block with nobody standing: *No rival has a Standing here.* |

## What was decided, in the designer's words

*"yup"* -- the rival **nearest its own price**, not the highest Standing. *"yes"* -- the line says
*No rival has a Standing here* rather than vanishing. *"yup"* -- the two figures on the line, the
arithmetic on the hover. *"akk three"* -- Regions, Colonies and stations. *"held only"*. *"looks
fine"* -- the sentence kept as written.

## Why the old line went

Ticket #75's warning picked **the rival with the highest Standing** and priced the place at **the
holder's Standing plus the challenge margin** -- which is not the rule. Since ticket #60 the one
price is `influence_needed_for`: the greater of the challenger's own threshold, with its Blame
multiplier inside, and the holder's Standing plus the margin *that pair* faces, Relations and
Constabulary included. Where Blame or Relations move a rival's price, the old line was wrong, and
it could name the wrong rival. Two lines saying almost the same thing, one of them inaccurate, is
the thing this version's map warns against; so the old line's two virtues -- its amber, and *Spend
here to stay ahead* when the rival is within two steps of the holder -- were folded into the new
one, and ticket #161's decay sentence into the hover. One line, one arithmetic, on every held card.

**Settled by the builder, say if wrong:** that the line lives in the Standings block rather than
at the top of the card where #75 put its warning. The block is above the fold at 1280x800.

## Witnessed red

`nearest_challenger` was written first in the naive form -- highest Standing -- and the test that
wants the nearest-to-its-price rival run against it: a rival at 35 with a price of 60 was named
over one at 32 with a price of 40. `left: (Seat(1), 35, 60), right: (Seat(2), 32, 40)`. Then the
one line changed to `min_by_key(price - standing)` and it passed. `319 passed`, clippy clean.
