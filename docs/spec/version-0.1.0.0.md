# Dying Earth — version 0.1.0.0, the more-room version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.0](https://github.com/whaleyjoshua2/Dying-Earth/issues/500), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-07-version-0.1.0.0/`](../dev-diary/2026-10-07-version-0.1.0.0/).

**What the version is.** Version 0.09.9 with the designer's list. **More room**: Mercury, Ceres and
Vesta join the board (§1).

**What it did to the win column** (80 games, per Faction):

| | 0.09.9 | the new worlds (§1) | §1, the computer kept off them (a control) | the ideal |
|---|---|---|---|---|
| Custodians | 2 | 1 | 3 | 15 |
| Prospectors | 23 | 15 | 22 | 15 |
| Arkwrights | 30 | 52 | 32 | 15 |
| Archivists | 2 | 2 | 4 | 15 |
| collapses | 20 | 8 | 18 | 20 |

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

- **The cap** is the window's turns and four more, or nine where that is less, the cap Mars and
  Venus keep.
- **Mars–Vesta is 9 turns, not the 8 the decision's summary gave**: 494.5 days is 8.2 turns of
  sixty, and every flight in the game rounds up.
- **A turn is read at a point every thirty degrees the phase angle moves in it**, so a window
  passed inside the turn is found. Mercury laps Earth in under two turns, its phase angle sweeping
  more than half the circle in one; read at its two ends alone, its window would never be found.
  The Mars and Venus crossings, which move under thirty-eight degrees a turn, read as before.

**Window-free crossings** take the same flight whenever flown, the gulf's own `turns`:
**Ceres–Vesta 9** (the designer's figure; its real window comes once in seventeen years),
Mercury–Ceres 7, Mercury–Vesta 5, Venus–Ceres 8, Venus–Vesta 6 (Hohmann, rounded up).

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
