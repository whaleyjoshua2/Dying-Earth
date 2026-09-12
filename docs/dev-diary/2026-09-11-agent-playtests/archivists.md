# Archivists playtest — version 0.06.0

Three games, seat 0, headless driver (`target/release/examples/play.exe`).

| # | Seed | Start | Result |
|---|------|-------|--------|
| 1 | 20260911 | Sub-Saharan Africa | **Archivists win, turn 24** (Archive 80/80, 20 Colonists at Axiom) |
| 2 | 20260912 | (see below) | |
| 3 | 20260913 | (see below) | |

---

## Game 1 — seed 20260911, Sub-Saharan Africa — WIN on turn 24

### Turn log

- **t1** Picked Public Science. 2 Factories + 1 Solar Array on Axiom. All 11 Influence at North America. *Felt: the only real decision was "where does my Influence go", and I got it wrong.*
- **t2** Solar Array #2. Realised all three AI seats were dumping 15/15/5 Influence a turn on North America; I make 11 a turn total. Switched to South Asia. *Good moment — a genuine read of the board.*
- **t3** Mothballed the Launch Site (I would not need it for eight turns) to buy 2 Energy/turn. Started mustering 4 Emigrants a turn, which is free and only costs population. Solar Array #3.
- **t4** Research Lab in SSA (education 0.7 — 3 Research a turn). Realised **nobody else on the board was researching at all**: the whole world ran on 2 Research/turn from two neutral Labs.
- **t5** South Asia at 33/40. 18 Materials in the bank, cheapest useful building 20. **A completely dead turn.**
- **t6** Took South Asia. First Observatory on Axiom (5 Research, no Nation State needed, no build slot).
- **t7-t8** Observatory #2, Factory. Custodians led two Techs off the back of one Research Lab in North America.
- **t9** **Led Clean Power 13-6 and took the pick.** Chose Deep Mining (Factory output x1.5). Provisional Findings was already paying: Factories read 7 instead of 6 while it was under research.
- **t10** Took the Middle East. Income 49 M/turn. Picked Expanded Habitats. Shipyard on Axiom. Restarted the Launch Site.
- **t11** A Breakthrough card dropped 25 Research on Expanded Habitats and finished it — **credited to nobody**, so the pick went to a random seat. Started the Archive **on Axiom, the station over Earth**: no transit, no fuel, no colony to found.
- **t12** Colony Ship (30 M + 30 Fuel — I was 1 Fuel short and had to buy it with my only 3 Ducats).
- **t13** **The Upload complete on turn 13.** Gate open, 23 turns to spare.
- **t13-t14** Ordered `fund-archive` twice and banked **0 both times** (see Bug 1). 32 Research lost.
- **t15** **The turn everything went wrong.** The sea took my Launch Site; and a Prospector Frigate appeared over Earth, so `unload` into my own station was refused: *"Archivists could not land at Earth: the orbit is contested."*
- **t16-t18** Fund 7 → 41 → 75. Rebuilt the Launch Site, built Refineries for warship fuel.
- **t18** Tried to move the Archive to the Moon instead. **Refused: the Archive cannot be decommissioned.** The Archive is nailed to wherever you first built it.
- **t19** **Archive complete, 80/80, on turn 19.** Both Prospector Frigates flew to the Moon; Earth orbit clear — and my loaded ship was at the Moon.
- **t20** Ship back at Earth, orbit contested again. Loaded a second Colony Ship with 8.
- **t21** Frigates left; **6 Colonists landed at Axiom**. Mothballed 5 Observatories and 4 Refineries (Research and Materials were now worthless) to fix a -19 Energy/turn hole.
- **t22-t23** Blocked twice more. Attacked with a Battleship and 2 Frigates on t23 and killed a Colony Ship, but their Frigate had already left and **two freshly built warships blocked the landing in the same Resolution** (builds resolve before cargo).
- **t24** Last possible turn — collapse projected for t25. Sold 120 Materials for Energy, attacked, and ordered both unloads. Their whole fleet flew to the Moon that turn. **14 Colonists landed. 20 of 12. Win on turn 24, at +2.93 C with the Collapse Line at +3.0.**

---

## Game 2 — seed 20260912, Europe — WIN on turn 14

Same Faction, opposite start: Europe (Education 1.45, GDP 20, Influence 5) instead of Sub-Saharan
Africa (0.7, 2, 1). The plan was the one game 1 taught me: **deliver the Colonists first, while
Earth orbit is still empty**, and build the Archive around them.

