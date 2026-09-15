# Map label mockups — ticket #216, question 4

Three mockups made while grilling [A card's heading wears the Faction's glyph alone, and every Ship is
listed by name](https://github.com/whaleyjoshua2/Dying-Earth/issues/216), on map
[#215](https://github.com/whaleyjoshua2/Dying-Earth/issues/215).

**The backgrounds are the game; the label blocks are drawings.** The captures in `captures/` are real
headless `shot:` output from the 0.08.1 tree at 1600x900, and the candidate label blocks are composited
onto them with Pillow, so label density and collisions can be judged by looking rather than by
describing. The Ship names are the real first entries of `assets/data/ship_names.toml` with the real
Faction prefixes; the leading glyph in a drawn block is a **placeholder square**, because these pictures
are about how much room the lines take and not about art.

## What the captures corrected

The ticket body said *"The map draws no Ship names… a Ship in an Orbital Slot is a bare glyph with no
label."* **That is wrong**, and the first capture showed it. Both maps already draw a label block for
Ships at a Body:

- the **Solar System Map** draws, above the Body's own label, one line per Faction — `Archivists x1 str 0`
  — in the Faction's colour with a Kind Glyph, closing with `Orbital Control: <Faction>`;
- the **Mars Body Surface Map** draws the same block headed `In orbit around Mars`, as
  `Custodians: 1 Ship(s), strength 3`.

So the question is not whether a map labels Ships. It is **whether that per-Faction block becomes a
per-hull list**, and on which map. The bare-glyph case is only a warship sitting in an Orbital Slot,
which is a narrower thing than the ticket claimed.

## The files

| file | what it shows |
|---|---|
| `a-surface-map.png` | The Mars Body Surface Map's `In orbit around Mars` block: **now** (one line per Faction, with strength) beside **one line per hull**. |
| `b-solar-map.png` | The same comparison on the Solar System Map at Mars, where the block sits between the Mars, Phobos and Deimos labels. |
| `c-stress-24-hulls.png` | The stress case at Earth: **24 hulls listed one per line** beside **the same list capped at five with a tail**. |

`make-mockups.py` regenerates them: `py -3.12 make-mockups.py captures .`

## What is visible in them, which the descriptions did not say

- **At the ordinary size the change costs nothing.** Four hulls make four lines, which is exactly what
  four Factions make today. The block does not grow; only its words change.
- **It does cost a figure.** Today each line carries `strength 3`; a per-hull line carrying a name, a
  type and a strength is long, and the mockups drop the strength to keep the line readable. Whether
  strength survives on the line, moves to the hover, or stays only in the stack total is a real question
  the drawings raise and do not answer.
- **The Solar System Map is the tight one.** The Mars block already sits in the gap between the Mars
  label and the Phobos and Deimos labels, which themselves nearly touch. A per-hull block widens by
  roughly a third at four hulls, because a name and a type are longer than `x1 str 0`.
- **The stress case is worse than arithmetic suggested.** Twenty-four lines do not merely make a long
  block: they **swallow Earth, the Moon and Venus entirely** and reach across to the Mars block. It is
  the single worst fleet in 200 measured games, so it is rare — but it is unbounded, and the same
  picture would be produced by any player who parks hulls.
- **Capping at five with a tail leaves the map legible** and still names the hulls a player is most
  likely to want. The cap is what makes per-hull safe on the Solar System Map at all.

## Where the fleet figure came from

The 24-hull case is not invented: it is the largest fleet observed in a 200-game measurement made for
this ticket (50 seeds x 4 seatings), in which the median Faction holds **zero** Ships and one or two is
typical. The measurement was run with a temporary engine example that was deleted afterwards; the
numbers are recorded in the ticket's grilling round.
