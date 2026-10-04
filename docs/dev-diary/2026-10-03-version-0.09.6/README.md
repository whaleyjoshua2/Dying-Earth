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

## Natural population growth and decline (ticket #444)

- [`colony-growing.png`](ticket-444-growth/colony-growing.png), taken with
  `marsstation:1 selectstation:mars seed:7 "tip:Growing"`. Mars Base Camp, 6 of 8: "Growing: +0.1 a
  turn, next Colonist in 9 turns."
- [`region-growth.png`](ticket-444-growth/region-growth.png), taken with
  `select:eastasia panel:0 seed:7 "tip:Growth +"`. China's population hover: "Growth +1.00% a turn,
  less 0.15% per tenth of a degree above +1.2 C. This turn +1.00%: +14.5 million."
- **Witnessed red:** the new test `a_colony_grows_by_two_percent_a_turn_and_declines_when_starved_or_dark`
  could not compile before the figure and the step existed.
- **The save test** now also cuts `colony_growth`, so an older save without it still loads.

**The sweep after this ticket** ([`sweeps/after-444.txt`](sweeps/after-444.txt)): 10 / 7 / 8 / 10 with
45 collapses, where it was 10 / 10 / 7 / 9 with 44. The move is inside the noise. The median
Colonists off Earth at the end, seating by seating, were 92 / 109 / 150 / 133, against
92 / 105 / 143 / 139. Growth stops at a Colony's room, and most computer Colonies stand full, so it
adds little as the computer plays today.

## The Custodians' aggression follows a rival's CO2 (ticket #446)

**Witnessed red:** the new test `the_custodians_aggression_follows_a_rivals_emissions` checks
×1.3 at a 40% share, ×2 at the cap, nothing at or under a quarter or for any other seat, and cause
at −3. Run with `emitter_cause` set back to −5, it failed on "the Custodians act at −3 against a
heavy emitter". No picture: nothing on screen changes.

**The sweep after this ticket** ([`sweeps/after-446.txt`](sweeps/after-446.txt)): 10 / 8 / 8 / 8 with
46 collapses, where it was 10 / 7 / 8 / 10 with 45. The move is inside the noise. In the seating with
the Custodians as seat 0, against the sweep after #444:

| Seat 0 (the Custodians) | after #444 | after #446 |
|---|---|---|
| Agitates landed | 135 | **382** |
| Battles opened | 15 | 17 |
| places taken by force | 11 | 6 |
| Smear laid on the Prospectors | 71 | 70 |

The new aggression shows almost entirely as Agitate, nearly three times as much. A Smear still needs
the rival's whole-game Blame above the quarter, so it does not move.

## Two texts made shorter (ticket #452)

- [`blame.png`](ticket-452-two-texts/blame.png), taken with
  `factions:custodians panel:0 seed:7 "tip:Above a quarter"`. The Blame hover in six wrapped lines,
  where it ran to eleven. It is drawn through `rule_tip` now, so the `tip:` aid can open it.
- [`slots.png`](ticket-452-two-texts/slots.png), taken with
  `select:eastasia panel:0 seed:7 "tip:Slots: Size"`. China's slots hover: "Size 3 + 3 + starting
  Industry 3, +1 inland per raise", then 4 coastal, then the line about mothballed and building.

## The Colonists line (ticket #450)

- [`full-red.png`](ticket-450-colonists-line/full-red.png), taken with `first:1 hab:ground seed:7`.
  The Moon Colony, 4 of 4: "4/4" in red.
- [`three-quarters-amber.png`](ticket-450-colonists-line/three-quarters-amber.png), taken with
  `marsstation:1 selectstation:mars seed:7 "tip:Colonists 6 of"`. "6/8" in amber, with the hover
  "Colonists 6 of 8 room. Growing: +0.1 a turn, next Colonist in 9 turns."
- [`half-plain.png`](ticket-450-colonists-line/half-plain.png), taken with `hab:1 seed:7`. The ISS,
  "2/4" in plain text.

