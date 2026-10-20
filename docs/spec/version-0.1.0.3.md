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
