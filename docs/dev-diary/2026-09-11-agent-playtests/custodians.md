# Playtest: the Custodians, version 0.06.0 (three games)

Played headless with `target/release/examples/play.exe`, seat 0, 36 turns available.

## Game 1 — won on turn 16 of 36

### Turn log

| T | What I did | What happened | How it felt |
|---|---|---|---|
| 1 | Picked **Public Science**; 2 Research Labs in Europe; 18 Influence on North America | I start with **no Research Lab at all**, so my Research is 0 until turn 2 | A real opening decision: labs or expansion. Good turn. |
| 2 | Bank in Europe (GDP 20 -> 8 Ducats/turn); 18 Inf -> NA | All three rivals also spent on NA (10+5+10) | Tense: a four-way race for the best state on the board. |
| 3 | Picked **Efficient Grids**; sold nothing, bought 8M + 10E; Power Plant Europe; 18 Inf -> NA | Energy went negative at -6/turn, first squeeze | The energy squeeze was the one genuine constraint of the early game. |
| 4 | Picked **Clean Power**; sold 20 Fuel -> bought 21 Materials; Bank in NA | **Took North America** (Inf 54/50) | Discovering that Fuel sells for Ducats and Ducats buy Materials 2:1 was the economic unlock of the game. |
| 5 | Picked **Green Consensus**; Research Lab NA; 26 Inf -> South Asia | Coral Die-off Break at +1.4 | |
| 6 | Picked **Deep Mining**; 2 Factories in South Asia; 30 Inf -> SE Asia | **Took South Asia** (pop 19.8 = 10 Scrubber slots). Green Consensus cut every Influence threshold 25% | The single best turn: Green Consensus made every remaining state 25% cheaper to buy. |
| 7 | Picked **Clean Manufacturing**; 2 more Factories South Asia; Inf -> Central America | **Took SE Asia**, Permafrost thawed, Antarctica opened | |
| 8 | Picked **Planetary Stewardship** (my gate); 2 Solar Arrays on the ISS; first Scrubber | **Took Central America**. Deep Mining landed: South Asia Factories went 6M -> 9M each | |
| 9 | Picked Expanded Habitats; ISS Shipyard; Scrubber; Embassy NA; **31 Inf -> East Asia** | **Planetary Stewardship complete on turn 9**; my gate was open with 27 turns to spare | |
| 10 | 3 Solar Arrays; 2 Scrubbers; Inf -> Europe + East Asia | **Took East Asia from the Prospectors** and **lost North America to them** in the same Resolution. Income 66M -> 141M/turn | The high point of the game. A straight swap that tripled my economy. |
| 11 | Efficient Transit; **Colony Ship** at the ISS; Habitat; 3 Scrubbers; mustered 4 Emigrants in Europe | Sea level +1.8 fired, took a Power Plant in Europe, a Factory + Power Plant in East Asia | |
| 12 | Hardened Hulls; 5 Scrubbers; Habitat; loaded 4 Colonists | Two Techs completed in one turn (pooled Research overflow) | |
| 13 | **Picked no Tech at all**; 4 Solar Arrays; 2 Scrubbers; unloaded 4 Colonists into the ISS | Nothing was auto-picked. The Tech Tree simply **froze**, and with it all three rivals' Victory gates | The moment I realised I had already won. |
| 14 | Colony Ship #2; 2 Solar Arrays; 2 Scrubbers | **Stabilization run 1/3** (sink 43 vs counted 40) | |
| 15 | Unloaded 4 (8 off Earth); loaded 6 | Stabilization 2/3 | |
| 16 | Unloaded 6 | **14 of 12 Colonists off Earth, run 3/3, gate DONE — win** | Anticlimactic: I could see the win coming three turns out and nothing could stop it. |

Final board, turn 16: me +1.90 C, net emissions **-10.56**, 15 Scrubbers, 141M/turn.
Rivals: Prospectors 135/750 in the Fund, Arkwrights 0/30 Colonists, Archivists 24/80 Research — **all three scored 0.00**.

