# Version 0.09.5, the fog version

The map is [Map: version 0.09.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/423), and the
spec is [`docs/spec/version-0.09.5.md`](../../spec/version-0.09.5.md).

## Each Victory Tech at the end of a line (ticket #424)

Taken headlessly with `shot: tech:1 panel:0 seed:7 cardshut:1 window:1500x1300`, adding
`techhover:<tech>` for the lit paths.

- [`tree.png`](ticket-424-victory-ends/tree.png): the tree at rest. The rows are Society,
  **Stewardship**, Off-world Living, Extraction, Propulsion. Planetary Stewardship is the lower box
  in Stewardship's rung-3 cell, under Clean Manufacturing. Society's rung 2 is Civil Defense alone.
  The line from Clean Power down to Closed-Loop Colonies is still drawn; that link belongs to the
  prerequisites ticket.
- [`lit-stewardship.png`](ticket-424-victory-ends/lit-stewardship.png): the Custodians' road lit, as
  Efficient Grids, Clean Power, Green Consensus and Planetary Stewardship, all in one row.
- [`lit-upload.png`](ticket-424-victory-ends/lit-upload.png): the Archivists' road lit, as Large
  Language Models, Civil Defense and The Upload, all in Society. "Large Language Models" fits its
  box.

## Four prerequisites moved, and the lines redrawn (ticket #425)

- [`tree.png`](ticket-425-prerequisites/tree.png): the whole tree. No line leaves its row. The
  Propulsion row is two lanes, with Orbital Refuelling on top. Clean Power's line into Planetary
  Stewardship forks out beside Clean Power and enters near the top of the box. Nuclear Rockets' line
  enters Hardened Hulls near the bottom.
- [`lit-hardened-hulls.png`](ticket-425-prerequisites/lit-hardened-hulls.png): Hardened Hulls' path
  lit, as Orbital Refuelling, Efficient Transit, Clean Propellant and Nuclear Rockets.

## Commodity Finance (ticket #426)

- [`tree.png`](ticket-426-commodity-finance/tree.png): the tree with twenty-five Techs. Commodity
  Finance is in the middle of Extraction's rung-2 stack, level with the Extraction Charter;
  Beneficiation is below it and leads nowhere.
- [`lit-charter.png`](ticket-426-commodity-finance/lit-charter.png): the Prospectors' road lit, as
  Deep Mining, Automated Refining, Commodity Finance and the Extraction Charter.

## Orbital Data Centers (ticket #433)

- [`tree.png`](ticket-433-orbital-data-centers/tree.png): the tree with twenty-six Techs. Society's
  rung 2 is Civil Defense over Orbital Data Centers; both lines enter The Upload, one from above
  and one from below.
- [`lit-upload.png`](ticket-433-orbital-data-centers/lit-upload.png): the Archivists' road lit, as
  Large Language Models, Civil Defense, Orbital Data Centers and The Upload.

## Pioneers moved by slider (ticket #428)

Taken with the new `colonyship:1` aid (an empty Colony Ship of yours at Earth) beside `emigrants:5`,
`room:1`, `antarctic:1` and `select:EastAsia`.

- [`region.png`](ticket-428-sliders/region.png): China's card. A Recruit slider at 2, then one shared
  slider at 5 above the doors: "Send 5 to ISS over Earth by lift" and "Send 4 to TSV Endeavour",
  each capped at its room.
- [`sea.png`](ticket-428-sliders/sea.png): the same card with the ice open. The sea doors take the
  slider's 5; the lift is capped at the station's room of 2.
- [`ship.png`](ticket-428-sliders/ship.png): the Colony Ship's card. The Region drop-down, a slider
  under it, and "Load 4 Pioneers".

## A warning on moving an empty Colony Ship or Carrier (ticket #429)

- [`card.png`](ticket-429-empty-ship/card.png): an empty Colony Ship's card at Earth (`colonyship:1
  stack:earth ship:1`). "Empty." in amber under Transit, above the moves.
- [`confirm.png`](ticket-429-empty-ship/confirm.png): the right-click's confirm, raised by the new
  `emptymove:1` aid: "Empty Colony Ship. Send?" with Send and Back.

## Fog of war (ticket #430)

Taken ten turns in (`turns:10`, seed 7), with the new `accords:0` aid. By turn 10 the computer
playing seat 0 had struck an Accord with all three rivals, and an Accord opens a rival's books, so
without the aid the fog lifts entirely. The `reveal:1` aid gives the same board with no fog.

- [`compare-solar.png`](ticket-430-fog/compare-solar.png): the Solar System Map, fogged on the left
  and revealed on the right. The Archivists' Colony Ship in flight to Deimos, its line and its label,
  are gone under the fog. Mars keeps its full detail, because seat 0 has a frigate there.
- [`compare-nigeria.png`](ticket-430-fog/compare-nigeria.png): Nigeria's Army shield, a Region out of
  seat 0's sight. Fogged it reads "x1" with no trench line; revealed it reads its strength, 5, dug
  in.
- [`fog-solar.png`](ticket-430-fog/fog-solar.png), [`revealed-solar.png`](ticket-430-fog/revealed-solar.png),
  [`fog-earth.png`](ticket-430-fog/fog-earth.png), [`revealed-earth.png`](ticket-430-fog/revealed-earth.png):
  the whole windows.
