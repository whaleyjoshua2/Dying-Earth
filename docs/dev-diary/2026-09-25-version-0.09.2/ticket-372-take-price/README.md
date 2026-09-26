# Ticket #372: the Influence figure on a card reads the holder

The designer: *"update influence thresholds on cards to reflect current holders influence."* Decided
on the ticket in one round: on a held place the row shows the price to take it; the player's own
price, the challenger line keeping each rival's; every hover uses the real margin.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/372) is the authority;
[§7 of the spec](../../../spec/version-0.09.2.md#7-the-influence-figure-on-a-card-reads-the-holder)
records it.

## What was built

`src/ui.rs` alone. Two helpers, `rival_take_price` (the lowest price any rival pays, each by the
rule) and `margin_words` (the margin from `challenge_margin_for`, with its parts named), and the
standings row's figure in three states: *"Take at N"* on a rival's place, *"A rival takes it at N"*
on the player's own, *"Threshold T"* on nobody's. The chip hovers and the row's explain hover read
the same helpers. No engine change.

## The pictures

Three Region cards, taken headlessly at `window:1400x900`, the Climate Panel closed.

![The European Union, the Prospectors': "Standings: Prospectors 29 · Take at 49"](rival-earth.png)

`shot:rival select:europe turns:11 panel:0`. **A rival's place, turn 12.** The row reads *"Prospectors
29 · Take at 49"*, and the breakdown beneath agrees with it line by line: *"Threshold 41 - 20 + 20 for
size 2, x1.03 (Blame)"*, *"Held by the Prospectors: 49 - 29 + 20 (margin)"*. Before this the row read
*"Threshold 41"* over the same breakdown.

![China, the player's own: "Standings: Custodians 38 · A rival takes it at 58"](own-earth.png)

`shot:own select:eastasia turns:11 panel:0`. **The player's own place.** *"Custodians 38 · A rival
takes it at 58"*, the lowest price any rival pays; *"Held by you: 58 - 38 + 20 (margin)"* under it.

![Nigeria, nobody's: "Standings: nobody has any yet · Threshold 51"](nobody-earth.png)

`shot:nobody select:subsaharanafrica panel:0`. **Nobody's place, turn 1.** *"Threshold 51"*, as before.

The first cut of the rival picture was under a card the staged turn drew, and the neutral one opened
no card because the aid takes a Region's id, not its name; both re-taken.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 492 + 6, root 8.
No sweep: nothing in the engine moved.