## Game 2 — the Custodians from East Asia, same seed: Collapse on turn 21, nobody wins

Played to test the seating the release notes call broken, and to see the mid and late game I never
reached in game 1. Same strategy shape, different home.

| T | What happened |
|---|---|
| 1-8 | Slower start: East Asia has no Research Lab, Education 1.1 (Europe is 1.45), and an Allotment of 16 against Europe's 18. Took South Asia t4, South-East Asia t6, Central America t9. The Prospectors took **Europe** as their home and turned it into a furnace (Industry Level 3 -> 12 by turn 16). |
| 9-11 | Planetary Stewardship on turn 11 (two turns later than game 1). Prospector emissions already 38 ppm against my 9. |
| 12 | **The Prospectors took my ISS with Influence.** I had never spent a point on my own station, so my Standing there was 0 and its threshold was 20. I lost the Shipyard, a Habitat and five Solar Arrays in one Resolution, and my Energy income went from +13 to -37. The two Solar Arrays I had already paid for **completed for them** two turns later. |
| 13 | Emergency: mothballed two Research Labs and two Refineries, built Power Plants in the only free slots I had left. |
| 14-15 | Built a new station at Starlab (40M) and two Solar Arrays on it. Founded Mare Imbrium on the Moon with 4 Colonists. |
| 16 | Marched my East Asia Army into Russia: lost the battle 4 strength against 7, but the **DecisiveBattle destroyed a Refinery and two Factories** there. Ice Sheets Break: the sea ate my last Launch Site in East Asia, so I could no longer lift Emigrants. |
| 17-18 | **The Prospectors took Starlab too**, with 55 Influence against my whole Allotment of 21. By now they held every one of the five orbital slots over Earth. |
| 19-21 | South Asia fell to them (and **my nine Scrubbers there were destroyed with it**), then Sub-Saharan Africa, then Central America. Sink 23 -> 5. Amazon Dieback. |
| 21 | **Collapse. Nobody wins.** +3.03 C, 1656 ppm, emissions 165.65 of which the Prospectors were 160.92. |

Final: Custodians 0.00, Prospectors 0.67 (538/750 Fund, 8/12 off Earth, gate never researched),
Arkwrights 0.00, Archivists 0.00.

## Game 3 - the Custodians from North America, seed 20260912: Collapse on turn 22, nobody wins

A different seed, a different home, and a deliberately different approach: a real Moon colony
instead of Habitats on the ISS, to test whether a Colony's unlimited Modules plus Build Where You
Dig give the Custodians the same runaway another playtester found for the Prospectors.

On this seed Europe was neutral, the Prospectors were in East Asia again, the Arkwrights in the
Middle East and the Archivists in Australia.

| T | What happened |
|---|---|
| 1-4 | Took **Europe on turn 2** for 40 Influence because it was neutral, then South Asia on turn 4. Public Science, Deep Mining, Efficient Grids, Clean Power and Green Consensus all done by turn 5. |
| 6 | **Planetary Stewardship complete on turn 6** - my gate open a third of the way through the second sixth of the game. Two Techs completed in that Resolution and only one was announced (again). |
| 6-9 | Took South-East Asia and Sub-Saharan Africa. Five states, 24 Scrubber slots. |
| 10-11 | Flew a Colony Ship to the Moon and founded South Pole-Aitken Basin (Mine yield 1.95) with 6 Colonists. **Lost North America - my own home - to the Prospectors on turn 10.** |
| 12-16 | **The Module stack.** Two Mines at 20M each, and from then on every Module at that Colony cost 0.6 of the row: Mines at **12 Materials returning 11 Materials a turn**, Generators at **15 Materials returning 9 Energy a turn with no upkeep and no emissions**, no slot limit and no cap. Seven Mines and twelve Generators took my income from 42M a turn to **119M a turn** and made Energy - the Custodians' only real constraint - free. |
| 16-17 | Tested the other half of it: **sell Materials -> buy Influence at 2 Materials a point.** Selling 80 Materials a turn bought 40 extra Influence against a base Allotment of 22. The Moon stack can buy the Earth. |
| 18-21 | It was not enough. The Prospector AI had East Asia at **Industry Level 19** and was raising the Industry Level in all twelve states in the same turn. World counted Emissions passed 98 ppm, against a Sink I could only take to about 70 with every Scrubber slot I owned. They took Europe back, then South-East Asia, South Asia and Sub-Saharan Africa. |
| 21 | **They took my Moon colony by Influence**, with its 6 Colonists, 7 Mines and 12 Generators. My Off-world Presence went 6 -> 0 in one Resolution. No battle, no warning. |
| 22 | **Collapse. Nobody wins.** +3.16 C, 1747.9 ppm, emissions 171.69 of which the Prospectors were 167.69. |

