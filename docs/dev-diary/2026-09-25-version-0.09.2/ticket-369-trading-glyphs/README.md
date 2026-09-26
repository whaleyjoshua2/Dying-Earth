# Ticket #369: glyphs in the Trading window

The designer: *"glyphs in market window."* Decided on the ticket in one round: a glyph at the head
of each row and on every figure, the way the top bar does it; the Buildings sentence and the
Trades-this-turn list by the same rule.

[The resolution](https://github.com/whaleyjoshua2/Dying-Earth/issues/369) is the authority;
[§3 of the spec](../../../spec/version-0.09.2.md#3-glyphs-in-the-trading-window) records it.

## What was built

`trading_window` in `src/ui.rs` alone. The header, the price column, the Buildings sentence and the
trades list go through `text_with_icons`, the one number-then-word rule; each row's name through
`icon_word`; the Buy and Sell buttons through `priced_button`, handed a Cost of Ducats alone, which
is how a glyph reaches a button face. The sell price gained the word "Ducats" so its figure has a
word to trade. No engine change, no data change, no save change.

## The pictures

Both taken headlessly, `window:1400x900`, the Trading window open over Earth.

![The Trading window as the Custodians: glyphs on the header, the row names, the prices and the button faces](trading-custodians-earth.png)

`shot:trading-custodians turns:12 trade:1 panel:0 ducats:120`. **The Custodians, turn 13.** The
header reads *"120 [ducats] to spend this turn (+49 [ducats] last Income)"*; each row opens with
its good's glyph and name; the price reads *"4 [ducats] each; sells for 2.0 [ducats]"*; the buttons
*"Buy for 40 [ducats]"* and *"Sell for 20 [ducats]"*. The Buildings sentence carries no glyph, since
no figure in it is a quantity of a good, which is the rule working.

![The Trading window as the Prospectors, with the discount clause on one line](trading-prospectors-earth.png)

`shot:trading-prospectors player:prospectors turns:11 trade:1 panel:0 ducats:120`. **The
Prospectors, turn 12.** The same window with the 15% off: *"1 [ducats] each; sells for 0.5 [ducats]
(x0.85 for you, over the lot)"*, the longest line the price cell ever holds, on one line.

**The first cut of both pictures was wrong and is not kept.** The price cell wrapped one word to a
line (*"3 [ducats] / each; / sells / for 1.5 / [ducats]"*, and the discount clause eleven lines
tall), because the glyph line wraps at the width it is given and a Grid cell has none until its
content has one. The cell is given a floor of 340 px, sized to the discount clause. The first
Prospectors picture was also under a card the staged turn drew, so it is taken a turn earlier.

## The gate

`cargo clippy --workspace --release --all-targets -- -D warnings` clean, the root crate re-checked;
engine 484 + 6, root 8. No sweep: nothing in the engine moved.
