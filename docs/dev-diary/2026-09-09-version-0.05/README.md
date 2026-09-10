# 2026-09-09: version 0.05, ticket by ticket

Work on the 0.05 map, on the branch `version-0.05`.

## #50: four Factions in every game

The engine seats four Factions; this is the window catching up. Four cards at New Game, the rivals'
deeds kept apart per Faction in the Report, four-way Standings on every card, stack markers at four
angles round a Body instead of a left side and a right side, four Earth tints, the attack preview
against everyone present, and a Battle Report line per party.

Every picture below was taken headlessly with the game's own `shot:` mode
(`dying-earth.exe shot:<prefix> ...`, the window off-screen) and opened before it was written about.

![The New Game Faction screen: four cards in two rows of two, each with its colour swatch, name, blurb, multipliers, signature rule and Victory Condition](factions.png)

- **factions.png** — `shot:four turns:6`. The choice screen deals all four cards: Custodians (teal),
  Prospectors (orange), Arkwrights (violet), Archivists (pale silver-blue), each with its swatch,
  blurb, multipliers, signature rule and Victory Condition in words, and its own Play button.

![The start screen: "You play the Custodians" and "Played by the computer: Prospectors, Arkwrights, Archivists", each name in its Faction colour, beside the spinning Earth](start-rivals.png)

- **start-rivals.png** — `shot:st menus:1`. After the pick, the starting-continent screen names the
  three Factions the computer plays, in their own colours.