Final: Custodians 0.00. **Prospectors 0.99** - 744 of 750 in the Fund and 24 of 12 Colonists off
Earth - and they still could not win, because The Extraction Charter was never researched.

## What held across the three games, and what did not

**Held on both seeds:**

- **The AI produces no Research and never picks a Tech.** Every completion in every game read
  "Prospectors 0, Arkwrights 0, Archivists 0". The Research Lead is a permanent one-seat monopoly,
  and from turn 13 of game 1 I stopped picking entirely and froze the tree with no penalty.
- **The Victory gates of ticket #84 are therefore a hard lock on three of the four seats.** Game 3's
  Prospectors finished on 0.99 of their own condition and could not win. In three games no rival
  ever scored above 0.00 except that one.
- **The Prospector AI is a climate engine, not a player.** It raises the Industry Level of every
  state it holds every turn - East Asia reached Industry Level 19 - and in both losses it was
  personally responsible for 160-168 of the world's 165-172 ppm. Both Collapses were its work.
- **Influence takes anything, and nothing can be defended by force.** Across games 2 and 3 the
  Prospector AI took, purely with Influence: five orbital stations including two of mine, my home
  Nation State twice, and in game 3 my Moon colony with six Colonists and nineteen Modules on it.
  It buys Influence with Ducats (80 points in one turn in game 3) and a player's flat Allotment
  cannot answer that.
- **The driver's Influence bar is unusable.** "your Influence 105/30" (game 2, Europe) and
  "your Influence 167/37" (game 3, East Asia) were both places I could not take. I wasted about
  270 Influence over the two games on hopeless pushes.
- **Energy, not Materials, is the Custodian brake** - until the Moon Module stack removes it.

**Did not hold:**

- **"The Custodian Victory Condition is easy" is only half true.** It is trivially easy if you get
  hold of a big dirty industrial state early - I took East Asia on turn 10 of game 1 and won on
  turn 16. If the Prospectors keep East Asia, world Emissions pass 50 ppm by turn 12 and 90 by turn
  18, and Stabilization becomes arithmetically impossible however many Scrubbers you own, because
  your Scrubber caps are fixed by the population of the states you hold. The outcome is decided
  around turn 10-12 and then it is a formality either way. Win on 16, or Collapse on 21-22.
- **The home state decides it.** Europe (won), East Asia (Collapse), North America (Collapse). A
  neutral Europe on seed 20260912 was a 40-Influence gift on turn 2 and still did not save the game.

## Addendum: the Custodian version of the Module-stack exploit

The Prospector finding (unlimited Modules plus Build Where You Dig) transfers to the Custodians
exactly, and is arguably worse for them, because it removes their only real constraint.

Measured in game 3 at South Pole-Aitken Basin on the Moon (Mine yield 1.95, Generator yield 1.25):

