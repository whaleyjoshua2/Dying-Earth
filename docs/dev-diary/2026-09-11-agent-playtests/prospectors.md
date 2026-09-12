# Playtest: the Prospectors, seed 20260911, North America

Version 0.06.0, headless driver (`target/release/examples/play.exe`), 36 turns available.

## GAME 1 — result: PROSPECTORS WIN ON TURN 15 OF 36

Venture Capital Fund 1119/750, 12/12 Colonists off Earth, The Extraction Charter done.
Temperature at the win +2.17 C, projected Collapse turn 27.

### Turn-by-turn log

| T | What I did | What happened | Felt |
|---|---|---|---|
| 1 | Tech Deep Mining. Industry raise + Factory + Lab + Power Plant in North America. Muster 4 Emigrants. 17 Influence -> Europe. | Board opened with me holding the next Tech pick. | Real decisions: which rung-1 Tech, how to split 80 Materials. Good opening. |
| 2 | Shipyard on Tiangong (bought 8 Materials with Ducats). Pivoted Influence off Europe to Central America — all three rivals were dogpiling Europe. | — | The three-way Influence dogpile on Europe was the most interesting read of the game. |
| 3 | Tech Efficient Grids. Factory in NA. Influence Central America. | Deep Mining done, I led 8-0-0-4. | — |
| 4 | Bought Energy, Factory in Central America, Industry raise. | **Took Central America.** Its materials-lean Factory makes 11/turn against North America's 7. | The lean multiplier is a big, legible decision. |
| 5 | Tech Automated Refining. Colony Ship at Tiangong. | Coral Die-off Break. | — |
| 6 | Loaded 4 Colonists. Two more Factories in Central America. | — | Annoyance: you cannot `load` and `transit` the same ship in one turn. Costs a turn per trip for no reason I could find in the spec. |
| 7 | Transit to the Moon. Bank + Lab in NA, Industry raise, Factory in CA. | Collapse projection fell to turn 26. | First "am I going to run out of world?" moment. |
| 8 | Unloaded 4 into Mare Imbrium (Moon). **Sent 4 Emigrants to Antarctica by sea — free, no ship, no launch.** Two Factories in South America. | Took South America. Permafrost Thaw Break; **Antarctica opened at +1.61 C**. | Free Antarctic colony felt like a loophole. |
| 9 | Tech The Extraction Charter. 3 Mines + 1 Generator on the Moon. Lab in NA. | Moon Colony founded, Antarctic Colony founded. All three rivals began spending Influence on my home, North America. | — |
| 10 | 4 Generators + 4 Mines on the Moon (Mines now 10 Materials each, Generators 12 — Build Where You Dig). Habitat on Tiangong, 2nd Colony Ship. Mothballed both Refineries. **Set Venture share to 40%.** | — | The moment the game broke open: a Moon Mine costs 10 Materials and pays 13/turn. One-turn payback. |
| 11 | 6 Mines + 4 Generators. Loaded 4+4 Colonists. | +1.8 C sea-level threshold took 2 of my 6 Central American Factories and slots all over the world. | The sea taking my best state hurt, and was the only genuine setback of the game. |
| 12 | 10 Mines + 5 Generators. Unloaded 8 Colonists into Tiangong (stations over Earth count as off Earth). **Share to 80%.** | Fund 175 -> 477. | From here it was arithmetic, not a game. |
| 13 | 2 Mines. | **Moonquake**: every Moon Module offline, one whole turn of extraction lost. **The Extraction Charter completed.** 12/12 Colonists off Earth. | The one event that actually mattered. It also silently removed the Build Where You Dig discount, so my Mines cost 17 instead of 10 that turn. |
| 14 | 9 Mines. | Fund 621. | Autopilot. |
| 15 | 4 Mines. | **Fund 1119. WIN.** | Autopilot. |

### The engine, in numbers

Turn 15 income: 125 Materials to the Stockpile **after** 498 had been skimmed into the Fund —
623 Materials of extraction in a turn, 34 Moon Mines at 16 each (546) plus 7 Earth Factories.

