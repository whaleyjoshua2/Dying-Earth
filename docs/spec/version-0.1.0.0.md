# Dying Earth — version 0.1.0.0, the more-room version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.0](https://github.com/whaleyjoshua2/Dying-Earth/issues/500), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-07-version-0.1.0.0/`](../dev-diary/2026-10-07-version-0.1.0.0/).

**What the version is.** Version 0.09.9 with the designer's list, and what the list led to. **More
room**: Mercury, Ceres and Vesta join the board (§1). **A fourth tier**, the Colonial Capital, one to
a Body (§2). **An Accord's Terms** on its hover (§3) and **Accords of five turns**, renewed or widened
(§4). **A Second Objective** for each Faction (§5). **The Tech tree's gates on a fourth rung** at 62,
with three new Techs (§6). **Two more Regions**, Vietnam and Argentina (§7). **Fifty turns**, the
climate and the Victory bars scaled, three new Choice Cards (§8). **The Archive at 20 Energy** and
Closed-Loop Colonies at a quarter off (§9). **Coach Class at half again** (§10). **The Arkwrights'
win waits on a Colonial Capital**, the computer chasing and gathering at one and every computer seat
keeping slots for Habitats, the climate re-swept to a step of 620 (§11). **The Solar System Map in
three dimensions**, each Body in its orbital plane (§12).

**What it did to the win column** (80 games, per Faction, after each ticket in build order):

| | 0.09.9 | §1 worlds | §2 Capital | §4 Accords | §5 Second | §6 tree | §7 Regions | §8 fifty turns | §9 Archive | §10 Coach Class | §11 closing | the ideal |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Custodians | 2 | 1 | 1 | 1 | 1 | 3 | 3 | 6 | 6 | 3 | **7** | 15 |
| Prospectors | 23 | 15 | 15 | 16 | 18 | 25 | 20 | 8 | 10 | 11 | **27** | 15 |
| Arkwrights | 30 | 52 | 52 | 49 | 49 | 31 | 39 | 35 | 32 | 30 | **17** | 15 |
| Archivists | 2 | 2 | 2 | 2 | 1 | 1 | 7 | 9 | 6 | 7 | **7** | 15 |
| collapses | 20 | 8 | 8 | 10 | 10 | 18 | 8 | 22 | 26 | 29 | **22** | 20 |

