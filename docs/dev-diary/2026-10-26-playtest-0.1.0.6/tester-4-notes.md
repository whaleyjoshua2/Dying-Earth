# Playtester 4 notes: the colonization-fantasy player

## Highlights so far
- I won all four games well inside 30 turns: Custodians T27, Prospectors T31 (at the head of turn 31, after 30 turns played), Arkwrights T23, Archivists T24. At my end no computer Faction scored above 0.49. The computer Arkwrights sat at 0/44 at T24 in G4.
- The space "firsts" carry the fantasy: named ships (TSV Magellan, ARK Challenger), "win the race to the Moon", "the first child born off Earth, at Tycho", first to Mars/Phobos/Deimos. But the human won the Moon race 4/4 by T6-T8, and the first child was born at the human's Tycho 4/4: the AIs build Shipyards and do not launch.
- The frontier is solved by turn ~15: one Moon colony and one 1-turn shuttle meets every off-world bar. After that the ships sit idle, or the same Earth-Moon run repeats (BORED in G4).
- G1 and G2 were won by Earth levers. EXPLOIT 1: Scrubber spam (+3 ppm for 33 Materials, capped only by population). EXPLOIT 2: compounding Fund interest (+1%/turn per Bank or Exchange).
- EXPLOIT 3 (suspected): `build module-ducats` charged 2 Ducats per Material (24.8 for a 12.4-Material Factory). The glossary says "the market price of twice its Materials" (~74). It also skips the off-world Widget bottleneck.
- Recurring friction, all four games: a Colony Ship's tank empties the Fuel pool, and every Power Plant/Generator then runs "at a quarter" with no warning in the text tool. Other repeats: `show` hides pending Accord/Trade offers; one bad line refuses the whole order list; `reactor` vs `fissionreactor`.
- The sea is the strongest drama: 6-19 Facilities lost per threshold, including the Archivists' own Accumulator (G4 T10) and the Prospectors' Banks and Launch Site (G2 T19). It is also, perversely, the world's biggest emissions cut.
- Biggest wasted story beat: losing Mars by the same Resolution after a 5-turn flight, with no sign the rival was en route (G2 T12).
- Rating 6/10 from the colonization-fantasy view (see Final report).

## Game 1: seed 1401, Custodians (China)

