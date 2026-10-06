# Dying Earth — version 0.09.9, the settlement version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.9](https://github.com/whaleyjoshua2/Dying-Earth/issues/488), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-05-version-0.09.9/`](../dev-diary/2026-10-05-version-0.09.9/).

**What the version is.** Version 0.09.8 with the designer's list. *(Summary and win column written
when the version closes.)*

## 1. Four Colonists to found

The authority is [ticket #489](https://github.com/whaleyjoshua2/Dying-Earth/issues/489).

**Every new Colony or station opens with four Colonists**, however it is made. The figure is
`found_with = 4` under `[emigrants]` in `factions.toml`. Materials prices are unchanged.

| How it is made | Where the four come from | Before |
|---|---|---|
| A Colony Ship unloads onto a free ground slot, or into a free ring off Earth or at Earth L4 or L5 | the Ship: the Unload must put down at least 4 | at least 1 |
| Pioneers sent to Antarctica by sea, into a free slot | the Region they sail from: at least 4 sent | at least 1 |
| A station over Earth, built from a Launch Site | 4 Pioneers waiting in a Region of the builder's with a working Launch Site | nobody |
| A station over another Body, built from a ground Colony there | 4 Colonists from that Colony | nobody |
| A ground Colony built from a station over that Body | 4 Colonists from that station | nobody |

- **A Colony Ship lands exactly four**, as it already could not land more: a new Colony's Core
  holds four and the rest stay aboard.
- **A Colony or station that builds one keeps enough people.** After the four leave it must
  still hold at least as many Colonists as its Modules in slots (Core and Archive not counted,
  a Module mothballed or building counted, as on the card's "Modules {used}"), and never fewer
  than four. So a source needs that number plus four.
- **Which source.** A station order and a station-built Colony name the place their four come from.
  The game chooses for the player: the qualifying Region or Colony with the most to spare.
- **People ordered once are ordered once.** A source's spare counts what the turn's other
  orders already take from it: Loads, lifts, sends by sea or down, and other builds; and a Load or
  a Send Down counts the four a build has claimed. A founding settles the people who really left,
  never more.
- **When they leave.** Builders leave their source when the turn is ended and arrive as the
  place is built at Resolution, as Pioneers sent by sea do. If the place is not built (another
  Faction took the slot), they go back where they came from.
- **A lift from a Launch Site** carrying the four is the station build itself: no second launch,
  no second emission.
- **The hover.** Every founding button's price reads "4 Colonists" beside the Materials. A
  refusal says what is short: "needs 4 Colonists", or "needs 4 Colonists to spare".
- **The computer seats** found only with four ready, by the same rule: a Colony Ship sets down
  four or does not found, and a Colony or station builds only with four to spare. A Colony Ship
  of theirs holding fewer than four sails only to a place of their own with room, or stays to
  load. **A seat with no station over Earth recruits the four a station takes**: their recruiting
  wanted room to put people, and with no station there was none, so a seat without one never
  recruited and never built one.
- **The Arkwrights start with 4 Pioneers waiting** in their start Region, where it was 2, so their
  first station can still be ordered on turn 1. The gift takes no population, as before.
- **Not changed:** unloading or sending into a Colony that already stands, the Arkwrights' Colony
  Ship carrying 8, and the stations every other Faction starts with.
- **Saves:** the source rides on the order and on the pending build; `SAVE_VERSION` stays at 9.

**Measured** (80 games, the standing cell,
[`after-489.txt`](../dev-diary/2026-10-05-version-0.09.9/sweeps/after-489.txt)):

| | 0.09.8 | after this | the ideal |
|---|---|---|---|
| Custodians | 9 | 0 | 15 |
| Prospectors | 27 | 24 | 15 |
| Arkwrights | 18 | 33 | 15 |
| Archivists | 7 | 6 | 15 |
| collapses | 17 | 16 | 20 |
| Arkwrights meet their Opening Objective | 29 of 80 | 55 of 80 | |
| stations off Earth at the end | 94 | 36 | |

The column moves far past noise. Stations off Earth fall to a third: a Colony or station must
hold eight before it builds one. The Arkwrights, whose Colony Ship carries eight and who start
with their four, reach the Moon first far more often, and their Opening Objective, a Colony on the
Moon, is met in 55 games where it was 29. The Custodians win none: traced on one game they still
had their twelve off Earth by turn 8, but never held Stabilization; the game diverged through Tech
picks and cards, not through any one rule, so their loss is **not traced** past that.
The dev diary's README has the trace.

With the Arkwrights starting at two Pioneers and the computer's recruiting fixed, the column read
5 / 23 / 24 / 5 and 23 collapses; at two before the fix, 8 / 45 / 1 / 2 and 24.

## 2. Colony tiers: Outpost, Settlement, Colony

The authority is [ticket #490](https://github.com/whaleyjoshua2/Dying-Earth/issues/490).

- **Every Colony and station stands at a tier**, and starts at the first. The tiers are a list in
  `modules.toml` (`[[tiers]]`), so a fourth is one more entry:

| Tier | Module slots at most | Colonists to reach it | Price |
|---|---|---|---|
| Outpost | 6 | (every place starts here) | |
| Settlement | 12 | 12 | 30 Materials, 4 Widgets |
| Colony | 18 | 18 | 50 Materials, 6 Widgets |

- **Module slots are the lower of the place's Colonists and its tier's cap.** The Core and the
  Archive take none, as before.
- **The place stays a Colony** in every rule and text, the Diaspora's included; Colony is also the
  top tier's name. The card's title carries the tier beside the name: "ISS over Earth Outpost".
- **Stations** climb the same tiers.
- **The upgrade** is a tile after the Module boxes, dashed in the Raise Industry Level tile's red,
  reading "Upgrade" with the next tier's name under it. A click orders it; it is built through the
  place's Widgets queue like a Module, one at a time, and lands when the build completes. The
  same price for every Faction. Greyed, its hover leads with what is short: "needs 12 Colonists".
  Its hover otherwise: "Upgrade to a Settlement: 30 Materials and 4 Widgets. Up to 12 Modules."
- **A place never falls back a tier.** Losing people already costs it slots.
- **A place past its cap** loses nothing; it builds no more until upgraded. The Modules heading is
  cut to "Modules 3 of 6"; full, it says "Full until more Colonists live here", or at the cap
  "Full: upgrade for more room", or at the top "Full". Its hover: "One a Colonist, at most 6 as an
  Outpost. Mothballed keeps a slot, building reserves one; the Core and the Archive take none."
- **A Module refused for want of room** says why: "full until more Colonists live here" below the
  tier's cap, "full: upgrade to a Settlement for more" at it, "full: a Colony holds 18" at the top.
- **The computer seats** order the upgrade for a place whose slots are all taken at its tier's cap
  and whose people are enough for the next tier, at a Producer's weight (`raise_tier` in
  `ai.toml`).
- **A rival's upgrade** reaches the Report where the fog allows: "upgraded Tycho on the Moon". The
  Report's word ceiling rises by its two words, to 1,783.
- **Saves:** a place with no tier recorded reads as an Outpost; `SAVE_VERSION` stays at 9.

**Measured** (80 games, the standing cell,
[`after-490.txt`](../dev-diary/2026-10-05-version-0.09.9/sweeps/after-490.txt)):

| | after §1 | after this | the ideal |
|---|---|---|---|
| Custodians | 0 | 3 | 15 |
| Prospectors | 24 | 26 | 15 |
| Arkwrights | 33 | 26 | 15 |
| Archivists | 6 | 4 | 15 |
| collapses | 16 | 20 | 20 |

At the end of the 80 games 1,032 Colonies and stations stand: 929 Outposts, 81 Settlements, 22
Colonies (the sweep's new line). The Arkwrights' fall of seven is past noise and **not traced**.

## 3. The Prospectors' Bank paying interest off Earth

The authority is [ticket #491](https://github.com/whaleyjoshua2/Dying-Earth/issues/491).

- **The Exchange**, the Prospectors' own Module in the Trade Post's place, pays an Investment Bank's
  interest on top of its Ducats: 1% of the Venture Capital Fund into the Fund, to the tenth.
- **One share a place**: each Colony or station with a working Exchange counts once, as each Region
  with a working Investment Bank does, however many stand there. The floor of 1 is still one floor
  for the whole Faction.
- **A captured Exchange** pays its captor 1% of that turn's Ducat income, at least 1, as a captured
  Investment Bank does.
- **A place under Blockade** makes nothing, so its Exchange pays no interest.
- **The computer Prospectors** weigh an Exchange by the Fund, as they weigh an Investment Bank: the
  fuller the Fund, the more it is wanted.
- **The Opening Objective** is unchanged: its three Investment Banks are Regions' alone.
- **Its hover:** "+1 Ducat over a Trade Post, and a Bank's interest". The income line names the
  payers: "2 Investment Bank or Exchange (interest banked)". The build hover runs to **seven** lines,
  as it did before this ticket: the four lines of arithmetic under it are what overflow, and cutting
  them is left to the designer.

**Measured** (80 games,
[`after-491.txt`](../dev-diary/2026-10-05-version-0.09.9/sweeps/after-491.txt)): 3 / 25 / 26 / 4 and 21
collapses, from 3 / 26 / 26 / 4 and 20: within noise. The Prospectors' median Fund at the end rises
in every seating: 2,616 to 2,637, 2,782 to 3,090, 1,357 to 1,545, 1,550 to 1,661.

## 4. East Africa and the Horn to the South Africa Region

The authority is [ticket #492](https://github.com/whaleyjoshua2/Dying-Earth/issues/492), its decision
and the amendment the designer made after the first build.

- **To South Africa's Region from Nigeria's:** Kenya, Tanzania, Uganda, Rwanda, Burundi, Ethiopia,
  Somalia (Somaliland with it) and Djibouti. Zambia was South Africa's already, since version 0.09.8.
- **To Nigeria's Region from South Africa's:** Angola.
- **To Egypt's Region from Nigeria's:** Eritrea.
- **People** move exactly, at 2023 figures (UN): Kenya 55.1, Tanzania 67.4, Uganda 48.6, Rwanda 14.1,
  Burundi 13.2, Ethiopia 126.5, Somalia 18.1, Djibouti 1.1, Angola 36.7, Eritrea 3.7 million.
  South Africa 517, from 210; Nigeria 619, from 930; Egypt 264, from 260. The world holds 7,860 still.
- **Nothing else on the cards moves**: GDP (Nigeria's and South Africa's 1 already the floor),
  Influence, emissions, Size, coast, Industry Level, Lean and start buildings.
- **Who touches whom.** South Africa: Nigeria, Egypt, and Saudi Arabia across the Red Sea. Egypt
  gains South Africa. Nigeria loses Saudi Arabia, the Horn being gone; Saudi Arabia faces South
  Africa instead.
- **Where the computer seats start.** The spreading rule takes no Region touching one already taken,
  and Saudi Arabia now touches South Africa, the third pick: **the fourth computer start is
  Australia**, where it was Saudi Arabia.
- **The map** is repainted from Natural Earth's countries, the same file as version 0.09.8 (a repaint
  before the change matched the old map to the pixel). It differs in these moves only: 9,630
  pixels from Nigeria's Region to South Africa's, 3,421 from South Africa's to Nigeria's, 392 from
  Nigeria's to Egypt's.
- **Saves:** `SAVE_VERSION` stays at 9.

**Measured** (80 games,
[`after-492.txt`](../dev-diary/2026-10-05-version-0.09.9/sweeps/after-492.txt)): 2 / 26 / 28 / 2 and 20
collapses, from 3 / 25 / 26 / 4 and 21: within noise. (The first build, East Africa alone, read
3 / 27 / 23 / 3 and 21.)

## 5. Stations drawn short in a Body's In orbit list

The authority is [ticket #493](https://github.com/whaleyjoshua2/Dying-Earth/issues/493).

- **On every Body's card**, each station in the In orbit list is one row: the station glyph
  (off-white), its name in its holder's colour, and its Colonists with the people glyph, as
  `[station] ISS 2 [people]`. A click selects it, as before.
- **Nobody's** station has its name in grey; an **occupied** one in the occupier's colour.
- **A station under Blockade** adds "blockaded" in red.
- **The owner's name and the modules leave the row.** The modules are its hover: "Shipyard, Solar
  Array", or "bare" for a Core alone.
- No rule moves.
