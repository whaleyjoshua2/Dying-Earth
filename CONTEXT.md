# Dying Earth

A single-player, turn-based strategy game about colonizing the solar system before ecological collapse overtakes Earth. It should feel like a strategy board game rather than an action game.

## Language

**First Playable**:
The small version of the game: twelve turns and two Factions as first built, twenty-four turns since version 0.02, four Factions since version 0.05 and thirty-six turns of two months since version 0.05.5. Its specification is `docs/spec/first-playable.md`, amended by `docs/spec/version-0.02.md`, `docs/spec/version-0.03.md`, `docs/spec/version-0.04.md`, `docs/spec/version-0.05.md` and `docs/spec/version-0.05.5.md`.
_Avoid_: slice, MVP, demo, v1, prototype

**Faction**:
A competing power with its own multipliers, signature rule and victory condition. The finished game has six; the First Playable has four since version 0.05, the Custodians, the Prospectors, the Arkwrights and the Archivists, and all four sit at every table.
_Avoid_: side, team, empire

**Custodians**:
The Faction that colonizes the solar system while limiting ecological damage to Earth. Their signature rules are the Scrubber, Leapfrog and, since version 0.06.0, Production Moved, and they win only if Earth is still habitable.
_Avoid_: Stewards (the retired name), environmentalists, greens, moderates, eco-terrorists

**Prospectors**:
The Faction that maximizes resource extraction without regard for ecological cost. Their signature rule is Cheap Industry and the Strip Permit, and since version 0.05.5 they win on a hoard: 750 Materials in their Venture Capital Fund, and expansion.
_Avoid_: Extractors (the retired name), capitalists, industrialists, exploiters

**Arkwrights**:
The Faction that exists to get people off Earth and spread them as widely as it can. Their signature rule is Steerage, and they win on Diaspora.
_Avoid_: settlers, arks, exodus, nomads

**Archivists**:
The Faction that means to save what humanity knows, and as many of its people as it can, in one place off Earth. Their signature rule is Provisional Findings, and they win by completing the Archive.
_Avoid_: scholars, librarians, scientists, the Academy

**Steerage**:
The Arkwrights' signature rule: their Colony Ships carry twice the Colonists and cost less to build, and they muster eight Emigrants a turn where others muster four, but every Emigrant costs their Nation State twice the population.
_Avoid_: mass transit, cattle class, overcrowding, packing them in

**Diaspora**:
The Arkwrights' Victory Condition: thirty Colonists living off Earth, spread over at least three Bodies with at least four Colonists on each. Antarctica counts toward neither part; since version 0.06.0 Colonists on a station over Earth count toward the thirty, but Earth is never one of the three Bodies.
_Avoid_: spread, expansion, exodus, colonization score

**Project**:
Retired in version 0.05.5. The word named a construction raised in stages, each paid in Research and Materials; the Archive was the only one, and it is a Module now.
_Avoid_: project, megaproject, wonder, great work, campaign, stage

**The Archive**:
A Module only the Archivists build, at one Colony off Earth and at most one per Faction, three turns to raise from its own button. Standing, it still wants its Research, paid into the Archive fund at any pace; complete, with every point paid, it takes a great deal of Energy to keep running. It is destroyed outright if its Colony changes hands, and the fund is kept.
_Avoid_: library, vault, monument, database, stage

**Fund the Archive**:
The Archivists' Orders-phase order, which sends a turn's Research from their own Labs into their Archive fund instead of the shared Tech, where it counts nothing toward the Research Lead. Until the Archive stands the fund holds only a quarter of what the Archive requires, and at that cap the order is refused.
_Avoid_: donate, invest, bank research, save up

**Provisional Findings**:
The Archivists' signature rule: they already have half the effect of the Tech under research, so long as their Research went to the shared Tech last turn rather than to the Archive.
_Avoid_: early access, preview, partial tech, head start

**Climate Model**:
The small model of global warming the game runs: Emissions add to the CO2 Stock, the Temperature follows the stock with a lag, and the Temperature acts on population and Events. It moves as a consequence of player and faction choices, never on a fixed schedule.
_Avoid_: warming track, doom clock, countdown, timer, disaster meter