- **t1** Picked Deep Mining, not Public Science — Provisional Findings paid a x1.25 on Factories from turn 1. 3 Factories. Income 4 → 24 M/turn in one turn.
- **t2** Shipyard on Axiom on turn 2. Bought Energy with Ducats (Europe pays 6 a turn; Sub-Saharan Africa paid 0 for ten turns).
- **t4** Colony Ship. **t5** Took North America — which came with a Research Lab making 6 Research a turn for me, a bigger research engine than anything I built by hand.
- **t5** Picked Expanded Habitats and learned that it raises **Colony Ship capacity** (4 → 5 under Provisional Findings), which its text does not say anywhere.
- **t6, t8, t10** Three ferry runs of 5. **15 Colonists at Axiom by turn 10** — before any rival had a warship over Earth.
- **t7** Picked The Upload; **complete on turn 10.**
- **t9** Archive ordered; **stands turn 12.**
- **t10-t14** Funded every turn: 6, 0, 15, 37, 22. **80/80 on turn 14. Win on turn 14**, at +2.18 C.

The 6 and the 0 are the funding bug (below). Without it this was a turn-12 win.

---

## Game 3 — seed 20260913, South Asia, Archive on the Moon — WIN on turn 26

Deliberately the hard version: a poor start (Education 0.75, GDP 4) **and** the ground route —
the Archive at a real Colony on the Moon rather than on my station over Earth.

- **t1-t8** 4 Factories, Shipyard, Solar Arrays. **t7: the Energy death spiral** — Energy hit 2, the shortfall shut down *the Solar Arrays on my own station*, so nothing could generate its way out. Escaped only by selling 40 Materials for 40 Ducats and buying 40 Energy.
- **t5, t11** Poured 60 Standing into North America and then 60 into East Asia and **lost both races to the AI**. ~120 Influence for nothing. The board never showed me a rival's Standing on a neutral state, so I had no way to know I was behind.
- **t11** Colony founded at Mare Tranquillitatis with 4 Colonists (a founded Colony comes with room for 8).
- **t12-t13** Mine first, then Archive: the Build-Where-You-Dig discount took the Archive from 50 to **37** and a Habitat from 25 to **18**. That rule is the nicest thing in 0.06.0.
- **t17** **Lost South Asia, my home state, to the Custodians** — with my Launch Site (already taken by the sea), my four Factories and **32 mustered Emigrants** on its card. Income fell from 37 M/turn to 10. With no Nation State I had no Launch Site, and with no Launch Site the last 4 Colonists could not leave Earth at all.
- **t19-t22** Fund 4 → 40 → 76 → 80. **Archive complete on turn 22** with no Earth economy at all: six Observatories on Axiom made 36 Research a turn on their own.
- **t23** Sold my entire Materials stock (79) for Ducats, bought 44 Influence, and took **Europe in a single turn** (40 + 54 Standing against a bar of 64) — for its Launch Site and its 17 Emigrants.
- **t24-t26** Load 5, fly, unload. **13 of 12 Colonists. Win on turn 26**, at +2.75 C.

Final: Archivists 1.00 MET, Arkwrights 0.67, Prospectors 0.00 (252/750), Custodians 0.00.

---

# Findings

## 1. The story, in one line each

Three games, three Archivist wins, on turns **24, 14 and 26** of 36. The Archive is not hard to
finish. What is hard - and what nearly cost me game 1 - is getting twelve people into it.

## 2. Balance

| | fund at t12 | Archive Module stands | fund full (80) | 12 Colonists in place | won |
|---|---|---|---|---|---|
| G1 Sub-Saharan Africa | 0 | t19 | **t19** | t24 | t24 |
| G2 Europe | 21 | t12 | **t14** | t10 | t14 |
| G3 South Asia, Moon | 0 | t17 | **t22** | t26 | t26 |

- **The pre-Archive cap of 20 never once bound.** It is meant to force a decision and it cannot,
  because the Archive Module is cheap (50 Materials, 37 with a Mine) and can be ordered by turn 9.
  In all three games the Module stood before I had banked 20, so the cap never refused a single
  order. As written it is a dead rule.
- **The Research bar (80) is far too low for what an Archivist engine produces.** Six Observatories
  on a station cost 168 Materials and make 36 Research a turn: the whole 80 in **three turns**. In
  game 3 I paid the entire Archive off from a station while holding no Nation State on Earth at all.
  I would put the bar at 160-200, or make the Observatory dearer per point than a Research Lab.
