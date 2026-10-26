# Playtester 2 notes (casual 4X player: Civ V/VI normal, some Stellaris/Endless Space)

## Highlights so far
- Results: G1 Arkwrights WON T25; G2 Archivists WON T27; G3 Custodians no win (65%, 5.6 ppm short of the Sink after T30); G4 Prospectors no win at T30 (82%, Fund 4,696/6,500 at +402/turn, ~T34 projected).
- Exploit: a Trade Post/Exchange Module scales with its colony's people: +51 then +77 Ducats a turn on the Moon (China's economy is +9.6). Every win ran through it.
- Won the Moon race in all 4 games (T6-T8); the AI never contests it.
- Top frustration: rules that bite with no warning at order time. Fuel at 0 quarters every Power Plant (3 of 4 games); founding needs 4 Colonists (found out at Mars, lost first-to-Mars); a crowded colonist dies silently; the sea ate both Arkwrights Launch Sites (G1 T19).
- Accords lapse every 5 turns; I missed renewals in all 4 games and got attacked after lapses (Indonesia G1, India G2/G4). War only ever happened TO me (6 attacks, 0 by me).
- Custodians' win depends on the whole world's Emissions; the AI Prospectors doubled theirs and one Labour Dispute card switched off 7 Scrubbers (-17 Sink).
- G4: home Region China flipped to the Arkwrights by Influence (T29) with no warning; bought 57 Influence with Ducats and took it back the next turn.
- Text bugs: raw debug in ferry lines ("Load { ship: ShipId(34) ...}") and the Report ("[exodus_call_order]"); `reactor` vs `fissionreactor`; the EVENT line shows last turn's card.
- Rating 6.5/10.

## Game 1: seed 1201, Arkwrights (China)

