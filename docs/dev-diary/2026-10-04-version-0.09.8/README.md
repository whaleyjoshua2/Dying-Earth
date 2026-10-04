# Version 0.09.8, the Moon race version

The map is [#473](https://github.com/whaleyjoshua2/Dying-Earth/issues/473); the spec is
[`docs/spec/version-0.09.8.md`](../../spec/version-0.09.8.md). One entry per ticket, in the order
they were built.

## Card sub-headers (#474)

The designer: *"for the nation/outpost cards increase the size of each sub-header by 15%, e.g.
Influence, Pioneers, Policies. Influence should share the same white color. add a break between
policies and facilities and like influence I want to see a glyph leading each heading."*

Decided on the ticket: every sub-header on both cards, with the Colony card's Modules and Influence
lines promoted to headings; 14.5 points; white for all; the rule below the Policies block.

Each picture is the 0.09.7 build on the left and this build on the right, the same seed (7), turn 4,
at `window:1280x1500` so the whole card shows.

- **The Region card** (`select:EastAsia turns:3 seed:7`):
  ![the Region card, before and after](ticket-474/region-before-after.png)
- **The Colony card**, the ISS (`hab:1 turns:3 seed:7`):
  ![the Colony card, before and after](ticket-474/outpost-before-after.png)
- **The Archivists' Colony card**, for The Archive's heading (`hab:1 player:archivists archive:1`):
  ![the Archivists' Colony card](ticket-474/archivists-colony-card.png)
- **The headings at four times their size**, to judge the glyphs. Policies' scroll and Orders' list
  are new drawings (`assets/icons/policies.svg`, `orders.svg`):
  ![the headings magnified](ticket-474/headings-magnified.png)

Seen in the pictures and left alone:

- The Region card is 23 pixels taller for the larger headings.
- On a station with no Shipyard the Orders heading has nothing under it. It was so before; the
  larger heading makes it plainer.

Not pictured: the Ships heading (no Shipyard on these boards), the Colony card's Armies heading, and
the full-place warning under Modules.