## The computer seats use the market (ticket #448)

Three sweeps, all filed:

| run | Cust | Pros | Ark | Arch | collapses |
|---|---|---|---|---|---|
| before (after #446) | 10 | 8 | 8 | 8 | 46 |
| [`after-448-first-build.txt`](sweeps/after-448-first-build.txt): sells beyond 20 + 3 turns of spending | 6 | 13 | 3 | 7 | 51 |
| [`after-448-no-churn.txt`](sweeps/after-448-no-churn.txt): no sale in a turn it bought, nor while saving | 6 | 8 | 2 | 18 | 45 |
| [`after-448.txt`](sweeps/after-448.txt): keeps 3 turns of income too, at the designer's word | 9 | 6 | 4 | 19 | 42 |

- **The first build churned.** Every seat bought and sold thousands of units, one seating buying
  3,651 and selling 3,586: Materials bought, then sold back at half the price.
- **Stopping same-turn and saving-turn sales** did not stop selling between big builds.
- **The designer chose A**, a reserve of income turns, at three. Measured, it moved little: one
  seating's trade figures were unchanged.
- **Offered C** (Fuel only) **or D** (no selling), **the designer chose E**: keep it as built, with
  these figures.

**Witnessed red:** the new test `the_ai_sells_its_surplus_and_buys_the_energy_it_lacks`, run against
the previous `ai.rs`, sold no Materials. An older test (#57, Fuel banked for the Mars window)
failed on the first cut, which sold 130 Fuel two turns from the window. Fuel sales now wait for the
window.

## The computer pace tables (ticket #449)

Three sweeps, all filed:

| run | Cust | Pros | Ark | Arch | collapses |
|---|---|---|---|---|---|
| before (after #448) | 9 | 6 | 4 | 19 | 42 |
| [`after-449-before-uploads.txt`](sweeps/after-449-before-uploads.txt): Prospectors rescaled, Arkwrights in Bodies | 7 | 3 | 6 | 22 | 41 |
| [`after-449-uploads-fault.txt`](sweeps/after-449-uploads-fault.txt): the Uploads pace read from turn 1 | 9 | 11 | 10 | **0** | 49 |
| [`after-449.txt`](sweeps/after-449.txt): the Uploads pace read once the Archive is complete | 8 | 3 | 6 | 22 | 41 |

- **The Arkwrights settle Bodies now.** They end at nought in 23 games, where it was 49; their median
  score is 0.33, where it was 0. Ground Colonies on Mars, Phobos and Deimos came to 79 over the
  batch, against 37. Their wins moved 4 to 6, inside the noise.
- **The Prospectors** fell 6 to 3, their Fund medians about the same.
- **The Uploads pace had a fault in its first cut.** Read from turn 1, the computer was "behind on
  Uploads" before any Archive could stand, chased Colonists, and built the Archive in 3 games of 80:
  no Archivist win. Read only once the Archive is complete, it changes almost nothing: the Archivists
  already Upload whenever they can.
- **Against the ideal** (15 / 15 / 15 / 15 and 20) the column is no closer than before the ticket.

**Witnessed red:**
- the guard test, on the Prospectors' 1000 against 2500;
- the Arkwrights' test, against the previous `ai.rs`: thirty over Earth and no Body read "gap x1.00";
- the Archivists' test, twice: nobody Uploaded at turn 30 read on pace before the pace existed, and
  with no Archive the first cut read "gap x3.00 on Presence".

## The computer seats break Blockades (ticket #447)

**Found at the start (measured):** Blockades had nearly vanished this version. Games with a Blockade
went 18 (0.09.5) to 5, orbital Battles 21 to 7 and warships built 199 to 77, the falls coming with
the market (#448) and pace (#449) tickets.

**The sweep after this ticket** ([`sweeps/after-447.txt`](sweeps/after-447.txt)):

| Over 80 games | before | after |
|---|---|---|
| games with an orbital Battle | 7 | 14 |
| games with a Blockade | 5 | 7 |
| warships built | 77 | 103 |
| Batteries standing at the end | 28 | 42 |
| Blockade-turns suffered | 0 | 5 |
| wins (Cust / Pros / Ark / Arch) | 8 / 3 / 6 / 22 | **15 / 10 / 10 / 5** |
| collapses | 41 | 40 |

- **The win column moved more than anything else.** The Archivists fell from 22 to 5 and end at
  nought in 51 games, where it was 34. The Custodians, Prospectors and Arkwrights each rose. Why the
  Archivists fell was not traced.
- **The sweep does not count stations over Earth**, so how often the backup yard was built is not
  measured.

**Witnessed red, three new tests:**
- a blockaded seat had no cause against the blockader;
- a Frigate scored 3 with no fleet and 3 with two warships;
- no second station over Earth was weighed.

## An offline building's hover says why (ticket #451)

A new aid, `offline:<case>` (`card`, `unkept`, `half`, `blockade`), sets up one way a building makes
nothing.

- [`struck-by-a-card.png`](ticket-451-offline-reasons/struck-by-a-card.png): China's Power Plant,
  dimmed, "Offline until next turn: struck by Labour Dispute."
- [`sea-wall-unkept.png`](ticket-451-offline-reasons/sea-wall-unkept.png): the Sea Wall's row,
  "unkept this turn", and its hover ending "Offline: upkeep unpaid." It said nothing about being
  offline before.
- [`at-half.png`](ticket-451-offline-reasons/at-half.png): China at Unrest 7. The Power Plant reads
  "At half: Unrest." above its sums, which end "halved at Unrest 7: 3". The first take lost the line:
  a hover with sums kept only its first line. Fixed, and the same fix keeps an offline line there.
- [`blockaded.png`](ticket-451-offline-reasons/blockaded.png): the ISS under a Prospector Blockade;
  its Core Module reads "Blockaded: makes nothing."

**Witnessed red:**
- the new test `an_offline_building_records_why` could not compile before the record existed;
- the Sea Wall test's new assertion, with the Unkept line taken out, read `left: None, right:
  Some(Unkept)`.
- The older-save test now cuts `offline_cause` too.

## Two Regions added: Pakistan and the United Kingdom (ticket #453)

- [`mask-preview.png`](ticket-453/mask-preview.png): the redrawn mask. Pakistan in dark green from
  the Arabian Sea to the Kazakh steppe; the United Kingdom, Ireland and Iceland in slate.
- [`pakistan-earth.png`](ticket-453/pakistan-earth.png): Pakistan's card on turn 1, with its flag, a
  Power Plant and a Mine, and its label between Iran, India and China.
- [`uk-earth.png`](ticket-453/uk-earth.png): the United Kingdom's card, with its four buildings. The
  first take had its label under the European Union's; it was moved north to Scotland.
- [`start-earth.png`](ticket-453/start-earth.png): the board on turn 1 with nothing selected.

**Witnessed red:** the new test `pakistan_and_the_united_kingdom_share_out_their_parents` could not
compile before the two Regions existed. Seventeen older tests then failed on the parents' old figures
and were brought up to the new ones; two enforced the start-building rule, which the first tables
broke and the designer settled (the rule kept, Pakistan's lone Mine the exception).

**The sweep** ([`after-453.txt`](sweeps/after-453.txt)): 5 / 3 / 11 / 15 wins and 46 collapses,
where fourteen Regions gave 15 / 10 / 10 / 5 and 40. Not traced.

## Closing the version (ticket #454)

[`sweeps/final-0.09.6.txt`](sweeps/final-0.09.6.txt): the closing sweep, the sweep after ticket #453
with one line a seating added (natural growth). 5 / 3 / 11 / 15 wins, 46 collapses. Colonists grown
379, lost to decline 9; 55,942 units sold on the market; 3 Blockade-turns.