| | Cost | Output/turn | Payback |
|---|---|---|---|
| Moon Mine (3rd onward, Build Where You Dig) | 10 Materials | 16 Materials | 1 turn |
| Moon Generator (ditto) | 12 Materials | 10 Energy | ~1.5 turns in bought-Energy terms |
| Central America Factory (materials lean) | 17 Materials | 11 Materials | 1.5 turns |
| North America Factory (no lean) | 17 Materials | 7 Materials | 2.5 turns |
| Bank in North America | 21 Materials | 11 Ducats (~13 Energy) | ~2 turns |

Fund at the checkpoints the brief asked for: **turn 12: 175. Turn 15: 1119 (win). Turns 24 and 36 never happened.**

## GAME 2 — same seed, started in East Asia (the seating the notes say runs hottest)

Played deliberately the way the Prospector card invites: Earth industry, a Strip Permit,
Antarctica, expansion. Result: **PROSPECTORS WIN ON TURN 16 OF 36**, Fund 953, at +2.59 C
with Collapse projected for turn 21.

Highlights:
- T5 **Strip Permit on East Asia** turned 42 Materials a turn into 121. Three turns of doubled
  output for +3 permanent Unrest. Best single button in the game.
- T8 Antarctica opened at +1.64 C and I founded Lake Vostok for free (`send-antarctica`, no ship,
  no launch). By T12 it held 14 Mines, 18 Generators and 3 Refineries — a Colony has NO module
  slot limit.
- T10 collapse projection: turn 20. T13: turn 18. My own Emissions were 71 of the world's 107.
  **Antarctic Generators are the hidden culprit**: a Module in Antarctica emits like its Facility,
  so 18 Generators emitted 1.5 x 1.25 = 33.75 ppm a turn, more than every Factory I owned. Nothing
  on the build button or in the Colony view says so.
- T13 **I lost East Asia, my home state**, while spending my whole Influence allotment on it every
  turn from T10. I had no way to see how close the challenger was.
- T13 Fund 730. T14 Fund 953 — the Fund was never the problem.
- T14 I mothballed all 35 Lake Vostok modules and all 6 Moon Mines in one turn (free, no Unrest in
  a Colony). My Emissions fell 71 -> 14 and Collapse moved from turn 18 to turn 22. One turn of
  orders bought four turns of world.
- T13-T16 the whole game was a scramble to get 8 more people off Earth: I had lost my only Launch
  Site with East Asia and had to build another (2 turns) and muster 4 Emigrants a turn.
  **The binding constraint on the Prospectors' Victory Condition is not 750 Materials.
  It is twelve Colonists.**

Fund at the checkpoints: **turn 12: 413. Turn 14: 953. Turn 16: 953 (win).**

---

# FINDINGS

## 1. Balance: is 750 reachable?

**Yes, and it is not close.** 750 was never the hard part of the Prospectors' Victory Condition.

| | Game 1 (North America) | Game 2 (East Asia) |
|---|---|---|
| Fund, turn 12 | 175 | 413 |
| Fund, turn 14 | 621 | 953 |
| Fund, turn 15 | **1119 - WIN** | - |
| Fund, turn 16 | - | **953 - WIN** |
| Fund, turns 24 / 36 | game over on turn 15 | game over on turn 16 |
| Peak extraction in one turn | 623 Materials | ~400 Materials |
| Most banked in one turn | 498 | 317 |

The AI's 450 ceiling measures the AI, not the bar. Two things a human does that the AI does not:

**(a) A Colony has no module slot limit, so extraction is unbounded.** By turn 15 of game 1 I had
34 Mines and 18 Generators in one Colony at Mare Imbrium. A Nation State has Size + Industry + 3
build slots and the sea eats them; a Colony has none at all.

**(b) Build Where You Dig makes the third Mine cheaper than the first.** With two working Mines a
Mine costs 20 x 0.85 x 0.6 = **10 Materials** and pays 16 Materials a turn. A one-turn payback,
repeatable without limit. A Moon Generator on the same discount is 12 Materials for 10 Energy, so
a package of 3 Mines + 1 Generator costs 42 Materials, is Energy-neutral, and pays 48 Materials a
turn forever.

