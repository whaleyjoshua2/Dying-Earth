# The sea on the globe: it shows, and it drowns the lakes too

The designer asked which version changed the Earth Map to show rising seas. **None of them did**:
`git log -S` on the drowning code in `src/textures.rs` and on `near_water` returns exactly one
commit, `94da9c4` (2026-09-09, the First Playable). The globe has taken its coasts since the first
build and nothing has touched that code since.

The code had never been **looked at** running, though. Every `shot:` capture in this diary is taken
at turn 1, before any Sea Level threshold has fired, so no picture in the repository had ever shown
the sea arrive. These are the first.

## How they were taken

Headlessly, from the 0.08.3 release binary, with `temp:` firing the thresholds in a quiet turn:

    target/release/dying-earth.exe shot:<prefix> panel:0             # +1.2 C, turn 1, nothing fired
    target/release/dying-earth.exe shot:<prefix> panel:0 temp:2.9    # +2.9 C, all three fired

| picture | what it shows |
|---|---|
| [`globe-plus-1.2.png`](globe-plus-1.2.png) | The board as every other capture has it: **+1.2 C, turn 1**, no threshold fired, coastlines as the photograph draws them. |
| [`globe-plus-2.9.png`](globe-plus-2.9.png) | **+2.9 C, all three thresholds fired.** The land has browned and the coast has visibly retreated: West Africa, the Gulf of Guinea, the Mediterranean shore and the Red Sea are all eaten into. |
| [`africa-pair.png`](africa-pair.png) | The same crop of Africa from both, magnified three times, cold on the left and warm on the right. This is the picture that settles it. |

## It works, and it reads

The sea shows. At three thresholds the Atlantic coast of West and Central Africa has lost a band of
land wide enough to see at the globe's default zoom, without zooming and without hunting for it.

**Two notes on what it is not.** `CONTEXT.md`'s Sea Level entry calls it *"a creeping waterline"*.
It is not a line: it is a band of land repainted the sea's own colour (18, 40, 90), so what the eye
reads is the **coastline moving**, not a rising mark. And it is **per state, not per world** — the
band widens only for states whose thresholds have fired, so a picture of it is a picture of who has
been hit.

## A defect the picture found: the lakes drown too

In `africa-pair.png` the warm side carries **rectangular slabs of navy inland** — the Rift lakes and
their neighbours grown into blocks. They are not coastline.

`near_water` asks whether any pixel within a square of radius `fired x 2` carries mask value **0**,
and mask 0 means *no Nation State*, which is every pixel of water there is. Sampled from
`assets/textures/earth_states.png` (2048 x 1024): **Lake Victoria 0, Lake Tanganyika 0, the Caspian
0, Lake Superior 0** — all indistinguishable from the Atlantic. So each fired threshold floods a
band around every inland lake as readily as around the ocean.

Two things make it read badly rather than merely being wrong:

- **The neighbourhood is a square**, not a disc, so an isolated lake grows a rectangle. A lake a few
  pixels across becomes a hard-edged block of sea.
- **The band is wide.** Radius is 2 pixels a threshold on a 2048-wide equirectangular mask, so three
  thresholds reach **6 pixels, about 1.05 degrees, roughly 117 km at the equator** — and since the
  mask is equirectangular, the same square covers far more longitude near the poles than at the
  equator.

The engine is not affected in the slightest: slot loss, displacement and Unrest all key on
**Coastal Exposure**, a figure on the card, and never on these pixels. This is a picture defect
only. Nothing has been changed here; it is recorded for the designer to decide on.

**Three ways it could be answered**, cheapest first, none taken:

1. **Leave it.** Rising seas do take lake shores, and at a glance the globe reads as a drowning
   world, which is the point. It costs nothing.
2. **Make the neighbourhood a disc** rather than a square. One line, and the blocks become blobs
   with the same reach.
3. **Tell the lakes from the sea.** The mask would need a third value for inland water, generated in
   `examples/prep_assets.rs` from Natural Earth's lakes layer, and the drowning would read only the
   ocean. It is the correct answer and the dearest one, and it makes a Region on a lake immune to a
   sea that should arguably still reach the Caspian.
