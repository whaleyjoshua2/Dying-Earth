# Version 0.1.0.0, the more-room version

The map is [Map: version 0.1.0.0](https://github.com/whaleyjoshua2/Dying-Earth/issues/500). The
standing baseline is 0.09.9's closing sweep: 2 / 23 / 30 / 2 (Custodians / Prospectors / Arkwrights
/ Archivists) and 20 collapses in 80 games.

## Ceres, 4 Vesta and Mercury (#502)

Three more Bodies, nine in all: six Colony Slots and two Orbital Slots each, Ceres and Vesta low
gravity, the research's yields times 1.15, sunlight left natural, every crossing on its real window
but Ceres to Vesta, nine turns whenever it is flown. Spec section 1.

**The pictures** ([`ticket-502/`](ticket-502/)).

- [`board-solar.png`](ticket-502/board-solar.png): turn 1, the pointer on Ceres. Mercury on its ring
  inside Venus's, the faint belt beyond Mars with Vesta and Ceres in it (in January 2030 both stand
  near Mars in the sky), Ceres's two station slots and its 25 Influence prize, and the window line:
  "Ceres window: in 3 turns (July 2030). Flight now: 11 turns, 75 Fuel. At the window: 8 turns, 38
  Fuel." The window line is drawn last; it was under Vesta's label in the first picture.
- [`board-mercury.png`](ticket-502/board-mercury.png), [`board-ceres.png`](ticket-502/board-ceres.png),
  [`board-vesta.png`](ticket-502/board-vesta.png): the three globes and their cards. Marcia's marker
  sits on Marcia's bright crater on Vesta's map.
- [`menu-credits.png`](ticket-502/menu-credits.png) and [`credits-tall.png`](ticket-502/credits-tall.png):
  the Credits screen at 1280 x 800, which now scrolls, and at 1280 x 1200 showing the Maps lines.

**The maps** came from USGS's 1024-pixel previews of the Dawn mosaics (Ceres, Vesta; Vesta's grey
stretched to about twice its brightness) and NASA's PIA17386 (Mercury, halves swapped to put 0 in
the middle), by `examples/prep_worlds.rs`. Checked: Occator, Marcia, Caloris and Rembrandt sit on
their Gazetteer pixels.

**The sweep** ([`ticket-502.txt`](sweeps/ticket-502.txt)): **1 / 15 / 52 / 2 and 8 collapses**,
from 2 / 23 / 30 / 2 and 20. Far past noise, away from the ideal.

- **Mercury** is settled in 78 of 80 games, median turn 20, and holds 302 Colonies at the end over
  the batch: two turns from Earth, its window open more turns than not, the best Generator yield on
  the board and a 20 prize. **Vesta** in 33 games (80 Colonies). **Ceres** in none, **not traced**:
  its leg is 8 turns and 38 Fuel at the window, against a Colony Ship's 40, and 75 on turn 1.
- The Moon falls from 241 Colonies to 175, Deimos from 74 to 42.
- **The control** ([`ticket-502-control-worlds-closed.txt`](sweeps/ticket-502-control-worlds-closed.txt)),
  the same build with the computer forbidden to choose the three as destinations: 3 / 22 / 32 / 4
  and 18, within noise of 0.09.9. So the move is the computer settling the new worlds, not the dice
  the extra slots reshuffled. Why the Arkwrights gain is **not traced**.

**Found on the way.** The sweep's per-Body tally read only the first six Bodies, so the first run
reported no Colony on any new world; fixed, and the batch rerun to the same win column.

## The Tech tree: the gates on a fourth rung (#505)

Four Techs at 58 on rung 3 (Clean Manufacturing raised; Regolith Printing, Rail Gun Freight and
Superconductors new) beside Propulsion's two at 48, the four Victory gates on a fourth rung at 62;
every road 202 Research from 130. Spec section 6.

**The picture** ([`ticket-505/`](ticket-505/)): [`tree.png`](ticket-505/tree.png), the Tech Tree
window on turn 1, the Custodians' seat. The third column holds the three new Techs and Clean
Manufacturing; the gates stand in a fourth, each boxed in its Faction's colour. The first picture
had the 58s in a column of their own and the gates in a fifth, which left a blank column through the
three gated branches; the designer: "there should only be four rungs".

**The sweep** ([`sweeps/ticket-505.txt`](sweeps/ticket-505.txt)): 3 / 25 / 31 / 1 and 18
collapses, from 1 / 18 / 49 / 1 and 10. Every gate lands two to three turns later; the tree
completes in 50 of 80 games, from 65.

## Two more Regions: Vietnam and Argentina (#510)

Vietnam (the South-East Asian mainland) cut out of Indonesia's Region, Argentina (the
Spanish-speaking mainland) out of Brazil's, each taking its parent's people, GDP and Influence by
real population; twenty Regions. Spec section 7.

**The pictures** ([`ticket-510/`](ticket-510/)).

- [`mask.png`](ticket-510/mask.png): the redrawn Region mask, every Region in its own colour, the
  generator's preview. Vietnam teal on the mainland, Argentina light blue down the west and south
  of South America; Brazil keeps Guyana and Suriname.
- [`vietnam.png`](ticket-510/vietnam.png): the Earth Map on turn 1 facing Vietnam, between India,
  China and Indonesia, with its label.
- [`argentina.png`](ticket-510/argentina.png): facing Argentina with its card open: the flag, 229
  million people, GDP 2, Industry Level 1 leaning Materials, a Factory and its Mine, five slots free.

**The sweep** ([`sweeps/ticket-510.txt`](sweeps/ticket-510.txt)): 3 / 20 / 39 / 7 and 8 collapses,
from 3 / 25 / 31 / 1 and 18. The Archivists' rise and the collapses' fall are past noise; not traced.

## Fifty turns (#506)

The game runs fifty turns, January 2030 to March 2038; `ppm_step` 440 from the climate grid; every
Victory bar that accumulates scaled, the computer's pace with it; three new Choice Cards; neutral
development every ten turns. Spec section 8.

**The sweeps.** [`sweeps/co2-grid.md`](sweeps/co2-grid.md) sums the eight climate cells by Faction
(raw: `ticket-506-grid.txt`, `ticket-506-grid-2.txt`); [`sweeps/ticket-506.txt`](sweeps/ticket-506.txt)
is the closing sweep at the cell taken.
