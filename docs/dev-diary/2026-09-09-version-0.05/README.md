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
