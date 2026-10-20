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

**What it did** ([`sweeps/ticket-559.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-559.txt),
80 games): **10 / 24 / 20 / 17 and 9 collapses**, from 9 / 24 / 20 / 11 and 15 after §2. The
Archivists' six and the collapses' six are past noise, both from cleaner power across the board.

## 4. Power burns Fuel

Ticket: [Power Plants, Reactors and their off-world counterparts burn 0.8 Fuel a turn](https://github.com/whaleyjoshua2/Dying-Earth/issues/575).

- **A Power Plant, a Reactor and a Generator burn 0.8 Fuel a turn** (the designer's 0.75, moved to
  the tenth the stockpile keeps, at the designer's word) while online, charged at the
  Income beside the Energy and the Ducat and named in the income sources; mothballed, they burn
  nothing. The Solar Array, the Heliostat and the Wind Farm burn nothing.
- **Short of Fuel**, the bill is paid building by building as far as the stockpile goes, in the
  Income's order; **a building whose Fuel is not there runs at a quarter** that Income, and the
  Report says so. Nothing goes dark for want of Fuel.
- The Fuel upkeep is a general field on both the Facility and the Module card (`fuel_upkeep`),
  nought on every other building.
- **The computer keeps a Fuel reserve for its power**, three turns of what its buildings burn
  (`power_fuel_reserve_turns`), at the designer's word after the first sweep: under it Fuel is its
  scarcest resource, a Ship is refuelled only from what stands above it, a Ship's top-up buys the
  reserve back, the market sells nothing out of it and buys up to it when its Ducats allow.
- The Energy shortfall forecast reads the Fuel this turn's orders leave, as the Income will.

**What it did** ([`sweeps/ticket-575.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-575.txt),
80 games): **8 / 21 / 17 / 10 and 24 collapses**, from 10 / 24 / 20 / 17 and 9 after §3. The
collapses' fifteen and the Archivists' seven are past noise. The collapses gather in the seatings
where the computer plays the Custodians: their Scrubbers fell (421 to 312 in one seating) and the
Natural Sink with them (19.1 to 12.6). A reading, not a measured cause: a computer seat short of
Fuel runs its power at a quarter, reads its Energy as tight, and a tight seat is offered no
Scrubber. Reported to the designer; nothing changed on the strength of it.