The turn-15 income line of game 1, exactly: `Last income: 125M 0F 14E 30D`, with
`Venture Capital Fund (banked) -498M` at the end of the source list. 623 Materials of extraction in
one turn, against a 750 LIFETIME bar.

**By roughly how much is it wrong?** If the Fund is meant to be the Prospectors' whole game, the
bar wants to be near **2500-3500**, or the 80% share cap wants to be far lower, or Mines want to
stop scaling. As it stands I passed 750 on turn 15 in one game and turn 14 in the other, from
opposite corners of the map.

**No rival ran away with it.** At my turn-15 win: Custodians 0/3 Stabilization, Arkwrights 0/30
Colonists, Archivists 36/80 Archive - and not one of the three had even picked its gate Tech. In
game 2 at turn 16 the Custodians had 12/12 Colonists but still 0/3 Stabilization. **The four
Victory Conditions are not on the same clock.**

## 2. The real bar is twelve Colonists, and the Fund share is not a real choice

The share was an obvious decision, not an interesting one. The correct play is **0% until building
stops paying, then 80%**, because a Mine banked at 80% is worth less than a Mine that buys another
Mine. I ran 0% to turn 10, 40% to turn 12, 80% after, and every 80% turn was arithmetic - nothing
to decide, just watch it fill. Game 2 proves it from the other side: a lazy 30% from turn 7 still
reached 413 by turn 12.

The share would become a real choice if banking carried real pressure: a share that must be set for
several turns at a time, a draw that costs much more than a tenth, or a bar high enough that 80%
from turn 6 is actually necessary.

Meanwhile the other half of the condition - 12 Colonists off Earth - took from turn 6 to turn 13 in
game 1 and nearly lost me game 2. Its pipeline is long and unforgiving: Shipyard (2 turns) ->
Colony Ship (1 turn, 30 Materials + 30 Fuel) -> muster Emigrants (4 a turn) -> load (1 turn) ->
transit (1 turn) -> unload (1 turn). **The two halves of the Prospectors' condition are wildly
mismatched in difficulty.**

## 3. Engagement, turn by turn

- **Turns 1-9: genuinely good.** Every turn had a real decision. Which rung-1 Tech. Whether to
  fight three rivals for Europe or take cheap, materials-lean Central America instead (I switched
  on turn 2 and it was the best call of the game). Whether a materials-lean Factory at 11 a turn
  beats a Bank at 11 Ducats. How much Energy to buy against how many Power Plants to build.
  Whether to spend the Influence allotment expanding or defending home.
- **Turn 10 is where the game breaks open and stops being a game.** The moment the first Moon
  Colony has two working Mines, a Mine costs 10 and the answer to "what do I build" is "another
  Mine" for the rest of the game. Turns 10-15 of game 1 were: build mines, buy energy, top up home
  Influence, end turn. Five turns of autopilot - and they were the five turns that won it.
- **The tense moments were all on Earth and none were about the Fund:** the +1.8 C sea level on
  turn 11 taking two of my six Central American Factories; three rivals starting to spend on my
  home state on turn 9; the Moonquake on turn 13 wiping a whole turn of extraction; watching
  "collapses on turn 24" while needing turn 23.
- **The climate line is the best thing in the interface.** "Last turn to act: 22 / On this course
  the world collapses on turn 26" reframed every build. It is the only number that ever made me
  hesitate.

## 4. Enjoyment

**Satisfying:** the materials-lean multiplier - the same Factory makes 11 in Central America and 7
in North America, which is legible and immediately changes where you point your Influence. The
Strip Permit: free, enormous, with a permanent price, and in game 2 the price came due and cost me
my home state. Build Where You Dig as a FEEL - the third mine really should be cheaper than the
first. Working out that Bank Ducats buy Energy at 0.85 each, so Banks are emission-free power.

**Tedious:** writing `build module 19 mine` ten times in an order file. Losing a whole turn because
`load` and `transit` cannot be given to one Ship in one turn. Energy bookkeeping - I hand-totalled
upkeep against generation every single turn to decide how much Energy to buy, and the board shows
only last turn's delta, never next turn's.

