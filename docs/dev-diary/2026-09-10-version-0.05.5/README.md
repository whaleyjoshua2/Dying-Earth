# 2026-09-10: version 0.05.5, ticket by ticket

Work on the 0.05.5 map ([issue #66](https://github.com/whaleyjoshua2/Dying-Earth/issues/66)), on the
branch `version-0.05.5`. Every picture below was taken headlessly with the game's own `shot:` mode
(`dying-earth.exe shot:<prefix> ...`, the window off-screen) and opened before it was written about.

## #67: thirty-six turns of two months

[The ticket](https://github.com/whaleyjoshua2/Dying-Earth/issues/67). The game runs thirty-six turns
of two calendar months, January 2030 to November 2035, named by the first month alone; every per-turn
figure stays per turn; the sky follows the calendar at sixty days a turn, so the Hohmann flight is
five turns and the game holds three Mars windows; the Tech costs stay; and `ppm_step` gets a
provisional re-sweep so the six tickets that follow measure a game that reaches its late turns.

### What moved

| file | figure | was | is |
| --- | --- | --- | --- |
| `victory.toml` | `turns` | 24 | 36 |
| `victory.toml` | `months_per_turn` (new) | 1 | 2 |
| `ephemeris.toml` | `days_per_turn` | 30 | 60 |
| `ephemeris.toml` | `max_turns` (the flight cap) | 18 | 9 |
| `climate.toml` | `ppm_step` | 180 | 270 (provisional) |

One rule changed with the calendar, a builder's call recorded on the ticket: a turn now spans two
months, over which the phase angle moves nearly thirty degrees, so a turn's window offset is the
nearest the angle comes to the Hohmann angle **anywhere in the turn**, zero when it crosses inside
the turn. Read at the turn's first instant alone, no turn would ever stand at the window and every
crossing paid a Fuel over the card (the first green run showed 21 for 20).

### The provisional sweep

The step alone, the Sink and the Breaks untouched, twenty seeds a cell.

| seat 0 | step | collapses | median Collapse turn (range) | end Temperature |
| --- | --- | --- | --- | --- |
| Prospectors in East Asia | 180 | 20/20 | 22 (20..23) | +3.03 |
| Prospectors in East Asia | 240 | 20/20 | 29 (27..32) | +3.02 |
| Prospectors in East Asia | **270** | **15/20** | **35 (32..36)** | **+3.02** |
| Prospectors in East Asia | 300 | 2/20 | 34 | +2.89 |
| Prospectors in East Asia | 360 | 0/20 | - | +2.58 |
| Custodians from Europe | 240 | 20/20 | 28 (26..32) | +3.02 |
| Custodians from Europe | **270** | **17/20** | **34 (31..36)** | **+3.01** |
| Custodians from Europe | 300 | 4/20 | 36 (35..36) | +2.93 |

270 chosen: three quarters of seeds collapse, late, and the world stays at +3.0 to the last turn.
There is nothing between 240 (everything collapses) and 300 (almost nothing does) but this cell.

### Twenty seeds at the chosen step

| seat 0 | wins | collapses | first Colony (median turn) | Mars system reached | Antarctic Colonies | Techs (median) | highest rung | Sea Walls |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Prospectors in East Asia | Custodians 5 (seat 1) | 15/20 | 10 | **20/20 seeds, median turn 11** | 32 | 2 | 2 | 0 |
| Custodians from Europe | Custodians 3 | 17/20 | 10 | **20/20 seeds, median turn 11** | 32 | 3 | 3 | 0 |

No Victory Condition was met outright in any seed; the wins are the last-turn ranking. Every Break
fired in every seed. Colonists off Earth at the end: median 28, all seats. Scrubbers 88 to 129,
Leapfrogs 425 to 434, Constabularies 100 to 133 a batch.

**The Mars system, unreachable in every 0.05 batch but one, is now reached in every seed**, at a
median turn 11, one window and one five-turn flight in. **Rung 2 arrives, and once rung 3**, where
0.05 never left rung 1 in six batches of eight. **The Sea Wall is still never built**, so Coastal
Engineering on rung 2 is still not reached in time; the Research ticket moves it to rung 1.

### Pictures

![The Report popup headed "Report, March 2030" over the Earth Map, the top bar reading "Turn 2 / 36, March 2030"](calendar-report.png)

- **calendar-report.png** — `shot:calendar turns:1 menus:1`. Turn 2. The top bar reads **"Turn 2 /
  36, March 2030"** and the dispatch is headed **"Report, March 2030"**: a turn is named by its first
  month alone, and turn 2 is March, not February. Under the 0.05 calendar this same turn read
  February 2030 and "Turn 2 / 24".

![The Solar System Map on turn 7: Earth and Mars both left of the Sun about forty-five degrees apart, the top bar reading "Turn 7 / 36, January 2031", and the tooltip line reading "Mars window: this turn (January 2031). Flight now: 5 turns, 20 Fuel. At the window: 5 turns, 20 Fuel."](window-solar.png)

- **window-solar.png** — `shot:window turns:6 hover:mars`. Turn 7, **January 2031**, the first Mars
  window, where 0.05's `solar-window-turn.png` showed the same real window on **turn 14, February
  2031**, with a nine-turn flight. The tooltip now reads **"Flight now: 5 turns, 20 Fuel"**: the
  card's Fuel at the window, which the span rule above is for. Earth and Mars stand left of the Sun
  about forty-five degrees apart, the departure geometry, as in the 0.05 picture.

Not pictured: the Load screen's month column and the save file names read the same `date` function
and so name the first month too; the six save tests pass unchanged.

## #68: the Archive as a Module of three turns, its fund capped at a quarter until it stands

[The ticket](https://github.com/whaleyjoshua2/Dying-Earth/issues/68). The Archive is one Module,
50 Materials and three turns from its own button at a Colony off Earth; the four stages are retired.
Its 80 Research is still required, paid into the fund at any pace once the Module stands; until then
the fund holds a quarter (20), and at the cap a turn of funding is refused rather than wasted. The
Archive is complete when it stands and every point is paid; from then it draws its 12 Energy and the
Victory Condition reads as before. Provisional Findings is unchanged.

### What moved

| file | figure | was | is |
| --- | --- | --- | --- |
| `modules.toml` | the Archive row | 30 Materials, 2 turns, a stage | 50 Materials, 3 turns, one Module |
| `modules.toml` | `[archive]` | 4 stages of 20 Research | `research` 80, `banked_before_built` 0.25 |
| `factions.toml` | the Archivists' first part | `archive_stages`, bar 4 | `archive_research`, bar 80 |
| `ai.toml` | `[pace.archivists] first` | stages by turn 8, 13, 18, 22 | Research 20, 40, 60, 80 by turns 10, 18, 26, 32 |
| `save.rs` | the rules version stamp | 0.05 | 0.05.5 (a Module lost its stage field) |

**The AI learned the whole path.** Ticket #51 never counted a Launch Site or a Shipyard as
advancing the Archive, so an Archivist AI whose station starts bare spent every turn on Influence
and never left Earth; both count now, and for the steps of the Archive's own path the AI banks
Materials over twelve turns of income rather than four, since 50 Materials is never within four
turns of four a turn. The glossary retires "Project" and rewords the Archive and its fund.

### Measured, twenty seeds each

| seat 0 | collapses | Archivist Shipyards | Colony Ships | Archivist Colonies founded | Archive begun | Archive standing | lost their start state |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Archivists in East Asia | 7/20 | 40 | 42 | 10 | 1 | **0/20** | **20/20, median turn 20 (13..23)** |
| Archivists from Europe | 20/20 | | | | | **0/20** | |

Before this ticket the same seat founded no Colony in twenty seeds. Now it builds a Shipyard at Axiom
in every seed and a Colony Ship in most, founds a Colony in about half, and began the Archive once;
the Module stood in none. The fund sits at its quarter (20) at the end of every seed.

**Why it stops there is not the Archive.** In every seed the Custodians take East Asia from the
Archivists by Influence at a median turn 20, and from then the Archivists' income reads `+0
Materials, Research 0`: a one-state Faction at output x0.8 and Influence x1.0, beside a Custodian
seat at x1.25, holds its state for half the game and then holds nothing. That is the card and the
start, which the map rules out of this version's scope; it is recorded for the build ticket's "Open
for the designer" beside 0.05's finding 4. Other lines from the same batches: the Mars system
reached in every seed (median turn 12), Techs at a median 8 and 13 a game with rung 3 reached, no
Sea Wall, and four Custodian Stabilization wins outright in the East Asia batch, the first ever seen.

### Pictures

![The Olympus Mons Colony card on Mars: the Modules list ends "The Archive: standing, 40 of 80 Research paid", and below it "The Archive. Archive fund 40 of 80" with the Fund checkbox](archive1-mars.png)

- **archive1-mars.png** — `shot:archive1 player:archivists turns:6 archive:1`. The Archive Module
  standing at Olympus Mons with half its Research paid: the Modules list says **"standing, 40 of 80
  Research paid"**, the Archive section reads **"Archive fund 40 of 80"** with the Fund box live, and
  there is no Build button, because it is built.

![The same card with the Archive on order: "The Archive: building, 2 turn(s) left", and "Archive fund 20 of 20 (a quarter of the 80 until the Archive stands)" with the Fund checkbox greyed](archive0-mars.png)

- **archive0-mars.png** — `shot:archive0 player:archivists turns:6 archive:0`. The Module on order:
  **"building, 2 turn(s) left"**, the fund at **"20 of 20 (a quarter of the 80 until the Archive
  stands)"**, and the Fund box **greyed out** at the cap, which is the refusal of the resolution's
  point 4 made visible.

## #69: two start Labs, the neutral half, and the Sea Wall's Tech on rung 1

[The ticket](https://github.com/whaleyjoshua2/Dying-Earth/issues/69). North America and South-East
Asia each start with a Research Lab added to their start Facilities, standing inland. A Lab in a
Nation State nobody holds, or one under Occupation, runs itself, pays no Energy, and pays half its
yield (rounded down) into the Tech under research for no Faction; under Occupation the occupier pays
the Lab's upkeep and draws nothing from it. A Faction starting in either state keeps its Lab whole.
Coastal Engineering moves from Industry rung 2 at 25 to rung 1 at 10 with no prerequisite.

### What moved

| file | figure | was | is |
| --- | --- | --- | --- |
| `nation_states.toml` | North America's start Facilities | Refinery, Power Plant, Factory | + Research Lab |
| `nation_states.toml` | South-East Asia's start Facilities | Refinery, Power Plant | + Research Lab |
| `techs.toml` | Coastal Engineering | rung 2, cost 25, needs Efficient Grids | rung 1, cost 10, needs nothing |

**Builder's calls.** A start Research Lab stands inland whatever the coastal count. The world's Lab
yield is the row's figure by the state's people and schooling, times Public Science once every
Faction has it, with no Faction multiplier: 3 in North America and 2 in South-East Asia, so 1 each a
turn to the pool. A Lab idled by a Wildfire or mothballed pays nothing. The Report says it under On
Earth on turns it is more than zero. The state card's Lab line says "in no one's hands: N Research a
turn to the Tech under research" rather than the old "idle, nobody directs this state". Ticket #24's
rule that start Facilities number as many as the Industry Level now reads "plus a start Lab".

### Measured, twenty seeds each

| seat 0 | collapses | end Temperature | Techs (median) | highest rung | Coastal Engineering done | Sea Walls | neutral Labs paid (median) | outright wins |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Prospectors in East Asia | **0/20** | +2.35 | **11** | 3 | 20/20, median turn 18 | **0** | 17 | Custodians 16/20 (Stabilization) |
| Custodians from Europe | **0/20** | +2.28 | **12** | 3 | 20/20, median turn 16 | **0** | 17 | Custodians 20/20 (Stabilization) |

After the calendar ticket the same seatings collapsed 15 and 17 of 20 at a median turn 34 to 35
with the world at +3.0, and completed 2 or 3 Techs a game. **The Tech pace has quadrupled and the
climate has turned over with it**: no seed collapses, the world ends at +2.3, and the Custodians
meet Stabilization outright in 36 of 40 games where no Custodian had ever met it in any version.
The neutral Labs' 17 Research a game is not the cause on its own; the two start Labs are a Faction's
from turn 1 whenever a seat starts there, and the AI now reaches Clean Power and Green Consensus.
The climate step (270, provisional) was chosen at the old pace; **the build ticket's re-sweep is
where the clock is set**, and this is recorded on the map rather than re-tuned here.

**Coastal Engineering is done by turn 16 to 18 in every seed, and still no Sea Wall is built.** The
logged run says why: the Sea Wall is offered, and every time the AI holds its Materials for a
Scrubber instead ("holding Materials for build a Scrubber in ..."), since only the Custodian AI
considers coastal defence and the Scrubber advances its Victory Condition while a Sea Wall does
not. Reachable now, never chosen: an AI weighting for the build ticket.

### Pictures

![The Tech Tree: Coastal Engineering "cost 10 - available" beside Efficient Grids on the Industry column's first rung, Clean Power alone on the second](tree-earth.png)

- **tree-earth.png** — `shot:tree tech:1 turns:0 panel:0`. **Coastal Engineering on Industry rung 1
  at "cost 10 - available"**, beside Efficient Grids, with Clean Power alone on rung 2 under
  Efficient Grids and Clean Manufacturing under it. In 0.05's `tech-tree-thirteen.png` the same box
  sat on rung 2 beside Clean Power at cost 25.

![The North America card at turn 1: "Coastal: Refinery Power Plant Factory", "Inland: Research Lab free free free free free", and the Facilities list ending "Research Lab (inland): in no one's hands: 1 Research a turn to the Tech under research"](lab-earth.png)

- **lab-earth.png** — `shot:lab select:northamerica turns:0 look:-100,40`. Neutral North America
  with its four start Facilities: the three from ticket #24 on the coast and the **Research Lab
  inland**, its line reading **"in no one's hands: 1 Research a turn to the Tech under research"**.
  The first take of this picture showed the old "idle, nobody directs this state" line, which is
  what the label fix above answered.

![The Report of March 2030 with, under On Earth, "The Labs of South-East Asia and North America, in no one's hands, added 2 Research to the Tech under research."](neutral-report.png)

- **neutral-report.png** — `shot:neutral turns:1 menus:1`. Turn 2's dispatch, under **On Earth**:
  "The Labs of South-East Asia and North America, in no one's hands, added 2 Research to the Tech
  under research."

## #70: Influence ties by lot, fifteen coastal slots moved inland, and the Sea Wall the AI now builds

[The ticket](https://github.com/whaleyjoshua2/Dying-Earth/issues/70). Two challengers at the same
Standing on a neutral place draw lots from the game's own generator; on a held place the holder
keeps it. Coastal slots per point of Coastal Exposure go from 3 to 2, so the world holds 34 coastal
slots where it held 49, and Europe's Refinery and North America's Factory stand inland from turn 1.
While the sea is within 0.2 C a Sea Wall takes the victory-gap and threat multipliers, so it competes
with the Scrubber.

### What moved

| file | figure | was | is |
| --- | --- | --- | --- |
| `nation_states.toml` | `coastal_per_exposure` | 3 | 2 |
| `report.toml` | `claim_lot` (new) | | "{place} is claimed by {factions} at the same Standing; the lot falls to the {winner}." |

**Builder's calls.** The lot is drawn with the same generator a contested orbital slot uses, so a
seed replays the same draw. A Faction's Launch Site, added after the card's Facilities, now stands
inland in Europe and North America because their two coastal slots are full. The Sea Wall's threat
multiplier is the sea itself: the state is under threat, as it is from an Army next door. Three older
tests moved with the count: East Asia has four coastal slots now, so the Ice Sheets Break and the
+1.8 threshold take them all and the scheduled +2.3 fires and finds nothing.

### Measured, twenty seeds each

| seat 0 | collapses | end Temperature | Sea Walls built | coastal slots lost (median) | Facilities drowned (median) | Custodian wins |
| --- | --- | --- | --- | --- | --- | --- |
| Prospectors in East Asia | 0/20 | +2.32 | **3** (was 0) | **34** (was 49) | **24** (was 27 to 29) | 20/20 |
| Custodians from Europe | 0/20 | +2.25 | **5** (was 0) | **34** (was 49) | **24** | 20/20 |

**The first Sea Walls ever built in any version**: three and five a batch, all by the Custodian AI,
where every earlier batch of every version reported zero. The sea still takes every coastal slot the
world has by the end of a game; there are fifteen fewer to take, and three to five fewer Facilities
drown. The rest of the picture is the Research ticket's: no Collapse, the world at +2.3, the
Custodians winning every seed on Stabilization.

### Picture

![The Europe card at turn 1, held by the Prospectors: "Coastal: Power Plant Factory", "Inland: Refinery Launch Site free free free free", and the Facilities list naming the Refinery and Launch Site as inland](coast-earth.png)

- **coast-earth.png** — `shot:coast select:europe turns:0 look:15,50`. Europe at turn 1 as the
  Prospectors' start state: **Coastal: Power Plant, Factory**; **Inland: Refinery, Launch Site**, and
  four free inland slots. In 0.05 all three start Facilities and the Launch Site stood on the coast
  (three coastal slots per point of Exposure), and the sea took the lot.
