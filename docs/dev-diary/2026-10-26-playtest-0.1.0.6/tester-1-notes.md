# Playtester 1 notes (persona: common gamer)

## Highlights so far
- RESULTS: Arkwrights WON T26, Archivists WON T27, Prospectors 87% at T31 (win due T32-33), Custodians 76% at T31 (Stabilization run 2 of 4, win due T33). All four Factions won or were set to win by turn 33 of 50 - the human beats the AI easily.
- The Arkwright and Archivist wins were UNCONTESTED: no rival ship or Army ever came near my off-Earth colonies. The AI Arkwrights in G4 sent their Pioneers to Antarctica (Earth - doesn't count) and had 0-7 Colonists off Earth at T23-T31.
- Research is far too fast: the whole 29-Tech tree was done by T25-T30, two Techs a turn. As Archivists my own gate (The Upload) was researched on TURN 8. Gates are no obstacle.
- Best moments: winning the race to the Moon (+5 Influence), first on Mars, watching the Prospectors' Venture Fund compound (+40 -> +648 a turn), taking the Prospectors' HOME Region by Influence as Custodians (40 points, one turn), getting the world under the Sink.
- Worst friction: Accord and Trade offers block the turn but `show` never prints their terms; one stray/invalid line refuses the WHOLE order list; multi-word names (`powerplant`, `fissionreactor` for "Reactor"); "not available yet" for a Tech that is already DONE.
- The Collapse forecast is a jittery alarm: G1 went "collapses on turn 40" -> 26 by T6 -> "not reached" by T30. Planning against it is impossible.
- AI dog-piles the leader's capital with Influence every game (China at T25-T30 in G1, G3, G4); in G3 it took China on the turn I won. Custodian AI attacks your weakest Region the turn a non-aggression Accord lapses (G1 T13, G3 T13).
- A Colony Ship's 32 Fuel build cost can empty the Stockpile and starve the Power Plants with no warning at the order (G2 T8 Power Plants at a quarter, G3 T5 0 Fuel).

## Pre-game
- README is fine, short. `help` is a WALL: ~200 lines of order grammar in rules-lawyer voice ("A LANDING OR FOUNDING IS SHUT ONLY BY A BLOCKADE OF LOW ORBIT, never by a Ship on Hold, never by a Passage partner"). A common gamer bounces here. FRUSTRATED.
- `show` is ~40KB a turn: every Region, every travel time Body-to-Body. Most of it is noise for me. The SIGNATURE MOVES line at the top is the single most useful line in the whole thing. EXCITED (small) - it told me what to do.

## Game 1: seed 1101, Prospectors (China)
- T1: Signature moves told me: Investment Bank + Venture share + Industry + Pioneers. Did all four, influence 13 on Japan. Tech tree via `show --techs` is actually clear: a 4-rung chain to my gate (Extraction Charter). Goal is clear: 6,500 Ducats in fund + 20 colonists off Earth. 6,500 looks impossibly far when my income is 11.5 D. LOST (how is that reachable?).
- T1 tech: I picked Deep Mining and it finished at once; then the AI picks. Took me a glossary search to find out the Tech is SHARED and the "Research Lead" picks. LOST then interesting.
- T2: Shipyard on Tiangong. Turn refused to end: three Accord offers, and `show` never says WHAT the offer holds (which terms). I accepted blind. LOST/FRUSTRATED. (They turned out to be non-aggression.)
- T2->T3: climate line went "collapses on turn 40" -> 38 -> 34 in two turns. That's actually a nice dread clock. EXCITED. But "Last turn to act: 28" - act how? LOST.
- T3: Japan Standing 25/33 after two turns of Influence. Visible progress bar, good.
- T4: Japan flipped to me by Influence. EXCITED - first real "I got something" moment, and its Mines/Power Plant show up on my income at once. I became Research Lead and picked Automated Refining.
- T4: card "Surplus Habitats" passed me by because I had "no room for a Habitat" - every rival took it. Felt like being punished for not knowing. FRUSTRATED (mild).
- T5-6: Climate clock fell 40 -> 38 -> 34 -> 32 -> 26 in five turns. I am 43% of world emissions. The Signature Move "Raise Industry Level" on China (1.4 bn people) plus taking Japan and the UK roughly tripled my emissions (9.85 -> 27.9 a turn). The game's own suggested opener walks you into the Collapse. LOST/FRUSTRATED: nothing warned me that Industry Level multiplies people's emissions until I dug into a hover (mouseovers "Population {n}" covers it) [hover covers, but you only find it after the damage].
- T6: UK flipped. T7: Colony founded on the Moon, "Prospectors win the race to the Moon" +5 Influence, Unrest eased everywhere. EXCITED - best moment yet; that's the spectacle beat.
- T7: Lab in China makes +0.5 Research. A whole building for half a point? BORED/LOST.
- T8: Mexico flipped. Permafrost Thaw Break. `change facility eastasia power plant replace` refused: '"plant" is not one of: mothball...' - multi-word names must be squashed ("powerplant") or use the index. FRUSTRATED.
- T9: Objective met (+50 into Fund). Raised Venture share to 60%.
- T10: Green Consensus done (I was Lead) - emissions fell 67 -> 60 and Collapse slid back to T28. Fund 174/6500, +40 a turn. 6,500 looks unreachable before Collapse. LOST: is the Prospector win even possible in this climate?
- T11: Indonesia flipped to me. Five Regions now.
- T12: The Moon Exchange paid +39.8 Ducats a turn by itself - more than all of China. Trade network pays per colonist at the Body + per Body held. EXCITED (found the engine) / possible EXPLOIT (see report). Fund now +61/turn.
- T12: Sea took the Wind Farm I had JUST built in China (T10). Nothing on the build order warned me that slot was coastal and the sea was due. FRUSTRATED.
- T11: Custodians refused to renew our non-aggression Accord (`accord renew` refused: "they will not renew it"). T12 it lapsed; T13 the American Army marched into Mexico and occupied it. No warning beyond "ends after this turn". EXCITED (finally drama!) but FRUSTRATED: I couldn't answer - `move-army` "not a neighbouring continent" from China, and a fresh Army in Mexico is built from Mexico's own people and dies in one round.
- T14: Custodians first on Mars (+15 Influence). T15: Mexico flipped back to me by Influence "after Custodians took it (Occupation complete)" and then T16 they occupy it again. A ping-pong I don't understand. LOST.
- T16: Ice Sheets Committed Break; the sea took 7 of my Facilities in one turn (Mine in China, Power Plant + Factory in Japan...). Spectacular, actually - the "dying earth" is real. EXCITED + FRUSTRATED.
- T17: First child born off Earth (+5 Influence) on my Moon colony. Nice flavour beat. EXCITED.
- T18: Mexico lost for good ("Pacified"). The sea took 7 more Facilities, incl. my Investment Bank in Japan. Regions shrink: Japan 1 slot, China 4. Earth is literally drowning my economy - strong theme.
- T19: `build module ... reactor` refused: the order word is `fissionreactor` though the cost list, README and income all say "Reactor". FRUSTRATED.
- T19: Can't `load` from Vietnam into a station orbit (needs a Launch Site); low orbit needs none - but a Ship in low orbit can't refuel (T22: "tank holds 12.6 of 32; leg needs 15.7"). So: load in low orbit, change-orbit to the station, refuel+fly next turn. Three turns of fiddling to move 8 people 1 turn away. FRUSTRATED/BORED. (Ferry `route` exists but I didn't trust it.)
- T20: I had forgotten the Mars window (the report said "next turn" one turn earlier). Caught it on the turn. Terra Nova to Mars.
- T21-25: The Fund snowballs: 1,048 -> 1,202 -> 1,398 -> 1,626 -> 2,182 -> 2,561 -> 2,980 (interest = 1%/payer/turn; 8+ Investment Banks + 2 Exchanges ~ 8.5%/turn). This is the "one more turn" hook: watching compounding interest beat the climate clock. EXCITED.
- T24: "colonists 8/6" on the ship card meant 2 were CROWDED; one died in transit. I'd read 8/6 as a typo. LOST.
- T25: Report: "China could fall to Custodians next turn" - my capital. Great warning text! EXCITED. Next turn all three AIs poured Influence into China (Custodians 35, Arkwrights 60, Archivists 10). Dog-pile on the leader. I defended with 57 then 47 Influence (bought). Felt like a real fight. EXCITED.
- T25: I was Research Lead with my own gate on the shortlist -> picked The Extraction Charter. Done T26. Victory 64%.
- T24: Mars colony founded (Chryse Planitia) + Exchange there.
- T28: Collapse forecast slid T39 -> T46 -> "not reached" by T30: the sea drowning Facilities + Custodian Scrubbers (Sink 8 -> 35) pulled it back. The doom clock I panicked about at T6 (T26!) was a false alarm; the forecast swings wildly. LOST (can't plan against a number that moves 20 turns).
- T29: Arkwrights offered an Accord mid-dogpile; accepted (presumably non-aggression) - they stopped spending on China.
- T30 (stopped after 30 turns of orders): Fund 5,202/6,500, +648/turn (455 interest). Victory 87%. The win lands T32 or T33. China Standing 313 vs Custodians 182 - the AIs never stop pouring Influence on my capital.
- GAME 1 RESULT at T31 report: not yet won, 87%, a near-certain win in 2 turns. Felt GOOD: a clear number going up fast.

## Game 2: seed 1102, Arkwrights (China)  -- WON on turn 26
- T1: Signature move told me exactly: Station over Earth, Shipyard on it, Colony Ship to the Moon. `build station earth 3` completed the same turn. EXCITED - clean start.
- T2: three Accord offers again with no terms shown. Accepted blind. LOST.
- T4: Hard Winter card, 40 Ducats or +2 Unrest everywhere. A clear choice, fine.
- T5: Japan by Influence. T6: Colony Ship done; Spaceport gives +1 Influence per Colonist lifted - nice combo.
- T7->T8: Moon colony founded, race to the Moon won again, Objective: free Launch Site. 12%. EXCITED.
- T8: The Colony Ship's fuel came out of the Stockpile and left 0.4 Fuel: "Power Plants ran at a quarter". Nothing warned me at the order. FRUSTRATED.
- T9: Mars without a window: 5 turns. Took it anyway.
- T13: Prospectors offered a Trade; the terms are nowhere in `show` (only "offered Arkwrights a Trade"). Refused blind. LOST/FRUSTRATED (same as Accords).
- T14: First on Mars (+15 Influence -> allotment 43). First child born off Earth. 40%. EXCITED.
- T15: Flew 3 leftover Colonists to Phobos: "needs 4 Colonists" to found. Wasted a hop, flew back. FRUSTRATED (the rule only shows at the refusal).
- T16: a stray `answer take` with no card that turn refuses the WHOLE order list. FRUSTRATED (harsh for a harmless line).
- T17-20: I was Research Lead most turns and walked my own chain: Closed-Loop -> Relay -> Regolith -> Generation Ships (done T22). With the Lead picking from a shortlist that always holds my gate, the Arkwright chain is basically mine to take. EXCITED / EXPLOIT-ish.
- T19: `relay` built as a "Chorus" (Arkwright unique module?) - surprise, no explanation. LOST (mild).
- T20: Mars window: Terra Nova Moon->Phobos with 7 lifted OFF the Moon colony, Challenger Earth->Deimos with 9. T18 Magellan to Venus, founded a station (Ishtar) with `unload ... into ring venus 0`.
- T22: Energy -17/turn; module-ducats fixed it in one turn. Ducats buy anything instantly - strong.
- T23-25: Colony tier -> Capital. I misread "Colonial Capital (24 Modules) with 1 slot open" and built all 18 slots; `upgrade` refused "needs 1 open slot"; had to decommission a Battery. FRUSTRATED (cost me a turn).
- T25: Phobos + Deimos founded the same turn (first on Deimos). 44/44, 5 Bodies. 91% "needs a Colonial Capital".
- T26: Upgrade to Capital -> "Game over, turn 26: Arkwrights win". EXCITED, but also: that was EASY. Nobody contested a single landing; no AI ship ever threatened a colony. The Custodians and Prospectors attacked my UK on the very last turn - too late to matter.
- Note the brief says the designer wants to bolster the Arkwrights' wins: in my hands they won at T26 with zero conflict. The AI Arkwrights in game 1 had only 24/44 at T30, so the gap is AI play, not the Faction (see report).

## Game 3: seed 1103, Archivists (China)
- T1-T8: I was Research Lead from turn 1 and every Tech finished in ONE turn: LLM T1, Civil Defense, Efficient Grids, Orbital Data Centers, Superconductors, The Upload done by T8. My own gate open on turn 8 of 50. EXCITED but also EXPLOIT-feeling: the gate is no obstacle at all for whoever leads research early.
- T4: `tech civil defense` refused "that Tech is not available yet" - it was already DONE (someone's pick had completed it). Wrong wording. FRUSTRATED/LOST.
- T4: Refugee Convoy "Send them to the stars": 2 fit, 4 died, +2 Unrest. The third answer reads better than it plays.
- T5: Colony Ship took 32 Fuel from the stockpile -> 0 Fuel left. Again no warning.
- T6: Archivist Modules cost 27.5 not 23.4 (refused "needs 27.5 Materials, 25.2 left") - the cost list I'd read earlier (Prospectors) didn't match. Fine once known.
- T8: fund-archive at 100%: 22-28 Research a turn into the fund. Full (250) on T18. I then unfunded and was Research Lead again at once.
- T9-T22: the real Archivist game is the Moon colony climbing Outpost (6 slots) -> Settlement (12) -> Colony (18 colonists, all 12 built). That's ~11 Modules and three upgrades on 4-12 Widgets a turn. A Factory Module on the colony doubled its Widgets (4 -> 8 -> 12). This middle stretch was BORED: same ferry + one Module a turn, and the Moon colony's "Modules 5/6" counts without the Core so I thought it was full when it wasn't (T16). LOST.
- T13: Custodians again attacked Mexico the turn our Accord ran out (same as game 1). T14 Mexico "Pacified" - gone. Pattern: the Custodian AI's first move after a lapsed Accord is to march on your weakest Region.
- T17: I forgot Magellan sat loaded at Axiom for 4 turns - the report doesn't nag about idle loaded ships. LOST.
- T21-T25: by T25 the shared tree was nearly empty: 27 Techs done, two a turn, "free choice of everything" with 3 Techs left, and Missile Technology picked itself because nothing else was left. My `tech X` lines were "not available yet" for Techs that were already done. The research race is far too fast - the whole tree in 25 turns. EXCITED early, then the tree stops mattering.
- T23: Colony tier. `build archive` -> complete in one turn (T24).
- T24: `upload southpole 16` refused: "four a turn". Fine rule, but nothing said so before (the help says "Free, and irreversible"). FRUSTRATED (mild).
- T25-27: 4 a turn: 4, 8, 12, 16 -> "Game over, turn 27: Archivists win". On the same turn "China changed hands" - the Custodians took my capital by Influence (they had started at T25: "8 of 46"). Didn't matter.
- GAME 3 RESULT: WON T27. Like the Arkwrights, no rival ever touched the Moon colony. The Archivist win is a build-order puzzle, not a contest.

## Game 4: seed 1104, Custodians (China)
- T1: Signature move: Scrubber at home. Two Scrubbers in China, Sink 8 -> 11.5 (+0.5 from the Objective at once) -> 14.5. Clear, satisfying number. EXCITED (small).
- T1-T12: The Custodian economy is TINY: 7-13 Ducats, 15-33 Materials a turn. Every build costs more than the last (Shipyard 38.5, Scrubber 33) and I'm buying 2-5 Materials to scrape over each refusal. BORED/FRUSTRATED.
- The hover for a Scrubber gives "the yield text" - I had to build one to learn it is worth +3 Sink. [hover covers, if the yield text names it; the build list does not]
- T3: Carbon Offset: "THE OFFER IS CLOSED TO YOU: you have 25.8 Ducats of the 50" - good, clear wording.
- T5: Strike at the Refineries - refused: a Refinery stopping is GOOD for a Custodian. Nice when a card has a Faction twist.
- T8-12: World Emissions climb 42 -> 58 while my Sink climbs 8 -> 20.5. The gap (Stabilization "ppm over the Sink") never closes: 34 -> 33 -> 37.6. The Prospector AI issued a Strip Permit (T12). My win depends on THEIR emissions and I have no lever on them except Influence and the sea. LOST: what am I supposed to do?
- T12: `unload` refused "no room in its Habitats" the turn the Habitat was building - fine, but the colony that just founded with 4 has room 4 and nothing said so.
- T13-14: Trades every turn, terms never shown. I accepted one blind (Archivists): it turned out to be 20 Fuel for 30 Energy. You only find out from the stockpile change. LOST.
- T14: Research Directive 50% into the Sink: 5 Research -> "+0.10 ppm for good". Worthless next to a Scrubber (+3). Switched it off. BORED.
- T15: The sea took a Power Plant and Factories - for a Custodian the Sea Level Break is GOOD news (fewer emitters). Nice irony.
- T17: India by Influence = room for 8 Scrubbers (one per 200 M people). The Scrubber cap per Region is the Custodian's real engine. EXCITED.
- T19-21: Prospector Frigate PMV Valiant shuttles Earth <-> Moon every turn and at T28 "set its Ships at the Moon to Attack". Nothing ever happened to my colony by T31. First warship I saw in four games.
- T21: Accords lapsed -> `influence europe 53` refused "breaks Accord: order it with break". Did it with `break`. T22: "The European Union now belongs to Custodians" - I took the Prospectors' HOME Region with one turn of Influence (40 to take it). EXCITED - best moment of the game; but it was far too cheap for a capital. EXPLOIT? (see report).
- T22-26: Mothballed every Factory, Refinery and Mine I held, replaced Power Plants with Wind Farms: gap 13.8 -> 5.6 -> 3.3 -> 0.1 UNDER (run 1 of 4). EXCITED: finally I control the number.
- T22-25: Energy crisis: each Scrubber eats 4 Energy + 1 Ducat; at 14 Scrubbers I was -28 Energy a turn. Buying Energy got pricier each turn (2 -> 3 Ducats). FRUSTRATED but a real puzzle.
- T27: run 2 of 4 at 0.2 under... T28 0.2 OVER -> run back to 0. Prospectors' Strip Permit in Egypt pushed it. FRUSTRATED but in a good "so close" way.
- T27: The Upload completed with the Archivists leading - the tech race got away from me while I funded nothing; by T30 the WHOLE tree (29 Techs) was done.
- T28-30: Rivals pour Influence on China (Arkwrights 60 of 88) and Europe. Dog-pile on the leader again.
- T31 (30 turns played): 9.9 under the Sink, run 2 of 4, 36/20 Colonists, Victory 76%. Win due T33 if China holds.
- My `tech <first on list>` shortcut picked Missile Technology at T28 by accident. The order took it with no "are you sure, this opens weapons to everyone".

## Final report

### 1. Per-Faction summary

**Prospectors (G1, seed 1101).** Not finished at T31: Venture Capital Fund 5,202/6,500, +648 a turn, 62/20 Colonists off Earth, gate done T26. Victory 87%; the win falls due T32-33. At first it felt hopeless: 11 Ducats a turn against a 6,500 bar, and a Collapse forecast of T26 by turn 6. Then the Moon Exchange (+40 a turn by itself) and the Fund's compounding interest (1% per Investment Bank or Exchange) took over. From T20 every turn was "one more turn" because the number jumped by hundreds. The low point was the sea eating my Facilities and the Custodians taking Mexico twice. The high point was beating off all three AIs pouring Influence into China.

**Arkwrights (G2, seed 1102).** WON T26 with 44/44 Colonists on 5 Bodies and a Capital on the Moon. It's the cleanest Faction to play: the signature line tells you station, Shipyard, Colony Ship, Moon, and the "first to" bonuses (Moon, Mars, Deimos) keep coming. Nobody contested any landing. The only real obstacles were my own mistakes: 3 Colonists can't found a colony, and the Capital needs one slot left OPEN. Fun and quick, but there was no opposition at all.

**Archivists (G3, seed 1103).** WON T27: Archive 250/250 and 16 uploaded. I led research from turn 1, and The Upload, my gate, was done on TURN 8. The rest was a build-order chore: grow one Moon colony Outpost -> Settlement -> Colony on 4-12 Widgets a turn, then upload four a turn. T9-T22 was the most BORED stretch of all four games. Nobody threatened the colony. The Custodians took China by Influence on the turn I won.

**Custodians (G4, seed 1104).** Not finished at T31: 9.9 ppm under the Sink, run 2 of 4, 36/20 Colonists, Victory 76%; the win falls due T33. For 20 turns it felt out of my hands: world Emissions kept climbing faster than Scrubbers could grow the Sink. Then I took India (room for 8 Scrubbers) and the Prospectors' home EU with one turn of Influence, and mothballed every emitter I held. In four turns the gap went 13.8 -> 0.1 under. The T28 reset (0.2 over) hurt in a good way. The energy upkeep of Scrubbers is a real puzzle.

### 2. Lost / bored / excited (the strongest)
- EXCITED: G1 T7 / G2 T8 "win the race to the Moon". G2 T14 first on Mars, Influence jumped to 43. G1 T21-T30 the Fund compounding. G1 T25 "China could fall to Custodians next turn", then fighting off the dog-pile. G4 T22 taking the Prospectors' home Region. G4 T26 "0.1 ppm under the Sink, run 1 of 4". G1 T16 the sea drowning 7 Facilities: spectacular theme.
- LOST: Accord/Trade offers with no terms (every game). The research race: picks I made seemed to be overridden, and "not available yet" was shown for Techs already done (G3 T4, T23-24). "colonists 8/6" meant crowded, and one died (G1 T24). Collapse forecast swings (G1). The Moon colony's "Modules 5/6" doesn't count the Core, so I thought it was full (G3 T16). Pouring Influence into a Region I already held, with no notice (G4 T17-21).
- BORED: G3 T9-T22 (ferry + one Module a turn). G1 T19-T23 low-orbit/station-orbit/refuel fiddling to move 8 people. G4 T1-T12 scraping 2-5 Materials to clear each refusal.

### 3. Frustrations (ranked)
1. Accord and Trade offers block End Turn, but `show` never shows what is offered (all games; G2 T13, G4 T13 accepted blind = 20 Fuel for 30 Energy).
2. One bad line refuses the whole order list. A leftover `answer take` with no card (G2 T16) or a 2-Material shortfall (G3/G4 many times) stops the turn.
3. Naming: `change facility eastasia power plant replace` was refused ('"plant" is not one of...'), and the order word `fissionreactor` is used where every text says "Reactor" (G1 T19). `relay` silently becomes a "Chorus" (G2 T19).
4. Tech order wording: "that Tech is not available yet" for a DONE Tech (G3 T4, T23; G4).
5. The Capital rule "with 1 slot open" cost a turn (G2 T25), as did "needs 4 Colonists" to found (G2 T15) and "four a turn" uploads (G3 T24). Each rule only shows up at the refusal.
6. Ship logistics: no refuel in low orbit, no lift to a station orbit without a Launch Site, and change-orbit costs the Ship's action (G1 T19-T22).
7. The sea took a Wind Farm I'd built two turns before, with no coastal warning at the order (G1 T12).
8. The Colony Ship's 32-Fuel build cost emptied the Stockpile and Power Plants ran at a quarter, with no warning at the order (G2 T8).

### 4. Exploits
- **Research is a sprint, and the Lead decides.** Reproduce: G3, Archivists, `tech large language models` T1, then take your own chain every pick: The Upload is researched T8. Strong: all four gates opened by about T15-T22, and the whole tree was done by T25-T30 in every game.
- **Taking a capital by Influence.** Reproduce: G4 T21, Custodians: Europe (the Prospectors' home, Industry 5) showed "40 to take it from the Prospectors". `influence europe 53 break` took it in one turn. Very strong: it removes a rival's home economy and emissions. The AI only spent 10-15 a turn on it.
- **Exchange network income.** Reproduce: G1, Prospectors, Moon colony + Exchange T10. It paid +39.8 Ducats a turn at once and grew with colonists, and its interest share feeds the Fund. Strong. It's the Prospectors' only real path to 6,500, so it may be intended.
- **Mothball everything (Custodians).** Reproduce: G4 T22-T25: mothball every Factory/Refinery/Mine you hold, replace Power Plants with Wind Farms. World Emissions fell about 5 at no Materials cost. Medium. It's thematic, but there's no Unrest or economic penalty I could see.
- **Uncontested off-Earth wins.** Arkwrights/Archivists: no AI ever attacked an off-Earth colony, so these wins are pure build orders. Strong (G2 T26, G3 T27).

### 5. Suggestions

**a. More interesting and compelling**
1. Make a rival contest your win. When a Faction passes 60% Victory, the AI should target its win condition: blockade its colony, smear it, take its Capital Body (G2, G3 had zero opposition). This solves the missing endgame tension.
2. Slow research a lot (two Techs a turn by T20 in G3/G4). Scale Tech cost with the turn, or make rung 4 cost 150+. Gates should land T20-30, not T8 (G3).
3. Make a "Victory meter" pop at 25/50/75% with a one-line "next step" ("Needs a Colonial Capital: upgrade South Pole-Aitken"). The G2 T26 "91% - needs a Colonial Capital" line was the best guidance in the game; do it at every step.
4. Keep the "race to X" Moments and add more. They were the most exciting beats (Moon, Mars, Deimos, first child).

**b. Balance: bolstering the Arkwrights' and Archivists' wins**
In my hands both won easily (T26, T27). The weakness is the AI, not the Faction:
1. AI Arkwrights: stop sending Pioneers to Antarctica. In G4 they had 0/44 off Earth at T23 with 10+ Pioneers landed at Lake Vostok and Ross Ice Shelf. Antarctica doesn't count toward their win. Weight Colony Ships to the Moon/Mars, and make the AI found on 4 Bodies (Phobos/Deimos are 1 turn from Mars).
2. AI Archivists: in G4 they had the Archive fund at 247-250 from T23 and The Upload from T27, but no Colony-tier colony to build it in. Have the AI grow one ground colony to Colony tier early, starting while it funds. The fund wait is the free time.
3. If the human Archivist is too fast: cap uploads not at 4 a turn but at the Archive's tier (Colony 2, Capital 4). Or make The Upload rung 5. Either way the gate should not open on T8.
4. Arkwrights: the Capital step is the only hard step. Keep it, but say "leave one slot open" on the Colony line: "Colonial Capital: 17 of 18 built, 1 open".

**c. UI fixes and text reductions**
1. Print the offer under every pending Accord/Trade: "Archivists offer: 20 Fuel for 30 Energy" / "Accord: non-aggression".
2. Refuse lines, not turns: run the legal lines and list the refused ones, with `--strict` for today's behaviour. Or at least don't refuse on a harmless `answer` when no card is up.
3. Tech refusal: "that Tech is not available yet" -> "Done already." (for a done Tech).
4. Accept the display names: "power plant", "reactor", "research lab".
5. Ship card: "colonists 8/6" -> "8 aboard (2 crowded: may die)".
6. Colony card: "Modules 5/6" -> "5 of 6 built (Core not counted), 1 free".
7. Text cuts (quote -> proposal):
   - help: "A LANDING OR FOUNDING IS SHUT ONLY BY A BLOCKADE OF LOW ORBIT, never by a Ship on Hold, never by a Passage partner." -> "Only a low-orbit Blockade stops a landing."
   - help (build archive): "the Archivists only, at a Colony OFF EARTH (neither Antarctica nor a station over Earth) that has climbed to the Colony tier. Both are checked at the order and never again. The Upload gates the Archivists' WIN, not this order." -> "Archivists: at a Colony-tier ground colony off Earth."
   - help (refuel): "FREE OF THE SHIP'S TURN: give the refuel first and the transit after it, in one turn; it fills before it goes." -> "Free; refuel then fly in one turn."
   - Report: "Custodians led and pick next" (every turn) -> drop it when the Lead isn't you; say "You pick next" when it is.
   - Break text: "The ice sheets let go. Greenland and Thwaites pass the point of return; the sea is coming for the coasts and it will not stop." -> "Ice sheets lost: coasts will flood."
8. Collapse line: show a range or a trend arrow ("T26-T40, improving") instead of a single turn that jumps 15 turns (G1).
9. Warn at the order when a build empties the Stockpile below upkeep: "leaves 0.4 Fuel: Power Plants will run at a quarter" (G2 T8).
10. Warn when Influence goes into a Region you already hold well past safe: "Middle East: you hold it at 113; no rival above 36" (G4).
11. A coastal flag on the build list: "Wind Farm (coastal slot: sea due at +1.8 C)" (G1 T10-12).

**d. Making the AI smarter**
1. Don't send Arkwright Pioneers to Antarctica (above).
2. Build toward the Colony tier for the Archive (above).
3. Defend your home. G4: the Prospectors lost the EU (their capital) to one turn of my Influence and only answered with 10-15 a turn after.
4. The dog-pile on the leader's capital is good. But aim it at the win condition, not just the capital: in G3 they took China on the turn I won by uploading on the Moon.
5. Use warships. The Prospectors' Frigate in G4 shuttled Earth <-> Moon for 10 turns, refuelling, and never attacked. Either strike undefended colonies or don't build it.
6. The Custodian AI's "attack Mexico the turn the Accord lapses" (G1 T13, G3 T13) is good drama. But it was predictable two games out of two. Vary the target.

**e. Bolstering colonization**
1. Make ferry routes the default and show them: "Magellan ferries China -> Moon: 8 a trip, every 2 turns". I never trusted `route` because nothing said what it would load from. And I forgot a loaded ship at Axiom for 4 turns (G3 T17). Add a report nag: "Magellan idle, 6 aboard".
2. Allow refuel in low orbit over your own Launch-Site Region, or let a load into a station orbit work from any Region you hold (G1 T19-22).
3. Give each Body a distinct reason to go: Mercury and Ceres were never worth it in 4 games. Make the Exchange's per-Body pay visible on the FREE SLOTS list ("Exchange pays +8 a turn for a Colony here").
4. Let 3 Colonists found an Outpost (or say "needs 4" on the Ship card before you fly) (G2 T15).

**f. Conflict, war and making spaceships interesting**
1. In four games I saw one warship (a Frigate), zero ship battles, and two land battles (Mexico x2). War is a rumour. Give each Faction one reason to build a warship by T15: a Blockade that halves a rival Exchange's/Trade Post's income would do it.
2. Show threats in the report: "PMV Valiant (Frigate, Attack) is in low orbit over your Moon colony". The Colony card names threats, but I never saw one rise to the report.
3. Escort/intercept should be one order: "escort <warship> <colony ship>", so a fleet means something on the Moon run.
4. Make "Missile Technology" a decision. I researched it by accident with my first-on-the-list shortcut (G4 T28). A Tech that "the whole table sees" deserves a confirmation line or a Moment: "Missile Technology: weapons are open to everyone."
5. Let Armies ride Colony Ships (or a cheap Carrier at the start) so an occupied Region across an ocean can be answered. I couldn't help Mexico from China (G1 T14: "not a neighbouring continent").

**g. One more dimension: ONBOARDING / A GUIDED FIRST GAME**
Why: a common gamer reads the signature line and nothing else. That line was the single most useful text in four games. Everything after it is a 40 KB wall plus 200 lines of help. The game is really fun from about turn 15, and most of my persona's friends would quit by turn 5. Make the first game a guided goal ladder: "1. Take Japan by Influence (14 a turn, 3 turns). 2. Build a Shipyard. 3. Land on the Moon. 4. Reach 25% Victory." Show one next step, a progress tick, and a short reward Moment for each. Use the existing Custodian tutorial notes, but for every Faction and tied to the Victory meter.

### 6. Rating: 6.5 / 10
As a common gamer: the first five turns are rough. The help wall, a 40 KB board, and turns that won't end because of offers I can't read. I'd have quit at turn 3 without the signature-move line. But once I had a colony on the Moon and a number going up (the Fund, the Colonist count, the ppm under the Sink), it hooked me hard. G1's compounding Fund racing the climate, and G4's crawl under the Sink with a reset at 0.2 ppm, were real "one more turn" stuff. The drowning-Earth theme is strong: watching the sea eat Facilities every few turns sells it. It loses points because the rivals don't fight for your win (two clean wins by T27), research finishes the whole tree by mid-game, and a lot of friction is wording, not design. I'd tell a friend who likes Civ: "Try it as Prospectors, it's weird and good after turn 15."
