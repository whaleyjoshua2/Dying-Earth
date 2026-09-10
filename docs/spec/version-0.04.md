# Dying Earth — version 0.04, the map: Antarctica, the moons of Mars, stations and Shipyards

**Status:** the destination of the map [Map: version 0.04, the map: Antarctica, Phobos and Deimos, orbital slots and Shipyards](https://github.com/whaleyjoshua2/Dying-Earth/issues/40). Every change here was decided on one of that map's tickets after the designer played version 0.03; each section names its ticket, and the ticket's resolution comment is the authority if this document and it ever disagree. Everything not amended here stands as written in [`first-playable.md`](first-playable.md), [`version-0.02.md`](version-0.02.md) and [`version-0.03.md`](version-0.03.md).

**Numbers** live in `assets/data/`, one file per table; this document names the file beside each change.

---

## 1. The 0.03 answers

Decided on [The 0.03 answers and two interface fixes](https://github.com/whaleyjoshua2/Dying-Earth/issues/41). Amends sections 1 and 4 of version 0.03 and section 16 of the First Playable.

- **A challenge margin.** A controlled place goes to a rival whose Standing is at least the controller's **plus 10** (`influence.toml`, `challenge_margin`) and at least the threshold. A neutral place still needs only the threshold. Measured over twenty seeds: places changing hands by Influence fell from 14.7 a game to about 12, and a contested state now needs a push of two or three turns.
- **Ducats at two for one.** A Restoration step (10 Energy) costs **20** Ducats and a repair point (5 Materials) costs **10**, the same rate as bought Influence (`factions.toml`, `[ducats]`).
- **The AI and the 0.03 buildings.** The first Embassy in a state and the first Relay at a Colony take the AI's threat multiplier while the rival's Standing there is within the margin of its own. Two stronger rules were tried and dropped after measurement (a Bank preferred while Ducats were short; an opportunity multiplier on the most valuable place): each cost the Custodians every win.

## 2. Interface

Decided on the same ticket.

- **The Climate Panel comes back**: its top-bar button reads "Hide Climate Panel (C)" or "Climate Panel (C)", the C key does the same, and a reopened panel returns to its home position at the bottom left.
- **The Tech Tree is drawn as a tree**: five columns for the five branches, a row per rung, a line from every Tech to each Tech that needs it, boxes coloured done, under research, available or locked, the effect on hover, Pick on the available boxes when it is the player's pick.

## 3. The trading window

Decided on [The trading window](https://github.com/whaleyjoshua2/Dying-Earth/issues/42). Amends section 4 of version 0.03.

- A **Trading** button in the top bar opens the window. It sells **Influence at 2 Ducats, Materials at 2, Fuel at 3, Energy at 1** per unit (`factions.toml`, `[ducats]`: `per_influence`, `per_materials`, `per_fuel`, `per_energy`). What is bought is spendable in the same turn's orders.
- It **buys back Materials and Fuel at half** the buying price, rounded down over the lot (`sell_divisor`): ten Materials sell for 10 Ducats, two Fuel for 3. Influence and Energy are not bought back.
- **A building can be bought outright for Ducats** at **twice** its Materials cost (`per_building_material`): an "or (40 Ducats)" button beside every Facility and Module build button. It is placed and queued exactly as a Materials build and takes a build slot like one.
- **No caps.** The Buy Influence button moved from the state card to the window; the top bar shows the Allotment as "free 22 + bought 10" once any is bought.
- The AI buys Materials in lots of ten while Materials are its scarcest resource; it does not sell.

## 4. Colony Ship and Carrier

Decided on [One cargo per Colony Ship, or a Colony Ship and a Carrier](https://github.com/whaleyjoshua2/Dying-Earth/issues/43). Amends section 6 of the First Playable.

- **A Colony Ship carries Colonists only**: four, six with Expanded Habitats.
- **A Carrier** carries one Army and nothing else: 30 Materials, one turn to build, 2 Energy upkeep, strength 0, 4 Hit Points, Pursuit 0 (`units.toml`). It cannot attack, needs an escort, and is a target for Intercept like a Colony Ship.
- **A Battleship fights and carries no Army.** Every landing needs a Carrier.
- The AI builds a Carrier over Earth when it has a free Army and a rival Colony to land on.

## 5. Antarctica

Decided on [Antarctica must be colonized](https://github.com/whaleyjoshua2/Dying-Earth/issues/44). Amends sections 4 and 8 of the First Playable and section 2 of version 0.03.

- **Antarctica is no longer a Nation State.** Eight remain: Africa, Asia, Australia and Oceania, Europe, North America, South America, Russia and the Middle East. Its state card, its Standing Army and its Influence value are gone (`nation_states.toml`).
- **Earth has three Colony Slots**, all in Antarctica (`bodies.toml`): the Antarctic Peninsula, Lake Vostok and the Ross Ice Shelf. A Colony Ship at Earth with Colonists aboard founds one with no transit, through the same order as the Moon. Modules are built there like any Colony.
- **Yields**: Mine 1.0, Generator 0.75, Refinery 1.5, Habitat 0.75. Cold and Fuel-leaning.
- **Its Colonists live on Earth**: they count nothing toward Off-world Presence, the twelve-Colonist condition or the Colonist tiebreak.
- **Its Modules emit**: a Mine as a Factory (1.0), a Generator as a Power Plant (1.5), a Refinery as a Refinery (1.5) (`modules.toml`, `earth_emissions`), counted on the Climate Panel's Factories, Power Plants and Refineries lines, with Clean Power, Clean Manufacturing and the Faction multiplier applying as they do to Facilities. Off Earth a Module still emits nothing.

## 6. Phobos and Deimos, and every Colony Slot a named place

Decided on [Phobos and Deimos](https://github.com/whaleyjoshua2/Dying-Earth/issues/45). Amends section 4 of the First Playable.

- **Two more Bodies**, five in all (`bodies.toml`). **Phobos**: two slots, Mine 1.75, Generator 0.75, Refinery 0.5, Habitat 0.5. **Deimos**: one slot, Mine 1.0, Generator 1.0, Refinery 0.25, Habitat 0.5. Their Colonists count as off Earth.
- **Reach.** A Body with a `parent` is that Body's satellite, and the two are `local_turns` and `local_fuel` apart (the Moon from Earth 1 turn and 6 Fuel; Phobos or Deimos from Mars 1 turn and 2 Fuel). Two satellites of one parent are `sibling_turns` and `sibling_fuel` apart (1 and 1). Earth and the Moon reach anything else at that Body's own figures (Mars 4 turns and 20 Fuel; Phobos or Deimos 5 turns and 24 Fuel). Anything else is the farther card. Efficient Transit multiplies the Fuel as before.
- **Every Colony Slot is a real geological place on its Body**, at its approximate longitude and latitude (`bodies.toml`, `slots`), drawn there on the Surface Map, and **a Colony takes its name**: "Olympus Mons on Mars", "Tycho on the Moon". The Moon: Mare Tranquillitatis, Mare Imbrium, Tycho, the South Pole-Aitken Basin. Mars: Olympus Mons, Valles Marineris, Chryse Planitia, Hellas Planitia, Isidis Planitia, Elysium Planitia. Antarctica: the Antarctic Peninsula, Lake Vostok, the Ross Ice Shelf. Phobos: Stickney, Roche. Deimos: Swift.
- **Pictures**: the USGS Viking cylindrical map of Phobos and Philip Stooke's map of Deimos, both public domain, on spheres (`examples/prep_moons.rs`). On the Solar System Map the moons circle Mars close in, drawn far larger than life to be clickable.

## 7. Orbital slots, Space Stations, Shipyards and Launch Sites

Decided on [Orbital slots, Space Stations, Shipyards for Ships, Launch Sites for lifting](https://github.com/whaleyjoshua2/Dying-Earth/issues/46). Amends sections 4, 5 and 6 of the First Playable.

- **Orbital slots**: Earth 4, the Moon 2, Mars 3, Phobos 1, Deimos 1 (`bodies.toml`, `orbital_slots`), each a named station site (`stations`: Earth the ISS, Tiangong, Skylab and Mir; the Moon Gateway and Selene; Mars Mars Base Camp, Ares and Hermes; Phobos Gateway; Deimos Gateway).
- **A Space Station** is built for **40 Materials** (`station_materials`) into a free orbital slot, with no crew, from a Nation State with a working Launch Site (over Earth) or from a Colony of the builder's on that Body (elsewhere). It appears at the Resolution; two Factions ordering one slot go to the orbital tiebreak. It is named for its slot: "Skylab over Earth".
- **A station holds only a Shipyard and Habitats.** Its Habitats take no Body yield. Influence, Occupation and Battles work on it as on a Colony; its threshold starts at **20** (`influence.toml`, `station_threshold_base`) plus 10 per Colonist. Colonists on a station over Earth are still on Earth for Off-world Presence, like Antarctica's; over any other Body they count.
- **Ships are built only at Shipyards**, on stations or Colonies. A Launch Site builds none.
- **The start**: the Custodians begin with the **ISS** and the Prospectors with **Tiangong**, bare core modules over Earth with no Shipyard (`factions.toml`, `start_station`). The first Shipyard is the first build.
- **A Launch Site lifts.** A Ship at Earth loads Colonists or an Army only from a Nation State with a working Launch Site. Each lift is a launch for Emissions (section 11 of the First Playable); leaving orbit no longer is. A **Launch Pad Fire** closes the state's Launch Site until the next Resolution; **Clean Propellant** keeps it open (`techs.toml`).

## 8. Acceptance for 0.04

- `cargo build --release`; clippy clean across every target under `-- -D warnings`.
- 72 formula tests, every new rule seen red first.
- `shot:` pictures of the drawn Tech Tree, the Climate Panel brought back, the trading window, the Carrier in a Ships list, Antarctica's slots, the Solar System Map with five Bodies and their stations, Phobos on a sphere, Mars's named slots and the two start stations, looked at and in the dev diary (`docs/dev-diary/2026-09-09-version-0.04/`).
- Twenty seeds of `simulate` per seating after each ticket, in the diary.

## 9. Open for the designer

Recorded on the tickets, not decided here:

1. **The Custodians win less and start slower.** The margin alone cost four of six AI wins over twenty seeds by slowing their conquest of Earth; the Shipyard before any Ship moved the first Colony from turn 7 to turn 10 and cut the buildings at the end from 60 to 32. Two wins of twenty at the end of this version, none with the Prospectors in seat 0.
2. **The AI builds no Bank, Trade Post or Relay** (a Factory compounds while Ducats only buy Influence; a Relay needs the rival to press a Colony), **no Carrier** (the Prospector AI has never raised an Army in any batch), **no Colony in Antarctica or on Phobos or Deimos** (a loaded Ship always has a transit first; Mars wins on yields), and **no station beyond its start one**.
3. **The trading window has no caps**: a seat holding several rich states can turn a turn's Ducats into forty-odd Materials.
4. **Two builder's calls on stations**: Colonists on a station over Earth count as on Earth; the start stations are named on the Faction cards.
5. **Slot positions** are to the nearest degree or so from the Gazetteer of Planetary Nomenclature; the Deimos map's zero meridian may not match the Gazetteer's, so Swift may sit off its crater.

---

*Decisions recorded on the map's tickets remain the source of truth. This document assembles them; it does not amend them.*
