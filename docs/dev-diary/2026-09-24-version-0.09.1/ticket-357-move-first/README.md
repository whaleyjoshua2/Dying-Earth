# Ticket #357: a shut door says what to do, not what the rule is

The designer's line: *"when trying to load pioneers into a colony ship that is not in low orbit
have the mouse over read something like 'move ship to low earth orbit to load'."* Asked about it,
the designer changed the rule instead of the lift's wording: *"let's say a launch site can reach
any orbit without one only low orbit (goes for colonies too and they don't have launch sites -
don't give them one this build)."*

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/357#issuecomment-5841113672)
is the authority, and served as the spec.

## What was built

- **The rule.** A lift from a Region with a working Launch Site (or the Arkwrights' Spaceport)
  reaches **any orbit of Earth**, Colonists and Armies both. A ground Colony has no Launch Site and
  is still reached from low orbit alone; a station from its own orbit. The lift's shut door is gone.
- **The refusals.** Every refusal whose cure is a change of orbit now reads **"Move this Ship to
  {orbit}, then {act} next turn: {rule}."** (`Game::move_first`). "Next turn" is there because a
  Ship takes one order a turn and the move spends it. Seven took the form: refuel, Bombard, Launch,
  loading off a Colony, unloading into one, an Army's lift off a Colony, and **founding a Colony**,
  which the ticket's list of six had missed and whose cure is the same.
- **Two refusals split**, so the move is offered only where a move would open the door. Refuel:
  wrong orbit gets the move, and *"every station that fuels you over {Body} is blockaded"* says
  itself. An Army: wrong orbit gets the move; *"that Army is not at this Body"* and *"that Army is
  aboard a Ship"* stand alone.
- **The Region card's send-to-Ship door** now opens for a Ship at a station's ring. While a door is
  shut, its hover is the engine's refusal alone; the capacity and crowding advice shows only while
  it is open.
- **The Launch Site's words** say "any orbit of Earth": the card's line, the Facility's `does`
  string, the driver's help, and the glossary's **Low orbit** entry.
- **The computer seats** load an empty Colony Ship at Earth in whatever orbit it sits in, and no
  longer bring it down to low orbit first.

Clippy gate clean; 470 + 8 + 6 tests pass. The new test,
`a_door_the_orbit_shuts_says_the_move_first`, and the three it changed were witnessed red on the old
engine before the change.

## The sweep moved

`sweep -- 20 --seatings --balance --steps=300 --sinks=6`, per-Faction totals from the foot of
[`sweep-before.txt`](sweep-before.txt) and [`sweep-after.txt`](sweep-after.txt):

| | before | after |
|---|---|---|
| Custodians | 4 | 4 |
| Prospectors | 37 | 34 |
| Arkwrights | 2 | 3 |
| Archivists | 1 | 0 |
| collapses | 36 | 39 |
| Arkwrights' gate completed | 53 | 56 |
| Prospectors' gate completed | 58 | 60 |

This is measured behaviour, not a rule: the computer's Colony Ships no longer spend a turn and a
Fuel coming down to low orbit before every load at Earth.

## The pictures

| picture | what it shows |
|---|---|
| [`lift-to-a-ship-at-the-iss.png`](lift-to-a-ship-at-the-iss.png) | `shot: seed:2 orbits:1 emigrants:4 select:eastasia panel:0 window:1920x1080`. China's card as the Custodians, with a Colony Ship at the ISS's ring: **Send 4 to TSV Beagle (at ISS)** is live. Under ticket #335 this door was shut. The Launch Site line reads *"lift to any orbit of Earth from here"*. |
| [`refuel-door-says-the-move.png`](refuel-door-says-the-move.png) | `shot: seed:2 settler:earth room:1 "tip:Move this Ship" panel:0 window:1920x1080`. A Colony Ship in low orbit; its refuel door's hover reads **"Move this Ship to Earth, at ISS, then refuel next turn: a station fuels a Ship in its own orbit alone."** Before, it read *"no station fuels a Ship in Earth, low orbit: it is in another orbit, or blockaded"*. |

## Noted, not done (the designer's word, Q5)

**Most of the game's other refusals still recite a rule rather than name a next move**: *"this Ship
already has an order"*, *"only 2 Pioneers are waiting there"*, *"no room in its Habitats"*, and
dozens more. This ticket changed only the refusals a change of orbit cures. Whether the rest should
say what to do is on the map's fog, not decided.