| Module | Price after two working Mines | What it returns each turn |
|---|---|---|
| Mine | **12 Materials** (20 x 0.6) | **11 Materials**, and +3 Energy upkeep |
| Generator | **15 Materials** (25 x 0.6) | **9 Energy**, no upkeep, no Emissions, no build slot |

A Mine pays for itself in one turn and a Generator in under two, there is no cap on how many a
Colony may hold, and nothing on the Moon emits. Seven Mines and twelve Generators took me from 42
to 119 Materials a turn. The second half of the loop is that Materials sell at 1 Ducat and
Influence costs 2, so **any surplus converts to Influence at 2 Materials a point** - in game 3 that
was 40 extra Influence a turn against a base Allotment of 22.

*Options:* (a) cap Modules per Colony, by Colonists or by Body (this is the cleanest, and gives
Habitats and Colonists a job they do not currently have); (b) make the in-situ discount apply to a
limited number of Modules - the first three at a Colony, say - rather than to every one forever;
(c) give Modules a rising Energy upkeep with the count at a Colony so a stack pays for itself.
*Trade-off:* (a) changes the feel of a Colony from "a pile of buildings" to "a place with a size",
which is probably the right fiction; (b) is the smallest change to the rule the version just added.

## Findings

### 1. Balance

**The Custodian Victory Condition, from Europe, is the easiest thing on the board.** Met on turn 16
of 36 with all three rivals on 0.00.

- **Planetary Stewardship on turn 9.** The chain is Public Science 15 -> Efficient Grids 15 ->
  Clean Power 25 -> Green Consensus 25 -> Planetary Stewardship 40 = 120 Research, and every one of
  those is a Tech a Custodian wants anyway. Two Research Labs in Europe (25M each, turn 1) paid it.
- **12 Colonists off Earth on turn 16, without flying anywhere.** Because a station over Earth
  counts as off Earth, the whole cost was a Shipyard (35M), two Habitats (50M), one Colony Ship
  (30M + 30 Fuel) and three turns of mustering four Emigrants in Europe. I never ordered a transit
  in the entire game. The Tank, the Launch Window, Venus, the Mass Driver, the Refuel and every
  Ship type but the Colony Ship were content I did not touch.
- **The Stabilization run started on turn 14** and never broke: counted Emissions 39.96 against a
  Sink of 43.00, then 40.5 against 49, then 40.5 against 55. Fifteen Scrubbers did it.

**The AI contributes no Research at all.** Every Tech completion in game 1 read
"Custodians N, Prospectors 0, Arkwrights 0, Archivists 0". I picked all twelve Techs. From turn 13
I stopped picking and the tree FROZE - Research pools, nothing is auto-picked - which locked all
three rivals out of their Victory gates permanently. With the gates of ticket #84, one seat's
Research Lead is a veto over three quarters of the game's Victory Conditions.

**The home state matters more than anything else on the card.** Same seed, same Faction: Europe
wins on turn 16; East Asia collapses the world on turn 21. Europe has Education 1.45 against East
Asia's 1.1 (a Lab is 6 Research instead of 4), Influence 5 against 4 (Allotment 18 against 16), and
Baseline Emissions 0.3 against 0.5 at Industry Level 3, which put my Blame share at 23% in Europe
and 32% in East Asia - a 7% tax on every Influence threshold from turn 1.

Numbers that felt wrong, with figures:

- **Leapfrog is priced about ten times too high.** 50 Ducats buys 0.03 off a state's per-person
  coefficient. In East Asia at turn 10 (pop 17.0, Industry Level 11, coefficient 0.37) taking it to
  the 0.04 floor is eleven Leapfrogs - 550 Ducats - for 2.10 ppm. A Scrubber is 30 Materials, which
  is 60 Ducats at the trading window, for 3.00 ppm. I bought zero Leapfrogs across two games.
