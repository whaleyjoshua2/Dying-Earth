# Version 0.06.0, the space version: the build diary

Pictures and measurements taken while the tickets of
[Map: version 0.06.0](https://github.com/whaleyjoshua2/Dying-Earth/issues/79) were built, on the
branch `version-0.06.0`. Every picture below was taken headlessly with the game's own `shot:` mode
(`dying-earth.exe shot:<prefix> ...`, the window off-screen) and opened before it was written about.
Every batch is twenty seeds of four-seat `simulate` through the engine's `sweep` example, its output
kept whole under `sweep/`.

## The Observatory ([ticket #80](https://github.com/whaleyjoshua2/Dying-Earth/issues/80))

- **observatory-mars.png** — `shot:observatory player:archivists turns:0 observatory:25`. The card of
  Olympus Mons on Mars, held by the Archivists, with **25 Colonists of 33 Habitat room** (three
  Habitats of eight at the slot's Habitat yield) and the Module list reading **"Observatory: +4
  Research, 3 Energy upkeep"**: 2 Research, times 1.25 for the 25 Colonists, times the Archivists'
  1.6, rounded down. The Build section carries the new button, **"Observatory (28 Materials) or (56
  Ducats)"**, beside the eight Modules that were there before.

### Measured, twenty seeds each (`sweep/observatory.txt`)

Two seatings at the 0.05.5 climate cell (`ppm_step` 300, Sink 6), the same ones the 0.05.5 balance
report opened with, so the figures can be read against it.

| Seat 0 | Wins by seat | Collapses | Techs (median) | Observatories at the end, all seeds, by seat | Research off Earth a game (median) |
|---|---|---|---|---|---|
| Custodians from East Asia | Custodians 5, nobody else | 15/20 | 9, rung 3 | [0, 0, 0, 0] | 0 everywhere |
| Archivists from East Asia | Custodians 20 (seat 1) | 0/20 | 13, rung 3 | [0, 0, 0, 3] (the Arkwrights' three) | 0 everywhere |

**The rule works and the AI almost never affords it.** In the one seed's log read closely (seed 1,
the Archivist seating), the Custodian AI offers "build Observatory at Lake Vostok" every turn a
Colony of its holds eight Colonists, scored 8.0 (its Research Lab weight) and 24.0 with the victory
gap, but the line always reads *save ... holding Materials for* a Scrubber, a Sea Wall or a Factory
scored higher: the same fate its Research Labs on Earth meet in the same list. The Arkwright AI,
whose Colonies fill to twelve, is the one that builds any. The Archivist AI, which qualifies at four
Colonists, loses East Asia in 17 of 20 seeds at a median turn 21 (0.05.5's open item) and ends with
no buildings at all. Nothing else moved: wins, Collapses, Techs and the Fund read as 0.05.5 did.
Whether the Observatory should outrank a Scrubber in the Custodian AI's hand is on the map.

## The Archivists' card ([ticket #81](https://github.com/whaleyjoshua2/Dying-Earth/issues/81))

- **archivists-factions.png** — `shot:archivists player:archivists turns:0 observatory:25`, the
  Faction choice screen. The Archivists' card now reads **"Facility and Module output x1"** (0.8
  before) and **"Research x1.5"**, and its signature ends **"Their Research is x1.5 on Earth and
  x1.75 off it, an Observatory on a station over Earth counting as off."** The Arkwrights' card
  reads "(12, 15 with Expanded Habitats)" where the Observatory ticket's build had left "(6, 9)".

### Measured, twenty seeds each (`sweep/archivists.txt`)

The same two seatings as the Observatory ticket, so the figures read against that file.

| Seat 0 | Wins by seat | Collapses | Techs (median) | Observatories at the end, by seat | The Archive standing | Research off Earth (median) |
|---|---|---|---|---|---|---|
| Archivists from East Asia | Custodians 18, Arkwrights 1 | 1/20 | 13, rung 3 | [2, 7, 0, 0] | **20/20 seeds, median turn 4**, complete 0/20 | 0 everywhere |
| Custodians from East Asia | Custodians 3 | 17/20 | 7, rung 3 | [0, 0, 0, 8] | **20/20 seeds, median turn 4**, complete 0/20 | 0 everywhere |

**The Archive stands on Axiom.** With a station over Earth counting as off Earth, the Archivist AI
raises the Archive on its start station on turn 4 in every seed, where before this ticket it stood
in no seed of either seating. It is never completed: the fund ends at a median 20 and 12, so the
Research is not being paid in, and the Archivists lose East Asia in 20 of 20 seeds at a median
turn 22 (0.05.5's open item) before it could be. Off-world Presence and the Archivists' twelve
Colonists at the Archive can now both be met in orbit over Earth; how far that should be allowed
is the designer's, and is on the map.

**Observatories** rose from 0 and 3 to 9 and 8 across the two batches with their own weight (the
Archivists' at 12), the Custodian AI in seat 1 building most of them (7); Research made off Earth
still reads a median 0 a game for every seat, so most seeds see none. Everything else moved within
seed noise: the Custodians' wins 5 to 3 and 20 to 18, Techs 9 to 7 and 13 to 13.

## The Custodians' card ([ticket #82](https://github.com/whaleyjoshua2/Dying-Earth/issues/82))

- **moved-mars.png** — `shot:moved player:custodians turns:0 observatory:16 idle:1`. A Custodian
  Colony on Mars with a Factory and a Research Lab standing mothballed in East Asia. The card reads
  **"Mine: +10 Materials, doubled by an idle Factory on Earth"** and **"Observatory: +4 Research,
  doubled by an idle Research Lab on Earth"**, and the top bar's Allotment reads **16 of 16**:
  (10 + 4) x 1.2, where 1.25 gave 17.

### Measured, twenty seeds each (`sweep/custodians.txt`)

The same two seatings as the two tickets before, read against `sweep/archivists.txt`.

| Seat 0 | Wins by seat | Collapses | Scrubbers | Techs (median) | Production Moved (median doubled Module-turns a game) |
|---|---|---|---|---|---|
| Custodians from East Asia | **nobody** (Custodians 3 before) | **20/20** (17) | **94** (173) | 7 | 0 for every seat |
| Archivists from East Asia | Custodians 15, Arkwrights 4 (18 and 1) | 1/20 (1) | 496 (539) | 13 | 0 for every seat |

**Influence 1.2 is the whole of the move, and it is not small.** Seed 3's log shows the new
mothball rule never taken (no "would double" order, no doubled Income line), so the doubling
cannot be what moved the Custodian seating. The other change is the Influence multiplier, so the
seating was rerun once with 1.25 put back and everything else kept: **Scrubbers 185, Collapses
17, Custodian wins 3**, the previous batch's figures. The five hundredths cost the Custodians in
East Asia every win in twenty seeds and half their Scrubbers; East Asia's Allotment falls from 17
to 16, and the AI's Influence game turns on those steps. Read against the Observatory batch, this
seat has gone 5, 3, 0 wins across the three tickets so far.

**Production Moved never fires in AI play.** The Custodian AI idles a Facility only when an
undoubled Module off Earth outproduces it, and its Colonies seldom hold one that does: an Earth
Factory in a Materials-leaning state makes 6, a Moon Mine 6, a Mars Mine 5; only Phobos (7) or
Deep Mining tips it, and the AI reaches neither in time. The rule works on the player's side, as
the picture shows; on the AI's it is a rule in waiting.
