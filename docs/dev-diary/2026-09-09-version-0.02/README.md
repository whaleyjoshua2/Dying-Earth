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

## #23: the roster

The designer's "liner for units and ships the player controls", decided on the ticket: the side
panel's default content when nothing is selected, listing Ship stacks (and Ships in transit), Armies,
Colonies and Nation States the player directs; each row a button that selects the thing and jumps to
its view; "no order" marked on any Ship stack or Army that has nothing pending this turn. Escape
clears a selection and brings the roster back.

![The roster at turn 9: a Colony Ship in transit, three Standing Armies with no order, three states](roster.png)

## #24: start buildings

Decided by the designer: every Nation State starts with as many Facilities as its Industry Level,
chosen by its Resource Lean (Materials: Factory, Power Plant, Refinery; Energy: Power Plant, Factory,
Refinery; Fuel: Refinery, Power Plant, Factory); they come with the state whoever takes it; the
Faction start states add their Launch Site; the Stockpile starts at 80 Materials rather than 60. A
Facility nobody directs stands idle: it makes nothing and emits nothing. All of it is one row per
state in `nation_states.toml` and one number in `factions.toml`.

![Asia at turn 1 with its Factory, Power Plant, Refinery and Launch Site](start-buildings.png)

Twenty seeds per pairing afterwards, against the first build's numbers in brackets:

| | Custodians v Prospectors | Prospectors v Prospectors | Prospectors v Custodians |
| --- | --- | --- | --- |
| First Colony | turn 7 or 8 (was 10 to 12) | turn 7 or 8 (none) | turn 7 or 8 (10 to 12) |
| Buildings per Faction | 5 to 8 (5 to 6 v 4) | 3 to 9 (2 to 4) | 3 to 12 (2 to 8) |
| Colonists off Earth | mostly 4 v 4 (4 v 0) | mostly 4 v 4 (0) | 4 v 4 or 4 v 8 |
| Outcome | Collapse, turn 10 or 11 (no Collapse) | Collapse, turn 9 to 11 (10 or 11) | Collapse, turn 10 or 11 (4 of 20) |

Two anchors moved toward their marks and one moved away: with three emitting Facilities per Faction
from turn one, the Temperature crosses +3.0 by turn 10 or 11 in every game. Reported, not retuned:
the CO2 clock is ticket #27's question, and it now has to allow for this.

## #25: Events ten percent rarer, and no more Calm Cards

Decided by the designer, in three rounds: ten points, from 60% to 50%; then, instead of more Calm
Cards, **no Calm Cards at all** and a chance each turn that no card is drawn; the Draw Chance and the
Climate scaling both rise with the Temperature (50% at +1.2 C, +2.5 points per full 0.2 C, 72.5% at
+3.0); the deck grows to **thirty cards, the twelve first-playable Events twice and six new ones
once**, chosen from proposals: Solar Maximum, Meteor Shower, Dust Storm, Unrest, Reactor Leak and
Permafrost Thaw. The deck is still never reshuffled; at these rates twenty-four turns draw about
fifteen cards.

All of it is in `events.toml`: each Event row has a `copies` count and the constants sit at the top.
`CONTEXT.md` retires Calm Card and adds Draw Chance. Nine new tests, five watched red first.

Twenty seeds per pairing afterwards: every pairing still ends in Collapse, and two Prospector AIs now
collapse on turn 7 or 8 rather than 9 to 11, since the deck no longer carries eight blanks and
Permafrost Thaw arrives scaled. Reported, not retuned; #27 owns the clock.