**Confusing:** nothing tells you a Module in Antarctica emits like its Facility (I built 18
Generators at Lake Vostok and doubled my own Emissions with no warning). Nothing tells you how
close a rival is to taking your state. And `unload 17 4 into slot moon 1` was reported as "founded
a Colony in slot 2 on the Moon".

## 5. Suggestions, ranked

**1. The Fund bar is far too low for the engine behind it - but raising it alone will not fix the
Faction.** Evidence: 1119 by turn 15, 953 by turn 14, from two different starts.
Option A: raise the bar to ~3000 and change nothing else. Trade-off: the win moves to about turn
25-28, which is right, but turns 10-30 become the same "build another Mine" turn twenty times - it
lengthens the autopilot instead of removing it.
Option B: cap what the Fund can take in a turn (the share applies only to the first N Materials of
extraction, or takes a share of Materials NET of upkeep). Trade-off: another number on an already
dense card.
Option C (recommended): both - a higher bar AND a limit on unbounded Mine stacking, below. The bar
is only cheap because the engine behind it has no ceiling.

**2. A Colony has no module slot limit and a Nation State does.** Thirty-four Mines stood in one
Colony. This is the single biggest balance lever in the build.
Option A: give a Colony slots the way a state has them - say 4, plus 2 per Habitat - so Colonists
gate industry and the two halves of the Prospector condition pull on each other.
Option B: diminishing returns - each Mine past the third at one Colony yields less.
Trade-off: A is the cleaner rule and makes Colonists matter everywhere, but it slows every
Faction's off-world build, and the Arkwrights and Archivists may not want that.

**3. Twelve Colonists is the harder bar, and its pipeline is the most frustrating thing in the
game.** Six discrete turns minimum across four order verbs, and losing the state that holds your
Launch Site starts you over (game 2, turn 13).
Option A: let `load` and `transit` be ordered to one Ship in one turn. I could find no rule saying
why not, and it costs a turn per round trip for nothing.
Option B: let a Launch Site lift Emigrants straight onto a station over Earth with no Ship, since a
station over Earth already counts as off Earth.
Trade-off: B makes the first 8 Colonists nearly free for everyone, which helps the bar-12 Factions
more than the bar-30 Arkwrights.

**4. A Module in Antarctica emits like its Facility and nothing says so at the point of decision.**
Eighteen Generators at Lake Vostok emitted 33.75 ppm a turn - more than all my Factories together -
and the first I knew was the Collapse projection falling from turn 22 to turn 18.
Option A: put the Emissions figure on the build button for Antarctic Modules, as the Facility
buttons have it.
Option B: give the Climate Panel a per-source Emissions breakdown for the player's own works, to
match the Income breakdown the Report already has.
Trade-off: none worth the name. This is an information bug, not a balance one.

**5. You cannot see a rival's Standing, so defending is guesswork.** I spent 25 Influence a turn on
East Asia from turn 10 and lost it on turn 13 with my own Standing at 116.
Option A: on a state you control, show the highest rival Standing and what it still needs.
Option B: a Report line when a rival crosses, say, your Standing minus 30. (The brief mentions an
orange warning in the GUI; the headless view has nothing at all, and even in the GUI a number would
beat a colour.)
Trade-off: exact rival Standings make the Influence game more solvable and less atmospheric.

**6. Mothballing is a free, instant, enormous climate lever, and the player finds it by accident.**
On turn 14 of game 2 I mothballed 41 Modules in one order file for nothing, with no Unrest cost
(Colony modules raise none). My Emissions fell 71 -> 14 and Collapse moved from turn 18 to turn 22.
Option A: give a Colony mothball a cost - a turn's delay, or a Restart price that bites.
Option B: leave it, but have the Report say what it did to the projection, so the lever teaches
itself.
Trade-off: A closes the only escape hatch a Prospector who over-built has for saving the world they
still need.

