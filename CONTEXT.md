# Dying Earth

A single-player, turn-based strategy game about colonizing the solar system before ecological collapse overtakes Earth. It should feel like a strategy board game rather than an action game.

## Language

**First Playable**:
The small version of the game covering only the Moon and Mars, with three resources and two factions: twelve turns as first built, twenty-four since version 0.02. Its specification is `docs/spec/first-playable.md`, amended by `docs/spec/version-0.02.md`.
_Avoid_: slice, MVP, demo, v1, prototype

**Faction**:
A competing power with its own multipliers, signature rule and victory condition. The finished game has six; the First Playable has two, the Custodians and the Prospectors, who differ in everything but combat and their starting position.
_Avoid_: side, team, empire

**Custodians**:
The Faction that colonizes the solar system while limiting ecological damage to Earth. Their signature rule is Restoration, and they win only if Earth is still habitable.
_Avoid_: Stewards (the retired name), environmentalists, greens, moderates, eco-terrorists

**Prospectors**:
The Faction that maximizes resource extraction without regard for ecological cost. Their signature rule is Cheap Industry, and they win on extraction and expansion.
_Avoid_: Extractors (the retired name), capitalists, industrialists, exploiters

**Climate Model**:
The small model of global warming the game runs: Emissions add to the CO2 Stock, the Temperature follows the stock with a lag, and the Temperature acts on population and Events. It moves as a consequence of player and faction choices, never on a fixed schedule.
_Avoid_: warming track, doom clock, countdown, timer, disaster meter

