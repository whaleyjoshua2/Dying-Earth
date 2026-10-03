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

## The Archive fund at 150 (ticket #439)

- [`victory.png`](ticket-439-archive-150/victory.png), taken with
  `victory:1 panel:0 seed:7 player:archivists`. The Victory window's first line reads "The Archive:
  0 of 150 - needs The Upload, not yet researched".
- **Witnessed red:** the new test `the_archive_costs_150_and_the_computer_paces_to_it` failed with
  `left: 125, right: 150`. Two older tests pinned the 125 (#347's figure, and the turn-one line's
  words). They read 150 now; the turn-one line itself already followed the data.

## The Custodians' Influence at ×1.15 (ticket #440)

- [`card.png`](ticket-440-custodians-influence/card.png), taken with
  `factions:custodians rulebook:1 panel:0 seed:7 "tip:Influence Allotment x"`. The forced hover
  reads "Influence Allotment x1.15". The top bar and the Greenwash line both read 14 Influence this
  turn, where 0.09.5's pictures read 15.
- **Witnessed red:** the new test `the_custodians_influence_multiplier_is_one_point_one_five`
  failed before the data moved. Two Allotment tests moved their figures (15 to 14, 20 to 19, 21 to
  20; 16 holds). The 0.06.0 test that pinned 1.2 is gone, superseded.

## A Shipyard wants one Factory, not one every turn (ticket #441)

**Witnessed red:** the new test `a_shipyard_wants_one_factory_module_and_no_more` gives a Moon
Colony a Shipyard and room for several Modules. Before the fix:

- the first Factory was wanted, as it should be;
- with one Factory already standing, a second still scored 12, where it should be wanted not at
  all.

After the fix it scores nothing, whether the first Factory is standing or on order.

