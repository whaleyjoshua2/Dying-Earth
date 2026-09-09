# 2026-09-09: version 0.02, ticket by ticket

Work on map [#21](https://github.com/whaleyjoshua2/Dying-Earth/issues/21), on the branch `version-0.02`.

## #22: each building's income on the card and on hover

Every Facility and Module on a Nation State's or Colony's card now shows what it makes each turn at
today's multipliers, its Energy upkeep and its Emissions, and says "offline, making nothing" when the
shortfall rule shut it. Every build button shows the same three figures on hover, for the building it
would make.

![Asia's card at turn 9: Factory +6 Materials, 2 Energy upkeep, 0.8 Emissions; Power Plant +6 Energy, 1.1 Emissions](asia-card.png)

The figures come from one place: `Game::facility_yield` and `Game::module_yield` in the engine, which
the Income phase itself now uses. A formula test builds a mixed board and checks that the sum of the
cards equals what the next Income phase pays and what the Climate phase charges. Two mutations were
tried against it: breaking the Faction's Emissions multiplier on the card went red; breaking the
Resource Lean inside the shared function stayed green, because card and Income share the code, and
that mutation is caught by the Deep Mining test instead.

The hover text cannot be captured off-screen, so it was checked by reading, not by picture.