- **The gate Tech is free.** Nobody else researches. The world runs on 2 Research/turn from two
  neutral Labs; one Research Lab makes you Research Lead forever. I led Clean Power 13-6 in game 1
  and then picked every Tech to the end. The Upload landed on t13, t10 and t18 - always before the
  Archive stood, so it never gated anything.
- **No rival was ever close.** Final Prospector Fund: 39, 252, 252 of 750. Final Custodian
  Stabilization run: 0/3 in all three (net emissions +42, +60, +21 against a Sink of 4-16). Only
  the Arkwrights looked alive (48/30 Colonists in game 3), held back by their gate.
- **Twelve Colonists is the whole difficulty**, and it is difficulty of the wrong kind - see 5.1.

**On the coordinator question - the Prospectors winning on t15/t16 on seed 20260911.** I never saw
it: their Fund ended at 39, 252 and 252. But the answer to "how long do I need the game to last" is
clear from my three. The two parts of the Archive were finished on **t19, t14 and t22**; everything
after that was the lift. An Archivist needs the game to reach roughly **turn 15 at the very best and
turn 26 at worst**. If the Prospectors are winning on turn 15, an Archivist cannot finish in a game
they play well, let alone one they play badly. The fix is not to make the Archive faster: a turn-15
win is too early for anybody, since at that point no other seat has even picked its gate Tech.

## 3. Engagement

**Turns with a real decision:** G1 t2 and t6 (reading that three AI seats were outbidding me on
North America, and switching target); G1 t9 (spending the Research Lead on Deep Mining rather than
rushing the gate); G1 t15-t18 (the Archive is nailed to a station I cannot reach - fight, move, or
wait?); G3 t23 (sell the entire Stockpile to buy a country in one turn).

**Autopilot:** almost everything else.
- G1 **t5 was literally empty**: 18 Materials in hand, cheapest useful building 20. Nothing to do
  but press End Turn.
- G1 **t16-t22, seven turns**, were the same five lines: fund, unload (refused), influence, build
  the next Observatory. I was not deciding, I was waiting on a die roll about somebody else fleet
  movement.
- G2 **t10-t14** were one line, `fund-archive`, five times.

**Fund versus Provisional Findings was never a decision.** It cannot be, in this shape:
- Before the Archive stands the cap is 20, so there is almost nothing to fund.
- After the gate Tech is done there is no Tech I want, so Provisional Findings is worth nothing and
  I fund every turn to the end.
- The window where both sides have value is the few turns between "Archive standing" and "The
  Upload complete" - and in all three games The Upload finished first (t13 vs t19, t10 vs t12,
  t18 vs t17).

Provisional Findings itself is lovely when it fires - Factories reading 7 instead of 6 on turn 2 of
game 2, a Colony Ship carrying 5 instead of 4 - but it was free money, never a price.

## 4. Enjoyment

**Satisfying:** Build-Where-You-Dig (watching the Archive drop from 50 to 37 because I laid a Mine
first is the best "the rules reward planning" moment in the build); the Research Lead chain in game
1; the turn-23 fire sale in game 3, selling every Material I owned to buy a continent in a single
turn; and the last turn of game 1 - +2.93 C against a Collapse Line of +3.0, ordering an unload that
had failed four turns running, and watching the enemy fleet leave.

**Tedious:** the seven-turn wait in game 1; funding five turns in a row in game 2; and the Emigrant
ferry, which is four orders and three turns of calendar for five people.

**Confusing:** `fund-archive` reporting "0 Research banked" with no reason, twice running;
discovering only by refusal that the Archive can never be moved; discovering only by refusal that I
could not put Colonists into a Space Station I own because a rival had a Frigate in the same orbit;
and Colony Ship capacity changing when I picked a Habitat Tech.

## 5. Suggestions, ranked

### 5.1 Orbital Control locks an Archivist out of their own Victory Condition, with no counterplay

**Observed:** game 1, turns 15-23. One Prospector Frigate over Earth made `unload ... into colony 13`
refuse - into my own station, which I had held since turn 1, over the planet the Colonists were
standing on. Seven turns blocked. On t23 I attacked with a Battleship and two Frigates, **won** the
battle (the report says "The Archivists hold Orbital Control") and still could not land, because two
Prospector warships finished building at Tiangong in the same Resolution and builds resolve before
cargo. I won on t24 only because their whole fleet happened to fly to the Moon that turn. And the
Archive cannot be moved: `change module 13 8 decommission` is refused outright.

