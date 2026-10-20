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
  two before): China 6 (5 at the start's 1,461), India 5, the Regions about 500 million 2, the
  rest 1. The ceiling of 10 stands, unreached.
- **The computer Custodians** offer a Scrubber only while last turn's Ducat income covers one more
  Scrubber's Ducat, beside the Energy check they already make.
- The Custodians' signature text, the card's standing line and the glossary say the Ducat.

**What it did** ([`sweeps/ticket-561.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-561.txt),
80 games): **9 / 24 / 20 / 11 and 15 collapses**, from 10 / 29 / 16 / 12 and 13 after §1. The
Prospectors' five is past noise; the rest within it. The computer built about as many Scrubbers
as before (493 against 464 in one seating): a Ducat a turn does not deter a seat whose Ducat income
is in the tens.
