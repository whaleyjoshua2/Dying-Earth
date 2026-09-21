# A Blockade that starves, and must be chosen

Ticket [#278](https://github.com/whaleyjoshua2/Dying-Earth/issues/278) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

| picture | what it shows |
|---|---|
| [`iss-card-blockaded.png`](iss-card-blockaded.png) | `shot: blockade:1 hab:1 panel:0`, 1280x800. The ISS's card over Earth with a Prospector Frigate in its slot ordered to Blockade: under *Held by the Custodians* the new line, in red, **"Blockaded by the Prospectors: producing nothing, upkeep still paid."** The Prospector ship marker sits beside the ISS on the globe. `blockade:1` is a new aid that plants the Frigate on the stance. |

The measurement, twenty seeds with the Custodians first:
[`sim-20-custodians-first.txt`](sim-20-custodians-first.txt).

## What was decided, in the designer's words

- *"q1 a"* -- **two rules.** A station starves under a Blockade of its slot; a Colony on the ground
  starves while one rival holds Orbital Control of its Body outright and has a stack there on
  Blockade. A contested orbit starves nobody, as it lands nobody.
- *"q2 a also the blockaid needs to be positivly choosen not just the presents of a ship"* -- the
  Colony **still pays its upkeep** while making nothing; and **a Blockade is a stance**, chosen,
  never a side effect of a warship sitting in a slot. This reaches back to 0.07.0: the three effects
  a Blockade had (no unloading, no refuelling, no building into the slot) now need the stance too.
- *"q3 that"* -- silenced: output, Research, the Trade Post's Ducats and the Relay's Allotment.
  Untouched: the Habitat's room, the Core Module, the Shipyard, the Barracks, the Archive's fund.
  Nobody dies of a blockade and nothing is destroyed by one.
- *"q4 yeah that"* -- read live at each Income.
- *"q5 land it now"* -- the 0.08.2 spec's weight-1 offence for blockading, never built, lands: one
  per Colony per turn of blockade, against the holder.
- *"q6 a we'll run the battery in a future version"* -- no counter this version; the computer
  seats learn to want a warship where a Colony of theirs is starved, and the sweep measures. The
  Battery Module is deferred to a later map.
- *"q7 sounds good"* -- blockade-turns suffered and imposed, by seat, in the sweep; the card's words
  as proposed.

## Settled by the builder, to be corrected if wrong

- **The stance is Ships only.** Ordering a stack to Blockade sets its warships to Blockade and its
  other Ships to Hold; the Army stance row never offers it. A blockading stack neither attacks nor
  intercepts; it defends like Hold if attacked.
- **The order is refused** unless a warship of the seat's sits in an Orbital Slot that is not its
  own station's: a rival's, or an empty one held against a builder (0.07.0's rule kept).
- **"Read live" and escapes.** Income clears every Ship's `escaped` flag before it counts the
  starving, so a blockader that escaped a Battle blockades nothing for the rest of that turn's
  Resolution and starves again at the next Income.
- **The computer.** Every AI warship transit already lands in the richest rival station's slot; it
  is now offered the Blockade stance there at weight 3 for every Faction (`stance_blockade` in
  `ai.toml`, a new figure the sweep can re-fit). A starved Colony counts as a threat when the seat
  weighs what to build there, so a Shipyard under blockade wants a warship.
- **The Report line**: *"ISS over Earth is blockaded by the Prospectors: it made nothing this turn,
  and its upkeep was paid."*, filed at the Colony.
- **Saved counters** on the seat for blockade-turns suffered and imposed, with defaults, so old
  saves load; the sweep prints both by seat, and the `sim` example per seed.
- **The glossary's Blockade entry** no longer says *"and nothing else"*; **Orbital Control** now
  says what starves the ground.

## Witnessed red

Three new tests, each watched to fail under a deliberate mutation. With starvation switched off in
the income pipeline: *"starved, the station made nothing and paid the same upkeep"*. With the
ground rule's blockading requirement removed: *"ordered to Blockade, the Colony on the ground
starves"*. With the AI's Blockade candidate removed: *"a Blockade of the station whose slot it sits
in"* against an order list with only Hold in it. Restored, `338 passed`, `6 passed`, clippy clean
with `-D warnings`. The two 0.07.0 blockade tests were moved to the new rule: a warship in the slot
on Hold now blockades nothing, and they say so.

## Measured, twenty seeds with the Custodians first

| figure | Greenwash batch | this ticket |
|---|---|---|
| wins, Custodians / Prospectors / Arkwrights / Archivists, of 20 | 0 / 16 / 0 / 2 | **0 / 16 / 0 / 2** |
| collapses | 2 | **2** |
| seeds with any blockade | -- | **4** |
| blockade-turns imposed over the batch, by seat | -- | **[0, 24, 0, 0]** |
| blockade-turns suffered over the batch, by seat | -- | **[1, 0, 8, 15]** |

Only the Prospectors blockade, in four seeds of twenty: three of them for three Colony-turns,
and one for eighteen, in which an Archivist station starved for twelve turns and an Arkwright one
for six. The win column did not move on this seating. The Archivists, at nought wins in the
0.08.4 sweep, are the seat this rule presses on; the closing sweep measures all four seatings, and
the Battery waits for a later map.