**Body**:
A place in the solar system that can hold a Colony or a Space Station. Five since version 0.04: Earth (its Colony Slots are Antarctica's), the Moon, Mars, Phobos and Deimos. A Body may be another's satellite, which sets how far apart they are.
_Avoid_: planet, world, site, location, node

**Colony**:
A permanent settlement a Faction holds on a Body, founded when a Colony Ship unloads Colonists into a free Colony Slot. It takes the name of its slot ("Tycho on the Moon"). None exist when the game starts.
_Avoid_: base, outpost, settlement

**Colony Slot**:
One of a fixed number of places on a Body where a Colony can be founded, shared by all Factions. Since version 0.04 each is a real geological place, drawn at its position on the Surface Map and giving its Colony its name. A landing Colony Ship is founded into a chosen one.
_Avoid_: site, plot, capacity

**Orbital Slot**:
One of a fixed number of places in orbit around a Body where a Space Station can be built, shared by all Factions, each with a station's name ready for it.
_Avoid_: dock, berth, orbit

**Space Station**:
A Colony in orbit, built for Materials into an Orbital Slot with no crew, holding only a Shipyard and Habitats. Influence, Occupation and Battles work on it as on a Colony. Each Faction starts with one over Earth (the Custodians the ISS, the Prospectors Tiangong), bare. Colonists on a station over Earth are still on Earth for Off-world Presence.
_Avoid_: base, platform, orbital, outpost

**Trading window**:
Where Ducats buy Influence, Materials, Fuel and Energy at table prices, spendable in the same turn's orders, and where Materials and Fuel sell back at half. A building can also be bought outright for Ducats from its own build button, at twice its Materials cost.
_Avoid_: market, shop, exchange, store
_Avoid_: site, plot, capacity

**Body Surface Map**:
The 3D view of one Body's surface, entered by clicking that Body on the Solar System Map. Every Body has one.
_Avoid_: planet view, ground view, zoomed view

**Earth Map**:
Earth's Body Surface Map, divided into Nation States, where Earth-side building, Army orders and the Climate Panel are seen and acted on.
_Avoid_: home view, globe view, terrestrial map

**Solar System Map**:
The 3D view of the solar system, where Bodies, Ship stacks, transits and Orbital Control are seen and acted on. Transits, launches, Ship Stances and attacks are ordered here and nowhere else.
_Avoid_: space view, orbital map, star map

**Event**:
An unplanned occurrence drawn from the Event Deck after orders are committed and taking effect during Resolution — a solar storm, an equipment failure, a discovery. It can never directly break a Victory Condition.
_Avoid_: incident, crisis, card

**Event Deck**:
The deck a card may be drawn from each turn, holding only Events: thirty cards in version 0.02, the twelve First Playable Events twice and six newer ones once. It is never reshuffled. Whether a card is drawn at all is the Draw Chance.
_Avoid_: event pool, random table, encounter deck, calm card (retired in version 0.02)

**Draw Chance**:
The chance each turn that a card is drawn from the Event Deck: half at +1.2 C, rising a little for every full fifth of a degree the Temperature stands above it. It replaced the Calm Cards of the First Playable, so the danger in a turn is a percentage rather than a count of blanks.
_Avoid_: event probability, calm cards, event rate

**Tech**:
An advance on the Tech Tree that changes an output, a capacity, an upkeep, a Ship strength, an Influence cost or how much a source emits. When a Tech completes, every Faction has it.
_Avoid_: research, upgrade, invention

**Tech Tree**:
The single tree of Techs shared by all Factions, in five branches: Industry, Propulsion, Off-world Living, Extraction, Society. One Tech is under research at a time, worldwide.
_Avoid_: per-faction tree, research tree

**Research Lead**:
The Faction that contributed the most Research to the Tech that just completed. It chooses the next Tech. Decided afresh for every Tech.
_Avoid_: science leader, tech leader

### Resources

**Materials**:
Raw metal and ore, spent to build ships, habitats and mines.
_Avoid_: minerals, supplies, ore

**Fuel**:
What is burned to move between Bodies. Launches and transits spend it; nothing else does.
_Avoid_: propellant, rocket fuel

**Energy**:
What runs a mine, habitat or industry where it stands, drained every turn it operates.
_Avoid_: power, electricity

**Stockpile**:
The single shared pool holding all Materials, Fuel, Energy and, since version 0.03, Ducats. It is not divided by Body, so ore mined on Mars is immediately spendable on Earth.
_Avoid_: central bank, per-world stocks, inventory

**Ducats**:
Money, the fourth resource since version 0.03. A controlled Nation State pays them from its GDP figure times its Industry Level; a Bank on Earth and a Trade Post in a Colony make more. They buy Influence at two for one, added to this turn's Allotment, and pay for Restoration steps and repair points in place of Energy and Materials.
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

**Module**:
A building placed inside a Colony. Eight kinds since version 0.03: Mine, Generator, Refinery, Habitat, Shipyard, Barracks, Trade Post and Relay.
_Avoid_: building, structure, facility, improvement

**Ship**:
A persistent piece that travels between Bodies. It is one of four types since version 0.04: Colony Ship, Carrier, Frigate or Battleship. It is built only at a Shipyard, on a Space Station or a Colony. It is not consumed on arrival; damage it takes persists until repaired.
_Avoid_: vessel, rocket, fleet, expedition

**Colony Ship**:
The Ship type that carries Colonists, and nothing else since version 0.04. It cannot attack and is weak if caught.
_Avoid_: transport, colony (that is the settlement), settler ship

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
A person counted in the population of a Colony or Nation State. Colonists are carried by Ships and held by Habitats; they are never spent as a resource.
_Avoid_: settler, crew, worker, population resource

**Nation State**:
One of eight regions of the Earth Map (six continents, plus Russia and the Middle East since version 0.02; Antarctica left the list in version 0.04 to become Earth's Colony Slots) that a Faction can control and build in. Each carries a population, an Industry Level, a Resource Lean, a Baseline Emissions figure, an Education Level and a Standing Army. Armies move only between neighbouring continents.
_Avoid_: country, nation, territory, region, state

### Earth

**Facility**:
A building placed in a Nation State. Seven kinds since version 0.03: Factory, Power Plant, Refinery, Launch Site, Research Lab, Bank and Embassy. Since version 0.04 a Launch Site builds no Ship: it lifts Colonists and Armies from its state into orbit, and each lift is a launch.
_Avoid_: item, building, module (that is the Colony word), structure

**Cheap Industry**:
The Prospectors' signature rule: raising a Nation State's Industry Level costs them half.
_Avoid_: industry discount, cheap building

**Restoration**:
The Custodians' signature rule: Energy spent in a turn enlarges the Natural Sink for that turn only.
_Avoid_: carbon capture, cleanup, scrubbing, terraforming

**Industry Level**:
How built-up a Nation State is. Together with the state's size it sets how many Facilities fit, and it scales the state's emissions. Raising it is a build action.
_Avoid_: development, tier, infrastructure

**Education Level**:
A fixed figure on a Nation State's card, taken from real-world values in the First Playable, that multiplies the Research each Lab in that state produces.
_Avoid_: literacy, science level, schooling

**Resource Lean**:
The one of Materials, Fuel or Energy a Nation State is naturally good at producing.
_Avoid_: specialty, bonus, affinity

**Baseline Emissions**:
How dirty a Nation State's industry is before any Facility is built there.
_Avoid_: pollution rating, carbon score

**Coastal Exposure**:
How much of a Nation State the sea can take. As Sea Level passes its thresholds, an exposed state permanently loses build slots.
_Avoid_: coastline, vulnerability, flood risk

**Influence**:
A Faction's claim on a Nation State or a Colony, spent from a per-turn Allotment onto a place, where it becomes the Faction's Standing there. A neutral place goes to the first Standing at its threshold; a controlled place goes to a rival whose Standing is at least the controller's plus the challenge margin (10 since version 0.04) and at least the threshold.
_Avoid_: diplomacy points, favour, reputation

**Standing**:
How much Influence a Faction has built up on one place. Since version 0.03 it persists: it is never wiped when the place changes hands, it decays 1 a turn on a place the Faction controls and 2 a turn elsewhere when nothing is spent, and spending on a place you hold raises it.
_Avoid_: accumulation, influence points, loyalty

**Allotment**:
The amount of Influence a Faction receives each turn, split freely across any number of targets during the Orders phase. It is a base plus the Influence value of every Nation State the Faction controls (since version 0.03 each state carries its own value, from its economic and military weight), and it does not carry over.
_Avoid_: influence budget, diplomacy pool, action points

**Research**:
Points produced by Research Labs and spent only on Techs. Held outside the Stockpile; it is not one of the three resources and never buys a Facility, Module or Ship.
_Avoid_: science, research points, RP, fourth resource

### The turn

**Turn**:
One month of game time, and the unit the whole game runs in. The First Playable is twelve turns long. Both Factions order simultaneously against the same board, then the turn runs through its phases.
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
The phase that opens a turn for the player, carrying the Battle Report, last turn's Event, completed builds, arrivals and control changes.
_Avoid_: summary, news, digest

### Combat

**Battle**:
A fight between opposed stacks at one Body, Nation State or Colony, resolved automatically in rounds during end-of-turn processing and always finished inside the turn. Each round each side rolls to land hits, biased by its share of the strength present.
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
An occupied population whose occupier's Influence, gained automatically each turn of Occupation, has passed the place's threshold. Control transfers at that moment.
_Avoid_: subdued, loyal, converted

### Winning

**Victory Condition**:
What one Faction must achieve to win. Each Faction has its own, and meeting it in an End phase wins the game at once. If neither Faction has met its condition by the end of the last turn, the higher percentage of its own condition wins.
_Avoid_: win condition, goal, objective, victory points

**Extraction Total**:
The Prospectors' measure: all the Materials and Fuel their Mines, Refineries and Factories have produced across the whole game. It is counted cumulatively and never spent down.
_Avoid_: production score, output total, wealth

**Stabilization**:
The Custodians' measure: net Emissions held under the Natural Sink for a run of consecutive turns. One turn over the Sink resets the run.
_Avoid_: carbon neutral, balance, equilibrium

**Off-world Presence**:
The number of Colonists living away from Earth, required by both Factions' Victory Conditions. Neither can win on Earth alone. Colonists in Antarctica or on a station over Earth are still on Earth.
_Avoid_: population off Earth, colony size, settlers

### Climate

**CO2 Stock**:
The amount of CO2-equivalent in Earth's atmosphere, in parts per million. Emissions add to it each turn; the Natural Sink takes a little away.
_Avoid_: pollution, carbon level, warming points

**Temperature**:
Degrees above pre-industrial. It follows the CO2 Stock with a lag of one to two turns and is what actually harms population and drives Events.
_Avoid_: heat, warming percentage

**Emissions**:
The CO2-equivalent a source adds to the CO2 Stock in a turn. Every source has its own figure and the Climate Panel shows them one by one before the sum; methane-heavy sources carry a heavier weight.
_Avoid_: output, carbon, footprint

**Natural Sink**:
The fixed amount of CO2 the oceans and forests remove from the CO2 Stock every turn. Net emissions below it stabilize the stock. Restoration enlarges it for a single turn.
_Avoid_: absorption, offset, carbon capture

**Sea Level**:
How far the oceans have risen with the Temperature. It is drawn on the globe as a creeping waterline, and it permanently takes build slots from Nation States according to their Coastal Exposure.
_Avoid_: flooding, water line, ocean rise

**Collapse Line**:
The one Temperature at which the game ends with nobody winning, unless a Faction had already met its Victory Condition in an earlier End phase. Every other effect of Temperature is continuous; this is the only line.
_Avoid_: threshold, tipping point, game over temperature

**Climate Panel**:
The screen showing the CO2 Stock, the Temperature and where it is heading, this turn's Emissions by source, the sink and the net, the penalties in force, and a projection to the last turn.
_Avoid_: warming meter, climate HUD
