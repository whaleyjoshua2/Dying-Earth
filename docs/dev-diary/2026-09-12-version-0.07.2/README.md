# Version 0.07.2, the atlas version

The map is [Map: version 0.07.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/120). Branch
`version-0.07.2`, cut from `main` after version 0.07.1 merged as
[pull request #119](https://github.com/whaleyjoshua2/Dying-Earth/pull/119).

## The Nation card: plain verbs, prices in glyphs, and no Influence controls

Ticket [#121](https://github.com/whaleyjoshua2/Dying-Earth/issues/121). The designer's four lines,
answered (a) each:

> Remove "(free)" in all cases that don't refer to build slots e.g mothball; decommission; load emigrant
> build portion of the nation card should use icons rather than words for built cost e.g no (25 materials) just +25 ICON
> build portion of the nation card needs to adjust mouse over to exclude costs - that's already stated. Also the words "once it stands" replace that part with the turn cost
> remove buttons to buy/spend influence from nation card

![Before and after](nation-card-before-and-after.png)

**"(free)" came from one place** — `Cost::text()` prints `free` for an order with no price, and every
button printed its price in parentheses. The button no longer prints a price it does not have:
`Mothball`, `Decommission`, `Muster 4 Emigrants` are plain verbs. The slot rows' `free`, which is a
count of empty slots, is untouched, as asked.

**A price is a figure and a glyph, unsigned.** `Factory (25 Materials)` is `Factory 20 🛒`, and the
Ducats alternative beside it is `or 40 💵`. The designer wrote `+25`; the figure carries **no sign**
because every other figure on the card is unsigned and the glyph already says it is a cost — offered
as a choice and taken. egui's own `Button` cannot hold an image mid-text, so a priced button is a
clickable group drawn in the button's own visuals (`UiBuilder::sense` for the response, the style's
`interact` for the hover and press colours); a free order stays a plain `Button`. A disabled one
dims the way a button does — Leapfrog, in the picture.

![The build hover](build-hover-says-how-long.png)

**The hover stopped repeating the price and started saying how long.** `Costs 20 Materials. Once it
stands: …` is now `Ready next turn: +9 🛒, 2 ⚡ upkeep, 0.8 🏭`, from the building's own `build_turns`.
Eight of the ten Facilities take one turn, so it is *next turn* far more often than *in 2 turns*.

**The Influence controls went; the figures stayed.** The spend box, the Spend button and the
"Buy more Influence in the Trading window" button are gone from a Nation State's card, since the
Command Cluster spends on the selected place and they had become a second copy. The Standings row,
the threshold line and the Blame note stay where 0.07.1 moved them, at the top — they are what a
player reads before pressing Spend in the corner. **A Colony's card keeps its controls**, that not
being what was asked; it is the obvious follow-up question and is on the ticket as one.

![East Asia's card as built](east-asia-card-as-built.png)

**The `tip:<word>` aid learned to fire once a frame.** Pointed at a card of twelve build buttons with
`tip:Ready`, it opened twelve tooltips at once, which is a picture of nothing. It now fires on the
first match per frame, and the build hovers go through `rule_tip` so the aid can reach them at all.

Clippy clean with `-D warnings`, 254 tests passing.