- **Fuel -> Ducats -> Materials is a free 50% economy.** Fuel buys at 3 and sells at 1.5, Materials
  buy at 2. My Refineries made 7-16 Fuel a turn I had no use for; selling it and buying Materials
  was worth 5-12 extra Materials a turn from turn 4 on, and was the move that started my economy.
- **A Bank in a GDP-23 state pays 9 Ducats a turn for 25 Materials** - a three-turn payback, and
  strictly better than anything else in the slot. Only Europe and North America qualify, which makes
  them the only two states worth fighting for early.
- **Taking one state beats twenty turns of building.** Turn 10, East Asia changed hands and my
  Materials income went 66 -> 141 a turn. Nothing buildable compares.
- **The Scrubber cap is the Custodians real map.** clamp(round(pop/2), 2, 10) makes South Asia (10)
  and East Asia (9) the only two states that matter. Good rule - but the cap moves as heat kills
  population, and an order was refused with "this Nation State holds its 9 Scrubbers already" in a
  South Asia that had been a 10 two turns earlier.
- **Emissions have a floor no rule can touch.** baseline_emissions x Industry Level was 8.5 ppm of
  turn 1's 28.2 and only ever grows: Industry Level only goes up, and a spent Strip Permit raises
  the Baseline for good. Green Consensus halves people and Clean Power plus Clean Manufacturing cut
  buildings by 60%, but nothing touches this line. It is what makes a Prospector-heavy board
  unstabilizable rather than merely hard.

### 2. Engagement

Turns with a real decision: 1 (labs or land grab), 3-5 (the Energy squeeze, and finding that Ducats
are the economy), 6-8 (which state next, and Green Consensus cutting every threshold by a quarter at
exactly the right moment), 9-10 (the best turn of the game: gamble the whole Allotment on taking
East Asia off the Prospectors while they were three turns from taking North America off me - I lost
North America and did not care). Game 2's turns 13-16 were tense differently: losing the ISS put me
one turn from an Energy blackout and I had to triage by mothballing.

Autopilot: turns 11-16 of game 1. I had 141 Materials a turn and a Scrubber cost 30. The only
question was which of my four states got the next one, and the answer was always "the one the
Prospectors are not attacking". I could see the win three turns out.

Where it dragged: nowhere, because it ended on turn 16. That is the problem - thirty-six turns is a
lot to pace for, and playing to win I saw fewer than half of them.

### 3. Enjoyment

Satisfying: the Influence race. Persistent Standing, the 20-point challenge margin, and spending
stopping decay make it a real tug of war, and swapping North America for East Asia in the same
Resolution was the most memorable thing in either game. Green Consensus cutting every threshold by
25% felt like a proper unlock. The climate model reads clearly - "Last turn to act: 28" and "On this
course the world collapses on turn 33" told me exactly how much slack I had. In game 2 the sea
eating a coastal slot out of every state, and taking my last Launch Site with it, was the best drama
of the playtest.

Tedious: buying Materials with Ducats every single turn; placing the eleventh Scrubber; the rival
summary, which by turn 17 of game 2 was a 400-word paragraph hiding the one clause that mattered
("spent 55 Influence on Starlab over Earth").

Confusing: that my own station could be taken by Influence at all; that Expanded Habitats changes a
Colony Ship's capacity; that picking no Tech is allowed and does nothing; that a Scrubber cap
silently tracks a falling population.

### 4. Suggestions, most valuable first

**S1. The AI does no Research, so the Research Lead is a one-seat veto over three Victory
Conditions.**
Evidence: every completion read "Prospectors 0, Arkwrights 0, Archivists 0"; from turn 13 I picked
nothing and the tree froze; in game 2 the Prospectors held ten states and emitted 160 ppm and still
finished on 0.00 because The Extraction Charter was never researched.
Options: (a) give the AI a Research-Lab weight so it contests the Lead; (b) let a Faction always be
able to pick its own gate when it leads, or let a gate complete off its own Research outside the
shared tree; (c) let the Lead choose from a shortlist of two or three rather than the whole tree.
Also: refuse to end the turn when a pick is owed, or pass the pick to the next Faction.
Trade-off: (a) is the honest fix and slows everyone's tech; (c) keeps the leader's agency but kills
the freeze, which is currently the strongest single play in the game.

