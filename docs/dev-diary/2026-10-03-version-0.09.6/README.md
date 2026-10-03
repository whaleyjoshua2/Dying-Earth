# Version 0.09.6, the growth version

The map is [Map: version 0.09.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/435), and the
spec is [`docs/spec/version-0.09.6.md`](../../spec/version-0.09.6.md).

## The crash on a Colony Ship at a Venus station (ticket #436)

Taken headlessly with the new `venusstation:1` aid: a station of seat 0's at Aphrodite over Venus,
and a Colony Ship of theirs with four Colonists in its ring. The command was
`shot:<prefix> venusstation:1 stack:venus ship:1 seed:7`.

- **Red, before the fix:** the same command exited 101 with
  `panicked at src\ui.rs:8450:203: index out of bounds: the len is 0 but the index is 1`. That is
  the panic the designer's save gives.
- [`card.png`](ticket-436-venus-crash/card.png), **after the fix:** TSV Endeavour's card at
  Aphrodite. Under Load and unload, a slider at 4 and "Unload 4 Colonists into Aphrodite".

## Moving Colonists from a Colony Ship onto a station (ticket #437)

Two new aids:
- `venusstation:low`: `venusstation:1`'s Ship in low orbit instead of Aphrodite's ring;
- `issload:1`: seat 0's first station over Earth holding four Colonists, with an empty Colony Ship
  in its ring.

The pictures:
- [`wrong-orbit.png`](ticket-437-onto-a-station/wrong-orbit.png), taken with
  `venusstation:low stack:venus ship:1 "tip:unload next turn"`. The Ship is in low orbit, so
  "Unload 4 Colonists into Aphrodite" is greyed, with the hover "Move this Ship to Venus, at
  Aphrodite, then unload next turn: Aphrodite over Venus is reached from there alone."
- [`load-from-iss.png`](ticket-437-onto-a-station/load-from-iss.png), taken with
  `issload:1 stack:earth ship:1`. The Ship card at Earth shows the Pioneers drop-down, then a slider
  at 4 and "Load 4 Colonists from ISS over Earth".

**The Education blend:** witnessed red in
`colonists_unloaded_onto_a_station_blend_their_education`, which failed with "four at 1.0 and four
at 2.0 make 1.5, not 1". It passes after the fix.

## Generation Ships needs Relay Networks (ticket #438)

- [`lit-generation-ships.png`](ticket-438-relay-networks/lit-generation-ships.png), taken with
  `tech:1 panel:0 seed:7 cardshut:1 window:1500x1300 techhover:generation_ships`. The Arkwrights'
  road is lit: Expanded Habitats, then Closed-Loop Colonies and Relay Networks stacked at rung 2,
  then Generation Ships. Each rung-2 box's line enters Generation Ships at its own quarter. No
  re-layout was needed.
- **Witnessed red:** the new test `generation_ships_needs_relay_networks_and_every_road_costs_130`
  failed with `left: [ClosedLoopColonies]`, `right: [ClosedLoopColonies, RelayNetworks]`. Two older
  tests pinned the old road (its 98 Research and its depth of 2). They were moved to 130 and 3, each
  with the ticket named.
