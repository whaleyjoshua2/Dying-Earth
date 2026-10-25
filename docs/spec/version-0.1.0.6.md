# Dying Earth — version 0.1.0.6, the fission version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.1.0.6](https://github.com/whaleyjoshua2/Dying-Earth/issues/627), and the
pictures and sweeps that decided it are in
[`docs/dev-diary/2026-10-25-version-0.1.0.6/`](../dev-diary/2026-10-25-version-0.1.0.6/).

**The baseline** is 0.1.0.5's closing board,
[`final-0.1.0.5.txt`](../dev-diary/2026-10-24-version-0.1.0.5/sweeps/final-0.1.0.5.txt):
**18 / 61 / 0 / 1 and 0 collapses**, against the designer's 15 each and 20.

**What the version is.** Version 0.1.0.5 with the designer's list of 2026-10-25. Its spine is a new
resource, **Fissionables**, found off Earth and richest in the far worlds (§1).

**What it did to the win column** (80 games, per Faction; every section's own sweep is in it):

| | 0.1.0.5 | §1 Fissionables | the ideal |
|---|---|---|---|
| Custodians | 18 | 23 | 15 |
| Prospectors | 61 | 51 | 15 |
| Arkwrights | 0 | 3 | 15 |
| Archivists | 1 | 1 | 15 |
| collapses | 0 | 2 | 20 |

## 1. Fissionables, the seventh resource, and Deep Mining and Rich Seam moved

Ticket: [Fissionables: a new resource found off Earth, by Body and by slot](https://github.com/whaleyjoshua2/Dying-Earth/issues/628).

**The resource.** Fissionables are held in the Stockpile (a seventh figure, carried to the tenth as the
others are), bought, sold and traded between seats as Fuel is. None is found on Earth.

**What makes them: the Mine** (the designer: *"we're using the existing mine"*). A Mine makes them
beside its Materials, `fissionables = 1.0` on its card in `modules.toml`, times:

- **its site's figure**, a fifth slot yield drawn at the start within `slot_yield_spread` (0.25) of
  its Body's `fissionables_yield` in `bodies.toml`: Earth 0, the Moon 0.25, Mars 0.80, Phobos and
  Deimos 1.1, Venus 0 (no ground), Mercury 1.3, **Ceres 1.6** (the list's "Eris", which the game does
  not have; the designer: *"yeah Ceres"*), Vesta 1.8;
- **the Faction's output multiplier**;
- **the Mine's Techs** -- Deep Mining, the Extraction Charter and Beneficiation, in the Mine's one
  chain (the designer: *"the techs will effect fisionables but not the card"*).

Nothing else reaches them: **no Discovery** (Rich Seam, the Deep Core Survey), no Mass Driver, and,
from the review, **neither Production Moved's factor nor a shut Habitat's halving**, which the first
build had let reach them though the designer named neither; they lift and halve the Materials alone.
A starved Colony's Mine makes none. A **station's**
Mine makes none: they are in the ground. The figure is to the tenth.

**Which sites hold them.** Every ground site of a Body with a figure, save **Hellas Planitia** on Mars
(`no_fissionables = true`; the designer: *"pick one mars slot to not have it"*), and on **the Moon two
or three of its five**, drawn with the game (`fissionables_sites = [2, 3]`; *"never more never less"*).
The draw runs on **dice of its own**, seeded from the game's seed (`FISSIONABLES_DICE`), so the game's
own dice -- every yield the version before drew and every roll after -- fall as they did.

**Deep Mining x1.25**, from x1.5, and **Rich Seam x1.5, x2 with Deep Mining**, from x2 and x3, at the
designer's word (*"Deep Mining should be 1.25 and rich seam 1.5/2"*). Rich Seam reads figures of its
own (`rich_seam_multiplier`, `rich_seam_multiplier_with_tech` in `events.toml`); the Ice Deposit and
the Helium-3 Vein keep the shared x2 and x3. Its card and the Tech's line say *Materials*.

**The market.** A fourth row, **8 Ducats** the midpoint of its band (`per_fissionables` in
`factions.toml`), selling back at half as Materials and Fuel do. The Market's arrays grew to four rows;
a save's three-row arrays read with the fourth at nought.

**Where it shows.** The top bar beside Fuel, its hover the Fuel's (each source last Income); the
radioactive trefoil in **lime** (140, 235, 60), the designer's **1A** off a sheet of six glyphs in four
colours, 48.6 from Energy and 49.9 from Ducats in CIELAB, its nearest. A site's yields line and its map
label carry the fifth figure where it has one and nothing where it does not; a Body's line reads its
card. A Colony's Output row, a Mine's figures, the Trading window's fifth line, a Trade's goods and a
rival's Income row on the Factions window carry it. The text tool prints it in the Stockpile and the
Market, as **Fi** in the slot lines and the Last income (**F** is the Fuel's), and takes it in `buy`,
`sell` and `trade`.

**The computer.** It reads every figure from the tables; it builds Mines as it did, so its Mines make
Fissionables where they stand. It does not chase them, and spends none: that is
[The computer chases Fissionables](https://github.com/whaleyjoshua2/Dying-Earth/issues/643), once
what they buy is built.

**Saves.** `SAVE_VERSION` stays 12. A save from before holds none in any Stockpile or slot and three
market rows; it loads, and the load draws the sites the seed would have drawn, on the same dice.

**Measured** ([`sweeps/ticket-628.txt`](../dev-diary/2026-10-25-version-0.1.0.6/sweeps/ticket-628.txt)):
**23 / 51 / 3 / 1 and 2 collapses**, from 18 / 61 / 0 / 1 and 0. Deep Mining's cut took a fifth off the
Materials (the Prospectors 2,618 to 2,046 a game, the Custodians 2,135 to 1,781), and the Prospectors
fell 10 wins. Fissionables made, a game, net: the Custodians 56.5, the Prospectors 36.9, the
Arkwrights 2.5, the Archivists 2.2. (The first build's sweep read the Custodians at 111.7, Production
Moved doubling their Mars Mines' Fissionables; the win column was the same to the game.)
