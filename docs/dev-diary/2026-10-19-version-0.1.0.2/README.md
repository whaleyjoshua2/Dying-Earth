# Version 0.1.0.2, the legibility version

The map is [Map: version 0.1.0.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/547); the spec
is [`docs/spec/version-0.1.0.2.md`](../../spec/version-0.1.0.2.md). The standing baseline is 0.1.0.1's
closing board, **7 / 24 / 19 / 19 and 11 collapses**
([`final-0.1.0.1.txt`](../2026-10-12-version-0.1.0.1/sweeps/final-0.1.0.1.txt)), against the
designer's 15 each and 20.

## Map labels give way

Ticket [#548](https://github.com/whaleyjoshua2/Dying-Earth/issues/548), spec §1.

**The pictures** ([`ticket-548/`](ticket-548/)), taken headlessly in `shot:` mode, seed 3, turn 1:
- `before-earth.png`, `after-earth.png` (`panel:0 look:50,32`): before, Turkey, Iran and Saudi
  Arabia ran together, the United Kingdom lay over the European Union, and China over Japan. After,
  Egypt, Saudi Arabia, India and Indonesia stand as their glyph and name; the European Union and
  China hide, each under a neighbour's Army shield.
- `china-selected-earth.png` (`look:80,35 select:eastasia`): China selected, its full block laid
  over Japan's shield. This is the case that put a question back to the designer (below).
- `before-solar.png`, `after-solar.png`: before, Mars's label lay across Ceres's, and the Earth L5
  name sat under the stack labels. After, the stacks stand whole, the Moon shrinks to its name, and
  Mercury, Phobos and Vesta hide, neutral and further from the middle than the Moon, Mars and Deimos.
- `after-mars.png`: the orbit band, a fixed mark, with the slot labels clear of it.
- `start.png`: the start screen's globe, every name standing.

**Found in building.**
- Labels set by hand a pixel or two apart counted as collisions: the Solar System Map's stack
  labels, 16 pixels apart in blocks 19 tall, knocked each other out in the first picture. Two blocks
  may now overlap by two pixels a side, the padding round their text.
- A neighbour's shield hid China, the player's own Region, even when selected, since fixed marks
  outranked every label. Put to the designer, who chose that **the selected label and the one under
  the pointer are laid over the marks**; every other label still gives way to them.
- Open windows (the Climate Panel, a card) are fixed marks too, so a label under one, which is not
  seen, does not push a seen one aside.

**The tests.** `src/labels.rs` holds the layout, apart from the painting, with nine tests: rank
order, nearness to the middle within a rank, shrink then hide, a mark's owner exempt, the slack,
the margin before growing back, the selected and hovered over marks, and the fade's memory. Each
rule was broken on purpose and its test watched to fail before it was believed.

No rule moves, so no sweep.
