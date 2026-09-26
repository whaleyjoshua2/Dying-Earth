# Ticket #377: equal starts

The designer: *"examine starting resources/buildings and equalize."* Decided in one round of four,
with an addition in the answer: every home Region starts with the same Facilities; the Arkwrights'
start stays their own; every starting station carries a Solar Array, and the Arkwrights a second
Power Plant in its place. The computer seats taking the richest free Regions was decided, built,
swept, and **reverted on the sweep's figures** in a second round, which also let the Solar Array
keep the Module slot it takes.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/377) is the authority;
[§12 of the spec](../../../spec/version-0.09.2.md#12-equal-starts) records it.

## What the driver found before the round

The ticket's table assumed the four seats sit in the EU, China, India and the United States. They
did not. The computer's rule since ticket #50 took the highest Industry Level *not next to a taken
Region*, so once the player took a big economy the computer went to Saudi Arabia, Australia, Iran
and Japan. Opened with the driver (`play new`, seed 1) and read on turn 1:

| home Region | Facilities at start | Ducats a turn |
|---|---|---|
| EU (player, Custodians) | Power Plant, Factory, Mine, Refinery + Launch Site | +12 |
| US (player, Archivists) | the four + Research Lab + Launch Site | +13 |
| China | the four + Launch Site | +12 |
| India (player, Arkwrights) | Factory, Mine, Power Plant (no Refinery) + Spaceport | +1 |
| Saudi Arabia (computer) | Refinery, Power Plant | ~1 |
| Australia (computer) | Power Plant, Factory, Mine | ~1 |

The Facilities were the small gap; the Region's economy (GDP x Industry Level) was the large one.
The stockpile was the same for all four, 80 / 20 / 20 / 0, as the ticket said.

## What was built

- `factions.toml [start]`: `home_facilities = [power_plant, factory, mine, refinery, launch_site]`
  and `station_modules = [solar_array]`; the Arkwrights' card: `start_extra_facilities =
  [power_plant]`. The loader reads all three.
- `Tables::ai_start_state`: **unchanged in the end.** The richest free Region (by GDP, ties by
  Industry Level, then population, adjacency not minded) was built and swept first; see the sweep
  below for why it was reverted. Its doc comment records the try.
- Setup (`Game::new`): a home Region's card list is cleared and the package stands in its place, in
  the Faction's own versions (the Archivists' Reactor, the Arkwrights' Spaceport); a starting
  station's Modules are the Core and the `[start]` Modules in the Faction's own versions (the
  Archivists' Heliostat for the Solar Array).

Opened again with the driver after the build, seed 1, under the spread rule as kept:

| player | the computer's Regions | player's Ducats a turn | Energy a turn |
|---|---|---|---|
| Custodians in the EU | China, Saudi Arabia, Australia | +12 | +5 (Solar Array +6) |
| Arkwrights in India | EU, Japan, Saudi Arabia | +1 | +3 (second Power Plant +6) |
| Archivists in the US | China, Iran, Australia | +13 | +6 (Heliostat +7) |
| Prospectors in China | EU, Saudi Arabia, Australia | +12 | +4 (Solar Array +7) |

Every home Region reads the same five (the Arkwrights' six), Saudi Arabia and Australia included;
the Regions' economies stay what the map makes them.

**The Solar Array takes a slot.** A starting station has 2 Colonists and so 2 Module slots; the
Array sits in one. The Prospectors' computer scores the Exchange (34.5) above the opening Habitat
of ticket #290 (30.0), so it opens with an Exchange in the one free slot and no Habitat. Put to the designer with three
options (exempt the start Array from the slot count, rank the Habitat above the Exchange, accept
it) and accepted as built. Measured afterwards under the spread rule: the Archivists, opening in
Australia, take a **Shipyard** into Axiom's one slot (90.0 against the Habitat's 42.0), so only the
Custodians still open with a Habitat. Each computer's own scoring, through the same one slot.

## The red witness

Two tests were written first, against the old rule, and failed on it:

    the_ai_seats_take_the_richest_free_regions
      left: [EastAsia]
     right: [NorthAmerica]
    every_home_region_starts_with_the_standard_facilities_and_every_starting_station_a_solar_array
      left: [Factory, Mine, PowerPlant, Refinery, LaunchSite]
     right: [PowerPlant, Factory, Mine, Refinery, LaunchSite]

Then the rule moved and both are green. Twenty-nine older tests had the old start built into
their figures (the computer's old Regions, a home Region's card list, a station with a bare Core)
and were brought to the new rule, never weakened; the `game()` fixture, a bare board by contract,
now strips the start Solar Array as it strips the start Facilities. See the commit.

## The sweep

A rule change, so the sweep was run twice, 20 seeds x four seatings at the shipped cell, against
the sweep after #376. First with the richest-Region pick, then, at the designer's word ("revert
the adjacency rule and check the numbers"), with the spread rule kept; the file
[`../sweeps/after-377.txt`](../sweeps/after-377.txt) holds the second, the rule that shipped.

| | after #376 | richest Regions (tried) | spread kept (**shipped**) |
|---|---|---|---|
| Custodians | 6 | 0 | **2** |
| Prospectors | 25 | 1 | **5** |
| Arkwrights | 1 | 0 | **0** |
| Archivists | 4 | 1 | **11** |
| collapses of 80 | 44 | 78 | **62** |

Under the richest-Region pick all four seats sat in the four biggest economies, which are the four
biggest emitters: the first seating's collapses went from 8 of 20 to 20 of 20 and its median
collapse turn from 32 to 28, the Archive stood in 18 seeds where it stood in 6, the Moon was
reached in every seed. The designer reverted it.

Under the spread rule with the new package, collapses rose from 44 to 62 (the first seating 15 of
20, median turn 30) and the win column turned over: the Archivists went from 4 to 11 and the
Prospectors from 25 to 5. The Archivists were first to the Moon in 20 of 20 seeds (median turn
15) where they were in 8 of 11: their starting station's Heliostat (+7 Energy) and their home's
Reactor put them in Energy from turn one, where before their computer seat opened in Australia on
three Facilities. The Prospectors' opening lost the Habitat to the Exchange (above) and the two
poor Regions the spread rule deals now carry a Refinery and a Factory they did not have, so every
seat is stronger and the Prospectors' lead shrank.

## The pictures

Headless, 1400x1000, `panel:0`, from this folder.

| picture | aids | what it shows |
|---|---|---|
| [`home-earth.png`](home-earth.png) | `select:eastasia` | **The home Region's card**, China as the Custodians: Power Plant, Factory, Mine, Refinery, Launch Site, and four slots free. |
| [`station-earth.png`](station-earth.png) | `hab:1` | **The starting station's card**, the ISS: the Core Module and a Solar Array, one slot free. |

## The review

Two axes, run as sub-agents over the commit. **Spec** found the build matching §12 on every claim
and three things beside it: a stale comment in the computer's early-Mine lane, which still said the
Arkwrights start with no Mine (reworded); the loader accepting a `[start]` table with no
`home_facilities`, which would open every home Region bare (it refuses a package with no Launch
Site now, witnessed red); and one edge §12 did not cover, an Arkwrights player opening in Mexico,
whose five slots cannot hold their six-piece package, so they open with no free slot until an
Industry Level raise. §12 says so now. **Standards** found the real defect: `start_emissions`, the
figure the setup card shows for a Region's opening Emissions, still summed the card's list where
the board would hold the package; it sums the package and the Faction's extras now, in the
Faction's own versions, pinned by a test written against the old body:

    the package and the card must differ for the test to mean anything  -- passes on the old body, then:
    panicked: 5.74 against 8.74

and green on the new. The rest were words: the old slot check's comment still leaving room for a
Launch Site the setup no longer adds; a test's doc quoting the computer's scores as if derived;
one test's sentence describing the old coastal order before the new; a convoluted assert spelled
as the file's own idiom; and the `[start]` comment saying "exactly these" two hundred lines from
the Arkwrights' extra.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 499 + 6, root 8.