![The Solar System Map at turn 7: four coloured stack labels above Mars and the Orbital Control flag in the Custodians' teal](solar-four-stacks.png)

- **solar-four-stacks.png** — `shot:four turns:6`. Four Ship stacks at Mars, one per seat, marked at
  four fixed angles round the Body (left, right, above, below) with their labels stacked above it in
  seat order and coloured by Faction; "Orbital Control: Custodians" in teal above the Mars label.

![The Mars Body Surface Map: the band along the top listing four Factions' stacks, one line each in its own colour, and the Orbital Control line](mars-four-stacks.png)

- **mars-four-stacks.png** — `shot:four turns:6`. The band along the top of a Body Surface Map, which
  used to be one line: now one line per Faction present, in its colour, then Orbital Control in the
  holder's colour.

![The Earth Map with four Nation States in four different Faction colours and the South America card open beside it](earth-four-tints.png)

- **earth-four-tints.png** — `shot:tint2 turns:6 tints:1 look:-15,10 select:southamerica`. Four
  controllers, four tints: South America teal, Europe orange, the Middle East violet, Africa pale
  silver-blue. (`tints:1` is a building aid that hands one state to each seat; over twelve AI turns
  the Custodians usually hold everything, so a real board rarely shows four colours at once.)

![The Europe card with the Standings line as four coloured chips: Custodians 72, Prospectors 63, Arkwrights 60, Archivists 64](standings-four-chips.png)

- **standings-four-chips.png** — `shot:stand turns:12 select:europe look:20,35`. The Standings line
  on a Nation State card: a chip per seat with a Standing, in Faction colours, then the threshold and
  the challenge margin.

![The Report popup at turn 9: the seating line, then What the rival Factions did with a coloured section per rival](report-rivals.png)

- **report-rivals.png** — `shot:rep menus:1 turns:8`. The Report names the table ("Seed ... You:
  Custodians. Computer: Prospectors, Arkwrights, Archivists", each in its colour) and lists what each
  rival Faction did under its own coloured heading. Empty sections are left out.

![The Report popup at turn 10 with a Battle Report at Mars orbit: four party lines, one per Faction, each in its colour](battle-four-parties.png)

- **battle-four-parties.png** — `shot:bat menus:1 turns:8 battle:1`. A Battle is a melee now, so the
  Battle Report gives the place, then a line per party in its Faction's colour with its units,
  strength, hits landed, losses and escapes, then the result. Four parties here: the Custodians
  attacking, the Prospectors, the Arkwrights and the Archivists. (`battle:1` is a building aid that
  sends everyone's Ships to Mars and orders the attack, since the AI rarely stages a four-way fight.)

![The Custodians' Ship stack at Mars selected: "Against Prospectors 3, Arkwrights 3 and Archivists 0 (6 in all). Attack odds (first round): 50%"](attack-preview.png)

- **attack-preview.png** — `shot:stk turns:8 battle:1 stack:1`. The stack card's attack preview names
  every Faction with Ships at the Body and its strength, then the total and the first-round odds,
  because an attack there is a melee against all of them at once.

![The Victory panel with four rows, one per Faction, each headed in its colour with its percentage and its Victory Condition in words](victory-four-rows.png)

- **victory-four-rows.png** — `shot:vic turns:12 victory:1`. The Victory panel: a row per seat in
  seat order, the Faction name and its percentage in its colour, the Victory Condition in plain
  words, then the two bars.

![The game-over modal after a Collapse at turn 23, with a row per Faction in its colour](game-over-four-rows.png)

- **game-over-four-rows.png** — `shot:over turns:24`. The game-over modal names the outcome (a
  Collapse in this seed), the turn and the seed, then a row per Faction in its colour with its
  measure and its percentage. Where a Faction wins, the same line names it ("The Custodians win:
  ...").

### The climate clock for four seats

Added to the ticket by the designer. `engine/examples/sweep.rs` now takes `--player=` (the Faction
in seat 0), `--start=` (its Nation State), `--sinks=` and `--steps=`, twenty seeds per cell, and
reports collapses, the collapse turns, the end temperature and the wins by seat. The sim always
starts seat 0 in Asia unless told otherwise; the AIs spread from there by the ticket's rule.

**What the sweep found.** The clock is decided by who holds Asia (population 43.5, a third of the
world's people). A Prospector there at the old step of 90 collapses the world on turn 13 in every
seed; a Custodian there never collapses at any step above 90. No single step reproduces the 0.02
target (collapse in most seeds between turns 15 and 21) for every seating, so the step was chosen
to keep every seating hot at the end rather than to hit one figure.

Seat 0 the Prospectors, starting in Asia:

| sink | step | collapses | collapse turn (median, range) | end temp (median) | wins by seat |
|---|---|---|---|---|---|
| 6 | 90 | 20/20 | 13 (13..13) | +3.06 | 0 0 0 0 |
| 6 | 120 | 20/20 | 17 (16..18) | +3.04 | 0 0 0 0 |
| 6 | 150 | 20/20 | 19 (19..21) | +3.02 | 0 0 0 0 |
| 6 | 160 | 20/20 | 20 (20..21) | +3.02 | 0 0 0 0 |
| 6 | 170 | 6/20 | 24 (22..24) | +2.92 | 7 7 0 0 |
| 6 | 180 | 1/20 | 22 | +2.87 | 11 8 0 0 |
| 8 | 120 | 20/20 | 18 (17..18) | +3.05 | 0 0 0 0 |
| 8 | 180 | 1/20 | 23 | +2.80 | 15 4 0 0 |
| 10 | 150 | 1/20 | 21 | +2.84 | 10 9 0 0 |

Seat 0 the Custodians, starting in Asia:

| sink | step | collapses | collapse turn (median, range) | end temp (median) | wins by seat |
|---|---|---|---|---|---|
| 6 | 90 | 11/20 | 21 (18..23) | +3.01 | 9 0 0 0 |
| 6 | 100 | 11/20 | 23 (21..24) | +3.00 | 9 0 0 0 |
| 6 | 110 | 4/20 | 24 (21..24) | +2.88 | 16 0 0 0 |
| 6 | 120 | 0/20 | - | +2.56 | 20 0 0 0 |
| 6 | 150 | 0/20 | - | +2.30 | 20 0 0 0 |
| 7 | 90 | 10/20 | 22 (19..23) | +3.00 | 10 0 0 0 |
| 8 | 90 | 6/20 | 23 (21..24) | +2.96 | 14 0 0 0 |

Seat 0 starting in Europe, sink 6 (an AI takes Asia):

| seat 0 | step | collapses | collapse turn (median, range) | end temp (median) | wins by seat |
|---|---|---|---|---|---|
| Custodians | 90 | 20/20 | 13 (13..14) | +3.04 | 0 0 0 0 |
| Custodians | 120 | 20/20 | 18 (17..20) | +3.02 | 0 0 0 0 |
| Custodians | 150 | 13/20 | 23 (21..24) | +3.00 | 1 6 0 0 |
| Prospectors | 120 | 4/20 | 24 (21..24) | +2.88 | 0 16 0 0 |
| Prospectors | 150 | 0/20 | - | +2.69 | 0 20 0 0 |
| Arkwrights | 120 | 1/20 | 21 | +2.80 | 0 19 0 0 |
| Archivists | 120 | 0/20 | - | +2.40 | 0 20 0 0 |

**Chosen: `ppm_step = 120`, the Natural Sink unchanged at 6.0** (`climate.toml`). At 120 a
Prospector in Asia collapses the world on turns 16 to 18, a Custodian starting outside Asia on
turns 17 to 20, and every other seating ends between +2.4 and +2.9 C with a collapse in a seed or
two. At 150 half the seatings finish comfortable. The step is re-swept on the build ticket once the
climate tickets (Mothball, per-person Emissions, Scrubbers, Blame, neutral development, Tipping
Points) are in, since each of them moves it.

**The four-way balance, as measured, not fixed.** In every seating the seat that ends up holding
Asia wins, and it is the only seat that ever wins: the Custodians 20 of 20 from Asia, the
Prospectors 16 to 20 of 20 when the Custodians start elsewhere; the Arkwrights and the Archivists,
on provisional cards, have not won a game. Every win is on the last-turn score or the Colonists
tiebreak; no Faction has met its Victory Condition outright. The Custodian AI still takes every
Nation State by Influence when it starts in Asia, so the other three seats end with nothing to
build in (buildings 41, 0, 0, 0 in seed 19). No Battle happened in any logged AI game: the melee
is proven by the formula tests and the `battle:1` shot aid, not by the AI.

Twenty seeds at the chosen step:

| seating | wins | collapses | median collapse turn | median first Colony |
|---|---|---|---|---|
| Custodians in Asia | Custodians 20 | 0 | - | 10 |
| Prospectors in Asia | none | 20 | 17 | 9 |
