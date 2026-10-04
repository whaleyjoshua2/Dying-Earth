# Dying Earth — version 0.09.8, the Moon race version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.8](https://github.com/whaleyjoshua2/Dying-Earth/issues/473), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-04-version-0.09.8/`](../dev-diary/2026-10-04-version-0.09.8/).

**What the version is.** Version 0.09.7 with the designer's list. The summary and the win column
are written when the version closes.

## 1. Card sub-headers: larger, white, a glyph leading each

The authority is [ticket #474](https://github.com/whaleyjoshua2/Dying-Earth/issues/474).

- **Every sub-header on the Region card and the Colony card is drawn one way**: its glyph, then its
  words in white at 14.5 points, where the body text is 12.5. The glyph is 17 pixels and keeps its
  own fill.
- **The Region card's headings:** Influence, Pioneers, Policies, Facilities, Armies, Orders.
- **The Colony card's headings** (a station's card is the same card): Modules, Armies, The Archive,
  Orders, Ships, Influence. **Modules and Influence are headings now**, where each was a plain line.
  - The warning that rode on the Modules line when a place is full stands under the heading.
  - The "Influence:" label beside the amount is gone; the heading says it, and the label's hover is
    on the amount.
- **Influence's heading is white**, where it was the body's grey.
- **The Armies heading takes the shared size.** Its list keeps the tenth it has had since 0.08.7.
- **A rule is drawn below the Policies block**, so the Widgets line and its queue read with the
  Facilities they build.
- **A rule is drawn above the Armies block** on both cards.
- **The glyphs:**

  | Heading | Glyph |
  |---|---|
  | Influence | Influence |
  | Pioneers | the Colony Ship |
  | Policies | a gavel on its block, drawn for the game |
  | Facilities | the Factory's |
  | Modules | the Habitat's |
  | Armies | the Battle mark |
  | Orders | a list of three rows, drawn for the game |
  | Ships | the Warship |
  | The Archive | the Archive Module's |

- No rule moves, and nothing a computer seat reads.

**Pictures:** [`ticket-474/`](../dev-diary/2026-10-04-version-0.09.8/ticket-474/).
