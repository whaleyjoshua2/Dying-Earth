# Ticket #359: a mothballed Barracks still builds an Army

Found while building ticket #353: nine building doors read `working()` and the Barracks read its mere
presence. The audit this ticket asked for found a second Barracks door and a larger hole in the
Habitat, and the designer ruled on both.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/359#issuecomment-5841282679)
is the authority, and served as the spec.

## What was built

- **A shut Barracks raises no Army and repairs none.** "Shut" means mothballed, or dark for want of
  Energy. Three refusals in the Shipyard's shape: *no Barracks here*, *the Barracks here is still
  building*, *the Barracks here is shut: mothballed, or dark for want of Energy*. An Army already
  standing stays.
- **A shut Habitat still houses its people**, and room is unchanged. While it is **occupied** -- more
  Colonists than the working Habitats and the Core hold -- the Colony makes **everything but Energy
  at half**, rounded down, once however many are shut (`Game::habitat_halves`). That reaches the
  Modules' resources and Research at Income, the Colony's Widgets, its Modules' Allotment and their
  Standing. Energy is spared at the designer's word: *"no one likes to be shot while they're
  down."* A spare Habitat standing empty can be mothballed for nothing.
- **The Colony card says so** in amber under the holder line, because the Module rows still show
  their whole figures.
- **The computer seats never mothball a Habitat**, as they never mothball a Launch Site or a
  Shipyard, and they raise an Army only at a working Barracks.
- `shut:1`, a new shot aid, given with `barracks:1`.
- Glossary: **Barracks** and **Mothball**.

Clippy gate clean; 472 + 8 + 6 tests pass. Both new tests were witnessed red: the Barracks test on
the old gate, the Habitat test against a stub that always answered no.

## The audit

Every `ModuleKind` test in the engine that does not read `working()`, and what it is:

| where | reads | verdict |
|---|---|---|
| Build an Army at a Colony | the Barracks standing | **fixed** |
| Repair an Army at a Colony | the Barracks standing | **fixed** (not in the ticket) |
| A Colony's room | every Habitat standing | **kept at the designer's word**, now with the half rule |
| One Trade Post a Body | standing or on order | right: it is a cap on building, not a service |
| A Bombard's targets, the Archive, the Core | standing | right: what a hit can burn |
| The Generator events | standing | right: the event strikes the building |
| `sim.rs` counts | standing | right: a census |
| `ai.rs` planning | standing | the computer's own wants, not rules |

## The sweep moved, and why

`sweep -- 20 --seatings --balance --steps=300 --sinks=6`, the per-Faction totals from the foot of
[`sweep-before.txt`](sweep-before.txt) (ticket #357's after) and [`sweep-after.txt`](sweep-after.txt):

| | before | after |
|---|---|---|
| Custodians | 4 | 5 |
| Prospectors | 34 | 39 |
| Arkwrights | 3 | 1 |
| Archivists | 0 | 2 |
| collapses | 39 | **33** |
| gates completed (C / P / Ar / Ar) | 69 / 60 / 56 / 43 | 76 / 62 / 59 / 47 |

**Isolated**: with the Barracks and Habitat rules but the computer still free to mothball Habitats,
collapses read **41** and wins 6 / 31 / 2 / 0. So the half rule alone hurt the computer seats, which
had been mothballing lived-in Habitats whenever Energy was tight -- the Habitat was the building that
"makes nothing and is dearest to run". Keeping them lit is what moved the sweep. This is measured
behaviour, not a rule.

## The picture

[`shut-barracks-and-habitat.png`](shut-barracks-and-habitat.png): `shot: seed:2 barracks:1 shut:1
hab:ground "tip:the Barracks here is shut" panel:0 window:1920x1080`. A Moon Colony of the
Custodians' with its Habitat and Barracks mothballed and six living on the Core's four of working
room: **"A shut Habitat houses people here: everything but Energy at half."**, *Colonists 6 of 8
room*, *Widgets 2 a turn* (the Core's 4 halved), and **Build Army (Barracks)** greyed with the shut
refusal.