Setup reading: README, glossary (Winning/space sections), mouseovers skimmed. Opening board is a wall of text (~40 KB per `show`); I wrote myself a filter.
- T1 first impression: the solar system is all there on turn one (Moon, Mars, Phobos, Deimos, Venus, Mercury, Ceres, Vesta, L4/L5) with named real sites (Tycho, Hellas, Occator) and a fuel/turn table. For my persona that table is the most exciting thing on the screen. EXCITED. But nobody has a single ship; the ISS is two people and a solar array.
- T1 LOST (mild): Custodians win wants emissions under the Sink for 4 phases; the world is at 42 ppm vs 8 sink. A gap of 34 with my own share 5.9. The goal looks unreachable on day one; it reads like "the other three must stop emitting".
- T1 played: Scrubber (Opening Objective met next turn, +0.5 Sink), Shipyard at the ISS, 2 Pioneers recruited, 14 Influence on Japan. Tech is SHARED (one research race for the table), which surprised me: Efficient Grids done the very next turn, picked by the Arkwrights.
- T2 FRUSTRATED (small): the turn refused to end because three rivals offered non-aggression Accords, but `show` had not printed the offers; only the refused `turn` did. Same for Trades later (T8, T11): the offer's terms show only after a failed `turn`.
- T4 Japan taken by Influence. Colony Ship ordered at the ISS: 33M + 32 Fuel. Spent every drop of Fuel on the tank.
- T5 FRUSTRATED: Fuel at 0 means both Power Plants "ran at a quarter" -> Energy -11. Nothing at order time warned me the plants burn Fuel; the order summary just said "After them: 0F". [hover covers? the income line shows "Power Plant (upkeep) -0.8F", so it is discoverable, but no warning].
- T5 EXCITED: my Colony Ship is the **TSV Magellan**. A name! Loaded 6 Pioneers straight from China, launched to the Moon the same turn.
- T6 Magellan arrived in one turn; ordered the landing at Tycho.
- T7 EXCITED (the best moment so far): "Custodians win the race to the Moon. Prospectors had a Shipyard; Archivists had a Shipyard; Arkwrights had no Shipyard. Tycho on the Moon is theirs." +5 Influence, Unrest eased everywhere. Victory 11%. This is the fantasy. But: "2 more found no Habitat room" - the colony opens with room 4 and my 6 aboard had to wait in orbit. LOST briefly: I'd loaded 6 because the ship holds 6; nothing told me the new Colony takes only 4.
- T7 Hard Winter card (40 Ducats or +2 Unrest): paid. Rivals all refused.
- T8 FRUSTRATED: tried to unload the 2 waiting Colonists the same turn the Habitat would finish; refused "no room in its Habitats". Reasonable rule, but the whole order list is refused for one line, so I re-sent everything.
- T9 2 Colonists down: Tycho at 6. Mine done (Build Where You Dig discount kicks in: Shipyard 28.9 instead of 38.5).
- T10 Break: Permafrost Thaw at +1.6 C, Antarctica opens. Temperature rising ~0.04/turn; collapse projected turn ~37-43. Leapfrogged China.
- T11 Planetary Stewardship (my gate) chosen: I hold the Research Lead. Magellan refuelled at the ISS, loaded 6 more Pioneers from China, back to the Moon. Shuttle rhythm: 2 turns a round trip. EXCITED, it feels like a real lunar shuttle.
- T12 Prospectors founded Lake Vostok in Antarctica. India mine by Influence (6 Regions held). Taking neutral Regions with Influence is very easy: 14-21 a turn, thresholds 30-40.
- T13 Tycho at 12 of 20 off Earth. Plan forms: Scrubbers are capped one per 200M people, so China (1,400M) takes 6-7 and India (1,600M) 8. Each adds 3 ppm to the Sink for 33 Materials. The 34-ppm gap that looked impossible on T1 is about 12 Scrubbers. EXCITED (a plan!), but see EXPLOIT below.
- T14 The sea rose at +1.8 C and took NINE of my Facilities in one Resolution (Power Plants, Factories, a Mine, a Refinery). Dramatic and fair, but my Widget rates collapsed. FRUSTRATED a little that nothing warned "the next Sea Level threshold is +1.8 C and will take N of your coastal Facilities" (maybe the Climate Panel does in the window). Planetary Stewardship (my gate) done.
- T15 FRUSTRATED: `build facility china reactor` refused: the order word is `fissionreactor`, though the cost table, README and Report all call it "Reactor". The refusal lists the words, so 1 retry.
- T15 Prospectors' PMV Challenger is 2 turns from Mars; I had missed the T7 Mars window. Ship names on rivals too (PMV Beagle, ACV Endeavour, ARK Resolution): lovely.
- T16 Tycho upgraded to Settlement. Archivists land at Mare Tranquillitatis next door (T17): neighbours on the Moon.
- T17 Second Colony Ship built AT Tycho (Moon shipyard: 24.8 Materials instead of 33). TSV Endurance. Fuel is my bottleneck all game: 25 Fuel cost 80 Ducats.
- T18 EXCITED: "Yours: the first child born off Earth, at Tycho on the Moon." Exactly the beat my persona plays for. Endurance loaded 6 Colonists from Tycho and set off for Phobos (6 turns).
- T19-22 Scrubbers everywhere; taking Regions by Influence keeps raising MY emissions (I'm the top emitter at 17-20 ppm by T19 because I inherit every Region's people and plants). Nice irony for the Custodians.
- T19-23 THE CONTEST: I spent Influence on the Prospectors' home, Europe. They answered (Standing 22 -> 41 -> 61 -> 81, a Strip Permit, a Sea Wall), and I took it on T23 with 104 vs 81. I saw them climb each turn in `show` and simply out-spent them (28-33 a turn). Then I mothballed its Mines and Refinery and built two Scrubbers there. EXCITED: the eco-faction annexing the polluters' capital and switching the mines off is a great story beat.
- T24 Emissions under the Sink, run 1 of 4.
- T25 EXCITED: Endurance lands at Roche: "Custodians are first to settle Phobos" (+20 Influence). Grounded Fleet card answered "take" while the ship was landing (unload was not blocked).
- T26 The AI turned on me: Archivists invaded Mexico (army destroyed, occupation), Prospectors attacked Ethiopia, both refused non-aggression. Run 3 of 4.
- T27 **WIN: "Game over, turn 27: Custodians win."** 26 of 20 Colonists off Earth, 12.3 ppm under the Sink. Rivals: Prospectors 0.30, Arkwrights 0.22, Archivists 0.33 (Archive 218/250).
- Game 1 verdict for my persona: the colonization half was the joy (Magellan shuttling, first on the Moon, first child, first on Phobos) but it was the EASY half: one Moon colony fed by one ship gave 20 Colonists by T20. The win was decided by building ~16 Scrubbers on Earth. My persona would feel the victory screen arrived before the space story had really started (no Mars city, no second-generation anything). BORED moments: few; T2-T4 waiting for the Shipyard.
- EXPLOIT (G1): Scrubber spam. A Scrubber is 33 Materials + 4 Energy + 1 Ducat a turn for +3 ppm Sink, capped by population (1 per 200M), so the two most populous Regions (China, India; Custodians start in China) hold ~15 Scrubbers = ~45 ppm, more than the whole opening gap (34). Repro: seed 1401 Custodians, take India by Influence (T10-12), build 2 Scrubbers a turn from T13 (Materials income ~45-70), run starts T24, win T27. Strong: it beat a "collapse on turn 37" world with 10 turns to spare.

## Game 2: seed 1402, Prospectors (China again)

