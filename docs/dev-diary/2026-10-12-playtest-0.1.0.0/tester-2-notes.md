# Playtest notes - tester-2 (casual 4X player persona), build 0.1.0.0

## Game 1 - Custodians, seed 201, start The United States

- T1 setup: README talks about double-clicking dying-earth.exe and a 3D map; I have play.exe in a text console. Bit disorienting, fine.
- T1 CONFUSING: task said `help` lists region ids - it doesn't. Had to feed a bogus `--start` to get the list.
- T1 CONFUSING: the region has two names. `--start "the united states"` works, but in orders `united` silently resolves to the UK ("you do not direct this Nation State"), and `states`/`usa` are rejected. The real order id is `northamerica` - which I only found by noticing the left column of `show`. The error should list the ids orders accept.
- T1 FRUSTRATED: no price list anywhere. `help` lists facility/module names only via an error message, and costs I learned by issuing orders and reading "After them: 64M". No description of what a Scrubber, Stadium, Constabulary, Chorus etc. *do*. As a Civ player I'd kill for a "build menu" view.
- T1 DELIGHT: refused orders don't end the turn, and the "After them:" stockpile preview is a great dry-run. I'm using it as my build menu.
- T1 the help text is full of "Ticket #337 (version 0.09.0)" developer notes - reads like a changelog, not a manual.
- T2 BROKEN-ish: three rivals offered me Accords, but `show` never mentions the offers or their Terms - I only learned when the turn refused to end ("STILL OWED"). Even then it doesn't say WHAT they're offering. Accepted blind.
- T2 `tech efficient` ambiguous between efficientgrids/efficienttransit - fine, good error.
- T2 Economy feels tiny: +4 Materials a turn from my one Mine; a Habitat is 25. Energy net 0 already after two builds.
- T2 Custodian win needs the WHOLE TABLE's emissions (43.8) under the Sink (9). My own are 5. Feels out of my hands on day one.
- T3 EXCITED: took Mexico by Influence in two turns. T5 took Japan too. Influence-flipping neutral countries is the most Civ-like fun so far.
- T4 CONFUSING: `build station earth 4` refused "needs 4 Colonists" while ISS held exactly 4. Doesn't say where those 4 must be.
- T5 my own emissions went 4.6 -> 13.6 in four turns just from building Mines/Power Plants - as the green faction! There's no per-building emissions figure shown, so I couldn't see this coming. Collapse forecast appeared: turn 47.
- T8 Archivists lifted 8 Pioneers in one turn while I'm capped at 2/turn per state. Don't know how; nothing tells me.
- T9 EXCITED: India flipped to me (pop 1,751!). Five countries by turn 9. Influence expansion is the strongest thing I do.
- T9 Accords I "accepted" on T2 never appeared; they show struck on T8. Not sure the first acceptance did anything.
- T10 FRUSTRATED: spent 24 Influence on Indonesia, and the Prospectors took it the same turn. I can only see MY standing in a country, not rivals' - no warning that I was racing someone.
- T10 Energy piles up (87) and there's no `sell energy`. Feels like a dead resource once positive.
- T11 EXCITED: Planetary Stewardship (my win gate) showed up on the shortlist and I led research, so I took it. Also first to settle the Moon (+5 Influence, unrest eased everywhere) - a nice "you did a first!" moment.
- T12 I hit 19/18 Colonists off Earth by turn 12 by just lifting 2 a turn to the ISS. That half of the Custodian win was trivially easy - all the difficulty is in the emissions half.
- T12 Unloaded 6 to found a Moon colony; only 4 fit ("2 more found no Habitat room") - would like the order to warn me up front.
- T12 My Standing in my own countries decays every turn (US 50 -> 39). Not explained; worry a rival will steal them.
- T13 ANGRY: my three Accords (struck T8) lapsed at T13 with NO line in the Report, and the very same turn the Prospectors marched into Kazakhstan and destroyed its army. I only worked out the Accords had gone by diffing the Accords line. README asks "did you notice one lapse?" - no, I noticed the invasion. Needs a "your Accord with X expires next turn - Renew?" warning in `show`.
- T13 the Kazakhstan Scrubber I paid 30 Materials for the turn before is now behind an occupation, "building: (out of sight)" in my own country.
- T14 BUG?: turn blocked by "Prospectors/Arkwrights/Archivists offer Custodians an Accord", but `accord accept` says "you already hold an Accord with them" (struck the same turn). Had to `accord refuse` offers for accords I already have. Still never told what Terms were on offer.
- T15 CONFUSING report: headline "Kazakhstan now belongs to Prospectors (Occupation complete)" and two lines later "Kazakhstan now belongs to Custodians (Influence)". Which is it? (Power Plant destroyed either way; my Energy is now -13.5 a turn.)
- T15 Planetary Stewardship done; Sink 29 vs world emissions 72, of which the Prospectors alone are 39 and rising. I have no lever on THEIR emissions beyond Blame/smear (which is bookkeeping, not CO2). Stabilization in the next 10 turns looks impossible; feel like a spectator on my own win condition.
- T16 The Prospectors attacked Kazakhstan AGAIN two turns after a non-aggression Accord was struck with them (T14). Their Accord has simply vanished from the list - no "Prospectors ended the Accord" line I could see. Then they spent 20 Influence on MY home, the US. Feels like the AI ignores the deal and I'm never told.
- T16 Brazil flipped to me: 8 countries. I can't see rivals' Standing anywhere, so I don't know how close the Prospectors are to taking the US. Just dumping Influence home defensively.
- T18 BROKEN: two rivals "offer Custodians a Trade" and the turn won't end until I answer, but NOTHING anywhere says what they're giving or asking. Refused blind. A trade I can't read is just a nag.
- T18 ENGAGED: Deep Mining + Moon mine pushed Materials to 70/turn. Worked out each Scrubber is ~+3 Sink for 30 Materials, so ~15 Scrubbers closes the gap. First time the Custodian win felt like a plan rather than a prayer. Fuel piles up unused (87) - selling it to fund Materials.
- T19 EXCITED (in a grim way): the +1.8 C sea-level break ate coastal slots across the whole world in one Report, two Factories of mine included. Great dramatic moment, the best "the world is dying" beat so far.
- T19 Widgets cap: small countries make 5 Widgets a turn, so stacking two Scrubbers in Brazil just queues them. Must spread builds - fine once understood, but `show` doesn't summarise "where can I build fastest".
- T22 `leapfrog` (my Faction's own special order) costs 50 Ducats and nothing anywhere says what it does. Report afterwards: "India: its people now emit 0.07 per hundred million" - but not what it was before, so I can't judge if it was worth it.
- T22 GOOD: Accord renewals arrived as offers this time and I accepted. But the Prospectors one (struck T16) expired silently again and on T23 they took JAPAN by Influence - with two Scrubbers and a fresh Nature Reserve and Power Plant I'd just paid for. Gap went from 12.75 back to 22. Most deflating moment of the game.
- T22 ENGAGED: the net-over-Sink figure dropping 46 -> 24 -> 15 -> 12.75 turn on turn was really satisfying feedback. A clear number to push down.
- T24 THE TURN I FELT I'D LOST: India (pop 1,700, three Scrubbers, a Nature Reserve, a Leapfrog I paid 50 for) flipped to the Prospectors by Influence. Every Sink I build in a country is just a gift to whoever out-Influences me next. The Prospectors are a runaway: 46 of the world's 75 emissions, ~12 countries, and nothing in my kit slows them. I refused their Accord to try retaking Japan; that probably cost me India.
- T25 end state: 33/18 Colonists off Earth, Sink 42 vs world emissions 84 (Prospectors alone 60). Prospectors' Fund 3,287/3,500 - they'll win within a turn or two. It went 61 -> 3,287 between T12 and T25; nothing in the Report flagged "the Prospectors are about to win" until I looked at the Victory block myself.
- T25 179 Materials stockpiled with nothing useful left to build: the per-country caps (2 Scrubbers, 1 Nature Reserve) and losing countries mean my late economy has no sink to pour into.

### Game 1 summary (Custodians)
Strong, fun start: Influence-flipping neutral countries (Mexico, Japan, Vietnam, India, Kazakhstan, Argentina, Brazil) felt like Civ city-states, and the colonist half of the win was done by T12. Then the game became "build Scrubbers and watch a number". It got that number from 46 down to 12.75 by T22 - genuinely engaging - and then the Prospectors took Japan and India by Influence (after silent Accord lapses) and the gap went back to 42. Ended at +1.96 C watching a computer Prospector about to win. Felt like I'd lost at T24.

## Game 2 - Prospectors, seed 202, start The European Union

- T1 Prospector builds are ~15% cheaper (21.3 vs 25) - nice, but discovered only from the order echo.
- T2 Accord acceptance worked first time this game (in game 1 my T2 accepts apparently didn't stick).
- T4 Took the UK in 2 turns with bought Influence. Investment Banks: 21 Materials for +8.5 Ducats a turn, and I can stack several in one country - feels like the obvious engine.
- T5 CONFUSING: the T5 Report header says "EVENT: Conscription Notice" (last turn's card), so I wrote `answer take` and got "No card is asking anything this turn". The EVENT line describes the card just resolved, not a new one.
- T5 Strip Permit doubled Europe's Materials (37.5/turn) for free, at a permanent Baseline Emissions cost. No downside I personally feel. Smells degenerate.
- T8-9 BUG?: I sent `accord accept custodians` (renewal offer); the order echoed "answered: you accept", yet no Accord with the Custodians was struck - and that same Resolution the Custodians marched two Armies into my Russia. Same thing happened in game 1 T2. If an acceptance can fail, the Report must say so and why.
- T9 Turkey flipped to me (4 countries by T9). Buy-influence at 2 Ducats a point is clearly the strongest lever in the game for a rich Faction - I'm converting fuel sales straight into countries.
- T11-T15 Russia ping-pong: Custodian Armies occupy it, I buy it straight back with Influence, every other turn. Each Report has paired lines "Russia now belongs to Custodians (Pacified)" / "Russia now belongs to Prospectors (Influence)". Lost three Armies trying to fight back; Armies feel like coin flips I don't understand (strength 8 vs 5, "attacked at 94% to hold the field" - what?).
- T13-15 The collapse forecast keeps falling: turn 48 -> 45 -> 43 -> 39 -> 37. I'm half the world's emissions (Strip Permits, Investment Banks). Interesting tension: my win is a race against the collapse I'm causing. As a casual I'm... honestly ignoring it, because nothing I'm told says what one Investment Bank emits.
- T15 Extraction Charter done (my gate). Fund 570/3500 at ~50 a turn - 60 turns at this rate. In game 1 the computer Prospectors reached 3,287 by T25, so there's some engine I'm not seeing.
- T16 CONFUSING: Tiangong refuses Habitats and an Exchange: "full: upgrade to a Settlement for more". There's no `upgrade` order in `help`. How?
- T18 EXPLOIT-ish: occupation is meaningless against a rich Influence buyer. Custodians occupy Russia every single turn; I re-own it by Influence the same Resolution, and it's back to "Prospectors" for my Income. Military conquest by AI is just noise. (Also the AI keeps throwing one Army at it forever.)
- T18 EXPLOIT: `sell materials` -> `buy-influence` at 2 Ducats a point converts my surplus straight into countries. 90 Influence in one turn flipped Vietnam out from under the Archivists. With no cap on buy-influence the richest Faction just buys the map.
- T19 Fund passed 1,000 at T19 (~120/turn now). The game is now: build Investment Banks wherever there's a slot, buy Influence with the rest. Engaging in a spreadsheet way, not a story way.
- T21-22 EXCITED: real race now. Fund 1,466 at +170/turn says I win ~T34; the collapse forecast says the world ends T35 - and I'm 43 of the 90 emissions. Best tension of the session. Took India and Argentina by Influence.
- T24 OOPS/LESSON: lost my HOME, the EU, to the Archivists by Influence while I was pouring everything abroad. No warning in the Report beforehand ("your Standing in X is falling / a rival is investing in X") - just "now belongs to Archivists". Bought it back T25 with 90 Influence.
- T25 `--force` does NOT skip a "STILL OWED" card/tech pick - fine, but my turn silently didn't run and I nearly didn't notice.
- T25 end: Fund 2,324/3,500 at ~+250/turn -> win around T30. 11 countries. Collapse forecast T43.

### Game 2 summary (Prospectors)
The most "4X" of the games: grab countries with Influence, stack Investment Banks (no per-country cap, 21 Materials for ~8.5 Ducats), Strip-Permit everything, sell surplus, buy more Influence, repeat. Clear feedback (Fund number climbing faster each turn), and the T21 race against my own collapse forecast was the best moment of the session. But it's a snowball with no brake: buy-influence is uncapped, occupation by the Custodians was pure noise, and the only threat was my own neglect of home Standing. Likely to win around T30.

## Game 3 - Arkwrights, seed 203, start South Africa

- T1 AH-HA (and retroactive frustration): `build station earth 3` worked instantly with my 4 waiting Pioneers. So "needs 4 Colonists" in game 1 meant 4 *Pioneers waiting in a Region*, not Colonists on a station. The error should say "needs 4 Pioneers waiting in a Region with a Launch Site".
- T1 Arkwrights start with 1 Ducat income and no station. Very poor.
- T5-7 Spaceport says "+6 Influence next turn", "+3 Influence next turn" after each lift, but my Allotment stayed at 8 every turn. Either it's hidden somewhere or it isn't paid.
- T7 FRUSTRATED: my first Colony Ship finished ON the Mars window turn, but a Ship takes one order a turn, so I can't `load` and `transit` together - window missed, and off-window Mars costs 40 Fuel against a 35 tank. Nothing warned me the ship needed to be done a turn earlier. Would love a combined "load-and-go" or loading not counting as the ship's order.
- T7 Shipyard queue: the Solar Array I ordered after it jumped ahead of my Colony Ship? (No - I ordered the array first. Queue order = order order. My fault, but a "reorder queue" would help.)
- T14 PATTERN across all three games: the computer Prospectors snowball. This seed they're 72 of the world's 106 ppm emissions by T14, own ~8 countries, and are occupying the Custodians' China. Collapse forecast dropped to T32. Every game so far, the Prospectors AI is the story and the other three seats (me included) are reacting to it.
- T12 "a Solar Array stands only on a station" - ground colonies can't have one; fine, but which modules work where isn't listed anywhere.
- T14 BUG: "STILL OWED" listed "Custodians offer Arkwrights a Trade" and "Archivists offer ... a Trade", but `trade accept cus` / `trade refuse arc` were refused: "Custodians have offered you no Trade". Phantom trades in the owed list.
- T16 Accepted a Custodian Trade blind (curiosity). Report says nothing about what moved. Still no idea what I traded.
- T20 EXCITED: two Colony Ships arrived the same turn, Mars and Venus. Then...
- T20 BROKEN (Venus): my ship sits "at Ishtar over Venus" with 9 Colonists. `unload 25 9 into slot venus 0` -> "Move this Ship to Venus, low orbit, then found the Colony next turn: a Colony is founded from low orbit alone." `change-orbit 25 low` -> REFUSED "Venus has low orbit and 3 Orbital Slots, numbered 0 to 2" (?!). `transit 25 venus low` -> "already there". `build station venus 0` -> "a Colony Ship in this orbit founds it". The FREE SLOTS list shows Venus has NO low orbit at all. Four orders, four contradictory answers; Venus can't be settled from where I am. 9 Colonists and a ship stranded with 19.8 Fuel, not enough to come home. This is the single most frustrating moment of the playtest.
- T23 I'd filtered the sea-level lines out of my reading because they're 20 lines long every time... and missed that the sea ate South Africa down to ONE slot: Spaceport, Mine, Factory, Refinery all gone, pop 522 -> 359. Found out when `lift` said "needs a working Launch Site". My own home's losses should be in "Your works", not buried in a 20-country list.
- T24 Lost South Africa itself to Prospector Influence (no Accord standing with them by then - again lapsed without me noticing).
- T25 end: 60/42 Colonists off Earth, 3 of 4 Bodies (Moon 20+, Mars 19, Phobos 9; Venus stranded by the bug). Gate tech Generation Ships never appeared on any shortlist I saw, and no Colonial Capital yet. Prospectors' Fund is 4,030/3,500 but they're held by 12/18 colonists.

### Game 3 summary (Arkwrights)
The most adventurous game - ships, launch windows, first-to-settle the Moon, Mars and Phobos bonuses, Colonists streaming outward. When it worked it was the best "space" feeling of the playtest. But: missed the first Mars window by one turn because a ship can't load and fly in one turn; Venus is unsettleable from orbit due to contradictory refusals (ship + 9 colonists stranded for good); the home Region quietly drowned; Influence income stayed ~11 all game despite the Spaceport's "+N Influence next turn" lines, so I could never hold Earth territory.

## Game 4 - Archivists, seed 204, start Japan

- T1 BUG: "Large Language Models" is on the tech list but `tech largelanguagemodels`/`tech large`/`tech llm` are all refused, and the valid-names list doesn't contain it. Its order id turns out to be `publicscience` (found by guessing the one odd name out). A casual player would never find that.
- T1 MY MISTAKE, but a QoL point: I "probed" a tech id with a one-line order file and, since the line was valid, it ended my whole turn 1 with nothing else ordered. A `--dry-run` / `check` command would let me test orders safely.
- T6 `influence india 12` was accepted while I held a non-aggression Accord with the Prospectors, whose home India is this seed. The help says non-aggression forbids exactly that. Either the order should be refused or I should be told I'm breaking the Accord. (Prospectors went "Wary".)
- T5-7 Archivist economy: 6 Materials a turn at start, Energy piling up (99) with nothing to spend it on. Archive banking 2-4 Research a turn against a 225 bar - 60+ turns at this rate. Feels hopeless early.
- T12 Built the Archive on the Moon (50 Materials). T16 took Kazakhstan by Influence, T17 Research Lab + Observatory finished and my banking jumped from 8 to 19 a turn. ENGAGED: finally a visible engine; Archive 96/225.
- T8-T11 FRUSTRATED: the Prospectors occupied Vietnam twice and destroyed its Research Lab the turn it finished. Influence buys it back "in name" but the occupier wrecks the buildings.
- T21 EXCITED: Archive full, 225/225, at T21 - from 8/turn at T15 to 30/turn by T20 once Kazakhstan and Argentina had Labs. Now I need The Upload, which is a SHARED tech I can't pick unless I lead research - and I've been sending 100% to the Archive. Switching the Directive to 0% to try to lead.
- T21 Colony Ship sat with 5.5 Fuel; refuel only at my station's own orbit. Fine, but `transit` refusal could suggest "refuel 27 first".
- T22 I finally led research (Directive at 0%) - and my gate, The Upload, wasn't on the 3-tech shortlist. Picked Superconductors and hoped. T23 it appeared; picked it T24.
- T24 CONFUSING: the Report said "Planetary Stewardship is complete" and "Generation Ships is complete" - not a word about The Upload, which was MY pick. It turns out three Techs finished that turn and The Upload was in the "Techs done" list. My own gate tech completing deserves the headline.
- T25 EXCITED then DEFLATED: uploaded 16 of 18 Colonists in one order (score 0.89!) - and the same Resolution "Energy ran short; shut down The Archive". The Archive's 20 Energy a turn ate my 150-Energy stockpile in ~6 turns while I wasn't looking; no forecast line told me "Energy runs out next turn". (`buy energy` existed; I never thought to use it because Energy had been a useless surplus all game.) Should win T26-27 once powered and 2 more Colonists arrive.

### Game 4 summary (Archivists)
Slowest start (6 Materials/turn, Archive banking 2/turn - felt hopeless to T12), best middle: Archive on the Moon, two countries flipped by Influence, Labs everywhere, Research banking climbed 8 -> 30 a turn and the Archive filled at T21. The shared-tech gate is a nice twist (stop funding the Archive to lead research, hope your Tech is on the shortlist). Ended T25 at 16/18 uploaded with the Archive browned out. Closest I came to winning, and the most satisfying arc.

---

# FINAL SUMMARY

## Per Faction
- **Custodians (seed 201, US):** Influence expansion was great fun early (8 countries by T16) and the colonist half of the win was done by T12. Then the game was building Scrubbers to push one number down (46 to 12.75 net by T22). The computer Prospectors erased that by flipping Japan and India, Scrubbers and all, after Accords lapsed without warning. Ended at +42 net, far from Stabilization. Lost by T24.
- **Prospectors (seed 202, EU):** Pure snowball: Influence, then stacked Investment Banks, Strip Permits, sell surplus, buy more Influence. Fund 2,324/3,500 at T25, about +250 a turn. The best moment was the race against my own collapse forecast. Would win around T30. Too easy once you see it.
- **Arkwrights (seed 203, South Africa):** The best "space" feeling: windows, ships, first to the Moon, Mars and Phobos. Spoiled by a missed window (a ship can't load and fly in one turn), the Venus bug that stranded 9 Colonists, my home quietly drowning, and Influence too weak to hold Earth. 60/42 Colonists but 3 of 4 Bodies.
- **Archivists (seed 204, Japan):** Bleak start, best arc. The Archive filled at T21 and I uploaded 16/18 at T25 before the Archive browned out from Energy. Likely to win T26-27.

## Top 5 frustrations
1. **Venus can't be settled from its orbital slot.** Four orders gave four answers that contradict each other, and a ship with 9 Colonists was stranded.
2. **Accords lapse without a word,** and some "accepted" Accords never take effect. Each time, the AI attacked or flipped my countries the same turn.
3. **Blind Trades and Accords.** I must answer offers whose terms are never shown. Some "owed" Trades don't even exist when answered.
4. **The computer Prospectors snowball in every seed** (half the world's emissions, 10+ countries) and set the collapse clock for everyone. I can see only my own Standing, so I never saw a takeover coming.
5. **No build menu or costs:** I learned prices by issuing orders. Two names for each region (`northamerica` / "The United States"). The LLM tech's order id is `publicscience`.

## Top 5 delights
1. Flipping neutral countries with Influence. It's like city-states in Civ and it's the core fun.
2. The order echo: "After them: 64M ...", plus refused lines that don't end the turn. It works as a safe build menu.
3. Climate breaks as story moments: sea level taking coastal slots worldwide, Coral Die-off, Permafrost.
4. "First to settle X" bonuses and ships arriving. The Arkwrights' T20 double landing was the best moment.
5. Clear progress numbers (Fund, Archive, net-over-Sink) that move every turn. The Prospectors' race against their own collapse forecast was genuinely tense.

## Exploits found
- **Uncapped `buy-influence` at 2 Ducats a point, fed by `sell materials`/`sell fuel`,** lets the richest Faction buy any country, even rivals' homes, in one turn.
- **Investment Banks have no per-country cap** (Scrubbers are capped at 2). At 21 Materials each, a Prospector just fills every slot.
- **Strip Permits:** they double output for free, and the cost (permanent emissions) lands on everyone.
- **Occupation is meaningless against Influence.** Russia changed hands every turn for 8 turns, and the AI kept feeding single Armies into it.

## QoL suggestions
- A `costs`/build-menu view with what each building does and where it can go.
- Show pending Accord and Trade terms in `show`. Add a "your Accord with X ends next turn" line.
- Show rivals' Standing in each country, and warn when a rival invests in mine.
- Put my own losses (sea, occupation, shutdowns) in "Your works". Add Energy-runway and "this ship needs to be ready by" warnings.
- Allow load+transit in one turn. Add a `--dry-run` for orders. Let me sell Energy.
- Cut the "Ticket #337 (version 0.09.0)" developer notes from `help`. Make the turn's EVENT line clearly last turn's card.

## Overall enjoyment: 6/10
The bones are a really good casual 4X. Flipping countries, launching ships at windows and watching the world drown around you are all fun, and the one-number-per-Faction win tracks made every turn feel like progress. All four games had a moment I leaned in for. But each game also had a moment where the game hid something I needed: a silent Accord lapse, a blind trade, an unexplained refusal, Venus. And the AI Prospectors' runaway turned two of my four games into watching someone else's game. Fix the information gaps and the Prospector snowball and this is an 8.
