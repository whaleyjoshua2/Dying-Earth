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

**Warming Track**:
The measure of global warming on Earth. It advances as a consequence of player and faction choices rather than on a fixed schedule, and losing control of it is how a game is lost.
_Avoid_: doom clock, countdown, timer, disaster meter

**Body**:
A place in the solar system that can hold a Colony. The First Playable has the Moon and Mars.
_Avoid_: planet, world, site, location, node

**Colony**:
A permanent settlement a Faction holds on a Body.
_Avoid_: base, outpost, settlement

**Earth Map**:
The 3D view of Earth, where industry, launch sites and the Warming Track are seen and acted on.
_Avoid_: home view, globe view, terrestrial map

**Solar System Map**:
The 3D view of the solar system, where Bodies, Colonies and ships in transit are seen and acted on.
_Avoid_: space view, orbital map, star map

**Event**:
An unplanned occurrence that interrupts a turn — a solar storm, an equipment failure, a discovery.
_Avoid_: incident, crisis, card

**Tech**:
An advance a Faction unlocks to change what it can build or how much ecological damage it does. Techs are arranged in a tech tree.
_Avoid_: research, upgrade, invention

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