**The closing board** ([`sweeps/final-0.1.0.0.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/final-0.1.0.0.txt)):
**7 / 27 / 17 / 7 and 22 collapses.** The collapses meet the ideal; the Prospectors are the one
Faction over the line, the Custodians and Archivists under 10 in every climate cell of the version.
The Arkwrights' 17 are 6 outright with a Capital and 11 by the last turn's ranking. Balance work is
the next version's, at the designer's word. `SAVE_VERSION` is 12: a 0.09.9 save is refused.

The new worlds moved the column far past noise, away from the ideal. The control, the same build
with the computer forbidden to choose the three as destinations, is within noise of 0.09.9, so the
move is the computer **settling** them, not the dice they reshuffled. Mercury is settled in 78 of 80
games (median turn 20) and stands at the end with 302 Colonies over the batch; Vesta in 33 (80
Colonies); Ceres in none. The Moon falls from 241 Colonies to 175 and Deimos from 74 to 42. Why the
Arkwrights gain is **not traced**. The sweeps are
[`sweeps/ticket-502.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-502.txt) and
[`sweeps/ticket-502-control-worlds-closed.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-502-control-worlds-closed.txt).

## 1. Ceres, 4 Vesta and Mercury

The authority is [ticket #502](https://github.com/whaleyjoshua2/Dying-Earth/issues/502); the figures
are [`docs/research/new-worlds.md`](../research/new-worlds.md).

**Three more Bodies, nine in all**, each a system of its own for travel.

| | Mercury | Ceres | Vesta |
|---|---|---|---|
| Colony Slots | 6 | 6 | 6 |
| Orbital Slots | 2: Messenger, Mariner | 2: Piazzi, Dawn | 2: Olbers, Vestalia |
| Low gravity | no | yes | yes |
| First to a Body | 20 | 25 | 25 |
| Mine / Refinery / Generator / Research | 1.725 / 0.575 / 2.3 / 1.38 | 0.8625 / 2.0125 / 0.575 / 1.265 | 2.0125 / 0.575 / 0.575 / 1.15 |
| A Trade Post's pay for holding it | 6.9 | 8.05 | 8.05 |
| Sunlight, x Earth's | 6.67 | 0.13 | 0.18 |
| Leave / arrive, km/s | 1.17 / 1.17 | 0.11 / 0.11 | 0.08 / 0.08 |

- **The Colony Slots** are the six best known of the research's eight (§6 there), at their IAU
  Gazetteer positions: Mercury Caloris Planitia, Borealis Planitia, Prokofiev, Rachmaninoff,
  Raditladi, Rembrandt; Ceres Occator, Ahuna Mons, Ernutet, Oxo, Kerwan, Yalode; Vesta Rheasilvia,
  Veneneia, Divalia Fossae, Marcia, Arruntia, Bellicia.
- **The yields** are the research's reading of what each holds, times 1.15, the Trade Post's pay
  too. Each slot draws its own about the Body's figure as every slot does.
- **Sunlight** is the inverse square of the mean distance, with no ceiling or floor, so a Solar
  Array over Mercury makes about 6.7 times Earth's. The designer: room for fusion later.
- **No air** at any of the three: arriving costs what leaving does.

**Travel** is priced as every journey is: leaving one end, the gulf between the systems, arriving
at the other, at 4 Fuel a km/s.

| Gulf | km/s | Gulf | km/s | Gulf | km/s |
|---|---|---|---|---|---|
| Earth–Mercury | 8.75 | Mars–Mercury | 14.39 | Far orbit–Mercury | 13.94 |
| Earth–Ceres | 6.19 | Mars–Ceres | 3.47 | Far orbit–Ceres | 10.81 |
| Earth–Vesta | 5.48 | Mars–Vesta | 2.56 | Far orbit–Vesta | 9.69 |
| Venus–Mercury | 5.39 | Venus–Ceres | 9.48 | Ceres–Vesta | 0.96 |
| Venus–Vesta | 8.82 | Mercury–Ceres | 20.70 | Mercury–Vesta | 20.23 |

**Windows.** Seven crossings have a real window, each priced as Mars's is: at the window the
Hohmann flight in turns of sixty days, rounded up, and the delta-v's Fuel; off it, 1.5 days and
0.83% of the Fuel more a degree. A far orbit reads Earth's rows.

| Crossing | Flight at the window | Turns | Comes round every | Never longer than |
|---|---|---|---|---|
| Earth–Mercury | 105.5 days | 2 | 115.9 days | 9 turns |
| Earth–Ceres | 472.1 | 8 | 466.6 | 12 |
| Earth–Vesta | 397.9 | 7 | 504.2 | 11 |
| Mars–Ceres | 573.9 | 10 | 1161.5 | 14 |
| Mars–Vesta | 494.5 | 9 | 1425.9 | 13 |
| Venus–Mercury | 75.6 | 2 | 144.6 | 9 |
| Mars–Mercury | 170.5 | 3 | 100.9 | 9 |

- **The cap** is the window's turns and four more, and never under nine, the cap Mars and Venus
  keep.
- **Mars–Vesta is 9 turns, not the 8 the decision's summary gave**: 494.5 days is 8.2 turns of
  sixty, and every flight in the game rounds up.
- **A turn is read at a point every thirty degrees the phase angle moves in it**, so a window
  passed inside the turn is found. Mercury laps Earth in under two turns, its phase angle sweeping
  more than half the circle in one; read at its two ends alone, its window would never be found.
  The step is `window_sample_degrees`, 30, in `ephemeris.toml`. Mars's crossing is read at its two
  ends as before; Venus's at three points and Venus–Mars at four, which on an angle moving steadily
  through the turn gives the answer the two ends gave.

**Window-free crossings** take the same flight whenever flown, the gulf's own `days` made turns as
a window's flight is, so Nuclear Rockets shortens them too: **Ceres–Vesta 9 turns** (540 days, the
designer's figure; its real window comes once in seventeen years, 8 with Nuclear Rockets),
Mercury–Ceres 7 (361.7 days), Mercury–Vesta 5 (294), Venus–Ceres 8 (421), Venus–Vesta 6 (349.9),
Hohmann flights rounded up.

**The table refuses a missing gulf.** Every pair of systems must have its row, or a crossing would
be priced at nothing; the refusal names the pair. Every planet a Body is listed under must have its
`[[planet]]` row.

**The computer** weighs the three as it weighs Mars: the best free slot's yields and the first's
windfall, discounted by the share of the game the flight from Earth eats, and only where a tank can
pay the leg. **The Diaspora** counts them as Bodies.

**The art.** Dawn's Ceres and Vesta (USGS mosaics) and MESSENGER's enhanced-colour Mercury,
1024 x 512, made by `examples/prep_worlds.rs`, credited on the Credits screen:
"Ceres and Vesta: NASA/JPL-Caltech/UCLA/MPS/DLR/IDA." and "Mercury: NASA/Johns Hopkins University
Applied Physics Laboratory/Carnegie Institution of Washington." The Credits screen scrolls, its list
having run past an 800-pixel window before the maps were added.

**The Solar System Map.** Each stands at its true heliocentric longitude: Mercury on a ring inside
Venus's (1.25), Vesta (7.3) and Ceres (7.9) in a faint belt band (7.1 to 8.1). The Sun is drawn at
0.55 where it was 0.8 and Venus's ring moves from 1.7 to 2.2, so Mercury clears both. The camera
stands back a quarter and further right, so the belt is in the picture. Vesta's label hangs below its
disc. The window line a hover shows is drawn after every label.

**Saves.** `SAVE_VERSION` moves to **10**: a save of six Bodies is refused.

## 2. The Colonial Capital

The authority is [ticket #503](https://github.com/whaleyjoshua2/Dying-Earth/issues/503).

**A fourth tier**: 24 Module slots, open at 24 Colonists, for 70 Materials and 8 Widgets, raised
through the Widgets queue as every tier is. In `modules.toml` it is one more `[[tiers]]` entry with
three keys of its own: `one_per_body`, `ground_only` and `influence = 1`.

- **One to a Body, the first to complete it.** It holds it for good, through trade and capture. An
  upgrade to it under way anywhere else on that Body is refunded its Materials to the seat that
  paid, and the Report says so: *"Plato's upgrade refunded, 70 Materials: Tycho is the Moon's
  Capital."* A new order there is refused: *"The Moon's Capital is Tycho."*
- **Two completing in one turn** are parted as a contested slot is: the greater fleet in orbit of
  whoever holds each place now, lots only when level. The other is refunded.
- **Ground only.** A station stops at Colony; its Upgrade tile is gone.
- **+1 Influence a turn** to whoever holds it now, at face value, after the Faction multiplier,
  beside First to a Body's +1: a Body's first Colony that becomes its Capital pays +2.
- **Antarctica** may carry Earth's. Venus, with no ground, never has one.
- **The card** reads *"Tycho on the Moon · Colonial Capital"*, the dot before every tier's name.
  Where another place holds the Body's Capital the Upgrade tile is greyed, its hover *"The Moon's
  Capital is Plato."*, and a full Colony's warning reads *"Full"*, not *"upgrade for more room"*.
- **The computer** raises a place to it as to any tier, full and with the people, where the Body
  has none. It sets no Trade value on it.

**Measured** over the 80 games
([`sweeps/ticket-503.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-503.txt)): the
sweep is §1's line for line; **no place reached the Colonial Capital**, and 27 reached Colony.