**With the reserve** ([`sweeps/ticket-575-reserve.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-575-reserve.txt)):
**10 / 25 / 20 / 10 and 15 collapses**, from 8 / 21 / 17 / 10 and 24 without it. Against the board
before the Fuel rule (10 / 24 / 20 / 17 and 9), the collapses' six are near noise's edge and the
Archivists' seven past it; the rest are back where they were.

## 5. Influence at 4.5 Ducats a point; Distress Call at +2

Ticket: [Influence at 4.5 Ducats a point, and Distress Call at +2 Relations](https://github.com/whaleyjoshua2/Dying-Earth/issues/562).

- **A point of Influence costs 4.5 Ducats** at the Trading window (4 since 0.1.0.1, 2 before; 5
  for one build of this version, cut to 4.5 on its sweep). Still no cap; the price may be a
  fraction, so eleven points cost 49.5. The computer reads the price from the table.
- **Distress Call** pays +2 Relations with every rival for holding a Ship a turn (+5 before);
  refusing still costs 3.

**What it did** ([`sweeps/ticket-562.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-562.txt),
80 games, the reserve's second fix included): **5 / 36 / 13 / 10 and 16 collapses**, from 10 / 25 /
20 / 10 and 15. The Prospectors' eleven, the Arkwrights' seven and the Custodians' five are all
past noise, and all one way: the price of a point hurts the seats poor in Ducats and leaves the
richest, the Prospectors, with the only Allotment that still buys. Reported to the designer, who
set the price at 4.5.

**At 4.5** ([`sweeps/ticket-562-45.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-562-45.txt),
80 games): **8 / 29 / 17 / 14 and 12 collapses**. The Prospectors give back seven of the eleven,
the Arkwrights take back four, the Custodians three; the Archivists' fourteen is the version's
best. Beside the figures before the ticket (10 / 25 / 20 / 10 and 15), only the Prospectors' +4 and
the Archivists' +4 stand at the edge of noise.

## 6. Production Moved makes x2.08; the computer weighs the trade whole

Ticket: [The computer Custodians use Production Moved](https://github.com/whaleyjoshua2/Dying-Earth/issues/563).

The ticket's premise, corrected: the computer Custodians did use Production Moved from three of
the four starts (20 to 30 Module-turns a game); from East Asia, never, since its Materials lean
makes its Mine 6.6 and no Module off Earth reached that. The designer: "rather than just doubling
it let's give them a small boost to entice them ... say 208% but also add some decision calculus on
the value of lower emissions to their victory condition."

- **Production Moved makes x2.08** (double before), on the Module's final figure, to the tenth;
  Widgets stay whole, floored (4 x 2.08 = 8); Research whole, to the nearest (3 x 2.08 = 6). The
  figure is `production_moved_factor` on the Custodians' card; the card line reads "x2.08 for an
  idle Mine on Earth", the Income source likewise, and the signature reads the figure.
- **The computer's trade.** A Facility is idled when the Module's lift plus the Emissions' worth
  reaches the Facility's output: `M x (2.08 - 1) + E x k x g >= F`, with M the best undoubled
  Module's output, F the Facility's, E the Facility's Emissions as the Custodians emit them (their
  x0.75: Mine and Factory 0.56, Power Plant and Refinery 1.12), k = `production_moved_emission_worth`
  = 1.0 (`ai.toml`), and g the computer's
  Stabilization gap multiplier (1 on pace, up to 3 as net Emissions stand over the Sink). Several
  qualifying, each candidate's weight is scaled by its Emissions per output, so the dirtiest is
  taken first. The build-side boost reads the same factor.
- **From East Asia**: a plain Moon Mine (5.0) lifts 5.4; with the Mine's 0.56 a point it reaches
  6.6 at g >= 2.1, the run well behind. Beside a Mass Driver (6.0) it lifts 6.48 and the 0.56
  carries it to 7.04 on pace, where the old doubling's 6.56 fell short. The Power Plant (6) against
  a Moon Generator (6.5): 7.02 + 1.12, taken.
- **The measure**: the sweep's Production Moved line above nought in the East Asia seating too, and
  no fall in the other three.

**What it did** ([`sweeps/ticket-563.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-563.txt),
80 games): **17 / 21 / 20 / 11 and 11 collapses**, from 8 / 29 / 17 / 14 and 12. Production Moved by
seating 0 / 34 / 31 / 36 Module-turns a game, from 0 / 21 / 27 / 30. East Asia stays at nought: the
computer Custodians starting there build nothing off Earth through turn 30 and lose the Region by
then in most seeds, so the rule has nothing to double -- a matter of their opening, not of this
trade. The sweep also prints each Faction's Income over a game (Materials / Fuel / Energy / Ducats /
Research, mean over the batch), at the designer's word.

## 7. Every computer seat races for the first landing

