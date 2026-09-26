# Ticket #358: Relay Networks and the Chorus show Influence the game never pays

Found building #345: the game printed Influence on a Module's line that the Allotment never summed.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/358#issuecomment-5842177182)
is the authority, and served as the spec.

## What was built

- **One path from a Module to the Allotment.** `building_allotment` reads each working Module's own
  figures (`module_yield_at`), the ones the card prints, instead of the raw table row. So **Relay
  Networks is paid** (a Relay's second point) and **the Chorus is paid**. A starved Colony still gives
  nothing (#278), and #359's shut-Habitat half now sits on the Module's figure with the rest of it.
- **The Chorus's per-Colonist Influence is paid outside the Faction multiplier**, at the designer's
  word, on the Spaceport's argument (#183): a new `Yield::allotment_outside`, summed after the
  multiplier in `influence_allotment`. Its base 1, and Relay Networks' +1, stay inside.
- **The audit, every `Yield` field against what the game actually charges or pays:** resources,
  Research and upkeep already came from the Module's figures; Standing has no modifier to lose.
  **Facility Emissions did not:** Climate recomputed them from the table and charged Clean Power and
  Clean Manufacturing only once done, while the card read them at half under the Archivists'
  Provisional Findings. **Climate now charges each directed Facility's `facility_yield` Emissions.**

No screen changed: the cards were right all along. No figure moved; no save moved.

## Tests

`relay_networks_is_paid_into_the_allotment`, `the_chorus_is_paid_at_face_value` (two points for twice
six Colonists at the Arkwrights' x0.8, not one) and `climate_charges_the_emissions_the_card_shows`,
all three witnessed red on the old code. The Unique Modules test now reads the Chorus in its two
halves. Clippy gate clean; 480 + 8 + 6 pass.

## The sweep

[`sweep-before.txt`](sweep-before.txt) is ticket #363's after; [`sweep-after.txt`](sweep-after.txt).
Wins (3 / 36 / 3 / 3) and collapses (35) did not move. Places taken by Influence per seating
311 / 366 / 362 / 348 to 309 / 367 / 351 / 354. The Allotment is not printed per seat by the sweep;
the move is small because few Relays and one Chorus in eighty games stand. The orbital war moved
with the board: Battles in 10 games of 80 (from 8), Launches in 15 (from 17) -- noise of a
different board, not a change to that lane.
