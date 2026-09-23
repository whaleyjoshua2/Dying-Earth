# A new way to signify Battles on the map, orbit included

Ticket [#317](https://github.com/whaleyjoshua2/Dying-Earth/issues/317) on
[map #316](https://github.com/whaleyjoshua2/Dying-Earth/issues/316).

| picture | what it shows |
|---|---|
| [`solar-mars-battle-mark.png`](solar-mars-battle-mark.png) | `shot: battle:1 panel:0 window:1600x900`. The Solar System Map the turn after a four-Faction Battle in Mars orbit: the **Battle mark**, crossed blades on a disc in the Custodians' teal (the aggressor), beside Mars's label to its left; and on the page at the right, **Battles last turn**: *Mars orbit: the Custodians attacked; 3 round(s). Nobody holds Orbital Control.* with the same glyph, a way there. Taken at 1600 wide because at 1280 Mars sits behind the side panel. |
| [`mars-surface-band.png`](mars-surface-band.png) | `shot: battle:1 panel:0 window:1600x900`, the Mars view. The orbit band's new last row, *A Battle here last turn: the Custodians attacked*, with the glyph, in teal, under *Orbital Control: nobody*. |
| [`earth-russia-battle-mark.png`](earth-russia-battle-mark.png) | `shot: battle:region panel:0 look:90.0,62.0`. Russia the turn after the Custodians' 2nd Chinese Army marched on it: the mark above the label in the Unrest label's row, the two shields beneath **without** the outline 0.08.7 gave them, the red damage pip still on the hurt Custodian shield. |

No batch was run: an interface change; the engine is untouched (the accessor from #311 gains its
orbital caller). The mark's hover is a map hover the `tip:` aid cannot photograph and was not
looked at; it is #311's summary unchanged.

## What was decided, in the designer's words

*"q1 its the ring i most want to replace q2 that exactly sounds good q3 yes q4 yes q5 let drop the
outlines"*: the ring gone everywhere and one mark in its place, a Region's, a Colony's and a Body's;
crossed swords off-white on a disc in the aggressor's colour, hover and click as before; the 3D
cones untouched; a *Battles last turn* list on the Solar map's page; the shields' outline dropped
(the damage pip was not named and stays).

## Settled by the builder, to be corrected if wrong

- **The glyph is drawn**, `assets/icons/battle.svg`, two blades with crossguards and pommels, and
  joins `DRAWN` beside the station glyph, so it owes no credit; the first drawing read as a plain
  X at eighteen pixels and was redrawn with heavier guards. It is a kind glyph (`Kind::Battle`), so
  it takes the off-white fill every kind glyph takes.
- **Where the mark sits**: on Earth, above the Region's label in the Unrest label's row, and a row
  higher when that label is showing; at a Colony, above the slot's point; on the Solar map, to the
  left of the Body's label at the label's height, measured from the label's width, because below
  the disc the moons' labels hang (the first placement, under the disc, sat beneath *Phobos*).
- **The orbit band's row** is the Body's surface view's mark; its hotspot is the mark's.
- **The list's rows** use the Report's own *Go there* jump; an old record with no place says so on
  hover instead.
- **The Battle Report glossary entry** says the mark, the band row and the list.

## Looked at, not tested

The three pictures above, each opened and read before this was committed, two of them also cropped
and enlarged to read the glyph and the pip.