**Body**:
A place in the solar system that can hold a Colony or a Space Station. Five since version 0.04: Earth (its Colony Slots are Antarctica's, shut under the ice until +1.6 C), the Moon, Mars, Phobos and Deimos. A Body may be another's satellite, which sets how far apart they are.
_Avoid_: planet, world, site, location, node

**Colony**:
A permanent settlement a Faction holds on a Body, founded when a Colony Ship unloads Colonists into a free Colony Slot. It takes the name of its slot ("Tycho on the Moon"). None exist when the game starts.
_Avoid_: base, outpost, settlement

**Colony Slot**:
One of a fixed number of places on a Body where a Colony can be founded, shared by all Factions. Since version 0.04 each is a real geological place, drawn at its position on the Surface Map and giving its Colony its name. Since version 0.05 each also has its own four yields, drawn when the game starts and never far from its Body's, so no two places on a world are equally worth settling; a free slot shows what a Colony there would get. A landing Colony Ship is founded into a chosen one. Earth's three, in Antarctica, are shut under the ice until the Temperature has stood at +1.6 C in a Climate phase; once open they stay open.
_Avoid_: site, plot, capacity

**Orbital Slot**:
One of a fixed number of places in orbit around a Body where a Space Station can be built, shared by all Factions, each with a station's name ready for it.
_Avoid_: dock, berth, orbit

**Space Station**:
A Colony in orbit, built for Materials into an Orbital Slot with no crew, holding only a Shipyard, Habitats and, since version 0.06.0, Observatories and Solar Arrays. Influence, Occupation and Battles work on it as on a Colony. Three Factions start with a bare one over Earth (the Custodians the ISS, the Prospectors Tiangong, the Archivists Axiom); the Arkwrights start with none, and build theirs at half price. Since version 0.06.0 a station over Earth is off Earth: its Colonists count for Off-world Presence and it may hold the Archive. A station is also the one place a Ship of its Faction can refuel.

**Tank**:
The Fuel a Ship carries, since version 0.06.0: a figure per Ship type, filled at the Shipyard for Fuel paid at the build, spent by transits, and refilled only by a Refuel. A Ship whose Tank cannot pay any leg from where it stands, with no Space Station of its Faction there, is stranded until one is built in orbit there.
_Avoid_: fuel tank, propellant, range, fuel level

**Build Where You Dig**:
The rule, since version 0.06.0, that a Module built at a Colony with a working Mine costs less: three quarters of its price with one Mine, three fifths with two or more, on top of the Faction's own discount and never below half the row. The Archive takes it; a Ship built at a Shipyard there and a Space Station built into orbit do not. A mothballed Mine, or one still building, counts for nothing.
_Avoid_: in-situ discount, local build bonus, mining discount

**Refuel**:
The Orders-phase order that fills a Ship's Tank from the Stockpile, as far as the Stockpile can pay, at a Body where the Ship's Faction holds a Space Station.
_Avoid_: resupply, top up, tanker
_Avoid_: base, platform, orbital, outpost

**Trading window**:
Where Ducats buy Influence, Materials, Fuel and Energy at table prices, spendable in the same turn's orders, and where Materials and Fuel sell back at half. A building can also be bought outright for Ducats from its own build button, at twice its Materials cost.
_Avoid_: market, shop, exchange, store

**Body Surface Map**:
The 3D view of one Body's surface, entered by clicking that Body on the Solar System Map. Every Body has one.
_Avoid_: planet view, ground view, zoomed view

**Earth Map**:
Earth's Body Surface Map, divided into Nation States, where Earth-side building, Army orders and the Climate Panel are seen and acted on.
_Avoid_: home view, globe view, terrestrial map

**Solar System Map**:
The 3D view of the solar system, where Bodies, Ship stacks, transits and Orbital Control are seen and acted on. Transits, launches, Ship Stances and attacks are ordered here and nowhere else. Since version 0.05 the sky it draws is the real one: Earth and Mars stand on their rings where they truly stand in the month the turn is, so the distance between them, and the Launch Window, are things the player can see.
_Avoid_: space view, orbital map, star map

**Launch Window**:
The months in which the flight between the Earth system and the Mars system is cheap, because the two worlds stand where a minimum-energy transfer wants them. On the window a crossing takes the shortest flight there is and costs the Fuel on the card; away from it it takes longer and costs more, the further off the dearer, up to a limit. There is one window in the game. A hop inside the Earth system or inside the Mars system does not have one.
_Avoid_: transfer window, launch period, alignment, conjunction

**Event**:
An unplanned occurrence drawn from the Event Deck after orders are committed and taking effect during Resolution — a solar storm, an equipment failure, a discovery. It can never directly break a Victory Condition.
_Avoid_: incident, crisis, card

**Event Deck**:
The deck a card may be drawn from each turn, holding only Events: forty cards since version 0.05.5 for a game of thirty-six turns (twenty-eight before), twenty-two Events in one to three copies each. It is never reshuffled. Whether a card is drawn at all is the Draw Chance.
_Avoid_: event pool, random table, encounter deck, calm card (retired in version 0.02)

**Draw Chance**:
The chance each turn that a card is drawn from the Event Deck: half at +1.2 C, rising a little for every full fifth of a degree the Temperature stands above it. It replaced the Calm Cards of the First Playable, so the danger in a turn is a percentage rather than a count of blanks.
_Avoid_: event probability, calm cards, event rate

**Tech**:
An advance on the Tech Tree that changes an output, a capacity, an upkeep, a Ship strength, an Influence cost or how much a source emits. When a Tech completes, every Faction has it.
_Avoid_: research, upgrade, invention

**Tech Tree**:
The single tree of Techs shared by all Factions, seventeen of them since version 0.06.0, in five branches: Industry, Propulsion, Off-world Living, Extraction, Society. A branch may hold more than one Tech on a rung. One Tech is under research at a time, worldwide. Four of the seventeen are Victory gates, one per Faction on rung 3: each is a Tech for everyone, and its Faction cannot win until it stands.
_Avoid_: per-faction tree, research tree

**Research Lead**:
The Faction that contributed the most Research to the Tech that just completed. It chooses the next Tech. Decided afresh for every Tech, and the race for it stands in the top bar as one bar of the four Factions' contributions in their own colours.
_Avoid_: science leader, tech leader

### Resources

**Materials**:
Raw metal and ore, spent to build ships, habitats and mines.
_Avoid_: minerals, supplies, ore

**Fuel**:
What is burned to move between Bodies. Since version 0.06.0 a transit spends it from the Ship's own Tank, which is filled at the Shipyard and refilled only by a Refuel order at a Body with a Space Station of the Ship's Faction; the Stockpile holds what Refineries make and the Trading window sells, and only a Refuel or a build moves it into a Tank. A lift from Earth spends none.
_Avoid_: propellant, rocket fuel

**Energy**:
What runs a mine, habitat or industry where it stands, drained every turn it operates.
_Avoid_: power, electricity

**Stockpile**:
The single shared pool holding all Materials, Fuel, Energy and, since version 0.03, Ducats. It is not divided by Body, so ore mined on Mars is immediately spendable on Earth. Since version 0.06.0 its Fuel reaches a Ship only through a Refuel at a Space Station of the Ship's Faction, or at the build.
_Avoid_: central bank, per-world stocks, inventory

**Ducats**:
Money, the fourth resource since version 0.03. A controlled Nation State pays them from its GDP figure times its Industry Level; a Bank on Earth and a Trade Post in a Colony make more. They buy Influence at two for one, added to this turn's Allotment, and pay for Relief, Resettle, a Leapfrog and repair points in place of Materials.
_Avoid_: Ducketts, credits, money, gold, cash

**Bank**:
The Facility that makes Ducats in a Nation State, in proportion to the state's GDP.
_Avoid_: treasury, mint, exchange

**Trade Post**:
The Module that makes Ducats in a Colony, in proportion to the Body's Habitat yield: trade goes where people live.
_Avoid_: market, exchange, shop

**Embassy**:
The Facility that raises Influence on Earth: while it stands and is online it adds to its controller's Allotment and raises its state's Standing for its controller each turn. Any number may stand in one state.
_Avoid_: consulate, ministry, propaganda office

**Relay**:
The Module that raises Influence off Earth: it adds to its holder's Allotment and raises its Colony's Standing for its holder each turn.
_Avoid_: antenna, transmitter, beacon

### Pieces

**Observatory**:
The Module that makes Research away from Earth, since version 0.06.0: on a Colony, a Space Station or in Antarctica. It takes no Body yield; every Colonist living at its Colony adds one per cent to what it makes, and Public Science lifts it as it lifts a Research Lab. Its Research counts toward the Research Lead as a Lab's does.
_Avoid_: lab, laboratory, research station, institute, science module

**Solar Array**:
The Module only a Space Station holds, since version 0.06.0. It makes Energy that follows the sunlight where the station is: six a turn at Earth's distance, less than half that over Mars, nearly twice it over Venus, by the inverse square of the Body's mean distance from the Sun. Efficient Grids lifts it as it lifts a Generator, a Solar Storm turn silences it, and any number may stand on one station.
_Avoid_: solar panel, power satellite, collector

**Module**:
A building placed inside a Colony. Eleven kinds since version 0.06.0: Mine, Generator, Refinery, Habitat, Shipyard, Barracks, Trade Post, Relay, the Observatory, the Solar Array (a Space Station's alone), and the Archive, which only the Archivists raise, one to a Faction, from its own button. A Habitat holds eight Colonists since version 0.06.0.
_Avoid_: building, structure, facility, improvement

**Ship**:
A persistent piece that travels between Bodies. It is one of four types since version 0.04: Colony Ship, Carrier, Frigate or Battleship. It is built only at a Shipyard, on a Space Station or a Colony. It is not consumed on arrival; damage it takes persists until repaired.
_Avoid_: vessel, rocket, fleet, expedition

**Colony Ship**:
The Ship type that carries Colonists, and nothing else since version 0.04. It cannot attack and is weak if caught. Since version 0.06.0 a warming Earth crowds it: lifting at Earth it may take Colonists beyond its capacity, one for every fifth of a degree the Temperature stands above +1.8, at most four, and each of those crowded aboard may die when it arrives.
_Avoid_: transport, colony (that is the settlement), settler ship

**Crowding**:
The extra Colonists a warming Earth puts aboard a Colony Ship lifting at Earth, beyond its capacity, since version 0.06.0, and the risk they run: rolled once at arrival, each crowded Colonist dies with a chance that grows with the size of the crowd. The player chooses whether to take them; the sea to Antarctica carries no crowd.
_Avoid_: overloading, steerage (that is the Arkwrights' rule), refugees aboard

**Carrier**:
The Ship type that carries one Army and nothing else, since version 0.04. Unarmed, it needs an escort and is a target for Intercept like a Colony Ship. Every landing needs one.
_Avoid_: troopship, transport, landing ship

**Frigate**:
The light warship type: cheap, with high Pursuit. It intercepts arriving Ships and runs down units that disengage.
_Avoid_: escort, corvette, destroyer

**Battleship**:
The heavy warship type: the most strength and hit points, low Pursuit, dear and slow to build. Since version 0.04 it carries no Army.
_Avoid_: capital ship, dreadnought, cruiser

**Army**:
A ground fighting unit. A Nation State's Armies belong to the state and are directed by the Faction that controls it, following the state if control changes. A Colony's Army belongs to the Colony, exists only where it has a Barracks, and only defends.
_Avoid_: troops, soldiers, garrison, marines

**Standing Army**:
The Armies a Nation State keeps on its own, sized by its card and replenished a little each turn, whether or not any Faction controls it.
_Avoid_: garrison, militia, defence value

**Barracks**:
The Module that lets a Colony hold and build a defensive Army. A Colony without one has no defenders.
_Avoid_: fort, garrison, base

**Colonist**:
A person counted in the population of a Colony or Nation State. Since version 0.05.5 Colonists are built: they muster in a Nation State as Emigrants, are carried by Ships and held by Habitats, and are never spent as a resource.

**Emigrant**:
A Colonist mustered in a Nation State and not yet gone: up to four a turn per Faction (eight for the Arkwrights), in one state the Faction directs, at a tenth of a person each, on the state's card at End Turn. A batch takes half a point off the state's Unrest. A working Launch Site lifts Emigrants onto a Ship, and once the Antarctic ice is open the sea takes them to Antarctica in a turn with no launch. They are people of their state until they leave it.
_Avoid_: settler, recruit, migrant, passenger, colonist-in-waiting
_Avoid_: settler, crew, worker, population resource

**Nation State**:
One of twelve regions of the Earth Map that a Faction can control and build in. Eight until version 0.05, which split Africa at the Sahara, Asia into East, South and South-East, and Central America and the Caribbean out of North America; Antarctica left the list in version 0.04 to become Earth's Colony Slots. Each carries a population, an Industry Level, a Resource Lean, a Baseline Emissions figure, an Education Level, a GDP, an Influence value, an Unrest figure and a Standing Army. Armies move only between neighbouring states.
_Avoid_: country, nation, territory, region, state, continent (some are, some are not)

### Earth

**Facility**:
A building placed in a Nation State. Ten kinds since version 0.05: Factory, Power Plant, Refinery, Launch Site, Research Lab, Bank, Embassy, Constabulary, and the Sea Wall and the Scrubber, which take no build slot (the Sea Wall stood in a Coastal Slot until version 0.05.5). Since version 0.04 a Launch Site builds no Ship: it lifts Emigrants and Armies from its state into orbit, and each lift is a launch.
_Avoid_: item, building, module (that is the Colony word), structure

**Cheap Industry**:
The first clause of the Prospectors' signature rule: raising a Nation State's Industry Level costs them half, and since version 0.05.5 every Facility and Colony Module costs them 15% less. The second is the Strip Permit.
_Avoid_: industry discount, cheap building

**Restoration**:
Retired in version 0.05; see Scrubber. It was the Custodians' signature rule: Energy spent in a turn enlarged the Natural Sink for that turn only.
_Avoid_: using it for anything the Scrubber now does

**Scrubber**:
The Custodians' signature Facility, which only they build and only in a Nation State they control. It takes no build slot, draws Energy, emits nothing, and while it is online it enlarges the Natural Sink and lowers its state's Unrest every turn. How many one state may hold follows its population, and they are destroyed outright if the state changes hands.
_Avoid_: carbon capture, cleanup, restoration (the retired rule), terraforming, filter

**Leapfrog**:
The Custodians' other signature clause: an Orders-phase order paid in Ducats on a Nation State they control, which lowers that state's per-person Emissions coefficient by one Industry Level's worth, for good. It may be bought any number of times, and never takes the figure below the base every state pays.
_Avoid_: clean development, technology transfer, offset, upgrade

**Strip Permit**:
The Prospectors' other signature clause: an Orders-phase order, free, that may be taken once in a Nation State's whole life. For three turns every Facility there produces double; when it ends, that state's Baseline Emissions and its Unrest rise for good.
_Avoid_: licence, boom, overdrive, exploitation

**Mothball**:
The Orders-phase order that stands a Facility or a Module down. A mothballed building produces nothing, pays no Energy upkeep, emits nothing, is online for no rule, and keeps its slot. It is free, it takes effect at the Resolution, it raises the Unrest of a Nation State it happens in, and only a Restart brings the building back, for Materials and a turn.
_Avoid_: pause, disable, switch off, idle, shut down (that is the Energy shortfall rule)

**Production Moved**:
The Custodians' third signature rule, since version 0.06.0. While a Factory, Power Plant, Refinery or Research Lab of theirs on Earth is mothballed, one Mine, Generator, Refinery or Observatory of theirs off Earth makes double, one Facility for one Module, the most productive undoubled Module first, on its final figure. A Restart ends it; with no idle Facility of the pair there is no bonus.
_Avoid_: offshoring, relocation bonus, the mothball bonus

**Decommission**:
The Orders-phase order that takes a Facility or a Module down for good: a turn later half its Materials come back, its slot is free, and it is gone. In a Nation State it raises Unrest more than a Mothball does; in a Colony it raises none.
_Avoid_: demolish, scrap, sell, destroy

**Industry Level**:
How built-up a Nation State is. Together with the state's size it sets how many Facilities fit, and every slot it adds is an Inland Slot. It scales the state's emissions. Raising it is a build action.
_Avoid_: development, tier, infrastructure

**Neutral Development**:
What a Nation State nobody holds does for itself: every ninth turn of unbroken neutrality it raises its own Industry Level by one and wakes one of its own Facilities to run itself, emitting to nobody and making nothing for anyone, until it is as built-up as a neutral state gets. A world already too hot, or a population already too restive, stops it. The clock is the state's own, starts again whenever it is freed, and the states neutral at the start are staggered so they do not all step together.
_Avoid_: growth, expansion, auto-build, industrialization, AI development

**Education Level**:
A fixed figure on a Nation State's card, taken from real-world values in the First Playable, that multiplies the Research each Lab in that state produces.
_Avoid_: literacy, science level, schooling

**Resource Lean**:
The one of Materials, Fuel or Energy a Nation State is naturally good at producing.
_Avoid_: specialty, bonus, affinity

**Baseline Emissions**:
How dirty a Nation State's industry is before any Facility is built there.
_Avoid_: pollution rating, carbon score

**Build Slot**:
One of the places a Facility occupies in a Nation State. A state has Size + Industry Level + three of them, and each is either a **Coastal Slot** or an **Inland Slot**: the sea takes only the first, and every slot a raise of the Industry Level adds is one of the second. Its start Facilities stand on the coast first; a later build fills an inland slot while one is free.
_Avoid_: building slot, plot, space, capacity

**Coastal Slot**:
A build slot on a Nation State's coast, where the sea can reach it. A state has three per point of Coastal Exposure, never more than its start slots less one, and never gains another. A Facility standing in one the sea takes is destroyed, oldest first.
_Avoid_: shore slot, waterfront, flood zone

**Inland Slot**:
A build slot the sea never reaches: the start slots the coastal ones leave over, and every slot a raise of the Industry Level adds.
_Avoid_: safe slot, interior, highland

**Coastal Exposure**:
How much of a Nation State stands on the coast, a figure on its card. It sets how many Coastal Slots the state has, two per point since version 0.05.5 (three before), and how many of them each Sea Level threshold takes. A state with no Coastal Slots left loses no more of them, though the threshold still drives out its people and raises its Unrest.
_Avoid_: coastline, vulnerability, flood risk

**Sea Wall**:
The Facility that holds one threshold off: it takes no build slot (since version 0.05.5; a Coastal Slot before), at most one to a state, and while it is working the state's next Sea Level threshold of any kind takes no slots at all. The wall is destroyed absorbing it. It needs Coastal Engineering.
_Avoid_: dyke, levee, barrier, flood defence

**Coastal Engineering**:
The Industry Tech that unlocks the Sea Wall, and does nothing else.
_Avoid_: sea defence, civil engineering, hydrology

**Unrest**:
How restive a Nation State's people are, a figure from 0 to 10 on its card, moving in halves. Heat, the rising sea, the Climate cards, Occupation and arriving Refugees raise it; it falls on its own every turn except the turn the state changed hands, so a rise and the fall net out, and it falls further to Relief, a Constabulary and a Scrubber. Past its first threshold the Standing Army stops replenishing, past its second the state's Facilities run at half, and at the top the state throws its controller off and goes neutral.
_Avoid_: unhappiness, morale, stability, dissent, revolt meter

**Relief**:
The Orders-phase order that buys a Nation State's calm: Ducats spent on a state you direct to lower its Unrest by one, as many times in a turn as you can pay for.
_Avoid_: aid, welfare, subsidy, bribe, appeasement

**Constabulary**:
The Facility that holds a Nation State's Unrest down: while it stands and is online it lowers Unrest every turn whatever else happened that turn, and softens what the climate and arriving Refugees add. At most one stands in a state.
_Avoid_: police, militia, garrison, barracks (that is the Colony word)

**Refugees**:
The people who leave a Nation State for its neighbours when the heat or the sea takes their homes, instead of simply being lost. They are added to the state that receives them, and their arrival raises its Unrest.
_Avoid_: migrants, displaced, evacuees, exodus, immigration

**Resettle**:
The Orders-phase order that steers Refugees: once a turn, for Ducats, every flow leaving the states a Faction directs goes to one Nation State of its choosing instead of to the neighbours, and its Standing there rises.
_Avoid_: relocate, evacuate, transfer, deport

**Blame**:
The CO2 a Faction is answerable for over the whole game: everything the sources it controlled has emitted, less everything it has taken back, and never less than nothing. What no Faction controls is nobody's. A Faction's share of the four Factions' Blame, when it rises above a fair quarter, makes every Nation State it does not hold harder for it to win over.
_Avoid_: carbon debt, guilt, pollution score, emissions total, footprint

**Influence**:
A Faction's claim on a Nation State or a Colony, spent from a per-turn Allotment onto a place, where it becomes the Faction's Standing there. A neutral place goes to the first Standing at its threshold, and when two claimants reach it on the same turn at the same Standing the lot decides between them (since version 0.05.5); a controlled place goes to a rival whose Standing is at least the controller's plus the challenge margin (20 since version 0.05.5, 10 from version 0.04) and at least the threshold, which since version 0.05 each Faction reads for itself, since its Blame raises it on every Nation State it does not hold. A holder is never tied with a challenger, and keeps the place when two challengers tie. Since version 0.05.5 every Faction begins with a Standing on its start state equal to that state's threshold: a claim on its home from turn 1.
_Avoid_: diplomacy points, favour, reputation

**Standing**:
How much Influence a Faction has built up on one place. Since version 0.03 it persists: it is never wiped when the place changes hands, it decays 1 a turn on a place the Faction controls and 2 a turn elsewhere when nothing is spent, and spending on a place you hold raises it.
_Avoid_: accumulation, influence points, loyalty

**Allotment**:
The amount of Influence a Faction receives each turn, split freely across any number of targets during the Orders phase. It is a base plus the Influence value of every Nation State the Faction controls (since version 0.03 each state carries its own value, from its economic and military weight), and it does not carry over.
_Avoid_: influence budget, diplomacy pool, action points

**Research**:
Points produced by Research Labs on Earth and, since version 0.06.0, by Observatories at Colonies and Space Stations, and spent only on Techs. Held outside the Stockpile; it is not one of the three resources and never buys a Facility, Module or Ship. Since version 0.05.5 a Lab in a Nation State nobody holds, or one under Occupation, runs itself and pays half its yield into the Tech under research for no Faction; North America and South-East Asia begin with such a Lab.
_Avoid_: science, research points, RP, fourth resource

### The turn

**Turn**:
Two calendar months of game time since version 0.05.5 (one before), and the unit the whole game runs in. The game begins in January 2030 and runs thirty-six turns, so turn 1 is January 2030, turn 2 March 2030 and the thirty-sixth November 2035; a turn is named by its first month alone, and the top bar names it. All four Factions order simultaneously against the same board, then the turn runs through its phases.
_Avoid_: round, month, tick, cycle

**Phase**:
One of the seven stages a turn runs through, always in this order: Income, Climate, Report, Orders, Event, Resolution, End.
_Avoid_: step, stage, segment

**Orders**:
The phase in which a Faction commits everything it will do that turn: building, raising Industry Level, allocating the Influence Allotment, launching, setting transits and Stances, and ordering attacks and landings. Any order can be cancelled and fully refunded until End Turn is pressed.
_Avoid_: commands, moves, actions, player phase

**End Turn**:
The commitment. Pressing it closes the Orders phase, after which the Event card is drawn and nothing can be taken back.
_Avoid_: submit, confirm, next turn

**Resolution**:
The phase in which the turn actually happens: transits advance, arrivals land, Battles resolve, Occupation counts down, control transfers, builds complete, repairs finish, and Colony Ships that unload found Colonies.
_Avoid_: processing, execution, upkeep phase

**Report**:
The phase that opens a turn for the player, and the dated dispatch it shows. It opens with a headline: the most serious thing that happened, chosen by a fixed order of severity. Everything else is grouped under four headings, In space, On Earth, The climate and Your works, and an empty heading is left out. Every line that is about somewhere is a way there. It ends with what each rival Faction did, told in plain sentences of what the board could see.
_Avoid_: summary, news, digest

**Moment**:
A short modal that stops the turn before the Report, for one sentence and one number: a Colony founded, a Break or the sea rising, a Battle that cost a unit, a place changing hands, a Tech completed, Antarctica opening, the Archive finished, or Colonists lost in transit. At most two a turn, the most serious first, and every kind can be switched off.
_Avoid_: popup, alert, notification, cutscene, interruption

**Spectator**:
Someone watching a game they hold no seat in. The computer plays all four Factions and the interface gives no orders at all; in return every Faction's board, card and Standing is open to be read. End Turn advances one turn, and the turns can be set to run on their own.
_Avoid_: observer, replay, demo

**Save**:
A turn start written to a file, holding everything about one game: the same seed, the same deck, the same board. The game writes one of its own every three turns and when the game ends, keeping the last three of a game, and the player may write one at any turn start where nothing has been ordered yet. Saves are loaded from the title screen, and one written by another version of the rules is refused rather than brought forward.
_Avoid_: checkpoint, snapshot, savegame

### Combat

**Battle**:
A melee at one Body, Nation State or Colony in which every Faction present is hostile to every other, resolved automatically in rounds during end-of-turn processing and always finished inside the turn. Each round a party's chance to hit is its share of the total strength present, and its hits are spread across the other parties in proportion to theirs.
_Avoid_: fight, engagement, skirmish, encounter

**Stance**:
The one order a stack carries into end-of-turn processing: Attack, Hold, Intercept (Ships only, engaging arrivals before they unload) or Evade.
_Avoid_: order, mode, posture, aggression setting

**Strength**:
How hard a unit hits. A unit's type sets it; Hardened Hulls raises it for every Ship.
_Avoid_: attack, power, combat value

**Hit Points**:
How much damage a unit can take before it is destroyed with everything it carries. Damage persists until repaired at a Shipyard or Launch Site (Ships) or in a controlled Nation State or a Colony with a Barracks (Armies).
_Avoid_: health, HP, hull, morale

**Pursuit**:
A unit's ability to catch an enemy unit that disengages, forcing it to take one more round of fire. Frigates have the most; Colony Ships none.
_Avoid_: speed, chase, initiative

**Disengage**:
A damaged unit's attempt to leave a Battle, more likely the more damage it carries. A unit that disengages and is not caught survives and cannot be attacked again that turn.
_Avoid_: retreat, rout, flee, break

**Battle Report**:
The account of every Battle from the last end-of-turn processing, read at the start of the next turn.
_Avoid_: combat log, after-action report

**Orbital Control**:
Held at a Body by a Faction that has a Frigate or Battleship there with no enemy warship still engaged. Armies and Colonists can land only where their Faction holds it or nobody contests it.
_Avoid_: blockade, orbital supremacy, space superiority

**Occupation**:
The state of a Nation State or Colony whose defenders were beaten by an Army. The occupier chooses build orders but does not direct its Armies; control transfers after the lesser of three turns or the population being Pacified.
_Avoid_: conquest, annexation, capture

**Pacified**:
An occupied population whose occupier's Influence, gained automatically each turn of Occupation, has passed the place's threshold. The gain is halved while the state's Unrest is past its first threshold. Control transfers at the moment the threshold is passed.
_Avoid_: subdued, loyal, converted

### Winning

**Victory Condition**:
What one Faction must achieve to win. Each Faction has its own, and meeting it in an End phase wins the game at once; since version 0.06.0 it is not met until the Faction's Victory gate, a Tech of its own on the Tech Tree, stands, though every part of it accrues before that; if more than one seat meets it in the same phase, the larger margin over its own bar wins and an exact tie is a draw. If no Faction has met its condition by the end of the last turn, the seats are ranked by the percentage of their own condition, then by Colonists off Earth, then by Colonies held.
_Avoid_: win condition, goal, objective, victory points

**Extraction Total**:
Retired in version 0.05.5 for the Venture Capital Fund. It was the Prospectors' measure: all the Materials and Fuel their Mines, Refineries and Factories had produced across the whole game, counted cumulatively and never spent down.
_Avoid_: extraction total, production score, output total, wealth

**Venture Capital Fund**:
The Prospectors' own pool beside the Stockpile, and their measure since version 0.05.5: 750 Materials in it is the first part of their Victory Condition. On any turn they set the share of their Factories' and Mines' Materials output that goes into it at Income, from nothing to four fifths in steps of a tenth; Materials bought, refunded or found are not output. A draw takes Materials back out at a tenth's loss.
_Avoid_: the pool, savings, treasury, war chest, bank

**Stabilization**:
The Custodians' measure: net Emissions held under the Natural Sink, counting every Scrubber, for a run of consecutive turns. One turn over the Sink resets the run.
_Avoid_: carbon neutral, balance, equilibrium

**Off-world Presence**:
The number of Colonists living away from Earth, required by the Custodians' and the Prospectors' Victory Conditions. No Faction can win on Earth alone. Colonists in Antarctica are still on Earth; since version 0.06.0 those on a station over Earth are off it, as is everything else that asks "off Earth": the Archive's place and the Archivists' Research.
_Avoid_: population off Earth, colony size, settlers

### Climate

**CO2 Stock**:
The amount of CO2-equivalent in Earth's atmosphere, in parts per million. Emissions add to it each turn; the Natural Sink takes a little away.
_Avoid_: pollution, carbon level, warming points

**Temperature**:
Degrees above pre-industrial. It follows the CO2 Stock with a lag of one to two turns and is what actually harms population and drives Events.
_Avoid_: heat, warming percentage

**Emissions**:
The CO2-equivalent a source adds to the CO2 Stock in a turn. Every source has its own figure and the Climate Panel shows them one by one before the sum; methane-heavy sources carry a heavier weight. Since version 0.05 a Nation State's people emit more the more built-up the state is, and a Leapfrog lowers that state's figure for good.
_Avoid_: output, carbon, footprint

**Natural Sink**:
The amount of CO2 the oceans and forests remove from the CO2 Stock every turn. Net emissions below it stabilize the stock. Every Scrubber standing and online enlarges it while it stands, and since version 0.05 a Break can weaken it for good.
_Avoid_: absorption, offset, carbon capture

**Sea Level**:
How far the oceans have risen with the Temperature. It is drawn on the globe as a creeping waterline, and at each of its thresholds it permanently takes Coastal Slots from every Nation State, as many as the state's Coastal Exposure and never more than it has left. It takes nothing else: an Inland Slot is out of its reach, and so is a state whose coast is already gone.
_Avoid_: flooding, water line, ocean rise

**Break**:
A Temperature at which a permanent change to the world fires once, the first time the Temperature stands at or above it. Five since version 0.05: the reefs die, the permafrost thaws, the Natural Sink weakens, the ice sheets go and the Amazon dies back. Nothing undoes a Break, and the Report says it happened rather than that it is coming.
_Avoid_: tipping point, threshold, trigger

**Committed Warming**:
The Temperature the CO2 Stock as it stands will deliver once the lag has caught up. It is what the world has already bought, whatever anybody builds or stops building.
_Avoid_: locked-in warming, pipeline, inertia, baked in

**Last Turn**:
The latest turn on which cutting net Emissions to zero from that turn onward still keeps the Temperature under the Collapse Line by the last turn, counting every Break the world would cross on the way. The Climate Panel names it, or says that cuts alone no longer avoid Collapse, or that Collapse is not reached on this path.
_Avoid_: deadline, point of no return, countdown

**Collapse Line**:
The one Temperature at which the game ends with nobody winning, unless a Faction had already met its Victory Condition in an earlier End phase. Every other effect of Temperature is continuous; this is the only line.
_Avoid_: threshold, tipping point, game over temperature

**Climate Panel**:
The screen showing the CO2 Stock, the Temperature and where it is heading, a Temperature bar notched with every Break, every Sea Level threshold and Antarctica's opening, this turn's Emissions by source, the sink and the net, the penalties in force, the Committed Warming, the Last Turn, and a projection to the last turn.
_Avoid_: warming meter, climate HUD