**S2. Off-world Presence costs nothing, so the space version can be won without going to space.**
Evidence: 12 Colonists off Earth for 115 Materials, 30 Fuel and zero transits; I never used the
Tank, a Launch Window, Venus, a Refuel or a Mass Driver in a winning game.
Options: (a) keep stations over Earth counting but raise the bar (12 -> 20+), or require the
Colonists to be spread over more than one Body as the Arkwrights condition already does; (b) narrow
ticket #85 so a station over Earth may hold the Archive but does not count for Off-world Presence.
Trade-off: (a) keeps the new rule and re-prices it; (b) is a partial revert but forces every seat to
actually fly, which is what the version is about.

**S3. Europe and East Asia are two different games.**
Evidence: win on turn 16 versus Collapse on turn 21, same seed, Faction and strategy.
Options: (a) narrow the home-card spread - Education 1.1 vs 1.45, Baseline 0.5 vs 0.3 and Influence
4 vs 5 all push the same way; (b) give a hot, dirty home a compensating start: a free Scrubber, a
Research Lab, or a Blame grace period for the first few turns.
Trade-off: (a) flattens the map's character; (b) keeps it and pays for it. The release notes flag
this seating as hot; from the inside it was not hot, it was lost by turn 13.

**S4. A Space Station cannot be defended.**
Evidence: game 2, the Prospectors took the ISS on turn 12 with my Standing at 0 against a threshold
of 20, and with it a Shipyard, a Habitat and five Solar Arrays; my Energy income went +13 to -37 in
one Resolution. They then took Tiangong, Axiom, Orbital Reef, and on turn 18 Starlab, which I had
built four turns earlier, with 55 Influence against my entire Allotment of 21. At the end they held
all five orbital slots over Earth.
Options: (a) give a station the same start Standing its owner gets on its home state; (b) let the
Modules standing on a station raise its threshold, so a built-up station is dear; (c) let a warship
holding Orbital Control block an Influence transfer.
Trade-off: (a) is one line and stops the ambush; (b) is more interesting and ties the rule to what
is actually at stake.

**S5. Leapfrog is dead content at 50 Ducats for 0.03.**
Options: (a) price it at 10-15 Ducats; (b) make one Leapfrog take the coefficient down by a whole
current Industry Level's worth rather than a flat 0.03, so it bites hardest where the emissions are;
(c) fold it into the Scrubber as a second effect.
Trade-off: a cheap Leapfrog makes holding a big dirty state better than mothballing it, which is the
Custodian fantasy - probably a good thing.

**S6. You cannot see how close a rival is to taking a place, or how close you are to taking theirs.**
Evidence: the headless driver's state card prints "your Influence 105/30" for a Europe I could not
take, because the printed figure is the raw threshold, not the holder's Standing plus the 20-point
margin. I spent 105 Influence over five turns on a hopeless push.
Options: (a) fix the driver to print influence_needed_for - the graphical game already does, at
src/ui.rs:1664; (b) in the real game, put a "contested, N turns" warning on a card you hold, since a
player also cannot see a rival closing on their own state.

**S7. Once the engine is running, the mid-game is arithmetic.**
Evidence: turns 11-16 of game 1 - 141 Materials a turn against a 30-Materials Scrubber, one
decision a turn, the same decision every turn.
Options: (a) cap how many builds one Faction may begin in a turn; (b) make Scrubber upkeep rise with
the number standing, so the twelfth costs more than the second and Energy stays the real constraint
it already nearly is; (c) let a runaway leader's Blame or Unrest bite harder.
Trade-off: (a) is crude but restores tension at once; (b) fits the fiction and keeps the one
decision that was already the best in the Custodian game - where does the Energy come from?

