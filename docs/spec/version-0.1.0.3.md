# Dying Earth — version 0.1.0.3, the fair-fight version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.3](https://github.com/whaleyjoshua2/Dying-Earth/issues/558), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-10-19-version-0.1.0.3/`](../dev-diary/2026-10-19-version-0.1.0.3/).

**The baseline** is 0.1.0.2's closing board,
[`final-0.1.0.2.txt`](../dev-diary/2026-10-19-version-0.1.0.2/sweeps/final-0.1.0.2.txt):
**3 / 23 / 26 / 13 and 14 collapses**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.2 with the designer's list after the 0.1.0.2 playtest
([`docs/dev-diary/2026-10-19-playtest-0.1.0.2/`](../dev-diary/2026-10-19-playtest-0.1.0.2/)), most
of it taking the slack out of the board: the loops that win alone, the chores that steal turns, and
the rules the computer never used.

## 1. One Bank to a Region

Ticket: [One Bank or Investment Bank to a Region](https://github.com/whaleyjoshua2/Dying-Earth/issues/560).

- **A Region holds one Bank**, by the Launch Site's door: standing, under construction or ordered
  this turn, and **the Prospectors' Investment Bank counts as one**, so a Region holds one of
  either kind and never both. A captured Investment Bank is the Region's one Bank, as a captured
  Academy is its School.
- **The refusal** is the pattern's: *"this Nation State already has a Bank"*, or *"an Investment
  Bank"*, naming the one there. The window greys the tile with the same words.
- **Any beyond one already standing** stay as they are: the rule is read at the order.
- The Prospectors' Opening Objective counts Regions with a working Investment Bank, so it stands.

## 2. A Scrubber at 4 Energy and 1 Ducat, one per 300 million

Ticket: [A Scrubber at 4 Energy and 1 Ducat, one per 3 population](https://github.com/whaleyjoshua2/Dying-Earth/issues/561).

- **A Scrubber draws 4 Energy and 1 Ducat a turn** while online (3 Energy and nothing before). The
  Ducat is charged at the Income beside the Energy, named in the income sources as the Energy is,
  and a mothballed Scrubber pays neither. The Ducat upkeep is a general field on the Facility card
  (`ducats_upkeep`), nought on every other building.
- **A Region holds one Scrubber per 300 million people**, rounded, one at least (200 million and
  two before): India 6, China 5 (1,370; 1,461 by turn 9), the Regions about 500 million 2, the
  rest 1. The ceiling of 10 stands, unreached.
- **The computer Custodians** offer a Scrubber only while last turn's Ducat income covers one more
  Scrubber's Ducat, beside the Energy check they already make.
- The Custodians' signature text, the card's standing line and the glossary say the Ducat.

**What it did** ([`sweeps/ticket-561.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-561.txt),
80 games): **9 / 24 / 20 / 11 and 15 collapses**, from 10 / 29 / 16 / 12 and 13 after §1. The
Prospectors' five is past noise; the rest within it. The computer built about as many Scrubbers
as before (493 against 464 in one seating): a Ducat a turn does not deter a seat whose Ducat income
is in the tens.

## 3. The Wind Farm

Ticket: [The Wind Farm: clean power on Earth, half on Mars, replacing a Power Plant at half cost](https://github.com/whaleyjoshua2/Dying-Earth/issues/559).

- **A new Facility, the Wind Farm**: 25 Materials, 8 Widgets, **3 Energy a turn, no emissions, 1
  Ducat a turn upkeep** (the Power Plant: 27.5 Materials, 8 Widgets, 6 Energy, 1.5 emissions, no
  upkeep). It takes a build slot as a Power Plant does and **needs Clean Power**. Its output takes
  the multipliers a Power Plant's takes (the lean, Solar Maximum, Efficient Grids, Superconductors).
- **A new Module, the Wind Farm, on Mars only**: base 2.5 Energy, half a Generator's 5, scaled by
  the site's Generator yield as every Module is (about 1.9 on Mars), no emissions, 1 Ducat a turn,
  25 Materials and 8 Widgets, needing Clean Power. Refused on every other Body: *"a Wind Farm stands
  on Mars alone"*.
- **Replace**: a new change on a standing Power Plant, *Replace with a Wind Farm*, at half the Wind
  Farm's price, 12.5 Materials and 4 Widgets, through the Region's queue as a build is. The Power
  Plant runs until the Wind Farm completes and is gone the Resolution it does; the slot never stands
  empty. A Power Plant only, never a Reactor; Clean Power needed.
- **The computer** builds a Wind Farm where it would build a Power Plant once Clean Power stands,
  **unless its Energy is tight**, when it prefers the Power Plant's 6; and it replaces a Power Plant
  with a Wind Farm when it is not short of Energy and its Blame weighs on it.
- Mothball, Restart and Decommission apply to a Wind Farm as to any Facility. Production Moved does
  not pair it (the Power Plant pairs with the Generator; the Wind Farm with nothing).

## 4. Power burns Fuel

Ticket: [Power Plants, Reactors and their off-world counterparts burn 0.75 Fuel a turn](https://github.com/whaleyjoshua2/Dying-Earth/issues/575).

- **A Power Plant, a Reactor and a Generator burn 0.75 Fuel a turn** while online, charged at the
  Income beside the Energy and the Ducat and named in the income sources; mothballed, they burn
  nothing. The Solar Array, the Heliostat and the Wind Farm burn nothing.
- **Short of Fuel**, the bill is paid building by building as far as the stockpile goes, in the
  Income's order; **a building whose Fuel is not there runs at a quarter** that Income, and the
  Report says so. Nothing goes dark for want of Fuel.
- The Fuel upkeep is a general field on both the Facility and the Module card (`fuel_upkeep`),
  nought on every other building.
- The computer's Refinery weight reads the drain, so a seat burning Fuel for power refines more.