- T1 The human seat starts in China again (Prospectors this time; the Custodians AI got Europe). Win: 6,500 Ducats in the Venture Capital Fund + 20 Colonists beyond Earth orbit + The Extraction Charter. The Fund earns 1% of its balance per Investment Bank / Exchange per turn, so the plan is: a bank in every Region I can take, Exchanges off Earth, and banking a share of income.
- T2 Carbon Levy card: +20 Ducats (they went straight into the Fund?). T3 FRUSTRATED (tiny): the Colony Ship is not discounted by Cheap Industry (15% off Facilities and Modules only) so my 30.8 Materials fell short; the cost table I read on T1 already showed the Prospectors' discounted Facilities, so I assumed ships too.
- T6 EXCITED: won the race to the Moon again (Tycho, turn 6). Second game in a row the human wins it; the AIs build Shipyards first and then sit.
- T7 Launched PMV Magellan to Mars in the January 2031 window (5 turns, 18.1 Fuel). The "Mars window: this turn" line made me feel like a mission planner. EXCITED.
- T9 FRUSTRATED (repeat of G1 T5): spent my Fuel on a Colony Ship, Power Plants starved at the next Income. Same trap twice in two games.
- T10 Opening Objective met (3 Investment Banks for 3 turns): +50 into the Fund. "Last turn to act: 24"! I am the dirty Faction and the world is cooking: net +58 ppm a turn, collapse on turn 26.
- T11 The sea at +1.8 C took six of my Facilities; perversely that cut world emissions from 69 to 47 and pushed the collapse to turn 36. Built three Nature Reserves.
- T12 LOST/FRUSTRATED: Magellan reached Mars after a 5-turn flight and landed at Olympus Mons... the same Resolution the Custodians landed at Isidis Planitia: "Custodians are first to settle Mars." I never saw their ship coming; I could not tell whether they launched before me or whether ties go by fleet. A five-turn race lost on the last day with no warning is a big story beat wasted. The same turn the Custodians invaded my Mexico with an Army (no war declared, no Accord between us since it lapsed).
- T13 EXCITED: first child born off Earth at Tycho again (+5 Influence). 21 of 20 Colonists off Earth by T14: the Off-world Presence bar is done before turn 15 in both games.
- T14 Venture share to 80%. Fund at 382, +65 a turn.
- T15 Sent Magellan (refuelled at nothing; it still had 13.9) on a 1-turn hop Mars -> Deimos with 4 Colonists from Olympus Mons. T16 EXCITED: "Prospectors are first to settle Deimos" (+20 Influence). Hopping inside the Mars system for 7 Fuel is lovely: it feels like The Expanse's Belt-and-moons logistics.
- T15 Refinery at Olympus Mons makes it a depot: Magellan refuelled at Mars on T18. EXCITED (a real off-world fuel economy).
- T17 Research Lead matters for my gate (The Extraction Charter). Built four Research Labs; led on T18, T20 (picked the Charter, done T22).
- T18-T20 The sea at +2.2 and +2.3 C took 19 more of my Facilities, including two Investment Banks and my only Launch Site. "No free build slot" in India and the UK afterwards: the sea eats slots for good. The Refugee Convoy's new "Send them to the stars" answer was SHUT for me because the sea had taken my Launch Site: a sad, apt moment, but I never got to use the new feature.
- T19-T22 THE CONTEST: Central America (Mexico) had been taken from me by a Custodians Army (T12). I spent 26 Influence a turn trying to buy it back: 59 vs 63, 85 vs 84, 137 vs 138... the Custodians matched me point for point every turn. FRUSTRATED: four turns and ~100 Influence for nothing; I could see both Standings each turn, so I knew, but the AI's matching felt mechanical. I gave up and took neutral Kazakhstan and Iran instead.
- T21-T23 FRUSTRATED: three whole-list refusals in a row on one turn (Mass Driver needs Efficient Transit; Solar Array only on a station; 0.9 Materials short). Each refusal re-sends the whole order list. With `--force` it is avoidable, but the default punishes the 10-line turn for one bad line.
- T22-T31 The Fund compounding is the Prospectors' game: interest grew from 4.8 (T10) to 104 (T21), 304 (T26), 524 (T31) as Banks and Exchanges multiplied (1% per payer). EXCITED in the "number goes up" way for the last eight turns, but it is a spreadsheet thrill, not a space one.
- T26 Second Objective met (China to Industry Level 5): +100.
- T29 the Custodians AI invaded my Turkey with two Armies; the Archivists and Arkwrights spent Influence on my China (48 of 113). The table ganged up as I neared the bar: good tension.
- **T31 WIN: "Game over, turn 31: Prospectors win."** Fund 6,547 / 6,500, 34 of 20 Colonists off Earth. (Collapse was projected for turn 26 at T10, pushed back to 43-49 by the sea drowning everyone's factories.) Rivals: Custodians 0.49, Arkwrights 0.33, Archivists 0.38 (Archive full 250/250 but The Upload never researched).
- Game 2 verdict for my persona: three colonies on three Bodies (Tycho, Olympus Mons, Voltaire on Deimos) by T16, then nothing to do in space for 15 turns but stack banks on a drowning Earth. My two Colony Ships sat idle from T19 to the end. The win again came from an Earth lever, not from the frontier.
- EXPLOIT (G2): compounding Fund interest. Every Investment Bank (one per Region) and every Exchange (one per Body) adds 1% of the Fund balance a turn. With ~9 payers the Fund grew 8-9% a turn, so the last 3,000 Ducats took 4 turns. Repro: seed 1402 Prospectors, venture-share 80 from T14, take neutral Regions by Influence (Japan, UK, Mexico, Australia, India, Vietnam, Turkey, Kazakhstan, Iran) and build an Investment Bank in each the turn after, Exchanges at Tycho, Voltaire, Tiangong. Win T31. Strong: interest (524) outran banked income (191) by T31.

## Game 3: seed 1403, Arkwrights (China, with a Spaceport)

- T1 My persona's own Faction: 44 Colonists beyond Earth orbit over 4 Bodies (4+ each), a Colonial Capital off Earth, Generation Ships researched. No station, 4 Pioneers waiting, 50 Fuel, 3 Pioneers a turn, Colony Ships carry 5. EXCITED from the brief alone.
- T1 Built Orbital Reef over Earth from the 4 waiting Pioneers (33 Materials, done the same Resolution). T2 Shipyard there. T4 ARK Magellan (carries 7).
- T5 "loaded 5 Colonists at Earth (Spaceport: +5 Influence)": the Spaceport pays Influence per Colonist lifted. Nice flavour; I never saw it explained [hover covers? not found in mouseovers.md under Facilities by my search].
- T6 EXCITED/also suspicious: won the race to the Moon AGAIN (3 of 3 games, every time by T6-T7). The AIs each build a Shipyard by T3-T4 and then do not launch for 10+ turns.
- T6 FRUSTRATED (3rd game running): built a Colony Ship, Fuel to 4, every Power Plant at a quarter. I knew the trap and still fell in, because the Colony Ship's Fuel is paid out of the same pool.
- T7 Mars window: ARK Challenger launched with 7. T9 Venus window: ARK Discovery launched with 7 for a station at Ishtar. T10: three ships in flight at once across the inner system, Magellan shuttling to Tycho. EXCITED: the best "fleet at sea" feeling of the playtest.
- T11 BUG?: every "The sea took your X" line is printed twice in the Report (e.g. "The sea took your Power Plant in China" x2) while the summary line lists it once.
- T12 EXCITED: "Arkwrights are first to settle Mars" (Valles Marineris) and "Arkwrights built Ishtar over Venus" the same Resolution. Second Objective met (a free Colonist).
- T13 Challenger took 1 Colonist from Valles (it had 5 in room 4) and hopped to Phobos for 3.8 Fuel. T14 "first to settle Phobos". 4 of 4 Bodies with 4+ by T14 (Moon, Mars, Venus, Phobos), 35/44. First child born off Earth at Tycho, T13 (3 of 3 games, always Tycho, always the human).
- T17 44/44 Colonists beyond Earth orbit. All that remained: a Colonial Capital and Generation Ships.
- T18 EXPLOIT?: `build module-ducats tycho factory` cost 24.8 Ducats (twice its 12.4 Materials as a NUMBER; the glossary says "at the market price of twice its Materials", which at 3 Ducats would be ~74), skipped the Widget queue and stood next Resolution. That is cheaper than buying the Materials and it bypasses the only real bottleneck off Earth (4 Widgets a turn). Repro: any Colony, `build module-ducats <colony> factory`, compare with `buy materials`.
- T18-T22 Filling Tycho's 12 Settlement slots to buy the Colony tier, then the Capital (77 Materials). The "every slot built" rule made me build a second Generator and an Observatory I did not need: tier-climbing is box-ticking. BORED a little.
- T20 Generation Ships done (I led with 15 Research a turn after three Labs).
- **T23 WIN: "Game over, turn 23: Arkwrights win."** 48/44 off Earth over 4 Bodies, Colonial Capital at Tycho. I never chose its capital building: the game ended the Resolution it stood. Rivals: Custodians 0.26, Prospectors 0.12, Archivists 0.21. World collapse projected T35.
- Game 3 verdict for my persona: the best game of the four for the fantasy (three ships in flight, Mars + Venus + Phobos landings within three turns) and also the fastest win. It ended just as it got good: no Capital building chosen, no ferry route ever needed, no second Mars city, no ship ever in danger. The Arkwrights' bar is EASY for a human (T23) even though the designer asks how to bolster their wins: what needs bolstering is the computer Arkwrights (they won no race in G1-G2: 7/44 at T27 in G1, 15/44 at T31 in G2).

## Game 4: seed 1404, Archivists (China, the Accumulator, Axiom station)

- T1 Win: the Archive (a Module at a Colony-tier ground Colony off Earth), 250 Research paid into it, 16 Colonists Uploaded (4 a turn), The Upload researched. Research is pulled two ways (Archive fund vs. the shared Tech that holds my gate). For my persona, "uploading" colonists is the least colonial win: the people I fly out are read into a machine and leave the living population.
- T1-T7 As the Archivists I led EVERY Tech pick from T1 to T8 (+5 Influence each time); the shortlist offered my own chain each time, and The Upload (my gate, rung 4, 62 Research) was on the list by T6 and DONE on T8. EXCITED at first, then suspicious: the gate of the slowest-looking Faction stood before I had a Colony.
- T4 FRUSTRATED (4th game of 4): Colony Ship ordered, Fuel to 4, Accumulator and Power Plant "at a quarter".
- T5 Opening Objective (Lab + Observatory off Earth): +10 Research into the Archive.
- T5 Hard Winter card: "THE OFFER IS CLOSED TO YOU: you have 34.2 Ducats of the 40". Good clear wording.
- T6 "On this course the world collapses on turn 29", Last turn to act 24. I was the top emitter (17-22 ppm) for no reason I could see beyond inheriting Japan/UK.
- T8 Won the race to the Moon (Tycho) - 4 games out of 4, always the human, always Tycho, always by T8.
- T8 Directive 100%: 26 Research a turn into the Archive. Full (250) on T16.
- T10 The sea took my Accumulator (the Archivists' signature building) at +1.8 C. My Energy went from +36 to -28 a turn. FRUSTRATED: a Faction's unique Facility can drown in the first sea rise; nothing warned it was coastal.
- T11-T18 Shuttle: ACV Magellan and ACV Challenger (4 Colonists each, 6 after Expanded Habitats) alternating Earth <-> Moon. BORED by T14: this was the 4th game of identical 1-turn Moon runs, with no choice in it but "load 4, fly, unload". A ferry route would have done it, but each run needed a refuel and a Habitat check so I kept doing it by hand.
- T14 Settlement; T16 the "Surplus Habitats" card filled my 12th slot for free; T18 Colony tier; T19 `build archive tycho`; T20 "Archivists completed the Archive at Tycho on the Moon".
- T17 EXCITED: "the first child born off Earth, at Tycho" (4 of 4 games, Tycho each time).
- T17-T20 Energy crisis: fuel-burning Generators on the Moon ran at a quarter when Fuel ran out (LOST: I did not expect a lunar Generator to burn Fuel). Rescued with two Reactors (needs Fissionables I had to buy: 8 Ducats each) and by mothballing Earth Mines, the Axiom Shipyard and an Observatory.
- T21-T24 Uploads, 4 a turn: "4 of Tycho's people were read into the Archive." For my persona this is the saddest win text in the game: the colony I built shrank from 33 to 21 people and that was the victory. It does fit the Archivists, and it is well written, but the colonization fantasy runs backwards here.
- T24 small oddity: Report lines "Unrest from 0.5 to 0.5 (the heat)" for eight Regions: a change line that records no change.
- **T24 WIN: "Game over, turn 24: Archivists win."** 16/16 uploaded, Archive 250/250. Rivals: Custodians 0.44, Prospectors 0.06, Arkwrights 0.00 (0 of 44 off Earth at T24!). World collapse projected T42.
- Game 4 verdict: a race against Energy, not against rivals. The Archivists' win wants one big Moon colony and nothing else; Mars, Venus and the moons of Mars never mattered. My two ships never left Earth-Moon space.

## Final report

### 1. Per-Faction summary

| Game | Faction | Outcome | Victory figure at the end |
|---|---|---|---|
| G1 seed 1401 | Custodians | **Won, turn 27** | Stabilization 4/4 (12.3 ppm under the Sink), 26/20 Colonists beyond Earth orbit, score 1.00 |
| G2 seed 1402 | Prospectors | **Won, at the head of turn 31** (30 turns played) | Fund 6,547/6,500, 34/20 off Earth, score 1.00 |
| G3 seed 1403 | Arkwrights | **Won, turn 23** | 48/44 beyond Earth orbit, 4/4 Bodies, Colonial Capital at Tycho, score 1.00 |
| G4 seed 1404 | Archivists | **Won, turn 24** | Archive 250/250, 16/16 uploaded, score 1.00 |

- **Custodians.** The off-world half was easy and lovely: TSV Magellan shuttled Pioneers from China to Tycho, and I was first on the Moon and first on Phobos. The first child born off Earth was at Tycho. The win, though, came from Earth. I took India by Influence, built about 16 Scrubbers at 33 Materials each, took the Prospectors' Europe and mothballed its Mines. A "collapse on turn 37" world reached four phases under the Sink by T27. As a story, the Custodians annexing the polluters and switching off their mines was great. As a colonization game, space was finished by T20.
- **Prospectors.** The game is a compounding spreadsheet. Investment Banks in nine Regions plus three Exchanges gave 8-9% interest a turn; interest was 524 a turn by T31 against 191 banked. The space part was three colonies on three Bodies by T16 (Tycho, Olympus Mons, Voltaire on Deimos, first on Deimos). Then my two ships sat idle for 15 turns. Lost the Mars race on the same Resolution, with no warning.
- **Arkwrights.** The best game for my persona and the fastest win. Three ships were in flight at once by T10 (Moon shuttle, Mars, Venus). Mars and Venus fell on T12, Phobos on T14, and 44 Colonists were off Earth by T17. After that it was only slot-filling at Tycho to buy the Colony tier and then the Capital. It ended before I could choose the Capital's building.
- **Archivists.** I led every Tech from T1 to T8, and The Upload (the gate) stood on T8. The Archive fund filled by T16 at 26 Research a turn, so the real race was against Energy, after the sea drowned my Accumulator on T10. One Moon colony, two ships doing 1-turn Moon runs, then four Uploads. The win shrinks the colony; that fits the Faction but runs against my fantasy.

### 2. Lost / bored / excited (strongest moments)

**EXCITED**
1. G3 T10-T12: three named Colony Ships in flight at once. "Arkwrights are first to settle Mars" and "built Ishtar over Venus" on the same Resolution.
2. G1 T7: "Custodians win the race to the Moon. Prospectors had a Shipyard; Archivists had a Shipyard..." The best line in the game. It names the losers and says why.
3. "Yours: the first child born off Earth, at Tycho on the Moon", in all four games. Exactly my persona's beat.
4. G1 T19-T23: an Influence fight for the Prospectors' home (Europe), visible turn by turn (22, 41, 61, 81), won with 104, followed by mothballing their mines.
5. G2 T15-T18: a 1-turn hop Mars to Deimos with 4 Colonists, "first to settle Deimos", then refuelling at my own Refinery on Mars. The Expanse-style local logistics are the best systems moment.
6. G2 T22-T31: the Fund's interest snowball ("number goes up"), and the table ganging up on me at the end.

**LOST**
1. G2 T12: I lost Mars on the landing turn to a Custodians ship I never saw coming. I could not tell how a same-turn landing is settled.
2. G1 T7: I loaded 6 because the ship holds 6, but a new Colony takes 4 and 2 were left aboard with no Habitat room.
3. G4 T17: lunar Generators burn Fuel; when Fuel ran out the Moon went dark.
4. G1 T1: the Custodians' bar (net 34 ppm over the Sink) looks impossible on turn one. Nothing hints that population-capped Scrubbers close it by themselves.

**BORED**
1. G4 T11-T18: the fourth game of identical "load 4, fly to the Moon, unload" runs.
2. G3 T18-T22 and G4 T14-T18: building Modules I did not want (a second Generator, an Observatory) only because a tier requires every slot built.
3. G2 T19-T31: nothing to do in space; my two ships idle for 12 turns.
4. Early turns (T2-T4) waiting for the first Shipyard at 4 Widgets a turn.

### 3. Frustrations (ranked)

1. **The Fuel trap, four games out of four.** A Colony Ship's tank (32 Fuel) is paid from the stockpile that Power Plants, Generators and the Accumulator burn. The next Income then says "Fuel ran short; Power Plant ... ran at a quarter", and Energy swings by 20-40. The order summary only says "After them: 0F". [hover covers in part: the window's Energy figure turns red and lists what goes dark when a Shortfall is forecast. It is unclear whether that forecast counts Fuel-starved plants, and the text tool shows no forecast.]
2. **One bad line refuses the whole turn.** G2 T23 took three resubmissions (Mass Driver needs Efficient Transit; Solar Array only on a station; 0.9 Materials short). `--force` exists, but by default a 10-line turn is lost to one typo.
3. **`show` does not print pending Accord or Trade offers.** I only saw them after a `turn` refused (G1 T2, T8, T11; G2 T2; G3 T19; G4 T19).
4. **No warning that a rival is racing me to a Body.** G2 T12, Mars.
5. **Influence ties with an AI that matches every point.** G2 T19-T22: Central America went 59/63, 85/84, 137/138, wasting ~100 Influence. Visible, but mechanical.
6. **The sea drowns unique buildings with no forecast** (G4 T10 Accumulator; G2 T19 two Investment Banks and my only Launch Site). [hover covers: the Facility hover says "On the coast, the sea can take it at a threshold", but nothing says which threshold or when.]
7. **Naming.** The order word is `fissionreactor` while the costs table, README and Report say "Reactor" (G1 T15). "Solar Array stands only on a station" was a surprise (G2 T23).
8. Small: the Colony Ship takes no Cheap Industry discount (G2 T3); "Unrest from 0.5 to 0.5" lines (G4 T24); every "The sea took your X" line is printed twice (G3 T11, T18).

### 4. Exploits

1. **Scrubber spam (Custodians). Strong.** It decided G1. A Scrubber costs 33 Materials + 4 Energy + 1 Ducat a turn for +3 ppm of Sink. The cap is 1 per 200M people, so China (1,400M, the start) holds ~7 and India (1,600M) ~8: about 45 ppm, more than the whole opening gap (34). Repro: seed 1401 Custodians; take India by Influence (T10-T12); from T13 build 2 Scrubbers a turn; also Scrubbers in every small Region taken (1 each). Run starts T24, win T27.
2. **Compounding Venture Fund (Prospectors). Strong.** It decided G2. Each Investment Bank (one per Region) and each Exchange (one per Body) pays 1% of the Fund balance a turn. With ~9 payers the Fund grew 8-9% a turn and the last 3,000 Ducats took four turns. Repro: seed 1402 Prospectors; venture-share 80 from T14; take cheap neutral Regions by Influence and build an Investment Bank each; Exchanges at Tycho, Voltaire and Tiangong. Win at the head of T31.
3. **`build module-ducats` is cheap and skips Widgets. Moderate; possibly a bug.** G3 T18: a Factory Module listed at 12.4 Materials cost 24.8 Ducats and stood next Resolution. G4 T16: an Observatory cost 46.2 Ducats. The glossary (Trading window) says "for Ducats at the market price of twice its Materials", which at 3 Ducats per Material would be ~74 for the Factory. Off Earth, Widgets (4 a turn per Colony) are the main bottleneck, so this is the fastest way to climb tiers. Repro: any Colony; compare `build module-ducats tycho factory` with `buy materials`.
4. **Research Lead snowball (Archivists). Moderate.** In G4 I led every pick from T1 to T8 with one Lab, +5 Influence each time. The shortlist always carried my next rung, so my rung-4 gate stood on T8. Repro: seed 1404 Archivists; Research Lab in China T1, Observatory at Axiom T3, pick your own chain each time.
5. **The sea as a climate fix. Weak to moderate, and not under the player's control.** Every sea threshold deletes 6-19 Facilities per seat; in G2 it cut world emissions from 69 to 47 and moved the collapse from T26 to T36. Nobody has to stop polluting; the ocean does it.

### 5. Suggestions

**a. Making the game more interesting and compelling**
1. **Keep the game going after the first off-world bar** (G1 T20, G2 T14, G3 T17, G4 T20). The frontier has no second act. Add later-game off-world goals worth score or Influence: a city of 30 on Mars, a station at Ceres, a self-sufficient colony (it makes its own Fuel and Energy).
2. **Make Earth's collapse a turning point, not a game over, for Factions with off-world people.** At +3.0 C, Earth's Regions could go dark while Colonies play on for N turns: the Expanse / Aurora moment. Today collapse simply ends the game for everyone. It solves the "dying Earth" fantasy having no last chapter.
3. **Put the frontier on the critical path of every win.** All four wins needed one Moon colony at most (Tycho). Have each win need a second Body, or distance, for example "20 Colonists, at least 8 of them beyond the Moon".
4. **Chronicle colony stories.** "Founded July 2032" exists; add per-colony milestones (first child, reaching each tier, first ship built off Earth) and show them in the Chronicle.

**b. Balance: bolstering the Arkwrights' and Archivists' wins**
In my hands both won fastest (T23, T24). What needs bolstering is the **computer** seats. Computer Arkwrights: 7/44 at T27 (G1), 15/44 at T31 (G2), 0/44 at T24 (G4). Computer Archivists: Archive full (250) in G2 but The Upload never researched; 0.33-0.38 at best.
1. **Computer Arkwrights should launch.** In G4 they had Orbital Reef and a Shipyard by T6 and zero Colonists off Earth by T24. Make them build a Colony Ship as soon as a Shipyard stands and fly every launch window. Dying-Earth story: the Arkwrights are the ones who leave.
2. **Computer Archivists should research their gate before over-filling the fund.** In G2 they hit 250/250 at T31 with no Upload. Once the fund is full, Directive 0 and push The Upload.
3. **If the human Arkwrights stay this fast, lengthen the Capital step, not the Colonist count.** Example: a Capital needs 6 turns at Colony tier. Today the Capital is box-ticking (fill 12 slots, pay 55, pay 77) and the win lands the turn it stands (G3 T23).

**c. UI fixes, adjustments and text reductions**
1. **Fuel forecast.** When orders leave Fuel below next Income's plant burn, say so in the order summary: "After them: 0F -- your Power Plants will run at a quarter." (G1 T5, G2 T9, G3 T6, G4 T4.)
2. **`show` lists pending offers** in a block at the top: "OWED: Accord from Prospectors (non-aggression) -- accept/refuse."
3. **A rival ship's destination on the Ship row.** "PMV Challenger seat 1 at Earth -> Mars (2 left)" was visible in G1 but I never saw the Custodians' Mars ship in G2. Add a Report line: "A Custodians Colony Ship is bound for Mars, arriving in 2 turns."
4. **Name the order word as the player sees it:** accept `reactor` for the Reactor.
5. **Text reductions** (quote, then proposal):
   - "Custodians win the race to the Moon. Prospectors had a Shipyard; Archivists had a Shipyard; Arkwrights had no Shipyard. Tycho on the Moon is theirs; Unrest eased by 0.5 in every Region." -> "Custodians win the Moon race: Tycho is theirs. Unrest -0.5 everywhere."
   - "Sea rose at +2.3 C: you lost a Refinery and a Launch Site (China), a Power Plant and an Investment Bank (India), ..." plus each loss again as its own line, twice -> keep the one summary line and drop the per-Facility duplicates.
   - "*** THE OFFER IS CLOSED TO YOU: you have 34.2 Ducats of the 40 it asks, so `answer refuse` is your only move. ***" -> "Closed: you have 34.2 of 40 Ducats. Only `answer refuse`."
   - "[Unrest] Japan: Unrest from 0.5 to 0.5 (the heat)." -> do not print a change that changes nothing.
   - "The turn did NOT end: 1 line was refused. Fix them, or pass --force to end the turn with the rest." -> "Turn not ended: 1 line refused. Fix it, or --force."

**d. Making the AI smarter**
1. **Launch when a Shipyard stands.** All three AIs built Shipyards by T3-T4 in every game and lost the Moon race to me 4/4 (T6-T8).
2. **Do not mirror a human's Influence point for point on a 1-slot Region** (G2 Central America). Defend home Regions and the Regions holding your unique building; let a marginal one go.
3. **War timing works; aim it better.** The late gang-up (G1 T26 Mexico/Ethiopia/Kazakhstan; G2 T29 Turkey) hit Regions that did not matter to my win. Hit the place that carries the leader's bar instead: the Custodians' Scrubber Regions, the Prospectors' Bank Regions, the Archivists' Archive colony.
4. **Fill the gate before the fund** (Archivists, see 5b.2).

**e. Bolstering the colonization side**
1. **Ferry routes need a reason to be used.** I never set one. Each Moon run needed a refuel decision and a Habitat-room check. Let a route refuel from the stockpile at each end automatically and skip a leg when the destination is full, with a Report line ("Magellan waited: Tycho full"). The shuttle then becomes a set-and-forget lifeline and my attention goes to new Bodies.
2. **Let a new Colony open with the ship's whole load, crowded.** Today it takes 4 and strands the rest aboard (G1 T7, G3 T6, G3 T12). Crowding already exists for ships, and it fits the frontier's hardship.
3. **Distance should pay.** Ceres and Vesta (25 Influence first-landing prizes) were never worth a ship in four games. Give far Bodies unique yields, such as Fissionables needed for nuclear hulls and Reactors, so a Belt run is a real economic choice. In G4 I paid 8 Ducats per Fissionable to keep Reactors running while Vesta sat empty.
4. **Off-world self-sufficiency.** Lunar Generators burning Earth Fuel (G4 T17) broke immersion. Let Colony power come from local yields (Solar on the ground, Reactors on local Fissionables).
5. **Colony events.** Choice Cards were all Earth-side or fleet-wide. Add colony cards: dust storm at Valles, a dome breach at Tycho, a birth boom.

**f. Conflict and war, and more interesting spaceships**
1. **Space was never contested.** In four games no ship was ever threatened; all fighting was Armies on Earth. Let a blockade of a Moon or Mars low orbit be a real lever: a Frigate parked over Tycho stops the shuttle. The Threats line exists; give the AI a reason to make one.
2. **Ships with histories.** Ship names are a joy (TSV Magellan, ARK Challenger, PMV Beagle). Track each hull's voyages and Colonists carried, show "veteran of 6 Moon runs" on its card, and list ships in the Chronicle.
3. **Escort and piracy at launch windows.** Mars windows are predictable (every ~13 turns), so a rival could intercept at the arrival orbit. That makes the window a convoy moment.
4. **Nuclear hulls should matter earlier.** Nuclear Rockets came on T24-T25 in G2 and G4, after the frontier was settled. Fissionables also live mostly at Mars, the moons and the Belt, which ties the tech to the frontier.

**g. My extra dimension: the people of the colonies (a population story)**
The game counts Colonists as a number. My persona wants them to be people: a colony's mood, its first native generation, its pull toward independence. Suggestions:
1. A colony Morale or Unrest figure like a Region's, driven by crowding, Energy shortfalls and births. My G4 Moon going dark would then have mattered beyond yields.
2. Second-generation Colonists (born off Earth) counted apart, unable to return to Earth, and worth more toward off-world bars.
3. A late "Independence" pressure for big colonies (Tycho had 33 people in G4), echoing the Mars trilogy, where a neglected colony can declare itself neutral.
Why: every win in this build is a bar of numbers. The first-child Moment was the one time the people felt real, and it was the best moment in all four games.

### 6. Rating: **6 / 10**

From the colonization-fantasy view: the bones are excellent and the moments land. There is a real solar system with real places and launch windows, named ships, races to each Body, the first child born off Earth, and a drowning Earth that eats your coastal cities while you build Tycho. Those beats would make me tell a friend about the game. But in all four games the frontier was settled by turn 15-20 with one Moon colony and one shuttle. Two of the four wins were decided by Earth levers (Scrubbers, compound interest). The AIs never contested space, and I won all four games before turn 31 without ever feeling threatened. My persona would close the game around turn 20 each time, when space went quiet, and pick it up again for the Arkwrights. That game (three ships in flight, Mars and Venus the same turn) is the game this could be for 50 turns. Give space a second act, make the AI launch, and let Earth's collapse be a chapter rather than the end, and this becomes an 8.
