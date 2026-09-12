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
