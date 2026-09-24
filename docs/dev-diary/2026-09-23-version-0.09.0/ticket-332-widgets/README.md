# Widgets: the build, the findings and the pictures

Ticket [#332](https://github.com/whaleyjoshua2/Dying-Earth/issues/332) on version 0.09.0. The engine
half (the rate, the figure and the count on a Build, the queue served in order, the Mine Facility
and the Factory Module, `CancelBuild`, SAVE_VERSION 3) is commit `9071342`; the computer seats, the
interface and the Factory Module on a station are `0d8ea6c`; the Factory wanted only behind a
two-deep queue and the sweep's Materials counters are `41aabc4`. `SPEC.md` beside this file is the
specification. The pictures below were retaken after the designer moved three figures on seeing the
first closing sweep (the Core Module at four Widgets, a Region at a flat four plus one per Industry
Level, the Factory and the Mine each emitting 0.75); `sweeps/` holds every sweep that led there and
`sweeps/breakdown-probe.md` the emissions probe that found the cause.

| picture | what it shows |
|---|---|
| [`widgets-region-earth.png`](widgets-region-earth.png) | `shot: queue:1 order:powerplant select:eastasia panel:0 window:1600x1000`. The top bar with the **cog** after the Materials' cart, reading `23 / 0`: twenty-three Widgets a turn across the three places the Custodians direct (China 11, the ISS 4, a Moon Colony 8), nought applied because no Resolution has run. China's card: **Widgets 11 a turn** with the queue under it in order -- *Power Plant 3 of 8, ready next turn; Research Lab 0 of 4, ready next turn; Bank 2 of 8, about 2 turns, begun by the Prospectors* -- and the hatched boxes wearing `3 of 8`, `0 of 4` and `2 of 8` in place of the turns, with the Power Plant ordered this turn hatched at `0 of 8`. The Scrubber button reads `30 [cart] 8 [cog]`. The Emissions line reads *Facilities 3.4* where it read 3.8 at 1.0. |
| [`widgets-cancel-earth.png`](widgets-cancel-earth.png) | `shot: queue:1 select:eastasia panel:0 tip:"Right-click to cancel it"`. The hover on the Prospectors' Bank, a build left behind when the Region changed hands: *Bank (inland): building, 2 of 8 Widgets, about 2 turns at this place's rate. Begun by the Prospectors, who paid for it. Right-click to cancel it: 25 Materials to your Stockpile at your own price, and its Widgets so far are lost.* The right-click gives `Order::CancelBuild`. |
| [`widgets-colony-moon.png`](widgets-colony-moon.png) | `shot: queue:1 hab:ground habtile:free tip:"turns here" window:1280x1300`. A Colony card: **Widgets 8 a turn** (the Core Module's four and a Factory Module's four), the queue of three under it, the tiles `2 of 4`, `0 of 4` and `1 of 8`, and the Build here strip with every Module's face reading Materials then Widgets -- `Mine 20 [cart] 4 [cog]`, down to the new **Factory 20 [cart] 4 [cog]** last in the list -- and the Mine's hover: *20 Materials, 4 Widgets, about 3 turns here. Once it stands: +7 Materials, 3 Energy upkeep.* Three turns is right: seventeen Widgets owed ahead of and including it at eight a turn. Build Army (Barracks) carries its cog too. |
| [`widgets-buttons-earth.png`](widgets-buttons-earth.png) | `shot: select:eastasia slotbox:free panel:0 tip:"Once it stands" window:1280x1500`. A fresh China with a free box clicked: the Facility list with the new **Mine 20 [cart] 4 [cog]** in it, every face priced in both halves, the Factory's hover *20 Materials, 4 Widgets, ready next turn here. Once it stands: +4 Widgets, 2 Energy upkeep, 0.6 Emissions* (0.75 through the Custodians' multiplier), and below the boxes `Raise Industry Level 30 [cart] 4 [cog]` and `Build Army 25 [cart] 4 [cog]`. The bar reads `15 / 0` here: China 11 and the ISS 4, no Colony planted. |
| [`widgets-bar-solar.png`](widgets-bar-solar.png) | `shot: queue:1 tip:"across the places"`. The top bar's Widgets hover: the two figures explained, the rule in three sentences, then a line per directed place -- *China: 11 a turn (4 from the Region itself, 3 from Industry Level 3, 4 from the Factory). Power Plant 3 of 8, ready next turn; ...*, *ISS over Earth: 4 a turn (4 from the Core Module). nothing under way.*, and the Moon Colony's *8 a turn (4 from the Core Module, 4 from the Factory)*. |
| [`widgets-underway-solar.png`](widgets-underway-solar.png) | `shot: queue:1 factions:1`. The Faction window's Under way block: *Building: a Habitat at Mare Tranquillitatis on the Moon (2 of 4, ready next turn), a Mine at Mare Tranquillitatis on the Moon (0 of 4, ready next turn), a Power Plant in China (3 of 8, ready next turn), a Research Lab in China (0 of 4, ready next turn)*, soonest first; the Prospectors' two builds are not the Custodians' and are not listed. |

## What was built

