# An offline building shown offline in its box

Ticket [#307](https://github.com/whaleyjoshua2/Dying-Earth/issues/307) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`china-facility-offline.png`](china-facility-offline.png) | `shot: select:eastasia offline:1 panel:0`. China's card with its Factory offline: the box dimmed to the mothballed tile's darkness, the picture at four tenths, and the word **offline** in its bottom-left corner in a warm colour, beside three working boxes drawn bright. |
| [`china-offline-hover.png`](china-offline-hover.png) | `shot: select:eastasia offline:1 panel:0 tip:(offline,`. The same box's hover: *Factory (coastal): ... (offline, short of Energy; making nothing)* above the Energy rule. |
| [`iss-habitat-offline.png`](iss-habitat-offline.png) | `shot: hab:1 morder:habitat commit:1 offline:1 panel:0`. The ISS's card with its Habitat offline: the tile dimmed and worded the same way, the Core Module bright beside it. |
| [`iss-offline-hover.png`](iss-offline-hover.png) | `shot: hab:1 morder:habitat commit:1 offline:1 panel:0 tip:(offline,`. The Habitat tile's hover: *Habitat: holds 4 Colonists ... (offline, short of Energy; making nothing)* above the Module rules. |

No batch was run: an interface change; the engine is untouched. The four pictures show the Energy
cause; a card, a grid failure and an Occupied Archive take the same tile and their own words on the
hover, and were not photographed, since no shot aid produces them.

## What was decided, in the designer's words

*"q1 as recommended q2 as recommended q3 yes q4 leave q5 none"*: every building whose online flag
is false and that is not mothballed; the mothballed dimming with the word **offline** in a warmer
colour; the cause on the hover; a blockaded Colony's tiles left as they are; no change to the
roster or the row.

## Settled by the builder, to be corrected if wrong

- **The word's colour** is a warm tan (225, 165, 115) against mothballed's cool grey (170, 170,
  190), so the two states read apart without a second shape.
- **The hover's words** are *(offline, short of Energy; making nothing)*, *(offline until the next
  Resolution, struck by a card; making nothing)*, *(offline, the grid is down; making nothing)* and
  *(offline while the Colony is Occupied)*. The card's name is not stored on the building, so the
  card case says only that a card did it.
- **The `offline:1` shot aid** puts seat 0's first start-state Facility and the first Module beyond
  the Core on seat 0's first station offline, as a shortfall would; a building aid, not a rule.
- A new **Offline** glossary entry names the four causes and the two look-alikes.

## Looked at, not tested

The four pictures above, each opened and read before this was committed.
