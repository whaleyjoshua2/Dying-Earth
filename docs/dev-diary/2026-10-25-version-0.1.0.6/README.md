# Version 0.1.0.6, the fission version

The map is [Map: version 0.1.0.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/627); the spec
is [`docs/spec/version-0.1.0.6.md`](../../spec/version-0.1.0.6.md). The standing baseline is 0.1.0.5's
closing board, **18 / 61 / 0 / 1 and 0 collapses**
([`final-0.1.0.5.txt`](../2026-10-24-version-0.1.0.5/sweeps/final-0.1.0.5.txt)), against the designer's
15 each and 20.

## Fissionables, the seventh resource

Ticket [#628](https://github.com/whaleyjoshua2/Dying-Earth/issues/628), spec §1.

**The glyph** was chosen off a sheet, six candidates in four colours, each large and at the 16 pixels
the top bar draws, beside the Materials, Fuel and Energy it stands with:
[`ticket-628/fissionables-glyph-candidates.png`](ticket-628/fissionables-glyph-candidates.png). The
designer took **1A**, the radioactive trefoil in lime; atom-core and unstable-orb blurred at bar size
and amber would have read as Fuel.

**Rich Seam**, asked about while deciding what lifts the new figure: a one-copy Event card off Earth
that doubles every Mine on one Body for two turns, x3 with Deep Mining. The first recommendation, that
the Mine Techs should not lift Fissionables lest the Prospectors become the Fissionables Faction, was
withdrawn in the same answer: a Tech, once complete, is every Faction's. The designer's answer: the
Techs lift them, no card does, Deep Mining x1.25 and Rich Seam 1.5 and 2.

**Built test first.** Seven tests went red on their missing names before any engine code: the Bodies'
figures, the sites (forty seeds: Hellas Planitia never, the Moon two or three and both seen), the
draw leaving the game's dice alone, the Mine's figure through Deep Mining with a Discovery that does
not reach it, Rich Seam's figures, the market, and a 0.1.0.5 save loading with its sites drawn. Two
passed the first time they ran and were broken on purpose:

- **The dice.** The draw moved onto the game's own dice still passed: the pins read the slot yields
  and the deck, both drawn before the sites. A third pin, the game's next roll read from 0.1.0.5's own
  code (stashed and run), went red when the draw spent one die.
- **The load.** Taking the load's draw out turned the save test red at its first site.

Six tests that pinned Deep Mining at x1.5 moved to x1.25 (8.3 for China's Mine where it was 9.9, the
Antarctic premise computed from the card since the chain tenths once at its end). The Arkwrights'
race test had its runner already five turns out to Mars by turn 10 on this seed; the test now counts
a runner under way.

**The first picture caught what the tests did not**: the Mars Colony's Output row carried no
Fissionables, since the row is built from a list of sources the second output never reached. A test
was added, watched red (0.0 for 0.8), then the list mended.

![The Mars Colony, its Output row and the top bar](ticket-628/colony-mars.png)

![The Mine's figures: +4.1 Materials, +0.9 Fissionables](ticket-628/mine-mars.png)

![Hellas Planitia, four yields; Chryse Planitia's label beside it, five](ticket-628/hellas-mars.png)

![The Trading window's fifth line](ticket-628/trade-earth.png)

**The sweep** ([`sweeps/ticket-628.txt`](sweeps/ticket-628.txt)): **23 / 51 / 3 / 1 and 2
collapses**, from 18 / 61 / 0 / 1 and 0. Income a game, net of upkeep, by Faction:

| | Materials | Fuel | Energy | Ducats | Research | Fissionables |
|---|---|---|---|---|---|---|
| Custodians | 1,781 (2,135) | 561 (560) | 1,104 (869) | 1,270 (1,279) | 1,095 (1,135) | 56.5 |
| Prospectors | 2,046 (2,618) | 471 (503) | 578 (537) | 1,802 (1,737) | 343 (393) | 36.9 |
| Arkwrights | 721 (821) | 147 (146) | 207 (248) | 2,927 (2,826) | 314 (311) | 2.5 |
| Archivists | 400 (472) | 232 (221) | 29 (54) | 500 (527) | 727 (710) | 2.2 |

In brackets, 0.1.0.5's closing board. Deep Mining's cut took a fifth off the Materials, the Prospectors
the most, and they fell 10 wins; nothing spends a Fissionable yet, and the computer does not seek them.

**The review** (a reader who did not build it) found one rule nobody had decided: the first build let
Production Moved's x2.08 and a shut Habitat's halving reach the Fissionables, where the designer named
three lifts -- the site, the Faction's output, the Mine's Techs. A test went red (1.7 where 0.8), both
were taken off, and the sweep run again: the Custodians' Fissionables 111.7 a game to 56.5, the win
column the same to the game. Five smaller findings were mended: the Deep Core Survey's question says
*Materials*, a Trade's refusal and the text tool's help name Fissionables, a lone `f` in the text tool
is still Fuel, and the game-over table has a Fissionables column. A Mass Driver beside the Mine is
pinned not to lift them. The computer's reading of a Colony's worth leaves Fissionables out, on
purpose, until the computer is taught them.

## What Fissionables buy: the nuclear hull and the Warhead

Ticket [#629](https://github.com/whaleyjoshua2/Dying-Earth/issues/629), spec §2.

Decided in one round. The recommendation for the second fuel was a bill on the Stockpile at each leg,
no tank; the designer chose the tank (Q4 a) and set its fill as the Fuel tank's is, a share that grows
with the tank's Techs. The yard's shape -- a second button per row, *Nuclear* -- was left to me and
shown.

**Built test first.** Five tests went red on their missing names: the Tech's gate and the fill, the
leg's burn and the nuclear hull's crossing, Refuel's two tanks, the Warhead's half, and the computer's
choice. They passed when built, so the rules were broken on purpose: the hull's leg read as chemical,
the burn left in the tank, Refuel's second fill dropped. Two tests went red (the crossing and the
burn in one, Refuel in the other) and green again on restoring.

Nine older tests moved. Nuclear Rockets reaches a nuclear hull alone, so its five tests read the
Tech's turns off a nuclear hull's quote; the table's own quote -- the window hover and the Report's
window line -- is a chemical hull's, and one test says so. Four Missile Carrier tests stood their
seat on no Fissionables, so the Warhead's half was refused; each was given some. The computer's
Missile Carrier test showed the rule's other side: a computer seat with no Fissionables arms no
carrier, which the computer's chase ticket takes up.

**The first pictures caught two things.** The yard's Nuclear buttons showed Materials, Fuel and
Widgets and no Fissionables, since the button face draws its own list of figures; and every button was
greyed, the aid's Stockpile 29.6 Fuel against tanks of 30 and more. The second picture's nuclear
Frigate read *To Earth: 6 turns*: checked, a chemical hull's leg that turn is 7.

![The yard: a Nuclear button beside every Ship, its Fissionables on its face](ticket-629/yard-mars.png)

![A nuclear Frigate: its two tanks, and each leg's Fissionables](ticket-629/hull-mars.png)

**The sweep** ([`sweeps/ticket-629.txt`](sweeps/ticket-629.txt)): **19 / 56 / 2 / 1 and 2
collapses**, from the last ticket's 23 / 51 / 3 / 1 and 2. Income a game, net of upkeep, by Faction:

| | Materials | Fuel | Energy | Ducats | Research | Fissionables | Nuclear hulls built (80 games) |
|---|---|---|---|---|---|---|---|
| Custodians | 1,806 | 572 | 1,084 | 1,266 | 1,122 | 57.3 | 95 |
| Prospectors | 2,052 | 470 | 575 | 1,821 | 350 | 32.7 | 176 |
| Arkwrights | 725 | 155 | 206 | 2,982 | 318 | 2.6 | 5 |
| Archivists | 404 | 235 | 27 | 513 | 739 | 2.1 | 2 |

Launches rose from 18 to 24 over the 80 games. Reported, not tuned.

**The review** (a reader who did not build it) found one defect the tests had missed: Refuel was
refused while a nuclear hull's Fuel tank stood full, so a hull filled when the Stockpile held no
Fissionables could never fill its second tank and never fly again, the computer's top-up refused with
it. A test went red, and Refuel now fills while either tank wants it and says which good is missing.
Four lesser findings were mended: Nuclear Rockets' line says it opens the nuclear hull; a nuclear
hull's drift line, its solar-view marker and the arrival-stranded warning read its own turns and its
Fissionables; and a change of orbit rescues a stranded hull only if it can pay its Fissionables too.
The sweep, run again, matched the first to the game.

## The Reactor and the Accumulator

Ticket [#630](https://github.com/whaleyjoshua2/Dying-Earth/issues/630), spec §3.

Decided in two rounds. The designer gated the Reactor behind Clean Power where I had recommended no
gate, asked for more names than my four, and took **the Accumulator** from seven.

**A thing I had wrong.** I told the designer the Archivists' building wore a cooling tower and that the
tower would go to the new Reactor. The candidate sheet, with the two reference pictures at its head,
showed otherwise: the Archivists' building wore the trefoil, and the cooling tower is the Power
Plant's. The decision still holds -- the trefoil goes with the name to the Reactor -- but the Reactor's
trefoil and the Fissionables glyph are now near kin, off-white on a building tile and lime beside a
figure; the Module's picture below shows both at once.

![The Accumulator candidates, the Power Plant and the trefoil at the head](ticket-630/accumulator-glyph-candidates.png)

**Built test first.** Four tests went red on their missing names: the cards and the rename, the
Reactor's burn and its quarter without Fissionables, the Clean Power gate and the Module's indifference
to the sun, and the computer's ten turns. A fifth, a save naming the Archivists' `reactor` loading it as
the Accumulator, passed first. Each was broken on purpose and went red: the alias removed, the
Reactor taken out of the Energy multipliers (8 where 15), and the Fissionables left out of the upkeep
reservation (a starved Reactor burning -0.1). Five older tests moved: the Facility count, the order of
the appended kinds on both lists, and the Report's word ceiling by the nine of *Fissionables ran short*.

![China, the Archivists' Accumulator first in its row](ticket-630/region-earth.png)

![A Reactor Module at Mars: +12 Energy with Efficient Grids, 2 Ducats and 0.1 Fissionables upkeep](ticket-630/module-mars.png)

**The review** (a reader who did not build it) found one high defect and five lesser. The high one:
the computer counted a Generator's Energy and never a Reactor Module's, so a seat powered by Reactor
Modules read itself short of power for ever and went on building them -- which the first sweep's 842
Custodian Reactors fitted. A test went red (0 where 8) and the count was mended, standing and on
order. The others: the mothball shut every Reactor when only some could not be fed, and could
mothball and restart in a cycle -- now only the unfed are shut, and a restart waits for ten turns of
every running Reactor and itself (a test pins both); the Shortfall forecast read the Fissionables
before this turn's orders spent them; a dark or blockaded Reactor Module burned Fissionables it had
not reserved, which could take the Stockpile below nought; the Collapse weighting gave a Reactor no
clean preference; and a window test's fixture still said *Reactor relief*.

**The sweep** ([`sweeps/ticket-630.txt`](sweeps/ticket-630.txt)), after the mending: **28 / 47 / 1 / 1
and 3 collapses**, from 19 / 56 / 2 / 1 and 2 (the first build's read 31 / 46 / 1 / 1 and 1). Income a
game, net of upkeep, by Faction:

| | Materials | Fuel | Energy | Ducats | Research | Fissionables | Reactors standing at the end (80 games) | Nuclear hulls built (80 games) |
|---|---|---|---|---|---|---|---|---|
| Custodians | 2,231 | 630 | 1,002 | 1,118 | 1,079 | 88.9 | 699 | 158 |
| Prospectors | 2,205 | 512 | 866 | 1,628 | 360 | 40.9 | 431 | 170 |
| Arkwrights | 727 | 158 | 207 | 2,798 | 290 | 5.5 | 53 | 17 |
| Archivists | 397 | 234 | 35 | 504 | 707 | 1.1 | 4 | 0 |

The Custodians rose nine wins and stand the most Reactors, about nine a game; the Prospectors' Energy
rose by half (575 a game to 866) and they fell nine. Launches 12, from 24. Reported, not tuned.

## The computer chases Fissionables

Ticket [#643](https://github.com/whaleyjoshua2/Dying-Earth/issues/643), spec §4. The designer's aside on
the Fissionables ticket, charted then: *"we need to teach the computer to chase this resource but that
might be better after its uses are built out."*

Decided in two rounds; Q5 and Q6 came back after a first sweep run on Q1 to Q4 alone.

**Built test first**, every test red on its missing names or its figure first. The buy test went red on
behaviour: the computer had spent its Ducats on Influence before the turn's end, where the Buy stood,
so the Buy moved to the head of the turn. Two were broken on purpose after passing: the site worth's
Tech gate and the Mine lift's reserve check.

**The review** found eight things. The worst: the sale kept two tanks where the nuclear build gate wants
a fill and two more, so a seat sold the Fissionables it was saving for a hull. The keep now covers the
gate, and every Reactor standing, mothballed or on order, which a restart had been selling out from
under itself. A Trade asked for the same shortfall the Buy had paid for, and crowded out every other
Trade while the seat could not pay; a partner gave away its own reserve; the Buy was all or nothing;
the sale had no price floor; and units offered in a Trade could be sold the same turn. Each is mended,
two with tests that went red first.

**The sweep** ([`sweeps/ticket-643.txt`](sweeps/ticket-643.txt)): **22 / 56 / 1 / 1 and 0 collapses**,
from the Reactor's 28 / 47 / 1 / 1 and 3. By Faction, over the 80 games:

| | Fissionables a game, net | Bought | Sold | Reactors standing at the end | Nuclear hulls built |
|---|---|---|---|---|---|
| Custodians | 97.4 | 43 | 3,776 | 705 | 164 |
| Prospectors | 55.8 | 110 | 1,144 | 490 | 210 |
| Arkwrights | 5.7 | 39 | 45 | 96 | 19 |
| Archivists | 2.0 | 2 | 25 | 1 | 0 |

The Custodians mine the most and sell most of it; the Prospectors build the most nuclear hulls (181 to
210 once the keep stopped selling their savings) and won nine more. The Archivists stand one Reactor in
eighty games. Reported, not tuned.
