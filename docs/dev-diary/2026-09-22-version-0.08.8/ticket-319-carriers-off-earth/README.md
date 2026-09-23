# Carriers off Earth for every seat, and an Intercept that fires

Ticket [#319](https://github.com/whaleyjoshua2/Dying-Earth/issues/319) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`ships-intercept-hover-measured.png`](ships-intercept-hover-measured.png) | `shot: stack:1 panel:0 tip:The computer seats intercept`. The Ship stack card's Intercept label with its hover's new third line: *The computer seats intercept with warships when an unarmed rival hull is inbound: a Colony Ship or a Carrier.* Measured behaviour, said as such. |

## The batch

`batch-after.txt`: 20 seeds x four seatings at the shipped climate cell, the computer seats under
the new appetites, against the 0.08.7 closing sweep (which is 0.08.6's).

| | 0.08.7 | after |
|---|---|---|
| wins, seat 0 first, by seating | 5/13/0/0, 7/13/0/0, 2/5/1/0, 0/20/0/0 | 5/13/0/0, **6/14/0/0**, 2/5/1/0, 0/20/0/0 |
| collapses | 2, 0, 12, 0 | 2, 0, 12, 0 |
| Battles opened | 24, 28, 7, 3 | 24, **25**, **9**, **2** |
| **Interceptions** | not counted | **0, 0, 2, 0** |
| Armies landed at a Colony | 0 everywhere | **0 everywhere** |
| warships built | 70, 29, 66, 53 | 70, **32**, 65, **55** |

**Intercept fires**: twice over eighty games, both the Custodians' in the Arkwrights-first seating,
where nobody had ever intercepted before. Small, and real.

**No Carrier was built by anybody, and no Army landed.** Two seeds simulated headlessly say why
(`dying-earth.exe simulate:3` and `simulate:7`): the appetite is a conjunction, a raised Army
standing in a Region, a rival Colony off Earth whose holder the seat is Wary or worse toward, and a
Shipyard over Earth, and in seed 3 the computer raised no Army at all in thirty-six turns and one
station stood off Earth. The extension reaches every seat and never has all its terms at once on
today's board. The 0.08.7 review's proposals 2 and 3 (a prize only force takes; a war footing that
raises Armies with cause) are what would make the terms coincide, and both are in the map's fog.

## What was decided, in the designer's words

*"q1 yes q2 yes q3 yes q4 yes q5 yes"*: any seat with cause wants a Carrier and loads one; the
targets are rival Colonies off Earth whose holder the seat has cause against; Intercept needs no
Orbital Control and outscores Hold when an unarmed enemy hull is inbound; the Intercept label says
so on hover; the sweep counts interceptions and the landing line already carries the seat.

## Settled by the builder, to be corrected if wrong

- **`carrier_target_exists`** is the one predicate the building and loading gates read, beside
  `rival_holds`; the Prospectors keep their unconditioned appetite through it.
- **Intercept's score** is its own weight or Hold's plus one, whichever is greater, so it wins its
  key over Hold whenever an unarmed hull is inbound; Attack still wins when it is allowed.
- **The interception counter** joins the war counters with a serde default, so a 0.08.7 save loads;
  an interception is also still counted among the orbit attacks, as it always was.
- **The landing line** already carried the Carrier's seat (it is by seat), so nothing was added to
  it.

## Tested, looked at, measured

`a_carrier_has_somewhere_to_go_only_with_cause_against_a_colony_off_earth` in
`engine/tests/formulas.rs` was watched to fail against a predicate made to return nothing, then to
pass restored. The picture was opened and read. The batch is above.
