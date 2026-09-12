# Version 0.07.1, the reading-and-reaching version

The map is [Map: version 0.07.1](https://github.com/whaleyjoshua2/Dying-Earth/issues/111).

## Antarctica showed on the start screen

![Without the fix, three markers on the ice; with it, none](antarctica-on-the-start-screen.png)

Reported by the designer against version 0.07.0: the Antarctic sites still showed while
choosing a starting continent. [Nothing of Antarctica is drawn until the ice
opens](https://github.com/whaleyjoshua2/Dying-Earth/issues/103) hid them in a game, and the
start screen is **not** a game: `sync_scene` returns early when `session.game` is `None`, and
that return sits **before** the loop that hides the markers, so they kept the visibility they
were spawned with. Before a game exists the ice is shut by definition, so they are hidden
there too.

It took two attempts to photograph, which is the interesting part. The start globe now opens
aimed at the Faction's home with a latitude tilt, so Antarctica is off-view and the first two
captures showed nothing **either way** — the fix looked confirmed by a picture that proved
nothing. The designer would have seen the markers only after dragging the globe south, which
[the globe ticket](https://github.com/whaleyjoshua2/Dying-Earth/issues/100) made possible in
the same version. Standing the camera where a dragging player stands is what produced the
picture above: three markers without the fix, none with it.

## Looking at the icons at the size they are actually drawn

The designer, reading the top bar: *"currently I don't know what materials is; fuel is jerry can,
energy lightning bolt, and duckets looks like some mineral; tech is a microscope."*

![The five icons as they stand](icons-as-they-stand.png)

Confirmed, and four of the five readings were exact. Left to right that sheet is **ducats,
energy, fuel, materials, research**. Fuel, Energy and Research read correctly. The two that
fail are the two the designer flagged, and they fail symmetrically: **Materials is drawn as
sparkling faceted gems** and reads as treasure, while **Ducats is drawn as a dozen stacked
coins** which, shrunk to the 16 pixels the bar draws, flatten into a lumpy mass — a mineral.
The glyph that looks like money is labelled Materials and the glyph that looks like a mineral
is labelled Ducats.

That vindicates the designer's line *"current ducket icon becomes materials and add new ducats
icon"*, which the charting round had flagged as possibly backwards. It was not backwards; the
charting was reading the filenames rather than the art.

### Candidates, rendered at both sizes

`cargo run --release --example icon_sheet -- out.png [folder]` renders any folder of SVGs twice:
large, and **at the 16 pixels the top bar actually uses**, magnified without smoothing. The
second row is the only one that decides anything.

![Materials candidates](candidates-materials.png)

*anvil, brick-pile, cubes, metal-bar, mine-wagon, packed-planks, rock, stone-pile.* Surviving
16px: **anvil, metal-bar, mine-wagon, packed-planks**. Dissolving: brick-pile, stone-pile (the
same failure as the current ore), rock, cubes.

![Ducats candidates](candidates-ducats.png)

*banknote, cash, gold-stack, money-stack, profit, purse, two-coins, wallet.* Surviving 16px:
**banknote, purse, two-coins, wallet**. Dissolving: gold-stack (the worst of the eight),
cash, money-stack; profit reads as an arrow rather than as money.

**The rule both sheets prove, and the one to hold the three new icons to:** a glyph made of
many small repeated shapes does not survive 16 pixels. Population and Emissions will both tempt
an artist toward lots of little things.

### The two replaced

![The top bar, before and above, after and below](top-bar-before-and-after.png)

The designer chose **the mine cart for Materials** and **the banknote for Ducats**, both
Delapouite's, both from the surviving half of their sheets.

![All five after the swap](icons-after-the-swap.png)

On the bar at its own size: Materials was a cluster of gem shards nobody could name and is now
a laden cart with wheels; Ducats was the lumpy mass that read as a mineral and is now
unmistakably a note. Fuel, Energy and Research are untouched, having read correctly all along.
The Credits screen follows the art, since the licence credits the icon actually used.

### Candidates for the three figures that have no icon

Population, Influence and Emissions are words on the board today. Eight candidates each, rendered
the same way: large above, and at the sixteen pixels the top bar draws, magnified, below. Only the
second row decides anything.

![Population candidates](candidates-population.png)

*character, family-house, family-tree, human-pyramid, meeple, person, player-base, village.*
Surviving: **person, meeple, character, player-base**. Dissolving: family-tree (a diagram, and the
wrong register), human-pyramid and village — both the many-small-shapes failure again.
**Recommended: person**, since population here counts people in a country rather than households
or settlements. **meeple** is the bolder glyph and would suit a game that means to feel like a
board game.

![Influence candidates](candidates-influence.png)

*conversation, crown, megaphone, public-speaker, ribbon-medal, shaking-hands, star-flag, vote.*
Surviving: **megaphone, star-flag, crown, shaking-hands**. Dissolving: conversation (two bubbles
smear into one), public-speaker (the figure vanishes into the podium), vote, ribbon-medal.
**Recommended: megaphone** — Influence is spent to win countries over, which is projection rather
than diplomacy between equals. **shaking-hands** reads as diplomacy if that is wanted; **crown**
implies you already rule the place.

![Emissions candidates](candidates-emissions.png)

*barrel-leak, chimney, cloud, coal-pile, coal-wagon, factory, gas-pump, smoking-volcano.*
Surviving: **factory, gas-pump, cloud, chimney**. Dissolving: barrel-leak, coal-pile.
**Recommended: factory**, the most legible on the sheet, and it names the source — the game's
Emissions are literally industry. **chimney** if the smoke is wanted rather than the thing making
it. **coal-wagon is out regardless**: Materials is a mine cart now and the two would be twins at
sixteen pixels.

### The three chosen, and in place

The designer chose **the head-and-shoulders bust for population** (`character`), **the megaphone for
Influence**, and **the chimney for Emissions** — all three Delapouite's, all three from the surviving
half of their sheets. Two of the three went against the recommendation above, and both departures
are defensible: `character` is the same silhouette as `person` with a heavier shoulder line, which
is what makes it hold at sixteen pixels; and `chimney` names the smoke rather than the building,
which is the more honest label for a figure that counts eight sources and a sink rather than
factories.

![All eight, large above and at sixteen pixels below](icons-all-eight.png)

Left to right: **ducats, emissions, energy, fuel, influence, materials, population, research**. The
bottom row is the deciding one. The bust is the cleanest of the eight at size. The chimney is the
weakest: its smoke plume takes most of the sixteen pixels and the brick stack below narrows to a
couple of columns, so it reads as *smoke* rather than as *chimney* — which, for a figure named
Emissions, is arguably the right failure.

![The Influence figure on the bar; population, Influence and Emissions on a Nation State's card](three-figures-in-place.png)

The rule the designer set for the resources — *"use the same way"* — is applied without exception:

- **On the top bar the glyph replaces the word.** Only Influence appears there, and it now reads
  `🕪 16 of 16` where it read `Influence 16 of 16`.
- **Everywhere else the glyph sits beside the words**, which is the Nation State card's population,
  Influence and Emissions lines, the card's Influence section, the Colony panel's Influence
  section, and the Climate Panel's Emissions block.
- **Inside a tooltip the glyph stands in for the word**, as it already did for the five resources.
  Population and Emissions needed their lower-case spellings added to that table as well, since the
  game's prose capitalises the five resources and Influence but writes those two mid-sentence.

The Credits screen carries all eight, and its heading is no longer "Resource icons", since three of
the eight are not resources.

### The glyphs come down into the lists, and Emissions joins the top bar

The designer's answer to *what does the Emissions glyph label*: everywhere it appears — the dense
lists, the Blame block, **and a new figure on the top bar**.

![A Nation State's Facility list](facility-lines-with-glyphs.png)

`Factory (coastal): +7 Materials, 2 Energy upkeep, 1.2 Emissions` is now
`Factory (coastal): +7 🛒, 2 ⚡ upkeep, 1.2 🏭`. This is the line a player compares two buildings
across, and the words were most of its width.

**The first attempt was wrong and the picture caught it.** Version 0.07.0's tooltip renderer swaps a
word for its glyph *anywhere* it appears, and pointed at a list that ate the word out of a
building's own name: `Research Lab (inland): +2 Research` came out as `🔬 Lab (inland): +2 🔬`. The
rule for a list is therefore narrower than the rule for prose — **a word is traded for its glyph
only directly after a number**, since that is what makes it a figure rather than a name. Tooltips
keep the old rule, because a tooltip is prose. A second, smaller fault in the same picture: the
commas floated a space away from their glyphs, because egui applies item spacing *after* a widget,
so the gap to close is the one the image leaves behind rather than the one before the punctuation.

![The Blame block, each ppm figure marked](blame-block-with-glyphs.png)

Every Blame figure is in ppm and so every one of them carries the glyph; *share* and *thresholds*
are not ppm and keep their words. `ppm` means Emissions **here and nowhere else** — four lines above
this block the same three letters are the CO2 Stock and a Scrubber's pull on the Natural Sink, and a
chimney against either of those would be a lie — so the renderer takes that word as a local extra
rather than learning it globally.

Note what the picture shows about **colour**, which is the question still open on this ticket: the
chimneys in the Blame block are already **Faction-coloured**, because the line takes the seat's
colour and the glyph takes the line's tint. Colour on an icon already means *whose* in at least one
place.

![Net Emissions on the top bar](net-emissions-on-the-bar.png)

`🏭 +47.5 ppm` sits after the Temperature. Until now the only way to learn whether the world went
over or under the Natural Sink this turn was to open the Climate Panel; the Temperature beside it
moves far too slowly to answer that question.

### Two building aids this needed

Photographing the Blame block turned out to be impossible, and the reason is a finding in itself.
The Climate Panel settles at about **400 rows tall whatever room it is given**, scrolls the rest,
and a scroll cannot be driven headlessly — so **the Blame block is below the fold for a player at
1280x800 too**, until they scroll or drag the panel. That is pre-existing, from ticket #53, and not
something this ticket introduced, but it is worth writing down.

Two aids now exist, neither part of the spec: `window:<w>x<h>` sets the off-screen window's size,
and `climate:top` opens the Climate Panel at the top of the window at full height. Together they
photograph a panel taller than the screen.
