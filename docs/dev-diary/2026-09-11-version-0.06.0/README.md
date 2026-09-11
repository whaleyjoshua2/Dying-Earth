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
