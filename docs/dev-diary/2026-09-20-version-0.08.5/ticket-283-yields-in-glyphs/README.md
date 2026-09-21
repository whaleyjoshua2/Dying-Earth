# Yields in glyphs on the planet card, and a founding door without the planet-wide line

Ticket [#283](https://github.com/whaleyjoshua2/Dying-Earth/issues/283) on
[map #275](https://github.com/whaleyjoshua2/Dying-Earth/issues/275).

All four at 1280x800, `panel:0`.

| picture | what it shows |
|---|---|
| [`mars-planet-card.png`](mars-planet-card.png) | `shot:` with nothing selected, the Mars picture. The planet card now opens with **"Mars as a whole: x1.25 x0.75 x1.50 x1.30"** in glyphs, weak, then a **Colonies and sites** block: six rows, *Olympus Mons: empty* through *Elysium Planitia: empty*, each with the site's four yields in glyphs beneath, then the In orbit block as before, without yields. Before this ticket the card was the heading, one sentence and the stations. |
| [`olympus-mons-card.png`](olympus-mons-card.png) | `archive:1`: the Olympus Mons Colony card, held by the Custodians, with **"Yields here: x1.26 x0.74 x1.65 x1.23"** under the heading, where there was nothing. |
| [`chryse-slot-panel.png`](chryse-slot-panel.png) | `site:mars,2`: the empty-slot panel for Chryse Planitia. **"Yields here"** stands alone; the *Mars as a whole* line that stood beneath it since #258 is gone from the founding door. |
| [`ship-card-doors.png`](ship-card-doors.png) | `settler:mars`: the Ship card's Load and unload block, the second founding door, unchanged: *Found a Colony at Olympus Mons* with the site's glyphs on its face, one button per open site. The stance row shows *Blockade* from ticket #278. |

No batch was run: nothing about play changed.

## What was decided, in the designer's words

- *"q1 a"* -- the planet card gets a **Colonies block**: one row per Colony on the ground and per
  open site, the slot's glyph row beneath, clicking a row selects it.
- *"q2 without"* -- **stations keep their rows without yields**: four identical rows say nothing.
- *"q3 that"* -- the **Colony card** gets the glyph row under its heading.
- *"q4 a"* -- the **planet-wide line moves to the head of the planet card** and off the founding
  door. This reverses #258's *"keep it"*; nothing is lost, the comparison between planets moves to
  the planet.
- *"q5 give it the same face"* -- the **Antarctic "by sea" door** wears the founding button's face
  with the site's glyphs, as the two Ship doors do.
- *"q6 yes"* -- the two Ship doors stay **identical and unchanged**.

## Settled by the builder, to be corrected if wrong

- **On Earth the block lists standing Colonies only.** Antarctica's shut sites stay the ice's
  business and the map labels'; once a Colony stands there it has a row.
- **A station's own card carries no yield row** either, since it reads the Body's figures, which
  now head the planet card.
- **Every new row goes through the glyph-row helper**, never through the prose renderer, so the
  #258 trap (a word before its figure comes out in words) cannot recur here.
- The Antarctic door founds through the same order as before; only its face changed.

## Looked at, not tested

No test reads a rendered row, as #258's record says; the four pictures are the check, and each
was looked at before filing. `342 passed`, `6 passed`, clippy clean with `-D warnings`, since the
engine is untouched.
