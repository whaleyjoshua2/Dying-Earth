# Dying Earth — version 0.1.0.6, the fission version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/627), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-10-25-version-0.1.0.6/`](../dev-diary/2026-10-25-version-0.1.0.6/).

**The baseline** is 0.1.0.5's closing board,
[`final-0.1.0.5.txt`](../dev-diary/2026-10-24-version-0.1.0.5/sweeps/final-0.1.0.5.txt):
**18 / 61 / 0 / 1 and 0 collapses**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.5 with the designer's list of 2026-10-25. Its spine is a new
resource, **Fissionables**, found off Earth and richest in the far worlds (§1), which a nuclear hull
is built and flown with and a Warhead is loaded with (§2), and a new common Reactor burns, the
Archivists' own building renamed the Accumulator (§3).

**What it did to the win column** (80 games, per Faction; every section's own sweep is in it):

| | 0.1.0.5 | §1 Fissionables | §2 the nuclear hull | §3 the Reactor | the ideal |
|---|---|---|---|---|---|
| Custodians | 18 | 23 | 19 | 28 | 15 |
| Prospectors | 61 | 51 | 56 | 47 | 15 |
| Arkwrights | 0 | 3 | 2 | 1 | 15 |
| Archivists | 1 | 1 | 1 | 1 | 15 |
| collapses | 0 | 2 | 2 | 3 | 20 |

## 1. Fissionables, the seventh resource, and Deep Mining and Rich Seam moved

Ticket: [Fissionables: a new resource found off Earth, by Body and by slot](https://github.com/whaleyjoshua2/Dying-Earth/issues/628).

**The resource.** Fissionables are held in the Stockpile (a seventh figure, carried to the tenth as the
others are), bought, sold and traded between seats as Fuel is. None is found on Earth.

**What makes them: the Mine** (the designer: *"we're using the existing mine"*). A Mine makes them
beside its Materials, `fissionables = 1.0` on its card in `modules.toml`, times:

- **its site's figure**, a fifth slot yield drawn at the start within `slot_yield_spread` (0.25) of
  its Body's `fissionables_yield` in `bodies.toml`: Earth 0, the Moon 0.25, Mars 0.80, Phobos and
  Deimos 1.1, Venus 0 (no ground), Mercury 1.3, **Ceres 1.6** (the list's "Eris", which the game does
  not have; the designer: *"yeah Ceres"*), Vesta 1.8;
- **the Faction's output multiplier**;
- **the Mine's Techs** -- Deep Mining, the Extraction Charter and Beneficiation, in the Mine's one
  chain (the designer: *"the techs will effect fisionables but not the card"*).

Nothing else reaches them: **no Discovery** (Rich Seam, the Deep Core Survey), no Mass Driver, and,
from the review, **neither Production Moved's factor nor a shut Habitat's halving**, which the first
build had let reach them though the designer named neither; they lift and halve the Materials alone.
A starved Colony's Mine makes none. A **station's**
Mine makes none: they are in the ground. The figure is to the tenth.

**Which sites hold them.** Every ground site of a Body with a figure, save **Hellas Planitia** on Mars
(`no_fissionables = true`; the designer: *"pick one mars slot to not have it"*), and on **the Moon two
or three of its five**, drawn with the game (`fissionables_sites = [2, 3]`; *"never more never less"*).
The draw runs on **dice of its own**, seeded from the game's seed (`FISSIONABLES_DICE`), so the game's
own dice -- every yield the version before drew and every roll after -- fall as they did.

**Deep Mining x1.25**, from x1.5, and **Rich Seam x1.5, x2 with Deep Mining**, from x2 and x3, at the
designer's word (*"Deep Mining should be 1.25 and rich seam 1.5/2"*). Rich Seam reads figures of its
own (`rich_seam_multiplier`, `rich_seam_multiplier_with_tech` in `events.toml`); the Ice Deposit and
the Helium-3 Vein keep the shared x2 and x3. Its card and the Tech's line say *Materials*.

**The market.** A fourth row, **8 Ducats** the midpoint of its band (`per_fissionables` in
`factions.toml`), selling back at half as Materials and Fuel do. The Market's arrays grew to four rows;
a save's three-row arrays read with the fourth at nought.

**Where it shows.** The top bar beside Fuel, its hover the Fuel's (each source last Income); the
radioactive trefoil in **lime** (140, 235, 60), the designer's **1A** off a sheet of six glyphs in four
colours, 48.6 from Energy and 49.9 from Ducats in CIELAB, its nearest. A site's yields line and its map
label carry the fifth figure where it has one and nothing where it does not; a Body's line reads its
card. A Colony's Output row, a Mine's figures, the Trading window's fifth line, a Trade's goods and a
rival's Income row on the Factions window carry it. The text tool prints it in the Stockpile and the
Market, as **Fi** in the slot lines and the Last income (**F** is the Fuel's), and takes it in `buy`,
`sell` and `trade`.

**The computer.** It reads every figure from the tables; it builds Mines as it did, so its Mines make
Fissionables where they stand. It does not chase them, and spends none: that is
[The computer chases Fissionables](https://github.com/whaleyjoshua2/Dying-Earth/issues/643), once
what they buy is built.

**Saves.** `SAVE_VERSION` stays 12. A save from before holds none in any Stockpile or slot and three
market rows; it loads, and the load draws the sites the seed would have drawn, on the same dice.

**Measured** ([`sweeps/ticket-628.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-628.txt)):
**23 / 51 / 3 / 1 and 2 collapses**, from 18 / 61 / 0 / 1 and 0. Deep Mining's cut took a fifth off the
Materials (the Prospectors 2,618 to 2,046 a game, the Custodians 2,135 to 1,781), and the Prospectors
fell 10 wins. Fissionables made, a game, net: the Custodians 56.5, the Prospectors 36.9, the
Arkwrights 2.5, the Archivists 2.2. (The first build's sweep read the Custodians at 111.7, Production
Moved doubling their Mars Mines' Fissionables; the win column was the same to the game.)

## 2. What Fissionables buy: the nuclear hull, its second tank, and the Warhead

Ticket: [What Fissionables buy: Ships after Nuclear Rockets, their second fuel, and the nuke](https://github.com/whaleyjoshua2/Dying-Earth/issues/629).

**The nuclear hull** (the designer's Q1 b). Once Nuclear Rockets stands, the yard offers each Ship as
a chemical hull, as before, or a **nuclear hull** (`Order::BuildShip { nuclear }`, a saved
`Ship::nuclear`). **Nuclear Rockets' x0.8 reaches a nuclear hull alone**: a chemical hull, and every
quote made for no hull in particular -- the seat's, the table's, the launch-window hover, the Report's
window line -- flies at the old speed (`journey_cost_hull`). Hulls built before the Tech are chemical.

**Its second tank** holds Fissionables, `[nuclear_hull] tank_share` (0.1) of its Fuel tank, so the Fuel
tank's Techs grow it too: a Colony Ship's 3.2, a warship's 3. **The build pays to fill it**, as it pays
the Fuel (the designer: *"as with fuel the cost is the tank so that cost matches tank increases
w/tech"*), and the hull is built holding what was paid; a cancel refunds it.

**Every leg burns a tenth of its Fuel in Fissionables** from that tank, beside the Fuel (*"both
apply"*): crossings and changes of orbit; never a Battle's charge. A leg the second tank cannot pay is
refused, *needs 2.1 Fissionables aboard, 1.5 left*; a nuclear hull that can pay neither its cheapest
leg's Fuel nor its Fissionables, with no depot, is **Stranded**. **Refuel fills both tanks** from the
Stockpile, as far as it pays (Q4 a).

**The Warhead** costs **half a Fissionable** (`[nuke] fissionables`), at the Missile Carrier's build,
which loads its first, and at every Rearm (Q5). A nuclear Missile Carrier pays both.

**The window.** Each Ship row of a yard gets a second button, **Nuclear**, its price on its face with
the Fissionables glyph, from the turn Nuclear Rockets stands; its hover says *Built full: 30 Fuel, 3
Fissionables*. A nuclear hull's card says *Nuclear frigate* and *30/30 Fuel, 1.5/3 Fissionables*, each
leg's heading names its Fissionables, and the roster says *(nuclear frigate)*. The order list says
*Build nuclear Frigate at ...*. The text tool takes `build ship <colony> <kind> nuclear` and prints
`nuclear, Fi 1.5/3` on the hull's line.

**The computer** (Q6) builds a nuclear hull when its Stockpile holds the second tank's fill and
`nuclear_hull_reserve_tanks` (2) tanks more, `ai_builds_nuclear`, and a chemical one otherwise; its
Refuel tops up a nuclear hull's second tank as well. It does not buy Fissionables for a hull or a
Warhead: a computer seat with none builds no Missile Carrier. That is
[The computer chases Fissionables](https://github.com/whaleyjoshua2/Dying-Earth/issues/643).

**Saves.** `SAVE_VERSION` stays 12: a Ship, a Build and a Ship order from before read chemical with
an empty second tank.

**Measured** ([`sweeps/ticket-629.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-629.txt)):
**19 / 56 / 2 / 1 and 2 collapses**, from §1's 23 / 51 / 3 / 1 and 2. Nuclear hulls built over the 80
games: the Prospectors 176, the Custodians 95, the Arkwrights 5, the Archivists 2. Launches 24, from
18. Fissionables made a game, net: the Custodians 57.3, the Prospectors 32.7, the Arkwrights 2.6, the
Archivists 2.1 -- the Prospectors built the most nuclear hulls on the fewer, not the more. Every hull
without them flies Nuclear Rockets' crossing no more. Reported, not tuned.

## 3. The Reactor, a common Fissionables power station, and the Archivists' Accumulator

Ticket: [The Reactor: a common Fissionables power plant on Earth and off it, and the Archivists' building renamed](https://github.com/whaleyjoshua2/Dying-Earth/issues/630).

**The Reactor** (Q1). A common Facility, every Faction's: **8 Energy a turn for 0.1 Fissionables and 2
Ducats, no Emissions**, at the Power Plant's price (27.5 Materials, 10 Widgets), one slot, no Energy
upkeep. **Behind Clean Power** (Q2, the designer: *"gate it clean power with the wind"*). Short of
Fissionables it runs at a quarter and burns none, as a Power Plant short of Fuel does: the Income pays
a building's Fuel and Fissionables from one reservation, and a building short of either is quartered;
the Report says *Fissionables ran short; the Reactor in China ran at a quarter.*

**The Reactor Module** (Q3): the same figures off Earth, at any Colony **or station**, with no sunlight
and no site yield, so it makes its 8 at Vesta and Ceres as at Mars.

**What lifts it** (Q4): Efficient Grids and Superconductors, as every power station; not Solar Maximum.

**The Accumulator** (Q5, the designer's pick of seven names). The Archivists' Unique Facility, the
Reactor until now, renamed; its rule untouched -- three quarters of the holder's whole Energy upkeep,
the Archive excepted, a second an ordinary power station. Its id is `accumulator`, and a save's
`reactor` loads as it (`#[serde(alias)]`); the common Reactor's id is `fission_reactor`. The
Archivists build the common Reactor as anyone does (Q6). The Income's line for its cut reads
*Accumulator relief*.

**The pictures** (Q7). The trefoil that was the Archivists' building's goes with the name to the
Reactor, Facility and Module. (I had told the designer it was a cooling tower; the sheet showed it is
the trefoil, and the cooling tower is the Power Plant's.) The Accumulator wears an energy tank,
provisional, off a sheet of nine for the designer:
[`ticket-630/accumulator-glyph-candidates.png`](../dev-diary/2026-10-25-version-0.1.0.6/ticket-630/accumulator-glyph-candidates.png).

**The computer** (Q8). `ai_feeds_reactors`: once Clean Power stands and its Stockpile holds
`reactor_reserve_turns` (10) turns of a Reactor's upkeep, one Fissionable, a Reactor takes the Power
Plant's and the Wind Farm's place on its build list, and the Reactor Module the Generator's and the
Wind Farm Module's; otherwise no Reactor is offered. The Archivists' first Accumulator keeps its own
reason, the upkeep cut. From the review: it counts a Reactor Module's 8 in its Energy, standing and on
order (`energy_production`), where it counted the Generator's alone; it mothballs only the Reactors its
Stockpile cannot feed this Income (`ai_reactors_to_mothball`), and restarts one only once the Stockpile
holds the reserve's turns for every running Reactor and that one (`ai_may_restart_reactor`); and it
weighs a Reactor as clean while it is losing the race to the Collapse, as the Wind Farm is.

**From the review, the rest.** The Shortfall forecast reads the Fissionables this turn's orders leave,
as it reads the Fuel; a Reactor that is dark, blockaded or knocked offline reserves and burns no
Fissionables, so the Stockpile never goes below nought.

**Saves.** `SAVE_VERSION` stays 12.

**Measured** ([`sweeps/ticket-630.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-630.txt)):
**28 / 47 / 1 / 1 and 3 collapses**, from §2's 19 / 56 / 2 / 1 and 2. Reactors standing at the end over
the 80 games: the Custodians 699, the Prospectors 431, the Arkwrights 53, the Archivists 4. (The
first build's sweep, before the review's mending, read 31 / 46 / 1 / 1 and 1, with 842 Custodian
Reactors: the computer had not counted its Reactor Modules' Energy and so built more.) Reported, not
tuned.