- **The top bar**: a Widgets entry with the cog after Materials, `made / applied` -- the sum of
  `widgets_at` over every place the player directs, and what was applied at the last Resolution.
  The second figure needed one engine field: `WidgetCounters::applied_last`, per seat, zeroed at
  the start of every `resolve_builds` and summed as each place applies; `#[serde(default)]`, so
  a save without it loads. The hover lists each place's rate, its makers and its queue.
- **Every Region, Colony and station card**: `Widgets N a turn` with the cog, the base and each
  maker named on its hover (`3 from Industry Level 3, 4 from the Factory`; `1 from the Core
  Module`), and the queue under it in order, each item `Habitat 3 of 8` with its estimate and, for
  a build another seat began, who began it. The makers are read off the same `facility_yield` and
  `module_yield_at` that `widgets_at` sums, so the parts always add up to the line. The lines
  that listed a queued Sea Wall, the Archive on order and a Ship under way have gone: the queue is
  in one place now.
- **The hatched tile** says `3 of 8` in the top-right where it said the turns; an ordered tile
  says `0 of 8`. `TileState::Building` carries `done` and `widgets`. The estimate moved to the
  hover: *building, 3 of 8 Widgets, about 2 turns at this place's rate*.
- **Every build button** (Facilities, Modules, Ships, Army, Industry raise, the Scrubber, the Sea
  Wall, the Archive) carries the Widget figure on its face after the price, `20 [cart] 8 [cog]`,
  and its hover opens `20 Materials, 8 Widgets, about 2 turns here` (`turns_to_build`), then
  `Once it stands:` and the yield. A Ducat buy says `50 Ducats, ready at the next Resolution ahead
  of the queue`; a place that makes nothing says so instead of a turn count. `build_turns_for`
  became `build_item_of` (the place and item an order raises) and `build_words`.
- **Cancel a rival's build**: a right-click on a hatched tile or box whose build belongs to
  another seat, at a place the player directs, places `Order::CancelBuild { place, index }`;
  the hover says the Materials it refunds at the player's own price (`cancel_refund`), and
  after the order is placed says it is cancelled this turn. An own build under way is not
  cancellable, as it never was. Both the Region's boxes and the Colony's tiles go through one
  `building_tip`.
- **The Under way block** reads `3 of 8, about 2 turns`, read off the queues since
  `Game::under_way` carries the estimate alone and two tests pin its shape.
- **The lists**: the Mine Facility and the Factory Module appear in their build lists by the
  existing iteration over the kinds; `Widgets` joined `ICON_WORDS`, so `+4 Widgets` in a produces
  sentence and `8 Widgets` in a hover wear the cog. The Mine Facility wears the Mine Module's
  picture and the Factory Module the Earth Factory's, one name one picture, as the Refinery.
- **The glyph**: `widgets.svg`, in `DRAWN` (no credit owed), with a steel-blue fill of its own in
  `FIGURES`, clear of the Materials' grey it stands beside.
- **Words**: the Faction rulebook's discount rows say `every Ship x0.85 [cart] and [cog]`,
  `a Colony Module x0.75 [cart] and [cog]`, since the discounts reach the Widget figure too. The
  tutorial's turn-2 and turn-4 notes say a Habitat on the ISS is ready next turn and that all
  four Pioneers fit on turn 4; at the Core Module's one Widget a turn, as first built, a
  four-Widget Habitat stood on turn 6 and the notes were re-worded, and at the designer's four
  they are true again as written, so the words from 0.08.8 were restored untouched.
- **A building aid**: `queue:1` plants three builds in the start Region (one of them the
  Prospectors') and a Moon Colony with a Factory Module and three builds, one the Prospectors'.

## Looked at

The six pictures above, opened and read before this was written. Two things were caught and fixed
by looking: the makers list first read `Industry Level 3 3` (the name and then the figure), now
`3 from Industry Level 3`; and at the default 1280x800 the build lists sat below the fold behind
the Influence corner, so the two list pictures were retaken at taller windows. Not photographed:
the right-click itself (headless, no pointer), the Ducat buy's hover, and a station's own card,
whose Widgets line is the same code as the Colony's and whose figure the bar hover shows.

## The closing sweep, at the designer's figures

`sweeps/final-ticket-332.txt`, 20 seeds x four seatings, the Core Module at four Widgets, a Region at a
flat four plus one per Industry Level, the Factory and the Mine each emitting 0.75:

| | 0.08.8 | ticket #332 |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists | 43 / 22 / 1 / 0 | **11 / 13 / 21 / 0** |
| collapses, of 80 (by seating) | 14 (1, 0, 13, 0) | **35 (16, 9, 10, 0)** |
| warships built | 165 | 99 |
| Techs completed, median, by seating | 20 | 20, 18, 20, 20 |
| Materials income a game, Custodians-first seating, by seat | 273 / 1745 / 172 / 131 | 458 / 790 / 302 / 152 |
| Mines and Factories completed over the batch | 176 and 120 | 370 and 827 |

The column is the balance version's, as the map's Notes say: the emissions probe in
`sweeps/breakdown-probe.md` names where the collapses come from, and the eight sweeps beside it show
what each figure does. A rising collapse rate is a figure, not an alarm.