**7. The four Victory Conditions are not on one clock.** At my turn-15 win: Prospectors 1.00,
Custodians 0.00, Arkwrights 0.00, Archivists 0.45, and no rival had picked its gate Tech - the AI
waits for half its first part or turn 24, and none were near half. A Faction whose first part fills
fast is doubly favoured, because it gets its gate early too.
Option A: let each AI pick its own gate Tech from turn 1.
Option B: re-pace the other three first parts against a 36-turn game the way the Prospector pace
table already is. Trade-off: a real re-tune that wants a sweep, not a guess.

**8. Small one-liners.** The final Report is headed `=== REPORT, turn 0 ===`. The Emigrant line
reads "its Unrest fell by 0 to 0" on a calm state. The Tech panel says "Prospectors pick next"
while a Tech is still under research, and a `tech` order then comes back "a Tech is already under
research".

## 6. Bugs, with exact commands and output

### 6a. The Prospectors buy unlimited Energy for nothing, and Materials at a third off

`market_price` multiplies the LOT TOTAL by the Faction's market multiplier and floors it, so
splitting a purchase across order lines is strictly cheaper - and for Energy (1 Ducat a unit x 0.85
= 0.85, floored to 0) it is free. On a fresh turn-1 board with 7 Ducats:

    $ play.exe check --save bug.ron --orders ten-lines-of-one.txt
    line 1: ok `buy energy 1` costs free
    ... (ten identical lines)
    10 order(s) stand, 0 refused. After them: 85M 25F 27E 7D, 17 Influence left of the allotment.

    $ play.exe check --save bug.ron --orders one-lot-of-ten.txt
    line 1: REFUSED `buy energy 10`: needs 8 Ducats, 7 left
    0 order(s) stand, 1 refused. After them: 85M 25F 17E 7D, 17 Influence left of the allotment.

Ten Energy for nothing, or ten Energy for 8 Ducats, depending only on how the order is written.
Materials show the same seam more mildly:

    line 1: ok `buy materials 4` costs 6 Ducats        <- one lot of four
    line 1..4: ok `buy materials 1` costs 1 Ducats     <- four lots of one, 4 Ducats in all

I did NOT use either of these in the two games above - every Energy purchase was a single lot at
the honest price - so the balance numbers in this report stand on their own. Fixes: apply the
multiplier to the per-unit price and multiply out, or round the lot price up rather than down, or
floor a lot at 1 Ducat.

### 6b. An Event that takes Modules offline silently removes the Build Where You Dig discount