**Options:** (a) let a Faction always land at a Colony or Station it controls, whoever holds the
orbit - treat its own dock as friendly ground; (b) keep the block but let the Archive be
decommissioned and rebuilt (half the Materials back, fund kept), so a blockade is expensive rather
than terminal; (c) resolve cargo before builds, so at least winning the battle wins the turn.

**Trade-off:** (a) removes a real use for warships over Earth and makes blockade toothless; (b)
keeps the tension and gives the player an out, at about 4 turns and 50 Materials; (c) is an ordering
change that fixes the most galling case without touching the rule. I would do (c) and (b).

### 5.2 Fund the Archive silently banks less than the turn Research

**Observed six times across three games:**
- G1 t13 `The Archivists are funding the Archive: 0 Research banked, 0 of 20 in the fund.` (16 made)
- G1 t14 `0 Research banked, 0 of 80 in the fund.` (22 made)
- G1 t15 `7 Research banked` (22 made)
- G2 t10 `6 Research banked` (14 made); G2 t11 nothing banked, fund stayed at 6
- G3 t18 `4 Research banked` (36 made); G3 t19 `0 Research banked`

Lost: about 32 Research in game 1 - 40% of the whole Archive - about 30 in game 2, about 72 in game 3.

**Why, from the code:** fund_archive takes the Research back out of research.contributions[seat]
when a Tech is under research and out of research.unallocated when none is. On the turn after a
Tech completes there is no Tech under research at Income, so the Research lands in unallocated -
and unallocated is then swept into the next Tech by pick_tech, which the driver applies the
moment it parses the tech line, before any order runs, or by whichever AI seat holds the pick.
Either way it is gone before Fund the Archive looks for it. Separately, accrue_research calls
check_tech_complete at Income, so a big Research turn that finishes the Tech leaves nothing to
take back at all.

**Options:** (a) have fund_archive bank research_last_turn outright and subtract it from wherever
it can, including a completed Tech progress, so the order always does what its name says; (b) make
funding a declaration set before Income, like the Prospectors Venture share, so the Research never
reaches the shared Tech; (c) at minimum, say why: "0 banked - this turn Research completed Clean
Manufacturing and cannot be recalled."

**Trade-off:** (b) is cleanest and matches the Prospectors, but it is a bigger change and removes the
"see what I made, then decide" feel that is the point of the order. (a) is smallest. (c) is not a fix
but would have saved me two wasted turns. The workaround I eventually found - keep an expensive Tech
under research that your own withdrawals keep emptying - is invisible to a player and feels like
cheating.

### 5.3 The Archive on a station over Earth is far cheaper than on a ground Colony, so the ground route is a trap

**Observed:** games 1 and 2 put the Archive on Axiom - 165 Materials all in (Shipyard 35, Colony Ship
30 M + 30 Fuel, two Habitats 50, Archive 50), zero transit Fuel, and a two-turn load/unload cycle.
Game 3 put it on the Moon: all of that plus a Colony to found, a Mine, Habitats, and a **four-turn**
delivery cycle, with my one ship down to tank 6/30 by turn 19, one leg from stranded. Game 2 won on
t14; game 3 on t26.

**Options:** (a) leave it - the station is the reward for starting with one, and the Moon is the
greedy route (Mine income, in-situ discount, Mass Driver); (b) require the Archive Colony to be on a
Body with ground, closing the station route; (c) keep both, but make the station route pay: no Mine
on a station means no Build-Where-You-Dig, so charge the Archive full price there and demand more
Energy.

**Trade-off:** (b) restores the "get off Earth properly" fantasy but makes the Archivists much the
weakest seat and reintroduces the blockade problem at every Body. I would keep the station route and
raise the Research bar instead.

### 5.4 The start state decides the Archivists game more than the Archivists do

**Observed:** same Faction, same rules, three starts.
- **Europe** (Education 1.45, GDP 20, Influence 5): 15 Influence/turn, 6 Ducats from turn 1 - won **turn 14**.
- **Sub-Saharan Africa** (0.7, GDP 2, Influence 1): 11 Influence/turn and **literally zero Ducats a
  turn for the first ten turns** (GDP 2 x Industry 1 / 10 rounds to 0), so the whole Trading window
  was shut to me. Won turn 24.
- **South Asia** (0.75, GDP 4, Influence 2): also 0 Ducats; lost its home state on t17; won turn 26
  only by an emergency fire sale.

