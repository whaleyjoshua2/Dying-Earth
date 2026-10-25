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
