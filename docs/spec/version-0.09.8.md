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

## 2. The population and GDP lines on the cards

The authority is [ticket #475](https://github.com/whaleyjoshua2/Dying-Earth/issues/475).

- **The Region card's population line** reads "Region population 385.6 (386M)" behind the people
  glyph, and no more. Its hover, the growth rule, is unchanged.
- **The Region card's GDP line** reads "GDP 23: Industry Level 3, leans Fuel", behind the Ducats glyph.
- **What the GDP line said before is its hover**, three lines: what the Region pays its controller
  in Ducats a turn; the rule, GDP x Industry Level / 5, never below 1; and what the player's Bank
  would add there. A rival's pay the player cannot see stays hidden.
- **The Colony card's people line** reads "14/16 (14.2M)" behind the people glyph: Colonists over
  room, then the people as a real number to a tenth of a million.
  - The bracket counts the part-grown next Colonist, so it moves each turn the place grows.
  - The figure before the slash stays whole; it is what room and Module slots count.
  - "14/16" keeps its amber at three-quarters full and its red when full; the bracket is plain.
- No rule moves, and nothing a computer seat reads.

**Pictures:** [`ticket-475/`](../dev-diary/2026-10-04-version-0.09.8/ticket-475/).

## 3. Raise Industry Level as a tile

The authority is [ticket #476](https://github.com/whaleyjoshua2/Dying-Earth/issues/476).

- **Raise Industry Level is a tile among the Facility boxes**, last of all, after the final inland
  box: dashed like a free slot and reading "Raise Industry Level". The button in the Policies block
  and the note under it are gone.
- **One click orders the raise.** The rule is unchanged: it is paid at End Turn, built through the
  Region's Widgets queue, and one may be under way in a Region at a time.
- **While it is ordered or under way the tile is drawn like any building being built**: hatched,
  "ordered" and "0 of 4" before End Turn, "building" and the Widgets done after, with "Industry
  Level" beneath it. A right-click takes back an order placed this turn.
- **When it completes** the Region has one more inland slot, drawn as a free "Click to Build" box,
  and a new Raise tile stands after it.
- **Its hover** gives the price, when it is ready, and "Adds an inland slot." Where the order would
  be refused the tile is greyed and the reason leads the same hover.
- **Its words and dashes are a muted red**, so it is told from a free slot at a glance; dimmed
  where the order would be refused.
- **Only on a Region the player directs.** A rival's or a neutral Region shows no Raise tile.
- The Widgets queue line for a raise stays. No rule moves, and nothing a computer seat reads.

**Pictures:** [`ticket-476/`](../dev-diary/2026-10-04-version-0.09.8/ticket-476/).

## 4. The tutorial, 14% fewer words

The authority is [ticket #477](https://github.com/whaleyjoshua2/Dying-Earth/issues/477).

- **The tutorial's seven notes hold 398 words, where they held 462**, counted as the words of each
  note's heading, text and quieter line. The designer asked for 10% off, which is 415.
- **Seven passages were cut**, on turns 1, 2, 3 and 5; the ticket holds each before and after.
  Turns 4, 6 and 7 are as they were, the designer's closing sentence on turn 7 among them.
- No step the player is asked to take was dropped. Two statements went with the words: that
  recruiting takes people from the Region (turn 3), and that the Scrubber is the only thing that
  visibly moves the climate (turn 5). Turn 5 says "Region" where it said "state".
- **No test holds a ceiling**, at the designer's word: the notes are to stay free to rewrite.

**Pictures:** [`ticket-477/`](../dev-diary/2026-10-04-version-0.09.8/ticket-477/).
