# The off-Earth Events join the deck on turn 12

Ticket [#259](https://github.com/whaleyjoshua2/Dying-Earth/issues/259) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`climate-panel-turn-1.png`](climate-panel-turn-1.png) | The Climate Panel on turn 1: *"a card comes 50% of turns at this Temperature (**28 cards left in the deck**, 12 of them Climate)"*. Forty since ticket #76; the twelve that can only land off Earth are not dealt yet. |

## What was decided, in the designer's words

Offered a stacked deck, a card that returns when it finds nothing, or both: *"those seven flag them
as off earth and add them to deck on turn 12"*. Neither of the offered shapes. The seven cards --
Grid Failure, Reactor Leak, Dust Storm, Moonquake, Helium-3 Vein, Rich Seam, Ice Deposit; twelve
with their copies -- carry `off_earth = true` in `events.toml`, are **not dealt at the start**, and
on **turn 12** (`off_earth_join_turn`) are shuffled into whatever remains of the deck, once. The
Report says so on the turn it happens. The questions about where a stacked deck's bottom starts and
what the Report says when a card returns *"don't apply under this scheme"*, and they do not: a card
drawn with nowhere to land is still spent, as it always was.

## Measured, before and after

Five headless games (`sim 1..5 --log`), counting cards drawn against cards that said *"so nothing
happens"*:

| | drawn a game | found nowhere to land | of which |
|---|---|---|---|
| before | 28.8 | **4.2** | Rich Seam 7, Reactor Leak 3, Ice Deposit 3, Dust Storm 3, Storm Surge 2, Breakthrough 2, Helium-3 Vein 1 |
| after | 27.6 | **2.4** | Dust Storm 4, Rich Seam 2, Reactor Leak 2, Ice Deposit 2, Moonquake 1, Helium-3 Vein 1 |

The waste falls by nearly half. What is left is mostly **Dust Storm**, which wants somebody living
on Mars -- a Colony founded there in 4 of 20 seeds at a median turn of 31 -- so turn 12 is early for
it; and the odd Rich Seam or Reactor Leak on a game slow to leave Earth. Those are the cases the
returning card would have covered, and the designer chose not to.

**The same seeds are not the same games any more.** The deck's shuffle comes off the seeded dice,
and a deck of 28 shuffled at the start and 40 reshuffled on turn 12 draws differently from a deck
of 40 shuffled once; every later roll moves with it. The closing sweep's seed-for-seed comparison
with 0.08.3 carries that noise, and nothing can be done about it short of a second RNG for the
deck, which nobody asked for.

## Witnessed red

The deck test was rewritten first -- 28 at the start, the twelve flagged and absent, 40 after turn
12's Event phase, once -- and run against the old rule: `left: 40, right: 28`. Then `new_deck`
learned to skip the flagged cards and `join_off_earth_cards` to shuffle them in; `316 passed`,
clippy clean with `-D warnings`. A save from before this version has never dealt them and joins
them on its next Event phase past turn 12 (`#[serde(default)]` on `off_earth_joined`).