Education multiplies every Research Lab and GDP decides whether you can buy anything at all - and the
Archivists are the Faction that lives on Research.

**Options:** (a) give the Ducat formula a floor, so a held state always pays at least 1-2 and the
Trading window is never fully shut; (b) give the Archivists a Research floor independent of
Education; (c) leave it, but say on the start-state picker what Education Level does.

**Trade-off:** (a) is a small number with a large effect on how many openings feel playable, and costs
nothing thematically - every country has some economy. (c) is free and honest but does not change
that the poor starts are simply worse.

### 5.5 Influence against the AI is a race you cannot see and usually lose

**Observed:** G1 t1-t5, I spent 11 a turn on North America while the Custodians spent 15, the
Prospectors 15 and the Arkwrights 5; the lot fell to the Custodians on t5 and my points were gone.
G3 was worse: 60 Standing into North America, lost; 60 into East Asia, lost; about 120 Influence
wasted out of a 12/turn allotment. **The board shows my Standing and the threshold but never a rival
Standing on a neutral state**, so there is no way to tell whether you are 5 points behind or 40 until
the state changes hands.

**Options:** (a) show each claimant Standing on a contested neutral place, or at least "you are
second of three"; (b) refund or protect Standing on a place that goes to someone else, so a lost race
is a delay rather than a total loss; (c) leave it as fog of war.

**Trade-off:** (a) is pure information and turns the Influence phase into a real auction instead of a
lottery; it does make the AI easier to out-bid. I would do (a).

### 5.6 The Energy shortfall can become a death spiral you cannot generate out of

**Observed:** G3 t7. Energy hit 2 and the shortfall took **the Solar Arrays on my own station**
offline along with everything else, so nothing could generate back up:
`modules: 0:Solar Array(offline), 1:Shipyard(offline), 2:Solar Array(offline), 3:Observatory(offline)`.
I escaped only by selling 40 Materials for 40 Ducats and buying 40 Energy - and from a GDP-2 start I
would have had no Ducats to do it with.

**Options:** (a) never shut down a producer that makes Energy - take everything else first; (b) let a
shortfall idle buildings for one turn rather than latching them offline.

**Trade-off:** (a) is a two-line rule and removes an unrecoverable state. It makes Energy slightly
less frightening, which I think is right: the frightening version is not a decision, it is a loss you
cannot see coming.

### 5.7 The Collapse clock swings too far to plan against

**Observed:** game 1 projected collapse turn, read off the Climate panel turn by turn: 34, 30, 28,
27, 25, 26, 24, 22, 23, 25. Game 3 went from "turn 23" to "not reached on this course" within four
turns once the Custodians got going. I built my whole late game around "finish by t22" and won on t24
with two turns I had not known I had.

**Options:** (a) show a range rather than a single turn; (b) show whose emissions drive it - the
panel already knows (game 1 Prospectors were 26 of 61 ppm); (c) damp the projection so it moves at
most a turn or two per turn.

**Trade-off:** (b) is the most interesting - it turns the climate readout from a doom counter into
intelligence about my rivals, which is what it actually is.

### 5.8 Nobody but you researches, so the Research Lead is not a race

**Observed:** the world entire Research output for the first four turns of game 1 was 2 a turn from
two neutral Labs. One Research Lab in a 0.7-Education country took me the Lead (13 against 6) and I
then picked every Tech to the end of the game, including my own Victory gate.

**Options:** (a) give the AI Factions a reason to build Research Labs; (b) make a gate Tech
unpickable by its own Faction, so an Archivist must trade or wait for it.

**Trade-off:** (b) is a sharp, cheap way to make The Upload a real gate, but it hands the Archivists
timetable to three AI seats that currently never pick a rival gate at all (ai.toml lists none in
any order list), so (a) has to come first.

## 6. Bugs and rules I could not find explained

**a. Fund the Archive banks less than the turn Research, silently.** Six occurrences and the cause in
5.2. Exact output, game 1 turn 14:

```
$ play.exe turn --save archivists.ron --orders t14.txt
line 1: ok `fund-archive` costs free
line 2: picked Clean Manufacturing
...
  [YourWorks] The Archivists are funding the Archive: 0 Research banked, 0 of 80 in the fund.
```

I produced 22 Research that turn and the fund had room for 80.

**b. A newly built warship blocks a landing in the same Resolution, after you have already won the
battle for the orbit.** Game 1 turn 23:

