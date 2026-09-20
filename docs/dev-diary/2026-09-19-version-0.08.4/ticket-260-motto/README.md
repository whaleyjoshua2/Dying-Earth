# A motto for every Faction

Ticket [#260](https://github.com/whaleyjoshua2/Dying-Earth/issues/260) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

| picture | what it shows |
|---|---|
| [`selection-screen-1080.png`](selection-screen-1080.png) | `shot: menus:1 window:1920x1080`, the size the game opens at. All four cards, each with its motto directly under the name, italic, in the Faction's colour, above the blurb. |
| [`selection-screen-800.png`](selection-screen-800.png) | The same at 1280x800, where the screen scrolls (ticket #217); the top two cards whole and the bottom two's heads, mottos visible on all four. |

## The four, as the designer chose them

| Faction | motto |
|---|---|
| Custodians | *Leave it better than we found it.* |
| Prospectors | *Everything has a price. We find it.* |
| Arkwrights | *Nothing left behind but the Earth.* |
| Archivists | *Everyone remembered.* |

Eight were proposed, two a Faction; the designer took the first for the Custodians and the
Prospectors and the second for the Arkwrights and the Archivists -- *"go with proposed for
custodians and prospectors the other two the or"*. The Arkwrights' rejected line was a
Tsiolkovsky paraphrase that would have wanted a credit; the chosen one wants none.

## Where, and where not

*"yes that exactly"* -- under the name, italic, in the Faction's colour, before the blurb. *"card/rule
book only"* -- the selection card and the in-game Faction window's rulebook are one function
(`faction_rulebook`, ticket #203), so the window's Rulebook header carries it for nothing and it
appears nowhere else: not the window's heading, not a Ship's hover, not the Report.

## The guard

A motto moves no mechanic, so no existing test would notice one going missing or two cards sharing
one. `every_faction_card_carries_a_motto_of_its_own` was written first and run with the field in
place and the data absent -- *"Custodians has no motto"* -- then the four lines went into
`factions.toml` and it passed. It also holds each to 48 characters and apart from its blurb.
