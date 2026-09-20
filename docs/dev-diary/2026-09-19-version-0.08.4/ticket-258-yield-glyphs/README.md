# The yields at a founding go to glyphs, and the button that never had them

Ticket [#258](https://github.com/whaleyjoshua2/Dying-Earth/issues/258) on
[map #254](https://github.com/whaleyjoshua2/Dying-Earth/issues/254).

Two new shot aids made these possible: `site:<body>,<slot>` opens an empty Colony Slot's panel in
that Body's picture, and `settler:<body>` parks a Colony Ship of seat 0's with eight Colonists at
that Body and selects it, so the Ship card's founding buttons can be seen. Nothing composed either
board before.

| picture | what it shows |
|---|---|
| [`before-site-panel-mars.png`](before-site-panel-mars.png) | **Before.** Valles Marineris, empty. *Yields here: Materials x1.37 - Energy x0.78 - Fuel x1.29 - Research x1.37* and the Body's line beneath -- both in **words**. |
| [`before-ship-card-moon.png`](before-ship-card-moon.png) | **Before, and the surprise.** The Ship card at the Moon, four *Found a Colony at…* buttons -- and their faces read *Materials x1.54 - Energy x1.57 …* in **words too**. Ticket #218 (version 0.08.2) said this button carried the yields "in glyph and number". It never did. |
| [`after-site-panel-mars.png`](after-site-panel-mars.png) | **After.** Both lines as glyph-and-figure rows, the Body's weak beneath, the notation the map labels have used since ticket #113. |
| [`after-ship-card-moon.png`](after-ship-card-moon.png) | **After.** The four founding buttons with glyph rows on their faces. |

## What was decided, in the designer's words

- *"both"* -- the slot panel's two lines go to glyphs.
- *"keep it"* -- the Body's line stays, weak beneath.
- *"that button needs yeilds"* -- the Ship card's founding button. The ticket recommended leaving
  it, on the belief that it already carried glyphs. The picture said otherwise, and the designer
  was right to insist.

## Why the button never had glyphs

The one glyph rule (`draw_with_icons`, tickets #112 and #116) trades a word for its glyph **only
where the word follows a figure**: "30 Materials" becomes a crate, "Materials x1.37" does not,
because the figure comes after the word. `slot_yield_hover` (ticket #211) wrote the yields in the
second form, and `found_button` (ticket #218) fed that line to `text_with_icons` expecting glyphs
back. Every line it ever drew was words. No test reads a rendered button, and the founding button
had not been photographed until this ticket.

The fix draws the row directly -- glyph, then figure, four times -- in a new `slot_yield_row`, so
it cannot fall through the rule again. The panel and the button share it, as they share the
decision.
