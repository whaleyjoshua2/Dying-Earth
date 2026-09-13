# Dying Earth — version 0.07.4, the reactions version: hovers back on the boxes, orbits that turn with the globe, a slower start globe, an Emissions history, the build list folded into the boxes, Venus clear of Earth, and Coastal Engineering put in its place

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.07.4](https://github.com/whaleyjoshua2/Dying-Earth/issues/149), and the
pictures that decided it are in
[`docs/dev-diary/2026-09-13-version-0.07.4/`](../dev-diary/2026-09-13-version-0.07.4/).

**What the version is.** Version 0.07.3 gave the places depth. This one is the designer's seven
reactions to it, plus one that grew out of them, each built as its ticket recommended and then
corrected on the designer's word off a picture of the built thing. **No rule, price or yield
changes**: the sweep at the end reproduces the 0.07.3 baseline to the game, and the baseline stands.

---

## 1. The building hovers return

*Ticket [#150](https://github.com/whaleyjoshua2/Dying-Earth/issues/150).*

Every slot box on the Nation card and every tile in the Hab View says on hover what its row said
before 0.07.3 moved the row into the click strip: the kind, its figures, and the upkeep, Emissions
and coastal rules. A Hab View tile reads its figures with the **Module rules**, written new, since
the Module rows never had a hover. A free box says `Free coastal slot: click it to build here` with
the sea's note; a building box its kind and the turn it is ready; a flooded box `Factory, lost to
the sea: a Sea Level threshold took this coastal slot` and what a Sea Wall does; a free tile `Room
for another Module` with the cap rule; the Archive its fund and its own rules. The click strip is
unchanged. **No written description per building**: the designer found figures and rules enough.

The Facility row's figures and rules are two helpers (`facility_figures`, `facility_rules`) the box
shares with the row, and the Hab View's strip line one (`module_line`) its tiles share, so hover and
row cannot drift apart again.

---

## 2. Orbits on the Surface Map: their own planes, turning with the globe, the stations travelling

*Ticket [#151](https://github.com/whaleyjoshua2/Dying-Earth/issues/151).*

The rings round a Body on its Surface Map go back into **the globe's own frame** (version 0.07.3 had
pinned them to the camera after a first attempt came out edge-on), **each on a plane of its own**:
leaning 32 to 61 degrees from the equator with a heading of its own per slot, so they turn with a
drag and cross rather than stack. A ring is edge-on for a moment at two headings per turn of the
globe, one ring at a time. They stand **1.08 to 1.24 of the globe's radius** out (the designer:
*"just slightly tighter"* than the first try's 1.12 to 1.30). **An empty slot's ring is a solid line
in the same grey**, no longer dashed. **A station travels its ring**: one revolution in ninety
seconds for the first slot and eight seconds longer for each after, so they drift apart; its name
rides beneath it and a blockading warship beside it. The click circle is sixteen pixels and **a
click lands where the mouse was pressed**, so a moving station is caught by the click that began on
it. The clock is stopped in `shot:` mode. The Solar System Map's rings are untouched.

---

## 3. The start globe turns slower

*Ticket [#152](https://github.com/whaleyjoshua2/Dying-Earth/issues/152).*

**Once every 75 seconds**, a third of the speed it opened at (once every 25 seconds); the designer
chose the third over the quarter and the tenth offered. **A click on a Region stops the spin for
good**, as a drag does, the globe taking the angle the spin had reached so it does not jump. No
pause on pointing. The number is a named constant in the code, `START_GLOBE_PERIOD_SECS`.

---

## 4. The Emissions history

*Ticket [#153](https://github.com/whaleyjoshua2/Dying-Earth/issues/153).*

The engine keeps **one record per Climate phase** (`EmissionsRecord`: the breakdown by source, the
CO2 Stock, the Temperature, and the indices of the Breaks that fired), pushed at the end of the
phase and saved with the game. A save from before this version cannot be loaded in any case (saves
are never migrated across versions); within the version the field defaults to empty.

Hovering the top bar's Emissions figure draws **three lines against a plain zero line**: what the
world emitted, what the Natural Sink and the Scrubbers removed, and the net between them, which is
the bar's figure and the CO2 Stock's change each turn; a red tick on the turn axis where a Break
fired; the last net figure at the line's end. **The same chart stands on the Climate Panel** under
the by-source list, at the panel's width. Hand-painted with egui's painter as the Temperature bar is;
no charting crate. A new `rule_tip_ui` lets the `tip:` aid photograph a hover that draws, and a new
`bar_resource_with` puts one tooltip on the glyph and label together so the chart is never painted
twice.

---

## 5. The Temperature history

*Ticket [#158](https://github.com/whaleyjoshua2/Dying-Earth/issues/158), graduated from the map's
fog when the designer said yes to it on ticket #153.*

Hovering the top bar's Temperature figure, hoverless until now, draws the Temperature turn by turn
from the same record, **on the data's own range** (the designer's choice over base-to-Collapse,
for the detail): a margin above and below, never narrower than half a degree, the range's ends
written small at the left. The Breaks' Temperatures that fall in the range are faint red lines
across it and the Collapse line is drawn, labelled, when it does; the Breaks fired are ticked red
on the turn axis; the last figure stands at the line's end. **The hover only**; the Climate Panel's
bar keeps the thresholds. The heading-to figure is a projection and is not drawn.

---

## 6. The build list folded into the boxes

*Ticket [#154](https://github.com/whaleyjoshua2/Dying-Earth/issues/154).*

The per-kind `Build Factory 20 · or 40` list at the bottom of the Nation card is gone: it was the
very list a free box's click strip draws, and **a Facility that takes a slot is built by clicking a
free box and nowhere else**. **The Scrubber and the Sea Wall stand under the boxes with no
heading**, inside the Facilities section: each a row when it stands (figures, Mothball,
Decommission), a line while it builds, and a build button pair when it may be built -- the
Scrubber's for the Custodians with `0 of 7 this state may hold` beside it, the Sea Wall's once
Coastal Engineering is in and while none stands or builds, its rule on hover. The Sea Wall leaves
the free box's strip. **The bottom header is `Orders (hover a button for what it does)`**, with
Leapfrog and the Strip Permit staying under it beside Raise Industry Level, Build Army, the
Emigrant orders, Relief and Resettle, and the Army orders. Two tidyings: the slot count is said once,
on the Facilities header (`Education Level 1.1` stands alone where `Build slots: 2 used of 7` was);
a no-slot row no longer says `(inland)` or, mothballed, `keeping its slot`.

---

## 7. The Solar System Map rescaled

*Ticket [#155](https://github.com/whaleyjoshua2/Dying-Earth/issues/155).*

Measured first: Venus's ring at 2.45 and Earth's at 3.4 were 0.95 apart, which the camera's
forty-degree tilt squashed to 0.61 on screen, less than the two discs' radii together (0.82), so the
discs overlapped by about fourteen pixels at every conjunction, and the station rings drawn in 0.07.3
crossed whenever the two were within twenty-five degrees. **Venus's ring is in to 1.7 and Earth's out
to 3.8**, Mars staying at 6.0, the three read from **one table** (`geo::solar_ring`) by the Bodies'
places and the drawn rings alike. The Moon keeps its one-unit distance from Earth. **Labels by
Body**: a planet's stands above its disc, Venus's and the satellites' hang below, so the two inner
planets' words never meet and a moon's never lie over its planet's; the Orbital Control line follows
its label's side. The station rings, the hover zones and the zoom range are unchanged. The game's
closest conjunction is turn 1, three degrees apart; the next is turn 11, seven.

---

## 8. Coastal Engineering put in its place on the Tech Tree

*Ticket [#156](https://github.com/whaleyjoshua2/Dying-Earth/issues/156).*

Read as a question of place, which the designer confirmed; the cost stays 12 as decided on 0.07.3.
**Two Techs on one rung of one branch stack in a taller branch row on every rung but the last**,
where nothing leaves them and they sit side by side (Planetary Stewardship beside The Upload). So
Coastal Engineering sits beneath Efficient Grids, every column is one box wide, every first-rung box
lines up, Efficient Grids' lines leave its right edge instead of running beneath Coastal Engineering,
and the tree is 768 pixels wide, inside the Moment popup's 780 with no change there. A lone box, on
any rung, stands at its band's middle. The tree is six rows tall and fits the window.

---

## 9. Builder's calls, for the designer to veto

- The orbit rings' per-slot angles (a lean of 0.55 + 0.13 per slot radians, a heading of 1.3 per
  slot) and the per-slot periods (90 + 8 per slot seconds) are chosen for the picture; none is in
  the data, none means anything to the rules.
- The Emissions history's colours: emitted in the Emissions glyph's orange, removed in green, the
  net in off-white; a Break's tick in the Temperature bar's red.
- The Temperature history's range margin: fifteen per cent of the span, never under a quarter of a
  degree, so the first turns do not read as a cliff.
- The Sea Wall's build hover carries its rule (no slot, one to a state, takes the next threshold
  whole and is destroyed doing it); the old card note that said the same is gone.
- The start globe's period is a code constant: `assets/data` has no view-tuning table and one
  number does not earn one.

---

## 10. Building aids added or changed this version

- `tip:<word>` now reaches a hover that draws (the two histories) through `rule_tip_ui`.
- The orbit stations' clock is stopped in `shot:` mode, so every picture is the same twice.
- No new aids; `select:EastAsia walls:1`, `hab:1 turns:12`, `look:<lon>,<lat> panel:0`,
  `climate:top`, `window:1280x1000`, `tech:1` and `menus:1 moment:tech` took this version's pictures.

---

## 11. What was measured, and the baseline stands

- **The suite is 254 tests**, clippy clean with `-D warnings`; none changed, since no rule did.
- **The sweep reproduces the 0.07.3 baseline to the game**, twenty seeds in each of four seatings on
  `simulate:<seed> --player=<faction>`:

| seat 0 | wins | Collapses | draws | tree completes |
|---|---|---|---|---|
| Custodians | Archivists 6, Custodians 3 | **11** | 0 | 20 of 20 |
| Prospectors | Custodians 16, Archivists 2, Prospectors 1 | 1 | 0 | 20 of 20 |
| Arkwrights | Archivists 8, Arkwrights 6, Custodians 4 | 2 | 0 | 19 of 20 |
| Archivists | Custodians 13, Archivists 5 | 0 | 2 | 9 of 20 |
| **totals** | **Custodians 36**, Archivists 21, Arkwrights 6, Prospectors 1 | **14** | **2** | **68 of 80** |

  Every cell is the 0.07.3 table's. **The baseline stands**; the balance version that follows reads
  against it.
- **A whole game from seat 0** was played headlessly through the `play` example; the result is in
  the dev diary's last section.

---

## 12. The kit

The **Windows kit** only: `dying-earth.exe` built with a statically linked CRT, `assets/`, and the
playtest note as `README.txt`, zipped into `dist/dying-earth-0.07.4-playtest.zip`. `dist/` is
gitignored, so the kit is an artifact on the machine and not a commit.

---

## 13. What is left open

Carried onto the map as fog or ruled out of scope, none of it decided here:

- **The balance version**, against the standing baseline.
- **Whether the Solar System Map's station rings should move** as the Surface Map's now do.
- **The Region glyph on the map labels**, kept.
- **What a Colony's card does about its slots**, **what population does once counted in real
  numbers**, **whether a Colony's card loses its Influence controls**, **India and the United States
  the same green**, **the start globe under the side panel**, **French Guiana with the European
  Union**: as 0.07.3 left them.
- **A price change for Coastal Engineering**: out of scope on this map; 12 stands.
