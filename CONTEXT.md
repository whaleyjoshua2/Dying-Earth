# Dying Earth

A single-player, turn-based strategy game about colonizing the solar system before ecological collapse overtakes Earth. It should feel like a strategy board game rather than an action game.

## Language

**First Playable**:
The ten-turn version of the game covering only the Moon and Mars, with three resources and two factions. The destination of the current effort is a written specification for it.
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
A permanent settlement a Faction holds on a Body.
_Avoid_: base, outpost, settlement

**Earth Map**:
The 3D view of Earth, divided into Nation States, where Earth-side building, launches and the Climate Panel are seen and acted on.
_Avoid_: home view, globe view, terrestrial map

**Solar System Map**:
The 3D view of the solar system, where Bodies, Colonies and ships in transit are seen and acted on.
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
A building placed inside a Colony. The First Playable has five kinds: Mine, Generator, Refinery, Habitat and Shipyard.
_Avoid_: building, structure, facility, improvement

**Ship**:
A persistent piece that travels between Bodies, carries Colonists, and can fight. It is not consumed on arrival.
_Avoid_: vessel, rocket, fleet, expedition

**Colonist**:
A person counted in the population of a Colony or Nation State. Colonists are carried by Ships and held by Habitats; they are never spent as a resource.
_Avoid_: settler, crew, worker, population resource

**Nation State**:
One of seven regions of the Earth Map mirroring the continents, Antarctica included, that a Faction can control and build in. Each carries a population, an Industry Level, a Resource Lean and a Baseline Emissions figure.
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

**Influence**:
A Faction's accumulated claim on a neutral Nation State, built by spending resources on it over turns and decaying when neglected. The first Faction past the state's threshold takes control.
_Avoid_: diplomacy points, favour, reputation

**Research**:
Points produced by Research Labs and spent only on Techs. Held outside the Stockpile; it is not one of the three resources and never buys a Facility, Module or Ship.
_Avoid_: science, research points, RP, fourth resource

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

**Collapse Line**:
The one Temperature at which the game ends. Every other effect of Temperature is continuous; this is the only line.
_Avoid_: threshold, tipping point, game over temperature

**Climate Panel**:
The screen showing the CO2 Stock, the Temperature and where it is heading, this turn's Emissions by source, the sink and the net, the penalties in force, and a projection to turn 10.
_Avoid_: warming meter, climate HUD
