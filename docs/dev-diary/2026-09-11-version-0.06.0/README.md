# Version 0.06.0, the space version: the build diary

Pictures and measurements taken while the tickets of
[Map: version 0.06.0](https://github.com/whaleyjoshua2/Dying-Earth/issues/79) were built, on the
branch `version-0.06.0`. Every picture below was taken headlessly with the game's own `shot:` mode
(`dying-earth.exe shot:<prefix> ...`, the window off-screen) and opened before it was written about.
Every batch is twenty seeds of four-seat `simulate` through the engine's `sweep` example, its output
kept whole under `sweep/`.

## The Observatory ([ticket #80](https://github.com/whaleyjoshua2/Dying-Earth/issues/80))

- **observatory-mars.png** — `shot:observatory player:archivists turns:0 observatory:25`. The card of
  Olympus Mons on Mars, held by the Archivists, with **25 Colonists of 33 Habitat room** (three
  Habitats of eight at the slot's Habitat yield) and the Module list reading **"Observatory: +4
  Research, 3 Energy upkeep"**: 2 Research, times 1.25 for the 25 Colonists, times the Archivists'
  1.6, rounded down. The Build section carries the new button, **"Observatory (28 Materials) or (56
  Ducats)"**, beside the eight Modules that were there before.

### Measured, twenty seeds each (`sweep/observatory.txt`)

Two seatings at the 0.05.5 climate cell (`ppm_step` 300, Sink 6), the same ones the 0.05.5 balance
report opened with, so the figures can be read against it.

| Seat 0 | Wins by seat | Collapses | Techs (median) | Observatories at the end, all seeds, by seat | Research off Earth a game (median) |
|---|---|---|---|---|---|
| Custodians from East Asia | Custodians 5, nobody else | 15/20 | 9, rung 3 | [0, 0, 0, 0] | 0 everywhere |
| Archivists from East Asia | Custodians 20 (seat 1) | 0/20 | 13, rung 3 | [0, 0, 0, 3] (the Arkwrights' three) | 0 everywhere |

**The rule works and the AI almost never affords it.** In the one seed's log read closely (seed 1,
the Archivist seating), the Custodian AI offers "build Observatory at Lake Vostok" every turn a
Colony of its holds eight Colonists, scored 8.0 (its Research Lab weight) and 24.0 with the victory
gap, but the line always reads *save ... holding Materials for* a Scrubber, a Sea Wall or a Factory
scored higher: the same fate its Research Labs on Earth meet in the same list. The Arkwright AI,
whose Colonies fill to twelve, is the one that builds any. The Archivist AI, which qualifies at four
Colonists, loses East Asia in 17 of 20 seeds at a median turn 21 (0.05.5's open item) and ends with
no buildings at all. Nothing else moved: wins, Collapses, Techs and the Fund read as 0.05.5 did.
Whether the Observatory should outrank a Scrubber in the Custodian AI's hand is on the map.

## The Archivists' card ([ticket #81](https://github.com/whaleyjoshua2/Dying-Earth/issues/81))

- **archivists-factions.png** — `shot:archivists player:archivists turns:0 observatory:25`, the
  Faction choice screen. The Archivists' card now reads **"Facility and Module output x1"** (0.8
  before) and **"Research x1.5"**, and its signature ends **"Their Research is x1.5 on Earth and
  x1.75 off it, an Observatory on a station over Earth counting as off."** The Arkwrights' card
  reads "(12, 15 with Expanded Habitats)" where the Observatory ticket's build had left "(6, 9)".

### Measured, twenty seeds each (`sweep/archivists.txt`)

The same two seatings as the Observatory ticket, so the figures read against that file.

| Seat 0 | Wins by seat | Collapses | Techs (median) | Observatories at the end, by seat | The Archive standing | Research off Earth (median) |
|---|---|---|---|---|---|---|
| Archivists from East Asia | Custodians 18, Arkwrights 1 | 1/20 | 13, rung 3 | [2, 7, 0, 0] | **20/20 seeds, median turn 4**, complete 0/20 | 0 everywhere |
| Custodians from East Asia | Custodians 3 | 17/20 | 7, rung 3 | [0, 0, 0, 8] | **20/20 seeds, median turn 4**, complete 0/20 | 0 everywhere |

**The Archive stands on Axiom.** With a station over Earth counting as off Earth, the Archivist AI
raises the Archive on its start station on turn 4 in every seed, where before this ticket it stood
in no seed of either seating. It is never completed: the fund ends at a median 20 and 12, so the
Research is not being paid in, and the Archivists lose East Asia in 20 of 20 seeds at a median
turn 22 (0.05.5's open item) before it could be. Off-world Presence and the Archivists' twelve
Colonists at the Archive can now both be met in orbit over Earth; how far that should be allowed
is the designer's, and is on the map.

**Observatories** rose from 0 and 3 to 9 and 8 across the two batches with their own weight (the
Archivists' at 12), the Custodian AI in seat 1 building most of them (7); Research made off Earth
still reads a median 0 a game for every seat, so most seeds see none. Everything else moved within
seed noise: the Custodians' wins 5 to 3 and 20 to 18, Techs 9 to 7 and 13 to 13.

## The Custodians' card ([ticket #82](https://github.com/whaleyjoshua2/Dying-Earth/issues/82))

- **moved-mars.png** — `shot:moved player:custodians turns:0 observatory:16 idle:1`. A Custodian
  Colony on Mars with a Factory and a Research Lab standing mothballed in East Asia. The card reads
  **"Mine: +10 Materials, doubled by an idle Factory on Earth"** and **"Observatory: +4 Research,
  doubled by an idle Research Lab on Earth"**, and the top bar's Allotment reads **16 of 16**:
  (10 + 4) x 1.2, where 1.25 gave 17.

### Measured, twenty seeds each (`sweep/custodians.txt`)

The same two seatings as the two tickets before, read against `sweep/archivists.txt`.

| Seat 0 | Wins by seat | Collapses | Scrubbers | Techs (median) | Production Moved (median doubled Module-turns a game) |
|---|---|---|---|---|---|
| Custodians from East Asia | **nobody** (Custodians 3 before) | **20/20** (17) | **94** (173) | 7 | 0 for every seat |
| Archivists from East Asia | Custodians 15, Arkwrights 4 (18 and 1) | 1/20 (1) | 496 (539) | 13 | 0 for every seat |

**Influence 1.2 is the whole of the move, and it is not small.** Seed 3's log shows the new
mothball rule never taken (no "would double" order, no doubled Income line), so the doubling
cannot be what moved the Custodian seating. The other change is the Influence multiplier, so the
seating was rerun once with 1.25 put back and everything else kept: **Scrubbers 185, Collapses
17, Custodian wins 3**, the previous batch's figures. The five hundredths cost the Custodians in
East Asia every win in twenty seeds and half their Scrubbers; East Asia's Allotment falls from 17
to 16, and the AI's Influence game turns on those steps. Read against the Observatory batch, this
seat has gone 5, 3, 0 wins across the three tickets so far.

**Production Moved never fires in AI play.** The Custodian AI idles a Facility only when an
undoubled Module off Earth outproduces it, and its Colonies seldom hold one that does: an Earth
Factory in a Materials-leaning state makes 6, a Moon Mine 6, a Mars Mine 5; only Phobos (7) or
Deep Mining tips it, and the AI reaches neither in time. The rule works on the player's side, as
the picture shows; on the AI's it is a rule in waiting.

## The Prospectors' Ducats and market; the Arkwrights' Ships ([ticket #83](https://github.com/whaleyjoshua2/Dying-Earth/issues/83))

- **cards-factions.png** — `shot:cards player:prospectors turns:0`. The Faction choice screen. The
  Prospectors' multipliers now read **"a state's Ducats x1.2; the Trading window's prices x0.85"**
  and their signature ends "Their states pay a fifth more Ducats, and the Trading window sells to
  them at 15% off." The Arkwrights' read **"every Ship x0.85 Materials"** and their signature says
  "every Ship costs them 15% less (a Colony Ship 25 Materials)" where it said 20.

### Measured, twenty seeds each (`sweep/prospectors-arkwrights.txt`)

The two seatings this ticket touches, read against the 0.05.5 balance report (the same seeds and
cell), so the figures carry every 0.06.0 ticket built so far, not this one alone.

| Seat 0 | Wins by seat | Collapses | Seat 0 lost its home | The Fund at the end (median) | Colonists off Earth (median) | Scrubbers |
|---|---|---|---|---|---|---|
| Prospectors from East Asia | Custodians 2 (11 in 0.05.5) | 18/20 (9) | 3/20 (11) | **437** (165) | 12 (16) | 109 (344) |
| Arkwrights from East Asia | Custodians 18 (19) | 2/20 (0) | 20/20 (20) | 176 (160) | 36 (50) | 425 (572) |

**The Prospectors keep their home and fill the Fund; nobody wins.** From East Asia they lose the
state in 3 seeds where they lost it in 11, and the Fund ends at a median 437 where it ended at 165,
its best figure yet; but 750 is not reached in any seed, and the seating now collapses in 18 of 20
where it collapsed in 9, because the Custodian AI beside them builds 109 Scrubbers where it built
344. That is the Influence 1.2 story of the Custodians' ticket seen from the other chair: the
Custodians hold fewer states, the Prospectors keep theirs, and the world burns sooner.

**The Arkwrights' cheaper Ships did not move their Colonists.** From East Asia they end with 36
Colonists off Earth, all seats, where 0.05.5 had 50; they lose East Asia in every seed at turn 23
as before, and the Custodians beside them still win 18. What a cheaper Ship buys them is spent on
the same shipping schedule; the bar that stops Diaspora is the three Bodies with four each, which
the Venus and tank tickets will change more than a price does.

## Every Victory Condition waits on a Tech ([ticket #84](https://github.com/whaleyjoshua2/Dying-Earth/issues/84))

- **gates-earth.png** — `shot:gates tech:1 turns:0 panel:0`. The Tech Tree on turn 1 with the four
  gates on rung 3, each bordered in its Faction's colour: **Generation Ships** (purple, Off-world
  Living, after Closed-Loop Colonies), **The Extraction Charter** (orange, Extraction, after
  Automated Refining), **Planetary Stewardship** (teal, Society, after Green Consensus) and **The
  Upload** (pale blue, Society, after Public Science and Expanded Habitats), all at cost 40.

### Measured, twenty seeds each, all four seatings (`sweep/gates.txt`)

| Seat 0 | Wins by seat | Collapses | Techs (median) | Gates completed, seeds by seat | Gate median turn |
|---|---|---|---|---|---|
| Custodians from East Asia | nobody | 20/20 | 7 | [5, 2, 2, 1] | 28, 32, 33, 30 |
| Prospectors from East Asia | Custodians 2 | 18/20 | 6 | [1, 2, 1, 1] | 27, 26, 27, 28 |
| Arkwrights from East Asia | Custodians 18 | 2/20 | 15 | [9, **20**, 10, 10] | 32, **27**, 31, 32 |
| Archivists from East Asia | Custodians 19 | 1/20 | **17** | [11, **20**, 15, 13] | 32, **27**, 31, 32 |

**The gate opens late and the Custodians walk through it anyway.** In the two seatings where the
world holds, the Custodian AI (seat 1) completes Planetary Stewardship in every seed at a median
turn 27 and still wins 18 and 19 of 20 (18 and 15 before the gate): its Stabilization run and its
twelve Colonists were already standing, and its Lead picks fetched the gate in time. The other
three complete theirs in about half the seeds, at turns 31 to 32, with nothing else of their
conditions met. In the two seatings that collapse around turn 19 to 21 almost no gate completes
at all, and nobody wins where nobody won before. Techs completed rose to a median 15 and 17 in the
calm seatings (13 before): with seventeen Techs on the tree and Research abundant late, the AIs
research the whole of it.

**The AI picks its gate as Lead** once its first part is past half or from turn 24
(`ai.toml`, `gate_pick_fraction` and `gate_pick_turn`), the road standing; that timing is the
builder's, and the median turn 27 for the Custodians says it fires about when it should.

## The Antarctic founding Moment ([ticket #85](https://github.com/whaleyjoshua2/Dying-Earth/issues/85))

- **antarctic-report.png** — `shot:antarctic antarctic:2 moment:colony menus:1`. The Custodians
  have sent four Emigrants by sea to each of the first two Antarctic slots on consecutive turns;
  the Moment for the second reads **"The Custodians founded Lake Vostok on Earth, their second
  Colony in Antarctica."** where it read "their 2 Colony off Earth". The roster beside it lists
  both Antarctic Colonies, and its station line now reads "a Shipyard, Habitats and Observatories",
  a text left behind by the Observatory ticket.

No batch for this ticket: it changes four phrases and a count, seen red first in a test that founds
twice in Antarctica and twice on the Moon and reads all four Moments.

## A warming Earth fills the Colony Ships ([ticket #86](https://github.com/whaleyjoshua2/Dying-Earth/issues/86))

- **crowded-report.png** — `shot:crowded crowded:1 moment:lost menus:1`. The world at **+2.6 C**
  (the top bar), and a Custodian Colony Ship that left Earth with eight aboard, four beyond its
  capacity, arriving at the Moon. The new Moment reads **"2 of the 4 crowded aboard the Custodians'
  Colony Ship 20 died on the way to the Moon."** with the figure "2 lost"; the roster beside it
  shows the ship at the Moon with six aboard.

### Measured, twenty seeds each (`sweep/crowding.txt`)

Read against `sweep/gates.txt`, the batch before this ticket.

| Seat 0 | Wins by seat | Collapses | Colonists off Earth (median) | Colonists lost in transit over the batch, by seat |
|---|---|---|---|---|
| Custodians from East Asia | Custodians 1 (0) | 19/20 (20) | 18 (16) | [7, 11, 0, 0] |
| Arkwrights from East Asia | Custodians 18 (18) | 2/20 (2) | 41 (39) | [1, 11, 0, 0] |

**The crowd flies, and it costs what the rule says.** In both seatings the AIs behind on Off-world
Presence lift the crowded load once the world passes +2.0 C, and about a Colonist a game dies for
it, all of them Custodians' and Prospectors' (the Arkwrights and Archivists, in seats 2 and 3, lose
none: the Arkwrights carry eight safely and are seldom behind, the Archivists seldom fly). The
Colonists who survive show as two more off Earth at the end in each seating. Nothing else moved.

## Every Ship carries its own tank ([ticket #87](https://github.com/whaleyjoshua2/Dying-Earth/issues/87))

- **tanks-solar.png** — `shot:tanks dry:1 stack:1 turns:0`. The Solar System Map with the
  Custodians' Frigate at Mars selected, one Fuel in its tank and no station of theirs overhead.
  The transit list reads **"To Phobos: 1 turn(s), 2 Fuel each from the tank"** with the button
  **"Frigate 16 (1/30 in the tank)"** greyed for every leg, and the new **Tanks** section reads
  **"Frigate 16: 1/30 Fuel — stranded: no leg it can pay, and no station of yours here to refuel
  at; a station built in orbit here rescues it"** in red. (The greyed button says "(free)" because
  nothing comes from the Stockpile; the tank's price is on the line above it.)

### Measured, twenty seeds each, all four seatings (`sweep/tanks.txt`)

Read against `sweep/gates.txt` (the crowding ticket ran only two seatings).

| Seat 0 | Wins by seat | Collapses | Colonists off Earth (median) | First Mars Colony (seeds, median turn) | Refuel orders | Ships stranded at the end, by seat | Stations off Earth at the end |
|---|---|---|---|---|---|---|---|
| Custodians from East Asia | nobody (nobody) | 20/20 (20) | 18 (16) | 7/20, turn 13 (7, 13) | 43 | [0, 1, 0, 0] | 0 |
| Prospectors from East Asia | Custodians 2 (2) | 18/20 (18) | 10 (12) | 20/20, turn 13 (20, 13) | 33 | [0, 0, 0, 0] | 0 |
| Arkwrights from East Asia | Custodians 19 (18) | 1/20 (2) | 40 (39) | 17/20, **turn 25** (18, 19) | 79 | [0, 0, 0, 0] | 2 |
| Archivists from East Asia | Custodians 16 (19) | 4/20 (1) | 53 (56) | 19/20, turn 15 (20, 15) | 114 | [0, 0, 0, **11**] | **40** |

**The tank is paid for and the game goes on.** The AIs refuel at their stations 33 to 114 times a
batch, almost nobody is stranded at the end, and Colonists off Earth, wins and Collapses hold
within seed noise. Two things moved. **The Arkwrights' own seating founds on Mars six turns
later** (a median turn 25 where it was 19): a Colony Ship that flies to Mars on 20 of its 30 Fuel
cannot come home without a station over Mars, and the Arkwright AI, which starts with no station
and has never built one off Earth, waits. **In the Archivist seating the Arkwright AI (seat 3)
builds forty stations off Earth and still strands eleven Ships** over twenty seeds: it builds the
station after the Ship has flown on, or over a Body its stranded Ship is not at. Both are the
supply line the designer asked for, seen from the AI's chair; whether the Arkwright AI should
build its forward station before the crossing is on the map.