## 3. An Accord's Terms on its hover

The authority is [ticket #508](https://github.com/whaleyjoshua2/Dying-Earth/issues/508).

On a rival's page in the Factions window, the line *"You hold an Accord with the Prospectors:
non-aggression, passage."* carries a hover with one line for each Term held, then the price of
breaking one:

> Non-aggression: no Influence or Battle on each other.
> Passage: Armies march in on Hold; Ships pass freely.
> Refuel: at each other's Space Stations.
> Research agreement: +10% Research to both.
> Acting against a term ends the Accord, at a cost.

- An Accord **declared over** keeps the hover until it lapses, its Terms binding until then.
- The **offer's checkboxes** say each Term in the same line; a greyed research agreement still
  says both sides must be Friendly.
- The **Relations grid** still names the Terms a pair holds without explaining them.

No rule moved, so no sweep.

## 4. Accords of five turns

The authority is [ticket #509](https://github.com/whaleyjoshua2/Dying-Earth/issues/509). The
designer's *"offered at five turn intervals"* means an Accord **runs five turns and may be renewed**.

- **Five turns**, the turn struck counted (`[relations] accord_turns = 5`). It lapses at the end of
  its fifth unless renewed.
- **Renewal.** In its last turn, a computer seat renews where it would accept the same Terms as a
  fresh offer. Two computer seats both willing renew it at once, for five more turns from the end of
  this run. The player is **not prompted**: the designer, after the build, found the prompt read as
  a new offer. Instead, the held Accord carries **Renew for five turns** in its last turn. The
  player's order is the acceptance; without it the Accord lapses quietly, with no offence and no
  refusal remembered. A rival that will not renew greys the button (*"they will not renew it"*), and
  the Report says *"Prospectors let your Accord lapse."*
- **A research agreement** renews only while both are Friendly; below that, the rest renews
  without it, and the Renew button's hover says *"The research agreement drops: you are no longer
  both Friendly."*
- **Offers** carry a research agreement between Friends, from the computer too.
- **Widening.** An Accord that stands may be offered the Terms it lacks: by the computer, or by
  the player with the checkboxes under the held Accord and **Offer to add**. Once accepted, they join
  it and the whole Accord starts five turns afresh. Offering a Term it already holds is refused:
  *"your Accord already holds non-aggression"*.
- **Kept** five turns, it raises Relations (`accord_kept_turns` 4 → 5), paid at the end of the
  fifth, counted from the strike through renewals.
- **Shown:** *"You hold an Accord with the Prospectors: non-aggression; 3 turns left."*, and the
  same turns in the Relations hover.
- **Unchanged:** Declare it over, breaking a Term, offer pacing, the 3-turn wait after a refusal.
- **In its last turn** nobody may offer it Terms, since it would lapse before the answer
  (*"your Accord lapses this turn: renew it first"*). Terms added and a renewal answered in the same
  turn keep every Term, and neither shortens the other's run. Terms offered to an Accord since gone
  are dropped unasked. The Report says *"Terms added to your Accord with the Prospectors."*
- **Saves.** `SAVE_VERSION` moves to **11**: a save of 10 has no Accord clock.

**Measured** over the 80 games
([`sweeps/ticket-509.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-509.txt)): wins
1 / 16 / 49 / 2 and 10 collapses, within noise of §2's. Accords struck rose from 601 to 644, and those
standing at the end fell from 87 to 76. Every one standing at the end is non-aggression alone:
Relations sit at a median of −4 to −7, so pairs are seldom Friendly.

## 5. The Second Objective

The authority is [ticket #504](https://github.com/whaleyjoshua2/Dying-Earth/issues/504).

Each Faction has a second Journal entry, the **Second Objective**, open from the Income after its
Opening Objective is met. It is met once, at an Income, with no deadline, rewarded once, and shown
below the first with its own *"Met, <date>"* or progress line. In `factions.toml` it is a `second`
card beside `opening`.

| Faction | Second Objective | Reward |
|---|---|---|
| Custodians | Leapfrog three Regions: three different Regions Leapfrogged after it opens. The Journal counts them (*"Not yet met: 2 of 3 Regions."*). | 25 Influence, this turn |
| Prospectors | Raise a Region to Industry Level 5: a Region held at level 5 or more, however it got there | 100 Ducats into the Venture Capital Fund |
| Arkwrights | Settle a Body beyond Earth's sphere: a Colony, ground or station, on any Body but Earth and the Moon | a free Colonist at that Colony, at one with room where there is one |
| Archivists | Have three Observatories working off Earth | 20 Research into the Archive fund |

The computer leans toward its Second Objective, as toward its first, once the first is met. The lean reads the orders, not their wording: a Leapfrog of a Region not yet counted, a raise, Colonists carried or put down beyond Earth and the Moon, an Observatory.

**Measured** over the 80 games
([`sweeps/ticket-504.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-504.txt)):
- Wins were 1 / 18 / 49 / 1 with 10 collapses, within noise of §4's.
- Second Objectives met: Custodians 67, Prospectors 78, Arkwrights 38, Archivists 22, of 80.
- The Arkwrights meet theirs in 38 of the 39 games where they meet their first. The Moon, not
  the far Body, is what holds them back.
- The Archivists meet theirs in none of the 20 games where they sit third.

## 6. The Tech tree: the gates on a fourth rung

The authority is [ticket #505](https://github.com/whaleyjoshua2/Dying-Earth/issues/505). The
designer: *"move victory gate techs to level 5 cost of 62 - two new level 4 techs move 2 level 3
techs - check math on victory gates"*, settled on the ticket as one Tech raised and three new, so
that every gate has a 58-Research Tech of its own branch under it; and, on seeing the picture,
*"there should only be four rungs"*. A rung is a column of the drawing; the price is each Tech's own.

- **Rung 3** keeps Hardened Hulls and Missile Technology at 48 and gains four Techs at 58: Clean
  Manufacturing, raised from 48, and three new, one to a branch, each the last step before its
  branch's gate.

  | Branch | Tech | Needs | Effect |
  |---|---|---|---|
  | Stewardship | Clean Manufacturing (raised) | Clean Power | unchanged |
  | Off-world Living | **Regolith Printing** | Closed-Loop Colonies | a Module built off Earth (any Body but Earth, or any Space Station) costs x0.75 Materials and x0.85 Widgets, inside the Faction's discount and Build Where You Dig, so the floor of half the row still holds; Antarctica pays the full price |
  | Extraction | **Rail Gun Freight** | Automated Refining | every working Mass Driver pays its holder 3 Ducats a turn, flat |
  | Society | **Superconductors** | Orbital Data Centers | every Energy output x1.25 -- Power Plant, Reactor, Generator, Solar Array, Heliostat -- in Efficient Grids' chain, so the two multiply |

- **The gates on rung 4, at 62**, each needing its branch's 58-Research Tech and the other rung-2
  Tech on its road (Green Consensus, Commodity Finance, Relay Networks, Civil Defense). The rung-2
  Tech under the 58 one is reached through it and no longer listed, by the no-redundant-prerequisites
  rule.
- **Propulsion**, which has no gate, ends on rung 3.
- **The arithmetic.** Every road is 18 + 32 + 32 + 58 + 62 = **202** Research, all four alike,
  from 130. The tree is 29 Techs and 1,093 Research, from 26 and 853. The drawing takes the fourth
  rung by itself; nothing else in the interface changed
  ([`ticket-505/tree.png`](../dev-diary/2026-10-07-version-0.1.0.0/ticket-505/tree.png)).
- **The computer.** Each seat's gate chain takes its 58-Research Tech by itself, and its pick list
  names it fourth, after its rung-2 pair; the three new Techs are on no rival's list. Hardened
  Hulls stays at the end of every list. The computer's appetite for a Mass Driver is unchanged.
- **Not retuned.** The length is fitted to this tree on the fifty-turn ticket.

**Measured** over the 80 games
([`sweeps/ticket-505.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-505.txt)),
against §5's; the rung numbers moved after the sweep and change no price, so it stands:
- Wins **3 / 25 / 31 / 1** with **18 collapses**, from 1 / 18 / 49 / 1 and 10. The Arkwrights
  lost 18 wins and the Prospectors gained 7; collapses nearly doubled. A real move, toward the ideal.
- Every gate completes later and in fewer games: Custodians 68 of 80 at a median turn 25 (71 at
  23), Prospectors 73 at 25 (78 at 22), Arkwrights 59 at 27 (69 at 24), Archivists 67 at 23 (73
  at 20). The whole tree completes in 50 of 80 at a median turn 27, from 65 at 24.
- Why the Arkwrights fall is **not traced**; their gate now lands three turns later in a game
  that still ends at 36.
- Rail Gun Freight completes in 78 of 80 games, but the computer seats build almost no Mass
  Driver (three standing at the end over the batch, one before), so in computer hands its Ducats
  are near-dead. Measured, not traced; the appetite is a carried note on the map.

## 7. Two more Regions: Vietnam and Argentina

The authority is [ticket #510](https://github.com/whaleyjoshua2/Dying-Earth/issues/510). The
designer: *"add two more nation states"*, and on the choice, *"let's go with Vietnam and
argentina"*. Twenty Regions, from eighteen.

| Region | Cut from | Covers | People | GDP | Influence | Industry | Education | Coast | Starts with |
|---|---|---|---|---|---|---|---|---|---|
| **Vietnam** | Indonesia's (680 to 430) | the South-East Asian mainland: Vietnam, Thailand, Myanmar, Cambodia, Laos | 250 | 1 of Indonesia's 3 | 1, added (Indonesia's card had only 1) | 2 | 0.9 | Exposure 2 | Factory, Mine, Power Plant |
| **Argentina** | Brazil's (450 to 223) | the Spanish-speaking mainland: Colombia and Venezuela to Chile and Argentina | 227 | 2 of Brazil's 5 | 1 of Brazil's 2 | 1 | 1.1 | Exposure 1 | Factory, Mine |

- **Shares by real population**, the world's 7,860 unchanged. Vietnam's Influence is the one figure
  added, as South Africa's was in 0.09.8; the world's Influence is 36.
- **Borders**, every edge on both cards: Vietnam touches Indonesia's Region, China's and (through
  Myanmar) India's; Indonesia's no longer touches India's. Argentina touches Brazil's and Mexico's
  (at Panama); Brazil's Region now touches Argentina's alone. Brazil keeps Guyana, Suriname and the
  Falklands (French Guiana is France's feature, so the European Union's, by the mask's standing
  rule). Indonesia's Region keeps China's as a sea neighbour, as Australia is its.
- **The flags** `vn` and `ar` are drawn for the game (a star on red; the sun on the bands, as a
  plain disc), not flag-icons' files.
- **No fixed start.** Seats pick their Region as they do today; the computer's pick (best Industry,
  then people) can land on either.
- **The map** is redrawn from Natural Earth's 50m borders (public domain, downloaded for the build,
  not committed), the two Regions' mask values 20 and 21; label points on the Khorat plateau and the
  pampas; flags `vn` and `ar`. `SAVE_VERSION` 12: a 0.1.0.0 save from before this ticket is refused,
  as every Region change has been.
- Nothing else changed: the Amazon Dieback still names Brazil's Region.

**Measured** over the 80 games
([`sweeps/ticket-510.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-510.txt)),
against §6's:
- Wins **3 / 20 / 39 / 7** with **8 collapses**, from 3 / 25 / 31 / 1 and 18. The Archivists' six
  and the collapses' ten are past noise; the Arkwrights' eight is at its edge.
- Every gate completes a little earlier and in more games (Arkwrights 66 of 80 at a median turn 27,
  from 59); the tree completes in 62 of 80, from 50.
- Why is **not traced**. Two more Regions to hold means more Research Labs and Facilities on the
  board for the same four seats; the fifty-turn ticket fits the length to this board.

## 8. Fifty turns

The authority is [ticket #506](https://github.com/whaleyjoshua2/Dying-Earth/issues/506). The
designer: *"50 turns scale co2 and victories"*. Built last, on the board the version ships.

- **Fifty turns** of two months (`victory.toml turns = 50`), January 2030 to **March 2038** (turn 36
  was November 2035; the ticket's "November 2038" was an arithmetic slip, corrected here).
- **The climate**, swept on the finished board ([`sweeps/co2-grid.md`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/co2-grid.md)):
  `ppm_step` **440**, from 330; the Sink stays 8. Nearest to the designer's 20 collapses (22).
- **The bars**, at the designer's word:

  | Faction | First part | Second part |
  |---|---|---|
  | Custodians | a Stabilization run of **4** (3) | **18** off Earth (12) |
  | Prospectors | **3,500** in the Fund (2,500) | **18** off Earth (12) |
  | Arkwrights | **42** off Earth (30) | **4 Bodies** at 4 each (3) |
  | Archivists | the Archive at **225** Research (150) | **18** uploaded (12) |

  The Archive's cost in `modules.toml` is the same 225, as it must be.
- **The computer** reads the length: its pace tables are stretched by 50/36 and end at the new bars
  (the Fund by turn 47, the Archive by 44, the Diaspora by 33, four Bodies by 47, 18 uploads by 47,
  18 off Earth by 31); it picks its gate from turn 33 (24) and wants its early Mines through turn 11 (8).
- **The deck:** off-Earth cards join on turn 15 (12); three new Choice Cards, built from effects the
  deck already had, 43 cards and 21 that ask:

  | Card | Take | Refuse | The computer takes it |
  |---|---|---|---|
  | **Carbon Levy** | +20 Ducats, 1 ppm off next turn's card emissions (which the Stabilization test does not count), +0.5 Unrest in every Region held | standing −3 in every Region held | under Unrest 3 |
  | **Deep Core Survey** | −30 Ducats; a Mine Discovery over the Body of the Colony of yours with the most Mines, x1.5 for four turns (a Faction with no Mine at any Colony is not asked) | nothing | with 60 Ducats or more |
  | **Trade Pact** | Relations +2 with every rival; Materials a Ducat cheaper for two turns; Factories x0.8 at the next Income | Relations −1 with every rival | always |

- **Neutral Regions** develop every 10 turns (9).
- **Two notes from the build.** A card side with no effect now reaches every seat (`side_reaches`
  in `events.rs`); until this the Deep Core Survey, whose refusal does nothing, was never asked.
  And the Trade Pact's "Trading x0.9" of the decision became **Materials a Ducat cheaper for two
  turns**, since the deck has no multiplier on the Market's prices, only a step; the step is the
  one Market's, so every seat buys cheaper while it holds.
- Everything else that reads the length reads `turns` and moved by itself: the dates, the Last Turn to
  Act, the projections, the computer's flight value, the sweep's checkpoint (turn 39, from 28). A
  fourth Mars window, spring 2037, is inside the game now.

**Measured** over the 80 games
([`sweeps/ticket-506.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-506.txt)), the
version's closing board:
- Wins **6 / 8 / 35 / 9** with **22 collapses**, against the ideal of 15 each and 20. The
  collapses are where the dial put them; the Arkwrights still take nearly half the games, as they
  did in every cell of the grid, and the win column is not retuned on this ticket.
- Every gate completes in 73 to 80 of 80 games at a median turn 25 to 27; the whole tree in 71 of
  80 at a median turn 28. 18 of 80 games are still running at turn 39.
- The grid's 440 cell read 6 / 8 / 37 / 7 before the Deep Core Survey was made askable (see
  the Notes); the closing sweep is the one that counts.

## 9. The Archive's Energy upkeep

The authority is [ticket #511](https://github.com/whaleyjoshua2/Dying-Earth/issues/511). The
designer: *"raise the energy requirement for the archive to 20"*, and on the Tech that halved it,
*"change closed loop colonies from a 50% discount to a 25% discount"*.

- **The Archive draws 20 Energy a turn** once complete (`modules.toml energy_upkeep = 20`, from
  12), still nothing until its Research is paid in full.
- **Closed-Loop Colonies** cuts every Module's Energy upkeep to **x0.75**, from x0.5
  (`techs.toml value = 0.75`), so a complete Archive draws 15 with it, where it drew 6; every
  other Module's upkeep rises with it (a Mine's 3 reads 2.3, not 1.5).
- Nothing else moves. The computer reads its Energy deficit and buys or builds as before.

**Measured** over the 80 games
([`sweeps/ticket-511.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-511.txt)),
against §8's closing sweep:
- Wins **6 / 10 / 32 / 6** with **26 collapses**, from 6 / 8 / 35 / 9 and 22: within noise on
  every count.
- The Archive stands complete in 53 of 80 games, from 51; the Archivists' gate completes in 72 of
  80 at a median turn 25, from 74. The dearer Archive does not stop the computer Archivists building
  it or keeping it lit.

## 10. Coach Class, half again

The authority is [ticket #513](https://github.com/whaleyjoshua2/Dying-Earth/issues/513). The
designer, on the Arkwrights winning 32 of 80: *"change their ships such that they get 1.5x the
capacity of everyone else"*, and the rule's other halves follow (*"A"*).

- **Colony Ships carry x1.5** (`colony_ship_capacity_multiplier = 1.5`, from 2.0): 6, 9 and 12 as
  Expanded Habitats and Generation Ships come, from 8, 12 and 16, rounded down.
- **Each Pioneer costs the Region x1.5 population** (`lift_population_multiplier`, from 2.0).
- **They recruit x1.5 a turn**, rounded down: 3 a Region, from 4 (`emigrants_multiplier`, from 2.0).
- The Colony Ship's price (x0.85) and the four Pioneers waiting on turn one are untouched.

**Measured** over the 80 games
([`sweeps/ticket-513.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-513.txt)),
against §9's:
- Wins **3 / 11 / 30 / 7** with **29 collapses**, from 6 / 10 / 32 / 6 and 26: within noise on
  every count. Half again instead of twice does not reach the Arkwrights' 30-odd wins in computer
  hands: their gate still lands at a median turn 27 in 75 of 80 games, and the win column is theirs
  by the same margin.
- Not traced here. The Arkwrights' edge, measured over the version, survives the climate dial, the
  Archive's price and now their load; what it rests on is a question for the balance work.

## 11. A Colonial Capital held

The authority is [ticket #514](https://github.com/whaleyjoshua2/Dying-Earth/issues/514). The
designer, with the Arkwrights at 30 of 80 after Coach Class: *"let's add a requirement to control at
least one colonial capital"*; theirs alone (Q1 A), as a gate on the win (Q3 A).

- **The Arkwrights' Victory Condition** reads *42 Colonists off Earth over four Bodies at 4 each,
  one of them holding a Colonial Capital of yours*. `factions.toml victory_capital = true` on their
  card; no other card asks it.
- **Held** when a Colonial Capital stands at a Colony the Arkwrights control (not Occupied: an
  Occupied Capital counts for neither side), on any Body but Earth. Lost with the Colony.
- **A gate on the win**, like the gate Tech: both parts accrue and score as before; `met` is false
  and the Victory window's held-back note reads *needs a Colonial Capital of yours, at a Colony you
  hold off Earth*. The gate Tech's note comes first when both hold.
- **The computer Arkwrights chase it** (`ai.toml capital_chase = 3.0`): holding none, their
  **Capital candidate** is their most peopled ground Colony off Earth on a Body with no Capital; its
  every upgrade and a disembark into it weigh three times. And once their Bodies part is within one
  of its bar they **gather** there: a Habitat at the candidate while its room is under the Capital's
  24, and a loaded Ship's crossing to its Body, weigh three times too. Measured before the chase: 5
  Capitals stood at the end of 80 games; with the chase but no gathering, 7, all in Antarctica.
- **The sweep** reports who held a Capital at the end, by Faction.

**Measured** over the 80 games
([`sweeps/ticket-514.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-514.txt)),
against §10's 3 / 11 / 30 / 7 and 29 collapses:
- Wins **4 / 12 / 1 / 7** with **56 collapses**. The Arkwrights fall from 30 wins to one, and the
  games they would have ended run on until the world collapses.
- **No Faction held a Capital at the end of any game.** Seven Capitals stood over the batch, every
  one in Antarctica, which is on Earth and counts for nothing. Off Earth no computer seat gathered
  24 Colonists at one ground Colony in 80 games, with the chase or without it (5 Capitals, all
  Antarctic, before the chase).
- **The gathering** (the designer's option 1) was built in three rounds and swept after each
  ([`sweeps/ticket-514-gathering.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-514-gathering.txt)
  is the last): Habitats and the crossing lifted, then every Module at the candidate lifted and a
  fresh Outpost beside it damped, then Habitats built ahead of need. **4 / 12 / 2 / 7 and 55
  collapses; no Capital held off Earth in 80 games.** Probed: the candidate's slots filled with
  producers while it held four, a deadlock the Habitat-ahead rule broke; after it the Arkwrights'
  best place reaches 24 to 28 people with every slot built, but only around turn 40 to 48, and the
  Capital upgrade is offered in 13 of 80 games, late, and never completes. The requirement is beyond
  the computer's reach in a game of this length as it plays it.
- **Slots kept for Habitats** (the designer: *"give the ais an understanding of the necessity to
  reserve spots for habitats"*): every computer seat's ground Colony off Earth keeps as many Module
  slots free as the Habitats its next tier's people still need (`habitats_to_reserve`), less any on
  order; no other Module takes one of them. A station, Antarctica and a place at its last tier
  reserve nothing, so nobody keeps room for the Moon's Capital once the Moon has one; and once the
  seat holds a Capital its places keep room only where the next step is a Capital on a Body without
  one, never for the lower tiers (the designer: *"once a colonial capital is raised the rest of the
  colonies are not still reserving spaces ... so long as it does not preclude them raising a second
  capital on another body"*). **Measured**
  ([`sweeps/ticket-514-reserve.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-514-reserve.txt)):
  **4 / 13 / 8 / 9 and 46 collapses**; the Arkwrights held a Capital off Earth at the end of **8 of
  80** games (none before), 13 Capitals on the board (7 before). The first rule that reaches the
  requirement; the Arkwrights stand at 8 wins against the 30 the requirement took away.
- **The climate re-swept** at the designer's word, the collapses having climbed to 46 with the games
  nobody ends: `ppm_step` **620**, from 440, the Sink still 8
  ([`sweeps/co2-grid.md`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/co2-grid.md), the second
  grid). At 620 the grid read 7 / 27 / 17 / 7 and 22 collapses; the closing sweep is
  [`sweeps/ticket-514-closing.txt`](../dev-diary/2026-10-07-version-0.1.0.0/sweeps/ticket-514-closing.txt).

## 12. The Solar System Map in three dimensions

The authority is [ticket #512](https://github.com/whaleyjoshua2/Dying-Earth/issues/512). The
designer: *"solar system map 3d with bodies in their correct orbital plain - no need to draw the
astroid belt"*.

- **Each ring lies in its Body's orbital plane**: tilted by the real inclination about the real
  ascending node, both from the ephemeris (`ephemeris::orbital_plane`; the rings read it at the
  game's first month and the Bodies at the turn's date, the nodes drifting by at most a degree and
  a half a century, so over the game a ring leaves its Body by three thousandths at Ceres). Mercury 7.0 degrees about a node
  at 48, Venus 3.4, Mars 1.85, Ceres 10.6, Vesta 7.1; Earth's ring is the ecliptic. The Body rides
  its tilted ring, so its height above the map is the ring's radius times the sine of its real
  ecliptic latitude (`Position.latitude`, new), within what its eccentric orbit adds; a test holds
  every Body to that over the fifty turns.
- **A satellite** (the Moon, Phobos, Deimos) circles its parent in the parent's plane; a Body's
  Orbital Control ring and its far orbits lie in that plane too.
- **The camera** opens where it always did, forty degrees above the ecliptic; dragging up and down
  tilts it between nearly edge-on (5 degrees) and nearly straight down (85), sideways turns it as
  before (`solar_pitch`; the shot aid `pitch:<degrees>`).
- **The belt** of ticket #502 is not drawn; Ceres and Vesta keep their rings. The window line and a
  flight's arc read the tilted places. The rings' radii are untouched, and the title screen's
  system, with no sky to read, is flat as before.
- No rule moves; no sweep.