- T1: Read README, help, skimmed the glossary (it's 1000 lines of dev history; as a casual I'd never read it). `show` is ~400 lines. LOST: where do I even look? The SIGNATURE MOVES line at the top is the most useful thing on the screen: "build a station, recruit Pioneers, Shipyard on it, Colony Ship to the Moon". I followed it.
- T1: China lists a "Spaceport" but the Report says "you hold a Launch Site". LOST for a second: is the Spaceport my Launch Site? (Glossary says yes, it's my unique one; the state line never says so.)
- T1: Orders: tech Expanded Habitats, station at Orbital Reef, recruit 3 Pioneers, Research Lab in China, 10 Influence on Japan (33 to take). All accepted first try. Nice: the turn echo prints the cost of each line.
- T2: EXCITED small: station built and 4 colonists already aboard (the gift Pioneers lifted by themselves?). Expanded Habitats done in one turn because research is a SHARED pot and the Custodians led. FRUSTRATED/LOST: my Research Lab cost 27.5 Materials and 3 Energy upkeep and makes 0.7 Research. Energy income went from +3 to -1. Feels like a trap build; nothing warned me.
- T2: The three AIs all signed non-aggression with each other on turn 2, nobody with me. Noted.
- T3: LOST/alarm: "On this course the world collapses on turn 33" by turn 3 (it was 41 on T1). CO2 454 -> 533 ppm in two turns. Nobody told me what to do about it; as a casual I don't know which of my buildings is the problem.
- T3: Recruiters card (4 free Pioneers): easy yes. Bought Materials and a Mine; Materials are the bottleneck every turn early (6.6/turn vs 28-33 a build).
- T4: I was the Research Lead and got a shortlist of 3. Nice small choice. Picked Closed-Loop (my win path).
- T5: EXCITED: Japan flipped to me by Influence on turn 5 (4 x 10 points). Income doubled (Materials 13 -> 27). That felt great, clear and Civ-like.
- T5: FRUSTRATED (small): took the Cheap Ore card then tried to spend the 30 Materials the same turn: refused, the ore lands at Resolution. The card text doesn't say "next turn".
- T6: Colony Ship built. Accords "end after this turn" after only 4 turns: had to renew all three by hand. BORED chore.
- T7: EXCITED: load 7 + transit Moon worked, Ship arrived the same turn, and the Spaceport paid +7 Influence for loading. Nice surprise.
- T8: EXCITED: "Arkwrights win the race to the Moon", +5 Influence, Unrest eased. Best moment so far. But only 4 of 7 Colonists could land (room 4); 3 stuck aboard. LOST: I didn't know a new Colony only has room 4 until it happened.
- T8: LOST: slot figures "[M 1.25 G 1.34 R 0.59 S 0.96 Fi 0.23]": M is Materials, Fi Fissionables; G and S are never spelled out in the text output or in mouseovers.md that I could find.
- T8: Sent the 3 leftover Colonists Moon -> Mars (7.8 Fuel, 5 turns: cheaper than from Earth) and a 2nd ship with 6 to Venus. Fuel is now my bottleneck (stock 9).
- T9: India flipped to me (Influence) and came with a FREE Launch Site. EXCITED. But my emissions jumped 10 -> 23 and the panel now says collapse on TURN 29. I'm the biggest emitter at the table, by far, from owning big populated Regions. LOST + worried: growth is the thing killing the planet and I have no obvious lever.
- T10: FRUSTRATED (big): built a Colony Ship that took my Fuel to 0.2. Next Resolution "Fuel ran short; [all 5] Power Plants ran at a quarter" and my Energy went from +15 to -33 a turn. The order echo said "After them: 0.2F" and nothing else; Power Plants burning Fuel is nowhere on the build line. One line in the Report under "Your works" is the only clue. Rule I never saw coming.
- T11: FRUSTRATED: `build facility india reactor` refused: the cost table and the README call it "Reactor", the order wants `fissionreactor`. The refusal does list the valid names, which saved me.
- T11: The refusal list offers me investmentbank, accumulator, spaceport (other Factions' uniques) as if I could build them.
- T12: FRUSTRATED (big): "Sea rose at +1.8 C: you lost a Power Plant and a Factory (China), a Factory and a Mine (India), a Factory and a Mine (Japan), a Factory (Vietnam)". 7 buildings gone in one turn. The state line said "(coastal 0)" which I read as "nothing coastal"; it actually means no FREE coastal slot. Coastal Engineering (Sea Wall) was in the very first tech list and I had no idea it mattered. Felt like being punished by a rule I never saw.
- T12: Victory 26%. Venus station founded (Ishtar) by unloading into a ring. Ferry route set up: a nice anti-micromanagement tool. But the turn echo prints raw debug: "route of ARK Resolution: Load { ship: ShipId(34), colonists: 7, from: State(SouthAsia), army: None }".
- T12-13: LOST: climate. Collapse turn bounces 26-30. Net +54-66 ppm a turn vs a Sink of 8; a Nature Reserve adds 1. I built one in Vietnam anyway; it is a rounding error. My emissions are the biggest at the table because I own China+India+Japan+Vietnam (people emit). Owning populous Regions is both my strength (Pioneers) and the planet-killer; nothing on screen says which of my things emit how much (the window's Climate Panel may break it down [hover covers, partly]).
- T13: FRUSTRATED (big): my first Ship reached Mars (5 turns) with the 3 Colonists that didn't fit on the Moon, and `unload ... into slot mars 2` was refused: "needs 4 Colonists". Also refused for a station ring. Nothing told me at send time. Two turns later the Custodians landed 4 on Mars and took "first to settle Mars" (+15 Influence) while my Ship sat in orbit. That's the moment a casual player swears at the screen.
- T14: `lift china 3 orbitalreef` refused "no room" on the same turn I loaded 8 OUT of that station: room is checked before the load frees it.
- T15: FRUSTRATED: Terra Nova "0/8" capacity; I loaded 8 and "1 of 1 crowded aboard died on the way to the Moon". The load line did not warn me the 8th was crowded or that crowded people die. Ouch.
- T15: Generation Ships (my gate) on the shortlist: picked it. Mothballed two Power Plants (I have 120 Energy banked and a fuel-free Reactor) to save Fuel and emissions.
- T16: EXCITED: "the first child born off Earth, at Mare Imbrium". Lovely flavour beat.
- T17: The sea took 6 more buildings. Accords lapsed because I missed the "ends after this turn" line again; when I re-offered on T17 all three refused (the help says a lapse counts as a refusal). FRUSTRATED: renewing every 5 turns is a chore, and missing it is punished.
- T19: FRUSTRATED (big): the sea took my Spaceport (China) AND India's Launch Site in one turn. As the Arkwrights, my whole engine is lifting people; suddenly 19 Pioneers were stuck on Earth. I rebuilt a Spaceport inland in Kazakhstan (the only Region of mine with free slots) - a real decision, I liked solving it, but I'd never have guessed the Launch Site was coastal.
- T19: Generation Ships done (I was Research Lead 4 times; the shared-tech/lead shortlist is good). Victory now "held: needs a Colonial Capital". Ladder: Outpost -> Settlement (12 Colonists, all slots built) -> Colony (18) -> Colonial Capital (all slots but one, 77 Materials). Clear once I read the `next upgrade` line, but "Settlement (12 Modules)" read as "needs 12 Modules"; it means the tier HAS 12 slots. LOST briefly.
- T20: "Modules 12/10" printed for Mare Imbrium (more built than slots?). Odd display.
- T21: EXCITED + EXPLOIT?: one Trade Post module on the Moon pays +51 Ducats a turn (China's whole economy is +9.6). By T24 it paid +77. That single 13.8-Material module funded my Capital (I bought 57 Materials with it over 3 turns). Looks far too strong; seems to scale with the Colony's people.
- T21: Energy went -16 to -22 a turn as the Moon colony grew (every Module has upkeep). A Reactor module fixed it. Fine, but I only noticed by reading the income line.
- T22: Buying in bulk raises the price (25 Materials cost 105, not 100). The refusal told me; fine.
- T23-24: Fram unloads 10 at Venus, Terra Nova founds Roche on Phobos (first to Phobos, +20 Influence), Capital bought at Mare Imbrium. EXCITED: the last 4 turns were a real race against the climate clock and the collapse turn kept sliding out (net emissions fell from +66 to +15 as the Custodians built Scrubbers and the seas ate everyone's factories).
- T25: WIN. Endeavour lands 7 at Olympus Mons, Mars, Arcology chosen, "Game over, turn 25: Arkwrights win". Same turn the Prospectors invaded Indonesia the moment our Accord lapsed (two Armies from Australia). I never saw that coming either; it didn't matter.

### Game 1 summary
Outcome: Arkwrights WIN on turn 25 (Victory 100%: 60/44 Colonists, 4 Bodies, Capital at Mare Imbrium). The AI never seriously contested my space side; Custodians beat me to Mars by two turns.

## Game 2: seed 1202, Archivists (China again)

- T1: Same start Region as game 1 (China) - seat 0 seems to always get China unless `--start`. No SIGNATURE MOVES line this time in `show` (the Arkwrights got one in game 1's opening). LOST for a moment: what's my opening?
- T1-T4: EXCITED: I led the shared Tech four turns running and each lead paid +5 Influence. LLM, Coastal Engineering (learned from game 1!), Civil Defense, Orbital Data Centers each finished in ONE turn - the whole table's Research pours into whatever the Lead picked. Being the Lead is a lovely little reward loop.
- T2: FRUSTRATED: three Accord offers were waiting for me but `show` never prints incoming offers; I only learn about them when End Turn is refused. Same in game 1.
- T3: Hard Winter card: paid 40 Ducats rather than +2 Unrest everywhere. Clear choice, good card.
- T4: sold 20 Fuel to the Fuel Contract card for 40 Ducats... then T5 "Fuel ran short; Power Plant in Japan ran at a quarter". FRUSTRATED: same trap as game 1, my fault but the game never warns at order time that Power Plants eat Fuel.
- T5: Colony Ship capacity is 4 for the Archivists (no Expanded Habitats yet) vs 7 for the Arkwrights. Shipyard cost me 38.5 Materials (Arkwrights 28.9).
- T6: Moon race won again (+5 Influence). Superconductors done.
- T7: The Upload picked (I was Lead). Set fund-archive (100%). Accords lapsed AGAIN because the "ends after this turn" line scrolled past. Re-offered.
- T8: Coastal Engineering made every neutral Region build a Sea Wall at once (13 lines of "X built a Sea Wall"). Built one in China myself to protect my Launch Site.
- T10: The Upload complete at T10 (Custodians' pick finished it). Archive fund 41/250 at ~14 a turn. Vietnam flipped to me. Victory 12%.
- T10-12: Energy piled up to 268 with nothing to spend it on: Energy can't be sold (`sell` takes materials|fuel|fissionables). Meanwhile Fuel ran dry. FRUSTRATED: let me sell Energy or convert it.
- T12: "EVENT: Salvage Rights" printed in the T12 report header was LAST turn's card; I answered it and got "No card is asking anything this turn". LOST: the EVENT line under HEADLINE describes the previous card, not this turn's question.
- T13: Mothballed 3 Power Plants (Energy glut): my emissions fell 20 -> 15 and Fuel stopped draining. A rare climate lever I could actually pull. Nice.
- T13-16: Two Colony Ships on ferry routes Earth <-> Moon, 4 a trip. Ferries really are the anti-micromanagement tool I want. But "0 Colonists disembarked into Mare Tranquillitatis" and "waits: no room" lines when Habitats were full.
- T15: `build module ... habitat` refused "full: upgrade to a Settlement for more" while my upgrade sat in the queue behind a Factory. The queue spends Widgets in order given, so my upgrade waited 2 turns. LOST: I didn't expect the upgrade to queue behind modules.
- T17: Sea Wall in India built just before the T17-19 sea rises: I lost only 2-3 buildings this game vs 21 in game 1. Learning paid off; but it's a rule a first-timer learns by being robbed.
- T18: `build module ... solararray` refused "a Solar Array stands only on a station". Fine refusal, but the cost list doesn't say which Modules are station-only.
- T19: Custodians attacked and occupied India (the Accord had lapsed). Next turn India revolted from them to neutral and its Refineries and Lab were destroyed. I had no warning and no Army to answer with. War happens TO you, never by you, in this game so far.
- T20: Archive fund full (250) on T20 with 100% Directive and 4 Research Labs (~23 a turn).
- T21: Colony tier reached; `build archive` at once; bought a Habitat with Ducats (module-ducats = twice the Materials, instant). The Trade Post again paid ~+49 Ducats a turn: I ended the game with 490 unspent Ducats.
- T23: Archive complete. `upload ... 16` refused: "four a turn". FRUSTRATED (mild): nowhere in `help` or the Archive text did it say 4 a turn; it turned a "win now" moment into 4 turns of pressing End Turn. BORED: T23-26 were pure waiting.
- T25-26: End Turn twice refused because I "hold the Research Lead and owe the table a Tech" - my status script missed the pick. In a hurry I let the first-listed Tech be picked: Generation Ships, the Arkwrights' gate. Oops; a Lead pick helps a rival's win.
- T27: WIN. "Game over, turn 27: Archivists win".

### Game 2 summary
Outcome: Archivists WIN on turn 27 (Archive 250/250, 16/16 uploaded). One Moon colony did everything. I never left the Moon.

## Game 3: seed 1203, Custodians (China again)

- T1: SIGNATURE MOVES line is back: "Build a Scrubber at home and spend your Influence. Leapfrog a Region you hold and recruit Pioneers." Clear and useful.
- T1-2: EXCITED: first Scrubber in China moved the Natural Sink 8 -> 11.5 (+3, plus an "Objective met: +0.5 ppm" bonus). China may hold 7 Scrubbers (one per 200M people), India 8. Finally a climate lever with a visible number!
- T4: Japan by Influence again (4 turns). T6: Moon race won AGAIN (3 for 3: the AI never contests the Moon in the first 6 turns).
- T6-12: One Scrubber (33 Materials) roughly every turn. Sink 11.5 -> 29.5 by T12. But world Emissions bounce 46 -> 61 -> 50 -> 60 from cards (Methane, Volcano) and the rivals building. The Victory line "30.7 ppm over the Sink" moves both ways by 10 a turn, so I can't tell if I'm winning. LOST: the run needs FOUR turns in a row under; with that much noise it feels like a lottery.
- T7: Leapfrog China (50 Ducats). Nothing in the Report or my status said what it did; I only know it was charged.
- T10: "emissions by seat" sums to 44.6 but world Emissions are 61.3: the other 17 are neutral Regions and the permafrost, which I can't touch. FRUSTRATED: a third of the problem is out of every Faction's reach.
- T11: Wrote myself a wrapper to auto-answer the AI's trade offers each turn (3 offers in 4 turns, each one blocks End Turn). A window player would hit a modal each time. BORED.
- T13: Sea rose at +1.8 C and took 6 buildings: I'd skipped Coastal Engineering this game (no Lead pick came round). Same punishment as game 1.
- T14: Planetary Stewardship (gate) done. The Custodians' Research Directive "into the Natural Sink" pays 0.020 ppm a point: my 4 points a turn = +0.08 ppm. A Scrubber is +3. The Directive is a trap/decoy; nobody would notice at 0.08.
- T15: The Rival line printed a raw key: "Arkwrights [exodus_call_order], recruited 3 Pioneers in Canada". UI bug.
- T17: EXPLOIT again: one Trade Post module at an 18-Colonist Moon colony took my Ducat income from +1.4 to +41 a turn. Third game in a row the Trade Post is the economy.
- T18: China "holds its 7 Scrubbers already" - good, clear cap message.
- T19: 20/20 Colonists beyond Earth orbit by T21 with ONE colony ship on a ferry. That half of the Custodians' win is trivial.
- T19-28: The other half is not. World Emissions climbed 57 -> 78 as the Prospectors grabbed Regions by Influence (their own emissions 25 -> 46). My Sink went 43 -> 72 with 19 Scrubbers, but the gap never closed. FRUSTRATED: my win depends on what the Prospectors do, and the game gives me no tool that touches their emissions. I tried taking Europe from them by Influence: their Standing went 64 -> 104 -> 263 as I pushed. Hopeless; I gave up. (The CONTEST question from the README: yes, they outbid me on Europe, I saw it coming in the numbers after one turn, I backed off.)
- T25: FRUSTRATED (3rd game!): the ferry refuelled from my stockpile, Fuel hit zero, all 5 Power Plants ran at a quarter, Energy -48 a turn and my Scrubbers were about to go offline. I had to kill the ferry. The ferry route should never spend the Fuel my Power Plants need, or should warn.
- T30: "Labour Dispute in China: its Facilities make nothing" - my 7 Chinese Scrubbers stopped for a turn and the Sink fell 63 -> 46 (+28 net). One card wiped 17 ppm of my win condition; a 4-turn run can be broken by a single draw. Next turn it was back: gap 5.6 ppm, the closest all game.
- T31 (after my 30th orders): 5.6 ppm over the Sink, run 0 of 4, Victory 65%. As a casual player I'd keep going to see if I can close it; this is the most tense the game got.

### Game 3 summary
Outcome at T30: no win. Victory 65% (28/20 Colonists done; Stabilization 5.6 ppm over the Sink, run 0 of 4). The climate never collapsed ("On this path Collapse is not reached").

## Game 4: seed 1204, Prospectors (China again)

- T1: SIGNATURE MOVES: "Build an Investment Bank and set your Venture share. Raise Industry Level and recruit Pioneers." Investment Bank built T1 (Cheap Industry: 23.4 Materials).
- T2: LOST: the Fund's interest is "a hundredth of the balance per payer" (glossary), so 2 Banks = 2% a turn. 6,500 needs ~215 a turn for 30 turns. I did that maths on T2 and my heart sank.
- T3: Japan by Influence on T4 again; T7 Moon race won again (4 for 4 games; the AI never beats me to the Moon).
- T7: Accords lapsed AGAIN (all three, fourth game running). I keep missing the "ends after this turn" line in a 60-line Report.
- T7: `build module ... tradepost` built an "Exchange" (the Prospectors' unique). Then `... exchange` was refused: "you already hold a Trade Post at the Moon; one per Body". LOST: the order word, the built name and the refusal use three names for one slot.
- T8-9: Fuel at 7: ferry stuck at Tiangong with 12.6 in the tank because the stockpile couldn't refuel it. Fuel is the chronic bottleneck in every game; Refineries make 4 a turn and every Ship wants 32.
- T10: Fund 114 at +21 a turn. BORED/LOST: the bar is 6,500. At this rate it's T300.
- T11: Strip Permit on China: Materials 36 -> 61 a turn. EXCITED briefly; then my emissions became the table's highest (20.5) and the collapse forecast came in to T33-34.
- T14-18: The Fund snowballs: interest is 1% per Investment Bank/Exchange. Banks in China, India, Japan, Vietnam, Indonesia, Australia + the Moon Exchange = ~7% a turn compounding. +21 (T10) -> +124 (T18) -> +236 (T19, with a 100 Objective reward) -> +415 (T28). EXCITED: watching the number climb is the most "satisfying build-up" in any of the four games.
- T16: Answered a card "answer take/refuse" but my status filter hid the third option ("answer third: Send them to the stars"). Fine in the game, my mistake.
- T17: Exchange is "one per Body": I founded a Venus station purely to put a second Exchange on it (+1% interest). That's a fun, readable puzzle.
- T21-27: India ping-pong: the Custodians' Iranian Army attacks and occupies India (my inherited Indian Army dies each time, strength 1 vs 4), then India returns to me "(Influence), after Custodians took it (Occupation complete / Pacified)", then they attack again. Three times. LOST: why does a completed occupation hand the Region back to ME? Feels like a bug or a loop.
- T26: Collapse forecast T33 and I'm the top emitter. Mothballed 7 Facilities (Materials 117 and Fuel 181 were piling up anyway): my emissions 20.5 -> 15.9, collapse slid to T35-38. Each mothball costs +1 Unrest, which Relief (10 Ducats) buys back; with 300 idle Ducats that's trivial.
- T28: The Extraction Charter (gate) picked by me as Lead and done the same turn. Victory 74%.
- T29: FRUSTRATED (big): "China now belongs to Arkwrights (Influence)". My HOME Region, with my Launch Site, Investment Bank and Reactor, flipped silently. Its Unrest was 9.5 and my Standing 28 while I'd been feeding Influence to Japan. Nothing ever warned "your home is being contested" in what I read (the window may show a Threat line). T30: bought 57 Influence (256 Ducats) and took it straight back (Standing 96). EXCITED to have the tool; annoyed I needed it.
- T30: the Arkwrights attacked and occupied Indonesia in retaliation (2 Chinese Armies, strength 12 vs my 5).
- After T30: Fund 4,696/6,500 at +402 a turn (214 of it interest). On this pace the win lands about T34; collapse forecast T38. I'd absolutely play on.

### Game 4 summary
Outcome at T30: no win yet. Victory 82% (Fund 4,696/6,500, 63/20 Colonists, Charter done). Projected win ~T34 if the climate holds (collapse forecast T38).

## Final report

### 1. Per-Faction summary

| Game | Faction | Outcome at T30 (or end) | Victory figure |
|---|---|---|---|
| 1 (seed 1201) | Arkwrights | **WON on turn 25** | 100% (60/44 Colonists, 4 Bodies, Capital at Mare Imbrium) |
| 2 (seed 1202) | Archivists | **WON on turn 27** | 100% (Archive 250/250, 16/16 uploaded) |
| 3 (seed 1203) | Custodians | No win by T30 | 65% (Colonists 28/20 done; 5.6 ppm over the Sink after my 30th turn, run 0 of 4) |
| 4 (seed 1204) | Prospectors | No win by T30 | 82% (Fund 4,696/6,500 at +402 a turn; Charter done; ~T34 projected, collapse forecast T38) |

- **Arkwrights**: the most "4X" game of the four: a real space race (Moon T8, first to Phobos, a Venus station, Mars), a ferry loop, and a Capital to buy at the end. The middle was spoiled by rules that bit without warning: Fuel ran out and quartered my Power Plants, the sea took 21 buildings including both my Launch Sites, and a Ship flew 5 turns to Mars with 3 Colonists and couldn't land because founding needs 4. Once the Moon colony had a Trade Post it paid for everything.
- **Archivists**: a tidy, readable plan: lead the Tech, fund the Archive at 100%, grow ONE Moon colony to Colony tier, build the Archive, upload. Being the Research Lead (+5 Influence each time) was a lovely loop. The last four turns were pure End Turn because uploads are 4 a turn (nowhere in `help`).
- **Custodians**: the only game where the climate was a lever I could pull: each Scrubber +3 Sink, and China holds 7. But the win depends on the WHOLE world's Emissions, and the Prospectors AI doubled theirs (25 -> 46) by grabbing Regions. A random Labour Dispute switched off my 7 Chinese Scrubbers for a turn (-17 Sink). It felt like a lottery I couldn't influence, though it ended tense (5.6 ppm short).
- **Prospectors**: the most satisfying number-goes-up game: Investment Banks and Exchanges compound the Fund (1% each), and I founded a Venus station purely for a second Exchange. Spoiled by my home Region China flipping to the Arkwrights by Influence on T29 with no warning I saw, and by the Custodians' Army ping-ponging India three times.

### 2. Lost / bored / excited (strongest moments)

**Lost**
- G1 T3: "On this course the world collapses on turn 33" by turn 3, with no breakdown of which of my things emit. (The window's Climate Panel may break it down [hover covers, partly]; the text `show` does not.)
- G1 T8: slot figures "[M 1.25 G 1.34 R 0.59 S 0.96 Fi 0.23]": G and S are never explained in `show`, the README or mouseovers.md.
- G1 T19: "Settlement (12 Modules) at 12 Colonists": I read "(12 Modules)" as a requirement; it's the new tier's slot count.
- G2 T12: the "EVENT:" line under the HEADLINE names LAST turn's card; I answered it and got "No card is asking anything this turn".
- G4 T21-27: a Region occupied by the Custodians comes back to me "(Influence), after Custodians took it (Occupation complete / Pacified)". Why does a finished occupation return it to me?
- G3 all game: the Stabilization gap swung by 10 a turn from cards and rivals; I couldn't tell if I was winning.

**Bored**
- G2 T23-26: four turns of `upload ... 4` and End Turn.
- Every game: renewing three Accords every five turns by hand; answering AI Trade offers that block End Turn (G3: 3 in 4 turns).
- G4 T2: doing the Fund maths (6,500 at +11 a turn) and seeing T300.

**Excited**
- G1 T8 "Arkwrights win the race to the Moon"; G1/G2 "the first child born off Earth" (lovely flavour beat).
- G1 T25: the final 4-turn race: Colonial Capital bought, first to Phobos, Endeavour lands on Mars, "Game over, turn 25: Arkwrights win".
- G2 T1-4: four Tech leads in a row, each finishing in one turn, each +5 Influence.
- G3 T1: first Scrubber moves the Sink 8 -> 11.5: a climate number I can push.
- G4 T14-28: the Fund's growth line climbing (+21 -> +415 a turn, 233 of it interest by T30).
- G4 T30: buying 57 Influence with 256 Ducats and taking my home China back in one turn.
- Influence-flipping Japan on T4-5 in every game: clear and Civ-like.

### 3. Frustrations (ranked)
1. **Rules that punish with no warning at order time.** (a) Power Plants burn Fuel: any order that takes Fuel to about 0 (a Colony Ship build, a ferry refuel) quarters every Power Plant next turn (G1 T10 Energy +15 -> -33; G2 T5; G3 T25 Energy -48 with Scrubbers about to go offline). Happened in 3 of 4 games. (b) "needs 4 Colonists" to found, discovered at Mars after a 5-turn flight (G1 T13), which cost me first-to-Mars. (c) Loading 8 into a "0/8" ship silently killed the crowded one (G1 T15).
2. **The sea.** G1 lost 21 buildings over three rises, including my Spaceport AND my other Launch Site (T19), the Arkwrights' whole engine. "(coastal 0)" in `show` reads as "nothing coastal" but means "no FREE coastal slot". The slot hover explains thresholds [hover covers], but nothing says WHICH standing Facility is coastal.
3. **Accord upkeep.** Every 5 turns all three lapse unless renewed; a missed renewal makes all three refuse you and invites attack (G1 T25 Prospectors invaded Indonesia the turn ours lapsed; G2 T19 Custodians took India). In the window an expiring Accord is "owed" before End Turn [hover covers, partly]; the text tool never blocked, so I missed it in all four games.
4. **Custodians' win is out of the player's hands** (G3): neutral Regions and the permafrost are a third of world Emissions, and the AI Prospectors' growth swamps 19 Scrubbers. One card (Labour Dispute) can wipe 17 ppm of Sink for a turn.
5. **Order vocabulary drift.** `reactor` vs `fissionreactor` (G1 T11); `tradepost` builds an "Exchange", then `exchange` is refused as "a Trade Post" (G4 T7); refusal lists offer other Factions' uniques (investmentbank, accumulator, spaceport) to me.
6. **Home Region flipped silently** (G4 T29) while I fed Influence elsewhere.
7. **Energy glut with no outlet** (G2: 268 Energy; there is no `sell energy`) while Fuel starved.
8. **Hidden caps**: uploads 4 a turn (G2 T23); one Trade Post/Exchange per Body (G4 T7); one Barracks per Colony; Solar Array station-only (G2 T18). All learned by refusal.

### 4. Exploits
1. **Trade Post / Exchange scales with the Colony's people (strong).** Repro: G1, build a Trade Post Module (13.8 Materials with Regolith Printing) at a Moon colony of about 18 Colonists: +51 Ducats a turn at once, +77 at 23+ Colonists (China's whole economy is +9.6). G3 T17: Ducat income +1.4 -> +41 in one turn. G4: the Exchange version grows the Fund too. It funded my Arkwrights Capital, my Archivists' Habitats (490 Ducats unspent at the end) and Custodian Leapfrogs. Strength: game-defining; every win ran through it.
2. **Ducats -> Influence -> Regions (medium-strong).** `buy-influence 57` (256 Ducats) plus a 19 allotment flipped a 76-to-take Region in one turn (G4 T30). With exploit 1 feeding Ducats, Regions are for sale.
3. **The Moon race is free (medium).** I won it in all four games on T6-T8 with a straight build: Shipyard T1-2, Colony Ship T4-5, land next turn. +5 Influence and -0.5 Unrest everywhere, every game.
4. **Mothball + Relief (mild).** Mothballing an emitter costs +1 Unrest; Relief buys it back for 10 Ducats. G4 T26: 7 mothballs cut my Emissions 20.5 -> 15.9 for 40 Ducats of Relief.
5. **Occupation loop (bug?).** G4 T21-27: the Custodians occupy India; when the occupation completes India returns to me "(Influence), after Custodians took it (Occupation complete)", with a fresh inherited Army to die again. Three times. I didn't exploit it, but it looks like a bug.

### 5. Suggestions

**a. Making the game more interesting and compelling**
1. **A pre-End-Turn warnings list** (solves the top frustration: G1 T10, G1 T13, G1 T15, G3 T25). One line each: "Fuel will be 0: 5 Power Plants run at a quarter", "Magellan carries 3: founding needs 4", "1 of 8 aboard is crowded and may die", "Accord with X lapses next turn".
2. **Make the Custodians' climate win something they can force** (G3). Give them a tool that hits OTHER Factions' Emissions (e.g. a Carbon Tariff: Influence spent on a rival's Region cuts its Emissions), or count only Factions' Emissions for the run, not neutrals and the permafrost.
3. **Show a "Contested" chip on my own Region** when a rival's Standing passes half its take price (G4 T29 China lost).
4. **Fix the Archivists' ending** (G2 T23-26): let one `upload 16` order run itself over the turns, or make each batch a Moment.

**b. Balance: bolstering the Arkwrights' and Archivists' wins**
Both won first in my hands (T25, T27), so the human versions are not weak; the AI versions are (G3/G4 AI Archivists: Archive 106/250 at T30; AI Arkwrights: no Capital by T30).
1. **Archivists AI: fund at 100% from the turn The Upload is picked, and build Research Labs** (G3/G4 the AI banked about 3 Research a turn; I made about 23 a turn with 4 Labs in G2).
2. **Arkwrights: let them found with 3 Colonists** (fits Coach Class). Solves G1 T13, where 3 stranded Colonists cost me first-to-Mars.
3. **Arkwrights: start their Spaceport inland, or give it a free Sea Wall.** Losing both lifts on T19 (G1) is the one thing most likely to lose an Arkwrights game.
4. **Archivists: 6 uploads a turn, or 4 plus 1 per Habitat at the Archive's colony.** Shortens G2's dead stretch without moving the bar.
5. **Cap the Trade Post first** (exploit 1), then re-measure both, since my wins leaned on it.

**c. UI fixes and text reductions**
1. Show incoming Accord/Trade offers in `show` (G2 T2: I only learned of three offers when End Turn was refused).
2. Ferry debug text: "route of ARK Resolution: Load { ship: ShipId(34), colonists: 7, from: State(SouthAsia), army: None }" -> "Resolution loads 7 at India".
3. Raw key in the Report (G3 T15): "Arkwrights [exodus_call_order], recruited 3 Pioneers" -> "Arkwrights called an Exodus, recruited 3 Pioneers".
4. "EVENT:" under the HEADLINE shows last turn's card (G2 T12): label it "Last card:" or drop it.
5. Victory line: "Colonists beyond Earth orbit: 0/44 | 0 of 4 Bodies with 4 Colonists or more | score 0.00 (held: needs Generation Ships, not yet researched) | gate: Generation Ships not yet" -> "Off Earth 0/44 | Bodies 0/4 | Gate: Generation Ships".
6. Climate: "Last turn to act: 34" + "On this course the world collapses on turn 41." -> "Collapse T41 (act by T34)".
7. Tier: "next `upgrade iss`: Settlement (12 Modules) at 12 Colonists, 33 Materials and 4 Widgets" -> "Next: Settlement (12 slots): 12 Colonists, all slots built, 33 M".
8. Archive: "Archive fund: 250 of a cap of 250; the Archive is complete at Mare Tranquillitatis on the Moon, where 33 Colonists stand to be uploaded; 0 remembered here" -> "Archive 250/250, complete. 33 can upload, 4 a turn."
9. Ship: "would arrive stranded at: Mars (3.6 Fuel left, nothing of yours to refuel at in low orbit); Phobos (1.1 Fuel left, ...); Deimos (...); Ceres (...)" -> "No fuel home from: Mars, Phobos, Deimos, Ceres".
10. "Colony Ship capacity: 7 safe, 7 crowded" -> "Colony Ship: 7 seats (+7 crowded, risky)", and say "crowded" on the load echo.
11. "(coastal 0)" -> "(0 coastal free)", and mark coastal Facilities (e.g. "~Mine") in the list.
12. Slot letters "[M 1.25 G 1.34 R 0.59 S 0.96 Fi 0.23]": spell out G and S once in the FREE SLOTS header.
13. One name per thing: accept `reactor`; let `exchange` and `tradepost` both work for the Prospectors; drop other Factions' uniques from my refusal list.

**d. Making the AI smarter**
1. **Contest the Moon.** In 4 games no AI landed before my T6-8. They build Shipyards T1-3 and then wait; send the first Colony Ship the turn it exists.
2. **Custodians AI: stop the India loop** (G4 T21-27): three attacks on a Region that returns to its old owner when the occupation completes is wasted Armies (or fix the rule).
3. **Prospectors AI: don't pour Influence into a Region already held at 263 Standing** (G3 T28 Europe). The counter-bid itself was smart; the overspend wasn't.
4. **Archivists AI: fund the Archive harder and build Labs** (see 5b.1).
5. **Keep**: Arkwrights AI taking my neglected home China at T29 and attacking the turn an Accord lapsed; Custodians AI building Scrubbers and Leapfrogs (G1). Those were the AI's best moments.

**e. Bolstering the colonization side**
1. **Pre-flight founding check** (G1 T13): the transit echo should say "arrives with 3: needs 4 to found".
2. **Fuel**: the chronic bottleneck in all four games (a Refinery makes 4 a turn; a Ship wants 32). Let a ferry draw only from a reserved Fuel pool, or let a station turn Energy into Fuel. That fixes G2's 268 idle Energy and G3 T25's crisis together.
3. **Give each Body a reason**: one Moon colony did everything (G2 never left the Moon). Make Mars/Phobos/Ceres' slot figures matter (an economy that only exists there), and explain G/S.
4. **Habitat-room warning on unload**: "only 4 of 7 fit" before the order, not "3 more found no Habitat room" after (G1 T8).

**f. Conflict and war, and making spaceships more interesting**
1. **War only happens TO you.** In 120 turns I never attacked and was attacked 6 times (Indonesia G1 and G4, India G2 and three times in G4) with no warning and no counter. Show a threat line on my Region when a rival Army sits next door ("Iranian Army (Custodians, str 4) borders India"), and let me raise an Army there in one order.
2. **Lapsing Accords as a declared state**: auto-renew non-aggression between Friendly sides; make a lapse read "Accord lapses: Custodians may attack".
3. **Spaceships need a peacetime job**: escort a Colony Ship (protection from intercept), blockade a rival's Moon colony to choke its Trade Post income (a direct counter to exploit 1), or raid a rival ferry for Fuel. I built zero warships in four games because nothing needed one.
4. **Missile Technology was researched in G2 and G4 and nobody fired one.** Make the "whole table sees it" moment matter: a Relations hit or a card.

**g. One more dimension: ONBOARDING (a first-game guided path)**
The game's best moments (Moon race, first child, Lead picks, the Fund snowball) all exist, but a casual player meets them only after being robbed by Fuel, the sea and the 4-Colonist rule. The SIGNATURE MOVES line is the best text in the game; extend it into a 5-step objective chain per Faction for the first 10 turns (e.g. Arkwrights: "Station; Shipyard; Colony Ship with 4+ aboard; Sea Wall your Spaceport; Refinery before your second Ship"). That one change would have prevented my top three frustrations.

### 6. Rating: 6.5 / 10
As a Civ/Stellaris-on-normal player: the bones are good. Four distinct win paths that each made me think differently (a space race, a research monument, a planet to clean, a fund to grow), shared research with a Lead pick, Influence-flipping Regions, ferries that remove busywork, and a few genuinely warm moments ("the first child born off Earth"). I won two of four, and both felt earned. But the game hides its sharp edges: Fuel quietly powers my grid, the sea quietly eats my Launch Sites, founding quietly needs 4, Accords quietly lapse, and my home Region quietly flipped. I'd play another game (the Prospectors' Fund at 4,696 and climbing had me wanting turn 31), and I'd tell a friend "great ideas, read the glossary first", which is exactly the friend who won't. Fix the warnings and the Trade Post and this is an 8.