Ticket: [Every computer seat pursues the first Moon landing](https://github.com/whaleyjoshua2/Dying-Earth/issues/564).

The playtest's human was first to the Moon in sixteen games of sixteen, by turns 6 to 10; the
computer's first landing came at median turn 15 / 26 / 32 by seating, and the Arkwrights took the
Moon in 37 of the 50 games anyone did. The path is a Shipyard (38.5 Materials, 8 Widgets) then a
Colony Ship (33 Materials, 4 Widgets), at weights that compete with every build on Earth.

- **The race weight**: while the seat holds no Colony Ship, built or on order, and **the Moon** is
  still unclaimed with a free slot (`first_at` is none), its Shipyard and Colony Ship candidates are
  weighed at `moon_race_weight` = **3** times their base (`ai.toml`), above the Prospectors' Bank
  (9) and the Custodians' Scrubber (8). The lift ends with the seat's first Colony Ship or the first
  landing, and returns for no other Body.
- **The first Ship flies to the Moon**: while the Moon is unclaimed and the seat directs no ground
  Colony off Earth, its Colony Ship's destination is the Moon, whatever the windfalls say
  (`best_body_for`). The computer's rule only; the windfalls are untouched. Tried first and
  withdrawn: a race for any unclaimed Body -- the Ships came early and the windfalls sent them past
  the Moon, the Prospectors to Mars by turn 11, and they took 53 of 80 games (47 at a weight of 2);
  `sweeps/ticket-564.txt` and `ticket-564-x2.txt` keep the figures.
- **Every seat** races; all four count Colonists off Earth.
- **The measure**: the sweep's "First to a Body: the Moon" median turn 12 or under in every seating,
  who wins it spread beyond the Arkwrights, and no fall in the four Factions' wins. The Income line
  per Faction is read beside it, at the designer's word.

**What it did** ([`sweeps/ticket-564-moon.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-564-moon.txt),
80 games): **14 / 56 / 4 / 5 and 1 collapse**, from 17 / 21 / 20 / 11 and 11. The Moon landed at
median turn 9 / 7 / 8 / 8 by seating, every game: the bar met. The Prospectors win the race 16 to
19 times in 20 and 56 of the 80 games; the designer closed the ticket on the measure and ticketed
their Venture Fund bar at 4250.

## 8. The Upload at 16, four a turn; an Archive that draws more as it fills; Modules dark without people

Ticket: [The Upload at 16, four a turn, an Archive that eats more as it fills, and Modules dark without people](https://github.com/whaleyjoshua2/Dying-Earth/issues/565).

The playtest ended Archivist games in one order (`upload 27 18`). The designer: *"Archive upload now
only requires 16 but capped at 4 per turn, energy upkeep grows by 1 per uploaded colonists, if any
modules at the archive's site lack the population to run it they go offline."*

- **The bar is 16** (18 since 0.1.0.0), and **at most four Colonists upload in a turn**, pending
  Uploads counted (`[archive] upload_per_turn = 4`): "four a turn; 2 already ordered". The room
  check comes first ("only 3 Colonists are left to upload"). The computer Archivists upload four a
  turn and accept what goes dark; their pace table is 5 / 11 / 16 by turns 36 / 42 / 47.
- **The Archive's draw is 20 Energy + 1 per uploaded Colonist** (`energy_per_uploaded = 1`), once
  complete as before, Closed-Loop Colonies' x0.75 on the whole; the card's detail reads "20 + 7
  uploaded".
- **Modules dark without people**, every Colony, one rule read off the slot formula backwards: a
  Colony's slots are one per living Colonist (base 0) up to its tier's cap, and the Core and the
  Archive stand outside the count. Walk the Modules in build order, each slot-taking Module taking a
  slot while slots remain; a Module past the count is **offline, "too few people to run it"**, and
  comes back as people return. **The Habitat is never darkened**: it houses the people who run the
  rest, and a dark Habitat would crowd the place on top of the uploads. A dark Module keeps its
  slot, pays no upkeep and makes nothing, as the Energy shortfall's shut Module does. So an Archive
  site uploaded from 16 to nought keeps its Core, its Archive and its housing, and everything else
  goes dark.
- **The measure**: the sweep's Archivists (5 of 80 at ticket #564) and their gate's median turn.

**What it did** ([`sweeps/ticket-565.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-565.txt),
80 games): **12 / 58 / 6 / 2 and 1 collapse**, from 14 / 56 / 4 / 5 and 1 -- inside noise but for
the Archivists' direction, five to two, four turns of uploading now standing between a complete
Archive (median turn 34 to 35) and the win. Their gate's median turn is 28, as before.

## 9. The Venture Fund's growth, shown whole

Ticket: [The Venture Fund grows past its banked line: explained and shown](https://github.com/whaleyjoshua2/Dying-Earth/issues/566).

Three testers read a Fund up 566 beside "90 banked last turn" as a bug. It is not: the Fund takes the
banked share of Ducat income, the Investment Banks' and Exchanges' interest (a hundredth of the
balance per payer, floor 1, compounding) and the Objectives' one-off rewards, and the line read the
share alone.

- **The interest is kept beside the share** (`venture_interest_last_turn`), and the Fund line says
  the growth whole: **"Fund 3742; +312 last turn (100 banked, 212 interest)."** in the window and
  the tool. An Objective's reward, paid at the end of an Income, is named on that turn alone:
  "(100 banked, 212 interest, 50 reward)" (the review).
- **The rate stands** (designer: "leave it"): one Bank a Region, the dearer Influence and the Fund
  bar at 4,250 (ticketed) are this version's brakes; the last sweep's Fund at the end was median
  3,742 / 8,419 / 3,582 / 3,597 by seating.
- No rule moves, so no sweep.

## 10. The Venture Fund and the Capital weighed into the percentage

Ticket: [The Venture Fund and the missing Capital weighed into the Victory percentage](https://github.com/whaleyjoshua2/Dying-Earth/issues/567).

The playtest: *"Your Victory: 59%"* with the Fund at 17% and the Presence full; *"100%"* while a
missing Colonial Capital held the win. The score is the weighted mean of the two parts' fractions
(ticket #555); the Capital and the gate Tech were holds `met()` read and `score()` never did.

- **The Prospectors' Fund counts 0.7** of their score, the Presence 0.3 (`weight = 0.7` on the card):
  the Fund is the long pole, 3,500 against 18 Colonists met by turn 12. The playtest's board reads 42%.
- **The Arkwrights' Capital is a part of its own**, `capital_weight = 0.1` on the card: the score is
  0.6 x Colonists off Earth + 0.3 x Bodies + 0.1 x Capital (held or not), so both bars met without one
  reads 90%. Every computer seat reads it as it reads the score: the leader for gang pressure, the
  three-quarters Moment, the Accord door at 95% and the last-turn ranking. The data refuses a
  `capital_weight` on a card that asks no Capital, or weights summing past one.
- **A headline with both parts full and a hold standing names the hold** in a few words after the
  percentage -- **"100% - needs Generation Ships"**, **"90% - needs a Colonial Capital"**, "100% - the
  Archive is not running" -- in the Victory window's row, the Report's head, the game-over page and
  the ranking. Short of the bars the percentage stands alone; the row's first line names the hold
  beneath as before, and the text tool already printed it. The parts are read as the percentage
  reads them (the review): the Custodians' best run fills their first part short of the bar (§406),
  and that 100% names its hold too.
- **The review** also noted what the Capital's tenth does to the computer seats' doors: a Trade or
  an Accord is refused to a seat at 95% or more of its Victory as it stands, and an Arkwright seat
  with both bars met and no Capital read 100% and now reads 90%, so rivals deal with it where they
  refused before, the win one upgrade away. A loosening, left standing for the designer's eye. The
  ranking page names a hold twice, the heading's few words over the chronicle's sentence.
- **Swept** (`sweeps/ticket-567.txt`, 20 seeds x four seatings): **9 / 60 / 5 / 6 and no collapse**,
  from 12 / 58 / 6 / 2 and 1 -- noise. Three games of 80 went to the last-turn ranking (one to the
  Prospectors, two to the Arkwrights), the rest met outright. The computer Arkwrights held a Capital
  at the end in 4 games of 80, so the tenth is off their score in nearly every game; the Fund at the
  end by seating 3,780 / 8,025 / 3,578 / 3,585; Income per Faction as the sweep file.
