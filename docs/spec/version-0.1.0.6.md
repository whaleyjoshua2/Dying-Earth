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
Archivists' own building renamed the Accumulator (§3); and the computer is taught to chase them (§4). Beside it, Colony Ships run standing ferry routes (§5), two Factions' figures and the Prospectors' bar move (§6), the first child born off Earth is a Moment (§7), the Refugee Convoy may be sent to the stars (§8), and a Colony's card names its threats (§9).

**What it did to the win column** (80 games, per Faction; every section's own sweep is in it):

| | 0.1.0.5 | §1 Fissionables | §2 the nuclear hull | §3 the Reactor | §4 the chase | §5 ferry routes | §6 the figures | §7 the first-born | §8 the convoy | §9 threats | the ideal |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Custodians | 18 | 23 | 19 | 28 | 22 | 22 | 30 | 31 | 30 | 30 | 15 |
| Prospectors | 61 | 51 | 56 | 47 | 56 | 55 | 46 | 45 | 46 | 46 | 15 |
| Arkwrights | 0 | 3 | 2 | 1 | 1 | 2 | 1 | 1 | 1 | 1 | 15 |
| Archivists | 1 | 1 | 1 | 1 | 1 | 1 | 3 | 3 | 3 | 3 | 15 |
| collapses | 0 | 2 | 2 | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 20 |

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

## 4. The computer chases Fissionables

Ticket: [The computer chases Fissionables](https://github.com/whaleyjoshua2/Dying-Earth/issues/643).

**The site** (Q1). A site's Fissionables count toward a computer seat's worth of it, at the Mine yield's
weight, whatever its Victory, once Clean Power or Nuclear Rockets stands (`ai_fission_worth`); nought
before, when nothing spends them.

**The reserve** (`ai_fission_reserve`): `reactor_reserve_turns` (10) of its running Reactors, each
nuclear hull's second tank topped up, and half a Fissionable for each Missile Carrier with no Warhead.

- **The Mine** (Q2). While the seat holds less than its reserve, a Mine on a site that holds them is
  weighed up by one plus the site's figure (`ai_fission_mine_lift`); bounded by the figure (1.8 at most)
  and off once the reserve is met.
- **The buy** (Q3). The shortfall is bought at the HEAD of the turn's orders, as far as the Ducats go:
  left to the turn's end, the Ducats had gone on Influence and no Buy was ever made.
- **The sale** (Q4). What stands beyond the **keep** is sold at the turn's end, at the band's midpoint
  or better (`ai_fission_keep`, `fission_keep_turns` 20): twenty turns of every Reactor standing or on
  order, mothballed or not, and once Nuclear Rockets stands the nuclear hull's build gate -- the
  largest hull's fill and two tanks more. The designer's "two tanks" is read as two beyond the fill, as
  §2's build gate reads; at two in all, the sale sold what the gate was saving. Never the units a Trade
  offered the same turn.
- **The Trade** (Q5). Fissionables join its trade goods. It asks for them only where the head-of-turn
  Buy cannot pay for one and no other good is short, asked as the shortfall; it offers them only from
  beyond its keep; and a computer partner never trades away its own reserve.

**Unchanged** (Q6): no plan to settle the far worlds for their yield, no change to the Tech pick lists.

**The measure** (Q7): Fissionables bought and sold at the market by seat, in the sweep.

**Measured** ([`sweeps/ticket-643.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-643.txt)):
**22 / 56 / 1 / 1 and 0 collapses**, from §3's 28 / 47 / 1 / 1 and 3. Over the 80 games, by Faction,
bought / sold: the Custodians 43 / 3,776, the Prospectors 110 / 1,144, the Arkwrights 39 / 45, the
Archivists 2 / 25. Nuclear hulls: the Prospectors 210, the Custodians 164, the Arkwrights 19, the
Archivists none. Reported, not tuned.

## 5. Standing ferry routes for Colony Ships

Ticket: [Standing ferry routes for Colony Ships](https://github.com/whaleyjoshua2/Dying-Earth/issues/631).

A Colony Ship given a **ferry route** carries people from its home end to its far end and comes back,
turn after turn (`engine/src/route.rs`).

- **The ends** (Q1, Q4). The **home end** is the orbit the Ship stands in when the route is set:
  over Earth, low orbit or a station's ring of the seat's own; elsewhere, an orbit that touches a
  Colony or station of the seat's. The **far end** is a Colony or station of the seat's, picked on
  the Ship card. A free slot is not a far end.
- **Loading** (Q2, Q3). Over Earth it lifts Pioneers from every Region that reaches its orbit, the
  most waiting first, until full. At a Colony or station it takes only the Colonists beyond one for
  each Module built there, the Core included; a Module under way counts nothing.
- **Leaving** (Q5). It leaves home full, or once it has waited `ferry.wait_turns` (3) turns with
  anyone aboard.
- **Unloading** (Q4). At the far end it puts down as many as there is room for. With people still
  aboard and no room, it waits there.
- **Refuelling** (Q6). At each end that is a depot it fills both tanks before it leaves, as an
  ordinary Refuel.
- **The window** (Q13, Q14). With **Wait for window**, ticked by default, a leg leaves only where its
  Fuel is at most `ferry.window_slack` (0.25) over the window's. Each leg is judged on its own window,
  and inside one system there is none.
- **Never stranded** (Q8). Toward an end that fills no tank, it leaves only with the leg there and
  the leg back aboard.
- **The waits** (Q8). It waits where it is for nobody to load, no room, short of Fuel, or the window,
  and the Report names each wait once to the player who runs it.
- **The gate** (Q7). A route needs a depot at one end at least. With one depot alone the round trip
  must fit the tank, and with the depot at the far end the first leg must fit the tank as it stands.
  With both, one leg must fit. Each leg is priced on the turn it would leave.
- **The card** (Q7). *Round trip 34.7 Fuel, tank 32; refuels at both ends; leaves in 5 turns*,
  Fissionables beside it for a nuclear hull, and the reason in words where the route cannot be set.
- **The orders** (Q9). Each turn's step is placed at the head of the turn as ordinary orders, as Max
  places the Allotment; cancelling one skips that turn alone. **End route** ends it, and it ends when
  an end or the Ship is lost.
- **Showing it** (Q10). *Ferry: Earth ↔ Olympus Mons on Mars* on the card and the roster row (a routed
  Ship wants no order of its own); a dashed line between the two Bodies in the solar view; *TSV
  Endeavour ferried 4 Colonists to …* in the Report.
- **The computer** (Q11, Q15, Q16). When it sends a full Colony Ship to a Colony or station of its own
  it has delivered to before, it sets a route there, waiting for windows. From the review: not to a
  ground Colony on a Body with a free slot, where the flight may be a founding, which stays with the
  planner. It ends a route whose far end has had no room for `ferry_idle_turns` (3) turns. A routed
  Ship is no runner in the planner's races.
- **The text tool** (Q12). `route <ship> <colony> [now]` and `route <ship> off`; each turn's step is
  placed after the player's own lines.
- **The measure** (Q17). Routes set, Colonists ferried and deliveries, by seat.

**Measured** ([`sweeps/ticket-631.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-631.txt)):
**22 / 55 / 2 / 1 and 0 collapses**, from §4's 22 / 56 / 1 / 1 and 0. Over the 80 games, routes set /
Colonists ferried / deliveries: the Custodians 1 / 6 / 1, the Prospectors 8 / 29 / 5, the Arkwrights
7 / 12 / 1, the Archivists 18 / 132 / 23. Reported, not tuned.

## 6. The Faction figures: the Arkwrights, the Archivists and the Fund

Ticket: [The Faction figures](https://github.com/whaleyjoshua2/Dying-Earth/issues/638).

| | was | now |
|---|---|---|
| Arkwrights' transit Fuel | x0.75 | **x0.8** |
| Arkwrights' Habitat capacity | x1.5 | **x1.25** (5, 10 with Expanded Habitats) |
| Arkwrights' Colony Ship capacity | x1.5 | **x1.25** (5, 7 with Expanded Habitats, 10 with Generation Ships too) |
| Arkwrights' lift population | x1.5 | **gone**: a Pioneer of theirs costs the ordinary population |
| Arkwrights' station Materials | x0.5 | **x0.75** (33 Materials) |
| Arkwrights' opening Fuel | 20 | **50** (`start_fuel`, a new per-Faction figure) |
| Archivists' opening Ducats | 10 | **25** |
| Prospectors' Fund bar | 6,250 | **6,500**, the computer's pace x6500/6250: 867 / 2,167 / 3,900 / 6,500 at turns 12 / 25 / 37 / 47 |

Q1: the designer's second line names the Archivists but its figures are all the Arkwrights', and are
read as theirs. Q2: the 50 Fuel is theirs too. Their recruiting, three a Region, is unchanged.

**Consequences, not decisions.** The Exodus Call suspended Coach Class's extra population charge; with
the charge gone it suspends nothing, and its hover no longer says it does. The Arkwrights' card shows
the 50 Fuel in its opening goods line and no longer lists a lift population multiplier.

**Measured** ([`sweeps/ticket-638.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-638.txt)):
**30 / 46 / 1 / 3 and 0 collapses**, from §5's 22 / 55 / 2 / 1 and 0. Reported, not tuned.

## 7. The first child born off Earth

Ticket: [The first humans born off Earth](https://github.com/whaleyjoshua2/Dying-Earth/issues/633).

- **Where** (Q1). The first Colonist ever grown by a Colony's growth (the Arcology's raised rate
  included) at a Colony **beyond Earth orbit**, the line the Victory count draws: off Earth and not
  a station in Earth's near orbits, so Earth's L4 and L5 count, Antarctica and the stations over
  Earth do not. A neutral Colony's births count for nobody and take nothing.
- **A tie** (Q2). Every seat whose Colony there grows on that turn is first.
- **The reward** (Q3). `climate.first_born_influence` (5) Influence into the Allotment the seat is
  about to spend: the growth runs in the Climate phase, after the Income has refilled it. Named
  second among the Allotment's parts, *First child born off Earth +5*, so the six-line hover shows it.
- **The words** (Q4, Q5). A Moment every player sees, a kind of its own in the Moments corner:
  *Yours: the first child born off Earth, at Mare Tranquillitatis on the Moon.*, *+5 Influence*; a
  rival's name where it is theirs. And a Report line in the same words. (The ticket's draft read
  *at Tycho: the Prospectors'*; the Report's templates never write "the {faction}", so the name leads.)
- **The computer** (Q6, Q7, Q8). No weight, except the computer Arkwrights: until somebody has it, a
  Habitat at, and a landing into, a Colony of their own beyond Earth orbit weigh
  `first_birth_chase` (1.25).
- **The measure**: games with the first birth, by seat.

**Measured** ([`sweeps/ticket-633.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-633.txt)):
**31 / 45 / 1 / 3 and 0 collapses**, from §6's 30 / 46 / 1 / 3 and 0. The first child born off Earth,
in games of 80: the Custodians 29, the Prospectors 22, the Arkwrights 16, the Archivists 13. Reported,
not tuned.

## 8. The Refugee Convoy's third answer: Send them to the stars

Ticket: [The Refugee Convoy's third answer](https://github.com/whaleyjoshua2/Dying-Earth/issues/632).

- **The convoy** (Q1, Q2). Six people on every answer. *Take them in*: +6 population in the most
  populous Region, +1 Unrest there, 0.5 ppm next turn. *Turn them away*: -5 Standing everywhere held,
  unchanged.
- **Send them to the stars** (Q3). A choice card may now carry a **third answer** (`third`,
  `third_does`, `third_when` in `events.toml`; the loader refuses a half-made one). The convoy's is
  `to_the_stars`: the six become Pioneers at a Launch Site of the seat's -- a Region with a working
  Spaceport first, then the most populous -- and are lifted **when the answer is given** into the
  room over Earth: its Colony Ships at Earth, then its stations' Habitats in Earth's near orbits.
  No Region pays population for them. It is a launch (when anyone is lifted) and the Spaceport's,
  as a lift is. With no working Launch Site the answer is shut, with the reason.
- **The lost** (Q4). Each who does not fit is lost, +0.5 Unrest in the Launch Site's Region, landing
  with the Resolution's card Unrest; the Report names the lifted, the lost and the Spaceport's pay.
- **The button** (Q5). *Send them to the stars (room for 2 of 6)*, and what it does beneath.
- **The computer** (Q6). It sends them when the room holds all six (`stars_room_at_least`, read
  before `take_when`); otherwise it answers as before.
- **The text tool**: `answer third`.
- **The measure**: convoys sent to the stars and refugees lost, by seat.

**Measured** ([`sweeps/ticket-632.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-632.txt)):
**30 / 46 / 1 / 3 and 0 collapses**, from §7's 31 / 45 / 1 / 3 and 0. Convoys sent to the stars over
the 80 games: the Arkwrights 12, the Prospectors 7, the Custodians 5, the Archivists 5; none lost.
Reported, not tuned.

## 9. A Colony card's threats line

Ticket: [A Colony card's threats line](https://github.com/whaleyjoshua2/Dying-Earth/issues/634).

- **What counts** (Q1). Rival warships in any orbit of the Colony's Body; rival Armies aboard
  Carriers there; rival warships and loaded Carriers bound for the Body, by the turns to go; a rival's
  Standing at the Colony at half its price or more (`influence_needed_for`, the figure the Standing
  chips' hover gives). Never an Accord partner under non-aggression, never a Ship that fled a Battle,
  never a Ship the reader cannot see.
- **For whom.** The Colony's holder -- the seat that formally controls it, so an Occupied Colony's
  threats are its holder's. The card shows the line on the player's own places, and on every seat's
  when spectating.
- **When and how** (Q2, Q3). Only when something threatens: *Threats: 2 Prospector warships in low
  orbit; a Prospector warship and an Army arriving in 2 turns; Prospectors at 14 of 30 Standing.*
  Each hull on the hover, a Carrier by the Army aboard, within six lines.
- **Order** (Q4). The orbit that touches the place first -- a station's own ring, a ground Colony's
  low orbit -- then the Body's other orbits.
- **The text tool and the roster** (Q5, Q6). The line under the Colony in `show`; *(threatened)* on
  a threatened place's roster row.

**Measured** ([`sweeps/ticket-634.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-634.txt)):
**30 / 46 / 1 / 3 and 0 collapses**, the same board as §8's, figure for figure: the line changes what
a player reads, not what any seat does.

## 10. A new Army joins its stack's stance

Ticket: [A new Army joins its stack's stance](https://github.com/whaleyjoshua2/Dying-Earth/issues/635).
The playtester's note: newly built Armies sat idle rather than reinforcing the stack that was there.
Every Army had been raised on Hold, and the card's stance row shows the first Army's, so a dug-in
stack's newcomer fought without the +2 while the row read *Dig In*. (The ticket first asked for
Barracks that raise and repair on their own; the designer: not the point.)

- **The rule** (Q1). A new Army takes the stance of its side's Armies at its place; Hold only where
  none stand. Every raise: a Region's or a Colony's, ordered or a card's, a Standing Army's return.
- **Dug in at once** (Q2), with Dig In's restraint: no march or boarding until the stance is changed
  and the turn has passed.
- **A mixed stack** (Q3). The stance most of it holds; on a tie the one the earliest raised of the
  tied holds, usually the one the card shows. Armies standing down under Occupation are no part of it, as the card reads it.
- **New Armies only** (Q4). An Army marching or landing in keeps the stance it was given.
- **No Report line** (Q5).
- **The computer** (from the review). A computer seat orders Dig In at a home with an enemy near,
  every turn. A new Army there used to stand its first turn on Hold and could march with a stack;
  now it is dug in from birth and stays. Reported, not changed.

**Measured** ([`sweeps/ticket-635.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-635.txt)):
**30 / 47 / 0 / 3 and 0 collapses**, from §9's 30 / 46 / 1 / 3 and 0. One game moved: seed 5 with the
Prospectors seated first, the Arkwrights' only win there, now the Prospectors'. Reported, not tuned.

## 11. An inherited Army stays on Hold until its holder orders it

Ticket: [An inherited Army stays on Hold until its holder orders it](https://github.com/whaleyjoshua2/Dying-Earth/issues/636).
Tester 2: *"they attacked Kazakhstan without my order"*. China passed to them by Influence while its
Armies stood in Kazakhstan, the Prospectors'; 0.1.0.0's grace held them on Hold one turn, and the
rule that counts an Army on Hold in a rival's held Region as on Attack (ticket #320) did the rest.

- **The rule** (Q1). An inherited Army abroad is never counted on Attack by that rule until its
  holder orders it.
- **What ends it** (Q2 b). A march, a boarding, or an Attack or Evade order at its place. An ordered
  Hold or Dig In keeps the peace.
- **The squatter** (Q3 a). Accepted: a Region's holder cannot open a Battle in their own Region, so an
  Army left on Hold there stays until its own holder moves it.
- **Stranded** (Q4). The previous holder's Armies standing in a Region when it passes to another
  holder -- not a partner under Passage -- are kept on Hold the same way. A third seat's Army there
  keeps its stance, and an Occupation completed strands nobody.
- **Boarding** ends it where the Army boards, after the ground Battles, not at the order.
- **The reminder** (Q5). Each turn after the first, while it stands on Hold: *the 1st Chinese Army
  waits on Hold in Kazakhstan.*
- **No Relations cost** for standing there: none existed, though the ticket's draft supposed one.
- **The witness** (Q6). No save of tester 2's game was kept; a test builds the case instead.

**Measured** ([`sweeps/ticket-636.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-636.txt)):
**29 / 48 / 1 / 2 and 0 collapses**, from §10's 30 / 47 / 0 / 3 and 0. Fewer Battles in every seating:
37, 102, 38 and 26, from 42, 110, 39 and 27. Reported, not tuned. Not measured: how often a computer
seat's inherited Army now waits abroad for good -- the review reads `ai.rs` as never marching an
Army home.

## 12. Scrapping a Colony Ship into the Colony below

Ticket: [Scrapping a Ship into the Colony below](https://github.com/whaleyjoshua2/Dying-Earth/issues/637).
The designer: stranded Ships (tester 3's at Mercury, 0.6 Fuel) should scrap into the Colony below.

- **Which** (Q1, Q5). An empty Colony Ship of the seat's -- no Colonists, no Army -- in the orbit that
  touches a Colony or station of its own (low orbit over a ground Colony, a station's ring), stranded
  or not, never in transit. Other hulls are never scrapped.
- **The yield** (Q2, Q3). A free Habitat at that Colony: no Materials, no Widgets, standing at the
  Resolution. The Habitat's own rules apply, a free slot among them (Q8: *full ...* otherwise).
- **The tank** (Q4). Lost, Fuel and Fissionables both.
- **Several Colonies under one orbit** (Q9). The player chooses: one button per Colony.
- **The words** (Q6). *Scrap for a Habitat at Tycho*; hover *Breaks the hull up into a Habitat at
  Tycho (+4 Colonists' room). Lands this turn.* Text tool: `scrap <ship> [colony]`.
- **The computer** (Q7). Scraps an empty Colony Ship stranded `scrap_stranded_turns` (3) turns or more
  over a Colony of its own, at the first with room.
- **Beside other orders** (from the review). A Scrap and any other order on the same hull -- a
  move, a load, a refuel, a route -- refuse each other.
- **When.** It lands as the orders are committed, before the turn's Battles. Left for the designer.

**Measured** ([`sweeps/ticket-637.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-637.txt)):
**32 / 45 / 1 / 2 and 0 collapses**, from §11's 29 / 48 / 1 / 2 and 0. Colony Ships scrapped over the
80 games: the Prospectors 65, the Archivists 36, the Custodians 30, the Arkwrights 22. Ships stranded
at the end: 109, from 244. Reported, not tuned.
