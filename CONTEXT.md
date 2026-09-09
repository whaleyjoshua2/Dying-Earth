# Dying Earth

A single-player, turn-based strategy game about colonizing the solar system before ecological collapse overtakes Earth. It should feel like a strategy board game rather than an action game.

## Language

**First Playable**:
The twelve-turn version of the game covering only the Moon and Mars, with three resources and two factions. The destination of the current effort is a written specification for it.
_Avoid_: slice, MVP, demo, v1, prototype

**Faction**:
A competing power with its own goals, costs and rules. The finished game has six; the First Playable has two.
_Avoid_: side, team, empire

**Stewards**:
The faction that colonizes the solar system while limiting ecological damage to Earth.
_Avoid_: environmentalists, greens, moderates, eco-terrorists

**Extractors**:
The faction that maximizes resource extraction without regard for ecological cost.
_Avoid_: capitalists, industrialists, exploiters

**Climate Model**:
The small model of global warming the game runs: Emissions add to the CO2 Stock, the Temperature follows the stock with a lag, and the Temperature acts on population and Events. It moves as a consequence of player and faction choices, never on a fixed schedule.
_Avoid_: warming track, doom clock, countdown, timer, disaster meter

**Body**:
A place in the solar system that can hold a Colony. The First Playable has the Moon and Mars.
_Avoid_: planet, world, site, location, node

**Colony**:
A permanent settlement a Faction holds on a Body, founded when a Colony Ship unloads Colonists into a free Colony Slot. None exist when the game starts.
_Avoid_: base, outpost, settlement

**Colony Slot**:
One of a fixed number of places on a Body where a Colony can be founded, shared by all Factions. Slots are visible places on that Body's Surface Map, and a landing Colony Ship is founded into a chosen one.
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
An unplanned occurrence that interrupts a turn — a solar storm, an equipment failure, a discovery.
_Avoid_: incident, crisis, card

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
The single shared pool holding all Materials, Fuel and Energy. It is not divided by Body, so ore mined on Mars is immediately spendable on Earth.
_Avoid_: central bank, per-world stocks, inventory

### Pieces

**Module**:
A building placed inside a Colony. The First Playable has six kinds: Mine, Generator, Refinery, Habitat, Shipyard and Barracks.
_Avoid_: building, structure, facility, improvement

**Ship**:
A persistent piece that travels between Bodies. It is one of three types: Colony Ship, Frigate or Battleship. It is not consumed on arrival; damage it takes persists until repaired.
_Avoid_: vessel, rocket, fleet, expedition

**Colony Ship**:
The Ship type that carries Colonists and Armies. It cannot attack and is weak if caught.
_Avoid_: transport, colony (that is the settlement), settler ship

**Frigate**:
The light warship type: cheap, with high Pursuit. It intercepts arriving Ships and runs down units that disengage.
_Avoid_: escort, corvette, destroyer

**Battleship**:
The heavy warship type: the most strength and hit points, low Pursuit, dear and slow to build. It carries one Army.
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
One of seven regions of the Earth Map mirroring the continents, Antarctica included, that a Faction can control and build in. Each carries a population, an Industry Level, a Resource Lean, a Baseline Emissions figure, an Education Level and a Standing Army. Armies move only between neighbouring continents.
_Avoid_: country, nation, territory, region, state

### Earth

**Facility**:
A building placed in a Nation State. The First Playable has five kinds: Factory, Power Plant, Refinery, Launch Site and Research Lab.
_Avoid_: item, building, module (that is the Colony word), structure

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
A Faction's accumulated claim on a Nation State or an enemy Colony, built from a fixed per-turn Allotment and decaying when neglected. The first Faction past the place's threshold takes control; the owner of a Colony can spend to push a rival's Influence back.
_Avoid_: diplomacy points, favour, reputation

**Allotment**:
The fixed amount of Influence a Faction receives each turn, split freely across any number of targets during the Orders phase. It grows with the number of Nation States the Faction controls and does not carry over.
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
The fixed amount of CO2 the oceans and forests remove from the CO2 Stock every turn. Net emissions below it stabilize the stock.
_Avoid_: absorption, offset, carbon capture

**Sea Level**:
How far the oceans have risen with the Temperature. It is drawn on the globe as a creeping waterline, and it permanently takes build slots from Nation States according to their Coastal Exposure.
_Avoid_: flooding, water line, ocean rise

**Collapse Line**:
The one Temperature at which the game ends. Every other effect of Temperature is continuous; this is the only line.
_Avoid_: threshold, tipping point, game over temperature

**Climate Panel**:
The screen showing the CO2 Stock, the Temperature and where it is heading, this turn's Emissions by source, the sink and the net, the penalties in force, and a projection to the last turn.
_Avoid_: warming meter, climate HUD