**S8. Nothing can lower baseline_emissions x Industry Level.**
Evidence: 8.5 of 28.2 ppm on turn 1 of game 1; by game 2's end six Prospector states at Industry
Level 7-12 made Stabilization arithmetically impossible however many Scrubbers I built.
Options: (a) let a Decommission lower a state's Industry Level; (b) let Leapfrog take the state's
Baseline down as well as its people's coefficient.
Trade-off: it makes a hostile world recoverable, which may be exactly what Collapse does not want.

**S9. Document what Expanded Habitats does to a Colony Ship.** Its card says "Each Habitat holds +2
Colonists"; it also adds +2 to a Colony Ship capacity (engine/src/state.rs:1056). The board line
went from "4 safe, 4 crowded" to "6 safe, 6 crowded" and nothing on the Tech said it would.

### 5. Bugs, and rules I could not find explained

1. **The headless driver prints the wrong Influence bar.** "play.exe show --save g2.ron" printed
   "Europe  Prospectors (seat 1)  pop 4.8 ind 12 unrest 1 | free slots 0/16 (coastal 0) | your
   Influence 105/30 | emigrants 4" - 105 against a printed bar of 30, and Europe still theirs.
   engine/examples/play.rs:469 passes influence_threshold_for; src/ui.rs:1664 correctly uses
   influence_needed_for, so the graphical game is right and the driver is not.

2. **Two Techs completed in one turn and only one was announced.** Game 1, turn 11 orders:
   "line 1: picked Efficient Transit". The turn 12 Report said "[TechComplete] Coastal Engineering
   is complete; every Faction has it." and nothing else, but the turn 12 "Techs done:" list held
   BOTH Efficient Transit and Coastal Engineering.

3. **"YOU MUST PICK THE NEXT TECH" is not enforced, and no rule says what happens if you do not.**
   I omitted the tech line from turn 13 of game 1 onward. Nothing was auto-picked, the Research
   pooled, and the tree stayed frozen for the rest of the game. That is the strongest move available
   to a Research leader and it is in no spec or CONTEXT entry I could find.

4. **You cannot set a stance for an Army you are moving this turn.** In one order file,
   "move-army 3 russia" was accepted and "army-stance russia attack" was
   "REFUSED: no Armies of yours there". The move attacked anyway ("[Army] Custodians Army moved from
   East Asia to Russia and attacks."), so the stance line was both refused and unnecessary - but it
   blocked the whole turn from ending until I deleted it.

5. **Modules you have paid for complete for whoever owns the station now, and the Report still calls
   them yours.** Game 2 turn 12: "[ControlChanged] ISS over Earth now belongs to the Prospectors".
   Game 2 turn 14: "[YourBuild] Custodians completed Solar Array at ISS over Earth", twice. At the
   end the ISS listed nine Solar Arrays, a Shipyard and a Habitat, all Prospector. No refund, no
   cancellation, and the line that told me was filed under [YourBuild].

6. **The Scrubber cap falls silently with population.** "build facility southasia scrubber" was
   "REFUSED: this Nation State holds its 9 Scrubbers already", in a state whose population had
   implied 10 two turns before. Nothing on the state card shows the cap or how many stand.

7. **Antarctica turns Emigrants away and nothing explains the room.** "12 Emigrants from Sub-Saharan
   Africa found no room in Antarctica and came home" (game 1, turn 15) happened to the Archivist AI
   four turns running. The rule is on no card I could read, and the AI cannot read it either.

8. **Shared scratchpad collision (harness, not the game).** Four playtest agents share one scratchpad
   directory. My t1.txt order file was overwritten by another agent between writing it and running
   it, and "play.exe turn" read their orders against my save. Every line was refused (they named
   states I did not hold) so nothing was lost, but a colliding LEGAL order file would have silently
   played someone else's turn. Order files need per-agent names.
