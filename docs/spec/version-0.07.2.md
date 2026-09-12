# Dying Earth — version 0.07.2, the atlas version: regions drawn along real borders and named for their primary powers, a start chosen on the map, and the card, roster and bar tidied

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.07.2](https://github.com/whaleyjoshua2/Dying-Earth/issues/120), and the
pictures that decided most of it are in
[`docs/dev-diary/2026-09-12-version-0.07.2/`](../dev-diary/2026-09-12-version-0.07.2/).

**What the version is.** Version 0.07.1 finished the reading of the board's figures. This one
redraws the board itself: the Earth Map's regions follow real country borders, there are fourteen of
them instead of twelve, each is named for the nation that leads it and wears that nation's flag, each
has a colour of its own, and the game begins by clicking one of them on the globe. Three smaller
tidyings ride with it — the Region card, the roster, and the top bar — and **no rule of play and no
balance number changed**. The sweep baseline restarts, because the board it is measured on is new.

---

## 1. The Earth Map is drawn along real borders

*Ticket [#125](https://github.com/whaleyjoshua2/Dying-Earth/issues/125).*

The region mask (`assets/textures/earth_states.png`, 2048 by 1024) is generated from **Natural Earth
Admin 0 countries at 1:50m** (public domain), rasterised by `examples/prep_assets.rs`, under one rule:
**a country belongs whole to one Region, and an island belongs to its country.** The coastline still
comes from the photograph; the 3.6 % of land pixels no polygon reached take the nearest Region by a
flood from the pixels that have one. 1:110m was rejected as indistinguishable at this width while
dropping Singapore and Malta.

The country-to-region table is `region_for()` in the generator, readable line by line. Its decided
points: **Cyprus to the European Union** (the one exception to geography); Turkey, Iran and the
Caucasus in Iran's Region; **Kazakhstan whole to China's**, ending the one place the old lines split a
country; all of Indonesia to Indonesia's, with the real border across New Guinea; **Greenland with the
United States' Region**; French Guiana, Réunion and Mayotte with the European Union, being parts of
France. No Faction's home moved.

---

## 2. Two more Regions: Japan and the Arabian Peninsula

*Ticket [#125](https://github.com/whaleyjoshua2/Dying-Earth/issues/125).*

Fourteen Regions. **Japan and Korea** are cut out of China's Region and **the Arabian Peninsula** out
of Iran's — the designer's split of the difference between the research's economy-first and
geography-first pairs. Ids `japan` and `arabian_peninsula`, mask values 14 and 15.

By the sharing rule of ticket #53, a split hands out its parent's figures exactly: China 16.4 / 23 / 4
becomes 14.4 / 17 / 3 with Japan 2.0 / 6 / 1; Iran 3.5 / 5 / 4 becomes 2.5 / 3 / 2 with the peninsula
1.0 / 2 / 2 (population in hundreds of millions / GDP / Influence value). The rest of each card is
hand-set and written beside it in `nation_states.toml`: Japan Industry 3, leaning Energy, Exposure 2,
Education 1.5, two Factories and a Power Plant; the peninsula Industry 2, leaning Fuel, Exposure 1, a
Refinery and a Power Plant, the Middle East's Emissions figure. Neighbours: Japan with China and
Russia; the peninsula with Iran, Egypt across the Sinai, and Nigeria across the strait to the Horn.

Japan's first draft carried a start Research Lab; the rule that only two Regions do (#69) won, and the
card was corrected rather than the rule.

---

## 3. A Region is named for its Nation, and wears its flag

*Ticket [#122](https://github.com/whaleyjoshua2/Dying-Earth/issues/122).*

**Renamed, not captioned.** Every Region is called by its Nation everywhere a name is said — map
labels, cards, roster, Report lines, the start screen. The words are **Region** for the territory and
**Nation** for the power (the designer's own word); *Nation State* is retired from the interface and
the glossary keeps its history. Ids did not move: `south_asia` is still India's id, so saves and
tests are untouched.

| Region | Nation | flag |
|---|---|---|
| East Asia | China | `cn` |
| Japan and Korea | Japan | `jp` |
| South Asia | India | `in` |
| South-East Asia | Indonesia | `id` |
| Australia and Oceania | Australia | `au` |
| Russia | Russia | `ru` |
| Europe | The European Union | `eu` |
| The Middle East | Iran | `ir` |
| The Arabian Peninsula | Saudi Arabia | `sa` |
| North Africa | Egypt | `eg` |
| Sub-Saharan Africa | Nigeria | `ng` |
| North America | The United States | `us` |
| Central America and the Caribbean | Mexico | `mx` |
| South America | Brazil | `br` |

The European Union and Iran were the designer's choices over Germany and Saudi Arabia for their
Regions. **The flag draws on the card title at thirty-two pixels with the name set to match**, the
research having measured that an emblem dissolves at sixteen and returns at thirty-two. Flags are
flag-icons (MIT), fourteen SVGs in `assets/flags/` with the licence text beside them, named on the
Credits screen though MIT asks nothing on screen. The flag draws nowhere else: map labels and roster
rows wear the Region's kind glyph (section 8) instead.

---

## 4. Every Region has a colour of its own

*Ticket [#126](https://github.com/whaleyjoshua2/Dying-Earth/issues/126).*

Each Region's card carries a `colour`. **A Region nobody holds wears it on the globe at 0.35
strength; a held Region wears its holder's Faction colour at 0.45 with the light coastline**, so the
two read apart; the start screen, where nothing is held, shows all fourteen in their colours with
their borders. The mask generator's preview reads the same numbers, so the picture it draws and the
board a player sees agree.

Six colours were moved after measurement in CIELAB showed them within reach of a Faction's — Brazil's
orange was the Prospectors' orange at a distance of 8 — and the designer took the six proposed: Brazil
salmon, Iran cream, Australia lavender, the Arabian Peninsula blue-violet, China brick, Egypt pale
rose, each 29 or more from every Faction and 30 or more from its neighbours. India and the United
States are both green and 11 apart, left as opposite sides of the globe.

---

## 5. The start is chosen on the map

*Ticket [#126](https://github.com/whaleyjoshua2/Dying-Earth/issues/126).*

The list of buttons is gone. Every Region's name is painted on the start globe in its colour; **the
Region under the pointer is lit** at 0.7 strength with its card in the panel — flag, name, population,
Industry, lean, education, Influence value, GDP; **a click chooses and Begin starts**, since real
borders put small Regions beside large ones and a mis-click on Japan should cost nothing. A click
answers by the mask — the border a player sees — not by the nearest label, which on real borders gave
Kazakhstan to Russia and the Nejd to Iran.

---

## 6. The Region card is tidied

*Ticket [#121](https://github.com/whaleyjoshua2/Dying-Earth/issues/121).*

- **Nothing replaces "(free)".** A button with no price prints no price: `Mothball`, `Decommission`,
  `Muster 4 Emigrants` are plain verbs. The slot rows' *free*, a count of empty slots, is untouched.
- **Prices are glyphs, unsigned.** `Factory (25 Materials)` is `Factory 20 [cart]`; the Ducats
  alternative stays beside it as `or 40 [banknote]`, two prices to click. A priced button is a
  clickable group in the button's own visuals, since egui's `Button` cannot hold an image mid-text.
- **The build hover says how long**: `Ready next turn:` or `Ready in N turns:` from the building's own
  build time, and the cost line is dropped, being already on the button.
- **The Influence controls are gone from the card** — the spend box, Spend and the Trading-window
  button — since the Command Cluster spends on the selected place. The Standings, the threshold and
  the Blame note stay at the top.

---

## 7. The roster marks each row that wants an order

*Ticket [#127](https://github.com/whaleyjoshua2/Dying-Earth/issues/127).*

**A ring at the end of every row that can want an order: open while it still wants one this turn,
filled once it has one.** Open in the warm amber the 0.07.1 count wore; filled in a quiet grey. A
Ship in transit and an Army wear no ring, neither being able to want anything (an Army keeps the
stance it was last given, as 0.07.1 decided). **The heading's count and its filter are gone** after
one version, at the designer's word; the heading is a heading again.

---

## 8. Every kind of thing wears a glyph

*Ticket [#127](https://github.com/whaleyjoshua2/Dying-Earth/issues/127).*

Five **kind glyphs**, off-white, in front of a thing's name **everywhere the interface names it**:
roster rows, the Earth Map's Region labels and the start globe's names, the Solar System Map's stack
and transit labels and the orbit band, the slot labels on a Surface Map, the Colony and stack card
titles, and every Report line that points at a Region or a Colony. Chosen off a sheet at fourteen and
sixteen pixels:

| kind | icon | author |
|---|---|---|
| warship | Spaceship | Delapouite |
| Colony Ship | Rocket | Lorc |
| station | Defense Satellite | Delapouite |
| Colony | Habitat Dome | Delapouite |
| Region | Modern City | Delapouite |

An **Army** wears the shield the Earth Map already draws for one — drawn, not loaded. A **Carrier**,
being a transport, wears the Colony Ship's glyph; a stack with one warship in it wears the warship.
All six share one named **kind** fill in the palette table, so "off white" is one number in one place
and cannot drift with the neutral fallback. On this board a colour says *whose*; a kind glyph says
*what*. The Region card keeps its flag and takes no glyph.

---

## 9. The top bar

*Ticket [#128](https://github.com/whaleyjoshua2/Dying-Earth/issues/128).*

- **Influence stands left of Research**; the race bar and the Pick a Tech button travel with Research.
- **Buttons one step larger**: 15-pixel text, 10 by 5 padding; the figures unchanged; one row at 1280.
- **A key for every button, named in its text**: `Tech Tree (T)`, `Climate Panel (C)`, `Victory (V)`,
  `Trading (R)`, `Save (Ctrl+S)`, `To <Body> (Tab)` / `Solar System Map (Tab)`, `Back (Esc)`. Each
  toggles its window as the button does; Save keeps the no-save-while-orders-are-pending rule.
- **End Turn (Enter)** in the Command Cluster, and no other key there. Button and key go through one
  function: dead while a Tech pick is owed or a popup is up, the confirmation when Influence is
  unspent, none for a spectator. A key can never do more than its button.
- **Behind a popup only Esc does anything**, with one exception: Enter on the End Turn confirmation
  confirms it. Every key still stands down while a text box has focus.

---

## 10. Builder's calls, for the designer to veto

Collected from the tickets; each is written beside its ticket's answer.

- **French Guiana, Réunion and Mayotte are the European Union's**, being parts of France (#125).
- **A leading *The*** on *The European Union* and *The United States*, following *The Middle East*
  before them (#122).
- **A click on the start globe answers by the mask**, not the nearest label (#126).
- **Strengths of 0.35 neutral / 0.45 held / 0.7 lit** live in the composer, being about the picture and
  not the game (#126).
- **The Region card keeps its flag and takes no kind glyph**; a third item in its title row read as
  clutter (#127).
- **A Carrier wears the Colony Ship's glyph**; a Report line about a Body wears none (#127).
- **The ring sits right after the row's button**, not at the panel's far edge (#127).
- **Enter on the End Turn confirmation confirms it**; button and key share `press_end_turn` (#128).
- **Unsigned prices** on the card, `20 [cart]` rather than `+20`, since every other figure there is
  unsigned and the glyph already says it is a cost (#121).

---

## 11. Building aids added or changed this version

None are part of the game; each exists because something could not otherwise be looked at.

| aid | what it does | why it had to exist |
|---|---|---|
| `start:<state>` | the start screen with that Region already chosen | a click cannot be made in a headless picture |
| `attend:1` | the standing Defence order on for seat 0, so orders are placed | a filled roster ring is otherwise a click (wants `threat:1`) |
| `select:<state>` | now matches the enum name, case-insensitive (`eastasia`) | as before, but the ids and the names have parted |
| `tip:<word>` | fires once a frame | `tip:Ready` against twelve build buttons opened twelve tooltips |
| `prep_assets --borders <geojson>` | rasterises the country borders into the mask, with a preview | the map is generated, not drawn |
| `roster:filter` | **removed** with the filter | — |

---

## 12. What was measured

- **The suite is 255 tests**, clippy clean with `-D warnings`. One test is new: every SVG in
  `assets/icons/` has its credit and every credit its file, witnessed red with `region.svg` hidden
  (`left: [..12 names]  right: [..13 names]`) and green restored. Fourteen tests that pinned the
  old twelve-region figures and graph were moved to the new ones, each with its reason written beside
  it; one had asserted a seed-dependent coincidence (eight Emigrants landing in a fresh Colony) and
  now asserts what it meant, that nobody is lost.
- **Colours were measured, not eyeballed**: every Region colour against every Faction colour and its
  neighbours in CIELAB, with 15 as confusable and 25 as safe.
- **Flags at label size**: the research measured a tricolour surviving sixteen pixels and an emblem
  not, which is why the card draws them at thirty-two and the labels draw none.
- **A whole game was played from seat 0 headlessly** through
  `cargo run -p dying-earth-engine --example play`, seed 11, Custodians from Europe, giving no order
  but the owed Tech picks: 29 turn commands, **none refused**; the world Collapsed at +3.0 C on turn
  29 with nobody winning and ten Techs done, the idle Custodians having built no Scrubber -- the
  same shape as the 0.07.1 run, on the new board.

---

## 13. The sweep baseline restarts here

Twenty seeds in each of four seatings on the standing instrument, `simulate:<seed>
--player=<faction>`, on **fourteen Regions**. The instrument seats seat 0 in China and spreads the
computer players by the highest untouched Industry, which on the new graph puts the third of them in
Saudi Arabia. **Nothing measured before this table is comparable to it**: the board is different.

| seat 0 | wins | Collapses | tree completes |
|---|---|---|---|
| Custodians | Custodians 12 | **8** | 20 of 20 |
| Prospectors | Custodians 20 | 0 | 20 of 20 |
| Arkwrights | Custodians 19, Arkwrights 1 | 0 | 13 of 20 |
| Archivists | Custodians 19, Archivists 1 | 0 | 20 of 20 |
| **totals** | **Custodians 70**, Arkwrights 1, Archivists 1, Prospectors 0 | **8** | **73 of 80** |

This reproduces the figure ticket #125 recorded when the map landed, to the game; nothing that
followed it touched a rule. **The Custodians remain the standing imbalance at 70 of 80, the
Prospectors and Archivists win one or none, and every Collapse is in the Custodian seating.** This
version does not address any of it, deliberately.

---

## 14. The kit

The **Windows kit** only, at the designer's word, as in 0.07.1: `dying-earth.exe` built with a
statically linked CRT, `assets/` (including the fourteen flags and the five new icons), and the
playtest note as `README.txt`, zipped into `dist/dying-earth-0.07.2-playtest.zip`. `dist/` is
gitignored, so the kit is an artifact on the machine and not a commit.

---

## 15. What is left open

Carried onto the map as fog, none of it decided here:

- **Whether a Colony's card also loses its Influence controls.** The Region card lost them because the
  Command Cluster spends on the selected place; the same is true of a selected Colony.
- **Saudi Arabia pays no Ducats**: GDP 2 × Industry 2 / 10 rounds to 0. The sharing rule produced it;
  one GDP point would fix it, and whether it should is the designer's.
- **The spreading rule seats the third computer player in Saudi Arabia**, the peninsula being untouched
  at Industry 2 and more populous than Australia. The rule working as written on a new graph.
- **India and the United States are the same green** (a CIELAB distance of 11), on opposite sides of
  the globe. The first thing to move if the palette is revisited.
- **The start globe runs under the side panel**, so a label at the limb is half hidden. A layout
  question for another version.
- **Japan loses coastal slots early**, having the most exposed coast on the map. Noted so nobody
  mistakes it for a fault.
- **Whether Defence should defend early**, **Research constrains nothing**, **the Custodians at 70 of
  80 with the Archivists at 1**, **map crowding at Antarctica**, and **the 0.07.0 sweep discrepancy**:
  all as version 0.07.1 left them, and the first four things a balance version should take.
