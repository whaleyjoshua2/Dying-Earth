# Last turn's Battle drawn on the map, and the shields that fought

Ticket [#311](https://github.com/whaleyjoshua2/Dying-Earth/issues/311) on
[map #304](https://github.com/whaleyjoshua2/Dying-Earth/issues/304).

| picture | what it shows |
|---|---|
| [`earth-battle-ring.png`](earth-battle-ring.png) | `shot: battle:region panel:0 look:90.0,62.0`. The turn after the Custodians' 2nd Chinese Army marched on Russia: a **ring in the Custodians' teal** round Russia's label, and beneath it the two shields that fought, both **outlined in that colour**, the Custodians' 4 wearing a **red pip** for the two damage it took, the Prospectors' 6 unhurt. Every other label and shield on the globe is as before. |
| [`earth-battle-ring-card.png`](earth-battle-ring-card.png) | `shot: battle:region select:russia panel:0 look:90.0,62.0`. The same with Russia's card open: the ring and the shields unchanged by the selection. |

No batch was run: an interface change with one engine accessor behind it (`battle_last_turn_at`,
which reads the Report and changes no rule). The ring's hover, which reads the record and names the
attacker, the rounds and each party's line, is a map hover and not an egui tooltip, so the `tip:`
aid cannot photograph it; it was not looked at.

## What was decided, in the designer's words

*"q1 as recommended drop the disc arround the body in the solar map q2 damage pip and outline for
fought q3 yes q4 this should be answered by q1"*: a ring in the aggressor's colour round a Region's
label on Earth and a Colony's label on its Body, its hover reading the record and a click opening
the Report; **no ring round a Body on the Solar map**, so a Battle in orbit is marked nowhere on the
map; both the outline and the damage pip on the shields; for the one turn the record lives; orbit
answered by the first.

## Settled by the builder, to be corrected if wrong

- **The ring is 26 pixels round the Region's point** at two pixels' width; **the click that opens
  the Report is on the ring's crown**, a ten-pixel spot at its top, so the label's own click still
  selects the Region.
- **The outline is on every shield at the place**, since a Battle is a melee of every party present
  and nothing records which Army fought; **the pip means hurt, not fought**: it shows whenever the
  stack carries damage, which is cumulative, so an Army hurt three turns ago wears it until it
  heals.
- **A Colony's ring** sits round its label on the Body's surface view, with the same crown hotspot.
- **A `battle:region` shot aid** stages a ground Battle: seat 1 takes China's first neighbour and
  raises an Army there, seat 0 raises one at home and marches, and the turn runs.
- The **Battle Report** glossary entry says the map draws it now.

## Looked at, not tested

The two pictures above, each opened and read before this was committed. The first pair was taken
with the globe facing Africa, and Russia, where the Battle was, off its edge; retaken with `look:`.
