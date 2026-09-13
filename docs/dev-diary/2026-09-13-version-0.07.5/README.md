# Version 0.07.5, the reading-and-founding version

The map is [Map: version 0.07.5](https://github.com/whaleyjoshua2/Dying-Earth/issues/160). Nothing
in this folder was opened on the designer's desktop: every picture here is a contact sheet written
straight to a PNG by a program that never starts Bevy.

## Candidate symbols for the four Factions

Ticket [#167](https://github.com/whaleyjoshua2/Dying-Earth/issues/167), a research ticket worked by
an agent alone. **Nothing here is chosen**; the choosing is
[#168](https://github.com/whaleyjoshua2/Dying-Earth/issues/168). The findings, the licences and the
collisions are in [`docs/research/faction-symbols.md`](../../research/faction-symbols.md). Forty-eight
candidates from game-icons.net, twelve per Faction, all CC BY 3.0, all fetched unmodified into
`assets/icons/candidates/<faction>/`.

Two shapes of sheet, and one control.

### The deciding sheets, one candidate a row

Each row is one candidate drawn at 96, 64, 40, 28 and 24 pixels as the game draws an icon (the
game-icons backing rectangle stripped, the glyph over the dark panel), then the **28 and the 24
blown up three times without smoothing** -- 24 because that is the exact side of the colour swatch
`faction_card` draws in a Faction card's title row today, and 28 to 40 because that is where a
symbol beside the name would sit. The row numbers match the tables in the findings.

Each Faction has the same sheet twice: `-untinted` (the file's own white) and `-tinted` (that
Faction's colour from `factions.toml`, applied as `egui::Image::tint` applies it).

**[faction-symbols-sheet-custodians-untinted.png](faction-symbols-sheet-custodians-untinted.png)**
and **[-tinted](faction-symbols-sheet-custodians-tinted.png)** (teal, 38/165/153):

1. `ecology` -- hands cupped under a globe; the gesture survives 24, the continents do not.
2. `earth-africa-europe` -- a globe with a dark continent on it; which continent stops reading at 28.
3. `wireframe-globe` -- dissolves: the grid moirés into a plain grey ball.
4. `globe` -- a desk globe on a stand in an arc; the stand and arc hold at 24.
5. `seedling` -- a pot, once the sprout above it has gone.
6. `sprout` -- a stem and two broad leaves; the boldest shape on any of the four sheets.
7. `oak-leaf` -- a leaf; the lobes merge to a smooth blade.
8. `recycle` -- the three-arrow triangle, clean at 24, and the Trade Post's arrow loop.
9. `greenhouse` -- dissolves: the glazing bars grey out to a haze.
10. `water-recycling` -- a dark ring with a notch; a cycle, not water.
11. `scales` -- a thin T with two pans; just holds at 24.
12. `custodian-helmet` -- a British police helmet, unmistakably; the name is a trap.

**[faction-symbols-sheet-prospectors-untinted.png](faction-symbols-sheet-prospectors-untinted.png)**
and **[-tinted](faction-symbols-sheet-prospectors-tinted.png)** (orange, 216/127/51):

1. `miner` -- a figure swinging a tool at rock; at 24 it is the Mine Module's picture.
2. `mining-helmet` -- a hard hat with a lamp barrel; the cleanest shape of the twelve.
3. `gold-nuggets` -- a busy speckle of chunks and sparkles.
4. `dig-hole` -- dissolves: the spade goes into the spoil and leaves a dark mass.
5. `oil-pump` -- a lattice tower with an arm, which is the Relay's and the Shipyard's shape too.
6. `bulldozer` -- body, cab and blade all still separable at 24.
7. `bucket-wheel-excavator` -- dissolves: the most detailed drawing here, a grey tangle at 28.
8. `profit` -- a money bag with arrows spraying upward; both halves hold.
9. `war-pick` -- a thin diagonal at 24, and the Mine Module's tool.
10. `ore` -- a faceted lump with sparkles; costs a new credit line (Faithtoken).
11. `dynamite` -- a banded bundle with a curling fuse; reads at 24.
12. `coins-pile` -- a heap of discs; money, where Ducats already wear a banknote.

**[faction-symbols-sheet-arkwrights-untinted.png](faction-symbols-sheet-arkwrights-untinted.png)**
and **[-tinted](faction-symbols-sheet-arkwrights-tinted.png)** (violet, 112/70/168 -- the darkest
of the four against the panel, and the sheet where the tint costs the most):

1. `meeple-group` -- dissolves into four blobs, and says "board game piece".
2. `three-friends` -- three heads over a picket of one-pixel legs; just reads as people.
3. `caravan` -- a towed holiday trailer, clean.
4. `suitcase` -- a hard case with a handle and two catches, clean.
5. `backpack` -- straps and buckles resolve into noise; a rounded bag.
6. `exit-door` -- an open door with a thick arrow through it; the boldest shape on this sheet.
7. `heaven-gate` -- dissolves: only the cloud base survives 24.
8. `journey` -- a winding road holds; the figure beside it becomes a stroke.
9. `apollo-capsule` -- a truncated cone with a window band, clean.
10. `cargo-ship` -- hull, bridge and waves hold; the containers smear.
11. `moon-orbit` -- a disc with a ring and a moon; bold at 24, and says "a planet".
12. `dove` -- a bird with spread wings, clean; says peace, not departure.

**[faction-symbols-sheet-archivists-untinted.png](faction-symbols-sheet-archivists-untinted.png)**
and **[-tinted](faction-symbols-sheet-archivists-tinted.png)** (crimson, 222/82/111):

1. `bookshelf` -- a row of upright spines; still books at 24.
2. `book-pile` -- a stack of layered slabs; the bookmark tab goes first.
3. `open-book` -- clean at 24, and exactly what the Archive Module already wears.
4. `cloud-upload` -- cloud and arrow both hold; says the thing the Faction does.
5. `hive-mind` -- something large over three people; busy, and the closest to "upload everyone".
6. `brain` -- a mottled bean once the convolutions fill in.
7. `server-rack` -- three shelves on a plinth, all separable at 24.
8. `cpu` -- a square die with a window and pins, clean.
9. `microchip` -- the same object tilted, which aliases the pins; worse than 8.
10. `classical-knowledge` -- an open book on a fluted column; crowded but legible.
11. `wisdom` -- dissolves into a sunburst, which is the Solar Array's shape.
12. `scroll-unfurled` -- a curled sheet with a rolled foot, clean.

### The contact sheets, twelve candidates across

Made with the existing aid,
`cargo run --release --example icon_sheet -- <out.png> assets/icons/candidates/<faction> 28,40,64`:
one column per candidate, in **alphabetical** order (not the numbered order above), each drawn large
and then at 28, 40 and 64 magnified. The quicker view for comparing candidates against each other
rather than against a size.

- [faction-symbols-contact-custodians.png](faction-symbols-contact-custodians.png), left to right:
  custodian-helmet, earth-africa-europe, ecology, globe, greenhouse, oak-leaf, recycle, scales,
  seedling, sprout, water-recycling, wireframe-globe.
- [faction-symbols-contact-prospectors.png](faction-symbols-contact-prospectors.png), left to right:
  bucket-wheel-excavator, bulldozer, coins-pile, dig-hole, dynamite, gold-nuggets, miner,
  mining-helmet, oil-pump, ore, profit, war-pick.
- [faction-symbols-contact-arkwrights.png](faction-symbols-contact-arkwrights.png), left to right:
  apollo-capsule, backpack, caravan, cargo-ship, dove, exit-door, heaven-gate, journey, meeple-group,
  moon-orbit, suitcase, three-friends.
- [faction-symbols-contact-archivists.png](faction-symbols-contact-archivists.png), left to right:
  book-pile, bookshelf, brain, classical-knowledge, cloud-upload, cpu, hive-mind, microchip,
  open-book, scroll-unfurled, server-rack, wisdom.

### The control: what the board already draws

[faction-symbols-sheet-worn-offwhite.png](faction-symbols-sheet-worn-offwhite.png) -- all
thirty-three SVGs in `assets/icons/` at the same sizes, in the kind fill, four to a row,
alphabetical. A candidate that looks like one of these at 24 pixels is a collision whatever it looks
like at 96.

1. colony (a geodesic dome on a stalk), colony_ship (a slanted rocket), ducats (a banknote),
   emissions (a chimney under a plume).
2. energy (a bolt), facility_bank (a columned building), facility_constabulary (handcuffs),
   facility_embassy (a handshake).
3. facility_factory (a sawtooth roof and two chimneys), facility_launch_site (a control tower),
   facility_power_plant (two cooling towers), facility_research_lab (a round-bottom flask).
4. facility_scrubber (a bladed fan in a square), facility_sea_wall (an angled slab), fuel (a
   jerrycan), influence (a megaphone).
5. materials (a mine wagon), module_archive (**an open book with a pen**), module_barracks (a Nissen
   hut with a flag), module_generator (a lumpy machine with a crest).
6. module_mass_driver (a radiation trefoil), module_mine (**a pick striking rock**),
   module_observatory (a dome with a telescope), module_refinery (tank, column, flare).
7. module_relay (a lattice radio tower), module_shipyard (a crane arm on a lattice),
   module_solar_array (**a tilted panel and a spiky sun**), module_trade_post (**two squares and two
   curving arrows**).
8. population (a bust), region (a city skyline), research (a microscope), station (the game's own
   drawing: two panels on a bar).
9. warship (a swept spaceship).

The Army's **shield** is not on this sheet because it is not a file: `shield_glyph` in `src/ui.rs`
paints it as a five-point convex polygon in the same kind fill. No candidate above is a shield, and
the thirty-odd `*-shield` icons on game-icons.net were deliberately left off the candidate lists for
that reason.

### The Tabler test

[faction-symbols-sheet-tabler-untinted.png](faction-symbols-sheet-tabler-untinted.png) -- four MIT
Tabler icons (`leaf`, `pick`, `users-group`, `archive`) through the same path, at the same sizes.

1. leaf -- **black on the panel.**
2. pick -- black.
3. users-group -- black.
4. archive -- black.

A Tabler file is `fill="none" stroke="currentColor"`, `currentColor` resolves to black in `usvg`,
and `Icons::from_ctx` tints by multiplying, so a black glyph stays black in every colour: the tinted
sheet came out **byte-identical** to this one. An MIT icon is adoptable, but not as a drop-in -- it
would need a change to the loader first.