```
  [Ship] Archivists could not land at Earth: the orbit is contested.
  [Battle] Earth orbit: Archivists, attacking, (Battleship, Colony Ship, Colony Ship, Frigate,
  Frigate, strength 13, 3 hit(s) landed; destroyed: none; escaped: none) against Prospectors,
  (Colony Ship, strength 0, 0 hit(s) landed; destroyed: Prospectors Colony Ship; escaped: none).
  1 round(s). The Archivists hold Orbital Control.
```

The battle line says I hold Orbital Control and the cargo line says the orbit is contested, in the
same turn. resolve_builds runs before resolve_cargo, and the same report carries
"Prospectors completed Battleship at Tiangong over Earth".

**c. You cannot unload Colonists into a Space Station you own if any rival warship is in that orbit**
- including the orbit of the planet the Colonists are standing on. Not in the CONTEXT.md entries for
Space Station or Orbital Control, nor in PLAYTEST.txt. It is the single most important rule for an
Archivist and I found it by having it happen to me on turn 15:

```
  [Ship] Archivists could not land at Earth: the orbit is contested.
```

**d. The Archive cannot be mothballed or decommissioned, ever.**

```
$ play.exe check --save archivists.ron --orders t18.txt
line 3: REFUSED `change module 13 8 decommission`: the Archive is raised and lost by its own
rules; it is not mothballed
```

CONTEXT.md says the Archive is destroyed if its Colony changes hands; it does not say the owner can
never take it down. With (c), this means the one irreversible Archivist decision is which Colony gets
the Archive, made around turn 9, with no way to know whether that orbit will be contested on turn 20.

**e. Expanded Habitats changes Colony Ship capacity**, which its text does not say. Game 2 turn 5,
the same order file, with and without the pick applied:

```
line 2: REFUSED `load 17 6 from europe`: this Ship carries at most 5 Colonists    (tech picked)
line 2: REFUSED `load 17 5 from europe`: this Ship carries at most 4 Colonists    (not picked)
```

The Tech reads "Each Habitat holds +2 Colonists"; nothing in techs.toml, CONTEXT.md or PLAYTEST.txt
mentions Ships.

**f. Research made on a turn when no Tech is under research loses its owner.** It goes to
research.unallocated and is folded into the next Tech with no seat attributed. Game 1 turn 11, after
a Breakthrough card finished Expanded Habitats:

```
  [TechComplete] Expanded Habitats is complete; every Faction has it. The Prospectors led
  (Archivists 0, Custodians 0, Prospectors 0, Arkwrights 0) and pick the next Tech.
```

An all-zero Lead line reads like a bug even if it is intended.

**g. The driver "not one of" error lists only Nation States**, though Colony ids are legal places:

```
line 1: REFUSED `influence axiom 5`: "axiom" is not one of: subsaharanafrica, northafrica, ...
```

`influence 13 5` works. Minor, but it cost me a turn of not knowing I could defend my own station.

**h. Load and unload cannot both happen in one turn** - both resolve in resolve_cargo, and the check
sees the pre-unload cargo - so a ferry is two turns per trip and an emptying ship cannot be refilled
the turn it empties:

```
line 4: REFUSED `load 19 6 from subsaharanafrica`: this Ship carries at most 7 Colonists
```

said while carrying 6 and ordered to unload all 6 the same turn. The message does not explain why.

## 7. What held across seeds, and what did not

**Held in all three games:**
- The Archive itself is easy: Module standing by t12-t19, 80 Research paid by t14-t22.
- The funding bug fired in every game, costing 30-72 Research each time.
- Nobody but me researched; I took the Research Lead with one Lab and never lost it.
- Fund versus Provisional Findings never became a decision, because The Upload always finished
  before the Archive stood.
- The Prospectors and Custodians were never close (Fund 39/252/252 of 750; Stabilization 0/3 every time).
- The twelve Colonists, not the Research, were the long pole in every game.

**Did not hold - one draw of luck:**
- The Earth-orbit blockade only bit in game 1, where the Prospectors happened to keep Frigates over
  Earth from t15. In games 2 and 3 orbit was empty when I needed it. That is the difference between
  winning on t14 and nearly losing on t24, and it was not something I chose.
- Losing my home Nation State happened only in game 3, and only because the sea took the Launch Site
  first. It was very nearly fatal and I would not have recovered without the Materials-to-Influence
  fire sale.
- The collapse clock read t22-t25 for most of game 1 and I played to that deadline; in game 3 it
  stopped threatening entirely by t18. Same rules, wildly different pressure.
