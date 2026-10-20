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

## 11. The Prospectors' Venture Fund at 4,250

Ticket: [The Prospectors' Venture Fund at 4250](https://github.com/whaleyjoshua2/Dying-Earth/issues/580).

The designer, closing the race for the Moon (§7): *"add a ticket to increase their victory fund to
4250"*. The race put the Prospectors first to the Moon in 16 to 19 games of 20 and at 56 wins of 80,
60 in the last sweep; their gate's median turn moved only 27 to 25.

- **The bar is 4,250** (`victory_first.bar`, 3,500 x 17/14). The card's long text reads the bar
  (`{first}`, `{second}`) as the short text does, through `Tables::victory_text`, and the data
  refuses a placeholder the card cannot fill; the other three cards' long texts still type their
  figures and read as written.
- **The computer Prospectors' pace** scales with it: 567 / 1,417 / 2,550 / 4,250 at turns 12 / 25 /
  37 / 47. The waypoints keep their turns; the pace asks a fifth more Fund at every turn, so a seat
  with the same Fund reads behind earlier and banks harder (the review).
- **The review** found the headless shot aid `rival:1` still setting the Fund at 2625, which at the
  new bar scored 0.67 and fired no Moment, so the Report's rival-Moment picture was dead: 3230 now.
- Nine older pins moved with the bar (margins at 5,100, the Moment's "3230 of 4250", the turn-one line).
- **Swept** (`sweeps/ticket-580.txt`, 20 seeds x four seatings): **9 / 59 / 5 / 7 and no collapse**,
  from 9 / 60 / 5 / 6 -- the bar did not bite. The Prospectors' gate median turn 25 as before; games
  still running at turn 39, 40 of 80 from 37; their Fund at the end by seating 4,495 / 8,113 / 3,753
  / 4,352, from 3,780 / 8,025 / 3,578 / 3,585 -- up by about the bar's fifth, since the pace table
  scaled with it and the computer Prospectors bank to their pace. Income per Faction as the sweep file.

## 12. An Influence spend that would break an Accord is refused unless it says break

Ticket: [An Influence spend that would break an Accord warns before the order stands](https://github.com/whaleyjoshua2/Dying-Earth/issues/569).

The playtest: all four testers broke an Accord by accident, seven times, through an Influence order
that went through as "ok ... breaks your Accord with X". The engine warned in the price alone; the
window's Spend and Max buttons carried fixed hovers; the breach fell on whoever held the place as the
spend settled. Decided in one round ("that all looks good"):

- **The order says break** (`Order::Influence { break_accord }`): a spend on a non-aggression
  partner's place is refused without it -- *"breaks your Accord with X: order it with break to break
  it"* -- and placed with it, the Accord breaking at the Resolution as before. The tool: `influence
  southasia 17 break`. The computer seats never set it; they drop such orders themselves (#520).
- **The window's three Influence buttons** (Spend, Max, the place card's Spend) share one helper:
  where the spend would cross, a red *"breaks your Accord with X"* stands beside the button and in
  its hover; the first click arms it and it reads **"Break the Accord with X and spend 17"** in red;
  the second places the order with the break meant. A spend that crosses nobody is placed at once.
- **Influence alone** is refused; Launch, Agitate, Army move, Smear and Bombard warn in the price as
  before, since nobody gives those by mistake.
- **The offence falls on the aim**: the pending spend carries the place's director as the order was
  given, and the Resolution offends that seat -- a rival's place that passed to a partner offends the
  rival and the Accord stands; a neutral place a partner took in the same Resolution offends nobody
  (the playtest's T13).
- A standing Max already ends by itself with a Report line when its place is no longer yours (#134),
  and spends on your own place alone, so it cannot cross.
- **The review**: the arming is keyed on the place and the amount, and any click elsewhere or End
  Turn disarms it (as the stack's, #323), so the second click confirms what the first proposed; the
  buttons' hovers go through `rule_tip`, the refusal first and the words after (#380); and an
  Influence order already placed when an Accord with that partner is struck in the same turn is
  withdrawn at once with the word -- *"Withdrawn: 17 Influence on South Asia. It would break the
  Accord you just struck."* -- where End Turn would have refused and dropped it silently.
- **Swept** (`sweeps/ticket-569.txt`, 20 seeds x four seatings): **19 / 49 / 4 / 8 and no collapse**,
  from 9 / 59 / 5 / 7 -- the Custodians doubled, the Prospectors down ten, the swing in the two
  seatings where the Custodians sit at seat 1. Not the aim rule: with it alone reverted the table
  reads 19 / 48 / 4 / 9. It is the door reaching the computer seats: a candidate spend on a partner's
  place used to pass `check_order`, take the Influence budget and be dropped at the end, the
  Influence wasted; refused at the candidate stage now, the next candidate spends it. Income per
  Faction as the sweep file.

## 13. A load off a Colony or station is free of the Ship's turn

Ticket: [Loading from anywhere and refuelling take no turn](https://github.com/whaleyjoshua2/Dying-Earth/issues/568).

The designer: *"Loading ships from anywhere including stations should not take a turn"* and
*"Refueling a free action that does not take a turn"*. The playtest: "change-orbit, refuel, load, fly,
unload: four or five turns a shipload". In the tree a lift from a Region was free (#533) and a load off
a Colony or station was the Ship's action; the refuel was free already, its felt cost the orbit rule
(#335). Decided in one round ("q1 yes q2 a q3 yes q4 leave it"):

- **A load off a Colony or station never uses the Ship's turn** (`is_load`; `is_ship_action` drops
  it). It stands beside the one action as a lift does and boards where the Ship sits, before it moves:
  a Ship that leaves for another Body takes it aboard at the commit (`carry_load`), one beside a change
  of orbit lands with the Resolution's cargo. Loads off several places of yours in one turn, within the
  room, counted over all of them; **never two off one place** -- *"this Ship already loads from that
  place this turn"* -- as never two lifts from one Region; **never beside an unload**, either way round
  (*"a Ship loads or unloads in a turn, not both"*). A load off a place is no launch.
- **The refuel's orbit rule stands**: a station fills a Ship in its own ring alone, a Refinery Colony
  low orbit (#335, #396). The refuel is free of the turn already (#533) and the words already say
  where it fills -- the card's tank line, the refusal, the tool's help. The tool's `refuel` help
  gains the one line it lacked, that the refuel is free of the turn and goes before the transit:
  the playtest's tester 4 sat in the ISS ring and spent a turn on it from the one-order belief.
- **The 15.6 / 15.7 Fuel line** comes off the map: the designer rules it no trap -- an agent player's
  complaint at not reaching the Moon twice on one tank, which a slightly smaller tank would never have
  raised. Out of scope, not fog.
- The computer seats never load off a Colony or station (`ai.rs` lifts from Regions alone), so the
  rule reaches them through nothing. **Swept** to confirm (`sweeps/ticket-568.txt`, 20 seeds x four
  seatings): the file is byte-for-byte the Accord ticket's, 19 / 49 / 4 / 8 and no collapse, the
  Income table the same.
- Red first on the pin (`a load off a station beside a flight`) and the new test
  (`a_load_off_a_station_is_free_and_boards_before_the_ship_leaves`), both refused *"this Ship
  already has an order"* under the old rule. Clippy clean at `-D warnings`; 812 tests green.
- **The review** (an independent agent) found no hole: the touch rule is read against the orbit the
  Ship sits in as the order is given, whichever way round the load and the move are typed, as a lift's
  Launch Site rule is. It asked for the change-of-orbit path to be pinned through the Resolution, not
  checked alone -- the test now commits a load off the ISS beside a drop to low orbit and resolves it,
  the people aboard, the station two down -- and three stale comments. It also noted a gap older than
  this ticket: a Load off a station and a Send Down from it in one turn each pass their check without
  counting the other, and the loser is dropped silently at the Resolution. Not touched here.

## 14. A Ship blocks a landing only on Blockade, and Passage exempts

Ticket: [A Ship blocks a landing only on Blockade, and Passage exempts](https://github.com/whaleyjoshua2/Dying-Earth/issues/570).

The designer: *"ships do not block landing unless they are actively blockading."* The playtest: a
rival Frigate on Hold in low orbit shut a player out of their own Colony with non-aggression and
Passage Accords standing, twice, and the Report said "orbit contested" a turn after the order. In the
tree the ground answered to Orbital Control -- the one seat with a fuelled warship in low orbit, any
stance, Passage ignored, read at the Resolution alone -- while a station's ring already wanted a
warship on Blockade and exempted Passage. Decided in one round ("q1 yes q2 stays q3 yes q4 a"):

- **The ground reads the ring's rule** (`may_land` is `!orbit_blockaded_against(.., Orbit::Low)`): a
  landing or founding is shut only by a rival warship on Blockade in low orbit, fuelled for a Battle;
  a Passage partner never shuts it; the seat's own ground Colony's working Battery lifts it. A rival
  on Hold, Attack or Intercept shuts nothing, held outright or not, and a contested orbit lifts no
  Blockade, as it lifts none of a ring's.
- **Orbital Control stays** for everything else it gated: a Bombard and a Launch from low orbit, the
  starving of a ground Colony (which wanted a Blockade too, already), and the flag on the map.
- **Refused at the order** (`blockaded_words`): the ground founding and the unload into a Colony --
  a station's too, which the door never read either -- refuse with *"a rival warship blockades low
  orbit over Mars"* or *"a rival warship blockades Ares"*, where they passed and failed at the
  Resolution a turn later. The Resolution's re-read says the same words through a `{why}` on
  `landing_contested`, or *"the Ship is not in the orbit that touches it"* when that is the reason;
  "orbit contested" named a rule that shuts nothing now. The Colonists-waiting line reads *"blocked: a
  rival warship blockades low orbit"*.
- **The computer blockades the ground**: its one Blockade candidate was against a rival station in
  the ring its warship sat in, so it never ordered one in low orbit and, with the ground off Orbital
  Control, could shut nobody's ground at all. It gains the mirror: a warship in low orbit over a
  rival's ground Colony whose holder it has cause against, no Battery of that holder below. That
  Blockade also starves the Colony under an outright Control and shuts its Refinery depot, standing
  rules the computer never reached.
- Red first on six tests: four pins that read Control as the landing rule, the waiting line's words,
  and the new test (the door, Passage, the Battery, the Resolution's words, the computer's
  candidate). Clippy clean at `-D warnings`; 813 tests green.
- **The review** (an independent agent) found no hole. The ring founding's door said "a rival warship
  holds Orbital Slot 2 over Mars" where its re-read said "blockades Ares": one wording now. The
  test's Battery-below assertion passed for the wrong reason (one Battery is a fight the Frigate's
  odds carry, and an Attack offered forces the Blockade off before the filter is read): three
  Batteries now, and no Attack asserted beside no Blockade. A dozen comments and glossary clauses
  still read Control as the landing rule: mended. Two flags for the designer, left as built: the
  computer's low-orbit arm has no Earth guard, where the ring arm has none either, so a computer
  hull with cause will blockade Earth's low orbit over a rival's Antarctic Colony, shutting Antarctic
  foundings for every rival and starving it under an outright Control; and a Blockade of a Passage
  partner's ground shuts no landing but still starves it and counts offences, as the ring arm always
  has. The Attack trigger "my Colony here and Control held against me" now opens a war on a rival that
  threatens only a Bombard or a Launch; left, the sweep measures it.
- **Swept** (`sweeps/ticket-570.txt`, 20 seeds x four seatings): **19 / 47 / 4 / 10 and no collapse**,
  from 19 / 49 / 4 / 8 -- noise on the wins. The rule reached the computer seats where it was meant to:
  Blockades ordered over the batch 254 from 12, Blockade-turns suffered 217 from 10. The Prospectors
  order 192 of them (91 in the seating where they sit at seat 0) and the Custodians suffer 150 of the
  turns; the Arkwrights order most of the rest. People killed by the nuke over the batch 2,318 from
  6,930 on one Launch fewer (45 from 46). Income per Faction as the sweep file, within a few percent
  of the last.

## 15. A station's colony id and its orbit number agree

Ticket: [A station's colony id and its orbit number agree](https://github.com/whaleyjoshua2/Dying-Earth/issues/571).

The designer: *"Make Colony ids and orbit-slot numbers match for the same station."* The playtest: Axiom
was colony 21 and orbit 2, and a `change-orbit` typed with the colony id sent a Ship to a ring that
did not exist, stranding it for three turns; another player asked for `change-orbit 27 tiangong`. In
the tree a Colony's id is a global counter and a station's orbit is its slot index per Body, so one
number cannot be made equal; the text tool took a slot number alone, and `show` never printed the
slot beside the station. The window selects by click and names orbits by name, so it had no such
confusion. Decided in one round ("q1 yes q2 yes q3 nothing"):

- **Every order that takes an orbit takes a name or a colony id too** (`orbit_arg` in `play.rs`):
  `transit <ship> <body> [orbit]`, `change-orbit <ship> <orbit>`, `unload ... into ring <body>
  <orbit>`. An orbit is `low`, the slot number as `show` lists it, the station's name by any unique
  prefix with spaces and case ignored (`tiangong`, `orb`, `marsbase`), or a standing station's
  colony id as `colony:21` / `c21`. **A bare number is always the slot number**, as it was, so nothing
  already written changes meaning; a number past the Body's slots is refused with the orbits listed,
  where the engine used to take it and the Ship flew to nothing. A colony id over another Body or on
  the ground is refused by name. The ground's `into slot <body> <slot>` takes the slot's name too
  (`slot_arg`).
- **`show`** prints the orbit number beside the station's name on its colony line -- `Tiangong
  (orbit 1)` -- so the two numberings sit side by side where the player reads them. The Ship's line
  stays "Earth, at Tiangong", since the name is now enough to type.
- **The window**: nothing.
- No rule moves, so no sweep. Red witnessed through the tool itself, which has no tests of its own:
  `change-orbit 1 tiangong` and `change-orbit 1 colony:21` refused *"is not a whole number"* by the
  parser before, and passed through to the engine's own door after. Clippy clean at `-D warnings`;
  813 tests green.
- **The review** (an independent agent): a word that is no number but norms to digits (`1.`, `-1`)
  would have matched the slot's number form in the name matcher -- refused as "not a whole number"
  now; the refusal's orbit list said "low" at Venus, which has none -- built from the Body's own
  orbits now; `change-orbit` answers "not your Ship" before it reads where a hull is, as the engine
  does, so a hidden hull in flight gives nothing away; and `build station <body> <orbit>` takes the
  ring by name or colony id too, being an order that takes an orbit.

## 16. A refused offer waits three turns before it comes again

Ticket: [A refused offer waits three turns before it comes again](https://github.com/whaleyjoshua2/Dying-Earth/issues/572).

The designer: *"If an offer is refused the AI must wait 3 turns before offering it again."* The
playtest: "every 5 turns all three rivals re-offer non-aggression, and they keep offering the same 20
Energy for 20 Fuel trade; the turn refuses until I answer each one" (three testers). Read in the tree
first: the cooldown already stood at three turns (`offer_refused_turns`), keyed by the pair, so a
refusal silenced that rival's Accords and Trades alike; and a human is never prompted to renew --
since 0.1.0.1 an Accord in its last turn is renewed by the player's own Renew order -- so what the
testers felt as "every five turns" was the turn after an Accord ran out, which was no refusal, and
the fresh offer came at once. Decided in two rounds ("q1 a q2 [with a counter] q3 yes q4 no", "q5 yes
q6 b q7 yes"):

- **The cooldown stays at three turns, keyed by the pair** (Q1 a). Three rivals on their own clocks
  give about three offers in four turns under blanket refusal; the two real nags are the next two.
- **The same Trade again** (Q2): the last Trade a seat refused is remembered per pair by its two
  goods -- what the offerer gave and what it asked (`RefusedTrade`, in the save) -- and the computer
  offers that seat no Trade of the same two goods again, the cooldown over or not, until a Trade of
  different goods has been refused in its place. The amounts do not count: 20 Energy for 20 Fuel and
  20 Energy for 25 Fuel are one Trade. The designer took this on condition that a counter could be
  negotiated, which is the next point.
- **The counter-offer** (Q5, Q6 b, Q7): a Trade in goods offered to the player may be countered once
  -- the same two goods, the player's own amounts -- and the offerer answers **at once**, in the same
  prompt, by its Trade rule (`counter_offer`): the goods move, or the counter is refused, before the
  turn goes on. A seat still **short of the good it asked for** (under `trade_short`, 15) pays up to
  **a quarter over fair value** at the Trading window's prices (`trade_counter_margin`, 0.25 in
  `ai.toml`), since a seat that is short pays over the odds; one not short pays fair value alone, as
  it takes any Trade. A refused counter is the refusal: the cooldown runs and the goods are
  remembered, and the Report reads *"{faction} refused your counter."*; a taken one reads as a Trade
  struck. Either way the offer is gone, so no second counter follows. A counter the player cannot
  pay, of nought, or on a place, is refused at the door and the offer stays. A computer seat never
  counters. The window: the offers prompt gives a Trade in goods a row *"Or counter: give [60]
  Ducats for [30] Materials [Counter]"*, prefilled from the offer. The tool: `trade counter <faction>
  <n> <good> for <n> <good>`, the player's give first and their ask second, mirroring `trade`; a
  counter naming other goods than the offer's is refused with the offer's goods named.
- **An Accord the player lets run out** (Q3): when the computer partner would have renewed and the
  player gave no Renew, the run-out is remembered by that partner as a refusal, so its fresh offer
  waits the cooldown where it came the next turn. A partner that would not renew, an Accord called
  off or broken, and two computer seats (who both renew or one refuses) are untouched.
- **No "refuse all"** (Q4).
- Red first on three tests: the two new ones below and the renewal test of ticket #509, whose
  "nothing is remembered" the decision reverses. Staged with the new calls as stubs so each failed
  on its rule, not on the compiler: *`Err("a counter is not yet answered")`* against `Ok(false)`,
  *`trade_refused_before` false* after a refusal, and *the run-out counts as a refusal* false. The
  tool's red: `trade counter prospectors 20 fuel for 25 ducats` refused *"counter" is not one of:
  custodians, prospectors, ...* before, and reaching the engine's door after (*Prospectors have
  offered you no Trade in goods*). Clippy clean at `-D warnings`; 815 tests green.
- **Swept** ([`sweeps/ticket-572.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-572.txt),
  20 seeds x four seatings): 21 / 48 / 4 / 7 and no collapse, from 19 / 47 / 4 / 10 -- noise. Accords
  struck over the four seatings 646 (156 / 168 / 166 / 156) from 656: the run-out rule reaches only a
  human seat's partner, and the same-goods rule only moves a computer seat's Trade to another
  partner, so the computer-only batch is unmoved, as it should be. Income per Faction (Materials /
  Fuel / Energy / Ducats / Research, mean a game): Custodians 1072 / 348 / 2524 / 1507 / 928;
  Prospectors 2142 / 443 / 2739 / 1188 / 382; Arkwrights 1528 / 306 / 2277 / 3212 / 261; Archivists
  473 / 259 / 1139 / 467 / 740 -- all within noise of the last sweep.
- **The review** (an independent agent) found every orientation right (who is `from` and `to` in
  each memory, the counter's give and get against the Trade rule and the settle, the save) and the
  AI test's premise holding for the right reason. It caught a taken counter withdrawing a crossing
  Influence order as a struck Accord does, which a Trade in goods never is -- removed; three test
  gaps -- a renewal asserted no refusal only after the cooldown would have run out, and no test of
  the run-out when the partner would not renew or between two computer seats -- all three asserted
  now; the stale "letting it lapse is no refusal" comment on a refused Renew offer; the `accord
  renew` help silent on what a run-out now costs, and "three turns" hard-coded where the help
  templates its figures (`{refusedturns}` now); and a counter written in the offer's own order
  refused without saying the form the line wants -- it says it now.

## 17. The Archive fund at 250

Ticket: [The Archive fund at 250](https://github.com/whaleyjoshua2/Dying-Earth/issues/579).

The designer: *"archive fund at 250."* One figure in two files: the Archive's Research cost
(`[archive] research`, `modules.toml`) and the Archivists' first Victory part (`victory_first.bar`,
`factions.toml`), a test holding them equal since ticket #439. 150 at #439, 225 at #506 with the
game at fifty turns. Decided in one round ("q1 yes q2 a q3 report and leave it"):

- **Both figures to 250.** The Fund the Archive order's cap, the `archive_funded` Report line, the
  Victory text on the Faction card, the turn-one line and the tool's `show` all read the figure.
- **The computer Archivists' pace** (`[pace.archivists] first`, `ai.toml`) is scaled by 250/225 on
  the same turns: 62 / 126 / 188 / 250 by turns 14 / 25 / 36 / 44, where it ran 56 / 113 / 169 / 225.
  A bar the computer does not pace to is a bar it stops short of.
- Nothing else by rule. The glossary's Archive entry reads 250 since 0.1.0.3.
- Red first on three tests (the one-figure test, the tables test with the pace, the turn-one line),
  each failing *left: 225, right: 250* before the tables moved. Clippy clean at `-D warnings`; 815
  tests green.
- **The review** (an independent agent) found no stray 225 and the pace's rounding right (62.2 / 125.6 / 187.8 to the nearest), and every intermediate pin relative to the table. It caught the Archivists' long Victory text still typing its figure where ticket #580 gave the card a placeholder -- it reads `{first}` and `{second}` now, pinned by a test, and the #347 test that pinned the typed copy pins the rendered text; a dead test arm setting the Archivists' Research total where their first part is scored on the fund -- it sets the fund from the table now; and a test name still saying 150 -- renamed. All mended.
- **Swept** ([`sweeps/ticket-579.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-579.txt),
  20 seeds x four seatings): **20 / 48 / 5 / 7 and no collapse**, from 21 / 48 / 4 / 7 -- noise, the Archivists' seven wins of eighty unmoved by the higher bar, their gate still completed in 68 of 80 games (69 before) at a median turn 30 (29); the computer paces to the new figure, the median fund at the end 250 in every seating where the Archive stands (225 before). Income per Faction (Materials / Fuel / Energy / Ducats / Research, mean a game): Custodians 1157 / 366 / 2641 / 1580 / 970; Prospectors 2190 / 434 / 2769 / 1234 / 390; Arkwrights 1554 / 316 / 2230 / 3273 / 256; Archivists 481 / 264 / 1137 / 474 / 734 -- the Archivists' Research 734 from 740, within noise. Reported and left, at the designer's word; the close-out's sweep sees the version whole.

## 18. Card and permafrost Emissions count toward the Stabilization run

Ticket: [Card and permafrost Emissions count toward the Stabilization run](https://github.com/whaleyjoshua2/Dying-Earth/issues/578).

The designer: *"make event and permafrost emissions count toward the custodian run."* In the tree the
run read the **counted** Emissions -- the sources somebody owns: industry, Factories, Power Plants,
Refineries, launches, population and war -- against the Sink and the Scrubbers, and left out the
Event cards' Emissions and the permafrost, which the net and the temperature count. So the world
could warm while a phase counted as under. Decided in one round ("q1 yes q2 a q3 yes q4 yes"):

- **The run reads the net.** A Climate phase counts toward the Stabilization run when the whole of
  the Emissions, cards and permafrost included, stand under the Natural Sink and the Scrubbers
  together (`climate.rs`: `breakdown.total() < breakdown.total_sink()`). One figure, one truth: the
  one the temperature reads. A card's Emissions land in one phase, so a single draw can break a run;
  the permafrost only ever rises, so a late run is harder and never easier.
- **The partial credit and the ease read it too**: the Custodians' opening gap and best share of it
  closed, and the once-a-game Under the Sink ease, read the net gap beside the run, so nothing in the
  game says "under the Sink" by two measures.
- **The computer Custodians** pace their run on the net (`victory_gap`), so their Scrubbers and
  Production Moved push against what the run reads.
- **The texts**: the Stabilization line reads *"Stabilization: 13.7 ppm over the Sink (Emissions 21.7,
  Sink 8.0), run 0 of 4"* where it said "counted"; the permafrost hover says it counts against a run
  where it said it did not; the glossary's Stabilization entry follows. Nothing new in the Report: the
  Climate line already prints the cards and the permafrost. `counted()` stays as the breakdown's sum
  of the sources somebody owns, which the Blame ledger is made of.
- Red first on three tests: the Permafrost Thaw's phase, which asserted *"exempt from Stabilization:
  the run goes on"* (left 2, right 0 now); the Under-the-Sink ease, which asserted the run under with
  the permafrost pushing the net over (left 1, right 0); and the Stabilization row's words ("counted"
  against "Emissions"). Clippy clean at `-D warnings`; 815 tests green.
- **The review** (an independent agent) found every reader right -- the run, the opening gap, the ease, the computer's pace, the Production Moved worth and the three printers of the line all on the net, `counted()` with no caller left but `total()`, and the Blame ledger untouched. It caught the Sink Weakens test proving its claim for the wrong reason (the Permafrost Thaw fires at 1.6 beside the Sink Weakens at 2.0, and its 4.0 now put the second phase over the old Sink as well -- the thaw is set aside in that test, which is about the Sink); the Methane Burst test named for the abolished exemption (renamed, and it witnesses a card breaking a run now); and five lines of prose still saying "counted" or "exempt" -- the glossary's opening gap, `unrest.toml`'s ease, the `opening_gap` field, the Carbon Pulse's doc and the permafrost line's comment. All mended. **One note, left as built**: a save from 0.1.0.2 carries an opening gap measured on the sources alone, so on that save the best share closed is compared across two measures for the rest of the game; it never falls back, so nothing is lost.
- **Swept** ([`sweeps/ticket-578.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-578.txt),
  20 seeds x four seatings): **20 / 49 / 4 / 7 and no collapse**, from 20 / 48 / 5 / 7 -- noise; the Custodians' twenty wins unmoved, their gate still completed in 75 of 80 games at a median turn 15. The world under the Natural Sink at least once, which the sweep counts by the run's test, in 28 of 80 games (median first turn 35) from 30 (34): the cards and the permafrost cost the computer Custodians two games' first run and a turn, no more, which says the late permafrost rarely decides it. Income per Faction (Materials / Fuel / Energy / Ducats / Research, mean a game): Custodians 1185 / 370 / 2740 / 1613 / 1045; Prospectors 2216 / 433 / 2796 / 1264 / 396; Arkwrights 1561 / 316 / 2273 / 3391 / 261; Archivists 497 / 272 / 1179 / 514 / 757 -- the Custodians' Research 1045 from 970, the one figure past noise; not traced.

## 19. The Cheap Ore Offer's refusal halves Materials for the refusing seat, one turn

Ticket: [The Cheap Ore Offer lasts one turn, and a refusal halves Materials for the refusing seat alone](https://github.com/whaleyjoshua2/Dying-Earth/issues/577).

The designer: *"Cheap Ore Offer now last for a single turn, a refusal makes material costs half for
just that seat."* The card is answered the turn it is drawn ("this turn only"), so the single turn is
the refusal's effect, two turns in the tree; and that effect set the **market's** Materials row to
1 Ducat for the whole table, rivals included, at a third of the banded price rather than half.
Decided in one round ("that all sounds good let's go"):

- **A refusal halves the Trading window's Materials price for the refusing seat alone**, every lot
  bought that turn (`CardEffect::SeatPrice`, `PriceCut` on the seat, in the save with a serde
  default). The market's own price does not move; a rival pays the window's price; the Materials
  cost of builds and the build-with-Ducats buttons are untouched -- the card is a broker's offer,
  and the market is where it lands.
- **One turn**: the refusal lands at the Resolution of the turn it is answered in, after that turn's
  trading, so the half price stands through the next Orders turn and is forgotten at its settle, as
  a card's market price counts its turns.
- **With the Prospectors' 0.85**: multiplied, to the tenth, as the lot is priced (`lot_price`): a
  Prospector pays 0.425 of the price.
- **The computer**: takes the lot when it holds 20 Ducats, as before -- thirty Materials for 20
  Ducats beats a turn of half price unless the seat would buy sixty -- and a computer refuser buys
  at its own price that turn through the same door as any Buy.
- **The texts**: the refuse line reads *"Let it go: Materials at half price for you next turn"*;
  the card's effect reads *"Materials at half price for you next turn"*; the Market line and the
  Trading window's price column say *"half for you, this turn"* beside Materials while it stands;
  a Prospector's column reads the two as one clause, *"(x0.85 and half for you, this turn)"*, since
  the cell is 340 wide (the review). Rivals see nothing.
- Red first: the new test and the Market line's test failed on the old card held back (its refusal
  still a market price), and compiled red on the new calls before that; the band-override test of
  ticket #337 reads the Fuel Contract now, since the Cheap Ore Offer no longer moves the market.
  Clippy clean at `-D warnings`; 816 tests green.
- **The review** (an independent agent) traced the clock through the turn (asked at the head of T, landed at T's Resolution, kept by T's settle, read by T+1's Orders, dropped at T+1's settle: one Orders turn, the three comparisons the card price uses), found every Buy figure going through the one cost door -- the check, the commit, the Resolution's line, the computer's top-up, the window's button -- the sell-back and the build-with-Ducats buttons untouched, the serde tag right, every match on the effect covered, and the save loading an older seat with no cuts at SAVE_VERSION 12. It caught a field on the cut written and never read (dropped); the Fuel Contract rewrite's tolerance accepting the moved price itself (an exact assert again); a spent-cut assertion looking for a card name the line never prints (it looks for the note now); the test's stale doc; two stale docs naming the old 1-Ducat Materials; and the Trading window's price column at some ninety characters for a Prospector with both notes -- one clause now, *"(x0.85 and half for you, this turn)"*. All mended.
- **Swept** ([`sweeps/ticket-577.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-577.txt),
  20 seeds x four seatings): **19 / 49 / 6 / 6 and no collapse**, from 20 / 49 / 4 / 7 -- noise, as a one-copy card's refusal should be. Income per Faction (Materials / Fuel / Energy / Ducats / Research, mean a game): Custodians 1177 / 364 / 2720 / 1602 / 1049; Prospectors 2216 / 432 / 2799 / 1268 / 396; Arkwrights 1589 / 329 / 2307 / 3524 / 270; Archivists 493 / 271 / 1181 / 518 / 755 -- within noise of the last sweep; the Arkwrights' Ducats 3524 from 3391 is the widest move and still inside the batch's swing.

## 20. The East Asia Custodians never leave Earth

Ticket: [The East Asia Custodians never leave Earth](https://github.com/whaleyjoshua2/Dying-Earth/issues/576).

Found closing [The computer Custodians use Production Moved](https://github.com/whaleyjoshua2/Dying-Earth/issues/563):
nought from the East Asia start. The designer: *"add a ticket to look at that."* Asked whether it
still stood, the last sweep said yes -- the Custodians from East Asia 0 wins of 20 against 3, 7 and
9 from the other starts, their gate done in 15 seeds at a median turn 26 against 20 at 11 to 17,
Production Moved 15 Module-turns against 120 to 195, Materials income 242 against 1150 to 1915.

**The diagnosis**, from a replay of seeds 1 to 3 with every seat on the computer
([`ticket-576/replay-east-asia.txt`](../dev-diary/2026-10-19-version-0.1.0.3/ticket-576/replay-east-asia.txt)):
not the Scrubber cap, the Materials reserve or the Influence fight the ticket guessed at, but the
computer Custodians' own mothball reason *"the run is still nothing"*, which fired every turn once a
Scrubber stood and the Stabilization run was nought. From East Asia the Scrubber is built on turn 1
(the biggest cap, a one-turn build, weight 16), so the reason shut the Refinery on turn 2, the Power
Plant on turn 3 and the Mine on turn 4, for 1.1 and 0.6 ppm against a world 32 ppm over the Sink;
the Scrubber was then mothballed on turn 4 for Energy, Fuel income was nought from turn 4, Energy at
nought by turn 12, and the Colony Ship the seat was saving for from turn 3 was never paid (*"needs 35
Fuel, 10.6 left"* on turn 9, two Fuel sold the turn before). Nothing left Earth but Antarctica by sea
on turn 16. From Europe the same reason fires, but the Scrubber completes on turn 5 and the Colony
Ship on turn 7, the Moon on turn 9, before it has gutted the home. Decided in one round ("q1 a q2
yes q3 sure"):

- **The guard**: the reason fires only where shutting the building could start the run -- the
  world's net over the Sink (the last Climate record's net, as the run reads it since §18) no more
  than the building's own Emissions plus the Custodians' pace `within_ppm` (4). Against a 32-ppm gap
  a mothball for 1.1 ppm is pure loss; late in the game, within reach of the Sink, the reason works
  as it did. The net is the last Climate phase's, which `Game::new` runs before turn 1's orders.
- **The turn-one Scrubber** at East Asia is left: with the Power Plant standing the Energy loop that
  shut it does not start.
- **The measure**: the Custodians' wins from East Asia above nought, their gate done in 18 or more
  of 20 seeds at a median turn near the other starts, Production Moved from East Asia well above its
  15, and no fall from the other three starts.
- **The replay after**: all three seeds build the Colony Ship on turn 6 and found the Moon on turn
  8, Antarctica on turns 15 and 17, Deimos or Phobos by turn 25; Energy in the hundreds by turn 16.
- Red first on one test (`fresh()`, the home's starting Refinery, Power Plant and Mine standing, a
  Scrubber added, the world far over the Sink): the old reason shut a producing Facility; with the
  guard it shuts none, and shuts the dirtiest where the net is within its reach. Clippy clean at
  `-D warnings`; 817 tests green.
- **The review** (an independent agent) found no blocking defect: the net read is the run's own figure in the same units as a Facility's Emissions, the let chain compiles under the 2024 edition, Modules stay excluded, the Energy and Production Moved reasons are untouched, and the test proves its claim (no other reason can produce the order, the choice is deterministic, the tie between the Power Plant and the Refinery falls the same way on both sides). It asked for the plain read of the last Climate phase's net in place of a dead `Option` (the phase runs inside `Game::new`, so there is always one), a name that does not shadow the Victory multiplier (`over`), the table's stale comment on the reason, and the test's margin to say the standing Scrubber's sink comes off the net too. All mended.
- **Swept** ([`sweeps/ticket-576.txt`](../dev-diary/2026-10-19-version-0.1.0.3/sweeps/ticket-576.txt),
  20 seeds x four seatings): **22 / 48 / 1 / 7 and 2 collapses**, from 19 / 49 / 6 / 6 and none. **The measure, part met and part missed.** Met: Production Moved from East Asia 60 Module-turns a game from 15; the Custodians' Materials income from East Asia 977 from 242; no fall from the other three starts (3 / 10 / 9 wins from 3 / 7 / 9). Missed: **the Custodians from East Asia still win 0 of 20**, and their gate is done in 16 seeds at a median turn 24, from 15 at 26 -- short of the 18 at a median near the other starts' 11 to 17. The home is no longer gutted, and the seat leaves Earth (the Moon on turn 8 in every replayed seed), but the seat 0 Custodians do not win from there, and the sweep's own line says what is left in the way: seat 0 loses its start state in 8 of 20 seeds (median turn 28), the ticket's third guess, the Influence fight for East Asia; not traced further. The Arkwrights' 1 win of 80 from 6 is at the edge of noise and not traced. The Custodians everywhere: Income per Faction (Materials / Fuel / Energy / Ducats / Research, mean a game): Custodians 2117 / 718 / 3755 / 1634 / 1320 from 1177 / 364 / 2720 / 1602 / 1049, since the reason gutted the home from every start, only later; Prospectors 2279 / 473 / 2946 / 1249 / 430; Arkwrights 1590 / 348 / 2300 / 3350 / 264; Archivists 468 / 243 / 1114 / 499 / 755.
