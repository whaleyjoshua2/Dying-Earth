# Ticket #366: seven defects from the list

Seven things the game said or did that were untrue or unreachable, from the designer's list for
0.09.2. Built on the ticket's own recommendations, the designer having said *"proceed to 366"* on
going afk; [the resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/366) is the
authority, [§6 of the spec](../../../spec/version-0.09.2.md#6-seven-defects-from-the-list) records
it.

## What was built

| | defect | what changed |
|---|---|---|
| 1 | The Exchange could not be built on Tiangong | The station's build check reads `stands_on_a_station`, the one predicate, in place of a second list of kinds; the Exchange and the Heliostat pass; the Trade Post cap reads the job. |
| 2 | The card says India, the order wants `southasia` | The driver's `pick_state` matches the board's names as well as the ids, and its error lists the names. |
| 3 | "you hold it" beside "occupied by", then "now belongs to" over a neutral board | A throw-off in the Resolution that transferred the place rewrites the transfer's line to one sentence; the driver's note says who occupies. |
| 4 | "slot 1 on the Moon" for slot 0, in the rival paragraph | The slot's name. |
| 5 | "you cannot pay what it asks" of a Fuel Contract wanting Fuel | `card_shortfall`, one source for the engine's refusal, the driver and the card modal. |
| 6 | The Spaceport's Influence did not show | `pay_spaceport` returns what it paid and the lift line says it; the Energy line names each shut building's place. |
| 7 | An unanswered Tech pick did not block the turn | Not reproduced; the reading is in the spec. |

## The red witness

Five tests, one a defect, all green on their first run, which is the case to distrust. Two were
perturbed on purpose and watched go red:

- The throw-off fold switched off: *"one line, not a transfer and a throw-off: ["The European Union
  now belongs to the Custodians (Pacified).", "The European Union threw off the Custodians: …"]"*,
  the two-line defect itself.
- `card_shortfall` made to return nothing: *"left: None, right: Some("you have 12 Fuel of the 20 it
  asks")"*.

The other three (the Exchange and the Heliostat, the rival's slot name, the lift line) and the
Energy line's place were not perturbed; each asserts a string the old code did not produce.

## Measured

Defect 1 reaches the computer seats: the computer's Prospectors can build the Exchange on a station
and the Archivists the Heliostat, where neither could before. The sweep after it,
[`../sweeps/after-366.txt`](../sweeps/after-366.txt), 20 seeds x four seatings at the shipped cell,
against the sweep after #367 (the last engine change on this map):

| | after #367 | after #366 |
|---|---|---|
| Custodians | 8 | **7** |
| Prospectors | 28 | **24** |
| Arkwrights | 0 | **1** |
| Archivists | 2 | **4** |
| collapses of 80 | 42 | **44** |

Gates completed 73 / 61 / 64 / 52 of 80, against 70 / 63 / 55 / 39. The Archivists' gate up from 39
to 52 and their wins from 2 to 4 is the shape a buildable Heliostat would give; whether the
Prospectors' 28 to 24 is the Exchange or the seeds is not measured here. Reported as figures.

## The driver, checked by hand

`play new --start india` starts the game in India, and an order line `relief india` is parsed as
India (it was refused for want of Ducats, which is the right refusal); `relief southasia` still
parses; `relief atlantis` is refused: *"atlantis" is not a Region; the board's are Nigeria, Egypt,
China, India, Indonesia, Australia, The European Union, The United States, Mexico, Brazil, Russia,
Iran, Japan, Saudi Arabia*.

**The hand check earned its keep**: the first cut of `pick_state` overflowed the stack on any word
that was not a name, because a blanket replace of the old call had turned its own fallback into a
call to itself. The suite does not run the driver; only running it found it.

## The review

Two axes, run as sub-agents over the commit.

**Standards**: no violation. Four fix-ups taken: a card's price has one truth (`card_effect_affordable`
reads `card_effect_shortfall`, so a cost effect added later cannot shut the offer while the reason
says nothing); the throw-off fold finds the transfer's line by an index recorded when it was written
rather than by searching the Report; one helper names a producer's place for the alarm's hover and
the Energy line alike; the driver's `pick_state` no longer reads `pick`'s error text to choose its
own. `pay_spaceport` is `#[must_use]`.

**Spec**: correct on all seven. One computer-seat loose end fixed: the computer's Trade Post
pre-filter read the kind, so a computer Prospector proposed a second Exchange the engine then
refused; it reads the job now. (Its weight already did: the weighing runs on the common job.) Defects 6 and 7 were closed by reading rather than by a driver
reproduction, which the review noted; the reading is in the spec.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean; engine 492 + 6, root 8.