Turn 13 of game 1, the turn of a Moonquake ("every Module on the Moon is offline until the next
Resolution"):

    line 3: ok `build module 19 mine` costs 17 Materials
    line 5: REFUSED `build module 19 mine`: needs 17 Materials, 3 left

The identical order cost 10 Materials the turn before and 10 again the turn after. A Mine offline
for one turn because of an Event is not "mothballed, or still building", but `working()` says it
is. It may be intended; it is written down nowhere I could find, and the build price changes with
no reason given.

### 6c. `load` and `transit` cannot be given to the same Ship in one turn

    line 1: ok `load 17 4 from northamerica` costs free
    line 2: REFUSED `transit 17 moon`: this Ship already has an order

I could not find this restriction in CONTEXT.md or any spec. It costs a turn on every round trip,
which is the largest single cost in the Colonist pipeline.

### 6d. Colony Slot indices are 0-based in orders and 1-based in the Report

`unload 17 4 into slot moon 1` (Mare Imbrium, index 1) was reported as
`The Prospectors founded a Colony in slot 2 on the Moon with 4 Colonists.`

### 6e. The game-over Report is headed turn 0

    === REPORT, turn 0 ===
    ...
      [Note] Game over on turn 15: the Prospectors win (met its Victory Condition).

### 6f. A Shipyard shut down by an Energy shortfall reports as "no Shipyard here"

    line 1: REFUSED `build ship colony:13 colonys`: no Shipyard here

The Shipyard was standing and complete; the previous turn's Energy shortfall had shut it down
("Prospectors: Energy ran short; shut down Shipyard."). The Colony view prints the Shipyard with no
mark that would explain the refusal.

## 7. Rules I could not find explained

- That a Module built in Antarctica emits like its Facility counterpart. It is `earth_emissions` in
  `modules.toml` and a version-0.04 ticket comment, but not in CONTEXT.md's Antarctica or Module
  entries and not in PLAYTEST.txt.
- That a Colony has no module slot limit. CONTEXT.md's Module entry says only "A building placed
  inside a Colony"; the fact lives in a `modules.toml` header comment. It is the most important
  structural fact in the game.
- Whether `load` and `transit` are meant to be exclusive in a turn (6c).
- Whether Build Where You Dig is meant to read "working" rather than "not mothballed and not
  building" (6b).
- What a rival's Standing on a state is. Not visible anywhere in the headless driver.

---

# GAME 3 — a fresh world (seed 20260913), North America again

**Result: PROSPECTORS WIN ON TURN 14 OF 36.** Fund 1881/750, 12/12 Colonists off Earth,
Extraction Charter done, +2.14 C, Collapse projected for turn 25.

**Why North America again:** to make the seed the only variable. Games 1 and 3 are the same
Faction, the same start, the same opening plan, on two different worlds. Game 2's East Asia start
already covered "does the start state matter" (it did not: turn 16 instead of turn 15).

## Was seed 20260911's Moon unusually rich? No.

Colony Slot yields are the Body's figure times a triangular draw centred on 1.0 with a spread of
0.25, and the Moon's base Mine yield is 1.65. With four slots on the Moon, the *best* of them is
reliably near the top of that range whatever the seed:

| Moon slot Mine yields | seed 20260911 | seed 20260913 |
|---|---|---|
| the four slots | 1.40, **1.76**, 1.68, 1.64 | **1.75**, 1.56, 1.53, 1.35 |
| best Generator yield at the chosen slot | 1.12 (Mare Imbrium) | 1.50 (Mare Tranquillitatis) |

The best Mine slot was **1.76 against 1.75** — a difference of half a per cent. The new world was
in fact slightly *kinder*, because Mare Tranquillitatis also had the best Generator yield on the
Moon (1.50 against 1.12), so each Generator paid 14 Energy instead of 10 and the Energy treadmill
that shaped turns 7-12 of game 1 barely bit.

The engine is **structural, not lucky**. It needs only: one Colony on the Moon, two working Mines
to open the Build Where You Dig discount, and no slot limit to stop you. Every seed hands you all
three.

## Turn-by-turn, terse

| T | What happened |
|---|---|
| 1 | Deep Mining; Industry raise + Factory + Lab + Power Plant in North America. |
| 2 | **Took Central America** (went straight for it this time; two turns faster than game 1). Shipyard on Tiangong. |
| 3 | Efficient Grids. Factory in Central America. |
| 4 | Colony Ship built (turn 4; game 1 was turn 5). |
| 5 | Automated Refining. **Took Sub-Saharan Africa** — 6 slots, materials lean, Factories at 11. Loaded 4 Colonists. |
| 6 | Transit to the Moon. Three Factories in Sub-Saharan Africa. |
| 7 | **Mare Tranquillitatis founded** (game 1 was turn 8). Took South America. Collapse projection turn 24. |
| 8 | 3 Mines + 4 Generators. Mothballed the North American Refinery. Antarctica opened at +1.61 C. |
| 9 | Extraction Charter picked. Emigrants sent to Lake Vostok by sea. **Moonquake — Mines cost 17 instead of 10 again** (second independent reproduction of that bug). 6 Mines + 2 Generators. |
| 10 | 12 Mines, 5 Generators, Habitat on Tiangong, another Lab. Income that turn: **408 Materials**. |
| 11 | **Share to 80%.** 20 Mines, 12 Generators, a Trade Post, a second Colony Ship. |
| 12 | **Fund 516.** 10 more Mines. 8 Colonists off Earth. |
| 13 | **FUND PASSES 750 — 1136, with 620 banked in that single turn.** 12 Mines more. |
| 14 | Extraction Charter completes, 12/12 Colonists. **WIN, Fund 1881.** |

## The board the turn the Fund passed 750 (turn 13, January 2032)

- Fund **1136**, having banked **620** in one turn — so extraction was **775 Materials a turn**,
  against a 750 *lifetime* bar. The bar was cleared by a single turn's output with 25 to spare.
- One Colony, Mare Tranquillitatis on the Moon: **51 Mines and 28 Generators**, plus a Habitat and
  a Trade Post. No slot limit, so no reason to stop.
- Four Nation States: North America, Central America, Sub-Saharan Africa, South America.
- 8 of 12 Colonists off Earth, and the Extraction Charter still 4 Research short — **both of the
  other requirements were behind the Fund, not ahead of it.**
- +2.05 C, Collapse projected for turn 25, so the world was never in danger of running out first.
- Rivals: Custodians 0/3 Stabilization, Arkwrights 0/30 Colonists, Archivists 32/80 Archive.
  None of the three had picked its gate Tech.

---

# REVISED FINAL VERDICT — three games

| | Game 1 | Game 2 | Game 3 |
|---|---|---|---|
| Seed | 20260911 | 20260911 | **20260913** |
| Start | North America | East Asia | North America |
| Play style | Moon-led | Earth-led, thematic | Moon-led |
| Fund at turn 12 | 175 | 413 | **516** |
| **Turn the Fund passed 750** | **15** | **14** | **13** |
| Result | **win, turn 15** | **win, turn 16** | **win, turn 14** |
| Fund at the win | 1119 | 953 | **1881** |
| Peak extraction in one turn | 623 | ~400 | **775** |
| Temperature at the win | +2.17 C | +2.59 C | +2.14 C |

**Three games, two seeds, two starting states, two play styles, three wins before turn 17.** The
750 bar was cleared on turn 15, 14 and 13. It is not a seed effect and it is not a start-state
effect. On seed 20260913 the Fund passed 750 on the *earliest* turn of the three, on a Moon whose
best Mine slot was 1.75 against the other world's 1.76.

## What the three games together say

**1. 750 is not a target; it is a rounding error on one turn's output.** On turn 13 of game 3 I
banked 620 Materials in a single turn against a 750 lifetime bar. The Prospectors' first Victory
part is, in practice, "survive to about turn 13".

**2. The cause is structural and reproducible, not lucky.** It needs three things, and every seed
supplies all three: (a) a Colony has no module slot limit; (b) Build Where You Dig drops a Mine to
10 Materials once two Mines work, against 16 Materials a turn of output — a one-turn payback that
repeats without limit; (c) a Generator on the same discount pays for the Mines' Energy, so the
package is self-financing. By turn 13 of game 3 one Colony held 51 Mines and 28 Generators.

**3. The Fund is the *easiest* of the three things the Prospectors need.** In all three games it
was met before the Extraction Charter and before the twelfth Colonist. In game 2 the last four
turns were a scramble for eight people, with 953 already banked and nothing to spend it on. The
Victory Condition's two halves are mismatched by a wide margin, and the gate Tech is a third,
separate clock.

**4. No rival was ever in the race.** Across all three games, at the moment I won: Custodians
0.00 (twice) and 0.00, Arkwrights 0.00 (three times), Archivists 0.45, 0.33, 0.45. In every game
not one of the three AI seats had picked its own gate Tech, because the AI waits for half its first
part or turn 24 and none of them were near half. The Prospector AI's observed 450 ceiling is a
measure of the AI, not of the bar.

**5. The recommendation is unchanged and now better supported.** In rank order:
- Cap the module count at a Colony (slots tied to Habitats is the cleanest form) — this is the one
  change that removes the unbounded engine. Everything else is downstream of it.
- Then, and only then, re-fit the 750 bar. With a slot cap the right figure is a sweep, not a
  guess; without one, no figure below several thousand will hold.
- Rebalance the two halves of the condition against each other, or ease the Colonist pipeline
  (`load` and `transit` in one turn; Emigrants liftable straight onto a station over Earth).
- Let each AI pick its gate Tech early, so the other three seats are in the same race at all.

Everything in the FINDINGS section above — the engagement notes, the enjoyment notes, and every
bug in section 6 — held in game 3 as well. The Moonquake removing the Build Where You Dig discount
reproduced exactly on the new seed (turn 9: `build module 19 mine costs 17 Materials`, against 10
on the turns either side).