**The sweep after four of the five figure tickets** ([`sweeps/after-441.txt`](sweeps/after-441.txt),
20 seeds x four seatings, the shipped cell): Relay Networks (#438), the Archive at 150 (#439), the
Custodians at ×1.15 (#440) and this cap. Colony Ships at 35 Fuel (#455) is not yet in.

| | Custodians | Prospectors | Arkwrights | Archivists | collapses |
|---|---|---|---|---|---|
| 0.09.5 | 10 | 10 | 1 | 17 | 42 |
| after #441 | 12 | 9 | 8 | 7 | 44 |
| designer's ideal | 15 | 15 | 15 | 15 | 20 |

- **The Arkwrights** win 8 and score nought in 37 games, where they won 1 and scored nought in 58.
- **The Archivists** fall from 17 to 7.
- **Collapses** are 44. The agent's trial of the cap alone gave 32. The difference is the other
  three figures: none of the four tickets was measured alone on this branch.
- The whole Tech tree completes in 61 of 80 games, median turn 25.

## Colony Ships hold 35 Fuel, and the build price matches the tank (ticket #455)

- [`build-hover.png`](ticket-455-colony-ship-tank/build-hover.png), taken with
  `first:1 hab:ground yard:1 seed:7 "tip:Built full"`. A Moon Colony with a Shipyard on turn 2.
  The Colony Ship's button reads 22.5 Materials, 35 Fuel and 4 Widgets. It is greyed for the Fuel
  (28 held), and the hover ends "Built full: 35 [Fuel]."
- **Witnessed red, four new tests:**
  - **the tank:** read `left: 40, right: 35`;
  - **a Frigate ordered before Cryogenic Tanks completed:** came out holding 45 having paid for 30;
  - **a cancel:** refunded no Fuel (`left: 0.0, right: -30.0`);
  - **the computer's warship:** weighed alone, with no Buy for its tank. With the old line the
    Frigate's skip read "needs 25 Materials, 0 left"; with the fix it reads "needs 90 Ducats", the
    tank's purchase in the bundle. A first version of this test passed both ways and was rewritten
    until it told them apart.
- **Older tests moved to 35:** four pinned 40 (the 0.06.0 tank test, Cryogenic Tanks' sums 40 / 45
  / 60 and its Provisional Findings 47, the Refinery depot's refuel of 39, and the market test's
  "40 wanted").

**The sweep after all five figure tickets** ([`sweeps/after-455.txt`](sweeps/after-455.txt)):

| | Custodians | Prospectors | Arkwrights | Archivists | collapses |
|---|---|---|---|---|---|
| 0.09.5 | 10 | 10 | 1 | 17 | 42 |
| after #441 (four tickets) | 12 | 9 | 8 | 7 | 44 |
| after #455 (all five) | 9 | 10 | 6 | 9 | 46 |
| designer's ideal | 15 | 15 | 15 | 15 | 20 |

The 35 tank moved every Faction by 3 wins or less, which is inside the noise. Over the four
seatings, Refuel orders came to 330, against 306 after #441. Ships stranded at the end came to 37,
against 38. The four Factions now sit within 4 wins of one another. Collapses, at 46, are the
figure farthest from the ideal.

## A captured Scrubber runs at half (ticket #445)

A new aid, `scrubber:<Region>` (or `scrubber:own`), stands one Scrubber in a Region.

- [`held-by-prospectors.png`](ticket-445-captured-scrubbers/held-by-prospectors.png), taken with
  `player:prospectors scrubber:own select:eastasia panel:0 seed:7 window:1280x1600`. China, held by
  the Prospectors: "Scrubber: +1.5 ppm Sink, 0.5 off Unrest a turn, 3 [Energy] upkeep (x0.5)".
- [`neutral.png`](ticket-445-captured-scrubbers/neutral.png), taken with
  `scrubber:middleeast select:middleeast panel:0 seed:7`. Neutral Iran: "Scrubber: +0.75 ppm Sink,
  0.25 off Unrest a turn (x0.25)", with no upkeep.
- **Witnessed red, two new tests:**
  - **the captured case:** read 0 Scrubbers standing after a transfer, where 2 were wanted;
  - **the throw-off case:** read 0 standing after a throw-off. Its setup first failed because the
    Scrubbers' own calm held the Region below the throw-off; they are switched off for that step.
  - Both now allow for the transfer's destruction roll, which can take one building of any kind.
- **Older tests:** the 0.05 test that pinned destruction now pins half. The ticket #351 test (a
  Region occupied from neutral adds nothing) holds unchanged, by keeping that case at nought.

## A Colony Ship takes Pioneers from several countries (ticket #443)

A new aid, `emigrantsall:<n>`, puts n Pioneers in every Region seat 0 holds.

- [`ship-card.png`](ticket-443-several-regions/ship-card.png), taken with
  `pressedat:1 emigrantsall:3 colonyship:1 stack:earth ship:1 panel:0 seed:7`. An empty Colony Ship
  in low orbit over Earth, with two rows: "Load 3 Pioneers from Nigeria" and "Load 3 Pioneers from
  China". Nigeria has no Launch Site, and its row is open because the Ship is in low orbit. Under
  them, "Load 2 Colonists from ISS over Earth" is greyed, since the ISS is reached from its own ring.
  The tank reads 35/35 (ticket #455).
- **Witnessed red, three new tests:**
  - **the rule:** a second Region's Load was refused, "this Ship already has an order";
  - **low orbit:** refused for want of a Launch Site;
  - **the computer:** run against the previous `ai.rs`, it loaded from China alone (`[EastAsia]`).
- **Older tests:** two checked a Launch Site refusal with the Ship in low orbit. They now check it from
  a station's ring.

## Founding a station from a ground Colony, and a ground Colony from a station (ticket #442)

New aids:
- `venusring:1`: a loaded Colony Ship in Ishtar's empty ring;
- `marsstation:1` and `marsstation:down`: a station of seat 0's over Mars, and with `down` an empty
  ground Colony below it;
- `selectstation:<body>`: opens seat 0's station card at that Body.

The pictures:
- [`found-at-venus.png`](ticket-442-founding/found-at-venus.png), taken with
  `venusring:1 stack:venus ship:1 seed:7`.
  - The Colony Ship at Ishtar, with "Found Ishtar with 4 Colonists" under a slider at 4.
  - The orbit moves offer Aphrodite and Lada, and no low orbit.
  - The map's "Orbital Control of low orbit" line is gone from Venus. It was there in the first take
    of this picture and was fixed before it was filed.
- [`build-from-station.png`](ticket-442-founding/build-from-station.png), taken with
  `marsstation:1 site:mars,1 seed:7`. Valles Marineris, empty, with "Build a Colony here from your
  station 40 [Materials]".
- [`send-down.png`](ticket-442-founding/send-down.png), taken with
  `marsstation:down selectstation:mars seed:7`. Mars Base Camp, six aboard, with a slider at 4 and
  "Send 4 Colonists down to Olympus Mons on Mars".

**Witnessed red:**
- **The 0.06.0 Venus test** (a station built from any Ship there) failed under the new rule: the
  Materials build it now refuses was still accepted. It is rewritten as founding by Colony Ship.
- **The Venus low-orbit test** could not compile before `has_low_orbit` existed. The Transit refusal
  was added after.
- **The save migration**, switched off, loaded the Ship with `left: None, right: Some(0)`.
- **The computer's station-and-colony test**, run against the previous `ai.rs`, weighed no Colony
  built from the station.
- **The orbit count** across six Bodies moved from 21 to 20.

**The sweep after this ticket** ([`sweeps/after-442.txt`](sweeps/after-442.txt)):

| | Cust | Pros | Ark | Arch | collapses |
|---|---|---|---|---|---|
| before (after #443) | 9 | 10 | 6 | 9 | 46 |
| after #442 | 10 | 10 | 7 | 9 | 44 |

The move is inside the noise. Settlement at the end barely moved:
- ground Colonies on the Moon 131, where it was 125;
- Venus stations 4, as before.

The sweep counts no use of the new routes (a station founded by Colony Ship, a Colony built from a
station, people sent down). How often the computer took them is not measured here.
