# Playtest notes - tester 4 (colonisation-fantasy player), build 0.1.0.0

## Game 1: Custodians, seed 401, start China
- T1 (first impressions): The opening `show` is a wall of ~250 lines. Climate at top (+1.21 C, 452 ppm) is a good stakes hook. But nowhere does the game list what I can build or what it costs - I found the facility/module/ship lists only by typing a bogus name and reading the refusal. QoL: a `costs`/`catalogue` command.
- T1: Nice: a refused order doesn't end the turn, and ok'd orders echo their cost. That's a great dry-run loop.
- T1: Confusing: `build module 21 habitat` on the ISS -> "full until more Colonists live here". ISS shows 2 colonists, room 4, Modules 1/2. So why is it full? Message reads backwards to me (I'd build a habitat to MAKE room).
- T1: Help text is full of "Ticket #337 (version 0.09.0)" developer notes. Breaks immersion; reads like a changelog, not a manual.
- T1: The free-slot list is lovely flavour: Mare Tranquillitatis, Valles Marineris, Occator, Rheasilvia, Caloris Planitia. Real names = sense of place. Want to go everywhere.
- T2: All three AIs signed non-aggression with each other turn 2, then offered me one too. Accepted all - I want to build, not fight.
- T3-4: Materials starvation. Colony Ship = 30M + 35F; the Fuel cost was only revealed after I fixed the Materials refusal. Bought Materials on the market to make it.
- T4: Took India by Influence (nice!). But then recruited Pioneers in India - which has no Launch Site, so they can't lift to my station. Nothing warned me at recruit time.
- T5: "TSV Magellan" - ships get names! Love it. Small touch, big flavour.
- T6: Colonists aboard a ship don't count as "off Earth" (counter dropped 4->1). Fair, but surprising.
- T7 FRUSTRATED: Flew 3 colonists to the Moon, arrived, and `unload ... into slot moon 0` -> "needs 4 Colonists". Nothing told me founding takes 4 before I launched (and "Colony Ship capacity: 4 safe" made 3 look fine). Now 2+ turns flying back to fetch one more person. This is exactly the moment the fantasy should land (first footprints on Mare Tranquillitatis!) and it fizzled into a rules error.
- T7: Custodian win looks remote: emissions 58 vs Sink 12.5 and rising as I absorb India/Indonesia (more people = more emissions). Nature Reserve gave +1 Sink. Feels like my expansion fights my win condition - thematically interesting, but I can't see a path yet.
- T8: Climate Break "Coral Die-off" - "the coasts that lived off them do not forget." Then refugees flow from India to Iran/Vietnam next turn. This is good: the planet dying feels real and has knock-ons.
- T9: Clean Propellant "+5 tank" raised capacity, not the fuel in it - still had to spend a turn refuelling. One order per Ship per turn (load, refuel, transit each eat a turn) makes the first colony feel like paperwork: it took turns 4-11 to found one Moon base 1 turn's flight away.
- T11/12 EXCITED: "Custodians founded a Colony at Mare Tranquillitatis on the Moon with 4 Colonists" + "The first Colony on the Moon eased Unrest by 0.5 in every Region." That second line is perfect - Earth looking up and feeling hope. More of this please.
- T10: Prospectors went Wary of me with no explanation I could see (guess: I was buying influence in Japan where they'd built an Investment Bank).
- T14: Discovered I'd been pouring Influence into Japan for 4 turns while the Prospectors held it - the "neutral" list I picked from was stale in my head and nothing on the influence order said "this is a rival's Region, they'll be offended". Prospectors went Cold. My own fault, but a one-line warning on the order would help.
- T14: Our turn-2 non-aggression Accords quietly lapsed (5 turns). I never noticed; the AIs' accords with each other persist. Would like a "your Accord with X lapses next turn" line in the report.
- T15: Colonist bar met (21/18 off Earth) - two Moon towns (Mare Tranquillitatis, Mare Imbrium) + ISS of 12. Felt good. Arkwrights are already on Mercury (Prokofiev, Raditladi) - envy!
- T15 CONFUSED: `leapfrog` - no text anywhere says what it does. Turns out it cuts a Region's per-person emissions. Fine mechanic, invisible to the player.
- T15 (my mistake / QoL): I used a "test" order file and since every line was valid the turn ended. There's no --dry-run. Ask: `turn --check` that validates without committing.
- T15: Custodian Stabilization counts the WHOLE WORLD's emissions (57) against the Sink (21). I'm 13 of that. My win depends on rivals cutting - interesting, but my levers (scrubber 3ppm, reserve 1ppm, directive 0.02/point) feel like teaspoons against an ocean.
- T19 EXPLOIT?: A Moon Mine yields 35.8 Materials/turn vs 9 for an Earth Mine, for ~15-20M build cost. By T20 I had 191 M/turn income and nothing to spend it on, so I dumped 100-150 M/turn on the market for Ducats (price fell 4 -> 2, so the market does push back). Ducats then buy Influence and Leapfrogs. The Moon becomes a money printer. Thematically lovely (lunar mining boom) but numerically absurd.
- T20: Refugee Convoy card: "take them in" vs "-5 Standing everywhere". Good moral texture. Took them.
- T20/21: Prospectors and Arkwrights went HOSTILE. Cause: every "neutral" Region I'd been influencing (Iran, Kazakhstan) had been taken by them a turn or two earlier, and I'd missed the one-line ControlChanged report. By T21 there were NO neutral Regions left on Earth. The Influence race is over by turn ~15 and nobody told me it was a race.
- T20 Launched TSV Magellan to Mars at the window with 6 colonists - 5 turn trip. Exciting!
- T22 BUG / BIG FRUSTRATION: Flew 6 colonists to Venus (Ishtar orbit). Cannot found anything:
  * `unload 41 6 into slot venus 0` -> "Move this Ship to Venus, low orbit, then found the Colony next turn"
  * `change-orbit 41 low` -> "Venus has low orbit and 3 Orbital Slots, numbered 0 to 2" (refused anyway)
  * `transit 41 venus low` -> "already there" (it's at Ishtar, not low)
  * `build station venus 0` -> "there is no ground to build from: a Colony Ship in this orbit founds it" (it IS in that orbit)
  `show` lists Venus orbits as only "0 Ishtar | 1 Aphrodite | 2 Lada" - no low orbit. Four messages contradict each other. The cloud-city fantasy is the one I wanted most and it's a dead end. Seed 401, Custodians, turn 22, ship 41.
- T23: "full: upgrade to a Settlement for more" on my Moon colonies - but there is no `upgrade` order and `help` never mentions how a colony becomes a Settlement. Stuck at 6 modules with 200+ Materials and 300+ Ducats idle.
- T24: AIs offered me Trades; neither `show` nor the report says WHAT they're offering. I accepted blind.
- T24: Founded Tycho (3rd Moon town). T25 EXCITED: "Custodians founded a Colony at Valles Marineris on Mars". 5-turn crossing at the window, worth the wait. But no "first on Mars" fanfare (Archivists got +20 Influence for first on Deimos, so the hook exists).
- Custodians end state T25: China, India, Indonesia, Vietnam; ISS, 3 Moon colonies, 1 Mars colony; 31/18 colonists off Earth. World at +2.00 C and climbing ~0.03 C/turn; Stabilization 40 ppm over the Sink, run 0 of 4. Win is out of reach - I can't move other Factions' emissions.

### Custodians summary
Best story arc of the four so far: lunar mining towns, a Mars outpost at the window, refugees, coral die-off. But the win condition (WORLD emissions under the Sink) is out of my hands - by mid-game I had money I couldn't spend on anything that moved the needle. Colonising was slowed by order-per-ship-per-turn logistics and the hidden "4 Colonists to found" rule; Venus was a bug-wall.

## Game 2: Prospectors, seed 402, start The United States
- T1: `usa`/`uk` are not accepted - I have to type `northamerica` / `unitedkingdom`, though the board calls them "The United States". The refusal lists display names that also don't work as typed ("The United States" has spaces). Minor but every new player will hit it.
- T2-9: Learned from game 1 - grabbed Mexico (T2), UK (T4), Japan (T6), Australia (T9) by Influence in quick succession. Influence early is king.
- T6 EXCITED: "Prospectors are first to settle the Moon: +5 Influence" - founded South Pole-Aitken on turn 6. The "first to settle" bonus is a great hook; I want it on every world.
- T7: Lift resolves before load, so you can't top a station back up in the same turn you empty it into a ship. Lots of one-turn waits.
- T8: Mars ship launched one turn after the window (5 turns, 19.7F) - the window line was clear and useful ("in 12 turns" next turn made me glad I went).
- T11: Permafrost Thaw break: "a thousand billion tonnes of old carbon starts breathing out." Chilling. Good writing.
- T12: Venture Fund 299/3500 at turn 12 while 464 Ducats sit idle in my stockpile. Only a % of *income* banks; I can't deposit savings. 3,500 looks unreachable inside 25 turns, maybe inside 50. The win bar feels arbitrary vs what I can actually earn.
- Card flavour is good (Whistleblower, Auditors, Carbon Offset Scheme) - small corporate-ethics choices that fit the Prospectors.
- T13 EXCITED: "Prospectors are first to settle Mars: +15 Influence" - Isidis Planitia. Plus Mare Imbrium the same turn. Peak moment of the game so far.
- T13+: Odd - the game changes MY attitude for me: "You are now Hostile toward the Custodians." I never chose that. If it's my Faction's mood, say why.
- T16: Antarctica opened; founded Lake Vostok (a colony ON Earth, under the ice - lovely idea). Rich Seam on the Moon x3 -> 376 Materials income in one turn.
- T17 SHOCK: Lost THE UNITED STATES - my home Region, my only Launch Site - to the Custodians by Influence. My Standing there had decayed and nothing warned me. Got it back next turn by buying 100 Influence with my idle Ducats, but losing your capital silently is a gut-punch. QoL: a warning when a Region of yours is within reach of a rival.
- T18: "The Sink Weakens" break at +2.0 C - Sink drops 11.7 -> 8.7 -> 6.7. Emissions ~100. The world is cooking and no one at the table is slowing down. The stakes feel real now.
- T20: Custodians invaded Indonesia by Army and destroyed its army ("Occupation complete"), then I took it back by Influence the same turn. The war happened entirely in report lines - I never saw it coming.
- T21: VENUS BUG REPRODUCED on seed 402: ship 44 at Ishtar, same contradictory refusals. Now flagged "*** STRANDED ***" (can't afford fuel home). 4 colonists lost to a bug, twice now.
- EXPLOIT (again): Moon/Mars mines -> 200-370 Materials a turn -> sell -> ~1000 idle Ducats -> buy 100 Influence a turn -> take any Region. Money solves everything by T15.
- T21: "Ice Sheets Committed. Greenland and Thwaites pass the point of return; the sea is coming for the coasts and it will not stop." Best line in the game.
- T22-24: Indonesia changed hands 6 times in 4 turns ("passed to Prospectors and threw them off at once: neutral"). Ping-pong, and each flip prints twice in the report (duplicate ControlChanged lines every time).
- T24: Founded Olympus Mons AND Valles Marineris on Mars the same turn - three Mars towns. This is the fantasy.
- T25: Fund 1802/3500 (+~190/turn), climate +2.43 C (+~0.06/turn). Collapse looks ~T34, fund done ~T34. That's a genuine race against the planet - the most tense the game got.

### Prospectors summary
The strongest game. Influence early to snowball regions, first to the Moon and first to Mars (great bonus moments), Moon/Mars mines turning into a money printer. Ten Regions, 2 Moon + 3 Mars + Antarctic colonies by T25. Weak spots: losing my home Region without warning, a 3,500 bar that needs Investment Bank spam rather than anything colony-flavoured, idle Ducats with nothing to buy, and Venus again.
- T25 end: lost The United States AGAIN, this time to the Arkwrights, on the very last turn I played, while I was spending 76 Influence a turn defending the smaller regions. The home-region tug-of-war is the least fun part of the game.

## Game 3: Arkwrights, seed 403, start Brazil
- T1-5 FRUSTRATED/BORED: Brazil makes 1 Ducat and 4 Materials a turn. Five turns of waiting to afford one Colony Ship. The Diaspora faction - the most colonisation-themed one - has the slowest start. Ironic.
- T1 again: a "probe" order file with all-valid lines ended my turn (no dry-run).
- T7: Lovely forecast line: "Last turn to act: 44. On this course the world collapses on turn 49." Exactly the kind of doom clock I want - wish it were at the top of every report.
- T7: The flight table and "Colony Ship capacity: 9 safe" only appear deep in `show`; I'd loaded 6 when I could carry 9.
- T8 MISTAKE/UX: "Grounded Fleet" card - I picked "take" without thinking and it froze my Mars ship for a turn. Fine, my call, but the card should say "your ship X en route to Mars will be delayed".
- T12-16 EXCITED: First on the Moon (+5), first on Mars (+15), first on Phobos (+20)! Stickney crater on Phobos. Racing to plant flags is the best loop in the game.
- T12: Founding uses exactly 4 colonists; the rest stay aboard. OK once you know.
- T15: Energy ran out -> game auto-shut my Shipyard. Then a Moonquake took modules offline and 1 colonist died in each Moon town, dropping them below 4 so I lost a "Body with 4+" for the Diaspora count. Brutal but evocative - space is hostile.
- POSSIBLE BUG: Spaceport says "+9 Influence next turn" after every lift and "first to settle Phobos: +20 Influence", but my "Influence allotment this turn" stayed 12-15 across those turns. I can't see where those points went.
- T19: Efficient Transit tech: Mercury drops from 52F to 23F, Mars 18F -> 8F. Suddenly the whole system opens up. Great "age of exploration" moment.
- T20-22: Three ships in flight at once - to Mars, Deimos and Mercury (9 colonists each). This is the Expanse feeling. Then the 25-turn cap meant I couldn't see them land, but the anticipation was real.
- T24 EXCITED: "Arkwrights founded a Colony at Swift on Deimos" + "Caloris Planitia on Mercury" the same turn, first on both (+20 each). Valles Marineris got 9 more settlers. Peak game moment of the whole playtest.
- T24: Ship to Mercury flagged "*** STRANDED ***" on arrival (8 Fuel left) - but it was a one-way colony ship, so that's fine; the warning is still good to have.
- T24-25: AIs keep offering Trades whose contents I can't read anywhere. Accepting blind every time.
- T25: Colonists off Earth 43/42, 5 of 4 Bodies, score 1.00 - "held: needs Generation Ships, not yet researched". Generation Ships has never once appeared in the Research Lead's shortlist for me. The research pick is shared and AI-led most turns (Custodians led most picks), so my win is gated behind a Tech I can't steer toward. (Also the README says Arkwrights need a Colonial Capital off Earth; `show`'s victory line doesn't mention it.)

### Arkwrights summary
Slowest start (1 Ducat/turn Brazil) but the best payoff: by T25 I had colonists on the Moon, Mars, Phobos, Deimos and Mercury, first-to-settle on five Bodies, a ship en route to Vesta. This is the game I wanted to play. Frustrations: the dead opening, a Moonquake + energy shortfall chain that killed colonists, and a win gated by a shared Tech I never got to pick.

## Game 4: Archivists, seed 404, start Japan
- T1 BUG: The board offers "Large Language Models" but `tech largelanguagemodels` / `llm` are refused; the id list contains no such name. It's actually `publicscience` (display name and order name don't match).
- T1-7: Another slow start (Japan: 4 Materials/turn). Five turns to afford a colony ship. Starts feel same-y: shipyard, wait, ship, wait, Moon.
- T8: First on the Moon again (Tycho). T9: `build archive 28` - The Archive raised on Tycho. A library of humanity on the Moon is a great image.
- T10+: Energy piles up uselessly (200+) - can't sell Energy on the market, only Materials/Fuel.
- Thematic tension: the Archivist win = "upload 18 Colonists... they leave the living population". As a colonisation player, my win condition is to *un-colonise* - digitise the people I worked to fly up. Dark and interesting in Frostpunk terms, but it means my towns shrink as I near victory.
- T15: Cryogenic Tanks (+15 tank) also raised a Colony Ship's BUILD cost from 35/40 to 55 Fuel - the research I didn't pick made my ships pricier. Not explained anywhere.
- T17 CONFUSED: "China now belongs to Archivists (Occupation complete). China was taken: Mine, Power Plant, Sea Wall destroyed." I never ordered an attack. When I took India by Influence (T14), I inherited the Prospectors' Army there, still on Attack stance, and it marched on China and then on Turkey by itself. Emergent and funny, but I destroyed a Sea Wall on a drowning coast without meaning to. A "you inherited Army X, stance Attack" line would help.
- T18: Archive full - 225/225 Research by turn 18 (fund-archive 100% works fast). But "held: needs The Upload, not yet researched" and the Upload never appears in the Research Lead's shortlist for me; the shared tree is mostly picked by the Custodians.
- T22: Lost India to the Custodians; spent 100 bought Influence and got it back. I'd also been dumping Influence into Indonesia for 5 turns while already holding it at Standing 277 - `influence` never says "you already hold this comfortably".
- T24 EXCITED: First on Mars (Hellas Planitia) and first on Phobos (Stickney) the same turn, after The Upload finally came up in the shortlist on T24.
- T25: First Upload: "[Archive] 4 Colonists uploaded at Tycho on the Moon: 4 in all." That's the whole moment: four people stop being alive and become data, and it gets one dry log line. Huge missed opportunity for writing.
- Score 0.22 at T25 (4/18 uploads). The Archivists look like they can win by T35-40 if the colonists keep coming.

### Archivists summary
A clean, focused plan (Moon base -> Archive -> fill it with Research) that worked fast: the Archive was full by T18. Then I waited six turns for the shared Tech tree to offer The Upload, which felt like being stuck behind someone else's pick. Inherited a rogue Army that conquered China for me. The upload mechanic is thematically the most daring idea in the game and the least dramatised.

---

# FINAL SUMMARY

**Custodians (seed 401, China):** Best story texture (refugees, coral die-off, Moon mining towns, Mars at the window), but a win that needs the whole world's emissions under the Sink. I couldn't move it, so by mid-game the win was out of reach and money piled up with nothing useful to buy. Venus was a bug-wall.
**Prospectors (seed 402, USA):** Strongest game. An early Influence snowball, first on the Moon and first on Mars, Moon/Mars mines as a money printer, 10 Regions. Fund 1,966/3,500 at T25 in a real race against +3.0 C. Lost my home Region twice with no warning.
**Arkwrights (seed 403, Brazil):** Dead opening (1 Ducat a turn), then the best payoff. First on the Moon, Mars, Phobos, Deimos and Mercury. Diaspora 43/42 on 5 Bodies by T25, held only by a shared Tech I never got to pick.
**Archivists (seed 404, Japan):** Archive full by T18, then six turns waiting for The Upload to appear in someone else's shortlist. The upload itself is one dry log line.

## Top 5 frustrations
1. **Venus can't be settled (bug, reproduced on 401 and 402).** At Ishtar, the four orders give four refusals that contradict each other ("move to low orbit" / "already there" / "Venus has low orbit" / "a Colony Ship in this orbit founds it"). Colonists stranded twice.
2. **Hidden rules that cost turns:** a colony needs 4 Colonists to found (my 3-person Moon trip had to fly home); lift resolves before load; one order per ship per turn; there is no `upgrade` order for "upgrade to a Settlement"; `leapfrog` is never explained; Trade offers never show what is offered.
3. **Win gates sit behind the shared Tech pick.** Generation Ships and The Upload don't appear in my shortlist for many turns while AIs lead the picks. I met every bar but one, with no lever on the last.
4. **Losing Regions silently.** Standing decays, and rivals take your home or your Launch Site by Influence with no warning. Region ownership ping-pongs, and every flip prints twice.
5. **Slow, same-y openings.** Arkwrights and Archivists sit at 4 Materials and 1 to 4 Ducats a turn, waiting about 5 turns for the first Colony Ship. The colonisation faction starts slowest.

## Top 5 delights
1. **"First to settle X: +N Influence."** Racing to plant flags on the Moon, Mars, Phobos, Deimos and Mercury is the best loop in the game.
2. **Climate Breaks writing.** "Ice Sheets Committed... the sea is coming for the coasts and it will not stop." Plus "The Sink Weakens", Coral Die-off and refugee flows. The planet visibly dies.
3. **Real place names and named ships** (Mare Tranquillitatis, Valles Marineris, Stickney, Caloris Planitia, TSV Magellan). Strong sense of place.
4. **"The first Colony on the Moon eased Unrest by 0.5 in every Region"**, and the doom clock "On this course the world collapses on turn 49."
5. **Refused orders don't end the turn, and valid ones echo their cost.** A great dry-run loop. The Efficient Transit moment that throws the whole system open also stands out.

## Exploits found
- **Off-Earth Mines:** about 35 Materials a turn each vs 9 on Earth. That reached 190 to 376 Materials a turn by T16 to T20. Sold on the market, it became 1,000+ idle Ducats, which bought 100 Influence a turn and any Region I wanted. Money stops mattering by T15.
- **Inherited Armies:** taking a Region by Influence hands you a rival's Army still set to Attack, and it goes on conquering for you.
- Energy piles up uselessly (it can't be sold).

## QoL suggestions
- `turn --check` (a dry run), and a `catalogue` command listing buildings, modules and ships with their costs.
- Accept natural names: `usa`, `uk`, the displayed "Large Language Models" (whose id is actually `publicscience`).
- Warnings: "your Region X is within a rival's reach"; "you already hold X"; "your Accord with Y lapses next turn"; the contents of every Trade offer.
- Explain `leapfrog`, the Settlement upgrade, and the 4-Colonist founding rule *before* launch.
- Put the doom clock and Victory progress at the top of every report.
- Give uploads, first landings and colony growth a line of flavour text.

## Overall enjoyment: 6/10
When it delivers, it really delivers. Three ships in flight to Mars, Deimos and Mercury while the ice sheets let go back home is exactly the Expanse/KSR feeling I came for. The climate writing and first-to-settle bonuses gave me goosebumps more than once. But too much of my time went on fighting the interface instead of the planet: hidden founding rules, one-order-per-ship paperwork, unreadable trades, regions vanishing without warning, a bug that made Venus impossible, and win gates locked behind a shared tech pick I couldn't steer. The economy breaks once off-world mines come online, so the second half of each game is about spending idle money, not about hard choices. Fix the Venus bug, surface the hidden rules and dramatise the big moments, and this is an 8.
