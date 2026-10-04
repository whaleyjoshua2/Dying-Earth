# Dying Earth — version 0.09.8, the Moon race version

Amendments to the specification, one section per decision. Each section names the ticket whose
resolution comment is the authority; where this document and a ticket disagree, the ticket is right.
The map is [Map: version 0.09.8](https://github.com/whaleyjoshua2/Dying-Earth/issues/473), and the
pictures and batches that decided it are in
[`docs/dev-diary/2026-10-04-version-0.09.8/`](../dev-diary/2026-10-04-version-0.09.8/).

**What the version is.** Version 0.09.7 with the designer's list. The summary and the win column
are written when the version closes.

## 1. Card sub-headers: larger, white, a glyph leading each

The authority is [ticket #474](https://github.com/whaleyjoshua2/Dying-Earth/issues/474).

- **Every sub-header on the Region card and the Colony card is drawn one way**: its glyph, then its
  words in white at 14.5 points, where the body text is 12.5. The glyph is 17 pixels and keeps its
  own fill.
- **The Region card's headings:** Influence, Pioneers, Policies, Facilities, Armies, Orders.
- **The Colony card's headings** (a station's card is the same card): Modules, Armies, The Archive,
  Orders, Ships, Influence. **Modules and Influence are headings now**, where each was a plain line.
  - The warning that rode on the Modules line when a place is full stands under the heading.
  - The "Influence:" label beside the amount is gone; the heading says it, and the label's hover is
    on the amount.
- **Influence's heading is white**, where it was the body's grey.
- **The Armies heading takes the shared size.** Its list keeps the tenth it has had since 0.08.7.
- **A rule is drawn below the Policies block**, so the Widgets line and its queue read with the
  Facilities they build.
- **A rule is drawn above the Armies block** on both cards.
- **The glyphs:**

  | Heading | Glyph |
  |---|---|
  | Influence | Influence |
  | Pioneers | the Colony Ship |
  | Policies | a gavel on its block, drawn for the game |
  | Facilities | the Factory's |
  | Modules | the Habitat's |
  | Armies | the Battle mark |
  | Orders | a list of three rows, drawn for the game |
  | Ships | the Warship |
  | The Archive | the Archive Module's |

- No rule moves, and nothing a computer seat reads.

**Pictures:** [`ticket-474/`](../dev-diary/2026-10-04-version-0.09.8/ticket-474/).

## 2. The population and GDP lines on the cards

The authority is [ticket #475](https://github.com/whaleyjoshua2/Dying-Earth/issues/475).

- **The Region card's population line** reads "Region population 385.6 (386M)" behind the people
  glyph, and no more. Its hover, the growth rule, is unchanged.
- **The Region card's GDP line** reads "GDP 23: Industry Level 3, leans Fuel", behind the Ducats glyph.
- **What the GDP line said before is its hover**, three lines: what the Region pays its controller
  in Ducats a turn; the rule, GDP x Industry Level / 5, never below 1; and what the player's Bank
  would add there. A rival's pay the player cannot see stays hidden.
- **The Colony card's people line** reads "14/16 (14.2M)" behind the people glyph: Colonists over
  room, then the people as a real number to a tenth of a million.
  - The bracket counts the part-grown next Colonist, so it moves each turn the place grows.
  - The figure before the slash stays whole; it is what room and Module slots count.
  - "14/16" keeps its amber at three-quarters full and its red when full; the bracket is plain.
- No rule moves, and nothing a computer seat reads.

**Pictures:** [`ticket-475/`](../dev-diary/2026-10-04-version-0.09.8/ticket-475/).

## 3. Raise Industry Level as a tile

The authority is [ticket #476](https://github.com/whaleyjoshua2/Dying-Earth/issues/476).

- **Raise Industry Level is a tile among the Facility boxes**, last of all, after the final inland
  box: dashed like a free slot and reading "Raise Industry Level". The button in the Policies block
  and the note under it are gone.
- **One click orders the raise.** The rule is unchanged: it is paid at End Turn, built through the
  Region's Widgets queue, and one may be under way in a Region at a time.
- **While it is ordered or under way the tile is drawn like any building being built**: hatched,
  "ordered" and "0 of 4" before End Turn, "building" and the Widgets done after, with "Industry
  Level" beneath it. A right-click takes back an order placed this turn.
- **When it completes** the Region has one more inland slot, drawn as a free "Click to Build" box,
  and a new Raise tile stands after it.
- **Its hover** gives the price, when it is ready, and "Adds an inland slot." Where the order would
  be refused the tile is greyed and the reason leads the same hover.
- **Its words and dashes are a muted red**, so it is told from a free slot at a glance; dimmed
  where the order would be refused.
- **Only on a Region the player directs.** A rival's or a neutral Region shows no Raise tile.
- The Widgets queue line for a raise stays. No rule moves, and nothing a computer seat reads.

**Pictures:** [`ticket-476/`](../dev-diary/2026-10-04-version-0.09.8/ticket-476/).

## 4. The tutorial, 14% fewer words

The authority is [ticket #477](https://github.com/whaleyjoshua2/Dying-Earth/issues/477).

- **The tutorial's seven notes hold 398 words, where they held 462**, counted as the words of each
  note's heading, text and quieter line. The designer asked for 10% off, which is 415.
- **Seven passages were cut**, on turns 1, 2, 3 and 5; the ticket holds each before and after.
  Turns 4, 6 and 7 are as they were, the designer's closing sentence on turn 7 among them.
- No step the player is asked to take was dropped. Two statements went with the words: that
  recruiting takes people from the Region (turn 3), and that the Scrubber is the only thing that
  visibly moves the climate (turn 5). Turn 5 says "Region" where it said "state".
- **No test holds a ceiling**, at the designer's word: the notes are to stay free to rewrite.

**Pictures:** [`ticket-477/`](../dev-diary/2026-10-04-version-0.09.8/ticket-477/).

## 5. The Prospectors' Opening Objective: 50 Ducats, three turns running

The authority is [ticket #478](https://github.com/whaleyjoshua2/Dying-Earth/issues/478).

- **The reward is 50 Ducats** into the Venture Capital Fund, where it was 75.
- **The objective wants three Incomes running** with a working Investment Bank in each of three
  different Regions the Prospectors control. Working is online and not mothballed, as before; two
  Banks in one Region count once, as before. It need not be the same three Regions each turn.
- **An Income where fewer than three Regions have a working Bank starts the count again.**
- It is still met once, paid once, and has no deadline.
- **Its words:** "Keep a working Investment Bank in three places for three turns" and "50 Ducats
  into the Venture Capital Fund".
- **The Journal shows the count** once it has started: "Not yet met: 2 of 3 turns."
- The other three Factions' objectives are unchanged: each is met the Income it is true.
- **The computer Prospectors** are taught nothing new: they lean toward building Banks until the
  objective is met, as before.
- **Saves:** the count is a new field that reads nought from an older save; `SAVE_VERSION` stays.

**Measured** (80 games, the standing cell,
[`after-478.txt`](../dev-diary/2026-10-04-version-0.09.8/sweeps/after-478.txt)):

| | 0.09.7 | after this |
|---|---|---|
| Custodians | 8 | 8 |
| Prospectors | 21 | 19 |
| Arkwrights | 18 | 20 |
| Archivists | 5 | 4 |
| collapses | 28 | 29 |
| Prospectors meet their objective | 58 of 80 | 51 of 80 |

Every move in the win column is within noise. The Prospectors meet the objective in seven fewer
games and a turn or two later.

## 6. Raise Industry Level at a rising price

The authority is [ticket #479](https://github.com/whaleyjoshua2/Dying-Earth/issues/479).

- **Each raise costs more than the last**: 30 Materials and 4 Widgets, then 40 and 5, then 50 and
  6, +10 Materials and +1 Widget every time, with no ceiling. The figures are in
  `facilities.toml` (`materials_step`, `materials_cheap_step`, `widgets_step`).
- **What is counted is the Region**: how far it stands above the Industry Level on its card,
  whoever raised it. Neutral Development's raises count. A level lost to a nuke comes off the price
  again. A Region at or below its card's level pays the first price.
- **The Prospectors pay half the Materials at every step**: 15, 20, 25. Their 15% off Widgets
  applies to the risen figure, rounded down: 3, 4, 5.
- **Nothing is added to the card's words.** The Raise tile's hover reads the Region's next price.
- A cancelled raise refunds at the price the Region charges at that moment, which is the price
  paid, since the level has not yet risen.
- **The computer seats** are taught nothing: they sum what an order costs from the engine's own
  price before weighing it, so they read the risen figure.

**Measured** (80 games, the standing cell,
[`after-479.txt`](../dev-diary/2026-10-04-version-0.09.8/sweeps/after-479.txt)):

| | before this | after this |
|---|---|---|
| Custodians | 8 | 8 |
| Prospectors | 19 | 19 |
| Arkwrights | 20 | 17 |
| Archivists | 4 | 5 |
| collapses | 29 | 31 |
| raises the Factions completed | 1,895 | 1,627 |

The win column is within noise. The computer seats raise about a seventh less often.

## 7. Earth L4 and Earth L5, two far orbits

The authority is [ticket #480](https://github.com/whaleyjoshua2/Dying-Earth/issues/480).

- **Earth has seven Orbital Slots**, where it had five. The sixth and seventh are **far orbits**,
  named **Earth L4** and **Earth L5**. Both begin empty.
- **A far orbit is reached only by Ship.**
  - A move to or from one costs **8 Fuel** (16 since section 8) from the tank and a turn, from any other orbit of Earth,
    the other far orbit included. Between Earth's other orbits a move is still 1 Fuel.
  - A leg between Bodies pays the 8 on top of its crossing for a far orbit it leaves, and again for
    a far orbit it names to arrive in.
- **A station there is founded, not built**, as at Venus: a Colony Ship with Colonists, sitting in
  that orbit, unloads into it; the station opens with a Core and those Colonists, for no Materials.
  A Launch Site builds none, and the refusal says so.
- **No lift reaches it.** A Region's Launch Site offers no lift to a far station, and the station's
  card says "No lift reaches here: its people come by Colony Ship." Its people arrive by Colony
  Ship unloading in its orbit; the Ship may be loaded from a Region or another station as before.
- **Its name stands alone:** "Earth L4", where another station reads "ISS over Earth". Its card
  reads "Founded", not "Built".
- **Where it is drawn.** Not on Earth's Surface Map at all, at the designer's word: no ring and no
  marker. On the Solar System Map each stands at its own point on Earth's path round the Sun, a
  sixth of the way ahead of Earth (L4) and behind (L5), named; an open circle while empty, its
  station's glyph in the holder's colour once founded, which is clicked to open its card.
- **Earth's planet card** lists each free far orbit as "Earth L4: free. Founded by a Colony Ship in
  that orbit; 8 Fuel to reach." in place of a Build button.
- **The computer seats** build stations only in the five. Once every one of those is taken, a
  loaded Colony Ship of theirs at Earth with 8 Fuel goes out to a free far orbit and founds there,
  at half the weight of founding a Colony, as Antarctica is weighed.
- **Saves:** slots are kept by number and the two are added at the end; `SAVE_VERSION` stays.
- Not changed: whether a stranded Ship counts a depot in a far orbit as in reach still reads the
  1 Fuel figure.

**Measured** (80 games, the standing cell,
[`after-480.txt`](../dev-diary/2026-10-04-version-0.09.8/sweeps/after-480.txt)): 8 / 19 / 17 / 3
and 33 collapses, from 8 / 19 / 17 / 5 and 31. Within noise. **The computer seats founded one far
station in the 80 games.**

**Pictures:** [`ticket-480/`](../dev-diary/2026-10-04-version-0.09.8/ticket-480/).

## 8. Fuel scaled to real delta-v

The authority is [ticket #485](https://github.com/whaleyjoshua2/Dying-Earth/issues/485). The figures
and their sources are in [`docs/research/delta-v.md`](../research/delta-v.md).

- **A leg's Fuel is its real delta-v, in km/s, times one scale: 4 Fuel a km/s**, to a tenth. Both
  figures are in `bodies.toml`: each leg's delta-v, and `fuel_per_delta_v`. The designer first took
  4.5 (Mars at about 20), had 4.0 swept beside it, and chose 4.0.
- **Ships brake on air where there is air, as missions have flown it**: a burn into a loose orbit,
  then months of passes through the upper air. That is Mars, Venus and, a little, Phobos. The
  Moon, Deimos and the far orbits have none.

  | Leg | Delta-v (km/s) | Fuel | Before |
  |---|---|---|---|
  | Earth to the Moon | 4.0 | 16.0 | 6 |
  | Earth to Venus | 4.4 | 17.6 | 16 |
  | Earth to Mars | 4.6 | 18.4 | 20 |
  | Earth to Phobos | 4.9 | 19.6 | 24 |
  | Earth to Deimos | 5.0 | 20.0 | 24 |
  | to or from Earth L4 or L5 | 4.0 | 16.0 | 8 |
  | Mars to Phobos | 1.2 | 4.8 | 2 |
  | Mars to Deimos | 1.7 | 6.8 | 2 |
  | Phobos to Deimos | 0.75 | 3.0 | 1 |

- **A leg costs the same both ways.** Earth's air would make the way home cheaper; that is not
  priced.
- **Not scaled:** the 1 Fuel between two ordinary orbits of one Body, and a lift, which costs no
  Fuel.
- **Unchanged:** the launch windows, which still raise a crossing's price the further it is from
  its window, on the new figures; the turns a leg takes; the tanks, 35 for a Colony Ship and 30 for
  the rest; the Faction's, Efficient Transit's and a Mass Driver's cuts, which apply after as before.
  A Mass Driver's result is rounded to the tenth, now that a leg may carry a decimal.
- **What follows from it:**
  - A full tank reaches any one Body from Earth at its window and does not come home without
    refuelling. The Moon is no longer the cheap first hop.
  - A Mass Driver's flat 4 Fuel is worth less against the new prices: 12 from the Moon to Earth
    where it left 2.
  - Off its window a crossing can pass a full tank: Mars reads 32.3 on turn 4, Venus 40.7.
- **The computer seats** read every leg's price from the engine and were taught nothing.

**Measured** (80 games, the standing cell,
[`after-485.txt`](../dev-diary/2026-10-04-version-0.09.8/sweeps/after-485.txt)):

| | before this | at 4.5 | at 4.0, as built |
|---|---|---|---|
| Custodians | 8 | 8 | 7 |
| Prospectors | 19 | 21 | 20 |
| Arkwrights | 17 | 17 | 17 |
| Archivists | 3 | 1 | 4 |
| collapses | 33 | 33 | 32 |
| Ships stranded at the end | 3 | 13 | 15 |

The win column is within noise at both scales. The Moon is first settled in about as many games and
at the same median turn. More Ships end the game stranded.

## 9. Every leg of every journey priced the same way

The authority is [ticket #486](https://github.com/whaleyjoshua2/Dying-Earth/issues/486). The figures
are section 3.9 of [`docs/research/delta-v.md`](../research/delta-v.md). This section supersedes the
table of section 8 and the fares of section 7.

- **Every journey between any two places is priced one way, each direction for itself**, at section
  8's scale of 4 Fuel a km/s.
  - **Inside a system** a hop has its own figure, out and back: Earth and the Moon; Mars, Phobos
    and Deimos.
  - **Between two systems** a journey is the **leaving** of one end, the **gulf** between the two
    systems, and the **arriving** at the other. Arriving is lower where there is air to brake on.
  - The figures are in `bodies.toml`: `leave_delta_v` and `arrive_delta_v` on each Body, a
    satellite's `local_delta_v` and `local_return_delta_v`, and a `[[gulf]]` for each pair of
    systems. They reproduce the research note's table of every pair to 0.18 km/s at worst.
- **Earth's air brakes the way home**, as flown: a capture burn into a loose orbit, then the air. A
  leg no longer costs the same both ways.
- **The Moon is no longer priced as Earth.** A journey from the Moon to another system is the
  Moon's own, and about half Earth's.
- **Earth L4 and Earth L5 are places of their own for travel.** A leg from one is priced from
  there, not by way of Earth, and costs nothing to leave or arrive in, being outside every gravity
  well. They remain Earth's stations in every other way (section 7).
- **Every pair is flown.** The refusal of a leg between Venus and the Mars system is gone.

  Fuel at the window, from the row to the column:

  | | Earth | Moon | Venus | Mars | Phobos | Deimos | L4, L5 |
  |---|---|---|---|---|---|---|---|
  | **Earth** | | 15.7 | 17.4 | 18.1 | 19.7 | 19.9 | 16.3 |
  | **Moon** | 3.7 | | 8.2 | 9.0 | 10.6 | 10.8 | 7.2 |
  | **Venus** | 16.6 | 18.1 | | 26.8 | 28.4 | 28.6 | 23.2 |
  | **Mars** | 12.1 | 13.6 | 21.6 | | 4.8 | 6.9 | 20.1 |
  | **Phobos** | 10.2 | 11.7 | 19.7 | 2.4 | | 3.0 | 18.2 |
  | **Deimos** | 9.2 | 10.8 | 18.7 | 2.8 | 3.0 | | 17.3 |
  | **L4, L5** | 5.6 | 7.2 | 13.4 | 15.5 | 17.1 | 17.3 | 10.6 |

- **Every journey between two systems has a window the same way**: cheapest and shortest when its
  two ends line up, dearer and longer the further off.
  - **Venus and Mars** have a table of their own: 217.5 days at the window, four turns, coming
    round every 334 days.
  - **A far orbit reads Earth's sky sixty degrees round**, L4 ahead and L5 behind, so its windows
    to Mars and Venus are Earth's shifted, and its flights as long as Earth's.
  - **No window** inside the Earth system, the far orbits among it, nor inside the Mars system: a
    move takes a turn.
- **A move between Earth's orbits** is still 1 Fuel, and to or from a far orbit is that journey:
  16.3 out, 5.6 home, 10.6 across. The seat's own Fuel multipliers apply to it as to any journey.
- **Unchanged:** the tanks; the scale; the Faction's and Efficient Transit's cuts.
- **A Mass Driver takes a quarter off** every leg its owner's Ships fly from its Body, after the
  multipliers and to the tenth, at the designer's word, where it took a flat 4 Fuel with a floor of
  1. Against legs priced by delta-v the flat figure erased the Moon's 3.7 home and barely touched
  the way out; a quarter leaves 2.8 home and 6.8 to Mars. Its card says "departures from here 25%
  less Fuel". The computer seats built no Mass Driver in the 80 games of the sweep, before or
  after, so the sweep is the same to the line.
- **The computer seats** read every leg's price from the engine. One test of theirs moved: a
  Battleship holding Mars's orbit with cause and a full tank now flies home to blockade its rival's
  station, the leg being cheap, where it bombarded; short of that leg's Fuel it bombards as before.

**Measured** (80 games, the standing cell,
[`after-486.txt`](../dev-diary/2026-10-04-version-0.09.8/sweeps/after-486.txt)):

| | before this | after this |
|---|---|---|
| Custodians | 7 | 7 |
| Prospectors | 20 | 21 |
| Arkwrights | 17 | 16 |
| Archivists | 4 | 4 |
| collapses | 32 | 32 |
| Ships stranded at the end | 15 | 3 |

The win column is within noise. Stranding is back to where it stood before section 8.

## 10. A Moon race everyone can watch

The authority is [ticket #481](https://github.com/whaleyjoshua2/Dying-Earth/issues/481).

- **Until somebody lands on the Moon, the race to it is everyone's to see**, through the fog. It
  changes no rule: the first landing's prize is what it was (5 Influence, +1 a turn, Unrest eased).
- **Each Faction stands on the furthest of six steps**, read off the board:
  no Shipyard; a Shipyard; a Colony Ship; Colonists aboard a Colony Ship; a loaded Colony Ship on
  the way to the Moon; a loaded Colony Ship in the Moon's orbit.
- **The Moon's card lists the race**, under the line that says nobody has landed: a heading, "The
  race to the Moon", and one line a Faction in its colour, the leader first and ties in seat order:
  "Prospectors: in the Moon's orbit". It leaves the card at the first landing.
- **The Report says when a loaded Colony Ship is sent to the Moon**, before the first landing, by
  any Faction, the player's own included: "Arkwrights sent a Colony Ship to the Moon." It carries no
  place and no seat, so every seat reads it. An empty Colony Ship and a warship write nothing.
- **The first landing's Moment has words of its own**: "Prospectors win the race to the Moon.
  Arkwrights were on the way; Archivists had built a Colony Ship; Custodians had no Shipyard. Mare
  Tranquillitatis on the Moon is theirs; Unrest eased by 0.5 in every Region." Its figure is the
  Influence, as for any first landing. Every other Body's first landing reads as before.
- **Only the Moon.** No other Body has a race card.
- **The Report's word ceiling rises from 1,727 to 1,781**: the 54 words these lines add. The 15% cut
  of version 0.09.7 stands on what the Report said before.
- **The computer seats** play as they did. The sweep is the same to the line as the one before it
  ([`after-481.txt`](../dev-diary/2026-10-04-version-0.09.8/sweeps/after-481.txt)): 7 / 21 / 16 / 4
  and 32 collapses.

**Pictures:** [`ticket-481/`](../dev-diary/2026-10-04-version-0.09.8/ticket-481/).

## 11. The Regions redrawn around Iran, to eighteen

The authority is [ticket #482](https://github.com/whaleyjoshua2/Dying-Earth/issues/482).

- **Eighteen Regions**, where there were sixteen.
  - **Iran** is Iran, Pakistan and Afghanistan. It keeps its name, its flag and its place.
  - **Turkey**, new: Turkey, Iraq, Syria, Lebanon, Israel and Palestine, Jordan, Armenia,
    Azerbaijan and Georgia, out of Iran's Region.
  - **Kazakhstan**: the five Central Asian republics, alone on the card that was Pakistan's.
  - **South Africa**, new: South Africa, Namibia, Botswana, Lesotho, Eswatini, Angola, Zambia,
    Zimbabwe, Malawi, Mozambique and Madagascar, out of Nigeria's Region.
- **The cards.** People and GDP are shared out of the parents exactly; the world still holds 7,860.

  | Region | People | GDP | Influence | Industry Level | Leans | Size | Coast | Baseline emissions | Starts with |
  |---|---|---|---|---|---|---|---|---|---|
  | Iran | 345 | 1 | 1 | 2 | Fuel | 2 | 1 | 0.5 | Refinery, Power Plant |
  | Turkey | 185 | 3 | 1 | 2 | Materials | 2 | 1 | 0.4 | Factory, Mine, Power Plant |
  | Kazakhstan | 70 | 1 | 1 | 1 | Fuel | 2 | 0 | 0.4 | Power Plant, Mine |
  | South Africa | 210 | 1 | 1 | 2 | Materials | 2 | 2 | 0.4 | Mine, Power Plant, Factory |
  | Nigeria | 930 | 1 | 1 | 1 | Materials | 3 | 1 | 0.3 | Factory, Mine |

- **Turkey starts with a Mine** beside its Factory, which the designer's table did not list: every
  start Factory has had one since version 0.09.0, and a test holds it.
- **The world's Influence is 35**, where it was 34: each of the five has 1, at the designer's word.
- **Kazakhstan is landlocked**: no coastal slot, and the sea takes nothing from it. It is the first
  such Region; nothing in the engine needed changing for it.
- **Who touches whom.** Iran: Turkey, Kazakhstan, China, India, Saudi Arabia. Turkey: the European
  Union, Russia, Egypt, Saudi Arabia, Iran. Kazakhstan: Russia, China, Iran. South Africa: Nigeria.
  India and Iran touch again; Iran no longer touches the European Union or Egypt.
- **Where the computer seats start.** The spreading rule's four picks are China, the European
  Union, South Africa and Saudi Arabia, where the last two were Saudi Arabia and Australia: South
  Africa stands at Industry Level 2 with more people than either.
- **The map** is repainted from Natural Earth's countries. Compared pixel by pixel with the map
  before, it differs in the three moves and nowhere else.
- **Saves:** `SAVE_VERSION` is 9. A save of sixteen Regions is refused with the usual message.
- **Emissions were set by hand**, as in 0.09.6, and are the CO2 sweep ticket's to square: the
  industry line rises by about 1.6 a turn.

**Measured** (80 games, the standing cell,
[`after-482.txt`](../dev-diary/2026-10-04-version-0.09.8/sweeps/after-482.txt)):

| | before this | after this |
|---|---|---|
| Custodians | 7 | 4 |
| Prospectors | 21 | 20 |
| Arkwrights | 16 | 11 |
| Archivists | 4 | 6 |
| collapses | 32 | 39 |

Collapses rise by seven and the Arkwrights lose five wins, both past noise. Not traced; the CO2
sweep is run on this board next.

**Pictures:** [`ticket-482/`](../dev-diary/2026-10-04-version-0.09.8/ticket-482/).
